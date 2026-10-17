#![allow(dead_code, clippy::too_many_arguments)]

use domain::{
    AssetIdentityHash, ParameterSetIdentifier, SuiteIdentifier, build_asset_identity_hash,
    build_parameter_set_identifier, build_suite_identifier,
};
use encoding::{CanonicalFieldValue, IEncoderAdapter};
use hash_to_scalar::IHashToScalarAdapter;
use pairing::IPairingAdapter;
use proof::{
    DELIVERY_PURPOSE_MINT, DELIVERY_PURPOSE_TRANSFER, DELIVERY_STATEMENT_VERSION_ONE,
    DeliveryProofDeclaration, MintPurpose, TransferPurpose,
};
use random::IRandomSourceAdapter;
use serde_json::{Map, Value};

use super::super::group_vectors::provides::{VectorCount, build_vector_count};
use super::interface::{
    CanonicalDeliveryContext, ChallengeVector, ChallengeVectorDescription,
    ChallengeVectorDescriptionConstructorParams, ChallengeVectorsDeps, ChallengeVectorsFn,
    ChallengeVectorsParams, ChallengeVectorsPayload, ChallengeVectorsReturn,
    ChallengeVectorsSuccessReturn,
};

#[derive(Default)]
pub struct CanonicalDeliveryContextOverrides {
    pub suite_identifier: Option<SuiteIdentifier>,
    pub chain: Option<CanonicalFieldValue>,
    pub entitlement_contract: Option<CanonicalFieldValue>,
    pub asset_identity_hash: Option<AssetIdentityHash>,
    pub parameter_set_identifier: Option<ParameterSetIdentifier>,
    pub source_entitlement: Option<CanonicalFieldValue>,
    pub target_entitlement: Option<CanonicalFieldValue>,
    pub old_interval: Option<CanonicalFieldValue>,
    pub new_interval: Option<CanonicalFieldValue>,
    pub seller: Option<CanonicalFieldValue>,
    pub buyer: Option<CanonicalFieldValue>,
    pub expiry: Option<u64>,
}

pub fn build_canonical_delivery_context(
    overrides: CanonicalDeliveryContextOverrides,
) -> CanonicalDeliveryContext {
    CanonicalDeliveryContext {
        suite_identifier: overrides
            .suite_identifier
            .unwrap_or_else(|| build_suite_identifier(Default::default())),
        chain: overrides
            .chain
            .unwrap_or(CanonicalFieldValue::Unsigned256([0x21; 32])),
        entitlement_contract: overrides
            .entitlement_contract
            .unwrap_or(CanonicalFieldValue::FixedBytes20([0x31; 20])),
        asset_identity_hash: overrides
            .asset_identity_hash
            .unwrap_or_else(|| build_asset_identity_hash(Default::default())),
        parameter_set_identifier: overrides
            .parameter_set_identifier
            .unwrap_or_else(|| build_parameter_set_identifier(Default::default())),
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
        seller: overrides
            .seller
            .unwrap_or(CanonicalFieldValue::FixedBytes20([0x61; 20])),
        buyer: overrides
            .buyer
            .unwrap_or(CanonicalFieldValue::FixedBytes20([0x62; 20])),
        expiry: overrides.expiry.unwrap_or(1_800_000_000),
    }
}

#[derive(Default)]
pub struct ChallengeVectorOverrides {
    pub purpose: Option<u16>,
    pub draws: Option<Vec<u8>>,
    pub encoding: Option<Vec<u8>>,
    pub challenge: Option<[u8; 32]>,
}

pub fn build_challenge_vector(overrides: ChallengeVectorOverrides) -> ChallengeVector {
    ChallengeVector {
        purpose: overrides.purpose.unwrap_or(1),
        draws: overrides.draws.unwrap_or_else(|| vec![0x01; 32]),
        encoding: overrides.encoding.unwrap_or_else(|| vec![0x02; 64]),
        challenge: overrides.challenge.unwrap_or([0x03; 32]),
    }
}

#[derive(Default)]
pub struct ChallengeVectorsParamsOverrides {
    pub count: Option<VectorCount>,
    pub statement_version: Option<u16>,
}

