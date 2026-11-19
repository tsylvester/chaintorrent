use chain::{ChainFormsDeclaration, ChainFormsIdentifier};
use encoding::{EncodingDeclaration, EncodingIdentifier};

use super::super::factory::provides::GenerateConcreteKind;
use super::super::lookup_generate_declaration::provides::LookupGenerateDeclarationFn;

#[derive(Clone, Copy)]
pub struct AdmitGenerateCompositionDeps {
    pub lookup_generate_declaration: LookupGenerateDeclarationFn,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdmitGenerateCompositionParams {
    pub identifier: GenerateConcreteKind,
}

#[derive(Clone, Copy)]
pub struct AdmitGenerateCompositionPayload<'a> {
    pub chain_forms: &'a ChainFormsDeclaration,
    pub encoder: &'a EncodingDeclaration,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdmitGenerateCompositionSuccessReturn;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChainFormsIdentifierDisagreement {
    pub required: ChainFormsIdentifier,
    pub declared: ChainFormsIdentifier,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EncodingIdentifierDisagreement {
    pub required: EncodingIdentifier,
    pub declared: EncodingIdentifier,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdmitGenerateCompositionErrorReturn {
    Incompatible {
        chain_forms: Option<ChainFormsIdentifierDisagreement>,
        encoding: Option<EncodingIdentifierDisagreement>,
    },
}

pub type AdmitGenerateCompositionReturn =
    Result<AdmitGenerateCompositionSuccessReturn, AdmitGenerateCompositionErrorReturn>;

pub type AdmitGenerateCompositionFn = for<'a> fn(
    &AdmitGenerateCompositionDeps,
    AdmitGenerateCompositionParams,
    AdmitGenerateCompositionPayload<'a>,
) -> AdmitGenerateCompositionReturn;
