#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::blake3_keyed::provides::{Blake3KeyedKdf, Blake3KeyedKdfConstructorParams};
use interface::{
    CreateKeyDerivationDeps, CreateKeyDerivationParams, CreateKeyDerivationPayload,
    CreateKeyDerivationReturn, CreateKeyDerivationSuccessReturn, KdfConcrete,
};
#[cfg(any(test, feature = "mocks"))]
use mock::{MockIKeyDerivationAdapter, MockIKeyDerivationAdapterConstructorParams};

pub fn create_key_derivation(
    _deps: &CreateKeyDerivationDeps,
    params: CreateKeyDerivationParams,
    _payload: CreateKeyDerivationPayload,
) -> CreateKeyDerivationReturn {
    match params.selection {
        KdfConcrete::Blake3Keyed => {
            let Ok(kdf) = Blake3KeyedKdf::try_new(Blake3KeyedKdfConstructorParams);
            Ok(CreateKeyDerivationSuccessReturn {
                adapter: Box::new(kdf),
            })
        }
        #[cfg(any(test, feature = "mocks"))]
        KdfConcrete::Mock(failure_mode) => {
            let Ok(mock) =
                MockIKeyDerivationAdapter::try_new(MockIKeyDerivationAdapterConstructorParams {
                    failure_mode,
                });
            Ok(CreateKeyDerivationSuccessReturn {
                adapter: Box::new(mock),
            })
        }
    }
}
