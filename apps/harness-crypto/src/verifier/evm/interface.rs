use alloy::primitives::Address;
use alloy::providers::DynProvider;
use alloy::transports::TransportError;
#[cfg(any(test, feature = "mocks"))]
use chain::{
    CreateChainErrorReturn, CreateChainFormsErrorReturn, CreateChainFormsParams, CreateChainParams,
};
use chain::{EstimateFeeErrorReturn, IChainAdapter, IChainForms, IEntitlementStateAdapter};
use encoding::{CanonicalFieldKind, CanonicalFieldValue, FromFieldErrorReturn};
#[cfg(any(test, feature = "mocks"))]
use encoding::{CreateEncodingErrorReturn, CreateEncodingParams};
#[cfg(any(test, feature = "mocks"))]
use hash_to_scalar::{CreateHashToScalarErrorReturn, CreateHashToScalarParams};
use pairing::IPairingAdapter;
#[cfg(any(test, feature = "mocks"))]
use pairing::{
    CreatePairingErrorReturn, CreatePairingParams, IPairingArithmetic, IPairingReference,
};
use proof::IDeliveryProofAdapter;
#[cfg(any(test, feature = "mocks"))]
use proof::{CreateDeliveryProofErrorReturn, CreateDeliveryProofParams};
#[cfg(any(test, feature = "mocks"))]
use random::{CreateRandomSourceErrorReturn, CreateRandomSourceParams};
use tokio::runtime::Runtime;
use url::ParseError;

use super::settings::provides::EvmVerifierSettings;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EvmVerifierField {
    VerifierAddress,
}

pub const EVM_VERIFIER_CONTEXT_FIELD_KINDS: &[CanonicalFieldKind] = &[
    CanonicalFieldKind::Unsigned256,
    CanonicalFieldKind::FixedBytes20,
    CanonicalFieldKind::Unsigned256,
    CanonicalFieldKind::Unsigned256,
    CanonicalFieldKind::Unsigned64,
    CanonicalFieldKind::Unsigned64,
    CanonicalFieldKind::FixedBytes20,
    CanonicalFieldKind::FixedBytes20,
];

pub struct EvmVerifierConstructorParams<
    'a,
    P: IPairingAdapter,
    C: IChainAdapter,
    D: IDeliveryProofAdapter<Pairing = P, Forms = <C as IEntitlementStateAdapter>::Forms>,
> {
    pub pairing: &'a P,
    pub chain: &'a C,
    pub delivery_proof: &'a D,
    pub verifier_address: CanonicalFieldValue,
    pub settings: EvmVerifierSettings,
}

pub struct EvmVerifier<
    'a,
    P: IPairingAdapter,
    C: IChainAdapter,
    D: IDeliveryProofAdapter<Pairing = P, Forms = <C as IEntitlementStateAdapter>::Forms>,
> {
    pub(super) pairing: &'a P,
    pub(super) chain: &'a C,
    pub(super) delivery_proof: &'a D,
    pub(super) verifier: <<C as IEntitlementStateAdapter>::Forms as IChainForms>::Identity,
    pub(super) verifier_address: Address,
    pub(super) provider: DynProvider,
    pub(super) runtime: Runtime,
}

pub type EvmVerifierTryNewReturn<'a, P, C, D> =
    Result<EvmVerifier<'a, P, C, D>, EvmVerifierTryNewErrorReturn>;

#[derive(Debug)]
pub enum EvmVerifierTryNewErrorReturn {
    Identity {
        field: EvmVerifierField,
        error: FromFieldErrorReturn,
    },
    Endpoint {
        error: ParseError,
    },
    Runtime(std::io::Error),
}

#[derive(Debug)]
pub enum EvmVerifierErrorReturn {
    NothingDeployed { address: [u8; 20] },
    ScalarLength { actual: usize },
    Rpc(TransportError),
    ResultDecode(alloy::sol_types::Error),
    Fee(EstimateFeeErrorReturn),
}

#[cfg(any(test, feature = "mocks"))]
pub trait IEvmVerifierCaseConsumer {
    type Output;

    fn consume_evm_verifier_case<P, C, D>(
        &self,
        params: ConsumeEvmVerifierCaseParams,
        payload: ConsumeEvmVerifierCasePayload<'_, P, C, D>,
    ) -> Self::Output
    where
        P: IPairingArithmetic + IPairingReference,
        C: IChainAdapter,
        D: IDeliveryProofAdapter<Pairing = P, Forms = <C as IEntitlementStateAdapter>::Forms>;
}

#[cfg(any(test, feature = "mocks"))]
pub struct ConsumeEvmVerifierCaseParams;

#[cfg(any(test, feature = "mocks"))]
pub struct ConsumeEvmVerifierCasePayload<
    'a,
    P: IPairingArithmetic + IPairingReference,
    C: IChainAdapter,
    D: IDeliveryProofAdapter<Pairing = P, Forms = <C as IEntitlementStateAdapter>::Forms>,
> {
    pub pairing: &'a P,
    pub chain: &'a C,
    pub delivery_proof: &'a D,
}

#[cfg(any(test, feature = "mocks"))]
pub struct EvmVerifierCaseDeps<C: IEvmVerifierCaseConsumer> {
    pub consumer: C,
}

#[cfg(any(test, feature = "mocks"))]
pub struct EvmVerifierCaseParams {
    pub pairing: CreatePairingParams,
    pub hash_to_scalar: CreateHashToScalarParams,
    pub encoding: CreateEncodingParams,
    pub chain_forms: CreateChainFormsParams,
    pub chain: CreateChainParams,
    pub delivery_proof: CreateDeliveryProofParams,
    pub random_source: CreateRandomSourceParams,
}

#[cfg(any(test, feature = "mocks"))]
pub struct EvmVerifierCasePayload;

#[cfg(any(test, feature = "mocks"))]
pub struct EvmVerifierCaseSuccessReturn<O> {
    pub output: O,
}

#[cfg(any(test, feature = "mocks"))]
#[derive(Debug)]
pub enum EvmVerifierCaseErrorReturn {
    Pairing(CreatePairingErrorReturn),
    HashToScalar(CreateHashToScalarErrorReturn),
    Encoding(CreateEncodingErrorReturn),
    ChainForms(CreateChainFormsErrorReturn),
    Chain(CreateChainErrorReturn),
    DeliveryProof(CreateDeliveryProofErrorReturn),
    RandomSource(CreateRandomSourceErrorReturn),
}

#[cfg(any(test, feature = "mocks"))]
pub type EvmVerifierCaseReturn<O> =
    Result<EvmVerifierCaseSuccessReturn<O>, EvmVerifierCaseErrorReturn>;

#[cfg(any(test, feature = "mocks"))]
pub type RunEvmVerifierCaseFn<C: IEvmVerifierCaseConsumer> =
    fn(
        &EvmVerifierCaseDeps<C>,
        EvmVerifierCaseParams,
        EvmVerifierCasePayload,
    ) -> EvmVerifierCaseReturn<<C as IEvmVerifierCaseConsumer>::Output>;
