use crate::factory::provides::{CommitmentConcrete, CommitmentSchemeIdentifier};
use crate::lookup_hashing_declaration::provides::LookupHashingDeclarationFn;

pub struct AdmitHashingCompositionDeps {
    pub lookup_hashing_declaration: LookupHashingDeclarationFn,
}

pub struct AdmitHashingCompositionParams {
    pub concrete: CommitmentConcrete,
    pub identifier: CommitmentSchemeIdentifier,
}

pub struct AdmitHashingCompositionPayload;

#[derive(Debug, PartialEq, Eq)]
pub struct AdmitHashingCompositionSuccessReturn;

#[derive(Debug, PartialEq, Eq)]
pub enum AdmitHashingCompositionErrorReturn {
    Incompatible {
        concrete: CommitmentConcrete,
        declared: CommitmentSchemeIdentifier,
        required: CommitmentSchemeIdentifier,
    },
}

pub type AdmitHashingCompositionReturn =
    Result<AdmitHashingCompositionSuccessReturn, AdmitHashingCompositionErrorReturn>;

pub type AdmitHashingCompositionFn = fn(
    &AdmitHashingCompositionDeps,
    AdmitHashingCompositionParams,
    AdmitHashingCompositionPayload,
) -> AdmitHashingCompositionReturn;
