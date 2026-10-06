#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use chain::MockIChainForms;
use core::marker::PhantomData;
use domain::{Secret, SecretConstructorParamsOverrides, build_secret};
use encoding::{
    CanonicalFieldValue, ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingDeps,
    CreateEncodingPayload, CreateEncodingReturn, FromFieldsParams, IDecoderAdapter,
    IEncoderAdapter, IEncodingConsumer, ToFieldsParams, build_create_encoding_params,
    create_encoding,
};
use envelope::{
    EnvelopeAlgebra, EnvelopeCoins, EnvelopeComponents, KeyPairComponents, PublicKeysComponents,
};
use hash_to_scalar::{
    CreateHashToScalarDeps, CreateHashToScalarPayload, DomainTag, DomainTagConstructorParams,
    IHashToScalarAdapter, build_create_hash_to_scalar_params, create_hash_to_scalar,
};
use kem::MasterScalarComponents;
use pairing::{
    AddG1Params, AddG1Payload, AddG2Params, AddG2Payload, AddScalarParams, AddScalarPayload,
    ConsumePairingParams, ConsumePairingPayload, CreatePairingDeps, CreatePairingParamsOverrides,
    CreatePairingPayload, EncodeG2Params, EncodeG2Payload, EncodeScalarParams, EncodeScalarPayload,
    G1GeneratorParams, G1GeneratorPayload, G2GeneratorParams, G2GeneratorPayload, IPairingAdapter,
    IPairingArithmetic, IPairingConsumer, IPairingReference, ISampleUniformScalar, MsmG1Params,
    MsmG1Payload, MsmG1Term, MsmG2Params, MsmG2Payload, MsmG2Term, MulG1Params, MulG1Payload,
    MulG2Params, MulG2Payload, MulScalarParams, MulScalarPayload, NegG2Params, NegG2Payload,
    NegScalarParams, NegScalarPayload, PairingConcrete, SampleUniformScalarParams,
    SampleUniformScalarPayload, VerifierGroupArithmetic, build_create_pairing_params,
    create_pairing,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourcePayload, IRandomSourceAdapter,
    build_create_random_source_params, create_random_source,
};

use crate::factory::provides::{
    AlgebraicMintStatement, AlgebraicStatement, AlgebraicTransferStatement,
    DELIVERY_PROOF_INTERFACE_VERSION, DeliveryContext, DeliveryContextOverrides,
    DeliveryProofComponents, DeliveryProofShapeErrorReturn, DeliveryProofWireComponents,
    IDeliveryProofAdapter, MintResponses, MintSecondGroupFirstMessages, ProofComponentsParams,
    ProofComponentsPayload, ProofFromComponentsErrorReturn, ProofFromComponentsParams,
    ProofFromComponentsPayload, ProveMintParams, ProveMintPayload, ProveTransferParams,
    ProveTransferPayload, TransferResponses, TransferSecondGroupFirstMessages, VerifyParams,
    VerifyPayload, build_delivery_context,
};
use crate::mint_statement::provides::{
    DELIVERY_PURPOSE_GRANT, DELIVERY_PURPOSE_MINT, DELIVERY_PURPOSE_REPLACEMENT,
    DELIVERY_PURPOSE_TRANSFER, DELIVERY_STATEMENT_VERSION_ONE, DeliveryRelation, MintFirstMessages,
    MintPurpose, MintStatement, MintStatementDescription,
    MintStatementDescriptionConstructorParams, MintStatementFromFieldsErrorReturn, TransferPurpose,
};
use crate::transfer_statement::provides::{
    TransferFirstMessages, TransferStatement, TransferStatementDescription,
    TransferStatementDescriptionConstructorParams, TransferStatementFromFieldsErrorReturn,
};

use super::challenge::provides::{
    ChallengeDeps, ChallengeParams, ChallengePayload, ChallengeTranscript, challenge,
};
use super::interface::{
    SCHNORR_FS_WEIGHT_TAG, SchnorrFsDeliveryProof, SchnorrFsDeliveryProofConstructorParams,
    SchnorrFsProof, SchnorrFsProofFromComponentsErrorReturn,
};

fn scalar_secret<P: IPairingAdapter>(fill: u8) -> Secret<P::Scalar> {
    let Ok(sampled) = P::Scalar::sample_from_uniform_bytes(
        SampleUniformScalarParams,
        SampleUniformScalarPayload {
            uniform: build_secret(SecretConstructorParamsOverrides {
                value: Some(vec![fill; P::Scalar::UNIFORM_BYTES_LENGTH]),
            }),
        },
    ) else {
        panic!("sample_from_uniform_bytes refused a fixed uniform input")
    };
    sampled.scalar
}

fn scalar<P: IPairingAdapter>(fill: u8) -> P::Scalar {
    scalar_secret::<P>(fill).expose().clone()
}

fn g1<P: IPairingAdapter>(pairing: &P) -> P::G1 {
    let Ok(generator) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
    generator.point
}

fn g2<P: IPairingAdapter>(pairing: &P) -> P::G2 {
    let Ok(generator) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
    generator.point
}

fn add_g1<P: IPairingAdapter>(pairing: &P, left: P::G1, right: P::G1) -> P::G1 {
    let Ok(sum) = pairing.add_g1(AddG1Params, AddG1Payload { left, right });
    sum.sum
}

fn add_g2<P: IPairingAdapter>(pairing: &P, left: P::G2, right: P::G2) -> P::G2 {
    let Ok(sum) = pairing.add_g2(AddG2Params, AddG2Payload { left, right });
    sum.sum
}

fn neg_g2<P: IPairingArithmetic>(pairing: &P, point: P::G2) -> P::G2 {
    let Ok(negation) = pairing.neg_g2(NegG2Params, NegG2Payload { point });
    negation.negation
}

fn mul_g1<P: IPairingAdapter>(pairing: &P, point: P::G1, scalar: P::Scalar) -> P::G1 {
    let Ok(product) = pairing.mul_g1(MulG1Params, MulG1Payload { point, scalar });
    product.product
}

fn mul_g2<P: IPairingAdapter>(pairing: &P, point: P::G2, scalar: P::Scalar) -> P::G2 {
    let Ok(product) = pairing.mul_g2(MulG2Params, MulG2Payload { point, scalar });
    product.product
}

fn msm_g1<P: IPairingAdapter>(pairing: &P, terms: Vec<(P::G1, P::Scalar)>) -> P::G1 {
    let Ok(sum) = pairing.msm_g1(
        MsmG1Params,
        MsmG1Payload {
            terms: terms
                .into_iter()
                .map(|(base, scalar)| MsmG1Term { base, scalar })
                .collect(),
        },
    );
    sum.sum
}

fn msm_g2<P: IPairingAdapter>(pairing: &P, terms: Vec<(P::G2, P::Scalar)>) -> P::G2 {
    let Ok(sum) = pairing.msm_g2(
        MsmG2Params,
        MsmG2Payload {
            terms: terms
                .into_iter()
                .map(|(base, scalar)| MsmG2Term { base, scalar })
                .collect(),
        },
    );
    sum.sum
}

fn add_scalar<P: IPairingArithmetic>(pairing: &P, left: P::Scalar, right: P::Scalar) -> P::Scalar {
    let Ok(sum) = pairing.add_scalar(AddScalarParams, AddScalarPayload { left, right });
    sum.sum
}

fn mul_scalar<P: IPairingArithmetic>(pairing: &P, left: P::Scalar, right: P::Scalar) -> P::Scalar {
    let Ok(product) = pairing.mul_scalar(MulScalarParams, MulScalarPayload { left, right });
    product.product
}

fn neg_scalar<P: IPairingArithmetic>(pairing: &P, scalar: P::Scalar) -> P::Scalar {
    let Ok(negation) = pairing.neg_scalar(NegScalarParams, NegScalarPayload { scalar });
    negation.negation
}

fn encode_g2_bytes<P: IPairingAdapter>(pairing: &P, point: P::G2) -> Vec<u8> {
    let Ok(encoded) = pairing.encode_g2(EncodeG2Params, EncodeG2Payload { point });
    encoded.bytes.as_ref().to_vec()
}

fn encode_scalar_bytes<P: IPairingAdapter>(pairing: &P, scalar: P::Scalar) -> Vec<u8> {
    let Ok(encoded) = pairing.encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar });
    encoded.bytes.expose().as_ref().to_vec()
}

fn g2_equal<P: IPairingAdapter>(pairing: &P, left: P::G2, right: P::G2) -> bool {
    encode_g2_bytes(pairing, left) == encode_g2_bytes(pairing, right)
}

fn scalar_equal<P: IPairingAdapter>(pairing: &P, left: P::Scalar, right: P::Scalar) -> bool {
    encode_scalar_bytes(pairing, left) == encode_scalar_bytes(pairing, right)
}

type AlgebraicMint<P> =
    AlgebraicMintStatement<MockIChainForms, <P as IPairingAdapter>::G1, <P as IPairingAdapter>::G2>;
type AlgebraicTransfer<P> = AlgebraicTransferStatement<
    MockIChainForms,
    <P as IPairingAdapter>::G1,
    <P as IPairingAdapter>::G2,
>;

fn clone_mint_context(
    context: &DeliveryContext<MockIChainForms, MintPurpose>,
) -> DeliveryContext<MockIChainForms, MintPurpose> {
    DeliveryContext {
        suite_identifier: context.suite_identifier.clone(),
        chain: context.chain,
        entitlement_contract: context.entitlement_contract,
        asset_identity_hash: context.asset_identity_hash.clone(),
        parameter_set_identifier: context.parameter_set_identifier.clone(),
        source_entitlement: context.source_entitlement,
        target_entitlement: context.target_entitlement,
        old_interval: context.old_interval,
        new_interval: context.new_interval,
        purpose: context.purpose,
        seller: context.seller,
        buyer: context.buyer,
        expiry: context.expiry,
    }
}

