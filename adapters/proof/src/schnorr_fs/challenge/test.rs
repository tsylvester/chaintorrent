#![allow(clippy::expect_used)]

use super::challenge;
use super::provides::{
    ChallengeErrorReturn, ChallengeTranscript, SCHNORR_FS_CHALLENGE_TAG, build_challenge_deps,
    build_challenge_params, build_challenge_payload,
};
use crate::mint_statement::provides::{
    MintStatementOverrides, build_mint_statement, build_mint_statement_description,
};
use crate::transfer_statement::provides::{
    TransferStatementOverrides, build_transfer_statement, build_transfer_statement_description,
};
use chain::{
    ConsumeChainFormsParams, ConsumeChainFormsPayload, CreateChainFormsReturn, IChainForms,
    IChainFormsConsumer, build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, create_chain_forms,
};
use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingReturn, EncodeParams,
    IDecoderAdapter, IEncoderAdapter, IEncodingConsumer, build_create_encoding_deps,
    build_create_encoding_params, build_create_encoding_payload, create_encoding,
};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DomainTagConstructorParamsOverrides, HashToScalarConcrete,
    HashToScalarErrorReturn, HashToScalarParams, HashToScalarPayload,
    MockIHashToScalarAdapterFailureMode, build_create_hash_to_scalar_deps,
    build_create_hash_to_scalar_params, build_create_hash_to_scalar_payload, build_domain_tag,
    create_hash_to_scalar,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingPayload, EncodeScalarParams,
    EncodeScalarPayload, IPairingAdapter, IPairingArithmetic, IPairingConsumer, IPairingReference,
    ISampleUniformScalar, SampleUniformScalarErrorReturn, build_create_pairing_deps,
    build_create_pairing_params, create_pairing,
};

/// Contract: `challenge`, encoding of a mint transcript: `payload.transcript`
///   is `ChallengeTranscript::Mint(statement)` → the encoder's `encode` over
///   `MintStatementDescription` yields the bytes, the adapter hashes them
///   under `deps.tag`, and the result is `Ok(ChallengeSuccessReturn
///   { challenge })` holding the adapter's scalar.
/// Arrange: `build_mint_statement(&pairing, Default::default())`; `params`
///   from `build_create_hash_to_scalar_params(Default::default())`; the
///   expected scalar built from the received encoder's `encode(…,
///   &statement)` under `EncodeParams` over
///   `build_mint_statement_description(&pairing, Default::default())`,
///   hashed by the received adapter under the tag, so a challenge that
///   hashes another tag, a part of the encoding, or another statement's
///   encoding yields another scalar; the tag differs from the tag
///   `build_hash_to_scalar_params` defaults to.
/// Act:     `challenge(&deps, params, payload)` over
///   `ChallengeTranscript::Mint(&statement)`.
/// Assert:  in place, the result is `Ok`, and the encoding of its
///   `challenge` through the received pairing's `encode_scalar`, read
///   through `Secret::expose`, equals the encoding of the expected scalar
///   by `assert_eq!`; outside, every factory result `is_ok()`.
/// Collaborators: the pairing, chain, encoding, and hash-to-scalar
///   families' mock concretes through `create_pairing`, `create_chain_forms`,
///   `create_encoding`, and `create_hash_to_scalar`.
#[test]
fn challenge_hashes_the_encoding_of_a_mint_transcript_under_the_tag() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<CreateEncodingReturn<()>>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct MiddleConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
            }

            impl<P: IPairingAdapter> IChainFormsConsumer for MiddleConsumer<'_, P> {
                type Output = CreateEncodingReturn<()>;

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
                            // Arrange
                            let hashed = create_hash_to_scalar::<P::Scalar>(
                                &build_create_hash_to_scalar_deps(Default::default()),
                                build_create_hash_to_scalar_params(Default::default()),
                                build_create_hash_to_scalar_payload(Default::default()),
                            )
                            .expect(
                                "the default hash-to-scalar selection constructs the family's mock concrete",
                            );
                            let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                                bytes: Some(SCHNORR_FS_CHALLENGE_TAG.to_vec()),
                            });
                            let statement = build_mint_statement::<P, F>(
                                self.pairing,
                                MintStatementOverrides::default(),
                            );
                            let deps = build_challenge_deps(
                                self.pairing,
                                &encoded.adapter,
                                &tag,
                                &*hashed.adapter,
                                Default::default(),
                            );
                            let params = build_challenge_params(Default::default());
                            let transcript = build_challenge_payload(
                                ChallengeTranscript::Mint(&statement),
                                Default::default(),
                            );
                            let description = build_mint_statement_description::<P, F>(
                                self.pairing,
                                Default::default(),
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
                            let expected = hashed
                                .adapter
                                .hash_to_scalar(
                                    HashToScalarParams { tag: &tag },
                                    HashToScalarPayload { message: &bytes },
                                )
                                .expect("the received adapter hashes the encoding")
                                .scalar;

                            // Act
                            let result = challenge(&deps, params, transcript);

                            // Assert
                            let challenge_scalar = result
                                .expect("the mint transcript admits a challenge")
                                .challenge;
                            assert_eq!(
                                self.pairing
                                    .encode_scalar(
                                        EncodeScalarParams,
                                        EncodeScalarPayload {
                                            scalar: challenge_scalar,
                                        },
                                    )
                                    .expect("encoding the challenge scalar succeeds")
                                    .bytes
                                    .expose()
                                    .as_ref(),
                                self.pairing
                                    .encode_scalar(
                                        EncodeScalarParams,
                                        EncodeScalarPayload { scalar: expected },
                                    )
                                    .expect("encoding the expected scalar succeeds")
                                    .bytes
                                    .expose()
                                    .as_ref(),
                            );
                        }
                    }

                    create_encoding(
                        &build_create_encoding_deps(InnerConsumer {
                            pairing: self.pairing,
                            _forms: core::marker::PhantomData::<F>,
                        }),
                        build_create_encoding_params(Default::default()),
                        build_create_encoding_payload(Default::default()),
                    )
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    MiddleConsumer {
                        pairing: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    assert!(created.is_ok());
    let chained = created
        .expect("the pairing factory ran the consumer")
        .output;
    assert!(chained.is_ok());
    let encoded = chained
        .expect("the chain-forms factory ran the consumer")
        .output;
    assert!(encoded.is_ok());
}

