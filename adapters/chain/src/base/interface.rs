use core::marker::PhantomData;
use std::io;

use super::settings::provides::BaseChainSettings;
use crate::factory::provides::IChainForms;
use alloy::providers::DynProvider;
use alloy::transports::TransportError;
#[cfg(any(test, feature = "mocks"))]
use alloy::transports::mock::Asserter;
use encoding::FromFieldErrorReturn;
use tokio::runtime::Runtime;

pub struct BaseChain<F: IChainForms> {
    pub(super) endpoint_provider: DynProvider,
    pub(super) settlement_provider: DynProvider,
    pub(super) runtime: Runtime,
    pub(super) settings: BaseChainSettings,
    pub(super) forms: PhantomData<F>,
}

pub struct BaseChainConstructorParams<'a> {
    pub settings: &'a BaseChainSettings,
}

#[cfg(any(test, feature = "mocks"))]
pub struct BaseChainMockedConstructorParams<'a> {
    pub settings: &'a BaseChainSettings,
    pub endpoint_transport: Asserter,
    pub settlement_transport: Asserter,
}

#[derive(Debug)]
pub enum BaseChainTryNewErrorReturn {
    Runtime(io::Error),
    Endpoint(TransportError),
    SettlementEndpoint(TransportError),
}

pub type BaseChainTryNewReturn<F> = Result<BaseChain<F>, BaseChainTryNewErrorReturn>;

#[derive(Debug)]
pub enum BaseCurrentReferenceAtErrorReturn {
    Transport(TransportError),
    AnchorDecode,
}

#[derive(Debug)]
pub enum BaseTierOfErrorReturn {
    Transport(TransportError),
    AnchorDecode,
}

#[derive(Debug)]
pub enum BaseReferenceAgeErrorReturn {
    Transport(TransportError),
}

#[derive(Debug)]
pub enum BaseEvaluateAuthorizationErrorReturn {
    Transport(TransportError),
    Decode,
    UnknownAuthorizationState { value: u8 },
}

#[derive(Debug)]
pub enum BaseEvaluateAuthorizationBatchErrorReturn {
    Transport(TransportError),
    Decode,
    UnknownAuthorizationState { value: u8 },
    ResultCount { expected: u64, actual: u64 },
}

#[derive(Debug)]
pub enum BaseEstimateFeeErrorReturn {
    Transport(TransportError),
    Decode,
    FeeOutOfRange { fee: [u8; 32] },
}

#[derive(Debug)]
pub enum BaseRecoverEnvelopeErrorReturn {
    Transport(TransportError),
    Decode,
    CalldataDisagrees,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BaseChainIdentifierErrorReturn {
    Form(FromFieldErrorReturn),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BaseEntitlementContractErrorReturn {
    Form(FromFieldErrorReturn),
}
