#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::path::PathBuf;

use chain::IChainForms;
use core::num::NonZeroU32;
use encoding::{IDecoderAdapter, IEncoderAdapter};
use envelope::IKeyAgreementAdapter;
use hash_to_scalar::IHashToScalarAdapter;
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::{ICredentialKemAdapter, KemIdentity, SetupScope};
use pairing::{IPairingArithmetic, IPairingReference};
use proof::{
    CreateDeliveryProofReturn, DELIVERY_PURPOSE_MINT, DELIVERY_PURPOSE_TRANSFER,
    DeliveryContextOverrides, IDeliveryProofAdapter, MintPurpose, TransferPurpose,
    build_create_delivery_proof_deps, build_create_delivery_proof_params,
    build_create_delivery_proof_payload, build_delivery_context, create_delivery_proof,
};
use random::IRandomSourceAdapter;

use super::interface::{DeliveryProofConsumer, DeliveryProofConsumerReturn};
use super::mock::{DeliveryProofConsumerConstructorParamsOverrides, build_delivery_proof_consumer};
use crate::delivery::provides::AdmittedKemIdentity;
use crate::generate::provides::{
    EmittedFileOverrides, ICreateGenerateCaseConsumer, VectorKind, build_create_generate_case_deps,
    build_create_generate_case_params, build_create_generate_case_payload, build_emitted_file,
    run_create_generate_case,
};
use crate::verifier::provides::{
    L1DataFee, MockIVerifierAdapterFailureMode, VerifierConcrete, VerifyCost,
};

/// Contract: Measured → the files the mock generate concrete returns and the
///   measurement the verifier step returns are the output whole, the files in
///   write order and the measurement's iteration count the consumer's and its
///   costs the verifier's accepted cost.
/// Arrange: the default params, which select the mock generate concrete and
///   the mock verifier concrete under `NoFailure`; `iterations` of `3`, a
///   count other than the builder's default; the expected files the five kinds
///   the mock concrete's declaration admits with each `path` the kind's name
///   as text, and the expected cost the literal `VerifyCost { l2_execution_gas:
///   100_000, l1_data_fee: L1DataFee::Charged { wei: 1_000 } }`, as
///   independent literals.
/// Act:     `create_delivery_proof`, once, inside the consumer.
/// Assert:  the output matches `Ok(CreateDeliveryProofSuccessReturn { output:
///   Ok(DeliveryProofConsumerSuccessReturn { files, measurement }) })`,
///   `files` equals the expected files in that order by `assert_eq!`,
///   `measurement.iterations.get()` equals `3`, and `measurement.costs.mint`
///   and `measurement.costs.transfer` each equal the expected cost.
#[test]
fn consume_delivery_proof_returns_the_files_and_the_measurement_both_steps_computed() {
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
        type Output = CreateDeliveryProofReturn<DeliveryProofConsumerReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            kem: &K,
            key_agreement: &A,
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
            let consumer: DeliveryProofConsumer<'_, P, E, K, A, F> = build_delivery_proof_consumer(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                kem,
                key_agreement,
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default()),
                build_delivery_context::<F, TransferPurpose>(
                    DELIVERY_PURPOSE_TRANSFER,
                    Default::default(),
                ),
                identities.clone(),
                identities[0],
                DeliveryProofConsumerConstructorParamsOverrides {
                    iterations: NonZeroU32::new(3),
                    ..Default::default()
                },
            );
            create_delivery_proof(
                &build_create_delivery_proof_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
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
    let output = result
        .expect("the case composes")
        .output
        .expect("the factory admits the mock concrete and runs the consumer")
        .output
        .expect("the consumer emits and measures");
    assert_eq!(output.files, expected_files);
    assert_eq!(output.measurement.iterations.get(), 3);
    assert_eq!(output.measurement.costs.mint, expected_cost);
    assert_eq!(output.measurement.costs.transfer, expected_cost);
    // consume_delivery_proof_returns_the_files_and_the_measurement_both_steps_computed
}

