use core::convert::Infallible;

use crate::factory::provides::{KemConcrete, KemDeclaration};

pub struct LookupKemDeclarationDeps;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LookupKemDeclarationParams {
    pub concrete: KemConcrete,
}

pub struct LookupKemDeclarationPayload;

pub struct LookupKemDeclarationSuccessReturn {
    pub declaration: KemDeclaration,
}

pub type LookupKemDeclarationReturn = Result<LookupKemDeclarationSuccessReturn, Infallible>;

pub type LookupKemDeclarationFn = fn(
    &LookupKemDeclarationDeps,
    LookupKemDeclarationParams,
    LookupKemDeclarationPayload,
) -> LookupKemDeclarationReturn;
