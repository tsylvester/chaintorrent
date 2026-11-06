#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use chain::IChainForms;
use core::num::NonZeroU32;
use encoding::{IDecoderAdapter, IEncoderAdapter};
use envelope::IKeyAgreementAdapter;
use hash_to_scalar::IHashToScalarAdapter;
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::{ICredentialKemAdapter, KemIdentity, SetupScope};
use pairing::{IPairingArithmetic, IPairingReference};
use proof::{
    DELIVERY_PURPOSE_MINT, DELIVERY_PURPOSE_TRANSFER, DeliveryContextOverrides,
    IDeliveryProofAdapter, MintPurpose, TransferPurpose, build_delivery_context,
};
use random::IRandomSourceAdapter;

use super::interface::{VerifierConsumer, VerifierConsumerReturn};
use super::mock::{VerifierConsumerConstructorParamsOverrides, build_verifier_consumer};
use crate::delivery::provides::AdmittedKemIdentity;
use crate::measure::provides::{
    IMeasureCaseConsumer, build_measure_case_deps, build_measure_case_params,
    build_measure_case_payload, run_measure_case,
};
use crate::verifier::provides::{
    CreateVerifierReturn, IVerifierAdapter, L1DataFee, VerifyCost, build_create_verifier_deps,
    build_create_verifier_params, build_create_verifier_payload, create_verifier,
};

/// Contract: Measured → the success `measure` returns is the output whole, its
///   iteration count the consumer's and its costs the verifier's accepted cost.
/// Arrange: the default params, which select the family's mock concrete under
///   `NoFailure`; `iterations` of `3`, a count other than the builder's
///   default; the expected cost the literal `VerifyCost { l2_execution_gas:
///   100_000, l1_data_fee: L1DataFee::Charged { wei: 1_000 } }`, as an
///   independent literal.
/// Act:     `create_verifier`, once, inside the consumer.
/// Assert:  the output matches `Ok(CreateVerifierSuccessReturn { output:
///   Ok(MeasureSuccessReturn { measurement }) })`,
///   `measurement.iterations.get()` equals `3`, and `measurement.costs.mint`
///   and `measurement.costs.transfer` each equal the expected cost.
#[test]
fn consume_verifier_returns_the_measurement_measure_computes() {
    // Arrange
    let expected_cost = VerifyCost {
        l2_execution_gas: 100_000,
        l1_data_fee: L1DataFee::Charged { wei: 1_000 },
    };

    struct CaseConsumer;

    impl IMeasureCaseConsumer for CaseConsumer {
        type Output = CreateVerifierReturn<VerifierConsumerReturn>;

        fn consume_measure_case<P, H, E, F, K, A, D, V>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            _verifier: &V,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
            V: IVerifierAdapter<DeliveryProof = D>,
        {
            let consumer: VerifierConsumer<'_, P, K, A, D> = build_verifier_consumer(
                pairing,
                kem,
                key_agreement,
                delivery_proof,
                random,
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default()),
                build_delivery_context::<F, TransferPurpose>(
                    DELIVERY_PURPOSE_TRANSFER,
                    Default::default(),
                ),
                identity,
                VerifierConsumerConstructorParamsOverrides {
                    iterations: NonZeroU32::new(3),
                },
            );
            create_verifier(
                &build_create_verifier_deps(pairing, delivery_proof, consumer, Default::default()),
                build_create_verifier_params(Default::default()),
                build_create_verifier_payload(Default::default()),
            )
        }
    }

    let canonical: &[u8] = b"pkg@1.0.0#1";
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    let deps = build_measure_case_deps(CaseConsumer, Default::default());

    // Act
    let result = run_measure_case(
        &deps,
        build_measure_case_params(Default::default()),
        build_measure_case_payload(canonical, &root, Default::default()),
    );

    // Assert
    let output = result
        .expect("the case composes")
        .output
        .expect("the factory admits the mock concrete and runs the consumer")
        .output
        .expect("measure admits the case");
    assert_eq!(output.measurement.iterations.get(), 3);
    assert_eq!(output.measurement.costs.mint, expected_cost);
    assert_eq!(output.measurement.costs.transfer, expected_cost);
    // consume_verifier_returns_the_measurement_measure_computes
}

