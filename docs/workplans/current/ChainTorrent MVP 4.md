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

## Delivery proof and Solidity verifier

* `[ ]`   `harness-crypto/generate/evm/render` **Pure rendering of one Solidity library's source from a library name and an ordered list of named constant entries, opening with the license identifier and the exact compiler version pragma its params carry; owns the entry type the EVM generate concrete's functions return and the validated library-name, constant-name, license-identifier, compiler-version, string-literal, 256-bit value, entries, and source-text types; creates the `generate` area and the `evm` module**

  * `[ ]`   `objective`
    * `[ ]`   Problem: the Solidity pairing library and delivery verifier agree with the Rust reference bit for bit only if their constants and test vectors are generated from it; the generated directory holds, per curve, a constants library the suite's contracts import and a vectors library its Foundry tests import, each opening with the license identifier and the compiler version pragma the configuration carries and never edited by hand; every function of the EVM generate concrete returns named entries, each a byte string, a 256-bit, 64-bit, or 16-bit unsigned value, a boolean, or a string, and one function renders a library from a name and those entries (CR-09 parity inputs; the dependency map's `harness-crypto/generate/evm/render` row; the technical requirements' file tree for `contracts/evm/generated`; the Delivery proof and Solidity verifier milestone's scope)
    * `[ ]`   Functional: a library name is an ASCII uppercase letter followed by ASCII letters and digits; empty text, a leading byte outside `A`–`Z`, and the lowest later byte outside that set are each refused, the refusal carrying the byte and, for a later byte, its index
    * `[ ]`   Functional: a constant name is an ASCII uppercase letter followed by ASCII uppercase letters, digits, and underscores; empty text, a leading byte outside `A`–`Z`, and the lowest later byte outside that set are each refused, the refusal carrying the byte and, for a later byte, its index
    * `[ ]`   Functional: a license identifier is an SPDX short identifier of ASCII letters, digits, `.`, and `-`, with at most one `+` as its final byte; text whose identifier part is empty is refused as empty, and the lowest byte of the identifier part outside that set is refused with its index in the whole text
    * `[ ]`   Functional: a string value is printable ASCII, `0x20` through `0x7E`, other than `"` and `\`; the lowest byte outside that set is refused with its index; the empty string is admitted
    * `[ ]`   Functional: a 256-bit value is 32 bytes, most significant first
    * `[ ]`   Functional: a library's entries are a non-empty ordered list with unique constant names; an empty list is refused, and at the lowest index whose name equals a name at a lower index, the refusal carries the lowest such earlier index and that index
    * `[ ]`   Functional: the rendered source is the SPDX license line, the exact compiler version pragma, an empty line, the library declaration, one `internal constant` declaration per entry in payload order indented by four spaces, and the closing brace, each line ending in `\n`
    * `[ ]`   Functional: a byte string renders as `bytes` with a `hex"…"` literal of two lowercase hex digits per byte, `hex""` when empty; a 256-bit value as `uint256` with `0x` and two lowercase hex digits per byte, most significant first; a 64-bit and a 16-bit value as `uint64` and `uint16` in decimal; a boolean as `bool` with `true` or `false`; a string as `string` with its text between double quotes
    * `[ ]`   Functional: rendering an admitted library returns its source; every refusal is a constructor's
    * `[ ]`   Non-functional: the function reads no adapter, draws no randomness, reads no clock, and touches no filesystem; it holds no literal for a license, a compiler version, a library name, or an entry

  * `[ ]`   `role`
    * `[ ]`   App module in the adapter role: a function the EVM concrete of the harness's generate family owns, beneath the concrete's module in the generate area of the `apps/harness-crypto` crate
    * `[ ]`   Owns the entry type, the value types, and the params types the concrete's functions, the concrete, and the generate family's factory consume
    * `[ ]`   Does not write a file, choose a library name, or name a destination; `harness-crypto/generate/evm` does
    * `[ ]`   Does not choose a license identifier or a compiler version; `harness-crypto/config` carries them and `harness-crypto/main` passes them through the generate factory to the concrete
    * `[ ]`   Does not compute a constant or a vector; `harness-crypto/generate/evm/constants` and the vector functions do
    * `[ ]`   Does not author the generate family's interface, declaration, or mock; `harness-crypto/generate/evm` does
    * `[ ]`   Does not carry a commit

  * `[ ]`   `module`
    * `[ ]`   Bounded context: the `render` module beneath the `evm` module of the `generate` module of `apps/harness-crypto`, holding the library-name, constant-name, license-identifier, compiler-version, string-literal, 256-bit value, constant-value, entry, entries, and source-text types with their constructor params, returns, and refusals; the function's deps, params, payload, success, return, and signature types; the function; and its builders
    * `[ ]`   Files: `apps/harness-crypto/src/lib.rs`, `apps/harness-crypto/src/generate/mod.rs`, `apps/harness-crypto/src/generate/evm/mod.rs`, and `apps/harness-crypto/src/generate/evm/render/interface.rs`, `interaction.spec.md`, `mock.rs`, `test.rs`, `mod.rs`, and `provides.rs`
    * `[ ]`   Outside: every constant's and vector's value, the library names, the destination, the file names, the curve directory, the configuration, Foundry's formatting configuration, and the generate family's interface and factory

  * `[ ]`   `deps`
    * `[ ]`   `core::convert::Infallible`, standard library, the error arm of `SolidityUint256::try_new` and of the function's return
    * `[ ]`   `String`, `Vec`, `format!`, and `ToString`, standard library, assembling the source text and its literals
    * `[ ]`   No repository crate and no external crate; the module names nothing in `benchmark`, and `benchmark` names nothing in it

  * `[ ]`   `context_slice`
    * `[ ]`   No injected collaborator; `RenderDeps` is fieldless and the function reads only its params and payload

  * `[ ]`   `apps/harness-crypto/src/lib.rs`
    * `[ ]`   Reads `mod benchmark;`, `mod generate;`, and `pub use benchmark::provides::*;`, nothing else
    * `[ ]`   Until `generate/mod.rs` exists, `cargo check` reports the unresolved `mod generate`, which is the RED state for every element below that precedes it

  * `[ ]`   `apps/harness-crypto/src/generate/mod.rs`
    * `[ ]`   Reads `mod evm;`, nothing else
    * `[ ]`   Until `generate/evm/mod.rs` exists, `cargo check` reports the unresolved `mod evm`

  * `[ ]`   `apps/harness-crypto/src/generate/evm/mod.rs`
    * `[ ]`   Reads `mod render;`, nothing else
    * `[ ]`   Until `generate/evm/render/mod.rs` exists, `cargo check` reports the unresolved `mod render`

  * `[ ]`   `apps/harness-crypto/src/generate/evm/render/interface.rs`
    * `[ ]`   `SolidityLibraryName`, a struct with `#[derive(Clone, Debug, PartialEq, Eq)]` and the one field `pub(super) text: String`
    * `[ ]`   `SolidityLibraryNameConstructorParams`, a struct with the one field `pub text: String`; no derives
    * `[ ]`   `SolidityLibraryNameTryNewErrorReturn`, an enum with `#[derive(Clone, Copy, Debug, PartialEq, Eq)]` and the variants `Empty`, `LeadingCharacter { byte: u8 }`, and `InvalidCharacter { index: usize, byte: u8 }`
    * `[ ]`   `SolidityLibraryNameTryNewReturn`, the alias `Result<SolidityLibraryName, SolidityLibraryNameTryNewErrorReturn>`
    * `[ ]`   `SolidityConstantName`, a struct with `#[derive(Clone, Debug, PartialEq, Eq)]` and the one field `pub(super) text: String`
    * `[ ]`   `SolidityConstantNameConstructorParams`, a struct with the one field `pub text: String`; no derives
    * `[ ]`   `SolidityConstantNameTryNewErrorReturn`, an enum with `#[derive(Clone, Copy, Debug, PartialEq, Eq)]` and the variants `Empty`, `LeadingCharacter { byte: u8 }`, and `InvalidCharacter { index: usize, byte: u8 }`
    * `[ ]`   `SolidityConstantNameTryNewReturn`, the alias `Result<SolidityConstantName, SolidityConstantNameTryNewErrorReturn>`
    * `[ ]`   `SpdxLicenseIdentifier`, a struct with `#[derive(Clone, Debug, PartialEq, Eq)]` and the one field `pub(super) text: String`, the whole admitted text including a final `+`
    * `[ ]`   `SpdxLicenseIdentifierConstructorParams`, a struct with the one field `pub text: String`; no derives
    * `[ ]`   `SpdxLicenseIdentifierTryNewErrorReturn`, an enum with `#[derive(Clone, Copy, Debug, PartialEq, Eq)]` and the variants `Empty` and `InvalidCharacter { index: usize, byte: u8 }`
    * `[ ]`   `SpdxLicenseIdentifierTryNewReturn`, the alias `Result<SpdxLicenseIdentifier, SpdxLicenseIdentifierTryNewErrorReturn>`
    * `[ ]`   `SolidityCompilerVersion`, a struct with `#[derive(Clone, Copy, Debug, PartialEq, Eq)]` and the fields `pub major: u16`, `pub minor: u16`, and `pub patch: u16`, the exact release the pragma names
    * `[ ]`   `SolidityStringLiteral`, a struct with `#[derive(Clone, Debug, PartialEq, Eq)]` and the one field `pub(super) text: String`
    * `[ ]`   `SolidityStringLiteralConstructorParams`, a struct with the one field `pub text: String`; no derives
    * `[ ]`   `SolidityStringLiteralTryNewErrorReturn`, an enum with `#[derive(Clone, Copy, Debug, PartialEq, Eq)]` and the one variant `InvalidCharacter { index: usize, byte: u8 }`
    * `[ ]`   `SolidityStringLiteralTryNewReturn`, the alias `Result<SolidityStringLiteral, SolidityStringLiteralTryNewErrorReturn>`
    * `[ ]`   `SolidityUint256`, a struct with `#[derive(Clone, Copy, Debug, PartialEq, Eq)]` and the one field `pub(super) big_endian: [u8; 32]`
    * `[ ]`   `SolidityUint256ConstructorParams`, a struct with the one field `pub big_endian: [u8; 32]`, the value's bytes most significant first; no derives
    * `[ ]`   `SolidityUint256TryNewReturn`, the alias `Result<SolidityUint256, Infallible>`
    * `[ ]`   `SolidityConstantValue`, an enum with `#[derive(Clone, Debug, PartialEq, Eq)]` and the variants `Bytes(Vec<u8>)`, `Uint256(SolidityUint256)`, `Uint64(u64)`, `Uint16(u16)`, `Bool(bool)`, and `String(SolidityStringLiteral)`
    * `[ ]`   `SolidityLibraryEntry`, a struct with `#[derive(Clone, Debug, PartialEq, Eq)]` and the fields `pub name: SolidityConstantName` and `pub value: SolidityConstantValue`
    * `[ ]`   `SolidityLibraryEntries`, a struct with `#[derive(Clone, Debug, PartialEq, Eq)]` and the one field `pub(super) entries: Vec<SolidityLibraryEntry>`
    * `[ ]`   `SolidityLibraryEntriesConstructorParams`, a struct with the one field `pub entries: Vec<SolidityLibraryEntry>`; no derives
    * `[ ]`   `SolidityLibraryEntriesTryNewErrorReturn`, an enum with `#[derive(Clone, Copy, Debug, PartialEq, Eq)]` and the variants `Empty` and `DuplicateName { first_index: usize, duplicate_index: usize }`
    * `[ ]`   `SolidityLibraryEntriesTryNewReturn`, the alias `Result<SolidityLibraryEntries, SolidityLibraryEntriesTryNewErrorReturn>`
    * `[ ]`   `SoliditySourceText`, a struct with `#[derive(Clone, Debug, PartialEq, Eq)]` and the one field `pub(super) text: String`; the function is its only producer
    * `[ ]`   `RenderDeps`, the fieldless struct `pub struct RenderDeps;`
    * `[ ]`   `RenderParams`, a struct with `#[derive(Clone, Debug, PartialEq, Eq)]` and the fields `pub license: SpdxLicenseIdentifier` and `pub compiler_version: SolidityCompilerVersion`
    * `[ ]`   `RenderPayload`, a struct with `#[derive(Clone, Debug, PartialEq, Eq)]` and the fields `pub library_name: SolidityLibraryName` and `pub entries: SolidityLibraryEntries`
    * `[ ]`   `RenderSuccessReturn`, a struct with `#[derive(Clone, Debug, PartialEq, Eq)]` and the one field `pub source: SoliditySourceText`
    * `[ ]`   `RenderReturn`, the alias `Result<RenderSuccessReturn, Infallible>`
    * `[ ]`   `RenderFn`, the alias `fn(&RenderDeps, RenderParams, RenderPayload) -> RenderReturn`
    * `[ ]`   No `Default` on any type in this file
    * `[ ]`   Imports `core::convert::Infallible`; declares nothing else

  * `[ ]`   `apps/harness-crypto/src/generate/evm/render/interaction.spec.md`
    * `[ ]`   `SolidityLibraryName::try_new(params: SolidityLibraryNameConstructorParams) -> SolidityLibraryNameTryNewReturn`, decided over `params.text.as_bytes()`: empty, condition no byte, outcome `Err(SolidityLibraryNameTryNewErrorReturn::Empty)`; leading byte, condition the byte at index `0` is outside `b'A'..=b'Z'`, outcome `Err(SolidityLibraryNameTryNewErrorReturn::LeadingCharacter { byte })`; later byte, condition the lowest index `i` from `1` whose byte is outside `b'A'..=b'Z'`, `b'a'..=b'z'`, and `b'0'..=b'9'`, outcome `Err(SolidityLibraryNameTryNewErrorReturn::InvalidCharacter { index: i, byte })`; admitted, outcome `Ok(SolidityLibraryName { text })`, the text moved from the params; no dependency call
    * `[ ]`   `SolidityConstantName::try_new(params: SolidityConstantNameConstructorParams) -> SolidityConstantNameTryNewReturn`: the same branches in the same order with `SolidityConstantNameTryNewErrorReturn`'s variants, the later-byte set being `b'A'..=b'Z'`, `b'0'..=b'9'`, and `b'_'`; admitted, outcome `Ok(SolidityConstantName { text })`
    * `[ ]`   `SpdxLicenseIdentifier::try_new(params: SpdxLicenseIdentifierConstructorParams) -> SpdxLicenseIdentifierTryNewReturn`: the identifier part is the text without its last byte when that byte is `b'+'`, and the whole text otherwise; empty, condition the identifier part has no byte, outcome `Err(SpdxLicenseIdentifierTryNewErrorReturn::Empty)`; invalid, condition the lowest index `i` of the identifier part whose byte is outside `b'A'..=b'Z'`, `b'a'..=b'z'`, `b'0'..=b'9'`, `b'.'`, and `b'-'`, outcome `Err(SpdxLicenseIdentifierTryNewErrorReturn::InvalidCharacter { index: i, byte })`; admitted, outcome `Ok(SpdxLicenseIdentifier { text })`, the whole text
    * `[ ]`   `SolidityStringLiteral::try_new(params: SolidityStringLiteralConstructorParams) -> SolidityStringLiteralTryNewReturn`: invalid, condition the lowest index `i` whose byte is outside `0x20..=0x7E` or equals `b'"'` or `b'\\'`, outcome `Err(SolidityStringLiteralTryNewErrorReturn::InvalidCharacter { index: i, byte })`; admitted, outcome `Ok(SolidityStringLiteral { text })`
    * `[ ]`   `SolidityUint256::try_new(params: SolidityUint256ConstructorParams) -> SolidityUint256TryNewReturn`: one branch; outcome `Ok(SolidityUint256 { big_endian })`, the array moved from the params
    * `[ ]`   `SolidityLibraryEntries::try_new(params: SolidityLibraryEntriesConstructorParams) -> SolidityLibraryEntriesTryNewReturn`: empty, condition `params.entries` is empty, outcome `Err(SolidityLibraryEntriesTryNewErrorReturn::Empty)`; duplicate, condition the lowest index `j` whose entry's `name` equals the `name` of an entry at a lower index, `i` being the lowest such lower index, outcome `Err(SolidityLibraryEntriesTryNewErrorReturn::DuplicateName { first_index: i, duplicate_index: j })`; admitted, outcome `Ok(SolidityLibraryEntries { entries })`, the vector moved from the params in its order
    * `[ ]`   `SoliditySourceText::as_str(&self) -> &str` returns the text
    * `[ ]`   `render(deps: &RenderDeps, params: RenderParams, payload: RenderPayload) -> RenderReturn`, the trusted form: one branch; dependency call none; outcome `Ok(RenderSuccessReturn { source: SoliditySourceText { text } })`, `text` being `// SPDX-License-Identifier: `, the license's text, and `\n`; `pragma solidity `, `major`, `.`, `minor`, `.`, and `patch` in decimal, `;`, and `\n`; `\n`; `library `, the library name's text, and ` {\n`; one line per entry in payload order; and `}\n`
    * `[ ]`   An entry's line is four spaces, the Solidity type, ` internal constant `, the constant name's text, ` = `, the literal, `;`, and `\n`; by value: `Bytes(bytes)`, type `bytes`, literal `hex"`, each byte as two lowercase hex digits, and `"`; `Uint256(value)`, type `uint256`, literal `0x` and each byte of `value.big_endian` as two lowercase hex digits in array order; `Uint64(value)`, type `uint64`, literal the value in decimal; `Uint16(value)`, type `uint16`, literal the value in decimal; `Bool(value)`, type `bool`, literal `true` or `false`; `String(literal)`, type `string`, literal `"`, the literal's text, and `"`
    * `[ ]`   Ordering: the license line, the pragma, the empty line, the library line, the entries in payload order, the closing brace; `deps` is not read; equal params and payload yield equal text; no side effect

  * `[ ]`   `apps/harness-crypto/src/generate/evm/render/mock.rs`
    * `[ ]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`
    * `[ ]`   `SolidityLibraryNameConstructorParamsOverrides`, `#[derive(Default)]`, one field `pub text: Option<String>`; `build_solidity_library_name_constructor_params(overrides: SolidityLibraryNameConstructorParamsOverrides) -> SolidityLibraryNameConstructorParams`, the text defaulting to `"Constants"`; `build_solidity_library_name(overrides: SolidityLibraryNameConstructorParamsOverrides) -> SolidityLibraryName`, returning `SolidityLibraryName::try_new(build_solidity_library_name_constructor_params(overrides)).expect("built params are valid")`
    * `[ ]`   `SolidityConstantNameConstructorParamsOverrides`, `#[derive(Default)]`, one field `pub text: Option<String>`; `build_solidity_constant_name_constructor_params(overrides: SolidityConstantNameConstructorParamsOverrides) -> SolidityConstantNameConstructorParams`, the text defaulting to `"VALUE"`; `build_solidity_constant_name(overrides: SolidityConstantNameConstructorParamsOverrides) -> SolidityConstantName`, returning `SolidityConstantName::try_new(build_solidity_constant_name_constructor_params(overrides)).expect("built params are valid")`
    * `[ ]`   `SpdxLicenseIdentifierConstructorParamsOverrides`, `#[derive(Default)]`, one field `pub text: Option<String>`; `build_spdx_license_identifier_constructor_params(overrides: SpdxLicenseIdentifierConstructorParamsOverrides) -> SpdxLicenseIdentifierConstructorParams`, the text defaulting to `"UNLICENSED"`; `build_spdx_license_identifier(overrides: SpdxLicenseIdentifierConstructorParamsOverrides) -> SpdxLicenseIdentifier`, returning `SpdxLicenseIdentifier::try_new(build_spdx_license_identifier_constructor_params(overrides)).expect("built params are valid")`
    * `[ ]`   `SolidityStringLiteralConstructorParamsOverrides`, `#[derive(Default)]`, one field `pub text: Option<String>`; `build_solidity_string_literal_constructor_params(overrides: SolidityStringLiteralConstructorParamsOverrides) -> SolidityStringLiteralConstructorParams`, the text defaulting to `"uint256"`; `build_solidity_string_literal(overrides: SolidityStringLiteralConstructorParamsOverrides) -> SolidityStringLiteral`, returning `SolidityStringLiteral::try_new(build_solidity_string_literal_constructor_params(overrides)).expect("built params are valid")`
    * `[ ]`   `SolidityUint256ConstructorParamsOverrides`, `#[derive(Default)]`, one field `pub big_endian: Option<[u8; 32]>`; `build_solidity_uint256_constructor_params(overrides: SolidityUint256ConstructorParamsOverrides) -> SolidityUint256ConstructorParams`, the bytes defaulting to `[0x01; 32]`; `build_solidity_uint256(overrides: SolidityUint256ConstructorParamsOverrides) -> SolidityUint256`, returning the value from `let Ok(value) = SolidityUint256::try_new(build_solidity_uint256_constructor_params(overrides));`
    * `[ ]`   `SolidityCompilerVersionOverrides`, `#[derive(Default)]`, fields `pub major: Option<u16>`, `pub minor: Option<u16>`, and `pub patch: Option<u16>`; `build_solidity_compiler_version(overrides: SolidityCompilerVersionOverrides) -> SolidityCompilerVersion`, defaulting to `0`, `8`, and `28`
    * `[ ]`   `SolidityLibraryEntryOverrides`, `#[derive(Default)]`, fields `pub name: Option<SolidityConstantName>` and `pub value: Option<SolidityConstantValue>`; `build_solidity_library_entry(overrides: SolidityLibraryEntryOverrides) -> SolidityLibraryEntry`, defaulting to `build_solidity_constant_name(Default::default())` and `SolidityConstantValue::Uint16(1)`
    * `[ ]`   `SolidityLibraryEntriesConstructorParamsOverrides`, `#[derive(Default)]`, one field `pub entries: Option<Vec<SolidityLibraryEntry>>`; `build_solidity_library_entries_constructor_params(overrides: SolidityLibraryEntriesConstructorParamsOverrides) -> SolidityLibraryEntriesConstructorParams`, defaulting to `vec![build_solidity_library_entry(Default::default())]`; `build_solidity_library_entries(overrides: SolidityLibraryEntriesConstructorParamsOverrides) -> SolidityLibraryEntries`, returning `SolidityLibraryEntries::try_new(build_solidity_library_entries_constructor_params(overrides)).expect("built params are valid")`
    * `[ ]`   `RenderParamsOverrides`, `#[derive(Default)]`, fields `pub license: Option<SpdxLicenseIdentifier>` and `pub compiler_version: Option<SolidityCompilerVersion>`; `build_render_params(overrides: RenderParamsOverrides) -> RenderParams`, defaulting to `build_spdx_license_identifier(Default::default())` and `build_solidity_compiler_version(Default::default())`
    * `[ ]`   `RenderPayloadOverrides`, `#[derive(Default)]`, fields `pub library_name: Option<SolidityLibraryName>` and `pub entries: Option<SolidityLibraryEntries>`; `build_render_payload(overrides: RenderPayloadOverrides) -> RenderPayload`, defaulting to `build_solidity_library_name(Default::default())` and `build_solidity_library_entries(Default::default())`
    * `[ ]`   Dispositions: `SoliditySourceText` and `RenderSuccessReturn`, produced by the function, no builder; `RenderDeps`, fieldless, used by its production value; `SolidityConstantValue` and the refusal enums, used by their production values; `render`, called directly by the concrete, no function mock; the constructor params, no corruptions type and no invalidator, malformed text reaching each `try_new` as a `String`
    * `[ ]`   Imports this module's types from `super::interface`; nothing else

  * `[ ]`   `apps/harness-crypto/src/generate/evm/render/test.rs`
    * `[ ]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`; imports `render` from `super`, every type of `super::interface`, and every builder and overrides type of `super::mock`
    * `[ ]`   Each `try_new` call is unpacked by `let Ok(value) = … else { panic!(…) };` or `let Err(error) = … else { panic!(…) };`; each `render` call by `let Ok(success) = render(&RenderDeps, params, payload);`; rendered lines are read from `success.source.as_str()`
    * `[ ]`   `solidity_library_name_admits_an_upper_camel_identifier`: act `SolidityLibraryName::try_new` over `"PairingConstants"`; assert `value` equals `build_solidity_library_name` with `text: Some("PairingConstants".into())`
    * `[ ]`   `solidity_library_name_refuses_empty_text`: act over `""`; assert `error` equals `SolidityLibraryNameTryNewErrorReturn::Empty`
    * `[ ]`   `solidity_library_name_refuses_a_leading_byte_that_is_not_an_uppercase_letter`: act over `"pairingConstants"`; assert `error` equals `SolidityLibraryNameTryNewErrorReturn::LeadingCharacter { byte: b'p' }`
    * `[ ]`   `solidity_library_name_refuses_the_lowest_later_byte_outside_its_set`: act over `"Pairing_Lib-V1"`; assert `error` equals `SolidityLibraryNameTryNewErrorReturn::InvalidCharacter { index: 7, byte: b'_' }`
    * `[ ]`   `solidity_constant_name_admits_an_upper_snake_identifier`: act `SolidityConstantName::try_new` over `"G1_GENERATOR"`; assert `value` equals `build_solidity_constant_name` with `text: Some("G1_GENERATOR".into())`
    * `[ ]`   `solidity_constant_name_refuses_empty_text`: act over `""`; assert `error` equals `SolidityConstantNameTryNewErrorReturn::Empty`
    * `[ ]`   `solidity_constant_name_refuses_a_leading_digit`: act over `"1G_GENERATOR"`; assert `error` equals `SolidityConstantNameTryNewErrorReturn::LeadingCharacter { byte: b'1' }`
    * `[ ]`   `solidity_constant_name_refuses_the_lowest_later_byte_outside_its_set`: act over `"G1_Generator"`; assert `error` equals `SolidityConstantNameTryNewErrorReturn::InvalidCharacter { index: 4, byte: b'e' }`
    * `[ ]`   `spdx_license_identifier_admits_a_short_identifier_and_one_with_a_final_plus`: act over `"Apache-2.0"` and over `"GPL-2.0+"`; assert each `value` equals `build_spdx_license_identifier` with its text
    * `[ ]`   `spdx_license_identifier_refuses_empty_text_and_a_lone_plus`: act over `""` and over `"+"`; assert each `error` equals `SpdxLicenseIdentifierTryNewErrorReturn::Empty`
    * `[ ]`   `spdx_license_identifier_refuses_a_compound_expression`: act over `"MIT OR Apache-2.0"`; assert `error` equals `SpdxLicenseIdentifierTryNewErrorReturn::InvalidCharacter { index: 3, byte: b' ' }`
    * `[ ]`   `spdx_license_identifier_refuses_a_plus_that_is_not_final`: act over `"GPL+2.0"`; assert `error` equals `SpdxLicenseIdentifierTryNewErrorReturn::InvalidCharacter { index: 3, byte: b'+' }`
    * `[ ]`   `solidity_string_literal_admits_printable_text_and_the_empty_string`: act over `"bytes32,bytes20,uint64"` and over `""`; assert each `value` equals `build_solidity_string_literal` with its text
    * `[ ]`   `solidity_string_literal_refuses_a_double_quote_and_a_backslash`: act over `"a\"b"` and over `"a\\b"`; assert the errors equal `SolidityStringLiteralTryNewErrorReturn::InvalidCharacter { index: 1, byte: b'"' }` and `SolidityStringLiteralTryNewErrorReturn::InvalidCharacter { index: 1, byte: b'\\' }`
    * `[ ]`   `solidity_string_literal_refuses_a_control_byte_and_a_non_ascii_byte`: act over `"a\nb"` and over `"é"`; assert the errors equal `SolidityStringLiteralTryNewErrorReturn::InvalidCharacter { index: 1, byte: 0x0A }` and `SolidityStringLiteralTryNewErrorReturn::InvalidCharacter { index: 0, byte: 0xC3 }`
    * `[ ]`   `solidity_library_entries_refuses_an_empty_list`: act `SolidityLibraryEntries::try_new` over `vec![]`; assert `error` equals `SolidityLibraryEntriesTryNewErrorReturn::Empty`
    * `[ ]`   `solidity_library_entries_refuses_a_repeated_constant_name`: arrange entries named `"A"`, `"B"`, `"A"`, and `"B"`, each `build_solidity_library_entry` with its name from `build_solidity_constant_name`; act `try_new`; assert `error` equals `SolidityLibraryEntriesTryNewErrorReturn::DuplicateName { first_index: 0, duplicate_index: 2 }`
    * `[ ]`   `solidity_library_entries_keeps_its_entries_in_order`: arrange entries named `"B"` then `"A"`; act `try_new`; assert `value` equals `build_solidity_library_entries` with those entries in that order
    * `[ ]`   `render_opens_with_the_license_line_the_exact_pragma_and_an_empty_line`: arrange `build_render_params` with the license over `"Apache-2.0"` and the compiler version `0`, `8`, `28`; act `render` with `build_render_payload(Default::default())`; assert the first three lines are `// SPDX-License-Identifier: Apache-2.0`, `pragma solidity 0.8.28;`, and the empty line
    * `[ ]`   `render_declares_the_library_by_name_and_closes_it_with_one_final_newline`: arrange `build_render_payload` with the library name over `"PairingVectors"`; act `render`; assert the fourth line is `library PairingVectors {`, the text ends with `}\n`, and the text does not end with `\n\n`
    * `[ ]`   `render_writes_a_byte_string_as_a_lowercase_hex_literal`: arrange one entry `CHALLENGE_TAG` with `SolidityConstantValue::Bytes(vec![0x00, 0xAB, 0x10])`; act `render`; assert the fifth line is `    bytes internal constant CHALLENGE_TAG = hex"00ab10";`
    * `[ ]`   `render_writes_an_empty_byte_string_as_an_empty_hex_literal`: arrange one entry `EMPTY` with `SolidityConstantValue::Bytes(vec![])`; act `render`; assert the fifth line is `    bytes internal constant EMPTY = hex"";`
    * `[ ]`   `render_writes_a_256_bit_value_as_big_endian_hex_digits`: arrange one entry `SCALAR_FIELD_ORDER` with `SolidityConstantValue::Uint256` over `build_solidity_uint256` with `big_endian` the bytes `0x00` through `0x1F` ascending; act `render`; assert the fifth line is `    uint256 internal constant SCALAR_FIELD_ORDER = 0x000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f;`
    * `[ ]`   `render_writes_64_bit_and_16_bit_values_in_decimal`: arrange entries `VECTOR_COUNT` with `SolidityConstantValue::Uint64(18446744073709551615)` and `STATEMENT_VERSION` with `SolidityConstantValue::Uint16(65535)`; act `render`; assert the fifth and sixth lines are `    uint64 internal constant VECTOR_COUNT = 18446744073709551615;` and `    uint16 internal constant STATEMENT_VERSION = 65535;`
    * `[ ]`   `render_writes_booleans_as_true_and_false`: arrange entries `SECOND_GROUP_ARITHMETIC` with `SolidityConstantValue::Bool(true)` and `FIRST_GROUP_ONLY` with `SolidityConstantValue::Bool(false)`; act `render`; assert the fifth and sixth lines are `    bool internal constant SECOND_GROUP_ARITHMETIC = true;` and `    bool internal constant FIRST_GROUP_ONLY = false;`
    * `[ ]`   `render_writes_a_string_between_double_quotes`: arrange one entry `MINT_FIELDS` with `SolidityConstantValue::String` over `build_solidity_string_literal` with `"bytes32,bytes20,uint64"`; act `render`; assert the fifth line is `    string internal constant MINT_FIELDS = "bytes32,bytes20,uint64";`
    * `[ ]`   `render_writes_entries_in_payload_order`: arrange entries `ZETA` with `SolidityConstantValue::Uint16(2)` then `ALPHA` with `SolidityConstantValue::Uint16(1)`; act `render`; assert the fifth line is `    uint16 internal constant ZETA = 2;` and the sixth `    uint16 internal constant ALPHA = 1;`
    * `[ ]`   `render_produces_the_exact_source_of_a_reference_library`: arrange the license over `"Apache-2.0"`, the compiler version `0.8.28`, the library name over `"PairingConstants"`, and entries `CHALLENGE_TAG` with `SolidityConstantValue::Bytes(vec![0x43, 0x54])`, `STATEMENT_VERSION` with `SolidityConstantValue::Uint16(1)`, and `SECOND_GROUP_ARITHMETIC` with `SolidityConstantValue::Bool(true)`; act `render`; assert `success.source.as_str()` equals `"// SPDX-License-Identifier: Apache-2.0\npragma solidity 0.8.28;\n\nlibrary PairingConstants {\n    bytes internal constant CHALLENGE_TAG = hex\"4354\";\n    uint16 internal constant STATEMENT_VERSION = 1;\n    bool internal constant SECOND_GROUP_ARITHMETIC = true;\n}\n"`
    * `[ ]`   Every test block carries the full `Contract`, `Arrange`, `Act`, `Assert` header and the inline markers

  * `[ ]`   `construction`
    * `[ ]`   Each validated type's `try_new` is its only producer; `SolidityCompilerVersion` is built from its fields; `SoliditySourceText` is produced by `render` alone
    * `[ ]`   The concrete's functions build each `SolidityLibraryEntry` from a `SolidityConstantName` they admit and a `SolidityConstantValue`, a 256-bit value through `SolidityUint256ConstructorParams { big_endian }`; `harness-crypto/generate/evm` admits each library's name and its entries through `SolidityLibraryName::try_new` and `SolidityLibraryEntries::try_new` and calls `render` once per library with the license identifier and compiler version it holds
    * `[ ]`   `harness-crypto/config` admits the configured license identifier through `SpdxLicenseIdentifier::try_new` and builds the `SolidityCompilerVersion`

  * `[ ]`   `apps/harness-crypto/src/generate/evm/render/mod.rs`
    * `[ ]`   Module declarations: `mod interface;`, `#[cfg(test)] mod mock;`, `pub(crate) mod provides;`, and `#[cfg(test)] mod test;`
    * `[ ]`   Inherent `impl` blocks with `pub fn try_new` for `SolidityLibraryName`, `SolidityConstantName`, `SpdxLicenseIdentifier`, `SolidityStringLiteral`, `SolidityUint256`, and `SolidityLibraryEntries`, each realizing its branches in the interaction spec, and `pub fn as_str(&self) -> &str` for `SoliditySourceText`
    * `[ ]`   `pub fn render(_deps: &RenderDeps, params: RenderParams, payload: RenderPayload) -> RenderReturn`, assembling the text by `String::push_str`, each hex digit pair by `format!("{byte:02x}")`, and each decimal by `to_string()`, through an exhaustive `match` over `SolidityConstantValue`
    * `[ ]`   Imports this module's names from `interface` and `core::convert::Infallible`
    * `[ ]`   No other item

  * `[ ]`   `apps/harness-crypto/src/generate/evm/render/provides.rs`
    * `[ ]`   `pub(crate) use super::render;`, `pub(crate) use super::interface::*;`, and `#[cfg(test)] pub(crate) use super::mock::*;`, nothing else

  * `[ ]`   `directionality`
    * `[ ]`   `render` depends on the standard library alone; `generate/evm/mod.rs` declares it and `generate/mod.rs` declares `evm`; no cycle
    * `[ ]`   `harness-crypto/generate/evm/constants` and the vector functions consume `SolidityLibraryEntry`, `SolidityConstantName`, `SolidityConstantValue`, `SolidityUint256`, and `SolidityStringLiteral` through `super::render::provides`; `harness-crypto/generate/evm` calls `render`; the generate family's factory and `harness-crypto/config` carry `SpdxLicenseIdentifier` and `SolidityCompilerVersion`
    * `[ ]`   The rendered libraries are written under the configured generated directory that `contracts/evm/PairingLib` and `contracts/evm/DeliveryVerifier` import

  * `[ ]`   `requirements`
    * `[ ]`   `apps/harness-crypto/Cargo.toml` is unchanged; `apps/harness-crypto/src/lib.rs`, `generate/mod.rs`, and `generate/evm/mod.rs` carry exactly the lines stated above; no file under `benchmark` changes
    * `[ ]`   `cargo check --workspace --all-targets --all-features`, `cargo fmt --all --check`, and `cargo deny check` complete without error; `cargo clippy --workspace --all-targets --all-features` reports nothing beyond the library target's unused-item warnings for the `render` module, which `harness-crypto/generate/evm` resolves by calling `render`
    * `[ ]`   The library-name, constant-name, license-identifier, string-literal, and entries tests pass (CR-09 parity inputs: only identifiers, license lines, and literals Solidity accepts reach a rendered library)
    * `[ ]`   `render_opens_with_the_license_line_the_exact_pragma_and_an_empty_line`, `render_declares_the_library_by_name_and_closes_it_with_one_final_newline`, and `render_produces_the_exact_source_of_a_reference_library` pass
    * `[ ]`   `render_writes_a_byte_string_as_a_lowercase_hex_literal`, `render_writes_an_empty_byte_string_as_an_empty_hex_literal`, `render_writes_a_256_bit_value_as_big_endian_hex_digits`, `render_writes_64_bit_and_16_bit_values_in_decimal`, `render_writes_booleans_as_true_and_false`, `render_writes_a_string_between_double_quotes`, and `render_writes_entries_in_payload_order` pass (CR-09: one rendering for every constant and every vector)
    * `[ ]`   Every test under `apps/harness-crypto/src/benchmark` passes unchanged

