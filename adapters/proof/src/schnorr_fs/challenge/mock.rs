#![allow(dead_code, clippy::expect_used, clippy::too_many_arguments)]

use super::interface::{
    ChallengeCaseDeps, ChallengeCaseErrorReturn, ChallengeCaseParams, ChallengeCasePayload,
    ChallengeCaseReturn, ChallengeCaseSuccessReturn, ChallengeDeps, ChallengeFn, ChallengeParams,
    ChallengePayload, ChallengeReturn, ChallengeSuccessReturn, ChallengeTranscript,
    IChallengeCaseConsumer,
};
use chain::{
    ConsumeChainFormsParams, ConsumeChainFormsPayload, CreateChainFormsParams, IChainForms,
    IChainFormsConsumer, build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, create_chain_forms,
};
use domain::build_secret;
use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingParams,
    CreateEncodingParamsOverrides, IDecoderAdapter, IEncoderAdapter, IEncodingConsumer,
    build_create_encoding_deps, build_create_encoding_params, build_create_encoding_payload,
    create_encoding,
};
use hash_to_scalar::{
    CreateHashToScalarParams, DomainTag, IHashToScalarAdapter, build_create_hash_to_scalar_deps,
    build_create_hash_to_scalar_params, build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParams, IPairingAdapter,
    IPairingArithmetic, IPairingConsumer, IPairingReference, ISampleUniformScalar,
    SampleUniformScalarParams, SampleUniformScalarPayload, build_create_pairing_deps,
    build_create_pairing_params, build_create_pairing_payload, create_pairing,
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
    let _: ChallengeFn<S, E, P, F> = mock_challenge::<S, E, P, F>;
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

#[derive(Default)]
pub struct ChallengeCaseDepsOverrides;

pub fn build_challenge_case_deps<C: IChallengeCaseConsumer>(
    consumer: C,
    _overrides: ChallengeCaseDepsOverrides,
) -> ChallengeCaseDeps<C> {
    ChallengeCaseDeps { consumer }
}

#[derive(Default)]
pub struct ChallengeCaseParamsOverrides {
    pub pairing: Option<CreatePairingParams>,
    pub chain_forms: Option<CreateChainFormsParams>,
    pub encoding: Option<CreateEncodingParams>,
    pub hash_to_scalar: Option<CreateHashToScalarParams>,
}

pub fn build_challenge_case_params(overrides: ChallengeCaseParamsOverrides) -> ChallengeCaseParams {
    ChallengeCaseParams {
        pairing: overrides
            .pairing
            .unwrap_or_else(|| build_create_pairing_params(Default::default())),
        chain_forms: overrides
            .chain_forms
            .unwrap_or_else(|| build_create_chain_forms_params(Default::default())),
        encoding: overrides
            .encoding
            .unwrap_or_else(|| build_create_encoding_params(Default::default())),
        hash_to_scalar: overrides
            .hash_to_scalar
            .unwrap_or_else(|| build_create_hash_to_scalar_params(Default::default())),
    }
}

#[derive(Default)]
pub struct ChallengeCasePayloadOverrides;

pub fn build_challenge_case_payload(
    _overrides: ChallengeCasePayloadOverrides,
) -> ChallengeCasePayload {
    ChallengeCasePayload
}

#[derive(Default)]
pub struct ChallengeCaseSuccessReturnOverrides;

pub fn build_challenge_case_success_return<O>(
    output: O,
    _overrides: ChallengeCaseSuccessReturnOverrides,
) -> ChallengeCaseSuccessReturn<O> {
    ChallengeCaseSuccessReturn { output }
}

struct CaseAtPairing<'a, C: IChallengeCaseConsumer> {
    pub consumer: &'a C,
    pub chain_forms: CreateChainFormsParams,
    pub encoding: CreateEncodingParams,
    pub hash_to_scalar: CreateHashToScalarParams,
}

