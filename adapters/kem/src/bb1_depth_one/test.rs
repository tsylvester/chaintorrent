#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::interface::{
    Bb1DepthOneCapsule, Bb1DepthOneCasePayload, Bb1DepthOneKem, Bb1DepthOneParameterSetScope,
    IBb1DepthOneCaseConsumer,
};
use super::mock::{
    Bb1DepthOneCaseParamsOverrides, Bb1DepthOneCredentialOverrides,
    Bb1DepthOneParameterSetOverrides, build_bb1_depth_one_case_deps,
    build_bb1_depth_one_case_params, build_bb1_depth_one_case_payload,
    build_bb1_depth_one_credential, build_bb1_depth_one_identity_element, build_bb1_depth_one_kem,
    build_bb1_depth_one_kem_constructor_params, build_bb1_depth_one_master_scalar,
    build_bb1_depth_one_parameter_set, run_bb1_depth_one_case,
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
use domain::{Secret, build_secret};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, HashToScalarConcrete, HashToScalarErrorReturn,
    IHashToScalarAdapter, MockIHashToScalarAdapterFailureMode, build_create_hash_to_scalar_params,
};
use hashing::{RootConstructorParamsOverrides, build_root};
use pairing::{
    EncodeG1Params, EncodeG1PayloadOverrides, EncodeG2Params, EncodeG2PayloadOverrides,
    EncodeGtParams, EncodeGtPayloadOverrides, EncodeScalarParams, EncodeScalarPayloadOverrides,
    IPairingAdapter, IPairingArithmetic, IPairingReference, ISampleUniformScalar, MulG1Params,
    MulG1PayloadOverrides, MulG2Params, MulG2PayloadOverrides, PairingProductParams,
    PairingProductPayloadOverrides, PairingProductTerm, PairingProductTermOverrides,
    SampleUniformScalarErrorReturn, SampleUniformScalarParams, SampleUniformScalarPayloadOverrides,
    build_encode_g1_payload, build_encode_g2_payload, build_encode_gt_payload,
    build_encode_scalar_payload, build_g1_generator_success_return,
    build_g2_generator_success_return, build_mul_g1_payload, build_mul_g2_payload,
    build_pairing_product_payload, build_pairing_product_term, build_sample_uniform_scalar_payload,
};

fn scalar_bytes<S: ISampleUniformScalar>(value: u8) -> Secret<Vec<u8>> {
    let mut bytes = vec![0u8; S::UNIFORM_BYTES_LENGTH];
    bytes[S::UNIFORM_BYTES_LENGTH - 1] = value;
    build_secret(bytes, Default::default())
}

fn shortened<S: ISampleUniformScalar>() -> Secret<Vec<u8>> {
    build_secret(vec![0u8; S::UNIFORM_BYTES_LENGTH - 1], Default::default())
}

fn scalar<P: IPairingAdapter>(value: u8) -> P::Scalar {
    P::Scalar::sample_from_uniform_bytes(
        SampleUniformScalarParams,
        build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
            uniform: Some(scalar_bytes::<P::Scalar>(value)),
        }),
    )
    .expect("uniform bytes of the declared length sample a scalar")
    .scalar
    .expose()
    .clone()
}

fn point_g1<P: IPairingAdapter>(pairing: &P) -> P::G1 {
    build_g1_generator_success_return(pairing, Default::default()).point
}

fn point_g2<P: IPairingAdapter>(pairing: &P) -> P::G2 {
    build_g2_generator_success_return(pairing, Default::default()).point
}

fn identity_g1<P: IPairingAdapter>(pairing: &P) -> P::G1 {
    let Ok(success) = pairing.mul_g1(
        MulG1Params,
        build_mul_g1_payload(
            pairing,
            MulG1PayloadOverrides {
                scalar: Some(scalar::<P>(0)),
                ..Default::default()
            },
        ),
    );
    success.product
}

fn identity_g2<P: IPairingAdapter>(pairing: &P) -> P::G2 {
    let Ok(success) = pairing.mul_g2(
        MulG2Params,
        build_mul_g2_payload(
            pairing,
            MulG2PayloadOverrides {
                scalar: Some(scalar::<P>(0)),
                ..Default::default()
            },
        ),
    );
    success.product
}

fn encoded_g1<P: IPairingAdapter>(pairing: &P, point: P::G1) -> P::EncodedG1 {
    let Ok(success) = pairing.encode_g1(
        EncodeG1Params,
        build_encode_g1_payload(pairing, EncodeG1PayloadOverrides { point: Some(point) }),
    );
    success.bytes
}

fn encoded_g2<P: IPairingAdapter>(pairing: &P, point: P::G2) -> P::EncodedG2 {
    let Ok(success) = pairing.encode_g2(
        EncodeG2Params,
        build_encode_g2_payload(pairing, EncodeG2PayloadOverrides { point: Some(point) }),
    );
    success.bytes
}

fn encoded_scalar<P: IPairingAdapter>(pairing: &P, scalar: P::Scalar) -> Vec<u8> {
    let Ok(success) = pairing.encode_scalar(
        EncodeScalarParams,
        build_encode_scalar_payload(
            pairing,
            EncodeScalarPayloadOverrides {
                scalar: Some(scalar),
            },
        ),
    );
    success.bytes.expose().as_ref().to_vec()
}

fn encoded_gt<P: IPairingArithmetic>(
    pairing: &P,
    terms: Vec<PairingProductTerm<P::G1, P::G2>>,
) -> Vec<u8> {
    let Ok(product) = pairing.pairing_product(
        PairingProductParams,
        build_pairing_product_payload(
            pairing,
            PairingProductPayloadOverrides { terms: Some(terms) },
        ),
    );
    let Ok(encoded) = pairing.encode_gt(
        EncodeGtParams,
        build_encode_gt_payload(
            pairing,
            EncodeGtPayloadOverrides {
                value: Some(product.product),
            },
        ),
    );
    encoded.bytes.expose().as_ref().to_vec()
}

/// Contract: `Bb1DepthOneKem::try_new`, admitted: the tag constructor succeeds
///   → `Ok(Bb1DepthOneKem { pairing, hash_to_scalar, tag })`, `tag` the
///   `DomainTag` built from `BB1_DEPTH_ONE_IDENTITY_TAG`.
/// Arrange: `build_bb1_depth_one_kem_constructor_params(pairing,
///   hash_to_scalar, Default::default())`; `kem` is not bound, so the
///   constructor is the call under test.
/// Act:     `Bb1DepthOneKem::try_new(params)` inside `Consumer`.
/// Assert:  `assert_eq!(success.tag.as_bytes(),
///   b"ChainTorrent-v1-kem-identity")`; outside the consumer,
///   `created.is_ok()`.
/// Collaborators: the pairing family's mock concrete through `create_pairing`
///   and the hash-to-scalar family's mock concrete through
///   `create_hash_to_scalar`, each by its default params.
#[test]
fn try_new_holds_the_tag_the_declaration_names() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            // Arrange
            let params = build_bb1_depth_one_kem_constructor_params(
                pairing,
                hash_to_scalar,
                Default::default(),
            );

            // Act
            let result = Bb1DepthOneKem::try_new(params);

            // Assert
            let Ok(success) = result else {
                panic!("the declared identity tag is admitted")
            };
            assert_eq!(success.tag.as_bytes(), b"ChainTorrent-v1-kem-identity");
        }
    }

    // Act
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// try_new_holds_the_tag_the_declaration_names

/// Contract: `setup`, sampling refused: the master uniform bytes are refused →
///   `Err(SetupErrorReturn::MasterScalarSampling(error))`, the refusal
///   unchanged.
/// Arrange: `Consumer` whose `kem` is `build_bb1_depth_one_kem`, and
///   `build_setup_payload` with `master_uniform` `shortened`, one byte shorter
///   than `P::Scalar::UNIFORM_BYTES_LENGTH`, so the other samplings succeed.
/// Act:     `kem.setup(params, payload)` inside `Consumer`.
/// Assert:  `assert_eq!` of the whole error against
///   `SetupErrorReturn::MasterScalarSampling(WrongLength { expected:
///   P::Scalar::UNIFORM_BYTES_LENGTH, actual:
///   P::Scalar::UNIFORM_BYTES_LENGTH - 1 })`; outside, `created.is_ok()`.
/// Collaborators: the pairing family's mock concrete through `create_pairing`
///   and the hash-to-scalar family's mock concrete through
///   `create_hash_to_scalar`, each by its default params.
#[test]
fn setup_returns_the_master_sampling_refusal_whole() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let params = build_setup_params(Default::default());
            let payload = build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                master_uniform: Some(shortened::<P::Scalar>()),
                ..Default::default()
            });

            // Act
            let result = kem.setup(params, payload);

            // Assert
            let Err(error) = result else {
                panic!("the shortened master uniform bytes are refused")
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
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// setup_returns_the_master_sampling_refusal_whole

/// Contract: `setup`, sampling refused: the `u0` uniform bytes are refused →
///   `Err(SetupErrorReturn::U0Sampling(error))`, the refusal unchanged.
/// Arrange: as `setup_returns_the_master_sampling_refusal_whole` with
///   `u0_uniform` `shortened` and the master and `u1` bytes default.
/// Act:     `kem.setup(params, payload)` inside `Consumer`.
/// Assert:  `assert_eq!` of the whole error against `U0Sampling` carrying
///   `WrongLength { expected: P::Scalar::UNIFORM_BYTES_LENGTH, actual:
///   P::Scalar::UNIFORM_BYTES_LENGTH - 1 }`; outside, `created.is_ok()`.
/// Collaborators: as `setup_returns_the_master_sampling_refusal_whole`.
#[test]
fn setup_returns_the_u0_sampling_refusal_whole() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let params = build_setup_params(Default::default());
            let payload = build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                u0_uniform: Some(shortened::<P::Scalar>()),
                ..Default::default()
            });

            // Act
            let result = kem.setup(params, payload);

            // Assert
            let Err(error) = result else {
                panic!("the shortened u0 uniform bytes are refused")
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
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// setup_returns_the_u0_sampling_refusal_whole

/// Contract: `setup`, sampling refused: the `u1` uniform bytes are refused →
///   `Err(SetupErrorReturn::U1Sampling(error))`, the refusal unchanged.
/// Arrange: as `setup_returns_the_master_sampling_refusal_whole` with
///   `u1_uniform` `shortened` and the master and `u0` bytes default.
/// Act:     `kem.setup(params, payload)` inside `Consumer`.
/// Assert:  `assert_eq!` of the whole error against `U1Sampling` carrying
///   `WrongLength { expected: P::Scalar::UNIFORM_BYTES_LENGTH, actual:
///   P::Scalar::UNIFORM_BYTES_LENGTH - 1 }`; outside, `created.is_ok()`.
/// Collaborators: as `setup_returns_the_master_sampling_refusal_whole`.
#[test]
fn setup_returns_the_u1_sampling_refusal_whole() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let params = build_setup_params(Default::default());
            let payload = build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                u1_uniform: Some(shortened::<P::Scalar>()),
                ..Default::default()
            });

            // Act
            let result = kem.setup(params, payload);

            // Assert
            let Err(error) = result else {
                panic!("the shortened u1 uniform bytes are refused")
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
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// setup_returns_the_u1_sampling_refusal_whole

/// Contract: `setup`, sampling refused: sampling runs in the order master,
///   `u0`, `u1`, and the refusal of the earliest sampler refused is returned.
/// Arrange: `master_uniform` and `u0_uniform` each `shortened`, so a sampling
///   order that reached `u0` first would return `U0Sampling`.
/// Act:     `kem.setup(params, payload)` inside `Consumer`.
/// Assert:  `assert_eq!` of the whole error against `MasterScalarSampling`
///   carrying `WrongLength { expected: P::Scalar::UNIFORM_BYTES_LENGTH,
///   actual: P::Scalar::UNIFORM_BYTES_LENGTH - 1 }`; outside,
///   `created.is_ok()`.
/// Collaborators: as `setup_returns_the_master_sampling_refusal_whole`.
#[test]
fn setup_returns_the_master_refusal_when_the_master_and_u0_bytes_are_both_refused() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let params = build_setup_params(Default::default());
            let payload = build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                master_uniform: Some(shortened::<P::Scalar>()),
                u0_uniform: Some(shortened::<P::Scalar>()),
                ..Default::default()
            });

            // Act
            let result = kem.setup(params, payload);

            // Assert
            let Err(error) = result else {
                panic!("the shortened uniform bytes are refused")
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
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// setup_returns_the_master_refusal_when_the_master_and_u0_bytes_are_both_refused

/// Contract: `setup`, entitlement scope: `params.scope` is
///   `SetupScope::Entitlement` and every sampling succeeds → `Ok` holding a
///   parameter set whose scope is `Bb1DepthOneParameterSetScope::Entitlement`.
/// Arrange: `build_setup_params(Default::default())`, the entitlement scope,
///   and `build_setup_payload::<P::Scalar>(Default::default())`.
/// Act:     `kem.setup(params, payload)` inside `Consumer`.
/// Assert:  `assert!(matches!(success.parameter_set.scope,
///   Bb1DepthOneParameterSetScope::Entitlement))`; outside, `created.is_ok()`.
/// Collaborators: as `setup_returns_the_master_sampling_refusal_whole`.
#[test]
fn setup_fixes_an_entitlement_scope_set_to_no_identity() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let params = build_setup_params(Default::default());
            let payload = build_setup_payload::<P::Scalar>(Default::default());

            // Act
            let result = kem.setup(params, payload);

            // Assert
            let Ok(success) = result else {
                panic!("the default uniform bytes sample under the entitlement scope")
            };
            assert!(matches!(
                success.parameter_set.scope,
                Bb1DepthOneParameterSetScope::Entitlement
            ));
        }
    }

    // Act
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// setup_fixes_an_entitlement_scope_set_to_no_identity

