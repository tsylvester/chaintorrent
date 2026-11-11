#![allow(dead_code, clippy::expect_used, clippy::too_many_arguments)]

use chain::IChainForms;
#[cfg(test)]
use chain::{
    ConsumeChainFormsParams, ConsumeChainFormsPayload, CreateChainFormsParams, IChainFormsConsumer,
    build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, create_chain_forms,
};
use domain::{Secret, SecretConstructorParamsOverrides, build_secret};
use encoding::IEncoderAdapter;
#[cfg(test)]
use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingParams,
    CreateEncodingParamsOverrides, IDecoderAdapter, IEncodingConsumer, build_create_encoding_deps,
    build_create_encoding_params, build_create_encoding_payload, create_encoding,
};
use envelope::{EnvelopeComponents, PublicKeysComponents};
use hash_to_scalar::IHashToScalarAdapter;
#[cfg(test)]
use hash_to_scalar::{
    CreateHashToScalarParams, build_create_hash_to_scalar_deps, build_create_hash_to_scalar_params,
    build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use pairing::{
    AddScalarParams, AddScalarPayload, G1GeneratorParams, G1GeneratorPayload, G2GeneratorParams,
    G2GeneratorPayload, IPairingAdapter, IPairingArithmetic, ISampleUniformScalar, MsmG1Params,
    MsmG1Payload, MsmG1Term, MsmG2Params, MsmG2Payload, MsmG2Term, MulG1Params, MulG1Payload,
    MulG2Params, MulG2Payload, MulScalarParams, MulScalarPayload, NegScalarParams,
    NegScalarPayload, SampleUniformScalarParams, SampleUniformScalarPayload,
    VerifierGroupArithmetic,
};
#[cfg(test)]
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParams, IPairingConsumer,
    IPairingReference, build_create_pairing_deps, build_create_pairing_params,
    build_create_pairing_payload, create_pairing,
};
use random::IRandomSourceAdapter;
#[cfg(test)]
use random::{
    CreateRandomSourceParams, CreateRandomSourceParamsOverrides, CreateRandomSourcePayload,
    build_create_random_source_deps, build_create_random_source_params, create_random_source,
};

use super::challenge::provides::{
    ChallengeDeps, ChallengeParams, ChallengePayload, ChallengeTranscript, challenge,
};
#[cfg(test)]
use super::interface::{
    ISchnorrFsCaseConsumer, RunSchnorrFsCaseFn, SchnorrFsCaseDeps, SchnorrFsCaseErrorReturn,
    SchnorrFsCaseParams, SchnorrFsCasePayload, SchnorrFsCaseReturn, SchnorrFsCaseSuccessReturn,
};
use super::interface::{
    SchnorrFsDeliveryProof, SchnorrFsDeliveryProofConstructorParams, SchnorrFsProof,
};
use crate::factory::provides::{
    AlgebraicMintStatement, AlgebraicTransferStatement, MintResponses,
    MintSecondGroupFirstMessages, ProveMintPayload, ProveTransferPayload, TransferResponses,
    TransferSecondGroupFirstMessages,
};
use crate::mint_statement::provides::{
    MintFirstMessages, MintStatement, MintStatementOverrides, build_mint_statement,
};
use crate::transfer_statement::provides::{
    TransferFirstMessages, TransferStatement, TransferStatementOverrides, build_transfer_statement,
};

const FILL_NONCE_ALPHA: u8 = 0x71;
const FILL_NONCE_R: u8 = 0x72;
const FILL_NONCE_RHO: u8 = 0x73;
const FILL_NONCE_SIGMA: u8 = 0x74;
const FILL_NONCE_X: u8 = 0x75;
const FILL_NONCE_Y: u8 = 0x76;
const FILL_NONCE_S: u8 = 0x77;
const FILL_NONCE_RHO_TRANSFER: u8 = 0x78;
const FILL_NONCE_SIGMA_TRANSFER: u8 = 0x79;

