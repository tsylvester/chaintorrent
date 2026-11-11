#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use chain::{
    CreateChainFormsReturn, CreateChainFormsSuccessReturn, IChainForms,
    build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, create_chain_forms,
};
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, FromFieldErrorReturn, IDecoderAdapter, IEncoderAdapter,
};
use envelope::IKeyAgreementAdapter;
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::{
    CreateKemErrorReturn, DECLARED_KEM_SELECTIONS, ICredentialKemAdapter, KemConcrete,
    build_create_kem_params,
};
use pairing::{IPairingArithmetic, IPairingReference};
use proof::{DeliveryRelation, IDeliveryProofAdapter};

use super::interface::{ChainFormsConsumerErrorReturn, ChainFormsConsumerReturn};
use super::mock::{ChainFormsConsumerConstructorParamsOverrides, build_chain_forms_consumer};
use crate::config::provides::{
    AdmittedKem, AdmittedStatementsOverrides, CanonicalDeliveryContextOverrides,
    build_admitted_identities, build_admitted_statements, build_canonical_delivery_context,
};
use crate::generate::provides::{
    ConsumeCreateGenerateCaseParams, ConsumeCreateGenerateCasePayload, EmitErrorReturn,
    GenerateConcrete, ICreateGenerateCaseConsumer, MockIGenerateAdapterEmitErrorReturn,
    MockIGenerateAdapterFailureMode, build_create_generate_case_deps,
    build_create_generate_case_params, build_create_generate_case_payload,
    run_create_generate_case,
};
use crate::measure::provides::MeasureErrorReturn;
use crate::run::provides::{
    AdmitContextsErrorReturn, DeliveryContextField, DeliveryProofConsumerErrorReturn,
    KemConsumerErrorReturn, KeyAgreementConsumerErrorReturn,
};
use crate::verifier::provides::{
    MockIVerifierAdapterFailureMode, MockIVerifierAdapterVerifyOnChainErrorReturn,
    VerifierConcrete, VerifyOnChainErrorReturn,
};

/// Contract: Mint admission refused → the refusal `admit_contexts` returns for
///   the mint context is the output whole under `AdmitMint`, and the transfer
///   context is not admitted and the KEM factory is not called.
/// Arrange: `AdmittedStatements` whose `mint` is
///   `build_canonical_delivery_context` overriding `seller` with a
///   `CanonicalFieldValue::Text`, a kind no form declares, and whose `transfer`
///   is the builder at its defaults, with `kem` an `AdmittedKem` of the
///   `Bb1DepthOne` member of `DECLARED_KEM_SELECTIONS`' `concrete` and `scope`
///   and the `identifier` of `build_create_kem_params(Default::default())`,
///   which the factory would refuse.
/// Act:     `create_chain_forms`, once, inside the consumer.
/// Assert:  the output matches
///   `Err(ChainFormsConsumerErrorReturn::AdmitMint(
///   AdmitContextsErrorReturn::Field { field: DeliveryContextField::Seller,
///   error: FromFieldErrorReturn::WrongKind { expected:
///   CanonicalFieldKind::FixedBytes20 } }))`, the pattern naming every field
///   because `ChainFormsConsumerErrorReturn` omits `PartialEq`.
#[test]
fn consume_chain_forms_returns_the_mint_contexts_refusal_with_the_field_that_held_the_value() {
    // Arrange
    struct CaseConsumer;

    impl ICreateGenerateCaseConsumer for CaseConsumer {
        type Output = CreateChainFormsReturn<ChainFormsConsumerReturn>;

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
            let declared = DECLARED_KEM_SELECTIONS
                .iter()
                .find(|selection| selection.concrete == KemConcrete::Bb1DepthOne)
                .expect("the declared set names the Bb1DepthOne member");
            let admitted_statements = build_admitted_statements(AdmittedStatementsOverrides {
                mint: Some(build_canonical_delivery_context(
                    CanonicalDeliveryContextOverrides {
                        seller: Some(CanonicalFieldValue::Text("refused".to_string())),
                        ..Default::default()
                    },
                )),
                ..Default::default()
            });
            let admitted_identities = build_admitted_identities(Default::default());
            let consumer = build_chain_forms_consumer(
                payload.pairing,
                payload.hash_to_scalar,
                payload.encoder,
                payload.random,
                &admitted_statements,
                &admitted_identities,
                ChainFormsConsumerConstructorParamsOverrides {
                    kem: Some(AdmittedKem {
                        concrete: declared.concrete,
                        identifier: build_create_kem_params(Default::default()).identifier,
                        scope: declared.scope,
                    }),
                    ..Default::default()
                },
            );
            create_chain_forms(
                &build_create_chain_forms_deps(consumer, payload.encoder, Default::default()),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
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
        Ok(CreateChainFormsSuccessReturn {
            output: Err(ChainFormsConsumerErrorReturn::AdmitMint(
                AdmitContextsErrorReturn::Field {
                    field: DeliveryContextField::Seller,
                    error: FromFieldErrorReturn::WrongKind {
                        expected: CanonicalFieldKind::FixedBytes20
                    }
                }
            )),
        })
    ));
    // consume_chain_forms_returns_the_mint_contexts_refusal_with_the_field_that_held_the_value
}

