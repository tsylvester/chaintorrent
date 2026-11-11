#![allow(dead_code, clippy::expect_used, clippy::too_many_arguments)]

use core::convert::Infallible;
use core::marker::PhantomData;

use chain::{
    ConsumeChainFormsParams, ConsumeChainFormsPayload, CreateChainFormsParams, IChainForms,
    IChainFormsConsumer, build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, create_chain_forms,
};
use domain::{
    ParameterSetIdentifierConstructorParamsOverrides, Secret, SecretConstructorParamsOverrides,
    SuiteIdentifierConstructorParamsOverrides, build_parameter_set_identifier, build_secret,
    build_suite_identifier,
};
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, ConsumeEncodingParams, ConsumeEncodingPayload,
    CreateEncodingParams, CreateEncodingParamsOverrides, ICanonicalField, IDecoderAdapter,
    IEncoderAdapter, IEncodingConsumer, ToFieldParams, build_create_encoding_deps,
    build_create_encoding_params, build_create_encoding_payload, build_from_field_params,
    create_encoding,
};
use envelope::{
    ConsumeKeyAgreementParams, ConsumeKeyAgreementPayload, CreateKeyAgreementParams,
    EnvelopeAlgebra, EnvelopeCoins, EnvelopeComponents, EnvelopeComponentsOverrides,
    IKeyAgreementAdapter, IKeyAgreementConsumer, KeyPairComponents, PublicKeysComponents,
    PublicKeysComponentsOverrides, build_create_key_agreement_deps,
    build_create_key_agreement_params, build_create_key_agreement_payload, build_envelope_coins,
    build_envelope_components, build_key_pair_components, build_public_keys_components,
    create_key_agreement,
};
use hash_to_scalar::{
    CreateHashToScalarParams, IHashToScalarAdapter, build_create_hash_to_scalar_deps,
    build_create_hash_to_scalar_params, build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::{
    ConsumeKemParams, ConsumeKemPayload, CreateKemParams, ICredentialKemAdapter, IKemConsumer,
    MasterScalarComponents, build_create_kem_deps, build_create_kem_params,
    build_create_kem_payload, build_master_scalar_components, create_kem,
};
use pairing::{
    AddG1Params, AddG1Payload, AddG2Params, AddG2Payload, AddScalarParams, AddScalarPayload,
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParams, EncodeG1Params,
    EncodeG1Payload, EncodeG2Params, EncodeG2Payload, EncodeScalarParams, EncodeScalarPayload,
    G1GeneratorParams, G1GeneratorPayload, G2GeneratorParams, G2GeneratorPayload, IPairingAdapter,
    IPairingArithmetic, IPairingConsumer, IPairingReference, ISampleUniformScalar, MsmG1Params,
    MsmG1Payload, MsmG1Term, MsmG2Params, MsmG2Payload, MsmG2Term, MulG1Params, MulG1Payload,
    MulG2Params, MulG2Payload, MulScalarParams, MulScalarPayload, NegG1Params, NegG1Payload,
    NegG2Params, NegG2Payload, NegScalarParams, NegScalarPayload, SampleUniformScalarParams,
    SampleUniformScalarPayload, VerifierGroupArithmetic, build_create_pairing_deps,
    build_create_pairing_params, build_create_pairing_payload, create_pairing,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParams, CreateRandomSourceParamsOverrides,
    CreateRandomSourcePayload, IRandomSourceAdapter, build_create_random_source_params,
    create_random_source,
};
use serde_json::{Map, Value};
use zeroize::Zeroize;

use super::interface::{
    AlgebraicMintStatement, AlgebraicStatement, AlgebraicTransferStatement,
    ConsumeDeliveryProofParams, ConsumeDeliveryProofPayload, CreateDeliveryProofCaseConsumeParams,
    CreateDeliveryProofCaseConsumePayload, CreateDeliveryProofCaseDeps,
    CreateDeliveryProofCaseErrorReturn, CreateDeliveryProofCaseParams,
    CreateDeliveryProofCasePayload, CreateDeliveryProofCaseReturn,
    CreateDeliveryProofCaseSuccessReturn, CreateDeliveryProofDeps, CreateDeliveryProofFn,
    CreateDeliveryProofParams, CreateDeliveryProofPayload, CreateDeliveryProofReturn,
    CreateDeliveryProofSuccessReturn, DELIVERY_PROOF_INTERFACE_VERSION, DeliveryContext,
    DeliveryProofComponents, DeliveryProofConcrete, DeliveryProofDeclaration,
    DeliveryProofWireComponents, ICreateDeliveryProofCaseConsumer, IDeliveryProofAdapter,
    IDeliveryProofConsumer, MintResponses, MintSecondGroupFirstMessages,
    MockIDeliveryProofAdapterFailureMode, MockIDeliveryProofAdapterProveMintErrorReturn,
    MockIDeliveryProofAdapterProveTransferErrorReturn, ProofComponentsParams,
    ProofComponentsPayload, ProofComponentsReturn, ProofComponentsSuccessReturn,
    ProofFromComponentsErrorReturn, ProofFromComponentsParams, ProofFromComponentsPayload,
    ProofFromComponentsReturn, ProofFromComponentsSuccessReturn, ProveMintErrorReturn,
    ProveMintParams, ProveMintPayload, ProveMintReturn, ProveMintSuccessReturn,
    ProveTransferErrorReturn, ProveTransferParams, ProveTransferPayload, ProveTransferReturn,
    ProveTransferSuccessReturn, RunCreateDeliveryProofCaseFn, TransferResponses,
    TransferSecondGroupFirstMessages, VerifyParams, VerifyPayload, VerifyReturn,
    VerifySuccessReturn,
};
use crate::mint_statement::provides::{
    DELIVERY_STATEMENT_VERSION_ONE, DeliveryRelation, MintFirstMessages, MintPurpose,
    TransferPurpose,
};
use crate::transfer_statement::provides::TransferFirstMessages;

pub const MOCK_DELIVERY_PROOF_CHALLENGE_TAG: &[u8] = b"ChainTorrent-v1-mock-proof-challenge";

pub const MOCK_DELIVERY_PROOF_WEIGHT_TAG: &[u8] = b"ChainTorrent-v1-mock-proof-weight";

pub(crate) const MOCK_I_DELIVERY_PROOF_ADAPTER_DECLARATION: DeliveryProofDeclaration =
    DeliveryProofDeclaration {
        algebras: &[EnvelopeAlgebra::PairingElGamal],
        verifier_forms: &[
            VerifierGroupArithmetic::BothGroups,
            VerifierGroupArithmetic::FirstGroupOnly,
        ],
        statement_versions: &[DELIVERY_STATEMENT_VERSION_ONE],
        challenge_tag: MOCK_DELIVERY_PROOF_CHALLENGE_TAG,
        weight_tag: MOCK_DELIVERY_PROOF_WEIGHT_TAG,
        adapter_version: 1,
        interface_version: DELIVERY_PROOF_INTERFACE_VERSION,
    };

const FILL_CHAIN: u8 = 0x11;
const FILL_ENTITLEMENT_CONTRACT: u8 = 0x12;
const FILL_SOURCE_ENTITLEMENT: u8 = 0x13;
const FILL_TARGET_ENTITLEMENT: u8 = 0x14;
const FILL_OLD_INTERVAL: u8 = 0x15;
const FILL_NEW_INTERVAL: u8 = 0x16;
const FILL_SELLER: u8 = 0x17;
const FILL_BUYER: u8 = 0x18;
const FILL_TRANSFER_BUYER_PK1: u8 = 0x31;
const FILL_TRANSFER_BUYER_PK2: u8 = 0x32;
const FILL_IDENTITY_ELEMENT: u8 = 0x50;
const FILL_OLD_C1: u8 = 0x51;
const FILL_OLD_C2: u8 = 0x52;
const FILL_OLD_D1: u8 = 0x53;
const FILL_OLD_D2: u8 = 0x54;
const FILL_CREDENTIAL_RANDOMNESS: u8 = 0x62;
const FILL_OFFSET: u8 = 0x63;
const FILL_NONCE_ALPHA: u8 = 0x71;
const FILL_NONCE_R: u8 = 0x72;
const FILL_NONCE_RHO: u8 = 0x73;
const FILL_NONCE_SIGMA: u8 = 0x74;
const FILL_NONCE_X: u8 = 0x75;
const FILL_NONCE_Y: u8 = 0x76;
const FILL_NONCE_S: u8 = 0x77;
const FILL_NONCE_RHO_TRANSFER: u8 = 0x78;
const FILL_NONCE_SIGMA_TRANSFER: u8 = 0x79;
const FILL_MINT_ALPHA: u8 = 0x81;
const FILL_MINT_R: u8 = 0x82;
const FILL_MINT_RHO: u8 = 0x83;
const FILL_MINT_SIGMA: u8 = 0x84;
const FILL_TRANSFER_X: u8 = 0x85;
const FILL_TRANSFER_Y: u8 = 0x86;
const FILL_TRANSFER_S: u8 = 0x87;
const FILL_TRANSFER_RHO: u8 = 0x88;
const FILL_TRANSFER_SIGMA: u8 = 0x89;
const FILL_MINT_FM_HPUB: u8 = 0x8A;
const FILL_MINT_FM_D1: u8 = 0x8B;
const FILL_MINT_FM_D2: u8 = 0x8C;
const FILL_TRANSFER_FM_PK2: u8 = 0x8D;
const FILL_TRANSFER_FM_D1: u8 = 0x8E;
const FILL_TRANSFER_FM_D2: u8 = 0x8F;
const FILL_WIRE_CHALLENGE: u8 = 0x90;

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

fn g1_mul<P: IPairingAdapter>(pairing: &P, fill: u8) -> P::G1 {
    let Ok(success) = pairing.mul_g1(
        MulG1Params,
        MulG1Payload {
            point: default_g1(pairing),
            scalar: sampled_scalar::<P>(fill),
        },
    );
    success.product
}

fn g2_mul<P: IPairingAdapter>(pairing: &P, fill: u8) -> P::G2 {
    let Ok(success) = pairing.mul_g2(
        MulG2Params,
        MulG2Payload {
            point: default_g2(pairing),
            scalar: sampled_scalar::<P>(fill),
        },
    );
    success.product
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

fn add_g1_of<P: IPairingAdapter>(pairing: &P, left: P::G1, right: P::G1) -> P::G1 {
    let Ok(success) = pairing.add_g1(AddG1Params, AddG1Payload { left, right });
    success.sum
}

fn add_g2_of<P: IPairingAdapter>(pairing: &P, left: P::G2, right: P::G2) -> P::G2 {
    let Ok(success) = pairing.add_g2(AddG2Params, AddG2Payload { left, right });
    success.sum
}

fn neg_g1_of<P: IPairingArithmetic>(pairing: &P, point: P::G1) -> P::G1 {
    let Ok(success) = pairing.neg_g1(NegG1Params, NegG1Payload { point });
    success.negation
}

fn neg_g2_of<P: IPairingArithmetic>(pairing: &P, point: P::G2) -> P::G2 {
    let Ok(success) = pairing.neg_g2(NegG2Params, NegG2Payload { point });
    success.negation
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

fn encoded_g1<P: IPairingAdapter>(pairing: &P, point: P::G1) -> Vec<u8> {
    let Ok(encoded) = pairing.encode_g1(EncodeG1Params, EncodeG1Payload { point });
    encoded.bytes.as_ref().to_vec()
}

fn encoded_g2<P: IPairingAdapter>(pairing: &P, point: P::G2) -> Vec<u8> {
    let Ok(encoded) = pairing.encode_g2(EncodeG2Params, EncodeG2Payload { point });
    encoded.bytes.as_ref().to_vec()
}

fn encoded_scalar<P: IPairingAdapter>(pairing: &P, scalar: P::Scalar) -> Vec<u8> {
    let Ok(encoded) = pairing.encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar });
    encoded.bytes.expose().as_ref().to_vec()
}

fn byte_array(bytes: &[u8]) -> Value {
    Value::Array(bytes.iter().map(|byte| Value::from(*byte)).collect())
}

fn encoded_g1_value<P: IPairingAdapter>(pairing: &P, point: &P::G1) -> Value {
    byte_array(&encoded_g1(pairing, point.clone()))
}

fn encoded_g2_value<P: IPairingAdapter>(pairing: &P, point: &P::G2) -> Value {
    byte_array(&encoded_g2(pairing, point.clone()))
}

fn encoded_scalar_value<P: IPairingAdapter>(pairing: &P, scalar: &P::Scalar) -> Value {
    byte_array(&encoded_scalar(pairing, scalar.clone()))
}

fn scalars_equal<P: IPairingAdapter>(pairing: &P, left: &P::Scalar, right: &P::Scalar) -> bool {
    encoded_scalar(pairing, left.clone()) == encoded_scalar(pairing, right.clone())
}

fn g2_equal<P: IPairingAdapter>(pairing: &P, left: &P::G2, right: &P::G2) -> bool {
    encoded_g2(pairing, left.clone()) == encoded_g2(pairing, right.clone())
}

fn seeded_field(kind: CanonicalFieldKind, seed: u8) -> CanonicalFieldValue {
    match kind {
        CanonicalFieldKind::FixedBytes20 => CanonicalFieldValue::FixedBytes20([seed; 20]),
        CanonicalFieldKind::FixedBytes32 => CanonicalFieldValue::FixedBytes32([seed; 32]),
        CanonicalFieldKind::Unsigned16 => CanonicalFieldValue::Unsigned16(u16::from(seed)),
        CanonicalFieldKind::Unsigned32 => CanonicalFieldValue::Unsigned32(u32::from(seed)),
        CanonicalFieldKind::Unsigned64 => CanonicalFieldValue::Unsigned64(u64::from(seed)),
        CanonicalFieldKind::Unsigned256 => CanonicalFieldValue::Unsigned256([seed; 32]),
        CanonicalFieldKind::Bytes => CanonicalFieldValue::Bytes(vec![seed]),
        CanonicalFieldKind::Text => CanonicalFieldValue::Text(seed.to_string()),
    }
}

fn form_from_seed<T: ICanonicalField>(seed: u8) -> T {
    T::from_field(
        build_from_field_params(Default::default()),
        seeded_field(T::KIND, seed),
    )
    .expect("a constant of the declared kind is admitted")
    .value
}

fn canonical_field_bytes(value: &CanonicalFieldValue) -> Vec<u8> {
    match value {
        CanonicalFieldValue::FixedBytes20(bytes) => bytes.to_vec(),
        CanonicalFieldValue::FixedBytes32(bytes) => bytes.to_vec(),
        CanonicalFieldValue::Unsigned16(value) => value.to_be_bytes().to_vec(),
        CanonicalFieldValue::Unsigned32(value) => value.to_be_bytes().to_vec(),
        CanonicalFieldValue::Unsigned64(value) => value.to_be_bytes().to_vec(),
        CanonicalFieldValue::Unsigned256(bytes) => bytes.to_vec(),
        CanonicalFieldValue::Bytes(bytes) => bytes.clone(),
        CanonicalFieldValue::Text(text) => text.clone().into_bytes(),
    }
}

fn form_field_bytes<T: ICanonicalField>(form: &T) -> Vec<u8> {
    let Ok(rendered) = T::to_field(ToFieldParams, form);
    canonical_field_bytes(&rendered.field)
}

fn context_mint_bytes<F: IChainForms>(context: &DeliveryContext<F, MintPurpose>) -> Vec<Vec<u8>> {
    vec![
        context.suite_identifier.identifier().to_vec(),
        context.suite_identifier.version().to_be_bytes().to_vec(),
        form_field_bytes(&context.chain),
        form_field_bytes(&context.entitlement_contract),
        context.asset_root.as_bytes().to_vec(),
        context.parameter_set_identifier.as_bytes().to_vec(),
        form_field_bytes(&context.source_entitlement),
        form_field_bytes(&context.target_entitlement),
        form_field_bytes(&context.old_interval),
        form_field_bytes(&context.new_interval),
        context.purpose.code().to_be_bytes().to_vec(),
        form_field_bytes(&context.seller),
        form_field_bytes(&context.buyer),
        context.expiry.to_be_bytes().to_vec(),
    ]
}

fn context_transfer_bytes<F: IChainForms>(
    context: &DeliveryContext<F, TransferPurpose>,
) -> Vec<Vec<u8>> {
    vec![
        context.suite_identifier.identifier().to_vec(),
        context.suite_identifier.version().to_be_bytes().to_vec(),
        form_field_bytes(&context.chain),
        form_field_bytes(&context.entitlement_contract),
        context.asset_root.as_bytes().to_vec(),
        context.parameter_set_identifier.as_bytes().to_vec(),
        form_field_bytes(&context.source_entitlement),
        form_field_bytes(&context.target_entitlement),
        form_field_bytes(&context.old_interval),
        form_field_bytes(&context.new_interval),
        context.purpose.code().to_be_bytes().to_vec(),
        form_field_bytes(&context.seller),
        form_field_bytes(&context.buyer),
        context.expiry.to_be_bytes().to_vec(),
    ]
}

fn mixed_scalar<P: IPairingAdapter>(inputs: &[Vec<u8>]) -> P::Scalar {
    let length = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
    let mut uniform = vec![0u8; length];
    for (index, byte) in inputs.iter().flatten().enumerate() {
        uniform[index % length] ^= *byte;
    }
    P::Scalar::sample_from_uniform_bytes(
        SampleUniformScalarParams,
        SampleUniformScalarPayload {
            uniform: build_secret(uniform, SecretConstructorParamsOverrides),
        },
    )
    .expect("sampling a default scalar always succeeds")
    .scalar
    .expose()
    .clone()
}

fn context_mint_rendered<F: IChainForms>(
    context: &DeliveryContext<F, MintPurpose>,
) -> Map<String, Value> {
    let mut fields = Map::new();
    let mut suite = context.suite_identifier.identifier().to_vec();
    suite.extend_from_slice(&context.suite_identifier.version().to_be_bytes());
    fields.insert("suite_identifier".to_string(), byte_array(&suite));
    fields.insert(
        "chain".to_string(),
        byte_array(&form_field_bytes(&context.chain)),
    );
    fields.insert(
        "entitlement_contract".to_string(),
        byte_array(&form_field_bytes(&context.entitlement_contract)),
    );
    fields.insert(
        "asset_root".to_string(),
        byte_array(context.asset_root.as_bytes()),
    );
    fields.insert(
        "parameter_set_identifier".to_string(),
        byte_array(context.parameter_set_identifier.as_bytes()),
    );
    fields.insert(
        "source_entitlement".to_string(),
        byte_array(&form_field_bytes(&context.source_entitlement)),
    );
    fields.insert(
        "target_entitlement".to_string(),
        byte_array(&form_field_bytes(&context.target_entitlement)),
    );
    fields.insert(
        "old_interval".to_string(),
        byte_array(&form_field_bytes(&context.old_interval)),
    );
    fields.insert(
        "new_interval".to_string(),
        byte_array(&form_field_bytes(&context.new_interval)),
    );
    fields.insert("purpose".to_string(), Value::from(context.purpose.code()));
    fields.insert(
        "seller".to_string(),
        byte_array(&form_field_bytes(&context.seller)),
    );
    fields.insert(
        "buyer".to_string(),
        byte_array(&form_field_bytes(&context.buyer)),
    );
    fields.insert("expiry".to_string(), Value::from(context.expiry));
    fields
}

fn context_transfer_rendered<F: IChainForms>(
    context: &DeliveryContext<F, TransferPurpose>,
) -> Map<String, Value> {
    let mut fields = Map::new();
    let mut suite = context.suite_identifier.identifier().to_vec();
    suite.extend_from_slice(&context.suite_identifier.version().to_be_bytes());
    fields.insert("suite_identifier".to_string(), byte_array(&suite));
    fields.insert(
        "chain".to_string(),
        byte_array(&form_field_bytes(&context.chain)),
    );
    fields.insert(
        "entitlement_contract".to_string(),
        byte_array(&form_field_bytes(&context.entitlement_contract)),
    );
    fields.insert(
        "asset_root".to_string(),
        byte_array(context.asset_root.as_bytes()),
    );
    fields.insert(
        "parameter_set_identifier".to_string(),
        byte_array(context.parameter_set_identifier.as_bytes()),
    );
    fields.insert(
        "source_entitlement".to_string(),
        byte_array(&form_field_bytes(&context.source_entitlement)),
    );
    fields.insert(
        "target_entitlement".to_string(),
        byte_array(&form_field_bytes(&context.target_entitlement)),
    );
    fields.insert(
        "old_interval".to_string(),
        byte_array(&form_field_bytes(&context.old_interval)),
    );
    fields.insert(
        "new_interval".to_string(),
        byte_array(&form_field_bytes(&context.new_interval)),
    );
    fields.insert("purpose".to_string(), Value::from(context.purpose.code()));
    fields.insert(
        "seller".to_string(),
        byte_array(&form_field_bytes(&context.seller)),
    );
    fields.insert(
        "buyer".to_string(),
        byte_array(&form_field_bytes(&context.buyer)),
    );
    fields.insert("expiry".to_string(), Value::from(context.expiry));
    fields
}

fn public_keys_value<P: IPairingAdapter>(
    pairing: &P,
    keys: &PublicKeysComponents<P::G1, P::G2>,
) -> Value {
    let mut fields = Map::new();
    fields.insert("pk1".to_string(), encoded_g1_value(pairing, &keys.pk1));
    fields.insert("pk2".to_string(), encoded_g2_value(pairing, &keys.pk2));
    Value::Object(fields)
}

fn envelope_value<P: IPairingAdapter>(
    pairing: &P,
    envelope: &EnvelopeComponents<P::G1, P::G2>,
) -> Value {
    let mut fields = Map::new();
    fields.insert("c1".to_string(), encoded_g1_value(pairing, &envelope.c1));
    fields.insert("c2".to_string(), encoded_g1_value(pairing, &envelope.c2));
    fields.insert("d1".to_string(), encoded_g2_value(pairing, &envelope.d1));
    fields.insert("d2".to_string(), encoded_g2_value(pairing, &envelope.d2));
    Value::Object(fields)
}

#[derive(Default)]
pub struct ProveMintParamsOverrides;

pub fn build_prove_mint_params(_overrides: ProveMintParamsOverrides) -> ProveMintParams {
    ProveMintParams
}

#[derive(Default)]
pub struct ProveTransferParamsOverrides;

pub fn build_prove_transfer_params(
    _overrides: ProveTransferParamsOverrides,
) -> ProveTransferParams {
    ProveTransferParams
}

#[derive(Default)]
pub struct VerifyParamsOverrides;

pub fn build_verify_params(_overrides: VerifyParamsOverrides) -> VerifyParams {
    VerifyParams
}

#[derive(Default)]
pub struct ProofComponentsParamsOverrides;

pub fn build_proof_components_params(
    _overrides: ProofComponentsParamsOverrides,
) -> ProofComponentsParams {
    ProofComponentsParams
}

#[derive(Default)]
pub struct ProofFromComponentsParamsOverrides;

pub fn build_proof_from_components_params(
    _overrides: ProofFromComponentsParamsOverrides,
) -> ProofFromComponentsParams {
    ProofFromComponentsParams
}

pub struct MintResponsesOverrides<S> {
    pub alpha: Option<S>,
    pub r: Option<S>,
    pub rho: Option<S>,
    pub sigma: Option<S>,
}

impl<S> Default for MintResponsesOverrides<S> {
    fn default() -> Self {
        Self {
            alpha: None,
            r: None,
            rho: None,
            sigma: None,
        }
    }
}

pub fn build_mint_responses<P: IPairingAdapter>(
    _pairing: &P,
    overrides: MintResponsesOverrides<P::Scalar>,
) -> MintResponses<P::Scalar> {
    MintResponses {
        alpha: overrides
            .alpha
            .unwrap_or_else(|| sampled_scalar::<P>(FILL_MINT_ALPHA)),
        r: overrides
            .r
            .unwrap_or_else(|| sampled_scalar::<P>(FILL_MINT_R)),
        rho: overrides
            .rho
            .unwrap_or_else(|| sampled_scalar::<P>(FILL_MINT_RHO)),
        sigma: overrides
            .sigma
            .unwrap_or_else(|| sampled_scalar::<P>(FILL_MINT_SIGMA)),
    }
}

#[derive(Default)]
pub struct MintResponsesCorruptions {
    pub alpha: Option<Value>,
    pub r: Option<Value>,
    pub rho: Option<Value>,
    pub sigma: Option<Value>,
}

pub fn invalidate_mint_responses<P: IPairingAdapter>(
    pairing: &P,
    corruptions: MintResponsesCorruptions,
) -> Value {
    let baseline = build_mint_responses(pairing, Default::default());
    let mut fields = Map::new();
    fields.insert(
        "alpha".to_string(),
        encoded_scalar_value(pairing, &baseline.alpha),
    );
    fields.insert("r".to_string(), encoded_scalar_value(pairing, &baseline.r));
    fields.insert(
        "rho".to_string(),
        encoded_scalar_value(pairing, &baseline.rho),
    );
    fields.insert(
        "sigma".to_string(),
        encoded_scalar_value(pairing, &baseline.sigma),
    );
    if let Some(alpha) = corruptions.alpha {
        fields.insert("alpha".to_string(), alpha);
    }
    if let Some(r) = corruptions.r {
        fields.insert("r".to_string(), r);
    }
    if let Some(rho) = corruptions.rho {
        fields.insert("rho".to_string(), rho);
    }
    if let Some(sigma) = corruptions.sigma {
        fields.insert("sigma".to_string(), sigma);
    }
    Value::Object(fields)
}

pub struct TransferResponsesOverrides<S> {
    pub x: Option<S>,
    pub y: Option<S>,
    pub s: Option<S>,
    pub rho: Option<S>,
    pub sigma: Option<S>,
}

impl<S> Default for TransferResponsesOverrides<S> {
    fn default() -> Self {
        Self {
            x: None,
            y: None,
            s: None,
            rho: None,
            sigma: None,
        }
    }
}

pub fn build_transfer_responses<P: IPairingAdapter>(
    _pairing: &P,
    overrides: TransferResponsesOverrides<P::Scalar>,
) -> TransferResponses<P::Scalar> {
    TransferResponses {
        x: overrides
            .x
            .unwrap_or_else(|| sampled_scalar::<P>(FILL_TRANSFER_X)),
        y: overrides
            .y
            .unwrap_or_else(|| sampled_scalar::<P>(FILL_TRANSFER_Y)),
        s: overrides
            .s
            .unwrap_or_else(|| sampled_scalar::<P>(FILL_TRANSFER_S)),
        rho: overrides
            .rho
            .unwrap_or_else(|| sampled_scalar::<P>(FILL_TRANSFER_RHO)),
        sigma: overrides
            .sigma
            .unwrap_or_else(|| sampled_scalar::<P>(FILL_TRANSFER_SIGMA)),
    }
}

#[derive(Default)]
pub struct TransferResponsesCorruptions {
    pub x: Option<Value>,
    pub y: Option<Value>,
    pub s: Option<Value>,
    pub rho: Option<Value>,
    pub sigma: Option<Value>,
}

pub fn invalidate_transfer_responses<P: IPairingAdapter>(
    pairing: &P,
    corruptions: TransferResponsesCorruptions,
) -> Value {
    let baseline = build_transfer_responses(pairing, Default::default());
    let mut fields = Map::new();
    fields.insert("x".to_string(), encoded_scalar_value(pairing, &baseline.x));
    fields.insert("y".to_string(), encoded_scalar_value(pairing, &baseline.y));
    fields.insert("s".to_string(), encoded_scalar_value(pairing, &baseline.s));
    fields.insert(
        "rho".to_string(),
        encoded_scalar_value(pairing, &baseline.rho),
    );
    fields.insert(
        "sigma".to_string(),
        encoded_scalar_value(pairing, &baseline.sigma),
    );
    if let Some(x) = corruptions.x {
        fields.insert("x".to_string(), x);
    }
    if let Some(y) = corruptions.y {
        fields.insert("y".to_string(), y);
    }
    if let Some(s) = corruptions.s {
        fields.insert("s".to_string(), s);
    }
    if let Some(rho) = corruptions.rho {
        fields.insert("rho".to_string(), rho);
    }
    if let Some(sigma) = corruptions.sigma {
        fields.insert("sigma".to_string(), sigma);
    }
    Value::Object(fields)
}

pub struct MintSecondGroupFirstMessagesOverrides<G2> {
    pub hpub: Option<G2>,
    pub d1: Option<G2>,
    pub d2: Option<G2>,
}

impl<G2> Default for MintSecondGroupFirstMessagesOverrides<G2> {
    fn default() -> Self {
        Self {
            hpub: None,
            d1: None,
            d2: None,
        }
    }
}

pub fn build_mint_second_group_first_messages<P: IPairingAdapter>(
    pairing: &P,
    overrides: MintSecondGroupFirstMessagesOverrides<P::G2>,
) -> MintSecondGroupFirstMessages<P::G2> {
    MintSecondGroupFirstMessages {
        hpub: overrides
            .hpub
            .unwrap_or_else(|| g2_mul(pairing, FILL_MINT_FM_HPUB)),
        d1: overrides
            .d1
            .unwrap_or_else(|| g2_mul(pairing, FILL_MINT_FM_D1)),
        d2: overrides
            .d2
            .unwrap_or_else(|| g2_mul(pairing, FILL_MINT_FM_D2)),
    }
}

#[derive(Default)]
pub struct MintSecondGroupFirstMessagesCorruptions {
    pub hpub: Option<Value>,
    pub d1: Option<Value>,
    pub d2: Option<Value>,
}

pub fn invalidate_mint_second_group_first_messages<P: IPairingAdapter>(
    pairing: &P,
    corruptions: MintSecondGroupFirstMessagesCorruptions,
) -> Value {
    let baseline = build_mint_second_group_first_messages(pairing, Default::default());
    let mut fields = Map::new();
    fields.insert(
        "hpub".to_string(),
        encoded_g2_value(pairing, &baseline.hpub),
    );
    fields.insert("d1".to_string(), encoded_g2_value(pairing, &baseline.d1));
    fields.insert("d2".to_string(), encoded_g2_value(pairing, &baseline.d2));
    if let Some(hpub) = corruptions.hpub {
        fields.insert("hpub".to_string(), hpub);
    }
    if let Some(d1) = corruptions.d1 {
        fields.insert("d1".to_string(), d1);
    }
    if let Some(d2) = corruptions.d2 {
        fields.insert("d2".to_string(), d2);
    }
    Value::Object(fields)
}

pub struct TransferSecondGroupFirstMessagesOverrides<G2> {
    pub pk2: Option<G2>,
    pub d1: Option<G2>,
    pub d2: Option<G2>,
}

impl<G2> Default for TransferSecondGroupFirstMessagesOverrides<G2> {
    fn default() -> Self {
        Self {
            pk2: None,
            d1: None,
            d2: None,
        }
    }
}

pub fn build_transfer_second_group_first_messages<P: IPairingAdapter>(
    pairing: &P,
    overrides: TransferSecondGroupFirstMessagesOverrides<P::G2>,
) -> TransferSecondGroupFirstMessages<P::G2> {
    TransferSecondGroupFirstMessages {
        pk2: overrides
            .pk2
            .unwrap_or_else(|| g2_mul(pairing, FILL_TRANSFER_FM_PK2)),
        d1: overrides
            .d1
            .unwrap_or_else(|| g2_mul(pairing, FILL_TRANSFER_FM_D1)),
        d2: overrides
            .d2
            .unwrap_or_else(|| g2_mul(pairing, FILL_TRANSFER_FM_D2)),
    }
}

#[derive(Default)]
pub struct TransferSecondGroupFirstMessagesCorruptions {
    pub pk2: Option<Value>,
    pub d1: Option<Value>,
    pub d2: Option<Value>,
}

pub fn invalidate_transfer_second_group_first_messages<P: IPairingAdapter>(
    pairing: &P,
    corruptions: TransferSecondGroupFirstMessagesCorruptions,
) -> Value {
    let baseline = build_transfer_second_group_first_messages(pairing, Default::default());
    let mut fields = Map::new();
    fields.insert("pk2".to_string(), encoded_g2_value(pairing, &baseline.pk2));
    fields.insert("d1".to_string(), encoded_g2_value(pairing, &baseline.d1));
    fields.insert("d2".to_string(), encoded_g2_value(pairing, &baseline.d2));
    if let Some(pk2) = corruptions.pk2 {
        fields.insert("pk2".to_string(), pk2);
    }
    if let Some(d1) = corruptions.d1 {
        fields.insert("d1".to_string(), d1);
    }
    if let Some(d2) = corruptions.d2 {
        fields.insert("d2".to_string(), d2);
    }
    Value::Object(fields)
}

pub struct DeliveryProofWireComponentsOverrides<S, G2> {
    pub relation: Option<DeliveryRelation>,
    pub verifier_form: Option<VerifierGroupArithmetic>,
    pub challenge: Option<S>,
    pub responses: Option<Vec<S>>,
    pub second_group_first_messages: Option<Vec<G2>>,
}

impl<S, G2> Default for DeliveryProofWireComponentsOverrides<S, G2> {
    fn default() -> Self {
        Self {
            relation: None,
            verifier_form: None,
            challenge: None,
            responses: None,
            second_group_first_messages: None,
        }
    }
}

pub fn build_delivery_proof_wire_components<P: IPairingAdapter>(
    pairing: &P,
    overrides: DeliveryProofWireComponentsOverrides<P::Scalar, P::G2>,
) -> DeliveryProofWireComponents<P::Scalar, P::G2> {
    DeliveryProofWireComponents {
        relation: overrides.relation.unwrap_or(DeliveryRelation::Mint),
        verifier_form: overrides
            .verifier_form
            .unwrap_or(VerifierGroupArithmetic::BothGroups),
        challenge: overrides
            .challenge
            .unwrap_or_else(|| sampled_scalar::<P>(FILL_WIRE_CHALLENGE)),
        responses: overrides.responses.unwrap_or_else(|| {
            let responses = build_mint_responses(pairing, Default::default());
            vec![responses.alpha, responses.r, responses.rho, responses.sigma]
        }),
        second_group_first_messages: overrides.second_group_first_messages.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct DeliveryProofWireComponentsCorruptions {
    pub relation: Option<Value>,
    pub verifier_form: Option<Value>,
    pub challenge: Option<Value>,
    pub responses: Option<Value>,
    pub second_group_first_messages: Option<Value>,
}

fn relation_name(relation: DeliveryRelation) -> &'static str {
    match relation {
        DeliveryRelation::Mint => "Mint",
        DeliveryRelation::Transfer => "Transfer",
    }
}

fn verifier_form_name(form: VerifierGroupArithmetic) -> &'static str {
    match form {
        VerifierGroupArithmetic::BothGroups => "BothGroups",
        VerifierGroupArithmetic::FirstGroupOnly => "FirstGroupOnly",
    }
}

pub fn invalidate_delivery_proof_wire_components<P: IPairingAdapter>(
    pairing: &P,
    corruptions: DeliveryProofWireComponentsCorruptions,
) -> Value {
    let baseline = build_delivery_proof_wire_components(pairing, Default::default());
    let mut fields = Map::new();
    fields.insert(
        "relation".to_string(),
        Value::from(relation_name(baseline.relation)),
    );
    fields.insert(
        "verifier_form".to_string(),
        Value::from(verifier_form_name(baseline.verifier_form)),
    );
    fields.insert(
        "challenge".to_string(),
        encoded_scalar_value(pairing, &baseline.challenge),
    );
    fields.insert(
        "responses".to_string(),
        Value::Array(
            baseline
                .responses
                .iter()
                .map(|scalar| encoded_scalar_value(pairing, scalar))
                .collect(),
        ),
    );
    fields.insert(
        "second_group_first_messages".to_string(),
        Value::Array(
            baseline
                .second_group_first_messages
                .iter()
                .map(|point| encoded_g2_value(pairing, point))
                .collect(),
        ),
    );
    if let Some(relation) = corruptions.relation {
        fields.insert("relation".to_string(), relation);
    }
    if let Some(verifier_form) = corruptions.verifier_form {
        fields.insert("verifier_form".to_string(), verifier_form);
    }
    if let Some(challenge) = corruptions.challenge {
        fields.insert("challenge".to_string(), challenge);
    }
    if let Some(responses) = corruptions.responses {
        fields.insert("responses".to_string(), responses);
    }
    if let Some(first_messages) = corruptions.second_group_first_messages {
        fields.insert("second_group_first_messages".to_string(), first_messages);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct DeliveryProofDeclarationOverrides {
    pub algebras: Option<&'static [EnvelopeAlgebra]>,
    pub verifier_forms: Option<&'static [VerifierGroupArithmetic]>,
    pub statement_versions: Option<&'static [u16]>,
    pub challenge_tag: Option<&'static [u8]>,
    pub weight_tag: Option<&'static [u8]>,
    pub adapter_version: Option<u32>,
    pub interface_version: Option<u32>,
}

pub fn build_delivery_proof_declaration(
    overrides: DeliveryProofDeclarationOverrides,
) -> DeliveryProofDeclaration {
    DeliveryProofDeclaration {
        algebras: overrides
            .algebras
            .unwrap_or(MOCK_I_DELIVERY_PROOF_ADAPTER_DECLARATION.algebras),
        verifier_forms: overrides
            .verifier_forms
            .unwrap_or(MOCK_I_DELIVERY_PROOF_ADAPTER_DECLARATION.verifier_forms),
        statement_versions: overrides
            .statement_versions
            .unwrap_or(MOCK_I_DELIVERY_PROOF_ADAPTER_DECLARATION.statement_versions),
        challenge_tag: overrides
            .challenge_tag
            .unwrap_or(MOCK_I_DELIVERY_PROOF_ADAPTER_DECLARATION.challenge_tag),
        weight_tag: overrides
            .weight_tag
            .unwrap_or(MOCK_I_DELIVERY_PROOF_ADAPTER_DECLARATION.weight_tag),
        adapter_version: overrides
            .adapter_version
            .unwrap_or(MOCK_I_DELIVERY_PROOF_ADAPTER_DECLARATION.adapter_version),
        interface_version: overrides
            .interface_version
            .unwrap_or(MOCK_I_DELIVERY_PROOF_ADAPTER_DECLARATION.interface_version),
    }
}

#[derive(Default)]
pub struct DeliveryProofDeclarationCorruptions {
    pub algebras: Option<Value>,
    pub verifier_forms: Option<Value>,
    pub statement_versions: Option<Value>,
    pub challenge_tag: Option<Value>,
    pub weight_tag: Option<Value>,
    pub adapter_version: Option<Value>,
    pub interface_version: Option<Value>,
}

pub fn invalidate_delivery_proof_declaration(
    corruptions: DeliveryProofDeclarationCorruptions,
) -> Value {
    let baseline = build_delivery_proof_declaration(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "algebras".to_string(),
        Value::Array(
            baseline
                .algebras
                .iter()
                .map(|algebra| {
                    Value::from(match algebra {
                        EnvelopeAlgebra::PairingElGamal => "PairingElGamal",
                    })
                })
                .collect(),
        ),
    );
    fields.insert(
        "verifier_forms".to_string(),
        Value::Array(
            baseline
                .verifier_forms
                .iter()
                .map(|form| Value::from(verifier_form_name(*form)))
                .collect(),
        ),
    );
    fields.insert(
        "statement_versions".to_string(),
        Value::Array(
            baseline
                .statement_versions
                .iter()
                .map(|version| Value::from(*version))
                .collect(),
        ),
    );
    fields.insert(
        "challenge_tag".to_string(),
        byte_array(baseline.challenge_tag),
    );
    fields.insert("weight_tag".to_string(), byte_array(baseline.weight_tag));
    fields.insert(
        "adapter_version".to_string(),
        Value::from(baseline.adapter_version),
    );
    fields.insert(
        "interface_version".to_string(),
        Value::from(baseline.interface_version),
    );
    if let Some(algebras) = corruptions.algebras {
        fields.insert("algebras".to_string(), algebras);
    }
    if let Some(verifier_forms) = corruptions.verifier_forms {
        fields.insert("verifier_forms".to_string(), verifier_forms);
    }
    if let Some(statement_versions) = corruptions.statement_versions {
        fields.insert("statement_versions".to_string(), statement_versions);
    }
    if let Some(challenge_tag) = corruptions.challenge_tag {
        fields.insert("challenge_tag".to_string(), challenge_tag);
    }
    if let Some(weight_tag) = corruptions.weight_tag {
        fields.insert("weight_tag".to_string(), weight_tag);
    }
    if let Some(adapter_version) = corruptions.adapter_version {
        fields.insert("adapter_version".to_string(), adapter_version);
    }
    if let Some(interface_version) = corruptions.interface_version {
        fields.insert("interface_version".to_string(), interface_version);
    }
    Value::Object(fields)
}

pub struct DeliveryContextOverrides<F: IChainForms> {
    pub suite_identifier: Option<domain::SuiteIdentifier>,
    pub chain: Option<F::ChainIdentifier>,
    pub entitlement_contract: Option<F::Identity>,
    pub asset_root: Option<hashing::Root>,
    pub parameter_set_identifier: Option<domain::ParameterSetIdentifier>,
    pub source_entitlement: Option<F::Entitlement>,
    pub target_entitlement: Option<F::Entitlement>,
    pub old_interval: Option<F::Interval>,
    pub new_interval: Option<F::Interval>,
    pub seller: Option<F::Identity>,
    pub buyer: Option<F::Identity>,
    pub expiry: Option<u64>,
}

impl<F: IChainForms> Default for DeliveryContextOverrides<F> {
    fn default() -> Self {
        Self {
            suite_identifier: None,
            chain: None,
            entitlement_contract: None,
            asset_root: None,
            parameter_set_identifier: None,
            source_entitlement: None,
            target_entitlement: None,
            old_interval: None,
            new_interval: None,
            seller: None,
            buyer: None,
            expiry: None,
        }
    }
}

pub fn build_delivery_context<F: IChainForms, Purpose>(
    purpose: Purpose,
    overrides: DeliveryContextOverrides<F>,
) -> DeliveryContext<F, Purpose> {
    DeliveryContext {
        suite_identifier: overrides.suite_identifier.unwrap_or_else(|| {
            build_suite_identifier(SuiteIdentifierConstructorParamsOverrides::default())
        }),
        chain: overrides
            .chain
            .unwrap_or_else(|| form_from_seed::<F::ChainIdentifier>(FILL_CHAIN)),
        entitlement_contract: overrides
            .entitlement_contract
            .unwrap_or_else(|| form_from_seed::<F::Identity>(FILL_ENTITLEMENT_CONTRACT)),
        asset_root: overrides
            .asset_root
            .unwrap_or_else(|| build_root(RootConstructorParamsOverrides::default())),
        parameter_set_identifier: overrides.parameter_set_identifier.unwrap_or_else(|| {
            build_parameter_set_identifier(
                ParameterSetIdentifierConstructorParamsOverrides::default(),
            )
        }),
        source_entitlement: overrides
            .source_entitlement
            .unwrap_or_else(|| form_from_seed::<F::Entitlement>(FILL_SOURCE_ENTITLEMENT)),
        target_entitlement: overrides
            .target_entitlement
            .unwrap_or_else(|| form_from_seed::<F::Entitlement>(FILL_TARGET_ENTITLEMENT)),
        old_interval: overrides
            .old_interval
            .unwrap_or_else(|| form_from_seed::<F::Interval>(FILL_OLD_INTERVAL)),
        new_interval: overrides
            .new_interval
            .unwrap_or_else(|| form_from_seed::<F::Interval>(FILL_NEW_INTERVAL)),
        purpose,
        seller: overrides
            .seller
            .unwrap_or_else(|| form_from_seed::<F::Identity>(FILL_SELLER)),
        buyer: overrides
            .buyer
            .unwrap_or_else(|| form_from_seed::<F::Identity>(FILL_BUYER)),
        expiry: overrides.expiry.unwrap_or(1_700_000_000),
    }
}

fn mint_envelope_default<P: IPairingArithmetic>(
    pairing: &P,
    alpha: &P::Scalar,
    randomness: &P::Scalar,
    identity_element: &P::G1,
    buyer_keys: &PublicKeysComponents<P::G1, P::G2>,
    coins: &EnvelopeCoins<P::Scalar>,
) -> EnvelopeComponents<P::G1, P::G2> {
    let g1 = default_g1(pairing);
    let g2 = default_g2(pairing);
    let rho = coins.rho.expose().clone();
    let sigma = coins.sigma.expose().clone();
    EnvelopeComponents {
        c1: mul_g1_by(pairing, g1.clone(), rho.clone()),
        c2: msm_g1_of(
            pairing,
            vec![
                (g1, alpha.clone()),
                (identity_element.clone(), randomness.clone()),
                (buyer_keys.pk1.clone(), rho),
            ],
        ),
        d1: mul_g2_by(pairing, g2.clone(), sigma.clone()),
        d2: msm_g2_of(
            pairing,
            vec![(g2, randomness.clone()), (buyer_keys.pk2.clone(), sigma)],
        ),
    }
}

pub struct AlgebraicMintStatementOverrides<P: IPairingAdapter, F: IChainForms> {
    pub context: Option<DeliveryContext<F, MintPurpose>>,
    pub hpub: Option<P::G2>,
    pub identity_element: Option<P::G1>,
    pub buyer_keys: Option<PublicKeysComponents<P::G1, P::G2>>,
    pub envelope: Option<EnvelopeComponents<P::G1, P::G2>>,
}

impl<P: IPairingAdapter, F: IChainForms> Default for AlgebraicMintStatementOverrides<P, F> {
    fn default() -> Self {
        Self {
            context: None,
            hpub: None,
            identity_element: None,
            buyer_keys: None,
            envelope: None,
        }
    }
}

pub fn build_algebraic_mint_statement<P: IPairingArithmetic, F: IChainForms>(
    pairing: &P,
    overrides: AlgebraicMintStatementOverrides<P, F>,
) -> AlgebraicMintStatement<F, P::G1, P::G2> {
    let alpha = build_master_scalar_components::<P::Scalar>(Default::default())
        .value
        .expose()
        .clone();
    let randomness = sampled_scalar::<P>(FILL_CREDENTIAL_RANDOMNESS);
    let coins = build_envelope_coins(pairing, Default::default());
    let identity_element = overrides
        .identity_element
        .unwrap_or_else(|| g1_mul(pairing, FILL_IDENTITY_ELEMENT));
    let buyer_keys = overrides
        .buyer_keys
        .unwrap_or_else(|| build_public_keys_components(pairing, Default::default()));
    AlgebraicMintStatement {
        context: overrides.context.unwrap_or_else(|| {
            build_delivery_context(MintPurpose::Mint, DeliveryContextOverrides::default())
        }),
        hpub: overrides
            .hpub
            .unwrap_or_else(|| mul_g2_by(pairing, default_g2(pairing), alpha.clone())),
        identity_element: identity_element.clone(),
        envelope: overrides.envelope.unwrap_or_else(|| {
            mint_envelope_default(
                pairing,
                &alpha,
                &randomness,
                &identity_element,
                &buyer_keys,
                &coins,
            )
        }),
        buyer_keys,
    }
}

#[derive(Default)]
pub struct AlgebraicMintStatementCorruptions {
    pub context: Option<Value>,
    pub hpub: Option<Value>,
    pub identity_element: Option<Value>,
    pub buyer_keys: Option<Value>,
    pub envelope: Option<Value>,
}

pub fn invalidate_algebraic_mint_statement<P: IPairingArithmetic, F: IChainForms>(
    pairing: &P,
    corruptions: AlgebraicMintStatementCorruptions,
) -> Value {
    let baseline =
        build_algebraic_mint_statement::<P, F>(pairing, AlgebraicMintStatementOverrides::default());
    let mut fields = Map::new();
    fields.insert(
        "context".to_string(),
        Value::Object(context_mint_rendered(&baseline.context)),
    );
    fields.insert(
        "hpub".to_string(),
        encoded_g2_value(pairing, &baseline.hpub),
    );
    fields.insert(
        "identity_element".to_string(),
        encoded_g1_value(pairing, &baseline.identity_element),
    );
    fields.insert(
        "buyer_keys".to_string(),
        public_keys_value(pairing, &baseline.buyer_keys),
    );
    fields.insert(
        "envelope".to_string(),
        envelope_value(pairing, &baseline.envelope),
    );
    if let Some(context) = corruptions.context {
        fields.insert("context".to_string(), context);
    }
    if let Some(hpub) = corruptions.hpub {
        fields.insert("hpub".to_string(), hpub);
    }
    if let Some(identity_element) = corruptions.identity_element {
        fields.insert("identity_element".to_string(), identity_element);
    }
    if let Some(buyer_keys) = corruptions.buyer_keys {
        fields.insert("buyer_keys".to_string(), buyer_keys);
    }
    if let Some(envelope) = corruptions.envelope {
        fields.insert("envelope".to_string(), envelope);
    }
    Value::Object(fields)
}

pub struct AlgebraicTransferStatementOverrides<P: IPairingAdapter, F: IChainForms> {
    pub context: Option<DeliveryContext<F, TransferPurpose>>,
    pub identity_element: Option<P::G1>,
    pub seller_keys: Option<PublicKeysComponents<P::G1, P::G2>>,
    pub buyer_keys: Option<PublicKeysComponents<P::G1, P::G2>>,
    pub old_envelope: Option<EnvelopeComponents<P::G1, P::G2>>,
    pub new_envelope: Option<EnvelopeComponents<P::G1, P::G2>>,
}

impl<P: IPairingAdapter, F: IChainForms> Default for AlgebraicTransferStatementOverrides<P, F> {
    fn default() -> Self {
        Self {
            context: None,
            identity_element: None,
            seller_keys: None,
            buyer_keys: None,
            old_envelope: None,
            new_envelope: None,
        }
    }
}

fn transfer_new_envelope_default<P: IPairingArithmetic>(
    pairing: &P,
    offset: &P::Scalar,
    identity_element: &P::G1,
    seller_secrets: &KeyPairComponents<P::Scalar>,
    buyer_keys: &PublicKeysComponents<P::G1, P::G2>,
    old_envelope: &EnvelopeComponents<P::G1, P::G2>,
    coins: &EnvelopeCoins<P::Scalar>,
) -> EnvelopeComponents<P::G1, P::G2> {
    let g1 = default_g1(pairing);
    let g2 = default_g2(pairing);
    let x = seller_secrets.x.expose().clone();
    let y = seller_secrets.y.expose().clone();
    let rho = coins.rho.expose().clone();
    let sigma = coins.sigma.expose().clone();
    EnvelopeComponents {
        c1: mul_g1_by(pairing, g1, rho.clone()),
        c2: add_g1_of(
            pairing,
            old_envelope.c2.clone(),
            msm_g1_of(
                pairing,
                vec![
                    (old_envelope.c1.clone(), neg_scalar_of(pairing, x)),
                    (identity_element.clone(), offset.clone()),
                    (buyer_keys.pk1.clone(), rho),
                ],
            ),
        ),
        d1: mul_g2_by(pairing, g2.clone(), sigma.clone()),
        d2: add_g2_of(
            pairing,
            old_envelope.d2.clone(),
            msm_g2_of(
                pairing,
                vec![
                    (old_envelope.d1.clone(), neg_scalar_of(pairing, y)),
                    (g2, offset.clone()),
                    (buyer_keys.pk2.clone(), sigma),
                ],
            ),
        ),
    }
}

pub fn build_algebraic_transfer_statement<P: IPairingArithmetic, F: IChainForms>(
    pairing: &P,
    overrides: AlgebraicTransferStatementOverrides<P, F>,
) -> AlgebraicTransferStatement<F, P::G1, P::G2> {
    let seller_secrets = build_key_pair_components(pairing, Default::default());
    let offset = sampled_scalar::<P>(FILL_OFFSET);
    let coins = build_envelope_coins(pairing, Default::default());
    let identity_element = overrides
        .identity_element
        .unwrap_or_else(|| g1_mul(pairing, FILL_IDENTITY_ELEMENT));
    let seller_keys = overrides.seller_keys.unwrap_or_else(|| {
        build_public_keys_components(
            pairing,
            PublicKeysComponentsOverrides {
                pk1: Some(mul_g1_by(
                    pairing,
                    default_g1(pairing),
                    seller_secrets.x.expose().clone(),
                )),
                pk2: Some(mul_g2_by(
                    pairing,
                    default_g2(pairing),
                    seller_secrets.y.expose().clone(),
                )),
            },
        )
    });
    let buyer_keys = overrides.buyer_keys.unwrap_or_else(|| {
        build_public_keys_components(
            pairing,
            PublicKeysComponentsOverrides {
                pk1: Some(g1_mul(pairing, FILL_TRANSFER_BUYER_PK1)),
                pk2: Some(g2_mul(pairing, FILL_TRANSFER_BUYER_PK2)),
            },
        )
    });
    let old_envelope = overrides.old_envelope.unwrap_or_else(|| {
        build_envelope_components(
            pairing,
            EnvelopeComponentsOverrides {
                c1: Some(g1_mul(pairing, FILL_OLD_C1)),
                c2: Some(g1_mul(pairing, FILL_OLD_C2)),
                d1: Some(g2_mul(pairing, FILL_OLD_D1)),
                d2: Some(g2_mul(pairing, FILL_OLD_D2)),
            },
        )
    });
    AlgebraicTransferStatement {
        context: overrides.context.unwrap_or_else(|| {
            build_delivery_context(
                TransferPurpose::Transfer,
                DeliveryContextOverrides::default(),
            )
        }),
        identity_element: identity_element.clone(),
        new_envelope: overrides.new_envelope.unwrap_or_else(|| {
            transfer_new_envelope_default(
                pairing,
                &offset,
                &identity_element,
                &seller_secrets,
                &buyer_keys,
                &old_envelope,
                &coins,
            )
        }),
        seller_keys,
        buyer_keys,
        old_envelope,
    }
}

#[derive(Default)]
pub struct AlgebraicTransferStatementCorruptions {
    pub context: Option<Value>,
    pub identity_element: Option<Value>,
    pub seller_keys: Option<Value>,
    pub buyer_keys: Option<Value>,
    pub old_envelope: Option<Value>,
    pub new_envelope: Option<Value>,
}

pub fn invalidate_algebraic_transfer_statement<P: IPairingArithmetic, F: IChainForms>(
    pairing: &P,
    corruptions: AlgebraicTransferStatementCorruptions,
) -> Value {
    let baseline = build_algebraic_transfer_statement::<P, F>(
        pairing,
        AlgebraicTransferStatementOverrides::default(),
    );
    let mut fields = Map::new();
    fields.insert(
        "context".to_string(),
        Value::Object(context_transfer_rendered(&baseline.context)),
    );
    fields.insert(
        "identity_element".to_string(),
        encoded_g1_value(pairing, &baseline.identity_element),
    );
    fields.insert(
        "seller_keys".to_string(),
        public_keys_value(pairing, &baseline.seller_keys),
    );
    fields.insert(
        "buyer_keys".to_string(),
        public_keys_value(pairing, &baseline.buyer_keys),
    );
    fields.insert(
        "old_envelope".to_string(),
        envelope_value(pairing, &baseline.old_envelope),
    );
    fields.insert(
        "new_envelope".to_string(),
        envelope_value(pairing, &baseline.new_envelope),
    );
    if let Some(context) = corruptions.context {
        fields.insert("context".to_string(), context);
    }
    if let Some(identity_element) = corruptions.identity_element {
        fields.insert("identity_element".to_string(), identity_element);
    }
    if let Some(seller_keys) = corruptions.seller_keys {
        fields.insert("seller_keys".to_string(), seller_keys);
    }
    if let Some(buyer_keys) = corruptions.buyer_keys {
        fields.insert("buyer_keys".to_string(), buyer_keys);
    }
    if let Some(old_envelope) = corruptions.old_envelope {
        fields.insert("old_envelope".to_string(), old_envelope);
    }
    if let Some(new_envelope) = corruptions.new_envelope {
        fields.insert("new_envelope".to_string(), new_envelope);
    }
    Value::Object(fields)
}

pub struct ProveMintPayloadOverrides<S: Zeroize> {
    pub master_scalar: Option<MasterScalarComponents<S>>,
    pub credential_randomness: Option<Secret<S>>,
    pub coins: Option<EnvelopeCoins<S>>,
}

impl<S: Zeroize> Default for ProveMintPayloadOverrides<S> {
    fn default() -> Self {
        Self {
            master_scalar: None,
            credential_randomness: None,
            coins: None,
        }
    }
}

pub fn build_prove_mint_payload<'a, P: IPairingArithmetic, F: IChainForms>(
    pairing: &P,
    statement: &'a AlgebraicMintStatement<F, P::G1, P::G2>,
    overrides: ProveMintPayloadOverrides<P::Scalar>,
) -> ProveMintPayload<'a, F, P::Scalar, P::G1, P::G2> {
    ProveMintPayload {
        master_scalar: overrides
            .master_scalar
            .unwrap_or_else(|| build_master_scalar_components::<P::Scalar>(Default::default())),
        credential_randomness: overrides.credential_randomness.unwrap_or_else(|| {
            build_secret(
                sampled_scalar::<P>(FILL_CREDENTIAL_RANDOMNESS),
                SecretConstructorParamsOverrides,
            )
        }),
        coins: overrides
            .coins
            .unwrap_or_else(|| build_envelope_coins(pairing, Default::default())),
        statement,
    }
}

pub struct ProveTransferPayloadOverrides<S: Zeroize> {
    pub seller_secrets: Option<KeyPairComponents<S>>,
    pub offset: Option<Secret<S>>,
    pub coins: Option<EnvelopeCoins<S>>,
}

impl<S: Zeroize> Default for ProveTransferPayloadOverrides<S> {
    fn default() -> Self {
        Self {
            seller_secrets: None,
            offset: None,
            coins: None,
        }
    }
}

pub fn build_prove_transfer_payload<'a, P: IPairingArithmetic, F: IChainForms>(
    pairing: &P,
    statement: &'a AlgebraicTransferStatement<F, P::G1, P::G2>,
    overrides: ProveTransferPayloadOverrides<P::Scalar>,
) -> ProveTransferPayload<'a, F, P::Scalar, P::G1, P::G2> {
    ProveTransferPayload {
        seller_secrets: overrides
            .seller_secrets
            .unwrap_or_else(|| build_key_pair_components(pairing, Default::default())),
        offset: overrides.offset.unwrap_or_else(|| {
            build_secret(
                sampled_scalar::<P>(FILL_OFFSET),
                SecretConstructorParamsOverrides,
            )
        }),
        coins: overrides
            .coins
            .unwrap_or_else(|| build_envelope_coins(pairing, Default::default())),
        statement,
    }
}