* `[ ]`   `harness-crypto/generate/evm/constants` **The constants library's entries, generic over the selected pairing and the chain's forms: the identity mapping's, the proofs of possession's, and the delivery proof's challenge and weight domain tags read from their families' declarations; the delivery-statement version and the delivery purposes' codes; the mint and transfer transcripts' field sequences as ABI type names; the scalar field's order; the generators' precompile encodings; and whether the verifier has second-group arithmetic**

  * `[ ]`   `objective`
    * `[ ]`   Problem: the Solidity verifier hashes under the same domain tags, recomputes transcripts with the same field sequence under the same delivery-statement version and purpose codes, reduces modulo the same scalar field order, and starts from the same generators as the Rust reference, so each of these values is read from the reference and emitted as a named entry of the constants library the suite's contracts import, never written by hand (CR-09 and CR-11 parity inputs; the dependency map's `harness-crypto/generate/evm/constants` row; the technical requirements' file tree for `contracts/evm/generated/<curve>/Constants.sol`; the Delivery proof and Solidity verifier milestone's scope)
    * `[ ]`   Functional: the identity mapping's tag is read from the KEM declaration, the first- and second-group proofs of possession's tags from the key-agreement declaration, and the challenge and weight tags from the delivery-proof declaration, each emitted as a byte string
    * `[ ]`   Functional: the delivery-statement version the function is handed is emitted as a 16-bit value when the proof family's transcript descriptions serve it, and refused with the version otherwise
    * `[ ]`   Functional: each declared delivery purpose's code is emitted as a 16-bit value under its purpose's name, in the declared purposes' order
    * `[ ]`   Functional: the mint and transfer transcripts' field kinds, read from their descriptions over the forms in play, are each emitted as one string of ABI type names separated by commas, a fixed twenty-byte kind as `bytes20`
    * `[ ]`   Functional: the scalar field's order, read from the pairing's reference trait as big-endian bytes, is emitted as a 256-bit value, and an order whose length is not 32 bytes is refused with its length
    * `[ ]`   Functional: the first- and second-group generators' precompile encodings are emitted as byte strings
    * `[ ]`   Functional: whether the verifier has second-group arithmetic, read from the pairing's declaration, is emitted as a boolean
    * `[ ]`   Functional: the entries are returned in the order the interaction spec lists them
    * `[ ]`   Non-functional: the function reads no clock, draws no randomness, and touches no filesystem; it names no curve, concrete, or chain; every tag, version, code, and kind it emits is read from its producer

  * `[ ]`   `role`
    * `[ ]`   App module in the adapter role: a function the EVM concrete of the harness's generate family owns, beside `render` beneath the concrete's module
    * `[ ]`   Holds the EVM suite's table from canonical field kinds to ABI type names
    * `[ ]`   Does not render or write the library; `harness-crypto/generate/evm` passes the entries to `render` and writes the result
    * `[ ]`   Does not construct a pairing, KEM, key agreement, delivery proof, or forms; `harness-crypto/main` obtains each through its factory and the generate concrete hands this function the pairing and the declarations
    * `[ ]`   Does not carry a commit

  * `[ ]`   `module`
    * `[ ]`   Bounded context: the `constants` module beneath the `evm` module of the `generate` module of `apps/harness-crypto`, holding the function's deps, params, payload, success, error, return, and signature types, the function, and its params builder
    * `[ ]`   Files: `apps/harness-crypto/Cargo.toml`, `apps/harness-crypto/src/generate/evm/mod.rs`, and `apps/harness-crypto/src/generate/evm/constants/interface.rs`, `interaction.spec.md`, `mock.rs`, `test.rs`, `mod.rs`, and `provides.rs`
    * `[ ]`   Outside: the vectors, the library's rendering and name, the destination, every family's construction and admission, and the values each declaration carries

  * `[ ]`   `deps`
    * `[ ]`   `render`, same concrete, through `super::render::provides`: `SolidityLibraryEntry`, `SolidityConstantName`, `SolidityConstantNameConstructorParams`, `SolidityConstantNameTryNewErrorReturn`, `SolidityConstantValue`, `SolidityStringLiteral`, `SolidityStringLiteralConstructorParams`, `SolidityStringLiteralTryNewErrorReturn`, `SolidityUint256`, and `SolidityUint256ConstructorParams`, as `harness-crypto/generate/evm/render` authors them; in `constants/test.rs`, `build_solidity_constant_name`, `SolidityConstantNameConstructorParamsOverrides`, `build_solidity_string_literal`, `SolidityStringLiteralConstructorParamsOverrides`, `build_solidity_uint256`, and `SolidityUint256ConstructorParamsOverrides`
    * `[ ]`   `pairing`, `adapters/pairing`, adapter ring: `IPairingAdapter`, `IPairingReference`, `VerifierGroupArithmetic`, `G1GeneratorParams`, `G1GeneratorPayload`, `G2GeneratorParams`, `G2GeneratorPayload`, `EncodeG1Params`, `EncodeG1Payload`, `EncodeG2Params`, `EncodeG2Payload`, `ScalarFieldOrderParams`, and `ScalarFieldOrderPayload`
    * `[ ]`   `kem`, `adapters/kem`, adapter ring: `KemDeclaration`
    * `[ ]`   `envelope`, `adapters/envelope`, adapter ring: `KeyAgreementDeclaration`, as `envelope/pairing_elgamal` authors it
    * `[ ]`   `proof`, `adapters/proof`, adapter ring: `DeliveryProofDeclaration`, as `proof/schnorr_fs` authors it; `DELIVERY_STATEMENT_VERSION_ONE`, `DeliveryPurpose`, `DELIVERY_PURPOSES`, and `MintStatementDescription`, as `proof/mint_statement` authors them; and `TransferStatementDescription`, as `proof/transfer_statement` authors it
    * `[ ]`   `chain`, `adapters/chain`, adapter ring: `IChainForms`
    * `[ ]`   `encoding`, `adapters/encoding`, adapter ring: `IEncodingContract` and `CanonicalFieldKind`
    * `[ ]`   In `constants/test.rs` only, through each crate's `mocks` feature: `create_pairing`, `CreatePairingDeps`, `CreatePairingPayload`, `build_create_pairing_params`, `CreatePairingParamsOverrides`, `PairingConcrete`, `IPairingConsumer`, `IPairingArithmetic`, `ConsumePairingParams`, and `ConsumePairingPayload` from `pairing`; `create_chain_forms`, `CreateChainFormsDeps`, `CreateChainFormsPayload`, `build_create_chain_forms_params`, `IChainFormsConsumer`, `ConsumeChainFormsParams`, and `ConsumeChainFormsPayload` from `chain`; `build_kem_declaration` and `KemDeclarationOverrides` from `kem`; `build_key_agreement_declaration` and `KeyAgreementDeclarationOverrides` from `envelope`; and `build_delivery_proof_declaration` and `DeliveryProofDeclarationOverrides` from `proof`
    * `[ ]`   Every repository dependency is an adapter-ring crate the harness's app ring may depend on; none names `apps/harness-crypto`; no external crate

  * `[ ]`   `context_slice`
    * `[ ]`   From `pairing`: `IPairingAdapter::DECLARATION: PairingDeclaration` with `verifier_group_arithmetic: VerifierGroupArithmetic`, whose variants are `FirstGroupOnly` and `BothGroups`; `g1_generator(&self, G1GeneratorParams, G1GeneratorPayload) -> Result<G1GeneratorSuccessReturn { point }, Infallible>` and `g2_generator` likewise; `encode_g1(&self, EncodeG1Params, EncodeG1Payload { point }) -> Result<EncodeG1SuccessReturn { bytes: Self::EncodedG1 }, Infallible>` and `encode_g2` likewise, each encoding `AsRef<[u8]>`; `IPairingReference::scalar_field_order(&self, ScalarFieldOrderParams, ScalarFieldOrderPayload) -> Result<ScalarFieldOrderSuccessReturn { bytes: Vec<u8> }, Infallible>`, the order's big-endian bytes
    * `[ ]`   From `kem`: `KemDeclaration` with `identity_tag: &'static [u8]`
    * `[ ]`   From `envelope`: `KeyAgreementDeclaration` with `possession_g1_tag: &'static [u8]` and `possession_g2_tag: &'static [u8]`
    * `[ ]`   From `proof`: `DeliveryProofDeclaration` with `challenge_tag: &'static [u8]` and `weight_tag: &'static [u8]`; `DELIVERY_STATEMENT_VERSION_ONE: u16`; the closed `DeliveryPurpose` with `Mint`, `Transfer`, `Grant`, and `Replacement` and its `code()` returning `u16`; `DELIVERY_PURPOSES`, the four variants in code order; `MintStatementDescription<'a, P, F>` and `TransferStatementDescription<'a, P, F>`, each implementing `IEncodingContract` with `const FIELDS: &'static [CanonicalFieldKind]`
    * `[ ]`   From `encoding`: `CanonicalFieldKind` with the variants `FixedBytes32`, `Unsigned16`, `Unsigned32`, `Unsigned64`, `Text`, `Bytes`, `FixedBytes20`, and `Unsigned256`
    * `[ ]`   From `chain`, in the tests: `create_chain_forms(&CreateChainFormsDeps { consumer }, CreateChainFormsParams, CreateChainFormsPayload)` returning `CreateChainFormsSuccessReturn { output }`, `build_create_chain_forms_params(Default::default())` naming the EVM forms concrete and identifier, and `IChainFormsConsumer::consume_chain_forms<F: IChainForms>(&self, ConsumeChainFormsParams, ConsumeChainFormsPayload<F>) -> Self::Output`, the EVM forms' identity of kind `FixedBytes20`, entitlement and chain identifier of kind `Unsigned256`, and interval of kind `Unsigned64`
    * `[ ]`   From `pairing`, in the tests: `create_pairing(&CreatePairingDeps { consumer }, CreatePairingParams, CreatePairingPayload)` returning `CreatePairingSuccessReturn { output }`; `build_create_pairing_params(CreatePairingParamsOverrides { concrete, .. })`, the target-group encoding defaulting to the concrete's curve; `IPairingConsumer::consume_pairing<P: IPairingArithmetic + IPairingReference>(&self, ConsumePairingParams, ConsumePairingPayload { adapter }) -> Self::Output`

  * `[ ]`   `apps/harness-crypto/Cargo.toml`
    * `[ ]`   `[package]` with `name = "harness-crypto"`, `edition.workspace = true`, `rust-version.workspace = true`, and `publish.workspace = true`
    * `[ ]`   `[dependencies]` with `chain = { path = "../../adapters/chain" }`, `encoding = { path = "../../adapters/encoding" }`, `envelope = { path = "../../adapters/envelope" }`, `kem = { path = "../../adapters/kem" }`, `pairing = { path = "../../adapters/pairing" }`, `proof = { path = "../../adapters/proof" }`, and `random = { path = "../../adapters/random" }`
    * `[ ]`   `[dev-dependencies]` with `chain`, `envelope`, `kem`, `pairing`, `proof`, and `random`, each at its path above with `features = ["mocks"]`
    * `[ ]`   `[features]` with `mocks = ["chain/mocks", "envelope/mocks", "kem/mocks", "pairing/mocks", "proof/mocks", "random/mocks"]`
    * `[ ]`   `[lints]` with `workspace = true`

  * `[ ]`   `apps/harness-crypto/src/generate/evm/mod.rs`
    * `[ ]`   Reads `mod constants;` and `mod render;`, nothing else

  * `[ ]`   `apps/harness-crypto/src/generate/evm/constants/interface.rs`
    * `[ ]`   `ConstantsDeps<'a, P: IPairingReference>`, a struct with the one field `pub pairing: &'a P`
    * `[ ]`   `ConstantsParams`, a struct with `#[derive(Clone, Copy, Debug, PartialEq, Eq)]` and the one field `pub statement_version: u16`, the delivery-statement version the hash-card names
    * `[ ]`   `ConstantsPayload<'a>`, a struct with the fields `pub kem: &'a KemDeclaration`, `pub key_agreement: &'a KeyAgreementDeclaration`, and `pub delivery_proof: &'a DeliveryProofDeclaration`
    * `[ ]`   `ConstantsSuccessReturn`, a struct with `#[derive(Clone, Debug, PartialEq, Eq)]` and the one field `pub entries: Vec<SolidityLibraryEntry>`
    * `[ ]`   `ConstantsErrorReturn`, an enum with `#[derive(Clone, Copy, Debug, PartialEq, Eq)]` and the variants `UnsupportedStatementVersion { version: u16 }`, `ScalarFieldOrderLength { actual: usize }`, `ConstantName(SolidityConstantNameTryNewErrorReturn)`, and `StringLiteral(SolidityStringLiteralTryNewErrorReturn)`
    * `[ ]`   `ConstantsReturn`, the alias `Result<ConstantsSuccessReturn, ConstantsErrorReturn>`
    * `[ ]`   `ConstantsFn<'a, P>`, the alias `fn(&ConstantsDeps<'a, P>, ConstantsParams, ConstantsPayload<'_>) -> ConstantsReturn`
    * `[ ]`   No `Default` on any type in this file
    * `[ ]`   Imports `IPairingReference` from `pairing`, `KemDeclaration` from `kem`, `KeyAgreementDeclaration` from `envelope`, `DeliveryProofDeclaration` from `proof`, and `SolidityLibraryEntry`, `SolidityConstantNameTryNewErrorReturn`, and `SolidityStringLiteralTryNewErrorReturn` from `super::super::render::provides`; declares nothing else

  * `[ ]`   `apps/harness-crypto/src/generate/evm/constants/interaction.spec.md`
    * `[ ]`   `constants<'a, P: IPairingReference, F: IChainForms>(deps: &ConstantsDeps<'a, P>, params: ConstantsParams, payload: ConstantsPayload<'_>) -> ConstantsReturn`, the trusted form: every input is a typed value from the generate concrete
    * `[ ]`   Unsupported version: condition `params.statement_version` is not `DELIVERY_STATEMENT_VERSION_ONE`; decision equality, before any other step; dependency call none; outcome `Err(ConstantsErrorReturn::UnsupportedStatementVersion { version: params.statement_version })`
    * `[ ]`   Constant name refused: condition `SolidityConstantName::try_new(SolidityConstantNameConstructorParams { text })` returns `Err(error)` for an entry's name; dependency call that constructor, once per entry; outcome `Err(ConstantsErrorReturn::ConstantName(error))`, the refusal unchanged; every name the function holds is an uppercase identifier the constructor admits, so no input takes this branch, and it has no unit test
    * `[ ]`   Field sequence: for each of `<MintStatementDescription<'a, P, F> as IEncodingContract>::FIELDS` and `<TransferStatementDescription<'a, P, F> as IEncodingContract>::FIELDS`, each kind maps by an exhaustive `match` to its ABI type name, `FixedBytes32` to `bytes32`, `Unsigned16` to `uint16`, `Unsigned32` to `uint32`, `Unsigned64` to `uint64`, `Text` to `string`, `Bytes` to `bytes`, `FixedBytes20` to `bytes20`, and `Unsigned256` to `uint256`, the names joined in field order by `,`; dependency call `SolidityStringLiteral::try_new(SolidityStringLiteralConstructorParams { text })`, once per sequence; a refusal returns `Err(ConstantsErrorReturn::StringLiteral(error))`, the refusal unchanged; every ABI type name and `,` is printable ASCII the constructor admits, so no input takes that branch, and it has no unit test
    * `[ ]`   Scalar field order: dependency call `deps.pairing.scalar_field_order(ScalarFieldOrderParams, ScalarFieldOrderPayload)`, its success unpacked irrefutably; condition `<[u8; 32]>::try_from(success.bytes)` returns the vector; outcome `Err(ConstantsErrorReturn::ScalarFieldOrderLength { actual })`, `actual` the vector's length; otherwise the array is the `big_endian` of `SolidityUint256::try_new(SolidityUint256ConstructorParams { big_endian })`, unpacked irrefutably; each pairing concrete returns its order as 32 big-endian bytes, so no input takes the refusal, and it has no unit test
    * `[ ]`   Generators: dependency calls `deps.pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload)`, then `deps.pairing.encode_g1(EncodeG1Params, EncodeG1Payload { point })`, then `g2_generator` and `encode_g2` likewise, each success unpacked irrefutably; each encoding's `as_ref()` bytes copied into `SolidityConstantValue::Bytes`
    * `[ ]`   Second-group arithmetic: decision an exhaustive `match` on `P::DECLARATION.verifier_group_arithmetic`, `BothGroups` to `true` and `FirstGroupOnly` to `false`
    * `[ ]`   Purposes: for each `purpose` of `DELIVERY_PURPOSES` in order, the name by an exhaustive `match`, `DeliveryPurpose::Mint` to `PURPOSE_MINT`, `Transfer` to `PURPOSE_TRANSFER`, `Grant` to `PURPOSE_GRANT`, and `Replacement` to `PURPOSE_REPLACEMENT`, and the value `SolidityConstantValue::Uint16(purpose.code())`
    * `[ ]`   Computed: outcome `Ok(ConstantsSuccessReturn { entries })`, the entries in this order: `IDENTITY_TAG`, `Bytes` of `payload.kem.identity_tag`; `POSSESSION_G1_TAG` and `POSSESSION_G2_TAG`, `Bytes` of `payload.key_agreement.possession_g1_tag` and `possession_g2_tag`; `CHALLENGE_TAG` and `WEIGHT_TAG`, `Bytes` of `payload.delivery_proof.challenge_tag` and `weight_tag`; `DELIVERY_STATEMENT_VERSION`, `Uint16` of `params.statement_version`; the purposes' entries; `MINT_FIELDS` and `TRANSFER_FIELDS`, `String` of the mint and the transfer field sequences; `SCALAR_FIELD_ORDER`, `Uint256` of the order; `G1_GENERATOR` and `G2_GENERATOR`, `Bytes` of the generators' encodings; and `SECOND_GROUP_ARITHMETIC`, `Bool` of the second-group arithmetic
    * `[ ]`   Ordering: the version check, then the entries in the order above, the first refusal returned; the same deps, params, and payload yield the same entries; no side effect

  * `[ ]`   `apps/harness-crypto/src/generate/evm/constants/mock.rs`
    * `[ ]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`
    * `[ ]`   `ConstantsParamsOverrides`, `#[derive(Default)]`, one field `pub statement_version: Option<u16>`; `build_constants_params(overrides: ConstantsParamsOverrides) -> ConstantsParams`, defaulting to `DELIVERY_STATEMENT_VERSION_ONE`
    * `[ ]`   Dispositions: `ConstantsDeps`, borrowing the pairing a test obtains from the pairing factory, and `ConstantsPayload`, borrowing the declarations a test builds, written at the call site; `ConstantsSuccessReturn`, produced by the function, no builder; `ConstantsErrorReturn`, used by its production values; `constants`, called directly by the concrete, no function mock
    * `[ ]`   Imports `ConstantsParams` from `super::interface` and `DELIVERY_STATEMENT_VERSION_ONE` from `proof`; nothing else

  * `[ ]`   `apps/harness-crypto/src/generate/evm/constants/test.rs`
    * `[ ]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`; imports `constants` from `super`, `ConstantsDeps`, `ConstantsPayload`, `ConstantsReturn`, and `ConstantsErrorReturn` from `super::interface`, `build_constants_params` and `ConstantsParamsOverrides` from `super::mock`, the render builders and types the `deps` element lists from `super::super::render::provides`, and the test-only names the `deps` element lists from `pairing`, `chain`, `kem`, `envelope`, and `proof`
    * `[ ]`   `ConstantsPairingProbe`, a test-local struct with the fields `params: ConstantsParams`, `kem: KemDeclaration`, `key_agreement: KeyAgreementDeclaration`, and `delivery_proof: DeliveryProofDeclaration`, implementing `IPairingConsumer` with `type Output = ConstantsReturn;`, whose `consume_pairing` calls `create_chain_forms(&CreateChainFormsDeps { consumer: ConstantsFormsProbe { pairing: &payload.adapter, probe: self } }, build_create_chain_forms_params(Default::default()), CreateChainFormsPayload)`, unpacked by `let Ok(success) = … else { panic!(…) };`, and returns `success.output`
    * `[ ]`   `ConstantsFormsProbe<'a, P>`, a test-local struct with the fields `pairing: &'a P` and `probe: &'a ConstantsPairingProbe`, implementing `IChainFormsConsumer` with `type Output = ConstantsReturn;`, whose `consume_chain_forms<F: IChainForms>` returns `constants::<P, F>(&ConstantsDeps { pairing: self.pairing }, self.probe.params, ConstantsPayload { kem: &self.probe.kem, key_agreement: &self.probe.key_agreement, delivery_proof: &self.probe.delivery_proof })`
    * `[ ]`   Each test builds a `ConstantsPairingProbe`, its declarations from `build_kem_declaration`, `build_key_agreement_declaration`, and `build_delivery_proof_declaration` with the overrides it names and its params from `build_constants_params`, runs it through `create_pairing(&CreatePairingDeps { consumer: probe }, build_create_pairing_params(CreatePairingParamsOverrides { concrete: Some(…), ..Default::default() }), CreatePairingPayload)`, unpacks `success.output`, and reads an entry as the one whose `name` equals `build_solidity_constant_name` over the entry's name; each test runs on `PairingConcrete::Bn254Arkworks` unless its name states another curve
    * `[ ]`   `constants_refuse_a_statement_version_no_description_serves`: arrange params with `statement_version: Some(2)`; act; assert the output is `Err(ConstantsErrorReturn::UnsupportedStatementVersion { version: 2 })`
    * `[ ]`   `constants_carry_each_family_declarations_domain_tags`: arrange the KEM declaration with `identity_tag: Some(b"test-identity")`, the key-agreement declaration with `possession_g1_tag: Some(b"test-possession-g1")` and `possession_g2_tag: Some(b"test-possession-g2")`, and the delivery-proof declaration with `challenge_tag: Some(b"test-challenge")` and `weight_tag: Some(b"test-weight")`; act; assert `IDENTITY_TAG`, `POSSESSION_G1_TAG`, `POSSESSION_G2_TAG`, `CHALLENGE_TAG`, and `WEIGHT_TAG` hold `SolidityConstantValue::Bytes` of `b"test-identity"`, `b"test-possession-g1"`, `b"test-possession-g2"`, `b"test-challenge"`, and `b"test-weight"`
    * `[ ]`   `constants_carry_the_statement_version_and_each_purpose_code`: act; assert `DELIVERY_STATEMENT_VERSION` holds `SolidityConstantValue::Uint16(1)`, and `PURPOSE_MINT`, `PURPOSE_TRANSFER`, `PURPOSE_GRANT`, and `PURPOSE_REPLACEMENT` hold `SolidityConstantValue::Uint16(1)`, `(2)`, `(3)`, and `(4)`
    * `[ ]`   `constants_render_the_mint_and_transfer_field_sequences_over_the_evm_forms`: act; assert `MINT_FIELDS` holds `SolidityConstantValue::String` of `build_solidity_string_literal` over `"bytes32,uint16,uint256,bytes20,bytes32,bytes32,uint256,uint256,uint64,uint64,uint16,bytes20,bytes20,bytes,bytes,bytes,bytes,bytes,bytes,uint64,bytes,bytes,bytes,bytes,bytes"` and `TRANSFER_FIELDS` of `build_solidity_string_literal` over `"bytes32,uint16,uint256,bytes20,bytes32,bytes32,uint256,uint256,uint64,uint64,uint16,bytes20,bytes20,bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes,uint64,bytes,bytes,bytes,bytes,bytes,bytes"`
    * `[ ]`   `constants_carry_the_bn254_scalar_field_order_big_endian`: act; assert `SCALAR_FIELD_ORDER` holds `SolidityConstantValue::Uint256` of `build_solidity_uint256` over the bytes of `0x30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001`
    * `[ ]`   `constants_carry_the_bls12_381_scalar_field_order_big_endian`: arrange `PairingConcrete::Bls12381Arkworks`; act; assert `SCALAR_FIELD_ORDER` holds `SolidityConstantValue::Uint256` of `build_solidity_uint256` over the bytes of `0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001`
    * `[ ]`   `constants_carry_the_bn254_generators_precompile_encodings`: act; assert `G1_GENERATOR` holds `SolidityConstantValue::Bytes` of 31 zero bytes, `0x01`, 31 zero bytes, and `0x02`, and `G2_GENERATOR` holds `SolidityConstantValue::Bytes` of the bytes of `0x198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c2`, `0x1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed`, `0x090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b`, and `0x12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa`, in that order
    * `[ ]`   `constants_declare_first_group_only_arithmetic_on_bn254_and_both_groups_on_bls12_381`: act on `PairingConcrete::Bn254Arkworks` and on `PairingConcrete::Bls12381Arkworks`; assert `SECOND_GROUP_ARITHMETIC` holds `SolidityConstantValue::Bool(false)` and `SolidityConstantValue::Bool(true)` respectively
    * `[ ]`   `constants_list_their_entries_in_declared_order`: act; assert the entries' names, in order, equal `build_solidity_constant_name` over `IDENTITY_TAG`, `POSSESSION_G1_TAG`, `POSSESSION_G2_TAG`, `CHALLENGE_TAG`, `WEIGHT_TAG`, `DELIVERY_STATEMENT_VERSION`, `PURPOSE_MINT`, `PURPOSE_TRANSFER`, `PURPOSE_GRANT`, `PURPOSE_REPLACEMENT`, `MINT_FIELDS`, `TRANSFER_FIELDS`, `SCALAR_FIELD_ORDER`, `G1_GENERATOR`, `G2_GENERATOR`, and `SECOND_GROUP_ARITHMETIC`
    * `[ ]`   Every test block carries the full `Contract`, `Arrange`, `Act`, `Assert` header and the inline markers

  * `[ ]`   `construction`
    * `[ ]`   `harness-crypto/generate/evm` calls `constants::<P, F>` with `ConstantsDeps { pairing }` over the pairing it borrows, `ConstantsParams { statement_version }` over the version it holds, and `ConstantsPayload` over references to the declarations it holds, and passes the entries to `render` for the constants library

  * `[ ]`   `apps/harness-crypto/src/generate/evm/constants/mod.rs`
    * `[ ]`   Module declarations: `mod interface;`, `#[cfg(test)] mod mock;`, `pub(crate) mod provides;`, and `#[cfg(test)] mod test;`
    * `[ ]`   `pub fn constants<'a, P: IPairingReference, F: IChainForms>(deps: &ConstantsDeps<'a, P>, params: ConstantsParams, payload: ConstantsPayload<'_>) -> ConstantsReturn`, realizing each branch of the interaction spec, the kind-to-type table and the purpose names each an exhaustive `match` within the function
    * `[ ]`   Imports this module's names from `interface`, the render types the `deps` element lists from `super::super::render::provides`, and the pairing, proof, chain, and encoding names the `deps` element lists from their crates
    * `[ ]`   No other item

  * `[ ]`   `apps/harness-crypto/src/generate/evm/constants/provides.rs`
    * `[ ]`   `pub(crate) use super::constants;`, `pub(crate) use super::interface::*;`, and `#[cfg(test)] pub(crate) use super::mock::*;`, nothing else

  * `[ ]`   `directionality`
    * `[ ]`   `constants` depends on `render` within the concrete and on the `pairing`, `kem`, `envelope`, `proof`, `chain`, and `encoding` crates' public surfaces; `render` names nothing in `constants`; none of those crates names `apps/harness-crypto`; no cycle
    * `[ ]`   `harness-crypto/generate/evm` calls `constants` and renders its entries as `Constants.sol`; `contracts/evm/PairingLib` and `contracts/evm/DeliveryVerifier` import that library

  * `[ ]`   `requirements`
    * `[ ]`   `apps/harness-crypto/Cargo.toml` carries exactly the tables and entries stated above, and `apps/harness-crypto/src/generate/evm/mod.rs` exactly the lines stated above
    * `[ ]`   `cargo check --workspace --all-targets --all-features`, `cargo fmt --all --check`, and `cargo deny check` complete without error; `cargo clippy --workspace --all-targets --all-features` reports no warning other than the library target's unused-item warnings for the `render` and `constants` modules
    * `[ ]`   `constants_refuse_a_statement_version_no_description_serves` passes
    * `[ ]`   `constants_carry_each_family_declarations_domain_tags`, `constants_carry_the_statement_version_and_each_purpose_code`, and `constants_render_the_mint_and_transfer_field_sequences_over_the_evm_forms` pass (CR-09 and CR-11: the tags, version, purpose codes, and transcript layout the contract hashes and encodes under)
    * `[ ]`   `constants_carry_the_bn254_scalar_field_order_big_endian`, `constants_carry_the_bls12_381_scalar_field_order_big_endian`, `constants_carry_the_bn254_generators_precompile_encodings`, and `constants_declare_first_group_only_arithmetic_on_bn254_and_both_groups_on_bls12_381` pass (CR-10 and CR-11: the modulus, generators, and verifier form the contract computes with)
    * `[ ]`   `constants_list_their_entries_in_declared_order` passes
    * `[ ]`   Every test under `apps/harness-crypto/src/generate/evm/render` and `apps/harness-crypto/src/benchmark` passes

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