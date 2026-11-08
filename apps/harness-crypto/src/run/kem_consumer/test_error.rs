#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use chain::IChainForms;
use encoding::{IDecoderAdapter, IEncoderAdapter};
use envelope::{
    CreateKeyAgreementErrorReturn, DECLARED_KEY_AGREEMENT_SELECTIONS, IKeyAgreementAdapter,
    KeyAgreementConcrete, build_create_key_agreement_params,
};
use hash_to_scalar::IHashToScalarAdapter;
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::{
    CreateKemReturn, CreateKemSuccessReturn, ICredentialKemAdapter, build_create_kem_deps,
    build_create_kem_params, build_create_kem_payload, create_kem,
};
use pairing::{IPairingArithmetic, IPairingReference};
use proof::{
    DELIVERY_PURPOSE_MINT, DELIVERY_PURPOSE_TRANSFER, DeliveryRelation, IDeliveryProofAdapter,
    MintPurpose, TransferPurpose, build_delivery_context,
};
use random::IRandomSourceAdapter;

use super::interface::{KemConsumerErrorReturn, KemConsumerReturn};
use super::mock::{KemConsumerConstructorParamsOverrides, build_kem_consumer};
use crate::config::provides::{AdmittedKeyAgreement, build_admitted_identities};
use crate::delivery::provides::AdmittedKemIdentity;
use crate::generate::provides::{
    EmitErrorReturn, GenerateConcrete, ICreateGenerateCaseConsumer,
    MockIGenerateAdapterEmitErrorReturn, MockIGenerateAdapterFailureMode,
    build_create_generate_case_deps, build_create_generate_case_params,
    build_create_generate_case_payload, run_create_generate_case,
};
use crate::measure::provides::MeasureErrorReturn;
use crate::run::provides::{DeliveryProofConsumerErrorReturn, KeyAgreementConsumerErrorReturn};
use crate::verifier::provides::{
    MockIVerifierAdapterFailureMode, MockIVerifierAdapterVerifyOnChainErrorReturn,
    VerifierConcrete, VerifyOnChainErrorReturn,
};

/// Contract: Key agreement refused at construction → the refusal
///   `create_key_agreement` returns is the output whole, and the key agreement
///   consumer is not reached.
/// Arrange: the consumer constructed with `key_agreement` an
///   `AdmittedKeyAgreement` holding the `PairingElGamal` member of
///   `DECLARED_KEY_AGREEMENT_SELECTIONS`' `concrete` and `algebra` and the
///   `identifier` of `build_create_key_agreement_params(Default::default())`,
///   and the default mock concretes elsewhere.
/// Act:     `create_kem`, once, inside the consumer.
/// Assert:  the output matches
///   `Err(KemConsumerErrorReturn::CreateKeyAgreement(
///   CreateKeyAgreementErrorReturn::UnsupportedKeyAgreementIdentifier))`, the
///   pattern naming every field.
#[test]
fn consume_kem_returns_the_key_agreement_factorys_refusal_for_an_undeclared_identifier() {
    // Arrange
    struct CaseConsumer;

    impl ICreateGenerateCaseConsumer for CaseConsumer {
        type Output = CreateKemReturn<KemConsumerReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            _kem: &K,
            _key_agreement: &A,
            _delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            let declared = DECLARED_KEY_AGREEMENT_SELECTIONS
                .iter()
                .find(|selection| selection.concrete == KeyAgreementConcrete::PairingElGamal)
                .expect("the declared set names the PairingElGamal member");
            let admitted_identities = build_admitted_identities(Default::default());
            let consumer = build_kem_consumer(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default()),
                build_delivery_context::<F, TransferPurpose>(
                    DELIVERY_PURPOSE_TRANSFER,
                    Default::default(),
                ),
                &admitted_identities,
                KemConsumerConstructorParamsOverrides {
                    key_agreement: Some(AdmittedKeyAgreement {
                        concrete: declared.concrete,
                        identifier: build_create_key_agreement_params(Default::default())
                            .identifier,
                        algebra: declared.algebra,
                    }),
                    ..Default::default()
                },
            );
            create_kem(
                &build_create_kem_deps(pairing, hash_to_scalar, consumer, Default::default()),
                build_create_kem_params(Default::default()),
                build_create_kem_payload(Default::default()),
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
        Ok(CreateKemSuccessReturn {
            output: Err(KemConsumerErrorReturn::CreateKeyAgreement(
                CreateKeyAgreementErrorReturn::UnsupportedKeyAgreementIdentifier
            )),
        })
    ));
    // consume_kem_returns_the_key_agreement_factorys_refusal_for_an_undeclared_identifier
}

