#![allow(dead_code, clippy::too_many_arguments)]

use chain::{
    ConsumeChainFormsParams, ConsumeChainFormsPayload, CreateChainFormsParams, IChainForms,
    IChainFormsConsumer, build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, create_chain_forms,
};
use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingParams,
    CreateEncodingParamsOverrides, IDecoderAdapter, IEncoderAdapter, IEncodingConsumer,
    build_create_encoding_deps, build_create_encoding_params, build_create_encoding_payload,
    create_encoding,
};
use hash_to_scalar::{
    CreateHashToScalarParams, IHashToScalarAdapter, build_create_hash_to_scalar_deps,
    build_create_hash_to_scalar_params, build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParams, IPairingAdapter,
    IPairingArithmetic, IPairingConsumer, IPairingReference, build_create_pairing_deps,
    build_create_pairing_params, build_create_pairing_payload, create_pairing,
};
use proof::{
    DELIVERY_PURPOSE_MINT, DELIVERY_PURPOSE_TRANSFER, DELIVERY_STATEMENT_VERSION_ONE,
    DeliveryContext, DeliveryProofDeclaration, MintPurpose, TransferPurpose,
    build_delivery_proof_declaration,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParams, CreateRandomSourcePayload,
    IRandomSourceAdapter, build_create_random_source_params, create_random_source,
};
use serde_json::{Map, Value};

use super::super::super::factory::provides::{VectorCount, build_vector_count};
use super::interface::{
    ChallengeVector, ChallengeVectorDescription, ChallengeVectorDescriptionConstructorParams,
    ChallengeVectorsCaseDeps, ChallengeVectorsCaseErrorReturn, ChallengeVectorsCaseParams,
    ChallengeVectorsCasePayload, ChallengeVectorsCaseReturn, ChallengeVectorsCaseSuccessReturn,
    ChallengeVectorsDeps, ChallengeVectorsParams, ChallengeVectorsPayload, ChallengeVectorsReturn,
    ChallengeVectorsSuccessReturn, IChallengeVectorsCaseConsumer, RunChallengeVectorsCaseFn,
};
use crate::config::provides::{CanonicalDeliveryContext, build_canonical_delivery_context};

