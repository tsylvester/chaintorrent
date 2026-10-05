#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use domain::{
    AssetIdentityConstructorParamsOverrides, AssetIdentityHash,
    AssetIdentityHashConstructorParamsOverrides, DeploymentIdentityConstructorParamsOverrides,
    DerivationContext, DerivationContextConstructorParamsOverrides,
    GroupIndexConstructorParamsOverrides, ParameterSetIdentifierConstructorParamsOverrides,
    PieceGeometryConstructorParamsOverrides, Secret, SecretConstructorParamsOverrides,
    SuiteIdentifierConstructorParamsOverrides, build_asset_identity, build_asset_identity_hash,
    build_deployment_identity, build_derivation_context, build_group_index,
    build_parameter_set_identifier, build_piece_geometry, build_secret, build_suite_identifier,
};
use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingDeps,
    CreateEncodingParamsOverrides, CreateEncodingPayload, EncodingConcrete, EncodingIdentifier,
    IDecoderAdapter, IEncoderAdapter, IEncodingConsumer, build_create_encoding_params,
    create_encoding,
};
use hash_to_scalar::{
    CreateHashToScalarDeps, CreateHashToScalarPayload, build_create_hash_to_scalar_params,
    create_hash_to_scalar,
};
use hex::decode;
use kdf::{
    CreateKeyDerivationDeps, CreateKeyDerivationParamsOverrides, CreateKeyDerivationPayload,
    KdfConcrete, KdfIdentifier, build_create_key_derivation_params, create_key_derivation,
};
use kem::{
    ConsumeKemParams, ConsumeKemPayload, CreateKemDeps, CreateKemParamsOverrides, CreateKemPayload,
    DecapsulateParams, DecapsulatePayload, DeriveIdentityParams, DeriveIdentityPayload,
    EncapsulateParams, EncapsulatePayload, EncapsulatedValue, EncapsulatedValueOverrides,
    ICredentialKemAdapter, IKemConsumer, IdentityScope, IssueParams, IssuePayload, KemIdentity,
    SetupParams, SetupPayload, SetupScope, build_create_kem_params, build_encapsulated_value,
    create_kem,
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
use workflows::{
    PieceGroupKey, PieceGroupKeyOverrides, UnwrapPieceGroupKeyDeps, UnwrapPieceGroupKeyParams,
    UnwrapPieceGroupKeyPayload, WrapPieceGroupKeyDeps, WrapPieceGroupKeyParams,
    WrapPieceGroupKeyPayload, build_piece_group_key, unwrap_piece_group_key, wrap_piece_group_key,
};

fn reference_context() -> DerivationContext {
    build_derivation_context(DerivationContextConstructorParamsOverrides {
        asset: Some(build_asset_identity(
            AssetIdentityConstructorParamsOverrides {
                name: Some("@scope/example-package".to_string()),
                version: Some("2.1.0-beta.3".to_string()),
            },
        )),
        deployment: Some(build_deployment_identity(
            DeploymentIdentityConstructorParamsOverrides {
                bytes: Some([0x0a; 32]),
            },
        )),
        suite: Some(build_suite_identifier(
            SuiteIdentifierConstructorParamsOverrides {
                identifier: Some([0x0b; 32]),
                version: Some(3),
            },
        )),
        parameter_set: Some(build_parameter_set_identifier(
            ParameterSetIdentifierConstructorParamsOverrides {
                bytes: Some([0x0c; 32]),
            },
        )),
        group_index: Some(build_group_index(GroupIndexConstructorParamsOverrides {
            value: Some(5),
        })),
        geometry: Some(build_piece_geometry(
            PieceGeometryConstructorParamsOverrides {
                piece_size: Some(16384),
                piece_group_size: Some(32768),
                total_extent: Some(1048576),
            },
        )),
    })
}

struct WrapUnderEncoding {
    context: DerivationContext,
    piece_group_key: PieceGroupKey,
}

impl IEncodingConsumer for WrapUnderEncoding {
    type Output = Vec<u8>;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
        let Ok(kdf) = create_key_derivation(
            &CreateKeyDerivationDeps,
            build_create_key_derivation_params(CreateKeyDerivationParamsOverrides {
                concrete: Some(KdfConcrete::Blake3Keyed),
                identifier: Some(KdfIdentifier::Blake3KeyedV1),
            }),
            CreateKeyDerivationPayload,
        ) else {
            panic!("the BLAKE3 keyed concrete is admitted")
        };
        let encapsulated = build_encapsulated_value(EncapsulatedValueOverrides {
            bytes: Some(build_secret(SecretConstructorParamsOverrides {
                value: Some(vec![0x42; 32]),
            })),
        });
        let Ok(success) = wrap_piece_group_key(
            &WrapPieceGroupKeyDeps {
                kdf: kdf.adapter.as_ref(),
                encoder: &payload.adapter,
            },
            WrapPieceGroupKeyParams,
            WrapPieceGroupKeyPayload {
                encapsulated: &encapsulated,
                context: &self.context,
                piece_group_key: &self.piece_group_key,
            },
        ) else {
            panic!("the wrap succeeds")
        };
        success.wrapped.as_bytes().to_vec()
    }
}

