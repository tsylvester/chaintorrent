#![allow(clippy::expect_used)]

use chain::IChainForms;
use core::convert::Infallible;
use core::marker::PhantomData;
use domain::{
    AssetIdentityHash, ParameterSetIdentifier, Secret, SecretConstructorParamsOverrides,
    SuiteIdentifier, build_asset_identity_hash, build_parameter_set_identifier, build_secret,
    build_suite_identifier,
};
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, ICanonicalField, IEncoderAdapter, ToFieldParams,
    build_from_field_params,
};
use envelope::{
    EnvelopeAlgebra, EnvelopeCoins, EnvelopeComponents, KeyPairComponents, PublicKeysComponents,
    build_envelope_coins, build_key_pair_components, build_public_keys_components,
};
use hash_to_scalar::IHashToScalarAdapter;
use kem::{MasterScalarComponents, build_master_scalar_components};
use pairing::{
    AddG1Params, AddG1Payload, AddG2Params, AddG2Payload, AddScalarParams, AddScalarPayload,
    EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload, EncodeScalarParams,
    EncodeScalarPayload, G1GeneratorParams, G1GeneratorPayload, G2GeneratorParams,
    G2GeneratorPayload, IPairingAdapter, IPairingArithmetic, ISampleUniformScalar, MsmG1Params,
    MsmG1Payload, MsmG1Term, MsmG2Params, MsmG2Payload, MsmG2Term, MulG1Params, MulG1Payload,
    MulG2Params, MulG2Payload, MulScalarParams, MulScalarPayload, NegG1Params, NegG1Payload,
    NegG2Params, NegG2Payload, NegScalarParams, NegScalarPayload, SampleUniformScalarParams,
    SampleUniformScalarPayload, VerifierGroupArithmetic,
};
use random::IRandomSourceAdapter;
use serde_json::{Map, Value};
use zeroize::Zeroize;

use crate::mint_statement::provides::{
    DELIVERY_STATEMENT_VERSION_ONE, DeliveryRelation, MintPurpose, TransferPurpose,
};

use super::interface::{
    AlgebraicMintStatement, AlgebraicStatement, AlgebraicTransferStatement,
    ConsumeDeliveryProofParams, ConsumeDeliveryProofPayload, CreateDeliveryProofDeps,
    CreateDeliveryProofFn, CreateDeliveryProofParams, CreateDeliveryProofPayload,
    CreateDeliveryProofReturn, CreateDeliveryProofSuccessReturn, DELIVERY_PROOF_INTERFACE_VERSION,
    DeliveryContext, DeliveryProofComponents, DeliveryProofConcrete, DeliveryProofDeclaration,
    DeliveryProofWireComponents, IDeliveryProofAdapter, IDeliveryProofConsumer, MintResponses,
    MintSecondGroupFirstMessages, MockIDeliveryProofAdapterFailureMode,
    MockIDeliveryProofAdapterProveMintErrorReturn,
    MockIDeliveryProofAdapterProveTransferErrorReturn, ProofComponentsParams,
    ProofComponentsPayload, ProofComponentsReturn, ProofComponentsSuccessReturn,
    ProofFromComponentsErrorReturn, ProofFromComponentsParams, ProofFromComponentsPayload,
    ProofFromComponentsReturn, ProofFromComponentsSuccessReturn, ProveMintErrorReturn,
    ProveMintParams, ProveMintPayload, ProveMintReturn, ProveMintSuccessReturn,
    ProveTransferErrorReturn, ProveTransferParams, ProveTransferPayload, ProveTransferReturn,
    ProveTransferSuccessReturn, TransferResponses, TransferSecondGroupFirstMessages, VerifyParams,
    VerifyPayload, VerifyReturn, VerifySuccessReturn,
};

pub const MOCK_DELIVERY_PROOF_CHALLENGE_TAG: &[u8] = b"ChainTorrent-v1-mock-proof-challenge";
pub const MOCK_DELIVERY_PROOF_WEIGHT_TAG: &[u8] = b"ChainTorrent-v1-mock-proof-weight";

fn uniform_bytes<S: ISampleUniformScalar>(value: u8) -> Secret<Vec<u8>> {
    let mut uniform = vec![0u8; S::UNIFORM_BYTES_LENGTH];
    uniform[S::UNIFORM_BYTES_LENGTH - 1] = value;
    build_secret(uniform, SecretConstructorParamsOverrides)
}

fn default_scalar_secret<S: ISampleUniformScalar>(value: u8) -> Secret<S> {
    S::sample_from_uniform_bytes(
        SampleUniformScalarParams,
        SampleUniformScalarPayload {
            uniform: uniform_bytes::<S>(value),
        },
    )
    .expect("sampling a default scalar always succeeds")
    .scalar
}

fn default_scalar<P: IPairingAdapter>(value: u8) -> P::Scalar {
    default_scalar_secret::<P::Scalar>(value).expose().clone()
}

fn generator_g1<P: IPairingAdapter>(pairing: &P) -> P::G1 {
    let Ok(success) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
    success.point
}

fn generator_g2<P: IPairingAdapter>(pairing: &P) -> P::G2 {
    let Ok(success) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
    success.point
}

fn sum_g1<P: IPairingAdapter>(pairing: &P, left: P::G1, right: P::G1) -> P::G1 {
    let Ok(success) = pairing.add_g1(AddG1Params, AddG1Payload { left, right });
    success.sum
}

fn sum_g2<P: IPairingAdapter>(pairing: &P, left: P::G2, right: P::G2) -> P::G2 {
    let Ok(success) = pairing.add_g2(AddG2Params, AddG2Payload { left, right });
    success.sum
}

fn product_g1<P: IPairingAdapter>(pairing: &P, point: P::G1, scalar: P::Scalar) -> P::G1 {
    let Ok(success) = pairing.mul_g1(MulG1Params, MulG1Payload { point, scalar });
    success.product
}

fn product_g2<P: IPairingAdapter>(pairing: &P, point: P::G2, scalar: P::Scalar) -> P::G2 {
    let Ok(success) = pairing.mul_g2(MulG2Params, MulG2Payload { point, scalar });
    success.product
}

fn msm_g1<P: IPairingAdapter>(pairing: &P, terms: Vec<MsmG1Term<P::G1, P::Scalar>>) -> P::G1 {
    let Ok(success) = pairing.msm_g1(MsmG1Params, MsmG1Payload { terms });
    success.sum
}

fn msm_g2<P: IPairingAdapter>(pairing: &P, terms: Vec<MsmG2Term<P::G2, P::Scalar>>) -> P::G2 {
    let Ok(success) = pairing.msm_g2(MsmG2Params, MsmG2Payload { terms });
    success.sum
}

fn negated_g1<P: IPairingArithmetic>(pairing: &P, point: P::G1) -> P::G1 {
    let Ok(success) = pairing.neg_g1(NegG1Params, NegG1Payload { point });
    success.negation
}

fn negated_g2<P: IPairingArithmetic>(pairing: &P, point: P::G2) -> P::G2 {
    let Ok(success) = pairing.neg_g2(NegG2Params, NegG2Payload { point });
    success.negation
}

