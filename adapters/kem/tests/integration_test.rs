#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use domain::{
    AssetIdentityHash, AssetIdentityHashConstructorParamsOverrides, Secret,
    build_asset_identity_hash,
};
use hash_to_scalar::{
    CreateHashToScalarDeps, CreateHashToScalarPayload, build_create_hash_to_scalar_params,
    create_hash_to_scalar,
};
use kem::{
    ConsumeKemParams, ConsumeKemPayload, CreateKemDeps, CreateKemParamsOverrides, CreateKemPayload,
    DecapsulateParams, DecapsulatePayload, DeriveIdentityParams, DeriveIdentityPayload,
    EncapsulateParams, EncapsulatePayload, ICredentialKemAdapter, IKemConsumer, IdentityScope,
    IsValidParams, IsValidPayload, IssueParams, IssuePayload, KemIdentity, RerandomizeParams,
    RerandomizePayload, SetupParams, SetupPayload, SetupScope, build_create_kem_params, create_kem,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingDeps, CreatePairingParamsOverrides,
    CreatePairingPayload, IPairingAdapter, IPairingArithmetic, IPairingConsumer, IPairingReference,
    ISampleUniformScalar, PairingConcrete, build_create_pairing_params, create_pairing,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParamsOverrides, CreateRandomSourcePayload,
    FillBytesParams, FillBytesPayloadOverrides, RandomSourceKind,
    build_create_random_source_params, build_fill_bytes_payload, create_random_source,
};

struct RoundTripOutcome {
    issued_is_valid: bool,
    rerandomized_is_valid: bool,
    encapsulated: Vec<u8>,
    decapsulated_by_issued: Vec<u8>,
    decapsulated_by_rerandomized: Vec<u8>,
}

#[derive(Clone)]
enum FactoryIdentity {
    Entitlement(Vec<u8>),
    Asset(AssetIdentityHash),
}

struct KemRoundTrip {
    identity: FactoryIdentity,
}

impl<P: IPairingAdapter> IKemConsumer<P> for KemRoundTrip {
    type Output = RoundTripOutcome;

    fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
        &self,
        _params: ConsumeKemParams,
        payload: ConsumeKemPayload<K>,
    ) -> Self::Output {
        // Arrange
        let required_scope = payload.scope();
        let adapter = payload.adapter;
        let Ok(source) = create_random_source(
            &CreateRandomSourceDeps,
            build_create_random_source_params(CreateRandomSourceParamsOverrides {
                kind: Some(RandomSourceKind::OperatingSystem),
            }),
            CreateRandomSourcePayload,
        );
        let draw = || -> Secret<Vec<u8>> {
            let Ok(success) = source.adapter.fill_bytes(
                FillBytesParams,
                build_fill_bytes_payload(FillBytesPayloadOverrides {
                    length: Some(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH),
                }),
            ) else {
                panic!("the operating system's generator fills the draw");
            };
            success.bytes
        };
        let (scope, identity) = match (required_scope, &self.identity) {
            (IdentityScope::Entitlement, FactoryIdentity::Entitlement(canonical)) => (
                SetupScope::Entitlement,
                KemIdentity::Entitlement {
                    canonical: canonical.as_slice(),
                },
            ),
            (IdentityScope::Asset, FactoryIdentity::Asset(identity_hash)) => (
                SetupScope::Asset { identity_hash },
                KemIdentity::Asset { identity_hash },
            ),
            _ => panic!("the fixture's scope and identity agree"),
        };

        // Act
        let Ok(setup) = adapter.setup(
            SetupParams { scope },
            SetupPayload {
                master_uniform: draw(),
                u0_uniform: draw(),
                u1_uniform: draw(),
            },
        ) else {
            panic!("the parameter set is set up from operating-system draws");
        };
        let Ok(identity_element) = adapter.derive_identity(
            DeriveIdentityParams,
            DeriveIdentityPayload {
                parameter_set: &setup.parameter_set,
                identity,
            },
        ) else {
            panic!("the identity element is derived");
        };
        let Ok(issued) = adapter.issue(
            IssueParams,
            IssuePayload {
                parameter_set: &setup.parameter_set,
                master_scalar: &setup.master_scalar,
                identity_element: &identity_element.identity_element,
                uniform: draw(),
            },
        ) else {
            panic!("a credential is issued");
        };
        let Ok(rerandomized) = adapter.rerandomize(
            RerandomizeParams,
            RerandomizePayload {
                parameter_set: &setup.parameter_set,
                identity_element: &identity_element.identity_element,
                credential: &issued.credential,
                uniform: draw(),
            },
        ) else {
            panic!("the credential is rerandomized");
        };
        let Ok(encapsulated) = adapter.encapsulate(
            EncapsulateParams,
            EncapsulatePayload {
                parameter_set: &setup.parameter_set,
                uniform: draw(),
            },
        ) else {
            panic!("the capsule is encapsulated");
        };
        let Ok(issued_validity) = adapter.is_valid(
            IsValidParams,
            IsValidPayload {
                parameter_set: &setup.parameter_set,
                identity_element: &identity_element.identity_element,
                credential: &issued.credential,
            },
        );
        let Ok(rerandomized_validity) = adapter.is_valid(
            IsValidParams,
            IsValidPayload {
                parameter_set: &setup.parameter_set,
                identity_element: &identity_element.identity_element,
                credential: &rerandomized.credential,
            },
        );
        let Ok(by_issued) = adapter.decapsulate(
            DecapsulateParams,
            DecapsulatePayload {
                identity_element: &identity_element.identity_element,
                credential: &issued.credential,
                capsule: &encapsulated.capsule,
            },
        );
        let Ok(by_rerandomized) = adapter.decapsulate(
            DecapsulateParams,
            DecapsulatePayload {
                identity_element: &identity_element.identity_element,
                credential: &rerandomized.credential,
                capsule: &encapsulated.capsule,
            },
        );

        RoundTripOutcome {
            issued_is_valid: issued_validity.is_valid,
            rerandomized_is_valid: rerandomized_validity.is_valid,
            encapsulated: encapsulated.encapsulated.key_material().expose().clone(),
            decapsulated_by_issued: by_issued.encapsulated.key_material().expose().clone(),
            decapsulated_by_rerandomized: by_rerandomized
                .encapsulated
                .key_material()
                .expose()
                .clone(),
        }
    }
}

