mod interface;
pub(crate) mod provides;
#[cfg(test)]
mod test;

use crate::mint_statement::provides::{
    MintStatementDescription, MintStatementDescriptionConstructorParams,
    MintStatementFromFieldsErrorReturn,
};
use crate::transfer_statement::provides::{
    TransferStatementDescription, TransferStatementDescriptionConstructorParams,
    TransferStatementFromFieldsErrorReturn,
};
use chain::IChainForms;
use encoding::{EncodeParams, IEncoderAdapter};
use hash_to_scalar::{HashToScalarParams, HashToScalarPayload};
use interface::{
    ChallengeDeps, ChallengeErrorReturn, ChallengeParams, ChallengePayload, ChallengeReturn,
    ChallengeSuccessReturn, ChallengeTranscript,
};
use pairing::{IPairingAdapter, ISampleUniformScalar};

pub fn challenge<
    S: ISampleUniformScalar + Clone,
    E: IEncoderAdapter,
    P: IPairingAdapter<Scalar = S>,
    F: IChainForms,
>(
    deps: &ChallengeDeps<'_, P, E>,
    _params: ChallengeParams,
    payload: ChallengePayload<'_, P, F>,
) -> ChallengeReturn<S>
where
    MintStatementFromFieldsErrorReturn<F>: core::fmt::Debug + PartialEq + Eq,
    TransferStatementFromFieldsErrorReturn<F>: core::fmt::Debug + PartialEq + Eq,
{
    let bytes = match payload.transcript {
        ChallengeTranscript::Mint(statement) => {
            let Ok(description) = MintStatementDescription::<P, F>::try_new(
                MintStatementDescriptionConstructorParams {
                    pairing: deps.pairing,
                },
            );
            match deps.encoder.encode(
                EncodeParams {
                    description: &description,
                },
                statement,
            ) {
                Ok(success) => success.bytes,
                Err(error) => return Err(ChallengeErrorReturn::Encoding(error)),
            }
        }
        ChallengeTranscript::Transfer(statement) => {
            let Ok(description) = TransferStatementDescription::<P, F>::try_new(
                TransferStatementDescriptionConstructorParams {
                    pairing: deps.pairing,
                },
            );
            match deps.encoder.encode(
                EncodeParams {
                    description: &description,
                },
                statement,
            ) {
                Ok(success) => success.bytes,
                Err(error) => return Err(ChallengeErrorReturn::Encoding(error)),
            }
        }
    };
    match deps.hash_to_scalar.hash_to_scalar(
        HashToScalarParams { tag: deps.tag },
        HashToScalarPayload { message: &bytes },
    ) {
        Ok(success) => Ok(ChallengeSuccessReturn {
            challenge: success.scalar,
        }),
        Err(error) => Err(ChallengeErrorReturn::HashToScalar(error)),
    }
}
