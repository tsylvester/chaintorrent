mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::bls12_381_arkworks::provides::Bls12381ArkworksPairing;
use crate::bls12_381_halo2curves::provides::Bls12381Halo2curvesPairing;
use crate::bn254_arkworks::provides::Bn254ArkworksPairing;
use crate::bn254_halo2curves::provides::Bn254Halo2curvesPairing;
use crate::factory::provides::PairingConcrete;
#[cfg(any(test, feature = "mocks"))]
use crate::factory::provides::{MOCK_I_PAIRING_ADAPTER_DECLARATION, PairingDeclaration};
use interface::{
    LookupPairingDeclarationDeps, LookupPairingDeclarationParams, LookupPairingDeclarationPayload,
    LookupPairingDeclarationReturn, LookupPairingDeclarationSuccessReturn,
};

pub fn lookup_pairing_declaration(
    _deps: &LookupPairingDeclarationDeps,
    params: LookupPairingDeclarationParams,
    _payload: LookupPairingDeclarationPayload,
) -> LookupPairingDeclarationReturn {
    match params.concrete {
        PairingConcrete::Bn254Arkworks => Ok(LookupPairingDeclarationSuccessReturn {
            declaration: Bn254ArkworksPairing::DECLARATION,
        }),
        PairingConcrete::Bn254Halo2curves => Ok(LookupPairingDeclarationSuccessReturn {
            declaration: Bn254Halo2curvesPairing::DECLARATION,
        }),
        PairingConcrete::Bls12381Arkworks => Ok(LookupPairingDeclarationSuccessReturn {
            declaration: Bls12381ArkworksPairing::DECLARATION,
        }),
        PairingConcrete::Bls12381Halo2curves => Ok(LookupPairingDeclarationSuccessReturn {
            declaration: Bls12381Halo2curvesPairing::DECLARATION,
        }),
        #[cfg(any(test, feature = "mocks"))]
        PairingConcrete::Mock(_) => Ok(LookupPairingDeclarationSuccessReturn {
            declaration: PairingDeclaration {
                concrete: params.concrete,
                curve: MOCK_I_PAIRING_ADAPTER_DECLARATION.curve,
                verifier_group_arithmetic: MOCK_I_PAIRING_ADAPTER_DECLARATION
                    .verifier_group_arithmetic,
                precompile_encoding: MOCK_I_PAIRING_ADAPTER_DECLARATION.precompile_encoding,
                target_group_encoding: MOCK_I_PAIRING_ADAPTER_DECLARATION.target_group_encoding,
                adapter_version: MOCK_I_PAIRING_ADAPTER_DECLARATION.adapter_version,
                interface_version: MOCK_I_PAIRING_ADAPTER_DECLARATION.interface_version,
            },
        }),
    }
}
