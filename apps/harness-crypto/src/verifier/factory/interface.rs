use pairing::{IPairingAdapter, VerifierGroupArithmetic};
use proof::{DeliveryRelation, IDeliveryProofAdapter, VerifyAdapterPayload};

use super::super::evm::provides::{
    EvmVerifierErrorReturn, EvmVerifierSettings, EvmVerifierTryNewErrorReturn,
};

pub const VERIFIER_INTERFACE_VERSION: u32 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VerifierDeclaration {
    pub verifier_forms: &'static [VerifierGroupArithmetic],
    pub statement_versions: &'static [u16],
    pub adapter_version: u32,
    pub interface_version: u32,
}

pub struct VerifyOnChainParams;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1DataFee {
    Charged { wei: u128 },
    NotApplicable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VerifyCost {
    pub l2_execution_gas: u64,
    pub l1_data_fee: L1DataFee,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VerifyOnChainSuccessReturn {
    Accepted { cost: VerifyCost },
    Rejected,
    Reverted { data: Vec<u8> },
}

#[derive(Debug)]
pub enum VerifyOnChainErrorReturn {
    RelationMismatch {
        statement: DeliveryRelation,
        proof: DeliveryRelation,
    },
    Evm(EvmVerifierErrorReturn),
    #[cfg(any(test, feature = "mocks"))]
    Mock(MockIVerifierAdapterVerifyOnChainErrorReturn),
}

#[cfg(any(test, feature = "mocks"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MockIVerifierAdapterFailureMode {
    NoFailure,
    RpcRefused,
}

#[cfg(any(test, feature = "mocks"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MockIVerifierAdapterVerifyOnChainErrorReturn {
    RpcRefused,
}

pub type VerifyOnChainReturn = Result<VerifyOnChainSuccessReturn, VerifyOnChainErrorReturn>;

pub trait IVerifierAdapter {
    const DECLARATION: VerifierDeclaration;

    type DeliveryProof: IDeliveryProofAdapter;

    fn verify_on_chain(
        &self,
        params: VerifyOnChainParams,
        payload: VerifyAdapterPayload<'_, Self::DeliveryProof>,
    ) -> VerifyOnChainReturn;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VerifierConcreteKind {
    Evm,
}

pub const VERIFIER_CONCRETES: &[VerifierConcreteKind] = &[VerifierConcreteKind::Evm];

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VerifierConcrete {
    Evm {
        settings: Box<EvmVerifierSettings>,
    },
    #[cfg(any(test, feature = "mocks"))]
    Mock(MockIVerifierAdapterFailureMode),
}

pub trait IVerifierConsumer<D: IDeliveryProofAdapter> {
    type Output;

    fn consume_verifier<V: IVerifierAdapter<DeliveryProof = D>>(
        &self,
        params: ConsumeVerifierParams,
        payload: ConsumeVerifierPayload<V>,
    ) -> Self::Output;
}

pub struct ConsumeVerifierParams;

pub struct ConsumeVerifierPayload<V: IVerifierAdapter> {
    pub adapter: V,
}

pub struct CreateVerifierDeps<'a, P: IPairingAdapter, D: IDeliveryProofAdapter<Pairing = P>, C> {
    pub pairing: &'a P,
    pub delivery_proof: &'a D,
    pub consumer: C,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreateVerifierParams {
    pub concrete: VerifierConcrete,
    pub statement_version: u16,
}

pub struct CreateVerifierPayload;

pub struct CreateVerifierSuccessReturn<O> {
    pub output: O,
}

#[derive(Debug)]
pub enum CreateVerifierErrorReturn {
    UnsupportedVerifierForm { form: VerifierGroupArithmetic },
    UnsupportedStatementVersion { version: u16 },
    Evm(EvmVerifierTryNewErrorReturn),
}

pub type CreateVerifierReturn<O> =
    Result<CreateVerifierSuccessReturn<O>, CreateVerifierErrorReturn>;

pub type CreateVerifierFn<'a, P, D, C> =
    fn(
        &CreateVerifierDeps<'a, P, D, C>,
        CreateVerifierParams,
        CreateVerifierPayload,
    ) -> CreateVerifierReturn<<C as IVerifierConsumer<D>>::Output>;