/// Contract: under the reference key material and the reference context, the
///   wrapping key the wrap derives through the real encoding and
///   key-derivation factories is the independent implementation's, so XOR
///   with a zero key exposes it.
/// Boundary: `create_encoding` (the ABI concrete and the derivation-context
///   description), `create_key_derivation` (the BLAKE3 keyed concrete), and
///   `wrap_piece_group_key`, each real.
/// Mocked:  nothing — `alloy` and `blake3` are the outer edges; the
///   encapsulated value is the key-derivation family's reference key material
///   rather than a KEM output.
/// Arrange: the reference context and the piece-group key `vec![0u8; 32]`.
/// Act:     `create_encoding` running `WrapUnderEncoding`.
/// Assert:  `success.output` equals the first 32 bytes of the wrapping-key
///   vector `kdf/blake3_keyed`'s tests state.
#[test]
fn wrapping_a_zero_key_yields_the_independent_wrapping_key_vector() {
    // Arrange
    let consumer = WrapUnderEncoding {
        context: reference_context(),
        piece_group_key: build_piece_group_key(PieceGroupKeyOverrides {
            bytes: Some(vec![0u8; 32]),
        }),
    };
    let Ok(expected) = decode("549874404f0b25b65da55dbce0b410d0aa34085695af4b91303e2c1c0410f760")
    else {
        panic!("the vector is valid hex")
    };

    // Act
    let Ok(success) = create_encoding(
        &CreateEncodingDeps { consumer },
        build_create_encoding_params(CreateEncodingParamsOverrides {
            concrete: Some(EncodingConcrete::Abi),
            identifier: Some(EncodingIdentifier::EthereumAbiV1),
        }),
        CreateEncodingPayload,
    ) else {
        panic!("the ABI concrete is admitted")
    };

    // Assert
    assert_eq!(success.output, expected);
}

/// Contract: the wrap is the byte-wise XOR, so an all-ones piece-group key
///   yields the complement of the independent wrapping-key vector.
/// Boundary: `create_encoding` (the ABI concrete and the derivation-context
///   description), `create_key_derivation` (the BLAKE3 keyed concrete), and
///   `wrap_piece_group_key`, each real.
/// Mocked:  nothing — `alloy` and `blake3` are the outer edges; the
///   encapsulated value is the key-derivation family's reference key material
///   rather than a KEM output.
/// Arrange: the reference context and the piece-group key `vec![0xff; 32]`.
/// Act:     `create_encoding` running `WrapUnderEncoding`.
/// Assert:  `success.output` equals the complement of the first 32 bytes of
///   the wrapping-key vector.
#[test]
fn wrapping_an_all_ones_key_yields_the_complement_of_the_wrapping_key_vector() {
    // Arrange
    let consumer = WrapUnderEncoding {
        context: reference_context(),
        piece_group_key: build_piece_group_key(PieceGroupKeyOverrides {
            bytes: Some(vec![0xff; 32]),
        }),
    };
    let Ok(expected) = decode("ab678bbfb0f4da49a25aa2431f4bef2f55cbf7a96a50b46ecfc1d3e3fbef089f")
    else {
        panic!("the vector is valid hex")
    };

    // Act
    let Ok(success) = create_encoding(
        &CreateEncodingDeps { consumer },
        build_create_encoding_params(CreateEncodingParamsOverrides {
            concrete: Some(EncodingConcrete::Abi),
            identifier: Some(EncodingIdentifier::EthereumAbiV1),
        }),
        CreateEncodingPayload,
    ) else {
        panic!("the ABI concrete is admitted")
    };

    // Assert
    assert_eq!(success.output, expected);
}

