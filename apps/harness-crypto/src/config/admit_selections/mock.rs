#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_arguments
)]

use chain::{ChainFormsConcrete, ChainFormsIdentifier};
use encoding::{EncodingConcrete, EncodingIdentifier};
use envelope::{EnvelopeAlgebra, KeyAgreementConcrete, KeyAgreementIdentifier};
use hash_to_scalar::{HashToScalarConcrete, HashToScalarIdentifier};
use kem::{IdentityScope, KemConcrete, KemIdentifier};
use pairing::{PairingConcrete, PrecompileEncoding, TargetGroupEncodingIdentifier};
use proof::{DELIVERY_STATEMENT_VERSION_ONE, DeliveryProofConcrete};
use random::RandomSourceKind;
use serde_json::Value;

use super::super::layers::provides::{
    CurveDocument, DeliveryProofDocument, KemDocument, KeyAgreementDocument, RandomDocument,
    SelectionDocument,
};
use super::interface::{
    AdmitSelectionsDeps, AdmitSelectionsFn, AdmitSelectionsParams, AdmitSelectionsPayload,
    AdmitSelectionsReturn, AdmitSelectionsSuccessReturn, ChainFormsSelection, CurveSelection,
    DeliveryProofSelection, EncodingSelection, HarnessSelections, HashToScalarSelection,
    KemSelection, KeyAgreementSelection,
};

#[derive(Default)]
pub struct EncodingSelectionOverrides {
    pub concrete: Option<EncodingConcrete>,
    pub identifier: Option<EncodingIdentifier>,
}

pub fn build_encoding_selection(overrides: EncodingSelectionOverrides) -> EncodingSelection {
    EncodingSelection {
        concrete: overrides.concrete.unwrap_or(EncodingConcrete::Abi),
        identifier: overrides
            .identifier
            .unwrap_or(EncodingIdentifier::EthereumAbiV1),
    }
}

#[derive(Default)]
pub struct EncodingSelectionCorruptions {
    pub concrete: Option<Value>,
    pub identifier: Option<Value>,
}

pub fn invalidate_encoding_selection(corruptions: EncodingSelectionCorruptions) -> Value {
    let mut value = serde_json::to_value(build_encoding_selection(Default::default()))
        .expect("a built selection renders as a value");
    if let Some(concrete) = corruptions.concrete {
        value["concrete"] = concrete;
    }
    if let Some(identifier) = corruptions.identifier {
        value["identifier"] = identifier;
    }
    value
}

#[derive(Default)]
pub struct HashToScalarSelectionOverrides {
    pub concrete: Option<HashToScalarConcrete>,
    pub identifier: Option<HashToScalarIdentifier>,
}

pub fn build_hash_to_scalar_selection(
    overrides: HashToScalarSelectionOverrides,
) -> HashToScalarSelection {
    HashToScalarSelection {
        concrete: overrides
            .concrete
            .unwrap_or(HashToScalarConcrete::Keccak256),
        identifier: overrides
            .identifier
            .unwrap_or(HashToScalarIdentifier::Keccak256V1),
    }
}

#[derive(Default)]
pub struct HashToScalarSelectionCorruptions {
    pub concrete: Option<Value>,
    pub identifier: Option<Value>,
}

pub fn invalidate_hash_to_scalar_selection(corruptions: HashToScalarSelectionCorruptions) -> Value {
    let mut value = serde_json::to_value(build_hash_to_scalar_selection(Default::default()))
        .expect("a built selection renders as a value");
    if let Some(concrete) = corruptions.concrete {
        value["concrete"] = concrete;
    }
    if let Some(identifier) = corruptions.identifier {
        value["identifier"] = identifier;
    }
    value
}

#[derive(Default)]
pub struct ChainFormsSelectionOverrides {
    pub concrete: Option<ChainFormsConcrete>,
    pub identifier: Option<ChainFormsIdentifier>,
}

pub fn build_chain_forms_selection(overrides: ChainFormsSelectionOverrides) -> ChainFormsSelection {
    ChainFormsSelection {
        concrete: overrides.concrete.unwrap_or(ChainFormsConcrete::Evm),
        identifier: overrides.identifier.unwrap_or(ChainFormsIdentifier::EvmV1),
    }
}

