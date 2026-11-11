#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use chain::{
    CreateChainFormsErrorReturn, DECLARED_CHAIN_FORMS_SELECTIONS, IChainForms,
    build_create_chain_forms_params,
};
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, CreateEncodingParamsOverrides, CreateEncodingReturn,
    CreateEncodingSuccessReturn, DECLARED_ENCODING_SELECTIONS, FromFieldErrorReturn,
    IDecoderAdapter, IEncoderAdapter, build_create_encoding_deps, build_create_encoding_params,
    build_create_encoding_payload, create_encoding,
};
use envelope::IKeyAgreementAdapter;
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::ICredentialKemAdapter;
use pairing::{IPairingArithmetic, IPairingReference};
use proof::{DeliveryRelation, IDeliveryProofAdapter};

use super::interface::{EncodingConsumerErrorReturn, EncodingConsumerReturn};
use super::mock::{EncodingConsumerConstructorParamsOverrides, build_encoding_consumer};
use crate::config::provides::{
    AdmittedChainForms, AdmittedStatementsOverrides, CanonicalDeliveryContextOverrides,
    build_admitted_identities, build_admitted_statements, build_canonical_delivery_context,
};
use crate::generate::provides::{
    ConsumeCreateGenerateCaseParams, ConsumeCreateGenerateCasePayload, ICreateGenerateCaseConsumer,
    build_create_generate_case_deps, build_create_generate_case_params,
    build_create_generate_case_payload, run_create_generate_case,
};
use crate::measure::provides::MeasureErrorReturn;
use crate::run::provides::{
    AdmitContextsErrorReturn, ChainFormsConsumerErrorReturn, DeliveryContextField,
    DeliveryProofConsumerErrorReturn, KemConsumerErrorReturn, KeyAgreementConsumerErrorReturn,
};
use crate::verifier::provides::{
    MockIVerifierAdapterFailureMode, MockIVerifierAdapterVerifyOnChainErrorReturn,
    VerifierConcrete, VerifyOnChainErrorReturn,
};

/// Contract: Forms refused → the refusal `create_chain_forms` returns for a
///   forms identifier the named concrete does not declare is the output whole
///   under `CreateChainForms`, and the forms consumer is not reached.
/// Arrange: the consumer constructed with `chain_forms` an `AdmittedChainForms`
///   holding the `concrete` of the member of `DECLARED_CHAIN_FORMS_SELECTIONS`
///   and the `identifier` of `build_create_chain_forms_params(Default::default
///   ())`, the default mock encoding selection passed to `create_encoding`,
///   and the default mock concretes elsewhere.
/// Act:     `create_encoding`, once, inside the consumer.
/// Assert:  the output matches `Ok(CreateEncodingSuccessReturn { output:
///   Err(EncodingConsumerErrorReturn::CreateChainForms(
///   CreateChainFormsErrorReturn::UnsupportedChainFormsIdentifier)) })`, the
///   pattern naming every field because `EncodingConsumerErrorReturn` omits
///   `PartialEq`.
#[test]
fn consume_encoding_returns_the_forms_functions_refusal_of_an_undeclared_forms_identifier() {
    // Arrange
    struct CaseConsumer;

    impl ICreateGenerateCaseConsumer for CaseConsumer {
        type Output = CreateEncodingReturn<EncodingConsumerReturn>;

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
            let consumer = build_encoding_consumer(
                payload.pairing,
                payload.hash_to_scalar,
                payload.random,
                &admitted_statements,
                &admitted_identities,
                EncodingConsumerConstructorParamsOverrides {
                    chain_forms: Some(AdmittedChainForms {
                        concrete: DECLARED_CHAIN_FORMS_SELECTIONS[0].concrete,
                        identifier: build_create_chain_forms_params(Default::default()).identifier,
                    }),
                    ..Default::default()
                },
            );
            create_encoding(
                &build_create_encoding_deps(consumer),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
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
        Ok(CreateEncodingSuccessReturn {
            output: Err(EncodingConsumerErrorReturn::CreateChainForms(
                CreateChainFormsErrorReturn::UnsupportedChainFormsIdentifier
            )),
        })
    ));
    // consume_encoding_returns_the_forms_functions_refusal_of_an_undeclared_forms_identifier
}

/// Contract: Encoder refused → the refusal `create_chain_forms` returns for an
///   encoder whose declared identifier the named forms concrete does not
///   require is the output whole under `CreateChainForms`.
/// Arrange: the consumer at its defaults, which select the mock forms
///   concrete, and the Ethereum ABI selection of `DECLARED_ENCODING_SELECTIONS`
///   passed to `create_encoding`.
/// Act:     `create_encoding`, once, inside the consumer.
/// Assert:  the output matches `Ok(CreateEncodingSuccessReturn { output:
///   Err(EncodingConsumerErrorReturn::CreateChainForms(
///   CreateChainFormsErrorReturn::UnsupportedEncodingIdentifier)) })`, the
///   pattern naming every field.
#[test]
fn consume_encoding_returns_the_forms_functions_refusal_of_an_encoder_declaring_another_encoding_identifier()
 {
    // Arrange
    struct CaseConsumer;

    impl ICreateGenerateCaseConsumer for CaseConsumer {
        type Output = CreateEncodingReturn<EncodingConsumerReturn>;

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
            let consumer = build_encoding_consumer(
                payload.pairing,
                payload.hash_to_scalar,
                payload.random,
                &admitted_statements,
                &admitted_identities,
                Default::default(),
            );
            create_encoding(
                &build_create_encoding_deps(consumer),
                build_create_encoding_params(CreateEncodingParamsOverrides {
                    concrete: Some(DECLARED_ENCODING_SELECTIONS[0].concrete),
                    identifier: Some(DECLARED_ENCODING_SELECTIONS[0].identifier),
                }),
                build_create_encoding_payload(Default::default()),
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
        Ok(CreateEncodingSuccessReturn {
            output: Err(EncodingConsumerErrorReturn::CreateChainForms(
                CreateChainFormsErrorReturn::UnsupportedEncodingIdentifier
            )),
        })
    ));
    // consume_encoding_returns_the_forms_functions_refusal_of_an_encoder_declaring_another_encoding_identifier
}

