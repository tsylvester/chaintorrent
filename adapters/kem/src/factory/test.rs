#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::create_kem;
use super::interface::{
    ConsumeKemParams, ConsumeKemPayload, CreateKemDeps, CreateKemPayload, ICredentialKemAdapter,
    IKemConsumer, IdentityScope, KEM_INTERFACE_VERSION, KemDeclaration, KemIdentifier,
};
use super::mock::{CreateKemParamsOverrides, build_create_kem_params};
use hash_to_scalar::{
    CreateHashToScalarDeps, CreateHashToScalarPayload, build_create_hash_to_scalar_params,
    create_hash_to_scalar,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingDeps, CreatePairingParamsOverrides,
    CreatePairingPayload, IPairingAdapter, IPairingArithmetic, IPairingConsumer, IPairingReference,
    PairingConcrete, build_create_pairing_params, create_pairing,
};

struct AdmissionRecord {
    declaration: KemDeclaration,
    scope: IdentityScope,
}

struct AdmissionProbe;

impl<P: IPairingAdapter> IKemConsumer<P> for AdmissionProbe {
    type Output = AdmissionRecord;

    fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
        &self,
        _params: ConsumeKemParams,
        payload: ConsumeKemPayload<K>,
    ) -> Self::Output {
        AdmissionRecord {
            declaration: K::DECLARATION,
            scope: payload.scope(),
        }
    }
}

struct FactoryProbe {
    scope: IdentityScope,
}

impl IPairingConsumer for FactoryProbe {
    type Output = AdmissionRecord;

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
                consumer: AdmissionProbe,
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

/// Contract: the Boneh–Boyen concrete, admitted for its identifier and the
///   entitlement scope, reaches the consumer with its declaration and that
///   scope.
/// Arrange: `FactoryProbe` over `IdentityScope::Entitlement`.
/// Act:     `create_pairing` on `PairingConcrete::Bls12381Arkworks`.
/// Assert:  `declaration.identifier` is `KemIdentifier::Bb1DepthOneV1`,
///   `declaration.identity_tag` is `b"ChainTorrent-v1-kem-identity"`,
///   `declaration.adapter_version` is `1`,
///   `declaration.interface_version` is `KEM_INTERFACE_VERSION`, and `scope`
///   is `IdentityScope::Entitlement`.
#[test]
fn create_kem_hands_the_consumer_the_bb1_depth_one_concrete_under_the_entitlement_scope() {
    // Arrange
    let consumer = FactoryProbe {
        scope: IdentityScope::Entitlement,
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
    assert!(matches!(
        success.output.declaration.identifier,
        KemIdentifier::Bb1DepthOneV1
    ));
    assert_eq!(
        success.output.declaration.identity_tag,
        b"ChainTorrent-v1-kem-identity"
    );
    assert_eq!(success.output.declaration.adapter_version, 1);
    assert_eq!(
        success.output.declaration.interface_version,
        KEM_INTERFACE_VERSION
    );
    assert_eq!(success.output.scope, IdentityScope::Entitlement);
}

/// Contract: the same concrete, admitted for the asset scope, reaches the
///   consumer with the asset scope (CD-08).
/// Arrange: `FactoryProbe` over `IdentityScope::Asset`.
/// Act:     `create_pairing` on `PairingConcrete::Bls12381Arkworks`.
/// Assert:  `declaration.identifier` is `KemIdentifier::Bb1DepthOneV1` and
///   `scope` is `IdentityScope::Asset`.
#[test]
fn create_kem_hands_the_consumer_the_bb1_depth_one_concrete_under_the_asset_scope() {
    // Arrange
    let consumer = FactoryProbe {
        scope: IdentityScope::Asset,
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
    assert!(matches!(
        success.output.declaration.identifier,
        KemIdentifier::Bb1DepthOneV1
    ));
    assert_eq!(success.output.scope, IdentityScope::Asset);
}
