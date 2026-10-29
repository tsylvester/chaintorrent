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
        encoder: &E,
        kdf: &dyn IKeyDerivationAdapter,
        pairing: &P,
        hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
        kem_first: &K1,
        kem_second: &K2,
        random: &dyn IRandomSourceAdapter,
        scope_first: IdentityScope,
        scope_second: IdentityScope,
        payload: &UnwrapPieceGroupKeyCasePayload,
    ) -> Self::Output;
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
    CreateEncoding(CreateEncodingErrorReturn),
    CreateKeyDerivation(CreateKeyDerivationErrorReturn),
    CreatePairing(CreatePairingErrorReturn),
    CreateHashToScalar(CreateHashToScalarErrorReturn),
    CreateRandomSource(CreateRandomSourceErrorReturn),
    CreateKem(CreateKemErrorReturn),
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