/// Contract: `setup`, asset scope: `params.scope` is `SetupScope::Asset {
///   root }` and the identity mapping succeeds → `Ok` holding a parameter set
///   whose scope is `Bb1DepthOneParameterSetScope::Asset { identity_element }`.
/// Arrange: `build_setup_params` with `scope: Some(SetupScope::Asset { root:
///   &root })`, `root` from `build_root` with `bytes: Some([0xa1; 32])`, and
///   `build_setup_payload::<P::Scalar>(Default::default())`.
/// Act:     `kem.setup(params, payload)` inside `Consumer`.
/// Assert:  `assert!(matches!(success.parameter_set.scope,
///   Bb1DepthOneParameterSetScope::Asset { .. }))`; outside, `created.is_ok()`.
/// Collaborators: as `setup_returns_the_master_sampling_refusal_whole`.
#[test]
fn setup_fixes_an_asset_scope_set_to_its_identity_element() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let root = build_root(RootConstructorParamsOverrides {
                bytes: Some([0xa1; 32]),
            });
            let params = build_setup_params(SetupParamsOverrides {
                scope: Some(SetupScope::Asset { root: &root }),
            });
            let payload = build_setup_payload::<P::Scalar>(Default::default());

            // Act
            let result = kem.setup(params, payload);

            // Assert
            let Ok(success) = result else {
                panic!("the default uniform bytes sample under the asset scope")
            };
            assert!(matches!(
                success.parameter_set.scope,
                Bb1DepthOneParameterSetScope::Asset { .. }
            ));
        }
    }

    // Act
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// setup_fixes_an_asset_scope_set_to_its_identity_element

/// Contract: `setup`, asset scope refused: `params.scope` is `SetupScope::
///   Asset { root }` and the hash adapter refuses →
///   `Err(SetupErrorReturn::HashToScalar(error))`, the refusal unchanged.
/// Arrange: the case params' `hash_to_scalar` field under the
///   `SamplingRefused` failure mode, the asset scope with `root` from
///   `build_root` with `bytes: Some([0xa1; 32])`, and
///   `build_setup_payload::<P::Scalar>(Default::default())`.
/// Act:     `kem.setup(params, payload)` inside `Consumer`.
/// Assert:  `assert_eq!` of the whole error against
///   `SetupErrorReturn::HashToScalar(HashToScalarErrorReturn::Sampling(
///   WrongLength { expected: P::Scalar::UNIFORM_BYTES_LENGTH, actual: 0 }))`;
///   outside, `created.is_ok()`.
/// Collaborators: the hash-to-scalar family's mock concrete under the
///   `SamplingRefused` failure mode, named by the case params; the pairing
///   family's mock concrete by default params.
#[test]
fn setup_returns_the_hash_refusal_whole_under_the_asset_scope() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let root = build_root(RootConstructorParamsOverrides {
                bytes: Some([0xa1; 32]),
            });
            let params = build_setup_params(SetupParamsOverrides {
                scope: Some(SetupScope::Asset { root: &root }),
            });
            let payload = build_setup_payload::<P::Scalar>(Default::default());

            // Act
            let result = kem.setup(params, payload);

            // Assert
            let Err(error) = result else {
                panic!("the refusing hash adapter refuses the asset-scope mapping")
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
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Bb1DepthOneCaseParamsOverrides {
            hash_to_scalar: Some(build_create_hash_to_scalar_params(
                CreateHashToScalarParamsOverrides {
                    concrete: Some(HashToScalarConcrete::Mock(
                        MockIHashToScalarAdapterFailureMode::SamplingRefused,
                    )),
                    ..Default::default()
                },
            )),
            ..Default::default()
        }),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// setup_returns_the_hash_refusal_whole_under_the_asset_scope

/// Contract: `setup`, asset scope refused: `params.scope` is `SetupScope::
///   Asset { root }` and `F` is the identity element →
///   `Err(SetupErrorReturn::TrivialIdentityElement)`.
/// Arrange: the asset scope as `setup_fixes_an_asset_scope_set_to_its_
///   identity_element` and `build_setup_payload` with `u0_uniform` and
///   `u1_uniform` each `Some(scalar_bytes(0))`, so `u0` and `u1` are the
///   identity element and `F` is it whatever `I` is.
/// Act:     `kem.setup(params, payload)` inside `Consumer`.
/// Assert:  `assert_eq!` of the whole error against
///   `SetupErrorReturn::TrivialIdentityElement`; outside, `created.is_ok()`.
/// Collaborators: as `setup_returns_the_master_sampling_refusal_whole`.
#[test]
fn setup_refuses_a_trivial_identity_element_under_the_asset_scope() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let root = build_root(RootConstructorParamsOverrides {
                bytes: Some([0xa1; 32]),
            });
            let params = build_setup_params(SetupParamsOverrides {
                scope: Some(SetupScope::Asset { root: &root }),
            });
            let payload = build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                u0_uniform: Some(scalar_bytes::<P::Scalar>(0)),
                u1_uniform: Some(scalar_bytes::<P::Scalar>(0)),
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
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// setup_refuses_a_trivial_identity_element_under_the_asset_scope

/// Contract: `derive_identity`, wrong role: the parameter set's scope is
///   `Entitlement` and `payload.identity` is `KemIdentity::Asset` →
///   `Err(DeriveIdentityErrorReturn::WrongIdentityScope)` before hashing.
/// Arrange: the case params' `hash_to_scalar` field under the
///   `SamplingRefused` failure mode, so a hash that ran first returns the hash
///   refusal; the default entitlement parameter set from
///   `build_bb1_depth_one_parameter_set`; `KemIdentity::Asset { root: &root }`,
///   `root` from `build_root` with `bytes: Some([0xa1; 32])`.
/// Act:     `kem.derive_identity(params, payload)` inside `Consumer`.
/// Assert:  `assert_eq!` of the whole error against
///   `DeriveIdentityErrorReturn::WrongIdentityScope`; outside,
///   `created.is_ok()`.
/// Collaborators: the hash-to-scalar family's mock concrete under the
///   `SamplingRefused` failure mode, named by the case params; the pairing
///   family's mock concrete by default params.
#[test]
fn derive_identity_refuses_an_asset_identity_under_an_entitlement_set_before_hashing() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let parameter_set = build_bb1_depth_one_parameter_set(pairing, Default::default());
            let root = build_root(RootConstructorParamsOverrides {
                bytes: Some([0xa1; 32]),
            });
            let params = build_derive_identity_params(Default::default());
            let payload = build_derive_identity_payload(
                &parameter_set,
                DeriveIdentityPayloadOverrides {
                    identity: Some(KemIdentity::Asset { root: &root }),
                },
            );

            // Act
            let result = kem.derive_identity(params, payload);

            // Assert
            let Err(error) = result else {
                panic!("an asset identity under an entitlement set is refused")
            };
            assert_eq!(error, DeriveIdentityErrorReturn::WrongIdentityScope);
        }
    }

    // Act
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Bb1DepthOneCaseParamsOverrides {
            hash_to_scalar: Some(build_create_hash_to_scalar_params(
                CreateHashToScalarParamsOverrides {
                    concrete: Some(HashToScalarConcrete::Mock(
                        MockIHashToScalarAdapterFailureMode::SamplingRefused,
                    )),
                    ..Default::default()
                },
            )),
            ..Default::default()
        }),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// derive_identity_refuses_an_asset_identity_under_an_entitlement_set_before_hashing

