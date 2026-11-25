#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::admit_hashing_composition::provides::{
    AdmitHashingCompositionDeps, AdmitHashingCompositionParams, AdmitHashingCompositionPayload,
};
use crate::blake3_bao::provides::{Blake3BaoCommitment, Blake3BaoCommitmentConstructorParams};
use crate::lookup_hashing_declaration::provides::{
    LookupHashingDeclarationDeps, LookupHashingDeclarationParams, LookupHashingDeclarationPayload,
};
use interface::{
    CommitmentConcrete, ConsumeCommitmentParams, ConsumeCommitmentPayload, CreateCommitmentDeps,
    CreateCommitmentErrorReturn, CreateCommitmentParams, CreateCommitmentPayload,
    CreateCommitmentReturn, CreateCommitmentSuccessReturn, ICommitmentConsumer, Root,
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

pub fn create_commitment<C: ICommitmentConsumer>(
    deps: &CreateCommitmentDeps<C>,
    params: CreateCommitmentParams,
    _payload: CreateCommitmentPayload,
) -> CreateCommitmentReturn<C::Output> {
    match (deps.admit_hashing_composition)(
        &AdmitHashingCompositionDeps {
            lookup_hashing_declaration: deps.lookup_hashing_declaration,
        },
        AdmitHashingCompositionParams {
            concrete: params.concrete,
            identifier: params.identifier,
        },
        AdmitHashingCompositionPayload,
    ) {
        Err(error) => Err(CreateCommitmentErrorReturn::Composition(error)),
        Ok(_) => {
            let Ok(lookup_success) = (deps.lookup_hashing_declaration)(
                &LookupHashingDeclarationDeps,
                LookupHashingDeclarationParams {
                    concrete: params.concrete,
                },
                LookupHashingDeclarationPayload,
            );
            let declaration = lookup_success.declaration;
            match params.concrete {
                CommitmentConcrete::Blake3Bao => {
                    let Ok(commitment) =
                        Blake3BaoCommitment::try_new(Blake3BaoCommitmentConstructorParams);
                    Ok(CreateCommitmentSuccessReturn {
                        output: deps.consumer.consume_commitment(
                            ConsumeCommitmentParams,
                            ConsumeCommitmentPayload {
                                adapter: commitment,
                                declaration,
                            },
                        ),
                    })
                }
                #[cfg(any(test, feature = "mocks"))]
                CommitmentConcrete::Mock(failure_mode) => {
                    let Ok(mock) =
                        MockICommitmentAdapter::try_new(MockICommitmentAdapterConstructorParams {
                            failure_mode,
                        });
                    Ok(CreateCommitmentSuccessReturn {
                        output: deps.consumer.consume_commitment(
                            ConsumeCommitmentParams,
                            ConsumeCommitmentPayload {
                                adapter: mock,
                                declaration,
                            },
                        ),
                    })
                }
            }
        }
    }
}