/// Contract: `challenge`, encoding of a transfer transcript:
///   `payload.transcript` is `ChallengeTranscript::Transfer(statement)` → the
///   encoder's `encode` over `TransferStatementDescription` yields the bytes,
///   the adapter hashes them under `deps.tag`, and the result is
///   `Ok(ChallengeSuccessReturn { challenge })` holding the adapter's scalar.
/// Arrange: `build_transfer_statement(&pairing, Default::default())`;
///   `params` from `build_create_hash_to_scalar_params(Default::default())`;
///   the expected scalar built from the received encoder's `encode(…,
///   &statement)` under `EncodeParams` over
///   `build_transfer_statement_description(&pairing, Default::default())`,
///   hashed by the received adapter under the tag, so a challenge that
///   hashes another tag, a part of the encoding, or the mint description's
///   encoding yields another scalar.
/// Act:     `challenge(&deps, params, payload)` over
///   `ChallengeTranscript::Transfer(&statement)`.
/// Assert:  in place, the result is `Ok`, and the encoding of its
///   `challenge` through the received pairing's `encode_scalar`, read
///   through `Secret::expose`, equals the encoding of the expected scalar
///   by `assert_eq!`; outside, every factory result `is_ok()`.
/// Collaborators: the pairing, chain, encoding, and hash-to-scalar
///   families' mock concretes through `create_pairing`, `create_chain_forms`,
///   `create_encoding`, and `create_hash_to_scalar`.
#[test]
fn challenge_hashes_the_encoding_of_a_transfer_transcript_under_the_tag() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<CreateEncodingReturn<()>>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct MiddleConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
            }

            impl<P: IPairingAdapter> IChainFormsConsumer for MiddleConsumer<'_, P> {
                type Output = CreateEncodingReturn<()>;

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
                            // Arrange
                            let hashed = create_hash_to_scalar::<P::Scalar>(
                                &build_create_hash_to_scalar_deps(Default::default()),
                                build_create_hash_to_scalar_params(Default::default()),
                                build_create_hash_to_scalar_payload(Default::default()),
                            )
                            .expect(
                                "the default hash-to-scalar selection constructs the family's mock concrete",
                            );
                            let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                                bytes: Some(SCHNORR_FS_CHALLENGE_TAG.to_vec()),
                            });
                            let statement = build_transfer_statement::<P, F>(
                                self.pairing,
                                TransferStatementOverrides::default(),
                            );
                            let deps = build_challenge_deps(
                                self.pairing,
                                &encoded.adapter,
                                &tag,
                                &*hashed.adapter,
                                Default::default(),
                            );
                            let params = build_challenge_params(Default::default());
                            let transcript = build_challenge_payload(
                                ChallengeTranscript::Transfer(&statement),
                                Default::default(),
                            );
                            let description = build_transfer_statement_description::<P, F>(
                                self.pairing,
                                Default::default(),
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
                            let expected = hashed
                                .adapter
                                .hash_to_scalar(
                                    HashToScalarParams { tag: &tag },
                                    HashToScalarPayload { message: &bytes },
                                )
                                .expect("the received adapter hashes the encoding")
                                .scalar;

                            // Act
                            let result = challenge(&deps, params, transcript);

                            // Assert
                            let challenge_scalar = result
                                .expect("the transfer transcript admits a challenge")
                                .challenge;
                            assert_eq!(
                                self.pairing
                                    .encode_scalar(
                                        EncodeScalarParams,
                                        EncodeScalarPayload {
                                            scalar: challenge_scalar,
                                        },
                                    )
                                    .expect("encoding the challenge scalar succeeds")
                                    .bytes
                                    .expose()
                                    .as_ref(),
                                self.pairing
                                    .encode_scalar(
                                        EncodeScalarParams,
                                        EncodeScalarPayload { scalar: expected },
                                    )
                                    .expect("encoding the expected scalar succeeds")
                                    .bytes
                                    .expose()
                                    .as_ref(),
                            );
                        }
                    }

                    create_encoding(
                        &build_create_encoding_deps(InnerConsumer {
                            pairing: self.pairing,
                            _forms: core::marker::PhantomData::<F>,
                        }),
                        build_create_encoding_params(Default::default()),
                        build_create_encoding_payload(Default::default()),
                    )
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    MiddleConsumer {
                        pairing: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    assert!(created.is_ok());
    let chained = created
        .expect("the pairing factory ran the consumer")
        .output;
    assert!(chained.is_ok());
    let encoded = chained
        .expect("the chain-forms factory ran the consumer")
        .output;
    assert!(encoded.is_ok());
}

