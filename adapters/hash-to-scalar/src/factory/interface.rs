use crate::domain_tag::provides::DomainTag;
use crate::keccak256::provides::Keccak256HashToScalarErrorReturn;
use crate::lookup_hash_to_scalar_declaration::provides::LookupHashToScalarDeclarationFn;
use core::convert::Infallible;
use core::marker::PhantomData;
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

pub trait IHashToScalarConsumer<S: ISampleUniformScalar + Clone> {
    type Output;

    fn consume_hash_to_scalar<K: IHashToScalarAdapter<S>>(
        &self,
        params: ConsumeHashToScalarParams,
        payload: ConsumeHashToScalarPayload<S, K>,
    ) -> Self::Output;
}

pub struct ConsumeHashToScalarParams;

pub struct ConsumeHashToScalarPayload<S: ISampleUniformScalar + Clone, K: IHashToScalarAdapter<S>> {
    pub adapter: K,
    pub declaration: HashToScalarDeclaration,
    pub(super) scalar: PhantomData<S>,
}

pub struct CreateHashToScalarDeps<C> {
    pub consumer: C,
    pub lookup: LookupHashToScalarDeclarationFn,
}

#[derive(Clone, Copy)]
pub struct CreateHashToScalarParams {
    pub concrete: HashToScalarConcrete,
}

pub struct CreateHashToScalarPayload;

pub struct CreateHashToScalarSuccessReturn<O> {
    pub output: O,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CreateHashToScalarErrorReturn {
    Keccak256(Infallible),
    #[cfg(any(test, feature = "mocks"))]
    Mock(Infallible),
}

pub type CreateHashToScalarReturn<O> =
    Result<CreateHashToScalarSuccessReturn<O>, CreateHashToScalarErrorReturn>;

pub type CreateHashToScalarFn<S, C> =
    fn(
        &CreateHashToScalarDeps<C>,
        CreateHashToScalarParams,
        CreateHashToScalarPayload,
    ) -> CreateHashToScalarReturn<<C as IHashToScalarConsumer<S>>::Output>;
