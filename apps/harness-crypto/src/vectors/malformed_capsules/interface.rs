#[cfg(any(test, feature = "mocks"))]
use hash_to_scalar::{
    CreateHashToScalarErrorReturn, CreateHashToScalarParams, IHashToScalarAdapter,
};
#[cfg(any(test, feature = "mocks"))]
use hashing::Root;
#[cfg(any(test, feature = "mocks"))]
use kem::{CreateKemErrorReturn, CreateKemParams};
use kem::{EncapsulateErrorReturn, ICredentialKemAdapter, SetupErrorReturn};
#[cfg(any(test, feature = "mocks"))]
use pairing::{
    CreatePairingErrorReturn, CreatePairingParams, IPairingArithmetic, IPairingReference,
};
#[cfg(any(test, feature = "mocks"))]
use random::{CreateRandomSourceErrorReturn, CreateRandomSourceParams};
use random::{FillBytesErrorReturn, IRandomSourceAdapter};

use crate::VectorCount;
use crate::delivery::provides::AdmittedKemIdentity;

pub struct MalformedCapsulesVector<K: ICredentialKemAdapter> {
    pub draws: Vec<u8>,
    pub parameter_set: K::ParameterSet,
    pub capsule: K::Capsule,
    pub displaced: K::Capsule,
    pub is_well_formed: bool,
    pub displaced_is_well_formed: bool,
}

pub struct MalformedCapsulesDeps<'a, K: ICredentialKemAdapter> {
    pub kem: &'a K,
    pub random: &'a dyn IRandomSourceAdapter,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MalformedCapsulesParams {
    pub count: VectorCount,
}

pub struct MalformedCapsulesPayload<'a> {
    pub identity: AdmittedKemIdentity<'a>,
}

pub struct MalformedCapsulesSuccessReturn<K: ICredentialKemAdapter> {
    pub vectors: Vec<MalformedCapsulesVector<K>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MalformedCapsulesErrorReturn {
    FillBytes(FillBytesErrorReturn),
    Setup(SetupErrorReturn),
    Encapsulate(EncapsulateErrorReturn),
}

pub type MalformedCapsulesReturn<K> =
    Result<MalformedCapsulesSuccessReturn<K>, MalformedCapsulesErrorReturn>;

pub type MalformedCapsulesFn<'a, K> = fn(
    &MalformedCapsulesDeps<'a, K>,
    MalformedCapsulesParams,
    MalformedCapsulesPayload<'_>,
) -> MalformedCapsulesReturn<K>;

#[cfg(any(test, feature = "mocks"))]
pub trait IMalformedCapsulesCaseConsumer {
    type Output;

    fn consume_malformed_capsules_case<P, H, K>(
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
pub struct MalformedCapsulesCaseDeps<C: IMalformedCapsulesCaseConsumer> {
    pub consumer: C,
}

#[cfg(any(test, feature = "mocks"))]
pub struct MalformedCapsulesCaseParams {
    pub pairing: CreatePairingParams,
    pub hash_to_scalar: CreateHashToScalarParams,
    pub kem: CreateKemParams,
    pub random_source: CreateRandomSourceParams,
}

#[cfg(any(test, feature = "mocks"))]
pub struct MalformedCapsulesCasePayload {
    pub canonical: Vec<u8>,
    pub root: Root,
}

#[cfg(any(test, feature = "mocks"))]
pub struct MalformedCapsulesCaseSuccessReturn<O> {
    pub output: O,
}

#[cfg(any(test, feature = "mocks"))]
#[derive(Debug, PartialEq, Eq)]
pub enum MalformedCapsulesCaseErrorReturn {
    CreatePairing(CreatePairingErrorReturn),
    CreateHashToScalar(CreateHashToScalarErrorReturn),
    CreateKem(CreateKemErrorReturn),
    CreateRandomSource(CreateRandomSourceErrorReturn),
}

#[cfg(any(test, feature = "mocks"))]
pub type MalformedCapsulesCaseReturn<O> =
    Result<MalformedCapsulesCaseSuccessReturn<O>, MalformedCapsulesCaseErrorReturn>;

#[cfg(any(test, feature = "mocks"))]
pub type RunMalformedCapsulesCaseFn<C> =
    fn(
        &MalformedCapsulesCaseDeps<C>,
        MalformedCapsulesCaseParams,
        MalformedCapsulesCasePayload,
    ) -> MalformedCapsulesCaseReturn<<C as IMalformedCapsulesCaseConsumer>::Output>;
