#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use pairing::IPairingArithmetic;
use random::IRandomSourceAdapter;
use serde_json::{Map, Value};

use super::interface::{
    FirstGroupVectorCounts, FirstGroupVectors, GroupOperationVector,
    GroupOperationVectorDescription, GroupOperationVectorDescriptionConstructorParams,
    GroupVectorsDeps, GroupVectorsFn, GroupVectorsParams, GroupVectorsPayload, GroupVectorsReturn,
    GroupVectorsSuccessReturn, MsmTermCount, MsmTermCountConstructorParams, MsmVectorSettings,
    PairingCheckVector, PairingCheckVectorDescription,
    PairingCheckVectorDescriptionConstructorParams, SecondGroupVectorCounts, SecondGroupVectors,
    VectorCount, VectorCountConstructorParams,
};

#[derive(Default)]
pub struct VectorCountConstructorParamsOverrides {
    pub count: Option<u32>,
}

pub fn build_vector_count_constructor_params(
    overrides: VectorCountConstructorParamsOverrides,
) -> VectorCountConstructorParams {
    VectorCountConstructorParams {
        count: overrides.count.unwrap_or(2),
    }
}

pub fn build_vector_count(overrides: VectorCountConstructorParamsOverrides) -> VectorCount {
    VectorCount::try_new(build_vector_count_constructor_params(overrides))
        .expect("built params are valid")
}

#[derive(Default)]
pub struct MsmTermCountConstructorParamsOverrides {
    pub count: Option<u32>,
}

pub fn build_msm_term_count_constructor_params(
    overrides: MsmTermCountConstructorParamsOverrides,
) -> MsmTermCountConstructorParams {
    MsmTermCountConstructorParams {
        count: overrides.count.unwrap_or(2),
    }
}

pub fn build_msm_term_count(overrides: MsmTermCountConstructorParamsOverrides) -> MsmTermCount {
    MsmTermCount::try_new(build_msm_term_count_constructor_params(overrides))
        .expect("built params are valid")
}

#[derive(Default)]
pub struct FirstGroupVectorCountsOverrides {
    pub g1_add: Option<VectorCount>,
    pub g1_mul: Option<VectorCount>,
    pub pairing_check: Option<VectorCount>,
}

pub fn build_first_group_vector_counts(
    overrides: FirstGroupVectorCountsOverrides,
) -> FirstGroupVectorCounts {
    FirstGroupVectorCounts {
        g1_add: overrides
            .g1_add
            .unwrap_or_else(|| build_vector_count(Default::default())),
        g1_mul: overrides
            .g1_mul
            .unwrap_or_else(|| build_vector_count(Default::default())),
        pairing_check: overrides
            .pairing_check
            .unwrap_or_else(|| build_vector_count(Default::default())),
    }
}

#[derive(Default)]
pub struct MsmVectorSettingsOverrides {
    pub count: Option<VectorCount>,
    pub terms: Option<MsmTermCount>,
}

pub fn build_msm_vector_settings(overrides: MsmVectorSettingsOverrides) -> MsmVectorSettings {
    MsmVectorSettings {
        count: overrides
            .count
            .unwrap_or_else(|| build_vector_count(Default::default())),
        terms: overrides
            .terms
            .unwrap_or_else(|| build_msm_term_count(Default::default())),
    }
}

#[derive(Default)]
pub struct SecondGroupVectorCountsOverrides {
    pub g2_add: Option<VectorCount>,
    pub g1_msm: Option<MsmVectorSettings>,
    pub g2_msm: Option<MsmVectorSettings>,
}

pub fn build_second_group_vector_counts(
    overrides: SecondGroupVectorCountsOverrides,
) -> SecondGroupVectorCounts {
    SecondGroupVectorCounts {
        g2_add: overrides
            .g2_add
            .unwrap_or_else(|| build_vector_count(Default::default())),
        g1_msm: overrides
            .g1_msm
            .unwrap_or_else(|| build_msm_vector_settings(Default::default())),
        g2_msm: overrides
            .g2_msm
            .unwrap_or_else(|| build_msm_vector_settings(Default::default())),
    }
}

#[derive(Default)]
pub struct GroupVectorsParamsOverrides {
    pub first_group: Option<FirstGroupVectorCounts>,
    pub second_group: Option<Option<SecondGroupVectorCounts>>,
}

