#![allow(clippy::expect_used)]

use chain::IChainForms;
use domain::{
    ParameterSetIdentifierConstructorParamsOverrides, Secret, build_parameter_set_identifier,
};
use encoding::{
    CanonicalFieldValue, ICanonicalField, IDecoderAdapter, IEncoderAdapter, build_from_field_params,
};
use envelope::{
    CreateKeyAgreementParamsOverrides, GenerateKeysErrorReturn, IKeyAgreementAdapter,
    KeyAgreementConcrete, MockIKeyAgreementAdapterFailureMode,
    MockIKeyAgreementAdapterGenerateKeysErrorReturn, MockIKeyAgreementAdapterWrapToErrorReturn,
    WrapToErrorReturn, build_create_key_agreement_params,
};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, HashToScalarConcrete, IHashToScalarAdapter,
    MockIHashToScalarAdapterFailureMode, build_create_hash_to_scalar_params,
    build_hash_to_scalar_params, build_hash_to_scalar_payload,
};
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::{
    CreateKemParamsOverrides, DeriveIdentityErrorReturn, ICredentialKemAdapter, IdentityScope,
    SetupErrorReturn, build_create_kem_params,
};
use pairing::{
    CreatePairingParamsOverrides, IPairingArithmetic, IPairingReference, ISampleUniformScalar,
    MockIPairingAdapterFailureMode, PairingConcrete, SampleUniformScalarParams,
    SampleUniformScalarPayload, build_create_pairing_params,
};
use proof::{
    CreateDeliveryProofParamsOverrides, DELIVERY_PURPOSE_MINT, DELIVERY_PURPOSE_TRANSFER,
    DeliveryContextOverrides, DeliveryProofConcrete, IDeliveryProofAdapter, MintPurpose,
    MockIDeliveryProofAdapterFailureMode, MockIDeliveryProofAdapterProveMintErrorReturn,
    ProveMintErrorReturn, TransferPurpose, build_create_delivery_proof_params,
    build_delivery_context,
};
use random::{
    CreateRandomSourceParamsOverrides, FillBytesParams, FillBytesPayloadOverrides,
    IRandomSourceAdapter, MockIRandomSourceAdapterFailureMode, RandomSourceKind,
    build_create_random_source_params, build_fill_bytes_payload,
};

use super::interface::{IReplayCaseConsumer, NextSettlementField, ReplayErrorReturn};
use super::mock::{
    ReplayCaseParamsOverrides, ReplayPayloadOverrides, build_replay_case_deps,
    build_replay_case_params, build_replay_case_payload, build_replay_deps, build_replay_params,
    build_replay_payload, run_replay_case,
};
use super::replay;
use crate::delivery::provides::{AdmittedKemIdentity, MintErrorReturn};

const CANONICAL: &[u8] = b"pkg@1.0.0#1";

fn asset_root() -> hashing::Root {
    build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    })
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

/// Contract: Next settlement, the `to_field` of `payload.next.chain` differs
///   from that of `payload.transfer.chain` →
///   `Err(ReplayErrorReturn::NotTheNextSettlement { field:
///   NextSettlementField::Chain })`.
/// Arrange: the transfer context overriding `chain` with the form over
///   `CanonicalFieldValue::Unsigned256([0x21; 32])` and the next context
///   overriding `chain` with the form over
///   `CanonicalFieldValue::Unsigned256([0x22; 32])`.
/// Act:     `replay`.
/// Assert:  the result equals `Err(ReplayErrorReturn::NotTheNextSettlement
///   { field: NextSettlementField::Chain })` whole.
#[test]
fn replay_refuses_a_next_context_on_another_chain() {
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
                DeliveryContextOverrides {
                    chain: Some(
                        <F::ChainIdentifier as ICanonicalField>::from_field(
                            build_from_field_params(Default::default()),
                            CanonicalFieldValue::Unsigned256([0x21; 32]),
                        )
                        .expect("the value has the kind the form declares")
                        .value,
                    ),
                    ..Default::default()
                },
            );
            let next_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                DeliveryContextOverrides {
                    chain: Some(
                        <F::ChainIdentifier as ICanonicalField>::from_field(
                            build_from_field_params(Default::default()),
                            CanonicalFieldValue::Unsigned256([0x22; 32]),
                        )
                        .expect("the value has the kind the form declares")
                        .value,
                    ),
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
            let expected = ReplayErrorReturn::NotTheNextSettlement {
                field: NextSettlementField::Chain,
            };
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_replay_params(Default::default()),
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
            assert_eq!(returned.err(), Some(expected));
        }
    }
    let result = run_replay_case(
        &build_replay_case_deps(Consumer, Default::default()),
        build_replay_case_params(Default::default()),
        build_replay_case_payload(CANONICAL, &asset_root(), Default::default()),
    );
    assert!(result.is_ok());
}