struct SetOutcome {
    encapsulated: EncapsulatedValue,
    decapsulated: EncapsulatedValue,
}

enum FactoryIdentity {
    Entitlement(Vec<u8>),
    Asset(AssetIdentityHash),
}

impl FactoryIdentity {
    fn for_scope(scope: IdentityScope) -> FactoryIdentity {
        match scope {
            IdentityScope::Entitlement => FactoryIdentity::Entitlement(b"entitlement-one".to_vec()),
            IdentityScope::Asset => FactoryIdentity::Asset(build_asset_identity_hash(
                AssetIdentityHashConstructorParamsOverrides {
                    bytes: Some([0xa1; 32]),
                },
            )),
        }
    }
}

struct KemSetProbe {
    identity: FactoryIdentity,
}

impl<P: IPairingAdapter> IKemConsumer<P> for KemSetProbe {
    type Output = SetOutcome;

    fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
        &self,
        _params: ConsumeKemParams,
        payload: ConsumeKemPayload<K>,
    ) -> Self::Output {
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
        let Ok(encapsulated) = adapter.encapsulate(
            EncapsulateParams,
            EncapsulatePayload {
                parameter_set: &setup.parameter_set,
                uniform: draw(),
            },
        ) else {
            panic!("the capsule is encapsulated");
        };
        let Ok(decapsulated) = adapter.decapsulate(
            DecapsulateParams,
            DecapsulatePayload {
                identity_element: &identity_element.identity_element,
                credential: &issued.credential,
                capsule: &encapsulated.capsule,
            },
        );

        SetOutcome {
            encapsulated: encapsulated.encapsulated,
            decapsulated: decapsulated.encapsulated,
        }
    }
}

struct CrossSetOutcome {
    piece_group_key: Vec<u8>,
    wrapped_under_first_set: Vec<u8>,
    wrapped_under_second_set: Vec<u8>,
    unwrapped_under_first_set: Vec<u8>,
    unwrapped_under_second_set: Vec<u8>,
}

struct WrapUnwrapUnderEncoding {
    first: SetOutcome,
    second: SetOutcome,
    piece_group_key: PieceGroupKey,
    first_context: DerivationContext,
    second_context: DerivationContext,
}

impl IEncodingConsumer for WrapUnwrapUnderEncoding {
    type Output = CrossSetOutcome;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
        let Ok(kdf) = create_key_derivation(
            &CreateKeyDerivationDeps,
            build_create_key_derivation_params(CreateKeyDerivationParamsOverrides {
                concrete: Some(KdfConcrete::Blake3Keyed),
                identifier: Some(KdfIdentifier::Blake3KeyedV1),
            }),
            CreateKeyDerivationPayload,
        ) else {
            panic!("the BLAKE3 keyed concrete is admitted")
        };
        let Ok(first_wrapped) = wrap_piece_group_key(
            &WrapPieceGroupKeyDeps {
                kdf: kdf.adapter.as_ref(),
                encoder: &payload.adapter,
            },
            WrapPieceGroupKeyParams,
            WrapPieceGroupKeyPayload {
                encapsulated: &self.first.encapsulated,
                context: &self.first_context,
                piece_group_key: &self.piece_group_key,
            },
        ) else {
            panic!("the first set's wrap succeeds")
        };
        let Ok(second_wrapped) = wrap_piece_group_key(
            &WrapPieceGroupKeyDeps {
                kdf: kdf.adapter.as_ref(),
                encoder: &payload.adapter,
            },
            WrapPieceGroupKeyParams,
            WrapPieceGroupKeyPayload {
                encapsulated: &self.second.encapsulated,
                context: &self.second_context,
                piece_group_key: &self.piece_group_key,
            },
        ) else {
            panic!("the second set's wrap succeeds")
        };
        let unwrap_deps = UnwrapPieceGroupKeyDeps {
            kdf: kdf.adapter.as_ref(),
            encoder: &payload.adapter,
        };
        let Ok(first_unwrapped) = unwrap_piece_group_key(
            &unwrap_deps,
            UnwrapPieceGroupKeyParams,
            UnwrapPieceGroupKeyPayload {
                encapsulated: &self.first.decapsulated,
                context: &self.first_context,
                wrapped: &first_wrapped.wrapped,
            },
        ) else {
            panic!("the first set's unwrap succeeds")
        };
        let Ok(second_unwrapped) = unwrap_piece_group_key(
            &unwrap_deps,
            UnwrapPieceGroupKeyParams,
            UnwrapPieceGroupKeyPayload {
                encapsulated: &self.second.decapsulated,
                context: &self.second_context,
                wrapped: &second_wrapped.wrapped,
            },
        ) else {
            panic!("the second set's unwrap succeeds")
        };