pub fn build_group_vectors_params(overrides: GroupVectorsParamsOverrides) -> GroupVectorsParams {
    GroupVectorsParams {
        first_group: overrides
            .first_group
            .unwrap_or_else(|| build_first_group_vector_counts(Default::default())),
        second_group: overrides.second_group.unwrap_or(None),
    }
}

#[derive(Default)]
pub struct GroupOperationVectorOverrides {
    pub draws: Option<Vec<u8>>,
    pub input: Option<Vec<u8>>,
    pub output: Option<Vec<u8>>,
}

pub fn build_group_operation_vector(
    overrides: GroupOperationVectorOverrides,
) -> GroupOperationVector {
    GroupOperationVector {
        draws: overrides.draws.unwrap_or_else(|| vec![0x01; 64]),
        input: overrides.input.unwrap_or_else(|| vec![0x02; 128]),
        output: overrides.output.unwrap_or_else(|| vec![0x03; 64]),
    }
}

#[derive(Default)]
pub struct PairingCheckVectorOverrides {
    pub draws: Option<Vec<u8>>,
    pub input: Option<Vec<u8>>,
    pub output: Option<[u8; 32]>,
}

pub fn build_pairing_check_vector(overrides: PairingCheckVectorOverrides) -> PairingCheckVector {
    PairingCheckVector {
        draws: overrides.draws.unwrap_or_else(|| vec![0x01; 64]),
        input: overrides.input.unwrap_or_else(|| vec![0x02; 384]),
        output: overrides.output.unwrap_or_else(|| {
            let mut word = [0x00; 32];
            word[31] = 0x01;
            word
        }),
    }
}

#[derive(Default)]
pub struct GroupOperationVectorDescriptionConstructorParamsOverrides;

pub fn build_group_operation_vector_description_constructor_params(
    _overrides: GroupOperationVectorDescriptionConstructorParamsOverrides,
) -> GroupOperationVectorDescriptionConstructorParams {
    GroupOperationVectorDescriptionConstructorParams
}

#[derive(Default)]
pub struct PairingCheckVectorDescriptionConstructorParamsOverrides;

pub fn build_pairing_check_vector_description_constructor_params(
    _overrides: PairingCheckVectorDescriptionConstructorParamsOverrides,
) -> PairingCheckVectorDescriptionConstructorParams {
    PairingCheckVectorDescriptionConstructorParams
}

pub fn build_group_operation_vector_description(
    overrides: GroupOperationVectorDescriptionConstructorParamsOverrides,
) -> GroupOperationVectorDescription {
    let Ok(description) = GroupOperationVectorDescription::try_new(
        build_group_operation_vector_description_constructor_params(overrides),
    );
    description
}

pub fn build_pairing_check_vector_description(
    overrides: PairingCheckVectorDescriptionConstructorParamsOverrides,
) -> PairingCheckVectorDescription {
    let Ok(description) = PairingCheckVectorDescription::try_new(
        build_pairing_check_vector_description_constructor_params(overrides),
    );
    description
}

#[derive(Default)]
pub struct GroupVectorsPayloadOverrides;

pub fn build_group_vectors_payload(
    _overrides: GroupVectorsPayloadOverrides,
) -> GroupVectorsPayload {
    GroupVectorsPayload
}

#[derive(Default)]
pub struct FirstGroupVectorsOverrides {
    pub g1_add: Option<Vec<GroupOperationVector>>,
    pub g1_mul: Option<Vec<GroupOperationVector>>,
    pub pairing_check: Option<Vec<PairingCheckVector>>,
}

pub fn build_first_group_vectors(overrides: FirstGroupVectorsOverrides) -> FirstGroupVectors {
    FirstGroupVectors {
        g1_add: overrides
            .g1_add
            .unwrap_or_else(|| vec![build_group_operation_vector(Default::default())]),
        g1_mul: overrides
            .g1_mul
            .unwrap_or_else(|| vec![build_group_operation_vector(Default::default())]),
        pairing_check: overrides
            .pairing_check
            .unwrap_or_else(|| vec![build_pairing_check_vector(Default::default())]),
    }
}

#[derive(Default)]
pub struct SecondGroupVectorsOverrides {
    pub g2_add: Option<Vec<GroupOperationVector>>,
    pub g1_msm: Option<Vec<GroupOperationVector>>,
    pub g2_msm: Option<Vec<GroupOperationVector>>,
}

