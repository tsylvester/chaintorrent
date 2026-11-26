#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::admit_kem_composition::provides::{
    AdmitKemCompositionDeps, AdmitKemCompositionParams, AdmitKemCompositionPayload,
};
use crate::bb1_depth_one::provides::{Bb1DepthOneKem, Bb1DepthOneKemConstructorParams};
#[cfg(any(test, feature = "mocks"))]
use hash_to_scalar::{DomainTag, DomainTagConstructorParams};
use interface::{
    ConsumeKemParams, ConsumeKemPayload, CreateKemDeps, CreateKemErrorReturn, CreateKemParams,
    CreateKemPayload, CreateKemReturn, CreateKemSuccessReturn, IKemConsumer, KemConcrete,
};
#[cfg(any(test, feature = "mocks"))]
use mock::{
    MOCK_I_CREDENTIAL_KEM_ADAPTER_IDENTITY_TAG, MockICredentialKemAdapter,
    MockICredentialKemAdapterConstructorParams,
};
use pairing::IPairingArithmetic;

pub fn create_kem<'a, P: IPairingArithmetic, C: IKemConsumer<P>>(
    deps: &CreateKemDeps<'a, P, C>,
    params: CreateKemParams,
    _payload: CreateKemPayload,
) -> CreateKemReturn<C::Output> {
    if let Err(error) = (deps.admit_kem_composition)(
        &AdmitKemCompositionDeps {
            lookup_kem_declaration: deps.lookup_kem_declaration,
        },
        AdmitKemCompositionParams {
            concrete: params.concrete,
            scope: params.scope,
        },
        AdmitKemCompositionPayload,
    ) {
        return Err(CreateKemErrorReturn::Composition(error));
    }
    match params.concrete {
        KemConcrete::Bb1DepthOne => {
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
            let tag = match DomainTag::try_new(DomainTagConstructorParams {
                bytes: MOCK_I_CREDENTIAL_KEM_ADAPTER_IDENTITY_TAG.to_vec(),
            }) {
                Ok(tag) => tag,
                Err(error) => {
                    return Err(CreateKemErrorReturn::MockICredentialKemAdapter(error));
                }
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
