#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{
    CreateRandomSourceDeps, CreateRandomSourceParams, CreateRandomSourcePayload,
    CreateRandomSourceReturn, CreateRandomSourceSuccessReturn, FillBytesParams, FillBytesPayload,
    FillBytesReturn, FillBytesSuccessReturn, IRandomSourceAdapter, RANDOM_SOURCE_INTERFACE_VERSION,
    RandomSourceDeclaration, RandomSourceKind,
};
use domain::{Secret, SecretConstructorParamsOverrides, build_secret};

#[derive(Default)]
pub struct RandomSourceDeclarationOverrides {
    pub source: Option<RandomSourceKind>,
    pub adapter_version: Option<u32>,
    pub interface_version: Option<u32>,
}

pub fn build_random_source_declaration(
    overrides: RandomSourceDeclarationOverrides,
) -> RandomSourceDeclaration {
    RandomSourceDeclaration {
        source: overrides
            .source
            .unwrap_or(RandomSourceKind::OperatingSystem),
        adapter_version: overrides.adapter_version.unwrap_or(1),
        interface_version: overrides
            .interface_version
            .unwrap_or(RANDOM_SOURCE_INTERFACE_VERSION),
    }
}

#[derive(Default)]
pub struct FillBytesPayloadOverrides {
    pub length: Option<usize>,
}

pub fn build_fill_bytes_payload(overrides: FillBytesPayloadOverrides) -> FillBytesPayload {
    FillBytesPayload {
        length: overrides.length.unwrap_or(32),
    }
}

#[derive(Default)]
pub struct FillBytesSuccessReturnOverrides {
    pub bytes: Option<Secret<Vec<u8>>>,
}

pub fn build_fill_bytes_success_return(
    overrides: FillBytesSuccessReturnOverrides,
) -> FillBytesSuccessReturn {
    FillBytesSuccessReturn {
        bytes: overrides.bytes.unwrap_or_else(|| {
            build_secret::<Vec<u8>>(SecretConstructorParamsOverrides::default())
        }),
    }
}

pub struct MockIRandomSourceAdapter;

impl IRandomSourceAdapter for MockIRandomSourceAdapter {
    fn fill_bytes(&self, _params: FillBytesParams, _payload: FillBytesPayload) -> FillBytesReturn {
        Ok(build_fill_bytes_success_return(Default::default()))
    }
}

#[derive(Default)]
pub struct CreateRandomSourcePayloadOverrides {
    pub kind: Option<RandomSourceKind>,
}

pub fn build_create_random_source_payload(
    overrides: CreateRandomSourcePayloadOverrides,
) -> CreateRandomSourcePayload {
    CreateRandomSourcePayload {
        kind: overrides.kind.unwrap_or(RandomSourceKind::OperatingSystem),
    }
}

#[derive(Default)]
pub struct CreateRandomSourceSuccessReturnOverrides {
    pub adapter: Option<Box<dyn IRandomSourceAdapter>>,
    pub declaration: Option<RandomSourceDeclaration>,
}

pub fn build_create_random_source_success_return(
    overrides: CreateRandomSourceSuccessReturnOverrides,
) -> CreateRandomSourceSuccessReturn {
    CreateRandomSourceSuccessReturn {
        adapter: overrides
            .adapter
            .unwrap_or_else(|| Box::new(MockIRandomSourceAdapter)),
        declaration: overrides
            .declaration
            .unwrap_or_else(|| build_random_source_declaration(Default::default())),
    }
}

pub fn mock_create_random_source(
    _deps: &CreateRandomSourceDeps,
    _params: CreateRandomSourceParams,
    _payload: CreateRandomSourcePayload,
) -> CreateRandomSourceReturn {
    Ok(build_create_random_source_success_return(Default::default()))
}