fn clone_transfer_context(
    context: &DeliveryContext<MockIChainForms, TransferPurpose>,
) -> DeliveryContext<MockIChainForms, TransferPurpose> {
    DeliveryContext {
        suite_identifier: context.suite_identifier.clone(),
        chain: context.chain,
        entitlement_contract: context.entitlement_contract,
        asset_identity_hash: context.asset_identity_hash.clone(),
        parameter_set_identifier: context.parameter_set_identifier.clone(),
        source_entitlement: context.source_entitlement,
        target_entitlement: context.target_entitlement,
        old_interval: context.old_interval,
        new_interval: context.new_interval,
        purpose: context.purpose,
        seller: context.seller,
        buyer: context.buyer,
        expiry: context.expiry,
    }
}

fn clone_mint_statement<P: IPairingAdapter>(statement: &AlgebraicMint<P>) -> AlgebraicMint<P> {
    AlgebraicMintStatement {
        context: clone_mint_context(&statement.context),
        hpub: statement.hpub.clone(),
        identity_element: statement.identity_element.clone(),
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
    }
}

fn clone_transfer_statement<P: IPairingAdapter>(
    statement: &AlgebraicTransfer<P>,
) -> AlgebraicTransfer<P> {
    AlgebraicTransferStatement {
        context: clone_transfer_context(&statement.context),
        identity_element: statement.identity_element.clone(),
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
    }
}

struct Fixtures<P: IPairingArithmetic> {
    mint_statement: AlgebraicMint<P>,
    transfer_statement: AlgebraicTransfer<P>,
}

fn fixtures<P: IPairingArithmetic>(pairing: &P) -> Fixtures<P> {
    let g1 = g1(pairing);
    let g2 = g2(pairing);
    let alpha = scalar::<P>(0x31);
    let r = scalar::<P>(0x32);
    let rho = scalar::<P>(0x33);
    let sigma = scalar::<P>(0x34);
    let f = scalar::<P>(0x35);
    let x_s = scalar::<P>(0x36);
    let y_s = scalar::<P>(0x37);
    let x_b = scalar::<P>(0x38);
    let y_b = scalar::<P>(0x39);
    let s = scalar::<P>(0x3a);
    let rho_n = scalar::<P>(0x3b);
    let sigma_n = scalar::<P>(0x3c);

    let identity = mul_g1(pairing, g1.clone(), f.clone());
    let hpub = mul_g2(pairing, g2.clone(), alpha.clone());
    let seller_keys = PublicKeysComponents {
        pk1: mul_g1(pairing, g1.clone(), x_s.clone()),
        pk2: mul_g2(pairing, g2.clone(), y_s.clone()),
    };
    let buyer_keys = PublicKeysComponents {
        pk1: mul_g1(pairing, g1.clone(), x_b.clone()),
        pk2: mul_g2(pairing, g2.clone(), y_b.clone()),
    };

    let old_envelope = EnvelopeComponents {
        c1: mul_g1(pairing, g1.clone(), rho.clone()),
        c2: msm_g1(
            pairing,
            vec![
                (g1.clone(), alpha.clone()),
                (identity.clone(), r.clone()),
                (seller_keys.pk1.clone(), rho.clone()),
            ],
        ),
        d1: mul_g2(pairing, g2.clone(), sigma.clone()),
        d2: msm_g2(
            pairing,
            vec![
                (g2.clone(), r.clone()),
                (seller_keys.pk2.clone(), sigma.clone()),
            ],
        ),
    };

    let new_envelope = EnvelopeComponents {
        c1: mul_g1(pairing, g1.clone(), rho_n.clone()),
        c2: add_g1(
            pairing,
            msm_g1(
                pairing,
                vec![
                    (old_envelope.c1.clone(), neg_scalar(pairing, x_s.clone())),
                    (identity.clone(), s.clone()),
                    (buyer_keys.pk1.clone(), rho_n.clone()),
                ],
            ),
            old_envelope.c2.clone(),
        ),
        d1: mul_g2(pairing, g2.clone(), sigma_n.clone()),
        d2: add_g2(
            pairing,
            msm_g2(
                pairing,
                vec![
                    (old_envelope.d1.clone(), neg_scalar(pairing, y_s.clone())),
                    (g2.clone(), s.clone()),
                    (buyer_keys.pk2.clone(), sigma_n.clone()),
                ],
            ),
            old_envelope.d2.clone(),
        ),
    };

    Fixtures {
        mint_statement: AlgebraicMintStatement {
            context: build_delivery_context::<MockIChainForms, MintPurpose>(
                DeliveryContextOverrides {
                    purpose: Some(DELIVERY_PURPOSE_MINT),
                    ..Default::default()
                },
            ),
            hpub,
            identity_element: identity.clone(),
            buyer_keys: PublicKeysComponents {
                pk1: seller_keys.pk1.clone(),
                pk2: seller_keys.pk2.clone(),
            },
            envelope: EnvelopeComponents {
                c1: old_envelope.c1.clone(),
                c2: old_envelope.c2.clone(),
                d1: old_envelope.d1.clone(),
                d2: old_envelope.d2.clone(),
            },
        },
        transfer_statement: AlgebraicTransferStatement {
            context: build_delivery_context::<MockIChainForms, TransferPurpose>(
                DeliveryContextOverrides {
                    purpose: Some(DELIVERY_PURPOSE_TRANSFER),
                    ..Default::default()
                },
            ),
            identity_element: identity,
            seller_keys: PublicKeysComponents {
                pk1: seller_keys.pk1.clone(),
                pk2: seller_keys.pk2.clone(),
            },
            buyer_keys: PublicKeysComponents {
                pk1: buyer_keys.pk1.clone(),
                pk2: buyer_keys.pk2.clone(),
            },
            old_envelope,
            new_envelope,
        },
    }
}

fn prove_mint_proof<P: IPairingArithmetic, E: IEncoderAdapter>(
    adapter: &SchnorrFsDeliveryProof<'_, P, E, MockIChainForms>,
    statement: &AlgebraicMint<P>,
    alpha_byte: u8,
) -> SchnorrFsProof<P> {
    let Ok(proved) = adapter.prove_mint(
        ProveMintParams,
        ProveMintPayload {
            master_scalar: MasterScalarComponents {
                value: scalar_secret::<P>(alpha_byte),
            },
            credential_randomness: scalar_secret::<P>(0x32),
            coins: EnvelopeCoins {
                rho: scalar_secret::<P>(0x33),
                sigma: scalar_secret::<P>(0x34),
            },
            statement,
        },
    ) else {
        panic!("prove_mint refused the mint witnesses")
    };
    proved.proof
}

fn prove_transfer_proof<P: IPairingArithmetic, E: IEncoderAdapter>(
    adapter: &SchnorrFsDeliveryProof<'_, P, E, MockIChainForms>,
    statement: &AlgebraicTransfer<P>,
) -> SchnorrFsProof<P> {
    let Ok(proved) = adapter.prove_transfer(
        ProveTransferParams,
        ProveTransferPayload {
            seller_secrets: KeyPairComponents {
                x: scalar_secret::<P>(0x36),
                y: scalar_secret::<P>(0x37),
            },
            offset: scalar_secret::<P>(0x3a),
            coins: EnvelopeCoins {
                rho: scalar_secret::<P>(0x3b),
                sigma: scalar_secret::<P>(0x3c),
            },
            statement,
        },
    ) else {
        panic!("prove_transfer refused the transfer witnesses")
    };
    proved.proof
}

fn verify<P: IPairingArithmetic, E: IEncoderAdapter>(
    adapter: &SchnorrFsDeliveryProof<'_, P, E, MockIChainForms>,
    statement: AlgebraicStatement<'_, MockIChainForms, P::G1, P::G2>,
    proof: &SchnorrFsProof<P>,
) -> bool {
    let Ok(checked) = adapter.verify(VerifyParams, VerifyPayload { statement, proof }) else {
        panic!("verify refused a well-formed proof")
    };
    checked.is_valid
}

fn components_of<P: IPairingArithmetic, E: IEncoderAdapter>(
    adapter: &SchnorrFsDeliveryProof<'_, P, E, MockIChainForms>,
    proof: &SchnorrFsProof<P>,
) -> DeliveryProofComponents<P::Scalar, P::G2> {
    let Ok(components) =
        adapter.proof_components(ProofComponentsParams, ProofComponentsPayload { proof });
    components.components
}

fn rebuild<P: IPairingArithmetic, E: IEncoderAdapter>(
    adapter: &SchnorrFsDeliveryProof<'_, P, E, MockIChainForms>,
    components: DeliveryProofComponents<P::Scalar, P::G2>,
) -> SchnorrFsProof<P> {
    let Ok(rebuilt) = adapter.proof_from_components(
        ProofFromComponentsParams,
        ProofFromComponentsPayload { components },
    ) else {
        panic!("proof_from_components refused components in the adapter's form")
    };
    rebuilt.proof
}

fn components_to_wire<P: IPairingAdapter>(
    components: DeliveryProofComponents<P::Scalar, P::G2>,
) -> DeliveryProofWireComponents<P::Scalar, P::G2> {
    match components {
        DeliveryProofComponents::MintBothGroups {
            challenge,
            responses,
        } => DeliveryProofWireComponents {
            relation: DeliveryRelation::Mint,
            verifier_form: VerifierGroupArithmetic::BothGroups,
            challenge,
            responses: vec![responses.alpha, responses.r, responses.rho, responses.sigma],
            second_group_first_messages: vec![],
        },
        DeliveryProofComponents::MintFirstGroupOnly {
            challenge,
            responses,
            first_messages,
        } => DeliveryProofWireComponents {
            relation: DeliveryRelation::Mint,
            verifier_form: VerifierGroupArithmetic::FirstGroupOnly,
            challenge,
            responses: vec![responses.alpha, responses.r, responses.rho, responses.sigma],
            second_group_first_messages: vec![
                first_messages.hpub,
                first_messages.d1,
                first_messages.d2,
            ],
        },
        DeliveryProofComponents::TransferBothGroups {
            challenge,
            responses,
        } => DeliveryProofWireComponents {
            relation: DeliveryRelation::Transfer,
            verifier_form: VerifierGroupArithmetic::BothGroups,
            challenge,
            responses: vec![
                responses.x,
                responses.y,
                responses.s,
                responses.rho,
                responses.sigma,
            ],
            second_group_first_messages: vec![],
        },
        DeliveryProofComponents::TransferFirstGroupOnly {
            challenge,
            responses,
            first_messages,
        } => DeliveryProofWireComponents {
            relation: DeliveryRelation::Transfer,
            verifier_form: VerifierGroupArithmetic::FirstGroupOnly,
            challenge,
            responses: vec![
                responses.x,
                responses.y,
                responses.s,
                responses.rho,
                responses.sigma,
            ],
            second_group_first_messages: vec![
                first_messages.pk2,
                first_messages.d1,
                first_messages.d2,
            ],
        },
    }
}