/// Contract: `derive_identity`, wrong role: the parameter set's scope is
///   `Asset` and `payload.identity` is `KemIdentity::Entitlement` →
///   `Err(DeriveIdentityErrorReturn::WrongIdentityScope)` before hashing.
/// Arrange: the case params' `hash_to_scalar` field under the
///   `SamplingRefused` failure mode; the parameter set from
///   `build_bb1_depth_one_parameter_set` with `scope:
///   Some(Bb1DepthOneParameterSetScope::Asset { identity_element })`,
///   `identity_element` the `element` of `build_bb1_depth_one_identity_element`;
///   `KemIdentity::Entitlement { canonical: b"entitlement-one" }`.
/// Act:     `kem.derive_identity(params, payload)` inside `Consumer`.
/// Assert:  `assert_eq!` of the whole error against
///   `DeriveIdentityErrorReturn::WrongIdentityScope`; outside,
///   `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_an_asset_identity_under_an_
///   entitlement_set_before_hashing`.
#[test]
fn derive_identity_refuses_an_entitlement_identity_under_an_asset_set_before_hashing() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let identity_element =
                build_bb1_depth_one_identity_element(pairing, Default::default());
            let parameter_set = build_bb1_depth_one_parameter_set(
                pairing,
                Bb1DepthOneParameterSetOverrides {
                    scope: Some(Bb1DepthOneParameterSetScope::Asset {
                        identity_element: identity_element.element.clone(),
                    }),
                    ..Default::default()
                },
            );
            let params = build_derive_identity_params(Default::default());
            let payload = build_derive_identity_payload(
                &parameter_set,
                DeriveIdentityPayloadOverrides {
                    identity: Some(KemIdentity::Entitlement {
                        canonical: b"entitlement-one",
                    }),
                },
            );

            // Act
            let result = kem.derive_identity(params, payload);

            // Assert
            let Err(error) = result else {
                panic!("an entitlement identity under an asset set is refused")
            };
            assert_eq!(error, DeriveIdentityErrorReturn::WrongIdentityScope);
        }
    }

    // Act
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Bb1DepthOneCaseParamsOverrides {
            hash_to_scalar: Some(build_create_hash_to_scalar_params(
                CreateHashToScalarParamsOverrides {
                    concrete: Some(HashToScalarConcrete::Mock(
                        MockIHashToScalarAdapterFailureMode::SamplingRefused,
                    )),
                    ..Default::default()
                },
            )),
            ..Default::default()
        }),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// derive_identity_refuses_an_entitlement_identity_under_an_asset_set_before_hashing

/// Contract: `derive_identity`, refused by the hash: the hash adapter refuses
///   → `Err(DeriveIdentityErrorReturn::HashToScalar(error))`, the refusal
///   unchanged.
/// Arrange: the case params' `hash_to_scalar` field under the
///   `SamplingRefused` failure mode, the default entitlement parameter set,
///   and the default identity of `build_derive_identity_payload`.
/// Act:     `kem.derive_identity(params, payload)` inside `Consumer`.
/// Assert:  `assert_eq!` of the whole error against
///   `DeriveIdentityErrorReturn::HashToScalar(HashToScalarErrorReturn::
///   Sampling(WrongLength { expected: P::Scalar::UNIFORM_BYTES_LENGTH,
///   actual: 0 }))`; outside, `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_an_asset_identity_under_an_
///   entitlement_set_before_hashing`.
#[test]
fn derive_identity_returns_the_hash_refusal_whole() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let parameter_set = build_bb1_depth_one_parameter_set(pairing, Default::default());
            let params = build_derive_identity_params(Default::default());
            let payload = build_derive_identity_payload(&parameter_set, Default::default());

            // Act
            let result = kem.derive_identity(params, payload);

            // Assert
            let Err(error) = result else {
                panic!("the refusing hash adapter refuses the identity mapping")
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
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Bb1DepthOneCaseParamsOverrides {
            hash_to_scalar: Some(build_create_hash_to_scalar_params(
                CreateHashToScalarParamsOverrides {
                    concrete: Some(HashToScalarConcrete::Mock(
                        MockIHashToScalarAdapterFailureMode::SamplingRefused,
                    )),
                    ..Default::default()
                },
            )),
            ..Default::default()
        }),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// derive_identity_returns_the_hash_refusal_whole

/// Contract: `derive_identity`, refused by the mapping: `F = u0 + I·u1` is the
///   identity element → `Err(DeriveIdentityErrorReturn::TrivialIdentityElement)`.
/// Arrange: the default hasher, the parameter set from
///   `build_bb1_depth_one_parameter_set` with `u0: Some(identity_g1.clone())`
///   and `u1: Some(identity_g1)`, so `F` is `identity_g1` whatever `I` is, and
///   the default entitlement identity.
/// Act:     `kem.derive_identity(params, payload)` inside `Consumer`.
/// Assert:  `assert_eq!` of the whole error against
///   `DeriveIdentityErrorReturn::TrivialIdentityElement`; outside,
///   `created.is_ok()`.
/// Collaborators: the pairing family's mock concrete through `create_pairing`
///   and the hash-to-scalar family's mock concrete through
///   `create_hash_to_scalar`, each by its default params.
#[test]
fn derive_identity_refuses_a_trivial_identity_element() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let identity = identity_g1(pairing);
            let parameter_set = build_bb1_depth_one_parameter_set(
                pairing,
                Bb1DepthOneParameterSetOverrides {
                    u0: Some(identity.clone()),
                    u1: Some(identity),
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
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// derive_identity_refuses_a_trivial_identity_element

/// Contract: `derive_identity`, outside the asset scope: the parameter set's
///   scope is `Asset { identity_element }` and `encode_g1(F)` differs from
///   `encode_g1(identity_element)` → `Err(DeriveIdentityErrorReturn::
///   OutsideAssetScope)`.
/// Arrange: the parameter set with `u1: Some(identity_g1)`, so `F` is the
///   set's default `u0`, and `scope: Some(Bb1DepthOneParameterSetScope::Asset {
///   identity_element: point_g1 })`, a point other than that `u0`;
///   `KemIdentity::Asset { root: &root }`, `root` from `build_root` with
///   `bytes: Some([0xa1; 32])`.
/// Act:     `kem.derive_identity(params, payload)` inside `Consumer`.
/// Assert:  `assert_eq!` of the whole error against
///   `DeriveIdentityErrorReturn::OutsideAssetScope`; outside, `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn derive_identity_refuses_an_element_outside_the_asset_scope() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let parameter_set = build_bb1_depth_one_parameter_set(
                pairing,
                Bb1DepthOneParameterSetOverrides {
                    u1: Some(identity_g1(pairing)),
                    scope: Some(Bb1DepthOneParameterSetScope::Asset {
                        identity_element: point_g1(pairing),
                    }),
                    ..Default::default()
                },
            );
            let root = build_root(RootConstructorParamsOverrides {
                bytes: Some([0xa1; 32]),
            });
            let params = build_derive_identity_params(Default::default());
            let payload = build_derive_identity_payload(
                &parameter_set,
                DeriveIdentityPayloadOverrides {
                    identity: Some(KemIdentity::Asset { root: &root }),
                },
            );

            // Act
            let result = kem.derive_identity(params, payload);

            // Assert
            let Err(error) = result else {
                panic!("an identity outside the asset scope is refused")
            };
            assert_eq!(error, DeriveIdentityErrorReturn::OutsideAssetScope);
        }
    }

    // Act
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// derive_identity_refuses_an_element_outside_the_asset_scope

/// Contract: `derive_identity`, derived under the asset scope: the mapping
///   succeeds and the encodings are equal → `Ok(DeriveIdentitySuccessReturn {
///   identity_element })`.
/// Arrange: `u0` the default parameter set's `u0`, and the parameter set with
///   `u1: Some(identity_g1)`, so `F` is `u0` whatever `I` is, `scope:
///   Some(Bb1DepthOneParameterSetScope::Asset { identity_element: u0.clone()
///   })`, and `KemIdentity::Asset { root: &root }`, `root` from `build_root`
///   with `bytes: Some([0xa1; 32])`.
/// Act:     `kem.derive_identity(params, payload)` inside `Consumer`.
/// Assert:  `assert!(encoded_g1(success.identity_element.element) ==
///   encoded_g1(u0))`, the sum of `u0` and a multiple of the identity element;
///   outside, `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn derive_identity_derives_the_element_under_the_asset_scope() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let default_set = build_bb1_depth_one_parameter_set(pairing, Default::default());
            let u0 = default_set.u0.clone();
            let parameter_set = build_bb1_depth_one_parameter_set(
                pairing,
                Bb1DepthOneParameterSetOverrides {
                    u1: Some(identity_g1(pairing)),
                    scope: Some(Bb1DepthOneParameterSetScope::Asset {
                        identity_element: u0.clone(),
                    }),
                    ..Default::default()
                },
            );
            let root = build_root(RootConstructorParamsOverrides {
                bytes: Some([0xa1; 32]),
            });
            let params = build_derive_identity_params(Default::default());
            let payload = build_derive_identity_payload(
                &parameter_set,
                DeriveIdentityPayloadOverrides {
                    identity: Some(KemIdentity::Asset { root: &root }),
                },
            );

            // Act
            let result = kem.derive_identity(params, payload);

            // Assert
            let Ok(success) = result else {
                panic!("the identity the parameter set fixes derives")
            };
            assert!(
                encoded_g1(pairing, success.identity_element.element) == encoded_g1(pairing, u0)
            );
        }
    }

    // Act
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// derive_identity_derives_the_element_under_the_asset_scope

/// Contract: `derive_identity`, derived under the entitlement scope: the
///   mapping succeeds with no comparison → `Ok(DeriveIdentitySuccessReturn {
///   identity_element })`.
/// Arrange: `u0` the default parameter set's `u0`, the parameter set with
///   `u1: Some(identity_g1)` and the entitlement scope, and the default
///   entitlement identity.
/// Act:     `kem.derive_identity(params, payload)` inside `Consumer`.
/// Assert:  `assert!(encoded_g1(success.identity_element.element) ==
///   encoded_g1(u0))`; outside, `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn derive_identity_derives_the_element_under_the_entitlement_scope() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let parameter_set = build_bb1_depth_one_parameter_set(
                pairing,
                Bb1DepthOneParameterSetOverrides {
                    u1: Some(identity_g1(pairing)),
                    ..Default::default()
                },
            );
            let u0 = parameter_set.u0.clone();
            let params = build_derive_identity_params(Default::default());
            let payload = build_derive_identity_payload(&parameter_set, Default::default());

            // Act
            let result = kem.derive_identity(params, payload);

            // Assert
            let Ok(success) = result else {
                panic!("the entitlement identity derives")
            };
            assert!(
                encoded_g1(pairing, success.identity_element.element) == encoded_g1(pairing, u0)
            );
        }
    }

    // Act
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// derive_identity_derives_the_element_under_the_entitlement_scope

