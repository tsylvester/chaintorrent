#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use chain::IChainForms;
use encoding::{IDecoderAdapter, IEncoderAdapter};
use envelope::IKeyAgreementAdapter;
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::ICredentialKemAdapter;
use pairing::{IPairingArithmetic, IPairingReference};
use proof::{
    CreateDeliveryProofReturn, CreateDeliveryProofSuccessReturn, DELIVERY_PURPOSE_MINT,
    DELIVERY_PURPOSE_TRANSFER, DeliveryRelation, IDeliveryProofAdapter, MintPurpose,
    TransferPurpose, build_create_delivery_proof_deps, build_create_delivery_proof_params,
    build_create_delivery_proof_payload, build_delivery_context, create_delivery_proof,
};

use super::interface::{DeliveryProofConsumerErrorReturn, DeliveryProofConsumerReturn};
use super::mock::{DeliveryProofConsumerConstructorParamsOverrides, build_delivery_proof_consumer};
use crate::generate::provides::{
    ConsumeCreateGenerateCaseParams, ConsumeCreateGenerateCasePayload, EmitErrorReturn,
    GenerateConcrete, ICreateGenerateCaseConsumer, MockIGenerateAdapterEmitErrorReturn,
    MockIGenerateAdapterFailureMode, build_create_generate_case_deps,
    build_create_generate_case_params, build_create_generate_case_payload,
    run_create_generate_case,
};
use crate::measure::provides::MeasureErrorReturn;
use crate::verifier::provides::{
    CreateVerifierErrorReturn, MockIVerifierAdapterFailureMode,
    MockIVerifierAdapterVerifyOnChainErrorReturn, VerifierConcrete, VerifyOnChainErrorReturn,
};

/// Contract: Emission refused → the refusal `emit` returns is the output
///   whole, and the verifier is not reached.
/// Arrange: the consumer constructed with `generate` `GenerateConcrete::Mock(
///   MockIGenerateAdapterFailureMode::WriteRefused)` and `verifier`
///   `VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::RpcRefused)`.
/// Act:     `create_delivery_proof`, once, inside the consumer.
/// Assert:  the output matches
///   `Err(DeliveryProofConsumerErrorReturn::Emit(EmitErrorReturn::Mock(
///   MockIGenerateAdapterEmitErrorReturn::WriteRefused)))`, the pattern naming
///   every field because `DeliveryProofConsumerErrorReturn` omits `PartialEq`.
#[test]
fn consume_delivery_proof_returns_the_generate_steps_refusal_before_the_verifier_is_reached() {
    // Arrange
    struct CaseConsumer;

    impl ICreateGenerateCaseConsumer for CaseConsumer {
        type Output = CreateDeliveryProofReturn<DeliveryProofConsumerReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            _params: ConsumeCreateGenerateCaseParams,
            payload: ConsumeCreateGenerateCasePayload<'_, P, E, K, A, D>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            let consumer = build_delivery_proof_consumer(
                payload.pairing,
                payload.hash_to_scalar,
                payload.encoder,
                payload.random,
                payload.kem,
                payload.key_agreement,
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default()),
                build_delivery_context::<F, TransferPurpose>(
                    DELIVERY_PURPOSE_TRANSFER,
                    Default::default(),
                ),
                payload.identities.clone(),
                payload.identities[0],
                DeliveryProofConsumerConstructorParamsOverrides {
                    generate: Some(GenerateConcrete::Mock(
                        MockIGenerateAdapterFailureMode::WriteRefused,
                    )),
                    verifier: Some(VerifierConcrete::Mock(
                        MockIVerifierAdapterFailureMode::RpcRefused,
                    )),
                    ..Default::default()
                },
            );
            create_delivery_proof(
                &build_create_delivery_proof_deps(
                    payload.pairing,
                    payload.hash_to_scalar,
                    payload.encoder,
                    payload.random,
                    consumer,
                    Default::default(),
                ),
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            )
        }
    }

    let canonicals: [&[u8]; 2] = [
        b"example-package@1.0.0".as_slice(),
        b"example-package@2.0.0".as_slice(),
    ];
    let first_root = build_root(Default::default());
    let second_root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x11; 32]),
    });
    let roots = [&first_root, &second_root];
    let deps = build_create_generate_case_deps(CaseConsumer, Default::default());

    // Act
    let result = run_create_generate_case(
        &deps,
        build_create_generate_case_params(Default::default()),
        build_create_generate_case_payload(&canonicals, &roots, Default::default()),
    );

    // Assert
    let output = result.expect("the case composes").output;
    assert!(matches!(
        output,
        Ok(CreateDeliveryProofSuccessReturn {
            output: Err(DeliveryProofConsumerErrorReturn::Emit(
                EmitErrorReturn::Mock(MockIGenerateAdapterEmitErrorReturn::WriteRefused)
            )),
        })
    ));
    // consume_delivery_proof_returns_the_generate_steps_refusal_before_the_verifier_is_reached
}

