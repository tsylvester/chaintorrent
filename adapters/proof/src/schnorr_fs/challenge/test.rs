#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::challenge;
use super::interface::{
    ChallengeDeps, ChallengeErrorReturn, ChallengeParams, ChallengePayload, ChallengeTranscript,
    SCHNORR_FS_CHALLENGE_TAG,
};
use crate::mint_statement::provides::{
    MintStatementDescription, MintStatementDescriptionConstructorParams, MintStatementOverrides,
    build_mint_statement,
};
use crate::transfer_statement::provides::{
    TransferStatementDescription, TransferStatementDescriptionConstructorParams,
    TransferStatementOverrides, build_transfer_statement,
};
use chain::MockIChainForms;
use core::cell::Cell;
use core::marker::PhantomData;
use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingDeps, CreateEncodingPayload,
    ENCODING_INTERFACE_VERSION, EncodeErrorReturn, EncodeParams, EncodeReturn, EncodingDeclaration,
    EncodingIdentifier, IEncoderAdapter, IEncodingConsumer, IEncodingContract,
    build_create_encoding_params, create_encoding,
};
use hash_to_scalar::{
    CreateHashToScalarDeps, CreateHashToScalarPayload, DomainTag, DomainTagConstructorParams,
    HashToScalarDeclaration, HashToScalarParams, HashToScalarPayload, HashToScalarReturn,
    IHashToScalarAdapter, build_create_hash_to_scalar_params, build_hash_to_scalar_declaration,
    create_hash_to_scalar,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingDeps, CreatePairingParamsOverrides,
    CreatePairingPayload, EncodeScalarParams, EncodeScalarPayload, IPairingAdapter,
    IPairingConsumer, PairingConcrete, build_create_pairing_params, create_pairing,
};

struct ChallengeOutcome {
    mint_is_the_tagged_hash: bool,
    transfer_is_the_tagged_hash: bool,
    mint_repeats: bool,
    transfer_repeats: bool,
    mint_field_changes_it: bool,
    transfer_field_changes_it: bool,
}

fn scalars_equal<P: IPairingAdapter>(pairing: &P, left: &P::Scalar, right: &P::Scalar) -> bool {
    let Ok(left_encoded) = pairing.encode_scalar(
        EncodeScalarParams,
        EncodeScalarPayload {
            scalar: left.clone(),
        },
    );
    let Ok(right_encoded) = pairing.encode_scalar(
        EncodeScalarParams,
        EncodeScalarPayload {
            scalar: right.clone(),
        },
    );
    left_encoded.bytes.expose().as_ref() == right_encoded.bytes.expose().as_ref()
}

struct ChallengeStep<'a, P: IPairingAdapter> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
}

