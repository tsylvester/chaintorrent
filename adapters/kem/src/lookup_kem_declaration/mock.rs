#![allow(dead_code, clippy::too_many_arguments)]

use super::interface::{
    LookupKemDeclarationDeps, LookupKemDeclarationFn, LookupKemDeclarationParams,
    LookupKemDeclarationPayload, LookupKemDeclarationReturn, LookupKemDeclarationSuccessReturn,
};
use crate::factory::provides::{
    KemConcrete, KemConcreteOverrides, KemDeclaration, KemDeclarationOverrides, build_kem_concrete,
    build_kem_declaration,
};

#[derive(Default)]
pub struct LookupKemDeclarationDepsOverrides;

pub fn build_lookup_kem_declaration_deps(
    _overrides: LookupKemDeclarationDepsOverrides,
) -> LookupKemDeclarationDeps {
    LookupKemDeclarationDeps
}

#[derive(Default)]
pub struct LookupKemDeclarationParamsOverrides {
    pub concrete: Option<KemConcrete>,
}

pub fn build_lookup_kem_declaration_params(
    overrides: LookupKemDeclarationParamsOverrides,
) -> LookupKemDeclarationParams {
    LookupKemDeclarationParams {
        concrete: overrides
            .concrete
            .unwrap_or_else(|| build_kem_concrete(KemConcrete::Bb1DepthOne, KemConcreteOverrides)),
    }
}

#[derive(Default)]
pub struct LookupKemDeclarationPayloadOverrides;

pub fn build_lookup_kem_declaration_payload(
    _overrides: LookupKemDeclarationPayloadOverrides,
) -> LookupKemDeclarationPayload {
    LookupKemDeclarationPayload
}

#[derive(Default)]
pub struct LookupKemDeclarationSuccessReturnOverrides {
    pub declaration: Option<KemDeclaration>,
}

pub fn build_lookup_kem_declaration_success_return(
    overrides: LookupKemDeclarationSuccessReturnOverrides,
) -> LookupKemDeclarationSuccessReturn {
    LookupKemDeclarationSuccessReturn {
        declaration: overrides
            .declaration
            .unwrap_or_else(|| build_kem_declaration(KemDeclarationOverrides::default())),
    }
}

pub fn mock_lookup_kem_declaration(
    _deps: &LookupKemDeclarationDeps,
    _params: LookupKemDeclarationParams,
    _payload: LookupKemDeclarationPayload,
) -> LookupKemDeclarationReturn {
    let _: LookupKemDeclarationFn = mock_lookup_kem_declaration;
    Ok(build_lookup_kem_declaration_success_return(
        Default::default(),
    ))
}
