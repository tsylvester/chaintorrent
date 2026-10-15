#![allow(clippy::expect_used)]

use super::challenge;
use super::provides::{
    ChallengeTranscript, SCHNORR_FS_CHALLENGE_TAG, build_challenge_deps, build_challenge_params,
    build_challenge_payload,
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
    CHAIN_FORMS_CONCRETES, ConsumeChainFormsParams, ConsumeChainFormsPayload,
    CreateChainFormsParamsOverrides, IChainForms, IChainFormsConsumer,
    build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, create_chain_forms,
};
use domain::{
    ASSET_IDENTITY_HASH_LENGTH, AssetIdentityHashConstructorParamsOverrides,
    PARAMETER_SET_IDENTIFIER_LENGTH, ParameterSetIdentifierConstructorParamsOverrides,
    SecretConstructorParamsOverrides, SuiteIdentifierConstructorParamsOverrides,
    build_asset_identity_hash, build_parameter_set_identifier, build_secret,
    build_suite_identifier,
};
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, ConsumeEncodingParams, ConsumeEncodingPayload,
    CreateEncodingParamsOverrides, DECLARED_ENCODING_SELECTIONS, EncodeParams,
    FromFieldParamsOverrides, ICanonicalField, IDecoderAdapter, IEncoderAdapter, IEncodingConsumer,
    build_create_encoding_deps, build_create_encoding_params, build_create_encoding_payload,
    build_from_field_params, create_encoding,
};
use envelope::{
    EnvelopeComponentsOverrides, PublicKeysComponentsOverrides, build_envelope_components,
    build_public_keys_components,
};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DECLARED_HASH_TO_SCALAR_SELECTIONS, DomainTag,
    DomainTagConstructorParamsOverrides, HashToScalarIdentifier, build_create_hash_to_scalar_deps,
    build_create_hash_to_scalar_params, build_create_hash_to_scalar_payload, build_domain_tag,
    build_domain_tag_constructor_params, create_hash_to_scalar, expected_uniform_bytes,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParamsOverrides,
    CreatePairingPayload, EncodeScalarParams, EncodeScalarPayload, IPairingAdapter,
    IPairingArithmetic, IPairingConsumer, IPairingReference, ISampleUniformScalar,
    PAIRING_CONCRETES, SampleUniformScalarParams, SampleUniformScalarPayload,
    build_create_pairing_deps, build_create_pairing_params, create_pairing,
};

