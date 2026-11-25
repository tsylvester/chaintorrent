use crate::factory::provides::{KdfConcrete, KdfDeclaration};

pub struct LookupKdfDeclarationDeps;

pub struct LookupKdfDeclarationParams {
    pub concrete: KdfConcrete,
}

pub struct LookupKdfDeclarationPayload;

pub struct LookupKdfDeclarationSuccessReturn {
    pub declaration: KdfDeclaration,
}

#[derive(Debug, PartialEq, Eq)]
pub enum LookupKdfDeclarationErrorReturn {}

pub type LookupKdfDeclarationReturn =
    Result<LookupKdfDeclarationSuccessReturn, LookupKdfDeclarationErrorReturn>;

pub type LookupKdfDeclarationFn = fn(
    &LookupKdfDeclarationDeps,
    LookupKdfDeclarationParams,
    LookupKdfDeclarationPayload,
) -> LookupKdfDeclarationReturn;
