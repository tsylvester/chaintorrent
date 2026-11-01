#![allow(clippy::expect_used)]

use super::challenge::provides::{
    ChallengeErrorReturn, ChallengeTranscript, SCHNORR_FS_CHALLENGE_TAG, build_challenge_deps,
    build_challenge_params, build_challenge_payload, challenge,
};
use super::provides::{
    ISchnorrFsCaseConsumer, SCHNORR_FS_WEIGHT_TAG, SchnorrFsCaseParamsOverrides,
    SchnorrFsCasePayload, SchnorrFsDeliveryProof, SchnorrFsDeliveryProofConstructorParamsOverrides,
    SchnorrFsMintProofOverrides, SchnorrFsProof, SchnorrFsProveMintErrorReturn,
    SchnorrFsProveTransferErrorReturn, SchnorrFsTransferProofOverrides, SchnorrFsVerifyErrorReturn,
    build_schnorr_fs_case_deps, build_schnorr_fs_case_params, build_schnorr_fs_case_payload,
    build_schnorr_fs_delivery_proof, build_schnorr_fs_delivery_proof_constructor_params,
    build_schnorr_fs_mint_proof, build_schnorr_fs_transfer_proof, run_schnorr_fs_case,
};
use crate::factory::provides::{
    AlgebraicStatement, AlgebraicTransferStatementOverrides, DeliveryProofComponents,
    IDeliveryProofAdapter, MintSecondGroupFirstMessagesOverrides, ProofComponentsParams,
    ProofFromComponentsErrorReturn, ProofFromComponentsParams, ProofFromComponentsSuccessReturn,
    ProveMintErrorReturn, ProveMintParams, ProveMintSuccessReturn, ProveTransferErrorReturn,
    ProveTransferParams, ProveTransferSuccessReturn, TransferSecondGroupFirstMessagesOverrides,
    VerifyErrorReturn, VerifyParams, build_algebraic_mint_statement,
    build_algebraic_transfer_statement, build_delivery_context, build_mint_responses,
    build_mint_second_group_first_messages, build_proof_components_payload,
    build_proof_from_components_payload, build_prove_mint_payload, build_prove_transfer_payload,
    build_transfer_responses, build_transfer_second_group_first_messages, build_verify_payload,
};
use crate::mint_statement::provides::{
    MintFirstMessages, MintStatementOverrides, TransferPurpose, build_mint_statement,
};
use crate::transfer_statement::provides::{
    TransferFirstMessages, TransferStatementOverrides, build_transfer_statement,
};
use chain::IChainForms;
use encoding::{IDecoderAdapter, IEncoderAdapter};
use envelope::{EnvelopeAlgebra, EnvelopeComponents, PublicKeysComponents};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DomainTagConstructorParamsOverrides, HashToScalarConcrete,
    HashToScalarErrorReturn, IHashToScalarAdapter, MockIHashToScalarAdapterFailureMode,
    build_create_hash_to_scalar_deps, build_create_hash_to_scalar_params,
    build_create_hash_to_scalar_payload, build_domain_tag, create_hash_to_scalar,
};
use hashing::{Root, RootConstructorParams};
use pairing::{
    AddScalarParams, AddScalarPayload, ConsumePairingParams, ConsumePairingPayload,
    CreatePairingParamsOverrides, CreatePairingPayload, EncodeG2Params, EncodeG2Payload,
    EncodeScalarParams, EncodeScalarPayload, G1GeneratorParams, G1GeneratorPayload,
    G2GeneratorParams, G2GeneratorPayload, IPairingArithmetic, IPairingConsumer, IPairingReference,
    ISampleUniformScalar, MockIPairingAdapterFailureMode, MsmG1Params, MsmG1Payload, MsmG1Term,
    MsmG2Params, MsmG2Payload, MsmG2Term, MulG1Params, MulG1Payload, MulG2Params, MulG2Payload,
    MulScalarParams, MulScalarPayload, NegScalarParams, NegScalarPayload, PairingConcrete,
    SampleUniformScalarErrorReturn, SampleUniformScalarParams, SampleUniformScalarPayload,
    VerifierGroupArithmetic, build_create_pairing_deps, build_create_pairing_params,
    create_pairing,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParamsOverrides, CreateRandomSourcePayload,
    FillBytesErrorReturn, FillBytesParams, FillBytesPayload, IRandomSourceAdapter,
    MockIRandomSourceAdapterFailureMode, RandomSourceKind, build_create_random_source_params,
    create_random_source,
};

/// Contract: `try_new`, admitted:
///   `Ok(SchnorrFsDeliveryProof { pairing, hash_to_scalar, encoder, random,
///   … })`, holding the params' borrows.
/// Arrange: `build_schnorr_fs_delivery_proof_constructor_params` over the
///   received pairing, the adapter and the source from their factories, and
///   the received encoder, each a distinct object.
/// Act:     `SchnorrFsDeliveryProof::<P, E, F>::try_new(params)`.
/// Assert:  in place, the result is `Ok` and each of the adapter's four
///   borrows is pointer-identical to the borrow handed in by `core::ptr::eq`;
///   outside, every factory result `is_ok()`.
/// Collaborators: the pairing, chain-forms, encoding, hash-to-scalar, and
///   randomness families' mock concretes through `create_pairing`,
///   `create_chain_forms`, `create_encoding`, `create_hash_to_scalar`, and
///   `create_random_source` under the default params.
#[test]
fn try_new_holds_the_params_borrows() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Arrange
            let params = build_schnorr_fs_delivery_proof_constructor_params(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );

            // Act
            let adapter = SchnorrFsDeliveryProof::<P, E, F>::try_new(params)
                .expect("the declared tags are admitted by the domain tag type");

            // Assert
            assert!(core::ptr::eq(adapter.pairing, pairing));
            assert!(core::ptr::eq(adapter.hash_to_scalar, hash_to_scalar));
            assert!(core::ptr::eq(adapter.encoder, encoder));
            assert!(core::ptr::eq(adapter.random, random));
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(Default::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
}

/// Contract: `try_new`, admitted: the adapter holds the params' verifier
///   form.
/// Arrange: the params with `verifier_form` `BothGroups`.
/// Act:     `try_new(params)`.
/// Assert:  in place, the result is `Ok` and `matches!(adapter.verifier_form,
///   VerifierGroupArithmetic::BothGroups)`; outside, every factory result
///   `is_ok()`.
/// Collaborators: the pairing, chain-forms, encoding, hash-to-scalar, and
///   randomness families' mock concretes through `create_pairing`,
///   `create_chain_forms`, `create_encoding`, `create_hash_to_scalar`, and
///   `create_random_source` under the default params.
#[test]
fn try_new_holds_the_both_groups_form() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Arrange
            let params = build_schnorr_fs_delivery_proof_constructor_params(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                SchnorrFsDeliveryProofConstructorParamsOverrides {
                    verifier_form: Some(VerifierGroupArithmetic::BothGroups),
                },
            );

            // Act
            let adapter = SchnorrFsDeliveryProof::<P, E, F>::try_new(params)
                .expect("the declared tags are admitted by the domain tag type");

            // Assert
            assert!(matches!(
                adapter.verifier_form,
                VerifierGroupArithmetic::BothGroups
            ));
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(Default::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
}

/// Contract: `try_new`, admitted: the adapter holds the params' verifier
///   form.
/// Arrange: the params with `verifier_form` `FirstGroupOnly`, the form the
///   builder does not default to.
/// Act:     `try_new(params)`.
/// Assert:  in place, the result is `Ok` and `matches!(adapter.verifier_form,
///   VerifierGroupArithmetic::FirstGroupOnly)`; outside, every factory result
///   `is_ok()`.
/// Collaborators: the pairing, chain-forms, encoding, hash-to-scalar, and
///   randomness families' mock concretes through `create_pairing`,
///   `create_chain_forms`, `create_encoding`, `create_hash_to_scalar`, and
///   `create_random_source` under the default params.
#[test]
fn try_new_holds_the_first_group_only_form() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Arrange
            let params = build_schnorr_fs_delivery_proof_constructor_params(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                SchnorrFsDeliveryProofConstructorParamsOverrides {
                    verifier_form: Some(VerifierGroupArithmetic::FirstGroupOnly),
                },
            );

            // Act
            let adapter = SchnorrFsDeliveryProof::<P, E, F>::try_new(params)
                .expect("the declared tags are admitted by the domain tag type");

            // Assert
            assert!(matches!(
                adapter.verifier_form,
                VerifierGroupArithmetic::FirstGroupOnly
            ));
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(Default::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
}

/// Contract: `try_new` admits `SCHNORR_FS_CHALLENGE_TAG` through
///   `DomainTag::try_new` → the adapter holds the admitted tag.
/// Arrange: the default params.
/// Act:     `try_new(params)`.
/// Assert:  in place, the result is `Ok` and `adapter.challenge_tag
///   .as_bytes()` equals `b"ChainTorrent-v1-proof-challenge"` by `assert_eq!`;
///   outside, every factory result `is_ok()`.
/// Collaborators: the pairing, chain-forms, encoding, hash-to-scalar, and
///   randomness families' mock concretes through `create_pairing`,
///   `create_chain_forms`, `create_encoding`, `create_hash_to_scalar`, and
///   `create_random_source` under the default params.
#[test]
fn try_new_holds_the_challenge_tag() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Arrange
            let params = build_schnorr_fs_delivery_proof_constructor_params(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );

            // Act
            let adapter = SchnorrFsDeliveryProof::<P, E, F>::try_new(params)
                .expect("the declared tags are admitted by the domain tag type");

            // Assert
            assert_eq!(adapter.challenge_tag.as_bytes(), SCHNORR_FS_CHALLENGE_TAG);
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(Default::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
}

/// Contract: `try_new` admits `SCHNORR_FS_WEIGHT_TAG` through
///   `DomainTag::try_new` → the adapter holds the admitted tag.
/// Arrange: the default params.
/// Act:     `try_new(params)`.
/// Assert:  in place, the result is `Ok` and `adapter.weight_tag.as_bytes()`
///   equals `b"ChainTorrent-v1-proof-weight"` by `assert_eq!`; outside, every
///   factory result `is_ok()`.
/// Collaborators: the pairing, chain-forms, encoding, hash-to-scalar, and
///   randomness families' mock concretes through `create_pairing`,
///   `create_chain_forms`, `create_encoding`, `create_hash_to_scalar`, and
///   `create_random_source` under the default params.
#[test]
fn try_new_holds_the_weight_tag() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Arrange
            let params = build_schnorr_fs_delivery_proof_constructor_params(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );

            // Act
            let adapter = SchnorrFsDeliveryProof::<P, E, F>::try_new(params)
                .expect("the declared tags are admitted by the domain tag type");

            // Assert
            assert_eq!(adapter.weight_tag.as_bytes(), SCHNORR_FS_WEIGHT_TAG);
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(Default::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
}

/// Contract: `DECLARATION`: `algebras` is `&[EnvelopeAlgebra::PairingElGamal]`.
/// Arrange: the inner consumer's received `P`, `E`, and `F`; no instance is
///   built.
/// Act:     reading `SchnorrFsDeliveryProof::<'_, P, E, F>::DECLARATION`.
/// Assert:  in place, `matches!(DECLARATION.algebras,
///   [EnvelopeAlgebra::PairingElGamal])`; outside, every factory result
///   `is_ok()`.
/// Collaborators: the pairing, chain-forms, and encoding families' mock
///   concretes through `create_pairing`, `create_chain_forms`, and
///   `create_encoding` under the default params.
#[test]
fn the_declaration_lists_the_pairing_elgamal_algebra() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            _pairing: &P,
            _forms: &F,
            _encoder: &E,
            _hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Act
            let declaration = SchnorrFsDeliveryProof::<'_, P, E, F>::DECLARATION;

            // Assert
            assert!(matches!(
                declaration.algebras,
                [EnvelopeAlgebra::PairingElGamal]
            ));
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(Default::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
}

/// Contract: `DECLARATION`: `verifier_forms` lists both forms, both-groups
///   first.
/// Arrange: the inner consumer's received `P`, `E`, and `F`; no instance is
///   built.
/// Act:     reading `DECLARATION`.
/// Assert:  in place, `matches!(DECLARATION.verifier_forms,
///   [VerifierGroupArithmetic::BothGroups,
///   VerifierGroupArithmetic::FirstGroupOnly])`; outside, every factory result
///   `is_ok()`.
/// Collaborators: the pairing, chain-forms, and encoding families' mock
///   concretes through `create_pairing`, `create_chain_forms`, and
///   `create_encoding` under the default params.
#[test]
fn the_declaration_lists_both_verifier_forms() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            _pairing: &P,
            _forms: &F,
            _encoder: &E,
            _hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Act
            let declaration = SchnorrFsDeliveryProof::<'_, P, E, F>::DECLARATION;

            // Assert
            assert!(matches!(
                declaration.verifier_forms,
                [
                    VerifierGroupArithmetic::BothGroups,
                    VerifierGroupArithmetic::FirstGroupOnly
                ]
            ));
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(Default::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
}

/// Contract: `DECLARATION`: `statement_versions` is `&[1]`.
/// Arrange: the inner consumer's received `P`, `E`, and `F`; no instance is
///   built.
/// Act:     reading `DECLARATION`.
/// Assert:  in place, `assert_eq!(DECLARATION.statement_versions, &[1u16])`;
///   outside, every factory result `is_ok()`.
/// Collaborators: the pairing, chain-forms, and encoding families' mock
///   concretes through `create_pairing`, `create_chain_forms`, and
///   `create_encoding` under the default params.
#[test]
fn the_declaration_lists_the_delivery_statement_version_one() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            _pairing: &P,
            _forms: &F,
            _encoder: &E,
            _hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Act
            let declaration = SchnorrFsDeliveryProof::<'_, P, E, F>::DECLARATION;

            // Assert
            assert_eq!(declaration.statement_versions, &[1u16]);
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(Default::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
}

/// Contract: `DECLARATION`: `challenge_tag` and `weight_tag` are the proof's
///   two domain tags.
/// Arrange: the inner consumer's received `P`, `E`, and `F`; no instance is
///   built.
/// Act:     reading `DECLARATION`.
/// Assert:  in place, `challenge_tag` equals `b"ChainTorrent-v1-proof-
///   challenge"` and `weight_tag` equals `b"ChainTorrent-v1-proof-weight"`,
///   each by `assert_eq!`; outside, every factory result `is_ok()`.
/// Collaborators: the pairing, chain-forms, and encoding families' mock
///   concretes through `create_pairing`, `create_chain_forms`, and
///   `create_encoding` under the default params.
#[test]
fn the_declaration_names_the_challenge_and_weight_tags() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            _pairing: &P,
            _forms: &F,
            _encoder: &E,
            _hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Act
            let declaration = SchnorrFsDeliveryProof::<'_, P, E, F>::DECLARATION;

            // Assert
            assert_eq!(declaration.challenge_tag, SCHNORR_FS_CHALLENGE_TAG);
            assert_eq!(declaration.weight_tag, SCHNORR_FS_WEIGHT_TAG);
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(Default::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
}

/// Contract: `DECLARATION`: `adapter_version` is `1` and `interface_version`
///   is the family's interface version.
/// Arrange: the inner consumer's received `P`, `E`, and `F`; no instance is
///   built.
/// Act:     reading `DECLARATION`.
/// Assert:  in place, `assert_eq!(DECLARATION.adapter_version, 1)` and
///   `assert_eq!(DECLARATION.interface_version, 1)`; outside, every factory
///   result `is_ok()`.
/// Collaborators: the pairing, chain-forms, and encoding families' mock
///   concretes through `create_pairing`, `create_chain_forms`, and
///   `create_encoding` under the default params.
#[test]
fn the_declaration_names_its_adapter_and_interface_versions() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            _pairing: &P,
            _forms: &F,
            _encoder: &E,
            _hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Act
            let declaration = SchnorrFsDeliveryProof::<'_, P, E, F>::DECLARATION;

            // Assert
            assert_eq!(declaration.adapter_version, 1);
            assert_eq!(declaration.interface_version, 1);
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(Default::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
}

/// Contract: `prove_mint`, nonce refused: the draw refuses →
///   `Err(ProveMintErrorReturn::SchnorrFs(SchnorrFsProveMintErrorReturn::
///   NonceDraw(error)))`, the refusal unchanged.
/// Arrange: the source's `params` from `build_create_random_source_params`
///   with `kind` `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode
///   ::FillBytesRefused)`; the both-groups adapter; the default mint
///   statement and payload.
/// Act:     `prove_mint(ProveMintParams, payload)`.
/// Assert:  in place, the whole error by `assert_eq!`; outside, every
///   factory result `is_ok()`.
/// Collaborators: the pairing, chain-forms, encoding, and hash-to-scalar
///   families' mock concretes through `create_pairing`,
///   `create_chain_forms`, `create_encoding`, and `create_hash_to_scalar`
///   under the default params, and the randomness family's mock concrete
///   under `MockIRandomSourceAdapterFailureMode::FillBytesRefused` through
///   `create_random_source`.
#[test]
fn prove_mint_returns_the_nonce_draw_refusal_unchanged() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter: SchnorrFsDeliveryProof<'_, P, E, F> = build_schnorr_fs_delivery_proof(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let payload = build_prove_mint_payload::<P, F>(pairing, &statement, Default::default());

            // Act
            let result = adapter.prove_mint(ProveMintParams, payload);

            // Assert
            assert_eq!(
                result.err(),
                Some(ProveMintErrorReturn::SchnorrFs(
                    SchnorrFsProveMintErrorReturn::NonceDraw(
                        FillBytesErrorReturn::MockIRandomSourceAdapter,
                    ),
                )),
            );
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides {
            random_source: Some(build_create_random_source_params(
                CreateRandomSourceParamsOverrides {
                    kind: Some(RandomSourceKind::Mock(
                        MockIRandomSourceAdapterFailureMode::FillBytesRefused,
                    )),
                },
            )),
            ..Default::default()
        }),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
}

