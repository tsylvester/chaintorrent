mod challenge;
#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(test)]
mod mock;
pub(crate) mod provides;
#[cfg(test)]
mod test;

use core::marker::PhantomData;

use chain::IChainForms;
use domain::Secret;
use encoding::IEncoderAdapter;
use envelope::{EnvelopeAlgebra, EnvelopeComponents, PublicKeysComponents};
use hash_to_scalar::{
    DomainTag, DomainTagConstructorParams, HashToScalarParams, HashToScalarPayload,
};
use pairing::{
    AddG1Params, AddG1Payload, AddG2Params, AddG2Payload, AddScalarParams, AddScalarPayload,
    EncodeScalarParams, EncodeScalarPayload, G1GeneratorParams, G1GeneratorPayload,
    G2GeneratorParams, G2GeneratorPayload, IPairingArithmetic, ISampleUniformScalar, MsmG1Params,
    MsmG1Payload, MsmG1Term, MsmG2Params, MsmG2Payload, MsmG2Term, MulG1Params, MulG1Payload,
    MulG2Params, MulG2Payload, MulScalarParams, MulScalarPayload, NegG1Params, NegG1Payload,
    NegG2Params, NegG2Payload, NegScalarParams, NegScalarPayload, PairingProductIsOneParams,
    PairingProductIsOnePayload, PairingProductTerm, SampleUniformScalarParams,
    SampleUniformScalarPayload, VerifierGroupArithmetic,
};
use random::{FillBytesParams, FillBytesPayload, IRandomSourceAdapter};

use self::challenge::provides::{
    ChallengeDeps, ChallengeParams, ChallengePayload, ChallengeTranscript,
    SCHNORR_FS_CHALLENGE_TAG, challenge,
};
use self::interface::{
    SCHNORR_FS_WEIGHT_TAG, SchnorrFsDeliveryProof, SchnorrFsDeliveryProofConstructorParams,
    SchnorrFsDeliveryProofTryNewErrorReturn, SchnorrFsDeliveryProofTryNewReturn, SchnorrFsProof,
    SchnorrFsProveMintErrorReturn, SchnorrFsProveTransferErrorReturn, SchnorrFsVerifyErrorReturn,
};
use crate::factory::provides::{
    AlgebraicStatement, DELIVERY_PROOF_INTERFACE_VERSION, DeliveryProofComponents,
    DeliveryProofDeclaration, IDeliveryProofAdapter, MintResponses, MintSecondGroupFirstMessages,
    ProofComponentsParams, ProofComponentsPayload, ProofComponentsReturn,
    ProofComponentsSuccessReturn, ProofFromComponentsErrorReturn, ProofFromComponentsParams,
    ProofFromComponentsPayload, ProofFromComponentsReturn, ProofFromComponentsSuccessReturn,
    ProveMintErrorReturn, ProveMintParams, ProveMintPayload, ProveMintReturn,
    ProveMintSuccessReturn, ProveTransferErrorReturn, ProveTransferParams, ProveTransferPayload,
    ProveTransferReturn, ProveTransferSuccessReturn, TransferResponses,
    TransferSecondGroupFirstMessages, VerifyErrorReturn, VerifyParams, VerifyPayload, VerifyReturn,
    VerifySuccessReturn,
};
use crate::mint_statement::provides::{
    DELIVERY_STATEMENT_VERSION_ONE, MintFirstMessages, MintStatement,
};
use crate::transfer_statement::provides::{TransferFirstMessages, TransferStatement};

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
}

