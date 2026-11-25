#![allow(clippy::expect_used)]

use super::create_hash_to_scalar;
use super::provides::{
    BINDING_LONGER_TAG, BINDING_SHORTER_MESSAGE, ConsumeHashToScalarParams,
    ConsumeHashToScalarPayload, CreateHashToScalarDepsOverrides, CreateHashToScalarParams,
    CreateHashToScalarParamsOverrides, EMPTY_MESSAGE, HASH_TO_SCALAR_CONCRETES,
    HashToScalarConcrete, HashToScalarErrorReturn, HashToScalarIdentifier,
    HashToScalarParamsOverrides, HashToScalarPayloadOverrides, HashToScalarReturn,
    IHashToScalarAdapter, IHashToScalarConsumer, MockIHashToScalarAdapterFailureMode, OTHER_TAG,
    REFERENCE_MESSAGE, REFERENCE_TAG, build_create_hash_to_scalar_deps,
    build_create_hash_to_scalar_params, build_create_hash_to_scalar_payload,
    build_hash_to_scalar_concrete, build_hash_to_scalar_params, build_hash_to_scalar_payload,
    expected_uniform_bytes,
};
use crate::domain_tag::provides::{DomainTagConstructorParamsOverrides, build_domain_tag};
use crate::lookup_hash_to_scalar_declaration::provides::lookup_hash_to_scalar_declaration;
use domain::{Secret, build_secret};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParamsOverrides,
    CreatePairingPayload, EncodeScalarParams, EncodeScalarPayload, IPairingAdapter,
    IPairingConsumer, ISampleUniformScalar, PAIRING_CONCRETES, SampleUniformScalarErrorReturn,
    SampleUniformScalarParams, SampleUniformScalarPayload, admit_pairing_composition,
    build_admit_pairing_composition_deps, build_create_pairing_deps, build_create_pairing_params,
    create_pairing, lookup_pairing_declaration,
};

