#![allow(clippy::expect_used)]

use super::create_hash_to_scalar;
use super::provides::{
    BINDING_LONGER_TAG, BINDING_SHORTER_MESSAGE, CreateHashToScalarCaseParamsOverrides,
    CreateHashToScalarCasePayload, CreateHashToScalarErrorReturn, CreateHashToScalarParams,
    CreateHashToScalarParamsOverrides, DECLARED_HASH_TO_SCALAR_SELECTIONS, EMPTY_MESSAGE,
    HashToScalarConcrete, HashToScalarErrorReturn, HashToScalarIdentifier,
    HashToScalarParamsOverrides, HashToScalarPayloadOverrides, ICreateHashToScalarCaseConsumer,
    IHashToScalarAdapter, MockIHashToScalarAdapterFailureMode, OTHER_TAG, REFERENCE_MESSAGE,
    REFERENCE_TAG, build_create_hash_to_scalar_case_deps, build_create_hash_to_scalar_case_params,
    build_create_hash_to_scalar_case_payload, build_create_hash_to_scalar_deps,
    build_create_hash_to_scalar_params, build_create_hash_to_scalar_payload,
    build_hash_to_scalar_params, build_hash_to_scalar_payload, expected_uniform_bytes,
    run_create_hash_to_scalar_case,
};
use crate::domain_tag::provides::{DomainTagConstructorParamsOverrides, build_domain_tag};
use domain::build_secret;
use pairing::{
    CreatePairingParamsOverrides, EncodeScalarParams, EncodeScalarPayload, IPairingAdapter,
    ISampleUniformScalar, PAIRING_CONCRETES, SampleUniformScalarErrorReturn,
    SampleUniformScalarParams, SampleUniformScalarPayload, build_create_pairing_params,
};

