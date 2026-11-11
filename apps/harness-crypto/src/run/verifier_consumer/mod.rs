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

use envelope::IKeyAgreementAdapter;
use kem::ICredentialKemAdapter;
use pairing::IPairingAdapter;
use proof::IDeliveryProofAdapter;

use crate::measure::provides::{MeasureDeps, MeasureParams, MeasurePayload, measure};
use crate::verifier::provides::{
    ConsumeVerifierParams, ConsumeVerifierPayload, IVerifierAdapter, IVerifierConsumer,
};
use interface::{
    VerifierConsumer, VerifierConsumerConstructorParams, VerifierConsumerReturn,
    VerifierConsumerTryNewReturn,
};

impl<'a, P, K, A, D> VerifierConsumer<'a, P, K, A, D>
where
    P: IPairingAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
{
    pub fn try_new(
        params: VerifierConsumerConstructorParams<'a, P, K, A, D>,
    ) -> VerifierConsumerTryNewReturn<'a, P, K, A, D> {
        Ok(VerifierConsumer {
            pairing: params.pairing,
            kem: params.kem,
            key_agreement: params.key_agreement,
            delivery_proof: params.delivery_proof,
            random: params.random,
            mint: params.mint,
            transfer: params.transfer,
            identity: params.identity,
            iterations: params.iterations,
        })
    }
}

impl<'a, P, K, A, D> IVerifierConsumer<D> for VerifierConsumer<'a, P, K, A, D>
where
    P: IPairingAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
{
    type Output = VerifierConsumerReturn;

    fn consume_verifier<V: IVerifierAdapter<DeliveryProof = D>>(
        &self,
        _params: ConsumeVerifierParams,
        payload: ConsumeVerifierPayload<V>,
    ) -> Self::Output {
        measure(
            &MeasureDeps {
                pairing: self.pairing,
                kem: self.kem,
                key_agreement: self.key_agreement,
                delivery_proof: self.delivery_proof,
                verifier: &payload.adapter,
                random: self.random,
            },
            MeasureParams {
                iterations: self.iterations,
            },
            MeasurePayload {
                mint: &self.mint,
                transfer: &self.transfer,
                identity: self.identity,
            },
        )
    }
}