/// Contract: `issue`, sampling refused: `payload.uniform` is refused →
///   `Err(IssueErrorReturn::Sampling(error))`, the refusal unchanged.
/// Arrange: the default parameter set, `master_scalar` from
///   `build_bb1_depth_one_master_scalar(Default::default())`, `identity_element`
///   from `build_bb1_depth_one_identity_element(pairing, Default::default())`,
///   and `build_issue_payload` with `uniform: Some(shortened)`, one byte
///   shorter than `P::Scalar::UNIFORM_BYTES_LENGTH`.
/// Act:     `kem.issue(params, payload)` inside `Consumer`.
/// Assert:  `assert_eq!` of the whole error against
///   `IssueErrorReturn::Sampling(WrongLength { expected:
///   P::Scalar::UNIFORM_BYTES_LENGTH, actual:
///   P::Scalar::UNIFORM_BYTES_LENGTH - 1 })`; outside, `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn issue_returns_the_sampling_refusal_whole() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let parameter_set = build_bb1_depth_one_parameter_set(pairing, Default::default());
            let master_scalar = build_bb1_depth_one_master_scalar::<P>(Default::default());
            let identity_element =
                build_bb1_depth_one_identity_element(pairing, Default::default());
            let params = build_issue_params(Default::default());
            let payload = build_issue_payload::<P::Scalar, _, _, _>(
                &parameter_set,
                &master_scalar,
                &identity_element,
                IssuePayloadOverrides {
                    uniform: Some(shortened::<P::Scalar>()),
                },
            );

            // Act
            let result = kem.issue(params, payload);

            // Assert
            let Err(error) = result else {
                panic!("the shortened uniform bytes are refused")
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
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// issue_returns_the_sampling_refusal_whole

/// Contract: `issue`, issued: `(A, B) = (α·g1 + r·F, r·g2)` for the sampled
///   `r`.
/// Arrange: the default parameter set, master scalar, and identity element,
///   `build_issue_payload` with `uniform: Some(scalar_bytes(1))`, and
///   `credential` the default `build_bb1_depth_one_credential(pairing,
///   Default::default())`, which the mock states as the credential for `r = 1`
///   over those defaults.
/// Act:     `kem.issue(params, payload)` inside `Consumer`.
/// Assert:  `assert!(encoded_g1(success.credential.a) ==
///   encoded_g1(credential.a))` and `assert!(encoded_g2(success.credential.b)
///   == encoded_g2(credential.b))`; outside, `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn issue_returns_the_default_credential_for_unit_randomness() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let parameter_set = build_bb1_depth_one_parameter_set(pairing, Default::default());
            let master_scalar = build_bb1_depth_one_master_scalar::<P>(Default::default());
            let identity_element =
                build_bb1_depth_one_identity_element(pairing, Default::default());
            let credential = build_bb1_depth_one_credential(pairing, Default::default());
            let params = build_issue_params(Default::default());
            let payload = build_issue_payload::<P::Scalar, _, _, _>(
                &parameter_set,
                &master_scalar,
                &identity_element,
                IssuePayloadOverrides {
                    uniform: Some(scalar_bytes::<P::Scalar>(1)),
                },
            );

            // Act
            let result = kem.issue(params, payload);

            // Assert
            let Ok(success) = result else {
                panic!("unit uniform bytes issue a credential")
            };
            assert!(encoded_g1(pairing, success.credential.a) == encoded_g1(pairing, credential.a));
            assert!(encoded_g2(pairing, success.credential.b) == encoded_g2(pairing, credential.b));
        }
    }

    // Act
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// issue_returns_the_default_credential_for_unit_randomness

/// Contract: `issue`, issued: `B = r·g2`.
/// Arrange: the defaults of `issue_returns_the_default_credential_for_unit_
///   randomness` with `uniform: Some(scalar_bytes(0))`.
/// Act:     `kem.issue(params, payload)` inside `Consumer`.
/// Assert:  `assert!(encoded_g2(success.credential.b) ==
///   encoded_g2(identity_g2))`, the product of `g2` by zero; outside,
///   `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn issue_derives_b_as_the_identity_element_for_zero_randomness() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let parameter_set = build_bb1_depth_one_parameter_set(pairing, Default::default());
            let master_scalar = build_bb1_depth_one_master_scalar::<P>(Default::default());
            let identity_element =
                build_bb1_depth_one_identity_element(pairing, Default::default());
            let params = build_issue_params(Default::default());
            let payload = build_issue_payload::<P::Scalar, _, _, _>(
                &parameter_set,
                &master_scalar,
                &identity_element,
                IssuePayloadOverrides {
                    uniform: Some(scalar_bytes::<P::Scalar>(0)),
                },
            );

            // Act
            let result = kem.issue(params, payload);

            // Assert
            let Ok(success) = result else {
                panic!("zero uniform bytes issue a credential")
            };
            assert!(
                encoded_g2(pairing, success.credential.b)
                    == encoded_g2(pairing, identity_g2(pairing))
            );
        }
    }

    // Act
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// issue_derives_b_as_the_identity_element_for_zero_randomness

/// Contract: `issue`, issued: `randomness` is the sampled `Secret` of `r`.
/// Arrange: the defaults of `issue_returns_the_default_credential_for_unit_
///   randomness` with `uniform: Some(scalar_bytes(1))`.
/// Act:     `kem.issue(params, payload)` inside `Consumer`.
/// Assert:  `assert!(encoded_scalar(success.randomness.expose().clone()) ==
///   encoded_scalar(scalar(1)))`; outside, `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn issue_returns_the_sampled_randomness() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let parameter_set = build_bb1_depth_one_parameter_set(pairing, Default::default());
            let master_scalar = build_bb1_depth_one_master_scalar::<P>(Default::default());
            let identity_element =
                build_bb1_depth_one_identity_element(pairing, Default::default());
            let params = build_issue_params(Default::default());
            let payload = build_issue_payload::<P::Scalar, _, _, _>(
                &parameter_set,
                &master_scalar,
                &identity_element,
                IssuePayloadOverrides {
                    uniform: Some(scalar_bytes::<P::Scalar>(1)),
                },
            );

            // Act
            let result = kem.issue(params, payload);

            // Assert
            let Ok(success) = result else {
                panic!("unit uniform bytes issue a credential")
            };
            assert!(
                encoded_scalar(pairing, success.randomness.expose().clone())
                    == encoded_scalar(pairing, scalar::<P>(1))
            );
        }
    }

    // Act
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// issue_returns_the_sampled_randomness

/// Contract: `rerandomize`, sampling refused: `payload.uniform` is refused →
///   `Err(RerandomizeErrorReturn::Sampling(error))`, the refusal unchanged.
/// Arrange: the default parameter set, identity element, and credential from
///   `build_bb1_depth_one_credential(pairing, Default::default())`, and
///   `build_rerandomize_payload` with `uniform: Some(shortened)`, one byte
///   shorter than `P::Scalar::UNIFORM_BYTES_LENGTH`.
/// Act:     `kem.rerandomize(params, payload)` inside `Consumer`.
/// Assert:  `assert_eq!` of the whole error against
///   `RerandomizeErrorReturn::Sampling(WrongLength { expected:
///   P::Scalar::UNIFORM_BYTES_LENGTH, actual:
///   P::Scalar::UNIFORM_BYTES_LENGTH - 1 })`; outside, `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn rerandomize_returns_the_sampling_refusal_whole() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let parameter_set = build_bb1_depth_one_parameter_set(pairing, Default::default());
            let identity_element =
                build_bb1_depth_one_identity_element(pairing, Default::default());
            let credential = build_bb1_depth_one_credential(pairing, Default::default());
            let params = build_rerandomize_params(Default::default());
            let payload = build_rerandomize_payload::<P::Scalar, _, _, _>(
                &parameter_set,
                &identity_element,
                &credential,
                RerandomizePayloadOverrides {
                    uniform: Some(shortened::<P::Scalar>()),
                },
            );

            // Act
            let result = kem.rerandomize(params, payload);

            // Assert
            let Err(error) = result else {
                panic!("the shortened uniform bytes are refused")
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
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// rerandomize_returns_the_sampling_refusal_whole

/// Contract: `rerandomize`, rerandomized: `A' = A + s·F` for the sampled `s`.
/// Arrange: the default parameter set, identity element, and credential, and
///   `build_rerandomize_payload` with `uniform: Some(scalar_bytes(0))`.
/// Act:     `kem.rerandomize(params, payload)` inside `Consumer`.
/// Assert:  `assert!(encoded_g1(success.credential.a) ==
///   encoded_g1(credential.a))`, the sum of `A` and the product of `F` by
///   zero; outside, `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn rerandomize_leaves_a_unchanged_for_a_zero_offset() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let parameter_set = build_bb1_depth_one_parameter_set(pairing, Default::default());
            let identity_element =
                build_bb1_depth_one_identity_element(pairing, Default::default());
            let credential = build_bb1_depth_one_credential(pairing, Default::default());
            let params = build_rerandomize_params(Default::default());
            let payload = build_rerandomize_payload::<P::Scalar, _, _, _>(
                &parameter_set,
                &identity_element,
                &credential,
                RerandomizePayloadOverrides {
                    uniform: Some(scalar_bytes::<P::Scalar>(0)),
                },
            );

            // Act
            let result = kem.rerandomize(params, payload);

            // Assert
            let Ok(success) = result else {
                panic!("zero uniform bytes rerandomize the credential")
            };
            assert!(encoded_g1(pairing, success.credential.a) == encoded_g1(pairing, credential.a));
        }
    }

    // Act
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// rerandomize_leaves_a_unchanged_for_a_zero_offset

/// Contract: `rerandomize`, rerandomized: `B' = B + s·g2`.
/// Arrange: as `rerandomize_leaves_a_unchanged_for_a_zero_offset` with
///   `uniform: Some(scalar_bytes(0))`.
/// Act:     `kem.rerandomize(params, payload)` inside `Consumer`.
/// Assert:  `assert!(encoded_g2(success.credential.b) ==
///   encoded_g2(credential.b))`; outside, `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn rerandomize_leaves_b_unchanged_for_a_zero_offset() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let parameter_set = build_bb1_depth_one_parameter_set(pairing, Default::default());
            let identity_element =
                build_bb1_depth_one_identity_element(pairing, Default::default());
            let credential = build_bb1_depth_one_credential(pairing, Default::default());
            let params = build_rerandomize_params(Default::default());
            let payload = build_rerandomize_payload::<P::Scalar, _, _, _>(
                &parameter_set,
                &identity_element,
                &credential,
                RerandomizePayloadOverrides {
                    uniform: Some(scalar_bytes::<P::Scalar>(0)),
                },
            );

            // Act
            let result = kem.rerandomize(params, payload);

            // Assert
            let Ok(success) = result else {
                panic!("zero uniform bytes rerandomize the credential")
            };
            assert!(encoded_g2(pairing, success.credential.b) == encoded_g2(pairing, credential.b));
        }
    }

    // Act
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// rerandomize_leaves_b_unchanged_for_a_zero_offset

/// Contract: `rerandomize`, rerandomized: `offset` is the sampled `Secret` of
///   `s`.
/// Arrange: the fixtures of `rerandomize_leaves_a_unchanged_for_a_zero_offset`
///   with `uniform: Some(scalar_bytes(1))`.
/// Act:     `kem.rerandomize(params, payload)` inside `Consumer`.
/// Assert:  `assert!(encoded_scalar(success.offset.expose().clone()) ==
///   encoded_scalar(scalar(1)))`; outside, `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn rerandomize_returns_the_sampled_offset() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let parameter_set = build_bb1_depth_one_parameter_set(pairing, Default::default());
            let identity_element =
                build_bb1_depth_one_identity_element(pairing, Default::default());
            let credential = build_bb1_depth_one_credential(pairing, Default::default());
            let params = build_rerandomize_params(Default::default());
            let payload = build_rerandomize_payload::<P::Scalar, _, _, _>(
                &parameter_set,
                &identity_element,
                &credential,
                RerandomizePayloadOverrides {
                    uniform: Some(scalar_bytes::<P::Scalar>(1)),
                },
            );

            // Act
            let result = kem.rerandomize(params, payload);

            // Assert
            let Ok(success) = result else {
                panic!("unit uniform bytes rerandomize the credential")
            };
            assert!(
                encoded_scalar(pairing, success.offset.expose().clone())
                    == encoded_scalar(pairing, scalar::<P>(1))
            );
        }
    }

    // Act
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// rerandomize_returns_the_sampled_offset

