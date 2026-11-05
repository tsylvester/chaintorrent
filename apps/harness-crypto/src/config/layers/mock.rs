#![allow(dead_code, clippy::too_many_arguments)]

use super::interface::{
    AdjustmentTable, BenchmarkConcreteTable, BenchmarkTable, CanonicalValueTable, ConfigArguments,
    ConfigDocument, ContextTable, CurveTable, DestinationTable, GenerateKindTable, GenerateTable,
    IdentitiesTable, LayersDeps, LayersFn, LayersParams, LayersPayload, LayersReturn,
    LayersSuccessReturn, LibraryTable, MeasurementTable, NetworkProfileTable, OutputTable,
    ReportTable, SelectionsTable, StatementsTable,
};
use std::ffi::OsString;
use std::path::PathBuf;

const LAYERS_DEFAULTS_TEXT: &str = r#"
[selections]
random_source = "operating_system"
encoding = "abi"
hash_to_scalar = "keccak256"
chain_forms = "evm"
kem = "bb1_depth_one"
key_agreement = "pairing_elgamal"
delivery_proof = "schnorr_fs"
verifier = "evm"
identity_scope = "entitlement"
envelope_algebra = "pairing_elgamal"
delivery_statement_version = 1
curves = [{ label = "curve-a", pairing_concrete = "concrete-a", target_group_encoding = "encoding-a" }]
benchmark_concretes = [{ pairing_concrete = "concrete-a", target_group_encoding = "encoding-a" }]

[network_profile]
chain_identifier = { kind = "uint256", value = "1" }
entitlement_contract = { kind = "bytes20", value = "0x1111111111111111111111111111111111111111" }
deployment_record = { kind = "bytes20", value = "0x2222222222222222222222222222222222222222" }
gas_price_oracle = { kind = "bytes20", value = "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" }
verifier_endpoint = "http://endpoint-a.test"
precompile_encodings = ["encoding-a"]

[statements.mint]
suite_identifier = "0x3333333333333333333333333333333333333333333333333333333333333333"
suite_version = 1
asset_root = "0x4444444444444444444444444444444444444444444444444444444444444444"
parameter_set_identifier = "0x5555555555555555555555555555555555555555555555555555555555555555"
source_entitlement = { kind = "uint256", value = "101" }
target_entitlement = { kind = "uint256", value = "102" }
old_interval = { kind = "uint64", value = "1" }
new_interval = { kind = "uint64", value = "2" }
seller = { kind = "bytes20", value = "0x9999999999999999999999999999999999999999" }
buyer = { kind = "bytes20", value = "0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb" }
expiry = 1000
purpose = 1

[statements.transfer]
suite_identifier = "0x3333333333333333333333333333333333333333333333333333333333333333"
suite_version = 1
asset_root = "0x4444444444444444444444444444444444444444444444444444444444444444"
parameter_set_identifier = "0x5555555555555555555555555555555555555555555555555555555555555555"
source_entitlement = { kind = "uint256", value = "101" }
target_entitlement = { kind = "uint256", value = "102" }
old_interval = { kind = "uint64", value = "1" }
new_interval = { kind = "uint64", value = "2" }
seller = { kind = "bytes20", value = "0x9999999999999999999999999999999999999999" }
buyer = { kind = "bytes20", value = "0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb" }
expiry = 1000
purpose = 2

[statements.next]
suite_identifier = "0x3333333333333333333333333333333333333333333333333333333333333333"
suite_version = 1
asset_root = "0x4444444444444444444444444444444444444444444444444444444444444444"
parameter_set_identifier = "0x5555555555555555555555555555555555555555555555555555555555555555"
source_entitlement = { kind = "uint256", value = "101" }
target_entitlement = { kind = "uint256", value = "102" }
old_interval = { kind = "uint64", value = "1" }
new_interval = { kind = "uint64", value = "2" }
seller = { kind = "bytes20", value = "0x9999999999999999999999999999999999999999" }
buyer = { kind = "bytes20", value = "0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb" }
expiry = 1000
purpose = 2

[identities]
proof = "0x6666666666666666666666666666666666666666666666666666666666666666"
mapping = ["0x7777777777777777777777777777777777777777777777777777777777777777"]
other = "0x8888888888888888888888888888888888888888888888888888888888888888"

