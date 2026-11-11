#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::path::PathBuf;

use chain::{DECLARED_CHAIN_FORMS_SELECTIONS, build_create_chain_forms_params};
use core::num::NonZeroU32;
use encoding::{DECLARED_ENCODING_SELECTIONS, build_create_encoding_params};
use envelope::build_create_key_agreement_params;
use hash_to_scalar::{DECLARED_HASH_TO_SCALAR_SELECTIONS, build_create_hash_to_scalar_params};
use kem::build_create_kem_params;
use pairing::{
    CreatePairingPayload, CreatePairingSuccessReturn, build_create_pairing_deps,
    build_create_pairing_params, create_pairing,
};
use proof::build_create_delivery_proof_params;
use random::{
    build_create_random_source_deps, build_create_random_source_params,
    build_create_random_source_payload, create_random_source,
};

use super::interface::PairingConsumer;
use super::mock::{PairingConsumerConstructorParamsOverrides, build_pairing_consumer};
use crate::config::provides::{
    AdmittedChainForms, AdmittedEncoding, AdmittedHashToScalar, AdmittedKem, AdmittedKeyAgreement,
    build_admitted_identities, build_admitted_statements,
};
use crate::generate::provides::{
    EmittedFileOverrides, GenerateConcreteKind, GenerateConcreteOverrides,
    GenerateKindConfigurationsOverrides, VectorKind, build_create_generate_params,
    build_emitted_file, build_generate_concrete, build_generate_kind_configurations,
};
use crate::run::provides::DeliveryProofConsumerSuccessReturn;
use crate::verifier::provides::{
    L1DataFee, MockIVerifierAdapterFailureMode, VerifierConcrete, VerifyCost,
};

/// Contract: Consumed → the files the mock generate concrete returns and the
///   measurement the verifier step returns are the output whole, the files in
///   write order and the measurement's iteration count the consumer's and its
///   costs the verifier's accepted cost.
/// Arrange: the default params, which select the pairing family's mock
///   concrete and the mock hash-to-scalar, encoding, forms, KEM, key
///   agreement, delivery proof, generate, and verifier concretes under
///   `NoFailure`; `iterations` of `3`, a count other than the builder's
///   default; the expected files built by `build_emitted_file` with `kind`
///   `Some(VectorKind::G1Addition)`,
///   `Some(VectorKind::G1MultiScalarMultiplication)`,
///   `Some(VectorKind::HashToScalar)`, `Some(VectorKind::DecodeRejection)`,
///   and `Some(VectorKind::IdentityMapping)` and `path` the kind's name as
///   text, and the expected cost the literal `VerifyCost { l2_execution_gas:
///   100_000, l1_data_fee: L1DataFee::Charged { wei: 1_000 } }`, as
///   independent literals.
/// Act:     `create_pairing`, once.
/// Assert:  the result matches `Ok(CreatePairingSuccessReturn { output:
///   Ok(DeliveryProofConsumerSuccessReturn { files, measurement }) })`,
///   `files` equals the expected files in that order by `assert_eq!`,
///   `measurement.iterations.get()` equals `3`, and `measurement.costs.mint`
///   and `measurement.costs.transfer` each equal the expected cost.
#[test]
fn consume_pairing_returns_the_files_and_the_measurement_both_steps_computed() {
    // Arrange
    let random_source = create_random_source(
        &build_create_random_source_deps(Default::default()),
        build_create_random_source_params(Default::default()),
        build_create_random_source_payload(Default::default()),
    )
    .expect("the mock selection constructs");
    let admitted_statements = build_admitted_statements(Default::default());
    let admitted_identities = build_admitted_identities(Default::default());
    let consumer: PairingConsumer<'_> = build_pairing_consumer(
        &*random_source.adapter,
        &admitted_statements,
        &admitted_identities,
        PairingConsumerConstructorParamsOverrides {
            iterations: NonZeroU32::new(3),
            ..Default::default()
        },
    );
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

    // Act
    let result = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    let output = result
        .expect("the factory admits the mock concrete and runs the consumer")
        .output
        .expect("the consumer calls the hash-to-scalar and encoding factories and the chain beneath them");
    assert_eq!(output.files, expected_files);
    assert_eq!(output.measurement.iterations.get(), 3);
    assert_eq!(output.measurement.costs.mint, expected_cost);
    assert_eq!(output.measurement.costs.transfer, expected_cost);
    // consume_pairing_returns_the_files_and_the_measurement_both_steps_computed
}

