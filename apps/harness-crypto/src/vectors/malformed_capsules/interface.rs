use domain::AssetIdentityHashTryNewErrorReturn;
use kem::{EncapsulateErrorReturn, ICredentialKemAdapter, IdentityScope, SetupErrorReturn};
use random::{FillBytesErrorReturn, IRandomSourceAdapter};

use crate::VectorCount;

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
    pub scope: IdentityScope,
}

pub struct MalformedCapsulesPayload<'a> {
    pub identity: &'a [u8],
}

pub struct MalformedCapsulesSuccessReturn<K: ICredentialKemAdapter> {
    pub vectors: Vec<MalformedCapsulesVector<K>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MalformedCapsulesErrorReturn {
    AssetIdentityLength { actual: usize },
    AssetIdentityHash(AssetIdentityHashTryNewErrorReturn),
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
