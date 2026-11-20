use core::convert::Infallible;

use super::super::factory::provides::{VerifierConcreteKind, VerifierDeclaration};

pub struct LookupVerifierDeclarationDeps;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LookupVerifierDeclarationParams {
    pub identifier: VerifierConcreteKind,
}

pub struct LookupVerifierDeclarationPayload;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LookupVerifierDeclarationSuccessReturn {
    pub declaration: VerifierDeclaration,
}

pub type LookupVerifierDeclarationReturn =
    Result<LookupVerifierDeclarationSuccessReturn, Infallible>;

pub type LookupVerifierDeclarationFn = fn(
    &LookupVerifierDeclarationDeps,
    LookupVerifierDeclarationParams,
    LookupVerifierDeclarationPayload,
) -> LookupVerifierDeclarationReturn;
