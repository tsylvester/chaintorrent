#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::path::PathBuf;

use chain::{DECLARED_CHAIN_FORMS_SELECTIONS, IChainForms, build_create_chain_forms_params};
use core::num::NonZeroU32;
use encoding::{
    CreateEncodingParamsOverrides, CreateEncodingReturn, CreateEncodingSuccessReturn,
    DECLARED_ENCODING_SELECTIONS, IDecoderAdapter, IEncoderAdapter, build_create_encoding_deps,
    build_create_encoding_params, build_create_encoding_payload, create_encoding,
};
use envelope::{IKeyAgreementAdapter, build_create_key_agreement_params};
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::{ICredentialKemAdapter, build_create_kem_params};
use pairing::{IPairingArithmetic, IPairingReference};
use proof::{IDeliveryProofAdapter, build_create_delivery_proof_params};

use super::interface::{EncodingConsumer, EncodingConsumerReturn};
use super::mock::{EncodingConsumerConstructorParamsOverrides, build_encoding_consumer};
use crate::config::provides::{
    AdmittedChainForms, AdmittedKem, AdmittedKeyAgreement, build_admitted_identities,
    build_admitted_statements,
};
use crate::generate::provides::{
    ConsumeCreateGenerateCaseParams, ConsumeCreateGenerateCasePayload, EmittedFileOverrides,
    GenerateConcreteKind, GenerateConcreteOverrides, GenerateKindConfigurationsOverrides,
    ICreateGenerateCaseConsumer, VectorKind, build_create_generate_case_deps,
    build_create_generate_case_params, build_create_generate_case_payload,
    build_create_generate_params, build_emitted_file, build_generate_concrete,
    build_generate_kind_configurations, run_create_generate_case,
};
use crate::verifier::provides::{
    L1DataFee, MockIVerifierAdapterFailureMode, VerifierConcrete, VerifyCost,
};

/// Contract: Consumed → the files the mock generate concrete returns and the
///   measurement the verifier step returns are the output whole, the files in
///   write order and the measurement's iteration count the consumer's and its
///   costs the verifier's accepted cost.
/// Arrange: the default params, which select the mock forms, KEM, key
///   agreement, delivery proof, generate, and verifier concretes under
///   `NoFailure`, and the mock encoding selection; `iterations` of `3`, a count
///   other than the builder's default; the expected files the five kinds the
///   mock concrete's declaration admits with each `path` the kind's name as
///   text, and the expected cost the literal `VerifyCost { l2_execution_gas:
///   100_000, l1_data_fee: L1DataFee::Charged { wei: 1_000 } }`, as
///   independent literals.
/// Act:     `create_encoding`, once, inside the consumer.
/// Assert:  the output matches `Ok(CreateEncodingSuccessReturn { output:
///   Ok(DeliveryProofConsumerSuccessReturn { files, measurement }) })`,
///   `files` equals the expected files in that order by `assert_eq!`,
///   `measurement.iterations.get()` equals `3`, and `measurement.costs.mint`
///   and `measurement.costs.transfer` each equal the expected cost.
#[test]
fn consume_encoding_returns_the_files_and_the_measurement_both_steps_computed() {
    // Arrange
    let expected_files = vec![
        build_emitted_file(EmittedFileOverrides {
            kind: Some(Some(VectorKind::G1Addition)),
            path: Some(PathBuf::from("G1Addition")),
        }),
        build_emitted_file(EmittedFileOverrides {
            kind: Some(Some(VectorKind::G1MultiScalarMultiplication)),
            path: Some(PathBuf::from("G1MultiScalarMultiplication")),
        }),
        build_emitted_file(EmittedFileOverrides {
            kind: Some(Some(VectorKind::HashToScalar)),
            path: Some(PathBuf::from("HashToScalar")),
        }),
        build_emitted_file(EmittedFileOverrides {
            kind: Some(Some(VectorKind::DecodeRejection)),
            path: Some(PathBuf::from("DecodeRejection")),
        }),
        build_emitted_file(EmittedFileOverrides {
            kind: Some(Some(VectorKind::IdentityMapping)),
            path: Some(PathBuf::from("IdentityMapping")),
        }),
    ];
    let expected_cost = VerifyCost {
        l2_execution_gas: 100_000,
        l1_data_fee: L1DataFee::Charged { wei: 1_000 },
    };

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
            let consumer: EncodingConsumer<'_, P> = build_encoding_consumer(
                payload.pairing,
                payload.hash_to_scalar,
                payload.random,
                &admitted_statements,
                &admitted_identities,
                EncodingConsumerConstructorParamsOverrides {
                    iterations: NonZeroU32::new(3),
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
    let output = result
        .expect("the case composes")
        .output
        .expect("the factory admits the mock concrete and runs the consumer")
        .output
        .expect("the consumer calls the forms function and the chain beneath it");
    assert_eq!(output.files, expected_files);
    assert_eq!(output.measurement.iterations.get(), 3);
    assert_eq!(output.measurement.costs.mint, expected_cost);
    assert_eq!(output.measurement.costs.transfer, expected_cost);
    // consume_encoding_returns_the_files_and_the_measurement_both_steps_computed
}

/// Contract: Construction → the consumer holds the adapters, the admitted
///   statements, the admitted identities, and the configured values it is
///   constructed with.
/// Arrange: `AdmittedStatements` and `AdmittedIdentities` built with
///   `build_admitted_statements` and `build_admitted_identities` at their
///   defaults, `statement_version` of `2`, `iterations` of `7`, and the
///   received adapters.
/// Act:     `build_encoding_consumer`, once, inside the case consumer's
///   method.
/// Assert:  the consumer's `statements` and `identities` are the built values'
///   borrows by `core::ptr::eq`, its `pairing` is the received borrow by
///   `core::ptr::eq`, its `hash_to_scalar` and `random` are the received
///   adapters by `core::ptr::addr_eq`, its `statement_version` equals `2`, its
///   `iterations.get()` equals `7`, and its `chain_forms`, `kem`,
///   `key_agreement`, `delivery_proof`, `generate`, `kinds`, and `verifier`
///   equal the builder's defaults.
#[test]
fn try_new_holds_every_value_it_is_handed() {
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
            let consumer: EncodingConsumer<'_, P> = build_encoding_consumer(
                payload.pairing,
                payload.hash_to_scalar,
                payload.random,
                &admitted_statements,
                &admitted_identities,
                EncodingConsumerConstructorParamsOverrides {
                    statement_version: Some(2),
                    iterations: NonZeroU32::new(7),
                    ..Default::default()
                },
            );
            assert!(core::ptr::eq(consumer.statements, &admitted_statements));
            assert!(core::ptr::eq(consumer.identities, &admitted_identities));
            assert!(core::ptr::eq(consumer.pairing, payload.pairing));
            assert!(core::ptr::addr_eq(
                consumer.hash_to_scalar,
                payload.hash_to_scalar
            ));
            assert!(core::ptr::addr_eq(consumer.random, payload.random));
            assert_eq!(consumer.statement_version, 2);
            assert_eq!(consumer.iterations.get(), 7);
            let chain_forms_params = build_create_chain_forms_params(Default::default());
            assert_eq!(
                consumer.chain_forms,
                AdmittedChainForms {
                    concrete: chain_forms_params.concrete,
                    identifier: chain_forms_params.identifier,
                },
            );
            let kem_params = build_create_kem_params(Default::default());
            assert_eq!(
                consumer.kem,
                AdmittedKem {
                    concrete: kem_params.concrete,
                    identifier: kem_params.identifier,
                    scope: kem_params.scope,
                },
            );
            let key_agreement_params = build_create_key_agreement_params(Default::default());
            assert_eq!(
                consumer.key_agreement,
                AdmittedKeyAgreement {
                    concrete: key_agreement_params.concrete,
                    identifier: key_agreement_params.identifier,
                    algebra: key_agreement_params.algebra,
                },
            );
            assert_eq!(
                consumer.delivery_proof,
                build_create_delivery_proof_params(Default::default()).concrete,
            );
            assert_eq!(
                consumer.generate,
                build_create_generate_params(Default::default()).concrete,
            );
            assert_eq!(
                consumer.kinds,
                build_create_generate_params(Default::default()).kinds,
            );
            assert_eq!(
                consumer.verifier,
                VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure),
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
    assert!(output.is_ok());
    // try_new_holds_every_value_it_is_handed
}

