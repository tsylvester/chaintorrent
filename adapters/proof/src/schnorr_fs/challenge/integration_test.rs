#![allow(clippy::expect_used)]

use super::challenge;
use super::provides::{
    ChallengeCaseParamsOverrides, ChallengeCasePayload, ChallengeTranscript,
    IChallengeCaseConsumer, SCHNORR_FS_CHALLENGE_TAG, build_challenge_case_deps,
    build_challenge_case_params, build_challenge_case_payload, build_challenge_deps,
    build_challenge_params, build_challenge_payload, run_challenge_case,
};
use crate::mint_statement::provides::{
    MintFirstMessagesOverrides, MintPurpose, MintStatementOverrides, TransferPurpose,
    build_mint_first_messages, build_mint_statement, build_mint_statement_description,
};
use crate::transfer_statement::provides::{
    TransferFirstMessagesOverrides, TransferStatementOverrides, build_transfer_first_messages,
    build_transfer_statement, build_transfer_statement_description,
};
use chain::{
    CHAIN_FORMS_CONCRETES, CreateChainFormsParamsOverrides, IChainForms,
    build_create_chain_forms_params,
};
use domain::{
    PARAMETER_SET_IDENTIFIER_LENGTH, ParameterSetIdentifierConstructorParamsOverrides,
    SecretConstructorParamsOverrides, SuiteIdentifierConstructorParamsOverrides,
    build_parameter_set_identifier, build_secret, build_suite_identifier,
};
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, CreateEncodingParamsOverrides,
    DECLARED_ENCODING_SELECTIONS, EncodeParams, FromFieldParamsOverrides, ICanonicalField,
    IDecoderAdapter, IEncoderAdapter, build_create_encoding_params, build_from_field_params,
};
use envelope::{
    EnvelopeComponentsOverrides, PublicKeysComponentsOverrides, build_envelope_components,
    build_public_keys_components,
};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DECLARED_HASH_TO_SCALAR_SELECTIONS, DomainTag,
    DomainTagConstructorParamsOverrides, HashToScalarIdentifier, IHashToScalarAdapter,
    build_create_hash_to_scalar_params, build_domain_tag, build_domain_tag_constructor_params,
    expected_uniform_bytes,
};
use hashing::{RootConstructorParamsOverrides, build_root};
use pairing::{
    CreatePairingParamsOverrides, EncodeScalarParams, EncodeScalarPayload, IPairingArithmetic,
    IPairingReference, ISampleUniformScalar, PAIRING_CONCRETES, SampleUniformScalarParams,
    SampleUniformScalarPayload, build_create_pairing_params,
};

