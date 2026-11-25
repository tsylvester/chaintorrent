#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use domain::{Secret, build_secret};
use encoding::{
    CanonicalFieldsOverrides, CreateEncodingDepsOverrides, CreateEncodingParamsOverrides,
    DECLARED_ENCODING_SELECTIONS, EncodingIdentifier, LookupEncodingDeclarationDeps,
    LookupEncodingDeclarationParams, LookupEncodingDeclarationPayload, MockIEncodingContract,
    build_canonical_fields, build_create_encoding_deps, build_create_encoding_params,
    build_create_encoding_payload, build_mock_i_encoding_consumer, create_encoding,
    lookup_encoding_declaration,
};
use hash_to_scalar::{
    AdmitHashToScalarSelectionDeps, AdmitHashToScalarSelectionParamsOverrides,
    ConsumeHashToScalarParams, ConsumeHashToScalarPayload, CreateHashToScalarDepsOverrides,
    CreateHashToScalarParamsOverrides, DOMAIN_TAG_MAXIMUM_LENGTH, DomainTag,
    DomainTagConstructorParamsOverrides, DomainTagTryNewErrorReturn, HASH_TO_SCALAR_CONCRETES,
    HashToScalarConcrete, HashToScalarErrorReturn, HashToScalarIdentifier,
    HashToScalarParamsOverrides, HashToScalarPayloadOverrides, HashToScalarReturn,
    IHashToScalarAdapter, IHashToScalarConsumer, MockIHashToScalarAdapterFailureMode,
    REFERENCE_MESSAGE, REFERENCE_TAG, admit_hash_to_scalar_selection,
    build_admit_hash_to_scalar_selection_params, build_create_hash_to_scalar_deps,
    build_create_hash_to_scalar_params, build_create_hash_to_scalar_payload,
    build_domain_tag_constructor_params, build_hash_to_scalar_concrete,
    build_hash_to_scalar_params, build_hash_to_scalar_payload, create_hash_to_scalar,
    expected_uniform_bytes, lookup_hash_to_scalar_declaration,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParamsOverrides,
    CreatePairingPayload, EncodeScalarParams, EncodeScalarPayload, IPairingAdapter,
    IPairingConsumer, ISampleUniformScalar, PAIRING_CONCRETES, SampleUniformScalarErrorReturn,
    SampleUniformScalarParams, SampleUniformScalarPayload, admit_pairing_composition,
    build_admit_pairing_composition_deps, build_create_pairing_deps, build_create_pairing_params,
    create_pairing, lookup_pairing_declaration,
};
use serde_json::{Map, Value};