#[derive(Default)]
pub struct VerifyPayloadOverrides;

pub fn build_verify_payload<'a, F: IChainForms, G1, G2, PR>(
    statement: AlgebraicStatement<'a, F, G1, G2>,
    proof: &'a PR,
    _overrides: VerifyPayloadOverrides,
) -> VerifyPayload<'a, F, G1, G2, PR> {
    VerifyPayload { statement, proof }
}

#[derive(Default)]
pub struct ProofComponentsPayloadOverrides;

pub fn build_proof_components_payload<'a, PR>(
    proof: &'a PR,
    _overrides: ProofComponentsPayloadOverrides,
) -> ProofComponentsPayload<'a, PR> {
    ProofComponentsPayload { proof }
}

#[derive(Default)]
pub struct ProofFromComponentsPayloadOverrides;

pub fn build_proof_from_components_payload<S, G2>(
    components: DeliveryProofComponents<S, G2>,
    _overrides: ProofFromComponentsPayloadOverrides,
) -> ProofFromComponentsPayload<S, G2> {
    ProofFromComponentsPayload { components }
}

#[derive(Default)]
pub struct ProveMintSuccessReturnOverrides;

pub fn build_prove_mint_success_return<PR>(
    proof: PR,
    _overrides: ProveMintSuccessReturnOverrides,
) -> ProveMintSuccessReturn<PR> {
    ProveMintSuccessReturn { proof }
}