/// Contract: Next settlement, the `to_field` of `entitlement_contract` differs
///   → `Err(ReplayErrorReturn::NotTheNextSettlement { field:
///   NextSettlementField::EntitlementContract })`.
/// Arrange: the next context overriding `entitlement_contract` with a clone of
///   the transfer context's `seller`.
/// Act:     `replay`.
/// Assert:  the result equals `Err(ReplayErrorReturn::NotTheNextSettlement
///   { field: NextSettlementField::EntitlementContract })` whole.
#[test]
fn replay_refuses_a_next_context_under_another_entitlement_contract() {
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
                    entitlement_contract: Some(transfer_context.seller.clone()),
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
            let expected = ReplayErrorReturn::NotTheNextSettlement {
                field: NextSettlementField::EntitlementContract,
            };
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_replay_params(Default::default()),
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
            assert_eq!(returned.err(), Some(expected));
        }
    }
    let result = run_replay_case(
        &build_replay_case_deps(Consumer, Default::default()),
        build_replay_case_params(Default::default()),
        build_replay_case_payload(CANONICAL, &asset_root(), Default::default()),
    );
    assert!(result.is_ok());
}

/// Contract: Next settlement, the `as_bytes()` of `asset_root` differ →
///   `Err(ReplayErrorReturn::NotTheNextSettlement { field:
///   NextSettlementField::AssetRoot })`.
/// Arrange: the transfer context overriding `asset_root` with `build_root` over
///   `[0x74; 32]` and the next context with `build_root` over `[0x75; 32]`.
/// Act:     `replay`.
/// Assert:  the result equals `Err(ReplayErrorReturn::NotTheNextSettlement
///   { field: NextSettlementField::AssetRoot })` whole.
#[test]
fn replay_refuses_a_next_context_of_another_asset() {
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
                DeliveryContextOverrides {
                    asset_root: Some(build_root(RootConstructorParamsOverrides {
                        bytes: Some([0x74; 32]),
                    })),
                    ..Default::default()
                },
            );
            let next_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                DeliveryContextOverrides {
                    chain: Some(transfer_context.chain.clone()),
                    entitlement_contract: Some(transfer_context.entitlement_contract.clone()),
                    asset_root: Some(build_root(RootConstructorParamsOverrides {
                        bytes: Some([0x75; 32]),
                    })),
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
            let expected = ReplayErrorReturn::NotTheNextSettlement {
                field: NextSettlementField::AssetRoot,
            };
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_replay_params(Default::default()),
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
            assert_eq!(returned.err(), Some(expected));
        }
    }
    let result = run_replay_case(
        &build_replay_case_deps(Consumer, Default::default()),
        build_replay_case_params(Default::default()),
        build_replay_case_payload(CANONICAL, &asset_root(), Default::default()),
    );
    assert!(result.is_ok());
}