#[derive(Default)]
pub struct ChainFormsSelectionCorruptions {
    pub concrete: Option<Value>,
    pub identifier: Option<Value>,
}

pub fn invalidate_chain_forms_selection(corruptions: ChainFormsSelectionCorruptions) -> Value {
    let mut value = serde_json::to_value(build_chain_forms_selection(Default::default()))
        .expect("a built selection renders as a value");
    if let Some(concrete) = corruptions.concrete {
        value["concrete"] = concrete;
    }
    if let Some(identifier) = corruptions.identifier {
        value["identifier"] = identifier;
    }
    value
}

#[derive(Default)]
pub struct KemSelectionOverrides {
    pub concrete: Option<KemConcrete>,
    pub identifier: Option<KemIdentifier>,
    pub scope: Option<IdentityScope>,
}

pub fn build_kem_selection(overrides: KemSelectionOverrides) -> KemSelection {
    KemSelection {
        concrete: overrides.concrete.unwrap_or(KemConcrete::Bb1DepthOne),
        identifier: overrides.identifier.unwrap_or(KemIdentifier::Bb1DepthOneV1),
        scope: overrides.scope.unwrap_or(IdentityScope::Entitlement),
    }
}

#[derive(Default)]
pub struct KemSelectionCorruptions {
    pub concrete: Option<Value>,
    pub identifier: Option<Value>,
    pub scope: Option<Value>,
}

pub fn invalidate_kem_selection(corruptions: KemSelectionCorruptions) -> Value {
    let mut value = serde_json::to_value(build_kem_selection(Default::default()))
        .expect("a built selection renders as a value");
    if let Some(concrete) = corruptions.concrete {
        value["concrete"] = concrete;
    }
    if let Some(identifier) = corruptions.identifier {
        value["identifier"] = identifier;
    }
    if let Some(scope) = corruptions.scope {
        value["scope"] = scope;
    }
    value
}

#[derive(Default)]
pub struct KeyAgreementSelectionOverrides {
    pub concrete: Option<KeyAgreementConcrete>,
    pub identifier: Option<KeyAgreementIdentifier>,
    pub algebra: Option<EnvelopeAlgebra>,
}

pub fn build_key_agreement_selection(
    overrides: KeyAgreementSelectionOverrides,
) -> KeyAgreementSelection {
    KeyAgreementSelection {
        concrete: overrides
            .concrete
            .unwrap_or(KeyAgreementConcrete::PairingElGamal),
        identifier: overrides
            .identifier
            .unwrap_or(KeyAgreementIdentifier::PairingElGamalV1),
        algebra: overrides.algebra.unwrap_or(EnvelopeAlgebra::PairingElGamal),
    }
}

#[derive(Default)]
pub struct KeyAgreementSelectionCorruptions {
    pub concrete: Option<Value>,
    pub identifier: Option<Value>,
    pub algebra: Option<Value>,
}

pub fn invalidate_key_agreement_selection(corruptions: KeyAgreementSelectionCorruptions) -> Value {
    let mut value = serde_json::to_value(build_key_agreement_selection(Default::default()))
        .expect("a built selection renders as a value");
    if let Some(concrete) = corruptions.concrete {
        value["concrete"] = concrete;
    }
    if let Some(identifier) = corruptions.identifier {
        value["identifier"] = identifier;
    }
    if let Some(algebra) = corruptions.algebra {
        value["algebra"] = algebra;
    }
    value
}

#[derive(Default)]
pub struct DeliveryProofSelectionOverrides {
    pub concrete: Option<DeliveryProofConcrete>,
    pub statement_version: Option<u16>,
}

pub fn build_delivery_proof_selection(
    overrides: DeliveryProofSelectionOverrides,
) -> DeliveryProofSelection {
    DeliveryProofSelection {
        concrete: overrides
            .concrete
            .unwrap_or(DeliveryProofConcrete::SchnorrFs),
        statement_version: overrides
            .statement_version
            .unwrap_or(DELIVERY_STATEMENT_VERSION_ONE),
    }
}

#[derive(Default)]
pub struct DeliveryProofSelectionCorruptions {
    pub concrete: Option<Value>,
    pub statement_version: Option<Value>,
}

