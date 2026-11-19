#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_arguments
)]

use super::super::factory::provides::{
    GenerateConcreteKind, GenerateDeclaration, GenerateDeclarationOverrides,
    build_generate_declaration,
};
use super::interface::{
    LookupGenerateDeclarationDeps, LookupGenerateDeclarationFn, LookupGenerateDeclarationParams,
    LookupGenerateDeclarationPayload, LookupGenerateDeclarationReturn,
    LookupGenerateDeclarationSuccessReturn,
};

#[derive(Default)]
pub struct LookupGenerateDeclarationDepsOverrides;

pub fn build_lookup_generate_declaration_deps(
    _overrides: LookupGenerateDeclarationDepsOverrides,
) -> LookupGenerateDeclarationDeps {
    LookupGenerateDeclarationDeps
}

#[derive(Default)]
pub struct LookupGenerateDeclarationParamsOverrides {
    pub identifier: Option<GenerateConcreteKind>,
}

pub fn build_lookup_generate_declaration_params(
    overrides: LookupGenerateDeclarationParamsOverrides,
) -> LookupGenerateDeclarationParams {
    LookupGenerateDeclarationParams {
        identifier: overrides
            .identifier
            .unwrap_or(GenerateConcreteKind::Solidity),
    }
}

#[derive(Default)]
pub struct LookupGenerateDeclarationPayloadOverrides;

pub fn build_lookup_generate_declaration_payload(
    _overrides: LookupGenerateDeclarationPayloadOverrides,
) -> LookupGenerateDeclarationPayload {
    LookupGenerateDeclarationPayload
}

#[derive(Default)]
pub struct LookupGenerateDeclarationSuccessReturnOverrides {
    pub declaration: Option<GenerateDeclaration>,
}

pub fn build_lookup_generate_declaration_success_return(
    overrides: LookupGenerateDeclarationSuccessReturnOverrides,
) -> LookupGenerateDeclarationSuccessReturn {
    LookupGenerateDeclarationSuccessReturn {
        declaration: overrides
            .declaration
            .unwrap_or_else(|| build_generate_declaration(GenerateDeclarationOverrides::default())),
    }
}

pub fn mock_lookup_generate_declaration(
    _deps: &LookupGenerateDeclarationDeps,
    _params: LookupGenerateDeclarationParams,
    _payload: LookupGenerateDeclarationPayload,
) -> LookupGenerateDeclarationReturn {
    let _: LookupGenerateDeclarationFn = mock_lookup_generate_declaration;
    Ok(build_lookup_generate_declaration_success_return(
        Default::default(),
    ))
}