/// Contract: Forms consumer refused → the refusal the `ChainFormsConsumer`
///   returns is the output whole, inside `EncodingConsumerErrorReturn::
///   ChainForms`, and is neither converted nor summarized.
/// Arrange: `AdmittedStatements` whose `mint` is
///   `build_canonical_delivery_context` overriding `seller` with a
///   `CanonicalFieldValue::Text` and whose `transfer` is the builder at its
///   defaults, with the default mock concretes elsewhere.
/// Act:     `create_encoding`, once, inside the consumer.
/// Assert:  the output matches `Ok(CreateEncodingSuccessReturn { output:
///   Err(EncodingConsumerErrorReturn::ChainForms(
///   ChainFormsConsumerErrorReturn::AdmitMint(AdmitContextsErrorReturn::Field
///   { field: DeliveryContextField::Seller, error:
///   FromFieldErrorReturn::WrongKind { expected:
///   CanonicalFieldKind::FixedBytes20 } }))) })`, the pattern naming every
///   field.
#[test]
fn consume_encoding_returns_the_forms_consumers_refusal_whole() {
    // Arrange
    struct CaseConsumer;

    impl ICreateGenerateCaseConsumer for CaseConsumer {
        type Output = CreateEncodingReturn<EncodingConsumerReturn>;

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
                mint: Some(build_canonical_delivery_context(
                    CanonicalDeliveryContextOverrides {
                        seller: Some(CanonicalFieldValue::Text("refused".to_string())),
                        ..Default::default()
                    },
                )),
                ..Default::default()
            });
            let admitted_identities = build_admitted_identities(Default::default());
            let consumer = build_encoding_consumer(
                payload.pairing,
                payload.hash_to_scalar,
                payload.random,
                &admitted_statements,
                &admitted_identities,
                Default::default(),
            );
            create_encoding(
                &build_create_encoding_deps(consumer),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
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
        Ok(CreateEncodingSuccessReturn {
            output: Err(EncodingConsumerErrorReturn::ChainForms(
                ChainFormsConsumerErrorReturn::AdmitMint(AdmitContextsErrorReturn::Field {
                    field: DeliveryContextField::Seller,
                    error: FromFieldErrorReturn::WrongKind {
                        expected: CanonicalFieldKind::FixedBytes20
                    }
                })
            )),
        })
    ));
    // consume_encoding_returns_the_forms_consumers_refusal_whole
}

/// Contract: Measurement refused → the refusal `measure` returns reaches the
///   output whole, inside `EncodingConsumerErrorReturn::ChainForms`.
/// Arrange: the consumer constructed with `verifier` `VerifierConcrete::Mock(
///   MockIVerifierAdapterFailureMode::RpcRefused)` and the default mock
///   concretes elsewhere.
/// Act:     `create_encoding`, once, inside the consumer.
/// Assert:  the output matches `Ok(CreateEncodingSuccessReturn { output:
///   Err(EncodingConsumerErrorReturn::ChainForms(
///   ChainFormsConsumerErrorReturn::Kem(KemConsumerErrorReturn::KeyAgreement(
///   KeyAgreementConsumerErrorReturn::DeliveryProof(
///   DeliveryProofConsumerErrorReturn::Measure(MeasureErrorReturn::OnChain
///   { relation: DeliveryRelation::Mint, error:
///   VerifyOnChainErrorReturn::Mock(
///   MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused) })))))) })`,
///   the pattern naming every field.
#[test]
fn consume_encoding_returns_the_verifiers_refusal_after_the_generate_step_succeeds() {
    // Arrange
    struct CaseConsumer;

    impl ICreateGenerateCaseConsumer for CaseConsumer {
        type Output = CreateEncodingReturn<EncodingConsumerReturn>;

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
            let consumer = build_encoding_consumer(
                payload.pairing,
                payload.hash_to_scalar,
                payload.random,
                &admitted_statements,
                &admitted_identities,
                EncodingConsumerConstructorParamsOverrides {
                    verifier: Some(VerifierConcrete::Mock(
                        MockIVerifierAdapterFailureMode::RpcRefused,
                    )),
                    ..Default::default()
                },
            );
            create_encoding(
                &build_create_encoding_deps(consumer),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
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
        Ok(CreateEncodingSuccessReturn {
            output: Err(EncodingConsumerErrorReturn::ChainForms(
                ChainFormsConsumerErrorReturn::Kem(KemConsumerErrorReturn::KeyAgreement(
                    KeyAgreementConsumerErrorReturn::DeliveryProof(
                        DeliveryProofConsumerErrorReturn::Measure(MeasureErrorReturn::OnChain {
                            relation: DeliveryRelation::Mint,
                            error: VerifyOnChainErrorReturn::Mock(
                                MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused
                            )
                        })
                    )
                ))
            )),
        })
    ));
    // consume_encoding_returns_the_verifiers_refusal_after_the_generate_step_succeeds
}
