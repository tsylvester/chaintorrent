#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use pairing::IPairingArithmetic;

#[cfg(any(test, feature = "mocks"))]
use self::interface::ICredentialKemAdapter;
use self::interface::{
    ConsumeKemParams, ConsumeKemPayload, CreateKemDeps, CreateKemErrorReturn, CreateKemParams,
    CreateKemPayload, CreateKemReturn, CreateKemSuccessReturn, IKemConsumer, KemConcrete,
};
#[cfg(any(test, feature = "mocks"))]
use self::mock::{
    MOCK_I_CREDENTIAL_KEM_ADAPTER_IDENTITY_TAG, MockICredentialKemAdapter,
    MockICredentialKemAdapterConstructorParams,
};
use crate::bb1_depth_one::provides::{Bb1DepthOneKem, Bb1DepthOneKemConstructorParams};
#[cfg(any(test, feature = "mocks"))]
use hash_to_scalar::{DomainTag, DomainTagConstructorParams};

pub fn create_kem<'a, P: IPairingArithmetic, C: IKemConsumer<P>>(
    deps: &CreateKemDeps<'a, P, C>,
    params: CreateKemParams,
    _payload: CreateKemPayload,
) -> CreateKemReturn<C::Output> {
    match params.concrete {
        KemConcrete::Bb1DepthOne => {
            if Bb1DepthOneKem::<'_, P>::DECLARATION.identifier != params.identifier {
                return Err(CreateKemErrorReturn::UnsupportedKemIdentifier);
            }
            if !Bb1DepthOneKem::<'_, P>::DECLARATION
                .identity_scopes
                .contains(&params.scope)
            {
                return Err(CreateKemErrorReturn::UnsupportedIdentityScope);
            }
            let adapter = match Bb1DepthOneKem::try_new(Bb1DepthOneKemConstructorParams {
                pairing: deps.pairing,
                hash_to_scalar: deps.hash_to_scalar,
            }) {
                Ok(adapter) => adapter,
                Err(error) => return Err(CreateKemErrorReturn::Bb1DepthOne(error)),
            };
            let output = deps.consumer.consume_kem(
                ConsumeKemParams,
                ConsumeKemPayload {
                    adapter,
                    scope: params.scope,
                },
            );
            Ok(CreateKemSuccessReturn { output })
        }
        #[cfg(any(test, feature = "mocks"))]
        KemConcrete::Mock => {
            if MockICredentialKemAdapter::<'_, P>::DECLARATION.identifier != params.identifier {
                return Err(CreateKemErrorReturn::UnsupportedKemIdentifier);
            }
            if !MockICredentialKemAdapter::<'_, P>::DECLARATION
                .identity_scopes
                .contains(&params.scope)
            {
                return Err(CreateKemErrorReturn::UnsupportedIdentityScope);
            }
            let tag = match DomainTag::try_new(DomainTagConstructorParams {
                bytes: MOCK_I_CREDENTIAL_KEM_ADAPTER_IDENTITY_TAG.to_vec(),
            }) {
                Ok(tag) => tag,
                Err(error) => return Err(CreateKemErrorReturn::MockICredentialKemAdapter(error)),
            };
            let Ok(adapter) =
                MockICredentialKemAdapter::try_new(MockICredentialKemAdapterConstructorParams {
                    pairing: deps.pairing,
                    hash_to_scalar: deps.hash_to_scalar,
                    tag,
                });
            let output = deps.consumer.consume_kem(
                ConsumeKemParams,
                ConsumeKemPayload {
                    adapter,
                    scope: params.scope,
                },
            );
            Ok(CreateKemSuccessReturn { output })
        }
    }
}
