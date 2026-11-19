use crate::factory::provides::{
    ChainFormsDeclaration, ChainFormsIdentifier, ChainMemberIdentifier,
};
use crate::lookup_chain_declaration::provides::{
    LookupChainDeclarationFn, lookup_chain_declaration,
};

#[derive(Clone, Copy)]
pub struct AdmitChainCompositionDeps {
    pub lookup_chain_declaration: LookupChainDeclarationFn,
}

pub(crate) const ADMIT_CHAIN_COMPOSITION_DEPS: AdmitChainCompositionDeps =
    AdmitChainCompositionDeps {
        lookup_chain_declaration,
    };

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdmitChainCompositionParams {
    pub identifier: ChainMemberIdentifier,
}

#[derive(Clone, Copy)]
pub struct AdmitChainCompositionPayload<'a> {
    pub forms: &'a ChainFormsDeclaration,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdmitChainCompositionSuccessReturn;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChainFormsIdentifierDisagreement {
    pub required: ChainFormsIdentifier,
    pub declared: ChainFormsIdentifier,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdmitChainCompositionErrorReturn {
    Incompatible {
        forms: ChainFormsIdentifierDisagreement,
    },
}

pub type AdmitChainCompositionReturn =
    Result<AdmitChainCompositionSuccessReturn, AdmitChainCompositionErrorReturn>;

pub type AdmitChainCompositionFn = for<'a> fn(
    &AdmitChainCompositionDeps,
    AdmitChainCompositionParams,
    AdmitChainCompositionPayload<'a>,
) -> AdmitChainCompositionReturn;