struct FactoryRoundTrip {
    scope: IdentityScope,
    identity: FactoryIdentity,
}

impl IPairingConsumer for FactoryRoundTrip {
    type Output = RoundTripOutcome;

    fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        // Arrange
        let Ok(hash_to_scalar) = create_hash_to_scalar::<P::Scalar>(
            &CreateHashToScalarDeps,
            build_create_hash_to_scalar_params(Default::default()),
            CreateHashToScalarPayload,
        ) else {
            panic!("the default hash-to-scalar concrete is constructed");
        };

        // Act
        let Ok(success) = create_kem(
            &CreateKemDeps {
                pairing: &payload.adapter,
                hash_to_scalar: hash_to_scalar.adapter.as_ref(),
                consumer: KemRoundTrip {
                    identity: self.identity.clone(),
                },
            },
            build_create_kem_params(CreateKemParamsOverrides {
                scope: Some(self.scope),
                ..Default::default()
            }),
            CreateKemPayload,
        ) else {
            panic!("the named concrete is admitted, constructed, and consumed");
        };

        success.output
    }
}

/// Contract: the KEM the factory constructs over the BLS12-381 arkworks
///   pairing, with every scalar drawn from the operating system's generator,
///   issues a valid credential, a seller's rerandomization of it is valid,
///   and both decapsulate the capsule to the encapsulated value.
/// Boundary: the pairing factory, the hash-to-scalar factory, the randomness
///   factory, `create_kem`, and the Boneh–Boyen concrete, each real.
/// Mocked:  nothing; the curve libraries, `sha3`, and the operating system's
///   generator are the outer edges.
/// Arrange: `FactoryRoundTrip` over `IdentityScope::Entitlement` and
///   `FactoryIdentity::Entitlement(b"entitlement-one")`.
/// Act:     `create_pairing` on `PairingConcrete::Bls12381Arkworks`.
/// Assert:  `issued_is_valid`, `rerandomized_is_valid`, and
///   `decapsulated_by_issued` and `decapsulated_by_rerandomized` each equal
///   `encapsulated`.
#[test]
fn a_kem_from_the_factory_round_trips_an_entitlement_scope_sale_from_operating_system_draws_on_bls12_381_arkworks()
 {
    // Arrange
    let consumer = FactoryRoundTrip {
        scope: IdentityScope::Entitlement,
        identity: FactoryIdentity::Entitlement(b"entitlement-one".to_vec()),
    };
    let deps = CreatePairingDeps { consumer };
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bls12381Arkworks),
        ..Default::default()
    });

    // Act
    let Ok(success) = create_pairing(&deps, params, CreatePairingPayload) else {
        panic!("the named concrete is constructed and consumed");
    };

    // Assert
    assert!(success.output.issued_is_valid);
    assert!(success.output.rerandomized_is_valid);
    assert_eq!(
        success.output.decapsulated_by_issued,
        success.output.encapsulated
    );
    assert_eq!(
        success.output.decapsulated_by_rerandomized,
        success.output.encapsulated
    );
}

/// Contract: the KEM the factory constructs over the BN254 halo2curves
///   pairing, with every scalar drawn from the operating system's generator,
///   issues a valid credential, a holder's rerandomization of it is valid,
///   and both decapsulate the capsule to the encapsulated value (CD-08).
/// Boundary: the pairing factory, the hash-to-scalar factory, the randomness
///   factory, `create_kem`, and the Boneh–Boyen concrete, each real.
/// Mocked:  nothing; the curve libraries, `sha3`, and the operating system's
///   generator are the outer edges.
/// Arrange: `FactoryRoundTrip` over `IdentityScope::Asset` and
///   `FactoryIdentity::Asset` built from bytes `[0xa1; 32]`.
/// Act:     `create_pairing` on `PairingConcrete::Bn254Halo2curves`.
/// Assert:  `issued_is_valid`, `rerandomized_is_valid`, and
///   `decapsulated_by_issued` and `decapsulated_by_rerandomized` each equal
///   `encapsulated`.
#[test]
fn a_kem_from_the_factory_round_trips_an_asset_scope_grant_from_operating_system_draws_on_bn254_halo2curves()
 {
    // Arrange
    let consumer = FactoryRoundTrip {
        scope: IdentityScope::Asset,
        identity: FactoryIdentity::Asset(build_asset_identity_hash(
            AssetIdentityHashConstructorParamsOverrides {
                bytes: Some([0xa1; 32]),
            },
        )),
    };
    let deps = CreatePairingDeps { consumer };
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bn254Halo2curves),
        ..Default::default()
    });

    // Act
    let Ok(success) = create_pairing(&deps, params, CreatePairingPayload) else {
        panic!("the named concrete is constructed and consumed");
    };

    // Assert
    assert!(success.output.issued_is_valid);
    assert!(success.output.rerandomized_is_valid);
    assert_eq!(
        success.output.decapsulated_by_issued,
        success.output.encapsulated
    );
    assert_eq!(
        success.output.decapsulated_by_rerandomized,
        success.output.encapsulated
    );
}