impl<P: IPairingAdapter> IEncodingConsumer for ChallengeStep<'_, P> {
    type Output = ChallengeOutcome;

    fn consume_encoding<E: IEncoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
        let encoder = &payload.adapter;
        let pairing = self.pairing;
        let Ok(tag) = DomainTag::try_new(DomainTagConstructorParams {
            bytes: SCHNORR_FS_CHALLENGE_TAG.to_vec(),
        }) else {
            panic!("the challenge tag is admitted as a domain tag")
        };
        let deps = ChallengeDeps {
            pairing,
            encoder,
            tag: &tag,
            hash_to_scalar: self.hash_to_scalar,
        };

        let mint = build_mint_statement::<P, MockIChainForms>(pairing, Default::default());
        let mint_altered = build_mint_statement::<P, MockIChainForms>(
            pairing,
            MintStatementOverrides {
                expiry: Some(1_700_000_001),
                ..Default::default()
            },
        );
        let transfer = build_transfer_statement::<P, MockIChainForms>(pairing, Default::default());
        let transfer_altered = build_transfer_statement::<P, MockIChainForms>(
            pairing,
            TransferStatementOverrides {
                expiry: Some(1_700_000_001),
                ..Default::default()
            },
        );

        let Ok(mint_description) = MintStatementDescription::<P, MockIChainForms>::try_new(
            MintStatementDescriptionConstructorParams { pairing },
        );
        let Ok(mint_bytes) = encoder.encode(
            EncodeParams {
                description: &mint_description,
            },
            &mint,
        ) else {
            panic!("the ABI encoder admits a well-typed mint transcript")
        };
        let Ok(mint_expected) = self.hash_to_scalar.hash_to_scalar(
            HashToScalarParams { tag: &tag },
            HashToScalarPayload {
                message: &mint_bytes.bytes,
            },
        ) else {
            panic!("the keccak256 mapping admits the tag and the encoding")
        };
        let mint_actual = challenge(
            &deps,
            ChallengeParams,
            ChallengePayload {
                transcript: ChallengeTranscript::Mint(&mint),
            },
        );
        let mint_repeat = challenge(
            &deps,
            ChallengeParams,
            ChallengePayload {
                transcript: ChallengeTranscript::Mint(&mint),
            },
        );
        let mint_altered_actual = challenge(
            &deps,
            ChallengeParams,
            ChallengePayload {
                transcript: ChallengeTranscript::Mint(&mint_altered),
            },
        );

        let Ok(transfer_description) = TransferStatementDescription::<P, MockIChainForms>::try_new(
            TransferStatementDescriptionConstructorParams { pairing },
        );
        let Ok(transfer_bytes) = encoder.encode(
            EncodeParams {
                description: &transfer_description,
            },
            &transfer,
        ) else {
            panic!("the ABI encoder admits a well-typed transfer transcript")
        };
        let Ok(transfer_expected) = self.hash_to_scalar.hash_to_scalar(
            HashToScalarParams { tag: &tag },
            HashToScalarPayload {
                message: &transfer_bytes.bytes,
            },
        ) else {
            panic!("the keccak256 mapping admits the tag and the encoding")
        };
        let transfer_actual = challenge(
            &deps,
            ChallengeParams,
            ChallengePayload {
                transcript: ChallengeTranscript::Transfer(&transfer),
            },
        );
        let transfer_repeat = challenge(
            &deps,
            ChallengeParams,
            ChallengePayload {
                transcript: ChallengeTranscript::Transfer(&transfer),
            },
        );
        let transfer_altered_actual = challenge(
            &deps,
            ChallengeParams,
            ChallengePayload {
                transcript: ChallengeTranscript::Transfer(&transfer_altered),
            },
        );

        let (
            Ok(mint_actual),
            Ok(mint_repeat),
            Ok(mint_altered_actual),
            Ok(transfer_actual),
            Ok(transfer_repeat),
            Ok(transfer_altered_actual),
        ) = (
            mint_actual,
            mint_repeat,
            mint_altered_actual,
            transfer_actual,
            transfer_repeat,
            transfer_altered_actual,
        )
        else {
            panic!("a well-typed transcript admits a challenge")
        };

        ChallengeOutcome {
            mint_is_the_tagged_hash: scalars_equal(
                pairing,
                &mint_actual.challenge,
                &mint_expected.scalar,
            ),
            transfer_is_the_tagged_hash: scalars_equal(
                pairing,
                &transfer_actual.challenge,
                &transfer_expected.scalar,
            ),
            mint_repeats: scalars_equal(pairing, &mint_actual.challenge, &mint_repeat.challenge),
            transfer_repeats: scalars_equal(
                pairing,
                &transfer_actual.challenge,
                &transfer_repeat.challenge,
            ),
            mint_field_changes_it: !scalars_equal(
                pairing,
                &mint_actual.challenge,
                &mint_altered_actual.challenge,
            ),
            transfer_field_changes_it: !scalars_equal(
                pairing,
                &transfer_actual.challenge,
                &transfer_altered_actual.challenge,
            ),
        }
    }
}

struct ChallengeProbe;

impl IPairingConsumer for ChallengeProbe {
    type Output = ChallengeOutcome;

    fn consume_pairing<P: IPairingAdapter>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let Ok(hash_to_scalar) = create_hash_to_scalar::<P::Scalar>(
            &CreateHashToScalarDeps,
            build_create_hash_to_scalar_params(Default::default()),
            CreateHashToScalarPayload,
        ) else {
            panic!("the hash-to-scalar factory resolves the concrete")
        };
        let step = ChallengeStep {
            pairing: &payload.adapter,
            hash_to_scalar: &*hash_to_scalar.adapter,
        };
        let Ok(success) = create_encoding(
            &CreateEncodingDeps { consumer: step },
            build_create_encoding_params(Default::default()),
            CreateEncodingPayload,
        ) else {
            panic!("the encoding factory resolves the concrete")
        };
        success.output
    }
}

struct RefusingEncoder;

impl IEncoderAdapter for RefusingEncoder {
    const DECLARATION: EncodingDeclaration = EncodingDeclaration {
        identifier: EncodingIdentifier::EthereumAbiV1,
        adapter_version: 1,
        interface_version: ENCODING_INTERFACE_VERSION,
    };

    fn encode<D: IEncodingContract>(
        &self,
        _params: EncodeParams<'_, D>,
        _payload: &D::Described,
    ) -> EncodeReturn {
        Err(EncodeErrorReturn::FieldCount {
            expected: 2,
            actual: 1,
        })
    }
}

struct HashToScalarSpy<S> {
    calls: Cell<usize>,
    scalar: PhantomData<S>,
}

