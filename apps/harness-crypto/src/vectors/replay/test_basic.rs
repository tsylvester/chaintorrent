#![allow(clippy::expect_used)]

use chain::IChainForms;
use domain::Secret;
use encoding::{
    CanonicalFieldValue, ICanonicalField, IDecoderAdapter, IEncoderAdapter, ToFieldParams,
    build_from_field_params,
};
use envelope::IKeyAgreementAdapter;
use hash_to_scalar::IHashToScalarAdapter;
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::{
    CreateKemParamsOverrides, DeriveIdentityPayloadOverrides, ICredentialKemAdapter, IdentityScope,
    SetupParamsOverrides, SetupPayloadOverrides, build_create_kem_params,
    build_derive_identity_params, build_derive_identity_payload,
    build_identity_element_components_params, build_identity_element_components_payload,
    build_setup_params, build_setup_payload,
};
use pairing::{
    EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload, IPairingArithmetic,
    IPairingReference, ISampleUniformScalar,
};
use proof::{
    AlgebraicStatement, DELIVERY_PURPOSE_GRANT, DELIVERY_PURPOSE_MINT, DELIVERY_PURPOSE_TRANSFER,
    DeliveryContextOverrides, IDeliveryProofAdapter, MintPurpose, TransferPurpose, VerifyParams,
    VerifyPayload, build_delivery_context,
};
use random::{
    FillBytesParams, FillBytesPayloadOverrides, IRandomSourceAdapter, build_fill_bytes_payload,
};

use super::interface::IReplayCaseConsumer;
use super::mock::{
    ReplayCaseParamsOverrides, ReplayParamsOverrides, ReplayPayloadOverrides,
    build_replay_case_deps, build_replay_case_params, build_replay_case_payload, build_replay_deps,
    build_replay_params, build_replay_payload, run_replay_case,
};
use super::replay;
use crate::delivery::provides::AdmittedKemIdentity;
use crate::{VectorCountConstructorParamsOverrides, build_vector_count};

const CANONICAL: &[u8] = b"pkg@1.0.0#1";

fn asset_root() -> hashing::Root {
    build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    })
}

fn encode_g1<P: IPairingArithmetic>(pairing: &P, point: &P::G1) -> Vec<u8> {
    pairing
        .encode_g1(
            EncodeG1Params,
            EncodeG1Payload {
                point: point.clone(),
            },
        )
        .expect("the first-group encoding is infallible")
        .bytes
        .as_ref()
        .to_vec()
}

fn encode_g2<P: IPairingArithmetic>(pairing: &P, point: &P::G2) -> Vec<u8> {
    pairing
        .encode_g2(
            EncodeG2Params,
            EncodeG2Payload {
                point: point.clone(),
            },
        )
        .expect("the second-group encoding is infallible")
        .bytes
        .as_ref()
        .to_vec()
}

fn drawn_uniform<P: IPairingArithmetic>(source: &dyn IRandomSourceAdapter) -> Secret<Vec<u8>> {
    source
        .fill_bytes(
            FillBytesParams,
            build_fill_bytes_payload(FillBytesPayloadOverrides {
                length: Some(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH),
            }),
        )
        .expect("the independent source yields its draws")
        .bytes
}

/// Contract: Computed → `Ok(ReplaySuccessReturn { vectors })`, a record for each
///   index from `0` below `params.count.get()`.
/// Arrange: `count` of `3`, a count other than the builder's default.
/// Act:     `replay`.
/// Assert:  `vectors` holds `3` records.
#[test]
fn replay_emits_the_configured_count_of_records() {
    struct Consumer;

    impl IReplayCaseConsumer for Consumer {
        type Output = ();

        fn consume_replay_case<P, H, E, F, K, A, D>(
            &self,
            _pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
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
        {
            // Arrange
            let mint_context =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            let next_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                DeliveryContextOverrides {
                    chain: Some(transfer_context.chain.clone()),
                    entitlement_contract: Some(transfer_context.entitlement_contract.clone()),
                    asset_root: Some(transfer_context.asset_root),
                    parameter_set_identifier: Some(
                        transfer_context.parameter_set_identifier.clone(),
                    ),
                    source_entitlement: Some(transfer_context.source_entitlement.clone()),
                    target_entitlement: Some(transfer_context.target_entitlement.clone()),
                    old_interval: Some(transfer_context.new_interval.clone()),
                    seller: Some(transfer_context.buyer.clone()),
                    ..Default::default()
                },
            );
            let params = build_replay_params(ReplayParamsOverrides {
                count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                    count: Some(3),
                })),
            });
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                params,
                build_replay_payload(
                    &mint_context,
                    &transfer_context,
                    &next_context,
                    ReplayPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            let success = returned.expect("the subject's calls succeed");
            assert_eq!(success.vectors.len(), 3);
        }
    }
    let result = run_replay_case(
        &build_replay_case_deps(Consumer, Default::default()),
        build_replay_case_params(Default::default()),
        build_replay_case_payload(CANONICAL, &asset_root(), Default::default()),
    );
    assert!(result.is_ok());
}