/// Contract: entry `the_mint_challenge_is_the_tagged_hash_of_the_mint_transcripts_encoding`;
///   `challenge` over `ChallengeTranscript::Mint(statement)` returns
///   `Ok(ChallengeSuccessReturn { challenge })` whose encoding equals the
///   encoding of the oracle scalar of the encoder's bytes for the statement
///   under the proof's tag.
/// Arrange: every member of `DECLARED_ENCODING_SELECTIONS`, each over every
///   member of `PAIRING_CONCRETES`, of `CHAIN_FORMS_CONCRETES`, and of
///   `keccak_cases` — the members of `DECLARED_HASH_TO_SCALAR_SELECTIONS`
///   whose `identifier` is `HashToScalarIdentifier::Keccak256V1`; for each
///   of `MintPurpose::Mint` and `MintPurpose::Replacement` the statement
///   `build_mint_statement(pairing, MintStatementOverrides { purpose:
///   Some(purpose), ..Default::default() })`, its values distinct at every
///   position so that a swap of same-kind fields fails; the bytes from the
///   received encoder's `encode` under `EncodeParams` over
///   `build_mint_statement_description(pairing, Default::default())`; the
///   oracle scalar sampled from `expected_uniform_bytes(&tag, &bytes,
///   P::Scalar::UNIFORM_BYTES_LENGTH)`.
/// Act:     `run_challenge_case` per case, the consumer's method calling
///   `challenge(&deps, params, payload)` over
///   `ChallengeTranscript::Mint(&statement)`.
/// Assert:  in place, the result is `Ok`, and the encoding of its
///   `challenge` through the received pairing's `encode_scalar` equals the
///   encoding of the oracle scalar by `assert_eq!`; the `Ok` arm of each
///   runner call proves every factory ran.
/// Boundary: the private call `challenge` and the route `run_challenge_case`,
///   `create_pairing`, `consume_pairing`, `create_chain_forms`,
///   `consume_chain_forms`, `create_encoding`, `consume_encoding`,
///   `create_hash_to_scalar`, `DomainTag::try_new`, the description's
///   `try_new` and `to_fields`, the encoder's `encode`, and the adapter's
///   `hash_to_scalar`.
/// Mocked:  nothing; the block proves the composition of every function on
///   its route against the oracle.
#[test]
fn the_mint_challenge_is_the_tagged_hash_of_the_mint_transcripts_encoding() {
    struct Consumer;

    impl IChallengeCaseConsumer for Consumer {
        type Output = ();

        fn consume_challenge_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &ChallengeCasePayload,
        ) -> Self::Output {
            // Arrange
            let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                bytes: Some(SCHNORR_FS_CHALLENGE_TAG.to_vec()),
            });
            let description = build_mint_statement_description::<P, F>(pairing, Default::default());
            let deps =
                build_challenge_deps(pairing, encoder, &tag, hash_to_scalar, Default::default());
            for purpose in [MintPurpose::Mint, MintPurpose::Replacement] {
                let statement = build_mint_statement::<P, F>(
                    pairing,
                    MintStatementOverrides {
                        purpose: Some(purpose),
                        ..Default::default()
                    },
                );
                let bytes = encoder
                    .encode(
                        EncodeParams {
                            description: &description,
                        },
                        &statement,
                    )
                    .expect("a built statement encodes")
                    .bytes;
                let oracle = P::Scalar::sample_from_uniform_bytes(
                    SampleUniformScalarParams,
                    SampleUniformScalarPayload {
                        uniform: build_secret(
                            expected_uniform_bytes(
                                &tag,
                                &bytes,
                                <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                            ),
                            SecretConstructorParamsOverrides,
                        ),
                    },
                )
                .expect("a full-length uniform samples")
                .scalar
                .expose()
                .clone();

                // Act
                let result = challenge(
                    &deps,
                    build_challenge_params(Default::default()),
                    build_challenge_payload(
                        ChallengeTranscript::Mint(&statement),
                        Default::default(),
                    ),
                );

                // Assert
                assert_eq!(
                    pairing
                        .encode_scalar(
                            EncodeScalarParams,
                            EncodeScalarPayload {
                                scalar: result
                                    .expect("the mint transcript admits a challenge")
                                    .challenge,
                            },
                        )
                        .expect("encoding the challenge scalar succeeds")
                        .bytes
                        .expose()
                        .as_ref(),
                    pairing
                        .encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar: oracle },)
                        .expect("encoding the oracle scalar succeeds")
                        .bytes
                        .expose()
                        .as_ref(),
                );
            }
        }
    }

    for pairing_concrete in PAIRING_CONCRETES {
        for chain_forms_concrete in CHAIN_FORMS_CONCRETES {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for hash_to_scalar_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS
                    .iter()
                    .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
                {
                    run_challenge_case(
                        &build_challenge_case_deps(Consumer, Default::default()),
                        build_challenge_case_params(ChallengeCaseParamsOverrides {
                            pairing: Some(build_create_pairing_params(
                                CreatePairingParamsOverrides {
                                    concrete: Some(*pairing_concrete),
                                    ..Default::default()
                                },
                            )),
                            chain_forms: Some(build_create_chain_forms_params(
                                CreateChainFormsParamsOverrides {
                                    concrete: Some(*chain_forms_concrete),
                                    ..Default::default()
                                },
                            )),
                            encoding: Some(build_create_encoding_params(
                                CreateEncodingParamsOverrides {
                                    concrete: Some(encoding_selection.concrete),
                                    identifier: Some(encoding_selection.identifier),
                                },
                            )),
                            hash_to_scalar: Some(build_create_hash_to_scalar_params(
                                CreateHashToScalarParamsOverrides {
                                    concrete: Some(hash_to_scalar_selection.concrete),
                                    identifier: Some(hash_to_scalar_selection.identifier),
                                },
                            )),
                        }),
                        build_challenge_case_payload(Default::default()),
                    )
                    .expect("the case runner ran the consumer");
                }
            }
        }
    }
}