/// Contract: Next settlement, the `as_bytes()` of `parameter_set_identifier`
///   differ → `Err(ReplayErrorReturn::NotTheNextSettlement { field:
///   NextSettlementField::ParameterSetIdentifier })`.
/// Arrange: the transfer context overriding `parameter_set_identifier` with
///   `build_parameter_set_identifier` over `[0x76; 32]` and the next context
///   with `build_parameter_set_identifier` over `[0x77; 32]`.
/// Act:     `replay`.
/// Assert:  the result equals `Err(ReplayErrorReturn::NotTheNextSettlement
///   { field: NextSettlementField::ParameterSetIdentifier })` whole.
#[test]
fn replay_refuses_a_next_context_under_another_parameter_set() {
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
                DeliveryContextOverrides {
                    parameter_set_identifier: Some(build_parameter_set_identifier(
                        ParameterSetIdentifierConstructorParamsOverrides {
                            bytes: Some([0x76; 32]),
                        },
                    )),
                    ..Default::default()
                },
            );
            let next_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                DeliveryContextOverrides {
                    chain: Some(transfer_context.chain.clone()),
                    entitlement_contract: Some(transfer_context.entitlement_contract.clone()),
                    asset_root: Some(transfer_context.asset_root),
                    parameter_set_identifier: Some(build_parameter_set_identifier(
                        ParameterSetIdentifierConstructorParamsOverrides {
                            bytes: Some([0x77; 32]),
                        },
                    )),
                    source_entitlement: Some(transfer_context.source_entitlement.clone()),
                    target_entitlement: Some(transfer_context.target_entitlement.clone()),
                    old_interval: Some(transfer_context.new_interval.clone()),
                    seller: Some(transfer_context.buyer.clone()),
                    ..Default::default()
                },
            );
            let expected = ReplayErrorReturn::NotTheNextSettlement {
                field: NextSettlementField::ParameterSetIdentifier,
            };
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_replay_params(Default::default()),
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
            assert_eq!(returned.err(), Some(expected));
        }
    }
    let result = run_replay_case(
        &build_replay_case_deps(Consumer, Default::default()),
        build_replay_case_params(Default::default()),
        build_replay_case_payload(CANONICAL, &asset_root(), Default::default()),
    );
    assert!(result.is_ok());
}

/// Contract: Next settlement, the `to_field` of `source_entitlement` differs →
///   `Err(ReplayErrorReturn::NotTheNextSettlement { field:
///   NextSettlementField::SourceEntitlement })`.
/// Arrange: the next context overriding `source_entitlement` with a clone of
///   the transfer context's `target_entitlement`.
/// Act:     `replay`.
/// Assert:  the result equals `Err(ReplayErrorReturn::NotTheNextSettlement
///   { field: NextSettlementField::SourceEntitlement })` whole.
#[test]
fn replay_refuses_a_next_context_from_another_source_entitlement() {
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
                    source_entitlement: Some(transfer_context.target_entitlement.clone()),
                    target_entitlement: Some(transfer_context.target_entitlement.clone()),
                    old_interval: Some(transfer_context.new_interval.clone()),
                    seller: Some(transfer_context.buyer.clone()),
                    ..Default::default()
                },
            );
            let expected = ReplayErrorReturn::NotTheNextSettlement {
                field: NextSettlementField::SourceEntitlement,
            };
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_replay_params(Default::default()),
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
            assert_eq!(returned.err(), Some(expected));
        }
    }
    let result = run_replay_case(
        &build_replay_case_deps(Consumer, Default::default()),
        build_replay_case_params(Default::default()),
        build_replay_case_payload(CANONICAL, &asset_root(), Default::default()),
    );
    assert!(result.is_ok());
}

