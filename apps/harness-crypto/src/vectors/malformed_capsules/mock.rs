#![allow(dead_code, clippy::too_many_arguments)]

use kem::{ICredentialKemAdapter, IdentityScope};
use random::IRandomSourceAdapter;
use serde_json::{Map, Value};

use super::interface::{
    MalformedCapsulesDeps, MalformedCapsulesFn, MalformedCapsulesParams, MalformedCapsulesPayload,
    MalformedCapsulesReturn, MalformedCapsulesSuccessReturn, MalformedCapsulesVector,
};
use crate::{VectorCount, build_vector_count};

#[derive(Default)]
pub struct MalformedCapsulesVectorOverrides {
    pub draws: Option<Vec<u8>>,
    pub is_well_formed: Option<bool>,
    pub displaced_is_well_formed: Option<bool>,
}

pub fn build_malformed_capsules_vector<K: ICredentialKemAdapter>(
    parameter_set: K::ParameterSet,
    capsule: K::Capsule,
    displaced: K::Capsule,
    overrides: MalformedCapsulesVectorOverrides,
) -> MalformedCapsulesVector<K> {
    MalformedCapsulesVector {
        draws: overrides.draws.unwrap_or_else(|| vec![0x01; 64]),
        parameter_set,
        capsule,
        displaced,
        is_well_formed: overrides.is_well_formed.unwrap_or(true),
        displaced_is_well_formed: overrides.displaced_is_well_formed.unwrap_or(false),
    }
}

#[derive(Default)]
pub struct MalformedCapsulesParamsOverrides {
    pub count: Option<VectorCount>,
    pub scope: Option<IdentityScope>,
}

pub fn build_malformed_capsules_params(
    overrides: MalformedCapsulesParamsOverrides,
) -> MalformedCapsulesParams {
    MalformedCapsulesParams {
        count: overrides
            .count
            .unwrap_or_else(|| build_vector_count(Default::default())),
        scope: overrides.scope.unwrap_or(IdentityScope::Entitlement),
    }
}

fn malformed_capsules_params_fields(params: &MalformedCapsulesParams) -> Map<String, Value> {
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
pub struct MalformedCapsulesParamsCorruptions {
    pub count: Option<Value>,
    pub scope: Option<Value>,
}

pub fn invalidate_malformed_capsules_params(
    corruptions: MalformedCapsulesParamsCorruptions,
) -> Value {
    let mut fields =
        malformed_capsules_params_fields(&build_malformed_capsules_params(Default::default()));
    if let Some(count) = corruptions.count {
        fields.insert("count".to_string(), count);
    }
    if let Some(scope) = corruptions.scope {
        fields.insert("scope".to_string(), scope);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct MalformedCapsulesPayloadOverrides;

pub fn build_malformed_capsules_payload<'a>(
    identity: &'a [u8],
    _overrides: MalformedCapsulesPayloadOverrides,
) -> MalformedCapsulesPayload<'a> {
    MalformedCapsulesPayload { identity }
}

#[derive(Default)]
pub struct MalformedCapsulesDepsOverrides;

pub fn build_malformed_capsules_deps<'a, K: ICredentialKemAdapter>(
    kem: &'a K,
    random: &'a dyn IRandomSourceAdapter,
    _overrides: MalformedCapsulesDepsOverrides,
) -> MalformedCapsulesDeps<'a, K> {
    MalformedCapsulesDeps { kem, random }
}

pub struct MalformedCapsulesSuccessReturnOverrides<K: ICredentialKemAdapter> {
    pub vectors: Option<Vec<MalformedCapsulesVector<K>>>,
}

impl<K: ICredentialKemAdapter> Default for MalformedCapsulesSuccessReturnOverrides<K> {
    fn default() -> Self {
        Self { vectors: None }
    }
}

pub fn build_malformed_capsules_success_return<K: ICredentialKemAdapter>(
    overrides: MalformedCapsulesSuccessReturnOverrides<K>,
) -> MalformedCapsulesSuccessReturn<K> {
    MalformedCapsulesSuccessReturn {
        vectors: overrides.vectors.unwrap_or_default(),
    }
}

pub fn mock_malformed_capsules<'a, K: ICredentialKemAdapter>(
    _deps: &MalformedCapsulesDeps<'a, K>,
    _params: MalformedCapsulesParams,
    _payload: MalformedCapsulesPayload<'_>,
) -> MalformedCapsulesReturn<K> {
    let _: MalformedCapsulesFn<'a, K> = mock_malformed_capsules;
    Ok(build_malformed_capsules_success_return(Default::default()))
}
