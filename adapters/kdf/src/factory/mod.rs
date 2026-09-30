mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::blake3_keyed::provides::{Blake3KeyedKdf, Blake3KeyedKdfConstructorParams};
use interface::{
    CreateKeyDerivationDeps, CreateKeyDerivationErrorReturn, CreateKeyDerivationParams,
    CreateKeyDerivationPayload, CreateKeyDerivationReturn, CreateKeyDerivationSuccessReturn,
    KdfConcrete,
};

pub fn create_key_derivation(
    _deps: &CreateKeyDerivationDeps,
    params: CreateKeyDerivationParams,
    _payload: CreateKeyDerivationPayload,
) -> CreateKeyDerivationReturn {
    match params.concrete {
        KdfConcrete::Blake3Keyed => {
            if Blake3KeyedKdf::DECLARATION.identifier != params.identifier {
                return Err(CreateKeyDerivationErrorReturn::UnsupportedKdfIdentifier);
            }
            let Ok(kdf) = Blake3KeyedKdf::try_new(Blake3KeyedKdfConstructorParams);
            Ok(CreateKeyDerivationSuccessReturn {
                adapter: Box::new(kdf),
                declaration: Blake3KeyedKdf::DECLARATION,
            })
        }
    }
}
