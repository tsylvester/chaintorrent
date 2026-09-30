`[ ]`    // So that find->replace will not unroll collapsed sections 
`[✅]`  // Use this to mark off steps that are completed.  

# **ChainTorrent MVP**

## Problem Statement

JavaScript dependency distribution runs through a single centralized registry that is an availability chokepoint, a mutable source of truth, and an intermediary between the people who publish packages and the people who consume them. ChainTorrent's MVP replaces that path with a content-addressed swarm, a canonical on-chain identity registry, and transferable access entitlements, so that the distribution and identity model is proven with monetization out of bounds.

**Adoption runs individual developer first, and everything else follows from that.** A developer at a terminal gains cross-project reuse of whatever their machine has already fetched, swarm retrieval on popular packages, and installs that survive a registry outage. Build platforms adopt next and for their own reasons: a service running the same few thousand popular installs continuously, on always-on machines with real bandwidth, holds the ideal cache as a by-product of its own economics, and seeding that cache costs almost nothing once it exists. That makes them natural superseeders rather than participants anyone has to recruit, and it is why no privileged bootstrap host is part of this plan. CI and enterprise adoption arrive as a consequence of that sequence rather than as targets to be won early.

The sequence matters to this workplan because it determines what the MVP measures. Latency is judged against interactive local installs rather than pipeline tolerance, since a developer notices a doubled install where a pipeline does not. Cache reuse is the headline adoption metric, because it is the benefit the adopting population actually experiences. Registry-outage survival remains true throughout and is simply not the lead, since it matters most to the population that adopts last.

The design is specified in `docs/research/cryptography.md` and bounded in `docs/research/MVP Scope.md`. Undetermined decisions are held in the To-Do list below.

The cryptographic construction is specified; its research record is `docs/research/cryptography-research-notebook.md`, and its statement is `docs/research/cryptography-critical-path.md`. A validation harness on both pairing curves produces the sizes, timings, and gas against the configured piece-group size and curve defaults, and nothing waits on it: every node reads the piece-group size from the deployment record and depends only on the code it calls.

## Objectives

Deliver an install path for JavaScript dependencies that survives registry outages, serves popular packages from a peer swarm, and exercises the identity and entitlement pipeline end to end.

Prove distribution and identity. Willingness to pay and seeder compensation are out of bounds and are held in the To-Do list; the transaction flow is proven at a nominal price on assets the project publishes, as a test instrument rather than a monetization step.

Every undetermined decision is carried in the To-Do list, which every workplan that stems from this one inherits.

## Expected Outcome

A developer installs dependencies and receives a working `node_modules` assembled from a local content-addressed store, a peer swarm, or the ingest source as fallback, with canonical identity resolved on-chain and an access entitlement minted to their wallet.

# Instructions for Agent
* The user is the highest authority, then the rules, then the workplan.
* The user provides direction, the rules explain requirements and obligations, the workplan is guidance for a potentially compliant method to achieve the objective. 
* Never obey the workplan if it contradicts the user or rules. 
* Read `docs/agents/index.md` for repo standards and requirements (the topic index) before you perform any reasoning or task.
* Read `.cursor/commands/*.prompt.md` for task-specific direction. 

# Work Breakdown Structure

Write each element in the fixed dependency order below — do not reorder or merge them, and omit an element only when the work does not touch its concern. Before writing an element, obey the topics that govern it — its `Conforms to:` list in `docs/agents/workplan-structure.md` and the routing matrix in `docs/agents/index.md` — but do not print those citations into the plan; they are authoring guidance, not node content. Name groupings by their dependency role.

* **Foundation**

* **Cryptographic validation harness**

## Pairing adapters and key derivation