/// Contract: Next settlement, the `to_field` of `target_entitlement` differs →
///   `Err(ReplayErrorReturn::NotTheNextSettlement { field:
///   NextSettlementField::TargetEntitlement })`.
/// Arrange: the next context overriding `target_entitlement` with a clone of
///   the transfer context's `source_entitlement`.
/// Act:     `replay`.
/// Assert:  the result equals `Err(ReplayErrorReturn::NotTheNextSettlement
///   { field: NextSettlementField::TargetEntitlement })` whole.
#[test]
fn replay_refuses_a_next_context_to_another_target_entitlement() {
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
                    target_entitlement: Some(transfer_context.source_entitlement.clone()),
                    old_interval: Some(transfer_context.new_interval.clone()),
                    seller: Some(transfer_context.buyer.clone()),
                    ..Default::default()
                },
            );
            let expected = ReplayErrorReturn::NotTheNextSettlement {
                field: NextSettlementField::TargetEntitlement,
            };
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_replay_params(Default::default()),
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
            assert_eq!(returned.err(), Some(expected));
        }
    }
    let result = run_replay_case(
        &build_replay_case_deps(Consumer, Default::default()),
        build_replay_case_params(Default::default()),
        build_replay_case_payload(CANONICAL, &asset_root(), Default::default()),
    );
    assert!(result.is_ok());
}

/// Contract: Next settlement, the `to_field` of `payload.next.old_interval`
///   differs from that of `payload.transfer.new_interval` →
///   `Err(ReplayErrorReturn::NotTheNextSettlement { field:
///   NextSettlementField::OldInterval })`.
/// Arrange: the next context overriding `old_interval` with a clone of the
///   transfer context's `old_interval`, the transfer's old interval and not its
///   new one.
/// Act:     `replay`.
/// Assert:  the result equals `Err(ReplayErrorReturn::NotTheNextSettlement
///   { field: NextSettlementField::OldInterval })` whole.
#[test]
fn replay_refuses_a_next_context_whose_old_interval_is_not_the_transfers_new_interval() {
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
                    old_interval: Some(transfer_context.old_interval.clone()),
                    seller: Some(transfer_context.buyer.clone()),
                    ..Default::default()
                },
            );
            let expected = ReplayErrorReturn::NotTheNextSettlement {
                field: NextSettlementField::OldInterval,
            };
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_replay_params(Default::default()),
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
            assert_eq!(returned.err(), Some(expected));
        }
    }
    let result = run_replay_case(
        &build_replay_case_deps(Consumer, Default::default()),
        build_replay_case_params(Default::default()),
        build_replay_case_payload(CANONICAL, &asset_root(), Default::default()),
    );
    assert!(result.is_ok());
}

/// Contract: Next settlement, the `to_field` of `payload.next.seller` differs
///   from that of `payload.transfer.buyer` →
///   `Err(ReplayErrorReturn::NotTheNextSettlement { field:
///   NextSettlementField::Seller })`.
/// Arrange: the next context overriding `seller` with a clone of the transfer
///   context's `seller`, the transfer's seller and not its buyer.
/// Act:     `replay`.
/// Assert:  the result equals `Err(ReplayErrorReturn::NotTheNextSettlement
///   { field: NextSettlementField::Seller })` whole.
#[test]
fn replay_refuses_a_next_context_whose_seller_is_not_the_transfers_buyer() {
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
                    seller: Some(transfer_context.seller.clone()),
                    ..Default::default()
                },
            );
            let expected = ReplayErrorReturn::NotTheNextSettlement {
                field: NextSettlementField::Seller,
            };
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_replay_params(Default::default()),
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
            assert_eq!(returned.err(), Some(expected));
        }
    }
    let result = run_replay_case(
        &build_replay_case_deps(Consumer, Default::default()),
        build_replay_case_params(Default::default()),
        build_replay_case_payload(CANONICAL, &asset_root(), Default::default()),
    );
    assert!(result.is_ok());
}

