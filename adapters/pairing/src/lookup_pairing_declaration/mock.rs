#![allow(dead_code, clippy::too_many_arguments)]

use super::interface::{
    LookupPairingDeclarationDeps, LookupPairingDeclarationParams, LookupPairingDeclarationPayload,
    LookupPairingDeclarationReturn, LookupPairingDeclarationSuccessReturn,
};
use crate::factory::provides::{
    PairingConcrete, PairingDeclaration, PairingDeclarationOverrides, build_pairing_declaration,
};

#[derive(Default)]
pub struct LookupPairingDeclarationDepsOverrides;

pub fn build_lookup_pairing_declaration_deps(
    _overrides: LookupPairingDeclarationDepsOverrides,
) -> LookupPairingDeclarationDeps {
    LookupPairingDeclarationDeps
}

#[derive(Default)]
pub struct LookupPairingDeclarationParamsOverrides {
    pub concrete: Option<PairingConcrete>,
}

pub fn build_lookup_pairing_declaration_params(
    overrides: LookupPairingDeclarationParamsOverrides,
) -> LookupPairingDeclarationParams {
    LookupPairingDeclarationParams {
        concrete: overrides.concrete.unwrap_or(PairingConcrete::Bn254Arkworks),
    }
}

#[derive(Default)]
pub struct LookupPairingDeclarationPayloadOverrides;

pub fn build_lookup_pairing_declaration_payload(
    _overrides: LookupPairingDeclarationPayloadOverrides,
) -> LookupPairingDeclarationPayload {
    LookupPairingDeclarationPayload
}

#[derive(Default)]
pub struct LookupPairingDeclarationSuccessReturnOverrides {
    pub declaration: Option<PairingDeclaration>,
}

pub fn build_lookup_pairing_declaration_success_return(
    overrides: LookupPairingDeclarationSuccessReturnOverrides,
) -> LookupPairingDeclarationSuccessReturn {
    LookupPairingDeclarationSuccessReturn {
        declaration: overrides
            .declaration
            .unwrap_or_else(|| build_pairing_declaration(PairingDeclarationOverrides::default())),
    }
}

pub fn mock_lookup_pairing_declaration(
    _deps: &LookupPairingDeclarationDeps,
    _params: LookupPairingDeclarationParams,
    _payload: LookupPairingDeclarationPayload,
) -> LookupPairingDeclarationReturn {
    Ok(build_lookup_pairing_declaration_success_return(
        Default::default(),
    ))
}
