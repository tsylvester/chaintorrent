#![allow(dead_code)]

use super::interface::{
    LookupEncodingDeclarationDeps, LookupEncodingDeclarationFn, LookupEncodingDeclarationParams,
    LookupEncodingDeclarationPayload, LookupEncodingDeclarationReturn,
    LookupEncodingDeclarationSuccessReturn,
};
use crate::factory::provides::{
    EncodingConcrete, EncodingConcreteOverrides, EncodingDeclaration, EncodingDeclarationOverrides,
    build_encoding_concrete, build_encoding_declaration, mock_type_name,
};

#[derive(Default)]
pub struct LookupEncodingDeclarationDepsOverrides;

pub fn build_lookup_encoding_declaration_deps(
    _overrides: LookupEncodingDeclarationDepsOverrides,
) -> LookupEncodingDeclarationDeps {
    LookupEncodingDeclarationDeps
}

#[derive(Default)]
pub struct LookupEncodingDeclarationParamsOverrides {
    pub concrete: Option<EncodingConcrete>,
}

pub fn build_lookup_encoding_declaration_params(
    overrides: LookupEncodingDeclarationParamsOverrides,
) -> LookupEncodingDeclarationParams {
    LookupEncodingDeclarationParams {
        concrete: overrides.concrete.unwrap_or_else(|| {
            build_encoding_concrete(EncodingConcrete::Abi, EncodingConcreteOverrides)
        }),
    }
}

#[derive(Default)]
pub struct LookupEncodingDeclarationPayloadOverrides;

pub fn build_lookup_encoding_declaration_payload(
    _overrides: LookupEncodingDeclarationPayloadOverrides,
) -> LookupEncodingDeclarationPayload {
    LookupEncodingDeclarationPayload
}

#[derive(Default)]
pub struct LookupEncodingDeclarationSuccessReturnOverrides {
    pub declaration: Option<EncodingDeclaration>,
}

pub fn build_lookup_encoding_declaration_success_return(
    overrides: LookupEncodingDeclarationSuccessReturnOverrides,
) -> LookupEncodingDeclarationSuccessReturn {
    LookupEncodingDeclarationSuccessReturn {
        declaration: overrides.declaration.unwrap_or_else(|| {
            build_encoding_declaration(mock_type_name, EncodingDeclarationOverrides::default())
        }),
    }
}

pub fn mock_lookup_encoding_declaration(
    _deps: &LookupEncodingDeclarationDeps,
    _params: LookupEncodingDeclarationParams,
    _payload: LookupEncodingDeclarationPayload,
) -> LookupEncodingDeclarationReturn {
    Ok(build_lookup_encoding_declaration_success_return(
        LookupEncodingDeclarationSuccessReturnOverrides::default(),
    ))
}

const _: LookupEncodingDeclarationFn = mock_lookup_encoding_declaration;
