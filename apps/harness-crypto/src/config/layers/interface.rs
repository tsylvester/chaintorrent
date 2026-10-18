use clap::Parser;
use serde::{Deserialize, Serialize};
use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq, Parser)]
pub struct ConfigArguments {
    #[arg(long = "config", value_name = "PATH")]
    pub config: Vec<PathBuf>,
    #[arg(long = "set", value_name = "KEY=VALUE")]
    pub set: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RandomDocument {
    pub source: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SelectionDocument {
    pub concrete: String,
    pub identifier: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct KemDocument {
    pub concrete: String,
    pub identifier: String,
    pub scope: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct KeyAgreementDocument {
    pub concrete: String,
    pub identifier: String,
    pub algebra: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DeliveryProofDocument {
    pub concrete: String,
    pub statement_version: u16,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CurveDocument {
    pub pairing: String,
    pub target_group_encoding: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CanonicalFieldDocumentValue {
    Integer(u64),
    Text(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CanonicalFieldDocument {
    pub kind: String,
    pub value: CanonicalFieldDocumentValue,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NetworkDocument {
    pub precompile_encodings: Vec<String>,
    pub chain: CanonicalFieldDocument,
    pub entitlement_contract: CanonicalFieldDocument,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BenchmarkDocument {
    pub iterations: u32,
    pub concretes: Vec<CurveDocument>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DeliveryContextDocument {
    pub suite_identifier: String,
    pub suite_version: u16,
    pub asset_identity_hash: String,
    pub parameter_set_identifier: String,
    pub source_entitlement: CanonicalFieldDocument,
    pub target_entitlement: CanonicalFieldDocument,
    pub old_interval: CanonicalFieldDocument,
    pub new_interval: CanonicalFieldDocument,
    pub seller: CanonicalFieldDocument,
    pub buyer: CanonicalFieldDocument,
    pub expiry: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StatementsDocument {
    pub mint: DeliveryContextDocument,
    pub transfer: DeliveryContextDocument,
    pub mint_purpose: u16,
    pub transfer_purpose: u16,
    pub identities: Vec<String>,
    pub proof_identity: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VectorKindDocument {
    pub kind: String,
    pub count: Option<u32>,
    pub terms: Option<u32>,
    pub message_length: Option<usize>,
    pub file_name: String,
    pub struct_name: String,
    pub decode_function_name: String,
    pub path_constant_name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LibraryDocument {
    pub file_name: String,
    pub library_name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CurveDirectoriesDocument {
    pub bn254: String,
    pub bls12_381: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CompilerVersionDocument {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GenerateDocument {
    pub concrete: String,
    pub kinds: Vec<VectorKindDocument>,
    pub constants_library: LibraryDocument,
    pub records_library: LibraryDocument,
    pub foundry_root: PathBuf,
    pub generated_directory: Vec<String>,
    pub curve_directories: CurveDirectoriesDocument,
    pub compiler_version: CompilerVersionDocument,
    pub license: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OutputDestinationDocument {
    pub kind: String,
    pub path: Option<PathBuf>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OutputDocument {
    pub destinations: Vec<OutputDestinationDocument>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HarnessConfigDocument {
    pub random: RandomDocument,
    pub encoding: SelectionDocument,
    pub hash_to_scalar: SelectionDocument,
    pub chain_forms: SelectionDocument,
    pub kem: KemDocument,
    pub key_agreement: KeyAgreementDocument,
    pub delivery_proof: DeliveryProofDocument,
    pub curves: Vec<CurveDocument>,
    pub network: NetworkDocument,
    pub benchmark: BenchmarkDocument,
    pub statements: StatementsDocument,
    pub generate: GenerateDocument,
    pub output: OutputDocument,
}

pub struct LayersDeps;

pub struct LayersParams;

pub struct LayersPayload<'a> {
    pub defaults: &'a str,
    pub arguments: Vec<OsString>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LayersSuccessReturn {
    pub document: HarnessConfigDocument,
}

#[derive(Debug)]
pub enum LayersErrorReturn {
    Arguments(clap::Error),
    ParseDefaults(toml::de::Error),
    ReadFile {
        path: PathBuf,
        error: std::io::Error,
    },
    ParseFile {
        path: PathBuf,
        error: toml::de::Error,
    },
    ParseAssignment {
        index: usize,
        error: toml::de::Error,
    },
    AssignmentNotKeyValue {
        index: usize,
        keys: usize,
    },
    Document(toml::de::Error),
}

pub type LayersReturn = Result<LayersSuccessReturn, LayersErrorReturn>;

pub type LayersFn = fn(&LayersDeps, LayersParams, LayersPayload<'_>) -> LayersReturn;
