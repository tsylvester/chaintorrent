#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_arguments
)]

use std::ffi::OsString;
use std::path::PathBuf;
use toml::{Table, Value};

use super::interface::{
    BenchmarkDocument, CanonicalFieldDocument, CanonicalFieldDocumentValue,
    CompilerVersionDocument, ConfigArguments, CurveDirectoriesDocument, CurveDocument,
    DeliveryContextDocument, DeliveryProofDocument, GenerateDocument, HarnessConfigDocument,
    KemDocument, KeyAgreementDocument, LayersDeps, LayersParams, LayersPayload, LayersReturn,
    LayersSuccessReturn, LibraryDocument, NetworkDocument, OutputDestinationDocument,
    OutputDocument, RandomDocument, SelectionDocument, StatementsDocument, VectorKindDocument,
};

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
pub struct RandomDocumentOverrides {
    pub source: Option<String>,
}

pub fn build_random_document(overrides: RandomDocumentOverrides) -> RandomDocument {
    RandomDocument {
        source: overrides
            .source
            .unwrap_or_else(|| "operating_system".to_string()),
    }
}

#[derive(Default)]
pub struct RandomDocumentCorruptions {
    pub source: Option<Value>,
}

pub fn invalidate_random_document(corruptions: RandomDocumentCorruptions) -> Table {
    let mut table = Table::try_from(build_random_document(Default::default()))
        .expect("a built document renders as a table");
    if let Some(source) = corruptions.source {
        table.insert("source".to_string(), source);
    }
    table
}

#[derive(Default)]
pub struct SelectionDocumentOverrides {
    pub concrete: Option<String>,
    pub identifier: Option<String>,
}

pub fn build_selection_document(overrides: SelectionDocumentOverrides) -> SelectionDocument {
    SelectionDocument {
        concrete: overrides.concrete.unwrap_or_else(|| "abi".to_string()),
        identifier: overrides
            .identifier
            .unwrap_or_else(|| "ethereum_abi_v1".to_string()),
    }
}

#[derive(Default)]
pub struct SelectionDocumentCorruptions {
    pub concrete: Option<Value>,
    pub identifier: Option<Value>,
}

pub fn invalidate_selection_document(corruptions: SelectionDocumentCorruptions) -> Table {
    let mut table = Table::try_from(build_selection_document(Default::default()))
        .expect("a built document renders as a table");
    if let Some(concrete) = corruptions.concrete {
        table.insert("concrete".to_string(), concrete);
    }
    if let Some(identifier) = corruptions.identifier {
        table.insert("identifier".to_string(), identifier);
    }
    table
}

#[derive(Default)]
pub struct KemDocumentOverrides {
    pub concrete: Option<String>,
    pub identifier: Option<String>,
    pub scope: Option<String>,
}

pub fn build_kem_document(overrides: KemDocumentOverrides) -> KemDocument {
    KemDocument {
        concrete: overrides
            .concrete
            .unwrap_or_else(|| "bb1_depth_one".to_string()),
        identifier: overrides
            .identifier
            .unwrap_or_else(|| "bb1_depth_one_v1".to_string()),
        scope: overrides.scope.unwrap_or_else(|| "entitlement".to_string()),
    }
}

#[derive(Default)]
pub struct KemDocumentCorruptions {
    pub concrete: Option<Value>,
    pub identifier: Option<Value>,
    pub scope: Option<Value>,
}

pub fn invalidate_kem_document(corruptions: KemDocumentCorruptions) -> Table {
    let mut table = Table::try_from(build_kem_document(Default::default()))
        .expect("a built document renders as a table");
    if let Some(concrete) = corruptions.concrete {
        table.insert("concrete".to_string(), concrete);
    }
    if let Some(identifier) = corruptions.identifier {
        table.insert("identifier".to_string(), identifier);
    }
    if let Some(scope) = corruptions.scope {
        table.insert("scope".to_string(), scope);
    }
    table
}

#[derive(Default)]
pub struct KeyAgreementDocumentOverrides {
    pub concrete: Option<String>,
    pub identifier: Option<String>,
    pub algebra: Option<String>,
}

pub fn build_key_agreement_document(
    overrides: KeyAgreementDocumentOverrides,
) -> KeyAgreementDocument {
    KeyAgreementDocument {
        concrete: overrides
            .concrete
            .unwrap_or_else(|| "pairing_elgamal".to_string()),
        identifier: overrides
            .identifier
            .unwrap_or_else(|| "pairing_elgamal_v1".to_string()),
        algebra: overrides
            .algebra
            .unwrap_or_else(|| "pairing_elgamal".to_string()),
    }
}

