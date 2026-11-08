#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::path::PathBuf;

use chain::IChainForms;
use core::num::NonZeroU32;
use encoding::{IDecoderAdapter, IEncoderAdapter};
use envelope::{IKeyAgreementAdapter, build_create_key_agreement_params};
use hash_to_scalar::IHashToScalarAdapter;
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::{
    CreateKemReturn, ICredentialKemAdapter, KemIdentity, SetupScope, build_create_kem_deps,
    build_create_kem_params, build_create_kem_payload, create_kem,
};
use pairing::{IPairingArithmetic, IPairingReference};
use proof::{
    DELIVERY_PURPOSE_MINT, DELIVERY_PURPOSE_TRANSFER, DeliveryContextOverrides,
    IDeliveryProofAdapter, MintPurpose, TransferPurpose, build_create_delivery_proof_params,
    build_delivery_context,
};
use random::IRandomSourceAdapter;

use super::interface::{KemConsumer, KemConsumerReturn};
use super::mock::{KemConsumerConstructorParamsOverrides, build_kem_consumer};
use super::pair_identities;
use crate::config::provides::{
    AdmittedIdentitiesOverrides, AdmittedIdentity, AdmittedKeyAgreement, build_admitted_identities,
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
/// Arrange: the default params, which select the mock key agreement, delivery
///   proof, generate, and verifier concretes under `NoFailure`; `iterations`
///   of `3`, a count other than the builder's default; the expected files the
///   five kinds the mock concrete's declaration admits with each `path` the
///   kind's name as text, and the expected cost the literal `VerifyCost {
///   l2_execution_gas: 100_000, l1_data_fee: L1DataFee::Charged { wei: 1_000 }
///   }`, as independent literals.
/// Act:     `create_kem`, once, inside the consumer.
/// Assert:  the output matches `Ok(CreateKemSuccessReturn { output:
///   Ok(DeliveryProofConsumerSuccessReturn { files, measurement }) })`,
///   `files` equals the expected files in that order by `assert_eq!`,
///   `measurement.iterations.get()` equals `3`, and `measurement.costs.mint`
///   and `measurement.costs.transfer` each equal the expected cost.
#[test]
fn consume_kem_returns_the_files_and_the_measurement_both_steps_computed() {
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
            let consumer: KemConsumer<'_, P, E, F> = build_kem_consumer(
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
                    iterations: NonZeroU32::new(3),
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
    // consume_kem_returns_the_files_and_the_measurement_both_steps_computed
}

/// Contract: Construction → the consumer holds the adapters, the mint context,
///   the transfer context, the admitted identities, and the configured values
///   it is constructed with.
/// Arrange: a mint context built with `expiry` `Some(11)` and
///   `MintPurpose::Replacement`, a transfer context built with `expiry`
///   `Some(12)` and `TransferPurpose::Grant`, `statement_version` of `2`,
///   `iterations` of `7`, an `AdmittedIdentities` whose `proof` and `mapping`
///   are the default entitlement identities, and the received adapters.
/// Act:     `build_kem_consumer`, once, inside the case consumer's method.
/// Assert:  the consumer's `mint.expiry` equals `11` and `mint.purpose.code()`
///   equals `4`, its `transfer.expiry` equals `12` and
///   `transfer.purpose.code()` equals `3`, its `statement_version` equals `2`,
///   its `iterations.get()` equals `7`, its `pairing`, `encoder`, and
///   `identities` are the received borrows by `core::ptr::eq`, its
///   `hash_to_scalar` and `random` are the received adapters by
///   `core::ptr::addr_eq`, and its `key_agreement`, `delivery_proof`,
///   `generate`, `kinds`, and `verifier` equal the builder's defaults.
#[test]
fn try_new_holds_every_value_it_is_handed() {
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
            let consumer: KemConsumer<'_, P, E, F> = build_kem_consumer(
                pairing,
                hash_to_scalar,
                encoder,
                random,
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
                &admitted_identities,
                KemConsumerConstructorParamsOverrides {
                    statement_version: Some(2),
                    iterations: NonZeroU32::new(7),
                    ..Default::default()
                },
            );
            assert_eq!(consumer.mint.expiry, 11);
            assert_eq!(consumer.mint.purpose.code(), 4);
            assert_eq!(consumer.transfer.expiry, 12);
            assert_eq!(consumer.transfer.purpose.code(), 3);
            assert_eq!(consumer.statement_version, 2);
            assert_eq!(consumer.iterations.get(), 7);
            assert!(core::ptr::eq(consumer.pairing, pairing));
            assert!(core::ptr::eq(consumer.encoder, encoder));
            assert!(core::ptr::eq(consumer.identities, &admitted_identities));
            assert!(core::ptr::addr_eq(consumer.hash_to_scalar, hash_to_scalar));
            assert!(core::ptr::addr_eq(consumer.random, random));
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
    assert!(output.is_ok());
    // try_new_holds_every_value_it_is_handed
}