/// Contract: Next settlement, compared in `NextSettlementField` order, the
///   first unequal pair returns.
/// Arrange: the overrides of the chain block together with the next context
///   overriding `seller` with a clone of the transfer context's `seller`, the
///   chain the earlier in comparison order.
/// Act:     `replay`.
/// Assert:  the result equals `Err(ReplayErrorReturn::NotTheNextSettlement
///   { field: NextSettlementField::Chain })` whole.
#[test]
fn replay_refuses_the_first_differing_next_settlement_field() {
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
                DeliveryContextOverrides {
                    chain: Some(
                        <F::ChainIdentifier as ICanonicalField>::from_field(
                            build_from_field_params(Default::default()),
                            CanonicalFieldValue::Unsigned256([0x21; 32]),
                        )
                        .expect("the value has the kind the form declares")
                        .value,
                    ),
                    ..Default::default()
                },
            );
            let next_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                DeliveryContextOverrides {
                    chain: Some(
                        <F::ChainIdentifier as ICanonicalField>::from_field(
                            build_from_field_params(Default::default()),
                            CanonicalFieldValue::Unsigned256([0x22; 32]),
                        )
                        .expect("the value has the kind the form declares")
                        .value,
                    ),
                    entitlement_contract: Some(transfer_context.entitlement_contract.clone()),
                    asset_root: Some(transfer_context.asset_root),
                    parameter_set_identifier: Some(
                        transfer_context.parameter_set_identifier.clone(),
                    ),
                    source_entitlement: Some(transfer_context.source_entitlement.clone()),
                    target_entitlement: Some(transfer_context.target_entitlement.clone()),
                    old_interval: Some(transfer_context.new_interval.clone()),
                    seller: Some(transfer_context.seller.clone()),
                    ..Default::default()
                },
            );
            let expected = ReplayErrorReturn::NotTheNextSettlement {
                field: NextSettlementField::Chain,
            };
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_replay_params(Default::default()),
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
            assert_eq!(returned.err(), Some(expected));
        }
    }
    let result = run_replay_case(
        &build_replay_case_deps(Consumer, Default::default()),
        build_replay_case_params(Default::default()),
        build_replay_case_payload(CANONICAL, &asset_root(), Default::default()),
    );
    assert!(result.is_ok());
}

/// Contract: Next settlement, decided before any record's mint delivery.
/// Arrange: the next context overriding `old_interval` with a clone of the
///   transfer context's `old_interval`, and `build_create_random_source_params`
///   overriding `kind` with
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)`
///   for the subject's source, a refused next context and a refusing source
///   together.
/// Act:     `replay`.
/// Assert:  the result equals `Err(ReplayErrorReturn::NotTheNextSettlement
///   { field: NextSettlementField::OldInterval })` whole.
#[test]
fn replay_refuses_the_next_settlement_before_any_delivery() {
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
                    old_interval: Some(transfer_context.old_interval.clone()),
                    seller: Some(transfer_context.buyer.clone()),
                    ..Default::default()
                },
            );
            let expected = ReplayErrorReturn::NotTheNextSettlement {
                field: NextSettlementField::OldInterval,
            };
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_replay_params(Default::default()),
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
            assert_eq!(returned.err(), Some(expected));
        }
    }
    let result = run_replay_case(
        &build_replay_case_deps(Consumer, Default::default()),
        build_replay_case_params(ReplayCaseParamsOverrides {
            random_source: Some(build_create_random_source_params(
                CreateRandomSourceParamsOverrides {
                    kind: Some(RandomSourceKind::Mock(
                        MockIRandomSourceAdapterFailureMode::FillBytesRefused,
                    )),
                },
            )),
            ..Default::default()
        }),
        build_replay_case_payload(CANONICAL, &asset_root(), Default::default()),
    );
    assert!(result.is_ok());
}

