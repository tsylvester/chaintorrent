#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_arguments
)]

use core::marker::PhantomData;

use super::interface::{
    AttemptContext, AuthorizationResult, BatchPage, BatchPageLimitParams, BatchPageLimitPayload,
    BatchPageLimitReturn, BatchPageLimitSuccessReturn, CHAIN_FORMS_INTERFACE_VERSION,
    CHAIN_INTERFACE_VERSION, ChainDeclaration, ChainEndpoint, ChainFormsDeclaration,
    ChainFormsIdentifier, ChainIdentifierErrorReturn, ChainIdentifierParams,
    ChainIdentifierPayload, ChainIdentifierReturn, ChainIdentifierSuccessReturn,
    ChainMemberIdentifier, ChainSelection, ChainSettingBounds, ChainSettingDisposition,
    ChainSettingStatement, ConsumeChainParams, ConsumeChainPayload, CreateChainDeps,
    CreateChainParams, CreateChainPayload, CreateChainReturn, CreateChainSuccessReturn,
    CurrentReferenceAtErrorReturn, CurrentReferenceAtParams, CurrentReferenceAtPayload,
    CurrentReferenceAtReturn, CurrentReferenceAtSuccessReturn, DeriveKeyIdentifierParams,
    DeriveKeyIdentifierPayload, DeriveKeyIdentifierReturn, DeriveKeyIdentifierSuccessReturn,
    EndpointParams, EndpointPayload, EndpointReturn, EndpointSuccessReturn,
    EntitlementContractErrorReturn, EntitlementContractParams, EntitlementContractPayload,
    EntitlementContractReturn, EntitlementContractSuccessReturn, EstimateFeeErrorReturn,
    EstimateFeeParams, EstimateFeePayload, EstimateFeeReturn, EstimateFeeSuccessReturn,
    EvaluateAuthorizationBatchErrorReturn, EvaluateAuthorizationBatchParams,
    EvaluateAuthorizationBatchPayload, EvaluateAuthorizationBatchReturn,
    EvaluateAuthorizationBatchSuccessReturn, EvaluateAuthorizationErrorReturn,
    EvaluateAuthorizationParams, EvaluateAuthorizationPayload, EvaluateAuthorizationReturn,
    EvaluateAuthorizationSuccessReturn, FeeDescription, FeeEntry, FeeEntryDeclaration, FeeQuantity,
    FeeRequest, IChainAdapter, IChainConsumer, IChainForms, IEntitlementStateAdapter,
    IIdentityForm, ISettlementAdapter, IntentMemberNames,
    MockIChainAdapterCurrentReferenceAtErrorReturn, MockIChainAdapterEstimateFeeErrorReturn,
    MockIChainAdapterEvaluateAuthorizationBatchErrorReturn,
    MockIChainAdapterEvaluateAuthorizationErrorReturn, MockIChainAdapterFailureMode,
    MockIChainAdapterRecoverEnvelopeErrorReturn, MockIChainAdapterReferenceAgeErrorReturn,
    MockIChainAdapterTierOfErrorReturn, RecoverEnvelopeErrorReturn, RecoverEnvelopeParams,
    RecoverEnvelopePayload, RecoverEnvelopeReturn, RecoverEnvelopeSuccessReturn, RecoveredEnvelope,
    ReferenceAge, ReferenceAgeErrorReturn, ReferenceAgeParams, ReferenceAgePayload,
    ReferenceAgeReturn, ReferenceAgeSuccessReturn, SettlementReference,
    SettlementReferenceConstructorParams, SettlementTier, SignedIntentParams, SignedIntentPayload,
    SignedIntentReturn, SignedIntentSuccessReturn, TierOfErrorReturn, TierOfParams, TierOfPayload,
    TierOfReturn, TierOfSuccessReturn,
};
use crate::admit_chain_composition::provides::AdmitChainCompositionFn;
use crate::base::provides::{
    BaseChainIdentifierErrorReturn, BaseEntitlementContractErrorReturn, build_base_chain_settings,
};
use domain::{
    DeploymentIdentity, HandshakeKeyCommitment, build_deployment_identity,
    build_handshake_key_commitment,
};
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, EncodingIdentifier, FromFieldErrorReturn,
    FromFieldParams, FromFieldReturn, FromFieldSuccessReturnOverrides, ICanonicalField,
    ToFieldParams, ToFieldReturn, ToFieldSuccessReturnOverrides, build_from_field_success_return,
    build_to_field_success_return,
};
use hashing::{Root, build_root};
use pairing::PrecompileEncoding;

#[derive(Default)]
pub struct ChainFormsDeclarationOverrides {
    pub identifier: Option<ChainFormsIdentifier>,
    pub adapter_version: Option<u32>,
    pub interface_version: Option<u32>,
    pub encoding_identifier: Option<EncodingIdentifier>,
    pub key_identifier_domain_tag: Option<&'static [u8]>,
    pub intent_domain_name: Option<&'static str>,
    pub intent_domain_version: Option<&'static str>,
    pub intent_struct_name: Option<&'static str>,
    pub intent_member_names: Option<IntentMemberNames>,
    pub nonce_kind: Option<CanonicalFieldKind>,
    pub expiry_kind: Option<CanonicalFieldKind>,
}

pub fn build_chain_forms_declaration(
    overrides: ChainFormsDeclarationOverrides,
) -> ChainFormsDeclaration {
    ChainFormsDeclaration {
        identifier: overrides.identifier.unwrap_or(ChainFormsIdentifier::EvmV1),
        adapter_version: overrides.adapter_version.unwrap_or(1),
        interface_version: overrides
            .interface_version
            .unwrap_or(CHAIN_FORMS_INTERFACE_VERSION),
        encoding_identifier: overrides
            .encoding_identifier
            .unwrap_or(EncodingIdentifier::EthereumAbiV1),
        key_identifier_domain_tag: overrides
            .key_identifier_domain_tag
            .unwrap_or(b"chaintorrent/evm/key-identifier/v1"),
        intent_domain_name: overrides.intent_domain_name.unwrap_or("ChainTorrent"),
        intent_domain_version: overrides.intent_domain_version.unwrap_or("1"),
        intent_struct_name: overrides.intent_struct_name.unwrap_or("Intent"),
        intent_member_names: overrides.intent_member_names.unwrap_or(IntentMemberNames {
            identity: "identity",
            nonce: "nonce",
            expiry: "expiry",
            call: "call",
        }),
        nonce_kind: overrides
            .nonce_kind
            .unwrap_or(CanonicalFieldKind::Unsigned64),
        expiry_kind: overrides
            .expiry_kind
            .unwrap_or(CanonicalFieldKind::Unsigned64),
    }
}