impl<'a, P: IPairingArithmetic, E: IEncoderAdapter, F: IChainForms> IDeliveryProofAdapter
    for SchnorrFsDeliveryProof<'a, P, E, F>
{
    const DECLARATION: DeliveryProofDeclaration = SchnorrFsDeliveryProof::<P, E, F>::DECLARATION;

    type Pairing = P;

    type Forms = F;

    type Proof = SchnorrFsProof<P>;

    fn prove_mint(
        &self,
        _params: ProveMintParams,
        payload: ProveMintPayload<'_, F, P::Scalar, P::G1, P::G2>,
    ) -> ProveMintReturn<SchnorrFsProof<P>> {
        let draw = |random: &dyn IRandomSourceAdapter| {
            let filled = match random.fill_bytes(
                FillBytesParams,
                FillBytesPayload {
                    length: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                },
            ) {
                Ok(success) => success,
                Err(error) => {
                    return Err(SchnorrFsProveMintErrorReturn::NonceDraw(error));
                }
            };
            let uniform: Secret<Vec<u8>> = filled.bytes;
            match P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload { uniform },
            ) {
                Ok(success) => Ok(success.scalar.expose().clone()),
                Err(error) => Err(SchnorrFsProveMintErrorReturn::NonceSampling(error)),
            }
        };
        let nonce_alpha = match draw(self.random) {
            Ok(nonce) => nonce,
            Err(error) => return Err(ProveMintErrorReturn::SchnorrFs(error)),
        };
        let nonce_r = match draw(self.random) {
            Ok(nonce) => nonce,
            Err(error) => return Err(ProveMintErrorReturn::SchnorrFs(error)),
        };
        let nonce_rho = match draw(self.random) {
            Ok(nonce) => nonce,
            Err(error) => return Err(ProveMintErrorReturn::SchnorrFs(error)),
        };
        let nonce_sigma = match draw(self.random) {
            Ok(nonce) => nonce,
            Err(error) => return Err(ProveMintErrorReturn::SchnorrFs(error)),
        };

        let statement = payload.statement;
        let Ok(g1) = self
            .pairing
            .g1_generator(G1GeneratorParams, G1GeneratorPayload);
        let Ok(g2) = self
            .pairing
            .g2_generator(G2GeneratorParams, G2GeneratorPayload);
        let t_hpub = {
            let Ok(success) = self.pairing.mul_g2(
                MulG2Params,
                MulG2Payload {
                    point: g2.point.clone(),
                    scalar: nonce_alpha.clone(),
                },
            );
            success.product
        };
        let t_c1 = {
            let Ok(success) = self.pairing.mul_g1(
                MulG1Params,
                MulG1Payload {
                    point: g1.point.clone(),
                    scalar: nonce_rho.clone(),
                },
            );
            success.product
        };
        let t_c2 = {
            let Ok(success) = self.pairing.msm_g1(
                MsmG1Params,
                MsmG1Payload {
                    terms: vec![
                        MsmG1Term {
                            base: g1.point.clone(),
                            scalar: nonce_alpha.clone(),
                        },
                        MsmG1Term {
                            base: statement.identity_element.clone(),
                            scalar: nonce_r.clone(),
                        },
                        MsmG1Term {
                            base: statement.buyer_keys.pk1.clone(),
                            scalar: nonce_rho.clone(),
                        },
                    ],
                },
            );
            success.sum
        };
        let t_d1 = {
            let Ok(success) = self.pairing.mul_g2(
                MulG2Params,
                MulG2Payload {
                    point: g2.point.clone(),
                    scalar: nonce_sigma.clone(),
                },
            );
            success.product
        };
        let t_d2 = {
            let Ok(success) = self.pairing.msm_g2(
                MsmG2Params,
                MsmG2Payload {
                    terms: vec![
                        MsmG2Term {
                            base: g2.point.clone(),
                            scalar: nonce_r.clone(),
                        },
                        MsmG2Term {
                            base: statement.buyer_keys.pk2.clone(),
                            scalar: nonce_sigma.clone(),
                        },
                    ],
                },
            );
            success.sum
        };

        let context = &statement.context;
        let transcript: MintStatement<P, F> = MintStatement {
            suite_identifier: context.suite_identifier.clone(),
            chain: context.chain.clone(),
            entitlement_contract: context.entitlement_contract.clone(),
            asset_root: context.asset_root,
            parameter_set_identifier: context.parameter_set_identifier.clone(),
            source_entitlement: context.source_entitlement.clone(),
            target_entitlement: context.target_entitlement.clone(),
            old_interval: context.old_interval.clone(),
            new_interval: context.new_interval.clone(),
            purpose: context.purpose,
            seller: context.seller.clone(),
            buyer: context.buyer.clone(),
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
            expiry: context.expiry,
            first_messages: MintFirstMessages {
                hpub: t_hpub.clone(),
                c1: t_c1,
                c2: t_c2,
                d1: t_d1.clone(),
                d2: t_d2.clone(),
            },
        };
        let challenge_scalar = match challenge(
            &ChallengeDeps {
                pairing: self.pairing,
                encoder: self.encoder,
                tag: &self.challenge_tag,
                hash_to_scalar: self.hash_to_scalar,
            },
            ChallengeParams,
            ChallengePayload {
                transcript: ChallengeTranscript::Mint(&transcript),
            },
        ) {
            Ok(success) => success.challenge,
            Err(error) => {
                return Err(ProveMintErrorReturn::SchnorrFs(
                    SchnorrFsProveMintErrorReturn::Challenge(error),
                ));
            }
        };

        let respond = |nonce: P::Scalar, witness: P::Scalar| -> P::Scalar {
            let Ok(product) = self.pairing.mul_scalar(
                MulScalarParams,
                MulScalarPayload {
                    left: challenge_scalar.clone(),
                    right: witness,
                },
            );
            let Ok(sum) = self.pairing.add_scalar(
                AddScalarParams,
                AddScalarPayload {
                    left: nonce,
                    right: product.product,
                },
            );
            sum.sum
        };
        let responses = MintResponses {
            alpha: respond(nonce_alpha, payload.master_scalar.value.expose().clone()),
            r: respond(nonce_r, payload.credential_randomness.expose().clone()),
            rho: respond(nonce_rho, payload.coins.rho.expose().clone()),
            sigma: respond(nonce_sigma, payload.coins.sigma.expose().clone()),
        };

        let proof = match self.verifier_form {
            VerifierGroupArithmetic::BothGroups => SchnorrFsProof::MintBothGroups {
                challenge: challenge_scalar,
                responses,
            },
            VerifierGroupArithmetic::FirstGroupOnly => SchnorrFsProof::MintFirstGroupOnly {
                challenge: challenge_scalar,
                responses,
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
        let draw = |random: &dyn IRandomSourceAdapter| {
            let filled = match random.fill_bytes(
                FillBytesParams,
                FillBytesPayload {
                    length: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                },
            ) {
                Ok(success) => success,
                Err(error) => {
                    return Err(SchnorrFsProveTransferErrorReturn::NonceDraw(error));
                }
            };
            let uniform: Secret<Vec<u8>> = filled.bytes;
            match P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload { uniform },
            ) {
                Ok(success) => Ok(success.scalar.expose().clone()),
                Err(error) => Err(SchnorrFsProveTransferErrorReturn::NonceSampling(error)),
            }
        };
        let nonce_x = match draw(self.random) {
            Ok(nonce) => nonce,
            Err(error) => return Err(ProveTransferErrorReturn::SchnorrFs(error)),
        };
        let nonce_y = match draw(self.random) {
            Ok(nonce) => nonce,
            Err(error) => return Err(ProveTransferErrorReturn::SchnorrFs(error)),
        };
        let nonce_s = match draw(self.random) {
            Ok(nonce) => nonce,
            Err(error) => return Err(ProveTransferErrorReturn::SchnorrFs(error)),
        };
        let nonce_rho = match draw(self.random) {
            Ok(nonce) => nonce,
            Err(error) => return Err(ProveTransferErrorReturn::SchnorrFs(error)),
        };
        let nonce_sigma = match draw(self.random) {
            Ok(nonce) => nonce,
            Err(error) => return Err(ProveTransferErrorReturn::SchnorrFs(error)),
        };

        let statement = payload.statement;
        let Ok(g1) = self
            .pairing
            .g1_generator(G1GeneratorParams, G1GeneratorPayload);
        let Ok(g2) = self
            .pairing
            .g2_generator(G2GeneratorParams, G2GeneratorPayload);
        let neg = |scalar: P::Scalar| -> P::Scalar {
            let Ok(success) = self
                .pairing
                .neg_scalar(NegScalarParams, NegScalarPayload { scalar });
            success.negation
        };
        let t_pk1 = {
            let Ok(success) = self.pairing.mul_g1(
                MulG1Params,
                MulG1Payload {
                    point: g1.point.clone(),
                    scalar: nonce_x.clone(),
                },
            );
            success.product
        };
        let t_c1 = {
            let Ok(success) = self.pairing.mul_g1(
                MulG1Params,
                MulG1Payload {
                    point: g1.point.clone(),
                    scalar: nonce_rho.clone(),
                },
            );
            success.product
        };
        let t_c2 = {
            let Ok(success) = self.pairing.msm_g1(
                MsmG1Params,
                MsmG1Payload {
                    terms: vec![
                        MsmG1Term {
                            base: statement.old_envelope.c1.clone(),
                            scalar: neg(nonce_x.clone()),
                        },
                        MsmG1Term {
                            base: statement.identity_element.clone(),
                            scalar: nonce_s.clone(),
                        },
                        MsmG1Term {
                            base: statement.buyer_keys.pk1.clone(),
                            scalar: nonce_rho.clone(),
                        },
                    ],
                },
            );
            success.sum
        };
        let t_pk2 = {
            let Ok(success) = self.pairing.mul_g2(
                MulG2Params,
                MulG2Payload {
                    point: g2.point.clone(),
                    scalar: nonce_y.clone(),
                },
            );
            success.product
        };
        let t_d1 = {
            let Ok(success) = self.pairing.mul_g2(
                MulG2Params,
                MulG2Payload {
                    point: g2.point.clone(),
                    scalar: nonce_sigma.clone(),
                },
            );
            success.product
        };
        let t_d2 = {
            let Ok(success) = self.pairing.msm_g2(
                MsmG2Params,
                MsmG2Payload {
                    terms: vec![
                        MsmG2Term {
                            base: statement.old_envelope.d1.clone(),
                            scalar: neg(nonce_y.clone()),
                        },
                        MsmG2Term {
                            base: g2.point.clone(),
                            scalar: nonce_s.clone(),
                        },
                        MsmG2Term {
                            base: statement.buyer_keys.pk2.clone(),
                            scalar: nonce_sigma.clone(),
                        },
                    ],
                },
            );
            success.sum
        };

        let context = &statement.context;
        let transcript: TransferStatement<P, F> = TransferStatement {
            suite_identifier: context.suite_identifier.clone(),
            chain: context.chain.clone(),
            entitlement_contract: context.entitlement_contract.clone(),
            asset_root: context.asset_root,
            parameter_set_identifier: context.parameter_set_identifier.clone(),
            source_entitlement: context.source_entitlement.clone(),
            target_entitlement: context.target_entitlement.clone(),
            old_interval: context.old_interval.clone(),
            new_interval: context.new_interval.clone(),
            purpose: context.purpose,
            seller: context.seller.clone(),
            buyer: context.buyer.clone(),
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
            expiry: context.expiry,
            first_messages: TransferFirstMessages {
                pk1: t_pk1,
                c1: t_c1,
                c2: t_c2,
                pk2: t_pk2.clone(),
                d1: t_d1.clone(),
                d2: t_d2.clone(),
            },
        };
        let challenge_scalar = match challenge(
            &ChallengeDeps {
                pairing: self.pairing,
                encoder: self.encoder,
                tag: &self.challenge_tag,
                hash_to_scalar: self.hash_to_scalar,
            },
            ChallengeParams,
            ChallengePayload {
                transcript: ChallengeTranscript::Transfer(&transcript),
            },
        ) {
            Ok(success) => success.challenge,
            Err(error) => {
                return Err(ProveTransferErrorReturn::SchnorrFs(
                    SchnorrFsProveTransferErrorReturn::Challenge(error),
                ));
            }
        };

        let respond = |nonce: P::Scalar, witness: P::Scalar| -> P::Scalar {
            let Ok(product) = self.pairing.mul_scalar(
                MulScalarParams,
                MulScalarPayload {
                    left: challenge_scalar.clone(),
                    right: witness,
                },
            );
            let Ok(sum) = self.pairing.add_scalar(
                AddScalarParams,
                AddScalarPayload {
                    left: nonce,
                    right: product.product,
                },
            );
            sum.sum
        };
        let responses = TransferResponses {
            x: respond(nonce_x, payload.seller_secrets.x.expose().clone()),
            y: respond(nonce_y, payload.seller_secrets.y.expose().clone()),
            s: respond(nonce_s, payload.offset.expose().clone()),
            rho: respond(nonce_rho, payload.coins.rho.expose().clone()),
            sigma: respond(nonce_sigma, payload.coins.sigma.expose().clone()),
        };

        let proof = match self.verifier_form {
            VerifierGroupArithmetic::BothGroups => SchnorrFsProof::TransferBothGroups {
                challenge: challenge_scalar,
                responses,
            },
            VerifierGroupArithmetic::FirstGroupOnly => SchnorrFsProof::TransferFirstGroupOnly {
                challenge: challenge_scalar,
                responses,
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
        let neg_scalar = |scalar: P::Scalar| -> P::Scalar {
            let Ok(success) = self
                .pairing
                .neg_scalar(NegScalarParams, NegScalarPayload { scalar });
            success.negation
        };
        let add_scalar = |left: P::Scalar, right: P::Scalar| -> P::Scalar {
            let Ok(success) = self
                .pairing
                .add_scalar(AddScalarParams, AddScalarPayload { left, right });
            success.sum
        };
        let mul_scalar = |left: P::Scalar, right: P::Scalar| -> P::Scalar {
            let Ok(success) = self
                .pairing
                .mul_scalar(MulScalarParams, MulScalarPayload { left, right });
            success.product
        };
        let neg_g1 = |point: P::G1| -> P::G1 {
            let Ok(success) = self.pairing.neg_g1(NegG1Params, NegG1Payload { point });
            success.negation
        };
        let mul_g1 = |point: P::G1, scalar: P::Scalar| -> P::G1 {
            let Ok(success) = self
                .pairing
                .mul_g1(MulG1Params, MulG1Payload { point, scalar });
            success.product
        };
        let msm_g1 = |terms: Vec<(P::G1, P::Scalar)>| -> P::G1 {
            let Ok(success) = self.pairing.msm_g1(
                MsmG1Params,
                MsmG1Payload {
                    terms: terms
                        .into_iter()
                        .map(|(base, scalar)| MsmG1Term { base, scalar })
                        .collect(),
                },
            );
            success.sum
        };
        let msm_g2 = |terms: Vec<(P::G2, P::Scalar)>| -> P::G2 {
            let Ok(success) = self.pairing.msm_g2(
                MsmG2Params,
                MsmG2Payload {
                    terms: terms
                        .into_iter()
                        .map(|(base, scalar)| MsmG2Term { base, scalar })
                        .collect(),
                },
            );
            success.sum
        };
        let encode_scalar = |scalar: P::Scalar| -> Vec<u8> {
            let Ok(success) = self
                .pairing
                .encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar });
            success.bytes.expose().as_ref().to_vec()
        };
        let scalars_equal = |left: P::Scalar, right: P::Scalar| -> bool {
            encode_scalar(left) == encode_scalar(right)
        };
        let Ok(g1) = self
            .pairing
            .g1_generator(G1GeneratorParams, G1GeneratorPayload);
        let Ok(g2) = self
            .pairing
            .g2_generator(G2GeneratorParams, G2GeneratorPayload);

        match (payload.statement, self.verifier_form, payload.proof) {
            (
                AlgebraicStatement::Mint(statement),
                VerifierGroupArithmetic::BothGroups,
                SchnorrFsProof::MintBothGroups {
                    challenge: c,
                    responses: z,
                },
            ) => {
                let n = neg_scalar(c.clone());
                let t_hpub = msm_g2(vec![
                    (g2.point.clone(), z.alpha.clone()),
                    (statement.hpub.clone(), n.clone()),
                ]);
                let t_c1 = msm_g1(vec![
                    (g1.point.clone(), z.rho.clone()),
                    (statement.envelope.c1.clone(), n.clone()),
                ]);
                let t_c2 = msm_g1(vec![
                    (g1.point.clone(), z.alpha.clone()),
                    (statement.identity_element.clone(), z.r.clone()),
                    (statement.buyer_keys.pk1.clone(), z.rho.clone()),
                    (statement.envelope.c2.clone(), n.clone()),
                ]);
                let t_d1 = msm_g2(vec![
                    (g2.point.clone(), z.sigma.clone()),
                    (statement.envelope.d1.clone(), n.clone()),
                ]);
                let t_d2 = msm_g2(vec![
                    (g2.point.clone(), z.r.clone()),
                    (statement.buyer_keys.pk2.clone(), z.sigma.clone()),
                    (statement.envelope.d2.clone(), n.clone()),
                ]);
                let context = &statement.context;
                let transcript: MintStatement<P, F> = MintStatement {
                    suite_identifier: context.suite_identifier.clone(),
                    chain: context.chain.clone(),
                    entitlement_contract: context.entitlement_contract.clone(),
                    asset_root: context.asset_root,
                    parameter_set_identifier: context.parameter_set_identifier.clone(),
                    source_entitlement: context.source_entitlement.clone(),
                    target_entitlement: context.target_entitlement.clone(),
                    old_interval: context.old_interval.clone(),
                    new_interval: context.new_interval.clone(),
                    purpose: context.purpose,
                    seller: context.seller.clone(),
                    buyer: context.buyer.clone(),
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
                    expiry: context.expiry,
                    first_messages: MintFirstMessages {
                        hpub: t_hpub,
                        c1: t_c1,
                        c2: t_c2,
                        d1: t_d1,
                        d2: t_d2,
                    },
                };
                let recomputed = match challenge(
                    &ChallengeDeps {
                        pairing: self.pairing,
                        encoder: self.encoder,
                        tag: &self.challenge_tag,
                        hash_to_scalar: self.hash_to_scalar,
                    },
                    ChallengeParams,
                    ChallengePayload {
                        transcript: ChallengeTranscript::Mint(&transcript),
                    },
                ) {
                    Ok(success) => success.challenge,
                    Err(error) => {
                        return Err(VerifyErrorReturn::SchnorrFs(
                            SchnorrFsVerifyErrorReturn::Challenge(error),
                        ));
                    }
                };
                Ok(VerifySuccessReturn {
                    is_valid: scalars_equal(recomputed, c.clone()),
                })
            }
            (
                AlgebraicStatement::Transfer(statement),
                VerifierGroupArithmetic::BothGroups,
                SchnorrFsProof::TransferBothGroups {
                    challenge: c,
                    responses: z,
                },
            ) => {
                let n = neg_scalar(c.clone());
                let delta_c2 = {
                    let Ok(negated) = self.pairing.neg_g1(
                        NegG1Params,
                        NegG1Payload {
                            point: statement.old_envelope.c2.clone(),
                        },
                    );
                    let Ok(sum) = self.pairing.add_g1(
                        AddG1Params,
                        AddG1Payload {
                            left: statement.new_envelope.c2.clone(),
                            right: negated.negation,
                        },
                    );
                    sum.sum
                };
                let delta_d2 = {
                    let Ok(negated) = self.pairing.neg_g2(
                        NegG2Params,
                        NegG2Payload {
                            point: statement.old_envelope.d2.clone(),
                        },
                    );
                    let Ok(sum) = self.pairing.add_g2(
                        AddG2Params,
                        AddG2Payload {
                            left: statement.new_envelope.d2.clone(),
                            right: negated.negation,
                        },
                    );
                    sum.sum
                };
                let t_pk1 = msm_g1(vec![
                    (g1.point.clone(), z.x.clone()),
                    (statement.seller_keys.pk1.clone(), n.clone()),
                ]);
                let t_c1 = msm_g1(vec![
                    (g1.point.clone(), z.rho.clone()),
                    (statement.new_envelope.c1.clone(), n.clone()),
                ]);
                let t_c2 = msm_g1(vec![
                    (statement.old_envelope.c1.clone(), neg_scalar(z.x.clone())),
                    (statement.identity_element.clone(), z.s.clone()),
                    (statement.buyer_keys.pk1.clone(), z.rho.clone()),
                    (delta_c2, n.clone()),
                ]);
                let t_pk2 = msm_g2(vec![
                    (g2.point.clone(), z.y.clone()),
                    (statement.seller_keys.pk2.clone(), n.clone()),
                ]);
                let t_d1 = msm_g2(vec![
                    (g2.point.clone(), z.sigma.clone()),
                    (statement.new_envelope.d1.clone(), n.clone()),
                ]);
                let t_d2 = msm_g2(vec![
                    (statement.old_envelope.d1.clone(), neg_scalar(z.y.clone())),
                    (g2.point.clone(), z.s.clone()),
                    (statement.buyer_keys.pk2.clone(), z.sigma.clone()),
                    (delta_d2, n.clone()),
                ]);
                let context = &statement.context;
                let transcript: TransferStatement<P, F> = TransferStatement {
                    suite_identifier: context.suite_identifier.clone(),
                    chain: context.chain.clone(),
                    entitlement_contract: context.entitlement_contract.clone(),
                    asset_root: context.asset_root,
                    parameter_set_identifier: context.parameter_set_identifier.clone(),
                    source_entitlement: context.source_entitlement.clone(),
                    target_entitlement: context.target_entitlement.clone(),
                    old_interval: context.old_interval.clone(),
                    new_interval: context.new_interval.clone(),
                    purpose: context.purpose,
                    seller: context.seller.clone(),
                    buyer: context.buyer.clone(),
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
                    expiry: context.expiry,
                    first_messages: TransferFirstMessages {
                        pk1: t_pk1,
                        c1: t_c1,
                        c2: t_c2,
                        pk2: t_pk2,
                        d1: t_d1,
                        d2: t_d2,
                    },
                };
                let recomputed = match challenge(
                    &ChallengeDeps {
                        pairing: self.pairing,
                        encoder: self.encoder,
                        tag: &self.challenge_tag,
                        hash_to_scalar: self.hash_to_scalar,
                    },
                    ChallengeParams,
                    ChallengePayload {
                        transcript: ChallengeTranscript::Transfer(&transcript),
                    },
                ) {
                    Ok(success) => success.challenge,
                    Err(error) => {
                        return Err(VerifyErrorReturn::SchnorrFs(
                            SchnorrFsVerifyErrorReturn::Challenge(error),
                        ));
                    }
                };
                Ok(VerifySuccessReturn {
                    is_valid: scalars_equal(recomputed, c.clone()),
                })
            }
            (
                AlgebraicStatement::Mint(statement),
                VerifierGroupArithmetic::FirstGroupOnly,
                SchnorrFsProof::MintFirstGroupOnly {
                    challenge: c,
                    responses: z,
                    first_messages: carried,
                },
            ) => {
                let n = neg_scalar(c.clone());
                let t_c1 = msm_g1(vec![
                    (g1.point.clone(), z.rho.clone()),
                    (statement.envelope.c1.clone(), n.clone()),
                ]);
                let t_c2 = msm_g1(vec![
                    (g1.point.clone(), z.alpha.clone()),
                    (statement.identity_element.clone(), z.r.clone()),
                    (statement.buyer_keys.pk1.clone(), z.rho.clone()),
                    (statement.envelope.c2.clone(), n.clone()),
                ]);
                let context = &statement.context;
                let transcript: MintStatement<P, F> = MintStatement {
                    suite_identifier: context.suite_identifier.clone(),
                    chain: context.chain.clone(),
                    entitlement_contract: context.entitlement_contract.clone(),
                    asset_root: context.asset_root,
                    parameter_set_identifier: context.parameter_set_identifier.clone(),
                    source_entitlement: context.source_entitlement.clone(),
                    target_entitlement: context.target_entitlement.clone(),
                    old_interval: context.old_interval.clone(),
                    new_interval: context.new_interval.clone(),
                    purpose: context.purpose,
                    seller: context.seller.clone(),
                    buyer: context.buyer.clone(),
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
                    expiry: context.expiry,
                    first_messages: MintFirstMessages {
                        hpub: carried.hpub.clone(),
                        c1: t_c1,
                        c2: t_c2,
                        d1: carried.d1.clone(),
                        d2: carried.d2.clone(),
                    },
                };
                let recomputed = match challenge(
                    &ChallengeDeps {
                        pairing: self.pairing,
                        encoder: self.encoder,
                        tag: &self.challenge_tag,
                        hash_to_scalar: self.hash_to_scalar,
                    },
                    ChallengeParams,
                    ChallengePayload {
                        transcript: ChallengeTranscript::Mint(&transcript),
                    },
                ) {
                    Ok(success) => success.challenge,
                    Err(error) => {
                        return Err(VerifyErrorReturn::SchnorrFs(
                            SchnorrFsVerifyErrorReturn::Challenge(error),
                        ));
                    }
                };
                if !scalars_equal(recomputed, c.clone()) {
                    return Ok(VerifySuccessReturn { is_valid: false });
                }
                let mut base = encode_scalar(c.clone());
                for response in [z.alpha.clone(), z.r.clone(), z.rho.clone(), z.sigma.clone()] {
                    base.extend(encode_scalar(response));
                }
                let mut message_two = base.clone();
                message_two.push(0x01u8);
                let w2 = match self.hash_to_scalar.hash_to_scalar(
                    HashToScalarParams {
                        tag: &self.weight_tag,
                    },
                    HashToScalarPayload {
                        message: &message_two,
                    },
                ) {
                    Ok(success) => success.scalar,
                    Err(error) => {
                        return Err(VerifyErrorReturn::SchnorrFs(
                            SchnorrFsVerifyErrorReturn::Weight(error),
                        ));
                    }
                };
                let mut message_three = base;
                message_three.push(0x02u8);
                let w3 = match self.hash_to_scalar.hash_to_scalar(
                    HashToScalarParams {
                        tag: &self.weight_tag,
                    },
                    HashToScalarPayload {
                        message: &message_three,
                    },
                ) {
                    Ok(success) => success.scalar,
                    Err(error) => {
                        return Err(VerifyErrorReturn::SchnorrFs(
                            SchnorrFsVerifyErrorReturn::Weight(error),
                        ));
                    }
                };
                let combined = add_scalar(
                    z.alpha.clone(),
                    add_scalar(
                        mul_scalar(w2.clone(), z.sigma.clone()),
                        mul_scalar(w3.clone(), z.r.clone()),
                    ),
                );
                let terms = vec![
                    PairingProductTerm {
                        g1: neg_g1(g1.point.clone()),
                        g2: carried.hpub.clone(),
                    },
                    PairingProductTerm {
                        g1: mul_g1(g1.point.clone(), neg_scalar(w2.clone())),
                        g2: carried.d1.clone(),
                    },
                    PairingProductTerm {
                        g1: mul_g1(g1.point.clone(), neg_scalar(w3.clone())),
                        g2: carried.d2.clone(),
                    },
                    PairingProductTerm {
                        g1: mul_g1(g1.point.clone(), combined),
                        g2: g2.point.clone(),
                    },
                    PairingProductTerm {
                        g1: mul_g1(g1.point.clone(), n.clone()),
                        g2: statement.hpub.clone(),
                    },
                    PairingProductTerm {
                        g1: mul_g1(g1.point.clone(), mul_scalar(w2.clone(), n.clone())),
                        g2: statement.envelope.d1.clone(),
                    },
                    PairingProductTerm {
                        g1: mul_g1(g1.point.clone(), mul_scalar(w3.clone(), z.sigma.clone())),
                        g2: statement.buyer_keys.pk2.clone(),
                    },
                    PairingProductTerm {
                        g1: mul_g1(g1.point.clone(), mul_scalar(w3.clone(), n.clone())),
                        g2: statement.envelope.d2.clone(),
                    },
                ];
                let Ok(product) = self.pairing.pairing_product_is_one(
                    PairingProductIsOneParams,
                    PairingProductIsOnePayload { terms },
                );
                Ok(VerifySuccessReturn {
                    is_valid: product.is_one,
                })
            }
            (
                AlgebraicStatement::Transfer(statement),
                VerifierGroupArithmetic::FirstGroupOnly,
                SchnorrFsProof::TransferFirstGroupOnly {
                    challenge: c,
                    responses: z,
                    first_messages: carried,
                },
            ) => {
                let n = neg_scalar(c.clone());
                let delta_c2 = {
                    let Ok(negated) = self.pairing.neg_g1(
                        NegG1Params,
                        NegG1Payload {
                            point: statement.old_envelope.c2.clone(),
                        },
                    );
                    let Ok(sum) = self.pairing.add_g1(
                        AddG1Params,
                        AddG1Payload {
                            left: statement.new_envelope.c2.clone(),
                            right: negated.negation,
                        },
                    );
                    sum.sum
                };
                let t_pk1 = msm_g1(vec![
                    (g1.point.clone(), z.x.clone()),
                    (statement.seller_keys.pk1.clone(), n.clone()),
                ]);
                let t_c1 = msm_g1(vec![
                    (g1.point.clone(), z.rho.clone()),
                    (statement.new_envelope.c1.clone(), n.clone()),
                ]);
                let t_c2 = msm_g1(vec![
                    (statement.old_envelope.c1.clone(), neg_scalar(z.x.clone())),
                    (statement.identity_element.clone(), z.s.clone()),
                    (statement.buyer_keys.pk1.clone(), z.rho.clone()),
                    (delta_c2, n.clone()),
                ]);
                let context = &statement.context;
                let transcript: TransferStatement<P, F> = TransferStatement {
                    suite_identifier: context.suite_identifier.clone(),
                    chain: context.chain.clone(),
                    entitlement_contract: context.entitlement_contract.clone(),
                    asset_root: context.asset_root,
                    parameter_set_identifier: context.parameter_set_identifier.clone(),
                    source_entitlement: context.source_entitlement.clone(),
                    target_entitlement: context.target_entitlement.clone(),
                    old_interval: context.old_interval.clone(),
                    new_interval: context.new_interval.clone(),
                    purpose: context.purpose,
                    seller: context.seller.clone(),
                    buyer: context.buyer.clone(),
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
                    expiry: context.expiry,
                    first_messages: TransferFirstMessages {
                        pk1: t_pk1,
                        c1: t_c1,
                        c2: t_c2,
                        pk2: carried.pk2.clone(),
                        d1: carried.d1.clone(),
                        d2: carried.d2.clone(),
                    },
                };
                let recomputed = match challenge(
                    &ChallengeDeps {
                        pairing: self.pairing,
                        encoder: self.encoder,
                        tag: &self.challenge_tag,
                        hash_to_scalar: self.hash_to_scalar,
                    },
                    ChallengeParams,
                    ChallengePayload {
                        transcript: ChallengeTranscript::Transfer(&transcript),
                    },
                ) {
                    Ok(success) => success.challenge,
                    Err(error) => {
                        return Err(VerifyErrorReturn::SchnorrFs(
                            SchnorrFsVerifyErrorReturn::Challenge(error),
                        ));
                    }
                };
                if !scalars_equal(recomputed, c.clone()) {
                    return Ok(VerifySuccessReturn { is_valid: false });
                }
                let mut base = encode_scalar(c.clone());
                for response in [
                    z.x.clone(),
                    z.y.clone(),
                    z.s.clone(),
                    z.rho.clone(),
                    z.sigma.clone(),
                ] {
                    base.extend(encode_scalar(response));
                }
                let mut message_two = base.clone();
                message_two.push(0x01u8);
                let w2 = match self.hash_to_scalar.hash_to_scalar(
                    HashToScalarParams {
                        tag: &self.weight_tag,
                    },
                    HashToScalarPayload {
                        message: &message_two,
                    },
                ) {
                    Ok(success) => success.scalar,
                    Err(error) => {
                        return Err(VerifyErrorReturn::SchnorrFs(
                            SchnorrFsVerifyErrorReturn::Weight(error),
                        ));
                    }
                };
                let mut message_three = base;
                message_three.push(0x02u8);
                let w3 = match self.hash_to_scalar.hash_to_scalar(
                    HashToScalarParams {
                        tag: &self.weight_tag,
                    },
                    HashToScalarPayload {
                        message: &message_three,
                    },
                ) {
                    Ok(success) => success.scalar,
                    Err(error) => {
                        return Err(VerifyErrorReturn::SchnorrFs(
                            SchnorrFsVerifyErrorReturn::Weight(error),
                        ));
                    }
                };
                let combined = add_scalar(
                    z.y.clone(),
                    add_scalar(
                        mul_scalar(w2.clone(), z.sigma.clone()),
                        mul_scalar(w3.clone(), z.s.clone()),
                    ),
                );
                let terms = vec![
                    PairingProductTerm {
                        g1: neg_g1(g1.point.clone()),
                        g2: carried.pk2.clone(),
                    },
                    PairingProductTerm {
                        g1: mul_g1(g1.point.clone(), neg_scalar(w2.clone())),
                        g2: carried.d1.clone(),
                    },
                    PairingProductTerm {
                        g1: mul_g1(g1.point.clone(), neg_scalar(w3.clone())),
                        g2: carried.d2.clone(),
                    },
                    PairingProductTerm {
                        g1: mul_g1(g1.point.clone(), combined),
                        g2: g2.point.clone(),
                    },
                    PairingProductTerm {
                        g1: mul_g1(g1.point.clone(), n.clone()),
                        g2: statement.seller_keys.pk2.clone(),
                    },
                    PairingProductTerm {
                        g1: mul_g1(g1.point.clone(), mul_scalar(w2.clone(), n.clone())),
                        g2: statement.new_envelope.d1.clone(),
                    },
                    PairingProductTerm {
                        g1: mul_g1(
                            g1.point.clone(),
                            neg_scalar(mul_scalar(w3.clone(), z.y.clone())),
                        ),
                        g2: statement.old_envelope.d1.clone(),
                    },
                    PairingProductTerm {
                        g1: mul_g1(g1.point.clone(), mul_scalar(w3.clone(), z.sigma.clone())),
                        g2: statement.buyer_keys.pk2.clone(),
                    },
                    PairingProductTerm {
                        g1: mul_g1(g1.point.clone(), mul_scalar(w3.clone(), n.clone())),
                        g2: statement.new_envelope.d2.clone(),
                    },
                    PairingProductTerm {
                        g1: mul_g1(g1.point.clone(), mul_scalar(w3.clone(), c.clone())),
                        g2: statement.old_envelope.d2.clone(),
                    },
                ];
                let Ok(product) = self.pairing.pairing_product_is_one(
                    PairingProductIsOneParams,
                    PairingProductIsOnePayload { terms },
                );
                Ok(VerifySuccessReturn {
                    is_valid: product.is_one,
                })
            }
            _ => Ok(VerifySuccessReturn { is_valid: false }),
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
        let form_matches = match (&payload.components, &self.verifier_form) {
            (
                DeliveryProofComponents::MintBothGroups { .. }
                | DeliveryProofComponents::TransferBothGroups { .. },
                VerifierGroupArithmetic::BothGroups,
            ) => true,
            (
                DeliveryProofComponents::MintFirstGroupOnly { .. }
                | DeliveryProofComponents::TransferFirstGroupOnly { .. },
                VerifierGroupArithmetic::FirstGroupOnly,
            ) => true,
            _ => false,
        };
        if !form_matches {
            return Err(ProofFromComponentsErrorReturn::VerifierFormMismatch);
        }
        let proof = match payload.components {
            DeliveryProofComponents::MintBothGroups {
                challenge,
                responses,
            } => SchnorrFsProof::MintBothGroups {
                challenge,
                responses,
            },
            DeliveryProofComponents::MintFirstGroupOnly {
                challenge,
                responses,
                first_messages,
            } => SchnorrFsProof::MintFirstGroupOnly {
                challenge,
                responses,
                first_messages,
            },
            DeliveryProofComponents::TransferBothGroups {
                challenge,
                responses,
            } => SchnorrFsProof::TransferBothGroups {
                challenge,
                responses,
            },
            DeliveryProofComponents::TransferFirstGroupOnly {
                challenge,
                responses,
                first_messages,
            } => SchnorrFsProof::TransferFirstGroupOnly {
                challenge,
                responses,
                first_messages,
            },
        };
        Ok(ProofFromComponentsSuccessReturn { proof })
    }
}
