use clap::{ArgAction, Parser};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Parser, Clone, Debug, PartialEq, Eq)]
#[command(name = "harness-crypto")]
pub struct ConfigArguments {
    #[arg(long = "config", value_name = "PATH", action = ArgAction::Append)]
    pub config: Vec<PathBuf>,
    #[arg(long = "set", value_name = "KEY=VALUE", action = ArgAction::Append)]
    pub set: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CanonicalValueTable {
    pub kind: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CurveTable {
    pub label: String,
    pub pairing_concrete: String,
    pub target_group_encoding: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BenchmarkConcreteTable {
    pub pairing_concrete: String,
    pub target_group_encoding: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SelectionsTable {
    pub random_source: String,
    pub encoding: String,
    pub hash_to_scalar: String,
    pub chain_forms: String,
    pub kem: String,
    pub key_agreement: String,
    pub delivery_proof: String,
    pub verifier: String,
    pub identity_scope: String,
    pub envelope_algebra: String,
    pub delivery_statement_version: u16,
    pub curves: Vec<CurveTable>,
    pub benchmark_concretes: Vec<BenchmarkConcreteTable>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NetworkProfileTable {
    pub chain_identifier: CanonicalValueTable,
    pub entitlement_contract: CanonicalValueTable,
    pub deployment_record: CanonicalValueTable,
    pub gas_price_oracle: CanonicalValueTable,
    pub verifier_endpoint: String,
    pub precompile_encodings: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContextTable {
    pub suite_identifier: String,
    pub suite_version: u32,
    pub asset_root: String,
    pub parameter_set_identifier: String,
    pub source_entitlement: CanonicalValueTable,
    pub target_entitlement: CanonicalValueTable,
    pub old_interval: CanonicalValueTable,
    pub new_interval: CanonicalValueTable,
    pub seller: CanonicalValueTable,
    pub buyer: CanonicalValueTable,
    pub expiry: u64,
    pub purpose: u16,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StatementsTable {
    pub mint: ContextTable,
    pub transfer: ContextTable,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IdentitiesTable {
    pub proof: String,
    pub mapping: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GenerateKindTable {
    pub count: Option<u32>,
    pub terms: Option<u32>,
    pub message_length: Option<usize>,
    pub file: String,
    pub struct_name: String,
    pub decode_function: String,
    pub path_constant: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LibraryTable {
    pub file: String,
    pub library: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GenerateTable {
    pub concrete: String,
    pub kinds: BTreeMap<String, GenerateKindTable>,
    pub constants_library: LibraryTable,
    pub records_library: LibraryTable,
    pub foundry_root: PathBuf,
    pub generated_directory: Vec<String>,
    pub curve_directories: BTreeMap<String, String>,
    pub compiler_version: String,
    pub license: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DestinationTable {
    pub kind: String,
    pub path: Option<PathBuf>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BenchmarkTable {
    pub iterations: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MeasurementTable {
    pub iterations: u32,
    pub conditions: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdjustmentTable {
    pub piece_group_size: Option<u32>,
    pub primary_curve: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReportTable {
    pub destinations: Vec<DestinationTable>,
    pub piece_group_size: u32,
    pub primary_curve: String,
    pub adjustment: Option<AdjustmentTable>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OutputTable {
    pub destinations: Vec<DestinationTable>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConfigDocument {
    pub selections: SelectionsTable,
    pub network_profile: NetworkProfileTable,
    pub statements: StatementsTable,
    pub identities: IdentitiesTable,
    pub generate: GenerateTable,
    pub benchmark: BenchmarkTable,
    pub measurement: MeasurementTable,
    pub report: ReportTable,
    pub output: OutputTable,
}

#[derive(Clone, Copy, Debug)]
pub struct LayersDeps;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LayersParams {
    pub arguments: Vec<OsString>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LayersPayload {
    pub defaults: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LayersSuccessReturn {
    pub document: ConfigDocument,
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
    MalformedAssignment {
        assignment: String,
    },
    ParseAssignment {
        assignment: String,
        error: toml::de::Error,
    },
    Document(toml::de::Error),
}

pub type LayersReturn = Result<LayersSuccessReturn, LayersErrorReturn>;

pub type LayersFn = fn(&LayersDeps, LayersParams, LayersPayload) -> LayersReturn;
