#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use chain::IChainForms;
use encoding::{IDecoderAdapter, IEncoderAdapter};
use envelope::{
    CreateKeyAgreementReturn, CreateKeyAgreementSuccessReturn, IKeyAgreementAdapter,
    build_create_key_agreement_deps, build_create_key_agreement_params,
    build_create_key_agreement_payload, create_key_agreement,
};
use hash_to_scalar::IHashToScalarAdapter;
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::ICredentialKemAdapter;
use pairing::{IPairingArithmetic, IPairingReference};
use proof::{
    CreateDeliveryProofErrorReturn, DELIVERY_PURPOSE_MINT, DELIVERY_PURPOSE_TRANSFER,
    DeliveryRelation, IDeliveryProofAdapter, MintPurpose, TransferPurpose, build_delivery_context,
};
use random::IRandomSourceAdapter;

use super::interface::{KeyAgreementConsumerErrorReturn, KeyAgreementConsumerReturn};
use super::mock::{KeyAgreementConsumerConstructorParamsOverrides, build_key_agreement_consumer};
use crate::delivery::provides::AdmittedKemIdentity;
use crate::generate::provides::{
    EmitErrorReturn, GenerateConcrete, ICreateGenerateCaseConsumer,
    MockIGenerateAdapterEmitErrorReturn, MockIGenerateAdapterFailureMode,
    build_create_generate_case_deps, build_create_generate_case_params,
    build_create_generate_case_payload, run_create_generate_case,
};
use crate::measure::provides::MeasureErrorReturn;
use crate::run::provides::DeliveryProofConsumerErrorReturn;
use crate::verifier::provides::{
    MockIVerifierAdapterFailureMode, MockIVerifierAdapterVerifyOnChainErrorReturn,
    VerifierConcrete, VerifyOnChainErrorReturn,
};

/// Contract: Delivery proof refused at construction → the refusal
///   `create_delivery_proof` returns is the output whole, and the delivery
///   proof consumer is not reached.
/// Arrange: the consumer constructed with `statement_version` `2` and the
///   default mock delivery proof, generate, and verifier concretes.
/// Act:     `create_key_agreement`, once, inside the consumer.
/// Assert:  the output matches
///   `Err(KeyAgreementConsumerErrorReturn::CreateDeliveryProof(
///   CreateDeliveryProofErrorReturn::UnsupportedStatementVersion))`, the
///   pattern naming every field.
#[test]
fn consume_key_agreement_returns_the_delivery_proof_factorys_refusal_for_an_unsupported_statement_version()
 {
    // Arrange
    struct CaseConsumer;

    impl ICreateGenerateCaseConsumer for CaseConsumer {
        type Output = CreateKeyAgreementReturn<KeyAgreementConsumerReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            kem: &K,
            _key_agreement: &A,
            _delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            let consumer = build_key_agreement_consumer(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                kem,
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default()),
                build_delivery_context::<F, TransferPurpose>(
                    DELIVERY_PURPOSE_TRANSFER,
                    Default::default(),
                ),
                identities.clone(),
                identities[0],
                KeyAgreementConsumerConstructorParamsOverrides {
                    statement_version: Some(2),
                    ..Default::default()
                },
            );
            create_key_agreement(
                &build_create_key_agreement_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    consumer,
                    Default::default(),
                ),
                build_create_key_agreement_params(Default::default()),
                build_create_key_agreement_payload(Default::default()),
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
        Ok(CreateKeyAgreementSuccessReturn {
            output: Err(KeyAgreementConsumerErrorReturn::CreateDeliveryProof(
                CreateDeliveryProofErrorReturn::UnsupportedStatementVersion
            )),
        })
    ));
    // consume_key_agreement_returns_the_delivery_proof_factorys_refusal_for_an_unsupported_statement_version
}

