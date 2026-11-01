#![allow(dead_code, clippy::expect_used, clippy::too_many_arguments)]

use super::challenge::provides::{
    ChallengeDeps, ChallengeParams, ChallengePayload, ChallengeTranscript, challenge,
};
use super::interface::{
    ISchnorrFsCaseConsumer, SchnorrFsCaseDeps, SchnorrFsCaseErrorReturn, SchnorrFsCaseParams,
    SchnorrFsCasePayload, SchnorrFsCaseReturn, SchnorrFsCaseSuccessReturn, SchnorrFsDeliveryProof,
    SchnorrFsDeliveryProofConstructorParams, SchnorrFsProof,
};
use crate::factory::provides::{
    MintResponses, MintSecondGroupFirstMessages, ProveMintPayload, ProveTransferPayload,
    TransferResponses, TransferSecondGroupFirstMessages,
};
use crate::mint_statement::provides::{
    MintFirstMessages, MintStatement, MintStatementOverrides, build_mint_statement,
};
use crate::transfer_statement::provides::{
    TransferFirstMessages, TransferStatement, TransferStatementOverrides, build_transfer_statement,
};
use chain::{
    ConsumeChainFormsParams, ConsumeChainFormsPayload, CreateChainFormsParams, IChainForms,
    IChainFormsConsumer, build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, create_chain_forms,
};
use domain::{Secret, build_secret};
use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingParams,
    CreateEncodingParamsOverrides, IDecoderAdapter, IEncoderAdapter, IEncodingConsumer,
    build_create_encoding_deps, build_create_encoding_params, build_create_encoding_payload,
    create_encoding,
};
use envelope::{EnvelopeComponents, PublicKeysComponents};
use hash_to_scalar::{
    CreateHashToScalarParams, DomainTag, IHashToScalarAdapter, build_create_hash_to_scalar_deps,
    build_create_hash_to_scalar_params, build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use hashing::{Root, RootConstructorParams};
use pairing::{
    AddScalarParams, AddScalarPayload, ConsumePairingParams, ConsumePairingPayload,
    CreatePairingParams, G1GeneratorParams, G1GeneratorPayload, G2GeneratorParams,
    G2GeneratorPayload, IPairingAdapter, IPairingArithmetic, IPairingConsumer, IPairingReference,
    ISampleUniformScalar, MsmG1Params, MsmG1Payload, MsmG1Term, MsmG2Params, MsmG2Payload,
    MsmG2Term, MulG1Params, MulG1Payload, MulG2Params, MulG2Payload, MulScalarParams,
    MulScalarPayload, NegScalarParams, NegScalarPayload, SampleUniformScalarParams,
    SampleUniformScalarPayload, VerifierGroupArithmetic, build_create_pairing_deps,
    build_create_pairing_params, build_create_pairing_payload, create_pairing,
};
use random::{
    CreateRandomSourceParams, CreateRandomSourceParamsOverrides, CreateRandomSourcePayload,
    IRandomSourceAdapter, build_create_random_source_deps, build_create_random_source_params,
    create_random_source,
};

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
    let k_alpha = constant_scalar::<P>(0x61);
    let k_r = constant_scalar::<P>(0x62);
    let k_rho = constant_scalar::<P>(0x63);
    let k_sigma = constant_scalar::<P>(0x64);

    let statement = payload.statement;
    let g1 = generator_g1(pairing);
    let g2 = generator_g2(pairing);
    let t_hpub = product_g2(pairing, g2.clone(), k_alpha.clone());
    let t_c1 = product_g1(pairing, g1.clone(), k_rho.clone());
    let t_c2 = msm_g1(
        pairing,
        vec![
            MsmG1Term {
                base: g1.clone(),
                scalar: k_alpha.clone(),
            },
            MsmG1Term {
                base: statement.identity_element.clone(),
                scalar: k_r.clone(),
            },
            MsmG1Term {
                base: statement.buyer_keys.pk1.clone(),
                scalar: k_rho.clone(),
            },
        ],
    );
    let t_d1 = product_g2(pairing, g2.clone(), k_sigma.clone());
    let t_d2 = msm_g2(
        pairing,
        vec![
            MsmG2Term {
                base: g2.clone(),
                scalar: k_r.clone(),
            },
            MsmG2Term {
                base: statement.buyer_keys.pk2.clone(),
                scalar: k_sigma.clone(),
            },
        ],
    );
    let first_messages = overrides
        .first_messages
        .unwrap_or_else(|| MintSecondGroupFirstMessages {
            hpub: t_hpub,
            d1: t_d1,
            d2: t_d2,
        });

    let asset_root = Root::try_new(RootConstructorParams {
        bytes: *statement.context.asset_identity_hash.as_bytes(),
    })
    .expect("the asset identity hash is never all zero");
    let transcript: MintStatement<P, F> = build_mint_statement::<P, F>(
        pairing,
        MintStatementOverrides {
            suite_identifier: Some(statement.context.suite_identifier.clone()),
            chain: Some(statement.context.chain.clone()),
            entitlement_contract: Some(statement.context.entitlement_contract.clone()),
            asset_root: Some(asset_root),
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
                c1: t_c1,
                c2: t_c2,
                d1: first_messages.d1.clone(),
                d2: first_messages.d2.clone(),
            }),
        },
    );
    let c = overrides
        .challenge
        .unwrap_or_else(|| challenge_scalar(adapter, ChallengeTranscript::Mint(&transcript)));

    let responses = overrides.responses.unwrap_or_else(|| MintResponses {
        alpha: response(pairing, k_alpha, &c, payload.master_scalar.value.expose()),
        r: response(pairing, k_r, &c, payload.credential_randomness.expose()),
        rho: response(pairing, k_rho, &c, payload.coins.rho.expose()),
        sigma: response(pairing, k_sigma, &c, payload.coins.sigma.expose()),
    });

    match adapter.verifier_form {
        VerifierGroupArithmetic::BothGroups => SchnorrFsProof::MintBothGroups {
            challenge: c,
            responses,
        },
        VerifierGroupArithmetic::FirstGroupOnly => SchnorrFsProof::MintFirstGroupOnly {
            challenge: c,
            responses,
            first_messages,
        },
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
    let k_x = constant_scalar::<P>(0x71);
    let k_y = constant_scalar::<P>(0x72);
    let k_s = constant_scalar::<P>(0x73);
    let k_rho = constant_scalar::<P>(0x74);
    let k_sigma = constant_scalar::<P>(0x75);

    let statement = payload.statement;
    let g1 = generator_g1(pairing);
    let g2 = generator_g2(pairing);
    let t_pk1 = product_g1(pairing, g1.clone(), k_x.clone());
    let t_c1 = product_g1(pairing, g1.clone(), k_rho.clone());
    let t_c2 = msm_g1(
        pairing,
        vec![
            MsmG1Term {
                base: statement.old_envelope.c1.clone(),
                scalar: negated_scalar(pairing, k_x.clone()),
            },
            MsmG1Term {
                base: statement.identity_element.clone(),
                scalar: k_s.clone(),
            },
            MsmG1Term {
                base: statement.buyer_keys.pk1.clone(),
                scalar: k_rho.clone(),
            },
        ],
    );
    let t_pk2 = product_g2(pairing, g2.clone(), k_y.clone());
    let t_d1 = product_g2(pairing, g2.clone(), k_sigma.clone());
    let t_d2 = msm_g2(
        pairing,
        vec![
            MsmG2Term {
                base: statement.old_envelope.d1.clone(),
                scalar: negated_scalar(pairing, k_y.clone()),
            },
            MsmG2Term {
                base: g2.clone(),
                scalar: k_s.clone(),
            },
            MsmG2Term {
                base: statement.buyer_keys.pk2.clone(),
                scalar: k_sigma.clone(),
            },
        ],
    );
    let first_messages =
        overrides
            .first_messages
            .unwrap_or_else(|| TransferSecondGroupFirstMessages {
                pk2: t_pk2,
                d1: t_d1,
                d2: t_d2,
            });

    let asset_root = Root::try_new(RootConstructorParams {
        bytes: *statement.context.asset_identity_hash.as_bytes(),
    })
    .expect("the asset identity hash is never all zero");
    let transcript: TransferStatement<P, F> = build_transfer_statement::<P, F>(
        pairing,
        TransferStatementOverrides {
            suite_identifier: Some(statement.context.suite_identifier.clone()),
            chain: Some(statement.context.chain.clone()),
            entitlement_contract: Some(statement.context.entitlement_contract.clone()),
            asset_root: Some(asset_root),
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
                pk1: t_pk1,
                c1: t_c1,
                c2: t_c2,
                pk2: first_messages.pk2.clone(),
                d1: first_messages.d1.clone(),
                d2: first_messages.d2.clone(),
            }),
        },
    );
    let c = overrides
        .challenge
        .unwrap_or_else(|| challenge_scalar(adapter, ChallengeTranscript::Transfer(&transcript)));

    let responses = overrides.responses.unwrap_or_else(|| TransferResponses {
        x: response(pairing, k_x, &c, payload.seller_secrets.x.expose()),
        y: response(pairing, k_y, &c, payload.seller_secrets.y.expose()),
        s: response(pairing, k_s, &c, payload.offset.expose()),
        rho: response(pairing, k_rho, &c, payload.coins.rho.expose()),
        sigma: response(pairing, k_sigma, &c, payload.coins.sigma.expose()),
    });

    match adapter.verifier_form {
        VerifierGroupArithmetic::BothGroups => SchnorrFsProof::TransferBothGroups {
            challenge: c,
            responses,
        },
        VerifierGroupArithmetic::FirstGroupOnly => SchnorrFsProof::TransferFirstGroupOnly {
            challenge: c,
            responses,
            first_messages,
        },
    }
}