pub fn build_challenge_vectors_params(
    overrides: ChallengeVectorsParamsOverrides,
) -> ChallengeVectorsParams {
    ChallengeVectorsParams {
        count: overrides
            .count
            .unwrap_or_else(|| build_vector_count(Default::default())),
        statement_version: overrides
            .statement_version
            .unwrap_or(DELIVERY_STATEMENT_VERSION_ONE),
    }
}

#[derive(Default)]
pub struct ChallengeVectorsSuccessReturnOverrides {
    pub records: Option<Vec<ChallengeVector>>,
}

pub fn build_challenge_vectors_success_return(
    overrides: ChallengeVectorsSuccessReturnOverrides,
) -> ChallengeVectorsSuccessReturn {
    ChallengeVectorsSuccessReturn {
        records: overrides
            .records
            .unwrap_or_else(|| vec![build_challenge_vector(Default::default())]),
    }
}

#[derive(Default)]
pub struct ChallengeVectorsDepsOverrides;

pub fn build_challenge_vectors_deps<'a, P: IPairingAdapter, E: IEncoderAdapter>(
    pairing: &'a P,
    encoder: &'a E,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    random: &'a dyn IRandomSourceAdapter,
    _overrides: ChallengeVectorsDepsOverrides,
) -> ChallengeVectorsDeps<'a, P, E> {
    ChallengeVectorsDeps {
        pairing,
        encoder,
        hash_to_scalar,
        random,
    }
}

#[derive(Default)]
pub struct ChallengeVectorsPayloadOverrides {
    pub mint_purpose: Option<MintPurpose>,
    pub transfer_purpose: Option<TransferPurpose>,
}

pub fn build_challenge_vectors_payload<'a>(
    delivery_proof: &'a DeliveryProofDeclaration,
    mint: &'a CanonicalDeliveryContext,
    transfer: &'a CanonicalDeliveryContext,
    overrides: ChallengeVectorsPayloadOverrides,
) -> ChallengeVectorsPayload<'a> {
    ChallengeVectorsPayload {
        delivery_proof,
        mint,
        mint_purpose: overrides.mint_purpose.unwrap_or(DELIVERY_PURPOSE_MINT),
        transfer,
        transfer_purpose: overrides
            .transfer_purpose
            .unwrap_or(DELIVERY_PURPOSE_TRANSFER),
    }
}

#[derive(Default)]
pub struct ChallengeVectorDescriptionConstructorParamsOverrides;

pub fn build_challenge_vector_description_constructor_params(
    _overrides: ChallengeVectorDescriptionConstructorParamsOverrides,
) -> ChallengeVectorDescriptionConstructorParams {
    ChallengeVectorDescriptionConstructorParams
}

