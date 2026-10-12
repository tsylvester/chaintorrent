#![allow(clippy::expect_used)]

use super::mock::{
    BINDING_LONGER_TAG, BINDING_SHORTER_MESSAGE, EMPTY_MESSAGE, OTHER_TAG, REFERENCE_MESSAGE,
    REFERENCE_TAG, build_keccak256_hash_to_scalar, expected_uniform_bytes,
};
use crate::domain_tag::provides::{DomainTagConstructorParamsOverrides, build_domain_tag};
use crate::factory::provides::{
    HashToScalarParamsOverrides, HashToScalarPayloadOverrides, IHashToScalarAdapter,
    build_hash_to_scalar_params, build_hash_to_scalar_payload,
};
use domain::build_secret;
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParamsOverrides,
    CreatePairingPayload, EncodeScalarParams, EncodeScalarPayload, IPairingAdapter,
    IPairingArithmetic, IPairingConsumer, IPairingReference, ISampleUniformScalar,
    PAIRING_CONCRETES, SampleUniformScalarParams, SampleUniformScalarPayload,
    build_create_pairing_deps, build_create_pairing_params, create_pairing,
};

/// Contract: entry `hash_to_scalar_matches_the_oracle_for_the_reference_tag_and_message`;
///   given `REFERENCE_TAG` and `REFERENCE_MESSAGE`, the encoding of the returned
///   scalar equals the encoding of the scalar sampled from
///   `expected_uniform_bytes`; absorbs the pairing entry
///   `a_sampled_scalar_encodes_through_secret`.
/// Arrange: `Consumer` with `TAG` as `REFERENCE_TAG` and `MESSAGE` as
///   `REFERENCE_MESSAGE`; every member of `PAIRING_CONCRETES` in turn, each with
///   its own scalar type.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)` per concrete.
/// Assert:  per concrete, the actual encoding equals the expected encoding, which
///   the oracle states independently of `hash_to_scalar`.
/// Boundary: the real chain is `create_pairing`, the arm's `try_new`,
///   `DomainTag::try_new`, `Keccak256HashToScalar::try_new`, `hash_to_scalar`,
///   the received scalar's `sample_from_uniform_bytes`, and `encode_scalar`.
/// Mocked:  none.
#[test]
fn hash_to_scalar_matches_the_oracle_for_the_reference_tag_and_message() {
    // Arrange
    const TAG: &[u8] = REFERENCE_TAG;
    const MESSAGE: &[u8] = REFERENCE_MESSAGE;

    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = (Vec<u8>, Vec<u8>);

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                bytes: Some(TAG.to_vec()),
            });
            let adapter = build_keccak256_hash_to_scalar(Default::default());
            let hashed = IHashToScalarAdapter::<P::Scalar>::hash_to_scalar(
                &adapter,
                build_hash_to_scalar_params(HashToScalarParamsOverrides { tag: Some(&tag) }),
                build_hash_to_scalar_payload(HashToScalarPayloadOverrides {
                    message: Some(MESSAGE),
                }),
            )
            .expect("hash_to_scalar admits the tag and message");
            let actual = consumed
                .adapter
                .encode_scalar(
                    EncodeScalarParams,
                    EncodeScalarPayload {
                        scalar: hashed.scalar,
                    },
                )
                .expect("the returned scalar encodes");
            let uniform = build_secret(
                expected_uniform_bytes(&tag, MESSAGE, P::Scalar::UNIFORM_BYTES_LENGTH),
                Default::default(),
            );
            let sampled = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload { uniform },
            )
            .expect("the uniform input samples a scalar");
            let expected = consumed
                .adapter
                .encode_scalar(
                    EncodeScalarParams,
                    EncodeScalarPayload {
                        scalar: sampled.scalar.expose().clone(),
                    },
                )
                .expect("the sampled scalar encodes");
            (
                actual.bytes.expose().as_ref().to_vec(),
                expected.bytes.expose().as_ref().to_vec(),
            )
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(Consumer, Default::default());

        // Act
        let result = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        let (actual, expected) = result
            .ok()
            .map(|success| success.output)
            .expect("the pairing factory admits the concrete");
        assert_eq!(actual, expected);
    }
}

