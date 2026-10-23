use kem::{
    DeriveIdentityErrorReturn, ICredentialKemAdapter, IdentityScope, IssueErrorReturn,
    SetupErrorReturn,
};
use random::{FillBytesErrorReturn, IRandomSourceAdapter};

use crate::VectorCount;

pub struct NonConvertibilityVector<K: ICredentialKemAdapter> {
    pub draws: Vec<u8>,
    pub credential: K::Credential,
    pub is_valid: bool,
    pub other_is_valid: bool,
}

pub struct NonConvertibilityDeps<'a, K: ICredentialKemAdapter> {
    pub kem: &'a K,
    pub random: &'a dyn IRandomSourceAdapter,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NonConvertibilityParams {
    pub count: VectorCount,
    pub scope: IdentityScope,
}

pub struct NonConvertibilityPayload<'a> {
    pub identity: &'a [u8],
    pub other_identity: &'a [u8],
}

pub struct NonConvertibilitySuccessReturn<K: ICredentialKemAdapter> {
    pub vectors: Vec<NonConvertibilityVector<K>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum NonConvertibilityErrorReturn {
    FillBytes(FillBytesErrorReturn),
    Setup(SetupErrorReturn),
    DeriveIdentity(DeriveIdentityErrorReturn),
    DeriveOtherIdentity(DeriveIdentityErrorReturn),
    Issue(IssueErrorReturn),
}

pub type NonConvertibilityReturn<K> =
    Result<NonConvertibilitySuccessReturn<K>, NonConvertibilityErrorReturn>;

pub type NonConvertibilityFn<'a, K> = fn(
    &NonConvertibilityDeps<'a, K>,
    NonConvertibilityParams,
    NonConvertibilityPayload<'_>,
) -> NonConvertibilityReturn<K>;
