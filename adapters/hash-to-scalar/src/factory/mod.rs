#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::keccak256::provides::{Keccak256HashToScalar, Keccak256HashToScalarConstructorParams};
use crate::lookup_hash_to_scalar_declaration::provides::{
    LookupHashToScalarDeclarationDeps, LookupHashToScalarDeclarationParams,
    LookupHashToScalarDeclarationPayload,
};
use core::marker::PhantomData;
use interface::{
    ConsumeHashToScalarParams, ConsumeHashToScalarPayload, CreateHashToScalarDeps,
    CreateHashToScalarParams, CreateHashToScalarPayload, CreateHashToScalarReturn,
    CreateHashToScalarSuccessReturn, HashToScalarConcrete, IHashToScalarConsumer,
};
#[cfg(any(test, feature = "mocks"))]
use mock::{MockIHashToScalarAdapter, MockIHashToScalarAdapterConstructorParams};
use pairing::ISampleUniformScalar;

pub fn create_hash_to_scalar<S: ISampleUniformScalar + Clone, C: IHashToScalarConsumer<S>>(
    deps: &CreateHashToScalarDeps<C>,
    params: CreateHashToScalarParams,
    _payload: CreateHashToScalarPayload,
) -> CreateHashToScalarReturn<C::Output> {
    let Ok(looked_up) = (deps.lookup)(
        &LookupHashToScalarDeclarationDeps,
        LookupHashToScalarDeclarationParams {
            concrete: params.concrete,
        },
        LookupHashToScalarDeclarationPayload,
    );
    match params.concrete {
        HashToScalarConcrete::Keccak256 => {
            let Ok(hasher) = Keccak256HashToScalar::try_new(Keccak256HashToScalarConstructorParams);
            Ok(CreateHashToScalarSuccessReturn {
                output: deps.consumer.consume_hash_to_scalar(
                    ConsumeHashToScalarParams,
                    ConsumeHashToScalarPayload {
                        adapter: hasher,
                        declaration: looked_up.declaration,
                        scalar: PhantomData,
                    },
                ),
            })
        }
        #[cfg(any(test, feature = "mocks"))]
        HashToScalarConcrete::Mock(failure_mode) => {
            let Ok(mock) =
                MockIHashToScalarAdapter::try_new(MockIHashToScalarAdapterConstructorParams {
                    failure_mode,
                });
            Ok(CreateHashToScalarSuccessReturn {
                output: deps.consumer.consume_hash_to_scalar(
                    ConsumeHashToScalarParams,
                    ConsumeHashToScalarPayload {
                        adapter: mock,
                        declaration: looked_up.declaration,
                        scalar: PhantomData,
                    },
                ),
            })
        }
    }
}
