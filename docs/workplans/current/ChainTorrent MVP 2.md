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

* `[ ]`   `encoding/factory` **Encoding factory constructing the concrete that implements the encoding identifier a hash-card or configuration names, and handing it to a consumer generic over the family's encoder and decoder traits; carries the family's integration test**

  * `[ ]`   `objective`
    * `[ ]`   Problem: a consumer obtains an encoder and decoder only through the family's generic surface, never by naming a concrete, and the encoding it uses is the one the deployment's hash-card or the configuration names, so every hashed, signed, stored, or framed value is encoded one way for one identifier (CR-11; Composition Boundary)
    * `[ ]`   Functional: given an encoding identifier, the factory constructs the concrete that implements it and hands it, with its declaration, to a consumer generic over `IEncoderAdapter` and `IDecoderAdapter`, and returns the consumer's output; the consumer never names the concrete
    * `[ ]`   Functional: a concrete's constructor error is returned unchanged in the factory's error arm, one variant per concrete
    * `[ ]`   Functional: the concrete the factory constructs encodes the reference derivation context to the known-answer vector and decodes it back through the family's traits and the description
    * `[ ]`   Non-functional: adding a concrete for a new identifier is its module, its identifier variant, its variant in the error enum, and its branch here; no consumer changes

  * `[ ]`   `role`
    * `[ ]`   Adapter family factory: the implementation of the `factory` module, the encoding family's construction point, and the crate's public surface beside the family-owned descriptions
    * `[ ]`   Hands the concrete to a consumer rather than returning it, because `IEncoderAdapter` and `IDecoderAdapter` carry generic methods and so cannot be returned as one type across concretes; the consumer is written once, generic over both traits, and the factory instantiates it for the concrete it constructs
    * `[ ]`   Selects by the encoding identifier alone, since each identifier has one concrete; a further concrete implementing an existing identifier adds a concrete selection to the params at the node that authors it
    * `[ ]`   Does not read a hash-card or the configuration; the composition resolver passes the identifier either names as a typed `EncodingIdentifier`
    * `[ ]`   Does not encode, decode, or describe; the concrete and the descriptions do
    * `[ ]`   Carries the family's integration test across factory, concrete, and description; does not carry a commit

  * `[ ]`   `module`
    * `[ ]`   Bounded context: the `factory` module of `adapters/encoding`, holding the factory function, its deps, params, payload, and return types, its signature type, the consumer trait with its params and payload types, the function mock and builders, and the crate's integration test under `adapters/encoding/tests`
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
    * `[ ]`   `IEncodingConsumer`, a trait with `type Output;` and `fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(&self, params: ConsumeEncodingParams, payload: ConsumeEncodingPayload<E>) -> Self::Output;`, the work a composition performs with whichever concrete the factory constructs
    * `[ ]`   `ConsumeEncodingParams`, the fieldless struct `pub struct ConsumeEncodingParams;`
    * `[ ]`   `ConsumeEncodingPayload<E>`, a struct with `pub adapter: E` and `pub declaration: EncodingDeclaration`
    * `[ ]`   `CreateEncodingDeps<C>`, a struct with `pub consumer: C`, the collaborator the factory hands the concrete to
    * `[ ]`   `CreateEncodingParams`, a struct with `pub identifier: EncodingIdentifier`, the selection of the encoding to construct
    * `[ ]`   `CreateEncodingPayload`, the fieldless struct `pub struct CreateEncodingPayload;`, since the factory operates on no data
    * `[ ]`   `CreateEncodingSuccessReturn<O>`, a struct with `pub output: O`
    * `[ ]`   `CreateEncodingErrorReturn`, an enum with the one variant `Abi(Infallible)`, the ABI concrete's constructor error carried unchanged; each further concrete's constructor error is its own variant
    * `[ ]`   `CreateEncodingReturn<O>`, the alias `Result<CreateEncodingSuccessReturn<O>, CreateEncodingErrorReturn>`
    * `[ ]`   `CreateEncodingFn<C>`, the alias `fn(&CreateEncodingDeps<C>, CreateEncodingParams, CreateEncodingPayload) -> CreateEncodingReturn<<C as IEncodingConsumer>::Output>`
    * `[ ]`   Every item `encoding/derivation_context` and `encoding/abi` authored in this file is unchanged

  * `[ ]`   `adapters/encoding/src/factory/interaction.spec.md`
    * `[ ]`   `create_encoding<C: IEncodingConsumer>(deps: &CreateEncodingDeps<C>, params: CreateEncodingParams, payload: CreateEncodingPayload) -> CreateEncodingReturn<C::Output>`: decision a `match` on `params.identifier`, one arm per `EncodingIdentifier` variant, exhaustive so an identifier with no arm fails to compile
    * `[ ]`   Ethereum ABI: condition `params.identifier` is `EncodingIdentifier::EthereumAbiV1`; decision the `match`; dependency calls `AbiEncoding::try_new(AbiEncodingConstructorParams)`, exactly once, its success destructured irrefutably because its error arm is uninhabited, then `deps.consumer.consume_encoding(ConsumeEncodingParams, ConsumeEncodingPayload { adapter, declaration: AbiEncoding::DECLARATION })`, exactly once; outcome `Ok(CreateEncodingSuccessReturn { output })` holding the consumer's output
    * `[ ]`   `CreateEncodingErrorReturn::Abi` carries the constructor's uninhabited error type in the return union, so no branch produces it
    * `[ ]`   `params.identifier` selects; `payload` carries nothing and is not read

  * `[ ]`   `adapters/encoding/src/factory/mock.rs`
    * `[ ]`   `CreateEncodingParamsOverrides`, `#[derive(Default)]`, one field `pub identifier: Option<EncodingIdentifier>`; `build_create_encoding_params(overrides: CreateEncodingParamsOverrides) -> CreateEncodingParams`, the identifier defaulting to `EncodingIdentifier::EthereumAbiV1`
    * `[ ]`   `CreateEncodingSuccessReturnOverrides<O>`, `#[derive(Default)]`, one field `pub output: Option<O>`; `build_create_encoding_success_return<O: Default>(overrides: CreateEncodingSuccessReturnOverrides<O>) -> CreateEncodingSuccessReturn<O>`, the output defaulting to `O::default()`
    * `[ ]`   `ConsumeEncodingPayloadOverrides<E>`, `#[derive(Default)]`, fields `pub adapter: Option<E>` and `pub declaration: Option<EncodingDeclaration>`; `build_consume_encoding_payload<E: Default>(overrides: ConsumeEncodingPayloadOverrides<E>) -> ConsumeEncodingPayload<E>`, the adapter defaulting to `E::default()` and the declaration to `build_encoding_declaration(Default::default())`
    * `[ ]`   `MockIEncodingConsumer`, the unit struct `pub struct MockIEncodingConsumer;`, implementing `IEncodingConsumer` with `type Output = ();` and `consume_encoding` returning `()` for any adapter; a test needing other behavior implements the trait on its own local struct
    * `[ ]`   `mock_create_encoding<C: IEncodingConsumer>(_deps: &CreateEncodingDeps<C>, _params: CreateEncodingParams, _payload: CreateEncodingPayload) -> CreateEncodingReturn<C::Output>` for `C::Output: Default`, returning `Ok(build_create_encoding_success_return(Default::default()))`
    * `[ ]`   No builder for the fieldless `ConsumeEncodingParams` and `CreateEncodingPayload`, used by their production values, for `CreateEncodingDeps`, whose one field is the consumer the test supplies, or for the enum `CreateEncodingErrorReturn`; every symbol `encoding/derivation_context` and `encoding/abi` authored in this file is unchanged

  * `[ ]`   `adapters/encoding/src/factory/test.rs`
    * `[ ]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`; imports `create_encoding` from `super`, `CreateEncodingDeps`, `CreateEncodingPayload`, `ConsumeEncodingParams`, `ConsumeEncodingPayload`, `IEncodingConsumer`, `IEncoderAdapter`, `IDecoderAdapter`, `EncodingDeclaration`, `EncodingIdentifier`, and `ENCODING_INTERFACE_VERSION` from `super::interface`, and `build_create_encoding_params` and `CreateEncodingParamsOverrides` from `super::mock`
    * `[ ]`   A test-local `DeclarationProbe`, the unit struct implementing `IEncodingConsumer` with `type Output = EncodingDeclaration;` and `consume_encoding` returning `payload.declaration`
    * `[ ]`   `create_encoding_hands_the_consumer_the_abi_concrete_and_its_declaration`: contract: the Ethereum ABI identifier reaches the consumer with the ABI concrete's declaration; arrange `build_create_encoding_params` with `identifier: Some(EncodingIdentifier::EthereumAbiV1)` and `CreateEncodingDeps { consumer: DeclarationProbe }`; act `create_encoding(&deps, params, CreateEncodingPayload)`, unpacked by `let Ok(success) = … else { panic!(…) };`; assert `success.output.identifier` matches `EncodingIdentifier::EthereumAbiV1`, `success.output.adapter_version` equals `1`, and `success.output.interface_version` equals `ENCODING_INTERFACE_VERSION`
    * `[ ]`   The test block carries the full `Contract`, `Arrange`, `Act`, `Assert` header and the inline markers

  * `[ ]`   `construction`
    * `[ ]`   The composition root writes its encoding-dependent work once as an `IEncodingConsumer`, generic over `E: IEncoderAdapter + IDecoderAdapter`, and calls `create_encoding` with `CreateEncodingDeps { consumer }`, `CreateEncodingParams` holding the identifier the hash-card or configuration names, and `CreateEncodingPayload`; no consumer constructs or names a concrete

  * `[ ]`   `adapters/encoding/src/factory/mod.rs`
    * `[ ]`   Adds `#[cfg(test)] mod test;` to the wiring `encoding/derivation_context` authored
    * `[ ]`   `pub fn create_encoding<C: IEncodingConsumer>(deps: &CreateEncodingDeps<C>, params: CreateEncodingParams, _payload: CreateEncodingPayload) -> CreateEncodingReturn<C::Output>`, a `match` on `params.identifier` whose `EncodingIdentifier::EthereumAbiV1` arm binds the concrete by `let Ok(adapter) = AbiEncoding::try_new(AbiEncodingConstructorParams);` and returns `Ok(CreateEncodingSuccessReturn { output: deps.consumer.consume_encoding(ConsumeEncodingParams, ConsumeEncodingPayload { adapter, declaration: AbiEncoding::DECLARATION }) })`
    * `[ ]`   Imports `AbiEncoding` and `AbiEncodingConstructorParams` from `crate::abi::provides`, and this module's types from `interface`
    * `[ ]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[ ]`   `adapters/encoding/src/factory/provides.rs`
    * `[ ]`   Adds `pub use super::create_encoding;` to the re-exports `encoding/derivation_context` authored

  * `[ ]`   `adapters/encoding/tests/integration_test.rs`
    * `[ ]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`; imports `create_encoding`, `CreateEncodingDeps`, `CreateEncodingPayload`, `build_create_encoding_params`, `CreateEncodingParamsOverrides`, `EncodingIdentifier`, `IEncodingConsumer`, `IEncoderAdapter`, `IDecoderAdapter`, `ConsumeEncodingParams`, `ConsumeEncodingPayload`, `EncodeParams`, `DecodeParams`, `DerivationContextDescription`, and `DerivationContextDescriptionConstructorParams` from `encoding`, the builders reached through the crate's `mocks` feature, which the workspace's test and check commands enable with `--all-features`; `DerivationContext` and the domain builders and overrides that make the reference context from `domain`; and `hex::decode`
    * `[ ]`   A test-local `RoundTripResult`, a struct with `bytes: Vec<u8>` and `described: DerivationContext`
    * `[ ]`   A test-local `RoundTripCheck`, a struct with `context: DerivationContext`, implementing `IEncodingConsumer` with `type Output = RoundTripResult;` and a `consume_encoding<E: IEncoderAdapter + IDecoderAdapter>` that, through `payload.adapter` alone, constructs `DerivationContextDescription` by `let Ok(description) = DerivationContextDescription::try_new(DerivationContextDescriptionConstructorParams);`, encodes `self.context` with `EncodeParams { description: &description }`, decodes the encoded bytes with `DecodeParams { description: &description }`, each call unpacked by `let Ok(…) = … else { panic!(…) };`, and returns the encoded bytes and the decoded context
    * `[ ]`   The reference context and the reference vector are those `encoding/abi`'s tests state
    * `[ ]`   `the_ethereum_abi_concrete_from_the_factory_round_trips_the_reference_context_through_the_family_traits`: contract: the concrete the factory constructs for the Ethereum ABI identifier encodes the reference context to the known-answer vector and decodes it back through `IEncoderAdapter`, `IDecoderAdapter`, and the description, with nothing mocked; arrange `build_create_encoding_params` with `identifier: Some(EncodingIdentifier::EthereumAbiV1)` and `CreateEncodingDeps { consumer: RoundTripCheck { context } }` holding the reference context; act `create_encoding(&deps, params, CreateEncodingPayload)`, unpacked by `let Ok(success) = … else { panic!(…) };`; assert `success.output.bytes` equals the reference vector and `success.output.described` equals the reference context, built again through the same builders
    * `[ ]`   The test block carries the full `Contract`, `Arrange`, `Act`, `Assert` header and the inline markers; nothing is mocked, since `alloy` is the outer edge

  * `[ ]`   `directionality`
    * `[ ]`   The `factory` module depends on the `abi` concrete through `crate::abi::provides` and on its own interface; `abi` depends on the `factory` module's surface, the family form's recorded cycle; `derivation_context` depends on the `factory` module's surface and nothing in the `factory` module names it; the crate's public surface is the `factory` and `derivation_context` modules' `provides`; nothing depends on the crate yet
    * `[ ]`   `kdf/blake3_keyed` consumes the family through `create_encoding` and an `IEncodingConsumer`

  * `[ ]`   `requirements`
    * `[ ]`   `create_encoding_hands_the_consumer_the_abi_concrete_and_its_declaration` passes
    * `[ ]`   `the_ethereum_abi_concrete_from_the_factory_round_trips_the_reference_context_through_the_family_traits` passes (CR-11, the derivation context's frozen serialization reached through the family's surface)
    * `[ ]`   `cargo check --workspace --all-targets --all-features`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo fmt --all --check`, and `cargo deny check` complete without error or warning in every target, the `abi` concrete's unused-item warnings having no remaining cause
    * `[ ]`   `alloy` is named nowhere outside `adapters/encoding/src/abi`, and no code outside `adapters/encoding` can name `AbiEncoding`

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