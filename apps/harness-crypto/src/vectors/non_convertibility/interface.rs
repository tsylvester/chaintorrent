#[cfg(any(test, feature = "mocks"))]
use hash_to_scalar::{
    CreateHashToScalarErrorReturn, CreateHashToScalarParams, IHashToScalarAdapter,
};
#[cfg(any(test, feature = "mocks"))]
use hashing::Root;
#[cfg(any(test, feature = "mocks"))]
use kem::{CreateKemErrorReturn, CreateKemParams};
use kem::{DeriveIdentityErrorReturn, ICredentialKemAdapter, IssueErrorReturn, SetupErrorReturn};
#[cfg(any(test, feature = "mocks"))]
use pairing::{
    CreatePairingErrorReturn, CreatePairingParams, IPairingArithmetic, IPairingReference,
};
#[cfg(any(test, feature = "mocks"))]
use random::{CreateRandomSourceErrorReturn, CreateRandomSourceParams};
use random::{FillBytesErrorReturn, IRandomSourceAdapter};

use crate::VectorCount;
use crate::delivery::provides::AdmittedKemIdentity;

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
}

pub struct NonConvertibilityPayload<'a> {
    pub identity: AdmittedKemIdentity<'a>,
    pub other_identity: AdmittedKemIdentity<'a>,
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

#[cfg(any(test, feature = "mocks"))]
pub trait INonConvertibilityCaseConsumer {
    type Output;

    fn consume_non_convertibility_case<P, H, K>(
        &self,
        pairing: &P,
        hash_to_scalar: &H,
        kem: &K,
        random: &dyn IRandomSourceAdapter,
        independent: &dyn IRandomSourceAdapter,
        identity: AdmittedKemIdentity<'_>,
        other_identity: AdmittedKemIdentity<'_>,
    ) -> Self::Output
    where
        P: IPairingArithmetic + IPairingReference,
        H: ?Sized + IHashToScalarAdapter<P::Scalar>,
        K: ICredentialKemAdapter<Pairing = P>;
}

#[cfg(any(test, feature = "mocks"))]
pub struct NonConvertibilityCaseDeps<C: INonConvertibilityCaseConsumer> {
    pub consumer: C,
}

#[cfg(any(test, feature = "mocks"))]
pub struct NonConvertibilityCaseParams {
    pub pairing: CreatePairingParams,
    pub hash_to_scalar: CreateHashToScalarParams,
    pub kem: CreateKemParams,
    pub random_source: CreateRandomSourceParams,
}

#[cfg(any(test, feature = "mocks"))]
pub struct NonConvertibilityCasePayload {
    pub canonical: Vec<u8>,
    pub other_canonical: Vec<u8>,
    pub root: Root,
    pub other_root: Root,
}

#[cfg(any(test, feature = "mocks"))]
pub struct NonConvertibilityCaseSuccessReturn<O> {
    pub output: O,
}

#[cfg(any(test, feature = "mocks"))]
#[derive(Debug, PartialEq, Eq)]
pub enum NonConvertibilityCaseErrorReturn {
    CreatePairing(CreatePairingErrorReturn),
    CreateHashToScalar(CreateHashToScalarErrorReturn),
    CreateKem(CreateKemErrorReturn),
    CreateRandomSource(CreateRandomSourceErrorReturn),
}

#[cfg(any(test, feature = "mocks"))]
pub type NonConvertibilityCaseReturn<O> =
    Result<NonConvertibilityCaseSuccessReturn<O>, NonConvertibilityCaseErrorReturn>;

#[cfg(any(test, feature = "mocks"))]
pub type RunNonConvertibilityCaseFn<C> =
    fn(
        &NonConvertibilityCaseDeps<C>,
        NonConvertibilityCaseParams,
        NonConvertibilityCasePayload,
    ) -> NonConvertibilityCaseReturn<<C as INonConvertibilityCaseConsumer>::Output>;