[generate]
concrete = "solidity"
kinds = { "kind-a" = { count = 2, file = "kind-a.txt", struct_name = "KindA", decode_function = "decodeKindA", path_constant = "KIND_A_PATH" } }
constants_library = { file = "constants-a.sol", library = "ConstantsA" }
records_library = { file = "records-a.sol", library = "RecordsA" }
foundry_root = "root"
generated_directory = ["generated"]
curve_directories = { "Bn254" = "directory-a", "Bls12381" = "directory-b" }
compiler_version = "0.8.0"
license = "MIT"

[benchmark]
iterations = 1

[measurement]
iterations = 1
conditions = "conditions-a"

[report]
destinations = [{ kind = "standard_output" }]
piece_group_size = 16384
primary_curve = "curve-a"

[output]
destinations = [{ kind = "standard_output" }]
"#;

#[derive(Default)]
pub struct CanonicalValueTableOverrides {
    pub kind: Option<String>,
    pub value: Option<String>,
}

pub fn build_canonical_value_table(overrides: CanonicalValueTableOverrides) -> CanonicalValueTable {
    CanonicalValueTable {
        kind: overrides.kind.unwrap_or_else(|| "uint256".to_string()),
        value: overrides.value.unwrap_or_else(|| "1".to_string()),
    }
}

#[derive(Default)]
pub struct CurveTableOverrides {
    pub label: Option<String>,
    pub pairing_concrete: Option<String>,
    pub target_group_encoding: Option<String>,
}

pub fn build_curve_table(overrides: CurveTableOverrides) -> CurveTable {
    CurveTable {
        label: overrides.label.unwrap_or_else(|| "curve-a".to_string()),
        pairing_concrete: overrides
            .pairing_concrete
            .unwrap_or_else(|| "concrete-a".to_string()),
        target_group_encoding: overrides
            .target_group_encoding
            .unwrap_or_else(|| "encoding-a".to_string()),
    }
}

#[derive(Default)]
pub struct BenchmarkConcreteTableOverrides {
    pub pairing_concrete: Option<String>,
    pub target_group_encoding: Option<String>,
}

pub fn build_benchmark_concrete_table(
    overrides: BenchmarkConcreteTableOverrides,
) -> BenchmarkConcreteTable {
    BenchmarkConcreteTable {
        pairing_concrete: overrides
            .pairing_concrete
            .unwrap_or_else(|| "concrete-a".to_string()),
        target_group_encoding: overrides
            .target_group_encoding
            .unwrap_or_else(|| "encoding-a".to_string()),
    }
}

#[derive(Default)]
pub struct SelectionsTableOverrides {
    pub random_source: Option<String>,
    pub encoding: Option<String>,
    pub hash_to_scalar: Option<String>,
    pub chain_forms: Option<String>,
    pub kem: Option<String>,
    pub key_agreement: Option<String>,
    pub delivery_proof: Option<String>,
    pub verifier: Option<String>,
    pub identity_scope: Option<String>,
    pub envelope_algebra: Option<String>,
    pub delivery_statement_version: Option<u16>,
    pub curves: Option<Vec<CurveTable>>,
    pub benchmark_concretes: Option<Vec<BenchmarkConcreteTable>>,
}

pub fn build_selections_table(overrides: SelectionsTableOverrides) -> SelectionsTable {
    SelectionsTable {
        random_source: overrides
            .random_source
            .unwrap_or_else(|| "operating_system".to_string()),
        encoding: overrides.encoding.unwrap_or_else(|| "abi".to_string()),
        hash_to_scalar: overrides
            .hash_to_scalar
            .unwrap_or_else(|| "keccak256".to_string()),
        chain_forms: overrides.chain_forms.unwrap_or_else(|| "evm".to_string()),
        kem: overrides.kem.unwrap_or_else(|| "bb1_depth_one".to_string()),
        key_agreement: overrides
            .key_agreement
            .unwrap_or_else(|| "pairing_elgamal".to_string()),
        delivery_proof: overrides
            .delivery_proof
            .unwrap_or_else(|| "schnorr_fs".to_string()),
        verifier: overrides.verifier.unwrap_or_else(|| "evm".to_string()),
        identity_scope: overrides
            .identity_scope
            .unwrap_or_else(|| "entitlement".to_string()),
        envelope_algebra: overrides
            .envelope_algebra
            .unwrap_or_else(|| "pairing_elgamal".to_string()),
        delivery_statement_version: overrides.delivery_statement_version.unwrap_or(1),
        curves: overrides
            .curves
            .unwrap_or_else(|| vec![build_curve_table(Default::default())]),
        benchmark_concretes: overrides
            .benchmark_concretes
            .unwrap_or_else(|| vec![build_benchmark_concrete_table(Default::default())]),
    }
}

