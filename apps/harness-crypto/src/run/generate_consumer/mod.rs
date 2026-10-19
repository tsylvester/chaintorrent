#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::generate::provides::{
    ConsumeGenerateParams, ConsumeGeneratePayload, EmitParams, EmitPayload, EmitReturn,
    IGenerateAdapter, IGenerateConsumer,
};
use interface::{
    GenerateConsumer, GenerateConsumerConstructorParams, GenerateConsumerTryNewReturn,
};

impl<'a> GenerateConsumer<'a> {
    pub fn try_new(
        params: GenerateConsumerConstructorParams<'a>,
    ) -> GenerateConsumerTryNewReturn<'a> {
        Ok(GenerateConsumer {
            statements: params.statements,
        })
    }
}

impl IGenerateConsumer for GenerateConsumer<'_> {
    type Output = EmitReturn;

    fn consume_generate<G: IGenerateAdapter>(
        &self,
        _params: ConsumeGenerateParams,
        payload: ConsumeGeneratePayload<G>,
    ) -> Self::Output {
        payload.adapter.emit(
            EmitParams,
            EmitPayload {
                mint: &self.statements.mint,
                mint_purpose: self.statements.mint_purpose,
                transfer: &self.statements.transfer,
                transfer_purpose: self.statements.transfer_purpose,
                identities: &self.statements.identities,
                proof_identity: &self.statements.proof_identity,
            },
        )
    }
}