/// Contract: entry `the_mint_challenge_is_the_tagged_hash_of_the_mint_transcripts_encoding`;
///   `challenge` over `ChallengeTranscript::Mint(statement)` returns
///   `Ok(ChallengeSuccessReturn { challenge })` whose encoding equals the
///   encoding of the oracle scalar of the encoder's bytes for the statement
///   under the proof's tag.
/// Arrange: every member of `DECLARED_ENCODING_SELECTIONS`, each over every
///   member of `PAIRING_CONCRETES`, of `CHAIN_FORMS_CONCRETES`, and of
///   `keccak_cases`; for each of `MintPurpose::Mint` and
///   `MintPurpose::Replacement` the statement
///   `build_mint_statement(&pairing, MintStatementOverrides { purpose:
///   Some(purpose), ..Default::default() })`, its values distinct at every
///   position; the bytes from the received encoder's `encode` under
///   `EncodeParams` over `build_mint_statement_description(&pairing,
///   Default::default())`; the oracle scalar sampled from
///   `expected_uniform_bytes(&tag, &bytes, P::Scalar::UNIFORM_BYTES_LENGTH)`.
/// Act:     `challenge(&deps, params, payload)` over
///   `ChallengeTranscript::Mint(&statement)`.
/// Assert:  in place, the result is `Ok`, and the encoding of its `challenge`
///   through the received pairing's `encode_scalar` equals the encoding of
///   the oracle scalar by `assert_eq!`; every factory result `is_ok()`.
/// Boundary: the private call `challenge` and the route `create_pairing`,
///   `consume_pairing`, `create_chain_forms`, `consume_chain_forms`,
///   `create_encoding`, `consume_encoding`, `create_hash_to_scalar`,
///   `DomainTag::try_new`, the description's `try_new` and `to_fields`, the
///   encoder's `encode`, and the adapter's `hash_to_scalar`.
/// Mocked:  nothing; the operating system is not reached.
#[test]
fn the_mint_challenge_is_the_tagged_hash_of_the_mint_transcripts_encoding() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct MiddleConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
            }

            impl<P: IPairingAdapter> IChainFormsConsumer for MiddleConsumer<'_, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    struct InnerConsumer<'a, P: IPairingAdapter, F: IChainForms> {
                        pairing: &'a P,
                        _forms: core::marker::PhantomData<F>,
                    }

                    impl<P: IPairingAdapter, F: IChainForms> IEncodingConsumer for InnerConsumer<'_, P, F> {
                        type Output = ();

                        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                            &self,
                            _params: ConsumeEncodingParams,
                            encoded: ConsumeEncodingPayload<E>,
                        ) -> Self::Output {
                            let keccak_cases =
                                DECLARED_HASH_TO_SCALAR_SELECTIONS.iter().filter(|case| {
                                    case.identifier == HashToScalarIdentifier::Keccak256V1
                                });
                            for case in keccak_cases {
                                // Arrange
                                let hashed = create_hash_to_scalar::<P::Scalar>(
                                    &build_create_hash_to_scalar_deps(Default::default()),
                                    build_create_hash_to_scalar_params(
                                        CreateHashToScalarParamsOverrides {
                                            concrete: Some(case.concrete),
                                            identifier: Some(case.identifier),
                                        },
                                    ),
                                    build_create_hash_to_scalar_payload(Default::default()),
                                )
                                .expect("the declared selection admits the concrete it names");
                                let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                                    bytes: Some(SCHNORR_FS_CHALLENGE_TAG.to_vec()),
                                });
                                let description = build_mint_statement_description::<P, F>(
                                    self.pairing,
                                    Default::default(),
                                );
                                let deps = build_challenge_deps(
                                    self.pairing,
                                    &encoded.adapter,
                                    &tag,
                                    &*hashed.adapter,
                                    Default::default(),
                                );
                                for purpose in [MintPurpose::Mint, MintPurpose::Replacement] {
                                    let statement = build_mint_statement::<P, F>(
                                        self.pairing,
                                        MintStatementOverrides {
                                            purpose: Some(purpose),
                                            ..Default::default()
                                        },
                                    );
                                    let bytes = encoded
                                        .adapter
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
                                        self.pairing
                                            .encode_scalar(
                                                EncodeScalarParams,
                                                EncodeScalarPayload {
                                                    scalar: result
                                                        .expect(
                                                            "the mint transcript admits a challenge",
                                                        )
                                                        .challenge,
                                                },
                                            )
                                            .expect("encoding the challenge scalar succeeds")
                                            .bytes
                                            .expose()
                                            .as_ref(),
                                        self.pairing
                                            .encode_scalar(
                                                EncodeScalarParams,
                                                EncodeScalarPayload { scalar: oracle },
                                            )
                                            .expect("encoding the oracle scalar succeeds")
                                            .bytes
                                            .expose()
                                            .as_ref(),
                                    );
                                }
                            }
                        }
                    }

                    for selection in DECLARED_ENCODING_SELECTIONS {
                        let created = create_encoding(
                            &build_create_encoding_deps(InnerConsumer {
                                pairing: self.pairing,
                                _forms: core::marker::PhantomData::<F>,
                            }),
                            build_create_encoding_params(CreateEncodingParamsOverrides {
                                concrete: Some(selection.concrete),
                                identifier: Some(selection.identifier),
                            }),
                            build_create_encoding_payload(Default::default()),
                        );
                        assert!(created.is_ok());
                    }
                }
            }

            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        MiddleConsumer {
                            pairing: &payload.adapter,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(OuterConsumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok());
    }
}

