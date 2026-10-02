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

* `[ ]`   `harness-crypto/generate/evm/render` **Rendering of one Solidity library's source from a library name and an ordered list of named entries, opening with the license identifier and the compiler version pragma its params carry; owns the entry type every other function of the EVM generate concrete returns; creates the `generate` area and the `evm` module**

  * `[ ]`   `objective`
    * `[ ]`   Problem: the Solidity verifier is proven bit for bit against the Rust reference only if every constant it hashes under and every vector its tests read is written from the reference rather than authored, and the constants and each kind of vector the EVM generate concrete emits reach a file through one rendering, so a value of a given kind is written one way wherever it appears; a Solidity source file opens with a license identifier and a compiler version pragma, which are configured values and not the generator's (CR-09 parity inputs; the dependency map's `harness-crypto/generate/evm/render` row and its flags on the generate concrete's decomposition and on the configured compiler version and license identifier; the Delivery proof and Solidity verifier milestone's scope; the technical requirements' file tree for `contracts/evm/generated`)
    * `[ ]`   Functional: the source opens with the license identifier line and the compiler version pragma line, each carrying the value `params` holds, followed by one library of the name `params` holds
    * `[ ]`   Functional: the library declares one `internal constant` per entry, in the entries' order, a byte string as `bytes` with a hex literal, a 256-bit unsigned value as `uint256` with a hex number of sixty-four digits, a sixty-four-bit and a sixteen-bit unsigned value as `uint64` and `uint16` in decimal, a boolean as `bool`, and a string as `string` with a quoted literal
    * `[ ]`   Functional: an entry list holding nothing renders a library with an empty body
    * `[ ]`   Functional: a license identifier or compiler version that would break its line, a library or entry name that is not an identifier, an entry name an earlier entry carries, and a string that cannot sit inside a quoted literal are each refused with a typed error, the entry-level refusals carrying the entry's index
    * `[ ]`   Functional: the same params and entries always render the same source
    * `[ ]`   Non-functional: pure; reads no adapter, no clock, and no file, and names no curve, family, path, compiler version, or license

  * `[ ]`   `role`
    * `[ ]`   App module, a function the EVM generate concrete owns, and the concrete's first source file, so it creates `apps/harness-crypto/src/generate/mod.rs` and `apps/harness-crypto/src/generate/evm/mod.rs` with the module wiring alone and adds the `generate` line to the crate barrel; the generate family's interface, declaration, mock, and adapter are `harness-crypto/generate/evm`'s
    * `[ ]`   Owns `SolidityEntry` and `SolidityValue`, the entry type `constants`, `group_vectors`, `rejection_vectors`, `mapping_vectors`, `possession_vectors`, `challenge_vectors`, and `proof_vectors` each return
    * `[ ]`   Does not choose a library's name, an entry's name, or the order of entries; does not gather a constant or produce a vector; does not write a file or name a directory; `harness-crypto/generate/evm` writes what this function returns
    * `[ ]`   Does not refuse a name that is a Solidity keyword; `forge build` refuses the file that carries one
    * `[ ]`   Does not carry a commit

  * `[ ]`   `module`
    * `[ ]`   Bounded context: the `render` module beneath the `evm` concrete of the harness's `generate` area, holding the entry and value types, the function's deps, params, payload, success, error, and return types, and the function
    * `[ ]`   Creates `apps/harness-crypto/src/generate/mod.rs`, `apps/harness-crypto/src/generate/evm/mod.rs`, and the `apps/harness-crypto/src/generate/evm/render` module; the crate barrel gains the area's line; the manifest is unchanged
    * `[ ]`   Outside: which entries a library holds and what they are named, the values themselves, the generated directory and its files, the configuration that ships the compiler version and license identifier, and the Solidity that imports the libraries

  * `[ ]`   `deps`
    * `[ ]`   `core::fmt::Write`, standard library, the hex and decimal digits written into the source `String` through `write!`; `std::collections::BTreeSet`, standard library, the entry names already rendered
    * `[ ]`   `harness-crypto/benchmark`, same crate, precedes this node as the module that created the crate and its barrel; this module names nothing in it and it names nothing here
    * `[ ]`   No repository crate, no external crate, and no new manifest entry; no reverse dependency; nothing depends on this module yet

  * `[ ]`   `context_slice`
    * `[ ]`   From `core::fmt::Write`: `write!(source, …)` on a `String`, returning `core::fmt::Result`, whose error a `String` never produces; `{byte:02x}` for a byte's two lowercase hex digits and `{value}` for a `u64` or `u16` in decimal
    * `[ ]`   From `BTreeSet<&str>`: `insert(name)`, returning `false` when the set already holds the name
    * `[ ]`   From `apps/harness-crypto/src/lib.rs` as it stands: `mod benchmark;` and `pub use benchmark::provides::*;`

  * `[ ]`   `apps/harness-crypto/src/lib.rs`
    * `[ ]`   The crate barrel reads `mod benchmark;`, `mod generate;`, and `pub use benchmark::provides::*;`, nothing else; the area's surface is not re-exported

  * `[ ]`   `apps/harness-crypto/src/generate/mod.rs`
    * `[ ]`   Module wiring only: `mod evm;`, nothing else; `harness-crypto/generate/factory` adds the family's factory module and public surface

  * `[ ]`   `apps/harness-crypto/src/generate/evm/mod.rs`
    * `[ ]`   Module wiring only: `mod render;`, nothing else; each further function the concrete owns adds its line, and `harness-crypto/generate/evm` adds the concrete's modules and the adapter
    * `[ ]`   Until `render/mod.rs` exists, `cargo check` reports the unresolved `mod render`, which is the RED state for every element below that precedes it

  * `[ ]`   `apps/harness-crypto/src/generate/evm/render/interface.rs`
    * `[ ]`   `SolidityValue`, an enum with the variants `Bytes(Vec<u8>)`, `Unsigned256([u8; 32])`, the value's big-endian bytes, `Unsigned64(u64)`, `Unsigned16(u16)`, `Boolean(bool)`, and `String(String)`
    * `[ ]`   `SolidityEntry`, a struct with `pub name: String` and `pub value: SolidityValue`
    * `[ ]`   `RenderDeps`, the fieldless struct `pub struct RenderDeps;`
    * `[ ]`   `RenderParams`, a struct with `pub license: String`, `pub compiler_version: String`, and `pub library_name: String`, the values that select the file's opening lines and the library rendered
    * `[ ]`   `RenderPayload`, a struct with `pub entries: Vec<SolidityEntry>`
    * `[ ]`   `RenderSuccessReturn`, a struct with `pub source: String`
    * `[ ]`   `RenderErrorReturn`, an enum with the variants `InvalidLicense`, `InvalidCompilerVersion`, `InvalidLibraryName`, `InvalidEntryName { index: usize }`, `DuplicateEntryName { index: usize }`, and `UnrenderableString { index: usize }`
    * `[ ]`   `RenderReturn`, the alias `Result<RenderSuccessReturn, RenderErrorReturn>`
    * `[ ]`   No derives on any type in this file; no `RenderFn` alias, since nothing injects the function and its signature is the one in `mod.rs`; no import; declares nothing else

  * `[ ]`   `apps/harness-crypto/src/generate/evm/render/interaction.spec.md`
    * `[ ]`   `render(deps: &RenderDeps, params: RenderParams, payload: RenderPayload) -> RenderReturn`, the trusted form: the entries arrive as typed values from the functions the concrete owns
    * `[ ]`   The section states the two admission rules the branches apply: a line value is admitted when it is not empty, every byte is `0x20` through `0x7E`, and neither its first nor its last byte is a space; an identifier is admitted when it is not empty, its first byte is an ASCII letter or `_`, and every later byte is an ASCII letter, an ASCII digit, or `_`
    * `[ ]`   License refused: condition `params.license` is not an admitted line value; decision the line-value rule; no dependency call; outcome `Err(RenderErrorReturn::InvalidLicense)`
    * `[ ]`   Compiler version refused: condition `params.compiler_version` is not an admitted line value or holds `;`; decision the line-value rule and the search for `;`; outcome `Err(RenderErrorReturn::InvalidCompilerVersion)`
    * `[ ]`   Library name refused: condition `params.library_name` is not an admitted identifier; decision the identifier rule; outcome `Err(RenderErrorReturn::InvalidLibraryName)`
    * `[ ]`   Entry name refused: condition, for the entry at `index`, `name` is not an admitted identifier; decision the identifier rule; outcome `Err(RenderErrorReturn::InvalidEntryName { index })`
    * `[ ]`   Entry name repeated: condition, for the entry at `index`, `BTreeSet::insert(name)` returns `false`; decision the insertion's result; dependency call `insert`, once per entry, after the identifier rule; outcome `Err(RenderErrorReturn::DuplicateEntryName { index })`
    * `[ ]`   String refused: condition, for the entry at `index`, the value is `SolidityValue::String` holding a byte outside `0x20` through `0x7E`, a `"`, or a `\`; decision the scan of the string's bytes; outcome `Err(RenderErrorReturn::UnrenderableString { index })`; an empty string is admitted
    * `[ ]`   Rendered: condition every check passes; outcome `Ok(RenderSuccessReturn { source })`, `source` being, each line ended by one `\n`: `// SPDX-License-Identifier: {license}`; `pragma solidity {compiler_version};`; an empty line; `library {library_name} {`; one line per entry, indented by four spaces, in the entries' order; and `}`; with no entry the library is the one line `library {library_name} {}`
    * `[ ]`   An entry's line, by its value's variant: `Bytes`, `bytes internal constant {name} = hex"{digits}";`, two lowercase hex digits per byte and none for an empty byte string; `Unsigned256`, `uint256 internal constant {name} = 0x{digits};`, the sixty-four lowercase hex digits of the thirty-two bytes in order; `Unsigned64`, `uint64 internal constant {name} = {value};` in decimal; `Unsigned16`, `uint16 internal constant {name} = {value};` in decimal; `Boolean`, `bool internal constant {name} = true;` or `= false;`; `String`, `string internal constant {name} = "{value}";`, the string's bytes unchanged
    * `[ ]`   Ordering: the license, the compiler version, and the library name are checked in that order before any entry; the entries are checked in order, each entry's name rule, then its repetition, then its string, and the first refusal is returned; nothing is rendered until every check has passed; `deps` carries nothing and is not read
    * `[ ]`   Each `write!` into the `String` returns a `core::fmt::Result` whose error a `String` never produces; it is discarded by `let _ =`, and no branch produces it
    * `[ ]`   Invariant: the source is a function of `params` and `payload` alone

  * `[ ]`   `apps/harness-crypto/src/generate/evm/render/test.rs`
    * `[ ]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`; imports `render` from `super` and `RenderDeps`, `RenderParams`, `RenderPayload`, `RenderErrorReturn`, `SolidityEntry`, and `SolidityValue` from `super::interface`
    * `[ ]`   `params()`, a test-local function returning `RenderParams { license: "UNLICENSED".to_string(), compiler_version: "^0.8.30".to_string(), library_name: "Constants".to_string() }`, and `entry(name, value)`, a test-local function returning `SolidityEntry { name: name.to_string(), value }`; a test that alters a param does so by struct update over `params()`
    * `[ ]`   Every successful call is unpacked by `let Ok(success) = … else { panic!(…) };`, and every refusal is asserted by `matches!` over the returned value
    * `[ ]`   `a_library_opens_with_the_license_and_the_pragma_and_holds_each_entry_in_order`: contract: the rendered branch writes the opening lines from `params` and one constant per entry in the entries' order; arrange the entries `TAG` as `Bytes(vec![0x43, 0x54])`, `ORDER` as `Unsigned256` of thirty-one zero bytes followed by `0x01`, `COUNT` as `Unsigned64(18_446_744_073_709_551_615)`, `VERSION` as `Unsigned16(1)`, `HAS_G2` as `Boolean(true)`, and `FIELDS` as `String("uint256,bytes20".to_string())`; act `render(&RenderDeps, params(), RenderPayload { entries })`; assert `source` equals the lines `// SPDX-License-Identifier: UNLICENSED`, `pragma solidity ^0.8.30;`, an empty line, `library Constants {`, `    bytes internal constant TAG = hex"4354";`, `    uint256 internal constant ORDER = 0x0000000000000000000000000000000000000000000000000000000000000001;`, `    uint64 internal constant COUNT = 18446744073709551615;`, `    uint16 internal constant VERSION = 1;`, `    bool internal constant HAS_G2 = true;`, `    string internal constant FIELDS = "uint256,bytes20";`, and `}`, each ended by `\n`
    * `[ ]`   `an_empty_entry_list_renders_an_empty_library`: contract: with no entry the library is one line; act `render` over `entries: Vec::new()`; assert `source` equals the opening lines, the empty line, and `library Constants {}`, each ended by `\n`
    * `[ ]`   `an_empty_byte_string_an_empty_string_and_a_false_boolean_render`: contract: the values at each kind's edge render; arrange `EMPTY` as `Bytes(Vec::new())`, `BLANK` as `String(String::new())`, and `OFF` as `Boolean(false)`; act `render`; assert `source` holds the lines `    bytes internal constant EMPTY = hex"";`, `    string internal constant BLANK = "";`, and `    bool internal constant OFF = false;`
    * `[ ]`   `a_256_bit_value_renders_every_byte_in_order`: contract: the hex number carries the thirty-two bytes in order as sixty-four lowercase digits; arrange `MAX` as `Unsigned256` whose byte at index `i` is `0xa0 + i` for each of its thirty-two indices, built with `u8::try_from`; act `render`; assert `source` holds `0xa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebf;`
    * `[ ]`   `the_same_input_renders_the_same_source`: contract: the source is a function of its input alone; act `render` twice over equal params and entries; assert the two `source` values are equal
    * `[ ]`   `a_license_that_cannot_sit_on_its_line_is_refused`: contract: the license-refused branch; act `render` with `license` as an empty string, as `"MIT\n"`, and as `" MIT"`; assert each matches `Err(RenderErrorReturn::InvalidLicense)`
    * `[ ]`   `a_compiler_version_that_cannot_sit_in_its_pragma_is_refused`: contract: the compiler-version-refused branch; act `render` with `compiler_version` as an empty string, as `"^0.8.30;"`, and as `"^0.8.30\n"`; assert each matches `Err(RenderErrorReturn::InvalidCompilerVersion)`
    * `[ ]`   `a_library_name_that_is_not_an_identifier_is_refused`: contract: the library-name-refused branch; act `render` with `library_name` as an empty string, as `"1Constants"`, and as `"Con stants"`; assert each matches `Err(RenderErrorReturn::InvalidLibraryName)`
    * `[ ]`   `an_entry_name_that_is_not_an_identifier_is_refused_with_its_index`: contract: the entry-name-refused branch names the entry; arrange `TAG` as `Boolean(true)` followed by `BAD-NAME` as `Boolean(true)`; act `render`; assert the result matches `Err(RenderErrorReturn::InvalidEntryName { index: 1 })`
    * `[ ]`   `a_repeated_entry_name_is_refused_with_its_index`: contract: the entry-name-repeated branch names the later entry; arrange `TAG`, `ORDER`, and `TAG`, each `Boolean(true)`; act `render`; assert the result matches `Err(RenderErrorReturn::DuplicateEntryName { index: 2 })`
    * `[ ]`   `a_string_that_cannot_sit_in_a_literal_is_refused_with_its_index`: contract: the string-refused branch names the entry; act `render` over one entry `FIELDS` whose string is `uint256"`, then `uint\256`, then `uint256` followed by a newline; assert each matches `Err(RenderErrorReturn::UnrenderableString { index: 0 })`
    * `[ ]`   `the_params_are_checked_before_any_entry`: contract: the ordering, a refused param being returned ahead of a refused entry; arrange `library_name` as an empty string and one entry named `BAD-NAME`; act `render`; assert the result matches `Err(RenderErrorReturn::InvalidLibraryName)`
    * `[ ]`   Every test block carries the full `Contract`, `Arrange`, `Act`, `Assert` header and the inline markers

  * `[ ]`   `construction`
    * `[ ]`   `render` is a standalone function with no instance; its caller is `harness-crypto/generate/evm`, which passes `&RenderDeps`, forms `RenderParams` from the license identifier and compiler version it holds from construction and the name of the library it is rendering, and forms `RenderPayload` from the entries the functions it owns return
    * `[ ]`   No `mock.rs`: the function is reachable only inside the crate, it has no collaborator, a source is produced by calling it, and an entry is constructed from its two public fields

  * `[ ]`   `apps/harness-crypto/src/generate/evm/render/mod.rs`
    * `[ ]`   Module declarations: `mod interface;`, `pub(crate) mod provides;`, and `#[cfg(test)] mod test;`
    * `[ ]`   `pub fn render(_deps: &RenderDeps, params: RenderParams, payload: RenderPayload) -> RenderReturn`, realizing the branches and ordering of the interaction spec, each refusal returned before the source is begun
    * `[ ]`   The two admission rules as private functions over `&str`, each returning `bool`; the entry's line written by `match` over `SolidityValue`, every variant named and no wildcard arm
    * `[ ]`   Imports `core::fmt::Write`, `std::collections::BTreeSet`, and this module's names from `interface`; no other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`; no license, compiler version, or library name written as a literal

  * `[ ]`   `apps/harness-crypto/src/generate/evm/render/provides.rs`
    * `[ ]`   `pub(crate) use super::render;` and `pub(crate) use super::interface::*;`, nothing else, so the function and the entry type are visible to the concrete and the functions it owns and to nothing outside the crate

  * `[ ]`   `directionality`
    * `[ ]`   `render` depends on the standard library alone; nothing in the crate names it yet, and the `benchmark` module and it name nothing of each other; no new edge between crates and no cycle
    * `[ ]`   `harness-crypto/generate/evm/constants`, `group_vectors`, `rejection_vectors`, `mapping_vectors`, `possession_vectors`, `challenge_vectors`, and `proof_vectors` each return `SolidityEntry` values through this module's `provides`; `harness-crypto/generate/evm` calls `render` for the constants library and for the vectors library and writes each source; `harness-crypto/config` ships the license identifier and the compiler version, and `harness-crypto/main` passes them through `harness-crypto/generate/factory`

  * `[ ]`   `requirements`
    * `[ ]`   `apps/harness-crypto/src/lib.rs`, `apps/harness-crypto/src/generate/mod.rs`, and `apps/harness-crypto/src/generate/evm/mod.rs` carry exactly the barrel and wiring stated above; `apps/harness-crypto/Cargo.toml`, every file under `benchmark`, and every file in another crate are unchanged
    * `[ ]`   `cargo check --workspace --all-targets --all-features`, `cargo fmt --all --check`, and `cargo deny check` complete without error; `cargo clippy --workspace --all-targets --all-features` reports nothing beyond the library target's unused-item warnings for the `render` module, which `harness-crypto/generate/evm` resolves by calling `render`
    * `[ ]`   `a_library_opens_with_the_license_and_the_pragma_and_holds_each_entry_in_order`, `an_empty_entry_list_renders_an_empty_library`, `an_empty_byte_string_an_empty_string_and_a_false_boolean_render`, and `a_256_bit_value_renders_every_byte_in_order` pass (CR-09 parity inputs, one rendering for every constant and every vector)
    * `[ ]`   `the_same_input_renders_the_same_source` passes (the generated directory reproducible from the reference)
    * `[ ]`   `a_license_that_cannot_sit_on_its_line_is_refused`, `a_compiler_version_that_cannot_sit_in_its_pragma_is_refused`, `a_library_name_that_is_not_an_identifier_is_refused`, `an_entry_name_that_is_not_an_identifier_is_refused_with_its_index`, `a_repeated_entry_name_is_refused_with_its_index`, `a_string_that_cannot_sit_in_a_literal_is_refused_with_its_index`, and `the_params_are_checked_before_any_entry` pass (no source is returned whose opening lines, a malformed or repeated name, or a string would break the file)
    * `[ ]`   Every existing test in `apps/harness-crypto` passes unchanged

