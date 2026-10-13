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
    PIECE_GROUP_KEY_LENGTH, PieceGroupKey, PieceGroupKeyConstructorParams,
    PieceGroupKeyErrorReturn, WrapPieceGroupKeyDeps, WrapPieceGroupKeyErrorReturn,
    WrapPieceGroupKeyParams, WrapPieceGroupKeyPayload, WrapPieceGroupKeyReturn,
    WrapPieceGroupKeySuccessReturn, WrappedPieceGroupKey, WrappedPieceGroupKeyConstructorParams,
    WrappedPieceGroupKeyErrorReturn,
};
use kdf::{DerivationPurpose, DeriveKeyParams, DeriveKeyPayload};

impl PieceGroupKey {
    pub fn try_new(
        params: PieceGroupKeyConstructorParams,
    ) -> Result<PieceGroupKey, PieceGroupKeyErrorReturn> {
        let actual = params.secret.expose().len();
        if actual != PIECE_GROUP_KEY_LENGTH {
            return Err(PieceGroupKeyErrorReturn::WrongLength {
                expected: PIECE_GROUP_KEY_LENGTH,
                actual,
            });
        }
        let mut key = [0u8; PIECE_GROUP_KEY_LENGTH];
        key.copy_from_slice(params.secret.expose());
        let Ok(key) = Secret::try_new(SecretConstructorParams { value: key });
        Ok(PieceGroupKey { key })
    }

    pub fn expose(&self) -> &[u8; PIECE_GROUP_KEY_LENGTH] {
        self.key.expose()
    }
}

impl WrappedPieceGroupKey {
    pub fn try_new(
        params: WrappedPieceGroupKeyConstructorParams,
    ) -> Result<WrappedPieceGroupKey, WrappedPieceGroupKeyErrorReturn> {
        let actual = params.bytes.len();
        if actual != PIECE_GROUP_KEY_LENGTH {
            return Err(WrappedPieceGroupKeyErrorReturn::WrongLength {
                expected: PIECE_GROUP_KEY_LENGTH,
                actual,
            });
        }
        let mut bytes = [0u8; PIECE_GROUP_KEY_LENGTH];
        bytes.copy_from_slice(&params.bytes);
        Ok(WrappedPieceGroupKey { bytes })
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
    let bytes = payload
        .piece_group_key
        .expose()
        .iter()
        .zip(wrapping_key.expose().iter())
        .map(|(key_byte, wrap_byte)| key_byte ^ wrap_byte)
        .collect();
    match WrappedPieceGroupKey::try_new(WrappedPieceGroupKeyConstructorParams { bytes }) {
        Ok(wrapped) => Ok(WrapPieceGroupKeySuccessReturn { wrapped }),
        Err(error) => Err(WrapPieceGroupKeyErrorReturn::WrappedKey(error)),
    }
}
