mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use domain::{Secret, SecretConstructorParams};
use encoding::{
    DerivationContextDescription, DerivationContextDescriptionConstructorParams, EncodeParams,
    IEncoderAdapter,
};
use interface::{
    PIECE_GROUP_KEY_LENGTH, PieceGroupKey, PieceGroupKeyErrorReturn, WrapPieceGroupKeyDeps,
    WrapPieceGroupKeyErrorReturn, WrapPieceGroupKeyParams, WrapPieceGroupKeyPayload,
    WrapPieceGroupKeyReturn, WrapPieceGroupKeySuccessReturn, WrappedPieceGroupKey,
    WrappedPieceGroupKeyErrorReturn,
};
use kdf::{DerivationPurpose, DeriveKeyParams, DeriveKeyPayload};

impl PieceGroupKey {
    pub fn try_from_secret_bytes(
        secret: Secret<Vec<u8>>,
    ) -> Result<PieceGroupKey, PieceGroupKeyErrorReturn> {
        let actual = secret.expose().len();
        if actual != PIECE_GROUP_KEY_LENGTH {
            return Err(PieceGroupKeyErrorReturn::WrongLength {
                expected: PIECE_GROUP_KEY_LENGTH,
                actual,
            });
        }
        let mut key = [0u8; PIECE_GROUP_KEY_LENGTH];
        key.copy_from_slice(secret.expose());
        let Ok(key) = Secret::try_new(SecretConstructorParams { value: key });
        Ok(PieceGroupKey { key })
    }

    pub(crate) fn from_array(key: Secret<[u8; PIECE_GROUP_KEY_LENGTH]>) -> PieceGroupKey {
        PieceGroupKey { key }
    }

    pub fn expose(&self) -> &[u8; PIECE_GROUP_KEY_LENGTH] {
        self.key.expose()
    }
}

impl WrappedPieceGroupKey {
    pub fn try_from_bytes(
        bytes: Vec<u8>,
    ) -> Result<WrappedPieceGroupKey, WrappedPieceGroupKeyErrorReturn> {
        let actual = bytes.len();
        if actual != PIECE_GROUP_KEY_LENGTH {
            return Err(WrappedPieceGroupKeyErrorReturn::WrongLength {
                expected: PIECE_GROUP_KEY_LENGTH,
                actual,
            });
        }
        let mut wrapped = [0u8; PIECE_GROUP_KEY_LENGTH];
        wrapped.copy_from_slice(&bytes);
        Ok(WrappedPieceGroupKey { bytes: wrapped })
    }

    pub(crate) fn from_array(bytes: [u8; PIECE_GROUP_KEY_LENGTH]) -> WrappedPieceGroupKey {
        WrappedPieceGroupKey { bytes }
    }

    pub fn as_bytes(&self) -> &[u8; PIECE_GROUP_KEY_LENGTH] {
        &self.bytes
    }
}

pub fn wrap_piece_group_key<E: IEncoderAdapter>(
    deps: &WrapPieceGroupKeyDeps<'_, E>,
    _params: WrapPieceGroupKeyParams,
    payload: WrapPieceGroupKeyPayload<'_>,
) -> WrapPieceGroupKeyReturn {
    let Ok(description) =
        DerivationContextDescription::try_new(DerivationContextDescriptionConstructorParams);
    let encoded = match deps.encoder.encode(
        EncodeParams {
            description: &description,
        },
        payload.context,
    ) {
        Ok(success) => success.bytes,
        Err(error) => return Err(WrapPieceGroupKeyErrorReturn::Encoding(error)),
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
        Err(error) => return Err(WrapPieceGroupKeyErrorReturn::KeyDerivation(error)),
    };
    let actual = wrapping_key.expose().len();
    if actual != PIECE_GROUP_KEY_LENGTH {
        return Err(WrapPieceGroupKeyErrorReturn::WrappingKeyLength {
            expected: PIECE_GROUP_KEY_LENGTH,
            actual,
        });
    }
    let mut bytes = [0u8; PIECE_GROUP_KEY_LENGTH];
    for (slot, (key_byte, wrap_byte)) in bytes.iter_mut().zip(
        payload
            .piece_group_key
            .expose()
            .iter()
            .zip(wrapping_key.expose().iter()),
    ) {
        *slot = key_byte ^ wrap_byte;
    }
    Ok(WrapPieceGroupKeySuccessReturn {
        wrapped: WrappedPieceGroupKey::from_array(bytes),
    })
}
