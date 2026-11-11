#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use chain::CreateChainFormsErrorReturn;
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, CreateEncodingErrorReturn,
    DECLARED_ENCODING_SELECTIONS, FromFieldErrorReturn, build_create_encoding_params,
};
use hash_to_scalar::{
    CreateHashToScalarErrorReturn, DECLARED_HASH_TO_SCALAR_SELECTIONS,
    build_create_hash_to_scalar_params,
};
use pairing::{
    CreatePairingPayload, CreatePairingSuccessReturn, build_create_pairing_deps,
    build_create_pairing_params, create_pairing,
};
use proof::DeliveryRelation;
use random::{
    build_create_random_source_deps, build_create_random_source_params,
    build_create_random_source_payload, create_random_source,
};

use super::interface::PairingConsumerErrorReturn;
use super::mock::{PairingConsumerConstructorParamsOverrides, build_pairing_consumer};
use crate::config::provides::{
    AdmittedEncoding, AdmittedHashToScalar, AdmittedStatementsOverrides,
    CanonicalDeliveryContextOverrides, build_admitted_identities, build_admitted_statements,
    build_canonical_delivery_context,
};
use crate::measure::provides::MeasureErrorReturn;
use crate::run::provides::{
    AdmitContextsErrorReturn, ChainFormsConsumerErrorReturn, DeliveryContextField,
    DeliveryProofConsumerErrorReturn, EncodingConsumerErrorReturn, KemConsumerErrorReturn,
    KeyAgreementConsumerErrorReturn,
};
use crate::verifier::provides::{
    MockIVerifierAdapterFailureMode, MockIVerifierAdapterVerifyOnChainErrorReturn,
    VerifierConcrete, VerifyOnChainErrorReturn,
};

/// Contract: Hash-to-scalar refused → the refusal `create_hash_to_scalar`
///   returns for an identifier the named concrete does not declare is the
///   output whole under `CreateHashToScalar`, and the encoding family is not
///   reached.
/// Arrange: the consumer constructed with `hash_to_scalar` an
///   `AdmittedHashToScalar` holding the `concrete` of the member of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS` and the `identifier` of
///   `build_create_hash_to_scalar_params(Default::default())`, and the
///   default mock concretes elsewhere.
/// Act:     `create_pairing`, once.
/// Assert:  the result matches `Ok(CreatePairingSuccessReturn { output:
///   Err(PairingConsumerErrorReturn::CreateHashToScalar(
///   CreateHashToScalarErrorReturn::UnsupportedHashToScalarIdentifier)) })`,
///   the pattern naming every field because `PairingConsumerErrorReturn`
///   omits `PartialEq`.
#[test]
fn consume_pairing_returns_the_hash_to_scalar_factorys_refusal_of_an_undeclared_identifier() {
    // Arrange
    let random_source = create_random_source(
        &build_create_random_source_deps(Default::default()),
        build_create_random_source_params(Default::default()),
        build_create_random_source_payload(Default::default()),
    )
    .expect("the mock selection constructs");
    let admitted_statements = build_admitted_statements(Default::default());
    let admitted_identities = build_admitted_identities(Default::default());
    let consumer = build_pairing_consumer(
        &*random_source.adapter,
        &admitted_statements,
        &admitted_identities,
        PairingConsumerConstructorParamsOverrides {
            hash_to_scalar: Some(AdmittedHashToScalar {
                concrete: DECLARED_HASH_TO_SCALAR_SELECTIONS[0].concrete,
                identifier: build_create_hash_to_scalar_params(Default::default()).identifier,
            }),
            ..Default::default()
        },
    );

    // Act
    let result = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(matches!(
        result,
        Ok(CreatePairingSuccessReturn {
            output: Err(PairingConsumerErrorReturn::CreateHashToScalar(
                CreateHashToScalarErrorReturn::UnsupportedHashToScalarIdentifier
            ))
        })
    ));
    // consume_pairing_returns_the_hash_to_scalar_factorys_refusal_of_an_undeclared_identifier
}

/// Contract: Encoding refused → the refusal `create_encoding` returns for an
///   identifier the named concrete does not declare is the output whole under
///   `CreateEncoding`, and the encoding consumer is not run.
/// Arrange: the consumer constructed with `encoding` an `AdmittedEncoding`
///   holding the `concrete` of the member of `DECLARED_ENCODING_SELECTIONS`
///   and the `identifier` of
///   `build_create_encoding_params(Default::default())`, and the default mock
///   concretes elsewhere.
/// Act:     `create_pairing`, once.
/// Assert:  the result matches `Ok(CreatePairingSuccessReturn { output:
///   Err(PairingConsumerErrorReturn::CreateEncoding(
///   CreateEncodingErrorReturn::UnsupportedEncodingIdentifier)) })`, the
///   pattern naming every field.
#[test]
fn consume_pairing_returns_the_encoding_factorys_refusal_of_an_undeclared_identifier() {
    // Arrange
    let random_source = create_random_source(
        &build_create_random_source_deps(Default::default()),
        build_create_random_source_params(Default::default()),
        build_create_random_source_payload(Default::default()),
    )
    .expect("the mock selection constructs");
    let admitted_statements = build_admitted_statements(Default::default());
    let admitted_identities = build_admitted_identities(Default::default());
    let consumer = build_pairing_consumer(
        &*random_source.adapter,
        &admitted_statements,
        &admitted_identities,
        PairingConsumerConstructorParamsOverrides {
            encoding: Some(AdmittedEncoding {
                concrete: DECLARED_ENCODING_SELECTIONS[0].concrete,
                identifier: build_create_encoding_params(Default::default()).identifier,
            }),
            ..Default::default()
        },
    );

    // Act
    let result = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(matches!(
        result,
        Ok(CreatePairingSuccessReturn {
            output: Err(PairingConsumerErrorReturn::CreateEncoding(
                CreateEncodingErrorReturn::UnsupportedEncodingIdentifier
            ))
        })
    ));
    // consume_pairing_returns_the_encoding_factorys_refusal_of_an_undeclared_identifier
}