/// Contract: `prove_mint`, nonce refused: the sampling refuses →
///   `Err(ProveMintErrorReturn::SchnorrFs(SchnorrFsProveMintErrorReturn::
///   NonceSampling(error)))`, the refusal unchanged.
/// Arrange: the pairing's `params` under the default `Succeeds` concrete,
///   because the family's mock concrete arms its sampling refusal
///   thread-locally in `try_new` and an already-armed refusal would fail
///   the fixture builders' own sampling; the both-groups adapter; the
///   default mint statement and payload; then a `create_pairing` under
///   `PairingConcrete::Mock(MockIPairingAdapterFailureMode::
///   SampleScalarWrongLength)` running a consumer that does nothing, whose
///   `try_new` arms the refusal after the fixtures exist.
/// Act:     `prove_mint(ProveMintParams, payload)`.
/// Assert:  in place, the arming `create_pairing`'s `Ok` result and the
///   whole error by `assert_eq!`; outside, every factory result `is_ok()`.
/// Collaborators: the pairing family's mock concrete under the default
///   params through `create_pairing`, and again under
///   `MockIPairingAdapterFailureMode::SampleScalarWrongLength` through a
///   second `create_pairing` whose `try_new` arms the refusal; the
///   chain-forms, encoding, hash-to-scalar, and randomness families' mock
///   concretes through `create_chain_forms`, `create_encoding`,
///   `create_hash_to_scalar`, and `create_random_source` under the
///   default params.
#[test]
fn prove_mint_returns_the_nonce_sampling_refusal_unchanged() {
    struct Consumer;
    struct ArmNonceSamplingRefusal;

    impl IPairingConsumer for ArmNonceSamplingRefusal {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            _payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
        }
    }

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter: SchnorrFsDeliveryProof<'_, P, E, F> = build_schnorr_fs_delivery_proof(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let payload = build_prove_mint_payload::<P, F>(pairing, &statement, Default::default());
            create_pairing(
                &build_create_pairing_deps(ArmNonceSamplingRefusal, Default::default()),
                build_create_pairing_params(CreatePairingParamsOverrides {
                    concrete: Some(PairingConcrete::Mock(
                        MockIPairingAdapterFailureMode::SampleScalarWrongLength,
                    )),
                    ..Default::default()
                }),
                CreatePairingPayload,
            )
            .expect("the refused selection admits the family's mock concrete");

            // Act
            let result = adapter.prove_mint(ProveMintParams, payload);

            // Assert
            assert_eq!(
                result.err(),
                Some(ProveMintErrorReturn::SchnorrFs(
                    SchnorrFsProveMintErrorReturn::NonceSampling(
                        SampleUniformScalarErrorReturn::WrongLength {
                            expected: P::Scalar::UNIFORM_BYTES_LENGTH,
                            actual: P::Scalar::UNIFORM_BYTES_LENGTH,
                        },
                    ),
                )),
            );
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(Default::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
}

/// Contract: `prove_mint`, challenge refused: `challenge` returns
///   `Err(error)` → `Err(ProveMintErrorReturn::SchnorrFs(
///   SchnorrFsProveMintErrorReturn::Challenge(error)))`, the refusal
///   unchanged.
/// Arrange: the adapter's `params` from `build_create_hash_to_scalar_params`
///   with `concrete` `HashToScalarConcrete::Mock(
///   MockIHashToScalarAdapterFailureMode::SamplingRefused)`; the both-groups
///   adapter; the default mint statement and payload.
/// Act:     `prove_mint(ProveMintParams, payload)`.
/// Assert:  in place, the whole error by `assert_eq!`; outside, every
///   factory result `is_ok()`.
/// Collaborators: the pairing, chain-forms, encoding, and randomness
///   families' mock concretes through `create_pairing`, `create_chain_forms`,
///   `create_encoding`, and `create_random_source` under the default params,
///   and the hash-to-scalar family's mock concrete under
///   `MockIHashToScalarAdapterFailureMode::SamplingRefused` through
///   `create_hash_to_scalar`.
#[test]
fn prove_mint_returns_the_challenge_refusal_unchanged() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter: SchnorrFsDeliveryProof<'_, P, E, F> = build_schnorr_fs_delivery_proof(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let payload = build_prove_mint_payload::<P, F>(pairing, &statement, Default::default());

            // Act
            let result = adapter.prove_mint(ProveMintParams, payload);

            // Assert
            assert_eq!(
                result.err(),
                Some(ProveMintErrorReturn::SchnorrFs(
                    SchnorrFsProveMintErrorReturn::Challenge(ChallengeErrorReturn::HashToScalar(
                        HashToScalarErrorReturn::Sampling(
                            SampleUniformScalarErrorReturn::WrongLength {
                                expected: P::Scalar::UNIFORM_BYTES_LENGTH,
                                actual: 0,
                            },
                        ),
                    ),),
                )),
            );
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides {
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
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
}

/// Contract: `prove_mint`, proved: the first messages `T_hpub`, `T_C1`,
///   `T_C2`, `T_D1`, `T_D2` formed from the nonces, the transcript built from
///   the statement and them, the challenge computed over it → the proof's
///   `challenge` is that challenge.
/// Arrange: the both-groups adapter; the default mint statement, whose
///   context values and keys differ at every position, and payload; the
///   expected nonces for `k_α`, `k_r`, `k_ρ`, `k_σ` and the expected first
///   messages and challenge as the expected-values bullet states, so a
///   prover that orders, omits, or alters a first message or a statement
///   field hashes another transcript.
/// Act:     `prove_mint(ProveMintParams, payload)`.
/// Assert:  in place, the result is `Ok(ProveMintSuccessReturn { proof })`,
///   `proof` matches `SchnorrFsProof::MintBothGroups { challenge, .. }`, and
///   the encoding of `challenge` equals the encoding of the expected
///   challenge by `assert_eq!`; outside, every factory result `is_ok()`.
/// Collaborators: the pairing, chain-forms, encoding, hash-to-scalar, and
///   randomness families' mock concretes through `create_pairing`,
///   `create_chain_forms`, `create_encoding`, `create_hash_to_scalar`, and
///   `create_random_source` under the default params.
#[test]
fn prove_mint_returns_the_challenge_over_the_transcript_of_the_relations_first_messages() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter: SchnorrFsDeliveryProof<'_, P, E, F> = build_schnorr_fs_delivery_proof(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let payload = build_prove_mint_payload::<P, F>(pairing, &statement, Default::default());
            let expected = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            )
            .expect("a second source at the defaults yields the sequence the first yields");
            let k_alpha = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload {
                    uniform: expected
                        .adapter
                        .fill_bytes(
                            FillBytesParams,
                            FillBytesPayload {
                                length: P::Scalar::UNIFORM_BYTES_LENGTH,
                            },
                        )
                        .expect("a full-length draw succeeds")
                        .bytes,
                },
            )
            .expect("a full-length uniform samples a scalar")
            .scalar
            .expose()
            .clone();
            let k_r = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload {
                    uniform: expected
                        .adapter
                        .fill_bytes(
                            FillBytesParams,
                            FillBytesPayload {
                                length: P::Scalar::UNIFORM_BYTES_LENGTH,
                            },
                        )
                        .expect("a full-length draw succeeds")
                        .bytes,
                },
            )
            .expect("a full-length uniform samples a scalar")
            .scalar
            .expose()
            .clone();
            let k_rho = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload {
                    uniform: expected
                        .adapter
                        .fill_bytes(
                            FillBytesParams,
                            FillBytesPayload {
                                length: P::Scalar::UNIFORM_BYTES_LENGTH,
                            },
                        )
                        .expect("a full-length draw succeeds")
                        .bytes,
                },
            )
            .expect("a full-length uniform samples a scalar")
            .scalar
            .expose()
            .clone();
            let k_sigma = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload {
                    uniform: expected
                        .adapter
                        .fill_bytes(
                            FillBytesParams,
                            FillBytesPayload {
                                length: P::Scalar::UNIFORM_BYTES_LENGTH,
                            },
                        )
                        .expect("a full-length draw succeeds")
                        .bytes,
                },
            )
            .expect("a full-length uniform samples a scalar")
            .scalar
            .expose()
            .clone();
            let g1 = pairing
                .g1_generator(G1GeneratorParams, G1GeneratorPayload)
                .expect("the first-group generator is admitted")
                .point;
            let g2 = pairing
                .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                .expect("the second-group generator is admitted")
                .point;
            let t_hpub = pairing
                .mul_g2(
                    MulG2Params,
                    MulG2Payload {
                        point: g2.clone(),
                        scalar: k_alpha.clone(),
                    },
                )
                .expect("a generator product succeeds")
                .product;
            let t_c1 = pairing
                .mul_g1(
                    MulG1Params,
                    MulG1Payload {
                        point: g1.clone(),
                        scalar: k_rho.clone(),
                    },
                )
                .expect("a generator product succeeds")
                .product;
            let t_c2 = pairing
                .msm_g1(
                    MsmG1Params,
                    MsmG1Payload {
                        terms: vec![
                            MsmG1Term {
                                base: g1.clone(),
                                scalar: k_alpha.clone(),
                            },
                            MsmG1Term {
                                base: statement.identity_element.clone(),
                                scalar: k_r.clone(),
                            },
                            MsmG1Term {
                                base: statement.buyer_keys.pk1.clone(),
                                scalar: k_rho.clone(),
                            },
                        ],
                    },
                )
                .expect("a generator multi-scalar product succeeds")
                .sum;
            let t_d1 = pairing
                .mul_g2(
                    MulG2Params,
                    MulG2Payload {
                        point: g2.clone(),
                        scalar: k_sigma.clone(),
                    },
                )
                .expect("a generator product succeeds")
                .product;
            let t_d2 = pairing
                .msm_g2(
                    MsmG2Params,
                    MsmG2Payload {
                        terms: vec![
                            MsmG2Term {
                                base: g2.clone(),
                                scalar: k_r.clone(),
                            },
                            MsmG2Term {
                                base: statement.buyer_keys.pk2.clone(),
                                scalar: k_sigma.clone(),
                            },
                        ],
                    },
                )
                .expect("a generator multi-scalar product succeeds")
                .sum;
            let asset_root = Root::try_new(RootConstructorParams {
                bytes: *statement.context.asset_identity_hash.as_bytes(),
            })
            .expect("the asset identity hash is never all zero");
            let transcript = build_mint_statement::<P, F>(
                pairing,
                MintStatementOverrides {
                    suite_identifier: Some(statement.context.suite_identifier.clone()),
                    chain: Some(statement.context.chain.clone()),
                    entitlement_contract: Some(statement.context.entitlement_contract.clone()),
                    asset_root: Some(asset_root),
                    parameter_set_identifier: Some(
                        statement.context.parameter_set_identifier.clone(),
                    ),
                    source_entitlement: Some(statement.context.source_entitlement.clone()),
                    target_entitlement: Some(statement.context.target_entitlement.clone()),
                    old_interval: Some(statement.context.old_interval.clone()),
                    new_interval: Some(statement.context.new_interval.clone()),
                    purpose: Some(statement.context.purpose),
                    seller: Some(statement.context.seller.clone()),
                    buyer: Some(statement.context.buyer.clone()),
                    buyer_keys: Some(PublicKeysComponents {
                        pk1: statement.buyer_keys.pk1.clone(),
                        pk2: statement.buyer_keys.pk2.clone(),
                    }),
                    envelope: Some(EnvelopeComponents {
                        c1: statement.envelope.c1.clone(),
                        c2: statement.envelope.c2.clone(),
                        d1: statement.envelope.d1.clone(),
                        d2: statement.envelope.d2.clone(),
                    }),
                    expiry: Some(statement.context.expiry),
                    first_messages: Some(MintFirstMessages {
                        hpub: t_hpub,
                        c1: t_c1,
                        c2: t_c2,
                        d1: t_d1,
                        d2: t_d2,
                    }),
                },
            );
            let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                bytes: Some(SCHNORR_FS_CHALLENGE_TAG.to_vec()),
            });
            let deps =
                build_challenge_deps(pairing, encoder, &tag, hash_to_scalar, Default::default());
            let expected_challenge = challenge(
                &deps,
                build_challenge_params(Default::default()),
                build_challenge_payload(ChallengeTranscript::Mint(&transcript), Default::default()),
            )
            .expect("the transcript admits a challenge")
            .challenge;

            // Act
            let result = adapter.prove_mint(ProveMintParams, payload);

            // Assert
            let Ok(ProveMintSuccessReturn { proof }) = result else {
                panic!("the mint proof is proved")
            };
            let SchnorrFsProof::MintBothGroups { challenge: c, .. } = proof else {
                panic!("the proof is the mint both-groups variant")
            };
            assert_eq!(
                pairing
                    .encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar: c },)
                    .expect("encoding the proof's challenge succeeds")
                    .bytes
                    .expose()
                    .as_ref(),
                pairing
                    .encode_scalar(
                        EncodeScalarParams,
                        EncodeScalarPayload {
                            scalar: expected_challenge,
                        },
                    )
                    .expect("encoding the expected challenge succeeds")
                    .bytes
                    .expose()
                    .as_ref(),
            );
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(Default::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
}

/// Contract: `prove_mint`, proved: each response is `k + c · w` for its
///   witness in witness order, `α` from `master_scalar.value`, `r` from
///   `credential_randomness`, `ρ` and `σ` from `coins`.
/// Arrange: the both-groups adapter; the default mint statement and
///   payload, whose four witnesses are distinct from each other; the
///   expected nonces; `c` the challenge the proof carries.
/// Act:     `prove_mint(ProveMintParams, payload)`.
/// Assert:  in place, `alpha`, `r`, `rho`, and `sigma` of the proof's
///   `MintResponses` each equal the expected `add_scalar(k, mul_scalar(c,
///   w))` of their own witness by `encode_scalar`, so a swapped witness or a
///   swapped nonce fails; outside, every factory result `is_ok()`.
/// Collaborators: the pairing, chain-forms, encoding, hash-to-scalar, and
///   randomness families' mock concretes through `create_pairing`,
///   `create_chain_forms`, `create_encoding`, `create_hash_to_scalar`, and
///   `create_random_source` under the default params.
#[test]
fn prove_mint_returns_the_responses_the_nonces_and_witnesses_define() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter: SchnorrFsDeliveryProof<'_, P, E, F> = build_schnorr_fs_delivery_proof(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let payload = build_prove_mint_payload::<P, F>(pairing, &statement, Default::default());
            let expected = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            )
            .expect("a second source at the defaults yields the sequence the first yields");
            let k_alpha = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload {
                    uniform: expected
                        .adapter
                        .fill_bytes(
                            FillBytesParams,
                            FillBytesPayload {
                                length: P::Scalar::UNIFORM_BYTES_LENGTH,
                            },
                        )
                        .expect("a full-length draw succeeds")
                        .bytes,
                },
            )
            .expect("a full-length uniform samples a scalar")
            .scalar
            .expose()
            .clone();
            let k_r = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload {
                    uniform: expected
                        .adapter
                        .fill_bytes(
                            FillBytesParams,
                            FillBytesPayload {
                                length: P::Scalar::UNIFORM_BYTES_LENGTH,
                            },
                        )
                        .expect("a full-length draw succeeds")
                        .bytes,
                },
            )
            .expect("a full-length uniform samples a scalar")
            .scalar
            .expose()
            .clone();
            let k_rho = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload {
                    uniform: expected
                        .adapter
                        .fill_bytes(
                            FillBytesParams,
                            FillBytesPayload {
                                length: P::Scalar::UNIFORM_BYTES_LENGTH,
                            },
                        )
                        .expect("a full-length draw succeeds")
                        .bytes,
                },
            )
            .expect("a full-length uniform samples a scalar")
            .scalar
            .expose()
            .clone();
            let k_sigma = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload {
                    uniform: expected
                        .adapter
                        .fill_bytes(
                            FillBytesParams,
                            FillBytesPayload {
                                length: P::Scalar::UNIFORM_BYTES_LENGTH,
                            },
                        )
                        .expect("a full-length draw succeeds")
                        .bytes,
                },
            )
            .expect("a full-length uniform samples a scalar")
            .scalar
            .expose()
            .clone();
            let master_scalar = payload.master_scalar.value.expose().clone();
            let credential_randomness = payload.credential_randomness.expose().clone();
            let rho = payload.coins.rho.expose().clone();
            let sigma = payload.coins.sigma.expose().clone();

            // Act
            let result = adapter.prove_mint(ProveMintParams, payload);

            // Assert
            let Ok(ProveMintSuccessReturn { proof }) = result else {
                panic!("the mint proof is proved")
            };
            let SchnorrFsProof::MintBothGroups {
                challenge: c,
                responses,
            } = proof
            else {
                panic!("the proof is the mint both-groups variant")
            };
            let expected_alpha = pairing
                .add_scalar(
                    AddScalarParams,
                    AddScalarPayload {
                        left: k_alpha,
                        right: pairing
                            .mul_scalar(
                                MulScalarParams,
                                MulScalarPayload {
                                    left: c.clone(),
                                    right: master_scalar,
                                },
                            )
                            .expect("a scalar product succeeds")
                            .product,
                    },
                )
                .expect("a scalar sum succeeds")
                .sum;
            let expected_r = pairing
                .add_scalar(
                    AddScalarParams,
                    AddScalarPayload {
                        left: k_r,
                        right: pairing
                            .mul_scalar(
                                MulScalarParams,
                                MulScalarPayload {
                                    left: c.clone(),
                                    right: credential_randomness,
                                },
                            )
                            .expect("a scalar product succeeds")
                            .product,
                    },
                )
                .expect("a scalar sum succeeds")
                .sum;
            let expected_rho = pairing
                .add_scalar(
                    AddScalarParams,
                    AddScalarPayload {
                        left: k_rho,
                        right: pairing
                            .mul_scalar(
                                MulScalarParams,
                                MulScalarPayload {
                                    left: c.clone(),
                                    right: rho,
                                },
                            )
                            .expect("a scalar product succeeds")
                            .product,
                    },
                )
                .expect("a scalar sum succeeds")
                .sum;
            let expected_sigma = pairing
                .add_scalar(
                    AddScalarParams,
                    AddScalarPayload {
                        left: k_sigma,
                        right: pairing
                            .mul_scalar(
                                MulScalarParams,
                                MulScalarPayload {
                                    left: c.clone(),
                                    right: sigma,
                                },
                            )
                            .expect("a scalar product succeeds")
                            .product,
                    },
                )
                .expect("a scalar sum succeeds")
                .sum;
            let encode_scalar = |scalar: P::Scalar| -> Vec<u8> {
                pairing
                    .encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar })
                    .expect("encoding a scalar succeeds")
                    .bytes
                    .expose()
                    .as_ref()
                    .to_vec()
            };
            assert_eq!(
                encode_scalar(responses.alpha),
                encode_scalar(expected_alpha)
            );
            assert_eq!(encode_scalar(responses.r), encode_scalar(expected_r));
            assert_eq!(encode_scalar(responses.rho), encode_scalar(expected_rho));
            assert_eq!(
                encode_scalar(responses.sigma),
                encode_scalar(expected_sigma)
            );
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(Default::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
}

