#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test_basic;
#[cfg(test)]
mod test_error;

use domain::Secret;
use encoding::IEncoderAdapter;
use kem::{
    DecapsulateParams, DecapsulatePayload, DeriveIdentityParams, DeriveIdentityPayload,
    EncapsulateParams, EncapsulatePayload, ICredentialKemAdapter, IssueParams, IssuePayload,
    SetupParams, SetupPayload,
};
use pairing::{IPairingAdapter, ISampleUniformScalar};
use random::{FillBytesParams, FillBytesPayload};
use workflows::{
    PIECE_GROUP_KEY_LENGTH, PieceGroupKey, PieceGroupKeyConstructorParams, UnwrapPieceGroupKeyDeps,
    UnwrapPieceGroupKeyParams, UnwrapPieceGroupKeyPayload, WrapPieceGroupKeyDeps,
    WrapPieceGroupKeyParams, WrapPieceGroupKeyPayload, unwrap_piece_group_key,
    wrap_piece_group_key,
};

use interface::{
    CrossSetDeps, CrossSetErrorReturn, CrossSetMember, CrossSetParams, CrossSetPayload,
    CrossSetReturn, CrossSetSuccessReturn, CrossSetVector,
};

pub fn cross_set<'a, K: ICredentialKemAdapter, E: IEncoderAdapter>(
    deps: &CrossSetDeps<'a, K, E>,
    params: CrossSetParams,
    payload: CrossSetPayload<'_>,
) -> CrossSetReturn<K> {
    let draw_uniform = |draws: &mut Vec<u8>| -> Result<Secret<Vec<u8>>, CrossSetErrorReturn> {
        let success = deps
            .random
            .fill_bytes(
                FillBytesParams,
                FillBytesPayload {
                    length: <K::Pairing as IPairingAdapter>::Scalar::UNIFORM_BYTES_LENGTH,
                },
            )
            .map_err(CrossSetErrorReturn::FillBytes)?;
        draws.extend_from_slice(success.bytes.expose());
        Ok(success.bytes)
    };
    let member = |draws: &mut Vec<u8>,
                  piece_group_key: &PieceGroupKey|
     -> Result<CrossSetMember<K>, CrossSetErrorReturn> {
        let master_uniform = draw_uniform(draws)?;
        let u0_uniform = draw_uniform(draws)?;
        let u1_uniform = draw_uniform(draws)?;
        let setup = deps
            .kem
            .setup(
                SetupParams {
                    scope: payload.identity.setup_scope,
                },
                SetupPayload {
                    master_uniform,
                    u0_uniform,
                    u1_uniform,
                },
            )
            .map_err(CrossSetErrorReturn::Setup)?;
        let derived = deps
            .kem
            .derive_identity(
                DeriveIdentityParams,
                DeriveIdentityPayload {
                    parameter_set: &setup.parameter_set,
                    identity: payload.identity.identity,
                },
            )
            .map_err(CrossSetErrorReturn::DeriveIdentity)?;
        let issued = deps
            .kem
            .issue(
                IssueParams,
                IssuePayload {
                    parameter_set: &setup.parameter_set,
                    master_scalar: &setup.master_scalar,
                    identity_element: &derived.identity_element,
                    uniform: draw_uniform(draws)?,
                },
            )
            .map_err(CrossSetErrorReturn::Issue)?;
        let encapsulated = deps
            .kem
            .encapsulate(
                EncapsulateParams,
                EncapsulatePayload {
                    parameter_set: &setup.parameter_set,
                    uniform: draw_uniform(draws)?,
                },
            )
            .map_err(CrossSetErrorReturn::Encapsulate)?;
        let wrapped = wrap_piece_group_key(
            &WrapPieceGroupKeyDeps {
                kdf: deps.kdf,
                encoder: deps.encoder,
            },
            WrapPieceGroupKeyParams,
            WrapPieceGroupKeyPayload {
                encapsulated: &encapsulated.encapsulated,
                context: payload.context,
                piece_group_key,
            },
        )
        .map_err(CrossSetErrorReturn::Wrap)?;
        let Ok(decapsulated) = deps.kem.decapsulate(
            DecapsulateParams,
            DecapsulatePayload {
                identity_element: &derived.identity_element,
                credential: &issued.credential,
                capsule: &encapsulated.capsule,
            },
        );
        let unwrapped = unwrap_piece_group_key(
            &UnwrapPieceGroupKeyDeps {
                kdf: deps.kdf,
                encoder: deps.encoder,
            },
            UnwrapPieceGroupKeyParams,
            UnwrapPieceGroupKeyPayload {
                encapsulated: &decapsulated.encapsulated,
                context: payload.context,
                wrapped: &wrapped.wrapped,
            },
        )
        .map_err(CrossSetErrorReturn::Unwrap)?;
        Ok(CrossSetMember {
            parameter_set: setup.parameter_set,
            identity_element: derived.identity_element,
            credential: issued.credential,
            capsule: encapsulated.capsule,
            encapsulated: encapsulated.encapsulated,
            wrapped: wrapped.wrapped,
            decapsulated: decapsulated.encapsulated,
            unwrapped: unwrapped.piece_group_key,
        })
    };
    let mut vectors = Vec::new();
    for _ in 0..params.count.get() {
        let mut draws = Vec::new();
        let key_secret = deps
            .random
            .fill_bytes(
                FillBytesParams,
                FillBytesPayload {
                    length: PIECE_GROUP_KEY_LENGTH,
                },
            )
            .map_err(CrossSetErrorReturn::FillBytes)?
            .bytes;
        draws.extend_from_slice(key_secret.expose());
        let piece_group_key =
            PieceGroupKey::try_new(PieceGroupKeyConstructorParams { secret: key_secret })
                .map_err(CrossSetErrorReturn::PieceGroupKey)?;
        let first = member(&mut draws, &piece_group_key)?;
        let second = member(&mut draws, &piece_group_key)?;
        vectors.push(CrossSetVector {
            draws,
            piece_group_key,
            first,
            second,
        });
    }
    Ok(CrossSetSuccessReturn { vectors })
}