/// Contract: entry `every_concrete_is_constructed_under_the_identifier_it_declares`;
///   given the selection of a concrete and the identifier it declares,
///   `create_hash_to_scalar` yields `Ok(CreateHashToScalarSuccessReturn {
///   adapter })` with `adapter.declaration()` reporting the identifier required
///   and the interface version `HASH_TO_SCALAR_INTERFACE_VERSION`.
/// Arrange: `Consumer` that, per case of `cases`, calls
///   `create_hash_to_scalar::<P::Scalar>` with the case's params; the declared
///   set and the mock selection declare different identifiers, so a
///   construction of a concrete other than the selected one reports another
///   identifier.
/// Act:     `run_create_hash_to_scalar_case` per concrete, whose case
///   consumer's single call per case is `create_hash_to_scalar::<P::Scalar>`.
/// Assert:  in the consumer, per case, the result is `Ok`,
///   `adapter.declaration().identifier == case.identifier` by `assert!`, and
///   `adapter.declaration().interface_version` equals `1`; outside the
///   consumer, `created.is_ok()`.
/// Boundary: the real chain is `run_create_hash_to_scalar_case`,
///   `create_pairing`, the arm's `try_new`, `create_hash_to_scalar`, the
///   selected concrete's `try_new`, and `declaration` on the returned adapter.
/// Mocked:  none.
#[test]
fn every_concrete_is_constructed_under_the_identifier_it_declares() {
    struct Consumer;

    impl ICreateHashToScalarCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_hash_to_scalar_case<P: IPairingAdapter>(
            &self,
            _pairing: &P,
            _payload: &CreateHashToScalarCasePayload,
        ) -> Self::Output {
            let mut cases: Vec<CreateHashToScalarParams> =
                DECLARED_HASH_TO_SCALAR_SELECTIONS.to_vec();
            cases.push(build_create_hash_to_scalar_params(Default::default()));
            for case in cases {
                let result = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    case,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the case's declared identifier admits its concrete");
                let declaration = result.adapter.declaration();
                assert!(declaration.identifier == case.identifier);
                assert_eq!(declaration.interface_version, 1);
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params =
            build_create_hash_to_scalar_case_params(CreateHashToScalarCaseParamsOverrides {
                pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                    concrete: Some(*concrete),
                    ..Default::default()
                })),
            });
        let deps = build_create_hash_to_scalar_case_deps(Consumer, Default::default());

        // Act
        let created = run_create_hash_to_scalar_case(
            &deps,
            params,
            build_create_hash_to_scalar_case_payload(Default::default()),
        );

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: entry `every_concrete_is_refused_under_an_identifier_it_does_not_declare`;
///   given the selection of a concrete and an identifier that concrete does not
///   declare, `create_hash_to_scalar` yields
///   `Err(CreateHashToScalarErrorReturn::UnsupportedHashToScalarIdentifier)`.
/// Arrange: `Consumer` that, per `case` of `cases` and per `other` of `cases`
///   whose `identifier` differs from the `case`'s, calls
///   `create_hash_to_scalar::<P::Scalar>` with
///   `build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
///   concrete: Some(case.concrete), identifier: Some(other.identifier) })`, so
///   an admission that compares against a constant or ignores the selection
///   admits a pairing it must refuse.
/// Act:     `run_create_hash_to_scalar_case` per concrete, whose case
///   consumer's single call per pairing is `create_hash_to_scalar::<P::Scalar>`.
/// Assert:  in the consumer, per pairing, `result.err()` equals
///   `Some(CreateHashToScalarErrorReturn::UnsupportedHashToScalarIdentifier)`
///   by `assert_eq!`; outside the consumer, `created.is_ok()`.
/// Boundary: the real chain is `run_create_hash_to_scalar_case`,
///   `create_pairing`, the arm's `try_new`, and `create_hash_to_scalar`.
/// Mocked:  none.
#[test]
fn every_concrete_is_refused_under_an_identifier_it_does_not_declare() {
    struct Consumer;

    impl ICreateHashToScalarCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_hash_to_scalar_case<P: IPairingAdapter>(
            &self,
            _pairing: &P,
            _payload: &CreateHashToScalarCasePayload,
        ) -> Self::Output {
            let mut cases: Vec<CreateHashToScalarParams> =
                DECLARED_HASH_TO_SCALAR_SELECTIONS.to_vec();
            cases.push(build_create_hash_to_scalar_params(Default::default()));
            for case in &cases {
                for other in &cases {
                    if other.identifier == case.identifier {
                        continue;
                    }
                    let result = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                            concrete: Some(case.concrete),
                            identifier: Some(other.identifier),
                        }),
                        build_create_hash_to_scalar_payload(Default::default()),
                    );
                    assert_eq!(
                        result.err(),
                        Some(CreateHashToScalarErrorReturn::UnsupportedHashToScalarIdentifier)
                    );
                }
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params =
            build_create_hash_to_scalar_case_params(CreateHashToScalarCaseParamsOverrides {
                pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                    concrete: Some(*concrete),
                    ..Default::default()
                })),
            });
        let deps = build_create_hash_to_scalar_case_deps(Consumer, Default::default());

        // Act
        let created = run_create_hash_to_scalar_case(
            &deps,
            params,
            build_create_hash_to_scalar_case_payload(Default::default()),
        );

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: entry `every_concrete_returns_the_sampling_bound_refusal_whole`;
///   given the mock selection under
///   `MockIHashToScalarAdapterFailureMode::SamplingRefused`, `hash_to_scalar`
///   yields `Err(HashToScalarErrorReturn::Sampling(
///   SampleUniformScalarErrorReturn::WrongLength { expected:
///   P::Scalar::UNIFORM_BYTES_LENGTH, actual: 0 }))`.
/// Arrange: `Consumer` that calls `create_hash_to_scalar::<P::Scalar>` with
///   `build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
///   concrete: Some(HashToScalarConcrete::Mock(
///   MockIHashToScalarAdapterFailureMode::SamplingRefused)),
///   ..Default::default() })`, binds its `Ok` through `expect`, and builds the
///   tag with `build_domain_tag(Default::default())`; the failure mode is the
///   case that reaches the arm, since no input takes it for a real concrete,
///   whose sampler receives exactly its bound; every member of
///   `PAIRING_CONCRETES` supplies its own bound.
/// Act:     `run_create_hash_to_scalar_case` per concrete, whose case
///   consumer's single call is `adapter.hash_to_scalar(
///   build_hash_to_scalar_params(HashToScalarParamsOverrides { tag:
///   Some(&tag) }), build_hash_to_scalar_payload(Default::default()))`.
/// Assert:  in the consumer, per concrete, `result.err()` equals
///   `Some(HashToScalarErrorReturn::Sampling(
///   SampleUniformScalarErrorReturn::WrongLength { expected:
///   P::Scalar::UNIFORM_BYTES_LENGTH, actual: 0 }))` by `assert_eq!`, the
///   bound read from the received scalar type and the length from the empty
///   input the failure mode supplies; outside the consumer, `created.is_ok()`.
/// Boundary: the real chain is `run_create_hash_to_scalar_case`,
///   `create_pairing`, the arm's `try_new`, `create_hash_to_scalar`, the mock
///   concrete's `try_new` and `hash_to_scalar`, and the received scalar's
///   `sample_from_uniform_bytes`.
/// Mocked:  none.
#[test]
fn every_concrete_returns_the_sampling_bound_refusal_whole() {
    struct Consumer;

    impl ICreateHashToScalarCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_hash_to_scalar_case<P: IPairingAdapter>(
            &self,
            _pairing: &P,
            _payload: &CreateHashToScalarCasePayload,
        ) -> Self::Output {
            let success = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: Some(HashToScalarConcrete::Mock(
                        MockIHashToScalarAdapterFailureMode::SamplingRefused,
                    )),
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the mock selection admits under the identifier it declares");
            let tag = build_domain_tag(Default::default());
            let adapter: &dyn IHashToScalarAdapter<P::Scalar> = &*success.adapter;
            let result = adapter.hash_to_scalar(
                build_hash_to_scalar_params(HashToScalarParamsOverrides { tag: Some(&tag) }),
                build_hash_to_scalar_payload(Default::default()),
            );
            assert_eq!(
                result.err(),
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
        let params =
            build_create_hash_to_scalar_case_params(CreateHashToScalarCaseParamsOverrides {
                pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                    concrete: Some(*concrete),
                    ..Default::default()
                })),
            });
        let deps = build_create_hash_to_scalar_case_deps(Consumer, Default::default());

        // Act
        let created = run_create_hash_to_scalar_case(
            &deps,
            params,
            build_create_hash_to_scalar_case_payload(Default::default()),
        );

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: absorbed entry
///   `hash_to_scalar_matches_the_oracle_for_the_reference_tag_and_message`;
///   given `REFERENCE_TAG` and `REFERENCE_MESSAGE`, the encoding of the
///   returned scalar equals the encoding of the scalar sampled from
///   `expected_uniform_bytes(&tag, REFERENCE_MESSAGE, S::UNIFORM_BYTES_LENGTH)`.
/// Arrange: oracle block with `TAG` as `REFERENCE_TAG` and `MESSAGE` as
///   `REFERENCE_MESSAGE`; every case of `keccak_cases` over every member of
///   `PAIRING_CONCRETES`, each with its own scalar type.
/// Act:     `run_create_hash_to_scalar_case` per concrete.
/// Assert:  per concrete and case, the actual encoding equals the expected
///   encoding, which the oracle states independently of `hash_to_scalar`;
///   outside the consumer, `created.is_ok()`.
/// Boundary: the real chain is `run_create_hash_to_scalar_case`,
///   `create_pairing`, the arm's `try_new`, `create_hash_to_scalar`,
///   `DomainTag::try_new`, the selected concrete's `try_new` and
///   `hash_to_scalar`, the received scalar's `sample_from_uniform_bytes`, and
///   `encode_scalar`.
/// Mocked:  none.
#[test]
fn every_concrete_declaring_the_keccak256_identifier_matches_the_oracle_for_the_reference_tag_and_message()
 {
    // Arrange
    const TAG: &[u8] = REFERENCE_TAG;
    const MESSAGE: &[u8] = REFERENCE_MESSAGE;

    struct Consumer;

    impl ICreateHashToScalarCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_hash_to_scalar_case<P: IPairingAdapter>(
            &self,
            pairing: &P,
            _payload: &CreateHashToScalarCasePayload,
        ) -> Self::Output {
            let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                bytes: Some(TAG.to_vec()),
            });
            let mut cases: Vec<CreateHashToScalarParams> =
                DECLARED_HASH_TO_SCALAR_SELECTIONS.to_vec();
            cases.push(build_create_hash_to_scalar_params(Default::default()));
            let keccak_cases: Vec<CreateHashToScalarParams> = cases
                .into_iter()
                .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
                .collect();
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
                            message: Some(MESSAGE),
                        }),
                    )
                    .expect("hash_to_scalar admits the tag and message");
                let actual = pairing
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
                let expected = pairing
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
        let params =
            build_create_hash_to_scalar_case_params(CreateHashToScalarCaseParamsOverrides {
                pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                    concrete: Some(*concrete),
                    ..Default::default()
                })),
            });
        let deps = build_create_hash_to_scalar_case_deps(Consumer, Default::default());

        // Act
        let created = run_create_hash_to_scalar_case(
            &deps,
            params,
            build_create_hash_to_scalar_case_payload(Default::default()),
        );

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
///   `EMPTY_MESSAGE`, the message segment absent; every case of `keccak_cases`
///   over every member of `PAIRING_CONCRETES`.
/// Act:     `run_create_hash_to_scalar_case` per concrete.
/// Assert:  per concrete and case, the actual encoding equals the expected
///   encoding; outside the consumer, `created.is_ok()`.
/// Boundary: as
///   `every_concrete_declaring_the_keccak256_identifier_matches_the_oracle_for_the_reference_tag_and_message`.
/// Mocked:  none.
#[test]
fn every_concrete_declaring_the_keccak256_identifier_matches_the_oracle_for_the_empty_message() {
    // Arrange
    const TAG: &[u8] = REFERENCE_TAG;
    const MESSAGE: &[u8] = EMPTY_MESSAGE;

    struct Consumer;

    impl ICreateHashToScalarCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_hash_to_scalar_case<P: IPairingAdapter>(
            &self,
            pairing: &P,
            _payload: &CreateHashToScalarCasePayload,
        ) -> Self::Output {
            let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                bytes: Some(TAG.to_vec()),
            });
            let mut cases: Vec<CreateHashToScalarParams> =
                DECLARED_HASH_TO_SCALAR_SELECTIONS.to_vec();
            cases.push(build_create_hash_to_scalar_params(Default::default()));
            let keccak_cases: Vec<CreateHashToScalarParams> = cases
                .into_iter()
                .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
                .collect();
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
                            message: Some(MESSAGE),
                        }),
                    )
                    .expect("hash_to_scalar admits the tag and message");
                let actual = pairing
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
                let expected = pairing
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
        let params =
            build_create_hash_to_scalar_case_params(CreateHashToScalarCaseParamsOverrides {
                pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                    concrete: Some(*concrete),
                    ..Default::default()
                })),
            });
        let deps = build_create_hash_to_scalar_case_deps(Consumer, Default::default());

        // Act
        let created = run_create_hash_to_scalar_case(
            &deps,
            params,
            build_create_hash_to_scalar_case_payload(Default::default()),
        );

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
///   `BINDING_SHORTER_MESSAGE`; every case of `keccak_cases` over every member
///   of `PAIRING_CONCRETES`, so a prefix that is absent or is not the given
///   tag's length yields a scalar the oracle does not state.
/// Act:     `run_create_hash_to_scalar_case` per concrete.
/// Assert:  per concrete and case, the actual encoding equals the expected
///   encoding; outside the consumer, `created.is_ok()`.
/// Boundary: as
///   `every_concrete_declaring_the_keccak256_identifier_matches_the_oracle_for_the_reference_tag_and_message`.
/// Mocked:  none.
#[test]
fn every_concrete_declaring_the_keccak256_identifier_binds_the_tag_length() {
    // Arrange
    const TAG: &[u8] = BINDING_LONGER_TAG;
    const MESSAGE: &[u8] = BINDING_SHORTER_MESSAGE;

    struct Consumer;

    impl ICreateHashToScalarCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_hash_to_scalar_case<P: IPairingAdapter>(
            &self,
            pairing: &P,
            _payload: &CreateHashToScalarCasePayload,
        ) -> Self::Output {
            let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                bytes: Some(TAG.to_vec()),
            });
            let mut cases: Vec<CreateHashToScalarParams> =
                DECLARED_HASH_TO_SCALAR_SELECTIONS.to_vec();
            cases.push(build_create_hash_to_scalar_params(Default::default()));
            let keccak_cases: Vec<CreateHashToScalarParams> = cases
                .into_iter()
                .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
                .collect();
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
                            message: Some(MESSAGE),
                        }),
                    )
                    .expect("hash_to_scalar admits the tag and message");
                let actual = pairing
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
                let expected = pairing
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
        let params =
            build_create_hash_to_scalar_case_params(CreateHashToScalarCaseParamsOverrides {
                pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                    concrete: Some(*concrete),
                    ..Default::default()
                })),
            });
        let deps = build_create_hash_to_scalar_case_deps(Consumer, Default::default());

        // Act
        let created = run_create_hash_to_scalar_case(
            &deps,
            params,
            build_create_hash_to_scalar_case_payload(Default::default()),
        );

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: absorbed entry `hash_to_scalar_separates_domains`; given
///   `OTHER_TAG` and `REFERENCE_MESSAGE`, the encoding of the returned scalar
///   equals the encoding of the scalar sampled from
///   `expected_uniform_bytes(&tag, REFERENCE_MESSAGE, S::UNIFORM_BYTES_LENGTH)`
///   for `OTHER_TAG`.
/// Arrange: oracle block with `TAG` as `OTHER_TAG` and `MESSAGE` as
///   `REFERENCE_MESSAGE`, the message of the reference block under another
///   tag; every case of `keccak_cases` over every member of
///   `PAIRING_CONCRETES`.
/// Act:     `run_create_hash_to_scalar_case` per concrete.
/// Assert:  per concrete and case, the actual encoding equals the expected
///   encoding; outside the consumer, `created.is_ok()`.
/// Boundary: as
///   `every_concrete_declaring_the_keccak256_identifier_matches_the_oracle_for_the_reference_tag_and_message`.
/// Mocked:  none.
#[test]
fn every_concrete_declaring_the_keccak256_identifier_separates_domains() {
    // Arrange
    const TAG: &[u8] = OTHER_TAG;
    const MESSAGE: &[u8] = REFERENCE_MESSAGE;

    struct Consumer;

    impl ICreateHashToScalarCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_hash_to_scalar_case<P: IPairingAdapter>(
            &self,
            pairing: &P,
            _payload: &CreateHashToScalarCasePayload,
        ) -> Self::Output {
            let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                bytes: Some(TAG.to_vec()),
            });
            let mut cases: Vec<CreateHashToScalarParams> =
                DECLARED_HASH_TO_SCALAR_SELECTIONS.to_vec();
            cases.push(build_create_hash_to_scalar_params(Default::default()));
            let keccak_cases: Vec<CreateHashToScalarParams> = cases
                .into_iter()
                .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
                .collect();
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
                            message: Some(MESSAGE),
                        }),
                    )
                    .expect("hash_to_scalar admits the tag and message");
                let actual = pairing
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
                let expected = pairing
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
        let params =
            build_create_hash_to_scalar_case_params(CreateHashToScalarCaseParamsOverrides {
                pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                    concrete: Some(*concrete),
                    ..Default::default()
                })),
            });
        let deps = build_create_hash_to_scalar_case_deps(Consumer, Default::default());

        // Act
        let created = run_create_hash_to_scalar_case(
            &deps,
            params,
            build_create_hash_to_scalar_case_payload(Default::default()),
        );

        // Assert
        assert!(created.is_ok());
    }
}