#[derive(Default)]
pub struct ProveTransferSuccessReturnOverrides;

pub fn build_prove_transfer_success_return<PR>(
    proof: PR,
    _overrides: ProveTransferSuccessReturnOverrides,
) -> ProveTransferSuccessReturn<PR> {
    ProveTransferSuccessReturn { proof }
}

#[derive(Default)]
pub struct VerifySuccessReturnOverrides {
    pub is_valid: Option<bool>,
}

pub fn build_verify_success_return(overrides: VerifySuccessReturnOverrides) -> VerifySuccessReturn {
    VerifySuccessReturn {
        is_valid: overrides.is_valid.unwrap_or(true),
    }
}

#[derive(Default)]
pub struct VerifySuccessReturnCorruptions {
    pub is_valid: Option<Value>,
}

pub fn invalidate_verify_success_return(corruptions: VerifySuccessReturnCorruptions) -> Value {
    let baseline = build_verify_success_return(Default::default());
    let mut fields = Map::new();
    fields.insert("is_valid".to_string(), Value::from(baseline.is_valid));
    if let Some(is_valid) = corruptions.is_valid {
        fields.insert("is_valid".to_string(), is_valid);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct ProofComponentsSuccessReturnOverrides;

pub fn build_proof_components_success_return<S, G2>(
    components: DeliveryProofComponents<S, G2>,
    _overrides: ProofComponentsSuccessReturnOverrides,
) -> ProofComponentsSuccessReturn<S, G2> {
    ProofComponentsSuccessReturn { components }
}

#[derive(Default)]
pub struct ProofFromComponentsSuccessReturnOverrides;

pub fn build_proof_from_components_success_return<PR>(
    proof: PR,
    _overrides: ProofFromComponentsSuccessReturnOverrides,
) -> ProofFromComponentsSuccessReturn<PR> {
    ProofFromComponentsSuccessReturn { proof }
}

pub struct MockDeliveryProof<P: IPairingAdapter> {
    pub components: DeliveryProofComponents<P::Scalar, P::G2>,
}

pub struct MockDeliveryProofOverrides<P: IPairingAdapter> {
    pub components: Option<DeliveryProofComponents<P::Scalar, P::G2>>,
}

impl<P: IPairingAdapter> Default for MockDeliveryProofOverrides<P> {
    fn default() -> Self {
        Self { components: None }
    }
}

pub fn build_mock_delivery_proof<P: IPairingAdapter>(
    pairing: &P,
    overrides: MockDeliveryProofOverrides<P>,
) -> MockDeliveryProof<P> {
    MockDeliveryProof {
        components: overrides.components.unwrap_or_else(|| {
            DeliveryProofComponents::MintBothGroups {
                challenge: sampled_scalar::<P>(FILL_WIRE_CHALLENGE),
                responses: build_mint_responses(pairing, Default::default()),
            }
        }),
    }
}

#[derive(Default)]
pub struct MockDeliveryProofCorruptions {
    pub components: Option<Value>,
}

fn mint_responses_value<P: IPairingAdapter>(
    pairing: &P,
    responses: &MintResponses<P::Scalar>,
) -> Value {
    let mut fields = Map::new();
    fields.insert(
        "alpha".to_string(),
        encoded_scalar_value(pairing, &responses.alpha),
    );
    fields.insert("r".to_string(), encoded_scalar_value(pairing, &responses.r));
    fields.insert(
        "rho".to_string(),
        encoded_scalar_value(pairing, &responses.rho),
    );
    fields.insert(
        "sigma".to_string(),
        encoded_scalar_value(pairing, &responses.sigma),
    );
    Value::Object(fields)
}

fn transfer_responses_value<P: IPairingAdapter>(
    pairing: &P,
    responses: &TransferResponses<P::Scalar>,
) -> Value {
    let mut fields = Map::new();
    fields.insert("x".to_string(), encoded_scalar_value(pairing, &responses.x));
    fields.insert("y".to_string(), encoded_scalar_value(pairing, &responses.y));
    fields.insert("s".to_string(), encoded_scalar_value(pairing, &responses.s));
    fields.insert(
        "rho".to_string(),
        encoded_scalar_value(pairing, &responses.rho),
    );
    fields.insert(
        "sigma".to_string(),
        encoded_scalar_value(pairing, &responses.sigma),
    );
    Value::Object(fields)
}

fn mint_first_messages_value<P: IPairingAdapter>(
    pairing: &P,
    first_messages: &MintSecondGroupFirstMessages<P::G2>,
) -> Value {
    let mut fields = Map::new();
    fields.insert(
        "hpub".to_string(),
        encoded_g2_value(pairing, &first_messages.hpub),
    );
    fields.insert(
        "d1".to_string(),
        encoded_g2_value(pairing, &first_messages.d1),
    );
    fields.insert(
        "d2".to_string(),
        encoded_g2_value(pairing, &first_messages.d2),
    );
    Value::Object(fields)
}

fn transfer_first_messages_value<P: IPairingAdapter>(
    pairing: &P,
    first_messages: &TransferSecondGroupFirstMessages<P::G2>,
) -> Value {
    let mut fields = Map::new();
    fields.insert(
        "pk2".to_string(),
        encoded_g2_value(pairing, &first_messages.pk2),
    );
    fields.insert(
        "d1".to_string(),
        encoded_g2_value(pairing, &first_messages.d1),
    );
    fields.insert(
        "d2".to_string(),
        encoded_g2_value(pairing, &first_messages.d2),
    );
    Value::Object(fields)
}

fn components_value<P: IPairingAdapter>(
    pairing: &P,
    components: &DeliveryProofComponents<P::Scalar, P::G2>,
) -> Value {
    let mut fields = Map::new();
    match components {
        DeliveryProofComponents::MintBothGroups {
            challenge,
            responses,
        } => {
            fields.insert("variant".to_string(), Value::from("MintBothGroups"));
            fields.insert(
                "challenge".to_string(),
                encoded_scalar_value(pairing, challenge),
            );
            fields.insert(
                "responses".to_string(),
                mint_responses_value(pairing, responses),
            );
        }
        DeliveryProofComponents::MintFirstGroupOnly {
            challenge,
            responses,
            first_messages,
        } => {
            fields.insert("variant".to_string(), Value::from("MintFirstGroupOnly"));
            fields.insert(
                "challenge".to_string(),
                encoded_scalar_value(pairing, challenge),
            );
            fields.insert(
                "responses".to_string(),
                mint_responses_value(pairing, responses),
            );
            fields.insert(
                "first_messages".to_string(),
                mint_first_messages_value(pairing, first_messages),
            );
        }
        DeliveryProofComponents::TransferBothGroups {
            challenge,
            responses,
        } => {
            fields.insert("variant".to_string(), Value::from("TransferBothGroups"));
            fields.insert(
                "challenge".to_string(),
                encoded_scalar_value(pairing, challenge),
            );
            fields.insert(
                "responses".to_string(),
                transfer_responses_value(pairing, responses),
            );
        }
        DeliveryProofComponents::TransferFirstGroupOnly {
            challenge,
            responses,
            first_messages,
        } => {
            fields.insert("variant".to_string(), Value::from("TransferFirstGroupOnly"));
            fields.insert(
                "challenge".to_string(),
                encoded_scalar_value(pairing, challenge),
            );
            fields.insert(
                "responses".to_string(),
                transfer_responses_value(pairing, responses),
            );
            fields.insert(
                "first_messages".to_string(),
                transfer_first_messages_value(pairing, first_messages),
            );
        }
    }
    Value::Object(fields)
}

pub fn invalidate_mock_delivery_proof<P: IPairingAdapter>(
    pairing: &P,
    corruptions: MockDeliveryProofCorruptions,
) -> Value {
    let baseline = build_mock_delivery_proof(pairing, Default::default());
    let mut fields = Map::new();
    fields.insert(
        "components".to_string(),
        components_value(pairing, &baseline.components),
    );
    if let Some(components) = corruptions.components {
        fields.insert("components".to_string(), components);
    }
    Value::Object(fields)
}

pub(crate) struct MockIDeliveryProofAdapter<'a, P: IPairingArithmetic, F: IChainForms> {
    pairing: &'a P,
    verifier_form: VerifierGroupArithmetic,
    failure_mode: MockIDeliveryProofAdapterFailureMode,
    forms: PhantomData<F>,
}

pub struct MockIDeliveryProofAdapterConstructorParams<'a, P: IPairingArithmetic> {
    pub pairing: &'a P,
    pub verifier_form: VerifierGroupArithmetic,
    pub failure_mode: MockIDeliveryProofAdapterFailureMode,
}

