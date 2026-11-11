use chain::IChainForms;
use domain::{SecretConstructorParamsOverrides, build_secret};
use encoding::{IDecoderAdapter, IEncoderAdapter};
use envelope::{EnvelopeAlgebra, EnvelopeComponents, PublicKeysComponents};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DomainTagConstructorParamsOverrides, HashToScalarConcrete,
    HashToScalarErrorReturn, IHashToScalarAdapter, MockIHashToScalarAdapterFailureMode,
    build_create_hash_to_scalar_deps, build_create_hash_to_scalar_params,
    build_create_hash_to_scalar_payload, build_domain_tag, create_hash_to_scalar,
};
use pairing::{
    AddScalarParams, AddScalarPayload, ConsumePairingParams, ConsumePairingPayload,
    CreatePairingParamsOverrides, EncodeG2Params, EncodeG2Payload, EncodeScalarParams,
    EncodeScalarPayload, G1GeneratorParams, G1GeneratorPayload, G2GeneratorParams,
    G2GeneratorPayload, IPairingArithmetic, IPairingConsumer, IPairingReference,
    ISampleUniformScalar, MockIPairingAdapterFailureMode, MsmG1Params, MsmG1Payload, MsmG1Term,
    MsmG2Params, MsmG2Payload, MsmG2Term, MulG1Params, MulG1Payload, MulG2Params, MulG2Payload,
    MulScalarParams, MulScalarPayload, NegScalarParams, NegScalarPayload, PairingConcrete,
    SampleUniformScalarErrorReturn, SampleUniformScalarParams, SampleUniformScalarPayload,
    VerifierGroupArithmetic, build_create_pairing_deps, build_create_pairing_params,
    build_create_pairing_payload, create_pairing,
};
use random::{
    CreateRandomSourceParamsOverrides, CreateRandomSourcePayload, FillBytesErrorReturn,
    FillBytesParams, FillBytesPayload, IRandomSourceAdapter, MockIRandomSourceAdapterFailureMode,
    RandomSourceKind, build_create_random_source_deps, build_create_random_source_params,
    create_random_source,
};

