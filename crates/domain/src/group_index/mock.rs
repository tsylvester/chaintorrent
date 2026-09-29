#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{GroupIndex, GroupIndexConstructorParams};

#[derive(Default)]
pub struct GroupIndexConstructorParamsOverrides {
    pub value: Option<u64>,
}

pub fn build_group_index_constructor_params(
    overrides: GroupIndexConstructorParamsOverrides,
) -> GroupIndexConstructorParams {
    GroupIndexConstructorParams {
        value: overrides.value.unwrap_or(7),
    }
}

pub fn build_group_index(overrides: GroupIndexConstructorParamsOverrides) -> GroupIndex {
    let Ok(index) = GroupIndex::try_new(build_group_index_constructor_params(overrides));
    index
}