/// Contract: entry
///   `every_concrete_declaring_the_keccak256_identifier_hashes_the_reference_inputs_to_the_oracle`;
///   given `REFERENCE_TAG` built by `DomainTag::try_new` and
///   `REFERENCE_MESSAGE`, the encoding of the returned scalar equals the
///   encoding of the scalar sampled from
///   `expected_uniform_bytes(&tag, REFERENCE_MESSAGE, S::UNIFORM_BYTES_LENGTH)`.
/// Arrange: the consumer's `tag_bytes` as `REFERENCE_TAG.to_vec()` and
///   `message` as `REFERENCE_MESSAGE.to_vec()`; every case of `cases` over
///   every member of `PAIRING_CONCRETES`, each with its own scalar type, the
///   cases whose `payload.declaration` carries
///   `HashToScalarIdentifier::Keccak256V1` running the oracle and the rest
///   returning `None`.
/// Act:     `create_pairing` per concrete, whose consumer's call per case is
///   `create_hash_to_scalar`.
/// Assert:  per concrete and case, the actual encoding equals the expected
///   encoding through `success.output`, which the oracle states independently
///   of `hash_to_scalar`; outside the consumer, `created.is_ok()`.
/// Boundary: the public calls `DomainTag::try_new`, `create_pairing`,
///   `admit_hash_to_scalar_selection`, `create_hash_to_scalar`, `deps.lookup`
///   for `params.concrete`, `consume_hash_to_scalar` calling `hash_to_scalar`
///   on the adapter it receives, `sample_from_uniform_bytes`, and
///   `encode_scalar`, the route `lib.rs` then `domain_tag::provides` then
///   `DomainTag::try_new`, and `lib.rs` then `factory::provides` then
///   `create_hash_to_scalar`.
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
            struct HashConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
                tag_bytes: Vec<u8>,
                message: Vec<u8>,
            }

            impl<'a, P: IPairingAdapter> IHashToScalarConsumer<P::Scalar> for HashConsumer<'a, P> {
                type Output = Option<(Vec<u8>, Vec<u8>)>;

                fn consume_hash_to_scalar<K: IHashToScalarAdapter<P::Scalar>>(
                    &self,
                    _params: ConsumeHashToScalarParams,
                    payload: ConsumeHashToScalarPayload<P::Scalar, K>,
                ) -> Self::Output {
                    if !matches!(
                        payload.declaration.identifier,
                        HashToScalarIdentifier::Keccak256V1
                    ) {
                        return None;
                    }
                    let tag = DomainTag::try_new(build_domain_tag_constructor_params(
                        DomainTagConstructorParamsOverrides {
                            bytes: Some(self.tag_bytes.clone()),
                        },
                    ))
                    .expect("the tag bytes admit a domain tag");
                    let hashed = payload
                        .adapter
                        .hash_to_scalar(
                            build_hash_to_scalar_params(HashToScalarParamsOverrides {
                                tag: Some(&tag),
                            }),
                            build_hash_to_scalar_payload(HashToScalarPayloadOverrides {
                                message: Some(&self.message),
                            }),
                        )
                        .expect("hash_to_scalar admits the tag and message");
                    let actual = self
                        .pairing
                        .encode_scalar(
                            EncodeScalarParams,
                            EncodeScalarPayload {
                                scalar: hashed.scalar,
                            },
                        )
                        .expect("the returned scalar encodes");
                    let uniform: Secret<Vec<u8>> = build_secret(
                        expected_uniform_bytes(
                            &tag,
                            &self.message,
                            P::Scalar::UNIFORM_BYTES_LENGTH,
                        ),
                        Default::default(),
                    );
                    let sampled = P::Scalar::sample_from_uniform_bytes(
                        SampleUniformScalarParams,
                        SampleUniformScalarPayload { uniform },
                    )
                    .expect("the uniform input samples a scalar");
                    let expected = self
                        .pairing
                        .encode_scalar(
                            EncodeScalarParams,
                            EncodeScalarPayload {
                                scalar: sampled.scalar.expose().clone(),
                            },
                        )
                        .expect("the sampled scalar encodes");
                    Some((
                        actual.bytes.expose().as_ref().to_vec(),
                        expected.bytes.expose().as_ref().to_vec(),
                    ))
                }
            }

            let mut cases = Vec::new();
            for member in HASH_TO_SCALAR_CONCRETES {
                let admitted = admit_hash_to_scalar_selection(
                    &AdmitHashToScalarSelectionDeps,
                    build_admit_hash_to_scalar_selection_params(
                        AdmitHashToScalarSelectionParamsOverrides {
                            concrete: Some(build_hash_to_scalar_concrete(
                                *member,
                                Default::default(),
                            )),
                        },
                    ),
                    Value::Null,
                )
                .expect("the member's document admits a selection");
                cases.push(build_create_hash_to_scalar_params(
                    CreateHashToScalarParamsOverrides {
                        concrete: Some(admitted.selection),
                    },
                ));
            }
            for case in cases {
                let success = create_hash_to_scalar::<P::Scalar, _>(
                    &build_create_hash_to_scalar_deps(
                        HashConsumer {
                            pairing: &consumed.adapter,
                            tag_bytes: self.tag_bytes.clone(),
                            message: self.message.clone(),
                        },
                        CreateHashToScalarDepsOverrides {
                            lookup: Some(lookup_hash_to_scalar_declaration),
                            ..Default::default()
                        },
                    ),
                    case,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the admitted selection is constructed by the factory");
                if let Some((actual, expected)) = success.output {
                    assert_eq!(actual, expected);
                }
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
            lookup_pairing_declaration,
            admit_pairing_composition,
            build_admit_pairing_composition_deps(lookup_pairing_declaration, Default::default()),
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
///   `REFERENCE_TAG`, and the adapter the consumer receives hashes that tag,
///   so the encoding of the returned scalar equals the encoding of the scalar
///   sampled from
///   `expected_uniform_bytes(&tag, REFERENCE_MESSAGE, S::UNIFORM_BYTES_LENGTH)`.
/// Arrange: as
///   `every_concrete_declaring_the_keccak256_identifier_hashes_the_reference_inputs_to_the_oracle`,
///   the consumer also reading the tag's bytes through `as_bytes`; the
///   variation is a read that truncates, trims, or re-encodes the tag, which
///   yields a scalar the oracle does not state.
/// Act:     `create_pairing` per concrete.
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
            struct HashConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
                tag_bytes: Vec<u8>,
                message: Vec<u8>,
            }

            impl<'a, P: IPairingAdapter> IHashToScalarConsumer<P::Scalar> for HashConsumer<'a, P> {
                type Output = Option<(Vec<u8>, Vec<u8>)>;

                fn consume_hash_to_scalar<K: IHashToScalarAdapter<P::Scalar>>(
                    &self,
                    _params: ConsumeHashToScalarParams,
                    payload: ConsumeHashToScalarPayload<P::Scalar, K>,
                ) -> Self::Output {
                    if !matches!(
                        payload.declaration.identifier,
                        HashToScalarIdentifier::Keccak256V1
                    ) {
                        return None;
                    }
                    let tag = DomainTag::try_new(build_domain_tag_constructor_params(
                        DomainTagConstructorParamsOverrides {
                            bytes: Some(self.tag_bytes.clone()),
                        },
                    ))
                    .expect("the tag bytes admit a domain tag");
                    assert_eq!(tag.as_bytes(), REFERENCE_TAG);
                    let hashed = payload
                        .adapter
                        .hash_to_scalar(
                            build_hash_to_scalar_params(HashToScalarParamsOverrides {
                                tag: Some(&tag),
                            }),
                            build_hash_to_scalar_payload(HashToScalarPayloadOverrides {
                                message: Some(&self.message),
                            }),
                        )
                        .expect("hash_to_scalar admits the tag and message");
                    let actual = self
                        .pairing
                        .encode_scalar(
                            EncodeScalarParams,
                            EncodeScalarPayload {
                                scalar: hashed.scalar,
                            },
                        )
                        .expect("the returned scalar encodes");
                    let uniform: Secret<Vec<u8>> = build_secret(
                        expected_uniform_bytes(
                            &tag,
                            &self.message,
                            P::Scalar::UNIFORM_BYTES_LENGTH,
                        ),
                        Default::default(),
                    );
                    let sampled = P::Scalar::sample_from_uniform_bytes(
                        SampleUniformScalarParams,
                        SampleUniformScalarPayload { uniform },
                    )
                    .expect("the uniform input samples a scalar");
                    let expected = self
                        .pairing
                        .encode_scalar(
                            EncodeScalarParams,
                            EncodeScalarPayload {
                                scalar: sampled.scalar.expose().clone(),
                            },
                        )
                        .expect("the sampled scalar encodes");
                    Some((
                        actual.bytes.expose().as_ref().to_vec(),
                        expected.bytes.expose().as_ref().to_vec(),
                    ))
                }
            }

            let mut cases = Vec::new();
            for member in HASH_TO_SCALAR_CONCRETES {
                let admitted = admit_hash_to_scalar_selection(
                    &AdmitHashToScalarSelectionDeps,
                    build_admit_hash_to_scalar_selection_params(
                        AdmitHashToScalarSelectionParamsOverrides {
                            concrete: Some(build_hash_to_scalar_concrete(
                                *member,
                                Default::default(),
                            )),
                        },
                    ),
                    Value::Null,
                )
                .expect("the member's document admits a selection");
                cases.push(build_create_hash_to_scalar_params(
                    CreateHashToScalarParamsOverrides {
                        concrete: Some(admitted.selection),
                    },
                ));
            }
            for case in cases {
                let success = create_hash_to_scalar::<P::Scalar, _>(
                    &build_create_hash_to_scalar_deps(
                        HashConsumer {
                            pairing: &consumed.adapter,
                            tag_bytes: self.tag_bytes.clone(),
                            message: self.message.clone(),
                        },
                        CreateHashToScalarDepsOverrides {
                            lookup: Some(lookup_hash_to_scalar_declaration),
                            ..Default::default()
                        },
                    ),
                    case,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the admitted selection is constructed by the factory");
                if let Some((actual, expected)) = success.output {
                    assert_eq!(actual, expected);
                }
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
            lookup_pairing_declaration,
            admit_pairing_composition,
            build_admit_pairing_composition_deps(lookup_pairing_declaration, Default::default()),
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
///   given `REFERENCE_TAG` and the bytes `create_encoding` returns for a value
///   of the family's mock description, the encoding of the returned scalar
///   equals the encoding of the scalar sampled from
///   `expected_uniform_bytes(&tag, bytes, S::UNIFORM_BYTES_LENGTH)`, `bytes`
///   the bytes the encoding returned.
/// Arrange: per case of `DECLARED_ENCODING_SELECTIONS` whose `identifier` is
///   `EncodingIdentifier::EthereumAbiV1`, the message bytes are the `bytes` of
///   the `Ok(EncodeSuccessReturn { bytes })` in `output.encoded` of
///   `create_encoding` over the official consumer mock; the consumer's
///   `tag_bytes` are `REFERENCE_TAG.to_vec()` and its `message` is those
///   bytes; every case of `cases` over every member of `PAIRING_CONCRETES`,
///   the cases whose `payload.declaration` carries
///   `HashToScalarIdentifier::Keccak256V1` running the oracle, so a hash that
///   frames the message by the reference message's shape, or reads other
///   bytes than the encoding returned, yields a scalar the oracle does not
///   state.
/// Act:     `create_pairing` per concrete, over a consumer holding the
///   encoded bytes.
/// Assert:  per concrete and case, the actual encoding equals the expected
///   encoding through `success.output`.
/// Boundary: the public calls `create_encoding` over the official consumer
///   mock, `DomainTag::try_new`, `create_pairing`,
///   `admit_hash_to_scalar_selection`, `create_hash_to_scalar`, `deps.lookup`
///   for `params.concrete`, `consume_hash_to_scalar` calling `hash_to_scalar`
///   on the adapter it receives, `sample_from_uniform_bytes`, and
///   `encode_scalar`.
/// Mocked:  the encoding family's consumer, replaced by the official
///   `MockIEncodingConsumer` carrying the mock description and a value of it
///   at defaults; it supplies the bytes the encoding concrete encodes and
///   proves nothing about the hash.
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
            struct HashConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
                tag_bytes: Vec<u8>,
                message: Vec<u8>,
            }

            impl<'a, P: IPairingAdapter> IHashToScalarConsumer<P::Scalar> for HashConsumer<'a, P> {
                type Output = Option<(Vec<u8>, Vec<u8>)>;

                fn consume_hash_to_scalar<K: IHashToScalarAdapter<P::Scalar>>(
                    &self,
                    _params: ConsumeHashToScalarParams,
                    payload: ConsumeHashToScalarPayload<P::Scalar, K>,
                ) -> Self::Output {
                    if !matches!(
                        payload.declaration.identifier,
                        HashToScalarIdentifier::Keccak256V1
                    ) {
                        return None;
                    }
                    let tag = DomainTag::try_new(build_domain_tag_constructor_params(
                        DomainTagConstructorParamsOverrides {
                            bytes: Some(self.tag_bytes.clone()),
                        },
                    ))
                    .expect("the tag bytes admit a domain tag");
                    let hashed = payload
                        .adapter
                        .hash_to_scalar(
                            build_hash_to_scalar_params(HashToScalarParamsOverrides {
                                tag: Some(&tag),
                            }),
                            build_hash_to_scalar_payload(HashToScalarPayloadOverrides {
                                message: Some(&self.message),
                            }),
                        )
                        .expect("hash_to_scalar admits the tag and message");
                    let actual = self
                        .pairing
                        .encode_scalar(
                            EncodeScalarParams,
                            EncodeScalarPayload {
                                scalar: hashed.scalar,
                            },
                        )
                        .expect("the returned scalar encodes");
                    let uniform: Secret<Vec<u8>> = build_secret(
                        expected_uniform_bytes(
                            &tag,
                            &self.message,
                            P::Scalar::UNIFORM_BYTES_LENGTH,
                        ),
                        Default::default(),
                    );
                    let sampled = P::Scalar::sample_from_uniform_bytes(
                        SampleUniformScalarParams,
                        SampleUniformScalarPayload { uniform },
                    )
                    .expect("the uniform input samples a scalar");
                    let expected = self
                        .pairing
                        .encode_scalar(
                            EncodeScalarParams,
                            EncodeScalarPayload {
                                scalar: sampled.scalar.expose().clone(),
                            },
                        )
                        .expect("the sampled scalar encodes");
                    Some((
                        actual.bytes.expose().as_ref().to_vec(),
                        expected.bytes.expose().as_ref().to_vec(),
                    ))
                }
            }

            let mut cases = Vec::new();
            for member in HASH_TO_SCALAR_CONCRETES {
                let admitted = admit_hash_to_scalar_selection(
                    &AdmitHashToScalarSelectionDeps,
                    build_admit_hash_to_scalar_selection_params(
                        AdmitHashToScalarSelectionParamsOverrides {
                            concrete: Some(build_hash_to_scalar_concrete(
                                *member,
                                Default::default(),
                            )),
                        },
                    ),
                    Value::Null,
                )
                .expect("the member's document admits a selection");
                cases.push(build_create_hash_to_scalar_params(
                    CreateHashToScalarParamsOverrides {
                        concrete: Some(admitted.selection),
                    },
                ));
            }
            for case in cases {
                let success = create_hash_to_scalar::<P::Scalar, _>(
                    &build_create_hash_to_scalar_deps(
                        HashConsumer {
                            pairing: &consumed.adapter,
                            tag_bytes: self.tag_bytes.clone(),
                            message: self.message.clone(),
                        },
                        CreateHashToScalarDepsOverrides {
                            lookup: Some(lookup_hash_to_scalar_declaration),
                            ..Default::default()
                        },
                    ),
                    case,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the admitted selection is constructed by the factory");
                if let Some((actual, expected)) = success.output {
                    assert_eq!(actual, expected);
                }
            }
        }
    }

    for selection in DECLARED_ENCODING_SELECTIONS {
        let Ok(looked_up) = lookup_encoding_declaration(
            &LookupEncodingDeclarationDeps,
            LookupEncodingDeclarationParams {
                concrete: selection.selection,
            },
            LookupEncodingDeclarationPayload,
        );
        if looked_up.declaration.identifier != EncodingIdentifier::EthereumAbiV1 {
            continue;
        }
        let encoded = create_encoding(
            &build_create_encoding_deps(
                build_mock_i_encoding_consumer(
                    &MockIEncodingContract,
                    build_canonical_fields(CanonicalFieldsOverrides::default()),
                    Vec::new(),
                ),
                CreateEncodingDepsOverrides {
                    lookup: Some(lookup_encoding_declaration),
                },
            ),
            build_create_encoding_params(CreateEncodingParamsOverrides {
                selection: Some(selection.selection),
            }),
            build_create_encoding_payload(Default::default()),
        )
        .expect("the encoding selection constructs its concrete")
        .output
        .encoded
        .expect("the consumer's encode produced bytes")
        .bytes;

        for concrete in PAIRING_CONCRETES {
            let params = build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            });
            let deps = build_create_pairing_deps(
                Consumer {
                    tag_bytes: REFERENCE_TAG.to_vec(),
                    message: encoded.clone(),
                },
                lookup_pairing_declaration,
                admit_pairing_composition,
                build_admit_pairing_composition_deps(
                    lookup_pairing_declaration,
                    Default::default(),
                ),
                Default::default(),
            );

            // Act
            let created = create_pairing(&deps, params, CreatePairingPayload);

            // Assert
            assert!(created.is_ok());
        }
    }
}

