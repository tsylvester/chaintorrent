use chain::{ChainFormsConcrete, ChainFormsIdentifier};
use encoding::{EncodingConcrete, EncodingIdentifier};
use envelope::{EnvelopeAlgebra, KeyAgreementConcrete, KeyAgreementIdentifier};
use hash_to_scalar::{HashToScalarConcrete, HashToScalarIdentifier};
use kem::{IdentityScope, KemConcrete, KemIdentifier};
use pairing::{PairingConcrete, PrecompileEncoding, TargetGroupEncodingIdentifier};
use proof::DeliveryProofConcrete;
use random::RandomSourceKind;

use super::super::layers::provides::SelectionsTable;
use crate::verifier::provides::VerifierConcreteKind;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdmittedEncoding {
    pub concrete: EncodingConcrete,
    pub identifier: EncodingIdentifier,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdmittedHashToScalar {
    pub concrete: HashToScalarConcrete,
    pub identifier: HashToScalarIdentifier,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdmittedChainForms {
    pub concrete: ChainFormsConcrete,
    pub identifier: ChainFormsIdentifier,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdmittedKem {
    pub concrete: KemConcrete,
    pub identifier: KemIdentifier,
    pub scope: IdentityScope,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdmittedKeyAgreement {
    pub concrete: KeyAgreementConcrete,
    pub identifier: KeyAgreementIdentifier,
    pub algebra: EnvelopeAlgebra,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdmittedDeliveryProof {
    pub concrete: DeliveryProofConcrete,
    pub algebra: EnvelopeAlgebra,
    pub statement_version: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdmittedPairing {
    pub concrete: PairingConcrete,
    pub target_group_encoding: TargetGroupEncodingIdentifier,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdmittedCurve {
    pub label: String,
    pub pairing: AdmittedPairing,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdmittedSelections {
    pub random_source: RandomSourceKind,
    pub encoding: AdmittedEncoding,
    pub hash_to_scalar: AdmittedHashToScalar,
    pub chain_forms: AdmittedChainForms,
    pub kem: AdmittedKem,
    pub key_agreement: AdmittedKeyAgreement,
    pub delivery_proof: AdmittedDeliveryProof,
    pub verifier: VerifierConcreteKind,
    pub curves: Vec<AdmittedCurve>,
    pub benchmark_concretes: Vec<AdmittedPairing>,
    pub precompile_encodings: Vec<PrecompileEncoding>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PairingField {
    PairingConcrete,
    TargetGroupEncoding,
}

#[derive(Clone, Copy, Debug)]
pub struct AdmitSelectionsDeps;

#[derive(Clone, Copy, Debug)]
pub struct AdmitSelectionsParams;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdmitSelectionsPayload {
    pub selections: SelectionsTable,
    pub precompile_encodings: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdmitSelectionsSuccessReturn {
    pub admitted: AdmittedSelections,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AdmitSelectionsErrorReturn {
    RandomSource {
        text: String,
    },
    Encoding {
        text: String,
    },
    HashToScalar {
        text: String,
    },
    ChainForms {
        text: String,
    },
    Verifier {
        text: String,
    },
    IdentityScope {
        text: String,
    },
    EnvelopeAlgebra {
        text: String,
    },
    Kem {
        text: String,
    },
    KeyAgreement {
        text: String,
    },
    DeliveryProof {
        text: String,
    },
    Curve {
        index: usize,
        field: PairingField,
        text: String,
    },
    BenchmarkConcrete {
        index: usize,
        field: PairingField,
        text: String,
    },
    PrecompileEncoding {
        index: usize,
        text: String,
    },
}

pub type AdmitSelectionsReturn = Result<AdmitSelectionsSuccessReturn, AdmitSelectionsErrorReturn>;

pub type AdmitSelectionsFn = fn(
    &AdmitSelectionsDeps,
    AdmitSelectionsParams,
    AdmitSelectionsPayload,
) -> AdmitSelectionsReturn;
