mod challenge;
mod interface;
pub(crate) mod provides;
#[cfg(test)]
mod test;

use crate::factory::provides::{
    AlgebraicMintStatement, AlgebraicStatement, AlgebraicTransferStatement,
    DELIVERY_PROOF_INTERFACE_VERSION, DeliveryProofComponents, DeliveryProofDeclaration,
    IDeliveryProofAdapter, MintResponses, MintSecondGroupFirstMessages, ProofComponentsParams,
    ProofComponentsPayload, ProofComponentsReturn, ProofComponentsSuccessReturn,
    ProofFromComponentsErrorReturn, ProofFromComponentsParams, ProofFromComponentsPayload,
    ProofFromComponentsReturn, ProofFromComponentsSuccessReturn, ProveMintErrorReturn,
    ProveMintParams, ProveMintPayload, ProveMintReturn, ProveMintSuccessReturn,
    ProveTransferErrorReturn, ProveTransferParams, ProveTransferPayload, ProveTransferReturn,
    ProveTransferSuccessReturn, TransferResponses, TransferSecondGroupFirstMessages,
    VerifyErrorReturn, VerifyParams, VerifyPayload, VerifyReturn, VerifySuccessReturn,
};
use crate::mint_statement::provides::{
    DELIVERY_STATEMENT_VERSION_ONE, MintFirstMessages, MintStatement,
    MintStatementFromFieldsErrorReturn,
};
use crate::transfer_statement::provides::{
    TransferFirstMessages, TransferStatement, TransferStatementFromFieldsErrorReturn,
};
use chain::IChainForms;
use challenge::provides::{
    ChallengeDeps, ChallengeErrorReturn, ChallengeParams, ChallengePayload, ChallengeTranscript,
    SCHNORR_FS_CHALLENGE_TAG, challenge,
};
use core::marker::PhantomData;
use domain::Secret;
use encoding::IEncoderAdapter;
use envelope::EnvelopeAlgebra;
use envelope::{EnvelopeComponents, PublicKeysComponents};
use hash_to_scalar::{
    DomainTag, DomainTagConstructorParams, HashToScalarParams, HashToScalarPayload,
};
use interface::{
    SCHNORR_FS_WEIGHT_TAG, SchnorrFsDeliveryProof, SchnorrFsDeliveryProofConstructorParams,
    SchnorrFsDeliveryProofTryNewErrorReturn, SchnorrFsDeliveryProofTryNewReturn, SchnorrFsProof,
    SchnorrFsProofFromComponentsErrorReturn, SchnorrFsProveMintErrorReturn,
    SchnorrFsProveTransferErrorReturn, SchnorrFsVerifyErrorReturn,
};
use pairing::{
    AddG1Params, AddG1Payload, AddG2Params, AddG2Payload, AddScalarParams, AddScalarPayload,
    EncodeScalarParams, EncodeScalarPayload, G1GeneratorParams, G1GeneratorPayload,
    G2GeneratorParams, G2GeneratorPayload, IPairingArithmetic, ISampleUniformScalar, MsmG1Params,
    MsmG1Payload, MsmG1Term, MsmG2Params, MsmG2Payload, MsmG2Term, MulG1Params, MulG1Payload,
    MulG2Params, MulG2Payload, MulScalarParams, MulScalarPayload, NegG1Params, NegG1Payload,
    NegG2Params, NegG2Payload, NegScalarParams, NegScalarPayload, PairingProductIsOneParams,
    PairingProductIsOnePayload, PairingProductTerm, SampleUniformScalarErrorReturn,
    SampleUniformScalarParams, SampleUniformScalarPayload, VerifierGroupArithmetic,
};
use random::{FillBytesParams, FillBytesPayload};
impl<'a, P: IPairingArithmetic, E: IEncoderAdapter, F: IChainForms>
    SchnorrFsDeliveryProof<'a, P, E, F>
{
    pub const DECLARATION: DeliveryProofDeclaration = DeliveryProofDeclaration {
        algebras: &[EnvelopeAlgebra::PairingElGamal],
        verifier_forms: &[
            VerifierGroupArithmetic::BothGroups,
            VerifierGroupArithmetic::FirstGroupOnly,
        ],
        statement_versions: &[DELIVERY_STATEMENT_VERSION_ONE],
        challenge_tag: SCHNORR_FS_CHALLENGE_TAG,
        weight_tag: SCHNORR_FS_WEIGHT_TAG,
        adapter_version: 1,
        interface_version: DELIVERY_PROOF_INTERFACE_VERSION,
    };

    pub fn try_new(
        params: SchnorrFsDeliveryProofConstructorParams<'a, P, E>,
    ) -> SchnorrFsDeliveryProofTryNewReturn<'a, P, E, F> {
        let challenge_tag = match DomainTag::try_new(DomainTagConstructorParams {
            bytes: SCHNORR_FS_CHALLENGE_TAG.to_vec(),
        }) {
            Ok(tag) => tag,
            Err(error) => {
                return Err(SchnorrFsDeliveryProofTryNewErrorReturn::ChallengeTag(error));
            }
        };
        let weight_tag = match DomainTag::try_new(DomainTagConstructorParams {
            bytes: SCHNORR_FS_WEIGHT_TAG.to_vec(),
        }) {
            Ok(tag) => tag,
            Err(error) => {
                return Err(SchnorrFsDeliveryProofTryNewErrorReturn::WeightTag(error));
            }
        };
        Ok(SchnorrFsDeliveryProof {
            pairing: params.pairing,
            hash_to_scalar: params.hash_to_scalar,
            encoder: params.encoder,
            random: params.random,
            verifier_form: params.verifier_form,
            challenge_tag,
            weight_tag,
            forms: PhantomData,
        })
    }

    fn g1(&self) -> P::G1 {
        let Ok(generator) = self
            .pairing
            .g1_generator(G1GeneratorParams, G1GeneratorPayload);
        generator.point
    }

    fn g2(&self) -> P::G2 {
        let Ok(generator) = self
            .pairing
            .g2_generator(G2GeneratorParams, G2GeneratorPayload);
        generator.point
    }

    fn nonce<NE>(
        &self,
        nonce_draw: impl FnOnce(random::FillBytesErrorReturn) -> NE,
        nonce_sampling: impl FnOnce(SampleUniformScalarErrorReturn) -> NE,
    ) -> Result<Secret<P::Scalar>, NE> {
        let drawn = match self.random.fill_bytes(
            FillBytesParams,
            FillBytesPayload {
                length: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
            },
        ) {
            Ok(drawn) => drawn,
            Err(error) => return Err(nonce_draw(error)),
        };
        match P::Scalar::sample_from_uniform_bytes(
            SampleUniformScalarParams,
            SampleUniformScalarPayload {
                uniform: drawn.bytes,
            },
        ) {
            Ok(sampled) => Ok(sampled.scalar),
            Err(error) => Err(nonce_sampling(error)),
        }
    }

    fn response(
        &self,
        nonce: Secret<P::Scalar>,
        challenge: &P::Scalar,
        witness: &Secret<P::Scalar>,
    ) -> P::Scalar {
        let Ok(product) = self.pairing.mul_scalar(
            MulScalarParams,
            MulScalarPayload {
                left: challenge.clone(),
                right: witness.expose().clone(),
            },
        );
        let Ok(sum) = self.pairing.add_scalar(
            AddScalarParams,
            AddScalarPayload {
                left: nonce.expose().clone(),
                right: product.product,
            },
        );
        sum.sum
    }

    fn scalar_equal(&self, left: &P::Scalar, right: &P::Scalar) -> bool {
        let Ok(left) = self.pairing.encode_scalar(
            EncodeScalarParams,
            EncodeScalarPayload {
                scalar: left.clone(),
            },
        );
        let Ok(right) = self.pairing.encode_scalar(
            EncodeScalarParams,
            EncodeScalarPayload {
                scalar: right.clone(),
            },
        );
        left.bytes.expose().as_ref() == right.bytes.expose().as_ref()
    }

    fn scalar_bytes(&self, scalar: &P::Scalar) -> Vec<u8> {
        let Ok(encoded) = self.pairing.encode_scalar(
            EncodeScalarParams,
            EncodeScalarPayload {
                scalar: scalar.clone(),
            },
        );
        encoded.bytes.expose().as_ref().to_vec()
    }

    fn neg_scalar(&self, scalar: P::Scalar) -> P::Scalar {
        let Ok(negation) = self
            .pairing
            .neg_scalar(NegScalarParams, NegScalarPayload { scalar });
        negation.negation
    }

    fn g1_times(&self, scalar: P::Scalar) -> P::G1 {
        let Ok(product) = self.pairing.mul_g1(
            MulG1Params,
            MulG1Payload {
                point: self.g1(),
                scalar,
            },
        );
        product.product
    }

    fn mint_transcript(
        &self,
        statement: &AlgebraicMintStatement<F, P::G1, P::G2>,
        first_messages: MintFirstMessages<P::G1, P::G2>,
    ) -> MintStatement<P, F> {
        MintStatement {
            suite_identifier: statement.context.suite_identifier.clone(),
            chain: statement.context.chain.clone(),
            entitlement_contract: statement.context.entitlement_contract.clone(),
            asset_identity_hash: statement.context.asset_identity_hash.clone(),
            parameter_set_identifier: statement.context.parameter_set_identifier.clone(),
            source_entitlement: statement.context.source_entitlement.clone(),
            target_entitlement: statement.context.target_entitlement.clone(),
            old_interval: statement.context.old_interval.clone(),
            new_interval: statement.context.new_interval.clone(),
            purpose: statement.context.purpose,
            seller: statement.context.seller.clone(),
            buyer: statement.context.buyer.clone(),
            buyer_keys: PublicKeysComponents {
                pk1: statement.buyer_keys.pk1.clone(),
                pk2: statement.buyer_keys.pk2.clone(),
            },
            envelope: EnvelopeComponents {
                c1: statement.envelope.c1.clone(),
                c2: statement.envelope.c2.clone(),
                d1: statement.envelope.d1.clone(),
                d2: statement.envelope.d2.clone(),
            },
            expiry: statement.context.expiry,
            first_messages,
        }
    }

    fn transfer_transcript(
        &self,
        statement: &AlgebraicTransferStatement<F, P::G1, P::G2>,
        first_messages: TransferFirstMessages<P::G1, P::G2>,
    ) -> TransferStatement<P, F> {
        TransferStatement {
            suite_identifier: statement.context.suite_identifier.clone(),
            chain: statement.context.chain.clone(),
            entitlement_contract: statement.context.entitlement_contract.clone(),
            asset_identity_hash: statement.context.asset_identity_hash.clone(),
            parameter_set_identifier: statement.context.parameter_set_identifier.clone(),
            source_entitlement: statement.context.source_entitlement.clone(),
            target_entitlement: statement.context.target_entitlement.clone(),
            old_interval: statement.context.old_interval.clone(),
            new_interval: statement.context.new_interval.clone(),
            purpose: statement.context.purpose,
            seller: statement.context.seller.clone(),
            buyer: statement.context.buyer.clone(),
            seller_keys: PublicKeysComponents {
                pk1: statement.seller_keys.pk1.clone(),
                pk2: statement.seller_keys.pk2.clone(),
            },
            buyer_keys: PublicKeysComponents {
                pk1: statement.buyer_keys.pk1.clone(),
                pk2: statement.buyer_keys.pk2.clone(),
            },
            old_envelope: EnvelopeComponents {
                c1: statement.old_envelope.c1.clone(),
                c2: statement.old_envelope.c2.clone(),
                d1: statement.old_envelope.d1.clone(),
                d2: statement.old_envelope.d2.clone(),
            },
            new_envelope: EnvelopeComponents {
                c1: statement.new_envelope.c1.clone(),
                c2: statement.new_envelope.c2.clone(),
                d1: statement.new_envelope.d1.clone(),
                d2: statement.new_envelope.d2.clone(),
            },
            expiry: statement.context.expiry,
            first_messages,
        }
    }

    fn challenge_scalar(
        &self,
        transcript: ChallengeTranscript<'_, P, F>,
    ) -> Result<P::Scalar, ChallengeErrorReturn>
    where
        MintStatementFromFieldsErrorReturn<F>: core::fmt::Debug + PartialEq + Eq,
        TransferStatementFromFieldsErrorReturn<F>: core::fmt::Debug + PartialEq + Eq,
    {
        match challenge(
            &ChallengeDeps {
                pairing: self.pairing,
                encoder: self.encoder,
                hash_to_scalar: self.hash_to_scalar,
                tag: &self.challenge_tag,
            },
            ChallengeParams,
            ChallengePayload { transcript },
        ) {
            Ok(challenged) => Ok(challenged.challenge),
            Err(error) => Err(error),
        }
    }
}

