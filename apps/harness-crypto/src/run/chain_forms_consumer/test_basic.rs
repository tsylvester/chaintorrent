#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::path::PathBuf;

use chain::{
    ConsumeCaseParams, ConsumeCasePayload, ConsumeChainFormsParams, ConsumeChainFormsPayload,
    CreateChainFormsReturn, IChainForms, IChainFormsConsumer, ICreateChainFormsCaseConsumer,
    build_create_chain_forms_case_deps, build_create_chain_forms_case_params,
    build_create_chain_forms_case_payload, build_create_chain_forms_deps,
    build_create_chain_forms_params, build_create_chain_forms_payload, create_chain_forms,
    create_chain_forms_case,
};
use core::num::NonZeroU32;
use encoding::{IDecoderAdapter, IEncoderAdapter};
use envelope::{IKeyAgreementAdapter, build_create_key_agreement_params};
use hash_to_scalar::IHashToScalarAdapter;
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::{ICredentialKemAdapter, build_create_kem_params};
use pairing::{IPairingArithmetic, IPairingReference};
use proof::{
    IDeliveryProofAdapter, MintPurpose, TransferPurpose, build_create_delivery_proof_params,
};
use random::IRandomSourceAdapter;

use super::admit_statement_contexts;
use super::interface::{ChainFormsConsumer, ChainFormsConsumerReturn};
use super::mock::{ChainFormsConsumerConstructorParamsOverrides, build_chain_forms_consumer};
use crate::config::provides::{
    AdmittedKem, AdmittedKeyAgreement, AdmittedStatements, AdmittedStatementsOverrides,
    CanonicalDeliveryContextOverrides, build_admitted_identities, build_admitted_statements,
    build_canonical_delivery_context,
};
use crate::delivery::provides::AdmittedKemIdentity;
use crate::generate::provides::{
    EmittedFileOverrides, ICreateGenerateCaseConsumer, VectorKind, build_create_generate_case_deps,
    build_create_generate_case_params, build_create_generate_case_payload,
    build_create_generate_params, build_emitted_file, run_create_generate_case,
};
use crate::verifier::provides::{
    L1DataFee, MockIVerifierAdapterFailureMode, VerifierConcrete, VerifyCost,
};

/// Contract: Consumed → the files the mock generate concrete returns and the
///   measurement the verifier step returns are the output whole, the files in
///   write order and the measurement's iteration count the consumer's and its
///   costs the verifier's accepted cost.
/// Arrange: the default params, which select the mock KEM, key agreement,
///   delivery proof, generate, and verifier concretes under `NoFailure`;
///   `iterations` of `3`, a count other than the builder's default; the
///   expected files the five kinds the mock concrete's declaration admits with
///   each `path` the kind's name as text, and the expected cost the literal
///   `VerifyCost { l2_execution_gas: 100_000, l1_data_fee:
///   L1DataFee::Charged { wei: 1_000 } }`, as independent literals.
/// Act:     `create_chain_forms`, once, inside the consumer.
/// Assert:  the output matches `Ok(CreateChainFormsSuccessReturn { output:
///   Ok(DeliveryProofConsumerSuccessReturn { files, measurement }) })`,
///   `files` equals the expected files in that order by `assert_eq!`,
///   `measurement.iterations.get()` equals `3`, and `measurement.costs.mint`
///   and `measurement.costs.transfer` each equal the expected cost.
#[test]
fn consume_chain_forms_returns_the_files_and_the_measurement_both_steps_computed() {
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
        type Output = CreateChainFormsReturn<ChainFormsConsumerReturn>;

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
            let admitted_statements = build_admitted_statements(Default::default());
            let admitted_identities = build_admitted_identities(Default::default());
            let consumer: ChainFormsConsumer<'_, P, E> = build_chain_forms_consumer(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                &admitted_statements,
                &admitted_identities,
                ChainFormsConsumerConstructorParamsOverrides {
                    iterations: NonZeroU32::new(3),
                    ..Default::default()
                },
            );
            create_chain_forms(
                &build_create_chain_forms_deps(consumer, encoder, Default::default()),
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
    let output = result
        .expect("the case composes")
        .output
        .expect("the factory admits the mock concrete and runs the consumer")
        .output
        .expect("the consumer admits the contexts and runs the KEM family");
    assert_eq!(output.files, expected_files);
    assert_eq!(output.measurement.iterations.get(), 3);
    assert_eq!(output.measurement.costs.mint, expected_cost);
    assert_eq!(output.measurement.costs.transfer, expected_cost);
    // consume_chain_forms_returns_the_files_and_the_measurement_both_steps_computed
}