#[derive(Default)]
pub struct KeyAgreementDocumentCorruptions {
    pub concrete: Option<Value>,
    pub identifier: Option<Value>,
    pub algebra: Option<Value>,
}

pub fn invalidate_key_agreement_document(corruptions: KeyAgreementDocumentCorruptions) -> Table {
    let mut table = Table::try_from(build_key_agreement_document(Default::default()))
        .expect("a built document renders as a table");
    if let Some(concrete) = corruptions.concrete {
        table.insert("concrete".to_string(), concrete);
    }
    if let Some(identifier) = corruptions.identifier {
        table.insert("identifier".to_string(), identifier);
    }
    if let Some(algebra) = corruptions.algebra {
        table.insert("algebra".to_string(), algebra);
    }
    table
}

#[derive(Default)]
pub struct DeliveryProofDocumentOverrides {
    pub concrete: Option<String>,
    pub statement_version: Option<u16>,
}

pub fn build_delivery_proof_document(
    overrides: DeliveryProofDocumentOverrides,
) -> DeliveryProofDocument {
    DeliveryProofDocument {
        concrete: overrides
            .concrete
            .unwrap_or_else(|| "schnorr_fs".to_string()),
        statement_version: overrides.statement_version.unwrap_or(1),
    }
}

#[derive(Default)]
pub struct DeliveryProofDocumentCorruptions {
    pub concrete: Option<Value>,
    pub statement_version: Option<Value>,
}

pub fn invalidate_delivery_proof_document(corruptions: DeliveryProofDocumentCorruptions) -> Table {
    let mut table = Table::try_from(build_delivery_proof_document(Default::default()))
        .expect("a built document renders as a table");
    if let Some(concrete) = corruptions.concrete {
        table.insert("concrete".to_string(), concrete);
    }
    if let Some(statement_version) = corruptions.statement_version {
        table.insert("statement_version".to_string(), statement_version);
    }
    table
}

#[derive(Default)]
pub struct CurveDocumentOverrides {
    pub pairing: Option<String>,
    pub target_group_encoding: Option<String>,
}

pub fn build_curve_document(overrides: CurveDocumentOverrides) -> CurveDocument {
    CurveDocument {
        pairing: overrides
            .pairing
            .unwrap_or_else(|| "bn254_arkworks".to_string()),
        target_group_encoding: overrides
            .target_group_encoding
            .unwrap_or_else(|| "bn254_v1".to_string()),
    }
}

#[derive(Default)]
pub struct CurveDocumentCorruptions {
    pub pairing: Option<Value>,
    pub target_group_encoding: Option<Value>,
}

pub fn invalidate_curve_document(corruptions: CurveDocumentCorruptions) -> Table {
    let mut table = Table::try_from(build_curve_document(Default::default()))
        .expect("a built document renders as a table");
    if let Some(pairing) = corruptions.pairing {
        table.insert("pairing".to_string(), pairing);
    }
    if let Some(target_group_encoding) = corruptions.target_group_encoding {
        table.insert("target_group_encoding".to_string(), target_group_encoding);
    }
    table
}

#[derive(Default)]
pub struct CanonicalFieldDocumentOverrides {
    pub kind: Option<String>,
    pub value: Option<CanonicalFieldDocumentValue>,
}

pub fn build_canonical_field_document(
    overrides: CanonicalFieldDocumentOverrides,
) -> CanonicalFieldDocument {
    CanonicalFieldDocument {
        kind: overrides.kind.unwrap_or_else(|| "unsigned64".to_string()),
        value: overrides
            .value
            .unwrap_or(CanonicalFieldDocumentValue::Integer(1)),
    }
}

#[derive(Default)]
pub struct CanonicalFieldDocumentCorruptions {
    pub kind: Option<Value>,
    pub value: Option<Value>,
}

pub fn invalidate_canonical_field_document(
    corruptions: CanonicalFieldDocumentCorruptions,
) -> Table {
    let mut table = Table::try_from(build_canonical_field_document(Default::default()))
        .expect("a built document renders as a table");
    if let Some(kind) = corruptions.kind {
        table.insert("kind".to_string(), kind);
    }
    if let Some(value) = corruptions.value {
        table.insert("value".to_string(), value);
    }
    table
}

#[derive(Default)]
pub struct NetworkDocumentOverrides {
    pub precompile_encodings: Option<Vec<String>>,
    pub chain: Option<CanonicalFieldDocument>,
    pub entitlement_contract: Option<CanonicalFieldDocument>,
}