/// Contract: `prove_mint`, proved, first-group-only form: the proof carries
///   `[T_hpub, T_D1, T_D2]`.
/// Arrange: the adapter constructed with `verifier_form` `FirstGroupOnly`;
///   the default mint statement and payload; the expected `T_hpub`, `T_D1`,
///   and `T_D2` from the expected nonces.
/// Act:     `prove_mint(ProveMintParams, payload)`.
/// Assert:  in place, `proof` matches `SchnorrFsProof::MintFirstGroupOnly {
///   first_messages, .. }` and `hpub`, `d1`, and `d2` of `first_messages`
///   each equal the expected message by `encode_g2`; outside, every factory
///   result `is_ok()`.
/// Collaborators: the pairing, chain-forms, encoding, hash-to-scalar, and
///   randomness families' mock concretes through `create_pairing`,
///   `create_chain_forms`, `create_encoding`, `create_hash_to_scalar`, and
///   `create_random_source` under the default params.
#[test]
fn prove_mint_carries_the_second_group_first_messages_in_the_first_group_only_form() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter: SchnorrFsDeliveryProof<'_, P, E, F> = build_schnorr_fs_delivery_proof(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                SchnorrFsDeliveryProofConstructorParamsOverrides {
                    verifier_form: Some(VerifierGroupArithmetic::FirstGroupOnly),
                },
            );
            let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let payload = build_prove_mint_payload::<P, F>(pairing, &statement, Default::default());
            let expected = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            )
            .expect("a second source at the defaults yields the sequence the first yields");
            let k_alpha = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload {
                    uniform: expected
                        .adapter
                        .fill_bytes(
                            FillBytesParams,
                            FillBytesPayload {
                                length: P::Scalar::UNIFORM_BYTES_LENGTH,
                            },
                        )
                        .expect("a full-length draw succeeds")
                        .bytes,
                },
            )
            .expect("a full-length uniform samples a scalar")
            .scalar
            .expose()
            .clone();
            let k_r = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload {
                    uniform: expected
                        .adapter
                        .fill_bytes(
                            FillBytesParams,
                            FillBytesPayload {
                                length: P::Scalar::UNIFORM_BYTES_LENGTH,
                            },
                        )
                        .expect("a full-length draw succeeds")
                        .bytes,
                },
            )
            .expect("a full-length uniform samples a scalar")
            .scalar
            .expose()
            .clone();
            let _k_rho = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload {
                    uniform: expected
                        .adapter
                        .fill_bytes(
                            FillBytesParams,
                            FillBytesPayload {
                                length: P::Scalar::UNIFORM_BYTES_LENGTH,
                            },
                        )
                        .expect("a full-length draw succeeds")
                        .bytes,
                },
            )
            .expect("a full-length uniform samples a scalar")
            .scalar
            .expose()
            .clone();
            let k_sigma = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload {
                    uniform: expected
                        .adapter
                        .fill_bytes(
                            FillBytesParams,
                            FillBytesPayload {
                                length: P::Scalar::UNIFORM_BYTES_LENGTH,
                            },
                        )
                        .expect("a full-length draw succeeds")
                        .bytes,
                },
            )
            .expect("a full-length uniform samples a scalar")
            .scalar
            .expose()
            .clone();
            let g2 = pairing
                .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                .expect("the second-group generator is admitted")
                .point;
            let expected_hpub = pairing
                .mul_g2(
                    MulG2Params,
                    MulG2Payload {
                        point: g2.clone(),
                        scalar: k_alpha,
                    },
                )
                .expect("a generator product succeeds")
                .product;
            let expected_d1 = pairing
                .mul_g2(
                    MulG2Params,
                    MulG2Payload {
                        point: g2.clone(),
                        scalar: k_sigma.clone(),
                    },
                )
                .expect("a generator product succeeds")
                .product;
            let expected_d2 = pairing
                .msm_g2(
                    MsmG2Params,
                    MsmG2Payload {
                        terms: vec![
                            MsmG2Term {
                                base: g2.clone(),
                                scalar: k_r,
                            },
                            MsmG2Term {
                                base: statement.buyer_keys.pk2.clone(),
                                scalar: k_sigma,
                            },
                        ],
                    },
                )
                .expect("a generator multi-scalar product succeeds")
                .sum;

            // Act
            let result = adapter.prove_mint(ProveMintParams, payload);

            // Assert
            let Ok(ProveMintSuccessReturn { proof }) = result else {
                panic!("the mint proof is proved")
            };
            let SchnorrFsProof::MintFirstGroupOnly { first_messages, .. } = proof else {
                panic!("the proof is the mint first-group-only variant")
            };
            let encode_g2 = |point: P::G2| -> Vec<u8> {
                pairing
                    .encode_g2(EncodeG2Params, EncodeG2Payload { point })
                    .expect("encoding a second-group point succeeds")
                    .bytes
                    .as_ref()
                    .to_vec()
            };
            assert_eq!(encode_g2(first_messages.hpub), encode_g2(expected_hpub));
            assert_eq!(encode_g2(first_messages.d1), encode_g2(expected_d1));
            assert_eq!(encode_g2(first_messages.d2), encode_g2(expected_d2));
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(Default::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
    // prove_mint_carries_the_second_group_first_messages_in_the_first_group_only_form
}

/// Contract: `prove_mint`, proved, both-groups form: the proof carries no
///   transported first messages.
/// Arrange: the both-groups adapter; the default mint statement and
///   payload.
/// Act:     `prove_mint(ProveMintParams, payload)`.
/// Assert:  in place, `proof` matches `SchnorrFsProof::MintBothGroups {
///   .. }`; outside, every factory result `is_ok()`.
/// Collaborators: the pairing, chain-forms, encoding, hash-to-scalar, and
///   randomness families' mock concretes through `create_pairing`,
///   `create_chain_forms`, `create_encoding`, `create_hash_to_scalar`, and
///   `create_random_source` under the default params.
#[test]
fn prove_mint_carries_no_first_messages_in_the_both_groups_form() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter: SchnorrFsDeliveryProof<'_, P, E, F> = build_schnorr_fs_delivery_proof(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let payload = build_prove_mint_payload::<P, F>(pairing, &statement, Default::default());

            // Act
            let result = adapter.prove_mint(ProveMintParams, payload);

            // Assert
            let Ok(ProveMintSuccessReturn { proof }) = result else {
                panic!("the mint proof is proved")
            };
            assert!(matches!(proof, SchnorrFsProof::MintBothGroups { .. }));
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(Default::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
    // prove_mint_carries_no_first_messages_in_the_both_groups_form
}

/// Contract: `prove_transfer`, nonce refused: the draw refuses →
///   `Err(ProveTransferErrorReturn::SchnorrFs(
///   SchnorrFsProveTransferErrorReturn::NonceDraw(error)))`, the refusal
///   unchanged.
/// Arrange: the source's `params` with `RandomSourceKind::Mock(
///   MockIRandomSourceAdapterFailureMode::FillBytesRefused)`; the
///   both-groups adapter; the default transfer statement and payload.
/// Act:     `prove_transfer(ProveTransferParams, payload)`.
/// Assert:  in place, the whole error by `assert_eq!`; outside, every
///   factory result `is_ok()`.
/// Collaborators: the pairing, chain-forms, encoding, and hash-to-scalar
///   families' mock concretes through `create_pairing`, `create_chain_forms`,
///   `create_encoding`, and `create_hash_to_scalar` under the default params,
///   and the randomness family's mock concrete under
///   `MockIRandomSourceAdapterFailureMode::FillBytesRefused` through
///   `create_random_source`.
#[test]
fn prove_transfer_returns_the_nonce_draw_refusal_unchanged() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter: SchnorrFsDeliveryProof<'_, P, E, F> = build_schnorr_fs_delivery_proof(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let statement = build_algebraic_transfer_statement::<P, F>(pairing, Default::default());
            let payload =
                build_prove_transfer_payload::<P, F>(pairing, &statement, Default::default());

            // Act
            let result = adapter.prove_transfer(ProveTransferParams, payload);

            // Assert
            assert_eq!(
                result.err(),
                Some(ProveTransferErrorReturn::SchnorrFs(
                    SchnorrFsProveTransferErrorReturn::NonceDraw(
                        FillBytesErrorReturn::MockIRandomSourceAdapter,
                    ),
                )),
            );
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides {
            random_source: Some(build_create_random_source_params(
                CreateRandomSourceParamsOverrides {
                    kind: Some(RandomSourceKind::Mock(
                        MockIRandomSourceAdapterFailureMode::FillBytesRefused,
                    )),
                },
            )),
            ..Default::default()
        }),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
    // prove_transfer_returns_the_nonce_draw_refusal_unchanged
}

/// Contract: `prove_transfer`, nonce refused: the sampling refuses →
///   `Err(ProveTransferErrorReturn::SchnorrFs(
///   SchnorrFsProveTransferErrorReturn::NonceSampling(error)))`, the refusal
///   unchanged.
/// Arrange: the pairing's `params` under the default `Succeeds` concrete,
///   because the family's mock concrete arms its sampling refusal
///   thread-locally in `try_new` and an already-armed refusal would fail
///   the fixture builders' own sampling; the both-groups adapter; the
///   default transfer statement and payload; then a `create_pairing` under
///   `PairingConcrete::Mock(MockIPairingAdapterFailureMode::
///   SampleScalarWrongLength)` running a consumer that does nothing, whose
///   `try_new` arms the refusal after the fixtures exist.
/// Act:     `prove_transfer(ProveTransferParams, payload)`.
/// Assert:  in place, the arming `create_pairing`'s `Ok` result and the
///   whole error by `assert_eq!`; outside, every factory result `is_ok()`.
/// Collaborators: the pairing family's mock concrete under the default
///   params through `create_pairing`, and again under
///   `MockIPairingAdapterFailureMode::SampleScalarWrongLength` through a
///   second `create_pairing` whose `try_new` arms the refusal; the
///   chain-forms, encoding, hash-to-scalar, and randomness families' mock
///   concretes through `create_chain_forms`, `create_encoding`,
///   `create_hash_to_scalar`, and `create_random_source` under the
///   default params.
#[test]
fn prove_transfer_returns_the_nonce_sampling_refusal_unchanged() {
    struct Consumer;
    struct ArmNonceSamplingRefusal;

    impl IPairingConsumer for ArmNonceSamplingRefusal {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            _payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
        }
    }

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter: SchnorrFsDeliveryProof<'_, P, E, F> = build_schnorr_fs_delivery_proof(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let statement = build_algebraic_transfer_statement::<P, F>(pairing, Default::default());
            let payload =
                build_prove_transfer_payload::<P, F>(pairing, &statement, Default::default());
            create_pairing(
                &build_create_pairing_deps(ArmNonceSamplingRefusal, Default::default()),
                build_create_pairing_params(CreatePairingParamsOverrides {
                    concrete: Some(PairingConcrete::Mock(
                        MockIPairingAdapterFailureMode::SampleScalarWrongLength,
                    )),
                    ..Default::default()
                }),
                CreatePairingPayload,
            )
            .expect("the refused selection admits the family's mock concrete");

            // Act
            let result = adapter.prove_transfer(ProveTransferParams, payload);

            // Assert
            assert_eq!(
                result.err(),
                Some(ProveTransferErrorReturn::SchnorrFs(
                    SchnorrFsProveTransferErrorReturn::NonceSampling(
                        SampleUniformScalarErrorReturn::WrongLength {
                            expected: P::Scalar::UNIFORM_BYTES_LENGTH,
                            actual: P::Scalar::UNIFORM_BYTES_LENGTH,
                        },
                    ),
                )),
            );
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(Default::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
    // prove_transfer_returns_the_nonce_sampling_refusal_unchanged
}

/// Contract: `prove_transfer`, challenge refused: `challenge` returns
///   `Err(error)` → `Err(ProveTransferErrorReturn::SchnorrFs(
///   SchnorrFsProveTransferErrorReturn::Challenge(error)))`, the refusal
///   unchanged.
/// Arrange: the adapter's `params` with `HashToScalarConcrete::Mock(
///   MockIHashToScalarAdapterFailureMode::SamplingRefused)`; the
///   both-groups adapter; the default transfer statement and payload.
/// Act:     `prove_transfer(ProveTransferParams, payload)`.
/// Assert:  in place, the whole error by `assert_eq!`; outside, every
///   factory result `is_ok()`.
/// Collaborators: the pairing, chain-forms, encoding, and randomness
///   families' mock concretes through `create_pairing`, `create_chain_forms`,
///   `create_encoding`, and `create_random_source` under the default params,
///   and the hash-to-scalar family's mock concrete under
///   `MockIHashToScalarAdapterFailureMode::SamplingRefused` through
///   `create_hash_to_scalar`.
#[test]
fn prove_transfer_returns_the_challenge_refusal_unchanged() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter: SchnorrFsDeliveryProof<'_, P, E, F> = build_schnorr_fs_delivery_proof(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let statement = build_algebraic_transfer_statement::<P, F>(pairing, Default::default());
            let payload =
                build_prove_transfer_payload::<P, F>(pairing, &statement, Default::default());

            // Act
            let result = adapter.prove_transfer(ProveTransferParams, payload);

            // Assert
            assert_eq!(
                result.err(),
                Some(ProveTransferErrorReturn::SchnorrFs(
                    SchnorrFsProveTransferErrorReturn::Challenge(
                        ChallengeErrorReturn::HashToScalar(HashToScalarErrorReturn::Sampling(
                            SampleUniformScalarErrorReturn::WrongLength {
                                expected: P::Scalar::UNIFORM_BYTES_LENGTH,
                                actual: 0,
                            },
                        ),),
                    ),
                )),
            );
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides {
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
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
    // prove_transfer_returns_the_challenge_refusal_unchanged
}