/// Contract: entry `the_transfer_challenge_is_the_tagged_hash_of_the_transfer_transcripts_encoding`;
///   `challenge` over `ChallengeTranscript::Transfer(statement)` returns
///   `Ok(ChallengeSuccessReturn { challenge })` whose encoding equals the
///   encoding of the oracle scalar of the encoder's bytes for the statement
///   under the proof's tag.
/// Arrange: the declared sets of the mint block, and for each of
///   `TransferPurpose::Transfer` and `TransferPurpose::Grant` the statement
///   `build_transfer_statement(pairing, TransferStatementOverrides { purpose:
///   Some(purpose), ..Default::default() })`, its values distinct at every
///   position; the bytes from the received encoder's `encode` over
///   `build_transfer_statement_description(pairing, Default::default())`;
///   the oracle scalar sampled from `expected_uniform_bytes(&tag, &bytes,
///   P::Scalar::UNIFORM_BYTES_LENGTH)`.
/// Act:     `run_challenge_case` per case, the consumer's method calling
///   `challenge(&deps, params, payload)` over
///   `ChallengeTranscript::Transfer(&statement)`.
/// Assert:  in place, the result is `Ok`, and the encoding of its
///   `challenge` through the received pairing's `encode_scalar` equals the
///   encoding of the oracle scalar by `assert_eq!`; the `Ok` arm of each
///   runner call proves every factory ran.
/// Boundary: the private call `challenge` and the route of the mint block
///   with `TransferStatementDescription` in place of
///   `MintStatementDescription`.
/// Mocked:  nothing.
#[test]
fn the_transfer_challenge_is_the_tagged_hash_of_the_transfer_transcripts_encoding() {
    struct Consumer;

    impl IChallengeCaseConsumer for Consumer {
        type Output = ();

        fn consume_challenge_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &ChallengeCasePayload,
        ) -> Self::Output {
            // Arrange
            let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                bytes: Some(SCHNORR_FS_CHALLENGE_TAG.to_vec()),
            });
            let description =
                build_transfer_statement_description::<P, F>(pairing, Default::default());
            let deps =
                build_challenge_deps(pairing, encoder, &tag, hash_to_scalar, Default::default());
            for purpose in [TransferPurpose::Transfer, TransferPurpose::Grant] {
                let statement = build_transfer_statement::<P, F>(
                    pairing,
                    TransferStatementOverrides {
                        purpose: Some(purpose),
                        ..Default::default()
                    },
                );
                let bytes = encoder
                    .encode(
                        EncodeParams {
                            description: &description,
                        },
                        &statement,
                    )
                    .expect("a built statement encodes")
                    .bytes;
                let oracle = P::Scalar::sample_from_uniform_bytes(
                    SampleUniformScalarParams,
                    SampleUniformScalarPayload {
                        uniform: build_secret(
                            expected_uniform_bytes(
                                &tag,
                                &bytes,
                                <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                            ),
                            SecretConstructorParamsOverrides,
                        ),
                    },
                )
                .expect("a full-length uniform samples")
                .scalar
                .expose()
                .clone();

                // Act
                let result = challenge(
                    &deps,
                    build_challenge_params(Default::default()),
                    build_challenge_payload(
                        ChallengeTranscript::Transfer(&statement),
                        Default::default(),
                    ),
                );

                // Assert
                assert_eq!(
                    pairing
                        .encode_scalar(
                            EncodeScalarParams,
                            EncodeScalarPayload {
                                scalar: result
                                    .expect("the transfer transcript admits a challenge")
                                    .challenge,
                            },
                        )
                        .expect("encoding the challenge scalar succeeds")
                        .bytes
                        .expose()
                        .as_ref(),
                    pairing
                        .encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar: oracle },)
                        .expect("encoding the oracle scalar succeeds")
                        .bytes
                        .expose()
                        .as_ref(),
                );
            }
        }
    }

    for pairing_concrete in PAIRING_CONCRETES {
        for chain_forms_concrete in CHAIN_FORMS_CONCRETES {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for hash_to_scalar_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS
                    .iter()
                    .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
                {
                    run_challenge_case(
                        &build_challenge_case_deps(Consumer, Default::default()),
                        build_challenge_case_params(ChallengeCaseParamsOverrides {
                            pairing: Some(build_create_pairing_params(
                                CreatePairingParamsOverrides {
                                    concrete: Some(*pairing_concrete),
                                    ..Default::default()
                                },
                            )),
                            chain_forms: Some(build_create_chain_forms_params(
                                CreateChainFormsParamsOverrides {
                                    concrete: Some(*chain_forms_concrete),
                                    ..Default::default()
                                },
                            )),
                            encoding: Some(build_create_encoding_params(
                                CreateEncodingParamsOverrides {
                                    concrete: Some(encoding_selection.concrete),
                                    identifier: Some(encoding_selection.identifier),
                                },
                            )),
                            hash_to_scalar: Some(build_create_hash_to_scalar_params(
                                CreateHashToScalarParamsOverrides {
                                    concrete: Some(hash_to_scalar_selection.concrete),
                                    identifier: Some(hash_to_scalar_selection.identifier),
                                },
                            )),
                        }),
                        build_challenge_case_payload(Default::default()),
                    )
                    .expect("the case runner ran the consumer");
                }
            }
        }
    }
}

