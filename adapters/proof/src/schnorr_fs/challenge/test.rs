#![allow(clippy::expect_used)]

use super::challenge;
use super::provides::{
    ChallengeCaseParamsOverrides, ChallengeCasePayload, ChallengeErrorReturn, ChallengeTranscript,
    IChallengeCaseConsumer, SCHNORR_FS_CHALLENGE_TAG, build_challenge_case_deps,
    build_challenge_case_params, build_challenge_case_payload, build_challenge_deps,
    build_challenge_params, build_challenge_payload, run_challenge_case,
};
use crate::mint_statement::provides::{
    MintStatementOverrides, build_mint_statement, build_mint_statement_description,
};
use crate::transfer_statement::provides::{
    TransferStatementOverrides, build_transfer_statement, build_transfer_statement_description,
};
use chain::IChainForms;
use encoding::{EncodeParams, IDecoderAdapter, IEncoderAdapter};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DomainTagConstructorParamsOverrides, HashToScalarConcrete,
    HashToScalarErrorReturn, HashToScalarParams, HashToScalarPayload, IHashToScalarAdapter,
    MockIHashToScalarAdapterFailureMode, build_create_hash_to_scalar_params, build_domain_tag,
};
use pairing::{
    EncodeScalarParams, EncodeScalarPayload, IPairingArithmetic, IPairingReference,
    ISampleUniformScalar, SampleUniformScalarErrorReturn,
};

/// Contract: `challenge`, encoding of a mint transcript: `payload.transcript`
///   is `ChallengeTranscript::Mint(statement)` → the encoder's `encode` over
///   `MintStatementDescription` yields the bytes, the adapter hashes them
///   under `deps.tag`, and the result is `Ok(ChallengeSuccessReturn
///   { challenge })` holding the adapter's scalar.
/// Arrange: `build_mint_statement(pairing, Default::default())`, its values
///   distinct at every position; the case params' `hash_to_scalar` from
///   `build_create_hash_to_scalar_params(Default::default())`; the expected
///   scalar built from the received encoder's `encode(…, &statement)` under
///   `EncodeParams` over `build_mint_statement_description(pairing,
///   Default::default())`, hashed by the received adapter under the tag, so
///   a challenge that hashes another tag, a part of the encoding, or another
///   statement's encoding yields another scalar; the tag differs from the
///   tag `build_hash_to_scalar_params` defaults to.
/// Act:     `challenge(&deps, params, payload)` over
///   `ChallengeTranscript::Mint(&statement)` inside the case consumer's
///   method.
/// Assert:  in place, the result is `Ok`, and the encoding of its
///   `challenge` through the received pairing's `encode_scalar`, read
///   through `Secret::expose`, equals the encoding of the expected scalar
///   by `assert_eq!`; outside, the case runner's `Ok` arm.
/// Collaborators: the pairing, chain, encoding, and hash-to-scalar families'
///   mock concretes reached through the case runner under the default
///   selections.
#[test]
fn challenge_hashes_the_encoding_of_a_mint_transcript_under_the_tag() {
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
            let statement =
                build_mint_statement::<P, F>(pairing, MintStatementOverrides::default());
            let description = build_mint_statement_description::<P, F>(pairing, Default::default());
            let bytes = encoder
                .encode(
                    EncodeParams {
                        description: &description,
                    },
                    &statement,
                )
                .expect("a built statement encodes")
                .bytes;
            let expected = hash_to_scalar
                .hash_to_scalar(
                    HashToScalarParams { tag: &tag },
                    HashToScalarPayload { message: &bytes },
                )
                .expect("the received adapter hashes the encoding")
                .scalar;
            let deps =
                build_challenge_deps(pairing, encoder, &tag, hash_to_scalar, Default::default());
            let params = build_challenge_params(Default::default());
            let payload =
                build_challenge_payload(ChallengeTranscript::Mint(&statement), Default::default());

            // Act
            let result = challenge(&deps, params, payload);

            // Assert
            let challenge_scalar = result
                .expect("the mint transcript admits a challenge")
                .challenge;
            assert_eq!(
                pairing
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
                pairing
                    .encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar: expected },)
                    .expect("encoding the expected scalar succeeds")
                    .bytes
                    .expose()
                    .as_ref(),
            );
        }
    }

    run_challenge_case(
        &build_challenge_case_deps(Consumer, Default::default()),
        build_challenge_case_params(ChallengeCaseParamsOverrides {
            hash_to_scalar: Some(build_create_hash_to_scalar_params(Default::default())),
            ..Default::default()
        }),
        build_challenge_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer");
}