/// Contract: `prove_transfer`, proved: the first messages `T_pk1`, `T_C1`,
///   `T_C2`, `T_pk2`, `T_D1`, `T_D2` formed from the nonces, the transcript
///   built from the statement and them, the challenge computed over it →
///   the proof's `challenge` is that challenge.
/// Arrange: the both-groups adapter; the transfer statement
///   `build_algebraic_transfer_statement` with its context purpose
///   `TransferPurpose::Grant`, the purpose the builder does not default to,
///   and the default payload; the expected nonces for `k_x`, `k_y`, `k_s`,
///   `k_ρ`, `k_σ`, the expected first messages with `neg_scalar` of `k_x`
///   and `k_y` on the old envelope's `C1_o` and `D1_o`, and the expected
///   challenge as the expected-values bullet states.
/// Act:     `prove_transfer(ProveTransferParams, payload)`.
/// Assert:  in place, `proof` matches `SchnorrFsProof::TransferBothGroups {
///   challenge, .. }` and the encoding of `challenge` equals the encoding
///   of the expected challenge by `assert_eq!`; outside, every factory
///   result `is_ok()`.
/// Collaborators: the pairing, chain-forms, encoding, hash-to-scalar, and
///   randomness families' mock concretes through `create_pairing`,
///   `create_chain_forms`, `create_encoding`, `create_hash_to_scalar`, and
///   `create_random_source` under the default params.
#[test]
fn prove_transfer_returns_the_challenge_over_the_transcript_of_the_relations_first_messages() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter: SchnorrFsDeliveryProof<'_, P, E, F> = build_schnorr_fs_delivery_proof(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let statement = build_algebraic_transfer_statement::<P, F>(
                pairing,
                AlgebraicTransferStatementOverrides {
                    context: Some(build_delivery_context(
                        TransferPurpose::Grant,
                        Default::default(),
                    )),
                    ..Default::default()
                },
            );
            let payload =
                build_prove_transfer_payload::<P, F>(pairing, &statement, Default::default());
            let expected = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            )
            .expect("a second source at the defaults yields the sequence the first yields");
            let k_x = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload {
                    uniform: expected
                        .adapter
                        .fill_bytes(
                            FillBytesParams,
                            FillBytesPayload {
                                length: P::Scalar::UNIFORM_BYTES_LENGTH,
                            },
                        )
                        .expect("a full-length draw succeeds")
                        .bytes,
                },
            )
            .expect("a full-length uniform samples a scalar")
            .scalar
            .expose()
            .clone();
            let k_y = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload {
                    uniform: expected
                        .adapter
                        .fill_bytes(
                            FillBytesParams,
                            FillBytesPayload {
                                length: P::Scalar::UNIFORM_BYTES_LENGTH,
                            },
                        )
                        .expect("a full-length draw succeeds")
                        .bytes,
                },
            )
            .expect("a full-length uniform samples a scalar")
            .scalar
            .expose()
            .clone();
            let k_s = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload {
                    uniform: expected
                        .adapter
                        .fill_bytes(
                            FillBytesParams,
                            FillBytesPayload {
                                length: P::Scalar::UNIFORM_BYTES_LENGTH,
                            },
                        )
                        .expect("a full-length draw succeeds")
                        .bytes,
                },
            )
            .expect("a full-length uniform samples a scalar")
            .scalar
            .expose()
            .clone();
            let k_rho = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload {
                    uniform: expected
                        .adapter
                        .fill_bytes(
                            FillBytesParams,
                            FillBytesPayload {
                                length: P::Scalar::UNIFORM_BYTES_LENGTH,
                            },
                        )
                        .expect("a full-length draw succeeds")
                        .bytes,
                },
            )
            .expect("a full-length uniform samples a scalar")
            .scalar
            .expose()
            .clone();
            let k_sigma = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload {
                    uniform: expected
                        .adapter
                        .fill_bytes(
                            FillBytesParams,
                            FillBytesPayload {
                                length: P::Scalar::UNIFORM_BYTES_LENGTH,
                            },
                        )
                        .expect("a full-length draw succeeds")
                        .bytes,
                },
            )
            .expect("a full-length uniform samples a scalar")
            .scalar
            .expose()
            .clone();
            let g1 = pairing
                .g1_generator(G1GeneratorParams, G1GeneratorPayload)
                .expect("the first-group generator is admitted")
                .point;
            let g2 = pairing
                .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                .expect("the second-group generator is admitted")
                .point;
            let t_pk1 = pairing
                .mul_g1(
                    MulG1Params,
                    MulG1Payload {
                        point: g1.clone(),
                        scalar: k_x.clone(),
                    },
                )
                .expect("a generator product succeeds")
                .product;
            let t_c1 = pairing
                .mul_g1(
                    MulG1Params,
                    MulG1Payload {
                        point: g1.clone(),
                        scalar: k_rho.clone(),
                    },
                )
                .expect("a generator product succeeds")
                .product;
            let t_c2 = pairing
                .msm_g1(
                    MsmG1Params,
                    MsmG1Payload {
                        terms: vec![
                            MsmG1Term {
                                base: statement.old_envelope.c1.clone(),
                                scalar: pairing
                                    .neg_scalar(
                                        NegScalarParams,
                                        NegScalarPayload {
                                            scalar: k_x.clone(),
                                        },
                                    )
                                    .expect("a scalar negation succeeds")
                                    .negation,
                            },
                            MsmG1Term {
                                base: statement.identity_element.clone(),
                                scalar: k_s.clone(),
                            },
                            MsmG1Term {
                                base: statement.buyer_keys.pk1.clone(),
                                scalar: k_rho.clone(),
                            },
                        ],
                    },
                )
                .expect("a generator multi-scalar product succeeds")
                .sum;
            let t_pk2 = pairing
                .mul_g2(
                    MulG2Params,
                    MulG2Payload {
                        point: g2.clone(),
                        scalar: k_y.clone(),
                    },
                )
                .expect("a generator product succeeds")
                .product;
            let t_d1 = pairing
                .mul_g2(
                    MulG2Params,
                    MulG2Payload {
                        point: g2.clone(),
                        scalar: k_sigma.clone(),
                    },
                )
                .expect("a generator product succeeds")
                .product;
            let t_d2 = pairing
                .msm_g2(
                    MsmG2Params,
                    MsmG2Payload {
                        terms: vec![
                            MsmG2Term {
                                base: statement.old_envelope.d1.clone(),
                                scalar: pairing
                                    .neg_scalar(
                                        NegScalarParams,
                                        NegScalarPayload {
                                            scalar: k_y.clone(),
                                        },
                                    )
                                    .expect("a scalar negation succeeds")
                                    .negation,
                            },
                            MsmG2Term {
                                base: g2.clone(),
                                scalar: k_s.clone(),
                            },
                            MsmG2Term {
                                base: statement.buyer_keys.pk2.clone(),
                                scalar: k_sigma.clone(),
                            },
                        ],
                    },
                )
                .expect("a generator multi-scalar product succeeds")
                .sum;
            let asset_root = Root::try_new(RootConstructorParams {
                bytes: *statement.context.asset_identity_hash.as_bytes(),
            })
            .expect("the asset identity hash is never all zero");
            let transcript = build_transfer_statement::<P, F>(
                pairing,
                TransferStatementOverrides {
                    suite_identifier: Some(statement.context.suite_identifier.clone()),
                    chain: Some(statement.context.chain.clone()),
                    entitlement_contract: Some(statement.context.entitlement_contract.clone()),
                    asset_root: Some(asset_root),
                    parameter_set_identifier: Some(
                        statement.context.parameter_set_identifier.clone(),
                    ),
                    source_entitlement: Some(statement.context.source_entitlement.clone()),
                    target_entitlement: Some(statement.context.target_entitlement.clone()),
                    old_interval: Some(statement.context.old_interval.clone()),
                    new_interval: Some(statement.context.new_interval.clone()),
                    purpose: Some(statement.context.purpose),
                    seller: Some(statement.context.seller.clone()),
                    buyer: Some(statement.context.buyer.clone()),
                    seller_keys: Some(PublicKeysComponents {
                        pk1: statement.seller_keys.pk1.clone(),
                        pk2: statement.seller_keys.pk2.clone(),
                    }),
                    buyer_keys: Some(PublicKeysComponents {
                        pk1: statement.buyer_keys.pk1.clone(),
                        pk2: statement.buyer_keys.pk2.clone(),
                    }),
                    old_envelope: Some(EnvelopeComponents {
                        c1: statement.old_envelope.c1.clone(),
                        c2: statement.old_envelope.c2.clone(),
                        d1: statement.old_envelope.d1.clone(),
                        d2: statement.old_envelope.d2.clone(),
                    }),
                    new_envelope: Some(EnvelopeComponents {
                        c1: statement.new_envelope.c1.clone(),
                        c2: statement.new_envelope.c2.clone(),
                        d1: statement.new_envelope.d1.clone(),
                        d2: statement.new_envelope.d2.clone(),
                    }),
                    expiry: Some(statement.context.expiry),
                    first_messages: Some(TransferFirstMessages {
                        pk1: t_pk1,
                        c1: t_c1,
                        c2: t_c2,
                        pk2: t_pk2,
                        d1: t_d1,
                        d2: t_d2,
                    }),
                },
            );
            let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                bytes: Some(SCHNORR_FS_CHALLENGE_TAG.to_vec()),
            });
            let deps =
                build_challenge_deps(pairing, encoder, &tag, hash_to_scalar, Default::default());
            let expected_challenge = challenge(
                &deps,
                build_challenge_params(Default::default()),
                build_challenge_payload(
                    ChallengeTranscript::Transfer(&transcript),
                    Default::default(),
                ),
            )
            .expect("the transcript admits a challenge")
            .challenge;

            // Act
            let result = adapter.prove_transfer(ProveTransferParams, payload);

            // Assert
            let Ok(ProveTransferSuccessReturn { proof }) = result else {
                panic!("the transfer proof is proved")
            };
            let SchnorrFsProof::TransferBothGroups { challenge: c, .. } = proof else {
                panic!("the proof is the transfer both-groups variant")
            };
            assert_eq!(
                pairing
                    .encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar: c },)
                    .expect("encoding the proof's challenge succeeds")
                    .bytes
                    .expose()
                    .as_ref(),
                pairing
                    .encode_scalar(
                        EncodeScalarParams,
                        EncodeScalarPayload {
                            scalar: expected_challenge,
                        },
                    )
                    .expect("encoding the expected challenge succeeds")
                    .bytes
                    .expose()
                    .as_ref(),
            );
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(Default::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
    // prove_transfer_returns_the_challenge_over_the_transcript_of_the_relations_first_messages
}

/// Contract: `prove_transfer`, proved: each response is `k + c · w` for its
///   witness in witness order, `x_s` and `y_s` from `seller_secrets`, `s`
///   from `offset`, `ρ` and `σ` from `coins`.
/// Arrange: the both-groups adapter; the default transfer statement and
///   payload, whose five witnesses are distinct from each other; the
///   expected nonces; `c` the challenge the proof carries.
/// Act:     `prove_transfer(ProveTransferParams, payload)`.
/// Assert:  in place, `x`, `y`, `s`, `rho`, and `sigma` of the proof's
///   `TransferResponses` each equal the expected `add_scalar(k,
///   mul_scalar(c, w))` of their own witness by `encode_scalar`; outside,
///   every factory result `is_ok()`.
/// Collaborators: the pairing, chain-forms, encoding, hash-to-scalar, and
///   randomness families' mock concretes through `create_pairing`,
///   `create_chain_forms`, `create_encoding`, `create_hash_to_scalar`, and
///   `create_random_source` under the default params.
#[test]
fn prove_transfer_returns_the_responses_the_nonces_and_witnesses_define() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter: SchnorrFsDeliveryProof<'_, P, E, F> = build_schnorr_fs_delivery_proof(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let statement = build_algebraic_transfer_statement::<P, F>(pairing, Default::default());
            let payload =
                build_prove_transfer_payload::<P, F>(pairing, &statement, Default::default());
            let expected = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            )
            .expect("a second source at the defaults yields the sequence the first yields");
            let k_x = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload {
                    uniform: expected
                        .adapter
                        .fill_bytes(
                            FillBytesParams,
                            FillBytesPayload {
                                length: P::Scalar::UNIFORM_BYTES_LENGTH,
                            },
                        )
                        .expect("a full-length draw succeeds")
                        .bytes,
                },
            )
            .expect("a full-length uniform samples a scalar")
            .scalar
            .expose()
            .clone();
            let k_y = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload {
                    uniform: expected
                        .adapter
                        .fill_bytes(
                            FillBytesParams,
                            FillBytesPayload {
                                length: P::Scalar::UNIFORM_BYTES_LENGTH,
                            },
                        )
                        .expect("a full-length draw succeeds")
                        .bytes,
                },
            )
            .expect("a full-length uniform samples a scalar")
            .scalar
            .expose()
            .clone();
            let k_s = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload {
                    uniform: expected
                        .adapter
                        .fill_bytes(
                            FillBytesParams,
                            FillBytesPayload {
                                length: P::Scalar::UNIFORM_BYTES_LENGTH,
                            },
                        )
                        .expect("a full-length draw succeeds")
                        .bytes,
                },
            )
            .expect("a full-length uniform samples a scalar")
            .scalar
            .expose()
            .clone();
            let k_rho = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload {
                    uniform: expected
                        .adapter
                        .fill_bytes(
                            FillBytesParams,
                            FillBytesPayload {
                                length: P::Scalar::UNIFORM_BYTES_LENGTH,
                            },
                        )
                        .expect("a full-length draw succeeds")
                        .bytes,
                },
            )
            .expect("a full-length uniform samples a scalar")
            .scalar
            .expose()
            .clone();
            let k_sigma = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload {
                    uniform: expected
                        .adapter
                        .fill_bytes(
                            FillBytesParams,
                            FillBytesPayload {
                                length: P::Scalar::UNIFORM_BYTES_LENGTH,
                            },
                        )
                        .expect("a full-length draw succeeds")
                        .bytes,
                },
            )
            .expect("a full-length uniform samples a scalar")
            .scalar
            .expose()
            .clone();
            let x_secret = payload.seller_secrets.x.expose().clone();
            let y_secret = payload.seller_secrets.y.expose().clone();
            let offset = payload.offset.expose().clone();
            let rho = payload.coins.rho.expose().clone();
            let sigma = payload.coins.sigma.expose().clone();

            // Act
            let result = adapter.prove_transfer(ProveTransferParams, payload);

            // Assert
            let Ok(ProveTransferSuccessReturn { proof }) = result else {
                panic!("the transfer proof is proved")
            };
            let SchnorrFsProof::TransferBothGroups {
                challenge: c,
                responses,
            } = proof
            else {
                panic!("the proof is the transfer both-groups variant")
            };
            let expected_x = pairing
                .add_scalar(
                    AddScalarParams,
                    AddScalarPayload {
                        left: k_x,
                        right: pairing
                            .mul_scalar(
                                MulScalarParams,
                                MulScalarPayload {
                                    left: c.clone(),
                                    right: x_secret,
                                },
                            )
                            .expect("a scalar product succeeds")
                            .product,
                    },
                )
                .expect("a scalar sum succeeds")
                .sum;
            let expected_y = pairing
                .add_scalar(
                    AddScalarParams,
                    AddScalarPayload {
                        left: k_y,
                        right: pairing
                            .mul_scalar(
                                MulScalarParams,
                                MulScalarPayload {
                                    left: c.clone(),
                                    right: y_secret,
                                },
                            )
                            .expect("a scalar product succeeds")
                            .product,
                    },
                )
                .expect("a scalar sum succeeds")
                .sum;
            let expected_s = pairing
                .add_scalar(
                    AddScalarParams,
                    AddScalarPayload {
                        left: k_s,
                        right: pairing
                            .mul_scalar(
                                MulScalarParams,
                                MulScalarPayload {
                                    left: c.clone(),
                                    right: offset,
                                },
                            )
                            .expect("a scalar product succeeds")
                            .product,
                    },
                )
                .expect("a scalar sum succeeds")
                .sum;
            let expected_rho = pairing
                .add_scalar(
                    AddScalarParams,
                    AddScalarPayload {
                        left: k_rho,
                        right: pairing
                            .mul_scalar(
                                MulScalarParams,
                                MulScalarPayload {
                                    left: c.clone(),
                                    right: rho,
                                },
                            )
                            .expect("a scalar product succeeds")
                            .product,
                    },
                )
                .expect("a scalar sum succeeds")
                .sum;
            let expected_sigma = pairing
                .add_scalar(
                    AddScalarParams,
                    AddScalarPayload {
                        left: k_sigma,
                        right: pairing
                            .mul_scalar(
                                MulScalarParams,
                                MulScalarPayload {
                                    left: c.clone(),
                                    right: sigma,
                                },
                            )
                            .expect("a scalar product succeeds")
                            .product,
                    },
                )
                .expect("a scalar sum succeeds")
                .sum;
            let encode_scalar = |scalar: P::Scalar| -> Vec<u8> {
                pairing
                    .encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar })
                    .expect("encoding a scalar succeeds")
                    .bytes
                    .expose()
                    .as_ref()
                    .to_vec()
            };
            assert_eq!(encode_scalar(responses.x), encode_scalar(expected_x));
            assert_eq!(encode_scalar(responses.y), encode_scalar(expected_y));
            assert_eq!(encode_scalar(responses.s), encode_scalar(expected_s));
            assert_eq!(encode_scalar(responses.rho), encode_scalar(expected_rho));
            assert_eq!(
                encode_scalar(responses.sigma),
                encode_scalar(expected_sigma)
            );
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(Default::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
    // prove_transfer_returns_the_responses_the_nonces_and_witnesses_define
}