fn sampled_scalar_secret<P: IPairingAdapter>(fill: u8) -> Secret<P::Scalar> {
    P::Scalar::sample_from_uniform_bytes(
        SampleUniformScalarParams,
        SampleUniformScalarPayload {
            uniform: build_secret(
                vec![fill; <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH],
                SecretConstructorParamsOverrides,
            ),
        },
    )
    .expect("sampling a default scalar always succeeds")
    .scalar
}

fn sampled_scalar<P: IPairingAdapter>(fill: u8) -> P::Scalar {
    sampled_scalar_secret::<P>(fill).expose().clone()
}

fn default_g1<P: IPairingAdapter>(pairing: &P) -> P::G1 {
    let Ok(success) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
    success.point
}

fn default_g2<P: IPairingAdapter>(pairing: &P) -> P::G2 {
    let Ok(success) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
    success.point
}

fn mul_g1_by<P: IPairingAdapter>(pairing: &P, point: P::G1, scalar: P::Scalar) -> P::G1 {
    let Ok(success) = pairing.mul_g1(MulG1Params, MulG1Payload { point, scalar });
    success.product
}

fn mul_g2_by<P: IPairingAdapter>(pairing: &P, point: P::G2, scalar: P::Scalar) -> P::G2 {
    let Ok(success) = pairing.mul_g2(MulG2Params, MulG2Payload { point, scalar });
    success.product
}

fn msm_g1_of<P: IPairingAdapter>(pairing: &P, terms: Vec<(P::G1, P::Scalar)>) -> P::G1 {
    let Ok(success) = pairing.msm_g1(
        MsmG1Params,
        MsmG1Payload {
            terms: terms
                .into_iter()
                .map(|(base, scalar)| MsmG1Term { base, scalar })
                .collect(),
        },
    );
    success.sum
}

fn msm_g2_of<P: IPairingAdapter>(pairing: &P, terms: Vec<(P::G2, P::Scalar)>) -> P::G2 {
    let Ok(success) = pairing.msm_g2(
        MsmG2Params,
        MsmG2Payload {
            terms: terms
                .into_iter()
                .map(|(base, scalar)| MsmG2Term { base, scalar })
                .collect(),
        },
    );
    success.sum
}

fn add_scalar_of<P: IPairingArithmetic>(
    pairing: &P,
    left: P::Scalar,
    right: P::Scalar,
) -> P::Scalar {
    let Ok(success) = pairing.add_scalar(AddScalarParams, AddScalarPayload { left, right });
    success.sum
}

fn mul_scalar_of<P: IPairingArithmetic>(
    pairing: &P,
    left: P::Scalar,
    right: P::Scalar,
) -> P::Scalar {
    let Ok(success) = pairing.mul_scalar(MulScalarParams, MulScalarPayload { left, right });
    success.product
}

fn neg_scalar_of<P: IPairingArithmetic>(pairing: &P, scalar: P::Scalar) -> P::Scalar {
    let Ok(success) = pairing.neg_scalar(NegScalarParams, NegScalarPayload { scalar });
    success.negation
}

fn response_of<P: IPairingArithmetic>(
    pairing: &P,
    nonce: P::Scalar,
    challenge: &P::Scalar,
    witness: &P::Scalar,
) -> P::Scalar {
    add_scalar_of(
        pairing,
        nonce,
        mul_scalar_of(pairing, challenge.clone(), witness.clone()),
    )
}

fn mint_first_messages_from_nonces<P: IPairingArithmetic, F: IChainForms>(
    pairing: &P,
    nonces: &[P::Scalar; 4],
    statement: &AlgebraicMintStatement<F, P::G1, P::G2>,
) -> MintFirstMessages<P::G1, P::G2> {
    let [k_alpha, k_r, k_rho, k_sigma] = nonces;
    let g1 = default_g1(pairing);
    let g2 = default_g2(pairing);
    MintFirstMessages {
        hpub: mul_g2_by(pairing, g2.clone(), k_alpha.clone()),
        c1: mul_g1_by(pairing, g1.clone(), k_rho.clone()),
        c2: msm_g1_of(
            pairing,
            vec![
                (g1, k_alpha.clone()),
                (statement.identity_element.clone(), k_r.clone()),
                (statement.buyer_keys.pk1.clone(), k_rho.clone()),
            ],
        ),
        d1: mul_g2_by(pairing, g2.clone(), k_sigma.clone()),
        d2: msm_g2_of(
            pairing,
            vec![
                (g2, k_r.clone()),
                (statement.buyer_keys.pk2.clone(), k_sigma.clone()),
            ],
        ),
    }
}

