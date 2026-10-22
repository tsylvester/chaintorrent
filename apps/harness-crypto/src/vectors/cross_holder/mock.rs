#![allow(dead_code, clippy::too_many_arguments)]

use kem::{EncapsulatedValue, ICredentialKemAdapter, IdentityScope};
use random::IRandomSourceAdapter;
use serde_json::{Map, Value};

use super::interface::{
    CrossHolderDeps, CrossHolderFn, CrossHolderParams, CrossHolderPayload, CrossHolderReturn,
    CrossHolderSuccessReturn, CrossHolderVector,
};
use crate::{VectorCount, build_vector_count};

#[derive(Default)]
pub struct CrossHolderVectorOverrides {
    pub draws: Option<Vec<u8>>,
}

pub fn build_cross_holder_vector<K: ICredentialKemAdapter>(
    parameter_set: K::ParameterSet,
    identity_element: K::IdentityElement,
    issued: K::Credential,
    successor: K::Credential,
    capsule: K::Capsule,
    encapsulated: EncapsulatedValue,
    issued_decapsulated: EncapsulatedValue,
    successor_decapsulated: EncapsulatedValue,
    overrides: CrossHolderVectorOverrides,
) -> CrossHolderVector<K> {
    CrossHolderVector {
        draws: overrides.draws.unwrap_or_else(|| vec![0x01; 64]),
        parameter_set,
        identity_element,
        issued,
        successor,
        capsule,
        encapsulated,
        issued_decapsulated,
        successor_decapsulated,
    }
}

#[derive(Default)]
pub struct CrossHolderParamsOverrides {
    pub count: Option<VectorCount>,
    pub scope: Option<IdentityScope>,
}

pub fn build_cross_holder_params(overrides: CrossHolderParamsOverrides) -> CrossHolderParams {
    CrossHolderParams {
        count: overrides
            .count
            .unwrap_or_else(|| build_vector_count(Default::default())),
        scope: overrides.scope.unwrap_or(IdentityScope::Entitlement),
    }
}

fn cross_holder_params_fields(params: &CrossHolderParams) -> Map<String, Value> {
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
pub struct CrossHolderParamsCorruptions {
    pub count: Option<Value>,
    pub scope: Option<Value>,
}

pub fn invalidate_cross_holder_params(corruptions: CrossHolderParamsCorruptions) -> Value {
    let mut fields = cross_holder_params_fields(&build_cross_holder_params(Default::default()));
    if let Some(count) = corruptions.count {
        fields.insert("count".to_string(), count);
    }
    if let Some(scope) = corruptions.scope {
        fields.insert("scope".to_string(), scope);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct CrossHolderPayloadOverrides;

pub fn build_cross_holder_payload<'a>(
    identity: &'a [u8],
    _overrides: CrossHolderPayloadOverrides,
) -> CrossHolderPayload<'a> {
    CrossHolderPayload { identity }
}

#[derive(Default)]
pub struct CrossHolderDepsOverrides;

pub fn build_cross_holder_deps<'a, K: ICredentialKemAdapter>(
    kem: &'a K,
    random: &'a dyn IRandomSourceAdapter,
    _overrides: CrossHolderDepsOverrides,
) -> CrossHolderDeps<'a, K> {
    CrossHolderDeps { kem, random }
}

pub struct CrossHolderSuccessReturnOverrides<K: ICredentialKemAdapter> {
    pub vectors: Option<Vec<CrossHolderVector<K>>>,
}

impl<K: ICredentialKemAdapter> Default for CrossHolderSuccessReturnOverrides<K> {
    fn default() -> Self {
        Self { vectors: None }
    }
}

pub fn build_cross_holder_success_return<K: ICredentialKemAdapter>(
    overrides: CrossHolderSuccessReturnOverrides<K>,
) -> CrossHolderSuccessReturn<K> {
    CrossHolderSuccessReturn {
        vectors: overrides.vectors.unwrap_or_default(),
    }
}

pub fn mock_cross_holder<'a, K: ICredentialKemAdapter>(
    _deps: &CrossHolderDeps<'a, K>,
    _params: CrossHolderParams,
    _payload: CrossHolderPayload<'_>,
) -> CrossHolderReturn<K> {
    let _: CrossHolderFn<'a, K> = mock_cross_holder;
    Ok(build_cross_holder_success_return(Default::default()))
}
