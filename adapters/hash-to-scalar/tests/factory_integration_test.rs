#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use domain::{build_derivation_context, build_secret};
use encoding::{
    CreateEncodingParamsOverrides, DECLARED_ENCODING_SELECTIONS, DerivationContextDescription,
    EncodingIdentifier, build_create_encoding_deps, build_create_encoding_params,
    build_create_encoding_payload, build_mock_i_encoding_consumer, create_encoding,
};
use hash_to_scalar::{
    DECLARED_HASH_TO_SCALAR_SELECTIONS, DOMAIN_TAG_MAXIMUM_LENGTH, DomainTag,
    DomainTagConstructorParamsOverrides, DomainTagTryNewErrorReturn, HashToScalarIdentifier,
    HashToScalarParamsOverrides, HashToScalarPayloadOverrides, IHashToScalarAdapter,
    REFERENCE_MESSAGE, REFERENCE_TAG, build_create_hash_to_scalar_deps,
    build_create_hash_to_scalar_payload, build_domain_tag_constructor_params,
    build_hash_to_scalar_params, build_hash_to_scalar_payload, create_hash_to_scalar,
    expected_uniform_bytes,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParamsOverrides,
    CreatePairingPayload, EncodeScalarParams, EncodeScalarPayload, IPairingAdapter,
    IPairingConsumer, ISampleUniformScalar, PAIRING_CONCRETES, SampleUniformScalarParams,
    SampleUniformScalarPayload, build_create_pairing_deps, build_create_pairing_params,
    create_pairing,
};