fn transfer_first_messages_from_nonces<P: IPairingArithmetic, F: IChainForms>(
    pairing: &P,
    nonces: &[P::Scalar; 5],
    statement: &AlgebraicTransferStatement<F, P::G1, P::G2>,
) -> TransferFirstMessages<P::G1, P::G2> {
    let [k_x, k_y, k_s, k_rho, k_sigma] = nonces;
    let g1 = default_g1(pairing);
    let g2 = default_g2(pairing);
    TransferFirstMessages {
        pk1: mul_g1_by(pairing, g1.clone(), k_x.clone()),
        c1: mul_g1_by(pairing, g1.clone(), k_rho.clone()),
        c2: msm_g1_of(
            pairing,
            vec![
                (
                    statement.old_envelope.c1.clone(),
                    neg_scalar_of(pairing, k_x.clone()),
                ),
                (statement.identity_element.clone(), k_s.clone()),
                (statement.buyer_keys.pk1.clone(), k_rho.clone()),
            ],
        ),
        pk2: mul_g2_by(pairing, g2.clone(), k_y.clone()),
        d1: mul_g2_by(pairing, g2.clone(), k_sigma.clone()),
        d2: msm_g2_of(
            pairing,
            vec![
                (
                    statement.old_envelope.d1.clone(),
                    neg_scalar_of(pairing, k_y.clone()),
                ),
                (g2, k_s.clone()),
                (statement.buyer_keys.pk2.clone(), k_sigma.clone()),
            ],
        ),
    }
}

#[derive(Default)]
pub struct SchnorrFsDeliveryProofConstructorParamsOverrides {
    pub verifier_form: Option<VerifierGroupArithmetic>,
}

pub fn build_schnorr_fs_delivery_proof_constructor_params<
    'a,
    P: IPairingArithmetic,
    E: IEncoderAdapter,
>(
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    random: &'a dyn IRandomSourceAdapter,
    overrides: SchnorrFsDeliveryProofConstructorParamsOverrides,
) -> SchnorrFsDeliveryProofConstructorParams<'a, P, E> {
    SchnorrFsDeliveryProofConstructorParams {
        pairing,
        hash_to_scalar,
        encoder,
        random,
        verifier_form: overrides
            .verifier_form
            .unwrap_or(VerifierGroupArithmetic::BothGroups),
    }
}

pub fn build_schnorr_fs_delivery_proof<
    'a,
    P: IPairingArithmetic,
    E: IEncoderAdapter,
    F: IChainForms,
>(
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    random: &'a dyn IRandomSourceAdapter,
    overrides: SchnorrFsDeliveryProofConstructorParamsOverrides,
) -> SchnorrFsDeliveryProof<'a, P, E, F> {
    SchnorrFsDeliveryProof::try_new(build_schnorr_fs_delivery_proof_constructor_params(
        pairing,
        hash_to_scalar,
        encoder,
        random,
        overrides,
    ))
    .expect("the tags are admitted by the domain tag type")
}

pub struct SchnorrFsMintProofOverrides<P: IPairingAdapter> {
    pub challenge: Option<P::Scalar>,
    pub responses: Option<MintResponses<P::Scalar>>,
    pub first_messages: Option<MintSecondGroupFirstMessages<P::G2>>,
}

impl<P: IPairingAdapter> Default for SchnorrFsMintProofOverrides<P> {
    fn default() -> Self {
        Self {
            challenge: None,
            responses: None,
            first_messages: None,
        }
    }
}

pub fn build_schnorr_fs_mint_proof<
    'a,
    P: IPairingArithmetic,
    E: IEncoderAdapter,
    F: IChainForms,
