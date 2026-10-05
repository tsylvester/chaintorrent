use crate::sidecar::wrap::provides::{PieceGroupKey, WrappedPieceGroupKey};
use domain::DerivationContext;
use encoding::{EncodeErrorReturn, IEncoderAdapter};
use kdf::{DeriveKeyErrorReturn, IKeyDerivationAdapter};
use kem::EncapsulatedValue;

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
}

pub type UnwrapPieceGroupKeyReturn =
    Result<UnwrapPieceGroupKeySuccessReturn, UnwrapPieceGroupKeyErrorReturn>;

pub type UnwrapPieceGroupKeyFn<E> = fn(
    &UnwrapPieceGroupKeyDeps<'_, E>,
    UnwrapPieceGroupKeyParams,
    UnwrapPieceGroupKeyPayload<'_>,
) -> UnwrapPieceGroupKeyReturn;
