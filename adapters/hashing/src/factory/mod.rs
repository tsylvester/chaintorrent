#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::blake3_bao::provides::{Blake3BaoCommitment, Blake3BaoCommitmentConstructorParams};
use interface::{
    CommitmentConcrete, CreateCommitmentDeps, CreateCommitmentErrorReturn, CreateCommitmentParams,
    CreateCommitmentPayload, CreateCommitmentReturn, CreateCommitmentSuccessReturn, Root,
    RootConstructorParams, RootTryNewErrorReturn, RootTryNewReturn,
};
#[cfg(any(test, feature = "mocks"))]
use mock::{MockICommitmentAdapter, MockICommitmentAdapterConstructorParams};

impl Root {
    pub fn try_new(params: RootConstructorParams) -> RootTryNewReturn {
        if params.bytes.iter().all(|byte| *byte == 0) {
            return Err(RootTryNewErrorReturn::AllZero);
        }
        Ok(Root {
            bytes: params.bytes,
        })
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.bytes
    }
}

pub fn create_commitment(
    _deps: &CreateCommitmentDeps,
    params: CreateCommitmentParams,
    _payload: CreateCommitmentPayload,
) -> CreateCommitmentReturn {
    match params.concrete {
        CommitmentConcrete::Blake3Bao => {
            if Blake3BaoCommitment::DECLARATION.identifier != params.identifier {
                return Err(CreateCommitmentErrorReturn::UnsupportedCommitmentScheme);
            }
            let Ok(commitment) = Blake3BaoCommitment::try_new(Blake3BaoCommitmentConstructorParams);
            Ok(CreateCommitmentSuccessReturn {
                adapter: Box::new(commitment),
            })
        }
        #[cfg(any(test, feature = "mocks"))]
        CommitmentConcrete::Mock(failure_mode) => {
            if MockICommitmentAdapter::DECLARATION.identifier != params.identifier {
                return Err(CreateCommitmentErrorReturn::UnsupportedCommitmentScheme);
            }
            let Ok(mock) =
                MockICommitmentAdapter::try_new(MockICommitmentAdapterConstructorParams {
                    failure_mode,
                });
            Ok(CreateCommitmentSuccessReturn {
                adapter: Box::new(mock),
            })
        }
    }
}