>(
    adapter: &SchnorrFsDeliveryProof<'a, P, E, F>,
    payload: &ProveMintPayload<'_, F, P::Scalar, P::G1, P::G2>,
    overrides: SchnorrFsMintProofOverrides<P>,
) -> SchnorrFsProof<P> {
    let pairing = adapter.pairing;
    let statement = payload.statement;
    let nonces = [
        sampled_scalar::<P>(FILL_NONCE_ALPHA),
        sampled_scalar::<P>(FILL_NONCE_R),
        sampled_scalar::<P>(FILL_NONCE_RHO),
        sampled_scalar::<P>(FILL_NONCE_SIGMA),
    ];
    let formed = mint_first_messages_from_nonces(pairing, &nonces, statement);
    let first_messages = overrides
        .first_messages
        .unwrap_or(MintSecondGroupFirstMessages {
            hpub: formed.hpub.clone(),
            d1: formed.d1.clone(),
            d2: formed.d2.clone(),
        });
    let transcript: MintStatement<P, F> = build_mint_statement(
        pairing,
        MintStatementOverrides {
            suite_identifier: Some(statement.context.suite_identifier.clone()),
            chain: Some(statement.context.chain.clone()),
            entitlement_contract: Some(statement.context.entitlement_contract.clone()),
            asset_root: Some(statement.context.asset_root),
            parameter_set_identifier: Some(statement.context.parameter_set_identifier.clone()),
            source_entitlement: Some(statement.context.source_entitlement.clone()),
            target_entitlement: Some(statement.context.target_entitlement.clone()),
            old_interval: Some(statement.context.old_interval.clone()),
            new_interval: Some(statement.context.new_interval.clone()),
            purpose: Some(statement.context.purpose),
            seller: Some(statement.context.seller.clone()),
            buyer: Some(statement.context.buyer.clone()),
            buyer_keys: Some(PublicKeysComponents {
                pk1: statement.buyer_keys.pk1.clone(),
                pk2: statement.buyer_keys.pk2.clone(),
            }),
            envelope: Some(EnvelopeComponents {
                c1: statement.envelope.c1.clone(),
                c2: statement.envelope.c2.clone(),
                d1: statement.envelope.d1.clone(),
                d2: statement.envelope.d2.clone(),
            }),
            expiry: Some(statement.context.expiry),
            first_messages: Some(MintFirstMessages {
                hpub: first_messages.hpub.clone(),
                c1: formed.c1.clone(),
                c2: formed.c2.clone(),
                d1: first_messages.d1.clone(),
                d2: first_messages.d2.clone(),
            }),
        },
    );
    let challenge = overrides.challenge.unwrap_or_else(|| {
        challenge(
            &ChallengeDeps {
                pairing,
                encoder: adapter.encoder,
                tag: &adapter.challenge_tag,
                hash_to_scalar: adapter.hash_to_scalar,
            },
            ChallengeParams,
            ChallengePayload {
                transcript: ChallengeTranscript::Mint(&transcript),
            },
        )
        .expect("the challenge over a formed transcript always computes")
        .challenge
    });
    let responses = overrides.responses.unwrap_or_else(|| MintResponses {
        alpha: response_of(
            pairing,
            nonces[0].clone(),
            &challenge,
            payload.master_scalar.value.expose(),
        ),
        r: response_of(
            pairing,
            nonces[1].clone(),
            &challenge,
            payload.credential_randomness.expose(),
        ),
        rho: response_of(
            pairing,
            nonces[2].clone(),
            &challenge,
            payload.coins.rho.expose(),
        ),
        sigma: response_of(
            pairing,
            nonces[3].clone(),
            &challenge,
            payload.coins.sigma.expose(),
        ),
    });
    match adapter.verifier_form {
        VerifierGroupArithmetic::BothGroups => SchnorrFsProof::MintBothGroups {
            challenge,
            responses,
        },
        VerifierGroupArithmetic::FirstGroupOnly => SchnorrFsProof::MintFirstGroupOnly {
            challenge,
            responses,
            first_messages,
        },
    }
}