#[derive(Default)]
pub struct NetworkProfileTableOverrides {
    pub chain_identifier: Option<CanonicalValueTable>,
    pub entitlement_contract: Option<CanonicalValueTable>,
    pub deployment_record: Option<CanonicalValueTable>,
    pub gas_price_oracle: Option<CanonicalValueTable>,
    pub verifier_endpoint: Option<String>,
    pub precompile_encodings: Option<Vec<String>>,
}

pub fn build_network_profile_table(overrides: NetworkProfileTableOverrides) -> NetworkProfileTable {
    NetworkProfileTable {
        chain_identifier: overrides.chain_identifier.unwrap_or_else(|| {
            build_canonical_value_table(CanonicalValueTableOverrides::default())
        }),
        entitlement_contract: overrides.entitlement_contract.unwrap_or_else(|| {
            build_canonical_value_table(CanonicalValueTableOverrides {
                kind: Some("bytes20".to_string()),
                value: Some("0x1111111111111111111111111111111111111111".to_string()),
            })
        }),
        deployment_record: overrides.deployment_record.unwrap_or_else(|| {
            build_canonical_value_table(CanonicalValueTableOverrides {
                kind: Some("bytes20".to_string()),
                value: Some("0x2222222222222222222222222222222222222222".to_string()),
            })
        }),
        gas_price_oracle: overrides.gas_price_oracle.unwrap_or_else(|| {
            build_canonical_value_table(CanonicalValueTableOverrides {
                kind: Some("bytes20".to_string()),
                value: Some("0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string()),
            })
        }),
        verifier_endpoint: overrides
            .verifier_endpoint
            .unwrap_or_else(|| "http://endpoint-a.test".to_string()),
        precompile_encodings: overrides
            .precompile_encodings
            .unwrap_or_else(|| vec!["encoding-a".to_string()]),
    }
}

#[derive(Default)]
pub struct ContextTableOverrides {
    pub suite_identifier: Option<String>,
    pub suite_version: Option<u32>,
    pub asset_root: Option<String>,
    pub parameter_set_identifier: Option<String>,
    pub source_entitlement: Option<CanonicalValueTable>,
    pub target_entitlement: Option<CanonicalValueTable>,
    pub old_interval: Option<CanonicalValueTable>,
    pub new_interval: Option<CanonicalValueTable>,
    pub seller: Option<CanonicalValueTable>,
    pub buyer: Option<CanonicalValueTable>,
    pub expiry: Option<u64>,
    pub purpose: Option<u16>,
}

pub fn build_context_table(overrides: ContextTableOverrides) -> ContextTable {
    ContextTable {
        suite_identifier: overrides.suite_identifier.unwrap_or_else(|| {
            "0x3333333333333333333333333333333333333333333333333333333333333333".to_string()
        }),
        suite_version: overrides.suite_version.unwrap_or(1),
        asset_root: overrides.asset_root.unwrap_or_else(|| {
            "0x4444444444444444444444444444444444444444444444444444444444444444".to_string()
        }),
        parameter_set_identifier: overrides.parameter_set_identifier.unwrap_or_else(|| {
            "0x5555555555555555555555555555555555555555555555555555555555555555".to_string()
        }),
        source_entitlement: overrides.source_entitlement.unwrap_or_else(|| {
            build_canonical_value_table(CanonicalValueTableOverrides {
                kind: Some("uint256".to_string()),
                value: Some("101".to_string()),
            })
        }),
        target_entitlement: overrides.target_entitlement.unwrap_or_else(|| {
            build_canonical_value_table(CanonicalValueTableOverrides {
                kind: Some("uint256".to_string()),
                value: Some("102".to_string()),
            })
        }),
        old_interval: overrides.old_interval.unwrap_or_else(|| {
            build_canonical_value_table(CanonicalValueTableOverrides {
                kind: Some("uint64".to_string()),
                value: Some("1".to_string()),
            })
        }),
        new_interval: overrides.new_interval.unwrap_or_else(|| {
            build_canonical_value_table(CanonicalValueTableOverrides {
                kind: Some("uint64".to_string()),
                value: Some("2".to_string()),
            })
        }),
        seller: overrides.seller.unwrap_or_else(|| {
            build_canonical_value_table(CanonicalValueTableOverrides {
                kind: Some("bytes20".to_string()),
                value: Some("0x9999999999999999999999999999999999999999".to_string()),
            })
        }),
        buyer: overrides.buyer.unwrap_or_else(|| {
            build_canonical_value_table(CanonicalValueTableOverrides {
                kind: Some("bytes20".to_string()),
                value: Some("0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".to_string()),
            })
        }),
        expiry: overrides.expiry.unwrap_or(1000),
        purpose: overrides.purpose.unwrap_or(1),
    }
}

#[derive(Default)]
pub struct StatementsTableOverrides {
    pub mint: Option<ContextTable>,
    pub transfer: Option<ContextTable>,
    pub next: Option<ContextTable>,
}

pub fn build_statements_table(overrides: StatementsTableOverrides) -> StatementsTable {
    StatementsTable {
        mint: overrides
            .mint
            .unwrap_or_else(|| build_context_table(Default::default())),
        transfer: overrides.transfer.unwrap_or_else(|| {
            build_context_table(ContextTableOverrides {
                purpose: Some(2),
                ..Default::default()
            })
        }),
        next: overrides.next.unwrap_or_else(|| {
            build_context_table(ContextTableOverrides {
                purpose: Some(2),
                ..Default::default()
            })
        }),
    }
}

#[derive(Default)]
pub struct IdentitiesTableOverrides {
    pub proof: Option<String>,
    pub mapping: Option<Vec<String>>,
    pub other: Option<String>,
}

pub fn build_identities_table(overrides: IdentitiesTableOverrides) -> IdentitiesTable {
    IdentitiesTable {
        proof: overrides.proof.unwrap_or_else(|| {
            "0x6666666666666666666666666666666666666666666666666666666666666666".to_string()
        }),
        mapping: overrides.mapping.unwrap_or_else(|| {
            vec!["0x7777777777777777777777777777777777777777777777777777777777777777".to_string()]
        }),
        other: overrides.other.unwrap_or_else(|| {
            "0x8888888888888888888888888888888888888888888888888888888888888888".to_string()
        }),
    }
}

#[derive(Default)]
pub struct GenerateKindTableOverrides {
    pub count: Option<Option<u32>>,
    pub terms: Option<Option<u32>>,
    pub message_length: Option<Option<usize>>,
    pub file: Option<String>,
    pub struct_name: Option<String>,
    pub decode_function: Option<String>,
    pub path_constant: Option<String>,
}

pub fn build_generate_kind_table(overrides: GenerateKindTableOverrides) -> GenerateKindTable {
    GenerateKindTable {
        count: overrides.count.unwrap_or(Some(2)),
        terms: overrides.terms.unwrap_or(None),
        message_length: overrides.message_length.unwrap_or(None),
        file: overrides.file.unwrap_or_else(|| "kind-a.txt".to_string()),
        struct_name: overrides.struct_name.unwrap_or_else(|| "KindA".to_string()),
        decode_function: overrides
            .decode_function
            .unwrap_or_else(|| "decodeKindA".to_string()),
        path_constant: overrides
            .path_constant
            .unwrap_or_else(|| "KIND_A_PATH".to_string()),
    }
}

#[derive(Default)]
pub struct LibraryTableOverrides {
    pub file: Option<String>,
    pub library: Option<String>,
}

pub fn build_library_table(overrides: LibraryTableOverrides) -> LibraryTable {
    LibraryTable {
        file: overrides
            .file
            .unwrap_or_else(|| "constants-a.sol".to_string()),
        library: overrides
            .library
            .unwrap_or_else(|| "ConstantsA".to_string()),
    }
}

#[derive(Default)]
pub struct GenerateTableOverrides {
    pub concrete: Option<String>,
    pub kinds: Option<std::collections::BTreeMap<String, GenerateKindTable>>,
    pub constants_library: Option<LibraryTable>,
    pub records_library: Option<LibraryTable>,
    pub foundry_root: Option<PathBuf>,
    pub generated_directory: Option<Vec<String>>,
    pub curve_directories: Option<std::collections::BTreeMap<String, String>>,
    pub compiler_version: Option<String>,
    pub license: Option<String>,
}

pub fn build_generate_table(overrides: GenerateTableOverrides) -> GenerateTable {
    GenerateTable {
        concrete: overrides.concrete.unwrap_or_else(|| "solidity".to_string()),
        kinds: overrides.kinds.unwrap_or_else(|| {
            std::collections::BTreeMap::from([(
                "kind-a".to_string(),
                build_generate_kind_table(Default::default()),
            )])
        }),
        constants_library: overrides
            .constants_library
            .unwrap_or_else(|| build_library_table(Default::default())),
        records_library: overrides.records_library.unwrap_or_else(|| {
            build_library_table(LibraryTableOverrides {
                file: Some("records-a.sol".to_string()),
                library: Some("RecordsA".to_string()),
            })
        }),
        foundry_root: overrides
            .foundry_root
            .unwrap_or_else(|| PathBuf::from("root")),
        generated_directory: overrides
            .generated_directory
            .unwrap_or_else(|| vec!["generated".to_string()]),
        curve_directories: overrides.curve_directories.unwrap_or_else(|| {
            std::collections::BTreeMap::from([
                ("Bn254".to_string(), "directory-a".to_string()),
                ("Bls12381".to_string(), "directory-b".to_string()),
            ])
        }),
        compiler_version: overrides
            .compiler_version
            .unwrap_or_else(|| "0.8.0".to_string()),
        license: overrides.license.unwrap_or_else(|| "MIT".to_string()),
    }
}

#[derive(Default)]
pub struct DestinationTableOverrides {
    pub kind: Option<String>,
    pub path: Option<Option<PathBuf>>,
}

pub fn build_destination_table(overrides: DestinationTableOverrides) -> DestinationTable {
    DestinationTable {
        kind: overrides
            .kind
            .unwrap_or_else(|| "standard_output".to_string()),
        path: overrides.path.unwrap_or(None),
    }
}

#[derive(Default)]
pub struct BenchmarkTableOverrides {
    pub iterations: Option<u32>,
}

pub fn build_benchmark_table(overrides: BenchmarkTableOverrides) -> BenchmarkTable {
    BenchmarkTable {
        iterations: overrides.iterations.unwrap_or(1),
    }
}

#[derive(Default)]
pub struct MeasurementTableOverrides {
    pub iterations: Option<u32>,
    pub conditions: Option<String>,
}

pub fn build_measurement_table(overrides: MeasurementTableOverrides) -> MeasurementTable {
    MeasurementTable {
        iterations: overrides.iterations.unwrap_or(1),
        conditions: overrides
            .conditions
            .unwrap_or_else(|| "conditions-a".to_string()),
    }
}

#[derive(Default)]
pub struct AdjustmentTableOverrides {
    pub piece_group_size: Option<Option<u32>>,
    pub primary_curve: Option<Option<String>>,
}

pub fn build_adjustment_table(overrides: AdjustmentTableOverrides) -> AdjustmentTable {
    AdjustmentTable {
        piece_group_size: overrides.piece_group_size.unwrap_or(None),
        primary_curve: overrides.primary_curve.unwrap_or(None),
    }
}

#[derive(Default)]
pub struct ReportTableOverrides {
    pub destinations: Option<Vec<DestinationTable>>,
    pub piece_group_size: Option<u32>,
    pub primary_curve: Option<String>,
    pub adjustment: Option<Option<AdjustmentTable>>,
}

pub fn build_report_table(overrides: ReportTableOverrides) -> ReportTable {
    ReportTable {
        destinations: overrides
            .destinations
            .unwrap_or_else(|| vec![build_destination_table(Default::default())]),
        piece_group_size: overrides.piece_group_size.unwrap_or(16384),
        primary_curve: overrides
            .primary_curve
            .unwrap_or_else(|| "curve-a".to_string()),
        adjustment: overrides.adjustment.unwrap_or(None),
    }
}

#[derive(Default)]
pub struct OutputTableOverrides {
    pub destinations: Option<Vec<DestinationTable>>,
}

pub fn build_output_table(overrides: OutputTableOverrides) -> OutputTable {
    OutputTable {
        destinations: overrides
            .destinations
            .unwrap_or_else(|| vec![build_destination_table(Default::default())]),
    }
}

#[derive(Default)]
pub struct ConfigDocumentOverrides {
    pub selections: Option<SelectionsTable>,
    pub network_profile: Option<NetworkProfileTable>,
    pub statements: Option<StatementsTable>,
    pub identities: Option<IdentitiesTable>,
    pub generate: Option<GenerateTable>,
    pub benchmark: Option<BenchmarkTable>,
    pub measurement: Option<MeasurementTable>,
    pub report: Option<ReportTable>,
    pub output: Option<OutputTable>,
}

pub fn build_config_document(overrides: ConfigDocumentOverrides) -> ConfigDocument {
    ConfigDocument {
        selections: overrides
            .selections
            .unwrap_or_else(|| build_selections_table(Default::default())),
        network_profile: overrides
            .network_profile
            .unwrap_or_else(|| build_network_profile_table(Default::default())),
        statements: overrides
            .statements
            .unwrap_or_else(|| build_statements_table(Default::default())),
        identities: overrides
            .identities
            .unwrap_or_else(|| build_identities_table(Default::default())),
        generate: overrides
            .generate
            .unwrap_or_else(|| build_generate_table(Default::default())),
        benchmark: overrides
            .benchmark
            .unwrap_or_else(|| build_benchmark_table(Default::default())),
        measurement: overrides
            .measurement
            .unwrap_or_else(|| build_measurement_table(Default::default())),
        report: overrides
            .report
            .unwrap_or_else(|| build_report_table(Default::default())),
        output: overrides
            .output
            .unwrap_or_else(|| build_output_table(Default::default())),
    }
}

#[derive(Default)]
pub struct ConfigArgumentsOverrides {
    pub config: Option<Vec<PathBuf>>,
    pub set: Option<Vec<String>>,
}

pub fn build_config_arguments(overrides: ConfigArgumentsOverrides) -> ConfigArguments {
    ConfigArguments {
        config: overrides.config.unwrap_or_default(),
        set: overrides.set.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct LayersDefaultsOverrides {
    pub text: Option<String>,
}

pub fn build_layers_defaults(overrides: LayersDefaultsOverrides) -> String {
    overrides
        .text
        .unwrap_or_else(|| LAYERS_DEFAULTS_TEXT.to_string())
}

#[derive(Default)]
pub struct LayersDepsOverrides;

pub fn build_layers_deps(_overrides: LayersDepsOverrides) -> LayersDeps {
    LayersDeps
}

#[derive(Default)]
pub struct LayersParamsOverrides {
    pub arguments: Option<Vec<OsString>>,
}

pub fn build_layers_params(overrides: LayersParamsOverrides) -> LayersParams {
    LayersParams {
        arguments: overrides
            .arguments
            .unwrap_or_else(|| vec![OsString::from("harness-crypto")]),
    }
}

#[derive(Default)]
pub struct LayersPayloadOverrides {
    pub defaults: Option<String>,
}

pub fn build_layers_payload(overrides: LayersPayloadOverrides) -> LayersPayload {
    LayersPayload {
        defaults: overrides
            .defaults
            .unwrap_or_else(|| build_layers_defaults(Default::default())),
    }
}

#[derive(Default)]
pub struct LayersSuccessReturnOverrides {
    pub document: Option<ConfigDocument>,
}

pub fn build_layers_success_return(overrides: LayersSuccessReturnOverrides) -> LayersSuccessReturn {
    LayersSuccessReturn {
        document: overrides
            .document
            .unwrap_or_else(|| build_config_document(Default::default())),
    }
}

pub fn mock_layers(
    _deps: &LayersDeps,
    _params: LayersParams,
    _payload: LayersPayload,
) -> LayersReturn {
    let _: LayersFn = mock_layers;
    Ok(build_layers_success_return(Default::default()))
}