/// Contract: Next settlement delivery → every uniform has the length the
///   pairing's scalar sampling bound declares, and a record's `draws` are the
///   mint delivery's draws, the transfer settlement's draws, and the next
///   settlement's draws concatenated.
/// Arrange: `count` of `1`.
/// Act:     `replay`.
/// Assert:  the record's `draws` hold `12` times
///   `<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH` bytes.
#[test]
fn replay_draws_each_record_twelve_uniforms_at_the_declared_length() {
    struct Consumer;

    impl IReplayCaseConsumer for Consumer {
        type Output = ();

        fn consume_replay_case<P, H, E, F, K, A, D>(
            &self,
            _pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
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
        {
            // Arrange
            let mint_context =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            let next_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                DeliveryContextOverrides {
                    chain: Some(transfer_context.chain.clone()),
                    entitlement_contract: Some(transfer_context.entitlement_contract.clone()),
                    asset_root: Some(transfer_context.asset_root),
                    parameter_set_identifier: Some(
                        transfer_context.parameter_set_identifier.clone(),
                    ),
                    source_entitlement: Some(transfer_context.source_entitlement.clone()),
                    target_entitlement: Some(transfer_context.target_entitlement.clone()),
                    old_interval: Some(transfer_context.new_interval.clone()),
                    seller: Some(transfer_context.buyer.clone()),
                    ..Default::default()
                },
            );
            let params = build_replay_params(ReplayParamsOverrides {
                count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                    count: Some(1),
                })),
            });
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                params,
                build_replay_payload(
                    &mint_context,
                    &transfer_context,
                    &next_context,
                    ReplayPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            let success = returned.expect("the subject's calls succeed");
            assert_eq!(
                success.vectors[0].draws.len(),
                12 * <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
            );
        }
    }
    let result = run_replay_case(
        &build_replay_case_deps(Consumer, Default::default()),
        build_replay_case_params(Default::default()),
        build_replay_case_payload(CANONICAL, &asset_root(), Default::default()),
    );
    assert!(result.is_ok());
}

/// Contract: Ordering → the records in index order, each record's draws in draw
///   order.
/// Arrange: `count` of `2`; the expected draws from the independent source,
///   twelve consecutive calls for each record in turn, so a transposed record or
///   draw reports otherwise.
/// Act:     `replay`.
/// Assert:  each record's `draws` equal its expected draws.
#[test]
fn replay_records_the_draws_in_the_sequence_the_source_yields() {
    struct Consumer;

    impl IReplayCaseConsumer for Consumer {
        type Output = ();

        fn consume_replay_case<P, H, E, F, K, A, D>(
            &self,
            _pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            independent: &dyn IRandomSourceAdapter,
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
        {
            // Arrange
            let mint_context =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            let next_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                DeliveryContextOverrides {
                    chain: Some(transfer_context.chain.clone()),
                    entitlement_contract: Some(transfer_context.entitlement_contract.clone()),
                    asset_root: Some(transfer_context.asset_root),
                    parameter_set_identifier: Some(
                        transfer_context.parameter_set_identifier.clone(),
                    ),
                    source_entitlement: Some(transfer_context.source_entitlement.clone()),
                    target_entitlement: Some(transfer_context.target_entitlement.clone()),
                    old_interval: Some(transfer_context.new_interval.clone()),
                    seller: Some(transfer_context.buyer.clone()),
                    ..Default::default()
                },
            );
            let params = build_replay_params(ReplayParamsOverrides {
                count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                    count: Some(2),
                })),
            });
            let mut expected: Vec<Vec<u8>> = Vec::new();
            for _ in 0..2 {
                let mut draws = Vec::new();
                for _ in 0..12 {
                    draws.extend_from_slice(drawn_uniform::<P>(independent).expose().as_ref());
                }
                expected.push(draws);
            }
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                params,
                build_replay_payload(
                    &mint_context,
                    &transfer_context,
                    &next_context,
                    ReplayPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            let success = returned.expect("the subject's calls succeed");
            assert_eq!(success.vectors.len(), 2);
            assert_eq!(success.vectors[0].draws, expected[0]);
            assert_eq!(success.vectors[1].draws, expected[1]);
        }
    }
    let result = run_replay_case(
        &build_replay_case_deps(Consumer, Default::default()),
        build_replay_case_params(Default::default()),
        build_replay_case_payload(CANONICAL, &asset_root(), Default::default()),
    );
    assert!(result.is_ok());
}