/// Contract: `prove_transfer`, proved, first-group-only form: the proof
///   carries `[T_pk2, T_D1, T_D2]`.
/// Arrange: the adapter constructed with `verifier_form` `FirstGroupOnly`;
///   the default transfer statement and payload; the expected `T_pk2`,
///   `T_D1`, and `T_D2` from the expected nonces.
/// Act:     `prove_transfer(ProveTransferParams, payload)`.
/// Assert:  in place, `proof` matches `SchnorrFsProof::TransferFirstGroupOnly
///   { first_messages, .. }` and each of the three fields of
///   `first_messages` equals the expected message at its position by
///   `encode_g2`; outside, every factory result `is_ok()`.
/// Collaborators: the pairing, chain-forms, encoding, hash-to-scalar, and
///   randomness families' mock concretes through `create_pairing`,
///   `create_chain_forms`, `create_encoding`, `create_hash_to_scalar`, and
///   `create_random_source` under the default params.
#[test]
fn prove_transfer_carries_the_second_group_first_messages_in_the_first_group_only_form() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter: SchnorrFsDeliveryProof<'_, P, E, F> = build_schnorr_fs_delivery_proof(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                SchnorrFsDeliveryProofConstructorParamsOverrides {
                    verifier_form: Some(VerifierGroupArithmetic::FirstGroupOnly),
                },
            );
            let statement = build_algebraic_transfer_statement::<P, F>(pairing, Default::default());
            let payload =
                build_prove_transfer_payload::<P, F>(pairing, &statement, Default::default());
            let expected = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            )
            .expect("a second source at the defaults yields the sequence the first yields");
            let _k_x = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload {
                    uniform: expected
                        .adapter
                        .fill_bytes(
                            FillBytesParams,
                            FillBytesPayload {
                                length: P::Scalar::UNIFORM_BYTES_LENGTH,
                            },
                        )
                        .expect("a full-length draw succeeds")
                        .bytes,
                },
            )
            .expect("a full-length uniform samples a scalar")
            .scalar
            .expose()
            .clone();
            let k_y = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload {
                    uniform: expected
                        .adapter
                        .fill_bytes(
                            FillBytesParams,
                            FillBytesPayload {
                                length: P::Scalar::UNIFORM_BYTES_LENGTH,
                            },
                        )
                        .expect("a full-length draw succeeds")
                        .bytes,
                },
            )
            .expect("a full-length uniform samples a scalar")
            .scalar
            .expose()
            .clone();
            let k_s = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload {
                    uniform: expected
                        .adapter
                        .fill_bytes(
                            FillBytesParams,
                            FillBytesPayload {
                                length: P::Scalar::UNIFORM_BYTES_LENGTH,
                            },
                        )
                        .expect("a full-length draw succeeds")
                        .bytes,
                },
            )
            .expect("a full-length uniform samples a scalar")
            .scalar
            .expose()
            .clone();
            let _k_rho = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload {
                    uniform: expected
                        .adapter
                        .fill_bytes(
                            FillBytesParams,
                            FillBytesPayload {
                                length: P::Scalar::UNIFORM_BYTES_LENGTH,
                            },
                        )
                        .expect("a full-length draw succeeds")
                        .bytes,
                },
            )
            .expect("a full-length uniform samples a scalar")
            .scalar
            .expose()
            .clone();
            let k_sigma = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload {
                    uniform: expected
                        .adapter
                        .fill_bytes(
                            FillBytesParams,
                            FillBytesPayload {
                                length: P::Scalar::UNIFORM_BYTES_LENGTH,
                            },
                        )
                        .expect("a full-length draw succeeds")
                        .bytes,
                },
            )
            .expect("a full-length uniform samples a scalar")
            .scalar
            .expose()
            .clone();
            let g2 = pairing
                .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                .expect("the second-group generator is admitted")
                .point;
            let expected_pk2 = pairing
                .mul_g2(
                    MulG2Params,
                    MulG2Payload {
                        point: g2.clone(),
                        scalar: k_y.clone(),
                    },
                )
                .expect("a generator product succeeds")
                .product;
            let expected_d1 = pairing
                .mul_g2(
                    MulG2Params,
                    MulG2Payload {
                        point: g2.clone(),
                        scalar: k_sigma.clone(),
                    },
                )
                .expect("a generator product succeeds")
                .product;
            let expected_d2 = pairing
                .msm_g2(
                    MsmG2Params,
                    MsmG2Payload {
                        terms: vec![
                            MsmG2Term {
                                base: statement.old_envelope.d1.clone(),
                                scalar: pairing
                                    .neg_scalar(NegScalarParams, NegScalarPayload { scalar: k_y })
                                    .expect("a scalar negation succeeds")
                                    .negation,
                            },
                            MsmG2Term {
                                base: g2.clone(),
                                scalar: k_s,
                            },
                            MsmG2Term {
                                base: statement.buyer_keys.pk2.clone(),
                                scalar: k_sigma,
                            },
                        ],
                    },
                )
                .expect("a generator multi-scalar product succeeds")
                .sum;

            // Act
            let result = adapter.prove_transfer(ProveTransferParams, payload);

            // Assert
            let Ok(ProveTransferSuccessReturn { proof }) = result else {
                panic!("the transfer proof is proved")
            };
            let SchnorrFsProof::TransferFirstGroupOnly { first_messages, .. } = proof else {
                panic!("the proof is the transfer first-group-only variant")
            };
            let encode_g2 = |point: P::G2| -> Vec<u8> {
                pairing
                    .encode_g2(EncodeG2Params, EncodeG2Payload { point })
                    .expect("encoding a second-group point succeeds")
                    .bytes
                    .as_ref()
                    .to_vec()
            };
            assert_eq!(encode_g2(first_messages.pk2), encode_g2(expected_pk2));
            assert_eq!(encode_g2(first_messages.d1), encode_g2(expected_d1));
            assert_eq!(encode_g2(first_messages.d2), encode_g2(expected_d2));
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(Default::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
    // prove_transfer_carries_the_second_group_first_messages_in_the_first_group_only_form
}

/// Contract: `prove_transfer`, proved, both-groups form: the proof is the
///   both-groups variant, holding the challenge and the responses alone.
/// Arrange: the adapter constructed with `verifier_form` `BothGroups`; the
///   default transfer statement and payload.
/// Act:     `prove_transfer(ProveTransferParams, payload)`.
/// Assert:  in place, `matches!(proof, SchnorrFsProof::TransferBothGroups {
///   .. })`; outside, every factory result `is_ok()`.
/// Collaborators: the pairing, chain-forms, encoding, hash-to-scalar, and
///   randomness families' mock concretes through `create_pairing`,
///   `create_chain_forms`, `create_encoding`, `create_hash_to_scalar`, and
///   `create_random_source` under the default params.
#[test]
fn prove_transfer_carries_no_first_messages_in_the_both_groups_form() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter: SchnorrFsDeliveryProof<'_, P, E, F> = build_schnorr_fs_delivery_proof(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                SchnorrFsDeliveryProofConstructorParamsOverrides {
                    verifier_form: Some(VerifierGroupArithmetic::BothGroups),
                },
            );
            let statement = build_algebraic_transfer_statement::<P, F>(pairing, Default::default());
            let payload =
                build_prove_transfer_payload::<P, F>(pairing, &statement, Default::default());

            // Act
            let result = adapter.prove_transfer(ProveTransferParams, payload);

            // Assert
            let Ok(ProveTransferSuccessReturn { proof }) = result else {
                panic!("the transfer proof is proved")
            };
            assert!(matches!(proof, SchnorrFsProof::TransferBothGroups { .. }));
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(Default::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
    // prove_transfer_carries_no_first_messages_in_the_both_groups_form
}

/// Contract: `verify` rejects a proof variant whose relation differs from
///   its statement → `Ok(VerifySuccessReturn { is_valid: false })`.
/// Arrange: the both-groups adapter; the default transfer statement and
///   payload, and `build_schnorr_fs_transfer_proof` over them; the default
///   mint statement.
/// Act:     `verify(VerifyParams, payload)` over
///   `AlgebraicStatement::Mint(&mint_statement)` and the transfer proof.
/// Assert:  in place, `result.ok().map(|success| success.is_valid)` equals
///   `Some(false)` by `assert_eq!`; outside, every factory result `is_ok()`.
/// Collaborators: the pairing, chain-forms, encoding, hash-to-scalar, and
///   randomness families' mock concretes through `create_pairing`,
///   `create_chain_forms`, `create_encoding`, `create_hash_to_scalar`, and
///   `create_random_source` under the default params.
#[test]
fn verify_returns_false_for_a_proof_of_the_other_relation() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter: SchnorrFsDeliveryProof<'_, P, E, F> = build_schnorr_fs_delivery_proof(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let transfer_statement =
                build_algebraic_transfer_statement::<P, F>(pairing, Default::default());
            let transfer_payload = build_prove_transfer_payload::<P, F>(
                pairing,
                &transfer_statement,
                Default::default(),
            );
            let proof =
                build_schnorr_fs_transfer_proof(&adapter, &transfer_payload, Default::default());
            let mint_statement =
                build_algebraic_mint_statement::<P, F>(pairing, Default::default());

            // Act
            let result = adapter.verify(
                VerifyParams,
                build_verify_payload(
                    AlgebraicStatement::Mint(&mint_statement),
                    &proof,
                    Default::default(),
                ),
            );

            // Assert
            assert_eq!(result.ok().map(|success| success.is_valid), Some(false));
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(Default::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
    // verify_returns_false_for_a_proof_of_the_other_relation
}

/// Contract: `verify` rejects a proof variant whose verifier form differs
///   from the adapter → `Ok(VerifySuccessReturn { is_valid: false })`.
/// Arrange: the subject's adapter constructed in the both-groups form; a
///   second adapter over the same collaborators constructed in the
///   first-group-only form; the default mint statement and payload; the
///   proof from `build_schnorr_fs_mint_proof` over the second adapter.
/// Act:     `verify(VerifyParams, payload)` on the subject's adapter over
///   `AlgebraicStatement::Mint(&statement)` and that proof.
/// Assert:  in place, `result.ok().map(|success| success.is_valid)` equals
///   `Some(false)` by `assert_eq!`; outside, every factory result `is_ok()`.
/// Collaborators: the pairing, chain-forms, encoding, hash-to-scalar, and
///   randomness families' mock concretes through `create_pairing`,
///   `create_chain_forms`, `create_encoding`, `create_hash_to_scalar`, and
///   `create_random_source` under the default params.
#[test]
fn verify_returns_false_for_a_proof_of_the_other_form() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter: SchnorrFsDeliveryProof<'_, P, E, F> = build_schnorr_fs_delivery_proof(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                SchnorrFsDeliveryProofConstructorParamsOverrides {
                    verifier_form: Some(VerifierGroupArithmetic::BothGroups),
                },
            );
            let second: SchnorrFsDeliveryProof<'_, P, E, F> = build_schnorr_fs_delivery_proof(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                SchnorrFsDeliveryProofConstructorParamsOverrides {
                    verifier_form: Some(VerifierGroupArithmetic::FirstGroupOnly),
                },
            );
            let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let payload = build_prove_mint_payload::<P, F>(pairing, &statement, Default::default());
            let proof = build_schnorr_fs_mint_proof(&second, &payload, Default::default());

            // Act
            let result = adapter.verify(
                VerifyParams,
                build_verify_payload(
                    AlgebraicStatement::Mint(&statement),
                    &proof,
                    Default::default(),
                ),
            );

            // Assert
            assert_eq!(result.ok().map(|success| success.is_valid), Some(false));
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(Default::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
    // verify_returns_false_for_a_proof_of_the_other_form
}

/// Contract: `verify`, both-groups form, mint: the first messages
///   recomputed from the responses and challenge reproduce the challenge →
///   `Ok(VerifySuccessReturn { is_valid: true })`.
/// Arrange: the both-groups adapter; the default mint statement and
///   payload; the proof from `build_schnorr_fs_mint_proof` with no
///   overrides.
/// Act:     `verify(VerifyParams, payload)` over
///   `AlgebraicStatement::Mint(&statement)` and the proof.
/// Assert:  in place, `result.ok().map(|success| success.is_valid)` equals
///   `Some(true)` by `assert_eq!`; outside, every factory result `is_ok()`.
/// Collaborators: the pairing, chain-forms, encoding, hash-to-scalar, and
///   randomness families' mock concretes through `create_pairing`,
///   `create_chain_forms`, `create_encoding`, `create_hash_to_scalar`, and
///   `create_random_source` under the default params.
#[test]
fn verify_accepts_a_both_groups_mint_proof() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter: SchnorrFsDeliveryProof<'_, P, E, F> = build_schnorr_fs_delivery_proof(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let payload = build_prove_mint_payload::<P, F>(pairing, &statement, Default::default());
            let proof = build_schnorr_fs_mint_proof(&adapter, &payload, Default::default());

            // Act
            let result = adapter.verify(
                VerifyParams,
                build_verify_payload(
                    AlgebraicStatement::Mint(&statement),
                    &proof,
                    Default::default(),
                ),
            );

            // Assert
            assert_eq!(result.ok().map(|success| success.is_valid), Some(true));
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(Default::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
    // verify_accepts_a_both_groups_mint_proof
}

/// Contract: `verify`, both-groups form, mint: a changed response yields
///   another challenge → `Ok(VerifySuccessReturn { is_valid: false })`.
/// Arrange: the both-groups adapter; the default mint statement and
///   payload; the proof from `build_schnorr_fs_mint_proof` with `responses`
///   overridden by `build_mint_responses` with `alpha` a scalar other than
///   the proof's.
/// Act:     `verify(VerifyParams, payload)` over
///   `AlgebraicStatement::Mint(&statement)` and the proof.
/// Assert:  in place, `result.ok().map(|success| success.is_valid)` equals
///   `Some(false)` by `assert_eq!`; outside, every factory result `is_ok()`.
/// Collaborators: the pairing, chain-forms, encoding, hash-to-scalar, and
///   randomness families' mock concretes through `create_pairing`,
///   `create_chain_forms`, `create_encoding`, `create_hash_to_scalar`, and
///   `create_random_source` under the default params.
#[test]
fn verify_rejects_a_both_groups_mint_proof_with_a_changed_response() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter: SchnorrFsDeliveryProof<'_, P, E, F> = build_schnorr_fs_delivery_proof(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let payload = build_prove_mint_payload::<P, F>(pairing, &statement, Default::default());
            let proof = build_schnorr_fs_mint_proof(
                &adapter,
                &payload,
                SchnorrFsMintProofOverrides {
                    responses: Some(build_mint_responses::<P>(pairing, Default::default())),
                    ..Default::default()
                },
            );

            // Act
            let result = adapter.verify(
                VerifyParams,
                build_verify_payload(
                    AlgebraicStatement::Mint(&statement),
                    &proof,
                    Default::default(),
                ),
            );

            // Assert
            assert_eq!(result.ok().map(|success| success.is_valid), Some(false));
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(Default::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
    // verify_rejects_a_both_groups_mint_proof_with_a_changed_response
}

/// Contract: `verify`, both-groups form, transfer: the first messages
///   recomputed from the responses and challenge reproduce the challenge →
///   `Ok(VerifySuccessReturn { is_valid: true })`.
/// Arrange: the both-groups adapter; the default transfer statement and
///   payload; the proof from `build_schnorr_fs_transfer_proof` with no
///   overrides.
/// Act:     `verify(VerifyParams, payload)` over
///   `AlgebraicStatement::Transfer(&statement)` and the proof.
/// Assert:  in place, `result.ok().map(|success| success.is_valid)` equals
///   `Some(true)` by `assert_eq!`; outside, every factory result `is_ok()`.
/// Collaborators: the pairing, chain-forms, encoding, hash-to-scalar, and
///   randomness families' mock concretes through `create_pairing`,
///   `create_chain_forms`, `create_encoding`, `create_hash_to_scalar`, and
///   `create_random_source` under the default params.
#[test]
fn verify_accepts_a_both_groups_transfer_proof() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter: SchnorrFsDeliveryProof<'_, P, E, F> = build_schnorr_fs_delivery_proof(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let statement = build_algebraic_transfer_statement::<P, F>(pairing, Default::default());
            let payload =
                build_prove_transfer_payload::<P, F>(pairing, &statement, Default::default());
            let proof = build_schnorr_fs_transfer_proof(&adapter, &payload, Default::default());

            // Act
            let result = adapter.verify(
                VerifyParams,
                build_verify_payload(
                    AlgebraicStatement::Transfer(&statement),
                    &proof,
                    Default::default(),
                ),
            );

            // Assert
            assert_eq!(result.ok().map(|success| success.is_valid), Some(true));
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(Default::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
    // verify_accepts_a_both_groups_transfer_proof
}