pub fn invalidate_delivery_proof_selection(
    corruptions: DeliveryProofSelectionCorruptions,
) -> Value {
    let mut value = serde_json::to_value(build_delivery_proof_selection(Default::default()))
        .expect("a built selection renders as a value");
    if let Some(concrete) = corruptions.concrete {
        value["concrete"] = concrete;
    }
    if let Some(statement_version) = corruptions.statement_version {
        value["statement_version"] = statement_version;
    }
    value
}

#[derive(Default)]
pub struct CurveSelectionOverrides {
    pub pairing: Option<PairingConcrete>,
    pub target_group_encoding: Option<TargetGroupEncodingIdentifier>,
}

pub fn build_curve_selection(overrides: CurveSelectionOverrides) -> CurveSelection {
    CurveSelection {
        pairing: overrides.pairing.unwrap_or(PairingConcrete::Bn254Arkworks),
        target_group_encoding: overrides
            .target_group_encoding
            .unwrap_or(TargetGroupEncodingIdentifier::Bn254V1),
    }
}

#[derive(Default)]
pub struct CurveSelectionCorruptions {
    pub pairing: Option<Value>,
    pub target_group_encoding: Option<Value>,
}

pub fn invalidate_curve_selection(corruptions: CurveSelectionCorruptions) -> Value {
    let mut value = serde_json::to_value(build_curve_selection(Default::default()))
        .expect("a built selection renders as a value");
    if let Some(pairing) = corruptions.pairing {
        value["pairing"] = pairing;
    }
    if let Some(target_group_encoding) = corruptions.target_group_encoding {
        value["target_group_encoding"] = target_group_encoding;
    }
    value
}

#[derive(Default)]
pub struct HarnessSelectionsOverrides {
    pub random: Option<RandomSourceKind>,
    pub encoding: Option<EncodingSelection>,
    pub hash_to_scalar: Option<HashToScalarSelection>,
    pub chain_forms: Option<ChainFormsSelection>,
    pub kem: Option<KemSelection>,
    pub key_agreement: Option<KeyAgreementSelection>,
    pub delivery_proof: Option<DeliveryProofSelection>,
    pub curves: Option<Vec<CurveSelection>>,
    pub benchmark_concretes: Option<Vec<CurveSelection>>,
    pub precompile_encodings: Option<Vec<PrecompileEncoding>>,
}

pub fn build_harness_selections(overrides: HarnessSelectionsOverrides) -> HarnessSelections {
    HarnessSelections {
        random: overrides
            .random
            .unwrap_or(RandomSourceKind::OperatingSystem),
        encoding: overrides
            .encoding
            .unwrap_or_else(|| build_encoding_selection(Default::default())),
        hash_to_scalar: overrides
            .hash_to_scalar
            .unwrap_or_else(|| build_hash_to_scalar_selection(Default::default())),
        chain_forms: overrides
            .chain_forms
            .unwrap_or_else(|| build_chain_forms_selection(Default::default())),
        kem: overrides
            .kem
            .unwrap_or_else(|| build_kem_selection(Default::default())),
        key_agreement: overrides
            .key_agreement
            .unwrap_or_else(|| build_key_agreement_selection(Default::default())),
        delivery_proof: overrides
            .delivery_proof
            .unwrap_or_else(|| build_delivery_proof_selection(Default::default())),
        curves: overrides
            .curves
            .unwrap_or_else(|| vec![build_curve_selection(Default::default())]),
        benchmark_concretes: overrides
            .benchmark_concretes
            .unwrap_or_else(|| vec![build_curve_selection(Default::default())]),
        precompile_encodings: overrides
            .precompile_encodings
            .unwrap_or_else(|| vec![PrecompileEncoding::Eip196Eip197]),
    }
}

#[derive(Default)]
pub struct HarnessSelectionsCorruptions {
    pub random: Option<Value>,
    pub encoding: Option<Value>,
    pub hash_to_scalar: Option<Value>,
    pub chain_forms: Option<Value>,
    pub kem: Option<Value>,
    pub key_agreement: Option<Value>,
    pub delivery_proof: Option<Value>,
    pub curves: Option<Value>,
    pub benchmark_concretes: Option<Value>,
    pub precompile_encodings: Option<Value>,
}