fn sum_scalar<P: IPairingArithmetic>(pairing: &P, left: P::Scalar, right: P::Scalar) -> P::Scalar {
    let Ok(success) = pairing.add_scalar(AddScalarParams, AddScalarPayload { left, right });
    success.sum
}

fn product_scalar<P: IPairingArithmetic>(
    pairing: &P,
    left: P::Scalar,
    right: P::Scalar,
) -> P::Scalar {
    let Ok(success) = pairing.mul_scalar(MulScalarParams, MulScalarPayload { left, right });
    success.product
}

fn negated_scalar<P: IPairingArithmetic>(pairing: &P, scalar: P::Scalar) -> P::Scalar {
    let Ok(success) = pairing.neg_scalar(NegScalarParams, NegScalarPayload { scalar });
    success.negation
}

fn encoded_g1_bytes<P: IPairingAdapter>(pairing: &P, point: &P::G1) -> Vec<u8> {
    let Ok(encoded) = pairing.encode_g1(
        EncodeG1Params,
        EncodeG1Payload {
            point: point.clone(),
        },
    );
    encoded.bytes.as_ref().to_vec()
}

fn encoded_g2_bytes<P: IPairingAdapter>(pairing: &P, point: &P::G2) -> Vec<u8> {
    let Ok(encoded) = pairing.encode_g2(
        EncodeG2Params,
        EncodeG2Payload {
            point: point.clone(),
        },
    );
    encoded.bytes.as_ref().to_vec()
}

fn encoded_scalar_bytes<P: IPairingAdapter>(pairing: &P, scalar: &P::Scalar) -> Vec<u8> {
    let Ok(encoded) = pairing.encode_scalar(
        EncodeScalarParams,
        EncodeScalarPayload {
            scalar: scalar.clone(),
        },
    );
    encoded.bytes.expose().as_ref().to_vec()
}

fn scalars_equal<P: IPairingAdapter>(pairing: &P, left: &P::Scalar, right: &P::Scalar) -> bool {
    encoded_scalar_bytes(pairing, left) == encoded_scalar_bytes(pairing, right)
}

fn g2_equal<P: IPairingAdapter>(pairing: &P, left: &P::G2, right: &P::G2) -> bool {
    encoded_g2_bytes(pairing, left) == encoded_g2_bytes(pairing, right)
}

fn byte_array(bytes: &[u8]) -> Value {
    Value::Array(bytes.iter().map(|byte| Value::from(*byte)).collect())
}

fn encoded_g1_value<P: IPairingAdapter>(pairing: &P, point: &P::G1) -> Value {
    byte_array(&encoded_g1_bytes(pairing, point))
}

fn encoded_g2_value<P: IPairingAdapter>(pairing: &P, point: &P::G2) -> Value {
    byte_array(&encoded_g2_bytes(pairing, point))
}

fn encoded_scalar_value<P: IPairingAdapter>(pairing: &P, scalar: &P::Scalar) -> Value {
    byte_array(&encoded_scalar_bytes(pairing, scalar))
}

fn canonical_field_seed(kind: CanonicalFieldKind, seed: u8) -> CanonicalFieldValue {
    match kind {
        CanonicalFieldKind::FixedBytes32 => CanonicalFieldValue::FixedBytes32([seed; 32]),
        CanonicalFieldKind::Unsigned16 => CanonicalFieldValue::Unsigned16(u16::from(seed)),
        CanonicalFieldKind::Unsigned32 => CanonicalFieldValue::Unsigned32(u32::from(seed)),
        CanonicalFieldKind::Unsigned64 => CanonicalFieldValue::Unsigned64(u64::from(seed)),
        CanonicalFieldKind::Text => CanonicalFieldValue::Text(format!("chain-torrent-{seed}")),
        CanonicalFieldKind::Bytes => CanonicalFieldValue::Bytes(vec![seed; 8]),
        CanonicalFieldKind::FixedBytes20 => CanonicalFieldValue::FixedBytes20([seed; 20]),
        CanonicalFieldKind::Unsigned256 => CanonicalFieldValue::Unsigned256([seed; 32]),
    }
}

fn canonical_form<T: ICanonicalField>(seed: u8) -> T {
    T::from_field(
        build_from_field_params(Default::default()),
        canonical_field_seed(T::KIND, seed),
    )
    .expect("a constant of the declared kind is admitted")
    .value
}

fn canonical_field_value<T: ICanonicalField>(value: &T) -> CanonicalFieldValue {
    let Ok(success) = T::to_field(ToFieldParams, value);
    success.field
}

fn canonical_field_bytes(value: &CanonicalFieldValue) -> Vec<u8> {
    match value {
        CanonicalFieldValue::FixedBytes32(bytes) => bytes.to_vec(),
        CanonicalFieldValue::Unsigned16(number) => number.to_be_bytes().to_vec(),
        CanonicalFieldValue::Unsigned32(number) => number.to_be_bytes().to_vec(),
        CanonicalFieldValue::Unsigned64(number) => number.to_be_bytes().to_vec(),
        CanonicalFieldValue::Text(text) => text.as_bytes().to_vec(),
        CanonicalFieldValue::Bytes(bytes) => bytes.clone(),
        CanonicalFieldValue::FixedBytes20(bytes) => bytes.to_vec(),
        CanonicalFieldValue::Unsigned256(bytes) => bytes.to_vec(),
    }
}

fn canonical_field_render(value: &CanonicalFieldValue) -> Value {
    match value {
        CanonicalFieldValue::FixedBytes32(bytes) => byte_array(bytes),
        CanonicalFieldValue::Unsigned16(number) => Value::from(*number),
        CanonicalFieldValue::Unsigned32(number) => Value::from(*number),
        CanonicalFieldValue::Unsigned64(number) => Value::from(*number),
        CanonicalFieldValue::Text(text) => Value::from(text.clone()),
        CanonicalFieldValue::Bytes(bytes) => byte_array(bytes),
        CanonicalFieldValue::FixedBytes20(bytes) => byte_array(bytes),
        CanonicalFieldValue::Unsigned256(bytes) => byte_array(bytes),
    }
}

fn form_bytes<T: ICanonicalField>(value: &T) -> Vec<u8> {
    canonical_field_bytes(&canonical_field_value(value))
}

fn context_bytes<F: IChainForms, Purpose>(
    context: &DeliveryContext<F, Purpose>,
    purpose_code: u16,
) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(context.suite_identifier.identifier());
    bytes.extend_from_slice(&context.suite_identifier.version().to_be_bytes());
    bytes.extend_from_slice(&form_bytes(&context.chain));
    bytes.extend_from_slice(&form_bytes(&context.entitlement_contract));
    bytes.extend_from_slice(context.asset_identity_hash.as_bytes());
    bytes.extend_from_slice(context.parameter_set_identifier.as_bytes());
    bytes.extend_from_slice(&form_bytes(&context.source_entitlement));
    bytes.extend_from_slice(&form_bytes(&context.target_entitlement));
    bytes.extend_from_slice(&form_bytes(&context.old_interval));
    bytes.extend_from_slice(&form_bytes(&context.new_interval));
    bytes.extend_from_slice(&purpose_code.to_be_bytes());
    bytes.extend_from_slice(&form_bytes(&context.seller));
    bytes.extend_from_slice(&form_bytes(&context.buyer));
    bytes.extend_from_slice(&context.expiry.to_be_bytes());
    bytes
}

