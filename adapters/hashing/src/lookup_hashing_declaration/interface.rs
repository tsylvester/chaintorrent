use crate::factory::provides::{CommitmentConcrete, CommitmentDeclaration};

pub struct LookupHashingDeclarationDeps;

pub struct LookupHashingDeclarationParams {
    pub concrete: CommitmentConcrete,
}

pub struct LookupHashingDeclarationPayload;

pub struct LookupHashingDeclarationSuccessReturn {
    pub declaration: CommitmentDeclaration,
}

#[derive(Debug, PartialEq, Eq)]
pub enum LookupHashingDeclarationErrorReturn {}

pub type LookupHashingDeclarationReturn =
    Result<LookupHashingDeclarationSuccessReturn, LookupHashingDeclarationErrorReturn>;

pub type LookupHashingDeclarationFn = fn(
    &LookupHashingDeclarationDeps,
    LookupHashingDeclarationParams,
    LookupHashingDeclarationPayload,
) -> LookupHashingDeclarationReturn;
