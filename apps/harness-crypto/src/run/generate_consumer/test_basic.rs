#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use chain::IChainForms;
use encoding::{IDecoderAdapter, IEncoderAdapter};
use envelope::IKeyAgreementAdapter;
use hash_to_scalar::IHashToScalarAdapter;
use hashing::{Root, RootConstructorParamsOverrides, build_root};
use kem::{ICredentialKemAdapter, KemIdentity, SetupScope};
use pairing::{IPairingArithmetic, IPairingReference};
use proof::{
    DeliveryContextOverrides, IDeliveryProofAdapter, MintPurpose, TransferPurpose,
    build_delivery_context,
};
use random::IRandomSourceAdapter;
use std::path::PathBuf;

use super::interface::{GenerateConsumer, GenerateConsumerReturn};
use super::mock::build_generate_consumer;
use crate::delivery::provides::AdmittedKemIdentity;
use crate::generate::provides::{
    CreateGenerateReturn, EmitSuccessReturn, EmittedFileOverrides, ICreateGenerateCaseConsumer,
    VectorKind, build_create_generate_case_deps, build_create_generate_case_params,
    build_create_generate_case_payload, build_create_generate_deps, build_create_generate_params,
    build_create_generate_payload, build_emitted_file, create_generate, run_create_generate_case,
};

/// Contract: Emitted → the success `emit` returns is the output whole, its
///   files in write order.
/// Arrange: the default params, which select the family's mock concrete under
///   `NoFailure`; the expected files built by `build_emitted_file` with `kind`
///   `Some(VectorKind::G1Addition)`,
///   `Some(VectorKind::G1MultiScalarMultiplication)`,
///   `Some(VectorKind::HashToScalar)`, `Some(VectorKind::DecodeRejection)`, and
///   `Some(VectorKind::IdentityMapping)` and `path` the kind's name as text, as
///   independent literals.
/// Act:     `create_generate`, once, inside the consumer.
/// Assert:  the output's `ok()` equals `Some(EmitSuccessReturn { files })` with
///   the expected files in that order.
#[test]
fn consume_generate_returns_the_files_the_concrete_emits() {
    // Arrange
    let expected = vec![
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

    struct CaseConsumer;

    impl ICreateGenerateCaseConsumer for CaseConsumer {
        type Output = CreateGenerateReturn<GenerateConsumerReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
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
            let proof_identity = identities[0];
            let consumer: GenerateConsumer<'_, F> = build_generate_consumer(
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default()),
                build_delivery_context::<F, TransferPurpose>(
                    TransferPurpose::Transfer,
                    Default::default(),
                ),
                identities,
                proof_identity,
                Default::default(),
            );
            create_generate(
                &build_create_generate_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    kem,
                    key_agreement,
                    delivery_proof,
                    consumer,
                    Default::default(),
                ),
                build_create_generate_params(Default::default()),
                build_create_generate_payload(Default::default()),
            )
        }
    }

    let canonicals: [&[u8]; 2] = [b"example-package@1.0.0", b"example-package@2.0.0"];
    let root_default = build_root(Default::default());
    let root_marked = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x11; 32]),
    });
    let roots: [&Root; 2] = [&root_default, &root_marked];
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
        .output;
    assert_eq!(output.ok(), Some(EmitSuccessReturn { files: expected }));
    // consume_generate_returns_the_files_the_concrete_emits
}

/// Contract: Construction → the consumer holds the mint context, the transfer
///   context, the identities, and the proof identity it is constructed with.
/// Arrange: a mint context built with `expiry` `Some(11)` and
///   `MintPurpose::Replacement`, a transfer context built with `expiry`
///   `Some(12)` and `TransferPurpose::Grant`, and the received identities.
/// Act:     `build_generate_consumer`, once, inside the case consumer's method.
/// Assert:  the consumer's `mint.expiry` equals `11` and `mint.purpose.code()`
///   equals `4`, its `transfer.expiry` equals `12` and
///   `transfer.purpose.code()` equals `3`, and its `identities` and
///   `proof_identity` hold the received values, the identities compared in
///   length and each element's `setup_scope` and `identity` by the kinds the
///   received scope reports.
#[test]
fn try_new_holds_every_value_it_is_handed() {
    // Arrange
    struct CaseConsumer;

    impl ICreateGenerateCaseConsumer for CaseConsumer {
        type Output = CreateGenerateReturn<GenerateConsumerReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
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
            let received = identities;
            let proof_identity = received[0];
            let consumer: GenerateConsumer<'_, F> = build_generate_consumer(
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
                received.clone(),
                proof_identity,
                Default::default(),
            );
            assert_eq!(consumer.mint.expiry, 11);
            assert_eq!(consumer.mint.purpose.code(), 4);
            assert_eq!(consumer.transfer.expiry, 12);
            assert_eq!(consumer.transfer.purpose.code(), 3);
            assert_eq!(consumer.identities.len(), received.len());
            for (held, expected) in consumer.identities.iter().zip(received.iter()) {
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
            match (consumer.proof_identity.setup_scope, received[0].setup_scope) {
                (SetupScope::Entitlement, SetupScope::Entitlement) => {}
                (
                    SetupScope::Asset { root: held_root },
                    SetupScope::Asset {
                        root: expected_root,
                    },
                ) => assert_eq!(held_root, expected_root),
                _ => panic!("the proof identity's setup scope is the first received one's kind"),
            }
            match (consumer.proof_identity.identity, received[0].identity) {
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
                _ => panic!("the proof identity's identity is the first received one's kind"),
            }
            create_generate(
                &build_create_generate_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    kem,
                    key_agreement,
                    delivery_proof,
                    consumer,
                    Default::default(),
                ),
                build_create_generate_params(Default::default()),
                build_create_generate_payload(Default::default()),
            )
        }
    }

    let canonicals: [&[u8]; 2] = [b"example-package@1.0.0", b"example-package@2.0.0"];
    let root_default = build_root(Default::default());
    let root_marked = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x11; 32]),
    });
    let roots: [&Root; 2] = [&root_default, &root_marked];
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