/// Contract: entry `a_transcript_differing_in_one_field_yields_another_challenge`;
///   `challenge` over a statement that differs from the baseline in one
///   field returns the oracle scalar of the variant's encoding, which is
///   not the oracle scalar of the baseline's encoding.
/// Arrange: the declared sets of the mint block; for each field of
///   `MintStatement` and of `TransferStatement`, the variant
///   `build_mint_statement(pairing, MintStatementOverrides { <field>:
///   Some(<value from that field's own builder distinct from the default>),
///   ..Default::default() })` or its transfer counterpart; the baseline
///   `build_mint_statement(pairing, Default::default())` or its transfer
///   counterpart; the oracle scalars sampled from `expected_uniform_bytes`
///   over the encoder's bytes for the variant and for the baseline.
/// Act:     `run_challenge_case` per case, the consumer's method calling
///   `challenge(&deps, params, payload)` over the variant.
/// Assert:  in place, the encoding of the result's `challenge` equals the
///   encoding of the variant's oracle scalar and differs from the encoding
///   of the baseline's oracle scalar, the equality by `assert_eq!` and the
///   difference by `assert_ne!`; the `Ok` arm of each runner call proves
///   every factory ran.
/// Boundary: the private call `challenge` and the route of the mint block,
///   run for each transcript variant.
/// Mocked:  nothing.
#[test]
fn a_transcript_differing_in_one_field_yields_another_challenge() {
    struct Consumer;

    impl IChallengeCaseConsumer for Consumer {
        type Output = ();

        fn consume_challenge_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &ChallengeCasePayload,
        ) -> Self::Output {
            fn variant_form<T: ICanonicalField>(seed: u8) -> T {
                T::from_field(
                    build_from_field_params(FromFieldParamsOverrides),
                    match T::KIND {
                        CanonicalFieldKind::FixedBytes20 => {
                            CanonicalFieldValue::FixedBytes20([seed; 20])
                        }
                        CanonicalFieldKind::FixedBytes32 => {
                            CanonicalFieldValue::FixedBytes32([seed; 32])
                        }
                        CanonicalFieldKind::Unsigned16 => {
                            CanonicalFieldValue::Unsigned16(u16::from(seed))
                        }
                        CanonicalFieldKind::Unsigned32 => {
                            CanonicalFieldValue::Unsigned32(u32::from(seed))
                        }
                        CanonicalFieldKind::Unsigned64 => {
                            CanonicalFieldValue::Unsigned64(u64::from(seed))
                        }
                        CanonicalFieldKind::Unsigned256 => {
                            CanonicalFieldValue::Unsigned256([seed; 32])
                        }
                        CanonicalFieldKind::Bytes => CanonicalFieldValue::Bytes(vec![seed]),
                        CanonicalFieldKind::Text => CanonicalFieldValue::Text(seed.to_string()),
                    },
                )
                .expect("a seeded field of the declared kind is admitted")
                .value
            }

            fn oracle_scalar<S: ISampleUniformScalar + Clone>(tag: &DomainTag, bytes: &[u8]) -> S {
                S::sample_from_uniform_bytes(
                    SampleUniformScalarParams,
                    SampleUniformScalarPayload {
                        uniform: build_secret(
                            expected_uniform_bytes(tag, bytes, S::UNIFORM_BYTES_LENGTH),
                            SecretConstructorParamsOverrides,
                        ),
                    },
                )
                .expect("a full-length uniform samples")
                .scalar
                .expose()
                .clone()
            }

            // Arrange
            let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                bytes: Some(SCHNORR_FS_CHALLENGE_TAG.to_vec()),
            });
            let deps =
                build_challenge_deps(pairing, encoder, &tag, hash_to_scalar, Default::default());
            let mint_description =
                build_mint_statement_description::<P, F>(pairing, Default::default());
            let mint_baseline = build_mint_statement::<P, F>(pairing, Default::default());
            let mint_baseline_bytes = encoder
                .encode(
                    EncodeParams {
                        description: &mint_description,
                    },
                    &mint_baseline,
                )
                .expect("a built statement encodes")
                .bytes;
            let mint_baseline_encoding = pairing
                .encode_scalar(
                    EncodeScalarParams,
                    EncodeScalarPayload {
                        scalar: oracle_scalar::<P::Scalar>(&tag, &mint_baseline_bytes),
                    },
                )
                .expect("encoding the oracle scalar succeeds")
                .bytes
                .expose()
                .as_ref()
                .to_vec();
            let mint_variants = [
                build_mint_statement::<P, F>(
                    pairing,
                    MintStatementOverrides {
                        suite_identifier: Some(build_suite_identifier(
                            SuiteIdentifierConstructorParamsOverrides {
                                version: Some(2),
                                ..Default::default()
                            },
                        )),
                        ..Default::default()
                    },
                ),
                build_mint_statement::<P, F>(
                    pairing,
                    MintStatementOverrides {
                        chain: Some(variant_form::<F::ChainIdentifier>(0xA1)),
                        ..Default::default()
                    },
                ),
                build_mint_statement::<P, F>(
                    pairing,
                    MintStatementOverrides {
                        entitlement_contract: Some(variant_form::<F::Identity>(0xA2)),
                        ..Default::default()
                    },
                ),
                build_mint_statement::<P, F>(
                    pairing,
                    MintStatementOverrides {
                        asset_root: Some(build_root(RootConstructorParamsOverrides {
                            bytes: Some([0xA4; 32]),
                            ..Default::default()
                        })),
                        ..Default::default()
                    },
                ),
                build_mint_statement::<P, F>(
                    pairing,
                    MintStatementOverrides {
                        parameter_set_identifier: Some(build_parameter_set_identifier(
                            ParameterSetIdentifierConstructorParamsOverrides {
                                bytes: Some([0xA5; PARAMETER_SET_IDENTIFIER_LENGTH]),
                                ..Default::default()
                            },
                        )),
                        ..Default::default()
                    },
                ),
                build_mint_statement::<P, F>(
                    pairing,
                    MintStatementOverrides {
                        source_entitlement: Some(variant_form::<F::Entitlement>(0xA6)),
                        ..Default::default()
                    },
                ),
                build_mint_statement::<P, F>(
                    pairing,
                    MintStatementOverrides {
                        target_entitlement: Some(variant_form::<F::Entitlement>(0xA7)),
                        ..Default::default()
                    },
                ),
                build_mint_statement::<P, F>(
                    pairing,
                    MintStatementOverrides {
                        old_interval: Some(variant_form::<F::Interval>(0xA8)),
                        ..Default::default()
                    },
                ),
                build_mint_statement::<P, F>(
                    pairing,
                    MintStatementOverrides {
                        new_interval: Some(variant_form::<F::Interval>(0xA9)),
                        ..Default::default()
                    },
                ),
                build_mint_statement::<P, F>(
                    pairing,
                    MintStatementOverrides {
                        purpose: Some(MintPurpose::Replacement),
                        ..Default::default()
                    },
                ),
                build_mint_statement::<P, F>(
                    pairing,
                    MintStatementOverrides {
                        seller: Some(variant_form::<F::Identity>(0xAA)),
                        ..Default::default()
                    },
                ),
                build_mint_statement::<P, F>(
                    pairing,
                    MintStatementOverrides {
                        buyer: Some(variant_form::<F::Identity>(0xAB)),
                        ..Default::default()
                    },
                ),
                build_mint_statement::<P, F>(
                    pairing,
                    MintStatementOverrides {
                        buyer_keys: Some(build_public_keys_components(
                            pairing,
                            PublicKeysComponentsOverrides::default(),
                        )),
                        ..Default::default()
                    },
                ),
                build_mint_statement::<P, F>(
                    pairing,
                    MintStatementOverrides {
                        envelope: Some(build_envelope_components(
                            pairing,
                            EnvelopeComponentsOverrides::default(),
                        )),
                        ..Default::default()
                    },
                ),
                build_mint_statement::<P, F>(
                    pairing,
                    MintStatementOverrides {
                        expiry: Some(1_700_000_001),
                        ..Default::default()
                    },
                ),
                build_mint_statement::<P, F>(
                    pairing,
                    MintStatementOverrides {
                        first_messages: Some(build_mint_first_messages(
                            pairing,
                            MintFirstMessagesOverrides {
                                hpub: Some(
                                    build_public_keys_components(
                                        pairing,
                                        PublicKeysComponentsOverrides::default(),
                                    )
                                    .pk2,
                                ),
                                ..Default::default()
                            },
                        )),
                        ..Default::default()
                    },
                ),
            ];
            for variant in &mint_variants {
                let variant_bytes = encoder
                    .encode(
                        EncodeParams {
                            description: &mint_description,
                        },
                        variant,
                    )
                    .expect("a variant statement encodes")
                    .bytes;
                let variant_encoding = pairing
                    .encode_scalar(
                        EncodeScalarParams,
                        EncodeScalarPayload {
                            scalar: oracle_scalar::<P::Scalar>(&tag, &variant_bytes),
                        },
                    )
                    .expect("encoding the oracle scalar succeeds")
                    .bytes
                    .expose()
                    .as_ref()
                    .to_vec();

                // Act
                let result = challenge(
                    &deps,
                    build_challenge_params(Default::default()),
                    build_challenge_payload(ChallengeTranscript::Mint(variant), Default::default()),
                );

                // Assert
                let challenge_encoding = pairing
                    .encode_scalar(
                        EncodeScalarParams,
                        EncodeScalarPayload {
                            scalar: result
                                .expect("the variant transcript admits a challenge")
                                .challenge,
                        },
                    )
                    .expect("encoding the challenge scalar succeeds")
                    .bytes
                    .expose()
                    .as_ref()
                    .to_vec();
                assert_eq!(challenge_encoding, variant_encoding);
                assert_ne!(challenge_encoding, mint_baseline_encoding);
            }

            // Arrange
            let transfer_description =
                build_transfer_statement_description::<P, F>(pairing, Default::default());
            let transfer_baseline = build_transfer_statement::<P, F>(pairing, Default::default());
            let transfer_baseline_bytes = encoder
                .encode(
                    EncodeParams {
                        description: &transfer_description,
                    },
                    &transfer_baseline,
                )
                .expect("a built statement encodes")
                .bytes;
            let transfer_baseline_encoding = pairing
                .encode_scalar(
                    EncodeScalarParams,
                    EncodeScalarPayload {
                        scalar: oracle_scalar::<P::Scalar>(&tag, &transfer_baseline_bytes),
                    },
                )
                .expect("encoding the oracle scalar succeeds")
                .bytes
                .expose()
                .as_ref()
                .to_vec();
            let transfer_variants = [
                build_transfer_statement::<P, F>(
                    pairing,
                    TransferStatementOverrides {
                        suite_identifier: Some(build_suite_identifier(
                            SuiteIdentifierConstructorParamsOverrides {
                                version: Some(2),
                                ..Default::default()
                            },
                        )),
                        ..Default::default()
                    },
                ),
                build_transfer_statement::<P, F>(
                    pairing,
                    TransferStatementOverrides {
                        chain: Some(variant_form::<F::ChainIdentifier>(0xB1)),
                        ..Default::default()
                    },
                ),
                build_transfer_statement::<P, F>(
                    pairing,
                    TransferStatementOverrides {
                        entitlement_contract: Some(variant_form::<F::Identity>(0xB2)),
                        ..Default::default()
                    },
                ),
                build_transfer_statement::<P, F>(
                    pairing,
                    TransferStatementOverrides {
                        asset_root: Some(build_root(RootConstructorParamsOverrides {
                            bytes: Some([0xB4; 32]),
                            ..Default::default()
                        })),
                        ..Default::default()
                    },
                ),
                build_transfer_statement::<P, F>(
                    pairing,
                    TransferStatementOverrides {
                        parameter_set_identifier: Some(build_parameter_set_identifier(
                            ParameterSetIdentifierConstructorParamsOverrides {
                                bytes: Some([0xB5; PARAMETER_SET_IDENTIFIER_LENGTH]),
                                ..Default::default()
                            },
                        )),
                        ..Default::default()
                    },
                ),
                build_transfer_statement::<P, F>(
                    pairing,
                    TransferStatementOverrides {
                        source_entitlement: Some(variant_form::<F::Entitlement>(0xB6)),
                        ..Default::default()
                    },
                ),
                build_transfer_statement::<P, F>(
                    pairing,
                    TransferStatementOverrides {
                        target_entitlement: Some(variant_form::<F::Entitlement>(0xB7)),
                        ..Default::default()
                    },
                ),
                build_transfer_statement::<P, F>(
                    pairing,
                    TransferStatementOverrides {
                        old_interval: Some(variant_form::<F::Interval>(0xB8)),
                        ..Default::default()
                    },
                ),
                build_transfer_statement::<P, F>(
                    pairing,
                    TransferStatementOverrides {
                        new_interval: Some(variant_form::<F::Interval>(0xB9)),
                        ..Default::default()
                    },
                ),
                build_transfer_statement::<P, F>(
                    pairing,
                    TransferStatementOverrides {
                        purpose: Some(TransferPurpose::Grant),
                        ..Default::default()
                    },
                ),
                build_transfer_statement::<P, F>(
                    pairing,
                    TransferStatementOverrides {
                        seller: Some(variant_form::<F::Identity>(0xBA)),
                        ..Default::default()
                    },
                ),
                build_transfer_statement::<P, F>(
                    pairing,
                    TransferStatementOverrides {
                        buyer: Some(variant_form::<F::Identity>(0xBB)),
                        ..Default::default()
                    },
                ),
                build_transfer_statement::<P, F>(
                    pairing,
                    TransferStatementOverrides {
                        seller_keys: Some(build_public_keys_components(
                            pairing,
                            PublicKeysComponentsOverrides::default(),
                        )),
                        ..Default::default()
                    },
                ),
                build_transfer_statement::<P, F>(
                    pairing,
                    TransferStatementOverrides {
                        buyer_keys: Some(build_public_keys_components(
                            pairing,
                            PublicKeysComponentsOverrides::default(),
                        )),
                        ..Default::default()
                    },
                ),
                build_transfer_statement::<P, F>(
                    pairing,
                    TransferStatementOverrides {
                        old_envelope: Some(build_envelope_components(
                            pairing,
                            EnvelopeComponentsOverrides::default(),
                        )),
                        ..Default::default()
                    },
                ),
                build_transfer_statement::<P, F>(
                    pairing,
                    TransferStatementOverrides {
                        new_envelope: Some(build_envelope_components(
                            pairing,
                            EnvelopeComponentsOverrides::default(),
                        )),
                        ..Default::default()
                    },
                ),
                build_transfer_statement::<P, F>(
                    pairing,
                    TransferStatementOverrides {
                        expiry: Some(1_700_000_001),
                        ..Default::default()
                    },
                ),
                build_transfer_statement::<P, F>(
                    pairing,
                    TransferStatementOverrides {
                        first_messages: Some(build_transfer_first_messages(
                            pairing,
                            TransferFirstMessagesOverrides {
                                pk1: Some(
                                    build_public_keys_components(
                                        pairing,
                                        PublicKeysComponentsOverrides::default(),
                                    )
                                    .pk1,
                                ),
                                ..Default::default()
                            },
                        )),
                        ..Default::default()
                    },
                ),
            ];
            for variant in &transfer_variants {
                let variant_bytes = encoder
                    .encode(
                        EncodeParams {
                            description: &transfer_description,
                        },
                        variant,
                    )
                    .expect("a variant statement encodes")
                    .bytes;
                let variant_encoding = pairing
                    .encode_scalar(
                        EncodeScalarParams,
                        EncodeScalarPayload {
                            scalar: oracle_scalar::<P::Scalar>(&tag, &variant_bytes),
                        },
                    )
                    .expect("encoding the oracle scalar succeeds")
                    .bytes
                    .expose()
                    .as_ref()
                    .to_vec();

                // Act
                let result = challenge(
                    &deps,
                    build_challenge_params(Default::default()),
                    build_challenge_payload(
                        ChallengeTranscript::Transfer(variant),
                        Default::default(),
                    ),
                );

                // Assert
                let challenge_encoding = pairing
                    .encode_scalar(
                        EncodeScalarParams,
                        EncodeScalarPayload {
                            scalar: result
                                .expect("the variant transcript admits a challenge")
                                .challenge,
                        },
                    )
                    .expect("encoding the challenge scalar succeeds")
                    .bytes
                    .expose()
                    .as_ref()
                    .to_vec();
                assert_eq!(challenge_encoding, variant_encoding);
                assert_ne!(challenge_encoding, transfer_baseline_encoding);
            }
        }
    }

    for pairing_concrete in PAIRING_CONCRETES {
        for chain_forms_concrete in CHAIN_FORMS_CONCRETES {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for hash_to_scalar_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS
                    .iter()
                    .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
                {
                    run_challenge_case(
                        &build_challenge_case_deps(Consumer, Default::default()),
                        build_challenge_case_params(ChallengeCaseParamsOverrides {
                            pairing: Some(build_create_pairing_params(
                                CreatePairingParamsOverrides {
                                    concrete: Some(*pairing_concrete),
                                    ..Default::default()
                                },
                            )),
                            chain_forms: Some(build_create_chain_forms_params(
                                CreateChainFormsParamsOverrides {
                                    concrete: Some(*chain_forms_concrete),
                                    ..Default::default()
                                },
                            )),
                            encoding: Some(build_create_encoding_params(
                                CreateEncodingParamsOverrides {
                                    concrete: Some(encoding_selection.concrete),
                                    identifier: Some(encoding_selection.identifier),
                                },
                            )),
                            hash_to_scalar: Some(build_create_hash_to_scalar_params(
                                CreateHashToScalarParamsOverrides {
                                    concrete: Some(hash_to_scalar_selection.concrete),
                                    identifier: Some(hash_to_scalar_selection.identifier),
                                },
                            )),
                        }),
                        build_challenge_case_payload(Default::default()),
                    )
                    .expect("the case runner ran the consumer");
                }
            }
        }
    }
}