/// Contract: entry `hash_to_scalar_matches_the_oracle_for_the_empty_message`;
///   given `REFERENCE_TAG` and `EMPTY_MESSAGE`, the encoding of the returned
///   scalar equals the encoding of the scalar sampled from
///   `expected_uniform_bytes`.
/// Arrange: `Consumer` with `TAG` as `REFERENCE_TAG` and `MESSAGE` as
///   `EMPTY_MESSAGE`, the message segment absent; every member of
///   `PAIRING_CONCRETES` in turn.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)` per concrete.
/// Assert:  per concrete, the actual encoding equals the expected encoding.
/// Boundary: as `hash_to_scalar_matches_the_oracle_for_the_reference_tag_and_message`.
/// Mocked:  none.
#[test]
fn hash_to_scalar_matches_the_oracle_for_the_empty_message() {
    // Arrange
    const TAG: &[u8] = REFERENCE_TAG;
    const MESSAGE: &[u8] = EMPTY_MESSAGE;

    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = (Vec<u8>, Vec<u8>);

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                bytes: Some(TAG.to_vec()),
            });
            let adapter = build_keccak256_hash_to_scalar(Default::default());
            let hashed = IHashToScalarAdapter::<P::Scalar>::hash_to_scalar(
                &adapter,
                build_hash_to_scalar_params(HashToScalarParamsOverrides { tag: Some(&tag) }),
                build_hash_to_scalar_payload(HashToScalarPayloadOverrides {
                    message: Some(MESSAGE),
                }),
            )
            .expect("hash_to_scalar admits the tag and message");
            let actual = consumed
                .adapter
                .encode_scalar(
                    EncodeScalarParams,
                    EncodeScalarPayload {
                        scalar: hashed.scalar,
                    },
                )
                .expect("the returned scalar encodes");
            let uniform = build_secret(
                expected_uniform_bytes(&tag, MESSAGE, P::Scalar::UNIFORM_BYTES_LENGTH),
                Default::default(),
            );
            let sampled = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload { uniform },
            )
            .expect("the uniform input samples a scalar");
            let expected = consumed
                .adapter
                .encode_scalar(
                    EncodeScalarParams,
                    EncodeScalarPayload {
                        scalar: sampled.scalar.expose().clone(),
                    },
                )
                .expect("the sampled scalar encodes");
            (
                actual.bytes.expose().as_ref().to_vec(),
                expected.bytes.expose().as_ref().to_vec(),
            )
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(Consumer, Default::default());

        // Act
        let result = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        let (actual, expected) = result
            .ok()
            .map(|success| success.output)
            .expect("the pairing factory admits the concrete");
        assert_eq!(actual, expected);
    }
}