/// Contract: carried `admit_hash_to_scalar_selection` entry
///   `every_declared_member_is_admitted_to_the_selection_that_names_it`; for
///   each member of `HASH_TO_SCALAR_CONCRETES`, the selection
///   `admit_hash_to_scalar_selection` returns is accepted by
///   `create_hash_to_scalar` as `params.concrete`, which yields
///   `Ok(CreateHashToScalarSuccessReturn { output })` whose `output` observes
///   `hash_to_scalar` yielding `Ok`.
/// Arrange: `HashConsumer` a unit struct declared inside the block
///   implementing `IHashToScalarConsumer<S>` for every `S` with `type Output
///   = HashToScalarReturn<S>`, its `consume_hash_to_scalar` calling
///   `payload.adapter.hash_to_scalar` with `build_hash_to_scalar_params` and
///   `build_hash_to_scalar_payload` at defaults on the adapter it receives
///   and returning its return; per member and document, the pairing consumer
///   admits the selection with the fieldless `AdmitHashToScalarSelectionDeps`
///   over each of `Value::Null` and an empty `Value::Object(Map::new())`,
///   calls `create_hash_to_scalar::<P::Scalar, _>` with
///   `build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
///   concrete: Some(selection) })` and deps
///   `build_create_hash_to_scalar_deps(HashConsumer, { lookup:
///   lookup_hash_to_scalar_declaration })`, and binds its `Ok` through
///   `expect`; every member of `PAIRING_CONCRETES`, so a selection the
///   admission returns that the factory refuses or cannot construct fails the
///   `expect`.
/// Act:     `create_pairing` per concrete, whose consumer's call per member
///   and document is `create_hash_to_scalar`, the `hash_to_scalar` occurring
///   inside `consume_hash_to_scalar`.
/// Assert:  in the consumer, per member and document,
///   `assert!(success.output.is_ok())`; outside the consumer,
///   `created.is_ok()`.
/// Boundary: the public calls `admit_hash_to_scalar_selection`,
///   `create_pairing`, `create_hash_to_scalar`, `deps.lookup` for
///   `params.concrete`, and `consume_hash_to_scalar` calling `hash_to_scalar`
///   on the adapter it receives, the route `lib.rs` then
///   `admit_hash_to_scalar_selection::provides` then
///   `admit_hash_to_scalar_selection`, and `lib.rs` then `factory::provides`
///   then `create_hash_to_scalar`.
/// Mocked:  none; the route reaches no outer-edge collaborator.
#[test]
fn every_selection_the_admission_returns_is_constructed_by_the_factory() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingAdapter>(
            &self,
            _params: ConsumePairingParams,
            _consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct HashConsumer;

            impl<S: ISampleUniformScalar + Clone> IHashToScalarConsumer<S> for HashConsumer {
                type Output = HashToScalarReturn<S>;

                fn consume_hash_to_scalar<K: IHashToScalarAdapter<S>>(
                    &self,
                    _params: ConsumeHashToScalarParams,
                    payload: ConsumeHashToScalarPayload<S, K>,
                ) -> Self::Output {
                    payload.adapter.hash_to_scalar(
                        build_hash_to_scalar_params(Default::default()),
                        build_hash_to_scalar_payload(Default::default()),
                    )
                }
            }

            for member in HASH_TO_SCALAR_CONCRETES {
                for document in [Value::Null, Value::Object(Map::new())] {
                    let admitted = admit_hash_to_scalar_selection(
                        &AdmitHashToScalarSelectionDeps,
                        build_admit_hash_to_scalar_selection_params(
                            AdmitHashToScalarSelectionParamsOverrides {
                                concrete: Some(build_hash_to_scalar_concrete(
                                    *member,
                                    Default::default(),
                                )),
                            },
                        ),
                        document,
                    )
                    .expect("the member's document admits a selection");
                    let success = create_hash_to_scalar::<P::Scalar, _>(
                        &build_create_hash_to_scalar_deps(
                            HashConsumer,
                            CreateHashToScalarDepsOverrides {
                                lookup: Some(lookup_hash_to_scalar_declaration),
                                ..Default::default()
                            },
                        ),
                        build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                            concrete: Some(admitted.selection),
                        }),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the admitted selection is constructed by the factory");
                    assert!(success.output.is_ok());
                }
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(
            Consumer,
            lookup_pairing_declaration,
            admit_pairing_composition,
            build_admit_pairing_composition_deps(lookup_pairing_declaration, Default::default()),
            Default::default(),
        );

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: carried `admit_hash_to_scalar_selection` entry
///   `a_mock_member_is_admitted_to_the_selection_naming_the_selected_mock`;
///   the selection `admit_hash_to_scalar_selection` returns for
///   `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::SamplingRefused)`
///   and an empty `Value::Object(Map::new())`, handed to
///   `create_hash_to_scalar`, yields `Ok(CreateHashToScalarSuccessReturn {
///   output })` whose `output` is `Err(HashToScalarErrorReturn::Sampling(
///   SampleUniformScalarErrorReturn::WrongLength { expected:
///   P::Scalar::UNIFORM_BYTES_LENGTH, actual: 0 }))` from `hash_to_scalar`.
/// Arrange: the consumer builds params for `admit_hash_to_scalar_selection`
///   with that member, a mode other than the `Succeeds` mode the mock
///   declaration constant names, and the fieldless
///   `AdmitHashToScalarSelectionDeps`, admits the selection, calls
///   `create_hash_to_scalar::<P::Scalar, _>` with
///   `build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
///   concrete: Some(selection) })` and deps
///   `build_create_hash_to_scalar_deps(HashConsumer, { lookup:
///   lookup_hash_to_scalar_declaration })`, `HashConsumer` as above
///   returning `payload.adapter.hash_to_scalar`'s return over a tag
///   `build_domain_tag` builds at defaults, and binds its `Ok` through
///   `expect`; every member of `PAIRING_CONCRETES`, so an admission that
///   returns the `Succeeds` member yields an adapter whose hash succeeds.
/// Act:     `create_pairing` per concrete, whose consumer's single call is
///   `create_hash_to_scalar`, the `hash_to_scalar` occurring inside
///   `consume_hash_to_scalar`.
/// Assert:  in the consumer, per concrete, `assert_eq!(success.output.err(),
///   Some(HashToScalarErrorReturn::Sampling(
///   SampleUniformScalarErrorReturn::WrongLength { expected:
///   P::Scalar::UNIFORM_BYTES_LENGTH, actual: 0 })))`, the bound read from
///   the received scalar type; outside the consumer, `created.is_ok()`.
/// Boundary: the public calls of
///   `every_selection_the_admission_returns_is_constructed_by_the_factory`,
///   the route ending at the mock concrete's `try_new` and
///   `consume_hash_to_scalar` calling `hash_to_scalar`.
/// Mocked:  the family's mock member, selected by the params, the official
///   collaborator the `mocks` feature provides.
#[test]
fn a_mock_selection_the_admission_returns_reaches_the_factory_with_its_failure_mode() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingAdapter>(
            &self,
            _params: ConsumePairingParams,
            _consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct HashConsumer;

            impl<S: ISampleUniformScalar + Clone> IHashToScalarConsumer<S> for HashConsumer {
                type Output = HashToScalarReturn<S>;

                fn consume_hash_to_scalar<K: IHashToScalarAdapter<S>>(
                    &self,
                    _params: ConsumeHashToScalarParams,
                    payload: ConsumeHashToScalarPayload<S, K>,
                ) -> Self::Output {
                    let tag =
                        DomainTag::try_new(build_domain_tag_constructor_params(Default::default()))
                            .expect("the default tag bytes admit a domain tag");
                    payload.adapter.hash_to_scalar(
                        build_hash_to_scalar_params(HashToScalarParamsOverrides {
                            tag: Some(&tag),
                        }),
                        build_hash_to_scalar_payload(Default::default()),
                    )
                }
            }

            let admitted = admit_hash_to_scalar_selection(
                &AdmitHashToScalarSelectionDeps,
                build_admit_hash_to_scalar_selection_params(
                    AdmitHashToScalarSelectionParamsOverrides {
                        concrete: Some(HashToScalarConcrete::Mock(
                            MockIHashToScalarAdapterFailureMode::SamplingRefused,
                        )),
                    },
                ),
                Value::Object(Map::new()),
            )
            .expect("the mock member's empty document admits a selection");
            let success = create_hash_to_scalar::<P::Scalar, _>(
                &build_create_hash_to_scalar_deps(
                    HashConsumer,
                    CreateHashToScalarDepsOverrides {
                        lookup: Some(lookup_hash_to_scalar_declaration),
                        ..Default::default()
                    },
                ),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: Some(admitted.selection),
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the admitted mock selection is constructed by the factory");
            assert_eq!(
                success.output.err(),
                Some(HashToScalarErrorReturn::Sampling(
                    SampleUniformScalarErrorReturn::WrongLength {
                        expected: P::Scalar::UNIFORM_BYTES_LENGTH,
                        actual: 0,
                    }
                ))
            );
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(
            Consumer,
            lookup_pairing_declaration,
            admit_pairing_composition,
            build_admit_pairing_composition_deps(lookup_pairing_declaration, Default::default()),
            Default::default(),
        );

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}
