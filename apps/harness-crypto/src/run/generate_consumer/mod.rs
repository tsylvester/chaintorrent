#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test_basic;
#[cfg(test)]
mod test_error;

use chain::IChainForms;

use crate::generate::provides::{
    ConsumeGenerateParams, ConsumeGeneratePayload, EmitParams, EmitPayload, IGenerateAdapter,
    IGenerateConsumer,
};
use interface::{
    GenerateConsumer, GenerateConsumerConstructorParams, GenerateConsumerReturn,
    GenerateConsumerTryNewReturn,
};

impl<'a, F: IChainForms> GenerateConsumer<'a, F> {
    pub fn try_new(
        params: GenerateConsumerConstructorParams<'a, F>,
    ) -> GenerateConsumerTryNewReturn<'a, F> {
        Ok(GenerateConsumer {
            mint: params.mint,
            transfer: params.transfer,
            identities: params.identities,
            proof_identity: params.proof_identity,
        })
    }
}

impl<'a, F: IChainForms> IGenerateConsumer<F> for GenerateConsumer<'a, F> {
    type Output = GenerateConsumerReturn;

    fn consume_generate<G: IGenerateAdapter<Forms = F>>(
        &self,
        _params: ConsumeGenerateParams,
        payload: ConsumeGeneratePayload<G>,
    ) -> Self::Output {
        payload.adapter.emit(
            EmitParams,
            EmitPayload {
                mint: &self.mint,
                transfer: &self.transfer,
                identities: &self.identities,
                proof_identity: self.proof_identity,
            },
        )
    }
}