/// Contract: Ordering → with both selections refused the hash-to-scalar
///   refusal is the output, so the hash-to-scalar adapter is obtained before
///   the encoding family is called.
/// Arrange: the consumer constructed with both selections as the two blocks
///   above arrange them.
/// Act:     `create_pairing`, once.
/// Assert:  the result matches `Ok(CreatePairingSuccessReturn { output:
///   Err(PairingConsumerErrorReturn::CreateHashToScalar(
///   CreateHashToScalarErrorReturn::UnsupportedHashToScalarIdentifier)) })`,
///   the pattern naming every field.
#[test]
fn consume_pairing_returns_the_hash_to_scalar_refusal_before_the_encoding_refusal() {
    // Arrange
    let random_source = create_random_source(
        &build_create_random_source_deps(Default::default()),
        build_create_random_source_params(Default::default()),
        build_create_random_source_payload(Default::default()),
    )
    .expect("the mock selection constructs");
    let admitted_statements = build_admitted_statements(Default::default());
    let admitted_identities = build_admitted_identities(Default::default());
    let consumer = build_pairing_consumer(
        &*random_source.adapter,
        &admitted_statements,
        &admitted_identities,
        PairingConsumerConstructorParamsOverrides {
            hash_to_scalar: Some(AdmittedHashToScalar {
                concrete: DECLARED_HASH_TO_SCALAR_SELECTIONS[0].concrete,
                identifier: build_create_hash_to_scalar_params(Default::default()).identifier,
            }),
            encoding: Some(AdmittedEncoding {
                concrete: DECLARED_ENCODING_SELECTIONS[0].concrete,
                identifier: build_create_encoding_params(Default::default()).identifier,
            }),
            ..Default::default()
        },
    );

    // Act
    let result = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(matches!(
        result,
        Ok(CreatePairingSuccessReturn {
            output: Err(PairingConsumerErrorReturn::CreateHashToScalar(
                CreateHashToScalarErrorReturn::UnsupportedHashToScalarIdentifier
            ))
        })
    ));
    // consume_pairing_returns_the_hash_to_scalar_refusal_before_the_encoding_refusal
}

/// Contract: Encoding consumer refused → the refusal the `EncodingConsumer`
///   returns for an encoder declaring another encoding identifier than the
///   forms concrete requires is the output whole, inside
///   `PairingConsumerErrorReturn::Encoding`, and is neither converted nor
///   summarized.
/// Arrange: the consumer at its defaults, which select the mock forms
///   concrete, with `encoding` an `AdmittedEncoding` holding the `concrete`
///   and `identifier` of the member of `DECLARED_ENCODING_SELECTIONS`, the
///   Ethereum ABI selection.
/// Act:     `create_pairing`, once.
/// Assert:  the result matches `Ok(CreatePairingSuccessReturn { output:
///   Err(PairingConsumerErrorReturn::Encoding(
///   EncodingConsumerErrorReturn::CreateChainForms(
///   CreateChainFormsErrorReturn::UnsupportedEncodingIdentifier)))) })`, the
///   pattern naming every field.
#[test]
fn consume_pairing_returns_the_encoding_consumers_forms_refusal_whole() {
    // Arrange
    let random_source = create_random_source(
        &build_create_random_source_deps(Default::default()),
        build_create_random_source_params(Default::default()),
        build_create_random_source_payload(Default::default()),
    )
    .expect("the mock selection constructs");
    let admitted_statements = build_admitted_statements(Default::default());
    let admitted_identities = build_admitted_identities(Default::default());
    let consumer = build_pairing_consumer(
        &*random_source.adapter,
        &admitted_statements,
        &admitted_identities,
        PairingConsumerConstructorParamsOverrides {
            encoding: Some(AdmittedEncoding {
                concrete: DECLARED_ENCODING_SELECTIONS[0].concrete,
                identifier: DECLARED_ENCODING_SELECTIONS[0].identifier,
            }),
            ..Default::default()
        },
    );

    // Act
    let result = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(matches!(
        result,
        Ok(CreatePairingSuccessReturn {
            output: Err(PairingConsumerErrorReturn::Encoding(
                EncodingConsumerErrorReturn::CreateChainForms(
                    CreateChainFormsErrorReturn::UnsupportedEncodingIdentifier
                )
            ))
        })
    ));
    // consume_pairing_returns_the_encoding_consumers_forms_refusal_whole
}