/// Contract: Verdict → `transfer_is_valid`, `next_is_valid`, and
///   `replay_is_valid` are the verdicts `verify` returns over the transfer
///   statement and proof, the next statement and proof, and the next statement
///   and the transfer proof.
/// Arrange: `count` of `1`; the settlements verify on the family's mock concrete
///   and the next statement differs from the transfer statement in its
///   intervals, parties, and envelopes, so a replay verdict read from anything
///   but `verify` over that pair reports otherwise.
/// Act:     `replay`.
/// Assert:  `transfer_is_valid` and `next_is_valid` are `true` and
///   `replay_is_valid` is `false`, and `replay_is_valid` equals the `is_valid`
///   the received delivery proof's `verify` returns over
///   `AlgebraicStatement::Transfer(&next_statement)` and `&transfer_proof`, the
///   pair this Assert line names from the record.
#[test]
fn replay_records_each_verdict_as_verify_returns_it() {
    struct Consumer;

    impl IReplayCaseConsumer for Consumer {
        type Output = ();

        fn consume_replay_case<P, H, E, F, K, A, D>(
            &self,
            _pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
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
        {
            // Arrange
            let mint_context =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            let next_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                DeliveryContextOverrides {
                    chain: Some(transfer_context.chain.clone()),
                    entitlement_contract: Some(transfer_context.entitlement_contract.clone()),
                    asset_root: Some(transfer_context.asset_root),
                    parameter_set_identifier: Some(
                        transfer_context.parameter_set_identifier.clone(),
                    ),
                    source_entitlement: Some(transfer_context.source_entitlement.clone()),
                    target_entitlement: Some(transfer_context.target_entitlement.clone()),
                    old_interval: Some(transfer_context.new_interval.clone()),
                    seller: Some(transfer_context.buyer.clone()),
                    ..Default::default()
                },
            );
            let params = build_replay_params(ReplayParamsOverrides {
                count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                    count: Some(1),
                })),
            });
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                params,
                build_replay_payload(
                    &mint_context,
                    &transfer_context,
                    &next_context,
                    ReplayPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            let success = returned.expect("the subject's calls succeed");
            let record = &success.vectors[0];
            assert!(record.transfer_is_valid);
            assert!(record.next_is_valid);
            assert!(!record.replay_is_valid);
            let replay_verdict = delivery_proof
                .verify(
                    VerifyParams,
                    VerifyPayload {
                        statement: AlgebraicStatement::Transfer(&record.next_statement),
                        proof: &record.transfer_proof,
                    },
                )
                .expect("the mock verifier returns a verdict")
                .is_valid;
            assert_eq!(record.replay_is_valid, replay_verdict);
        }
    }
    let result = run_replay_case(
        &build_replay_case_deps(Consumer, Default::default()),
        build_replay_case_params(Default::default()),
        build_replay_case_payload(CANONICAL, &asset_root(), Default::default()),
    );
    assert!(result.is_ok());
}

