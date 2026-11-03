#[cfg(any(test, feature = "mocks"))]
use hash_to_scalar::{
    CreateHashToScalarErrorReturn, CreateHashToScalarParams, IHashToScalarAdapter,
};
#[cfg(any(test, feature = "mocks"))]
use hashing::Root;
#[cfg(any(test, feature = "mocks"))]
use kem::{CreateKemErrorReturn, CreateKemParams};
use kem::{
    DeriveIdentityErrorReturn, ICredentialKemAdapter, IssueErrorReturn, RerandomizeErrorReturn,
    SetupErrorReturn,
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
}

pub struct RerandomizationPayload<'a> {
    pub identity: AdmittedKemIdentity<'a>,
}

pub struct RerandomizationSuccessReturn<K: ICredentialKemAdapter> {
    pub vectors: Vec<RerandomizationVector<K>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RerandomizationErrorReturn {
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

#[cfg(any(test, feature = "mocks"))]
pub trait IRerandomizationCaseConsumer {
    type Output;

    fn consume_rerandomization_case<P, H, K>(
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
pub struct RerandomizationCaseDeps<C: IRerandomizationCaseConsumer> {
    pub consumer: C,
}

#[cfg(any(test, feature = "mocks"))]
pub struct RerandomizationCaseParams {
    pub pairing: CreatePairingParams,
    pub hash_to_scalar: CreateHashToScalarParams,
    pub kem: CreateKemParams,
    pub random_source: CreateRandomSourceParams,
}

#[cfg(any(test, feature = "mocks"))]
pub struct RerandomizationCasePayload {
    pub canonical: Vec<u8>,
    pub root: Root,
}

#[cfg(any(test, feature = "mocks"))]
pub struct RerandomizationCaseSuccessReturn<O> {
    pub output: O,
}

#[cfg(any(test, feature = "mocks"))]
#[derive(Debug, PartialEq, Eq)]
pub enum RerandomizationCaseErrorReturn {
    CreatePairing(CreatePairingErrorReturn),
    CreateHashToScalar(CreateHashToScalarErrorReturn),
    CreateKem(CreateKemErrorReturn),
    CreateRandomSource(CreateRandomSourceErrorReturn),
}

#[cfg(any(test, feature = "mocks"))]
pub type RerandomizationCaseReturn<O> =
    Result<RerandomizationCaseSuccessReturn<O>, RerandomizationCaseErrorReturn>;

#[cfg(any(test, feature = "mocks"))]
pub type RunRerandomizationCaseFn<C> =
    fn(
        &RerandomizationCaseDeps<C>,
        RerandomizationCaseParams,
        RerandomizationCasePayload,
    ) -> RerandomizationCaseReturn<<C as IRerandomizationCaseConsumer>::Output>;