/// Contract: `challenge`, hash refused: the adapter's `hash_to_scalar`
///   returns `Err(error)` → `Err(ChallengeErrorReturn::HashToScalar(error))`,
///   the refusal unchanged.
/// Arrange: the adapter's `params` from
///   `build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides
///   { concrete: Some(HashToScalarConcrete::Mock
///   (MockIHashToScalarAdapterFailureMode::SamplingRefused)),
///   ..Default::default() })`, whose adapter hashes into the sampling bound's
///   refusal; the transcript `ChallengeTranscript::Mint(&statement)` with
///   `build_mint_statement(&pairing, Default::default())`.
/// Act:     `challenge(&deps, params, payload)`.
/// Assert:  in place, `result.err()` equals
///   `Some(ChallengeErrorReturn::HashToScalar(HashToScalarErrorReturn::
///   Sampling(SampleUniformScalarErrorReturn::WrongLength { expected:
///   P::Scalar::UNIFORM_BYTES_LENGTH, actual: 0 })))` by `assert_eq!`, the
///   bound read from the received scalar type and the length from the empty
///   input the failure mode supplies; outside, every factory result
///   `is_ok()`.
/// Collaborators: the pairing, chain, and encoding families' mock concretes
///   through `create_pairing`, `create_chain_forms`, and `create_encoding`,
///   and the hash-to-scalar family's mock concrete under
///   `MockIHashToScalarAdapterFailureMode::SamplingRefused` through
///   `create_hash_to_scalar`.
#[test]
fn challenge_returns_the_hash_to_scalar_refusal_unchanged() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<CreateEncodingReturn<()>>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct MiddleConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
            }

            impl<P: IPairingAdapter> IChainFormsConsumer for MiddleConsumer<'_, P> {
                type Output = CreateEncodingReturn<()>;

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
                            // Arrange
                            let hashed = create_hash_to_scalar::<P::Scalar>(
                                &build_create_hash_to_scalar_deps(Default::default()),
                                build_create_hash_to_scalar_params(
                                    CreateHashToScalarParamsOverrides {
                                        concrete: Some(HashToScalarConcrete::Mock(
                                            MockIHashToScalarAdapterFailureMode::SamplingRefused,
                                        )),
                                        ..Default::default()
                                    },
                                ),
                                build_create_hash_to_scalar_payload(Default::default()),
                            )
                            .expect(
                                "the sampling-refused hash-to-scalar selection constructs the family's mock concrete",
                            );
                            let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                                bytes: Some(SCHNORR_FS_CHALLENGE_TAG.to_vec()),
                            });
                            let statement = build_mint_statement::<P, F>(
                                self.pairing,
                                MintStatementOverrides::default(),
                            );
                            let deps = build_challenge_deps(
                                self.pairing,
                                &encoded.adapter,
                                &tag,
                                &*hashed.adapter,
                                Default::default(),
                            );
                            let params = build_challenge_params(Default::default());
                            let transcript = build_challenge_payload(
                                ChallengeTranscript::Mint(&statement),
                                Default::default(),
                            );

                            // Act
                            let result = challenge(&deps, params, transcript);

                            // Assert
                            assert_eq!(
                                result.err(),
                                Some(ChallengeErrorReturn::HashToScalar(
                                    HashToScalarErrorReturn::Sampling(
                                        SampleUniformScalarErrorReturn::WrongLength {
                                            expected: P::Scalar::UNIFORM_BYTES_LENGTH,
                                            actual: 0,
                                        },
                                    ),
                                )),
                            );
                        }
                    }

                    create_encoding(
                        &build_create_encoding_deps(InnerConsumer {
                            pairing: self.pairing,
                            _forms: core::marker::PhantomData::<F>,
                        }),
                        build_create_encoding_params(Default::default()),
                        build_create_encoding_payload(Default::default()),
                    )
                }
            }

            create_chain_forms(
                &build_create_chain_forms_deps(
                    MiddleConsumer {
                        pairing: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    assert!(created.is_ok());
    let chained = created
        .expect("the pairing factory ran the consumer")
        .output;
    assert!(chained.is_ok());
    let encoded = chained
        .expect("the chain-forms factory ran the consumer")
        .output;
    assert!(encoded.is_ok());
}