fn challenge_scalar<'a, P: IPairingArithmetic, E: IEncoderAdapter, F: IChainForms>(
    adapter: &SchnorrFsDeliveryProof<'a, P, E, F>,
    transcript: ChallengeTranscript<'_, P, F>,
) -> P::Scalar {
    let tag: &DomainTag = &adapter.challenge_tag;
    challenge(
        &ChallengeDeps {
            pairing: adapter.pairing,
            encoder: adapter.encoder,
            tag,
            hash_to_scalar: adapter.hash_to_scalar,
        },
        ChallengeParams,
        ChallengePayload { transcript },
    )
    .expect("the transcript's challenge is computed")
    .challenge
}

fn constant_scalar<P: IPairingAdapter>(value: u8) -> P::Scalar {
    let uniform: Secret<Vec<u8>> = build_secret(
        vec![value; <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH],
        Default::default(),
    );
    P::Scalar::sample_from_uniform_bytes(
        SampleUniformScalarParams,
        SampleUniformScalarPayload { uniform },
    )
    .expect("sampling uniform bytes of the declared length succeeds")
    .scalar
    .expose()
    .clone()
}

fn generator_g1<P: IPairingAdapter>(pairing: &P) -> P::G1 {
    let Ok(generator) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
    generator.point
}

fn generator_g2<P: IPairingAdapter>(pairing: &P) -> P::G2 {
    let Ok(generator) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
    generator.point
}