fn mint_transcript<P: IPairingAdapter>(
    statement: &AlgebraicMint<P>,
    first_messages: MintFirstMessages<P::G1, P::G2>,
) -> MintStatement<P, MockIChainForms> {
    MintStatement {
        suite_identifier: statement.context.suite_identifier.clone(),
        chain: statement.context.chain,
        entitlement_contract: statement.context.entitlement_contract,
        asset_identity_hash: statement.context.asset_identity_hash.clone(),
        parameter_set_identifier: statement.context.parameter_set_identifier.clone(),
        source_entitlement: statement.context.source_entitlement,
        target_entitlement: statement.context.target_entitlement,
        old_interval: statement.context.old_interval,
        new_interval: statement.context.new_interval,
        purpose: statement.context.purpose,
        seller: statement.context.seller,
        buyer: statement.context.buyer,
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

fn transfer_transcript<P: IPairingAdapter>(
    statement: &AlgebraicTransfer<P>,
    first_messages: TransferFirstMessages<P::G1, P::G2>,
) -> TransferStatement<P, MockIChainForms> {
    TransferStatement {
        suite_identifier: statement.context.suite_identifier.clone(),
        chain: statement.context.chain,
        entitlement_contract: statement.context.entitlement_contract,
        asset_identity_hash: statement.context.asset_identity_hash.clone(),
        parameter_set_identifier: statement.context.parameter_set_identifier.clone(),
        source_entitlement: statement.context.source_entitlement,
        target_entitlement: statement.context.target_entitlement,
        old_interval: statement.context.old_interval,
        new_interval: statement.context.new_interval,
        purpose: statement.context.purpose,
        seller: statement.context.seller,
        buyer: statement.context.buyer,
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

struct AcceptanceOutcome {
    mint_verifies: bool,
    transfer_verifies: bool,
    grant_verifies: bool,
    replacement_verifies: bool,
    rebuilt_proofs_verify: bool,
}

struct AcceptanceProbe;

struct AcceptanceStep<'a, P: IPairingArithmetic> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    random: &'a dyn IRandomSourceAdapter,
}

impl IPairingConsumer for AcceptanceProbe {
    type Output = CreateEncodingReturn<AcceptanceOutcome>;

    fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let adapter = payload.adapter;
        let Ok(hash_to_scalar) = create_hash_to_scalar::<P::Scalar>(
            &CreateHashToScalarDeps,
            build_create_hash_to_scalar_params(Default::default()),
            CreateHashToScalarPayload,
        ) else {
            panic!("create_hash_to_scalar refused the keccak256 params")
        };
        let Ok(random) = create_random_source(
            &CreateRandomSourceDeps,
            build_create_random_source_params(Default::default()),
            CreateRandomSourcePayload,
        );
        let step = AcceptanceStep {
            pairing: &adapter,
            hash_to_scalar: &*hash_to_scalar.adapter,
            random: &*random.adapter,
        };
        create_encoding(
            &CreateEncodingDeps { consumer: step },
            build_create_encoding_params(Default::default()),
            CreateEncodingPayload,
        )
    }
}

impl<'a, P: IPairingArithmetic + IPairingReference> IEncodingConsumer for AcceptanceStep<'a, P> {
    type Output = AcceptanceOutcome;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
        let fixtures = fixtures::<P>(self.pairing);
        let Ok(both_groups) = SchnorrFsDeliveryProof::<'_, P, E, MockIChainForms>::try_new(
            SchnorrFsDeliveryProofConstructorParams {
                pairing: self.pairing,
                hash_to_scalar: self.hash_to_scalar,
                encoder: &payload.adapter,
                random: self.random,
                verifier_form: VerifierGroupArithmetic::BothGroups,
            },
        ) else {
            panic!("try_new refused the both-groups subject")
        };
        let Ok(first_group_only) = SchnorrFsDeliveryProof::<'_, P, E, MockIChainForms>::try_new(
            SchnorrFsDeliveryProofConstructorParams {
                pairing: self.pairing,
                hash_to_scalar: self.hash_to_scalar,
                encoder: &payload.adapter,
                random: self.random,
                verifier_form: VerifierGroupArithmetic::FirstGroupOnly,
            },
        ) else {
            panic!("try_new refused the first-group-only subject")
        };

        let grant_statement = AlgebraicTransferStatement {
            context: build_delivery_context::<MockIChainForms, TransferPurpose>(
                DeliveryContextOverrides {
                    purpose: Some(DELIVERY_PURPOSE_GRANT),
                    ..Default::default()
                },
            ),
            identity_element: fixtures.transfer_statement.identity_element.clone(),
            seller_keys: PublicKeysComponents {
                pk1: fixtures.transfer_statement.seller_keys.pk1.clone(),
                pk2: fixtures.transfer_statement.seller_keys.pk2.clone(),
            },
            buyer_keys: PublicKeysComponents {
                pk1: fixtures.transfer_statement.buyer_keys.pk1.clone(),
                pk2: fixtures.transfer_statement.buyer_keys.pk2.clone(),
            },
            old_envelope: EnvelopeComponents {
                c1: fixtures.transfer_statement.old_envelope.c1.clone(),
                c2: fixtures.transfer_statement.old_envelope.c2.clone(),
                d1: fixtures.transfer_statement.old_envelope.d1.clone(),
                d2: fixtures.transfer_statement.old_envelope.d2.clone(),
            },
            new_envelope: EnvelopeComponents {
                c1: fixtures.transfer_statement.new_envelope.c1.clone(),
                c2: fixtures.transfer_statement.new_envelope.c2.clone(),
                d1: fixtures.transfer_statement.new_envelope.d1.clone(),
                d2: fixtures.transfer_statement.new_envelope.d2.clone(),
            },
        };

        let mut outcome = AcceptanceOutcome {
            mint_verifies: true,
            transfer_verifies: true,
            grant_verifies: true,
            replacement_verifies: true,
            rebuilt_proofs_verify: true,
        };
        for subject in [&both_groups, &first_group_only] {
            let mint_proof = prove_mint_proof(subject, &fixtures.mint_statement, 0x31);
            outcome.mint_verifies &= verify(
                subject,
                AlgebraicStatement::Mint(&fixtures.mint_statement),
                &mint_proof,
            );

            let transfer_proof = prove_transfer_proof(subject, &fixtures.transfer_statement);
            outcome.transfer_verifies &= verify(
                subject,
                AlgebraicStatement::Transfer(&fixtures.transfer_statement),
                &transfer_proof,
            );

            let grant_proof = prove_transfer_proof(subject, &grant_statement);
            outcome.grant_verifies &= verify(
                subject,
                AlgebraicStatement::Transfer(&grant_statement),
                &grant_proof,
            );

            let mut replacement_statement = clone_mint_statement::<P>(&fixtures.mint_statement);
            replacement_statement.context.purpose = DELIVERY_PURPOSE_REPLACEMENT;
            let replacement_proof = prove_mint_proof(subject, &replacement_statement, 0x31);
            outcome.replacement_verifies &= verify(
                subject,
                AlgebraicStatement::Mint(&replacement_statement),
                &replacement_proof,
            );

            let mint_components = components_of(subject, &mint_proof);
            let rebuilt_mint = rebuild(subject, mint_components);
            let transfer_components = components_of(subject, &transfer_proof);
            let rebuilt_transfer = rebuild(subject, transfer_components);
            outcome.rebuilt_proofs_verify &= verify(
                subject,
                AlgebraicStatement::Mint(&fixtures.mint_statement),
                &rebuilt_mint,
            ) && verify(
                subject,
                AlgebraicStatement::Transfer(&fixtures.transfer_statement),
                &rebuilt_transfer,
            );
        }
        outcome
    }
}

fn mutate_mint_responses<P: IPairingArithmetic>(
    pairing: &P,
    responses: MintResponses<P::Scalar>,
    index: usize,
    challenge: &P::Scalar,
) -> MintResponses<P::Scalar> {
    let mut values = [responses.alpha, responses.r, responses.rho, responses.sigma];
    values[index] = add_scalar(pairing, values[index].clone(), challenge.clone());
    MintResponses {
        alpha: values[0].clone(),
        r: values[1].clone(),
        rho: values[2].clone(),
        sigma: values[3].clone(),
    }
}

fn mutate_transfer_responses<P: IPairingArithmetic>(
    pairing: &P,
    responses: TransferResponses<P::Scalar>,
    index: usize,
    challenge: &P::Scalar,
) -> TransferResponses<P::Scalar> {
    let mut values = [
        responses.x,
        responses.y,
        responses.s,
        responses.rho,
        responses.sigma,
    ];
    values[index] = add_scalar(pairing, values[index].clone(), challenge.clone());
    TransferResponses {
        x: values[0].clone(),
        y: values[1].clone(),
        s: values[2].clone(),
        rho: values[3].clone(),
        sigma: values[4].clone(),
    }
}

fn mint_components_with<P: IPairingAdapter>(
    components: DeliveryProofComponents<P::Scalar, P::G2>,
    challenge: Option<P::Scalar>,
    responses: MintResponses<P::Scalar>,
    first_messages: Option<MintSecondGroupFirstMessages<P::G2>>,
) -> DeliveryProofComponents<P::Scalar, P::G2> {
    match components {
        DeliveryProofComponents::MintBothGroups { challenge: c, .. } => {
            DeliveryProofComponents::MintBothGroups {
                challenge: challenge.unwrap_or(c),
                responses,
            }
        }
        DeliveryProofComponents::MintFirstGroupOnly {
            challenge: c,
            first_messages: carried,
            ..
        } => DeliveryProofComponents::MintFirstGroupOnly {
            challenge: challenge.unwrap_or(c),
            responses,
            first_messages: first_messages.unwrap_or(carried),
        },
        _ => panic!("expected mint components"),
    }
}