/// Contract: `verify`, both-groups form, transfer: a changed response
///   yields another challenge → `Ok(VerifySuccessReturn { is_valid: false
///   })`.
/// Arrange: the both-groups adapter; the default transfer statement and
///   payload; the proof from `build_schnorr_fs_transfer_proof` with
///   `responses` overridden by `build_transfer_responses` with `s` a scalar
///   other than the proof's.
/// Act:     `verify(VerifyParams, payload)` over
///   `AlgebraicStatement::Transfer(&statement)` and the proof.
/// Assert:  in place, `result.ok().map(|success| success.is_valid)` equals
///   `Some(false)` by `assert_eq!`; outside, every factory result `is_ok()`.
/// Collaborators: the pairing, chain-forms, encoding, hash-to-scalar, and
///   randomness families' mock concretes through `create_pairing`,
///   `create_chain_forms`, `create_encoding`, `create_hash_to_scalar`, and
///   `create_random_source` under the default params.
#[test]
fn verify_rejects_a_both_groups_transfer_proof_with_a_changed_response() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter: SchnorrFsDeliveryProof<'_, P, E, F> = build_schnorr_fs_delivery_proof(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let statement = build_algebraic_transfer_statement::<P, F>(pairing, Default::default());
            let payload =
                build_prove_transfer_payload::<P, F>(pairing, &statement, Default::default());
            let proof = build_schnorr_fs_transfer_proof(
                &adapter,
                &payload,
                SchnorrFsTransferProofOverrides {
                    responses: Some(build_transfer_responses::<P>(pairing, Default::default())),
                    ..Default::default()
                },
            );

            // Act
            let result = adapter.verify(
                VerifyParams,
                build_verify_payload(
                    AlgebraicStatement::Transfer(&statement),
                    &proof,
                    Default::default(),
                ),
            );

            // Assert
            assert_eq!(result.ok().map(|success| success.is_valid), Some(false));
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(Default::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
    // verify_rejects_a_both_groups_transfer_proof_with_a_changed_response
}

/// Contract: `verify`, first-group-only form, mint: the challenge
///   reproduces and the weighted pairing product is one →
///   `Ok(VerifySuccessReturn { is_valid: true })`.
/// Arrange: the adapter constructed with `verifier_form` `FirstGroupOnly`;
///   the default mint statement and payload; the proof from
///   `build_schnorr_fs_mint_proof` with no overrides.
/// Act:     `verify(VerifyParams, payload)` over
///   `AlgebraicStatement::Mint(&statement)` and the proof.
/// Assert:  in place, `result.ok().map(|success| success.is_valid)` equals
///   `Some(true)` by `assert_eq!`; outside, every factory result `is_ok()`.
/// Collaborators: the pairing, chain-forms, encoding, hash-to-scalar, and
///   randomness families' mock concretes through `create_pairing`,
///   `create_chain_forms`, `create_encoding`, `create_hash_to_scalar`, and
///   `create_random_source` under the default params.
#[test]
fn verify_accepts_a_first_group_only_mint_proof() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter: SchnorrFsDeliveryProof<'_, P, E, F> = build_schnorr_fs_delivery_proof(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                SchnorrFsDeliveryProofConstructorParamsOverrides {
                    verifier_form: Some(VerifierGroupArithmetic::FirstGroupOnly),
                },
            );
            let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let payload = build_prove_mint_payload::<P, F>(pairing, &statement, Default::default());
            let proof = build_schnorr_fs_mint_proof(&adapter, &payload, Default::default());

            // Act
            let result = adapter.verify(
                VerifyParams,
                build_verify_payload(
                    AlgebraicStatement::Mint(&statement),
                    &proof,
                    Default::default(),
                ),
            );

            // Assert
            assert_eq!(result.ok().map(|success| success.is_valid), Some(true));
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(Default::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
    // verify_accepts_a_first_group_only_mint_proof
}

/// Contract: `verify`, first-group-only form, challenge: the recomputed
///   challenge differs from the proof's → `Ok(VerifySuccessReturn {
///   is_valid: false })`.
/// Arrange: the adapter constructed with `verifier_form` `FirstGroupOnly`;
///   the default mint statement and payload; the proof from
///   `build_schnorr_fs_mint_proof` with `challenge` overridden by a scalar
///   other than the proof's.
/// Act:     `verify(VerifyParams, payload)` over
///   `AlgebraicStatement::Mint(&statement)` and the proof.
/// Assert:  in place, `result.ok().map(|success| success.is_valid)` equals
///   `Some(false)` by `assert_eq!`; outside, every factory result `is_ok()`.
/// Collaborators: the pairing, chain-forms, encoding, hash-to-scalar, and
///   randomness families' mock concretes through `create_pairing`,
///   `create_chain_forms`, `create_encoding`, `create_hash_to_scalar`, and
///   `create_random_source` under the default params.
#[test]
fn verify_rejects_a_first_group_only_mint_proof_with_a_changed_challenge() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter: SchnorrFsDeliveryProof<'_, P, E, F> = build_schnorr_fs_delivery_proof(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                SchnorrFsDeliveryProofConstructorParamsOverrides {
                    verifier_form: Some(VerifierGroupArithmetic::FirstGroupOnly),
                },
            );
            let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let payload = build_prove_mint_payload::<P, F>(pairing, &statement, Default::default());
            let proof = build_schnorr_fs_mint_proof(
                &adapter,
                &payload,
                SchnorrFsMintProofOverrides {
                    challenge: Some(payload.master_scalar.value.expose().clone()),
                    ..Default::default()
                },
            );

            // Act
            let result = adapter.verify(
                VerifyParams,
                build_verify_payload(
                    AlgebraicStatement::Mint(&statement),
                    &proof,
                    Default::default(),
                ),
            );

            // Assert
            assert_eq!(result.ok().map(|success| success.is_valid), Some(false));
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(Default::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
    // verify_rejects_a_first_group_only_mint_proof_with_a_changed_challenge
}

/// Contract: `verify`, first-group-only form, mint product: a carried
///   second-group first message that does not satisfy its equation fails
///   the product though the challenge was computed over it →
///   `Ok(VerifySuccessReturn { is_valid: false })`.
/// Arrange: the adapter constructed with `verifier_form` `FirstGroupOnly`;
///   the default mint statement and payload; the proof from
///   `build_schnorr_fs_mint_proof` with `first_messages` overridden by
///   `build_mint_second_group_first_messages` with `d1` a point other than
///   the proof's, the challenge default recomputed over the changed
///   message.
/// Act:     `verify(VerifyParams, payload)` over
///   `AlgebraicStatement::Mint(&statement)` and the proof.
/// Assert:  in place, `result.ok().map(|success| success.is_valid)` equals
///   `Some(false)` by `assert_eq!`; outside, every factory result `is_ok()`.
/// Collaborators: the pairing, chain-forms, encoding, hash-to-scalar, and
///   randomness families' mock concretes through `create_pairing`,
///   `create_chain_forms`, `create_encoding`, `create_hash_to_scalar`, and
///   `create_random_source` under the default params.
#[test]
fn verify_rejects_a_first_group_only_mint_proof_with_a_changed_second_group_first_message() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter: SchnorrFsDeliveryProof<'_, P, E, F> = build_schnorr_fs_delivery_proof(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                SchnorrFsDeliveryProofConstructorParamsOverrides {
                    verifier_form: Some(VerifierGroupArithmetic::FirstGroupOnly),
                },
            );
            let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let payload = build_prove_mint_payload::<P, F>(pairing, &statement, Default::default());
            let proof = build_schnorr_fs_mint_proof(
                &adapter,
                &payload,
                SchnorrFsMintProofOverrides {
                    first_messages: Some(build_mint_second_group_first_messages::<P>(
                        pairing,
                        MintSecondGroupFirstMessagesOverrides {
                            d1: Some(statement.envelope.d1.clone()),
                            ..Default::default()
                        },
                    )),
                    ..Default::default()
                },
            );

            // Act
            let result = adapter.verify(
                VerifyParams,
                build_verify_payload(
                    AlgebraicStatement::Mint(&statement),
                    &proof,
                    Default::default(),
                ),
            );

            // Assert
            assert_eq!(result.ok().map(|success| success.is_valid), Some(false));
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(Default::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
    // verify_rejects_a_first_group_only_mint_proof_with_a_changed_second_group_first_message
}

/// Contract: `verify`, first-group-only form, transfer: the challenge
///   reproduces and the weighted pairing product is one →
///   `Ok(VerifySuccessReturn { is_valid: true })`.
/// Arrange: the adapter constructed with `verifier_form` `FirstGroupOnly`;
///   the default transfer statement and payload; the proof from
///   `build_schnorr_fs_transfer_proof` with no overrides.
/// Act:     `verify(VerifyParams, payload)` over
///   `AlgebraicStatement::Transfer(&statement)` and the proof.
/// Assert:  in place, `result.ok().map(|success| success.is_valid)` equals
///   `Some(true)` by `assert_eq!`; outside, every factory result `is_ok()`.
/// Collaborators: the pairing, chain-forms, encoding, hash-to-scalar, and
///   randomness families' mock concretes through `create_pairing`,
///   `create_chain_forms`, `create_encoding`, `create_hash_to_scalar`, and
///   `create_random_source` under the default params.
#[test]
fn verify_accepts_a_first_group_only_transfer_proof() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter: SchnorrFsDeliveryProof<'_, P, E, F> = build_schnorr_fs_delivery_proof(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                SchnorrFsDeliveryProofConstructorParamsOverrides {
                    verifier_form: Some(VerifierGroupArithmetic::FirstGroupOnly),
                },
            );
            let statement = build_algebraic_transfer_statement::<P, F>(pairing, Default::default());
            let payload =
                build_prove_transfer_payload::<P, F>(pairing, &statement, Default::default());
            let proof = build_schnorr_fs_transfer_proof(&adapter, &payload, Default::default());

            // Act
            let result = adapter.verify(
                VerifyParams,
                build_verify_payload(
                    AlgebraicStatement::Transfer(&statement),
                    &proof,
                    Default::default(),
                ),
            );

            // Assert
            assert_eq!(result.ok().map(|success| success.is_valid), Some(true));
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(Default::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
    // verify_accepts_a_first_group_only_transfer_proof
}

/// Contract: `verify`, first-group-only form, challenge: the recomputed
///   challenge differs from the proof's → `Ok(VerifySuccessReturn {
///   is_valid: false })`.
/// Arrange: the adapter constructed with `verifier_form` `FirstGroupOnly`;
///   the default transfer statement and payload; the proof from
///   `build_schnorr_fs_transfer_proof` with `challenge` overridden by a
///   scalar other than the proof's.
/// Act:     `verify(VerifyParams, payload)` over
///   `AlgebraicStatement::Transfer(&statement)` and the proof.
/// Assert:  in place, `result.ok().map(|success| success.is_valid)` equals
///   `Some(false)` by `assert_eq!`; outside, every factory result `is_ok()`.
/// Collaborators: the pairing, chain-forms, encoding, hash-to-scalar, and
///   randomness families' mock concretes through `create_pairing`,
///   `create_chain_forms`, `create_encoding`, `create_hash_to_scalar`, and
///   `create_random_source` under the default params.
#[test]
fn verify_rejects_a_first_group_only_transfer_proof_with_a_changed_challenge() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter: SchnorrFsDeliveryProof<'_, P, E, F> = build_schnorr_fs_delivery_proof(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                SchnorrFsDeliveryProofConstructorParamsOverrides {
                    verifier_form: Some(VerifierGroupArithmetic::FirstGroupOnly),
                },
            );
            let statement = build_algebraic_transfer_statement::<P, F>(pairing, Default::default());
            let payload =
                build_prove_transfer_payload::<P, F>(pairing, &statement, Default::default());
            let proof = build_schnorr_fs_transfer_proof(
                &adapter,
                &payload,
                SchnorrFsTransferProofOverrides {
                    challenge: Some(payload.offset.expose().clone()),
                    ..Default::default()
                },
            );

            // Act
            let result = adapter.verify(
                VerifyParams,
                build_verify_payload(
                    AlgebraicStatement::Transfer(&statement),
                    &proof,
                    Default::default(),
                ),
            );

            // Assert
            assert_eq!(result.ok().map(|success| success.is_valid), Some(false));
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(Default::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
    // verify_rejects_a_first_group_only_transfer_proof_with_a_changed_challenge
}

/// Contract: `verify`, first-group-only form, transfer product: a carried
///   second-group first message that does not satisfy its equation fails
///   the product though the challenge was computed over it →
///   `Ok(VerifySuccessReturn { is_valid: false })`.
/// Arrange: the adapter constructed with `verifier_form` `FirstGroupOnly`;
///   the default transfer statement and payload; the proof from
///   `build_schnorr_fs_transfer_proof` with `first_messages` overridden by
///   `build_transfer_second_group_first_messages` with its `d2` field a
///   point other than the proof's, the challenge default recomputed over
///   the changed message.
/// Act:     `verify(VerifyParams, payload)` over
///   `AlgebraicStatement::Transfer(&statement)` and the proof.
/// Assert:  in place, `result.ok().map(|success| success.is_valid)` equals
///   `Some(false)` by `assert_eq!`; outside, every factory result `is_ok()`.
/// Collaborators: the pairing, chain-forms, encoding, hash-to-scalar, and
///   randomness families' mock concretes through `create_pairing`,
///   `create_chain_forms`, `create_encoding`, `create_hash_to_scalar`, and
///   `create_random_source` under the default params.
#[test]
fn verify_rejects_a_first_group_only_transfer_proof_with_a_changed_second_group_first_message() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter: SchnorrFsDeliveryProof<'_, P, E, F> = build_schnorr_fs_delivery_proof(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                SchnorrFsDeliveryProofConstructorParamsOverrides {
                    verifier_form: Some(VerifierGroupArithmetic::FirstGroupOnly),
                },
            );
            let statement = build_algebraic_transfer_statement::<P, F>(pairing, Default::default());
            let payload =
                build_prove_transfer_payload::<P, F>(pairing, &statement, Default::default());
            let proof = build_schnorr_fs_transfer_proof(
                &adapter,
                &payload,
                SchnorrFsTransferProofOverrides {
                    first_messages: Some(build_transfer_second_group_first_messages::<P>(
                        pairing,
                        TransferSecondGroupFirstMessagesOverrides {
                            d2: Some(statement.old_envelope.d2.clone()),
                            ..Default::default()
                        },
                    )),
                    ..Default::default()
                },
            );

            // Act
            let result = adapter.verify(
                VerifyParams,
                build_verify_payload(
                    AlgebraicStatement::Transfer(&statement),
                    &proof,
                    Default::default(),
                ),
            );

            // Assert
            assert_eq!(result.ok().map(|success| success.is_valid), Some(false));
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(Default::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
    // verify_rejects_a_first_group_only_transfer_proof_with_a_changed_second_group_first_message
}

/// Contract: `verify`, both-groups form: `challenge` returns `Err(error)` →
///   `Err(VerifyErrorReturn::SchnorrFs(SchnorrFsVerifyErrorReturn::Challenge(
///   error)))`, the refusal unchanged.
/// Arrange: a proving adapter over the default hash-to-scalar adapter forms
///   the proof from `build_schnorr_fs_mint_proof`; the subject's adapter
///   holds the adapter `create_hash_to_scalar` returns under
///   `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::
///   SamplingRefused)`.
/// Act:     `verify(VerifyParams, payload)` on the subject's adapter.
/// Assert:  in place, the whole error by `assert_eq!`; outside, every
///   factory result `is_ok()`.
/// Collaborators: the pairing, chain-forms, encoding, and randomness
///   families' mock concretes through `create_pairing`, `create_chain_forms`,
///   `create_encoding`, and `create_random_source` under the default params,
///   the hash-to-scalar family's mock concrete under the default params for
///   the proving adapter, and the hash-to-scalar family's mock concrete
///   under `MockIHashToScalarAdapterFailureMode::SamplingRefused` for the
///   subject's adapter through `create_hash_to_scalar`.
#[test]
fn verify_returns_the_challenge_refusal_unchanged_in_the_both_groups_form() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Arrange
            let refused = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: Some(HashToScalarConcrete::Mock(
                        MockIHashToScalarAdapterFailureMode::SamplingRefused,
                    )),
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the sampling-refused selection constructs the family's mock concrete");
            let proving: SchnorrFsDeliveryProof<'_, P, E, F> = build_schnorr_fs_delivery_proof(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let adapter: SchnorrFsDeliveryProof<'_, P, E, F> = build_schnorr_fs_delivery_proof(
                pairing,
                &*refused.adapter,
                encoder,
                random,
                Default::default(),
            );
            let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let payload = build_prove_mint_payload::<P, F>(pairing, &statement, Default::default());
            let proof = build_schnorr_fs_mint_proof(&proving, &payload, Default::default());

            // Act
            let result = adapter.verify(
                VerifyParams,
                build_verify_payload(
                    AlgebraicStatement::Mint(&statement),
                    &proof,
                    Default::default(),
                ),
            );

            // Assert
            assert_eq!(
                result.err(),
                Some(VerifyErrorReturn::SchnorrFs(
                    SchnorrFsVerifyErrorReturn::Challenge(ChallengeErrorReturn::HashToScalar(
                        HashToScalarErrorReturn::Sampling(
                            SampleUniformScalarErrorReturn::WrongLength {
                                expected: P::Scalar::UNIFORM_BYTES_LENGTH,
                                actual: 0,
                            },
                        ),
                    ),),
                )),
            );
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(Default::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
    // verify_returns_the_challenge_refusal_unchanged_in_the_both_groups_form
}