/// Contract: entry `every_admitted_selection_is_constructed_into_an_adapter_that_hashes`;
///   given the selection of a member of `HASH_TO_SCALAR_CONCRETES` or of the
///   mock member under `Succeeds`, `create_hash_to_scalar` yields
///   `Ok(CreateHashToScalarSuccessReturn { output })` whose `output` observes
///   `hash_to_scalar` over the default tag and message yielding `Ok`.
/// Arrange: `Consumer` whose `consume_pairing` calls, per case of `cases`,
///   `create_hash_to_scalar::<P::Scalar, _>` with deps
///   `build_create_hash_to_scalar_deps(HashConsumer, { lookup:
///   lookup_hash_to_scalar_declaration })`, `HashConsumer` calling
///   `hash_to_scalar` on the adapter it receives over the default tag and
///   message and returning its return; the cases are every selection the
///   family declares and the mock selection, so a factory that refuses or
///   fails to construct a selection the family declares fails the `expect`.
/// Act:     `create_pairing` per concrete, whose consumer's call per case is
///   `create_hash_to_scalar`, the `hash_to_scalar` occurring inside
///   `consume_hash_to_scalar`.
/// Assert:  in the consumer, per case, the success arm is bound through
///   `expect` and `success.output.is_ok()` holds; outside the consumer,
///   `created.is_ok()`.
/// Boundary: the real chain is `create_pairing`, the arm's `try_new`,
///   `create_hash_to_scalar`, `deps.lookup` for `params.concrete`, the
///   selected concrete's `try_new`, `consume_hash_to_scalar` calling
///   `hash_to_scalar` on the adapter it receives, and the received scalar's
///   `sample_from_uniform_bytes`.
/// Mocked:  none.
#[test]
fn every_admitted_selection_is_constructed_into_an_adapter_that_hashes() {
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

            let mut cases: Vec<CreateHashToScalarParams> = HASH_TO_SCALAR_CONCRETES
                .iter()
                .map(|member| {
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(build_hash_to_scalar_concrete(*member, Default::default())),
                    })
                })
                .collect();
            cases.push(build_create_hash_to_scalar_params(Default::default()));
            for case in cases {
                let success = create_hash_to_scalar::<P::Scalar, _>(
                    &build_create_hash_to_scalar_deps(
                        HashConsumer,
                        CreateHashToScalarDepsOverrides {
                            lookup: Some(lookup_hash_to_scalar_declaration),
                        },
                    ),
                    case,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the case's declared selection admits its concrete");
                assert!(success.output.is_ok());
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

/// Contract: entry `every_concrete_returns_the_sampling_bound_refusal_whole`;
///   given the mock selection under
///   `MockIHashToScalarAdapterFailureMode::SamplingRefused`,
///   `create_hash_to_scalar` yields `Ok(CreateHashToScalarSuccessReturn {
///   output })` whose `output` is `Err(HashToScalarErrorReturn::Sampling(
///   SampleUniformScalarErrorReturn::WrongLength { expected:
///   P::Scalar::UNIFORM_BYTES_LENGTH, actual: 0 }))` from `hash_to_scalar`.
/// Arrange: `Consumer` whose `consume_pairing` calls
///   `create_hash_to_scalar::<P::Scalar, _>` with the mock selection under
///   `SamplingRefused` and deps `build_create_hash_to_scalar_deps(
///   HashConsumer, { lookup: lookup_hash_to_scalar_declaration })`,
///   `HashConsumer` building the tag with `build_domain_tag` and calling
///   `hash_to_scalar` on the adapter it receives and returning its return;
///   the failure mode is the case that reaches the arm, since no input takes
///   it for a real concrete, whose sampler receives exactly its bound; every
///   member of `PAIRING_CONCRETES` supplies its own bound.
/// Act:     `create_pairing` per concrete, whose consumer's single call is
///   `create_hash_to_scalar`, the `hash_to_scalar` occurring inside
///   `consume_hash_to_scalar`.
/// Assert:  in the consumer, per concrete, `success.output.err()` equals
///   `Some(HashToScalarErrorReturn::Sampling(
///   SampleUniformScalarErrorReturn::WrongLength { expected:
///   P::Scalar::UNIFORM_BYTES_LENGTH, actual: 0 }))` by `assert_eq!`, the
///   bound read from the received scalar type and the length from the empty
///   input the failure mode supplies; outside the consumer,
///   `created.is_ok()`.
/// Boundary: the real chain is `create_pairing`, the arm's `try_new`,
///   `create_hash_to_scalar`, `deps.lookup` for `params.concrete`, the mock
///   concrete's `try_new`, and `consume_hash_to_scalar` calling
///   `hash_to_scalar` on the adapter it receives over the received scalar's
///   `sample_from_uniform_bytes`.
/// Mocked:  none.
#[test]
fn every_concrete_returns_the_sampling_bound_refusal_whole() {
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
                    let tag = build_domain_tag(Default::default());
                    payload.adapter.hash_to_scalar(
                        build_hash_to_scalar_params(HashToScalarParamsOverrides {
                            tag: Some(&tag),
                        }),
                        build_hash_to_scalar_payload(Default::default()),
                    )
                }
            }

            let success = create_hash_to_scalar::<P::Scalar, _>(
                &build_create_hash_to_scalar_deps(
                    HashConsumer,
                    CreateHashToScalarDepsOverrides {
                        lookup: Some(lookup_hash_to_scalar_declaration),
                    },
                ),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: Some(HashToScalarConcrete::Mock(
                        MockIHashToScalarAdapterFailureMode::SamplingRefused,
                    )),
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the mock selection admits under the failure mode");
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

/// Contract: absorbed entry
///   `hash_to_scalar_matches_the_oracle_for_the_reference_tag_and_message`;
///   given `REFERENCE_TAG` and `REFERENCE_MESSAGE`, the encoding of the scalar
///   `payload.adapter` yields equals the encoding of the scalar sampled from
///   `expected_uniform_bytes(&tag, REFERENCE_MESSAGE,
///   S::UNIFORM_BYTES_LENGTH)`, observed through `success.output`.
/// Arrange: oracle block with `TAG` as `REFERENCE_TAG` and `MESSAGE` as
///   `REFERENCE_MESSAGE`; every case of `cases` over every member of
///   `PAIRING_CONCRETES`, each with its own scalar type, the cases whose
///   `payload.declaration` carries `HashToScalarIdentifier::Keccak256V1`
///   running the oracle and the rest returning `None`.
/// Act:     `create_pairing` per concrete, whose consumer's call per case is
///   `create_hash_to_scalar`.
/// Assert:  in the consumer, per case, the success arm is bound through
///   `expect`; for every `Some((actual, expected))` in `success.output`,
///   `assert_eq!(actual, expected)`, which the oracle states independently of
///   `hash_to_scalar`; outside the consumer, `created.is_ok()`.
/// Boundary: the real chain is `create_pairing`, the arm's `try_new`,
///   `create_hash_to_scalar`, `deps.lookup` for `params.concrete`,
///   `DomainTag::try_new`, the selected concrete's `try_new`,
///   `consume_hash_to_scalar` calling `hash_to_scalar` on the adapter it
///   receives, the received scalar's `sample_from_uniform_bytes`, and
///   `encode_scalar`.
/// Mocked:  none.
#[test]
fn every_concrete_declaring_the_keccak256_identifier_matches_the_oracle_for_the_reference_tag_and_message()
 {
    // Arrange
    const TAG: &[u8] = REFERENCE_TAG;
    const MESSAGE: &[u8] = REFERENCE_MESSAGE;

    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingAdapter>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct HashConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
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
                    let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                        bytes: Some(TAG.to_vec()),
                    });
                    let hashed = payload
                        .adapter
                        .hash_to_scalar(
                            build_hash_to_scalar_params(HashToScalarParamsOverrides {
                                tag: Some(&tag),
                            }),
                            build_hash_to_scalar_payload(HashToScalarPayloadOverrides {
                                message: Some(MESSAGE),
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
                        expected_uniform_bytes(&tag, MESSAGE, P::Scalar::UNIFORM_BYTES_LENGTH),
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

            let cases: Vec<CreateHashToScalarParams> = HASH_TO_SCALAR_CONCRETES
                .iter()
                .map(|member| {
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(build_hash_to_scalar_concrete(*member, Default::default())),
                    })
                })
                .collect();
            for case in cases {
                let success = create_hash_to_scalar::<P::Scalar, _>(
                    &build_create_hash_to_scalar_deps(
                        HashConsumer {
                            pairing: &consumed.adapter,
                        },
                        CreateHashToScalarDepsOverrides {
                            lookup: Some(lookup_hash_to_scalar_declaration),
                        },
                    ),
                    case,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the case's declared selection admits its concrete");
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

/// Contract: absorbed entry
///   `hash_to_scalar_matches_the_oracle_for_the_empty_message`; given
///   `REFERENCE_TAG` and `EMPTY_MESSAGE`, the encoding of the returned scalar
///   equals the encoding of the scalar sampled from
///   `expected_uniform_bytes(&tag, EMPTY_MESSAGE, S::UNIFORM_BYTES_LENGTH)`.
/// Arrange: oracle block with `TAG` as `REFERENCE_TAG` and `MESSAGE` as
///   `EMPTY_MESSAGE`, the message segment absent; every case of `cases` over
///   every member of `PAIRING_CONCRETES`.
/// Act:     `create_pairing` per concrete.
/// Assert:  as
///   `every_concrete_declaring_the_keccak256_identifier_matches_the_oracle_for_the_reference_tag_and_message`.
/// Boundary: as
///   `every_concrete_declaring_the_keccak256_identifier_matches_the_oracle_for_the_reference_tag_and_message`.
/// Mocked:  none.
#[test]
fn every_concrete_declaring_the_keccak256_identifier_matches_the_oracle_for_the_empty_message() {
    // Arrange
    const TAG: &[u8] = REFERENCE_TAG;
    const MESSAGE: &[u8] = EMPTY_MESSAGE;

    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingAdapter>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct HashConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
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
                    let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                        bytes: Some(TAG.to_vec()),
                    });
                    let hashed = payload
                        .adapter
                        .hash_to_scalar(
                            build_hash_to_scalar_params(HashToScalarParamsOverrides {
                                tag: Some(&tag),
                            }),
                            build_hash_to_scalar_payload(HashToScalarPayloadOverrides {
                                message: Some(MESSAGE),
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
                        expected_uniform_bytes(&tag, MESSAGE, P::Scalar::UNIFORM_BYTES_LENGTH),
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

            let cases: Vec<CreateHashToScalarParams> = HASH_TO_SCALAR_CONCRETES
                .iter()
                .map(|member| {
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(build_hash_to_scalar_concrete(*member, Default::default())),
                    })
                })
                .collect();
            for case in cases {
                let success = create_hash_to_scalar::<P::Scalar, _>(
                    &build_create_hash_to_scalar_deps(
                        HashConsumer {
                            pairing: &consumed.adapter,
                        },
                        CreateHashToScalarDepsOverrides {
                            lookup: Some(lookup_hash_to_scalar_declaration),
                        },
                    ),
                    case,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the case's declared selection admits its concrete");
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

/// Contract: absorbed entry `hash_to_scalar_binds_the_tag_length`; given
///   `BINDING_LONGER_TAG` and `BINDING_SHORTER_MESSAGE`, which concatenate to
///   the same bytes as a split with a tag one byte shorter, the encoding of
///   the returned scalar equals the encoding of the scalar sampled from
///   `expected_uniform_bytes(&tag, BINDING_SHORTER_MESSAGE,
///   S::UNIFORM_BYTES_LENGTH)`, whose input begins with the length of the
///   longer tag.
/// Arrange: oracle block with `TAG` as `BINDING_LONGER_TAG` and `MESSAGE` as
///   `BINDING_SHORTER_MESSAGE`; every case of `cases` over every member of
///   `PAIRING_CONCRETES`, so a prefix that is absent or is not the given
///   tag's length yields a scalar the oracle does not state.
/// Act:     `create_pairing` per concrete.
/// Assert:  as
///   `every_concrete_declaring_the_keccak256_identifier_matches_the_oracle_for_the_reference_tag_and_message`.
/// Boundary: as
///   `every_concrete_declaring_the_keccak256_identifier_matches_the_oracle_for_the_reference_tag_and_message`.
/// Mocked:  none.
#[test]
fn every_concrete_declaring_the_keccak256_identifier_binds_the_tag_length() {
    // Arrange
    const TAG: &[u8] = BINDING_LONGER_TAG;
    const MESSAGE: &[u8] = BINDING_SHORTER_MESSAGE;

    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingAdapter>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct HashConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
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
                    let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                        bytes: Some(TAG.to_vec()),
                    });
                    let hashed = payload
                        .adapter
                        .hash_to_scalar(
                            build_hash_to_scalar_params(HashToScalarParamsOverrides {
                                tag: Some(&tag),
                            }),
                            build_hash_to_scalar_payload(HashToScalarPayloadOverrides {
                                message: Some(MESSAGE),
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
                        expected_uniform_bytes(&tag, MESSAGE, P::Scalar::UNIFORM_BYTES_LENGTH),
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

            let cases: Vec<CreateHashToScalarParams> = HASH_TO_SCALAR_CONCRETES
                .iter()
                .map(|member| {
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(build_hash_to_scalar_concrete(*member, Default::default())),
                    })
                })
                .collect();
            for case in cases {
                let success = create_hash_to_scalar::<P::Scalar, _>(
                    &build_create_hash_to_scalar_deps(
                        HashConsumer {
                            pairing: &consumed.adapter,
                        },
                        CreateHashToScalarDepsOverrides {
                            lookup: Some(lookup_hash_to_scalar_declaration),
                        },
                    ),
                    case,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the case's declared selection admits its concrete");
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

/// Contract: absorbed entry `hash_to_scalar_separates_domains`; given
///   `OTHER_TAG` and `REFERENCE_MESSAGE`, the encoding of the returned scalar
///   equals the encoding of the scalar sampled from
///   `expected_uniform_bytes(&tag, REFERENCE_MESSAGE,
///   S::UNIFORM_BYTES_LENGTH)` for `OTHER_TAG`.
/// Arrange: oracle block with `TAG` as `OTHER_TAG` and `MESSAGE` as
///   `REFERENCE_MESSAGE`, the message of the reference block under another
///   tag; every case of `cases` over every member of `PAIRING_CONCRETES`.
/// Act:     `create_pairing` per concrete.
/// Assert:  as
///   `every_concrete_declaring_the_keccak256_identifier_matches_the_oracle_for_the_reference_tag_and_message`.
/// Boundary: as
///   `every_concrete_declaring_the_keccak256_identifier_matches_the_oracle_for_the_reference_tag_and_message`.
/// Mocked:  none.
#[test]
fn every_concrete_declaring_the_keccak256_identifier_separates_domains() {
    // Arrange
    const TAG: &[u8] = OTHER_TAG;
    const MESSAGE: &[u8] = REFERENCE_MESSAGE;

    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingAdapter>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct HashConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
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
                    let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                        bytes: Some(TAG.to_vec()),
                    });
                    let hashed = payload
                        .adapter
                        .hash_to_scalar(
                            build_hash_to_scalar_params(HashToScalarParamsOverrides {
                                tag: Some(&tag),
                            }),
                            build_hash_to_scalar_payload(HashToScalarPayloadOverrides {
                                message: Some(MESSAGE),
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
                        expected_uniform_bytes(&tag, MESSAGE, P::Scalar::UNIFORM_BYTES_LENGTH),
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

            let cases: Vec<CreateHashToScalarParams> = HASH_TO_SCALAR_CONCRETES
                .iter()
                .map(|member| {
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(build_hash_to_scalar_concrete(*member, Default::default())),
                    })
                })
                .collect();
            for case in cases {
                let success = create_hash_to_scalar::<P::Scalar, _>(
                    &build_create_hash_to_scalar_deps(
                        HashConsumer {
                            pairing: &consumed.adapter,
                        },
                        CreateHashToScalarDepsOverrides {
                            lookup: Some(lookup_hash_to_scalar_declaration),
                        },
                    ),
                    case,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the case's declared selection admits its concrete");
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