fn byte_array(bytes: &[u8]) -> Value {
    Value::Array(bytes.iter().map(|byte| Value::from(*byte)).collect())
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

fn challenge_vector_fields(vector: &ChallengeVector) -> Map<String, Value> {
    let mut fields = Map::new();
    fields.insert("purpose".to_string(), Value::from(vector.purpose));
    fields.insert("draws".to_string(), byte_array(&vector.draws));
    fields.insert("encoding".to_string(), byte_array(&vector.encoding));
    fields.insert("challenge".to_string(), byte_array(&vector.challenge));
    fields
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
pub struct ChallengeVectorsParamsCorruptions {
    pub count: Option<Value>,
    pub statement_version: Option<Value>,
}

pub fn invalidate_challenge_vectors_params(
    corruptions: ChallengeVectorsParamsCorruptions,
) -> Value {
    let params = build_challenge_vectors_params(Default::default());
    let mut fields = Map::new();
    fields.insert("count".to_string(), Value::from(params.count.get()));
    fields.insert(
        "statement_version".to_string(),
        Value::from(params.statement_version),
    );
    if let Some(count) = corruptions.count {
        fields.insert("count".to_string(), count);
    }
    if let Some(statement_version) = corruptions.statement_version {
        fields.insert("statement_version".to_string(), statement_version);
    }
    Value::Object(fields)
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
pub struct ChallengeVectorsSuccessReturnCorruptions {
    pub records: Option<Value>,
}

pub fn invalidate_challenge_vectors_success_return(
    corruptions: ChallengeVectorsSuccessReturnCorruptions,
) -> Value {
    let success = build_challenge_vectors_success_return(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "records".to_string(),
        Value::Array(
            success
                .records
                .iter()
                .map(|record| Value::Object(challenge_vector_fields(record)))
                .collect(),
        ),
    );
    if let Some(records) = corruptions.records {
        fields.insert("records".to_string(), records);
    }
    Value::Object(fields)
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
pub struct ChallengeVectorsPayloadOverrides;

pub fn build_challenge_vectors_payload<'a, F: IChainForms>(
    delivery_proof: &'a DeliveryProofDeclaration,
    mint: &'a DeliveryContext<F, MintPurpose>,
    transfer: &'a DeliveryContext<F, TransferPurpose>,
    _overrides: ChallengeVectorsPayloadOverrides,
) -> ChallengeVectorsPayload<'a, F> {
    ChallengeVectorsPayload {
        delivery_proof,
        mint,
        transfer,
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

pub fn mock_challenge_vectors<'a, P: IPairingAdapter, E: IEncoderAdapter, F: IChainForms>(
    _deps: &ChallengeVectorsDeps<'a, P, E>,
    _params: ChallengeVectorsParams,
    _payload: ChallengeVectorsPayload<'_, F>,
) -> ChallengeVectorsReturn {
    Ok(build_challenge_vectors_success_return(Default::default()))
}

#[derive(Default)]
pub struct ChallengeVectorsCaseParamsOverrides {
    pub pairing: Option<CreatePairingParams>,
    pub hash_to_scalar: Option<CreateHashToScalarParams>,
    pub encoding: Option<CreateEncodingParams>,
    pub chain_forms: Option<CreateChainFormsParams>,
    pub random_source: Option<CreateRandomSourceParams>,
}

pub fn build_challenge_vectors_case_params(
    overrides: ChallengeVectorsCaseParamsOverrides,
) -> ChallengeVectorsCaseParams {
    ChallengeVectorsCaseParams {
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
        random_source: overrides
            .random_source
            .unwrap_or_else(|| build_create_random_source_params(Default::default())),
    }
}

#[derive(Default)]
pub struct ChallengeVectorsCaseDepsOverrides;

pub fn build_challenge_vectors_case_deps<C: IChallengeVectorsCaseConsumer>(
    consumer: C,
    _overrides: ChallengeVectorsCaseDepsOverrides,
) -> ChallengeVectorsCaseDeps<C> {
    ChallengeVectorsCaseDeps { consumer }
}

#[derive(Default)]
pub struct ChallengeVectorsCasePayloadOverrides {
    pub delivery_proof: Option<DeliveryProofDeclaration>,
    pub mint: Option<CanonicalDeliveryContext>,
    pub mint_purpose: Option<MintPurpose>,
    pub transfer: Option<CanonicalDeliveryContext>,
    pub transfer_purpose: Option<TransferPurpose>,
}

pub fn build_challenge_vectors_case_payload(
    overrides: ChallengeVectorsCasePayloadOverrides,
) -> ChallengeVectorsCasePayload {
    ChallengeVectorsCasePayload {
        delivery_proof: overrides
            .delivery_proof
            .unwrap_or_else(|| build_delivery_proof_declaration(Default::default())),
        mint: overrides
            .mint
            .unwrap_or_else(|| build_canonical_delivery_context(Default::default())),
        mint_purpose: overrides.mint_purpose.unwrap_or(DELIVERY_PURPOSE_MINT),
        transfer: overrides
            .transfer
            .unwrap_or_else(|| build_canonical_delivery_context(Default::default())),
        transfer_purpose: overrides
            .transfer_purpose
            .unwrap_or(DELIVERY_PURPOSE_TRANSFER),
    }
}

#[derive(Default)]
pub struct ChallengeVectorsCaseSuccessReturnOverrides;

pub fn build_challenge_vectors_case_success_return<O>(
    output: O,
    _overrides: ChallengeVectorsCaseSuccessReturnOverrides,
) -> ChallengeVectorsCaseSuccessReturn<O> {
    ChallengeVectorsCaseSuccessReturn { output }
}

struct CaseAtChainForms<'a, P: IPairingArithmetic, E, C: IChallengeVectorsCaseConsumer> {
    pairing: &'a P,
    hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    source: &'a dyn IRandomSourceAdapter,
    case_payload: &'a ChallengeVectorsCasePayload,
    consumer: &'a C,
}

impl<P, E, C> IChainFormsConsumer for CaseAtChainForms<'_, P, E, C>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
    C: IChallengeVectorsCaseConsumer,
{
    type Output = ChallengeVectorsCaseReturn<C::Output>;

    fn consume_chain_forms<F: IChainForms>(
        &self,
        _params: ConsumeChainFormsParams,
        payload: ConsumeChainFormsPayload<F>,
    ) -> Self::Output {
        Ok(ChallengeVectorsCaseSuccessReturn {
            output: self.consumer.consume_challenge_vectors_case(
                self.pairing,
                self.hashing,
                self.encoder,
                self.source,
                &payload.forms,
                self.case_payload,
            ),
        })
    }
}

struct CaseAtEncoding<'a, P: IPairingArithmetic, C: IChallengeVectorsCaseConsumer> {
    pairing: &'a P,
    hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
    source: &'a dyn IRandomSourceAdapter,
    chain_forms: &'a CreateChainFormsParams,
    payload: &'a ChallengeVectorsCasePayload,
    consumer: &'a C,
}

impl<P, C> IEncodingConsumer for CaseAtEncoding<'_, P, C>
where
    P: IPairingArithmetic + IPairingReference,
    C: IChallengeVectorsCaseConsumer,
{
    type Output = ChallengeVectorsCaseReturn<C::Output>;

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
                    case_payload: self.payload,
                    consumer: self.consumer,
                },
                &payload.adapter,
                Default::default(),
            ),
            *self.chain_forms,
            build_create_chain_forms_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(ChallengeVectorsCaseErrorReturn::CreateChainForms(error)),
        }
    }
}

