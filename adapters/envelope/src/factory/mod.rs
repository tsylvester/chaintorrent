#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::pairing_elgamal::provides::{
    PairingElGamalKeyAgreement, PairingElGamalKeyAgreementConstructorParams,
};
use encoding::IEncoderAdapter;
#[cfg(any(test, feature = "mocks"))]
use interface::IKeyAgreementAdapter;
use interface::{
    ConsumeKeyAgreementParams, ConsumeKeyAgreementPayload, CreateKeyAgreementDeps,
    CreateKeyAgreementErrorReturn, CreateKeyAgreementParams, CreateKeyAgreementPayload,
    CreateKeyAgreementReturn, CreateKeyAgreementSuccessReturn, IKeyAgreementConsumer,
    KeyAgreementConcrete,
};
#[cfg(any(test, feature = "mocks"))]
use mock::{MockIKeyAgreementAdapter, MockIKeyAgreementAdapterConstructorParams};
use pairing::IPairingArithmetic;

pub fn create_key_agreement<
    'a,
    P: IPairingArithmetic,
    E: IEncoderAdapter,
    C: IKeyAgreementConsumer<P>,
>(
    deps: &CreateKeyAgreementDeps<'a, P, E, C>,
    params: CreateKeyAgreementParams,
    _payload: CreateKeyAgreementPayload,
) -> CreateKeyAgreementReturn<C::Output> {
    match params.concrete {
        KeyAgreementConcrete::PairingElGamal => {
            if PairingElGamalKeyAgreement::<'_, P, E>::DECLARATION.identifier != params.identifier {
                return Err(CreateKeyAgreementErrorReturn::UnsupportedKeyAgreementIdentifier);
            }
            if PairingElGamalKeyAgreement::<'_, P, E>::DECLARATION.algebra != params.algebra {
                return Err(CreateKeyAgreementErrorReturn::UnsupportedEnvelopeAlgebra);
            }
            let adapter = match PairingElGamalKeyAgreement::try_new(
                PairingElGamalKeyAgreementConstructorParams {
                    pairing: deps.pairing,
                    hash_to_scalar: deps.hash_to_scalar,
                    encoder: deps.encoder,
                    random: deps.random,
                },
            ) {
                Ok(adapter) => adapter,
                Err(error) => return Err(CreateKeyAgreementErrorReturn::PairingElGamal(error)),
            };
            let output = deps.consumer.consume_key_agreement(
                ConsumeKeyAgreementParams,
                ConsumeKeyAgreementPayload { adapter },
            );
            Ok(CreateKeyAgreementSuccessReturn { output })
        }
        #[cfg(any(test, feature = "mocks"))]
        KeyAgreementConcrete::Mock(failure_mode) => {
            if MockIKeyAgreementAdapter::<'_, P>::DECLARATION.identifier != params.identifier {
                return Err(CreateKeyAgreementErrorReturn::UnsupportedKeyAgreementIdentifier);
            }
            if MockIKeyAgreementAdapter::<'_, P>::DECLARATION.algebra != params.algebra {
                return Err(CreateKeyAgreementErrorReturn::UnsupportedEnvelopeAlgebra);
            }
            let Ok(adapter) =
                MockIKeyAgreementAdapter::try_new(MockIKeyAgreementAdapterConstructorParams {
                    pairing: deps.pairing,
                    failure_mode,
                });
            let output = deps.consumer.consume_key_agreement(
                ConsumeKeyAgreementParams,
                ConsumeKeyAgreementPayload { adapter },
            );
            Ok(CreateKeyAgreementSuccessReturn { output })
        }
    }
}