pub struct SchnorrFsTransferProofOverrides<P: IPairingAdapter> {
    pub challenge: Option<P::Scalar>,
    pub responses: Option<TransferResponses<P::Scalar>>,
    pub first_messages: Option<TransferSecondGroupFirstMessages<P::G2>>,
}

impl<P: IPairingAdapter> Default for SchnorrFsTransferProofOverrides<P> {
    fn default() -> Self {
        Self {
            challenge: None,
            responses: None,
            first_messages: None,
        }
    }
}

pub fn build_schnorr_fs_transfer_proof<
    'a,
    P: IPairingArithmetic,
    E: IEncoderAdapter,
    F: IChainForms,
>(
    adapter: &SchnorrFsDeliveryProof<'a, P, E, F>,
    payload: &ProveTransferPayload<'_, F, P::Scalar, P::G1, P::G2>,
    overrides: SchnorrFsTransferProofOverrides<P>,
) -> SchnorrFsProof<P> {
    let pairing = adapter.pairing;
    let statement = payload.statement;
    let nonces = [
        sampled_scalar::<P>(FILL_NONCE_X),
        sampled_scalar::<P>(FILL_NONCE_Y),
        sampled_scalar::<P>(FILL_NONCE_S),
        sampled_scalar::<P>(FILL_NONCE_RHO_TRANSFER),
        sampled_scalar::<P>(FILL_NONCE_SIGMA_TRANSFER),
    ];
    let formed = transfer_first_messages_from_nonces(pairing, &nonces, statement);
    let first_messages = overrides
        .first_messages
        .unwrap_or(TransferSecondGroupFirstMessages {
            pk2: formed.pk2.clone(),
            d1: formed.d1.clone(),
            d2: formed.d2.clone(),
        });
    let transcript: TransferStatement<P, F> = build_transfer_statement(
        pairing,
        TransferStatementOverrides {
            suite_identifier: Some(statement.context.suite_identifier.clone()),
            chain: Some(statement.context.chain.clone()),
            entitlement_contract: Some(statement.context.entitlement_contract.clone()),
            asset_root: Some(statement.context.asset_root),
            parameter_set_identifier: Some(statement.context.parameter_set_identifier.clone()),
            source_entitlement: Some(statement.context.source_entitlement.clone()),
            target_entitlement: Some(statement.context.target_entitlement.clone()),
            old_interval: Some(statement.context.old_interval.clone()),
            new_interval: Some(statement.context.new_interval.clone()),
            purpose: Some(statement.context.purpose),
            seller: Some(statement.context.seller.clone()),
            buyer: Some(statement.context.buyer.clone()),
            seller_keys: Some(PublicKeysComponents {
                pk1: statement.seller_keys.pk1.clone(),
                pk2: statement.seller_keys.pk2.clone(),
            }),
            buyer_keys: Some(PublicKeysComponents {
                pk1: statement.buyer_keys.pk1.clone(),
                pk2: statement.buyer_keys.pk2.clone(),
            }),
            old_envelope: Some(EnvelopeComponents {
                c1: statement.old_envelope.c1.clone(),
                c2: statement.old_envelope.c2.clone(),
                d1: statement.old_envelope.d1.clone(),
                d2: statement.old_envelope.d2.clone(),
            }),
            new_envelope: Some(EnvelopeComponents {
                c1: statement.new_envelope.c1.clone(),
                c2: statement.new_envelope.c2.clone(),
                d1: statement.new_envelope.d1.clone(),
                d2: statement.new_envelope.d2.clone(),
            }),
            expiry: Some(statement.context.expiry),
            first_messages: Some(TransferFirstMessages {
                pk1: formed.pk1.clone(),
                c1: formed.c1.clone(),
                c2: formed.c2.clone(),
                pk2: first_messages.pk2.clone(),
                d1: first_messages.d1.clone(),
                d2: first_messages.d2.clone(),
            }),
        },
    );
    let challenge = overrides.challenge.unwrap_or_else(|| {
        challenge(
            &ChallengeDeps {
                pairing,
                encoder: adapter.encoder,
                tag: &adapter.challenge_tag,
                hash_to_scalar: adapter.hash_to_scalar,
            },
            ChallengeParams,
            ChallengePayload {
                transcript: ChallengeTranscript::Transfer(&transcript),
            },
        )
        .expect("the challenge over a formed transcript always computes")
        .challenge
    });
    let responses = overrides.responses.unwrap_or_else(|| TransferResponses {
        x: response_of(
            pairing,
            nonces[0].clone(),
            &challenge,
            payload.seller_secrets.x.expose(),
        ),
        y: response_of(
            pairing,
            nonces[1].clone(),
            &challenge,
            payload.seller_secrets.y.expose(),
        ),
        s: response_of(
            pairing,
            nonces[2].clone(),
            &challenge,
            payload.offset.expose(),
        ),
        rho: response_of(
            pairing,
            nonces[3].clone(),
            &challenge,
            payload.coins.rho.expose(),
        ),
        sigma: response_of(
            pairing,
            nonces[4].clone(),
            &challenge,
            payload.coins.sigma.expose(),
        ),
    });
    match adapter.verifier_form {
        VerifierGroupArithmetic::BothGroups => SchnorrFsProof::TransferBothGroups {
            challenge,
            responses,
        },
        VerifierGroupArithmetic::FirstGroupOnly => SchnorrFsProof::TransferFirstGroupOnly {
            challenge,
            responses,
            first_messages,
        },
    }
}

