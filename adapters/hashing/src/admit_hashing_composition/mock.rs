#![allow(dead_code, clippy::too_many_arguments)]

use crate::factory::provides::{
    CommitmentConcrete, CommitmentSchemeIdentifier, build_commitment_concrete,
};
use crate::lookup_hashing_declaration::provides::{
    LookupHashingDeclarationFn, mock_lookup_hashing_declaration,
};

use super::interface::{
    AdmitHashingCompositionDeps, AdmitHashingCompositionParams, AdmitHashingCompositionPayload,
    AdmitHashingCompositionReturn, AdmitHashingCompositionSuccessReturn,
};

#[derive(Default)]
pub struct AdmitHashingCompositionDepsOverrides {
    pub lookup_hashing_declaration: Option<LookupHashingDeclarationFn>,
}

pub fn build_admit_hashing_composition_deps(
    overrides: AdmitHashingCompositionDepsOverrides,
) -> AdmitHashingCompositionDeps {
    AdmitHashingCompositionDeps {
        lookup_hashing_declaration: overrides
            .lookup_hashing_declaration
            .unwrap_or(mock_lookup_hashing_declaration),
    }
}

#[derive(Default)]
pub struct AdmitHashingCompositionParamsOverrides {
    pub concrete: Option<CommitmentConcrete>,
    pub identifier: Option<CommitmentSchemeIdentifier>,
}

pub fn build_admit_hashing_composition_params(
    overrides: AdmitHashingCompositionParamsOverrides,
) -> AdmitHashingCompositionParams {
    AdmitHashingCompositionParams {
        concrete: overrides.concrete.unwrap_or(build_commitment_concrete(
            CommitmentConcrete::Blake3Bao,
            Default::default(),
        )),
        identifier: overrides
            .identifier
            .unwrap_or(CommitmentSchemeIdentifier::Blake3BaoV1),
    }
}

#[derive(Default)]
pub struct AdmitHashingCompositionPayloadOverrides;

pub fn build_admit_hashing_composition_payload(
    _overrides: AdmitHashingCompositionPayloadOverrides,
) -> AdmitHashingCompositionPayload {
    AdmitHashingCompositionPayload
}

#[derive(Default)]
pub struct AdmitHashingCompositionSuccessReturnOverrides;

pub fn build_admit_hashing_composition_success_return(
    _overrides: AdmitHashingCompositionSuccessReturnOverrides,
) -> AdmitHashingCompositionSuccessReturn {
    AdmitHashingCompositionSuccessReturn
}

pub fn mock_admit_hashing_composition(
    _deps: &AdmitHashingCompositionDeps,
    _params: AdmitHashingCompositionParams,
    _payload: AdmitHashingCompositionPayload,
) -> AdmitHashingCompositionReturn {
    Ok(build_admit_hashing_composition_success_return(
        Default::default(),
    ))
}
