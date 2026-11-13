#![allow(dead_code, clippy::too_many_arguments)]

use chain::{
    ConsumeChainFormsParams, ConsumeChainFormsPayload, CreateChainFormsParams, IChainForms,
    IChainFormsConsumer, build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, create_chain_forms,
};
use core::marker::PhantomData;
use encoding::{
    CanonicalFieldValue, ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingParams,
    IDecoderAdapter, IEncoderAdapter, IEncodingConsumer, build_create_encoding_deps,
    build_create_encoding_params, build_create_encoding_payload, create_encoding,
};
use envelope::{
    ConsumeKeyAgreementParams, ConsumeKeyAgreementPayload, CreateKeyAgreementParams,
    IKeyAgreementAdapter, IKeyAgreementConsumer, build_create_key_agreement_deps,
    build_create_key_agreement_params, build_create_key_agreement_payload, create_key_agreement,
};
use hash_to_scalar::{
    CreateHashToScalarParams, IHashToScalarAdapter, build_create_hash_to_scalar_deps,
    build_create_hash_to_scalar_params, build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use hashing::Root;
use kem::{
    ConsumeKemParams, ConsumeKemPayload, CreateKemParams, ICredentialKemAdapter, IKemConsumer,
    IdentityScope, KemIdentity, SetupScope, build_create_kem_deps, build_create_kem_params,
    build_create_kem_payload, create_kem,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParams, IPairingAdapter,
    IPairingArithmetic, IPairingConsumer, IPairingReference, build_create_pairing_deps,
    build_create_pairing_params, build_create_pairing_payload, create_pairing,
};
use proof::{
    ConsumeDeliveryProofParams, ConsumeDeliveryProofPayload, CreateDeliveryProofParams,
    DeliveryContext, IDeliveryProofAdapter, IDeliveryProofConsumer, MintPurpose, TransferPurpose,
    build_create_delivery_proof_deps, build_create_delivery_proof_params,
    build_create_delivery_proof_payload, create_delivery_proof,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParams, CreateRandomSourcePayload,
    IRandomSourceAdapter, build_create_random_source_params, create_random_source,
};
use serde_json::{Map, Value};

use super::super::super::factory::provides::{VectorCount, build_vector_count};
use super::interface::{
    ConsumeProofVectorsCaseParams, ConsumeProofVectorsCasePayload, IProofVectorsCaseConsumer,
    MintProofVector, MintProofVectorDescription, MintProofVectorDescriptionConstructorParams,
    ProofVectorContext, ProofVectorsCaseDeps, ProofVectorsCaseErrorReturn, ProofVectorsCaseParams,
    ProofVectorsCasePayload, ProofVectorsCaseReturn, ProofVectorsCaseSuccessReturn,
    ProofVectorsDeps, ProofVectorsFn, ProofVectorsParams, ProofVectorsPayload, ProofVectorsReturn,
    ProofVectorsSuccessReturn, RunProofVectorsCaseFn, TransferProofVector,
    TransferProofVectorDescription, TransferProofVectorDescriptionConstructorParams,
};
use crate::delivery::provides::{AdmittedKemIdentity, build_admitted_kem_identity};

#[derive(Default)]
pub struct ProofVectorContextOverrides {
    pub suite_identifier: Option<[u8; 32]>,
    pub suite_version: Option<u16>,
    pub chain: Option<CanonicalFieldValue>,
    pub entitlement_contract: Option<CanonicalFieldValue>,
    pub asset_root: Option<[u8; 32]>,
    pub parameter_set: Option<[u8; 32]>,
    pub source_entitlement: Option<CanonicalFieldValue>,
    pub target_entitlement: Option<CanonicalFieldValue>,
    pub old_interval: Option<CanonicalFieldValue>,
    pub new_interval: Option<CanonicalFieldValue>,
    pub purpose: Option<u16>,
    pub seller: Option<CanonicalFieldValue>,
    pub buyer: Option<CanonicalFieldValue>,
    pub expiry: Option<u64>,
}

pub fn build_proof_vector_context(overrides: ProofVectorContextOverrides) -> ProofVectorContext {
    ProofVectorContext {
        suite_identifier: overrides.suite_identifier.unwrap_or([0x22; 32]),
        suite_version: overrides.suite_version.unwrap_or(1),
        chain: overrides
            .chain
            .unwrap_or(CanonicalFieldValue::Unsigned256([0x2A; 32])),
        entitlement_contract: overrides
            .entitlement_contract
            .unwrap_or(CanonicalFieldValue::FixedBytes20([0x31; 20])),
        asset_root: overrides.asset_root.unwrap_or([0x01; 32]),
        parameter_set: overrides.parameter_set.unwrap_or([0x33; 32]),
        source_entitlement: overrides
            .source_entitlement
            .unwrap_or(CanonicalFieldValue::Unsigned256([0x41; 32])),
        target_entitlement: overrides
            .target_entitlement
            .unwrap_or(CanonicalFieldValue::Unsigned256([0x42; 32])),
        old_interval: overrides
            .old_interval
            .unwrap_or(CanonicalFieldValue::Unsigned64(1)),
        new_interval: overrides
            .new_interval
            .unwrap_or(CanonicalFieldValue::Unsigned64(2)),
        purpose: overrides.purpose.unwrap_or(1),
        seller: overrides
            .seller
            .unwrap_or(CanonicalFieldValue::FixedBytes20([0x61; 20])),
        buyer: overrides
            .buyer
            .unwrap_or(CanonicalFieldValue::FixedBytes20([0x62; 20])),
        expiry: overrides.expiry.unwrap_or(1_800_000_000),
    }
}

fn byte_array(bytes: &[u8]) -> Value {
    Value::Array(bytes.iter().map(|byte| Value::from(*byte)).collect())
}

fn canonical_field_value(value: &CanonicalFieldValue) -> Value {
    let mut variant = Map::new();
    match value {
        CanonicalFieldValue::FixedBytes32(bytes) => {
            variant.insert("FixedBytes32".to_string(), byte_array(bytes));
        }
        CanonicalFieldValue::Unsigned16(value) => {
            variant.insert("Unsigned16".to_string(), Value::from(*value));
        }
        CanonicalFieldValue::Unsigned32(value) => {
            variant.insert("Unsigned32".to_string(), Value::from(*value));
        }
        CanonicalFieldValue::Unsigned64(value) => {
            variant.insert("Unsigned64".to_string(), Value::from(*value));
        }
        CanonicalFieldValue::Text(text) => {
            variant.insert("Text".to_string(), Value::from(text.clone()));
        }
        CanonicalFieldValue::Bytes(bytes) => {
            variant.insert("Bytes".to_string(), byte_array(bytes));
        }
        CanonicalFieldValue::FixedBytes20(bytes) => {
            variant.insert("FixedBytes20".to_string(), byte_array(bytes));
        }
        CanonicalFieldValue::Unsigned256(bytes) => {
            variant.insert("Unsigned256".to_string(), byte_array(bytes));
        }
    }
    Value::Object(variant)
}

fn proof_vector_context_fields(context: &ProofVectorContext) -> Map<String, Value> {
    let mut fields = Map::new();
    fields.insert(
        "suite_identifier".to_string(),
        byte_array(&context.suite_identifier),
    );
    fields.insert(
        "suite_version".to_string(),
        Value::from(context.suite_version),
    );
    fields.insert("chain".to_string(), canonical_field_value(&context.chain));
    fields.insert(
        "entitlement_contract".to_string(),
        canonical_field_value(&context.entitlement_contract),
    );
    fields.insert("asset_root".to_string(), byte_array(&context.asset_root));
    fields.insert(
        "parameter_set".to_string(),
        byte_array(&context.parameter_set),
    );
    fields.insert(
        "source_entitlement".to_string(),
        canonical_field_value(&context.source_entitlement),
    );
    fields.insert(
        "target_entitlement".to_string(),
        canonical_field_value(&context.target_entitlement),
    );
    fields.insert(
        "old_interval".to_string(),
        canonical_field_value(&context.old_interval),
    );
    fields.insert(
        "new_interval".to_string(),
        canonical_field_value(&context.new_interval),
    );
    fields.insert("purpose".to_string(), Value::from(context.purpose));
    fields.insert("seller".to_string(), canonical_field_value(&context.seller));
    fields.insert("buyer".to_string(), canonical_field_value(&context.buyer));
    fields.insert("expiry".to_string(), Value::from(context.expiry));
    fields
}

#[derive(Default)]
pub struct ProofVectorContextCorruptions {
    pub suite_identifier: Option<Value>,
    pub suite_version: Option<Value>,
    pub chain: Option<Value>,
    pub entitlement_contract: Option<Value>,
    pub asset_root: Option<Value>,
    pub parameter_set: Option<Value>,
    pub source_entitlement: Option<Value>,
    pub target_entitlement: Option<Value>,
    pub old_interval: Option<Value>,
    pub new_interval: Option<Value>,
    pub purpose: Option<Value>,
    pub seller: Option<Value>,
    pub buyer: Option<Value>,
    pub expiry: Option<Value>,
}

pub fn invalidate_proof_vector_context(corruptions: ProofVectorContextCorruptions) -> Value {
    let mut fields = proof_vector_context_fields(&build_proof_vector_context(Default::default()));
    if let Some(suite_identifier) = corruptions.suite_identifier {
        fields.insert("suite_identifier".to_string(), suite_identifier);
    }
    if let Some(suite_version) = corruptions.suite_version {
        fields.insert("suite_version".to_string(), suite_version);
    }
    if let Some(chain) = corruptions.chain {
        fields.insert("chain".to_string(), chain);
    }
    if let Some(entitlement_contract) = corruptions.entitlement_contract {
        fields.insert("entitlement_contract".to_string(), entitlement_contract);
    }
    if let Some(asset_root) = corruptions.asset_root {
        fields.insert("asset_root".to_string(), asset_root);
    }
    if let Some(parameter_set) = corruptions.parameter_set {
        fields.insert("parameter_set".to_string(), parameter_set);
    }
    if let Some(source_entitlement) = corruptions.source_entitlement {
        fields.insert("source_entitlement".to_string(), source_entitlement);
    }
    if let Some(target_entitlement) = corruptions.target_entitlement {
        fields.insert("target_entitlement".to_string(), target_entitlement);
    }
    if let Some(old_interval) = corruptions.old_interval {
        fields.insert("old_interval".to_string(), old_interval);
    }
    if let Some(new_interval) = corruptions.new_interval {
        fields.insert("new_interval".to_string(), new_interval);
    }
    if let Some(purpose) = corruptions.purpose {
        fields.insert("purpose".to_string(), purpose);
    }
    if let Some(seller) = corruptions.seller {
        fields.insert("seller".to_string(), seller);
    }
    if let Some(buyer) = corruptions.buyer {
        fields.insert("buyer".to_string(), buyer);
    }
    if let Some(expiry) = corruptions.expiry {
        fields.insert("expiry".to_string(), expiry);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct MintProofVectorOverrides {
    pub draws: Option<Vec<u8>>,
    pub context: Option<ProofVectorContext>,
    pub hpub: Option<Vec<u8>>,
    pub identity_element: Option<Vec<u8>>,
    pub buyer_pk1: Option<Vec<u8>>,
    pub buyer_pk2: Option<Vec<u8>>,
    pub envelope_c1: Option<Vec<u8>>,
    pub envelope_c2: Option<Vec<u8>>,
    pub envelope_d1: Option<Vec<u8>>,
    pub envelope_d2: Option<Vec<u8>>,
    pub challenge: Option<[u8; 32]>,
    pub alpha: Option<[u8; 32]>,
    pub r: Option<[u8; 32]>,
    pub rho: Option<[u8; 32]>,
    pub sigma: Option<[u8; 32]>,
    pub second_group_first_messages: Option<Vec<u8>>,
}

pub fn build_mint_proof_vector(overrides: MintProofVectorOverrides) -> MintProofVector {
    MintProofVector {
        draws: overrides.draws.unwrap_or_else(|| vec![0x01; 64]),
        context: overrides
            .context
            .unwrap_or_else(|| build_proof_vector_context(Default::default())),
        hpub: overrides.hpub.unwrap_or_else(|| vec![0x10; 128]),
        identity_element: overrides.identity_element.unwrap_or_else(|| vec![0x11; 64]),
        buyer_pk1: overrides.buyer_pk1.unwrap_or_else(|| vec![0x12; 64]),
        buyer_pk2: overrides.buyer_pk2.unwrap_or_else(|| vec![0x13; 128]),
        envelope_c1: overrides.envelope_c1.unwrap_or_else(|| vec![0x14; 64]),
        envelope_c2: overrides.envelope_c2.unwrap_or_else(|| vec![0x15; 64]),
        envelope_d1: overrides.envelope_d1.unwrap_or_else(|| vec![0x16; 128]),
        envelope_d2: overrides.envelope_d2.unwrap_or_else(|| vec![0x17; 128]),
        challenge: overrides.challenge.unwrap_or([0x20; 32]),
        alpha: overrides.alpha.unwrap_or([0x21; 32]),
        r: overrides.r.unwrap_or([0x22; 32]),
        rho: overrides.rho.unwrap_or([0x23; 32]),
        sigma: overrides.sigma.unwrap_or([0x24; 32]),
        second_group_first_messages: overrides.second_group_first_messages.unwrap_or_default(),
    }
}

fn mint_proof_vector_fields(vector: &MintProofVector) -> Map<String, Value> {
    let mut fields = Map::new();
    fields.insert("draws".to_string(), byte_array(&vector.draws));
    fields.insert(
        "context".to_string(),
        Value::Object(proof_vector_context_fields(&vector.context)),
    );
    fields.insert("hpub".to_string(), byte_array(&vector.hpub));
    fields.insert(
        "identity_element".to_string(),
        byte_array(&vector.identity_element),
    );
    fields.insert("buyer_pk1".to_string(), byte_array(&vector.buyer_pk1));
    fields.insert("buyer_pk2".to_string(), byte_array(&vector.buyer_pk2));
    fields.insert("envelope_c1".to_string(), byte_array(&vector.envelope_c1));
    fields.insert("envelope_c2".to_string(), byte_array(&vector.envelope_c2));
    fields.insert("envelope_d1".to_string(), byte_array(&vector.envelope_d1));
    fields.insert("envelope_d2".to_string(), byte_array(&vector.envelope_d2));
    fields.insert("challenge".to_string(), byte_array(&vector.challenge));
    fields.insert("alpha".to_string(), byte_array(&vector.alpha));
    fields.insert("r".to_string(), byte_array(&vector.r));
    fields.insert("rho".to_string(), byte_array(&vector.rho));
    fields.insert("sigma".to_string(), byte_array(&vector.sigma));
    fields.insert(
        "second_group_first_messages".to_string(),
        byte_array(&vector.second_group_first_messages),
    );
    fields
}

#[derive(Default)]
pub struct MintProofVectorCorruptions {
    pub draws: Option<Value>,
    pub context: Option<Value>,
    pub hpub: Option<Value>,
    pub identity_element: Option<Value>,
    pub buyer_pk1: Option<Value>,
    pub buyer_pk2: Option<Value>,
    pub envelope_c1: Option<Value>,
    pub envelope_c2: Option<Value>,
    pub envelope_d1: Option<Value>,
    pub envelope_d2: Option<Value>,
    pub challenge: Option<Value>,
    pub alpha: Option<Value>,
    pub r: Option<Value>,
    pub rho: Option<Value>,
    pub sigma: Option<Value>,
    pub second_group_first_messages: Option<Value>,
}

pub fn invalidate_mint_proof_vector(corruptions: MintProofVectorCorruptions) -> Value {
    let mut fields = mint_proof_vector_fields(&build_mint_proof_vector(Default::default()));
    if let Some(draws) = corruptions.draws {
        fields.insert("draws".to_string(), draws);
    }
    if let Some(context) = corruptions.context {
        fields.insert("context".to_string(), context);
    }
    if let Some(hpub) = corruptions.hpub {
        fields.insert("hpub".to_string(), hpub);
    }
    if let Some(identity_element) = corruptions.identity_element {
        fields.insert("identity_element".to_string(), identity_element);
    }
    if let Some(buyer_pk1) = corruptions.buyer_pk1 {
        fields.insert("buyer_pk1".to_string(), buyer_pk1);
    }
    if let Some(buyer_pk2) = corruptions.buyer_pk2 {
        fields.insert("buyer_pk2".to_string(), buyer_pk2);
    }
    if let Some(envelope_c1) = corruptions.envelope_c1 {
        fields.insert("envelope_c1".to_string(), envelope_c1);
    }
    if let Some(envelope_c2) = corruptions.envelope_c2 {
        fields.insert("envelope_c2".to_string(), envelope_c2);
    }
    if let Some(envelope_d1) = corruptions.envelope_d1 {
        fields.insert("envelope_d1".to_string(), envelope_d1);
    }
    if let Some(envelope_d2) = corruptions.envelope_d2 {
        fields.insert("envelope_d2".to_string(), envelope_d2);
    }
    if let Some(challenge) = corruptions.challenge {
        fields.insert("challenge".to_string(), challenge);
    }
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
    if let Some(second_group_first_messages) = corruptions.second_group_first_messages {
        fields.insert(
            "second_group_first_messages".to_string(),
            second_group_first_messages,
        );
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct TransferProofVectorOverrides {
    pub draws: Option<Vec<u8>>,
    pub context: Option<ProofVectorContext>,
    pub identity_element: Option<Vec<u8>>,
    pub seller_pk1: Option<Vec<u8>>,
    pub seller_pk2: Option<Vec<u8>>,
    pub buyer_pk1: Option<Vec<u8>>,
    pub buyer_pk2: Option<Vec<u8>>,
    pub old_envelope_c1: Option<Vec<u8>>,
    pub old_envelope_c2: Option<Vec<u8>>,
    pub old_envelope_d1: Option<Vec<u8>>,
    pub old_envelope_d2: Option<Vec<u8>>,
    pub new_envelope_c1: Option<Vec<u8>>,
    pub new_envelope_c2: Option<Vec<u8>>,
    pub new_envelope_d1: Option<Vec<u8>>,
    pub new_envelope_d2: Option<Vec<u8>>,
    pub challenge: Option<[u8; 32]>,
    pub x: Option<[u8; 32]>,
    pub y: Option<[u8; 32]>,
    pub s: Option<[u8; 32]>,
    pub rho: Option<[u8; 32]>,
    pub sigma: Option<[u8; 32]>,
    pub second_group_first_messages: Option<Vec<u8>>,
}

pub fn build_transfer_proof_vector(overrides: TransferProofVectorOverrides) -> TransferProofVector {
    TransferProofVector {
        draws: overrides.draws.unwrap_or_else(|| vec![0x01; 64]),
        context: overrides
            .context
            .unwrap_or_else(|| build_proof_vector_context(Default::default())),
        identity_element: overrides.identity_element.unwrap_or_else(|| vec![0x11; 64]),
        seller_pk1: overrides.seller_pk1.unwrap_or_else(|| vec![0x12; 64]),
        seller_pk2: overrides.seller_pk2.unwrap_or_else(|| vec![0x13; 128]),
        buyer_pk1: overrides.buyer_pk1.unwrap_or_else(|| vec![0x18; 64]),
        buyer_pk2: overrides.buyer_pk2.unwrap_or_else(|| vec![0x19; 128]),
        old_envelope_c1: overrides.old_envelope_c1.unwrap_or_else(|| vec![0x14; 64]),
        old_envelope_c2: overrides.old_envelope_c2.unwrap_or_else(|| vec![0x15; 64]),
        old_envelope_d1: overrides.old_envelope_d1.unwrap_or_else(|| vec![0x16; 128]),
        old_envelope_d2: overrides.old_envelope_d2.unwrap_or_else(|| vec![0x17; 128]),
        new_envelope_c1: overrides.new_envelope_c1.unwrap_or_else(|| vec![0x1A; 64]),
        new_envelope_c2: overrides.new_envelope_c2.unwrap_or_else(|| vec![0x1B; 64]),
        new_envelope_d1: overrides.new_envelope_d1.unwrap_or_else(|| vec![0x1C; 128]),
        new_envelope_d2: overrides.new_envelope_d2.unwrap_or_else(|| vec![0x1D; 128]),
        challenge: overrides.challenge.unwrap_or([0x20; 32]),
        x: overrides.x.unwrap_or([0x25; 32]),
        y: overrides.y.unwrap_or([0x26; 32]),
        s: overrides.s.unwrap_or([0x27; 32]),
        rho: overrides.rho.unwrap_or([0x23; 32]),
        sigma: overrides.sigma.unwrap_or([0x24; 32]),
        second_group_first_messages: overrides.second_group_first_messages.unwrap_or_default(),
    }
}

fn transfer_proof_vector_fields(vector: &TransferProofVector) -> Map<String, Value> {
    let mut fields = Map::new();
    fields.insert("draws".to_string(), byte_array(&vector.draws));
    fields.insert(
        "context".to_string(),
        Value::Object(proof_vector_context_fields(&vector.context)),
    );
    fields.insert(
        "identity_element".to_string(),
        byte_array(&vector.identity_element),
    );
    fields.insert("seller_pk1".to_string(), byte_array(&vector.seller_pk1));
    fields.insert("seller_pk2".to_string(), byte_array(&vector.seller_pk2));
    fields.insert("buyer_pk1".to_string(), byte_array(&vector.buyer_pk1));
    fields.insert("buyer_pk2".to_string(), byte_array(&vector.buyer_pk2));
    fields.insert(
        "old_envelope_c1".to_string(),
        byte_array(&vector.old_envelope_c1),
    );
    fields.insert(
        "old_envelope_c2".to_string(),
        byte_array(&vector.old_envelope_c2),
    );
    fields.insert(
        "old_envelope_d1".to_string(),
        byte_array(&vector.old_envelope_d1),
    );
    fields.insert(
        "old_envelope_d2".to_string(),
        byte_array(&vector.old_envelope_d2),
    );
    fields.insert(
        "new_envelope_c1".to_string(),
        byte_array(&vector.new_envelope_c1),
    );
    fields.insert(
        "new_envelope_c2".to_string(),
        byte_array(&vector.new_envelope_c2),
    );
    fields.insert(
        "new_envelope_d1".to_string(),
        byte_array(&vector.new_envelope_d1),
    );
    fields.insert(
        "new_envelope_d2".to_string(),
        byte_array(&vector.new_envelope_d2),
    );
    fields.insert("challenge".to_string(), byte_array(&vector.challenge));
    fields.insert("x".to_string(), byte_array(&vector.x));
    fields.insert("y".to_string(), byte_array(&vector.y));
    fields.insert("s".to_string(), byte_array(&vector.s));
    fields.insert("rho".to_string(), byte_array(&vector.rho));
    fields.insert("sigma".to_string(), byte_array(&vector.sigma));
    fields.insert(
        "second_group_first_messages".to_string(),
        byte_array(&vector.second_group_first_messages),
    );
    fields
}

#[derive(Default)]
pub struct TransferProofVectorCorruptions {
    pub draws: Option<Value>,
    pub context: Option<Value>,
    pub identity_element: Option<Value>,
    pub seller_pk1: Option<Value>,
    pub seller_pk2: Option<Value>,
    pub buyer_pk1: Option<Value>,
    pub buyer_pk2: Option<Value>,
    pub old_envelope_c1: Option<Value>,
    pub old_envelope_c2: Option<Value>,
    pub old_envelope_d1: Option<Value>,
    pub old_envelope_d2: Option<Value>,
    pub new_envelope_c1: Option<Value>,
    pub new_envelope_c2: Option<Value>,
    pub new_envelope_d1: Option<Value>,
    pub new_envelope_d2: Option<Value>,
    pub challenge: Option<Value>,
    pub x: Option<Value>,
    pub y: Option<Value>,
    pub s: Option<Value>,
    pub rho: Option<Value>,
    pub sigma: Option<Value>,
    pub second_group_first_messages: Option<Value>,
}

pub fn invalidate_transfer_proof_vector(corruptions: TransferProofVectorCorruptions) -> Value {
    let mut fields = transfer_proof_vector_fields(&build_transfer_proof_vector(Default::default()));
    if let Some(draws) = corruptions.draws {
        fields.insert("draws".to_string(), draws);
    }
    if let Some(context) = corruptions.context {
        fields.insert("context".to_string(), context);
    }
    if let Some(identity_element) = corruptions.identity_element {
        fields.insert("identity_element".to_string(), identity_element);
    }
    if let Some(seller_pk1) = corruptions.seller_pk1 {
        fields.insert("seller_pk1".to_string(), seller_pk1);
    }
    if let Some(seller_pk2) = corruptions.seller_pk2 {
        fields.insert("seller_pk2".to_string(), seller_pk2);
    }
    if let Some(buyer_pk1) = corruptions.buyer_pk1 {
        fields.insert("buyer_pk1".to_string(), buyer_pk1);
    }
    if let Some(buyer_pk2) = corruptions.buyer_pk2 {
        fields.insert("buyer_pk2".to_string(), buyer_pk2);
    }
    if let Some(old_envelope_c1) = corruptions.old_envelope_c1 {
        fields.insert("old_envelope_c1".to_string(), old_envelope_c1);
    }
    if let Some(old_envelope_c2) = corruptions.old_envelope_c2 {
        fields.insert("old_envelope_c2".to_string(), old_envelope_c2);
    }
    if let Some(old_envelope_d1) = corruptions.old_envelope_d1 {
        fields.insert("old_envelope_d1".to_string(), old_envelope_d1);
    }
    if let Some(old_envelope_d2) = corruptions.old_envelope_d2 {
        fields.insert("old_envelope_d2".to_string(), old_envelope_d2);
    }
    if let Some(new_envelope_c1) = corruptions.new_envelope_c1 {
        fields.insert("new_envelope_c1".to_string(), new_envelope_c1);
    }
    if let Some(new_envelope_c2) = corruptions.new_envelope_c2 {
        fields.insert("new_envelope_c2".to_string(), new_envelope_c2);
    }
    if let Some(new_envelope_d1) = corruptions.new_envelope_d1 {
        fields.insert("new_envelope_d1".to_string(), new_envelope_d1);
    }
    if let Some(new_envelope_d2) = corruptions.new_envelope_d2 {
        fields.insert("new_envelope_d2".to_string(), new_envelope_d2);
    }
    if let Some(challenge) = corruptions.challenge {
        fields.insert("challenge".to_string(), challenge);
    }
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
    if let Some(second_group_first_messages) = corruptions.second_group_first_messages {
        fields.insert(
            "second_group_first_messages".to_string(),
            second_group_first_messages,
        );
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct ProofVectorsParamsOverrides {
    pub mint_count: Option<VectorCount>,
    pub transfer_count: Option<VectorCount>,
}

pub fn build_proof_vectors_params(overrides: ProofVectorsParamsOverrides) -> ProofVectorsParams {
    ProofVectorsParams {
        mint_count: overrides
            .mint_count
            .unwrap_or_else(|| build_vector_count(Default::default())),
        transfer_count: overrides
            .transfer_count
            .unwrap_or_else(|| build_vector_count(Default::default())),
    }
}

#[derive(Default)]
pub struct ProofVectorsParamsCorruptions {
    pub mint_count: Option<Value>,
    pub transfer_count: Option<Value>,
}

pub fn invalidate_proof_vectors_params(corruptions: ProofVectorsParamsCorruptions) -> Value {
    let params = build_proof_vectors_params(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "mint_count".to_string(),
        Value::from(params.mint_count.get()),
    );
    fields.insert(
        "transfer_count".to_string(),
        Value::from(params.transfer_count.get()),
    );
    if let Some(mint_count) = corruptions.mint_count {
        fields.insert("mint_count".to_string(), mint_count);
    }
    if let Some(transfer_count) = corruptions.transfer_count {
        fields.insert("transfer_count".to_string(), transfer_count);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct ProofVectorsPayloadOverrides<'a> {
    pub identity: Option<AdmittedKemIdentity<'a>>,
}

pub fn build_proof_vectors_payload<'a, F: IChainForms>(
    mint: &'a DeliveryContext<F, MintPurpose>,
    transfer: &'a DeliveryContext<F, TransferPurpose>,
    overrides: ProofVectorsPayloadOverrides<'a>,
) -> ProofVectorsPayload<'a, F> {
    ProofVectorsPayload {
        mint,
        transfer,
        identity: overrides
            .identity
            .unwrap_or_else(|| build_admitted_kem_identity(Default::default())),
    }
}

#[derive(Default)]
pub struct ProofVectorsDepsOverrides;

pub fn build_proof_vectors_deps<
    'a,
    P: IPairingAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
>(
    pairing: &'a P,
    kem: &'a K,
    key_agreement: &'a A,
    delivery_proof: &'a D,
    random: &'a dyn IRandomSourceAdapter,
    _overrides: ProofVectorsDepsOverrides,
) -> ProofVectorsDeps<'a, P, K, A, D> {
    ProofVectorsDeps {
        pairing,
        kem,
        key_agreement,
        delivery_proof,
        random,
    }
}

#[derive(Default)]
pub struct ProofVectorsSuccessReturnOverrides {
    pub mint: Option<Vec<MintProofVector>>,
    pub transfer: Option<Vec<TransferProofVector>>,
}

pub fn build_proof_vectors_success_return(
    overrides: ProofVectorsSuccessReturnOverrides,
) -> ProofVectorsSuccessReturn {
    ProofVectorsSuccessReturn {
        mint: overrides
            .mint
            .unwrap_or_else(|| vec![build_mint_proof_vector(Default::default())]),
        transfer: overrides
            .transfer
            .unwrap_or_else(|| vec![build_transfer_proof_vector(Default::default())]),
    }
}

#[derive(Default)]
pub struct ProofVectorsSuccessReturnCorruptions {
    pub mint: Option<Value>,
    pub transfer: Option<Value>,
}

pub fn invalidate_proof_vectors_success_return(
    corruptions: ProofVectorsSuccessReturnCorruptions,
) -> Value {
    let success = build_proof_vectors_success_return(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "mint".to_string(),
        Value::Array(
            success
                .mint
                .iter()
                .map(|record| Value::Object(mint_proof_vector_fields(record)))
                .collect(),
        ),
    );
    fields.insert(
        "transfer".to_string(),
        Value::Array(
            success
                .transfer
                .iter()
                .map(|record| Value::Object(transfer_proof_vector_fields(record)))
                .collect(),
        ),
    );
    if let Some(mint) = corruptions.mint {
        fields.insert("mint".to_string(), mint);
    }
    if let Some(transfer) = corruptions.transfer {
        fields.insert("transfer".to_string(), transfer);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct MintProofVectorDescriptionConstructorParamsOverrides;

pub fn build_mint_proof_vector_description_constructor_params(
    _overrides: MintProofVectorDescriptionConstructorParamsOverrides,
) -> MintProofVectorDescriptionConstructorParams {
    MintProofVectorDescriptionConstructorParams
}

pub fn build_mint_proof_vector_description<F: IChainForms>(
    overrides: MintProofVectorDescriptionConstructorParamsOverrides,
) -> MintProofVectorDescription<F> {
    let Ok(description) = MintProofVectorDescription::<F>::try_new(
        build_mint_proof_vector_description_constructor_params(overrides),
    );
    description
}

#[derive(Default)]
pub struct TransferProofVectorDescriptionConstructorParamsOverrides;

pub fn build_transfer_proof_vector_description_constructor_params(
    _overrides: TransferProofVectorDescriptionConstructorParamsOverrides,
) -> TransferProofVectorDescriptionConstructorParams {
    TransferProofVectorDescriptionConstructorParams
}

pub fn build_transfer_proof_vector_description<F: IChainForms>(
    overrides: TransferProofVectorDescriptionConstructorParamsOverrides,
) -> TransferProofVectorDescription<F> {
    let Ok(description) = TransferProofVectorDescription::<F>::try_new(
        build_transfer_proof_vector_description_constructor_params(overrides),
    );
    description
}

pub fn mock_proof_vectors<
    'a,
    P: IPairingAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
>(
    _deps: &ProofVectorsDeps<'a, P, K, A, D>,
    _params: ProofVectorsParams,
    _payload: ProofVectorsPayload<'_, D::Forms>,
) -> ProofVectorsReturn {
    let _: ProofVectorsFn<'a, P, K, A, D> = mock_proof_vectors;
    Ok(build_proof_vectors_success_return(Default::default()))
}

#[derive(Default)]
pub struct ProofVectorsCaseParamsOverrides {
    pub pairing: Option<CreatePairingParams>,
    pub hash_to_scalar: Option<CreateHashToScalarParams>,
    pub encoding: Option<CreateEncodingParams>,
    pub chain_forms: Option<CreateChainFormsParams>,
    pub kem: Option<CreateKemParams>,
    pub key_agreement: Option<CreateKeyAgreementParams>,
    pub delivery_proof: Option<CreateDeliveryProofParams>,
    pub random_source: Option<CreateRandomSourceParams>,
}

pub fn build_proof_vectors_case_params(
    overrides: ProofVectorsCaseParamsOverrides,
) -> ProofVectorsCaseParams {
    ProofVectorsCaseParams {
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
        delivery_proof: overrides
            .delivery_proof
            .unwrap_or_else(|| build_create_delivery_proof_params(Default::default())),
        random_source: overrides
            .random_source
            .unwrap_or_else(|| build_create_random_source_params(Default::default())),
    }
}

#[derive(Default)]
pub struct ProofVectorsCaseDepsOverrides;

pub fn build_proof_vectors_case_deps<C: IProofVectorsCaseConsumer>(
    consumer: C,
    _overrides: ProofVectorsCaseDepsOverrides,
) -> ProofVectorsCaseDeps<C> {
    ProofVectorsCaseDeps { consumer }
}

#[derive(Default)]
pub struct ProofVectorsCasePayloadOverrides;

pub fn build_proof_vectors_case_payload<'a>(
    canonical: &'a [u8],
    root: &'a Root,
    _overrides: ProofVectorsCasePayloadOverrides,
) -> ProofVectorsCasePayload<'a> {
    ProofVectorsCasePayload { canonical, root }
}

#[derive(Default)]
pub struct ProofVectorsCaseSuccessReturnOverrides;

pub fn build_proof_vectors_case_success_return<O>(
    output: O,
    _overrides: ProofVectorsCaseSuccessReturnOverrides,
) -> ProofVectorsCaseSuccessReturn<O> {
    ProofVectorsCaseSuccessReturn { output }
}

#[derive(Default)]
pub struct ConsumeProofVectorsCaseParamsOverrides;

pub fn build_consume_proof_vectors_case_params(
    _overrides: ConsumeProofVectorsCaseParamsOverrides,
) -> ConsumeProofVectorsCaseParams {
    ConsumeProofVectorsCaseParams
}

#[derive(Default)]
pub struct ConsumeProofVectorsCasePayloadOverrides;

pub fn build_consume_proof_vectors_case_payload<'a, P, H, E, K, A, D>(
    pairing: &'a P,
    hash_to_scalar: &'a H,
    encoder: &'a E,
    decoder: &'a E,
    kem: &'a K,
    key_agreement: &'a A,
    delivery_proof: &'a D,
    random: &'a dyn IRandomSourceAdapter,
    identity: AdmittedKemIdentity<'a>,
    _overrides: ConsumeProofVectorsCasePayloadOverrides,
) -> ConsumeProofVectorsCasePayload<'a, P, H, E, K, A, D>
where
    P: IPairingArithmetic + IPairingReference,
    H: ?Sized + IHashToScalarAdapter<P::Scalar>,
    E: IEncoderAdapter + IDecoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
{
    ConsumeProofVectorsCasePayload {
        pairing,
        hash_to_scalar,
        encoder,
        decoder,
        kem,
        key_agreement,
        delivery_proof,
        random,
        identity,
    }
}

struct CaseAtDeliveryProof<
    'a,
    P: IPairingArithmetic,
    E,
    F: IChainForms,
    K,
    A,
    C: IProofVectorsCaseConsumer,
> {
    pairing: &'a P,
    hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    source: &'a dyn IRandomSourceAdapter,
    kem: &'a K,
    key_agreement: &'a A,
    identity: AdmittedKemIdentity<'a>,
    consumer: &'a C,
    marker: PhantomData<F>,
}

impl<'a, P, E, F, K, A, C> IDeliveryProofConsumer<P, F>
    for CaseAtDeliveryProof<'a, P, E, F, K, A, C>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
    F: IChainForms,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    C: IProofVectorsCaseConsumer,
{
    type Output = ProofVectorsCaseReturn<C::Output>;

    fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
        &self,
        _params: ConsumeDeliveryProofParams,
        payload: ConsumeDeliveryProofPayload<D>,
    ) -> Self::Output {
        Ok(ProofVectorsCaseSuccessReturn {
            output: self.consumer.consume_proof_vectors_case(
                build_consume_proof_vectors_case_params(Default::default()),
                build_consume_proof_vectors_case_payload(
                    self.pairing,
                    self.hashing,
                    self.encoder,
                    self.encoder,
                    self.kem,
                    self.key_agreement,
                    &payload.adapter,
                    self.source,
                    self.identity,
                    Default::default(),
                ),
            ),
        })
    }
}

struct CaseAtKeyAgreement<
    'a,
    P: IPairingArithmetic,
    E,
    F: IChainForms,
    K,
    C: IProofVectorsCaseConsumer,
> {
    pairing: &'a P,
    hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    source: &'a dyn IRandomSourceAdapter,
    kem: &'a K,
    delivery_proof: &'a CreateDeliveryProofParams,
    identity: AdmittedKemIdentity<'a>,
    consumer: &'a C,
    marker: PhantomData<F>,
}

impl<'a, P, E, F, K, C> IKeyAgreementConsumer<P> for CaseAtKeyAgreement<'a, P, E, F, K, C>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
    F: IChainForms,
    K: ICredentialKemAdapter<Pairing = P>,
    C: IProofVectorsCaseConsumer,
{
    type Output = ProofVectorsCaseReturn<C::Output>;

    fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
        &self,
        _params: ConsumeKeyAgreementParams,
        payload: ConsumeKeyAgreementPayload<A>,
    ) -> Self::Output {
        match create_delivery_proof(
            &build_create_delivery_proof_deps(
                self.pairing,
                self.hashing,
                self.encoder,
                self.source,
                CaseAtDeliveryProof {
                    pairing: self.pairing,
                    hashing: self.hashing,
                    encoder: self.encoder,
                    source: self.source,
                    kem: self.kem,
                    key_agreement: &payload.adapter,
                    identity: self.identity,
                    consumer: self.consumer,
                    marker: PhantomData::<F>,
                },
                Default::default(),
            ),
            CreateDeliveryProofParams {
                concrete: self.delivery_proof.concrete,
                algebra: self.delivery_proof.algebra,
                statement_version: self.delivery_proof.statement_version,
            },
            build_create_delivery_proof_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(ProofVectorsCaseErrorReturn::CreateDeliveryProof(error)),
        }
    }
}

struct CaseAtKem<'a, P: IPairingArithmetic, E, F: IChainForms, C: IProofVectorsCaseConsumer> {
    pairing: &'a P,
    hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    source: &'a dyn IRandomSourceAdapter,
    key_agreement: &'a CreateKeyAgreementParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    payload: &'a ProofVectorsCasePayload<'a>,
    consumer: &'a C,
    marker: PhantomData<F>,
}

impl<'a, P, E, F, C> IKemConsumer<P> for CaseAtKem<'a, P, E, F, C>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
    F: IChainForms,
    C: IProofVectorsCaseConsumer,
{
    type Output = ProofVectorsCaseReturn<C::Output>;

    fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
        &self,
        _params: ConsumeKemParams,
        payload: ConsumeKemPayload<K>,
    ) -> Self::Output {
        let identity = match payload.scope() {
            IdentityScope::Entitlement => AdmittedKemIdentity {
                setup_scope: SetupScope::Entitlement,
                identity: KemIdentity::Entitlement {
                    canonical: self.payload.canonical,
                },
            },
            IdentityScope::Asset => AdmittedKemIdentity {
                setup_scope: SetupScope::Asset {
                    root: self.payload.root,
                },
                identity: KemIdentity::Asset {
                    root: self.payload.root,
                },
            },
        };
        match create_key_agreement(
            &build_create_key_agreement_deps(
                self.pairing,
                self.hashing,
                self.encoder,
                self.source,
                CaseAtKeyAgreement {
                    pairing: self.pairing,
                    hashing: self.hashing,
                    encoder: self.encoder,
                    source: self.source,
                    kem: &payload.adapter,
                    delivery_proof: self.delivery_proof,
                    identity,
                    consumer: self.consumer,
                    marker: PhantomData::<F>,
                },
                Default::default(),
            ),
            *self.key_agreement,
            build_create_key_agreement_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(ProofVectorsCaseErrorReturn::CreateKeyAgreement(error)),
        }
    }
}

