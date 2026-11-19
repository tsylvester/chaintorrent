use core::convert::Infallible;

use super::super::factory::provides::{GenerateConcreteKind, GenerateDeclaration};

pub struct LookupGenerateDeclarationDeps;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LookupGenerateDeclarationParams {
    pub identifier: GenerateConcreteKind,
}

pub struct LookupGenerateDeclarationPayload;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LookupGenerateDeclarationSuccessReturn {
    pub declaration: GenerateDeclaration,
}

pub type LookupGenerateDeclarationReturn =
    Result<LookupGenerateDeclarationSuccessReturn, Infallible>;

pub type LookupGenerateDeclarationFn = fn(
    &LookupGenerateDeclarationDeps,
    LookupGenerateDeclarationParams,
    LookupGenerateDeclarationPayload,
) -> LookupGenerateDeclarationReturn;