/// Contract: Key agreement consumer refused → the refusal the key agreement
///   consumer returns is the output whole, inside
///   `KemConsumerErrorReturn::KeyAgreement`.
/// Arrange: the consumer constructed with `generate` `GenerateConcrete::Mock(
///   MockIGenerateAdapterFailureMode::WriteRefused)` and `verifier`
///   `VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::RpcRefused)`.
/// Act:     `create_kem`, once, inside the consumer.
/// Assert:  the output matches
///   `Err(KemConsumerErrorReturn::KeyAgreement(
///   KeyAgreementConsumerErrorReturn::DeliveryProof(
///   DeliveryProofConsumerErrorReturn::Emit(EmitErrorReturn::Mock(
///   MockIGenerateAdapterEmitErrorReturn::WriteRefused)))))`, the pattern
///   naming every field because `KemConsumerErrorReturn` omits `PartialEq`.
#[test]
fn consume_kem_returns_the_key_agreement_consumers_refusal_whole() {
    // Arrange
    struct CaseConsumer;

    impl ICreateGenerateCaseConsumer for CaseConsumer {
        type Output = CreateKemReturn<KemConsumerReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            _kem: &K,
            _key_agreement: &A,
            _delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            let admitted_identities = build_admitted_identities(Default::default());
            let consumer = build_kem_consumer(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default()),
                build_delivery_context::<F, TransferPurpose>(
                    DELIVERY_PURPOSE_TRANSFER,
                    Default::default(),
                ),
                &admitted_identities,
                KemConsumerConstructorParamsOverrides {
                    generate: Some(GenerateConcrete::Mock(
                        MockIGenerateAdapterFailureMode::WriteRefused,
                    )),
                    verifier: Some(VerifierConcrete::Mock(
                        MockIVerifierAdapterFailureMode::RpcRefused,
                    )),
                    ..Default::default()
                },
            );
            create_kem(
                &build_create_kem_deps(pairing, hash_to_scalar, consumer, Default::default()),
                build_create_kem_params(Default::default()),
                build_create_kem_payload(Default::default()),
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
        Ok(CreateKemSuccessReturn {
            output: Err(KemConsumerErrorReturn::KeyAgreement(
                KeyAgreementConsumerErrorReturn::DeliveryProof(
                    DeliveryProofConsumerErrorReturn::Emit(EmitErrorReturn::Mock(
                        MockIGenerateAdapterEmitErrorReturn::WriteRefused
                    ))
                )
            )),
        })
    ));
    // consume_kem_returns_the_key_agreement_consumers_refusal_whole
}

/// Contract: Measurement refused → the refusal `measure` returns reaches the
///   output whole, neither converted nor summarized.
/// Arrange: the consumer constructed with `verifier` `VerifierConcrete::Mock(
///   MockIVerifierAdapterFailureMode::RpcRefused)` and the default mock
///   concretes elsewhere.
/// Act:     `create_kem`, once, inside the consumer.
/// Assert:  the output matches
///   `Err(KemConsumerErrorReturn::KeyAgreement(
///   KeyAgreementConsumerErrorReturn::DeliveryProof(
///   DeliveryProofConsumerErrorReturn::Measure(MeasureErrorReturn::OnChain {
///   relation: DeliveryRelation::Mint, error: VerifyOnChainErrorReturn::Mock(
///   MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused) }))))`, the
///   pattern naming every field.
#[test]
fn consume_kem_returns_the_verifiers_refusal_after_the_generate_step_succeeds() {
    // Arrange
    struct CaseConsumer;

    impl ICreateGenerateCaseConsumer for CaseConsumer {
        type Output = CreateKemReturn<KemConsumerReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            _kem: &K,
            _key_agreement: &A,
            _delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            let admitted_identities = build_admitted_identities(Default::default());
            let consumer = build_kem_consumer(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default()),
                build_delivery_context::<F, TransferPurpose>(
                    DELIVERY_PURPOSE_TRANSFER,
                    Default::default(),
                ),
                &admitted_identities,
                KemConsumerConstructorParamsOverrides {
                    verifier: Some(VerifierConcrete::Mock(
                        MockIVerifierAdapterFailureMode::RpcRefused,
                    )),
                    ..Default::default()
                },
            );
            create_kem(
                &build_create_kem_deps(pairing, hash_to_scalar, consumer, Default::default()),
                build_create_kem_params(Default::default()),
                build_create_kem_payload(Default::default()),
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
        Ok(CreateKemSuccessReturn {
            output: Err(KemConsumerErrorReturn::KeyAgreement(
                KeyAgreementConsumerErrorReturn::DeliveryProof(
                    DeliveryProofConsumerErrorReturn::Measure(MeasureErrorReturn::OnChain {
                        relation: DeliveryRelation::Mint,
                        error: VerifyOnChainErrorReturn::Mock(
                            MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused
                        ),
                    })
                )
            )),
        })
    ));
    // consume_kem_returns_the_verifiers_refusal_after_the_generate_step_succeeds
}