#[derive(Default)]
pub struct MockIDeliveryProofAdapterConstructorParamsOverrides {
    pub verifier_form: Option<VerifierGroupArithmetic>,
    pub failure_mode: Option<MockIDeliveryProofAdapterFailureMode>,
}

pub fn build_mock_i_delivery_proof_adapter_constructor_params<'a, P: IPairingArithmetic>(
    pairing: &'a P,
    overrides: MockIDeliveryProofAdapterConstructorParamsOverrides,
) -> MockIDeliveryProofAdapterConstructorParams<'a, P> {
    MockIDeliveryProofAdapterConstructorParams {
        pairing,
        verifier_form: overrides
            .verifier_form
            .unwrap_or(VerifierGroupArithmetic::BothGroups),
        failure_mode: overrides
            .failure_mode
            .unwrap_or(MockIDeliveryProofAdapterFailureMode::NoFailure),
    }
}

impl<'a, P: IPairingArithmetic, F: IChainForms> MockIDeliveryProofAdapter<'a, P, F> {
    pub fn try_new(
        params: MockIDeliveryProofAdapterConstructorParams<'a, P>,
    ) -> Result<Self, Infallible> {
        Ok(Self {
            pairing: params.pairing,
            verifier_form: params.verifier_form,
            failure_mode: params.failure_mode,
            forms: PhantomData,
        })
    }
}