/// Contract: `verify`, first-group-only form: `challenge` returns
///   `Err(error)` before the weights are derived → `Err(
///   VerifyErrorReturn::SchnorrFs(SchnorrFsVerifyErrorReturn::Challenge(
///   error)))`, the refusal unchanged.
/// Arrange: a proving adapter over the default hash-to-scalar adapter forms
///   the proof from `build_schnorr_fs_transfer_proof`; the subject's
///   adapter holds the adapter `create_hash_to_scalar` returns under
///   `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::
///   SamplingRefused)`; both adapters constructed in the first-group-only
///   form.
/// Act:     `verify(VerifyParams, payload)` on the subject's adapter.
/// Assert:  in place, the whole error by `assert_eq!`; outside, every
///   factory result `is_ok()`.
/// Collaborators: the pairing, chain-forms, encoding, and randomness
///   families' mock concretes through `create_pairing`, `create_chain_forms`,
///   `create_encoding`, and `create_random_source` under the default params,
///   the hash-to-scalar family's mock concrete under the default params for
///   the proving adapter, and the hash-to-scalar family's mock concrete
///   under `MockIHashToScalarAdapterFailureMode::SamplingRefused` for the
///   subject's adapter through `create_hash_to_scalar`.
#[test]
fn verify_returns_the_challenge_refusal_unchanged_in_the_first_group_only_form() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Arrange
            let refused = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: Some(HashToScalarConcrete::Mock(
                        MockIHashToScalarAdapterFailureMode::SamplingRefused,
                    )),
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the sampling-refused selection constructs the family's mock concrete");
            let proving: SchnorrFsDeliveryProof<'_, P, E, F> = build_schnorr_fs_delivery_proof(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                SchnorrFsDeliveryProofConstructorParamsOverrides {
                    verifier_form: Some(VerifierGroupArithmetic::FirstGroupOnly),
                },
            );
            let adapter: SchnorrFsDeliveryProof<'_, P, E, F> = build_schnorr_fs_delivery_proof(
                pairing,
                &*refused.adapter,
                encoder,
                random,
                SchnorrFsDeliveryProofConstructorParamsOverrides {
                    verifier_form: Some(VerifierGroupArithmetic::FirstGroupOnly),
                },
            );
            let statement = build_algebraic_transfer_statement::<P, F>(pairing, Default::default());
            let payload =
                build_prove_transfer_payload::<P, F>(pairing, &statement, Default::default());
            let proof = build_schnorr_fs_transfer_proof(&proving, &payload, Default::default());

            // Act
            let result = adapter.verify(
                VerifyParams,
                build_verify_payload(
                    AlgebraicStatement::Transfer(&statement),
                    &proof,
                    Default::default(),
                ),
            );

            // Assert
            assert_eq!(
                result.err(),
                Some(VerifyErrorReturn::SchnorrFs(
                    SchnorrFsVerifyErrorReturn::Challenge(ChallengeErrorReturn::HashToScalar(
                        HashToScalarErrorReturn::Sampling(
                            SampleUniformScalarErrorReturn::WrongLength {
                                expected: P::Scalar::UNIFORM_BYTES_LENGTH,
                                actual: 0,
                            },
                        ),
                    ),),
                )),
            );
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(Default::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
    // verify_returns_the_challenge_refusal_unchanged_in_the_first_group_only_form
}

/// Contract: `proof_components` clones the challenge and the relation's
///   named responses into the matching variant.
/// Arrange: the both-groups adapter; the proof from
///   `build_schnorr_fs_mint_proof` with `challenge` the `x` of
///   `build_transfer_responses` and `responses` from
///   `build_mint_responses`, every scalar distinct.
/// Act:     `proof_components(ProofComponentsParams, payload)`.
/// Assert:  in place, the components match `DeliveryProofComponents::
///   MintBothGroups { challenge, responses }` and the challenge, `alpha`,
///   `r`, `rho`, and `sigma` each equal their own input by `encode_scalar`;
///   outside, every factory result `is_ok()`.
/// Collaborators: the pairing, chain-forms, encoding, hash-to-scalar, and
///   randomness families' mock concretes through `create_pairing`,
///   `create_chain_forms`, `create_encoding`, `create_hash_to_scalar`, and
///   `create_random_source` under the default params.
#[test]
fn proof_components_returns_the_mint_both_groups_components() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter: SchnorrFsDeliveryProof<'_, P, E, F> = build_schnorr_fs_delivery_proof(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let payload = build_prove_mint_payload::<P, F>(pairing, &statement, Default::default());
            let challenge_input = build_transfer_responses::<P>(pairing, Default::default()).x;
            let responses_input = build_mint_responses::<P>(pairing, Default::default());
            let expected_challenge = challenge_input.clone();
            let expected_alpha = responses_input.alpha.clone();
            let expected_r = responses_input.r.clone();
            let expected_rho = responses_input.rho.clone();
            let expected_sigma = responses_input.sigma.clone();
            let proof = build_schnorr_fs_mint_proof(
                &adapter,
                &payload,
                SchnorrFsMintProofOverrides {
                    challenge: Some(challenge_input),
                    responses: Some(responses_input),
                    ..Default::default()
                },
            );

            // Act
            let result = adapter.proof_components(
                ProofComponentsParams,
                build_proof_components_payload(&proof, Default::default()),
            );

            // Assert
            let Ok(success) = result;
            let DeliveryProofComponents::MintBothGroups {
                challenge,
                responses,
            } = success.components
            else {
                panic!("the components are the mint both-groups variant")
            };
            let encode_scalar = |scalar: P::Scalar| -> Vec<u8> {
                pairing
                    .encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar })
                    .expect("encoding a scalar succeeds")
                    .bytes
                    .expose()
                    .as_ref()
                    .to_vec()
            };
            assert_eq!(encode_scalar(challenge), encode_scalar(expected_challenge));
            assert_eq!(
                encode_scalar(responses.alpha),
                encode_scalar(expected_alpha)
            );
            assert_eq!(encode_scalar(responses.r), encode_scalar(expected_r));
            assert_eq!(encode_scalar(responses.rho), encode_scalar(expected_rho));
            assert_eq!(
                encode_scalar(responses.sigma),
                encode_scalar(expected_sigma)
            );
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(Default::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
    // proof_components_returns_the_mint_both_groups_components
}

/// Contract: `proof_components` clones the challenge, the responses, and
///   the carried first messages into the matching variant.
/// Arrange: the first-group-only adapter; the proof as the both-groups
///   block with `first_messages` from
///   `build_mint_second_group_first_messages`.
/// Act:     `proof_components(ProofComponentsParams, payload)`.
/// Assert:  in place, the components match `DeliveryProofComponents::
///   MintFirstGroupOnly { challenge, responses, first_messages }`, the
///   scalars as the both-groups block and `hpub`, `d1`, and `d2` each
///   equal their own input by `encode_g2`; outside, every factory result
///   `is_ok()`.
/// Collaborators: the pairing, chain-forms, encoding, hash-to-scalar, and
///   randomness families' mock concretes through `create_pairing`,
///   `create_chain_forms`, `create_encoding`, `create_hash_to_scalar`, and
///   `create_random_source` under the default params.
#[test]
fn proof_components_returns_the_mint_first_group_only_components() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter: SchnorrFsDeliveryProof<'_, P, E, F> = build_schnorr_fs_delivery_proof(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                SchnorrFsDeliveryProofConstructorParamsOverrides {
                    verifier_form: Some(VerifierGroupArithmetic::FirstGroupOnly),
                },
            );
            let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let payload = build_prove_mint_payload::<P, F>(pairing, &statement, Default::default());
            let challenge_input = build_transfer_responses::<P>(pairing, Default::default()).x;
            let responses_input = build_mint_responses::<P>(pairing, Default::default());
            let first_messages_input =
                build_mint_second_group_first_messages::<P>(pairing, Default::default());
            let expected_challenge = challenge_input.clone();
            let expected_alpha = responses_input.alpha.clone();
            let expected_r = responses_input.r.clone();
            let expected_rho = responses_input.rho.clone();
            let expected_sigma = responses_input.sigma.clone();
            let expected_hpub = first_messages_input.hpub.clone();
            let expected_d1 = first_messages_input.d1.clone();
            let expected_d2 = first_messages_input.d2.clone();
            let proof = build_schnorr_fs_mint_proof(
                &adapter,
                &payload,
                SchnorrFsMintProofOverrides {
                    challenge: Some(challenge_input),
                    responses: Some(responses_input),
                    first_messages: Some(first_messages_input),
                },
            );

            // Act
            let result = adapter.proof_components(
                ProofComponentsParams,
                build_proof_components_payload(&proof, Default::default()),
            );

            // Assert
            let Ok(success) = result;
            let DeliveryProofComponents::MintFirstGroupOnly {
                challenge,
                responses,
                first_messages,
            } = success.components
            else {
                panic!("the components are the mint first-group-only variant")
            };
            let encode_scalar = |scalar: P::Scalar| -> Vec<u8> {
                pairing
                    .encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar })
                    .expect("encoding a scalar succeeds")
                    .bytes
                    .expose()
                    .as_ref()
                    .to_vec()
            };
            let encode_g2 = |point: P::G2| -> Vec<u8> {
                pairing
                    .encode_g2(EncodeG2Params, EncodeG2Payload { point })
                    .expect("encoding a second-group point succeeds")
                    .bytes
                    .as_ref()
                    .to_vec()
            };
            assert_eq!(encode_scalar(challenge), encode_scalar(expected_challenge));
            assert_eq!(
                encode_scalar(responses.alpha),
                encode_scalar(expected_alpha)
            );
            assert_eq!(encode_scalar(responses.r), encode_scalar(expected_r));
            assert_eq!(encode_scalar(responses.rho), encode_scalar(expected_rho));
            assert_eq!(
                encode_scalar(responses.sigma),
                encode_scalar(expected_sigma)
            );
            assert_eq!(encode_g2(first_messages.hpub), encode_g2(expected_hpub));
            assert_eq!(encode_g2(first_messages.d1), encode_g2(expected_d1));
            assert_eq!(encode_g2(first_messages.d2), encode_g2(expected_d2));
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(Default::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
    // proof_components_returns_the_mint_first_group_only_components
}

/// Contract: `proof_components` clones the challenge and the relation's
///   named responses into the matching variant.
/// Arrange: the both-groups adapter; the proof from
///   `build_schnorr_fs_transfer_proof` with `challenge` the `alpha` of
///   `build_mint_responses` and `responses` from
///   `build_transfer_responses`, every scalar distinct.
/// Act:     `proof_components(ProofComponentsParams, payload)`.
/// Assert:  in place, the components match `DeliveryProofComponents::
///   TransferBothGroups { challenge, responses }` and the challenge and
///   `x`, `y`, `s`, `rho`, and `sigma` each equal their own input by
///   `encode_scalar`; outside, every factory result `is_ok()`.
/// Collaborators: the pairing, chain-forms, encoding, hash-to-scalar, and
///   randomness families' mock concretes through `create_pairing`,
///   `create_chain_forms`, `create_encoding`, `create_hash_to_scalar`, and
///   `create_random_source` under the default params.
#[test]
fn proof_components_returns_the_transfer_both_groups_components() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter: SchnorrFsDeliveryProof<'_, P, E, F> = build_schnorr_fs_delivery_proof(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let statement = build_algebraic_transfer_statement::<P, F>(pairing, Default::default());
            let payload =
                build_prove_transfer_payload::<P, F>(pairing, &statement, Default::default());
            let challenge_input = build_mint_responses::<P>(pairing, Default::default()).alpha;
            let responses_input = build_transfer_responses::<P>(pairing, Default::default());
            let expected_challenge = challenge_input.clone();
            let expected_x = responses_input.x.clone();
            let expected_y = responses_input.y.clone();
            let expected_s = responses_input.s.clone();
            let expected_rho = responses_input.rho.clone();
            let expected_sigma = responses_input.sigma.clone();
            let proof = build_schnorr_fs_transfer_proof(
                &adapter,
                &payload,
                SchnorrFsTransferProofOverrides {
                    challenge: Some(challenge_input),
                    responses: Some(responses_input),
                    ..Default::default()
                },
            );

            // Act
            let result = adapter.proof_components(
                ProofComponentsParams,
                build_proof_components_payload(&proof, Default::default()),
            );

            // Assert
            let Ok(success) = result;
            let DeliveryProofComponents::TransferBothGroups {
                challenge,
                responses,
            } = success.components
            else {
                panic!("the components are the transfer both-groups variant")
            };
            let encode_scalar = |scalar: P::Scalar| -> Vec<u8> {
                pairing
                    .encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar })
                    .expect("encoding a scalar succeeds")
                    .bytes
                    .expose()
                    .as_ref()
                    .to_vec()
            };
            assert_eq!(encode_scalar(challenge), encode_scalar(expected_challenge));
            assert_eq!(encode_scalar(responses.x), encode_scalar(expected_x));
            assert_eq!(encode_scalar(responses.y), encode_scalar(expected_y));
            assert_eq!(encode_scalar(responses.s), encode_scalar(expected_s));
            assert_eq!(encode_scalar(responses.rho), encode_scalar(expected_rho));
            assert_eq!(
                encode_scalar(responses.sigma),
                encode_scalar(expected_sigma)
            );
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(Default::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
    // proof_components_returns_the_transfer_both_groups_components
}

/// Contract: `proof_components` clones the challenge, the responses, and
///   the carried first messages into the matching variant.
/// Arrange: the first-group-only adapter; the proof as the both-groups
///   block with `first_messages` from
///   `build_transfer_second_group_first_messages`.
/// Act:     `proof_components(ProofComponentsParams, payload)`.
/// Assert:  in place, the components match `DeliveryProofComponents::
///   TransferFirstGroupOnly { challenge, responses, first_messages }`,
///   the scalars as the both-groups block and each of the three fields of
///   `first_messages` equal to its own input by `encode_g2`; outside,
///   every factory result `is_ok()`.
/// Collaborators: the pairing, chain-forms, encoding, hash-to-scalar, and
///   randomness families' mock concretes through `create_pairing`,
///   `create_chain_forms`, `create_encoding`, `create_hash_to_scalar`, and
///   `create_random_source` under the default params.
#[test]
fn proof_components_returns_the_transfer_first_group_only_components() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter: SchnorrFsDeliveryProof<'_, P, E, F> = build_schnorr_fs_delivery_proof(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                SchnorrFsDeliveryProofConstructorParamsOverrides {
                    verifier_form: Some(VerifierGroupArithmetic::FirstGroupOnly),
                },
            );
            let statement = build_algebraic_transfer_statement::<P, F>(pairing, Default::default());
            let payload =
                build_prove_transfer_payload::<P, F>(pairing, &statement, Default::default());
            let challenge_input = build_mint_responses::<P>(pairing, Default::default()).alpha;
            let responses_input = build_transfer_responses::<P>(pairing, Default::default());
            let first_messages_input =
                build_transfer_second_group_first_messages::<P>(pairing, Default::default());
            let expected_challenge = challenge_input.clone();
            let expected_x = responses_input.x.clone();
            let expected_y = responses_input.y.clone();
            let expected_s = responses_input.s.clone();
            let expected_rho = responses_input.rho.clone();
            let expected_sigma = responses_input.sigma.clone();
            let expected_pk2 = first_messages_input.pk2.clone();
            let expected_d1 = first_messages_input.d1.clone();
            let expected_d2 = first_messages_input.d2.clone();
            let proof = build_schnorr_fs_transfer_proof(
                &adapter,
                &payload,
                SchnorrFsTransferProofOverrides {
                    challenge: Some(challenge_input),
                    responses: Some(responses_input),
                    first_messages: Some(first_messages_input),
                },
            );

            // Act
            let result = adapter.proof_components(
                ProofComponentsParams,
                build_proof_components_payload(&proof, Default::default()),
            );

            // Assert
            let Ok(success) = result;
            let DeliveryProofComponents::TransferFirstGroupOnly {
                challenge,
                responses,
                first_messages,
            } = success.components
            else {
                panic!("the components are the transfer first-group-only variant")
            };
            let encode_scalar = |scalar: P::Scalar| -> Vec<u8> {
                pairing
                    .encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar })
                    .expect("encoding a scalar succeeds")
                    .bytes
                    .expose()
                    .as_ref()
                    .to_vec()
            };
            let encode_g2 = |point: P::G2| -> Vec<u8> {
                pairing
                    .encode_g2(EncodeG2Params, EncodeG2Payload { point })
                    .expect("encoding a second-group point succeeds")
                    .bytes
                    .as_ref()
                    .to_vec()
            };
            assert_eq!(encode_scalar(challenge), encode_scalar(expected_challenge));
            assert_eq!(encode_scalar(responses.x), encode_scalar(expected_x));
            assert_eq!(encode_scalar(responses.y), encode_scalar(expected_y));
            assert_eq!(encode_scalar(responses.s), encode_scalar(expected_s));
            assert_eq!(encode_scalar(responses.rho), encode_scalar(expected_rho));
            assert_eq!(
                encode_scalar(responses.sigma),
                encode_scalar(expected_sigma)
            );
            assert_eq!(encode_g2(first_messages.pk2), encode_g2(expected_pk2));
            assert_eq!(encode_g2(first_messages.d1), encode_g2(expected_d1));
            assert_eq!(encode_g2(first_messages.d2), encode_g2(expected_d2));
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(Default::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
    // proof_components_returns_the_transfer_first_group_only_components
}