struct CaseAtPairing<'a, C: IChallengeVectorsCaseConsumer> {
    hash_to_scalar: &'a CreateHashToScalarParams,
    encoding: &'a CreateEncodingParams,
    chain_forms: &'a CreateChainFormsParams,
    random_source: &'a CreateRandomSourceParams,
    payload: &'a ChallengeVectorsCasePayload,
    consumer: &'a C,
}

impl<C: IChallengeVectorsCaseConsumer> IPairingConsumer for CaseAtPairing<'_, C> {
    type Output = ChallengeVectorsCaseReturn<C::Output>;

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
                return Err(ChallengeVectorsCaseErrorReturn::CreateHashToScalar(error));
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
                return Err(ChallengeVectorsCaseErrorReturn::CreateRandomSource(error));
            }
        };
        match create_encoding(
            &build_create_encoding_deps(CaseAtEncoding {
                pairing: &payload.adapter,
                hashing: &*hashing.adapter,
                source: &*source.adapter,
                chain_forms: self.chain_forms,
                payload: self.payload,
                consumer: self.consumer,
            }),
            build_create_encoding_params(CreateEncodingParamsOverrides {
                concrete: Some(self.encoding.concrete),
                identifier: Some(self.encoding.identifier),
            }),
            build_create_encoding_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(ChallengeVectorsCaseErrorReturn::CreateEncoding(error)),
        }
    }
}

pub fn run_challenge_vectors_case<C: IChallengeVectorsCaseConsumer>(
    deps: &ChallengeVectorsCaseDeps<C>,
    params: ChallengeVectorsCaseParams,
    payload: ChallengeVectorsCasePayload,
) -> ChallengeVectorsCaseReturn<C::Output> {
    let _: RunChallengeVectorsCaseFn<C> = run_challenge_vectors_case;
    match create_pairing(
        &build_create_pairing_deps(
            CaseAtPairing {
                hash_to_scalar: &params.hash_to_scalar,
                encoding: &params.encoding,
                chain_forms: &params.chain_forms,
                random_source: &params.random_source,
                payload: &payload,
                consumer: &deps.consumer,
            },
            Default::default(),
        ),
        params.pairing,
        build_create_pairing_payload(Default::default()),
    ) {
        Ok(success) => success.output,
        Err(error) => Err(ChallengeVectorsCaseErrorReturn::CreatePairing(error)),
    }
}