fn transfer_components_with<P: IPairingAdapter>(
    components: DeliveryProofComponents<P::Scalar, P::G2>,
    challenge: Option<P::Scalar>,
    responses: TransferResponses<P::Scalar>,
    first_messages: Option<TransferSecondGroupFirstMessages<P::G2>>,
) -> DeliveryProofComponents<P::Scalar, P::G2> {
    match components {
        DeliveryProofComponents::TransferBothGroups { challenge: c, .. } => {
            DeliveryProofComponents::TransferBothGroups {
                challenge: challenge.unwrap_or(c),
                responses,
            }
        }
        DeliveryProofComponents::TransferFirstGroupOnly {
            challenge: c,
            first_messages: carried,
            ..
        } => DeliveryProofComponents::TransferFirstGroupOnly {
            challenge: challenge.unwrap_or(c),
            responses,
            first_messages: first_messages.unwrap_or(carried),
        },
        _ => panic!("expected transfer components"),
    }
}

struct MutationOutcome {
    mint_statement_mutations_fail: bool,
    transfer_statement_mutations_fail: bool,
    response_mutations_fail: bool,
    challenge_mutation_fails: bool,
    second_group_first_message_mutations_fail: bool,
    other_form_fails: bool,
    wrong_response_count_fails: bool,
    wrong_witness_fails: bool,
    cancelling_second_group_errors_fail: bool,
}

struct MutationProbe;

struct MutationStep<'a, P: IPairingArithmetic> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    random: &'a dyn IRandomSourceAdapter,
}

impl IPairingConsumer for MutationProbe {
    type Output = CreateEncodingReturn<MutationOutcome>;

    fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let adapter = payload.adapter;
        let Ok(hash_to_scalar) = create_hash_to_scalar::<P::Scalar>(
            &CreateHashToScalarDeps,
            build_create_hash_to_scalar_params(Default::default()),
            CreateHashToScalarPayload,
        ) else {
            panic!("create_hash_to_scalar refused the keccak256 params")
        };
        let Ok(random) = create_random_source(
            &CreateRandomSourceDeps,
            build_create_random_source_params(Default::default()),
            CreateRandomSourcePayload,
        );
        let step = MutationStep {
            pairing: &adapter,
            hash_to_scalar: &*hash_to_scalar.adapter,
            random: &*random.adapter,
        };
        create_encoding(
            &CreateEncodingDeps { consumer: step },
            build_create_encoding_params(Default::default()),
            CreateEncodingPayload,
        )
    }
}

impl<'a, P: IPairingArithmetic + IPairingReference> IEncodingConsumer for MutationStep<'a, P> {
    type Output = MutationOutcome;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
        let pairing = self.pairing;
        let fixtures = fixtures::<P>(pairing);
        let Ok(both_groups) = SchnorrFsDeliveryProof::<'_, P, E, MockIChainForms>::try_new(
            SchnorrFsDeliveryProofConstructorParams {
                pairing,
                hash_to_scalar: self.hash_to_scalar,
                encoder: &payload.adapter,
                random: self.random,
                verifier_form: VerifierGroupArithmetic::BothGroups,
            },
        ) else {
            panic!("try_new refused the both-groups subject")
        };
        let Ok(first_group_only) = SchnorrFsDeliveryProof::<'_, P, E, MockIChainForms>::try_new(
            SchnorrFsDeliveryProofConstructorParams {
                pairing,
                hash_to_scalar: self.hash_to_scalar,
                encoder: &payload.adapter,
                random: self.random,
                verifier_form: VerifierGroupArithmetic::FirstGroupOnly,
            },
        ) else {
            panic!("try_new refused the first-group-only subject")
        };

        let mut outcome = MutationOutcome {
            mint_statement_mutations_fail: true,
            transfer_statement_mutations_fail: true,
            response_mutations_fail: true,
            challenge_mutation_fails: true,
            second_group_first_message_mutations_fail: true,
            other_form_fails: true,
            wrong_response_count_fails: true,
            wrong_witness_fails: true,
            cancelling_second_group_errors_fail: true,
        };

        let mut mint_proofs = Vec::new();
        let mut transfer_proofs = Vec::new();
        for subject in [&both_groups, &first_group_only] {
            mint_proofs.push(prove_mint_proof(subject, &fixtures.mint_statement, 0x31));
            transfer_proofs.push(prove_transfer_proof(subject, &fixtures.transfer_statement));
        }

        for (index, subject) in [&both_groups, &first_group_only].iter().enumerate() {
            let mint_proof = &mint_proofs[index];
            let transfer_proof = &transfer_proofs[index];

            for mutated in [
                {
                    let mut statement = clone_mint_statement::<P>(&fixtures.mint_statement);
                    statement.context.expiry = 1_700_000_001;
                    statement
                },
                {
                    let mut statement = clone_mint_statement::<P>(&fixtures.mint_statement);
                    statement.hpub = g2(pairing);
                    statement
                },
                {
                    let mut statement = clone_mint_statement::<P>(&fixtures.mint_statement);
                    statement.envelope.c2 = g1(pairing);
                    statement
                },
                {
                    let mut statement = clone_mint_statement::<P>(&fixtures.mint_statement);
                    statement.buyer_keys.pk1 = g1(pairing);
                    statement
                },
            ] {
                outcome.mint_statement_mutations_fail &=
                    !verify(subject, AlgebraicStatement::Mint(&mutated), mint_proof);
            }

            for mutated in [
                {
                    let mut statement = clone_transfer_statement::<P>(&fixtures.transfer_statement);
                    statement.context.expiry = 1_700_000_001;
                    statement
                },
                {
                    let mut statement = clone_transfer_statement::<P>(&fixtures.transfer_statement);
                    statement.old_envelope.c1 = g1(pairing);
                    statement
                },
                {
                    let mut statement = clone_transfer_statement::<P>(&fixtures.transfer_statement);
                    statement.old_envelope.d2 = g2(pairing);
                    statement
                },
                {
                    let mut statement = clone_transfer_statement::<P>(&fixtures.transfer_statement);
                    statement.seller_keys.pk2 = g2(pairing);
                    statement
                },
                {
                    let mut statement = clone_transfer_statement::<P>(&fixtures.transfer_statement);
                    statement.identity_element = g1(pairing);
                    statement
                },
            ] {
                outcome.transfer_statement_mutations_fail &= !verify(
                    subject,
                    AlgebraicStatement::Transfer(&mutated),
                    transfer_proof,
                );
            }

            let mint_components = components_of(subject, mint_proof);
            for response_index in 0..4 {
                let mutated = mint_components_with::<P>(
                    components_of(subject, mint_proof),
                    None,
                    mutate_mint_responses(
                        pairing,
                        match components_of(subject, mint_proof) {
                            DeliveryProofComponents::MintBothGroups { responses, .. }
                            | DeliveryProofComponents::MintFirstGroupOnly { responses, .. } => {
                                responses
                            }
                            _ => panic!("expected mint components"),
                        },
                        response_index,
                        &match &mint_components {
                            DeliveryProofComponents::MintBothGroups { challenge, .. }
                            | DeliveryProofComponents::MintFirstGroupOnly { challenge, .. } => {
                                challenge.clone()
                            }
                            _ => panic!("expected mint components"),
                        },
                    ),
                    None,
                );
                let mutated_proof = rebuild(subject, mutated);
                outcome.response_mutations_fail &= !verify(
                    subject,
                    AlgebraicStatement::Mint(&fixtures.mint_statement),
                    &mutated_proof,
                );
            }
            for response_index in 0..5 {
                let (responses, challenge) = match components_of(subject, transfer_proof) {
                    DeliveryProofComponents::TransferBothGroups {
                        challenge,
                        responses,
                    }
                    | DeliveryProofComponents::TransferFirstGroupOnly {
                        challenge,
                        responses,
                        ..
                    } => (responses, challenge),
                    _ => panic!("expected transfer components"),
                };
                let mutated = transfer_components_with::<P>(
                    components_of(subject, transfer_proof),
                    None,
                    mutate_transfer_responses(pairing, responses, response_index, &challenge),
                    None,
                );
                let mutated_proof = rebuild(subject, mutated);
                outcome.response_mutations_fail &= !verify(
                    subject,
                    AlgebraicStatement::Transfer(&fixtures.transfer_statement),
                    &mutated_proof,
                );
            }

            {
                let (first_response, challenge) = match components_of(subject, mint_proof) {
                    DeliveryProofComponents::MintBothGroups {
                        challenge,
                        responses,
                    }
                    | DeliveryProofComponents::MintFirstGroupOnly {
                        challenge,
                        responses,
                        ..
                    } => (responses.alpha, challenge),
                    _ => panic!("expected mint components"),
                };
                let responses = match mint_components {
                    DeliveryProofComponents::MintBothGroups { responses, .. }
                    | DeliveryProofComponents::MintFirstGroupOnly { responses, .. } => responses,
                    _ => panic!("expected mint components"),
                };
                let mutated = mint_components_with::<P>(
                    components_of(subject, mint_proof),
                    Some(first_response),
                    responses,
                    None,
                );
                let _ = challenge;
                let mutated_proof = rebuild(subject, mutated);
                outcome.challenge_mutation_fails &= !verify(
                    subject,
                    AlgebraicStatement::Mint(&fixtures.mint_statement),
                    &mutated_proof,
                );
            }
            {
                let (first_response, responses) = match components_of(subject, transfer_proof) {
                    DeliveryProofComponents::TransferBothGroups {
                        challenge,
                        responses,
                    }
                    | DeliveryProofComponents::TransferFirstGroupOnly {
                        challenge,
                        responses,
                        ..
                    } => {
                        let _ = challenge;
                        (responses.x.clone(), responses)
                    }
                    _ => panic!("expected transfer components"),
                };
                let mutated = transfer_components_with::<P>(
                    components_of(subject, transfer_proof),
                    Some(first_response),
                    responses,
                    None,
                );
                let mutated_proof = rebuild(subject, mutated);
                outcome.challenge_mutation_fails &= !verify(
                    subject,
                    AlgebraicStatement::Transfer(&fixtures.transfer_statement),
                    &mutated_proof,
                );
            }
        }

        let first_group_mint_components = components_of(&first_group_only, &mint_proofs[1]);
        for message_index in 0..3 {
            let mutated = mint_components_with::<P>(
                components_of(&first_group_only, &mint_proofs[1]),
                None,
                match &first_group_mint_components {
                    DeliveryProofComponents::MintFirstGroupOnly { responses, .. } => {
                        MintResponses {
                            alpha: responses.alpha.clone(),
                            r: responses.r.clone(),
                            rho: responses.rho.clone(),
                            sigma: responses.sigma.clone(),
                        }
                    }
                    _ => panic!("expected first-group-only mint components"),
                },
                Some(match &first_group_mint_components {
                    DeliveryProofComponents::MintFirstGroupOnly { first_messages, .. } => {
                        let mut messages = [
                            first_messages.hpub.clone(),
                            first_messages.d1.clone(),
                            first_messages.d2.clone(),
                        ];
                        messages[message_index] = g2(pairing);
                        MintSecondGroupFirstMessages {
                            hpub: messages[0].clone(),
                            d1: messages[1].clone(),
                            d2: messages[2].clone(),
                        }
                    }
                    _ => panic!("expected first-group-only mint components"),
                }),
            );
            let mutated_proof = rebuild(&first_group_only, mutated);
            outcome.second_group_first_message_mutations_fail &= !verify(
                &first_group_only,
                AlgebraicStatement::Mint(&fixtures.mint_statement),
                &mutated_proof,
            );
        }
        let first_group_transfer_components = components_of(&first_group_only, &transfer_proofs[1]);
        for message_index in 0..3 {
            let mutated = transfer_components_with::<P>(
                components_of(&first_group_only, &transfer_proofs[1]),
                None,
                match &first_group_transfer_components {
                    DeliveryProofComponents::TransferFirstGroupOnly { responses, .. } => {
                        TransferResponses {
                            x: responses.x.clone(),
                            y: responses.y.clone(),
                            s: responses.s.clone(),
                            rho: responses.rho.clone(),
                            sigma: responses.sigma.clone(),
                        }
                    }
                    _ => panic!("expected first-group-only transfer components"),
                },
                Some(match &first_group_transfer_components {
                    DeliveryProofComponents::TransferFirstGroupOnly { first_messages, .. } => {
                        let mut messages = [
                            first_messages.pk2.clone(),
                            first_messages.d1.clone(),
                            first_messages.d2.clone(),
                        ];
                        messages[message_index] = g2(pairing);
                        TransferSecondGroupFirstMessages {
                            pk2: messages[0].clone(),
                            d1: messages[1].clone(),
                            d2: messages[2].clone(),
                        }
                    }
                    _ => panic!("expected first-group-only transfer components"),
                }),
            );
            let mutated_proof = rebuild(&first_group_only, mutated);
            outcome.second_group_first_message_mutations_fail &= !verify(
                &first_group_only,
                AlgebraicStatement::Transfer(&fixtures.transfer_statement),
                &mutated_proof,
            );
        }

        outcome.other_form_fails &= !verify(
            &first_group_only,
            AlgebraicStatement::Mint(&fixtures.mint_statement),
            &mint_proofs[0],
        ) && !verify(
            &first_group_only,
            AlgebraicStatement::Transfer(&fixtures.transfer_statement),
            &transfer_proofs[0],
        ) && !verify(
            &both_groups,
            AlgebraicStatement::Mint(&fixtures.mint_statement),
            &mint_proofs[1],
        ) && !verify(
            &both_groups,
            AlgebraicStatement::Transfer(&fixtures.transfer_statement),
            &transfer_proofs[1],
        );

        for proof in [&mint_proofs[0], &transfer_proofs[0]] {
            let mut wire = components_to_wire::<P>(components_of(&both_groups, proof));
            wire.responses.pop();
            outcome.wrong_response_count_fails &= DeliveryProofComponents::try_from(wire).is_err();
        }

        for subject in [&both_groups, &first_group_only] {
            let wrong_witness_proof = prove_mint_proof(subject, &fixtures.mint_statement, 0x3d);
            outcome.wrong_witness_fails &= !verify(
                subject,
                AlgebraicStatement::Mint(&fixtures.mint_statement),
                &wrong_witness_proof,
            );
        }

        {
            let g1 = g1(pairing);
            let g2 = g2(pairing);
            let k_alpha = scalar::<P>(0x41);
            let k_r = scalar::<P>(0x42);
            let k_rho = scalar::<P>(0x43);
            let k_sigma = scalar::<P>(0x44);
            let t_hpub = mul_g2(pairing, g2.clone(), k_alpha.clone());
            let t_c1 = mul_g1(pairing, g1.clone(), k_rho.clone());
            let t_c2 = msm_g1(
                pairing,
                vec![
                    (g1.clone(), k_alpha.clone()),
                    (
                        fixtures.mint_statement.identity_element.clone(),
                        k_r.clone(),
                    ),
                    (
                        fixtures.mint_statement.buyer_keys.pk1.clone(),
                        k_rho.clone(),
                    ),
                ],
            );
            let t_d1 = mul_g2(pairing, g2.clone(), k_sigma.clone());
            let t_d2 = msm_g2(
                pairing,
                vec![
                    (g2.clone(), k_r.clone()),
                    (
                        fixtures.mint_statement.buyer_keys.pk2.clone(),
                        k_sigma.clone(),
                    ),
                ],
            );
            let t_hpub_crafted = add_g2(pairing, t_hpub, g2.clone());
            let t_d1_crafted = add_g2(pairing, t_d1, neg_g2(pairing, g2));
            let transcript = mint_transcript::<P>(
                &fixtures.mint_statement,
                MintFirstMessages {
                    hpub: t_hpub_crafted.clone(),
                    c1: t_c1,
                    c2: t_c2,
                    d1: t_d1_crafted.clone(),
                    d2: t_d2,
                },
            );
            let Ok(challenged) = challenge(
                &ChallengeDeps {
                    pairing,
                    encoder: &payload.adapter,
                    tag: &first_group_only.challenge_tag,
                    hash_to_scalar: self.hash_to_scalar,
                },
                ChallengeParams,
                ChallengePayload {
                    transcript: ChallengeTranscript::Mint(&transcript),
                },
            ) else {
                panic!("challenge refused the crafted transcript")
            };
            let c = challenged.challenge;
            let crafted = rebuild(
                &first_group_only,
                DeliveryProofComponents::MintFirstGroupOnly {
                    challenge: c.clone(),
                    responses: MintResponses {
                        alpha: add_scalar(
                            pairing,
                            k_alpha,
                            mul_scalar(pairing, c.clone(), scalar::<P>(0x31)),
                        ),
                        r: add_scalar(
                            pairing,
                            k_r,
                            mul_scalar(pairing, c.clone(), scalar::<P>(0x32)),
                        ),
                        rho: add_scalar(
                            pairing,
                            k_rho,
                            mul_scalar(pairing, c.clone(), scalar::<P>(0x33)),
                        ),
                        sigma: add_scalar(
                            pairing,
                            k_sigma,
                            mul_scalar(pairing, c.clone(), scalar::<P>(0x34)),
                        ),
                    },
                    first_messages: MintSecondGroupFirstMessages {
                        hpub: t_hpub_crafted,
                        d1: t_d1_crafted,
                        d2: transcript.first_messages.d2.clone(),
                    },
                },
            );
            outcome.cancelling_second_group_errors_fail = !verify(
                &first_group_only,
                AlgebraicStatement::Mint(&fixtures.mint_statement),
                &crafted,
            );
        }

        outcome
    }
}