/// Contract: Construction → the consumer holds the adapters, the mint context,
///   the transfer context, the identity, and the iteration count it is
///   constructed with.
/// Arrange: a mint context built with `expiry` `Some(11)` and a transfer
///   context built with `expiry` `Some(12)`, `iterations` of `7`, and the
///   received adapters and identity.
/// Act:     `build_verifier_consumer`, once, inside the case consumer's method.
/// Assert:  the consumer's `mint.expiry` equals `11`, its `transfer.expiry`
///   equals `12`, its `iterations.get()` equals `7`, its `pairing`, `kem`,
///   `key_agreement`, and `delivery_proof` are the received borrows by
///   `core::ptr::eq`, its `random` is the received source by
///   `core::ptr::addr_eq`, and its `identity` holds the received setup scope
///   and identity compared by the kinds the received scope reports.
#[test]
fn try_new_holds_every_value_it_is_handed() {
    // Arrange
    struct CaseConsumer;

    impl IMeasureCaseConsumer for CaseConsumer {
        type Output = CreateVerifierReturn<VerifierConsumerReturn>;

        fn consume_measure_case<P, H, E, F, K, A, D, V>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            _verifier: &V,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
            V: IVerifierAdapter<DeliveryProof = D>,
        {
            let consumer: VerifierConsumer<'_, P, K, A, D> = build_verifier_consumer(
                pairing,
                kem,
                key_agreement,
                delivery_proof,
                random,
                build_delivery_context::<F, MintPurpose>(
                    DELIVERY_PURPOSE_MINT,
                    DeliveryContextOverrides {
                        expiry: Some(11),
                        ..Default::default()
                    },
                ),
                build_delivery_context::<F, TransferPurpose>(
                    DELIVERY_PURPOSE_TRANSFER,
                    DeliveryContextOverrides {
                        expiry: Some(12),
                        ..Default::default()
                    },
                ),
                identity,
                VerifierConsumerConstructorParamsOverrides {
                    iterations: NonZeroU32::new(7),
                },
            );
            assert_eq!(consumer.mint.expiry, 11);
            assert_eq!(consumer.transfer.expiry, 12);
            assert_eq!(consumer.iterations.get(), 7);
            assert!(core::ptr::eq(consumer.pairing, pairing));
            assert!(core::ptr::eq(consumer.kem, kem));
            assert!(core::ptr::eq(consumer.key_agreement, key_agreement));
            assert!(core::ptr::eq(consumer.delivery_proof, delivery_proof));
            assert!(core::ptr::addr_eq(consumer.random, random));
            match (consumer.identity.setup_scope, identity.setup_scope) {
                (SetupScope::Entitlement, SetupScope::Entitlement) => {}
                (
                    SetupScope::Asset { root: held_root },
                    SetupScope::Asset {
                        root: expected_root,
                    },
                ) => assert_eq!(held_root, expected_root),
                _ => panic!("a held setup scope is the received scope's kind"),
            }
            match (consumer.identity.identity, identity.identity) {
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
            create_verifier(
                &build_create_verifier_deps(pairing, delivery_proof, consumer, Default::default()),
                build_create_verifier_params(Default::default()),
                build_create_verifier_payload(Default::default()),
            )
        }
    }

    let canonical: &[u8] = b"pkg@1.0.0#1";
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    let deps = build_measure_case_deps(CaseConsumer, Default::default());

    // Act
    let result = run_measure_case(
        &deps,
        build_measure_case_params(Default::default()),
        build_measure_case_payload(canonical, &root, Default::default()),
    );

    // Assert
    let output = result.expect("the case composes").output;
    assert!(output.is_ok());
    // try_new_holds_every_value_it_is_handed
}