/// Contract: `is_valid`, checked: the pairing-product check `e(A, g2) = e(g1,
///   hpub) · e(F, B)` holds → `Ok(IsValidSuccessReturn { is_valid: true })`.
/// Arrange: the default parameter set, identity element, and credential, which
///   satisfy the check by the relation `bb1_depth_one/mock.rs` states.
/// Act:     `kem.is_valid(params, payload)` inside `Consumer`.
/// Assert:  `assert!(success.is_valid)`; outside, `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn is_valid_accepts_a_credential_that_satisfies_the_check() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let parameter_set = build_bb1_depth_one_parameter_set(pairing, Default::default());
            let identity_element =
                build_bb1_depth_one_identity_element(pairing, Default::default());
            let credential = build_bb1_depth_one_credential(pairing, Default::default());
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
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// is_valid_accepts_a_credential_that_satisfies_the_check

/// Contract: `is_valid`, checked: the pairing-product check fails →
///   `Ok(IsValidSuccessReturn { is_valid: false })`.
/// Arrange: the fixtures of `is_valid_accepts_a_credential_that_satisfies_the_
///   check` with the credential from `build_bb1_depth_one_credential` with
///   `a: Some(parameter_set.g1.clone())`, a point other than the default `A`.
/// Act:     `kem.is_valid(params, payload)` inside `Consumer`.
/// Assert:  `assert!(!success.is_valid)`; outside, `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn is_valid_rejects_a_credential_with_another_a() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let parameter_set = build_bb1_depth_one_parameter_set(pairing, Default::default());
            let identity_element =
                build_bb1_depth_one_identity_element(pairing, Default::default());
            let credential = build_bb1_depth_one_credential(
                pairing,
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
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// is_valid_rejects_a_credential_with_another_a

/// Contract: `encapsulate`, sampling refused: `payload.uniform` is refused →
///   `Err(EncapsulateErrorReturn::Sampling(error))`, the refusal unchanged.
/// Arrange: the default parameter set and `build_encapsulate_payload` with
///   `uniform: Some(shortened)`, one byte shorter than
///   `P::Scalar::UNIFORM_BYTES_LENGTH`.
/// Act:     `kem.encapsulate(params, payload)` inside `Consumer`.
/// Assert:  `assert_eq!` of the whole error against
///   `EncapsulateErrorReturn::Sampling(WrongLength { expected:
///   P::Scalar::UNIFORM_BYTES_LENGTH, actual:
///   P::Scalar::UNIFORM_BYTES_LENGTH - 1 })`; outside, `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn encapsulate_returns_the_sampling_refusal_whole() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let parameter_set = build_bb1_depth_one_parameter_set(pairing, Default::default());
            let params = build_encapsulate_params(Default::default());
            let payload = build_encapsulate_payload::<P::Scalar, _>(
                &parameter_set,
                EncapsulatePayloadOverrides {
                    uniform: Some(shortened::<P::Scalar>()),
                },
            );

            // Act
            let result = kem.encapsulate(params, payload);

            // Assert
            let Err(error) = result else {
                panic!("the shortened uniform bytes are refused")
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
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// encapsulate_returns_the_sampling_refusal_whole

/// Contract: `encapsulate`, encapsulated: `U = t·g2` for the sampled `t`.
/// Arrange: the default parameter set and `build_encapsulate_payload` with
///   `uniform: Some(scalar_bytes(1))`.
/// Act:     `kem.encapsulate(params, payload)` inside `Consumer`.
/// Assert:  the `u` of `success.capsule`, matched against
///   `Bb1DepthOneCapsule::Entitlement { u, .. }`, satisfies
///   `assert!(encoded_g2(u) == encoded_g2(point_g2))`, the product of `g2` by
///   one; outside, `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn encapsulate_returns_u_as_g2_for_unit_randomness() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let parameter_set = build_bb1_depth_one_parameter_set(pairing, Default::default());
            let params = build_encapsulate_params(Default::default());
            let payload = build_encapsulate_payload::<P::Scalar, _>(
                &parameter_set,
                EncapsulatePayloadOverrides {
                    uniform: Some(scalar_bytes::<P::Scalar>(1)),
                },
            );

            // Act
            let result = kem.encapsulate(params, payload);

            // Assert
            let Ok(success) = result else {
                panic!("unit uniform bytes encapsulate")
            };
            let Bb1DepthOneCapsule::Entitlement { u, .. } = success.capsule else {
                panic!("the entitlement scope yields an entitlement capsule")
            };
            assert!(encoded_g2(pairing, u) == encoded_g2(pairing, point_g2(pairing)));
        }
    }

    // Act
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// encapsulate_returns_u_as_g2_for_unit_randomness

/// Contract: `encapsulate`, encapsulated: `V = t·u0` under the entitlement
///   scope.
/// Arrange: as `encapsulate_returns_u_as_g2_for_unit_randomness`.
/// Act:     `kem.encapsulate(params, payload)` inside `Consumer`.
/// Assert:  the `v` of `Bb1DepthOneCapsule::Entitlement { v, .. }` satisfies
///   `assert!(encoded_g1(v) == encoded_g1(parameter_set.u0))`; outside,
///   `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn encapsulate_returns_v_as_u0_for_unit_randomness_under_the_entitlement_scope() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let parameter_set = build_bb1_depth_one_parameter_set(pairing, Default::default());
            let params = build_encapsulate_params(Default::default());
            let payload = build_encapsulate_payload::<P::Scalar, _>(
                &parameter_set,
                EncapsulatePayloadOverrides {
                    uniform: Some(scalar_bytes::<P::Scalar>(1)),
                },
            );

            // Act
            let result = kem.encapsulate(params, payload);

            // Assert
            let Ok(success) = result else {
                panic!("unit uniform bytes encapsulate")
            };
            let Bb1DepthOneCapsule::Entitlement { v, .. } = success.capsule else {
                panic!("the entitlement scope yields an entitlement capsule")
            };
            assert!(encoded_g1(pairing, v) == encoded_g1(pairing, parameter_set.u0.clone()));
        }
    }

    // Act
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// encapsulate_returns_v_as_u0_for_unit_randomness_under_the_entitlement_scope

/// Contract: `encapsulate`, encapsulated: `W = t·u1` under the entitlement
///   scope.
/// Arrange: as `encapsulate_returns_u_as_g2_for_unit_randomness`.
/// Act:     `kem.encapsulate(params, payload)` inside `Consumer`.
/// Assert:  the `w` of `Bb1DepthOneCapsule::Entitlement { w, .. }` satisfies
///   `assert!(encoded_g1(w) == encoded_g1(parameter_set.u1))`; outside,
///   `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn encapsulate_returns_w_as_u1_for_unit_randomness_under_the_entitlement_scope() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let parameter_set = build_bb1_depth_one_parameter_set(pairing, Default::default());
            let params = build_encapsulate_params(Default::default());
            let payload = build_encapsulate_payload::<P::Scalar, _>(
                &parameter_set,
                EncapsulatePayloadOverrides {
                    uniform: Some(scalar_bytes::<P::Scalar>(1)),
                },
            );

            // Act
            let result = kem.encapsulate(params, payload);

            // Assert
            let Ok(success) = result else {
                panic!("unit uniform bytes encapsulate")
            };
            let Bb1DepthOneCapsule::Entitlement { w, .. } = success.capsule else {
                panic!("the entitlement scope yields an entitlement capsule")
            };
            assert!(encoded_g1(pairing, w) == encoded_g1(pairing, parameter_set.u1.clone()));
        }
    }

    // Act
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// encapsulate_returns_w_as_u1_for_unit_randomness_under_the_entitlement_scope

/// Contract: `encapsulate`, encapsulated: `V = t·F` under the asset scope.
/// Arrange: the parameter set with `scope: Some(Bb1DepthOneParameterSetScope::
///   Asset { identity_element })`, `identity_element` the `element` of
///   `build_bb1_depth_one_identity_element(pairing, Default::default())`, and
///   `uniform: Some(scalar_bytes(1))`.
/// Act:     `kem.encapsulate(params, payload)` inside `Consumer`.
/// Assert:  the `v` of `Bb1DepthOneCapsule::Asset { v, .. }` satisfies
///   `assert!(encoded_g1(v) == encoded_g1(identity_element))`; outside,
///   `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn encapsulate_returns_v_as_the_fixed_element_for_unit_randomness_under_the_asset_scope() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let identity_element =
                build_bb1_depth_one_identity_element(pairing, Default::default());
            let parameter_set = build_bb1_depth_one_parameter_set(
                pairing,
                Bb1DepthOneParameterSetOverrides {
                    scope: Some(Bb1DepthOneParameterSetScope::Asset {
                        identity_element: identity_element.element.clone(),
                    }),
                    ..Default::default()
                },
            );
            let params = build_encapsulate_params(Default::default());
            let payload = build_encapsulate_payload::<P::Scalar, _>(
                &parameter_set,
                EncapsulatePayloadOverrides {
                    uniform: Some(scalar_bytes::<P::Scalar>(1)),
                },
            );

            // Act
            let result = kem.encapsulate(params, payload);

            // Assert
            let Ok(success) = result else {
                panic!("unit uniform bytes encapsulate")
            };
            let Bb1DepthOneCapsule::Asset { v, .. } = success.capsule else {
                panic!("the asset scope yields an asset capsule")
            };
            assert!(
                encoded_g1(pairing, v) == encoded_g1(pairing, identity_element.element.clone())
            );
        }
    }

    // Act
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// encapsulate_returns_v_as_the_fixed_element_for_unit_randomness_under_the_asset_scope

/// Contract: `encapsulate`, encapsulated: `K = e(t·g1, hpub)` encoded by
///   `encode_gt` is the encapsulated value.
/// Arrange: the default parameter set and `uniform: Some(scalar_bytes(1))`.
/// Act:     `kem.encapsulate(params, payload)` inside `Consumer`.
/// Assert:  the bytes of `success.encapsulated.key_material().expose()` equal
///   `encoded_gt` over the one term `build_pairing_product_term` with
///   `g1: Some(point_g1)` and `g2: Some(parameter_set.hpub.clone())`, the
///   pairing of the generator and `hpub`; outside, `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn encapsulate_returns_the_pairing_of_g1_and_hpub_as_the_encapsulated_value_for_unit_randomness() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let parameter_set = build_bb1_depth_one_parameter_set(pairing, Default::default());
            let params = build_encapsulate_params(Default::default());
            let payload = build_encapsulate_payload::<P::Scalar, _>(
                &parameter_set,
                EncapsulatePayloadOverrides {
                    uniform: Some(scalar_bytes::<P::Scalar>(1)),
                },
            );
            let expected = encoded_gt(
                pairing,
                vec![build_pairing_product_term(
                    pairing,
                    PairingProductTermOverrides {
                        g1: Some(point_g1(pairing)),
                        g2: Some(parameter_set.hpub.clone()),
                    },
                )],
            );

            // Act
            let result = kem.encapsulate(params, payload);

            // Assert
            let Ok(success) = result else {
                panic!("unit uniform bytes encapsulate")
            };
            assert_eq!(success.encapsulated.key_material().expose(), &expected);
        }
    }

    // Act
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// encapsulate_returns_the_pairing_of_g1_and_hpub_as_the_encapsulated_value_for_unit_randomness