pub fn build_challenge_vector_description(
    overrides: ChallengeVectorDescriptionConstructorParamsOverrides,
) -> ChallengeVectorDescription {
    let Ok(description) = ChallengeVectorDescription::try_new(
        build_challenge_vector_description_constructor_params(overrides),
    );
    description
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

fn canonical_delivery_context_fields(context: &CanonicalDeliveryContext) -> Map<String, Value> {
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
    fields.insert("chain".to_string(), canonical_field_value(&context.chain));
    fields.insert(
        "entitlement_contract".to_string(),
        canonical_field_value(&context.entitlement_contract),
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
    fields.insert("seller".to_string(), canonical_field_value(&context.seller));
    fields.insert("buyer".to_string(), canonical_field_value(&context.buyer));
    fields.insert("expiry".to_string(), Value::from(context.expiry));
    fields
}

fn challenge_vector_fields(vector: &ChallengeVector) -> Map<String, Value> {
    let mut fields = Map::new();
    fields.insert("purpose".to_string(), Value::from(vector.purpose));
    fields.insert("draws".to_string(), byte_array(&vector.draws));
    fields.insert("encoding".to_string(), byte_array(&vector.encoding));
    fields.insert("challenge".to_string(), byte_array(&vector.challenge));
    fields
}

fn challenge_vectors_params_fields(params: &ChallengeVectorsParams) -> Map<String, Value> {
    let mut fields = Map::new();
    fields.insert("count".to_string(), Value::from(params.count.get()));
    fields.insert(
        "statement_version".to_string(),
        Value::from(params.statement_version),
    );
    fields
}

fn challenge_vectors_success_return_fields(
    value: &ChallengeVectorsSuccessReturn,
) -> Map<String, Value> {
    let mut fields = Map::new();
    fields.insert(
        "records".to_string(),
        Value::Array(
            value
                .records
                .iter()
                .map(|record| Value::Object(challenge_vector_fields(record)))
                .collect(),
        ),
    );
    fields
}

#[derive(Default)]
pub struct CanonicalDeliveryContextCorruptions {
    pub suite_identifier: Option<Value>,
    pub chain: Option<Value>,
    pub entitlement_contract: Option<Value>,
    pub asset_identity_hash: Option<Value>,
    pub parameter_set_identifier: Option<Value>,
    pub source_entitlement: Option<Value>,
    pub target_entitlement: Option<Value>,
    pub old_interval: Option<Value>,
    pub new_interval: Option<Value>,
    pub seller: Option<Value>,
    pub buyer: Option<Value>,
    pub expiry: Option<Value>,
}

pub fn invalidate_canonical_delivery_context(
    corruptions: CanonicalDeliveryContextCorruptions,
) -> Value {
    let mut fields =
        canonical_delivery_context_fields(&build_canonical_delivery_context(Default::default()));
    if let Some(suite_identifier) = corruptions.suite_identifier {
        fields.insert("suite_identifier".to_string(), suite_identifier);
    }
    if let Some(chain) = corruptions.chain {
        fields.insert("chain".to_string(), chain);
    }
    if let Some(entitlement_contract) = corruptions.entitlement_contract {
        fields.insert("entitlement_contract".to_string(), entitlement_contract);
    }
    if let Some(asset_identity_hash) = corruptions.asset_identity_hash {
        fields.insert("asset_identity_hash".to_string(), asset_identity_hash);
    }
    if let Some(parameter_set_identifier) = corruptions.parameter_set_identifier {
        fields.insert(
            "parameter_set_identifier".to_string(),
            parameter_set_identifier,
        );
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
pub struct ChallengeVectorCorruptions {
    pub purpose: Option<Value>,
    pub draws: Option<Value>,
    pub encoding: Option<Value>,
    pub challenge: Option<Value>,
}

pub fn invalidate_challenge_vector(corruptions: ChallengeVectorCorruptions) -> Value {
    let mut fields = challenge_vector_fields(&build_challenge_vector(Default::default()));
    if let Some(purpose) = corruptions.purpose {
        fields.insert("purpose".to_string(), purpose);
    }
    if let Some(draws) = corruptions.draws {
        fields.insert("draws".to_string(), draws);
    }
    if let Some(encoding) = corruptions.encoding {
        fields.insert("encoding".to_string(), encoding);
    }
    if let Some(challenge) = corruptions.challenge {
        fields.insert("challenge".to_string(), challenge);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct ChallengeVectorsParamsCorruptions {
    pub count: Option<Value>,
    pub statement_version: Option<Value>,
}

pub fn invalidate_challenge_vectors_params(
    corruptions: ChallengeVectorsParamsCorruptions,
) -> Value {
    let mut fields =
        challenge_vectors_params_fields(&build_challenge_vectors_params(Default::default()));
    if let Some(count) = corruptions.count {
        fields.insert("count".to_string(), count);
    }
    if let Some(statement_version) = corruptions.statement_version {
        fields.insert("statement_version".to_string(), statement_version);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct ChallengeVectorsSuccessReturnCorruptions {
    pub records: Option<Value>,
}

pub fn invalidate_challenge_vectors_success_return(
    corruptions: ChallengeVectorsSuccessReturnCorruptions,
) -> Value {
    let mut fields = challenge_vectors_success_return_fields(
        &build_challenge_vectors_success_return(Default::default()),
    );
    if let Some(records) = corruptions.records {
        fields.insert("records".to_string(), records);
    }
    Value::Object(fields)
}

pub fn mock_challenge_vectors<'a, P: IPairingAdapter, E: IEncoderAdapter>(
    _deps: &ChallengeVectorsDeps<'a, P, E>,
    _params: ChallengeVectorsParams,
    _payload: ChallengeVectorsPayload<'_>,
) -> ChallengeVectorsReturn {
    let _: ChallengeVectorsFn<'a, P, E> = mock_challenge_vectors::<P, E>;
    Ok(build_challenge_vectors_success_return(Default::default()))
}