struct CaseAtChainForms<'a, P: IPairingArithmetic, E, C: IProofVectorsCaseConsumer> {
    pairing: &'a P,
    hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    source: &'a dyn IRandomSourceAdapter,
    kem: &'a CreateKemParams,
    key_agreement: &'a CreateKeyAgreementParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    payload: &'a ProofVectorsCasePayload<'a>,
    consumer: &'a C,
}

impl<'a, P, E, C> IChainFormsConsumer for CaseAtChainForms<'a, P, E, C>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
    C: IProofVectorsCaseConsumer,
{
    type Output = ProofVectorsCaseReturn<C::Output>;

    fn consume_chain_forms<F: IChainForms>(
        &self,
        _params: ConsumeChainFormsParams,
        _payload: ConsumeChainFormsPayload<F>,
    ) -> Self::Output {
        match create_kem(
            &build_create_kem_deps(
                self.pairing,
                self.hashing,
                CaseAtKem {
                    pairing: self.pairing,
                    hashing: self.hashing,
                    encoder: self.encoder,
                    source: self.source,
                    key_agreement: self.key_agreement,
                    delivery_proof: self.delivery_proof,
                    payload: self.payload,
                    consumer: self.consumer,
                    marker: PhantomData::<F>,
                },
                Default::default(),
            ),
            *self.kem,
            build_create_kem_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(ProofVectorsCaseErrorReturn::CreateKem(error)),
        }
    }
}

