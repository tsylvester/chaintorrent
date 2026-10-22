#![allow(dead_code, clippy::too_many_arguments)]

use kem::{ICredentialKemAdapter, IdentityScope};
use random::IRandomSourceAdapter;
use serde_json::{Map, Value};

use super::interface::{
    RerandomizationDeps, RerandomizationFn, RerandomizationParams, RerandomizationPayload,
    RerandomizationReturn, RerandomizationSuccessReturn, RerandomizationVector,
};
use crate::{VectorCount, build_vector_count};

#[derive(Default)]
pub struct RerandomizationVectorOverrides {
    pub draws: Option<Vec<u8>>,
    pub issued_is_valid: Option<bool>,
    pub rerandomized_is_valid: Option<bool>,
}

pub fn build_rerandomization_vector<K: ICredentialKemAdapter>(
    parameter_set: K::ParameterSet,
    identity_element: K::IdentityElement,
    issued: K::Credential,
    rerandomized: K::Credential,
    overrides: RerandomizationVectorOverrides,
) -> RerandomizationVector<K> {
    RerandomizationVector {
        draws: overrides.draws.unwrap_or_else(|| vec![0x01; 64]),
        parameter_set,
        identity_element,
        issued,
        rerandomized,
        issued_is_valid: overrides.issued_is_valid.unwrap_or(true),
        rerandomized_is_valid: overrides.rerandomized_is_valid.unwrap_or(true),
    }
}

#[derive(Default)]
pub struct RerandomizationParamsOverrides {
    pub count: Option<VectorCount>,
    pub scope: Option<IdentityScope>,
}

pub fn build_rerandomization_params(
    overrides: RerandomizationParamsOverrides,
) -> RerandomizationParams {
    RerandomizationParams {
        count: overrides
            .count
            .unwrap_or_else(|| build_vector_count(Default::default())),
        scope: overrides.scope.unwrap_or(IdentityScope::Entitlement),
    }
}

fn rerandomization_params_fields(params: &RerandomizationParams) -> Map<String, Value> {
    let mut fields = Map::new();
    fields.insert("count".to_string(), Value::from(params.count.get()));
    let scope = match params.scope {
        IdentityScope::Entitlement => "Entitlement",
        IdentityScope::Asset => "Asset",
    };
    fields.insert("scope".to_string(), Value::from(scope));
    fields
}

#[derive(Default)]
pub struct RerandomizationParamsCorruptions {
    pub count: Option<Value>,
    pub scope: Option<Value>,
}

pub fn invalidate_rerandomization_params(corruptions: RerandomizationParamsCorruptions) -> Value {
    let mut fields =
        rerandomization_params_fields(&build_rerandomization_params(Default::default()));
    if let Some(count) = corruptions.count {
        fields.insert("count".to_string(), count);
    }
    if let Some(scope) = corruptions.scope {
        fields.insert("scope".to_string(), scope);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct RerandomizationPayloadOverrides;

pub fn build_rerandomization_payload<'a>(
    identity: &'a [u8],
    _overrides: RerandomizationPayloadOverrides,
) -> RerandomizationPayload<'a> {
    RerandomizationPayload { identity }
}

#[derive(Default)]
pub struct RerandomizationDepsOverrides;

pub fn build_rerandomization_deps<'a, K: ICredentialKemAdapter>(
    kem: &'a K,
    random: &'a dyn IRandomSourceAdapter,
    _overrides: RerandomizationDepsOverrides,
) -> RerandomizationDeps<'a, K> {
    RerandomizationDeps { kem, random }
}

pub struct RerandomizationSuccessReturnOverrides<K: ICredentialKemAdapter> {
    pub vectors: Option<Vec<RerandomizationVector<K>>>,
}

impl<K: ICredentialKemAdapter> Default for RerandomizationSuccessReturnOverrides<K> {
    fn default() -> Self {
        Self { vectors: None }
    }
}

pub fn build_rerandomization_success_return<K: ICredentialKemAdapter>(
    overrides: RerandomizationSuccessReturnOverrides<K>,
) -> RerandomizationSuccessReturn<K> {
    RerandomizationSuccessReturn {
        vectors: overrides.vectors.unwrap_or_default(),
    }
}

pub fn mock_rerandomization<'a, K: ICredentialKemAdapter>(
    _deps: &RerandomizationDeps<'a, K>,
    _params: RerandomizationParams,
    _payload: RerandomizationPayload<'_>,
) -> RerandomizationReturn<K> {
    let _: RerandomizationFn<'a, K> = mock_rerandomization;
    Ok(build_rerandomization_success_return(Default::default()))
}