pub fn build_network_document(overrides: NetworkDocumentOverrides) -> NetworkDocument {
    NetworkDocument {
        precompile_encodings: overrides
            .precompile_encodings
            .unwrap_or_else(|| vec!["eip196_eip197".to_string()]),
        chain: overrides.chain.unwrap_or_else(|| {
            build_canonical_field_document(CanonicalFieldDocumentOverrides {
                kind: Some("unsigned256".to_string()),
                value: Some(CanonicalFieldDocumentValue::Integer(84532)),
            })
        }),
        entitlement_contract: overrides.entitlement_contract.unwrap_or_else(|| {
            build_canonical_field_document(CanonicalFieldDocumentOverrides {
                kind: Some("fixed_bytes20".to_string()),
                value: Some(CanonicalFieldDocumentValue::Text(
                    "0x0000000000000000000000000000000000000000".to_string(),
                )),
            })
        }),
    }
}

#[derive(Default)]
pub struct NetworkDocumentCorruptions {
    pub precompile_encodings: Option<Value>,
    pub chain: Option<Value>,
    pub entitlement_contract: Option<Value>,
}

pub fn invalidate_network_document(corruptions: NetworkDocumentCorruptions) -> Table {
    let mut table = Table::try_from(build_network_document(Default::default()))
        .expect("a built document renders as a table");
    if let Some(precompile_encodings) = corruptions.precompile_encodings {
        table.insert("precompile_encodings".to_string(), precompile_encodings);
    }
    if let Some(chain) = corruptions.chain {
        table.insert("chain".to_string(), chain);
    }
    if let Some(entitlement_contract) = corruptions.entitlement_contract {
        table.insert("entitlement_contract".to_string(), entitlement_contract);
    }
    table
}

#[derive(Default)]
pub struct BenchmarkDocumentOverrides {
    pub iterations: Option<u32>,
    pub concretes: Option<Vec<CurveDocument>>,
}

pub fn build_benchmark_document(overrides: BenchmarkDocumentOverrides) -> BenchmarkDocument {
    BenchmarkDocument {
        iterations: overrides.iterations.unwrap_or(2),
        concretes: overrides
            .concretes
            .unwrap_or_else(|| vec![build_curve_document(Default::default())]),
    }
}

#[derive(Default)]
pub struct BenchmarkDocumentCorruptions {
    pub iterations: Option<Value>,
    pub concretes: Option<Value>,
}

pub fn invalidate_benchmark_document(corruptions: BenchmarkDocumentCorruptions) -> Table {
    let mut table = Table::try_from(build_benchmark_document(Default::default()))
        .expect("a built document renders as a table");
    if let Some(iterations) = corruptions.iterations {
        table.insert("iterations".to_string(), iterations);
    }
    if let Some(concretes) = corruptions.concretes {
        table.insert("concretes".to_string(), concretes);
    }
    table
}

#[derive(Default)]
pub struct DeliveryContextDocumentOverrides {
    pub suite_identifier: Option<String>,
    pub suite_version: Option<u16>,
    pub asset_identity_hash: Option<String>,
    pub parameter_set_identifier: Option<String>,
    pub source_entitlement: Option<CanonicalFieldDocument>,
    pub target_entitlement: Option<CanonicalFieldDocument>,
    pub old_interval: Option<CanonicalFieldDocument>,
    pub new_interval: Option<CanonicalFieldDocument>,
    pub seller: Option<CanonicalFieldDocument>,
    pub buyer: Option<CanonicalFieldDocument>,
    pub expiry: Option<u64>,
}

pub fn build_delivery_context_document(
    overrides: DeliveryContextDocumentOverrides,
) -> DeliveryContextDocument {
    DeliveryContextDocument {
        suite_identifier: overrides.suite_identifier.unwrap_or_else(|| {
            "0x0000000000000000000000000000000000000000000000000000000000000001".to_string()
        }),
        suite_version: overrides.suite_version.unwrap_or(1),
        asset_identity_hash: overrides.asset_identity_hash.unwrap_or_else(|| {
            "0x0000000000000000000000000000000000000000000000000000000000000001".to_string()
        }),
        parameter_set_identifier: overrides.parameter_set_identifier.unwrap_or_else(|| {
            "0x0000000000000000000000000000000000000000000000000000000000000001".to_string()
        }),
        source_entitlement: overrides.source_entitlement.unwrap_or_else(|| {
            build_canonical_field_document(CanonicalFieldDocumentOverrides {
                kind: Some("unsigned256".to_string()),
                value: Some(CanonicalFieldDocumentValue::Integer(1)),
            })
        }),
        target_entitlement: overrides.target_entitlement.unwrap_or_else(|| {
            build_canonical_field_document(CanonicalFieldDocumentOverrides {
                kind: Some("unsigned256".to_string()),
                value: Some(CanonicalFieldDocumentValue::Integer(1)),
            })
        }),
        old_interval: overrides.old_interval.unwrap_or_else(|| {
            build_canonical_field_document(CanonicalFieldDocumentOverrides {
                kind: Some("unsigned64".to_string()),
                value: Some(CanonicalFieldDocumentValue::Integer(0)),
            })
        }),
        new_interval: overrides.new_interval.unwrap_or_else(|| {
            build_canonical_field_document(CanonicalFieldDocumentOverrides {
                kind: Some("unsigned64".to_string()),
                value: Some(CanonicalFieldDocumentValue::Integer(1)),
            })
        }),
        seller: overrides.seller.unwrap_or_else(|| {
            build_canonical_field_document(CanonicalFieldDocumentOverrides {
                kind: Some("fixed_bytes20".to_string()),
                value: Some(CanonicalFieldDocumentValue::Text(
                    "0x0000000000000000000000000000000000000001".to_string(),
                )),
            })
        }),
        buyer: overrides.buyer.unwrap_or_else(|| {
            build_canonical_field_document(CanonicalFieldDocumentOverrides {
                kind: Some("fixed_bytes20".to_string()),
                value: Some(CanonicalFieldDocumentValue::Text(
                    "0x0000000000000000000000000000000000000002".to_string(),
                )),
            })
        }),
        expiry: overrides.expiry.unwrap_or(4_102_444_800),
    }
}

