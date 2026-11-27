mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::factory::provides::KeyAgreementConcrete;
#[cfg(any(test, feature = "mocks"))]
use crate::factory::provides::{KeyAgreementDeclaration, MOCK_I_KEY_AGREEMENT_ADAPTER_DECLARATION};
use crate::pairing_elgamal::provides::PAIRING_ELGAMAL_DECLARATION;
use interface::{
    LookupEnvelopeDeclarationDeps, LookupEnvelopeDeclarationParams,
    LookupEnvelopeDeclarationPayload, LookupEnvelopeDeclarationReturn,
    LookupEnvelopeDeclarationSuccessReturn,
};

pub fn lookup_envelope_declaration(
    _deps: &LookupEnvelopeDeclarationDeps,
    params: LookupEnvelopeDeclarationParams,
    _payload: LookupEnvelopeDeclarationPayload,
) -> LookupEnvelopeDeclarationReturn {
    let declaration = match params.concrete {
        KeyAgreementConcrete::PairingElGamal => PAIRING_ELGAMAL_DECLARATION,
        #[cfg(any(test, feature = "mocks"))]
        KeyAgreementConcrete::Mock(failure_mode) => KeyAgreementDeclaration {
            concrete: KeyAgreementConcrete::Mock(failure_mode),
            ..MOCK_I_KEY_AGREEMENT_ADAPTER_DECLARATION
        },
    };
    Ok(LookupEnvelopeDeclarationSuccessReturn { declaration })
}