/// Contract: entry `the_transfer_challenge_is_the_tagged_hash_of_the_transfer_transcripts_encoding`;
///   `challenge` over `ChallengeTranscript::Transfer(statement)` returns
///   `Ok(ChallengeSuccessReturn { challenge })` whose encoding equals the
///   encoding of the oracle scalar of the encoder's bytes for the statement
///   under the proof's tag.
/// Arrange: the declared sets of the mint block, and for each of
///   `TransferPurpose::Transfer` and `TransferPurpose::Grant` the statement
///   `build_transfer_statement(&pairing, TransferStatementOverrides { purpose:
///   Some(purpose), ..Default::default() })`, its values distinct at every
///   position; the bytes from the received encoder's `encode` over
///   `build_transfer_statement_description(&pairing, Default::default())`; the
///   oracle scalar sampled from `expected_uniform_bytes(&tag, &bytes,
///   P::Scalar::UNIFORM_BYTES_LENGTH)`.
/// Act:     `challenge(&deps, params, payload)` over
///   `ChallengeTranscript::Transfer(&statement)`.
/// Assert:  in place, the result is `Ok`, and the encoding of its `challenge`
///   through the received pairing's `encode_scalar` equals the encoding of
///   the oracle scalar by `assert_eq!`; every factory result `is_ok()`.
/// Boundary: the private call `challenge` and the route of the mint block with
///   `TransferStatementDescription` in place of `MintStatementDescription`.
/// Mocked:  nothing.
#[test]
fn the_transfer_challenge_is_the_tagged_hash_of_the_transfer_transcripts_encoding() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct MiddleConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
            }

            impl<P: IPairingAdapter> IChainFormsConsumer for MiddleConsumer<'_, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    struct InnerConsumer<'a, P: IPairingAdapter, F: IChainForms> {
                        pairing: &'a P,
                        _forms: core::marker::PhantomData<F>,
                    }

                    impl<P: IPairingAdapter, F: IChainForms> IEncodingConsumer for InnerConsumer<'_, P, F> {
                        type Output = ();

                        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                            &self,
                            _params: ConsumeEncodingParams,
                            encoded: ConsumeEncodingPayload<E>,
                        ) -> Self::Output {
                            let keccak_cases =
                                DECLARED_HASH_TO_SCALAR_SELECTIONS.iter().filter(|case| {
                                    case.identifier == HashToScalarIdentifier::Keccak256V1
                                });
                            for case in keccak_cases {
                                // Arrange
                                let hashed = create_hash_to_scalar::<P::Scalar>(
                                    &build_create_hash_to_scalar_deps(Default::default()),
                                    build_create_hash_to_scalar_params(
                                        CreateHashToScalarParamsOverrides {
                                            concrete: Some(case.concrete),
                                            identifier: Some(case.identifier),
                                        },
                                    ),
                                    build_create_hash_to_scalar_payload(Default::default()),
                                )
                                .expect("the declared selection admits the concrete it names");
                                let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                                    bytes: Some(SCHNORR_FS_CHALLENGE_TAG.to_vec()),
                                });
                                let description = build_transfer_statement_description::<P, F>(
                                    self.pairing,
                                    Default::default(),
                                );
                                let deps = build_challenge_deps(
                                    self.pairing,
                                    &encoded.adapter,
                                    &tag,
                                    &*hashed.adapter,
                                    Default::default(),
                                );
                                for purpose in [TransferPurpose::Transfer, TransferPurpose::Grant] {
                                    let statement = build_transfer_statement::<P, F>(
                                        self.pairing,
                                        TransferStatementOverrides {
                                            purpose: Some(purpose),
                                            ..Default::default()
                                        },
                                    );
                                    let bytes = encoded
                                        .adapter
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
                                        self.pairing
                                            .encode_scalar(
                                                EncodeScalarParams,
                                                EncodeScalarPayload {
                                                    scalar: result
                                                        .expect(
                                                            "the transfer transcript admits a challenge",
                                                        )
                                                        .challenge,
                                                },
                                            )
                                            .expect("encoding the challenge scalar succeeds")
                                            .bytes
                                            .expose()
                                            .as_ref(),
                                        self.pairing
                                            .encode_scalar(
                                                EncodeScalarParams,
                                                EncodeScalarPayload { scalar: oracle },
                                            )
                                            .expect("encoding the oracle scalar succeeds")
                                            .bytes
                                            .expose()
                                            .as_ref(),
                                    );
                                }
                            }
                        }
                    }

                    for selection in DECLARED_ENCODING_SELECTIONS {
                        let created = create_encoding(
                            &build_create_encoding_deps(InnerConsumer {
                                pairing: self.pairing,
                                _forms: core::marker::PhantomData::<F>,
                            }),
                            build_create_encoding_params(CreateEncodingParamsOverrides {
                                concrete: Some(selection.concrete),
                                identifier: Some(selection.identifier),
                            }),
                            build_create_encoding_payload(Default::default()),
                        );
                        assert!(created.is_ok());
                    }
                }
            }

            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        MiddleConsumer {
                            pairing: &payload.adapter,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(OuterConsumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok());
    }
}

