#![allow(dead_code)]

use super::interface::{
    LookupEnvelopeDeclarationDeps, LookupEnvelopeDeclarationFn, LookupEnvelopeDeclarationParams,
    LookupEnvelopeDeclarationPayload, LookupEnvelopeDeclarationReturn,
    LookupEnvelopeDeclarationSuccessReturn,
};
use crate::factory::provides::{
    KeyAgreementConcrete, KeyAgreementDeclaration, MockIKeyAgreementAdapterFailureMode,
    build_key_agreement_declaration, invalidate_key_agreement_declaration,
};
use serde_json::Value;

#[derive(Default)]
pub struct LookupEnvelopeDeclarationDepsOverrides;

pub fn build_lookup_envelope_declaration_deps(
    _overrides: LookupEnvelopeDeclarationDepsOverrides,
) -> LookupEnvelopeDeclarationDeps {
    LookupEnvelopeDeclarationDeps
}

#[derive(Default)]
pub struct LookupEnvelopeDeclarationParamsOverrides {
    pub concrete: Option<KeyAgreementConcrete>,
}

pub fn build_lookup_envelope_declaration_params(
    overrides: LookupEnvelopeDeclarationParamsOverrides,
) -> LookupEnvelopeDeclarationParams {
    LookupEnvelopeDeclarationParams {
        concrete: overrides.concrete.unwrap_or(KeyAgreementConcrete::Mock(
            MockIKeyAgreementAdapterFailureMode::NoFailure,
        )),
    }
}

#[derive(Default)]
pub struct LookupEnvelopeDeclarationPayloadOverrides;

pub fn build_lookup_envelope_declaration_payload(
    _overrides: LookupEnvelopeDeclarationPayloadOverrides,
) -> LookupEnvelopeDeclarationPayload {
    LookupEnvelopeDeclarationPayload
}

#[derive(Default)]
pub struct LookupEnvelopeDeclarationSuccessReturnOverrides {
    pub declaration: Option<KeyAgreementDeclaration>,
}

pub fn build_lookup_envelope_declaration_success_return(
    overrides: LookupEnvelopeDeclarationSuccessReturnOverrides,
) -> LookupEnvelopeDeclarationSuccessReturn {
    LookupEnvelopeDeclarationSuccessReturn {
        declaration: overrides
            .declaration
            .unwrap_or_else(|| build_key_agreement_declaration(Default::default())),
    }
}

#[derive(Default)]
pub struct LookupEnvelopeDeclarationSuccessReturnCorruptions {
    pub declaration: Option<Value>,
}

pub fn invalidate_lookup_envelope_declaration_success_return(
    corruptions: LookupEnvelopeDeclarationSuccessReturnCorruptions,
) -> Value {
    let mut fields = serde_json::Map::new();
    fields.insert(
        "declaration".to_string(),
        invalidate_key_agreement_declaration(Default::default()),
    );
    if let Some(declaration) = corruptions.declaration {
        fields.insert("declaration".to_string(), declaration);
    }
    Value::Object(fields)
}

pub fn mock_lookup_envelope_declaration(
    _deps: &LookupEnvelopeDeclarationDeps,
    _params: LookupEnvelopeDeclarationParams,
    _payload: LookupEnvelopeDeclarationPayload,
) -> LookupEnvelopeDeclarationReturn {
    let _: LookupEnvelopeDeclarationFn = mock_lookup_envelope_declaration;
    Ok(build_lookup_envelope_declaration_success_return(
        Default::default(),
    ))
}