impl<'a, P: IPairingArithmetic, E: IEncoderAdapter, F: IChainForms> IDeliveryProofAdapter
    for SchnorrFsDeliveryProof<'a, P, E, F>
where
    MintStatementFromFieldsErrorReturn<F>: core::fmt::Debug + PartialEq + Eq,
    TransferStatementFromFieldsErrorReturn<F>: core::fmt::Debug + PartialEq + Eq,
{
    const DECLARATION: DeliveryProofDeclaration = Self::DECLARATION;

    type Pairing = P;
    type Forms = F;
    type Proof = SchnorrFsProof<P>;

    fn prove_mint(
        &self,
        _params: ProveMintParams,
        payload: ProveMintPayload<'_, F, P::Scalar, P::G1, P::G2>,
    ) -> ProveMintReturn<SchnorrFsProof<P>> {
        let k_alpha = self.nonce(
            |error| {
                ProveMintErrorReturn::SchnorrFs(SchnorrFsProveMintErrorReturn::NonceDraw(error))
            },
            |error| {
                ProveMintErrorReturn::SchnorrFs(SchnorrFsProveMintErrorReturn::NonceSampling(error))
            },
        )?;
        let k_r = self.nonce(
            |error| {
                ProveMintErrorReturn::SchnorrFs(SchnorrFsProveMintErrorReturn::NonceDraw(error))
            },
            |error| {
                ProveMintErrorReturn::SchnorrFs(SchnorrFsProveMintErrorReturn::NonceSampling(error))
            },
        )?;
        let k_rho = self.nonce(
            |error| {
                ProveMintErrorReturn::SchnorrFs(SchnorrFsProveMintErrorReturn::NonceDraw(error))
            },
            |error| {
                ProveMintErrorReturn::SchnorrFs(SchnorrFsProveMintErrorReturn::NonceSampling(error))
            },
        )?;
        let k_sigma = self.nonce(
            |error| {
                ProveMintErrorReturn::SchnorrFs(SchnorrFsProveMintErrorReturn::NonceDraw(error))
            },
            |error| {
                ProveMintErrorReturn::SchnorrFs(SchnorrFsProveMintErrorReturn::NonceSampling(error))
            },
        )?;

        let statement = payload.statement;
        let g1 = self.g1();
        let g2 = self.g2();
        let t_hpub = {
            let Ok(product) = self.pairing.mul_g2(
                MulG2Params,
                MulG2Payload {
                    point: g2.clone(),
                    scalar: k_alpha.expose().clone(),
                },
            );
            product.product
        };
        let t_c1 = {
            let Ok(product) = self.pairing.mul_g1(
                MulG1Params,
                MulG1Payload {
                    point: g1.clone(),
                    scalar: k_rho.expose().clone(),
                },
            );
            product.product
        };
        let t_c2 = {
            let Ok(sum) = self.pairing.msm_g1(
                MsmG1Params,
                MsmG1Payload {
                    terms: vec![
                        MsmG1Term {
                            base: g1.clone(),
                            scalar: k_alpha.expose().clone(),
                        },
                        MsmG1Term {
                            base: statement.identity_element.clone(),
                            scalar: k_r.expose().clone(),
                        },
                        MsmG1Term {
                            base: statement.buyer_keys.pk1.clone(),
                            scalar: k_rho.expose().clone(),
                        },
                    ],
                },
            );
            sum.sum
        };
        let t_d1 = {
            let Ok(product) = self.pairing.mul_g2(
                MulG2Params,
                MulG2Payload {
                    point: g2.clone(),
                    scalar: k_sigma.expose().clone(),
                },
            );
            product.product
        };
        let t_d2 = {
            let Ok(sum) = self.pairing.msm_g2(
                MsmG2Params,
                MsmG2Payload {
                    terms: vec![
                        MsmG2Term {
                            base: g2.clone(),
                            scalar: k_r.expose().clone(),
                        },
                        MsmG2Term {
                            base: statement.buyer_keys.pk2.clone(),
                            scalar: k_sigma.expose().clone(),
                        },
                    ],
                },
            );
            sum.sum
        };

        let transcript = self.mint_transcript(
            statement,
            MintFirstMessages {
                hpub: t_hpub.clone(),
                c1: t_c1,
                c2: t_c2,
                d1: t_d1.clone(),
                d2: t_d2.clone(),
            },
        );
        let c = match self.challenge_scalar(ChallengeTranscript::Mint(&transcript)) {
            Ok(challenge) => challenge,
            Err(error) => {
                return Err(ProveMintErrorReturn::SchnorrFs(
                    SchnorrFsProveMintErrorReturn::Challenge(error),
                ));
            }
        };

        let alpha = self.response(k_alpha, &c, &payload.master_scalar.value);
        let r = self.response(k_r, &c, &payload.credential_randomness);
        let rho = self.response(k_rho, &c, &payload.coins.rho);
        let sigma = self.response(k_sigma, &c, &payload.coins.sigma);

        let proof = match self.verifier_form {
            VerifierGroupArithmetic::BothGroups => SchnorrFsProof::MintBothGroups {
                challenge: c,
                responses: MintResponses {
                    alpha,
                    r,
                    rho,
                    sigma,
                },
            },
            VerifierGroupArithmetic::FirstGroupOnly => SchnorrFsProof::MintFirstGroupOnly {
                challenge: c,
                responses: MintResponses {
                    alpha,
                    r,
                    rho,
                    sigma,
                },
                first_messages: MintSecondGroupFirstMessages {
                    hpub: t_hpub,
                    d1: t_d1,
                    d2: t_d2,
                },
            },
        };
        Ok(ProveMintSuccessReturn { proof })
    }

    fn prove_transfer(
        &self,
        _params: ProveTransferParams,
        payload: ProveTransferPayload<'_, F, P::Scalar, P::G1, P::G2>,
    ) -> ProveTransferReturn<SchnorrFsProof<P>> {
        let k_x = self.nonce(
            |error| {
                ProveTransferErrorReturn::SchnorrFs(SchnorrFsProveTransferErrorReturn::NonceDraw(
                    error,
                ))
            },
            |error| {
                ProveTransferErrorReturn::SchnorrFs(
                    SchnorrFsProveTransferErrorReturn::NonceSampling(error),
                )
            },
        )?;
        let k_y = self.nonce(
            |error| {
                ProveTransferErrorReturn::SchnorrFs(SchnorrFsProveTransferErrorReturn::NonceDraw(
                    error,
                ))
            },
            |error| {
                ProveTransferErrorReturn::SchnorrFs(
                    SchnorrFsProveTransferErrorReturn::NonceSampling(error),
                )
            },
        )?;
        let k_s = self.nonce(
            |error| {
                ProveTransferErrorReturn::SchnorrFs(SchnorrFsProveTransferErrorReturn::NonceDraw(
                    error,
                ))
            },
            |error| {
                ProveTransferErrorReturn::SchnorrFs(
                    SchnorrFsProveTransferErrorReturn::NonceSampling(error),
                )
            },
        )?;
        let k_rho = self.nonce(
            |error| {
                ProveTransferErrorReturn::SchnorrFs(SchnorrFsProveTransferErrorReturn::NonceDraw(
                    error,
                ))
            },
            |error| {
                ProveTransferErrorReturn::SchnorrFs(
                    SchnorrFsProveTransferErrorReturn::NonceSampling(error),
                )
            },
        )?;
        let k_sigma = self.nonce(
            |error| {
                ProveTransferErrorReturn::SchnorrFs(SchnorrFsProveTransferErrorReturn::NonceDraw(
                    error,
                ))
            },
            |error| {
                ProveTransferErrorReturn::SchnorrFs(
                    SchnorrFsProveTransferErrorReturn::NonceSampling(error),
                )
            },
        )?;

        let statement = payload.statement;
        let g1 = self.g1();
        let g2 = self.g2();
        let t_pk1 = {
            let Ok(product) = self.pairing.mul_g1(
                MulG1Params,
                MulG1Payload {
                    point: g1.clone(),
                    scalar: k_x.expose().clone(),
                },
            );
            product.product
        };
        let t_c1 = {
            let Ok(product) = self.pairing.mul_g1(
                MulG1Params,
                MulG1Payload {
                    point: g1.clone(),
                    scalar: k_rho.expose().clone(),
                },
            );
            product.product
        };
        let t_c2 = {
            let Ok(sum) = self.pairing.msm_g1(
                MsmG1Params,
                MsmG1Payload {
                    terms: vec![
                        MsmG1Term {
                            base: statement.old_envelope.c1.clone(),
                            scalar: self.neg_scalar(k_x.expose().clone()),
                        },
                        MsmG1Term {
                            base: statement.identity_element.clone(),
                            scalar: k_s.expose().clone(),
                        },
                        MsmG1Term {
                            base: statement.buyer_keys.pk1.clone(),
                            scalar: k_rho.expose().clone(),
                        },
                    ],
                },
            );
            sum.sum
        };
        let t_pk2 = {
            let Ok(product) = self.pairing.mul_g2(
                MulG2Params,
                MulG2Payload {
                    point: g2.clone(),
                    scalar: k_y.expose().clone(),
                },
            );
            product.product
        };
        let t_d1 = {
            let Ok(product) = self.pairing.mul_g2(
                MulG2Params,
                MulG2Payload {
                    point: g2.clone(),
                    scalar: k_sigma.expose().clone(),
                },
            );
            product.product
        };
        let t_d2 = {
            let Ok(sum) = self.pairing.msm_g2(
                MsmG2Params,
                MsmG2Payload {
                    terms: vec![
                        MsmG2Term {
                            base: statement.old_envelope.d1.clone(),
                            scalar: self.neg_scalar(k_y.expose().clone()),
                        },
                        MsmG2Term {
                            base: g2.clone(),
                            scalar: k_s.expose().clone(),
                        },
                        MsmG2Term {
                            base: statement.buyer_keys.pk2.clone(),
                            scalar: k_sigma.expose().clone(),
                        },
                    ],
                },
            );
            sum.sum
        };

        let transcript = self.transfer_transcript(
            statement,
            TransferFirstMessages {
                pk1: t_pk1,
                c1: t_c1,
                c2: t_c2,
                pk2: t_pk2.clone(),
                d1: t_d1.clone(),
                d2: t_d2.clone(),
            },
        );
        let c = match self.challenge_scalar(ChallengeTranscript::Transfer(&transcript)) {
            Ok(challenge) => challenge,
            Err(error) => {
                return Err(ProveTransferErrorReturn::SchnorrFs(
                    SchnorrFsProveTransferErrorReturn::Challenge(error),
                ));
            }
        };

        let x = self.response(k_x, &c, &payload.seller_secrets.x);
        let y = self.response(k_y, &c, &payload.seller_secrets.y);
        let s = self.response(k_s, &c, &payload.offset);
        let rho = self.response(k_rho, &c, &payload.coins.rho);
        let sigma = self.response(k_sigma, &c, &payload.coins.sigma);

        let proof = match self.verifier_form {
            VerifierGroupArithmetic::BothGroups => SchnorrFsProof::TransferBothGroups {
                challenge: c,
                responses: TransferResponses {
                    x,
                    y,
                    s,
                    rho,
                    sigma,
                },
            },
            VerifierGroupArithmetic::FirstGroupOnly => SchnorrFsProof::TransferFirstGroupOnly {
                challenge: c,
                responses: TransferResponses {
                    x,
                    y,
                    s,
                    rho,
                    sigma,
                },
                first_messages: TransferSecondGroupFirstMessages {
                    pk2: t_pk2,
                    d1: t_d1,
                    d2: t_d2,
                },
            },
        };
        Ok(ProveTransferSuccessReturn { proof })
    }

    fn verify(
        &self,
        _params: VerifyParams,
        payload: VerifyPayload<'_, F, P::G1, P::G2, SchnorrFsProof<P>>,
    ) -> VerifyReturn {
        match payload.statement {
            AlgebraicStatement::Mint(statement) => match payload.proof {
                SchnorrFsProof::MintBothGroups {
                    challenge,
                    responses,
                } => {
                    if !matches!(self.verifier_form, VerifierGroupArithmetic::BothGroups) {
                        return Ok(VerifySuccessReturn { is_valid: false });
                    }
                    let n = self.neg_scalar(challenge.clone());
                    let g1 = self.g1();
                    let g2 = self.g2();
                    let t_hpub = {
                        let Ok(sum) = self.pairing.msm_g2(
                            MsmG2Params,
                            MsmG2Payload {
                                terms: vec![
                                    MsmG2Term {
                                        base: g2.clone(),
                                        scalar: responses.alpha.clone(),
                                    },
                                    MsmG2Term {
                                        base: statement.hpub.clone(),
                                        scalar: n.clone(),
                                    },
                                ],
                            },
                        );
                        sum.sum
                    };
                    let t_c1 = {
                        let Ok(sum) = self.pairing.msm_g1(
                            MsmG1Params,
                            MsmG1Payload {
                                terms: vec![
                                    MsmG1Term {
                                        base: g1.clone(),
                                        scalar: responses.rho.clone(),
                                    },
                                    MsmG1Term {
                                        base: statement.envelope.c1.clone(),
                                        scalar: n.clone(),
                                    },
                                ],
                            },
                        );
                        sum.sum
                    };
                    let t_c2 = {
                        let Ok(sum) = self.pairing.msm_g1(
                            MsmG1Params,
                            MsmG1Payload {
                                terms: vec![
                                    MsmG1Term {
                                        base: g1.clone(),
                                        scalar: responses.alpha.clone(),
                                    },
                                    MsmG1Term {
                                        base: statement.identity_element.clone(),
                                        scalar: responses.r.clone(),
                                    },
                                    MsmG1Term {
                                        base: statement.buyer_keys.pk1.clone(),
                                        scalar: responses.rho.clone(),
                                    },
                                    MsmG1Term {
                                        base: statement.envelope.c2.clone(),
                                        scalar: n.clone(),
                                    },
                                ],
                            },
                        );
                        sum.sum
                    };
                    let t_d1 = {
                        let Ok(sum) = self.pairing.msm_g2(
                            MsmG2Params,
                            MsmG2Payload {
                                terms: vec![
                                    MsmG2Term {
                                        base: g2.clone(),
                                        scalar: responses.sigma.clone(),
                                    },
                                    MsmG2Term {
                                        base: statement.envelope.d1.clone(),
                                        scalar: n.clone(),
                                    },
                                ],
                            },
                        );
                        sum.sum
                    };
                    let t_d2 = {
                        let Ok(sum) = self.pairing.msm_g2(
                            MsmG2Params,
                            MsmG2Payload {
                                terms: vec![
                                    MsmG2Term {
                                        base: g2.clone(),
                                        scalar: responses.r.clone(),
                                    },
                                    MsmG2Term {
                                        base: statement.buyer_keys.pk2.clone(),
                                        scalar: responses.sigma.clone(),
                                    },
                                    MsmG2Term {
                                        base: statement.envelope.d2.clone(),
                                        scalar: n,
                                    },
                                ],
                            },
                        );
                        sum.sum
                    };
                    let transcript = self.mint_transcript(
                        statement,
                        MintFirstMessages {
                            hpub: t_hpub,
                            c1: t_c1,
                            c2: t_c2,
                            d1: t_d1,
                            d2: t_d2,
                        },
                    );
                    let c_prime =
                        match self.challenge_scalar(ChallengeTranscript::Mint(&transcript)) {
                            Ok(challenge) => challenge,
                            Err(error) => {
                                return Err(VerifyErrorReturn::SchnorrFs(
                                    SchnorrFsVerifyErrorReturn::Challenge(error),
                                ));
                            }
                        };
                    Ok(VerifySuccessReturn {
                        is_valid: self.scalar_equal(&c_prime, challenge),
                    })
                }
                SchnorrFsProof::MintFirstGroupOnly {
                    challenge,
                    responses,
                    first_messages,
                } => {
                    if !matches!(self.verifier_form, VerifierGroupArithmetic::FirstGroupOnly) {
                        return Ok(VerifySuccessReturn { is_valid: false });
                    }
                    self.verify_mint_first_group_only(
                        statement,
                        challenge,
                        responses,
                        first_messages,
                    )
                }
                _ => Ok(VerifySuccessReturn { is_valid: false }),
            },
            AlgebraicStatement::Transfer(statement) => match payload.proof {
                SchnorrFsProof::TransferBothGroups {
                    challenge,
                    responses,
                } => {
                    if !matches!(self.verifier_form, VerifierGroupArithmetic::BothGroups) {
                        return Ok(VerifySuccessReturn { is_valid: false });
                    }
                    let n = self.neg_scalar(challenge.clone());
                    let g1 = self.g1();
                    let g2 = self.g2();
                    let neg_c2_old = {
                        let Ok(negation) = self.pairing.neg_g1(
                            NegG1Params,
                            NegG1Payload {
                                point: statement.old_envelope.c2.clone(),
                            },
                        );
                        negation.negation
                    };
                    let delta_c2 = {
                        let Ok(sum) = self.pairing.add_g1(
                            AddG1Params,
                            AddG1Payload {
                                left: statement.new_envelope.c2.clone(),
                                right: neg_c2_old,
                            },
                        );
                        sum.sum
                    };
                    let neg_d2_old = {
                        let Ok(negation) = self.pairing.neg_g2(
                            NegG2Params,
                            NegG2Payload {
                                point: statement.old_envelope.d2.clone(),
                            },
                        );
                        negation.negation
                    };
                    let delta_d2 = {
                        let Ok(sum) = self.pairing.add_g2(
                            AddG2Params,
                            AddG2Payload {
                                left: statement.new_envelope.d2.clone(),
                                right: neg_d2_old,
                            },
                        );
                        sum.sum
                    };
                    let t_pk1 = {
                        let Ok(sum) = self.pairing.msm_g1(
                            MsmG1Params,
                            MsmG1Payload {
                                terms: vec![
                                    MsmG1Term {
                                        base: g1.clone(),
                                        scalar: responses.x.clone(),
                                    },
                                    MsmG1Term {
                                        base: statement.seller_keys.pk1.clone(),
                                        scalar: n.clone(),
                                    },
                                ],
                            },
                        );
                        sum.sum
                    };
                    let t_c1 = {
                        let Ok(sum) = self.pairing.msm_g1(
                            MsmG1Params,
                            MsmG1Payload {
                                terms: vec![
                                    MsmG1Term {
                                        base: g1.clone(),
                                        scalar: responses.rho.clone(),
                                    },
                                    MsmG1Term {
                                        base: statement.new_envelope.c1.clone(),
                                        scalar: n.clone(),
                                    },
                                ],
                            },
                        );
                        sum.sum
                    };
                    let t_c2 = {
                        let Ok(sum) = self.pairing.msm_g1(
                            MsmG1Params,
                            MsmG1Payload {
                                terms: vec![
                                    MsmG1Term {
                                        base: statement.old_envelope.c1.clone(),
                                        scalar: self.neg_scalar(responses.x.clone()),
                                    },
                                    MsmG1Term {
                                        base: statement.identity_element.clone(),
                                        scalar: responses.s.clone(),
                                    },
                                    MsmG1Term {
                                        base: statement.buyer_keys.pk1.clone(),
                                        scalar: responses.rho.clone(),
                                    },
                                    MsmG1Term {
                                        base: delta_c2,
                                        scalar: n.clone(),
                                    },
                                ],
                            },
                        );
                        sum.sum
                    };
                    let t_pk2 = {
                        let Ok(sum) = self.pairing.msm_g2(
                            MsmG2Params,
                            MsmG2Payload {
                                terms: vec![
                                    MsmG2Term {
                                        base: g2.clone(),
                                        scalar: responses.y.clone(),
                                    },
                                    MsmG2Term {
                                        base: statement.seller_keys.pk2.clone(),
                                        scalar: n.clone(),
                                    },
                                ],
                            },
                        );
                        sum.sum
                    };
                    let t_d1 = {
                        let Ok(sum) = self.pairing.msm_g2(
                            MsmG2Params,
                            MsmG2Payload {
                                terms: vec![
                                    MsmG2Term {
                                        base: g2.clone(),
                                        scalar: responses.sigma.clone(),
                                    },
                                    MsmG2Term {
                                        base: statement.new_envelope.d1.clone(),
                                        scalar: n.clone(),
                                    },
                                ],
                            },
                        );
                        sum.sum
                    };
                    let t_d2 = {
                        let Ok(sum) = self.pairing.msm_g2(
                            MsmG2Params,
                            MsmG2Payload {
                                terms: vec![
                                    MsmG2Term {
                                        base: statement.old_envelope.d1.clone(),
                                        scalar: self.neg_scalar(responses.y.clone()),
                                    },
                                    MsmG2Term {
                                        base: g2.clone(),
                                        scalar: responses.s.clone(),
                                    },
                                    MsmG2Term {
                                        base: statement.buyer_keys.pk2.clone(),
                                        scalar: responses.sigma.clone(),
                                    },
                                    MsmG2Term {
                                        base: delta_d2,
                                        scalar: n,
                                    },
                                ],
                            },
                        );
                        sum.sum
                    };
                    let transcript = self.transfer_transcript(
                        statement,
                        TransferFirstMessages {
                            pk1: t_pk1,
                            c1: t_c1,
                            c2: t_c2,
                            pk2: t_pk2,
                            d1: t_d1,
                            d2: t_d2,
                        },
                    );
                    let c_prime =
                        match self.challenge_scalar(ChallengeTranscript::Transfer(&transcript)) {
                            Ok(challenge) => challenge,
                            Err(error) => {
                                return Err(VerifyErrorReturn::SchnorrFs(
                                    SchnorrFsVerifyErrorReturn::Challenge(error),
                                ));
                            }
                        };
                    Ok(VerifySuccessReturn {
                        is_valid: self.scalar_equal(&c_prime, challenge),
                    })
                }
                SchnorrFsProof::TransferFirstGroupOnly {
                    challenge,
                    responses,
                    first_messages,
                } => {
                    if !matches!(self.verifier_form, VerifierGroupArithmetic::FirstGroupOnly) {
                        return Ok(VerifySuccessReturn { is_valid: false });
                    }
                    self.verify_transfer_first_group_only(
                        statement,
                        challenge,
                        responses,
                        first_messages,
                    )
                }
                _ => Ok(VerifySuccessReturn { is_valid: false }),
            },
        }
    }

    fn proof_components(
        &self,
        _params: ProofComponentsParams,
        payload: ProofComponentsPayload<'_, SchnorrFsProof<P>>,
    ) -> ProofComponentsReturn<P::Scalar, P::G2> {
        let components = match payload.proof {
            SchnorrFsProof::MintBothGroups {
                challenge,
                responses,
            } => DeliveryProofComponents::MintBothGroups {
                challenge: challenge.clone(),
                responses: MintResponses {
                    alpha: responses.alpha.clone(),
                    r: responses.r.clone(),
                    rho: responses.rho.clone(),
                    sigma: responses.sigma.clone(),
                },
            },
            SchnorrFsProof::MintFirstGroupOnly {
                challenge,
                responses,
                first_messages,
            } => DeliveryProofComponents::MintFirstGroupOnly {
                challenge: challenge.clone(),
                responses: MintResponses {
                    alpha: responses.alpha.clone(),
                    r: responses.r.clone(),
                    rho: responses.rho.clone(),
                    sigma: responses.sigma.clone(),
                },
                first_messages: MintSecondGroupFirstMessages {
                    hpub: first_messages.hpub.clone(),
                    d1: first_messages.d1.clone(),
                    d2: first_messages.d2.clone(),
                },
            },
            SchnorrFsProof::TransferBothGroups {
                challenge,
                responses,
            } => DeliveryProofComponents::TransferBothGroups {
                challenge: challenge.clone(),
                responses: TransferResponses {
                    x: responses.x.clone(),
                    y: responses.y.clone(),
                    s: responses.s.clone(),
                    rho: responses.rho.clone(),
                    sigma: responses.sigma.clone(),
                },
            },
            SchnorrFsProof::TransferFirstGroupOnly {
                challenge,
                responses,
                first_messages,
            } => DeliveryProofComponents::TransferFirstGroupOnly {
                challenge: challenge.clone(),
                responses: TransferResponses {
                    x: responses.x.clone(),
                    y: responses.y.clone(),
                    s: responses.s.clone(),
                    rho: responses.rho.clone(),
                    sigma: responses.sigma.clone(),
                },
                first_messages: TransferSecondGroupFirstMessages {
                    pk2: first_messages.pk2.clone(),
                    d1: first_messages.d1.clone(),
                    d2: first_messages.d2.clone(),
                },
            },
        };
        Ok(ProofComponentsSuccessReturn { components })
    }

    fn proof_from_components(
        &self,
        _params: ProofFromComponentsParams,
        payload: ProofFromComponentsPayload<P::Scalar, P::G2>,
    ) -> ProofFromComponentsReturn<SchnorrFsProof<P>> {
        match payload.components {
            DeliveryProofComponents::MintBothGroups {
                challenge,
                responses,
            } => {
                if !matches!(self.verifier_form, VerifierGroupArithmetic::BothGroups) {
                    return Err(ProofFromComponentsErrorReturn::SchnorrFs(
                        SchnorrFsProofFromComponentsErrorReturn::VerifierFormMismatch,
                    ));
                }
                Ok(ProofFromComponentsSuccessReturn {
                    proof: SchnorrFsProof::MintBothGroups {
                        challenge,
                        responses,
                    },
                })
            }
            DeliveryProofComponents::MintFirstGroupOnly {
                challenge,
                responses,
                first_messages,
            } => {
                if !matches!(self.verifier_form, VerifierGroupArithmetic::FirstGroupOnly) {
                    return Err(ProofFromComponentsErrorReturn::SchnorrFs(
                        SchnorrFsProofFromComponentsErrorReturn::VerifierFormMismatch,
                    ));
                }
                Ok(ProofFromComponentsSuccessReturn {
                    proof: SchnorrFsProof::MintFirstGroupOnly {
                        challenge,
                        responses,
                        first_messages,
                    },
                })
            }
            DeliveryProofComponents::TransferBothGroups {
                challenge,
                responses,
            } => {
                if !matches!(self.verifier_form, VerifierGroupArithmetic::BothGroups) {
                    return Err(ProofFromComponentsErrorReturn::SchnorrFs(
                        SchnorrFsProofFromComponentsErrorReturn::VerifierFormMismatch,
                    ));
                }
                Ok(ProofFromComponentsSuccessReturn {
                    proof: SchnorrFsProof::TransferBothGroups {
                        challenge,
                        responses,
                    },
                })
            }
            DeliveryProofComponents::TransferFirstGroupOnly {
                challenge,
                responses,
                first_messages,
            } => {
                if !matches!(self.verifier_form, VerifierGroupArithmetic::FirstGroupOnly) {
                    return Err(ProofFromComponentsErrorReturn::SchnorrFs(
                        SchnorrFsProofFromComponentsErrorReturn::VerifierFormMismatch,
                    ));
                }
                Ok(ProofFromComponentsSuccessReturn {
                    proof: SchnorrFsProof::TransferFirstGroupOnly {
                        challenge,
                        responses,
                        first_messages,
                    },
                })
            }
        }
    }
}