/// Contract: Next settlement delivery → the transfer settlement's buyer is the
///   next settlement's holder, so its keys are the next statement's seller keys
///   and the envelope the transfer delivered is its old envelope.
/// Arrange: `count` of `1`.
/// Act:     `replay`.
/// Assert:  the `encode_g1` and `encode_g2` of the next statement's
///   `seller_keys` `pk1` and `pk2` equal those of the transfer statement's
///   `buyer_keys`, and the encodings of the next statement's `old_envelope`
///   `c1`, `c2`, `d1`, and `d2` equal those of the transfer statement's
///   `new_envelope`.
#[test]
fn replay_builds_the_next_settlement_from_the_transfers_buyer_and_envelope() {
    struct Consumer;

    impl IReplayCaseConsumer for Consumer {
        type Output = ();

        fn consume_replay_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
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
        {
            // Arrange
            let mint_context =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            let next_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                DeliveryContextOverrides {
                    chain: Some(transfer_context.chain.clone()),
                    entitlement_contract: Some(transfer_context.entitlement_contract.clone()),
                    asset_root: Some(transfer_context.asset_root),
                    parameter_set_identifier: Some(
                        transfer_context.parameter_set_identifier.clone(),
                    ),
                    source_entitlement: Some(transfer_context.source_entitlement.clone()),
                    target_entitlement: Some(transfer_context.target_entitlement.clone()),
                    old_interval: Some(transfer_context.new_interval.clone()),
                    seller: Some(transfer_context.buyer.clone()),
                    ..Default::default()
                },
            );
            let params = build_replay_params(ReplayParamsOverrides {
                count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                    count: Some(1),
                })),
            });
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                params,
                build_replay_payload(
                    &mint_context,
                    &transfer_context,
                    &next_context,
                    ReplayPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            let success = returned.expect("the subject's calls succeed");
            let record = &success.vectors[0];
            assert_eq!(
                encode_g1(pairing, &record.next_statement.seller_keys.pk1),
                encode_g1(pairing, &record.transfer_statement.buyer_keys.pk1)
            );
            assert_eq!(
                encode_g2(pairing, &record.next_statement.seller_keys.pk2),
                encode_g2(pairing, &record.transfer_statement.buyer_keys.pk2)
            );
            assert_eq!(
                encode_g1(pairing, &record.next_statement.old_envelope.c1),
                encode_g1(pairing, &record.transfer_statement.new_envelope.c1)
            );
            assert_eq!(
                encode_g1(pairing, &record.next_statement.old_envelope.c2),
                encode_g1(pairing, &record.transfer_statement.new_envelope.c2)
            );
            assert_eq!(
                encode_g2(pairing, &record.next_statement.old_envelope.d1),
                encode_g2(pairing, &record.transfer_statement.new_envelope.d1)
            );
            assert_eq!(
                encode_g2(pairing, &record.next_statement.old_envelope.d2),
                encode_g2(pairing, &record.transfer_statement.new_envelope.d2)
            );
        }
    }
    let result = run_replay_case(
        &build_replay_case_deps(Consumer, Default::default()),
        build_replay_case_params(Default::default()),
        build_replay_case_payload(CANONICAL, &asset_root(), Default::default()),
    );
    assert!(result.is_ok());
}