impl<S: pairing::ISampleUniformScalar + Clone> IHashToScalarAdapter<S> for HashToScalarSpy<S> {
    fn declaration(&self) -> HashToScalarDeclaration {
        build_hash_to_scalar_declaration(Default::default())
    }

    fn hash_to_scalar(
        &self,
        _params: HashToScalarParams<'_>,
        _payload: HashToScalarPayload<'_>,
    ) -> HashToScalarReturn<S> {
        self.calls.set(self.calls.get() + 1);
        panic!("an encoding refusal prevents the hash call")
    }
}

struct RefusalOutcome {
    mint_error: ChallengeErrorReturn,
    transfer_error: ChallengeErrorReturn,
    hash_calls: usize,
}

struct RefusalProbe;

impl IPairingConsumer for RefusalProbe {
    type Output = RefusalOutcome;

    fn consume_pairing<P: IPairingAdapter>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let pairing = &payload.adapter;
        let encoder = RefusingEncoder;
        let spy = HashToScalarSpy {
            calls: Cell::new(0),
            scalar: PhantomData,
        };
        let Ok(tag) = DomainTag::try_new(DomainTagConstructorParams {
            bytes: SCHNORR_FS_CHALLENGE_TAG.to_vec(),
        }) else {
            panic!("the challenge tag is admitted as a domain tag")
        };
        let deps = ChallengeDeps {
            pairing,
            encoder: &encoder,
            tag: &tag,
            hash_to_scalar: &spy,
        };

        let mint = build_mint_statement::<P, MockIChainForms>(pairing, Default::default());
        let transfer = build_transfer_statement::<P, MockIChainForms>(pairing, Default::default());

        let Err(mint_error) = challenge(
            &deps,
            ChallengeParams,
            ChallengePayload {
                transcript: ChallengeTranscript::Mint(&mint),
            },
        ) else {
            panic!("a refusing encoder is refused")
        };
        let Err(transfer_error) = challenge(
            &deps,
            ChallengeParams,
            ChallengePayload {
                transcript: ChallengeTranscript::Transfer(&transfer),
            },
        ) else {
            panic!("a refusing encoder is refused")
        };

        RefusalOutcome {
            mint_error,
            transfer_error,
            hash_calls: spy.calls.get(),
        }
    }
}

/// Contract: a mint transcript's challenge is the mapping under the proof's
///   tag of its encoding through the mint description (CR-09).
/// Arrange: `ChallengeProbe` — the keccak256 hash-to-scalar adapter and the ABI
///   encoder resolved by their factories inside a real BLS12-381 arkworks
///   pairing, and a typed mint transcript over `MockIChainForms`.
/// Act:     `create_pairing`, which runs `challenge` on the mint transcript and
///   hashes the transcript's independently computed encoding under the tag.
/// Assert:  `mint_is_the_tagged_hash` — the function's scalar equals the
///   independently computed one, compared through `encode_scalar`.
#[test]
fn the_mint_challenge_is_the_tagged_hash_of_the_mint_transcripts_encoding() {
    // Arrange
    let consumer = ChallengeProbe;
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bls12381Arkworks),
        ..Default::default()
    });

    // Act
    let Ok(success) = create_pairing(
        &CreatePairingDeps { consumer },
        params,
        CreatePairingPayload,
    ) else {
        panic!("the pairing factory resolves the concrete")
    };

    // Assert
    assert!(success.output.mint_is_the_tagged_hash);
}

/// Contract: a transfer transcript's challenge is the mapping under the
///   proof's tag of its encoding through the transfer description (CR-09).
/// Arrange: `ChallengeProbe` as above, with a typed transfer transcript.
/// Act:     `create_pairing`, which runs `challenge` on the transfer transcript
///   and hashes its independently computed encoding under the tag.
/// Assert:  `transfer_is_the_tagged_hash`.
#[test]
fn the_transfer_challenge_is_the_tagged_hash_of_the_transfer_transcripts_encoding() {
    // Arrange
    let consumer = ChallengeProbe;
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bls12381Arkworks),
        ..Default::default()
    });

    // Act
    let Ok(success) = create_pairing(
        &CreatePairingDeps { consumer },
        params,
        CreatePairingPayload,
    ) else {
        panic!("the pairing factory resolves the concrete")
    };

    // Assert
    assert!(success.output.transfer_is_the_tagged_hash);
}

/// Contract: the challenge the prover computes is the one the verifier
///   recomputes — the same transcript always yields the same challenge.
/// Arrange: `ChallengeProbe` as above, each transcript challenged twice.
/// Act:     `create_pairing`, which calls `challenge` twice per transcript.
/// Assert:  `mint_repeats` and `transfer_repeats`.
#[test]
fn the_same_transcript_always_yields_the_same_challenge() {
    // Arrange
    let consumer = ChallengeProbe;
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bls12381Arkworks),
        ..Default::default()
    });

    // Act
    let Ok(success) = create_pairing(
        &CreatePairingDeps { consumer },
        params,
        CreatePairingPayload,
    ) else {
        panic!("the pairing factory resolves the concrete")
    };

    // Assert
    assert!(success.output.mint_repeats);
    assert!(success.output.transfer_repeats);
}