/// Contract: `proof_from_components` moves the fields of components of the
///   adapter's form into the proof variant of the same relation and form.
/// Arrange: the both-groups adapter; the components `DeliveryProofComponents
///   ::MintBothGroups` over a challenge and `build_mint_responses`, every
///   scalar distinct.
/// Act:     `proof_from_components(ProofFromComponentsParams, payload)`.
/// Assert:  in place, the result is `Ok(ProofFromComponentsSuccessReturn {
///   proof })`, `proof` matches `SchnorrFsProof::MintBothGroups {
///   challenge, responses }`, and each scalar equals its own input by
///   `encode_scalar`; outside, every factory result `is_ok()`.
/// Collaborators: the pairing, chain-forms, encoding, hash-to-scalar, and
///   randomness families' mock concretes through `create_pairing`,
///   `create_chain_forms`, `create_encoding`, `create_hash_to_scalar`, and
///   `create_random_source` under the default params.
#[test]
fn proof_from_components_moves_the_mint_both_groups_components_into_the_proof() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter: SchnorrFsDeliveryProof<'_, P, E, F> = build_schnorr_fs_delivery_proof(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let challenge_input = build_transfer_responses::<P>(pairing, Default::default()).x;
            let responses_input = build_mint_responses::<P>(pairing, Default::default());
            let expected_challenge = challenge_input.clone();
            let expected_alpha = responses_input.alpha.clone();
            let expected_r = responses_input.r.clone();
            let expected_rho = responses_input.rho.clone();
            let expected_sigma = responses_input.sigma.clone();
            let components = DeliveryProofComponents::MintBothGroups {
                challenge: challenge_input,
                responses: responses_input,
            };

            // Act
            let result = adapter.proof_from_components(
                ProofFromComponentsParams,
                build_proof_from_components_payload(components, Default::default()),
            );

            // Assert
            let Ok(ProofFromComponentsSuccessReturn { proof }) = result else {
                panic!("the components form a proof")
            };
            let SchnorrFsProof::MintBothGroups {
                challenge,
                responses,
            } = proof
            else {
                panic!("the proof is the mint both-groups variant")
            };
            let encode_scalar = |scalar: P::Scalar| -> Vec<u8> {
                pairing
                    .encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar })
                    .expect("encoding a scalar succeeds")
                    .bytes
                    .expose()
                    .as_ref()
                    .to_vec()
            };
            assert_eq!(encode_scalar(challenge), encode_scalar(expected_challenge));
            assert_eq!(
                encode_scalar(responses.alpha),
                encode_scalar(expected_alpha)
            );
            assert_eq!(encode_scalar(responses.r), encode_scalar(expected_r));
            assert_eq!(encode_scalar(responses.rho), encode_scalar(expected_rho));
            assert_eq!(
                encode_scalar(responses.sigma),
                encode_scalar(expected_sigma)
            );
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(Default::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
    // proof_from_components_moves_the_mint_both_groups_components_into_the_proof
}

/// Contract: `proof_from_components` moves the fields of components of the
///   adapter's form into the proof variant of the same relation and form.
/// Arrange: the first-group-only adapter; the components
///   `DeliveryProofComponents::MintFirstGroupOnly` over a challenge,
///   `build_mint_responses`, and
///   `build_mint_second_group_first_messages`.
/// Act:     `proof_from_components(ProofFromComponentsParams, payload)`.
/// Assert:  in place, `proof` matches `SchnorrFsProof::MintFirstGroupOnly {
///   challenge, responses, first_messages }`, the scalars equal by
///   `encode_scalar` and each first message by `encode_g2`; outside, every
///   factory result `is_ok()`.
/// Collaborators: the pairing, chain-forms, encoding, hash-to-scalar, and
///   randomness families' mock concretes through `create_pairing`,
///   `create_chain_forms`, `create_encoding`, `create_hash_to_scalar`, and
///   `create_random_source` under the default params.
#[test]
fn proof_from_components_moves_the_mint_first_group_only_components_into_the_proof() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter: SchnorrFsDeliveryProof<'_, P, E, F> = build_schnorr_fs_delivery_proof(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                SchnorrFsDeliveryProofConstructorParamsOverrides {
                    verifier_form: Some(VerifierGroupArithmetic::FirstGroupOnly),
                },
            );
            let challenge_input = build_transfer_responses::<P>(pairing, Default::default()).x;
            let responses_input = build_mint_responses::<P>(pairing, Default::default());
            let first_messages_input =
                build_mint_second_group_first_messages::<P>(pairing, Default::default());
            let expected_challenge = challenge_input.clone();
            let expected_alpha = responses_input.alpha.clone();
            let expected_r = responses_input.r.clone();
            let expected_rho = responses_input.rho.clone();
            let expected_sigma = responses_input.sigma.clone();
            let expected_hpub = first_messages_input.hpub.clone();
            let expected_d1 = first_messages_input.d1.clone();
            let expected_d2 = first_messages_input.d2.clone();
            let components = DeliveryProofComponents::MintFirstGroupOnly {
                challenge: challenge_input,
                responses: responses_input,
                first_messages: first_messages_input,
            };

            // Act
            let result = adapter.proof_from_components(
                ProofFromComponentsParams,
                build_proof_from_components_payload(components, Default::default()),
            );

            // Assert
            let Ok(ProofFromComponentsSuccessReturn { proof }) = result else {
                panic!("the components form a proof")
            };
            let SchnorrFsProof::MintFirstGroupOnly {
                challenge,
                responses,
                first_messages,
            } = proof
            else {
                panic!("the proof is the mint first-group-only variant")
            };
            let encode_scalar = |scalar: P::Scalar| -> Vec<u8> {
                pairing
                    .encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar })
                    .expect("encoding a scalar succeeds")
                    .bytes
                    .expose()
                    .as_ref()
                    .to_vec()
            };
            let encode_g2 = |point: P::G2| -> Vec<u8> {
                pairing
                    .encode_g2(EncodeG2Params, EncodeG2Payload { point })
                    .expect("encoding a second-group point succeeds")
                    .bytes
                    .as_ref()
                    .to_vec()
            };
            assert_eq!(encode_scalar(challenge), encode_scalar(expected_challenge));
            assert_eq!(
                encode_scalar(responses.alpha),
                encode_scalar(expected_alpha)
            );
            assert_eq!(encode_scalar(responses.r), encode_scalar(expected_r));
            assert_eq!(encode_scalar(responses.rho), encode_scalar(expected_rho));
            assert_eq!(
                encode_scalar(responses.sigma),
                encode_scalar(expected_sigma)
            );
            assert_eq!(encode_g2(first_messages.hpub), encode_g2(expected_hpub));
            assert_eq!(encode_g2(first_messages.d1), encode_g2(expected_d1));
            assert_eq!(encode_g2(first_messages.d2), encode_g2(expected_d2));
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(Default::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
    // proof_from_components_moves_the_mint_first_group_only_components_into_the_proof
}

/// Contract: `proof_from_components` moves the fields of components of the
///   adapter's form into the proof variant of the same relation and form.
/// Arrange: the both-groups adapter; the components `DeliveryProofComponents
///   ::TransferBothGroups` over a challenge and
///   `build_transfer_responses`.
/// Act:     `proof_from_components(ProofFromComponentsParams, payload)`.
/// Assert:  in place, `proof` matches `SchnorrFsProof::TransferBothGroups {
///   challenge, responses }`, each scalar equal to its own input by
///   `encode_scalar`; outside, every factory result `is_ok()`.
/// Collaborators: the pairing, chain-forms, encoding, hash-to-scalar, and
///   randomness families' mock concretes through `create_pairing`,
///   `create_chain_forms`, `create_encoding`, `create_hash_to_scalar`, and
///   `create_random_source` under the default params.
#[test]
fn proof_from_components_moves_the_transfer_both_groups_components_into_the_proof() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter: SchnorrFsDeliveryProof<'_, P, E, F> = build_schnorr_fs_delivery_proof(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let challenge_input = build_mint_responses::<P>(pairing, Default::default()).alpha;
            let responses_input = build_transfer_responses::<P>(pairing, Default::default());
            let expected_challenge = challenge_input.clone();
            let expected_x = responses_input.x.clone();
            let expected_y = responses_input.y.clone();
            let expected_s = responses_input.s.clone();
            let expected_rho = responses_input.rho.clone();
            let expected_sigma = responses_input.sigma.clone();
            let components = DeliveryProofComponents::TransferBothGroups {
                challenge: challenge_input,
                responses: responses_input,
            };

            // Act
            let result = adapter.proof_from_components(
                ProofFromComponentsParams,
                build_proof_from_components_payload(components, Default::default()),
            );

            // Assert
            let Ok(ProofFromComponentsSuccessReturn { proof }) = result else {
                panic!("the components form a proof")
            };
            let SchnorrFsProof::TransferBothGroups {
                challenge,
                responses,
            } = proof
            else {
                panic!("the proof is the transfer both-groups variant")
            };
            let encode_scalar = |scalar: P::Scalar| -> Vec<u8> {
                pairing
                    .encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar })
                    .expect("encoding a scalar succeeds")
                    .bytes
                    .expose()
                    .as_ref()
                    .to_vec()
            };
            assert_eq!(encode_scalar(challenge), encode_scalar(expected_challenge));
            assert_eq!(encode_scalar(responses.x), encode_scalar(expected_x));
            assert_eq!(encode_scalar(responses.y), encode_scalar(expected_y));
            assert_eq!(encode_scalar(responses.s), encode_scalar(expected_s));
            assert_eq!(encode_scalar(responses.rho), encode_scalar(expected_rho));
            assert_eq!(
                encode_scalar(responses.sigma),
                encode_scalar(expected_sigma)
            );
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(Default::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
    // proof_from_components_moves_the_transfer_both_groups_components_into_the_proof
}

/// Contract: `proof_from_components` moves the fields of components of the
///   adapter's form into the proof variant of the same relation and form.
/// Arrange: the first-group-only adapter; the components
///   `DeliveryProofComponents::TransferFirstGroupOnly` over a challenge,
///   `build_transfer_responses`, and
///   `build_transfer_second_group_first_messages`.
/// Act:     `proof_from_components(ProofFromComponentsParams, payload)`.
/// Assert:  in place, `proof` matches `SchnorrFsProof::TransferFirstGroupOnly
///   { challenge, responses, first_messages }`, the scalars equal by
///   `encode_scalar` and each first message by `encode_g2`; outside, every
///   factory result `is_ok()`.
/// Collaborators: the pairing, chain-forms, encoding, hash-to-scalar, and
///   randomness families' mock concretes through `create_pairing`,
///   `create_chain_forms`, `create_encoding`, `create_hash_to_scalar`, and
///   `create_random_source` under the default params.
#[test]
fn proof_from_components_moves_the_transfer_first_group_only_components_into_the_proof() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter: SchnorrFsDeliveryProof<'_, P, E, F> = build_schnorr_fs_delivery_proof(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                SchnorrFsDeliveryProofConstructorParamsOverrides {
                    verifier_form: Some(VerifierGroupArithmetic::FirstGroupOnly),
                },
            );
            let challenge_input = build_mint_responses::<P>(pairing, Default::default()).alpha;
            let responses_input = build_transfer_responses::<P>(pairing, Default::default());
            let first_messages_input =
                build_transfer_second_group_first_messages::<P>(pairing, Default::default());
            let expected_challenge = challenge_input.clone();
            let expected_x = responses_input.x.clone();
            let expected_y = responses_input.y.clone();
            let expected_s = responses_input.s.clone();
            let expected_rho = responses_input.rho.clone();
            let expected_sigma = responses_input.sigma.clone();
            let expected_pk2 = first_messages_input.pk2.clone();
            let expected_d1 = first_messages_input.d1.clone();
            let expected_d2 = first_messages_input.d2.clone();
            let components = DeliveryProofComponents::TransferFirstGroupOnly {
                challenge: challenge_input,
                responses: responses_input,
                first_messages: first_messages_input,
            };

            // Act
            let result = adapter.proof_from_components(
                ProofFromComponentsParams,
                build_proof_from_components_payload(components, Default::default()),
            );

            // Assert
            let Ok(ProofFromComponentsSuccessReturn { proof }) = result else {
                panic!("the components form a proof")
            };
            let SchnorrFsProof::TransferFirstGroupOnly {
                challenge,
                responses,
                first_messages,
            } = proof
            else {
                panic!("the proof is the transfer first-group-only variant")
            };
            let encode_scalar = |scalar: P::Scalar| -> Vec<u8> {
                pairing
                    .encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar })
                    .expect("encoding a scalar succeeds")
                    .bytes
                    .expose()
                    .as_ref()
                    .to_vec()
            };
            let encode_g2 = |point: P::G2| -> Vec<u8> {
                pairing
                    .encode_g2(EncodeG2Params, EncodeG2Payload { point })
                    .expect("encoding a second-group point succeeds")
                    .bytes
                    .as_ref()
                    .to_vec()
            };
            assert_eq!(encode_scalar(challenge), encode_scalar(expected_challenge));
            assert_eq!(encode_scalar(responses.x), encode_scalar(expected_x));
            assert_eq!(encode_scalar(responses.y), encode_scalar(expected_y));
            assert_eq!(encode_scalar(responses.s), encode_scalar(expected_s));
            assert_eq!(encode_scalar(responses.rho), encode_scalar(expected_rho));
            assert_eq!(
                encode_scalar(responses.sigma),
                encode_scalar(expected_sigma)
            );
            assert_eq!(encode_g2(first_messages.pk2), encode_g2(expected_pk2));
            assert_eq!(encode_g2(first_messages.d1), encode_g2(expected_d1));
            assert_eq!(encode_g2(first_messages.d2), encode_g2(expected_d2));
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(Default::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
    // proof_from_components_moves_the_transfer_first_group_only_components_into_the_proof
}

/// Contract: `proof_from_components`, form mismatch: the components' form
///   differs from `self.verifier_form` → `Err(
///   ProofFromComponentsErrorReturn::VerifierFormMismatch)`.
/// Arrange: the both-groups adapter; the components
///   `DeliveryProofComponents::MintFirstGroupOnly`.
/// Act:     `proof_from_components(ProofFromComponentsParams, payload)`.
/// Assert:  in place, the whole error by `assert_eq!`; outside, every
///   factory result `is_ok()`.
/// Collaborators: the pairing, chain-forms, encoding, hash-to-scalar, and
///   randomness families' mock concretes through `create_pairing`,
///   `create_chain_forms`, `create_encoding`, `create_hash_to_scalar`, and
///   `create_random_source` under the default params.
#[test]
fn proof_from_components_refuses_first_group_only_components_under_a_both_groups_adapter() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter: SchnorrFsDeliveryProof<'_, P, E, F> = build_schnorr_fs_delivery_proof(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let components = DeliveryProofComponents::MintFirstGroupOnly {
                challenge: build_transfer_responses::<P>(pairing, Default::default()).x,
                responses: build_mint_responses::<P>(pairing, Default::default()),
                first_messages: build_mint_second_group_first_messages::<P>(
                    pairing,
                    Default::default(),
                ),
            };

            // Act
            let result = adapter.proof_from_components(
                ProofFromComponentsParams,
                build_proof_from_components_payload(components, Default::default()),
            );

            // Assert
            assert_eq!(
                result.err(),
                Some(ProofFromComponentsErrorReturn::VerifierFormMismatch)
            );
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(Default::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
    // proof_from_components_refuses_first_group_only_components_under_a_both_groups_adapter
}

/// Contract: `proof_from_components`, form mismatch: the components' form
///   differs from `self.verifier_form` → `Err(
///   ProofFromComponentsErrorReturn::VerifierFormMismatch)`.
/// Arrange: the first-group-only adapter; the components
///   `DeliveryProofComponents::TransferBothGroups`.
/// Act:     `proof_from_components(ProofFromComponentsParams, payload)`.
/// Assert:  in place, the whole error by `assert_eq!`; outside, every
///   factory result `is_ok()`.
/// Collaborators: the pairing, chain-forms, encoding, hash-to-scalar, and
///   randomness families' mock concretes through `create_pairing`,
///   `create_chain_forms`, `create_encoding`, `create_hash_to_scalar`, and
///   `create_random_source` under the default params.
#[test]
fn proof_from_components_refuses_both_groups_components_under_a_first_group_only_adapter() {
    struct Consumer;

    impl ISchnorrFsCaseConsumer for Consumer {
        type Output = ();

        fn consume_schnorr_fs_case<
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            E: IEncoderAdapter + IDecoderAdapter,
        >(
            &self,
            pairing: &P,
            _forms: &F,
            encoder: &E,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            random: &dyn IRandomSourceAdapter,
            _payload: &SchnorrFsCasePayload,
        ) -> Self::Output {
            // Arrange
            let adapter: SchnorrFsDeliveryProof<'_, P, E, F> = build_schnorr_fs_delivery_proof(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                SchnorrFsDeliveryProofConstructorParamsOverrides {
                    verifier_form: Some(VerifierGroupArithmetic::FirstGroupOnly),
                },
            );
            let components = DeliveryProofComponents::TransferBothGroups {
                challenge: build_mint_responses::<P>(pairing, Default::default()).alpha,
                responses: build_transfer_responses::<P>(pairing, Default::default()),
            };

            // Act
            let result = adapter.proof_from_components(
                ProofFromComponentsParams,
                build_proof_from_components_payload(components, Default::default()),
            );

            // Assert
            assert_eq!(
                result.err(),
                Some(ProofFromComponentsErrorReturn::VerifierFormMismatch)
            );
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(Default::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(result.is_ok());
    // proof_from_components_refuses_both_groups_components_under_a_first_group_only_adapter
}