fn mock_challenge<P: IPairingAdapter>(parts: Vec<Vec<u8>>) -> P::Scalar {
    let mut folded = vec![0u8; P::Scalar::UNIFORM_BYTES_LENGTH];
    let mut index = 0usize;
    for byte in MOCK_DELIVERY_PROOF_CHALLENGE_TAG
        .iter()
        .chain(parts.iter().flat_map(|part| part.iter()))
    {
        folded[index] = folded[index].wrapping_add(*byte).rotate_left(1);
        index = (index + 1) % folded.len();
    }
    P::Scalar::sample_from_uniform_bytes(
        SampleUniformScalarParams,
        SampleUniformScalarPayload {
            uniform: build_secret(folded, SecretConstructorParamsOverrides),
        },
    )
    .expect("sampling the folded challenge succeeds: its bytes carry the sampling bound's length")
    .scalar
    .expose()
    .clone()
}

fn response<P: IPairingArithmetic>(
    pairing: &P,
    nonce: P::Scalar,
    challenge: &P::Scalar,
    witness: &P::Scalar,
) -> P::Scalar {
    sum_scalar(
        pairing,
        nonce,
        product_scalar(pairing, challenge.clone(), witness.clone()),
    )
}

fn mint_challenge<P: IPairingAdapter, F: IChainForms>(
    pairing: &P,
    statement: &AlgebraicMintStatement<F, P::G1, P::G2>,
    t_hpub: &P::G2,
    t_c1: &P::G1,
    t_c2: &P::G1,
    t_d1: &P::G2,
    t_d2: &P::G2,
) -> P::Scalar {
    mock_challenge::<P>(vec![
        context_bytes(&statement.context, statement.context.purpose.code()),
        encoded_g2_bytes(pairing, &statement.hpub),
        encoded_g1_bytes(pairing, &statement.identity_element),
        encoded_g1_bytes(pairing, &statement.buyer_keys.pk1),
        encoded_g2_bytes(pairing, &statement.buyer_keys.pk2),
        encoded_g1_bytes(pairing, &statement.envelope.c1),
        encoded_g1_bytes(pairing, &statement.envelope.c2),
        encoded_g2_bytes(pairing, &statement.envelope.d1),
        encoded_g2_bytes(pairing, &statement.envelope.d2),
        encoded_g2_bytes(pairing, t_hpub),
        encoded_g1_bytes(pairing, t_c1),
        encoded_g1_bytes(pairing, t_c2),
        encoded_g2_bytes(pairing, t_d1),
        encoded_g2_bytes(pairing, t_d2),
    ])
}

fn transfer_challenge<P: IPairingAdapter, F: IChainForms>(
    pairing: &P,
    statement: &AlgebraicTransferStatement<F, P::G1, P::G2>,
    t_pk1: &P::G1,
    t_c1: &P::G1,
    t_c2: &P::G1,
    t_pk2: &P::G2,
    t_d1: &P::G2,
    t_d2: &P::G2,
) -> P::Scalar {
    mock_challenge::<P>(vec![
        context_bytes(&statement.context, statement.context.purpose.code()),
        encoded_g1_bytes(pairing, &statement.identity_element),
        encoded_g1_bytes(pairing, &statement.seller_keys.pk1),
        encoded_g2_bytes(pairing, &statement.seller_keys.pk2),
        encoded_g1_bytes(pairing, &statement.buyer_keys.pk1),
        encoded_g2_bytes(pairing, &statement.buyer_keys.pk2),
        encoded_g1_bytes(pairing, &statement.old_envelope.c1),
        encoded_g1_bytes(pairing, &statement.old_envelope.c2),
        encoded_g2_bytes(pairing, &statement.old_envelope.d1),
        encoded_g2_bytes(pairing, &statement.old_envelope.d2),
        encoded_g1_bytes(pairing, &statement.new_envelope.c1),
        encoded_g1_bytes(pairing, &statement.new_envelope.c2),
        encoded_g2_bytes(pairing, &statement.new_envelope.d1),
        encoded_g2_bytes(pairing, &statement.new_envelope.d2),
        encoded_g1_bytes(pairing, t_pk1),
        encoded_g1_bytes(pairing, t_c1),
        encoded_g1_bytes(pairing, t_c2),
        encoded_g2_bytes(pairing, t_pk2),
        encoded_g2_bytes(pairing, t_d1),
        encoded_g2_bytes(pairing, t_d2),
    ])
}

fn default_alpha<P: IPairingAdapter>() -> P::Scalar {
    build_master_scalar_components::<P::Scalar>(Default::default())
        .value
        .expose()
        .clone()
}

fn default_credential_randomness<P: IPairingAdapter>() -> Secret<P::Scalar> {
    build_secret(default_scalar::<P>(0x41), SecretConstructorParamsOverrides)
}

