#![allow(dead_code, clippy::too_many_arguments)]

use crate::factory::provides::{
    CommitmentConcrete, CommitmentDeclaration, build_commitment_concrete,
    build_commitment_declaration,
};

use super::interface::{
    LookupHashingDeclarationDeps, LookupHashingDeclarationParams, LookupHashingDeclarationPayload,
    LookupHashingDeclarationReturn, LookupHashingDeclarationSuccessReturn,
};

#[derive(Default)]
pub struct LookupHashingDeclarationDepsOverrides;

pub fn build_lookup_hashing_declaration_deps(
    _overrides: LookupHashingDeclarationDepsOverrides,
) -> LookupHashingDeclarationDeps {
    LookupHashingDeclarationDeps
}

#[derive(Default)]
pub struct LookupHashingDeclarationParamsOverrides {
    pub concrete: Option<CommitmentConcrete>,
}

pub fn build_lookup_hashing_declaration_params(
    overrides: LookupHashingDeclarationParamsOverrides,
) -> LookupHashingDeclarationParams {
    LookupHashingDeclarationParams {
        concrete: overrides.concrete.unwrap_or(build_commitment_concrete(
            CommitmentConcrete::Blake3Bao,
            Default::default(),
        )),
    }
}

#[derive(Default)]
pub struct LookupHashingDeclarationPayloadOverrides;

pub fn build_lookup_hashing_declaration_payload(
    _overrides: LookupHashingDeclarationPayloadOverrides,
) -> LookupHashingDeclarationPayload {
    LookupHashingDeclarationPayload
}

#[derive(Default)]
pub struct LookupHashingDeclarationSuccessReturnOverrides {
    pub declaration: Option<CommitmentDeclaration>,
}

pub fn build_lookup_hashing_declaration_success_return(
    overrides: LookupHashingDeclarationSuccessReturnOverrides,
) -> LookupHashingDeclarationSuccessReturn {
    LookupHashingDeclarationSuccessReturn {
        declaration: overrides
            .declaration
            .unwrap_or_else(|| build_commitment_declaration(Default::default())),
    }
}

pub fn mock_lookup_hashing_declaration(
    _deps: &LookupHashingDeclarationDeps,
    _params: LookupHashingDeclarationParams,
    _payload: LookupHashingDeclarationPayload,
) -> LookupHashingDeclarationReturn {
    Ok(build_lookup_hashing_declaration_success_return(
        Default::default(),
    ))
}