/// Contract: Construction → the consumer holds the adapters, the mint context,
///   the transfer context, the identities, the proof identity, and the
///   configured values it is constructed with.
/// Arrange: a mint context built with `expiry` `Some(11)` and
///   `MintPurpose::Replacement`, a transfer context built with `expiry`
///   `Some(12)` and `TransferPurpose::Grant`, `statement_version` of `2`,
///   `iterations` of `7`, `verifier` of `VerifierConcrete::Mock(
///   MockIVerifierAdapterFailureMode::RpcRefused)`, a value other than the
///   builder's default, standing for the verifier concrete of the one curve
///   the consumer composes, and the received adapters and identities.
/// Act:     `build_delivery_proof_consumer`, once, inside the case consumer's
///   method.
/// Assert:  the consumer's `verifier` equals `VerifierConcrete::Mock(
///   MockIVerifierAdapterFailureMode::RpcRefused)`, its `mint.expiry` equals
///   `11` and `mint.purpose.code()` equals `4`, its `transfer.expiry` equals
///   `12` and `transfer.purpose.code()` equals `3`, its `statement_version`
///   equals `2`, its `iterations.get()` equals `7`, its `pairing`, `encoder`,
///   `kem`, and `key_agreement` are the received borrows by `core::ptr::eq`, its
///   `hash_to_scalar` and `random` are the received adapters by
///   `core::ptr::addr_eq`, and its `identities` and `proof_identity` hold the
///   received values, the identities compared in length and each element's
///   `setup_scope` and `identity` by the kinds the received scope reports.
#[test]
fn try_new_holds_every_value_it_is_handed() {
    // Arrange
    struct CaseConsumer;

    impl ICreateGenerateCaseConsumer for CaseConsumer {
        type Output = CreateDeliveryProofReturn<DeliveryProofConsumerReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            kem: &K,
            key_agreement: &A,
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
            let consumer: DeliveryProofConsumer<'_, P, E, K, A, F> = build_delivery_proof_consumer(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                kem,
                key_agreement,
                build_delivery_context::<F, MintPurpose>(
                    MintPurpose::Replacement,
                    DeliveryContextOverrides {
                        expiry: Some(11),
                        ..Default::default()
                    },
                ),
                build_delivery_context::<F, TransferPurpose>(
                    TransferPurpose::Grant,
                    DeliveryContextOverrides {
                        expiry: Some(12),
                        ..Default::default()
                    },
                ),
                identities.clone(),
                identities[0],
                DeliveryProofConsumerConstructorParamsOverrides {
                    verifier: Some(VerifierConcrete::Mock(
                        MockIVerifierAdapterFailureMode::RpcRefused,
                    )),
                    statement_version: Some(2),
                    iterations: NonZeroU32::new(7),
                    ..Default::default()
                },
            );
            assert_eq!(
                consumer.verifier,
                VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::RpcRefused)
            );
            assert_eq!(consumer.mint.expiry, 11);
            assert_eq!(consumer.mint.purpose.code(), 4);
            assert_eq!(consumer.transfer.expiry, 12);
            assert_eq!(consumer.transfer.purpose.code(), 3);
            assert_eq!(consumer.statement_version, 2);
            assert_eq!(consumer.iterations.get(), 7);
            assert!(core::ptr::eq(consumer.pairing, pairing));
            assert!(core::ptr::eq(consumer.encoder, encoder));
            assert!(core::ptr::eq(consumer.kem, kem));
            assert!(core::ptr::eq(consumer.key_agreement, key_agreement));
            assert!(core::ptr::addr_eq(consumer.hash_to_scalar, hash_to_scalar));
            assert!(core::ptr::addr_eq(consumer.random, random));
            assert_eq!(consumer.identities.len(), identities.len());
            for (held, expected) in consumer
                .identities
                .iter()
                .chain(core::iter::once(&consumer.proof_identity))
                .zip(identities.iter().chain(core::iter::once(&identities[0])))
            {
                match (held.setup_scope, expected.setup_scope) {
                    (SetupScope::Entitlement, SetupScope::Entitlement) => {}
                    (
                        SetupScope::Asset { root: held_root },
                        SetupScope::Asset {
                            root: expected_root,
                        },
                    ) => assert_eq!(held_root, expected_root),
                    _ => panic!("a held setup scope is the received scope's kind"),
                }
                match (held.identity, expected.identity) {
                    (
                        KemIdentity::Entitlement {
                            canonical: held_canonical,
                        },
                        KemIdentity::Entitlement {
                            canonical: expected_canonical,
                        },
                    ) => assert_eq!(held_canonical, expected_canonical),
                    (
                        KemIdentity::Asset { root: held_root },
                        KemIdentity::Asset {
                            root: expected_root,
                        },
                    ) => assert_eq!(held_root, expected_root),
                    _ => panic!("a held identity is the received identity's kind"),
                }
            }
            create_delivery_proof(
                &build_create_delivery_proof_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
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
    assert!(output.is_ok());
    // try_new_holds_every_value_it_is_handed
}
