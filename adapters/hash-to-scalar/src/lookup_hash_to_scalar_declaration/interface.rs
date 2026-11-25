use crate::factory::provides::{HashToScalarConcrete, HashToScalarDeclaration};

pub struct LookupHashToScalarDeclarationDeps;

pub struct LookupHashToScalarDeclarationParams {
    pub concrete: HashToScalarConcrete,
}

pub struct LookupHashToScalarDeclarationPayload;

pub struct LookupHashToScalarDeclarationSuccessReturn {
    pub declaration: HashToScalarDeclaration,
}

#[derive(Debug, PartialEq, Eq)]
pub enum LookupHashToScalarDeclarationErrorReturn {}

pub type LookupHashToScalarDeclarationReturn =
    Result<LookupHashToScalarDeclarationSuccessReturn, LookupHashToScalarDeclarationErrorReturn>;

pub type LookupHashToScalarDeclarationFn = fn(
    &LookupHashToScalarDeclarationDeps,
    LookupHashToScalarDeclarationParams,
    LookupHashToScalarDeclarationPayload,
) -> LookupHashToScalarDeclarationReturn;