/// Contract: `is_well_formed`, entitlement: the scope is `Entitlement`, the
///   capsule is `Entitlement { u, v, w }`, and `e(V, g2) = e(u0, U)` and
///   `e(W, g2) = e(u1, U)` hold → `Ok(IsWellFormedSuccessReturn {
///   is_well_formed: true })`.
/// Arrange: the default entitlement parameter set, the capsule
///   `Bb1DepthOneCapsule::Entitlement { u: point_g2, v: parameter_set.u0.clone(),
///   w: parameter_set.u1.clone() }`, the capsule for `t = 1`.
/// Act:     `kem.is_well_formed(params, payload)` inside `Consumer`.
/// Assert:  `assert!(success.is_well_formed)`; outside, `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn is_well_formed_accepts_an_entitlement_capsule_of_the_set() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let parameter_set = build_bb1_depth_one_parameter_set(pairing, Default::default());
            let capsule = Bb1DepthOneCapsule::Entitlement {
                u: point_g2(pairing),
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
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// is_well_formed_accepts_an_entitlement_capsule_of_the_set

/// Contract: `is_well_formed`, entitlement: `e(V, g2) = e(u0, U)` fails →
///   `Ok(IsWellFormedSuccessReturn { is_well_formed: false })`.
/// Arrange: the capsule of `is_well_formed_accepts_an_entitlement_capsule_of_
///   the_set` with `v: point_g1`, a point other than the set's `u0`.
/// Act:     `kem.is_well_formed(params, payload)` inside `Consumer`.
/// Assert:  `assert!(!success.is_well_formed)`; outside, `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn is_well_formed_rejects_an_entitlement_capsule_with_another_v() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let parameter_set = build_bb1_depth_one_parameter_set(pairing, Default::default());
            let capsule = Bb1DepthOneCapsule::Entitlement {
                u: point_g2(pairing),
                v: point_g1(pairing),
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
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// is_well_formed_rejects_an_entitlement_capsule_with_another_v

/// Contract: `is_well_formed`, entitlement: `e(W, g2) = e(u1, U)` fails →
///   `Ok(IsWellFormedSuccessReturn { is_well_formed: false })`.
/// Arrange: the capsule of `is_well_formed_accepts_an_entitlement_capsule_of_
///   the_set` with `w: point_g1`, a point other than the set's `u1`.
/// Act:     `kem.is_well_formed(params, payload)` inside `Consumer`.
/// Assert:  `assert!(!success.is_well_formed)`; outside, `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn is_well_formed_rejects_an_entitlement_capsule_with_another_w() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let parameter_set = build_bb1_depth_one_parameter_set(pairing, Default::default());
            let capsule = Bb1DepthOneCapsule::Entitlement {
                u: point_g2(pairing),
                v: parameter_set.u0.clone(),
                w: point_g1(pairing),
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
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// is_well_formed_rejects_an_entitlement_capsule_with_another_w

/// Contract: `is_well_formed`, asset: the scope is `Asset { identity_element }`,
///   the capsule is `Asset { u, v }`, and `e(V, g2) = e(F, U)` holds → `Ok`
///   holding `true`.
/// Arrange: the parameter set with `scope: Some(Bb1DepthOneParameterSetScope::
///   Asset { identity_element })`, `identity_element` the `element` of
///   `build_bb1_depth_one_identity_element(pairing, Default::default())`, and
///   the capsule `Bb1DepthOneCapsule::Asset { u: point_g2, v:
///   identity_element.clone() }`, the capsule for `t = 1`.
/// Act:     `kem.is_well_formed(params, payload)` inside `Consumer`.
/// Assert:  `assert!(success.is_well_formed)`; outside, `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn is_well_formed_accepts_an_asset_capsule_of_the_set() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let identity_element =
                build_bb1_depth_one_identity_element(pairing, Default::default());
            let parameter_set = build_bb1_depth_one_parameter_set(
                pairing,
                Bb1DepthOneParameterSetOverrides {
                    scope: Some(Bb1DepthOneParameterSetScope::Asset {
                        identity_element: identity_element.element.clone(),
                    }),
                    ..Default::default()
                },
            );
            let capsule = Bb1DepthOneCapsule::Asset {
                u: point_g2(pairing),
                v: identity_element.element.clone(),
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
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// is_well_formed_accepts_an_asset_capsule_of_the_set

/// Contract: `is_well_formed`, asset: `e(V, g2) = e(F, U)` fails → `Ok`
///   holding `false`.
/// Arrange: the asset capsule of `is_well_formed_accepts_an_asset_capsule_of_
///   the_set` with `v: point_g1`, a point other than `F`.
/// Act:     `kem.is_well_formed(params, payload)` inside `Consumer`.
/// Assert:  `assert!(!success.is_well_formed)`; outside, `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn is_well_formed_rejects_an_asset_capsule_with_another_v() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let identity_element =
                build_bb1_depth_one_identity_element(pairing, Default::default());
            let parameter_set = build_bb1_depth_one_parameter_set(
                pairing,
                Bb1DepthOneParameterSetOverrides {
                    scope: Some(Bb1DepthOneParameterSetScope::Asset {
                        identity_element: identity_element.element.clone(),
                    }),
                    ..Default::default()
                },
            );
            let capsule = Bb1DepthOneCapsule::Asset {
                u: point_g2(pairing),
                v: point_g1(pairing),
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
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// is_well_formed_rejects_an_asset_capsule_with_another_v

/// Contract: `is_well_formed`, scope mismatch: the capsule's variant is not
///   the parameter set's scope → `Ok(IsWellFormedSuccessReturn {
///   is_well_formed: false })`, with no pairing check.
/// Arrange: the default entitlement parameter set and the capsule
///   `Bb1DepthOneCapsule::Asset { u: parameter_set.g2.clone(), v:
///   parameter_set.g1.clone() }`.
/// Act:     `kem.is_well_formed(params, payload)` inside `Consumer`.
/// Assert:  `assert!(!success.is_well_formed)`, a verdict the variant alone
///   decides, whatever the elements; outside, `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn is_well_formed_rejects_an_asset_capsule_under_an_entitlement_set() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let parameter_set = build_bb1_depth_one_parameter_set(pairing, Default::default());
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
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// is_well_formed_rejects_an_asset_capsule_under_an_entitlement_set

/// Contract: `is_well_formed`, scope mismatch: the capsule's variant is not
///   the parameter set's scope → `Ok` holding `false`.
/// Arrange: the asset parameter set of `is_well_formed_accepts_an_asset_
///   capsule_of_the_set` and the capsule `Bb1DepthOneCapsule::Entitlement {
///   u: point_g2, v: point_g1, w: point_g1 }`.
/// Act:     `kem.is_well_formed(params, payload)` inside `Consumer`.
/// Assert:  `assert!(!success.is_well_formed)`; outside, `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn is_well_formed_rejects_an_entitlement_capsule_under_an_asset_set() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let identity_element =
                build_bb1_depth_one_identity_element(pairing, Default::default());
            let parameter_set = build_bb1_depth_one_parameter_set(
                pairing,
                Bb1DepthOneParameterSetOverrides {
                    scope: Some(Bb1DepthOneParameterSetScope::Asset {
                        identity_element: identity_element.element.clone(),
                    }),
                    ..Default::default()
                },
            );
            let capsule = Bb1DepthOneCapsule::Entitlement {
                u: point_g2(pairing),
                v: point_g1(pairing),
                w: point_g1(pairing),
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
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// is_well_formed_rejects_an_entitlement_capsule_under_an_asset_set

/// Contract: `decapsulate`, entitlement: the capsule is `Entitlement { u, v,
///   w }` → `Ok(DecapsulateSuccessReturn { encapsulated })` holding `K =
///   e(A, U) / e(V + I·W, B)`.
/// Arrange: the default identity element and credential, the default
///   parameter set, the capsule `Bb1DepthOneCapsule::Entitlement { u: point_g2,
///   v: parameter_set.u0.clone(), w: parameter_set.u1.clone() }`, the capsule
///   for `t = 1`.
/// Act:     `kem.decapsulate(params, payload)` inside `Consumer`.
/// Assert:  the bytes of `success.encapsulated.key_material().expose()` equal
///   `encoded_gt` over the one term with `g1: Some(point_g1)` and
///   `g2: Some(parameter_set.hpub.clone())`, the construction's `K = e(t·g1,
///   hpub)` for `t = 1`, reached by the public-key route and not by the
///   formula the subject computes; outside, `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn decapsulate_recovers_the_pairing_of_g1_and_hpub_from_an_entitlement_capsule_for_unit_randomness()
{
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let parameter_set = build_bb1_depth_one_parameter_set(pairing, Default::default());
            let identity_element =
                build_bb1_depth_one_identity_element(pairing, Default::default());
            let credential = build_bb1_depth_one_credential(pairing, Default::default());
            let capsule = Bb1DepthOneCapsule::Entitlement {
                u: point_g2(pairing),
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
            let expected = encoded_gt(
                pairing,
                vec![build_pairing_product_term(
                    pairing,
                    PairingProductTermOverrides {
                        g1: Some(point_g1(pairing)),
                        g2: Some(parameter_set.hpub.clone()),
                    },
                )],
            );

            // Act
            let result = kem.decapsulate(params, payload);

            // Assert
            let Ok(success) = result;
            assert_eq!(success.encapsulated.key_material().expose(), &expected);
        }
    }

    // Act
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// decapsulate_recovers_the_pairing_of_g1_and_hpub_from_an_entitlement_capsule_for_unit_randomness

/// Contract: `decapsulate`, asset: the capsule is `Asset { u, v }` →
///   `Ok(DecapsulateSuccessReturn { encapsulated })` holding `K = e(A, U) /
///   e(V, B)`.
/// Arrange: the defaults of `decapsulate_recovers_the_pairing_of_g1_and_hpub_
///   from_an_entitlement_capsule_for_unit_randomness` and the capsule
///   `Bb1DepthOneCapsule::Asset { u: point_g2, v:
///   identity_element.element.clone() }`, the capsule for `t = 1` under the
///   fixed element the default identity element holds.
/// Act:     `kem.decapsulate(params, payload)` inside `Consumer`.
/// Assert:  as `decapsulate_recovers_the_pairing_of_g1_and_hpub_from_an_
///   entitlement_capsule_for_unit_randomness`; outside, `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn decapsulate_recovers_the_pairing_of_g1_and_hpub_from_an_asset_capsule_for_unit_randomness() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let parameter_set = build_bb1_depth_one_parameter_set(pairing, Default::default());
            let identity_element =
                build_bb1_depth_one_identity_element(pairing, Default::default());
            let credential = build_bb1_depth_one_credential(pairing, Default::default());
            let capsule = Bb1DepthOneCapsule::Asset {
                u: point_g2(pairing),
                v: identity_element.element.clone(),
            };
            let params = build_decapsulate_params(Default::default());
            let payload = build_decapsulate_payload(
                &identity_element,
                &credential,
                &capsule,
                Default::default(),
            );
            let expected = encoded_gt(
                pairing,
                vec![build_pairing_product_term(
                    pairing,
                    PairingProductTermOverrides {
                        g1: Some(point_g1(pairing)),
                        g2: Some(parameter_set.hpub.clone()),
                    },
                )],
            );

            // Act
            let result = kem.decapsulate(params, payload);

            // Assert
            let Ok(success) = result;
            assert_eq!(success.encapsulated.key_material().expose(), &expected);
        }
    }

    // Act
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// decapsulate_recovers_the_pairing_of_g1_and_hpub_from_an_asset_capsule_for_unit_randomness

/// Contract: `parameter_set_components`: `Bb1DepthOneParameterSetScope::
///   Entitlement` maps to `ParameterSetScopeComponents::Entitlement`.
/// Arrange: the default entitlement parameter set and
///   `build_parameter_set_components_payload(&parameter_set, Default::default())`.
/// Act:     `kem.parameter_set_components(params, payload)` inside `Consumer`.
/// Assert:  `assert!(matches!(success.components.scope,
///   ParameterSetScopeComponents::Entitlement))`; outside, `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn parameter_set_components_maps_the_entitlement_scope_to_the_entitlement_components() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let parameter_set = build_bb1_depth_one_parameter_set(pairing, Default::default());
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
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// parameter_set_components_maps_the_entitlement_scope_to_the_entitlement_components

/// Contract: `parameter_set_components`: `Bb1DepthOneParameterSetScope::Asset {
///   identity_element }` maps to `ParameterSetScopeComponents::Asset {
///   identity_element }`.
/// Arrange: the parameter set with `scope: Some(Bb1DepthOneParameterSetScope::
///   Asset { identity_element })`, `identity_element` the `element` of
///   `build_bb1_depth_one_identity_element(pairing, Default::default())`.
/// Act:     `kem.parameter_set_components(params, payload)` inside `Consumer`.
/// Assert:  `assert!(matches!(success.components.scope,
///   ParameterSetScopeComponents::Asset { .. }))`; outside, `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn parameter_set_components_maps_the_asset_scope_to_the_asset_components() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let identity_element =
                build_bb1_depth_one_identity_element(pairing, Default::default());
            let parameter_set = build_bb1_depth_one_parameter_set(
                pairing,
                Bb1DepthOneParameterSetOverrides {
                    scope: Some(Bb1DepthOneParameterSetScope::Asset {
                        identity_element: identity_element.element.clone(),
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
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// parameter_set_components_maps_the_asset_scope_to_the_asset_components

/// Contract: `parameter_set_from_components`: `ParameterSetScopeComponents::
///   Entitlement` maps back to `Bb1DepthOneParameterSetScope::Entitlement`.
/// Arrange: `build_parameter_set_from_components_payload` with `components`
///   from `build_parameter_set_components(pairing, Default::default())`, whose
///   default scope is the entitlement components.
/// Act:     `kem.parameter_set_from_components(params, payload)` inside
///   `Consumer`.
/// Assert:  `assert!(matches!(success.parameter_set.scope,
///   Bb1DepthOneParameterSetScope::Entitlement))`; outside, `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn parameter_set_from_components_maps_the_entitlement_components_back() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let components = build_parameter_set_components(pairing, Default::default());
            let params = build_parameter_set_from_components_params(Default::default());
            let payload = build_parameter_set_from_components_payload(
                pairing,
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
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// parameter_set_from_components_maps_the_entitlement_components_back

/// Contract: `parameter_set_from_components`: `ParameterSetScopeComponents::
///   Asset { identity_element }` maps back to `Bb1DepthOneParameterSetScope::
///   Asset { identity_element }`.
/// Arrange: `components` from `build_parameter_set_components` with `scope:
///   Some(ParameterSetScopeComponents::Asset { identity_element })`,
///   `identity_element` the `element` of `build_bb1_depth_one_identity_element`.
/// Act:     `kem.parameter_set_from_components(params, payload)` inside
///   `Consumer`.
/// Assert:  `assert!(matches!(success.parameter_set.scope,
///   Bb1DepthOneParameterSetScope::Asset { .. }))`; outside, `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn parameter_set_from_components_maps_the_asset_components_back() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let identity_element =
                build_bb1_depth_one_identity_element(pairing, Default::default());
            let components = build_parameter_set_components(
                pairing,
                ParameterSetComponentsOverrides {
                    scope: Some(ParameterSetScopeComponents::Asset {
                        identity_element: identity_element.element.clone(),
                    }),
                    ..Default::default()
                },
            );
            let params = build_parameter_set_from_components_params(Default::default());
            let payload = build_parameter_set_from_components_payload(
                pairing,
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
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// parameter_set_from_components_maps_the_asset_components_back

/// Contract: `capsule_components`: `Bb1DepthOneCapsule::Entitlement { u, v, w }`
///   maps to `CapsuleComponents::Entitlement { u, v, w }`.
/// Arrange: the default parameter set and the capsule
///   `Bb1DepthOneCapsule::Entitlement { u: parameter_set.g2.clone(), v:
///   parameter_set.g1.clone(), w: parameter_set.g1.clone() }` with
///   `build_capsule_components_payload(&capsule, Default::default())`.
/// Act:     `kem.capsule_components(params, payload)` inside `Consumer`.
/// Assert:  `assert!(matches!(success.components, CapsuleComponents::Entitlement {
///   .. }))`; outside, `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn capsule_components_maps_the_entitlement_capsule_to_the_entitlement_components() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let parameter_set = build_bb1_depth_one_parameter_set(pairing, Default::default());
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
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// capsule_components_maps_the_entitlement_capsule_to_the_entitlement_components

/// Contract: `capsule_components`: `Bb1DepthOneCapsule::Asset { u, v }` maps to
///   `CapsuleComponents::Asset { u, v }`.
/// Arrange: the capsule `Bb1DepthOneCapsule::Asset { u: parameter_set.g2.clone(),
///   v: parameter_set.g1.clone() }`.
/// Act:     `kem.capsule_components(params, payload)` inside `Consumer`.
/// Assert:  `assert!(matches!(success.components, CapsuleComponents::Asset {
///   .. }))`; outside, `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn capsule_components_maps_the_asset_capsule_to_the_asset_components() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let parameter_set = build_bb1_depth_one_parameter_set(pairing, Default::default());
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
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// capsule_components_maps_the_asset_capsule_to_the_asset_components

/// Contract: `capsule_from_components`: `CapsuleComponents::Entitlement { u, v,
///   w }` maps to `Bb1DepthOneCapsule::Entitlement { u, v, w }`.
/// Arrange: `build_capsule_from_components_payload` with `components:
///   Some(CapsuleComponents::Entitlement { u, v, w })`, the elements
///   `parameter_set.g2.clone()`, `parameter_set.g1.clone()`, and
///   `parameter_set.g1.clone()` of the default parameter set.
/// Act:     `kem.capsule_from_components(params, payload)` inside `Consumer`.
/// Assert:  `assert!(matches!(success.capsule, Bb1DepthOneCapsule::Entitlement {
///   .. }))`; outside, `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn capsule_from_components_maps_the_entitlement_components_to_the_entitlement_capsule() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let parameter_set = build_bb1_depth_one_parameter_set(pairing, Default::default());
            let params = build_capsule_from_components_params(Default::default());
            let payload = build_capsule_from_components_payload(
                pairing,
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
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// capsule_from_components_maps_the_entitlement_components_to_the_entitlement_capsule

/// Contract: `capsule_from_components`: `CapsuleComponents::Asset { u, v }`
///   maps to `Bb1DepthOneCapsule::Asset { u, v }`.
/// Arrange: `components: Some(CapsuleComponents::Asset { u, v })`, the elements
///   `parameter_set.g2.clone()` and `parameter_set.g1.clone()`.
/// Act:     `kem.capsule_from_components(params, payload)` inside `Consumer`.
/// Assert:  `assert!(matches!(success.capsule, Bb1DepthOneCapsule::Asset {
///   .. }))`; outside, `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn capsule_from_components_maps_the_asset_components_to_the_asset_capsule() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let parameter_set = build_bb1_depth_one_parameter_set(pairing, Default::default());
            let params = build_capsule_from_components_params(Default::default());
            let payload = build_capsule_from_components_payload(
                pairing,
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
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// capsule_from_components_maps_the_asset_components_to_the_asset_capsule

/// Contract: `credential_components` returns `CredentialComponents { a, b }`,
///   clones of the credential's `a` and `b`.
/// Arrange: the default credential and `build_credential_components_payload(
///   &credential, Default::default())`.
/// Act:     `kem.credential_components(params, payload)` inside `Consumer`.
/// Assert:  `assert!(encoded_g1(success.components.a) ==
///   encoded_g1(credential.a.clone()))` and `assert!(encoded_g2(success.components.b)
///   == encoded_g2(credential.b.clone()))`, the field-to-slot mapping, `a` and
///   `b` being points of different groups; outside, `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn credential_components_returns_the_a_and_b_of_the_credential() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let credential = build_bb1_depth_one_credential(pairing, Default::default());
            let params = build_credential_components_params(Default::default());
            let payload = build_credential_components_payload(&credential, Default::default());

            // Act
            let result = kem.credential_components(params, payload);

            // Assert
            let Ok(success) = result;
            assert!(
                encoded_g1(pairing, success.components.a)
                    == encoded_g1(pairing, credential.a.clone())
            );
            assert!(
                encoded_g2(pairing, success.components.b)
                    == encoded_g2(pairing, credential.b.clone())
            );
        }
    }

    // Act
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// credential_components_returns_the_a_and_b_of_the_credential

/// Contract: `credential_from_components` returns `Bb1DepthOneCredential { a,
///   b }`, the components moved.
/// Arrange: `build_credential_from_components_payload` with `components` from
///   `build_credential_components(pairing, CredentialComponentsOverrides {
///   a: Some(parameter_set.u0.clone()), b: Some(parameter_set.hpub.clone()) })`,
///   points other than the builder's default generators.
/// Act:     `kem.credential_from_components(params, payload)` inside `Consumer`.
/// Assert:  the encoding of `success.credential.a` equals that of
///   `parameter_set.u0` and of `success.credential.b` equals that of
///   `parameter_set.hpub`; outside, `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn credential_from_components_moves_the_components_into_the_credential() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let parameter_set = build_bb1_depth_one_parameter_set(pairing, Default::default());
            let params = build_credential_from_components_params(Default::default());
            let payload = build_credential_from_components_payload(
                pairing,
                CredentialFromComponentsPayloadOverrides {
                    components: Some(build_credential_components(
                        pairing,
                        CredentialComponentsOverrides {
                            a: Some(parameter_set.u0.clone()),
                            b: Some(parameter_set.hpub.clone()),
                        },
                    )),
                },
            );

            // Act
            let result = kem.credential_from_components(params, payload);

            // Assert
            let Ok(success) = result;
            assert!(
                encoded_g1(pairing, success.credential.a)
                    == encoded_g1(pairing, parameter_set.u0.clone())
            );
            assert!(
                encoded_g2(pairing, success.credential.b)
                    == encoded_g2(pairing, parameter_set.hpub.clone())
            );
        }
    }

    // Act
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// credential_from_components_moves_the_components_into_the_credential

/// Contract: `parameter_set_components` returns clones of `g1`, `u0`, `u1`,
///   `g2`, and `hpub`.
/// Arrange: the default parameter set, whose `g1`, `u0`, and `u1` are distinct
///   points, and `build_parameter_set_components_payload(&parameter_set,
///   Default::default())`.
/// Act:     `kem.parameter_set_components(params, payload)` inside `Consumer`.
/// Assert:  each of `g1`, `u0`, and `u1` of `success.components` encodes equal
///   to the like-named field of the set by `encoded_g1`, and each of `g2` and
///   `hpub` by `encoded_g2`; outside, `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn parameter_set_components_returns_the_fields_of_the_parameter_set() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let parameter_set = build_bb1_depth_one_parameter_set(pairing, Default::default());
            let params = build_parameter_set_components_params(Default::default());
            let payload =
                build_parameter_set_components_payload(&parameter_set, Default::default());

            // Act
            let result = kem.parameter_set_components(params, payload);

            // Assert
            let Ok(success) = result;
            assert!(
                encoded_g1(pairing, success.components.g1)
                    == encoded_g1(pairing, parameter_set.g1.clone())
            );
            assert!(
                encoded_g1(pairing, success.components.u0)
                    == encoded_g1(pairing, parameter_set.u0.clone())
            );
            assert!(
                encoded_g1(pairing, success.components.u1)
                    == encoded_g1(pairing, parameter_set.u1.clone())
            );
            assert!(
                encoded_g2(pairing, success.components.g2)
                    == encoded_g2(pairing, parameter_set.g2.clone())
            );
            assert!(
                encoded_g2(pairing, success.components.hpub)
                    == encoded_g2(pairing, parameter_set.hpub.clone())
            );
        }
    }

    // Act
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// parameter_set_components_returns_the_fields_of_the_parameter_set

/// Contract: `parameter_set_from_components` returns the parameter set holding
///   the components moved.
/// Arrange: `build_parameter_set_from_components_payload` with `components`
///   from `build_parameter_set_components(pairing, Default::default())`, whose
///   `g1`, `u0`, and `u1` are distinct points.
/// Act:     `kem.parameter_set_from_components(params, payload)` inside
///   `Consumer`.
/// Assert:  each of `g1`, `u0`, `u1`, `g2`, and `hpub` of
///   `success.parameter_set` encodes equal to the like-named field of those
///   components; outside, `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn parameter_set_from_components_moves_the_components_into_the_parameter_set() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let components = build_parameter_set_components(pairing, Default::default());
            let expected_g1 = components.g1.clone();
            let expected_u0 = components.u0.clone();
            let expected_u1 = components.u1.clone();
            let expected_g2 = components.g2.clone();
            let expected_hpub = components.hpub.clone();
            let params = build_parameter_set_from_components_params(Default::default());
            let payload = build_parameter_set_from_components_payload(
                pairing,
                ParameterSetFromComponentsPayloadOverrides {
                    components: Some(components),
                },
            );

            // Act
            let result = kem.parameter_set_from_components(params, payload);

            // Assert
            let Ok(success) = result;
            assert!(
                encoded_g1(pairing, success.parameter_set.g1) == encoded_g1(pairing, expected_g1)
            );
            assert!(
                encoded_g1(pairing, success.parameter_set.u0) == encoded_g1(pairing, expected_u0)
            );
            assert!(
                encoded_g1(pairing, success.parameter_set.u1) == encoded_g1(pairing, expected_u1)
            );
            assert!(
                encoded_g2(pairing, success.parameter_set.g2) == encoded_g2(pairing, expected_g2)
            );
            assert!(
                encoded_g2(pairing, success.parameter_set.hpub)
                    == encoded_g2(pairing, expected_hpub)
            );
        }
    }

    // Act
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// parameter_set_from_components_moves_the_components_into_the_parameter_set

/// Contract: `identity_element_components` returns clones of `scalar` and
///   `element`.
/// Arrange: the default identity element and
///   `build_identity_element_components_payload(&identity_element,
///   Default::default())`.
/// Act:     `kem.identity_element_components(params, payload)` inside `Consumer`.
/// Assert:  `assert!(encoded_scalar(success.components.scalar) ==
///   encoded_scalar(identity_element.scalar.clone()))` and
///   `assert!(encoded_g1(success.components.element) ==
///   encoded_g1(identity_element.element.clone()))`; outside, `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn identity_element_components_returns_the_scalar_and_the_element() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let identity_element =
                build_bb1_depth_one_identity_element(pairing, Default::default());
            let params = build_identity_element_components_params(Default::default());
            let payload =
                build_identity_element_components_payload(&identity_element, Default::default());

            // Act
            let result = kem.identity_element_components(params, payload);

            // Assert
            let Ok(success) = result;
            assert!(
                encoded_scalar(pairing, success.components.scalar)
                    == encoded_scalar(pairing, identity_element.scalar.clone())
            );
            assert!(
                encoded_g1(pairing, success.components.element)
                    == encoded_g1(pairing, identity_element.element.clone())
            );
        }
    }

    // Act
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// identity_element_components_returns_the_scalar_and_the_element

/// Contract: `master_scalar_components` returns `MasterScalarComponents {
///   value }`, `value` a new `Secret` of a clone of the master scalar's value.
/// Arrange: the default master scalar and
///   `build_master_scalar_components_payload(&master_scalar, Default::default())`.
/// Act:     `kem.master_scalar_components(params, payload)` inside `Consumer`.
/// Assert:  `assert!(encoded_scalar(success.components.value.expose().clone())
///   == encoded_scalar(master_scalar.value.expose().clone()))`; outside,
///   `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn master_scalar_components_returns_a_secret_of_the_value() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let master_scalar = build_bb1_depth_one_master_scalar::<P>(Default::default());
            let params = build_master_scalar_components_params(Default::default());
            let payload =
                build_master_scalar_components_payload(&master_scalar, Default::default());

            // Act
            let result = kem.master_scalar_components(params, payload);

            // Assert
            let Ok(success) = result;
            assert!(
                encoded_scalar(pairing, success.components.value.expose().clone())
                    == encoded_scalar(pairing, master_scalar.value.expose().clone())
            );
        }
    }

    // Act
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// master_scalar_components_returns_a_secret_of_the_value

