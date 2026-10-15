#![allow(clippy::expect_used)]

use super::interface::{
    ChallengeDeps, ChallengeParams, ChallengePayload, ChallengeReturn, ChallengeSuccessReturn,
    ChallengeTranscript,
};
use chain::IChainForms;
use domain::{SecretConstructorParamsOverrides, build_secret};
use encoding::IEncoderAdapter;
use hash_to_scalar::{DomainTag, IHashToScalarAdapter};
use pairing::{
    IPairingAdapter, ISampleUniformScalar, SampleUniformScalarParams, SampleUniformScalarPayload,
};

#[derive(Default)]
pub struct ChallengeDepsOverrides;

pub fn build_challenge_deps<'a, P: IPairingAdapter, E: IEncoderAdapter>(
    pairing: &'a P,
    encoder: &'a E,
    tag: &'a DomainTag,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    _overrides: ChallengeDepsOverrides,
) -> ChallengeDeps<'a, P, E> {
    ChallengeDeps {
        pairing,
        encoder,
        tag,
        hash_to_scalar,
    }
}

#[derive(Default)]
pub struct ChallengeParamsOverrides;

pub fn build_challenge_params(_overrides: ChallengeParamsOverrides) -> ChallengeParams {
    ChallengeParams
}

#[derive(Default)]
pub struct ChallengePayloadOverrides;

pub fn build_challenge_payload<'a, P: IPairingAdapter, F: IChainForms>(
    transcript: ChallengeTranscript<'a, P, F>,
    _overrides: ChallengePayloadOverrides,
) -> ChallengePayload<'a, P, F> {
    ChallengePayload { transcript }
}

#[derive(Default)]
pub struct ChallengeSuccessReturnOverrides;

pub fn build_challenge_success_return<S>(
    challenge: S,
    _overrides: ChallengeSuccessReturnOverrides,
) -> ChallengeSuccessReturn<S> {
    ChallengeSuccessReturn { challenge }
}

pub fn mock_challenge<S, E, P, F>(
    _deps: &ChallengeDeps<'_, P, E>,
    _params: ChallengeParams,
    _payload: ChallengePayload<'_, P, F>,
) -> ChallengeReturn<S>
where
    S: ISampleUniformScalar + Clone,
    E: IEncoderAdapter,
    P: IPairingAdapter<Scalar = S>,
    F: IChainForms,
{
    let scalar = S::sample_from_uniform_bytes(
        SampleUniformScalarParams,
        SampleUniformScalarPayload {
            uniform: build_secret(vec![1u8; S::UNIFORM_BYTES_LENGTH], Default::default()),
        },
    )
    .expect("a uniform input of UNIFORM_BYTES_LENGTH samples a scalar")
    .scalar
    .expose()
    .clone();
    Ok(build_challenge_success_return(scalar, Default::default()))
}