fn default_offset<P: IPairingAdapter>() -> Secret<P::Scalar> {
    build_secret(default_scalar::<P>(0x43), SecretConstructorParamsOverrides)
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
            .unwrap_or(&[EnvelopeAlgebra::PairingElGamal]),
        verifier_forms: overrides.verifier_forms.unwrap_or(&[
            VerifierGroupArithmetic::BothGroups,
            VerifierGroupArithmetic::FirstGroupOnly,
        ]),
        statement_versions: overrides
            .statement_versions
            .unwrap_or(&[DELIVERY_STATEMENT_VERSION_ONE]),
        challenge_tag: overrides
            .challenge_tag
            .unwrap_or(MOCK_DELIVERY_PROOF_CHALLENGE_TAG),
        weight_tag: overrides
            .weight_tag
            .unwrap_or(MOCK_DELIVERY_PROOF_WEIGHT_TAG),
        adapter_version: overrides.adapter_version.unwrap_or(1),
        interface_version: overrides
            .interface_version
            .unwrap_or(DELIVERY_PROOF_INTERFACE_VERSION),
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
    let declaration = build_delivery_proof_declaration(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "algebras".to_string(),
        Value::Array(
            declaration
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
            declaration
                .verifier_forms
                .iter()
                .map(|form| {
                    Value::from(match form {
                        VerifierGroupArithmetic::BothGroups => "BothGroups",
                        VerifierGroupArithmetic::FirstGroupOnly => "FirstGroupOnly",
                    })
                })
                .collect(),
        ),
    );
    fields.insert(
        "statement_versions".to_string(),
        Value::Array(
            declaration
                .statement_versions
                .iter()
                .map(|version| Value::from(*version))
                .collect(),
        ),
    );
    fields.insert(
        "challenge_tag".to_string(),
        byte_array(declaration.challenge_tag),
    );
    fields.insert("weight_tag".to_string(), byte_array(declaration.weight_tag));
    fields.insert(
        "adapter_version".to_string(),
        Value::from(declaration.adapter_version),
    );
    fields.insert(
        "interface_version".to_string(),
        Value::from(declaration.interface_version),
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
    pub suite_identifier: Option<SuiteIdentifier>,
    pub chain: Option<F::ChainIdentifier>,
    pub entitlement_contract: Option<F::Identity>,
    pub asset_identity_hash: Option<AssetIdentityHash>,
    pub parameter_set_identifier: Option<ParameterSetIdentifier>,
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
            asset_identity_hash: None,
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
        suite_identifier: overrides
            .suite_identifier
            .unwrap_or_else(|| build_suite_identifier(Default::default())),
        chain: overrides
            .chain
            .unwrap_or_else(|| canonical_form::<F::ChainIdentifier>(0x31)),
        entitlement_contract: overrides
            .entitlement_contract
            .unwrap_or_else(|| canonical_form::<F::Identity>(0x32)),
        asset_identity_hash: overrides
            .asset_identity_hash
            .unwrap_or_else(|| build_asset_identity_hash(Default::default())),
        parameter_set_identifier: overrides
            .parameter_set_identifier
            .unwrap_or_else(|| build_parameter_set_identifier(Default::default())),
        source_entitlement: overrides
            .source_entitlement
            .unwrap_or_else(|| canonical_form::<F::Entitlement>(0x33)),
        target_entitlement: overrides
            .target_entitlement
            .unwrap_or_else(|| canonical_form::<F::Entitlement>(0x34)),
        old_interval: overrides
            .old_interval
            .unwrap_or_else(|| canonical_form::<F::Interval>(0x35)),
        new_interval: overrides
            .new_interval
            .unwrap_or_else(|| canonical_form::<F::Interval>(0x36)),
        purpose,
        seller: overrides
            .seller
            .unwrap_or_else(|| canonical_form::<F::Identity>(0x37)),
        buyer: overrides
            .buyer
            .unwrap_or_else(|| canonical_form::<F::Identity>(0x38)),
        expiry: overrides.expiry.unwrap_or(1_700_000_000),
    }
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
        alpha: overrides.alpha.unwrap_or_else(|| default_scalar::<P>(0x51)),
        r: overrides.r.unwrap_or_else(|| default_scalar::<P>(0x52)),
        rho: overrides.rho.unwrap_or_else(|| default_scalar::<P>(0x53)),
        sigma: overrides.sigma.unwrap_or_else(|| default_scalar::<P>(0x54)),
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
    let responses = build_mint_responses(pairing, Default::default());
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
        x: overrides.x.unwrap_or_else(|| default_scalar::<P>(0x55)),
        y: overrides.y.unwrap_or_else(|| default_scalar::<P>(0x56)),
        s: overrides.s.unwrap_or_else(|| default_scalar::<P>(0x57)),
        rho: overrides.rho.unwrap_or_else(|| default_scalar::<P>(0x58)),
        sigma: overrides.sigma.unwrap_or_else(|| default_scalar::<P>(0x59)),
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
    let responses = build_transfer_responses(pairing, Default::default());
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
    let generator = generator_g2(pairing);
    MintSecondGroupFirstMessages {
        hpub: overrides
            .hpub
            .unwrap_or_else(|| product_g2(pairing, generator.clone(), default_scalar::<P>(0x5A))),
        d1: overrides
            .d1
            .unwrap_or_else(|| product_g2(pairing, generator.clone(), default_scalar::<P>(0x5B))),
        d2: overrides
            .d2
            .unwrap_or_else(|| product_g2(pairing, generator.clone(), default_scalar::<P>(0x5C))),
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
    let first_messages = build_mint_second_group_first_messages(pairing, Default::default());
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
    let generator = generator_g2(pairing);
    TransferSecondGroupFirstMessages {
        pk2: overrides
            .pk2
            .unwrap_or_else(|| product_g2(pairing, generator.clone(), default_scalar::<P>(0x5D))),
        d1: overrides
            .d1
            .unwrap_or_else(|| product_g2(pairing, generator.clone(), default_scalar::<P>(0x5E))),
        d2: overrides
            .d2
            .unwrap_or_else(|| product_g2(pairing, generator.clone(), default_scalar::<P>(0x5F))),
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
    let first_messages = build_transfer_second_group_first_messages(pairing, Default::default());
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
            .unwrap_or_else(|| default_scalar::<P>(0x50)),
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

pub fn invalidate_delivery_proof_wire_components<P: IPairingAdapter>(
    pairing: &P,
    corruptions: DeliveryProofWireComponentsCorruptions,
) -> Value {
    let wire = build_delivery_proof_wire_components(pairing, Default::default());
    let mut fields = Map::new();
    fields.insert(
        "relation".to_string(),
        Value::from(match wire.relation {
            DeliveryRelation::Mint => "Mint",
            DeliveryRelation::Transfer => "Transfer",
        }),
    );
    fields.insert(
        "verifier_form".to_string(),
        Value::from(match wire.verifier_form {
            VerifierGroupArithmetic::BothGroups => "BothGroups",
            VerifierGroupArithmetic::FirstGroupOnly => "FirstGroupOnly",
        }),
    );
    fields.insert(
        "challenge".to_string(),
        encoded_scalar_value(pairing, &wire.challenge),
    );
    fields.insert(
        "responses".to_string(),
        Value::Array(
            wire.responses
                .iter()
                .map(|response| encoded_scalar_value(pairing, response))
                .collect(),
        ),
    );
    fields.insert(
        "second_group_first_messages".to_string(),
        Value::Array(
            wire.second_group_first_messages
                .iter()
                .map(|message| encoded_g2_value(pairing, message))
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
    if let Some(second_group_first_messages) = corruptions.second_group_first_messages {
        fields.insert(
            "second_group_first_messages".to_string(),
            second_group_first_messages,
        );
    }
    Value::Object(fields)
}

fn context_value<F: IChainForms, Purpose>(
    context: &DeliveryContext<F, Purpose>,
    purpose: Value,
) -> Value {
    let mut fields = Map::new();
    let mut suite_identifier = Map::new();
    suite_identifier.insert(
        "identifier".to_string(),
        byte_array(context.suite_identifier.identifier()),
    );
    suite_identifier.insert(
        "version".to_string(),
        Value::from(context.suite_identifier.version()),
    );
    fields.insert(
        "suite_identifier".to_string(),
        Value::Object(suite_identifier),
    );
    fields.insert(
        "chain".to_string(),
        canonical_field_render(&canonical_field_value(&context.chain)),
    );
    fields.insert(
        "entitlement_contract".to_string(),
        canonical_field_render(&canonical_field_value(&context.entitlement_contract)),
    );
    fields.insert(
        "asset_identity_hash".to_string(),
        byte_array(context.asset_identity_hash.as_bytes()),
    );
    fields.insert(
        "parameter_set_identifier".to_string(),
        byte_array(context.parameter_set_identifier.as_bytes()),
    );
    fields.insert(
        "source_entitlement".to_string(),
        canonical_field_render(&canonical_field_value(&context.source_entitlement)),
    );
    fields.insert(
        "target_entitlement".to_string(),
        canonical_field_render(&canonical_field_value(&context.target_entitlement)),
    );
    fields.insert(
        "old_interval".to_string(),
        canonical_field_render(&canonical_field_value(&context.old_interval)),
    );
    fields.insert(
        "new_interval".to_string(),
        canonical_field_render(&canonical_field_value(&context.new_interval)),
    );
    fields.insert("purpose".to_string(), purpose);
    fields.insert(
        "seller".to_string(),
        canonical_field_render(&canonical_field_value(&context.seller)),
    );
    fields.insert(
        "buyer".to_string(),
        canonical_field_render(&canonical_field_value(&context.buyer)),
    );
    fields.insert("expiry".to_string(), Value::from(context.expiry));
    Value::Object(fields)
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
    let g1 = generator_g1(pairing);
    let g2 = generator_g2(pairing);
    let alpha = default_alpha::<P>();
    let r = default_credential_randomness::<P>().expose().clone();
    let coins = build_envelope_coins(pairing, Default::default());
    let rho = coins.rho.expose().clone();
    let sigma = coins.sigma.expose().clone();
    let identity_element = overrides
        .identity_element
        .unwrap_or_else(|| product_g1(pairing, g1.clone(), default_scalar::<P>(0x44)));
    let buyer_keys = overrides
        .buyer_keys
        .unwrap_or_else(|| build_public_keys_components(pairing, Default::default()));
    AlgebraicMintStatement {
        context: overrides
            .context
            .unwrap_or_else(|| build_delivery_context(MintPurpose::Mint, Default::default())),
        hpub: overrides
            .hpub
            .unwrap_or_else(|| product_g2(pairing, g2.clone(), alpha.clone())),
        envelope: overrides.envelope.unwrap_or_else(|| EnvelopeComponents {
            c1: product_g1(pairing, g1.clone(), rho.clone()),
            c2: msm_g1(
                pairing,
                vec![
                    MsmG1Term {
                        base: g1.clone(),
                        scalar: alpha,
                    },
                    MsmG1Term {
                        base: identity_element.clone(),
                        scalar: r.clone(),
                    },
                    MsmG1Term {
                        base: buyer_keys.pk1.clone(),
                        scalar: rho,
                    },
                ],
            ),
            d1: product_g2(pairing, g2.clone(), sigma.clone()),
            d2: msm_g2(
                pairing,
                vec![
                    MsmG2Term {
                        base: g2.clone(),
                        scalar: r,
                    },
                    MsmG2Term {
                        base: buyer_keys.pk2.clone(),
                        scalar: sigma,
                    },
                ],
            ),
        }),
        identity_element,
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
    let statement = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
    let mut fields = Map::new();
    fields.insert(
        "context".to_string(),
        context_value(
            &statement.context,
            Value::from(statement.context.purpose.code()),
        ),
    );
    fields.insert(
        "hpub".to_string(),
        encoded_g2_value(pairing, &statement.hpub),
    );
    fields.insert(
        "identity_element".to_string(),
        encoded_g1_value(pairing, &statement.identity_element),
    );
    fields.insert(
        "buyer_keys".to_string(),
        public_keys_value(pairing, &statement.buyer_keys),
    );
    fields.insert(
        "envelope".to_string(),
        envelope_value(pairing, &statement.envelope),
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

pub fn build_algebraic_transfer_statement<P: IPairingArithmetic, F: IChainForms>(
    pairing: &P,
    overrides: AlgebraicTransferStatementOverrides<P, F>,
) -> AlgebraicTransferStatement<F, P::G1, P::G2> {
    let g1 = generator_g1(pairing);
    let g2 = generator_g2(pairing);
    let seller_secrets = build_key_pair_components(pairing, Default::default());
    let x = seller_secrets.x.expose().clone();
    let y = seller_secrets.y.expose().clone();
    let s = default_offset::<P>().expose().clone();
    let coins = build_envelope_coins(pairing, Default::default());
    let rho = coins.rho.expose().clone();
    let sigma = coins.sigma.expose().clone();
    let identity_element = overrides
        .identity_element
        .unwrap_or_else(|| product_g1(pairing, g1.clone(), default_scalar::<P>(0x44)));
    let seller_keys = overrides
        .seller_keys
        .unwrap_or_else(|| PublicKeysComponents {
            pk1: product_g1(pairing, g1.clone(), x.clone()),
            pk2: product_g2(pairing, g2.clone(), y.clone()),
        });
    let buyer_keys = overrides
        .buyer_keys
        .unwrap_or_else(|| PublicKeysComponents {
            pk1: product_g1(pairing, g1.clone(), default_scalar::<P>(0x45)),
            pk2: product_g2(pairing, g2.clone(), default_scalar::<P>(0x46)),
        });
    let old_envelope = overrides
        .old_envelope
        .unwrap_or_else(|| EnvelopeComponents {
            c1: product_g1(pairing, g1.clone(), default_scalar::<P>(0x47)),
            c2: product_g1(pairing, g1.clone(), default_scalar::<P>(0x48)),
            d1: product_g2(pairing, g2.clone(), default_scalar::<P>(0x49)),
            d2: product_g2(pairing, g2.clone(), default_scalar::<P>(0x4A)),
        });
    let new_envelope = overrides.new_envelope.unwrap_or_else(|| {
        let c2 = sum_g1(
            pairing,
            old_envelope.c2.clone(),
            msm_g1(
                pairing,
                vec![
                    MsmG1Term {
                        base: old_envelope.c1.clone(),
                        scalar: negated_scalar(pairing, x),
                    },
                    MsmG1Term {
                        base: identity_element.clone(),
                        scalar: s.clone(),
                    },
                    MsmG1Term {
                        base: buyer_keys.pk1.clone(),
                        scalar: rho.clone(),
                    },
                ],
            ),
        );
        let d2 = sum_g2(
            pairing,
            old_envelope.d2.clone(),
            msm_g2(
                pairing,
                vec![
                    MsmG2Term {
                        base: old_envelope.d1.clone(),
                        scalar: negated_scalar(pairing, y),
                    },
                    MsmG2Term {
                        base: g2.clone(),
                        scalar: s,
                    },
                    MsmG2Term {
                        base: buyer_keys.pk2.clone(),
                        scalar: sigma.clone(),
                    },
                ],
            ),
        );
        EnvelopeComponents {
            c1: product_g1(pairing, g1.clone(), rho),
            c2,
            d1: product_g2(pairing, g2.clone(), sigma),
            d2,
        }
    });
    AlgebraicTransferStatement {
        context: overrides.context.unwrap_or_else(|| {
            build_delivery_context(TransferPurpose::Transfer, Default::default())
        }),
        identity_element,
        seller_keys,
        buyer_keys,
        old_envelope,
        new_envelope,
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
    let statement = build_algebraic_transfer_statement::<P, F>(pairing, Default::default());
    let mut fields = Map::new();
    fields.insert(
        "context".to_string(),
        context_value(
            &statement.context,
            Value::from(statement.context.purpose.code()),
        ),
    );
    fields.insert(
        "identity_element".to_string(),
        encoded_g1_value(pairing, &statement.identity_element),
    );
    fields.insert(
        "seller_keys".to_string(),
        public_keys_value(pairing, &statement.seller_keys),
    );
    fields.insert(
        "buyer_keys".to_string(),
        public_keys_value(pairing, &statement.buyer_keys),
    );
    fields.insert(
        "old_envelope".to_string(),
        envelope_value(pairing, &statement.old_envelope),
    );
    fields.insert(
        "new_envelope".to_string(),
        envelope_value(pairing, &statement.new_envelope),
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
            .unwrap_or_else(|| build_master_scalar_components(Default::default())),
        credential_randomness: overrides
            .credential_randomness
            .unwrap_or_else(|| default_credential_randomness::<P>()),
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
        offset: overrides.offset.unwrap_or_else(|| default_offset::<P>()),
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
    let success = build_verify_success_return(Default::default());
    corruptions
        .is_valid
        .unwrap_or_else(|| Value::from(success.is_valid))
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
            let mut messages = Map::new();
            messages.insert(
                "hpub".to_string(),
                encoded_g2_value(pairing, &first_messages.hpub),
            );
            messages.insert(
                "d1".to_string(),
                encoded_g2_value(pairing, &first_messages.d1),
            );
            messages.insert(
                "d2".to_string(),
                encoded_g2_value(pairing, &first_messages.d2),
            );
            fields.insert("first_messages".to_string(), Value::Object(messages));
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
            let mut messages = Map::new();
            messages.insert(
                "pk2".to_string(),
                encoded_g2_value(pairing, &first_messages.pk2),
            );
            messages.insert(
                "d1".to_string(),
                encoded_g2_value(pairing, &first_messages.d1),
            );
            messages.insert(
                "d2".to_string(),
                encoded_g2_value(pairing, &first_messages.d2),
            );
            fields.insert("first_messages".to_string(), Value::Object(messages));
        }
    }
    Value::Object(fields)
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
                challenge: default_scalar::<P>(0x50),
                responses: build_mint_responses(pairing, Default::default()),
            }
        }),
    }
}

#[derive(Default)]
pub struct MockDeliveryProofCorruptions {
    pub components: Option<Value>,
}

pub fn invalidate_mock_delivery_proof<P: IPairingAdapter>(
    pairing: &P,
    corruptions: MockDeliveryProofCorruptions,
) -> Value {
    let proof = build_mock_delivery_proof(pairing, Default::default());
    corruptions
        .components
        .unwrap_or_else(|| components_value(pairing, &proof.components))
}

pub(crate) struct MockIDeliveryProofAdapter<'a, P: IPairingArithmetic, F: IChainForms> {
    pub(super) pairing: &'a P,
    pub(super) verifier_form: VerifierGroupArithmetic,
    pub(super) failure_mode: MockIDeliveryProofAdapterFailureMode,
    pub(super) forms: PhantomData<F>,
}

pub(crate) struct MockIDeliveryProofAdapterConstructorParams<'a, P: IPairingArithmetic> {
    pub pairing: &'a P,
    pub verifier_form: VerifierGroupArithmetic,
    pub failure_mode: MockIDeliveryProofAdapterFailureMode,
}

pub(crate) type MockIDeliveryProofAdapterTryNewReturn<'a, P, F> =
    Result<MockIDeliveryProofAdapter<'a, P, F>, Infallible>;

impl<'a, P: IPairingArithmetic, F: IChainForms> MockIDeliveryProofAdapter<'a, P, F> {
    pub(crate) fn try_new(
        params: MockIDeliveryProofAdapterConstructorParams<'a, P>,
    ) -> MockIDeliveryProofAdapterTryNewReturn<'a, P, F> {
        Ok(MockIDeliveryProofAdapter {
            pairing: params.pairing,
            verifier_form: params.verifier_form,
            failure_mode: params.failure_mode,
            forms: PhantomData,
        })
    }
}

#[derive(Default)]
pub struct MockIDeliveryProofAdapterConstructorParamsOverrides {
    pub verifier_form: Option<VerifierGroupArithmetic>,
    pub failure_mode: Option<MockIDeliveryProofAdapterFailureMode>,
}

pub(crate) fn build_mock_i_delivery_proof_adapter_constructor_params<'a, P: IPairingArithmetic>(
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

pub(crate) fn build_mock_i_delivery_proof_adapter<'a, P: IPairingArithmetic, F: IChainForms>(
    pairing: &'a P,
    overrides: MockIDeliveryProofAdapterConstructorParamsOverrides,
) -> MockIDeliveryProofAdapter<'a, P, F> {
    let Ok(adapter) = MockIDeliveryProofAdapter::try_new(
        build_mock_i_delivery_proof_adapter_constructor_params(pairing, overrides),
    );
    adapter
}