pub fn build_second_group_vectors(overrides: SecondGroupVectorsOverrides) -> SecondGroupVectors {
    SecondGroupVectors {
        g2_add: overrides
            .g2_add
            .unwrap_or_else(|| vec![build_group_operation_vector(Default::default())]),
        g1_msm: overrides
            .g1_msm
            .unwrap_or_else(|| vec![build_group_operation_vector(Default::default())]),
        g2_msm: overrides
            .g2_msm
            .unwrap_or_else(|| vec![build_group_operation_vector(Default::default())]),
    }
}

#[derive(Default)]
pub struct GroupVectorsSuccessReturnOverrides {
    pub first_group: Option<FirstGroupVectors>,
    pub second_group: Option<Option<SecondGroupVectors>>,
}

pub fn build_group_vectors_success_return(
    overrides: GroupVectorsSuccessReturnOverrides,
) -> GroupVectorsSuccessReturn {
    GroupVectorsSuccessReturn {
        first_group: overrides
            .first_group
            .unwrap_or_else(|| build_first_group_vectors(Default::default())),
        second_group: overrides.second_group.unwrap_or(None),
    }
}

fn byte_array(bytes: &[u8]) -> Value {
    Value::Array(bytes.iter().map(|byte| Value::from(*byte)).collect())
}

fn first_group_vector_counts_fields(counts: &FirstGroupVectorCounts) -> Map<String, Value> {
    let mut fields = Map::new();
    fields.insert("g1_add".to_string(), Value::from(counts.g1_add.get()));
    fields.insert("g1_mul".to_string(), Value::from(counts.g1_mul.get()));
    fields.insert(
        "pairing_check".to_string(),
        Value::from(counts.pairing_check.get()),
    );
    fields
}

fn msm_vector_settings_fields(settings: &MsmVectorSettings) -> Map<String, Value> {
    let mut fields = Map::new();
    fields.insert("count".to_string(), Value::from(settings.count.get()));
    fields.insert("terms".to_string(), Value::from(settings.terms.get()));
    fields
}

fn second_group_vector_counts_fields(counts: &SecondGroupVectorCounts) -> Map<String, Value> {
    let mut fields = Map::new();
    fields.insert("g2_add".to_string(), Value::from(counts.g2_add.get()));
    fields.insert(
        "g1_msm".to_string(),
        Value::Object(msm_vector_settings_fields(&counts.g1_msm)),
    );
    fields.insert(
        "g2_msm".to_string(),
        Value::Object(msm_vector_settings_fields(&counts.g2_msm)),
    );
    fields
}

fn group_vectors_params_fields(params: &GroupVectorsParams) -> Map<String, Value> {
    let mut fields = Map::new();
    fields.insert(
        "first_group".to_string(),
        Value::Object(first_group_vector_counts_fields(&params.first_group)),
    );
    fields.insert(
        "second_group".to_string(),
        params.second_group.as_ref().map_or(Value::Null, |counts| {
            Value::Object(second_group_vector_counts_fields(counts))
        }),
    );
    fields
}

fn group_operation_vector_fields(vector: &GroupOperationVector) -> Map<String, Value> {
    let mut fields = Map::new();
    fields.insert("draws".to_string(), byte_array(&vector.draws));
    fields.insert("input".to_string(), byte_array(&vector.input));
    fields.insert("output".to_string(), byte_array(&vector.output));
    fields
}

fn pairing_check_vector_fields(vector: &PairingCheckVector) -> Map<String, Value> {
    let mut fields = Map::new();
    fields.insert("draws".to_string(), byte_array(&vector.draws));
    fields.insert("input".to_string(), byte_array(&vector.input));
    fields.insert("output".to_string(), byte_array(&vector.output));
    fields
}

fn group_operation_vector_array(vectors: &[GroupOperationVector]) -> Value {
    Value::Array(
        vectors
            .iter()
            .map(|vector| Value::Object(group_operation_vector_fields(vector)))
            .collect(),
    )
}

fn pairing_check_vector_array(vectors: &[PairingCheckVector]) -> Value {
    Value::Array(
        vectors
            .iter()
            .map(|vector| Value::Object(pairing_check_vector_fields(vector)))
            .collect(),
    )
}

fn first_group_vectors_fields(vectors: &FirstGroupVectors) -> Map<String, Value> {
    let mut fields = Map::new();
    fields.insert(
        "g1_add".to_string(),
        group_operation_vector_array(&vectors.g1_add),
    );
    fields.insert(
        "g1_mul".to_string(),
        group_operation_vector_array(&vectors.g1_mul),
    );
    fields.insert(
        "pairing_check".to_string(),
        pairing_check_vector_array(&vectors.pairing_check),
    );
    fields
}

