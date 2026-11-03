use encoding::IEncoderAdapter;
#[cfg(any(test, feature = "mocks"))]
use encoding::{CreateEncodingErrorReturn, CreateEncodingParams};
#[cfg(any(test, feature = "mocks"))]
use hash_to_scalar::{
    CreateHashToScalarErrorReturn, CreateHashToScalarParams, IHashToScalarAdapter,
};
use kdf::IKeyDerivationAdapter;
#[cfg(any(test, feature = "mocks"))]
use kdf::{CreateKeyDerivationErrorReturn, CreateKeyDerivationParams};
#[cfg(any(test, feature = "mocks"))]
use kem::{CreateKemErrorReturn, CreateKemParams, IdentityScope};
use kem::{
    DeriveIdentityErrorReturn, EncapsulateErrorReturn, EncapsulatedValue, ICredentialKemAdapter,
    IssueErrorReturn, SetupErrorReturn,
};
#[cfg(any(test, feature = "mocks"))]
use pairing::{
    CreatePairingErrorReturn, CreatePairingParams, IPairingArithmetic, IPairingReference,
};
#[cfg(any(test, feature = "mocks"))]
use random::{CreateRandomSourceErrorReturn, CreateRandomSourceParams};
use random::{FillBytesErrorReturn, IRandomSourceAdapter};
use workflows::{
    DerivationContext, PieceGroupKey, PieceGroupKeyErrorReturn, UnwrapPieceGroupKeyErrorReturn,
    WrapPieceGroupKeyErrorReturn, WrappedPieceGroupKey,
};

use crate::VectorCount;
use crate::delivery::provides::AdmittedKemIdentity;

pub struct CrossSetMember<K: ICredentialKemAdapter> {
    pub parameter_set: K::ParameterSet,
    pub identity_element: K::IdentityElement,
    pub credential: K::Credential,
    pub capsule: K::Capsule,
    pub encapsulated: EncapsulatedValue,
    pub wrapped: WrappedPieceGroupKey,
    pub decapsulated: EncapsulatedValue,
    pub unwrapped: PieceGroupKey,
}

pub struct CrossSetVector<K: ICredentialKemAdapter> {
    pub draws: Vec<u8>,
    pub piece_group_key: PieceGroupKey,
    pub first: CrossSetMember<K>,
    pub second: CrossSetMember<K>,
}

pub struct CrossSetDeps<'a, K: ICredentialKemAdapter, E: IEncoderAdapter> {
    pub kem: &'a K,
    pub random: &'a dyn IRandomSourceAdapter,
    pub kdf: &'a dyn IKeyDerivationAdapter,
    pub encoder: &'a E,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CrossSetParams {
    pub count: VectorCount,
}

pub struct CrossSetPayload<'a> {
    pub identity: AdmittedKemIdentity<'a>,
    pub context: &'a DerivationContext,
}

pub struct CrossSetSuccessReturn<K: ICredentialKemAdapter> {
    pub vectors: Vec<CrossSetVector<K>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CrossSetErrorReturn {
    FillBytes(FillBytesErrorReturn),
    PieceGroupKey(PieceGroupKeyErrorReturn),
    Setup(SetupErrorReturn),
    DeriveIdentity(DeriveIdentityErrorReturn),
    Issue(IssueErrorReturn),
    Encapsulate(EncapsulateErrorReturn),
    Wrap(WrapPieceGroupKeyErrorReturn),
    Unwrap(UnwrapPieceGroupKeyErrorReturn),
}

pub type CrossSetReturn<K> = Result<CrossSetSuccessReturn<K>, CrossSetErrorReturn>;

pub type CrossSetFn<'a, K, E> =
    fn(&CrossSetDeps<'a, K, E>, CrossSetParams, CrossSetPayload<'_>) -> CrossSetReturn<K>;

#[cfg(any(test, feature = "mocks"))]
pub trait ICrossSetCaseConsumer {
    type Output;

    fn consume_cross_set_case<P, H, K, E>(
        &self,
        pairing: &P,
        hash_to_scalar: &H,
        kem: &K,
        encoder: &E,
        kdf: &dyn IKeyDerivationAdapter,
        random: &dyn IRandomSourceAdapter,
        independent: &dyn IRandomSourceAdapter,
        scope: IdentityScope,
        identity: &[u8],
        context: &DerivationContext,
    ) -> Self::Output
    where
        P: IPairingArithmetic + IPairingReference,
        H: ?Sized + IHashToScalarAdapter<P::Scalar>,
        K: ICredentialKemAdapter<Pairing = P>,
        E: IEncoderAdapter;
}

#[cfg(any(test, feature = "mocks"))]
pub struct CrossSetCaseDeps<C: ICrossSetCaseConsumer> {
    pub consumer: C,
}

#[cfg(any(test, feature = "mocks"))]
pub struct CrossSetCaseParams {
    pub pairing: CreatePairingParams,
    pub hash_to_scalar: CreateHashToScalarParams,
    pub encoding: CreateEncodingParams,
    pub kem: CreateKemParams,
    pub key_derivation: CreateKeyDerivationParams,
    pub random_source: CreateRandomSourceParams,
}

#[cfg(any(test, feature = "mocks"))]
pub struct CrossSetCasePayload {
    pub identity: Vec<u8>,
    pub context: DerivationContext,
}

#[cfg(any(test, feature = "mocks"))]
pub struct CrossSetCaseSuccessReturn<O> {
    pub output: O,
}

#[cfg(any(test, feature = "mocks"))]
#[derive(Debug, PartialEq, Eq)]
pub enum CrossSetCaseErrorReturn {
    CreatePairing(CreatePairingErrorReturn),
    CreateHashToScalar(CreateHashToScalarErrorReturn),
    CreateEncoding(CreateEncodingErrorReturn),
    CreateKem(CreateKemErrorReturn),
    CreateKeyDerivation(CreateKeyDerivationErrorReturn),
    CreateRandomSource(CreateRandomSourceErrorReturn),
}

#[cfg(any(test, feature = "mocks"))]
pub type CrossSetCaseReturn<O> = Result<CrossSetCaseSuccessReturn<O>, CrossSetCaseErrorReturn>;

#[cfg(any(test, feature = "mocks"))]
pub type RunCrossSetCaseFn<C> = fn(
    &CrossSetCaseDeps<C>,
    CrossSetCaseParams,
    CrossSetCasePayload,
) -> CrossSetCaseReturn<<C as ICrossSetCaseConsumer>::Output>;
