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

  * `[✅]`   `objective`
    * `[✅]`   Problem: every value hashed, signed, stored, or framed over IPC passes through one canonical encoding, each wrapping key is a KDF of the encapsulated value and the encoded derivation context, and the contracts recompute values from the same encoding, so a domain type's canonical field sequence is stated once, independent of any wire format, and every encoding concrete encodes and decodes that one sequence (CR-11, serialization frozen; the product requirements' canonical encoding position)
    * `[✅]`   Functional: the family's encoding contract states, for one described domain type, the kinds of its canonical fields in order, maps an admitted instance to its canonical field values, and maps canonical field values back to an admitted instance or to the refusal that names what failed
    * `[✅]`   Functional: the contract names value kinds only, a 32-byte fixed string, unsigned integers of 16, 32, and 64 bits, and text, and names no wire format, no byte layout, and no vendor
    * `[✅]`   Functional: the derivation context's canonical fields are, in order, the asset name, the asset version, the deployment identity, the suite identifier, the suite version, the parameter-set identifier, the group index, the piece size, the piece-group size, and the total extent, the context's component order in `docs/research/cryptography.md`'s Credential KEM statement with each component's fields in its own declared order
    * `[✅]`   Functional: mapping fields back refuses a field list of the wrong length and a field of the wrong kind, the lowest such index deciding, before any component is constructed; it then constructs each component in field order through that component's own constructor and returns the first refusal unchanged, and finally constructs the context and returns its refusal unchanged
    * `[✅]`   Functional: the reference context maps to the reference fields and the reference fields map back to the reference context
    * `[✅]`   Non-functional: the crate depends on `crates/domain` alone and on no external crate; nothing in the crate names a format or a vendor

  * `[✅]`   `role`
    * `[✅]`   Adapter family: the encoding family's family-owned description of `DerivationContext`, the first encodable domain type, and the first source file that requires the family's encoding contract, which it authors in the family's `factory` module as its producer
    * `[✅]`   Creates `factory/mod.rs` with the module wiring alone and `factory/provides.rs` with the contract and mock surface alone; `IEncoderAdapter`, `IDecoderAdapter`, the versioned encoding identifier, the capability declaration, and the ABI concrete are `encoding/abi`'s; the factory function, its types, its unit test, and the family's integration test are `encoding/factory`'s
    * `[✅]`   Does not produce or read bytes; a canonical field value is a typed value, and every byte layout belongs to an encoding concrete
    * `[✅]`   Does not check any component's own invariants; each component's constructor checks them and its refusal is carried unchanged
    * `[✅]`   Does not describe any other domain type; each further encoded type receives its own description module beside the factory, authored after its type and before the first ticket that encodes it, and adds to the value kinds only the kinds it needs
    * `[✅]`   Does not derive a key or hash; `kdf/blake3_keyed` derives from the encoding of this description's fields
    * `[✅]`   Does not carry a commit

  * `[✅]`   `module`
    * `[✅]`   Bounded context: the `adapters/encoding` crate's `factory` module, holding the encoding contract trait, the canonical value kinds, the canonical field values, the canonical field list, the contract methods' params, success, and return types, and the family's mock; and the family-owned `derivation_context` module, holding `DerivationContextDescription`, its field kinds and field count, its constructor params and return, its from-fields error, and its mock
    * `[✅]`   Creates the crate at `adapters/encoding`, admitted by the workspace's `adapters/*` glob with no edit to the root manifest
    * `[✅]`   The crate's public surface is the `factory` module's `provides` and the `derivation_context` module's `provides`, since a consumer names the description it hands an encoder
    * `[✅]`   Outside: every wire format and byte layout, the versioned encoding identifier, the encoder and decoder interfaces, the factory function, every derivation, and every component's invariants

  * `[ ]`   `deps`
    * `[✅]`   `domain`, `crates/domain`, protocol and domain ring, path dependency; supplies `DerivationContext` and its six components, their constructors, constructor params, accessors, and constructor error types; direction inward, adapter ring on domain ring, and the domain crate names nothing in this crate
    * `[ ]`   `domain` with its `mocks` feature, as a dev-dependency only; supplies the component builders and their constructor-params overrides for `derivation_context/test.rs` and `tests/derivation_context_integration_test.rs`; no mock of this crate reads a domain mock, so this crate's `mocks` feature enables nothing in `domain`
    * `[ ]`   This crate's own `mocks` feature, in `tests/derivation_context_integration_test.rs` only: `build_derivation_context_description`, `build_to_fields_params`, and `build_from_fields_params`, each with its overrides, through the crate's public surface, enabled by `--all-features`
    * `[✅]`   `domain/derivation_context` is this node's producer: `DerivationContext`, `DerivationContextConstructorParams`, `DerivationContextTryNewErrorReturn`, `DerivationContext::try_new`, the six accessors, `build_derivation_context`, and `DerivationContextConstructorParamsOverrides`, as that node states them and as `crates/domain/src/derivation_context/interface.rs` declares the types
    * `[✅]`   `core::convert::Infallible`, standard library, the error arm of every operation with no failure; `TryFrom<Vec<T>> for [T; N]`, standard library, the field-count check
    * `[✅]`   No external crate; no reverse dependency; nothing depends on this crate yet

  * `[ ]`   `context_slice`
    * `[✅]`   From `domain`: `DerivationContext::asset`, `deployment`, `suite`, `parameter_set`, `group_index`, and `geometry`, each returning a shared reference; `AssetIdentity::name` and `version` returning `&str`; `DeploymentIdentity::as_bytes` and `ParameterSetIdentifier::as_bytes` returning `&[u8; 32]`; `SuiteIdentifier::identifier` returning `&[u8; 32]` and `version` returning `u16`; `GroupIndex::value` returning `u64`; `PieceGeometry::piece_size` and `piece_group_size` returning `u32` and `total_extent` returning `u64`
    * `[✅]`   From `domain`: `AssetIdentity::try_new(AssetIdentityConstructorParams { name, version })`, `DeploymentIdentity::try_new(DeploymentIdentityConstructorParams { bytes })`, `SuiteIdentifier::try_new(SuiteIdentifierConstructorParams { identifier, version })`, `ParameterSetIdentifier::try_new(ParameterSetIdentifierConstructorParams { bytes })`, `GroupIndex::try_new(GroupIndexConstructorParams { value })` returning `Result<GroupIndex, Infallible>`, `PieceGeometry::try_new(PieceGeometryConstructorParams { piece_size, piece_group_size, total_extent })`, and `DerivationContext::try_new(DerivationContextConstructorParams { asset, deployment, suite, parameter_set, group_index, geometry })`, with the error types `AssetIdentityTryNewErrorReturn`, `DeploymentIdentityTryNewErrorReturn`, `SuiteIdentifierTryNewErrorReturn`, `ParameterSetIdentifierTryNewErrorReturn`, `PieceGeometryTryNewErrorReturn`, and `DerivationContextTryNewErrorReturn`
    * `[ ]`   From this crate's `factory` mocks, through `crate::factory::provides`, in `derivation_context/test.rs` only: `build_canonical_fields` with `CanonicalFieldsOverrides`, `build_to_fields_params` with `ToFieldsParamsOverrides`, and `build_from_fields_params` with `FromFieldsParamsOverrides`
    * `[ ]`   From `domain`'s mocks, in `derivation_context/test.rs` and `tests/derivation_context_integration_test.rs` only: `build_derivation_context` with `DerivationContextConstructorParamsOverrides`, `build_asset_identity` with `AssetIdentityConstructorParamsOverrides`, `build_deployment_identity` with `DeploymentIdentityConstructorParamsOverrides`, `build_suite_identifier` with `SuiteIdentifierConstructorParamsOverrides`, `build_parameter_set_identifier` with `ParameterSetIdentifierConstructorParamsOverrides`, `build_group_index` with `GroupIndexConstructorParamsOverrides`, and `build_piece_geometry` with `PieceGeometryConstructorParamsOverrides`

  * `[✅]`   `adapters/encoding/Cargo.toml`
    * `[✅]`   `[package]` with `name = "encoding"`, `edition.workspace = true`, `rust-version.workspace = true`, and `publish.workspace = true`; no `version` key
    * `[✅]`   `[dependencies]` with `domain = { path = "../../crates/domain" }`
    * `[✅]`   `[dev-dependencies]` with `domain = { path = "../../crates/domain", features = ["mocks"] }`
    * `[✅]`   `[features]` with `mocks = []`
    * `[✅]`   `[lints]` with `workspace = true`
    * `[✅]`   No other table

  * `[✅]`   `adapters/encoding/src/lib.rs`
    * `[✅]`   The crate barrel: `mod derivation_context;`, `mod factory;`, `pub use derivation_context::provides::*;`, and `pub use factory::provides::*;`, nothing else
    * `[✅]`   Until `factory/mod.rs` and `derivation_context/mod.rs` exist, `cargo check` reports the unresolved modules, which is the RED state for every element below that precedes them

  * `[✅]`   `adapters/encoding/src/factory/interface.rs`
    * `[✅]`   `CanonicalFieldKind`, an enum with `#[derive(Clone, Copy, Debug, PartialEq, Eq)]` and the variants `FixedBytes32`, `Unsigned16`, `Unsigned32`, `Unsigned64`, and `Text`
    * `[✅]`   `CanonicalFieldValue`, an enum with `#[derive(Clone, Debug, PartialEq, Eq)]` and the variants `FixedBytes32([u8; 32])`, `Unsigned16(u16)`, `Unsigned32(u32)`, `Unsigned64(u64)`, and `Text(String)`, each the value of the like-named kind
    * `[✅]`   `CanonicalFields`, a struct with `#[derive(Clone, Debug, PartialEq, Eq)]` and the one field `pub values: Vec<CanonicalFieldValue>`, a described instance's field values in canonical order
    * `[✅]`   `ToFieldsParams`, the fieldless struct `pub struct ToFieldsParams;`; `ToFieldsSuccessReturn`, a struct with `pub fields: CanonicalFields`; `ToFieldsReturn`, the alias `Result<ToFieldsSuccessReturn, Infallible>`, the error arm uninhabited because every admitted instance has its fields
    * `[✅]`   `FromFieldsParams`, the fieldless struct `pub struct FromFieldsParams;`; `FromFieldsSuccessReturn<T>`, a struct with `pub described: T`; `FromFieldsReturn<T, E>`, the alias `Result<FromFieldsSuccessReturn<T>, E>`
    * `[✅]`   `IEncodingContract`, the encoding contract, a trait with `type Described;`, `type FromFieldsErrorReturn: core::fmt::Debug + PartialEq + Eq;`, so every description's refusal is printable and comparable and a decoder carries it with its derives, `const FIELDS: &'static [CanonicalFieldKind];`, `fn to_fields(&self, params: ToFieldsParams, payload: &Self::Described) -> ToFieldsReturn;`, and `fn fields_to_value(&self, params: FromFieldsParams, payload: CanonicalFields) -> FromFieldsReturn<Self::Described, Self::FromFieldsErrorReturn>;`
    * `[✅]`   No derives beyond those stated; imports `core::convert::Infallible`; names no format, no vendor, no domain type, and no description

  * `[✅]`   `adapters/encoding/src/derivation_context/interface.rs`
    * `[✅]`   `DERIVATION_CONTEXT_FIELD_COUNT`, a `pub const` of type `usize` with value `10`
    * `[✅]`   `DERIVATION_CONTEXT_FIELD_KINDS`, a `pub const` of type `[CanonicalFieldKind; DERIVATION_CONTEXT_FIELD_COUNT]` with the value `[CanonicalFieldKind::Text, CanonicalFieldKind::Text, CanonicalFieldKind::FixedBytes32, CanonicalFieldKind::FixedBytes32, CanonicalFieldKind::Unsigned16, CanonicalFieldKind::FixedBytes32, CanonicalFieldKind::Unsigned64, CanonicalFieldKind::Unsigned32, CanonicalFieldKind::Unsigned32, CanonicalFieldKind::Unsigned64]`: asset name, asset version, deployment identity, suite identifier, suite version, parameter-set identifier, group index, piece size, piece-group size, total extent
    * `[✅]`   `DerivationContextDescription`, the unit struct `pub struct DerivationContextDescription;`, the family's description of `DerivationContext`
    * `[✅]`   `DerivationContextDescriptionConstructorParams`, the fieldless struct `pub struct DerivationContextDescriptionConstructorParams;`, the constructor's deps slot
    * `[✅]`   `DerivationContextDescriptionTryNewReturn`, the alias `Result<DerivationContextDescription, Infallible>`; the error arm is uninhabited because the description takes no configuration
    * `[✅]`   `DerivationContextFromFieldsErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]` and the variants `FieldCount { expected: usize, actual: usize }`, `FieldKind { index: usize, expected: CanonicalFieldKind }`, `AssetIdentity(AssetIdentityTryNewErrorReturn)`, `DeploymentIdentity(DeploymentIdentityTryNewErrorReturn)`, `SuiteIdentifier(SuiteIdentifierTryNewErrorReturn)`, `ParameterSetIdentifier(ParameterSetIdentifierTryNewErrorReturn)`, `PieceGeometry(PieceGeometryTryNewErrorReturn)`, and `DerivationContext(DerivationContextTryNewErrorReturn)`, each component's refusal carried unchanged in its own variant; `GroupIndex` has no variant, since its constructor has no refusal
    * `[✅]`   Imports `CanonicalFieldKind` from `crate::factory::provides`, the six domain error types from `domain`, and `core::convert::Infallible`; declares nothing else

  * `[✅]`   `adapters/encoding/src/derivation_context/interaction.spec.md`
    * `[✅]`   `DerivationContextDescription::try_new(params: DerivationContextDescriptionConstructorParams) -> DerivationContextDescriptionTryNewReturn`: one branch; condition any params; decision none; dependency call none; outcome `Ok(DerivationContextDescription)`; the error arm has no branch
    * `[✅]`   `IEncodingContract` for `DerivationContextDescription`: `Described` is `DerivationContext`, `FromFieldsErrorReturn` is `DerivationContextFromFieldsErrorReturn`, and `FIELDS` is `&DERIVATION_CONTEXT_FIELD_KINDS`
    * `[✅]`   `to_fields`: one branch; condition any admitted context; decision none; dependency calls the context's six accessors and each component's accessors, once each; outcome `Ok(ToFieldsSuccessReturn { fields: CanonicalFields { values } })` where `values` is `Text` of the asset name, `Text` of the asset version, each copied into a `String`, `FixedBytes32` of the deployment identity's bytes, `FixedBytes32` of the suite identifier, `Unsigned16` of the suite version, `FixedBytes32` of the parameter-set identifier's bytes, `Unsigned64` of the group index, `Unsigned32` of the piece size, `Unsigned32` of the piece-group size, and `Unsigned64` of the total extent, in that order; `params` carries no control and is not read
    * `[✅]`   `fields_to_value`, wrong count: condition `payload.values` does not convert into `[CanonicalFieldValue; DERIVATION_CONTEXT_FIELD_COUNT]`; decision `TryFrom<Vec<CanonicalFieldValue>>` for the array, which returns the vector on failure; dependency call none; outcome `Err(DerivationContextFromFieldsErrorReturn::FieldCount { expected: DERIVATION_CONTEXT_FIELD_COUNT, actual })`, `actual` the returned vector's length
    * `[✅]`   `fields_to_value`, wrong kind: condition the count matches and the value at some index is not the variant `DERIVATION_CONTEXT_FIELD_KINDS` names at that index; decision the array is destructured into its fields and each is matched against its expected variant, index `0` through `9` in ascending order, before any constructor is called; dependency call none; outcome `Err(DerivationContextFromFieldsErrorReturn::FieldKind { index, expected })` for the lowest such index, `expected` the kind `DERIVATION_CONTEXT_FIELD_KINDS` names there
    * `[✅]`   `fields_to_value`, component refused: condition every kind matches and a component's constructor refuses; decision the constructors are called in field order, `AssetIdentity::try_new` over the asset name and version, `DeploymentIdentity::try_new`, `SuiteIdentifier::try_new` over the suite identifier and version, `ParameterSetIdentifier::try_new`, `GroupIndex::try_new` unpacked irrefutably, and `PieceGeometry::try_new` over the piece size, piece-group size, and total extent; dependency call each constructor at most once, none after the first refusal; outcome `Err` holding the first refusal unchanged in its variant, `AssetIdentity`, `DeploymentIdentity`, `SuiteIdentifier`, `ParameterSetIdentifier`, or `PieceGeometry`
    * `[✅]`   `fields_to_value`, context refused: condition every component is admitted and `DerivationContext::try_new` refuses; decision the context constructor's result; dependency call `DerivationContext::try_new` once over the six components; outcome `Err(DerivationContextFromFieldsErrorReturn::DerivationContext(error))`, the refusal unchanged
    * `[✅]`   `fields_to_value`, admitted: condition every check and constructor passes; decision the same; dependency call the same constructors; outcome `Ok(FromFieldsSuccessReturn { described })` holding the constructed context
    * `[✅]`   Ordering: the count precedes every kind, every kind precedes every constructor, the constructors run in field order, and the context constructor runs last; the same payload always yields the same outcome; `params` carries no control and is not read
    * `[✅]`   Invariant: the kinds of the values `to_fields` returns are `DERIVATION_CONTEXT_FIELD_KINDS` in order, and `fields_to_value` admits exactly that sequence
    * `[ ]`   Own entry, round trip through the canonical fields: condition an admitted `DerivationContext` is handed to `to_fields` and the fields it returns are handed to `fields_to_value`; outcome `Ok(FromFieldsSuccessReturn { described })` with `described` equal to the original context; variation a context whose same-kind fields hold distinct values, among the text values, among the 32-byte values, among the 32-bit values, and among the 64-bit values, whose 16-bit suite version is at its maximum, whose piece-group size exceeds the `i32` range, and whose group index and total extent exceed the 32-bit range, so a swap of same-kind fields, a dropped field, or a narrowed width yields a refusal or an unequal context; edge that must survive the composition: the context constructor's group-index rule admits the rebuilt context at the full width of the extent, so the fields of an admitted context are never refused on the way back
    * `[ ]`   Public surface: an outside caller invokes `DerivationContextDescription::try_new` (under the `mocks` feature, the official mock builder), `IEncodingContract::to_fields`, and `IEncodingContract::fields_to_value`, and observes the fields, the context, or the refusal each returns; the entry proven is the round-trip entry

  * `[✅]`   `adapters/encoding/src/factory/mock.rs`
    * `[ ]`   `CanonicalFieldsOverrides`, `#[derive(Default)]`, the one field `pub values: Option<Vec<CanonicalFieldValue>>`
    * `[ ]`   `build_canonical_fields(overrides: CanonicalFieldsOverrides) -> CanonicalFields`, the omitted `values` defaulting to one value of each kind: `FixedBytes32` of bytes each `0x01`, `Unsigned16(1)`, `Unsigned32(2)`, `Unsigned64(3)`, and `Text("text")`
    * `[ ]`   `ToFieldsParamsOverrides`, the fieldless `#[derive(Default)]` struct `pub struct ToFieldsParamsOverrides;`, and `build_to_fields_params(overrides: ToFieldsParamsOverrides) -> ToFieldsParams`
    * `[ ]`   `ToFieldsSuccessReturnOverrides`, `#[derive(Default)]`, the one field `pub fields: Option<CanonicalFields>`, and `build_to_fields_success_return(overrides: ToFieldsSuccessReturnOverrides) -> ToFieldsSuccessReturn`, the omitted `fields` defaulting to `build_canonical_fields` called with `Default::default()`
    * `[ ]`   `FromFieldsParamsOverrides`, the fieldless `#[derive(Default)]` struct `pub struct FromFieldsParamsOverrides;`, and `build_from_fields_params(overrides: FromFieldsParamsOverrides) -> FromFieldsParams`
    * `[ ]`   `FromFieldsSuccessReturnOverrides`, `#[derive(Default)]`, the one field `pub described: Option<CanonicalFields>`, and `build_from_fields_success_return(overrides: FromFieldsSuccessReturnOverrides) -> FromFieldsSuccessReturn<CanonicalFields>`, the omitted `described` defaulting to `build_canonical_fields` called with `Default::default()`; the described type is fixed at `CanonicalFields`, the type `MockIEncodingContract` describes
    * `[ ]`   `MockIEncodingContract`, the unit struct `pub struct MockIEncodingContract;` implementing `IEncodingContract` with `type Described = CanonicalFields;`, `type FromFieldsErrorReturn = Infallible;`, and `const FIELDS: &'static [CanonicalFieldKind]` holding `FixedBytes32`, `Unsigned16`, `Unsigned32`, `Unsigned64`, and `Text`, the kinds of the default `values` of `build_canonical_fields` in order; `to_fields` returns `Ok(build_to_fields_success_return(Default::default()))` and `fields_to_value` returns `Ok(build_from_fields_success_return(Default::default()))`, neither reading its params or its payload
    * `[ ]`   No corruptions type and no invalidator: every owned object type is a typed value the crate never deserializes, the crate has no serialization dependency, and the field list a decoder refuses is a `values` override; no mock for `CanonicalFieldKind` or `CanonicalFieldValue`, enums used by their production variants, none for `ToFieldsReturn` or `FromFieldsReturn`, aliases of `Result`, and no function mock, since the trait's methods are `MockIEncodingContract`'s
    * `[ ]`   Imports the contract's names from `super::interface` and `core::convert::Infallible`

  * `[✅]`   `adapters/encoding/src/factory/mod.rs`
    * `[✅]`   Module wiring only: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, and `pub mod provides;`, nothing else

  * `[✅]`   `adapters/encoding/src/factory/provides.rs`
    * `[✅]`   `pub use super::interface::*;` and `#[cfg(any(test, feature = "mocks"))] pub use super::mock::*;`, nothing else

  * `[ ]`   `adapters/encoding/src/derivation_context/mock.rs`
    * `[ ]`   `DerivationContextDescriptionConstructorParamsOverrides`, the fieldless `#[derive(Default)]` struct `pub struct DerivationContextDescriptionConstructorParamsOverrides;`
    * `[ ]`   `build_derivation_context_description_constructor_params(overrides: DerivationContextDescriptionConstructorParamsOverrides) -> DerivationContextDescriptionConstructorParams`
    * `[ ]`   `build_derivation_context_description(overrides: DerivationContextDescriptionConstructorParamsOverrides) -> DerivationContextDescription`, returning the real instance from `DerivationContextDescription::try_new(build_derivation_context_description_constructor_params(overrides))`, whose error arm is uninhabited and is unpacked irrefutably
    * `[ ]`   No corruptions type and no invalidator: the constructor params carry no field and the crate has no serialization dependency; no `DerivationContextDescription` overrides, invalidator, or mock function, since the type is built as a real instance and owns no free function; no mock for `DerivationContextFromFieldsErrorReturn`, an enum used by its production variants, for `DerivationContextDescriptionTryNewReturn`, an alias of `Result`, or for `DERIVATION_CONTEXT_FIELD_COUNT` and `DERIVATION_CONTEXT_FIELD_KINDS`, constants used by their production values
    * `[ ]`   Imports `DerivationContextDescription` and `DerivationContextDescriptionConstructorParams` from `super::interface`

  * `[ ]`   `adapters/encoding/src/derivation_context/test.rs`
    * `[ ]`   Collaborators: the `domain` accessors and constructors are imported functions with no official mock and run as themselves; every context, component, and expected value is built with `domain`'s builders, every subject with `build_derivation_context_description`, and every params value with `build_to_fields_params` or `build_from_fields_params`
    * `[ ]`   Reference values by index of `DERIVATION_CONTEXT_FIELD_KINDS`: `Text("left-pad")`, `Text("1.3.0")`, `FixedBytes32` of bytes each `0x44`, `FixedBytes32` of bytes each `0x55`, `Unsigned16(0x0102)`, `FixedBytes32` of bytes each `0x66`, `Unsigned64(2)`, `Unsigned32(32768)`, `Unsigned32(65536)`, and `Unsigned64(262144)`
    * `[ ]`   Reference fields: `build_canonical_fields` overriding `values` with the reference values
    * `[ ]`   Reference context: `build_derivation_context` overriding the asset with `build_asset_identity` at name `left-pad` and version `1.3.0`, the deployment with `build_deployment_identity` at bytes each `0x44`, the suite with `build_suite_identifier` at identifier bytes each `0x55` and version `0x0102`, the parameter set with `build_parameter_set_identifier` at bytes each `0x66`, the group index with `build_group_index` at value `2`, and the geometry with `build_piece_geometry` at piece size `32768`, piece-group size `65536`, and extent `262144`, a geometry of four groups
    * `[ ]`   `try_new_returns_the_description`
      * `[ ]`   Contract: `DerivationContextDescription::try_new(params)`, one branch: any params → `Ok(DerivationContextDescription)`; the error arm has no branch
      * `[ ]`   Arrange: the params from `build_derivation_context_description_constructor_params`
      * `[ ]`   Act: `DerivationContextDescription::try_new(params)`
      * `[ ]`   Assert: `result.is_ok()` is true
    * `[ ]`   `to_fields_lists_the_reference_context_fields_in_canonical_order`
      * `[ ]`   Contract: `to_fields`, one branch: any admitted context → `Ok(ToFieldsSuccessReturn { fields: CanonicalFields { values } })` with `values` the asset name and version as `Text`, the deployment bytes, suite identifier, and parameter-set bytes as `FixedBytes32`, the suite version as `Unsigned16`, the group index as `Unsigned64`, the piece size and piece-group size as `Unsigned32`, and the total extent as `Unsigned64`, in that order
      * `[ ]`   Arrange: the reference context, whose same-kind fields hold distinct values, so a swap among the text values, among the 32-byte values, among the 32-bit values, or among the 64-bit values changes the list
      * `[ ]`   Act: `description.to_fields(params, &context)`
      * `[ ]`   Assert: the `fields` of the `Ok` arm equal the reference fields by `assert_eq!`, the expectation the reference values in canonical order, stated apart from the context builder's overrides
    * `[ ]`   `fields_to_value_returns_the_context_the_reference_fields_describe`
      * `[ ]`   Contract: `fields_to_value`, admitted: every check and constructor passes → `Ok(FromFieldsSuccessReturn { described })` holding the constructed context
      * `[ ]`   Arrange: the reference fields, whose same-kind values are distinct, so a value taken from the wrong field changes the constructed context or is refused
      * `[ ]`   Act: `description.fields_to_value(params, fields)`
      * `[ ]`   Assert: the `described` of the `Ok` arm equals the reference context by `assert_eq!`, the reference context built by `domain`'s builders and not by the subject
    * `[ ]`   `fields_to_value_rejects_a_field_list_one_short`
      * `[ ]`   Contract: `fields_to_value`, wrong count: `payload.values` does not convert into `[CanonicalFieldValue; DERIVATION_CONTEXT_FIELD_COUNT]` → `Err(DerivationContextFromFieldsErrorReturn::FieldCount { expected: DERIVATION_CONTEXT_FIELD_COUNT, actual })`, `actual` the vector's length
      * `[ ]`   Arrange: `build_canonical_fields` overriding `values` with the reference values without index `9`, every value present of its declared kind, so only the count refuses
      * `[ ]`   Act: `description.fields_to_value(params, fields)`
      * `[ ]`   Assert: `result.err()` equals `Some(DerivationContextFromFieldsErrorReturn::FieldCount { expected: DERIVATION_CONTEXT_FIELD_COUNT, actual: 9 })` by `assert_eq!`
    * `[ ]`   `fields_to_value_rejects_a_field_list_one_long`
      * `[ ]`   Contract: `fields_to_value`, wrong count: `payload.values` does not convert into `[CanonicalFieldValue; DERIVATION_CONTEXT_FIELD_COUNT]` → `Err(DerivationContextFromFieldsErrorReturn::FieldCount { expected: DERIVATION_CONTEXT_FIELD_COUNT, actual })`, `actual` the vector's length
      * `[ ]`   Arrange: `build_canonical_fields` overriding `values` with the reference values followed by `Unsigned64(0)`, every value at indexes `0` through `9` of its declared kind, so only the count refuses
      * `[ ]`   Act: `description.fields_to_value(params, fields)`
      * `[ ]`   Assert: `result.err()` equals `Some(DerivationContextFromFieldsErrorReturn::FieldCount { expected: DERIVATION_CONTEXT_FIELD_COUNT, actual: 11 })` by `assert_eq!`
    * `[ ]`   `fields_to_value_checks_the_count_before_any_kind`
      * `[ ]`   Contract: `fields_to_value`, ordering: the count precedes every kind
      * `[ ]`   Arrange: `build_canonical_fields` overriding `values` with the reference values without index `9` and with the value at index `0` replaced by `Unsigned16(1)`, a list both short and holding a wrong kind, so a kind check ahead of the count reports the kind
      * `[ ]`   Act: `description.fields_to_value(params, fields)`
      * `[ ]`   Assert: `result.err()` equals `Some(DerivationContextFromFieldsErrorReturn::FieldCount { expected: DERIVATION_CONTEXT_FIELD_COUNT, actual: 9 })` by `assert_eq!`
    * `[ ]`   `fields_to_value_rejects_an_asset_name_of_the_wrong_kind`
      * `[ ]`   Contract: `fields_to_value`, wrong kind: the count matches and the value at index `0` is not the variant `DERIVATION_CONTEXT_FIELD_KINDS` names there → `Err(DerivationContextFromFieldsErrorReturn::FieldKind { index: 0, expected })`
      * `[ ]`   Arrange: `build_canonical_fields` overriding `values` with the reference values and the value at index `0` replaced by `Unsigned16(1)`, every other value of its declared kind, so only the kind at index `0` refuses
      * `[ ]`   Act: `description.fields_to_value(params, fields)`
      * `[ ]`   Assert: `result.err()` equals `Some(DerivationContextFromFieldsErrorReturn::FieldKind { index: 0, expected: CanonicalFieldKind::Text })` by `assert_eq!`
    * `[ ]`   `fields_to_value_rejects_an_asset_version_of_the_wrong_kind`
      * `[ ]`   Contract: `fields_to_value`, wrong kind: the count matches and the value at index `1` is not the variant `DERIVATION_CONTEXT_FIELD_KINDS` names there → `Err(DerivationContextFromFieldsErrorReturn::FieldKind { index: 1, expected })`
      * `[ ]`   Arrange: `build_canonical_fields` overriding `values` with the reference values and the value at index `1` replaced by `Unsigned16(1)`, every other value of its declared kind, so only the kind at index `1` refuses
      * `[ ]`   Act: `description.fields_to_value(params, fields)`
      * `[ ]`   Assert: `result.err()` equals `Some(DerivationContextFromFieldsErrorReturn::FieldKind { index: 1, expected: CanonicalFieldKind::Text })` by `assert_eq!`
    * `[ ]`   `fields_to_value_rejects_a_deployment_identity_of_the_wrong_kind`
      * `[ ]`   Contract: `fields_to_value`, wrong kind: the count matches and the value at index `2` is not the variant `DERIVATION_CONTEXT_FIELD_KINDS` names there → `Err(DerivationContextFromFieldsErrorReturn::FieldKind { index: 2, expected })`
      * `[ ]`   Arrange: `build_canonical_fields` overriding `values` with the reference values and the value at index `2` replaced by `Text("left-pad")`, every other value of its declared kind, so only the kind at index `2` refuses
      * `[ ]`   Act: `description.fields_to_value(params, fields)`
      * `[ ]`   Assert: `result.err()` equals `Some(DerivationContextFromFieldsErrorReturn::FieldKind { index: 2, expected: CanonicalFieldKind::FixedBytes32 })` by `assert_eq!`
    * `[ ]`   `fields_to_value_rejects_a_suite_identifier_of_the_wrong_kind`
      * `[ ]`   Contract: `fields_to_value`, wrong kind: the count matches and the value at index `3` is not the variant `DERIVATION_CONTEXT_FIELD_KINDS` names there → `Err(DerivationContextFromFieldsErrorReturn::FieldKind { index: 3, expected })`
      * `[ ]`   Arrange: `build_canonical_fields` overriding `values` with the reference values and the value at index `3` replaced by `Text("left-pad")`, every other value of its declared kind, so only the kind at index `3` refuses
      * `[ ]`   Act: `description.fields_to_value(params, fields)`
      * `[ ]`   Assert: `result.err()` equals `Some(DerivationContextFromFieldsErrorReturn::FieldKind { index: 3, expected: CanonicalFieldKind::FixedBytes32 })` by `assert_eq!`
    * `[ ]`   `fields_to_value_rejects_a_suite_version_of_the_wrong_kind`
      * `[ ]`   Contract: `fields_to_value`, wrong kind: the count matches and the value at index `4` is not the variant `DERIVATION_CONTEXT_FIELD_KINDS` names there → `Err(DerivationContextFromFieldsErrorReturn::FieldKind { index: 4, expected })`
      * `[ ]`   Arrange: `build_canonical_fields` overriding `values` with the reference values and the value at index `4` replaced by `Unsigned32(0x0102)`, the same number at a wider width, every other value of its declared kind, so only the kind at index `4` refuses
      * `[ ]`   Act: `description.fields_to_value(params, fields)`
      * `[ ]`   Assert: `result.err()` equals `Some(DerivationContextFromFieldsErrorReturn::FieldKind { index: 4, expected: CanonicalFieldKind::Unsigned16 })` by `assert_eq!`
    * `[ ]`   `fields_to_value_rejects_a_parameter_set_identifier_of_the_wrong_kind`
      * `[ ]`   Contract: `fields_to_value`, wrong kind: the count matches and the value at index `5` is not the variant `DERIVATION_CONTEXT_FIELD_KINDS` names there → `Err(DerivationContextFromFieldsErrorReturn::FieldKind { index: 5, expected })`
      * `[ ]`   Arrange: `build_canonical_fields` overriding `values` with the reference values and the value at index `5` replaced by `Text("left-pad")`, every other value of its declared kind, so only the kind at index `5` refuses
      * `[ ]`   Act: `description.fields_to_value(params, fields)`
      * `[ ]`   Assert: `result.err()` equals `Some(DerivationContextFromFieldsErrorReturn::FieldKind { index: 5, expected: CanonicalFieldKind::FixedBytes32 })` by `assert_eq!`
    * `[ ]`   `fields_to_value_rejects_a_group_index_of_the_wrong_kind`
      * `[ ]`   Contract: `fields_to_value`, wrong kind: the count matches and the value at index `6` is not the variant `DERIVATION_CONTEXT_FIELD_KINDS` names there → `Err(DerivationContextFromFieldsErrorReturn::FieldKind { index: 6, expected })`
      * `[ ]`   Arrange: `build_canonical_fields` overriding `values` with the reference values and the value at index `6` replaced by `Unsigned32(2)`, the same number at a narrower width, every other value of its declared kind, so only the kind at index `6` refuses
      * `[ ]`   Act: `description.fields_to_value(params, fields)`
      * `[ ]`   Assert: `result.err()` equals `Some(DerivationContextFromFieldsErrorReturn::FieldKind { index: 6, expected: CanonicalFieldKind::Unsigned64 })` by `assert_eq!`
    * `[ ]`   `fields_to_value_rejects_a_piece_size_of_the_wrong_kind`
      * `[ ]`   Contract: `fields_to_value`, wrong kind: the count matches and the value at index `7` is not the variant `DERIVATION_CONTEXT_FIELD_KINDS` names there → `Err(DerivationContextFromFieldsErrorReturn::FieldKind { index: 7, expected })`
      * `[ ]`   Arrange: `build_canonical_fields` overriding `values` with the reference values and the value at index `7` replaced by `Unsigned64(32768)`, the same number at a wider width, every other value of its declared kind, so only the kind at index `7` refuses
      * `[ ]`   Act: `description.fields_to_value(params, fields)`
      * `[ ]`   Assert: `result.err()` equals `Some(DerivationContextFromFieldsErrorReturn::FieldKind { index: 7, expected: CanonicalFieldKind::Unsigned32 })` by `assert_eq!`
    * `[ ]`   `fields_to_value_rejects_a_piece_group_size_of_the_wrong_kind`
      * `[ ]`   Contract: `fields_to_value`, wrong kind: the count matches and the value at index `8` is not the variant `DERIVATION_CONTEXT_FIELD_KINDS` names there → `Err(DerivationContextFromFieldsErrorReturn::FieldKind { index: 8, expected })`
      * `[ ]`   Arrange: `build_canonical_fields` overriding `values` with the reference values and the value at index `8` replaced by `Unsigned64(65536)`, the same number at a wider width, every other value of its declared kind, so only the kind at index `8` refuses
      * `[ ]`   Act: `description.fields_to_value(params, fields)`
      * `[ ]`   Assert: `result.err()` equals `Some(DerivationContextFromFieldsErrorReturn::FieldKind { index: 8, expected: CanonicalFieldKind::Unsigned32 })` by `assert_eq!`
    * `[ ]`   `fields_to_value_rejects_a_total_extent_of_the_wrong_kind`
      * `[ ]`   Contract: `fields_to_value`, wrong kind: the count matches and the value at index `9` is not the variant `DERIVATION_CONTEXT_FIELD_KINDS` names there → `Err(DerivationContextFromFieldsErrorReturn::FieldKind { index: 9, expected })`
      * `[ ]`   Arrange: `build_canonical_fields` overriding `values` with the reference values and the value at index `9` replaced by `Unsigned32(262144)`, the same number at a narrower width, every other value of its declared kind, so only the kind at index `9` refuses
      * `[ ]`   Act: `description.fields_to_value(params, fields)`
      * `[ ]`   Assert: `result.err()` equals `Some(DerivationContextFromFieldsErrorReturn::FieldKind { index: 9, expected: CanonicalFieldKind::Unsigned64 })` by `assert_eq!`
    * `[ ]`   `fields_to_value_reports_the_lowest_field_of_the_wrong_kind`
      * `[ ]`   Contract: `fields_to_value`, wrong kind: the count matches and more than one value is not the variant `DERIVATION_CONTEXT_FIELD_KINDS` names at its index → `Err(DerivationContextFromFieldsErrorReturn::FieldKind { index, expected })` for the lowest such index
      * `[ ]`   Arrange: `build_canonical_fields` overriding `values` with the reference values, the value at index `3` replaced by `Text("left-pad")` and the value at index `8` replaced by `Unsigned64(65536)`, so a check that runs from the highest index reports index `8`
      * `[ ]`   Act: `description.fields_to_value(params, fields)`
      * `[ ]`   Assert: `result.err()` equals `Some(DerivationContextFromFieldsErrorReturn::FieldKind { index: 3, expected: CanonicalFieldKind::FixedBytes32 })` by `assert_eq!`
    * `[ ]`   `fields_to_value_checks_every_kind_before_constructing_any_component`
      * `[ ]`   Contract: `fields_to_value`, ordering: every kind precedes every constructor
      * `[ ]`   Arrange: `build_canonical_fields` overriding `values` with the reference values, the value at index `0` replaced by `Text("")`, the empty asset name `AssetIdentity::try_new` refuses, and the value at index `9` replaced by `Unsigned32(262144)`, so a constructor that ran before the kind at index `9` was checked returns the asset-identity refusal
      * `[ ]`   Act: `description.fields_to_value(params, fields)`
      * `[ ]`   Assert: `result.err()` equals `Some(DerivationContextFromFieldsErrorReturn::FieldKind { index: 9, expected: CanonicalFieldKind::Unsigned64 })` by `assert_eq!`
    * `[ ]`   `fields_to_value_returns_the_asset_identity_refusal_unchanged`
      * `[ ]`   Contract: `fields_to_value`, component refused: every kind matches and `AssetIdentity::try_new` refuses → `Err` holding the refusal unchanged in the `AssetIdentity` variant
      * `[ ]`   Arrange: `build_canonical_fields` overriding `values` with the reference values and the value at index `0` replaced by `Text("left pad")`, whose space at index `4` is outside visible ASCII, every other component admitted
      * `[ ]`   Act: `description.fields_to_value(params, fields)`
      * `[ ]`   Assert: `result.err()` equals `Some(DerivationContextFromFieldsErrorReturn::AssetIdentity(AssetIdentityTryNewErrorReturn::NameByteOutsideVisibleAscii { index: 4, byte: 0x20 }))` by `assert_eq!`, the whole refusal the asset identity's constructor declares for that name
    * `[ ]`   `fields_to_value_returns_the_deployment_identity_refusal_unchanged`
      * `[ ]`   Contract: `fields_to_value`, component refused: every kind matches and `DeploymentIdentity::try_new` refuses → `Err` holding the refusal unchanged in the `DeploymentIdentity` variant
      * `[ ]`   Arrange: `build_canonical_fields` overriding `values` with the reference values and the value at index `2` replaced by `FixedBytes32` of bytes each `0x00`, every other component admitted
      * `[ ]`   Act: `description.fields_to_value(params, fields)`
      * `[ ]`   Assert: `result.err()` equals `Some(DerivationContextFromFieldsErrorReturn::DeploymentIdentity(DeploymentIdentityTryNewErrorReturn::AllZero))` by `assert_eq!`
    * `[ ]`   `fields_to_value_returns_the_suite_identifier_refusal_unchanged`
      * `[ ]`   Contract: `fields_to_value`, component refused: every kind matches and `SuiteIdentifier::try_new` refuses → `Err` holding the refusal unchanged in the `SuiteIdentifier` variant
      * `[ ]`   Arrange: `build_canonical_fields` overriding `values` with the reference values and the value at index `3` replaced by `FixedBytes32` of bytes each `0x00`, the suite version at index `4` keeping `Unsigned16(0x0102)`, every other component admitted
      * `[ ]`   Act: `description.fields_to_value(params, fields)`
      * `[ ]`   Assert: `result.err()` equals `Some(DerivationContextFromFieldsErrorReturn::SuiteIdentifier(SuiteIdentifierTryNewErrorReturn::AllZeroIdentifier))` by `assert_eq!`
    * `[ ]`   `fields_to_value_returns_the_parameter_set_identifier_refusal_unchanged`
      * `[ ]`   Contract: `fields_to_value`, component refused: every kind matches and `ParameterSetIdentifier::try_new` refuses → `Err` holding the refusal unchanged in the `ParameterSetIdentifier` variant
      * `[ ]`   Arrange: `build_canonical_fields` overriding `values` with the reference values and the value at index `5` replaced by `FixedBytes32` of bytes each `0x00`, every other component admitted
      * `[ ]`   Act: `description.fields_to_value(params, fields)`
      * `[ ]`   Assert: `result.err()` equals `Some(DerivationContextFromFieldsErrorReturn::ParameterSetIdentifier(ParameterSetIdentifierTryNewErrorReturn::AllZero))` by `assert_eq!`
    * `[ ]`   `fields_to_value_returns_the_piece_geometry_refusal_unchanged`
      * `[ ]`   Contract: `fields_to_value`, component refused: every kind matches and `PieceGeometry::try_new` refuses → `Err` holding the refusal unchanged in the `PieceGeometry` variant
      * `[ ]`   Arrange: `build_canonical_fields` overriding `values` with the reference values and the value at index `8` replaced by `Unsigned32(20000)`, a piece-group size that is not a multiple of the piece size `32768` at index `7`, every other component admitted
      * `[ ]`   Act: `description.fields_to_value(params, fields)`
      * `[ ]`   Assert: `result.err()` equals `Some(DerivationContextFromFieldsErrorReturn::PieceGeometry(PieceGeometryTryNewErrorReturn::PieceGroupSizeNotMultipleOfPieceSize { piece_group_size: 20000, piece_size: 32768 }))` by `assert_eq!`
    * `[ ]`   `fields_to_value_constructs_the_components_in_field_order`
      * `[ ]`   Contract: `fields_to_value`, component refused: the constructors are called in field order and none after the first refusal → `Err` holding the first refusal unchanged
      * `[ ]`   Arrange: `build_canonical_fields` overriding `values` with the reference values, the value at index `0` replaced by `Text("")`, which `AssetIdentity::try_new` refuses, and the value at index `8` replaced by `Unsigned32(20000)`, which `PieceGeometry::try_new` refuses, the first and the last component constructed, so a reverse or interleaved order reports the geometry refusal
      * `[ ]`   Act: `description.fields_to_value(params, fields)`
      * `[ ]`   Assert: `result.err()` equals `Some(DerivationContextFromFieldsErrorReturn::AssetIdentity(AssetIdentityTryNewErrorReturn::EmptyName))` by `assert_eq!`
    * `[ ]`   `fields_to_value_returns_the_derivation_context_refusal_unchanged`
      * `[ ]`   Contract: `fields_to_value`, context refused: every component is admitted and `DerivationContext::try_new` refuses → `Err(DerivationContextFromFieldsErrorReturn::DerivationContext(error))`, the refusal unchanged
      * `[ ]`   Arrange: `build_canonical_fields` overriding `values` with the reference values and the values at indexes `6` through `9` replaced by `Unsigned64(4)`, `Unsigned32(16384)`, `Unsigned32(16384)`, and `Unsigned64(65536)`, a geometry of four groups and a group index of `4`, one beyond its last group, every component admitted
      * `[ ]`   Act: `description.fields_to_value(params, fields)`
      * `[ ]`   Assert: `result.err()` equals `Some(DerivationContextFromFieldsErrorReturn::DerivationContext(DerivationContextTryNewErrorReturn::GroupIndexOutOfRange { group_index: 4, group_count: 4 }))` by `assert_eq!`

  * `[✅]`   `construction`
    * `[✅]`   `DerivationContextDescription::try_new` is the description's only producer; a consumer that encodes or decodes a derivation context constructs it once and hands it to the encoder or decoder the encoding factory returns; it holds no state, so one instance serves every call
    * `[ ]`   `build_derivation_context_description` is the description's mock producer: it returns the real instance `try_new` constructs over the fieldless params `build_derivation_context_description_constructor_params` builds, so a test reaches a description through the builder and never by naming the unit struct

  * `[✅]`   `adapters/encoding/src/derivation_context/mod.rs`
    * `[✅]`   Module declarations: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, `pub mod provides;`, and `#[cfg(test)] mod test;`
    * `[✅]`   `impl DerivationContextDescription` with `pub fn try_new(_params: DerivationContextDescriptionConstructorParams) -> DerivationContextDescriptionTryNewReturn` returning `Ok(DerivationContextDescription)`
    * `[✅]`   `impl IEncodingContract for DerivationContextDescription` with `type Described = DerivationContext;`, `type FromFieldsErrorReturn = DerivationContextFromFieldsErrorReturn;`, `const FIELDS: &'static [CanonicalFieldKind] = &DERIVATION_CONTEXT_FIELD_KINDS;`, and `to_fields` and `fields_to_value` realizing the branches and ordering of the interaction spec; `fields_to_value` converts the vector into the array, destructures it into its fields, matches each against its expected variant by `let … else` in ascending index order, then calls the constructors, the group index unpacked by `let Ok(group_index) = GroupIndex::try_new(…);` and every other refusal wrapped in its variant and returned
    * `[✅]`   Imports the contract's names from `crate::factory::provides`, the domain types, constructor params, and error types from `domain`, and this module's names from `interface`
    * `[✅]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[✅]`   `adapters/encoding/src/derivation_context/provides.rs`
    * `[✅]`   `pub use super::interface::*;` and `#[cfg(any(test, feature = "mocks"))] pub use super::mock::*;`, nothing else

  * `[ ]`   `adapters/encoding/tests/derivation_context_integration_test.rs`
    * `[ ]`   `a_context_survives_the_round_trip_through_its_canonical_fields`
      * `[ ]`   Contract: the own entry, round trip through the canonical fields: an admitted `DerivationContext` handed to `to_fields`, and the fields it returns handed to `fields_to_value`, yields `Ok(FromFieldsSuccessReturn { described })` with `described` equal to the original context
      * `[ ]`   Arrange: `build_derivation_context_description`, `build_to_fields_params`, and `build_from_fields_params` from the crate's public surface under the `mocks` feature; `build_derivation_context` overriding the asset with `build_asset_identity` at name `left-pad` and version `1.3.0`, the deployment with `build_deployment_identity` at bytes each `0x44`, the suite with `build_suite_identifier` at identifier bytes each `0x55` and version `u16::MAX`, the parameter set with `build_parameter_set_identifier` at bytes each `0x66`, the group index with `build_group_index` at value `4294967296`, and the geometry with `build_piece_geometry` at piece size `16384`, piece-group size `2147483648`, and extent `u64::MAX`; the same-kind fields hold distinct values, the suite version is at its maximum, the piece-group size exceeds the `i32` range, and the group index and extent exceed the 32-bit range, so a swap of same-kind fields, a dropped field, or a narrowed width yields a refusal or an unequal context
      * `[ ]`   Act: `description.to_fields(to_fields_params, &context)`, then `description.fields_to_value(from_fields_params, fields)` over the fields the first call returned
      * `[ ]`   Assert: the `described` of the second call's `Ok` arm equals the arranged context by `assert_eq!`
      * `[ ]`   Boundary: the public calls `IEncodingContract::to_fields` and `IEncodingContract::fields_to_value` on `DerivationContextDescription`, a public entry of the crate; the route `to_fields` then `fields_to_value`, each running `domain`'s accessors and constructors as themselves
      * `[ ]`   Mocked: nothing; `domain` is a pure value library with no outer-edge collaborator on this route

  * `[✅]`   `directionality`
    * `[✅]`   `derivation_context` depends on the `factory` module's surface through `crate::factory::provides` and on `domain`; the `factory` module depends on nothing in the crate and names no description; among repository crates the crate depends on `crates/domain` alone, inward; nothing depends on the crate yet; no cycle
    * `[✅]`   `encoding/abi` consumes the contract and this description; `kdf/blake3_keyed` consumes the encoding of this description's fields through the encoding factory

  * `[ ]`   `requirements`
    * `[✅]`   `IEncodingContract::FromFieldsErrorReturn` is bounded by `Debug`, `PartialEq`, and `Eq`, so a description whose refusal lacks them fails to compile
    * `[✅]`   `adapters/encoding/Cargo.toml` carries exactly the tables and keys stated above, and no external crate is named in the crate
    * `[✅]`   `cargo check --workspace --all-targets --all-features`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo fmt --all --check`, and `cargo deny check` complete without error or warning
    * `[✅]`   `try_new_returns_the_description`, `to_fields_lists_the_reference_context_fields_in_canonical_order`, and `fields_to_value_returns_the_context_the_reference_fields_describe` pass (CR-11, the context's serialization frozen as one field sequence)
    * `[✅]`   `fields_to_value_rejects_a_field_list_one_short`, `fields_to_value_rejects_a_field_list_one_long`, `fields_to_value_checks_the_count_before_any_kind`, `fields_to_value_rejects_an_asset_name_of_the_wrong_kind`, `fields_to_value_rejects_an_asset_version_of_the_wrong_kind`, `fields_to_value_rejects_a_deployment_identity_of_the_wrong_kind`, `fields_to_value_rejects_a_suite_identifier_of_the_wrong_kind`, `fields_to_value_rejects_a_suite_version_of_the_wrong_kind`, `fields_to_value_rejects_a_parameter_set_identifier_of_the_wrong_kind`, `fields_to_value_rejects_a_group_index_of_the_wrong_kind`, `fields_to_value_rejects_a_piece_size_of_the_wrong_kind`, `fields_to_value_rejects_a_piece_group_size_of_the_wrong_kind`, `fields_to_value_rejects_a_total_extent_of_the_wrong_kind`, `fields_to_value_reports_the_lowest_field_of_the_wrong_kind`, and `fields_to_value_checks_every_kind_before_constructing_any_component` pass
    * `[✅]`   `fields_to_value_returns_the_asset_identity_refusal_unchanged`, `fields_to_value_returns_the_deployment_identity_refusal_unchanged`, `fields_to_value_returns_the_suite_identifier_refusal_unchanged`, `fields_to_value_returns_the_parameter_set_identifier_refusal_unchanged`, `fields_to_value_returns_the_piece_geometry_refusal_unchanged`, `fields_to_value_returns_the_derivation_context_refusal_unchanged`, and `fields_to_value_constructs_the_components_in_field_order` pass
    * `[ ]`   `a_context_survives_the_round_trip_through_its_canonical_fields` passes
    * `[✅]`   Nothing in `crates/domain` names `encoding`, and code outside `crates/domain` still cannot read any field of a domain type

* `[ ]`   `encoding/abi` **Ethereum ABI concrete encoding and decoding any described domain type as the ABI parameter encoding of its canonical fields, byte strings and width kinds included; authors the encoding family's encoder and decoder interfaces, the versioned encoding identifier, the declaration, and their mocks, and adds the byte-string, fixed twenty-byte, and 256-bit unsigned kinds to the encoding contract**

  * `[✅]`   `objective`
    * `[✅]`   Problem: the contracts recompute derivations, identity mappings, and challenges from ABI-encoded values, and the client hashes, signs, stores, and frames the same values, so one encoding serves both sides and turns a description's canonical fields into bytes and untrusted bytes back into an admitted domain value, with exactly one byte form per value (CR-11, serialization frozen with known-answer vectors; CR-09 statement binding)
    * `[✅]`   Functional: the family's encoder interface takes a description and a described value and returns its bytes; the family's decoder interface takes a description and untrusted bytes and returns the admitted described value or the refusal that names what failed
    * `[✅]`   Functional: both interfaces sit under one versioned encoding identifier, and every concrete declares that identifier, its adapter version, and the interface version it implements, readable before any instance exists
    * `[✅]`   Functional: the ABI concrete encodes a described value as Solidity's `abi.encode` over its canonical fields in order, a 32-byte fixed string as `bytes32`, a twenty-byte fixed string as `bytes20`, an unsigned integer of 16, 32, 64, or 256 bits as `uint16`, `uint32`, `uint64`, or `uint256`, text as `string`, and a byte string as `bytes`
    * `[✅]`   Functional: the encoding contract carries a byte string of any length, which holds a pairing group element's precompile encoding, so a statement over group elements, a proof of possession's or a delivery proof's, passes through the one encoding the contract recomputes it from (CR-09; CR-10; LC-08)
    * `[✅]`   Functional: a described value holding a byte string before a 32-byte fixed string encodes to the known-answer vector authored from the ABI specification, the byte string's offset in the head and its length and padded bytes in the tail, and the vector decodes to that value
    * `[✅]`   Functional: the encoding contract carries a fixed twenty-byte string and a 256-bit unsigned integer, each named as a width, so a transcript's parties and entitlements in the forms the entitlement-state adapter declares pass through the one encoding the contract recomputes them from, the verifier contract encoding to match this concrete's mapping (CR-09)
    * `[✅]`   Functional: a described value holding a twenty-byte fixed string before a 256-bit unsigned integer encodes to the known-answer vector authored from the ABI specification, the twenty bytes left-aligned in their word and the integer as its big-endian word, and the vector decodes to that value; nonzero padding after the twenty bytes is refused
    * `[✅]`   Functional: the ABI concrete decodes only the one canonical byte form: truncated input, bytes past the canonical end, nonzero padding, a field list of the wrong length, and an integer word wider than its field are refused, and a value its description refuses is returned with the description's refusal unchanged
    * `[✅]`   Functional: the reference derivation context encodes to the known-answer vector authored from the ABI specification, and the vector decodes to the reference context
    * `[✅]`   Non-functional: `alloy` is named only inside `adapters/encoding/src/abi`; the family's interfaces name no vendor and no format

  * `[✅]`   `role`
    * `[✅]`   Adds the byte-string, fixed twenty-byte, and 256-bit unsigned kinds to the encoding contract `encoding/derivation_context` authored, since this concrete is the first that encodes and decodes each; the derivation context's description and its field kinds are unchanged
    * `[✅]`   Adapter: the encoding family's first concrete, and the first source file that requires the family's encoder and decoder interfaces, the versioned encoding identifier, the declaration, and their mocks, which it authors in the family's `factory` module as its producers beside the encoding contract `encoding/derivation_context` authored
    * `[✅]`   Adds to `factory/interface.rs` and `factory/mock.rs`; `factory/mod.rs` and `factory/provides.rs` are unchanged; the factory function, its types, its unit test, and the family's integration test are `encoding/factory`'s
    * `[✅]`   Does not describe any domain type or check any domain invariant; the description supplies the fields and admits the value
    * `[✅]`   Does not emit the Solidity mirror of the vectors; `harness-crypto/generate/evm` mirrors them
    * `[✅]`   Does not map the encoding identifier to the hash-card's `encodingId` byte; the hash-card's own encoding does that
    * `[✅]`   Does not carry a commit

  * `[✅]`   `module`
    * `[✅]`   Bounded context, in the contract: the byte-string, fixed twenty-byte, and 256-bit unsigned variants of `CanonicalFieldKind` and of `CanonicalFieldValue` in `factory/interface.rs`
    * `[✅]`   Bounded context: the `factory` module's encoder and decoder traits with their params, success, error, and return types, the encoding identifier, the declaration, the interface version, and their mocks; and the private `abi` concrete, holding the adapter over `alloy`'s dynamic ABI, its constructor params and return, its decoding error, and its mock
    * `[✅]`   Adds the `abi` module to the existing crate at `adapters/encoding`; the crate's manifest gains `alloy` and the `hex` dev-dependency, and its barrel gains the module's line
    * `[✅]`   Outside: every description and domain type, the factory's selection of a concrete, every derivation, and the Solidity mirror

  * `[ ]`   `deps`
    * `[✅]`   The `factory` module's contract, same crate: `IEncodingContract`, `CanonicalFieldKind`, `CanonicalFieldValue`, `CanonicalFields`, `ToFieldsParams`, `FromFieldsParams`, and `FromFieldsSuccessReturn`, as `encoding/derivation_context` authors them
    * `[ ]`   The `factory` module's mock, same crate, through `crate::factory::provides`, in `abi/test.rs` and `abi/integration_test.rs` only: `MockIEncodingContract`, the description every unit test passes, `build_canonical_fields` with `CanonicalFieldsOverrides`, `build_encode_params`, and `build_decode_params`
    * `[ ]`   `derivation_context`, same crate, through `crate::derivation_context::provides`, in `abi/integration_test.rs` only: `DerivationContextDescription` through `build_derivation_context_description` with `DerivationContextDescriptionConstructorParamsOverrides`, and `DerivationContextFromFieldsErrorReturn`, the refusal the real description returns
    * `[ ]`   `domain` with its `mocks` feature, the dev-dependency `encoding/derivation_context` declares, in `abi/integration_test.rs` only: the component builders and their constructor-params overrides that build the reference context
    * `[✅]`   `alloy` `2.5.0`, external crate, MIT OR Apache-2.0, runtime dependency with default features off and the features `std` and `dyn-abi`, named only in `abi`; supplies `alloy::dyn_abi::{DynSolType, DynSolValue, Error}` and `alloy::primitives::{B256, U256}`; the dynamic ABI's decoder does not reject trailing bytes, nonzero padding, or integer words wider than their type, so the concrete enforces the canonical form itself
    * `[✅]`   `hex` `0.4.3`, external crate, MIT OR Apache-2.0, dev-dependency only; supplies `hex::decode` for the vectors
    * `[✅]`   `core::convert::Infallible`, standard library, the error arm of every operation with no failure; `TryFrom<&[u8]> for [u8; 32]`, standard library
    * `[✅]`   No reverse dependency beyond the family form's recorded cycle: the `factory` module's decoder error carries this concrete's error
    * `[✅]`   Nothing depends on the crate yet

  * `[ ]`   `context_slice`
    * `[✅]`   From `alloy::dyn_abi`, for the byte-string kind: `DynSolType::Bytes`, `DynSolValue::Bytes(Vec<u8>)`, and `DynSolValue::as_bytes(&self) -> Option<&[u8]>`
    * `[✅]`   From `alloy::dyn_abi`, for the width kinds: `DynSolType::FixedBytes(20)` and `DynSolType::Uint(256)`; `DynSolValue::FixedBytes(B256, 20)`, the twenty bytes left-aligned in the word, and `DynSolValue::Uint(U256, 256)`; `as_fixed_bytes` returning the full word with size `20`, and `as_uint` with width `256`
    * `[✅]`   From `alloy::primitives`, for the width kinds: `B256::right_padding_from(&[u8])`, which places the twenty bytes at the front of a zero word; `U256::from_be_bytes([u8; 32])`; and `U256::to_be_bytes::<32>(&self) -> [u8; 32]`
    * `[✅]`   From the contract: `IEncodingContract::FIELDS`, `to_fields(&self, ToFieldsParams, &Self::Described) -> Result<ToFieldsSuccessReturn, Infallible>`, and `fields_to_value(&self, FromFieldsParams, CanonicalFields) -> Result<FromFieldsSuccessReturn<Self::Described>, Self::FromFieldsErrorReturn>`
    * `[✅]`   From `alloy::dyn_abi`: `DynSolType::FixedBytes(usize)`, `DynSolType::Uint(usize)`, `DynSolType::String`, and `DynSolType::Tuple(Vec<DynSolType>)`; `DynSolType::abi_decode_params(&self, data: &[u8]) -> Result<DynSolValue, Error>`, which decodes a tuple as a parameter sequence; `DynSolValue::FixedBytes(B256, usize)`, `DynSolValue::Uint(U256, usize)`, `DynSolValue::String(String)`, and `DynSolValue::Tuple(Vec<DynSolValue>)`; `DynSolValue::abi_encode_params(&self) -> Vec<u8>`, which encodes a tuple as a parameter sequence, as `abi.encode` does; `DynSolValue::as_tuple(&self) -> Option<&[DynSolValue]>`, `as_fixed_bytes(&self) -> Option<(&[u8], usize)>`, `as_uint(&self) -> Option<(U256, usize)>`, and `as_str(&self) -> Option<&str>`; `Error`, which implements `Debug`
    * `[✅]`   From `alloy::primitives`: `B256::from([u8; 32])`, `U256::from(u16)`, `U256::from(u32)`, `U256::from(u64)`, and `u16::try_from(U256)`, `u32::try_from(U256)`, and `u64::try_from(U256)`, each failing when the value exceeds the target's width
    * `[ ]`   From the contract's mock, in `abi/test.rs` and `abi/integration_test.rs` only: `MockIEncodingContract`, `build_canonical_fields(CanonicalFieldsOverrides) -> CanonicalFields`, `build_encode_params(&D) -> EncodeParams<'_, D>`, and `build_decode_params(&D) -> DecodeParams<'_, D>`
    * `[ ]`   From `derivation_context`'s mock and `domain`'s mocks, in `abi/integration_test.rs` only: `build_derivation_context_description`, `build_derivation_context` with `DerivationContextConstructorParamsOverrides`, `build_asset_identity`, `build_deployment_identity`, `build_suite_identifier`, `build_parameter_set_identifier`, `build_group_index`, and `build_piece_geometry`, each with its constructor-params overrides

  * `[✅]`   `adapters/encoding/Cargo.toml`
    * `[✅]`   `[dependencies]` reads `domain = { path = "../../crates/domain" }` and `alloy = { version = "2.5.0", default-features = false, features = ["std", "dyn-abi"] }`
    * `[✅]`   `[dev-dependencies]` reads `domain = { path = "../../crates/domain", features = ["mocks"] }` and `hex = "0.4.3"`
    * `[✅]`   `[package]`, `[features]`, and `[lints]` are unchanged; no other table

  * `[✅]`   `adapters/encoding/src/lib.rs`
    * `[✅]`   The crate barrel reads `mod abi;`, `mod derivation_context;`, `mod factory;`, `pub use derivation_context::provides::*;`, and `pub use factory::provides::*;`, nothing else; the `abi` concrete's surface is not re-exported
    * `[✅]`   Until `abi/mod.rs` exists, `cargo check` reports the unresolved `mod abi`, which is the RED state for every element below that precedes it

  * `[✅]`   `adapters/encoding/src/factory/interface.rs`
    * `[✅]`   `CanonicalFieldKind` gains the variants `Bytes`, `FixedBytes20`, and `Unsigned256`, and `CanonicalFieldValue` gains the variants `Bytes(Vec<u8>)`, a byte string of any length, `FixedBytes20([u8; 20])`, and `Unsigned256([u8; 32])`, the integer's big-endian bytes, each placed after `Text` in that order; every other variant and every derive is unchanged
    * `[✅]`   `ENCODING_INTERFACE_VERSION`, a `pub const` of type `u32` with value `1`
    * `[✅]`   `EncodingIdentifier`, an enum with the one variant `EthereumAbiV1`, the versioned identifier both interfaces sit under and a deployment's hash-card names
    * `[✅]`   `EncodingDeclaration`, a struct with `pub identifier: EncodingIdentifier`, `pub adapter_version: u32`, and `pub interface_version: u32`
    * `[✅]`   `EncodeParams<'a, D>`, a struct with `pub description: &'a D`, the description that selects the field sequence; `EncodeSuccessReturn`, a struct with `pub bytes: Vec<u8>`; `EncodeErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]` and the variants `FieldCount { expected: usize, actual: usize }` and `FieldKind { index: usize, expected: CanonicalFieldKind, actual: CanonicalFieldKind }`, describing a disagreement between `to_fields` and `D::FIELDS`; `EncodeReturn`, the alias `Result<EncodeSuccessReturn, EncodeErrorReturn>`; the byte vector is the format boundary, not a claim that it is a typed domain value
    * `[✅]`   `IEncoderAdapter`, a trait with `const DECLARATION: EncodingDeclaration;` and `fn encode<D: IEncodingContract>(&self, params: EncodeParams<'_, D>, payload: &D::Described) -> EncodeReturn;`; each concrete's associated constant refers to its one inherent `DECLARATION`
    * `[✅]`   `DecodeParams<'a, D>`, a struct with `pub description: &'a D`; `DecodeSuccessReturn<T>`, a struct with `pub described: T`; `DecodeErrorReturn<E>`, an enum with `#[derive(Debug, PartialEq)]`, omitting `Eq` because `AbiDecoderErrorReturn` carries the vendor's `alloy::dyn_abi::Error`, which implements `PartialEq` and not `Eq`, and the variants `Abi(AbiDecoderErrorReturn)`, the ABI concrete's refusal carried unchanged, `Description(E)`, the description's refusal carried unchanged, and `EncoderContract(EncodeErrorReturn)`, a schema disagreement found when canonical re-encoding calls `encode`; each further concrete's refusal is its own variant; `DecodeReturn<T, E>`, the alias `Result<DecodeSuccessReturn<T>, DecodeErrorReturn<E>>`
    * `[✅]`   `IDecoderAdapter`, a trait with the one method `fn decode<D: IEncodingContract>(&self, params: DecodeParams<'_, D>, payload: &[u8]) -> DecodeReturn<D::Described, D::FromFieldsErrorReturn>;`, the validating form, its payload the untrusted bytes and its narrowing target `D::Described`
    * `[ ]`   Under `#[cfg(any(test, feature = "mocks"))]`, `EncodingIdentifier` gains the variant `Mock`, the identifier of the family mock concretes, `DecodeErrorReturn<E>` gains the variant `Mock(MockIDecoderAdapterErrorReturn)`, and the file declares `MockIDecoderAdapterErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]` and the variants `Truncated { index: usize }`, `InvalidText { index: usize }`, and `TrailingBytes`, the mock decoder's refusals
    * `[✅]`   Adds the import of `AbiDecoderErrorReturn` from `crate::abi::provides`; every item `encoding/derivation_context` authored in this file is unchanged; names no vendor

  * `[✅]`   `adapters/encoding/src/abi/interface.rs`
    * `[✅]`   `AbiEncoding`, the unit struct `pub struct AbiEncoding;`, the adapter over `alloy`'s dynamic ABI, implementing both the encoder and the decoder interface
    * `[✅]`   `AbiEncodingConstructorParams`, the fieldless struct `pub struct AbiEncodingConstructorParams;`, the constructor's deps slot
    * `[✅]`   `AbiEncodingTryNewReturn`, the alias `Result<AbiEncoding, Infallible>`; the error arm is uninhabited because the adapter takes no configuration
    * `[✅]`   `AbiDecoderErrorReturn`, an enum with `#[derive(Debug, PartialEq)]`, omitting `Eq` because the vendor's `alloy::dyn_abi::Error` implements `PartialEq` and not `Eq`, and the variants `Malformed(alloy::dyn_abi::Error)`, the ABI decoder's refusal carried unchanged; `DecodedShapeMismatch { index: usize }`, the decoded value holding no item of the described kind at `index`; `ValueOutOfRange { index: usize, kind: CanonicalFieldKind }`, an integer word at `index` wider than its field; and `NonCanonical`, the decoded value re-encoding to bytes other than the input
    * `[✅]`   Imports `CanonicalFieldKind` from `crate::factory::provides` and `core::convert::Infallible`; names `alloy::dyn_abi::Error` by its full path; declares nothing else

  * `[✅]`   `adapters/encoding/src/abi/interaction.spec.md`
    * `[✅]`   `AbiEncoding::try_new(params: AbiEncodingConstructorParams) -> AbiEncodingTryNewReturn`: one branch; outcome `Ok(AbiEncoding)`; the error arm has no branch
    * `[✅]`   `AbiEncoding::DECLARATION`: the inherent constant `EncodingDeclaration { identifier: EncodingIdentifier::EthereumAbiV1, adapter_version: 1, interface_version: ENCODING_INTERFACE_VERSION }`
    * `[✅]`   `encode`, wrong field count: call `params.description.to_fields(ToFieldsParams, payload)` once and unpack its infallible result; if the emitted count differs from `D::FIELDS.len()`, return `Err(EncodeErrorReturn::FieldCount { expected, actual })` before ABI encoding
    * `[✅]`   `encode`, wrong field kind: after the count passes, compare each emitted value's variant with the corresponding `D::FIELDS` kind in index order; return `Err(EncodeErrorReturn::FieldKind { index, expected, actual })` for the lowest mismatch, before ABI encoding
    * `[✅]`   `encode`, admitted fields: condition every emitted value matches `D::FIELDS` in count and kind; dependency call `DynSolValue::abi_encode_params` once; each canonical value maps in order, `FixedBytes32(bytes)` to `DynSolValue::FixedBytes(B256::from(bytes), 32)`, `Unsigned16(value)`, `Unsigned32(value)`, and `Unsigned64(value)` to `DynSolValue::Uint(U256::from(value), 16)`, `32`, and `64`, `Text(text)` to `DynSolValue::String(text)`, `Bytes(bytes)` to `DynSolValue::Bytes(bytes)`, `FixedBytes20(bytes)` to `DynSolValue::FixedBytes(B256::right_padding_from(&bytes), 20)`, and `Unsigned256(bytes)` to `DynSolValue::Uint(U256::from_be_bytes(bytes), 256)`, collected into `DynSolValue::Tuple`; outcome `Ok(EncodeSuccessReturn { bytes })` holding the parameter encoding
    * `[✅]`   `decode`, malformed: condition `DynSolType::Tuple` over the description's `FIELDS`, each kind mapped to `DynSolType::FixedBytes(32)`, `DynSolType::Uint(16)`, `DynSolType::Uint(32)`, `DynSolType::Uint(64)`, `DynSolType::String`, `DynSolType::Bytes`, `DynSolType::FixedBytes(20)`, or `DynSolType::Uint(256)`, refuses the payload through `abi_decode_params`; decision the decoder's result; dependency call `abi_decode_params` once; outcome `Err(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::Malformed(error)))`, the error unchanged
    * `[✅]`   `decode`, shape mismatch: condition the decoded value is not a tuple, or holds at some index of `FIELDS` no item or an item not of the kind named there, a `bytes32` item being `as_fixed_bytes` of size `32` converting into `[u8; 32]`, a `bytes20` item `as_fixed_bytes` of size `20` whose word's first twenty bytes convert into `[u8; 20]`, an integer item `as_uint` of the field's width, a `uint256` item's word taken as its big-endian `[u8; 32]`, a text item `as_str`, and a byte-string item `as_bytes`, copied into a `Vec<u8>` as `CanonicalFieldValue::Bytes`; decision the fields are read in ascending index order; dependency call none; outcome `Err(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::DecodedShapeMismatch { index }))` for the lowest such index, `0` when the value is not a tuple; the decoder returns the type it was given, so no input takes this branch and it has no unit test
    * `[✅]`   `decode`, integer out of range: condition an integer item's value does not convert into the field's `u16`, `u32`, or `u64` through `try_from`; decision the conversion, in the same ascending pass; dependency call none; outcome `Err(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::ValueOutOfRange { index, kind }))` for the lowest such index, `kind` the field's kind; a 256-bit field never takes this branch, since every word fits it
    * `[✅]`   `decode`, description refused: condition every item converts; decision the description's result; dependency call `params.description.fields_to_value(FromFieldsParams, CanonicalFields { values })` once over the converted values in field order; outcome `Err(DecodeErrorReturn::Description(error))`, the refusal unchanged
    * `[✅]`   `decode`, encoder contract refused: condition the description admits the value and re-encoding returns `Err(error)`; dependency call `self.encode(EncodeParams { description: params.description }, &described)` once; outcome `Err(DecodeErrorReturn::EncoderContract(error))`
    * `[✅]`   `decode`, non-canonical: condition the description admits the value and re-encoding succeeds with bytes other than the payload; decision byte equality; dependency call `encode` once; outcome `Err(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::NonCanonical))`; this refuses trailing bytes, nonzero padding, non-canonical offsets, and a head of the wrong length, which the dynamic decoder reads past
    * `[✅]`   `decode`, admitted: condition the re-encoding equals the payload; outcome `Ok(DecodeSuccessReturn { described })`
    * `[✅]`   Ordering: the ABI decode precedes the item pass, the item pass precedes the description, and the re-encoding comparison runs last; `params` supplies the description and nothing else is read from it
    * `[ ]`   Callee disposition, `encoding/derivation_context` entry round trip through the canonical fields: absorbed, proven by `a_context_survives_the_abi_round_trip` in the private integration test element `abi/integration_test.rs`, whose context carries the entry's variation and whose fields pass through the ABI byte form between `to_fields` and `fields_to_value`
    * `[ ]`   Own entry, reference context encodes to the reference vector: condition the reference derivation context is handed to `encode` with `DerivationContextDescription`; outcome `Ok(EncodeSuccessReturn { bytes })` with `bytes` the known-answer vector authored from the ABI specification for the reference fields; variation a context whose text fields are dynamic items between static items and whose same-kind fields differ, so a swapped field, a wrong word width, or a misplaced tail changes the bytes; edge that must survive the composition: the description's emitted order and kinds are the order and kinds `FIELDS` declares, so the encoder's check admits them
    * `[ ]`   Own entry, reference vector decodes to the reference context: condition the reference vector is handed to `decode` with `DerivationContextDescription`; outcome `Ok(DecodeSuccessReturn { described })` with `described` equal to the reference context; variation the same context, so a field read from the wrong word or tail yields a refusal or an unequal context; edge that must survive the composition: the canonical re-encoding of the described context equals the vector, so the one admitted byte form is the byte form `encode` writes
    * `[ ]`   Own entry, a context survives the ABI round trip: condition an admitted `DerivationContext` is handed to `encode` and the bytes it returns are handed to `decode`; outcome `Ok(DecodeSuccessReturn { described })` with `described` equal to the original context; variation a context whose same-kind fields hold distinct values, whose 16-bit suite version is at its maximum, whose piece-group size exceeds the `i32` range, and whose group index and total extent exceed the 32-bit range, so a swap of same-kind fields, a dropped field, or a narrowed width yields a refusal or an unequal context; edge that must survive the composition: the rebuilt context is admitted at the full width of the extent, and the canonical comparison accepts every byte string `encode` writes
    * `[ ]`   Own entry, a value the description refuses is refused with the description's refusal: condition a canonical ABI encoding whose deployment-identity word is all zero is handed to `decode` with `DerivationContextDescription`; outcome `Err(DecodeErrorReturn::Description(DerivationContextFromFieldsErrorReturn::DeploymentIdentity(DeploymentIdentityTryNewErrorReturn::AllZero)))`; variation an encoding that is canonical and well-kinded, so the refusal comes from the domain's constructor through the description and not from the ABI decode or the canonical comparison; edge that must survive the composition: the refusal passes through the decoder unchanged and whole
    * `[ ]`   Private surface: the chain of real functions `AbiEncoding::encode` and `AbiEncoding::decode`, `DerivationContextDescription::to_fields` and `fields_to_value`, and `domain`'s accessors and constructors; no outer-edge collaborator is mocked, `alloy` running as the concrete's own vendor library; the observable result is the bytes `encode` returns and the context or the refusal `decode` returns

  * `[✅]`   `adapters/encoding/src/factory/mock.rs`
    * `[ ]`   Module-level `#![allow(clippy::expect_used)]`, for the 32-bit length prefix the mock concretes write
    * `[ ]`   `build_canonical_fields`' default `values` become, in order, `Text("left-pad")`, `Bytes` of the bytes `0xa1`, `0xa2`, and `0xa3`, `FixedBytes32` of bytes each `0x11`, `Unsigned16(0x0102)`, `Unsigned32(0x01020304)`, `Unsigned64(0x0102030405060708)`, `FixedBytes20` of bytes each `0x22`, and `Unsigned256` of a first byte `0x80` and every other byte `0x00`, one value of each kind
    * `[ ]`   `MockIEncodingContract` changes: `const FIELDS` holds `Text`, `Bytes`, `FixedBytes32`, `Unsigned16`, `Unsigned32`, `Unsigned64`, `FixedBytes20`, and `Unsigned256`, the kinds of those default `values` in order; `to_fields` returns `Ok(ToFieldsSuccessReturn { fields })` with `fields` a clone of its payload, and `fields_to_value` returns `Ok(FromFieldsSuccessReturn { described })` with `described` its payload, each computed from its input, so a test reaches the field list the description emits or admits by the payload it passes
    * `[ ]`   `EncodingDeclarationOverrides`, `#[derive(Default)]`, the fields `pub identifier: Option<EncodingIdentifier>`, `pub adapter_version: Option<u32>`, and `pub interface_version: Option<u32>`, and `build_encoding_declaration(overrides: EncodingDeclarationOverrides) -> EncodingDeclaration`, the omitted fields defaulting to `EncodingIdentifier::Mock`, `1`, and `ENCODING_INTERFACE_VERSION`
    * `[ ]`   `EncodeSuccessReturnOverrides`, `#[derive(Default)]`, the one field `pub bytes: Option<Vec<u8>>`, and `build_encode_success_return(overrides: EncodeSuccessReturnOverrides) -> EncodeSuccessReturn`, the omitted `bytes` defaulting to `0x01`, `0x02`, and `0x03`
    * `[ ]`   `build_encode_params(description: &D) -> EncodeParams<'_, D>`, `build_decode_params(description: &D) -> DecodeParams<'_, D>`, and `build_decode_success_return(described: T) -> DecodeSuccessReturn<T>`, each generic over `D: IEncodingContract` or over `T` and taking its one property as the argument, since a generic property has no default and the type has no other property
    * `[ ]`   `MockIEncoderAdapter`, the unit struct `pub(crate) struct MockIEncoderAdapter;` implementing `IEncoderAdapter` with `const DECLARATION` equal to `EncodingDeclaration { identifier: EncodingIdentifier::Mock, adapter_version: 1, interface_version: ENCODING_INTERFACE_VERSION }`; `encode` calls `params.description.to_fields(ToFieldsParams, payload)` once and unpacks its infallible result, returns `Err(EncodeErrorReturn::FieldCount { expected, actual })` when the emitted count differs from `D::FIELDS.len()` and `Err(EncodeErrorReturn::FieldKind { index, expected, actual })` for the lowest index whose variant differs from the `D::FIELDS` kind, and otherwise returns `Ok(EncodeSuccessReturn { bytes })` with each value written in order, `FixedBytes32`, `FixedBytes20`, and `Unsigned256` as their bytes, `Unsigned16`, `Unsigned32`, and `Unsigned64` as the big-endian bytes of their own width, and `Text` and `Bytes` as a 32-bit big-endian length followed by their bytes
    * `[ ]`   `MockIDecoderAdapter`, the unit struct `pub(crate) struct MockIDecoderAdapter;` implementing `IDecoderAdapter` with the same `DECLARATION`; `decode` reads one value per `D::FIELDS` kind in order from the payload in the byte form `MockIEncoderAdapter` writes, returns `Err(DecodeErrorReturn::Mock(MockIDecoderAdapterErrorReturn::Truncated { index }))` when the payload ends inside the field at `index`, `InvalidText { index }` when a `Text` field's bytes are not UTF-8, and `TrailingBytes` when bytes remain after the last field, then calls `params.description.fields_to_value(FromFieldsParams, CanonicalFields { values })` once and returns `Err(DecodeErrorReturn::Description(error))` with the refusal unchanged, and otherwise `Ok(DecodeSuccessReturn { described })`
    * `[ ]`   No failure mode: every arm each mock concrete states is reached by an input, a field list the description emits or admits, a payload, or a refusal the description returns
    * `[ ]`   No corruptions type and no invalidator for any owned object type: the crate has no serialization dependency, and a payload the decoder refuses is a byte vector a test derives from the known-answer vector; no mock for `CanonicalFieldKind`, `CanonicalFieldValue`, `EncodingIdentifier`, `EncodeErrorReturn`, `DecodeErrorReturn`, or `MockIDecoderAdapterErrorReturn`, enums used by their production variants, for `ENCODING_INTERFACE_VERSION`, a constant used by its production value, or for `EncodeReturn` and `DecodeReturn`, aliases of `Result`
    * `[ ]`   Imports the added names from `super::interface`

  * `[ ]`   `adapters/encoding/src/abi/mock.rs`
    * `[ ]`   `AbiEncodingConstructorParamsOverrides`, the fieldless `#[derive(Default)]` struct `pub struct AbiEncodingConstructorParamsOverrides;`
    * `[ ]`   `build_abi_encoding_constructor_params(overrides: AbiEncodingConstructorParamsOverrides) -> AbiEncodingConstructorParams`
    * `[ ]`   `build_abi_encoding(overrides: AbiEncodingConstructorParamsOverrides) -> AbiEncoding`, returning the real instance from `AbiEncoding::try_new(build_abi_encoding_constructor_params(overrides))`, whose error arm is uninhabited and is unpacked irrefutably
    * `[ ]`   No corruptions type and no invalidator: the constructor params carry no field and the crate has no serialization dependency; no `AbiEncoding` overrides, invalidator, or mock function, since the type is built as a real instance and the trait methods it implements are the subjects of its tests; no mock for `AbiDecoderErrorReturn`, an enum used by its production variants, or for `AbiEncodingTryNewReturn`, an alias of `Result`
    * `[ ]`   Imports `AbiEncoding` and `AbiEncodingConstructorParams` from `super::interface`

  * `[ ]`   `adapters/encoding/src/abi/test.rs`
    * `[ ]`   Collaborators: `alloy`'s dynamic ABI is the concrete's vendor library and runs as itself; the description is the official mock `MockIEncodingContract`, whose `to_fields` and `fields_to_value` return their payloads, so a test reaches the field list the description emits or admits by the payload it passes; every subject is built with `build_abi_encoding`, and every params value with `build_encode_params` or `build_decode_params` over `MockIEncodingContract`
    * `[ ]`   Reference values by index of `MockIEncodingContract::FIELDS`: `Text("left-pad")`, `Bytes` of the bytes `0xa1`, `0xa2`, and `0xa3`, `FixedBytes32` of bytes each `0x11`, `Unsigned16(0x0102)`, `Unsigned32(0x01020304)`, `Unsigned64(0x0102030405060708)`, `FixedBytes20` of bytes each `0x22`, and `Unsigned256` of a first byte `0x80` and every other byte `0x00`
    * `[ ]`   Reference fields: `build_canonical_fields` at its defaults, which hold the reference values
    * `[ ]`   Reference vector: the `abi.encode` parameter encoding of the reference values, authored by hand from the Solidity Contract ABI Specification, one head word per field with the `Text` and `Bytes` offsets in their head words and each length and zero-padded bytes in the tail, and written as a hex string for `hex::decode`; it is not produced by running `alloy` or this crate
    * `[ ]`   `try_new_returns_the_adapter`
      * `[ ]`   Contract: `AbiEncoding::try_new(params)`, one branch: any params → `Ok(AbiEncoding)`; the error arm has no branch
      * `[ ]`   Arrange: the params from `build_abi_encoding_constructor_params`
      * `[ ]`   Act: `AbiEncoding::try_new(params)`
      * `[ ]`   Assert: `result.is_ok()` is true
    * `[ ]`   `encode_writes_every_kind_as_its_abi_parameter_encoding`
      * `[ ]`   Contract: `encode`, admitted fields: every emitted value matches `D::FIELDS` in count and kind → `Ok(EncodeSuccessReturn { bytes })` holding the parameter encoding, each canonical value mapped in order to its ABI type
      * `[ ]`   Arrange: the reference fields as the payload; the values are distinct in every kind, a byte string and a text precede the static items, and the 20-byte string precedes the 256-bit integer, so a wrong map, a swapped word, a misplaced tail, or a misaligned `bytes20` changes the bytes
      * `[ ]`   Act: `adapter.encode(params, &fields)`
      * `[ ]`   Assert: `result.ok().map(|success| success.bytes)` equals `Some` of the reference vector's bytes by `assert_eq!`, the expectation authored from the ABI specification and not read back from the arrangement
    * `[ ]`   `encode_rejects_a_description_emitting_one_value_short`
      * `[ ]`   Contract: `encode`, wrong field count: the emitted count differs from `D::FIELDS.len()` → `Err(EncodeErrorReturn::FieldCount { expected, actual })` before ABI encoding
      * `[ ]`   Arrange: `build_canonical_fields` overriding `values` with the reference values without index `7` as the payload, which the mock description emits as its fields, every value present of its declared kind, so only the count differs
      * `[ ]`   Act: `adapter.encode(params, &fields)`
      * `[ ]`   Assert: `result.err()` equals `Some(EncodeErrorReturn::FieldCount { expected: <MockIEncodingContract as IEncodingContract>::FIELDS.len(), actual: 7 })` by `assert_eq!`
    * `[ ]`   `encode_rejects_a_description_emitting_one_value_long`
      * `[ ]`   Contract: `encode`, wrong field count: the emitted count differs from `D::FIELDS.len()` → `Err(EncodeErrorReturn::FieldCount { expected, actual })` before ABI encoding
      * `[ ]`   Arrange: `build_canonical_fields` overriding `values` with the reference values followed by `Unsigned64(0)` as the payload, every value at indexes `0` through `7` of its declared kind, so only the count differs
      * `[ ]`   Act: `adapter.encode(params, &fields)`
      * `[ ]`   Assert: `result.err()` equals `Some(EncodeErrorReturn::FieldCount { expected: <MockIEncodingContract as IEncodingContract>::FIELDS.len(), actual: 9 })` by `assert_eq!`
    * `[ ]`   `encode_checks_the_count_before_any_kind`
      * `[ ]`   Contract: `encode`, ordering: the count check precedes the kind check
      * `[ ]`   Arrange: `build_canonical_fields` overriding `values` with the reference values without index `7` and with the value at index `0` replaced by `Unsigned16(1)`, a payload both short and holding a wrong kind, so a kind check ahead of the count reports the kind
      * `[ ]`   Act: `adapter.encode(params, &fields)`
      * `[ ]`   Assert: `result.err()` equals `Some(EncodeErrorReturn::FieldCount { expected: <MockIEncodingContract as IEncodingContract>::FIELDS.len(), actual: 7 })` by `assert_eq!`
    * `[ ]`   `encode_rejects_a_description_emitting_the_wrong_kind`
      * `[ ]`   Contract: `encode`, wrong field kind: the count matches and an emitted value's variant differs from the `D::FIELDS` kind at its index → `Err(EncodeErrorReturn::FieldKind { index, expected, actual })` before ABI encoding
      * `[ ]`   Arrange: `build_canonical_fields` overriding `values` with the reference values and the value at index `1` replaced by `Text("x")` as the payload, every other value of its declared kind, so only the kind at index `1` differs
      * `[ ]`   Act: `adapter.encode(params, &fields)`
      * `[ ]`   Assert: `result.err()` equals `Some(EncodeErrorReturn::FieldKind { index: 1, expected: CanonicalFieldKind::Bytes, actual: CanonicalFieldKind::Text })` by `assert_eq!`
    * `[ ]`   `encode_reports_the_lowest_field_of_the_wrong_kind`
      * `[ ]`   Contract: `encode`, wrong field kind: more than one emitted value differs from its `D::FIELDS` kind → `Err(EncodeErrorReturn::FieldKind { index, expected, actual })` for the lowest such index
      * `[ ]`   Arrange: `build_canonical_fields` overriding `values` with the reference values, the value at index `2` replaced by `Unsigned16(1)` and the value at index `6` replaced by `Unsigned256` of bytes each `0x00`, so a check that runs from the highest index reports index `6`
      * `[ ]`   Act: `adapter.encode(params, &fields)`
      * `[ ]`   Assert: `result.err()` equals `Some(EncodeErrorReturn::FieldKind { index: 2, expected: CanonicalFieldKind::FixedBytes32, actual: CanonicalFieldKind::Unsigned16 })` by `assert_eq!`
    * `[ ]`   `decode_reads_the_vector_as_its_value`
      * `[ ]`   Contract: `decode`, admitted: the re-encoding equals the payload → `Ok(DecodeSuccessReturn { described })`
      * `[ ]`   Arrange: the reference vector's bytes as the payload, a canonical encoding whose every kind is distinct, so a field read from the wrong word or tail changes the described value
      * `[ ]`   Act: `adapter.decode(params, &payload)`
      * `[ ]`   Assert: `result.ok().map(|success| success.described)` equals `Some` of the reference fields by `assert_eq!`
    * `[ ]`   `decode_rejects_input_truncated_to_the_head`
      * `[ ]`   Contract: `decode`, malformed: `DynSolType::Tuple` over the description's `FIELDS` refuses the payload through `abi_decode_params` → `Err(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::Malformed(error)))`, the error unchanged
      * `[ ]`   Arrange: the reference vector's bytes cut after the head, one word per field, so the `Text` and `Bytes` offsets point past the end of the payload
      * `[ ]`   Act: `adapter.decode(params, &payload)`
      * `[ ]`   Assert: `result.err()` equals `Some(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::Malformed(error)))` by `assert_eq!`, `error` the vendor's `Overrun` refusal
    * `[ ]`   `decode_rejects_trailing_bytes`
      * `[ ]`   Contract: `decode`, non-canonical: the description admits the value and re-encoding succeeds with bytes other than the payload → `Err(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::NonCanonical))`
      * `[ ]`   Arrange: the reference vector's bytes followed by one `0x00` byte, which the dynamic decoder reads past
      * `[ ]`   Act: `adapter.decode(params, &payload)`
      * `[ ]`   Assert: `result.err()` equals `Some(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::NonCanonical))` by `assert_eq!`
    * `[ ]`   `decode_rejects_nonzero_string_padding`
      * `[ ]`   Contract: `decode`, non-canonical: the description admits the value and re-encoding succeeds with bytes other than the payload → `Err(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::NonCanonical))`
      * `[ ]`   Arrange: the reference vector's bytes with the last padding byte of the `Text` tail set to `0x01`, which leaves the decoded text unchanged
      * `[ ]`   Act: `adapter.decode(params, &payload)`
      * `[ ]`   Assert: `result.err()` equals `Some(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::NonCanonical))` by `assert_eq!`
    * `[ ]`   `decode_rejects_nonzero_byte_string_padding`
      * `[ ]`   Contract: `decode`, non-canonical: the description admits the value and re-encoding succeeds with bytes other than the payload → `Err(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::NonCanonical))`
      * `[ ]`   Arrange: the reference vector's bytes with the last padding byte of the `Bytes` tail set to `0x01`, which leaves the decoded bytes unchanged
      * `[ ]`   Act: `adapter.decode(params, &payload)`
      * `[ ]`   Assert: `result.err()` equals `Some(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::NonCanonical))` by `assert_eq!`
    * `[ ]`   `decode_rejects_nonzero_fixed_bytes20_padding`
      * `[ ]`   Contract: `decode`, non-canonical: the description admits the value and re-encoding succeeds with bytes other than the payload → `Err(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::NonCanonical))`
      * `[ ]`   Arrange: the reference vector's bytes with the last byte of the `bytes20` head word set to `0x01`, which leaves the first twenty bytes of the word unchanged
      * `[ ]`   Act: `adapter.decode(params, &payload)`
      * `[ ]`   Assert: `result.err()` equals `Some(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::NonCanonical))` by `assert_eq!`
    * `[ ]`   `decode_rejects_a_non_canonical_offset`
      * `[ ]`   Contract: `decode`, non-canonical: the description admits the value and re-encoding succeeds with bytes other than the payload → `Err(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::NonCanonical))`
      * `[ ]`   Arrange: the reference vector's bytes with a zero word inserted between the head and the tails and the `Text` and `Bytes` offsets each increased by one word, a valid layout that is not the canonical one
      * `[ ]`   Act: `adapter.decode(params, &payload)`
      * `[ ]`   Assert: `result.err()` equals `Some(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::NonCanonical))` by `assert_eq!`
    * `[ ]`   `decode_rejects_the_encoding_of_one_field_too_many`
      * `[ ]`   Contract: `decode`, non-canonical: the description admits the value and re-encoding succeeds with bytes other than the payload → `Err(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::NonCanonical))`
      * `[ ]`   Arrange: the `abi.encode` parameter encoding of the reference values followed by an extra `Unsigned64(0)`, authored from the ABI specification, a head one word longer than the described tuple, which the dynamic decoder reads past
      * `[ ]`   Act: `adapter.decode(params, &payload)`
      * `[ ]`   Assert: `result.err()` equals `Some(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::NonCanonical))` by `assert_eq!`
    * `[ ]`   `decode_rejects_a_uint16_word_wider_than_sixteen_bits`
      * `[ ]`   Contract: `decode`, integer out of range: an integer item's value does not convert into the field's `u16` through `try_from` → `Err(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::ValueOutOfRange { index, kind }))`
      * `[ ]`   Arrange: the reference vector's bytes with the high-order byte of the `uint16` head word set to `0x01`, a word wider than 16 bits
      * `[ ]`   Act: `adapter.decode(params, &payload)`
      * `[ ]`   Assert: `result.err()` equals `Some(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::ValueOutOfRange { index: 3, kind: CanonicalFieldKind::Unsigned16 }))` by `assert_eq!`
    * `[ ]`   `decode_rejects_a_uint32_word_wider_than_thirty_two_bits`
      * `[ ]`   Contract: `decode`, integer out of range: an integer item's value does not convert into the field's `u32` through `try_from` → `Err(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::ValueOutOfRange { index, kind }))`
      * `[ ]`   Arrange: the reference vector's bytes with the high-order byte of the `uint32` head word set to `0x01`, a word wider than 32 bits
      * `[ ]`   Act: `adapter.decode(params, &payload)`
      * `[ ]`   Assert: `result.err()` equals `Some(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::ValueOutOfRange { index: 4, kind: CanonicalFieldKind::Unsigned32 }))` by `assert_eq!`
    * `[ ]`   `decode_rejects_a_uint64_word_wider_than_sixty_four_bits`
      * `[ ]`   Contract: `decode`, integer out of range: an integer item's value does not convert into the field's `u64` through `try_from` → `Err(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::ValueOutOfRange { index, kind }))`
      * `[ ]`   Arrange: the reference vector's bytes with the high-order byte of the `uint64` head word set to `0x01`, a word wider than 64 bits
      * `[ ]`   Act: `adapter.decode(params, &payload)`
      * `[ ]`   Assert: `result.err()` equals `Some(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::ValueOutOfRange { index: 5, kind: CanonicalFieldKind::Unsigned64 }))` by `assert_eq!`

  * `[✅]`   `construction`
    * `[✅]`   `AbiEncoding::try_new` is the concrete's only producer, and its only caller is the encoding factory, which reads `AbiEncoding::DECLARATION` before constructing and hands the adapter to consumers generic over `IEncoderAdapter` and `IDecoderAdapter`
    * `[ ]`   `build_abi_encoding` is the concrete's mock producer: it returns the real instance `try_new` constructs over the fieldless params `build_abi_encoding_constructor_params` builds, so a test reaches the adapter through the builder and never by naming the unit struct

  * `[✅]`   `adapters/encoding/src/abi/mod.rs`
    * `[✅]`   The `encode` map gains the arm `CanonicalFieldValue::Bytes(bytes) => DynSolValue::Bytes(bytes)`; the decode type map gains `CanonicalFieldKind::Bytes => DynSolType::Bytes`; the item pass gains the arm `CanonicalFieldKind::Bytes`, reading the item by `let Some(bytes) = item.as_bytes() else` into `DecodedShapeMismatch { index }` and yielding `CanonicalFieldValue::Bytes(bytes.to_vec())`; every other arm is unchanged
    * `[✅]`   The `encode` map gains the arms `CanonicalFieldValue::FixedBytes20(bytes) => DynSolValue::FixedBytes(B256::right_padding_from(&bytes), 20)` and `CanonicalFieldValue::Unsigned256(bytes) => DynSolValue::Uint(U256::from_be_bytes(bytes), 256)`; the decode type map gains `CanonicalFieldKind::FixedBytes20 => DynSolType::FixedBytes(20)` and `CanonicalFieldKind::Unsigned256 => DynSolType::Uint(256)`; the item pass gains the arm `CanonicalFieldKind::FixedBytes20`, reading the item by `let Some((bytes, size)) = item.as_fixed_bytes() else` into `DecodedShapeMismatch { index }`, refusing `size != 20` the same way, taking the word's first twenty bytes by `let Some(head) = bytes.get(..20) else` into the same refusal, and converting them by `<[u8; 20]>::try_from(head)` into `CanonicalFieldValue::FixedBytes20`, and the arm `CanonicalFieldKind::Unsigned256`, reading the item by `let Some((word, width)) = item.as_uint() else` into `DecodedShapeMismatch { index }`, refusing `width != 256` the same way, and yielding `CanonicalFieldValue::Unsigned256(word.to_be_bytes::<32>())`; every other arm is unchanged
    * `[✅]`   Module declarations: `mod interface;`, `#[cfg(test)] mod integration_test;`, `#[cfg(test)] mod mock;`, `pub(crate) mod provides;`, and `#[cfg(test)] mod test;`
    * `[✅]`   `impl AbiEncoding` with `pub const DECLARATION: EncodingDeclaration` and `pub fn try_new(_params: AbiEncodingConstructorParams) -> AbiEncodingTryNewReturn` returning `Ok(AbiEncoding)`
    * `[✅]`   `impl IEncoderAdapter for AbiEncoding` with `const DECLARATION: EncodingDeclaration = AbiEncoding::DECLARATION;`, and `impl IDecoderAdapter for AbiEncoding`, each method realizing the branches and ordering of the interaction spec; encoding checks emitted count and kinds against `D::FIELDS` before making any ABI value; the decoded item pass matches each `FIELDS` kind against its item with `let … else`, and the re-encoding calls this adapter's own `encode` and carries its schema refusal unchanged
    * `[✅]`   Imports the family's names from `crate::factory::provides`, `DynSolType` and `DynSolValue` from `alloy::dyn_abi`, `B256` and `U256` from `alloy::primitives`, and this module's types from `interface`
    * `[✅]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[ ]`   `adapters/encoding/src/abi/integration_test.rs`
    * `[ ]`   Reference values by index of `DERIVATION_CONTEXT_FIELD_KINDS`: `Text("left-pad")`, `Text("1.3.0")`, `FixedBytes32` of bytes each `0x44`, `FixedBytes32` of bytes each `0x55`, `Unsigned16(0x0102)`, `FixedBytes32` of bytes each `0x66`, `Unsigned64(2)`, `Unsigned32(32768)`, `Unsigned32(65536)`, and `Unsigned64(262144)`
    * `[ ]`   Reference context: `build_derivation_context` overriding the asset with `build_asset_identity` at name `left-pad` and version `1.3.0`, the deployment with `build_deployment_identity` at bytes each `0x44`, the suite with `build_suite_identifier` at identifier bytes each `0x55` and version `0x0102`, the parameter set with `build_parameter_set_identifier` at bytes each `0x66`, the group index with `build_group_index` at value `2`, and the geometry with `build_piece_geometry` at piece size `32768`, piece-group size `65536`, and extent `262144`, a geometry of four groups
    * `[ ]`   Reference vector: the `abi.encode` parameter encoding of the reference values, authored by hand from the Solidity Contract ABI Specification, one head word per field with the text offsets in their head words and each length and zero-padded bytes in the tail, and written as a hex string for `hex::decode`; it is not produced by running `alloy` or this crate
    * `[ ]`   `reference_context_encodes_to_the_reference_vector`
      * `[ ]`   Contract: the own entry, reference context encodes to the reference vector: the reference derivation context handed to `encode` with `DerivationContextDescription` → `Ok(EncodeSuccessReturn { bytes })` with `bytes` the known-answer vector authored from the ABI specification for the reference fields
      * `[ ]`   Arrange: `build_abi_encoding`, `build_derivation_context_description`, and `build_encode_params` over the description; the reference context, whose text fields are dynamic items between static items and whose same-kind fields differ, so a swapped field, a wrong word width, or a misplaced tail changes the bytes
      * `[ ]`   Act: `adapter.encode(params, &context)`
      * `[ ]`   Assert: `result.ok().map(|success| success.bytes)` equals `Some` of the reference vector's bytes by `assert_eq!`, the expectation authored from the ABI specification and not read back from the arrangement
      * `[ ]`   Boundary: the crate-internal call `AbiEncoding::encode`; the route `encode`, `DerivationContextDescription::to_fields`, and `domain`'s accessors, with the count and kind check against `FIELDS` between them
      * `[ ]`   Mocked: nothing; `alloy` runs as the concrete's own vendor library and `domain` is a pure value library
    * `[ ]`   `reference_vector_decodes_to_the_reference_context`
      * `[ ]`   Contract: the own entry, reference vector decodes to the reference context: the reference vector handed to `decode` with `DerivationContextDescription` → `Ok(DecodeSuccessReturn { described })` with `described` equal to the reference context
      * `[ ]`   Arrange: `build_abi_encoding`, `build_derivation_context_description`, and `build_decode_params` over the description; the reference vector's bytes as the payload and the reference context as the expectation, whose same-kind fields differ, so a field read from the wrong word or tail yields a refusal or an unequal context
      * `[ ]`   Act: `adapter.decode(params, &payload)`
      * `[ ]`   Assert: `result.ok().map(|success| success.described)` equals `Some` of the reference context by `assert_eq!`, the reference context built by `domain`'s builders and not by the subject
      * `[ ]`   Boundary: the crate-internal call `AbiEncoding::decode`; the route `decode`, the ABI decode, `DerivationContextDescription::fields_to_value`, `domain`'s constructors, and the canonical re-encoding through `encode`, `DerivationContextDescription::to_fields`, and `domain`'s accessors
      * `[ ]`   Mocked: nothing; `alloy` runs as the concrete's own vendor library and `domain` is a pure value library
    * `[ ]`   `a_context_survives_the_abi_round_trip`
      * `[ ]`   Contract: the own entry, a context survives the ABI round trip, which absorbs the `encoding/derivation_context` entry round trip through the canonical fields: an admitted `DerivationContext` handed to `encode`, and the bytes it returns handed to `decode`, yields `Ok(DecodeSuccessReturn { described })` with `described` equal to the original context
      * `[ ]`   Arrange: `build_abi_encoding`, `build_derivation_context_description`, `build_encode_params`, and `build_decode_params` over the description; `build_derivation_context` overriding the asset with `build_asset_identity` at name `left-pad` and version `1.3.0`, the deployment with `build_deployment_identity` at bytes each `0x44`, the suite with `build_suite_identifier` at identifier bytes each `0x55` and version `u16::MAX`, the parameter set with `build_parameter_set_identifier` at bytes each `0x66`, the group index with `build_group_index` at value `4294967296`, and the geometry with `build_piece_geometry` at piece size `16384`, piece-group size `2147483648`, and extent `u64::MAX`; the same-kind fields hold distinct values, the suite version is at its maximum, the piece-group size exceeds the `i32` range, and the group index and extent exceed the 32-bit range, so a swap of same-kind fields, a dropped field, or a narrowed width yields a refusal or an unequal context
      * `[ ]`   Act: `adapter.encode(encode_params, &context)`, then `adapter.decode(decode_params, &bytes)` over the bytes the first call returned
      * `[ ]`   Assert: the `described` of the second call's `Ok` arm equals the arranged context by `assert_eq!`
      * `[ ]`   Boundary: the crate-internal calls `AbiEncoding::encode` and `AbiEncoding::decode`; the route `encode`, `DerivationContextDescription::to_fields`, then `decode`, `DerivationContextDescription::fields_to_value`, and the canonical re-encoding, each running `domain`'s accessors and constructors as themselves
      * `[ ]`   Mocked: nothing; `alloy` runs as the concrete's own vendor library and `domain` is a pure value library
    * `[ ]`   `a_value_the_description_refuses_is_refused_with_the_description_refusal`
      * `[ ]`   Contract: the own entry, a value the description refuses is refused with the description's refusal: a canonical ABI encoding whose deployment-identity word is all zero handed to `decode` with `DerivationContextDescription` → `Err(DecodeErrorReturn::Description(DerivationContextFromFieldsErrorReturn::DeploymentIdentity(DeploymentIdentityTryNewErrorReturn::AllZero)))`
      * `[ ]`   Arrange: `build_abi_encoding`, `build_derivation_context_description`, and `build_decode_params` over the description; the reference vector's bytes with the deployment-identity word replaced by zero bytes, an encoding that is canonical and well-kinded, so the ABI decode and the canonical comparison admit it and only the domain's constructor refuses
      * `[ ]`   Act: `adapter.decode(params, &payload)`
      * `[ ]`   Assert: `result.err()` equals `Some(DecodeErrorReturn::Description(DerivationContextFromFieldsErrorReturn::DeploymentIdentity(DeploymentIdentityTryNewErrorReturn::AllZero)))` by `assert_eq!`, the whole refusal the deployment identity's constructor declares for all-zero bytes
      * `[ ]`   Boundary: the crate-internal call `AbiEncoding::decode`; the route `decode`, the ABI decode, the item pass, `DerivationContextDescription::fields_to_value`, and `DeploymentIdentity::try_new`
      * `[ ]`   Mocked: nothing; `alloy` runs as the concrete's own vendor library and `domain` is a pure value library

  * `[✅]`   `adapters/encoding/src/abi/provides.rs`
    * `[✅]`   `pub(crate) use super::interface::*;` and `#[cfg(test)] pub(crate) use super::mock::*;`, nothing else, so the concrete is visible to the crate's factory and to nothing outside the crate

  * `[✅]`   `directionality`
    * `[✅]`   `abi` depends on the `factory` module's surface through `crate::factory::provides` and on `alloy`; the `factory` module depends on `abi`'s error through `crate::abi::provides`, the family form's recorded cycle, which `encoding/factory` completes by constructing the concrete; `abi` names no description outside its tests; among repository crates the crate depends on `crates/domain` alone; nothing depends on the crate yet

  * `[ ]`   `requirements`
    * `[✅]`   `encode_writes_every_kind_as_its_abi_parameter_encoding`, `decode_reads_the_vector_as_its_value`, and `decode_rejects_nonzero_byte_string_padding` pass (CR-10's group-element encodings carried through the one ABI encoding; the byte form a proof-of-possession or delivery-proof challenge is hashed from, CR-09 and LC-08)
    * `[✅]`   `decode_rejects_nonzero_fixed_bytes20_padding` passes, and the vector tests it accompanies carry the `bytes20` and `uint256` widths (CR-09, the widths a transcript's parties and entitlements are encoded as, the verifier contract encoding `bytes20` and `uint256` to match)
    * `[✅]`   Every existing test in `adapters/encoding` passes unchanged, the derivation context's description and vectors unaffected by the new kinds
    * `[✅]`   `adapters/encoding/Cargo.toml` carries exactly the tables and keys stated above, and `alloy` is named nowhere in the crate outside `adapters/encoding/src/abi`
    * `[✅]`   `cargo check --workspace --all-targets --all-features`, `cargo fmt --all --check`, and `cargo deny check` complete without error; `cargo clippy --workspace --all-targets --all-features` reports nothing beyond the library target's unused-item warnings for the `abi` concrete, which `encoding/factory` resolves by constructing the concrete
    * `[ ]`   `reference_context_encodes_to_the_reference_vector` and `reference_vector_decodes_to_the_reference_context` pass (CR-11, the derivation context's canonical field sequence frozen against a known-answer vector)
    * `[✅]`   `decode_rejects_input_truncated_to_the_head`, `decode_rejects_trailing_bytes`, `decode_rejects_nonzero_string_padding`, `decode_rejects_a_non_canonical_offset`, `decode_rejects_a_uint16_word_wider_than_sixteen_bits`, `decode_rejects_a_uint32_word_wider_than_thirty_two_bits`, `decode_rejects_a_uint64_word_wider_than_sixty_four_bits`, and `decode_rejects_the_encoding_of_one_field_too_many` pass (one byte form per value)
    * `[ ]`   `try_new_returns_the_adapter`, `encode_rejects_a_description_emitting_one_value_short`, `encode_rejects_a_description_emitting_one_value_long`, `encode_checks_the_count_before_any_kind`, `encode_rejects_a_description_emitting_the_wrong_kind`, and `encode_reports_the_lowest_field_of_the_wrong_kind` pass
    * `[ ]`   `a_context_survives_the_abi_round_trip` and `a_value_the_description_refuses_is_refused_with_the_description_refusal` pass
    * `[✅]`   `DecodeErrorReturn` and `AbiDecoderErrorReturn` derive `Debug` and `PartialEq` and omit only `Eq`, which the vendor's error lacks; every refusal the concrete's tests assert is asserted by `assert_eq!` against the whole expected error, the truncated head naming the vendor's `Overrun`
    * `[✅]`   Code outside `adapters/encoding` naming `AbiEncoding` or anything under `abi` fails to compile; the crate's public surface is the `factory` and `derivation_context` modules' `provides`

* `[ ]`   `encoding/factory` **Encoding factory constructing the concrete the configuration names, admitted against the encoding identifier the hash-card or configuration requires, and handing it to a consumer generic over the family's encoder and decoder traits; carries the family's integration test**

  * `[✅]`   `objective`
    * `[✅]`   Problem: a consumer obtains an encoder and decoder only through the family's generic surface, never by naming a concrete, and the encoding it uses is the one the deployment's hash-card or the configuration requires, so a concrete that does not declare the required identifier is refused before anything is constructed and every hashed, signed, stored, or framed value is encoded one way for one identifier (CR-11; Composition Boundary)
    * `[✅]`   Functional: given the concrete the configuration names and the encoding identifier required, the factory refuses a concrete whose declared identifier is not the required one, with no construction and no call to the consumer
    * `[✅]`   Functional: an admitted concrete is constructed and handed to a consumer generic over `IEncoderAdapter` and `IDecoderAdapter` that reads `E::DECLARATION`, and the consumer's output is returned; the consumer never names the concrete
    * `[✅]`   Functional: a concrete's constructor error is returned unchanged in the factory's error arm, one variant per concrete
    * `[✅]`   Functional: the concrete the factory constructs encodes the reference derivation context to the known-answer vector and decodes it back through the family's traits and the description
    * `[✅]`   Non-functional: adding a concrete is its module, its variant in the selection enum and in the error enum, its branch here, and its entry in the declared set; adding an identifier is its variant in `EncodingIdentifier`; no consumer changes

  * `[✅]`   `role`
    * `[✅]`   Adapter family factory: the implementation of the `factory` module, the encoding family's construction point, and the crate's public surface beside the family-owned descriptions
    * `[✅]`   Hands the concrete to a consumer rather than returning it, because `IEncoderAdapter` and `IDecoderAdapter` carry generic methods and so cannot be returned as one type across concretes; the consumer is written once, generic over both traits, and the factory instantiates it for the concrete it constructs
    * `[✅]`   Selects by concrete and admits by identifier, so a further concrete implementing an existing identifier and a further identifier each arrive as a variant and a branch, with no change to the params or to any consumer
    * `[✅]`   Does not read a hash-card or the configuration; the composition resolver passes the concrete the configuration names as a typed `EncodingConcrete` and the identifier the hash-card or configuration requires as a typed `EncodingIdentifier`
    * `[✅]`   Does not encode, decode, or describe; the concrete and the descriptions do
    * `[✅]`   Carries the family's integration test across factory, concrete, and description; does not carry a commit

  * `[✅]`   `module`
    * `[✅]`   Bounded context: the `factory` module of `adapters/encoding`, holding the factory function, its deps, params, payload, and return types, its signature type, the consumer trait with its params and payload types, the selection enum and the declared set, the function mock, the consumer mock, and the builders, and the module's integration tests, one inside the module and one under `adapters/encoding/tests`
    * `[✅]`   Outside: the concrete's behavior, every description and domain type, the hash-card, the configuration catalogue, and every consumer of the family

  * `[ ]`   `deps`
    * `[✅]`   The `abi` concrete, through `crate::abi::provides`: `AbiEncoding`, `AbiEncodingConstructorParams`, `AbiEncoding::try_new`, and `AbiEncoding::DECLARATION`; the factory constructs its concrete, completing the family form's recorded cycle that `encoding/abi` opened
    * `[✅]`   The `factory` module's own interface: `IEncoderAdapter`, `IDecoderAdapter`, `EncodingIdentifier`, `EncodingDeclaration`, and `ENCODING_INTERFACE_VERSION`
    * `[ ]`   In `factory/test.rs`, `factory/integration_test.rs`, and `tests/factory_integration_test.rs` only: the module's own mock through `provides` under the `mocks` feature, `MockIEncodingConsumer`, `build_mock_i_encoding_consumer`, `MockIEncodingContract`, `MockIEncodingContractFromFieldsErrorReturn`, `build_create_encoding_deps`, `build_create_encoding_params`, `build_create_encoding_payload`, and `build_canonical_fields`, each with its overrides where it has them, and the declared set `DECLARED_ENCODING_SELECTIONS`; the integration tests reach the family only through `create_encoding`, and no test names a concrete
    * `[ ]`   In `tests/factory_integration_test.rs` only: the crate's `derivation_context` surface under the `mocks` feature, `DerivationContextDescription` through `build_derivation_context_description`, and `DerivationContextFromFieldsErrorReturn`; `domain` with its `mocks` feature, the dev-dependency `encoding/derivation_context` declares, for the component builders and their constructor-params overrides and the refusal types; and `hex`, the dev-dependency `encoding/abi` declares, for the reference vector
    * `[✅]`   `core::convert::Infallible`, standard library, the concrete's constructor error carried in the factory's error arm
    * `[✅]`   No new external crate; `adapters/encoding/Cargo.toml` is unchanged

  * `[ ]`   `context_slice`
    * `[✅]`   From the concrete: `AbiEncoding::try_new(AbiEncodingConstructorParams) -> Result<AbiEncoding, Infallible>`, the inherent constant `AbiEncoding::DECLARATION: EncodingDeclaration`, and `AbiEncoding`'s implementations of `IEncoderAdapter` and `IDecoderAdapter`
    * `[ ]`   In the tests: the `MockIEncodingConsumerOutput` the official consumer mock returns, read through its public fields `declaration`, `encoded`, `round_trip`, `trailing_bytes`, `truncated_bytes`, and `decoded`, and the `EncodingDeclaration` in `declaration`, read through its public fields `identifier`, `adapter_version`, and `interface_version`

  * `[✅]`   `adapters/encoding/src/factory/interface.rs`
    * `[✅]`   `EncodingIdentifier` gains `#[derive(Clone, Copy, PartialEq, Eq)]`, so the admission compares a declared identifier with the required one and a declared selection is read by value
    * `[✅]`   `EncodingConcrete`, an enum with `#[derive(Clone, Copy)]` and the one variant `Abi`, the selection of the concrete to construct
    * `[✅]`   `IEncodingConsumer`, a trait with `type Output;` and `fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(&self, params: ConsumeEncodingParams, payload: ConsumeEncodingPayload<E>) -> Self::Output;`, the work a composition performs with whichever concrete the factory constructs
    * `[✅]`   `ConsumeEncodingParams`, the fieldless struct `pub struct ConsumeEncodingParams;`
    * `[✅]`   `ConsumeEncodingPayload<E: IEncoderAdapter>`, a struct with only `pub adapter: E`; the consumer reads `E::DECLARATION`, so no independent declaration can be paired with the adapter
    * `[✅]`   `CreateEncodingDeps<C>`, a struct with `pub consumer: C`, the collaborator the factory hands the concrete to
    * `[✅]`   `CreateEncodingParams`, a struct with `pub concrete: EncodingConcrete` and `pub identifier: EncodingIdentifier`, the selection and the identifier the concrete must declare
    * `[✅]`   `CreateEncodingPayload`, the fieldless struct `pub struct CreateEncodingPayload;`, since the factory operates on no data
    * `[✅]`   `CreateEncodingSuccessReturn<O>`, a struct with `pub output: O`
    * `[✅]`   `CreateEncodingErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]`, which `Infallible` satisfies, and the variants `UnsupportedEncodingIdentifier`, the named concrete not declaring the required identifier, and `Abi(Infallible)`, the ABI concrete's constructor error carried unchanged; each further concrete's constructor error is its own variant
    * `[✅]`   `CreateEncodingReturn<O>`, the alias `Result<CreateEncodingSuccessReturn<O>, CreateEncodingErrorReturn>`
    * `[✅]`   `CreateEncodingFn<C>`, the alias `fn(&CreateEncodingDeps<C>, CreateEncodingParams, CreateEncodingPayload) -> CreateEncodingReturn<<C as IEncodingConsumer>::Output>`
    * `[ ]`   `DECLARED_ENCODING_SELECTIONS`, a `pub const` slice of `CreateEncodingParams` holding, for each real concrete, its selection and the identifier it declares, here `CreateEncodingParams { concrete: EncodingConcrete::Abi, identifier: EncodingIdentifier::EthereumAbiV1 }`, the declared set, which never holds the mock selection
    * `[ ]`   Under `#[cfg(any(test, feature = "mocks"))]`, `EncodingConcrete` gains the variant `Mock`, the selection of the family mock concrete
    * `[ ]`   Under `#[cfg(any(test, feature = "mocks"))]`, the file declares `MockIEncodingContractFromFieldsErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]` and the one variant `EmptyText { index: usize }`, the refusal of the official description mock
    * `[ ]`   Under `#[cfg(any(test, feature = "mocks"))]`, the file declares `MockIEncodingConsumerOutput<D: IEncodingContract>`, a struct with the fields `pub declaration: EncodingDeclaration`, `pub encoded: EncodeReturn`, `pub round_trip: DecodeReturn<D::Described, D::FromFieldsErrorReturn>`, `pub trailing_bytes: DecodeReturn<D::Described, D::FromFieldsErrorReturn>`, `pub truncated_bytes: DecodeReturn<D::Described, D::FromFieldsErrorReturn>`, and `pub decoded: DecodeReturn<D::Described, D::FromFieldsErrorReturn>`, the results the official consumer mock reads from the concrete it is handed
    * `[✅]`   Every item `encoding/derivation_context` and `encoding/abi` authored in this file is unchanged except the derive on `EncodingIdentifier`; `CreateEncodingErrorReturn` derives only what is stated

  * `[✅]`   `adapters/encoding/src/factory/interaction.spec.md`
    * `[✅]`   `create_encoding<C: IEncodingConsumer>(deps: &CreateEncodingDeps<C>, params: CreateEncodingParams, payload: CreateEncodingPayload) -> CreateEncodingReturn<C::Output>`: decision a `match` on `params.concrete`, one arm per `EncodingConcrete` variant, exhaustive so a variant with no arm fails to compile
    * `[✅]`   Unsupported identifier: condition the named concrete's `DECLARATION.identifier` is not `params.identifier`; decision equality, read before any construction; dependency call none; outcome `Err(CreateEncodingErrorReturn::UnsupportedEncodingIdentifier)`, with nothing constructed and the consumer not called; under the `mocks` feature the mock concrete declares `EncodingIdentifier::Mock`, so a selection paired with another concrete's identifier takes this branch
    * `[✅]`   Admitted: condition the named concrete's declared identifier is `params.identifier`; dependency calls the concrete's `try_new` with its fieldless constructor params, exactly once, its success destructured irrefutably because its error arm is uninhabited, then `deps.consumer.consume_encoding(ConsumeEncodingParams, ConsumeEncodingPayload { adapter })` exactly once; the consumer reads the concrete's declaration through `E::DECLARATION`; outcome `Ok(CreateEncodingSuccessReturn { output })` holding the consumer's output
    * `[ ]`   `Mock` arm, under `#[cfg(any(test, feature = "mocks"))]`: the same admission over `MockIEncoderAdapter::DECLARATION.identifier`; after admission, dependency call `deps.consumer.consume_encoding(ConsumeEncodingParams, ConsumeEncodingPayload { adapter })` exactly once with the mock concrete by value, which has no constructor and so no constructor error; outcome `Ok(CreateEncodingSuccessReturn { output })` holding the consumer's output
    * `[✅]`   `CreateEncodingErrorReturn::Abi` carries the constructor's uninhabited error type in the return union, so no branch produces it
    * `[✅]`   `params.concrete` selects and `params.identifier` admits; `payload` carries nothing and is not read
    * `[ ]`   Callee disposition, `encoding/abi` entry reference context encodes to the reference vector: carried; the factory selects the concrete the entry runs, so in this node's terms every concrete that declares `EncodingIdentifier::EthereumAbiV1` encodes the reference context to the reference vector; route `AbiEncoding::encode`, `DerivationContextDescription::to_fields`, and `domain`'s accessors, reached through `create_encoding` and the consumer it hands the concrete to; the outer-edge collaborator on the route is the consumer, replaced by the official `MockIEncodingConsumer` carrying the real description and the reference context
    * `[ ]`   Callee disposition, `encoding/abi` entry reference vector decodes to the reference context: carried; in this node's terms every concrete that declares `EncodingIdentifier::EthereumAbiV1` decodes the reference vector to the reference context; route `AbiEncoding::decode`, `DerivationContextDescription::fields_to_value`, `domain`'s constructors, and the canonical re-encoding through `AbiEncoding::encode`, reached through `create_encoding` and the consumer it hands the concrete to; the outer-edge collaborator on the route is the consumer, replaced by the official `MockIEncodingConsumer` carrying the real description and the reference vector
    * `[ ]`   Callee disposition, `encoding/abi` entry a context survives the ABI round trip: carried; in this node's terms every concrete encodes and decodes a context whose same-kind fields differ and whose widths are full, through the real description, and returns the context; route `AbiEncoding::encode`, `AbiEncoding::decode`, `DerivationContextDescription::to_fields` and `fields_to_value`, and `domain`'s accessors and constructors, reached through `create_encoding` and the consumer it hands the concrete to; the outer-edge collaborator on the route is the consumer, replaced by the official `MockIEncodingConsumer` carrying the real description and the context
    * `[ ]`   Callee disposition, `encoding/abi` entry a value the description refuses is refused with the description's refusal: carried; in this node's terms every concrete that declares `EncodingIdentifier::EthereumAbiV1` returns the domain's refusal unchanged for a canonical encoding whose deployment-identity word is all zero; route `AbiEncoding::decode`, `DerivationContextDescription::fields_to_value`, and `domain`'s constructors, reached through `create_encoding` and the consumer it hands the concrete to; the outer-edge collaborator on the route is the consumer, replaced by the official `MockIEncodingConsumer` carrying the real description and the encoding
    * `[ ]`   Callee disposition, `encoding/derivation_context` entry round trip through the canonical fields: carried; in this node's terms every concrete encodes the fields `to_fields` returns for a context and decodes the bytes into the fields `fields_to_value` admits, returning the context; route `AbiEncoding::encode`, `AbiEncoding::decode`, `DerivationContextDescription::to_fields` and `fields_to_value`, and `domain`'s accessors and constructors, reached through `create_encoding` and the consumer it hands the concrete to; the outer-edge collaborator on the route is the consumer, replaced by the official `MockIEncodingConsumer` carrying the real description and the context
    * `[ ]`   Own entry, every concrete is handed to the consumer under the identifier it declares: condition `create_encoding` is called with the selection of a concrete and the identifier that concrete declares; outcome `Ok(CreateEncodingSuccessReturn { output })` with `output.declaration` the declaration the consumer read through `E::DECLARATION`, whose identifier is the identifier required and whose interface version is `ENCODING_INTERFACE_VERSION`; variation each case of the declared set and the mock selection, whose declared identifiers differ, so a hand-over of a concrete other than the selected one returns another identifier; edge that must survive the composition: every concrete the factory admits declares the interface version the consumer is written against
    * `[ ]`   Own entry, every concrete is refused under an identifier it does not declare: condition `create_encoding` is called with the selection of a concrete and an identifier that concrete does not declare; outcome `Err(CreateEncodingErrorReturn::UnsupportedEncodingIdentifier)`; variation each case paired with the declared identifier of every other case whose declared identifier differs from its own, so an admission that compares against a constant or ignores the selection admits a pairing it must refuse; edge that must survive the composition: the refusal is decided by the selected concrete's own declaration
    * `[ ]`   Own entry, every concrete decodes what it encodes: condition `create_encoding` is called with the selection of a concrete and the identifier it declares, and the consumer encodes a field list through the concrete under the official description and decodes the bytes; outcome `output.round_trip` an `Ok(DecodeSuccessReturn { described })` whose `described` equals the field list encoded; variation each case of the declared set and the mock selection, whose byte forms differ, and a field list that differs from the builder's default in every value, so a concrete whose decode returns the default field list, or any value its bytes do not hold, fails; edge that must survive the composition: the concrete the factory constructs is the one whose `encode` and `decode` the consumer calls
    * `[ ]`   Own entry, every concrete refuses a description that emits the wrong count: condition `create_encoding` is called with the selection of a concrete and the identifier it declares, and the consumer encodes a field list one value short of the description's `FIELDS`; outcome `output.encoded` equal to `Err(EncodeErrorReturn::FieldCount { expected, actual })`, no bytes written; variation each case of the declared set and the mock selection, so a concrete that encodes a field list of the wrong count fails; edge that must survive the composition: the refusal is the family's own error, naming the description's declared count and the emitted count
    * `[ ]`   Own entry, every concrete refuses a description that emits the wrong kind: condition `create_encoding` is called with the selection of a concrete and the identifier it declares, and the consumer encodes a field list whose value at index `0` is not the kind the description's `FIELDS` names there; outcome `output.encoded` equal to `Err(EncodeErrorReturn::FieldKind { index, expected, actual })`, no bytes written; variation each case of the declared set and the mock selection, so a concrete that encodes a field list of the wrong kind fails; edge that must survive the composition: the refusal names the index, the kind declared there, and the kind emitted
    * `[ ]`   Own entry, every concrete refuses bytes past a complete encoding: condition `create_encoding` is called with the selection of a concrete and the identifier it declares, and the consumer decodes the bytes the concrete wrote for a field list followed by one extra byte; outcome `output.trailing_bytes` an `Err` in the concrete's own variant of `DecodeErrorReturn`; variation each case of the declared set and the mock selection, so a concrete that accepts bytes past a complete encoding fails; edge that must survive the composition: the extra byte is refused though the bytes before it are a complete encoding
    * `[ ]`   Own entry, every concrete refuses bytes that end inside an encoding: condition `create_encoding` is called with the selection of a concrete and the identifier it declares, and the consumer decodes the bytes the concrete wrote for a field list with the last byte removed; outcome `output.truncated_bytes` an `Err` in the concrete's own variant of `DecodeErrorReturn`; variation each case of the declared set and the mock selection, so a concrete that accepts bytes ending inside a field fails; edge that must survive the composition: the refusal is returned though every byte present is a byte the concrete wrote
    * `[ ]`   Own entry, every concrete returns the description's refusal unchanged: condition `create_encoding` is called with the selection of a concrete and the identifier it declares, and the consumer encodes a field list the official description refuses and decodes the bytes; outcome `output.round_trip` equal to `Err(DecodeErrorReturn::Description(error))` with `error` the description's refusal; variation each case of the declared set and the mock selection, whose byte forms differ, so a concrete that maps, drops, or replaces the description's refusal fails; edge that must survive the composition: the refusal arrives whole in the family's `Description` variant whatever the concrete's byte form
    * `[ ]`   Private surface: the chain of real functions `create_encoding`, the selected concrete's constructor where it has one, and the concrete's `encode` and `decode` as the consumer calls them, with the consumer's read of the concrete's `E::DECLARATION`; the outer-edge collaborators mocked are the consumer, replaced by the official `MockIEncodingConsumer`, and the description it carries, replaced by the official `MockIEncodingContract`; the cases are the declared set and the mock selection; the observable result is the output or the refusal `create_encoding` returns
    * `[ ]`   Public surface: an outside caller invokes `create_encoding` with `CreateEncodingDeps` built over the official consumer mock carrying a description, the described value, and the bytes it operates on, a `CreateEncodingParams` taken from the declared set, or from the declared set paired with an identifier no declared concrete declares, and a `CreateEncodingPayload`, and observes the output or the refusal it returns; the entries proven are the own entries above and the carried entries dispositioned above

  * `[✅]`   `adapters/encoding/src/factory/mock.rs`
    * `[ ]`   `MockIEncoderAdapter`, the unit struct `pub(crate) struct MockIEncoderAdapter;`, the family mock concrete, implementing `IEncoderAdapter` and `IDecoderAdapter`, since a consumer is generic over both traits and one concrete serves both, with `const DECLARATION` equal to `EncodingDeclaration { identifier: EncodingIdentifier::Mock, adapter_version: 1, interface_version: ENCODING_INTERFACE_VERSION }`; its `encode` calls `params.description.to_fields(ToFieldsParams, payload)` once and unpacks its infallible result, returns `Err(EncodeErrorReturn::FieldCount { expected, actual })` when the emitted count differs from `D::FIELDS.len()` and `Err(EncodeErrorReturn::FieldKind { index, expected, actual })` for the lowest index whose variant differs from the `D::FIELDS` kind, and otherwise returns `Ok(EncodeSuccessReturn { bytes })` with each value written in order, `FixedBytes32`, `FixedBytes20`, and `Unsigned256` as their bytes, `Unsigned16`, `Unsigned32`, and `Unsigned64` as the big-endian bytes of their own width, and `Text` and `Bytes` as a 32-bit big-endian length followed by their bytes; its `decode` reads one value per `D::FIELDS` kind in order from the payload in the byte form `encode` writes, returns `Err(DecodeErrorReturn::Mock(MockIDecoderAdapterErrorReturn::Truncated { index }))` when the payload ends inside the field at `index`, `InvalidText { index }` when a `Text` field's bytes are not UTF-8, and `TrailingBytes` when bytes remain after the last field, then calls `params.description.fields_to_value(FromFieldsParams, CanonicalFields { values })` once and returns `Err(DecodeErrorReturn::Description(error))` with the refusal unchanged, and otherwise `Ok(DecodeSuccessReturn { described })`
    * `[ ]`   `MockIEncodingContract`, the unit struct `pub struct MockIEncodingContract;` implementing `IEncodingContract` with `type Described = CanonicalFields;`, `type FromFieldsErrorReturn = MockIEncodingContractFromFieldsErrorReturn;`, and `const FIELDS` holding `Text`, `Bytes`, `FixedBytes32`, `Unsigned16`, `Unsigned32`, `Unsigned64`, `FixedBytes20`, and `Unsigned256`, the kinds of the default `values` of `build_canonical_fields` in order; `to_fields` returns `Ok(ToFieldsSuccessReturn { fields })` with `fields` a clone of its payload, and `fields_to_value` returns `Err(MockIEncodingContractFromFieldsErrorReturn::EmptyText { index })` for the lowest index holding an empty `Text` value, and otherwise `Ok(FromFieldsSuccessReturn { described })` with `described` its payload, so a test reaches the description's outcome by the field list it passes
    * `[ ]`   `MockIEncodingConsumer<'a, D: IEncodingContract>`, the struct `pub struct MockIEncodingConsumer<'a, D: IEncodingContract>` with the fields `pub description: &'a D`, `pub described: D::Described`, and `pub bytes: Vec<u8>`, the data its work operates on, implementing `IEncodingConsumer` with `type Output = MockIEncodingConsumerOutput<D>;`; its `consume_encoding` reads the concrete it is handed through the family's traits and returns a `MockIEncodingConsumerOutput` with `declaration` the concrete's `E::DECLARATION`; `encoded` the result of `encode` over `described` under `build_encode_params(description)`; `round_trip` the result of `decode` under `build_decode_params(description)` over the bytes `encode` of `described` returns when called again; `trailing_bytes` the result of `decode` over those bytes followed by one `0x00` byte; `truncated_bytes` the result of `decode` over those bytes without the last byte; and `decoded` the result of `decode` over `bytes`; an `encode` refusal inside `round_trip`, `trailing_bytes`, or `truncated_bytes` is carried unchanged as `DecodeErrorReturn::EncoderContract`, and it reads neither its params nor anything of the adapter beyond its declaration and its two methods
    * `[ ]`   `build_mock_i_encoding_consumer(description: &D, described: D::Described, bytes: Vec<u8>) -> MockIEncodingConsumer<'_, D>`, generic over `D: IEncodingContract` and taking its three properties as the arguments, since each depends on the type parameter and has no default
    * `[ ]`   `MockIEncodingConsumerOutputOverrides`, `#[derive(Default)]`, the fields `pub declaration: Option<EncodingDeclaration>`, `pub encoded: Option<EncodeReturn>`, `pub round_trip: Option<DecodeReturn<CanonicalFields, MockIEncodingContractFromFieldsErrorReturn>>`, `pub trailing_bytes: Option<DecodeReturn<CanonicalFields, MockIEncodingContractFromFieldsErrorReturn>>`, `pub truncated_bytes: Option<DecodeReturn<CanonicalFields, MockIEncodingContractFromFieldsErrorReturn>>`, and `pub decoded: Option<DecodeReturn<CanonicalFields, MockIEncodingContractFromFieldsErrorReturn>>`, and `build_mock_i_encoding_consumer_output(overrides: MockIEncodingConsumerOutputOverrides) -> MockIEncodingConsumerOutput<MockIEncodingContract>`, the description type fixed at `MockIEncodingContract`, the type the function mock's consumer carries; the omitted fields default to the results a conforming concrete yields for the default field list: `build_encoding_declaration` called with `Default::default()`; `Ok` of `build_encode_success_return` called with `Default::default()`; `Ok` of `build_decode_success_return` taking `build_canonical_fields` at its defaults, for `round_trip` and for `decoded`; `Err(DecodeErrorReturn::Mock(MockIDecoderAdapterErrorReturn::TrailingBytes))`, for `trailing_bytes`; and `Err(DecodeErrorReturn::Mock(MockIDecoderAdapterErrorReturn::Truncated { index }))`, for `truncated_bytes`, `index` the position of the last kind of `MockIEncodingContract::FIELDS`
    * `[ ]`   `ConsumeEncodingParamsOverrides`, the fieldless `#[derive(Default)]` struct `pub struct ConsumeEncodingParamsOverrides;`, and `build_consume_encoding_params(overrides: ConsumeEncodingParamsOverrides) -> ConsumeEncodingParams`
    * `[ ]`   `CreateEncodingPayloadOverrides`, the fieldless `#[derive(Default)]` struct `pub struct CreateEncodingPayloadOverrides;`, and `build_create_encoding_payload(overrides: CreateEncodingPayloadOverrides) -> CreateEncodingPayload`
    * `[ ]`   `CreateEncodingParamsOverrides`, `#[derive(Default)]`, the fields `pub concrete: Option<EncodingConcrete>` and `pub identifier: Option<EncodingIdentifier>`, and `build_create_encoding_params(overrides: CreateEncodingParamsOverrides) -> CreateEncodingParams`, the omitted fields defaulting to `EncodingConcrete::Mock` and `EncodingIdentifier::Mock`, so the defaults select the mock concrete under the identifier it declares
    * `[ ]`   `build_consume_encoding_payload(adapter: E) -> ConsumeEncodingPayload<E>`, `build_create_encoding_deps(consumer: C) -> CreateEncodingDeps<C>`, and `build_create_encoding_success_return(output: O) -> CreateEncodingSuccessReturn<O>`, each generic over the type of its one property and taking that property as the argument, since a generic property has no default and the type has no other property
    * `[ ]`   `mock_create_encoding`, a function of the production type `CreateEncodingFn<MockIEncodingConsumer<'static, MockIEncodingContract>>`, returning `Ok(build_create_encoding_success_return(build_mock_i_encoding_consumer_output(Default::default())))`
    * `[ ]`   No failure mode: every arm the mock concrete states is reached by an input
    * `[ ]`   No corruptions type and no invalidator for any owned object type: the crate has no serialization dependency; no mock for `EncodingConcrete`, `CreateEncodingErrorReturn`, and `MockIEncodingContractFromFieldsErrorReturn`, enums used by their production variants, for `DECLARED_ENCODING_SELECTIONS`, a constant used by its production value, or for `CreateEncodingReturn`, an alias of `Result`, and `CreateEncodingFn`, a function type whose mock is `mock_create_encoding`
    * `[ ]`   Imports the added names from `super::interface`

  * `[ ]`   `adapters/encoding/src/factory/test.rs`
    * `[ ]`   Collaborators: the consumer is the official mock `MockIEncodingConsumer`, built by `build_mock_i_encoding_consumer` over the official description `MockIEncodingContract`, `build_canonical_fields` at its defaults, and no bytes, whose output carries the declaration of the concrete it is handed; the concrete is the family's mock concrete, reached through `create_encoding` by `build_create_encoding_params` at its defaults, which select it under the identifier it declares; every deps value is `build_create_encoding_deps` over the consumer and every payload `build_create_encoding_payload`; no test names a concrete
    * `[ ]`   `create_encoding_hands_the_selected_mock_concrete_to_the_consumer`
      * `[ ]`   Contract: `create_encoding`, admitted: the named concrete's declared identifier is `params.identifier` → `Ok(CreateEncodingSuccessReturn { output })` holding the consumer's output
      * `[ ]`   Arrange: `build_create_encoding_deps` over the consumer, `build_create_encoding_params` at its defaults, and `build_create_encoding_payload`; the consumer's output carries the declaration of the concrete it is handed, so a factory that hands another concrete returns another declaration
      * `[ ]`   Act: `create_encoding(&deps, params, payload)`
      * `[ ]`   Assert: the `declaration` of the `output` of the `Ok` arm declares the identifier `EncodingIdentifier::Mock` by `assert!`, the identifier the family mock concrete declares
    * `[ ]`   `create_encoding_refuses_a_concrete_that_declares_another_identifier`
      * `[ ]`   Contract: `create_encoding`, unsupported identifier: the named concrete's `DECLARATION.identifier` is not `params.identifier` → `Err(CreateEncodingErrorReturn::UnsupportedEncodingIdentifier)`
      * `[ ]`   Arrange: `build_create_encoding_deps` over the consumer, `build_create_encoding_params` overriding `identifier` with `EncodingIdentifier::EthereumAbiV1` and keeping the default selection of the mock concrete, which declares `EncodingIdentifier::Mock`, so the identifier required differs from the identifier declared, and an admission that ignores the identifier or compares it with a constant fails; `build_create_encoding_payload`
      * `[ ]`   Act: `create_encoding(&deps, params, payload)`
      * `[ ]`   Assert: `result.err()` equals `Some(CreateEncodingErrorReturn::UnsupportedEncodingIdentifier)` by `assert_eq!`

  * `[✅]`   `construction`
    * `[✅]`   The composition root writes its encoding-dependent work once as an `IEncodingConsumer`, generic over `E: IEncoderAdapter + IDecoderAdapter`, and calls `create_encoding` with `CreateEncodingDeps { consumer }`, `CreateEncodingParams` holding the concrete the configuration names and the identifier the hash-card or configuration requires, and `CreateEncodingPayload`; no consumer constructs or names a concrete

  * `[✅]`   `adapters/encoding/src/factory/mod.rs`
    * `[✅]`   Adds `#[cfg(test)] mod integration_test;` and `#[cfg(test)] mod test;` to the wiring `encoding/derivation_context` authored
    * `[✅]`   `pub fn create_encoding<C: IEncodingConsumer>(deps: &CreateEncodingDeps<C>, params: CreateEncodingParams, _payload: CreateEncodingPayload) -> CreateEncodingReturn<C::Output>`, a `match` on `params.concrete` whose `EncodingConcrete::Abi` arm returns the refusal when `AbiEncoding::DECLARATION.identifier` is not `params.identifier`, then binds the concrete by `let Ok(adapter) = AbiEncoding::try_new(AbiEncodingConstructorParams);` and returns `Ok(CreateEncodingSuccessReturn { output: deps.consumer.consume_encoding(ConsumeEncodingParams, ConsumeEncodingPayload { adapter }) })`; under `#[cfg(any(test, feature = "mocks"))]` an `EncodingConcrete::Mock` arm returns the same refusal when `MockIEncoderAdapter::DECLARATION.identifier` is not `params.identifier`, then returns `Ok(CreateEncodingSuccessReturn { output: deps.consumer.consume_encoding(ConsumeEncodingParams, ConsumeEncodingPayload { adapter: MockIEncoderAdapter }) })`
    * `[✅]`   Imports `AbiEncoding` and `AbiEncodingConstructorParams` from `crate::abi::provides`, this module's types from `interface`, and, under the same configuration as the `Mock` arm, `MockIEncoderAdapter` from `mock`
    * `[✅]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[ ]`   `adapters/encoding/src/factory/integration_test.rs`
    * `[ ]`   Cases: every entry of `DECLARED_ENCODING_SELECTIONS` and the mock selection from `build_create_encoding_params` at its defaults; each block's body is written once and runs over every case, so a further concrete adds a case and no edit to the test
    * `[ ]`   `every_concrete_is_handed_to_the_consumer_under_the_identifier_it_declares`
      * `[ ]`   Contract: the own entry, every concrete is handed to the consumer under the identifier it declares: `create_encoding` called with a concrete's selection and the identifier it declares → `Ok(CreateEncodingSuccessReturn { output })` with `output.declaration` the declaration the consumer read through `E::DECLARATION`, whose identifier is the identifier required and whose interface version is `ENCODING_INTERFACE_VERSION`
      * `[ ]`   Arrange: for each case, `build_create_encoding_deps` over `build_mock_i_encoding_consumer` taking `&MockIEncodingContract`, `build_canonical_fields` at its defaults, and no bytes, `build_create_encoding_params` overriding `concrete` and `identifier` with the case's selection and identifier, and `build_create_encoding_payload`; the cases carry different identifiers, so a hand-over of a concrete other than the selected one returns another identifier
      * `[ ]`   Act: `create_encoding(&deps, params, payload)` for the case
      * `[ ]`   Assert: the `declaration` of the `output` of the `Ok` arm declares the case's identifier by `assert!` and the interface version `ENCODING_INTERFACE_VERSION` by `assert_eq!`, each compared apart
      * `[ ]`   Boundary: the crate-internal call `create_encoding`; the route `create_encoding`, the selected concrete's constructor where it has one, and the consumer's read of the concrete's `E::DECLARATION`
      * `[ ]`   Mocked: the consumer, replaced by `MockIEncodingConsumer`, and the description it carries, replaced by `MockIEncodingContract`, so the test does not prove what a real consumer or a real description does; the concretes are real, the mock selection reaching the family's official mock concrete
    * `[ ]`   `every_concrete_is_refused_under_an_identifier_it_does_not_declare`
      * `[ ]`   Contract: the own entry, every concrete is refused under an identifier it does not declare: `create_encoding` called with a concrete's selection and an identifier that concrete does not declare → `Err(CreateEncodingErrorReturn::UnsupportedEncodingIdentifier)`
      * `[ ]`   Arrange: for each case and for each other case whose declared identifier differs from its own, `build_create_encoding_deps` over `build_mock_i_encoding_consumer` taking `&MockIEncodingContract`, `build_canonical_fields` at its defaults, and no bytes, `build_create_encoding_params` overriding `concrete` with the case's selection and `identifier` with the other case's identifier, and `build_create_encoding_payload`; every pairing carries an identifier the selected concrete does not declare, so an admission that compares against a constant or ignores the selection admits a pairing it must refuse
      * `[ ]`   Act: `create_encoding(&deps, params, payload)` for the pairing
      * `[ ]`   Assert: `result.err()` equals `Some(CreateEncodingErrorReturn::UnsupportedEncodingIdentifier)` by `assert_eq!`
      * `[ ]`   Boundary: the crate-internal call `create_encoding`; the route `create_encoding` reading the selected concrete's declaration before any construction
      * `[ ]`   Mocked: the consumer, replaced by `MockIEncodingConsumer`, which the refusal never reaches
    * `[ ]`   `every_concrete_decodes_what_it_encodes`
      * `[ ]`   Contract: the own entry, every concrete decodes what it encodes: `create_encoding` called with a concrete's selection and the identifier it declares, the consumer encoding a field list through the concrete under the official description and decoding the bytes → `Ok(CreateEncodingSuccessReturn { output })` with `output.round_trip` an `Ok(DecodeSuccessReturn { described })` whose `described` equals the field list encoded
      * `[ ]`   Arrange: for each case, `build_create_encoding_deps` over `build_mock_i_encoding_consumer` taking `&MockIEncodingContract`, `build_canonical_fields` overriding `values` with `Text("round-trip")`, `Bytes` of the bytes `0xb1` and `0xb2`, `FixedBytes32` of bytes each `0x33`, `Unsigned16(0x0304)`, `Unsigned32(0x05060708)`, `Unsigned64(0x090a0b0c0d0e0f10)`, `FixedBytes20` of bytes each `0x44`, and `Unsigned256` of a last byte `0x07` and every other byte `0x00`, and no bytes, `build_create_encoding_params` overriding `concrete` and `identifier` with the case's selection and identifier, and `build_create_encoding_payload`; the cases write different byte forms, and the field list differs from the builder's default in every value, so a decode that returns the default field list, or any value its bytes do not hold, fails
      * `[ ]`   Act: `create_encoding(&deps, params, payload)` for the case
      * `[ ]`   Assert: the `described` of the `Ok` arm of `output.round_trip` equals `build_canonical_fields` overriding `values` with `Text("round-trip")`, `Bytes` of the bytes `0xb1` and `0xb2`, `FixedBytes32` of bytes each `0x33`, `Unsigned16(0x0304)`, `Unsigned32(0x05060708)`, `Unsigned64(0x090a0b0c0d0e0f10)`, `FixedBytes20` of bytes each `0x44`, and `Unsigned256` of a last byte `0x07` and every other byte `0x00` by `assert_eq!`, the expectation built apart from the field list the consumer was given
      * `[ ]`   Boundary: the crate-internal call `create_encoding`; the route `create_encoding`, the selected concrete's constructor where it has one, and the concrete's `encode` and `decode` as the consumer calls them
      * `[ ]`   Mocked: the consumer, replaced by `MockIEncodingConsumer`, and the description it carries, replaced by `MockIEncodingContract`, so the test does not prove what a real consumer or a real description does; the concretes are real, the mock selection reaching the family's official mock concrete
    * `[ ]`   `every_concrete_refuses_a_description_emitting_the_wrong_count`
      * `[ ]`   Contract: the own entry, every concrete refuses a description that emits the wrong count: `create_encoding` called with a concrete's selection and the identifier it declares, the consumer encoding a field list one value short of the description's `FIELDS` → `Ok(CreateEncodingSuccessReturn { output })` with `output.encoded` equal to `Err(EncodeErrorReturn::FieldCount { expected, actual })`
      * `[ ]`   Arrange: for each case, `build_create_encoding_deps` over `build_mock_i_encoding_consumer` taking `&MockIEncodingContract`, `build_canonical_fields` overriding `values` with `Text("short")`, `Bytes` of the byte `0xc1`, `FixedBytes32` of bytes each `0x55`, `Unsigned16(5)`, `Unsigned32(6)`, `Unsigned64(7)`, and `FixedBytes20` of bytes each `0x66`, and no bytes, `build_create_encoding_params` overriding `concrete` and `identifier` with the case's selection and identifier, and `build_create_encoding_payload`; the field list is one value short, so a concrete that encodes a field list of the wrong count returns bytes instead
      * `[ ]`   Act: `create_encoding(&deps, params, payload)` for the case
      * `[ ]`   Assert: the `encoded` of the `output` of the `Ok` arm, by `.err()`, equals `Some(EncodeErrorReturn::FieldCount { expected: <MockIEncodingContract as IEncodingContract>::FIELDS.len(), actual: 7 })` by `assert_eq!`
      * `[ ]`   Boundary: the crate-internal call `create_encoding`; the route `create_encoding`, the selected concrete's constructor where it has one, and the concrete's `encode` as the consumer calls it
      * `[ ]`   Mocked: the consumer, replaced by `MockIEncodingConsumer`, and the description it carries, replaced by `MockIEncodingContract`; the concretes are real, the mock selection reaching the family's official mock concrete
    * `[ ]`   `every_concrete_refuses_a_description_emitting_the_wrong_kind`
      * `[ ]`   Contract: the own entry, every concrete refuses a description that emits the wrong kind: `create_encoding` called with a concrete's selection and the identifier it declares, the consumer encoding a field list whose value at index `0` is not the kind the description's `FIELDS` names there → `Ok(CreateEncodingSuccessReturn { output })` with `output.encoded` equal to `Err(EncodeErrorReturn::FieldKind { index, expected, actual })`
      * `[ ]`   Arrange: for each case, `build_create_encoding_deps` over `build_mock_i_encoding_consumer` taking `&MockIEncodingContract`, `build_canonical_fields` overriding `values` with `Unsigned16(1)`, `Bytes` of the byte `0xc1`, `FixedBytes32` of bytes each `0x55`, `Unsigned16(5)`, `Unsigned32(6)`, `Unsigned64(7)`, `FixedBytes20` of bytes each `0x66`, and `Unsigned256` of a last byte `0x08` and every other byte `0x00`, and no bytes, `build_create_encoding_params` overriding `concrete` and `identifier` with the case's selection and identifier, and `build_create_encoding_payload`; the value at index `0` is of another kind, so a concrete that encodes a field list of the wrong kind returns bytes instead
      * `[ ]`   Act: `create_encoding(&deps, params, payload)` for the case
      * `[ ]`   Assert: the `encoded` of the `output` of the `Ok` arm, by `.err()`, equals `Some(EncodeErrorReturn::FieldKind { index: 0, expected: CanonicalFieldKind::Text, actual: CanonicalFieldKind::Unsigned16 })` by `assert_eq!`
      * `[ ]`   Boundary: the crate-internal call `create_encoding`; the route `create_encoding`, the selected concrete's constructor where it has one, and the concrete's `encode` as the consumer calls it
      * `[ ]`   Mocked: the consumer, replaced by `MockIEncodingConsumer`, and the description it carries, replaced by `MockIEncodingContract`; the concretes are real, the mock selection reaching the family's official mock concrete
    * `[ ]`   `every_concrete_refuses_bytes_past_a_complete_encoding`
      * `[ ]`   Contract: the own entry, every concrete refuses bytes past a complete encoding: `create_encoding` called with a concrete's selection and the identifier it declares, the consumer decoding the bytes the concrete wrote followed by one extra byte → `Ok(CreateEncodingSuccessReturn { output })` with `output.trailing_bytes` an `Err` in the concrete's own variant of `DecodeErrorReturn`
      * `[ ]`   Arrange: for each case, `build_create_encoding_deps` over `build_mock_i_encoding_consumer` taking `&MockIEncodingContract`, `build_canonical_fields` at its defaults, and no bytes, `build_create_encoding_params` overriding `concrete` and `identifier` with the case's selection and identifier, and `build_create_encoding_payload`; the bytes decoded are a complete encoding followed by one `0x00` byte, so a decode that accepts bytes past a complete encoding returns a value instead
      * `[ ]`   Act: `create_encoding(&deps, params, payload)` for the case
      * `[ ]`   Assert: the `trailing_bytes` of the `output` of the `Ok` arm is an `Err`, by `.is_err()`, the variant differing by concrete
      * `[ ]`   Boundary: the crate-internal call `create_encoding`; the route `create_encoding`, the selected concrete's constructor where it has one, and the concrete's `encode` and `decode` as the consumer calls them
      * `[ ]`   Mocked: the consumer, replaced by `MockIEncodingConsumer`, and the description it carries, replaced by `MockIEncodingContract`; the concretes are real, the mock selection reaching the family's official mock concrete
    * `[ ]`   `every_concrete_refuses_bytes_that_end_inside_an_encoding`
      * `[ ]`   Contract: the own entry, every concrete refuses bytes that end inside an encoding: `create_encoding` called with a concrete's selection and the identifier it declares, the consumer decoding the bytes the concrete wrote with the last byte removed → `Ok(CreateEncodingSuccessReturn { output })` with `output.truncated_bytes` an `Err` in the concrete's own variant of `DecodeErrorReturn`
      * `[ ]`   Arrange: for each case, `build_create_encoding_deps` over `build_mock_i_encoding_consumer` taking `&MockIEncodingContract`, `build_canonical_fields` at its defaults, and no bytes, `build_create_encoding_params` overriding `concrete` and `identifier` with the case's selection and identifier, and `build_create_encoding_payload`; the bytes decoded are a complete encoding without its last byte, so a decode that accepts bytes ending inside a field returns a value instead
      * `[ ]`   Act: `create_encoding(&deps, params, payload)` for the case
      * `[ ]`   Assert: the `truncated_bytes` of the `output` of the `Ok` arm is an `Err`, by `.is_err()`, the variant differing by concrete
      * `[ ]`   Boundary: the crate-internal call `create_encoding`; the route `create_encoding`, the selected concrete's constructor where it has one, and the concrete's `encode` and `decode` as the consumer calls them
      * `[ ]`   Mocked: the consumer, replaced by `MockIEncodingConsumer`, and the description it carries, replaced by `MockIEncodingContract`; the concretes are real, the mock selection reaching the family's official mock concrete
    * `[ ]`   `every_concrete_returns_the_description_refusal_unchanged`
      * `[ ]`   Contract: the own entry, every concrete returns the description's refusal unchanged: `create_encoding` called with a concrete's selection and the identifier it declares, the consumer encoding a field list the official description refuses and decoding the bytes → `Ok(CreateEncodingSuccessReturn { output })` with `output.round_trip` equal to `Err(DecodeErrorReturn::Description(error))` with `error` the description's refusal
      * `[ ]`   Arrange: for each case, `build_create_encoding_deps` over `build_mock_i_encoding_consumer` taking `&MockIEncodingContract`, `build_canonical_fields` overriding `values` with `Text("")`, `Bytes` of the byte `0xc1`, `FixedBytes32` of bytes each `0x55`, `Unsigned16(5)`, `Unsigned32(6)`, `Unsigned64(7)`, `FixedBytes20` of bytes each `0x66`, and `Unsigned256` of a last byte `0x08` and every other byte `0x00`, and no bytes, `build_create_encoding_params` overriding `concrete` and `identifier` with the case's selection and identifier, and `build_create_encoding_payload`; the field list holds an empty `Text` the description refuses, and the cases write different byte forms, so a concrete that maps, drops, or replaces the refusal fails
      * `[ ]`   Act: `create_encoding(&deps, params, payload)` for the case
      * `[ ]`   Assert: the `round_trip` of the `output` of the `Ok` arm, by `.err()`, equals `Some(DecodeErrorReturn::Description(MockIEncodingContractFromFieldsErrorReturn::EmptyText { index: 0 }))` by `assert_eq!`
      * `[ ]`   Boundary: the crate-internal call `create_encoding`; the route `create_encoding`, the selected concrete's constructor where it has one, and the concrete's `encode` and `decode` as the consumer calls them
      * `[ ]`   Mocked: the consumer, replaced by `MockIEncodingConsumer`, and the description it carries, replaced by `MockIEncodingContract`, so the test does not prove what a real consumer or a real description does; the concretes are real, the mock selection reaching the family's official mock concrete

  * `[✅]`   `adapters/encoding/src/factory/provides.rs`
    * `[✅]`   Adds `pub use super::create_encoding;` to the re-exports `encoding/derivation_context` authored

  * `[ ]`   `adapters/encoding/tests/factory_integration_test.rs`
    * `[ ]`   Cases: every entry of `DECLARED_ENCODING_SELECTIONS`, from the crate's public surface; each block's body is written once and runs over every case, so a further concrete adds a case and no edit to the test; a block that asserts the reference vector runs over the cases whose identifier is `EncodingIdentifier::EthereumAbiV1`, the identifier the vector is authored for
    * `[ ]`   `every_concrete_is_handed_to_the_consumer_under_the_identifier_it_declares`
      * `[ ]`   Contract: the own entry, every concrete is handed to the consumer under the identifier it declares: `create_encoding` called with a concrete's selection and the identifier it declares → `Ok(CreateEncodingSuccessReturn { output })` with `output.declaration` the declaration the consumer read through `E::DECLARATION`, whose identifier is the identifier required and whose interface version is `ENCODING_INTERFACE_VERSION`
      * `[ ]`   Arrange: for each case, `build_create_encoding_deps` over `build_mock_i_encoding_consumer` taking `&MockIEncodingContract`, `build_canonical_fields` at its defaults, and no bytes, `build_create_encoding_params` overriding `concrete` and `identifier` with the case's selection and identifier, and `build_create_encoding_payload`, all from the crate's public surface under the `mocks` feature; a hand-over of a concrete other than the selected one returns another identifier
      * `[ ]`   Act: `create_encoding(&deps, params, payload)` for the case
      * `[ ]`   Assert: the `declaration` of the `output` of the `Ok` arm declares the case's identifier by `assert!` and the interface version `ENCODING_INTERFACE_VERSION` by `assert_eq!`, each compared apart
      * `[ ]`   Boundary: the public call `create_encoding`; the route `create_encoding`, the selected concrete's constructor where it has one, and the consumer's read of the concrete's `E::DECLARATION`
      * `[ ]`   Mocked: the consumer, replaced by the official `MockIEncodingConsumer`, and the description it carries, replaced by the official `MockIEncodingContract`, so the test does not prove what a real consumer or a real description does
    * `[ ]`   `every_concrete_is_refused_under_an_identifier_it_does_not_declare`
      * `[ ]`   Contract: the own entry, every concrete is refused under an identifier it does not declare: `create_encoding` called with a concrete's selection and an identifier that concrete does not declare → `Err(CreateEncodingErrorReturn::UnsupportedEncodingIdentifier)`
      * `[ ]`   Arrange: for each case, `build_create_encoding_deps` over `build_mock_i_encoding_consumer` taking `&MockIEncodingContract`, `build_canonical_fields` at its defaults, and no bytes, `build_create_encoding_params` overriding `concrete` with the case's selection and `identifier` with `EncodingIdentifier::Mock`, an identifier no case of the declared set declares, and `build_create_encoding_payload`, all from the crate's public surface under the `mocks` feature; the pairing carries an identifier the selected concrete does not declare, so an admission that compares against a constant or ignores the selection admits a pairing it must refuse
      * `[ ]`   Act: `create_encoding(&deps, params, payload)` for the case
      * `[ ]`   Assert: `result.err()` equals `Some(CreateEncodingErrorReturn::UnsupportedEncodingIdentifier)` by `assert_eq!`
      * `[ ]`   Boundary: the public call `create_encoding`; the route `create_encoding` reading the selected concrete's declaration before any construction
      * `[ ]`   Mocked: the consumer, replaced by the official `MockIEncodingConsumer`, which the refusal never reaches
    * `[ ]`   `every_concrete_decodes_what_it_encodes`
      * `[ ]`   Contract: the own entry, every concrete decodes what it encodes: `create_encoding` called with a concrete's selection and the identifier it declares, the consumer encoding a field list through the concrete under the official description and decoding the bytes → `Ok(CreateEncodingSuccessReturn { output })` with `output.round_trip` an `Ok(DecodeSuccessReturn { described })` whose `described` equals the field list encoded
      * `[ ]`   Arrange: for each case, `build_create_encoding_deps` over `build_mock_i_encoding_consumer` taking `&MockIEncodingContract`, `build_canonical_fields` overriding `values` with `Text("round-trip")`, `Bytes` of the bytes `0xb1` and `0xb2`, `FixedBytes32` of bytes each `0x33`, `Unsigned16(0x0304)`, `Unsigned32(0x05060708)`, `Unsigned64(0x090a0b0c0d0e0f10)`, `FixedBytes20` of bytes each `0x44`, and `Unsigned256` of a last byte `0x07` and every other byte `0x00`, and no bytes, `build_create_encoding_params` overriding `concrete` and `identifier` with the case's selection and identifier, and `build_create_encoding_payload`, all from the crate's public surface under the `mocks` feature; the field list differs from the builder's default in every value, so a decode that returns the default field list, or any value its bytes do not hold, fails
      * `[ ]`   Act: `create_encoding(&deps, params, payload)` for the case
      * `[ ]`   Assert: the `described` of the `Ok` arm of `output.round_trip` equals `build_canonical_fields` overriding `values` with `Text("round-trip")`, `Bytes` of the bytes `0xb1` and `0xb2`, `FixedBytes32` of bytes each `0x33`, `Unsigned16(0x0304)`, `Unsigned32(0x05060708)`, `Unsigned64(0x090a0b0c0d0e0f10)`, `FixedBytes20` of bytes each `0x44`, and `Unsigned256` of a last byte `0x07` and every other byte `0x00` by `assert_eq!`, the expectation built apart from the field list the consumer was given
      * `[ ]`   Boundary: the public call `create_encoding`; the route `create_encoding`, the selected concrete's constructor where it has one, and the concrete's `encode` and `decode` as the consumer calls them
      * `[ ]`   Mocked: the consumer, replaced by the official `MockIEncodingConsumer`, and the description it carries, replaced by the official `MockIEncodingContract`, so the test does not prove what a real consumer or a real description does
    * `[ ]`   `every_concrete_refuses_a_description_emitting_the_wrong_count`
      * `[ ]`   Contract: the own entry, every concrete refuses a description that emits the wrong count: `create_encoding` called with a concrete's selection and the identifier it declares, the consumer encoding a field list one value short of the description's `FIELDS` → `Ok(CreateEncodingSuccessReturn { output })` with `output.encoded` equal to `Err(EncodeErrorReturn::FieldCount { expected, actual })`
      * `[ ]`   Arrange: for each case, `build_create_encoding_deps` over `build_mock_i_encoding_consumer` taking `&MockIEncodingContract`, `build_canonical_fields` overriding `values` with `Text("short")`, `Bytes` of the byte `0xc1`, `FixedBytes32` of bytes each `0x55`, `Unsigned16(5)`, `Unsigned32(6)`, `Unsigned64(7)`, and `FixedBytes20` of bytes each `0x66`, and no bytes, `build_create_encoding_params` overriding `concrete` and `identifier` with the case's selection and identifier, and `build_create_encoding_payload`, all from the crate's public surface under the `mocks` feature; the field list is one value short, so a concrete that encodes a field list of the wrong count returns bytes instead
      * `[ ]`   Act: `create_encoding(&deps, params, payload)` for the case
      * `[ ]`   Assert: the `encoded` of the `output` of the `Ok` arm, by `.err()`, equals `Some(EncodeErrorReturn::FieldCount { expected: <MockIEncodingContract as IEncodingContract>::FIELDS.len(), actual: 7 })` by `assert_eq!`
      * `[ ]`   Boundary: the public call `create_encoding`; the route `create_encoding`, the selected concrete's constructor where it has one, and the concrete's `encode` as the consumer calls it
      * `[ ]`   Mocked: the consumer, replaced by the official `MockIEncodingConsumer`, and the description it carries, replaced by the official `MockIEncodingContract`
    * `[ ]`   `every_concrete_refuses_a_description_emitting_the_wrong_kind`
      * `[ ]`   Contract: the own entry, every concrete refuses a description that emits the wrong kind: `create_encoding` called with a concrete's selection and the identifier it declares, the consumer encoding a field list whose value at index `0` is not the kind the description's `FIELDS` names there → `Ok(CreateEncodingSuccessReturn { output })` with `output.encoded` equal to `Err(EncodeErrorReturn::FieldKind { index, expected, actual })`
      * `[ ]`   Arrange: for each case, `build_create_encoding_deps` over `build_mock_i_encoding_consumer` taking `&MockIEncodingContract`, `build_canonical_fields` overriding `values` with `Unsigned16(1)`, `Bytes` of the byte `0xc1`, `FixedBytes32` of bytes each `0x55`, `Unsigned16(5)`, `Unsigned32(6)`, `Unsigned64(7)`, `FixedBytes20` of bytes each `0x66`, and `Unsigned256` of a last byte `0x08` and every other byte `0x00`, and no bytes, `build_create_encoding_params` overriding `concrete` and `identifier` with the case's selection and identifier, and `build_create_encoding_payload`, all from the crate's public surface under the `mocks` feature; the value at index `0` is of another kind, so a concrete that encodes a field list of the wrong kind returns bytes instead
      * `[ ]`   Act: `create_encoding(&deps, params, payload)` for the case
      * `[ ]`   Assert: the `encoded` of the `output` of the `Ok` arm, by `.err()`, equals `Some(EncodeErrorReturn::FieldKind { index: 0, expected: CanonicalFieldKind::Text, actual: CanonicalFieldKind::Unsigned16 })` by `assert_eq!`
      * `[ ]`   Boundary: the public call `create_encoding`; the route `create_encoding`, the selected concrete's constructor where it has one, and the concrete's `encode` as the consumer calls it
      * `[ ]`   Mocked: the consumer, replaced by the official `MockIEncodingConsumer`, and the description it carries, replaced by the official `MockIEncodingContract`
    * `[ ]`   `every_concrete_refuses_bytes_past_a_complete_encoding`
      * `[ ]`   Contract: the own entry, every concrete refuses bytes past a complete encoding: `create_encoding` called with a concrete's selection and the identifier it declares, the consumer decoding the bytes the concrete wrote followed by one extra byte → `Ok(CreateEncodingSuccessReturn { output })` with `output.trailing_bytes` an `Err` in the concrete's own variant of `DecodeErrorReturn`
      * `[ ]`   Arrange: for each case, `build_create_encoding_deps` over `build_mock_i_encoding_consumer` taking `&MockIEncodingContract`, `build_canonical_fields` at its defaults, and no bytes, `build_create_encoding_params` overriding `concrete` and `identifier` with the case's selection and identifier, and `build_create_encoding_payload`, all from the crate's public surface under the `mocks` feature; the bytes decoded are a complete encoding followed by one `0x00` byte, so a decode that accepts bytes past a complete encoding returns a value instead
      * `[ ]`   Act: `create_encoding(&deps, params, payload)` for the case
      * `[ ]`   Assert: the `trailing_bytes` of the `output` of the `Ok` arm is an `Err`, by `.is_err()`, the variant differing by concrete
      * `[ ]`   Boundary: the public call `create_encoding`; the route `create_encoding`, the selected concrete's constructor where it has one, and the concrete's `encode` and `decode` as the consumer calls them
      * `[ ]`   Mocked: the consumer, replaced by the official `MockIEncodingConsumer`, and the description it carries, replaced by the official `MockIEncodingContract`
    * `[ ]`   `every_concrete_refuses_bytes_that_end_inside_an_encoding`
      * `[ ]`   Contract: the own entry, every concrete refuses bytes that end inside an encoding: `create_encoding` called with a concrete's selection and the identifier it declares, the consumer decoding the bytes the concrete wrote with the last byte removed → `Ok(CreateEncodingSuccessReturn { output })` with `output.truncated_bytes` an `Err` in the concrete's own variant of `DecodeErrorReturn`
      * `[ ]`   Arrange: for each case, `build_create_encoding_deps` over `build_mock_i_encoding_consumer` taking `&MockIEncodingContract`, `build_canonical_fields` at its defaults, and no bytes, `build_create_encoding_params` overriding `concrete` and `identifier` with the case's selection and identifier, and `build_create_encoding_payload`, all from the crate's public surface under the `mocks` feature; the bytes decoded are a complete encoding without its last byte, so a decode that accepts bytes ending inside a field returns a value instead
      * `[ ]`   Act: `create_encoding(&deps, params, payload)` for the case
      * `[ ]`   Assert: the `truncated_bytes` of the `output` of the `Ok` arm is an `Err`, by `.is_err()`, the variant differing by concrete
      * `[ ]`   Boundary: the public call `create_encoding`; the route `create_encoding`, the selected concrete's constructor where it has one, and the concrete's `encode` and `decode` as the consumer calls them
      * `[ ]`   Mocked: the consumer, replaced by the official `MockIEncodingConsumer`, and the description it carries, replaced by the official `MockIEncodingContract`
    * `[ ]`   `every_concrete_returns_the_description_refusal_unchanged`
      * `[ ]`   Contract: the own entry, every concrete returns the description's refusal unchanged: `create_encoding` called with a concrete's selection and the identifier it declares, the consumer encoding a field list the official description refuses and decoding the bytes → `Ok(CreateEncodingSuccessReturn { output })` with `output.round_trip` equal to `Err(DecodeErrorReturn::Description(error))` with `error` the description's refusal
      * `[ ]`   Arrange: for each case, `build_create_encoding_deps` over `build_mock_i_encoding_consumer` taking `&MockIEncodingContract`, `build_canonical_fields` overriding `values` with `Text("")`, `Bytes` of the byte `0xc1`, `FixedBytes32` of bytes each `0x55`, `Unsigned16(5)`, `Unsigned32(6)`, `Unsigned64(7)`, `FixedBytes20` of bytes each `0x66`, and `Unsigned256` of a last byte `0x08` and every other byte `0x00`, and no bytes, `build_create_encoding_params` overriding `concrete` and `identifier` with the case's selection and identifier, and `build_create_encoding_payload`, all from the crate's public surface under the `mocks` feature; the field list holds an empty `Text` the description refuses, so a concrete that maps, drops, or replaces the refusal fails
      * `[ ]`   Act: `create_encoding(&deps, params, payload)` for the case
      * `[ ]`   Assert: the `round_trip` of the `output` of the `Ok` arm, by `.err()`, equals `Some(DecodeErrorReturn::Description(MockIEncodingContractFromFieldsErrorReturn::EmptyText { index: 0 }))` by `assert_eq!`
      * `[ ]`   Boundary: the public call `create_encoding`; the route `create_encoding`, the selected concrete's constructor where it has one, and the concrete's `encode` and `decode` as the consumer calls them
      * `[ ]`   Mocked: the consumer, replaced by the official `MockIEncodingConsumer`, and the description it carries, replaced by the official `MockIEncodingContract`
    * `[ ]`   `every_concrete_declaring_the_ethereum_abi_identifier_encodes_the_reference_context_to_the_reference_vector`
      * `[ ]`   Contract: the carried entry `encoding/abi` reference context encodes to the reference vector, in this node's terms: `create_encoding` called with the selection of a concrete that declares `EncodingIdentifier::EthereumAbiV1` and that identifier, the consumer encoding the reference context through the concrete under the real description → `Ok(CreateEncodingSuccessReturn { output })` with `output.encoded` an `Ok(EncodeSuccessReturn { bytes })` with `bytes` the reference vector
      * `[ ]`   Arrange: for each case whose identifier is `EncodingIdentifier::EthereumAbiV1`, `build_derivation_context_description`, `build_create_encoding_deps` over `build_mock_i_encoding_consumer` taking that description, the reference context built as `build_derivation_context` overriding the asset with `build_asset_identity` at name `left-pad` and version `1.3.0`, the deployment with `build_deployment_identity` at bytes each `0x44`, the suite with `build_suite_identifier` at identifier bytes each `0x55` and version `0x0102`, the parameter set with `build_parameter_set_identifier` at bytes each `0x66`, the group index with `build_group_index` at value `2`, and the geometry with `build_piece_geometry` at piece size `32768`, piece-group size `65536`, and extent `262144`, and no bytes, `build_create_encoding_params` overriding `concrete` and `identifier` with the case's selection and identifier, and `build_create_encoding_payload`, all from the crate's public surface under the `mocks` feature; the reference context's text fields are dynamic items between static items and its same-kind fields differ, so a swapped field, a wrong word width, or a misplaced tail changes the bytes
      * `[ ]`   Act: `create_encoding(&deps, params, payload)` for the case
      * `[ ]`   Assert: the `encoded` of the `output` of the `Ok` arm, by `.ok().map(|success| success.bytes)`, equals `Some` of the reference vector's bytes by `assert_eq!`, the reference vector being the bytes of the `abi.encode` parameter encoding of `Text("left-pad")`, `Text("1.3.0")`, `FixedBytes32` of bytes each `0x44`, `FixedBytes32` of bytes each `0x55`, `Unsigned16(0x0102)`, `FixedBytes32` of bytes each `0x66`, `Unsigned64(2)`, `Unsigned32(32768)`, `Unsigned32(65536)`, and `Unsigned64(262144)`, a hex string authored by hand from the Solidity Contract ABI Specification with one head word per field, the text offsets in their head words, and each length and zero-padded bytes in the tail, decoded by `hex::decode`, and not produced by running `alloy` or this crate; the expectation is not read back from the arrangement
      * `[ ]`   Boundary: the public call `create_encoding`; the route `create_encoding`, the selected concrete's constructor, `encode`, `DerivationContextDescription::to_fields`, and `domain`'s accessors
      * `[ ]`   Mocked: the consumer, replaced by the official `MockIEncodingConsumer`, so the test does not prove what a real consumer does with the concrete; the description and `domain` run as themselves
    * `[ ]`   `every_concrete_declaring_the_ethereum_abi_identifier_decodes_the_reference_vector_to_the_reference_context`
      * `[ ]`   Contract: the carried entry `encoding/abi` reference vector decodes to the reference context, in this node's terms: `create_encoding` called with the selection of a concrete that declares `EncodingIdentifier::EthereumAbiV1` and that identifier, the consumer decoding the reference vector under the real description → `Ok(CreateEncodingSuccessReturn { output })` with `output.decoded` an `Ok(DecodeSuccessReturn { described })` with `described` equal to the reference context
      * `[ ]`   Arrange: for each case whose identifier is `EncodingIdentifier::EthereumAbiV1`, `build_derivation_context_description`, `build_create_encoding_deps` over `build_mock_i_encoding_consumer` taking that description, the reference context built as `build_derivation_context` overriding the asset with `build_asset_identity` at name `left-pad` and version `1.3.0`, the deployment with `build_deployment_identity` at bytes each `0x44`, the suite with `build_suite_identifier` at identifier bytes each `0x55` and version `0x0102`, the parameter set with `build_parameter_set_identifier` at bytes each `0x66`, the group index with `build_group_index` at value `2`, and the geometry with `build_piece_geometry` at piece size `32768`, piece-group size `65536`, and extent `262144`, and the reference vector's bytes, the `abi.encode` parameter encoding of `Text("left-pad")`, `Text("1.3.0")`, `FixedBytes32` of bytes each `0x44`, `FixedBytes32` of bytes each `0x55`, `Unsigned16(0x0102)`, `FixedBytes32` of bytes each `0x66`, `Unsigned64(2)`, `Unsigned32(32768)`, `Unsigned32(65536)`, and `Unsigned64(262144)`, a hex string authored by hand from the Solidity Contract ABI Specification with one head word per field, the text offsets in their head words, and each length and zero-padded bytes in the tail, decoded by `hex::decode`, and not produced by running `alloy` or this crate, `build_create_encoding_params` overriding `concrete` and `identifier` with the case's selection and identifier, and `build_create_encoding_payload`, all from the crate's public surface under the `mocks` feature; the reference context's same-kind fields differ, so a field read from the wrong word or tail yields a refusal or an unequal context
      * `[ ]`   Act: `create_encoding(&deps, params, payload)` for the case
      * `[ ]`   Assert: the `decoded` of the `output` of the `Ok` arm, by `.ok().map(|success| success.described)`, equals `Some` of the reference context built again by `build_derivation_context` with the same overrides, by `assert_eq!`
      * `[ ]`   Boundary: the public call `create_encoding`; the route `create_encoding`, the selected concrete's constructor, `decode`, `DerivationContextDescription::fields_to_value`, `domain`'s constructors, and the canonical re-encoding through `encode`, `DerivationContextDescription::to_fields`, and `domain`'s accessors
      * `[ ]`   Mocked: the consumer, replaced by the official `MockIEncodingConsumer`, so the test does not prove what a real consumer does with the concrete; the description and `domain` run as themselves
    * `[ ]`   `every_concrete_round_trips_a_context_through_the_real_description`
      * `[ ]`   Contract: the carried entry `encoding/abi` a context survives the ABI round trip, in this node's terms: `create_encoding` called with a concrete's selection and the identifier it declares, the consumer encoding the wide context through the concrete under the real description and decoding the bytes → `Ok(CreateEncodingSuccessReturn { output })` with `output.round_trip` an `Ok(DecodeSuccessReturn { described })` with `described` equal to the wide context
      * `[ ]`   Arrange: for each case, `build_derivation_context_description`, `build_create_encoding_deps` over `build_mock_i_encoding_consumer` taking that description, the wide context built as `build_derivation_context` overriding the asset with `build_asset_identity` at name `left-pad` and version `1.3.0`, the deployment with `build_deployment_identity` at bytes each `0x44`, the suite with `build_suite_identifier` at identifier bytes each `0x55` and version `u16::MAX`, the parameter set with `build_parameter_set_identifier` at bytes each `0x66`, the group index with `build_group_index` at value `4294967296`, and the geometry with `build_piece_geometry` at piece size `16384`, piece-group size `2147483648`, and extent `u64::MAX`, and no bytes, `build_create_encoding_params` overriding `concrete` and `identifier` with the case's selection and identifier, and `build_create_encoding_payload`, all from the crate's public surface under the `mocks` feature; the same-kind fields hold distinct values and the widths are full, so a swap of same-kind fields, a dropped field, or a narrowed width yields a refusal or an unequal context
      * `[ ]`   Act: `create_encoding(&deps, params, payload)` for the case
      * `[ ]`   Assert: the `round_trip` of the `output` of the `Ok` arm, by `.ok().map(|success| success.described)`, equals `Some` of the wide context built again by `build_derivation_context` with the same overrides, by `assert_eq!`
      * `[ ]`   Boundary: the public call `create_encoding`; the route `create_encoding`, the selected concrete's constructor where it has one, `encode`, `decode`, `DerivationContextDescription::to_fields` and `fields_to_value`, and `domain`'s accessors and constructors
      * `[ ]`   Mocked: the consumer, replaced by the official `MockIEncodingConsumer`, so the test does not prove what a real consumer does with the concrete; the description and `domain` run as themselves
    * `[ ]`   `every_concrete_declaring_the_ethereum_abi_identifier_returns_the_domain_refusal_unchanged`
      * `[ ]`   Contract: the carried entry `encoding/abi` a value the description refuses is refused with the description's refusal, in this node's terms: `create_encoding` called with the selection of a concrete that declares `EncodingIdentifier::EthereumAbiV1` and that identifier, the consumer decoding a canonical encoding whose deployment-identity word is all zero under the real description → `Ok(CreateEncodingSuccessReturn { output })` with `output.decoded` equal to `Err(DecodeErrorReturn::Description(DerivationContextFromFieldsErrorReturn::DeploymentIdentity(DeploymentIdentityTryNewErrorReturn::AllZero)))`
      * `[ ]`   Arrange: for each case whose identifier is `EncodingIdentifier::EthereumAbiV1`, `build_derivation_context_description`, `build_create_encoding_deps` over `build_mock_i_encoding_consumer` taking that description, the reference context built as `build_derivation_context` overriding the asset with `build_asset_identity` at name `left-pad` and version `1.3.0`, the deployment with `build_deployment_identity` at bytes each `0x44`, the suite with `build_suite_identifier` at identifier bytes each `0x55` and version `0x0102`, the parameter set with `build_parameter_set_identifier` at bytes each `0x66`, the group index with `build_group_index` at value `2`, and the geometry with `build_piece_geometry` at piece size `32768`, piece-group size `65536`, and extent `262144`, and the bytes of the `abi.encode` parameter encoding of the same ten values with `FixedBytes32` of zero bytes in place of the deployment identity's bytes, a hex string authored by hand from the Solidity Contract ABI Specification and decoded by `hex::decode`, `build_create_encoding_params` overriding `concrete` and `identifier` with the case's selection and identifier, and `build_create_encoding_payload`, all from the crate's public surface under the `mocks` feature; the encoding is canonical and well-kinded, so the ABI decode and the canonical comparison admit it and only the domain's constructor refuses
      * `[ ]`   Act: `create_encoding(&deps, params, payload)` for the case
      * `[ ]`   Assert: the `decoded` of the `output` of the `Ok` arm, by `.err()`, equals `Some(DecodeErrorReturn::Description(DerivationContextFromFieldsErrorReturn::DeploymentIdentity(DeploymentIdentityTryNewErrorReturn::AllZero)))` by `assert_eq!`, the whole refusal the deployment identity's constructor declares for all-zero bytes
      * `[ ]`   Boundary: the public call `create_encoding`; the route `create_encoding`, the selected concrete's constructor, `decode`, the item pass, `DerivationContextDescription::fields_to_value`, and `DeploymentIdentity::try_new`
      * `[ ]`   Mocked: the consumer, replaced by the official `MockIEncodingConsumer`, so the test does not prove what a real consumer does with the concrete; the description and `domain` run as themselves

  * `[✅]`   `directionality`
    * `[✅]`   The `factory` module depends on the `abi` concrete through `crate::abi::provides` and on its own interface; `abi` depends on the `factory` module's surface, the family form's recorded cycle; `derivation_context` depends on the `factory` module's surface and nothing in the `factory` module names it; the crate's public surface is the `factory` and `derivation_context` modules' `provides`; nothing depends on the crate yet
    * `[✅]`   `kdf/blake3_keyed` consumes the family through `create_encoding` and an `IEncodingConsumer`

  * `[ ]`   `requirements`
    * `[✅]`   `CreateEncodingErrorReturn` derives `Debug`, `PartialEq`, and `Eq`
    * `[ ]`   `create_encoding_hands_the_selected_mock_concrete_to_the_consumer` and `create_encoding_refuses_a_concrete_that_declares_another_identifier` pass
    * `[ ]`   `every_concrete_is_handed_to_the_consumer_under_the_identifier_it_declares`, `every_concrete_is_refused_under_an_identifier_it_does_not_declare`, `every_concrete_decodes_what_it_encodes`, `every_concrete_refuses_a_description_emitting_the_wrong_count`, `every_concrete_refuses_a_description_emitting_the_wrong_kind`, `every_concrete_refuses_bytes_past_a_complete_encoding`, `every_concrete_refuses_bytes_that_end_inside_an_encoding`, and `every_concrete_returns_the_description_refusal_unchanged` pass in the module's integration test and in the crate's `tests/factory_integration_test.rs`
    * `[ ]`   `every_concrete_declaring_the_ethereum_abi_identifier_encodes_the_reference_context_to_the_reference_vector`, `every_concrete_declaring_the_ethereum_abi_identifier_decodes_the_reference_vector_to_the_reference_context`, `every_concrete_round_trips_a_context_through_the_real_description`, and `every_concrete_declaring_the_ethereum_abi_identifier_returns_the_domain_refusal_unchanged` pass in the crate's `tests/factory_integration_test.rs` (CR-11, the concrete the factory constructs encodes and decodes the derivation context through the family's traits and the description)
    * `[✅]`   `cargo check --workspace --all-targets --all-features`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo fmt --all --check`, and `cargo deny check` complete without error or warning in every target, the `abi` concrete's unused-item warnings having no remaining cause
    * `[✅]`   `alloy` is named nowhere outside `adapters/encoding/src/abi`, and no code outside `adapters/encoding` can name `AbiEncoding`

* `[ ]`   `kdf/blake3_keyed` **BLAKE3 keyed-derivation concrete deriving a key of a requested length from secret key material under an encoded context and a fixed context string per purpose; creates the `adapters/kdf` crate and authors the key-derivation family's generic interface, the KDF identifier, the declaration, and the mock**

  * `[ ]`   `objective`
    * `[✅]`   Problem: every derivation a client performs off chain, the wrapping key, the publisher lineage's roots, master scalar, identity bases, capsule randomness, and piece-group keys, and the keyed plaintext-root key, is a domain-separated derivation of secret key material under a context, so the derivation passes through one repo-owned interface, its context strings, input serialization, and output lengths are frozen, and no module outside a concrete names the hash library (CR-05; CR-11; `docs/research/cryptography.md`'s Credential KEM statement, Publisher lineage, and Plaintext-root disclosure modes)
    * `[✅]`   Functional: the family's generic interface derives, for a named purpose and a requested length, a key from borrowed secret key material and the canonical encoding of a context, and returns the key inside a `Secret`
    * `[✅]`   Functional: the purposes are those the specification names: wrapping key, publisher root, asset root, master scalar, identity bases, capsule randomness, piece-group key, and plaintext-root key; each maps, in every concrete, to that concrete's fixed domain separation for the purpose
    * `[✅]`   Functional: every concrete declares the KDF identifier a deployment's hash-card names, its adapter version, and the interface version it implements, readable before any instance exists
    * `[✅]`   Functional: the BLAKE3 concrete derives in BLAKE3's derive-key mode under the purpose's context string, over the key material serialized as the secret's length as an 8-byte big-endian integer, then the secret, then the encoded context, and reads the extendable output to the requested length
    * `[ ]`   Functional: the concrete's output for every purpose matches the independent oracle `expected_derived_key` over the reference key material and context, and a shorter output is the prefix of a longer one
    * `[✅]`   Non-functional: `blake3` is named only inside `adapters/kdf/src/blake3_keyed`; the hasher and the output reader are zeroized after every derivation; the crate depends on no repository crate but `domain` at runtime

  * `[✅]`   `role`
    * `[✅]`   Adapter: the key-derivation family's first concrete, and the first source file that requires the family's generic interface, the purpose enum, the KDF identifier, the declaration, and the family's mock, which it authors in the family's `factory` module as its producers
    * `[✅]`   Creates `factory/mod.rs` with the module wiring alone and `factory/provides.rs` with the interface and mock surface alone; the factory function, its types, its interaction spec, its unit test, its re-export, and the family's integration test are `kdf/factory`'s
    * `[✅]`   Does not encode any context; each caller encodes its context through the encoding factory and hands the canonical bytes to the derivation
    * `[✅]`   Does not sample a scalar, wrap a key, or compute a plaintext root; `workflows/sidecar/wrap`, the pairing family's sampling bound, and the commitment layer consume the derived bytes
    * `[✅]`   Does not map the KDF identifier to the hash-card's `kdfId` byte; the hash-card's own encoding does that
    * `[✅]`   Does not carry a commit

  * `[✅]`   `module`
    * `[✅]`   Bounded context: the `adapters/kdf` crate's `factory` module, holding the family's generic trait, the derivation purpose enum, the method's params, payload, success, error, and return types, the KDF identifier, the declaration, the interface version, and the family's mock; and its private `blake3_keyed` concrete, holding the adapter over `blake3`, its context string per purpose, its constructor params and return, and its derivation error
    * `[✅]`   Creates the crate at `adapters/kdf`, admitted by the workspace's `adapters/*` glob with no edit to the root manifest
    * `[✅]`   Outside: every context type and its encoding, what each caller does with the derived bytes, and the factory's selection of a concrete

  * `[ ]`   `deps`
    * `[✅]`   `domain`, `crates/domain`, protocol and domain ring, path dependency; supplies `Secret` and `SecretConstructorParams`, the wrapper for the key material and the derived key; direction inward, adapter ring on domain ring
    * `[✅]`   `domain` with its `mocks` feature, as a dev-dependency and through this crate's `mocks` feature; supplies `build_secret`, `SecretConstructorParamsOverrides`, `build_asset_identity_hash`, and `AssetIdentityHashConstructorParamsOverrides`
    * `[✅]`   `zeroize` `1.9.0`, external crate, Apache-2.0 OR MIT, runtime dependency; supplies the `Zeroize` trait whose `zeroize` the concrete calls on the hasher and the output reader
    * `[✅]`   `blake3` `1.8.7`, external crate, CC0-1.0 OR Apache-2.0 OR Apache-2.0 WITH LLVM-exception, runtime dependency with the `zeroize` feature, named only in `blake3_keyed`; supplies `blake3::Hasher` and `blake3::OutputReader`, each implementing `Zeroize` under that feature
    * `[ ]`   `serde_json` `1.0.151`, external crate, MIT OR Apache-2.0, optional runtime dependency enabled by this crate's `mocks` feature and a dev-dependency; supplies `serde_json::Value` and `serde_json::Map`, the untrusted type every invalidator returns
    * `[ ]`   The crate does not depend on `adapters/encoding`, since the derivation takes the canonical encoding as bytes
    * `[ ]`   `core::convert::Infallible`, standard library, the constructor's error arm; `TryFrom<usize> for u64`, standard library, the length prefix; `std::collections::hash_map::DefaultHasher` and `std::sync::OnceLock`, standard library, the family mock's byte computation and process-lifetime reference fixtures
    * `[✅]`   No reverse dependency beyond the family form's recorded cycle: the `factory` module's error carries this concrete's error; nothing depends on the crate yet

  * `[ ]`   `context_slice`
    * `[✅]`   From `domain`: `Secret::try_new(SecretConstructorParams { value })` returning `Result<Secret<T>, Infallible>`, and `Secret::expose(&self) -> &T`
    * `[✅]`   From `domain`'s mocks: `build_secret(SecretConstructorParamsOverrides<T>) -> Secret<T>` for `T: Zeroize + Default`, instantiated at `Vec<u8>`
    * `[✅]`   From `blake3`: `Hasher::new_derive_key(context: &str) -> Hasher`, `Hasher::update(&mut self, input: &[u8]) -> &mut Hasher`, `Hasher::finalize_xof(&self) -> OutputReader`, and `OutputReader::fill(&mut self, buf: &mut [u8])`
    * `[✅]`   From `zeroize`: `Zeroize::zeroize(&mut self)` on `Hasher` and `OutputReader`
    * `[ ]`   From `serde_json`: `Value`, `Map<String, Value>`, and `Value::from` over integers, strings, and arrays, in the `mock.rs` files only

  * `[ ]`   `adapters/kdf/Cargo.toml`
    * `[✅]`   `[package]` with `name = "kdf"`, `edition.workspace = true`, `rust-version.workspace = true`, and `publish.workspace = true`; no `version` key
    * `[ ]`   `[dependencies]` with `domain = { path = "../../crates/domain" }`, `zeroize = "1.9.0"`, `blake3 = { version = "1.8.7", features = ["zeroize"] }`, and `serde_json = { version = "1.0.151", optional = true }`
    * `[ ]`   `[dev-dependencies]` with `domain = { path = "../../crates/domain", features = ["mocks"] }` and `serde_json = "1.0.151"`
    * `[ ]`   `[features]` with `mocks = ["domain/mocks", "dep:serde_json"]`
    * `[✅]`   `[lints]` with `workspace = true`
    * `[✅]`   No other table

  * `[✅]`   `adapters/kdf/src/lib.rs`
    * `[✅]`   The crate barrel: `mod blake3_keyed;`, `mod factory;`, and `pub use factory::provides::*;`, nothing else
    * `[✅]`   Until `factory/mod.rs` and `blake3_keyed/mod.rs` exist, `cargo check` reports the unresolved modules, which is the RED state for every element below that precedes them

  * `[ ]`   `adapters/kdf/src/factory/interface.rs`
    * `[✅]`   `KDF_INTERFACE_VERSION`, a `pub const` of type `u32` with value `1`
    * `[ ]`   `KdfIdentifier`, an enum with the variant `Blake3KeyedV1`, the identifier a deployment's hash-card names, and, behind `#[cfg(any(test, feature = "mocks"))]`, the variant `MockV1`, the identifier the family's mock concrete declares
    * `[✅]`   `KdfDeclaration`, a struct with `pub identifier: KdfIdentifier`, `pub adapter_version: u32`, and `pub interface_version: u32`
    * `[✅]`   `DerivationPurpose`, an enum with the variants `WrappingKey`, `PublisherRoot`, `AssetRoot`, `MasterScalar`, `IdentityBases`, `CapsuleRandomness`, `PieceGroupKey`, and `PlaintextRootKey`, the derivation's use, which selects its domain separation
    * `[✅]`   `DeriveKeyParams`, a struct with `pub purpose: DerivationPurpose` and `pub length: usize`, the selection of the derivation and the number of bytes to derive
    * `[✅]`   `DeriveKeyPayload<'a>`, a struct with `pub key_material: &'a Secret<Vec<u8>>` and `pub context: &'a [u8]`, the secret input and the canonical encoding of the context, both borrowed so a caller derives several keys from one secret without copying it
    * `[✅]`   `DeriveKeySuccessReturn`, a struct with `pub key: Secret<Vec<u8>>`
    * `[ ]`   `DeriveKeyErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]` and the variant `Blake3Keyed(Blake3KeyedKdfDeriveKeyErrorReturn)`, the BLAKE3 concrete's error carried unchanged, and, behind `#[cfg(any(test, feature = "mocks"))]`, the variant `Mock(MockIKeyDerivationAdapterDeriveKeyErrorReturn)`, the mock concrete's error carried unchanged; each further concrete's error is its own variant
    * `[✅]`   `DeriveKeyReturn`, the alias `Result<DeriveKeySuccessReturn, DeriveKeyErrorReturn>`
    * `[ ]`   `MockIKeyDerivationAdapterFailureMode`, behind `#[cfg(any(test, feature = "mocks"))]`, an enum with the variants `NoFailure` and `KeyMaterialLengthUnrepresentable`, the failure modes a configuration selects for the family's mock concrete; `KeyMaterialLengthUnrepresentable` stands for the error arm a concrete returns when the key material's length cannot be framed
    * `[✅]`   `IKeyDerivationAdapter`, an object-safe trait with `fn declaration(&self) -> KdfDeclaration;` and `fn derive_key(&self, params: DeriveKeyParams, payload: DeriveKeyPayload<'_>) -> DeriveKeyReturn;`; metadata remains available through `Box<dyn IKeyDerivationAdapter>` and is supplied by that adapter
    * `[ ]`   No derives on any type in this file beyond those stated; imports `domain::Secret` and `Blake3KeyedKdfDeriveKeyErrorReturn` from `crate::blake3_keyed::provides` and, behind `#[cfg(any(test, feature = "mocks"))]`, `MockIKeyDerivationAdapterDeriveKeyErrorReturn` from `super::mock`; names no vendor

  * `[✅]`   `adapters/kdf/src/blake3_keyed/interface.rs`
    * `[✅]`   `Blake3KeyedKdf`, the unit struct `pub struct Blake3KeyedKdf;`, the adapter over `blake3`'s derive-key mode
    * `[✅]`   `Blake3KeyedKdfConstructorParams`, the fieldless struct `pub struct Blake3KeyedKdfConstructorParams;`, the constructor's deps slot
    * `[✅]`   `Blake3KeyedKdfTryNewReturn`, the alias `Result<Blake3KeyedKdf, Infallible>`; the error arm is uninhabited because the adapter takes no configuration
    * `[✅]`   The context strings, each a `pub const` of type `&str`: `BLAKE3_KEYED_WRAPPING_KEY_CONTEXT` with value `"ChainTorrent v1 wrapping-key"`, `BLAKE3_KEYED_PUBLISHER_ROOT_CONTEXT` with value `"ChainTorrent v1 publisher-root"`, `BLAKE3_KEYED_ASSET_ROOT_CONTEXT` with value `"ChainTorrent v1 asset-root"`, `BLAKE3_KEYED_MASTER_SCALAR_CONTEXT` with value `"ChainTorrent v1 master-scalar"`, `BLAKE3_KEYED_IDENTITY_BASES_CONTEXT` with value `"ChainTorrent v1 identity-bases"`, `BLAKE3_KEYED_CAPSULE_RANDOMNESS_CONTEXT` with value `"ChainTorrent v1 capsule-randomness"`, `BLAKE3_KEYED_PIECE_GROUP_KEY_CONTEXT` with value `"ChainTorrent v1 piece-group-key"`, and `BLAKE3_KEYED_PLAINTEXT_ROOT_KEY_CONTEXT` with value `"ChainTorrent v1 plaintext-root"`
    * `[✅]`   `Blake3KeyedKdfDeriveKeyErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]` and the one variant `KeyMaterialLengthExceedsPrefix { length: usize }`, a key material length the 8-byte prefix cannot hold
    * `[✅]`   Imports `core::convert::Infallible`; declares nothing else

  * `[ ]`   `adapters/kdf/src/blake3_keyed/interaction.spec.md`
    * `[✅]`   `Blake3KeyedKdf::try_new(params: Blake3KeyedKdfConstructorParams) -> Blake3KeyedKdfTryNewReturn`: one branch; condition any params; decision none; dependency call none; outcome `Ok(Blake3KeyedKdf)`; the error arm has no branch
    * `[✅]`   `Blake3KeyedKdf::DECLARATION`: the inherent constant `KdfDeclaration { identifier: KdfIdentifier::Blake3KeyedV1, adapter_version: 1, interface_version: KDF_INTERFACE_VERSION }`
    * `[ ]`   `declaration(&self) -> KdfDeclaration`: one branch; condition any adapter; decision none; dependency call none; outcome `Blake3KeyedKdf::DECLARATION`
    * `[✅]`   Purpose mapping: a `match` on `params.purpose`, exhaustive, each variant to its context string constant, `WrappingKey` to `BLAKE3_KEYED_WRAPPING_KEY_CONTEXT`, `PublisherRoot` to `BLAKE3_KEYED_PUBLISHER_ROOT_CONTEXT`, `AssetRoot` to `BLAKE3_KEYED_ASSET_ROOT_CONTEXT`, `MasterScalar` to `BLAKE3_KEYED_MASTER_SCALAR_CONTEXT`, `IdentityBases` to `BLAKE3_KEYED_IDENTITY_BASES_CONTEXT`, `CapsuleRandomness` to `BLAKE3_KEYED_CAPSULE_RANDOMNESS_CONTEXT`, `PieceGroupKey` to `BLAKE3_KEYED_PIECE_GROUP_KEY_CONTEXT`, and `PlaintextRootKey` to `BLAKE3_KEYED_PLAINTEXT_ROOT_KEY_CONTEXT`
    * `[✅]`   `derive_key`, prefix overflow: condition `u64::try_from(payload.key_material.expose().len())` fails; decision the conversion, before any hashing; dependency call none; outcome `Err(DeriveKeyErrorReturn::Blake3Keyed(Blake3KeyedKdfDeriveKeyErrorReturn::KeyMaterialLengthExceedsPrefix { length }))`; `usize` is at most 64 bits on every supported target, so no input takes this branch and it has no unit test
    * `[✅]`   `derive_key`, derived: condition the length converts; decision none further; dependency calls `Hasher::new_derive_key` with the purpose's context string, then `update` with the length's 8 big-endian bytes, `update` with the exposed key material, and `update` with `payload.context`, in that order, each once; then `finalize_xof` once and `fill` once over a zero-initialized buffer of `params.length` bytes; then `zeroize` on the output reader and on the hasher; outcome `Ok(DeriveKeySuccessReturn { key })`, the filled buffer moved into a `Secret` without copy
    * `[✅]`   Ordering: the length conversion precedes the hasher; the prefix, the key material, and the context are absorbed in that order; the reader and the hasher are zeroized before the buffer is moved into the `Secret`; the same params and payload always yield the same key; a `params.length` of zero yields an empty key
    * `[ ]`   Own entry `derive_key_leaves_the_borrowed_key_material_intact`: condition `derive_key` is called with a living `Secret<Vec<u8>>` borrowed in the payload; outcome after the call the secret's `expose()` equals the bytes it held before the call, so the zeroization of the hasher and the output reader, which absorbed those bytes, wipes only their own copies; variation the key material is the reference key material, whose bytes are not all zero, so a zeroized secret reads differently from an intact one; the failure that must survive composition is the hasher's zeroization reaching the caller's secret
    * `[ ]`   Own entry `derive_key_returns_a_key_that_outlives_the_key_material`: condition `derive_key` returns `Ok` and the key material's secret is then dropped, its drop zeroizing its value; outcome the returned key's `expose()` still equals the derived bytes; variation the key material's secret is dropped after the call and before the key is read, so a returned key that shares storage with the key material reads as zeros; the failure that must survive composition is the returned `Secret` aliasing any input's storage
    * `[ ]`   Private surface: the chain of real functions is `build_blake3_keyed_kdf` through `Blake3KeyedKdf::try_new`, then `Blake3KeyedKdf::derive_key`, which runs `blake3::Hasher`, `blake3::OutputReader`, and `Zeroize::zeroize` for real and moves the buffer into a `Secret` through `Secret::try_new`; the input and output secrets are `domain` `Secret` values whose `expose` and drop run for real; no outer-edge collaborator is mocked; the observable result is the bytes read through `expose()` of the key material before and after the call and of the returned key after the key material is dropped

  * `[ ]`   `adapters/kdf/src/factory/mock.rs`
    * `[ ]`   Imports the family's names from `super::interface`, `Secret` from `domain`, `build_secret` and `SecretConstructorParamsOverrides` from `domain`'s mocks, and `serde_json::Value` and `serde_json::Map`
    * `[ ]`   The reference derivation inputs, defined here and public so the concrete's mock reads the same values the builders default to: `reference_key_material_bytes() -> Vec<u8>`, 32 bytes whose byte at index `i` is `i`; `reference_key_material() -> &'static Secret<Vec<u8>>`, built once by `build_secret` in a `OnceLock` from `reference_key_material_bytes()`; `reference_context() -> &'static [u8]`, 48 bytes whose byte at index `i` is `128 + i`; `REFERENCE_LENGTH`, a `pub const` of type `usize` with value `64`; and `REFERENCE_PREFIX_LENGTH`, a `pub const` of type `usize` with value `32`; the key material and the context differ in length, so a derivation that frames them wrongly yields different bytes
    * `[ ]`   Every invalidator in this file returns `serde_json::Value`, a JSON object holding the representation of the built default's fields with each present corruption replacing its key: an enum as its variant name text by an exhaustive `match`, an integer as a number, a byte slice as an array of byte numbers, and a `Secret<Vec<u8>>` as the array of the bytes its `expose()` returns; no production type is serialized, since `Secret` has no serialization and no type in the interface derives it
    * `[ ]`   `KdfDeclarationOverrides`, `#[derive(Default)]`, one `Option` per field of `KdfDeclaration`; `build_kdf_declaration(overrides: KdfDeclarationOverrides) -> KdfDeclaration`, the defaults the mock concrete's own declaration: `identifier` `KdfIdentifier::MockV1`, `adapter_version` the mock concrete's adapter version, and `interface_version` `KDF_INTERFACE_VERSION`; `KdfDeclarationCorruptions`, `#[derive(Default)]`, one `Option<serde_json::Value>` per field; `invalidate_kdf_declaration(corruptions: KdfDeclarationCorruptions) -> serde_json::Value` over `build_kdf_declaration` with no override
    * `[ ]`   `DeriveKeyParamsOverrides`, `#[derive(Default)]`, one `Option` per field of `DeriveKeyParams`; `build_derive_key_params(overrides: DeriveKeyParamsOverrides) -> DeriveKeyParams`, the defaults `purpose` `DerivationPurpose::WrappingKey` and `length` `REFERENCE_LENGTH`; `DeriveKeyParamsCorruptions`, `#[derive(Default)]`, one `Option<serde_json::Value>` per field; `invalidate_derive_key_params(corruptions: DeriveKeyParamsCorruptions) -> serde_json::Value` over `build_derive_key_params` with no override
    * `[ ]`   `DeriveKeyPayloadOverrides<'a>`, `#[derive(Default)]`, one `Option` per field of `DeriveKeyPayload<'a>`; `build_derive_key_payload<'a>(overrides: DeriveKeyPayloadOverrides<'a>) -> DeriveKeyPayload<'a>`, the defaults `key_material` `reference_key_material()` and `context` `reference_context()`; `DeriveKeyPayloadCorruptions`, `#[derive(Default)]`, one `Option<serde_json::Value>` per field; `invalidate_derive_key_payload(corruptions: DeriveKeyPayloadCorruptions) -> serde_json::Value` over `build_derive_key_payload` with no override
    * `[ ]`   `DeriveKeySuccessReturnOverrides`, `#[derive(Default)]`, one `Option` per field of `DeriveKeySuccessReturn`; `build_derive_key_success_return(overrides: DeriveKeySuccessReturnOverrides) -> DeriveKeySuccessReturn`, the default `key` from `build_secret` with no override; `DeriveKeySuccessReturnCorruptions`, `#[derive(Default)]`, one `Option<serde_json::Value>` per field; `invalidate_derive_key_success_return(corruptions: DeriveKeySuccessReturnCorruptions) -> serde_json::Value` over `build_derive_key_success_return` with no override
    * `[ ]`   `MockIKeyDerivationAdapterDeriveKeyErrorReturn`, a `pub` enum with `#[derive(Debug, PartialEq, Eq)]` and the one variant `KeyMaterialLengthUnrepresentable { length: usize }`, the mock concrete's own error
    * `[ ]`   `MockIKeyDerivationAdapterConstructorParams`, a `pub(crate)` struct with one field `pub failure_mode: MockIKeyDerivationAdapterFailureMode`, the selecting param; `MockIKeyDerivationAdapterTryNewReturn`, the alias `Result<MockIKeyDerivationAdapter, Infallible>`
    * `[ ]`   `MockIKeyDerivationAdapter`, a `pub(crate)` struct holding the failure mode in a private field, the family's mock concrete: self-contained, taking no concrete as a type parameter and wrapping, delegating to, or borrowing from none; `MockIKeyDerivationAdapter::DECLARATION`, the inherent constant `KdfDeclaration { identifier: KdfIdentifier::MockV1, adapter_version, interface_version: KDF_INTERFACE_VERSION }`; `MockIKeyDerivationAdapter::try_new(params: MockIKeyDerivationAdapterConstructorParams) -> MockIKeyDerivationAdapterTryNewReturn`, returning `Ok` holding the params' failure mode
    * `[ ]`   `impl IKeyDerivationAdapter for MockIKeyDerivationAdapter`: `declaration()` returns `Self::DECLARATION`; `derive_key` under the failure mode `KeyMaterialLengthUnrepresentable` returns `Err(DeriveKeyErrorReturn::Mock(MockIKeyDerivationAdapterDeriveKeyErrorReturn::KeyMaterialLengthUnrepresentable { length }))` with `length` the exposed key material's length, whatever the inputs; under `NoFailure` it converts the key material's length to a `u64` through `u64::try_from`, a conversion failure returning that same error, then computes exactly `params.length` bytes with the standard library's `DefaultHasher` over the purpose, the length as 8 big-endian bytes, the key material, and the context, in that order, expanded by a running counter so each byte is a function of those inputs and its position alone, and returns `Ok(build_derive_key_success_return(..))` with the key a `build_secret` over those bytes; identical params and payload yield identical bytes, a shorter output is the prefix of a longer one, and a length of zero yields an empty key
    * `[ ]`   `KDF_INTERFACE_VERSION`, `KdfIdentifier`, `DerivationPurpose`, `MockIKeyDerivationAdapterFailureMode`, and `DeriveKeyErrorReturn` are used by their production value or type with no mock; `DeriveKeyReturn` is an alias with no mock; `IKeyDerivationAdapter` is answered by `MockIKeyDerivationAdapter`

  * `[✅]`   `adapters/kdf/src/factory/mod.rs`
    * `[✅]`   Module wiring only: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, and `pub mod provides;`, nothing else

  * `[✅]`   `adapters/kdf/src/factory/provides.rs`
    * `[✅]`   `pub use super::interface::*;` and `#[cfg(any(test, feature = "mocks"))] pub use super::mock::*;`, and `#[cfg(any(test, feature = "mocks"))] pub use crate::blake3_keyed::provides::expected_derived_key;`, the oracle the `blake3_keyed` concrete authors, so the crate's public mock surface carries the oracle and no other item of the concrete, nothing else

  * `[ ]`   `adapters/kdf/src/blake3_keyed/mock.rs`
    * `[ ]`   Module-level `#![allow(clippy::expect_used)]`; imports `Blake3KeyedKdf` and `Blake3KeyedKdfConstructorParams` from `super::interface`, `DerivationPurpose` and `reference_key_material` from `crate::factory::provides`, `blake3::Hasher`, and `serde_json::Value` and `serde_json::Map`; `expected_derived_key` and the imports only it uses, `DerivationPurpose`, `reference_key_material`, and `blake3::Hasher`, are available under `#[cfg(any(test, feature = "mocks"))]`, and every other item and import in this file is under `#[cfg(test)]`
    * `[ ]`   `Blake3KeyedKdfConstructorParamsOverrides`, `#[derive(Default)]`, fieldless, since the params declare no field; `build_blake3_keyed_kdf_constructor_params(overrides: Blake3KeyedKdfConstructorParamsOverrides) -> Blake3KeyedKdfConstructorParams`
    * `[ ]`   `Blake3KeyedKdfConstructorParamsCorruptions`, `#[derive(Default)]`, fieldless; `invalidate_blake3_keyed_kdf_constructor_params(corruptions: Blake3KeyedKdfConstructorParamsCorruptions) -> serde_json::Value`, returning the empty JSON object, the representation of an object type that declares no field
    * `[ ]`   `build_blake3_keyed_kdf(overrides: Blake3KeyedKdfConstructorParamsOverrides) -> Blake3KeyedKdf`, returning the real instance from `Blake3KeyedKdf::try_new(build_blake3_keyed_kdf_constructor_params(overrides))` through the irrefutable pattern `let Ok(adapter) = …;`
    * `[ ]`   `expected_derived_key(purpose: DerivationPurpose, context: &[u8], length: usize) -> Vec<u8>`, a `pub` function available under `#[cfg(any(test, feature = "mocks"))]`, the oracle the unit tests of this concrete and the tests of the factory assert against, computed independently of `derive_key`: the context string comes from an exhaustive `match` on the purpose over text literals written in this file, never the production constants, `WrappingKey` to `"ChainTorrent v1 wrapping-key"`, `PublisherRoot` to `"ChainTorrent v1 publisher-root"`, `AssetRoot` to `"ChainTorrent v1 asset-root"`, `MasterScalar` to `"ChainTorrent v1 master-scalar"`, `IdentityBases` to `"ChainTorrent v1 identity-bases"`, `CapsuleRandomness` to `"ChainTorrent v1 capsule-randomness"`, `PieceGroupKey` to `"ChainTorrent v1 piece-group-key"`, and `PlaintextRootKey` to `"ChainTorrent v1 plaintext-root"`; the input is one buffer concatenating the length of `reference_key_material().expose()` as 8 big-endian bytes through `u64::try_from` and `u64::to_be_bytes`, then those key material bytes, then `context`; the output is `Hasher::new_derive_key` over the context string, one `update` over the whole buffer, `finalize_xof`, and `fill` over a zero-initialized buffer of `length` bytes
    * `[ ]`   The context string constants, `Blake3KeyedKdfTryNewReturn`, and `Blake3KeyedKdfDeriveKeyErrorReturn` are used by their production value or type with no mock; `Blake3KeyedKdf` takes no overrides, invalidator, or mock function, and the trait it implements is answered by the family's mock concrete

  * `[ ]`   `adapters/kdf/src/blake3_keyed/test.rs`
    * `[ ]`   Module-level `#![allow(clippy::expect_used)]`; imports `Blake3KeyedKdf`, `build_blake3_keyed_kdf`, `build_blake3_keyed_kdf_constructor_params`, and `expected_derived_key` from `super::provides`, and `IKeyDerivationAdapter`, `DerivationPurpose`, `KdfIdentifier`, `KDF_INTERFACE_VERSION`, `REFERENCE_LENGTH`, `REFERENCE_PREFIX_LENGTH`, `reference_context`, `DeriveKeyParamsOverrides`, `build_derive_key_params`, and `build_derive_key_payload` from `crate::factory::provides`
    * `[ ]`   `try_new_returns_the_adapter`
      * `[ ]`   Contract: any params → `Ok(Blake3KeyedKdf)`
      * `[ ]`   Collaborators: none; the fixture is `build_blake3_keyed_kdf_constructor_params`
      * `[ ]`   Arrange: `build_blake3_keyed_kdf_constructor_params` with no override, since the params declare no field
      * `[ ]`   Act: `Blake3KeyedKdf::try_new` on the built params
      * `[ ]`   Assert: the result matches `Ok(Blake3KeyedKdf)`
    * `[ ]`   `declaration_returns_the_blake3_keyed_declaration`
      * `[ ]`   Contract: any adapter → `Blake3KeyedKdf::DECLARATION`, the identifier `KdfIdentifier::Blake3KeyedV1`, the adapter version the interaction spec states, and the interface version `KDF_INTERFACE_VERSION`
      * `[ ]`   Collaborators: none; the fixture is `build_blake3_keyed_kdf`
      * `[ ]`   Arrange: `build_blake3_keyed_kdf` with no override
      * `[ ]`   Act: `adapter.declaration()`
      * `[ ]`   Assert: the identifier matches `KdfIdentifier::Blake3KeyedV1`; the adapter version equals the literal the interaction spec's `DECLARATION` entry states; the interface version equals `KDF_INTERFACE_VERSION`
    * `[ ]`   `derive_key_for_{purpose}_matches_the_expected_derived_key`, one block per variant of `DerivationPurpose`, each purpose written in snake case in its name
      * `[ ]`   Contract: the length converts → `Ok` with a key of the requested length derived under the purpose's context string over the length prefix, the key material, and the context, in that order
      * `[ ]`   Collaborators: `blake3::Hasher` and `blake3::OutputReader`, imported vendor types with no mock, run for real; `expected_derived_key` is the independent oracle the expectation comes from; the reference key material and context come from `build_derive_key_payload`
      * `[ ]`   Arrange: `build_blake3_keyed_kdf` with no override; `build_derive_key_params` with the purpose override set to the variant under test, so the expected key differs per block; `build_derive_key_payload` with no override
      * `[ ]`   Act: `adapter.derive_key(params, payload)`
      * `[ ]`   Assert: the success arm is bound through `expect`; the key's `expose()` equals `expected_derived_key(variant under test, reference_context(), REFERENCE_LENGTH)`
    * `[ ]`   `derive_key_of_the_prefix_length_is_the_prefix_of_the_reference_length_output`
      * `[ ]`   Contract: the length converts, a requested length shorter than the reference length → `Ok` with a key that is the leading bytes of the longer output
      * `[ ]`   Collaborators: `blake3::Hasher` and `blake3::OutputReader`, run for real; `expected_derived_key` is the independent oracle
      * `[ ]`   Arrange: `build_blake3_keyed_kdf` with no override; `build_derive_key_params` with the purpose override `DerivationPurpose::WrappingKey` and the length override `REFERENCE_PREFIX_LENGTH`, so the requested length differs from the reference length; `build_derive_key_payload` with no override
      * `[ ]`   Act: `adapter.derive_key(params, payload)`
      * `[ ]`   Assert: the success arm is bound through `expect`; the key's `expose()` equals the first `REFERENCE_PREFIX_LENGTH` bytes of `expected_derived_key(DerivationPurpose::WrappingKey, reference_context(), REFERENCE_LENGTH)`
    * `[ ]`   `derive_key_of_zero_bytes_returns_an_empty_key`
      * `[ ]`   Contract: the length converts, `params.length` of zero → `Ok` with an empty key
      * `[ ]`   Collaborators: `blake3::Hasher` and `blake3::OutputReader`, run for real; the expectation is the empty byte sequence, stated in the assertion
      * `[ ]`   Arrange: `build_blake3_keyed_kdf` with no override; `build_derive_key_params` with the length override zero, so the length differs from `REFERENCE_LENGTH`; `build_derive_key_payload` with no override
      * `[ ]`   Act: `adapter.derive_key(params, payload)`
      * `[ ]`   Assert: the success arm is bound through `expect`; the key's `expose()` equals the empty byte sequence

  * `[ ]`   `construction`
    * `[✅]`   `Blake3KeyedKdf::try_new` is the concrete's only producer, and its only caller is the key-derivation factory, which reads `Blake3KeyedKdf::DECLARATION` before constructing
    * `[ ]`   The adapter is a unit struct constructed by `try_new` over fieldless params; `build_blake3_keyed_kdf` in `blake3_keyed/mock.rs` builds it as a real instance for the concrete's own unit tests, and no consumer names the concrete

  * `[ ]`   `adapters/kdf/src/blake3_keyed/mod.rs`
    * `[ ]`   Module declarations: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, `pub(crate) mod provides;`, `#[cfg(test)] mod test;`, and `#[cfg(test)] mod integration_test;`
    * `[✅]`   `impl Blake3KeyedKdf` with `pub const DECLARATION: KdfDeclaration` as the interaction spec states and `pub fn try_new(_params: Blake3KeyedKdfConstructorParams) -> Blake3KeyedKdfTryNewReturn` returning `Ok(Blake3KeyedKdf)`
    * `[✅]`   `impl IKeyDerivationAdapter for Blake3KeyedKdf` with `declaration()` returning `Self::DECLARATION` and `derive_key` realizing the purpose mapping, branches, and ordering of the interaction spec; the buffer is `vec![0u8; params.length]`, and it is moved into the `Secret` by `let Ok(key) = Secret::try_new(SecretConstructorParams { value: buffer });`
    * `[✅]`   Imports the family's names from `crate::factory::provides`, `Secret` and `SecretConstructorParams` from `domain`, `zeroize::Zeroize`, `blake3::Hasher`, and this module's names from `interface`
    * `[✅]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[ ]`   `adapters/kdf/src/blake3_keyed/integration_test.rs`
    * `[ ]`   Module-level `#![allow(clippy::expect_used)]`; imports `build_blake3_keyed_kdf` and `expected_derived_key` from `super::provides`, `IKeyDerivationAdapter`, `DerivationPurpose`, `REFERENCE_LENGTH`, `reference_context`, `reference_key_material_bytes`, `DeriveKeyParamsOverrides`, `DeriveKeyPayloadOverrides`, `build_derive_key_params`, and `build_derive_key_payload` from `crate::factory::provides`, and `build_secret` and `SecretConstructorParamsOverrides` from `domain`
    * `[ ]`   `derive_key_leaves_the_borrowed_key_material_intact`
      * `[ ]`   Contract: `derive_key` is called with a living `Secret<Vec<u8>>` borrowed in the payload → after the call the secret's `expose()` equals the bytes it held before the call
      * `[ ]`   Arrange: `build_secret` with the value override `reference_key_material_bytes()`, whose bytes are not all zero, so a zeroized secret differs from an intact one; `build_blake3_keyed_kdf` with no override; `build_derive_key_params` with the purpose override `DerivationPurpose::WrappingKey`; `build_derive_key_payload` with the key material override set to a borrow of that secret
      * `[ ]`   Act: `adapter.derive_key(params, payload)`
      * `[ ]`   Assert: the success arm is bound through `expect`; the secret's `expose()` equals `reference_key_material_bytes()`
      * `[ ]`   Boundary: the crate-internal path `Blake3KeyedKdf::derive_key` through `build_blake3_keyed_kdf` and `Blake3KeyedKdf::try_new`, running `blake3::Hasher`, `blake3::OutputReader`, `Zeroize::zeroize`, and `Secret::try_new`, with `Secret::expose` reading the result
      * `[ ]`   Mocked: nothing; `blake3`, `zeroize`, and `domain` run for real, so this block does not prove the zeroization of the hasher and the reader themselves, which no caller can observe
    * `[ ]`   `derive_key_returns_a_key_that_outlives_the_key_material`
      * `[ ]`   Contract: `derive_key` returns `Ok` and the key material's secret is then dropped, its drop zeroizing its value → the returned key's `expose()` still equals the derived bytes
      * `[ ]`   Arrange: `build_secret` with the value override `reference_key_material_bytes()`, so the dropped secret holds non-zero bytes that its drop overwrites; `build_blake3_keyed_kdf` with no override; `build_derive_key_params` with the purpose override `DerivationPurpose::WrappingKey`; `build_derive_key_payload` with the key material override set to a borrow of that secret
      * `[ ]`   Act: `adapter.derive_key(params, payload)`
      * `[ ]`   Assert: the success arm is bound through `expect`; after `drop` of the key material's secret, the returned key's `expose()` equals `expected_derived_key(DerivationPurpose::WrappingKey, reference_context(), REFERENCE_LENGTH)`
      * `[ ]`   Boundary: the crate-internal path `Blake3KeyedKdf::derive_key` through `build_blake3_keyed_kdf` and `Blake3KeyedKdf::try_new`, running `blake3::Hasher`, `blake3::OutputReader`, `Zeroize::zeroize`, and `Secret::try_new`, with the key material's `Secret` drop running for real
      * `[ ]`   Mocked: nothing; `blake3`, `zeroize`, and `domain` run for real

  * `[ ]`   `adapters/kdf/src/blake3_keyed/provides.rs`
    * `[ ]`   `pub(crate) use super::interface::*;`, `#[cfg(test)] pub(crate) use super::mock::*;`, and `#[cfg(any(test, feature = "mocks"))] pub use super::mock::expected_derived_key;`, nothing else, so the concrete is visible to the crate's factory and to nothing outside the crate, its unit test reaches its mock through this file, and the oracle is the one item the factory's `provides` can re-export

  * `[✅]`   `directionality`
    * `[✅]`   `blake3_keyed` depends on the `factory` module's surface through `crate::factory::provides`, on `domain`, on `zeroize`, and on `blake3`; the `factory` module depends on `domain` and on `blake3_keyed`'s error through `crate::blake3_keyed::provides`, the family form's recorded cycle, which `kdf/factory` completes by constructing the concrete; among repository crates the crate depends on `crates/domain` alone; nothing depends on the crate yet

  * `[ ]`   `requirements`
    * `[✅]`   `adapters/kdf/Cargo.toml` carries exactly the tables and keys stated above, and `blake3` is named nowhere in the crate outside `adapters/kdf/src/blake3_keyed`
    * `[ ]`   `DeriveKeyErrorReturn`, `Blake3KeyedKdfDeriveKeyErrorReturn`, and `MockIKeyDerivationAdapterDeriveKeyErrorReturn` derive `Debug`, `PartialEq`, and `Eq`
    * `[✅]`   `cargo check --workspace --all-targets --all-features`, `cargo fmt --all --check`, and `cargo deny check` complete without error; `cargo clippy --workspace --all-targets --all-features` reports nothing beyond the library target's unused-item warnings for the `blake3_keyed` concrete, which `kdf/factory` resolves by constructing the concrete
    * `[ ]`   Every `derive_key_for_…_matches_the_expected_derived_key` test passes (CR-11, context strings, serialization, and output frozen against the independent oracle; CR-05 for the lineage purposes)
    * `[ ]`   `try_new_returns_the_adapter`, `declaration_returns_the_blake3_keyed_declaration`, `derive_key_of_the_prefix_length_is_the_prefix_of_the_reference_length_output`, and `derive_key_of_zero_bytes_returns_an_empty_key` pass
    * `[ ]`   `derive_key_leaves_the_borrowed_key_material_intact` and `derive_key_returns_a_key_that_outlives_the_key_material` pass
    * `[✅]`   Code outside `adapters/kdf` naming `Blake3KeyedKdf` or anything under `blake3_keyed` fails to compile; the crate's public surface is the `factory` module's `provides`

* `[ ]`   `kdf/factory` **Key-derivation factory constructing the concrete the configuration names, admitted against the KDF identifier the hash-card requires, and returning it behind the family's trait, which reports its declaration; carries the family's integration test**

  * `[✅]`   `objective`
    * `[✅]`   Problem: a consumer obtains a key-derivation adapter only through the family's generic surface, never by naming a concrete, and the derivation it uses is the one the deployment's hash-card names, so a concrete that does not declare the required KDF identifier is refused before anything is constructed (CR-11; Composition Boundary)
    * `[✅]`   Functional: given the concrete the configuration names and the KDF identifier required, the factory refuses a concrete whose declared identifier is not the required one, with no construction
    * `[✅]`   Functional: an admitted concrete is constructed and returned as `Box<dyn IKeyDerivationAdapter>` that reports its declaration
    * `[✅]`   Functional: a concrete's constructor error is returned unchanged in the factory's error arm, one variant per concrete
    * `[✅]`   Functional: the concrete the factory returns derives the wrapping key over the reference context encoded through the encoding factory, matching the independent oracle `expected_derived_key`
    * `[✅]`   Non-functional: adding a concrete is its module, its variant in the selection enum and in the error enum, its branch here, and its entry in the declared set; adding an identifier is its variant in `KdfIdentifier`; no consumer changes

  * `[✅]`   `role`
    * `[✅]`   Adapter family factory: the implementation of the `factory` module, the key-derivation family's construction point, and the crate's public surface
    * `[✅]`   Returns the concrete behind `Box<dyn IKeyDerivationAdapter>`, because the family's trait has no generic method and no associated type and so is dyn-compatible, as the randomness factory returns its source
    * `[✅]`   Selects by concrete and admits by identifier, so a further concrete implementing an existing identifier and a further identifier each arrive as a variant and a branch, with no change to the params or to any consumer
    * `[✅]`   Does not read a hash-card or the configuration; the composition resolver passes the concrete the configuration names as a typed `KdfConcrete` and the identifier the hash-card requires as a typed `KdfIdentifier`
    * `[✅]`   Does not derive; derivation is the concrete's
    * `[✅]`   Carries the family's integration test across factory, concrete, and the encoding family that produces the context; does not carry a commit

  * `[✅]`   `module`
    * `[✅]`   Bounded context: the `factory` module of `adapters/kdf`, holding the factory function, its deps, params, payload, and return types, its signature type, the selection enum and the declared set, the function mock and builders, the module's private integration test, and the crate's public integration test under `adapters/kdf/tests`
    * `[✅]`   Outside: the concrete's behavior, the encoding of any context, the hash-card, the configuration catalogue, and every consumer of the family

  * `[✅]`   `deps`
    * `[✅]`   The `blake3_keyed` concrete, through `crate::blake3_keyed::provides`: `Blake3KeyedKdf`, `Blake3KeyedKdfConstructorParams`, `Blake3KeyedKdf::try_new`, and `Blake3KeyedKdf::DECLARATION`; the factory constructs its concrete, completing the family form's recorded cycle that `kdf/blake3_keyed` opened
    * `[✅]`   The `factory` module's own interface: `IKeyDerivationAdapter`, `KdfIdentifier`, `KdfDeclaration`, and `KDF_INTERFACE_VERSION`
    * `[✅]`   The `factory` module's own mock through `provides` under the `mocks` feature, in `factory/test.rs`, `factory/integration_test.rs`, and `tests/factory_integration_test.rs` only: the builders and overrides of this module, the builders of the family's interface, `reference_key_material_bytes`, `REFERENCE_LENGTH`, `REFERENCE_PREFIX_LENGTH`, `MockIKeyDerivationAdapterDeriveKeyErrorReturn`, the declared set `DECLARED_KDF_SELECTIONS`, and the oracle `expected_derived_key` that `kdf/blake3_keyed` authors and the factory's mock surface re-exports, taking the purpose, the context bytes, and the length over the reference key material; the tests reach the family only through `create_key_derivation` and name no concrete
    * `[✅]`   `domain` with its `mocks` feature, the existing dev-dependency, in `factory/integration_test.rs` and `tests/factory_integration_test.rs` only: `build_secret` with `SecretConstructorParamsOverrides`, and in the public integration test the builders that make the reference derivation context
    * `[✅]`   `core::convert::Infallible`, standard library, the concrete's constructor error carried in the factory's error arm

  * `[ ]`   `context_slice`
    * `[✅]`   From the concrete: `Blake3KeyedKdf::try_new(Blake3KeyedKdfConstructorParams) -> Result<Blake3KeyedKdf, Infallible>`, the inherent constant `Blake3KeyedKdf::DECLARATION: KdfDeclaration`, and `Blake3KeyedKdf`'s implementation of `IKeyDerivationAdapter`
    * `[ ]`   From the family's interface and this module's mock, in the tests: `IKeyDerivationAdapter::declaration(&self) -> KdfDeclaration` and `IKeyDerivationAdapter::derive_key(&self, DeriveKeyParams, DeriveKeyPayload<'_>) -> DeriveKeyReturn`, `build_derive_key_params` with `DeriveKeyParamsOverrides`, `build_derive_key_payload` with `DeriveKeyPayloadOverrides`, and `expected_derived_key`

  * `[✅]`   `adapters/kdf/Cargo.toml`
    * `[✅]`   `[dev-dependencies]` reads `domain = { path = "../../crates/domain", features = ["mocks"] }`, `serde_json = "1.0.151"`, and `encoding = { path = "../encoding", features = ["mocks"] }`
    * `[✅]`   `[package]`, `[dependencies]`, `[features]`, and `[lints]` are unchanged; no other table

  * `[✅]`   `adapters/kdf/src/factory/interface.rs`
    * `[✅]`   `KdfIdentifier` gains `#[derive(Clone, Copy, PartialEq, Eq)]`, so the admission compares a declared identifier with the required one and a declared selection is read by value
    * `[✅]`   `MockIKeyDerivationAdapterFailureMode` gains `#[derive(Clone, Copy)]`, so the selection that carries it is read by value
    * `[✅]`   `KdfConcrete`, an enum with `#[derive(Clone, Copy)]`, the variant `Blake3Keyed`, and, behind `#[cfg(any(test, feature = "mocks"))]`, the variant `Mock(MockIKeyDerivationAdapterFailureMode)`, the selection of the family's mock concrete carrying the failure mode the configuration names
    * `[✅]`   `CreateKeyDerivationDeps`, the fieldless struct `pub struct CreateKeyDerivationDeps;`
    * `[✅]`   `CreateKeyDerivationParams`, a struct with `#[derive(Clone, Copy)]`, `pub concrete: KdfConcrete` and `pub identifier: KdfIdentifier`, the selection and the identifier the concrete must declare
    * `[✅]`   `CreateKeyDerivationPayload`, the fieldless struct `pub struct CreateKeyDerivationPayload;`, since the factory operates on no data
    * `[✅]`   `CreateKeyDerivationSuccessReturn`, a struct with only `pub adapter: Box<dyn IKeyDerivationAdapter>`; callers read the declaration through `adapter.declaration()`
    * `[✅]`   `CreateKeyDerivationErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]`, which `Infallible` satisfies, and the variants `UnsupportedKdfIdentifier`, the named concrete not declaring the required identifier, and `Blake3Keyed(Infallible)`, the BLAKE3 concrete's constructor error carried unchanged, and, behind `#[cfg(any(test, feature = "mocks"))]`, `Mock(Infallible)`, the mock concrete's constructor error carried unchanged; each further concrete's constructor error is its own variant
    * `[✅]`   `CreateKeyDerivationReturn`, the alias `Result<CreateKeyDerivationSuccessReturn, CreateKeyDerivationErrorReturn>`
    * `[✅]`   `CreateKeyDerivationFn`, the alias `fn(&CreateKeyDerivationDeps, CreateKeyDerivationParams, CreateKeyDerivationPayload) -> CreateKeyDerivationReturn`
    * `[✅]`   `DECLARED_KDF_SELECTIONS`, a `pub const` slice of `CreateKeyDerivationParams` holding, for each real concrete, its selection and the identifier it declares, here `CreateKeyDerivationParams { concrete: KdfConcrete::Blake3Keyed, identifier: KdfIdentifier::Blake3KeyedV1 }`, the declared set, which never holds the mock selection
    * `[✅]`   Adds the import of `core::convert::Infallible`; every item `kdf/blake3_keyed` authored in this file is unchanged except the derives on `KdfIdentifier` and `MockIKeyDerivationAdapterFailureMode`

  * `[✅]`   `adapters/kdf/src/factory/interaction.spec.md`
    * `[✅]`   `create_key_derivation(deps: &CreateKeyDerivationDeps, params: CreateKeyDerivationParams, payload: CreateKeyDerivationPayload) -> CreateKeyDerivationReturn`: decision a `match` on `params.concrete`, one arm per `KdfConcrete` variant, exhaustive so a variant with no arm fails to compile
    * `[✅]`   Unsupported identifier: condition the named concrete's `DECLARATION.identifier` is not `params.identifier`; decision equality, read before any construction; dependency call none; outcome `Err(CreateKeyDerivationErrorReturn::UnsupportedKdfIdentifier)`, with nothing constructed; under the `mocks` feature the mock concrete declares `KdfIdentifier::MockV1`, so a `Mock` selection paired with another concrete's identifier takes this branch
    * `[✅]`   Admitted: condition the named concrete's declared identifier is `params.identifier`; dependency call the concrete's `try_new` with its fieldless constructor params, exactly once, its success destructured irrefutably because its error arm is uninhabited; outcome `Ok(CreateKeyDerivationSuccessReturn { adapter: Box::new(kdf) })`, whose adapter reports the concrete's `DECLARATION`
    * `[ ]`   `Mock` arm, under `#[cfg(any(test, feature = "mocks"))]`: the same admission over `MockIKeyDerivationAdapter::DECLARATION.identifier`; dependency call the mock concrete's `try_new` with `MockIKeyDerivationAdapterConstructorParams { failure_mode }`, the failure mode the selection carries, exactly once, its success destructured irrefutably because its error arm is uninhabited; outcome `Ok(CreateKeyDerivationSuccessReturn { adapter: Box::new(mock) })`, whose adapter reports `KdfIdentifier::MockV1` and derives under the failure mode
    * `[✅]`   `CreateKeyDerivationErrorReturn::Blake3Keyed` and `CreateKeyDerivationErrorReturn::Mock` carry the constructors' uninhabited error type in the return union, so no branch produces them
    * `[✅]`   `params.concrete` selects and `params.identifier` admits; `deps` and `payload` carry nothing and are not read
    * `[ ]`   Callee disposition, `kdf/blake3_keyed` entry `derive_key_leaves_the_borrowed_key_material_intact`: absorbed, proven by `every_concrete_leaves_the_borrowed_key_material_intact` in the private integration test element `factory/integration_test.rs`, which runs the entry through `create_key_derivation` over the declared set and the mock selection
    * `[ ]`   Callee disposition, `kdf/blake3_keyed` entry `derive_key_returns_a_key_that_outlives_the_key_material`: absorbed, proven by `every_concrete_returns_a_key_that_outlives_the_key_material` in the private integration test element `factory/integration_test.rs`, which runs the entry through `create_key_derivation` over the declared set and the mock selection
    * `[ ]`   Own entry `every_concrete_is_constructed_under_the_identifier_it_declares`: condition `create_key_derivation` is called with the selection of a concrete and the identifier that concrete declares; outcome `Ok(CreateKeyDerivationSuccessReturn { adapter })` with `adapter.declaration()` reporting the identifier required and the interface version `KDF_INTERFACE_VERSION`; variation each case of the declared set and the mock selection, whose declared identifiers differ, so a factory that constructs a concrete other than the selected one reports another identifier; edge that must survive the composition: every concrete the factory admits declares the interface version the family's consumers are written against
    * `[ ]`   Own entry `every_concrete_is_refused_under_an_identifier_it_does_not_declare`: condition `create_key_derivation` is called with the selection of a concrete and an identifier that concrete does not declare; outcome `Err(CreateKeyDerivationErrorReturn::UnsupportedKdfIdentifier)`; variation each case paired with the declared identifier of every other case whose declared identifier differs from its own, so an admission that compares against a constant or ignores the selection admits a pairing it must refuse; edge that must survive the composition: the refusal is decided by the selected concrete's own declaration
    * `[ ]`   Own entry `every_concrete_derives_a_key_of_the_requested_length`: condition the adapter `create_key_derivation` returns derives with a `params.length`; outcome `Ok(DeriveKeySuccessReturn { key })` with the key's byte length equal to `params.length`; variation each case of the declared set and the mock selection over `REFERENCE_LENGTH`, `REFERENCE_PREFIX_LENGTH`, and zero, so a concrete that returns the reference length whatever is requested, or a non-empty key for zero, fails; edge that must survive the composition: a `params.length` of zero returns an empty key and no refusal
    * `[ ]`   Own entry `every_concrete_returns_its_own_error_when_the_key_material_length_cannot_be_framed`: condition the adapter `create_key_derivation` returns for the mock selection under the failure mode `KeyMaterialLengthUnrepresentable` derives; outcome `Err(DeriveKeyErrorReturn::Mock(MockIKeyDerivationAdapterDeriveKeyErrorReturn::KeyMaterialLengthUnrepresentable { length }))` with `length` the key material's length; variation the mock selection under that failure mode, the case that reaches this arm, since a `usize` fits the 8-byte prefix on every supported target and no input takes it for a real concrete; edge that must survive the composition: the failure mode the configuration names is the failure mode of the concrete the factory constructs, and the concrete's own error variant arrives whole in the family's error union
    * `[ ]`   Own entry `every_concrete_declaring_the_blake3_keyed_identifier_derives_the_wrapping_key_over_the_reference_inputs`: condition `create_key_derivation` is called with the selection of a concrete that declares `KdfIdentifier::Blake3KeyedV1` and that identifier, and the adapter derives with `DerivationPurpose::WrappingKey` and `REFERENCE_LENGTH` over the reference key material and the reference context; outcome the key equals `expected_derived_key` over the wrapping-key purpose, the reference context, and `REFERENCE_LENGTH`; variation each case of the declared set that declares the identifier; edge that must survive the composition: the concrete the factory constructs derives, through the family's trait object, the key the independent oracle states
    * `[ ]`   Own entry `every_concrete_declaring_the_blake3_keyed_identifier_derives_the_wrapping_key_over_the_context_the_encoding_factory_encodes`: condition `create_key_derivation` is called with the selection of a concrete that declares `KdfIdentifier::Blake3KeyedV1` and that identifier, and the adapter derives with `DerivationPurpose::WrappingKey` and `REFERENCE_LENGTH` over the reference key material and the bytes `create_encoding` returns for a derivation context; outcome the key equals `expected_derived_key` over the wrapping-key purpose, those bytes, and `REFERENCE_LENGTH`; variation each case of the declared set that declares the identifier over each case of `DECLARED_ENCODING_SELECTIONS`, whose encoded bytes differ in length and content from the reference context, so a derivation that frames the context by the reference context's shape fails; edge that must survive the composition: the bytes the encoding family produces are consumed by the derivation as returned
    * `[ ]`   Private surface: the chain of real functions `create_key_derivation`, the selected concrete's `try_new`, and `declaration` and `derive_key` as called on the returned adapter, with `Secret::expose` and the `Secret` drop of `domain` running for real; the cases are the declared set and the mock selection; no outer-edge collaborator is mocked, `blake3`, `zeroize`, and `domain` running as themselves; the observable result is the `Result` `create_key_derivation` returns, the declaration, the key's bytes, and the key material's bytes before and after the call; the entries proven are `every_concrete_is_constructed_under_the_identifier_it_declares`, `every_concrete_is_refused_under_an_identifier_it_does_not_declare`, `every_concrete_derives_a_key_of_the_requested_length`, `every_concrete_returns_its_own_error_when_the_key_material_length_cannot_be_framed`, and the absorbed entries
    * `[ ]`   Public surface: an outside caller invokes `create_key_derivation` with `CreateKeyDerivationDeps`, a `CreateKeyDerivationParams` taken from `DECLARED_KDF_SELECTIONS`, and a `CreateKeyDerivationPayload`, calls `derive_key` on the returned adapter with `DeriveKeyParams` and `DeriveKeyPayload`, and obtains the context bytes from `create_encoding` over the official consumer mock, and observes the key's bytes; the entries proven are `every_concrete_declaring_the_blake3_keyed_identifier_derives_the_wrapping_key_over_the_reference_inputs` and `every_concrete_declaring_the_blake3_keyed_identifier_derives_the_wrapping_key_over_the_context_the_encoding_factory_encodes`

  * `[✅]`   `adapters/kdf/src/factory/mock.rs`
    * `[ ]`   Imports the added names from `super::interface`
    * `[ ]`   `CreateKeyDerivationDepsOverrides`, `#[derive(Default)]`, fieldless, since the deps declare no field; `build_create_key_derivation_deps(overrides: CreateKeyDerivationDepsOverrides) -> CreateKeyDerivationDeps`; `CreateKeyDerivationDepsCorruptions`, `#[derive(Default)]`, fieldless; `invalidate_create_key_derivation_deps(corruptions: CreateKeyDerivationDepsCorruptions) -> serde_json::Value`, returning the empty JSON object, the representation of an object type that declares no field
    * `[ ]`   `CreateKeyDerivationPayloadOverrides`, `#[derive(Default)]`, fieldless; `build_create_key_derivation_payload(overrides: CreateKeyDerivationPayloadOverrides) -> CreateKeyDerivationPayload`; `CreateKeyDerivationPayloadCorruptions`, `#[derive(Default)]`, fieldless; `invalidate_create_key_derivation_payload(corruptions: CreateKeyDerivationPayloadCorruptions) -> serde_json::Value`, returning the empty JSON object
    * `[ ]`   `CreateKeyDerivationParamsOverrides`, `#[derive(Default)]`, one `Option` per field of `CreateKeyDerivationParams`; `build_create_key_derivation_params(overrides: CreateKeyDerivationParamsOverrides) -> CreateKeyDerivationParams`, the omitted fields defaulting to `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::NoFailure)` and `KdfIdentifier::MockV1`, so the defaults select the mock concrete under the identifier it declares; `CreateKeyDerivationParamsCorruptions`, `#[derive(Default)]`, one `Option<serde_json::Value>` per field; `invalidate_create_key_derivation_params(corruptions: CreateKeyDerivationParamsCorruptions) -> serde_json::Value` over `build_create_key_derivation_params` with no override, an enum represented as its variant name text by an exhaustive `match` and the `Mock` variant of `KdfConcrete` as a JSON object keyed `Mock` holding the failure mode's variant name text
    * `[ ]`   `CreateKeyDerivationSuccessReturnOverrides`, `#[derive(Default)]`, one `Option` per field of `CreateKeyDerivationSuccessReturn`; `build_create_key_derivation_success_return(overrides: CreateKeyDerivationSuccessReturnOverrides) -> CreateKeyDerivationSuccessReturn`, the default `adapter` the family's mock concrete, boxed, from its `try_new` under `MockIKeyDerivationAdapterFailureMode::NoFailure` through the irrefutable pattern `let Ok(adapter) = …;`; `CreateKeyDerivationSuccessReturnCorruptions`, `#[derive(Default)]`, one `Option<serde_json::Value>` per field; `invalidate_create_key_derivation_success_return(corruptions: CreateKeyDerivationSuccessReturnCorruptions) -> serde_json::Value` over `build_create_key_derivation_success_return` with no override, the `adapter` represented by `invalidate_kdf_declaration` over no corruption, since a trait object has no field representation
    * `[ ]`   `mock_create_key_derivation`, a function of the production type `CreateKeyDerivationFn`, returning `Ok(build_create_key_derivation_success_return(Default::default()))`
    * `[ ]`   `KdfConcrete` and `CreateKeyDerivationErrorReturn` are used by their production variants with no mock; `DECLARED_KDF_SELECTIONS` is used by its production value; `CreateKeyDerivationReturn` is an alias of `Result` and `CreateKeyDerivationFn` is a function type whose mock is `mock_create_key_derivation`; no failure mode is added, since `MockIKeyDerivationAdapterFailureMode` carries the arm no input reaches

  * `[ ]`   `adapters/kdf/src/factory/test.rs`
    * `[ ]`   Module-level `#![allow(clippy::expect_used)]`; imports `create_key_derivation`, `CreateKeyDerivationErrorReturn`, `CreateKeyDerivationParamsOverrides`, `IKeyDerivationAdapter`, `KdfIdentifier`, `build_create_key_derivation_deps`, `build_create_key_derivation_params`, and `build_create_key_derivation_payload` from `super::provides`
    * `[ ]`   `create_key_derivation_returns_the_adapter_of_the_selected_concrete`
      * `[ ]`   Contract: `create_key_derivation`, admitted: the named concrete's declared identifier is `params.identifier` → `Ok(CreateKeyDerivationSuccessReturn { adapter })` whose adapter reports the declaration of the concrete the selection names
      * `[ ]`   Collaborators: the family's mock concrete, reached through `create_key_derivation` by `build_create_key_derivation_params` at its defaults, which select it under the identifier it declares; `build_create_key_derivation_deps` and `build_create_key_derivation_payload` at their defaults; no test names a concrete
      * `[ ]`   Arrange: `build_create_key_derivation_deps`, `build_create_key_derivation_params`, and `build_create_key_derivation_payload`, each with no override; the selected concrete declares an identifier no other concrete declares, so a factory that constructs another concrete reports another identifier
      * `[ ]`   Act: `create_key_derivation(&deps, params, payload)`
      * `[ ]`   Assert: the success arm is bound through `expect`; `adapter.declaration().identifier` equals `KdfIdentifier::MockV1`, the identifier the family's mock concrete declares, by `assert!`, the expectation a literal and not read from the arrangement
    * `[ ]`   `create_key_derivation_refuses_a_concrete_that_declares_another_identifier`
      * `[ ]`   Contract: `create_key_derivation`, unsupported identifier: the named concrete's `DECLARATION.identifier` is not `params.identifier` → `Err(CreateKeyDerivationErrorReturn::UnsupportedKdfIdentifier)`
      * `[ ]`   Collaborators: the family's mock concrete, reached through `create_key_derivation` by `build_create_key_derivation_params`; `build_create_key_derivation_deps` and `build_create_key_derivation_payload` at their defaults; no test names a concrete
      * `[ ]`   Arrange: `build_create_key_derivation_deps` and `build_create_key_derivation_payload` with no override; `build_create_key_derivation_params` with the identifier override `KdfIdentifier::Blake3KeyedV1`, an identifier the mock concrete does not declare, so the declared identifier differs from the required one
      * `[ ]`   Act: `create_key_derivation(&deps, params, payload)`
      * `[ ]`   Assert: `result.err()` equals `Some(CreateKeyDerivationErrorReturn::UnsupportedKdfIdentifier)` by `assert_eq!`

  * `[✅]`   `construction`
    * `[✅]`   The composition root calls `create_key_derivation` with `&CreateKeyDerivationDeps`, `CreateKeyDerivationParams` holding the concrete the configuration names and the identifier the hash-card requires, and `CreateKeyDerivationPayload`, and places the returned `Box<dyn IKeyDerivationAdapter>` in each consumer's deps; no consumer constructs or names a concrete

  * `[✅]`   `adapters/kdf/src/factory/mod.rs`
    * `[✅]`   Adds `#[cfg(test)] mod integration_test;` and `#[cfg(test)] mod test;` to the wiring `kdf/blake3_keyed` authored
    * `[✅]`   `pub fn create_key_derivation(_deps: &CreateKeyDerivationDeps, params: CreateKeyDerivationParams, _payload: CreateKeyDerivationPayload) -> CreateKeyDerivationReturn`, a `match` on `params.concrete` whose `KdfConcrete::Blake3Keyed` arm returns the refusal when `Blake3KeyedKdf::DECLARATION.identifier` is not `params.identifier`, then binds the concrete by `let Ok(kdf) = Blake3KeyedKdf::try_new(Blake3KeyedKdfConstructorParams);` and returns `Ok(CreateKeyDerivationSuccessReturn { adapter: Box::new(kdf) })`, and, behind `#[cfg(any(test, feature = "mocks"))]`, whose `KdfConcrete::Mock(failure_mode)` arm returns the refusal when `MockIKeyDerivationAdapter::DECLARATION.identifier` is not `params.identifier`, then binds the concrete by `let Ok(mock) = MockIKeyDerivationAdapter::try_new(MockIKeyDerivationAdapterConstructorParams { failure_mode });` and returns `Ok(CreateKeyDerivationSuccessReturn { adapter: Box::new(mock) })`
    * `[✅]`   Imports `Blake3KeyedKdf` and `Blake3KeyedKdfConstructorParams` from `crate::blake3_keyed::provides`, and this module's types from `interface`, and, behind `#[cfg(any(test, feature = "mocks"))]`, `MockIKeyDerivationAdapter` and `MockIKeyDerivationAdapterConstructorParams` from `super::mock`
    * `[✅]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[ ]`   `adapters/kdf/src/factory/integration_test.rs`
    * `[ ]`   Module-level `#![allow(clippy::expect_used)]`; imports `create_key_derivation`, `DECLARED_KDF_SELECTIONS`, `CreateKeyDerivationErrorReturn`, `CreateKeyDerivationParamsOverrides`, `KdfConcrete`, `IKeyDerivationAdapter`, `KDF_INTERFACE_VERSION`, `DeriveKeyErrorReturn`, `MockIKeyDerivationAdapterFailureMode`, `MockIKeyDerivationAdapterDeriveKeyErrorReturn`, `REFERENCE_LENGTH`, `REFERENCE_PREFIX_LENGTH`, `reference_key_material_bytes`, `DeriveKeyParamsOverrides`, `DeriveKeyPayloadOverrides`, `build_create_key_derivation_deps`, `build_create_key_derivation_params`, `build_create_key_derivation_payload`, `build_derive_key_params`, and `build_derive_key_payload` from `super::provides`, and `build_secret` and `SecretConstructorParamsOverrides` from `domain`
    * `[ ]`   Cases: every block's body is written once and runs over each entry of `DECLARED_KDF_SELECTIONS` and over the mock selection, `build_create_key_derivation_params` at its defaults; no block names a concrete
    * `[ ]`   `every_concrete_is_constructed_under_the_identifier_it_declares`
      * `[ ]`   Contract: `create_key_derivation` is called with the selection of a concrete and the identifier that concrete declares → `Ok(CreateKeyDerivationSuccessReturn { adapter })` with `adapter.declaration()` reporting the identifier required and the interface version `KDF_INTERFACE_VERSION`
      * `[ ]`   Arrange: for each case, `build_create_key_derivation_deps`, `build_create_key_derivation_params` overriding `concrete` and `identifier` with the case's selection and identifier, and `build_create_key_derivation_payload`; the cases declare different identifiers, so a factory that constructs a concrete other than the selected one reports another identifier
      * `[ ]`   Act: `create_key_derivation(&deps, params, payload)` for the case
      * `[ ]`   Assert: the success arm is bound through `expect`; `adapter.declaration().identifier` equals the case's identifier by `assert!`, and `adapter.declaration().interface_version` equals `KDF_INTERFACE_VERSION` by `assert_eq!`
      * `[ ]`   Boundary: the crate-internal path `create_key_derivation` through the selected concrete's `try_new`, with the declaration read through the returned adapter
      * `[ ]`   Mocked: nothing; the declared concretes and the mock concrete run as themselves, so this block does not prove a derivation
    * `[ ]`   `every_concrete_is_refused_under_an_identifier_it_does_not_declare`
      * `[ ]`   Contract: `create_key_derivation` is called with the selection of a concrete and an identifier that concrete does not declare → `Err(CreateKeyDerivationErrorReturn::UnsupportedKdfIdentifier)`
      * `[ ]`   Arrange: for each pair of cases whose declared identifiers differ, `build_create_key_derivation_deps`, `build_create_key_derivation_params` overriding `concrete` with the first case's selection and `identifier` with the second case's identifier, and `build_create_key_derivation_payload`; every pairing carries an identifier the selected concrete does not declare, so an admission that compares against a constant or ignores the selection admits a pairing it must refuse
      * `[ ]`   Act: `create_key_derivation(&deps, params, payload)` for the pairing
      * `[ ]`   Assert: `result.err()` equals `Some(CreateKeyDerivationErrorReturn::UnsupportedKdfIdentifier)` by `assert_eq!`
      * `[ ]`   Boundary: the crate-internal path `create_key_derivation`, whose admission reads the selected concrete's declaration
      * `[ ]`   Mocked: nothing
    * `[ ]`   `every_concrete_derives_a_key_of_the_requested_length`
      * `[ ]`   Contract: the adapter `create_key_derivation` returns derives with a `params.length` → `Ok(DeriveKeySuccessReturn { key })` with the key's byte length equal to `params.length`
      * `[ ]`   Arrange: for each case, the adapter from `create_key_derivation` over `build_create_key_derivation_deps`, the case's params, and `build_create_key_derivation_payload`, bound through `expect`; for each length of `REFERENCE_LENGTH`, `REFERENCE_PREFIX_LENGTH`, and zero, `build_derive_key_params` with the length override and `build_derive_key_payload` with no override; the lengths differ, so a concrete that returns the reference length whatever is requested fails
      * `[ ]`   Act: `adapter.derive_key(params, payload)` for the case and the length
      * `[ ]`   Assert: the success arm is bound through `expect`; the key's `expose().len()` equals the length override by `assert_eq!`
      * `[ ]`   Boundary: the crate-internal path `create_key_derivation` through the selected concrete's `try_new`, then the returned adapter's `derive_key`
      * `[ ]`   Mocked: nothing; `blake3`, `zeroize`, and `domain` run for real
    * `[ ]`   `every_concrete_returns_its_own_error_when_the_key_material_length_cannot_be_framed`
      * `[ ]`   Contract: the adapter `create_key_derivation` returns for the mock selection under the failure mode `KeyMaterialLengthUnrepresentable` derives → `Err(DeriveKeyErrorReturn::Mock(MockIKeyDerivationAdapterDeriveKeyErrorReturn::KeyMaterialLengthUnrepresentable { length }))` with `length` the key material's length
      * `[ ]`   Arrange: the case is the mock selection alone, which the declared set cannot reach; `build_create_key_derivation_params` with the `concrete` override `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable)`; the adapter from `create_key_derivation` over `build_create_key_derivation_deps`, those params, and `build_create_key_derivation_payload`, bound through `expect`; `build_derive_key_params` and `build_derive_key_payload` with no override, whose key material is the reference key material
      * `[ ]`   Act: `adapter.derive_key(params, payload)`
      * `[ ]`   Assert: `result.err()` equals `Some(DeriveKeyErrorReturn::Mock(MockIKeyDerivationAdapterDeriveKeyErrorReturn::KeyMaterialLengthUnrepresentable { length }))` by `assert_eq!`, with `length` the length of `reference_key_material_bytes()`
      * `[ ]`   Boundary: the crate-internal path `create_key_derivation` through the mock concrete's `try_new` with the configured failure mode, then the returned adapter's `derive_key`
      * `[ ]`   Mocked: nothing
    * `[ ]`   `every_concrete_leaves_the_borrowed_key_material_intact`
      * `[ ]`   Contract: the absorbed entry `kdf/blake3_keyed` `derive_key_leaves_the_borrowed_key_material_intact`, in this node's terms: `derive_key` is called on the adapter `create_key_derivation` returns with a living `Secret<Vec<u8>>` borrowed in the payload → after the call the secret's `expose()` equals the bytes it held before the call
      * `[ ]`   Arrange: `build_secret` with the value override `reference_key_material_bytes()`, whose bytes are not all zero, so a zeroized secret differs from an intact one; for each case, the adapter from `create_key_derivation` over `build_create_key_derivation_deps`, the case's params, and `build_create_key_derivation_payload`, bound through `expect`; `build_derive_key_params` with no override; `build_derive_key_payload` with the key material override set to a borrow of that secret
      * `[ ]`   Act: `adapter.derive_key(params, payload)` for the case
      * `[ ]`   Assert: the success arm is bound through `expect`; the secret's `expose()` equals `reference_key_material_bytes()` by `assert_eq!`
      * `[ ]`   Boundary: the crate-internal path `create_key_derivation` through the selected concrete's `try_new`, then the returned adapter's `derive_key`, with `Secret::expose` reading the key material
      * `[ ]`   Mocked: nothing; `blake3`, `zeroize`, and `domain` run for real, so this block does not prove the zeroization of a vendor's hasher and reader themselves, which no caller can observe
    * `[ ]`   `every_concrete_returns_a_key_that_outlives_the_key_material`
      * `[ ]`   Contract: the absorbed entry `kdf/blake3_keyed` `derive_key_returns_a_key_that_outlives_the_key_material`, in this node's terms: `derive_key` returns `Ok` on the adapter `create_key_derivation` returns and the key material's secret is then dropped, its drop zeroizing its value → the returned key's `expose()` still equals the bytes it held before the drop
      * `[ ]`   Arrange: `build_secret` with the value override `reference_key_material_bytes()`, so the dropped secret holds non-zero bytes that its drop overwrites; for each case, the adapter from `create_key_derivation` over `build_create_key_derivation_deps`, the case's params, and `build_create_key_derivation_payload`, bound through `expect`; `build_derive_key_params` with no override; `build_derive_key_payload` with the key material override set to a borrow of that secret
      * `[ ]`   Act: `adapter.derive_key(params, payload)` for the case
      * `[ ]`   Assert: the success arm is bound through `expect`; a copy of the key's `expose()` bytes is read before `drop` of the key material's secret, and the key's `expose()` after the drop equals that copy by `assert_eq!`, so a key that shares storage with the key material reads as zeros
      * `[ ]`   Boundary: the crate-internal path `create_key_derivation` through the selected concrete's `try_new`, then the returned adapter's `derive_key`, with the key material's `Secret` drop running for real
      * `[ ]`   Mocked: nothing; `blake3`, `zeroize`, and `domain` run for real

  * `[✅]`   `adapters/kdf/src/factory/provides.rs`
    * `[✅]`   Adds `pub use super::create_key_derivation;` to the re-exports `kdf/blake3_keyed` authored

  * `[ ]`   `adapters/kdf/tests/factory_integration_test.rs`
    * `[ ]`   Imports from the crate's public surface under the `mocks` feature `create_key_derivation`, `DECLARED_KDF_SELECTIONS`, `KdfIdentifier`, `IKeyDerivationAdapter`, `DerivationPurpose`, `REFERENCE_LENGTH`, `expected_derived_key`, `reference_context`, `build_create_key_derivation_deps`, `build_create_key_derivation_params`, `CreateKeyDerivationParamsOverrides`, `build_create_key_derivation_payload`, `build_derive_key_params`, `DeriveKeyParamsOverrides`, `build_derive_key_payload`, and `DeriveKeyPayloadOverrides`; from `encoding` under its `mocks` feature `create_encoding`, `DECLARED_ENCODING_SELECTIONS`, `build_create_encoding_deps`, `build_create_encoding_params`, `CreateEncodingParamsOverrides`, `build_create_encoding_payload`, `build_mock_i_encoding_consumer`, and `build_derivation_context_description`; and `build_derivation_context` from `domain` under its `mocks` feature; module-level `#![allow(clippy::expect_used)]`
    * `[ ]`   Cases: every block's body is written once and runs over each entry of `DECLARED_KDF_SELECTIONS` whose identifier is `KdfIdentifier::Blake3KeyedV1`; no block names a concrete
    * `[ ]`   `every_concrete_declaring_the_blake3_keyed_identifier_derives_the_wrapping_key_over_the_reference_inputs`
      * `[ ]`   Contract: `create_key_derivation` is called with the selection of a concrete that declares `KdfIdentifier::Blake3KeyedV1` and that identifier, and the adapter derives with `DerivationPurpose::WrappingKey` and `REFERENCE_LENGTH` over the reference key material and the reference context → the key equals `expected_derived_key` over the wrapping-key purpose, the reference context, and `REFERENCE_LENGTH`
      * `[ ]`   Arrange: for each case, the adapter from `create_key_derivation` over `build_create_key_derivation_deps`, `build_create_key_derivation_params` overriding `concrete` and `identifier` with the case's selection and identifier, and `build_create_key_derivation_payload`, bound through `expect`; `build_derive_key_params` overriding the purpose with `DerivationPurpose::WrappingKey` and the length with `REFERENCE_LENGTH`; `build_derive_key_payload` with no override, whose key material and context are the reference key material and the reference context, so the oracle's inputs are the arrangement's inputs and its output is not
      * `[ ]`   Act: `adapter.derive_key(params, payload)` for the case
      * `[ ]`   Assert: the success arm is bound through `expect`; the key's `expose()` equals `expected_derived_key` over `DerivationPurpose::WrappingKey`, `reference_context()`, and `REFERENCE_LENGTH` by `assert_eq!`, the expectation computed by the independent oracle and not read from the arrangement
      * `[ ]`   Boundary: the public call `create_key_derivation` and the returned adapter's `derive_key`; the route `create_key_derivation`, the selected concrete's constructor, and its `derive_key`
      * `[ ]`   Mocked: nothing; `blake3`, `zeroize`, and `domain` run for real
    * `[ ]`   `every_concrete_declaring_the_blake3_keyed_identifier_derives_the_wrapping_key_over_the_context_the_encoding_factory_encodes`
      * `[ ]`   Contract: `create_key_derivation` is called with the selection of a concrete that declares `KdfIdentifier::Blake3KeyedV1` and that identifier, and the adapter derives with `DerivationPurpose::WrappingKey` and `REFERENCE_LENGTH` over the reference key material and the bytes `create_encoding` returns for a derivation context → the key equals `expected_derived_key` over the wrapping-key purpose, those bytes, and `REFERENCE_LENGTH`
      * `[ ]`   Arrange: for each case and each entry of `DECLARED_ENCODING_SELECTIONS`, the context bytes obtained from `create_encoding` over `build_create_encoding_deps` taking `build_mock_i_encoding_consumer` over `build_derivation_context_description`, `build_derivation_context` with no override, and no bytes, `build_create_encoding_params` overriding `concrete` and `identifier` with the encoding entry's selection and identifier, and `build_create_encoding_payload`, the `encoded` of the output bound through `expect`; the adapter as in the block above; `build_derive_key_params` as in the block above; `build_derive_key_payload` with the context override set to a borrow of those bytes, whose length and content differ from the reference context's, so a derivation that frames the context by the reference context's shape fails
      * `[ ]`   Act: `adapter.derive_key(params, payload)` for the case and the encoding entry
      * `[ ]`   Assert: the success arm is bound through `expect`; the key's `expose()` equals `expected_derived_key` over `DerivationPurpose::WrappingKey`, the context bytes, and `REFERENCE_LENGTH` by `assert_eq!`, the expectation computed by the independent oracle and not read from the arrangement
      * `[ ]`   Boundary: the public calls `create_encoding`, `create_key_derivation`, and the returned adapter's `derive_key`; the route `create_encoding`, the selected encoding concrete's `encode`, `DerivationContextDescription::to_fields`, and `domain`'s accessors, then `create_key_derivation`, the selected concrete's constructor, and its `derive_key`
      * `[ ]`   Mocked: the encoding consumer, replaced by the official `MockIEncodingConsumer`, so the test does not prove what a real consumer does with the concrete; the description, `domain`, `blake3`, and `zeroize` run for real

  * `[✅]`   `directionality`
    * `[✅]`   The `factory` module depends on the `blake3_keyed` concrete through `crate::blake3_keyed::provides` and on its own interface; `blake3_keyed` depends on the `factory` module's surface, the family form's recorded cycle; the crate's public surface is the `factory` module's `provides`
    * `[✅]`   Among repository crates the crate depends on `crates/domain` at runtime and on `adapters/encoding` for its integration test only; `adapters/encoding` names nothing in this crate; no cycle
    * `[✅]`   `workflows/sidecar/wrap` consumes the family through `create_key_derivation` and `IKeyDerivationAdapter`

  * `[ ]`   `requirements`
    * `[✅]`   `CreateKeyDerivationErrorReturn` derives `Debug`, `PartialEq`, and `Eq`
    * `[✅]`   `adapters/kdf/Cargo.toml` carries exactly the dev-dependencies stated above, and every other table `kdf/blake3_keyed` stated is unchanged
    * `[ ]`   `create_key_derivation_returns_the_adapter_of_the_selected_concrete` and `create_key_derivation_refuses_a_concrete_that_declares_another_identifier` pass
    * `[ ]`   `every_concrete_is_constructed_under_the_identifier_it_declares`, `every_concrete_is_refused_under_an_identifier_it_does_not_declare`, `every_concrete_derives_a_key_of_the_requested_length`, `every_concrete_returns_its_own_error_when_the_key_material_length_cannot_be_framed`, `every_concrete_leaves_the_borrowed_key_material_intact`, and `every_concrete_returns_a_key_that_outlives_the_key_material` pass
    * `[ ]`   `every_concrete_declaring_the_blake3_keyed_identifier_derives_the_wrapping_key_over_the_reference_inputs` and `every_concrete_declaring_the_blake3_keyed_identifier_derives_the_wrapping_key_over_the_context_the_encoding_factory_encodes` pass (CR-11, the wrapping-key derivation over the encoded derivation context reached through both families' surfaces)
    * `[✅]`   `cargo check --workspace --all-targets --all-features`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo fmt --all --check`, and `cargo deny check` complete without error or warning in every target, the `blake3_keyed` concrete's unused-item warnings having no remaining cause
    * `[✅]`   `blake3` is named nowhere outside `adapters/kdf/src/blake3_keyed`, and no code outside `adapters/kdf` can name `Blake3KeyedKdf`

* `[ ]`   `hash-to-scalar/domain_tag` **Domain tag, the byte string that separates one hash-to-scalar use from every other, admitted only when it is non-empty, fits a one-byte length prefix, and is printable ASCII with no space at either edge; creates the `adapters/hash-to-scalar` crate**

  * `[✅]`   `objective`
    * `[✅]`   Problem: every value a contract recomputes, the identity mapping and the delivery proof's challenge, is keccak256 under a domain tag reduced modulo the group order, and the client and the contract must hash the same tag in the same byte form, so a tag is a value with one byte form that both sides can state as a constant, refused before anything is hashed under it when it has none (CR-08; CR-09; CR-11; `docs/research/cryptography.md`'s Credential KEM and Delivery Proof statements)
    * `[✅]`   Functional: one type holds a domain tag's bytes, reachable only through a read accessor, and its only producer is a fallible constructor
    * `[✅]`   Functional: the constructor refuses an empty tag, a tag longer than 255 bytes, so its length fits the one-byte prefix the hash-to-scalar concrete absorbs, and any byte outside printable ASCII, `0x20` through `0x7E`
    * `[✅]`   Functional: the constructor refuses a space, `0x20`, as the tag's first or last byte, so every byte at a tag's edges is visible where the tag is written, mirrored to Solidity, or quoted; a space between visible bytes is admitted
    * `[✅]`   Functional: a refusal names the failed check and the values it failed on, and the same input always yields the same refusal: the length checks precede the byte scan, the byte scan precedes the edge checks, within the scan the lowest offending index decides, and the leading edge is checked before the trailing edge
    * `[✅]`   Non-functional: the module depends on the standard library alone, and its `mock.rs` additionally uses `serde_json` under the `mocks` feature; the crate names no hash library and no pairing library

  * `[✅]`   `role`
    * `[✅]`   Adapter family: the hash-to-scalar family's family-owned value type, and the family's first source file, so it creates the crate; the tag is a family-owned module beside the factory because a type lives in the module that implements it, and the tag's constructor is its implementation
    * `[✅]`   Does not hash, prefix, or reduce; `hash-to-scalar/keccak256` absorbs the tag's length and bytes and reduces the digest
    * `[✅]`   Does not name any use's tag; each consumer that hashes to a scalar, `kem/bb1_depth_one` for the identity mapping and `proof/schnorr_fs/challenge` for the challenge, states its own tag as a constant and constructs it through this type, and the generate family mirrors those constants to Solidity
    * `[✅]`   Does not create the `factory` module; `hash-to-scalar/keccak256` authors the family's generic interface, identifier, declaration, and mock there
    * `[✅]`   Does not carry a commit

  * `[✅]`   `module`
    * `[✅]`   Bounded context: the family-owned `domain_tag` module of the `adapters/hash-to-scalar` crate, holding `DomainTag`, its maximum length, its constructor params, and its constructor's error and return types
    * `[✅]`   Creates the crate at `adapters/hash-to-scalar`, admitted by the workspace's `adapters/*` glob with no edit to the root manifest
    * `[✅]`   The crate's public surface is the `domain_tag` module's `provides`, which every consumer that constructs a tag imports; the `factory` module's `provides` joins it when `hash-to-scalar/keccak256` creates that module
    * `[✅]`   Outside: every use's tag value, the hash, the prefix, the reduction, and the Solidity mirror

  * `[✅]`   `deps`
    * `[✅]`   The standard library: `Vec<u8>`, `u8::is_ascii_graphic`, `Iterator::enumerate`, and `Iterator::find`, through the prelude
    * `[✅]`   `serde_json` `1.0.151`, external crate, MIT OR Apache-2.0, optional runtime dependency enabled by this crate's `mocks` feature and a dev-dependency; supplies `serde_json::Value` and `serde_json::Map`, the untrusted type the invalidator returns, in `mock.rs` only
    * `[✅]`   No repository crate; no reverse dependency; nothing depends on the crate yet

  * `[✅]`   `context_slice`
    * `[✅]`   From the standard library: `<[u8]>::is_empty`, `<[u8]>::len`, `<[u8]>::iter`, `<[u8]>::first`, `<[u8]>::last`, `Iterator::enumerate`, `Iterator::find`, and `u8::is_ascii_graphic`, which is true exactly for `0x21` through `0x7E`, so a byte is printable ASCII when it is `b' '` or `is_ascii_graphic` holds
    * `[✅]`   From `serde_json`: `Value`, `Map<String, Value>`, and `Value::from` over integers and arrays, in `mock.rs` only

  * `[✅]`   `adapters/hash-to-scalar/Cargo.toml`
    * `[✅]`   `[package]` with `name = "hash-to-scalar"`, `edition.workspace = true`, `rust-version.workspace = true`, and `publish.workspace = true`; no `version` key
    * `[✅]`   `[dependencies]` with `serde_json = { version = "1.0.151", optional = true }`
    * `[✅]`   `[dev-dependencies]` with `serde_json = "1.0.151"`
    * `[✅]`   `[features]` with `mocks = ["dep:serde_json"]`
    * `[✅]`   `[lints]` with `workspace = true`
    * `[✅]`   No other table; `hash-to-scalar/keccak256` adds its own entries to the dependency tables

  * `[✅]`   `adapters/hash-to-scalar/src/lib.rs`
    * `[✅]`   The crate barrel: `mod domain_tag;` and `pub use domain_tag::provides::*;`, nothing else
    * `[✅]`   Until `domain_tag/mod.rs` exists, `cargo check` reports the unresolved `mod domain_tag`, which is the RED state for every element below that precedes the implementation

  * `[✅]`   `adapters/hash-to-scalar/src/domain_tag/interface.rs`
    * `[✅]`   `DOMAIN_TAG_MAXIMUM_LENGTH`, a `pub const` of type `usize` with value `255`, the largest length a one-byte prefix holds
    * `[✅]`   `DomainTag`, a struct with `#[derive(Clone, Debug, PartialEq, Eq)]` and the one field `pub(super) bytes: Vec<u8>`, so only the `domain_tag` module and its children reach the field
    * `[✅]`   `DomainTagConstructorParams`, a struct with the one field `pub bytes: Vec<u8>`; no derives
    * `[✅]`   `DomainTagTryNewErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]` and the variants `Empty`, `TooLong { length: usize, maximum: usize }`, `ByteOutsidePrintableAscii { index: usize, byte: u8 }`, and `SpaceAtEdge { index: usize }`
    * `[✅]`   `DomainTagTryNewReturn`, the alias `Result<DomainTag, DomainTagTryNewErrorReturn>`
    * `[✅]`   Imports nothing; declares nothing else

  * `[✅]`   `adapters/hash-to-scalar/src/domain_tag/interaction.spec.md`
    * `[✅]`   `DomainTag::try_new(params: DomainTagConstructorParams) -> DomainTagTryNewReturn`, empty: condition `params.bytes.is_empty()`; decision the emptiness check; dependency call none; outcome `Err(DomainTagTryNewErrorReturn::Empty)`
    * `[✅]`   Too long: condition the tag is non-empty and `params.bytes.len() > DOMAIN_TAG_MAXIMUM_LENGTH`; decision the comparison; dependency call none; outcome `Err(DomainTagTryNewErrorReturn::TooLong { length, maximum: DOMAIN_TAG_MAXIMUM_LENGTH })`, `length` the tag's length
    * `[✅]`   Byte outside printable ASCII: condition the length passes and some byte is neither `b' '` nor `is_ascii_graphic`; decision the first such byte by index, `iter().enumerate().find(…)`; dependency call none; outcome `Err(DomainTagTryNewErrorReturn::ByteOutsidePrintableAscii { index, byte })` for the lowest such index
    * `[✅]`   Leading space: condition every byte is printable ASCII and `params.bytes.first()` is `Some(&b' ')`; decision the comparison; dependency call none; outcome `Err(DomainTagTryNewErrorReturn::SpaceAtEdge { index: 0 })`
    * `[✅]`   Trailing space: condition the leading byte is not a space and `params.bytes.last()` is `Some(&b' ')`; decision the comparison; dependency call none; outcome `Err(DomainTagTryNewErrorReturn::SpaceAtEdge { index })`, `index` the tag's length less one
    * `[✅]`   Admitted: condition every check passes; outcome `Ok(DomainTag { bytes })`, the vector moved from the params without copy
    * `[✅]`   `DomainTag::as_bytes(&self) -> &[u8]`: one branch; outcome a shared reference to the held bytes, no copy, no side effect
    * `[✅]`   Ordering: emptiness, then length, then the byte scan, then the leading edge, then the trailing edge; the same params always yield the same outcome
    * `[✅]`   Invariants: every `DomainTag` holds between 1 and 255 bytes, each printable ASCII, its first and last byte visible ASCII; its only producer is `try_new`
    * `[ ]`   Own entry, an admitted tag reads back through the crate root: condition a caller outside the crate names `DomainTag`, `DomainTagConstructorParams`, and `DOMAIN_TAG_MAXIMUM_LENGTH` through the crate root and passes `try_new` a tag of exactly `DOMAIN_TAG_MAXIMUM_LENGTH` printable bytes with an interior space and a visible byte at each edge; outcome `Ok(tag)` whose `as_bytes()` yields exactly the bytes passed; variation the maximum length with an interior space, which a read that truncates, trims, or re-encodes fails; edge that must survive: the bytes an outside caller reads are the bytes the tag was constructed from, reached through the accessor alone
    * `[ ]`   Own entry, a refusal reaches the caller whole and yields no tag: condition a caller outside the crate passes `try_new` a tag of `DOMAIN_TAG_MAXIMUM_LENGTH + 1` printable bytes; outcome `Err(DomainTagTryNewErrorReturn::TooLong { length: DOMAIN_TAG_MAXIMUM_LENGTH + 1, maximum: DOMAIN_TAG_MAXIMUM_LENGTH })`, matched through the crate-root error type, and no `DomainTag` exists to read; variation one byte over the length the read-back entry admits; edge that must survive: the refusal carries both values it failed on
    * `[ ]`   Route, both own entries: the crate barrel `lib.rs`, then `domain_tag::provides`, then `DomainTag::try_new`, and for the read-back entry `DomainTag::as_bytes`; the route reaches no outer-edge collaborator
    * `[ ]`   Public surface: an outside caller invokes `DomainTag::try_new` and `DomainTag::as_bytes`, building params with `build_domain_tag_constructor_params` from the `mocks` feature; it observes the `Ok` tag through `as_bytes` and the `Err` through `DomainTagTryNewErrorReturn`; the entries proven are the read-back entry and the refusal entry

  * `[✅]`   `adapters/hash-to-scalar/src/domain_tag/mock.rs`
    * `[ ]`   `DomainTagConstructorParamsOverrides`, a struct with `#[derive(Default)]` and the one field `bytes: Option<Vec<u8>>`
    * `[ ]`   `build_domain_tag_constructor_params(overrides: DomainTagConstructorParamsOverrides) -> DomainTagConstructorParams`, taking `bytes` from the override when present; the default is a byte string constant declared in `mock.rs` that is non-empty, at most `DOMAIN_TAG_MAXIMUM_LENGTH` bytes, printable ASCII, and without a space at either edge
    * `[ ]`   `DomainTagConstructorParamsCorruptions`, a struct with `#[derive(Default)]` and the one field `bytes: Option<serde_json::Value>`
    * `[ ]`   `invalidate_domain_tag_constructor_params(corruptions: DomainTagConstructorParamsCorruptions) -> serde_json::Value`, a `Map` holding the built params' `bytes` converted with `Value::from` to an array of integers, overwritten by the corrupted key when present; `DomainTagConstructorParams` stays free of derives
    * `[ ]`   `build_domain_tag(overrides: DomainTagConstructorParamsOverrides) -> DomainTag`, the result of `DomainTag::try_new` over `build_domain_tag_constructor_params(overrides)`, a real instance with its field private
    * `[ ]`   Enumeration of the interface's exports: `DOMAIN_TAG_MAXIMUM_LENGTH` is a constant used by its production value; `DomainTagTryNewErrorReturn` is an enum and `DomainTagTryNewReturn` is an alias, used directly by their production types; `DomainTag` takes `build_domain_tag` alone, its corruption living in the params invalidator; `DomainTagConstructorParams` takes the four symbols above; `DomainTag::try_new` and `DomainTag::as_bytes` take no function mock because the interface declares no function type for them
    * `[ ]`   Imports `DomainTag` and `DomainTagConstructorParams` from `super::interface`, and `serde_json::Value` and `serde_json::Map`

  * `[ ]`   `adapters/hash-to-scalar/src/domain_tag/test.rs`
    * `[ ]`   Imports `DomainTag` from `super`, `DomainTagTryNewErrorReturn` and `DOMAIN_TAG_MAXIMUM_LENGTH` from `super::interface`, and `build_domain_tag`, `build_domain_tag_constructor_params`, and `DomainTagConstructorParamsOverrides` from `super::mock`; the module's only dependency is the standard library, so no collaborator is replaced; every fixture is a direct builder call overriding only the `bytes` the block's variation needs; every refusal is asserted by `assert_eq!` against the whole expected `Err`
    * `[ ]`   `try_new_rejects_an_empty_tag`
      * `[ ]`   Contract: `params.bytes.is_empty()` yields `Err(DomainTagTryNewErrorReturn::Empty)`
      * `[ ]`   Arrange: params whose `bytes` is the empty vector
      * `[ ]`   Act: `DomainTag::try_new` over those params
      * `[ ]`   Assert: the result equals `Err(DomainTagTryNewErrorReturn::Empty)`
    * `[ ]`   `try_new_rejects_a_tag_one_byte_over_the_maximum`
      * `[ ]`   Contract: a non-empty tag longer than `DOMAIN_TAG_MAXIMUM_LENGTH` yields `Err(DomainTagTryNewErrorReturn::TooLong { length, maximum: DOMAIN_TAG_MAXIMUM_LENGTH })`
      * `[ ]`   Arrange: params whose `bytes` is `DOMAIN_TAG_MAXIMUM_LENGTH + 1` printable non-space bytes, the least length the check refuses
      * `[ ]`   Act: `DomainTag::try_new` over those params
      * `[ ]`   Assert: the result equals `Err(DomainTagTryNewErrorReturn::TooLong { length: DOMAIN_TAG_MAXIMUM_LENGTH + 1, maximum: DOMAIN_TAG_MAXIMUM_LENGTH })`
    * `[ ]`   `try_new_rejects_a_byte_below_the_printable_range`
      * `[ ]`   Contract: a length-passing tag with a byte below `0x20` yields `Err(DomainTagTryNewErrorReturn::ByteOutsidePrintableAscii { index, byte })`
      * `[ ]`   Arrange: params whose `bytes` is three bytes, visible at both edges and `0x1F`, the byte directly below the range, at the interior index
      * `[ ]`   Act: `DomainTag::try_new` over those params
      * `[ ]`   Assert: the result equals `Err(DomainTagTryNewErrorReturn::ByteOutsidePrintableAscii { index: 1, byte: 0x1F })`, the index and byte stated in the assertion
    * `[ ]`   `try_new_rejects_the_byte_above_the_printable_range`
      * `[ ]`   Contract: a length-passing tag with a byte above `0x7E` yields `Err(DomainTagTryNewErrorReturn::ByteOutsidePrintableAscii { index, byte })`
      * `[ ]`   Arrange: params whose `bytes` is three bytes, visible at both edges and `0x7F`, the byte directly above the range, at the interior index
      * `[ ]`   Act: `DomainTag::try_new` over those params
      * `[ ]`   Assert: the result equals `Err(DomainTagTryNewErrorReturn::ByteOutsidePrintableAscii { index: 1, byte: 0x7F })`, the index and byte stated in the assertion
    * `[ ]`   `try_new_rejects_a_non_ascii_byte`
      * `[ ]`   Contract: a length-passing tag with a byte above `0x7F` yields `Err(DomainTagTryNewErrorReturn::ByteOutsidePrintableAscii { index, byte })`
      * `[ ]`   Arrange: params whose `bytes` is three bytes, visible at both edges and `0x80`, the least non-ASCII byte, at the interior index
      * `[ ]`   Act: `DomainTag::try_new` over those params
      * `[ ]`   Assert: the result equals `Err(DomainTagTryNewErrorReturn::ByteOutsidePrintableAscii { index: 1, byte: 0x80 })`, the index and byte stated in the assertion
    * `[ ]`   `try_new_rejects_the_lowest_byte_outside_printable_ascii`
      * `[ ]`   Contract: when several bytes are outside printable ASCII, the outcome names the lowest index
      * `[ ]`   Arrange: params whose `bytes` is visible at both edges and holds two offending bytes of different values, `0x1F` at index 1 and `0x80` at index 3, so the lowest index and the later one name different bytes
      * `[ ]`   Act: `DomainTag::try_new` over those params
      * `[ ]`   Assert: the result equals `Err(DomainTagTryNewErrorReturn::ByteOutsidePrintableAscii { index: 1, byte: 0x1F })`
    * `[ ]`   `try_new_rejects_a_leading_space`
      * `[ ]`   Contract: every byte printable ASCII and `params.bytes.first()` equal to `Some(&b' ')` yields `Err(DomainTagTryNewErrorReturn::SpaceAtEdge { index: 0 })`
      * `[ ]`   Arrange: params whose `bytes` is three bytes, a space then two visible bytes
      * `[ ]`   Act: `DomainTag::try_new` over those params
      * `[ ]`   Assert: the result equals `Err(DomainTagTryNewErrorReturn::SpaceAtEdge { index: 0 })`
    * `[ ]`   `try_new_rejects_a_trailing_space`
      * `[ ]`   Contract: a leading byte that is not a space and `params.bytes.last()` equal to `Some(&b' ')` yields `Err(DomainTagTryNewErrorReturn::SpaceAtEdge { index })`, `index` the length less one
      * `[ ]`   Arrange: params whose `bytes` is three bytes, two visible bytes then a space, so the length less one differs from 0, 1, and the length
      * `[ ]`   Act: `DomainTag::try_new` over those params
      * `[ ]`   Assert: the result equals `Err(DomainTagTryNewErrorReturn::SpaceAtEdge { index: 2 })`
    * `[ ]`   `try_new_reports_the_leading_edge_before_the_trailing_edge`
      * `[ ]`   Contract: the leading edge is checked before the trailing edge
      * `[ ]`   Arrange: params whose `bytes` is three bytes, a space, a visible byte, a space, so both edges fail
      * `[ ]`   Act: `DomainTag::try_new` over those params
      * `[ ]`   Assert: the result equals `Err(DomainTagTryNewErrorReturn::SpaceAtEdge { index: 0 })`
    * `[ ]`   `try_new_reports_the_bytes_before_the_edges`
      * `[ ]`   Contract: the byte scan precedes the edge checks
      * `[ ]`   Arrange: params whose `bytes` is a leading space, a visible byte, `0x1F` at index 2, and a visible byte, so an edge check and the scan both fail
      * `[ ]`   Act: `DomainTag::try_new` over those params
      * `[ ]`   Assert: the result equals `Err(DomainTagTryNewErrorReturn::ByteOutsidePrintableAscii { index: 2, byte: 0x1F })`
    * `[ ]`   `try_new_reports_the_length_before_the_bytes`
      * `[ ]`   Contract: the length check precedes the byte scan
      * `[ ]`   Arrange: params whose `bytes` is `DOMAIN_TAG_MAXIMUM_LENGTH + 1` bytes, printable except `0x1F` at an interior index, so the length check and the scan both fail
      * `[ ]`   Act: `DomainTag::try_new` over those params
      * `[ ]`   Assert: the result equals `Err(DomainTagTryNewErrorReturn::TooLong { length: DOMAIN_TAG_MAXIMUM_LENGTH + 1, maximum: DOMAIN_TAG_MAXIMUM_LENGTH })`
    * `[ ]`   `try_new_admits_a_tag_with_interior_spaces`
      * `[ ]`   Contract: every check passing yields `Ok(DomainTag { bytes })`, and a space between visible bytes passes
      * `[ ]`   Arrange: params whose `bytes` is a visible byte, a space, a visible byte
      * `[ ]`   Act: `DomainTag::try_new` over those params
      * `[ ]`   Assert: the result mapped to the held `bytes` equals `Ok` of the same three bytes, stated in the assertion, read from the field the test module's visibility reaches
    * `[ ]`   `try_new_admits_a_tag_of_the_maximum_length`
      * `[ ]`   Contract: a tag of exactly `DOMAIN_TAG_MAXIMUM_LENGTH` bytes passes the length check
      * `[ ]`   Arrange: params whose `bytes` is `DOMAIN_TAG_MAXIMUM_LENGTH` printable non-space bytes, the length directly below the refused length
      * `[ ]`   Act: `DomainTag::try_new` over those params
      * `[ ]`   Assert: the result mapped to the held `bytes` equals `Ok` of `DOMAIN_TAG_MAXIMUM_LENGTH` bytes stated in the assertion
    * `[ ]`   `try_new_admits_the_lowest_and_highest_graphic_bytes_at_the_edges`
      * `[ ]`   Contract: `0x21` through `0x7E` are printable and visible, so they pass the scan and the edge checks
      * `[ ]`   Arrange: params whose `bytes` is two bytes, `0x21` first and `0x7E` last
      * `[ ]`   Act: `DomainTag::try_new` over those params
      * `[ ]`   Assert: the result mapped to the held `bytes` equals `Ok` of `0x21` then `0x7E`, stated in the assertion
    * `[ ]`   `try_new_moves_the_params_bytes_into_the_tag`
      * `[ ]`   Contract: the admitted outcome moves the vector from the params without copy
      * `[ ]`   Arrange: params from the builder with its default bytes, the address and length of `params.bytes` read before the call
      * `[ ]`   Act: `DomainTag::try_new` over those params
      * `[ ]`   Assert: the result is `Ok`, and the held `bytes` has the address and the length read before the call
    * `[ ]`   `as_bytes_returns_the_held_bytes_without_copy`
      * `[ ]`   Contract: `as_bytes` returns a shared reference to the held bytes, no copy
      * `[ ]`   Arrange: a tag from `build_domain_tag` whose `bytes` override is distinct visible bytes
      * `[ ]`   Act: `DomainTag::as_bytes` on that tag
      * `[ ]`   Assert: the returned slice has the address and the length of the tag's held `bytes`

  * `[✅]`   `construction`
    * `[✅]`   `DomainTag::try_new` is the only producer; no `Default`, `From`, or other constructor exists; a consumer states its tag bytes as a constant and constructs the tag once, handling the refusal arm

  * `[✅]`   `adapters/hash-to-scalar/src/domain_tag/mod.rs`
    * `[✅]`   Module declarations: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, `pub mod provides;`, and `#[cfg(test)] mod test;`
    * `[✅]`   `impl DomainTag` with `pub fn try_new(params: DomainTagConstructorParams) -> DomainTagTryNewReturn` realizing the branches and ordering of the interaction spec, and `pub fn as_bytes(&self) -> &[u8]` returning `&self.bytes`
    * `[✅]`   Imports `DomainTag`, `DomainTagConstructorParams`, `DomainTagTryNewErrorReturn`, `DomainTagTryNewReturn`, and `DOMAIN_TAG_MAXIMUM_LENGTH` from `interface`
    * `[✅]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[✅]`   `adapters/hash-to-scalar/src/domain_tag/provides.rs`
    * `[✅]`   `pub use super::interface::*;` and `#[cfg(any(test, feature = "mocks"))] pub use super::mock::*;`, nothing else

  * `[ ]`   `adapters/hash-to-scalar/tests/domain_tag_integration_test.rs`
    * `[ ]`   Imports `DomainTag`, `DomainTagTryNewErrorReturn`, and `DOMAIN_TAG_MAXIMUM_LENGTH` through the crate root `hash_to_scalar`, and `build_domain_tag_constructor_params` and `DomainTagConstructorParamsOverrides` from the crate root under the `mocks` feature; nothing is imported from a module path
    * `[ ]`   `a_tag_of_the_maximum_length_reads_back_through_the_crate_root`
      * `[ ]`   Contract: the own entry, an admitted tag reads back through the crate root; a tag of exactly `DOMAIN_TAG_MAXIMUM_LENGTH` printable bytes with an interior space and a visible byte at each edge yields `Ok(tag)` whose `as_bytes()` yields exactly the bytes passed
      * `[ ]`   Arrange: params from `build_domain_tag_constructor_params` whose `bytes` is `DOMAIN_TAG_MAXIMUM_LENGTH` printable bytes with an interior space and visible edge bytes, the variation a read that truncates, trims, or re-encodes fails
      * `[ ]`   Act: `DomainTag::try_new` over those params, the `Ok` tag read through `as_bytes`
      * `[ ]`   Assert: the result is `Ok`, and the slice `as_bytes` returns equals the expected bytes stated in the assertion as their own value, since the params are moved into the call
      * `[ ]`   Boundary: the crate-root public call `DomainTag::try_new` and the read `DomainTag::as_bytes`, the route `lib.rs` then `domain_tag::provides` then `try_new` then `as_bytes`
      * `[ ]`   Mocked: none; the route reaches no outer-edge collaborator, and the params come from the official mock under the `mocks` feature
    * `[ ]`   `a_tag_one_byte_over_the_maximum_is_refused_whole_through_the_crate_root`
      * `[ ]`   Contract: the own entry, a refusal reaches the caller whole and yields no tag; a tag of `DOMAIN_TAG_MAXIMUM_LENGTH + 1` printable bytes yields `Err(DomainTagTryNewErrorReturn::TooLong { length: DOMAIN_TAG_MAXIMUM_LENGTH + 1, maximum: DOMAIN_TAG_MAXIMUM_LENGTH })`
      * `[ ]`   Arrange: params from `build_domain_tag_constructor_params` whose `bytes` is `DOMAIN_TAG_MAXIMUM_LENGTH + 1` printable non-space bytes, one byte over the length the read-back block admits
      * `[ ]`   Act: `DomainTag::try_new` over those params
      * `[ ]`   Assert: the result equals the whole `Err(DomainTagTryNewErrorReturn::TooLong { length: DOMAIN_TAG_MAXIMUM_LENGTH + 1, maximum: DOMAIN_TAG_MAXIMUM_LENGTH })`, matched through the crate-root error type, so no `DomainTag` is held
      * `[ ]`   Boundary: the crate-root public call `DomainTag::try_new`, the route `lib.rs` then `domain_tag::provides` then `try_new`
      * `[ ]`   Mocked: none; the route reaches no outer-edge collaborator, and the params come from the official mock under the `mocks` feature
    * `[ ]`   Enumeration: the read-back entry is proven by `a_tag_of_the_maximum_length_reads_back_through_the_crate_root`, and the refusal entry by `a_tag_one_byte_over_the_maximum_is_refused_whole_through_the_crate_root`

  * `[✅]`   `directionality`
    * `[✅]`   `domain_tag` depends on the standard library alone, and its `mock.rs` additionally depends on `serde_json` under the `mocks` feature; the crate depends on no repository crate; `hash-to-scalar/keccak256` consumes the tag through `crate::domain_tag::provides` in the generic interface's payload; no cycle

  * `[✅]`   `requirements`
    * `[✅]`   `adapters/hash-to-scalar/Cargo.toml` carries exactly the tables and keys stated above, and `adapters/hash-to-scalar/src/lib.rs` carries exactly the barrel stated above
    * `[✅]`   `cargo check --workspace --all-targets --all-features`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `cargo fmt --all --check` complete without error or warning
    * `[✅]`   `try_new_admits_a_tag_with_interior_spaces`, `try_new_admits_a_tag_of_the_maximum_length`, `try_new_admits_the_lowest_and_highest_graphic_bytes_at_the_edges`, `try_new_moves_the_params_bytes_into_the_tag`, and `as_bytes_returns_the_held_bytes_without_copy` pass
    * `[✅]`   `try_new_rejects_an_empty_tag`, `try_new_rejects_a_tag_one_byte_over_the_maximum`, `try_new_rejects_a_byte_below_the_printable_range`, `try_new_rejects_the_byte_above_the_printable_range`, `try_new_rejects_the_lowest_byte_outside_printable_ascii`, `try_new_rejects_a_non_ascii_byte`, `try_new_rejects_a_leading_space`, `try_new_rejects_a_trailing_space`, `try_new_reports_the_leading_edge_before_the_trailing_edge`, `try_new_reports_the_bytes_before_the_edges`, and `try_new_reports_the_length_before_the_bytes` pass (CR-11, a tag has one byte form, refused before any hash)
    * `[ ]`   `a_tag_of_the_maximum_length_reads_back_through_the_crate_root` and `a_tag_one_byte_over_the_maximum_is_refused_whole_through_the_crate_root` pass
    * `[✅]`   Code outside `adapters/hash-to-scalar/src/domain_tag` reading the `bytes` field fails to compile

* `[ ]`   `hash-to-scalar/keccak256` **Keccak-256 concrete hashing a length-prefixed domain tag and a message and reducing the digest modulo the group order of whichever pairing scalar it is asked for; authors the hash-to-scalar family's generic interface, identifier, declaration, and mock**

  * `[✅]`   `objective`
    * `[✅]`   Problem: the identity mapping and the delivery proof's challenge are scalars the contract recomputes, and the EVM has keccak256 and no BLAKE3, so both are keccak256 under a domain tag reduced modulo the group order, computed identically by the client and the contract, through one repo-owned interface, with no module outside a concrete naming the hash library (CR-08; CR-09; CR-11)
    * `[✅]`   Functional: the family's generic interface maps a domain tag and a message to a scalar of the pairing scalar type it is instantiated at, so one concrete serves every curve and every pairing library
    * `[✅]`   Functional: every concrete declares the hash-to-scalar identifier a deployment's hash-card names, its adapter version, and the interface version it implements, readable before any instance exists
    * `[✅]`   Functional: the Keccak-256 concrete hashes the tag's length as one byte, then the tag, then the message, reads the 32-byte digest as a big-endian integer, and reduces it modulo the group order through the scalar type's own sampling bound, the digest zero-extended at the front to the bound's input length; the contract reproduces it as `uint256(keccak256(abi.encodePacked(uint8(tag.length), tag, message))) % r`
    * `[✅]`   Functional: the concrete's scalars match the known-answer vectors computed by an independent Keccak implementation, on BN254 and BLS12-381 and on both libraries of each, including digests at or above the group order
    * `[✅]`   Non-functional: `sha3` is named only inside `adapters/hash-to-scalar/src/keccak256`; no pairing library is named in the crate

  * `[✅]`   `role`
    * `[✅]`   Adapter: the hash-to-scalar family's first concrete, and the first source file that requires the family's generic interface, identifier, declaration, and mock, which it authors in the family's `factory` module as its producers
    * `[✅]`   Creates `factory/mod.rs` with the module wiring alone and `factory/provides.rs` with the interface and mock surface alone; the factory function, its types, its interaction spec, its unit test, its re-export, and the family's integration test are `hash-to-scalar/factory`'s
    * `[✅]`   Does not reduce a scalar itself; the pairing concrete's `ISampleUniformScalar` reduces, so no field arithmetic is duplicated and no library is named here
    * `[✅]`   Does not name a use's tag or encode a message; each consumer holds its tag and encodes its message through the encoding factory
    * `[✅]`   Does not emit the Solidity mirror; `harness-crypto/generate/evm` mirrors the vectors
    * `[✅]`   Does not map the identifier to the hash-card's `hashToScalarId` byte; the hash-card's own encoding does that
    * `[✅]`   Does not carry a commit

  * `[✅]`   `module`
    * `[✅]`   Bounded context: the `factory` module's generic trait over a scalar type, its params, payload, success, error, and return types, the identifier, the declaration, the interface version, and the family's mock; and the private `keccak256` concrete, holding the adapter over `sha3`'s Keccak-256, its digest length, its constructor params and return, and its error
    * `[✅]`   Adds the `factory` and `keccak256` modules to the existing crate at `adapters/hash-to-scalar`; the crate's manifest gains its dependency tables and its barrel gains the modules' lines
    * `[✅]`   Outside: every tag value and message encoding, the pairing concretes' reduction, the factory's selection of a concrete, and the Solidity mirror

  * `[ ]`   `deps`
    * `[✅]`   `domain_tag`, same crate, through `crate::domain_tag::provides`: `DomainTag` with `as_bytes`, held by the generic interface's params; and `build_domain_tag` with `DomainTagConstructorParamsOverrides` in `keccak256/test.rs`
    * `[✅]`   `domain`, `crates/domain`, protocol and domain ring, path dependency; supplies `Secret` and `SecretConstructorParams`, the wrapper the sampling bound's payload takes; direction inward
    * `[✅]`   `pairing`, `adapters/pairing`, adapter ring, path dependency; supplies `ISampleUniformScalar` with `UNIFORM_BYTES_LENGTH` and `sample_from_uniform_bytes`, `SampleUniformScalarParams`, `SampleUniformScalarPayload`, and `SampleUniformScalarErrorReturn`, the family's scalar being the resolved pairing's scalar, the dependency map's edge from `pairing/factory`; the pairing crate names nothing in this crate
    * `[ ]`   `pairing` with its `mocks` feature, as a dev-dependency, in `keccak256/test.rs` only: the pairing family's builder for `SampleUniformScalarSuccessReturn<S>` and its overrides, from which the test-local scalar's sampler composes its return
    * `[✅]`   `sha3` `0.12.0`, external crate, MIT OR Apache-2.0, runtime dependency with default features, named only in `keccak256`; supplies `sha3::Keccak256` and the `sha3::Digest` trait
    * `[✅]`   `hex` `0.4.3`, external crate, MIT OR Apache-2.0, dev-dependency only; supplies `hex::decode` for the vectors
    * `[✅]`   `core::convert::Infallible`, standard library, the constructor's error arm; `core::marker::PhantomData`, standard library, in `factory/mock.rs` only; `TryFrom<usize> for u8`, standard library, the tag-length prefix
    * `[✅]`   No reverse dependency beyond the family form's recorded cycle: the `factory` module's error carries this concrete's error; nothing depends on the crate yet

  * `[ ]`   `context_slice`
    * `[✅]`   From `domain_tag`: `DomainTag::as_bytes(&self) -> &[u8]`, between 1 and 255 printable-ASCII bytes, no space at either edge
    * `[✅]`   From `domain`: `Secret::try_new(SecretConstructorParams { value })` returning `Result<Secret<T>, Infallible>`, and `Secret::expose(&self) -> &T`
    * `[✅]`   From `pairing`: `ISampleUniformScalar::UNIFORM_BYTES_LENGTH`, `64` on every current concrete, and `sample_from_uniform_bytes(SampleUniformScalarParams, SampleUniformScalarPayload { uniform }) -> Result<SampleUniformScalarSuccessReturn<S>, SampleUniformScalarErrorReturn>`, which reads `uniform` as a big-endian integer and reduces it modulo the group order into `Secret<S>`; `S: Clone` through `IPairingAdapter::Scalar`'s bound
    * `[✅]`   From `sha3`: `Keccak256::new()`, `Digest::update(&mut self, data: impl AsRef<[u8]>)`, and `Digest::finalize(self)`, whose 32-byte output dereferences to a byte slice
    * `[ ]`   From `pairing`, in `keccak256/test.rs`: `ISampleUniformScalar`, which the test-local scalar implements, and the pairing family's builder for `SampleUniformScalarSuccessReturn<S>`; from `domain`: `Secret::try_new(SecretConstructorParams { value })`, which wraps the test-local scalar the sampler returns

  * `[✅]`   `adapters/hash-to-scalar/Cargo.toml`
    * `[✅]`   `[dependencies]` with `domain = { path = "../../crates/domain" }`, `pairing = { path = "../pairing" }`, and `sha3 = "0.12.0"`
    * `[✅]`   `[dev-dependencies]` with `pairing = { path = "../pairing", features = ["mocks"] }` and `hex = "0.4.3"`
    * `[✅]`   `[package]`, `[features]`, and `[lints]` are unchanged; no other table

  * `[✅]`   `adapters/hash-to-scalar/src/lib.rs`
    * `[✅]`   The crate barrel reads `mod domain_tag;`, `mod factory;`, `mod keccak256;`, `pub use domain_tag::provides::*;`, and `pub use factory::provides::*;`, nothing else; the `keccak256` concrete's surface is not re-exported
    * `[✅]`   Until `factory/mod.rs` and `keccak256/mod.rs` exist, `cargo check` reports the unresolved modules, which is the RED state for every element below that precedes them

  * `[✅]`   `adapters/hash-to-scalar/src/factory/interface.rs`
    * `[✅]`   `HASH_TO_SCALAR_INTERFACE_VERSION`, a `pub const` of type `u32` with value `1`
    * `[✅]`   `HashToScalarIdentifier`, an enum with the one variant `Keccak256V1`, the identifier a deployment's hash-card names
    * `[✅]`   `HashToScalarDeclaration`, a struct with `pub identifier: HashToScalarIdentifier`, `pub adapter_version: u32`, and `pub interface_version: u32`
    * `[✅]`   `HashToScalarParams<'a>`, a struct with `pub tag: &'a DomainTag`, the domain the hash is separated into, borrowed so a consumer constructs its tag once
    * `[✅]`   `HashToScalarPayload<'a>`, a struct with `pub message: &'a [u8]`, the canonical encoding of what is hashed
    * `[✅]`   `HashToScalarSuccessReturn<S>`, a struct with `pub scalar: S`; the scalar is public, since the identity mapping and the challenge are values the contract recomputes
    * `[✅]`   `HashToScalarErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]` and the one variant `Keccak256(Keccak256HashToScalarErrorReturn)`, the Keccak-256 concrete's error carried unchanged; each further concrete's error is its own variant
    * `[✅]`   `HashToScalarReturn<S>`, the alias `Result<HashToScalarSuccessReturn<S>, HashToScalarErrorReturn>`
    * `[✅]`   `IHashToScalarAdapter<S: ISampleUniformScalar + Clone>`, an object-safe trait with `fn declaration(&self) -> HashToScalarDeclaration;` and `fn hash_to_scalar(&self, params: HashToScalarParams<'_>, payload: HashToScalarPayload<'_>) -> HashToScalarReturn<S>;`, generic over the scalar type rather than its method, so a consumer holds `Box<dyn IHashToScalarAdapter<P::Scalar>>` and obtains metadata from that adapter
    * `[✅]`   No derives on any type in this file beyond those stated; imports `DomainTag` from `crate::domain_tag::provides`, `ISampleUniformScalar` from `pairing`, and `Keccak256HashToScalarErrorReturn` from `crate::keccak256::provides`; names no vendor

  * `[✅]`   `adapters/hash-to-scalar/src/keccak256/interface.rs`
    * `[✅]`   `KECCAK256_DIGEST_LENGTH`, a `pub const` of type `usize` with value `32`
    * `[✅]`   `Keccak256HashToScalar`, the unit struct `pub struct Keccak256HashToScalar;`, the adapter over `sha3`'s Keccak-256
    * `[✅]`   `Keccak256HashToScalarConstructorParams`, the fieldless struct `pub struct Keccak256HashToScalarConstructorParams;`, the constructor's deps slot
    * `[✅]`   `Keccak256HashToScalarTryNewReturn`, the alias `Result<Keccak256HashToScalar, Infallible>`; the error arm is uninhabited because the adapter takes no configuration
    * `[✅]`   `Keccak256HashToScalarErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]` and the variants `TagLengthExceedsPrefix { length: usize }`, a tag longer than a one-byte prefix holds; `UniformLengthBelowDigest { uniform_length: usize, digest_length: usize }`, a scalar type whose sampling input cannot hold the digest; and `Sampling(SampleUniformScalarErrorReturn)`, the scalar type's sampling refusal carried unchanged
    * `[✅]`   Imports `SampleUniformScalarErrorReturn` from `pairing` and `core::convert::Infallible`; declares nothing else

  * `[✅]`   `adapters/hash-to-scalar/src/keccak256/interaction.spec.md`
    * `[✅]`   `Keccak256HashToScalar::try_new(params: Keccak256HashToScalarConstructorParams) -> Keccak256HashToScalarTryNewReturn`: one branch; outcome `Ok(Keccak256HashToScalar)`; the error arm has no branch
    * `[✅]`   `Keccak256HashToScalar::DECLARATION`: the inherent constant `HashToScalarDeclaration { identifier: HashToScalarIdentifier::Keccak256V1, adapter_version: 1, interface_version: HASH_TO_SCALAR_INTERFACE_VERSION }`
    * `[✅]`   `hash_to_scalar`, tag too long: condition `u8::try_from(params.tag.as_bytes().len())` fails; decision the conversion, first; dependency call none; outcome `Err(HashToScalarErrorReturn::Keccak256(Keccak256HashToScalarErrorReturn::TagLengthExceedsPrefix { length }))`; a `DomainTag` holds at most 255 bytes, so no input takes this branch and it has no unit test
    * `[✅]`   `hash_to_scalar`, sampling input too short: condition `S::UNIFORM_BYTES_LENGTH.checked_sub(KECCAK256_DIGEST_LENGTH)` is `None`; decision the subtraction, before hashing; dependency call none; outcome `Err(HashToScalarErrorReturn::Keccak256(Keccak256HashToScalarErrorReturn::UniformLengthBelowDigest { uniform_length: S::UNIFORM_BYTES_LENGTH, digest_length: KECCAK256_DIGEST_LENGTH }))`; every current scalar type samples from 64 bytes, so no input takes this branch and it has no unit test
    * `[✅]`   `hash_to_scalar`, sampling refused: condition `S::sample_from_uniform_bytes` returns `Err(error)`; decision the sampler's result; dependency call as below; outcome `Err(HashToScalarErrorReturn::Keccak256(Keccak256HashToScalarErrorReturn::Sampling(error)))`, the error unchanged; the input is exactly `S::UNIFORM_BYTES_LENGTH` bytes, the sampler's one refusal is a wrong length, so no input takes this branch and it has no unit test
    * `[✅]`   `hash_to_scalar`, hashed: condition both checks pass; dependency calls `Keccak256::new()`, then `update` with the one prefix byte, `update` with `params.tag.as_bytes()`, and `update` with `payload.message`, in that order, each once, then `finalize` once; a buffer of `S::UNIFORM_BYTES_LENGTH` zero bytes receives the digest in its last `KECCAK256_DIGEST_LENGTH` bytes and is moved into a `Secret` by `let Ok(uniform) = Secret::try_new(SecretConstructorParams { value: buffer });`; then `S::sample_from_uniform_bytes(SampleUniformScalarParams, SampleUniformScalarPayload { uniform })` once; outcome `Ok(HashToScalarSuccessReturn { scalar })`, `scalar` a clone of the sampled `Secret`'s exposed value, the `Secret` then dropping and zeroizing its copy
    * `[✅]`   Ordering: the prefix conversion and the length subtraction precede the hasher; the prefix, the tag, and the message are absorbed in that order; the same tag and message always yield the same scalar for one scalar type

  * `[✅]`   `adapters/hash-to-scalar/src/factory/mock.rs`

  * `[✅]`   `adapters/hash-to-scalar/src/factory/mod.rs`
    * `[✅]`   Module wiring only: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, and `pub mod provides;`, nothing else

  * `[✅]`   `adapters/hash-to-scalar/src/factory/provides.rs`
    * `[✅]`   `pub use super::interface::*;` and `#[cfg(any(test, feature = "mocks"))] pub use super::mock::*;`, nothing else

  * `[ ]`   `adapters/hash-to-scalar/src/keccak256/test.rs`

  * `[✅]`   `construction`
    * `[✅]`   `Keccak256HashToScalar::try_new` is the concrete's only producer, and its only caller is the hash-to-scalar factory, which reads `Keccak256HashToScalar::DECLARATION` before constructing
    * `[✅]`   The concrete owns no object type a consumer builds as a fixture: the adapter is a unit struct constructed by `try_new` over fieldless params and its error is an enum, so it has no `mock.rs`

  * `[✅]`   `adapters/hash-to-scalar/src/keccak256/mod.rs`
    * `[✅]`   Module declarations: `mod interface;`, `pub(crate) mod provides;`, and `#[cfg(test)] mod test;`
    * `[✅]`   `impl Keccak256HashToScalar` with `pub const DECLARATION: HashToScalarDeclaration` as the interaction spec states and `pub fn try_new(_params: Keccak256HashToScalarConstructorParams) -> Keccak256HashToScalarTryNewReturn` returning `Ok(Keccak256HashToScalar)`
    * `[✅]`   `impl<S: ISampleUniformScalar + Clone> IHashToScalarAdapter<S> for Keccak256HashToScalar` with `declaration()` returning `Self::DECLARATION` and `hash_to_scalar` realizing the branches and ordering of the interaction spec; the digest is copied into the buffer's tail by `copy_from_slice` over the range starting at the subtraction's result
    * `[✅]`   Imports the family's names from `crate::factory::provides`, `Secret` and `SecretConstructorParams` from `domain`, `ISampleUniformScalar`, `SampleUniformScalarParams`, and `SampleUniformScalarPayload` from `pairing`, `sha3::{Digest, Keccak256}`, and this module's names from `interface`
    * `[✅]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[✅]`   `adapters/hash-to-scalar/src/keccak256/provides.rs`
    * `[✅]`   `pub(crate) use super::interface::*;`, nothing else, so the concrete is visible to the crate's factory and to nothing outside the crate

  * `[✅]`   `directionality`
    * `[✅]`   `keccak256` depends on the `factory` module's surface through `crate::factory::provides`, on `domain_tag`'s through `crate::domain_tag::provides`, on `domain`, on `pairing`, and on `sha3`; the `factory` module depends on `domain_tag`, on `pairing`, and on `keccak256`'s error through `crate::keccak256::provides`, the family form's recorded cycle, which `hash-to-scalar/factory` completes by constructing the concrete
    * `[✅]`   Among repository crates the crate depends on `crates/domain` and `adapters/pairing`, which names nothing in this crate; nothing depends on the crate yet; no other cycle

  * `[ ]`   `requirements`
    * `[✅]`   `HashToScalarErrorReturn` and `Keccak256HashToScalarErrorReturn` derive `Debug`, `PartialEq`, and `Eq`
    * `[✅]`   `adapters/hash-to-scalar/Cargo.toml` carries exactly the tables and keys stated above, and `sha3` is named nowhere in the crate outside `adapters/hash-to-scalar/src/keccak256`
    * `[✅]`   `cargo check --workspace --all-targets --all-features`, `cargo fmt --all --check`, and `cargo deny check` complete without error; `cargo clippy --workspace --all-targets --all-features` reports nothing beyond the library target's unused-item warnings for the `keccak256` concrete, which `hash-to-scalar/factory` resolves by constructing the concrete
    * `[ ]`   `hash_to_scalar_hands_the_sampler_the_zero_extended_digest_of_the_reference_message` and `hash_to_scalar_hands_the_sampler_the_zero_extended_digest_of_the_empty_message` pass (CR-11, the tag prefix, serialization, and digest placement frozen against an independent implementation's digests)
    * `[ ]`   `hash_to_scalar_binds_the_tag_length` and `hash_to_scalar_separates_domains` pass
    * `[✅]`   Code outside `adapters/hash-to-scalar` naming `Keccak256HashToScalar` or anything under `keccak256` fails to compile; the crate's public surface is the `domain_tag` and `factory` modules' `provides`

* `[ ]`   `hash-to-scalar/factory` **Hash-to-scalar factory constructing the concrete the configuration names, admitted against the hash-to-scalar identifier the hash-card requires, and returning it behind the family's trait for the pairing scalar type asked for; carries the family's integration test and the milestone's commit**

  * `[✅]`   `objective`
    * `[✅]`   Problem: a consumer obtains a hash-to-scalar adapter only through the family's generic surface, never by naming a concrete, for the scalar type of the pairing it resolved, and the mapping it uses is the one the deployment's hash-card names, so a concrete that does not declare the required identifier is refused before anything is constructed (CR-08; CR-09; CR-11; Composition Boundary)
    * `[✅]`   Functional: given the concrete the configuration names and the identifier required, the factory refuses a concrete whose declared identifier is not the required one, with no construction
    * `[✅]`   Functional: an admitted concrete is constructed and returned as `Box<dyn IHashToScalarAdapter<S>>` for the scalar type `S` the caller names; the adapter reports its declaration
    * `[✅]`   Functional: a concrete's constructor error is returned unchanged in the factory's error arm, one variant per concrete
    * `[✅]`   Functional: the concrete the factory returns, handed the scalar type of the pairing the pairing factory resolves, maps the reference tag over the reference context encoded through the encoding factory to the independent scalar on each curve
    * `[✅]`   Non-functional: adding a concrete is its module, its variant in the selection enum and in the error enum, and its branch here; adding an identifier is its variant in `HashToScalarIdentifier`; no consumer changes

  * `[✅]`   `role`
    * `[✅]`   Adapter family factory: the implementation of the `factory` module, the hash-to-scalar family's construction point, and the crate's public surface beside the family-owned domain tag
    * `[✅]`   Returns the concrete behind `Box<dyn IHashToScalarAdapter<S>>`, because the family's trait is generic over the scalar type and not its method, so it is dyn-compatible for each scalar type; the factory is generic over that type and the caller names it as its pairing's `P::Scalar`
    * `[✅]`   Selects by concrete and admits by identifier, so a further concrete implementing an existing identifier and a further identifier each arrive as a variant and a branch, with no change to the params or to any consumer
    * `[✅]`   Does not read a hash-card or the configuration; the composition resolver passes the concrete the configuration names as a typed `HashToScalarConcrete` and the identifier the hash-card requires as a typed `HashToScalarIdentifier`
    * `[✅]`   Does not hash; hashing is the concrete's
    * `[✅]`   Carries the family's integration test across factory, concrete, domain tag, the pairing family that supplies the scalar type, and the encoding family that produces the message; carries the commit that closes the pairing adapters and key derivation milestone

  * `[✅]`   `module`
    * `[✅]`   Bounded context: the `factory` module of `adapters/hash-to-scalar`, holding the factory function, its deps, params, payload, and return types, its signature type, the selection enum, the function mock and builders, and the crate's integration test under `adapters/hash-to-scalar/tests`
    * `[✅]`   Outside: the concrete's behavior, every tag value and message encoding, the pairing concretes, the hash-card, the configuration catalogue, and every consumer of the family

  * `[ ]`   `deps`
    * `[✅]`   The `keccak256` concrete, through `crate::keccak256::provides`: `Keccak256HashToScalar`, `Keccak256HashToScalarConstructorParams`, `Keccak256HashToScalar::try_new`, and `Keccak256HashToScalar::DECLARATION`; the factory constructs its concrete, completing the family form's recorded cycle that `hash-to-scalar/keccak256` opened
    * `[✅]`   The `factory` module's own interface: `IHashToScalarAdapter`, `HashToScalarIdentifier`, `HashToScalarDeclaration`, and `HASH_TO_SCALAR_INTERFACE_VERSION`; `ISampleUniformScalar` from `pairing`, the factory's bound on the scalar type
    * `[ ]`   `pairing` with its `mocks` feature, the existing dev-dependency: in `factory/test.rs`, the pairing family's builder for `SampleUniformScalarSuccessReturn<S>` and its overrides, from which the test-local scalar's sampler composes its return; in the integration test, `create_pairing` and the consumer names that reach a pairing's scalar type, and `encode_scalar` to compare a scalar with its vector
    * `[✅]`   `encoding`, `adapters/encoding`, adapter ring, dev-dependency with its `mocks` feature, in the integration test only: `create_encoding` and the names that encode the reference derivation context through the encoding family; nothing at runtime, and the encoding crate names nothing in this crate
    * `[✅]`   `domain`, the existing runtime dependency, also as a dev-dependency with its `mocks` feature, in the integration test only: `DerivationContext` and the builders and overrides that make the reference context
    * `[✅]`   `hex`, the existing dev-dependency, for the vectors
    * `[✅]`   `core::convert::Infallible`, standard library, the concrete's constructor error carried in the factory's error arm; `core::marker::PhantomData`, standard library, in `factory/mock.rs`

  * `[ ]`   `context_slice`
    * `[✅]`   From the concrete: `Keccak256HashToScalar::try_new(Keccak256HashToScalarConstructorParams) -> Result<Keccak256HashToScalar, Infallible>`, the inherent constant `Keccak256HashToScalar::DECLARATION: HashToScalarDeclaration`, and `Keccak256HashToScalar`'s implementation of `IHashToScalarAdapter<S>` for every `S: ISampleUniformScalar + Clone`
    * `[ ]`   From `pairing`, in `factory/test.rs`: `ISampleUniformScalar`, which the test-local scalar implements, and the pairing family's builder for `SampleUniformScalarSuccessReturn<S>`; from `domain`: `Secret::try_new(SecretConstructorParams { value })`, which wraps the test-local scalar its sampler returns
    * `[ ]`   From `pairing`, in the integration test: `create_pairing<C: IPairingConsumer>(&CreatePairingDeps<C>, CreatePairingParams, CreatePairingPayload)`, `build_create_pairing_params` with `CreatePairingParamsOverrides`, `PairingConcrete`, `IPairingConsumer::consume_pairing<P: IPairingAdapter>`, `ConsumePairingParams`, `ConsumePairingPayload`, and `IPairingAdapter::encode_scalar(&self, EncodeScalarParams, EncodeScalarPayload { scalar })` returning 32 big-endian bytes in a `Secret`
    * `[ ]`   From `encoding`, in the integration test: `create_encoding`, `CreateEncodingDeps`, `CreateEncodingPayload`, `build_create_encoding_params` with `CreateEncodingParamsOverrides`, `EncodingConcrete::Abi`, `EncodingIdentifier::EthereumAbiV1`, `IEncodingConsumer`, `IEncoderAdapter`, `IDecoderAdapter`, `ConsumeEncodingParams`, `ConsumeEncodingPayload`, `EncodeParams`, `EncodeReturn`, and the unit value `DerivationContextDescription`

  * `[✅]`   `adapters/hash-to-scalar/Cargo.toml`
    * `[✅]`   `[dev-dependencies]` reads `domain = { path = "../../crates/domain", features = ["mocks"] }`, `encoding = { path = "../encoding", features = ["mocks"] }`, `pairing = { path = "../pairing", features = ["mocks"] }`, and `hex = "0.4.3"`
    * `[✅]`   `[package]`, `[dependencies]`, `[features]`, and `[lints]` are unchanged; no other table

  * `[✅]`   `adapters/hash-to-scalar/src/factory/interface.rs`
    * `[✅]`   `HashToScalarIdentifier` gains `#[derive(PartialEq, Eq)]`, so the admission compares a declared identifier with the required one
    * `[✅]`   `HashToScalarConcrete`, an enum with the one variant `Keccak256`, the selection of the concrete to construct
    * `[✅]`   `CreateHashToScalarDeps`, the fieldless struct `pub struct CreateHashToScalarDeps;`
    * `[✅]`   `CreateHashToScalarParams`, a struct with `pub concrete: HashToScalarConcrete` and `pub identifier: HashToScalarIdentifier`, the selection and the identifier the concrete must declare
    * `[✅]`   `CreateHashToScalarPayload`, the fieldless struct `pub struct CreateHashToScalarPayload;`, since the factory operates on no data
    * `[✅]`   `CreateHashToScalarSuccessReturn<S>`, a struct with only `pub adapter: Box<dyn IHashToScalarAdapter<S>>`; callers read the declaration through `adapter.declaration()`
    * `[✅]`   `CreateHashToScalarErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]`, which `Infallible` satisfies, and the variants `UnsupportedHashToScalarIdentifier`, the named concrete not declaring the required identifier, and `Keccak256(Infallible)`, the Keccak-256 concrete's constructor error carried unchanged; each further concrete's constructor error is its own variant
    * `[✅]`   `CreateHashToScalarReturn<S>`, the alias `Result<CreateHashToScalarSuccessReturn<S>, CreateHashToScalarErrorReturn>`
    * `[✅]`   `CreateHashToScalarFn<S>`, the alias `fn(&CreateHashToScalarDeps, CreateHashToScalarParams, CreateHashToScalarPayload) -> CreateHashToScalarReturn<S>`
    * `[✅]`   Adds the import of `core::convert::Infallible`; every item `hash-to-scalar/keccak256` authored in this file is unchanged except the derive on `HashToScalarIdentifier`

  * `[✅]`   `adapters/hash-to-scalar/src/factory/interaction.spec.md`
    * `[✅]`   `create_hash_to_scalar<S: ISampleUniformScalar + Clone>(deps: &CreateHashToScalarDeps, params: CreateHashToScalarParams, payload: CreateHashToScalarPayload) -> CreateHashToScalarReturn<S>`: decision a `match` on `params.concrete`, one arm per `HashToScalarConcrete` variant, exhaustive so a variant with no arm fails to compile
    * `[✅]`   Unsupported identifier: condition the named concrete's `DECLARATION.identifier` is not `params.identifier`; decision equality, read before any construction; dependency call none; outcome `Err(CreateHashToScalarErrorReturn::UnsupportedHashToScalarIdentifier)`, with nothing constructed; `HashToScalarIdentifier` has the one variant the Keccak-256 concrete declares, so no input takes this branch until a further identifier exists, and it has no unit test
    * `[✅]`   Admitted: condition the named concrete's declared identifier is `params.identifier`; dependency call the concrete's `try_new` with its fieldless constructor params, exactly once, its success destructured irrefutably because its error arm is uninhabited; outcome `Ok(CreateHashToScalarSuccessReturn { adapter: Box::new(hasher) })`, whose boxed adapter reports the concrete's `DECLARATION`
    * `[✅]`   `CreateHashToScalarErrorReturn::Keccak256` carries the constructor's uninhabited error type in the return union, so no branch produces it
    * `[✅]`   `params.concrete` selects and `params.identifier` admits; `S` fixes the scalar type the returned adapter produces; `deps` and `payload` carry nothing and are not read

  * `[✅]`   `adapters/hash-to-scalar/src/factory/mock.rs`

  * `[ ]`   `adapters/hash-to-scalar/src/factory/test.rs`

  * `[✅]`   `construction`
    * `[✅]`   A consumer working with a resolved pairing `P` calls `create_hash_to_scalar::<P::Scalar>` with `&CreateHashToScalarDeps`, `CreateHashToScalarParams` holding the concrete the configuration names and the identifier the hash-card requires, and `CreateHashToScalarPayload`, and holds the returned `Box<dyn IHashToScalarAdapter<P::Scalar>>` in its deps; no consumer constructs or names a concrete

  * `[✅]`   `adapters/hash-to-scalar/src/factory/mod.rs`
    * `[✅]`   Adds `#[cfg(test)] mod test;` to the wiring `hash-to-scalar/keccak256` authored
    * `[✅]`   `pub fn create_hash_to_scalar<S: ISampleUniformScalar + Clone>(_deps: &CreateHashToScalarDeps, params: CreateHashToScalarParams, _payload: CreateHashToScalarPayload) -> CreateHashToScalarReturn<S>`, a `match` on `params.concrete` whose `HashToScalarConcrete::Keccak256` arm returns the refusal when `Keccak256HashToScalar::DECLARATION.identifier` is not `params.identifier`, then binds the concrete by `let Ok(hasher) = Keccak256HashToScalar::try_new(Keccak256HashToScalarConstructorParams);` and returns `Ok(CreateHashToScalarSuccessReturn { adapter: Box::new(hasher) })`
    * `[✅]`   Imports `Keccak256HashToScalar` and `Keccak256HashToScalarConstructorParams` from `crate::keccak256::provides`, `ISampleUniformScalar` from `pairing`, and this module's types from `interface`
    * `[✅]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[✅]`   `adapters/hash-to-scalar/src/factory/provides.rs`
    * `[✅]`   Adds `pub use super::create_hash_to_scalar;` to the re-exports `hash-to-scalar/keccak256` authored

  * `[ ]`   `adapters/hash-to-scalar/tests/integration_test.rs`

  * `[✅]`   `directionality`
    * `[✅]`   The `factory` module depends on the `keccak256` concrete through `crate::keccak256::provides`, on `domain_tag`, and on `pairing`; `keccak256` depends on the `factory` module's surface, the family form's recorded cycle; the crate's public surface is the `domain_tag` and `factory` modules' `provides`
    * `[✅]`   Among repository crates the crate depends on `crates/domain` and `adapters/pairing` at runtime and on `adapters/encoding` for its integration test only; none of them names this crate; no other cycle
    * `[✅]`   `kem/bb1_depth_one` consumes the family for the identity mapping and `proof/schnorr_fs/challenge` for the challenge, each through `create_hash_to_scalar` and `IHashToScalarAdapter`

  * `[ ]`   `requirements`
    * `[✅]`   `CreateHashToScalarErrorReturn` derives `Debug`, `PartialEq`, and `Eq`
    * `[✅]`   `adapters/hash-to-scalar/Cargo.toml` carries exactly the dev-dependencies stated above, and every other table `hash-to-scalar/keccak256` stated is unchanged
    * `[ ]`   `create_hash_to_scalar_returns_an_admitted_keccak256_adapter` passes
    * `[ ]`   `the_factory_adapter_reduces_the_reference_digest_on_bn254_arkworks`, `the_factory_adapter_reduces_the_reference_digest_on_bn254_halo2curves`, `the_factory_adapter_reduces_the_empty_message_digest_on_bls12_381_arkworks`, and `the_factory_adapter_reduces_the_empty_message_digest_on_bls12_381_halo2curves` pass (CR-11, the reduction frozen against an independent implementation's vectors on both curves and all four pairing concretes; CR-08 and CR-09 for the identity mapping and the challenge that consume it)
    * `[ ]`   `the_factory_adapter_maps_the_context_the_encoding_factory_encodes` passes (CR-11, the hash-to-scalar mapping over an encoded domain value reached through the encoding, pairing, and hash-to-scalar families' surfaces)
    * `[✅]`   `cargo check --workspace --all-targets --all-features`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo fmt --all --check`, and `cargo deny check` complete without error or warning in every target, the `keccak256` concrete's unused-item warnings having no remaining cause
    * `[✅]`   `sha3` is named nowhere outside `adapters/hash-to-scalar/src/keccak256`, and no code outside `adapters/hash-to-scalar` can name `Keccak256HashToScalar`
    * `[✅]`   The pairing adapters and key derivation milestone's exit holds: CR-10 on both curves through the pairing factory, the benchmark's record, the domain identifiers and derivation context, the derivation context's ABI encoding against its known-answer vector, and CR-11's derivation and hash-to-scalar against independent implementations' vectors, every crate building and passing the Rust CI definition on Windows, macOS, and Linux

  * `[✅]`   **Commit** `feat(harness): pairing, encoding, key-derivation, and hash-to-scalar families with the domain identifiers and derivation context`
    * `[✅]`   Structural: the `adapters/pairing` crate with its factory and four concretes; the `apps/harness-crypto` crate with its benchmark; the `domain` crate's asset identity, deployment identity, suite identifier, parameter-set identifier, group index, piece geometry, and derivation context modules; the `adapters/encoding` crate with its factory, ABI concrete, and derivation-context description; the `adapters/kdf` crate with its factory and BLAKE3 keyed concrete; the `adapters/hash-to-scalar` crate with its factory, domain tag, and Keccak-256 concrete
    * `[✅]`   Behavioral: group arithmetic, precompile encodings, and scalar sampling on BN254 and BLS12-381 across arkworks and halo2curves; the pairing benchmark; the domain identifiers and the derivation context admitted only under their invariants; the derivation context encoded and decoded in its one ABI byte form; keys derived under a fixed context string per purpose; domain-tagged messages mapped to scalars on either curve
    * `[✅]`   Contract: `IPairingAdapter` with `ISampleUniformScalar` and `create_pairing`; `IEncodingContract`, `IEncoderAdapter`, `IDecoderAdapter`, and `create_encoding`; `IKeyDerivationAdapter` with `DerivationPurpose` and `create_key_derivation`; `DomainTag`, `IHashToScalarAdapter`, and `create_hash_to_scalar`; each family's identifier, declaration, and interface version

## Credential KEM

* `[ ]`   `domain/asset_identity_hash` **Asset identity hash, the 32 bytes of the Registry's `bytes32` asset key, `BLAKE3(packageName@version)`, admitting nonzero candidate keys without asserting Registry membership**

  * `[✅]`   `objective`
    * `[✅]`   Problem: the Registry keys every asset record, and the per-name version index, by the identity hash `BLAKE3(packageName@version)`, the `bytes32` `identityHash`; every delivery transcript carries the asset identity hash as a field; and under the asset scope the credential KEM's identity mapping `I` is the hash-to-scalar mapping of the asset's identity hash; so one value names the asset across the record, the transcript, and the identity mapping, and the all-zero unassigned-slot value must be refused before binding (PR-02; CR-09's transcript; `docs/research/MVP Scope.md`'s Canonical Identity and As-Is Ingestion; `docs/research/cryptography.md`'s transcript per delivery-statement version and its `Bb1DepthOneKemAdapter` statement; the technical requirements' `Asset` and `VersionIndex` schemas; the dependency map's `domain/asset_identity_hash` row; the Delivery proof and Solidity verifier milestone)
    * `[✅]`   Functional: one type holds the asset identity hash's 32 bytes, reachable only through a read accessor, and its only producer is a fallible constructor
    * `[✅]`   Functional: the constructor takes exactly 32 bytes, so the length is a fact of the params type and never a runtime check
    * `[✅]`   Functional: the constructor refuses the all-zero value, the value read from an unassigned `bytes32` storage slot; admission of any other value does not establish Registry membership
    * `[✅]`   Functional: every other 32-byte value is admitted and read back unchanged
    * `[✅]`   Non-functional: the module depends on the standard library alone; the `domain` crate's dependencies are unchanged

  * `[✅]`   `role`
    * `[✅]`   Domain: an owned value type in the protocol and domain ring, the asset key every asset record, delivery transcript, and asset-scope identity mapping names
    * `[✅]`   Does not compute the hash; the `domain` crate names no hash library, and `BLAKE3(name@version)` over `domain/asset_identity`'s join is computed through the hashing family, whose result a caller passes to this constructor
    * `[✅]`   Does not map the hash to a scalar; the credential KEM's identity mapping takes its bytes through the hash-to-scalar family
    * `[✅]`   Does not decode a hash from wire bytes of unknown length; the encoding family decodes a `bytes32` into the 32 bytes this constructor takes, and the proof family's transcript descriptions encode the held bytes as their fixed 32-byte field
    * `[✅]`   Does not look up, register, or check the Registry record the hash keys
    * `[✅]`   Does not create any other module of the `domain` crate
    * `[✅]`   Does not carry a commit

  * `[✅]`   `module`
    * `[✅]`   Bounded context: the `asset_identity_hash` module of the `domain` crate, holding `AssetIdentityHash`, its length, its constructor params, and its constructor's error and return types
    * `[✅]`   Adds the module to the existing `domain` crate at `crates/domain`; the crate's manifest is unchanged and its barrel gains this module's lines
    * `[✅]`   Outside: the hash's computation, its mapping to a scalar, the Registry record and version index it keys, and the encoding of the hash on the wire

  * `[✅]`   `deps`
    * `[✅]`   `domain/secret` created the `domain` crate this module joins; this module imports nothing from `secret`, `asset_identity`, `deployment_identity`, `suite_identifier`, `parameter_set_identifier`, `group_index`, `piece_geometry`, or `derivation_context`
    * `[✅]`   The standard library: `u8`, arrays, and `Iterator::all`, through the prelude
    * `[✅]`   No external crate and no repository crate; `crates/domain/Cargo.toml` is unchanged; direction inward, `domain` is the innermost ring
    * `[✅]`   The credential KEM's asset-scope identity mapping is this module's first consumer; proof statements and the Registry follow

  * `[✅]`   `context_slice`
    * `[✅]`   From the standard library: `<[u8]>::iter` and `Iterator::all`

  * `[✅]`   `crates/domain/src/lib.rs`
    * `[✅]`   The crate barrel reads `mod asset_identity;`, `mod asset_identity_hash;`, `mod deployment_identity;`, `mod derivation_context;`, `mod group_index;`, `mod parameter_set_identifier;`, `mod piece_geometry;`, `mod secret;`, `mod suite_identifier;`, `pub use asset_identity::provides::*;`, `pub use asset_identity_hash::provides::*;`, `pub use deployment_identity::provides::*;`, `pub use derivation_context::provides::*;`, `pub use group_index::provides::*;`, `pub use parameter_set_identifier::provides::*;`, `pub use piece_geometry::provides::*;`, `pub use secret::provides::*;`, and `pub use suite_identifier::provides::*;`, nothing else
    * `[✅]`   Until `asset_identity_hash/mod.rs` exists, `cargo check` reports the unresolved `mod asset_identity_hash`, which is the RED state for every element below that precedes the implementation

  * `[✅]`   `crates/domain/src/asset_identity_hash/interface.rs`
    * `[✅]`   `ASSET_IDENTITY_HASH_LENGTH`, a `pub const` of type `usize` with value `32`, the width of the Registry's `bytes32` asset key and of a BLAKE3 digest
    * `[✅]`   `AssetIdentityHash`, a struct with `#[derive(Clone, Debug, PartialEq, Eq)]` and the one field `pub(super) bytes: [u8; ASSET_IDENTITY_HASH_LENGTH]`, so only the `asset_identity_hash` module and its children reach the field
    * `[✅]`   `AssetIdentityHashConstructorParams`, a struct with the one field `pub bytes: [u8; ASSET_IDENTITY_HASH_LENGTH]`; no derives
    * `[✅]`   `AssetIdentityHashTryNewErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]` and the one variant `AllZero`
    * `[✅]`   `AssetIdentityHashTryNewReturn`, the type alias `Result<AssetIdentityHash, AssetIdentityHashTryNewErrorReturn>`
    * `[✅]`   Imports nothing; declares nothing else

  * `[✅]`   `crates/domain/src/asset_identity_hash/interaction.spec.md`
    * `[✅]`   `AssetIdentityHash::try_new(params: AssetIdentityHashConstructorParams) -> AssetIdentityHashTryNewReturn`, all zero: condition every byte of `params.bytes` is `0`; decision `params.bytes.iter().all(…)` over the byte equal to `0`; dependency call none; outcome `Err(AssetIdentityHashTryNewErrorReturn::AllZero)`
    * `[✅]`   Admitted: condition some byte of `params.bytes` is nonzero; decision the same check; dependency call none; outcome `Ok(AssetIdentityHash { bytes })`, the array moved from the params
    * `[✅]`   `AssetIdentityHash::as_bytes(&self) -> &[u8; ASSET_IDENTITY_HASH_LENGTH]`: one branch; outcome a shared reference to the held array, no copy, no side effect
    * `[✅]`   Ordering: the single check fully decides the outcome; the same params always yield the same outcome
    * `[✅]`   Invariants: every `AssetIdentityHash` holds exactly 32 bytes, not all zero; its only producer is `try_new`, and `Clone` copies only an already-admitted value

  * `[✅]`   `crates/domain/src/asset_identity_hash/mock.rs`

  * `[ ]`   `crates/domain/src/asset_identity_hash/test.rs`

  * `[✅]`   `construction`
    * `[✅]`   `AssetIdentityHash::try_new` is the only producer; no `Default`, `From`, or other constructor exists; a caller holding a computed digest or a decoded `bytes32` passes it as `AssetIdentityHashConstructorParams` and handles the refusal arm

  * `[✅]`   `crates/domain/src/asset_identity_hash/mod.rs`
    * `[✅]`   Module declarations: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, `pub mod provides;`, and `#[cfg(test)] mod test;`
    * `[✅]`   `impl AssetIdentityHash` with `pub fn try_new(params: AssetIdentityHashConstructorParams) -> AssetIdentityHashTryNewReturn` realizing the branches of the interaction spec, and `pub fn as_bytes(&self) -> &[u8; ASSET_IDENTITY_HASH_LENGTH]` returning `&self.bytes`
    * `[✅]`   Imports `AssetIdentityHash`, `AssetIdentityHashConstructorParams`, `AssetIdentityHashTryNewErrorReturn`, `AssetIdentityHashTryNewReturn`, and `ASSET_IDENTITY_HASH_LENGTH` from `interface`
    * `[✅]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[✅]`   `crates/domain/src/asset_identity_hash/provides.rs`
    * `[✅]`   `pub use super::interface::*;` and `#[cfg(any(test, feature = "mocks"))] pub use super::mock::*;`, nothing else, so a consumer's test and mock reach this module's builders in the crate's own test build as well as under the `mocks` feature

  * `[✅]`   `directionality`
    * `[✅]`   `asset_identity_hash` depends on the standard library alone and on no other module of the crate; `domain` depends on no repository crate; later consumers reach it through `lib.rs`'s re-export of `asset_identity_hash::provides`; no cycle
    * `[✅]`   `proof/mint_statement` and `proof/transfer_statement` carry it as the transcript's asset identity hash field; the credential KEM's asset-scope identity mapping, the Registry's asset record and version index, and the ingest and resolution workflows name the asset by it

  * `[✅]`   `requirements`
    * `[✅]`   `crates/domain/Cargo.toml` is unchanged, and `crates/domain/src/lib.rs` carries exactly the barrel stated above
    * `[✅]`   `cargo check --workspace --all-targets --all-features`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `cargo fmt --all --check` complete without error or warning
    * `[✅]`   `try_new_admits_a_hash_whose_only_nonzero_byte_is_the_last` and `try_new_admits_a_hash_whose_only_nonzero_byte_is_the_first` pass (PR-02, every other 32-byte value admitted and read back unchanged)
    * `[✅]`   `try_new_rejects_the_all_zero_hash` passes (the unassigned-slot value refused; this does not prove a Registry record exists)
    * `[✅]`   Code outside `crates/domain/src/asset_identity_hash` reading the `bytes` field fails to compile

* `[ ]`   `kem/bb1_depth_one` **Depth-one Boneh–Boyen credential KEM concrete over any pairing concrete: setup under either identity scope, identity mapping with trivial-element refusal, issuance, seller-side rerandomization, the public validity check, encapsulation, well-formedness, and decapsulation to the encapsulated value, with every value exposed as and rebuilt from its pairing components and the issuance and rerandomization scalars returned for the delivery proof; creates the `adapters/kem` crate and authors the credential KEM family's generic interface, identifier, scope declaration, components, and mock**

  * `[✅]`   `objective`
    * `[✅]`   Problem: a decryption capability must be native and distinct per ownership interval while every holder decrypts one canonical ciphertext, so the construction issues each holder a credential under a parameter set, lets a seller rerandomize its own credential for a buyer without the master scalar, lets anyone check a credential against public parameters, and lets every valid credential recover the same encapsulated value from every well-formed capsule; the construction passes through one repo-owned interface, and no module outside a concrete names its algebra (CR-08; CD-08; LC-08; EC-06; `docs/research/cryptography.md`'s Credential KEM statement; `docs/research/cryptography-critical-path.md`'s construction)
    * `[✅]`   Functional: setup samples the master scalar `α` and the identity-base scalars `a` and `b` from caller-supplied uniform bytes through the pairing scalar type's sampling bound, and returns the parameter set `g1`, `u0 = a·g1`, `u1 = b·g1`, `g2`, `hpub = α·g2` with its identity scope, and the master scalar inside a `Secret`
    * `[✅]`   Functional: under the asset scope, setup fixes the parameter set's identity element `F = u0 + I·u1` from the 32 bytes of the admitted asset identity hash, refusing a trivial `F`; under the entitlement scope the parameter set fixes no identity
    * `[✅]`   Functional: the identity mapping hashes the supplied identity representation to the scalar `I` under the KEM's domain tag through the hash-to-scalar family, computes `F_I = u0 + I·u1`, refuses an `F_I` that is the identity element, and under the asset scope refuses any identity whose element is not the parameter set's fixed `F`; production composition supplies canonical entitlement bytes or borrowed `AssetIdentityHash` according to scope
    * `[✅]`   Functional: issuance samples `r` from caller-supplied uniform bytes and returns the credential `(A, B) = (α·g1 + r·F, r·g2)` with `r` inside a `Secret`, the mint proof's witness; rerandomization samples `s` and returns `(A + s·F, B + s·g2)` from a credential alone, with no master scalar, and with `s` inside a `Secret`, the transfer proof's witness
    * `[✅]`   Functional: validity is the public pairing check `e(A, g2) = e(g1, hpub) · e(F, B)`, computed as one pairing-product check over negated first-group inputs
    * `[✅]`   Functional: encapsulation samples `t` from caller-supplied uniform bytes and returns the capsule `(U, V, W) = (t·g2, t·u0, t·u1)` under the entitlement scope or `(U, V) = (t·g2, t·F)` under the asset scope, with the encapsulated value `K = e(t·g1, hpub)` as its target-group encoding inside a `Secret`
    * `[✅]`   Functional: well-formedness is `e(V, g2) = e(u0, U)` and `e(W, g2) = e(u1, U)` under the entitlement scope and `e(V, g2) = e(F, U)` under the asset scope, a capsule of the other scope's form being ill-formed
    * `[✅]`   Functional: decapsulation returns `K` as `e(A, U) / e(V + I·W, B)` for an entitlement-scope capsule and `e(A, U) / e(V, B)` for an asset-scope capsule, encoded as encapsulation encodes it, so every valid credential for an identity, issued or rerandomized, and every entitlement's credential under one entitlement-scope parameter set, recovers the same bytes
    * `[✅]`   Functional: a credential is invalid for another entitlement's identity and under another parameter set; a capsule with its identity-base elements exchanged, or checked under another parameter set, is ill-formed
    * `[✅]`   Functional: every concrete declares the KEM identifier a deployment's hash-card names, the identity scopes it supports, the domain tag its identity mapping hashes under, its adapter version, and the interface version it implements, readable before any instance exists, so a consumer that mirrors the mapping reads the tag from the declaration and names no concrete (CD-08; CR-11)
    * `[✅]`   Functional: the family's interface names the pairing concrete a KEM computes over, so a consumer that holds that pairing receives the KEM's values in the pairing's own scalar and group types and combines them with the envelope's and the delivery proof's
    * `[✅]`   Functional: the credential is exposed as its components `(A, B)` and rebuilt from them, so the envelope encrypts them and the credential engine rebuilds a credential from a decrypted envelope; the parameter set as `g1`, `u0`, `u1`, `g2`, `hpub` and its scope with any fixed `F`, so the registry registers it, the delivery proof states it, and a holder rebuilds it from registry state; the identity element as `(I, F)`, so the contract's recomputed `I` and the proof's `F_I` are read from it; the capsule as its scope's elements, so the sidecar layer encodes and rebuilds it; and the master scalar as `α` inside a `Secret`, so the mint proof takes it as a witness and custody persists and restores it
    * `[✅]`   Functional: rebuilding a value from components checks nothing beyond its types; `is_valid` checks a rebuilt credential and `is_well_formed` a rebuilt capsule, and the pairing family's decoders have already checked every point's encoding and subgroup
    * `[✅]`   Non-functional: every source-group, scalar, and target-group operation passes through the pairing family's `IPairingArithmetic` and every identity mapping through `IHashToScalarAdapter`; the crate names no curve library, no hash library, and no randomness source; decrypt-capable values, the master scalar, the credential, the issuance and rerandomization scalars, their components, and `K`, are held in types that zeroize on drop

  * `[✅]`   `role`
    * `[✅]`   Adapter: the credential KEM family's first concrete, and the first source file that requires the family's generic interface, the identity scope, the KEM identifier, the declaration, the encapsulated-value type, the component types, and the family's mock, which it authors in the family's `factory` module as its producers
    * `[✅]`   Creates `factory/mod.rs` with the module wiring alone and `factory/provides.rs` with the interface and mock surface alone; the factory function, its types, its interaction spec, its unit test, its re-export, the admission by identity scope, and the family's integration test are `kem/factory`'s
    * `[✅]`   Takes every random scalar as caller-supplied uniform bytes, so the First Finder draws them from the randomness family and the explicit publisher derives them through the key-derivation family's `MasterScalar`, `IdentityBases`, and `CapsuleRandomness` purposes, with no change here (CR-05)
    * `[✅]`   Takes identity bytes only at the KEM's final hash boundary: composition supplies a borrowed `AssetIdentityHash` for asset scope and the canonical encoding of the entitlement identifier for entitlement scope. These representations are distinct; the asset coordinate's `name@version` bytes are not the asset identity hash. The KEM does not assemble either representation
    * `[✅]`   Does not choose a scope for a deployment; the suite fixes it, and `kem/factory` admits a concrete by the scope the suite requires
    * `[✅]`   Does not wrap a piece-group key or derive a wrapping key; `workflows/sidecar/wrap` derives from the encapsulated value
    * `[✅]`   Does not encode, decode, subgroup-check, or register a parameter set, credential, or capsule; it exposes each as pairing components and rebuilds each from them, the pairing family's decoders check encodings and subgroups, and the registry, envelope, sidecar, and custody consumers encode what they carry
    * `[✅]`   Does not prove delivery or encrypt an envelope; it returns the scalars and components `envelope/pairing_elgamal` and `proof/schnorr_fs` take
    * `[✅]`   Does not carry a commit

  * `[✅]`   `module`
    * `[✅]`   Bounded context: the `adapters/kem` crate's `factory` module, holding the family's generic trait with its associated pairing, parameter-set, master-scalar, identity-element, credential, and capsule types, every method's params, payload, success, error, and return types, the identity scope, the setup scope, the encapsulated value, the credential, parameter-set, identity-element, capsule, and master-scalar components, the KEM identifier, the declaration, the interface version, and the family's mock; and its private `bb1_depth_one` concrete, holding the adapter generic over a pairing concrete, its identity tag, its constructor params and return, its parameter-set, scope, master-scalar, identity-element, credential, and capsule types, and its per-method errors
    * `[✅]`   Creates the crate at `adapters/kem`, admitted by the workspace's `adapters/*` glob with no edit to the root manifest
    * `[✅]`   Outside: the source of uniform bytes, the encoding of identifiers, the wrap of piece-group keys, envelopes, delivery proofs, the factory's selection and scope admission, custody, and every wire form of the KEM's values and components

  * `[ ]`   `deps`
    * `[✅]`   `domain`, `crates/domain`, protocol and domain ring, path dependency; supplies `Secret` and `AssetIdentityHash`, the typed candidate asset key (not proof of Registry membership), and the wrapper for uniform bytes, the master scalar, and `K`; direction inward
    * `[ ]`   `domain` with its `mocks` feature, as a dev-dependency and through this crate's `mocks` feature; supplies `build_secret` and `SecretConstructorParamsOverrides`, and, in `bb1_depth_one/test.rs`, `build_asset_identity_hash` and `AssetIdentityHashConstructorParamsOverrides`
    * `[✅]`   `zeroize` `1.9.0`, external crate, Apache-2.0 OR MIT, runtime dependency, the version `domain` and `pairing` pin; supplies the `Zeroize` trait the component and success types bound their scalar by, since `Secret` requires it and `domain` does not re-export it
    * `[✅]`   `pairing`, `adapters/pairing`, adapter ring, path dependency; supplies `IPairingAdapter` with its `Scalar`, `G1`, and `G2` associated types and its generator, addition, multiplication, multi-scalar multiplication, pairing-product check, and first-group encoding methods; `IPairingArithmetic` with its `Gt` associated type and its scalar negation, first-group negation, identity test, pairing product, and target-group encoding methods; `ISampleUniformScalar` and `SampleUniformScalarErrorReturn`; and each method's params, payload, and term types; the pairing crate names nothing in this crate
    * `[✅]`   `hash-to-scalar`, `adapters/hash-to-scalar`, adapter ring, path dependency; supplies `IHashToScalarAdapter`, `HashToScalarParams`, `HashToScalarPayload`, `HashToScalarErrorReturn`, `DomainTag`, `DomainTagConstructorParams`, and `DomainTagTryNewErrorReturn`; the hash-to-scalar crate names nothing in this crate
    * `[ ]`   `pairing` and `hash-to-scalar`, each with its `mocks` feature, as dev-dependencies, in `bb1_depth_one/test.rs` and `bb1_depth_one/mock.rs` only: the pairing family's official mock adapter, the struct the `pairing` mock surface provides implementing `IPairingArithmetic`, located by that implementation; and `MockIHashToScalarAdapter`, `build_hash_to_scalar_declaration` with `HashToScalarDeclarationOverrides`, and `build_domain_tag` with `DomainTagConstructorParamsOverrides`
    * `[✅]`   `core::convert::Infallible`, standard library, the error arm of every infallible method; `core::marker::PhantomData`, standard library, in `factory/mock.rs` only
    * `[✅]`   No external crate beyond `zeroize`; no reverse dependency beyond the family form's recorded cycle: the `factory` module's error enums carry this concrete's errors; nothing depends on the crate yet

  * `[ ]`   `context_slice`
    * `[✅]`   From `IPairingAdapter`: `g1_generator(G1GeneratorParams, G1GeneratorPayload)` and `g2_generator(G2GeneratorParams, G2GeneratorPayload)` returning `{ point }`; `add_g1` and `add_g2` over `{ left, right }` returning `{ sum }`; `mul_g1` over `MulG1Payload { point, scalar }` and `mul_g2` over `MulG2Payload { point, scalar }` returning `{ product }`; `msm_g1` over `MsmG1Payload { terms }` of `MsmG1Term { base, scalar }` returning `{ sum }`; `pairing_product_is_one` over `PairingProductIsOnePayload { terms }` of `PairingProductTerm { g1, g2 }` returning `{ is_one }`; `encode_g1` over `EncodeG1Payload { point }` returning `{ bytes: Self::EncodedG1 }`; `encode_g2` over `EncodeG2Payload { point }` returning `{ bytes: Self::EncodedG2 }`, in the tests only; every one `Result<…, Infallible>`; `Scalar: ISampleUniformScalar + Clone`, `G1: Clone`, `G2: Clone`
    * `[✅]`   From `IPairingArithmetic`: `neg_scalar` over `NegScalarPayload { scalar }` returning `{ negation }`, in the tests only; `neg_g1` over `NegG1Payload { point }` returning `{ negation }`; `is_identity_g1` over `IsIdentityG1Payload { point }` returning `{ is_identity }`; `pairing_product` over `PairingProductPayload { terms }` returning `{ product: Self::Gt }`; `encode_gt` over `EncodeGtPayload { value }` returning `{ bytes: Secret<Self::EncodedGt> }`; every one `Result<…, Infallible>`; `G1: Zeroize`, `G2: Zeroize`, `Gt: Zeroize`, `EncodedGt: AsRef<[u8]> + Zeroize`
    * `[✅]`   From `ISampleUniformScalar`: `UNIFORM_BYTES_LENGTH`, `64` on every current scalar type, and `sample_from_uniform_bytes(SampleUniformScalarParams, SampleUniformScalarPayload { uniform: Secret<Vec<u8>> })` returning `Result<SampleUniformScalarSuccessReturn<S> { scalar: Secret<S> }, SampleUniformScalarErrorReturn>`, whose one refusal is `WrongLength { expected, actual }`
    * `[✅]`   From `IHashToScalarAdapter<S>`: `hash_to_scalar(HashToScalarParams { tag: &DomainTag }, HashToScalarPayload { message: &[u8] })` returning `Result<HashToScalarSuccessReturn<S> { scalar: S }, HashToScalarErrorReturn>`; and `DomainTag::try_new(DomainTagConstructorParams { bytes })` returning `Result<DomainTag, DomainTagTryNewErrorReturn>`, admitting between 1 and 255 bytes of `0x20` through `0x7E` with no space at either edge
    * `[✅]`   From `domain`: `AssetIdentityHash::as_bytes(&self) -> &[u8; 32]`, `Secret::try_new(SecretConstructorParams { value })` returning `Result<Secret<T>, Infallible>`, and `Secret::expose(&self) -> &T`
    * `[ ]`   In `bb1_depth_one/test.rs` and `bb1_depth_one/mock.rs`: the pairing family's official mock adapter, implementing `IPairingAdapter` and `IPairingArithmetic` and returning built defaults; `MockIHashToScalarAdapter<S> { scalar: PhantomData<S> }`, implementing `IHashToScalarAdapter<S>` for `S: ISampleUniformScalar + Clone + Default`; `build_hash_to_scalar_declaration(HashToScalarDeclarationOverrides) -> HashToScalarDeclaration`; `build_domain_tag(DomainTagConstructorParamsOverrides) -> DomainTag`; and `build_asset_identity_hash(AssetIdentityHashConstructorParamsOverrides) -> AssetIdentityHash`

  * `[✅]`   `adapters/kem/Cargo.toml`
    * `[✅]`   `[package]` with `name = "kem"`, `edition.workspace = true`, `rust-version.workspace = true`, and `publish.workspace = true`; no `version` key
    * `[✅]`   `[dependencies]` with `domain = { path = "../../crates/domain" }`, `zeroize = "1.9.0"`, `pairing = { path = "../pairing" }`, and `hash-to-scalar = { path = "../hash-to-scalar" }`
    * `[✅]`   `[dev-dependencies]` with `domain = { path = "../../crates/domain", features = ["mocks"] }`, `pairing = { path = "../pairing", features = ["mocks"] }`, and `hash-to-scalar = { path = "../hash-to-scalar", features = ["mocks"] }`
    * `[✅]`   `[features]` with `mocks = ["domain/mocks"]`
    * `[✅]`   `[lints]` with `workspace = true`
    * `[✅]`   No other table

  * `[✅]`   `adapters/kem/src/lib.rs`
    * `[✅]`   The crate barrel: `mod bb1_depth_one;`, `mod factory;`, and `pub use factory::provides::*;`, nothing else; the concrete's surface is not re-exported
    * `[✅]`   Until `factory/mod.rs` and `bb1_depth_one/mod.rs` exist, `cargo check` reports the unresolved modules, which is the RED state for every element below that precedes them

  * `[✅]`   `adapters/kem/src/factory/interface.rs`
    * `[✅]`   `KEM_INTERFACE_VERSION`, a `pub const` of type `u32` with value `1`
    * `[✅]`   `KemIdentifier`, an enum with the one variant `Bb1DepthOneV1`, the identifier a deployment's hash-card names for its credential KEM
    * `[✅]`   `IdentityScope`, an enum with `#[derive(Clone, Copy, Debug, PartialEq, Eq)]` and the variants `Entitlement` and `Asset`
    * `[✅]`   `KemDeclaration`, a struct with `pub identifier: KemIdentifier`, `pub identity_scopes: &'static [IdentityScope]`, `pub identity_tag: &'static [u8]`, the domain tag the concrete's identity mapping hashes under, `pub adapter_version: u32`, and `pub interface_version: u32`
    * `[✅]`   `EncapsulatedValue`, a struct with a crate-private `Secret<Vec<u8>>` field and no public field constructor; only the KEM's `encapsulate` and `decapsulate` paths construct it by copying the bytes of `Secret<P::EncodedGt>` returned by that pairing's `encode_gt` into a new `Secret<Vec<u8>>`. Its public `key_material(&self) -> &Secret<Vec<u8>>` borrows the admitted encoding for the byte-oriented KDF without exposing a mutable field. The curve-specific encoding type is retained through production and narrowed to secret KDF material at this boundary; no arbitrary secret vector is an `EncapsulatedValue`
    * `[✅]`   Setup: `SetupScope<'a>`, an enum with the variants `Entitlement` and `Asset { identity_hash: &'a AssetIdentityHash }`, the scope the parameter set is fixed to and, under the asset scope, the candidate asset key borrowed as a typed value until the hash-to-scalar call; `SetupParams<'a>` with `pub scope: SetupScope<'a>`; `SetupPayload` with `pub master_uniform: Secret<Vec<u8>>`, `pub u0_uniform: Secret<Vec<u8>>`, and `pub u1_uniform: Secret<Vec<u8>>`; `SetupSuccessReturn<PS, M>` with `pub parameter_set: PS` and `pub master_scalar: M`; `SetupErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]` and the one variant `Bb1DepthOne(Bb1DepthOneSetupErrorReturn)`; `SetupReturn<PS, M>`, the alias `Result<SetupSuccessReturn<PS, M>, SetupErrorReturn>`
    * `[✅]`   Identity mapping: `KemIdentity<'a>`, an enum with `Entitlement { canonical: &'a [u8] }` and `Asset { identity_hash: &'a AssetIdentityHash }`, preserving the role until the final hash call; the fieldless `DeriveIdentityParams`; `DeriveIdentityPayload<'a, PS>` with `pub parameter_set: &'a PS` and `pub identity: KemIdentity<'a>`; `DeriveIdentitySuccessReturn<IE>` with `pub identity_element: IE`; `DeriveIdentityErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]` and the one variant `Bb1DepthOne(Bb1DepthOneDeriveIdentityErrorReturn)`; `DeriveIdentityReturn<IE>`, the alias `Result<DeriveIdentitySuccessReturn<IE>, DeriveIdentityErrorReturn>`
    * `[✅]`   Issuance: the fieldless `IssueParams`; `IssuePayload<'a, PS, M, IE>` with `pub parameter_set: &'a PS`, `pub master_scalar: &'a M`, `pub identity_element: &'a IE`, and `pub uniform: Secret<Vec<u8>>`; `IssueSuccessReturn<C, S: Zeroize>` with `pub credential: C` and `pub randomness: Secret<S>`, the sampled `r`; `IssueErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]` and the one variant `Bb1DepthOne(Bb1DepthOneIssueErrorReturn)`; `IssueReturn<C, S>`, the alias `Result<IssueSuccessReturn<C, S>, IssueErrorReturn>`
    * `[✅]`   Rerandomization: the fieldless `RerandomizeParams`; `RerandomizePayload<'a, PS, IE, C>` with `pub parameter_set: &'a PS`, `pub identity_element: &'a IE`, `pub credential: &'a C`, and `pub uniform: Secret<Vec<u8>>`; `RerandomizeSuccessReturn<C, S: Zeroize>` with `pub credential: C` and `pub offset: Secret<S>`, the sampled `s`; `RerandomizeErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]` and the one variant `Bb1DepthOne(Bb1DepthOneRerandomizeErrorReturn)`; `RerandomizeReturn<C, S>`, the alias `Result<RerandomizeSuccessReturn<C, S>, RerandomizeErrorReturn>`
    * `[✅]`   Validity: the fieldless `IsValidParams`; `IsValidPayload<'a, PS, IE, C>` with `pub parameter_set: &'a PS`, `pub identity_element: &'a IE`, and `pub credential: &'a C`; `IsValidSuccessReturn` with `pub is_valid: bool`; `IsValidReturn`, the alias `Result<IsValidSuccessReturn, Infallible>`
    * `[✅]`   Encapsulation: the fieldless `EncapsulateParams`; `EncapsulatePayload<'a, PS>` with `pub parameter_set: &'a PS` and `pub uniform: Secret<Vec<u8>>`; `EncapsulateSuccessReturn<CA>` with `pub capsule: CA` and `pub encapsulated: EncapsulatedValue`; `EncapsulateErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]` and the one variant `Bb1DepthOne(Bb1DepthOneEncapsulateErrorReturn)`; `EncapsulateReturn<CA>`, the alias `Result<EncapsulateSuccessReturn<CA>, EncapsulateErrorReturn>`
    * `[✅]`   Well-formedness: the fieldless `IsWellFormedParams`; `IsWellFormedPayload<'a, PS, CA>` with `pub parameter_set: &'a PS` and `pub capsule: &'a CA`; `IsWellFormedSuccessReturn` with `pub is_well_formed: bool`; `IsWellFormedReturn`, the alias `Result<IsWellFormedSuccessReturn, Infallible>`
    * `[✅]`   Decapsulation: the fieldless `DecapsulateParams`; `DecapsulatePayload<'a, IE, C, CA>` with `pub identity_element: &'a IE`, `pub credential: &'a C`, and `pub capsule: &'a CA`; `DecapsulateSuccessReturn` with `pub encapsulated: EncapsulatedValue`; `DecapsulateReturn`, the alias `Result<DecapsulateSuccessReturn, Infallible>`
    * `[✅]`   The components, each generic over the pairing's scalar `S` and group types `G1` and `G2`: `CredentialComponents<G1, G2>` with `pub a: G1` and `pub b: G2`; `ParameterSetScopeComponents<G1>`, an enum with the variants `Entitlement` and `Asset { identity_element: G1 }`; `ParameterSetComponents<G1, G2>` with `pub g1: G1`, `pub u0: G1`, `pub u1: G1`, `pub g2: G2`, `pub hpub: G2`, and `pub scope: ParameterSetScopeComponents<G1>`; `IdentityElementComponents<S, G1>` with `pub scalar: S` and `pub element: G1`; `CapsuleComponents<G1, G2>`, an enum with the variants `Entitlement { u: G2, v: G1, w: G1 }` and `Asset { u: G2, v: G1 }`; `MasterScalarComponents<S: Zeroize>` with `pub value: Secret<S>`
    * `[✅]`   Component access, one method pair per value, each method with its fieldless `…Params`, its `…Payload`, its `…SuccessReturn`, and its `…Return` alias of `Result<…SuccessReturn, Infallible>`, named by the method in upper camel case: `credential_components` over `CredentialComponentsPayload<'a, C>` with `pub credential: &'a C` returning `CredentialComponentsSuccessReturn<G1, G2>` with `pub components: CredentialComponents<G1, G2>`, and `credential_from_components` over `CredentialFromComponentsPayload<G1, G2>` with `pub components: CredentialComponents<G1, G2>` returning `CredentialFromComponentsSuccessReturn<C>` with `pub credential: C`; `parameter_set_components` over `ParameterSetComponentsPayload<'a, PS>` with `pub parameter_set: &'a PS` returning `ParameterSetComponentsSuccessReturn<G1, G2>` with `pub components: ParameterSetComponents<G1, G2>`, and `parameter_set_from_components` over `ParameterSetFromComponentsPayload<G1, G2>` with `pub components` returning `ParameterSetFromComponentsSuccessReturn<PS>` with `pub parameter_set: PS`; `identity_element_components` over `IdentityElementComponentsPayload<'a, IE>` with `pub identity_element: &'a IE` returning `IdentityElementComponentsSuccessReturn<S, G1>` with `pub components: IdentityElementComponents<S, G1>`, the identity element being rebuilt only by `derive_identity`; `capsule_components` over `CapsuleComponentsPayload<'a, CA>` with `pub capsule: &'a CA` returning `CapsuleComponentsSuccessReturn<G1, G2>` with `pub components: CapsuleComponents<G1, G2>`, and `capsule_from_components` over `CapsuleFromComponentsPayload<G1, G2>` with `pub components` returning `CapsuleFromComponentsSuccessReturn<CA>` with `pub capsule: CA`; `master_scalar_components` over `MasterScalarComponentsPayload<'a, M>` with `pub master_scalar: &'a M` returning `MasterScalarComponentsSuccessReturn<S: Zeroize>` with `pub components: MasterScalarComponents<S>`, and `master_scalar_from_components` over `MasterScalarFromComponentsPayload<S: Zeroize>` with `pub components` returning `MasterScalarFromComponentsSuccessReturn<M>` with `pub master_scalar: M`
    * `[✅]`   `ICredentialKemAdapter`, a trait with `const DECLARATION: KemDeclaration;` and the associated types `type Pairing: IPairingAdapter;`, `type ParameterSet;`, `type MasterScalar;`, `type IdentityElement;`, `type Credential;`, and `type Capsule;`; writing `S`, `G1`, and `G2` below for `<Self::Pairing as IPairingAdapter>::Scalar`, `::G1`, and `::G2`, the methods `setup(&self, params: SetupParams<'_>, payload: SetupPayload) -> SetupReturn<Self::ParameterSet, Self::MasterScalar>`, `derive_identity(&self, params: DeriveIdentityParams, payload: DeriveIdentityPayload<'_, Self::ParameterSet>) -> DeriveIdentityReturn<Self::IdentityElement>`, `issue(&self, params: IssueParams, payload: IssuePayload<'_, Self::ParameterSet, Self::MasterScalar, Self::IdentityElement>) -> IssueReturn<Self::Credential, S>`, `rerandomize(&self, params: RerandomizeParams, payload: RerandomizePayload<'_, Self::ParameterSet, Self::IdentityElement, Self::Credential>) -> RerandomizeReturn<Self::Credential, S>`, `is_valid(&self, params: IsValidParams, payload: IsValidPayload<'_, Self::ParameterSet, Self::IdentityElement, Self::Credential>) -> IsValidReturn`, `encapsulate(&self, params: EncapsulateParams, payload: EncapsulatePayload<'_, Self::ParameterSet>) -> EncapsulateReturn<Self::Capsule>`, `is_well_formed(&self, params: IsWellFormedParams, payload: IsWellFormedPayload<'_, Self::ParameterSet, Self::Capsule>) -> IsWellFormedReturn`, `decapsulate(&self, params: DecapsulateParams, payload: DecapsulatePayload<'_, Self::IdentityElement, Self::Credential, Self::Capsule>) -> DecapsulateReturn`, and the component-access methods above over the associated types and `S`, `G1`, and `G2`
    * `[✅]`   No derives beyond those stated; imports `domain::{AssetIdentityHash, Secret}`, `zeroize::Zeroize`, the bound `Secret` places on what it holds, `IPairingAdapter` from `pairing`, `core::convert::Infallible`, and the five concrete error types from `crate::bb1_depth_one::provides`; names no curve, hash, or vendor

  * `[✅]`   `adapters/kem/src/bb1_depth_one/interface.rs`
    * `[✅]`   `BB1_DEPTH_ONE_IDENTITY_TAG`, a `pub const` of type `&[u8]` with value `b"ChainTorrent-v1-kem-identity"`, the identity mapping's domain tag, which the concrete's declaration carries as `identity_tag` and the generate family mirrors to Solidity from that declaration
    * `[✅]`   `BB1_DEPTH_ONE_DECLARATION`, a `pub(crate) const KemDeclaration` defined once beside the BB1 adapter, with `identifier: KemIdentifier::Bb1DepthOneV1`, `identity_scopes: &[IdentityScope::Entitlement, IdentityScope::Asset]`, `identity_tag: BB1_DEPTH_ONE_IDENTITY_TAG`, `adapter_version: 1`, and `interface_version: KEM_INTERFACE_VERSION`; the concrete's inherent declaration, its trait-associated declaration, the mock adapter, and the default declaration builder refer to this constant
    * `[✅]`   `Bb1DepthOneKem<'a, P: IPairingArithmetic>`, a struct with `pub(super) pairing: &'a P`, `pub(super) hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>`, and `pub(super) tag: DomainTag`, the adapter; it borrows its pairing and hash-to-scalar adapters, so the caller that resolved the pairing keeps it for the envelope and the delivery proof, which compute in the same groups
    * `[✅]`   `Bb1DepthOneKemConstructorParams<'a, P: IPairingArithmetic>`, a struct with `pub pairing: &'a P` and `pub hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>`, the constructor's deps slot
    * `[✅]`   `Bb1DepthOneKemTryNewErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]` and the one variant `DomainTag(DomainTagTryNewErrorReturn)`; `Bb1DepthOneKemTryNewReturn<'a, P>`, the alias `Result<Bb1DepthOneKem<'a, P>, Bb1DepthOneKemTryNewErrorReturn>`
    * `[✅]`   `Bb1DepthOneParameterSetScope<G>`, an enum with the variants `Entitlement` and `Asset { identity_element: G }`, the asset scope carrying its fixed `F`
    * `[✅]`   `Bb1DepthOneParameterSet<P: IPairingAdapter>`, a struct with `pub(super) g1: P::G1`, `pub(super) u0: P::G1`, `pub(super) u1: P::G1`, `pub(super) g2: P::G2`, `pub(super) hpub: P::G2`, and `pub(super) scope: Bb1DepthOneParameterSetScope<P::G1>`
    * `[✅]`   `Bb1DepthOneMasterScalar<P: IPairingAdapter>`, a struct with `pub(super) value: Secret<P::Scalar>`
    * `[✅]`   `Bb1DepthOneIdentityElement<P: IPairingAdapter>`, a struct with `pub(super) scalar: P::Scalar`, the public `I`, and `pub(super) element: P::G1`, `F`
    * `[✅]`   `Bb1DepthOneCredential<P: IPairingAdapter>`, a struct with `pub(super) a: P::G1` and `pub(super) b: P::G2`, each zeroized on drop by its pairing type
    * `[✅]`   `Bb1DepthOneCapsule<P: IPairingAdapter>`, an enum with the variants `Entitlement { u: P::G2, v: P::G1, w: P::G1 }` and `Asset { u: P::G2, v: P::G1 }`
    * `[✅]`   `Bb1DepthOneSetupErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]` and the variants `MasterScalarSampling(SampleUniformScalarErrorReturn)`, `U0Sampling(SampleUniformScalarErrorReturn)`, `U1Sampling(SampleUniformScalarErrorReturn)`, `HashToScalar(HashToScalarErrorReturn)`, and `TrivialIdentityElement`
    * `[✅]`   `Bb1DepthOneDeriveIdentityErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]` and the variants `WrongIdentityScope`, `HashToScalar(HashToScalarErrorReturn)`, `TrivialIdentityElement`, and `OutsideAssetScope`
    * `[✅]`   `Bb1DepthOneIssueErrorReturn`, `Bb1DepthOneRerandomizeErrorReturn`, and `Bb1DepthOneEncapsulateErrorReturn`, each an enum with `#[derive(Debug, PartialEq, Eq)]` and the one variant `Sampling(SampleUniformScalarErrorReturn)`
    * `[✅]`   No derives on any type in this file beyond those stated; imports `AssetIdentityHash` and `Secret` from `domain`, `IPairingAdapter`, `IPairingArithmetic`, and `SampleUniformScalarErrorReturn` from `pairing`, and `IHashToScalarAdapter`, `HashToScalarErrorReturn`, `DomainTag`, and `DomainTagTryNewErrorReturn` from `hash_to_scalar`; imports `KemDeclaration`, `KemIdentifier`, `IdentityScope`, and `KEM_INTERFACE_VERSION` from `crate::factory::provides` for the one declaration constant; declares nothing else

  * `[✅]`   `adapters/kem/src/bb1_depth_one/interaction.spec.md`
    * `[✅]`   `Bb1DepthOneKem::try_new(params: Bb1DepthOneKemConstructorParams<'a, P>) -> Bb1DepthOneKemTryNewReturn<'a, P>`, tag refused: condition `DomainTag::try_new(DomainTagConstructorParams { bytes: BB1_DEPTH_ONE_IDENTITY_TAG.to_vec() })` returns `Err(error)`; decision the constructor's result; dependency call `DomainTag::try_new` once; outcome `Err(Bb1DepthOneKemTryNewErrorReturn::DomainTag(error))`, the refusal unchanged; the tag is 28 visible-ASCII bytes, so no input takes this branch and it has no unit test
    * `[✅]`   `try_new`, admitted: outcome `Ok(Bb1DepthOneKem { pairing, hash_to_scalar, tag })`, holding the params' borrows
    * `[✅]`   `Bb1DepthOneKem::<'_, P>::DECLARATION`: the inherent constant `KemDeclaration = BB1_DEPTH_ONE_DECLARATION`, its tag the constant `try_new` constructs the adapter's `DomainTag` from
    * `[✅]`   Sampling, the shared step every sampling branch below names: `P::Scalar::sample_from_uniform_bytes(SampleUniformScalarParams, SampleUniformScalarPayload { uniform })` with the payload's uniform bytes moved in, its `Err(error)` returned in the method's own variant and its `Ok` yielding a `Secret<P::Scalar>` whose exposed value is cloned into each pairing payload that needs it
    * `[✅]`   Identity mapping, the shared step setup and `derive_identity` name: borrow `canonical` from `KemIdentity::Entitlement` or call `identity_hash.as_bytes()` on `KemIdentity::Asset` only when building `HashToScalarPayload`, the setup asset hash being the latter; `self.hash_to_scalar.hash_to_scalar(HashToScalarParams { tag: &self.tag }, HashToScalarPayload { message: bytes })` yields `I`, its `Err(error)` returned in the method's `HashToScalar` variant; then `F = add_g1(u0, mul_g1(u1, I))`; then `is_identity_g1(F)`, `true` returned in the method's `TrivialIdentityElement` variant
    * `[✅]`   `setup`, sampling refused: condition the master, `u0`, or `u1` uniform bytes are refused, sampled in that order; outcome `Err(SetupErrorReturn::Bb1DepthOne(…))` holding `MasterScalarSampling`, `U0Sampling`, or `U1Sampling` with the refusal unchanged, for the first refused
    * `[✅]`   `setup`, entitlement scope: condition `params.scope` is `SetupScope::Entitlement` and all three samplings succeed; dependency calls `g1_generator`, `g2_generator`, `mul_g1(g1, a)` for `u0`, `mul_g1(g1, b)` for `u1`, and `mul_g2(g2, α)` for `hpub`, each once; outcome `Ok(SetupSuccessReturn { parameter_set, master_scalar })` with the scope `Bb1DepthOneParameterSetScope::Entitlement` and the master scalar the sampled `Secret` of `α` in `Bb1DepthOneMasterScalar`; the sampled `a` and `b` drop and zeroize
    * `[✅]`   `setup`, asset scope refused: condition `params.scope` is `SetupScope::Asset { identity_hash }` and the identity mapping over `KemIdentity::Asset { identity_hash }` and the new `u0`, `u1` fails; outcome `Err(SetupErrorReturn::Bb1DepthOne(Bb1DepthOneSetupErrorReturn::HashToScalar(error)))` or `Err(SetupErrorReturn::Bb1DepthOne(Bb1DepthOneSetupErrorReturn::TrivialIdentityElement))`; `a` and `b` are uniform, so `F` is trivial with negligible probability and the hash refuses no tag this concrete holds, so neither branch has a unit test
    * `[✅]`   `setup`, asset scope: condition the identity mapping succeeds; outcome as the entitlement scope with the scope `Bb1DepthOneParameterSetScope::Asset { identity_element: F }`
    * `[✅]`   `derive_identity`, wrong role: condition the parameter set's scope is `Entitlement` and `payload.identity` is `KemIdentity::Asset`, or its scope is `Asset` and the identity is `KemIdentity::Entitlement`; dependency call none; outcome `Err(DeriveIdentityErrorReturn::Bb1DepthOne(Bb1DepthOneDeriveIdentityErrorReturn::WrongIdentityScope))` before hashing. If roles match, the identity mapping over `payload.identity` and the parameter set's `u0`, `u1` may refuse with `HashToScalar(error)` unchanged or `TrivialIdentityElement` (LC-08)
    * `[✅]`   `derive_identity`, outside the asset scope: condition the parameter set's scope is `Asset { identity_element }` and `encode_g1(F)` differs from `encode_g1(identity_element)`; decision equality of the two `P::EncodedG1` values, without discarding their type into byte vectors; outcome `Err(DeriveIdentityErrorReturn::Bb1DepthOne(Bb1DepthOneDeriveIdentityErrorReturn::OutsideAssetScope))`
    * `[✅]`   `derive_identity`, derived: condition the mapping succeeds and, under the asset scope, the encodings are equal; outcome `Ok(DeriveIdentitySuccessReturn { identity_element: Bb1DepthOneIdentityElement { scalar: I, element: F } })`
    * `[✅]`   `issue`: sampling refused, `Err(IssueErrorReturn::Bb1DepthOne(Bb1DepthOneIssueErrorReturn::Sampling(error)))`; sampled, dependency calls `msm_g1` over the terms `(g1, α)` and `(F, r)` for `A` and `mul_g2(g2, r)` for `B`, each once, outcome `Ok(IssueSuccessReturn { credential: Bb1DepthOneCredential { a, b }, randomness })`, `randomness` the sampled `Secret` of `r` moved without copy
    * `[✅]`   `rerandomize`: sampling refused, `Err(RerandomizeErrorReturn::Bb1DepthOne(Bb1DepthOneRerandomizeErrorReturn::Sampling(error)))`; sampled, dependency calls `add_g1(A, mul_g1(F, s))` and `add_g2(B, mul_g2(g2, s))`, outcome `Ok(RerandomizeSuccessReturn { credential, offset })` holding the new credential and the sampled `Secret` of `s` moved without copy; the master scalar is not an input; the given credential is unchanged
    * `[✅]`   `is_valid`: one branch; dependency calls `neg_g1(g1)`, `neg_g1(F)`, then `pairing_product_is_one` over the terms `(A, g2)`, `(-g1, hpub)`, and `(-F, B)`, once; outcome `Ok(IsValidSuccessReturn { is_valid })`, `is_one` unchanged
    * `[✅]`   `encapsulate`: sampling refused, `Err(EncapsulateErrorReturn::Bb1DepthOne(Bb1DepthOneEncapsulateErrorReturn::Sampling(error)))`; sampled, dependency calls `mul_g2(g2, t)` for `U`; under the entitlement scope `mul_g1(u0, t)` for `V` and `mul_g1(u1, t)` for `W`, the capsule `Bb1DepthOneCapsule::Entitlement { u, v, w }`; under the asset scope `mul_g1(F, t)` for `V`, the capsule `Bb1DepthOneCapsule::Asset { u, v }`; then `mul_g1(g1, t)`, `pairing_product` over the one term `(t·g1, hpub)`, and `encode_gt`; copy `Secret<P::EncodedGt>::expose().as_ref()` once into the private `EncapsulatedValue` secret buffer; outcome `Ok(EncapsulateSuccessReturn { capsule, encapsulated })`; the sampled `t`, encoded target-group value, and target-group value drop and zeroize
    * `[✅]`   `is_well_formed`, entitlement: condition the parameter set's scope is `Entitlement` and the capsule is `Entitlement { u, v, w }`; dependency calls `pairing_product_is_one` over `(v, g2)` and `(-u0, u)`, then over `(w, g2)` and `(-u1, u)`, the negations from `neg_g1`; outcome `Ok(IsWellFormedSuccessReturn { is_well_formed })`, the conjunction of both `is_one`
    * `[✅]`   `is_well_formed`, asset: condition the scope is `Asset { identity_element }` and the capsule is `Asset { u, v }`; dependency call `pairing_product_is_one` over `(v, g2)` and `(-identity_element, u)`; outcome `Ok` holding `is_one`
    * `[✅]`   `is_well_formed`, scope mismatch: condition the capsule's variant is not the parameter set's scope; dependency call none; outcome `Ok(IsWellFormedSuccessReturn { is_well_formed: false })`
    * `[✅]`   `decapsulate`, entitlement: condition the capsule is `Entitlement { u, v, w }`; dependency calls `add_g1(v, mul_g1(w, I))`, `neg_g1` of that sum, `pairing_product` over `(A, u)` and `(-(v + I·w), B)`, and `encode_gt`; copy the typed encoded target-group bytes into the private `EncapsulatedValue` as in `encapsulate`; outcome `Ok(DecapsulateSuccessReturn { encapsulated })`
    * `[✅]`   `decapsulate`, asset: condition the capsule is `Asset { u, v }`; dependency calls `neg_g1(v)`, `pairing_product` over `(A, u)` and `(-v, B)`, and `encode_gt`; outcome as above; the identity element is not read, since the asset's `F` is fixed in `v`
    * `[✅]`   Component access, each method one branch with no dependency call and each outcome `Ok`: `credential_components` returns `CredentialComponents { a, b }`, clones of the credential's `a` and `b`; `credential_from_components` returns `Bb1DepthOneCredential { a, b }`, the components moved; `parameter_set_components` returns clones of `g1`, `u0`, `u1`, `g2`, `hpub`, and the scope, `Bb1DepthOneParameterSetScope::Entitlement` as `ParameterSetScopeComponents::Entitlement` and `Asset { identity_element }` as `ParameterSetScopeComponents::Asset { identity_element }` with a clone; `parameter_set_from_components` returns the parameter set holding the components moved, the scope mapped back the same way; `identity_element_components` returns clones of `scalar` and `element`; `capsule_components` returns clones of the capsule's elements in the like-named `CapsuleComponents` variant, and `capsule_from_components` the `Bb1DepthOneCapsule` variant holding the components moved; `master_scalar_components` returns `MasterScalarComponents { value }`, `value` a new `Secret` from `Secret::try_new(SecretConstructorParams { value: self.value.expose().clone() })` unpacked irrefutably, and `master_scalar_from_components` returns `Bb1DepthOneMasterScalar { value }`, the `Secret` moved
    * `[✅]`   Ordering: every sampling precedes the arithmetic it feeds, and setup samples `α`, `a`, then `b`; the identity mapping hashes before it multiplies, and the trivial test precedes the asset-scope comparison; every point and scalar a pairing payload consumes is a clone of a held value, and the held values are unchanged; `params` carries no control and is not read in any method but `setup`
    * `[✅]`   Invariants: the same inputs always yield the same outputs; every credential `issue` or `rerandomize` returns passes `is_valid` for its identity element under its parameter set; every capsule `encapsulate` returns passes `is_well_formed` under its parameter set, and every valid credential decapsulates it to `encapsulate`'s bytes; every value rebuilt from its own components behaves as the value it was read from; `B` is `randomness·g2` for an issued credential and grows by `offset·g2` under rerandomization

  * `[✅]`   `adapters/kem/src/factory/mock.rs`

  * `[✅]`   `adapters/kem/src/factory/mod.rs`
    * `[✅]`   Module wiring only: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, and `pub mod provides;`, nothing else

  * `[✅]`   `adapters/kem/src/factory/provides.rs`
    * `[✅]`   `pub use super::interface::*;` and `#[cfg(any(test, feature = "mocks"))] pub use super::mock::*;`, nothing else

  * `[ ]`   `adapters/kem/src/bb1_depth_one/mock.rs`

  * `[ ]`   `adapters/kem/src/bb1_depth_one/test.rs`
    * `[ ]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`; imports this module's names from `super::interface` and `build_bb1_depth_one_parameter_set` with `Bb1DepthOneParameterSetOverrides` from `super::mock`; `ICredentialKemAdapter`, `SetupScope`, `KemIdentity`, `DeriveIdentityParams`, `DeriveIdentityPayload`, `DeriveIdentityErrorReturn`, `build_setup_params`, `SetupParamsOverrides`, `build_setup_payload`, and `SetupPayloadOverrides` from `crate::factory::provides`; the pairing family's official mock adapter, `G1GeneratorParams`, and `G1GeneratorPayload` from `pairing`; `IHashToScalarAdapter`, `HashToScalarParams`, `HashToScalarPayload`, `HashToScalarReturn`, `HashToScalarDeclaration`, `MockIHashToScalarAdapter`, `build_hash_to_scalar_declaration`, `HashToScalarDeclarationOverrides`, `build_domain_tag`, and `DomainTagConstructorParamsOverrides` from `hash_to_scalar`; `build_asset_identity_hash` and `AssetIdentityHashConstructorParamsOverrides` from `domain`; and `core::marker::PhantomData`
    * `[ ]`   The pairing is the pairing family's official mock adapter, bound as `pairing`; every group element a block needs comes from it or from a builder over it
    * `[ ]`   The subject is `Bb1DepthOneKem { pairing: &pairing, hash_to_scalar: &hasher, tag }`, written from the test module, a child of `bb1_depth_one`, with `tag` from `build_domain_tag` with `bytes: Some(BB1_DEPTH_ONE_IDENTITY_TAG.to_vec())`; `Bb1DepthOneKem::try_new` runs in no block; each block's act is its one call to the method under test, unpacked by `let Ok(success) = … else { panic!(…) };` or `let Err(error) = … else { panic!(…) };`
    * `[ ]`   The hasher is the official `MockIHashToScalarAdapter { scalar: PhantomData }` over the mock pairing's scalar, except where a block names `UncalledHasher`, a test-local unit struct declared in the test module implementing `IHashToScalarAdapter` over the mock pairing's scalar, its `declaration()` returning `build_hash_to_scalar_declaration(Default::default())` and its `hash_to_scalar` panicking, so a block whose subject hashes fails
    * `[ ]`   `setup_fixes_an_entitlement_scope_set_to_no_identity`: contract: `params.scope` is `SetupScope::Entitlement` and the three samplings succeed → `Ok` holding a parameter set whose scope is `Bb1DepthOneParameterSetScope::Entitlement`; arrange the subject, `build_setup_params(Default::default())`, the entitlement scope, and `build_setup_payload(Default::default())`; act `kem.setup(params, payload)`; assert `assert!(matches!(success.parameter_set.scope, Bb1DepthOneParameterSetScope::Entitlement))`, the scope the subject chose from the params
    * `[ ]`   `setup_fixes_an_asset_scope_set_to_its_identity_element`: contract: `params.scope` is `SetupScope::Asset { identity_hash }` and the identity mapping succeeds → `Ok` holding a parameter set whose scope is `Bb1DepthOneParameterSetScope::Asset { identity_element }`; arrange the subject, `build_setup_params` with `scope: Some(SetupScope::Asset { identity_hash: &hash })`, `hash` from `build_asset_identity_hash` with `bytes: Some([0xa1; 32])`, and `build_setup_payload(Default::default())`; act `kem.setup(params, payload)`; assert `assert!(matches!(success.parameter_set.scope, Bb1DepthOneParameterSetScope::Asset { .. }))`, the scope the subject chose from the params
    * `[ ]`   `derive_identity_refuses_an_asset_identity_under_an_entitlement_set_before_hashing`: contract: the parameter set's scope is `Entitlement` and the identity is `KemIdentity::Asset` → `Err(DeriveIdentityErrorReturn::Bb1DepthOne(Bb1DepthOneDeriveIdentityErrorReturn::WrongIdentityScope))` before hashing; arrange the subject with `UncalledHasher`, the parameter set from `build_bb1_depth_one_parameter_set(&pairing, Default::default())`, entitlement-scoped, and `KemIdentity::Asset { identity_hash: &hash }`, `hash` from `build_asset_identity_hash` with `bytes: Some([0xa1; 32])`; act `kem.derive_identity(DeriveIdentityParams, DeriveIdentityPayload { parameter_set: &parameter_set, identity })`; assert `assert_eq!(error, DeriveIdentityErrorReturn::Bb1DepthOne(Bb1DepthOneDeriveIdentityErrorReturn::WrongIdentityScope))`, the block completing without the hasher's panic
    * `[ ]`   `derive_identity_refuses_an_entitlement_identity_under_an_asset_set_before_hashing`: contract: the parameter set's scope is `Asset` and the identity is `KemIdentity::Entitlement` → `Err(DeriveIdentityErrorReturn::Bb1DepthOne(Bb1DepthOneDeriveIdentityErrorReturn::WrongIdentityScope))` before hashing; arrange the subject with `UncalledHasher`, the parameter set from `build_bb1_depth_one_parameter_set` with `scope: Some(Bb1DepthOneParameterSetScope::Asset { identity_element })`, `identity_element` the point the pairing's `g1_generator(G1GeneratorParams, G1GeneratorPayload)` returns, and `KemIdentity::Entitlement { canonical: b"entitlement-one" }`; act `kem.derive_identity(DeriveIdentityParams, DeriveIdentityPayload { parameter_set: &parameter_set, identity })`; assert `assert_eq!(error, DeriveIdentityErrorReturn::Bb1DepthOne(Bb1DepthOneDeriveIdentityErrorReturn::WrongIdentityScope))`, the block completing without the hasher's panic
    * `[ ]`   Every test block carries the full `Contract`, `Arrange`, `Act`, `Assert` header and the inline markers

  * `[✅]`   `construction`
    * `[✅]`   `Bb1DepthOneKem::try_new` is the concrete's only producer, and its only caller is the credential KEM factory, which reads `Bb1DepthOneKem::<'_, P>::DECLARATION` before constructing and constructs it from borrows of the resolved pairing concrete and of the hash-to-scalar adapter for that pairing's scalar type, both held by the factory's caller, which keeps the pairing for every other family that computes in its groups
    * `[✅]`   Parameter sets, master scalars, identity elements, credentials, and capsules are produced only by the adapter's methods, including the component-access methods that rebuild them; fixtures of them are produced by calling the adapter, so the concrete has no `mock.rs`; the adapter itself is injected and is mocked through `MockICredentialKemAdapter`

  * `[✅]`   `adapters/kem/src/bb1_depth_one/mod.rs`
    * `[✅]`   Module declarations: `mod interface;`, `pub(crate) mod provides;`, and `#[cfg(test)] mod test;`
    * `[✅]`   `impl<'a, P: IPairingArithmetic> Bb1DepthOneKem<'a, P>` with `pub const DECLARATION: KemDeclaration = BB1_DEPTH_ONE_DECLARATION;` and `pub fn try_new(params: Bb1DepthOneKemConstructorParams<'a, P>) -> Bb1DepthOneKemTryNewReturn<'a, P>`, each as the interaction spec states
    * `[✅]`   `impl<'a, P: IPairingArithmetic> ICredentialKemAdapter for Bb1DepthOneKem<'a, P>` with `const DECLARATION: KemDeclaration = BB1_DEPTH_ONE_DECLARATION;`, `type Pairing = P;`, `type ParameterSet = Bb1DepthOneParameterSet<P>;`, `type MasterScalar = Bb1DepthOneMasterScalar<P>;`, `type IdentityElement = Bb1DepthOneIdentityElement<P>;`, `type Credential = Bb1DepthOneCredential<P>;`, and `type Capsule = Bb1DepthOneCapsule<P>;`, and every method realizing its branches and ordering in the interaction spec, each pairing call on `self.pairing` and each hash on `self.hash_to_scalar`; its two encapsulated-value branches consume only typed `P::EncodedGt` before narrowing to secret KDF material
    * `[✅]`   `impl EncapsulatedValue` defines `pub fn key_material(&self) -> &Secret<Vec<u8>>`, borrowing the private field without permitting callers to install bytes; this type's only production constructors remain the two KEM branches and the feature-gated mock builder
    * `[✅]`   Imports the family's names from `crate::factory::provides`, `AssetIdentityHash`, `Secret`, and `SecretConstructorParams` from `domain`, the pairing traits, params, payloads, and term types from `pairing`, the hash-to-scalar names from `hash_to_scalar`, and this module's names from `interface`
    * `[✅]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[✅]`   `adapters/kem/src/bb1_depth_one/provides.rs`
    * `[✅]`   `pub(crate) use super::interface::*;`, nothing else, so the concrete is visible to the crate's factory and to nothing outside the crate

  * `[✅]`   `directionality`
    * `[✅]`   `bb1_depth_one` depends on the `factory` module's surface through `crate::factory::provides`, on `domain`, on `pairing`, and on `hash-to-scalar`; the `factory` module depends on `domain`, on `zeroize`, on `pairing`'s `IPairingAdapter`, and on `bb1_depth_one`'s errors through `crate::bb1_depth_one::provides`, the family form's recorded cycle, which `kem/factory` completes by constructing the concrete
    * `[✅]`   Among repository crates the crate depends on `crates/domain`, `adapters/pairing`, and `adapters/hash-to-scalar`, none of which names it; nothing depends on the crate yet; no other cycle
    * `[✅]`   `kem/factory` constructs the concrete and admits it by identity scope; `workflows/sidecar/wrap` derives from `EncapsulatedValue`; `workflows/sidecar/build` and `workflows/sidecar/validate` read and rebuild capsules through their components; `envelope/pairing_elgamal` encrypts `CredentialComponents` and the credential engine rebuilds a credential from them; `proof/schnorr_fs` takes the master scalar's components and the issuance `randomness` as the mint proof's witnesses, the rerandomization `offset` as the transfer proof's, and the parameter-set and identity-element components into its statement; the registry encodes the parameter-set components, and custody persists and restores the master scalar's; `harness-crypto/main` obtains the KEM through `kem/factory` and hands the adapter and its declaration to the generate concrete, whose `harness-crypto/generate/evm/constants` reads the identity mapping's tag from that declaration and whose `harness-crypto/generate/evm/mapping_vectors` reads the mapped scalar from the identity element's components

  * `[ ]`   `requirements`
    * `[✅]`   `adapters/kem/Cargo.toml` carries exactly the tables and keys stated above, and no curve, hash, or randomness library is named in the crate
    * `[✅]`   `cargo check --workspace --all-targets --all-features`, `cargo fmt --all --check`, and `cargo deny check` complete without error; `cargo clippy --workspace --all-targets --all-features` reports nothing beyond the library target's unused-item warnings for the `bb1_depth_one` concrete, which `kem/factory` resolves by constructing the concrete
    * `[ ]`   `setup_fixes_an_entitlement_scope_set_to_no_identity` and `setup_fixes_an_asset_scope_set_to_its_identity_element` pass (the parameter set's scope fixed from the setup scope)
    * `[ ]`   `derive_identity_refuses_an_asset_identity_under_an_entitlement_set_before_hashing` and `derive_identity_refuses_an_entitlement_identity_under_an_asset_set_before_hashing` pass (CD-08 typed identity admission)
    * `[✅]`   Every family and concrete error enum the KEM declares derives `Debug`, `PartialEq`, and `Eq`, and every refusal the concrete's tests assert is asserted by `assert_eq!` against the whole expected error
    * `[✅]`   Code outside `adapters/kem` naming `Bb1DepthOneKem` or anything under `bb1_depth_one` fails to compile; the crate's public surface is the `factory` module's `provides`

* `[ ]`   `kem/factory` **Credential KEM factory constructing the concrete the configuration names over a borrowed pairing and hash-to-scalar adapter, admitted against the KEM identifier and the identity scope the suite requires, and handing it to a consumer generic over the family's trait for that pairing; carries the family's integration test**

  * `[✅]`   `objective`
    * `[✅]`   Problem: a consumer obtains a credential KEM only through the family's generic surface, never by naming a concrete, over the pairing it already resolved, and the KEM it uses is the one the deployment's suite names under the identity scope the suite fixes, so a concrete that does not declare the required identifier or scope is refused before anything is constructed (CR-08; CD-08; Composition Boundary)
    * `[✅]`   Functional: given the concrete the configuration names, the KEM identifier required, and the identity scope required, the factory refuses a concrete whose declared identifier is not the required one, and a concrete whose declared scopes do not include the required scope, each with no construction and no call to the consumer
    * `[✅]`   Functional: an admitted concrete is constructed from the caller's borrowed pairing and hash-to-scalar adapter and handed with its admitted scope to a consumer generic over `ICredentialKemAdapter` whose `Pairing` is the caller's pairing type; the consumer reads `K::DECLARATION`, and the consumer's output is returned; the consumer never names the concrete
    * `[✅]`   Functional: a concrete's constructor refusal is returned unchanged in the factory's error arm, one variant per concrete
    * `[✅]`   Functional: the KEM the factory constructs, with every uniform draw taken from the randomness family's operating-system source, issues, rerandomizes, encapsulates, and decapsulates to one encapsulated value under either scope, with nothing mocked
    * `[✅]`   Non-functional: adding a concrete is its module, its variant in the selection enum and in the error enum, and its branch here; adding an identifier is its variant in `KemIdentifier`; no consumer changes

  * `[✅]`   `role`
    * `[✅]`   Adapter family factory: the implementation of the `factory` module, the credential KEM family's construction point, and the crate's public surface
    * `[✅]`   Hands the concrete to a consumer rather than returning it, because `ICredentialKemAdapter` carries associated types that differ per concrete and so cannot be returned as one type across concretes, as the encoding factory hands its concrete; the consumer trait is generic over the pairing, so the consumer combines the KEM's values with the pairing it holds
    * `[✅]`   Borrows the pairing and the hash-to-scalar adapter from its deps, so the caller keeps both for the envelope and the delivery proof
    * `[✅]`   Selects by concrete and admits by identifier and scope (CD-08's declared capability), so a further concrete implementing an existing identifier or a subset of the scopes arrives as a variant and a branch, with no change to the params or to any consumer
    * `[✅]`   Does not read a hash-card or the configuration; the composition resolver passes the concrete the configuration names as a typed `KemConcrete`, the identifier the hash-card names as a typed `KemIdentifier`, and the scope the suite fixes as a typed `IdentityScope`
    * `[✅]`   Does not construct the pairing or the hash-to-scalar adapter; the caller resolves both through their own factories
    * `[✅]`   Carries the family's integration test across the pairing, hash-to-scalar, and randomness factories, this factory, and the concrete; does not carry a commit, which the credential KEM milestone's chain carries on its last node

  * `[✅]`   `module`
    * `[✅]`   Bounded context: the `factory` module of `adapters/kem`, holding the factory function, its deps, params, payload, and return types, its signature type, the consumer trait with its params and payload types, the selection enum, the function mock and builders, and the crate's integration test under `adapters/kem/tests`
    * `[✅]`   Edits `adapters/kem/Cargo.toml`, `adapters/kem/src/factory/interface.rs`, `interaction.spec.md`, `mock.rs`, `test.rs`, `mod.rs`, and `provides.rs`, and creates `adapters/kem/tests/integration_test.rs`; nothing under `bb1_depth_one` changes
    * `[✅]`   Outside: the concrete's behavior, the pairing and hash-to-scalar factories, the source of uniform bytes, the hash-card, the configuration catalogue, and every consumer of the family

  * `[ ]`   `deps`
    * `[✅]`   The `bb1_depth_one` concrete, through `crate::bb1_depth_one::provides`: `Bb1DepthOneKem`, `Bb1DepthOneKemConstructorParams`, `Bb1DepthOneKem::try_new`, `Bb1DepthOneKem::<'_, P>::DECLARATION`, and `Bb1DepthOneKemTryNewErrorReturn`; the factory constructs its concrete, completing the family form's recorded cycle that `kem/bb1_depth_one` opened
    * `[✅]`   The `factory` module's own interface: `ICredentialKemAdapter`, `KemIdentifier`, `IdentityScope`, `KemDeclaration`, and `KEM_INTERFACE_VERSION`, as `kem/bb1_depth_one` authors them
    * `[✅]`   `pairing`, the existing runtime dependency: `IPairingAdapter` and `IPairingArithmetic`, the consumer trait's and the deps' bounds
    * `[✅]`   `hash-to-scalar`, the existing runtime dependency: `IHashToScalarAdapter`, the deps' borrowed adapter type
    * `[✅]`   `random`, `adapters/random`, adapter ring, dev-dependency with its `mocks` feature, in the integration test only: `create_random_source`, `CreateRandomSourceDeps`, `CreateRandomSourcePayload`, `build_create_random_source_params`, `CreateRandomSourceParamsOverrides`, `RandomSourceKind`, `IRandomSourceAdapter`, `FillBytesParams`, `build_fill_bytes_payload`, and `FillBytesPayloadOverrides`, the First Finder's source of uniform bytes; nothing at runtime, and the random crate names nothing in this crate
    * `[ ]`   `pairing` and `hash-to-scalar` with their `mocks` features, the existing dev-dependencies: in `factory/test.rs`, the pairing family's official mock adapter, the struct the `pairing` mock surface provides implementing `IPairingArithmetic`, located by that implementation, and `MockIHashToScalarAdapter`; in the integration test, `create_pairing` with its consumer names and builders, `create_hash_to_scalar` with its builders, `DomainTag` with `DomainTagConstructorParams`, and the pairing arithmetic, encoding, and sampling params, payloads, and terms the scenarios compute expected values with
    * `[ ]`   `domain` with its `mocks` feature, the existing dev-dependency, in the integration test: `AssetIdentityHash`, `build_asset_identity_hash` with `AssetIdentityHashConstructorParamsOverrides`, and `build_secret` with `SecretConstructorParamsOverrides`
    * `[✅]`   `<[IdentityScope]>::contains`, standard library, the scope check

  * `[ ]`   `context_slice`
    * `[ ]`   In `factory/test.rs`: the pairing family's official mock adapter, implementing `IPairingAdapter` and `IPairingArithmetic` and returning built defaults; `MockIHashToScalarAdapter<S> { scalar: PhantomData<S> }`; and `MockIKemConsumer`, implementing `IKemConsumer<P>` with `type Output = ();`
    * `[ ]`   In the integration test: every `ICredentialKemAdapter` method and component type `kem/bb1_depth_one` declares, called on the adapter `create_kem` hands the consumer; and the pairing's `g1_generator`, `g2_generator`, `add_g1`, `add_g2`, `mul_g1`, `mul_g2`, `msm_g1`, `neg_scalar`, `pairing_product`, `encode_g1`, `encode_g2`, `encode_gt`, and `encode_scalar`, and its scalar's `sample_from_uniform_bytes`, with which the scenarios compute expected values
    * `[✅]`   From the concrete: `Bb1DepthOneKem::try_new(Bb1DepthOneKemConstructorParams { pairing: &'a P, hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar> }) -> Result<Bb1DepthOneKem<'a, P>, Bb1DepthOneKemTryNewErrorReturn>`, the inherent constant `Bb1DepthOneKem::<'_, P>::DECLARATION: KemDeclaration` with `identity_scopes` `&[IdentityScope::Entitlement, IdentityScope::Asset]` and `identity_tag` `b"ChainTorrent-v1-kem-identity"`, and `Bb1DepthOneKem`'s implementation of `ICredentialKemAdapter` with `type Pairing = P`
    * `[✅]`   From `pairing`, in the tests: `create_pairing<C: IPairingConsumer>(&CreatePairingDeps<C>, CreatePairingParams, CreatePairingPayload)`, `build_create_pairing_params` with `CreatePairingParamsOverrides { concrete, .. }`, `PairingConcrete`, `IPairingConsumer::consume_pairing<P: IPairingArithmetic>`, `ConsumePairingParams`, and `ConsumePairingPayload { adapter }`; `ISampleUniformScalar::UNIFORM_BYTES_LENGTH`
    * `[✅]`   From `hash-to-scalar`, in the tests: `create_hash_to_scalar::<S>(&CreateHashToScalarDeps, CreateHashToScalarParams, CreateHashToScalarPayload)` returning `{ adapter: Box<dyn IHashToScalarAdapter<S>> }`, whose declaration comes from `adapter.declaration()`, and `build_create_hash_to_scalar_params(Default::default())`, defaulting to the Keccak-256 concrete and identifier
    * `[✅]`   From `random`, in the integration test: `create_random_source(&CreateRandomSourceDeps, CreateRandomSourceParams { kind }, CreateRandomSourcePayload)` returning `{ adapter: Box<dyn IRandomSourceAdapter> }`, whose declaration comes from `adapter.declaration()`, and `fill_bytes(&self, FillBytesParams, FillBytesPayload { length })` returning `Result<FillBytesSuccessReturn { bytes: Secret<Vec<u8>> }, FillBytesErrorReturn>`

  * `[✅]`   `adapters/kem/Cargo.toml`
    * `[✅]`   `[dev-dependencies]` reads `domain = { path = "../../crates/domain", features = ["mocks"] }`, `pairing = { path = "../pairing", features = ["mocks"] }`, `hash-to-scalar = { path = "../hash-to-scalar", features = ["mocks"] }`, and `random = { path = "../random", features = ["mocks"] }`
    * `[✅]`   `[package]`, `[dependencies]`, `[features]`, and `[lints]` are unchanged; no other table

  * `[✅]`   `adapters/kem/src/factory/interface.rs`
    * `[✅]`   `KemIdentifier` gains `#[derive(PartialEq, Eq)]`, so the admission compares a declared identifier with the required one
    * `[✅]`   `KemConcrete`, an enum with the one variant `Bb1DepthOne`, the selection of the concrete to construct
    * `[✅]`   `IKemConsumer<P: IPairingAdapter>`, a trait with `type Output;` and `fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(&self, params: ConsumeKemParams, payload: ConsumeKemPayload<K>) -> Self::Output;`, the work a composition performs with whichever concrete the factory constructs over the pairing it holds
    * `[✅]`   `ConsumeKemParams`, the fieldless struct `pub struct ConsumeKemParams;`
    * `[✅]`   `ConsumeKemPayload<K: ICredentialKemAdapter>`, a struct with `pub adapter: K` and `pub(super) scope: IdentityScope` (private outside `factory`); `pub fn scope(&self) -> IdentityScope` returns the admitted scope. The consumer reads `K::DECLARATION`; only the production factory can construct this payload; the test-only mock builder can construct fixtures, so callers cannot substitute a different scope or declaration
    * `[✅]`   `CreateKemDeps<'a, P: IPairingArithmetic, C>`, a struct with `pub pairing: &'a P`, `pub hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>`, and `pub consumer: C`
    * `[✅]`   `CreateKemParams`, a struct with `pub concrete: KemConcrete`, `pub identifier: KemIdentifier`, and `pub scope: IdentityScope`, the selection, the identifier the concrete must declare, and the scope it must declare
    * `[✅]`   `CreateKemPayload`, the fieldless struct `pub struct CreateKemPayload;`, since the factory operates on no data
    * `[✅]`   `CreateKemSuccessReturn<O>`, a struct with `pub output: O`
    * `[✅]`   `CreateKemErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]` and the variants `UnsupportedKemIdentifier`, the named concrete not declaring the required identifier; `UnsupportedIdentityScope`, the named concrete not declaring the required scope; and `Bb1DepthOne(Bb1DepthOneKemTryNewErrorReturn)`, the concrete's constructor refusal carried unchanged; each further concrete's constructor refusal is its own variant
    * `[✅]`   `CreateKemReturn<O>`, the alias `Result<CreateKemSuccessReturn<O>, CreateKemErrorReturn>`
    * `[✅]`   `CreateKemFn<'a, P, C>`, the alias `fn(&CreateKemDeps<'a, P, C>, CreateKemParams, CreateKemPayload) -> CreateKemReturn<<C as IKemConsumer<P>>::Output>`
    * `[✅]`   The imports gain `IPairingArithmetic` from `pairing`, `IHashToScalarAdapter` from `hash_to_scalar`, and `Bb1DepthOneKemTryNewErrorReturn` from `crate::bb1_depth_one::provides`; every item `kem/bb1_depth_one` authored in this file is unchanged except the derive on `KemIdentifier`

  * `[✅]`   `adapters/kem/src/factory/interaction.spec.md`
    * `[✅]`   `create_kem<'a, P: IPairingArithmetic, C: IKemConsumer<P>>(deps: &CreateKemDeps<'a, P, C>, params: CreateKemParams, payload: CreateKemPayload) -> CreateKemReturn<C::Output>`: decision a `match` on `params.concrete`, one arm per `KemConcrete` variant, exhaustive so a variant with no arm fails to compile
    * `[✅]`   Unsupported identifier: condition `Bb1DepthOneKem::<'_, P>::DECLARATION.identifier` is not `params.identifier`; decision equality, read before any construction; dependency call none; outcome `Err(CreateKemErrorReturn::UnsupportedKemIdentifier)`, with nothing constructed and the consumer not called; `KemIdentifier` has the one variant the concrete declares, so no input takes this branch until a further identifier exists, and it has no unit test
    * `[✅]`   Unsupported scope: condition the identifier matches and `DECLARATION.identity_scopes` does not contain `params.scope`; decision `contains`, read before any construction; dependency call none; outcome `Err(CreateKemErrorReturn::UnsupportedIdentityScope)`, with nothing constructed and the consumer not called; the concrete declares both scopes, so no input takes this branch until a concrete declaring fewer exists, and it has no unit test
    * `[✅]`   Constructor refused: condition both checks pass and `Bb1DepthOneKem::try_new(Bb1DepthOneKemConstructorParams { pairing: deps.pairing, hash_to_scalar: deps.hash_to_scalar })` returns `Err(error)`; dependency call `try_new` once; outcome `Err(CreateKemErrorReturn::Bb1DepthOne(error))`, the refusal unchanged, the consumer not called; the concrete's tag is admitted, so no input takes this branch and it has no unit test
    * `[✅]`   Admitted: condition `try_new` returns `Ok(adapter)`; dependency call `deps.consumer.consume_kem(ConsumeKemParams, ConsumeKemPayload { adapter, scope: params.scope })`, exactly once; outcome `Ok(CreateKemSuccessReturn { output })` holding the consumer's output
    * `[✅]`   Ordering: the identifier check, then the scope check, then construction, then the consumer; `params.concrete` selects, `params.identifier` and `params.scope` admit; `payload` carries nothing and is not read; the deps' borrows are copied into the constructor params and the deps are unchanged

  * `[✅]`   `adapters/kem/src/factory/mock.rs`

  * `[ ]`   `adapters/kem/src/factory/test.rs`

  * `[✅]`   `construction`
    * `[✅]`   A composition working with a resolved pairing `P`, inside its pairing consumer, obtains the hash-to-scalar adapter for `P::Scalar` from `create_hash_to_scalar`, writes its KEM-dependent work once as an `IKemConsumer<P>` generic over `K: ICredentialKemAdapter<Pairing = P>`, and calls `create_kem` with `CreateKemDeps` borrowing the pairing and the adapter and holding the consumer, `CreateKemParams` holding the concrete the configuration names, the identifier the hash-card names, and the scope the suite fixes, and `CreateKemPayload`; no consumer constructs or names a concrete, and the composition keeps the pairing for the envelope and the delivery proof

  * `[✅]`   `adapters/kem/src/factory/mod.rs`
    * `[✅]`   Adds `#[cfg(test)] mod test;` to the wiring `kem/bb1_depth_one` authored
    * `[✅]`   `pub fn create_kem<'a, P: IPairingArithmetic, C: IKemConsumer<P>>(deps: &CreateKemDeps<'a, P, C>, params: CreateKemParams, _payload: CreateKemPayload) -> CreateKemReturn<C::Output>`, a `match` on `params.concrete` whose `KemConcrete::Bb1DepthOne` arm realizes the branches and ordering of the interaction spec, constructing by `match Bb1DepthOneKem::try_new(…)` so the refusal is wrapped in `CreateKemErrorReturn::Bb1DepthOne` and the admitted adapter moved into the consumer's payload
    * `[✅]`   Imports `Bb1DepthOneKem` and `Bb1DepthOneKemConstructorParams` from `crate::bb1_depth_one::provides`, `IPairingArithmetic` from `pairing`, and this module's types from `interface`
    * `[✅]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[✅]`   `adapters/kem/src/factory/provides.rs`
    * `[✅]`   Adds `pub use super::create_kem;` to the re-exports `kem/bb1_depth_one` authored

  * `[ ]`   `adapters/kem/tests/integration_test.rs`

  * `[✅]`   `directionality`
    * `[✅]`   The `factory` module depends on the `bb1_depth_one` concrete through `crate::bb1_depth_one::provides`, on its own interface, on `pairing`, and on `hash-to-scalar`; `bb1_depth_one` depends on the `factory` module's surface, the family form's recorded cycle; the crate's public surface is the `factory` module's `provides`
    * `[✅]`   Among repository crates the crate depends on `crates/domain`, `adapters/pairing`, and `adapters/hash-to-scalar` at runtime and on `adapters/random` for its integration test only; none of them names this crate; no other cycle
    * `[✅]`   `workflows/sidecar/wrap` consumes the family's surface; `harness-crypto/main` obtains the KEM through `create_kem` and an `IKemConsumer` and hands the adapter and admitted scope; consumers obtain its declaration through `K::DECLARATION` to the generate concrete and to `harness-crypto/vectors`, neither of which calls the factory, `harness-crypto/generate/evm/constants` reading the identity mapping's tag from that declaration

  * `[ ]`   `requirements`
    * `[✅]`   `CreateKemErrorReturn` derives `Debug`, `PartialEq`, and `Eq`
    * `[✅]`   `adapters/kem/Cargo.toml` carries exactly the dev-dependencies stated above, and every other table `kem/bb1_depth_one` stated is unchanged
    * `[ ]`   `create_kem_admits_the_bb1_depth_one_concrete_under_the_entitlement_scope` and `create_kem_admits_the_bb1_depth_one_concrete_under_the_asset_scope` pass (CD-08, the concrete admitted by its declared scope)
    * `[ ]`   `an_issued_credential_is_valid_for_its_identity`, `a_rerandomized_credential_is_valid_without_the_master_scalar`, `a_credential_is_invalid_for_another_entitlement`, and `a_credential_is_invalid_under_another_parameter_set` pass (CR-08 issuance, rerandomization without the master scalar, validity, and non-convertibility)
    * `[ ]`   `every_credential_for_an_identity_decapsulates_the_encapsulated_value`, `another_entitlements_credential_decapsulates_the_same_capsule_to_the_same_value`, and the three `every_credential_decapsulates_the_encapsulated_value_on_…` tests pass (CR-08 cross-holder agreement on every pairing concrete)
    * `[ ]`   `a_capsule_is_well_formed_under_its_parameter_set`, `a_capsule_with_its_identity_bases_exchanged_is_not_well_formed`, and `a_capsule_is_not_well_formed_under_another_parameter_set` pass (CR-08 malformed-capsule rejection; EC-06)
    * `[ ]`   `an_asset_scope_capsule_has_two_elements`, `an_asset_scope_capsule_is_well_formed_under_its_parameter_set`, `a_holder_authors_a_valid_asset_scope_credential_without_the_master_scalar`, `every_asset_scope_credential_decapsulates_the_encapsulated_value`, `derive_identity_refuses_an_identity_outside_the_asset_scope`, and `an_asset_scope_parameter_set_carries_the_derived_identity_element` pass (CD-08 holder-authored grants under the asset scope)
    * `[ ]`   `derive_identity_refuses_a_trivial_identity_element` passes (CR-08 and LC-08 trivial identity-element refusal)
    * `[ ]`   `the_parameter_set_components_bind_hpub_to_the_master_scalar`, `the_identity_element_components_are_the_mapped_scalar_and_its_element`, `the_identity_mapping_hashes_under_the_declared_tag`, `issuance_returns_the_randomness_its_credential_was_formed_with`, `rerandomization_returns_the_offset_it_applied`, `a_credential_rebuilt_from_its_components_is_valid`, `a_parameter_set_rebuilt_from_its_components_validates_its_credentials`, `a_capsule_rebuilt_from_its_components_is_well_formed`, `a_capsule_rebuilt_from_its_components_decapsulates_the_encapsulated_value`, and `a_master_scalar_restored_from_its_components_issues_valid_credentials` pass (the components and witnesses the envelope, delivery proof, sidecar layer, registry, and custody consume; CR-09's mint and transfer witnesses)
    * `[ ]`   The five `…_refuses_a_…draw_of_the_wrong_length` tests and the four `encapsulation_exposes_the_target_group_encoding_on_…` tests pass
    * `[ ]`   `a_kem_from_the_factory_round_trips_an_entitlement_scope_sale_from_operating_system_draws_on_bls12_381_arkworks` and `a_kem_from_the_factory_round_trips_an_asset_scope_grant_from_operating_system_draws_on_bn254_halo2curves` pass (CR-08 through the family's surface under each scope on each curve; CR-05, production draws from the operating system's generator)
    * `[✅]`   `cargo check --workspace --all-targets --all-features`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo fmt --all --check`, and `cargo deny check` complete without error or warning in every target, the `bb1_depth_one` concrete's unused-item warnings having no remaining cause
    * `[✅]`   No code outside `adapters/kem` can name `Bb1DepthOneKem`

* `[ ]`   `workflows/sidecar/wrap` **Wrap of a piece-group key under a parameter set's wrapping key, the key-derivation family's wrapping-key derivation of the encapsulated value over the encoded derivation context, XORed with the key; creates the `workflows` crate**

  * `[✅]`   `objective`
    * `[✅]`   Problem: the encryptor chooses each piece-group key independently of any parameter set, and each live set's sidecar entry for a group carries that key wrapped under a wrapping key only that set's credentials recover, so independently generated sets open one ciphertext; the wrapping key is a domain-separated derivation of the set's encapsulated value `K` and the context, and the wrap is their XOR with no tag, since the sidecar's Bao root authenticates every entry (CR-11; `docs/research/cryptography.md`'s Credential KEM statement; `docs/research/cryptography-critical-path.md`'s Payload line)
    * `[✅]`   Functional: the wrap encodes the derivation context, asset, deployment, suite, parameter set, group index, and geometry, through the encoding family's canonical encoding
    * `[✅]`   Functional: the wrap derives the wrapping key through the key-derivation family under the `WrappingKey` purpose, at the piece-group key's length, from the encapsulated value's bytes as key material and the encoded context
    * `[✅]`   Functional: the wrapped key is the byte-wise XOR of the piece-group key and the wrapping key, of the piece-group key's length
    * `[✅]`   Functional: a derived key whose length is not the requested length is refused before any byte is combined, and a key-derivation refusal is returned unchanged
    * `[✅]`   Functional: wrapping a zero key under the reference key material and the reference context yields the key-derivation family's independent wrapping-key vector, and wrapping an all-ones key yields its complement
    * `[✅]`   Non-functional: the wrapping key is held in a `Secret` and zeroized when the wrap returns; the crate depends on the domain crate and on the encoding, key-derivation, and credential KEM families' surfaces alone, and names no vendor

  * `[✅]`   `role`
    * `[✅]`   Workflow: the application ring's first module, and the first source file of the `workflows` crate, which it creates; the wrap composes the encoding and key-derivation families over the KEM's output, so it belongs to no one adapter family
    * `[✅]`   Owns `PieceGroupKey`, the admitted 32-byte secret consumed by wrap and returned by unwrap, and `WrappedPieceGroupKey`, the 32-byte sidecar value it originates; `workflows/sidecar/unwrap` consumes both contracts
    * `[✅]`   Does not unwrap; `workflows/sidecar/unwrap` derives the same wrapping key and recovers the piece-group key
    * `[✅]`   Does not encapsulate, choose a piece-group key, or build a sidecar entry; `workflows/sidecar/build` encapsulates through the KEM family, draws or derives each piece-group key, and assembles each set's entries
    * `[✅]`   Does not carry a commit; the credential KEM milestone's commit sits on `workflows/sidecar/unwrap`

  * `[✅]`   `module`
    * `[✅]`   Bounded context: the `wrap` function module of the `workflows` crate's `sidecar` area, holding `wrap_piece_group_key`, its deps, params, payload, success, error, and return types, its signature type, `PieceGroupKey`, `WrappedPieceGroupKey`, and their mocks
    * `[✅]`   Creates the crate at `crates/workflows`, admitted by the workspace's `crates/*` glob with no edit to the root manifest, and the `sidecar` area directory, whose `mod.rs` declares its function modules and nothing else
    * `[✅]`   Outside: the derivation, the encoding, the encapsulated value's production, the piece-group key's source, the unwrap, and the sidecar's layout

  * `[ ]`   `deps`
    * `[✅]`   `domain`, `crates/domain`, protocol and domain ring, path dependency; supplies `DerivationContext`, the context the wrapping key is bound to, and `Secret`; `domain` with its `mocks` feature as a dev-dependency supplies `build_secret`, `SecretConstructorParamsOverrides`, and the derivation context's component builders and overrides; direction inward
    * `[ ]`   `encoding`, `adapters/encoding`, adapter ring, path dependency; supplies `IEncoderAdapter` with `encode`, `EncodeParams`, `DerivationContextDescription`, and `DerivationContextDescriptionConstructorParams`; direction inward, workflow ring on adapter surfaces; with its `mocks` feature as a dev-dependency, `build_encode_success_return` and `EncodeSuccessReturnOverrides` in the unit tests and `create_encoding` with its consumer names and builders in the integration test
    * `[✅]`   `kdf`, `adapters/kdf`, adapter ring, path dependency; supplies `IKeyDerivationAdapter` with `derive_key`, `DeriveKeyParams`, `DeriveKeyPayload`, `DerivationPurpose`, and `DeriveKeyErrorReturn`; with its `mocks` feature as a dev-dependency, `MockIKeyDerivationAdapter`, `build_derive_key_success_return`, and `DeriveKeySuccessReturnOverrides` in the unit tests and `create_key_derivation` with its builders in the integration test
    * `[✅]`   `kem`, `adapters/kem`, adapter ring, path dependency; supplies `EncapsulatedValue`; with its `mocks` feature as a dev-dependency, `build_encapsulated_value` and `EncapsulatedValueOverrides`
    * `[✅]`   `hex` `0.4.3`, external crate, MIT OR Apache-2.0, dev-dependency only; supplies `hex::decode` for the vectors
    * `[✅]`   `TryFrom<Vec<u8>> for [u8; 32]`, standard library, for admission of produced key material and sidecar bytes; `Iterator::zip`, standard library, the XOR; `core::cell::RefCell`, standard library, in `test.rs` only
    * `[✅]`   No adapter family names this crate; no reverse dependency

  * `[✅]`   `context_slice`
    * `[✅]`   From `encoding`: `DerivationContextDescription::try_new(DerivationContextDescriptionConstructorParams) -> Result<DerivationContextDescription, Infallible>`, and `IEncoderAdapter::encode<D: IEncodingContract>(&self, EncodeParams { description: &D }, &D::Described) -> Result<EncodeSuccessReturn { bytes: Vec<u8> }, EncodeErrorReturn>`, with `D` the description and `D::Described` `DerivationContext`
    * `[✅]`   From `kdf`: `IKeyDerivationAdapter::derive_key(&self, DeriveKeyParams { purpose, length }, DeriveKeyPayload { key_material: &Secret<Vec<u8>>, context: &[u8] }) -> Result<DeriveKeySuccessReturn { key: Secret<Vec<u8>> }, DeriveKeyErrorReturn>`, `DerivationPurpose::WrappingKey`, and the trait's dyn-compatibility
    * `[✅]`   From `kem`: `EncapsulatedValue::key_material(&self) -> &Secret<Vec<u8>>`; callers cannot construct an encapsulated value from arbitrary secret bytes
    * `[✅]`   From `domain`: `Secret::expose(&self) -> &T`

  * `[✅]`   `crates/workflows/Cargo.toml`
    * `[✅]`   `[package]` with `name = "workflows"`, `edition.workspace = true`, `rust-version.workspace = true`, and `publish.workspace = true`; no `version` key
    * `[✅]`   `[dependencies]` with `domain = { path = "../domain" }`, `encoding = { path = "../../adapters/encoding" }`, `kdf = { path = "../../adapters/kdf" }`, and `kem = { path = "../../adapters/kem" }`
    * `[✅]`   `[dev-dependencies]` with `domain = { path = "../domain", features = ["mocks"] }`, `encoding = { path = "../../adapters/encoding", features = ["mocks"] }`, `kdf = { path = "../../adapters/kdf", features = ["mocks"] }`, `kem = { path = "../../adapters/kem", features = ["mocks"] }`, and `hex = "0.4.3"`
    * `[✅]`   `[features]` with `mocks = []`
    * `[✅]`   `[lints]` with `workspace = true`
    * `[✅]`   No other table

  * `[✅]`   `crates/workflows/src/lib.rs`
    * `[✅]`   The crate barrel: `mod sidecar;` and `pub use sidecar::wrap::provides::*;`, nothing else
    * `[✅]`   Until `sidecar/mod.rs` and `sidecar/wrap/mod.rs` exist, `cargo check` reports the unresolved modules, which is the RED state for every element below that precedes them

  * `[✅]`   `crates/workflows/src/sidecar/mod.rs`
    * `[✅]`   Module wiring only: `pub(crate) mod wrap;`, nothing else; each further sidecar function's node adds its own line

  * `[✅]`   `crates/workflows/src/sidecar/wrap/interface.rs`
    * `[✅]`   `WrapPieceGroupKeyDeps<'a, E: IEncoderAdapter>`, a struct with `pub kdf: &'a dyn IKeyDerivationAdapter` and `pub encoder: &'a E`, generic over the encoder because `IEncoderAdapter`'s method is generic and the trait is not dyn-compatible
    * `[✅]`   `WrapPieceGroupKeyParams`, the fieldless struct `pub struct WrapPieceGroupKeyParams;`
    * `[✅]`   `PIECE_GROUP_KEY_LENGTH: usize = 32`; `PieceGroupKey`, a struct with private `Secret<[u8; 32]>`; `PieceGroupKey::try_from_secret_bytes(Secret<Vec<u8>>) -> Result<PieceGroupKey, PieceGroupKeyErrorReturn>` checks the length once, copies into the fixed array, and drops the original secret; `PieceGroupKey::from_array(Secret<[u8; 32]>) -> PieceGroupKey` is crate-private for unwrap's fixed-size XOR result; `PieceGroupKey::expose(&self) -> &[u8; 32]` borrows the key for encryption or XOR; `PieceGroupKeyErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]` and the one variant `WrongLength { expected: usize, actual: usize }`, is the admission refusal
    * `[✅]`   `WrapPieceGroupKeyPayload<'a>`, a struct with `pub encapsulated: &'a EncapsulatedValue`, the set's `K`, `pub context: &'a DerivationContext`, the group's context, and `pub piece_group_key: &'a PieceGroupKey`
    * `[✅]`   `WrappedPieceGroupKey`, a struct with `#[derive(Clone, Debug, PartialEq, Eq)]` and private `[u8; 32]`; `WrappedPieceGroupKey::try_from_bytes(Vec<u8>) -> Result<WrappedPieceGroupKey, WrappedPieceGroupKeyErrorReturn>` is the sidecar decoding boundary, `WrappedPieceGroupKeyErrorReturn` an enum with `#[derive(Debug, PartialEq, Eq)]` and the one variant `WrongLength { expected: usize, actual: usize }`; `WrappedPieceGroupKey::from_array([u8; 32]) -> WrappedPieceGroupKey` is crate-private for wrap's fixed-size XOR result; `WrappedPieceGroupKey::as_bytes(&self) -> &[u8; 32]` is its immutable serialization view
    * `[✅]`   `WrapPieceGroupKeySuccessReturn`, a struct with `pub wrapped: WrappedPieceGroupKey`
    * `[✅]`   `WrapPieceGroupKeyErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]` and the variants `Encoding(EncodeErrorReturn)`, the description-schema refusal carried unchanged, `KeyDerivation(DeriveKeyErrorReturn)`, the key-derivation refusal carried unchanged, and `WrappingKeyLength { expected: usize, actual: usize }`, a derived key of another length than requested
    * `[✅]`   `WrapPieceGroupKeyReturn`, the alias `Result<WrapPieceGroupKeySuccessReturn, WrapPieceGroupKeyErrorReturn>`
    * `[✅]`   `WrapPieceGroupKeyFn<E>`, the alias `fn(&WrapPieceGroupKeyDeps<'_, E>, WrapPieceGroupKeyParams, WrapPieceGroupKeyPayload<'_>) -> WrapPieceGroupKeyReturn`
    * `[✅]`   No derives beyond those stated; imports `DerivationContext` and `Secret` from `domain`, `IEncoderAdapter` and `EncodeErrorReturn` from `encoding`, `IKeyDerivationAdapter` and `DeriveKeyErrorReturn` from `kdf`, and `EncapsulatedValue` from `kem`

  * `[✅]`   `crates/workflows/src/sidecar/wrap/interaction.spec.md`
    * `[✅]`   `wrap_piece_group_key<E: IEncoderAdapter>(deps: &WrapPieceGroupKeyDeps<'_, E>, params: WrapPieceGroupKeyParams, payload: WrapPieceGroupKeyPayload<'_>) -> WrapPieceGroupKeyReturn`, in order:
    * `[✅]`   Encoding: `DerivationContextDescription::try_new(DerivationContextDescriptionConstructorParams)` unpacked irrefutably, then `deps.encoder.encode(EncodeParams { description: &description }, payload.context)` once; an encoding refusal returns `Err(WrapPieceGroupKeyErrorReturn::Encoding(error))` unchanged before KDF use; success yields the encoded context
    * `[✅]`   Key derivation refused: condition `deps.kdf.derive_key(DeriveKeyParams { purpose: DerivationPurpose::WrappingKey, length: PIECE_GROUP_KEY_LENGTH }, DeriveKeyPayload { key_material: payload.encapsulated.key_material(), context: &encoded })`, called once, returns `Err(error)`; outcome `Err(WrapPieceGroupKeyErrorReturn::KeyDerivation(error))`, the refusal unchanged
    * `[✅]`   Wrapping key of another length: condition the derivation succeeds and the derived key's exposed length is not the piece-group key's length; decision the comparison, before any byte is combined; outcome `Err(WrapPieceGroupKeyErrorReturn::WrappingKeyLength { expected, actual })`, `expected` the piece-group key's length and `actual` the derived key's
    * `[✅]`   Wrapped: condition the derived length is `PIECE_GROUP_KEY_LENGTH`; dependency call none; outcome `Ok(WrapPieceGroupKeySuccessReturn { wrapped })`, `wrapped` holding the 32-byte array filled by XORing the admitted piece-group key with the wrapping key; no variable-length wrapped value enters trusted code
    * `[✅]`   Ordering and lifecycle: encoding precedes derivation, the length check precedes the XOR; the derived wrapping key's `Secret` drops, and so zeroizes, when the function returns on every branch; the piece-group key and the encapsulated value are borrowed and unchanged; `params` carries no control and is not read; the same payload always yields the same outcome

  * `[✅]`   `crates/workflows/src/sidecar/wrap/mock.rs`

  * `[ ]`   `crates/workflows/src/sidecar/wrap/test.rs`

  * `[✅]`   `construction`
    * `[✅]`   `wrap_piece_group_key` is a function; its caller, `workflows/sidecar/build`, holds the key-derivation adapter from `create_key_derivation` and calls the wrap inside its `IEncodingConsumer`, passing the encoder `create_encoding` hands it

  * `[✅]`   `crates/workflows/src/sidecar/wrap/mod.rs`
    * `[✅]`   Module declarations: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, `pub mod provides;`, and `#[cfg(test)] mod test;`
    * `[✅]`   `pub fn wrap_piece_group_key<E: IEncoderAdapter>(deps: &WrapPieceGroupKeyDeps<'_, E>, _params: WrapPieceGroupKeyParams, payload: WrapPieceGroupKeyPayload<'_>) -> WrapPieceGroupKeyReturn`, realizing the branches and ordering of the interaction spec
    * `[✅]`   Imports `DerivationContextDescription`, `DerivationContextDescriptionConstructorParams`, `IEncoderAdapter`, and `EncodeParams` from `encoding`; `DeriveKeyParams`, `DeriveKeyPayload`, and `DerivationPurpose` from `kdf`; and this module's types from `interface`
    * `[✅]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[✅]`   `crates/workflows/src/sidecar/wrap/provides.rs`
    * `[✅]`   `pub use super::wrap_piece_group_key;`, `pub use super::interface::*;`, and `#[cfg(any(test, feature = "mocks"))] pub use super::mock::*;`, nothing else

  * `[ ]`   `crates/workflows/tests/integration_test.rs`

  * `[✅]`   `directionality`
    * `[✅]`   `sidecar/wrap` depends on `domain`, on the encoding family's surface, on the key-derivation family's surface, and on the credential KEM family's `EncapsulatedValue`, each inward from the application ring to the domain ring and to adapter surfaces; no adapter crate names `workflows`; no cycle
    * `[✅]`   `workflows/sidecar/unwrap` consumes `WrappedPieceGroupKey` and derives through the same families; `workflows/sidecar/build` calls the wrap for each group under each live set

  * `[✅]`   `requirements`
    * `[✅]`   `crates/workflows/Cargo.toml` carries exactly the tables and keys stated above, and `crates/workflows/src/lib.rs` and `crates/workflows/src/sidecar/mod.rs` carry exactly the lines stated above
    * `[✅]`   `cargo check --workspace --all-targets --all-features`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo fmt --all --check`, and `cargo deny check` complete without error or warning
    * `[✅]`   `piece_group_key_rejects_a_short_secret`, `wrapped_piece_group_key_rejects_a_short_sidecar_value`, `wrap_xors_the_piece_group_key_with_the_derived_wrapping_key`, `wrap_derives_the_wrapping_key_under_its_purpose_at_the_piece_group_keys_length`, `wrap_derives_from_the_encapsulated_value_and_the_encoded_context`, `wrap_refuses_a_wrapping_key_of_another_length`, and `wrap_returns_an_encoder_contract_refusal` pass, each refusal asserted by `assert_eq!` against the whole expected error
    * `[✅]`   `PieceGroupKeyErrorReturn`, `WrappedPieceGroupKeyErrorReturn`, and `WrapPieceGroupKeyErrorReturn` derive `Debug`, `PartialEq`, and `Eq`
    * `[✅]`   `wrapping_a_zero_key_yields_the_independent_wrapping_key_vector` and `wrapping_an_all_ones_key_yields_the_complement_of_the_wrapping_key_vector` pass (CR-11, the wrapping key derived from the encapsulated value and context and the XOR wrap, frozen against the independent implementation's vector)
    * `[✅]`   No vendor library is named in `crates/workflows`

* `[ ]`   `workflows/sidecar/unwrap` **Unwrap of a wrapped piece-group key under the wrapping key a holder's encapsulated value derives; carries the credential KEM milestone's integration test, two independently generated parameter sets opening one piece-group key, and its commit**

  * `[✅]`   `objective`
    * `[✅]`   Problem: a holder recovers a group's piece-group key from its own set's sidecar entry by deriving that set's wrapping key from the encapsulated value its credential decapsulates and removing it from the wrapped key, and every live set, however independently generated, must recover the same piece-group key, so one ciphertext decrypts under every live set (CR-08 cross-set agreement; CR-11; `docs/research/cryptography.md`'s Credential KEM statement)
    * `[✅]`   Functional: the unwrap encodes the derivation context through the encoding family and derives the wrapping key through the key-derivation family under the `WrappingKey` purpose, at the wrapped key's length, from the encapsulated value's bytes, exactly as `workflows/sidecar/wrap` derives it
    * `[✅]`   Functional: the piece-group key is the byte-wise XOR of the wrapped key and the wrapping key, returned inside a `Secret`
    * `[✅]`   Functional: a derived key whose length is not the wrapped key's length is refused before any byte is combined, and a key-derivation refusal is returned unchanged
    * `[✅]`   Functional: unwrapping what the wrap produced under the same encapsulated value and context returns the original key
    * `[✅]`   Functional: two parameter sets set up independently over one pairing, each through the KEM factory and with every scalar drawn from the operating system's generator, each wrapping one piece-group key under its own encapsulated value and context, each unwrapped with the value a holder's credential under that set decapsulates, both return that piece-group key, whether both sets use the entitlement scope or one is an escrow set under the asset scope and the other a claimant's set under the entitlement scope
    * `[✅]`   Non-functional: the recovered piece-group key and the wrapping key are held in `Secret`s, the XOR written into a buffer that is moved into its `Secret` before the function returns; the crate names no vendor

  * `[✅]`   `role`
    * `[✅]`   Workflow: the `sidecar` area's second function, the inverse of the wrap under the same derivation
    * `[✅]`   Consumes `WrappedPieceGroupKey`, which `workflows/sidecar/wrap` owns
    * `[✅]`   Does not decapsulate, validate a capsule, or decrypt a piece; `workflows/decrypt` decapsulates through the KEM family after `workflows/sidecar/validate` checks the capsule, then unwraps here and decrypts through the cipher family
    * `[✅]`   Carries the credential KEM milestone's integration test across the pairing, hash-to-scalar, randomness, credential KEM, encoding, and key-derivation factories, the wrap, and the unwrap, and the milestone's commit

  * `[✅]`   `module`
    * `[✅]`   Bounded context: the `unwrap` function module of the `workflows` crate's `sidecar` area, holding `unwrap_piece_group_key`, its deps, params, payload, success, error, and return types, its signature type, and its mocks; and the crate's integration test's cross-set chain
    * `[✅]`   Edits `crates/workflows/Cargo.toml`, `crates/workflows/src/lib.rs`, `crates/workflows/src/sidecar/mod.rs`, and `crates/workflows/tests/integration_test.rs`, and creates the `sidecar/unwrap` module's files; nothing under `sidecar/wrap` changes
    * `[✅]`   Outside: the derivation, the encoding, the encapsulated value, the capsule's validation, the payload cipher, and the sidecar's layout

  * `[ ]`   `deps`
    * `[ ]`   `sidecar/wrap`, same crate, through `crate::sidecar::wrap::provides`: `WrappedPieceGroupKey`; in `unwrap/test.rs`, `build_wrapped_piece_group_key` and `WrappedPieceGroupKeyOverrides`; through `workflows` in the integration test, `wrap_piece_group_key`, `WrapPieceGroupKeyDeps`, `WrapPieceGroupKeyParams`, `WrapPieceGroupKeyPayload`, `PieceGroupKey`, `build_piece_group_key`, and `PieceGroupKeyOverrides`
    * `[✅]`   `domain`, `encoding`, `kdf`, and `kem`, the existing runtime dependencies: `DerivationContext`, `Secret`, and `SecretConstructorParams`; `IEncoderAdapter`, `EncodeParams`, `DerivationContextDescription`, and `DerivationContextDescriptionConstructorParams`; `IKeyDerivationAdapter`, `DeriveKeyParams`, `DeriveKeyPayload`, `DerivationPurpose`, and `DeriveKeyErrorReturn`; `EncapsulatedValue`
    * `[✅]`   `domain` with its `mocks` feature, now also through this crate's `mocks` feature, since the success builder calls `build_secret`
    * `[✅]`   `pairing`, `adapters/pairing`, `hash-to-scalar`, `adapters/hash-to-scalar`, and `random`, `adapters/random`, adapter ring, dev-dependencies each with its `mocks` feature, in the integration test only: `create_pairing` with its consumer names and builders, `create_hash_to_scalar` with its builders, and `create_random_source` with `fill_bytes` and its builders; nothing at runtime, and none names this crate
    * `[✅]`   `kem` with its `mocks` feature, the existing dev-dependency, in the integration test: `create_kem` with `CreateKemDeps`, `CreateKemPayload`, `build_create_kem_params`, `CreateKemParamsOverrides`, `IKemConsumer`, `ICredentialKemAdapter`, `ConsumeKemParams`, `ConsumeKemPayload`, `IdentityScope`, `SetupScope`, `KemIdentity`, and the setup, identity-mapping, issuance, encapsulation, and decapsulation params and payloads
    * `[✅]`   `Iterator::zip`, standard library, the XOR; `core::cell::RefCell`, standard library, in `unwrap/test.rs` only

  * `[ ]`   `context_slice`
    * `[ ]`   From `sidecar/wrap`: `PieceGroupKey` holding `Secret<[u8; 32]>` and `WrappedPieceGroupKey` holding private `[u8; 32]` with `as_bytes()` and checked `try_from_bytes`; in the integration test `wrap_piece_group_key<E: IEncoderAdapter>(&WrapPieceGroupKeyDeps { kdf, encoder }, WrapPieceGroupKeyParams, WrapPieceGroupKeyPayload { encapsulated, context, piece_group_key }) -> Result<WrapPieceGroupKeySuccessReturn { wrapped }, WrapPieceGroupKeyErrorReturn>`
    * `[✅]`   From `encoding`, `kdf`, and `kem`: the same calls `workflows/sidecar/wrap` states for its derivation
    * `[✅]`   From `kem`, in the integration test: `create_kem<'a, P: IPairingArithmetic, C: IKemConsumer<P>>(&CreateKemDeps { pairing, hash_to_scalar, consumer }, CreateKemParams { concrete, identifier, scope }, CreateKemPayload)` returning `{ output }`; `consume_kem<K: ICredentialKemAdapter<Pairing = P>>(&self, ConsumeKemParams, ConsumeKemPayload { adapter, scope })`; and `setup`, `derive_identity`, `issue`, `encapsulate`, and `decapsulate` as `kem/bb1_depth_one` states them, `encapsulate` returning `{ capsule, encapsulated: EncapsulatedValue }` and `decapsulate` returning `{ encapsulated: EncapsulatedValue }`
    * `[✅]`   From `pairing`, `hash-to-scalar`, and `random`, in the integration test: the calls `kem/factory`'s integration test states

  * `[✅]`   `crates/workflows/Cargo.toml`
    * `[✅]`   `[dev-dependencies]` reads `domain = { path = "../domain", features = ["mocks"] }`, `encoding = { path = "../../adapters/encoding", features = ["mocks"] }`, `kdf = { path = "../../adapters/kdf", features = ["mocks"] }`, `kem = { path = "../../adapters/kem", features = ["mocks"] }`, `pairing = { path = "../../adapters/pairing", features = ["mocks"] }`, `hash-to-scalar = { path = "../../adapters/hash-to-scalar", features = ["mocks"] }`, `random = { path = "../../adapters/random", features = ["mocks"] }`, and `hex = "0.4.3"`
    * `[✅]`   `[features]` reads `mocks = ["domain/mocks"]`
    * `[✅]`   `[package]`, `[dependencies]`, and `[lints]` are unchanged; no other table

  * `[✅]`   `crates/workflows/src/lib.rs`
    * `[✅]`   The barrel reads `mod sidecar;`, `pub use sidecar::unwrap::provides::*;`, and `pub use sidecar::wrap::provides::*;`, nothing else

  * `[✅]`   `crates/workflows/src/sidecar/mod.rs`
    * `[✅]`   Reads `pub(crate) mod unwrap;` and `pub(crate) mod wrap;`, nothing else

  * `[✅]`   `crates/workflows/src/sidecar/unwrap/interface.rs`
    * `[✅]`   `UnwrapPieceGroupKeyDeps<'a, E: IEncoderAdapter>`, a struct with `pub kdf: &'a dyn IKeyDerivationAdapter` and `pub encoder: &'a E`
    * `[✅]`   `UnwrapPieceGroupKeyParams`, the fieldless struct `pub struct UnwrapPieceGroupKeyParams;`
    * `[✅]`   `UnwrapPieceGroupKeyPayload<'a>`, a struct with `pub encapsulated: &'a EncapsulatedValue`, the value the holder's credential decapsulates, `pub context: &'a DerivationContext`, and `pub wrapped: &'a WrappedPieceGroupKey`, the set's sidecar entry's wrapped key
    * `[✅]`   `UnwrapPieceGroupKeySuccessReturn`, a struct with `pub piece_group_key: PieceGroupKey`
    * `[✅]`   `UnwrapPieceGroupKeyErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]` and the variants `Encoding(EncodeErrorReturn)`, `KeyDerivation(DeriveKeyErrorReturn)`, and `WrappingKeyLength { expected: usize, actual: usize }`
    * `[✅]`   `UnwrapPieceGroupKeyReturn`, the alias `Result<UnwrapPieceGroupKeySuccessReturn, UnwrapPieceGroupKeyErrorReturn>`
    * `[✅]`   `UnwrapPieceGroupKeyFn<E>`, the alias `fn(&UnwrapPieceGroupKeyDeps<'_, E>, UnwrapPieceGroupKeyParams, UnwrapPieceGroupKeyPayload<'_>) -> UnwrapPieceGroupKeyReturn`
    * `[✅]`   No derives on any type in this file beyond those stated; imports `DerivationContext` from `domain`, `IEncoderAdapter` and `EncodeErrorReturn` from `encoding`, `IKeyDerivationAdapter` and `DeriveKeyErrorReturn` from `kdf`, `EncapsulatedValue` from `kem`, and `PieceGroupKey`, `PIECE_GROUP_KEY_LENGTH`, and `WrappedPieceGroupKey` from `crate::sidecar::wrap::provides`

  * `[✅]`   `crates/workflows/src/sidecar/unwrap/interaction.spec.md`
    * `[✅]`   `unwrap_piece_group_key<E: IEncoderAdapter>(deps: &UnwrapPieceGroupKeyDeps<'_, E>, params: UnwrapPieceGroupKeyParams, payload: UnwrapPieceGroupKeyPayload<'_>) -> UnwrapPieceGroupKeyReturn`, in order:
    * `[✅]`   Encoding: `DerivationContextDescription::try_new(DerivationContextDescriptionConstructorParams)` unpacked irrefutably, then `deps.encoder.encode(EncodeParams { description: &description }, payload.context)` once; an encoding refusal returns `Err(UnwrapPieceGroupKeyErrorReturn::Encoding(error))` unchanged before KDF use; success yields the encoded context
    * `[✅]`   Key derivation refused: condition `deps.kdf.derive_key(DeriveKeyParams { purpose: DerivationPurpose::WrappingKey, length: PIECE_GROUP_KEY_LENGTH }, DeriveKeyPayload { key_material: payload.encapsulated.key_material(), context: &encoded })`, called once, returns `Err(error)`; outcome `Err(UnwrapPieceGroupKeyErrorReturn::KeyDerivation(error))`, the refusal unchanged
    * `[✅]`   Wrapping key of another length: condition the derivation succeeds and the derived key's exposed length is not `PIECE_GROUP_KEY_LENGTH`; decision the comparison, before any byte is combined; outcome `Err(UnwrapPieceGroupKeyErrorReturn::WrappingKeyLength { expected, actual })`, `expected` is `PIECE_GROUP_KEY_LENGTH` and `actual` is the derived key's length
    * `[✅]`   Unwrapped: condition the derived length is `PIECE_GROUP_KEY_LENGTH`; dependency call none; outcome `Ok(UnwrapPieceGroupKeySuccessReturn { piece_group_key })`, `piece_group_key` built by the crate-private `PieceGroupKey::from_array` from a `Secret<[u8; 32]>` filled by XORing `payload.wrapped.as_bytes()` with the wrapping key
    * `[✅]`   Ordering and lifecycle: encoding precedes derivation, the length check precedes the XOR, and the buffer is moved into its `Secret` as soon as it is filled; the wrapping key's `Secret` drops, and so zeroizes, when the function returns on every branch; the payload is borrowed and unchanged; `params` carries no control and is not read; the same payload always yields the same outcome
    * `[✅]`   Invariant: for any encapsulated value, context, and key, unwrapping the wrap's output returns the key

  * `[✅]`   `crates/workflows/src/sidecar/unwrap/mock.rs`

  * `[ ]`   `crates/workflows/src/sidecar/unwrap/test.rs`

  * `[✅]`   `construction`
    * `[✅]`   `unwrap_piece_group_key` is a function; its caller, `workflows/decrypt`, holds the key-derivation adapter from `create_key_derivation` and calls the unwrap inside its `IEncodingConsumer`, passing the encoder `create_encoding` hands it and the encapsulated value the KEM family's `decapsulate` returned

  * `[✅]`   `crates/workflows/src/sidecar/unwrap/mod.rs`
    * `[✅]`   Module declarations: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, `pub mod provides;`, and `#[cfg(test)] mod test;`
    * `[✅]`   `pub fn unwrap_piece_group_key<E: IEncoderAdapter>(deps: &UnwrapPieceGroupKeyDeps<'_, E>, _params: UnwrapPieceGroupKeyParams, payload: UnwrapPieceGroupKeyPayload<'_>) -> UnwrapPieceGroupKeyReturn`, realizing the branches and ordering of the interaction spec
    * `[✅]`   Imports `Secret` and `SecretConstructorParams` from `domain`, `DerivationContextDescription`, `DerivationContextDescriptionConstructorParams`, `IEncoderAdapter`, and `EncodeParams` from `encoding`, `DeriveKeyParams`, `DeriveKeyPayload`, and `DerivationPurpose` from `kdf`, and this module's types from `interface`
    * `[✅]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[✅]`   `crates/workflows/src/sidecar/unwrap/provides.rs`
    * `[✅]`   `pub use super::unwrap_piece_group_key;`, `pub use super::interface::*;`, and `#[cfg(any(test, feature = "mocks"))] pub use super::mock::*;`, nothing else

  * `[ ]`   `crates/workflows/tests/integration_test.rs`

  * `[✅]`   `directionality`
    * `[✅]`   `sidecar/unwrap` depends on `sidecar/wrap`'s `WrappedPieceGroupKey` within the crate, on `domain`, and on the encoding, key-derivation, and credential KEM families' surfaces; nothing in `sidecar/wrap` names `sidecar/unwrap`; no adapter crate names `workflows`; the integration test's dev-dependencies add no runtime edge; no cycle
    * `[✅]`   `workflows/decrypt` calls the unwrap after decapsulation; `harness-crypto/vectors` exercises it in the cross-set vector

  * `[ ]`   `requirements`
    * `[✅]`   `UnwrapPieceGroupKeyErrorReturn` derives `Debug`, `PartialEq`, and `Eq`, and the unwrap's refusals are asserted by `assert_eq!` against the whole expected error
    * `[✅]`   `crates/workflows/Cargo.toml`, `crates/workflows/src/lib.rs`, and `crates/workflows/src/sidecar/mod.rs` carry exactly the entries stated above
    * `[✅]`   `cargo check --workspace --all-targets --all-features`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo fmt --all --check`, and `cargo deny check` complete without error or warning
    * `[ ]`   `unwrap_xors_the_wrapped_key_with_the_derived_wrapping_key`, `unwrap_derives_the_wrapping_key_under_its_purpose_at_the_wrapped_keys_length`, `unwrap_derives_from_the_encapsulated_value_and_the_encoded_context`, `unwrap_refuses_a_wrapping_key_of_another_length`, and `unwrap_returns_an_encoder_contract_refusal` pass (CR-11 unwrap)
    * `[✅]`   Every test `workflows/sidecar/wrap` authored passes unchanged
    * `[✅]`   `two_independently_generated_entitlement_scope_sets_unwrap_one_piece_group_key_on_bls12_381_arkworks` and `an_escrow_set_and_a_claimants_set_unwrap_one_piece_group_key_on_bn254_arkworks` pass (CR-08 cross-set agreement; CR-11)
    * `[✅]`   The credential KEM milestone's exit holds: the pairing arithmetic trait on every pairing concrete with both libraries on each curve encoding the target group identically; CR-08's algebraic vectors, cross-holder agreement, cross-set agreement through the wrap and unwrap, rerandomized-credential validity, non-convertibility under the entitlement scope, holder-authored grants under the asset scope, malformed-capsule rejection, and trivial identity-element refusal; CD-08's scope declaration and admission; every crate building and passing the Rust CI definition on Windows, macOS, and Linux

  * `[✅]`   **Commit** `feat(harness): credential KEM over the pairing arithmetic trait, with the sidecar wrap and unwrap`
    * `[✅]`   Structural: the pairing family's arithmetic trait and its implementation on the four pairing concretes, with the `halo2curves` overlay in the workspace manifest and dependency policy; the `adapters/kem` crate with its factory and Boneh–Boyen depth-one concrete; the `crates/workflows` crate with its `sidecar` area's wrap and unwrap
    * `[✅]`   Behavioral: scalar arithmetic, source-group negation, identity tests, pairing products, and target-group encoding identical across libraries per curve; parameter-set setup under either identity scope, identity mapping with trivial-element refusal, issuance and rerandomization returning their witnesses, the public validity check, encapsulation, well-formedness, and decapsulation to one encapsulated value; every KEM value exposed as and rebuilt from its pairing components; a piece-group key wrapped and unwrapped under each set's wrapping key, one key opened by independently generated sets
    * `[✅]`   Contract: `IPairingArithmetic` and `IPairingConsumer` bounded by it; `ICredentialKemAdapter` with its associated types, component types, `KemDeclaration`, `IdentityScope`, `EncapsulatedValue`, `IKemConsumer`, and `create_kem`; `wrap_piece_group_key` with `WrappedPieceGroupKey`, and `unwrap_piece_group_key`

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
