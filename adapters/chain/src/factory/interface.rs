use core::convert::Infallible;

use crate::admit_chain_composition::provides::{
    AdmitChainCompositionErrorReturn, AdmitChainCompositionFn,
};
use crate::base::provides::{
    BaseChainIdentifierErrorReturn, BaseChainSettings, BaseChainTryNewErrorReturn,
    BaseCurrentReferenceAtErrorReturn, BaseEntitlementContractErrorReturn,
    BaseEstimateFeeErrorReturn, BaseEvaluateAuthorizationBatchErrorReturn,
    BaseEvaluateAuthorizationErrorReturn, BaseRecoverEnvelopeErrorReturn,
    BaseReferenceAgeErrorReturn, BaseTierOfErrorReturn,
};
use domain::{DeploymentIdentity, HandshakeKeyCommitment, SignatureSchemeIdentifier};
use encoding::{CanonicalFieldKind, CanonicalFieldValue, EncodingIdentifier, ICanonicalField};
use hashing::Root;
use pairing::PrecompileEncoding;

pub const CHAIN_FORMS_INTERFACE_VERSION: u32 = 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChainFormsIdentifier {
    EvmV1,
    #[cfg(any(test, feature = "mocks"))]
    Mock,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IntentMemberNames {
    pub identity: &'static str,
    pub nonce: &'static str,
    pub expiry: &'static str,
    pub call: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChainFormsDeclaration {
    pub identifier: ChainFormsIdentifier,
    pub adapter_version: u32,
    pub interface_version: u32,
    pub encoding_identifier: EncodingIdentifier,
    pub key_identifier_domain_tag: &'static [u8],
    pub intent_domain_name: &'static str,
    pub intent_domain_version: &'static str,
    pub intent_struct_name: &'static str,
    pub intent_member_names: IntentMemberNames,
    pub nonce_kind: CanonicalFieldKind,
    pub expiry_kind: CanonicalFieldKind,
}

pub struct DeriveKeyIdentifierParams;

pub struct DeriveKeyIdentifierPayload<'a> {
    pub scheme: SignatureSchemeIdentifier,
    pub key: &'a [u8],
}

pub struct DeriveKeyIdentifierSuccessReturn<K> {
    pub key_identifier: K,
}

pub type DeriveKeyIdentifierReturn<K> = Result<DeriveKeyIdentifierSuccessReturn<K>, Infallible>;

pub trait IIdentityForm: ICanonicalField {
    type KeyIdentifier: ICanonicalField + Clone + core::fmt::Debug + PartialEq + Eq;

    fn derive_key_identifier(
        params: DeriveKeyIdentifierParams,
        payload: DeriveKeyIdentifierPayload<'_>,
    ) -> DeriveKeyIdentifierReturn<Self::KeyIdentifier>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntentField {
    pub name: String,
    pub value: CanonicalFieldValue,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntentCall {
    pub operation: String,
    pub fields: Vec<IntentField>,
}

pub struct SignedIntentParams;

pub struct SignedIntentPayload<'a, F: IChainForms> {
    pub chain: &'a F::ChainIdentifier,
    pub contract: &'a F::Identity,
    pub identity: &'a F::Identity,
    pub nonce: CanonicalFieldValue,
    pub expiry: CanonicalFieldValue,
    pub call: &'a IntentCall,
}

pub struct SignedIntentSuccessReturn<S> {
    pub signed_intent: S,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SignedIntentErrorReturn {
    OperationNotIdentifier {
        operation: String,
    },
    OperationIsStructName {
        operation: String,
    },
    FieldNotIdentifier {
        name: String,
    },
    FieldRepeated {
        name: String,
    },
    NonceKind {
        expected: CanonicalFieldKind,
        actual: CanonicalFieldKind,
    },
    ExpiryKind {
        expected: CanonicalFieldKind,
        actual: CanonicalFieldKind,
    },
}

pub type SignedIntentReturn<S> = Result<SignedIntentSuccessReturn<S>, SignedIntentErrorReturn>;

pub trait IChainForms: core::fmt::Debug + PartialEq + Eq {
    const DECLARATION: ChainFormsDeclaration;

    type Identity: IIdentityForm + Clone + core::fmt::Debug + PartialEq + Eq;
    type Entitlement: ICanonicalField + Clone + core::fmt::Debug + PartialEq + Eq;
    type Interval: ICanonicalField + Clone + core::fmt::Debug + PartialEq + Eq;
    type ChainIdentifier: ICanonicalField + Clone + core::fmt::Debug + PartialEq + Eq;
    type SignedIntent: ICanonicalField + Clone + core::fmt::Debug + PartialEq + Eq;

    fn signed_intent(
        &self,
        params: SignedIntentParams,
        payload: SignedIntentPayload<'_, Self>,
    ) -> SignedIntentReturn<Self::SignedIntent>
    where
        Self: Sized;
}

pub const CHAIN_INTERFACE_VERSION: u32 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChainMemberIdentifier {
    Base,
    #[cfg(any(test, feature = "mocks"))]
    Mock,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChainEndpoint {
    pub text: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChainSettingBounds {
    Unbounded,
    TextPrefixedBy { prefixes: &'static [&'static str] },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChainSettingDisposition {
    Fixed {
        value: &'static str,
    },
    Settable {
        bounds: ChainSettingBounds,
        default: &'static str,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChainSettingStatement {
    pub name: &'static str,
    pub disposition: ChainSettingDisposition,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FeeEntryDeclaration {
    pub name: &'static str,
    pub unit: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChainDeclaration {
    pub identifier: ChainMemberIdentifier,
    pub name: &'static str,
    pub adapter_version: u32,
    pub interface_version: u32,
    pub forms_identifier: ChainFormsIdentifier,
    pub precompile_encodings: &'static [PrecompileEncoding],
    pub fee_entries: &'static [FeeEntryDeclaration],
    pub settings: &'static [ChainSettingStatement],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SettlementTier {
    Included,
    Soft,
    Hard,
    Settled,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementReference {
    pub(super) ordinal: u64,
    pub(super) identity: [u8; 32],
}

pub struct SettlementReferenceConstructorParams {
    pub ordinal: u64,
    pub identity: [u8; 32],
}

pub type SettlementReferenceTryNewReturn = Result<SettlementReference, Infallible>;

impl SettlementReference {
    pub fn try_new(
        params: SettlementReferenceConstructorParams,
    ) -> SettlementReferenceTryNewReturn {
        Ok(SettlementReference {
            ordinal: params.ordinal,
            identity: params.identity,
        })
    }

    pub fn ordinal(&self) -> u64 {
        self.ordinal
    }

    pub fn identity(&self) -> &[u8; 32] {
        &self.identity
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReferenceAge {
    pub seconds: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuthorizationResult {
    Authorized,
    PendingSettlement,
    Denied,
}

pub struct AttemptContext<F: IChainForms> {
    pub asset: Root,
    pub deployment: DeploymentIdentity,
    pub entitlement_contract: F::Identity,
    pub entitlement: F::Entitlement,
    pub identity: F::Identity,
    pub handshake_key: HandshakeKeyCommitment,
    pub interval: F::Interval,
    pub reference: SettlementReference,
}

pub struct FeeRequest<F: IChainForms> {
    pub to: F::Identity,
    pub input: Vec<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FeeQuantity {
    pub value: u128,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FeeEntry {
    pub name: &'static str,
    pub unit: &'static str,
    pub quantity: FeeQuantity,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FeeDescription {
    pub entries: Vec<FeeEntry>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecoveredEnvelope {
    pub envelope: Vec<u8>,
    pub transaction: [u8; 32],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BatchPage {
    pub index: u64,
}

pub struct CurrentReferenceAtParams {
    pub tier: SettlementTier,
}

pub struct CurrentReferenceAtPayload;

pub struct CurrentReferenceAtSuccessReturn {
    pub reference: SettlementReference,
}

#[derive(Debug)]
pub enum CurrentReferenceAtErrorReturn {
    NotAttained {
        tier: SettlementTier,
    },
    Base(BaseCurrentReferenceAtErrorReturn),
    #[cfg(any(test, feature = "mocks"))]
    Mock(MockIChainAdapterCurrentReferenceAtErrorReturn),
}

pub type CurrentReferenceAtReturn =
    Result<CurrentReferenceAtSuccessReturn, CurrentReferenceAtErrorReturn>;

pub struct TierOfParams;

pub struct TierOfPayload<'a> {
    pub reference: &'a SettlementReference,
}

pub struct TierOfSuccessReturn {
    pub tier: SettlementTier,
}

#[derive(Debug)]
pub enum TierOfErrorReturn {
    NotIncluded {
        reference: SettlementReference,
    },
    Reorganized {
        reference: SettlementReference,
    },
    Base(BaseTierOfErrorReturn),
    #[cfg(any(test, feature = "mocks"))]
    Mock(MockIChainAdapterTierOfErrorReturn),
}

pub type TierOfReturn = Result<TierOfSuccessReturn, TierOfErrorReturn>;

pub struct ReferenceAgeParams;

pub struct ReferenceAgePayload<'a> {
    pub reference: &'a SettlementReference,
}

pub struct ReferenceAgeSuccessReturn {
    pub age: ReferenceAge,
}

#[derive(Debug)]
pub enum ReferenceAgeErrorReturn {
    NotIncluded {
        reference: SettlementReference,
    },
    Base(BaseReferenceAgeErrorReturn),
    #[cfg(any(test, feature = "mocks"))]
    Mock(MockIChainAdapterReferenceAgeErrorReturn),
}

pub type ReferenceAgeReturn = Result<ReferenceAgeSuccessReturn, ReferenceAgeErrorReturn>;

pub struct EvaluateAuthorizationParams;

pub struct EvaluateAuthorizationPayload<'a, F: IChainForms> {
    pub context: &'a AttemptContext<F>,
}

pub struct EvaluateAuthorizationSuccessReturn {
    pub result: AuthorizationResult,
}

#[derive(Debug)]
pub enum EvaluateAuthorizationErrorReturn {
    Reorganized {
        reference: SettlementReference,
    },
    Base(BaseEvaluateAuthorizationErrorReturn),
    #[cfg(any(test, feature = "mocks"))]
    Mock(MockIChainAdapterEvaluateAuthorizationErrorReturn),
}

pub type EvaluateAuthorizationReturn =
    Result<EvaluateAuthorizationSuccessReturn, EvaluateAuthorizationErrorReturn>;

pub struct EvaluateAuthorizationBatchParams {
    pub page: BatchPage,
}

pub struct EvaluateAuthorizationBatchPayload<'a, F: IChainForms> {
    pub contexts: &'a [AttemptContext<F>],
}

pub struct EvaluateAuthorizationBatchSuccessReturn {
    pub results: Vec<AuthorizationResult>,
    pub next: Option<BatchPage>,
}

#[derive(Debug)]
pub enum EvaluateAuthorizationBatchErrorReturn {
    PageTooLarge {
        limit: u64,
        supplied: u64,
    },
    Reorganized {
        reference: SettlementReference,
    },
    Base(BaseEvaluateAuthorizationBatchErrorReturn),
    #[cfg(any(test, feature = "mocks"))]
    Mock(MockIChainAdapterEvaluateAuthorizationBatchErrorReturn),
}

pub type EvaluateAuthorizationBatchReturn =
    Result<EvaluateAuthorizationBatchSuccessReturn, EvaluateAuthorizationBatchErrorReturn>;

pub struct BatchPageLimitParams;

pub struct BatchPageLimitPayload;

pub struct BatchPageLimitSuccessReturn {
    pub limit: u64,
}

pub type BatchPageLimitReturn = Result<BatchPageLimitSuccessReturn, Infallible>;

pub struct EndpointParams;

pub struct EndpointPayload;

pub struct EndpointSuccessReturn {
    pub endpoint: ChainEndpoint,
}

pub type EndpointReturn = Result<EndpointSuccessReturn, Infallible>;

pub struct ChainIdentifierParams;

pub struct ChainIdentifierPayload;

pub struct ChainIdentifierSuccessReturn<I> {
    pub chain_identifier: I,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ChainIdentifierErrorReturn {
    Base(BaseChainIdentifierErrorReturn),
}

pub type ChainIdentifierReturn<I> =
    Result<ChainIdentifierSuccessReturn<I>, ChainIdentifierErrorReturn>;

pub struct EntitlementContractParams;

pub struct EntitlementContractPayload;

pub struct EntitlementContractSuccessReturn<I> {
    pub contract: I,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EntitlementContractErrorReturn {
    Base(BaseEntitlementContractErrorReturn),
}

pub type EntitlementContractReturn<I> =
    Result<EntitlementContractSuccessReturn<I>, EntitlementContractErrorReturn>;

pub struct EstimateFeeParams;

pub struct EstimateFeePayload<'a, F: IChainForms> {
    pub request: &'a FeeRequest<F>,
}

pub struct EstimateFeeSuccessReturn {
    pub description: FeeDescription,
}

#[derive(Debug)]
pub enum EstimateFeeErrorReturn {
    Base(BaseEstimateFeeErrorReturn),
    #[cfg(any(test, feature = "mocks"))]
    Mock(MockIChainAdapterEstimateFeeErrorReturn),
}

pub type EstimateFeeReturn = Result<EstimateFeeSuccessReturn, EstimateFeeErrorReturn>;

pub struct RecoverEnvelopeParams;

pub struct RecoverEnvelopePayload<'a, F: IChainForms> {
    pub entitlement: &'a F::Entitlement,
    pub interval: &'a F::Interval,
}

pub struct RecoverEnvelopeSuccessReturn {
    pub recovered: RecoveredEnvelope,
}

#[derive(Debug)]
pub enum RecoverEnvelopeErrorReturn {
    NotFound,
    Base(BaseRecoverEnvelopeErrorReturn),
    #[cfg(any(test, feature = "mocks"))]
    Mock(MockIChainAdapterRecoverEnvelopeErrorReturn),
}

pub type RecoverEnvelopeReturn = Result<RecoverEnvelopeSuccessReturn, RecoverEnvelopeErrorReturn>;

pub trait ISettlementAdapter {
    fn current_reference_at(
        &self,
        params: CurrentReferenceAtParams,
        payload: CurrentReferenceAtPayload,
    ) -> CurrentReferenceAtReturn;

    fn tier_of(&self, params: TierOfParams, payload: TierOfPayload<'_>) -> TierOfReturn;

    fn reference_age(
        &self,
        params: ReferenceAgeParams,
        payload: ReferenceAgePayload<'_>,
    ) -> ReferenceAgeReturn;
}

pub trait IEntitlementStateAdapter {
    type Forms: IChainForms;

    fn evaluate_authorization(
        &self,
        params: EvaluateAuthorizationParams,
        payload: EvaluateAuthorizationPayload<'_, Self::Forms>,
    ) -> EvaluateAuthorizationReturn;

    fn evaluate_authorization_batch(
        &self,
        params: EvaluateAuthorizationBatchParams,
        payload: EvaluateAuthorizationBatchPayload<'_, Self::Forms>,
    ) -> EvaluateAuthorizationBatchReturn;

    fn batch_page_limit(
        &self,
        params: BatchPageLimitParams,
        payload: BatchPageLimitPayload,
    ) -> BatchPageLimitReturn;
}

pub trait IChainAdapter: ISettlementAdapter + IEntitlementStateAdapter {
    const DECLARATION: ChainDeclaration;

    fn endpoint(&self, params: EndpointParams, payload: EndpointPayload) -> EndpointReturn;

    fn chain_identifier(
        &self,
        params: ChainIdentifierParams,
        payload: ChainIdentifierPayload,
    ) -> ChainIdentifierReturn<<Self::Forms as IChainForms>::ChainIdentifier>;

    fn entitlement_contract(
        &self,
        params: EntitlementContractParams,
        payload: EntitlementContractPayload,
    ) -> EntitlementContractReturn<<Self::Forms as IChainForms>::Identity>;

    fn estimate_fee(
        &self,
        params: EstimateFeeParams,
        payload: EstimateFeePayload<'_, Self::Forms>,
    ) -> EstimateFeeReturn;

    fn recover_envelope(
        &self,
        params: RecoverEnvelopeParams,
        payload: RecoverEnvelopePayload<'_, Self::Forms>,
    ) -> RecoverEnvelopeReturn;
}

pub struct ConsumeChainParams;

pub struct ConsumeChainPayload<K: IChainAdapter> {
    pub adapter: K,
}

pub trait IChainConsumer {
    type Output;

    fn consume_chain<K: IChainAdapter>(
        &self,
        params: ConsumeChainParams,
        payload: ConsumeChainPayload<K>,
    ) -> Self::Output;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ChainSelection {
    Base {
        settings: BaseChainSettings,
    },
    #[cfg(any(test, feature = "mocks"))]
    Mock {
        failure_mode: MockIChainAdapterFailureMode,
    },
}

pub const DECLARED_CHAIN_MEMBERS: &[ChainMemberIdentifier] = &[ChainMemberIdentifier::Base];

#[cfg(any(test, feature = "mocks"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MockIChainAdapterFailureMode {
    NoFailure,
    TransportRefused,
}

#[cfg(any(test, feature = "mocks"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MockIChainAdapterCurrentReferenceAtErrorReturn {
    TransportRefused,
}

#[cfg(any(test, feature = "mocks"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MockIChainAdapterTierOfErrorReturn {
    TransportRefused,
}

#[cfg(any(test, feature = "mocks"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MockIChainAdapterReferenceAgeErrorReturn {
    TransportRefused,
}

#[cfg(any(test, feature = "mocks"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MockIChainAdapterEvaluateAuthorizationErrorReturn {
    TransportRefused,
}

#[cfg(any(test, feature = "mocks"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MockIChainAdapterEvaluateAuthorizationBatchErrorReturn {
    TransportRefused,
}

#[cfg(any(test, feature = "mocks"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MockIChainAdapterEstimateFeeErrorReturn {
    TransportRefused,
}

#[cfg(any(test, feature = "mocks"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MockIChainAdapterRecoverEnvelopeErrorReturn {
    TransportRefused,
}

pub struct CreateChainDeps<C: IChainConsumer> {
    pub admit_chain_composition: AdmitChainCompositionFn,
    pub consumer: C,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreateChainParams {
    pub selection: ChainSelection,
}

pub struct CreateChainPayload;

pub struct CreateChainSuccessReturn<O> {
    pub output: O,
}

#[derive(Debug)]
pub enum CreateChainErrorReturn {
    Composition(AdmitChainCompositionErrorReturn),
    Base(BaseChainTryNewErrorReturn),
}

pub type CreateChainReturn<O> = Result<CreateChainSuccessReturn<O>, CreateChainErrorReturn>;

pub type CreateChainFn<C> = fn(
    &CreateChainDeps<C>,
    CreateChainParams,
    CreateChainPayload,
) -> CreateChainReturn<<C as IChainConsumer>::Output>;