/// Contract: the challenge binds every field, so a proof is valid for exactly
///   one settlement — a transcript differing in one field yields another
///   challenge (CR-09).
/// Arrange: `ChallengeProbe` as above, each transcript alongside a fixture
///   altered only in `expiry`.
/// Act:     `create_pairing`, which challenges the unaltered and the altered
///   transcript of each relation.
/// Assert:  `mint_field_changes_it` and `transfer_field_changes_it`.
#[test]
fn a_transcript_differing_in_one_field_yields_another_challenge() {
    // Arrange
    let consumer = ChallengeProbe;
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bls12381Arkworks),
        ..Default::default()
    });

    // Act
    let Ok(success) = create_pairing(
        &CreatePairingDeps { consumer },
        params,
        CreatePairingPayload,
    ) else {
        panic!("the pairing factory resolves the concrete")
    };

    // Assert
    assert!(success.output.mint_field_changes_it);
    assert!(success.output.transfer_field_changes_it);
}

/// Contract: the mapping holds on the other curve's scalar type — a mint or
///   transfer transcript's challenge is the tagged hash of its encoding on
///   BN254's scalar too.
/// Arrange: `ChallengeProbe` under `PairingConcrete::Bn254Arkworks`.
/// Act:     `create_pairing`.
/// Assert:  `mint_is_the_tagged_hash` and `transfer_is_the_tagged_hash`.
#[test]
fn the_challenge_is_the_tagged_hash_on_bn254_arkworks() {
    // Arrange
    let consumer = ChallengeProbe;
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bn254Arkworks),
        ..Default::default()
    });

    // Act
    let Ok(success) = create_pairing(
        &CreatePairingDeps { consumer },
        params,
        CreatePairingPayload,
    ) else {
        panic!("the pairing factory resolves the concrete")
    };

    // Assert
    assert!(success.output.mint_is_the_tagged_hash);
    assert!(success.output.transfer_is_the_tagged_hash);
}

/// Contract: the proof's domain tag is the declared literal and the
///   hash-to-scalar family's tag type admits it.
/// Arrange: `SCHNORR_FS_CHALLENGE_TAG` and its one-time `DomainTag`
///   construction.
/// Act:     `DomainTag::try_new` over the constant.
/// Assert:  the constant equals `b"ChainTorrent-v1-proof-challenge"` and the
///   construction succeeds.
#[test]
fn the_challenge_tag_is_admitted_as_a_domain_tag() {
    // Arrange
    let bytes = SCHNORR_FS_CHALLENGE_TAG;

    // Act
    let tag = DomainTag::try_new(DomainTagConstructorParams {
        bytes: bytes.to_vec(),
    });

    // Assert
    assert_eq!(bytes, b"ChainTorrent-v1-proof-challenge");
    assert!(tag.is_ok());
}

/// Contract: an encoding refusal returns `ChallengeErrorReturn::Encoding` with
///   the refusal unchanged and prevents the hash call.
/// Arrange: `RefusalProbe` — a test-local encoder whose `encode` always returns
///   `EncodeErrorReturn::FieldCount { expected: 2, actual: 1 }`, a hash-to-scalar
///   spy recording calls, and well-typed mint and transfer transcripts.
/// Act:     `create_pairing`, which calls `challenge` on both transcripts.
/// Assert:  each refusal equals `ChallengeErrorReturn::Encoding(
///   EncodeErrorReturn::FieldCount { expected: 2, actual: 1 })` and the spy
///   recorded no call.
#[test]
fn encoding_refusal_prevents_hashing() {
    // Arrange
    let consumer = RefusalProbe;
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bls12381Arkworks),
        ..Default::default()
    });

    // Act
    let Ok(success) = create_pairing(
        &CreatePairingDeps { consumer },
        params,
        CreatePairingPayload,
    ) else {
        panic!("the pairing factory resolves the concrete")
    };

    // Assert
    assert_eq!(
        success.output.mint_error,
        ChallengeErrorReturn::Encoding(EncodeErrorReturn::FieldCount {
            expected: 2,
            actual: 1
        })
    );
    assert_eq!(
        success.output.transfer_error,
        ChallengeErrorReturn::Encoding(EncodeErrorReturn::FieldCount {
            expected: 2,
            actual: 1
        })
    );
    assert_eq!(success.output.hash_calls, 0);
}