* `[ ]`   `harness-crypto/generate/evm/constants` **Entries of the constants library, generic over the form interface: each family's domain tags from its declaration, the delivery-statement version and the purposes' codes, the mint and transfer transcripts' field sequences as strings of ABI type names through the EVM suite's kind-to-type table, the scalar field's order, the generators' precompile encodings, and whether the verifier has second-group arithmetic**

  * `[ ]`   `objective`
    * `[ ]`   Problem: a contract recomputes every challenge and identity mapping as a hash under a domain tag reduced by the scalar field's order, encodes a transcript as its ordered fields, and branches on a purpose's code, so each of those values in Solidity must be the one the Rust reference holds; they are read from the families' declarations, the proof family's descriptions and declared purposes, and the pairing the binary resolved, never written a second time; and the encoding concrete encodes the fixed twenty-byte kind as a right-padded `bytes20`, so a transcript's identity fields are named `bytes20`, an `address` being left-padded and yielding another challenge (CR-09 parity inputs; CR-11 the tags and the modulus a contract hashes under; the dependency map's `harness-crypto/generate/evm/constants` row and its flag on an identity encoding as `bytes20`; the Delivery proof and Solidity verifier milestone's scope and exit; the technical requirements' `Constants.sol` line)
    * `[ ]`   Functional: the entries carry, as byte strings, the identity mapping's tag from the credential KEM's declaration, the two proofs of possession's tags from the key agreement's declaration, and the challenge and weight tags from the delivery proof's declaration
    * `[ ]`   Functional: the entries carry, as sixteen-bit unsigned values, the delivery-statement version the payload holds and the code of each purpose the proof family declares
    * `[ ]`   Functional: the entries carry each transcript's field sequence, read from its description over the forms in play, as one string of ABI type names separated by commas, a fixed twenty-byte kind as `bytes20`
    * `[ ]`   Functional: the entries carry the scalar field's order as a 256-bit unsigned value, read from the pairing's reference trait, and each generator's precompile encoding as a byte string
    * `[ ]`   Functional: the entries carry, as a boolean, whether the pairing's declaration states second-group arithmetic at the verifier
    * `[ ]`   Functional: the entries' names and order are fixed, so the same inputs always yield the same entries
    * `[ ]`   Non-functional: calls no factory, names no curve, concrete, chain's forms, or vendor, writes no tag, code, order, or field sequence as a literal, and touches no operating-system facility

  * `[ ]`   `role`
    * `[ ]`   App module, a function the EVM generate concrete owns; holds the EVM suite's kind-to-type table, the one place a canonical field kind is given its ABI type name
    * `[ ]`   Reads what it is handed: the pairing `harness-crypto/main` obtained through `create_pairing`, borrowed, and the declarations the binary holds from the pairing, credential KEM, key-agreement, and delivery-proof factories; the forms are the type parameter the concrete carries from `create_chain_forms`' consumer
    * `[ ]`   Does not check the statement version against the delivery proof's declaration; `proof/factory` admitted it
    * `[ ]`   Does not render or write; `harness-crypto/generate/evm` passes the entries to `render` as the constants library
    * `[ ]`   Does not produce a vector; each vector function does
    * `[ ]`   Does not carry a commit

  * `[ ]`   `module`
    * `[ ]`   Bounded context: the `constants` module beneath the `evm` concrete, holding the function's deps, params, payload, success, error, and return types, the kind-to-type table, and the function
    * `[ ]`   Creates the `apps/harness-crypto/src/generate/evm/constants` module; `apps/harness-crypto/src/generate/evm/mod.rs` gains its line and the manifest gains its dependencies
    * `[ ]`   Outside: the tags' values, the purposes, the transcripts' fields, the forms' kinds, the pairing's arithmetic and encodings, the library's name, and the Solidity that reads the constants

  * `[ ]`   `deps`
    * `[ ]`   `render`, same concrete, through `super::render::provides`: `SolidityEntry` and `SolidityValue`, as `harness-crypto/generate/evm/render` authors them; `render` names nothing in this module
    * `[ ]`   `pairing`, `adapters/pairing`, adapter ring, the crate's existing path dependency; supplies `IPairingReference`, `IPairingAdapter`, `PairingDeclaration`, `VerifierGroupArithmetic`, and the params and payload types of `scalar_field_order`, `g1_generator`, `g2_generator`, `encode_g1`, and `encode_g2`, as `adapters/pairing/src/factory/interface.rs` declares them and `pairing/bn254_arkworks` adds the reference trait
    * `[ ]`   `kem`, `adapters/kem`, adapter ring, a new path dependency; supplies `KemDeclaration`, as `kem/bb1_depth_one` authors it
    * `[ ]`   `envelope`, `adapters/envelope`, adapter ring, a new path dependency; supplies `KeyAgreementDeclaration`, as `envelope/pairing_elgamal` authors it
    * `[ ]`   `proof`, `adapters/proof`, adapter ring, a new path dependency; supplies `DeliveryProofDeclaration`, as `proof/schnorr_fs` authors it, `DELIVERY_PURPOSE_MINT`, `DELIVERY_PURPOSE_TRANSFER`, `DELIVERY_PURPOSE_GRANT`, `DELIVERY_PURPOSE_REPLACEMENT`, and `MintStatementDescription`, as `proof/mint_statement` authors them, and `TransferStatementDescription`, as `proof/transfer_statement` authors it
    * `[ ]`   `encoding`, `adapters/encoding`, adapter ring, a new path dependency; supplies `IEncodingContract` and `CanonicalFieldKind`
    * `[ ]`   `chain`, `adapters/chain`, adapter ring, a new path dependency; supplies `IChainForms`
    * `[ ]`   `pairing`, `kem`, `envelope`, `proof`, `encoding`, and `chain`, each with its `mocks` feature, as dev-dependencies, in `constants/test.rs` only: `create_pairing` with its consumer names, `build_create_pairing_params`, and `build_pairing_declaration`; `build_kem_declaration`; `build_key_agreement_declaration`; `build_delivery_proof_declaration`; `MockICanonicalField` and the canonical-field contract's names; and nothing from `chain` beyond `IChainForms`
    * `[ ]`   No external crate; no reverse dependency, none of those crates naming the harness; nothing depends on this module yet

  * `[ ]`   `context_slice`
    * `[ ]`   From `pairing`: `IPairingReference: IPairingAdapter` with `scalar_field_order(&self, ScalarFieldOrderParams, ScalarFieldOrderPayload) -> Result<ScalarFieldOrderSuccessReturn { bytes: Vec<u8> }, Infallible>`, the order's big-endian bytes; `g1_generator(&self, G1GeneratorParams, G1GeneratorPayload)` and `g2_generator(&self, G2GeneratorParams, G2GeneratorPayload)`, each returning `Ok({ point })`; `encode_g1(&self, EncodeG1Params, EncodeG1Payload { point })` and `encode_g2(&self, EncodeG2Params, EncodeG2Payload { point })`, each returning `Ok({ bytes: Vec<u8> })`; every one of these with the error arm `Infallible`; `PairingDeclaration { verifier_group_arithmetic: VerifierGroupArithmetic, .. }` with the variants `FirstGroupOnly` and `BothGroups`
    * `[ ]`   From `kem`: `KemDeclaration { identity_tag: &'static [u8], .. }`; from `envelope`: `KeyAgreementDeclaration { possession_g1_tag: &'static [u8], possession_g2_tag: &'static [u8], .. }`; from `proof`: `DeliveryProofDeclaration { challenge_tag: &'static [u8], weight_tag: &'static [u8], .. }`
    * `[ ]`   From `proof`: the four purpose constants, each a `DeliveryPurpose { code: u16, relation }`; `MintStatementDescription<F>` and `TransferStatementDescription<F>`, each implementing `IEncodingContract` for every `F: IChainForms`, whose `const FIELDS: &'static [CanonicalFieldKind]` is read as `<MintStatementDescription<F> as IEncodingContract>::FIELDS` with no instance
    * `[ ]`   From `encoding`: `CanonicalFieldKind` with the variants `FixedBytes32`, `Unsigned16`, `Unsigned32`, `Unsigned64`, `Text`, `Bytes`, `FixedBytes20`, and `Unsigned256`; and, in `constants/test.rs`, `ICanonicalField` with `type FromFieldErrorReturn;`, `const KIND: CanonicalFieldKind;`, `to_field(ToFieldParams, &Self) -> ToFieldReturn`, and `from_field(FromFieldParams, CanonicalFieldValue) -> FromFieldReturn<Self, Self::FromFieldErrorReturn>`, and `MockICanonicalField`, whose `KIND` is `Unsigned256`
    * `[ ]`   From `chain`: `IChainForms` with `type Identity`, `type Entitlement`, `type Interval`, and `type ChainIdentifier`, each bounded by `ICanonicalField + Clone`
    * `[ ]`   From `pairing`, in `constants/test.rs`: `create_pairing(&CreatePairingDeps { consumer }, CreatePairingParams, CreatePairingPayload)` returning `CreatePairingSuccessReturn { output }`; `IPairingConsumer::consume_pairing<P: IPairingArithmetic + IPairingReference>(&self, ConsumePairingParams, ConsumePairingPayload { adapter, declaration })`, as `pairing/factory` bounds it; `build_create_pairing_params(CreatePairingParamsOverrides { concrete, .. })`; `build_pairing_declaration(PairingDeclarationOverrides { verifier_group_arithmetic, .. })`
    * `[ ]`   From the declarations' mocks, in `constants/test.rs`: `build_kem_declaration(Default::default())`, `build_key_agreement_declaration(Default::default())`, and `build_delivery_proof_declaration(Default::default())`, each carrying its concrete's tags

  * `[ ]`   `apps/harness-crypto/Cargo.toml`
    * `[ ]`   `[dependencies]` reads `chain = { path = "../../adapters/chain" }`, `encoding = { path = "../../adapters/encoding" }`, `envelope = { path = "../../adapters/envelope" }`, `kem = { path = "../../adapters/kem" }`, `pairing = { path = "../../adapters/pairing" }`, `proof = { path = "../../adapters/proof" }`, and `random = { path = "../../adapters/random" }`
    * `[ ]`   `[dev-dependencies]` reads the same crates, each with `features = ["mocks"]`
    * `[ ]`   `[package]`, `[features]`, and `[lints]` are unchanged; no other table

  * `[ ]`   `apps/harness-crypto/src/generate/evm/mod.rs`
    * `[ ]`   Module wiring only: `mod constants;` and `mod render;`, nothing else
    * `[ ]`   Until `constants/mod.rs` exists, `cargo check` reports the unresolved `mod constants`, which is the RED state for every element below that precedes it

  * `[ ]`   `apps/harness-crypto/src/generate/evm/constants/interface.rs`
    * `[ ]`   `ConstantsDeps<'a, P: IPairingReference>`, a struct with `pub pairing: &'a P`, the pairing the binary resolved
    * `[ ]`   `ConstantsParams`, the fieldless struct `pub struct ConstantsParams;`
    * `[ ]`   `ConstantsPayload<'a>`, a struct with `pub pairing_declaration: &'a PairingDeclaration`, `pub kem_declaration: &'a KemDeclaration`, `pub key_agreement_declaration: &'a KeyAgreementDeclaration`, `pub delivery_proof_declaration: &'a DeliveryProofDeclaration`, and `pub statement_version: u16`
    * `[ ]`   `ConstantsSuccessReturn`, a struct with `pub entries: Vec<SolidityEntry>`
    * `[ ]`   `ConstantsErrorReturn`, an enum with the one variant `ScalarFieldOrderLength { actual: usize }`
    * `[ ]`   `ConstantsReturn`, the alias `Result<ConstantsSuccessReturn, ConstantsErrorReturn>`
    * `[ ]`   No derives on any type in this file; no `ConstantsFn` alias, since nothing injects the function and its signature is the one in `mod.rs`; imports `IPairingReference` and `PairingDeclaration` from `pairing`, `KemDeclaration` from `kem`, `KeyAgreementDeclaration` from `envelope`, `DeliveryProofDeclaration` from `proof`, and `SolidityEntry` from `super::super::render::provides`; declares nothing else

  * `[ ]`   `apps/harness-crypto/src/generate/evm/constants/interaction.spec.md`
    * `[ ]`   `constants<P: IPairingReference, F: IChainForms>(deps: &ConstantsDeps<'_, P>, params: ConstantsParams, payload: ConstantsPayload<'_>) -> ConstantsReturn`, the trusted form: every value arrives typed from the binary's composition; `F` is named by the caller and appears in no argument
    * `[ ]`   The kind-to-type table, `abi_type_name(kind: CanonicalFieldKind) -> &'static str`: `FixedBytes32` is `bytes32`, `Unsigned16` is `uint16`, `Unsigned32` is `uint32`, `Unsigned64` is `uint64`, `Text` is `string`, `Bytes` is `bytes`, `FixedBytes20` is `bytes20`, and `Unsigned256` is `uint256`; every variant is named and there is no wildcard arm
    * `[ ]`   `field_sequence(kinds: &[CanonicalFieldKind]) -> String`: each kind's name through `abi_type_name`, in order, joined by `,` with no space; an empty slice yields an empty string
    * `[ ]`   Order refused: condition `deps.pairing.scalar_field_order(ScalarFieldOrderParams, ScalarFieldOrderPayload)`, unpacked irrefutably, returns `bytes` that do not convert into `[u8; 32]`; decision `<[u8; 32]>::try_from(bytes)`, which returns the vector on failure; dependency call `scalar_field_order`, once, first; outcome `Err(ConstantsErrorReturn::ScalarFieldOrderLength { actual })`, `actual` the returned vector's length; every pairing concrete returns thirty-two bytes, so no admitted concrete takes this branch and it has no unit test
    * `[ ]`   Gathered: condition the order converts; dependency calls, each once and unpacked irrefutably, `g1_generator(G1GeneratorParams, G1GeneratorPayload)` then `encode_g1(EncodeG1Params, EncodeG1Payload { point })`, and `g2_generator(G2GeneratorParams, G2GeneratorPayload)` then `encode_g2(EncodeG2Params, EncodeG2Payload { point })`; outcome `Ok(ConstantsSuccessReturn { entries })`, the entries, in this order, by name and value:
    * `[ ]`   `IDENTITY_TAG`, `Bytes` of `payload.kem_declaration.identity_tag`; `POSSESSION_G1_TAG` and `POSSESSION_G2_TAG`, `Bytes` of `payload.key_agreement_declaration.possession_g1_tag` and `possession_g2_tag`; `CHALLENGE_TAG` and `WEIGHT_TAG`, `Bytes` of `payload.delivery_proof_declaration.challenge_tag` and `weight_tag`, each tag copied by `to_vec()`
    * `[ ]`   `STATEMENT_VERSION`, `Unsigned16` of `payload.statement_version`; `PURPOSE_MINT`, `PURPOSE_TRANSFER`, `PURPOSE_GRANT`, and `PURPOSE_REPLACEMENT`, `Unsigned16` of the `code` of `DELIVERY_PURPOSE_MINT`, `DELIVERY_PURPOSE_TRANSFER`, `DELIVERY_PURPOSE_GRANT`, and `DELIVERY_PURPOSE_REPLACEMENT`
    * `[ ]`   `MINT_FIELDS`, `String` of `field_sequence(<MintStatementDescription<F> as IEncodingContract>::FIELDS)`; `TRANSFER_FIELDS`, `String` of `field_sequence(<TransferStatementDescription<F> as IEncodingContract>::FIELDS)`
    * `[ ]`   `SCALAR_FIELD_ORDER`, `Unsigned256` of the converted order; `G1_GENERATOR` and `G2_GENERATOR`, `Bytes` of the two encodings
    * `[ ]`   `HAS_G2_ARITHMETIC`, `Boolean` of `matches!(payload.pairing_declaration.verifier_group_arithmetic, VerifierGroupArithmetic::BothGroups)`
    * `[ ]`   Ordering: the order is read and converted before any other call; the generators follow; the entries are assembled last; `params` carries nothing and is not read; nothing the function is handed is changed
    * `[ ]`   Invariant: every entry's value is read from a declaration, a description, a declared purpose, the payload, or the pairing; the entries are a function of the pairing, the forms, and the payload alone

  * `[ ]`   `apps/harness-crypto/src/generate/evm/constants/test.rs`
    * `[ ]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`; imports `constants` and `field_sequence` from `super`; `ConstantsDeps`, `ConstantsParams`, and `ConstantsPayload` from `super::interface`; `SolidityEntry` and `SolidityValue` from `super::super::render::provides`; the pairing names the context slice lists, `PairingConcrete`, `PairingDeclarationOverrides`, `CreatePairingParamsOverrides`, and `VerifierGroupArithmetic` from `pairing`; `build_kem_declaration` from `kem`; `build_key_agreement_declaration` from `envelope`; `build_delivery_proof_declaration` from `proof`; `CanonicalFieldKind`, `CanonicalFieldValue`, `ICanonicalField`, `MockICanonicalField`, and the canonical-field contract's params, success, and return types from `encoding`; and `IChainForms` from `chain`
    * `[ ]`   `TwentyByteIdentity`, a test-local struct over `[u8; 20]` with `#[derive(Clone)]`, implementing `ICanonicalField` with `type FromFieldErrorReturn = ();`, `const KIND: CanonicalFieldKind = CanonicalFieldKind::FixedBytes20;`, `to_field` returning the bytes as `CanonicalFieldValue::FixedBytes20`, and `from_field` admitting that variant and refusing every other with `Err(())`; `TestForms`, a test-local unit struct implementing `IChainForms` with `type Identity = TwentyByteIdentity;` and `MockICanonicalField` for the entitlement, interval, and chain-identifier forms
    * `[ ]`   `ConstantsProbe`, a test-local unit struct implementing `IPairingConsumer` with `Output = ConstantsObserved`, whose `consume_pairing<P: IPairingArithmetic + IPairingReference>` builds the three family declarations by their builders called with `Default::default()`, calls `constants::<P, TestForms>(&ConstantsDeps { pairing: &payload.adapter }, ConstantsParams, ConstantsPayload { … })` with `statement_version: 7` once over `payload.declaration`, once over `build_pairing_declaration` with `verifier_group_arithmetic` overridden by `BothGroups`, and once with it overridden by `FirstGroupOnly`, and reads the order, the two generators, and their encodings from `payload.adapter` through the family's traits; every call is unpacked by `let Ok(…) = … else { panic!(…) };`
    * `[ ]`   `ConstantsObserved`, a test-local struct with `entries: Vec<SolidityEntry>`, the entries of the first call; `both_groups: Vec<SolidityEntry>` and `first_group_only: Vec<SolidityEntry>`, the entries of the other two; `declared_both_groups: bool`, whether `payload.declaration` states `BothGroups`; `order: Vec<u8>`, `g1_generator: Vec<u8>`, and `g2_generator: Vec<u8>`, as read from the adapter; and the tags of the three built declarations as `Vec<u8>` fields
    * `[ ]`   `value(entries, name)`, a test-local function returning the `&SolidityValue` of the entry with that name and panicking when no entry carries it; each test runs `ConstantsProbe` through `create_pairing(&CreatePairingDeps { consumer: ConstantsProbe }, build_create_pairing_params(CreatePairingParamsOverrides { concrete: Some(…), ..Default::default() }), CreatePairingPayload)` and unpacks `success.output`, on `PairingConcrete::Bls12381Arkworks` unless its name states another concrete; a value's variant is unpacked by `let SolidityValue::… = … else { panic!(…) };`
    * `[ ]`   `the_entries_carry_the_fixed_names_in_the_fixed_order`: contract: the gathered branch returns the entries the interaction spec names, in its order; assert the names of `entries` equal `IDENTITY_TAG`, `POSSESSION_G1_TAG`, `POSSESSION_G2_TAG`, `CHALLENGE_TAG`, `WEIGHT_TAG`, `STATEMENT_VERSION`, `PURPOSE_MINT`, `PURPOSE_TRANSFER`, `PURPOSE_GRANT`, `PURPOSE_REPLACEMENT`, `MINT_FIELDS`, `TRANSFER_FIELDS`, `SCALAR_FIELD_ORDER`, `G1_GENERATOR`, `G2_GENERATOR`, and `HAS_G2_ARITHMETIC`, in that order
    * `[ ]`   `each_domain_tag_is_the_bytes_its_declaration_carries`: contract: a tag is mirrored from its family's declaration (CR-11); assert each of the five tag entries is `Bytes` equal to the tag the probe recorded from the built declaration, and none is empty
    * `[ ]`   `the_statement_version_is_the_one_handed_and_the_purposes_carry_their_declared_codes`: contract: the version is the payload's and the codes are the proof family's (CR-09); assert `STATEMENT_VERSION` is `Unsigned16(7)` and the four purpose entries are `Unsigned16` of `1`, `2`, `3`, and `4`
    * `[ ]`   `the_field_sequences_name_an_identity_as_bytes20`: contract: each transcript's kinds are rendered through the table over the forms in play, an identity as `bytes20` (CR-09; the milestone's exit); assert `MINT_FIELDS` is `String` equal to `bytes32,uint16,uint256,bytes20,bytes32,bytes32,uint256,uint256,uint256,uint256,uint16,bytes20,bytes20,bytes,bytes,bytes,bytes,bytes,bytes,uint64,bytes,bytes,bytes,bytes,bytes`, and `TRANSFER_FIELDS` is `String` equal to the same first thirteen names followed by `bytes` twelve times, `uint64`, and `bytes` six times, written out in full in the test; and neither holds `address`
    * `[ ]`   `every_kind_has_its_abi_type_name`: contract: the kind-to-type table covers every canonical field kind; act `field_sequence` over the eight kinds in the enum's order; assert the result is `bytes32,uint16,uint32,uint64,string,bytes,bytes20,uint256`, and `field_sequence(&[])` is empty
    * `[ ]`   `the_order_and_the_generators_are_the_pairings_own`: contract: the modulus and the generators are read from the resolved pairing (CR-11); assert `SCALAR_FIELD_ORDER` is `Unsigned256` whose bytes equal `order`, and `G1_GENERATOR` and `G2_GENERATOR` are `Bytes` equal to `g1_generator` and `g2_generator`, each non-empty
    * `[ ]`   `the_order_and_the_generators_are_the_pairings_own_on_bn254_arkworks`: the same over `PairingConcrete::Bn254Arkworks`
    * `[ ]`   `the_verifier_form_is_read_from_the_pairings_declaration`: contract: the boolean states what the declaration states; assert `HAS_G2_ARITHMETIC` is `Boolean(true)` in `both_groups`, `Boolean(false)` in `first_group_only`, and `Boolean(declared_both_groups)` in `entries`
    * `[ ]`   Every test block carries the full `Contract`, `Arrange`, `Act`, `Assert` header and the inline markers; the order-refused branch has no unit test, as the interaction spec states

  * `[ ]`   `construction`
    * `[ ]`   `constants` is a standalone function with no instance; its caller is `harness-crypto/generate/evm`, which forms `ConstantsDeps` from its borrowed pairing and `ConstantsPayload` from the declarations and the statement version it holds from construction, and names its own forms parameter as `F`
    * `[ ]`   No `mock.rs`: the function is reachable only inside the crate, its collaborator is mocked by the pairing family, and entries are produced by calling it

  * `[ ]`   `apps/harness-crypto/src/generate/evm/constants/mod.rs`
    * `[ ]`   Module declarations: `mod interface;`, `pub(crate) mod provides;`, and `#[cfg(test)] mod test;`
    * `[ ]`   `pub fn constants<P: IPairingReference, F: IChainForms>(deps: &ConstantsDeps<'_, P>, _params: ConstantsParams, payload: ConstantsPayload<'_>) -> ConstantsReturn`, realizing the branches and ordering of the interaction spec, the order-refused branch returned by `let … else`
    * `[ ]`   `fn abi_type_name(kind: CanonicalFieldKind) -> &'static str` and `fn field_sequence(kinds: &[CanonicalFieldKind]) -> String`, private, as the interaction spec states them
    * `[ ]`   Imports the pairing, declaration, purpose, description, encoding, and forms names the interaction spec uses from `pairing`, `kem`, `envelope`, `proof`, `encoding`, and `chain`, `SolidityEntry` and `SolidityValue` from `super::render::provides`, and this module's names from `interface`; no other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`; no tag, code, order, or encoding written as a literal

  * `[ ]`   `apps/harness-crypto/src/generate/evm/constants/provides.rs`
    * `[ ]`   `pub(crate) use super::constants;` and `pub(crate) use super::interface::*;`, nothing else

  * `[ ]`   `directionality`
    * `[ ]`   `constants` depends on `render`'s entry type through its `provides`, on `pairing`'s reference trait and declaration, on the `kem`, `envelope`, and `proof` declarations, on `proof`'s purposes and descriptions, on `encoding`'s contract and kinds, and on `chain`'s form interface; `render` names nothing here; the harness now depends on `adapters/chain`, `adapters/encoding`, `adapters/envelope`, `adapters/kem`, and `adapters/proof` beside `adapters/pairing` and `adapters/random`, none of which names it; no cycle
    * `[ ]`   `harness-crypto/generate/evm` calls `constants` and renders its entries as the constants library; `contracts/evm/PairingLib` reads `SCALAR_FIELD_ORDER`, the generators, and `HAS_G2_ARITHMETIC`, and `contracts/evm/DeliveryVerifier` reads the tags, the version, the purposes' codes, and the field sequences, encoding every identity field as `bytes20`

  * `[ ]`   `requirements`
    * `[ ]`   `apps/harness-crypto/Cargo.toml` and `apps/harness-crypto/src/generate/evm/mod.rs` carry exactly the tables, keys, and wiring stated above; nothing under `render` or `benchmark`, and no file in another crate, changes
    * `[ ]`   `cargo check --workspace --all-targets --all-features`, `cargo fmt --all --check`, and `cargo deny check` complete without error; `cargo clippy --workspace --all-targets --all-features` reports nothing beyond the library target's unused-item warnings for the `render` and `constants` modules, which `harness-crypto/generate/evm` resolves by calling them
    * `[ ]`   `the_entries_carry_the_fixed_names_in_the_fixed_order` passes (CR-09 parity inputs, one constants library per curve with the same names)
    * `[ ]`   `each_domain_tag_is_the_bytes_its_declaration_carries`, `the_order_and_the_generators_are_the_pairings_own`, and `the_order_and_the_generators_are_the_pairings_own_on_bn254_arkworks` pass (CR-11, the tags and the modulus a contract hashes under, on each curve)
    * `[ ]`   `the_statement_version_is_the_one_handed_and_the_purposes_carry_their_declared_codes`, `the_field_sequences_name_an_identity_as_bytes20`, and `every_kind_has_its_abi_type_name` pass (CR-09, the transcript a contract encodes, every identity field as `bytes20`)
    * `[ ]`   `the_verifier_form_is_read_from_the_pairings_declaration` passes (the form each curve's libraries are generated in)
    * `[ ]`   Every existing test in `apps/harness-crypto` passes unchanged

* `[ ]`   `harness-crypto/generate/evm/group_vectors` **Entries of the pairing library's group-operation and pairing-product vectors over scalars drawn through the randomness family, the configured count of each kind and each vector carrying its draws: first-group addition and multiplication and the pairing-product check for every pairing, and second-group addition and both groups' multi-scalar multiplication where the pairing declares second-group arithmetic at the verifier**

  * `[ ]`   `objective`
    * `[ ]`   Problem: the Solidity pairing library calls the precompiles the Rust reference computes in software, and it is proven against the reference only on inputs and results the reference produced; the operations a curve's verifier has differ, a verifier with first-group arithmetic only having no second-group addition and no multi-scalar multiplication, so the vectors a curve's library is tested on are the ones its pairing's declaration admits; and a vector is reproducible only if it carries the values drawn to produce it (CR-10 on-chain vectors; the dependency map's `harness-crypto/generate/evm/group_vectors` row; the Delivery proof and Solidity verifier milestone's scope; the technical requirements' `Vectors.sol` line)
    * `[ ]`   Functional: for every pairing, the entries carry the count `params` holds of first-group addition vectors, of first-group multiplication vectors, and of pairing-product vectors
    * `[ ]`   Functional: where the pairing's declaration states second-group arithmetic at the verifier, the entries also carry that count of second-group addition vectors and of each group's multi-scalar multiplication vectors, and otherwise none of them
    * `[ ]`   Functional: every element is its precompile encoding and every scalar its canonical encoding, each a byte string, and every vector carries the scalars drawn to produce it
    * `[ ]`   Functional: a pairing-product vector carries a product that is one and the same product with its balancing element un-negated, each with the verdict the reference's own check returns
    * `[ ]`   Functional: a refused draw and a refused sampling are each returned unchanged in the function's own error, and no entry is returned
    * `[ ]`   Non-functional: every draw passes through `IRandomSourceAdapter` and every operation through the pairing family's traits; calls no factory, names no curve or library, and writes no scalar, element, or verdict as a literal

  * `[ ]`   `role`
    * `[ ]`   App module, a function the EVM generate concrete owns
    * `[ ]`   Reads what it is handed: the borrowed pairing and randomness source `harness-crypto/main` obtained through their factories, and the pairing's declaration
    * `[ ]`   Does not decide a curve's verifier form; the pairing's declaration states it
    * `[ ]`   Does not produce a decode-rejection, hash-to-scalar, identity-mapping, possession, challenge, or proof vector; each is another function's
    * `[ ]`   Does not render or write; `harness-crypto/generate/evm` joins these entries with the other vector functions' and passes them to `render` as the vectors library
    * `[ ]`   Does not carry a commit

  * `[ ]`   `module`
    * `[ ]`   Bounded context: the `group_vectors` module beneath the `evm` concrete, holding the function's deps, params, payload, success, error, and return types and the function
    * `[ ]`   Creates the `apps/harness-crypto/src/generate/evm/group_vectors` module; `apps/harness-crypto/src/generate/evm/mod.rs` gains its line; the manifest is unchanged
    * `[ ]`   Outside: the group arithmetic, the encodings, the source of randomness, the vector count's value, the library's name, and the Solidity tests that read the vectors

  * `[ ]`   `deps`
    * `[ ]`   `render`, same concrete, through `super::render::provides`: `SolidityEntry` and `SolidityValue`; `render` names nothing in this module
    * `[ ]`   `pairing`, `adapters/pairing`, adapter ring, the crate's existing path dependency; supplies `IPairingArithmetic`, `IPairingAdapter`, `ISampleUniformScalar`, `PairingDeclaration`, `VerifierGroupArithmetic`, `SampleUniformScalarErrorReturn`, and the params, payload, and term types of the generator, addition, multiplication, multi-scalar multiplication, negation, scalar multiplication, pairing-product check, and encoding methods, as `adapters/pairing/src/factory/interface.rs` declares them
    * `[ ]`   `random`, `adapters/random`, adapter ring, the crate's existing path dependency; supplies `IRandomSourceAdapter`, `FillBytesParams`, `FillBytesPayload`, and `FillBytesErrorReturn`, as `adapters/random/src/factory/interface.rs` declares them
    * `[ ]`   `core::num::NonZeroU32`, standard library, the vector count
    * `[ ]`   `pairing` and `random`, each with its `mocks` feature, the crate's existing dev-dependencies, in `group_vectors/test.rs` only: `create_pairing` with its consumer names, `build_create_pairing_params`, and `build_pairing_declaration`; `create_random_source`, `build_create_random_source_params`, and `MockIRandomSourceAdapter`
    * `[ ]`   No new dependency; no reverse dependency; nothing depends on this module yet

  * `[ ]`   `context_slice`
    * `[ ]`   From `random`: `IRandomSourceAdapter::fill_bytes(&self, FillBytesParams, FillBytesPayload { length: usize }) -> Result<FillBytesSuccessReturn { bytes: Secret<Vec<u8>> }, FillBytesErrorReturn>`
    * `[ ]`   From `pairing`, the scalar: `ISampleUniformScalar::UNIFORM_BYTES_LENGTH` and `sample_from_uniform_bytes(SampleUniformScalarParams, SampleUniformScalarPayload { uniform: Secret<Vec<u8>> }) -> Result<SampleUniformScalarSuccessReturn { scalar: Secret<S> }, SampleUniformScalarErrorReturn>`, the scalar read by `expose().clone()`; `encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar })` returning `Ok({ bytes: Secret<Vec<u8>> })`, read by `expose().clone()`; `mul_scalar(MulScalarParams, MulScalarPayload { left, right })` returning `Ok({ product })`
    * `[ ]`   From `pairing`, the groups: `g1_generator` and `g2_generator`, each returning `Ok({ point })`; `add_g1(AddG1Params, AddG1Payload { left, right })` and `add_g2(AddG2Params, AddG2Payload { left, right })`, each returning `Ok({ sum })`; `mul_g1(MulG1Params, MulG1Payload { point, scalar })` and `mul_g2(MulG2Params, MulG2Payload { point, scalar })`, each returning `Ok({ product })`; `msm_g1(MsmG1Params, MsmG1Payload { terms: Vec<MsmG1Term { base, scalar }> })` and `msm_g2` over `MsmG2Term`, each returning `Ok({ sum })`; `neg_g1(NegG1Params, NegG1Payload { point })` returning `Ok({ negation })`; `pairing_product_is_one(PairingProductIsOneParams, PairingProductIsOnePayload { terms: Vec<PairingProductTerm { g1, g2 }> })` returning `Ok({ is_one })`; `encode_g1(EncodeG1Params, EncodeG1Payload { point })` and `encode_g2(EncodeG2Params, EncodeG2Payload { point })`, each returning `Ok({ bytes: Vec<u8> })`; every one of these with the error arm `Infallible`; `G1`, `G2`, and `Scalar` each `Clone`
    * `[ ]`   From `pairing`: `PairingDeclaration { verifier_group_arithmetic, .. }` with the variants `FirstGroupOnly` and `BothGroups`
    * `[ ]`   From `pairing`, in `group_vectors/test.rs`: `decode_scalar(DecodeScalarParams, &[u8])`, `decode_g1(DecodeG1Params, &[u8])`, and `decode_g2(DecodeG2Params, &[u8])`, each returning its decoded value or a typed refusal; `create_pairing`, `IPairingConsumer::consume_pairing<P: IPairingArithmetic + IPairingReference>`, `build_create_pairing_params(CreatePairingParamsOverrides { concrete, .. })`, and `build_pairing_declaration(PairingDeclarationOverrides { verifier_group_arithmetic, .. })`
    * `[ ]`   From `random`, in `group_vectors/test.rs`: `create_random_source(&CreateRandomSourceDeps, build_create_random_source_params(CreateRandomSourceParamsOverrides { kind: Some(RandomSourceKind::OperatingSystem) }), CreateRandomSourcePayload)`, unpacked irrefutably, returning `{ adapter: Box<dyn IRandomSourceAdapter>, .. }`; `MockIRandomSourceAdapter`, whose draw is empty

  * `[ ]`   `apps/harness-crypto/src/generate/evm/mod.rs`
    * `[ ]`   Module wiring only: `mod constants;`, `mod group_vectors;`, and `mod render;`, nothing else
    * `[ ]`   Until `group_vectors/mod.rs` exists, `cargo check` reports the unresolved `mod group_vectors`, which is the RED state for every element below that precedes it

  * `[ ]`   `apps/harness-crypto/src/generate/evm/group_vectors/interface.rs`
    * `[ ]`   `GroupVectorsDeps<'a, P: IPairingArithmetic>`, a struct with `pub pairing: &'a P` and `pub random: &'a dyn IRandomSourceAdapter`
    * `[ ]`   `GroupVectorsParams`, a struct with `pub count: NonZeroU32`, the number of vectors of each kind
    * `[ ]`   `GroupVectorsPayload<'a>`, a struct with `pub pairing_declaration: &'a PairingDeclaration`
    * `[ ]`   `GroupVectorsSuccessReturn`, a struct with `pub entries: Vec<SolidityEntry>`
    * `[ ]`   `GroupVectorsErrorReturn`, an enum with the variants `FillBytes(FillBytesErrorReturn)` and `SampleScalar(SampleUniformScalarErrorReturn)`
    * `[ ]`   `GroupVectorsReturn`, the alias `Result<GroupVectorsSuccessReturn, GroupVectorsErrorReturn>`
    * `[ ]`   No derives on any type in this file; no `GroupVectorsFn` alias, since nothing injects the function; imports `IPairingArithmetic`, `PairingDeclaration`, and `SampleUniformScalarErrorReturn` from `pairing`, `IRandomSourceAdapter` and `FillBytesErrorReturn` from `random`, `core::num::NonZeroU32`, and `SolidityEntry` from `super::super::render::provides`; declares nothing else

  * `[ ]`   `apps/harness-crypto/src/generate/evm/group_vectors/interaction.spec.md`
    * `[ ]`   `group_vectors<P: IPairingArithmetic>(deps: &GroupVectorsDeps<'_, P>, params: GroupVectorsParams, payload: GroupVectorsPayload<'_>) -> GroupVectorsReturn`, the trusted form
    * `[ ]`   A draw, stated once and used by every vector: `deps.random.fill_bytes(FillBytesParams, FillBytesPayload { length: P::Scalar::UNIFORM_BYTES_LENGTH })`, then `P::Scalar::sample_from_uniform_bytes(SampleUniformScalarParams, SampleUniformScalarPayload { uniform: draw.bytes })`, yielding the scalar
    * `[ ]`   Draw refused: condition `fill_bytes` returns `Err(error)` for any draw; decision the `Err` arm; outcome `Err(GroupVectorsErrorReturn::FillBytes(error))`, the refusal unchanged, and no entry
    * `[ ]`   Sampling refused: condition `sample_from_uniform_bytes` returns `Err(error)` for any draw; decision the `Err` arm; outcome `Err(GroupVectorsErrorReturn::SampleScalar(error))`, the refusal unchanged, and no entry
    * `[ ]`   Gathered: condition every draw is sampled; outcome `Ok(GroupVectorsSuccessReturn { entries })`; the generators `g1` and `g2` are read once, first; each kind's vectors follow in the order below, a kind's vectors in ascending `index` from `0` to `params.count.get() - 1`; an entry is named `{KIND}_{index}_{FIELD}`, `index` in decimal; every element entry is `Bytes` of its `encode_g1` or `encode_g2`, every scalar entry `Bytes` of its `encode_scalar` read by `expose().clone()`, and every verdict entry `Boolean`; every pairing call is unpacked irrefutably
    * `[ ]`   `G1_ADD`, every pairing: draws `a` and `b`; `left = mul_g1(g1, a)`, `right = mul_g1(g1, b)`, `sum = add_g1(left, right)`; entries `A_SCALAR`, `B_SCALAR`, `A`, `B`, and `SUM`
    * `[ ]`   `G1_MUL`, every pairing: draws `a` and `s`; `base = mul_g1(g1, a)`, `product = mul_g1(base, s)`; entries `BASE_SCALAR`, `SCALAR`, `BASE`, and `PRODUCT`
    * `[ ]`   `PAIRING`, every pairing: draws `a` and `b`; `left = mul_g1(g1, a)`, `right = mul_g2(g2, b)`, `imbalance = mul_g1(g1, mul_scalar(a, b))`, `balance = neg_g1(imbalance)`; `is_one = pairing_product_is_one` over the terms `(left, right)` and `(balance, g2)`, and `imbalanced_is_one = pairing_product_is_one` over the terms `(left, right)` and `(imbalance, g2)`; entries `A_SCALAR`, `B_SCALAR`, `G1_A`, `G2_B`, `G1_BALANCE`, `IS_ONE`, `G1_IMBALANCE`, and `IMBALANCED_IS_ONE`; the second term's second-group element is the generator the constants library carries
    * `[ ]`   Second-group arithmetic: condition `matches!(payload.pairing_declaration.verifier_group_arithmetic, VerifierGroupArithmetic::BothGroups)`; when it holds the three kinds below follow, and otherwise no entry of them is returned and no draw is made for them
    * `[ ]`   `G2_ADD`: draws `a` and `b`; `left = mul_g2(g2, a)`, `right = mul_g2(g2, b)`, `sum = add_g2(left, right)`; entries `A_SCALAR`, `B_SCALAR`, `A`, `B`, and `SUM`
    * `[ ]`   `G1_MSM`: draws `a`, `b`, `s`, and `t`; `base_0 = mul_g1(g1, a)`, `base_1 = mul_g1(g1, b)`, `sum = msm_g1` over the terms `(base_0, s)` and `(base_1, t)`; entries `BASE_0_SCALAR`, `BASE_1_SCALAR`, `SCALAR_0`, `SCALAR_1`, `BASE_0`, `BASE_1`, and `SUM`
    * `[ ]`   `G2_MSM`: the same over `mul_g2`, `g2`, and `msm_g2`, with the same entries
    * `[ ]`   Ordering: a vector's draws are made in the order its line names them, before any of its operations; the first refusal is returned and the entries gathered so far are dropped; `payload.pairing_declaration` is read for its verifier form alone
    * `[ ]`   Invariant: every element and verdict entry of a vector is the pairing's own result over the scalars the vector's scalar entries carry, so a vector is reproduced from its entries alone

  * `[ ]`   `apps/harness-crypto/src/generate/evm/group_vectors/test.rs`
    * `[ ]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`; imports `group_vectors` from `super`; `GroupVectorsDeps`, `GroupVectorsParams`, `GroupVectorsPayload`, and `GroupVectorsErrorReturn` from `super::interface`; `SolidityEntry` and `SolidityValue` from `super::super::render::provides`; `core::num::NonZeroU32`; and the `pairing` and `random` names the context slice lists
    * `[ ]`   `GroupVectorsProbe`, a test-local struct with `random: Box<dyn IRandomSourceAdapter>`, implementing `IPairingConsumer` with `Output = Result<GroupVectorsObserved, GroupVectorsErrorReturn>`, whose `consume_pairing<P: IPairingArithmetic + IPairingReference>` calls `group_vectors(&GroupVectorsDeps { pairing: &payload.adapter, random: self.random.as_ref() }, GroupVectorsParams { count }, GroupVectorsPayload { pairing_declaration: &declaration })` with `count` three, once over `build_pairing_declaration` with `verifier_group_arithmetic` overridden by `BothGroups` and once with it overridden by `FirstGroupOnly`, returning the first `Err` by `?`
    * `[ ]`   The probe reproduces each vector of the `BothGroups` call through `payload.adapter` alone: it reads a vector's scalar entries through `decode_scalar` and its element entries through `decode_g1` and `decode_g2`, recomputes the vector's elements and verdicts from the decoded scalars by the operations the interaction spec states, and compares each recomputed element's `encode_g1` or `encode_g2` with the entry's bytes; every call is unpacked by `let Ok(…) = … else { panic!(…) };`, an entry is found by name through a test-local `value(entries, name)` that panics when no entry carries it, and a value's variant is unpacked by `let SolidityValue::… = … else { panic!(…) };`
    * `[ ]`   `GroupVectorsObserved`, a test-local struct with `both_groups_names: Vec<String>` and `first_group_only_names: Vec<String>`, the entries' names in order; the `bool` fields `g1_add_reproduced`, `g1_mul_reproduced`, `pairing_reproduced`, `g2_add_reproduced`, `g1_msm_reproduced`, and `g2_msm_reproduced`, each true when every vector of its kind is reproduced; `balanced_products_are_one` and `imbalanced_products_are_not_one`, over every pairing vector's two verdict entries; and `draws_differ`, true when the `A_SCALAR` entries of the first-group addition vectors at index `0` and index `1` differ
    * `[ ]`   Each test builds the probe over the operating-system source from `create_random_source` unless it states another, runs it through `create_pairing(&CreatePairingDeps { consumer: probe }, build_create_pairing_params(CreatePairingParamsOverrides { concrete: Some(…), ..Default::default() }), CreatePairingPayload)`, and unpacks `success.output`; every test runs on `PairingConcrete::Bls12381Arkworks` unless its name states another concrete
    * `[ ]`   `each_kind_is_present_the_counted_number_of_times_in_order`: contract: the gathered branch returns the count of each kind, a kind's vectors in ascending index and the kinds in the stated order (CR-10); assert `both_groups_names` equals the names the interaction spec states for `G1_ADD`, `G1_MUL`, `PAIRING`, `G2_ADD`, `G1_MSM`, and `G2_MSM` at indices `0`, `1`, and `2`, built in the test by the same `{KIND}_{index}_{FIELD}` form from the stated fields, and holds no name at index `3`
    * `[ ]`   `a_first_group_only_verifier_gets_no_second_group_or_multi_scalar_vector`: contract: the second-group-arithmetic condition (CR-10, each curve's library tested on the operations its verifier has); assert `first_group_only_names` equals the `G1_ADD`, `G1_MUL`, and `PAIRING` names alone and no name in it begins with `G2_ADD`, `G1_MSM`, or `G2_MSM`
    * `[ ]`   `every_first_group_vector_is_reproduced_from_the_draws_it_carries`: contract: the invariant for `G1_ADD` and `G1_MUL`; assert `g1_add_reproduced` and `g1_mul_reproduced`
    * `[ ]`   `every_pairing_vector_is_reproduced_and_carries_both_verdicts`: contract: the invariant for `PAIRING`, a balanced product being one and its imbalanced form not; assert `pairing_reproduced`, `balanced_products_are_one`, and `imbalanced_products_are_not_one`
    * `[ ]`   `every_second_group_and_multi_scalar_vector_is_reproduced_from_the_draws_it_carries`: contract: the invariant for `G2_ADD`, `G1_MSM`, and `G2_MSM`; assert `g2_add_reproduced`, `g1_msm_reproduced`, and `g2_msm_reproduced`
    * `[ ]`   `every_vector_is_reproduced_on_bn254_arkworks`: contract: the invariant holds on the other curve; arrange `PairingConcrete::Bn254Arkworks`; assert every `…_reproduced` field, `balanced_products_are_one`, and `imbalanced_products_are_not_one`
    * `[ ]`   `vectors_of_one_kind_carry_different_draws`: contract: each vector makes its own draws; assert `draws_differ`
    * `[ ]`   `a_draw_the_sampling_refuses_is_returned_and_no_entry_is`: contract: the sampling-refused branch; arrange the probe over `Box::new(MockIRandomSourceAdapter)` and `PairingConcrete::Bn254Arkworks`; assert the output matches `Err(GroupVectorsErrorReturn::SampleScalar(SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual: 0 }))`
    * `[ ]`   Every test block carries the full `Contract`, `Arrange`, `Act`, `Assert` header and the inline markers; the draw-refused branch is fixed by the return union and the `match` and has no unit test, as the benchmark's has none

  * `[ ]`   `construction`
    * `[ ]`   `group_vectors` is a standalone function with no instance; its caller is `harness-crypto/generate/evm`, which forms `GroupVectorsDeps` from its borrowed pairing and randomness source, `GroupVectorsParams` from the vector count it holds from construction, and `GroupVectorsPayload` from the pairing's declaration
    * `[ ]`   No `mock.rs`: the function is reachable only inside the crate, its collaborators are mocked by their own families, and entries are produced by calling it

  * `[ ]`   `apps/harness-crypto/src/generate/evm/group_vectors/mod.rs`
    * `[ ]`   Module declarations: `mod interface;`, `pub(crate) mod provides;`, and `#[cfg(test)] mod test;`
    * `[ ]`   `pub fn group_vectors<P: IPairingArithmetic>(deps: &GroupVectorsDeps<'_, P>, params: GroupVectorsParams, payload: GroupVectorsPayload<'_>) -> GroupVectorsReturn`, realizing the branches and ordering of the interaction spec
    * `[ ]`   One private function for the draw, returning the sampled scalar or the function's error, each refusal mapped by `match`; one private function per kind, each taking the deps, the generators it uses, and the index, and returning its vector's entries or the function's error, so no function holds more than one kind
    * `[ ]`   Imports the `pairing` and `random` names the interaction spec uses, `SolidityEntry` and `SolidityValue` from `super::render::provides`, and this module's names from `interface`; no other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[ ]`   `apps/harness-crypto/src/generate/evm/group_vectors/provides.rs`
    * `[ ]`   `pub(crate) use super::group_vectors;` and `pub(crate) use super::interface::*;`, nothing else

  * `[ ]`   `directionality`
    * `[ ]`   `group_vectors` depends on `render`'s entry type through its `provides`, on `pairing`'s traits and declaration, and on `random`'s adapter trait; `render` and `constants` name nothing here and it names nothing of `constants`; no new edge between crates and no cycle
    * `[ ]`   `harness-crypto/generate/evm` calls `group_vectors` and renders its entries within the vectors library; `contracts/evm/PairingLib`'s Foundry tests read the vectors and compare each precompile's result with the reference's

  * `[ ]`   `requirements`
    * `[ ]`   `apps/harness-crypto/src/generate/evm/mod.rs` carries exactly the wiring stated above; `apps/harness-crypto/Cargo.toml`, every file under `render`, `constants`, and `benchmark`, and every file in another crate are unchanged
    * `[ ]`   `cargo check --workspace --all-targets --all-features`, `cargo fmt --all --check`, and `cargo deny check` complete without error; `cargo clippy --workspace --all-targets --all-features` reports nothing beyond the library target's unused-item warnings for the concrete's function modules, which `harness-crypto/generate/evm` resolves by calling them
    * `[ ]`   `each_kind_is_present_the_counted_number_of_times_in_order` and `a_first_group_only_verifier_gets_no_second_group_or_multi_scalar_vector` pass (CR-10, the vectors each verifier form is tested on, the configured number of times)
    * `[ ]`   `every_first_group_vector_is_reproduced_from_the_draws_it_carries`, `every_pairing_vector_is_reproduced_and_carries_both_verdicts`, `every_second_group_and_multi_scalar_vector_is_reproduced_from_the_draws_it_carries`, and `every_vector_is_reproduced_on_bn254_arkworks` pass (CR-10, each vector the reference's own result over the draws it carries, on each curve)
    * `[ ]`   `vectors_of_one_kind_carry_different_draws` and `a_draw_the_sampling_refuses_is_returned_and_no_entry_is` pass
    * `[ ]`   Every existing test in `apps/harness-crypto` passes unchanged

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