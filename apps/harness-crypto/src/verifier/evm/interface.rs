use alloy::primitives::Address;
use alloy::providers::DynProvider;
use alloy::transports::TransportError;
#[cfg(any(test, feature = "mocks"))]
use chain::{CreateChainFormsErrorReturn, CreateChainFormsParams, IChainForms};
use encoding::{CanonicalFieldKind, CanonicalFieldValue, FromFieldErrorReturn};
#[cfg(any(test, feature = "mocks"))]
use encoding::{CreateEncodingErrorReturn, CreateEncodingParams};
#[cfg(any(test, feature = "mocks"))]
use hash_to_scalar::{CreateHashToScalarErrorReturn, CreateHashToScalarParams};
#[cfg(any(test, feature = "mocks"))]
use pairing::{
    CreatePairingErrorReturn, CreatePairingParams, IPairingArithmetic, IPairingReference,
};
use pairing::{IPairingAdapter, VerifierGroupArithmetic};
#[cfg(any(test, feature = "mocks"))]
use proof::{CreateDeliveryProofErrorReturn, CreateDeliveryProofParams};
use proof::{DELIVERY_STATEMENT_VERSION_ONE, IDeliveryProofAdapter};
#[cfg(any(test, feature = "mocks"))]
use random::{CreateRandomSourceErrorReturn, CreateRandomSourceParams};
use tokio::runtime::Runtime;
use url::{ParseError, Url};

use super::super::factory::provides::{VERIFIER_INTERFACE_VERSION, VerifierDeclaration};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EvmVerifierEndpoint {
    pub(super) url: Url,
}

pub struct EvmVerifierEndpointConstructorParams {
    pub text: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EvmVerifierEndpointTryNewErrorReturn {
    Unparsable { error: ParseError },
}

pub type EvmVerifierEndpointTryNewReturn =
    Result<EvmVerifierEndpoint, EvmVerifierEndpointTryNewErrorReturn>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EvmVerifierSettings {
    pub endpoint: EvmVerifierEndpoint,
    pub verifier_address: CanonicalFieldValue,
    pub gas_price_oracle: CanonicalFieldValue,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EvmVerifierField {
    VerifierAddress,
    GasPriceOracle,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EvmVerifierContextField {
    ChainIdentifier,
    EntitlementContract,
    SourceEntitlement,
    TargetEntitlement,
    OldInterval,
    NewInterval,
    Seller,
    Buyer,
}

pub struct EvmVerifierConstructorParams<
    'a,
    P: IPairingAdapter,
    D: IDeliveryProofAdapter<Pairing = P>,
> {
    pub pairing: &'a P,
    pub delivery_proof: &'a D,
    pub settings: EvmVerifierSettings,
}

pub struct EvmVerifier<'a, P: IPairingAdapter, D: IDeliveryProofAdapter<Pairing = P>> {
    pub(super) pairing: &'a P,
    pub(super) delivery_proof: &'a D,
    pub(super) verifier: Address,
    pub(super) oracle: Address,
    pub(super) provider: DynProvider,
    pub(super) runtime: Runtime,
}

pub type EvmVerifierTryNewReturn<'a, P, D> =
    Result<EvmVerifier<'a, P, D>, EvmVerifierTryNewErrorReturn>;

#[derive(Debug)]
pub enum EvmVerifierTryNewErrorReturn {
    Identity {
        field: EvmVerifierField,
        error: FromFieldErrorReturn,
    },
    IdentityKind {
        field: EvmVerifierField,
    },
    Runtime(std::io::Error),
}

#[derive(Debug)]
pub enum EvmVerifierErrorReturn {
    NothingDeployed {
        address: [u8; 20],
    },
    ContextFieldKind {
        field: EvmVerifierContextField,
        expected: CanonicalFieldKind,
    },
    ScalarLength {
        actual: usize,
    },
    Rpc(TransportError),
    ResultDecode(alloy::sol_types::Error),
    L1FeeOutOfRange {
        fee: [u8; 32],
    },
}

pub const EVM_VERIFIER_DECLARATION: VerifierDeclaration = VerifierDeclaration {
    verifier_forms: &[
        VerifierGroupArithmetic::FirstGroupOnly,
        VerifierGroupArithmetic::BothGroups,
    ],
    statement_versions: &[DELIVERY_STATEMENT_VERSION_ONE],
    adapter_version: 1,
    interface_version: VERIFIER_INTERFACE_VERSION,
};

#[cfg(any(test, feature = "mocks"))]
pub trait IEvmVerifierCaseConsumer {
    type Output;

    fn consume_evm_verifier_case<P, F, D>(
        &self,
        pairing: &P,
        forms: &F,
        delivery_proof: &D,
    ) -> Self::Output
    where
        P: IPairingArithmetic + IPairingReference,
        F: IChainForms,
        D: IDeliveryProofAdapter<Pairing = P, Forms = F>;
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
#[derive(Debug, PartialEq, Eq)]
pub enum EvmVerifierCaseErrorReturn {
    CreatePairing(CreatePairingErrorReturn),
    CreateHashToScalar(CreateHashToScalarErrorReturn),
    CreateEncoding(CreateEncodingErrorReturn),
    CreateChainForms(CreateChainFormsErrorReturn),
    CreateDeliveryProof(CreateDeliveryProofErrorReturn),
    CreateRandomSource(CreateRandomSourceErrorReturn),
}

#[cfg(any(test, feature = "mocks"))]
pub type EvmVerifierCaseReturn<O> =
    Result<EvmVerifierCaseSuccessReturn<O>, EvmVerifierCaseErrorReturn>;

#[cfg(any(test, feature = "mocks"))]
pub type RunEvmVerifierCaseFn<C> =
    fn(
        &EvmVerifierCaseDeps<C>,
        EvmVerifierCaseParams,
        EvmVerifierCasePayload,
    ) -> EvmVerifierCaseReturn<<C as IEvmVerifierCaseConsumer>::Output>;