/// Contract: `challenge`, encoding of a transfer transcript:
///   `payload.transcript` is `ChallengeTranscript::Transfer(statement)` →
///   the encoder's `encode` over `TransferStatementDescription` yields the
///   bytes, the adapter hashes them under `deps.tag`, and the result is
///   `Ok(ChallengeSuccessReturn { challenge })` holding the adapter's
///   scalar.
/// Arrange: `build_transfer_statement(pairing, Default::default())`, its
///   values distinct at every position; the case params' `hash_to_scalar`
///   from `build_create_hash_to_scalar_params(Default::default())`; the
///   expected scalar built from the received encoder's `encode(…,
///   &statement)` under `EncodeParams` over
///   `build_transfer_statement_description(pairing, Default::default())`,
///   hashed by the received adapter under the tag, so a challenge that
///   hashes another tag, a part of the encoding, or the mint description's
///   encoding yields another scalar.
/// Act:     `challenge(&deps, params, payload)` over
///   `ChallengeTranscript::Transfer(&statement)` inside the case consumer's
///   method.
/// Assert:  in place, the result is `Ok`, and the encoding of its
///   `challenge` through the received pairing's `encode_scalar`, read
///   through `Secret::expose`, equals the encoding of the expected scalar
///   by `assert_eq!`; outside, the case runner's `Ok` arm.
/// Collaborators: the pairing, chain, encoding, and hash-to-scalar families'
///   mock concretes reached through the case runner under the default
///   selections.
#[test]
fn challenge_hashes_the_encoding_of_a_transfer_transcript_under_the_tag() {
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
            let statement =
                build_transfer_statement::<P, F>(pairing, TransferStatementOverrides::default());
            let description =
                build_transfer_statement_description::<P, F>(pairing, Default::default());
            let bytes = encoder
                .encode(
                    EncodeParams {
                        description: &description,
                    },
                    &statement,
                )
                .expect("a built statement encodes")
                .bytes;
            let expected = hash_to_scalar
                .hash_to_scalar(
                    HashToScalarParams { tag: &tag },
                    HashToScalarPayload { message: &bytes },
                )
                .expect("the received adapter hashes the encoding")
                .scalar;
            let deps =
                build_challenge_deps(pairing, encoder, &tag, hash_to_scalar, Default::default());
            let params = build_challenge_params(Default::default());
            let payload = build_challenge_payload(
                ChallengeTranscript::Transfer(&statement),
                Default::default(),
            );

            // Act
            let result = challenge(&deps, params, payload);

            // Assert
            let challenge_scalar = result
                .expect("the transfer transcript admits a challenge")
                .challenge;
            assert_eq!(
                pairing
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
                pairing
                    .encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar: expected },)
                    .expect("encoding the expected scalar succeeds")
                    .bytes
                    .expose()
                    .as_ref(),
            );
        }
    }

    run_challenge_case(
        &build_challenge_case_deps(Consumer, Default::default()),
        build_challenge_case_params(ChallengeCaseParamsOverrides {
            hash_to_scalar: Some(build_create_hash_to_scalar_params(Default::default())),
            ..Default::default()
        }),
        build_challenge_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer");
}

/// Contract: `challenge`, hash refused: the adapter's `hash_to_scalar`
///   returns `Err(error)` → `Err(ChallengeErrorReturn::HashToScalar(error))`,
///   the refusal unchanged.
/// Arrange: the case params' `hash_to_scalar` overridden to
///   `build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides
///   { concrete: Some(HashToScalarConcrete::Mock
///   (MockIHashToScalarAdapterFailureMode::SamplingRefused)),
///   ..Default::default() })`, whose adapter hashes into the sampling
///   bound's refusal; the transcript `ChallengeTranscript::Mint(&statement)`
///   with `build_mint_statement(pairing, Default::default())`.
/// Act:     `challenge(&deps, params, payload)` inside the case consumer's
///   method.
/// Assert:  in place, `result.err()` equals
///   `Some(ChallengeErrorReturn::HashToScalar(HashToScalarErrorReturn::
///   Sampling(SampleUniformScalarErrorReturn::WrongLength { expected:
///   P::Scalar::UNIFORM_BYTES_LENGTH, actual: 0 })))` by `assert_eq!`, the
///   bound read from the received scalar type and the length from the empty
///   input the failure mode supplies; outside, the case runner's `Ok` arm.
/// Collaborators: the pairing, chain, and encoding families' mock concretes
///   reached through the case runner, and the hash-to-scalar family's mock
///   concrete under `MockIHashToScalarAdapterFailureMode::SamplingRefused`.
#[test]
fn challenge_returns_the_hash_to_scalar_refusal_unchanged() {
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
            let statement =
                build_mint_statement::<P, F>(pairing, MintStatementOverrides::default());
            let deps =
                build_challenge_deps(pairing, encoder, &tag, hash_to_scalar, Default::default());
            let params = build_challenge_params(Default::default());
            let payload =
                build_challenge_payload(ChallengeTranscript::Mint(&statement), Default::default());

            // Act
            let result = challenge(&deps, params, payload);

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

    run_challenge_case(
        &build_challenge_case_deps(Consumer, Default::default()),
        build_challenge_case_params(ChallengeCaseParamsOverrides {
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
        build_challenge_case_payload(Default::default()),
    )
    .expect("the case runner ran the consumer");
}
