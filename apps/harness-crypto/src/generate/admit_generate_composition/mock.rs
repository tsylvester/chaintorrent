#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_arguments
)]

use chain::{ChainFormsDeclaration, ChainFormsIdentifier};
use encoding::{EncodingDeclaration, EncodingIdentifier};

use super::super::factory::provides::GenerateConcreteKind;
use super::super::lookup_generate_declaration::provides::{
    LookupGenerateDeclarationFn, mock_lookup_generate_declaration,
};
use super::interface::{
    AdmitGenerateCompositionDeps, AdmitGenerateCompositionFn, AdmitGenerateCompositionParams,
    AdmitGenerateCompositionPayload, AdmitGenerateCompositionReturn,
    AdmitGenerateCompositionSuccessReturn, ChainFormsIdentifierDisagreement,
    EncodingIdentifierDisagreement,
};

#[derive(Default)]
pub struct AdmitGenerateCompositionDepsOverrides {
    pub lookup_generate_declaration: Option<LookupGenerateDeclarationFn>,
}

pub fn build_admit_generate_composition_deps(
    overrides: AdmitGenerateCompositionDepsOverrides,
) -> AdmitGenerateCompositionDeps {
    AdmitGenerateCompositionDeps {
        lookup_generate_declaration: overrides
            .lookup_generate_declaration
            .unwrap_or(mock_lookup_generate_declaration),
    }
}

#[derive(Default)]
pub struct AdmitGenerateCompositionParamsOverrides {
    pub identifier: Option<GenerateConcreteKind>,
}

pub fn build_admit_generate_composition_params(
    overrides: AdmitGenerateCompositionParamsOverrides,
) -> AdmitGenerateCompositionParams {
    AdmitGenerateCompositionParams {
        identifier: overrides
            .identifier
            .unwrap_or(GenerateConcreteKind::Solidity),
    }
}

#[derive(Default)]
pub struct AdmitGenerateCompositionPayloadOverrides;

pub fn build_admit_generate_composition_payload<'a>(
    chain_forms: &'a ChainFormsDeclaration,
    encoder: &'a EncodingDeclaration,
    _overrides: AdmitGenerateCompositionPayloadOverrides,
) -> AdmitGenerateCompositionPayload<'a> {
    AdmitGenerateCompositionPayload {
        chain_forms,
        encoder,
    }
}

#[derive(Default)]
pub struct AdmitGenerateCompositionSuccessReturnOverrides;

pub fn build_admit_generate_composition_success_return(
    _overrides: AdmitGenerateCompositionSuccessReturnOverrides,
) -> AdmitGenerateCompositionSuccessReturn {
    AdmitGenerateCompositionSuccessReturn
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

#[derive(Default)]
pub struct EncodingIdentifierDisagreementOverrides {
    pub required: Option<EncodingIdentifier>,
    pub declared: Option<EncodingIdentifier>,
}

pub fn build_encoding_identifier_disagreement(
    overrides: EncodingIdentifierDisagreementOverrides,
) -> EncodingIdentifierDisagreement {
    EncodingIdentifierDisagreement {
        required: overrides
            .required
            .unwrap_or(EncodingIdentifier::EthereumAbiV1),
        declared: overrides.declared.unwrap_or(EncodingIdentifier::Mock),
    }
}

pub fn mock_admit_generate_composition(
    _deps: &AdmitGenerateCompositionDeps,
    _params: AdmitGenerateCompositionParams,
    _payload: AdmitGenerateCompositionPayload<'_>,
) -> AdmitGenerateCompositionReturn {
    let _: AdmitGenerateCompositionFn = mock_admit_generate_composition;
    Ok(build_admit_generate_composition_success_return(
        Default::default(),
    ))
}