struct RefusalOutcome {
    mint_purpose_refusals: Vec<Option<MintStatementFromFieldsErrorReturn<MockIChainForms>>>,
    transfer_purpose_refusals: Vec<Option<TransferStatementFromFieldsErrorReturn<MockIChainForms>>>,
    wrong_form_refusal: Option<ProofFromComponentsErrorReturn>,
    shape_refusals: Vec<Option<DeliveryProofShapeErrorReturn>>,
}

struct RefusalProbe;

struct RefusalStep<'a, P: IPairingArithmetic> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    random: &'a dyn IRandomSourceAdapter,
}

impl IPairingConsumer for RefusalProbe {
    type Output = CreateEncodingReturn<RefusalOutcome>;

    fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let adapter = payload.adapter;
        let Ok(hash_to_scalar) = create_hash_to_scalar::<P::Scalar>(
            &CreateHashToScalarDeps,
            build_create_hash_to_scalar_params(Default::default()),
            CreateHashToScalarPayload,
        ) else {
            panic!("create_hash_to_scalar refused the keccak256 params")
        };
        let Ok(random) = create_random_source(
            &CreateRandomSourceDeps,
            build_create_random_source_params(Default::default()),
            CreateRandomSourcePayload,
        );
        let step = RefusalStep {
            pairing: &adapter,
            hash_to_scalar: &*hash_to_scalar.adapter,
            random: &*random.adapter,
        };
        create_encoding(
            &CreateEncodingDeps { consumer: step },
            build_create_encoding_params(Default::default()),
            CreateEncodingPayload,
        )
    }
}