impl<C: IChallengeCaseConsumer> IPairingConsumer for CaseAtPairing<'_, C> {
    type Output = ChallengeCaseReturn<C::Output>;

    fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let encoding_deps = build_create_encoding_deps(CaseAtEncoding {
            consumer: self.consumer,
            pairing: &payload.adapter,
            chain_forms: self.chain_forms,
            hash_to_scalar: self.hash_to_scalar,
        });
        match create_encoding(
            &encoding_deps,
            build_create_encoding_params(CreateEncodingParamsOverrides {
                concrete: Some(self.encoding.concrete),
                identifier: Some(self.encoding.identifier),
            }),
            build_create_encoding_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(ChallengeCaseErrorReturn::CreateEncoding(error)),
        }
    }
}

struct CaseAtEncoding<'a, 'p, C: IChallengeCaseConsumer, P: IPairingArithmetic + IPairingReference>
{
    pub consumer: &'a C,
    pub pairing: &'p P,
    pub chain_forms: CreateChainFormsParams,
    pub hash_to_scalar: CreateHashToScalarParams,
}

impl<C: IChallengeCaseConsumer, P: IPairingArithmetic + IPairingReference> IEncodingConsumer
    for CaseAtEncoding<'_, '_, C, P>
{
    type Output = ChallengeCaseReturn<C::Output>;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
        let chain_forms_deps = build_create_chain_forms_deps(
            CaseAtChainForms {
                consumer: self.consumer,
                pairing: self.pairing,
                encoder: &payload.adapter,
                hash_to_scalar: self.hash_to_scalar,
            },
            &payload.adapter,
            Default::default(),
        );
        match create_chain_forms(
            &chain_forms_deps,
            self.chain_forms,
            build_create_chain_forms_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(ChallengeCaseErrorReturn::CreateChainForms(error)),
        }
    }
}

struct CaseAtChainForms<
    'a,
    'p,
    C: IChallengeCaseConsumer,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
> {
    pub consumer: &'a C,
    pub pairing: &'p P,
    pub encoder: &'a E,
    pub hash_to_scalar: CreateHashToScalarParams,
}

impl<
    C: IChallengeCaseConsumer,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
> IChainFormsConsumer for CaseAtChainForms<'_, '_, C, P, E>
{
    type Output = ChallengeCaseReturn<C::Output>;

    fn consume_chain_forms<F: IChainForms>(
        &self,
        _params: ConsumeChainFormsParams,
        payload: ConsumeChainFormsPayload<F>,
    ) -> Self::Output {
        match create_hash_to_scalar::<P::Scalar>(
            &build_create_hash_to_scalar_deps(Default::default()),
            self.hash_to_scalar,
            build_create_hash_to_scalar_payload(Default::default()),
        ) {
            Ok(success) => Ok(ChallengeCaseSuccessReturn {
                output: self.consumer.consume_challenge_case(
                    self.pairing,
                    &payload.forms,
                    self.encoder,
                    &*success.adapter,
                    &ChallengeCasePayload,
                ),
            }),
            Err(error) => Err(ChallengeCaseErrorReturn::CreateHashToScalar(error)),
        }
    }
}

pub fn run_challenge_case<C: IChallengeCaseConsumer>(
    deps: &ChallengeCaseDeps<C>,
    params: ChallengeCaseParams,
    _payload: ChallengeCasePayload,
) -> ChallengeCaseReturn<C::Output> {
    let pairing_deps = build_create_pairing_deps(
        CaseAtPairing {
            consumer: &deps.consumer,
            chain_forms: params.chain_forms,
            encoding: params.encoding,
            hash_to_scalar: params.hash_to_scalar,
        },
        Default::default(),
    );
    match create_pairing(
        &pairing_deps,
        params.pairing,
        build_create_pairing_payload(Default::default()),
    ) {
        Ok(success) => success.output,
        Err(error) => Err(ChallengeCaseErrorReturn::CreatePairing(error)),
    }
}