pub fn invalidate_harness_selections(corruptions: HarnessSelectionsCorruptions) -> Value {
    let mut value = serde_json::to_value(build_harness_selections(Default::default()))
        .expect("a built selections set renders as a value");
    if let Some(random) = corruptions.random {
        value["random"] = random;
    }
    if let Some(encoding) = corruptions.encoding {
        value["encoding"] = encoding;
    }
    if let Some(hash_to_scalar) = corruptions.hash_to_scalar {
        value["hash_to_scalar"] = hash_to_scalar;
    }
    if let Some(chain_forms) = corruptions.chain_forms {
        value["chain_forms"] = chain_forms;
    }
    if let Some(kem) = corruptions.kem {
        value["kem"] = kem;
    }
    if let Some(key_agreement) = corruptions.key_agreement {
        value["key_agreement"] = key_agreement;
    }
    if let Some(delivery_proof) = corruptions.delivery_proof {
        value["delivery_proof"] = delivery_proof;
    }
    if let Some(curves) = corruptions.curves {
        value["curves"] = curves;
    }
    if let Some(benchmark_concretes) = corruptions.benchmark_concretes {
        value["benchmark_concretes"] = benchmark_concretes;
    }
    if let Some(precompile_encodings) = corruptions.precompile_encodings {
        value["precompile_encodings"] = precompile_encodings;
    }
    value
}

#[derive(Default)]
pub struct AdmitSelectionsDepsOverrides;

pub fn build_admit_selections_deps(
    _overrides: AdmitSelectionsDepsOverrides,
) -> AdmitSelectionsDeps {
    AdmitSelectionsDeps
}

#[derive(Default)]
pub struct AdmitSelectionsParamsOverrides;

pub fn build_admit_selections_params(
    _overrides: AdmitSelectionsParamsOverrides,
) -> AdmitSelectionsParams {
    AdmitSelectionsParams
}

#[derive(Default)]
pub struct AdmitSelectionsPayloadOverrides;

pub fn build_admit_selections_payload<'a>(
    random: &'a RandomDocument,
    encoding: &'a SelectionDocument,
    hash_to_scalar: &'a SelectionDocument,
    chain_forms: &'a SelectionDocument,
    kem: &'a KemDocument,
    key_agreement: &'a KeyAgreementDocument,
    delivery_proof: &'a DeliveryProofDocument,
    curves: &'a [CurveDocument],
    benchmark_concretes: &'a [CurveDocument],
    precompile_encodings: &'a [String],
    _overrides: AdmitSelectionsPayloadOverrides,
) -> AdmitSelectionsPayload<'a> {
    AdmitSelectionsPayload {
        random,
        encoding,
        hash_to_scalar,
        chain_forms,
        kem,
        key_agreement,
        delivery_proof,
        curves,
        benchmark_concretes,
        precompile_encodings,
    }
}

#[derive(Default)]
pub struct AdmitSelectionsSuccessReturnOverrides {
    pub selections: Option<HarnessSelections>,
}

pub fn build_admit_selections_success_return(
    overrides: AdmitSelectionsSuccessReturnOverrides,
) -> AdmitSelectionsSuccessReturn {
    AdmitSelectionsSuccessReturn {
        selections: overrides
            .selections
            .unwrap_or_else(|| build_harness_selections(Default::default())),
    }
}

#[derive(Default)]
pub struct AdmitSelectionsSuccessReturnCorruptions {
    pub selections: Option<Value>,
}

pub fn invalidate_admit_selections_success_return(
    corruptions: AdmitSelectionsSuccessReturnCorruptions,
) -> Value {
    let mut value = serde_json::to_value(build_admit_selections_success_return(Default::default()))
        .expect("a built success return renders as a value");
    if let Some(selections) = corruptions.selections {
        value["selections"] = selections;
    }
    value
}

pub fn mock_admit_selections(
    _deps: &AdmitSelectionsDeps,
    _params: AdmitSelectionsParams,
    _payload: AdmitSelectionsPayload<'_>,
) -> AdmitSelectionsReturn {
    let _: AdmitSelectionsFn = mock_admit_selections;
    Ok(build_admit_selections_success_return(Default::default()))
}