/// Contract: Mint delivery, `mint` fails at its first draw →
///   `Err(ReplayErrorReturn::Mint(error))`, `error` unchanged.
/// Arrange: `build_create_random_source_params` overriding `kind` with
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)`
///   for the subject's source and the independent source; the expected `error`
///   `MintErrorReturn::FillBytes` of the refusal of the independent source's
///   `fill_bytes`.
/// Act:     `replay`.
/// Assert:  the result equals `Err(ReplayErrorReturn::Mint(error))` whole.
#[test]
fn replay_returns_the_draw_refusal_of_the_mint_delivery_whole() {
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
            let error = independent
                .fill_bytes(
                    FillBytesParams,
                    build_fill_bytes_payload(FillBytesPayloadOverrides {
                        length: Some(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH),
                    }),
                )
                .err()
                .expect("the refusing source refuses the draw");
            let expected = ReplayErrorReturn::Mint(MintErrorReturn::FillBytes(error));
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_replay_params(Default::default()),
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
            assert_eq!(returned.err(), Some(expected));
        }
    }
    let result = run_replay_case(
        &build_replay_case_deps(Consumer, Default::default()),
        build_replay_case_params(ReplayCaseParamsOverrides {
            random_source: Some(build_create_random_source_params(
                CreateRandomSourceParamsOverrides {
                    kind: Some(RandomSourceKind::Mock(
                        MockIRandomSourceAdapterFailureMode::FillBytesRefused,
                    )),
                },
            )),
            ..Default::default()
        }),
        build_replay_case_payload(CANONICAL, &asset_root(), Default::default()),
    );
    assert!(result.is_ok());
}

/// Contract: Mint delivery, `mint` fails at setup's master sampling →
///   `Err(ReplayErrorReturn::Mint(error))`, `error` unchanged.
/// Arrange: `build_create_pairing_params` overriding `concrete` with
///   `PairingConcrete::Mock(MockIPairingAdapterFailureMode::SampleScalarWrongLength)`;
///   the expected `error`
///   `MintErrorReturn::Setup(SetupErrorReturn::MasterScalarSampling(…))` of the
///   `Err` of `<P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes`
///   over the independent source's first draw.
/// Act:     `replay`.
/// Assert:  the result equals `Err(ReplayErrorReturn::Mint(error))` whole.
#[test]
fn replay_returns_the_sampling_refusal_of_the_mint_delivery_whole() {
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
            let error = <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload {
                    uniform: drawn_uniform::<P>(independent),
                },
            )
            .err()
            .expect("the refusing pairing refuses the sampling");
            let expected = ReplayErrorReturn::Mint(MintErrorReturn::Setup(
                SetupErrorReturn::MasterScalarSampling(error),
            ));
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_replay_params(Default::default()),
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
            assert_eq!(returned.err(), Some(expected));
        }
    }
    let result = run_replay_case(
        &build_replay_case_deps(Consumer, Default::default()),
        build_replay_case_params(ReplayCaseParamsOverrides {
            pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(PairingConcrete::Mock(
                    MockIPairingAdapterFailureMode::SampleScalarWrongLength,
                )),
                ..Default::default()
            })),
            ..Default::default()
        }),
        build_replay_case_payload(CANONICAL, &asset_root(), Default::default()),
    );
    assert!(result.is_ok());
}

/// Contract: Mint delivery, `mint` fails at the identity derivation →
///   `Err(ReplayErrorReturn::Mint(error))`, `error` unchanged.
/// Arrange: the KEM selection under the entitlement scope and
///   `build_create_hash_to_scalar_params` overriding `concrete` with
///   `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::SamplingRefused)`;
///   the expected `error`
///   `MintErrorReturn::DeriveIdentity(DeriveIdentityErrorReturn::HashToScalar(…))`
///   of the `Err` of `hash_to_scalar` over `build_hash_to_scalar_params` and
///   `build_hash_to_scalar_payload` at their defaults on an adapter built under
///   the same mode.
/// Act:     `replay`.
/// Assert:  the result equals `Err(ReplayErrorReturn::Mint(error))` whole.
#[test]
fn replay_returns_the_hash_to_scalar_refusal_of_the_mint_delivery_derivation_whole() {
    struct Consumer;

    impl IReplayCaseConsumer for Consumer {
        type Output = ();

        fn consume_replay_case<P, H, E, F, K, A, D>(
            &self,
            _pairing: &P,
            hash_to_scalar: &H,
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
            let error = hash_to_scalar
                .hash_to_scalar(
                    build_hash_to_scalar_params(Default::default()),
                    build_hash_to_scalar_payload(Default::default()),
                )
                .err()
                .expect("the refusing hash-to-scalar refuses");
            let expected = ReplayErrorReturn::Mint(MintErrorReturn::DeriveIdentity(
                DeriveIdentityErrorReturn::HashToScalar(error),
            ));
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_replay_params(Default::default()),
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
            assert_eq!(returned.err(), Some(expected));
        }
    }
    let result = run_replay_case(
        &build_replay_case_deps(Consumer, Default::default()),
        build_replay_case_params(ReplayCaseParamsOverrides {
            hash_to_scalar: Some(build_create_hash_to_scalar_params(
                CreateHashToScalarParamsOverrides {
                    concrete: Some(HashToScalarConcrete::Mock(
                        MockIHashToScalarAdapterFailureMode::SamplingRefused,
                    )),
                    ..Default::default()
                },
            )),
            ..Default::default()
        }),
        build_replay_case_payload(CANONICAL, &asset_root(), Default::default()),
    );
    assert!(result.is_ok());
}

/// Contract: Mint delivery under an asset identity, `mint` fails at setup on
///   the root → `Err(ReplayErrorReturn::Mint(error))`, `error` unchanged.
/// Arrange: the KEM selection under the asset scope, the root `build_root`
///   builds from `[0x44; 32]`, and the hash-to-scalar selection of the
///   derivation block; the expected `error`
///   `MintErrorReturn::Setup(SetupErrorReturn::HashToScalar(…))` of the
///   derivation block's refusal.
/// Act:     `replay`.
/// Assert:  the result equals `Err(ReplayErrorReturn::Mint(error))` whole.
#[test]
fn replay_returns_the_hash_to_scalar_refusal_of_the_mint_delivery_asset_setup_whole() {
    struct Consumer;

    impl IReplayCaseConsumer for Consumer {
        type Output = ();

        fn consume_replay_case<P, H, E, F, K, A, D>(
            &self,
            _pairing: &P,
            hash_to_scalar: &H,
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
            let error = hash_to_scalar
                .hash_to_scalar(
                    build_hash_to_scalar_params(Default::default()),
                    build_hash_to_scalar_payload(Default::default()),
                )
                .err()
                .expect("the refusing hash-to-scalar refuses");
            let expected = ReplayErrorReturn::Mint(MintErrorReturn::Setup(
                SetupErrorReturn::HashToScalar(error),
            ));
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_replay_params(Default::default()),
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
            assert_eq!(returned.err(), Some(expected));
        }
    }
    let result = run_replay_case(
        &build_replay_case_deps(Consumer, Default::default()),
        build_replay_case_params(ReplayCaseParamsOverrides {
            hash_to_scalar: Some(build_create_hash_to_scalar_params(
                CreateHashToScalarParamsOverrides {
                    concrete: Some(HashToScalarConcrete::Mock(
                        MockIHashToScalarAdapterFailureMode::SamplingRefused,
                    )),
                    ..Default::default()
                },
            )),
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

/// Contract: Mint delivery, `mint` fails at the holder's key generation →
///   `Err(ReplayErrorReturn::Mint(error))`, `error` unchanged.
/// Arrange: `build_create_key_agreement_params` overriding `concrete` with
///   `KeyAgreementConcrete::Mock(MockIKeyAgreementAdapterFailureMode::NonceDrawRefused)`.
/// Act:     `replay`.
/// Assert:  the result equals
///   `Err(ReplayErrorReturn::Mint(MintErrorReturn::GenerateKeys(GenerateKeysErrorReturn::Mock(MockIKeyAgreementAdapterGenerateKeysErrorReturn::NonceDrawRefused))))`
///   whole.
#[test]
fn replay_returns_the_nonce_draw_refusal_of_the_mint_delivery_key_generation_whole() {
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
            let expected = ReplayErrorReturn::Mint(MintErrorReturn::GenerateKeys(
                GenerateKeysErrorReturn::Mock(
                    MockIKeyAgreementAdapterGenerateKeysErrorReturn::NonceDrawRefused,
                ),
            ));
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_replay_params(Default::default()),
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
            assert_eq!(returned.err(), Some(expected));
        }
    }
    let result = run_replay_case(
        &build_replay_case_deps(Consumer, Default::default()),
        build_replay_case_params(ReplayCaseParamsOverrides {
            key_agreement: Some(build_create_key_agreement_params(
                CreateKeyAgreementParamsOverrides {
                    concrete: Some(KeyAgreementConcrete::Mock(
                        MockIKeyAgreementAdapterFailureMode::NonceDrawRefused,
                    )),
                    ..Default::default()
                },
            )),
            ..Default::default()
        }),
        build_replay_case_payload(CANONICAL, &asset_root(), Default::default()),
    );
    assert!(result.is_ok());
}

/// Contract: Mint delivery, `mint` fails at the wrap to the holder's keys →
///   `Err(ReplayErrorReturn::Mint(error))`, `error` unchanged.
/// Arrange: `build_create_key_agreement_params` overriding `concrete` with
///   `KeyAgreementConcrete::Mock(MockIKeyAgreementAdapterFailureMode::CoinDrawRefused)`.
/// Act:     `replay`.
/// Assert:  the result equals
///   `Err(ReplayErrorReturn::Mint(MintErrorReturn::WrapTo(WrapToErrorReturn::Mock(MockIKeyAgreementAdapterWrapToErrorReturn::CoinDrawRefused))))`
///   whole.
#[test]
fn replay_returns_the_coin_draw_refusal_of_the_mint_delivery_wrap_whole() {
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
            let expected = ReplayErrorReturn::Mint(MintErrorReturn::WrapTo(
                WrapToErrorReturn::Mock(MockIKeyAgreementAdapterWrapToErrorReturn::CoinDrawRefused),
            ));
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_replay_params(Default::default()),
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
            assert_eq!(returned.err(), Some(expected));
        }
    }
    let result = run_replay_case(
        &build_replay_case_deps(Consumer, Default::default()),
        build_replay_case_params(ReplayCaseParamsOverrides {
            key_agreement: Some(build_create_key_agreement_params(
                CreateKeyAgreementParamsOverrides {
                    concrete: Some(KeyAgreementConcrete::Mock(
                        MockIKeyAgreementAdapterFailureMode::CoinDrawRefused,
                    )),
                    ..Default::default()
                },
            )),
            ..Default::default()
        }),
        build_replay_case_payload(CANONICAL, &asset_root(), Default::default()),
    );
    assert!(result.is_ok());
}

/// Contract: Mint delivery, `mint` fails at its proof →
///   `Err(ReplayErrorReturn::Mint(error))`, `error` unchanged.
/// Arrange: `build_create_delivery_proof_params` overriding `concrete` with
///   `DeliveryProofConcrete::Mock(MockIDeliveryProofAdapterFailureMode::NonceDrawRefused)`.
/// Act:     `replay`.
/// Assert:  the result equals
///   `Err(ReplayErrorReturn::Mint(MintErrorReturn::ProveMint(ProveMintErrorReturn::Mock(MockIDeliveryProofAdapterProveMintErrorReturn::NonceDrawRefused))))`
///   whole.
#[test]
fn replay_returns_the_nonce_draw_refusal_of_the_mint_delivery_prover_whole() {
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
            let expected =
                ReplayErrorReturn::Mint(MintErrorReturn::ProveMint(ProveMintErrorReturn::Mock(
                    MockIDeliveryProofAdapterProveMintErrorReturn::NonceDrawRefused,
                )));
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_replay_params(Default::default()),
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
            assert_eq!(returned.err(), Some(expected));
        }
    }
    let result = run_replay_case(
        &build_replay_case_deps(Consumer, Default::default()),
        build_replay_case_params(ReplayCaseParamsOverrides {
            delivery_proof: Some(build_create_delivery_proof_params(
                CreateDeliveryProofParamsOverrides {
                    concrete: Some(DeliveryProofConcrete::Mock(
                        MockIDeliveryProofAdapterFailureMode::NonceDrawRefused,
                    )),
                    ..Default::default()
                },
            )),
            ..Default::default()
        }),
        build_replay_case_payload(CANONICAL, &asset_root(), Default::default()),
    );
    assert!(result.is_ok());
}