/// Contract: Pairing → the proof identity and each mapping identity, in the
///   order held, are paired as `SetupScope::Entitlement` with
///   `KemIdentity::Entitlement` over the canonical bytes for an entitlement
///   identity and as `SetupScope::Asset` with `KemIdentity::Asset` over the
///   root for an asset identity, each borrowing the held value.
/// Arrange: an `AdmittedIdentities` built with `build_admitted_identities`
///   whose `proof` is `AdmittedIdentity::Entitlement` over `vec![0xaa, 0x01]`
///   and whose `mapping` is `AdmittedIdentity::Asset` over `build_root` at its
///   defaults followed by `AdmittedIdentity::Entitlement` over
///   `vec![0xbb, 0x02]`, as independent literals.
/// Act:     `pair_identities` over the arranged identities, once.
/// Assert:  the proof identity's `setup_scope` matches `SetupScope::Entitlement`
///   and its `identity` matches `KemIdentity::Entitlement { canonical }` with
///   `canonical` equal to `[0xaa, 0x01]`; the mapping holds two values, the
///   first with `setup_scope` matching `SetupScope::Asset { root }` and
///   `identity` matching `KemIdentity::Asset { root }` with both `root`
///   borrows `core::ptr::eq` to the held root, the second with
///   `SetupScope::Entitlement` and `KemIdentity::Entitlement` over
///   `[0xbb, 0x02]`, the patterns naming every field.
#[test]
fn consume_kem_pairs_each_admitted_identity_with_the_setup_scope_its_variant_names() {
    // Arrange
    let root = build_root(Default::default());
    let admitted = build_admitted_identities(AdmittedIdentitiesOverrides {
        proof: Some(AdmittedIdentity::Entitlement {
            canonical: vec![0xaa, 0x01],
        }),
        mapping: Some(vec![
            AdmittedIdentity::Asset { root },
            AdmittedIdentity::Entitlement {
                canonical: vec![0xbb, 0x02],
            },
        ]),
    });

    // Act
    let (proof_identity, mapping) = pair_identities(&admitted);

    // Assert
    assert!(matches!(
        proof_identity.setup_scope,
        SetupScope::Entitlement
    ));
    match proof_identity.identity {
        KemIdentity::Entitlement { canonical } => {
            assert_eq!(canonical, [0xaa, 0x01].as_slice());
        }
        KemIdentity::Asset { root: _ } => {
            panic!("the proof identity pairs as its variant states")
        }
    }
    assert_eq!(mapping.len(), 2);
    let AdmittedIdentity::Asset { root: held_root } = &admitted.mapping[0] else {
        panic!("the arranged first mapping identity is the asset variant")
    };
    match (mapping[0].setup_scope, mapping[0].identity) {
        (
            SetupScope::Asset { root: scope_root },
            KemIdentity::Asset {
                root: identity_root,
            },
        ) => {
            assert!(core::ptr::eq(scope_root, held_root));
            assert!(core::ptr::eq(identity_root, held_root));
        }
        _ => panic!("the first mapping identity pairs as its variant states"),
    }
    match (mapping[1].setup_scope, mapping[1].identity) {
        (SetupScope::Entitlement, KemIdentity::Entitlement { canonical }) => {
            assert_eq!(canonical, [0xbb, 0x02].as_slice());
        }
        _ => panic!("the second mapping identity pairs as its variant states"),
    }
    // consume_kem_pairs_each_admitted_identity_with_the_setup_scope_its_variant_names
}