/// Contract: entry `a_transcript_differing_in_one_field_yields_another_challenge`;
///   `challenge` over a statement that differs from the baseline in one field
///   returns the oracle scalar of the variant's encoding, which is not the
///   oracle scalar of the baseline's encoding.
/// Arrange: the declared sets of the mint block; for each field of
///   `MintStatement` and of `TransferStatement`, the variant
///   `build_mint_statement(&pairing, MintStatementOverrides { <field>:
///   Some(<value from that field's own builder distinct from the default>),
///   ..Default::default() })` or its transfer counterpart; the baseline
///   `build_mint_statement(&pairing, Default::default())` or its transfer
///   counterpart; the oracle scalars sampled from `expected_uniform_bytes`
///   over the encoder's bytes for the variant and for the baseline.
/// Act:     `challenge(&deps, params, payload)` over the variant.
/// Assert:  in place, the encoding of the result's `challenge` equals the
///   encoding of the variant's oracle scalar and differs from the encoding of
///   the baseline's oracle scalar, the equality by `assert_eq!` and the
///   difference by `assert_ne!`; every factory result `is_ok()`.
/// Boundary: the private call `challenge` and the route of the mint block, run
///   for each transcript variant.
/// Mocked:  nothing.
#[test]
fn a_transcript_differing_in_one_field_yields_another_challenge() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct MiddleConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
            }

            impl<P: IPairingAdapter> IChainFormsConsumer for MiddleConsumer<'_, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    struct InnerConsumer<'a, P: IPairingAdapter, F: IChainForms> {
                        pairing: &'a P,
                        _forms: core::marker::PhantomData<F>,
                    }

                    impl<P: IPairingAdapter, F: IChainForms> IEncodingConsumer for InnerConsumer<'_, P, F> {
                        type Output = ();

                        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                            &self,
                            _params: ConsumeEncodingParams,
                            encoded: ConsumeEncodingPayload<E>,
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
                                        CanonicalFieldKind::Bytes => {
                                            CanonicalFieldValue::Bytes(vec![seed])
                                        }
                                        CanonicalFieldKind::Text => {
                                            CanonicalFieldValue::Text(seed.to_string())
                                        }
                                    },
                                )
                                .expect("a seeded field of the declared kind is admitted")
                                .value
                            }

                            let keccak_cases =
                                DECLARED_HASH_TO_SCALAR_SELECTIONS.iter().filter(|case| {
                                    case.identifier == HashToScalarIdentifier::Keccak256V1
                                });
                            for case in keccak_cases {
                                // Arrange
                                let hashed = create_hash_to_scalar::<P::Scalar>(
                                    &build_create_hash_to_scalar_deps(Default::default()),
                                    build_create_hash_to_scalar_params(
                                        CreateHashToScalarParamsOverrides {
                                            concrete: Some(case.concrete),
                                            identifier: Some(case.identifier),
                                        },
                                    ),
                                    build_create_hash_to_scalar_payload(Default::default()),
                                )
                                .expect("the declared selection admits the concrete it names");
                                let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                                    bytes: Some(SCHNORR_FS_CHALLENGE_TAG.to_vec()),
                                });
                                let deps = build_challenge_deps(
                                    self.pairing,
                                    &encoded.adapter,
                                    &tag,
                                    &*hashed.adapter,
                                    Default::default(),
                                );
                                let mint_description = build_mint_statement_description::<P, F>(
                                    self.pairing,
                                    Default::default(),
                                );
                                let mint_baseline =
                                    build_mint_statement::<P, F>(self.pairing, Default::default());
                                let mint_baseline_bytes = encoded
                                    .adapter
                                    .encode(
                                        EncodeParams {
                                            description: &mint_description,
                                        },
                                        &mint_baseline,
                                    )
                                    .expect("a built statement encodes")
                                    .bytes;
                                let mint_baseline_oracle =
                                    P::Scalar::sample_from_uniform_bytes(
                                        SampleUniformScalarParams,
                                        SampleUniformScalarPayload {
                                            uniform: build_secret(
                                                expected_uniform_bytes(
                                                    &tag,
                                                    &mint_baseline_bytes,
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
                                let mint_baseline_encoding = self
                                    .pairing
                                    .encode_scalar(
                                        EncodeScalarParams,
                                        EncodeScalarPayload {
                                            scalar: mint_baseline_oracle,
                                        },
                                    )
                                    .expect("encoding the oracle scalar succeeds")
                                    .bytes
                                    .expose()
                                    .as_ref()
                                    .to_vec();
                                let mint_variants = [
                                    build_mint_statement::<P, F>(
                                        self.pairing,
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
                                        self.pairing,
                                        MintStatementOverrides {
                                            chain: Some(
                                                variant_form::<F::ChainIdentifier>(0xA1),
                                            ),
                                            ..Default::default()
                                        },
                                    ),
                                    build_mint_statement::<P, F>(
                                        self.pairing,
                                        MintStatementOverrides {
                                            entitlement_contract: Some(
                                                variant_form::<F::Identity>(0xA2),
                                            ),
                                            ..Default::default()
                                        },
                                    ),
                                    build_mint_statement::<P, F>(
                                        self.pairing,
                                        MintStatementOverrides {
                                            asset_identity_hash: Some(
                                                build_asset_identity_hash(
                                                    AssetIdentityHashConstructorParamsOverrides {
                                                        bytes: Some(
                                                            [0xA4; ASSET_IDENTITY_HASH_LENGTH],
                                                        ),
                                                    },
                                                ),
                                            ),
                                            ..Default::default()
                                        },
                                    ),
                                    build_mint_statement::<P, F>(
                                        self.pairing,
                                        MintStatementOverrides {
                                            parameter_set_identifier: Some(
                                                build_parameter_set_identifier(
                                                    ParameterSetIdentifierConstructorParamsOverrides {
                                                        bytes: Some(
                                                            [0xA5; PARAMETER_SET_IDENTIFIER_LENGTH],
                                                        ),
                                                    },
                                                ),
                                            ),
                                            ..Default::default()
                                        },
                                    ),
                                    build_mint_statement::<P, F>(
                                        self.pairing,
                                        MintStatementOverrides {
                                            source_entitlement: Some(
                                                variant_form::<F::Entitlement>(0xA6),
                                            ),
                                            ..Default::default()
                                        },
                                    ),
                                    build_mint_statement::<P, F>(
                                        self.pairing,
                                        MintStatementOverrides {
                                            target_entitlement: Some(
                                                variant_form::<F::Entitlement>(0xA7),
                                            ),
                                            ..Default::default()
                                        },
                                    ),
                                    build_mint_statement::<P, F>(
                                        self.pairing,
                                        MintStatementOverrides {
                                            old_interval: Some(
                                                variant_form::<F::Interval>(0xA8),
                                            ),
                                            ..Default::default()
                                        },
                                    ),
                                    build_mint_statement::<P, F>(
                                        self.pairing,
                                        MintStatementOverrides {
                                            new_interval: Some(
                                                variant_form::<F::Interval>(0xA9),
                                            ),
                                            ..Default::default()
                                        },
                                    ),
                                    build_mint_statement::<P, F>(
                                        self.pairing,
                                        MintStatementOverrides {
                                            purpose: Some(MintPurpose::Replacement),
                                            ..Default::default()
                                        },
                                    ),
                                    build_mint_statement::<P, F>(
                                        self.pairing,
                                        MintStatementOverrides {
                                            seller: Some(variant_form::<F::Identity>(0xAA)),
                                            ..Default::default()
                                        },
                                    ),
                                    build_mint_statement::<P, F>(
                                        self.pairing,
                                        MintStatementOverrides {
                                            buyer: Some(variant_form::<F::Identity>(0xAB)),
                                            ..Default::default()
                                        },
                                    ),
                                    build_mint_statement::<P, F>(
                                        self.pairing,
                                        MintStatementOverrides {
                                            buyer_keys: Some(build_public_keys_components(
                                                self.pairing,
                                                PublicKeysComponentsOverrides::default(),
                                            )),
                                            ..Default::default()
                                        },
                                    ),
                                    build_mint_statement::<P, F>(
                                        self.pairing,
                                        MintStatementOverrides {
                                            envelope: Some(build_envelope_components(
                                                self.pairing,
                                                EnvelopeComponentsOverrides::default(),
                                            )),
                                            ..Default::default()
                                        },
                                    ),
                                    build_mint_statement::<P, F>(
                                        self.pairing,
                                        MintStatementOverrides {
                                            expiry: Some(1_700_000_001),
                                            ..Default::default()
                                        },
                                    ),
                                    build_mint_statement::<P, F>(
                                        self.pairing,
                                        MintStatementOverrides {
                                            first_messages: Some(build_mint_first_messages(
                                                self.pairing,
                                                MintFirstMessagesOverrides {
                                                    hpub: Some(
                                                        build_public_keys_components(
                                                            self.pairing,
                                                            PublicKeysComponentsOverrides::default(
                                                            ),
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
                                    let variant_bytes = encoded
                                        .adapter
                                        .encode(
                                            EncodeParams {
                                                description: &mint_description,
                                            },
                                            variant,
                                        )
                                        .expect("a variant statement encodes")
                                        .bytes;
                                    let variant_oracle =
                                        P::Scalar::sample_from_uniform_bytes(
                                            SampleUniformScalarParams,
                                            SampleUniformScalarPayload {
                                                uniform: build_secret(
                                                    expected_uniform_bytes(
                                                        &tag,
                                                        &variant_bytes,
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
                                    let variant_encoding = self
                                        .pairing
                                        .encode_scalar(
                                            EncodeScalarParams,
                                            EncodeScalarPayload {
                                                scalar: variant_oracle,
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
                                            ChallengeTranscript::Mint(variant),
                                            Default::default(),
                                        ),
                                    );

                                    // Assert
                                    let challenge_encoding = self
                                        .pairing
                                        .encode_scalar(
                                            EncodeScalarParams,
                                            EncodeScalarPayload {
                                                scalar: result
                                                    .expect(
                                                        "the variant transcript admits a challenge",
                                                    )
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
                                    build_transfer_statement_description::<P, F>(
                                        self.pairing,
                                        Default::default(),
                                    );
                                let transfer_baseline = build_transfer_statement::<P, F>(
                                    self.pairing,
                                    Default::default(),
                                );
                                let transfer_baseline_bytes = encoded
                                    .adapter
                                    .encode(
                                        EncodeParams {
                                            description: &transfer_description,
                                        },
                                        &transfer_baseline,
                                    )
                                    .expect("a built statement encodes")
                                    .bytes;
                                let transfer_baseline_oracle =
                                    P::Scalar::sample_from_uniform_bytes(
                                        SampleUniformScalarParams,
                                        SampleUniformScalarPayload {
                                            uniform: build_secret(
                                                expected_uniform_bytes(
                                                    &tag,
                                                    &transfer_baseline_bytes,
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
                                let transfer_baseline_encoding = self
                                    .pairing
                                    .encode_scalar(
                                        EncodeScalarParams,
                                        EncodeScalarPayload {
                                            scalar: transfer_baseline_oracle,
                                        },
                                    )
                                    .expect("encoding the oracle scalar succeeds")
                                    .bytes
                                    .expose()
                                    .as_ref()
                                    .to_vec();
                                let transfer_variants = [
                                    build_transfer_statement::<P, F>(
                                        self.pairing,
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
                                        self.pairing,
                                        TransferStatementOverrides {
                                            chain: Some(
                                                variant_form::<F::ChainIdentifier>(0xB1),
                                            ),
                                            ..Default::default()
                                        },
                                    ),
                                    build_transfer_statement::<P, F>(
                                        self.pairing,
                                        TransferStatementOverrides {
                                            entitlement_contract: Some(
                                                variant_form::<F::Identity>(0xB2),
                                            ),
                                            ..Default::default()
                                        },
                                    ),
                                    build_transfer_statement::<P, F>(
                                        self.pairing,
                                        TransferStatementOverrides {
                                            asset_identity_hash: Some(
                                                build_asset_identity_hash(
                                                    AssetIdentityHashConstructorParamsOverrides {
                                                        bytes: Some(
                                                            [0xB4; ASSET_IDENTITY_HASH_LENGTH],
                                                        ),
                                                    },
                                                ),
                                            ),
                                            ..Default::default()
                                        },
                                    ),
                                    build_transfer_statement::<P, F>(
                                        self.pairing,
                                        TransferStatementOverrides {
                                            parameter_set_identifier: Some(
                                                build_parameter_set_identifier(
                                                    ParameterSetIdentifierConstructorParamsOverrides {
                                                        bytes: Some(
                                                            [0xB5; PARAMETER_SET_IDENTIFIER_LENGTH],
                                                        ),
                                                    },
                                                ),
                                            ),
                                            ..Default::default()
                                        },
                                    ),
                                    build_transfer_statement::<P, F>(
                                        self.pairing,
                                        TransferStatementOverrides {
                                            source_entitlement: Some(
                                                variant_form::<F::Entitlement>(0xB6),
                                            ),
                                            ..Default::default()
                                        },
                                    ),
                                    build_transfer_statement::<P, F>(
                                        self.pairing,
                                        TransferStatementOverrides {
                                            target_entitlement: Some(
                                                variant_form::<F::Entitlement>(0xB7),
                                            ),
                                            ..Default::default()
                                        },
                                    ),
                                    build_transfer_statement::<P, F>(
                                        self.pairing,
                                        TransferStatementOverrides {
                                            old_interval: Some(
                                                variant_form::<F::Interval>(0xB8),
                                            ),
                                            ..Default::default()
                                        },
                                    ),
                                    build_transfer_statement::<P, F>(
                                        self.pairing,
                                        TransferStatementOverrides {
                                            new_interval: Some(
                                                variant_form::<F::Interval>(0xB9),
                                            ),
                                            ..Default::default()
                                        },
                                    ),
                                    build_transfer_statement::<P, F>(
                                        self.pairing,
                                        TransferStatementOverrides {
                                            purpose: Some(TransferPurpose::Grant),
                                            ..Default::default()
                                        },
                                    ),
                                    build_transfer_statement::<P, F>(
                                        self.pairing,
                                        TransferStatementOverrides {
                                            seller: Some(variant_form::<F::Identity>(0xBA)),
                                            ..Default::default()
                                        },
                                    ),
                                    build_transfer_statement::<P, F>(
                                        self.pairing,
                                        TransferStatementOverrides {
                                            buyer: Some(variant_form::<F::Identity>(0xBB)),
                                            ..Default::default()
                                        },
                                    ),
                                    build_transfer_statement::<P, F>(
                                        self.pairing,
                                        TransferStatementOverrides {
                                            seller_keys: Some(build_public_keys_components(
                                                self.pairing,
                                                PublicKeysComponentsOverrides::default(),
                                            )),
                                            ..Default::default()
                                        },
                                    ),
                                    build_transfer_statement::<P, F>(
                                        self.pairing,
                                        TransferStatementOverrides {
                                            buyer_keys: Some(build_public_keys_components(
                                                self.pairing,
                                                PublicKeysComponentsOverrides::default(),
                                            )),
                                            ..Default::default()
                                        },
                                    ),
                                    build_transfer_statement::<P, F>(
                                        self.pairing,
                                        TransferStatementOverrides {
                                            old_envelope: Some(build_envelope_components(
                                                self.pairing,
                                                EnvelopeComponentsOverrides::default(),
                                            )),
                                            ..Default::default()
                                        },
                                    ),
                                    build_transfer_statement::<P, F>(
                                        self.pairing,
                                        TransferStatementOverrides {
                                            new_envelope: Some(build_envelope_components(
                                                self.pairing,
                                                EnvelopeComponentsOverrides::default(),
                                            )),
                                            ..Default::default()
                                        },
                                    ),
                                    build_transfer_statement::<P, F>(
                                        self.pairing,
                                        TransferStatementOverrides {
                                            expiry: Some(1_700_000_001),
                                            ..Default::default()
                                        },
                                    ),
                                    build_transfer_statement::<P, F>(
                                        self.pairing,
                                        TransferStatementOverrides {
                                            first_messages: Some(
                                                build_transfer_first_messages(
                                                    self.pairing,
                                                    TransferFirstMessagesOverrides {
                                                        pk1: Some(
                                                            build_public_keys_components(
                                                                self.pairing,
                                                                PublicKeysComponentsOverrides::default(),
                                                            )
                                                            .pk1,
                                                        ),
                                                        ..Default::default()
                                                    },
                                                ),
                                            ),
                                            ..Default::default()
                                        },
                                    ),
                                ];
                                for variant in &transfer_variants {
                                    let variant_bytes = encoded
                                        .adapter
                                        .encode(
                                            EncodeParams {
                                                description: &transfer_description,
                                            },
                                            variant,
                                        )
                                        .expect("a variant statement encodes")
                                        .bytes;
                                    let variant_oracle =
                                        P::Scalar::sample_from_uniform_bytes(
                                            SampleUniformScalarParams,
                                            SampleUniformScalarPayload {
                                                uniform: build_secret(
                                                    expected_uniform_bytes(
                                                        &tag,
                                                        &variant_bytes,
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
                                    let variant_encoding = self
                                        .pairing
                                        .encode_scalar(
                                            EncodeScalarParams,
                                            EncodeScalarPayload {
                                                scalar: variant_oracle,
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
                                    let challenge_encoding = self
                                        .pairing
                                        .encode_scalar(
                                            EncodeScalarParams,
                                            EncodeScalarPayload {
                                                scalar: result
                                                    .expect(
                                                        "the variant transcript admits a challenge",
                                                    )
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
                    }

                    for selection in DECLARED_ENCODING_SELECTIONS {
                        let created = create_encoding(
                            &build_create_encoding_deps(InnerConsumer {
                                pairing: self.pairing,
                                _forms: core::marker::PhantomData::<F>,
                            }),
                            build_create_encoding_params(CreateEncodingParamsOverrides {
                                concrete: Some(selection.concrete),
                                identifier: Some(selection.identifier),
                            }),
                            build_create_encoding_payload(Default::default()),
                        );
                        assert!(created.is_ok());
                    }
                }
            }

            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        MiddleConsumer {
                            pairing: &payload.adapter,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(OuterConsumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok());
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