#[derive(Default)]
pub struct DeliveryContextDocumentCorruptions {
    pub suite_identifier: Option<Value>,
    pub suite_version: Option<Value>,
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

pub fn invalidate_delivery_context_document(
    corruptions: DeliveryContextDocumentCorruptions,
) -> Table {
    let mut table = Table::try_from(build_delivery_context_document(Default::default()))
        .expect("a built document renders as a table");
    if let Some(suite_identifier) = corruptions.suite_identifier {
        table.insert("suite_identifier".to_string(), suite_identifier);
    }
    if let Some(suite_version) = corruptions.suite_version {
        table.insert("suite_version".to_string(), suite_version);
    }
    if let Some(asset_identity_hash) = corruptions.asset_identity_hash {
        table.insert("asset_identity_hash".to_string(), asset_identity_hash);
    }
    if let Some(parameter_set_identifier) = corruptions.parameter_set_identifier {
        table.insert(
            "parameter_set_identifier".to_string(),
            parameter_set_identifier,
        );
    }
    if let Some(source_entitlement) = corruptions.source_entitlement {
        table.insert("source_entitlement".to_string(), source_entitlement);
    }
    if let Some(target_entitlement) = corruptions.target_entitlement {
        table.insert("target_entitlement".to_string(), target_entitlement);
    }
    if let Some(old_interval) = corruptions.old_interval {
        table.insert("old_interval".to_string(), old_interval);
    }
    if let Some(new_interval) = corruptions.new_interval {
        table.insert("new_interval".to_string(), new_interval);
    }
    if let Some(seller) = corruptions.seller {
        table.insert("seller".to_string(), seller);
    }
    if let Some(buyer) = corruptions.buyer {
        table.insert("buyer".to_string(), buyer);
    }
    if let Some(expiry) = corruptions.expiry {
        table.insert("expiry".to_string(), expiry);
    }
    table
}

#[derive(Default)]
pub struct StatementsDocumentOverrides {
    pub mint: Option<DeliveryContextDocument>,
    pub transfer: Option<DeliveryContextDocument>,
    pub mint_purpose: Option<u16>,
    pub transfer_purpose: Option<u16>,
    pub identities: Option<Vec<String>>,
    pub proof_identity: Option<String>,
}

pub fn build_statements_document(overrides: StatementsDocumentOverrides) -> StatementsDocument {
    StatementsDocument {
        mint: overrides
            .mint
            .unwrap_or_else(|| build_delivery_context_document(Default::default())),
        transfer: overrides
            .transfer
            .unwrap_or_else(|| build_delivery_context_document(Default::default())),
        mint_purpose: overrides.mint_purpose.unwrap_or(1),
        transfer_purpose: overrides.transfer_purpose.unwrap_or(2),
        identities: overrides
            .identities
            .unwrap_or_else(|| vec!["0x01".to_string()]),
        proof_identity: overrides
            .proof_identity
            .unwrap_or_else(|| "0x01".to_string()),
    }
}

#[derive(Default)]
pub struct StatementsDocumentCorruptions {
    pub mint: Option<Value>,
    pub transfer: Option<Value>,
    pub mint_purpose: Option<Value>,
    pub transfer_purpose: Option<Value>,
    pub identities: Option<Value>,
    pub proof_identity: Option<Value>,
}

pub fn invalidate_statements_document(corruptions: StatementsDocumentCorruptions) -> Table {
    let mut table = Table::try_from(build_statements_document(Default::default()))
        .expect("a built document renders as a table");
    if let Some(mint) = corruptions.mint {
        table.insert("mint".to_string(), mint);
    }
    if let Some(transfer) = corruptions.transfer {
        table.insert("transfer".to_string(), transfer);
    }
    if let Some(mint_purpose) = corruptions.mint_purpose {
        table.insert("mint_purpose".to_string(), mint_purpose);
    }
    if let Some(transfer_purpose) = corruptions.transfer_purpose {
        table.insert("transfer_purpose".to_string(), transfer_purpose);
    }
    if let Some(identities) = corruptions.identities {
        table.insert("identities".to_string(), identities);
    }
    if let Some(proof_identity) = corruptions.proof_identity {
        table.insert("proof_identity".to_string(), proof_identity);
    }
    table
}

#[derive(Default)]
pub struct VectorKindDocumentOverrides {
    pub kind: Option<String>,
    pub count: Option<Option<u32>>,
    pub terms: Option<Option<u32>>,
    pub message_length: Option<Option<usize>>,
    pub file_name: Option<String>,
    pub struct_name: Option<String>,
    pub decode_function_name: Option<String>,
    pub path_constant_name: Option<String>,
}

pub fn build_vector_kind_document(overrides: VectorKindDocumentOverrides) -> VectorKindDocument {
    VectorKindDocument {
        kind: overrides.kind.unwrap_or_else(|| "g1_addition".to_string()),
        count: overrides.count.unwrap_or(Some(2)),
        terms: overrides.terms.unwrap_or(None),
        message_length: overrides.message_length.unwrap_or(None),
        file_name: overrides
            .file_name
            .unwrap_or_else(|| "g1_add.vectors".to_string()),
        struct_name: overrides
            .struct_name
            .unwrap_or_else(|| "G1AddVector".to_string()),
        decode_function_name: overrides
            .decode_function_name
            .unwrap_or_else(|| "decodeG1Add".to_string()),
        path_constant_name: overrides
            .path_constant_name
            .unwrap_or_else(|| "G1_ADD_PATH".to_string()),
    }
}

#[derive(Default)]
pub struct VectorKindDocumentCorruptions {
    pub kind: Option<Value>,
    pub count: Option<Value>,
    pub terms: Option<Value>,
    pub message_length: Option<Value>,
    pub file_name: Option<Value>,
    pub struct_name: Option<Value>,
    pub decode_function_name: Option<Value>,
    pub path_constant_name: Option<Value>,
}

pub fn invalidate_vector_kind_document(corruptions: VectorKindDocumentCorruptions) -> Table {
    let mut table = Table::try_from(build_vector_kind_document(Default::default()))
        .expect("a built document renders as a table");
    if let Some(kind) = corruptions.kind {
        table.insert("kind".to_string(), kind);
    }
    if let Some(count) = corruptions.count {
        table.insert("count".to_string(), count);
    }
    if let Some(terms) = corruptions.terms {
        table.insert("terms".to_string(), terms);
    }
    if let Some(message_length) = corruptions.message_length {
        table.insert("message_length".to_string(), message_length);
    }
    if let Some(file_name) = corruptions.file_name {
        table.insert("file_name".to_string(), file_name);
    }
    if let Some(struct_name) = corruptions.struct_name {
        table.insert("struct_name".to_string(), struct_name);
    }
    if let Some(decode_function_name) = corruptions.decode_function_name {
        table.insert("decode_function_name".to_string(), decode_function_name);
    }
    if let Some(path_constant_name) = corruptions.path_constant_name {
        table.insert("path_constant_name".to_string(), path_constant_name);
    }
    table
}

#[derive(Default)]
pub struct LibraryDocumentOverrides {
    pub file_name: Option<String>,
    pub library_name: Option<String>,
}

pub fn build_library_document(overrides: LibraryDocumentOverrides) -> LibraryDocument {
    LibraryDocument {
        file_name: overrides
            .file_name
            .unwrap_or_else(|| "Constants.sol".to_string()),
        library_name: overrides
            .library_name
            .unwrap_or_else(|| "Constants".to_string()),
    }
}

#[derive(Default)]
pub struct LibraryDocumentCorruptions {
    pub file_name: Option<Value>,
    pub library_name: Option<Value>,
}

pub fn invalidate_library_document(corruptions: LibraryDocumentCorruptions) -> Table {
    let mut table = Table::try_from(build_library_document(Default::default()))
        .expect("a built document renders as a table");
    if let Some(file_name) = corruptions.file_name {
        table.insert("file_name".to_string(), file_name);
    }
    if let Some(library_name) = corruptions.library_name {
        table.insert("library_name".to_string(), library_name);
    }
    table
}

#[derive(Default)]
pub struct CurveDirectoriesDocumentOverrides {
    pub bn254: Option<String>,
    pub bls12_381: Option<String>,
}

pub fn build_curve_directories_document(
    overrides: CurveDirectoriesDocumentOverrides,
) -> CurveDirectoriesDocument {
    CurveDirectoriesDocument {
        bn254: overrides.bn254.unwrap_or_else(|| "bn254".to_string()),
        bls12_381: overrides
            .bls12_381
            .unwrap_or_else(|| "bls12_381".to_string()),
    }
}

#[derive(Default)]
pub struct CurveDirectoriesDocumentCorruptions {
    pub bn254: Option<Value>,
    pub bls12_381: Option<Value>,
}

pub fn invalidate_curve_directories_document(
    corruptions: CurveDirectoriesDocumentCorruptions,
) -> Table {
    let mut table = Table::try_from(build_curve_directories_document(Default::default()))
        .expect("a built document renders as a table");
    if let Some(bn254) = corruptions.bn254 {
        table.insert("bn254".to_string(), bn254);
    }
    if let Some(bls12_381) = corruptions.bls12_381 {
        table.insert("bls12_381".to_string(), bls12_381);
    }
    table
}

#[derive(Default)]
pub struct CompilerVersionDocumentOverrides {
    pub major: Option<u16>,
    pub minor: Option<u16>,
    pub patch: Option<u16>,
}

pub fn build_compiler_version_document(
    overrides: CompilerVersionDocumentOverrides,
) -> CompilerVersionDocument {
    CompilerVersionDocument {
        major: overrides.major.unwrap_or(0),
        minor: overrides.minor.unwrap_or(8),
        patch: overrides.patch.unwrap_or(37),
    }
}

#[derive(Default)]
pub struct CompilerVersionDocumentCorruptions {
    pub major: Option<Value>,
    pub minor: Option<Value>,
    pub patch: Option<Value>,
}

pub fn invalidate_compiler_version_document(
    corruptions: CompilerVersionDocumentCorruptions,
) -> Table {
    let mut table = Table::try_from(build_compiler_version_document(Default::default()))
        .expect("a built document renders as a table");
    if let Some(major) = corruptions.major {
        table.insert("major".to_string(), major);
    }
    if let Some(minor) = corruptions.minor {
        table.insert("minor".to_string(), minor);
    }
    if let Some(patch) = corruptions.patch {
        table.insert("patch".to_string(), patch);
    }
    table
}

#[derive(Default)]
pub struct GenerateDocumentOverrides {
    pub concrete: Option<String>,
    pub kinds: Option<Vec<VectorKindDocument>>,
    pub constants_library: Option<LibraryDocument>,
    pub records_library: Option<LibraryDocument>,
    pub foundry_root: Option<PathBuf>,
    pub generated_directory: Option<Vec<String>>,
    pub curve_directories: Option<CurveDirectoriesDocument>,
    pub compiler_version: Option<CompilerVersionDocument>,
    pub license: Option<String>,
}

pub fn build_generate_document(overrides: GenerateDocumentOverrides) -> GenerateDocument {
    GenerateDocument {
        concrete: overrides.concrete.unwrap_or_else(|| "evm".to_string()),
        kinds: overrides
            .kinds
            .unwrap_or_else(|| vec![build_vector_kind_document(Default::default())]),
        constants_library: overrides
            .constants_library
            .unwrap_or_else(|| build_library_document(Default::default())),
        records_library: overrides.records_library.unwrap_or_else(|| {
            build_library_document(LibraryDocumentOverrides {
                file_name: Some("VectorRecords.sol".to_string()),
                library_name: Some("VectorRecords".to_string()),
            })
        }),
        foundry_root: overrides
            .foundry_root
            .unwrap_or_else(|| PathBuf::from("contracts/evm")),
        generated_directory: overrides
            .generated_directory
            .unwrap_or_else(|| vec!["generated".to_string()]),
        curve_directories: overrides
            .curve_directories
            .unwrap_or_else(|| build_curve_directories_document(Default::default())),
        compiler_version: overrides
            .compiler_version
            .unwrap_or_else(|| build_compiler_version_document(Default::default())),
        license: overrides
            .license
            .unwrap_or_else(|| "UNLICENSED".to_string()),
    }
}

#[derive(Default)]
pub struct GenerateDocumentCorruptions {
    pub concrete: Option<Value>,
    pub kinds: Option<Value>,
    pub constants_library: Option<Value>,
    pub records_library: Option<Value>,
    pub foundry_root: Option<Value>,
    pub generated_directory: Option<Value>,
    pub curve_directories: Option<Value>,
    pub compiler_version: Option<Value>,
    pub license: Option<Value>,
}

pub fn invalidate_generate_document(corruptions: GenerateDocumentCorruptions) -> Table {
    let mut table = Table::try_from(build_generate_document(Default::default()))
        .expect("a built document renders as a table");
    if let Some(concrete) = corruptions.concrete {
        table.insert("concrete".to_string(), concrete);
    }
    if let Some(kinds) = corruptions.kinds {
        table.insert("kinds".to_string(), kinds);
    }
    if let Some(constants_library) = corruptions.constants_library {
        table.insert("constants_library".to_string(), constants_library);
    }
    if let Some(records_library) = corruptions.records_library {
        table.insert("records_library".to_string(), records_library);
    }
    if let Some(foundry_root) = corruptions.foundry_root {
        table.insert("foundry_root".to_string(), foundry_root);
    }
    if let Some(generated_directory) = corruptions.generated_directory {
        table.insert("generated_directory".to_string(), generated_directory);
    }
    if let Some(curve_directories) = corruptions.curve_directories {
        table.insert("curve_directories".to_string(), curve_directories);
    }
    if let Some(compiler_version) = corruptions.compiler_version {
        table.insert("compiler_version".to_string(), compiler_version);
    }
    if let Some(license) = corruptions.license {
        table.insert("license".to_string(), license);
    }
    table
}

#[derive(Default)]
pub struct OutputDestinationDocumentOverrides {
    pub kind: Option<String>,
    pub path: Option<Option<PathBuf>>,
}

pub fn build_output_destination_document(
    overrides: OutputDestinationDocumentOverrides,
) -> OutputDestinationDocument {
    OutputDestinationDocument {
        kind: overrides
            .kind
            .unwrap_or_else(|| "standard_output".to_string()),
        path: overrides.path.unwrap_or(None),
    }
}

#[derive(Default)]
pub struct OutputDestinationDocumentCorruptions {
    pub kind: Option<Value>,
    pub path: Option<Value>,
}

pub fn invalidate_output_destination_document(
    corruptions: OutputDestinationDocumentCorruptions,
) -> Table {
    let mut table = Table::try_from(build_output_destination_document(Default::default()))
        .expect("a built document renders as a table");
    if let Some(kind) = corruptions.kind {
        table.insert("kind".to_string(), kind);
    }
    if let Some(path) = corruptions.path {
        table.insert("path".to_string(), path);
    }
    table
}

#[derive(Default)]
pub struct OutputDocumentOverrides {
    pub destinations: Option<Vec<OutputDestinationDocument>>,
}

pub fn build_output_document(overrides: OutputDocumentOverrides) -> OutputDocument {
    OutputDocument {
        destinations: overrides
            .destinations
            .unwrap_or_else(|| vec![build_output_destination_document(Default::default())]),
    }
}

#[derive(Default)]
pub struct OutputDocumentCorruptions {
    pub destinations: Option<Value>,
}

pub fn invalidate_output_document(corruptions: OutputDocumentCorruptions) -> Table {
    let mut table = Table::try_from(build_output_document(Default::default()))
        .expect("a built document renders as a table");
    if let Some(destinations) = corruptions.destinations {
        table.insert("destinations".to_string(), destinations);
    }
    table
}

#[derive(Default)]
pub struct HarnessConfigDocumentOverrides {
    pub random: Option<RandomDocument>,
    pub encoding: Option<SelectionDocument>,
    pub hash_to_scalar: Option<SelectionDocument>,
    pub chain_forms: Option<SelectionDocument>,
    pub kem: Option<KemDocument>,
    pub key_agreement: Option<KeyAgreementDocument>,
    pub delivery_proof: Option<DeliveryProofDocument>,
    pub curves: Option<Vec<CurveDocument>>,
    pub network: Option<NetworkDocument>,
    pub benchmark: Option<BenchmarkDocument>,
    pub statements: Option<StatementsDocument>,
    pub generate: Option<GenerateDocument>,
    pub output: Option<OutputDocument>,
}

pub fn build_harness_config_document(
    overrides: HarnessConfigDocumentOverrides,
) -> HarnessConfigDocument {
    HarnessConfigDocument {
        random: overrides
            .random
            .unwrap_or_else(|| build_random_document(Default::default())),
        encoding: overrides
            .encoding
            .unwrap_or_else(|| build_selection_document(Default::default())),
        hash_to_scalar: overrides.hash_to_scalar.unwrap_or_else(|| {
            build_selection_document(SelectionDocumentOverrides {
                concrete: Some("keccak256".to_string()),
                identifier: Some("keccak256_v1".to_string()),
            })
        }),
        chain_forms: overrides.chain_forms.unwrap_or_else(|| {
            build_selection_document(SelectionDocumentOverrides {
                concrete: Some("evm".to_string()),
                identifier: Some("evm_v1".to_string()),
            })
        }),
        kem: overrides
            .kem
            .unwrap_or_else(|| build_kem_document(Default::default())),
        key_agreement: overrides
            .key_agreement
            .unwrap_or_else(|| build_key_agreement_document(Default::default())),
        delivery_proof: overrides
            .delivery_proof
            .unwrap_or_else(|| build_delivery_proof_document(Default::default())),
        curves: overrides
            .curves
            .unwrap_or_else(|| vec![build_curve_document(Default::default())]),
        network: overrides
            .network
            .unwrap_or_else(|| build_network_document(Default::default())),
        benchmark: overrides
            .benchmark
            .unwrap_or_else(|| build_benchmark_document(Default::default())),
        statements: overrides
            .statements
            .unwrap_or_else(|| build_statements_document(Default::default())),
        generate: overrides
            .generate
            .unwrap_or_else(|| build_generate_document(Default::default())),
        output: overrides
            .output
            .unwrap_or_else(|| build_output_document(Default::default())),
    }
}

#[derive(Default)]
pub struct HarnessConfigDocumentCorruptions {
    pub random: Option<Value>,
    pub encoding: Option<Value>,
    pub hash_to_scalar: Option<Value>,
    pub chain_forms: Option<Value>,
    pub kem: Option<Value>,
    pub key_agreement: Option<Value>,
    pub delivery_proof: Option<Value>,
    pub curves: Option<Value>,
    pub network: Option<Value>,
    pub benchmark: Option<Value>,
    pub statements: Option<Value>,
    pub generate: Option<Value>,
    pub output: Option<Value>,
}

pub fn invalidate_harness_config_document(corruptions: HarnessConfigDocumentCorruptions) -> Table {
    let mut table = Table::try_from(build_harness_config_document(Default::default()))
        .expect("a built document renders as a table");
    if let Some(random) = corruptions.random {
        table.insert("random".to_string(), random);
    }
    if let Some(encoding) = corruptions.encoding {
        table.insert("encoding".to_string(), encoding);
    }
    if let Some(hash_to_scalar) = corruptions.hash_to_scalar {
        table.insert("hash_to_scalar".to_string(), hash_to_scalar);
    }
    if let Some(chain_forms) = corruptions.chain_forms {
        table.insert("chain_forms".to_string(), chain_forms);
    }
    if let Some(kem) = corruptions.kem {
        table.insert("kem".to_string(), kem);
    }
    if let Some(key_agreement) = corruptions.key_agreement {
        table.insert("key_agreement".to_string(), key_agreement);
    }
    if let Some(delivery_proof) = corruptions.delivery_proof {
        table.insert("delivery_proof".to_string(), delivery_proof);
    }
    if let Some(curves) = corruptions.curves {
        table.insert("curves".to_string(), curves);
    }
    if let Some(network) = corruptions.network {
        table.insert("network".to_string(), network);
    }
    if let Some(benchmark) = corruptions.benchmark {
        table.insert("benchmark".to_string(), benchmark);
    }
    if let Some(statements) = corruptions.statements {
        table.insert("statements".to_string(), statements);
    }
    if let Some(generate) = corruptions.generate {
        table.insert("generate".to_string(), generate);
    }
    if let Some(output) = corruptions.output {
        table.insert("output".to_string(), output);
    }
    table
}

#[derive(Default)]
pub struct LayersDepsOverrides;

pub fn build_layers_deps(_overrides: LayersDepsOverrides) -> LayersDeps {
    LayersDeps
}

#[derive(Default)]
pub struct LayersParamsOverrides;

pub fn build_layers_params(_overrides: LayersParamsOverrides) -> LayersParams {
    LayersParams
}

#[derive(Default)]
pub struct LayersPayloadOverrides {
    pub arguments: Option<Vec<OsString>>,
}

pub fn build_layers_payload<'a>(
    defaults: &'a str,
    overrides: LayersPayloadOverrides,
) -> LayersPayload<'a> {
    LayersPayload {
        defaults,
        arguments: overrides
            .arguments
            .unwrap_or_else(|| vec![OsString::from("harness-crypto")]),
    }
}

#[derive(Default)]
pub struct LayersSuccessReturnOverrides {
    pub document: Option<HarnessConfigDocument>,
}

pub fn build_layers_success_return(overrides: LayersSuccessReturnOverrides) -> LayersSuccessReturn {
    LayersSuccessReturn {
        document: overrides
            .document
            .unwrap_or_else(|| build_harness_config_document(Default::default())),
    }
}

pub fn mock_layers(
    _deps: &LayersDeps,
    _params: LayersParams,
    _payload: LayersPayload<'_>,
) -> LayersReturn {
    Ok(build_layers_success_return(Default::default()))
}
