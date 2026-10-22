use domain::AssetIdentityHashTryNewErrorReturn;
use kem::{
    DeriveIdentityErrorReturn, ICredentialKemAdapter, IdentityScope, IssueErrorReturn,
    RerandomizeErrorReturn, SetupErrorReturn,
};
use random::{FillBytesErrorReturn, IRandomSourceAdapter};

use crate::VectorCount;

pub struct RerandomizationVector<K: ICredentialKemAdapter> {
    pub draws: Vec<u8>,
    pub parameter_set: K::ParameterSet,
    pub identity_element: K::IdentityElement,
    pub issued: K::Credential,
    pub rerandomized: K::Credential,
    pub issued_is_valid: bool,
    pub rerandomized_is_valid: bool,
}

pub struct RerandomizationDeps<'a, K: ICredentialKemAdapter> {
    pub kem: &'a K,
    pub random: &'a dyn IRandomSourceAdapter,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RerandomizationParams {
    pub count: VectorCount,
    pub scope: IdentityScope,
}

pub struct RerandomizationPayload<'a> {
    pub identity: &'a [u8],
}

pub struct RerandomizationSuccessReturn<K: ICredentialKemAdapter> {
    pub vectors: Vec<RerandomizationVector<K>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RerandomizationErrorReturn {
    AssetIdentityLength { actual: usize },
    AssetIdentityHash(AssetIdentityHashTryNewErrorReturn),
    FillBytes(FillBytesErrorReturn),
    Setup(SetupErrorReturn),
    DeriveIdentity(DeriveIdentityErrorReturn),
    Issue(IssueErrorReturn),
    Rerandomize(RerandomizeErrorReturn),
}

pub type RerandomizationReturn<K> =
    Result<RerandomizationSuccessReturn<K>, RerandomizationErrorReturn>;

pub type RerandomizationFn<'a, K> = fn(
    &RerandomizationDeps<'a, K>,
    RerandomizationParams,
    RerandomizationPayload<'_>,
) -> RerandomizationReturn<K>;