/// Contract: entry `the_challenge_tag_is_admitted_as_a_domain_tag`;
///   `DomainTag::try_new` over `SCHNORR_FS_CHALLENGE_TAG` returns `Ok(tag)`
///   with `tag.as_bytes()` equal to the constant.
/// Arrange: this block runs no consumer; the params
///   `build_domain_tag_constructor_params(DomainTagConstructorParamsOverrides
///   { bytes: Some(SCHNORR_FS_CHALLENGE_TAG.to_vec()) })`, so a constant the
///   family refuses fails.
/// Act:     `DomainTag::try_new(params)`.
/// Assert:  the result is `Ok`, and `tag.as_bytes()` equals
///   `SCHNORR_FS_CHALLENGE_TAG` by `assert_eq!`.
/// Boundary: the call `DomainTag::try_new` over the constant this interface
///   declares and the route `DomainTag::try_new` and `DomainTag::as_bytes`.
/// Mocked:  nothing.
#[test]
fn the_challenge_tag_is_admitted_as_a_domain_tag() {
    // Arrange
    let params = build_domain_tag_constructor_params(DomainTagConstructorParamsOverrides {
        bytes: Some(SCHNORR_FS_CHALLENGE_TAG.to_vec()),
    });

    // Act
    let tag = DomainTag::try_new(params).expect("the proof tag is admitted as a domain tag");

    // Assert
    assert_eq!(tag.as_bytes(), SCHNORR_FS_CHALLENGE_TAG);
}
