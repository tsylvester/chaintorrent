#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::interface::{
    BB1_DEPTH_ONE_IDENTITY_TAG, Bb1DepthOneCapsule, Bb1DepthOneKem, Bb1DepthOneParameterSetScope,
};
use super::mock::{
    Bb1DepthOneCredentialOverrides, Bb1DepthOneParameterSetOverrides,
    build_bb1_depth_one_credential, build_bb1_depth_one_identity_element, build_bb1_depth_one_kem,
    build_bb1_depth_one_kem_constructor_params, build_bb1_depth_one_master_scalar,
    build_bb1_depth_one_parameter_set,
};
use crate::factory::provides::{
    CapsuleComponents, CapsuleFromComponentsPayloadOverrides, CredentialComponentsOverrides,
    CredentialFromComponentsPayloadOverrides, DeriveIdentityErrorReturn,
    DeriveIdentityPayloadOverrides, EncapsulateErrorReturn, EncapsulatePayloadOverrides,
    ICredentialKemAdapter, IssueErrorReturn, IssuePayloadOverrides, KemIdentity,
    MasterScalarComponentsOverrides, MasterScalarFromComponentsPayloadOverrides,
    ParameterSetComponentsOverrides, ParameterSetFromComponentsPayloadOverrides,
    ParameterSetScopeComponents, RerandomizeErrorReturn, RerandomizePayloadOverrides,
    SetupErrorReturn, SetupParamsOverrides, SetupPayloadOverrides, SetupScope,
    build_capsule_components_params, build_capsule_components_payload,
    build_capsule_from_components_params, build_capsule_from_components_payload,
    build_credential_components, build_credential_components_params,
    build_credential_components_payload, build_credential_from_components_params,
    build_credential_from_components_payload, build_decapsulate_params, build_decapsulate_payload,
    build_derive_identity_params, build_derive_identity_payload, build_encapsulate_params,
    build_encapsulate_payload, build_identity_element_components_params,
    build_identity_element_components_payload, build_is_valid_params, build_is_valid_payload,
    build_is_well_formed_params, build_is_well_formed_payload, build_issue_params,
    build_issue_payload, build_master_scalar_components, build_master_scalar_components_params,
    build_master_scalar_components_payload, build_master_scalar_from_components_params,
    build_master_scalar_from_components_payload, build_parameter_set_components,
    build_parameter_set_components_params, build_parameter_set_components_payload,
    build_parameter_set_from_components_params, build_parameter_set_from_components_payload,
    build_rerandomize_params, build_rerandomize_payload, build_setup_params, build_setup_payload,
};
use domain::{
    AssetIdentityHashConstructorParamsOverrides, Secret, build_asset_identity_hash, build_secret,
};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, HashToScalarConcrete, HashToScalarErrorReturn,
    MockIHashToScalarAdapterFailureMode, build_create_hash_to_scalar_deps,
    build_create_hash_to_scalar_params, build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingPayload, EncodeG1Params,
    EncodeG1PayloadOverrides, EncodeG2Params, EncodeG2PayloadOverrides, EncodeGtParams,
    EncodeGtPayloadOverrides, EncodeScalarParams, EncodeScalarPayloadOverrides, IPairingAdapter,
    IPairingArithmetic, IPairingConsumer, ISampleUniformScalar, MulG1Params, MulG1PayloadOverrides,
    MulG2Params, MulG2PayloadOverrides, PairingProductParams, PairingProductPayloadOverrides,
    PairingProductTerm, PairingProductTermOverrides, SampleUniformScalarErrorReturn,
    SampleUniformScalarParams, SampleUniformScalarPayloadOverrides, build_create_pairing_deps,
    build_create_pairing_params, build_encode_g1_payload, build_encode_g2_payload,
    build_encode_gt_payload, build_encode_scalar_payload, build_g1_generator_success_return,
    build_g2_generator_success_return, build_mul_g1_payload, build_mul_g2_payload,
    build_pairing_product_payload, build_pairing_product_term, build_sample_uniform_scalar_payload,
    create_pairing,
};

fn scalar_bytes<P: IPairingAdapter>(value: u8) -> Secret<Vec<u8>> {
    let mut bytes = vec![0u8; P::Scalar::UNIFORM_BYTES_LENGTH];
    bytes[P::Scalar::UNIFORM_BYTES_LENGTH - 1] = value;
    build_secret(bytes, Default::default())
}

fn shortened<P: IPairingAdapter>() -> Secret<Vec<u8>> {
    build_secret(
        vec![0u8; P::Scalar::UNIFORM_BYTES_LENGTH - 1],
        Default::default(),
    )
}

fn scalar<P: IPairingAdapter>(value: u8) -> P::Scalar {
    let Ok(sampled) = P::Scalar::sample_from_uniform_bytes(
        SampleUniformScalarParams,
        build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
            uniform: Some(scalar_bytes::<P>(value)),
        }),
    ) else {
        panic!("the family's sampler admits uniform bytes of the exact length")
    };
    sampled.scalar.expose().clone()
}

fn point_g1<P: IPairingAdapter>(pairing: &P) -> P::G1 {
    build_g1_generator_success_return(pairing, Default::default()).point
}

fn point_g2<P: IPairingAdapter>(pairing: &P) -> P::G2 {
    build_g2_generator_success_return(pairing, Default::default()).point
}

fn identity_g1<P: IPairingAdapter>(pairing: &P) -> P::G1 {
    pairing
        .mul_g1(
            MulG1Params,
            build_mul_g1_payload(
                pairing,
                MulG1PayloadOverrides {
                    scalar: Some(scalar::<P>(0)),
                    ..Default::default()
                },
            ),
        )
        .expect("scalar multiplication never refuses")
        .product
}

fn identity_g2<P: IPairingAdapter>(pairing: &P) -> P::G2 {
    pairing
        .mul_g2(
            MulG2Params,
            build_mul_g2_payload(
                pairing,
                MulG2PayloadOverrides {
                    scalar: Some(scalar::<P>(0)),
                    ..Default::default()
                },
            ),
        )
        .expect("scalar multiplication never refuses")
        .product
}

fn encoded_g1<P: IPairingAdapter>(pairing: &P, point: P::G1) -> P::EncodedG1 {
    pairing
        .encode_g1(
            EncodeG1Params,
            build_encode_g1_payload(pairing, EncodeG1PayloadOverrides { point: Some(point) }),
        )
        .expect("encoding a first-group point never refuses")
        .bytes
}

fn encoded_g2<P: IPairingAdapter>(pairing: &P, point: P::G2) -> P::EncodedG2 {
    pairing
        .encode_g2(
            EncodeG2Params,
            build_encode_g2_payload(pairing, EncodeG2PayloadOverrides { point: Some(point) }),
        )
        .expect("encoding a second-group point never refuses")
        .bytes
}

fn encoded_scalar<P: IPairingAdapter>(pairing: &P, value: P::Scalar) -> Vec<u8> {
    pairing
        .encode_scalar(
            EncodeScalarParams,
            build_encode_scalar_payload(
                pairing,
                EncodeScalarPayloadOverrides {
                    scalar: Some(value),
                },
            ),
        )
        .expect("encoding a scalar never refuses")
        .bytes
        .expose()
        .as_ref()
        .to_vec()
}

fn encoded_gt<P: IPairingArithmetic>(
    pairing: &P,
    terms: Vec<PairingProductTerm<P::G1, P::G2>>,
) -> Vec<u8> {
    let product = pairing
        .pairing_product(
            PairingProductParams,
            build_pairing_product_payload(
                pairing,
                PairingProductPayloadOverrides { terms: Some(terms) },
            ),
        )
        .expect("the pairing product never refuses")
        .product;
    pairing
        .encode_gt(
            EncodeGtParams,
            build_encode_gt_payload(
                pairing,
                EncodeGtPayloadOverrides {
                    value: Some(product),
                },
            ),
        )
        .expect("encoding a target-group value never refuses")
        .bytes
        .expose()
        .as_ref()
        .to_vec()
}