/// Contract: Transfer admission refused → the refusal `admit_contexts` returns
///   for the transfer context is the output whole under `AdmitTransfer`.
/// Arrange: `AdmittedStatements` whose `mint` is the builder at its defaults
///   and whose `transfer` overrides `buyer` with a `CanonicalFieldValue::Text`.
/// Act:     `create_chain_forms`, once, inside the consumer.
/// Assert:  the output matches
///   `Err(ChainFormsConsumerErrorReturn::AdmitTransfer(
///   AdmitContextsErrorReturn::Field { field: DeliveryContextField::Buyer,
///   error: FromFieldErrorReturn::WrongKind { expected:
///   CanonicalFieldKind::FixedBytes20 } }))`, the pattern naming every field.
#[test]
fn consume_chain_forms_returns_the_transfer_contexts_refusal_after_the_mint_context_is_admitted() {
    // Arrange
    struct CaseConsumer;

    impl ICreateGenerateCaseConsumer for CaseConsumer {
        type Output = CreateChainFormsReturn<ChainFormsConsumerReturn>;

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
            let admitted_statements = build_admitted_statements(AdmittedStatementsOverrides {
                transfer: Some(build_canonical_delivery_context(
                    CanonicalDeliveryContextOverrides {
                        buyer: Some(CanonicalFieldValue::Text("refused".to_string())),
                        ..Default::default()
                    },
                )),
                ..Default::default()
            });
            let admitted_identities = build_admitted_identities(Default::default());
            let consumer = build_chain_forms_consumer(
                payload.pairing,
                payload.hash_to_scalar,
                payload.encoder,
                payload.random,
                &admitted_statements,
                &admitted_identities,
                Default::default(),
            );
            create_chain_forms(
                &build_create_chain_forms_deps(consumer, payload.encoder, Default::default()),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
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
        Ok(CreateChainFormsSuccessReturn {
            output: Err(ChainFormsConsumerErrorReturn::AdmitTransfer(
                AdmitContextsErrorReturn::Field {
                    field: DeliveryContextField::Buyer,
                    error: FromFieldErrorReturn::WrongKind {
                        expected: CanonicalFieldKind::FixedBytes20
                    }
                }
            )),
        })
    ));
    // consume_chain_forms_returns_the_transfer_contexts_refusal_after_the_mint_context_is_admitted
}

/// Contract: KEM refused at construction → the refusal `create_kem` returns is
///   the output whole, and the KEM consumer is not reached.
/// Arrange: the consumer constructed with `kem` an `AdmittedKem` holding the
///   `Bb1DepthOne` member of `DECLARED_KEM_SELECTIONS`' `concrete` and `scope`
///   and the `identifier` of `build_create_kem_params(Default::default())`, and
///   the default mock concretes elsewhere.
/// Act:     `create_chain_forms`, once, inside the consumer.
/// Assert:  the output matches
///   `Err(ChainFormsConsumerErrorReturn::CreateKem(
///   CreateKemErrorReturn::UnsupportedKemIdentifier))`, the pattern naming
///   every field.
#[test]
fn consume_chain_forms_returns_the_kem_factorys_refusal_for_an_undeclared_identifier() {
    // Arrange
    struct CaseConsumer;

    impl ICreateGenerateCaseConsumer for CaseConsumer {
        type Output = CreateChainFormsReturn<ChainFormsConsumerReturn>;

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
            let declared = DECLARED_KEM_SELECTIONS
                .iter()
                .find(|selection| selection.concrete == KemConcrete::Bb1DepthOne)
                .expect("the declared set names the Bb1DepthOne member");
            let admitted_statements = build_admitted_statements(Default::default());
            let admitted_identities = build_admitted_identities(Default::default());
            let consumer = build_chain_forms_consumer(
                payload.pairing,
                payload.hash_to_scalar,
                payload.encoder,
                payload.random,
                &admitted_statements,
                &admitted_identities,
                ChainFormsConsumerConstructorParamsOverrides {
                    kem: Some(AdmittedKem {
                        concrete: declared.concrete,
                        identifier: build_create_kem_params(Default::default()).identifier,
                        scope: declared.scope,
                    }),
                    ..Default::default()
                },
            );
            create_chain_forms(
                &build_create_chain_forms_deps(consumer, payload.encoder, Default::default()),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
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
        Ok(CreateChainFormsSuccessReturn {
            output: Err(ChainFormsConsumerErrorReturn::CreateKem(
                CreateKemErrorReturn::UnsupportedKemIdentifier
            )),
        })
    ));
    // consume_chain_forms_returns_the_kem_factorys_refusal_for_an_undeclared_identifier
}

