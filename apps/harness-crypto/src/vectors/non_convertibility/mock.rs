#![allow(dead_code, clippy::too_many_arguments)]

use kem::{ICredentialKemAdapter, IdentityScope};
use random::IRandomSourceAdapter;
use serde_json::{Map, Value};

use super::interface::{
    NonConvertibilityDeps, NonConvertibilityFn, NonConvertibilityParams, NonConvertibilityPayload,
    NonConvertibilityReturn, NonConvertibilitySuccessReturn, NonConvertibilityVector,
};
use crate::{VectorCount, build_vector_count};

#[derive(Default)]
pub struct NonConvertibilityVectorOverrides {
    pub draws: Option<Vec<u8>>,
    pub is_valid: Option<bool>,
    pub other_is_valid: Option<bool>,
}

pub fn build_non_convertibility_vector<K: ICredentialKemAdapter>(
    credential: K::Credential,
    overrides: NonConvertibilityVectorOverrides,
) -> NonConvertibilityVector<K> {
    NonConvertibilityVector {
        draws: overrides.draws.unwrap_or_else(|| vec![0x01; 64]),
        credential,
        is_valid: overrides.is_valid.unwrap_or(true),
        other_is_valid: overrides.other_is_valid.unwrap_or(false),
    }
}

#[derive(Default)]
pub struct NonConvertibilityParamsOverrides {
    pub count: Option<VectorCount>,
    pub scope: Option<IdentityScope>,
}

pub fn build_non_convertibility_params(
    overrides: NonConvertibilityParamsOverrides,
) -> NonConvertibilityParams {
    NonConvertibilityParams {
        count: overrides
            .count
            .unwrap_or_else(|| build_vector_count(Default::default())),
        scope: overrides.scope.unwrap_or(IdentityScope::Entitlement),
    }
}

fn non_convertibility_params_fields(params: &NonConvertibilityParams) -> Map<String, Value> {
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
pub struct NonConvertibilityParamsCorruptions {
    pub count: Option<Value>,
    pub scope: Option<Value>,
}

pub fn invalidate_non_convertibility_params(
    corruptions: NonConvertibilityParamsCorruptions,
) -> Value {
    let mut fields =
        non_convertibility_params_fields(&build_non_convertibility_params(Default::default()));
    if let Some(count) = corruptions.count {
        fields.insert("count".to_string(), count);
    }
    if let Some(scope) = corruptions.scope {
        fields.insert("scope".to_string(), scope);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct NonConvertibilityPayloadOverrides;

pub fn build_non_convertibility_payload<'a>(
    identity: &'a [u8],
    other_identity: &'a [u8],
    _overrides: NonConvertibilityPayloadOverrides,
) -> NonConvertibilityPayload<'a> {
    NonConvertibilityPayload {
        identity,
        other_identity,
    }
}

#[derive(Default)]
pub struct NonConvertibilityDepsOverrides;

pub fn build_non_convertibility_deps<'a, K: ICredentialKemAdapter>(
    kem: &'a K,
    random: &'a dyn IRandomSourceAdapter,
    _overrides: NonConvertibilityDepsOverrides,
) -> NonConvertibilityDeps<'a, K> {
    NonConvertibilityDeps { kem, random }
}

pub struct NonConvertibilitySuccessReturnOverrides<K: ICredentialKemAdapter> {
    pub vectors: Option<Vec<NonConvertibilityVector<K>>>,
}

impl<K: ICredentialKemAdapter> Default for NonConvertibilitySuccessReturnOverrides<K> {
    fn default() -> Self {
        Self { vectors: None }
    }
}

pub fn build_non_convertibility_success_return<K: ICredentialKemAdapter>(
    overrides: NonConvertibilitySuccessReturnOverrides<K>,
) -> NonConvertibilitySuccessReturn<K> {
    NonConvertibilitySuccessReturn {
        vectors: overrides.vectors.unwrap_or_default(),
    }
}

pub fn mock_non_convertibility<'a, K: ICredentialKemAdapter>(
    _deps: &NonConvertibilityDeps<'a, K>,
    _params: NonConvertibilityParams,
    _payload: NonConvertibilityPayload<'_>,
) -> NonConvertibilityReturn<K> {
    let _: NonConvertibilityFn<'a, K> = mock_non_convertibility;
    Ok(build_non_convertibility_success_return(Default::default()))
}