#[cfg(test)]
#[derive(Default)]
pub struct SchnorrFsCaseDepsOverrides;

#[cfg(test)]
pub fn build_schnorr_fs_case_deps<C: ISchnorrFsCaseConsumer>(
    consumer: C,
    _overrides: SchnorrFsCaseDepsOverrides,
) -> SchnorrFsCaseDeps<C> {
    SchnorrFsCaseDeps { consumer }
}

#[cfg(test)]
#[derive(Default)]
pub struct SchnorrFsCaseParamsOverrides {
    pub pairing: Option<CreatePairingParams>,
    pub chain_forms: Option<CreateChainFormsParams>,
    pub encoding: Option<CreateEncodingParams>,
    pub hash_to_scalar: Option<CreateHashToScalarParams>,
    pub random_source: Option<CreateRandomSourceParams>,
}

#[cfg(test)]
pub fn build_schnorr_fs_case_params(
    overrides: SchnorrFsCaseParamsOverrides,
) -> SchnorrFsCaseParams {
    SchnorrFsCaseParams {
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
        random_source: overrides
            .random_source
            .unwrap_or_else(|| build_create_random_source_params(Default::default())),
    }
}

#[cfg(test)]
#[derive(Default)]
pub struct SchnorrFsCasePayloadOverrides;

#[cfg(test)]
pub fn build_schnorr_fs_case_payload(
    _overrides: SchnorrFsCasePayloadOverrides,
) -> SchnorrFsCasePayload {
    SchnorrFsCasePayload
}

#[cfg(test)]
#[derive(Default)]
pub struct SchnorrFsCaseSuccessReturnOverrides;

#[cfg(test)]
pub fn build_schnorr_fs_case_success_return<O>(
    output: O,
    _overrides: SchnorrFsCaseSuccessReturnOverrides,
) -> SchnorrFsCaseSuccessReturn<O> {
    SchnorrFsCaseSuccessReturn { output }
}

#[cfg(test)]
struct CaseAtPairing<'a, C: ISchnorrFsCaseConsumer> {
    pub consumer: &'a C,
    pub chain_forms: CreateChainFormsParams,
    pub encoding: CreateEncodingParams,
    pub hash_to_scalar: CreateHashToScalarParams,
    pub random_source: CreateRandomSourceParams,
}

#[cfg(test)]
impl<C: ISchnorrFsCaseConsumer> IPairingConsumer for CaseAtPairing<'_, C> {
    type Output = SchnorrFsCaseReturn<C::Output>;

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
            random_source: build_create_random_source_params(CreateRandomSourceParamsOverrides {
                kind: Some(self.random_source.kind),
            }),
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
            Err(error) => Err(SchnorrFsCaseErrorReturn::Encoding(error)),
        }
    }
}