/// Contract: Transfer settlement and Next settlement delivery → each
///   statement's context is its own typed context, the next context's buyer,
///   new interval, and expiry carried into the next statement alone.
/// Arrange: `count` of `1`; the next context additionally overriding
///   `new_interval` with the form over `CanonicalFieldValue::Unsigned64(3)`,
///   `buyer` with the form over `CanonicalFieldValue::FixedBytes20([0x63; 20])`,
///   and `expiry` with `1_900_000_000`.
/// Act:     `replay`.
/// Assert:  the `to_field` of the transfer statement's context's `buyer` and
///   `new_interval` equal those of the typed transfer context's and its `expiry`
///   equals the typed transfer context's; those of the next statement's context
///   equal `CanonicalFieldValue::FixedBytes20([0x63; 20])` and
///   `CanonicalFieldValue::Unsigned64(3)` and its `expiry` equals
///   `1_900_000_000`.
#[test]
fn replay_builds_each_statement_over_its_typed_context() {
    struct Consumer;

    impl IReplayCaseConsumer for Consumer {
        type Output = ();

        fn consume_replay_case<P, H, E, F, K, A, D>(
            &self,
            _pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
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
        {
            // Arrange
            let mint_context =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            let next_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                DeliveryContextOverrides {
                    chain: Some(transfer_context.chain.clone()),
                    entitlement_contract: Some(transfer_context.entitlement_contract.clone()),
                    asset_root: Some(transfer_context.asset_root),
                    parameter_set_identifier: Some(
                        transfer_context.parameter_set_identifier.clone(),
                    ),
                    source_entitlement: Some(transfer_context.source_entitlement.clone()),
                    target_entitlement: Some(transfer_context.target_entitlement.clone()),
                    old_interval: Some(transfer_context.new_interval.clone()),
                    seller: Some(transfer_context.buyer.clone()),
                    new_interval: Some(
                        <F::Interval as ICanonicalField>::from_field(
                            build_from_field_params(Default::default()),
                            CanonicalFieldValue::Unsigned64(3),
                        )
                        .expect("the value has the kind the form declares")
                        .value,
                    ),
                    buyer: Some(
                        <F::Identity as ICanonicalField>::from_field(
                            build_from_field_params(Default::default()),
                            CanonicalFieldValue::FixedBytes20([0x63; 20]),
                        )
                        .expect("the value has the kind the form declares")
                        .value,
                    ),
                    expiry: Some(1_900_000_000),
                    ..Default::default()
                },
            );
            let params = build_replay_params(ReplayParamsOverrides {
                count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                    count: Some(1),
                })),
            });
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                params,
                build_replay_payload(
                    &mint_context,
                    &transfer_context,
                    &next_context,
                    ReplayPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            let success = returned.expect("the subject's calls succeed");
            let record = &success.vectors[0];
            assert_eq!(
                <F::Identity as ICanonicalField>::to_field(
                    ToFieldParams,
                    &record.transfer_statement.context.buyer,
                )
                .expect("the buyer field reads")
                .field,
                <F::Identity as ICanonicalField>::to_field(ToFieldParams, &transfer_context.buyer,)
                    .expect("the buyer field reads")
                    .field
            );
            assert_eq!(
                <F::Interval as ICanonicalField>::to_field(
                    ToFieldParams,
                    &record.transfer_statement.context.new_interval,
                )
                .expect("the interval field reads")
                .field,
                <F::Interval as ICanonicalField>::to_field(
                    ToFieldParams,
                    &transfer_context.new_interval,
                )
                .expect("the interval field reads")
                .field
            );
            assert_eq!(
                record.transfer_statement.context.expiry,
                transfer_context.expiry
            );
            assert_eq!(
                <F::Identity as ICanonicalField>::to_field(
                    ToFieldParams,
                    &record.next_statement.context.buyer,
                )
                .expect("the buyer field reads")
                .field,
                CanonicalFieldValue::FixedBytes20([0x63; 20])
            );
            assert_eq!(
                <F::Interval as ICanonicalField>::to_field(
                    ToFieldParams,
                    &record.next_statement.context.new_interval,
                )
                .expect("the interval field reads")
                .field,
                CanonicalFieldValue::Unsigned64(3)
            );
            assert_eq!(record.next_statement.context.expiry, 1_900_000_000);
        }
    }
    let result = run_replay_case(
        &build_replay_case_deps(Consumer, Default::default()),
        build_replay_case_params(Default::default()),
        build_replay_case_payload(CANONICAL, &asset_root(), Default::default()),
    );
    assert!(result.is_ok());
}

/// Contract: Transfer settlement and Next settlement delivery → each statement
///   carries the purpose its typed context carries.
/// Arrange: `count` of `1`; the transfer and the next contexts built with
///   `DELIVERY_PURPOSE_GRANT`.
/// Act:     `replay`.
/// Assert:  the `code()` of the purpose of each statement's context equals that
///   of `DELIVERY_PURPOSE_GRANT`.
#[test]
fn replay_carries_each_contexts_purpose_into_its_statement() {
    struct Consumer;

    impl IReplayCaseConsumer for Consumer {
        type Output = ();

        fn consume_replay_case<P, H, E, F, K, A, D>(
            &self,
            _pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
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
        {
            // Arrange
            let mint_context =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_GRANT,
                Default::default(),
            );
            let next_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_GRANT,
                DeliveryContextOverrides {
                    chain: Some(transfer_context.chain.clone()),
                    entitlement_contract: Some(transfer_context.entitlement_contract.clone()),
                    asset_root: Some(transfer_context.asset_root),
                    parameter_set_identifier: Some(
                        transfer_context.parameter_set_identifier.clone(),
                    ),
                    source_entitlement: Some(transfer_context.source_entitlement.clone()),
                    target_entitlement: Some(transfer_context.target_entitlement.clone()),
                    old_interval: Some(transfer_context.new_interval.clone()),
                    seller: Some(transfer_context.buyer.clone()),
                    ..Default::default()
                },
            );
            let params = build_replay_params(ReplayParamsOverrides {
                count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                    count: Some(1),
                })),
            });
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                params,
                build_replay_payload(
                    &mint_context,
                    &transfer_context,
                    &next_context,
                    ReplayPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            let success = returned.expect("the subject's calls succeed");
            let record = &success.vectors[0];
            assert_eq!(
                record.transfer_statement.context.purpose.code(),
                DELIVERY_PURPOSE_GRANT.code()
            );
            assert_eq!(
                record.next_statement.context.purpose.code(),
                DELIVERY_PURPOSE_GRANT.code()
            );
        }
    }
    let result = run_replay_case(
        &build_replay_case_deps(Consumer, Default::default()),
        build_replay_case_params(Default::default()),
        build_replay_case_payload(CANONICAL, &asset_root(), Default::default()),
    );
    assert!(result.is_ok());
}