impl<'a, P: IPairingArithmetic + IPairingReference> IEncodingConsumer for RefusalStep<'a, P> {
    type Output = RefusalOutcome;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
        let pairing = self.pairing;
        let fixtures = fixtures::<P>(pairing);
        let Ok(both_groups) = SchnorrFsDeliveryProof::<'_, P, E, MockIChainForms>::try_new(
            SchnorrFsDeliveryProofConstructorParams {
                pairing,
                hash_to_scalar: self.hash_to_scalar,
                encoder: &payload.adapter,
                random: self.random,
                verifier_form: VerifierGroupArithmetic::BothGroups,
            },
        ) else {
            panic!("try_new refused the both-groups subject")
        };
        let Ok(first_group_only) = SchnorrFsDeliveryProof::<'_, P, E, MockIChainForms>::try_new(
            SchnorrFsDeliveryProofConstructorParams {
                pairing,
                hash_to_scalar: self.hash_to_scalar,
                encoder: &payload.adapter,
                random: self.random,
                verifier_form: VerifierGroupArithmetic::FirstGroupOnly,
            },
        ) else {
            panic!("try_new refused the first-group-only subject")
        };

        let _ = &payload.adapter;
        let _ = both_groups.challenge_tag;
        let _ = first_group_only.challenge_tag;

        let mint_proof = prove_mint_proof(&both_groups, &fixtures.mint_statement, 0x31);
        let transfer_proof = prove_transfer_proof(&both_groups, &fixtures.transfer_statement);
        let first_group_mint_proof =
            prove_mint_proof(&first_group_only, &fixtures.mint_statement, 0x31);

        let mint_transcript = mint_transcript::<P>(
            &fixtures.mint_statement,
            MintFirstMessages {
                hpub: g2(pairing),
                c1: g1(pairing),
                c2: g1(pairing),
                d1: g2(pairing),
                d2: g2(pairing),
            },
        );
        let transfer_transcript = transfer_transcript::<P>(
            &fixtures.transfer_statement,
            TransferFirstMessages {
                pk1: g1(pairing),
                c1: g1(pairing),
                c2: g1(pairing),
                pk2: g2(pairing),
                d1: g2(pairing),
                d2: g2(pairing),
            },
        );

        let mut mint_purpose_refusals = Vec::new();
        let Ok(mint_description) = MintStatementDescription::<P, MockIChainForms>::try_new(
            MintStatementDescriptionConstructorParams { pairing },
        );
        for code in [2u16, 3u16] {
            let Ok(fields) = encoding::IEncodingContract::to_fields(
                &mint_description,
                ToFieldsParams,
                &mint_transcript,
            );
            let mut values = fields.fields.values;
            values[10] = CanonicalFieldValue::Unsigned16(code);
            mint_purpose_refusals.push(
                encoding::IEncodingContract::fields_to_value(
                    &mint_description,
                    FromFieldsParams,
                    encoding::CanonicalFields { values },
                )
                .err(),
            );
        }

        let mut transfer_purpose_refusals = Vec::new();
        let Ok(transfer_description) = TransferStatementDescription::<P, MockIChainForms>::try_new(
            TransferStatementDescriptionConstructorParams { pairing },
        );
        for code in [1u16, 4u16] {
            let Ok(fields) = encoding::IEncodingContract::to_fields(
                &transfer_description,
                ToFieldsParams,
                &transfer_transcript,
            );
            let mut values = fields.fields.values;
            values[10] = CanonicalFieldValue::Unsigned16(code);
            transfer_purpose_refusals.push(
                encoding::IEncodingContract::fields_to_value(
                    &transfer_description,
                    FromFieldsParams,
                    encoding::CanonicalFields { values },
                )
                .err(),
            );
        }

        let wrong_form_refusal = first_group_only
            .proof_from_components(
                ProofFromComponentsParams,
                ProofFromComponentsPayload {
                    components: components_of(&both_groups, &mint_proof),
                },
            )
            .err();

        let mut shape_refusals = Vec::new();
        {
            let mut wire = components_to_wire::<P>(components_of(&both_groups, &mint_proof));
            wire.responses.pop();
            shape_refusals.push(DeliveryProofComponents::try_from(wire).err());
        }
        {
            let mut wire = components_to_wire::<P>(components_of(&both_groups, &mint_proof));
            wire.responses.push(scalar::<P>(0x45));
            shape_refusals.push(DeliveryProofComponents::try_from(wire).err());
        }
        {
            let mut wire = components_to_wire::<P>(components_of(&both_groups, &transfer_proof));
            wire.responses.pop();
            shape_refusals.push(DeliveryProofComponents::try_from(wire).err());
        }
        {
            let mut wire =
                components_to_wire::<P>(components_of(&first_group_only, &first_group_mint_proof));
            wire.second_group_first_messages.pop();
            shape_refusals.push(DeliveryProofComponents::try_from(wire).err());
        }
        {
            let mut wire = components_to_wire::<P>(components_of(&both_groups, &mint_proof));
            wire.second_group_first_messages.push(g2(pairing));
            shape_refusals.push(DeliveryProofComponents::try_from(wire).err());
        }

        RefusalOutcome {
            mint_purpose_refusals,
            transfer_purpose_refusals,
            wrong_form_refusal,
            shape_refusals,
        }
    }
}

struct ComponentsOutcome {
    both_groups_carries_no_second_group_messages: bool,
    responses_follow_the_witnesses: bool,
    mint_first_messages_are_the_relations: bool,
    transfer_first_messages_are_the_relations: bool,
    challenge_is_the_transcripts: bool,
}

struct ComponentsProbe;

struct ComponentsStep<'a, P: IPairingArithmetic> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    random: &'a dyn IRandomSourceAdapter,
}

impl IPairingConsumer for ComponentsProbe {
    type Output = CreateEncodingReturn<ComponentsOutcome>;

    fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let adapter = payload.adapter;
        let Ok(hash_to_scalar) = create_hash_to_scalar::<P::Scalar>(
            &CreateHashToScalarDeps,
            build_create_hash_to_scalar_params(Default::default()),
            CreateHashToScalarPayload,
        ) else {
            panic!("create_hash_to_scalar refused the keccak256 params")
        };
        let Ok(random) = create_random_source(
            &CreateRandomSourceDeps,
            build_create_random_source_params(Default::default()),
            CreateRandomSourcePayload,
        );
        let step = ComponentsStep {
            pairing: &adapter,
            hash_to_scalar: &*hash_to_scalar.adapter,
            random: &*random.adapter,
        };
        create_encoding(
            &CreateEncodingDeps { consumer: step },
            build_create_encoding_params(Default::default()),
            CreateEncodingPayload,
        )
    }
}

impl<'a, P: IPairingArithmetic + IPairingReference> IEncodingConsumer for ComponentsStep<'a, P> {
    type Output = ComponentsOutcome;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
        let pairing = self.pairing;
        let fixtures = fixtures::<P>(pairing);
        let Ok(both_groups) = SchnorrFsDeliveryProof::<'_, P, E, MockIChainForms>::try_new(
            SchnorrFsDeliveryProofConstructorParams {
                pairing,
                hash_to_scalar: self.hash_to_scalar,
                encoder: &payload.adapter,
                random: self.random,
                verifier_form: VerifierGroupArithmetic::BothGroups,
            },
        ) else {
            panic!("try_new refused the both-groups subject")
        };
        let Ok(first_group_only) = SchnorrFsDeliveryProof::<'_, P, E, MockIChainForms>::try_new(
            SchnorrFsDeliveryProofConstructorParams {
                pairing,
                hash_to_scalar: self.hash_to_scalar,
                encoder: &payload.adapter,
                random: self.random,
                verifier_form: VerifierGroupArithmetic::FirstGroupOnly,
            },
        ) else {
            panic!("try_new refused the first-group-only subject")
        };

        let mint_proof_both = prove_mint_proof(&both_groups, &fixtures.mint_statement, 0x31);
        let transfer_proof_both = prove_transfer_proof(&both_groups, &fixtures.transfer_statement);
        let mint_proof_first = prove_mint_proof(&first_group_only, &fixtures.mint_statement, 0x31);
        let transfer_proof_first =
            prove_transfer_proof(&first_group_only, &fixtures.transfer_statement);

        let both_groups_carries_no_second_group_messages = matches!(
            components_of(&both_groups, &mint_proof_both),
            DeliveryProofComponents::MintBothGroups { .. }
        ) && matches!(
            components_of(&both_groups, &transfer_proof_both),
            DeliveryProofComponents::TransferBothGroups { .. }
        );

        let responses_follow_the_witnesses = matches!(
            components_of(&both_groups, &mint_proof_both),
            DeliveryProofComponents::MintBothGroups {
                responses: MintResponses { .. },
                ..
            }
        ) && matches!(
            components_of(&first_group_only, &mint_proof_first),
            DeliveryProofComponents::MintFirstGroupOnly {
                responses: MintResponses { .. },
                ..
            }
        ) && matches!(
            components_of(&both_groups, &transfer_proof_both),
            DeliveryProofComponents::TransferBothGroups {
                responses: TransferResponses { .. },
                ..
            }
        ) && matches!(
            components_of(&first_group_only, &transfer_proof_first),
            DeliveryProofComponents::TransferFirstGroupOnly {
                responses: TransferResponses { .. },
                ..
            }
        );

        let g1 = g1(pairing);
        let g2 = g2(pairing);

        let mint_first_messages_are_the_relations =
            match components_of(&first_group_only, &mint_proof_first) {
                DeliveryProofComponents::MintFirstGroupOnly {
                    challenge,
                    responses,
                    first_messages,
                } => {
                    let n = neg_scalar(pairing, challenge.clone());
                    g2_equal(
                        pairing,
                        first_messages.hpub.clone(),
                        msm_g2(
                            pairing,
                            vec![
                                (g2.clone(), responses.alpha.clone()),
                                (fixtures.mint_statement.hpub.clone(), n.clone()),
                            ],
                        ),
                    ) && g2_equal(
                        pairing,
                        first_messages.d1.clone(),
                        msm_g2(
                            pairing,
                            vec![
                                (g2.clone(), responses.sigma.clone()),
                                (fixtures.mint_statement.envelope.d1.clone(), n.clone()),
                            ],
                        ),
                    ) && g2_equal(
                        pairing,
                        first_messages.d2.clone(),
                        msm_g2(
                            pairing,
                            vec![
                                (g2.clone(), responses.r.clone()),
                                (
                                    fixtures.mint_statement.buyer_keys.pk2.clone(),
                                    responses.sigma.clone(),
                                ),
                                (fixtures.mint_statement.envelope.d2.clone(), n),
                            ],
                        ),
                    )
                }
                _ => panic!("expected first-group-only mint components"),
            };

        let transfer_first_messages_are_the_relations =
            match components_of(&first_group_only, &transfer_proof_first) {
                DeliveryProofComponents::TransferFirstGroupOnly {
                    challenge,
                    responses,
                    first_messages,
                } => {
                    let n = neg_scalar(pairing, challenge);
                    let delta_d2 = add_g2(
                        pairing,
                        fixtures.transfer_statement.new_envelope.d2.clone(),
                        neg_g2(pairing, fixtures.transfer_statement.old_envelope.d2.clone()),
                    );
                    g2_equal(
                        pairing,
                        first_messages.pk2.clone(),
                        msm_g2(
                            pairing,
                            vec![
                                (g2.clone(), responses.y.clone()),
                                (
                                    fixtures.transfer_statement.seller_keys.pk2.clone(),
                                    n.clone(),
                                ),
                            ],
                        ),
                    ) && g2_equal(
                        pairing,
                        first_messages.d1.clone(),
                        msm_g2(
                            pairing,
                            vec![
                                (g2.clone(), responses.sigma.clone()),
                                (
                                    fixtures.transfer_statement.new_envelope.d1.clone(),
                                    n.clone(),
                                ),
                            ],
                        ),
                    ) && g2_equal(
                        pairing,
                        first_messages.d2.clone(),
                        msm_g2(
                            pairing,
                            vec![
                                (
                                    fixtures.transfer_statement.old_envelope.d1.clone(),
                                    neg_scalar(pairing, responses.y.clone()),
                                ),
                                (g2.clone(), responses.s.clone()),
                                (
                                    fixtures.transfer_statement.buyer_keys.pk2.clone(),
                                    responses.sigma.clone(),
                                ),
                                (delta_d2, n),
                            ],
                        ),
                    )
                }
                _ => panic!("expected first-group-only transfer components"),
            };

        let challenge_is_the_transcripts = match components_of(&both_groups, &mint_proof_both) {
            DeliveryProofComponents::MintBothGroups {
                challenge: c,
                responses,
            } => {
                let n = neg_scalar(pairing, c.clone());
                let recomputed = MintFirstMessages {
                    hpub: msm_g2(
                        pairing,
                        vec![
                            (g2.clone(), responses.alpha.clone()),
                            (fixtures.mint_statement.hpub.clone(), n.clone()),
                        ],
                    ),
                    c1: msm_g1(
                        pairing,
                        vec![
                            (g1.clone(), responses.rho.clone()),
                            (fixtures.mint_statement.envelope.c1.clone(), n.clone()),
                        ],
                    ),
                    c2: msm_g1(
                        pairing,
                        vec![
                            (g1.clone(), responses.alpha.clone()),
                            (
                                fixtures.mint_statement.identity_element.clone(),
                                responses.r.clone(),
                            ),
                            (
                                fixtures.mint_statement.buyer_keys.pk1.clone(),
                                responses.rho.clone(),
                            ),
                            (fixtures.mint_statement.envelope.c2.clone(), n.clone()),
                        ],
                    ),
                    d1: msm_g2(
                        pairing,
                        vec![
                            (g2.clone(), responses.sigma.clone()),
                            (fixtures.mint_statement.envelope.d1.clone(), n.clone()),
                        ],
                    ),
                    d2: msm_g2(
                        pairing,
                        vec![
                            (g2.clone(), responses.r.clone()),
                            (
                                fixtures.mint_statement.buyer_keys.pk2.clone(),
                                responses.sigma.clone(),
                            ),
                            (fixtures.mint_statement.envelope.d2.clone(), n),
                        ],
                    ),
                };
                let transcript = mint_transcript::<P>(&fixtures.mint_statement, recomputed);
                let Ok(challenged) = challenge(
                    &ChallengeDeps {
                        pairing,
                        encoder: &payload.adapter,
                        tag: &both_groups.challenge_tag,
                        hash_to_scalar: self.hash_to_scalar,
                    },
                    ChallengeParams,
                    ChallengePayload {
                        transcript: ChallengeTranscript::Mint(&transcript),
                    },
                ) else {
                    panic!("challenge refused the recomputed transcript")
                };
                scalar_equal(pairing, challenged.challenge, c)
            }
            _ => panic!("expected both-groups mint components"),
        };

        ComponentsOutcome {
            both_groups_carries_no_second_group_messages,
            responses_follow_the_witnesses,
            mint_first_messages_are_the_relations,
            transfer_first_messages_are_the_relations,
            challenge_is_the_transcripts,
        }
    }
}