#[cfg(test)]
struct CaseAtEncoding<'a, 'p, C: ISchnorrFsCaseConsumer, P: IPairingArithmetic + IPairingReference>
{
    pub consumer: &'a C,
    pub pairing: &'p P,
    pub chain_forms: CreateChainFormsParams,
    pub hash_to_scalar: CreateHashToScalarParams,
    pub random_source: CreateRandomSourceParams,
}

#[cfg(test)]
impl<C: ISchnorrFsCaseConsumer, P: IPairingArithmetic + IPairingReference> IEncodingConsumer
    for CaseAtEncoding<'_, '_, C, P>
{
    type Output = SchnorrFsCaseReturn<C::Output>;

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
                random_source: build_create_random_source_params(
                    CreateRandomSourceParamsOverrides {
                        kind: Some(self.random_source.kind),
                    },
                ),
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
            Err(error) => Err(SchnorrFsCaseErrorReturn::ChainForms(error)),
        }
    }
}

#[cfg(test)]
struct CaseAtChainForms<
    'a,
    'p,
    'e,
    C: ISchnorrFsCaseConsumer,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
> {
    pub consumer: &'a C,
    pub pairing: &'p P,
    pub encoder: &'e E,
    pub hash_to_scalar: CreateHashToScalarParams,
    pub random_source: CreateRandomSourceParams,
}

#[cfg(test)]
impl<
    C: ISchnorrFsCaseConsumer,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
> IChainFormsConsumer for CaseAtChainForms<'_, '_, '_, C, P, E>
{
    type Output = SchnorrFsCaseReturn<C::Output>;

    fn consume_chain_forms<F: IChainForms>(
        &self,
        _params: ConsumeChainFormsParams,
        payload: ConsumeChainFormsPayload<F>,
    ) -> Self::Output {
        let hash_to_scalar = match create_hash_to_scalar::<P::Scalar>(
            &build_create_hash_to_scalar_deps(Default::default()),
            self.hash_to_scalar,
            build_create_hash_to_scalar_payload(Default::default()),
        ) {
            Ok(success) => success,
            Err(error) => return Err(SchnorrFsCaseErrorReturn::HashToScalar(error)),
        };
        let random = match create_random_source(
            &build_create_random_source_deps(Default::default()),
            build_create_random_source_params(CreateRandomSourceParamsOverrides {
                kind: Some(self.random_source.kind),
            }),
            CreateRandomSourcePayload,
        ) {
            Ok(success) => success,
            Err(error) => return Err(SchnorrFsCaseErrorReturn::RandomSource(error)),
        };
        let output = self.consumer.consume_schnorr_fs_case(
            self.pairing,
            &payload.forms,
            self.encoder,
            &*hash_to_scalar.adapter,
            &*random.adapter,
            &SchnorrFsCasePayload,
        );
        Ok(SchnorrFsCaseSuccessReturn { output })
    }
}

#[cfg(test)]
pub fn run_schnorr_fs_case<C: ISchnorrFsCaseConsumer>(
    deps: &SchnorrFsCaseDeps<C>,
    params: SchnorrFsCaseParams,
    _payload: SchnorrFsCasePayload,
) -> SchnorrFsCaseReturn<C::Output> {
    let _: RunSchnorrFsCaseFn<C> = run_schnorr_fs_case::<C>;
    let pairing_deps = build_create_pairing_deps(
        CaseAtPairing {
            consumer: &deps.consumer,
            chain_forms: params.chain_forms,
            encoding: params.encoding,
            hash_to_scalar: params.hash_to_scalar,
            random_source: params.random_source,
        },
        Default::default(),
    );
    match create_pairing(
        &pairing_deps,
        params.pairing,
        build_create_pairing_payload(Default::default()),
    ) {
        Ok(success) => match success.output {
            Ok(SchnorrFsCaseSuccessReturn { output }) => Ok(SchnorrFsCaseSuccessReturn { output }),
            Err(error) => Err(error),
        },
        Err(error) => Err(SchnorrFsCaseErrorReturn::Pairing(error)),
    }
}
