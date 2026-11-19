#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_arguments
)]

use super::interface::{
    LookupChainDeclarationDeps, LookupChainDeclarationFn, LookupChainDeclarationParams,
    LookupChainDeclarationPayload, LookupChainDeclarationReturn,
    LookupChainDeclarationSuccessReturn,
};
use crate::factory::provides::{
    ChainDeclaration, ChainDeclarationOverrides, ChainMemberIdentifier, build_chain_declaration,
};

#[derive(Default)]
pub struct LookupChainDeclarationDepsOverrides;

pub fn build_lookup_chain_declaration_deps(
    _overrides: LookupChainDeclarationDepsOverrides,
) -> LookupChainDeclarationDeps {
    LookupChainDeclarationDeps
}

#[derive(Default)]
pub struct LookupChainDeclarationParamsOverrides {
    pub identifier: Option<ChainMemberIdentifier>,
}

pub fn build_lookup_chain_declaration_params(
    overrides: LookupChainDeclarationParamsOverrides,
) -> LookupChainDeclarationParams {
    LookupChainDeclarationParams {
        identifier: overrides.identifier.unwrap_or(ChainMemberIdentifier::Base),
    }
}

#[derive(Default)]
pub struct LookupChainDeclarationPayloadOverrides;

pub fn build_lookup_chain_declaration_payload(
    _overrides: LookupChainDeclarationPayloadOverrides,
) -> LookupChainDeclarationPayload {
    LookupChainDeclarationPayload
}

#[derive(Default)]
pub struct LookupChainDeclarationSuccessReturnOverrides {
    pub declaration: Option<ChainDeclaration>,
}

pub fn build_lookup_chain_declaration_success_return(
    overrides: LookupChainDeclarationSuccessReturnOverrides,
) -> LookupChainDeclarationSuccessReturn {
    LookupChainDeclarationSuccessReturn {
        declaration: overrides
            .declaration
            .unwrap_or_else(|| build_chain_declaration(ChainDeclarationOverrides::default())),
    }
}

pub fn mock_lookup_chain_declaration(
    _deps: &LookupChainDeclarationDeps,
    _params: LookupChainDeclarationParams,
    _payload: LookupChainDeclarationPayload,
) -> LookupChainDeclarationReturn {
    Ok(build_lookup_chain_declaration_success_return(
        Default::default(),
    ))
}

const _: LookupChainDeclarationFn = mock_lookup_chain_declaration;