/// Contract: Selection → the forms function is called with the held
///   selection's `concrete` and `identifier`, so a selection the function
///   admits over the encoder it is handed constructs.
/// Arrange: the consumer constructed with `chain_forms` an `AdmittedChainForms`
///   holding the `concrete` and `identifier` of the member of
///   `DECLARED_CHAIN_FORMS_SELECTIONS`, the Ethereum ABI selection of
///   `DECLARED_ENCODING_SELECTIONS` passed to `create_encoding`, `generate` and
///   `kinds` the member of `GENERATE_CONCRETES` and its declared kind
///   configurations, since the mock generate concrete's declaration requires
///   `EncodingIdentifier::Mock` and would refuse the ABI encoder before `emit`,
///   and the default mock concretes elsewhere.
/// Act:     `create_encoding`, once, inside the consumer.
/// Assert:  the output matches `Ok(CreateEncodingSuccessReturn { output:
///   Ok(DeliveryProofConsumerSuccessReturn { .. }) })`, which a consumer that
///   ignored the held selection and called the forms function with the mock
///   selection would not return over the Ethereum ABI encoder.
#[test]
fn consume_encoding_passes_the_held_chain_forms_selection_to_the_forms_function() {
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
            let consumer: EncodingConsumer<'_, P> = build_encoding_consumer(
                payload.pairing,
                payload.hash_to_scalar,
                payload.random,
                &admitted_statements,
                &admitted_identities,
                EncodingConsumerConstructorParamsOverrides {
                    chain_forms: Some(AdmittedChainForms {
                        concrete: DECLARED_CHAIN_FORMS_SELECTIONS[0].concrete,
                        identifier: DECLARED_CHAIN_FORMS_SELECTIONS[0].identifier,
                    }),
                    generate: Some(build_generate_concrete(
                        GenerateConcreteKind::Solidity,
                        GenerateConcreteOverrides,
                    )),
                    kinds: Some(build_generate_kind_configurations(
                        GenerateConcreteKind::Solidity,
                        GenerateKindConfigurationsOverrides,
                    )),
                    ..Default::default()
                },
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
        Ok(CreateEncodingSuccessReturn { output: Ok(_) })
    ));
    // consume_encoding_passes_the_held_chain_forms_selection_to_the_forms_function
}
