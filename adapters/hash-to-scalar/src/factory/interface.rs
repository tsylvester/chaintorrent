use crate::domain_tag::provides::DomainTag;
use crate::keccak256::provides::Keccak256HashToScalarErrorReturn;
use core::convert::Infallible;
use pairing::ISampleUniformScalar;

pub const HASH_TO_SCALAR_INTERFACE_VERSION: u32 = 1;

#[derive(PartialEq, Eq)]
pub enum HashToScalarIdentifier {
    Keccak256V1,
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
    Keccak256(Keccak256HashToScalarErrorReturn),
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

pub enum HashToScalarConcrete {
    Keccak256,
}

pub struct CreateHashToScalarDeps;

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
}

pub type CreateHashToScalarReturn<S> =
    Result<CreateHashToScalarSuccessReturn<S>, CreateHashToScalarErrorReturn>;

pub type CreateHashToScalarFn<S> = fn(
    &CreateHashToScalarDeps,
    CreateHashToScalarParams,
    CreateHashToScalarPayload,
) -> CreateHashToScalarReturn<S>;
