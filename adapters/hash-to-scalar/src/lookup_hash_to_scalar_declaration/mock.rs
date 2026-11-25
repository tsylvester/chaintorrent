#![allow(dead_code)]

use super::interface::{
    LookupHashToScalarDeclarationDeps, LookupHashToScalarDeclarationFn,
    LookupHashToScalarDeclarationParams, LookupHashToScalarDeclarationPayload,
    LookupHashToScalarDeclarationReturn, LookupHashToScalarDeclarationSuccessReturn,
};
use crate::factory::provides::{
    HashToScalarConcrete, HashToScalarDeclaration, MockIHashToScalarAdapterFailureMode,
    build_hash_to_scalar_declaration,
};

#[derive(Default)]
pub struct LookupHashToScalarDeclarationDepsOverrides;

pub fn build_lookup_hash_to_scalar_declaration_deps(
    _overrides: LookupHashToScalarDeclarationDepsOverrides,
) -> LookupHashToScalarDeclarationDeps {
    LookupHashToScalarDeclarationDeps
}

#[derive(Default)]
pub struct LookupHashToScalarDeclarationParamsOverrides {
    pub concrete: Option<HashToScalarConcrete>,
}

pub fn build_lookup_hash_to_scalar_declaration_params(
    overrides: LookupHashToScalarDeclarationParamsOverrides,
) -> LookupHashToScalarDeclarationParams {
    LookupHashToScalarDeclarationParams {
        concrete: overrides.concrete.unwrap_or(HashToScalarConcrete::Mock(
            MockIHashToScalarAdapterFailureMode::Succeeds,
        )),
    }
}

#[derive(Default)]
pub struct LookupHashToScalarDeclarationPayloadOverrides;

pub fn build_lookup_hash_to_scalar_declaration_payload(
    _overrides: LookupHashToScalarDeclarationPayloadOverrides,
) -> LookupHashToScalarDeclarationPayload {
    LookupHashToScalarDeclarationPayload
}

#[derive(Default)]
pub struct LookupHashToScalarDeclarationSuccessReturnOverrides {
    pub declaration: Option<HashToScalarDeclaration>,
}

pub fn build_lookup_hash_to_scalar_declaration_success_return(
    overrides: LookupHashToScalarDeclarationSuccessReturnOverrides,
) -> LookupHashToScalarDeclarationSuccessReturn {
    LookupHashToScalarDeclarationSuccessReturn {
        declaration: overrides
            .declaration
            .unwrap_or_else(|| build_hash_to_scalar_declaration(Default::default())),
    }
}

pub fn mock_lookup_hash_to_scalar_declaration(
    _deps: &LookupHashToScalarDeclarationDeps,
    _params: LookupHashToScalarDeclarationParams,
    _payload: LookupHashToScalarDeclarationPayload,
) -> LookupHashToScalarDeclarationReturn {
    Ok(build_lookup_hash_to_scalar_declaration_success_return(
        Default::default(),
    ))
}

const _: LookupHashToScalarDeclarationFn = mock_lookup_hash_to_scalar_declaration;
