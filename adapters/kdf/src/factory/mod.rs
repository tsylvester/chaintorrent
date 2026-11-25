#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::blake3_keyed::provides::{Blake3KeyedKdf, Blake3KeyedKdfConstructorParams};
use crate::lookup_kdf_declaration::provides::{
    LookupKdfDeclarationDeps, LookupKdfDeclarationParams, LookupKdfDeclarationPayload,
};
use interface::{
    ConsumeKeyDerivationParams, ConsumeKeyDerivationPayload, CreateKeyDerivationDeps,
    CreateKeyDerivationParams, CreateKeyDerivationPayload, CreateKeyDerivationReturn,
    CreateKeyDerivationSuccessReturn, IKeyDerivationConsumer, KdfConcrete,
};
#[cfg(any(test, feature = "mocks"))]
use mock::{MockIKeyDerivationAdapter, MockIKeyDerivationAdapterConstructorParams};

pub fn create_key_derivation<C: IKeyDerivationConsumer>(
    deps: &CreateKeyDerivationDeps<C>,
    params: CreateKeyDerivationParams,
    _payload: CreateKeyDerivationPayload,
) -> CreateKeyDerivationReturn<C::Output> {
    let Ok(lookup_success) = (deps.lookup)(
        &LookupKdfDeclarationDeps,
        LookupKdfDeclarationParams {
            concrete: params.selection,
        },
        LookupKdfDeclarationPayload,
    );
    let declaration = lookup_success.declaration;
    match params.selection {
        KdfConcrete::Blake3Keyed => {
            let Ok(kdf) = Blake3KeyedKdf::try_new(Blake3KeyedKdfConstructorParams);
            Ok(CreateKeyDerivationSuccessReturn {
                output: deps.consumer.consume_key_derivation(
                    ConsumeKeyDerivationParams,
                    ConsumeKeyDerivationPayload {
                        adapter: kdf,
                        declaration,
                    },
                ),
            })
        }
        #[cfg(any(test, feature = "mocks"))]
        KdfConcrete::Mock(failure_mode) => {
            let Ok(mock) =
                MockIKeyDerivationAdapter::try_new(MockIKeyDerivationAdapterConstructorParams {
                    failure_mode,
                });
            Ok(CreateKeyDerivationSuccessReturn {
                output: deps.consumer.consume_key_derivation(
                    ConsumeKeyDerivationParams,
                    ConsumeKeyDerivationPayload {
                        adapter: mock,
                        declaration,
                    },
                ),
            })
        }
    }
}
