#![allow(dead_code)]

use super::interface::{
    AdmitKdfSelectionDeps, AdmitKdfSelectionFn, AdmitKdfSelectionParams, AdmitKdfSelectionPayload,
    AdmitKdfSelectionReturn, AdmitKdfSelectionSuccessReturn,
};
use crate::factory::provides::{KdfConcrete, KdfConcreteOverrides, build_kdf_concrete};
use serde_json::{Map, Value};

#[derive(Default)]
pub struct AdmitKdfSelectionDepsOverrides;

pub fn build_admit_kdf_selection_deps(
    _overrides: AdmitKdfSelectionDepsOverrides,
) -> AdmitKdfSelectionDeps {
    AdmitKdfSelectionDeps
}

#[derive(Default)]
pub struct AdmitKdfSelectionParamsOverrides {
    pub concrete: Option<KdfConcrete>,
}

pub fn build_admit_kdf_selection_params(
    overrides: AdmitKdfSelectionParamsOverrides,
) -> AdmitKdfSelectionParams {
    AdmitKdfSelectionParams {
        concrete: overrides
            .concrete
            .unwrap_or_else(|| build_kdf_concrete(KdfConcrete::Blake3Keyed, KdfConcreteOverrides)),
    }
}

#[derive(Default)]
pub struct AdmitKdfSelectionParamsCorruptions {
    pub concrete: Option<Value>,
}

pub fn invalidate_admit_kdf_selection_params(
    corruptions: AdmitKdfSelectionParamsCorruptions,
) -> Value {
    let params = build_admit_kdf_selection_params(Default::default());
    let mut fields = Map::new();
    fields.insert("concrete".to_string(), Value::from(params.concrete.name()));
    if let Some(concrete) = corruptions.concrete {
        fields.insert("concrete".to_string(), concrete);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct AdmitKdfSelectionPayloadOverrides {
    pub settings: Option<Map<String, Value>>,
}

pub fn build_admit_kdf_selection_payload(
    overrides: AdmitKdfSelectionPayloadOverrides,
) -> AdmitKdfSelectionPayload {
    AdmitKdfSelectionPayload {
        settings: overrides.settings.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct AdmitKdfSelectionPayloadCorruptions {
    pub settings: Option<Value>,
}

pub fn invalidate_admit_kdf_selection_payload(
    corruptions: AdmitKdfSelectionPayloadCorruptions,
) -> Value {
    let payload = build_admit_kdf_selection_payload(Default::default());
    corruptions
        .settings
        .unwrap_or(Value::Object(payload.settings))
}

#[derive(Default)]
pub struct AdmitKdfSelectionSuccessReturnOverrides {
    pub selection: Option<KdfConcrete>,
}

pub fn build_admit_kdf_selection_success_return(
    overrides: AdmitKdfSelectionSuccessReturnOverrides,
) -> AdmitKdfSelectionSuccessReturn {
    AdmitKdfSelectionSuccessReturn {
        selection: overrides
            .selection
            .unwrap_or_else(|| build_kdf_concrete(KdfConcrete::Blake3Keyed, KdfConcreteOverrides)),
    }
}

#[derive(Default)]
pub struct AdmitKdfSelectionSuccessReturnCorruptions {
    pub selection: Option<Value>,
}

pub fn invalidate_admit_kdf_selection_success_return(
    corruptions: AdmitKdfSelectionSuccessReturnCorruptions,
) -> Value {
    let success = build_admit_kdf_selection_success_return(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "selection".to_string(),
        Value::from(success.selection.name()),
    );
    if let Some(selection) = corruptions.selection {
        fields.insert("selection".to_string(), selection);
    }
    Value::Object(fields)
}

pub fn mock_admit_kdf_selection(
    _deps: &AdmitKdfSelectionDeps,
    _params: AdmitKdfSelectionParams,
    _payload: Value,
) -> AdmitKdfSelectionReturn {
    Ok(build_admit_kdf_selection_success_return(Default::default()))
}

const _: AdmitKdfSelectionFn = mock_admit_kdf_selection;