/// Contract: Forms consumer refused → the refusal the `ChainFormsConsumer`
///   returns reaches the output whole, inside
///   `PairingConsumerErrorReturn::Encoding`.
/// Arrange: `AdmittedStatements` whose `mint` is
///   `build_canonical_delivery_context` overriding `seller` with a
///   `CanonicalFieldValue::Text` and whose `transfer` is the builder at its
///   defaults, with the default mock concretes elsewhere.
/// Act:     `create_pairing`, once.
/// Assert:  the result matches `Ok(CreatePairingSuccessReturn { output:
///   Err(PairingConsumerErrorReturn::Encoding(
///   EncodingConsumerErrorReturn::ChainForms(
///   ChainFormsConsumerErrorReturn::AdmitMint(AdmitContextsErrorReturn::Field
///   { field: DeliveryContextField::Seller, error:
///   FromFieldErrorReturn::WrongKind { expected:
///   CanonicalFieldKind::FixedBytes20 } })))) })`, the pattern naming every
///   field.
#[test]
fn consume_pairing_returns_the_forms_consumers_refusal_whole() {
    // Arrange
    let random_source = create_random_source(
        &build_create_random_source_deps(Default::default()),
        build_create_random_source_params(Default::default()),
        build_create_random_source_payload(Default::default()),
    )
    .expect("the mock selection constructs");
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
    let consumer = build_pairing_consumer(
        &*random_source.adapter,
        &admitted_statements,
        &admitted_identities,
        PairingConsumerConstructorParamsOverrides::default(),
    );

    // Act
    let result = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(matches!(
        result,
        Ok(CreatePairingSuccessReturn {
            output: Err(PairingConsumerErrorReturn::Encoding(
                EncodingConsumerErrorReturn::ChainForms(ChainFormsConsumerErrorReturn::AdmitMint(
                    AdmitContextsErrorReturn::Field {
                        field: DeliveryContextField::Seller,
                        error: FromFieldErrorReturn::WrongKind {
                            expected: CanonicalFieldKind::FixedBytes20
                        }
                    }
                ))
            ))
        })
    ));
    // consume_pairing_returns_the_forms_consumers_refusal_whole
}

/// Contract: Measurement refused → the refusal `measure` returns reaches the
///   output whole, inside `PairingConsumerErrorReturn::Encoding`.
/// Arrange: the consumer constructed with `verifier` `VerifierConcrete::Mock(
///   MockIVerifierAdapterFailureMode::RpcRefused)` and the default mock
///   concretes elsewhere.
/// Act:     `create_pairing`, once.
/// Assert:  the result matches `Ok(CreatePairingSuccessReturn { output:
///   Err(PairingConsumerErrorReturn::Encoding(
///   EncodingConsumerErrorReturn::ChainForms(ChainFormsConsumerErrorReturn::
///   Kem(KemConsumerErrorReturn::KeyAgreement(
///   KeyAgreementConsumerErrorReturn::DeliveryProof(
///   DeliveryProofConsumerErrorReturn::Measure(MeasureErrorReturn::OnChain {
///   relation: DeliveryRelation::Mint, error: VerifyOnChainErrorReturn::Mock(
///   MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused) }))))))) })`,
///   the pattern naming every field.
#[test]
fn consume_pairing_returns_the_verifiers_refusal_after_the_generate_step_succeeds() {
    // Arrange
    let random_source = create_random_source(
        &build_create_random_source_deps(Default::default()),
        build_create_random_source_params(Default::default()),
        build_create_random_source_payload(Default::default()),
    )
    .expect("the mock selection constructs");
    let admitted_statements = build_admitted_statements(Default::default());
    let admitted_identities = build_admitted_identities(Default::default());
    let consumer = build_pairing_consumer(
        &*random_source.adapter,
        &admitted_statements,
        &admitted_identities,
        PairingConsumerConstructorParamsOverrides {
            verifier: Some(VerifierConcrete::Mock(
                MockIVerifierAdapterFailureMode::RpcRefused,
            )),
            ..Default::default()
        },
    );

    // Act
    let result = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(matches!(
        result,
        Ok(CreatePairingSuccessReturn {
            output: Err(PairingConsumerErrorReturn::Encoding(
                EncodingConsumerErrorReturn::ChainForms(ChainFormsConsumerErrorReturn::Kem(
                    KemConsumerErrorReturn::KeyAgreement(
                        KeyAgreementConsumerErrorReturn::DeliveryProof(
                            DeliveryProofConsumerErrorReturn::Measure(
                                MeasureErrorReturn::OnChain {
                                    relation: DeliveryRelation::Mint,
                                    error: VerifyOnChainErrorReturn::Mock(
                                        MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused
                                    ),
                                }
                            )
                        )
                    )
                ))
            ))
        })
    ));
    // consume_pairing_returns_the_verifiers_refusal_after_the_generate_step_succeeds
}