/// Contract: entry `hash_to_scalar_binds_the_tag_length`; given
///   `BINDING_LONGER_TAG` and `BINDING_SHORTER_MESSAGE`, the encoding of the
///   returned scalar equals the encoding of the scalar sampled from
///   `expected_uniform_bytes`, whose input begins with the length of the longer
///   tag.
/// Arrange: `Consumer` with `TAG` as `BINDING_LONGER_TAG` and `MESSAGE` as
///   `BINDING_SHORTER_MESSAGE`, a split of the bytes `BINDING_TAG` and
///   `BINDING_MESSAGE` concatenate to, with the tag one byte longer; every member
///   of `PAIRING_CONCRETES` in turn.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)` per concrete.
/// Assert:  per concrete, the actual encoding equals the expected encoding.
/// Boundary: as `hash_to_scalar_matches_the_oracle_for_the_reference_tag_and_message`.
/// Mocked:  none.
#[test]
fn hash_to_scalar_binds_the_tag_length() {
    // Arrange
    const TAG: &[u8] = BINDING_LONGER_TAG;
    const MESSAGE: &[u8] = BINDING_SHORTER_MESSAGE;

    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = (Vec<u8>, Vec<u8>);

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                bytes: Some(TAG.to_vec()),
            });
            let adapter = build_keccak256_hash_to_scalar(Default::default());
            let hashed = IHashToScalarAdapter::<P::Scalar>::hash_to_scalar(
                &adapter,
                build_hash_to_scalar_params(HashToScalarParamsOverrides { tag: Some(&tag) }),
                build_hash_to_scalar_payload(HashToScalarPayloadOverrides {
                    message: Some(MESSAGE),
                }),
            )
            .expect("hash_to_scalar admits the tag and message");
            let actual = consumed
                .adapter
                .encode_scalar(
                    EncodeScalarParams,
                    EncodeScalarPayload {
                        scalar: hashed.scalar,
                    },
                )
                .expect("the returned scalar encodes");
            let uniform = build_secret(
                expected_uniform_bytes(&tag, MESSAGE, P::Scalar::UNIFORM_BYTES_LENGTH),
                Default::default(),
            );
            let sampled = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload { uniform },
            )
            .expect("the uniform input samples a scalar");
            let expected = consumed
                .adapter
                .encode_scalar(
                    EncodeScalarParams,
                    EncodeScalarPayload {
                        scalar: sampled.scalar.expose().clone(),
                    },
                )
                .expect("the sampled scalar encodes");
            (
                actual.bytes.expose().as_ref().to_vec(),
                expected.bytes.expose().as_ref().to_vec(),
            )
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(Consumer, Default::default());

        // Act
        let result = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        let (actual, expected) = result
            .ok()
            .map(|success| success.output)
            .expect("the pairing factory admits the concrete");
        assert_eq!(actual, expected);
    }
}

/// Contract: entry `hash_to_scalar_separates_domains`; given `OTHER_TAG` and
///   `REFERENCE_MESSAGE`, the encoding of the returned scalar equals the encoding
///   of the scalar sampled from `expected_uniform_bytes` for `OTHER_TAG`.
/// Arrange: `Consumer` with `TAG` as `OTHER_TAG` and `MESSAGE` as
///   `REFERENCE_MESSAGE`, the message of the reference block under another tag;
///   every member of `PAIRING_CONCRETES` in turn.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)` per concrete.
/// Assert:  per concrete, the actual encoding equals the expected encoding.
/// Boundary: as `hash_to_scalar_matches_the_oracle_for_the_reference_tag_and_message`.
/// Mocked:  none.
#[test]
fn hash_to_scalar_separates_domains() {
    // Arrange
    const TAG: &[u8] = OTHER_TAG;
    const MESSAGE: &[u8] = REFERENCE_MESSAGE;

    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = (Vec<u8>, Vec<u8>);

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                bytes: Some(TAG.to_vec()),
            });
            let adapter = build_keccak256_hash_to_scalar(Default::default());
            let hashed = IHashToScalarAdapter::<P::Scalar>::hash_to_scalar(
                &adapter,
                build_hash_to_scalar_params(HashToScalarParamsOverrides { tag: Some(&tag) }),
                build_hash_to_scalar_payload(HashToScalarPayloadOverrides {
                    message: Some(MESSAGE),
                }),
            )
            .expect("hash_to_scalar admits the tag and message");
            let actual = consumed
                .adapter
                .encode_scalar(
                    EncodeScalarParams,
                    EncodeScalarPayload {
                        scalar: hashed.scalar,
                    },
                )
                .expect("the returned scalar encodes");
            let uniform = build_secret(
                expected_uniform_bytes(&tag, MESSAGE, P::Scalar::UNIFORM_BYTES_LENGTH),
                Default::default(),
            );
            let sampled = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload { uniform },
            )
            .expect("the uniform input samples a scalar");
            let expected = consumed
                .adapter
                .encode_scalar(
                    EncodeScalarParams,
                    EncodeScalarPayload {
                        scalar: sampled.scalar.expose().clone(),
                    },
                )
                .expect("the sampled scalar encodes");
            (
                actual.bytes.expose().as_ref().to_vec(),
                expected.bytes.expose().as_ref().to_vec(),
            )
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(Consumer, Default::default());

        // Act
        let result = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        let (actual, expected) = result
            .ok()
            .map(|success| success.output)
            .expect("the pairing factory admits the concrete");
        assert_eq!(actual, expected);
    }
}