fn verify_mint<P: IPairingArithmetic, F: IChainForms>(
    pairing: &P,
    statement: &AlgebraicMintStatement<F, P::G1, P::G2>,
    challenge: &P::Scalar,
    responses: &MintResponses<P::Scalar>,
    first_messages: Option<&MintSecondGroupFirstMessages<P::G2>>,
) -> bool {
    let g1 = generator_g1(pairing);
    let g2 = generator_g2(pairing);
    let n = negated_scalar(pairing, challenge.clone());
    let t_hpub = msm_g2(
        pairing,
        vec![
            MsmG2Term {
                base: g2.clone(),
                scalar: responses.alpha.clone(),
            },
            MsmG2Term {
                base: statement.hpub.clone(),
                scalar: n.clone(),
            },
        ],
    );
    let t_c1 = msm_g1(
        pairing,
        vec![
            MsmG1Term {
                base: g1.clone(),
                scalar: responses.rho.clone(),
            },
            MsmG1Term {
                base: statement.envelope.c1.clone(),
                scalar: n.clone(),
            },
        ],
    );
    let t_c2 = msm_g1(
        pairing,
        vec![
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
    );
    let t_d1 = msm_g2(
        pairing,
        vec![
            MsmG2Term {
                base: g2.clone(),
                scalar: responses.sigma.clone(),
            },
            MsmG2Term {
                base: statement.envelope.d1.clone(),
                scalar: n.clone(),
            },
        ],
    );
    let t_d2 = msm_g2(
        pairing,
        vec![
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
    );
    let recomputed =
        mint_challenge::<P, F>(pairing, statement, &t_hpub, &t_c1, &t_c2, &t_d1, &t_d2);
    if !scalars_equal(pairing, &recomputed, challenge) {
        return false;
    }
    match first_messages {
        None => true,
        Some(messages) => {
            g2_equal(pairing, &messages.hpub, &t_hpub)
                && g2_equal(pairing, &messages.d1, &t_d1)
                && g2_equal(pairing, &messages.d2, &t_d2)
        }
    }
}

fn verify_transfer<P: IPairingArithmetic, F: IChainForms>(
    pairing: &P,
    statement: &AlgebraicTransferStatement<F, P::G1, P::G2>,
    challenge: &P::Scalar,
    responses: &TransferResponses<P::Scalar>,
    first_messages: Option<&TransferSecondGroupFirstMessages<P::G2>>,
) -> bool {
    let g1 = generator_g1(pairing);
    let g2 = generator_g2(pairing);
    let n = negated_scalar(pairing, challenge.clone());
    let delta_c2 = sum_g1(
        pairing,
        statement.new_envelope.c2.clone(),
        negated_g1(pairing, statement.old_envelope.c2.clone()),
    );
    let delta_d2 = sum_g2(
        pairing,
        statement.new_envelope.d2.clone(),
        negated_g2(pairing, statement.old_envelope.d2.clone()),
    );
    let t_pk1 = msm_g1(
        pairing,
        vec![
            MsmG1Term {
                base: g1.clone(),
                scalar: responses.x.clone(),
            },
            MsmG1Term {
                base: statement.seller_keys.pk1.clone(),
                scalar: n.clone(),
            },
        ],
    );
    let t_c1 = msm_g1(
        pairing,
        vec![
            MsmG1Term {
                base: g1.clone(),
                scalar: responses.rho.clone(),
            },
            MsmG1Term {
                base: statement.new_envelope.c1.clone(),
                scalar: n.clone(),
            },
        ],
    );
    let t_c2 = msm_g1(
        pairing,
        vec![
            MsmG1Term {
                base: statement.old_envelope.c1.clone(),
                scalar: negated_scalar(pairing, responses.x.clone()),
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
    );
    let t_pk2 = msm_g2(
        pairing,
        vec![
            MsmG2Term {
                base: g2.clone(),
                scalar: responses.y.clone(),
            },
            MsmG2Term {
                base: statement.seller_keys.pk2.clone(),
                scalar: n.clone(),
            },
        ],
    );
    let t_d1 = msm_g2(
        pairing,
        vec![
            MsmG2Term {
                base: g2.clone(),
                scalar: responses.sigma.clone(),
            },
            MsmG2Term {
                base: statement.new_envelope.d1.clone(),
                scalar: n.clone(),
            },
        ],
    );
    let t_d2 = msm_g2(
        pairing,
        vec![
            MsmG2Term {
                base: statement.old_envelope.d1.clone(),
                scalar: negated_scalar(pairing, responses.y.clone()),
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
    );
    let recomputed = transfer_challenge::<P, F>(
        pairing, statement, &t_pk1, &t_c1, &t_c2, &t_pk2, &t_d1, &t_d2,
    );
    if !scalars_equal(pairing, &recomputed, challenge) {
        return false;
    }
    match first_messages {
        None => true,
        Some(messages) => {
            g2_equal(pairing, &messages.pk2, &t_pk2)
                && g2_equal(pairing, &messages.d1, &t_d1)
                && g2_equal(pairing, &messages.d2, &t_d2)
        }
    }
}

fn clone_mint_responses<S: Clone>(responses: &MintResponses<S>) -> MintResponses<S> {
    MintResponses {
        alpha: responses.alpha.clone(),
        r: responses.r.clone(),
        rho: responses.rho.clone(),
        sigma: responses.sigma.clone(),
    }
}

fn clone_transfer_responses<S: Clone>(responses: &TransferResponses<S>) -> TransferResponses<S> {
    TransferResponses {
        x: responses.x.clone(),
        y: responses.y.clone(),
        s: responses.s.clone(),
        rho: responses.rho.clone(),
        sigma: responses.sigma.clone(),
    }
}

fn clone_components<S: Clone, G2: Clone>(
    components: &DeliveryProofComponents<S, G2>,
) -> DeliveryProofComponents<S, G2> {
    match components {
        DeliveryProofComponents::MintBothGroups {
            challenge,
            responses,
        } => DeliveryProofComponents::MintBothGroups {
            challenge: challenge.clone(),
            responses: clone_mint_responses(responses),
        },
        DeliveryProofComponents::MintFirstGroupOnly {
            challenge,
            responses,
            first_messages,
        } => DeliveryProofComponents::MintFirstGroupOnly {
            challenge: challenge.clone(),
            responses: clone_mint_responses(responses),
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
            responses: clone_transfer_responses(responses),
        },
        DeliveryProofComponents::TransferFirstGroupOnly {
            challenge,
            responses,
            first_messages,
        } => DeliveryProofComponents::TransferFirstGroupOnly {
            challenge: challenge.clone(),
            responses: clone_transfer_responses(responses),
            first_messages: TransferSecondGroupFirstMessages {
                pk2: first_messages.pk2.clone(),
                d1: first_messages.d1.clone(),
                d2: first_messages.d2.clone(),
            },
        },
    }
}

impl<'a, P: IPairingArithmetic, F: IChainForms> IDeliveryProofAdapter
    for MockIDeliveryProofAdapter<'a, P, F>
{
    const DECLARATION: DeliveryProofDeclaration = DeliveryProofDeclaration {
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

    type Pairing = P;
    type Forms = F;
    type Proof = MockDeliveryProof<P>;

    fn prove_mint(
        &self,
        _params: ProveMintParams,
        payload: ProveMintPayload<'_, F, P::Scalar, P::G1, P::G2>,
    ) -> ProveMintReturn<Self::Proof> {
        if matches!(
            self.failure_mode,
            MockIDeliveryProofAdapterFailureMode::NonceDrawRefused
        ) {
            return Err(ProveMintErrorReturn::Mock(
                MockIDeliveryProofAdapterProveMintErrorReturn::NonceDrawRefused,
            ));
        }
        let statement = payload.statement;
        let g1 = generator_g1(self.pairing);
        let g2 = generator_g2(self.pairing);
        let k_alpha = default_scalar::<P>(0x61);
        let k_r = default_scalar::<P>(0x62);
        let k_rho = default_scalar::<P>(0x63);
        let k_sigma = default_scalar::<P>(0x64);
        let t_hpub = product_g2(self.pairing, g2.clone(), k_alpha.clone());
        let t_c1 = product_g1(self.pairing, g1.clone(), k_rho.clone());
        let t_c2 = msm_g1(
            self.pairing,
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
        let t_d1 = product_g2(self.pairing, g2.clone(), k_sigma.clone());
        let t_d2 = msm_g2(
            self.pairing,
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
        let challenge =
            mint_challenge::<P, F>(self.pairing, statement, &t_hpub, &t_c1, &t_c2, &t_d1, &t_d2);
        let responses = MintResponses {
            alpha: response(
                self.pairing,
                k_alpha,
                &challenge,
                payload.master_scalar.value.expose(),
            ),
            r: response(
                self.pairing,
                k_r,
                &challenge,
                payload.credential_randomness.expose(),
            ),
            rho: response(self.pairing, k_rho, &challenge, payload.coins.rho.expose()),
            sigma: response(
                self.pairing,
                k_sigma,
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
                        hpub: t_hpub,
                        d1: t_d1,
                        d2: t_d2,
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
        payload: ProveTransferPayload<'_, F, P::Scalar, P::G1, P::G2>,
    ) -> ProveTransferReturn<Self::Proof> {
        if matches!(
            self.failure_mode,
            MockIDeliveryProofAdapterFailureMode::NonceDrawRefused
        ) {
            return Err(ProveTransferErrorReturn::Mock(
                MockIDeliveryProofAdapterProveTransferErrorReturn::NonceDrawRefused,
            ));
        }
        let statement = payload.statement;
        let g1 = generator_g1(self.pairing);
        let g2 = generator_g2(self.pairing);
        let k_x = default_scalar::<P>(0x71);
        let k_y = default_scalar::<P>(0x72);
        let k_s = default_scalar::<P>(0x73);
        let k_rho = default_scalar::<P>(0x74);
        let k_sigma = default_scalar::<P>(0x75);
        let t_pk1 = product_g1(self.pairing, g1.clone(), k_x.clone());
        let t_c1 = product_g1(self.pairing, g1.clone(), k_rho.clone());
        let t_c2 = msm_g1(
            self.pairing,
            vec![
                MsmG1Term {
                    base: statement.old_envelope.c1.clone(),
                    scalar: negated_scalar(self.pairing, k_x.clone()),
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
        let t_pk2 = product_g2(self.pairing, g2.clone(), k_y.clone());
        let t_d1 = product_g2(self.pairing, g2.clone(), k_sigma.clone());
        let t_d2 = msm_g2(
            self.pairing,
            vec![
                MsmG2Term {
                    base: statement.old_envelope.d1.clone(),
                    scalar: negated_scalar(self.pairing, k_y.clone()),
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
        let challenge = transfer_challenge::<P, F>(
            self.pairing,
            statement,
            &t_pk1,
            &t_c1,
            &t_c2,
            &t_pk2,
            &t_d1,
            &t_d2,
        );
        let responses = TransferResponses {
            x: response(
                self.pairing,
                k_x,
                &challenge,
                payload.seller_secrets.x.expose(),
            ),
            y: response(
                self.pairing,
                k_y,
                &challenge,
                payload.seller_secrets.y.expose(),
            ),
            s: response(self.pairing, k_s, &challenge, payload.offset.expose()),
            rho: response(self.pairing, k_rho, &challenge, payload.coins.rho.expose()),
            sigma: response(
                self.pairing,
                k_sigma,
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
                        pk2: t_pk2,
                        d1: t_d1,
                        d2: t_d2,
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
        payload: VerifyPayload<'_, F, P::G1, P::G2, Self::Proof>,
    ) -> VerifyReturn {
        let is_valid = match payload.statement {
            AlgebraicStatement::Mint(statement) => match &payload.proof.components {
                DeliveryProofComponents::MintBothGroups {
                    challenge,
                    responses,
                } if self.verifier_form == VerifierGroupArithmetic::BothGroups => {
                    verify_mint(self.pairing, statement, challenge, responses, None)
                }
                DeliveryProofComponents::MintFirstGroupOnly {
                    challenge,
                    responses,
                    first_messages,
                } if self.verifier_form == VerifierGroupArithmetic::FirstGroupOnly => verify_mint(
                    self.pairing,
                    statement,
                    challenge,
                    responses,
                    Some(first_messages),
                ),
                _ => false,
            },
            AlgebraicStatement::Transfer(statement) => match &payload.proof.components {
                DeliveryProofComponents::TransferBothGroups {
                    challenge,
                    responses,
                } if self.verifier_form == VerifierGroupArithmetic::BothGroups => {
                    verify_transfer(self.pairing, statement, challenge, responses, None)
                }
                DeliveryProofComponents::TransferFirstGroupOnly {
                    challenge,
                    responses,
                    first_messages,
                } if self.verifier_form == VerifierGroupArithmetic::FirstGroupOnly => {
                    verify_transfer(
                        self.pairing,
                        statement,
                        challenge,
                        responses,
                        Some(first_messages),
                    )
                }
                _ => false,
            },
        };
        Ok(VerifySuccessReturn { is_valid })
    }

    fn proof_components(
        &self,
        _params: ProofComponentsParams,
        payload: ProofComponentsPayload<'_, Self::Proof>,
    ) -> ProofComponentsReturn<P::Scalar, P::G2> {
        Ok(ProofComponentsSuccessReturn {
            components: clone_components(&payload.proof.components),
        })
    }

    fn proof_from_components(
        &self,
        _params: ProofFromComponentsParams,
        payload: ProofFromComponentsPayload<P::Scalar, P::G2>,
    ) -> ProofFromComponentsReturn<Self::Proof> {
        let components_form = match &payload.components {
            DeliveryProofComponents::MintBothGroups { .. }
            | DeliveryProofComponents::TransferBothGroups { .. } => {
                VerifierGroupArithmetic::BothGroups
            }
            DeliveryProofComponents::MintFirstGroupOnly { .. }
            | DeliveryProofComponents::TransferFirstGroupOnly { .. } => {
                VerifierGroupArithmetic::FirstGroupOnly
            }
        };
        if components_form != self.verifier_form {
            return Err(ProofFromComponentsErrorReturn::VerifierFormMismatch);
        }
        Ok(ProofFromComponentsSuccessReturn {
            proof: MockDeliveryProof {
                components: payload.components,
            },
        })
    }
}

#[derive(Default)]
pub struct MockIDeliveryProofConsumer;

impl<P: IPairingAdapter, F: IChainForms> IDeliveryProofConsumer<P, F>
    for MockIDeliveryProofConsumer
{
    type Output = ();

    fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
        &self,
        _params: ConsumeDeliveryProofParams,
        _payload: ConsumeDeliveryProofPayload<D>,
    ) -> Self::Output {
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
pub struct CreateDeliveryProofParamsOverrides {
    pub concrete: Option<DeliveryProofConcrete>,
    pub algebra: Option<EnvelopeAlgebra>,
    pub verifier_form: Option<VerifierGroupArithmetic>,
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
        verifier_form: overrides
            .verifier_form
            .unwrap_or(VerifierGroupArithmetic::BothGroups),
        statement_version: overrides
            .statement_version
            .unwrap_or(DELIVERY_STATEMENT_VERSION_ONE),
    }
}

#[derive(Default)]
pub struct CreateDeliveryProofParamsCorruptions {
    pub concrete: Option<Value>,
    pub algebra: Option<Value>,
    pub verifier_form: Option<Value>,
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
        "verifier_form".to_string(),
        Value::from(match params.verifier_form {
            VerifierGroupArithmetic::BothGroups => "BothGroups",
            VerifierGroupArithmetic::FirstGroupOnly => "FirstGroupOnly",
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
    if let Some(verifier_form) = corruptions.verifier_form {
        fields.insert("verifier_form".to_string(), verifier_form);
    }
    if let Some(statement_version) = corruptions.statement_version {
        fields.insert("statement_version".to_string(), statement_version);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct CreateDeliveryProofPayloadOverrides;

pub fn build_create_delivery_proof_payload(
    _overrides: CreateDeliveryProofPayloadOverrides,
) -> CreateDeliveryProofPayload {
    CreateDeliveryProofPayload
}

#[derive(Default)]
pub struct CreateDeliveryProofSuccessReturnOverrides;

pub fn build_create_delivery_proof_success_return<O>(
    output: O,
    _overrides: CreateDeliveryProofSuccessReturnOverrides,
) -> CreateDeliveryProofSuccessReturn<O> {
    CreateDeliveryProofSuccessReturn { output }
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
    let _: CreateDeliveryProofFn<'a, P, E, F, C> = mock_create_delivery_proof;
    let adapter = build_mock_i_delivery_proof_adapter::<P, F>(deps.pairing, Default::default());
    let output = deps.consumer.consume_delivery_proof(
        build_consume_delivery_proof_params(Default::default()),
        build_consume_delivery_proof_payload(adapter, Default::default()),
    );
    Ok(build_create_delivery_proof_success_return(
        output,
        Default::default(),
    ))
}