fn product_g1<P: IPairingAdapter>(pairing: &P, point: P::G1, scalar: P::Scalar) -> P::G1 {
    let Ok(product) = pairing.mul_g1(MulG1Params, MulG1Payload { point, scalar });
    product.product
}

fn product_g2<P: IPairingAdapter>(pairing: &P, point: P::G2, scalar: P::Scalar) -> P::G2 {
    let Ok(product) = pairing.mul_g2(MulG2Params, MulG2Payload { point, scalar });
    product.product
}

fn msm_g1<P: IPairingAdapter>(pairing: &P, terms: Vec<MsmG1Term<P::G1, P::Scalar>>) -> P::G1 {
    let Ok(sum) = pairing.msm_g1(MsmG1Params, MsmG1Payload { terms });
    sum.sum
}

fn msm_g2<P: IPairingAdapter>(pairing: &P, terms: Vec<MsmG2Term<P::G2, P::Scalar>>) -> P::G2 {
    let Ok(sum) = pairing.msm_g2(MsmG2Params, MsmG2Payload { terms });
    sum.sum
}

fn negated_scalar<P: IPairingArithmetic>(pairing: &P, scalar: P::Scalar) -> P::Scalar {
    let Ok(negation) = pairing.neg_scalar(NegScalarParams, NegScalarPayload { scalar });
    negation.negation
}

fn response<P: IPairingArithmetic>(
    pairing: &P,
    nonce: P::Scalar,
    challenge: &P::Scalar,
    witness: &P::Scalar,
) -> P::Scalar {
    let Ok(product) = pairing.mul_scalar(
        MulScalarParams,
        MulScalarPayload {
            left: challenge.clone(),
            right: witness.clone(),
        },
    );
    let Ok(sum) = pairing.add_scalar(
        AddScalarParams,
        AddScalarPayload {
            left: nonce,
            right: product.product,
        },
    );
    sum.sum
}

