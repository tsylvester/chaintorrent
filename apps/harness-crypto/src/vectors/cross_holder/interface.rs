use domain::AssetIdentityHashTryNewErrorReturn;
use kem::{
    DeriveIdentityErrorReturn, EncapsulateErrorReturn, EncapsulatedValue, ICredentialKemAdapter,
    IdentityScope, IssueErrorReturn, RerandomizeErrorReturn, SetupErrorReturn,
};
use random::{FillBytesErrorReturn, IRandomSourceAdapter};

use crate::VectorCount;

pub struct CrossHolderVector<K: ICredentialKemAdapter> {
    pub draws: Vec<u8>,
    pub parameter_set: K::ParameterSet,
    pub identity_element: K::IdentityElement,
    pub issued: K::Credential,
    pub successor: K::Credential,
    pub capsule: K::Capsule,
    pub encapsulated: EncapsulatedValue,
    pub issued_decapsulated: EncapsulatedValue,
    pub successor_decapsulated: EncapsulatedValue,
}

pub struct CrossHolderDeps<'a, K: ICredentialKemAdapter> {
    pub kem: &'a K,
    pub random: &'a dyn IRandomSourceAdapter,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CrossHolderParams {
    pub count: VectorCount,
    pub scope: IdentityScope,
}

pub struct CrossHolderPayload<'a> {
    pub identity: &'a [u8],
}

pub struct CrossHolderSuccessReturn<K: ICredentialKemAdapter> {
    pub vectors: Vec<CrossHolderVector<K>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CrossHolderErrorReturn {
    AssetIdentityLength { actual: usize },
    AssetIdentityHash(AssetIdentityHashTryNewErrorReturn),
    FillBytes(FillBytesErrorReturn),
    Setup(SetupErrorReturn),
    DeriveIdentity(DeriveIdentityErrorReturn),
    Issue(IssueErrorReturn),
    Rerandomize(RerandomizeErrorReturn),
    Encapsulate(EncapsulateErrorReturn),
}

pub type CrossHolderReturn<K> = Result<CrossHolderSuccessReturn<K>, CrossHolderErrorReturn>;

pub type CrossHolderFn<'a, K> =
    fn(&CrossHolderDeps<'a, K>, CrossHolderParams, CrossHolderPayload<'_>) -> CrossHolderReturn<K>;