        CrossSetOutcome {
            piece_group_key: self.piece_group_key.expose().to_vec(),
            wrapped_under_first_set: first_wrapped.wrapped.as_bytes().to_vec(),
            wrapped_under_second_set: second_wrapped.wrapped.as_bytes().to_vec(),
            unwrapped_under_first_set: first_unwrapped.piece_group_key.expose().to_vec(),
            unwrapped_under_second_set: second_unwrapped.piece_group_key.expose().to_vec(),
        }
    }
}

struct CrossSetProbe {
    first_scope: IdentityScope,
    second_scope: IdentityScope,
}

impl IPairingConsumer for CrossSetProbe {
    type Output = CrossSetOutcome;

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
        let Ok(first) = create_kem(
            &CreateKemDeps {
                pairing: &payload.adapter,
                hash_to_scalar: hash_to_scalar.adapter.as_ref(),
                consumer: KemSetProbe {
                    identity: FactoryIdentity::for_scope(self.first_scope),
                },
            },
            build_create_kem_params(CreateKemParamsOverrides {
                scope: Some(self.first_scope),
                ..Default::default()
            }),
            CreateKemPayload,
        ) else {
            panic!("the first set's KEM is admitted, constructed, and consumed");
        };
        let Ok(second) = create_kem(
            &CreateKemDeps {
                pairing: &payload.adapter,
                hash_to_scalar: hash_to_scalar.adapter.as_ref(),
                consumer: KemSetProbe {
                    identity: FactoryIdentity::for_scope(self.second_scope),
                },
            },
            build_create_kem_params(CreateKemParamsOverrides {
                scope: Some(self.second_scope),
                ..Default::default()
            }),
            CreateKemPayload,
        ) else {
            panic!("the second set's KEM is admitted, constructed, and consumed");
        };
        let Ok(source) = create_random_source(
            &CreateRandomSourceDeps,
            build_create_random_source_params(CreateRandomSourceParamsOverrides {
                kind: Some(RandomSourceKind::OperatingSystem),
            }),
            CreateRandomSourcePayload,
        );
        let Ok(drawn) = source.adapter.fill_bytes(
            FillBytesParams,
            build_fill_bytes_payload(FillBytesPayloadOverrides { length: Some(32) }),
        ) else {
            panic!("the operating system's generator fills the piece-group key");
        };
        let Ok(piece_group_key) = PieceGroupKey::try_from_secret_bytes(drawn.bytes) else {
            panic!("the drawn piece-group key is the admitted length");
        };
        let first_context = build_derivation_context(DerivationContextConstructorParamsOverrides {
            parameter_set: Some(build_parameter_set_identifier(
                ParameterSetIdentifierConstructorParamsOverrides {
                    bytes: Some([0x0c; 32]),
                },
            )),
            ..Default::default()
        });
        let second_context =
            build_derivation_context(DerivationContextConstructorParamsOverrides {
                parameter_set: Some(build_parameter_set_identifier(
                    ParameterSetIdentifierConstructorParamsOverrides {
                        bytes: Some([0x0d; 32]),
                    },
                )),
                ..Default::default()
            });

