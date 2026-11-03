#[cfg(any(test, feature = "mocks"))]
use hash_to_scalar::{
    CreateHashToScalarErrorReturn, CreateHashToScalarParams, IHashToScalarAdapter,
};
#[cfg(any(test, feature = "mocks"))]
use hashing::Root;
#[cfg(any(test, feature = "mocks"))]
use kem::{CreateKemErrorReturn, CreateKemParams};
use kem::{
    DeriveIdentityErrorReturn, EncapsulateErrorReturn, EncapsulatedValue, ICredentialKemAdapter,
    IssueErrorReturn, RerandomizeErrorReturn, SetupErrorReturn,
};
#[cfg(any(test, feature = "mocks"))]
use pairing::{
    CreatePairingErrorReturn, CreatePairingParams, IPairingArithmetic, IPairingReference,
};
#[cfg(any(test, feature = "mocks"))]
use random::{CreateRandomSourceErrorReturn, CreateRandomSourceParams};
use random::{FillBytesErrorReturn, IRandomSourceAdapter};

use crate::VectorCount;
use crate::delivery::provides::AdmittedKemIdentity;

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
}

pub struct CrossHolderPayload<'a> {
    pub identity: AdmittedKemIdentity<'a>,
}

pub struct CrossHolderSuccessReturn<K: ICredentialKemAdapter> {
    pub vectors: Vec<CrossHolderVector<K>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CrossHolderErrorReturn {
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

#[cfg(any(test, feature = "mocks"))]
pub trait ICrossHolderCaseConsumer {
    type Output;

    fn consume_cross_holder_case<P, H, K>(
        &self,
        pairing: &P,
        hash_to_scalar: &H,
        kem: &K,
        random: &dyn IRandomSourceAdapter,
        independent: &dyn IRandomSourceAdapter,
        identity: AdmittedKemIdentity<'_>,
    ) -> Self::Output
    where
        P: IPairingArithmetic + IPairingReference,
        H: ?Sized + IHashToScalarAdapter<P::Scalar>,
        K: ICredentialKemAdapter<Pairing = P>;
}

#[cfg(any(test, feature = "mocks"))]
pub struct CrossHolderCaseDeps<C: ICrossHolderCaseConsumer> {
    pub consumer: C,
}

#[cfg(any(test, feature = "mocks"))]
pub struct CrossHolderCaseParams {
    pub pairing: CreatePairingParams,
    pub hash_to_scalar: CreateHashToScalarParams,
    pub kem: CreateKemParams,
    pub random_source: CreateRandomSourceParams,
}

#[cfg(any(test, feature = "mocks"))]
pub struct CrossHolderCasePayload {
    pub canonical: Vec<u8>,
    pub root: Root,
}

#[cfg(any(test, feature = "mocks"))]
pub struct CrossHolderCaseSuccessReturn<O> {
    pub output: O,
}

#[cfg(any(test, feature = "mocks"))]
#[derive(Debug, PartialEq, Eq)]
pub enum CrossHolderCaseErrorReturn {
    CreatePairing(CreatePairingErrorReturn),
    CreateHashToScalar(CreateHashToScalarErrorReturn),
    CreateKem(CreateKemErrorReturn),
    CreateRandomSource(CreateRandomSourceErrorReturn),
}

#[cfg(any(test, feature = "mocks"))]
pub type CrossHolderCaseReturn<O> =
    Result<CrossHolderCaseSuccessReturn<O>, CrossHolderCaseErrorReturn>;

#[cfg(any(test, feature = "mocks"))]
pub type RunCrossHolderCaseFn<C> =
    fn(
        &CrossHolderCaseDeps<C>,
        CrossHolderCaseParams,
        CrossHolderCasePayload,
    ) -> CrossHolderCaseReturn<<C as ICrossHolderCaseConsumer>::Output>;