use super::challenge::provides::{
    ChallengeDeps, ChallengeErrorReturn, ChallengeParams, ChallengePayload, ChallengeTranscript,
    SCHNORR_FS_CHALLENGE_TAG, challenge,
};
use super::provides::{
    ISchnorrFsCaseConsumer, SCHNORR_FS_WEIGHT_TAG, SchnorrFsCaseParamsOverrides,
    SchnorrFsCasePayload, SchnorrFsCaseSuccessReturn, SchnorrFsDeliveryProof,
    SchnorrFsDeliveryProofConstructorParams, SchnorrFsMintProofOverrides, SchnorrFsProof,
    SchnorrFsProveMintErrorReturn, SchnorrFsProveTransferErrorReturn,
    SchnorrFsTransferProofOverrides, SchnorrFsVerifyErrorReturn, build_schnorr_fs_case_deps,
    build_schnorr_fs_case_params, build_schnorr_fs_case_payload, build_schnorr_fs_delivery_proof,
    build_schnorr_fs_delivery_proof_constructor_params, build_schnorr_fs_mint_proof,
    build_schnorr_fs_transfer_proof, run_schnorr_fs_case,
};
use crate::factory::provides::{
    AlgebraicStatement, AlgebraicTransferStatementOverrides, DeliveryContextOverrides,
    DeliveryProofComponents, IDeliveryProofAdapter, MintResponsesOverrides,
    MintSecondGroupFirstMessagesOverrides, ProofComponentsParams, ProofFromComponentsErrorReturn,
    ProofFromComponentsParams, ProveMintErrorReturn, ProveMintParams, ProveTransferErrorReturn,
    ProveTransferParams, TransferResponsesOverrides, TransferSecondGroupFirstMessagesOverrides,
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

/// Contract: `try_new`, admitted: `Ok(SchnorrFsDeliveryProof { pairing,
///   hash_to_scalar, encoder, random, … })`, holding the params' borrows.
/// Arrange: `build_schnorr_fs_delivery_proof_constructor_params` over the
///   received pairing, the received adapter and source, and the received
///   encoder, each a distinct object.
/// Act:     `SchnorrFsDeliveryProof::<P, E, F>::try_new(params)` inside the
///   consumer's method.
/// Assert:  in place, the result is `Ok` and each of the adapter's four
///   borrows is pointer-identical to the borrow handed in by `core::ptr::eq`;
///   outside, the case return is `Ok`.
/// Collaborators: all five families' mock concretes through the case runner at
///   the default params.
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
                .expect("the tags are admitted by the domain tag type");

            // Assert
            assert!(core::ptr::eq(adapter.pairing, pairing));
            assert!(core::ptr::eq(adapter.hash_to_scalar, hash_to_scalar));
            assert!(core::ptr::eq(adapter.encoder, encoder));
            assert!(core::ptr::eq(adapter.random, random));
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `try_new`, admitted: the adapter holds the params' verifier form.
/// Arrange: the params with `verifier_form` `BothGroups`.
/// Act:     `try_new(params)` inside the consumer's method.
/// Assert:  in place, the result is `Ok` and `matches!(adapter.verifier_form,
///   VerifierGroupArithmetic::BothGroups)`; outside, the case return is `Ok`.
/// Collaborators: all five families' mock concretes through the case runner at
///   the default params.
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
            let params = SchnorrFsDeliveryProofConstructorParams {
                pairing,
                hash_to_scalar,
                encoder,
                random,
                verifier_form: VerifierGroupArithmetic::BothGroups,
            };

            // Act
            let adapter = SchnorrFsDeliveryProof::<P, E, F>::try_new(params)
                .expect("the tags are admitted by the domain tag type");

            // Assert
            assert!(matches!(
                adapter.verifier_form,
                VerifierGroupArithmetic::BothGroups
            ));
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `try_new`, admitted: the adapter holds the params' verifier form.
/// Arrange: the params with `verifier_form` `FirstGroupOnly`, the form the
///   builder does not default to.
/// Act:     `try_new(params)` inside the consumer's method.
/// Assert:  in place, the result is `Ok` and `matches!(adapter.verifier_form,
///   VerifierGroupArithmetic::FirstGroupOnly)`; outside, the case return is
///   `Ok`.
/// Collaborators: all five families' mock concretes through the case runner at
///   the default params.
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
            let params = SchnorrFsDeliveryProofConstructorParams {
                pairing,
                hash_to_scalar,
                encoder,
                random,
                verifier_form: VerifierGroupArithmetic::FirstGroupOnly,
            };

            // Act
            let adapter = SchnorrFsDeliveryProof::<P, E, F>::try_new(params)
                .expect("the tags are admitted by the domain tag type");

            // Assert
            assert!(matches!(
                adapter.verifier_form,
                VerifierGroupArithmetic::FirstGroupOnly
            ));
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `try_new` admits `SCHNORR_FS_CHALLENGE_TAG` through
///   `DomainTag::try_new` → the adapter holds the admitted tag.
/// Arrange: the default params.
/// Act:     `try_new(params)` inside the consumer's method.
/// Assert:  in place, the result is `Ok` and
///   `adapter.challenge_tag.as_bytes()` equals
///   `b"ChainTorrent-v1-proof-challenge"` by `assert_eq!`; outside, the case
///   return is `Ok`.
/// Collaborators: all five families' mock concretes through the case runner at
///   the default params.
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
                .expect("the tags are admitted by the domain tag type");

            // Assert
            assert_eq!(adapter.challenge_tag.as_bytes(), SCHNORR_FS_CHALLENGE_TAG);
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `try_new` admits `SCHNORR_FS_WEIGHT_TAG` through
///   `DomainTag::try_new` → the adapter holds the admitted tag.
/// Arrange: the default params.
/// Act:     `try_new(params)` inside the consumer's method.
/// Assert:  in place, the result is `Ok` and `adapter.weight_tag.as_bytes()`
///   equals `b"ChainTorrent-v1-proof-weight"` by `assert_eq!`; outside, the
///   case return is `Ok`.
/// Collaborators: all five families' mock concretes through the case runner at
///   the default params.
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
                .expect("the tags are admitted by the domain tag type");

            // Assert
            assert_eq!(adapter.weight_tag.as_bytes(), SCHNORR_FS_WEIGHT_TAG);
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `DECLARATION`: `algebras` is `&[EnvelopeAlgebra::PairingElGamal]`.
/// Arrange: the consumer's received `P`, `E`, and `F`; no instance is built.
/// Act:     reading `SchnorrFsDeliveryProof::<'_, P, E, F>::DECLARATION` inside
///   the consumer's method.
/// Assert:  `matches!(DECLARATION.algebras,
///   [EnvelopeAlgebra::PairingElGamal])`; outside, the case return is `Ok`.
/// Collaborators: all five families' mock concretes through the case runner at
///   the default params.
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
            let declaration =
                <SchnorrFsDeliveryProof<'_, P, E, F> as IDeliveryProofAdapter>::DECLARATION;

            // Assert
            assert!(matches!(
                declaration.algebras,
                [EnvelopeAlgebra::PairingElGamal]
            ));
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `DECLARATION`: `verifier_forms` lists both forms, both-groups
///   first.
/// Arrange: as above.
/// Act:     reading `DECLARATION` inside the consumer's method.
/// Assert:  `matches!(DECLARATION.verifier_forms,
///   [VerifierGroupArithmetic::BothGroups,
///   VerifierGroupArithmetic::FirstGroupOnly])`; outside, the case return is
///   `Ok`.
/// Collaborators: all five families' mock concretes through the case runner at
///   the default params.
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
            let declaration =
                <SchnorrFsDeliveryProof<'_, P, E, F> as IDeliveryProofAdapter>::DECLARATION;

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
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `DECLARATION`: `statement_versions` is `&[1]`.
/// Arrange: as above.
/// Act:     reading `DECLARATION` inside the consumer's method.
/// Assert:  `assert_eq!(DECLARATION.statement_versions, &[1u16])`; outside,
///   the case return is `Ok`.
/// Collaborators: all five families' mock concretes through the case runner at
///   the default params.
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
            let declaration =
                <SchnorrFsDeliveryProof<'_, P, E, F> as IDeliveryProofAdapter>::DECLARATION;

            // Assert
            assert_eq!(declaration.statement_versions, &[1u16]);
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `DECLARATION`: `challenge_tag` and `weight_tag` are the proof's
///   two domain tags.
/// Arrange: as above.
/// Act:     reading `DECLARATION` inside the consumer's method.
/// Assert:  `challenge_tag` equals `b"ChainTorrent-v1-proof-challenge"` and
///   `weight_tag` equals `b"ChainTorrent-v1-proof-weight"`, each by
///   `assert_eq!`; outside, the case return is `Ok`.
/// Collaborators: all five families' mock concretes through the case runner at
///   the default params.
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
            let declaration =
                <SchnorrFsDeliveryProof<'_, P, E, F> as IDeliveryProofAdapter>::DECLARATION;

            // Assert
            assert_eq!(declaration.challenge_tag, SCHNORR_FS_CHALLENGE_TAG);
            assert_eq!(declaration.weight_tag, SCHNORR_FS_WEIGHT_TAG);
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `DECLARATION`: `adapter_version` is `1` and `interface_version`
///   is the family's interface version.
/// Arrange: as above.
/// Act:     reading `DECLARATION` inside the consumer's method.
/// Assert:  `assert_eq!(DECLARATION.adapter_version, 1)` and
///   `assert_eq!(DECLARATION.interface_version, 1)`; outside, the case return
///   is `Ok`.
/// Collaborators: all five families' mock concretes through the case runner at
///   the default params.
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
            let declaration =
                <SchnorrFsDeliveryProof<'_, P, E, F> as IDeliveryProofAdapter>::DECLARATION;

            // Assert
            assert_eq!(declaration.adapter_version, 1);
            assert_eq!(declaration.interface_version, 1);
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `prove_mint`, nonce refused: the draw refuses →
///   `Err(ProveMintErrorReturn::SchnorrFs(
///   SchnorrFsProveMintErrorReturn::NonceDraw(error)))`, the refusal
///   unchanged.
/// Arrange: the case params' `random_source` overridden to
///   `build_create_random_source_params` with `kind`
///   `RandomSourceKind::Mock(
///   MockIRandomSourceAdapterFailureMode::FillBytesRefused)`; the both-groups
///   adapter; the default mint statement and payload.
/// Act:     `prove_mint(ProveMintParams, payload)` inside the consumer's
///   method.
/// Assert:  `assert_eq!(result.err(),
///   Some(ProveMintErrorReturn::SchnorrFs(
///   SchnorrFsProveMintErrorReturn::NonceDraw(
///   FillBytesErrorReturn::MockIRandomSourceAdapter))))`; outside, the case
///   return is `Ok`.
/// Collaborators: the randomness family's failure mode named through the case
///   params; the other families' mock concretes at the defaults.
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
            let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let payload = build_prove_mint_payload(pairing, &statement, Default::default());
            let adapter = build_schnorr_fs_delivery_proof::<P, E, F>(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );

            // Act
            let result = adapter.prove_mint(ProveMintParams, payload);

            // Assert
            assert_eq!(
                result.err(),
                Some(ProveMintErrorReturn::SchnorrFs(
                    SchnorrFsProveMintErrorReturn::NonceDraw(
                        FillBytesErrorReturn::MockIRandomSourceAdapter
                    )
                ))
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

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `prove_mint`, nonce refused: the sampling refuses →
///   `Err(ProveMintErrorReturn::SchnorrFs(
///   SchnorrFsProveMintErrorReturn::NonceSampling(error)))`, the refusal
///   unchanged.
/// Arrange: the case params' `pairing` at the defaults; the both-groups
///   adapter, the default mint statement, and the payload built while the
///   default mock concrete's sampling succeeds; then, inside the consumer's
///   method immediately before the subject call, a second `create_pairing`
///   with `concrete` `PairingConcrete::Mock(
///   MockIPairingAdapterFailureMode::SampleScalarWrongLength)` and a consumer
///   that does nothing, whose `try_new` arms the sampling refusal so the
///   subject's first nonce sampling refuses.
/// Act:     `prove_mint(ProveMintParams, payload)` inside the consumer's
///   method.
/// Assert:  `assert_eq!(result.err(),
///   Some(ProveMintErrorReturn::SchnorrFs(
///   SchnorrFsProveMintErrorReturn::NonceSampling(
///   SampleUniformScalarErrorReturn::WrongLength { expected:
///   P::Scalar::UNIFORM_BYTES_LENGTH, actual:
///   P::Scalar::UNIFORM_BYTES_LENGTH }))))`; outside, the case return is `Ok`.
/// Collaborators: the pairing family's failure mode armed inside the case;
///   the other families' mock concretes at the defaults.
#[test]
fn prove_mint_returns_the_nonce_sampling_refusal_unchanged() {
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
            struct ArmingConsumer;

            impl IPairingConsumer for ArmingConsumer {
                type Output = ();

                fn consume_pairing<Q: IPairingArithmetic + IPairingReference>(
                    &self,
                    _params: ConsumePairingParams,
                    _payload: ConsumePairingPayload<Q>,
                ) -> Self::Output {
                }
            }

            let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let payload = build_prove_mint_payload(pairing, &statement, Default::default());
            let adapter = build_schnorr_fs_delivery_proof::<P, E, F>(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let armed = create_pairing(
                &build_create_pairing_deps(ArmingConsumer, Default::default()),
                build_create_pairing_params(CreatePairingParamsOverrides {
                    concrete: Some(PairingConcrete::Mock(
                        MockIPairingAdapterFailureMode::SampleScalarWrongLength,
                    )),
                    ..Default::default()
                }),
                build_create_pairing_payload(Default::default()),
            );
            assert!(armed.is_ok());

            // Act
            let result = adapter.prove_mint(ProveMintParams, payload);

            // Assert
            assert_eq!(
                result.err(),
                Some(ProveMintErrorReturn::SchnorrFs(
                    SchnorrFsProveMintErrorReturn::NonceSampling(
                        SampleUniformScalarErrorReturn::WrongLength {
                            expected: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                            actual: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                        }
                    )
                ))
            );
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `prove_mint`, challenge refused: `challenge` returns
///   `Err(error)` → `Err(ProveMintErrorReturn::SchnorrFs(
///   SchnorrFsProveMintErrorReturn::Challenge(error)))`, the refusal
///   unchanged.
/// Arrange: the case params' `hash_to_scalar` overridden to
///   `build_create_hash_to_scalar_params` with `concrete`
///   `HashToScalarConcrete::Mock(
///   MockIHashToScalarAdapterFailureMode::SamplingRefused)`; the both-groups
///   adapter; the default mint statement and payload.
/// Act:     `prove_mint(ProveMintParams, payload)` inside the consumer's
///   method.
/// Assert:  `assert_eq!(result.err(),
///   Some(ProveMintErrorReturn::SchnorrFs(
///   SchnorrFsProveMintErrorReturn::Challenge(ChallengeErrorReturn::
///   HashToScalar(HashToScalarErrorReturn::Sampling(
///   SampleUniformScalarErrorReturn::WrongLength { expected:
///   P::Scalar::UNIFORM_BYTES_LENGTH, actual: 0 }))))))`; outside, the case
///   return is `Ok`.
/// Collaborators: the hash-to-scalar family's failure mode named through the
///   case params; the other families' mock concretes at the defaults.
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
            let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let payload = build_prove_mint_payload(pairing, &statement, Default::default());
            let adapter = build_schnorr_fs_delivery_proof::<P, E, F>(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );

            // Act
            let result = adapter.prove_mint(ProveMintParams, payload);

            // Assert
            assert_eq!(
                result.err(),
                Some(ProveMintErrorReturn::SchnorrFs(
                    SchnorrFsProveMintErrorReturn::Challenge(ChallengeErrorReturn::HashToScalar(
                        HashToScalarErrorReturn::Sampling(
                            SampleUniformScalarErrorReturn::WrongLength {
                                expected: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                                actual: 0,
                            }
                        )
                    ))
                ))
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

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `prove_mint`, proved: the first messages `T_hpub`, `T_C1`,
///   `T_C2`, `T_D1`, `T_D2` formed from the nonces, the transcript built from
///   the statement and them, the challenge computed over it → the proof's
///   `challenge` is that challenge.
/// Arrange: the both-groups adapter; the default mint statement, whose
///   context values and keys differ at every position, and payload; the
///   expected nonces for `k_α`, `k_r`, `k_ρ`, `k_σ` and the expected first
///   messages and challenge as the expected-values bullet states, so a prover
///   that orders, omits, or alters a first message or a statement field
///   hashes another transcript.
/// Act:     `prove_mint(ProveMintParams, payload)` inside the consumer's
///   method.
/// Assert:  the result is `Ok(ProveMintSuccessReturn { proof })`, `proof`
///   matches `SchnorrFsProof::MintBothGroups { challenge, .. }`, and the
///   encoding of `challenge` equals the encoding of the expected challenge by
///   `assert_eq!`; outside, the case return is `Ok`.
/// Collaborators: all five families' mock concretes through the case runner at
///   the default params; the second randomness source reproduces the nonce
///   stream.
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
            let encode_scalar = |scalar: P::Scalar| -> Vec<u8> {
                let Ok(encoded) =
                    pairing.encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar });
                encoded.bytes.expose().as_ref().to_vec()
            };
            let mul_g1 = |point: P::G1, scalar: P::Scalar| -> P::G1 {
                let Ok(success) = pairing.mul_g1(MulG1Params, MulG1Payload { point, scalar });
                success.product
            };
            let mul_g2 = |point: P::G2, scalar: P::Scalar| -> P::G2 {
                let Ok(success) = pairing.mul_g2(MulG2Params, MulG2Payload { point, scalar });
                success.product
            };
            let msm_g1 = |terms: Vec<(P::G1, P::Scalar)>| -> P::G1 {
                let Ok(success) = pairing.msm_g1(
                    MsmG1Params,
                    MsmG1Payload {
                        terms: terms
                            .into_iter()
                            .map(|(base, scalar)| MsmG1Term { base, scalar })
                            .collect(),
                    },
                );
                success.sum
            };
            let msm_g2 = |terms: Vec<(P::G2, P::Scalar)>| -> P::G2 {
                let Ok(success) = pairing.msm_g2(
                    MsmG2Params,
                    MsmG2Payload {
                        terms: terms
                            .into_iter()
                            .map(|(base, scalar)| MsmG2Term { base, scalar })
                            .collect(),
                    },
                );
                success.sum
            };

            // Arrange
            let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let payload = build_prove_mint_payload(pairing, &statement, Default::default());
            let adapter = build_schnorr_fs_delivery_proof::<P, E, F>(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let second = create_random_source(
                &build_create_random_source_deps(Default::default()),
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            )
            .expect("the second source is admitted");
            let next_nonce = || -> P::Scalar {
                let filled = second
                    .adapter
                    .fill_bytes(
                        FillBytesParams,
                        FillBytesPayload {
                            length: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                        },
                    )
                    .expect("the second source fills");
                P::Scalar::sample_from_uniform_bytes(
                    SampleUniformScalarParams,
                    SampleUniformScalarPayload {
                        uniform: build_secret(
                            filled.bytes.expose().clone(),
                            SecretConstructorParamsOverrides,
                        ),
                    },
                )
                .expect("sampling from uniform bytes of the declared length always succeeds")
                .scalar
                .expose()
                .clone()
            };
            let nonces: Vec<P::Scalar> = (0..4).map(|_| next_nonce()).collect();
            let Ok(g1) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
            let Ok(g2) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
            let expected_first_messages = MintFirstMessages {
                hpub: mul_g2(g2.point.clone(), nonces[0].clone()),
                c1: mul_g1(g1.point.clone(), nonces[2].clone()),
                c2: msm_g1(vec![
                    (g1.point.clone(), nonces[0].clone()),
                    (statement.identity_element.clone(), nonces[1].clone()),
                    (statement.buyer_keys.pk1.clone(), nonces[2].clone()),
                ]),
                d1: mul_g2(g2.point.clone(), nonces[3].clone()),
                d2: msm_g2(vec![
                    (g2.point, nonces[1].clone()),
                    (statement.buyer_keys.pk2.clone(), nonces[3].clone()),
                ]),
            };
            let transcript = build_mint_statement::<P, F>(
                pairing,
                MintStatementOverrides {
                    suite_identifier: Some(statement.context.suite_identifier.clone()),
                    chain: Some(statement.context.chain.clone()),
                    entitlement_contract: Some(statement.context.entitlement_contract.clone()),
                    asset_root: Some(statement.context.asset_root),
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
                    first_messages: Some(expected_first_messages),
                },
            );
            let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                bytes: Some(SCHNORR_FS_CHALLENGE_TAG.to_vec()),
            });
            let expected_challenge = challenge(
                &ChallengeDeps {
                    pairing,
                    encoder,
                    tag: &tag,
                    hash_to_scalar,
                },
                ChallengeParams,
                ChallengePayload {
                    transcript: ChallengeTranscript::Mint(&transcript),
                },
            )
            .expect("the challenge over a formed transcript always computes")
            .challenge;

            // Act
            let result = adapter.prove_mint(ProveMintParams, payload);

            // Assert
            let Ok(success) = result else {
                panic!("prove_mint failed")
            };
            let SchnorrFsProof::MintBothGroups {
                challenge: actual, ..
            } = success.proof
            else {
                panic!("the proof is the both-groups variant")
            };
            assert_eq!(encode_scalar(actual), encode_scalar(expected_challenge));
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `prove_mint`, proved: each response is `k + c · w` for its
///   witness in witness order, `α` from `master_scalar.value`, `r` from
///   `credential_randomness`, `ρ` and `σ` from `coins`.
/// Arrange: the both-groups adapter; the default mint statement and payload,
///   whose four witnesses are distinct from each other; the expected nonces;
///   `c` the challenge the proof carries.
/// Act:     `prove_mint(ProveMintParams, payload)` inside the consumer's
///   method.
/// Assert:  `alpha`, `r`, `rho`, and `sigma` of the proof's `MintResponses`
///   each equal the expected `add_scalar(k, mul_scalar(c, w))` of their own
///   witness by `encode_scalar`, so a swapped witness or a swapped nonce
///   fails; outside, the case return is `Ok`.
/// Collaborators: all five families' mock concretes through the case runner at
///   the default params; the second randomness source reproduces the nonce
///   stream.
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
            let encode_scalar = |scalar: P::Scalar| -> Vec<u8> {
                let Ok(encoded) =
                    pairing.encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar });
                encoded.bytes.expose().as_ref().to_vec()
            };
            let expected_response =
                |nonce: P::Scalar, witness: P::Scalar, c: P::Scalar| -> P::Scalar {
                    let Ok(product) = pairing.mul_scalar(
                        MulScalarParams,
                        MulScalarPayload {
                            left: c,
                            right: witness,
                        },
                    );
                    let Ok(sum) = pairing.add_scalar(
                        AddScalarParams,
                        AddScalarPayload {
                            left: nonce,
                            right: product.product,
                        },
                    );
                    sum.sum
                };

            // Arrange
            let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let payload = build_prove_mint_payload(pairing, &statement, Default::default());
            let adapter = build_schnorr_fs_delivery_proof::<P, E, F>(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let second = create_random_source(
                &build_create_random_source_deps(Default::default()),
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            )
            .expect("the second source is admitted");
            let next_nonce = || -> P::Scalar {
                let filled = second
                    .adapter
                    .fill_bytes(
                        FillBytesParams,
                        FillBytesPayload {
                            length: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                        },
                    )
                    .expect("the second source fills");
                P::Scalar::sample_from_uniform_bytes(
                    SampleUniformScalarParams,
                    SampleUniformScalarPayload {
                        uniform: build_secret(
                            filled.bytes.expose().clone(),
                            SecretConstructorParamsOverrides,
                        ),
                    },
                )
                .expect("sampling from uniform bytes of the declared length always succeeds")
                .scalar
                .expose()
                .clone()
            };
            let nonces: Vec<P::Scalar> = (0..4).map(|_| next_nonce()).collect();
            let witnesses = [
                payload.master_scalar.value.expose().clone(),
                payload.credential_randomness.expose().clone(),
                payload.coins.rho.expose().clone(),
                payload.coins.sigma.expose().clone(),
            ];

            // Act
            let result = adapter.prove_mint(ProveMintParams, payload);

            // Assert
            let Ok(success) = result else {
                panic!("prove_mint failed")
            };
            let SchnorrFsProof::MintBothGroups {
                challenge,
                responses,
            } = success.proof
            else {
                panic!("the proof is the both-groups variant")
            };
            let actual = [responses.alpha, responses.r, responses.rho, responses.sigma];
            for (index, actual) in actual.into_iter().enumerate() {
                let expected = expected_response(
                    nonces[index].clone(),
                    witnesses[index].clone(),
                    challenge.clone(),
                );
                assert_eq!(encode_scalar(actual), encode_scalar(expected));
            }
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `prove_mint`, proved, first-group-only form: the proof carries
///   `[T_hpub, T_D1, T_D2]`.
/// Arrange: the adapter constructed with `verifier_form` `FirstGroupOnly`; the
///   default mint statement and payload; the expected `T_hpub`, `T_D1`, and
///   `T_D2` from the expected nonces.
/// Act:     `prove_mint(ProveMintParams, payload)` inside the consumer's
///   method.
/// Assert:  `proof` matches `SchnorrFsProof::MintFirstGroupOnly {
///   first_messages, .. }` and `hpub`, `d1`, and `d2` of `first_messages` each
///   equal the expected message by `encode_g2`; outside, the case return is
///   `Ok`.
/// Collaborators: all five families' mock concretes through the case runner at
///   the default params; the second randomness source reproduces the nonce
///   stream.
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
            let encode_g2 = |point: P::G2| -> Vec<u8> {
                let Ok(encoded) = pairing.encode_g2(EncodeG2Params, EncodeG2Payload { point });
                encoded.bytes.as_ref().to_vec()
            };

            // Arrange
            let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let payload = build_prove_mint_payload(pairing, &statement, Default::default());
            let adapter = SchnorrFsDeliveryProof::<P, E, F>::try_new(
                SchnorrFsDeliveryProofConstructorParams {
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    verifier_form: VerifierGroupArithmetic::FirstGroupOnly,
                },
            )
            .expect("the tags are admitted by the domain tag type");
            let second = create_random_source(
                &build_create_random_source_deps(Default::default()),
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            )
            .expect("the second source is admitted");
            let next_nonce = || -> P::Scalar {
                let filled = second
                    .adapter
                    .fill_bytes(
                        FillBytesParams,
                        FillBytesPayload {
                            length: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                        },
                    )
                    .expect("the second source fills");
                P::Scalar::sample_from_uniform_bytes(
                    SampleUniformScalarParams,
                    SampleUniformScalarPayload {
                        uniform: build_secret(
                            filled.bytes.expose().clone(),
                            SecretConstructorParamsOverrides,
                        ),
                    },
                )
                .expect("sampling from uniform bytes of the declared length always succeeds")
                .scalar
                .expose()
                .clone()
            };
            let nonces: Vec<P::Scalar> = (0..4).map(|_| next_nonce()).collect();
            let Ok(g2) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
            let mul_g2 = |scalar: P::Scalar| -> P::G2 {
                let Ok(success) = pairing.mul_g2(
                    MulG2Params,
                    MulG2Payload {
                        point: g2.point.clone(),
                        scalar,
                    },
                );
                success.product
            };
            let expected_hpub = mul_g2(nonces[0].clone());
            let expected_d1 = mul_g2(nonces[3].clone());
            let expected_d2 = {
                let Ok(success) = pairing.msm_g2(
                    MsmG2Params,
                    MsmG2Payload {
                        terms: vec![
                            MsmG2Term {
                                base: g2.point.clone(),
                                scalar: nonces[1].clone(),
                            },
                            MsmG2Term {
                                base: statement.buyer_keys.pk2.clone(),
                                scalar: nonces[3].clone(),
                            },
                        ],
                    },
                );
                success.sum
            };

            // Act
            let result = adapter.prove_mint(ProveMintParams, payload);

            // Assert
            let Ok(success) = result else {
                panic!("prove_mint failed")
            };
            let SchnorrFsProof::MintFirstGroupOnly { first_messages, .. } = success.proof else {
                panic!("the proof is the first-group-only variant")
            };
            assert_eq!(encode_g2(first_messages.hpub), encode_g2(expected_hpub));
            assert_eq!(encode_g2(first_messages.d1), encode_g2(expected_d1));
            assert_eq!(encode_g2(first_messages.d2), encode_g2(expected_d2));
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `prove_mint`, proved, both-groups form: the proof is the
///   both-groups variant, holding the challenge and the responses alone.
/// Arrange: the adapter constructed with `verifier_form` `BothGroups`; the
///   default mint statement and payload.
/// Act:     `prove_mint(ProveMintParams, payload)` inside the consumer's
///   method.
/// Assert:  `matches!(proof, SchnorrFsProof::MintBothGroups { .. })`; outside,
///   the case return is `Ok`.
/// Collaborators: all five families' mock concretes through the case runner at
///   the default params.
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
            let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let payload = build_prove_mint_payload(pairing, &statement, Default::default());
            let adapter = build_schnorr_fs_delivery_proof::<P, E, F>(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );

            // Act
            let result = adapter.prove_mint(ProveMintParams, payload);

            // Assert
            let Ok(success) = result else {
                panic!("prove_mint failed")
            };
            assert!(matches!(
                success.proof,
                SchnorrFsProof::MintBothGroups { .. }
            ));
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `prove_transfer`, nonce refused: the draw refuses →
///   `Err(ProveTransferErrorReturn::SchnorrFs(
///   SchnorrFsProveTransferErrorReturn::NonceDraw(error)))`, the refusal
///   unchanged.
/// Arrange: the case params' `random_source` overridden to
///   `build_create_random_source_params` with `kind`
///   `RandomSourceKind::Mock(
///   MockIRandomSourceAdapterFailureMode::FillBytesRefused)`; the both-groups
///   adapter; the default transfer statement and payload.
/// Act:     `prove_transfer(ProveTransferParams, payload)` inside the
///   consumer's method.
/// Assert:  `assert_eq!(result.err(),
///   Some(ProveTransferErrorReturn::SchnorrFs(
///   SchnorrFsProveTransferErrorReturn::NonceDraw(
///   FillBytesErrorReturn::MockIRandomSourceAdapter))))`; outside, the case
///   return is `Ok`.
/// Collaborators: the randomness family's failure mode named through the case
///   params; the other families' mock concretes at the defaults.
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
            let statement = build_algebraic_transfer_statement::<P, F>(pairing, Default::default());
            let payload = build_prove_transfer_payload(pairing, &statement, Default::default());
            let adapter = build_schnorr_fs_delivery_proof::<P, E, F>(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );

            // Act
            let result = adapter.prove_transfer(ProveTransferParams, payload);

            // Assert
            assert_eq!(
                result.err(),
                Some(ProveTransferErrorReturn::SchnorrFs(
                    SchnorrFsProveTransferErrorReturn::NonceDraw(
                        FillBytesErrorReturn::MockIRandomSourceAdapter
                    )
                ))
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

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `prove_transfer`, nonce refused: the sampling refuses →
///   `Err(ProveTransferErrorReturn::SchnorrFs(
///   SchnorrFsProveTransferErrorReturn::NonceSampling(error)))`, the refusal
///   unchanged.
/// Arrange: the case params' `pairing` at the defaults; the both-groups
///   adapter, the default transfer statement, and the payload built while the
///   default mock concrete's sampling succeeds; then, inside the consumer's
///   method immediately before the subject call, a second `create_pairing`
///   with `concrete` `PairingConcrete::Mock(
///   MockIPairingAdapterFailureMode::SampleScalarWrongLength)` and a consumer
///   that does nothing, whose `try_new` arms the sampling refusal so the
///   subject's first nonce sampling refuses.
/// Act:     `prove_transfer(ProveTransferParams, payload)` inside the
///   consumer's method.
/// Assert:  `assert_eq!(result.err(),
///   Some(ProveTransferErrorReturn::SchnorrFs(
///   SchnorrFsProveTransferErrorReturn::NonceSampling(
///   SampleUniformScalarErrorReturn::WrongLength { expected:
///   P::Scalar::UNIFORM_BYTES_LENGTH, actual:
///   P::Scalar::UNIFORM_BYTES_LENGTH }))))`; outside, the case return is `Ok`.
/// Collaborators: the pairing family's failure mode armed inside the case;
///   the other families' mock concretes at the defaults.
#[test]
fn prove_transfer_returns_the_nonce_sampling_refusal_unchanged() {
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
            struct ArmingConsumer;

            impl IPairingConsumer for ArmingConsumer {
                type Output = ();

                fn consume_pairing<Q: IPairingArithmetic + IPairingReference>(
                    &self,
                    _params: ConsumePairingParams,
                    _payload: ConsumePairingPayload<Q>,
                ) -> Self::Output {
                }
            }

            let statement = build_algebraic_transfer_statement::<P, F>(pairing, Default::default());
            let payload = build_prove_transfer_payload(pairing, &statement, Default::default());
            let adapter = build_schnorr_fs_delivery_proof::<P, E, F>(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let armed = create_pairing(
                &build_create_pairing_deps(ArmingConsumer, Default::default()),
                build_create_pairing_params(CreatePairingParamsOverrides {
                    concrete: Some(PairingConcrete::Mock(
                        MockIPairingAdapterFailureMode::SampleScalarWrongLength,
                    )),
                    ..Default::default()
                }),
                build_create_pairing_payload(Default::default()),
            );
            assert!(armed.is_ok());

            // Act
            let result = adapter.prove_transfer(ProveTransferParams, payload);

            // Assert
            assert_eq!(
                result.err(),
                Some(ProveTransferErrorReturn::SchnorrFs(
                    SchnorrFsProveTransferErrorReturn::NonceSampling(
                        SampleUniformScalarErrorReturn::WrongLength {
                            expected: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                            actual: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                        }
                    )
                ))
            );
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `prove_transfer`, challenge refused: `challenge` returns
///   `Err(error)` → `Err(ProveTransferErrorReturn::SchnorrFs(
///   SchnorrFsProveTransferErrorReturn::Challenge(error)))`, the refusal
///   unchanged.
/// Arrange: the case params' `hash_to_scalar` overridden to
///   `build_create_hash_to_scalar_params` with `concrete`
///   `HashToScalarConcrete::Mock(
///   MockIHashToScalarAdapterFailureMode::SamplingRefused)`; the both-groups
///   adapter; the default transfer statement and payload.
/// Act:     `prove_transfer(ProveTransferParams, payload)` inside the
///   consumer's method.
/// Assert:  `assert_eq!(result.err(),
///   Some(ProveTransferErrorReturn::SchnorrFs(
///   SchnorrFsProveTransferErrorReturn::Challenge(ChallengeErrorReturn::
///   HashToScalar(HashToScalarErrorReturn::Sampling(
///   SampleUniformScalarErrorReturn::WrongLength { expected:
///   P::Scalar::UNIFORM_BYTES_LENGTH, actual: 0 }))))))`; outside, the case
///   return is `Ok`.
/// Collaborators: the hash-to-scalar family's failure mode named through the
///   case params; the other families' mock concretes at the defaults.
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
            let statement = build_algebraic_transfer_statement::<P, F>(pairing, Default::default());
            let payload = build_prove_transfer_payload(pairing, &statement, Default::default());
            let adapter = build_schnorr_fs_delivery_proof::<P, E, F>(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );

            // Act
            let result = adapter.prove_transfer(ProveTransferParams, payload);

            // Assert
            assert_eq!(
                result.err(),
                Some(ProveTransferErrorReturn::SchnorrFs(
                    SchnorrFsProveTransferErrorReturn::Challenge(
                        ChallengeErrorReturn::HashToScalar(HashToScalarErrorReturn::Sampling(
                            SampleUniformScalarErrorReturn::WrongLength {
                                expected: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                                actual: 0,
                            }
                        ))
                    )
                ))
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

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `prove_transfer`, proved: the first messages `T_pk1`, `T_C1`,
///   `T_C2`, `T_pk2`, `T_D1`, `T_D2` formed from the nonces, the transcript
///   built from the statement and them, the challenge computed over it → the
///   proof's `challenge` is that challenge.
/// Arrange: the both-groups adapter; the transfer statement
///   `build_algebraic_transfer_statement` with its context purpose
///   `TransferPurpose::Grant`, the purpose the builder does not default to,
///   and the default payload; the expected nonces for `k_x`, `k_y`, `k_s`,
///   `k_ρ`, `k_σ`, the expected first messages with `neg_scalar` of `k_x` and
///   `k_y` on the old envelope's `C1_o` and `D1_o`, and the expected
///   challenge as the expected-values bullet states.
/// Act:     `prove_transfer(ProveTransferParams, payload)` inside the
///   consumer's method.
/// Assert:  `proof` matches `SchnorrFsProof::TransferBothGroups { challenge,
///   .. }` and the encoding of `challenge` equals the encoding of the
///   expected challenge by `assert_eq!`; outside, the case return is `Ok`.
/// Collaborators: all five families' mock concretes through the case runner at
///   the default params; the second randomness source reproduces the nonce
///   stream.
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
            let encode_scalar = |scalar: P::Scalar| -> Vec<u8> {
                let Ok(encoded) =
                    pairing.encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar });
                encoded.bytes.expose().as_ref().to_vec()
            };
            let neg = |scalar: P::Scalar| -> P::Scalar {
                let Ok(n) = pairing.neg_scalar(NegScalarParams, NegScalarPayload { scalar });
                n.negation
            };
            let mul_g1 = |point: P::G1, scalar: P::Scalar| -> P::G1 {
                let Ok(success) = pairing.mul_g1(MulG1Params, MulG1Payload { point, scalar });
                success.product
            };
            let mul_g2 = |point: P::G2, scalar: P::Scalar| -> P::G2 {
                let Ok(success) = pairing.mul_g2(MulG2Params, MulG2Payload { point, scalar });
                success.product
            };
            let msm_g1 = |terms: Vec<(P::G1, P::Scalar)>| -> P::G1 {
                let Ok(success) = pairing.msm_g1(
                    MsmG1Params,
                    MsmG1Payload {
                        terms: terms
                            .into_iter()
                            .map(|(base, scalar)| MsmG1Term { base, scalar })
                            .collect(),
                    },
                );
                success.sum
            };
            let msm_g2 = |terms: Vec<(P::G2, P::Scalar)>| -> P::G2 {
                let Ok(success) = pairing.msm_g2(
                    MsmG2Params,
                    MsmG2Payload {
                        terms: terms
                            .into_iter()
                            .map(|(base, scalar)| MsmG2Term { base, scalar })
                            .collect(),
                    },
                );
                success.sum
            };

            // Arrange
            let statement = build_algebraic_transfer_statement::<P, F>(
                pairing,
                AlgebraicTransferStatementOverrides {
                    context: Some(build_delivery_context(
                        TransferPurpose::Grant,
                        DeliveryContextOverrides::default(),
                    )),
                    ..Default::default()
                },
            );
            let payload = build_prove_transfer_payload(pairing, &statement, Default::default());
            let adapter = build_schnorr_fs_delivery_proof::<P, E, F>(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let second = create_random_source(
                &build_create_random_source_deps(Default::default()),
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            )
            .expect("the second source is admitted");
            let next_nonce = || -> P::Scalar {
                let filled = second
                    .adapter
                    .fill_bytes(
                        FillBytesParams,
                        FillBytesPayload {
                            length: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                        },
                    )
                    .expect("the second source fills");
                P::Scalar::sample_from_uniform_bytes(
                    SampleUniformScalarParams,
                    SampleUniformScalarPayload {
                        uniform: build_secret(
                            filled.bytes.expose().clone(),
                            SecretConstructorParamsOverrides,
                        ),
                    },
                )
                .expect("sampling from uniform bytes of the declared length always succeeds")
                .scalar
                .expose()
                .clone()
            };
            let nonces: Vec<P::Scalar> = (0..5).map(|_| next_nonce()).collect();
            let Ok(g1) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
            let Ok(g2) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
            let expected_first_messages = TransferFirstMessages {
                pk1: mul_g1(g1.point.clone(), nonces[0].clone()),
                c1: mul_g1(g1.point.clone(), nonces[3].clone()),
                c2: msm_g1(vec![
                    (statement.old_envelope.c1.clone(), neg(nonces[0].clone())),
                    (statement.identity_element.clone(), nonces[2].clone()),
                    (statement.buyer_keys.pk1.clone(), nonces[3].clone()),
                ]),
                pk2: mul_g2(g2.point.clone(), nonces[1].clone()),
                d1: mul_g2(g2.point.clone(), nonces[4].clone()),
                d2: msm_g2(vec![
                    (statement.old_envelope.d1.clone(), neg(nonces[1].clone())),
                    (g2.point, nonces[2].clone()),
                    (statement.buyer_keys.pk2.clone(), nonces[4].clone()),
                ]),
            };
            let transcript = build_transfer_statement::<P, F>(
                pairing,
                TransferStatementOverrides {
                    suite_identifier: Some(statement.context.suite_identifier.clone()),
                    chain: Some(statement.context.chain.clone()),
                    entitlement_contract: Some(statement.context.entitlement_contract.clone()),
                    asset_root: Some(statement.context.asset_root),
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
                    first_messages: Some(expected_first_messages),
                },
            );
            let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                bytes: Some(SCHNORR_FS_CHALLENGE_TAG.to_vec()),
            });
            let expected_challenge = challenge(
                &ChallengeDeps {
                    pairing,
                    encoder,
                    tag: &tag,
                    hash_to_scalar,
                },
                ChallengeParams,
                ChallengePayload {
                    transcript: ChallengeTranscript::Transfer(&transcript),
                },
            )
            .expect("the challenge over a formed transcript always computes")
            .challenge;

            // Act
            let result = adapter.prove_transfer(ProveTransferParams, payload);

            // Assert
            let Ok(success) = result else {
                panic!("prove_transfer failed")
            };
            let SchnorrFsProof::TransferBothGroups {
                challenge: actual, ..
            } = success.proof
            else {
                panic!("the proof is the both-groups variant")
            };
            assert_eq!(encode_scalar(actual), encode_scalar(expected_challenge));
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `prove_transfer`, proved: each response is `k + c · w` for its
///   witness in witness order, `x_s` and `y_s` from `seller_secrets`, `s` from
///   `offset`, `ρ` and `σ` from `coins`.
/// Arrange: the both-groups adapter; the default transfer statement and
///   payload, whose five witnesses are distinct from each other; the expected
///   nonces; `c` the challenge the proof carries.
/// Act:     `prove_transfer(ProveTransferParams, payload)` inside the
///   consumer's method.
/// Assert:  `x`, `y`, `s`, `rho`, and `sigma` of the proof's
///   `TransferResponses` each equal the expected `add_scalar(k, mul_scalar(c,
///   w))` of their own witness by `encode_scalar`; outside, the case return is
///   `Ok`.
/// Collaborators: all five families' mock concretes through the case runner at
///   the default params; the second randomness source reproduces the nonce
///   stream.
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
            let encode_scalar = |scalar: P::Scalar| -> Vec<u8> {
                let Ok(encoded) =
                    pairing.encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar });
                encoded.bytes.expose().as_ref().to_vec()
            };
            let expected_response =
                |nonce: P::Scalar, witness: P::Scalar, c: P::Scalar| -> P::Scalar {
                    let Ok(product) = pairing.mul_scalar(
                        MulScalarParams,
                        MulScalarPayload {
                            left: c,
                            right: witness,
                        },
                    );
                    let Ok(sum) = pairing.add_scalar(
                        AddScalarParams,
                        AddScalarPayload {
                            left: nonce,
                            right: product.product,
                        },
                    );
                    sum.sum
                };

            // Arrange
            let statement = build_algebraic_transfer_statement::<P, F>(pairing, Default::default());
            let payload = build_prove_transfer_payload(pairing, &statement, Default::default());
            let adapter = build_schnorr_fs_delivery_proof::<P, E, F>(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let second = create_random_source(
                &build_create_random_source_deps(Default::default()),
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            )
            .expect("the second source is admitted");
            let next_nonce = || -> P::Scalar {
                let filled = second
                    .adapter
                    .fill_bytes(
                        FillBytesParams,
                        FillBytesPayload {
                            length: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                        },
                    )
                    .expect("the second source fills");
                P::Scalar::sample_from_uniform_bytes(
                    SampleUniformScalarParams,
                    SampleUniformScalarPayload {
                        uniform: build_secret(
                            filled.bytes.expose().clone(),
                            SecretConstructorParamsOverrides,
                        ),
                    },
                )
                .expect("sampling from uniform bytes of the declared length always succeeds")
                .scalar
                .expose()
                .clone()
            };
            let nonces: Vec<P::Scalar> = (0..5).map(|_| next_nonce()).collect();
            let witnesses = [
                payload.seller_secrets.x.expose().clone(),
                payload.seller_secrets.y.expose().clone(),
                payload.offset.expose().clone(),
                payload.coins.rho.expose().clone(),
                payload.coins.sigma.expose().clone(),
            ];

            // Act
            let result = adapter.prove_transfer(ProveTransferParams, payload);

            // Assert
            let Ok(success) = result else {
                panic!("prove_transfer failed")
            };
            let SchnorrFsProof::TransferBothGroups {
                challenge,
                responses,
            } = success.proof
            else {
                panic!("the proof is the both-groups variant")
            };
            let actual = [
                responses.x,
                responses.y,
                responses.s,
                responses.rho,
                responses.sigma,
            ];
            for (index, actual) in actual.into_iter().enumerate() {
                let expected = expected_response(
                    nonces[index].clone(),
                    witnesses[index].clone(),
                    challenge.clone(),
                );
                assert_eq!(encode_scalar(actual), encode_scalar(expected));
            }
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `prove_transfer`, proved, first-group-only form: the proof
///   carries `[T_pk2, T_D1, T_D2]`.
/// Arrange: the adapter constructed with `verifier_form` `FirstGroupOnly`; the
///   default transfer statement and payload; the expected `T_pk2`, `T_D1`,
///   and `T_D2` from the expected nonces.
/// Act:     `prove_transfer(ProveTransferParams, payload)` inside the
///   consumer's method.
/// Assert:  `proof` matches `SchnorrFsProof::TransferFirstGroupOnly {
///   first_messages, .. }` and each of the three fields of `first_messages`
///   equals the expected message at its position by `encode_g2`; outside, the
///   case return is `Ok`.
/// Collaborators: all five families' mock concretes through the case runner at
///   the default params; the second randomness source reproduces the nonce
///   stream.
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
            let encode_g2 = |point: P::G2| -> Vec<u8> {
                let Ok(encoded) = pairing.encode_g2(EncodeG2Params, EncodeG2Payload { point });
                encoded.bytes.as_ref().to_vec()
            };
            let neg = |scalar: P::Scalar| -> P::Scalar {
                let Ok(n) = pairing.neg_scalar(NegScalarParams, NegScalarPayload { scalar });
                n.negation
            };

            // Arrange
            let statement = build_algebraic_transfer_statement::<P, F>(pairing, Default::default());
            let payload = build_prove_transfer_payload(pairing, &statement, Default::default());
            let adapter = SchnorrFsDeliveryProof::<P, E, F>::try_new(
                SchnorrFsDeliveryProofConstructorParams {
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    verifier_form: VerifierGroupArithmetic::FirstGroupOnly,
                },
            )
            .expect("the tags are admitted by the domain tag type");
            let second = create_random_source(
                &build_create_random_source_deps(Default::default()),
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            )
            .expect("the second source is admitted");
            let next_nonce = || -> P::Scalar {
                let filled = second
                    .adapter
                    .fill_bytes(
                        FillBytesParams,
                        FillBytesPayload {
                            length: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                        },
                    )
                    .expect("the second source fills");
                P::Scalar::sample_from_uniform_bytes(
                    SampleUniformScalarParams,
                    SampleUniformScalarPayload {
                        uniform: build_secret(
                            filled.bytes.expose().clone(),
                            SecretConstructorParamsOverrides,
                        ),
                    },
                )
                .expect("sampling from uniform bytes of the declared length always succeeds")
                .scalar
                .expose()
                .clone()
            };
            let nonces: Vec<P::Scalar> = (0..5).map(|_| next_nonce()).collect();
            let Ok(g2) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
            let mul_g2 = |scalar: P::Scalar| -> P::G2 {
                let Ok(success) = pairing.mul_g2(
                    MulG2Params,
                    MulG2Payload {
                        point: g2.point.clone(),
                        scalar,
                    },
                );
                success.product
            };
            let expected_pk2 = mul_g2(nonces[1].clone());
            let expected_d1 = mul_g2(nonces[4].clone());
            let expected_d2 = {
                let Ok(success) = pairing.msm_g2(
                    MsmG2Params,
                    MsmG2Payload {
                        terms: vec![
                            MsmG2Term {
                                base: statement.old_envelope.d1.clone(),
                                scalar: neg(nonces[1].clone()),
                            },
                            MsmG2Term {
                                base: g2.point.clone(),
                                scalar: nonces[2].clone(),
                            },
                            MsmG2Term {
                                base: statement.buyer_keys.pk2.clone(),
                                scalar: nonces[4].clone(),
                            },
                        ],
                    },
                );
                success.sum
            };

            // Act
            let result = adapter.prove_transfer(ProveTransferParams, payload);

            // Assert
            let Ok(success) = result else {
                panic!("prove_transfer failed")
            };
            let SchnorrFsProof::TransferFirstGroupOnly { first_messages, .. } = success.proof
            else {
                panic!("the proof is the first-group-only variant")
            };
            assert_eq!(encode_g2(first_messages.pk2), encode_g2(expected_pk2));
            assert_eq!(encode_g2(first_messages.d1), encode_g2(expected_d1));
            assert_eq!(encode_g2(first_messages.d2), encode_g2(expected_d2));
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `prove_transfer`, proved, both-groups form: the proof is the
///   both-groups variant, holding the challenge and the responses alone.
/// Arrange: the adapter constructed with `verifier_form` `BothGroups`; the
///   default transfer statement and payload.
/// Act:     `prove_transfer(ProveTransferParams, payload)` inside the
///   consumer's method.
/// Assert:  `matches!(proof, SchnorrFsProof::TransferBothGroups { .. })`;
///   outside, the case return is `Ok`.
/// Collaborators: all five families' mock concretes through the case runner at
///   the default params.
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
            let statement = build_algebraic_transfer_statement::<P, F>(pairing, Default::default());
            let payload = build_prove_transfer_payload(pairing, &statement, Default::default());
            let adapter = build_schnorr_fs_delivery_proof::<P, E, F>(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );

            // Act
            let result = adapter.prove_transfer(ProveTransferParams, payload);

            // Assert
            let Ok(success) = result else {
                panic!("prove_transfer failed")
            };
            assert!(matches!(
                success.proof,
                SchnorrFsProof::TransferBothGroups { .. }
            ));
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `verify` rejects a proof variant whose relation differs from
///   its statement → `Ok(VerifySuccessReturn { is_valid: false })`.
/// Arrange: the both-groups adapter; the default transfer statement and
///   payload, and `build_schnorr_fs_transfer_proof` over them; the default
///   mint statement.
/// Act:     `verify(VerifyParams, payload)` over
///   `AlgebraicStatement::Mint(&mint_statement)` and the transfer proof.
/// Assert:  `result.ok().map(|success| success.is_valid)` equals
///   `Some(false)`; outside, the case return is `Ok`.
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
            let transfer_statement =
                build_algebraic_transfer_statement::<P, F>(pairing, Default::default());
            let transfer_payload =
                build_prove_transfer_payload(pairing, &transfer_statement, Default::default());
            let adapter = build_schnorr_fs_delivery_proof::<P, E, F>(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let proof = build_schnorr_fs_transfer_proof(
                &adapter,
                &transfer_payload,
                SchnorrFsTransferProofOverrides::default(),
            );
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
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `verify` rejects a proof variant whose verifier form differs
///   from the adapter → `Ok(VerifySuccessReturn { is_valid: false })`.
/// Arrange: the subject's adapter constructed in the both-groups form; a
///   second adapter over the same collaborators constructed in the
///   first-group-only form; the default mint statement and payload; the proof
///   from `build_schnorr_fs_mint_proof` over the second adapter.
/// Act:     `verify(VerifyParams, payload)` on the subject's adapter over
///   `AlgebraicStatement::Mint(&statement)` and that proof.
/// Assert:  `result.ok().map(|success| success.is_valid)` equals
///   `Some(false)`; outside, the case return is `Ok`.
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
            let subject = SchnorrFsDeliveryProof::<P, E, F>::try_new(
                SchnorrFsDeliveryProofConstructorParams {
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    verifier_form: VerifierGroupArithmetic::BothGroups,
                },
            )
            .expect("the tags are admitted by the domain tag type");
            let other = SchnorrFsDeliveryProof::<P, E, F>::try_new(
                SchnorrFsDeliveryProofConstructorParams {
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    verifier_form: VerifierGroupArithmetic::FirstGroupOnly,
                },
            )
            .expect("the tags are admitted by the domain tag type");
            let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let prove_payload = build_prove_mint_payload(pairing, &statement, Default::default());
            let proof = build_schnorr_fs_mint_proof(
                &other,
                &prove_payload,
                SchnorrFsMintProofOverrides::default(),
            );

            // Act
            let result = subject.verify(
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
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `verify`, both-groups form, mint: the first messages recomputed
///   from the responses and challenge reproduce the challenge → `is_valid`
///   true.
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
            let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let prove_payload = build_prove_mint_payload(pairing, &statement, Default::default());
            let adapter = build_schnorr_fs_delivery_proof::<P, E, F>(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let proof = build_schnorr_fs_mint_proof(
                &adapter,
                &prove_payload,
                SchnorrFsMintProofOverrides::default(),
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
            assert_eq!(result.ok().map(|success| success.is_valid), Some(true));
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `verify`, both-groups form, mint: a changed response yields
///   another challenge → `is_valid` false.
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
            let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let prove_payload = build_prove_mint_payload(pairing, &statement, Default::default());
            let adapter = build_schnorr_fs_delivery_proof::<P, E, F>(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let responses = build_mint_responses(
                pairing,
                MintResponsesOverrides {
                    alpha: Some(build_transfer_responses(pairing, Default::default()).x),
                    ..Default::default()
                },
            );
            let proof = build_schnorr_fs_mint_proof(
                &adapter,
                &prove_payload,
                SchnorrFsMintProofOverrides {
                    responses: Some(responses),
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
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `verify`, both-groups form, transfer: the first messages
///   recomputed from the responses and challenge reproduce the challenge →
///   `is_valid` true.
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
            let statement = build_algebraic_transfer_statement::<P, F>(pairing, Default::default());
            let prove_payload =
                build_prove_transfer_payload(pairing, &statement, Default::default());
            let adapter = build_schnorr_fs_delivery_proof::<P, E, F>(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let proof = build_schnorr_fs_transfer_proof(
                &adapter,
                &prove_payload,
                SchnorrFsTransferProofOverrides::default(),
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
            assert_eq!(result.ok().map(|success| success.is_valid), Some(true));
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `verify`, both-groups form, transfer: a changed response yields
///   another challenge → `is_valid` false.
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
            let statement = build_algebraic_transfer_statement::<P, F>(pairing, Default::default());
            let prove_payload =
                build_prove_transfer_payload(pairing, &statement, Default::default());
            let adapter = build_schnorr_fs_delivery_proof::<P, E, F>(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let responses = build_transfer_responses(
                pairing,
                TransferResponsesOverrides {
                    s: Some(build_mint_responses(pairing, Default::default()).alpha),
                    ..Default::default()
                },
            );
            let proof = build_schnorr_fs_transfer_proof(
                &adapter,
                &prove_payload,
                SchnorrFsTransferProofOverrides {
                    responses: Some(responses),
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
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `verify`, first-group-only form, mint: the challenge reproduces
///   and the weighted pairing product is one → `is_valid` true.
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
            let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let prove_payload = build_prove_mint_payload(pairing, &statement, Default::default());
            let adapter = SchnorrFsDeliveryProof::<P, E, F>::try_new(
                SchnorrFsDeliveryProofConstructorParams {
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    verifier_form: VerifierGroupArithmetic::FirstGroupOnly,
                },
            )
            .expect("the tags are admitted by the domain tag type");
            let proof = build_schnorr_fs_mint_proof(
                &adapter,
                &prove_payload,
                SchnorrFsMintProofOverrides::default(),
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
            assert_eq!(result.ok().map(|success| success.is_valid), Some(true));
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `verify`, first-group-only form, challenge: the recomputed
///   challenge differs from the proof's → `is_valid` false.
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
            let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let prove_payload = build_prove_mint_payload(pairing, &statement, Default::default());
            let adapter = SchnorrFsDeliveryProof::<P, E, F>::try_new(
                SchnorrFsDeliveryProofConstructorParams {
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    verifier_form: VerifierGroupArithmetic::FirstGroupOnly,
                },
            )
            .expect("the tags are admitted by the domain tag type");
            let proof = build_schnorr_fs_mint_proof(
                &adapter,
                &prove_payload,
                SchnorrFsMintProofOverrides {
                    challenge: Some(build_transfer_responses(pairing, Default::default()).x),
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
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `verify`, first-group-only form, mint product: a carried
///   second-group first message that does not satisfy its equation fails the
///   product though the challenge was computed over it → `is_valid` false.
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
            let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let prove_payload = build_prove_mint_payload(pairing, &statement, Default::default());
            let adapter = SchnorrFsDeliveryProof::<P, E, F>::try_new(
                SchnorrFsDeliveryProofConstructorParams {
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    verifier_form: VerifierGroupArithmetic::FirstGroupOnly,
                },
            )
            .expect("the tags are admitted by the domain tag type");
            let proof = build_schnorr_fs_mint_proof(
                &adapter,
                &prove_payload,
                SchnorrFsMintProofOverrides {
                    first_messages: Some(build_mint_second_group_first_messages(
                        pairing,
                        MintSecondGroupFirstMessagesOverrides {
                            d1: Some(
                                build_transfer_second_group_first_messages(
                                    pairing,
                                    Default::default(),
                                )
                                .d1,
                            ),
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
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `verify`, first-group-only form, transfer: the challenge
///   reproduces and the weighted pairing product is one → `is_valid` true.
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
            let statement = build_algebraic_transfer_statement::<P, F>(pairing, Default::default());
            let prove_payload =
                build_prove_transfer_payload(pairing, &statement, Default::default());
            let adapter = SchnorrFsDeliveryProof::<P, E, F>::try_new(
                SchnorrFsDeliveryProofConstructorParams {
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    verifier_form: VerifierGroupArithmetic::FirstGroupOnly,
                },
            )
            .expect("the tags are admitted by the domain tag type");
            let proof = build_schnorr_fs_transfer_proof(
                &adapter,
                &prove_payload,
                SchnorrFsTransferProofOverrides::default(),
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
            assert_eq!(result.ok().map(|success| success.is_valid), Some(true));
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `verify`, first-group-only form, challenge: the recomputed
///   challenge differs from the proof's → `is_valid` false.
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
            let statement = build_algebraic_transfer_statement::<P, F>(pairing, Default::default());
            let prove_payload =
                build_prove_transfer_payload(pairing, &statement, Default::default());
            let adapter = SchnorrFsDeliveryProof::<P, E, F>::try_new(
                SchnorrFsDeliveryProofConstructorParams {
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    verifier_form: VerifierGroupArithmetic::FirstGroupOnly,
                },
            )
            .expect("the tags are admitted by the domain tag type");
            let proof = build_schnorr_fs_transfer_proof(
                &adapter,
                &prove_payload,
                SchnorrFsTransferProofOverrides {
                    challenge: Some(build_mint_responses(pairing, Default::default()).alpha),
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
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `verify`, first-group-only form, transfer product: a carried
///   second-group first message that does not satisfy its equation fails the
///   product though the challenge was computed over it → `is_valid` false.
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
            let statement = build_algebraic_transfer_statement::<P, F>(pairing, Default::default());
            let prove_payload =
                build_prove_transfer_payload(pairing, &statement, Default::default());
            let adapter = SchnorrFsDeliveryProof::<P, E, F>::try_new(
                SchnorrFsDeliveryProofConstructorParams {
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    verifier_form: VerifierGroupArithmetic::FirstGroupOnly,
                },
            )
            .expect("the tags are admitted by the domain tag type");
            let proof = build_schnorr_fs_transfer_proof(
                &adapter,
                &prove_payload,
                SchnorrFsTransferProofOverrides {
                    first_messages: Some(build_transfer_second_group_first_messages(
                        pairing,
                        TransferSecondGroupFirstMessagesOverrides {
                            d2: Some(
                                build_mint_second_group_first_messages(pairing, Default::default())
                                    .d2,
                            ),
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
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `verify`, both-groups form: `challenge` returns `Err(error)` →
///   `Err(VerifyErrorReturn::SchnorrFs(
///   SchnorrFsVerifyErrorReturn::Challenge(error)))`, the refusal unchanged.
/// Arrange: the case params' `hash_to_scalar` at the defaults; inside the
///   consumer's method, a proving adapter over the received adapter forms the
///   proof from `build_schnorr_fs_mint_proof`; the subject's adapter holds
///   the adapter a second `create_hash_to_scalar` returns under
///   `HashToScalarConcrete::Mock(
///   MockIHashToScalarAdapterFailureMode::SamplingRefused)`.
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
            let armed = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: Some(HashToScalarConcrete::Mock(
                        MockIHashToScalarAdapterFailureMode::SamplingRefused,
                    )),
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(armed) = armed else {
                panic!("the armed adapter is admitted")
            };
            let proving = build_schnorr_fs_delivery_proof(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let subject = SchnorrFsDeliveryProof::<P, E, F>::try_new(
                SchnorrFsDeliveryProofConstructorParams {
                    pairing,
                    hash_to_scalar: &*armed.adapter,
                    encoder,
                    random,
                    verifier_form: VerifierGroupArithmetic::BothGroups,
                },
            )
            .expect("the tags are admitted by the domain tag type");
            let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let prove_payload = build_prove_mint_payload(pairing, &statement, Default::default());
            let proof = build_schnorr_fs_mint_proof(
                &proving,
                &prove_payload,
                SchnorrFsMintProofOverrides::default(),
            );

            // Act
            let result = subject.verify(
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
                                expected: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                                actual: 0,
                            }
                        )
                    ))
                ))
            );
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `verify`, first-group-only form: `challenge` returns
///   `Err(error)` before the weights are derived →
///   `Err(VerifyErrorReturn::SchnorrFs(
///   SchnorrFsVerifyErrorReturn::Challenge(error)))`, the refusal unchanged.
/// Arrange: as the block above with both adapters constructed in the
///   first-group-only form and the transfer proof.
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
            let armed = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: Some(HashToScalarConcrete::Mock(
                        MockIHashToScalarAdapterFailureMode::SamplingRefused,
                    )),
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(armed) = armed else {
                panic!("the armed adapter is admitted")
            };
            let proving = SchnorrFsDeliveryProof::<P, E, F>::try_new(
                SchnorrFsDeliveryProofConstructorParams {
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    verifier_form: VerifierGroupArithmetic::FirstGroupOnly,
                },
            )
            .expect("the tags are admitted by the domain tag type");
            let subject = SchnorrFsDeliveryProof::<P, E, F>::try_new(
                SchnorrFsDeliveryProofConstructorParams {
                    pairing,
                    hash_to_scalar: &*armed.adapter,
                    encoder,
                    random,
                    verifier_form: VerifierGroupArithmetic::FirstGroupOnly,
                },
            )
            .expect("the tags are admitted by the domain tag type");
            let statement = build_algebraic_transfer_statement::<P, F>(pairing, Default::default());
            let prove_payload =
                build_prove_transfer_payload(pairing, &statement, Default::default());
            let proof = build_schnorr_fs_transfer_proof(
                &proving,
                &prove_payload,
                SchnorrFsTransferProofOverrides::default(),
            );

            // Act
            let result = subject.verify(
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
                                expected: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                                actual: 0,
                            }
                        )
                    ))
                ))
            );
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `proof_components` clones the challenge and the relation's
///   named responses into the matching variant.
/// Arrange: the both-groups adapter; the proof from
///   `build_schnorr_fs_mint_proof` with `challenge` the `x` of
///   `build_transfer_responses` and `responses` from `build_mint_responses`,
///   every scalar distinct.
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
            let encode_scalar = |scalar: P::Scalar| -> Vec<u8> {
                let Ok(encoded) =
                    pairing.encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar });
                encoded.bytes.expose().as_ref().to_vec()
            };

            // Arrange
            let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let prove_payload = build_prove_mint_payload(pairing, &statement, Default::default());
            let adapter = build_schnorr_fs_delivery_proof::<P, E, F>(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let challenge = build_transfer_responses(pairing, Default::default()).x;
            let proof = build_schnorr_fs_mint_proof(
                &adapter,
                &prove_payload,
                SchnorrFsMintProofOverrides {
                    challenge: Some(challenge.clone()),
                    responses: Some(build_mint_responses(pairing, Default::default())),
                    ..Default::default()
                },
            );
            let expected = build_mint_responses(pairing, Default::default());

            // Act
            let result = adapter.proof_components(
                ProofComponentsParams,
                build_proof_components_payload(&proof, Default::default()),
            );

            // Assert
            let success = result.expect("proof_components failed");
            let DeliveryProofComponents::MintBothGroups {
                challenge: actual,
                responses,
            } = success.components
            else {
                panic!("the components are the mint both-groups variant")
            };
            assert_eq!(encode_scalar(actual), encode_scalar(challenge));
            assert_eq!(
                encode_scalar(responses.alpha),
                encode_scalar(expected.alpha)
            );
            assert_eq!(encode_scalar(responses.r), encode_scalar(expected.r));
            assert_eq!(encode_scalar(responses.rho), encode_scalar(expected.rho));
            assert_eq!(
                encode_scalar(responses.sigma),
                encode_scalar(expected.sigma)
            );
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `proof_components` clones the challenge, the responses, and the
///   carried first messages into the matching variant.
/// Arrange: the first-group-only adapter; the proof as above with
///   `first_messages` from `build_mint_second_group_first_messages`.
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
            let encode_scalar = |scalar: P::Scalar| -> Vec<u8> {
                let Ok(encoded) =
                    pairing.encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar });
                encoded.bytes.expose().as_ref().to_vec()
            };
            let encode_g2 = |point: P::G2| -> Vec<u8> {
                let Ok(encoded) = pairing.encode_g2(EncodeG2Params, EncodeG2Payload { point });
                encoded.bytes.as_ref().to_vec()
            };

            // Arrange
            let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let prove_payload = build_prove_mint_payload(pairing, &statement, Default::default());
            let adapter = SchnorrFsDeliveryProof::<P, E, F>::try_new(
                SchnorrFsDeliveryProofConstructorParams {
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    verifier_form: VerifierGroupArithmetic::FirstGroupOnly,
                },
            )
            .expect("the tags are admitted by the domain tag type");
            let challenge = build_transfer_responses(pairing, Default::default()).x;
            let proof = build_schnorr_fs_mint_proof(
                &adapter,
                &prove_payload,
                SchnorrFsMintProofOverrides {
                    challenge: Some(challenge.clone()),
                    responses: Some(build_mint_responses(pairing, Default::default())),
                    first_messages: Some(build_mint_second_group_first_messages(
                        pairing,
                        Default::default(),
                    )),
                },
            );
            let expected_responses = build_mint_responses(pairing, Default::default());
            let expected_messages =
                build_mint_second_group_first_messages(pairing, Default::default());

            // Act
            let result = adapter.proof_components(
                ProofComponentsParams,
                build_proof_components_payload(&proof, Default::default()),
            );

            // Assert
            let success = result.expect("proof_components failed");
            let DeliveryProofComponents::MintFirstGroupOnly {
                challenge: actual,
                responses,
                first_messages,
            } = success.components
            else {
                panic!("the components are the mint first-group-only variant")
            };
            assert_eq!(encode_scalar(actual), encode_scalar(challenge));
            assert_eq!(
                encode_scalar(responses.alpha),
                encode_scalar(expected_responses.alpha)
            );
            assert_eq!(
                encode_scalar(responses.r),
                encode_scalar(expected_responses.r)
            );
            assert_eq!(
                encode_scalar(responses.rho),
                encode_scalar(expected_responses.rho)
            );
            assert_eq!(
                encode_scalar(responses.sigma),
                encode_scalar(expected_responses.sigma)
            );
            assert_eq!(
                encode_g2(first_messages.hpub),
                encode_g2(expected_messages.hpub)
            );
            assert_eq!(
                encode_g2(first_messages.d1),
                encode_g2(expected_messages.d1)
            );
            assert_eq!(
                encode_g2(first_messages.d2),
                encode_g2(expected_messages.d2)
            );
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `proof_components` clones the challenge and the relation's
///   named responses into the matching variant.
/// Arrange: the both-groups adapter; the proof from
///   `build_schnorr_fs_transfer_proof` with `challenge` the `alpha` of
///   `build_mint_responses` and `responses` from `build_transfer_responses`,
///   every scalar distinct.
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
            let encode_scalar = |scalar: P::Scalar| -> Vec<u8> {
                let Ok(encoded) =
                    pairing.encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar });
                encoded.bytes.expose().as_ref().to_vec()
            };

            // Arrange
            let statement = build_algebraic_transfer_statement::<P, F>(pairing, Default::default());
            let prove_payload =
                build_prove_transfer_payload(pairing, &statement, Default::default());
            let adapter = build_schnorr_fs_delivery_proof::<P, E, F>(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let challenge = build_mint_responses(pairing, Default::default()).alpha;
            let proof = build_schnorr_fs_transfer_proof(
                &adapter,
                &prove_payload,
                SchnorrFsTransferProofOverrides {
                    challenge: Some(challenge.clone()),
                    responses: Some(build_transfer_responses(pairing, Default::default())),
                    ..Default::default()
                },
            );
            let expected = build_transfer_responses(pairing, Default::default());

            // Act
            let result = adapter.proof_components(
                ProofComponentsParams,
                build_proof_components_payload(&proof, Default::default()),
            );

            // Assert
            let success = result.expect("proof_components failed");
            let DeliveryProofComponents::TransferBothGroups {
                challenge: actual,
                responses,
            } = success.components
            else {
                panic!("the components are the transfer both-groups variant")
            };
            assert_eq!(encode_scalar(actual), encode_scalar(challenge));
            assert_eq!(encode_scalar(responses.x), encode_scalar(expected.x));
            assert_eq!(encode_scalar(responses.y), encode_scalar(expected.y));
            assert_eq!(encode_scalar(responses.s), encode_scalar(expected.s));
            assert_eq!(encode_scalar(responses.rho), encode_scalar(expected.rho));
            assert_eq!(
                encode_scalar(responses.sigma),
                encode_scalar(expected.sigma)
            );
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `proof_components` clones the challenge, the responses, and the
///   carried first messages into the matching variant.
/// Arrange: the first-group-only adapter; the proof as above with
///   `first_messages` from `build_transfer_second_group_first_messages`.
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
            let encode_scalar = |scalar: P::Scalar| -> Vec<u8> {
                let Ok(encoded) =
                    pairing.encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar });
                encoded.bytes.expose().as_ref().to_vec()
            };
            let encode_g2 = |point: P::G2| -> Vec<u8> {
                let Ok(encoded) = pairing.encode_g2(EncodeG2Params, EncodeG2Payload { point });
                encoded.bytes.as_ref().to_vec()
            };

            // Arrange
            let statement = build_algebraic_transfer_statement::<P, F>(pairing, Default::default());
            let prove_payload =
                build_prove_transfer_payload(pairing, &statement, Default::default());
            let adapter = SchnorrFsDeliveryProof::<P, E, F>::try_new(
                SchnorrFsDeliveryProofConstructorParams {
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    verifier_form: VerifierGroupArithmetic::FirstGroupOnly,
                },
            )
            .expect("the tags are admitted by the domain tag type");
            let challenge = build_mint_responses(pairing, Default::default()).alpha;
            let proof = build_schnorr_fs_transfer_proof(
                &adapter,
                &prove_payload,
                SchnorrFsTransferProofOverrides {
                    challenge: Some(challenge.clone()),
                    responses: Some(build_transfer_responses(pairing, Default::default())),
                    first_messages: Some(build_transfer_second_group_first_messages(
                        pairing,
                        Default::default(),
                    )),
                },
            );
            let expected_responses = build_transfer_responses(pairing, Default::default());
            let expected_messages =
                build_transfer_second_group_first_messages(pairing, Default::default());

            // Act
            let result = adapter.proof_components(
                ProofComponentsParams,
                build_proof_components_payload(&proof, Default::default()),
            );

            // Assert
            let success = result.expect("proof_components failed");
            let DeliveryProofComponents::TransferFirstGroupOnly {
                challenge: actual,
                responses,
                first_messages,
            } = success.components
            else {
                panic!("the components are the transfer first-group-only variant")
            };
            assert_eq!(encode_scalar(actual), encode_scalar(challenge));
            assert_eq!(
                encode_scalar(responses.x),
                encode_scalar(expected_responses.x)
            );
            assert_eq!(
                encode_scalar(responses.y),
                encode_scalar(expected_responses.y)
            );
            assert_eq!(
                encode_scalar(responses.s),
                encode_scalar(expected_responses.s)
            );
            assert_eq!(
                encode_scalar(responses.rho),
                encode_scalar(expected_responses.rho)
            );
            assert_eq!(
                encode_scalar(responses.sigma),
                encode_scalar(expected_responses.sigma)
            );
            assert_eq!(
                encode_g2(first_messages.pk2),
                encode_g2(expected_messages.pk2)
            );
            assert_eq!(
                encode_g2(first_messages.d1),
                encode_g2(expected_messages.d1)
            );
            assert_eq!(
                encode_g2(first_messages.d2),
                encode_g2(expected_messages.d2)
            );
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `proof_from_components` moves the fields of components of the
///   adapter's form into the proof variant of the same relation and form.
/// Arrange: the both-groups adapter; the components
///   `DeliveryProofComponents::MintBothGroups` over a challenge and
///   `build_mint_responses`, every scalar distinct.
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
            let encode_scalar = |scalar: P::Scalar| -> Vec<u8> {
                let Ok(encoded) =
                    pairing.encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar });
                encoded.bytes.expose().as_ref().to_vec()
            };

            // Arrange
            let adapter = build_schnorr_fs_delivery_proof::<P, E, F>(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let challenge = build_transfer_responses(pairing, Default::default()).x;
            let expected = build_mint_responses(pairing, Default::default());
            let components = DeliveryProofComponents::MintBothGroups {
                challenge: challenge.clone(),
                responses: expected,
            };
            let expected = build_mint_responses(pairing, Default::default());

            // Act
            let result = adapter.proof_from_components(
                ProofFromComponentsParams,
                build_proof_from_components_payload(components, Default::default()),
            );

            // Assert
            let Ok(success) = result else {
                panic!("proof_from_components failed")
            };
            let SchnorrFsProof::MintBothGroups {
                challenge: actual,
                responses,
            } = success.proof
            else {
                panic!("the proof is the mint both-groups variant")
            };
            assert_eq!(encode_scalar(actual), encode_scalar(challenge));
            assert_eq!(
                encode_scalar(responses.alpha),
                encode_scalar(expected.alpha)
            );
            assert_eq!(encode_scalar(responses.r), encode_scalar(expected.r));
            assert_eq!(encode_scalar(responses.rho), encode_scalar(expected.rho));
            assert_eq!(
                encode_scalar(responses.sigma),
                encode_scalar(expected.sigma)
            );
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `proof_from_components` moves the fields of components of the
///   adapter's form into the proof variant of the same relation and form.
/// Arrange: the first-group-only adapter; the components
///   `DeliveryProofComponents::MintFirstGroupOnly` over a challenge,
///   `build_mint_responses`, and `build_mint_second_group_first_messages`.
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
            let encode_scalar = |scalar: P::Scalar| -> Vec<u8> {
                let Ok(encoded) =
                    pairing.encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar });
                encoded.bytes.expose().as_ref().to_vec()
            };
            let encode_g2 = |point: P::G2| -> Vec<u8> {
                let Ok(encoded) = pairing.encode_g2(EncodeG2Params, EncodeG2Payload { point });
                encoded.bytes.as_ref().to_vec()
            };

            // Arrange
            let adapter = SchnorrFsDeliveryProof::<P, E, F>::try_new(
                SchnorrFsDeliveryProofConstructorParams {
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    verifier_form: VerifierGroupArithmetic::FirstGroupOnly,
                },
            )
            .expect("the tags are admitted by the domain tag type");
            let challenge = build_transfer_responses(pairing, Default::default()).x;
            let components = DeliveryProofComponents::MintFirstGroupOnly {
                challenge: challenge.clone(),
                responses: build_mint_responses(pairing, Default::default()),
                first_messages: build_mint_second_group_first_messages(pairing, Default::default()),
            };
            let expected = build_mint_responses(pairing, Default::default());
            let expected_messages =
                build_mint_second_group_first_messages(pairing, Default::default());

            // Act
            let result = adapter.proof_from_components(
                ProofFromComponentsParams,
                build_proof_from_components_payload(components, Default::default()),
            );

            // Assert
            let Ok(success) = result else {
                panic!("proof_from_components failed")
            };
            let SchnorrFsProof::MintFirstGroupOnly {
                challenge: actual,
                responses,
                first_messages,
            } = success.proof
            else {
                panic!("the proof is the mint first-group-only variant")
            };
            assert_eq!(encode_scalar(actual), encode_scalar(challenge));
            assert_eq!(
                encode_scalar(responses.alpha),
                encode_scalar(expected.alpha)
            );
            assert_eq!(encode_scalar(responses.r), encode_scalar(expected.r));
            assert_eq!(encode_scalar(responses.rho), encode_scalar(expected.rho));
            assert_eq!(
                encode_scalar(responses.sigma),
                encode_scalar(expected.sigma)
            );
            assert_eq!(
                encode_g2(first_messages.hpub),
                encode_g2(expected_messages.hpub)
            );
            assert_eq!(
                encode_g2(first_messages.d1),
                encode_g2(expected_messages.d1)
            );
            assert_eq!(
                encode_g2(first_messages.d2),
                encode_g2(expected_messages.d2)
            );
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `proof_from_components` moves the fields of components of the
///   adapter's form into the proof variant of the same relation and form.
/// Arrange: the both-groups adapter; the components
///   `DeliveryProofComponents::TransferBothGroups` over a challenge and
///   `build_transfer_responses`.
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
            let encode_scalar = |scalar: P::Scalar| -> Vec<u8> {
                let Ok(encoded) =
                    pairing.encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar });
                encoded.bytes.expose().as_ref().to_vec()
            };

            // Arrange
            let adapter = build_schnorr_fs_delivery_proof::<P, E, F>(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let challenge = build_mint_responses(pairing, Default::default()).alpha;
            let components = DeliveryProofComponents::TransferBothGroups {
                challenge: challenge.clone(),
                responses: build_transfer_responses(pairing, Default::default()),
            };
            let expected = build_transfer_responses(pairing, Default::default());

            // Act
            let result = adapter.proof_from_components(
                ProofFromComponentsParams,
                build_proof_from_components_payload(components, Default::default()),
            );

            // Assert
            let Ok(success) = result else {
                panic!("proof_from_components failed")
            };
            let SchnorrFsProof::TransferBothGroups {
                challenge: actual,
                responses,
            } = success.proof
            else {
                panic!("the proof is the transfer both-groups variant")
            };
            assert_eq!(encode_scalar(actual), encode_scalar(challenge));
            assert_eq!(encode_scalar(responses.x), encode_scalar(expected.x));
            assert_eq!(encode_scalar(responses.y), encode_scalar(expected.y));
            assert_eq!(encode_scalar(responses.s), encode_scalar(expected.s));
            assert_eq!(encode_scalar(responses.rho), encode_scalar(expected.rho));
            assert_eq!(
                encode_scalar(responses.sigma),
                encode_scalar(expected.sigma)
            );
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `proof_from_components` moves the fields of components of the
///   adapter's form into the proof variant of the same relation and form.
/// Arrange: the first-group-only adapter; the components
///   `DeliveryProofComponents::TransferFirstGroupOnly` over a challenge,
///   `build_transfer_responses`, and
///   `build_transfer_second_group_first_messages`.
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
            let encode_scalar = |scalar: P::Scalar| -> Vec<u8> {
                let Ok(encoded) =
                    pairing.encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar });
                encoded.bytes.expose().as_ref().to_vec()
            };
            let encode_g2 = |point: P::G2| -> Vec<u8> {
                let Ok(encoded) = pairing.encode_g2(EncodeG2Params, EncodeG2Payload { point });
                encoded.bytes.as_ref().to_vec()
            };

            // Arrange
            let adapter = SchnorrFsDeliveryProof::<P, E, F>::try_new(
                SchnorrFsDeliveryProofConstructorParams {
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    verifier_form: VerifierGroupArithmetic::FirstGroupOnly,
                },
            )
            .expect("the tags are admitted by the domain tag type");
            let challenge = build_mint_responses(pairing, Default::default()).alpha;
            let components = DeliveryProofComponents::TransferFirstGroupOnly {
                challenge: challenge.clone(),
                responses: build_transfer_responses(pairing, Default::default()),
                first_messages: build_transfer_second_group_first_messages(
                    pairing,
                    Default::default(),
                ),
            };
            let expected = build_transfer_responses(pairing, Default::default());
            let expected_messages =
                build_transfer_second_group_first_messages(pairing, Default::default());

            // Act
            let result = adapter.proof_from_components(
                ProofFromComponentsParams,
                build_proof_from_components_payload(components, Default::default()),
            );

            // Assert
            let Ok(success) = result else {
                panic!("proof_from_components failed")
            };
            let SchnorrFsProof::TransferFirstGroupOnly {
                challenge: actual,
                responses,
                first_messages,
            } = success.proof
            else {
                panic!("the proof is the transfer first-group-only variant")
            };
            assert_eq!(encode_scalar(actual), encode_scalar(challenge));
            assert_eq!(encode_scalar(responses.x), encode_scalar(expected.x));
            assert_eq!(encode_scalar(responses.y), encode_scalar(expected.y));
            assert_eq!(encode_scalar(responses.s), encode_scalar(expected.s));
            assert_eq!(encode_scalar(responses.rho), encode_scalar(expected.rho));
            assert_eq!(
                encode_scalar(responses.sigma),
                encode_scalar(expected.sigma)
            );
            assert_eq!(
                encode_g2(first_messages.pk2),
                encode_g2(expected_messages.pk2)
            );
            assert_eq!(
                encode_g2(first_messages.d1),
                encode_g2(expected_messages.d1)
            );
            assert_eq!(
                encode_g2(first_messages.d2),
                encode_g2(expected_messages.d2)
            );
        }
    }

    let result = run_schnorr_fs_case(
        &build_schnorr_fs_case_deps(Consumer, Default::default()),
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `proof_from_components`, form mismatch: the components' form
///   differs from `self.verifier_form` →
///   `Err(ProofFromComponentsErrorReturn::VerifierFormMismatch)`.
/// Arrange: the both-groups adapter; the components
///   `DeliveryProofComponents::MintFirstGroupOnly`.
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
            let adapter = build_schnorr_fs_delivery_proof::<P, E, F>(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                Default::default(),
            );
            let components = DeliveryProofComponents::MintFirstGroupOnly {
                challenge: build_mint_responses(pairing, Default::default()).alpha,
                responses: build_mint_responses(pairing, Default::default()),
                first_messages: build_mint_second_group_first_messages(pairing, Default::default()),
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
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}

/// Contract: `proof_from_components`, form mismatch: the components' form
///   differs from `self.verifier_form` →
///   `Err(ProofFromComponentsErrorReturn::VerifierFormMismatch)`.
/// Arrange: the first-group-only adapter; the components
///   `DeliveryProofComponents::TransferBothGroups`.
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
            let adapter = SchnorrFsDeliveryProof::<P, E, F>::try_new(
                SchnorrFsDeliveryProofConstructorParams {
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    verifier_form: VerifierGroupArithmetic::FirstGroupOnly,
                },
            )
            .expect("the tags are admitted by the domain tag type");
            let components = DeliveryProofComponents::TransferBothGroups {
                challenge: build_mint_responses(pairing, Default::default()).alpha,
                responses: build_transfer_responses(pairing, Default::default()),
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
        build_schnorr_fs_case_params(SchnorrFsCaseParamsOverrides::default()),
        build_schnorr_fs_case_payload(Default::default()),
    );

    assert!(matches!(result, Ok(SchnorrFsCaseSuccessReturn { .. })));
}