#[derive(Default)]
pub struct SchnorrFsCaseDepsOverrides;

pub fn build_schnorr_fs_case_deps<C: ISchnorrFsCaseConsumer>(
    consumer: C,
    _overrides: SchnorrFsCaseDepsOverrides,
) -> SchnorrFsCaseDeps<C> {
    SchnorrFsCaseDeps { consumer }
}

#[derive(Default)]
pub struct SchnorrFsCaseParamsOverrides {
    pub pairing: Option<CreatePairingParams>,
    pub chain_forms: Option<CreateChainFormsParams>,
    pub encoding: Option<CreateEncodingParams>,
    pub hash_to_scalar: Option<CreateHashToScalarParams>,
    pub random_source: Option<CreateRandomSourceParams>,
}

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

#[derive(Default)]
pub struct SchnorrFsCasePayloadOverrides;

pub fn build_schnorr_fs_case_payload(
    _overrides: SchnorrFsCasePayloadOverrides,
) -> SchnorrFsCasePayload {
    SchnorrFsCasePayload
}

#[derive(Default)]
pub struct SchnorrFsCaseSuccessReturnOverrides;

pub fn build_schnorr_fs_case_success_return<O>(
    output: O,
    _overrides: SchnorrFsCaseSuccessReturnOverrides,
) -> SchnorrFsCaseSuccessReturn<O> {
    SchnorrFsCaseSuccessReturn { output }
}

struct CaseAtPairing<'a, C: ISchnorrFsCaseConsumer> {
    pub consumer: &'a C,
    pub chain_forms: CreateChainFormsParams,
    pub encoding: CreateEncodingParams,
    pub hash_to_scalar: CreateHashToScalarParams,
    pub random_source: CreateRandomSourceParams,
}

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
            Err(error) => Err(SchnorrFsCaseErrorReturn::CreateEncoding(error)),
        }
    }
}

struct CaseAtEncoding<'a, 'p, C: ISchnorrFsCaseConsumer, P: IPairingArithmetic + IPairingReference>
{
    pub consumer: &'a C,
    pub pairing: &'p P,
    pub chain_forms: CreateChainFormsParams,
    pub hash_to_scalar: CreateHashToScalarParams,
    pub random_source: CreateRandomSourceParams,
}

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
            Err(error) => Err(SchnorrFsCaseErrorReturn::CreateChainForms(error)),
        }
    }
}

struct CaseAtChainForms<
    'a,
    'p,
    C: ISchnorrFsCaseConsumer,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
> {
    pub consumer: &'a C,
    pub pairing: &'p P,
    pub encoder: &'a E,
    pub hash_to_scalar: CreateHashToScalarParams,
    pub random_source: CreateRandomSourceParams,
}

impl<
    C: ISchnorrFsCaseConsumer,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
> IChainFormsConsumer for CaseAtChainForms<'_, '_, C, P, E>
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
            Err(error) => return Err(SchnorrFsCaseErrorReturn::CreateHashToScalar(error)),
        };
        let random = match create_random_source(
            &build_create_random_source_deps(Default::default()),
            build_create_random_source_params(CreateRandomSourceParamsOverrides {
                kind: Some(self.random_source.kind),
            }),
            CreateRandomSourcePayload,
        ) {
            Ok(success) => success,
            Err(error) => return Err(SchnorrFsCaseErrorReturn::CreateRandomSource(error)),
        };
        Ok(SchnorrFsCaseSuccessReturn {
            output: self.consumer.consume_schnorr_fs_case(
                self.pairing,
                &payload.forms,
                self.encoder,
                &*hash_to_scalar.adapter,
                &*random.adapter,
                &SchnorrFsCasePayload,
            ),
        })
    }
}

pub fn run_schnorr_fs_case<C: ISchnorrFsCaseConsumer>(
    deps: &SchnorrFsCaseDeps<C>,
    params: SchnorrFsCaseParams,
    _payload: SchnorrFsCasePayload,
) -> SchnorrFsCaseReturn<C::Output> {
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
        Ok(success) => success.output,
        Err(error) => Err(SchnorrFsCaseErrorReturn::CreatePairing(error)),
    }
}