/// Contract: `Bb1DepthOneKem::try_new`, admitted: the tag builds →
///   `Ok(Bb1DepthOneKem { pairing, hash_to_scalar, tag })`.
/// Arrange: constructor params over the pairing and the hash-to-scalar family's
///   mock concrete, `kem` not bound.
/// Act:     `Bb1DepthOneKem::try_new` over those params.
/// Assert:  `success.tag.as_bytes()` equals the identity tag the interface
///   states.
#[test]
fn try_new_holds_the_tag_the_declaration_names() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let params = build_bb1_depth_one_kem_constructor_params(
                &consumed.adapter,
                &*made.adapter,
                Default::default(),
            );

            // Act
            let result = Bb1DepthOneKem::try_new(params);

            // Assert
            let Ok(success) = result else {
                panic!("admitted constructor params build the concrete")
            };
            assert_eq!(success.tag.as_bytes(), BB1_DEPTH_ONE_IDENTITY_TAG);
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: the master uniform bytes are refused →
///   `Err(SetupErrorReturn::MasterScalarSampling(error))`, the refusal
///   unchanged.
/// Arrange: `master_uniform` one byte shorter than
///   `P::Scalar::UNIFORM_BYTES_LENGTH`, so the other samplings succeed.
/// Act:     `kem.setup(params, payload)`.
/// Assert:  the whole error equals `MasterScalarSampling` carrying
///   `WrongLength { expected, actual: expected - 1 }`.
#[test]
fn setup_returns_the_master_sampling_refusal_whole() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let params = build_setup_params(Default::default());
            let payload = build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                master_uniform: Some(shortened::<P>()),
                ..Default::default()
            });

            // Act
            let result = kem.setup(params, payload);

            // Assert
            let Err(error) = result else {
                panic!("a refused master sampling returns the refusal")
            };
            assert_eq!(
                error,
                SetupErrorReturn::MasterScalarSampling(
                    SampleUniformScalarErrorReturn::WrongLength {
                        expected: P::Scalar::UNIFORM_BYTES_LENGTH,
                        actual: P::Scalar::UNIFORM_BYTES_LENGTH - 1,
                    }
                )
            );
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: the `u0` uniform bytes are refused →
///   `Err(SetupErrorReturn::U0Sampling(error))`, the refusal unchanged.
/// Arrange: `u0_uniform` one byte shorter than
///   `P::Scalar::UNIFORM_BYTES_LENGTH`, the master and `u1` bytes default.
/// Act:     `kem.setup(params, payload)`.
/// Assert:  the whole error equals `U0Sampling` carrying `WrongLength
///   { expected, actual: expected - 1 }`.
#[test]
fn setup_returns_the_u0_sampling_refusal_whole() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let params = build_setup_params(Default::default());
            let payload = build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                u0_uniform: Some(shortened::<P>()),
                ..Default::default()
            });

            // Act
            let result = kem.setup(params, payload);

            // Assert
            let Err(error) = result else {
                panic!("a refused u0 sampling returns the refusal")
            };
            assert_eq!(
                error,
                SetupErrorReturn::U0Sampling(SampleUniformScalarErrorReturn::WrongLength {
                    expected: P::Scalar::UNIFORM_BYTES_LENGTH,
                    actual: P::Scalar::UNIFORM_BYTES_LENGTH - 1,
                })
            );
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: the `u1` uniform bytes are refused →
///   `Err(SetupErrorReturn::U1Sampling(error))`, the refusal unchanged.
/// Arrange: `u1_uniform` one byte shorter than
///   `P::Scalar::UNIFORM_BYTES_LENGTH`, the master and `u0` bytes default.
/// Act:     `kem.setup(params, payload)`.
/// Assert:  the whole error equals `U1Sampling` carrying `WrongLength
///   { expected, actual: expected - 1 }`.
#[test]
fn setup_returns_the_u1_sampling_refusal_whole() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let params = build_setup_params(Default::default());
            let payload = build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                u1_uniform: Some(shortened::<P>()),
                ..Default::default()
            });

            // Act
            let result = kem.setup(params, payload);

            // Assert
            let Err(error) = result else {
                panic!("a refused u1 sampling returns the refusal")
            };
            assert_eq!(
                error,
                SetupErrorReturn::U1Sampling(SampleUniformScalarErrorReturn::WrongLength {
                    expected: P::Scalar::UNIFORM_BYTES_LENGTH,
                    actual: P::Scalar::UNIFORM_BYTES_LENGTH - 1,
                })
            );
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: sampling runs in the order master, `u0`, `u1`, and the refusal of
///   the earliest sampler to be refused is returned.
/// Arrange: `master_uniform` and `u0_uniform` each one byte shorter, so a
///   sampling order that reached `u0` first returns `U0Sampling`.
/// Act:     `kem.setup(params, payload)`.
/// Assert:  the whole error equals `MasterScalarSampling` carrying
///   `WrongLength { expected, actual: expected - 1 }`.
#[test]
fn setup_returns_the_master_refusal_when_the_master_and_u0_bytes_are_both_refused() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let params = build_setup_params(Default::default());
            let payload = build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                master_uniform: Some(shortened::<P>()),
                u0_uniform: Some(shortened::<P>()),
                ..Default::default()
            });

            // Act
            let result = kem.setup(params, payload);

            // Assert
            let Err(error) = result else {
                panic!("a refused master sampling returns the refusal")
            };
            assert_eq!(
                error,
                SetupErrorReturn::MasterScalarSampling(
                    SampleUniformScalarErrorReturn::WrongLength {
                        expected: P::Scalar::UNIFORM_BYTES_LENGTH,
                        actual: P::Scalar::UNIFORM_BYTES_LENGTH - 1,
                    }
                )
            );
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `params.scope` is `SetupScope::Entitlement` and every sampling
///   succeeds → `Ok` holding a parameter set whose scope is
///   `Bb1DepthOneParameterSetScope::Entitlement`.
/// Arrange: `build_setup_params(Default::default())`, the entitlement scope,
///   and `build_setup_payload(Default::default())`.
/// Act:     `kem.setup(params, payload)`.
/// Assert:  `success.parameter_set.scope` matches
///   `Bb1DepthOneParameterSetScope::Entitlement`.
#[test]
fn setup_fixes_an_entitlement_scope_set_to_no_identity() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let params = build_setup_params(Default::default());
            let payload = build_setup_payload::<P::Scalar>(Default::default());

            // Act
            let result = kem.setup(params, payload);

            // Assert
            let Ok(success) = result else {
                panic!("the default payload is admitted")
            };
            assert!(matches!(
                success.parameter_set.scope,
                Bb1DepthOneParameterSetScope::Entitlement
            ));
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `params.scope` is `SetupScope::Asset { identity_hash }` and the
///   identity mapping succeeds → `Ok` holding a parameter set whose scope is
///   `Bb1DepthOneParameterSetScope::Asset { identity_element }`.
/// Arrange: the asset scope with `hash` from `build_asset_identity_hash` with
///   `bytes: Some([0xa1; 32])`, and `build_setup_payload(Default::default())`.
/// Act:     `kem.setup(params, payload)`.
/// Assert:  `success.parameter_set.scope` matches
///   `Bb1DepthOneParameterSetScope::Asset { .. }`.
#[test]
fn setup_fixes_an_asset_scope_set_to_its_identity_element() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let hash = build_asset_identity_hash(AssetIdentityHashConstructorParamsOverrides {
                bytes: Some([0xa1; 32]),
            });
            let params = build_setup_params(SetupParamsOverrides {
                scope: Some(SetupScope::Asset {
                    identity_hash: &hash,
                }),
            });
            let payload = build_setup_payload::<P::Scalar>(Default::default());

            // Act
            let result = kem.setup(params, payload);

            // Assert
            let Ok(success) = result else {
                panic!("the identity mapping over the asset scope is admitted")
            };
            assert!(matches!(
                success.parameter_set.scope,
                Bb1DepthOneParameterSetScope::Asset { .. }
            ));
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `params.scope` is `SetupScope::Asset { identity_hash }` and the
///   hash adapter refuses → `Err(SetupErrorReturn::HashToScalar(error))`, the
///   refusal unchanged.
/// Arrange: `made` under the `SamplingRefused` failure mode, the asset scope
///   with `hash` from `build_asset_identity_hash` with `bytes:
///   Some([0xa1; 32])`, and `build_setup_payload(Default::default())`.
/// Act:     `kem.setup(params, payload)`.
/// Assert:  the whole error equals `SetupErrorReturn::HashToScalar(
///   HashToScalarErrorReturn::Sampling(WrongLength { expected, actual: 0 }))`.
#[test]
fn setup_returns_the_hash_refusal_whole_under_the_asset_scope() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = CreateHashToScalarParamsOverrides {
                concrete: Some(HashToScalarConcrete::Mock(
                    MockIHashToScalarAdapterFailureMode::SamplingRefused,
                )),
                ..Default::default()
            };
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the named selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let hash = build_asset_identity_hash(AssetIdentityHashConstructorParamsOverrides {
                bytes: Some([0xa1; 32]),
            });
            let params = build_setup_params(SetupParamsOverrides {
                scope: Some(SetupScope::Asset {
                    identity_hash: &hash,
                }),
            });
            let payload = build_setup_payload::<P::Scalar>(Default::default());

            // Act
            let result = kem.setup(params, payload);

            // Assert
            let Err(error) = result else {
                panic!("a refusing hash adapter returns the refusal")
            };
            assert_eq!(
                error,
                SetupErrorReturn::HashToScalar(HashToScalarErrorReturn::Sampling(
                    SampleUniformScalarErrorReturn::WrongLength {
                        expected: P::Scalar::UNIFORM_BYTES_LENGTH,
                        actual: 0,
                    }
                ))
            );
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `params.scope` is `SetupScope::Asset { identity_hash }` and `F`
///   is the identity element → `Err(SetupErrorReturn::TrivialIdentityElement)`.
/// Arrange: the asset scope as the asset-scope admission block and
///   `build_setup_payload` with `u0_uniform` and `u1_uniform` each
///   `Some(scalar_bytes(0))`, so `u0` and `u1` are the identity element and
///   `F` is it whatever `I` is.
/// Act:     `kem.setup(params, payload)`.
/// Assert:  the whole error equals `SetupErrorReturn::TrivialIdentityElement`.
#[test]
fn setup_refuses_a_trivial_identity_element_under_the_asset_scope() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let hash = build_asset_identity_hash(AssetIdentityHashConstructorParamsOverrides {
                bytes: Some([0xa1; 32]),
            });
            let params = build_setup_params(SetupParamsOverrides {
                scope: Some(SetupScope::Asset {
                    identity_hash: &hash,
                }),
            });
            let payload = build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                u0_uniform: Some(scalar_bytes::<P>(0)),
                u1_uniform: Some(scalar_bytes::<P>(0)),
                ..Default::default()
            });

            // Act
            let result = kem.setup(params, payload);

            // Assert
            let Err(error) = result else {
                panic!("a trivial identity element is refused")
            };
            assert_eq!(error, SetupErrorReturn::TrivialIdentityElement);
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: the parameter set's scope is `Entitlement` and the identity is
///   `KemIdentity::Asset` → `Err(DeriveIdentityErrorReturn::WrongIdentityScope)`
///   before hashing.
/// Arrange: `made` under the `SamplingRefused` failure mode, so a hash that
///   ran first returns the hash refusal, the default entitlement parameter
///   set, and `KemIdentity::Asset { identity_hash: &hash }`.
/// Act:     `kem.derive_identity(params, payload)`.
/// Assert:  the whole error equals `DeriveIdentityErrorReturn::
///   WrongIdentityScope`.
#[test]
fn derive_identity_refuses_an_asset_identity_under_an_entitlement_set_before_hashing() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = CreateHashToScalarParamsOverrides {
                concrete: Some(HashToScalarConcrete::Mock(
                    MockIHashToScalarAdapterFailureMode::SamplingRefused,
                )),
                ..Default::default()
            };
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the named selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let parameter_set =
                build_bb1_depth_one_parameter_set(&consumed.adapter, Default::default());
            let hash = build_asset_identity_hash(AssetIdentityHashConstructorParamsOverrides {
                bytes: Some([0xa1; 32]),
            });
            let identity = KemIdentity::Asset {
                identity_hash: &hash,
            };
            let params = build_derive_identity_params(Default::default());
            let payload = build_derive_identity_payload(
                &parameter_set,
                DeriveIdentityPayloadOverrides {
                    identity: Some(identity),
                },
            );

            // Act
            let result = kem.derive_identity(params, payload);

            // Assert
            let Err(error) = result else {
                panic!("an identity of the wrong scope is refused")
            };
            assert_eq!(error, DeriveIdentityErrorReturn::WrongIdentityScope);
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: the parameter set's scope is `Asset` and the identity is
///   `KemIdentity::Entitlement` → `Err(DeriveIdentityErrorReturn::
///   WrongIdentityScope)` before hashing.
/// Arrange: `made` under the `SamplingRefused` failure mode, the parameter set
///   under the asset scope of the default identity element's `element`, and
///   `KemIdentity::Entitlement { canonical: b"entitlement-one" }`.
/// Act:     `kem.derive_identity(params, payload)`.
/// Assert:  the whole error equals `DeriveIdentityErrorReturn::
///   WrongIdentityScope`.
#[test]
fn derive_identity_refuses_an_entitlement_identity_under_an_asset_set_before_hashing() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = CreateHashToScalarParamsOverrides {
                concrete: Some(HashToScalarConcrete::Mock(
                    MockIHashToScalarAdapterFailureMode::SamplingRefused,
                )),
                ..Default::default()
            };
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the named selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let parameter_set = build_bb1_depth_one_parameter_set(
                &consumed.adapter,
                Bb1DepthOneParameterSetOverrides {
                    scope: Some(Bb1DepthOneParameterSetScope::Asset {
                        identity_element: build_bb1_depth_one_identity_element(
                            &consumed.adapter,
                            Default::default(),
                        )
                        .element,
                    }),
                    ..Default::default()
                },
            );
            let identity = KemIdentity::Entitlement {
                canonical: b"entitlement-one",
            };
            let params = build_derive_identity_params(Default::default());
            let payload = build_derive_identity_payload(
                &parameter_set,
                DeriveIdentityPayloadOverrides {
                    identity: Some(identity),
                },
            );

            // Act
            let result = kem.derive_identity(params, payload);

            // Assert
            let Err(error) = result else {
                panic!("an identity of the wrong scope is refused")
            };
            assert_eq!(error, DeriveIdentityErrorReturn::WrongIdentityScope);
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: the hash adapter refuses → `Err(DeriveIdentityErrorReturn::
///   HashToScalar(error))`, the refusal unchanged.
/// Arrange: `made` under the `SamplingRefused` failure mode, the default
///   entitlement parameter set, and the default identity of
///   `build_derive_identity_payload`, `KemIdentity::Entitlement`.
/// Act:     `kem.derive_identity(params, payload)`.
/// Assert:  the whole error equals `DeriveIdentityErrorReturn::HashToScalar(
///   HashToScalarErrorReturn::Sampling(WrongLength { expected, actual: 0 }))`.
#[test]
fn derive_identity_returns_the_hash_refusal_whole() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = CreateHashToScalarParamsOverrides {
                concrete: Some(HashToScalarConcrete::Mock(
                    MockIHashToScalarAdapterFailureMode::SamplingRefused,
                )),
                ..Default::default()
            };
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the named selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let parameter_set =
                build_bb1_depth_one_parameter_set(&consumed.adapter, Default::default());
            let params = build_derive_identity_params(Default::default());
            let payload = build_derive_identity_payload(&parameter_set, Default::default());

            // Act
            let result = kem.derive_identity(params, payload);

            // Assert
            let Err(error) = result else {
                panic!("a refusing hash adapter returns the refusal")
            };
            assert_eq!(
                error,
                DeriveIdentityErrorReturn::HashToScalar(HashToScalarErrorReturn::Sampling(
                    SampleUniformScalarErrorReturn::WrongLength {
                        expected: P::Scalar::UNIFORM_BYTES_LENGTH,
                        actual: 0,
                    }
                ))
            );
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `F = u0 + I·u1` is the identity element →
///   `Err(DeriveIdentityErrorReturn::TrivialIdentityElement)`.
/// Arrange: the default hasher, the parameter set with `u0` and `u1` each the
///   first-group identity element, so `F` is it whatever `I` is, and the
///   default entitlement identity.
/// Act:     `kem.derive_identity(params, payload)`.
/// Assert:  the whole error equals `DeriveIdentityErrorReturn::
///   TrivialIdentityElement`.
#[test]
fn derive_identity_refuses_a_trivial_identity_element() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let identity_point = identity_g1(&consumed.adapter);
            let parameter_set = build_bb1_depth_one_parameter_set(
                &consumed.adapter,
                Bb1DepthOneParameterSetOverrides {
                    u0: Some(identity_point.clone()),
                    u1: Some(identity_point),
                    ..Default::default()
                },
            );
            let params = build_derive_identity_params(Default::default());
            let payload = build_derive_identity_payload(&parameter_set, Default::default());

            // Act
            let result = kem.derive_identity(params, payload);

            // Assert
            let Err(error) = result else {
                panic!("a trivial identity element is refused")
            };
            assert_eq!(error, DeriveIdentityErrorReturn::TrivialIdentityElement);
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: the scope is `Asset { identity_element }` and `encode_g1(F)`
///   differs from `encode_g1(identity_element)` →
///   `Err(DeriveIdentityErrorReturn::OutsideAssetScope)`.
/// Arrange: the parameter set with `u1` the first-group identity element, so
///   `F` is the set's default `u0`, `scope` the asset scope fixing the
///   first-group generator, a point other than that `u0`, and
///   `KemIdentity::Asset { identity_hash: &hash }`.
/// Act:     `kem.derive_identity(params, payload)`.
/// Assert:  the whole error equals `DeriveIdentityErrorReturn::
///   OutsideAssetScope`.
#[test]
fn derive_identity_refuses_an_element_outside_the_asset_scope() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let parameter_set = build_bb1_depth_one_parameter_set(
                &consumed.adapter,
                Bb1DepthOneParameterSetOverrides {
                    u1: Some(identity_g1(&consumed.adapter)),
                    scope: Some(Bb1DepthOneParameterSetScope::Asset {
                        identity_element: point_g1(&consumed.adapter),
                    }),
                    ..Default::default()
                },
            );
            let hash = build_asset_identity_hash(AssetIdentityHashConstructorParamsOverrides {
                bytes: Some([0xa1; 32]),
            });
            let identity = KemIdentity::Asset {
                identity_hash: &hash,
            };
            let params = build_derive_identity_params(Default::default());
            let payload = build_derive_identity_payload(
                &parameter_set,
                DeriveIdentityPayloadOverrides {
                    identity: Some(identity),
                },
            );

            // Act
            let result = kem.derive_identity(params, payload);

            // Assert
            let Err(error) = result else {
                panic!("a derived element outside the asset scope is refused")
            };
            assert_eq!(error, DeriveIdentityErrorReturn::OutsideAssetScope);
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}
/// Contract: the mapping succeeds and the encodings are equal →
///   `Ok(DeriveIdentitySuccessReturn { identity_element:
///   Bb1DepthOneIdentityElement { scalar: I, element: F } })`.
/// Arrange: `u0` the default parameter set's `u0`, the parameter set with `u1`
///   the first-group identity element, so `F` is `u0` whatever `I` is, `scope`
///   the asset scope fixing `u0`, and `KemIdentity::Asset
///   { identity_hash: &hash }`.
/// Act:     `kem.derive_identity(params, payload)`.
/// Assert:  `encoded_g1(success.identity_element.element)` equals
///   `encoded_g1(u0)`, the sum of `u0` and a multiple of the identity element.
#[test]
fn derive_identity_derives_the_element_under_the_asset_scope() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let u0 = build_bb1_depth_one_parameter_set(&consumed.adapter, Default::default()).u0;
            let parameter_set = build_bb1_depth_one_parameter_set(
                &consumed.adapter,
                Bb1DepthOneParameterSetOverrides {
                    u1: Some(identity_g1(&consumed.adapter)),
                    scope: Some(Bb1DepthOneParameterSetScope::Asset {
                        identity_element: u0.clone(),
                    }),
                    ..Default::default()
                },
            );
            let hash = build_asset_identity_hash(AssetIdentityHashConstructorParamsOverrides {
                bytes: Some([0xa1; 32]),
            });
            let identity = KemIdentity::Asset {
                identity_hash: &hash,
            };
            let params = build_derive_identity_params(Default::default());
            let payload = build_derive_identity_payload(
                &parameter_set,
                DeriveIdentityPayloadOverrides {
                    identity: Some(identity),
                },
            );

            // Act
            let result = kem.derive_identity(params, payload);

            // Assert
            let Ok(success) = result else {
                panic!("a derived element inside the asset scope is admitted")
            };
            assert!(
                encoded_g1(&consumed.adapter, success.identity_element.element)
                    == encoded_g1(&consumed.adapter, u0)
            );
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: the mapping succeeds under the entitlement scope with no
///   comparison → `Ok(DeriveIdentitySuccessReturn { identity_element })`.
/// Arrange: `u0` the default parameter set's `u0`, the parameter set with `u1`
///   the first-group identity element and the entitlement scope, and the
///   default entitlement identity.
/// Act:     `kem.derive_identity(params, payload)`.
/// Assert:  `encoded_g1(success.identity_element.element)` equals
///   `encoded_g1(u0)`.
#[test]
fn derive_identity_derives_the_element_under_the_entitlement_scope() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let u0 = build_bb1_depth_one_parameter_set(&consumed.adapter, Default::default()).u0;
            let parameter_set = build_bb1_depth_one_parameter_set(
                &consumed.adapter,
                Bb1DepthOneParameterSetOverrides {
                    u1: Some(identity_g1(&consumed.adapter)),
                    ..Default::default()
                },
            );
            let params = build_derive_identity_params(Default::default());
            let payload = build_derive_identity_payload(&parameter_set, Default::default());

            // Act
            let result = kem.derive_identity(params, payload);

            // Assert
            let Ok(success) = result else {
                panic!("the mapping under the entitlement scope is admitted")
            };
            assert!(
                encoded_g1(&consumed.adapter, success.identity_element.element)
                    == encoded_g1(&consumed.adapter, u0)
            );
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: the uniform bytes are refused →
///   `Err(IssueErrorReturn::Sampling(error))`, the refusal unchanged.
/// Arrange: the default parameter set, master scalar, and identity element,
///   and `build_issue_payload` with `uniform` one byte shorter than
///   `P::Scalar::UNIFORM_BYTES_LENGTH`.
/// Act:     `kem.issue(params, payload)`.
/// Assert:  the whole error equals `IssueErrorReturn::Sampling(WrongLength
///   { expected, actual: expected - 1 })`.
#[test]
fn issue_returns_the_sampling_refusal_whole() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let parameter_set =
                build_bb1_depth_one_parameter_set(&consumed.adapter, Default::default());
            let master_scalar = build_bb1_depth_one_master_scalar(Default::default());
            let identity_element =
                build_bb1_depth_one_identity_element(&consumed.adapter, Default::default());
            let params = build_issue_params(Default::default());
            let payload = build_issue_payload::<P::Scalar, _, _, _>(
                &parameter_set,
                &master_scalar,
                &identity_element,
                IssuePayloadOverrides {
                    uniform: Some(shortened::<P>()),
                },
            );

            // Act
            let result = kem.issue(params, payload);

            // Assert
            let Err(error) = result else {
                panic!("a refused sampling returns the refusal")
            };
            assert_eq!(
                error,
                IssueErrorReturn::Sampling(SampleUniformScalarErrorReturn::WrongLength {
                    expected: P::Scalar::UNIFORM_BYTES_LENGTH,
                    actual: P::Scalar::UNIFORM_BYTES_LENGTH - 1,
                })
            );
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `(A, B) = (α·g1 + r·F, r·g2)` for the sampled `r`.
/// Arrange: the default parameter set, master scalar, and identity element,
///   `uniform` `scalar_bytes(1)`, and `credential` the default
///   `build_bb1_depth_one_credential`, the credential for `r = 1` over those
///   defaults.
/// Act:     `kem.issue(params, payload)`.
/// Assert:  `success.credential.a` encodes equal to `credential.a` and
///   `success.credential.b` to `credential.b`.
#[test]
fn issue_returns_the_default_credential_for_unit_randomness() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let parameter_set =
                build_bb1_depth_one_parameter_set(&consumed.adapter, Default::default());
            let master_scalar = build_bb1_depth_one_master_scalar(Default::default());
            let identity_element =
                build_bb1_depth_one_identity_element(&consumed.adapter, Default::default());
            let credential = build_bb1_depth_one_credential(&consumed.adapter, Default::default());
            let params = build_issue_params(Default::default());
            let payload = build_issue_payload::<P::Scalar, _, _, _>(
                &parameter_set,
                &master_scalar,
                &identity_element,
                IssuePayloadOverrides {
                    uniform: Some(scalar_bytes::<P>(1)),
                },
            );

            // Act
            let result = kem.issue(params, payload);

            // Assert
            let Ok(success) = result else {
                panic!("the default payload is admitted")
            };
            assert!(
                encoded_g1(&consumed.adapter, success.credential.a)
                    == encoded_g1(&consumed.adapter, credential.a)
            );
            assert!(
                encoded_g2(&consumed.adapter, success.credential.b)
                    == encoded_g2(&consumed.adapter, credential.b)
            );
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `B = r·g2`.
/// Arrange: the same defaults with `uniform` `scalar_bytes(0)`.
/// Act:     `kem.issue(params, payload)`.
/// Assert:  `success.credential.b` encodes equal to the second-group identity
///   element, the product of `g2` by zero.
#[test]
fn issue_derives_b_as_the_identity_element_for_zero_randomness() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let parameter_set =
                build_bb1_depth_one_parameter_set(&consumed.adapter, Default::default());
            let master_scalar = build_bb1_depth_one_master_scalar(Default::default());
            let identity_element =
                build_bb1_depth_one_identity_element(&consumed.adapter, Default::default());
            let params = build_issue_params(Default::default());
            let payload = build_issue_payload::<P::Scalar, _, _, _>(
                &parameter_set,
                &master_scalar,
                &identity_element,
                IssuePayloadOverrides {
                    uniform: Some(scalar_bytes::<P>(0)),
                },
            );

            // Act
            let result = kem.issue(params, payload);

            // Assert
            let Ok(success) = result else {
                panic!("the default payload is admitted")
            };
            assert!(
                encoded_g2(&consumed.adapter, success.credential.b)
                    == encoded_g2(&consumed.adapter, identity_g2(&consumed.adapter))
            );
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `randomness` is the sampled `Secret` of `r`.
/// Arrange: the same defaults with `uniform` `scalar_bytes(1)`.
/// Act:     `kem.issue(params, payload)`.
/// Assert:  `success.randomness.expose()` encodes equal to `scalar(1)`.
#[test]
fn issue_returns_the_sampled_randomness() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let parameter_set =
                build_bb1_depth_one_parameter_set(&consumed.adapter, Default::default());
            let master_scalar = build_bb1_depth_one_master_scalar(Default::default());
            let identity_element =
                build_bb1_depth_one_identity_element(&consumed.adapter, Default::default());
            let params = build_issue_params(Default::default());
            let payload = build_issue_payload::<P::Scalar, _, _, _>(
                &parameter_set,
                &master_scalar,
                &identity_element,
                IssuePayloadOverrides {
                    uniform: Some(scalar_bytes::<P>(1)),
                },
            );

            // Act
            let result = kem.issue(params, payload);

            // Assert
            let Ok(success) = result else {
                panic!("the default payload is admitted")
            };
            assert!(
                encoded_scalar(&consumed.adapter, success.randomness.expose().clone())
                    == encoded_scalar(&consumed.adapter, scalar::<P>(1))
            );
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: the uniform bytes are refused →
///   `Err(RerandomizeErrorReturn::Sampling(error))`, the refusal unchanged.
/// Arrange: the default parameter set, identity element, and credential, and
///   `build_rerandomize_payload` with `uniform` one byte shorter than
///   `P::Scalar::UNIFORM_BYTES_LENGTH`.
/// Act:     `kem.rerandomize(params, payload)`.
/// Assert:  the whole error equals `RerandomizeErrorReturn::Sampling(
///   WrongLength { expected, actual: expected - 1 })`.
#[test]
fn rerandomize_returns_the_sampling_refusal_whole() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let parameter_set =
                build_bb1_depth_one_parameter_set(&consumed.adapter, Default::default());
            let identity_element =
                build_bb1_depth_one_identity_element(&consumed.adapter, Default::default());
            let credential = build_bb1_depth_one_credential(&consumed.adapter, Default::default());
            let params = build_rerandomize_params(Default::default());
            let payload = build_rerandomize_payload::<P::Scalar, _, _, _>(
                &parameter_set,
                &identity_element,
                &credential,
                RerandomizePayloadOverrides {
                    uniform: Some(shortened::<P>()),
                },
            );

            // Act
            let result = kem.rerandomize(params, payload);

            // Assert
            let Err(error) = result else {
                panic!("a refused sampling returns the refusal")
            };
            assert_eq!(
                error,
                RerandomizeErrorReturn::Sampling(SampleUniformScalarErrorReturn::WrongLength {
                    expected: P::Scalar::UNIFORM_BYTES_LENGTH,
                    actual: P::Scalar::UNIFORM_BYTES_LENGTH - 1,
                })
            );
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `A' = A + s·F` for the sampled `s`.
/// Arrange: the default parameter set, identity element, and credential, and
///   `uniform` `scalar_bytes(0)`.
/// Act:     `kem.rerandomize(params, payload)`.
/// Assert:  `success.credential.a` encodes equal to `credential.a`, the sum of
///   `A` and the product of `F` by zero.
#[test]
fn rerandomize_leaves_a_unchanged_for_a_zero_offset() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let parameter_set =
                build_bb1_depth_one_parameter_set(&consumed.adapter, Default::default());
            let identity_element =
                build_bb1_depth_one_identity_element(&consumed.adapter, Default::default());
            let credential = build_bb1_depth_one_credential(&consumed.adapter, Default::default());
            let params = build_rerandomize_params(Default::default());
            let payload = build_rerandomize_payload::<P::Scalar, _, _, _>(
                &parameter_set,
                &identity_element,
                &credential,
                RerandomizePayloadOverrides {
                    uniform: Some(scalar_bytes::<P>(0)),
                },
            );

            // Act
            let result = kem.rerandomize(params, payload);

            // Assert
            let Ok(success) = result else {
                panic!("the default payload is admitted")
            };
            assert!(
                encoded_g1(&consumed.adapter, success.credential.a)
                    == encoded_g1(&consumed.adapter, credential.a)
            );
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `B' = B + s·g2`.
/// Arrange: as the zero-offset block for `a`.
/// Act:     `kem.rerandomize(params, payload)`.
/// Assert:  `success.credential.b` encodes equal to `credential.b`.
#[test]
fn rerandomize_leaves_b_unchanged_for_a_zero_offset() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let parameter_set =
                build_bb1_depth_one_parameter_set(&consumed.adapter, Default::default());
            let identity_element =
                build_bb1_depth_one_identity_element(&consumed.adapter, Default::default());
            let credential = build_bb1_depth_one_credential(&consumed.adapter, Default::default());
            let params = build_rerandomize_params(Default::default());
            let payload = build_rerandomize_payload::<P::Scalar, _, _, _>(
                &parameter_set,
                &identity_element,
                &credential,
                RerandomizePayloadOverrides {
                    uniform: Some(scalar_bytes::<P>(0)),
                },
            );

            // Act
            let result = kem.rerandomize(params, payload);

            // Assert
            let Ok(success) = result else {
                panic!("the default payload is admitted")
            };
            assert!(
                encoded_g2(&consumed.adapter, success.credential.b)
                    == encoded_g2(&consumed.adapter, credential.b)
            );
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `offset` is the sampled `Secret` of `s`.
/// Arrange: the same fixtures with `uniform` `scalar_bytes(1)`.
/// Act:     `kem.rerandomize(params, payload)`.
/// Assert:  `success.offset.expose()` encodes equal to `scalar(1)`.
#[test]
fn rerandomize_returns_the_sampled_offset() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let parameter_set =
                build_bb1_depth_one_parameter_set(&consumed.adapter, Default::default());
            let identity_element =
                build_bb1_depth_one_identity_element(&consumed.adapter, Default::default());
            let credential = build_bb1_depth_one_credential(&consumed.adapter, Default::default());
            let params = build_rerandomize_params(Default::default());
            let payload = build_rerandomize_payload::<P::Scalar, _, _, _>(
                &parameter_set,
                &identity_element,
                &credential,
                RerandomizePayloadOverrides {
                    uniform: Some(scalar_bytes::<P>(1)),
                },
            );

            // Act
            let result = kem.rerandomize(params, payload);

            // Assert
            let Ok(success) = result else {
                panic!("the default payload is admitted")
            };
            assert!(
                encoded_scalar(&consumed.adapter, success.offset.expose().clone())
                    == encoded_scalar(&consumed.adapter, scalar::<P>(1))
            );
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: the pairing-product check `e(A, g2) = e(g1, hpub) · e(F, B)`
///   holds → `Ok(IsValidSuccessReturn { is_valid: true })`.
/// Arrange: the default parameter set, identity element, and credential, which
///   satisfy the check by the relation `bb1_depth_one/mock.rs` states.
/// Act:     `kem.is_valid(params, payload)`.
/// Assert:  `success.is_valid` holds.
#[test]
fn is_valid_accepts_a_credential_that_satisfies_the_check() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let parameter_set =
                build_bb1_depth_one_parameter_set(&consumed.adapter, Default::default());
            let identity_element =
                build_bb1_depth_one_identity_element(&consumed.adapter, Default::default());
            let credential = build_bb1_depth_one_credential(&consumed.adapter, Default::default());
            let params = build_is_valid_params(Default::default());
            let payload = build_is_valid_payload(
                &parameter_set,
                &identity_element,
                &credential,
                Default::default(),
            );

            // Act
            let result = kem.is_valid(params, payload);

            // Assert
            let Ok(success) = result;
            assert!(success.is_valid);
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: the check fails → `Ok(IsValidSuccessReturn { is_valid: false })`.
/// Arrange: the same fixtures with the credential's `a` the set's `g1`, a
///   point other than the default `A`.
/// Act:     `kem.is_valid(params, payload)`.
/// Assert:  `success.is_valid` does not hold.
#[test]
fn is_valid_rejects_a_credential_with_another_a() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let parameter_set =
                build_bb1_depth_one_parameter_set(&consumed.adapter, Default::default());
            let identity_element =
                build_bb1_depth_one_identity_element(&consumed.adapter, Default::default());
            let credential = build_bb1_depth_one_credential(
                &consumed.adapter,
                Bb1DepthOneCredentialOverrides {
                    a: Some(parameter_set.g1.clone()),
                    ..Default::default()
                },
            );
            let params = build_is_valid_params(Default::default());
            let payload = build_is_valid_payload(
                &parameter_set,
                &identity_element,
                &credential,
                Default::default(),
            );

            // Act
            let result = kem.is_valid(params, payload);

            // Assert
            let Ok(success) = result;
            assert!(!success.is_valid);
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: the uniform bytes are refused →
///   `Err(EncapsulateErrorReturn::Sampling(error))`, the refusal unchanged.
/// Arrange: the default parameter set and `build_encapsulate_payload` with
///   `uniform` one byte shorter than `P::Scalar::UNIFORM_BYTES_LENGTH`.
/// Act:     `kem.encapsulate(params, payload)`.
/// Assert:  the whole error equals `EncapsulateErrorReturn::Sampling(
///   WrongLength { expected, actual: expected - 1 })`.
#[test]
fn encapsulate_returns_the_sampling_refusal_whole() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let parameter_set =
                build_bb1_depth_one_parameter_set(&consumed.adapter, Default::default());
            let params = build_encapsulate_params(Default::default());
            let payload = build_encapsulate_payload::<P::Scalar, _>(
                &parameter_set,
                EncapsulatePayloadOverrides {
                    uniform: Some(shortened::<P>()),
                },
            );

            // Act
            let result = kem.encapsulate(params, payload);

            // Assert
            let Err(error) = result else {
                panic!("a refused sampling returns the refusal")
            };
            assert_eq!(
                error,
                EncapsulateErrorReturn::Sampling(SampleUniformScalarErrorReturn::WrongLength {
                    expected: P::Scalar::UNIFORM_BYTES_LENGTH,
                    actual: P::Scalar::UNIFORM_BYTES_LENGTH - 1,
                })
            );
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}
/// Contract: `U = t·g2` for the sampled `t`.
/// Arrange: the default parameter set and `uniform` `scalar_bytes(1)`.
/// Act:     `kem.encapsulate(params, payload)`.
/// Assert:  the `u` of `Bb1DepthOneCapsule::Entitlement { u, .. }` encodes
///   equal to the second-group generator, the product of `g2` by one.
#[test]
fn encapsulate_returns_u_as_g2_for_unit_randomness() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let parameter_set =
                build_bb1_depth_one_parameter_set(&consumed.adapter, Default::default());
            let params = build_encapsulate_params(Default::default());
            let payload = build_encapsulate_payload::<P::Scalar, _>(
                &parameter_set,
                EncapsulatePayloadOverrides {
                    uniform: Some(scalar_bytes::<P>(1)),
                },
            );

            // Act
            let result = kem.encapsulate(params, payload);

            // Assert
            let Ok(success) = result else {
                panic!("the default payload is admitted")
            };
            let Bb1DepthOneCapsule::Entitlement { u, .. } = success.capsule else {
                panic!("the entitlement scope returns an entitlement capsule")
            };
            assert!(
                encoded_g2(&consumed.adapter, u)
                    == encoded_g2(&consumed.adapter, point_g2(&consumed.adapter))
            );
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `V = t·u0`.
/// Arrange: as the block for `u`.
/// Act:     `kem.encapsulate(params, payload)`.
/// Assert:  the `v` of `Bb1DepthOneCapsule::Entitlement { v, .. }` encodes
///   equal to `parameter_set.u0`, the product of `u0` by one.
#[test]
fn encapsulate_returns_v_as_u0_for_unit_randomness_under_the_entitlement_scope() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let parameter_set =
                build_bb1_depth_one_parameter_set(&consumed.adapter, Default::default());
            let params = build_encapsulate_params(Default::default());
            let payload = build_encapsulate_payload::<P::Scalar, _>(
                &parameter_set,
                EncapsulatePayloadOverrides {
                    uniform: Some(scalar_bytes::<P>(1)),
                },
            );

            // Act
            let result = kem.encapsulate(params, payload);

            // Assert
            let Ok(success) = result else {
                panic!("the default payload is admitted")
            };
            let Bb1DepthOneCapsule::Entitlement { v, .. } = success.capsule else {
                panic!("the entitlement scope returns an entitlement capsule")
            };
            assert!(
                encoded_g1(&consumed.adapter, v) == encoded_g1(&consumed.adapter, parameter_set.u0)
            );
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `W = t·u1`.
/// Arrange: as the block for `u`.
/// Act:     `kem.encapsulate(params, payload)`.
/// Assert:  the `w` of `Bb1DepthOneCapsule::Entitlement { w, .. }` encodes
///   equal to `parameter_set.u1`, the product of `u1` by one.
#[test]
fn encapsulate_returns_w_as_u1_for_unit_randomness_under_the_entitlement_scope() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let parameter_set =
                build_bb1_depth_one_parameter_set(&consumed.adapter, Default::default());
            let params = build_encapsulate_params(Default::default());
            let payload = build_encapsulate_payload::<P::Scalar, _>(
                &parameter_set,
                EncapsulatePayloadOverrides {
                    uniform: Some(scalar_bytes::<P>(1)),
                },
            );

            // Act
            let result = kem.encapsulate(params, payload);

            // Assert
            let Ok(success) = result else {
                panic!("the default payload is admitted")
            };
            let Bb1DepthOneCapsule::Entitlement { w, .. } = success.capsule else {
                panic!("the entitlement scope returns an entitlement capsule")
            };
            assert!(
                encoded_g1(&consumed.adapter, w) == encoded_g1(&consumed.adapter, parameter_set.u1)
            );
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `V = t·F`.
/// Arrange: the parameter set under the asset scope fixing the default
///   identity element's `element`, and `uniform` `scalar_bytes(1)`.
/// Act:     `kem.encapsulate(params, payload)`.
/// Assert:  the `v` of `Bb1DepthOneCapsule::Asset { v, .. }` encodes equal to
///   the fixed identity element, the product of `F` by one.
#[test]
fn encapsulate_returns_v_as_the_fixed_element_for_unit_randomness_under_the_asset_scope() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let identity_element =
                build_bb1_depth_one_identity_element(&consumed.adapter, Default::default()).element;
            let parameter_set = build_bb1_depth_one_parameter_set(
                &consumed.adapter,
                Bb1DepthOneParameterSetOverrides {
                    scope: Some(Bb1DepthOneParameterSetScope::Asset {
                        identity_element: identity_element.clone(),
                    }),
                    ..Default::default()
                },
            );
            let params = build_encapsulate_params(Default::default());
            let payload = build_encapsulate_payload::<P::Scalar, _>(
                &parameter_set,
                EncapsulatePayloadOverrides {
                    uniform: Some(scalar_bytes::<P>(1)),
                },
            );

            // Act
            let result = kem.encapsulate(params, payload);

            // Assert
            let Ok(success) = result else {
                panic!("the default payload is admitted")
            };
            let Bb1DepthOneCapsule::Asset { v, .. } = success.capsule else {
                panic!("the asset scope returns an asset capsule")
            };
            assert!(
                encoded_g1(&consumed.adapter, v) == encoded_g1(&consumed.adapter, identity_element)
            );
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `K = e(t·g1, hpub)` encoded by `encode_gt` is the encapsulated
///   value.
/// Arrange: the default parameter set and `uniform` `scalar_bytes(1)`.
/// Act:     `kem.encapsulate(params, payload)`.
/// Assert:  `success.encapsulated.key_material().expose()` equals the
///   `encode_gt` of the pairing of the first-group generator and `hpub`, the
///   one term `g1 × hpub`.
#[test]
fn encapsulate_returns_the_pairing_of_g1_and_hpub_as_the_encapsulated_value_for_unit_randomness() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let parameter_set =
                build_bb1_depth_one_parameter_set(&consumed.adapter, Default::default());
            let params = build_encapsulate_params(Default::default());
            let payload = build_encapsulate_payload::<P::Scalar, _>(
                &parameter_set,
                EncapsulatePayloadOverrides {
                    uniform: Some(scalar_bytes::<P>(1)),
                },
            );

            // Act
            let result = kem.encapsulate(params, payload);

            // Assert
            let Ok(success) = result else {
                panic!("the default payload is admitted")
            };
            let expected = encoded_gt(
                &consumed.adapter,
                vec![build_pairing_product_term(
                    &consumed.adapter,
                    PairingProductTermOverrides {
                        g1: Some(point_g1(&consumed.adapter)),
                        g2: Some(parameter_set.hpub.clone()),
                    },
                )],
            );
            assert_eq!(success.encapsulated.key_material().expose(), &expected);
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: the scope is `Entitlement`, the capsule is `Entitlement
///   { u, v, w }`, and `e(V, g2) = e(u0, U)` and `e(W, g2) = e(u1, U)` hold →
///   `Ok(IsWellFormedSuccessReturn { is_well_formed: true })`.
/// Arrange: the default entitlement parameter set and the capsule for
///   `t = 1`.
/// Act:     `kem.is_well_formed(params, payload)`.
/// Assert:  `success.is_well_formed` holds.
#[test]
fn is_well_formed_accepts_an_entitlement_capsule_of_the_set() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let parameter_set =
                build_bb1_depth_one_parameter_set(&consumed.adapter, Default::default());
            let capsule = Bb1DepthOneCapsule::Entitlement {
                u: point_g2(&consumed.adapter),
                v: parameter_set.u0.clone(),
                w: parameter_set.u1.clone(),
            };
            let params = build_is_well_formed_params(Default::default());
            let payload =
                build_is_well_formed_payload(&parameter_set, &capsule, Default::default());

            // Act
            let result = kem.is_well_formed(params, payload);

            // Assert
            let Ok(success) = result;
            assert!(success.is_well_formed);
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `e(V, g2) = e(u0, U)` fails → `Ok(IsWellFormedSuccessReturn
///   { is_well_formed: false })`.
/// Arrange: the capsule for `t = 1` with `v` the first-group generator, a
///   point other than the set's `u0`.
/// Act:     `kem.is_well_formed(params, payload)`.
/// Assert:  `success.is_well_formed` does not hold.
#[test]
fn is_well_formed_rejects_an_entitlement_capsule_with_another_v() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let parameter_set =
                build_bb1_depth_one_parameter_set(&consumed.adapter, Default::default());
            let capsule = Bb1DepthOneCapsule::Entitlement {
                u: point_g2(&consumed.adapter),
                v: point_g1(&consumed.adapter),
                w: parameter_set.u1.clone(),
            };
            let params = build_is_well_formed_params(Default::default());
            let payload =
                build_is_well_formed_payload(&parameter_set, &capsule, Default::default());

            // Act
            let result = kem.is_well_formed(params, payload);

            // Assert
            let Ok(success) = result;
            assert!(!success.is_well_formed);
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `e(W, g2) = e(u1, U)` fails → `Ok(IsWellFormedSuccessReturn
///   { is_well_formed: false })`.
/// Arrange: the capsule for `t = 1` with `w` the first-group generator, a
///   point other than the set's `u1`.
/// Act:     `kem.is_well_formed(params, payload)`.
/// Assert:  `success.is_well_formed` does not hold.
#[test]
fn is_well_formed_rejects_an_entitlement_capsule_with_another_w() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let parameter_set =
                build_bb1_depth_one_parameter_set(&consumed.adapter, Default::default());
            let capsule = Bb1DepthOneCapsule::Entitlement {
                u: point_g2(&consumed.adapter),
                v: parameter_set.u0.clone(),
                w: point_g1(&consumed.adapter),
            };
            let params = build_is_well_formed_params(Default::default());
            let payload =
                build_is_well_formed_payload(&parameter_set, &capsule, Default::default());

            // Act
            let result = kem.is_well_formed(params, payload);

            // Assert
            let Ok(success) = result;
            assert!(!success.is_well_formed);
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: the scope is `Asset { identity_element }`, the capsule is
///   `Asset { u, v }`, and `e(V, g2) = e(F, U)` holds → `Ok` holding `true`.
/// Arrange: the parameter set under the asset scope fixing the default
///   identity element's `element`, and the capsule for `t = 1`.
/// Act:     `kem.is_well_formed(params, payload)`.
/// Assert:  `success.is_well_formed` holds.
#[test]
fn is_well_formed_accepts_an_asset_capsule_of_the_set() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let identity_element =
                build_bb1_depth_one_identity_element(&consumed.adapter, Default::default()).element;
            let parameter_set = build_bb1_depth_one_parameter_set(
                &consumed.adapter,
                Bb1DepthOneParameterSetOverrides {
                    scope: Some(Bb1DepthOneParameterSetScope::Asset {
                        identity_element: identity_element.clone(),
                    }),
                    ..Default::default()
                },
            );
            let capsule = Bb1DepthOneCapsule::Asset {
                u: point_g2(&consumed.adapter),
                v: identity_element,
            };
            let params = build_is_well_formed_params(Default::default());
            let payload =
                build_is_well_formed_payload(&parameter_set, &capsule, Default::default());

            // Act
            let result = kem.is_well_formed(params, payload);

            // Assert
            let Ok(success) = result;
            assert!(success.is_well_formed);
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `e(V, g2) = e(F, U)` fails → `Ok` holding `false`.
/// Arrange: the asset capsule for `t = 1` with `v` the first-group generator,
///   a point other than `F`.
/// Act:     `kem.is_well_formed(params, payload)`.
/// Assert:  `success.is_well_formed` does not hold.
#[test]
fn is_well_formed_rejects_an_asset_capsule_with_another_v() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let identity_element =
                build_bb1_depth_one_identity_element(&consumed.adapter, Default::default()).element;
            let parameter_set = build_bb1_depth_one_parameter_set(
                &consumed.adapter,
                Bb1DepthOneParameterSetOverrides {
                    scope: Some(Bb1DepthOneParameterSetScope::Asset { identity_element }),
                    ..Default::default()
                },
            );
            let capsule = Bb1DepthOneCapsule::Asset {
                u: point_g2(&consumed.adapter),
                v: point_g1(&consumed.adapter),
            };
            let params = build_is_well_formed_params(Default::default());
            let payload =
                build_is_well_formed_payload(&parameter_set, &capsule, Default::default());

            // Act
            let result = kem.is_well_formed(params, payload);

            // Assert
            let Ok(success) = result;
            assert!(!success.is_well_formed);
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: the capsule's variant is not the parameter set's scope →
///   `Ok(IsWellFormedSuccessReturn { is_well_formed: false })`, with no
///   pairing check.
/// Arrange: the default entitlement parameter set and an asset capsule over
///   the set's own `g2` and `g1`, a verdict the variant alone decides,
///   whatever the elements.
/// Act:     `kem.is_well_formed(params, payload)`.
/// Assert:  `success.is_well_formed` does not hold.
#[test]
fn is_well_formed_rejects_an_asset_capsule_under_an_entitlement_set() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let parameter_set =
                build_bb1_depth_one_parameter_set(&consumed.adapter, Default::default());
            let capsule = Bb1DepthOneCapsule::Asset {
                u: parameter_set.g2.clone(),
                v: parameter_set.g1.clone(),
            };
            let params = build_is_well_formed_params(Default::default());
            let payload =
                build_is_well_formed_payload(&parameter_set, &capsule, Default::default());

            // Act
            let result = kem.is_well_formed(params, payload);

            // Assert
            let Ok(success) = result;
            assert!(!success.is_well_formed);
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: the capsule's variant is not the parameter set's scope → `Ok`
///   holding `false`.
/// Arrange: the asset parameter set of the asset-capsule acceptance block and
///   an entitlement capsule.
/// Act:     `kem.is_well_formed(params, payload)`.
/// Assert:  `success.is_well_formed` does not hold.
#[test]
fn is_well_formed_rejects_an_entitlement_capsule_under_an_asset_set() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let parameter_set = build_bb1_depth_one_parameter_set(
                &consumed.adapter,
                Bb1DepthOneParameterSetOverrides {
                    scope: Some(Bb1DepthOneParameterSetScope::Asset {
                        identity_element: build_bb1_depth_one_identity_element(
                            &consumed.adapter,
                            Default::default(),
                        )
                        .element,
                    }),
                    ..Default::default()
                },
            );
            let capsule = Bb1DepthOneCapsule::Entitlement {
                u: point_g2(&consumed.adapter),
                v: point_g1(&consumed.adapter),
                w: point_g1(&consumed.adapter),
            };
            let params = build_is_well_formed_params(Default::default());
            let payload =
                build_is_well_formed_payload(&parameter_set, &capsule, Default::default());

            // Act
            let result = kem.is_well_formed(params, payload);

            // Assert
            let Ok(success) = result;
            assert!(!success.is_well_formed);
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: the capsule is `Entitlement { u, v, w }` →
///   `Ok(DecapsulateSuccessReturn { encapsulated })` holding `K = e(A, U) /
///   e(V + I·W, B)`.
/// Arrange: the default identity element and credential, the default
///   parameter set, and the capsule for `t = 1`.
/// Act:     `kem.decapsulate(params, payload)`.
/// Assert:  `success.encapsulated.key_material().expose()` equals the
///   `encode_gt` of the pairing of the first-group generator and `hpub`, the
///   construction's `K = e(t·g1, hpub)` for `t = 1`, reached by the public-key
///   route and not by the formula the subject computes.
#[test]
fn decapsulate_recovers_the_pairing_of_g1_and_hpub_from_an_entitlement_capsule_for_unit_randomness()
{
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let parameter_set =
                build_bb1_depth_one_parameter_set(&consumed.adapter, Default::default());
            let identity_element =
                build_bb1_depth_one_identity_element(&consumed.adapter, Default::default());
            let credential = build_bb1_depth_one_credential(&consumed.adapter, Default::default());
            let capsule = Bb1DepthOneCapsule::Entitlement {
                u: point_g2(&consumed.adapter),
                v: parameter_set.u0.clone(),
                w: parameter_set.u1.clone(),
            };
            let params = build_decapsulate_params(Default::default());
            let payload = build_decapsulate_payload(
                &identity_element,
                &credential,
                &capsule,
                Default::default(),
            );

            // Act
            let result = kem.decapsulate(params, payload);

            // Assert
            let Ok(success) = result;
            let expected = encoded_gt(
                &consumed.adapter,
                vec![build_pairing_product_term(
                    &consumed.adapter,
                    PairingProductTermOverrides {
                        g1: Some(point_g1(&consumed.adapter)),
                        g2: Some(parameter_set.hpub.clone()),
                    },
                )],
            );
            assert_eq!(success.encapsulated.key_material().expose(), &expected);
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: the capsule is `Asset { u, v }` →
///   `Ok(DecapsulateSuccessReturn { encapsulated })` holding `K = e(A, U) /
///   e(V, B)`.
/// Arrange: the same defaults and the asset capsule for `t = 1` under the
///   fixed element the default identity element holds.
/// Act:     `kem.decapsulate(params, payload)`.
/// Assert:  as the entitlement-capsule block above.
#[test]
fn decapsulate_recovers_the_pairing_of_g1_and_hpub_from_an_asset_capsule_for_unit_randomness() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let parameter_set =
                build_bb1_depth_one_parameter_set(&consumed.adapter, Default::default());
            let identity_element =
                build_bb1_depth_one_identity_element(&consumed.adapter, Default::default());
            let credential = build_bb1_depth_one_credential(&consumed.adapter, Default::default());
            let capsule = Bb1DepthOneCapsule::Asset {
                u: point_g2(&consumed.adapter),
                v: identity_element.element.clone(),
            };
            let params = build_decapsulate_params(Default::default());
            let payload = build_decapsulate_payload(
                &identity_element,
                &credential,
                &capsule,
                Default::default(),
            );

            // Act
            let result = kem.decapsulate(params, payload);

            // Assert
            let Ok(success) = result;
            let expected = encoded_gt(
                &consumed.adapter,
                vec![build_pairing_product_term(
                    &consumed.adapter,
                    PairingProductTermOverrides {
                        g1: Some(point_g1(&consumed.adapter)),
                        g2: Some(parameter_set.hpub.clone()),
                    },
                )],
            );
            assert_eq!(success.encapsulated.key_material().expose(), &expected);
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}
/// Contract: `parameter_set_components` maps
///   `Bb1DepthOneParameterSetScope::Entitlement` to
///   `ParameterSetScopeComponents::Entitlement`.
/// Arrange: the default entitlement parameter set.
/// Act:     `kem.parameter_set_components(params, payload)`.
/// Assert:  `success.components.scope` matches
///   `ParameterSetScopeComponents::Entitlement`.
#[test]
fn parameter_set_components_maps_the_entitlement_scope_to_the_entitlement_components() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let parameter_set =
                build_bb1_depth_one_parameter_set(&consumed.adapter, Default::default());
            let params = build_parameter_set_components_params(Default::default());
            let payload =
                build_parameter_set_components_payload(&parameter_set, Default::default());

            // Act
            let result = kem.parameter_set_components(params, payload);

            // Assert
            let Ok(success) = result;
            assert!(matches!(
                success.components.scope,
                ParameterSetScopeComponents::Entitlement
            ));
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `Asset { identity_element }` maps to
///   `ParameterSetScopeComponents::Asset { identity_element }`.
/// Arrange: the parameter set under the asset scope fixing the default
///   identity element's `element`.
/// Act:     `kem.parameter_set_components(params, payload)`.
/// Assert:  `success.components.scope` matches
///   `ParameterSetScopeComponents::Asset { .. }`.
#[test]
fn parameter_set_components_maps_the_asset_scope_to_the_asset_components() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let parameter_set = build_bb1_depth_one_parameter_set(
                &consumed.adapter,
                Bb1DepthOneParameterSetOverrides {
                    scope: Some(Bb1DepthOneParameterSetScope::Asset {
                        identity_element: build_bb1_depth_one_identity_element(
                            &consumed.adapter,
                            Default::default(),
                        )
                        .element,
                    }),
                    ..Default::default()
                },
            );
            let params = build_parameter_set_components_params(Default::default());
            let payload =
                build_parameter_set_components_payload(&parameter_set, Default::default());

            // Act
            let result = kem.parameter_set_components(params, payload);

            // Assert
            let Ok(success) = result;
            assert!(matches!(
                success.components.scope,
                ParameterSetScopeComponents::Asset { .. }
            ));
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `ParameterSetScopeComponents::Entitlement` maps back to
///   `Bb1DepthOneParameterSetScope::Entitlement`.
/// Arrange: `components` from `build_parameter_set_components` with no
///   override, whose default scope is the entitlement components.
/// Act:     `kem.parameter_set_from_components(params, payload)`.
/// Assert:  `success.parameter_set.scope` matches
///   `Bb1DepthOneParameterSetScope::Entitlement`.
#[test]
fn parameter_set_from_components_maps_the_entitlement_components_back() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let components = build_parameter_set_components(&consumed.adapter, Default::default());
            let params = build_parameter_set_from_components_params(Default::default());
            let payload = build_parameter_set_from_components_payload(
                &consumed.adapter,
                ParameterSetFromComponentsPayloadOverrides {
                    components: Some(components),
                },
            );

            // Act
            let result = kem.parameter_set_from_components(params, payload);

            // Assert
            let Ok(success) = result;
            assert!(matches!(
                success.parameter_set.scope,
                Bb1DepthOneParameterSetScope::Entitlement
            ));
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `ParameterSetScopeComponents::Asset { identity_element }` maps
///   back to `Bb1DepthOneParameterSetScope::Asset { identity_element }`.
/// Arrange: `components` from `build_parameter_set_components` with `scope`
///   the asset components of the default identity element's `element`.
/// Act:     `kem.parameter_set_from_components(params, payload)`.
/// Assert:  `success.parameter_set.scope` matches
///   `Bb1DepthOneParameterSetScope::Asset { .. }`.
#[test]
fn parameter_set_from_components_maps_the_asset_components_back() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let components = build_parameter_set_components(
                &consumed.adapter,
                ParameterSetComponentsOverrides {
                    scope: Some(ParameterSetScopeComponents::Asset {
                        identity_element: build_bb1_depth_one_identity_element(
                            &consumed.adapter,
                            Default::default(),
                        )
                        .element,
                    }),
                    ..Default::default()
                },
            );
            let params = build_parameter_set_from_components_params(Default::default());
            let payload = build_parameter_set_from_components_payload(
                &consumed.adapter,
                ParameterSetFromComponentsPayloadOverrides {
                    components: Some(components),
                },
            );

            // Act
            let result = kem.parameter_set_from_components(params, payload);

            // Assert
            let Ok(success) = result;
            assert!(matches!(
                success.parameter_set.scope,
                Bb1DepthOneParameterSetScope::Asset { .. }
            ));
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `capsule_components` returns the elements of
///   `Bb1DepthOneCapsule::Entitlement { u, v, w }` in
///   `CapsuleComponents::Entitlement { u, v, w }`.
/// Arrange: the default parameter set and an entitlement capsule over the
///   set's elements.
/// Act:     `kem.capsule_components(params, payload)`.
/// Assert:  `success.components` matches `CapsuleComponents::Entitlement
///   { .. }`.
#[test]
fn capsule_components_maps_the_entitlement_capsule_to_the_entitlement_components() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let parameter_set =
                build_bb1_depth_one_parameter_set(&consumed.adapter, Default::default());
            let capsule = Bb1DepthOneCapsule::Entitlement {
                u: parameter_set.g2.clone(),
                v: parameter_set.g1.clone(),
                w: parameter_set.g1.clone(),
            };
            let params = build_capsule_components_params(Default::default());
            let payload = build_capsule_components_payload(&capsule, Default::default());

            // Act
            let result = kem.capsule_components(params, payload);

            // Assert
            let Ok(success) = result;
            assert!(matches!(
                success.components,
                CapsuleComponents::Entitlement { .. }
            ));
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: the asset capsule maps to `CapsuleComponents::Asset { u, v }`.
/// Arrange: the default parameter set and an asset capsule over the set's
///   elements.
/// Act:     `kem.capsule_components(params, payload)`.
/// Assert:  `success.components` matches `CapsuleComponents::Asset { .. }`.
#[test]
fn capsule_components_maps_the_asset_capsule_to_the_asset_components() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let parameter_set =
                build_bb1_depth_one_parameter_set(&consumed.adapter, Default::default());
            let capsule = Bb1DepthOneCapsule::Asset {
                u: parameter_set.g2.clone(),
                v: parameter_set.g1.clone(),
            };
            let params = build_capsule_components_params(Default::default());
            let payload = build_capsule_components_payload(&capsule, Default::default());

            // Act
            let result = kem.capsule_components(params, payload);

            // Assert
            let Ok(success) = result;
            assert!(matches!(
                success.components,
                CapsuleComponents::Asset { .. }
            ));
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `capsule_from_components` returns the `Bb1DepthOneCapsule`
///   variant of the components' variant.
/// Arrange: `components` the entitlement components over the default
///   parameter set's elements.
/// Act:     `kem.capsule_from_components(params, payload)`.
/// Assert:  `success.capsule` matches `Bb1DepthOneCapsule::Entitlement
///   { .. }`.
#[test]
fn capsule_from_components_maps_the_entitlement_components_to_the_entitlement_capsule() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let parameter_set =
                build_bb1_depth_one_parameter_set(&consumed.adapter, Default::default());
            let params = build_capsule_from_components_params(Default::default());
            let payload = build_capsule_from_components_payload(
                &consumed.adapter,
                CapsuleFromComponentsPayloadOverrides {
                    components: Some(CapsuleComponents::Entitlement {
                        u: parameter_set.g2.clone(),
                        v: parameter_set.g1.clone(),
                        w: parameter_set.g1.clone(),
                    }),
                },
            );

            // Act
            let result = kem.capsule_from_components(params, payload);

            // Assert
            let Ok(success) = result;
            assert!(matches!(
                success.capsule,
                Bb1DepthOneCapsule::Entitlement { .. }
            ));
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: the asset components map to `Bb1DepthOneCapsule::Asset
///   { u, v }`.
/// Arrange: `components` the asset components over the default parameter
///   set's elements.
/// Act:     `kem.capsule_from_components(params, payload)`.
/// Assert:  `success.capsule` matches `Bb1DepthOneCapsule::Asset { .. }`.
#[test]
fn capsule_from_components_maps_the_asset_components_to_the_asset_capsule() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let parameter_set =
                build_bb1_depth_one_parameter_set(&consumed.adapter, Default::default());
            let params = build_capsule_from_components_params(Default::default());
            let payload = build_capsule_from_components_payload(
                &consumed.adapter,
                CapsuleFromComponentsPayloadOverrides {
                    components: Some(CapsuleComponents::Asset {
                        u: parameter_set.g2.clone(),
                        v: parameter_set.g1.clone(),
                    }),
                },
            );

            // Act
            let result = kem.capsule_from_components(params, payload);

            // Assert
            let Ok(success) = result;
            assert!(matches!(success.capsule, Bb1DepthOneCapsule::Asset { .. }));
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `credential_components` returns `CredentialComponents { a, b }`,
///   clones of the credential's `a` and `b`.
/// Arrange: the default credential.
/// Act:     `kem.credential_components(params, payload)`.
/// Assert:  `success.components.a` encodes equal to `credential.a` and
///   `success.components.b` to `credential.b`, the field-to-slot mapping,
///   `a` and `b` being points of different groups.
#[test]
fn credential_components_returns_the_a_and_b_of_the_credential() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let credential = build_bb1_depth_one_credential(&consumed.adapter, Default::default());
            let params = build_credential_components_params(Default::default());
            let payload = build_credential_components_payload(&credential, Default::default());

            // Act
            let result = kem.credential_components(params, payload);

            // Assert
            let Ok(success) = result;
            assert!(
                encoded_g1(&consumed.adapter, success.components.a)
                    == encoded_g1(&consumed.adapter, credential.a.clone())
            );
            assert!(
                encoded_g2(&consumed.adapter, success.components.b)
                    == encoded_g2(&consumed.adapter, credential.b.clone())
            );
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `credential_from_components` returns `Bb1DepthOneCredential
///   { a, b }`, the components moved.
/// Arrange: `components` from `build_credential_components` with `a` the
///   set's `u0` and `b` its `hpub`, points other than the builder's default
///   generators.
/// Act:     `kem.credential_from_components(params, payload)`.
/// Assert:  `success.credential.a` encodes equal to `parameter_set.u0` and
///   `success.credential.b` to `parameter_set.hpub`.
#[test]
fn credential_from_components_moves_the_components_into_the_credential() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let parameter_set =
                build_bb1_depth_one_parameter_set(&consumed.adapter, Default::default());
            let components = build_credential_components(
                &consumed.adapter,
                CredentialComponentsOverrides {
                    a: Some(parameter_set.u0.clone()),
                    b: Some(parameter_set.hpub.clone()),
                },
            );
            let params = build_credential_from_components_params(Default::default());
            let payload = build_credential_from_components_payload(
                &consumed.adapter,
                CredentialFromComponentsPayloadOverrides {
                    components: Some(components),
                },
            );

            // Act
            let result = kem.credential_from_components(params, payload);

            // Assert
            let Ok(success) = result;
            assert!(
                encoded_g1(&consumed.adapter, success.credential.a)
                    == encoded_g1(&consumed.adapter, parameter_set.u0)
            );
            assert!(
                encoded_g2(&consumed.adapter, success.credential.b)
                    == encoded_g2(&consumed.adapter, parameter_set.hpub)
            );
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `parameter_set_components` returns clones of `g1`, `u0`, `u1`,
///   `g2`, and `hpub`.
/// Arrange: the default parameter set, whose `g1`, `u0`, and `u1` are
///   distinct points.
/// Act:     `kem.parameter_set_components(params, payload)`.
/// Assert:  each of `g1`, `u0`, and `u1` of `success.components` encodes equal
///   to the like-named field of the set by `encoded_g1`, and each of `g2` and
///   `hpub` by `encoded_g2`.
#[test]
fn parameter_set_components_returns_the_fields_of_the_parameter_set() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let parameter_set =
                build_bb1_depth_one_parameter_set(&consumed.adapter, Default::default());
            let params = build_parameter_set_components_params(Default::default());
            let payload =
                build_parameter_set_components_payload(&parameter_set, Default::default());

            // Act
            let result = kem.parameter_set_components(params, payload);

            // Assert
            let Ok(success) = result;
            assert!(
                encoded_g1(&consumed.adapter, success.components.g1)
                    == encoded_g1(&consumed.adapter, parameter_set.g1)
            );
            assert!(
                encoded_g1(&consumed.adapter, success.components.u0)
                    == encoded_g1(&consumed.adapter, parameter_set.u0)
            );
            assert!(
                encoded_g1(&consumed.adapter, success.components.u1)
                    == encoded_g1(&consumed.adapter, parameter_set.u1)
            );
            assert!(
                encoded_g2(&consumed.adapter, success.components.g2)
                    == encoded_g2(&consumed.adapter, parameter_set.g2)
            );
            assert!(
                encoded_g2(&consumed.adapter, success.components.hpub)
                    == encoded_g2(&consumed.adapter, parameter_set.hpub)
            );
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `parameter_set_from_components` returns the parameter set
///   holding the components moved.
/// Arrange: `components` from `build_parameter_set_components` with no
///   override, whose `g1`, `u0`, and `u1` are distinct points.
/// Act:     `kem.parameter_set_from_components(params, payload)`.
/// Assert:  each of `g1`, `u0`, `u1`, `g2`, and `hpub` of
///   `success.parameter_set` encodes equal to the like-named field of those
///   components.
#[test]
fn parameter_set_from_components_moves_the_components_into_the_parameter_set() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let components = build_parameter_set_components(&consumed.adapter, Default::default());
            let g1 = components.g1.clone();
            let u0 = components.u0.clone();
            let u1 = components.u1.clone();
            let g2 = components.g2.clone();
            let hpub = components.hpub.clone();
            let params = build_parameter_set_from_components_params(Default::default());
            let payload = build_parameter_set_from_components_payload(
                &consumed.adapter,
                ParameterSetFromComponentsPayloadOverrides {
                    components: Some(components),
                },
            );

            // Act
            let result = kem.parameter_set_from_components(params, payload);

            // Assert
            let Ok(success) = result;
            assert!(
                encoded_g1(&consumed.adapter, success.parameter_set.g1)
                    == encoded_g1(&consumed.adapter, g1)
            );
            assert!(
                encoded_g1(&consumed.adapter, success.parameter_set.u0)
                    == encoded_g1(&consumed.adapter, u0)
            );
            assert!(
                encoded_g1(&consumed.adapter, success.parameter_set.u1)
                    == encoded_g1(&consumed.adapter, u1)
            );
            assert!(
                encoded_g2(&consumed.adapter, success.parameter_set.g2)
                    == encoded_g2(&consumed.adapter, g2)
            );
            assert!(
                encoded_g2(&consumed.adapter, success.parameter_set.hpub)
                    == encoded_g2(&consumed.adapter, hpub)
            );
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `identity_element_components` returns clones of `scalar` and
///   `element`.
/// Arrange: the default identity element.
/// Act:     `kem.identity_element_components(params, payload)`.
/// Assert:  `success.components.scalar` encodes equal to
///   `identity_element.scalar` by `encoded_scalar` and
///   `success.components.element` to `identity_element.element` by
///   `encoded_g1`.
#[test]
fn identity_element_components_returns_the_scalar_and_the_element() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let identity_element =
                build_bb1_depth_one_identity_element(&consumed.adapter, Default::default());
            let params = build_identity_element_components_params(Default::default());
            let payload =
                build_identity_element_components_payload(&identity_element, Default::default());

            // Act
            let result = kem.identity_element_components(params, payload);

            // Assert
            let Ok(success) = result;
            assert!(
                encoded_scalar(&consumed.adapter, success.components.scalar)
                    == encoded_scalar(&consumed.adapter, identity_element.scalar.clone())
            );
            assert!(
                encoded_g1(&consumed.adapter, success.components.element)
                    == encoded_g1(&consumed.adapter, identity_element.element.clone())
            );
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `master_scalar_components` returns `MasterScalarComponents
///   { value }`, `value` a new `Secret` of a clone of the master scalar's
///   value.
/// Arrange: the default master scalar.
/// Act:     `kem.master_scalar_components(params, payload)`.
/// Assert:  `success.components.value.expose()` encodes equal to
///   `master_scalar.value.expose()`.
#[test]
fn master_scalar_components_returns_a_secret_of_the_value() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let master_scalar = build_bb1_depth_one_master_scalar(Default::default());
            let params = build_master_scalar_components_params(Default::default());
            let payload =
                build_master_scalar_components_payload(&master_scalar, Default::default());

            // Act
            let result = kem.master_scalar_components(params, payload);

            // Assert
            let Ok(success) = result;
            assert!(
                encoded_scalar(&consumed.adapter, success.components.value.expose().clone())
                    == encoded_scalar(&consumed.adapter, master_scalar.value.expose().clone())
            );
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `master_scalar_from_components` returns
///   `Bb1DepthOneMasterScalar { value }`, the `Secret` moved.
/// Arrange: `components` from `build_master_scalar_components` with `value` a
///   `Secret` of `scalar(3)`, a scalar other than the builder's default.
/// Act:     `kem.master_scalar_from_components(params, payload)`.
/// Assert:  `success.master_scalar.value.expose()` encodes equal to
///   `scalar(3)`.
#[test]
fn master_scalar_from_components_moves_the_secret_into_the_master_scalar() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let components =
                build_master_scalar_components::<P::Scalar>(MasterScalarComponentsOverrides {
                    value: Some(build_secret(scalar::<P>(3), Default::default())),
                });
            let params = build_master_scalar_from_components_params(Default::default());
            let payload = build_master_scalar_from_components_payload::<P::Scalar>(
                MasterScalarFromComponentsPayloadOverrides {
                    components: Some(components),
                },
            );

            // Act
            let result = kem.master_scalar_from_components(params, payload);

            // Assert
            let Ok(success) = result;
            assert!(
                encoded_scalar(
                    &consumed.adapter,
                    success.master_scalar.value.expose().clone()
                ) == encoded_scalar(&consumed.adapter, scalar::<P>(3))
            );
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `capsule_components` returns clones of `u`, `v`, and `w` in
///   `CapsuleComponents::Entitlement { u, v, w }`.
/// Arrange: the default parameter set and the entitlement capsule for
///   `t = 1`.
/// Act:     `kem.capsule_components(params, payload)`.
/// Assert:  `success.components` matches `CapsuleComponents::Entitlement
///   { u, v, w }` and `u`, `v`, and `w` encode equal to the second-group
///   generator, `parameter_set.u0`, and `parameter_set.u1`.
#[test]
fn capsule_components_returns_the_entitlement_elements_in_their_slots() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let parameter_set =
                build_bb1_depth_one_parameter_set(&consumed.adapter, Default::default());
            let capsule = Bb1DepthOneCapsule::Entitlement {
                u: point_g2(&consumed.adapter),
                v: parameter_set.u0.clone(),
                w: parameter_set.u1.clone(),
            };
            let params = build_capsule_components_params(Default::default());
            let payload = build_capsule_components_payload(&capsule, Default::default());

            // Act
            let result = kem.capsule_components(params, payload);

            // Assert
            let Ok(success) = result;
            let CapsuleComponents::Entitlement { u, v, w } = success.components else {
                panic!("an entitlement capsule returns entitlement components")
            };
            assert!(
                encoded_g2(&consumed.adapter, u)
                    == encoded_g2(&consumed.adapter, point_g2(&consumed.adapter))
            );
            assert!(
                encoded_g1(&consumed.adapter, v) == encoded_g1(&consumed.adapter, parameter_set.u0)
            );
            assert!(
                encoded_g1(&consumed.adapter, w) == encoded_g1(&consumed.adapter, parameter_set.u1)
            );
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `capsule_from_components` returns
///   `Bb1DepthOneCapsule::Entitlement { u, v, w }` holding the components
///   moved.
/// Arrange: `components` the entitlement components of the `t = 1` capsule.
/// Act:     `kem.capsule_from_components(params, payload)`.
/// Assert:  `success.capsule` matches `Bb1DepthOneCapsule::Entitlement
///   { u, v, w }` and `u`, `v`, and `w` encode equal to the second-group
///   generator, `parameter_set.u0`, and `parameter_set.u1`.
#[test]
fn capsule_from_components_moves_the_entitlement_elements_into_their_slots() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let params = Default::default();
            let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(params),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the default selection constructs the hash-to-scalar mock concrete")
            };
            let kem =
                build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
            let parameter_set =
                build_bb1_depth_one_parameter_set(&consumed.adapter, Default::default());
            let params = build_capsule_from_components_params(Default::default());
            let payload = build_capsule_from_components_payload(
                &consumed.adapter,
                CapsuleFromComponentsPayloadOverrides {
                    components: Some(CapsuleComponents::Entitlement {
                        u: point_g2(&consumed.adapter),
                        v: parameter_set.u0.clone(),
                        w: parameter_set.u1.clone(),
                    }),
                },
            );

            // Act
            let result = kem.capsule_from_components(params, payload);

            // Assert
            let Ok(success) = result;
            let Bb1DepthOneCapsule::Entitlement { u, v, w } = success.capsule else {
                panic!("entitlement components return an entitlement capsule")
            };
            assert!(
                encoded_g2(&consumed.adapter, u)
                    == encoded_g2(&consumed.adapter, point_g2(&consumed.adapter))
            );
            assert!(
                encoded_g1(&consumed.adapter, v) == encoded_g1(&consumed.adapter, parameter_set.u0)
            );
            assert!(
                encoded_g1(&consumed.adapter, w) == encoded_g1(&consumed.adapter, parameter_set.u1)
            );
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}