#[derive(Default)]
pub struct ChainEndpointOverrides {
    pub text: Option<String>,
}

pub fn build_chain_endpoint(overrides: ChainEndpointOverrides) -> ChainEndpoint {
    ChainEndpoint {
        text: overrides
            .text
            .unwrap_or_else(|| "https://mainnet.base.org".to_string()),
    }
}

#[derive(Default)]
pub struct FeeEntryDeclarationOverrides {
    pub name: Option<&'static str>,
    pub unit: Option<&'static str>,
}

pub fn build_fee_entry_declaration(overrides: FeeEntryDeclarationOverrides) -> FeeEntryDeclaration {
    FeeEntryDeclaration {
        name: overrides.name.unwrap_or("execution_gas"),
        unit: overrides.unit.unwrap_or("gas"),
    }
}

#[derive(Default)]
pub struct ChainSettingStatementOverrides {
    pub name: Option<&'static str>,
    pub disposition: Option<ChainSettingDisposition>,
}

pub fn build_chain_setting_statement(
    overrides: ChainSettingStatementOverrides,
) -> ChainSettingStatement {
    ChainSettingStatement {
        name: overrides.name.unwrap_or("endpoint"),
        disposition: overrides
            .disposition
            .unwrap_or(ChainSettingDisposition::Settable {
                bounds: ChainSettingBounds::Unbounded,
                default: "https://mainnet.base.org",
            }),
    }
}

#[derive(Default)]
pub struct ChainDeclarationOverrides {
    pub identifier: Option<ChainMemberIdentifier>,
    pub name: Option<&'static str>,
    pub adapter_version: Option<u32>,
    pub interface_version: Option<u32>,
    pub forms_identifier: Option<ChainFormsIdentifier>,
    pub precompile_encodings: Option<&'static [PrecompileEncoding]>,
    pub fee_entries: Option<&'static [FeeEntryDeclaration]>,
    pub settings: Option<&'static [ChainSettingStatement]>,
}

pub fn build_chain_declaration(overrides: ChainDeclarationOverrides) -> ChainDeclaration {
    ChainDeclaration {
        identifier: overrides.identifier.unwrap_or(ChainMemberIdentifier::Base),
        name: overrides.name.unwrap_or("Base"),
        adapter_version: overrides.adapter_version.unwrap_or(1),
        interface_version: overrides
            .interface_version
            .unwrap_or(CHAIN_INTERFACE_VERSION),
        forms_identifier: overrides
            .forms_identifier
            .unwrap_or(ChainFormsIdentifier::EvmV1),
        precompile_encodings: overrides.precompile_encodings.unwrap_or(&[]),
        fee_entries: overrides.fee_entries.unwrap_or(&[]),
        settings: overrides.settings.unwrap_or(&[]),
    }
}

pub(crate) const MOCK_PREFIXED_CHAIN_IDENTIFIER_SETTING_STATEMENTS: &[ChainSettingStatement] = &[
    ChainSettingStatement {
        name: "endpoint",
        disposition: ChainSettingDisposition::Settable {
            bounds: ChainSettingBounds::Unbounded,
            default: "https://mainnet.base.org",
        },
    },
    ChainSettingStatement {
        name: "settlement_endpoint",
        disposition: ChainSettingDisposition::Settable {
            bounds: ChainSettingBounds::TextPrefixedBy {
                prefixes: &["http://", "https://"],
            },
            default: "https://ethereum-rpc.publicnode.com",
        },
    },
    ChainSettingStatement {
        name: "chain_identifier",
        disposition: ChainSettingDisposition::Settable {
            bounds: ChainSettingBounds::TextPrefixedBy { prefixes: &["8"] },
            default: "8453",
        },
    },
    ChainSettingStatement {
        name: "registry",
        disposition: ChainSettingDisposition::Settable {
            bounds: ChainSettingBounds::Unbounded,
            default: "0xdddddddddddddddddddddddddddddddddddddddd",
        },
    },
    ChainSettingStatement {
        name: "entitlement_contract",
        disposition: ChainSettingDisposition::Settable {
            bounds: ChainSettingBounds::Unbounded,
            default: "0x0000000000000000000000000000000000000000",
        },
    },
    ChainSettingStatement {
        name: "gas_price_oracle",
        disposition: ChainSettingDisposition::Fixed {
            value: "0x420000000000000000000000000000000000000f",
        },
    },
    ChainSettingStatement {
        name: "anchor_state_registry",
        disposition: ChainSettingDisposition::Settable {
            bounds: ChainSettingBounds::Unbounded,
            default: "0x909f6cf47ed12f010a796527f562bfc26c7f4e72",
        },
    },
    ChainSettingStatement {
        name: "batch_page_limit",
        disposition: ChainSettingDisposition::Settable {
            bounds: ChainSettingBounds::Unbounded,
            default: "64",
        },
    },
];

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct MockIChainForms;

impl IChainForms for MockIChainForms {
    const DECLARATION: ChainFormsDeclaration = ChainFormsDeclaration {
        identifier: ChainFormsIdentifier::EvmV1,
        adapter_version: 1,
        interface_version: CHAIN_FORMS_INTERFACE_VERSION,
        encoding_identifier: EncodingIdentifier::Mock,
        key_identifier_domain_tag: b"chaintorrent/evm/key-identifier/v1",
        intent_domain_name: "ChainTorrent",
        intent_domain_version: "1",
        intent_struct_name: "Intent",
        intent_member_names: IntentMemberNames {
            identity: "identity",
            nonce: "nonce",
            expiry: "expiry",
            call: "call",
        },
        nonce_kind: CanonicalFieldKind::Unsigned64,
        expiry_kind: CanonicalFieldKind::Unsigned64,
    };

    type Identity = MockIdentity;
    type Entitlement = MockEntitlement;
    type Interval = MockInterval;
    type ChainIdentifier = MockChainIdentifier;
    type SignedIntent = MockSignedIntent;

    fn signed_intent(
        &self,
        _params: SignedIntentParams,
        _payload: SignedIntentPayload<'_, Self>,
    ) -> SignedIntentReturn<Self::SignedIntent> {
        Ok(SignedIntentSuccessReturn {
            signed_intent: build_mock_signed_intent(Default::default()),
        })
    }
}

#[derive(Default)]
pub(crate) struct MockIChainFormsOverrides;