/// Contract: KEM consumer refused → the refusal the KEM consumer returns is the
///   output whole, inside `ChainFormsConsumerErrorReturn::Kem`.
/// Arrange: the consumer constructed with `generate` `GenerateConcrete::Mock(
///   MockIGenerateAdapterFailureMode::WriteRefused)` and `verifier`
///   `VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::RpcRefused)`.
/// Act:     `create_chain_forms`, once, inside the consumer.
/// Assert:  the output matches
///   `Err(ChainFormsConsumerErrorReturn::Kem(KemConsumerErrorReturn::
///   KeyAgreement(KeyAgreementConsumerErrorReturn::DeliveryProof(
///   DeliveryProofConsumerErrorReturn::Emit(EmitErrorReturn::Mock(
///   MockIGenerateAdapterEmitErrorReturn::WriteRefused))))))`, the pattern
///   naming every field.
#[test]
fn consume_chain_forms_returns_the_kem_consumers_refusal_whole() {
    // Arrange
    struct CaseConsumer;

    impl ICreateGenerateCaseConsumer for CaseConsumer {
        type Output = CreateChainFormsReturn<ChainFormsConsumerReturn>;

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
            let admitted_statements = build_admitted_statements(Default::default());
            let admitted_identities = build_admitted_identities(Default::default());
            let consumer = build_chain_forms_consumer(
                payload.pairing,
                payload.hash_to_scalar,
                payload.encoder,
                payload.random,
                &admitted_statements,
                &admitted_identities,
                ChainFormsConsumerConstructorParamsOverrides {
                    generate: Some(GenerateConcrete::Mock(
                        MockIGenerateAdapterFailureMode::WriteRefused,
                    )),
                    verifier: Some(VerifierConcrete::Mock(
                        MockIVerifierAdapterFailureMode::RpcRefused,
                    )),
                    ..Default::default()
                },
            );
            create_chain_forms(
                &build_create_chain_forms_deps(consumer, payload.encoder, Default::default()),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
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
        Ok(CreateChainFormsSuccessReturn {
            output: Err(ChainFormsConsumerErrorReturn::Kem(
                KemConsumerErrorReturn::KeyAgreement(
                    KeyAgreementConsumerErrorReturn::DeliveryProof(
                        DeliveryProofConsumerErrorReturn::Emit(EmitErrorReturn::Mock(
                            MockIGenerateAdapterEmitErrorReturn::WriteRefused
                        ))
                    )
                )
            )),
        })
    ));
    // consume_chain_forms_returns_the_kem_consumers_refusal_whole
}

/// Contract: Measurement refused → the refusal `measure` returns reaches the
///   output whole, neither converted nor summarized.
/// Arrange: the consumer constructed with `verifier` `VerifierConcrete::Mock(
///   MockIVerifierAdapterFailureMode::RpcRefused)` and the default mock
///   concretes elsewhere.
/// Act:     `create_chain_forms`, once, inside the consumer.
/// Assert:  the output matches
///   `Err(ChainFormsConsumerErrorReturn::Kem(KemConsumerErrorReturn::
///   KeyAgreement(KeyAgreementConsumerErrorReturn::DeliveryProof(
///   DeliveryProofConsumerErrorReturn::Measure(MeasureErrorReturn::OnChain {
///   relation: DeliveryRelation::Mint, error: VerifyOnChainErrorReturn::Mock(
///   MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused) })))))`, the
///   pattern naming every field.
#[test]
fn consume_chain_forms_returns_the_verifiers_refusal_after_the_generate_step_succeeds() {
    // Arrange
    struct CaseConsumer;

    impl ICreateGenerateCaseConsumer for CaseConsumer {
        type Output = CreateChainFormsReturn<ChainFormsConsumerReturn>;

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
            let admitted_statements = build_admitted_statements(Default::default());
            let admitted_identities = build_admitted_identities(Default::default());
            let consumer = build_chain_forms_consumer(
                payload.pairing,
                payload.hash_to_scalar,
                payload.encoder,
                payload.random,
                &admitted_statements,
                &admitted_identities,
                ChainFormsConsumerConstructorParamsOverrides {
                    verifier: Some(VerifierConcrete::Mock(
                        MockIVerifierAdapterFailureMode::RpcRefused,
                    )),
                    ..Default::default()
                },
            );
            create_chain_forms(
                &build_create_chain_forms_deps(consumer, payload.encoder, Default::default()),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
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
        Ok(CreateChainFormsSuccessReturn {
            output: Err(ChainFormsConsumerErrorReturn::Kem(
                KemConsumerErrorReturn::KeyAgreement(
                    KeyAgreementConsumerErrorReturn::DeliveryProof(
                        DeliveryProofConsumerErrorReturn::Measure(MeasureErrorReturn::OnChain {
                            relation: DeliveryRelation::Mint,
                            error: VerifyOnChainErrorReturn::Mock(
                                MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused
                            ),
                        })
                    )
                )
            )),
        })
    ));
    // consume_chain_forms_returns_the_verifiers_refusal_after_the_generate_step_succeeds
}
