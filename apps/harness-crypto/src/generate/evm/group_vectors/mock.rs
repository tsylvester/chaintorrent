#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{
    FirstGroupVectorCounts, GroupOperationVector, GroupOperationVectorDescription,
    GroupOperationVectorDescriptionConstructorParams, GroupVectorsParams, MsmTermCount,
    MsmTermCountConstructorParams, MsmVectorSettings, PairingCheckVector,
    PairingCheckVectorDescription, PairingCheckVectorDescriptionConstructorParams,
    SecondGroupVectorCounts, VectorCount, VectorCountConstructorParams,
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

pub fn build_group_operation_vector_description() -> GroupOperationVectorDescription {
    let Ok(description) =
        GroupOperationVectorDescription::try_new(GroupOperationVectorDescriptionConstructorParams);
    description
}

pub fn build_pairing_check_vector_description() -> PairingCheckVectorDescription {
    let Ok(description) =
        PairingCheckVectorDescription::try_new(PairingCheckVectorDescriptionConstructorParams);
    description
}