/// Contract: Construction → the consumer holds the adapters, the admitted
///   statements, the admitted identities, and the configured values it is
///   constructed with.
/// Arrange: `AdmittedStatements` and `AdmittedIdentities` built with
///   `build_admitted_statements` and `build_admitted_identities` at their
///   defaults, `statement_version` of `2`, `iterations` of `7`, and the
///   received adapters.
/// Act:     `build_chain_forms_consumer`, once, inside the case consumer's
///   method.
/// Assert:  the consumer's `statements` and `identities` are the built values'
///   borrows by `core::ptr::eq`, its `pairing` and `encoder` are the received
///   borrows by `core::ptr::eq`, its `hash_to_scalar` and `random` are the
///   received adapters by `core::ptr::addr_eq`, its `statement_version` equals
///   `2`, its `iterations.get()` equals `7`, and its `kem`, `key_agreement`,
///   `delivery_proof`, `generate`, `kinds`, and `verifier` equal the builder's
///   defaults.
#[test]
fn try_new_holds_every_value_it_is_handed() {
    // Arrange
    struct CaseConsumer;

    impl ICreateGenerateCaseConsumer for CaseConsumer {
        type Output = CreateChainFormsReturn<ChainFormsConsumerReturn>;

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
            let admitted_statements = build_admitted_statements(Default::default());
            let admitted_identities = build_admitted_identities(Default::default());
            let consumer: ChainFormsConsumer<'_, P, E> = build_chain_forms_consumer(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                &admitted_statements,
                &admitted_identities,
                ChainFormsConsumerConstructorParamsOverrides {
                    statement_version: Some(2),
                    iterations: NonZeroU32::new(7),
                    ..Default::default()
                },
            );
            assert!(core::ptr::eq(consumer.statements, &admitted_statements));
            assert!(core::ptr::eq(consumer.identities, &admitted_identities));
            assert!(core::ptr::eq(consumer.pairing, pairing));
            assert!(core::ptr::eq(consumer.encoder, encoder));
            assert!(core::ptr::addr_eq(consumer.hash_to_scalar, hash_to_scalar));
            assert!(core::ptr::addr_eq(consumer.random, random));
            assert_eq!(consumer.statement_version, 2);
            assert_eq!(consumer.iterations.get(), 7);
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
            create_chain_forms(
                &build_create_chain_forms_deps(consumer, encoder, Default::default()),
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
    assert!(output.is_ok());
    // try_new_holds_every_value_it_is_handed
}

/// Contract: Admission → the mint context is admitted from the statements'
///   `mint` with `mint_purpose` and the transfer context from `transfer` with
///   `transfer_purpose`, each in the forms handed.
/// Arrange: `AdmittedStatements` built with `build_admitted_statements` whose
///   `mint` is `build_canonical_delivery_context` overriding `expiry` with `11`
///   and whose `transfer` is the same builder overriding `expiry` with `12`,
///   `mint_purpose` `MintPurpose::Replacement`, and `transfer_purpose`
///   `TransferPurpose::Grant`, as independent literals; the helper
///   `admit_statement_contexts` reached by the unit-visible helper the module
///   exposes to its tests, called inside a local `IChainFormsConsumer` whose
///   `consume_chain_forms<F: IChainForms>` makes the block's single call and
///   asserts in place, reached through `create_chain_forms_case` at its
///   defaults, which select the family's mock forms over the mock encoding
///   concrete.
/// Act:     `admit_statement_contexts` over the arranged statements, once.
/// Assert:  the result is `Ok`; the mint context's `expiry` equals `11` and its
///   `purpose.code()` equals `4`; the transfer context's `expiry` equals `12`
///   and its `purpose.code()` equals `3`.
#[test]
fn admit_statement_contexts_admits_the_mint_and_the_transfer_context_each_with_its_own_value_and_purpose()
 {
    // Arrange
    let statements = build_admitted_statements(AdmittedStatementsOverrides {
        mint: Some(build_canonical_delivery_context(
            CanonicalDeliveryContextOverrides {
                expiry: Some(11),
                ..Default::default()
            },
        )),
        mint_purpose: Some(MintPurpose::Replacement),
        transfer: Some(build_canonical_delivery_context(
            CanonicalDeliveryContextOverrides {
                expiry: Some(12),
                ..Default::default()
            },
        )),
        transfer_purpose: Some(TransferPurpose::Grant),
    });

    struct FormsConsumer<'a> {
        statements: &'a AdmittedStatements,
    }

    impl IChainFormsConsumer for FormsConsumer<'_> {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            let (mint, transfer) =
                admit_statement_contexts::<F>(self.statements).expect("the contexts admit");
            assert_eq!(mint.expiry, 11);
            assert_eq!(mint.purpose.code(), 4);
            assert_eq!(transfer.expiry, 12);
            assert_eq!(transfer.purpose.code(), 3);
        }
    }

    struct CaseConsumer {
        statements: AdmittedStatements,
    }

    impl ICreateChainFormsCaseConsumer for CaseConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_case<E: IEncoderAdapter>(
            &self,
            _params: ConsumeCaseParams,
            payload: ConsumeCasePayload<'_, E>,
        ) -> Self::Output {
            create_chain_forms(
                &build_create_chain_forms_deps(
                    FormsConsumer {
                        statements: &self.statements,
                    },
                    payload.encoder,
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let deps = build_create_chain_forms_case_deps(CaseConsumer { statements }, Default::default());

    // Act
    let result = create_chain_forms_case(
        &deps,
        build_create_chain_forms_case_params(Default::default()),
        build_create_chain_forms_case_payload(Default::default()),
    );

    // Assert
    result
        .expect("the case composes")
        .output
        .expect("the factory admits the mock forms and runs the consumer");
    // admit_statement_contexts_admits_the_mint_and_the_transfer_context_each_with_its_own_value_and_purpose
}