/// Contract: `master_scalar_from_components` returns `Bb1DepthOneMasterScalar {
///   value }`, the `Secret` moved.
/// Arrange: `build_master_scalar_from_components_payload` with `components`
///   from `build_master_scalar_components` with `value` a `Secret` of
///   `scalar(3)`, a scalar other than the builder's default.
/// Act:     `kem.master_scalar_from_components(params, payload)` inside
///   `Consumer`.
/// Assert:  `assert!(encoded_scalar(success.master_scalar.value.expose().clone())
///   == encoded_scalar(scalar(3)))`; outside, `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn master_scalar_from_components_moves_the_secret_into_the_master_scalar() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
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
                encoded_scalar(pairing, success.master_scalar.value.expose().clone())
                    == encoded_scalar(pairing, scalar::<P>(3))
            );
        }
    }

    // Act
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// master_scalar_from_components_moves_the_secret_into_the_master_scalar

/// Contract: `capsule_components` returns clones of `u`, `v`, and `w` in
///   `CapsuleComponents::Entitlement { u, v, w }`.
/// Arrange: the default parameter set and the capsule
///   `Bb1DepthOneCapsule::Entitlement { u: point_g2, v: parameter_set.u0.clone(),
///   w: parameter_set.u1.clone() }` with `build_capsule_components_payload(
///   &capsule, Default::default())`.
/// Act:     `kem.capsule_components(params, payload)` inside `Consumer`.
/// Assert:  `success.components` matches `CapsuleComponents::Entitlement { u,
///   v, w }` and `u`, `v`, and `w` encode equal to `point_g2`,
///   `parameter_set.u0`, and `parameter_set.u1`; outside, `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn capsule_components_returns_the_entitlement_elements_in_their_slots() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let parameter_set = build_bb1_depth_one_parameter_set(pairing, Default::default());
            let capsule = Bb1DepthOneCapsule::Entitlement {
                u: point_g2(pairing),
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
                panic!("the entitlement capsule yields entitlement components")
            };
            assert!(encoded_g2(pairing, u) == encoded_g2(pairing, point_g2(pairing)));
            assert!(encoded_g1(pairing, v) == encoded_g1(pairing, parameter_set.u0.clone()));
            assert!(encoded_g1(pairing, w) == encoded_g1(pairing, parameter_set.u1.clone()));
        }
    }

    // Act
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// capsule_components_returns_the_entitlement_elements_in_their_slots

