use pairing::VerifierGroupArithmetic;
use proof::{DeliveryRelation, IDeliveryProofAdapter, VerifyAdapterPayload};

use super::super::evm::provides::EvmVerifierErrorReturn;

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