impl<'a, P: IPairingArithmetic, E: IEncoderAdapter, F: IChainForms>
    SchnorrFsDeliveryProof<'a, P, E, F>
where
    MintStatementFromFieldsErrorReturn<F>: core::fmt::Debug + PartialEq + Eq,
    TransferStatementFromFieldsErrorReturn<F>: core::fmt::Debug + PartialEq + Eq,
{
    fn weight(&self, base_message: &[u8], position: u8) -> Result<P::Scalar, VerifyErrorReturn> {
        let mut message = base_message.to_vec();
        message.push(position);
        match self.hash_to_scalar.hash_to_scalar(
            HashToScalarParams {
                tag: &self.weight_tag,
            },
            HashToScalarPayload { message: &message },
        ) {
            Ok(hashed) => Ok(hashed.scalar),
            Err(error) => Err(VerifyErrorReturn::SchnorrFs(
                SchnorrFsVerifyErrorReturn::Weight(error),
            )),
        }
    }

    fn verify_mint_first_group_only(
        &self,
        statement: &AlgebraicMintStatement<F, P::G1, P::G2>,
        challenge: &P::Scalar,
        responses: &MintResponses<P::Scalar>,
        first_messages: &MintSecondGroupFirstMessages<P::G2>,
    ) -> VerifyReturn {
        let n = self.neg_scalar(challenge.clone());
        let g1 = self.g1();
        let g2 = self.g2();
        let t_c1 = {
            let Ok(sum) = self.pairing.msm_g1(
                MsmG1Params,
                MsmG1Payload {
                    terms: vec![
                        MsmG1Term {
                            base: g1.clone(),
                            scalar: responses.rho.clone(),
                        },
                        MsmG1Term {
                            base: statement.envelope.c1.clone(),
                            scalar: n.clone(),
                        },
                    ],
                },
            );
            sum.sum
        };
        let t_c2 = {
            let Ok(sum) = self.pairing.msm_g1(
                MsmG1Params,
                MsmG1Payload {
                    terms: vec![
                        MsmG1Term {
                            base: g1.clone(),
                            scalar: responses.alpha.clone(),
                        },
                        MsmG1Term {
                            base: statement.identity_element.clone(),
                            scalar: responses.r.clone(),
                        },
                        MsmG1Term {
                            base: statement.buyer_keys.pk1.clone(),
                            scalar: responses.rho.clone(),
                        },
                        MsmG1Term {
                            base: statement.envelope.c2.clone(),
                            scalar: n.clone(),
                        },
                    ],
                },
            );
            sum.sum
        };
        let transcript = self.mint_transcript(
            statement,
            MintFirstMessages {
                hpub: first_messages.hpub.clone(),
                c1: t_c1,
                c2: t_c2,
                d1: first_messages.d1.clone(),
                d2: first_messages.d2.clone(),
            },
        );
        let c_prime = match self.challenge_scalar(ChallengeTranscript::Mint(&transcript)) {
            Ok(challenge) => challenge,
            Err(error) => {
                return Err(VerifyErrorReturn::SchnorrFs(
                    SchnorrFsVerifyErrorReturn::Challenge(error),
                ));
            }
        };
        if !self.scalar_equal(&c_prime, challenge) {
            return Ok(VerifySuccessReturn { is_valid: false });
        }

        let mut base_message = self.scalar_bytes(challenge);
        for response in [
            &responses.alpha,
            &responses.r,
            &responses.rho,
            &responses.sigma,
        ] {
            base_message.extend_from_slice(&self.scalar_bytes(response));
        }
        let w2 = self.weight(&base_message, 0x01)?;
        let w3 = self.weight(&base_message, 0x02)?;

        let neg_g1 = {
            let Ok(negation) = self
                .pairing
                .neg_g1(NegG1Params, NegG1Payload { point: g1.clone() });
            negation.negation
        };
        let Ok(sum) = self.pairing.add_scalar(
            AddScalarParams,
            AddScalarPayload {
                left: responses.alpha.clone(),
                right: {
                    let Ok(product) = self.pairing.mul_scalar(
                        MulScalarParams,
                        MulScalarPayload {
                            left: w2.clone(),
                            right: responses.sigma.clone(),
                        },
                    );
                    product.product
                },
            },
        );
        let combined = {
            let Ok(product) = self.pairing.mul_scalar(
                MulScalarParams,
                MulScalarPayload {
                    left: w3.clone(),
                    right: responses.r.clone(),
                },
            );
            let Ok(sum) = self.pairing.add_scalar(
                AddScalarParams,
                AddScalarPayload {
                    left: sum.sum,
                    right: product.product,
                },
            );
            sum.sum
        };
        let w2_n = {
            let Ok(product) = self.pairing.mul_scalar(
                MulScalarParams,
                MulScalarPayload {
                    left: w2.clone(),
                    right: n.clone(),
                },
            );
            product.product
        };
        let w3_sigma = {
            let Ok(product) = self.pairing.mul_scalar(
                MulScalarParams,
                MulScalarPayload {
                    left: w3.clone(),
                    right: responses.sigma.clone(),
                },
            );
            product.product
        };
        let w3_n = {
            let Ok(product) = self.pairing.mul_scalar(
                MulScalarParams,
                MulScalarPayload {
                    left: w3.clone(),
                    right: n.clone(),
                },
            );
            product.product
        };
        let Ok(checked) = self.pairing.pairing_product_is_one(
            PairingProductIsOneParams,
            PairingProductIsOnePayload {
                terms: vec![
                    PairingProductTerm {
                        g1: neg_g1,
                        g2: first_messages.hpub.clone(),
                    },
                    PairingProductTerm {
                        g1: self.g1_times(self.neg_scalar(w2)),
                        g2: first_messages.d1.clone(),
                    },
                    PairingProductTerm {
                        g1: self.g1_times(self.neg_scalar(w3)),
                        g2: first_messages.d2.clone(),
                    },
                    PairingProductTerm {
                        g1: self.g1_times(combined),
                        g2,
                    },
                    PairingProductTerm {
                        g1: self.g1_times(n),
                        g2: statement.hpub.clone(),
                    },
                    PairingProductTerm {
                        g1: self.g1_times(w2_n),
                        g2: statement.envelope.d1.clone(),
                    },
                    PairingProductTerm {
                        g1: self.g1_times(w3_sigma),
                        g2: statement.buyer_keys.pk2.clone(),
                    },
                    PairingProductTerm {
                        g1: self.g1_times(w3_n),
                        g2: statement.envelope.d2.clone(),
                    },
                ],
            },
        );
        Ok(VerifySuccessReturn {
            is_valid: checked.is_one,
        })
    }

    fn verify_transfer_first_group_only(
        &self,
        statement: &AlgebraicTransferStatement<F, P::G1, P::G2>,
        challenge: &P::Scalar,
        responses: &TransferResponses<P::Scalar>,
        first_messages: &TransferSecondGroupFirstMessages<P::G2>,
    ) -> VerifyReturn {
        let n = self.neg_scalar(challenge.clone());
        let g1 = self.g1();
        let g2 = self.g2();
        let neg_c2_old = {
            let Ok(negation) = self.pairing.neg_g1(
                NegG1Params,
                NegG1Payload {
                    point: statement.old_envelope.c2.clone(),
                },
            );
            negation.negation
        };
        let delta_c2 = {
            let Ok(sum) = self.pairing.add_g1(
                AddG1Params,
                AddG1Payload {
                    left: statement.new_envelope.c2.clone(),
                    right: neg_c2_old,
                },
            );
            sum.sum
        };
        let t_pk1 = {
            let Ok(sum) = self.pairing.msm_g1(
                MsmG1Params,
                MsmG1Payload {
                    terms: vec![
                        MsmG1Term {
                            base: g1.clone(),
                            scalar: responses.x.clone(),
                        },
                        MsmG1Term {
                            base: statement.seller_keys.pk1.clone(),
                            scalar: n.clone(),
                        },
                    ],
                },
            );
            sum.sum
        };
        let t_c1 = {
            let Ok(sum) = self.pairing.msm_g1(
                MsmG1Params,
                MsmG1Payload {
                    terms: vec![
                        MsmG1Term {
                            base: g1.clone(),
                            scalar: responses.rho.clone(),
                        },
                        MsmG1Term {
                            base: statement.new_envelope.c1.clone(),
                            scalar: n.clone(),
                        },
                    ],
                },
            );
            sum.sum
        };
        let t_c2 = {
            let Ok(sum) = self.pairing.msm_g1(
                MsmG1Params,
                MsmG1Payload {
                    terms: vec![
                        MsmG1Term {
                            base: statement.old_envelope.c1.clone(),
                            scalar: self.neg_scalar(responses.x.clone()),
                        },
                        MsmG1Term {
                            base: statement.identity_element.clone(),
                            scalar: responses.s.clone(),
                        },
                        MsmG1Term {
                            base: statement.buyer_keys.pk1.clone(),
                            scalar: responses.rho.clone(),
                        },
                        MsmG1Term {
                            base: delta_c2,
                            scalar: n.clone(),
                        },
                    ],
                },
            );
            sum.sum
        };
        let transcript = self.transfer_transcript(
            statement,
            TransferFirstMessages {
                pk1: t_pk1,
                c1: t_c1,
                c2: t_c2,
                pk2: first_messages.pk2.clone(),
                d1: first_messages.d1.clone(),
                d2: first_messages.d2.clone(),
            },
        );
        let c_prime = match self.challenge_scalar(ChallengeTranscript::Transfer(&transcript)) {
            Ok(challenge) => challenge,
            Err(error) => {
                return Err(VerifyErrorReturn::SchnorrFs(
                    SchnorrFsVerifyErrorReturn::Challenge(error),
                ));
            }
        };
        if !self.scalar_equal(&c_prime, challenge) {
            return Ok(VerifySuccessReturn { is_valid: false });
        }

        let mut base_message = self.scalar_bytes(challenge);
        for response in [
            &responses.x,
            &responses.y,
            &responses.s,
            &responses.rho,
            &responses.sigma,
        ] {
            base_message.extend_from_slice(&self.scalar_bytes(response));
        }
        let w2 = self.weight(&base_message, 0x01)?;
        let w3 = self.weight(&base_message, 0x02)?;

        let neg_g1 = {
            let Ok(negation) = self
                .pairing
                .neg_g1(NegG1Params, NegG1Payload { point: g1.clone() });
            negation.negation
        };
        let combined = {
            let Ok(w2_sigma) = self.pairing.mul_scalar(
                MulScalarParams,
                MulScalarPayload {
                    left: w2.clone(),
                    right: responses.sigma.clone(),
                },
            );
            let Ok(w3_s) = self.pairing.mul_scalar(
                MulScalarParams,
                MulScalarPayload {
                    left: w3.clone(),
                    right: responses.s.clone(),
                },
            );
            let Ok(y_plus) = self.pairing.add_scalar(
                AddScalarParams,
                AddScalarPayload {
                    left: responses.y.clone(),
                    right: w2_sigma.product,
                },
            );
            let Ok(sum) = self.pairing.add_scalar(
                AddScalarParams,
                AddScalarPayload {
                    left: y_plus.sum,
                    right: w3_s.product,
                },
            );
            sum.sum
        };
        let w2_n = {
            let Ok(product) = self.pairing.mul_scalar(
                MulScalarParams,
                MulScalarPayload {
                    left: w2.clone(),
                    right: n.clone(),
                },
            );
            product.product
        };
        let neg_w3_y = {
            let Ok(product) = self.pairing.mul_scalar(
                MulScalarParams,
                MulScalarPayload {
                    left: w3.clone(),
                    right: responses.y.clone(),
                },
            );
            self.neg_scalar(product.product)
        };
        let w3_sigma = {
            let Ok(product) = self.pairing.mul_scalar(
                MulScalarParams,
                MulScalarPayload {
                    left: w3.clone(),
                    right: responses.sigma.clone(),
                },
            );
            product.product
        };
        let w3_n = {
            let Ok(product) = self.pairing.mul_scalar(
                MulScalarParams,
                MulScalarPayload {
                    left: w3.clone(),
                    right: n.clone(),
                },
            );
            product.product
        };
        let w3_c = {
            let Ok(product) = self.pairing.mul_scalar(
                MulScalarParams,
                MulScalarPayload {
                    left: w3.clone(),
                    right: challenge.clone(),
                },
            );
            product.product
        };
        let Ok(checked) = self.pairing.pairing_product_is_one(
            PairingProductIsOneParams,
            PairingProductIsOnePayload {
                terms: vec![
                    PairingProductTerm {
                        g1: neg_g1,
                        g2: first_messages.pk2.clone(),
                    },
                    PairingProductTerm {
                        g1: self.g1_times(self.neg_scalar(w2)),
                        g2: first_messages.d1.clone(),
                    },
                    PairingProductTerm {
                        g1: self.g1_times(self.neg_scalar(w3)),
                        g2: first_messages.d2.clone(),
                    },
                    PairingProductTerm {
                        g1: self.g1_times(combined),
                        g2,
                    },
                    PairingProductTerm {
                        g1: self.g1_times(n),
                        g2: statement.seller_keys.pk2.clone(),
                    },
                    PairingProductTerm {
                        g1: self.g1_times(w2_n),
                        g2: statement.new_envelope.d1.clone(),
                    },
                    PairingProductTerm {
                        g1: self.g1_times(neg_w3_y),
                        g2: statement.old_envelope.d1.clone(),
                    },
                    PairingProductTerm {
                        g1: self.g1_times(w3_sigma),
                        g2: statement.buyer_keys.pk2.clone(),
                    },
                    PairingProductTerm {
                        g1: self.g1_times(w3_n),
                        g2: statement.new_envelope.d2.clone(),
                    },
                    PairingProductTerm {
                        g1: self.g1_times(w3_c),
                        g2: statement.old_envelope.d2.clone(),
                    },
                ],
            },
        );
        Ok(VerifySuccessReturn {
            is_valid: checked.is_one,
        })
    }
}