/// Contract: `capsule_from_components` returns `Bb1DepthOneCapsule::Entitlement
///   { u, v, w }` holding the components moved.
/// Arrange: `build_capsule_from_components_payload` with `components:
///   Some(CapsuleComponents::Entitlement { u: point_g2, v: parameter_set.u0.clone(),
///   w: parameter_set.u1.clone() })`.
/// Act:     `kem.capsule_from_components(params, payload)` inside `Consumer`.
/// Assert:  `success.capsule` matches `Bb1DepthOneCapsule::Entitlement { u, v,
///   w }` and `u`, `v`, and `w` encode equal to `point_g2`,
///   `parameter_set.u0`, and `parameter_set.u1`; outside, `created.is_ok()`.
/// Collaborators: as `derive_identity_refuses_a_trivial_identity_element`.
#[test]
fn capsule_from_components_moves_the_entitlement_elements_into_their_slots() {
    struct Consumer;

    impl IBb1DepthOneCaseConsumer for Consumer {
        type Output = ();

        fn consume_bb1_depth_one_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &Bb1DepthOneCasePayload,
        ) -> Self::Output {
            let kem: Bb1DepthOneKem<'_, P> =
                build_bb1_depth_one_kem(pairing, hash_to_scalar, Default::default());

            // Arrange
            let parameter_set = build_bb1_depth_one_parameter_set(pairing, Default::default());
            let params = build_capsule_from_components_params(Default::default());
            let payload = build_capsule_from_components_payload(
                pairing,
                CapsuleFromComponentsPayloadOverrides {
                    components: Some(CapsuleComponents::Entitlement {
                        u: point_g2(pairing),
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
                panic!("the entitlement components rebuild an entitlement capsule")
            };
            assert!(encoded_g2(pairing, u) == encoded_g2(pairing, point_g2(pairing)));
            assert!(encoded_g1(pairing, v) == encoded_g1(pairing, parameter_set.u0.clone()));
            assert!(encoded_g1(pairing, w) == encoded_g1(pairing, parameter_set.u1.clone()));
        }
    }

    // Act
    let created = run_bb1_depth_one_case(
        &build_bb1_depth_one_case_deps(Consumer, Default::default()),
        build_bb1_depth_one_case_params(Default::default()),
        build_bb1_depth_one_case_payload(Default::default()),
    );

    // Assert
    assert!(created.is_ok());
}
// capsule_from_components_moves_the_entitlement_elements_into_their_slots