/// Contract: entry
///   `every_concrete_declaring_the_keccak256_identifier_hashes_the_reference_inputs_to_the_oracle`;
///   given `REFERENCE_TAG` built by `DomainTag::try_new` and
///   `REFERENCE_MESSAGE`, the encoding of the returned scalar equals the
///   encoding of the scalar sampled from
///   `expected_uniform_bytes(&tag, REFERENCE_MESSAGE, S::UNIFORM_BYTES_LENGTH)`.
/// Arrange: the consumer's `tag_bytes` as `REFERENCE_TAG.to_vec()` and
///   `message` as `REFERENCE_MESSAGE.to_vec()`; every case of `keccak_cases`
///   over every member of `PAIRING_CONCRETES`, each with its own scalar type.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)` per concrete.
/// Assert:  per concrete and case, the actual encoding equals the expected
///   encoding, which the oracle states independently of `hash_to_scalar`;
///   outside the consumer, `created.is_ok()`.
/// Boundary: the public calls `DomainTag::try_new`, `create_pairing`,
///   `create_hash_to_scalar`, `hash_to_scalar` on the returned adapter,
///   `sample_from_uniform_bytes`, and `encode_scalar`, the route `lib.rs` then
///   `domain_tag::provides` then `DomainTag::try_new`, and `lib.rs` then
///   `factory::provides` then `create_hash_to_scalar`.
/// Mocked:  none; the route reaches no outer-edge collaborator, and the
///   fixtures and the oracle come from the official mock under the `mocks`
///   feature.
#[test]
fn every_concrete_declaring_the_keccak256_identifier_hashes_the_reference_inputs_to_the_oracle() {
    // Arrange
    struct Consumer {
        tag_bytes: Vec<u8>,
        message: Vec<u8>,
    }

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingAdapter>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let tag = DomainTag::try_new(build_domain_tag_constructor_params(
                DomainTagConstructorParamsOverrides {
                    bytes: Some(self.tag_bytes.clone()),
                },
            ))
            .expect("the tag bytes admit a domain tag");
            let keccak_cases = DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .copied()
                .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
                .collect::<Vec<_>>();
            for case in keccak_cases {
                let success = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    case,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the case's declared identifier admits its concrete");
                let adapter: &dyn IHashToScalarAdapter<P::Scalar> = &*success.adapter;
                let hashed = adapter
                    .hash_to_scalar(
                        build_hash_to_scalar_params(HashToScalarParamsOverrides {
                            tag: Some(&tag),
                        }),
                        build_hash_to_scalar_payload(HashToScalarPayloadOverrides {
                            message: Some(&self.message),
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
                    expected_uniform_bytes(&tag, &self.message, P::Scalar::UNIFORM_BYTES_LENGTH),
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
                assert_eq!(
                    actual.bytes.expose().as_ref().to_vec(),
                    expected.bytes.expose().as_ref().to_vec()
                );
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(
            Consumer {
                tag_bytes: REFERENCE_TAG.to_vec(),
                message: REFERENCE_MESSAGE.to_vec(),
            },
            Default::default(),
        );

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: carried `domain_tag` entry `an admitted tag reads back through
///   the crate root`; the bytes `as_bytes()` returns for the tag are
///   `REFERENCE_TAG`, and the adapter `create_hash_to_scalar` returns hashes
///   that tag, so the encoding of the returned scalar equals the encoding of
///   the scalar sampled from
///   `expected_uniform_bytes(&tag, REFERENCE_MESSAGE, S::UNIFORM_BYTES_LENGTH)`.
/// Arrange: as
///   `every_concrete_declaring_the_keccak256_identifier_hashes_the_reference_inputs_to_the_oracle`,
///   the consumer also reading the tag's bytes through `as_bytes`; the
///   variation is a read that truncates, trims, or re-encodes the tag, which
///   yields a scalar the oracle does not state.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)` per concrete.
/// Assert:  in the consumer, `tag.as_bytes()` equals `REFERENCE_TAG` by
///   `assert_eq!`, and per case the actual encoding equals the expected
///   encoding; outside the consumer, `created.is_ok()`.
/// Boundary: the route of
///   `every_concrete_declaring_the_keccak256_identifier_hashes_the_reference_inputs_to_the_oracle`
///   extended by the public read `DomainTag::as_bytes`.
/// Mocked:  none.
#[test]
fn a_tag_read_back_through_the_crate_root_is_the_tag_the_factory_adapter_hashes() {
    // Arrange
    struct Consumer {
        tag_bytes: Vec<u8>,
        message: Vec<u8>,
    }

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingAdapter>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let tag = DomainTag::try_new(build_domain_tag_constructor_params(
                DomainTagConstructorParamsOverrides {
                    bytes: Some(self.tag_bytes.clone()),
                },
            ))
            .expect("the tag bytes admit a domain tag");
            assert_eq!(tag.as_bytes(), REFERENCE_TAG);
            let keccak_cases = DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .copied()
                .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
                .collect::<Vec<_>>();
            for case in keccak_cases {
                let success = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    case,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the case's declared identifier admits its concrete");
                let adapter: &dyn IHashToScalarAdapter<P::Scalar> = &*success.adapter;
                let hashed = adapter
                    .hash_to_scalar(
                        build_hash_to_scalar_params(HashToScalarParamsOverrides {
                            tag: Some(&tag),
                        }),
                        build_hash_to_scalar_payload(HashToScalarPayloadOverrides {
                            message: Some(&self.message),
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
                    expected_uniform_bytes(&tag, &self.message, P::Scalar::UNIFORM_BYTES_LENGTH),
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
                assert_eq!(
                    actual.bytes.expose().as_ref().to_vec(),
                    expected.bytes.expose().as_ref().to_vec()
                );
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(
            Consumer {
                tag_bytes: REFERENCE_TAG.to_vec(),
                message: REFERENCE_MESSAGE.to_vec(),
            },
            Default::default(),
        );

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: carried `domain_tag` entry `a refusal reaches the caller whole
///   and yields no tag`; a tag of `DOMAIN_TAG_MAXIMUM_LENGTH + 1` printable
///   bytes yields `Err(DomainTagTryNewErrorReturn::TooLong { length:
///   DOMAIN_TAG_MAXIMUM_LENGTH + 1, maximum: DOMAIN_TAG_MAXIMUM_LENGTH })`,
///   and no `DomainTag` exists to place in `HashToScalarParams`.
/// Arrange: params from `build_domain_tag_constructor_params(
///   DomainTagConstructorParamsOverrides { bytes: Some(vec![b'a';
///   DOMAIN_TAG_MAXIMUM_LENGTH + 1]) })`, one byte over the length the
///   read-back block admits.
/// Act:     `DomainTag::try_new` over those params.
/// Assert:  the result equals the whole `Err(
///   DomainTagTryNewErrorReturn::TooLong { length:
///   DOMAIN_TAG_MAXIMUM_LENGTH + 1, maximum: DOMAIN_TAG_MAXIMUM_LENGTH })`
///   by `assert_eq!`, so no tag is held.
/// Boundary: the public call `DomainTag::try_new`, the route `lib.rs` then
///   `domain_tag::provides` then `DomainTag::try_new`.
/// Mocked:  none; the route reaches no outer-edge collaborator.
#[test]
fn a_tag_one_byte_over_the_maximum_is_refused_whole_before_the_factory_is_reached() {
    // Arrange
    let params = build_domain_tag_constructor_params(DomainTagConstructorParamsOverrides {
        bytes: Some(vec![b'a'; DOMAIN_TAG_MAXIMUM_LENGTH + 1]),
    });

    // Act
    let result = DomainTag::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(DomainTagTryNewErrorReturn::TooLong {
            length: DOMAIN_TAG_MAXIMUM_LENGTH + 1,
            maximum: DOMAIN_TAG_MAXIMUM_LENGTH
        })
    );
}

/// Contract: entry
///   `every_concrete_declaring_the_keccak256_identifier_hashes_the_context_the_encoding_factory_encodes_to_the_oracle`;
///   given `REFERENCE_TAG` and the bytes `create_encoding` returns for the
///   reference derivation context, the encoding of the returned scalar equals
///   the encoding of the scalar sampled from `expected_uniform_bytes(&tag,
///   bytes, S::UNIFORM_BYTES_LENGTH)`, `bytes` the bytes the encoding
///   returned.
/// Arrange: per case of `DECLARED_ENCODING_SELECTIONS` whose `identifier` is
///   `EncodingIdentifier::EthereumAbiV1`, the message bytes are the `bytes` of
///   the `Ok(EncodeSuccessReturn { bytes })` in `output.encoded` of
///   `create_encoding(&build_create_encoding_deps(
///   build_mock_i_encoding_consumer(&DerivationContextDescription,
///   build_derivation_context(Default::default()), Vec::new())),
///   build_create_encoding_params(CreateEncodingParamsOverrides { concrete:
///   Some(selection.concrete), identifier: Some(selection.identifier) }),
///   build_create_encoding_payload(Default::default()))`; the consumer's
///   `tag_bytes` are `REFERENCE_TAG.to_vec()` and its `message` is those
///   bytes; every case of `keccak_cases` over every member of
///   `PAIRING_CONCRETES`, so a hash that frames the message by the reference
///   message's shape, or reads other bytes than the encoding returned, yields
///   a scalar the oracle does not state.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)` per
///   concrete, over a consumer holding the encoded bytes.
/// Assert:  per concrete and case, the actual encoding equals the expected
///   encoding; outside the consumer, `created.is_ok()`.
/// Boundary: the public calls `create_encoding` over the official consumer
///   mock, `DomainTag::try_new`, `create_pairing`, `create_hash_to_scalar`,
///   `hash_to_scalar`, `sample_from_uniform_bytes`, and `encode_scalar`.
/// Mocked:  the encoding family's consumer, replaced by the official
///   `MockIEncodingConsumer` carrying the real description and the reference
///   context; it supplies the bytes the encoding concrete encodes and proves
///   nothing about the hash.
#[test]
fn every_concrete_declaring_the_keccak256_identifier_hashes_the_context_the_encoding_factory_encodes_to_the_oracle()
 {
    // Arrange
    struct Consumer {
        tag_bytes: Vec<u8>,
        message: Vec<u8>,
    }

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingAdapter>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let tag = DomainTag::try_new(build_domain_tag_constructor_params(
                DomainTagConstructorParamsOverrides {
                    bytes: Some(self.tag_bytes.clone()),
                },
            ))
            .expect("the tag bytes admit a domain tag");
            let keccak_cases = DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .copied()
                .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
                .collect::<Vec<_>>();
            for case in keccak_cases {
                let success = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    case,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the case's declared identifier admits its concrete");
                let adapter: &dyn IHashToScalarAdapter<P::Scalar> = &*success.adapter;
                let hashed = adapter
                    .hash_to_scalar(
                        build_hash_to_scalar_params(HashToScalarParamsOverrides {
                            tag: Some(&tag),
                        }),
                        build_hash_to_scalar_payload(HashToScalarPayloadOverrides {
                            message: Some(&self.message),
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
                    expected_uniform_bytes(&tag, &self.message, P::Scalar::UNIFORM_BYTES_LENGTH),
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
                assert_eq!(
                    actual.bytes.expose().as_ref().to_vec(),
                    expected.bytes.expose().as_ref().to_vec()
                );
            }
        }
    }

    for encoding_selection in DECLARED_ENCODING_SELECTIONS
        .iter()
        .filter(|selection| selection.identifier == EncodingIdentifier::EthereumAbiV1)
    {
        let consumer = build_mock_i_encoding_consumer(
            &DerivationContextDescription,
            build_derivation_context(Default::default()),
            Vec::new(),
        );
        let message = create_encoding(
            &build_create_encoding_deps(consumer),
            build_create_encoding_params(CreateEncodingParamsOverrides {
                concrete: Some(encoding_selection.concrete),
                identifier: Some(encoding_selection.identifier),
            }),
            build_create_encoding_payload(Default::default()),
        )
        .expect("an admitted encoding selection returns the consumer's output")
        .output
        .encoded
        .expect("the derivation context encodes")
        .bytes;

        for concrete in PAIRING_CONCRETES {
            let params = build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            });
            let deps = build_create_pairing_deps(
                Consumer {
                    tag_bytes: REFERENCE_TAG.to_vec(),
                    message: message.clone(),
                },
                Default::default(),
            );

            // Act
            let created = create_pairing(&deps, params, CreatePairingPayload);

            // Assert
            assert!(created.is_ok());
        }
    }
}