/// Contract: Measurement refused → the refusal `measure` returns is the output
///   whole, neither converted nor summarized.
/// Arrange: the consumer constructed with `verifier` `VerifierConcrete::Mock(
///   MockIVerifierAdapterFailureMode::RpcRefused)` and the default mock
///   generate concrete.
/// Act:     `create_delivery_proof`, once, inside the consumer.
/// Assert:  the output matches
///   `Err(DeliveryProofConsumerErrorReturn::Measure(MeasureErrorReturn::OnChain {
///   relation: DeliveryRelation::Mint, error: VerifyOnChainErrorReturn::Mock(
///   MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused) }))`, the
///   pattern naming every field.
#[test]
fn consume_delivery_proof_returns_the_verifiers_refusal_after_the_generate_step_succeeds() {
    // Arrange
    struct CaseConsumer;

    impl ICreateGenerateCaseConsumer for CaseConsumer {
        type Output = CreateDeliveryProofReturn<DeliveryProofConsumerReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            _params: ConsumeCreateGenerateCaseParams,
            payload: ConsumeCreateGenerateCasePayload<'_, P, E, K, A, D>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            let consumer = build_delivery_proof_consumer(
                payload.pairing,
                payload.hash_to_scalar,
                payload.encoder,
                payload.random,
                payload.kem,
                payload.key_agreement,
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default()),
                build_delivery_context::<F, TransferPurpose>(
                    DELIVERY_PURPOSE_TRANSFER,
                    Default::default(),
                ),
                payload.identities.clone(),
                payload.identities[0],
                DeliveryProofConsumerConstructorParamsOverrides {
                    verifier: Some(VerifierConcrete::Mock(
                        MockIVerifierAdapterFailureMode::RpcRefused,
                    )),
                    ..Default::default()
                },
            );
            create_delivery_proof(
                &build_create_delivery_proof_deps(
                    payload.pairing,
                    payload.hash_to_scalar,
                    payload.encoder,
                    payload.random,
                    consumer,
                    Default::default(),
                ),
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            )
        }
    }

    let canonicals: [&[u8]; 2] = [
        b"example-package@1.0.0".as_slice(),
        b"example-package@2.0.0".as_slice(),
    ];
    let first_root = build_root(Default::default());
    let second_root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x11; 32]),
    });
    let roots = [&first_root, &second_root];
    let deps = build_create_generate_case_deps(CaseConsumer, Default::default());

    // Act
    let result = run_create_generate_case(
        &deps,
        build_create_generate_case_params(Default::default()),
        build_create_generate_case_payload(&canonicals, &roots, Default::default()),
    );

    // Assert
    let output = result.expect("the case composes").output;
    assert!(matches!(
        output,
        Ok(CreateDeliveryProofSuccessReturn {
            output: Err(DeliveryProofConsumerErrorReturn::Measure(
                MeasureErrorReturn::OnChain {
                    relation: DeliveryRelation::Mint,
                    error: VerifyOnChainErrorReturn::Mock(
                        MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused
                    ),
                }
            )),
        })
    ));
    // consume_delivery_proof_returns_the_verifiers_refusal_after_the_generate_step_succeeds
}

/// Contract: Verifier refused at construction → the refusal `create_verifier`
///   returns is the output whole.
/// Arrange: the consumer constructed with `statement_version` `2` and the
///   default mock generate and verifier concretes.
/// Act:     `create_delivery_proof`, once, inside the consumer.
/// Assert:  the output matches
///   `Err(DeliveryProofConsumerErrorReturn::CreateVerifier(
///   CreateVerifierErrorReturn::UnsupportedStatementVersion { version: 2 }))`,
///   the pattern naming every field.
#[test]
fn consume_delivery_proof_returns_the_verifier_factorys_refusal_for_an_unsupported_statement_version()
 {
    // Arrange
    struct CaseConsumer;

    impl ICreateGenerateCaseConsumer for CaseConsumer {
        type Output = CreateDeliveryProofReturn<DeliveryProofConsumerReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            _params: ConsumeCreateGenerateCaseParams,
            payload: ConsumeCreateGenerateCasePayload<'_, P, E, K, A, D>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            let consumer = build_delivery_proof_consumer(
                payload.pairing,
                payload.hash_to_scalar,
                payload.encoder,
                payload.random,
                payload.kem,
                payload.key_agreement,
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default()),
                build_delivery_context::<F, TransferPurpose>(
                    DELIVERY_PURPOSE_TRANSFER,
                    Default::default(),
                ),
                payload.identities.clone(),
                payload.identities[0],
                DeliveryProofConsumerConstructorParamsOverrides {
                    statement_version: Some(2),
                    ..Default::default()
                },
            );
            create_delivery_proof(
                &build_create_delivery_proof_deps(
                    payload.pairing,
                    payload.hash_to_scalar,
                    payload.encoder,
                    payload.random,
                    consumer,
                    Default::default(),
                ),
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            )
        }
    }

    let canonicals: [&[u8]; 2] = [
        b"example-package@1.0.0".as_slice(),
        b"example-package@2.0.0".as_slice(),
    ];
    let first_root = build_root(Default::default());
    let second_root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x11; 32]),
    });
    let roots = [&first_root, &second_root];
    let deps = build_create_generate_case_deps(CaseConsumer, Default::default());

    // Act
    let result = run_create_generate_case(
        &deps,
        build_create_generate_case_params(Default::default()),
        build_create_generate_case_payload(&canonicals, &roots, Default::default()),
    );

    // Assert
    let output = result.expect("the case composes").output;
    assert!(matches!(
        output,
        Ok(CreateDeliveryProofSuccessReturn {
            output: Err(DeliveryProofConsumerErrorReturn::CreateVerifier(
                CreateVerifierErrorReturn::UnsupportedStatementVersion { version: 2 }
            )),
        })
    ));
    // consume_delivery_proof_returns_the_verifier_factorys_refusal_for_an_unsupported_statement_version
}