/// Contract: Delivery proof consumer refused → the refusal the delivery proof
///   consumer returns is the output whole, inside
///   `KeyAgreementConsumerErrorReturn::DeliveryProof`.
/// Arrange: the consumer constructed with `generate` `GenerateConcrete::Mock(
///   MockIGenerateAdapterFailureMode::WriteRefused)` and `verifier`
///   `VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::RpcRefused)`.
/// Act:     `create_key_agreement`, once, inside the consumer.
/// Assert:  the output matches
///   `Err(KeyAgreementConsumerErrorReturn::DeliveryProof(
///   DeliveryProofConsumerErrorReturn::Emit(EmitErrorReturn::Mock(
///   MockIGenerateAdapterEmitErrorReturn::WriteRefused))))`, the pattern
///   naming every field because `KeyAgreementConsumerErrorReturn` omits
///   `PartialEq`.
#[test]
fn consume_key_agreement_returns_the_delivery_proof_consumers_refusal_whole() {
    // Arrange
    struct CaseConsumer;

    impl ICreateGenerateCaseConsumer for CaseConsumer {
        type Output = CreateKeyAgreementReturn<KeyAgreementConsumerReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            kem: &K,
            _key_agreement: &A,
            _delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            let consumer = build_key_agreement_consumer(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                kem,
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default()),
                build_delivery_context::<F, TransferPurpose>(
                    DELIVERY_PURPOSE_TRANSFER,
                    Default::default(),
                ),
                identities.clone(),
                identities[0],
                KeyAgreementConsumerConstructorParamsOverrides {
                    generate: Some(GenerateConcrete::Mock(
                        MockIGenerateAdapterFailureMode::WriteRefused,
                    )),
                    verifier: Some(VerifierConcrete::Mock(
                        MockIVerifierAdapterFailureMode::RpcRefused,
                    )),
                    ..Default::default()
                },
            );
            create_key_agreement(
                &build_create_key_agreement_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    consumer,
                    Default::default(),
                ),
                build_create_key_agreement_params(Default::default()),
                build_create_key_agreement_payload(Default::default()),
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
        Ok(CreateKeyAgreementSuccessReturn {
            output: Err(KeyAgreementConsumerErrorReturn::DeliveryProof(
                DeliveryProofConsumerErrorReturn::Emit(EmitErrorReturn::Mock(
                    MockIGenerateAdapterEmitErrorReturn::WriteRefused
                ))
            )),
        })
    ));
    // consume_key_agreement_returns_the_delivery_proof_consumers_refusal_whole
}

/// Contract: Measurement refused → the refusal `measure` returns reaches the
///   output whole, neither converted nor summarized.
/// Arrange: the consumer constructed with `verifier` `VerifierConcrete::Mock(
///   MockIVerifierAdapterFailureMode::RpcRefused)` and the default mock
///   generate and delivery proof concretes.
/// Act:     `create_key_agreement`, once, inside the consumer.
/// Assert:  the output matches
///   `Err(KeyAgreementConsumerErrorReturn::DeliveryProof(
///   DeliveryProofConsumerErrorReturn::Measure(MeasureErrorReturn::OnChain {
///   relation: DeliveryRelation::Mint, error: VerifyOnChainErrorReturn::Mock(
///   MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused) })))`, the
///   pattern naming every field.
#[test]
fn consume_key_agreement_returns_the_verifiers_refusal_after_the_generate_step_succeeds() {
    // Arrange
    struct CaseConsumer;

    impl ICreateGenerateCaseConsumer for CaseConsumer {
        type Output = CreateKeyAgreementReturn<KeyAgreementConsumerReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            kem: &K,
            _key_agreement: &A,
            _delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            let consumer = build_key_agreement_consumer(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                kem,
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default()),
                build_delivery_context::<F, TransferPurpose>(
                    DELIVERY_PURPOSE_TRANSFER,
                    Default::default(),
                ),
                identities.clone(),
                identities[0],
                KeyAgreementConsumerConstructorParamsOverrides {
                    verifier: Some(VerifierConcrete::Mock(
                        MockIVerifierAdapterFailureMode::RpcRefused,
                    )),
                    ..Default::default()
                },
            );
            create_key_agreement(
                &build_create_key_agreement_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    consumer,
                    Default::default(),
                ),
                build_create_key_agreement_params(Default::default()),
                build_create_key_agreement_payload(Default::default()),
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
        Ok(CreateKeyAgreementSuccessReturn {
            output: Err(KeyAgreementConsumerErrorReturn::DeliveryProof(
                DeliveryProofConsumerErrorReturn::Measure(MeasureErrorReturn::OnChain {
                    relation: DeliveryRelation::Mint,
                    error: VerifyOnChainErrorReturn::Mock(
                        MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused
                    ),
                })
            )),
        })
    ));
    // consume_key_agreement_returns_the_verifiers_refusal_after_the_generate_step_succeeds
}
