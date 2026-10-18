use chain::{ChainFormsConcrete, ChainFormsIdentifier};
use encoding::{EncodingConcrete, EncodingIdentifier};
use envelope::{EnvelopeAlgebra, KeyAgreementConcrete, KeyAgreementIdentifier};
use hash_to_scalar::{HashToScalarConcrete, HashToScalarIdentifier};
use kem::{IdentityScope, KemConcrete, KemIdentifier};
use pairing::{PairingConcrete, PrecompileEncoding, TargetGroupEncodingIdentifier};
use proof::DeliveryProofConcrete;
use random::RandomSourceKind;
use serde::Serialize;

use super::super::layers::provides::{
    CurveDocument, DeliveryProofDocument, KemDocument, KeyAgreementDocument, RandomDocument,
    SelectionDocument,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct EncodingSelection {
    pub concrete: EncodingConcrete,
    pub identifier: EncodingIdentifier,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct HashToScalarSelection {
    pub concrete: HashToScalarConcrete,
    pub identifier: HashToScalarIdentifier,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct ChainFormsSelection {
    pub concrete: ChainFormsConcrete,
    pub identifier: ChainFormsIdentifier,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct KemSelection {
    pub concrete: KemConcrete,
    pub identifier: KemIdentifier,
    pub scope: IdentityScope,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct KeyAgreementSelection {
    pub concrete: KeyAgreementConcrete,
    pub identifier: KeyAgreementIdentifier,
    pub algebra: EnvelopeAlgebra,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct DeliveryProofSelection {
    pub concrete: DeliveryProofConcrete,
    pub statement_version: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct CurveSelection {
    pub pairing: PairingConcrete,
    pub target_group_encoding: TargetGroupEncodingIdentifier,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct HarnessSelections {
    pub random: RandomSourceKind,
    pub encoding: EncodingSelection,
    pub hash_to_scalar: HashToScalarSelection,
    pub chain_forms: ChainFormsSelection,
    pub kem: KemSelection,
    pub key_agreement: KeyAgreementSelection,
    pub delivery_proof: DeliveryProofSelection,
    pub curves: Vec<CurveSelection>,
    pub benchmark_concretes: Vec<CurveSelection>,
    pub precompile_encodings: Vec<PrecompileEncoding>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectionField {
    RandomSource,
    EncodingConcrete,
    EncodingIdentifier,
    HashToScalarConcrete,
    HashToScalarIdentifier,
    ChainFormsConcrete,
    ChainFormsIdentifier,
    KemConcrete,
    KemIdentifier,
    KemScope,
    KeyAgreementConcrete,
    KeyAgreementIdentifier,
    KeyAgreementAlgebra,
    DeliveryProofConcrete,
    CurvePairing { index: usize },
    CurveTargetGroupEncoding { index: usize },
    BenchmarkPairing { index: usize },
    BenchmarkTargetGroupEncoding { index: usize },
    PrecompileEncoding { index: usize },
}

pub struct AdmitSelectionsDeps;

pub struct AdmitSelectionsParams;

pub struct AdmitSelectionsPayload<'a> {
    pub random: &'a RandomDocument,
    pub encoding: &'a SelectionDocument,
    pub hash_to_scalar: &'a SelectionDocument,
    pub chain_forms: &'a SelectionDocument,
    pub kem: &'a KemDocument,
    pub key_agreement: &'a KeyAgreementDocument,
    pub delivery_proof: &'a DeliveryProofDocument,
    pub curves: &'a [CurveDocument],
    pub benchmark_concretes: &'a [CurveDocument],
    pub precompile_encodings: &'a [String],
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct AdmitSelectionsSuccessReturn {
    pub selections: HarnessSelections,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AdmitSelectionsErrorReturn {
    UnknownValue { field: SelectionField, text: String },
    NoCurves,
}

pub type AdmitSelectionsReturn = Result<AdmitSelectionsSuccessReturn, AdmitSelectionsErrorReturn>;

pub type AdmitSelectionsFn = fn(
    &AdmitSelectionsDeps,
    AdmitSelectionsParams,
    AdmitSelectionsPayload<'_>,
) -> AdmitSelectionsReturn;
