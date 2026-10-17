#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use encoding::{
    CanonicalFieldValue, CanonicalFields, CanonicalFieldsOverrides, build_canonical_fields,
};
use pairing::IPairingReference;
use serde_json::{Value, json};

use super::interface::{
    DecodeTarget, DecodeTargetConstructorParams, RejectionVector, RejectionVectorDescription,
    RejectionVectorDescriptionConstructorParams, RejectionVectorsDeps, RejectionVectorsFn,
    RejectionVectorsParams, RejectionVectorsPayload, RejectionVectorsReturn,
    RejectionVectorsSuccessReturn,
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
pub struct DecodeTargetConstructorParamsCorruptions {
    pub text: Option<Value>,
}

pub fn invalidate_decode_target_constructor_params(
    corruptions: DecodeTargetConstructorParamsCorruptions,
) -> Value {
    let params = build_decode_target_constructor_params(Default::default());
    let mut object = json!({ "text": params.text });
    if let Some(text) = corruptions.text {
        object["text"] = text;
    }
    object
}

#[derive(Default)]
pub struct RejectionVectorCorruptions {
    pub target: Option<CanonicalFieldValue>,
    pub input: Option<CanonicalFieldValue>,
}

pub fn invalidate_rejection_vector(corruptions: RejectionVectorCorruptions) -> CanonicalFields {
    let vector = build_rejection_vector(Default::default());
    build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            corruptions
                .target
                .unwrap_or_else(|| CanonicalFieldValue::Text(vector.target.as_str().to_string())),
            corruptions
                .input
                .unwrap_or(CanonicalFieldValue::Bytes(vector.input)),
        ]),
    })
}

#[derive(Default)]
pub struct RejectionVectorsSuccessReturnCorruptions {
    pub records: Option<Value>,
}

pub fn invalidate_rejection_vectors_success_return(
    corruptions: RejectionVectorsSuccessReturnCorruptions,
) -> Value {
    let success = build_rejection_vectors_success_return(Default::default());
    let mut object = json!({
        "records": success
            .records
            .into_iter()
            .map(|record| json!({ "target": record.target.as_str(), "input": record.input }))
            .collect::<Vec<Value>>(),
    });
    if let Some(records) = corruptions.records {
        object["records"] = records;
    }
    object
}

pub fn mock_rejection_vectors<'a, P: IPairingReference>(
    _deps: &RejectionVectorsDeps<'a, P>,
    _params: RejectionVectorsParams,
    _payload: RejectionVectorsPayload,
) -> RejectionVectorsReturn {
    let _: RejectionVectorsFn<'a, P> = mock_rejection_vectors;
    Ok(build_rejection_vectors_success_return(Default::default()))
}

#[derive(Default)]
pub struct RejectionVectorsDepsOverrides;

pub fn build_rejection_vectors_deps<'a, P: IPairingReference>(
    pairing: &'a P,
    _overrides: RejectionVectorsDepsOverrides,
) -> RejectionVectorsDeps<'a, P> {
    RejectionVectorsDeps { pairing }
}
