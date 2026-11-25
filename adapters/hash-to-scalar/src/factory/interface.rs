use crate::domain_tag::provides::DomainTag;
use crate::keccak256::provides::Keccak256HashToScalarErrorReturn;
use core::convert::Infallible;
use pairing::{ISampleUniformScalar, SampleUniformScalarErrorReturn};

pub const HASH_TO_SCALAR_INTERFACE_VERSION: u32 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HashToScalarIdentifier {
    Keccak256V1,
}

impl HashToScalarIdentifier {
    pub const fn name(self) -> &'static str {
        match self {
            HashToScalarIdentifier::Keccak256V1 => "keccak256_v1",
        }
    }

    pub const fn wire_code(self) -> u8 {
        match self {
            HashToScalarIdentifier::Keccak256V1 => 1,
        }
    }
}

pub const HASH_TO_SCALAR_IDENTIFIERS: &[HashToScalarIdentifier] =
    &[HashToScalarIdentifier::Keccak256V1];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HashToScalarConcrete {
    Keccak256,
    #[cfg(any(test, feature = "mocks"))]
    Mock(MockIHashToScalarAdapterFailureMode),
}

impl HashToScalarConcrete {
    pub const fn name(self) -> &'static str {
        match self {
            HashToScalarConcrete::Keccak256 => "keccak256",
            #[cfg(any(test, feature = "mocks"))]
            HashToScalarConcrete::Mock(_) => "mock",
        }
    }
}

pub const HASH_TO_SCALAR_CONCRETES: &[HashToScalarConcrete] = &[HashToScalarConcrete::Keccak256];

pub struct HashToScalarDeclaration {
    pub concrete: HashToScalarConcrete,
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MockIHashToScalarAdapterFailureMode {
    Succeeds,
    SamplingRefused,
}

pub type HashToScalarReturn<S> = Result<HashToScalarSuccessReturn<S>, HashToScalarErrorReturn>;

pub trait IHashToScalarAdapter<S: ISampleUniformScalar + Clone> {
    fn hash_to_scalar(
        &self,
        params: HashToScalarParams<'_>,
        payload: HashToScalarPayload<'_>,
    ) -> HashToScalarReturn<S>;
}

pub struct CreateHashToScalarDeps;

#[derive(Clone, Copy)]
pub struct CreateHashToScalarParams {
    pub concrete: HashToScalarConcrete,
}

pub struct CreateHashToScalarPayload;

pub struct CreateHashToScalarSuccessReturn<S> {
    pub adapter: Box<dyn IHashToScalarAdapter<S>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CreateHashToScalarErrorReturn {
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
