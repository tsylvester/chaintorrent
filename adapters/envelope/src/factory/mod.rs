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
use interface::{
    ConsumeKeyAgreementParams, ConsumeKeyAgreementPayload, CreateKeyAgreementDeps,
    CreateKeyAgreementErrorReturn, CreateKeyAgreementParams, CreateKeyAgreementPayload,
    CreateKeyAgreementReturn, CreateKeyAgreementSuccessReturn, ENVELOPE_ALGEBRAS, EnvelopeAlgebra,
    EnvelopeAlgebraConstructorParams, EnvelopeAlgebraTryNewErrorReturn,
    EnvelopeAlgebraTryNewReturn, IKeyAgreementConsumer, KEY_AGREEMENT_CONCRETES,
    KEY_AGREEMENT_IDENTIFIERS, KeyAgreementConcrete, KeyAgreementConcreteConstructorParams,
    KeyAgreementConcreteTryNewErrorReturn, KeyAgreementConcreteTryNewReturn,
    KeyAgreementIdentifier, KeyAgreementIdentifierConstructorParams,
    KeyAgreementIdentifierTryNewErrorReturn, KeyAgreementIdentifierTryNewReturn,
};
#[cfg(any(test, feature = "mocks"))]
use mock::{MockIKeyAgreementAdapter, MockIKeyAgreementAdapterConstructorParams};
use pairing::IPairingArithmetic;

impl KeyAgreementConcrete {
    pub fn try_new(
        params: KeyAgreementConcreteConstructorParams,
    ) -> KeyAgreementConcreteTryNewReturn {
        for concrete in KEY_AGREEMENT_CONCRETES {
            if params.text == concrete.name() {
                return Ok(*concrete);
            }
        }
        Err(KeyAgreementConcreteTryNewErrorReturn::Unknown { text: params.text })
    }
}

impl KeyAgreementIdentifier {
    pub fn try_new(
        params: KeyAgreementIdentifierConstructorParams,
    ) -> KeyAgreementIdentifierTryNewReturn {
        for identifier in KEY_AGREEMENT_IDENTIFIERS {
            if params.text == identifier.name() {
                return Ok(*identifier);
            }
        }
        Err(KeyAgreementIdentifierTryNewErrorReturn::Unknown { text: params.text })
    }
}

impl EnvelopeAlgebra {
    pub fn try_new(params: EnvelopeAlgebraConstructorParams) -> EnvelopeAlgebraTryNewReturn {
        for algebra in ENVELOPE_ALGEBRAS {
            if params.text == algebra.name() {
                return Ok(*algebra);
            }
        }
        Err(EnvelopeAlgebraTryNewErrorReturn::Unknown { text: params.text })
    }
}

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