/// Contract: Construction → the consumer holds the randomness source, the
///   admitted statements, the admitted identities, and the configured values
///   it is constructed with.
/// Arrange: `AdmittedStatements` and `AdmittedIdentities` built with
///   `build_admitted_statements` and `build_admitted_identities` at their
///   defaults, a randomness source from `create_random_source`,
///   `statement_version` of `2`, and `iterations` of `7`.
/// Act:     `build_pairing_consumer`, once.
/// Assert:  the consumer's `statements` and `identities` are the built
///   values' borrows by `core::ptr::eq`, its `random` is the received borrow
///   by `core::ptr::addr_eq`, its `statement_version` equals `2`, its
///   `iterations.get()` equals `7`, and its `hash_to_scalar`, `encoding`,
///   `chain_forms`, `kem`, `key_agreement`, `delivery_proof`, `generate`,
///   `kinds`, and `verifier` equal the builder's defaults.
#[test]
fn try_new_holds_every_value_it_is_handed() {
    // Arrange
    let random_source = create_random_source(
        &build_create_random_source_deps(Default::default()),
        build_create_random_source_params(Default::default()),
        build_create_random_source_payload(Default::default()),
    )
    .expect("the mock selection constructs");
    let random = &*random_source.adapter;
    let admitted_statements = build_admitted_statements(Default::default());
    let admitted_identities = build_admitted_identities(Default::default());

    // Act
    let consumer: PairingConsumer<'_> = build_pairing_consumer(
        random,
        &admitted_statements,
        &admitted_identities,
        PairingConsumerConstructorParamsOverrides {
            statement_version: Some(2),
            iterations: NonZeroU32::new(7),
            ..Default::default()
        },
    );

    // Assert
    assert!(core::ptr::eq(consumer.statements, &admitted_statements));
    assert!(core::ptr::eq(consumer.identities, &admitted_identities));
    assert!(core::ptr::addr_eq(consumer.random, random));
    assert_eq!(consumer.statement_version, 2);
    assert_eq!(consumer.iterations.get(), 7);
    let hash_to_scalar_params = build_create_hash_to_scalar_params(Default::default());
    assert_eq!(
        consumer.hash_to_scalar,
        AdmittedHashToScalar {
            concrete: hash_to_scalar_params.concrete,
            identifier: hash_to_scalar_params.identifier,
        },
    );
    let encoding_params = build_create_encoding_params(Default::default());
    assert_eq!(
        consumer.encoding,
        AdmittedEncoding {
            concrete: encoding_params.concrete,
            identifier: encoding_params.identifier,
        },
    );
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
    // try_new_holds_every_value_it_is_handed
}

/// Contract: Selection → the hash-to-scalar factory is called with the held
///   selection's `concrete` and `identifier`, so a selection the factory
///   admits constructs.
/// Arrange: the consumer constructed with `hash_to_scalar` an
///   `AdmittedHashToScalar` holding the `concrete` and `identifier` of the
///   member of `DECLARED_HASH_TO_SCALAR_SELECTIONS`, and the default mock
///   concretes elsewhere.
/// Act:     `create_pairing`, once.
/// Assert:  the result matches `Ok(CreatePairingSuccessReturn { output:
///   Ok(DeliveryProofConsumerSuccessReturn { .. }) })`; the block is paired
///   with the refusal block of `test_error.rs`, which a consumer that ignored
///   the held selection does not satisfy.
#[test]
fn consume_pairing_passes_the_held_hash_to_scalar_selection_to_the_factory() {
    // Arrange
    let random_source = create_random_source(
        &build_create_random_source_deps(Default::default()),
        build_create_random_source_params(Default::default()),
        build_create_random_source_payload(Default::default()),
    )
    .expect("the mock selection constructs");
    let admitted_statements = build_admitted_statements(Default::default());
    let admitted_identities = build_admitted_identities(Default::default());
    let consumer: PairingConsumer<'_> = build_pairing_consumer(
        &*random_source.adapter,
        &admitted_statements,
        &admitted_identities,
        PairingConsumerConstructorParamsOverrides {
            hash_to_scalar: Some(AdmittedHashToScalar {
                concrete: DECLARED_HASH_TO_SCALAR_SELECTIONS[0].concrete,
                identifier: DECLARED_HASH_TO_SCALAR_SELECTIONS[0].identifier,
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
            output: Ok(DeliveryProofConsumerSuccessReturn { .. })
        })
    ));
    // consume_pairing_passes_the_held_hash_to_scalar_selection_to_the_factory
}

/// Contract: Selection → the encoding factory is called with the held
///   selection's `concrete` and `identifier`, so a selection the factory
///   admits over the forms concrete that requires its identifier constructs.
/// Arrange: the consumer constructed with `encoding` an `AdmittedEncoding`
///   holding the `concrete` and `identifier` of the member of
///   `DECLARED_ENCODING_SELECTIONS` and `chain_forms` an `AdmittedChainForms`
///   holding the `concrete` and `identifier` of the member of
///   `DECLARED_CHAIN_FORMS_SELECTIONS`, `generate` and `kinds` the member of
///   `GENERATE_CONCRETES` and its declared kind configurations, since the
///   mock generate concrete's declaration requires `EncodingIdentifier::Mock`
///   and would refuse the ABI encoder before `emit`, and the default mock
///   concretes elsewhere.
/// Act:     `create_pairing`, once.
/// Assert:  the result matches `Ok(CreatePairingSuccessReturn { output:
///   Ok(DeliveryProofConsumerSuccessReturn { .. }) })`, which a consumer that
///   ignored the held encoding selection and called the factory with the mock
///   selection would not return over the Ethereum ABI forms concrete, whose
///   required identifier the mock encoder does not declare.
#[test]
fn consume_pairing_passes_the_held_encoding_selection_to_the_factory() {
    // Arrange
    let random_source = create_random_source(
        &build_create_random_source_deps(Default::default()),
        build_create_random_source_params(Default::default()),
        build_create_random_source_payload(Default::default()),
    )
    .expect("the mock selection constructs");
    let admitted_statements = build_admitted_statements(Default::default());
    let admitted_identities = build_admitted_identities(Default::default());
    let consumer: PairingConsumer<'_> = build_pairing_consumer(
        &*random_source.adapter,
        &admitted_statements,
        &admitted_identities,
        PairingConsumerConstructorParamsOverrides {
            encoding: Some(AdmittedEncoding {
                concrete: DECLARED_ENCODING_SELECTIONS[0].concrete,
                identifier: DECLARED_ENCODING_SELECTIONS[0].identifier,
            }),
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
            output: Ok(DeliveryProofConsumerSuccessReturn { .. })
        })
    ));
    // consume_pairing_passes_the_held_encoding_selection_to_the_factory
}