        // Act
        let Ok(success) = create_encoding(
            &CreateEncodingDeps {
                consumer: WrapUnwrapUnderEncoding {
                    first: first.output,
                    second: second.output,
                    piece_group_key,
                    first_context,
                    second_context,
                },
            },
            build_create_encoding_params(CreateEncodingParamsOverrides {
                concrete: Some(EncodingConcrete::Abi),
                identifier: Some(EncodingIdentifier::EthereumAbiV1),
            }),
            CreateEncodingPayload,
        ) else {
            panic!("the ABI concrete is admitted")
        };

        success.output
    }
}

/// Contract: one piece-group key wrapped under each of two independently
///   generated entitlement-scope sets is recovered by a holder of each set,
///   and the two sidecar entries differ (CR-08 cross-set agreement; CR-11).
/// Boundary: `create_pairing`, `create_hash_to_scalar`,
///   `create_random_source`, `create_kem` with the Boneh–Boyen concrete,
///   `create_encoding` with the ABI concrete, `create_key_derivation` with
///   the BLAKE3 keyed concrete, `wrap_piece_group_key`, and
///   `unwrap_piece_group_key`, each real.
/// Mocked:  nothing; the curve libraries, `sha3`, `alloy`, `blake3`, and the
///   operating system's generator are the outer edges.
/// Arrange: `CrossSetProbe` over two `IdentityScope::Entitlement` scopes.
/// Act:     `create_pairing` on `PairingConcrete::Bls12381Arkworks`.
/// Assert:  `unwrapped_under_first_set` and `unwrapped_under_second_set` each
///   equal `piece_group_key`, and `wrapped_under_first_set` differs from
///   `wrapped_under_second_set`.
#[test]
fn two_independently_generated_entitlement_scope_sets_unwrap_one_piece_group_key_on_bls12_381_arkworks()
 {
    // Arrange
    let consumer = CrossSetProbe {
        first_scope: IdentityScope::Entitlement,
        second_scope: IdentityScope::Entitlement,
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
    assert_eq!(
        success.output.unwrapped_under_first_set,
        success.output.piece_group_key
    );
    assert_eq!(
        success.output.unwrapped_under_second_set,
        success.output.piece_group_key
    );
    assert_ne!(
        success.output.wrapped_under_first_set,
        success.output.wrapped_under_second_set
    );
}

/// Contract: an escrow set under the asset scope and a claimant's set under
///   the entitlement scope open one ciphertext's piece-group key, the
///   sidecar coverage a successor body relies on (CR-08; CD-08; LC-09).
/// Boundary: `create_pairing`, `create_hash_to_scalar`,
///   `create_random_source`, `create_kem` with the Boneh–Boyen concrete,
///   `create_encoding` with the ABI concrete, `create_key_derivation` with
///   the BLAKE3 keyed concrete, `wrap_piece_group_key`, and
///   `unwrap_piece_group_key`, each real.
/// Mocked:  nothing; the curve libraries, `sha3`, `alloy`, `blake3`, and the
///   operating system's generator are the outer edges.
/// Arrange: `CrossSetProbe` over `IdentityScope::Asset` and
///   `IdentityScope::Entitlement`.
/// Act:     `create_pairing` on `PairingConcrete::Bn254Arkworks`.
/// Assert:  `unwrapped_under_first_set` and `unwrapped_under_second_set` each
///   equal `piece_group_key`, and `wrapped_under_first_set` differs from
///   `wrapped_under_second_set`.
#[test]
fn an_escrow_set_and_a_claimants_set_unwrap_one_piece_group_key_on_bn254_arkworks() {
    // Arrange
    let consumer = CrossSetProbe {
        first_scope: IdentityScope::Asset,
        second_scope: IdentityScope::Entitlement,
    };
    let deps = CreatePairingDeps { consumer };
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bn254Arkworks),
        ..Default::default()
    });

    // Act
    let Ok(success) = create_pairing(&deps, params, CreatePairingPayload) else {
        panic!("the named concrete is constructed and consumed");
    };

    // Assert
    assert_eq!(
        success.output.unwrapped_under_first_set,
        success.output.piece_group_key
    );
    assert_eq!(
        success.output.unwrapped_under_second_set,
        success.output.piece_group_key
    );
    assert_ne!(
        success.output.wrapped_under_first_set,
        success.output.wrapped_under_second_set
    );
}