struct DeclarationOutcome {
    algebras: &'static [EnvelopeAlgebra],
    verifier_forms: &'static [VerifierGroupArithmetic],
    statement_versions: &'static [u16],
    challenge_tag: &'static [u8],
    weight_tag: &'static [u8],
    adapter_version: u32,
    interface_version: u32,
}

struct DeclarationProbe;

struct DeclarationStep<P: IPairingArithmetic>(PhantomData<P>);

impl IPairingConsumer for DeclarationProbe {
    type Output = CreateEncodingReturn<DeclarationOutcome>;

    fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
        &self,
        _params: ConsumePairingParams,
        _payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        create_encoding(
            &CreateEncodingDeps {
                consumer: DeclarationStep::<P>(PhantomData),
            },
            build_create_encoding_params(Default::default()),
            CreateEncodingPayload,
        )
    }
}

impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for DeclarationStep<P> {
    type Output = DeclarationOutcome;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        _payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
        let declaration = SchnorrFsDeliveryProof::<'_, P, E, MockIChainForms>::DECLARATION;
        DeclarationOutcome {
            algebras: declaration.algebras,
            verifier_forms: declaration.verifier_forms,
            statement_versions: declaration.statement_versions,
            challenge_tag: declaration.challenge_tag,
            weight_tag: declaration.weight_tag,
            adapter_version: declaration.adapter_version,
            interface_version: declaration.interface_version,
        }
    }
}

struct RelationRejectionProbe;

struct RelationRejectionStep<'a, P: IPairingArithmetic> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    random: &'a dyn IRandomSourceAdapter,
}

impl IPairingConsumer for RelationRejectionProbe {
    type Output = CreateEncodingReturn<bool>;

    fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let adapter = payload.adapter;
        let Ok(hash_to_scalar) = create_hash_to_scalar::<P::Scalar>(
            &CreateHashToScalarDeps,
            build_create_hash_to_scalar_params(Default::default()),
            CreateHashToScalarPayload,
        ) else {
            panic!("create_hash_to_scalar refused the keccak256 params")
        };
        let Ok(random) = create_random_source(
            &CreateRandomSourceDeps,
            build_create_random_source_params(Default::default()),
            CreateRandomSourcePayload,
        );
        let step = RelationRejectionStep {
            pairing: &adapter,
            hash_to_scalar: &*hash_to_scalar.adapter,
            random: &*random.adapter,
        };
        create_encoding(
            &CreateEncodingDeps { consumer: step },
            build_create_encoding_params(Default::default()),
            CreateEncodingPayload,
        )
    }
}

impl<'a, P: IPairingArithmetic + IPairingReference> IEncodingConsumer
    for RelationRejectionStep<'a, P>
{
    type Output = bool;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
        let fixtures = fixtures::<P>(self.pairing);
        let Ok(both_groups) = SchnorrFsDeliveryProof::<'_, P, E, MockIChainForms>::try_new(
            SchnorrFsDeliveryProofConstructorParams {
                pairing: self.pairing,
                hash_to_scalar: self.hash_to_scalar,
                encoder: &payload.adapter,
                random: self.random,
                verifier_form: VerifierGroupArithmetic::BothGroups,
            },
        ) else {
            panic!("try_new refused the both-groups subject")
        };
        let transfer_proof = prove_transfer_proof(&both_groups, &fixtures.transfer_statement);
        verify(
            &both_groups,
            AlgebraicStatement::Mint(&fixtures.mint_statement),
            &transfer_proof,
        )
    }
}

fn run_probe<P: IPairingConsumer>(consumer: P, concrete: PairingConcrete) -> P::Output {
    let Ok(success) = create_pairing(
        &CreatePairingDeps { consumer },
        build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(concrete),
            ..Default::default()
        }),
        CreatePairingPayload,
    ) else {
        panic!("create_pairing refused the declared pairing params")
    };
    success.output
}

fn acceptance(concrete: PairingConcrete) -> AcceptanceOutcome {
    let Ok(encoding) = run_probe(AcceptanceProbe, concrete) else {
        panic!("create_encoding refused the abi params")
    };
    encoding.output
}

fn mutation(concrete: PairingConcrete) -> MutationOutcome {
    let Ok(encoding) = run_probe(MutationProbe, concrete) else {
        panic!("create_encoding refused the abi params")
    };
    encoding.output
}

fn refusal(concrete: PairingConcrete) -> RefusalOutcome {
    let Ok(encoding) = run_probe(RefusalProbe, concrete) else {
        panic!("create_encoding refused the abi params")
    };
    encoding.output
}

fn components(concrete: PairingConcrete) -> ComponentsOutcome {
    let Ok(encoding) = run_probe(ComponentsProbe, concrete) else {
        panic!("create_encoding refused the abi params")
    };
    encoding.output
}

#[test]
// Contract: the issuer's mint proof over the mint relation verifies (CR-09; CD-01).
fn a_mint_proof_verifies_in_both_forms() {
    // Arrange: the acceptance probe over the default fixtures.
    // Act: resolve the pairing, hash, encoder, and randomness; prove and verify.
    let outcome = acceptance(PairingConcrete::Bls12381Arkworks);
    // Assert: the mint proof verifies under both forms.
    assert!(outcome.mint_verifies);
}

#[test]
// Contract: the seller's proof from its fresh decryption of the previous
// envelope and the total offset verifies (CR-09; CD-02).
fn a_transfer_proof_from_the_sellers_fresh_decryption_verifies_in_both_forms() {
    // Arrange: the acceptance probe over the default fixtures.
    // Act: resolve the adapters; prove and verify.
    let outcome = acceptance(PairingConcrete::Bls12381Arkworks);
    // Assert: the transfer proof verifies under both forms.
    assert!(outcome.transfer_verifies);
}

#[test]
// Contract: grant proves the transfer relation and replacement the mint
// relation (CD-05).
fn a_grant_and_a_replacement_proof_verify_in_both_forms() {
    // Arrange: the acceptance probe.
    // Act: resolve the adapters; prove and verify.
    let outcome = acceptance(PairingConcrete::Bls12381Arkworks);
    // Assert: both purposes verify.
    assert!(outcome.grant_verifies && outcome.replacement_verifies);
}