pub(crate) fn build_mock_i_chain_forms(_overrides: MockIChainFormsOverrides) -> MockIChainForms {
    MockIChainForms
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct MockIdentity {
    value: [u8; 20],
}

impl ICanonicalField for MockIdentity {
    const KIND: CanonicalFieldKind = CanonicalFieldKind::FixedBytes20;

    fn to_field(_params: ToFieldParams, payload: &Self) -> ToFieldReturn {
        Ok(build_to_field_success_return(
            ToFieldSuccessReturnOverrides {
                field: Some(CanonicalFieldValue::FixedBytes20(payload.value)),
            },
        ))
    }

    fn from_field(_params: FromFieldParams, payload: CanonicalFieldValue) -> FromFieldReturn<Self> {
        let CanonicalFieldValue::FixedBytes20(value) = payload else {
            return Err(FromFieldErrorReturn::WrongKind {
                expected: CanonicalFieldKind::FixedBytes20,
            });
        };
        Ok(build_from_field_success_return(
            build_mock_identity(MockIdentityOverrides { value: Some(value) }),
            FromFieldSuccessReturnOverrides,
        ))
    }
}

impl IIdentityForm for MockIdentity {
    type KeyIdentifier = MockKeyIdentifier;

    fn derive_key_identifier(
        _params: DeriveKeyIdentifierParams,
        _payload: DeriveKeyIdentifierPayload<'_>,
    ) -> DeriveKeyIdentifierReturn<Self::KeyIdentifier> {
        Ok(DeriveKeyIdentifierSuccessReturn {
            key_identifier: build_mock_key_identifier(Default::default()),
        })
    }
}

#[derive(Default)]
pub(crate) struct MockIdentityOverrides {
    pub(crate) value: Option<[u8; 20]>,
}

pub(crate) fn build_mock_identity(overrides: MockIdentityOverrides) -> MockIdentity {
    MockIdentity {
        value: overrides.value.unwrap_or([0x01; 20]),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct MockEntitlement {
    value: [u8; 32],
}

impl ICanonicalField for MockEntitlement {
    const KIND: CanonicalFieldKind = CanonicalFieldKind::Unsigned256;

    fn to_field(_params: ToFieldParams, payload: &Self) -> ToFieldReturn {
        Ok(build_to_field_success_return(
            ToFieldSuccessReturnOverrides {
                field: Some(CanonicalFieldValue::Unsigned256(payload.value)),
            },
        ))
    }

    fn from_field(_params: FromFieldParams, payload: CanonicalFieldValue) -> FromFieldReturn<Self> {
        let CanonicalFieldValue::Unsigned256(value) = payload else {
            return Err(FromFieldErrorReturn::WrongKind {
                expected: CanonicalFieldKind::Unsigned256,
            });
        };
        Ok(build_from_field_success_return(
            build_mock_entitlement(MockEntitlementOverrides { value: Some(value) }),
            FromFieldSuccessReturnOverrides,
        ))
    }
}

#[derive(Default)]
pub(crate) struct MockEntitlementOverrides {
    pub(crate) value: Option<[u8; 32]>,
}

pub(crate) fn build_mock_entitlement(overrides: MockEntitlementOverrides) -> MockEntitlement {
    MockEntitlement {
        value: overrides.value.unwrap_or([0x01; 32]),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct MockInterval {
    value: u64,
}

impl ICanonicalField for MockInterval {
    const KIND: CanonicalFieldKind = CanonicalFieldKind::Unsigned64;

    fn to_field(_params: ToFieldParams, payload: &Self) -> ToFieldReturn {
        Ok(build_to_field_success_return(
            ToFieldSuccessReturnOverrides {
                field: Some(CanonicalFieldValue::Unsigned64(payload.value)),
            },
        ))
    }

    fn from_field(_params: FromFieldParams, payload: CanonicalFieldValue) -> FromFieldReturn<Self> {
        let CanonicalFieldValue::Unsigned64(value) = payload else {
            return Err(FromFieldErrorReturn::WrongKind {
                expected: CanonicalFieldKind::Unsigned64,
            });
        };
        Ok(build_from_field_success_return(
            build_mock_interval(MockIntervalOverrides { value: Some(value) }),
            FromFieldSuccessReturnOverrides,
        ))
    }
}

#[derive(Default)]
pub(crate) struct MockIntervalOverrides {
    pub(crate) value: Option<u64>,
}

pub(crate) fn build_mock_interval(overrides: MockIntervalOverrides) -> MockInterval {
    MockInterval {
        value: overrides.value.unwrap_or(1),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct MockChainIdentifier {
    value: [u8; 32],
}

impl ICanonicalField for MockChainIdentifier {
    const KIND: CanonicalFieldKind = CanonicalFieldKind::Unsigned256;

    fn to_field(_params: ToFieldParams, payload: &Self) -> ToFieldReturn {
        Ok(build_to_field_success_return(
            ToFieldSuccessReturnOverrides {
                field: Some(CanonicalFieldValue::Unsigned256(payload.value)),
            },
        ))
    }

    fn from_field(_params: FromFieldParams, payload: CanonicalFieldValue) -> FromFieldReturn<Self> {
        let CanonicalFieldValue::Unsigned256(value) = payload else {
            return Err(FromFieldErrorReturn::WrongKind {
                expected: CanonicalFieldKind::Unsigned256,
            });
        };
        Ok(build_from_field_success_return(
            build_mock_chain_identifier(MockChainIdentifierOverrides { value: Some(value) }),
            FromFieldSuccessReturnOverrides,
        ))
    }
}

#[derive(Default)]
pub(crate) struct MockChainIdentifierOverrides {
    pub(crate) value: Option<[u8; 32]>,
}

pub(crate) fn build_mock_chain_identifier(
    overrides: MockChainIdentifierOverrides,
) -> MockChainIdentifier {
    MockChainIdentifier {
        value: overrides.value.unwrap_or([0x01; 32]),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct MockKeyIdentifier {
    value: [u8; 32],
}

impl ICanonicalField for MockKeyIdentifier {
    const KIND: CanonicalFieldKind = CanonicalFieldKind::FixedBytes32;

    fn to_field(_params: ToFieldParams, payload: &Self) -> ToFieldReturn {
        Ok(build_to_field_success_return(
            ToFieldSuccessReturnOverrides {
                field: Some(CanonicalFieldValue::FixedBytes32(payload.value)),
            },
        ))
    }

    fn from_field(_params: FromFieldParams, payload: CanonicalFieldValue) -> FromFieldReturn<Self> {
        let CanonicalFieldValue::FixedBytes32(value) = payload else {
            return Err(FromFieldErrorReturn::WrongKind {
                expected: CanonicalFieldKind::FixedBytes32,
            });
        };
        Ok(build_from_field_success_return(
            build_mock_key_identifier(MockKeyIdentifierOverrides { value: Some(value) }),
            FromFieldSuccessReturnOverrides,
        ))
    }
}

#[derive(Default)]
pub(crate) struct MockKeyIdentifierOverrides {
    pub(crate) value: Option<[u8; 32]>,
}

pub(crate) fn build_mock_key_identifier(
    overrides: MockKeyIdentifierOverrides,
) -> MockKeyIdentifier {
    MockKeyIdentifier {
        value: overrides.value.unwrap_or([0x01; 32]),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct MockSignedIntent {
    value: [u8; 32],
}

impl ICanonicalField for MockSignedIntent {
    const KIND: CanonicalFieldKind = CanonicalFieldKind::FixedBytes32;

    fn to_field(_params: ToFieldParams, payload: &Self) -> ToFieldReturn {
        Ok(build_to_field_success_return(
            ToFieldSuccessReturnOverrides {
                field: Some(CanonicalFieldValue::FixedBytes32(payload.value)),
            },
        ))
    }

    fn from_field(_params: FromFieldParams, payload: CanonicalFieldValue) -> FromFieldReturn<Self> {
        let CanonicalFieldValue::FixedBytes32(value) = payload else {
            return Err(FromFieldErrorReturn::WrongKind {
                expected: CanonicalFieldKind::FixedBytes32,
            });
        };
        Ok(build_from_field_success_return(
            build_mock_signed_intent(MockSignedIntentOverrides { value: Some(value) }),
            FromFieldSuccessReturnOverrides,
        ))
    }
}

#[derive(Default)]
pub(crate) struct MockSignedIntentOverrides {
    pub(crate) value: Option<[u8; 32]>,
}

pub(crate) fn build_mock_signed_intent(overrides: MockSignedIntentOverrides) -> MockSignedIntent {
    MockSignedIntent {
        value: overrides.value.unwrap_or([0x01; 32]),
    }
}

#[derive(Default)]
pub struct SettlementReferenceConstructorParamsOverrides {
    pub ordinal: Option<u64>,
    pub identity: Option<[u8; 32]>,
}

pub fn build_settlement_reference_constructor_params(
    overrides: SettlementReferenceConstructorParamsOverrides,
) -> SettlementReferenceConstructorParams {
    SettlementReferenceConstructorParams {
        ordinal: overrides.ordinal.unwrap_or(10),
        identity: overrides.identity.unwrap_or([0x21; 32]),
    }
}

pub fn build_settlement_reference(
    overrides: SettlementReferenceConstructorParamsOverrides,
) -> SettlementReference {
    SettlementReference::try_new(build_settlement_reference_constructor_params(overrides))
        .expect("built settlement reference constructor params are admitted")
}

#[derive(Default)]
pub struct ReferenceAgeOverrides {
    pub seconds: Option<u64>,
}

pub fn build_reference_age(overrides: ReferenceAgeOverrides) -> ReferenceAge {
    ReferenceAge {
        seconds: overrides.seconds.unwrap_or(4),
    }
}

#[derive(Default)]
pub struct FeeQuantityOverrides {
    pub value: Option<u128>,
}

pub fn build_fee_quantity(overrides: FeeQuantityOverrides) -> FeeQuantity {
    FeeQuantity {
        value: overrides.value.unwrap_or(7),
    }
}

#[derive(Default)]
pub struct FeeEntryOverrides {
    pub name: Option<&'static str>,
    pub unit: Option<&'static str>,
    pub quantity: Option<FeeQuantity>,
}

pub fn build_fee_entry(overrides: FeeEntryOverrides) -> FeeEntry {
    FeeEntry {
        name: overrides.name.unwrap_or("execution_gas"),
        unit: overrides.unit.unwrap_or("gas"),
        quantity: overrides
            .quantity
            .unwrap_or_else(|| build_fee_quantity(Default::default())),
    }
}

#[derive(Default)]
pub struct FeeDescriptionOverrides {
    pub entries: Option<Vec<FeeEntry>>,
}

pub fn build_fee_description(overrides: FeeDescriptionOverrides) -> FeeDescription {
    FeeDescription {
        entries: overrides
            .entries
            .unwrap_or_else(|| vec![build_fee_entry(Default::default())]),
    }
}

#[derive(Default)]
pub struct RecoveredEnvelopeOverrides {
    pub envelope: Option<Vec<u8>>,
    pub transaction: Option<[u8; 32]>,
}

pub fn build_recovered_envelope(overrides: RecoveredEnvelopeOverrides) -> RecoveredEnvelope {
    RecoveredEnvelope {
        envelope: overrides.envelope.unwrap_or_else(|| vec![0x77; 4]),
        transaction: overrides.transaction.unwrap_or([0x78; 32]),
    }
}

#[derive(Default)]
pub struct BatchPageOverrides {
    pub index: Option<u64>,
}

pub fn build_batch_page(overrides: BatchPageOverrides) -> BatchPage {
    BatchPage {
        index: overrides.index.unwrap_or(0),
    }
}

#[derive(Default)]
pub struct CurrentReferenceAtParamsOverrides {
    pub tier: Option<SettlementTier>,
}

pub fn build_current_reference_at_params(
    overrides: CurrentReferenceAtParamsOverrides,
) -> CurrentReferenceAtParams {
    CurrentReferenceAtParams {
        tier: overrides.tier.unwrap_or(SettlementTier::Included),
    }
}

#[derive(Default)]
pub struct CurrentReferenceAtPayloadOverrides;

pub fn build_current_reference_at_payload(
    _overrides: CurrentReferenceAtPayloadOverrides,
) -> CurrentReferenceAtPayload {
    CurrentReferenceAtPayload
}

#[derive(Default)]
pub struct CurrentReferenceAtSuccessReturnOverrides {
    pub reference: Option<SettlementReference>,
}

pub fn build_current_reference_at_success_return(
    overrides: CurrentReferenceAtSuccessReturnOverrides,
) -> CurrentReferenceAtSuccessReturn {
    CurrentReferenceAtSuccessReturn {
        reference: overrides
            .reference
            .unwrap_or_else(|| build_settlement_reference(Default::default())),
    }
}

#[derive(Default)]
pub struct TierOfParamsOverrides;

pub fn build_tier_of_params(_overrides: TierOfParamsOverrides) -> TierOfParams {
    TierOfParams
}

#[derive(Default)]
pub struct TierOfPayloadOverrides;

pub fn build_tier_of_payload<'a>(
    reference: &'a SettlementReference,
    _overrides: TierOfPayloadOverrides,
) -> TierOfPayload<'a> {
    TierOfPayload { reference }
}

#[derive(Default)]
pub struct TierOfSuccessReturnOverrides {
    pub tier: Option<SettlementTier>,
}

pub fn build_tier_of_success_return(
    overrides: TierOfSuccessReturnOverrides,
) -> TierOfSuccessReturn {
    TierOfSuccessReturn {
        tier: overrides.tier.unwrap_or(SettlementTier::Soft),
    }
}

#[derive(Default)]
pub struct ReferenceAgeParamsOverrides;

pub fn build_reference_age_params(_overrides: ReferenceAgeParamsOverrides) -> ReferenceAgeParams {
    ReferenceAgeParams
}

#[derive(Default)]
pub struct ReferenceAgePayloadOverrides;

pub fn build_reference_age_payload<'a>(
    reference: &'a SettlementReference,
    _overrides: ReferenceAgePayloadOverrides,
) -> ReferenceAgePayload<'a> {
    ReferenceAgePayload { reference }
}

#[derive(Default)]
pub struct ReferenceAgeSuccessReturnOverrides {
    pub age: Option<ReferenceAge>,
}

pub fn build_reference_age_success_return(
    overrides: ReferenceAgeSuccessReturnOverrides,
) -> ReferenceAgeSuccessReturn {
    ReferenceAgeSuccessReturn {
        age: overrides
            .age
            .unwrap_or_else(|| build_reference_age(Default::default())),
    }
}

#[derive(Default)]
pub struct EvaluateAuthorizationParamsOverrides;

pub fn build_evaluate_authorization_params(
    _overrides: EvaluateAuthorizationParamsOverrides,
) -> EvaluateAuthorizationParams {
    EvaluateAuthorizationParams
}

pub struct EvaluateAuthorizationPayloadOverrides<F> {
    pub forms: PhantomData<F>,
}

impl<F> Default for EvaluateAuthorizationPayloadOverrides<F> {
    fn default() -> Self {
        Self { forms: PhantomData }
    }
}

pub fn build_evaluate_authorization_payload<'a, F: IChainForms>(
    context: &'a AttemptContext<F>,
    _overrides: EvaluateAuthorizationPayloadOverrides<F>,
) -> EvaluateAuthorizationPayload<'a, F> {
    EvaluateAuthorizationPayload { context }
}

#[derive(Default)]
pub struct EvaluateAuthorizationSuccessReturnOverrides {
    pub result: Option<AuthorizationResult>,
}

pub fn build_evaluate_authorization_success_return(
    overrides: EvaluateAuthorizationSuccessReturnOverrides,
) -> EvaluateAuthorizationSuccessReturn {
    EvaluateAuthorizationSuccessReturn {
        result: overrides.result.unwrap_or(AuthorizationResult::Authorized),
    }
}

#[derive(Default)]
pub struct EvaluateAuthorizationBatchParamsOverrides {
    pub page: Option<BatchPage>,
}

pub fn build_evaluate_authorization_batch_params(
    overrides: EvaluateAuthorizationBatchParamsOverrides,
) -> EvaluateAuthorizationBatchParams {
    EvaluateAuthorizationBatchParams {
        page: overrides
            .page
            .unwrap_or_else(|| build_batch_page(Default::default())),
    }
}

pub struct EvaluateAuthorizationBatchPayloadOverrides<F> {
    pub forms: PhantomData<F>,
}

impl<F> Default for EvaluateAuthorizationBatchPayloadOverrides<F> {
    fn default() -> Self {
        Self { forms: PhantomData }
    }
}

pub fn build_evaluate_authorization_batch_payload<'a, F: IChainForms>(
    contexts: &'a [AttemptContext<F>],
    _overrides: EvaluateAuthorizationBatchPayloadOverrides<F>,
) -> EvaluateAuthorizationBatchPayload<'a, F> {
    EvaluateAuthorizationBatchPayload { contexts }
}

#[derive(Default)]
pub struct EvaluateAuthorizationBatchSuccessReturnOverrides {
    pub results: Option<Vec<AuthorizationResult>>,
    pub next: Option<Option<BatchPage>>,
}

pub fn build_evaluate_authorization_batch_success_return(
    overrides: EvaluateAuthorizationBatchSuccessReturnOverrides,
) -> EvaluateAuthorizationBatchSuccessReturn {
    EvaluateAuthorizationBatchSuccessReturn {
        results: overrides
            .results
            .unwrap_or_else(|| vec![AuthorizationResult::Authorized]),
        next: overrides.next.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct BatchPageLimitParamsOverrides;

pub fn build_batch_page_limit_params(
    _overrides: BatchPageLimitParamsOverrides,
) -> BatchPageLimitParams {
    BatchPageLimitParams
}

#[derive(Default)]
pub struct BatchPageLimitPayloadOverrides;

pub fn build_batch_page_limit_payload(
    _overrides: BatchPageLimitPayloadOverrides,
) -> BatchPageLimitPayload {
    BatchPageLimitPayload
}

#[derive(Default)]
pub struct BatchPageLimitSuccessReturnOverrides {
    pub limit: Option<u64>,
}

pub fn build_batch_page_limit_success_return(
    overrides: BatchPageLimitSuccessReturnOverrides,
) -> BatchPageLimitSuccessReturn {
    BatchPageLimitSuccessReturn {
        limit: overrides.limit.unwrap_or(64),
    }
}

#[derive(Default)]
pub struct EndpointParamsOverrides;

pub fn build_endpoint_params(_overrides: EndpointParamsOverrides) -> EndpointParams {
    EndpointParams
}

#[derive(Default)]
pub struct EndpointPayloadOverrides;

pub fn build_endpoint_payload(_overrides: EndpointPayloadOverrides) -> EndpointPayload {
    EndpointPayload
}

#[derive(Default)]
pub struct EndpointSuccessReturnOverrides {
    pub endpoint: Option<ChainEndpoint>,
}

pub fn build_endpoint_success_return(
    overrides: EndpointSuccessReturnOverrides,
) -> EndpointSuccessReturn {
    EndpointSuccessReturn {
        endpoint: overrides
            .endpoint
            .unwrap_or_else(|| build_chain_endpoint(Default::default())),
    }
}

#[derive(Default)]
pub struct ChainIdentifierParamsOverrides;

pub fn build_chain_identifier_params(
    _overrides: ChainIdentifierParamsOverrides,
) -> ChainIdentifierParams {
    ChainIdentifierParams
}

#[derive(Default)]
pub struct ChainIdentifierPayloadOverrides;

pub fn build_chain_identifier_payload(
    _overrides: ChainIdentifierPayloadOverrides,
) -> ChainIdentifierPayload {
    ChainIdentifierPayload
}

#[derive(Default)]
pub struct ChainIdentifierSuccessReturnOverrides;

pub fn build_chain_identifier_success_return<I>(
    chain_identifier: I,
    _overrides: ChainIdentifierSuccessReturnOverrides,
) -> ChainIdentifierSuccessReturn<I> {
    ChainIdentifierSuccessReturn { chain_identifier }
}

#[derive(Default)]
pub struct EntitlementContractParamsOverrides;

pub fn build_entitlement_contract_params(
    _overrides: EntitlementContractParamsOverrides,
) -> EntitlementContractParams {
    EntitlementContractParams
}

#[derive(Default)]
pub struct EntitlementContractPayloadOverrides;

pub fn build_entitlement_contract_payload(
    _overrides: EntitlementContractPayloadOverrides,
) -> EntitlementContractPayload {
    EntitlementContractPayload
}

#[derive(Default)]
pub struct EntitlementContractSuccessReturnOverrides;

pub fn build_entitlement_contract_success_return<I>(
    contract: I,
    _overrides: EntitlementContractSuccessReturnOverrides,
) -> EntitlementContractSuccessReturn<I> {
    EntitlementContractSuccessReturn { contract }
}

#[derive(Default)]
pub struct EstimateFeeParamsOverrides;

pub fn build_estimate_fee_params(_overrides: EstimateFeeParamsOverrides) -> EstimateFeeParams {
    EstimateFeeParams
}

pub struct EstimateFeePayloadOverrides<F> {
    pub forms: PhantomData<F>,
}

impl<F> Default for EstimateFeePayloadOverrides<F> {
    fn default() -> Self {
        Self { forms: PhantomData }
    }
}

pub fn build_estimate_fee_payload<'a, F: IChainForms>(
    request: &'a FeeRequest<F>,
    _overrides: EstimateFeePayloadOverrides<F>,
) -> EstimateFeePayload<'a, F> {
    EstimateFeePayload { request }
}

#[derive(Default)]
pub struct EstimateFeeSuccessReturnOverrides {
    pub description: Option<FeeDescription>,
}

pub fn build_estimate_fee_success_return(
    overrides: EstimateFeeSuccessReturnOverrides,
) -> EstimateFeeSuccessReturn {
    EstimateFeeSuccessReturn {
        description: overrides
            .description
            .unwrap_or_else(|| build_fee_description(Default::default())),
    }
}

#[derive(Default)]
pub struct RecoverEnvelopeParamsOverrides;

pub fn build_recover_envelope_params(
    _overrides: RecoverEnvelopeParamsOverrides,
) -> RecoverEnvelopeParams {
    RecoverEnvelopeParams
}

pub struct RecoverEnvelopePayloadOverrides<F> {
    pub forms: PhantomData<F>,
}

impl<F> Default for RecoverEnvelopePayloadOverrides<F> {
    fn default() -> Self {
        Self { forms: PhantomData }
    }
}

pub fn build_recover_envelope_payload<'a, F: IChainForms>(
    entitlement: &'a F::Entitlement,
    interval: &'a F::Interval,
    _overrides: RecoverEnvelopePayloadOverrides<F>,
) -> RecoverEnvelopePayload<'a, F> {
    RecoverEnvelopePayload {
        entitlement,
        interval,
    }
}

#[derive(Default)]
pub struct RecoverEnvelopeSuccessReturnOverrides {
    pub recovered: Option<RecoveredEnvelope>,
}

pub fn build_recover_envelope_success_return(
    overrides: RecoverEnvelopeSuccessReturnOverrides,
) -> RecoverEnvelopeSuccessReturn {
    RecoverEnvelopeSuccessReturn {
        recovered: overrides
            .recovered
            .unwrap_or_else(|| build_recovered_envelope(Default::default())),
    }
}

#[derive(Default)]
pub struct ConsumeChainParamsOverrides;

pub fn build_consume_chain_params(_overrides: ConsumeChainParamsOverrides) -> ConsumeChainParams {
    ConsumeChainParams
}

#[derive(Default)]
pub struct ConsumeChainPayloadOverrides;

pub fn build_consume_chain_payload<K: IChainAdapter>(
    adapter: K,
    _overrides: ConsumeChainPayloadOverrides,
) -> ConsumeChainPayload<K> {
    ConsumeChainPayload { adapter }
}

pub struct AttemptContextOverrides<F: IChainForms> {
    pub asset: Option<Root>,
    pub deployment: Option<DeploymentIdentity>,
    pub entitlement_contract: Option<F::Identity>,
    pub entitlement: Option<F::Entitlement>,
    pub identity: Option<F::Identity>,
    pub handshake_key: Option<HandshakeKeyCommitment>,
    pub interval: Option<F::Interval>,
    pub reference: Option<SettlementReference>,
}

impl<F: IChainForms> Default for AttemptContextOverrides<F> {
    fn default() -> Self {
        Self {
            asset: None,
            deployment: None,
            entitlement_contract: None,
            entitlement: None,
            identity: None,
            handshake_key: None,
            interval: None,
            reference: None,
        }
    }
}

pub fn build_attempt_context<F: IChainForms>(
    overrides: AttemptContextOverrides<F>,
) -> AttemptContext<F> {
    AttemptContext {
        asset: overrides
            .asset
            .unwrap_or_else(|| build_root(Default::default())),
        deployment: overrides
            .deployment
            .unwrap_or_else(|| build_deployment_identity(Default::default())),
        entitlement_contract: overrides.entitlement_contract.unwrap_or_else(|| {
            F::Identity::from_field(
                FromFieldParams,
                CanonicalFieldValue::FixedBytes20([0x11; 20]),
            )
            .expect("the canonical fixed-bytes-20 value is admitted by every identity form")
            .value
        }),
        entitlement: overrides.entitlement.unwrap_or_else(|| {
            F::Entitlement::from_field(
                FromFieldParams,
                CanonicalFieldValue::Unsigned256([0x12; 32]),
            )
            .expect("the canonical unsigned-256 value is admitted by every entitlement form")
            .value
        }),
        identity: overrides.identity.unwrap_or_else(|| {
            F::Identity::from_field(
                FromFieldParams,
                CanonicalFieldValue::FixedBytes20([0x11; 20]),
            )
            .expect("the canonical fixed-bytes-20 value is admitted by every identity form")
            .value
        }),
        handshake_key: overrides
            .handshake_key
            .unwrap_or_else(|| build_handshake_key_commitment(Default::default())),
        interval: overrides.interval.unwrap_or_else(|| {
            F::Interval::from_field(FromFieldParams, CanonicalFieldValue::Unsigned64(9))
                .expect("the canonical unsigned-64 value is admitted by every interval form")
                .value
        }),
        reference: overrides
            .reference
            .unwrap_or_else(|| build_settlement_reference(Default::default())),
    }
}

pub struct FeeRequestOverrides<F: IChainForms> {
    pub to: Option<F::Identity>,
    pub input: Option<Vec<u8>>,
}

impl<F: IChainForms> Default for FeeRequestOverrides<F> {
    fn default() -> Self {
        Self {
            to: None,
            input: None,
        }
    }
}

pub fn build_fee_request<F: IChainForms>(overrides: FeeRequestOverrides<F>) -> FeeRequest<F> {
    FeeRequest {
        to: overrides.to.unwrap_or_else(|| {
            F::Identity::from_field(
                FromFieldParams,
                CanonicalFieldValue::FixedBytes20([0x11; 20]),
            )
            .expect("the canonical fixed-bytes-20 value is admitted by every identity form")
            .value
        }),
        input: overrides.input.unwrap_or_else(|| vec![0xaa]),
    }
}

pub(crate) const MOCK_I_CHAIN_ADAPTER_DECLARATION: ChainDeclaration = ChainDeclaration {
    identifier: ChainMemberIdentifier::Mock,
    name: "Mock",
    adapter_version: 1,
    interface_version: CHAIN_INTERFACE_VERSION,
    forms_identifier: ChainFormsIdentifier::EvmV1,
    precompile_encodings: &[
        PrecompileEncoding::Eip196Eip197,
        PrecompileEncoding::Eip2537,
        PrecompileEncoding::Mock,
    ],
    fee_entries: &[
        FeeEntryDeclaration {
            name: "execution_gas",
            unit: "gas",
        },
        FeeEntryDeclaration {
            name: "l1_data_fee",
            unit: "wei",
        },
    ],
    settings: &[],
};

pub(crate) struct MockIChainAdapter<F> {
    failure_mode: MockIChainAdapterFailureMode,
    forms: PhantomData<F>,
}

impl<F: IChainForms> ISettlementAdapter for MockIChainAdapter<F> {
    fn current_reference_at(
        &self,
        params: CurrentReferenceAtParams,
        _payload: CurrentReferenceAtPayload,
    ) -> CurrentReferenceAtReturn {
        if self.failure_mode == MockIChainAdapterFailureMode::TransportRefused {
            return Err(CurrentReferenceAtErrorReturn::Mock(
                MockIChainAdapterCurrentReferenceAtErrorReturn::TransportRefused,
            ));
        }
        let ordinal = match params.tier {
            SettlementTier::Included => 40,
            SettlementTier::Soft => 30,
            SettlementTier::Hard => 20,
            SettlementTier::Settled => 10,
        };
        Ok(CurrentReferenceAtSuccessReturn {
            reference: build_settlement_reference(SettlementReferenceConstructorParamsOverrides {
                ordinal: Some(ordinal),
                identity: None,
            }),
        })
    }

    fn tier_of(&self, _params: TierOfParams, payload: TierOfPayload<'_>) -> TierOfReturn {
        if self.failure_mode == MockIChainAdapterFailureMode::TransportRefused {
            return Err(TierOfErrorReturn::Mock(
                MockIChainAdapterTierOfErrorReturn::TransportRefused,
            ));
        }
        let reference = *payload.reference;
        if reference.ordinal() > 40 {
            return Err(TierOfErrorReturn::NotIncluded { reference });
        }
        let tier = if reference.ordinal() <= 10 {
            SettlementTier::Settled
        } else if reference.ordinal() <= 20 {
            SettlementTier::Hard
        } else if reference.ordinal() <= 30 {
            SettlementTier::Soft
        } else {
            SettlementTier::Included
        };
        Ok(TierOfSuccessReturn { tier })
    }

    fn reference_age(
        &self,
        _params: ReferenceAgeParams,
        payload: ReferenceAgePayload<'_>,
    ) -> ReferenceAgeReturn {
        if self.failure_mode == MockIChainAdapterFailureMode::TransportRefused {
            return Err(ReferenceAgeErrorReturn::Mock(
                MockIChainAdapterReferenceAgeErrorReturn::TransportRefused,
            ));
        }
        let reference = *payload.reference;
        if reference.ordinal() > 40 {
            return Err(ReferenceAgeErrorReturn::NotIncluded { reference });
        }
        Ok(ReferenceAgeSuccessReturn {
            age: ReferenceAge {
                seconds: 2 * (40 - reference.ordinal()),
            },
        })
    }
}

impl<F: IChainForms> IEntitlementStateAdapter for MockIChainAdapter<F> {
    type Forms = F;

    fn evaluate_authorization(
        &self,
        _params: EvaluateAuthorizationParams,
        payload: EvaluateAuthorizationPayload<'_, Self::Forms>,
    ) -> EvaluateAuthorizationReturn {
        if self.failure_mode == MockIChainAdapterFailureMode::TransportRefused {
            return Err(EvaluateAuthorizationErrorReturn::Mock(
                MockIChainAdapterEvaluateAuthorizationErrorReturn::TransportRefused,
            ));
        }
        let result = match payload.context.reference.ordinal() {
            0..=20 => AuthorizationResult::Authorized,
            21..=40 => AuthorizationResult::PendingSettlement,
            _ => AuthorizationResult::Denied,
        };
        Ok(EvaluateAuthorizationSuccessReturn { result })
    }

    fn evaluate_authorization_batch(
        &self,
        _params: EvaluateAuthorizationBatchParams,
        payload: EvaluateAuthorizationBatchPayload<'_, Self::Forms>,
    ) -> EvaluateAuthorizationBatchReturn {
        if self.failure_mode == MockIChainAdapterFailureMode::TransportRefused {
            return Err(EvaluateAuthorizationBatchErrorReturn::Mock(
                MockIChainAdapterEvaluateAuthorizationBatchErrorReturn::TransportRefused,
            ));
        }
        let limit: u64 = 64;
        let supplied = u64::try_from(payload.contexts.len())
            .expect("a context count fits a u64 on every supported target");
        if supplied > limit {
            return Err(EvaluateAuthorizationBatchErrorReturn::PageTooLarge { limit, supplied });
        }
        let results = payload
            .contexts
            .iter()
            .map(|context| match context.reference.ordinal() {
                0..=20 => AuthorizationResult::Authorized,
                21..=40 => AuthorizationResult::PendingSettlement,
                _ => AuthorizationResult::Denied,
            })
            .collect();
        Ok(EvaluateAuthorizationBatchSuccessReturn {
            results,
            next: None,
        })
    }

    fn batch_page_limit(
        &self,
        _params: BatchPageLimitParams,
        _payload: BatchPageLimitPayload,
    ) -> BatchPageLimitReturn {
        Ok(BatchPageLimitSuccessReturn { limit: 64 })
    }
}

impl<F: IChainForms> IChainAdapter for MockIChainAdapter<F> {
    const DECLARATION: ChainDeclaration = MOCK_I_CHAIN_ADAPTER_DECLARATION;

    fn endpoint(&self, _params: EndpointParams, _payload: EndpointPayload) -> EndpointReturn {
        Ok(EndpointSuccessReturn {
            endpoint: build_chain_endpoint(Default::default()),
        })
    }

    fn chain_identifier(
        &self,
        _params: ChainIdentifierParams,
        _payload: ChainIdentifierPayload,
    ) -> ChainIdentifierReturn<<Self::Forms as IChainForms>::ChainIdentifier> {
        F::ChainIdentifier::from_field(
            FromFieldParams,
            CanonicalFieldValue::Unsigned256([0x13; 32]),
        )
        .map(|success| ChainIdentifierSuccessReturn {
            chain_identifier: success.value,
        })
        .map_err(|error| {
            ChainIdentifierErrorReturn::Base(BaseChainIdentifierErrorReturn::Form(error))
        })
    }

    fn entitlement_contract(
        &self,
        _params: EntitlementContractParams,
        _payload: EntitlementContractPayload,
    ) -> EntitlementContractReturn<<Self::Forms as IChainForms>::Identity> {
        F::Identity::from_field(
            FromFieldParams,
            CanonicalFieldValue::FixedBytes20([0x14; 20]),
        )
        .map(|success| EntitlementContractSuccessReturn {
            contract: success.value,
        })
        .map_err(|error| {
            EntitlementContractErrorReturn::Base(BaseEntitlementContractErrorReturn::Form(error))
        })
    }

    fn estimate_fee(
        &self,
        _params: EstimateFeeParams,
        _payload: EstimateFeePayload<'_, Self::Forms>,
    ) -> EstimateFeeReturn {
        if self.failure_mode == MockIChainAdapterFailureMode::TransportRefused {
            return Err(EstimateFeeErrorReturn::Mock(
                MockIChainAdapterEstimateFeeErrorReturn::TransportRefused,
            ));
        }
        let entries = Self::DECLARATION
            .fee_entries
            .iter()
            .enumerate()
            .map(|(index, entry)| FeeEntry {
                name: entry.name,
                unit: entry.unit,
                quantity: FeeQuantity {
                    value: u128::try_from(index + 1)
                        .expect("a fee entry index fits a u128 on every supported target"),
                },
            })
            .collect();
        Ok(EstimateFeeSuccessReturn {
            description: FeeDescription { entries },
        })
    }

    fn recover_envelope(
        &self,
        _params: RecoverEnvelopeParams,
        payload: RecoverEnvelopePayload<'_, Self::Forms>,
    ) -> RecoverEnvelopeReturn {
        if self.failure_mode == MockIChainAdapterFailureMode::TransportRefused {
            return Err(RecoverEnvelopeErrorReturn::Mock(
                MockIChainAdapterRecoverEnvelopeErrorReturn::TransportRefused,
            ));
        }
        let CanonicalFieldValue::Unsigned64(interval) =
            F::Interval::to_field(ToFieldParams, payload.interval)
                .expect("the interval form yields its field")
                .field
        else {
            unreachable!("the interval form yields its 64-bit value");
        };
        if interval > 1000 {
            return Err(RecoverEnvelopeErrorReturn::NotFound);
        }
        let CanonicalFieldValue::Unsigned256(envelope) =
            F::Entitlement::to_field(ToFieldParams, payload.entitlement)
                .expect("the entitlement form yields its field")
                .field
        else {
            unreachable!("the entitlement form yields its 32 bytes");
        };
        Ok(RecoverEnvelopeSuccessReturn {
            recovered: RecoveredEnvelope {
                envelope: envelope.to_vec(),
                transaction: [0x78; 32],
            },
        })
    }
}

#[derive(Default)]
pub(crate) struct MockIChainAdapterOverrides {
    pub(crate) failure_mode: Option<MockIChainAdapterFailureMode>,
}

pub(crate) fn build_mock_i_chain_adapter<F>(
    overrides: MockIChainAdapterOverrides,
) -> MockIChainAdapter<F> {
    MockIChainAdapter {
        failure_mode: overrides
            .failure_mode
            .unwrap_or(MockIChainAdapterFailureMode::NoFailure),
        forms: PhantomData,
    }
}

#[derive(Default)]
pub struct ChainSelectionOverrides;

pub fn build_chain_selection(
    identifier: ChainMemberIdentifier,
    _overrides: ChainSelectionOverrides,
) -> ChainSelection {
    match identifier {
        ChainMemberIdentifier::Base => ChainSelection::Base {
            settings: build_base_chain_settings(Default::default()),
        },
        ChainMemberIdentifier::Mock => ChainSelection::Mock {
            failure_mode: MockIChainAdapterFailureMode::NoFailure,
        },
    }
}

pub struct MockIChainConsumer;

impl IChainConsumer for MockIChainConsumer {
    type Output = ();

    fn consume_chain<K: IChainAdapter>(
        &self,
        _params: ConsumeChainParams,
        _payload: ConsumeChainPayload<K>,
    ) {
    }
}

#[derive(Default)]
pub struct CreateChainDepsOverrides;

pub fn build_create_chain_deps<C: IChainConsumer>(
    admit_chain_composition: AdmitChainCompositionFn,
    consumer: C,
    _overrides: CreateChainDepsOverrides,
) -> CreateChainDeps<C> {
    CreateChainDeps {
        admit_chain_composition,
        consumer,
    }
}

#[derive(Default)]
pub struct CreateChainParamsOverrides {
    pub selection: Option<ChainSelection>,
}

pub fn build_create_chain_params(overrides: CreateChainParamsOverrides) -> CreateChainParams {
    CreateChainParams {
        selection: overrides.selection.unwrap_or_else(|| {
            build_chain_selection(ChainMemberIdentifier::Mock, ChainSelectionOverrides)
        }),
    }
}

#[derive(Default)]
pub struct CreateChainPayloadOverrides;

pub fn build_create_chain_payload(_overrides: CreateChainPayloadOverrides) -> CreateChainPayload {
    CreateChainPayload
}

#[derive(Default)]
pub struct CreateChainSuccessReturnOverrides;

pub fn build_create_chain_success_return<O>(
    output: O,
    _overrides: CreateChainSuccessReturnOverrides,
) -> CreateChainSuccessReturn<O> {
    CreateChainSuccessReturn { output }
}

pub fn mock_create_chain<F: IChainForms, C: IChainConsumer>(
    deps: &CreateChainDeps<C>,
    _params: CreateChainParams,
    _payload: CreateChainPayload,
) -> CreateChainReturn<C::Output> {
    let adapter = build_mock_i_chain_adapter::<F>(Default::default());
    let output = deps.consumer.consume_chain(
        build_consume_chain_params(Default::default()),
        build_consume_chain_payload(adapter, Default::default()),
    );
    Ok(build_create_chain_success_return(
        output,
        Default::default(),
    ))
}