struct CaseAtEncoding<'a, P: IPairingArithmetic, C: IProofVectorsCaseConsumer> {
    pairing: &'a P,
    hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
    source: &'a dyn IRandomSourceAdapter,
    chain_forms: &'a CreateChainFormsParams,
    kem: &'a CreateKemParams,
    key_agreement: &'a CreateKeyAgreementParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    payload: &'a ProofVectorsCasePayload<'a>,
    consumer: &'a C,
}

impl<'a, P, C> IEncodingConsumer for CaseAtEncoding<'a, P, C>
where
    P: IPairingArithmetic + IPairingReference,
    C: IProofVectorsCaseConsumer,
{
    type Output = ProofVectorsCaseReturn<C::Output>;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
        match create_chain_forms(
            &build_create_chain_forms_deps(
                CaseAtChainForms {
                    pairing: self.pairing,
                    hashing: self.hashing,
                    encoder: &payload.adapter,
                    source: self.source,
                    kem: self.kem,
                    key_agreement: self.key_agreement,
                    delivery_proof: self.delivery_proof,
                    payload: self.payload,
                    consumer: self.consumer,
                },
                &payload.adapter,
                Default::default(),
            ),
            *self.chain_forms,
            build_create_chain_forms_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(ProofVectorsCaseErrorReturn::CreateChainForms(error)),
        }
    }
}

