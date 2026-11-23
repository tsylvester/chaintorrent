#![allow(dead_code, clippy::too_many_arguments)]

use super::interface::{
    LookupRandomDeclarationDeps, LookupRandomDeclarationFn, LookupRandomDeclarationParams,
    LookupRandomDeclarationPayload, LookupRandomDeclarationReturn,
    LookupRandomDeclarationSuccessReturn,
};
use crate::factory::provides::{
    RandomSourceDeclaration, RandomSourceDeclarationOverrides, RandomSourceKind,
    build_random_source_declaration,
};

#[derive(Default)]
pub struct LookupRandomDeclarationDepsOverrides;

pub fn build_lookup_random_declaration_deps(
    _overrides: LookupRandomDeclarationDepsOverrides,
) -> LookupRandomDeclarationDeps {
    LookupRandomDeclarationDeps
}

#[derive(Default)]
pub struct LookupRandomDeclarationParamsOverrides {
    pub identifier: Option<RandomSourceKind>,
}

pub fn build_lookup_random_declaration_params(
    overrides: LookupRandomDeclarationParamsOverrides,
) -> LookupRandomDeclarationParams {
    LookupRandomDeclarationParams {
        identifier: overrides
            .identifier
            .unwrap_or(RandomSourceKind::OperatingSystem),
    }
}

#[derive(Default)]
pub struct LookupRandomDeclarationPayloadOverrides;

pub fn build_lookup_random_declaration_payload(
    _overrides: LookupRandomDeclarationPayloadOverrides,
) -> LookupRandomDeclarationPayload {
    LookupRandomDeclarationPayload
}

#[derive(Default)]
pub struct LookupRandomDeclarationSuccessReturnOverrides {
    pub declaration: Option<RandomSourceDeclaration>,
}

pub fn build_lookup_random_declaration_success_return(
    overrides: LookupRandomDeclarationSuccessReturnOverrides,
) -> LookupRandomDeclarationSuccessReturn {
    LookupRandomDeclarationSuccessReturn {
        declaration: overrides.declaration.unwrap_or_else(|| {
            build_random_source_declaration(RandomSourceDeclarationOverrides::default())
        }),
    }
}

pub fn mock_lookup_random_declaration(
    _deps: &LookupRandomDeclarationDeps,
    _params: LookupRandomDeclarationParams,
    _payload: LookupRandomDeclarationPayload,
) -> LookupRandomDeclarationReturn {
    Ok(build_lookup_random_declaration_success_return(
        Default::default(),
    ))
}

const _: LookupRandomDeclarationFn = mock_lookup_random_declaration;