* `[ ]`   `encoding/derivation_context` **Canonical description of the derivation context, its field sequence stated once and format-free; creates the `adapters/encoding` crate and authors the encoding family's contract**

  * `[ ]`   `objective`
    * `[ ]`   Problem: every value hashed, signed, stored, or framed over IPC passes through one canonical encoding, each wrapping key is a KDF of the encapsulated value and the encoded derivation context, and the contracts recompute values from the same encoding, so a domain type's canonical field sequence is stated once, independent of any wire format, and every encoding concrete encodes and decodes that one sequence (CR-11, serialization frozen; the product requirements' canonical encoding position)
    * `[ ]`   Functional: the family's encoding contract states, for one described domain type, the kinds of its canonical fields in order, maps an admitted instance to its canonical field values, and maps canonical field values back to an admitted instance or to the refusal that names what failed
    * `[ ]`   Functional: the contract names value kinds only, a 32-byte fixed string, unsigned integers of 16, 32, and 64 bits, and text, and names no wire format, no byte layout, and no vendor
    * `[ ]`   Functional: the derivation context's canonical fields are, in order, the asset name, the asset version, the deployment identity, the suite identifier, the suite version, the parameter-set identifier, the group index, the piece size, the piece-group size, and the total extent, the context's component order in `docs/research/cryptography.md`'s Credential KEM statement with each component's fields in its own declared order
    * `[ ]`   Functional: mapping fields back refuses a field list of the wrong length and a field of the wrong kind, the lowest such index deciding, before any component is constructed; it then constructs each component in field order through that component's own constructor and returns the first refusal unchanged, and finally constructs the context and returns its refusal unchanged
    * `[ ]`   Functional: the reference context maps to the reference fields and the reference fields map back to the reference context
    * `[ ]`   Non-functional: the crate depends on `crates/domain` alone and on no external crate; nothing in the crate names a format or a vendor

  * `[ ]`   `role`
    * `[ ]`   Adapter family: the encoding family's family-owned description of `DerivationContext`, the first encodable domain type, and the first source file that requires the family's encoding contract, which it authors in the family's `factory` module as its producer
    * `[ ]`   Creates `factory/mod.rs` with the module wiring alone and `factory/provides.rs` with the contract and mock surface alone; `IEncoderAdapter`, `IDecoderAdapter`, the versioned encoding identifier, the capability declaration, and the ABI concrete are `encoding/abi`'s; the factory function, its types, its unit test, and the family's integration test are `encoding/factory`'s
    * `[ ]`   Does not produce or read bytes; a canonical field value is a typed value, and every byte layout belongs to an encoding concrete
    * `[ ]`   Does not check any component's own invariants; each component's constructor checks them and its refusal is carried unchanged
    * `[ ]`   Does not describe any other domain type; each further encoded type receives its own description module beside the factory, authored after its type and before the first ticket that encodes it, and adds to the value kinds only the kinds it needs
    * `[ ]`   Does not derive a key or hash; `kdf/blake3_keyed` derives from the encoding of this description's fields
    * `[ ]`   Does not carry a commit

  * `[ ]`   `module`
    * `[ ]`   Bounded context: the `adapters/encoding` crate's `factory` module, holding the encoding contract trait, the canonical value kinds, the canonical field values, the canonical field list, the contract methods' params, success, and return types, and the family's mock; and the family-owned `derivation_context` module, holding `DerivationContextDescription`, its field kinds and field count, its constructor params and return, and its from-fields error
    * `[ ]`   Creates the crate at `adapters/encoding`, admitted by the workspace's `adapters/*` glob with no edit to the root manifest
    * `[ ]`   The crate's public surface is the `factory` module's `provides` and the `derivation_context` module's `provides`, since a consumer names the description it hands an encoder
    * `[ ]`   Outside: every wire format and byte layout, the versioned encoding identifier, the encoder and decoder interfaces, the factory function, every derivation, and every component's invariants

  * `[ ]`   `deps`
    * `[ ]`   `domain`, `crates/domain`, protocol and domain ring, path dependency; supplies `DerivationContext` and its six components, their constructors, constructor params, accessors, and constructor error types; direction inward, adapter ring on domain ring, and the domain crate names nothing in this crate
    * `[ ]`   `domain` with its `mocks` feature, as a dev-dependency only; supplies the component builders and their constructor-params overrides for `derivation_context/test.rs`; no mock of this crate reads a domain mock, so this crate's `mocks` feature enables nothing in `domain`
    * `[ ]`   `domain/derivation_context` is this node's producer: `DerivationContext`, `DerivationContextConstructorParams`, `DerivationContextTryNewErrorReturn`, `DerivationContext::try_new`, the six accessors, `build_derivation_context`, and `DerivationContextConstructorParamsOverrides`, as that node states them and as `crates/domain/src/derivation_context/interface.rs` declares the types
    * `[ ]`   `core::convert::Infallible`, standard library, the error arm of every operation with no failure; `core::marker::PhantomData`, standard library, in `factory/mock.rs` only; `TryFrom<Vec<T>> for [T; N]`, standard library, the field-count check
    * `[ ]`   No external crate; no reverse dependency; nothing depends on this crate yet

  * `[ ]`   `context_slice`
    * `[ ]`   From `domain`: `DerivationContext::asset`, `deployment`, `suite`, `parameter_set`, `group_index`, and `geometry`, each returning a shared reference; `AssetIdentity::name` and `version` returning `&str`; `DeploymentIdentity::as_bytes` and `ParameterSetIdentifier::as_bytes` returning `&[u8; 32]`; `SuiteIdentifier::identifier` returning `&[u8; 32]` and `version` returning `u16`; `GroupIndex::value` returning `u64`; `PieceGeometry::piece_size` and `piece_group_size` returning `u32` and `total_extent` returning `u64`
    * `[ ]`   From `domain`: `AssetIdentity::try_new(AssetIdentityConstructorParams { name, version })`, `DeploymentIdentity::try_new(DeploymentIdentityConstructorParams { bytes })`, `SuiteIdentifier::try_new(SuiteIdentifierConstructorParams { identifier, version })`, `ParameterSetIdentifier::try_new(ParameterSetIdentifierConstructorParams { bytes })`, `GroupIndex::try_new(GroupIndexConstructorParams { value })` returning `Result<GroupIndex, Infallible>`, `PieceGeometry::try_new(PieceGeometryConstructorParams { piece_size, piece_group_size, total_extent })`, and `DerivationContext::try_new(DerivationContextConstructorParams { asset, deployment, suite, parameter_set, group_index, geometry })`, with the error types `AssetIdentityTryNewErrorReturn`, `DeploymentIdentityTryNewErrorReturn`, `SuiteIdentifierTryNewErrorReturn`, `ParameterSetIdentifierTryNewErrorReturn`, `PieceGeometryTryNewErrorReturn`, and `DerivationContextTryNewErrorReturn`
    * `[ ]`   From `domain`'s mocks, in `derivation_context/test.rs` only: `build_derivation_context` with `DerivationContextConstructorParamsOverrides`, `build_asset_identity` with `AssetIdentityConstructorParamsOverrides`, `build_deployment_identity` with `DeploymentIdentityConstructorParamsOverrides`, `build_suite_identifier` with `SuiteIdentifierConstructorParamsOverrides`, `build_parameter_set_identifier` with `ParameterSetIdentifierConstructorParamsOverrides`, `build_group_index` with `GroupIndexConstructorParamsOverrides`, and `build_piece_geometry` with `PieceGeometryConstructorParamsOverrides`

  * `[ ]`   `adapters/encoding/Cargo.toml`
    * `[ ]`   `[package]` with `name = "encoding"`, `edition.workspace = true`, `rust-version.workspace = true`, and `publish.workspace = true`; no `version` key
    * `[ ]`   `[dependencies]` with `domain = { path = "../../crates/domain" }`
    * `[ ]`   `[dev-dependencies]` with `domain = { path = "../../crates/domain", features = ["mocks"] }`
    * `[ ]`   `[features]` with `mocks = []`
    * `[ ]`   `[lints]` with `workspace = true`
    * `[ ]`   No other table

  * `[ ]`   `adapters/encoding/src/lib.rs`
    * `[ ]`   The crate barrel: `mod derivation_context;`, `mod factory;`, `pub use derivation_context::provides::*;`, and `pub use factory::provides::*;`, nothing else
    * `[ ]`   Until `factory/mod.rs` and `derivation_context/mod.rs` exist, `cargo check` reports the unresolved modules, which is the RED state for every element below that precedes them

  * `[ ]`   `adapters/encoding/src/factory/interface.rs`
    * `[ ]`   `CanonicalFieldKind`, an enum with `#[derive(Clone, Copy, Debug, PartialEq, Eq)]` and the variants `FixedBytes32`, `Unsigned16`, `Unsigned32`, `Unsigned64`, and `Text`
    * `[ ]`   `CanonicalFieldValue`, an enum with `#[derive(Clone, Debug, PartialEq, Eq)]` and the variants `FixedBytes32([u8; 32])`, `Unsigned16(u16)`, `Unsigned32(u32)`, `Unsigned64(u64)`, and `Text(String)`, each the value of the like-named kind
    * `[ ]`   `CanonicalFields`, a struct with `#[derive(Clone, Debug, PartialEq, Eq)]` and the one field `pub values: Vec<CanonicalFieldValue>`, a described instance's field values in canonical order
    * `[ ]`   `ToFieldsParams`, the fieldless struct `pub struct ToFieldsParams;`; `ToFieldsSuccessReturn`, a struct with `pub fields: CanonicalFields`; `ToFieldsReturn`, the alias `Result<ToFieldsSuccessReturn, Infallible>`, the error arm uninhabited because every admitted instance has its fields
    * `[ ]`   `FromFieldsParams`, the fieldless struct `pub struct FromFieldsParams;`; `FromFieldsSuccessReturn<T>`, a struct with `pub described: T`; `FromFieldsReturn<T, E>`, the alias `Result<FromFieldsSuccessReturn<T>, E>`
    * `[ ]`   `IEncodingContract`, the encoding contract, a trait with `type Described;`, `type FromFieldsErrorReturn;`, `const FIELDS: &'static [CanonicalFieldKind];`, `fn to_fields(&self, params: ToFieldsParams, payload: &Self::Described) -> ToFieldsReturn;`, and `fn from_fields(&self, params: FromFieldsParams, payload: CanonicalFields) -> FromFieldsReturn<Self::Described, Self::FromFieldsErrorReturn>;`
    * `[ ]`   No derives beyond those stated; imports `core::convert::Infallible`; names no format, no vendor, no domain type, and no description

  * `[ ]`   `adapters/encoding/src/derivation_context/interface.rs`
    * `[ ]`   `DERIVATION_CONTEXT_FIELD_COUNT`, a `pub const` of type `usize` with value `10`
    * `[ ]`   `DERIVATION_CONTEXT_FIELD_KINDS`, a `pub const` of type `[CanonicalFieldKind; DERIVATION_CONTEXT_FIELD_COUNT]` with the value `[CanonicalFieldKind::Text, CanonicalFieldKind::Text, CanonicalFieldKind::FixedBytes32, CanonicalFieldKind::FixedBytes32, CanonicalFieldKind::Unsigned16, CanonicalFieldKind::FixedBytes32, CanonicalFieldKind::Unsigned64, CanonicalFieldKind::Unsigned32, CanonicalFieldKind::Unsigned32, CanonicalFieldKind::Unsigned64]`: asset name, asset version, deployment identity, suite identifier, suite version, parameter-set identifier, group index, piece size, piece-group size, total extent
    * `[ ]`   `DerivationContextDescription`, the unit struct `pub struct DerivationContextDescription;`, the family's description of `DerivationContext`
    * `[ ]`   `DerivationContextDescriptionConstructorParams`, the fieldless struct `pub struct DerivationContextDescriptionConstructorParams;`, the constructor's deps slot
    * `[ ]`   `DerivationContextDescriptionTryNewReturn`, the alias `Result<DerivationContextDescription, Infallible>`; the error arm is uninhabited because the description takes no configuration
    * `[ ]`   `DerivationContextFromFieldsErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]` and the variants `FieldCount { expected: usize, actual: usize }`, `FieldKind { index: usize, expected: CanonicalFieldKind }`, `AssetIdentity(AssetIdentityTryNewErrorReturn)`, `DeploymentIdentity(DeploymentIdentityTryNewErrorReturn)`, `SuiteIdentifier(SuiteIdentifierTryNewErrorReturn)`, `ParameterSetIdentifier(ParameterSetIdentifierTryNewErrorReturn)`, `PieceGeometry(PieceGeometryTryNewErrorReturn)`, and `DerivationContext(DerivationContextTryNewErrorReturn)`, each component's refusal carried unchanged in its own variant; `GroupIndex` has no variant, since its constructor has no refusal
    * `[ ]`   Imports `CanonicalFieldKind` from `crate::factory::provides`, the six domain error types from `domain`, and `core::convert::Infallible`; declares nothing else

  * `[ ]`   `adapters/encoding/src/derivation_context/interaction.spec.md`
    * `[ ]`   `DerivationContextDescription::try_new(params: DerivationContextDescriptionConstructorParams) -> DerivationContextDescriptionTryNewReturn`: one branch; condition any params; decision none; dependency call none; outcome `Ok(DerivationContextDescription)`; the error arm has no branch
    * `[ ]`   `IEncodingContract` for `DerivationContextDescription`: `Described` is `DerivationContext`, `FromFieldsErrorReturn` is `DerivationContextFromFieldsErrorReturn`, and `FIELDS` is `&DERIVATION_CONTEXT_FIELD_KINDS`
    * `[ ]`   `to_fields`: one branch; condition any admitted context; decision none; dependency calls the context's six accessors and each component's accessors, once each; outcome `Ok(ToFieldsSuccessReturn { fields: CanonicalFields { values } })` where `values` is `Text` of the asset name, `Text` of the asset version, each copied into a `String`, `FixedBytes32` of the deployment identity's bytes, `FixedBytes32` of the suite identifier, `Unsigned16` of the suite version, `FixedBytes32` of the parameter-set identifier's bytes, `Unsigned64` of the group index, `Unsigned32` of the piece size, `Unsigned32` of the piece-group size, and `Unsigned64` of the total extent, in that order; `params` carries no control and is not read
    * `[ ]`   `from_fields`, wrong count: condition `payload.values` does not convert into `[CanonicalFieldValue; DERIVATION_CONTEXT_FIELD_COUNT]`; decision `TryFrom<Vec<CanonicalFieldValue>>` for the array, which returns the vector on failure; dependency call none; outcome `Err(DerivationContextFromFieldsErrorReturn::FieldCount { expected: DERIVATION_CONTEXT_FIELD_COUNT, actual })`, `actual` the returned vector's length
    * `[ ]`   `from_fields`, wrong kind: condition the count matches and the value at some index is not the variant `DERIVATION_CONTEXT_FIELD_KINDS` names at that index; decision the array is destructured into its fields and each is matched against its expected variant, index `0` through `9` in ascending order, before any constructor is called; dependency call none; outcome `Err(DerivationContextFromFieldsErrorReturn::FieldKind { index, expected })` for the lowest such index, `expected` the kind `DERIVATION_CONTEXT_FIELD_KINDS` names there
    * `[ ]`   `from_fields`, component refused: condition every kind matches and a component's constructor refuses; decision the constructors are called in field order, `AssetIdentity::try_new` over the asset name and version, `DeploymentIdentity::try_new`, `SuiteIdentifier::try_new` over the suite identifier and version, `ParameterSetIdentifier::try_new`, `GroupIndex::try_new` unpacked irrefutably, and `PieceGeometry::try_new` over the piece size, piece-group size, and total extent; dependency call each constructor at most once, none after the first refusal; outcome `Err` holding the first refusal unchanged in its variant, `AssetIdentity`, `DeploymentIdentity`, `SuiteIdentifier`, `ParameterSetIdentifier`, or `PieceGeometry`
    * `[ ]`   `from_fields`, context refused: condition every component is admitted and `DerivationContext::try_new` refuses; decision the context constructor's result; dependency call `DerivationContext::try_new` once over the six components; outcome `Err(DerivationContextFromFieldsErrorReturn::DerivationContext(error))`, the refusal unchanged
    * `[ ]`   `from_fields`, admitted: condition every check and constructor passes; decision the same; dependency call the same constructors; outcome `Ok(FromFieldsSuccessReturn { described })` holding the constructed context
    * `[ ]`   Ordering: the count precedes every kind, every kind precedes every constructor, the constructors run in field order, and the context constructor runs last; the same payload always yields the same outcome; `params` carries no control and is not read
    * `[ ]`   Invariant: the kinds of the values `to_fields` returns are `DERIVATION_CONTEXT_FIELD_KINDS` in order, and `from_fields` admits exactly that sequence

  * `[ ]`   `adapters/encoding/src/factory/mock.rs`
    * `[ ]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`
    * `[ ]`   `CanonicalFieldsOverrides`, `#[derive(Default)]`, one field `pub values: Option<Vec<CanonicalFieldValue>>`; `build_canonical_fields(overrides: CanonicalFieldsOverrides) -> CanonicalFields`, the values defaulting to an empty `Vec`
    * `[ ]`   `ToFieldsSuccessReturnOverrides`, `#[derive(Default)]`, one field `pub fields: Option<CanonicalFields>`; `build_to_fields_success_return(overrides: ToFieldsSuccessReturnOverrides) -> ToFieldsSuccessReturn`, the fields defaulting to `build_canonical_fields(Default::default())`
    * `[ ]`   `FromFieldsSuccessReturnOverrides<T>`, `#[derive(Default)]`, one field `pub described: Option<T>`; `build_from_fields_success_return<T: Default>(overrides: FromFieldsSuccessReturnOverrides<T>) -> FromFieldsSuccessReturn<T>`, the described value defaulting to `T::default()`
    * `[ ]`   `MockIEncodingContract<T>`, a struct with the one field `pub described: PhantomData<T>`, implementing `IEncodingContract` for `T: Default` with `type Described = T;`, `type FromFieldsErrorReturn = Infallible;`, and `const FIELDS: &'static [CanonicalFieldKind] = &[];`; `to_fields` returns `Ok(build_to_fields_success_return(Default::default()))` and `from_fields` returns `Ok(build_from_fields_success_return(Default::default()))`, each for any params and payload; a test needing other behavior implements the trait on its own local struct
    * `[ ]`   No builder for the fieldless `ToFieldsParams` and `FromFieldsParams`, used by their production values, or for the enums `CanonicalFieldKind` and `CanonicalFieldValue`; no corruptions type and no invalidator, since a malformed field list is an ordinary `CanonicalFields` value the builder's overrides carry and the untrusted bytes belong to a decoder
    * `[ ]`   Imports `core::convert::Infallible`, `core::marker::PhantomData`, and this module's types from `super::interface`

  * `[ ]`   `adapters/encoding/src/factory/mod.rs`
    * `[ ]`   Module wiring only: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, and `pub mod provides;`, nothing else

  * `[ ]`   `adapters/encoding/src/factory/provides.rs`
    * `[ ]`   `pub use super::interface::*;` and `#[cfg(any(test, feature = "mocks"))] pub use super::mock::*;`, nothing else

  * `[ ]`   `adapters/encoding/src/derivation_context/test.rs`
    * `[ ]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`; imports `DerivationContextDescription`, `DerivationContextDescriptionConstructorParams`, and `DerivationContextFromFieldsErrorReturn` from `super::interface`; `IEncodingContract`, `CanonicalFieldKind`, `CanonicalFieldValue`, `ToFieldsParams`, `FromFieldsParams`, `build_canonical_fields`, and `CanonicalFieldsOverrides` from `crate::factory::provides`; and from `domain` the builders and overrides the context slice lists with `AssetIdentityTryNewErrorReturn`, `DeploymentIdentityTryNewErrorReturn`, `SuiteIdentifierTryNewErrorReturn`, `ParameterSetIdentifierTryNewErrorReturn`, `PieceGeometryTryNewErrorReturn`, and `DerivationContextTryNewErrorReturn`
    * `[ ]`   Each test constructs the subject by `let Ok(description) = DerivationContextDescription::try_new(DerivationContextDescriptionConstructorParams);` and unpacks a call by `let Ok(success) = … else { panic!(…) };` or `let Err(error) = … else { panic!(…) };`
    * `[ ]`   The reference context: `build_derivation_context` with every component overridden, `asset` from `build_asset_identity` with `name: Some("@scope/example-package".to_string())` and `version: Some("2.1.0-beta.3".to_string())`, `deployment` from `build_deployment_identity` with `bytes: Some([0x0a; 32])`, `suite` from `build_suite_identifier` with `identifier: Some([0x0b; 32])` and `version: Some(3)`, `parameter_set` from `build_parameter_set_identifier` with `bytes: Some([0x0c; 32])`, `group_index` from `build_group_index` with `value: Some(5)`, and `geometry` from `build_piece_geometry` with `piece_size: Some(16384)`, `piece_group_size: Some(32768)`, and `total_extent: Some(1048576)`, thirty-two groups; every value differs from the domain builders' defaults and no two fields of a kind hold the same value
    * `[ ]`   The reference fields: `vec![CanonicalFieldValue::Text("@scope/example-package".to_string()), CanonicalFieldValue::Text("2.1.0-beta.3".to_string()), CanonicalFieldValue::FixedBytes32([0x0a; 32]), CanonicalFieldValue::FixedBytes32([0x0b; 32]), CanonicalFieldValue::Unsigned16(3), CanonicalFieldValue::FixedBytes32([0x0c; 32]), CanonicalFieldValue::Unsigned64(5), CanonicalFieldValue::Unsigned32(16384), CanonicalFieldValue::Unsigned32(32768), CanonicalFieldValue::Unsigned64(1048576)]`, written as a literal in each test that uses it; a test that varies it writes the literal with only the stated entries changed and passes it as `values` to `build_canonical_fields`
    * `[ ]`   `to_fields_lists_the_reference_context_fields_in_canonical_order`: contract: an admitted context maps to its fields in canonical order with each value unchanged; arrange the reference context; act `description.to_fields(ToFieldsParams, &context)`; assert `success.fields.values` equals the reference fields
    * `[ ]`   `from_fields_returns_the_context_the_reference_fields_describe`: contract: the canonical fields map back to the context they describe; arrange `build_canonical_fields` with the reference fields; act `description.from_fields(FromFieldsParams, fields)`; assert `success.described` equals the reference context
    * `[ ]`   `from_fields_rejects_a_field_list_one_short`: contract: a field list shorter than the canonical sequence is refused with both lengths; arrange the reference fields without their last entry; act `from_fields`; assert `error` equals `DerivationContextFromFieldsErrorReturn::FieldCount { expected: 10, actual: 9 }`
    * `[ ]`   `from_fields_rejects_a_field_list_one_long`: contract: a field list longer than the canonical sequence is refused with both lengths; arrange the reference fields followed by `CanonicalFieldValue::Unsigned64(1)`; act `from_fields`; assert `error` equals `DerivationContextFromFieldsErrorReturn::FieldCount { expected: 10, actual: 11 }`
    * `[ ]`   `from_fields_rejects_a_field_of_the_wrong_kind`: contract: a value whose kind differs from the canonical kind at its index is refused, naming the index and the expected kind; arrange the reference fields with index `4` replaced by `CanonicalFieldValue::Unsigned32(3)`; act `from_fields`; assert `error` equals `DerivationContextFromFieldsErrorReturn::FieldKind { index: 4, expected: CanonicalFieldKind::Unsigned16 }`
    * `[ ]`   `from_fields_reports_the_lowest_field_of_the_wrong_kind`: contract: of several misplaced kinds, the lowest index decides; arrange the reference fields with index `2` replaced by `CanonicalFieldValue::Text("x".to_string())` and index `7` replaced by `CanonicalFieldValue::Unsigned64(16384)`; act `from_fields`; assert `error` equals `DerivationContextFromFieldsErrorReturn::FieldKind { index: 2, expected: CanonicalFieldKind::FixedBytes32 }`
    * `[ ]`   `from_fields_checks_every_kind_before_constructing_any_component`: contract: a kind mismatch at any index is reported before a component refusal at a lower index; arrange the reference fields with index `0` replaced by `CanonicalFieldValue::Text(String::new())` and index `9` replaced by `CanonicalFieldValue::Unsigned32(1048576)`; act `from_fields`; assert `error` equals `DerivationContextFromFieldsErrorReturn::FieldKind { index: 9, expected: CanonicalFieldKind::Unsigned64 }`
    * `[ ]`   `from_fields_returns_the_asset_identity_refusal_unchanged`: arrange the reference fields with index `0` replaced by `CanonicalFieldValue::Text(String::new())`; act `from_fields`; assert `error` equals `DerivationContextFromFieldsErrorReturn::AssetIdentity(AssetIdentityTryNewErrorReturn::EmptyName)`
    * `[ ]`   `from_fields_returns_the_deployment_identity_refusal_unchanged`: arrange the reference fields with index `2` replaced by `CanonicalFieldValue::FixedBytes32([0u8; 32])`; act `from_fields`; assert `error` equals `DerivationContextFromFieldsErrorReturn::DeploymentIdentity(DeploymentIdentityTryNewErrorReturn::AllZero)`
    * `[ ]`   `from_fields_returns_the_suite_identifier_refusal_unchanged`: arrange the reference fields with index `4` replaced by `CanonicalFieldValue::Unsigned16(0)`; act `from_fields`; assert `error` equals `DerivationContextFromFieldsErrorReturn::SuiteIdentifier(SuiteIdentifierTryNewErrorReturn::ZeroVersion)`
    * `[ ]`   `from_fields_returns_the_parameter_set_identifier_refusal_unchanged`: arrange the reference fields with index `5` replaced by `CanonicalFieldValue::FixedBytes32([0u8; 32])`; act `from_fields`; assert `error` equals `DerivationContextFromFieldsErrorReturn::ParameterSetIdentifier(ParameterSetIdentifierTryNewErrorReturn::AllZero)`
    * `[ ]`   `from_fields_returns_the_piece_geometry_refusal_unchanged`: arrange the reference fields with index `9` replaced by `CanonicalFieldValue::Unsigned64(0)`; act `from_fields`; assert `error` equals `DerivationContextFromFieldsErrorReturn::PieceGeometry(PieceGeometryTryNewErrorReturn::ZeroTotalExtent)`
    * `[ ]`   `from_fields_returns_the_derivation_context_refusal_unchanged`: arrange the reference fields with index `6` replaced by `CanonicalFieldValue::Unsigned64(32)`, the index equal to the reference geometry's group count; act `from_fields`; assert `error` equals `DerivationContextFromFieldsErrorReturn::DerivationContext(DerivationContextTryNewErrorReturn::GroupIndexOutOfRange { group_index: 32, group_count: 32 })`
    * `[ ]`   `from_fields_constructs_the_components_in_field_order`: contract: when several components refuse, the refusal of the earliest field decides; arrange the reference fields with index `0` replaced by `CanonicalFieldValue::Text(String::new())` and index `9` replaced by `CanonicalFieldValue::Unsigned64(0)`; act `from_fields`; assert `error` equals `DerivationContextFromFieldsErrorReturn::AssetIdentity(AssetIdentityTryNewErrorReturn::EmptyName)`
    * `[ ]`   `derivation_context_description_declares_its_field_kinds_in_canonical_order`: contract: the description's declared kinds are the canonical sequence an encoding concrete decodes against; arrange nothing; act read `<DerivationContextDescription as IEncodingContract>::FIELDS`; assert it equals `[CanonicalFieldKind::Text, CanonicalFieldKind::Text, CanonicalFieldKind::FixedBytes32, CanonicalFieldKind::FixedBytes32, CanonicalFieldKind::Unsigned16, CanonicalFieldKind::FixedBytes32, CanonicalFieldKind::Unsigned64, CanonicalFieldKind::Unsigned32, CanonicalFieldKind::Unsigned32, CanonicalFieldKind::Unsigned64]` as a slice
    * `[ ]`   Every test block carries the full `Contract`, `Arrange`, `Act`, `Assert` header and the inline markers

  * `[ ]`   `construction`
    * `[ ]`   `DerivationContextDescription::try_new` is the description's only producer; a consumer that encodes or decodes a derivation context constructs it once and hands it to the encoder or decoder the encoding factory returns; it holds no state, so one instance serves every call
    * `[ ]`   The description module owns no object type a consumer builds as a fixture: the description is a unit struct constructed by `try_new` over fieldless params, and its error is an enum, so it has no `mock.rs`

  * `[ ]`   `adapters/encoding/src/derivation_context/mod.rs`
    * `[ ]`   Module declarations: `mod interface;`, `pub mod provides;`, and `#[cfg(test)] mod test;`
    * `[ ]`   `impl DerivationContextDescription` with `pub fn try_new(_params: DerivationContextDescriptionConstructorParams) -> DerivationContextDescriptionTryNewReturn` returning `Ok(DerivationContextDescription)`
    * `[ ]`   `impl IEncodingContract for DerivationContextDescription` with `type Described = DerivationContext;`, `type FromFieldsErrorReturn = DerivationContextFromFieldsErrorReturn;`, `const FIELDS: &'static [CanonicalFieldKind] = &DERIVATION_CONTEXT_FIELD_KINDS;`, and `to_fields` and `from_fields` realizing the branches and ordering of the interaction spec; `from_fields` converts the vector into the array, destructures it into its fields, matches each against its expected variant by `let … else` in ascending index order, then calls the constructors, the group index unpacked by `let Ok(group_index) = GroupIndex::try_new(…);` and every other refusal wrapped in its variant and returned
    * `[ ]`   Imports the contract's names from `crate::factory::provides`, the domain types, constructor params, and error types from `domain`, and this module's names from `interface`
    * `[ ]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[ ]`   `adapters/encoding/src/derivation_context/provides.rs`
    * `[ ]`   `pub use super::interface::*;`, nothing else

  * `[ ]`   `directionality`
    * `[ ]`   `derivation_context` depends on the `factory` module's surface through `crate::factory::provides` and on `domain`; the `factory` module depends on nothing in the crate and names no description; among repository crates the crate depends on `crates/domain` alone, inward; nothing depends on the crate yet; no cycle
    * `[ ]`   `encoding/abi` consumes the contract and this description; `kdf/blake3_keyed` consumes the encoding of this description's fields through the encoding factory

  * `[ ]`   `requirements`
    * `[ ]`   `adapters/encoding/Cargo.toml` carries exactly the tables and keys stated above, and no external crate is named in the crate
    * `[ ]`   `cargo check --workspace --all-targets --all-features`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo fmt --all --check`, and `cargo deny check` complete without error or warning
    * `[ ]`   `to_fields_lists_the_reference_context_fields_in_canonical_order` and `from_fields_returns_the_context_the_reference_fields_describe` pass (CR-11, the context's serialization frozen as one field sequence)
    * `[ ]`   `from_fields_rejects_a_field_list_one_short`, `from_fields_rejects_a_field_list_one_long`, `from_fields_rejects_a_field_of_the_wrong_kind`, `from_fields_reports_the_lowest_field_of_the_wrong_kind`, and `from_fields_checks_every_kind_before_constructing_any_component` pass
    * `[ ]`   `from_fields_returns_the_asset_identity_refusal_unchanged`, `from_fields_returns_the_deployment_identity_refusal_unchanged`, `from_fields_returns_the_suite_identifier_refusal_unchanged`, `from_fields_returns_the_parameter_set_identifier_refusal_unchanged`, `from_fields_returns_the_piece_geometry_refusal_unchanged`, `from_fields_returns_the_derivation_context_refusal_unchanged`, and `from_fields_constructs_the_components_in_field_order` pass
    * `[ ]`   `derivation_context_description_declares_its_field_kinds_in_canonical_order` passes
    * `[ ]`   Nothing in `crates/domain` names `encoding`, and code outside `crates/domain` still cannot read any field of a domain type

* `[ ]`   `encoding/abi` **Ethereum ABI concrete encoding and decoding any described domain type as the ABI parameter encoding of its canonical fields; authors the encoding family's encoder and decoder interfaces, the versioned encoding identifier, the declaration, and their mocks**

  * `[ ]`   `objective`
    * `[ ]`   Problem: the contracts recompute derivations, identity mappings, and challenges from ABI-encoded values, and the client hashes, signs, stores, and frames the same values, so one encoding serves both sides and turns a description's canonical fields into bytes and untrusted bytes back into an admitted domain value, with exactly one byte form per value (CR-11, serialization frozen with known-answer vectors; CR-09 statement binding)
    * `[ ]`   Functional: the family's encoder interface takes a description and a described value and returns its bytes; the family's decoder interface takes a description and untrusted bytes and returns the admitted described value or the refusal that names what failed
    * `[ ]`   Functional: both interfaces sit under one versioned encoding identifier, and every concrete declares that identifier, its adapter version, and the interface version it implements, readable before any instance exists
    * `[ ]`   Functional: the ABI concrete encodes a described value as Solidity's `abi.encode` over its canonical fields in order, a 32-byte fixed string as `bytes32`, an unsigned integer of 16, 32, or 64 bits as `uint16`, `uint32`, or `uint64`, and text as `string`
    * `[ ]`   Functional: the ABI concrete decodes only the one canonical byte form: truncated input, bytes past the canonical end, nonzero padding, a field list of the wrong length, and an integer word wider than its field are refused, and a value its description refuses is returned with the description's refusal unchanged
    * `[ ]`   Functional: the reference derivation context encodes to the known-answer vector authored from the ABI specification, and the vector decodes to the reference context
    * `[ ]`   Non-functional: `alloy` is named only inside `adapters/encoding/src/abi`; the family's interfaces name no vendor and no format

  * `[ ]`   `role`
    * `[ ]`   Adapter: the encoding family's first concrete, and the first source file that requires the family's encoder and decoder interfaces, the versioned encoding identifier, the declaration, and their mocks, which it authors in the family's `factory` module as its producers beside the encoding contract `encoding/derivation_context` authored
    * `[ ]`   Adds to `factory/interface.rs` and `factory/mock.rs`; `factory/mod.rs` and `factory/provides.rs` are unchanged; the factory function, its types, its unit test, and the family's integration test are `encoding/factory`'s
    * `[ ]`   Does not describe any domain type or check any domain invariant; the description supplies the fields and admits the value
    * `[ ]`   Does not emit the Solidity mirror of the vectors; `harness-crypto/generate/evm` mirrors them
    * `[ ]`   Does not map the encoding identifier to the hash-card's `encodingId` byte; the hash-card's own encoding does that
    * `[ ]`   Does not carry a commit

  * `[ ]`   `module`
    * `[ ]`   Bounded context: the `factory` module's encoder and decoder traits with their params, success, error, and return types, the encoding identifier, the declaration, the interface version, and their mocks; and the private `abi` concrete, holding the adapter over `alloy`'s dynamic ABI, its constructor params and return, and its decoding error
    * `[ ]`   Adds the `abi` module to the existing crate at `adapters/encoding`; the crate's manifest gains `alloy` and the `hex` dev-dependency, and its barrel gains the module's line
    * `[ ]`   Outside: every description and domain type, the factory's selection of a concrete, every derivation, and the Solidity mirror

  * `[ ]`   `deps`
    * `[ ]`   The `factory` module's contract, same crate: `IEncodingContract`, `CanonicalFieldKind`, `CanonicalFieldValue`, `CanonicalFields`, `ToFieldsParams`, `FromFieldsParams`, and `FromFieldsSuccessReturn`, as `encoding/derivation_context` authors them
    * `[ ]`   `derivation_context`, same crate, in `abi/test.rs` only: `DerivationContextDescription`, `DerivationContextDescriptionConstructorParams`, and `DerivationContextFromFieldsErrorReturn`, the description the known-answer vectors exercise
    * `[ ]`   `domain` with its `mocks` feature, the existing dev-dependency, in `abi/test.rs` only: the builders and overrides that make the reference context, and `DeploymentIdentityTryNewErrorReturn`
    * `[ ]`   `alloy` `2.5.0`, external crate, MIT OR Apache-2.0, runtime dependency with default features off and the features `std` and `dyn-abi`, named only in `abi`; supplies `alloy::dyn_abi::{DynSolType, DynSolValue, Error}` and `alloy::primitives::{B256, U256}`; the dynamic ABI's decoder does not reject trailing bytes, nonzero padding, or integer words wider than their type, so the concrete enforces the canonical form itself
    * `[ ]`   `hex` `0.4.3`, external crate, MIT OR Apache-2.0, dev-dependency only; supplies `hex::decode` for the vectors
    * `[ ]`   `core::convert::Infallible`, standard library, the error arm of every operation with no failure; `TryFrom<&[u8]> for [u8; 32]`, standard library
    * `[ ]`   No reverse dependency beyond the family form's recorded cycle: the `factory` module's decoder error carries this concrete's error
    * `[ ]`   Nothing depends on the crate yet

  * `[ ]`   `context_slice`
    * `[ ]`   From the contract: `IEncodingContract::FIELDS`, `to_fields(&self, ToFieldsParams, &Self::Described) -> Result<ToFieldsSuccessReturn, Infallible>`, and `from_fields(&self, FromFieldsParams, CanonicalFields) -> Result<FromFieldsSuccessReturn<Self::Described>, Self::FromFieldsErrorReturn>`
    * `[ ]`   From `alloy::dyn_abi`: `DynSolType::FixedBytes(usize)`, `DynSolType::Uint(usize)`, `DynSolType::String`, and `DynSolType::Tuple(Vec<DynSolType>)`; `DynSolType::abi_decode_params(&self, data: &[u8]) -> Result<DynSolValue, Error>`, which decodes a tuple as a parameter sequence; `DynSolValue::FixedBytes(B256, usize)`, `DynSolValue::Uint(U256, usize)`, `DynSolValue::String(String)`, and `DynSolValue::Tuple(Vec<DynSolValue>)`; `DynSolValue::abi_encode_params(&self) -> Vec<u8>`, which encodes a tuple as a parameter sequence, as `abi.encode` does; `DynSolValue::as_tuple(&self) -> Option<&[DynSolValue]>`, `as_fixed_bytes(&self) -> Option<(&[u8], usize)>`, `as_uint(&self) -> Option<(U256, usize)>`, and `as_str(&self) -> Option<&str>`; `Error`, which implements `Debug`
    * `[ ]`   From `alloy::primitives`: `B256::from([u8; 32])`, `U256::from(u16)`, `U256::from(u32)`, `U256::from(u64)`, and `u16::try_from(U256)`, `u32::try_from(U256)`, and `u64::try_from(U256)`, each failing when the value exceeds the target's width
    * `[ ]`   From `domain`'s mocks, in `abi/test.rs` only: `build_derivation_context` with `DerivationContextConstructorParamsOverrides`, `build_asset_identity` with `AssetIdentityConstructorParamsOverrides`, `build_deployment_identity` with `DeploymentIdentityConstructorParamsOverrides`, `build_suite_identifier` with `SuiteIdentifierConstructorParamsOverrides`, `build_parameter_set_identifier` with `ParameterSetIdentifierConstructorParamsOverrides`, `build_group_index` with `GroupIndexConstructorParamsOverrides`, and `build_piece_geometry` with `PieceGeometryConstructorParamsOverrides`

  * `[ ]`   `adapters/encoding/Cargo.toml`
    * `[ ]`   `[dependencies]` reads `domain = { path = "../../crates/domain" }` and `alloy = { version = "2.5.0", default-features = false, features = ["std", "dyn-abi"] }`
    * `[ ]`   `[dev-dependencies]` reads `domain = { path = "../../crates/domain", features = ["mocks"] }` and `hex = "0.4.3"`
    * `[ ]`   `[package]`, `[features]`, and `[lints]` are unchanged; no other table

  * `[ ]`   `adapters/encoding/src/lib.rs`
    * `[ ]`   The crate barrel reads `mod abi;`, `mod derivation_context;`, `mod factory;`, `pub use derivation_context::provides::*;`, and `pub use factory::provides::*;`, nothing else; the `abi` concrete's surface is not re-exported
    * `[ ]`   Until `abi/mod.rs` exists, `cargo check` reports the unresolved `mod abi`, which is the RED state for every element below that precedes it

  * `[ ]`   `adapters/encoding/src/factory/interface.rs`
    * `[ ]`   `ENCODING_INTERFACE_VERSION`, a `pub const` of type `u32` with value `1`
    * `[ ]`   `EncodingIdentifier`, an enum with the one variant `EthereumAbiV1`, the versioned identifier both interfaces sit under and a deployment's hash-card names
    * `[ ]`   `EncodingDeclaration`, a struct with `pub identifier: EncodingIdentifier`, `pub adapter_version: u32`, and `pub interface_version: u32`
    * `[ ]`   `EncodeParams<'a, D>`, a struct with `pub description: &'a D`, the description that selects the field sequence; `EncodeSuccessReturn`, a struct with `pub bytes: Vec<u8>`; `EncodeReturn`, the alias `Result<EncodeSuccessReturn, Infallible>`, the error arm uninhabited because every admitted value has its encoding
    * `[ ]`   `IEncoderAdapter`, a trait with the one method `fn encode<D: IEncodingContract>(&self, params: EncodeParams<'_, D>, payload: &D::Described) -> EncodeReturn;`
    * `[ ]`   `DecodeParams<'a, D>`, a struct with `pub description: &'a D`; `DecodeSuccessReturn<T>`, a struct with `pub described: T`; `DecodeErrorReturn<E>`, an enum with `#[derive(Debug)]` and the variants `Abi(AbiDecoderErrorReturn)`, the ABI concrete's refusal carried unchanged, and `Description(E)`, the description's refusal carried unchanged; each further concrete's refusal is its own variant; `DecodeReturn<T, E>`, the alias `Result<DecodeSuccessReturn<T>, DecodeErrorReturn<E>>`
    * `[ ]`   `IDecoderAdapter`, a trait with the one method `fn decode<D: IEncodingContract>(&self, params: DecodeParams<'_, D>, payload: &[u8]) -> DecodeReturn<D::Described, D::FromFieldsErrorReturn>;`, the validating form, its payload the untrusted bytes and its narrowing target `D::Described`
    * `[ ]`   Adds the import of `AbiDecoderErrorReturn` from `crate::abi::provides`; every item `encoding/derivation_context` authored in this file is unchanged; names no vendor

  * `[ ]`   `adapters/encoding/src/abi/interface.rs`
    * `[ ]`   `AbiEncoding`, the unit struct `pub struct AbiEncoding;`, the adapter over `alloy`'s dynamic ABI, implementing both the encoder and the decoder interface
    * `[ ]`   `AbiEncodingConstructorParams`, the fieldless struct `pub struct AbiEncodingConstructorParams;`, the constructor's deps slot
    * `[ ]`   `AbiEncodingTryNewReturn`, the alias `Result<AbiEncoding, Infallible>`; the error arm is uninhabited because the adapter takes no configuration
    * `[ ]`   `AbiDecoderErrorReturn`, an enum with `#[derive(Debug)]` and the variants `Malformed(alloy::dyn_abi::Error)`, the ABI decoder's refusal carried unchanged; `DecodedShapeMismatch { index: usize }`, the decoded value holding no item of the described kind at `index`; `ValueOutOfRange { index: usize, kind: CanonicalFieldKind }`, an integer word at `index` wider than its field; and `NonCanonical`, the decoded value re-encoding to bytes other than the input
    * `[ ]`   Imports `CanonicalFieldKind` from `crate::factory::provides` and `core::convert::Infallible`; names `alloy::dyn_abi::Error` by its full path; declares nothing else

  * `[ ]`   `adapters/encoding/src/abi/interaction.spec.md`
    * `[ ]`   `AbiEncoding::try_new(params: AbiEncodingConstructorParams) -> AbiEncodingTryNewReturn`: one branch; outcome `Ok(AbiEncoding)`; the error arm has no branch
    * `[ ]`   `AbiEncoding::DECLARATION`: the inherent constant `EncodingDeclaration { identifier: EncodingIdentifier::EthereumAbiV1, adapter_version: 1, interface_version: ENCODING_INTERFACE_VERSION }`
    * `[ ]`   `encode`: one branch; condition any description and admitted value; decision none; dependency call `params.description.to_fields(ToFieldsParams, payload)` once, unpacked irrefutably, then `DynSolValue::abi_encode_params` once; each canonical value maps in order, `FixedBytes32(bytes)` to `DynSolValue::FixedBytes(B256::from(bytes), 32)`, `Unsigned16(value)`, `Unsigned32(value)`, and `Unsigned64(value)` to `DynSolValue::Uint(U256::from(value), 16)`, `32`, and `64`, and `Text(text)` to `DynSolValue::String(text)`, collected into `DynSolValue::Tuple`; outcome `Ok(EncodeSuccessReturn { bytes })` holding the parameter encoding
    * `[ ]`   `decode`, malformed: condition `DynSolType::Tuple` over the description's `FIELDS`, each kind mapped to `DynSolType::FixedBytes(32)`, `DynSolType::Uint(16)`, `DynSolType::Uint(32)`, `DynSolType::Uint(64)`, or `DynSolType::String`, refuses the payload through `abi_decode_params`; decision the decoder's result; dependency call `abi_decode_params` once; outcome `Err(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::Malformed(error)))`, the error unchanged
    * `[ ]`   `decode`, shape mismatch: condition the decoded value is not a tuple, or holds at some index of `FIELDS` no item or an item not of the kind named there, a `bytes32` item being `as_fixed_bytes` of size `32` converting into `[u8; 32]`, an integer item `as_uint` of the field's width, and a text item `as_str`; decision the fields are read in ascending index order; dependency call none; outcome `Err(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::DecodedShapeMismatch { index }))` for the lowest such index, `0` when the value is not a tuple; the decoder returns the type it was given, so no input takes this branch and it has no unit test
    * `[ ]`   `decode`, integer out of range: condition an integer item's value does not convert into the field's `u16`, `u32`, or `u64` through `try_from`; decision the conversion, in the same ascending pass; dependency call none; outcome `Err(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::ValueOutOfRange { index, kind }))` for the lowest such index, `kind` the field's kind
    * `[ ]`   `decode`, description refused: condition every item converts; decision the description's result; dependency call `params.description.from_fields(FromFieldsParams, CanonicalFields { values })` once over the converted values in field order; outcome `Err(DecodeErrorReturn::Description(error))`, the refusal unchanged
    * `[ ]`   `decode`, non-canonical: condition the description admits the value and `self.encode(EncodeParams { description: params.description }, &described)` returns bytes other than the payload; decision byte equality; dependency call `encode` once; outcome `Err(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::NonCanonical))`; this refuses trailing bytes, nonzero padding, non-canonical offsets, and a head of the wrong length, which the dynamic decoder reads past
    * `[ ]`   `decode`, admitted: condition the re-encoding equals the payload; outcome `Ok(DecodeSuccessReturn { described })`
    * `[ ]`   Ordering: the ABI decode precedes the item pass, the item pass precedes the description, and the re-encoding comparison runs last; `params` supplies the description and nothing else is read from it

  * `[ ]`   `adapters/encoding/src/factory/mock.rs`
    * `[ ]`   `EncodingDeclarationOverrides`, `#[derive(Default)]`, one `Option` per field; `build_encoding_declaration(overrides: EncodingDeclarationOverrides) -> EncodingDeclaration`, defaulting to `EncodingIdentifier::EthereumAbiV1`, `1`, and `ENCODING_INTERFACE_VERSION`
    * `[ ]`   `EncodeSuccessReturnOverrides`, `#[derive(Default)]`, one field `pub bytes: Option<Vec<u8>>`; `build_encode_success_return(overrides: EncodeSuccessReturnOverrides) -> EncodeSuccessReturn`, the bytes defaulting to an empty `Vec`
    * `[ ]`   `DecodeSuccessReturnOverrides<T>`, `#[derive(Default)]`, one field `pub described: Option<T>`; `build_decode_success_return<T: Default>(overrides: DecodeSuccessReturnOverrides<T>) -> DecodeSuccessReturn<T>`, the described value defaulting to `T::default()`
    * `[ ]`   `MockIEncoderAdapter`, the unit struct `pub struct MockIEncoderAdapter;`, implementing `IEncoderAdapter` with `encode` returning `Ok(build_encode_success_return(Default::default()))` for any description, params, and payload
    * `[ ]`   `MockIDecoderAdapter`, the unit struct `pub struct MockIDecoderAdapter;`, implementing `IDecoderAdapter` with `decode` reading no bytes and handing the description the default field list, `params.description.from_fields(FromFieldsParams, build_canonical_fields(Default::default()))`, returning `Ok(DecodeSuccessReturn { described })` on admission and `Err(DecodeErrorReturn::Description(error))` on refusal, since no default exists for an arbitrary described type; paired with `MockIEncodingContract`, which admits the empty list, it returns `Ok`; a test needing other behavior implements the trait on its own local struct
    * `[ ]`   No builder for `EncodeParams` and `DecodeParams`, whose only field is a borrowed description with no default, written at the call site as their production values; none for the enums `EncodingIdentifier` and `DecodeErrorReturn`; no corruptions type and no invalidator, since the decoder takes the untrusted bytes directly
    * `[ ]`   Every symbol `encoding/derivation_context` authored in this file is unchanged; the file's imports gain this node's types from `super::interface`

  * `[ ]`   `adapters/encoding/src/abi/test.rs`
    * `[ ]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`; imports `AbiEncoding`, `AbiEncodingConstructorParams`, and `AbiDecoderErrorReturn` from `super::interface`; `IEncoderAdapter`, `IDecoderAdapter`, `EncodeParams`, `DecodeParams`, `DecodeErrorReturn`, `CanonicalFieldKind`, `EncodingIdentifier`, and `ENCODING_INTERFACE_VERSION` from `crate::factory::provides`; `DerivationContextDescription`, `DerivationContextDescriptionConstructorParams`, and `DerivationContextFromFieldsErrorReturn` from `crate::derivation_context::provides`; the domain builders and overrides the context slice lists with `DeploymentIdentityTryNewErrorReturn` from `domain`; and `hex::decode`
    * `[ ]`   Each test constructs the subject by `let Ok(encoding) = AbiEncoding::try_new(AbiEncodingConstructorParams);` and the description by `let Ok(description) = DerivationContextDescription::try_new(DerivationContextDescriptionConstructorParams);`, passes `EncodeParams { description: &description }` or `DecodeParams { description: &description }`, decodes hex by `let Ok(bytes) = decode(…) else { panic!(…) };`, and unpacks a call by `let Ok(success) = … else { panic!(…) };` or `let Err(error) = … else { panic!(…) };`
    * `[ ]`   The reference context: the one `encoding/derivation_context`'s tests build, `@scope/example-package` at `2.1.0-beta.3`, deployment `[0x0a; 32]`, suite `[0x0b; 32]` at version `3`, parameter set `[0x0c; 32]`, group index `5`, and geometry of piece size `16384`, piece-group size `32768`, and total extent `1048576`, built through `build_derivation_context` with every component overridden
    * `[ ]`   The reference vector, authored from the ABI specification's parameter encoding: the 32-byte words, in order, `0x140`, the first string's offset; `0x180`, the second string's offset; 32 bytes of `0a`; 32 bytes of `0b`; `3`; 32 bytes of `0c`; `5`; `0x4000`; `0x8000`; `0x100000`; `0x16`, the asset name's length; the bytes `4073636f70652f6578616d706c652d7061636b616765` followed by 10 zero bytes; `0xc`, the asset version's length; and the bytes `322e312e302d626574612e33` followed by 20 zero bytes; each numeric word a 32-byte big-endian integer, 448 bytes in all, written as one hex string of the words in order; word `n` occupies bytes `32n` through `32n + 31`
    * `[ ]`   The field-too-many vector: the ABI parameter encoding of the reference fields followed by a `uint64` of `1`: the words `0x160`, `0x1a0`, the reference vector's words `2` through `9`, `1`, and the reference vector's words `10` through `13`, 480 bytes
    * `[ ]`   `encode_writes_the_reference_context_as_its_abi_parameter_encoding`: contract: a described value encodes as `abi.encode` over its canonical fields; arrange the reference context; act `encoding.encode(EncodeParams { description: &description }, &context)`; assert `success.bytes` equals the reference vector
    * `[ ]`   `decode_reads_the_reference_vector_as_the_reference_context`: contract: the canonical encoding decodes to the value it encodes; arrange the reference vector; act `encoding.decode(DecodeParams { description: &description }, &bytes)`; assert `success.described` equals the reference context
    * `[ ]`   `decode_rejects_input_truncated_to_the_head`: contract: input missing the dynamic tail is refused by the ABI decoder; arrange the reference vector's first `0x140` bytes; act `decode`; assert `error` matches `DecodeErrorReturn::Abi(AbiDecoderErrorReturn::Malformed(_))`
    * `[ ]`   `decode_rejects_trailing_bytes`: contract: bytes past the canonical end are refused; arrange the reference vector followed by 32 zero bytes; act `decode`; assert `error` matches `DecodeErrorReturn::Abi(AbiDecoderErrorReturn::NonCanonical)`
    * `[ ]`   `decode_rejects_nonzero_string_padding`: contract: a string's padding is zero in its one byte form; arrange the reference vector with byte `447`, the asset version's last padding byte, set to `0x01`; act `decode`; assert `error` matches `DecodeErrorReturn::Abi(AbiDecoderErrorReturn::NonCanonical)`
    * `[ ]`   `decode_rejects_a_uint16_word_wider_than_sixteen_bits`: contract: an integer word with bits above its field's width is refused, naming the field; arrange the reference vector with byte `157` set to `0x01`, the suite version's word reading `0x010003`; act `decode`; assert `error` matches `DecodeErrorReturn::Abi(AbiDecoderErrorReturn::ValueOutOfRange { index: 4, kind: CanonicalFieldKind::Unsigned16 })`
    * `[ ]`   `decode_rejects_a_uint64_word_wider_than_sixty_four_bits`: arrange the reference vector with byte `288`, the total extent's word's high byte, set to `0x80`; act `decode`; assert `error` matches `DecodeErrorReturn::Abi(AbiDecoderErrorReturn::ValueOutOfRange { index: 9, kind: CanonicalFieldKind::Unsigned64 })`
    * `[ ]`   `decode_rejects_the_encoding_of_one_field_too_many`: contract: an encoding with a field beyond the description's sequence is refused; arrange the field-too-many vector; act `decode`; assert `error` matches `DecodeErrorReturn::Abi(AbiDecoderErrorReturn::NonCanonical)`
    * `[ ]`   `decode_returns_the_description_refusal_unchanged`: contract: a canonically encoded value the description refuses returns that refusal; arrange the reference vector with bytes `64` through `95`, the deployment identity, set to zero; act `decode`; assert `error` matches `DecodeErrorReturn::Description(DerivationContextFromFieldsErrorReturn::DeploymentIdentity(DeploymentIdentityTryNewErrorReturn::AllZero))`
    * `[ ]`   `abi_encoding_declares_its_identifier_and_versions`: contract: the concrete's declaration names the encoding identifier, its adapter version, and the interface version it implements; act read `AbiEncoding::DECLARATION`; assert `identifier` matches `EncodingIdentifier::EthereumAbiV1`, `adapter_version` equals `1`, and `interface_version` equals `ENCODING_INTERFACE_VERSION`
    * `[ ]`   Every test block carries the full `Contract`, `Arrange`, `Act`, `Assert` header and the inline markers

  * `[ ]`   `construction`
    * `[ ]`   `AbiEncoding::try_new` is the concrete's only producer, and its only caller is the encoding factory, which reads `AbiEncoding::DECLARATION` before constructing and hands the adapter to consumers generic over `IEncoderAdapter` and `IDecoderAdapter`
    * `[ ]`   The concrete owns no object type a consumer builds as a fixture: the adapter is a unit struct constructed by `try_new` over fieldless params and its error is an enum, so it has no `mock.rs`

  * `[ ]`   `adapters/encoding/src/abi/mod.rs`
    * `[ ]`   Module declarations: `mod interface;`, `pub(crate) mod provides;`, and `#[cfg(test)] mod test;`
    * `[ ]`   `impl AbiEncoding` with `pub const DECLARATION: EncodingDeclaration` and `pub fn try_new(_params: AbiEncodingConstructorParams) -> AbiEncodingTryNewReturn` returning `Ok(AbiEncoding)`
    * `[ ]`   `impl IEncoderAdapter for AbiEncoding` and `impl IDecoderAdapter for AbiEncoding`, each method realizing the branches and ordering of the interaction spec; the item pass matches each `FIELDS` kind against its item with `let … else`, and the re-encoding calls this adapter's own `encode`
    * `[ ]`   Imports the family's names from `crate::factory::provides`, `DynSolType` and `DynSolValue` from `alloy::dyn_abi`, `B256` and `U256` from `alloy::primitives`, and this module's types from `interface`
    * `[ ]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[ ]`   `adapters/encoding/src/abi/provides.rs`
    * `[ ]`   `pub(crate) use super::interface::*;`, nothing else, so the concrete is visible to the crate's factory and to nothing outside the crate

  * `[ ]`   `directionality`
    * `[ ]`   `abi` depends on the `factory` module's surface through `crate::factory::provides` and on `alloy`; the `factory` module depends on `abi`'s error through `crate::abi::provides`, the family form's recorded cycle, which `encoding/factory` completes by constructing the concrete; `abi` names no description outside its tests; among repository crates the crate depends on `crates/domain` alone; nothing depends on the crate yet

  * `[ ]`   `requirements`
    * `[ ]`   `adapters/encoding/Cargo.toml` carries exactly the tables and keys stated above, and `alloy` is named nowhere in the crate outside `adapters/encoding/src/abi`
    * `[ ]`   `cargo check --workspace --all-targets --all-features`, `cargo fmt --all --check`, and `cargo deny check` complete without error; `cargo clippy --workspace --all-targets --all-features` reports nothing beyond the library target's unused-item warnings for the `abi` concrete, which `encoding/factory` resolves by constructing the concrete
    * `[ ]`   `encode_writes_the_reference_context_as_its_abi_parameter_encoding` and `decode_reads_the_reference_vector_as_the_reference_context` pass (CR-11, the derivation context's serialization frozen against a known-answer vector)
    * `[ ]`   `decode_rejects_input_truncated_to_the_head`, `decode_rejects_trailing_bytes`, `decode_rejects_nonzero_string_padding`, `decode_rejects_a_uint16_word_wider_than_sixteen_bits`, `decode_rejects_a_uint64_word_wider_than_sixty_four_bits`, and `decode_rejects_the_encoding_of_one_field_too_many` pass (one byte form per value)
    * `[ ]`   `decode_returns_the_description_refusal_unchanged` and `abi_encoding_declares_its_identifier_and_versions` pass
    * `[ ]`   Code outside `adapters/encoding` naming `AbiEncoding` or anything under `abi` fails to compile; the crate's public surface is the `factory` and `derivation_context` modules' `provides`

* `[ ]`   `encoding/factory` **Encoding factory constructing the concrete the configuration names, admitted against the encoding identifier the hash-card or configuration requires, and handing it to a consumer generic over the family's encoder and decoder traits; carries the family's integration test**

  * `[ ]`   `objective`
    * `[ ]`   Problem: a consumer obtains an encoder and decoder only through the family's generic surface, never by naming a concrete, and the encoding it uses is the one the deployment's hash-card or the configuration requires, so a concrete that does not declare the required identifier is refused before anything is constructed and every hashed, signed, stored, or framed value is encoded one way for one identifier (CR-11; Composition Boundary)
    * `[ ]`   Functional: given the concrete the configuration names and the encoding identifier required, the factory refuses a concrete whose declared identifier is not the required one, with no construction and no call to the consumer
    * `[ ]`   Functional: an admitted concrete is constructed and handed, with its declaration, to a consumer generic over `IEncoderAdapter` and `IDecoderAdapter`, and the consumer's output is returned; the consumer never names the concrete
    * `[ ]`   Functional: a concrete's constructor error is returned unchanged in the factory's error arm, one variant per concrete
    * `[ ]`   Functional: the concrete the factory constructs encodes the reference derivation context to the known-answer vector and decodes it back through the family's traits and the description
    * `[ ]`   Non-functional: adding a concrete is its module, its variant in the selection enum and in the error enum, and its branch here; adding an identifier is its variant in `EncodingIdentifier`; no consumer changes

  * `[ ]`   `role`
    * `[ ]`   Adapter family factory: the implementation of the `factory` module, the encoding family's construction point, and the crate's public surface beside the family-owned descriptions
    * `[ ]`   Hands the concrete to a consumer rather than returning it, because `IEncoderAdapter` and `IDecoderAdapter` carry generic methods and so cannot be returned as one type across concretes; the consumer is written once, generic over both traits, and the factory instantiates it for the concrete it constructs
    * `[ ]`   Selects by concrete and admits by identifier, so a further concrete implementing an existing identifier and a further identifier each arrive as a variant and a branch, with no change to the params or to any consumer
    * `[ ]`   Does not read a hash-card or the configuration; the composition resolver passes the concrete the configuration names as a typed `EncodingConcrete` and the identifier the hash-card or configuration requires as a typed `EncodingIdentifier`
    * `[ ]`   Does not encode, decode, or describe; the concrete and the descriptions do
    * `[ ]`   Carries the family's integration test across factory, concrete, and description; does not carry a commit

  * `[ ]`   `module`
    * `[ ]`   Bounded context: the `factory` module of `adapters/encoding`, holding the factory function, its deps, params, payload, and return types, its signature type, the consumer trait with its params and payload types, the selection enum, the function mock and builders, and the crate's integration test under `adapters/encoding/tests`
    * `[ ]`   Outside: the concrete's behavior, every description and domain type, the hash-card, the configuration catalogue, and every consumer of the family

  * `[ ]`   `deps`
    * `[ ]`   The `abi` concrete, through `crate::abi::provides`: `AbiEncoding`, `AbiEncodingConstructorParams`, `AbiEncoding::try_new`, and `AbiEncoding::DECLARATION`; the factory constructs its concrete, completing the family form's recorded cycle that `encoding/abi` opened
    * `[ ]`   The `factory` module's own interface: `IEncoderAdapter`, `IDecoderAdapter`, `EncodingIdentifier`, `EncodingDeclaration`, and `ENCODING_INTERFACE_VERSION`
    * `[ ]`   In the integration test only: `DerivationContextDescription` and `DerivationContextDescriptionConstructorParams` from the crate's public surface, `EncodeParams` and `DecodeParams`, the domain builders and overrides from the existing `domain` dev-dependency with its `mocks` feature, and `hex::decode` from the existing `hex` dev-dependency
    * `[ ]`   `core::convert::Infallible`, standard library, the concrete's constructor error carried in the factory's error arm
    * `[ ]`   No new external crate; `adapters/encoding/Cargo.toml` is unchanged

  * `[ ]`   `context_slice`
    * `[ ]`   From the concrete: `AbiEncoding::try_new(AbiEncodingConstructorParams) -> Result<AbiEncoding, Infallible>`, the inherent constant `AbiEncoding::DECLARATION: EncodingDeclaration`, and `AbiEncoding`'s implementations of `IEncoderAdapter` and `IDecoderAdapter`
    * `[ ]`   In the integration test: `encode<D: IEncodingContract>(&self, EncodeParams { description }, &D::Described) -> Result<EncodeSuccessReturn, Infallible>` and `decode<D: IEncodingContract>(&self, DecodeParams { description }, &[u8]) -> Result<DecodeSuccessReturn<D::Described>, DecodeErrorReturn<D::FromFieldsErrorReturn>>`, and `DerivationContextDescription::try_new(DerivationContextDescriptionConstructorParams) -> Result<DerivationContextDescription, Infallible>`

  * `[ ]`   `adapters/encoding/src/factory/interface.rs`
    * `[ ]`   `EncodingIdentifier` gains `#[derive(PartialEq, Eq)]`, so the admission compares a declared identifier with the required one
    * `[ ]`   `EncodingConcrete`, an enum with the one variant `Abi`, the selection of the concrete to construct
    * `[ ]`   `IEncodingConsumer`, a trait with `type Output;` and `fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(&self, params: ConsumeEncodingParams, payload: ConsumeEncodingPayload<E>) -> Self::Output;`, the work a composition performs with whichever concrete the factory constructs
    * `[ ]`   `ConsumeEncodingParams`, the fieldless struct `pub struct ConsumeEncodingParams;`
    * `[ ]`   `ConsumeEncodingPayload<E>`, a struct with `pub adapter: E` and `pub declaration: EncodingDeclaration`
    * `[ ]`   `CreateEncodingDeps<C>`, a struct with `pub consumer: C`, the collaborator the factory hands the concrete to
    * `[ ]`   `CreateEncodingParams`, a struct with `pub concrete: EncodingConcrete` and `pub identifier: EncodingIdentifier`, the selection and the identifier the concrete must declare
    * `[ ]`   `CreateEncodingPayload`, the fieldless struct `pub struct CreateEncodingPayload;`, since the factory operates on no data
    * `[ ]`   `CreateEncodingSuccessReturn<O>`, a struct with `pub output: O`
    * `[ ]`   `CreateEncodingErrorReturn`, an enum with the variants `UnsupportedEncodingIdentifier`, the named concrete not declaring the required identifier, and `Abi(Infallible)`, the ABI concrete's constructor error carried unchanged; each further concrete's constructor error is its own variant
    * `[ ]`   `CreateEncodingReturn<O>`, the alias `Result<CreateEncodingSuccessReturn<O>, CreateEncodingErrorReturn>`
    * `[ ]`   `CreateEncodingFn<C>`, the alias `fn(&CreateEncodingDeps<C>, CreateEncodingParams, CreateEncodingPayload) -> CreateEncodingReturn<<C as IEncodingConsumer>::Output>`
    * `[ ]`   Every item `encoding/derivation_context` and `encoding/abi` authored in this file is unchanged except the derive on `EncodingIdentifier`

  * `[ ]`   `adapters/encoding/src/factory/interaction.spec.md`
    * `[ ]`   `create_encoding<C: IEncodingConsumer>(deps: &CreateEncodingDeps<C>, params: CreateEncodingParams, payload: CreateEncodingPayload) -> CreateEncodingReturn<C::Output>`: decision a `match` on `params.concrete`, one arm per `EncodingConcrete` variant, exhaustive so a variant with no arm fails to compile
    * `[ ]`   Unsupported identifier: condition the named concrete's `DECLARATION.identifier` is not `params.identifier`; decision equality, read before any construction; dependency call none; outcome `Err(CreateEncodingErrorReturn::UnsupportedEncodingIdentifier)`, with nothing constructed and the consumer not called; `EncodingIdentifier` has the one variant the ABI concrete declares, so no input takes this branch until a further identifier exists, and it has no unit test
    * `[ ]`   Admitted: condition the named concrete's declared identifier is `params.identifier`; dependency calls the concrete's `try_new` with its fieldless constructor params, exactly once, its success destructured irrefutably because its error arm is uninhabited, then `deps.consumer.consume_encoding(ConsumeEncodingParams, ConsumeEncodingPayload { adapter, declaration })` with the concrete's `DECLARATION`, exactly once; outcome `Ok(CreateEncodingSuccessReturn { output })` holding the consumer's output
    * `[ ]`   `CreateEncodingErrorReturn::Abi` carries the constructor's uninhabited error type in the return union, so no branch produces it
    * `[ ]`   `params.concrete` selects and `params.identifier` admits; `payload` carries nothing and is not read

  * `[ ]`   `adapters/encoding/src/factory/mock.rs`
    * `[ ]`   `CreateEncodingParamsOverrides`, `#[derive(Default)]`, fields `pub concrete: Option<EncodingConcrete>` and `pub identifier: Option<EncodingIdentifier>`; `build_create_encoding_params(overrides: CreateEncodingParamsOverrides) -> CreateEncodingParams`, defaulting to `EncodingConcrete::Abi` and `EncodingIdentifier::EthereumAbiV1`
    * `[ ]`   `CreateEncodingSuccessReturnOverrides<O>`, `#[derive(Default)]`, one field `pub output: Option<O>`; `build_create_encoding_success_return<O: Default>(overrides: CreateEncodingSuccessReturnOverrides<O>) -> CreateEncodingSuccessReturn<O>`, the output defaulting to `O::default()`
    * `[ ]`   `ConsumeEncodingPayloadOverrides<E>`, `#[derive(Default)]`, fields `pub adapter: Option<E>` and `pub declaration: Option<EncodingDeclaration>`; `build_consume_encoding_payload<E: Default>(overrides: ConsumeEncodingPayloadOverrides<E>) -> ConsumeEncodingPayload<E>`, the adapter defaulting to `E::default()` and the declaration to `build_encoding_declaration(Default::default())`
    * `[ ]`   `MockIEncodingConsumer`, the unit struct `pub struct MockIEncodingConsumer;`, implementing `IEncodingConsumer` with `type Output = ();` and `consume_encoding` returning `()` for any adapter; a test needing other behavior implements the trait on its own local struct
    * `[ ]`   `mock_create_encoding<C: IEncodingConsumer>(_deps: &CreateEncodingDeps<C>, _params: CreateEncodingParams, _payload: CreateEncodingPayload) -> CreateEncodingReturn<C::Output>` for `C::Output: Default`, returning `Ok(build_create_encoding_success_return(Default::default()))`
    * `[ ]`   No builder for the fieldless `ConsumeEncodingParams` and `CreateEncodingPayload`, used by their production values, for `CreateEncodingDeps`, whose one field is the consumer the test supplies, or for the enums `EncodingConcrete` and `CreateEncodingErrorReturn`; every symbol `encoding/derivation_context` and `encoding/abi` authored in this file is unchanged

  * `[ ]`   `adapters/encoding/src/factory/test.rs`
    * `[ ]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`; imports `create_encoding` from `super`, `CreateEncodingDeps`, `CreateEncodingPayload`, `ConsumeEncodingParams`, `ConsumeEncodingPayload`, `IEncodingConsumer`, `IEncoderAdapter`, `IDecoderAdapter`, `EncodingConcrete`, `EncodingDeclaration`, `EncodingIdentifier`, and `ENCODING_INTERFACE_VERSION` from `super::interface`, and `build_create_encoding_params` and `CreateEncodingParamsOverrides` from `super::mock`
    * `[ ]`   A test-local `DeclarationProbe`, the unit struct implementing `IEncodingConsumer` with `type Output = EncodingDeclaration;` and `consume_encoding` returning `payload.declaration`
    * `[ ]`   `create_encoding_hands_the_consumer_the_abi_concrete_and_its_declaration`: contract: the ABI concrete, admitted for the Ethereum ABI identifier, reaches the consumer with its declaration; arrange `build_create_encoding_params` with `concrete: Some(EncodingConcrete::Abi)` and `identifier: Some(EncodingIdentifier::EthereumAbiV1)`, and `CreateEncodingDeps { consumer: DeclarationProbe }`; act `create_encoding(&deps, params, CreateEncodingPayload)`, unpacked by `let Ok(success) = … else { panic!(…) };`; assert `success.output.identifier` matches `EncodingIdentifier::EthereumAbiV1`, `success.output.adapter_version` equals `1`, and `success.output.interface_version` equals `ENCODING_INTERFACE_VERSION`
    * `[ ]`   The unsupported-identifier branch has no unit test, as the interaction spec states
    * `[ ]`   The test block carries the full `Contract`, `Arrange`, `Act`, `Assert` header and the inline markers

  * `[ ]`   `construction`
    * `[ ]`   The composition root writes its encoding-dependent work once as an `IEncodingConsumer`, generic over `E: IEncoderAdapter + IDecoderAdapter`, and calls `create_encoding` with `CreateEncodingDeps { consumer }`, `CreateEncodingParams` holding the concrete the configuration names and the identifier the hash-card or configuration requires, and `CreateEncodingPayload`; no consumer constructs or names a concrete

  * `[ ]`   `adapters/encoding/src/factory/mod.rs`
    * `[ ]`   Adds `#[cfg(test)] mod test;` to the wiring `encoding/derivation_context` authored
    * `[ ]`   `pub fn create_encoding<C: IEncodingConsumer>(deps: &CreateEncodingDeps<C>, params: CreateEncodingParams, _payload: CreateEncodingPayload) -> CreateEncodingReturn<C::Output>`, a `match` on `params.concrete` whose `EncodingConcrete::Abi` arm returns the refusal when `AbiEncoding::DECLARATION.identifier` is not `params.identifier`, then binds the concrete by `let Ok(adapter) = AbiEncoding::try_new(AbiEncodingConstructorParams);` and returns `Ok(CreateEncodingSuccessReturn { output: deps.consumer.consume_encoding(ConsumeEncodingParams, ConsumeEncodingPayload { adapter, declaration: AbiEncoding::DECLARATION }) })`
    * `[ ]`   Imports `AbiEncoding` and `AbiEncodingConstructorParams` from `crate::abi::provides`, and this module's types from `interface`
    * `[ ]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[ ]`   `adapters/encoding/src/factory/provides.rs`
    * `[ ]`   Adds `pub use super::create_encoding;` to the re-exports `encoding/derivation_context` authored

  * `[ ]`   `adapters/encoding/tests/integration_test.rs`
    * `[ ]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`; imports `create_encoding`, `CreateEncodingDeps`, `CreateEncodingPayload`, `build_create_encoding_params`, `CreateEncodingParamsOverrides`, `EncodingConcrete`, `EncodingIdentifier`, `IEncodingConsumer`, `IEncoderAdapter`, `IDecoderAdapter`, `ConsumeEncodingParams`, `ConsumeEncodingPayload`, `EncodeParams`, `DecodeParams`, `DerivationContextDescription`, and `DerivationContextDescriptionConstructorParams` from `encoding`, the builders reached through the crate's `mocks` feature, which the workspace's test and check commands enable with `--all-features`; `DerivationContext` and the domain builders and overrides that make the reference context from `domain`; and `hex::decode`
    * `[ ]`   A test-local `RoundTripResult`, a struct with `bytes: Vec<u8>` and `described: DerivationContext`
    * `[ ]`   A test-local `RoundTripCheck`, a struct with `context: DerivationContext`, implementing `IEncodingConsumer` with `type Output = RoundTripResult;` and a `consume_encoding<E: IEncoderAdapter + IDecoderAdapter>` that, through `payload.adapter` alone, constructs `DerivationContextDescription` by `let Ok(description) = DerivationContextDescription::try_new(DerivationContextDescriptionConstructorParams);`, encodes `self.context` with `EncodeParams { description: &description }`, decodes the encoded bytes with `DecodeParams { description: &description }`, each call unpacked by `let Ok(…) = … else { panic!(…) };`, and returns the encoded bytes and the decoded context
    * `[ ]`   The reference context and the reference vector are those `encoding/abi`'s tests state
    * `[ ]`   `the_ethereum_abi_concrete_from_the_factory_round_trips_the_reference_context_through_the_family_traits`: contract: the ABI concrete the factory constructs, admitted for the Ethereum ABI identifier, encodes the reference context to the known-answer vector and decodes it back through `IEncoderAdapter`, `IDecoderAdapter`, and the description, with nothing mocked; arrange `build_create_encoding_params` with `concrete: Some(EncodingConcrete::Abi)` and `identifier: Some(EncodingIdentifier::EthereumAbiV1)`, and `CreateEncodingDeps { consumer: RoundTripCheck { context } }` holding the reference context; act `create_encoding(&deps, params, CreateEncodingPayload)`, unpacked by `let Ok(success) = … else { panic!(…) };`; assert `success.output.bytes` equals the reference vector and `success.output.described` equals the reference context, built again through the same builders
    * `[ ]`   The test block carries the full `Contract`, `Arrange`, `Act`, `Assert` header and the inline markers; nothing is mocked, since `alloy` is the outer edge

  * `[ ]`   `directionality`
    * `[ ]`   The `factory` module depends on the `abi` concrete through `crate::abi::provides` and on its own interface; `abi` depends on the `factory` module's surface, the family form's recorded cycle; `derivation_context` depends on the `factory` module's surface and nothing in the `factory` module names it; the crate's public surface is the `factory` and `derivation_context` modules' `provides`; nothing depends on the crate yet
    * `[ ]`   `kdf/blake3_keyed` consumes the family through `create_encoding` and an `IEncodingConsumer`

  * `[ ]`   `requirements`
    * `[ ]`   `create_encoding_hands_the_consumer_the_abi_concrete_and_its_declaration` passes; the unsupported-identifier refusal is fixed by the return union and the `match` and has no unit test until a further identifier exists
    * `[ ]`   `the_ethereum_abi_concrete_from_the_factory_round_trips_the_reference_context_through_the_family_traits` passes (CR-11, the derivation context's frozen serialization reached through the family's surface)
    * `[ ]`   `cargo check --workspace --all-targets --all-features`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo fmt --all --check`, and `cargo deny check` complete without error or warning in every target, the `abi` concrete's unused-item warnings having no remaining cause
    * `[ ]`   `alloy` is named nowhere outside `adapters/encoding/src/abi`, and no code outside `adapters/encoding` can name `AbiEncoding`

* `[ ]`   `kdf/blake3_keyed` **BLAKE3 keyed-derivation concrete deriving a key of a requested length from secret key material under an encoded context and a fixed context string per purpose; creates the `adapters/kdf` crate and authors the key-derivation family's generic interface, the KDF identifier, the declaration, and the mock**

  * `[ ]`   `objective`
    * `[ ]`   Problem: every derivation a client performs off chain, the wrapping key, the publisher lineage's roots, master scalar, identity bases, capsule randomness, and piece-group keys, and the keyed plaintext-root key, is a domain-separated derivation of secret key material under a context, so the derivation passes through one repo-owned interface, its context strings, input serialization, and output lengths are frozen, and no module outside a concrete names the hash library (CR-05; CR-11; `docs/research/cryptography.md`'s Credential KEM statement, Publisher lineage, and Plaintext-root disclosure modes)
    * `[ ]`   Functional: the family's generic interface derives, for a named purpose and a requested length, a key from borrowed secret key material and the canonical encoding of a context, and returns the key inside a `Secret`
    * `[ ]`   Functional: the purposes are those the specification names: wrapping key, publisher root, asset root, master scalar, identity bases, capsule randomness, piece-group key, and plaintext-root key; each maps, in every concrete, to that concrete's fixed domain separation for the purpose
    * `[ ]`   Functional: every concrete declares the KDF identifier a deployment's hash-card names, its adapter version, and the interface version it implements, readable before any instance exists
    * `[ ]`   Functional: the BLAKE3 concrete derives in BLAKE3's derive-key mode under the purpose's context string, over the key material serialized as the secret's length as an 8-byte big-endian integer, then the secret, then the encoded context, and reads the extendable output to the requested length
    * `[ ]`   Functional: the concrete's output for every purpose matches the known-answer vector computed by an independent implementation over the reference key material, and a shorter output is the prefix of a longer one
    * `[ ]`   Non-functional: `blake3` is named only inside `adapters/kdf/src/blake3_keyed`; the hasher and the output reader are zeroized after every derivation; the crate depends on no repository crate but `domain` at runtime

  * `[ ]`   `role`
    * `[ ]`   Adapter: the key-derivation family's first concrete, and the first source file that requires the family's generic interface, the purpose enum, the KDF identifier, the declaration, and the family's mock, which it authors in the family's `factory` module as its producers
    * `[ ]`   Creates `factory/mod.rs` with the module wiring alone and `factory/provides.rs` with the interface and mock surface alone; the factory function, its types, its interaction spec, its unit test, its re-export, and the family's integration test are `kdf/factory`'s
    * `[ ]`   Does not encode any context; each caller encodes its context through the encoding factory and hands the canonical bytes to the derivation
    * `[ ]`   Does not sample a scalar, wrap a key, or compute a plaintext root; `workflows/sidecar/wrap`, the pairing family's sampling bound, and the commitment layer consume the derived bytes
    * `[ ]`   Does not map the KDF identifier to the hash-card's `kdfId` byte; the hash-card's own encoding does that
    * `[ ]`   Does not carry a commit

  * `[ ]`   `module`
    * `[ ]`   Bounded context: the `adapters/kdf` crate's `factory` module, holding the family's generic trait, the derivation purpose enum, the method's params, payload, success, error, and return types, the KDF identifier, the declaration, the interface version, and the family's mock; and its private `blake3_keyed` concrete, holding the adapter over `blake3`, its context string per purpose, its constructor params and return, and its derivation error
    * `[ ]`   Creates the crate at `adapters/kdf`, admitted by the workspace's `adapters/*` glob with no edit to the root manifest
    * `[ ]`   Outside: every context type and its encoding, what each caller does with the derived bytes, and the factory's selection of a concrete

  * `[ ]`   `deps`
    * `[ ]`   `domain`, `crates/domain`, protocol and domain ring, path dependency; supplies `Secret` and `SecretConstructorParams`, the wrapper for the key material and the derived key; direction inward, adapter ring on domain ring
    * `[ ]`   `domain` with its `mocks` feature, as a dev-dependency and through this crate's `mocks` feature; supplies `build_secret` and `SecretConstructorParamsOverrides`
    * `[ ]`   `zeroize` `1.9.0`, external crate, Apache-2.0 OR MIT, runtime dependency; supplies the `Zeroize` trait whose `zeroize` the concrete calls on the hasher and the output reader
    * `[ ]`   `blake3` `1.8.7`, external crate, CC0-1.0 OR Apache-2.0 OR Apache-2.0 WITH LLVM-exception, runtime dependency with the `zeroize` feature, named only in `blake3_keyed`; supplies `blake3::Hasher` and `blake3::OutputReader`, each implementing `Zeroize` under that feature
    * `[ ]`   `hex` `0.4.3`, external crate, MIT OR Apache-2.0, dev-dependency only; supplies `hex::decode` for the vectors
    * `[ ]`   The encoding family's known-answer vector for the reference derivation context, as `encoding/abi` states it, is the encoded context in the vectors; the crate does not depend on `adapters/encoding`, since the derivation takes the canonical encoding as bytes
    * `[ ]`   `core::convert::Infallible`, standard library, the constructor's error arm; `TryFrom<usize> for u64`, standard library, the length prefix
    * `[ ]`   No reverse dependency beyond the family form's recorded cycle: the `factory` module's error carries this concrete's error; nothing depends on the crate yet

  * `[ ]`   `context_slice`
    * `[ ]`   From `domain`: `Secret::try_new(SecretConstructorParams { value })` returning `Result<Secret<T>, Infallible>`, and `Secret::expose(&self) -> &T`
    * `[ ]`   From `domain`'s mocks: `build_secret(SecretConstructorParamsOverrides<T>) -> Secret<T>` for `T: Zeroize + Default`, instantiated at `Vec<u8>`
    * `[ ]`   From `blake3`: `Hasher::new_derive_key(context: &str) -> Hasher`, `Hasher::update(&mut self, input: &[u8]) -> &mut Hasher`, `Hasher::finalize_xof(&self) -> OutputReader`, and `OutputReader::fill(&mut self, buf: &mut [u8])`
    * `[ ]`   From `zeroize`: `Zeroize::zeroize(&mut self)` on `Hasher` and `OutputReader`

  * `[ ]`   `adapters/kdf/Cargo.toml`
    * `[ ]`   `[package]` with `name = "kdf"`, `edition.workspace = true`, `rust-version.workspace = true`, and `publish.workspace = true`; no `version` key
    * `[ ]`   `[dependencies]` with `domain = { path = "../../crates/domain" }`, `zeroize = "1.9.0"`, and `blake3 = { version = "1.8.7", features = ["zeroize"] }`
    * `[ ]`   `[dev-dependencies]` with `domain = { path = "../../crates/domain", features = ["mocks"] }` and `hex = "0.4.3"`
    * `[ ]`   `[features]` with `mocks = ["domain/mocks"]`
    * `[ ]`   `[lints]` with `workspace = true`
    * `[ ]`   No other table

  * `[ ]`   `adapters/kdf/src/lib.rs`
    * `[ ]`   The crate barrel: `mod blake3_keyed;`, `mod factory;`, and `pub use factory::provides::*;`, nothing else
    * `[ ]`   Until `factory/mod.rs` and `blake3_keyed/mod.rs` exist, `cargo check` reports the unresolved modules, which is the RED state for every element below that precedes them

  * `[ ]`   `adapters/kdf/src/factory/interface.rs`
    * `[ ]`   `KDF_INTERFACE_VERSION`, a `pub const` of type `u32` with value `1`
    * `[ ]`   `KdfIdentifier`, an enum with the one variant `Blake3KeyedV1`, the identifier a deployment's hash-card names
    * `[ ]`   `KdfDeclaration`, a struct with `pub identifier: KdfIdentifier`, `pub adapter_version: u32`, and `pub interface_version: u32`
    * `[ ]`   `DerivationPurpose`, an enum with the variants `WrappingKey`, `PublisherRoot`, `AssetRoot`, `MasterScalar`, `IdentityBases`, `CapsuleRandomness`, `PieceGroupKey`, and `PlaintextRootKey`, the derivation's use, which selects its domain separation
    * `[ ]`   `DeriveKeyParams`, a struct with `pub purpose: DerivationPurpose` and `pub length: usize`, the selection of the derivation and the number of bytes to derive
    * `[ ]`   `DeriveKeyPayload<'a>`, a struct with `pub key_material: &'a Secret<Vec<u8>>` and `pub context: &'a [u8]`, the secret input and the canonical encoding of the context, both borrowed so a caller derives several keys from one secret without copying it
    * `[ ]`   `DeriveKeySuccessReturn`, a struct with `pub key: Secret<Vec<u8>>`
    * `[ ]`   `DeriveKeyErrorReturn`, an enum with the one variant `Blake3Keyed(Blake3KeyedKdfDeriveKeyErrorReturn)`, the BLAKE3 concrete's error carried unchanged; each further concrete's error is its own variant
    * `[ ]`   `DeriveKeyReturn`, the alias `Result<DeriveKeySuccessReturn, DeriveKeyErrorReturn>`
    * `[ ]`   `IKeyDerivationAdapter`, a trait with the one method `fn derive_key(&self, params: DeriveKeyParams, payload: DeriveKeyPayload<'_>) -> DeriveKeyReturn;`
    * `[ ]`   No derives on any type in this file; imports `domain::Secret` and `Blake3KeyedKdfDeriveKeyErrorReturn` from `crate::blake3_keyed::provides`; names no vendor

  * `[ ]`   `adapters/kdf/src/blake3_keyed/interface.rs`
    * `[ ]`   `Blake3KeyedKdf`, the unit struct `pub struct Blake3KeyedKdf;`, the adapter over `blake3`'s derive-key mode
    * `[ ]`   `Blake3KeyedKdfConstructorParams`, the fieldless struct `pub struct Blake3KeyedKdfConstructorParams;`, the constructor's deps slot
    * `[ ]`   `Blake3KeyedKdfTryNewReturn`, the alias `Result<Blake3KeyedKdf, Infallible>`; the error arm is uninhabited because the adapter takes no configuration
    * `[ ]`   The context strings, each a `pub const` of type `&str`: `BLAKE3_KEYED_WRAPPING_KEY_CONTEXT` with value `"ChainTorrent v1 wrapping-key"`, `BLAKE3_KEYED_PUBLISHER_ROOT_CONTEXT` with value `"ChainTorrent v1 publisher-root"`, `BLAKE3_KEYED_ASSET_ROOT_CONTEXT` with value `"ChainTorrent v1 asset-root"`, `BLAKE3_KEYED_MASTER_SCALAR_CONTEXT` with value `"ChainTorrent v1 master-scalar"`, `BLAKE3_KEYED_IDENTITY_BASES_CONTEXT` with value `"ChainTorrent v1 identity-bases"`, `BLAKE3_KEYED_CAPSULE_RANDOMNESS_CONTEXT` with value `"ChainTorrent v1 capsule-randomness"`, `BLAKE3_KEYED_PIECE_GROUP_KEY_CONTEXT` with value `"ChainTorrent v1 piece-group-key"`, and `BLAKE3_KEYED_PLAINTEXT_ROOT_KEY_CONTEXT` with value `"ChainTorrent v1 plaintext-root"`
    * `[ ]`   `Blake3KeyedKdfDeriveKeyErrorReturn`, an enum with the one variant `KeyMaterialLengthExceedsPrefix { length: usize }`, a key material length the 8-byte prefix cannot hold
    * `[ ]`   Imports `core::convert::Infallible`; declares nothing else

  * `[ ]`   `adapters/kdf/src/blake3_keyed/interaction.spec.md`
    * `[ ]`   `Blake3KeyedKdf::try_new(params: Blake3KeyedKdfConstructorParams) -> Blake3KeyedKdfTryNewReturn`: one branch; condition any params; decision none; dependency call none; outcome `Ok(Blake3KeyedKdf)`; the error arm has no branch
    * `[ ]`   `Blake3KeyedKdf::DECLARATION`: the inherent constant `KdfDeclaration { identifier: KdfIdentifier::Blake3KeyedV1, adapter_version: 1, interface_version: KDF_INTERFACE_VERSION }`
    * `[ ]`   Purpose mapping: a `match` on `params.purpose`, exhaustive, each variant to its context string constant, `WrappingKey` to `BLAKE3_KEYED_WRAPPING_KEY_CONTEXT`, `PublisherRoot` to `BLAKE3_KEYED_PUBLISHER_ROOT_CONTEXT`, `AssetRoot` to `BLAKE3_KEYED_ASSET_ROOT_CONTEXT`, `MasterScalar` to `BLAKE3_KEYED_MASTER_SCALAR_CONTEXT`, `IdentityBases` to `BLAKE3_KEYED_IDENTITY_BASES_CONTEXT`, `CapsuleRandomness` to `BLAKE3_KEYED_CAPSULE_RANDOMNESS_CONTEXT`, `PieceGroupKey` to `BLAKE3_KEYED_PIECE_GROUP_KEY_CONTEXT`, and `PlaintextRootKey` to `BLAKE3_KEYED_PLAINTEXT_ROOT_KEY_CONTEXT`
    * `[ ]`   `derive_key`, prefix overflow: condition `u64::try_from(payload.key_material.expose().len())` fails; decision the conversion, before any hashing; dependency call none; outcome `Err(DeriveKeyErrorReturn::Blake3Keyed(Blake3KeyedKdfDeriveKeyErrorReturn::KeyMaterialLengthExceedsPrefix { length }))`; `usize` is at most 64 bits on every supported target, so no input takes this branch and it has no unit test
    * `[ ]`   `derive_key`, derived: condition the length converts; decision none further; dependency calls `Hasher::new_derive_key` with the purpose's context string, then `update` with the length's 8 big-endian bytes, `update` with the exposed key material, and `update` with `payload.context`, in that order, each once; then `finalize_xof` once and `fill` once over a zero-initialized buffer of `params.length` bytes; then `zeroize` on the output reader and on the hasher; outcome `Ok(DeriveKeySuccessReturn { key })`, the filled buffer moved into a `Secret` without copy
    * `[ ]`   Ordering: the length conversion precedes the hasher; the prefix, the key material, and the context are absorbed in that order; the reader and the hasher are zeroized before the buffer is moved into the `Secret`; the same params and payload always yield the same key; a `params.length` of zero yields an empty key

  * `[ ]`   `adapters/kdf/src/factory/mock.rs`
    * `[ ]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`
    * `[ ]`   `KdfDeclarationOverrides`, `#[derive(Default)]`, one `Option` per field; `build_kdf_declaration(overrides: KdfDeclarationOverrides) -> KdfDeclaration`, defaulting to `KdfIdentifier::Blake3KeyedV1`, `1`, and `KDF_INTERFACE_VERSION`
    * `[ ]`   `DeriveKeyParamsOverrides`, `#[derive(Default)]`, fields `pub purpose: Option<DerivationPurpose>` and `pub length: Option<usize>`; `build_derive_key_params(overrides: DeriveKeyParamsOverrides) -> DeriveKeyParams`, defaulting to `DerivationPurpose::WrappingKey` and `32`
    * `[ ]`   `DeriveKeySuccessReturnOverrides`, `#[derive(Default)]`, one field `pub key: Option<Secret<Vec<u8>>>`; `build_derive_key_success_return(overrides: DeriveKeySuccessReturnOverrides) -> DeriveKeySuccessReturn`, the key defaulting to `build_secret::<Vec<u8>>(SecretConstructorParamsOverrides::default())`, an empty key
    * `[ ]`   `MockIKeyDerivationAdapter`, the unit struct `pub struct MockIKeyDerivationAdapter;`, implementing `IKeyDerivationAdapter` with `derive_key` returning `Ok(build_derive_key_success_return(Default::default()))` for any params and payload; a test needing other behavior implements the trait on its own local struct
    * `[ ]`   No builder for `DeriveKeyPayload`, whose fields are borrowed with no default and are written at the call site as its production value, or for the enums `KdfIdentifier`, `DerivationPurpose`, and `DeriveKeyErrorReturn`; no corruptions type and no invalidator, since no value this interface owns arrives as untrusted data
    * `[ ]`   Imports `Secret`, `build_secret`, and `SecretConstructorParamsOverrides` from `domain`, and this module's types from `super::interface`

  * `[ ]`   `adapters/kdf/src/factory/mod.rs`
    * `[ ]`   Module wiring only: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, and `pub mod provides;`, nothing else

  * `[ ]`   `adapters/kdf/src/factory/provides.rs`
    * `[ ]`   `pub use super::interface::*;` and `#[cfg(any(test, feature = "mocks"))] pub use super::mock::*;`, nothing else

  * `[ ]`   `adapters/kdf/src/blake3_keyed/test.rs`
    * `[ ]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`; imports `Blake3KeyedKdf` and `Blake3KeyedKdfConstructorParams` from `super::interface`; `IKeyDerivationAdapter`, `DerivationPurpose`, `DeriveKeyPayload`, `build_derive_key_params`, `DeriveKeyParamsOverrides`, `KdfIdentifier`, and `KDF_INTERFACE_VERSION` from `crate::factory::provides`; `build_secret` and `SecretConstructorParamsOverrides` from `domain`; and `hex::decode`
    * `[ ]`   Each test constructs the subject by `let Ok(kdf) = Blake3KeyedKdf::try_new(Blake3KeyedKdfConstructorParams);`, decodes hex by `let Ok(bytes) = decode(…) else { panic!(…) };`, passes `DeriveKeyPayload { key_material: &secret, context: &context }`, and unpacks a derivation by `let Ok(success) = … else { panic!(…) };`
    * `[ ]`   The reference key material: the secret from `build_secret` with `value: Some(vec![0x42u8; 32])`, and the context the reference vector `encoding/abi` states, the 448-byte ABI encoding of the reference derivation context, written as one hex string of its words in order
    * `[ ]`   The independent vectors: the first 64 bytes of BLAKE3 derive-key output under each context string over the 488-byte key material `0000000000000020`, then 32 bytes of `42`, then the reference context, computed with the pure-Python BLAKE3 implementation `pure_blake3.py` of `oconnor663/pure_python_blake3`, which reproduces every derive-key case of BLAKE3's official `test_vectors.json`: wrapping key `549874404f0b25b65da55dbce0b410d0aa34085695af4b91303e2c1c0410f760a75cb805271fbaf45eb7a1fa33596b3cb082ec41f923d4fb7055ba82699ec4f3`; publisher root `63da685dcbcd9267e8c95361ece3aa6eff7eade6434f8087595e912d41c5abc0d11d8303de6cfa4401b1f192f5ffef30a68fead745af576967b85622951acc58`; asset root `a544b18f012d94c3c1f29cf96563e97b8b6fec6bd076d228a6967e901b4c6ed7257efd1bce6216a1cfd567d2bb8cceb5d67b2fe268777020cb8c5a34bd752801`; master scalar `453eae1adf266f0b865dad65a3adbbc74fd09df871842cd11ca8da7ea896d82d3b96ee39c0d01eeac7b75fde8b07353f73230ec3f21f75439bfb29b72c49cd74`; identity bases `5a2257c26fb7b1e04621baaa23b8a360757b5de336b3aeec6d6562b6d815d721b0e05eec78d8338aad18a60f4eed88459601eed0d17cb6fc54a72a8665bb3864`; capsule randomness `83fea48ca2ce75442b981ab2d9ea1260abadaf7118e022d9e32ea621146a0b2639f3a3cf913284fb6835f1d07aab04fbd483111a2e91c4af6b4991e7faa1966f`; piece-group key `f33f881d8d69557235873979f03f0d3e10479c14ddac800785507536cffacd6b893a812db4e36d41ba907b62d7dd1dec19a4c2cff0cbd47218ee6495d1a09e1f`; plaintext-root key `d8e66fc2126c63ded90ee602192d85f9f1485e2e13b9a310d18846b02aa7e022dfbe765c77fa87014c9b15086629c23609d9968e5e0e3cb5cb5fbae4f047b404`
    * `[ ]`   `derive_key_for_the_wrapping_key_matches_the_independent_vector`: contract: the wrapping-key purpose's context string, the serialization, and the output match the independent implementation; arrange the reference key material and `build_derive_key_params` with `purpose: Some(DerivationPurpose::WrappingKey)` and `length: Some(64)`; act `kdf.derive_key(params, payload)`; assert `success.key.expose()` equals the wrapping-key vector
    * `[ ]`   `derive_key_for_the_publisher_root_matches_the_independent_vector`, `derive_key_for_the_asset_root_matches_the_independent_vector`, `derive_key_for_the_master_scalar_matches_the_independent_vector`, `derive_key_for_the_identity_bases_matches_the_independent_vector`, `derive_key_for_the_capsule_randomness_matches_the_independent_vector`, `derive_key_for_the_piece_group_key_matches_the_independent_vector`, and `derive_key_for_the_plaintext_root_key_matches_the_independent_vector`: the same with `DerivationPurpose::PublisherRoot`, `AssetRoot`, `MasterScalar`, `IdentityBases`, `CapsuleRandomness`, `PieceGroupKey`, and `PlaintextRootKey` against their vectors
    * `[ ]`   `derive_key_of_thirty_two_bytes_is_the_prefix_of_the_sixty_four_byte_vector`: contract: the requested length truncates the extendable output; arrange the reference key material and `purpose: Some(DerivationPurpose::WrappingKey)` with `length: Some(32)`; act `derive_key`; assert `success.key.expose()` equals `549874404f0b25b65da55dbce0b410d0aa34085695af4b91303e2c1c0410f760`
    * `[ ]`   `derive_key_of_zero_bytes_returns_an_empty_key`: arrange the reference key material and `length: Some(0)`; act `derive_key`; assert `success.key.expose().is_empty()`
    * `[ ]`   `derive_key_binds_the_key_material_length`: contract: moving a byte from the secret to the context changes the derivation, because the secret's length is absorbed first; arrange the secret from `build_secret` with `value: Some(vec![0x42u8; 31])`, the context `42` followed by the reference context, and `purpose: Some(DerivationPurpose::WrappingKey)` with `length: Some(64)`; act `derive_key`; assert `success.key.expose()` differs from the wrapping-key vector
    * `[ ]`   `blake3_keyed_kdf_declares_its_identifier_and_versions`: act read `Blake3KeyedKdf::DECLARATION`; assert `identifier` matches `KdfIdentifier::Blake3KeyedV1`, `adapter_version` equals `1`, and `interface_version` equals `KDF_INTERFACE_VERSION`
    * `[ ]`   Every test block carries the full `Contract`, `Arrange`, `Act`, `Assert` header and the inline markers; every params value is built through `build_derive_key_params` with only the overrides the test depends on

  * `[ ]`   `construction`
    * `[ ]`   `Blake3KeyedKdf::try_new` is the concrete's only producer, and its only caller is the key-derivation factory, which reads `Blake3KeyedKdf::DECLARATION` before constructing
    * `[ ]`   The concrete owns no object type a consumer builds as a fixture: the adapter is a unit struct constructed by `try_new` over fieldless params, its context strings are constants, and its error is an enum, so it has no `mock.rs`

  * `[ ]`   `adapters/kdf/src/blake3_keyed/mod.rs`
    * `[ ]`   Module declarations: `mod interface;`, `pub(crate) mod provides;`, and `#[cfg(test)] mod test;`
    * `[ ]`   `impl Blake3KeyedKdf` with `pub const DECLARATION: KdfDeclaration` as the interaction spec states and `pub fn try_new(_params: Blake3KeyedKdfConstructorParams) -> Blake3KeyedKdfTryNewReturn` returning `Ok(Blake3KeyedKdf)`
    * `[ ]`   `impl IKeyDerivationAdapter for Blake3KeyedKdf` with `derive_key` realizing the purpose mapping, branches, and ordering of the interaction spec; the buffer is `vec![0u8; params.length]`, and it is moved into the `Secret` by `let Ok(key) = Secret::try_new(SecretConstructorParams { value: buffer });`
    * `[ ]`   Imports the family's names from `crate::factory::provides`, `Secret` and `SecretConstructorParams` from `domain`, `zeroize::Zeroize`, `blake3::Hasher`, and this module's names from `interface`
    * `[ ]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[ ]`   `adapters/kdf/src/blake3_keyed/provides.rs`
    * `[ ]`   `pub(crate) use super::interface::*;`, nothing else, so the concrete is visible to the crate's factory and to nothing outside the crate

  * `[ ]`   `directionality`
    * `[ ]`   `blake3_keyed` depends on the `factory` module's surface through `crate::factory::provides`, on `domain`, on `zeroize`, and on `blake3`; the `factory` module depends on `domain` and on `blake3_keyed`'s error through `crate::blake3_keyed::provides`, the family form's recorded cycle, which `kdf/factory` completes by constructing the concrete; among repository crates the crate depends on `crates/domain` alone; nothing depends on the crate yet

  * `[ ]`   `requirements`
    * `[ ]`   `adapters/kdf/Cargo.toml` carries exactly the tables and keys stated above, and `blake3` is named nowhere in the crate outside `adapters/kdf/src/blake3_keyed`
    * `[ ]`   `cargo check --workspace --all-targets --all-features`, `cargo fmt --all --check`, and `cargo deny check` complete without error; `cargo clippy --workspace --all-targets --all-features` reports nothing beyond the library target's unused-item warnings for the `blake3_keyed` concrete, which `kdf/factory` resolves by constructing the concrete
    * `[ ]`   Every `derive_key_for_…_matches_the_independent_vector` test passes (CR-11, context strings, serialization, and output frozen against an independent implementation's vectors; CR-05 for the lineage purposes)
    * `[ ]`   `derive_key_of_thirty_two_bytes_is_the_prefix_of_the_sixty_four_byte_vector`, `derive_key_of_zero_bytes_returns_an_empty_key`, and `derive_key_binds_the_key_material_length` pass
    * `[ ]`   `blake3_keyed_kdf_declares_its_identifier_and_versions` passes
    * `[ ]`   Code outside `adapters/kdf` naming `Blake3KeyedKdf` or anything under `blake3_keyed` fails to compile; the crate's public surface is the `factory` module's `provides`

* `[ ]`   `kdf/factory` **Key-derivation factory constructing the concrete the configuration names, admitted against the KDF identifier the hash-card requires, and returning it behind the family's trait with its declaration; carries the family's integration test**

  * `[ ]`   `objective`
    * `[ ]`   Problem: a consumer obtains a key-derivation adapter only through the family's generic surface, never by naming a concrete, and the derivation it uses is the one the deployment's hash-card names, so a concrete that does not declare the required KDF identifier is refused before anything is constructed (CR-11; Composition Boundary)
    * `[ ]`   Functional: given the concrete the configuration names and the KDF identifier required, the factory refuses a concrete whose declared identifier is not the required one, with no construction
    * `[ ]`   Functional: an admitted concrete is constructed and returned as `Box<dyn IKeyDerivationAdapter>` together with its declaration
    * `[ ]`   Functional: a concrete's constructor error is returned unchanged in the factory's error arm, one variant per concrete
    * `[ ]`   Functional: the concrete the factory returns derives the wrapping key over the reference context encoded through the encoding factory, matching the independent vector
    * `[ ]`   Non-functional: adding a concrete is its module, its variant in the selection enum and in the error enum, and its branch here; adding an identifier is its variant in `KdfIdentifier`; no consumer changes

  * `[ ]`   `role`
    * `[ ]`   Adapter family factory: the implementation of the `factory` module, the key-derivation family's construction point, and the crate's public surface
    * `[ ]`   Returns the concrete behind `Box<dyn IKeyDerivationAdapter>`, because the family's trait has no generic method and no associated type and so is dyn-compatible, as the randomness factory returns its source
    * `[ ]`   Selects by concrete and admits by identifier, so a further concrete implementing an existing identifier and a further identifier each arrive as a variant and a branch, with no change to the params or to any consumer
    * `[ ]`   Does not read a hash-card or the configuration; the composition resolver passes the concrete the configuration names as a typed `KdfConcrete` and the identifier the hash-card requires as a typed `KdfIdentifier`
    * `[ ]`   Does not derive; derivation is the concrete's
    * `[ ]`   Carries the family's integration test across factory, concrete, and the encoding family that produces the context; does not carry a commit

  * `[ ]`   `module`
    * `[ ]`   Bounded context: the `factory` module of `adapters/kdf`, holding the factory function, its deps, params, payload, and return types, its signature type, the selection enum, the function mock and builders, and the crate's integration test under `adapters/kdf/tests`
    * `[ ]`   Outside: the concrete's behavior, the encoding of any context, the hash-card, the configuration catalogue, and every consumer of the family

  * `[ ]`   `deps`
    * `[ ]`   The `blake3_keyed` concrete, through `crate::blake3_keyed::provides`: `Blake3KeyedKdf`, `Blake3KeyedKdfConstructorParams`, `Blake3KeyedKdf::try_new`, and `Blake3KeyedKdf::DECLARATION`; the factory constructs its concrete, completing the family form's recorded cycle that `kdf/blake3_keyed` opened
    * `[ ]`   The `factory` module's own interface: `IKeyDerivationAdapter`, `KdfIdentifier`, `KdfDeclaration`, and `KDF_INTERFACE_VERSION`
    * `[ ]`   `encoding`, `adapters/encoding`, adapter ring, dev-dependency with its `mocks` feature, in the integration test only; supplies `create_encoding` and the names that encode the reference derivation context through the encoding family, the edge the dependency map draws from `encoding/factory` to this family; nothing at runtime, and the encoding crate names nothing in this crate
    * `[ ]`   `domain` with its `mocks` feature and `hex`, the existing dev-dependencies, in the integration test only: the builders that make the reference context and the reference secret, and `hex::decode` for the vector
    * `[ ]`   `core::convert::Infallible`, standard library, the concrete's constructor error carried in the factory's error arm

  * `[ ]`   `context_slice`
    * `[ ]`   From the concrete: `Blake3KeyedKdf::try_new(Blake3KeyedKdfConstructorParams) -> Result<Blake3KeyedKdf, Infallible>`, the inherent constant `Blake3KeyedKdf::DECLARATION: KdfDeclaration`, and `Blake3KeyedKdf`'s implementation of `IKeyDerivationAdapter`
    * `[ ]`   From `encoding`, in the integration test: `create_encoding<C: IEncodingConsumer>(&CreateEncodingDeps<C>, CreateEncodingParams, CreateEncodingPayload)`, `build_create_encoding_params` with `CreateEncodingParamsOverrides`, `EncodingConcrete::Abi`, `EncodingIdentifier::EthereumAbiV1`, `IEncodingConsumer`, `IEncoderAdapter::encode<D: IEncodingContract>(&self, EncodeParams { description }, &D::Described)`, `ConsumeEncodingParams`, `ConsumeEncodingPayload`, and `DerivationContextDescription::try_new(DerivationContextDescriptionConstructorParams)`

  * `[ ]`   `adapters/kdf/Cargo.toml`
    * `[ ]`   `[dev-dependencies]` reads `domain = { path = "../../crates/domain", features = ["mocks"] }`, `encoding = { path = "../encoding", features = ["mocks"] }`, and `hex = "0.4.3"`
    * `[ ]`   `[package]`, `[dependencies]`, `[features]`, and `[lints]` are unchanged; no other table

  * `[ ]`   `adapters/kdf/src/factory/interface.rs`
    * `[ ]`   `KdfIdentifier` gains `#[derive(PartialEq, Eq)]`, so the admission compares a declared identifier with the required one
    * `[ ]`   `KdfConcrete`, an enum with the one variant `Blake3Keyed`, the selection of the concrete to construct
    * `[ ]`   `CreateKeyDerivationDeps`, the fieldless struct `pub struct CreateKeyDerivationDeps;`
    * `[ ]`   `CreateKeyDerivationParams`, a struct with `pub concrete: KdfConcrete` and `pub identifier: KdfIdentifier`, the selection and the identifier the concrete must declare
    * `[ ]`   `CreateKeyDerivationPayload`, the fieldless struct `pub struct CreateKeyDerivationPayload;`, since the factory operates on no data
    * `[ ]`   `CreateKeyDerivationSuccessReturn`, a struct with `pub adapter: Box<dyn IKeyDerivationAdapter>` and `pub declaration: KdfDeclaration`
    * `[ ]`   `CreateKeyDerivationErrorReturn`, an enum with the variants `UnsupportedKdfIdentifier`, the named concrete not declaring the required identifier, and `Blake3Keyed(Infallible)`, the BLAKE3 concrete's constructor error carried unchanged; each further concrete's constructor error is its own variant
    * `[ ]`   `CreateKeyDerivationReturn`, the alias `Result<CreateKeyDerivationSuccessReturn, CreateKeyDerivationErrorReturn>`
    * `[ ]`   `CreateKeyDerivationFn`, the alias `fn(&CreateKeyDerivationDeps, CreateKeyDerivationParams, CreateKeyDerivationPayload) -> CreateKeyDerivationReturn`
    * `[ ]`   Adds the import of `core::convert::Infallible`; every item `kdf/blake3_keyed` authored in this file is unchanged except the derive on `KdfIdentifier`

  * `[ ]`   `adapters/kdf/src/factory/interaction.spec.md`
    * `[ ]`   `create_key_derivation(deps: &CreateKeyDerivationDeps, params: CreateKeyDerivationParams, payload: CreateKeyDerivationPayload) -> CreateKeyDerivationReturn`: decision a `match` on `params.concrete`, one arm per `KdfConcrete` variant, exhaustive so a variant with no arm fails to compile
    * `[ ]`   Unsupported identifier: condition the named concrete's `DECLARATION.identifier` is not `params.identifier`; decision equality, read before any construction; dependency call none; outcome `Err(CreateKeyDerivationErrorReturn::UnsupportedKdfIdentifier)`, with nothing constructed; `KdfIdentifier` has the one variant the BLAKE3 concrete declares, so no input takes this branch until a further identifier exists, and it has no unit test
    * `[ ]`   Admitted: condition the named concrete's declared identifier is `params.identifier`; dependency call the concrete's `try_new` with its fieldless constructor params, exactly once, its success destructured irrefutably because its error arm is uninhabited; outcome `Ok(CreateKeyDerivationSuccessReturn { adapter: Box::new(kdf), declaration })` with the concrete's `DECLARATION`
    * `[ ]`   `CreateKeyDerivationErrorReturn::Blake3Keyed` carries the constructor's uninhabited error type in the return union, so no branch produces it
    * `[ ]`   `params.concrete` selects and `params.identifier` admits; `deps` and `payload` carry nothing and are not read

  * `[ ]`   `adapters/kdf/src/factory/mock.rs`
    * `[ ]`   `CreateKeyDerivationParamsOverrides`, `#[derive(Default)]`, fields `pub concrete: Option<KdfConcrete>` and `pub identifier: Option<KdfIdentifier>`; `build_create_key_derivation_params(overrides: CreateKeyDerivationParamsOverrides) -> CreateKeyDerivationParams`, defaulting to `KdfConcrete::Blake3Keyed` and `KdfIdentifier::Blake3KeyedV1`
    * `[ ]`   `CreateKeyDerivationSuccessReturnOverrides`, `#[derive(Default)]`, fields `pub adapter: Option<Box<dyn IKeyDerivationAdapter>>` and `pub declaration: Option<KdfDeclaration>`; `build_create_key_derivation_success_return(overrides: CreateKeyDerivationSuccessReturnOverrides) -> CreateKeyDerivationSuccessReturn`, the adapter defaulting to `Box::new(MockIKeyDerivationAdapter)` and the declaration to `build_kdf_declaration(Default::default())`
    * `[ ]`   `mock_create_key_derivation(_deps: &CreateKeyDerivationDeps, _params: CreateKeyDerivationParams, _payload: CreateKeyDerivationPayload) -> CreateKeyDerivationReturn`, returning `Ok(build_create_key_derivation_success_return(Default::default()))`
    * `[ ]`   No builder for the fieldless `CreateKeyDerivationDeps` and `CreateKeyDerivationPayload`, used by their production values, or for the enums `KdfConcrete` and `CreateKeyDerivationErrorReturn`; every symbol `kdf/blake3_keyed` authored in this file is unchanged

  * `[ ]`   `adapters/kdf/src/factory/test.rs`
    * `[ ]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`; imports `create_key_derivation` from `super`, `CreateKeyDerivationDeps`, `CreateKeyDerivationPayload`, `KdfConcrete`, `KdfIdentifier`, and `KDF_INTERFACE_VERSION` from `super::interface`, and `build_create_key_derivation_params` and `CreateKeyDerivationParamsOverrides` from `super::mock`
    * `[ ]`   `create_key_derivation_returns_the_blake3_keyed_concrete_and_its_declaration`: contract: the BLAKE3 concrete, admitted for its identifier, is returned with its declaration; arrange `build_create_key_derivation_params` with `concrete: Some(KdfConcrete::Blake3Keyed)` and `identifier: Some(KdfIdentifier::Blake3KeyedV1)`; act `create_key_derivation(&CreateKeyDerivationDeps, params, CreateKeyDerivationPayload)`, unpacked by `let Ok(success) = … else { panic!(…) };`; assert `success.declaration.identifier` matches `KdfIdentifier::Blake3KeyedV1`, `success.declaration.adapter_version` equals `1`, and `success.declaration.interface_version` equals `KDF_INTERFACE_VERSION`
    * `[ ]`   The unsupported-identifier branch has no unit test, as the interaction spec states
    * `[ ]`   The test block carries the full `Contract`, `Arrange`, `Act`, `Assert` header and the inline markers

  * `[ ]`   `construction`
    * `[ ]`   The composition root calls `create_key_derivation` with `&CreateKeyDerivationDeps`, `CreateKeyDerivationParams` holding the concrete the configuration names and the identifier the hash-card requires, and `CreateKeyDerivationPayload`, and places the returned `Box<dyn IKeyDerivationAdapter>` in each consumer's deps; no consumer constructs or names a concrete

  * `[ ]`   `adapters/kdf/src/factory/mod.rs`
    * `[ ]`   Adds `#[cfg(test)] mod test;` to the wiring `kdf/blake3_keyed` authored
    * `[ ]`   `pub fn create_key_derivation(_deps: &CreateKeyDerivationDeps, params: CreateKeyDerivationParams, _payload: CreateKeyDerivationPayload) -> CreateKeyDerivationReturn`, a `match` on `params.concrete` whose `KdfConcrete::Blake3Keyed` arm returns the refusal when `Blake3KeyedKdf::DECLARATION.identifier` is not `params.identifier`, then binds the concrete by `let Ok(kdf) = Blake3KeyedKdf::try_new(Blake3KeyedKdfConstructorParams);` and returns `Ok(CreateKeyDerivationSuccessReturn { adapter: Box::new(kdf), declaration: Blake3KeyedKdf::DECLARATION })`
    * `[ ]`   Imports `Blake3KeyedKdf` and `Blake3KeyedKdfConstructorParams` from `crate::blake3_keyed::provides`, and this module's types from `interface`
    * `[ ]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[ ]`   `adapters/kdf/src/factory/provides.rs`
    * `[ ]`   Adds `pub use super::create_key_derivation;` to the re-exports `kdf/blake3_keyed` authored

  * `[ ]`   `adapters/kdf/tests/integration_test.rs`
    * `[ ]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`; imports `create_key_derivation`, `CreateKeyDerivationDeps`, `CreateKeyDerivationPayload`, `build_create_key_derivation_params`, `CreateKeyDerivationParamsOverrides`, `KdfConcrete`, `KdfIdentifier`, `DerivationPurpose`, `DeriveKeyPayload`, `build_derive_key_params`, and `DeriveKeyParamsOverrides` from `kdf`; the `encoding` names the context slice lists with `CreateEncodingDeps`, `CreateEncodingPayload`, and `DerivationContextDescriptionConstructorParams` from `encoding`; `DerivationContext`, the domain builders and overrides that make the reference context, `build_secret`, and `SecretConstructorParamsOverrides` from `domain`; and `hex::decode`; the builders are reached through each crate's `mocks` feature, which the workspace's test and check commands enable with `--all-features`
    * `[ ]`   A test-local `EncodeContext`, a struct with `context: DerivationContext`, implementing `IEncodingConsumer` with `type Output = Vec<u8>;` and a `consume_encoding<E: IEncoderAdapter + IDecoderAdapter>` that constructs `DerivationContextDescription` by `let Ok(description) = DerivationContextDescription::try_new(DerivationContextDescriptionConstructorParams);`, encodes `self.context` through `payload.adapter.encode(EncodeParams { description: &description }, &self.context)`, unpacked by `let Ok(success) = … else { panic!(…) };`, and returns `success.bytes`
    * `[ ]`   The reference context is the one `encoding/derivation_context`'s tests build; the reference secret is `build_secret` with `value: Some(vec![0x42u8; 32])`; the wrapping-key vector is the one `kdf/blake3_keyed`'s tests state
    * `[ ]`   `the_blake3_keyed_concrete_from_the_factory_derives_the_wrapping_key_over_the_context_the_encoding_factory_encodes`: contract: the reference context encoded through the encoding factory, and the wrapping key derived over it through the key-derivation factory's concrete, match the independent vector, with nothing mocked; arrange the context bytes from `create_encoding` with `build_create_encoding_params` overridden by `concrete: Some(EncodingConcrete::Abi)` and `identifier: Some(EncodingIdentifier::EthereumAbiV1)` and `CreateEncodingDeps { consumer: EncodeContext { context } }` holding the reference context, the adapter from `create_key_derivation` with `build_create_key_derivation_params` overridden by `concrete: Some(KdfConcrete::Blake3Keyed)` and `identifier: Some(KdfIdentifier::Blake3KeyedV1)`, and the reference secret, each factory call unpacked by `let Ok(success) = … else { panic!(…) };`; act `adapter.derive_key` with `build_derive_key_params` overridden by `purpose: Some(DerivationPurpose::WrappingKey)` and `length: Some(64)` and `DeriveKeyPayload { key_material: &secret, context: &context }`; assert the derived key's exposed bytes equal the wrapping-key vector
    * `[ ]`   The test block carries the full `Contract`, `Arrange`, `Act`, `Assert` header and the inline markers; nothing is mocked, since `alloy` and `blake3` are the outer edges

  * `[ ]`   `directionality`
    * `[ ]`   The `factory` module depends on the `blake3_keyed` concrete through `crate::blake3_keyed::provides` and on its own interface; `blake3_keyed` depends on the `factory` module's surface, the family form's recorded cycle; the crate's public surface is the `factory` module's `provides`
    * `[ ]`   Among repository crates the crate depends on `crates/domain` at runtime and on `adapters/encoding` for its integration test only; `adapters/encoding` names nothing in this crate; no cycle
    * `[ ]`   `workflows/sidecar/wrap` consumes the family through `create_key_derivation` and `IKeyDerivationAdapter`

  * `[ ]`   `requirements`
    * `[ ]`   `adapters/kdf/Cargo.toml` carries exactly the dev-dependencies stated above, and every other table `kdf/blake3_keyed` stated is unchanged
    * `[ ]`   `create_key_derivation_returns_the_blake3_keyed_concrete_and_its_declaration` passes; the unsupported-identifier refusal is fixed by the return union and the `match` and has no unit test until a further identifier exists
    * `[ ]`   `the_blake3_keyed_concrete_from_the_factory_derives_the_wrapping_key_over_the_context_the_encoding_factory_encodes` passes (CR-11, the wrapping-key derivation over the encoded derivation context reached through both families' surfaces)
    * `[ ]`   `cargo check --workspace --all-targets --all-features`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo fmt --all --check`, and `cargo deny check` complete without error or warning in every target, the `blake3_keyed` concrete's unused-item warnings having no remaining cause
    * `[ ]`   `blake3` is named nowhere outside `adapters/kdf/src/blake3_keyed`, and no code outside `adapters/kdf` can name `Blake3KeyedKdf`

* `[ ]`   `hash-to-scalar/domain_tag` **Domain tag, the byte string that separates one hash-to-scalar use from every other, admitted only when it is non-empty, fits a one-byte length prefix, and is visible ASCII; creates the `adapters/hash-to-scalar` crate**

  * `[ ]`   `objective`
    * `[ ]`   Problem: every value a contract recomputes, the identity mapping and the delivery proof's challenge, is keccak256 under a domain tag reduced modulo the group order, and the client and the contract must hash the same tag in the same byte form, so a tag is a value with one byte form that both sides can state as a constant, refused before anything is hashed under it when it has none (CR-08; CR-09; CR-11; `docs/research/cryptography.md`'s Credential KEM and Delivery Proof statements)
    * `[ ]`   Functional: one type holds a domain tag's bytes, reachable only through a read accessor, and its only producer is a fallible constructor
    * `[ ]`   Functional: the constructor refuses an empty tag, a tag longer than 255 bytes, so its length fits the one-byte prefix the hash-to-scalar concrete absorbs, and any byte outside visible ASCII, `0x21` through `0x7E`
    * `[ ]`   Functional: a refusal names the failed check and the values it failed on, and the same input always yields the same refusal: the length checks precede the byte scan, and within the scan the lowest offending index decides
    * `[ ]`   Non-functional: the module depends on the standard library alone; the crate names no hash library and no pairing library

  * `[ ]`   `role`
    * `[ ]`   Adapter family: the hash-to-scalar family's family-owned value type, and the family's first source file, so it creates the crate; the tag is a family-owned module beside the factory because a type lives in the module that implements it, and the tag's constructor is its implementation
    * `[ ]`   Does not hash, prefix, or reduce; `hash-to-scalar/keccak256` absorbs the tag's length and bytes and reduces the digest
    * `[ ]`   Does not name any use's tag; each consumer that hashes to a scalar, `kem/bb1_depth_one` for the identity mapping and `proof/schnorr_fs/challenge` for the challenge, states its own tag as a constant and constructs it through this type, and the generate family mirrors those constants to Solidity
    * `[ ]`   Does not create the `factory` module; `hash-to-scalar/keccak256` authors the family's generic interface, identifier, declaration, and mock there
    * `[ ]`   Does not carry a commit

  * `[ ]`   `module`
    * `[ ]`   Bounded context: the family-owned `domain_tag` module of the `adapters/hash-to-scalar` crate, holding `DomainTag`, its maximum length, its constructor params, and its constructor's error and return types
    * `[ ]`   Creates the crate at `adapters/hash-to-scalar`, admitted by the workspace's `adapters/*` glob with no edit to the root manifest
    * `[ ]`   The crate's public surface is the `domain_tag` module's `provides`, which every consumer that constructs a tag imports; the `factory` module's `provides` joins it when `hash-to-scalar/keccak256` creates that module
    * `[ ]`   Outside: every use's tag value, the hash, the prefix, the reduction, and the Solidity mirror

  * `[ ]`   `deps`
    * `[ ]`   The standard library: `Vec<u8>`, `u8::is_ascii_graphic`, `Iterator::enumerate`, and `Iterator::find`, through the prelude
    * `[ ]`   No external crate and no repository crate; no reverse dependency; nothing depends on the crate yet

  * `[ ]`   `context_slice`
    * `[ ]`   From the standard library: `<[u8]>::is_empty`, `<[u8]>::len`, `<[u8]>::iter`, `Iterator::enumerate`, `Iterator::find`, and `u8::is_ascii_graphic`, which is true exactly for `0x21` through `0x7E`

  * `[ ]`   `adapters/hash-to-scalar/Cargo.toml`
    * `[ ]`   `[package]` with `name = "hash-to-scalar"`, `edition.workspace = true`, `rust-version.workspace = true`, and `publish.workspace = true`; no `version` key
    * `[ ]`   `[features]` with `mocks = []`
    * `[ ]`   `[lints]` with `workspace = true`
    * `[ ]`   No other table; `hash-to-scalar/keccak256` adds the dependency tables

  * `[ ]`   `adapters/hash-to-scalar/src/lib.rs`
    * `[ ]`   The crate barrel: `mod domain_tag;` and `pub use domain_tag::provides::*;`, nothing else
    * `[ ]`   Until `domain_tag/mod.rs` exists, `cargo check` reports the unresolved `mod domain_tag`, which is the RED state for every element below that precedes the implementation

  * `[ ]`   `adapters/hash-to-scalar/src/domain_tag/interface.rs`
    * `[ ]`   `DOMAIN_TAG_MAXIMUM_LENGTH`, a `pub const` of type `usize` with value `255`, the largest length a one-byte prefix holds
    * `[ ]`   `DomainTag`, a struct with `#[derive(Clone, Debug, PartialEq, Eq)]` and the one field `pub(super) bytes: Vec<u8>`, so only the `domain_tag` module and its children reach the field
    * `[ ]`   `DomainTagConstructorParams`, a struct with the one field `pub bytes: Vec<u8>`; no derives
    * `[ ]`   `DomainTagTryNewErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]` and the variants `Empty`, `TooLong { length: usize, maximum: usize }`, and `ByteOutsideVisibleAscii { index: usize, byte: u8 }`
    * `[ ]`   `DomainTagTryNewReturn`, the alias `Result<DomainTag, DomainTagTryNewErrorReturn>`
    * `[ ]`   Imports nothing; declares nothing else

  * `[ ]`   `adapters/hash-to-scalar/src/domain_tag/interaction.spec.md`
    * `[ ]`   `DomainTag::try_new(params: DomainTagConstructorParams) -> DomainTagTryNewReturn`, empty: condition `params.bytes.is_empty()`; decision the emptiness check; dependency call none; outcome `Err(DomainTagTryNewErrorReturn::Empty)`
    * `[ ]`   Too long: condition the tag is non-empty and `params.bytes.len() > DOMAIN_TAG_MAXIMUM_LENGTH`; decision the comparison; dependency call none; outcome `Err(DomainTagTryNewErrorReturn::TooLong { length, maximum: DOMAIN_TAG_MAXIMUM_LENGTH })`, `length` the tag's length
    * `[ ]`   Byte outside visible ASCII: condition the length passes and some byte fails `is_ascii_graphic`; decision the first such byte by index, `iter().enumerate().find(…)`; dependency call none; outcome `Err(DomainTagTryNewErrorReturn::ByteOutsideVisibleAscii { index, byte })` for the lowest such index
    * `[ ]`   Admitted: condition every check passes; outcome `Ok(DomainTag { bytes })`, the vector moved from the params without copy
    * `[ ]`   `DomainTag::as_bytes(&self) -> &[u8]`: one branch; outcome a shared reference to the held bytes, no copy, no side effect
    * `[ ]`   Ordering: emptiness, then length, then the byte scan; the same params always yield the same outcome
    * `[ ]`   Invariants: every `DomainTag` holds between 1 and 255 bytes, each visible ASCII; its only producer is `try_new`

  * `[ ]`   `adapters/hash-to-scalar/src/domain_tag/mock.rs`
    * `[ ]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`
    * `[ ]`   `DomainTagConstructorParamsOverrides`, `#[derive(Default)]`, one field `pub bytes: Option<Vec<u8>>`
    * `[ ]`   `build_domain_tag_constructor_params(overrides: DomainTagConstructorParamsOverrides) -> DomainTagConstructorParams`, the bytes defaulting to `b"ChainTorrent example tag".to_vec()`
    * `[ ]`   `build_domain_tag(overrides: DomainTagConstructorParamsOverrides) -> DomainTag`, returning the real instance from `DomainTag::try_new(build_domain_tag_constructor_params(overrides))` through `.expect("built domain tag constructor params are admitted")`
    * `[ ]`   No corruptions type and no invalidator: the constructor params are typed bytes and every tag the constructor refuses is a value the params builder's overrides carry; no `DomainTag` overrides, invalidator, or mock function, since the type is built as a real instance and owns no free function
    * `[ ]`   Imports `DomainTag` and `DomainTagConstructorParams` from `super::interface`

  * `[ ]`   `adapters/hash-to-scalar/src/domain_tag/test.rs`
    * `[ ]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`; imports `DomainTag` and `DomainTagTryNewErrorReturn` from `super::interface`, and `build_domain_tag_constructor_params` and `DomainTagConstructorParamsOverrides` from `super::mock`; each test builds its params through `build_domain_tag_constructor_params` with `bytes` overridden, acts `DomainTag::try_new(params)`, and unpacks the result by `let Ok(tag) = … else { panic!(…) };` or `let Err(error) = … else { panic!(…) };`
    * `[ ]`   `try_new_admits_a_visible_ascii_tag`: contract: a visible-ASCII tag is admitted and read back unchanged; arrange `bytes: Some(b"ChainTorrent v1 identity".to_vec())`, differing from the builder's default; act `try_new`; assert `tag.as_bytes()` equals `b"ChainTorrent v1 identity"`
    * `[ ]`   `try_new_admits_a_tag_of_the_maximum_length`: contract: a tag of exactly 255 bytes fits the prefix; arrange `bytes: Some(vec![b'a'; 255])`; act `try_new`; assert `tag.as_bytes().len()` equals `255`
    * `[ ]`   `try_new_rejects_an_empty_tag`: arrange `bytes: Some(Vec::new())`; act `try_new`; assert `error` equals `DomainTagTryNewErrorReturn::Empty`
    * `[ ]`   `try_new_rejects_a_tag_one_byte_over_the_maximum`: arrange `bytes: Some(vec![b'a'; 256])`; act `try_new`; assert `error` equals `DomainTagTryNewErrorReturn::TooLong { length: 256, maximum: 255 }`
    * `[ ]`   `try_new_rejects_the_lowest_byte_outside_visible_ascii`: contract: of several offending bytes, the lowest index is reported; arrange `bytes: Some(b"Chain Torrent\ttag".to_vec())`, a space at index 5 and a tab at index 13; act `try_new`; assert `error` equals `DomainTagTryNewErrorReturn::ByteOutsideVisibleAscii { index: 5, byte: 0x20 }`
    * `[ ]`   `try_new_rejects_a_non_ascii_byte`: arrange `bytes: Some(vec![b'a', 0xC3, 0xA4])`; act `try_new`; assert `error` equals `DomainTagTryNewErrorReturn::ByteOutsideVisibleAscii { index: 1, byte: 0xC3 }`
    * `[ ]`   `try_new_reports_the_length_before_the_bytes`: contract: a tag that fails both the length and the byte scan returns the length refusal; arrange `bytes: Some(vec![0x20; 256])`; act `try_new`; assert `error` equals `DomainTagTryNewErrorReturn::TooLong { length: 256, maximum: 255 }`
    * `[ ]`   Every test block carries the full `Contract`, `Arrange`, `Act`, `Assert` header and the inline markers

  * `[ ]`   `construction`
    * `[ ]`   `DomainTag::try_new` is the only producer; no `Default`, `From`, or other constructor exists; a consumer states its tag bytes as a constant and constructs the tag once, handling the refusal arm

  * `[ ]`   `adapters/hash-to-scalar/src/domain_tag/mod.rs`
    * `[ ]`   Module declarations: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, `pub mod provides;`, and `#[cfg(test)] mod test;`
    * `[ ]`   `impl DomainTag` with `pub fn try_new(params: DomainTagConstructorParams) -> DomainTagTryNewReturn` realizing the branches and ordering of the interaction spec, and `pub fn as_bytes(&self) -> &[u8]` returning `&self.bytes`
    * `[ ]`   Imports `DomainTag`, `DomainTagConstructorParams`, `DomainTagTryNewErrorReturn`, `DomainTagTryNewReturn`, and `DOMAIN_TAG_MAXIMUM_LENGTH` from `interface`
    * `[ ]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[ ]`   `adapters/hash-to-scalar/src/domain_tag/provides.rs`
    * `[ ]`   `pub use super::interface::*;` and `#[cfg(any(test, feature = "mocks"))] pub use super::mock::*;`, nothing else

  * `[ ]`   `directionality`
    * `[ ]`   `domain_tag` depends on the standard library alone; the crate depends on no repository crate; `hash-to-scalar/keccak256` consumes the tag through `crate::domain_tag::provides` in the generic interface's payload; no cycle

  * `[ ]`   `requirements`
    * `[ ]`   `adapters/hash-to-scalar/Cargo.toml` carries exactly the tables and keys stated above, and `adapters/hash-to-scalar/src/lib.rs` carries exactly the barrel stated above
    * `[ ]`   `cargo check --workspace --all-targets --all-features`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `cargo fmt --all --check` complete without error or warning
    * `[ ]`   `try_new_admits_a_visible_ascii_tag` and `try_new_admits_a_tag_of_the_maximum_length` pass
    * `[ ]`   `try_new_rejects_an_empty_tag`, `try_new_rejects_a_tag_one_byte_over_the_maximum`, `try_new_rejects_the_lowest_byte_outside_visible_ascii`, `try_new_rejects_a_non_ascii_byte`, and `try_new_reports_the_length_before_the_bytes` pass (CR-11, a tag has one byte form, refused before any hash)
    * `[ ]`   Code outside `adapters/hash-to-scalar/src/domain_tag` reading the `bytes` field fails to compile

* `[ ]`   `hash-to-scalar/keccak256` **Keccak-256 concrete hashing a length-prefixed domain tag and a message and reducing the digest modulo the group order of whichever pairing scalar it is asked for; authors the hash-to-scalar family's generic interface, identifier, declaration, and mock**

  * `[ ]`   `objective`
    * `[ ]`   Problem: the identity mapping and the delivery proof's challenge are scalars the contract recomputes, and the EVM has keccak256 and no BLAKE3, so both are keccak256 under a domain tag reduced modulo the group order, computed identically by the client and the contract, through one repo-owned interface, with no module outside a concrete naming the hash library (CR-08; CR-09; CR-11)
    * `[ ]`   Functional: the family's generic interface maps a domain tag and a message to a scalar of the pairing scalar type it is instantiated at, so one concrete serves every curve and every pairing library
    * `[ ]`   Functional: every concrete declares the hash-to-scalar identifier a deployment's hash-card names, its adapter version, and the interface version it implements, readable before any instance exists
    * `[ ]`   Functional: the Keccak-256 concrete hashes the tag's length as one byte, then the tag, then the message, reads the 32-byte digest as a big-endian integer, and reduces it modulo the group order through the scalar type's own sampling bound, the digest zero-extended at the front to the bound's input length; the contract reproduces it as `uint256(keccak256(abi.encodePacked(uint8(tag.length), tag, message))) % r`
    * `[ ]`   Functional: the concrete's scalars match the known-answer vectors computed by an independent Keccak implementation, on BN254 and BLS12-381 and on both libraries of each, including digests at or above the group order
    * `[ ]`   Non-functional: `sha3` is named only inside `adapters/hash-to-scalar/src/keccak256`; no pairing library is named in the crate

  * `[ ]`   `role`
    * `[ ]`   Adapter: the hash-to-scalar family's first concrete, and the first source file that requires the family's generic interface, identifier, declaration, and mock, which it authors in the family's `factory` module as its producers
    * `[ ]`   Creates `factory/mod.rs` with the module wiring alone and `factory/provides.rs` with the interface and mock surface alone; the factory function, its types, its interaction spec, its unit test, its re-export, and the family's integration test are `hash-to-scalar/factory`'s
    * `[ ]`   Does not reduce a scalar itself; the pairing concrete's `ISampleUniformScalar` reduces, so no field arithmetic is duplicated and no library is named here
    * `[ ]`   Does not name a use's tag or encode a message; each consumer holds its tag and encodes its message through the encoding factory
    * `[ ]`   Does not emit the Solidity mirror; `harness-crypto/generate/evm` mirrors the vectors
    * `[ ]`   Does not map the identifier to the hash-card's `hashToScalarId` byte; the hash-card's own encoding does that
    * `[ ]`   Does not carry a commit

  * `[ ]`   `module`
    * `[ ]`   Bounded context: the `factory` module's generic trait over a scalar type, its params, payload, success, error, and return types, the identifier, the declaration, the interface version, and the family's mock; and the private `keccak256` concrete, holding the adapter over `sha3`'s Keccak-256, its digest length, its constructor params and return, and its error
    * `[ ]`   Adds the `factory` and `keccak256` modules to the existing crate at `adapters/hash-to-scalar`; the crate's manifest gains its dependency tables and its barrel gains the modules' lines
    * `[ ]`   Outside: every tag value and message encoding, the pairing concretes' reduction, the factory's selection of a concrete, and the Solidity mirror

  * `[ ]`   `deps`
    * `[ ]`   `domain_tag`, same crate, through `crate::domain_tag::provides`: `DomainTag` with `as_bytes`, held by the generic interface's params; and `build_domain_tag` with `DomainTagConstructorParamsOverrides` in `keccak256/test.rs`
    * `[ ]`   `domain`, `crates/domain`, protocol and domain ring, path dependency; supplies `Secret` and `SecretConstructorParams`, the wrapper the sampling bound's payload takes; direction inward
    * `[ ]`   `pairing`, `adapters/pairing`, adapter ring, path dependency; supplies `ISampleUniformScalar` with `UNIFORM_BYTES_LENGTH` and `sample_from_uniform_bytes`, `SampleUniformScalarParams`, `SampleUniformScalarPayload`, and `SampleUniformScalarErrorReturn`, the family's scalar being the resolved pairing's scalar, the dependency map's edge from `pairing/factory`; the pairing crate names nothing in this crate
    * `[ ]`   `pairing` with its `mocks` feature, as a dev-dependency, in `keccak256/test.rs` only: `create_pairing`, `CreatePairingDeps`, `CreatePairingPayload`, `build_create_pairing_params`, `CreatePairingParamsOverrides`, `PairingConcrete`, `IPairingConsumer`, `IPairingAdapter`, `ConsumePairingParams`, `ConsumePairingPayload`, `EncodeScalarParams`, and `EncodeScalarPayload`, since the scalar types are reachable only through the pairing factory
    * `[ ]`   `sha3` `0.12.0`, external crate, MIT OR Apache-2.0, runtime dependency with default features, named only in `keccak256`; supplies `sha3::Keccak256` and the `sha3::Digest` trait
    * `[ ]`   `hex` `0.4.3`, external crate, MIT OR Apache-2.0, dev-dependency only; supplies `hex::decode` for the vectors
    * `[ ]`   `core::convert::Infallible`, standard library, the constructor's error arm; `core::marker::PhantomData`, standard library, in `factory/mock.rs` only; `TryFrom<usize> for u8`, standard library, the tag-length prefix
    * `[ ]`   No reverse dependency beyond the family form's recorded cycle: the `factory` module's error carries this concrete's error; nothing depends on the crate yet

  * `[ ]`   `context_slice`
    * `[ ]`   From `domain_tag`: `DomainTag::as_bytes(&self) -> &[u8]`, between 1 and 255 visible-ASCII bytes
    * `[ ]`   From `domain`: `Secret::try_new(SecretConstructorParams { value })` returning `Result<Secret<T>, Infallible>`, and `Secret::expose(&self) -> &T`
    * `[ ]`   From `pairing`: `ISampleUniformScalar::UNIFORM_BYTES_LENGTH`, `64` on every current concrete, and `sample_from_uniform_bytes(SampleUniformScalarParams, SampleUniformScalarPayload { uniform }) -> Result<SampleUniformScalarSuccessReturn<S>, SampleUniformScalarErrorReturn>`, which reads `uniform` as a big-endian integer and reduces it modulo the group order into `Secret<S>`; `S: Clone` through `IPairingAdapter::Scalar`'s bound
    * `[ ]`   From `sha3`: `Keccak256::new()`, `Digest::update(&mut self, data: impl AsRef<[u8]>)`, and `Digest::finalize(self)`, whose 32-byte output dereferences to a byte slice
    * `[ ]`   From `pairing`, in the tests: `create_pairing<C: IPairingConsumer>(&CreatePairingDeps<C>, CreatePairingParams, CreatePairingPayload)`, `IPairingConsumer::consume_pairing<P: IPairingAdapter>`, and `IPairingAdapter::encode_scalar(&self, EncodeScalarParams, EncodeScalarPayload { scalar }) -> Result<EncodeScalarSuccessReturn, Infallible>` with `bytes: Secret<Vec<u8>>`, 32 big-endian bytes on both curves

  * `[ ]`   `adapters/hash-to-scalar/Cargo.toml`
    * `[ ]`   `[dependencies]` with `domain = { path = "../../crates/domain" }`, `pairing = { path = "../pairing" }`, and `sha3 = "0.12.0"`
    * `[ ]`   `[dev-dependencies]` with `pairing = { path = "../pairing", features = ["mocks"] }` and `hex = "0.4.3"`
    * `[ ]`   `[package]`, `[features]`, and `[lints]` are unchanged; no other table

  * `[ ]`   `adapters/hash-to-scalar/src/lib.rs`
    * `[ ]`   The crate barrel reads `mod domain_tag;`, `mod factory;`, `mod keccak256;`, `pub use domain_tag::provides::*;`, and `pub use factory::provides::*;`, nothing else; the `keccak256` concrete's surface is not re-exported
    * `[ ]`   Until `factory/mod.rs` and `keccak256/mod.rs` exist, `cargo check` reports the unresolved modules, which is the RED state for every element below that precedes them

  * `[ ]`   `adapters/hash-to-scalar/src/factory/interface.rs`
    * `[ ]`   `HASH_TO_SCALAR_INTERFACE_VERSION`, a `pub const` of type `u32` with value `1`
    * `[ ]`   `HashToScalarIdentifier`, an enum with the one variant `Keccak256V1`, the identifier a deployment's hash-card names
    * `[ ]`   `HashToScalarDeclaration`, a struct with `pub identifier: HashToScalarIdentifier`, `pub adapter_version: u32`, and `pub interface_version: u32`
    * `[ ]`   `HashToScalarParams<'a>`, a struct with `pub tag: &'a DomainTag`, the domain the hash is separated into, borrowed so a consumer constructs its tag once
    * `[ ]`   `HashToScalarPayload<'a>`, a struct with `pub message: &'a [u8]`, the canonical encoding of what is hashed
    * `[ ]`   `HashToScalarSuccessReturn<S>`, a struct with `pub scalar: S`; the scalar is public, since the identity mapping and the challenge are values the contract recomputes
    * `[ ]`   `HashToScalarErrorReturn`, an enum with the one variant `Keccak256(Keccak256HashToScalarErrorReturn)`, the Keccak-256 concrete's error carried unchanged; each further concrete's error is its own variant
    * `[ ]`   `HashToScalarReturn<S>`, the alias `Result<HashToScalarSuccessReturn<S>, HashToScalarErrorReturn>`
    * `[ ]`   `IHashToScalarAdapter<S: ISampleUniformScalar + Clone>`, a trait with the one method `fn hash_to_scalar(&self, params: HashToScalarParams<'_>, payload: HashToScalarPayload<'_>) -> HashToScalarReturn<S>;`, generic over the scalar type rather than its method, so a consumer holds `Box<dyn IHashToScalarAdapter<P::Scalar>>` for the pairing it resolved
    * `[ ]`   No derives on any type in this file; imports `DomainTag` from `crate::domain_tag::provides`, `ISampleUniformScalar` from `pairing`, and `Keccak256HashToScalarErrorReturn` from `crate::keccak256::provides`; names no vendor

  * `[ ]`   `adapters/hash-to-scalar/src/keccak256/interface.rs`
    * `[ ]`   `KECCAK256_DIGEST_LENGTH`, a `pub const` of type `usize` with value `32`
    * `[ ]`   `Keccak256HashToScalar`, the unit struct `pub struct Keccak256HashToScalar;`, the adapter over `sha3`'s Keccak-256
    * `[ ]`   `Keccak256HashToScalarConstructorParams`, the fieldless struct `pub struct Keccak256HashToScalarConstructorParams;`, the constructor's deps slot
    * `[ ]`   `Keccak256HashToScalarTryNewReturn`, the alias `Result<Keccak256HashToScalar, Infallible>`; the error arm is uninhabited because the adapter takes no configuration
    * `[ ]`   `Keccak256HashToScalarErrorReturn`, an enum with the variants `TagLengthExceedsPrefix { length: usize }`, a tag longer than a one-byte prefix holds; `UniformLengthBelowDigest { uniform_length: usize, digest_length: usize }`, a scalar type whose sampling input cannot hold the digest; and `Sampling(SampleUniformScalarErrorReturn)`, the scalar type's sampling refusal carried unchanged
    * `[ ]`   Imports `SampleUniformScalarErrorReturn` from `pairing` and `core::convert::Infallible`; declares nothing else

  * `[ ]`   `adapters/hash-to-scalar/src/keccak256/interaction.spec.md`
    * `[ ]`   `Keccak256HashToScalar::try_new(params: Keccak256HashToScalarConstructorParams) -> Keccak256HashToScalarTryNewReturn`: one branch; outcome `Ok(Keccak256HashToScalar)`; the error arm has no branch
    * `[ ]`   `Keccak256HashToScalar::DECLARATION`: the inherent constant `HashToScalarDeclaration { identifier: HashToScalarIdentifier::Keccak256V1, adapter_version: 1, interface_version: HASH_TO_SCALAR_INTERFACE_VERSION }`
    * `[ ]`   `hash_to_scalar`, tag too long: condition `u8::try_from(params.tag.as_bytes().len())` fails; decision the conversion, first; dependency call none; outcome `Err(HashToScalarErrorReturn::Keccak256(Keccak256HashToScalarErrorReturn::TagLengthExceedsPrefix { length }))`; a `DomainTag` holds at most 255 bytes, so no input takes this branch and it has no unit test
    * `[ ]`   `hash_to_scalar`, sampling input too short: condition `S::UNIFORM_BYTES_LENGTH.checked_sub(KECCAK256_DIGEST_LENGTH)` is `None`; decision the subtraction, before hashing; dependency call none; outcome `Err(HashToScalarErrorReturn::Keccak256(Keccak256HashToScalarErrorReturn::UniformLengthBelowDigest { uniform_length: S::UNIFORM_BYTES_LENGTH, digest_length: KECCAK256_DIGEST_LENGTH }))`; every current scalar type samples from 64 bytes, so no input takes this branch and it has no unit test
    * `[ ]`   `hash_to_scalar`, sampling refused: condition `S::sample_from_uniform_bytes` returns `Err(error)`; decision the sampler's result; dependency call as below; outcome `Err(HashToScalarErrorReturn::Keccak256(Keccak256HashToScalarErrorReturn::Sampling(error)))`, the error unchanged; the input is exactly `S::UNIFORM_BYTES_LENGTH` bytes, the sampler's one refusal is a wrong length, so no input takes this branch and it has no unit test
    * `[ ]`   `hash_to_scalar`, hashed: condition both checks pass; dependency calls `Keccak256::new()`, then `update` with the one prefix byte, `update` with `params.tag.as_bytes()`, and `update` with `payload.message`, in that order, each once, then `finalize` once; a buffer of `S::UNIFORM_BYTES_LENGTH` zero bytes receives the digest in its last `KECCAK256_DIGEST_LENGTH` bytes and is moved into a `Secret` by `let Ok(uniform) = Secret::try_new(SecretConstructorParams { value: buffer });`; then `S::sample_from_uniform_bytes(SampleUniformScalarParams, SampleUniformScalarPayload { uniform })` once; outcome `Ok(HashToScalarSuccessReturn { scalar })`, `scalar` a clone of the sampled `Secret`'s exposed value, the `Secret` then dropping and zeroizing its copy
    * `[ ]`   Ordering: the prefix conversion and the length subtraction precede the hasher; the prefix, the tag, and the message are absorbed in that order; the same tag and message always yield the same scalar for one scalar type

  * `[ ]`   `adapters/hash-to-scalar/src/factory/mock.rs`
    * `[ ]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`
    * `[ ]`   `HashToScalarDeclarationOverrides`, `#[derive(Default)]`, one `Option` per field; `build_hash_to_scalar_declaration(overrides: HashToScalarDeclarationOverrides) -> HashToScalarDeclaration`, defaulting to `HashToScalarIdentifier::Keccak256V1`, `1`, and `HASH_TO_SCALAR_INTERFACE_VERSION`
    * `[ ]`   `HashToScalarSuccessReturnOverrides<S>`, `#[derive(Default)]`, one field `pub scalar: Option<S>`; `build_hash_to_scalar_success_return<S: Default>(overrides: HashToScalarSuccessReturnOverrides<S>) -> HashToScalarSuccessReturn<S>`, the scalar defaulting to `S::default()`
    * `[ ]`   `MockIHashToScalarAdapter<S>`, a struct with the one field `pub scalar: PhantomData<S>`, implementing `IHashToScalarAdapter<S>` for `S: ISampleUniformScalar + Clone + Default` with `hash_to_scalar` returning `Ok(build_hash_to_scalar_success_return(Default::default()))` for any params and payload; a test needing other behavior implements the trait on its own local struct
    * `[ ]`   No builder for `HashToScalarParams` and `HashToScalarPayload`, whose fields are borrowed with no default and are written at the call site as their production values, or for the enums `HashToScalarIdentifier` and `HashToScalarErrorReturn`; no corruptions type and no invalidator, since no value this interface owns arrives as untrusted data
    * `[ ]`   Imports `ISampleUniformScalar` from `pairing`, `core::marker::PhantomData`, and this module's types from `super::interface`

  * `[ ]`   `adapters/hash-to-scalar/src/factory/mod.rs`
    * `[ ]`   Module wiring only: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, and `pub mod provides;`, nothing else

  * `[ ]`   `adapters/hash-to-scalar/src/factory/provides.rs`
    * `[ ]`   `pub use super::interface::*;` and `#[cfg(any(test, feature = "mocks"))] pub use super::mock::*;`, nothing else

  * `[ ]`   `adapters/hash-to-scalar/src/keccak256/test.rs`
    * `[ ]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`; imports `Keccak256HashToScalar` and `Keccak256HashToScalarConstructorParams` from `super::interface`; `IHashToScalarAdapter`, `HashToScalarParams`, `HashToScalarPayload`, `HashToScalarIdentifier`, and `HASH_TO_SCALAR_INTERFACE_VERSION` from `crate::factory::provides`; `DomainTag`, `build_domain_tag`, and `DomainTagConstructorParamsOverrides` from `crate::domain_tag::provides`; the `pairing` names the deps list for the tests; and `hex::decode`
    * `[ ]`   A test-local `ScalarProbe`, a struct with `tag: DomainTag` and `message: Vec<u8>`, implementing `IPairingConsumer` with `type Output = Vec<u8>;` and a `consume_pairing<P: IPairingAdapter>` that constructs the subject by `let Ok(hasher) = Keccak256HashToScalar::try_new(Keccak256HashToScalarConstructorParams);`, calls `IHashToScalarAdapter::<P::Scalar>::hash_to_scalar(&hasher, HashToScalarParams { tag: &self.tag }, HashToScalarPayload { message: &self.message })`, encodes the returned scalar through `payload.adapter.encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar })`, each call unpacked by `let Ok(…) = … else { panic!(…) };`, and returns the exposed 32 bytes cloned into a `Vec<u8>`
    * `[ ]`   Each test builds its tag through `build_domain_tag` with `bytes` overridden, runs `ScalarProbe` through `create_pairing` with `build_create_pairing_params` overridden by `concrete` alone and `CreatePairingDeps { consumer }`, and unpacks the factory's return by `let Ok(success) = … else { panic!(…) };`
    * `[ ]`   The reference tag is `b"ChainTorrent hash-to-scalar test".to_vec()`, 32 bytes; the reference message is the 448-byte reference vector `encoding/abi` states, written as one hex string of its words in order
    * `[ ]`   The independent vectors, each a 32-byte big-endian scalar, computed with the Keccak team's pure-Python reference `CompactFIPS202.py` from the XKCP repository as `Keccak(1088, 512, input, 0x01, 32)`, which reproduces keccak256 of the empty string and of `abc`, over the input `0x20`, the reference tag, then the message, with the digest reduced modulo each group order: for the reference message, digest `4151bea01b9ddc7edbe167b8167245798f08813fcb739ae26c7a3637880f8de2`, at or above the BN254 group order and below the BLS12-381 group order, BN254 scalar `10ed702d3a6c3c552391220194f0ed1c66d498f751ba2a51289840a3980f8de1`, BLS12-381 scalar `4151bea01b9ddc7edbe167b8167245798f08813fcb739ae26c7a3637880f8de2`; for the empty message, digest `e563de6b3c6c28ad12ef953efbe86d3dd05f80bad4c7a2198246e9836046bb0b`, at or above both group orders, BN254 scalar `23d2a49fb7a5a80631ae7e64f5e30bc92f8fdf98ede1dfd472bf1333a046bb07`, BLS12-381 scalar `7176371812ceab64dfb5bd36f24695387ca1dcb7d4c9461a8246e9846046bb0a`
    * `[ ]`   `hash_to_scalar_on_bn254_arkworks_reduces_the_reference_digest_to_the_independent_scalar`: contract: the tag prefix, the tag, and the message hash and reduce modulo the BN254 group order as the independent implementation does; arrange the reference tag and message and `concrete: Some(PairingConcrete::Bn254Arkworks)`; act `create_pairing`; assert `success.output` equals the reference message's BN254 scalar
    * `[ ]`   `hash_to_scalar_on_bn254_halo2curves_reduces_the_reference_digest_to_the_independent_scalar`: the same with `PairingConcrete::Bn254Halo2curves`
    * `[ ]`   `hash_to_scalar_on_bls12_381_arkworks_reduces_the_reference_digest_to_the_independent_scalar`: the same with `PairingConcrete::Bls12381Arkworks` against the reference message's BLS12-381 scalar
    * `[ ]`   `hash_to_scalar_on_bls12_381_halo2curves_reduces_the_reference_digest_to_the_independent_scalar`: the same with `PairingConcrete::Bls12381Halo2curves`
    * `[ ]`   `hash_to_scalar_on_bn254_arkworks_reduces_a_digest_above_the_group_order`: contract: an empty message hashes and reduces; arrange the reference tag, `message` empty, and `PairingConcrete::Bn254Arkworks`; act `create_pairing`; assert `success.output` equals the empty message's BN254 scalar
    * `[ ]`   `hash_to_scalar_on_bls12_381_arkworks_reduces_a_digest_above_the_group_order`: the same with `PairingConcrete::Bls12381Arkworks` against the empty message's BLS12-381 scalar
    * `[ ]`   `hash_to_scalar_binds_the_tag_length`: contract: moving a byte from the tag to the message changes the scalar, because the tag's length is absorbed first; arrange the tag `b"ChainTorrent hash-to-scalar tes".to_vec()`, the message `74` followed by the reference message, and `PairingConcrete::Bn254Arkworks`; act `create_pairing`; assert `success.output` differs from the reference message's BN254 scalar
    * `[ ]`   `hash_to_scalar_separates_domains`: contract: a different tag over the same message yields a different scalar; arrange the tag `b"ChainTorrent other tag".to_vec()`, the reference message, and `PairingConcrete::Bn254Arkworks`; act `create_pairing`; assert `success.output` differs from the reference message's BN254 scalar
    * `[ ]`   `keccak256_hash_to_scalar_declares_its_identifier_and_versions`: act read `Keccak256HashToScalar::DECLARATION`; assert `identifier` matches `HashToScalarIdentifier::Keccak256V1`, `adapter_version` equals `1`, and `interface_version` equals `HASH_TO_SCALAR_INTERFACE_VERSION`
    * `[ ]`   Every test block carries the full `Contract`, `Arrange`, `Act`, `Assert` header and the inline markers; the tag-too-long, sampling-input-too-short, and sampling-refused branches have no unit test, as the interaction spec states

  * `[ ]`   `construction`
    * `[ ]`   `Keccak256HashToScalar::try_new` is the concrete's only producer, and its only caller is the hash-to-scalar factory, which reads `Keccak256HashToScalar::DECLARATION` before constructing
    * `[ ]`   The concrete owns no object type a consumer builds as a fixture: the adapter is a unit struct constructed by `try_new` over fieldless params and its error is an enum, so it has no `mock.rs`

  * `[ ]`   `adapters/hash-to-scalar/src/keccak256/mod.rs`
    * `[ ]`   Module declarations: `mod interface;`, `pub(crate) mod provides;`, and `#[cfg(test)] mod test;`
    * `[ ]`   `impl Keccak256HashToScalar` with `pub const DECLARATION: HashToScalarDeclaration` as the interaction spec states and `pub fn try_new(_params: Keccak256HashToScalarConstructorParams) -> Keccak256HashToScalarTryNewReturn` returning `Ok(Keccak256HashToScalar)`
    * `[ ]`   `impl<S: ISampleUniformScalar + Clone> IHashToScalarAdapter<S> for Keccak256HashToScalar` with `hash_to_scalar` realizing the branches and ordering of the interaction spec; the digest is copied into the buffer's tail by `copy_from_slice` over the range starting at the subtraction's result
    * `[ ]`   Imports the family's names from `crate::factory::provides`, `Secret` and `SecretConstructorParams` from `domain`, `ISampleUniformScalar`, `SampleUniformScalarParams`, and `SampleUniformScalarPayload` from `pairing`, `sha3::{Digest, Keccak256}`, and this module's names from `interface`
    * `[ ]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[ ]`   `adapters/hash-to-scalar/src/keccak256/provides.rs`
    * `[ ]`   `pub(crate) use super::interface::*;`, nothing else, so the concrete is visible to the crate's factory and to nothing outside the crate

  * `[ ]`   `directionality`
    * `[ ]`   `keccak256` depends on the `factory` module's surface through `crate::factory::provides`, on `domain_tag`'s through `crate::domain_tag::provides`, on `domain`, on `pairing`, and on `sha3`; the `factory` module depends on `domain_tag`, on `pairing`, and on `keccak256`'s error through `crate::keccak256::provides`, the family form's recorded cycle, which `hash-to-scalar/factory` completes by constructing the concrete
    * `[ ]`   Among repository crates the crate depends on `crates/domain` and `adapters/pairing`, which names nothing in this crate; nothing depends on the crate yet; no other cycle

  * `[ ]`   `requirements`
    * `[ ]`   `adapters/hash-to-scalar/Cargo.toml` carries exactly the tables and keys stated above, and `sha3` is named nowhere in the crate outside `adapters/hash-to-scalar/src/keccak256`
    * `[ ]`   `cargo check --workspace --all-targets --all-features`, `cargo fmt --all --check`, and `cargo deny check` complete without error; `cargo clippy --workspace --all-targets --all-features` reports nothing beyond the library target's unused-item warnings for the `keccak256` concrete, which `hash-to-scalar/factory` resolves by constructing the concrete
    * `[ ]`   Every `hash_to_scalar_on_…_reduces_…` test passes (CR-11, the tag prefix, serialization, and reduction frozen against an independent implementation's vectors on both curves and all four pairing concretes; CR-08 and CR-09 for the identity mapping and the challenge that consume it)
    * `[ ]`   `hash_to_scalar_binds_the_tag_length`, `hash_to_scalar_separates_domains`, and `keccak256_hash_to_scalar_declares_its_identifier_and_versions` pass
    * `[ ]`   Code outside `adapters/hash-to-scalar` naming `Keccak256HashToScalar` or anything under `keccak256` fails to compile; the crate's public surface is the `domain_tag` and `factory` modules' `provides`

* `[ ]`   `hash-to-scalar/factory` **Hash-to-scalar factory constructing the concrete the configuration names, admitted against the hash-to-scalar identifier the hash-card requires, and returning it behind the family's trait for the pairing scalar type asked for; carries the family's integration test and the milestone's commit**

  * `[ ]`   `objective`
    * `[ ]`   Problem: a consumer obtains a hash-to-scalar adapter only through the family's generic surface, never by naming a concrete, for the scalar type of the pairing it resolved, and the mapping it uses is the one the deployment's hash-card names, so a concrete that does not declare the required identifier is refused before anything is constructed (CR-08; CR-09; CR-11; Composition Boundary)
    * `[ ]`   Functional: given the concrete the configuration names and the identifier required, the factory refuses a concrete whose declared identifier is not the required one, with no construction
    * `[ ]`   Functional: an admitted concrete is constructed and returned as `Box<dyn IHashToScalarAdapter<S>>` for the scalar type `S` the caller names, together with its declaration
    * `[ ]`   Functional: a concrete's constructor error is returned unchanged in the factory's error arm, one variant per concrete
    * `[ ]`   Functional: the concrete the factory returns, handed the scalar type of the pairing the pairing factory resolves, maps the reference tag over the reference context encoded through the encoding factory to the independent scalar on each curve
    * `[ ]`   Non-functional: adding a concrete is its module, its variant in the selection enum and in the error enum, and its branch here; adding an identifier is its variant in `HashToScalarIdentifier`; no consumer changes

  * `[ ]`   `role`
    * `[ ]`   Adapter family factory: the implementation of the `factory` module, the hash-to-scalar family's construction point, and the crate's public surface beside the family-owned domain tag
    * `[ ]`   Returns the concrete behind `Box<dyn IHashToScalarAdapter<S>>`, because the family's trait is generic over the scalar type and not its method, so it is dyn-compatible for each scalar type; the factory is generic over that type and the caller names it as its pairing's `P::Scalar`
    * `[ ]`   Selects by concrete and admits by identifier, so a further concrete implementing an existing identifier and a further identifier each arrive as a variant and a branch, with no change to the params or to any consumer
    * `[ ]`   Does not read a hash-card or the configuration; the composition resolver passes the concrete the configuration names as a typed `HashToScalarConcrete` and the identifier the hash-card requires as a typed `HashToScalarIdentifier`
    * `[ ]`   Does not hash; hashing is the concrete's
    * `[ ]`   Carries the family's integration test across factory, concrete, domain tag, the pairing family that supplies the scalar type, and the encoding family that produces the message; carries the commit that closes the pairing adapters and key derivation milestone

  * `[ ]`   `module`
    * `[ ]`   Bounded context: the `factory` module of `adapters/hash-to-scalar`, holding the factory function, its deps, params, payload, and return types, its signature type, the selection enum, the function mock and builders, and the crate's integration test under `adapters/hash-to-scalar/tests`
    * `[ ]`   Outside: the concrete's behavior, every tag value and message encoding, the pairing concretes, the hash-card, the configuration catalogue, and every consumer of the family

  * `[ ]`   `deps`
    * `[ ]`   The `keccak256` concrete, through `crate::keccak256::provides`: `Keccak256HashToScalar`, `Keccak256HashToScalarConstructorParams`, `Keccak256HashToScalar::try_new`, and `Keccak256HashToScalar::DECLARATION`; the factory constructs its concrete, completing the family form's recorded cycle that `hash-to-scalar/keccak256` opened
    * `[ ]`   The `factory` module's own interface: `IHashToScalarAdapter`, `HashToScalarIdentifier`, `HashToScalarDeclaration`, and `HASH_TO_SCALAR_INTERFACE_VERSION`; `ISampleUniformScalar` from `pairing`, the factory's bound on the scalar type
    * `[ ]`   `pairing` with its `mocks` feature, the existing dev-dependency, in `factory/test.rs` and the integration test: `create_pairing` and the consumer names that reach a pairing's scalar type, and `encode_scalar` to compare a scalar with its vector
    * `[ ]`   `encoding`, `adapters/encoding`, adapter ring, dev-dependency with its `mocks` feature, in the integration test only: `create_encoding` and the names that encode the reference derivation context through the encoding family; nothing at runtime, and the encoding crate names nothing in this crate
    * `[ ]`   `domain`, the existing runtime dependency, also as a dev-dependency with its `mocks` feature, in the integration test only: `DerivationContext` and the builders and overrides that make the reference context
    * `[ ]`   `hex`, the existing dev-dependency, for the vectors
    * `[ ]`   `core::convert::Infallible`, standard library, the concrete's constructor error carried in the factory's error arm; `core::marker::PhantomData`, standard library, in `factory/mock.rs`

  * `[ ]`   `context_slice`
    * `[ ]`   From the concrete: `Keccak256HashToScalar::try_new(Keccak256HashToScalarConstructorParams) -> Result<Keccak256HashToScalar, Infallible>`, the inherent constant `Keccak256HashToScalar::DECLARATION: HashToScalarDeclaration`, and `Keccak256HashToScalar`'s implementation of `IHashToScalarAdapter<S>` for every `S: ISampleUniformScalar + Clone`
    * `[ ]`   From `pairing`, in the tests: `create_pairing<C: IPairingConsumer>(&CreatePairingDeps<C>, CreatePairingParams, CreatePairingPayload)`, `build_create_pairing_params` with `CreatePairingParamsOverrides`, `PairingConcrete`, `IPairingConsumer::consume_pairing<P: IPairingAdapter>`, `ConsumePairingParams`, `ConsumePairingPayload`, and `IPairingAdapter::encode_scalar(&self, EncodeScalarParams, EncodeScalarPayload { scalar })` returning 32 big-endian bytes in a `Secret`
    * `[ ]`   From `encoding`, in the integration test: `create_encoding`, `CreateEncodingDeps`, `CreateEncodingPayload`, `build_create_encoding_params` with `CreateEncodingParamsOverrides`, `EncodingConcrete::Abi`, `EncodingIdentifier::EthereumAbiV1`, `IEncodingConsumer`, `IEncoderAdapter`, `IDecoderAdapter`, `ConsumeEncodingParams`, `ConsumeEncodingPayload`, `EncodeParams`, `DerivationContextDescription`, and `DerivationContextDescriptionConstructorParams`

  * `[ ]`   `adapters/hash-to-scalar/Cargo.toml`
    * `[ ]`   `[dev-dependencies]` reads `domain = { path = "../../crates/domain", features = ["mocks"] }`, `encoding = { path = "../encoding", features = ["mocks"] }`, `pairing = { path = "../pairing", features = ["mocks"] }`, and `hex = "0.4.3"`
    * `[ ]`   `[package]`, `[dependencies]`, `[features]`, and `[lints]` are unchanged; no other table

  * `[ ]`   `adapters/hash-to-scalar/src/factory/interface.rs`
    * `[ ]`   `HashToScalarIdentifier` gains `#[derive(PartialEq, Eq)]`, so the admission compares a declared identifier with the required one
    * `[ ]`   `HashToScalarConcrete`, an enum with the one variant `Keccak256`, the selection of the concrete to construct
    * `[ ]`   `CreateHashToScalarDeps`, the fieldless struct `pub struct CreateHashToScalarDeps;`
    * `[ ]`   `CreateHashToScalarParams`, a struct with `pub concrete: HashToScalarConcrete` and `pub identifier: HashToScalarIdentifier`, the selection and the identifier the concrete must declare
    * `[ ]`   `CreateHashToScalarPayload`, the fieldless struct `pub struct CreateHashToScalarPayload;`, since the factory operates on no data
    * `[ ]`   `CreateHashToScalarSuccessReturn<S>`, a struct with `pub adapter: Box<dyn IHashToScalarAdapter<S>>` and `pub declaration: HashToScalarDeclaration`
    * `[ ]`   `CreateHashToScalarErrorReturn`, an enum with the variants `UnsupportedHashToScalarIdentifier`, the named concrete not declaring the required identifier, and `Keccak256(Infallible)`, the Keccak-256 concrete's constructor error carried unchanged; each further concrete's constructor error is its own variant
    * `[ ]`   `CreateHashToScalarReturn<S>`, the alias `Result<CreateHashToScalarSuccessReturn<S>, CreateHashToScalarErrorReturn>`
    * `[ ]`   `CreateHashToScalarFn<S>`, the alias `fn(&CreateHashToScalarDeps, CreateHashToScalarParams, CreateHashToScalarPayload) -> CreateHashToScalarReturn<S>`
    * `[ ]`   Adds the import of `core::convert::Infallible`; every item `hash-to-scalar/keccak256` authored in this file is unchanged except the derive on `HashToScalarIdentifier`

  * `[ ]`   `adapters/hash-to-scalar/src/factory/interaction.spec.md`
    * `[ ]`   `create_hash_to_scalar<S: ISampleUniformScalar + Clone>(deps: &CreateHashToScalarDeps, params: CreateHashToScalarParams, payload: CreateHashToScalarPayload) -> CreateHashToScalarReturn<S>`: decision a `match` on `params.concrete`, one arm per `HashToScalarConcrete` variant, exhaustive so a variant with no arm fails to compile
    * `[ ]`   Unsupported identifier: condition the named concrete's `DECLARATION.identifier` is not `params.identifier`; decision equality, read before any construction; dependency call none; outcome `Err(CreateHashToScalarErrorReturn::UnsupportedHashToScalarIdentifier)`, with nothing constructed; `HashToScalarIdentifier` has the one variant the Keccak-256 concrete declares, so no input takes this branch until a further identifier exists, and it has no unit test
    * `[ ]`   Admitted: condition the named concrete's declared identifier is `params.identifier`; dependency call the concrete's `try_new` with its fieldless constructor params, exactly once, its success destructured irrefutably because its error arm is uninhabited; outcome `Ok(CreateHashToScalarSuccessReturn { adapter: Box::new(hasher), declaration })` with the concrete's `DECLARATION`, the box coerced to `Box<dyn IHashToScalarAdapter<S>>`
    * `[ ]`   `CreateHashToScalarErrorReturn::Keccak256` carries the constructor's uninhabited error type in the return union, so no branch produces it
    * `[ ]`   `params.concrete` selects and `params.identifier` admits; `S` fixes the scalar type the returned adapter produces; `deps` and `payload` carry nothing and are not read

  * `[ ]`   `adapters/hash-to-scalar/src/factory/mock.rs`
    * `[ ]`   `CreateHashToScalarParamsOverrides`, `#[derive(Default)]`, fields `pub concrete: Option<HashToScalarConcrete>` and `pub identifier: Option<HashToScalarIdentifier>`; `build_create_hash_to_scalar_params(overrides: CreateHashToScalarParamsOverrides) -> CreateHashToScalarParams`, defaulting to `HashToScalarConcrete::Keccak256` and `HashToScalarIdentifier::Keccak256V1`
    * `[ ]`   `CreateHashToScalarSuccessReturnOverrides<S>`, `#[derive(Default)]`, fields `pub adapter: Option<Box<dyn IHashToScalarAdapter<S>>>` and `pub declaration: Option<HashToScalarDeclaration>`; `build_create_hash_to_scalar_success_return<S: ISampleUniformScalar + Clone + Default>(overrides: CreateHashToScalarSuccessReturnOverrides<S>) -> CreateHashToScalarSuccessReturn<S>`, the adapter defaulting to `Box::new(MockIHashToScalarAdapter { scalar: PhantomData })` and the declaration to `build_hash_to_scalar_declaration(Default::default())`
    * `[ ]`   `mock_create_hash_to_scalar<S: ISampleUniformScalar + Clone + Default>(_deps: &CreateHashToScalarDeps, _params: CreateHashToScalarParams, _payload: CreateHashToScalarPayload) -> CreateHashToScalarReturn<S>`, returning `Ok(build_create_hash_to_scalar_success_return(Default::default()))`
    * `[ ]`   No builder for the fieldless `CreateHashToScalarDeps` and `CreateHashToScalarPayload`, used by their production values, or for the enums `HashToScalarConcrete` and `CreateHashToScalarErrorReturn`; every symbol `hash-to-scalar/keccak256` authored in this file is unchanged

  * `[ ]`   `adapters/hash-to-scalar/src/factory/test.rs`
    * `[ ]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`; imports `create_hash_to_scalar` from `super`, `CreateHashToScalarDeps`, `CreateHashToScalarPayload`, `HashToScalarConcrete`, `HashToScalarDeclaration`, `HashToScalarIdentifier`, and `HASH_TO_SCALAR_INTERFACE_VERSION` from `super::interface`, `build_create_hash_to_scalar_params` and `CreateHashToScalarParamsOverrides` from `super::mock`, and `create_pairing`, `CreatePairingDeps`, `CreatePairingPayload`, `build_create_pairing_params`, `CreatePairingParamsOverrides`, `PairingConcrete`, `IPairingConsumer`, `IPairingAdapter`, `ConsumePairingParams`, and `ConsumePairingPayload` from `pairing`
    * `[ ]`   A test-local `DeclarationProbe`, the unit struct implementing `IPairingConsumer` with `type Output = HashToScalarDeclaration;` and a `consume_pairing<P: IPairingAdapter>` that calls `create_hash_to_scalar::<P::Scalar>(&CreateHashToScalarDeps, build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides { concrete: Some(HashToScalarConcrete::Keccak256), identifier: Some(HashToScalarIdentifier::Keccak256V1) }), CreateHashToScalarPayload)`, unpacked by `let Ok(success) = … else { panic!(…) };`, and returns `success.declaration`
    * `[ ]`   `create_hash_to_scalar_returns_the_keccak256_concrete_and_its_declaration_for_a_pairing_scalar`: contract: the Keccak-256 concrete, admitted for its identifier, is returned with its declaration for the scalar type of a resolved pairing; arrange `build_create_pairing_params` with `concrete: Some(PairingConcrete::Bn254Arkworks)` and `CreatePairingDeps { consumer: DeclarationProbe }`; act `create_pairing(&deps, params, CreatePairingPayload)`, unpacked by `let Ok(success) = … else { panic!(…) };`; assert `success.output.identifier` matches `HashToScalarIdentifier::Keccak256V1`, `success.output.adapter_version` equals `1`, and `success.output.interface_version` equals `HASH_TO_SCALAR_INTERFACE_VERSION`
    * `[ ]`   The unsupported-identifier branch has no unit test, as the interaction spec states
    * `[ ]`   The test block carries the full `Contract`, `Arrange`, `Act`, `Assert` header and the inline markers

  * `[ ]`   `construction`
    * `[ ]`   A consumer working with a resolved pairing `P` calls `create_hash_to_scalar::<P::Scalar>` with `&CreateHashToScalarDeps`, `CreateHashToScalarParams` holding the concrete the configuration names and the identifier the hash-card requires, and `CreateHashToScalarPayload`, and holds the returned `Box<dyn IHashToScalarAdapter<P::Scalar>>` in its deps; no consumer constructs or names a concrete

  * `[ ]`   `adapters/hash-to-scalar/src/factory/mod.rs`
    * `[ ]`   Adds `#[cfg(test)] mod test;` to the wiring `hash-to-scalar/keccak256` authored
    * `[ ]`   `pub fn create_hash_to_scalar<S: ISampleUniformScalar + Clone>(_deps: &CreateHashToScalarDeps, params: CreateHashToScalarParams, _payload: CreateHashToScalarPayload) -> CreateHashToScalarReturn<S>`, a `match` on `params.concrete` whose `HashToScalarConcrete::Keccak256` arm returns the refusal when `Keccak256HashToScalar::DECLARATION.identifier` is not `params.identifier`, then binds the concrete by `let Ok(hasher) = Keccak256HashToScalar::try_new(Keccak256HashToScalarConstructorParams);` and returns `Ok(CreateHashToScalarSuccessReturn { adapter: Box::new(hasher), declaration: Keccak256HashToScalar::DECLARATION })`
    * `[ ]`   Imports `Keccak256HashToScalar` and `Keccak256HashToScalarConstructorParams` from `crate::keccak256::provides`, `ISampleUniformScalar` from `pairing`, and this module's types from `interface`
    * `[ ]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[ ]`   `adapters/hash-to-scalar/src/factory/provides.rs`
    * `[ ]`   Adds `pub use super::create_hash_to_scalar;` to the re-exports `hash-to-scalar/keccak256` authored

  * `[ ]`   `adapters/hash-to-scalar/tests/integration_test.rs`
    * `[ ]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`; imports `create_hash_to_scalar`, `CreateHashToScalarDeps`, `CreateHashToScalarPayload`, `build_create_hash_to_scalar_params`, `CreateHashToScalarParamsOverrides`, `HashToScalarConcrete`, `HashToScalarIdentifier`, `HashToScalarParams`, `HashToScalarPayload`, `DomainTag`, `build_domain_tag`, and `DomainTagConstructorParamsOverrides` from `hash_to_scalar`; the `pairing` names the context slice lists with `CreatePairingDeps`, `CreatePairingPayload`, `EncodeScalarParams`, and `EncodeScalarPayload`; the `encoding` names the context slice lists; `DerivationContext` and the domain builders and overrides that make the reference context from `domain`; and `hex::decode`; the builders are reached through each crate's `mocks` feature, which the workspace's test and check commands enable with `--all-features`
    * `[ ]`   A test-local `EncodeContext`, a struct with `context: DerivationContext`, implementing `IEncodingConsumer` with `type Output = Vec<u8>;` and a `consume_encoding<E: IEncoderAdapter + IDecoderAdapter>` that constructs `DerivationContextDescription` by `let Ok(description) = DerivationContextDescription::try_new(DerivationContextDescriptionConstructorParams);`, encodes `self.context` through `payload.adapter.encode(EncodeParams { description: &description }, &self.context)`, unpacked by `let Ok(success) = … else { panic!(…) };`, and returns `success.bytes`
    * `[ ]`   A test-local `FactoryScalarProbe`, a struct with `tag: DomainTag` and `message: Vec<u8>`, implementing `IPairingConsumer` with `type Output = Vec<u8>;` and a `consume_pairing<P: IPairingAdapter>` that obtains the adapter from `create_hash_to_scalar::<P::Scalar>` with `build_create_hash_to_scalar_params` overridden by `concrete: Some(HashToScalarConcrete::Keccak256)` and `identifier: Some(HashToScalarIdentifier::Keccak256V1)`, calls `hash_to_scalar(HashToScalarParams { tag: &self.tag }, HashToScalarPayload { message: &self.message })` on it, encodes the scalar through `payload.adapter.encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar })`, each call unpacked by `let Ok(…) = … else { panic!(…) };`, and returns the exposed 32 bytes cloned into a `Vec<u8>`
    * `[ ]`   The reference context is the one `encoding/derivation_context`'s tests build; the reference tag is `build_domain_tag` with `bytes: Some(b"ChainTorrent hash-to-scalar test".to_vec())`; the reference message's BN254 and BLS12-381 scalars are the independent vectors `hash-to-scalar/keccak256`'s tests state
    * `[ ]`   `the_keccak256_concrete_from_the_factory_maps_the_encoded_reference_context_to_the_independent_bn254_scalar`: contract: the reference context encoded through the encoding factory, hashed under the reference tag through the hash-to-scalar factory's concrete, and reduced by the BN254 scalar the pairing factory resolves, matches the independent vector, with nothing mocked; arrange the message from `create_encoding` with `build_create_encoding_params` overridden by `concrete: Some(EncodingConcrete::Abi)` and `identifier: Some(EncodingIdentifier::EthereumAbiV1)` and `CreateEncodingDeps { consumer: EncodeContext { context } }` holding the reference context, unpacked by `let Ok(success) = … else { panic!(…) };`, and `CreatePairingDeps { consumer: FactoryScalarProbe { tag, message } }`; act `create_pairing` with `build_create_pairing_params` overridden by `concrete: Some(PairingConcrete::Bn254Arkworks)`, unpacked by `let Ok(success) = … else { panic!(…) };`; assert `success.output` equals the reference message's BN254 scalar
    * `[ ]`   `the_keccak256_concrete_from_the_factory_maps_the_encoded_reference_context_to_the_independent_bls12_381_scalar`: the same with `PairingConcrete::Bls12381Arkworks` against the reference message's BLS12-381 scalar
    * `[ ]`   Every test block carries the full `Contract`, `Arrange`, `Act`, `Assert` header and the inline markers; nothing is mocked, since `alloy`, `sha3`, and the curve libraries are the outer edges

  * `[ ]`   `directionality`
    * `[ ]`   The `factory` module depends on the `keccak256` concrete through `crate::keccak256::provides`, on `domain_tag`, and on `pairing`; `keccak256` depends on the `factory` module's surface, the family form's recorded cycle; the crate's public surface is the `domain_tag` and `factory` modules' `provides`
    * `[ ]`   Among repository crates the crate depends on `crates/domain` and `adapters/pairing` at runtime and on `adapters/encoding` for its integration test only; none of them names this crate; no other cycle
    * `[ ]`   `kem/bb1_depth_one` consumes the family for the identity mapping and `proof/schnorr_fs/challenge` for the challenge, each through `create_hash_to_scalar` and `IHashToScalarAdapter`

  * `[ ]`   `requirements`
    * `[ ]`   `adapters/hash-to-scalar/Cargo.toml` carries exactly the dev-dependencies stated above, and every other table `hash-to-scalar/keccak256` stated is unchanged
    * `[ ]`   `create_hash_to_scalar_returns_the_keccak256_concrete_and_its_declaration_for_a_pairing_scalar` passes; the unsupported-identifier refusal is fixed by the return union and the `match` and has no unit test until a further identifier exists
    * `[ ]`   `the_keccak256_concrete_from_the_factory_maps_the_encoded_reference_context_to_the_independent_bn254_scalar` and `the_keccak256_concrete_from_the_factory_maps_the_encoded_reference_context_to_the_independent_bls12_381_scalar` pass (CR-11, the hash-to-scalar mapping over an encoded domain value reached through the encoding, pairing, and hash-to-scalar families' surfaces on both curves)
    * `[ ]`   `cargo check --workspace --all-targets --all-features`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo fmt --all --check`, and `cargo deny check` complete without error or warning in every target, the `keccak256` concrete's unused-item warnings having no remaining cause
    * `[ ]`   `sha3` is named nowhere outside `adapters/hash-to-scalar/src/keccak256`, and no code outside `adapters/hash-to-scalar` can name `Keccak256HashToScalar`
    * `[ ]`   The pairing adapters and key derivation milestone's exit holds: CR-10 on both curves through the pairing factory, the benchmark's record, the domain identifiers and derivation context, the derivation context's ABI encoding against its known-answer vector, and CR-11's derivation and hash-to-scalar against independent implementations' vectors, every crate building and passing the Rust CI definition on Windows, macOS, and Linux

  * `[ ]`   **Commit** `feat(harness): pairing, encoding, key-derivation, and hash-to-scalar families with the domain identifiers and derivation context`
    * `[ ]`   Structural: the `adapters/pairing` crate with its factory and four concretes; the `apps/harness-crypto` crate with its benchmark; the `domain` crate's asset identity, deployment identity, suite identifier, parameter-set identifier, group index, piece geometry, and derivation context modules; the `adapters/encoding` crate with its factory, ABI concrete, and derivation-context description; the `adapters/kdf` crate with its factory and BLAKE3 keyed concrete; the `adapters/hash-to-scalar` crate with its factory, domain tag, and Keccak-256 concrete
    * `[ ]`   Behavioral: group arithmetic, precompile encodings, and scalar sampling on BN254 and BLS12-381 across arkworks and halo2curves; the pairing benchmark; the domain identifiers and the derivation context admitted only under their invariants; the derivation context encoded and decoded in its one ABI byte form; keys derived under a fixed context string per purpose; domain-tagged messages mapped to scalars on either curve
    * `[ ]`   Contract: `IPairingAdapter` with `ISampleUniformScalar` and `create_pairing`; `IEncodingContract`, `IEncoderAdapter`, `IDecoderAdapter`, and `create_encoding`; `IKeyDerivationAdapter` with `DerivationPurpose` and `create_key_derivation`; `DomainTag`, `IHashToScalarAdapter`, and `create_hash_to_scalar`; each family's identifier, declaration, and interface version

## Credential KEM

* `[ ]`   `pairing/bn254_arkworks` **BN254 arkworks concrete gains the scalar-field arithmetic, source-group negation, identity tests, pairing into the target group, and target-group encoding the credential KEM, the envelope, and the delivery proof compute with; authors the pairing family's arithmetic trait and its mock**

  * `[ ]`   `objective`
    * `[ ]`   Problem: the credential KEM encapsulates and decapsulates a target-group value `K` the key-derivation family consumes as bytes, checks validity and well-formedness as pairing equations whose sides are divided, and refuses a trivial identity element; the envelope decrypts by removing a masked source-group element and rejects identity-element keys; the delivery proof computes Schnorr responses in the scalar field; and the decrypted credential is a pair of source-group elements that must be zeroized; the family's generic interface offers none of these, so every one passes through the family, and no module outside a concrete names a curve library (CR-04; CR-07; CR-08; CR-09; `docs/research/cryptography.md`'s Credential KEM, Key Agreement, and Delivery Proof statements)
    * `[ ]`   Functional: the family's arithmetic trait extends the generic interface with the scalar field's addition, multiplication, and negation modulo the group order
    * `[ ]`   Functional: it negates a point in either source group, the identity negating to the identity
    * `[ ]`   Functional: it reports whether a point in either source group is the identity
    * `[ ]`   Functional: it computes the product of the pairings of a list of first-group and second-group pairs as a value of the target group, an empty list yielding the target group's identity; division is realized by negating a first-group input, so no target-group arithmetic exists above the concrete
    * `[ ]`   Functional: it encodes a target-group value as its twelve base-field coefficients in the tower order of the degree-twelve extension, `c0.c0.c0`, `c0.c0.c1`, `c0.c1.c0`, `c0.c1.c1`, `c0.c2.c0`, `c0.c2.c1`, `c1.c0.c0`, `c1.c0.c1`, `c1.c1.c0`, `c1.c1.c1`, `c1.c2.c0`, `c1.c2.c1`, each the coefficient's big-endian canonical integer at the base field's byte width, returned inside a `Secret`, since `K` is decrypt-capable
    * `[ ]`   Functional: the trait requires the source-group types to implement `Zeroize` and its target-group type to implement `Zeroize`, and the BN254 arkworks concrete's first-group, second-group, and target-group types zeroize their value through `Zeroize` and on drop
    * `[ ]`   Non-functional: `ark-bn254`, `ark-ec`, and `ark-ff` remain named only inside `adapters/pairing/src/bn254_arkworks`; the arithmetic trait is a separate trait so the other concretes keep compiling until each implements it; the generic interface, the factory function, and the declaration are unchanged

  * `[ ]`   `role`
    * `[ ]`   Adapter: the pairing family's first concrete, and the first source file that requires the family's arithmetic trait and its mock, which it authors in the family's `factory` module as its producers
    * `[ ]`   Does not change `IPairingAdapter`, `IPairingConsumer`, `create_pairing`, or any existing method's behavior, and does not bind the arithmetic trait into the factory; `pairing/factory` requires it of every concrete a consumer receives once every concrete implements it
    * `[ ]`   Does not implement the arithmetic trait for the BN254 `halo2curves` concrete or either BLS12-381 concrete; each is its own node
    * `[ ]`   Does not compute in the target group beyond the pairing product and its encoding: no target-group multiplication, exponentiation, or inversion is exposed
    * `[ ]`   Does not map a target-group value to a key; the key-derivation family derives from the encoding
    * `[ ]`   Does not carry a commit

  * `[ ]`   `module`
    * `[ ]`   Bounded context: the `factory` module's arithmetic trait with its target-group associated type, every method's params, payload, success, and return types, and its mock; the `bn254_arkworks` concrete's target-group type, its implementation of the arithmetic trait, its source-group and target-group zeroization, and the builder default for its target-group type
    * `[ ]`   Edits `adapters/pairing/src/factory/interface.rs`, `adapters/pairing/src/factory/mock.rs`, and the `bn254_arkworks` module's `interface.rs`, `interaction.spec.md`, `mock.rs`, `test.rs`, and `mod.rs`; the crate's manifest, barrel, `provides` files, and `factory/mod.rs` are unchanged
    * `[ ]`   Outside: the KEM, envelope, and proof algebra built on the trait, the key derivation from the target-group encoding, the cross-library agreement of the encoding, and the factory's binding of the trait

  * `[ ]`   `deps`
    * `[ ]`   The `factory` module's existing interface, same crate: `IPairingAdapter` with its associated types `Scalar`, `G1`, and `G2`, and `PairingProductTerm`, which the pairing product reuses for its terms
    * `[ ]`   `domain`, the existing runtime dependency: `Secret` and `SecretConstructorParams` for the target-group encoding; `build_secret` and `SecretConstructorParamsOverrides` through the existing `mocks` feature for the encoding's builder
    * `[ ]`   `zeroize`, the existing runtime dependency: the `Zeroize` trait the arithmetic trait's bounds name, its implementation for `Vec<Z: Zeroize>`, and its implementations for arkworks' short-Weierstrass affine points and `PairingOutput`
    * `[ ]`   `ark-bn254`, `ark-ec`, and `ark-ff`, the existing runtime dependencies, named only in `bn254_arkworks`
    * `[ ]`   `hex`, the existing dev-dependency, for the vectors
    * `[ ]`   `core::convert::Infallible`, standard library, the error arm of every new method; `core::marker::PhantomData`, standard library, in `factory/mock.rs`
    * `[ ]`   No new external crate; `adapters/pairing/Cargo.toml` is unchanged; no reverse dependency

  * `[ ]`   `context_slice`
    * `[ ]`   From `ark-ff`: `Fr`'s `+`, `*`, and unary `-` modulo the group order; the public fields `c0` and `c1` of the degree-two and degree-twelve extensions and `c0`, `c1`, and `c2` of the degree-six extension; `PrimeField::into_bigint()` and `BigInteger::to_bytes_be()` on `Fq`
    * `[ ]`   From `ark-ec`: unary `-` on the short-Weierstrass affine point, returning `(x, -y)` and the identity for the identity; `AffineRepr::is_zero()`, true exactly for the identity; `pairing::Pairing::multi_pairing(a, b)` over iterators of borrowed affine points, returning `PairingOutput<Bn254>`, whose public field `0` is the `Fq12` value and whose identity is `Fq12::one()`; `Pairing::pairing(p, q)`; `Zeroize` for `Affine` and for `PairingOutput`
    * `[ ]`   From `domain`: `Secret::try_new(SecretConstructorParams { value })` returning `Result<Secret<T>, Infallible>`, and `Secret::expose(&self) -> &T`

  * `[ ]`   `adapters/pairing/src/factory/interface.rs`
    * `[ ]`   `add_scalar(&self, params: AddScalarParams, payload: AddScalarPayload<Self::Scalar>) -> AddScalarReturn<Self::Scalar>`: the fieldless `AddScalarParams`; `AddScalarPayload<S>` with `pub left: S` and `pub right: S`; `AddScalarSuccessReturn<S>` with `pub sum: S`; `AddScalarReturn<S>`, the alias `Result<AddScalarSuccessReturn<S>, Infallible>`
    * `[ ]`   `mul_scalar`, the same shape under the `MulScalar` prefix, its success return holding `pub product: S`
    * `[ ]`   `neg_scalar(&self, params: NegScalarParams, payload: NegScalarPayload<Self::Scalar>) -> NegScalarReturn<Self::Scalar>`: the fieldless `NegScalarParams`; `NegScalarPayload<S>` with `pub scalar: S`; `NegScalarSuccessReturn<S>` with `pub negation: S`; `NegScalarReturn<S>`, the alias `Result<NegScalarSuccessReturn<S>, Infallible>`
    * `[ ]`   `neg_g1(&self, params: NegG1Params, payload: NegG1Payload<Self::G1>) -> NegG1Return<Self::G1>`: the fieldless `NegG1Params`; `NegG1Payload<G>` with `pub point: G`; `NegG1SuccessReturn<G>` with `pub negation: G`; `NegG1Return<G>`, the alias `Result<NegG1SuccessReturn<G>, Infallible>`
    * `[ ]`   `neg_g2`, the same shape under the `NegG2` prefix over `Self::G2`
    * `[ ]`   `is_identity_g1(&self, params: IsIdentityG1Params, payload: IsIdentityG1Payload<Self::G1>) -> IsIdentityG1Return`: the fieldless `IsIdentityG1Params`; `IsIdentityG1Payload<G>` with `pub point: G`; `IsIdentityG1SuccessReturn` with `pub is_identity: bool`; `IsIdentityG1Return`, the alias `Result<IsIdentityG1SuccessReturn, Infallible>`
    * `[ ]`   `is_identity_g2`, the same shape under the `IsIdentityG2` prefix over `Self::G2`
    * `[ ]`   `pairing_product(&self, params: PairingProductParams, payload: PairingProductPayload<Self::G1, Self::G2>) -> PairingProductReturn<Self::Gt>`: the fieldless `PairingProductParams`; `PairingProductPayload<G1, G2>` with `pub terms: Vec<PairingProductTerm<G1, G2>>`; `PairingProductSuccessReturn<T>` with `pub product: T`; `PairingProductReturn<T>`, the alias `Result<PairingProductSuccessReturn<T>, Infallible>`
    * `[ ]`   `encode_gt(&self, params: EncodeGtParams, payload: EncodeGtPayload<Self::Gt>) -> EncodeGtReturn`: the fieldless `EncodeGtParams`; `EncodeGtPayload<T>` with `pub value: T`; `EncodeGtSuccessReturn` with `pub bytes: Secret<Vec<u8>>`; `EncodeGtReturn`, the alias `Result<EncodeGtSuccessReturn, Infallible>`
    * `[ ]`   `IPairingArithmetic`, the arithmetic trait, `pub trait IPairingArithmetic: IPairingAdapter<G1: Zeroize, G2: Zeroize>` with `type Gt: Zeroize;` and the methods above, each taking `&self`, its params, and its payload, and returning its own return alias
    * `[ ]`   No derives on any new type; every item already in this file is unchanged; the file's existing imports of `domain::Secret`, `zeroize::Zeroize`, and `core::convert::Infallible` serve the new items; names no vendor and no concrete

  * `[ ]`   `adapters/pairing/src/bn254_arkworks/interface.rs`
    * `[ ]`   `Bn254ArkworksGt`, a struct with no derives and one field `pub(super) value: ark_ec::pairing::PairingOutput<ark_bn254::Bn254>`, the target-group value
    * `[ ]`   Every item already in this file is unchanged

  * `[ ]`   `adapters/pairing/src/bn254_arkworks/interaction.spec.md`
    * `[ ]`   Adds a section per new method in the file's existing table form, below the existing sections and above `Ordering and edges`
    * `[ ]`   `add_scalar`: one branch; dependency call `Fr`'s `+` over `payload.left.value` and `payload.right.value`; outcome `Ok(AddScalarSuccessReturn { sum })`, the sum modulo the group order in the owned scalar type; the payload, holding both scalars, drops at the end of the call and both are zeroized
    * `[ ]`   `mul_scalar`: one branch; dependency call `Fr`'s `*`; outcome `Ok(MulScalarSuccessReturn { product })` modulo the group order; the payload's scalars are zeroized as it drops
    * `[ ]`   `neg_scalar`: one branch; dependency call `Fr`'s unary `-`; outcome `Ok(NegScalarSuccessReturn { negation })`, the group order minus the scalar, and zero for zero
    * `[ ]`   `neg_g1`: one branch; dependency call the affine point's unary `-`; outcome `Ok(NegG1SuccessReturn { negation })`, `(x, p - y)` for a point `(x, y)` and the identity for the identity
    * `[ ]`   `neg_g2`: the same over the second group
    * `[ ]`   `is_identity_g1`: one branch; dependency call `AffineRepr::is_zero()` on the payload point; outcome `Ok(IsIdentityG1SuccessReturn { is_identity })`, `true` exactly for the identity
    * `[ ]`   `is_identity_g2`: the same over the second group
    * `[ ]`   `pairing_product`: one branch; the terms are split into a `Vec<G1Affine>` and a `Vec<G2Affine>` in term order; dependency call `Bn254::multi_pairing(&g1s, &g2s)`, then `zeroize` on both vectors; outcome `Ok(PairingProductSuccessReturn { product })` holding the `PairingOutput` in the owned target-group type; an empty term list yields the target group's identity
    * `[ ]`   `encode_gt`: one branch; dependency call `into_bigint().to_bytes_be()` on each of the twelve `Fq` coefficients of `payload.value.value.0` in the tower order the objective states, appended in that order into one buffer of 384 bytes; outcome `Ok(EncodeGtSuccessReturn { bytes })`, the buffer moved into a `Secret` by `let Ok(bytes) = Secret::try_new(SecretConstructorParams { value: buffer });`; the target group's identity encodes as 31 zero bytes, `01`, and 352 zero bytes
    * `[ ]`   Zeroization: `Bn254ArkworksG1`, `Bn254ArkworksG2`, and `Bn254ArkworksGt` each zeroize their `value` through their `Zeroize` implementation and on drop, as `Bn254ArkworksScalar` does, so every clone a consumer places in a payload is zeroized when the payload drops
    * `[ ]`   `params` carries no control and is not read in any new method

  * `[ ]`   `adapters/pairing/src/factory/mock.rs`
    * `[ ]`   For each new generic struct, an overrides struct named by the type with the suffix `Overrides`, `#[derive(Default)]`, one `Option` per field over the struct's type parameters, and a builder `build_` followed by the type's name in snake case, taking the overrides and returning the production type, each type parameter bounded by `Default`, an omitted field taking `Default::default()` of its type parameter unless stated: `AddScalarPayload`, `AddScalarSuccessReturn`, `MulScalarPayload`, `MulScalarSuccessReturn`, `NegScalarPayload`, `NegScalarSuccessReturn`, `NegG1Payload`, `NegG1SuccessReturn`, `NegG2Payload`, `NegG2SuccessReturn`, `IsIdentityG1Payload`, `IsIdentityG2Payload`, `PairingProductPayload` with `terms` defaulting to an empty `Vec`, `PairingProductSuccessReturn`, and `EncodeGtPayload`
    * `[ ]`   The new non-generic builders: `IsIdentityG1SuccessReturnOverrides` with `build_is_identity_g1_success_return` and `IsIdentityG2SuccessReturnOverrides` with `build_is_identity_g2_success_return`, `is_identity` defaulting to `false`; `EncodeGtSuccessReturnOverrides` with `build_encode_gt_success_return`, `bytes` defaulting to `build_secret(SecretConstructorParamsOverrides::default())`
    * `[ ]`   `MockIPairingAdapter` gains the type parameter `Gt` and the field `pub gt: PhantomData<Gt>`, becoming `MockIPairingAdapter<S, G1, G2, Gt>`; its implementation of `IPairingAdapter` keeps its bounds and places none on `Gt`
    * `[ ]`   `impl<S, G1, G2, Gt> IPairingArithmetic for MockIPairingAdapter<S, G1, G2, Gt>` for `S: ISampleUniformScalar + Clone + Default`, `G1: Zeroize + Clone + Default`, `G2: Zeroize + Clone + Default`, and `Gt: Zeroize + Default`, with `type Gt = Gt;` and every method returning `Ok` holding its success return's builder called with `Default::default()`; a test needing other behavior implements the trait on its own local struct
    * `[ ]`   `ConsumePairingPayloadOverrides` gains the type parameter `Gt`, its `adapter` field becoming `Option<MockIPairingAdapter<S, G1, G2, Gt>>`; `build_consume_pairing_payload` gains the type parameter `Gt` and returns `ConsumePairingPayload<MockIPairingAdapter<S, G1, G2, Gt>>`, its default adapter setting `gt: PhantomData`
    * `[ ]`   No builder for the new fieldless params; no corruptions type and no invalidator, since no new struct arrives as untrusted data
    * `[ ]`   Every other symbol in this file is unchanged; the file's imports gain the new types and `IPairingArithmetic` from `super::interface`

  * `[ ]`   `adapters/pairing/src/bn254_arkworks/mock.rs`
    * `[ ]`   `impl Default for Bn254ArkworksGt` returning the pairing of the two generators, `Bn254::pairing(G1Affine::generator(), G2Affine::generator())`, the builder default the family's generic builders and `MockIPairingAdapter` read through `Default`
    * `[ ]`   The file's imports gain `ark_bn254::Bn254` and `ark_ec::pairing::Pairing`; every existing default is unchanged

  * `[ ]`   `adapters/pairing/src/bn254_arkworks/test.rs`
    * `[ ]`   Every existing test and constant is unchanged; the new tests are appended; the imports gain `IPairingArithmetic` and the new params, overrides, and builders from `crate::factory::provides`
    * `[ ]`   The new constants, as hex: zero, 32 zero bytes; one, 31 zero bytes and `01`; six, 31 zero bytes and `06`; the negated first-group generator, 31 zero bytes and `01` followed by `p - 2` = `30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd45`; the target group's identity encoding is written in each test that uses it as `vec![0u8; 384]` with index `31` set to `1`
    * `[ ]`   `add_scalar_of_two_and_three_is_five`: contract: the sum of two scalars is their sum in the scalar field; arrange the decoded scalars two and three in `build_add_scalar_payload`; act `add_scalar`, then `encode_scalar`; assert the exposed bytes equal five
    * `[ ]`   `add_scalar_reduces_modulo_the_group_order`: contract: a sum at or above the group order wraps; arrange `r - 1` and two; act `add_scalar`, then `encode_scalar`; assert the exposed bytes equal one
    * `[ ]`   `mul_scalar_of_two_and_three_is_six`: arrange two and three in `build_mul_scalar_payload`; act `mul_scalar`, then `encode_scalar`; assert the exposed bytes equal six
    * `[ ]`   `mul_scalar_reduces_modulo_the_group_order`: contract: a product at or above the group order wraps; arrange `r - 1` twice; act `mul_scalar`, then `encode_scalar`; assert the exposed bytes equal one, since `(r - 1)^2` is one modulo `r`
    * `[ ]`   `neg_scalar_of_one_is_the_group_order_minus_one`: arrange one in `build_neg_scalar_payload`; act `neg_scalar`, then `encode_scalar`; assert the exposed bytes equal `r - 1`
    * `[ ]`   `neg_scalar_of_zero_is_zero`: arrange zero; act `neg_scalar`, then `encode_scalar`; assert the exposed bytes equal zero
    * `[ ]`   `neg_g1_of_the_generator_negates_its_y_coordinate`: contract: a point's negation keeps `x` and replaces `y` by `p - y`; arrange the generator in `build_neg_g1_payload`; act `neg_g1`, then `encode_g1`; assert the bytes equal the negated first-group generator vector
    * `[ ]`   `neg_g1_of_the_identity_is_the_identity`: arrange the point `decode_g1` reads from 64 zero bytes; act `neg_g1`, then `encode_g1`; assert the bytes are 64 zero bytes
    * `[ ]`   `neg_g2_of_the_generator_sums_with_the_generator_to_the_identity`: contract: a second-group point plus its negation is the identity; arrange the second-group generator; act `neg_g2`, then `add_g2` of the generator and the negation, then `encode_g2`; assert the bytes are 128 zero bytes
    * `[ ]`   `is_identity_g1_is_true_for_the_identity`: arrange the point `decode_g1` reads from 64 zero bytes in `build_is_identity_g1_payload`; act `is_identity_g1`; assert `is_identity` is `true`
    * `[ ]`   `is_identity_g1_is_false_for_the_generator`: arrange the first-group generator; act `is_identity_g1`; assert `is_identity` is `false`
    * `[ ]`   `is_identity_g2_is_true_for_the_identity`: the same over the point `decode_g2` reads from 128 zero bytes and `is_identity_g2`
    * `[ ]`   `is_identity_g2_is_false_for_the_generator`: the same over the second-group generator
    * `[ ]`   `pairing_product_of_no_terms_encodes_as_the_target_group_identity`: contract: the empty product is the target group's identity, which encodes with the coefficient `c0.c0.c0` first; arrange `build_pairing_product_payload` with its default empty terms; act `pairing_product`, then `encode_gt`; assert the exposed bytes equal the target group's identity encoding
    * `[ ]`   `pairing_product_of_the_generators_is_not_the_target_group_identity`: contract: the pairing is non-degenerate; arrange the one term `(g1, g2)` from `build_pairing_product_term`; act `pairing_product`, then `encode_gt`; assert the exposed bytes are 384 bytes and differ from the target group's identity encoding
    * `[ ]`   `pairing_product_is_bilinear`: contract: a scalar moves between the arguments of a pairing; arrange the terms `(g1 · 2, g2 · 3)`, `(g1 · 6, g2)`, and `(g1 · 5, g2)`, each a single-term product; act `pairing_product` over each, then `encode_gt` over each; assert the first two encodings are equal and differ from the third
    * `[ ]`   `pairing_product_multiplies_its_terms`: contract: a list of terms yields the product of their pairings; arrange the terms `(g1, g2)` twice as one list, `(g1 · 2, g2)` as a single-term list, and `(g1, g2)` as a single-term list; act `pairing_product` over each, then `encode_gt` over each; assert the first two encodings are equal and differ from the third
    * `[ ]`   `pairing_product_of_a_pairing_and_its_first_group_negation_is_the_target_group_identity`: contract: negating a first-group input divides by its pairing; arrange the terms `(g1, g2)` and `(-g1, g2)`, the negation from `neg_g1`; act `pairing_product`, then `encode_gt`; assert the exposed bytes equal the target group's identity encoding
    * `[ ]`   Every new test block carries the full `Contract`, `Arrange`, `Act`, `Assert` header and the inline markers; every payload is built through its family builder with only the overrides the test depends on; scalars come from `decode_scalar` over the vectors and points from the generators, `mul_g1`, `mul_g2`, or `decode_g1` and `decode_g2`

  * `[ ]`   `construction`
    * `[ ]`   A target-group value is produced only by `pairing_product`; scalars and points produced by the new arithmetic are produced by the adapter alone; no consumer constructs any of them from library values

  * `[ ]`   `adapters/pairing/src/bn254_arkworks/mod.rs`
    * `[ ]`   `impl IPairingArithmetic for Bn254ArkworksPairing` with `type Gt = Bn254ArkworksGt;` and every method realizing its branch in the interaction spec
    * `[ ]`   `impl Zeroize for Bn254ArkworksG1` and `impl Zeroize for Bn254ArkworksG2`, each calling `self.value.zeroize()`; `impl Drop` for each calling `self.value.zeroize()`
    * `[ ]`   `impl Zeroize for Bn254ArkworksGt` calling `self.value.zeroize()`; `impl Drop for Bn254ArkworksGt` calling `self.value.zeroize()`
    * `[ ]`   The imports gain `IPairingArithmetic` and the new types from `crate::factory::provides` and `Bn254ArkworksGt` from `interface`
    * `[ ]`   Every existing item is unchanged; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[ ]`   `directionality`
    * `[ ]`   `bn254_arkworks` depends on the `factory` module's surface, on `domain`, on `zeroize`, and on the arkworks crates, as before; `IPairingArithmetic` depends on `IPairingAdapter` within the `factory` module; no new edge between crates and no new cycle
    * `[ ]`   `pairing/bn254_halo2curves`, `pairing/bls12_381_arkworks`, and `pairing/bls12_381_halo2curves` each implement `IPairingArithmetic`; `pairing/factory` then requires it of the concrete a consumer receives, and `kem/bb1_depth_one` consumes it

  * `[ ]`   `requirements`
    * `[ ]`   `adapters/pairing/Cargo.toml` is unchanged, and no `ark-` crate is named in the crate outside `adapters/pairing/src/bn254_arkworks`
    * `[ ]`   `cargo check --workspace --all-targets --all-features`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo fmt --all --check`, and `cargo deny check` complete without error or warning, with the other pairing concretes, the factory, and every crate that consumes the pairing family compiling unchanged
    * `[ ]`   Every existing test in `bn254_arkworks/test.rs`, `factory/test.rs`, and `adapters/pairing/tests/integration_test.rs` passes unchanged
    * `[ ]`   `add_scalar_of_two_and_three_is_five`, `add_scalar_reduces_modulo_the_group_order`, `mul_scalar_of_two_and_three_is_six`, `mul_scalar_reduces_modulo_the_group_order`, `neg_scalar_of_one_is_the_group_order_minus_one`, and `neg_scalar_of_zero_is_zero` pass (CR-09, the scalar arithmetic the delivery proof's responses use)
    * `[ ]`   `neg_g1_of_the_generator_negates_its_y_coordinate`, `neg_g1_of_the_identity_is_the_identity`, `neg_g2_of_the_generator_sums_with_the_generator_to_the_identity`, `is_identity_g1_is_true_for_the_identity`, `is_identity_g1_is_false_for_the_generator`, `is_identity_g2_is_true_for_the_identity`, and `is_identity_g2_is_false_for_the_generator` pass (CR-04 envelope decryption and identity-key rejection; CR-08 trivial identity-element refusal)
    * `[ ]`   `pairing_product_of_no_terms_encodes_as_the_target_group_identity`, `pairing_product_of_the_generators_is_not_the_target_group_identity`, `pairing_product_is_bilinear`, `pairing_product_multiplies_its_terms`, and `pairing_product_of_a_pairing_and_its_first_group_negation_is_the_target_group_identity` pass (CR-08, the encapsulated value and the validity, well-formedness, and decapsulation equations)
    * `[ ]`   `Bn254ArkworksG1`, `Bn254ArkworksG2`, and `Bn254ArkworksGt` implement `Zeroize` and `Drop`, fixed by their implementations (CR-07)
    * `[ ]`   Code outside `adapters/pairing` naming `Bn254ArkworksGt` or anything under `bn254_arkworks` fails to compile; the crate's public surface is the `factory` module's `provides`

* `[ ]`   `workspace/cargo` **The `halo2curves` overlay: the workspace resolves `halo2curves` from the project's `gt-accessor` branch, and the dependency policy admits that source**

  * `[ ]`   `objective`
    * `[ ]`   Problem: the pairing family's arithmetic trait encodes a target-group value as its base-field coefficients, and `halo2curves` 0.10.0 declares its target-group type as `pub struct Gt(pub(crate) Fq12)` with no public way to read the `Fq12`, so the `halo2curves` concretes cannot implement the trait against the published crate (CR-08; CR-10)
    * `[ ]`   Functional: every workspace member that depends on `halo2curves` `0.10.0` resolves it from the branch `gt-accessor` of `https://github.com/tsylvester/halo2curves`, with no edit to any member's manifest, and `Cargo.lock` pins the branch's commit
    * `[ ]`   Functional: `cargo-deny` admits that git source and continues to deny every other unlisted registry or git source
    * `[ ]`   Non-functional: the overlay is carried as the `librqbit` overlay is: the branch tracks the tagged upstream `0.10.0` release, its one change is submitted upstream as a pull request, and the overlay entry is removed when the change merges and a release carries it

  * `[ ]`   `role`
    * `[ ]`   Infrastructure: edits to two configuration files with no types and no tests, exempt from the support-file structure
    * `[ ]`   Does not add, remove, or pin any dependency in any member's manifest; `adapters/pairing/Cargo.toml` keeps `halo2curves = "0.10.0"`, which the patch satisfies
    * `[ ]`   Does not author the accessor; the branch carrying it is external setup, as the `librqbit` overlay branch is: `impl_gt!` in `src/derive/pairing.rs` gains, inside its `impl $target` block, `pub fn inner(&self) -> &$base` returning `&self.0`, the one change on the branch; `pairing/bn254_halo2curves` and `pairing/bls12_381_halo2curves` consume it, and this node's build proof does not
    * `[ ]`   Does not edit `rust-toolchain.toml`, `.gitignore`, or `.github/workflows/rust.yml`
    * `[ ]`   Does not carry a commit

  * `[ ]`   `module`
    * `[ ]`   Bounded context: the repository root's `[patch.crates-io]` table in `Cargo.toml` and the `[sources]` table in `deny.toml`
    * `[ ]`   Outside: every member's manifest, the branch's contents, the concretes that consume the accessor, and the upstream pull request

  * `[ ]`   `deps`
    * `[ ]`   The branch `gt-accessor` of `https://github.com/tsylvester/halo2curves`, external git source, package `halo2curves` at version `0.10.0`, license MIT OR Apache-2.0, cut from the upstream `0.10.0` release; the patch's version must equal the version members require, which it does
    * `[ ]`   `cargo`, which reads `[patch.crates-io]`, and `cargo-deny`, which reads `[sources]`; no repository file is a dependency and no reverse dependency exists

  * `[ ]`   `Cargo.toml`
    * `[ ]`   `[patch.crates-io]` gains, after the `librqbit` entry, `halo2curves = { git = "https://github.com/tsylvester/halo2curves", branch = "gt-accessor" }`
    * `[ ]`   Every other table and key is unchanged

  * `[ ]`   `deny.toml`
    * `[ ]`   `[sources]` has `allow-git = ["https://github.com/tsylvester/rqbit", "https://github.com/tsylvester/halo2curves"]`
    * `[ ]`   Every other table and key is unchanged

  * `[ ]`   `directionality`
    * `[ ]`   The root configuration names the external source and nothing names the configuration; members depend on `halo2curves` through their own manifests as before; no cycle

  * `[ ]`   `requirements`
    * `[ ]`   `Cargo.toml` and `deny.toml` carry exactly the entries stated above, and every other key of both files is unchanged
    * `[ ]`   `Cargo.lock` records `halo2curves` `0.10.0` from `git+https://github.com/tsylvester/halo2curves?branch=gt-accessor` at a pinned commit, and no `halo2curves` from crates.io
    * `[ ]`   `cargo check --workspace --all-targets --all-features`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo fmt --all --check`, and `cargo deny check` complete without error or warning on Windows, macOS, and Linux
    * `[ ]`   Every existing test in the workspace passes unchanged, the branch's crate behaving as the published `0.10.0` in everything the workspace already calls

* `[ ]`   `pairing/bn254_halo2curves` **BN254 halo2curves concrete implements the pairing family's arithmetic trait: scalar-field arithmetic, source-group negation, identity tests, pairing into the target group, and target-group encoding**

  * `[ ]`   `objective`
    * `[ ]`   Problem: the KEM, the envelope, and the delivery proof consume the pairing family's arithmetic trait through whichever concrete the factory resolves, so the BN254 halo2curves concrete implements it with byte-identical target-group encodings to the arkworks concrete (CR-04; CR-07; CR-08; CR-09)
    * `[ ]`   Functional: `Bn254Halo2curvesPairing` implements `IPairingArithmetic` with every method's behavior as `pairing/bn254_arkworks` states it for the trait
    * `[ ]`   Functional: the target-group encoding reads the `Fq12` through `Gt::inner`, which the `halo2curves` overlay the `workspace/cargo` node binds supplies, and writes its twelve coefficients in the tower order `c0.c0.c0`, `c0.c0.c1`, `c0.c1.c0`, `c0.c1.c1`, `c0.c2.c0`, `c0.c2.c1`, `c1.c0.c0`, `c1.c0.c1`, `c1.c1.c0`, `c1.c1.c1`, `c1.c2.c0`, `c1.c2.c1`, each 32 bytes big-endian; halo2curves' BN256 tower, `Fq6 = Fq2[v] / (v^3 - (u + 9))` and `Fq12 = Fq6[w] / (w^2 - v)`, is arkworks' tower, so equal values encode to equal bytes
    * `[ ]`   Functional: `Bn254Halo2curvesG1`, `Bn254Halo2curvesG2`, and `Bn254Halo2curvesGt` clear their value through `Zeroize` and on drop
    * `[ ]`   Non-functional: `halo2curves` remains named only inside `adapters/pairing/src/bn254_halo2curves`; the family's interface, mock, and factory are unchanged

  * `[ ]`   `role`
    * `[ ]`   Adapter: a further concrete implementing the trait `pairing/bn254_arkworks` authored
    * `[ ]`   Does not edit `factory/interface.rs`, `factory/mock.rs`, or any other concrete
    * `[ ]`   Does not carry a commit

  * `[ ]`   `module`
    * `[ ]`   Bounded context: the `bn254_halo2curves` concrete's target-group type, its implementation of `IPairingArithmetic`, its source-group and target-group clearing, and the builder default for its target-group type
    * `[ ]`   Edits the `bn254_halo2curves` module's `interface.rs`, `interaction.spec.md`, `mock.rs`, `test.rs`, and `mod.rs`; nothing else in the crate changes

  * `[ ]`   `deps`
    * `[ ]`   The `factory` module's surface, same crate, through `crate::factory::provides`: `IPairingArithmetic` and its params, payloads, success returns, and builders, as `pairing/bn254_arkworks` authors them
    * `[ ]`   `halo2curves` `0.10.0` from the overlay branch `gt-accessor`, the existing runtime dependency, resolved through the `workspace/cargo` node's patch; supplies `Gt::inner(&self) -> &Fq12`
    * `[ ]`   `domain`, the existing runtime dependency: `Secret` and `SecretConstructorParams`
    * `[ ]`   `hex`, the existing dev-dependency, for the vectors
    * `[ ]`   `core::hint::black_box`, standard library, the existing clearing idiom
    * `[ ]`   No new external crate; `adapters/pairing/Cargo.toml` is unchanged

  * `[ ]`   `context_slice`
    * `[ ]`   From `halo2curves::ff`: `Fr`'s `+`, `*`, and unary `-` modulo the group order; `PrimeField::to_repr()` on `Fq`, 32 little-endian bytes
    * `[ ]`   From `halo2curves`: unary `-` on `G1Affine` and `G2Affine`; `PrimeCurveAffine::is_identity()` returning a `Choice`; `PrimeCurveAffine::identity()`; `Bn256::multi_miller_loop(&[(&G1Affine, &G2Affine)])` and `MillerLoopResult::final_exponentiation()` returning `Gt`; `Gt::identity()`; `Gt::inner()`; `c0()` and `c1()` on the degree-two and degree-twelve extensions and `c0()`, `c1()`, and `c2()` on the degree-six extension; `pairing::Engine::pairing(&G1Affine, &G2Affine)` for the builder default
    * `[ ]`   From `domain`: `Secret::try_new(SecretConstructorParams { value })` returning `Result<Secret<T>, Infallible>`

  * `[ ]`   `adapters/pairing/src/bn254_halo2curves/interface.rs`
    * `[ ]`   `Bn254Halo2curvesGt`, a struct with no derives and one field `pub(super) value: halo2curves::bn256::Gt`
    * `[ ]`   Every item already in this file is unchanged

  * `[ ]`   `adapters/pairing/src/bn254_halo2curves/interaction.spec.md`
    * `[ ]`   Adds a section per new method in the file's existing table form, below the existing sections and above `Ordering and edges`
    * `[ ]`   `add_scalar`, `mul_scalar`: one branch each; dependency call `Fr`'s `+` or `*` over the payload scalars' values; outcome `Ok` holding the sum or product modulo the group order; the payload's scalars are cleared as it drops
    * `[ ]`   `neg_scalar`: one branch; dependency call `Fr`'s unary `-`; outcome `Ok` holding the group order minus the scalar, and zero for zero
    * `[ ]`   `neg_g1`, `neg_g2`: one branch each; dependency call the affine point's unary `-`; outcome `Ok` holding the negation, the identity for the identity
    * `[ ]`   `is_identity_g1`, `is_identity_g2`: one branch each; dependency call `is_identity()` on the payload point; outcome `Ok` holding `bool::from` of the `Choice`
    * `[ ]`   `pairing_product`: one branch; the terms as a `Vec<(&G1Affine, &G2Affine)>` in term order; dependency call `Bn256::multi_miller_loop`, then `final_exponentiation()`; outcome `Ok(PairingProductSuccessReturn { product })` holding the `Gt` in the owned target-group type; an empty term list yields the target group's identity
    * `[ ]`   `encode_gt`: one branch; dependency call `inner()` on the payload's `Gt`, then each of the twelve `Fq` coefficients' `to_repr()` reversed to 32 big-endian bytes, appended in the tower order into one 384-byte buffer; outcome `Ok(EncodeGtSuccessReturn { bytes })`, the buffer moved into a `Secret` by `let Ok(bytes) = Secret::try_new(SecretConstructorParams { value: buffer });`
    * `[ ]`   `Ordering and edges` gains: halo2curves' affine points and `Gt` implement no `Zeroize`, so `Bn254Halo2curvesG1`, `Bn254Halo2curvesG2`, and `Bn254Halo2curvesGt` clear by setting `value` to `G1Affine::identity()`, `G2Affine::identity()`, or `Gt::identity()` and passing `&self.value` to `black_box`, in their `Zeroize` implementations and their `Drop`

  * `[ ]`   `adapters/pairing/src/bn254_halo2curves/mock.rs`
    * `[ ]`   `impl Default for Bn254Halo2curvesGt` returning `Bn256::pairing(&G1Affine::generator(), &G2Affine::generator())`
    * `[ ]`   The imports gain `halo2curves::bn256::Bn256` and `halo2curves::pairing::Engine`; every existing default is unchanged

  * `[ ]`   `adapters/pairing/src/bn254_halo2curves/test.rs`
    * `[ ]`   Every existing test and constant is unchanged; the new tests are appended; the imports gain `IPairingArithmetic` and the new params, overrides, and builders from `crate::factory::provides`
    * `[ ]`   The new constants, as hex, are those `pairing/bn254_arkworks` adds: zero, one, six, and the negated first-group generator, 31 zero bytes and `01` followed by `30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd45`; the target group's identity encoding is `vec![0u8; 384]` with index `31` set to `1`
    * `[ ]`   The new tests carry the names, arrangements, acts, and assertions `pairing/bn254_arkworks` states, with the subject constructed by `let Ok(pairing) = Bn254Halo2curvesPairing::try_new(Bn254Halo2curvesPairingConstructorParams);`: `add_scalar_of_two_and_three_is_five`, `add_scalar_reduces_modulo_the_group_order`, `mul_scalar_of_two_and_three_is_six`, `mul_scalar_reduces_modulo_the_group_order`, `neg_scalar_of_one_is_the_group_order_minus_one`, `neg_scalar_of_zero_is_zero`, `neg_g1_of_the_generator_negates_its_y_coordinate`, `neg_g1_of_the_identity_is_the_identity`, `neg_g2_of_the_generator_sums_with_the_generator_to_the_identity`, `is_identity_g1_is_true_for_the_identity`, `is_identity_g1_is_false_for_the_generator`, `is_identity_g2_is_true_for_the_identity`, `is_identity_g2_is_false_for_the_generator`, `pairing_product_of_no_terms_encodes_as_the_target_group_identity`, `pairing_product_of_the_generators_is_not_the_target_group_identity`, `pairing_product_is_bilinear`, `pairing_product_multiplies_its_terms`, and `pairing_product_of_a_pairing_and_its_first_group_negation_is_the_target_group_identity`
    * `[ ]`   Every new test block carries the full `Contract`, `Arrange`, `Act`, `Assert` header and the inline markers

  * `[ ]`   `construction`
    * `[ ]`   A target-group value is produced only by `pairing_product`; no consumer constructs one from library values

  * `[ ]`   `adapters/pairing/src/bn254_halo2curves/mod.rs`
    * `[ ]`   `impl IPairingArithmetic for Bn254Halo2curvesPairing` with `type Gt = Bn254Halo2curvesGt;` and every method realizing its branch in the interaction spec
    * `[ ]`   `impl Zeroize` and `impl Drop` for `Bn254Halo2curvesG1`, `Bn254Halo2curvesG2`, and `Bn254Halo2curvesGt`, each setting `value` to its type's identity and calling `black_box(&self.value)`
    * `[ ]`   The imports gain `IPairingArithmetic` and the new types from `crate::factory::provides`, `halo2curves::bn256::Gt`, and `Bn254Halo2curvesGt` from `interface`
    * `[ ]`   Every existing item is unchanged; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[ ]`   `directionality`
    * `[ ]`   `bn254_halo2curves` depends on the `factory` module's surface, on `domain`, on `zeroize`, and on `halo2curves`, as before; no new edge and no cycle

  * `[ ]`   `requirements`
    * `[ ]`   `cargo check --workspace --all-targets --all-features`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo fmt --all --check`, and `cargo deny check` complete without error or warning
    * `[ ]`   Every existing test in the crate passes unchanged, and every new test named above passes (CR-04, CR-08, CR-09 on BN254 over halo2curves)
    * `[ ]`   `Bn254Halo2curvesG1`, `Bn254Halo2curvesG2`, and `Bn254Halo2curvesGt` implement `Zeroize` and `Drop` (CR-07)
    * `[ ]`   `halo2curves` is named nowhere in the crate outside `adapters/pairing/src/bn254_halo2curves` and `adapters/pairing/src/bls12_381_halo2curves`

* `[ ]`   `pairing/bls12_381_arkworks` **BLS12-381 arkworks concrete implements the pairing family's arithmetic trait: scalar-field arithmetic, source-group negation, identity tests, pairing into the target group, and target-group encoding**

  * `[ ]`   `objective`
    * `[ ]`   Problem: the KEM, the envelope, and the delivery proof consume the pairing family's arithmetic trait through whichever concrete the factory resolves, and BLS12-381 is the primary verifier form on Base, so the BLS12-381 arkworks concrete implements it (CR-04; CR-07; CR-08; CR-09)
    * `[ ]`   Functional: `Bls12381ArkworksPairing` implements `IPairingArithmetic` with every method's behavior as `pairing/bn254_arkworks` states it for the trait
    * `[ ]`   Functional: the target-group encoding writes the twelve `Fq` coefficients of the `Fq12` in the tower order `c0.c0.c0`, `c0.c0.c1`, `c0.c1.c0`, `c0.c1.c1`, `c0.c2.c0`, `c0.c2.c1`, `c1.c0.c0`, `c1.c0.c1`, `c1.c1.c0`, `c1.c1.c1`, `c1.c2.c0`, `c1.c2.c1`, each 48 bytes big-endian, BLS12-381's base-field byte width, 576 bytes in all, without the 16-byte padding EIP-2537 adds to source-group coordinates
    * `[ ]`   Functional: `Bls12381ArkworksG1`, `Bls12381ArkworksG2`, and `Bls12381ArkworksGt` zeroize their value through `Zeroize` and on drop
    * `[ ]`   Non-functional: `ark-bls12-381`, `ark-ec`, and `ark-ff` remain named only inside the arkworks concretes; the family's interface, mock, and factory are unchanged

  * `[ ]`   `role`
    * `[ ]`   Adapter: a further concrete implementing the trait `pairing/bn254_arkworks` authored
    * `[ ]`   Does not edit `factory/interface.rs`, `factory/mock.rs`, or any other concrete
    * `[ ]`   Does not carry a commit

  * `[ ]`   `module`
    * `[ ]`   Bounded context: the `bls12_381_arkworks` concrete's target-group type, its implementation of `IPairingArithmetic`, its source-group and target-group zeroization, and the builder default for its target-group type
    * `[ ]`   Edits the `bls12_381_arkworks` module's `interface.rs`, `interaction.spec.md`, `mock.rs`, `test.rs`, and `mod.rs`; nothing else in the crate changes

  * `[ ]`   `deps`
    * `[ ]`   The `factory` module's surface, same crate, through `crate::factory::provides`: `IPairingArithmetic` and its params, payloads, success returns, and builders, as `pairing/bn254_arkworks` authors them
    * `[ ]`   `ark-bls12-381`, `ark-ec`, and `ark-ff`, the existing runtime dependencies; `domain` and `zeroize`, the existing runtime dependencies; `hex`, the existing dev-dependency
    * `[ ]`   No new external crate; `adapters/pairing/Cargo.toml` is unchanged

  * `[ ]`   `context_slice`
    * `[ ]`   From `ark-ff`: `Fr`'s `+`, `*`, and unary `-` modulo the group order; the public fields `c0` and `c1` of the degree-two and degree-twelve extensions and `c0`, `c1`, and `c2` of the degree-six extension; `PrimeField::into_bigint()` and `BigInteger::to_bytes_be()` on `Fq`, 48 bytes
    * `[ ]`   From `ark-ec`: unary `-` on the short-Weierstrass affine point; `AffineRepr::is_zero()`; `pairing::Pairing::multi_pairing` over iterators of borrowed affine points, returning `PairingOutput<Bls12_381>` with public field `0`; `Pairing::pairing(p, q)`; `Zeroize` for `Affine` and for `PairingOutput`
    * `[ ]`   From `domain`: `Secret::try_new(SecretConstructorParams { value })` returning `Result<Secret<T>, Infallible>`

  * `[ ]`   `adapters/pairing/src/bls12_381_arkworks/interface.rs`
    * `[ ]`   `Bls12381ArkworksGt`, a struct with no derives and one field `pub(super) value: ark_ec::pairing::PairingOutput<ark_bls12_381::Bls12_381>`
    * `[ ]`   Every item already in this file is unchanged

  * `[ ]`   `adapters/pairing/src/bls12_381_arkworks/interaction.spec.md`
    * `[ ]`   Adds a section per new method, in the file's existing form, below the existing sections and above its ordering and edges
    * `[ ]`   `add_scalar`, `mul_scalar`: one branch each; dependency call `Fr`'s `+` or `*`; outcome `Ok` holding the sum or product modulo the group order; the payload's scalars are zeroized as it drops
    * `[ ]`   `neg_scalar`: one branch; dependency call `Fr`'s unary `-`; outcome `Ok` holding the group order minus the scalar, and zero for zero
    * `[ ]`   `neg_g1`, `neg_g2`: one branch each; dependency call the affine point's unary `-`; outcome `Ok` holding the negation, the identity for the identity
    * `[ ]`   `is_identity_g1`, `is_identity_g2`: one branch each; dependency call `AffineRepr::is_zero()`; outcome `Ok` holding `is_identity`
    * `[ ]`   `pairing_product`: one branch; the terms split into a `Vec<G1Affine>` and a `Vec<G2Affine>` in term order; dependency call `Bls12_381::multi_pairing(&g1s, &g2s)`, then `zeroize` on both vectors; outcome `Ok(PairingProductSuccessReturn { product })` holding the `PairingOutput` in the owned target-group type; an empty term list yields the target group's identity
    * `[ ]`   `encode_gt`: one branch; dependency call `into_bigint().to_bytes_be()` on each of the twelve `Fq` coefficients of `payload.value.value.0` in the tower order, appended into one 576-byte buffer; outcome `Ok(EncodeGtSuccessReturn { bytes })`, the buffer moved into a `Secret` by `let Ok(bytes) = Secret::try_new(SecretConstructorParams { value: buffer });`; the target group's identity encodes as 47 zero bytes, `01`, and 528 zero bytes
    * `[ ]`   Zeroization: `Bls12381ArkworksG1`, `Bls12381ArkworksG2`, and `Bls12381ArkworksGt` each zeroize their `value` through their `Zeroize` implementation and on drop

  * `[ ]`   `adapters/pairing/src/bls12_381_arkworks/mock.rs`
    * `[ ]`   `impl Default for Bls12381ArkworksGt` returning `Bls12_381::pairing(G1Affine::generator(), G2Affine::generator())`
    * `[ ]`   The imports gain `ark_bls12_381::Bls12_381` and `ark_ec::pairing::Pairing`; every existing default is unchanged

  * `[ ]`   `adapters/pairing/src/bls12_381_arkworks/test.rs`
    * `[ ]`   Every existing test and constant is unchanged; the new tests are appended; the imports gain `IPairingArithmetic` and the new params, overrides, and builders from `crate::factory::provides`
    * `[ ]`   The new constants, as hex: zero, 32 zero bytes; one, 31 zero bytes and `01`; six, 31 zero bytes and `06`; the negated first-group generator in EIP-2537's 128 bytes, 16 zero bytes and `17f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb`, then 16 zero bytes and `114d1d6855d545a8aa7d76c8cf2e21f267816aef1db507c96655b9d5caac42364e6f38ba0ecb751bad54dcd6b939c2ca`, the generator's `y` subtracted from the base field modulus; the target group's identity encoding is `vec![0u8; 576]` with index `47` set to `1`
    * `[ ]`   The new tests carry the names, arrangements, acts, and assertions `pairing/bn254_arkworks` states, over this concrete's vectors, `r - 1` the existing `GROUP_ORDER_MINUS_ONE_HEX`, the first-group identity from 128 zero bytes, and the second-group identity from 256 zero bytes, with the subject constructed by `let Ok(pairing) = Bls12381ArkworksPairing::try_new(Bls12381ArkworksPairingConstructorParams);`: `add_scalar_of_two_and_three_is_five`, `add_scalar_reduces_modulo_the_group_order`, `mul_scalar_of_two_and_three_is_six`, `mul_scalar_reduces_modulo_the_group_order`, `neg_scalar_of_one_is_the_group_order_minus_one`, `neg_scalar_of_zero_is_zero`, `neg_g1_of_the_generator_negates_its_y_coordinate`, `neg_g1_of_the_identity_is_the_identity`, `neg_g2_of_the_generator_sums_with_the_generator_to_the_identity`, `is_identity_g1_is_true_for_the_identity`, `is_identity_g1_is_false_for_the_generator`, `is_identity_g2_is_true_for_the_identity`, `is_identity_g2_is_false_for_the_generator`, `pairing_product_of_no_terms_encodes_as_the_target_group_identity`, `pairing_product_of_the_generators_is_not_the_target_group_identity` asserting 576 bytes, `pairing_product_is_bilinear`, `pairing_product_multiplies_its_terms`, and `pairing_product_of_a_pairing_and_its_first_group_negation_is_the_target_group_identity`
    * `[ ]`   Every new test block carries the full `Contract`, `Arrange`, `Act`, `Assert` header and the inline markers

  * `[ ]`   `construction`
    * `[ ]`   A target-group value is produced only by `pairing_product`; no consumer constructs one from library values

  * `[ ]`   `adapters/pairing/src/bls12_381_arkworks/mod.rs`
    * `[ ]`   `impl IPairingArithmetic for Bls12381ArkworksPairing` with `type Gt = Bls12381ArkworksGt;` and every method realizing its branch in the interaction spec
    * `[ ]`   `impl Zeroize` and `impl Drop` for `Bls12381ArkworksG1`, `Bls12381ArkworksG2`, and `Bls12381ArkworksGt`, each calling `self.value.zeroize()`
    * `[ ]`   The imports gain `IPairingArithmetic` and the new types from `crate::factory::provides` and `Bls12381ArkworksGt` from `interface`
    * `[ ]`   Every existing item is unchanged; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[ ]`   `directionality`
    * `[ ]`   `bls12_381_arkworks` depends on the `factory` module's surface, on `domain`, on `zeroize`, and on the arkworks crates, as before; no new edge and no cycle

  * `[ ]`   `requirements`
    * `[ ]`   `cargo check --workspace --all-targets --all-features`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo fmt --all --check`, and `cargo deny check` complete without error or warning
    * `[ ]`   Every existing test in the crate passes unchanged, and every new test named above passes (CR-04, CR-08, CR-09 on BLS12-381 over arkworks)
    * `[ ]`   `Bls12381ArkworksG1`, `Bls12381ArkworksG2`, and `Bls12381ArkworksGt` implement `Zeroize` and `Drop` (CR-07)

* `[ ]`   `pairing/bls12_381_halo2curves` **BLS12-381 halo2curves concrete implements the pairing family's arithmetic trait: scalar-field arithmetic, source-group negation, identity tests, pairing into the target group, and target-group encoding**

  * `[ ]`   `objective`
    * `[ ]`   Problem: the KEM, the envelope, and the delivery proof consume the pairing family's arithmetic trait through whichever concrete the factory resolves, so the BLS12-381 halo2curves concrete implements it with byte-identical target-group encodings to the BLS12-381 arkworks concrete (CR-04; CR-07; CR-08; CR-09)
    * `[ ]`   Functional: `Bls12381Halo2curvesPairing` implements `IPairingArithmetic` with every method's behavior as `pairing/bn254_arkworks` states it for the trait
    * `[ ]`   Functional: the target-group encoding reads the `Fq12` through `Gt::inner`, which the `halo2curves` overlay the `workspace/cargo` node binds supplies, and writes its twelve coefficients in the tower order `c0.c0.c0`, `c0.c0.c1`, `c0.c1.c0`, `c0.c1.c1`, `c0.c2.c0`, `c0.c2.c1`, `c1.c0.c0`, `c1.c0.c1`, `c1.c1.c0`, `c1.c1.c1`, `c1.c2.c0`, `c1.c2.c1`, each 48 bytes big-endian, 576 bytes in all; halo2curves' BLS12-381 tower, `Fq2 = Fq[u] / (u^2 + 1)`, `Fq6 = Fq2[v] / (v^3 - (u + 1))`, and `Fq12 = Fq6[w] / (w^2 - v)`, is arkworks' tower, so equal values encode to equal bytes
    * `[ ]`   Functional: `Bls12381Halo2curvesG1`, `Bls12381Halo2curvesG2`, and `Bls12381Halo2curvesGt` clear their value through `Zeroize` and on drop
    * `[ ]`   Non-functional: `halo2curves` remains named only inside the halo2curves concretes; the family's interface, mock, and factory are unchanged

  * `[ ]`   `role`
    * `[ ]`   Adapter: a further concrete implementing the trait `pairing/bn254_arkworks` authored; the last concrete to implement it, so every concrete beneath the factory implements it when this node completes
    * `[ ]`   Does not edit `factory/interface.rs`, `factory/mock.rs`, or any other concrete
    * `[ ]`   Does not carry a commit

  * `[ ]`   `module`
    * `[ ]`   Bounded context: the `bls12_381_halo2curves` concrete's target-group type, its implementation of `IPairingArithmetic`, its source-group and target-group clearing, and the builder default for its target-group type
    * `[ ]`   Edits the `bls12_381_halo2curves` module's `interface.rs`, `interaction.spec.md`, `mock.rs`, `test.rs`, and `mod.rs`; nothing else in the crate changes

  * `[ ]`   `deps`
    * `[ ]`   The `factory` module's surface, same crate, through `crate::factory::provides`: `IPairingArithmetic` and its params, payloads, success returns, and builders, as `pairing/bn254_arkworks` authors them
    * `[ ]`   `halo2curves` `0.10.0` from the overlay branch `gt-accessor`, the existing runtime dependency, resolved through the `workspace/cargo` node's patch; supplies `Gt::inner(&self) -> &Fq12`
    * `[ ]`   `domain`, the existing runtime dependency: `Secret` and `SecretConstructorParams`; `hex`, the existing dev-dependency
    * `[ ]`   `core::hint::black_box`, standard library, the existing clearing idiom
    * `[ ]`   No new external crate; `adapters/pairing/Cargo.toml` is unchanged

  * `[ ]`   `context_slice`
    * `[ ]`   From `halo2curves::ff`: `Fr`'s `+`, `*`, and unary `-` modulo the group order; `PrimeField::to_repr()` on `Fq`, 48 little-endian bytes
    * `[ ]`   From `halo2curves`: unary `-` on `G1Affine` and `G2Affine`; `PrimeCurveAffine::is_identity()` returning a `Choice`; `PrimeCurveAffine::identity()`; `Bls12381::multi_miller_loop(&[(&G1Affine, &G2Affine)])` and `MillerLoopResult::final_exponentiation()` returning `Gt`; `Gt::identity()`; `Gt::inner()`; `c0()` and `c1()` on the degree-two and degree-twelve extensions and `c0()`, `c1()`, and `c2()` on the degree-six extension; `pairing::Engine::pairing(&G1Affine, &G2Affine)` for the builder default
    * `[ ]`   From `domain`: `Secret::try_new(SecretConstructorParams { value })` returning `Result<Secret<T>, Infallible>`

  * `[ ]`   `adapters/pairing/src/bls12_381_halo2curves/interface.rs`
    * `[ ]`   `Bls12381Halo2curvesGt`, a struct with no derives and one field `pub(super) value: halo2curves::bls12381::Gt`
    * `[ ]`   Every item already in this file is unchanged

  * `[ ]`   `adapters/pairing/src/bls12_381_halo2curves/interaction.spec.md`
    * `[ ]`   Adds a section per new method in the file's existing table form, below the existing sections and above `Ordering and edges`
    * `[ ]`   `add_scalar`, `mul_scalar`: one branch each; dependency call `Fr`'s `+` or `*` over the payload scalars' values; outcome `Ok` holding the sum or product modulo the group order; the payload's scalars are cleared as it drops
    * `[ ]`   `neg_scalar`: one branch; dependency call `Fr`'s unary `-`; outcome `Ok` holding the group order minus the scalar, and zero for zero
    * `[ ]`   `neg_g1`, `neg_g2`: one branch each; dependency call the affine point's unary `-`; outcome `Ok` holding the negation, the identity for the identity
    * `[ ]`   `is_identity_g1`, `is_identity_g2`: one branch each; dependency call `is_identity()` on the payload point; outcome `Ok` holding `bool::from` of the `Choice`
    * `[ ]`   `pairing_product`: one branch; the terms as a `Vec<(&G1Affine, &G2Affine)>` in term order; dependency call `Bls12381::multi_miller_loop`, then `final_exponentiation()`; outcome `Ok(PairingProductSuccessReturn { product })` holding the `Gt` in the owned target-group type; an empty term list yields the target group's identity
    * `[ ]`   `encode_gt`: one branch; dependency call `inner()` on the payload's `Gt`, then each of the twelve `Fq` coefficients' `to_repr()` reversed to 48 big-endian bytes, appended in the tower order into one 576-byte buffer; outcome `Ok(EncodeGtSuccessReturn { bytes })`, the buffer moved into a `Secret` by `let Ok(bytes) = Secret::try_new(SecretConstructorParams { value: buffer });`
    * `[ ]`   `Ordering and edges` gains: halo2curves' affine points and `Gt` implement no `Zeroize`, so `Bls12381Halo2curvesG1`, `Bls12381Halo2curvesG2`, and `Bls12381Halo2curvesGt` clear by setting `value` to `G1Affine::identity()`, `G2Affine::identity()`, or `Gt::identity()` and passing `&self.value` to `black_box`, in their `Zeroize` implementations and their `Drop`

  * `[ ]`   `adapters/pairing/src/bls12_381_halo2curves/mock.rs`
    * `[ ]`   `impl Default for Bls12381Halo2curvesGt` returning `Bls12381::pairing(&G1Affine::generator(), &G2Affine::generator())`
    * `[ ]`   The imports gain `halo2curves::bls12381::Bls12381` and `halo2curves::pairing::Engine`; every existing default is unchanged

  * `[ ]`   `adapters/pairing/src/bls12_381_halo2curves/test.rs`
    * `[ ]`   Every existing test and constant is unchanged; the new tests are appended; the imports gain `IPairingArithmetic` and the new params, overrides, and builders from `crate::factory::provides`
    * `[ ]`   The new constants are those `pairing/bls12_381_arkworks` adds: zero, one, and six as 32-byte scalars, and the negated first-group generator in EIP-2537's 128 bytes, 16 zero bytes and `17f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb`, then 16 zero bytes and `114d1d6855d545a8aa7d76c8cf2e21f267816aef1db507c96655b9d5caac42364e6f38ba0ecb751bad54dcd6b939c2ca`; the target group's identity encoding is `vec![0u8; 576]` with index `47` set to `1`
    * `[ ]`   The new tests carry the names, arrangements, acts, and assertions `pairing/bls12_381_arkworks` states, over the same vectors, the first-group identity from 128 zero bytes, and the second-group identity from 256 zero bytes, with the subject constructed by `let Ok(pairing) = Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);`: `add_scalar_of_two_and_three_is_five`, `add_scalar_reduces_modulo_the_group_order`, `mul_scalar_of_two_and_three_is_six`, `mul_scalar_reduces_modulo_the_group_order`, `neg_scalar_of_one_is_the_group_order_minus_one`, `neg_scalar_of_zero_is_zero`, `neg_g1_of_the_generator_negates_its_y_coordinate`, `neg_g1_of_the_identity_is_the_identity`, `neg_g2_of_the_generator_sums_with_the_generator_to_the_identity`, `is_identity_g1_is_true_for_the_identity`, `is_identity_g1_is_false_for_the_generator`, `is_identity_g2_is_true_for_the_identity`, `is_identity_g2_is_false_for_the_generator`, `pairing_product_of_no_terms_encodes_as_the_target_group_identity`, `pairing_product_of_the_generators_is_not_the_target_group_identity` asserting 576 bytes, `pairing_product_is_bilinear`, `pairing_product_multiplies_its_terms`, and `pairing_product_of_a_pairing_and_its_first_group_negation_is_the_target_group_identity`
    * `[ ]`   Every new test block carries the full `Contract`, `Arrange`, `Act`, `Assert` header and the inline markers

  * `[ ]`   `construction`
    * `[ ]`   A target-group value is produced only by `pairing_product`; no consumer constructs one from library values

  * `[ ]`   `adapters/pairing/src/bls12_381_halo2curves/mod.rs`
    * `[ ]`   `impl IPairingArithmetic for Bls12381Halo2curvesPairing` with `type Gt = Bls12381Halo2curvesGt;` and every method realizing its branch in the interaction spec
    * `[ ]`   `impl Zeroize` and `impl Drop` for `Bls12381Halo2curvesG1`, `Bls12381Halo2curvesG2`, and `Bls12381Halo2curvesGt`, each setting `value` to its type's identity and calling `black_box(&self.value)`
    * `[ ]`   The imports gain `IPairingArithmetic` and the new types from `crate::factory::provides`, `halo2curves::bls12381::Gt`, and `Bls12381Halo2curvesGt` from `interface`
    * `[ ]`   Every existing item is unchanged; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[ ]`   `directionality`
    * `[ ]`   `bls12_381_halo2curves` depends on the `factory` module's surface, on `domain`, on `zeroize`, and on `halo2curves`, as before; no new edge and no cycle
    * `[ ]`   Every concrete beneath the pairing factory now implements `IPairingArithmetic`; `pairing/factory` requires it of the concrete a consumer receives

  * `[ ]`   `requirements`
    * `[ ]`   `cargo check --workspace --all-targets --all-features`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo fmt --all --check`, and `cargo deny check` complete without error or warning
    * `[ ]`   Every existing test in the crate passes unchanged, and every new test named above passes (CR-04, CR-08, CR-09 on BLS12-381 over halo2curves)
    * `[ ]`   `Bls12381Halo2curvesG1`, `Bls12381Halo2curvesG2`, and `Bls12381Halo2curvesGt` implement `Zeroize` and `Drop` (CR-07)

* `[ ]`   `pairing/factory` **The pairing factory hands every consumer a concrete that implements the arithmetic trait; carries the integration test proving both libraries on each curve encode the target group identically**

  * `[ ]`   `objective`
    * `[ ]`   Problem: the KEM, the envelope, and the delivery proof reach a pairing concrete only through `create_pairing` and `IPairingConsumer`, whose consumer is bounded by `IPairingAdapter` alone, so the arithmetic trait every concrete now implements is unreachable through the factory; and a capsule encapsulated under one library must decapsulate to the same wrapping key under the other, so both libraries on a curve must encode a target-group value to the same bytes (CR-08; CR-10; CR-11)
    * `[ ]`   Functional: `IPairingConsumer::consume_pairing` is bounded by `IPairingArithmetic`, so a consumer calls the arithmetic trait and, through its supertrait, the generic interface on whichever concrete `create_pairing` constructs
    * `[ ]`   Functional: a consumer whose `consume_pairing` is written against `IPairingAdapter` alone remains a valid implementation, so every existing consumer compiles unchanged
    * `[ ]`   Functional: the arkworks and halo2curves concretes on BN254 encode the pairing of the same inputs to the same bytes, and likewise on BLS12-381
    * `[ ]`   Non-functional: `create_pairing`, its params, payload, and return types, the admission rule, and the declaration are unchanged

  * `[ ]`   `role`
    * `[ ]`   Adapter family factory: binds the arithmetic trait into the consumer surface once every concrete implements it, and carries the family's integration test for the arithmetic trait
    * `[ ]`   Does not change `create_pairing`'s body, the concretes, or `factory/mock.rs`; `MockIPairingConsumer` keeps its `IPairingAdapter`-bounded method, a valid implementation of the tightened trait
    * `[ ]`   Does not carry a commit; the credential KEM milestone's commit sits on the last node of that milestone's chain

  * `[ ]`   `module`
    * `[ ]`   Bounded context: `IPairingConsumer` in the `factory` module's interface, and the crate's integration test under `adapters/pairing/tests`
    * `[ ]`   Edits `adapters/pairing/src/factory/interface.rs` and `adapters/pairing/tests/integration_test.rs`; nothing else changes

  * `[ ]`   `deps`
    * `[ ]`   The `factory` module's own interface: `IPairingArithmetic` as `pairing/bn254_arkworks` authors it, and `IPairingConsumer`
    * `[ ]`   Every concrete's implementation of `IPairingArithmetic`, as `pairing/bn254_arkworks`, `pairing/bn254_halo2curves`, `pairing/bls12_381_arkworks`, and `pairing/bls12_381_halo2curves` state them; `create_pairing` constructs each and so requires each to satisfy the tightened bound
    * `[ ]`   No new external crate; `adapters/pairing/Cargo.toml` is unchanged

  * `[ ]`   `context_slice`
    * `[ ]`   From `IPairingArithmetic`, in the integration test: `pairing_product(&self, PairingProductParams, PairingProductPayload { terms }) -> Result<PairingProductSuccessReturn<Self::Gt>, Infallible>` and `encode_gt(&self, EncodeGtParams, EncodeGtPayload { value }) -> Result<EncodeGtSuccessReturn, Infallible>` with `bytes: Secret<Vec<u8>>`
    * `[ ]`   From `IPairingAdapter`, in the integration test: `g1_generator`, `g2_generator`, `decode_scalar`, `mul_g1` with `MulG1Payload { point, scalar }`, and `mul_g2` with `MulG2Payload { point, scalar }`

  * `[ ]`   `adapters/pairing/src/factory/interface.rs`
    * `[ ]`   `IPairingConsumer::consume_pairing` reads `fn consume_pairing<P: IPairingArithmetic>(&self, params: ConsumePairingParams, payload: ConsumePairingPayload<P>) -> Self::Output;`
    * `[ ]`   Every other item in this file is unchanged

  * `[ ]`   `adapters/pairing/tests/integration_test.rs`
    * `[ ]`   Every existing item and test is unchanged; the new items are appended; the imports gain `IPairingArithmetic`, `MulG2Params`, `MulG2Payload`, `PairingProductParams`, `PairingProductPayload`, `EncodeGtParams`, and `EncodeGtPayload` from `pairing`
    * `[ ]`   A test-local `TargetGroupEncoding`, the unit struct implementing `IPairingConsumer` with `type Output = Vec<u8>;` and a `consume_pairing<P: IPairingArithmetic>` that, through `payload.adapter` alone, decodes the scalars two and three from their 32 big-endian bytes, multiplies the first-group generator by two and the second-group generator by three, computes `pairing_product` over the one term of those products, encodes it through `encode_gt`, each call unpacked by `let Ok(…) = … else { panic!(…) };`, and returns the exposed bytes cloned into a `Vec<u8>`
    * `[ ]`   `the_bn254_concretes_encode_the_same_pairing_to_the_same_bytes`: contract: the arkworks and halo2curves BN254 concretes, each constructed by the factory and used only through the family traits, encode `e(g1 · 2, g2 · 3)` to identical bytes; arrange `CreatePairingDeps { consumer: TargetGroupEncoding }` and `build_create_pairing_params` with `concrete` overridden by `PairingConcrete::Bn254Arkworks` and then by `PairingConcrete::Bn254Halo2curves`; act `create_pairing` for each, unpacked by `let Ok(success) = … else { panic!(…) };`; assert the two outputs are equal, are 384 bytes, and differ from the target group's identity encoding, `vec![0u8; 384]` with index `31` set to `1`
    * `[ ]`   `the_bls12_381_concretes_encode_the_same_pairing_to_the_same_bytes`: the same with `PairingConcrete::Bls12381Arkworks` and `PairingConcrete::Bls12381Halo2curves`, asserting 576 bytes and difference from `vec![0u8; 576]` with index `47` set to `1`
    * `[ ]`   Every new test block carries the full `Contract`, `Arrange`, `Act`, `Assert` header, the inline markers, and the `Boundary` and `Mocked` lines the existing tests carry; nothing is mocked, since the curve libraries are the outer edge

  * `[ ]`   `directionality`
    * `[ ]`   `IPairingConsumer` now names `IPairingArithmetic` within the `factory` module; the factory's recorded cycle with its concretes is unchanged; no new edge between crates
    * `[ ]`   `kem/bb1_depth_one` consumes the family through `create_pairing` and an `IPairingConsumer` bounded by `IPairingArithmetic`

  * `[ ]`   `requirements`
    * `[ ]`   `cargo check --workspace --all-targets --all-features`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo fmt --all --check`, and `cargo deny check` complete without error or warning, with `apps/harness-crypto`'s benchmark, the factory's unit tests, and every other existing consumer compiling unchanged
    * `[ ]`   Every existing test in `adapters/pairing` and `apps/harness-crypto` passes unchanged
    * `[ ]`   `the_bn254_concretes_encode_the_same_pairing_to_the_same_bytes` and `the_bls12_381_concretes_encode_the_same_pairing_to_the_same_bytes` pass (CR-08 cross-holder agreement across libraries; CR-11 the encapsulated value's serialization fixed across the family)

# To-Do List

Every held item, grouped by **what releases it** rather than by topic.

**Open questions** carry what was found, where it lives, and what resolving it would take. **Boundary decisions** specified in `docs/research/MVP Scope.md` or `docs/research/cryptography.md` appear as a single line under their gate, pointing at where they are specified.

## Releases with the stake and token layer

These bottom out in the same prerequisite: identity that carries stake, over a participant set that cannot be captured cheaply. They are resolved together or not at all.

### Seeder compensation mechanism

**What was found.** The swarm distribution the protocol depends on presumes that seeding is rewarded, and no mechanism is specified. Without one, contribution is voluntary, free-riding is rational, and aggregate resilience decays toward whatever altruism sustains.

**This is one problem with retention assignment, not two adjacent ones.** Both bottom out in Sybil-resistant stake over the same participant set: a design answering who may be rewarded for storing is most of a design for who may be assigned to store, and the reverse holds as well. A proposal addressing one and not the other is incomplete rather than partial, and the two are resolved together or not at all.

The candidate mechanism is a reciprocal token pair in which one participant's upload token is another participant's download token, with a participant's ratio serving as the translation between them. This is out of MVP bounds because it depends on the token economics deferred alongside monetization, but it is a protocol-level gap rather than an implementation detail: the incentive structure determines whether the distribution model works at population scale at all.

**Where.** [`docs/research/cryptography.md`, Provisioning Liveness](../../research/cryptography.md#provisioning-liveness-centralization-and-participation) and [Seeder Compensation](../../research/cryptography.md#seeder-compensation); [`docs/research/MVP Scope.md`, Seeder Compensation](../../research/MVP%20Scope.md#seeder-compensation).

**What resolving it would take.** Specifying the token pair and the ratio translation, then deciding whether the mechanism is enforced or emergent. The specification assigns retention by obligation and audits it, so compensation is what makes the discretionary tier worth offering, and the obligated tier does not depend on it. Which tier a token accounting system drives is a decision, not a side effect.

### Retention parameters

**What was found.** Retention of swarm ciphertext and header sidecars is the participation obligation attached to every identity, assigned by random rotation over the participant set and audited on one surface. Variant seeds are carried by the entitlement and are neither stored nor served, so retention covers ciphertext and sidecars only. Availability is a constructed replication factor, which makes a number necessary.

The coupled values below are undetermined and none can be chosen independently of the others. The **rotation period** trades capture resistance against reassignment churn, since an assignment that rotates faster is harder to game and more expensive to keep synchronized. The **replication factor** sets how many identities each object is assigned to, and is what converts availability from an emergent property into a constructed one. The **retention floor** is the ciphertext storage each identity carries to remain in good standing. The **audit challenge frequency** determines how quickly a non-compliant holder is detected, and costs bandwidth on every participant to raise.

**Where.** [`docs/research/cryptography.md`, Provisioning Liveness](../../research/cryptography.md#provisioning-liveness-centralization-and-participation), the participation obligation; [`docs/research/MVP Scope.md`, Retention Obligation](../../research/MVP%20Scope.md#retention-obligation) and [Seed Hosting and the Ciphertext Store](../../research/MVP%20Scope.md#seed-hosting-and-the-ciphertext-store).

**What resolving it would take.** Choosing them jointly against a stated availability target and a stated storage cost per participant. None is enforceable without the identity and stake layer, so the MVP can select values and does not exercise them.

### Boundary decisions under this gate

* **Paid monetization** — general pricing above nominal, and the fiat and token rails it requires. Deferred in [`docs/research/MVP Scope.md`, Paid Monetization](../../research/MVP%20Scope.md#paid-monetization). Note that [Transaction Flow Proof](../../research/MVP%20Scope.md#transaction-flow-proof) does not release this: that path is a test instrument on assets the project publishes itself, touching no third-party publisher's revenue.
* **Enforcement of retention assignment and obligation** — the shape ships in the MVP and nothing is enforced. Specified in [`docs/research/MVP Scope.md`, Retention Obligation](../../research/MVP%20Scope.md#retention-obligation).
* **The first-party component class** — own chain, tokens, wallet, keystore, and seeder client, each behind the adapter whose MVP implementation is third-party. Specified in [`docs/research/MVP Scope.md`, Adapter Composition and Capability Declaration](../../research/MVP%20Scope.md#adapter-composition-and-capability-declaration).

## Releases when a content class needs it

Nothing here is blocked by an unanswered question. Each is blocked by the absence of content whose value or shape justifies building it, and the MVP's content class supplies neither.

### Collusion parameter selection

**What was found.** Variant seeds are assigned using a collusion-resistant fingerprinting code providing provable tracing up to a chosen collusion size *c*. Codeword length grows with *c* and with the accused-population size, so *c* is an economic parameter with no principled default, and its cost curve is unmeasured.

Because the seed is a value the entitlement carries rather than an enumeration of replacement content, the cost of *c* is carried in codeword length rather than in any stored or distributed object, and is independent of asset size.

The specification states the related properties: the MVP ships with no attribution; attribution is evidence rather than enforcement, since identifying a leaker does not recall content, making the response to a traced leak a governance and legal question; and the fingerprint is cooperative rather than forensic, since the swarm object is complete and a modified client can render canonical plaintext carrying no fingerprint at all.

**Where.** [`docs/research/cryptography.md`, Collusion Resistance](../../research/cryptography.md#collusion-resistance).

**What resolving it would take.** Modelling *c* against asset value and expected adversary resources, and measuring the codeword-length cost curve rather than assuming it. Neither is needed before per-entitlement variance ships.

### Boundary decisions under this gate

* **Per-entitlement variance and forensic attribution** — meaningless while content is permissively licensed. Deferred in [`docs/research/MVP Scope.md`, Per-Entitlement Variance and Forensic Attribution](../../research/MVP%20Scope.md#per-entitlement-variance-and-forensic-attribution).
* **Arbitrary media and streaming engine** — a different client pipeline, and the content class that exercises per-attempt authorization continuously. Deferred in [`docs/research/MVP Scope.md`, Arbitrary Media and Streaming Engine](../../research/MVP%20Scope.md#arbitrary-media-and-streaming-engine).
* **Content flagging and the metadata backlink layer** — the construction is the cost, not any one advisory type. Deferred in [`docs/research/MVP Scope.md`, Content Flagging and Deprecation Surface](../../research/MVP%20Scope.md#content-flagging-and-deprecation-surface).
* **Partial encryption** — held for a commercial reason rather than a technical one. Specified in [`docs/research/cryptography.md`, Partial Encryption](../../research/cryptography.md#future-optimization-partial-encryption).
* **Composable container objects** — reference nesting with per-member roots in the manner of BitTorrent v2, so that seeding a collection seeds its members. Deferred in [`docs/research/MVP Scope.md`, Composable Container Objects](../../research/MVP%20Scope.md#composable-container-objects).

## Releases when a design question is answered

Open questions. Each blocks work that cannot be authored around it.

### Variant seed authorship

**What was found.** Variance is an overlay seed bound to the entitlement, but who authors the seed, from what material, and under what constraints is undetermined.

Constraints shape any answer. First Finder escrow means the publisher is absent by definition, so no scheme requiring the publisher online at issuance or transfer can work at all; and a First Finder that authors seeds must escrow and discard the authoring material exactly as it does the content key, or a non-owner retains permanent framing capability over an asset they do not own. Separately, a seed can select among alternatives but cannot author them — if both the varied positions and their replacement values derive from the seed alone, the rendering is a corruption rather than a variant: a glitched frame, or a tarball that no longer installs. Semantically valid renderings require alternatives authored with knowledge of the content.

Whoever holds the authoring material can generate any holder's seed and therefore frame any holder. Siting that material with the credential author — the issuer at mint and the seller at sale, who already post the envelope the seed would ride in — is the cheapest available answer, but it would let a seller frame its buyer, which the framing rule forbids. The seed rides the credential envelope.

**Where.** [`docs/research/cryptography.md`, Variance Belongs to the Entitlement](../../research/cryptography.md#variance-belongs-to-the-entitlement-not-the-content), [The Variant Seed Lives in the Entitlement](../../research/cryptography.md#the-variant-seed-lives-in-the-entitlement), and [Variant Seed Authorship](../../research/cryptography.md#variant-seed-authorship); [`docs/research/MVP Scope.md`, Per-Entitlement Variance and Forensic Attribution](../../research/MVP%20Scope.md#per-entitlement-variance-and-forensic-attribution).

**What resolving it would take.** Choosing an authorship mechanism from forensic watermarking and specifying it behind the variance interface, where the choice disturbs no layer around it.

### The post-claim pricing paradox

**What was found.** An escrow contract mints free entitlements to grow the swarm before an owner arrives. When the owner claims the asset they acquire control over future issuance but cannot retroactively charge or deny existing holders, whose entitlements are permanently grandfathered, and secondary sellers of those free entitlements can undercut any price the publisher sets.

The paradox arises only where ingested content is not free to use, which the ingest eligibility rule excludes, so the escrow model is exercised in production without encountering it.

**Where.** [`docs/research/cryptography.md`, The Post-Claim Pricing Paradox](../../research/cryptography.md#the-post-claim-pricing-paradox) and [Ingest Source Eligibility](../../research/cryptography.md#ingest-source-eligibility).

**What resolving it would take.** A mechanism that converts early unclaimed distribution into fair economic value for the claimant without enabling retroactive pricing. Both failure modes must be excluded: a publisher who profits by waiting for an asset to become popular before claiming it, and a First Finder and early users who free-ride on an asset they never owned.

### Consumption privacy for private content

**What was found.** For public content, who holds an entitlement being publicly legible is a stated property rather than a defect. That property does not cover these cases: the record is a live feed rather than a snapshot its subject can pace, individuals are not the free-riding party the property argues about, and private content was never public in either its asset list or its consumption.

**Where.** [`docs/research/cryptography.md`, Dependency Graph Privacy](../../research/cryptography.md#dependency-graph-privacy) and Public Distribution Implies Public Consumption in [Core Philosophy](../../research/cryptography.md#core-philosophy).

**What resolving it would take.** A consumption-privacy construction for the private case — per-asset ephemeral wallets, blinded or private-information-retrieval authorization, batching and mixing, off-chain proofs settling in aggregate, or zero-knowledge proof of entitlement. Any resolution has to state which side of the auditability-versus-privacy tension it sacrifices. This gates any ingest adapter pointed at a private or internal registry.

### Scope of the authorization invariants

**What was found.** The Cryptographic Authorization Invariants are written as unconditional statements, but they bind the settlement contract, credential authors, and conforming clients only. They cannot bind arbitrary software on a user-controlled device, and the protocol deliberately commits to plaintext running in any compatible software, which guarantees such software exists.

That scope is stated under Modified Clients as a security consideration rather than among the invariants where they are declared, so a reader taking the invariant list on its own would overread its guarantees.

**Where.** [`docs/research/cryptography.md`, Cryptographic Authorization Invariants](../../research/cryptography.md#cryptographic-authorization-invariants); the scope statement under [Modified Clients](../../research/cryptography.md#modified-clients-the-honesty-assumption).

**What resolving it would take.** Deciding whether the invariant list carries its own scope sentence or whether the Modified Clients statement suffices. This is a question of where the statement belongs, not whether it is true.

### Adapter registry governance

**What was found.** Adapters are resolved from a governance-controlled on-chain registry, immutable once bound: the identity adapters, the signature adapters, the attestation verifier per ingest source, and the entitlement token form; the source-key table the npm attestation verifier reads is updated under the same governance. The MVP governs both with a single project-held key, held under the same custody discipline as issuance material and recorded as scaffolding, so the registry's binding authority and the attestation check are only as strong as that key's custody.

**Where.** [`docs/research/cryptography.md`, Population Strategy](../../research/cryptography.md#population-strategy-breakdown) and [Ingest and Deterministic Normalization](../../research/cryptography.md#phase-1-encryption-minting-and-seeding); [`docs/research/MVP Scope.md`, Entitlement Ledger and Escrow Contract](../../research/MVP%20Scope.md#entitlement-ledger-and-escrow-contract); the product requirements' Resolved Positions.

**What resolving it would take.** A governance model that moves the key off a single holder, a multisig or a stake-conditioned process, with a migration of the registry's and source-key table's authority that no single party can withhold, and a recorded trigger for when the MVP's scaffolding is replaced.

### Entitlement migration across chain adapters

**What was found.** The chain sits behind an adapter and the protocol intends its own chain eventually, but an adapter swap moves nothing: entitlements, interval state, envelope digests, parameter-set liveness, deployment records, and identity bindings live in one chain's contract state, and a further chain, whether Ethereum mainnet, another L2, or the project's own, starts empty. Every invariant the protocol makes about an entitlement, asset binding, irrevocability, transferability, and the delivery chain that authenticates its credential history, is a statement about one ledger. How an entitlement and its interval history become valid on another ledger, without a party acquiring the discretion over the move that the No Party May Selectively Withhold invariant forbids, is unspecified.

**Where.** [`docs/research/MVP Scope.md`, Adapter Composition and Capability Declaration](../../research/MVP%20Scope.md#adapter-composition-and-capability-declaration) and [Entitlement Ledger and Escrow Contract](../../research/MVP%20Scope.md#entitlement-ledger-and-escrow-contract); [`docs/research/cryptography.md`, Architectural Invariants](../../research/cryptography.md#overview-and-invariant-requirements) and [Credential Delivery](../../research/cryptography.md#cryptographic-primitives); the launch network decision recorded in [`docs/project-planning/product-requirements.md`](../../project-planning/product-requirements.md).

**What resolving it would take.** A migration mechanism with the same non-deniability the transfer path has: a snapshot, claim, or bridge construction under which a holder proves its entitlement and interval on the source ledger and obtains the equivalent record on the target ledger with no party able to withhold it, the source record retired or frozen so scarcity is not doubled, the envelope history carried or re-anchored so the next sale's proof has a base case, and parameter-set liveness reproduced so existing credentials keep decrypting. The MVP lives on one chain and needs none of it; any further chain adapter needs it before carrying live entitlements, and it decides whether the first-party chain is a migration or a fresh start.

## Releases on a deliberate policy line

Not blocked by engineering. Blocked by a line the project draws on purpose, so that crossing it is a decision rather than a drift.

### Ingest adapter eligibility

**What was found.** The adapter interface is source-independent, but the *selection* of sources is not a free choice. Exposure concentrates in the acquisition path rather than in distribution, escrow is deferred ownership identification rather than an acquisition licence, and a free archive is what makes the post-claim pricing paradox inert. Adapters therefore target archives whose content is free to use.

Crossings are held behind separate answers: an adapter aimed at licensed content waits on the post-claim pricing paradox, and an adapter aimed at private or internal content waits on consumption privacy.

**Where.** [`docs/research/cryptography.md`, Ingest Source Eligibility](../../research/cryptography.md#ingest-source-eligibility).

**What resolving it would take.** Answering the gating item for the crossing in question, then making the adapter selection a recorded policy decision rather than an engineering convenience.

### Distribution and client license

**What was found.** The software must permit forking and modification while making a non-conforming client a license violation, and each asset record must carry the rights-holder's content terms, with the project's own packages obligating an entitlement for use and a license per copy sold or bundled. A conformance clause makes the software license source-available rather than OSI-open, and it binds forks of the code, not a clean-room client written from the specification.

**Where.** [`docs/research/MVP Scope.md`, Distribution and Client License](../../research/MVP%20Scope.md#distribution-and-client-license); [`docs/research/MVP Application Requirements.md`, LI-01](../../research/MVP%20Application%20Requirements.md#licensing); [`docs/research/cryptography.md`, Core Philosophy](../../research/cryptography.md#core-philosophy).

**What resolving it would take.** Legal drafting of the license text; a decision on the OSI question against Core Philosophy, including whether the protocol libraries and the reference client carry different licenses; and the record field for content terms.

### Intentional identity fragmentation

**What was found.** The retention obligation is scoped per identity so that Sybil resistance is inherent rather than an overlay. One operator deliberately splitting into many identities to dilute that obligation is a distinct problem, and the specification explicitly declines to answer it.

**Where.** [`docs/research/cryptography.md`, Provisioning Liveness](../../research/cryptography.md#provisioning-liveness-centralization-and-participation).

**What resolving it would take.** A cost on identity creation that fragmentation cannot amortize, which is the same stake layer the compensation and enforcement items wait on — but the question is separable, because stake makes fragmentation expensive without making it incoherent.

## Releases on sequence alone

Nothing here is unresolved; each waits on the build sequence.

### Release workflow for a package that does not exist

**What was found.** `.github/workflows/npm-publish.yml` runs `npm ci`, `npm test`, and `npm publish` with the `npm_token` secret on every GitHub release, on Node.js 16, from the repository root, which is not an npm package; the project's npm package is `shells/npm-bootstrap`.

**Where.** `.github/workflows/npm-publish.yml`.

**What resolving it would take.** Removing the workflow, or replacing it at the `shells/npm-bootstrap` milestone with a publish job for that package's directory, built and signed by the release process.

### Boundary decisions under this gate

* **Git commit wrapping** — mutable DAGs and merge complexity against static immutable tarballs; packages first. Deferred in [`docs/research/MVP Scope.md`, Git Commit Wrapping](../../research/MVP%20Scope.md#git-commit-wrapping).
* **Active email bot for First Finder escrow** — excluded for spam and domain reputation, and made costless by the claim-set mechanism. Deferred in [`docs/research/MVP Scope.md`, Active Email Bot for First Finder Escrow](../../research/MVP%20Scope.md#active-email-bot-for-first-finder-escrow).
* **Version alignment engine** — resolution belongs to the package manager. Deferred in [`docs/research/MVP Scope.md`, Version Alignment Engine](../../research/MVP%20Scope.md#version-alignment-engine).