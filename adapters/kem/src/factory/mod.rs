mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use pairing::IPairingArithmetic;

use self::interface::{
    ConsumeKemParams, ConsumeKemPayload, CreateKemDeps, CreateKemErrorReturn, CreateKemParams,
    CreateKemPayload, CreateKemReturn, CreateKemSuccessReturn, IKemConsumer, KemConcrete,
};
use crate::bb1_depth_one::provides::{Bb1DepthOneKem, Bb1DepthOneKemConstructorParams};

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
    }
}
