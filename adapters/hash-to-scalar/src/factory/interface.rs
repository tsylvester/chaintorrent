use crate::domain_tag::provides::DomainTag;
use crate::keccak256::provides::Keccak256HashToScalarErrorReturn;
use core::convert::Infallible;
use pairing::{ISampleUniformScalar, SampleUniformScalarErrorReturn};
use serde::Serialize;

pub const HASH_TO_SCALAR_INTERFACE_VERSION: u32 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum HashToScalarIdentifier {
    Keccak256V1,
    #[cfg(any(test, feature = "mocks"))]
    Mock,
}

pub struct HashToScalarDeclaration {
    pub identifier: HashToScalarIdentifier,
    pub adapter_version: u32,
    pub interface_version: u32,
}

pub struct HashToScalarParams<'a> {
    pub tag: &'a DomainTag,
}

pub struct HashToScalarPayload<'a> {
    pub message: &'a [u8],
}

pub struct HashToScalarSuccessReturn<S> {
    pub scalar: S,
}

#[derive(Debug, PartialEq, Eq)]
pub enum HashToScalarErrorReturn {
    Sampling(SampleUniformScalarErrorReturn),
    Keccak256(Keccak256HashToScalarErrorReturn),
}

#[cfg(any(test, feature = "mocks"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum MockIHashToScalarAdapterFailureMode {
    Succeeds,
    SamplingRefused,
}

pub type HashToScalarReturn<S> = Result<HashToScalarSuccessReturn<S>, HashToScalarErrorReturn>;

pub trait IHashToScalarAdapter<S: ISampleUniformScalar + Clone> {
    fn declaration(&self) -> HashToScalarDeclaration;

    fn hash_to_scalar(
        &self,
        params: HashToScalarParams<'_>,
        payload: HashToScalarPayload<'_>,
    ) -> HashToScalarReturn<S>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum HashToScalarConcrete {
    Keccak256,
    #[cfg(any(test, feature = "mocks"))]
    Mock(MockIHashToScalarAdapterFailureMode),
}

pub struct CreateHashToScalarDeps;

#[derive(Clone, Copy)]
pub struct CreateHashToScalarParams {
    pub concrete: HashToScalarConcrete,
    pub identifier: HashToScalarIdentifier,
}

pub struct CreateHashToScalarPayload;

pub struct CreateHashToScalarSuccessReturn<S> {
    pub adapter: Box<dyn IHashToScalarAdapter<S>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CreateHashToScalarErrorReturn {
    UnsupportedHashToScalarIdentifier,
    Keccak256(Infallible),
    #[cfg(any(test, feature = "mocks"))]
    Mock(Infallible),
}

pub type CreateHashToScalarReturn<S> =
    Result<CreateHashToScalarSuccessReturn<S>, CreateHashToScalarErrorReturn>;

pub type CreateHashToScalarFn<S> = fn(
    &CreateHashToScalarDeps,
    CreateHashToScalarParams,
    CreateHashToScalarPayload,
) -> CreateHashToScalarReturn<S>;

pub const DECLARED_HASH_TO_SCALAR_SELECTIONS: &[CreateHashToScalarParams] =
    &[CreateHashToScalarParams {
        concrete: HashToScalarConcrete::Keccak256,
        identifier: HashToScalarIdentifier::Keccak256V1,
    }];
