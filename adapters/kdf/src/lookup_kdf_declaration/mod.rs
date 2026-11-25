mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::blake3_keyed::provides::Blake3KeyedKdf;
#[cfg(any(test, feature = "mocks"))]
use crate::factory::provides::MOCK_I_KEY_DERIVATION_ADAPTER_DECLARATION;
use crate::factory::provides::{KdfConcrete, KdfDeclaration};
use interface::{
    LookupKdfDeclarationDeps, LookupKdfDeclarationParams, LookupKdfDeclarationPayload,
    LookupKdfDeclarationReturn, LookupKdfDeclarationSuccessReturn,
};

pub fn lookup_kdf_declaration(
    _deps: &LookupKdfDeclarationDeps,
    params: LookupKdfDeclarationParams,
    _payload: LookupKdfDeclarationPayload,
) -> LookupKdfDeclarationReturn {
    match params.concrete {
        KdfConcrete::Blake3Keyed => Ok(LookupKdfDeclarationSuccessReturn {
            declaration: Blake3KeyedKdf::DECLARATION,
        }),
        #[cfg(any(test, feature = "mocks"))]
        KdfConcrete::Mock(failure_mode) => Ok(LookupKdfDeclarationSuccessReturn {
            declaration: KdfDeclaration {
                concrete: KdfConcrete::Mock(failure_mode),
                identifier: MOCK_I_KEY_DERIVATION_ADAPTER_DECLARATION.identifier,
                adapter_version: MOCK_I_KEY_DERIVATION_ADAPTER_DECLARATION.adapter_version,
                interface_version: MOCK_I_KEY_DERIVATION_ADAPTER_DECLARATION.interface_version,
            },
        }),
    }
}