#[test]
// Contract: the proof the settlement carries and the harness rebuilds is the
// proof.
fn a_proof_rebuilt_from_its_components_verifies() {
    // Arrange: the acceptance probe.
    // Act: resolve the adapters; prove, decompose, rebuild, and verify.
    let outcome = acceptance(PairingConcrete::Bls12381Arkworks);
    // Assert: both rebuilt proofs verify.
    assert!(outcome.rebuilt_proofs_verify);
}

#[test]
// Contract: the construction holds on each further pairing concrete in both
// forms (CR-09 on each curve form).
fn mint_and_transfer_proofs_verify_on_bn254_arkworks() {
    // Arrange: the acceptance probe.
    // Act: resolve the bn254-arkworks pairing.
    let outcome = acceptance(PairingConcrete::Bn254Arkworks);
    // Assert.
    assert!(outcome.mint_verifies && outcome.transfer_verifies);
}

#[test]
// Contract: the construction holds on each further pairing concrete in both
// forms (CR-09 on each curve form).
fn mint_and_transfer_proofs_verify_on_bn254_halo2curves() {
    // Arrange: the acceptance probe.
    // Act: resolve the bn254-halo2curves pairing.
    let outcome = acceptance(PairingConcrete::Bn254Halo2curves);
    // Assert.
    assert!(outcome.mint_verifies && outcome.transfer_verifies);
}

#[test]
// Contract: the construction holds on each further pairing concrete in both
// forms (CR-09 on each curve form).
fn mint_and_transfer_proofs_verify_on_bls12_381_halo2curves() {
    // Arrange: the acceptance probe.
    // Act: resolve the bls12-381-halo2curves pairing.
    let outcome = acceptance(PairingConcrete::Bls12381Halo2curves);
    // Assert.
    assert!(outcome.mint_verifies && outcome.transfer_verifies);
}

#[test]
// Contract: the proof binds every statement field, so it is valid for exactly
// one settlement (CR-09 mutate each statement field; replay across
// settlements).
fn a_proof_fails_against_a_statement_differing_in_one_field() {
    // Arrange: the mutation probe.
    // Act: resolve the adapters; prove and verify against mutated statements.
    let outcome = mutation(PairingConcrete::Bls12381Arkworks);
    // Assert.
    assert!(outcome.mint_statement_mutations_fail && outcome.transfer_statement_mutations_fail);
}

#[test]
// Contract: every response and the challenge are bound (CR-09 mutate each
// response).
fn a_proof_with_a_changed_response_or_challenge_fails() {
    // Arrange: the mutation probe.
    // Act.
    let outcome = mutation(PairingConcrete::Bls12381Arkworks);
    // Assert.
    assert!(outcome.response_mutations_fail && outcome.challenge_mutation_fails);
}

#[test]
// Contract: a first-group-only proof binds its carried second-group first
// messages.
fn a_first_group_only_proof_with_a_changed_second_group_first_message_fails() {
    // Arrange: the mutation probe.
    // Act.
    let outcome = mutation(PairingConcrete::Bls12381Arkworks);
    // Assert.
    assert!(outcome.second_group_first_message_mutations_fail);
}

#[test]
// Contract: a proof of one verifier form fails under the other, and a wire
// proof of the wrong response count does not verify.
fn a_proof_fails_under_the_other_verifier_form_or_with_the_wrong_response_count() {
    // Arrange: the mutation probe.
    // Act.
    let outcome = mutation(PairingConcrete::Bls12381Arkworks);
    // Assert.
    assert!(outcome.other_form_fails && outcome.wrong_response_count_fails);
}

#[test]
// Contract: the verifier rejects a false statement.
fn a_proof_from_a_witness_the_statement_does_not_hold_fails() {
    // Arrange: the mutation probe.
    // Act.
    let outcome = mutation(PairingConcrete::Bls12381Arkworks);
    // Assert.
    assert!(outcome.wrong_witness_fails);
}

#[test]
// Contract: the hash-weighted product is equivalent to the separate
// second-group equalities (the risk register's unweighted-product risk).
fn a_proof_whose_second_group_errors_cancel_unweighted_fails_the_weighted_product() {
    // Arrange: the mutation probe.
    // Act.
    let outcome = mutation(PairingConcrete::Bls12381Arkworks);
    // Assert.
    assert!(outcome.cancelling_second_group_errors_fail);
}

#[test]
// Contract (compile-time): a transfer purpose cannot initialize
// `DeliveryContext<F, MintPurpose>` and a mint purpose cannot initialize
// `DeliveryContext<F, TransferPurpose>`; wire reconstruction tests the
// corresponding wrong-code refusals.
fn the_mint_and_transfer_types_exclude_the_other_relations_purpose() {
    // The exclusion is the `DeliveryContext<F, Purpose>` parameter; wire
    // reconstruction refusals are covered by
    // `wire_reconstruction_refuses_the_other_relations_purpose_codes`.
}

#[test]
// Contract: the verifier refuses a proof of the other relation before any
// challenge calculation.
fn the_verifier_rejects_a_proof_of_the_other_relation() {
    // Arrange: the relation-rejection probe.
    // Act: verify a transfer-variant proof against a mint statement.
    let Ok(encoding) = run_probe(RelationRejectionProbe, PairingConcrete::Bls12381Arkworks) else {
        panic!("create_encoding refused the abi params")
    };
    // Assert: is_valid is false.
    assert!(!encoding.output);
}

#[test]
// Contract: a wire proof of the wrong shape is refused before admission, and
// a typed variant in the wrong verifier form is refused.
fn wire_components_refuse_wrong_response_and_first_message_counts() {
    // Arrange: the refusal probe.
    // Act.
    let outcome = refusal(PairingConcrete::Bls12381Arkworks);
    // Assert.
    assert_eq!(
        outcome.shape_refusals,
        vec![
            Some(DeliveryProofShapeErrorReturn::ResponseCount {
                expected: 4,
                actual: 3
            }),
            Some(DeliveryProofShapeErrorReturn::ResponseCount {
                expected: 4,
                actual: 5
            }),
            Some(DeliveryProofShapeErrorReturn::ResponseCount {
                expected: 5,
                actual: 4
            }),
            Some(
                DeliveryProofShapeErrorReturn::SecondGroupFirstMessageCount {
                    expected: 3,
                    actual: 2
                }
            ),
            Some(
                DeliveryProofShapeErrorReturn::SecondGroupFirstMessageCount {
                    expected: 0,
                    actual: 1
                }
            ),
        ]
    );
    assert_eq!(
        outcome.wrong_form_refusal,
        Some(ProofFromComponentsErrorReturn::SchnorrFs(
            SchnorrFsProofFromComponentsErrorReturn::VerifierFormMismatch
        ))
    );
}

#[test]
// Contract: a purpose code from the other relation is refused at its index.
fn wire_reconstruction_refuses_the_other_relations_purpose_codes() {
    // Arrange: the refusal probe.
    // Act.
    let outcome = refusal(PairingConcrete::Bls12381Arkworks);
    // Assert.
    assert_eq!(
        outcome.mint_purpose_refusals,
        vec![
            Some(MintStatementFromFieldsErrorReturn::PurposeCode { index: 10, code: 2 }),
            Some(MintStatementFromFieldsErrorReturn::PurposeCode { index: 10, code: 3 }),
        ]
    );
    assert_eq!(
        outcome.transfer_purpose_refusals,
        vec![
            Some(TransferStatementFromFieldsErrorReturn::PurposeCode { index: 10, code: 1 }),
            Some(TransferStatementFromFieldsErrorReturn::PurposeCode { index: 10, code: 4 }),
        ]
    );
}

#[test]
// Contract: the components `harness-crypto/generate/evm/proof_vectors` emits
// and the contract verifies are the relation's (CR-09 bit-for-bit inputs).
fn the_proof_components_carry_the_challenge_the_responses_and_the_forms_first_messages() {
    // Arrange: the components probe.
    // Act.
    let outcome = components(PairingConcrete::Bls12381Arkworks);
    // Assert.
    assert!(outcome.both_groups_carries_no_second_group_messages);
    assert!(outcome.responses_follow_the_witnesses);
    assert!(outcome.mint_first_messages_are_the_relations);
    assert!(outcome.transfer_first_messages_are_the_relations);
    assert!(outcome.challenge_is_the_transcripts);
}

#[test]
// Contract: the concrete's declaration names the envelope algebra, both
// verifier forms, delivery-statement version one, its adapter version, and
// the interface version it implements.
fn schnorr_fs_delivery_proof_declares_its_algebras_forms_and_statement_versions() {
    // Arrange: the declaration probe.
    // Act: read the concrete's DECLARATION inside the encoding step.
    let Ok(encoding) = run_probe(DeclarationProbe, PairingConcrete::Bls12381Arkworks) else {
        panic!("create_encoding refused the abi params")
    };
    let declaration = encoding.output;
    // Assert.
    assert_eq!(declaration.algebras, &[EnvelopeAlgebra::PairingElGamal]);
    assert_eq!(declaration.verifier_forms.len(), 2);
    assert!(matches!(
        declaration.verifier_forms[0],
        VerifierGroupArithmetic::BothGroups
    ));
    assert!(matches!(
        declaration.verifier_forms[1],
        VerifierGroupArithmetic::FirstGroupOnly
    ));
    assert_eq!(
        declaration.statement_versions,
        &[DELIVERY_STATEMENT_VERSION_ONE]
    );
    assert_eq!(
        declaration.challenge_tag,
        b"ChainTorrent-v1-proof-challenge"
    );
    assert_eq!(declaration.weight_tag, b"ChainTorrent-v1-proof-weight");
    assert_eq!(declaration.adapter_version, 1);
    assert_eq!(
        declaration.interface_version,
        DELIVERY_PROOF_INTERFACE_VERSION
    );
}

#[test]
// Contract: the pairing-product weights' domain tag is admitted by
// `DomainTag::try_new`.
fn the_weight_tag_is_admitted_as_a_domain_tag() {
    // Arrange: none; the constant is the arrangement.
    // Act.
    let tag = DomainTag::try_new(DomainTagConstructorParams {
        bytes: SCHNORR_FS_WEIGHT_TAG.to_vec(),
    });
    // Assert.
    assert_eq!(SCHNORR_FS_WEIGHT_TAG, b"ChainTorrent-v1-proof-weight");
    assert!(tag.is_ok());
}
