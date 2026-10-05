mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::sidecar::wrap::provides::{PIECE_GROUP_KEY_LENGTH, PieceGroupKey};
use domain::{Secret, SecretConstructorParams};
use encoding::{
    DerivationContextDescription, DerivationContextDescriptionConstructorParams, EncodeParams,
    IEncoderAdapter,
};
use interface::{
    UnwrapPieceGroupKeyDeps, UnwrapPieceGroupKeyErrorReturn, UnwrapPieceGroupKeyParams,
    UnwrapPieceGroupKeyPayload, UnwrapPieceGroupKeyReturn, UnwrapPieceGroupKeySuccessReturn,
};
use kdf::{DerivationPurpose, DeriveKeyParams, DeriveKeyPayload};

pub fn unwrap_piece_group_key<E: IEncoderAdapter>(
    deps: &UnwrapPieceGroupKeyDeps<'_, E>,
    _params: UnwrapPieceGroupKeyParams,
    payload: UnwrapPieceGroupKeyPayload<'_>,
) -> UnwrapPieceGroupKeyReturn {
    let Ok(description) =
        DerivationContextDescription::try_new(DerivationContextDescriptionConstructorParams);
    let encoded = match deps.encoder.encode(
        EncodeParams {
            description: &description,
        },
        payload.context,
    ) {
        Ok(success) => success.bytes,
        Err(error) => return Err(UnwrapPieceGroupKeyErrorReturn::Encoding(error)),
    };
    let wrapping_key = match deps.kdf.derive_key(
        DeriveKeyParams {
            purpose: DerivationPurpose::WrappingKey,
            length: PIECE_GROUP_KEY_LENGTH,
        },
        DeriveKeyPayload {
            key_material: payload.encapsulated.key_material(),
            context: &encoded,
        },
    ) {
        Ok(success) => success.key,
        Err(error) => return Err(UnwrapPieceGroupKeyErrorReturn::KeyDerivation(error)),
    };
    let actual = wrapping_key.expose().len();
    if actual != PIECE_GROUP_KEY_LENGTH {
        return Err(UnwrapPieceGroupKeyErrorReturn::WrappingKeyLength {
            expected: PIECE_GROUP_KEY_LENGTH,
            actual,
        });
    }
    let mut bytes = [0u8; PIECE_GROUP_KEY_LENGTH];
    for (slot, (wrapped_byte, wrap_byte)) in bytes.iter_mut().zip(
        payload
            .wrapped
            .as_bytes()
            .iter()
            .zip(wrapping_key.expose().iter()),
    ) {
        *slot = wrapped_byte ^ wrap_byte;
    }
    let Ok(key) = Secret::try_new(SecretConstructorParams { value: bytes });
    Ok(UnwrapPieceGroupKeySuccessReturn {
        piece_group_key: PieceGroupKey::from_array(key),
    })
}