pub(crate) fn build_mock_i_delivery_proof_adapter<'a, P: IPairingArithmetic, F: IChainForms>(
    pairing: &'a P,
    overrides: MockIDeliveryProofAdapterConstructorParamsOverrides,
) -> MockIDeliveryProofAdapter<'a, P, F> {
    let Ok(adapter) = MockIDeliveryProofAdapter::try_new(
        build_mock_i_delivery_proof_adapter_constructor_params(pairing, overrides),
    );
    adapter
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

fn mint_first_messages_recomputed<P: IPairingArithmetic, F: IChainForms>(
    pairing: &P,
    n: &P::Scalar,
    responses: &MintResponses<P::Scalar>,
    statement: &AlgebraicMintStatement<F, P::G1, P::G2>,
) -> MintFirstMessages<P::G1, P::G2> {
    let g1 = default_g1(pairing);
    let g2 = default_g2(pairing);
    MintFirstMessages {
        hpub: msm_g2_of(
            pairing,
            vec![
                (g2.clone(), responses.alpha.clone()),
                (statement.hpub.clone(), n.clone()),
            ],
        ),
        c1: msm_g1_of(
            pairing,
            vec![
                (g1.clone(), responses.rho.clone()),
                (statement.envelope.c1.clone(), n.clone()),
            ],
        ),
        c2: msm_g1_of(
            pairing,
            vec![
                (g1, responses.alpha.clone()),
                (statement.identity_element.clone(), responses.r.clone()),
                (statement.buyer_keys.pk1.clone(), responses.rho.clone()),
                (statement.envelope.c2.clone(), n.clone()),
            ],
        ),
        d1: msm_g2_of(
            pairing,
            vec![
                (g2.clone(), responses.sigma.clone()),
                (statement.envelope.d1.clone(), n.clone()),
            ],
        ),
        d2: msm_g2_of(
            pairing,
            vec![
                (g2, responses.r.clone()),
                (statement.buyer_keys.pk2.clone(), responses.sigma.clone()),
                (statement.envelope.d2.clone(), n.clone()),
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

fn transfer_first_messages_recomputed<P: IPairingArithmetic, F: IChainForms>(
    pairing: &P,
    n: &P::Scalar,
    responses: &TransferResponses<P::Scalar>,
    statement: &AlgebraicTransferStatement<F, P::G1, P::G2>,
) -> TransferFirstMessages<P::G1, P::G2> {
    let g1 = default_g1(pairing);
    let g2 = default_g2(pairing);
    let delta_c2 = add_g1_of(
        pairing,
        statement.new_envelope.c2.clone(),
        neg_g1_of(pairing, statement.old_envelope.c2.clone()),
    );
    let delta_d2 = add_g2_of(
        pairing,
        statement.new_envelope.d2.clone(),
        neg_g2_of(pairing, statement.old_envelope.d2.clone()),
    );
    TransferFirstMessages {
        pk1: msm_g1_of(
            pairing,
            vec![
                (g1.clone(), responses.x.clone()),
                (statement.seller_keys.pk1.clone(), n.clone()),
            ],
        ),
        c1: msm_g1_of(
            pairing,
            vec![
                (g1.clone(), responses.rho.clone()),
                (statement.new_envelope.c1.clone(), n.clone()),
            ],
        ),
        c2: msm_g1_of(
            pairing,
            vec![
                (
                    statement.old_envelope.c1.clone(),
                    neg_scalar_of(pairing, responses.x.clone()),
                ),
                (statement.identity_element.clone(), responses.s.clone()),
                (statement.buyer_keys.pk1.clone(), responses.rho.clone()),
                (delta_c2, n.clone()),
            ],
        ),
        pk2: msm_g2_of(
            pairing,
            vec![
                (g2.clone(), responses.y.clone()),
                (statement.seller_keys.pk2.clone(), n.clone()),
            ],
        ),
        d1: msm_g2_of(
            pairing,
            vec![
                (g2.clone(), responses.sigma.clone()),
                (statement.new_envelope.d1.clone(), n.clone()),
            ],
        ),
        d2: msm_g2_of(
            pairing,
            vec![
                (
                    statement.old_envelope.d1.clone(),
                    neg_scalar_of(pairing, responses.y.clone()),
                ),
                (g2, responses.s.clone()),
                (statement.buyer_keys.pk2.clone(), responses.sigma.clone()),
                (delta_d2, n.clone()),
            ],
        ),
    }
}

fn mint_challenge<P: IPairingArithmetic, F: IChainForms>(
    pairing: &P,
    statement: &AlgebraicMintStatement<F, P::G1, P::G2>,
    first_messages: &MintFirstMessages<P::G1, P::G2>,
) -> P::Scalar {
    let mut inputs = vec![MOCK_DELIVERY_PROOF_CHALLENGE_TAG.to_vec()];
    inputs.extend(context_mint_bytes(&statement.context));
    inputs.extend(vec![
        encoded_g2(pairing, statement.hpub.clone()),
        encoded_g1(pairing, statement.identity_element.clone()),
        encoded_g1(pairing, statement.buyer_keys.pk1.clone()),
        encoded_g2(pairing, statement.buyer_keys.pk2.clone()),
        encoded_g1(pairing, statement.envelope.c1.clone()),
        encoded_g1(pairing, statement.envelope.c2.clone()),
        encoded_g2(pairing, statement.envelope.d1.clone()),
        encoded_g2(pairing, statement.envelope.d2.clone()),
        encoded_g2(pairing, first_messages.hpub.clone()),
        encoded_g1(pairing, first_messages.c1.clone()),
        encoded_g1(pairing, first_messages.c2.clone()),
        encoded_g2(pairing, first_messages.d1.clone()),
        encoded_g2(pairing, first_messages.d2.clone()),
    ]);
    mixed_scalar::<P>(&inputs)
}

fn transfer_challenge<P: IPairingArithmetic, F: IChainForms>(
    pairing: &P,
    statement: &AlgebraicTransferStatement<F, P::G1, P::G2>,
    first_messages: &TransferFirstMessages<P::G1, P::G2>,
) -> P::Scalar {
    let mut inputs = vec![MOCK_DELIVERY_PROOF_CHALLENGE_TAG.to_vec()];
    inputs.extend(context_transfer_bytes(&statement.context));
    inputs.extend(vec![
        encoded_g1(pairing, statement.identity_element.clone()),
        encoded_g1(pairing, statement.seller_keys.pk1.clone()),
        encoded_g2(pairing, statement.seller_keys.pk2.clone()),
        encoded_g1(pairing, statement.buyer_keys.pk1.clone()),
        encoded_g2(pairing, statement.buyer_keys.pk2.clone()),
        encoded_g1(pairing, statement.old_envelope.c1.clone()),
        encoded_g1(pairing, statement.old_envelope.c2.clone()),
        encoded_g2(pairing, statement.old_envelope.d1.clone()),
        encoded_g2(pairing, statement.old_envelope.d2.clone()),
        encoded_g1(pairing, statement.new_envelope.c1.clone()),
        encoded_g1(pairing, statement.new_envelope.c2.clone()),
        encoded_g2(pairing, statement.new_envelope.d1.clone()),
        encoded_g2(pairing, statement.new_envelope.d2.clone()),
        encoded_g1(pairing, first_messages.pk1.clone()),
        encoded_g1(pairing, first_messages.c1.clone()),
        encoded_g1(pairing, first_messages.c2.clone()),
        encoded_g2(pairing, first_messages.pk2.clone()),
        encoded_g2(pairing, first_messages.d1.clone()),
        encoded_g2(pairing, first_messages.d2.clone()),
    ]);
    mixed_scalar::<P>(&inputs)
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

impl<'a, P: IPairingArithmetic, F: IChainForms> IDeliveryProofAdapter
    for MockIDeliveryProofAdapter<'a, P, F>
{
    const DECLARATION: DeliveryProofDeclaration = MOCK_I_DELIVERY_PROOF_ADAPTER_DECLARATION;

    type Pairing = P;
    type Forms = F;
    type Proof = MockDeliveryProof<P>;

    fn prove_mint(
        &self,
        _params: ProveMintParams,
        payload: ProveMintPayload<
            '_,
            Self::Forms,
            <Self::Pairing as IPairingAdapter>::Scalar,
            <Self::Pairing as IPairingAdapter>::G1,
            <Self::Pairing as IPairingAdapter>::G2,
        >,
    ) -> ProveMintReturn<Self::Proof> {
        match self.failure_mode {
            MockIDeliveryProofAdapterFailureMode::NonceDrawRefused => {
                return Err(ProveMintErrorReturn::Mock(
                    MockIDeliveryProofAdapterProveMintErrorReturn::NonceDrawRefused,
                ));
            }
            MockIDeliveryProofAdapterFailureMode::NoFailure => {}
        }
        let pairing = self.pairing;
        let statement = payload.statement;
        let nonces = [
            sampled_scalar::<P>(FILL_NONCE_ALPHA),
            sampled_scalar::<P>(FILL_NONCE_R),
            sampled_scalar::<P>(FILL_NONCE_RHO),
            sampled_scalar::<P>(FILL_NONCE_SIGMA),
        ];
        let first_messages = mint_first_messages_from_nonces(pairing, &nonces, statement);
        let challenge = mint_challenge(pairing, statement, &first_messages);
        let responses = MintResponses {
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
        };
        let components = match self.verifier_form {
            VerifierGroupArithmetic::BothGroups => DeliveryProofComponents::MintBothGroups {
                challenge,
                responses,
            },
            VerifierGroupArithmetic::FirstGroupOnly => {
                DeliveryProofComponents::MintFirstGroupOnly {
                    challenge,
                    responses,
                    first_messages: MintSecondGroupFirstMessages {
                        hpub: first_messages.hpub,
                        d1: first_messages.d1,
                        d2: first_messages.d2,
                    },
                }
            }
        };
        Ok(ProveMintSuccessReturn {
            proof: MockDeliveryProof { components },
        })
    }

    fn prove_transfer(
        &self,
        _params: ProveTransferParams,
        payload: ProveTransferPayload<
            '_,
            Self::Forms,
            <Self::Pairing as IPairingAdapter>::Scalar,
            <Self::Pairing as IPairingAdapter>::G1,
            <Self::Pairing as IPairingAdapter>::G2,
        >,
    ) -> ProveTransferReturn<Self::Proof> {
        match self.failure_mode {
            MockIDeliveryProofAdapterFailureMode::NonceDrawRefused => {
                return Err(ProveTransferErrorReturn::Mock(
                    MockIDeliveryProofAdapterProveTransferErrorReturn::NonceDrawRefused,
                ));
            }
            MockIDeliveryProofAdapterFailureMode::NoFailure => {}
        }
        let pairing = self.pairing;
        let statement = payload.statement;
        let nonces = [
            sampled_scalar::<P>(FILL_NONCE_X),
            sampled_scalar::<P>(FILL_NONCE_Y),
            sampled_scalar::<P>(FILL_NONCE_S),
            sampled_scalar::<P>(FILL_NONCE_RHO_TRANSFER),
            sampled_scalar::<P>(FILL_NONCE_SIGMA_TRANSFER),
        ];
        let first_messages = transfer_first_messages_from_nonces(pairing, &nonces, statement);
        let challenge = transfer_challenge(pairing, statement, &first_messages);
        let responses = TransferResponses {
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
        };
        let components = match self.verifier_form {
            VerifierGroupArithmetic::BothGroups => DeliveryProofComponents::TransferBothGroups {
                challenge,
                responses,
            },
            VerifierGroupArithmetic::FirstGroupOnly => {
                DeliveryProofComponents::TransferFirstGroupOnly {
                    challenge,
                    responses,
                    first_messages: TransferSecondGroupFirstMessages {
                        pk2: first_messages.pk2,
                        d1: first_messages.d1,
                        d2: first_messages.d2,
                    },
                }
            }
        };
        Ok(ProveTransferSuccessReturn {
            proof: MockDeliveryProof { components },
        })
    }

    fn verify(
        &self,
        _params: VerifyParams,
        payload: VerifyPayload<
            '_,
            Self::Forms,
            <Self::Pairing as IPairingAdapter>::G1,
            <Self::Pairing as IPairingAdapter>::G2,
            Self::Proof,
        >,
    ) -> VerifyReturn {
        let pairing = self.pairing;
        match payload.statement {
            AlgebraicStatement::Mint(statement) => match &payload.proof.components {
                DeliveryProofComponents::MintBothGroups {
                    challenge,
                    responses,
                } => {
                    if !matches!(self.verifier_form, VerifierGroupArithmetic::BothGroups) {
                        return Ok(VerifySuccessReturn { is_valid: false });
                    }
                    let n = neg_scalar_of(pairing, challenge.clone());
                    let first_messages =
                        mint_first_messages_recomputed(pairing, &n, responses, statement);
                    let recomputed = mint_challenge(pairing, statement, &first_messages);
                    Ok(VerifySuccessReturn {
                        is_valid: scalars_equal(pairing, &recomputed, challenge),
                    })
                }
                DeliveryProofComponents::MintFirstGroupOnly {
                    challenge,
                    responses,
                    first_messages,
                } => {
                    if !matches!(self.verifier_form, VerifierGroupArithmetic::FirstGroupOnly) {
                        return Ok(VerifySuccessReturn { is_valid: false });
                    }
                    let n = neg_scalar_of(pairing, challenge.clone());
                    let recomputed_messages =
                        mint_first_messages_recomputed(pairing, &n, responses, statement);
                    let recomputed = mint_challenge(pairing, statement, &recomputed_messages);
                    let carries_match =
                        g2_equal(pairing, &first_messages.hpub, &recomputed_messages.hpub)
                            && g2_equal(pairing, &first_messages.d1, &recomputed_messages.d1)
                            && g2_equal(pairing, &first_messages.d2, &recomputed_messages.d2);
                    Ok(VerifySuccessReturn {
                        is_valid: scalars_equal(pairing, &recomputed, challenge) && carries_match,
                    })
                }
                DeliveryProofComponents::TransferBothGroups { .. }
                | DeliveryProofComponents::TransferFirstGroupOnly { .. } => {
                    Ok(VerifySuccessReturn { is_valid: false })
                }
            },
            AlgebraicStatement::Transfer(statement) => match &payload.proof.components {
                DeliveryProofComponents::TransferBothGroups {
                    challenge,
                    responses,
                } => {
                    if !matches!(self.verifier_form, VerifierGroupArithmetic::BothGroups) {
                        return Ok(VerifySuccessReturn { is_valid: false });
                    }
                    let n = neg_scalar_of(pairing, challenge.clone());
                    let first_messages =
                        transfer_first_messages_recomputed(pairing, &n, responses, statement);
                    let recomputed = transfer_challenge(pairing, statement, &first_messages);
                    Ok(VerifySuccessReturn {
                        is_valid: scalars_equal(pairing, &recomputed, challenge),
                    })
                }
                DeliveryProofComponents::TransferFirstGroupOnly {
                    challenge,
                    responses,
                    first_messages,
                } => {
                    if !matches!(self.verifier_form, VerifierGroupArithmetic::FirstGroupOnly) {
                        return Ok(VerifySuccessReturn { is_valid: false });
                    }
                    let n = neg_scalar_of(pairing, challenge.clone());
                    let recomputed_messages =
                        transfer_first_messages_recomputed(pairing, &n, responses, statement);
                    let recomputed = transfer_challenge(pairing, statement, &recomputed_messages);
                    let carries_match =
                        g2_equal(pairing, &first_messages.pk2, &recomputed_messages.pk2)
                            && g2_equal(pairing, &first_messages.d1, &recomputed_messages.d1)
                            && g2_equal(pairing, &first_messages.d2, &recomputed_messages.d2);
                    Ok(VerifySuccessReturn {
                        is_valid: scalars_equal(pairing, &recomputed, challenge) && carries_match,
                    })
                }
                DeliveryProofComponents::MintBothGroups { .. }
                | DeliveryProofComponents::MintFirstGroupOnly { .. } => {
                    Ok(VerifySuccessReturn { is_valid: false })
                }
            },
        }
    }

    fn proof_components(
        &self,
        _params: ProofComponentsParams,
        payload: ProofComponentsPayload<'_, Self::Proof>,
    ) -> ProofComponentsReturn<
        <Self::Pairing as IPairingAdapter>::Scalar,
        <Self::Pairing as IPairingAdapter>::G2,
    > {
        let components = match &payload.proof.components {
            DeliveryProofComponents::MintBothGroups {
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
            DeliveryProofComponents::MintFirstGroupOnly {
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
            DeliveryProofComponents::TransferBothGroups {
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
            DeliveryProofComponents::TransferFirstGroupOnly {
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
        payload: ProofFromComponentsPayload<
            <Self::Pairing as IPairingAdapter>::Scalar,
            <Self::Pairing as IPairingAdapter>::G2,
        >,
    ) -> ProofFromComponentsReturn<Self::Proof> {
        let form_matches = matches!(
            (&payload.components, self.verifier_form),
            (
                DeliveryProofComponents::MintBothGroups { .. }
                    | DeliveryProofComponents::TransferBothGroups { .. },
                VerifierGroupArithmetic::BothGroups
            ) | (
                DeliveryProofComponents::MintFirstGroupOnly { .. }
                    | DeliveryProofComponents::TransferFirstGroupOnly { .. },
                VerifierGroupArithmetic::FirstGroupOnly
            )
        );
        if !form_matches {
            return Err(ProofFromComponentsErrorReturn::VerifierFormMismatch);
        }
        Ok(ProofFromComponentsSuccessReturn {
            proof: MockDeliveryProof {
                components: payload.components,
            },
        })
    }
}

pub struct MockIDeliveryProofConsumer;

impl<P: IPairingAdapter, F: IChainForms> IDeliveryProofConsumer<P, F>
    for MockIDeliveryProofConsumer
{
    type Output = ();

    fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
        &self,
        _params: ConsumeDeliveryProofParams,
        _payload: ConsumeDeliveryProofPayload<D>,
    ) {
    }
}

#[derive(Default)]
pub struct ConsumeDeliveryProofParamsOverrides;

pub fn build_consume_delivery_proof_params(
    _overrides: ConsumeDeliveryProofParamsOverrides,
) -> ConsumeDeliveryProofParams {
    ConsumeDeliveryProofParams
}

#[derive(Default)]
pub struct CreateDeliveryProofPayloadOverrides;

pub fn build_create_delivery_proof_payload(
    _overrides: CreateDeliveryProofPayloadOverrides,
) -> CreateDeliveryProofPayload {
    CreateDeliveryProofPayload
}

#[derive(Default)]
pub struct ConsumeDeliveryProofPayloadOverrides;

pub fn build_consume_delivery_proof_payload<D: IDeliveryProofAdapter>(
    adapter: D,
    _overrides: ConsumeDeliveryProofPayloadOverrides,
) -> ConsumeDeliveryProofPayload<D> {
    ConsumeDeliveryProofPayload { adapter }
}

#[derive(Default)]
pub struct CreateDeliveryProofDepsOverrides;

pub fn build_create_delivery_proof_deps<'a, P: IPairingArithmetic, E: IEncoderAdapter, C>(
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    random: &'a dyn IRandomSourceAdapter,
    consumer: C,
    _overrides: CreateDeliveryProofDepsOverrides,
) -> CreateDeliveryProofDeps<'a, P, E, C> {
    CreateDeliveryProofDeps {
        pairing,
        hash_to_scalar,
        encoder,
        random,
        consumer,
    }
}

#[derive(Default)]
pub struct CreateDeliveryProofSuccessReturnOverrides;

pub fn build_create_delivery_proof_success_return<O>(
    output: O,
    _overrides: CreateDeliveryProofSuccessReturnOverrides,
) -> CreateDeliveryProofSuccessReturn<O> {
    CreateDeliveryProofSuccessReturn { output }
}

#[derive(Default)]
pub struct CreateDeliveryProofParamsOverrides {
    pub concrete: Option<DeliveryProofConcrete>,
    pub algebra: Option<EnvelopeAlgebra>,
    pub statement_version: Option<u16>,
}

pub fn build_create_delivery_proof_params(
    overrides: CreateDeliveryProofParamsOverrides,
) -> CreateDeliveryProofParams {
    CreateDeliveryProofParams {
        concrete: overrides.concrete.unwrap_or(DeliveryProofConcrete::Mock(
            MockIDeliveryProofAdapterFailureMode::NoFailure,
        )),
        algebra: overrides.algebra.unwrap_or(EnvelopeAlgebra::PairingElGamal),
        statement_version: overrides
            .statement_version
            .unwrap_or(DELIVERY_STATEMENT_VERSION_ONE),
    }
}

#[derive(Default)]
pub struct CreateDeliveryProofParamsCorruptions {
    pub concrete: Option<Value>,
    pub algebra: Option<Value>,
    pub statement_version: Option<Value>,
}

pub fn invalidate_create_delivery_proof_params(
    corruptions: CreateDeliveryProofParamsCorruptions,
) -> Value {
    let params = build_create_delivery_proof_params(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "concrete".to_string(),
        Value::from(match params.concrete {
            DeliveryProofConcrete::SchnorrFs => "SchnorrFs",
            DeliveryProofConcrete::Mock(_) => "Mock",
        }),
    );
    fields.insert(
        "algebra".to_string(),
        Value::from(match params.algebra {
            EnvelopeAlgebra::PairingElGamal => "PairingElGamal",
        }),
    );
    fields.insert(
        "statement_version".to_string(),
        Value::from(params.statement_version),
    );
    if let Some(concrete) = corruptions.concrete {
        fields.insert("concrete".to_string(), concrete);
    }
    if let Some(algebra) = corruptions.algebra {
        fields.insert("algebra".to_string(), algebra);
    }
    if let Some(statement_version) = corruptions.statement_version {
        fields.insert("statement_version".to_string(), statement_version);
    }
    Value::Object(fields)
}

pub fn mock_create_delivery_proof<
    'a,
    P: IPairingArithmetic,
    E: IEncoderAdapter,
    F: IChainForms,
    C: IDeliveryProofConsumer<P, F>,
>(
    deps: &CreateDeliveryProofDeps<'a, P, E, C>,
    _params: CreateDeliveryProofParams,
    _payload: CreateDeliveryProofPayload,
) -> CreateDeliveryProofReturn<C::Output> {
    let _: CreateDeliveryProofFn<'a, P, E, F, C> = mock_create_delivery_proof::<P, E, F, C>;
    let adapter: MockIDeliveryProofAdapter<'a, P, F> =
        build_mock_i_delivery_proof_adapter(deps.pairing, Default::default());
    let output = deps.consumer.consume_delivery_proof(
        build_consume_delivery_proof_params(Default::default()),
        build_consume_delivery_proof_payload(adapter, Default::default()),
    );
    Ok(build_create_delivery_proof_success_return(
        output,
        Default::default(),
    ))
}

#[derive(Default)]
pub struct CreateDeliveryProofCasePayloadOverrides;

pub fn build_create_delivery_proof_case_payload(
    _overrides: CreateDeliveryProofCasePayloadOverrides,
) -> CreateDeliveryProofCasePayload {
    CreateDeliveryProofCasePayload
}

#[derive(Default)]
pub struct CreateDeliveryProofCaseDepsOverrides;

pub fn build_create_delivery_proof_case_deps<C: ICreateDeliveryProofCaseConsumer>(
    consumer: C,
    _overrides: CreateDeliveryProofCaseDepsOverrides,
) -> CreateDeliveryProofCaseDeps<C> {
    CreateDeliveryProofCaseDeps { consumer }
}

#[derive(Default)]
pub struct CreateDeliveryProofCaseSuccessReturnOverrides;

pub fn build_create_delivery_proof_case_success_return<O>(
    output: O,
    _overrides: CreateDeliveryProofCaseSuccessReturnOverrides,
) -> CreateDeliveryProofCaseSuccessReturn<O> {
    CreateDeliveryProofCaseSuccessReturn { output }
}

#[derive(Default)]
pub struct CreateDeliveryProofCaseParamsOverrides {
    pub pairing: Option<CreatePairingParams>,
    pub hash_to_scalar: Option<CreateHashToScalarParams>,
    pub encoding: Option<CreateEncodingParams>,
    pub chain_forms: Option<CreateChainFormsParams>,
    pub kem: Option<CreateKemParams>,
    pub key_agreement: Option<CreateKeyAgreementParams>,
    pub random_source: Option<CreateRandomSourceParams>,
}

pub fn build_create_delivery_proof_case_params(
    overrides: CreateDeliveryProofCaseParamsOverrides,
) -> CreateDeliveryProofCaseParams {
    CreateDeliveryProofCaseParams {
        pairing: overrides
            .pairing
            .unwrap_or_else(|| build_create_pairing_params(Default::default())),
        hash_to_scalar: overrides
            .hash_to_scalar
            .unwrap_or_else(|| build_create_hash_to_scalar_params(Default::default())),
        encoding: overrides
            .encoding
            .unwrap_or_else(|| build_create_encoding_params(Default::default())),
        chain_forms: overrides
            .chain_forms
            .unwrap_or_else(|| build_create_chain_forms_params(Default::default())),
        kem: overrides
            .kem
            .unwrap_or_else(|| build_create_kem_params(Default::default())),
        key_agreement: overrides
            .key_agreement
            .unwrap_or_else(|| build_create_key_agreement_params(Default::default())),
        random_source: overrides
            .random_source
            .unwrap_or_else(|| build_create_random_source_params(Default::default())),
    }
}

#[derive(Default)]
pub struct CreateDeliveryProofCaseConsumeParamsOverrides;

pub fn build_create_delivery_proof_case_consume_params(
    _overrides: CreateDeliveryProofCaseConsumeParamsOverrides,
) -> CreateDeliveryProofCaseConsumeParams {
    CreateDeliveryProofCaseConsumeParams
}

#[derive(Default)]
pub struct CreateDeliveryProofCaseConsumePayloadOverrides;

pub fn build_create_delivery_proof_case_consume_payload<
    'a,
    P: IPairingArithmetic,
    E: IEncoderAdapter,
    F: IChainForms,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
>(
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    random: &'a dyn IRandomSourceAdapter,
    forms: &'a F,
    kem: &'a K,
    key_agreement: &'a A,
    _overrides: CreateDeliveryProofCaseConsumePayloadOverrides,
) -> CreateDeliveryProofCaseConsumePayload<'a, P, E, F, K, A> {
    CreateDeliveryProofCaseConsumePayload {
        pairing,
        hash_to_scalar,
        encoder,
        random,
        forms,
        kem,
        key_agreement,
    }
}

struct CaseAtPairing<'a, C: ICreateDeliveryProofCaseConsumer> {
    pub consumer: &'a C,
    pub encoding: CreateEncodingParams,
    pub chain_forms: CreateChainFormsParams,
    pub kem: CreateKemParams,
    pub key_agreement: CreateKeyAgreementParams,
    pub hash_to_scalar: CreateHashToScalarParams,
    pub random_source: CreateRandomSourceParams,
}

impl<C: ICreateDeliveryProofCaseConsumer> IPairingConsumer for CaseAtPairing<'_, C> {
    type Output = CreateDeliveryProofCaseReturn<C::Output>;

    fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let hash_to_scalar = match create_hash_to_scalar::<P::Scalar>(
            &build_create_hash_to_scalar_deps(Default::default()),
            self.hash_to_scalar,
            build_create_hash_to_scalar_payload(Default::default()),
        ) {
            Ok(success) => success,
            Err(error) => {
                return Err(CreateDeliveryProofCaseErrorReturn::HashToScalar(error));
            }
        };
        let random = match create_random_source(
            &CreateRandomSourceDeps,
            build_create_random_source_params(CreateRandomSourceParamsOverrides {
                kind: Some(self.random_source.kind),
            }),
            CreateRandomSourcePayload,
        ) {
            Ok(success) => success,
            Err(error) => {
                return Err(CreateDeliveryProofCaseErrorReturn::RandomSource(error));
            }
        };
        let encoding_deps = build_create_encoding_deps(CaseAtEncoding {
            consumer: self.consumer,
            pairing: &payload.adapter,
            hash_to_scalar: &*hash_to_scalar.adapter,
            random: &*random.adapter,
            chain_forms: self.chain_forms,
            kem: self.kem,
            key_agreement: self.key_agreement,
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
            Err(error) => Err(CreateDeliveryProofCaseErrorReturn::Encoding(error)),
        }
    }
}

struct CaseAtEncoding<
    'a,
    'p,
    'h,
    'r,
    C: ICreateDeliveryProofCaseConsumer,
    P: IPairingArithmetic + IPairingReference,
> {
    pub consumer: &'a C,
    pub pairing: &'p P,
    pub hash_to_scalar: &'h dyn IHashToScalarAdapter<P::Scalar>,
    pub random: &'r dyn IRandomSourceAdapter,
    pub chain_forms: CreateChainFormsParams,
    pub kem: CreateKemParams,
    pub key_agreement: CreateKeyAgreementParams,
}

impl<C: ICreateDeliveryProofCaseConsumer, P: IPairingArithmetic + IPairingReference>
    IEncodingConsumer for CaseAtEncoding<'_, '_, '_, '_, C, P>
{
    type Output = CreateDeliveryProofCaseReturn<C::Output>;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
        let chain_forms_deps = build_create_chain_forms_deps(
            CaseAtChainForms {
                consumer: self.consumer,
                pairing: self.pairing,
                hash_to_scalar: self.hash_to_scalar,
                encoder: &payload.adapter,
                random: self.random,
                kem: self.kem,
                key_agreement: self.key_agreement,
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
            Err(error) => Err(CreateDeliveryProofCaseErrorReturn::ChainForms(error)),
        }
    }
}

struct CaseAtChainForms<
    'a,
    'p,
    'h,
    'e,
    'r,
    C: ICreateDeliveryProofCaseConsumer,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
> {
    pub consumer: &'a C,
    pub pairing: &'p P,
    pub hash_to_scalar: &'h dyn IHashToScalarAdapter<P::Scalar>,
    pub encoder: &'e E,
    pub random: &'r dyn IRandomSourceAdapter,
    pub kem: CreateKemParams,
    pub key_agreement: CreateKeyAgreementParams,
}

impl<
    C: ICreateDeliveryProofCaseConsumer,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
> IChainFormsConsumer for CaseAtChainForms<'_, '_, '_, '_, '_, C, P, E>
{
    type Output = CreateDeliveryProofCaseReturn<C::Output>;

    fn consume_chain_forms<F: IChainForms>(
        &self,
        _params: ConsumeChainFormsParams,
        payload: ConsumeChainFormsPayload<F>,
    ) -> Self::Output {
        let kem_deps = build_create_kem_deps(
            self.pairing,
            self.hash_to_scalar,
            CaseAtKem {
                consumer: self.consumer,
                pairing: self.pairing,
                hash_to_scalar: self.hash_to_scalar,
                encoder: self.encoder,
                random: self.random,
                forms: &payload.forms,
                key_agreement: self.key_agreement,
            },
            Default::default(),
        );
        match create_kem(
            &kem_deps,
            self.kem,
            build_create_kem_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(CreateDeliveryProofCaseErrorReturn::Kem(error)),
        }
    }
}

struct CaseAtKem<
    'a,
    'p,
    'h,
    'e,
    'r,
    'f,
    C: ICreateDeliveryProofCaseConsumer,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
    F: IChainForms,
> {
    pub consumer: &'a C,
    pub pairing: &'p P,
    pub hash_to_scalar: &'h dyn IHashToScalarAdapter<P::Scalar>,
    pub encoder: &'e E,
    pub random: &'r dyn IRandomSourceAdapter,
    pub forms: &'f F,
    pub key_agreement: CreateKeyAgreementParams,
}

impl<
    C: ICreateDeliveryProofCaseConsumer,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
    F: IChainForms,
> IKemConsumer<P> for CaseAtKem<'_, '_, '_, '_, '_, '_, C, P, E, F>
{
    type Output = CreateDeliveryProofCaseReturn<C::Output>;

    fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
        &self,
        _params: ConsumeKemParams,
        payload: ConsumeKemPayload<K>,
    ) -> Self::Output {
        let key_agreement_deps = build_create_key_agreement_deps(
            self.pairing,
            self.hash_to_scalar,
            self.encoder,
            self.random,
            CaseAtKeyAgreement {
                consumer: self.consumer,
                pairing: self.pairing,
                hash_to_scalar: self.hash_to_scalar,
                encoder: self.encoder,
                random: self.random,
                forms: self.forms,
                kem: &payload.adapter,
            },
            Default::default(),
        );
        match create_key_agreement(
            &key_agreement_deps,
            self.key_agreement,
            build_create_key_agreement_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(CreateDeliveryProofCaseErrorReturn::KeyAgreement(error)),
        }
    }
}

struct CaseAtKeyAgreement<
    'a,
    'p,
    'h,
    'e,
    'r,
    'f,
    'k,
    C: ICreateDeliveryProofCaseConsumer,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
    F: IChainForms,
    K: ICredentialKemAdapter<Pairing = P>,
> {
    pub consumer: &'a C,
    pub pairing: &'p P,
    pub hash_to_scalar: &'h dyn IHashToScalarAdapter<P::Scalar>,
    pub encoder: &'e E,
    pub random: &'r dyn IRandomSourceAdapter,
    pub forms: &'f F,
    pub kem: &'k K,
}

impl<
    C: ICreateDeliveryProofCaseConsumer,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
    F: IChainForms,
    K: ICredentialKemAdapter<Pairing = P>,
> IKeyAgreementConsumer<P> for CaseAtKeyAgreement<'_, '_, '_, '_, '_, '_, '_, C, P, E, F, K>
{
    type Output = CreateDeliveryProofCaseReturn<C::Output>;

    fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
        &self,
        _params: ConsumeKeyAgreementParams,
        payload: ConsumeKeyAgreementPayload<A>,
    ) -> Self::Output {
        let output = self.consumer.consume_create_delivery_proof_case(
            build_create_delivery_proof_case_consume_params(Default::default()),
            build_create_delivery_proof_case_consume_payload(
                self.pairing,
                self.hash_to_scalar,
                self.encoder,
                self.random,
                self.forms,
                self.kem,
                &payload.adapter,
                Default::default(),
            ),
        );
        Ok(CreateDeliveryProofCaseSuccessReturn { output })
    }
}

pub fn run_create_delivery_proof_case<C: ICreateDeliveryProofCaseConsumer>(
    deps: &CreateDeliveryProofCaseDeps<C>,
    params: CreateDeliveryProofCaseParams,
    _payload: CreateDeliveryProofCasePayload,
) -> CreateDeliveryProofCaseReturn<C::Output> {
    let _: RunCreateDeliveryProofCaseFn<C> = run_create_delivery_proof_case::<C>;
    let pairing_deps = build_create_pairing_deps(
        CaseAtPairing {
            consumer: &deps.consumer,
            encoding: params.encoding,
            chain_forms: params.chain_forms,
            kem: params.kem,
            key_agreement: params.key_agreement,
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
        Err(error) => Err(CreateDeliveryProofCaseErrorReturn::Pairing(error)),
    }
}
