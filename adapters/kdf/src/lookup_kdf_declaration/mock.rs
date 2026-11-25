#![allow(dead_code)]

use super::interface::{
    LookupKdfDeclarationDeps, LookupKdfDeclarationFn, LookupKdfDeclarationParams,
    LookupKdfDeclarationPayload, LookupKdfDeclarationReturn, LookupKdfDeclarationSuccessReturn,
};
use crate::factory::provides::{
    KdfConcrete, KdfDeclaration, build_kdf_concrete, build_kdf_declaration,
    invalidate_kdf_declaration,
};
use serde_json::{Map, Value};

#[derive(Default)]
pub struct LookupKdfDeclarationDepsOverrides;

pub fn build_lookup_kdf_declaration_deps(
    _overrides: LookupKdfDeclarationDepsOverrides,
) -> LookupKdfDeclarationDeps {
    LookupKdfDeclarationDeps
}

#[derive(Default)]
pub struct LookupKdfDeclarationParamsOverrides {
    pub concrete: Option<KdfConcrete>,
}

pub fn build_lookup_kdf_declaration_params(
    overrides: LookupKdfDeclarationParamsOverrides,
) -> LookupKdfDeclarationParams {
    LookupKdfDeclarationParams {
        concrete: overrides
            .concrete
            .unwrap_or_else(|| build_kdf_concrete(KdfConcrete::Blake3Keyed, Default::default())),
    }
}

#[derive(Default)]
pub struct LookupKdfDeclarationParamsCorruptions {
    pub concrete: Option<Value>,
}

pub fn invalidate_lookup_kdf_declaration_params(
    corruptions: LookupKdfDeclarationParamsCorruptions,
) -> Value {
    let params = build_lookup_kdf_declaration_params(Default::default());
    let mut fields = Map::new();
    fields.insert("concrete".to_string(), Value::from(params.concrete.name()));
    if let Some(concrete) = corruptions.concrete {
        fields.insert("concrete".to_string(), concrete);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct LookupKdfDeclarationPayloadOverrides;

pub fn build_lookup_kdf_declaration_payload(
    _overrides: LookupKdfDeclarationPayloadOverrides,
) -> LookupKdfDeclarationPayload {
    LookupKdfDeclarationPayload
}

#[derive(Default)]
pub struct LookupKdfDeclarationSuccessReturnOverrides {
    pub declaration: Option<KdfDeclaration>,
}

pub fn build_lookup_kdf_declaration_success_return(
    overrides: LookupKdfDeclarationSuccessReturnOverrides,
) -> LookupKdfDeclarationSuccessReturn {
    LookupKdfDeclarationSuccessReturn {
        declaration: overrides
            .declaration
            .unwrap_or_else(|| build_kdf_declaration(Default::default())),
    }
}

#[derive(Default)]
pub struct LookupKdfDeclarationSuccessReturnCorruptions {
    pub declaration: Option<Value>,
}

pub fn invalidate_lookup_kdf_declaration_success_return(
    corruptions: LookupKdfDeclarationSuccessReturnCorruptions,
) -> Value {
    let mut fields = Map::new();
    fields.insert(
        "declaration".to_string(),
        invalidate_kdf_declaration(Default::default()),
    );
    if let Some(declaration) = corruptions.declaration {
        fields.insert("declaration".to_string(), declaration);
    }
    Value::Object(fields)
}

pub fn mock_lookup_kdf_declaration(
    _deps: &LookupKdfDeclarationDeps,
    _params: LookupKdfDeclarationParams,
    _payload: LookupKdfDeclarationPayload,
) -> LookupKdfDeclarationReturn {
    Ok(build_lookup_kdf_declaration_success_return(
        Default::default(),
    ))
}

const _: LookupKdfDeclarationFn = mock_lookup_kdf_declaration;
