use crate::sidecar::derivation_context::provides::DerivationContext;
use crate::sidecar::wrap::provides::{
    PieceGroupKey, PieceGroupKeyErrorReturn, WrappedPieceGroupKey,
};
#[cfg(any(test, feature = "mocks"))]
use encoding::{CreateEncodingErrorReturn, CreateEncodingParams, IDecoderAdapter};
use encoding::{EncodeErrorReturn, IEncoderAdapter};
#[cfg(any(test, feature = "mocks"))]
use hash_to_scalar::{
    CreateHashToScalarErrorReturn, CreateHashToScalarParams, IHashToScalarAdapter,
};
#[cfg(any(test, feature = "mocks"))]
use kdf::{CreateKeyDerivationErrorReturn, CreateKeyDerivationParams};
use kdf::{DeriveKeyErrorReturn, IKeyDerivationAdapter};
use kem::EncapsulatedValue;
#[cfg(any(test, feature = "mocks"))]
use kem::{CreateKemErrorReturn, CreateKemParams, ICredentialKemAdapter, IdentityScope};
#[cfg(any(test, feature = "mocks"))]
use pairing::{
    CreatePairingErrorReturn, CreatePairingParams, IPairingArithmetic, IPairingReference,
};
#[cfg(any(test, feature = "mocks"))]
use random::{CreateRandomSourceErrorReturn, CreateRandomSourceParams, IRandomSourceAdapter};

pub struct UnwrapPieceGroupKeyDeps<'a, E: IEncoderAdapter> {
    pub kdf: &'a dyn IKeyDerivationAdapter,
    pub encoder: &'a E,
}

pub struct UnwrapPieceGroupKeyParams;

pub struct UnwrapPieceGroupKeyPayload<'a> {
    pub encapsulated: &'a EncapsulatedValue,
    pub context: &'a DerivationContext,
    pub wrapped: &'a WrappedPieceGroupKey,
}

pub struct UnwrapPieceGroupKeySuccessReturn {
    pub piece_group_key: PieceGroupKey,
}

#[derive(Debug, PartialEq, Eq)]
pub enum UnwrapPieceGroupKeyErrorReturn {
    Encoding(EncodeErrorReturn),
    KeyDerivation(DeriveKeyErrorReturn),
    WrappingKeyLength { expected: usize, actual: usize },
    PieceGroupKey(PieceGroupKeyErrorReturn),
}

pub type UnwrapPieceGroupKeyReturn =
    Result<UnwrapPieceGroupKeySuccessReturn, UnwrapPieceGroupKeyErrorReturn>;

pub type UnwrapPieceGroupKeyFn<E> = fn(
    &UnwrapPieceGroupKeyDeps<'_, E>,
    UnwrapPieceGroupKeyParams,
    UnwrapPieceGroupKeyPayload<'_>,
) -> UnwrapPieceGroupKeyReturn;

#[cfg(any(test, feature = "mocks"))]
pub trait IUnwrapPieceGroupKeyCaseConsumer {
    type Output;

    fn consume_unwrap_piece_group_key_case<
        E: IEncoderAdapter + IDecoderAdapter,
        P: IPairingArithmetic + IPairingReference,
        K1: ICredentialKemAdapter<Pairing = P>,
        K2: ICredentialKemAdapter<Pairing = P>,
    >(
        &self,
        params: ConsumeUnwrapPieceGroupKeyCaseParams,
        payload: ConsumeUnwrapPieceGroupKeyCasePayload<'_, E, P, K1, K2>,
    ) -> Self::Output;
}

#[cfg(any(test, feature = "mocks"))]
pub struct ConsumeUnwrapPieceGroupKeyCaseParams;

#[cfg(any(test, feature = "mocks"))]
pub struct ConsumeUnwrapPieceGroupKeyCasePayload<
    'a,
    E: IEncoderAdapter + IDecoderAdapter,
    P: IPairingArithmetic + IPairingReference,
    K1: ICredentialKemAdapter<Pairing = P>,
    K2: ICredentialKemAdapter<Pairing = P>,
> {
    pub encoder: &'a E,
    pub kdf: &'a dyn IKeyDerivationAdapter,
    pub pairing: &'a P,
    pub hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    pub kem_first: &'a K1,
    pub kem_second: &'a K2,
    pub random: &'a dyn IRandomSourceAdapter,
    pub scope_first: IdentityScope,
    pub scope_second: IdentityScope,
    pub case: &'a UnwrapPieceGroupKeyCasePayload,
}

#[cfg(any(test, feature = "mocks"))]
pub struct UnwrapPieceGroupKeyCaseDeps<C: IUnwrapPieceGroupKeyCaseConsumer> {
    pub consumer: C,
}

#[cfg(any(test, feature = "mocks"))]
pub struct UnwrapPieceGroupKeyCaseParams {
    pub encoding: CreateEncodingParams,
    pub key_derivation: CreateKeyDerivationParams,
    pub pairing: CreatePairingParams,
    pub hash_to_scalar: CreateHashToScalarParams,
    pub random_source: CreateRandomSourceParams,
    pub kem: CreateKemParams,
    pub kem_second: CreateKemParams,
}

#[cfg(any(test, feature = "mocks"))]
pub struct UnwrapPieceGroupKeyCasePayload;

#[cfg(any(test, feature = "mocks"))]
pub struct UnwrapPieceGroupKeyCaseSuccessReturn<O> {
    pub output: O,
}

#[cfg(any(test, feature = "mocks"))]
#[derive(Debug, PartialEq, Eq)]
pub enum UnwrapPieceGroupKeyCaseErrorReturn {
    Encoding(CreateEncodingErrorReturn),
    KeyDerivation(CreateKeyDerivationErrorReturn),
    Pairing(CreatePairingErrorReturn),
    HashToScalar(CreateHashToScalarErrorReturn),
    RandomSource(CreateRandomSourceErrorReturn),
    Kem(CreateKemErrorReturn),
}

#[cfg(any(test, feature = "mocks"))]
pub type UnwrapPieceGroupKeyCaseReturn<O> =
    Result<UnwrapPieceGroupKeyCaseSuccessReturn<O>, UnwrapPieceGroupKeyCaseErrorReturn>;

#[cfg(any(test, feature = "mocks"))]
pub type RunUnwrapPieceGroupKeyCaseFn<C> =
    fn(
        &UnwrapPieceGroupKeyCaseDeps<C>,
        UnwrapPieceGroupKeyCaseParams,
        UnwrapPieceGroupKeyCasePayload,
    ) -> UnwrapPieceGroupKeyCaseReturn<<C as IUnwrapPieceGroupKeyCaseConsumer>::Output>;
