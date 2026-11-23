use core::convert::Infallible;

use crate::factory::provides::{RandomSourceDeclaration, RandomSourceKind};

pub struct LookupRandomDeclarationDeps;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LookupRandomDeclarationParams {
    pub identifier: RandomSourceKind,
}

pub struct LookupRandomDeclarationPayload;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LookupRandomDeclarationSuccessReturn {
    pub declaration: RandomSourceDeclaration,
}

pub type LookupRandomDeclarationReturn = Result<LookupRandomDeclarationSuccessReturn, Infallible>;

pub type LookupRandomDeclarationFn = fn(
    &LookupRandomDeclarationDeps,
    LookupRandomDeclarationParams,
    LookupRandomDeclarationPayload,
) -> LookupRandomDeclarationReturn;