struct CaseAtPairing<'a, C: IProofVectorsCaseConsumer> {
    hash_to_scalar: &'a CreateHashToScalarParams,
    random_source: &'a CreateRandomSourceParams,
    encoding: &'a CreateEncodingParams,
    chain_forms: &'a CreateChainFormsParams,
    kem: &'a CreateKemParams,
    key_agreement: &'a CreateKeyAgreementParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    payload: &'a ProofVectorsCasePayload<'a>,
    consumer: &'a C,
}

impl<C: IProofVectorsCaseConsumer> IPairingConsumer for CaseAtPairing<'_, C> {
    type Output = ProofVectorsCaseReturn<C::Output>;

    fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let hashing = match create_hash_to_scalar::<P::Scalar>(
            &build_create_hash_to_scalar_deps(Default::default()),
            *self.hash_to_scalar,
            build_create_hash_to_scalar_payload(Default::default()),
        ) {
            Ok(hashing) => hashing,
            Err(error) => {
                return Err(ProofVectorsCaseErrorReturn::CreateHashToScalar(error));
            }
        };
        let source = match create_random_source(
            &CreateRandomSourceDeps,
            CreateRandomSourceParams {
                kind: self.random_source.kind,
            },
            CreateRandomSourcePayload,
        ) {
            Ok(source) => source,
            Err(error) => {
                return Err(ProofVectorsCaseErrorReturn::CreateRandomSource(error));
            }
        };
        match create_encoding(
            &build_create_encoding_deps(CaseAtEncoding {
                pairing: &payload.adapter,
                hashing: &*hashing.adapter,
                source: &*source.adapter,
                chain_forms: self.chain_forms,
                kem: self.kem,
                key_agreement: self.key_agreement,
                delivery_proof: self.delivery_proof,
                payload: self.payload,
                consumer: self.consumer,
            }),
            CreateEncodingParams {
                concrete: self.encoding.concrete,
                identifier: self.encoding.identifier,
            },
            build_create_encoding_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(ProofVectorsCaseErrorReturn::CreateEncoding(error)),
        }
    }
}

pub fn run_proof_vectors_case<C: IProofVectorsCaseConsumer>(
    deps: &ProofVectorsCaseDeps<C>,
    params: ProofVectorsCaseParams,
    payload: ProofVectorsCasePayload<'_>,
) -> ProofVectorsCaseReturn<C::Output> {
    let _: RunProofVectorsCaseFn<C> = run_proof_vectors_case;
    match create_pairing(
        &build_create_pairing_deps(
            CaseAtPairing {
                hash_to_scalar: &params.hash_to_scalar,
                random_source: &params.random_source,
                encoding: &params.encoding,
                chain_forms: &params.chain_forms,
                kem: &params.kem,
                key_agreement: &params.key_agreement,
                delivery_proof: &params.delivery_proof,
                payload: &payload,
                consumer: &deps.consumer,
            },
            Default::default(),
        ),
        params.pairing,
        build_create_pairing_payload(Default::default()),
    ) {
        Ok(success) => success.output,
        Err(error) => Err(ProofVectorsCaseErrorReturn::CreatePairing(error)),
    }
}
