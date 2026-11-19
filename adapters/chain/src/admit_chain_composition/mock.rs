#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_arguments
)]

use super::interface::{
    AdmitChainCompositionDeps, AdmitChainCompositionFn, AdmitChainCompositionParams,
    AdmitChainCompositionPayload, AdmitChainCompositionReturn, AdmitChainCompositionSuccessReturn,
    ChainFormsIdentifierDisagreement,
};
use crate::factory::provides::{
    ChainFormsDeclaration, ChainFormsIdentifier, ChainMemberIdentifier,
};
use crate::lookup_chain_declaration::provides::LookupChainDeclarationFn;

#[derive(Default)]
pub(crate) struct AdmitChainCompositionDepsOverrides;

pub(crate) fn build_admit_chain_composition_deps(
    lookup_chain_declaration: LookupChainDeclarationFn,
    _overrides: AdmitChainCompositionDepsOverrides,
) -> AdmitChainCompositionDeps {
    AdmitChainCompositionDeps {
        lookup_chain_declaration,
    }
}

#[derive(Default)]
pub struct AdmitChainCompositionParamsOverrides {
    pub identifier: Option<ChainMemberIdentifier>,
}

pub fn build_admit_chain_composition_params(
    overrides: AdmitChainCompositionParamsOverrides,
) -> AdmitChainCompositionParams {
    AdmitChainCompositionParams {
        identifier: overrides.identifier.unwrap_or(ChainMemberIdentifier::Base),
    }
}

#[derive(Default)]
pub struct AdmitChainCompositionPayloadOverrides;

pub fn build_admit_chain_composition_payload<'a>(
    forms: &'a ChainFormsDeclaration,
    _overrides: AdmitChainCompositionPayloadOverrides,
) -> AdmitChainCompositionPayload<'a> {
    AdmitChainCompositionPayload { forms }
}

#[derive(Default)]
pub struct AdmitChainCompositionSuccessReturnOverrides;

pub fn build_admit_chain_composition_success_return(
    _overrides: AdmitChainCompositionSuccessReturnOverrides,
) -> AdmitChainCompositionSuccessReturn {
    AdmitChainCompositionSuccessReturn
}

#[derive(Default)]
pub struct ChainFormsIdentifierDisagreementOverrides {
    pub required: Option<ChainFormsIdentifier>,
    pub declared: Option<ChainFormsIdentifier>,
}

pub fn build_chain_forms_identifier_disagreement(
    overrides: ChainFormsIdentifierDisagreementOverrides,
) -> ChainFormsIdentifierDisagreement {
    ChainFormsIdentifierDisagreement {
        required: overrides.required.unwrap_or(ChainFormsIdentifier::EvmV1),
        declared: overrides.declared.unwrap_or(ChainFormsIdentifier::EvmV1),
    }
}

pub fn mock_admit_chain_composition(
    _deps: &AdmitChainCompositionDeps,
    _params: AdmitChainCompositionParams,
    _payload: AdmitChainCompositionPayload<'_>,
) -> AdmitChainCompositionReturn {
    Ok(build_admit_chain_composition_success_return(
        Default::default(),
    ))
}

const _: AdmitChainCompositionFn = mock_admit_chain_composition;
