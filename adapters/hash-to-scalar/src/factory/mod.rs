mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::keccak256::provides::{Keccak256HashToScalar, Keccak256HashToScalarConstructorParams};
use interface::{
    CreateHashToScalarDeps, CreateHashToScalarErrorReturn, CreateHashToScalarParams,
    CreateHashToScalarPayload, CreateHashToScalarReturn, CreateHashToScalarSuccessReturn,
    HashToScalarConcrete,
};
use pairing::ISampleUniformScalar;

pub fn create_hash_to_scalar<S: ISampleUniformScalar + Clone>(
    _deps: &CreateHashToScalarDeps,
    params: CreateHashToScalarParams,
    _payload: CreateHashToScalarPayload,
) -> CreateHashToScalarReturn<S> {
    match params.concrete {
        HashToScalarConcrete::Keccak256 => {
            if Keccak256HashToScalar::DECLARATION.identifier != params.identifier {
                return Err(CreateHashToScalarErrorReturn::UnsupportedHashToScalarIdentifier);
            }
            let Ok(hasher) = Keccak256HashToScalar::try_new(Keccak256HashToScalarConstructorParams);
            Ok(CreateHashToScalarSuccessReturn {
                adapter: Box::new(hasher),
                declaration: Keccak256HashToScalar::DECLARATION,
            })
        }
    }
}
