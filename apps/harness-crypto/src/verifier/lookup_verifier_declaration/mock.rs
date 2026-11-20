#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_arguments
)]

use super::super::factory::provides::{
    VerifierConcreteKind, VerifierDeclaration, VerifierDeclarationOverrides,
    build_verifier_declaration,
};
use super::interface::{
    LookupVerifierDeclarationDeps, LookupVerifierDeclarationFn, LookupVerifierDeclarationParams,
    LookupVerifierDeclarationPayload, LookupVerifierDeclarationReturn,
    LookupVerifierDeclarationSuccessReturn,
};

#[derive(Default)]
pub struct LookupVerifierDeclarationDepsOverrides;

pub fn build_lookup_verifier_declaration_deps(
    _overrides: LookupVerifierDeclarationDepsOverrides,
) -> LookupVerifierDeclarationDeps {
    LookupVerifierDeclarationDeps
}

#[derive(Default)]
pub struct LookupVerifierDeclarationParamsOverrides {
    pub identifier: Option<VerifierConcreteKind>,
}

pub fn build_lookup_verifier_declaration_params(
    overrides: LookupVerifierDeclarationParamsOverrides,
) -> LookupVerifierDeclarationParams {
    LookupVerifierDeclarationParams {
        identifier: overrides.identifier.unwrap_or(VerifierConcreteKind::Evm),
    }
}

#[derive(Default)]
pub struct LookupVerifierDeclarationPayloadOverrides;

pub fn build_lookup_verifier_declaration_payload(
    _overrides: LookupVerifierDeclarationPayloadOverrides,
) -> LookupVerifierDeclarationPayload {
    LookupVerifierDeclarationPayload
}

#[derive(Default)]
pub struct LookupVerifierDeclarationSuccessReturnOverrides {
    pub declaration: Option<VerifierDeclaration>,
}

pub fn build_lookup_verifier_declaration_success_return(
    overrides: LookupVerifierDeclarationSuccessReturnOverrides,
) -> LookupVerifierDeclarationSuccessReturn {
    LookupVerifierDeclarationSuccessReturn {
        declaration: overrides
            .declaration
            .unwrap_or_else(|| build_verifier_declaration(VerifierDeclarationOverrides::default())),
    }
}

pub fn mock_lookup_verifier_declaration(
    _deps: &LookupVerifierDeclarationDeps,
    _params: LookupVerifierDeclarationParams,
    _payload: LookupVerifierDeclarationPayload,
) -> LookupVerifierDeclarationReturn {
    let _: LookupVerifierDeclarationFn = mock_lookup_verifier_declaration;
    Ok(build_lookup_verifier_declaration_success_return(
        Default::default(),
    ))
}