/// Contract: Mint delivery under an asset identity → the admitted identity's
///   setup scope `SetupScope::Asset { root }` and identity
///   `KemIdentity::Asset { root }` reach the mint delivery unchanged, and both
///   statements carry the identity element it derived.
/// Arrange: the KEM selection under the asset scope, the root `build_root`
///   builds from `[0x44; 32]`, `count` of `1`; the expected element from the
///   received KEM over the independent source's first three draws: `setup`
///   under the admitted identity's `setup_scope`, `derive_identity` over its
///   `identity`, and `identity_element_components`, each through the family's
///   builders.
/// Act:     `replay`.
/// Assert:  the `encode_g1` of the transfer statement's and the next
///   statement's `identity_element` each equal the `encode_g1` of the expected
///   element's `element`.
#[test]
fn replay_carries_the_identity_element_of_an_asset_identity() {
    struct Consumer;

    impl IReplayCaseConsumer for Consumer {
        type Output = ();

        fn consume_replay_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            independent: &dyn IRandomSourceAdapter,
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
        {
            // Arrange
            let mint_context =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            let next_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                DeliveryContextOverrides {
                    chain: Some(transfer_context.chain.clone()),
                    entitlement_contract: Some(transfer_context.entitlement_contract.clone()),
                    asset_root: Some(transfer_context.asset_root),
                    parameter_set_identifier: Some(
                        transfer_context.parameter_set_identifier.clone(),
                    ),
                    source_entitlement: Some(transfer_context.source_entitlement.clone()),
                    target_entitlement: Some(transfer_context.target_entitlement.clone()),
                    old_interval: Some(transfer_context.new_interval.clone()),
                    seller: Some(transfer_context.buyer.clone()),
                    ..Default::default()
                },
            );
            let params = build_replay_params(ReplayParamsOverrides {
                count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                    count: Some(1),
                })),
            });
            let setup = kem
                .setup(
                    build_setup_params(SetupParamsOverrides {
                        scope: Some(identity.setup_scope),
                    }),
                    build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                        master_uniform: Some(drawn_uniform::<P>(independent)),
                        u0_uniform: Some(drawn_uniform::<P>(independent)),
                        u1_uniform: Some(drawn_uniform::<P>(independent)),
                    }),
                )
                .expect("setup over the replayed uniforms succeeds");
            let derived = kem
                .derive_identity(
                    build_derive_identity_params(Default::default()),
                    build_derive_identity_payload(
                        &setup.parameter_set,
                        DeriveIdentityPayloadOverrides {
                            identity: Some(identity.identity),
                        },
                    ),
                )
                .expect("derive_identity over the admitted identity succeeds");
            let expected = kem
                .identity_element_components(
                    build_identity_element_components_params(Default::default()),
                    build_identity_element_components_payload(
                        &derived.identity_element,
                        Default::default(),
                    ),
                )
                .expect("identity_element_components over the derived element succeeds");
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                params,
                build_replay_payload(
                    &mint_context,
                    &transfer_context,
                    &next_context,
                    ReplayPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            let success = returned.expect("the subject's calls succeed");
            let record = &success.vectors[0];
            assert_eq!(
                encode_g1(pairing, &record.transfer_statement.identity_element),
                encode_g1(pairing, &expected.components.element)
            );
            assert_eq!(
                encode_g1(pairing, &record.next_statement.identity_element),
                encode_g1(pairing, &expected.components.element)
            );
        }
    }
    let result = run_replay_case(
        &build_replay_case_deps(Consumer, Default::default()),
        build_replay_case_params(ReplayCaseParamsOverrides {
            kem: Some(build_create_kem_params(CreateKemParamsOverrides {
                scope: Some(IdentityScope::Asset),
                ..Default::default()
            })),
            ..Default::default()
        }),
        build_replay_case_payload(CANONICAL, &asset_root(), Default::default()),
    );
    assert!(result.is_ok());
}
