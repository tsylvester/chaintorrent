#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{
    CreateHashToScalarDeps, CreateHashToScalarParams, CreateHashToScalarPayload,
    CreateHashToScalarReturn, CreateHashToScalarSuccessReturn, HASH_TO_SCALAR_INTERFACE_VERSION,
    HashToScalarConcrete, HashToScalarDeclaration, HashToScalarIdentifier, HashToScalarParams,
    HashToScalarPayload, HashToScalarReturn, HashToScalarSuccessReturn, IHashToScalarAdapter,
};
use core::marker::PhantomData;
use pairing::ISampleUniformScalar;

#[derive(Default)]
pub struct HashToScalarDeclarationOverrides {
    pub identifier: Option<HashToScalarIdentifier>,
    pub adapter_version: Option<u32>,
    pub interface_version: Option<u32>,
}

pub fn build_hash_to_scalar_declaration(
    overrides: HashToScalarDeclarationOverrides,
) -> HashToScalarDeclaration {
    HashToScalarDeclaration {
        identifier: overrides
            .identifier
            .unwrap_or(HashToScalarIdentifier::Keccak256V1),
        adapter_version: overrides.adapter_version.unwrap_or(1),
        interface_version: overrides
            .interface_version
            .unwrap_or(HASH_TO_SCALAR_INTERFACE_VERSION),
    }
}

#[derive(Default)]
pub struct HashToScalarSuccessReturnOverrides<S> {
    pub scalar: Option<S>,
}

pub fn build_hash_to_scalar_success_return<S: Default>(
    overrides: HashToScalarSuccessReturnOverrides<S>,
) -> HashToScalarSuccessReturn<S> {
    HashToScalarSuccessReturn {
        scalar: overrides.scalar.unwrap_or_default(),
    }
}

pub struct MockIHashToScalarAdapter<S> {
    pub scalar: PhantomData<S>,
}

impl<S: ISampleUniformScalar + Clone + Default> IHashToScalarAdapter<S>
    for MockIHashToScalarAdapter<S>
{
    fn hash_to_scalar(
        &self,
        _params: HashToScalarParams<'_>,
        _payload: HashToScalarPayload<'_>,
    ) -> HashToScalarReturn<S> {
        Ok(build_hash_to_scalar_success_return(Default::default()))
    }
}

#[derive(Default)]
pub struct CreateHashToScalarParamsOverrides {
    pub concrete: Option<HashToScalarConcrete>,
    pub identifier: Option<HashToScalarIdentifier>,
}

pub fn build_create_hash_to_scalar_params(
    overrides: CreateHashToScalarParamsOverrides,
) -> CreateHashToScalarParams {
    CreateHashToScalarParams {
        concrete: overrides
            .concrete
            .unwrap_or(HashToScalarConcrete::Keccak256),
        identifier: overrides
            .identifier
            .unwrap_or(HashToScalarIdentifier::Keccak256V1),
    }
}

#[derive(Default)]
pub struct CreateHashToScalarSuccessReturnOverrides<S> {
    pub adapter: Option<Box<dyn IHashToScalarAdapter<S>>>,
    pub declaration: Option<HashToScalarDeclaration>,
}

pub fn build_create_hash_to_scalar_success_return<
    S: ISampleUniformScalar + Clone + Default + 'static,
>(
    overrides: CreateHashToScalarSuccessReturnOverrides<S>,
) -> CreateHashToScalarSuccessReturn<S> {
    CreateHashToScalarSuccessReturn {
        adapter: overrides.adapter.unwrap_or_else(|| {
            Box::new(MockIHashToScalarAdapter {
                scalar: PhantomData,
            })
        }),
        declaration: overrides
            .declaration
            .unwrap_or_else(|| build_hash_to_scalar_declaration(Default::default())),
    }
}

pub fn mock_create_hash_to_scalar<S: ISampleUniformScalar + Clone + Default + 'static>(
    _deps: &CreateHashToScalarDeps,
    _params: CreateHashToScalarParams,
    _payload: CreateHashToScalarPayload,
) -> CreateHashToScalarReturn<S> {
    Ok(build_create_hash_to_scalar_success_return(
        Default::default(),
    ))
}
