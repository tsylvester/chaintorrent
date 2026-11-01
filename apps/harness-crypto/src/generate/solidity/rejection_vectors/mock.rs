#![allow(dead_code)]

use encoding::{
    CanonicalFieldValue, CanonicalFields, CanonicalFieldsOverrides, build_canonical_fields,
};
use pairing::IPairingReference;
use serde_json::{Value, json};

use super::interface::{
    DecodeTarget, DecodeTargetConstructorParams, RejectionVector, RejectionVectorDescription,
    RejectionVectorDescriptionConstructorParams, RejectionVectorsDeps, RejectionVectorsParams,
    RejectionVectorsPayload, RejectionVectorsReturn, RejectionVectorsSuccessReturn,
};

#[derive(Default)]
pub struct DecodeTargetConstructorParamsOverrides {
    pub text: Option<String>,
}

pub fn build_decode_target_constructor_params(
    overrides: DecodeTargetConstructorParamsOverrides,
) -> DecodeTargetConstructorParams {
    DecodeTargetConstructorParams {
        text: overrides.text.unwrap_or_else(|| "G2".to_string()),
    }
}

#[derive(Default)]
pub struct DecodeTargetConstructorParamsCorruptions {
    pub text: Option<Value>,
}

pub fn invalidate_decode_target_constructor_params(
    corruptions: DecodeTargetConstructorParamsCorruptions,
) -> Value {
    let params = build_decode_target_constructor_params(Default::default());
    let mut value = json!({ "text": params.text });
    if let Some(text) = corruptions.text {
        value["text"] = text;
    }
    value
}

#[derive(Default)]
pub struct RejectionVectorOverrides {
    pub target: Option<DecodeTarget>,
    pub input: Option<Vec<u8>>,
}

pub fn build_rejection_vector(overrides: RejectionVectorOverrides) -> RejectionVector {
    RejectionVector {
        target: overrides.target.unwrap_or(DecodeTarget::G2),
        input: overrides.input.unwrap_or_else(|| vec![0x01; 128]),
    }
}

#[derive(Default)]
pub struct RejectionVectorCorruptions {
    pub target: Option<CanonicalFieldValue>,
    pub input: Option<CanonicalFieldValue>,
}

pub fn invalidate_rejection_vector(corruptions: RejectionVectorCorruptions) -> CanonicalFields {
    let vector = build_rejection_vector(Default::default());
    let mut values = vec![
        CanonicalFieldValue::Text(vector.target.as_str().to_string()),
        CanonicalFieldValue::Bytes(vector.input),
    ];
    if let Some(target) = corruptions.target {
        values[0] = target;
    }
    if let Some(input) = corruptions.input {
        values[1] = input;
    }
    build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(values),
    })
}

#[derive(Default)]
pub struct RejectionVectorDescriptionConstructorParamsOverrides;

pub fn build_rejection_vector_description_constructor_params(
    _overrides: RejectionVectorDescriptionConstructorParamsOverrides,
) -> RejectionVectorDescriptionConstructorParams {
    RejectionVectorDescriptionConstructorParams
}

pub fn build_rejection_vector_description(
    overrides: RejectionVectorDescriptionConstructorParamsOverrides,
) -> RejectionVectorDescription {
    let Ok(description) = RejectionVectorDescription::try_new(
        build_rejection_vector_description_constructor_params(overrides),
    );
    description
}

#[derive(Default)]
pub struct RejectionVectorsDepsOverrides;

pub fn build_rejection_vectors_deps<'a, P: IPairingReference>(
    pairing: &'a P,
    _overrides: RejectionVectorsDepsOverrides,
) -> RejectionVectorsDeps<'a, P> {
    RejectionVectorsDeps { pairing }
}

#[derive(Default)]
pub struct RejectionVectorsParamsOverrides;

pub fn build_rejection_vectors_params(
    _overrides: RejectionVectorsParamsOverrides,
) -> RejectionVectorsParams {
    RejectionVectorsParams
}

#[derive(Default)]
pub struct RejectionVectorsPayloadOverrides;

pub fn build_rejection_vectors_payload(
    _overrides: RejectionVectorsPayloadOverrides,
) -> RejectionVectorsPayload {
    RejectionVectorsPayload
}

#[derive(Default)]
pub struct RejectionVectorsSuccessReturnOverrides {
    pub records: Option<Vec<RejectionVector>>,
}

pub fn build_rejection_vectors_success_return(
    overrides: RejectionVectorsSuccessReturnOverrides,
) -> RejectionVectorsSuccessReturn {
    RejectionVectorsSuccessReturn {
        records: overrides
            .records
            .unwrap_or_else(|| vec![build_rejection_vector(Default::default())]),
    }
}

#[derive(Default)]
pub struct RejectionVectorsSuccessReturnCorruptions {
    pub records: Option<Value>,
}

pub fn invalidate_rejection_vectors_success_return(
    corruptions: RejectionVectorsSuccessReturnCorruptions,
) -> Value {
    let success = build_rejection_vectors_success_return(Default::default());
    let mut value = json!({
        "records": success
            .records
            .iter()
            .map(|record| json!({ "target": record.target.as_str(), "input": record.input }))
            .collect::<Vec<Value>>()
    });
    if let Some(records) = corruptions.records {
        value["records"] = records;
    }
    value
}

pub fn mock_rejection_vectors<'a, P: IPairingReference>(
    _deps: &RejectionVectorsDeps<'a, P>,
    _params: RejectionVectorsParams,
    _payload: RejectionVectorsPayload,
) -> RejectionVectorsReturn {
    Ok(build_rejection_vectors_success_return(Default::default()))
}