fn second_group_vectors_fields(vectors: &SecondGroupVectors) -> Map<String, Value> {
    let mut fields = Map::new();
    fields.insert(
        "g2_add".to_string(),
        group_operation_vector_array(&vectors.g2_add),
    );
    fields.insert(
        "g1_msm".to_string(),
        group_operation_vector_array(&vectors.g1_msm),
    );
    fields.insert(
        "g2_msm".to_string(),
        group_operation_vector_array(&vectors.g2_msm),
    );
    fields
}

fn group_vectors_success_return_fields(value: &GroupVectorsSuccessReturn) -> Map<String, Value> {
    let mut fields = Map::new();
    fields.insert(
        "first_group".to_string(),
        Value::Object(first_group_vectors_fields(&value.first_group)),
    );
    fields.insert(
        "second_group".to_string(),
        value.second_group.as_ref().map_or(Value::Null, |vectors| {
            Value::Object(second_group_vectors_fields(vectors))
        }),
    );
    fields
}

#[derive(Default)]
pub struct VectorCountConstructorParamsCorruptions {
    pub count: Option<Value>,
}

pub fn invalidate_vector_count_constructor_params(
    corruptions: VectorCountConstructorParamsCorruptions,
) -> Value {
    let params = build_vector_count_constructor_params(Default::default());
    let mut fields = Map::new();
    fields.insert("count".to_string(), Value::from(params.count));
    if let Some(count) = corruptions.count {
        fields.insert("count".to_string(), count);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct MsmTermCountConstructorParamsCorruptions {
    pub count: Option<Value>,
}

pub fn invalidate_msm_term_count_constructor_params(
    corruptions: MsmTermCountConstructorParamsCorruptions,
) -> Value {
    let params = build_msm_term_count_constructor_params(Default::default());
    let mut fields = Map::new();
    fields.insert("count".to_string(), Value::from(params.count));
    if let Some(count) = corruptions.count {
        fields.insert("count".to_string(), count);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct FirstGroupVectorCountsCorruptions {
    pub g1_add: Option<Value>,
    pub g1_mul: Option<Value>,
    pub pairing_check: Option<Value>,
}

pub fn invalidate_first_group_vector_counts(
    corruptions: FirstGroupVectorCountsCorruptions,
) -> Value {
    let mut fields =
        first_group_vector_counts_fields(&build_first_group_vector_counts(Default::default()));
    if let Some(g1_add) = corruptions.g1_add {
        fields.insert("g1_add".to_string(), g1_add);
    }
    if let Some(g1_mul) = corruptions.g1_mul {
        fields.insert("g1_mul".to_string(), g1_mul);
    }
    if let Some(pairing_check) = corruptions.pairing_check {
        fields.insert("pairing_check".to_string(), pairing_check);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct MsmVectorSettingsCorruptions {
    pub count: Option<Value>,
    pub terms: Option<Value>,
}

pub fn invalidate_msm_vector_settings(corruptions: MsmVectorSettingsCorruptions) -> Value {
    let mut fields = msm_vector_settings_fields(&build_msm_vector_settings(Default::default()));
    if let Some(count) = corruptions.count {
        fields.insert("count".to_string(), count);
    }
    if let Some(terms) = corruptions.terms {
        fields.insert("terms".to_string(), terms);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct SecondGroupVectorCountsCorruptions {
    pub g2_add: Option<Value>,
    pub g1_msm: Option<Value>,
    pub g2_msm: Option<Value>,
}

pub fn invalidate_second_group_vector_counts(
    corruptions: SecondGroupVectorCountsCorruptions,
) -> Value {
    let mut fields =
        second_group_vector_counts_fields(&build_second_group_vector_counts(Default::default()));
    if let Some(g2_add) = corruptions.g2_add {
        fields.insert("g2_add".to_string(), g2_add);
    }
    if let Some(g1_msm) = corruptions.g1_msm {
        fields.insert("g1_msm".to_string(), g1_msm);
    }
    if let Some(g2_msm) = corruptions.g2_msm {
        fields.insert("g2_msm".to_string(), g2_msm);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct GroupVectorsParamsCorruptions {
    pub first_group: Option<Value>,
    pub second_group: Option<Value>,
}

pub fn invalidate_group_vectors_params(corruptions: GroupVectorsParamsCorruptions) -> Value {
    let mut fields = group_vectors_params_fields(&build_group_vectors_params(Default::default()));
    if let Some(first_group) = corruptions.first_group {
        fields.insert("first_group".to_string(), first_group);
    }
    if let Some(second_group) = corruptions.second_group {
        fields.insert("second_group".to_string(), second_group);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct GroupOperationVectorCorruptions {
    pub draws: Option<Value>,
    pub input: Option<Value>,
    pub output: Option<Value>,
}

pub fn invalidate_group_operation_vector(corruptions: GroupOperationVectorCorruptions) -> Value {
    let mut fields =
        group_operation_vector_fields(&build_group_operation_vector(Default::default()));
    if let Some(draws) = corruptions.draws {
        fields.insert("draws".to_string(), draws);
    }
    if let Some(input) = corruptions.input {
        fields.insert("input".to_string(), input);
    }
    if let Some(output) = corruptions.output {
        fields.insert("output".to_string(), output);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct PairingCheckVectorCorruptions {
    pub draws: Option<Value>,
    pub input: Option<Value>,
    pub output: Option<Value>,
}

pub fn invalidate_pairing_check_vector(corruptions: PairingCheckVectorCorruptions) -> Value {
    let mut fields = pairing_check_vector_fields(&build_pairing_check_vector(Default::default()));
    if let Some(draws) = corruptions.draws {
        fields.insert("draws".to_string(), draws);
    }
    if let Some(input) = corruptions.input {
        fields.insert("input".to_string(), input);
    }
    if let Some(output) = corruptions.output {
        fields.insert("output".to_string(), output);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct FirstGroupVectorsCorruptions {
    pub g1_add: Option<Value>,
    pub g1_mul: Option<Value>,
    pub pairing_check: Option<Value>,
}

pub fn invalidate_first_group_vectors(corruptions: FirstGroupVectorsCorruptions) -> Value {
    let mut fields = first_group_vectors_fields(&build_first_group_vectors(Default::default()));
    if let Some(g1_add) = corruptions.g1_add {
        fields.insert("g1_add".to_string(), g1_add);
    }
    if let Some(g1_mul) = corruptions.g1_mul {
        fields.insert("g1_mul".to_string(), g1_mul);
    }
    if let Some(pairing_check) = corruptions.pairing_check {
        fields.insert("pairing_check".to_string(), pairing_check);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct SecondGroupVectorsCorruptions {
    pub g2_add: Option<Value>,
    pub g1_msm: Option<Value>,
    pub g2_msm: Option<Value>,
}

pub fn invalidate_second_group_vectors(corruptions: SecondGroupVectorsCorruptions) -> Value {
    let mut fields = second_group_vectors_fields(&build_second_group_vectors(Default::default()));
    if let Some(g2_add) = corruptions.g2_add {
        fields.insert("g2_add".to_string(), g2_add);
    }
    if let Some(g1_msm) = corruptions.g1_msm {
        fields.insert("g1_msm".to_string(), g1_msm);
    }
    if let Some(g2_msm) = corruptions.g2_msm {
        fields.insert("g2_msm".to_string(), g2_msm);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct GroupVectorsSuccessReturnCorruptions {
    pub first_group: Option<Value>,
    pub second_group: Option<Value>,
}

pub fn invalidate_group_vectors_success_return(
    corruptions: GroupVectorsSuccessReturnCorruptions,
) -> Value {
    let mut fields = group_vectors_success_return_fields(&build_group_vectors_success_return(
        Default::default(),
    ));
    if let Some(first_group) = corruptions.first_group {
        fields.insert("first_group".to_string(), first_group);
    }
    if let Some(second_group) = corruptions.second_group {
        fields.insert("second_group".to_string(), second_group);
    }
    Value::Object(fields)
}

pub fn mock_group_vectors<'a, P: IPairingArithmetic>(
    _deps: &GroupVectorsDeps<'a, P>,
    _params: GroupVectorsParams,
    _payload: GroupVectorsPayload,
) -> GroupVectorsReturn {
    let _: GroupVectorsFn<'a, P> = mock_group_vectors;
    Ok(build_group_vectors_success_return(Default::default()))
}

#[derive(Default)]
pub struct GroupVectorsDepsOverrides;

pub fn build_group_vectors_deps<'a, P: IPairingArithmetic>(
    pairing: &'a P,
    random: &'a dyn IRandomSourceAdapter,
    _overrides: GroupVectorsDepsOverrides,
) -> GroupVectorsDeps<'a, P> {
    GroupVectorsDeps { pairing, random }
}
