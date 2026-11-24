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

## Workspace and discipline bootstrap

* `[✅]`   `workspace/cargo` **Virtual workspace manifest, toolchain pin, dependency policy, and build-output rules at the repository root**

  * `[✅]`   `objective`
    * `[✅]`   Problem: a crate is created, built, and linted only inside a Cargo workspace, and every crate is bound by one toolchain, one lint policy, and one dependency policy
    * `[✅]`   Functional: a virtual workspace manifest at the repository root admits every crate directory under `crates/`, `adapters/`, and `apps/` by glob, so a node that creates a crate never edits the root manifest
    * `[✅]`   Functional: the workspace lint table forbids `unsafe_code`, warns on `dead_code`, denies clippy's `unwrap_used`, `expect_used`, `panic`, and `as_conversions`, and warns on clippy's `too_many_arguments`, and every member inherits it through its own `[lints] workspace = true`
    * `[✅]`   Functional: the workspace declares the inheritable package keys every member takes through `edition.workspace = true`, `rust-version.workspace = true`, and `publish.workspace = true`
    * `[✅]`   Functional: every machine and every CI runner builds with one pinned toolchain carrying `clippy` and `rustfmt`
    * `[✅]`   Functional: `cargo-deny` admits only permissively licensed dependencies from crates.io, and denies yanked versions, wildcard version requirements on published dependencies, and any unlisted registry or git source
    * `[✅]`   Functional: build output is untracked and `Cargo.lock` is tracked
    * `[✅]`   Non-functional: no dependency is pinned, no crate is created, and no tool, target, or component is configured beyond what the four files below state

  * `[✅]`   `role`
    * `[✅]`   Infrastructure: four configuration files with no types and no tests, exempt from the support-file structure; the root of the build graph every later node compiles within
    * `[✅]`   Does not create any directory under `crates/`, `adapters/`, or `apps/`; the node that creates a crate writes that crate's `Cargo.toml`
    * `[✅]`   Does not declare a `[workspace.dependencies]` table or any dependency; each dependency is pinned by the node that first consumes it
    * `[✅]`   Does not create `clippy.toml`, `rustfmt.toml`, or `.cargo/config.toml`; test modules and `mock.rs` files carry their own module-level `allow` for the lint-table entries they are exempt from
    * `[✅]`   Does not add a compilation target or toolchain component beyond `clippy` and `rustfmt`; `wasm32-unknown-unknown` arrives with `apps/wasm-demo`
    * `[✅]`   Does not edit `.github/workflows/rust.yml`; the CI matrix, `cargo-audit`, and `cargo-deny` invocation belong to `workspace/ci`
    * `[✅]`   Does not carry a commit; the grouping's integration test and commit are carried by `random/factory`

  * `[✅]`   `module`
    * `[✅]`   Bounded context: repository-root build configuration — member layout, workspace lint table, inheritable package keys, the `librqbit` overlay, toolchain pin, dependency-policy file, and build-output ignore rules
    * `[✅]`   Member layout by ring: `crates/` holds `domain`, and `workflows`; `adapters/` holds one crate per adapter family, the crate path `adapters/<family>` the planning set names, each crate holding the family's `factory` module and its concretes as private modules beneath it; `apps/` holds one crate per deployable
    * `[✅]`   Outside: every member's own `Cargo.toml`, the CI definition, dependency versions, the Solidity workspace under `contracts/`, and the TypeScript shells under `shells/`, none of which is a Cargo member

  * `[✅]`   `deps`
    * `[✅]`   `rustup`, external toolchain manager, reads `rust-toolchain.toml` at the repository root; direction inward, the configuration names the toolchain and nothing names the configuration
    * `[✅]`   `cargo`, external build tool, reads `Cargo.toml`; resolver `3` requires the pinned toolchain, which satisfies it
    * `[✅]`   `cargo-deny`, external policy checker installed and invoked by `workspace/ci`, reads `deny.toml`
    * `[✅]`   No repository file is a dependency of this node, and no reverse dependency exists

  * `[✅]`   `Cargo.toml`
    * `[✅]`   `[workspace]` with `resolver = "3"` and `members = ["crates/*", "adapters/*", "apps/*"]`; no `exclude`, no `default-members`, no `[package]` table, so the manifest is virtual
    * `[✅]`   `[workspace.package]` with `edition = "2024"`, `rust-version = "1.98"`, and `publish = false`; no `license` key, since the license text is a release prerequisite
    * `[✅]`   `[workspace.lints.rust]` with `unsafe_code = "forbid"` and `dead_code = "warn"`
    * `[✅]`   `[workspace.lints.clippy]` with `unwrap_used = "deny"`, `expect_used = "deny"`, `panic = "deny"`, `as_conversions = "deny"`, and `too_many_arguments = "warn"`
    * `[✅]`   `[patch.crates-io]` with `librqbit = { git = "https://github.com/tsylvester/rqbit", branch = "chaintorrent-overlay" }`, the branch existing at a tagged upstream release before this node is authored; `Cargo.lock` pins the commit
    * `[✅]`   No other table

  * `[✅]`   `rust-toolchain.toml`
    * `[✅]`   `[toolchain]` with `channel = "1.98.1"`, `components = ["clippy", "rustfmt"]`, and `profile = "minimal"`

  * `[✅]`   `deny.toml`
    * `[✅]`   `[graph]` with `all-features = true`
    * `[✅]`   `[advisories]` with `yanked = "deny"`, `unmaintained = "all"`, `unsound = "all"`, and `ignore = []`; no `version`, `vulnerability`, `notice`, or `severity-threshold` key
    * `[✅]`   `[licenses]` with `allow = ["Apache-2.0", "Apache-2.0 WITH LLVM-exception", "MIT", "BSD-2-Clause", "BSD-3-Clause", "CC0-1.0", "Unicode-3.0"]` and `confidence-threshold = 0.93`; no `version` key
    * `[✅]`   `[licenses.private]` with `ignore = true`, so the unpublished, unlicensed workspace members are not license-checked
    * `[✅]`   `[bans]` with `multiple-versions = "warn"`, `wildcards = "deny"`, and `allow-wildcard-paths = true`, so path dependencies between the private workspace members are admitted
    * `[✅]`   `[sources]` with `unknown-registry = "deny"`, `unknown-git = "deny"`, `allow-registry = ["https://github.com/rust-lang/crates.io-index"]`, and `allow-git = ["https://github.com/tsylvester/rqbit"]`, the source the `[patch.crates-io]` table names

  * `[✅]`   `.gitignore`
    * `[✅]`   One entry, `/target`; `Cargo.lock` is not ignored

  * `[✅]`   `directionality`
    * `[✅]`   The root manifest declares no dependency and depends on no member; members depend inward across rings, `apps/` on `adapters/` and `crates/`, `adapters/` on `crates/`, and `crates/domain` on nothing outside itself, each through its own manifest
    * `[✅]`   No cycle is introduced; the root is the build graph's container, not a node in it

  * `[✅]`   `requirements`
    * `[✅]`   `Cargo.toml`, `rust-toolchain.toml`, `deny.toml`, and `.gitignore` exist at the repository root with exactly the keys and values stated above and no others
    * `[✅]`   No directory exists under `crates/`, `adapters/`, or `apps/` when this node completes

* `[✅]`   `domain/secret` **Secret-typed value that cannot be formatted, cloned, or serialized, exposes its value only through an explicit accessor, and zeroizes on drop; creates the `domain` crate**

  * `[✅]`   `objective`
    * `[✅]`   Problem: master scalars, credentials, piece-group keys, envelope secrets, and seeds are ordinary values unless a type prevents them from reaching a log, a diagnostic, a serialized record, or a stray copy, and from outliving their use in memory (CR-07)
    * `[✅]`   Functional: one generic type wraps any secret value whose type implements `zeroize::Zeroize`
    * `[✅]`   Functional: the wrapped value is reachable only through one explicit accessor returning a shared reference, never by field access outside the `secret` module
    * `[✅]`   Functional: the wrapped value is zeroized when the secret is dropped, on every path that ends its lifetime
    * `[✅]`   Functional: the type implements none of `core::fmt::Debug`, `core::fmt::Display`, `Clone`, or `Copy`, proven by a compile-time assertion
    * `[✅]`   Functional: the type cannot implement a serialization trait, because the `domain` crate has no serialization dependency
    * `[✅]`   Non-functional: the `domain` crate depends on `zeroize` alone at runtime and on no repository crate, host, chain SDK, wallet, transport, or storage engine

  * `[✅]`   `role`
    * `[✅]`   Domain: an owned value type in the protocol and domain ring; every secret-producing module in later nodes wraps its material in this type
    * `[✅]`   Does not generate, derive, compare, encrypt, or store secrets; producers and custody do that and hand the value to `Secret::try_new`
    * `[✅]`   Does not offer a mutable accessor, an owning extractor, equality, or ordering
    * `[✅]`   Does not create any other module of the `domain` crate; each further module is created by the node of the type it implements
    * `[✅]`   Does not carry a commit; the grouping's integration test and commit are carried by `random/factory`

  * `[✅]`   `module`
    * `[✅]`   Bounded context: the `secret` module of the `domain` crate, holding the `Secret<T>` type, its constructor-params type, and its lifecycle: construction by move, shared read access, zeroization on drop
    * `[✅]`   Creates the `domain` crate at `crates/domain`, admitted by the workspace's `crates/*` glob with no edit to the root manifest
    * `[✅]`   Outside: which values are secret, how they are produced, how custody wraps them for storage, and the memory-only secret region the daemon builds from them

  * `[✅]`   `deps`
    * `[✅]`   `zeroize` `1.9.0`, external crate, Apache-2.0 OR MIT, runtime dependency of `domain`; supplies the `Zeroize` trait the wrapped type is bound by and whose `zeroize` the drop calls; zeroization on drop is a property of the type, not an external touchpoint, so no adapter wraps it
    * `[✅]`   `static_assertions` `1.1.0`, external crate, MIT OR Apache-2.0, dev-dependency of `domain` only; supplies `assert_not_impl_any!`, the compile-time proof that the type implements no formatting, cloning, or copying trait
    * `[✅]`   `core::convert::Infallible`, standard library, the constructor's error arm
    * `[✅]`   No repository crate is a dependency; nothing depends on this module yet; direction inward, `domain` is the innermost ring

  * `[✅]`   `context_slice`
    * `[✅]`   From `zeroize`: the `Zeroize` trait's `zeroize(&mut self)` as a bound on `T`; nothing else
    * `[✅]`   From `static_assertions`: the `assert_not_impl_any!` macro, in `test.rs` only

  * `[✅]`   `crates/domain/Cargo.toml`
    * `[✅]`   `[package]` with `name = "domain"`, `edition.workspace = true`, `rust-version.workspace = true`, and `publish.workspace = true`; no `version` key
    * `[✅]`   `[dependencies]` with `zeroize = "1.9.0"`, default features
    * `[✅]`   `[dev-dependencies]` with `static_assertions = "1.1.0"`
    * `[✅]`   `[features]` with `mocks = []`
    * `[✅]`   `[lints]` with `workspace = true`
    * `[✅]`   No other table and no serialization dependency

  * `[✅]`   `crates/domain/src/lib.rs`
    * `[✅]`   The crate barrel: `mod secret;` and `pub use secret::provides::*;`, nothing else
    * `[✅]`   Until `secret/mod.rs` exists, `cargo check` reports the unresolved `mod secret`, which is the RED state for every element below that precedes the implementation

  * `[✅]`   `crates/domain/src/secret/interface.rs`
    * `[✅]`   `Secret<T: Zeroize>`, a struct with one field `pub(super) value: T`, so only the `secret` module and its children reach the field; no derives
    * `[✅]`   `SecretConstructorParams<T: Zeroize>`, a struct with one field `pub value: T`; no derives
    * `[✅]`   `SecretTryNewReturn<T>`, the type alias `Result<Secret<T>, Infallible>`, the two-arm return of the constructor; the error arm is uninhabited because moving a value into the wrapper has no failure
    * `[✅]`   Imports `zeroize::Zeroize` and `core::convert::Infallible`; declares nothing else

  * `[✅]`   `crates/domain/src/secret/interaction.spec.md`
    * `[✅]`   `Secret::try_new(params: SecretConstructorParams<T>) -> SecretTryNewReturn<T>`: one branch; condition: any params; decision: none; dependency call: none; outcome: `Ok(Secret)` holding `params.value`, moved without copy; the error arm has no branch
    * `[✅]`   `Secret::expose(&self) -> &T`: one branch; condition: a living secret; decision: none; dependency call: none; outcome: a shared reference to the held value, no copy, no side effect
    * `[✅]`   Drop: one branch; condition: the secret's lifetime ends, by scope end, move into a consumer that drops it, or unwinding; decision: none; dependency call: `Zeroize::zeroize` on the held value, exactly once; outcome: the held value's memory is zeroized before release
    * `[✅]`   Invariants: `Secret<T>` implements none of `Debug`, `Display`, `Clone`, or `Copy`, and cannot implement a serialization trait; its only producer is `try_new`

  * `[✅]`   `crates/domain/src/secret/mock.rs`
    * `[✅]`   Module-level `#![allow(dead_code, clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::too_many_arguments)]`
    * `[✅]`   `SecretConstructorParamsOverrides`, the fieldless unit struct with `#[derive(Default)]`, since the value is handed to the builder
    * `[✅]`   `build_secret_constructor_params<T: Zeroize>(value: T, overrides: SecretConstructorParamsOverrides) -> SecretConstructorParams<T>`, the value its leading parameter, typed exactly as the field, since a value of a type parameter has no default
    * `[✅]`   `build_secret<T: Zeroize>(value: T, overrides: SecretConstructorParamsOverrides) -> Secret<T>`, returning the real instance from `Secret::try_new(build_secret_constructor_params(value, overrides))` through the irrefutable pattern `let Ok(secret) = …;`
    * `[✅]`   No corruptions type and no invalidator: the params never arrive as untrusted data, and the crate carries no serialization dependency to express corruption; no `Secret` overrides, invalidator, or mock function, since the class is built as a real instance and owns no free function

  * `[✅]`   `crates/domain/src/secret/test.rs`
    * `[✅]`   Imports `Secret`, `build_secret`, and `build_secret_constructor_params` from `super::provides`, `Zeroize` from `zeroize`, and `assert_not_impl_any` from `static_assertions`; every test holds `Vec<u8>` except where it holds the block-local recorder named in that test
    * `[✅]`   Compile-time assertion over `Secret<Vec<u8>>`
      * `[✅]`   Contract: `Secret<T>` implements none of `core::fmt::Debug`, `core::fmt::Display`, `Clone`, or `Copy`
      * `[✅]`   Arrange: the type `Secret<Vec<u8>>`
      * `[✅]`   Act: `assert_not_impl_any!` over that type naming `core::fmt::Debug`, `core::fmt::Display`, `Clone`, and `Copy`
      * `[✅]`   Assert: the module compiles only if none of the four traits is implemented
    * `[✅]`   `try_new_holds_the_value_in_params`
      * `[✅]`   Contract: any params → `Ok(Secret)` holding `params.value`
      * `[✅]`   Collaborators: none; the fixture is `build_secret_constructor_params::<Vec<u8>>`
      * `[✅]`   Arrange: `build_secret_constructor_params` with a nonempty byte sequence handed as the value, so an empty vector and the given value differ
      * `[✅]`   Act: `Secret::try_new` on the built params
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(secret) = …;`; the held `value`, read through the field the child test module reaches, equals the same byte sequence written as a literal in the assertion
    * `[✅]`   `expose_returns_a_reference_to_the_held_value`
      * `[✅]`   Contract: a living secret → a shared reference to the held value, no copy
      * `[✅]`   Collaborators: none; the fixture is `build_secret::<Vec<u8>>`
      * `[✅]`   Arrange: `build_secret` with a nonempty byte sequence handed as the value, so an empty vector and the held value differ
      * `[✅]`   Act: `secret.expose()`
      * `[✅]`   Assert: the returned reference equals the same byte sequence written as a literal in the assertion; the returned reference is address-identical to the secret's held `value` field under `core::ptr::eq`
    * `[✅]`   `dropping_a_secret_zeroizes_its_value`
      * `[✅]`   Contract: the secret's lifetime ends by move into a consumer that drops it → `Zeroize::zeroize` is called on the held value exactly once
      * `[✅]`   Collaborators: `Zeroize::zeroize` on the held value, replaced by a block-local struct implementing `Zeroize` whose `zeroize` increments an `Arc<AtomicUsize>` counter shared with the test
      * `[✅]`   Arrange: the shared counter at its initial value; the recorder holding a handle to it; `build_secret` with the recorder handed as the value
      * `[✅]`   Act: `drop(secret)`
      * `[✅]`   Assert: the shared counter equals the literal one
    * `[✅]`   `exposing_a_secret_does_not_zeroize_it`
      * `[✅]`   Contract: a living secret → `expose` returns a shared reference with no side effect on the held value
      * `[✅]`   Collaborators: `Zeroize::zeroize` on the held value, replaced by a block-local struct implementing `Zeroize` whose `zeroize` increments an `Arc<AtomicUsize>` counter shared with the test
      * `[✅]`   Arrange: the shared counter at its initial value; the recorder holding a handle to it; `build_secret` with the recorder handed as the value
      * `[✅]`   Act: `secret.expose()`
      * `[✅]`   Assert: the shared counter equals the literal zero while the secret is alive
    * `[✅]`   `try_new_moves_the_value_without_copying_it`
      * `[✅]`   Contract: any params → `Ok(Secret)` holding `params.value`, moved without copy
      * `[✅]`   Collaborators: none; the fixture is `build_secret_constructor_params::<Vec<u8>>`
      * `[✅]`   Arrange: a nonempty byte vector bound to a local whose heap address is read before the call; `build_secret_constructor_params` with that vector handed as the value
      * `[✅]`   Act: `Secret::try_new` on the built params
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(secret) = …;`; the heap address of the held `value` equals the address read before the call
    * `[✅]`   `dropping_a_secret_during_unwinding_zeroizes_its_value`
      * `[✅]`   Contract: the secret's lifetime ends by unwinding → `Zeroize::zeroize` is called on the held value exactly once
      * `[✅]`   Collaborators: `Zeroize::zeroize` on the held value, replaced by a block-local struct implementing `Zeroize` whose `zeroize` increments an `Arc<AtomicUsize>` counter shared with the test
      * `[✅]`   Arrange: the shared counter at its initial value; the recorder holding a handle to it; `build_secret` with the recorder handed as the value; a closure that takes the secret by move and panics
      * `[✅]`   Act: `std::panic::catch_unwind` on the closure
      * `[✅]`   Assert: the outcome is `Err`; the shared counter equals the literal one

  * `[✅]`   `construction`
    * `[✅]`   `Secret::try_new` is the only producer; no `Default`, `From`, or other constructor exists; each later producer of secret material constructs the secret where the material is produced and passes it by move

  * `[✅]`   `crates/domain/src/secret/mod.rs`
    * `[✅]`   Module declarations: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, `pub mod provides;`, `#[cfg(test)] mod test;`
    * `[✅]`   `impl<T: Zeroize> Secret<T>` with `pub fn try_new(params: SecretConstructorParams<T>) -> SecretTryNewReturn<T>` returning `Ok(Secret { value: params.value })`, and `pub fn expose(&self) -> &T` returning `&self.value`
    * `[✅]`   `impl<T: Zeroize> Drop for Secret<T>` whose `drop` calls `self.value.zeroize()`
    * `[✅]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[✅]`   `crates/domain/src/secret/provides.rs`
    * `[✅]`   `pub use super::interface::*;` and `#[cfg(any(test, feature = "mocks"))] pub use super::mock::*;`, nothing else, so a sibling module's test and mock reach this module's builders in the crate's own test build as well as under the `mocks` feature

  * `[✅]`   `directionality`
    * `[✅]`   `secret` depends on `zeroize` and `core` only; `domain` depends on no repository crate; later consumers in the adapter and workflow rings depend on `domain` through `lib.rs`'s re-export of `secret::provides`; no cycle

  * `[✅]`   `requirements`
    * `[✅]`   `crates/domain/Cargo.toml` carries exactly the tables and keys stated above, and no serialization dependency
    * `[✅]`   `cargo check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo fmt --check` complete without error with the `domain` crate as the workspace's only member
    * `[✅]`   The `assert_not_impl_any!` assertion compiles, proving `Secret` implements none of `Debug`, `Display`, `Clone`, or `Copy` (CR-07, formatting excluded at compile time)
    * `[✅]`   `expose_returns_a_reference_to_the_held_value` passes
    * `[✅]`   `dropping_a_secret_zeroizes_its_value` passes (CR-07 zeroization)
    * `[✅]`   Code outside `crates/domain/src/secret` reading the `value` field fails to compile
    * `[✅]`   A production `unwrap`, `expect`, `panic!`, numeric `as`, or `unsafe` block in the first production crate, `domain/secret`, is rejected by `cargo clippy` under the inherited lint table
    * `[✅]`   `cargo check`, `cargo clippy`, and `cargo fmt --check` at the repository root complete without error

* `[✅]`   `random/os` **Operating-system randomness concrete, the one source every production draw passes through; creates the `adapters/random` crate and authors the randomness family's generic interface, member identifier with one total spelling per real member, declared set, declaration, selection, and mock**

  * `[✅]`   `objective`
    * `[✅]`   Problem: master scalars, capsule randomness, piece-group keys, and IVs come from a cryptographic random source (CR-05), and every production path draws through one repo-owned interface, so no module outside the concrete names the generator's library
    * `[✅]`   Functional: the family's generic interface draws a requested number of bytes and returns exactly that many bytes inside a `Secret`, including zero for a zero-length request, so every draw is zeroized when dropped whether or not the caller keeps it; every implementation, including the default mock, obeys the requested length
    * `[✅]`   Functional: every concrete declares its source kind, its adapter version, and the interface version it implements, readable from the type before any instance exists
    * `[✅]`   Functional: the member identifier spells each real member once and totally, the declared set holds the real members alone, the real selection carries no settings, and the mock selection alone carries its failure mode
    * `[✅]`   Functional: the operating-system concrete draws from the operating system's generator through `getrandom` and returns the generator's error unchanged in its error arm
    * `[✅]`   Functional: repeated draws of a fixed width are pairwise distinct across a fixed count, and a draw of a fixed length holds more than one distinct byte value
    * `[✅]`   Non-functional: `getrandom` is named only inside `adapters/random/src/os`; the crate depends on no repository crate but `domain`

  * `[✅]`   `role`
    * `[✅]`   Adapter: the randomness family's first concrete, and the first source file that requires the family's generic interface, member identifier with one total spelling per real member, declared set, declaration, selection, and mock, which it authors in the family's `factory` module as its producers
    * `[✅]`   Creates `factory/mod.rs` with the module wiring alone and `factory/provides.rs` with the interface and mock surface alone; the factory function with its types, interaction spec, mock, builders, unit test, and re-export, and the family's integration test, are `random/factory`'s
    * `[✅]`   Does not author the sampling trait a scalar type implements to be drawn from uniform bytes; `pairing/bn254_arkworks`, its first consumer, adds it to the family's interface
    * `[✅]`   Does not provide a deterministic source, a seeded source, or any source but the operating system's
    * `[✅]`   Does not carry a settings module, settings node, or settings table; the concrete has no settable value, so the real selection carries no settings
    * `[✅]`   Does not configure a `getrandom` backend for `wasm32-unknown-unknown`; that arrives with `apps/wasm-demo`
    * `[✅]`   Does not carry a commit; the grouping's integration test and commit are carried by `random/factory`

  * `[✅]`   `module`
    * `[✅]`   Bounded context: the `adapters/random` crate's `factory` module, holding the family's generic trait, its method's params, payload, success, and error types, the member identifier with its total spellings, the declared set, the declaration type, the selection, the interface version, and the family's mock; and its private `os` concrete, holding the adapter over the operating system's generator, its declaration constant, its constructor params, and its error type
    * `[✅]`   Creates the crate at `adapters/random`, admitted by the workspace's `adapters/*` glob with no edit to the root manifest
    * `[✅]`   Outside: which values a caller draws and what it builds from them, the sampling of scalars from uniform bytes, and the factory's admission rule

  * `[✅]`   `deps`
    * `[✅]`   `domain`, `crates/domain`, protocol and domain ring, path dependency; supplies `Secret` and `SecretConstructorParams`, the wrapper every draw is returned in; direction inward, adapter ring on domain ring
    * `[✅]`   `domain` with its `mocks` feature, as a dev-dependency and through this crate's `mocks` feature; supplies `build_secret` and `SecretConstructorParamsOverrides` for the success-return builder
    * `[✅]`   `getrandom` `0.4.3`, external crate, MIT OR Apache-2.0, runtime dependency named only in `os`; supplies `getrandom::fill` and `getrandom::Error`
    * `[✅]`   `core::convert::Infallible`, standard library, the constructor's error arm
    * `[✅]`   `std::collections::HashSet`, standard library, in `os/test.rs` only
    * `[✅]`   No reverse dependency; nothing depends on this crate yet

  * `[✅]`   `context_slice`
    * `[✅]`   From `domain`: `Secret::try_new(SecretConstructorParams { value })` returning `Result<Secret<T>, Infallible>`, and `Secret::expose(&self) -> &T` in tests
    * `[✅]`   From `domain`'s mocks: `build_secret(value: T, SecretConstructorParamsOverrides) -> Secret<T>` for `T: Zeroize`, instantiated at `Vec<u8>`
    * `[✅]`   From `getrandom`: `fill(dest: &mut [u8]) -> Result<(), Error>`, and `Error`, which implements `Debug`

  * `[✅]`   `adapters/random/Cargo.toml`
    * `[✅]`   `[package]` with `name = "random"`, `edition.workspace = true`, `rust-version.workspace = true`, and `publish.workspace = true`; no `version` key
    * `[✅]`   `[dependencies]` with `domain = { path = "../../crates/domain" }` and `getrandom = "0.4.3"`, default features
    * `[✅]`   `[dev-dependencies]` with `domain = { path = "../../crates/domain", features = ["mocks"] }`
    * `[✅]`   `[features]` with `mocks = ["domain/mocks"]`
    * `[✅]`   `[lints]` with `workspace = true`
    * `[✅]`   No other table

  * `[✅]`   `adapters/random/src/lib.rs`
    * `[✅]`   The crate barrel: `mod factory;`, `mod os;`, and `pub use factory::provides::*;`, nothing else
    * `[✅]`   Until `factory/mod.rs` and `os/mod.rs` exist, `cargo check` reports the unresolved modules, which is the RED state for every element below that precedes them

  * `[✅]`   `adapters/random/src/factory/interface.rs`
    * `[✅]`   `RANDOM_SOURCE_INTERFACE_VERSION`, a `pub const` of type `u32` with value `1`, the version of this interface a concrete declares it implements
    * `[✅]`   `RandomSourceKind`, an enum, the typed kind the family's declarations and selections carry, with the variant `OperatingSystem` and, under `#[cfg(any(test, feature = "mocks"))]`, the variant `Mock(MockIRandomSourceAdapterFailureMode)`, the mock selection carrying the failure mode its configuration names; no derives
    * `[✅]`   `RandomSourceKind::name()`, an exhaustive mapping spelling `OperatingSystem` as `operating_system` and, under `#[cfg(any(test, feature = "mocks"))]`, `Mock(_)` as `mock`, each declared member's only spelling at a boundary
    * `[✅]`   `RANDOM_SOURCE_KINDS`, a `pub const` of type `&[RandomSourceKind]` containing only the real `OperatingSystem` member, so input admission accepts only declared real spellings and never mock text
    * `[✅]`   `MockIRandomSourceAdapterFailureMode`, under `#[cfg(any(test, feature = "mocks"))]`, an enum with `#[derive(Clone, Copy)]` and the variants `Succeeds` and `FillBytesRefused`, the second standing for the arm of `fill_bytes` that no input reaches, the operating system refusing a draw
    * `[✅]`   `RandomSourceDeclaration`, a struct with `pub source: RandomSourceKind`, `pub adapter_version: u32`, and `pub interface_version: u32`; no derives; the source is the member's typed identifier and the declaration carries no duplicate text name
    * `[✅]`   `FillBytesParams`, the fieldless struct `pub struct FillBytesParams;`, the per-call control slot of `fill_bytes`
    * `[✅]`   `FillBytesPayload`, a struct with `pub length: usize`, the number of bytes to draw
    * `[✅]`   `FillBytesSuccessReturn`, a struct with `pub bytes: Secret<Vec<u8>>`
    * `[✅]`   `FillBytesErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]` and the variant `OperatingSystem(OsRandomSourceFillBytesErrorReturn)`, the operating-system concrete's error carried unchanged, and, under `#[cfg(any(test, feature = "mocks"))]`, the fieldless variant `MockIRandomSourceAdapter`, the mock concrete's own error; each concrete's error is its own variant
    * `[✅]`   `FillBytesReturn`, the type alias `Result<FillBytesSuccessReturn, FillBytesErrorReturn>`
    * `[✅]`   `IRandomSourceAdapter`, an object-safe trait with `fn fill_bytes(&self, params: FillBytesParams, payload: FillBytesPayload) -> FillBytesReturn;`; a member's declaration is read through `random/lookup_random_declaration` without constructing the member, never from the adapter
    * `[✅]`   Imports `domain::Secret` and `OsRandomSourceFillBytesErrorReturn` from `crate::os::provides`; names no vendor

  * `[✅]`   `adapters/random/src/os/interface.rs`
    * `[✅]`   `OsRandomSource`, the unit struct `pub struct OsRandomSource;`, the adapter over the operating system's generator
    * `[✅]`   `OsRandomSourceConstructorParams`, the fieldless struct `pub struct OsRandomSourceConstructorParams;`, the constructor's deps slot
    * `[✅]`   `OsRandomSourceTryNewReturn`, the type alias `Result<OsRandomSource, Infallible>`; the error arm is uninhabited because the operating-system source takes no configuration
    * `[✅]`   `OsRandomSourceFillBytesErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]`, which `getrandom::Error` satisfies, and the one variant `OperatingSystem(getrandom::Error)`, the generator's failure carried unchanged
    * `[✅]`   `OsRandomSource::DECLARATION`, an inherent `pub const` declared in this interface, `RandomSourceDeclaration { source: RandomSourceKind::OperatingSystem, adapter_version: 1, interface_version: RANDOM_SOURCE_INTERFACE_VERSION }`; the constructor stays settings-free and the concrete has no settings module
    * `[✅]`   Imports `core::convert::Infallible` and `RandomSourceDeclaration`, `RandomSourceKind`, and `RANDOM_SOURCE_INTERFACE_VERSION` from `crate::factory::provides`; names `getrandom::Error` by its full path; declares nothing else

  * `[✅]`   `adapters/random/src/os/interaction.spec.md`
    * `[✅]`   `OsRandomSource::try_new(params: OsRandomSourceConstructorParams) -> OsRandomSourceTryNewReturn`: one branch; condition: any params; decision: none; dependency call: none; outcome: `Ok(OsRandomSource)`; the error arm has no branch
    * `[✅]`   `OsRandomSource::DECLARATION`: an inherent constant held by `os/interface.rs`, `RandomSourceDeclaration { source: RandomSourceKind::OperatingSystem, adapter_version: 1, interface_version: RANDOM_SOURCE_INTERFACE_VERSION }`, read through `random/lookup_random_declaration` without an instance, never obtained from a constructed adapter
    * `[✅]`   `fill_bytes`, drawn: condition: `getrandom::fill` returns `Ok(())` over a zero-initialized buffer of `payload.length` bytes; decision: the fill result; dependency call: `getrandom::fill`, exactly once; outcome: `Ok(FillBytesSuccessReturn { bytes })`, the filled buffer moved into a `Secret` without copy
    * `[✅]`   `fill_bytes`, generator failed: condition: `getrandom::fill` returns `Err(error)`; decision: the fill result; dependency call: `getrandom::fill`, exactly once; outcome: `Err(FillBytesErrorReturn::OperatingSystem(OsRandomSourceFillBytesErrorReturn::OperatingSystem(error)))`, the error unchanged; the buffer, already moved into a `Secret`, is zeroized when it drops
    * `[✅]`   Ordering: the buffer is moved into a `Secret` after `getrandom::fill` returns and before its result is inspected, so the buffer is zeroized on both branches
    * `[✅]`   A `payload.length` of zero takes the drawn branch with an empty buffer; `params` carries no control and is not read

  * `[✅]`   `adapters/random/src/factory/mock.rs`
    * `[✅]`   Module-level `#![allow(dead_code, clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::too_many_arguments)]`
    * `[✅]`   `RandomSourceDeclarationOverrides`, `#[derive(Default)]`, fields `pub source: Option<RandomSourceKind>`, `pub adapter_version: Option<u32>`, and `pub interface_version: Option<u32>`; `build_random_source_declaration(overrides: RandomSourceDeclarationOverrides) -> RandomSourceDeclaration`, defaulting to `RandomSourceKind::OperatingSystem`, `1`, and `RANDOM_SOURCE_INTERFACE_VERSION`
    * `[✅]`   `FillBytesPayloadOverrides`, `#[derive(Default)]`, one field `pub length: Option<usize>`; `build_fill_bytes_payload(overrides: FillBytesPayloadOverrides) -> FillBytesPayload`, the length defaulting to `32`
    * `[✅]`   `FillBytesSuccessReturnOverrides`, `#[derive(Default)]`, one field `pub bytes: Option<Secret<Vec<u8>>>`; `build_fill_bytes_success_return(overrides: FillBytesSuccessReturnOverrides) -> FillBytesSuccessReturn`, the bytes defaulting to `build_secret(vec![0u8; 32], Default::default())`, matching the payload builder's default length; tests needing another length supply an override
    * `[✅]`   `MockIRandomSourceAdapterConstructorParams`, a struct with `pub failure_mode: MockIRandomSourceAdapterFailureMode`; `MockIRandomSourceAdapterConstructorParamsOverrides`, `#[derive(Default)]`, one field `pub failure_mode: Option<MockIRandomSourceAdapterFailureMode>`; `build_mock_i_random_source_adapter_constructor_params(overrides: MockIRandomSourceAdapterConstructorParamsOverrides) -> MockIRandomSourceAdapterConstructorParams`, the mode defaulting to `MockIRandomSourceAdapterFailureMode::Succeeds`; no corruptions type and no invalidator, since the params arrive typed from the factory
    * `[✅]`   `MockIRandomSourceAdapter`, the family's mock concrete, a `pub(crate) struct` with `pub(super) failure_mode: MockIRandomSourceAdapterFailureMode` and `pub(super) counter: Cell<u64>`; `MockIRandomSourceAdapterTryNewReturn`, the alias `Result<MockIRandomSourceAdapter, Infallible>`; `impl MockIRandomSourceAdapter` with `pub(crate) fn try_new(params: MockIRandomSourceAdapterConstructorParams) -> MockIRandomSourceAdapterTryNewReturn` returning `Ok` with the params' mode and a counter of zero; the type is crate-visible and the public surface carries the configuration that selects it
    * `[✅]`   `impl IRandomSourceAdapter for MockIRandomSourceAdapter`, self-contained and answering the family's contract: `fill_bytes` returns `Err(FillBytesErrorReturn::MockIRandomSourceAdapter)` when the mode is `FillBytesRefused`, and otherwise `Ok` with exactly `payload.length` bytes, zero for zero, moved into a `Secret`, each byte produced by advancing `counter` through a fixed mixing function, so the bytes differ within a draw and from one draw to the next; `params` is not read
    * `[✅]`   `build_mock_i_random_source_adapter(overrides: MockIRandomSourceAdapterConstructorParamsOverrides) -> MockIRandomSourceAdapter`, returning the real instance from `MockIRandomSourceAdapter::try_new(build_mock_i_random_source_adapter_constructor_params(overrides))` through the irrefutable pattern `let Ok(adapter) = …;`
    * `[✅]`   `RandomSourceKindOverrides`, the fieldless overrides struct, and `build_random_source_kind(kind: RandomSourceKind, overrides: RandomSourceKindOverrides) -> RandomSourceKind`, the family's declared-selection builder, an exhaustive `match` over `RANDOM_SOURCE_KINDS` whose real `OperatingSystem` arm returns the selection with no configuration; the failure mode stays only in the mock selection's configuration, outside the declared real set
    * `[✅]`   No builder for `FillBytesParams`, which is fieldless and used by its production value; no corruptions type and no invalidator, since no value this interface owns arrives as untrusted data
    * `[✅]`   Imports `Secret` and `build_secret` from `domain`, `core::cell::Cell` and `core::convert::Infallible`, and this module's types from `super::interface`

  * `[✅]`   `adapters/random/src/factory/mod.rs`
    * `[✅]`   Module wiring only: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, and `pub mod provides;`, nothing else

  * `[✅]`   `adapters/random/src/factory/provides.rs`
    * `[✅]`   `pub use super::interface::*;` and `#[cfg(any(test, feature = "mocks"))] pub use super::mock::*;`, nothing else

  * `[✅]`   `adapters/random/src/os/mock.rs`
    * `[✅]`   Module-level `#![allow(dead_code, clippy::too_many_arguments)]`
    * `[✅]`   `build_os_random_source() -> OsRandomSource`, returning the real instance from `OsRandomSource::try_new(OsRandomSourceConstructorParams)` through the irrefutable pattern `let Ok(source) = …;`
    * `[✅]`   No overrides type, corruptions type, or invalidator for `OsRandomSourceConstructorParams`, which is fieldless, used by its production value, and never arrives as untrusted data; no `OsRandomSource` overrides, invalidator, or mock function, since the concrete is built as a real instance and owns no free function
    * `[✅]`   Imports this module's types from `super::interface`

  * `[✅]`   `adapters/random/src/os/test.rs`
    * `[✅]`   Module-level `#![allow(clippy::expect_used)]`; imports `OsRandomSource` and `build_os_random_source` from `super::provides`, `FillBytesParams`, `FillBytesPayloadOverrides`, `IRandomSourceAdapter`, `RandomSourceKind`, `RANDOM_SOURCE_INTERFACE_VERSION`, `RANDOM_SOURCE_KINDS`, and `build_fill_bytes_payload` from `crate::factory::provides`, and `HashSet` from `std::collections`
    * `[✅]`   `fill_bytes_returns_the_number_of_bytes_requested`
      * `[✅]`   Contract: `getrandom::fill` returns `Ok(())` over a buffer of `payload.length` bytes → `Ok(FillBytesSuccessReturn)` whose `Secret` holds exactly `payload.length` bytes
      * `[✅]`   Collaborators: `getrandom::fill`, the operating system's generator, called for real as the outer edge; fixtures `build_os_random_source` and `build_fill_bytes_payload`; `FillBytesParams` by its production value
      * `[✅]`   Arrange: `build_fill_bytes_payload` with the length override set to 48, a nonzero length that differs from the builder's default of 32, so a source returning the default width fails
      * `[✅]`   Act: `fill_bytes(FillBytesParams, payload)` on the built source
      * `[✅]`   Assert: the success arm is extracted with `expect`; the length of `bytes.expose()` equals the literal 48 written in the assertion
    * `[✅]`   `fill_bytes_returns_an_empty_draw_for_a_zero_length`
      * `[✅]`   Contract: `payload.length` of zero takes the drawn branch with an empty buffer → `Ok(FillBytesSuccessReturn)` whose `Secret` holds no bytes
      * `[✅]`   Collaborators: `getrandom::fill`, the operating system's generator, called for real as the outer edge; fixtures `build_os_random_source` and `build_fill_bytes_payload`; `FillBytesParams` by its production value
      * `[✅]`   Arrange: `build_fill_bytes_payload` with the length override set to 0, so a source that ignores the length and returns the builder's default width fails
      * `[✅]`   Act: `fill_bytes(FillBytesParams, payload)` on the built source
      * `[✅]`   Assert: the success arm is extracted with `expect`; `bytes.expose()` is empty
    * `[✅]`   `fill_bytes_draws_pairwise_distinct_segments_within_a_draw`
      * `[✅]`   Contract: a drawn buffer is filled by the generator → segments of a fixed width within one draw are pairwise distinct
      * `[✅]`   Collaborators: `getrandom::fill`, the operating system's generator, called for real as the outer edge; fixtures `build_os_random_source` and `build_fill_bytes_payload`; `FillBytesParams` by its production value
      * `[✅]`   Arrange: `build_fill_bytes_payload` with the length override set to 1024, which splits into segments of 32 bytes, so a zero-filled or repeating buffer yields fewer distinct segments than segments
      * `[✅]`   Act: `fill_bytes(FillBytesParams, payload)` on the built source
      * `[✅]`   Assert: the success arm is extracted with `expect`; the 32-byte segments of `bytes.expose()` collected into a `HashSet` number the literal 32 written in the assertion
    * `[✅]`   `fill_bytes_fills_a_draw_with_more_than_one_distinct_byte_value`
      * `[✅]`   Contract: a drawn buffer is filled by the generator → a draw of a fixed length holds more than one distinct byte value
      * `[✅]`   Collaborators: `getrandom::fill`, the operating system's generator, called for real as the outer edge; fixtures `build_os_random_source` and `build_fill_bytes_payload`; `FillBytesParams` by its production value
      * `[✅]`   Arrange: `build_fill_bytes_payload` with the length override set to 256, so a zero-filled buffer holds one distinct byte value
      * `[✅]`   Act: `fill_bytes(FillBytesParams, payload)` on the built source
      * `[✅]`   Assert: the success arm is extracted with `expect`; the bytes of `bytes.expose()` collected into a `HashSet` number more than the literal 1 written in the assertion
    * `[✅]`   `os_random_source_declares_its_adapter_and_interface_versions`
      * `[✅]`   Contract: `OsRandomSource::DECLARATION` is `RandomSourceDeclaration { source: RandomSourceKind::OperatingSystem, adapter_version: 1, interface_version: RANDOM_SOURCE_INTERFACE_VERSION }`, readable from the type before any instance exists
      * `[✅]`   Collaborators: none; no instance is built
      * `[✅]`   Arrange: none; the type alone
      * `[✅]`   Act: reading `OsRandomSource::DECLARATION`
      * `[✅]`   Assert: `source` matches `RandomSourceKind::OperatingSystem` under `matches!`; `adapter_version` equals the literal 1 written in the assertion; `interface_version` equals `RANDOM_SOURCE_INTERFACE_VERSION`
    * `[✅]`   `random_source_kinds_declares_only_real_members`
      * `[✅]`   Contract: the declared set holds the real members alone → `RANDOM_SOURCE_KINDS` contains `RandomSourceKind::OperatingSystem` and no `Mock(_)` member
      * `[✅]`   Collaborators: none; the declared set is read as a constant, no instance is built
      * `[✅]`   Arrange: none; the constant alone
      * `[✅]`   Act: reading `RANDOM_SOURCE_KINDS`
      * `[✅]`   Assert: every member is `RandomSourceKind::OperatingSystem` and none is `Mock(_)`
    * `[✅]`   `every_declared_kind_spells_a_distinct_name`
      * `[✅]`   Contract: each declared member has one total spelling → the `name()` spellings of the members of `RANDOM_SOURCE_KINDS` are pairwise distinct
      * `[✅]`   Collaborators: none; the declared set is read as a constant, no instance is built
      * `[✅]`   Arrange: none; the constant alone
      * `[✅]`   Act: collecting each member's `name()` into a `HashSet`
      * `[✅]`   Assert: the `HashSet`'s length equals the length of `RANDOM_SOURCE_KINDS`
    * `[✅]`   `try_new_returns_the_operating_system_source`
      * `[✅]`   Contract: any params → `Ok(OsRandomSource)`
      * `[✅]`   Collaborators: none; `OsRandomSourceConstructorParams` by its production value
      * `[✅]`   Arrange: `OsRandomSourceConstructorParams` by its production value
      * `[✅]`   Act: `OsRandomSource::try_new(OsRandomSourceConstructorParams)`
      * `[✅]`   Assert: `result.is_ok()` is true

  * `[✅]`   `construction`
    * `[✅]`   `OsRandomSource::try_new` is the concrete's only producer, and its only caller is the randomness factory, which returns it as `Box<dyn IRandomSourceAdapter>`; consumers read the declaration through `random/lookup_random_declaration` without constructing the source

  * `[✅]`   `adapters/random/src/os/mod.rs`
    * `[✅]`   Module declarations: `mod interface;`, `#[cfg(test)] mod mock;`, `pub(crate) mod provides;`, and `#[cfg(test)] mod test;`
    * `[✅]`   `impl OsRandomSource` with `pub fn try_new(_params: OsRandomSourceConstructorParams) -> OsRandomSourceTryNewReturn` returning `Ok(OsRandomSource)`
    * `[✅]`   `impl IRandomSourceAdapter for OsRandomSource` with `fn fill_bytes(&self, _params: FillBytesParams, payload: FillBytesPayload) -> FillBytesReturn`, which allocates `vec![0u8; payload.length]`, calls `getrandom::fill` on it, moves the buffer into a `Secret` by `let Ok(bytes) = Secret::try_new(SecretConstructorParams { value: buffer });`, and then matches the fill result into the two branches of the interaction spec
    * `[✅]`   Imports the factory's names from `crate::factory::provides`, `Secret` and `SecretConstructorParams` from `domain`, and this module's types from `interface`
    * `[✅]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[✅]`   `adapters/random/src/os/provides.rs`
    * `[✅]`   `pub(crate) use super::interface::*;` and `#[cfg(test)] pub(crate) use super::mock::*;`, nothing else, so the concrete is visible to the crate's factory and to nothing outside the crate

  * `[✅]`   `directionality`
    * `[✅]`   `os` depends on the `factory` module's surface through `crate::factory::provides`, on `domain`, and on `getrandom`; the `factory` module depends on `domain` and on `os`'s error type through `crate::os::provides`; among repository crates the crate depends on `crates/domain` alone, inward; nothing depends on the crate yet
    * `[✅]`   The mutual dependency between the `factory` module and `os` is the family form's recorded cycle: a concrete implements the factory's trait, the factory's error enum carries the concrete's error, and the factory function constructs the concrete

  * `[✅]`   `requirements`
    * `[✅]`   `adapters/random/Cargo.toml` carries exactly the tables and keys stated above, and `getrandom` is named nowhere in the crate outside `adapters/random/src/os`
    * `[✅]`   `FillBytesErrorReturn` and `OsRandomSourceFillBytesErrorReturn` derive `Debug`, `PartialEq`, and `Eq`
    * `[✅]`   `cargo check --all-targets --all-features` and `cargo fmt --check` complete without error; `cargo clippy --all-targets --all-features` reports nothing beyond the library target's unused-item warnings for the `os` concrete, which `random/factory` resolves by constructing the concrete
    * `[✅]`   `fill_bytes_returns_the_number_of_bytes_requested` passes
    * `[✅]`   `fill_bytes_returns_an_empty_draw_for_a_zero_length` passes
    * `[✅]`   `fill_bytes_draws_pairwise_distinct_segments_within_a_draw` passes (CR-05, the source's non-repetition)
    * `[✅]`   `fill_bytes_fills_a_draw_with_more_than_one_distinct_byte_value` passes (CR-05, the source fills what it is asked to fill)
    * `[✅]`   `os_random_source_declares_its_adapter_and_interface_versions` passes
    * `[✅]`   `random_source_kinds_declares_only_real_members` passes
    * `[✅]`   `every_declared_kind_spells_a_distinct_name` passes
    * `[✅]`   `RandomSourceDeclaration.source` is the member's typed `RandomSourceKind` and the declaration carries no duplicate text name
    * `[✅]`   `OsRandomSource::DECLARATION` is readable from the type before any instance exists
    * `[✅]`   The concrete has no settings module and no settings table
    * `[✅]`   `build_random_source_kind` returns the real `OperatingSystem` selection with no configuration
    * `[✅]`   A generator failure is returned as `FillBytesErrorReturn::OperatingSystem` holding `OsRandomSourceFillBytesErrorReturn::OperatingSystem` with the `getrandom::Error` unchanged, fixed by the error arm's type; the failure branch has no unit test, since the operating system's generator cannot be driven to fail from a test and the vendor is not mocked
    * `[✅]`   Code outside `adapters/random` naming `OsRandomSource` or anything under `os` fails to compile; the crate's public surface is the `factory` module's `provides`

* `[✅]`   `random/lookup_random_declaration` **The randomness family's declaration lookup: a total function from a member identifier to that member's declaration, so a level that holds an identifier reads the source kind, the adapter version, and the interface version the member fixes without constructing the member; an exhaustive branch on the identifier returns the member's own declaration; creates the `lookup_random_declaration` module**

  * `[✅]`   `objective`
    * `[✅]`   Problem: a value a randomness member's declaration fixes is known before the member is constructed, so a caller that holds the identifier configuration named reads the declaration through one family-owned function and never restates a member's declared value from its own knowledge of the member, and a caller outside the crate, which cannot name a member's declaration constant, reads it here alone
      * `[✅]`   Sources, the planning documents: the dependency map's `random/lookup_random_declaration` row, "The family's declaration lookup, returning a member's declaration without constructing the member", with its proof "a declaration read with no concrete constructed"; the Workspace and discipline bootstrap milestone's scope entry, "`random/lookup_random_declaration` reading declarations without constructing the concrete"; the technical requirements' Randomness family register row, "the family's declaration lookup, selection admission, and composition admission stand ahead of the factory"; the product requirements' Resolved Positions on the adapter family form
      * `[✅]`   Sources, `random/os`: its `IRandomSourceAdapter` element, "a member's declaration is read through `random/lookup_random_declaration` without constructing the member, never from the adapter"; its interaction spec, "`OsRandomSource::DECLARATION` ... read through `random/lookup_random_declaration` without an instance, never obtained from a constructed adapter"
      * `[✅]`   Sources, `random/os`: its `RandomSourceKind` element, "the mock selection carrying the failure mode its configuration names"; its `RandomSourceDeclaration` element, "the source is the member's typed identifier and the declaration carries no duplicate text name"
      * `[✅]`   Sources, the topics: `families.md`, "What a family owns", "Reading a declaration before construction", and "Ownership at the first consumer"; `composition.md`, "Nested calls build their inputs from what they receive (Rust)"
    * `[✅]`   Functional: the params carry the member identifier `RandomSourceKind`, the deps and the payload carry nothing, and the success return carries the `RandomSourceDeclaration` of the member the identifier names; the function reads each declaration from that member's own module and never from its caller
    * `[✅]`   Functional: for the operating-system member the function returns `OsRandomSource::DECLARATION`; under the `mocks` feature, for the mock member, it returns the declaration `MOCK_I_RANDOM_SOURCE_ADAPTER_DECLARATION` holds with `source` set to the identifier the caller looked up, so the mock declaration names the failure mode its selection carries
    * `[✅]`   Functional: the function constructs no adapter, reads no settings document, admits no settings, compares no declaration with another, and has no side effect; its error arm is uninhabited because the identifier is a closed enum the `match` covers
    * `[✅]`   Functional: a caller outside the `adapters/random` crate reads a declared member's declaration through this function alone, since the member's declaration constant is unreachable outside its family
    * `[✅]`   Non-functional: adding a further randomness concrete adds its module with its own declaration constant, a variant of `RandomSourceKind`, an entry in `RANDOM_SOURCE_KINDS`, and an arm in this function; this node's types name no concrete, and the function holds no literal of a name, version, identifier, or kind

  * `[✅]`   `role`
    * `[✅]`   Adapter-ring family-owned function of the randomness family: its own module beside `factory` and `os`, ahead of the family's selection admission, composition admission, and factory, in the trusted form because its params and payload carry no untrusted data; it reads no clock, no environment variable, no network, and no filesystem
    * `[✅]`   Owns `LookupRandomDeclarationDeps`, `LookupRandomDeclarationParams`, `LookupRandomDeclarationPayload`, `LookupRandomDeclarationSuccessReturn`, `LookupRandomDeclarationReturn`, `LookupRandomDeclarationFn`, and the function; every other type the function names is a member's or the family's
    * `[✅]`   As the first consumer that holds them by value and compares them, adds to `factory/interface.rs` the derives `RandomSourceKind`, `RandomSourceDeclaration`, and `MockIRandomSourceAdapterFailureMode` carry
    * `[✅]`   Does not author the mock concrete's declaration constant `MOCK_I_RANDOM_SOURCE_ADAPTER_DECLARATION`, which `random/os`'s `factory/mock.rs` element owns with the mock concrete
    * `[✅]`   Does not construct an adapter, select a member, admit a member's settings, or compare a declaration with another, which `random/factory`, and `random/admit_random_selection` do
    * `[✅]`   Does not carry a settings module, settings node, or settings table; the family's members have no settable value
    * `[✅]`   Does not carry a commit

  * `[✅]`   `module`
    * `[✅]`   Bounded context: the `lookup_random_declaration` module of `adapters/random`, holding the function's deps, params, payload, success, return, and signature types, the function, and their builders and function mock
    * `[✅]`   Files: `adapters/random/src/lib.rs`; `adapters/random/src/factory/interface.rs`, gaining the derives the role element states; `adapters/random/src/lookup_random_declaration/interface.rs`, `interaction.spec.md`, `mock.rs`, `test.rs`, `mod.rs`, and `provides.rs`; `adapters/random/tests/lookup_random_declaration_integration_test.rs`
    * `[✅]`   Outside: the members' declarations and what they state, the family's selection, declared set, and declared-selection builder, the selection admission, the composition admission, the factory function, and the configuration

  * `[✅]`   `deps`
    * `[✅]`   `os`, same crate, through `crate::os::provides`, the family's private path to its concrete: `OsRandomSource` and its inherent constant `OsRandomSource::DECLARATION`, as `random/os` authors them in `os/interface.rs`; direction inward, the family reaching its concrete and the concrete naming the family's `factory` module alone among the family's modules
    * `[✅]`   `factory`, same crate, through `crate::factory::provides`: `RandomSourceKind`, `RandomSourceDeclaration`, `RANDOM_SOURCE_INTERFACE_VERSION`, and `RANDOM_SOURCE_KINDS`, as `random/os` authors them in `factory/interface.rs`; under the `mocks` feature `MockIRandomSourceAdapterFailureMode`, as `random/os` authors it, and `MOCK_I_RANDOM_SOURCE_ADAPTER_DECLARATION`, the `pub(crate)` constant `random/os` authors in `factory/mock.rs`
    * `[✅]`   In `mock.rs` and `test.rs`: `build_random_source_declaration` and `RandomSourceDeclarationOverrides` through `crate::factory::provides`, as `random/os` authors them in `factory/mock.rs`
    * `[✅]`   Declared here: the derives named in the `factory/interface.rs` element
    * `[✅]`   `core::convert::Infallible`, standard library: the error arm of the return
    * `[✅]`   No reverse dependency: no module of the concrete names this module, and no later module of the family is named by it

  * `[✅]`   `context_slice`
    * `[✅]`   From `os`: `OsRandomSource::DECLARATION: RandomSourceDeclaration`, an associated `pub const` read by value
    * `[✅]`   From `factory`: `RandomSourceKind` with `OperatingSystem` and, under `#[cfg(any(test, feature = "mocks"))]`, `Mock(MockIRandomSourceAdapterFailureMode)`; `RandomSourceDeclaration` with the fields `source`, `adapter_version`, and `interface_version`; `RANDOM_SOURCE_INTERFACE_VERSION: u32`; `RANDOM_SOURCE_KINDS: &[RandomSourceKind]`; under the `mocks` feature `MockIRandomSourceAdapterFailureMode` with the variants `Succeeds` and `FillBytesRefused`, and `MOCK_I_RANDOM_SOURCE_ADAPTER_DECLARATION: RandomSourceDeclaration`, a `pub(crate)` constant
    * `[✅]`   From `core`: `Infallible`

  * `[✅]`   `adapters/random/src/lib.rs`
    * `[✅]`   The crate barrel gains `mod lookup_random_declaration;` beside the module declarations and `pub use lookup_random_declaration::provides::*;` beside the re-exports

  * `[✅]`   `adapters/random/src/factory/interface.rs`
    * `[✅]`   `RandomSourceKind` carries `#[derive(Clone, Copy, Debug, PartialEq, Eq)]`, so the lookup's params hold it by value and compare it
    * `[✅]`   `RandomSourceDeclaration` carries `#[derive(Clone, Copy, Debug, PartialEq, Eq)]`, so the lookup's success return holds it by value and a test asserts the whole result
    * `[✅]`   `MockIRandomSourceAdapterFailureMode`, under `#[cfg(any(test, feature = "mocks"))]`, carries `#[derive(Clone, Copy, Debug, PartialEq, Eq)]`, so `RandomSourceKind::Mock` derives the same traits

  * `[✅]`   `adapters/random/src/lookup_random_declaration/interface.rs`
    * `[✅]`   `LookupRandomDeclarationDeps`, the fieldless struct `pub struct LookupRandomDeclarationDeps;`, the deps slot, which carries no concrete's value
    * `[✅]`   `LookupRandomDeclarationParams`, a struct with `#[derive(Clone, Copy, Debug, PartialEq, Eq)]` and the field `pub identifier: RandomSourceKind`, the member whose declaration is read
    * `[✅]`   `LookupRandomDeclarationPayload`, the fieldless struct `pub struct LookupRandomDeclarationPayload;`, the payload slot, which carries no data the function operates on
    * `[✅]`   `LookupRandomDeclarationSuccessReturn`, a struct with `#[derive(Clone, Copy, Debug, PartialEq, Eq)]` and the field `pub declaration: RandomSourceDeclaration`
    * `[✅]`   `LookupRandomDeclarationReturn`, the alias `Result<LookupRandomDeclarationSuccessReturn, Infallible>`, the two-arm union whose error arm is uninhabited; `LookupRandomDeclarationFn`, the alias `fn(&LookupRandomDeclarationDeps, LookupRandomDeclarationParams, LookupRandomDeclarationPayload) -> LookupRandomDeclarationReturn`, the trusted form
    * `[✅]`   No `Default` on any type in this file
    * `[✅]`   Imports `RandomSourceKind` and `RandomSourceDeclaration` from `crate::factory::provides` and `core::convert::Infallible`

  * `[✅]`   `adapters/random/src/lookup_random_declaration/interaction.spec.md`
    * `[✅]`   `lookup_random_declaration(deps: &LookupRandomDeclarationDeps, params: LookupRandomDeclarationParams, payload: LookupRandomDeclarationPayload) -> LookupRandomDeclarationReturn`, the trusted form; `deps` and `payload` are not read; the decision is an exhaustive `match` on `params.identifier`, so a further member adds an arm
    * `[✅]`   Operating system: condition `params.identifier` is `RandomSourceKind::OperatingSystem`; no dependency call; outcome `Ok(LookupRandomDeclarationSuccessReturn { declaration: OsRandomSource::DECLARATION })`
    * `[✅]`   Mock, under `#[cfg(any(test, feature = "mocks"))]`: condition `params.identifier` is `RandomSourceKind::Mock(mode)`; no dependency call; outcome `Ok(LookupRandomDeclarationSuccessReturn { declaration: RandomSourceDeclaration { source: params.identifier, ..MOCK_I_RANDOM_SOURCE_ADAPTER_DECLARATION } })`, the declaration naming the identifier looked up, the mode included, and carrying the versions the mock concrete's constant holds
    * `[✅]`   Ordering: the matched branch returns; no side effect
    * `[✅]`   Callee dispositions, `random/os`: its interaction spec states no integration entry, so no entry passes through a function of this node or ends at one of its tests, this node calling none of the concrete's functions and reading `OsRandomSource::DECLARATION` alone
    * `[✅]`   Own entry, `the_operating_system_source_is_looked_up_to_the_declaration_it_owns`: condition `lookup_random_declaration` is called over `RandomSourceKind::OperatingSystem` by a caller outside the crate; outcome `Ok` whose declaration's `source` is `RandomSourceKind::OperatingSystem`, whose `adapter_version` is the literal 1, and whose `interface_version` is `RANDOM_SOURCE_INTERFACE_VERSION`; variation the operating-system identifier, whose declaration names a source that differs from any `Mock` kind, so a lookup that returns the mock member's declaration fails; edge that must survive the composition: the declaration reaches the outside caller through this function alone, the member's constant being unnameable outside the crate
    * `[✅]`   Own entry, `the_mock_source_is_looked_up_to_a_declaration_naming_the_identifier_it_was_given`: condition `lookup_random_declaration` is called by a caller outside the crate under the `mocks` feature over `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)` and over the first identifier of `RANDOM_SOURCE_KINDS`; outcome `Ok` whose declaration's `source` is the mock identifier with its failure mode and which differs from the declaration the declared member returns; variation the mock identifier carrying a mode other than the mode the mock concrete's constant holds, beside a declared member's identifier, so a branch that returns the constant's own `source`, or the declared member's declaration, for the mock identifier fails; edge that must survive the composition: the mock member's declaration is its own and the declared set never holds it
    * `[✅]`   Private surface: the real function `lookup_random_declaration`, which calls no function; the observable result is the `LookupRandomDeclarationReturn`; the branches are proven by the unit blocks of `test.rs`, and the node carries no private integration test
    * `[✅]`   Public surface: the crate's `lib.rs` re-exports `lookup_random_declaration::provides`, so an outside caller invokes `lookup_random_declaration` over a member identifier
      * `[✅]`   It observes the `LookupRandomDeclarationReturn`: the `Ok` arm's declaration through its public fields
      * `[✅]`   The entries proven are `the_operating_system_source_is_looked_up_to_the_declaration_it_owns` and `the_mock_source_is_looked_up_to_a_declaration_naming_the_identifier_it_was_given`
      * `[✅]`   The route is `lookup_random_declaration`, with no outer-edge collaborator

  * `[✅]`   `adapters/random/src/lookup_random_declaration/mock.rs`
    * `[✅]`   Module-level `#![allow(dead_code, clippy::too_many_arguments)]`, compiled under `#[cfg(any(test, feature = "mocks"))]`
    * `[✅]`   `LookupRandomDeclarationDepsOverrides`, the fieldless unit struct with `#[derive(Default)]`; `build_lookup_random_declaration_deps(_overrides: LookupRandomDeclarationDepsOverrides) -> LookupRandomDeclarationDeps`; no corruptions type and no invalidator, since deps are never serialized
    * `[✅]`   `LookupRandomDeclarationParamsOverrides`, `#[derive(Default)]`, the field `pub identifier: Option<RandomSourceKind>`; `build_lookup_random_declaration_params(overrides) -> LookupRandomDeclarationParams`, the identifier defaulting to `RandomSourceKind::OperatingSystem`; no corruptions type and no invalidator, since params are assembled in-crate from admitted values
    * `[✅]`   `LookupRandomDeclarationPayloadOverrides`, the fieldless unit struct with `#[derive(Default)]`; `build_lookup_random_declaration_payload(_overrides: LookupRandomDeclarationPayloadOverrides) -> LookupRandomDeclarationPayload`; no corruptions type and no invalidator, since a fieldless type has no rendering to corrupt
    * `[✅]`   `LookupRandomDeclarationSuccessReturnOverrides`, `#[derive(Default)]`, the field `pub declaration: Option<RandomSourceDeclaration>`; `build_lookup_random_declaration_success_return(overrides) -> LookupRandomDeclarationSuccessReturn`, the declaration defaulting to `build_random_source_declaration(Default::default())`; no corruptions type and no invalidator, since a declaration is a constant and never arrives as untrusted data
    * `[✅]`   `mock_lookup_random_declaration(_deps: &LookupRandomDeclarationDeps, _params: LookupRandomDeclarationParams, _payload: LookupRandomDeclarationPayload) -> LookupRandomDeclarationReturn`, the production type `LookupRandomDeclarationFn`, returning `Ok(build_lookup_random_declaration_success_return(Default::default()))`
    * `[✅]`   Dispositions: the return alias is used by its production value, the error arm being `Infallible`; `LookupRandomDeclarationFn` is the alias the function mock is typed by
    * `[✅]`   Imports this module's types from `super::interface`, and `RandomSourceKind`, `RandomSourceDeclaration`, `build_random_source_declaration`, and `RandomSourceDeclarationOverrides` from `crate::factory::provides`

  * `[✅]`   `adapters/random/src/lookup_random_declaration/test.rs`
    * `[✅]`   No module-level `allow`; the blocks call no `unwrap`, `expect`, or `panic!`; imports `lookup_random_declaration` from `super`, every type of `super::interface`, every builder and overrides type of `super::mock`, and `RandomSourceKind`, `RandomSourceDeclarationOverrides`, `build_random_source_declaration`, `MockIRandomSourceAdapterFailureMode`, and `RANDOM_SOURCE_INTERFACE_VERSION` from `crate::factory::provides`; the file names no member's declaration constant, stating each declared value as an independent literal
    * `[✅]`   Each block reads the declaration from the `Ok` arm through the irrefutable pattern `let Ok(success) = result;`, the error arm being `Infallible`
    * `[✅]`   Each block builds its deps with `build_lookup_random_declaration_deps`, its params with `build_lookup_random_declaration_params` naming the identifier the block names, its payload with `build_lookup_random_declaration_payload`, and makes the single call to `lookup_random_declaration`; no block constructs an adapter
    * `[✅]`   `the_operating_system_source_returns_the_declaration_its_member_owns`
      * `[✅]`   Contract: the identifier `RandomSourceKind::OperatingSystem` → `Ok` holding the operating-system member's declaration
      * `[✅]`   Arrange: the identifier `OperatingSystem`, a member whose declaration names itself as the source, so a branch that returned the mock concrete's declaration, whose `source` is a `Mock` kind, fails
      * `[✅]`   Act: `lookup_random_declaration`
      * `[✅]`   Assert: the result equals `Ok(build_lookup_random_declaration_success_return(...))` built with the declaration `build_random_source_declaration` returns for the overrides `source` `RandomSourceKind::OperatingSystem`, `adapter_version` the literal 1, and `interface_version` `RANDOM_SOURCE_INTERFACE_VERSION`
    * `[✅]`   `the_mock_source_returns_a_declaration_naming_the_identifier_it_was_given`
      * `[✅]`   Contract: the identifier `RandomSourceKind::Mock(mode)` → `Ok` holding a declaration whose `source` is that identifier and whose versions are the mock concrete's
      * `[✅]`   Arrange: the identifier `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)`, a mode that differs from the `Succeeds` mode the mock concrete's constant holds, so a branch that returned the constant unchanged, or the operating-system member's declaration, fails
      * `[✅]`   Act: `lookup_random_declaration`
      * `[✅]`   Assert: the result equals `Ok(build_lookup_random_declaration_success_return(...))` built with the declaration `build_random_source_declaration` returns for the overrides `source` `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)`, `adapter_version` the literal 1, and `interface_version` `RANDOM_SOURCE_INTERFACE_VERSION`
    * `[✅]`   Every test block carries the full `Contract`, `Arrange`, `Act`, `Assert` header and the inline markers, and ends with a comment stating the test's name

  * `[✅]`   `construction`
    * `[✅]`   `LookupRandomDeclarationSuccessReturn` is produced only by `lookup_random_declaration` and its builder; a declaration is produced only by the member that owns its constant
    * `[✅]`   A caller holding a member identifier calls `lookup_random_declaration` and receives the member's declaration; no caller restates a member's declared value from its own knowledge of the member, and no caller reads a declaration from a constructed adapter

  * `[✅]`   `adapters/random/src/lookup_random_declaration/mod.rs`
    * `[✅]`   Module declarations: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, `pub mod provides;`, and `#[cfg(test)] mod test;`
    * `[✅]`   `pub fn lookup_random_declaration(_deps: &LookupRandomDeclarationDeps, params: LookupRandomDeclarationParams, _payload: LookupRandomDeclarationPayload) -> LookupRandomDeclarationReturn`, an exhaustive `match` on `params.identifier`, each arm returning `Ok(LookupRandomDeclarationSuccessReturn { declaration })`: the `RandomSourceKind::OperatingSystem` arm with `OsRandomSource::DECLARATION`, and the `RandomSourceKind::Mock(_)` arm, under `#[cfg(any(test, feature = "mocks"))]`, with `RandomSourceDeclaration { source: params.identifier, ..MOCK_I_RANDOM_SOURCE_ADAPTER_DECLARATION }`; no `unwrap`, `expect`, `panic!`, numeric `as`, or `unsafe`
    * `[✅]`   Imports this module's names from `interface`, `RandomSourceKind` from `crate::factory::provides`, under the `mocks` gate `RandomSourceDeclaration` and `MOCK_I_RANDOM_SOURCE_ADAPTER_DECLARATION` from the same path, and `OsRandomSource` from `crate::os::provides`

  * `[✅]`   `adapters/random/src/lookup_random_declaration/provides.rs`
    * `[✅]`   `pub use super::lookup_random_declaration;`, the function at public visibility since the crate's barrel re-exports it, `pub use super::interface::*;`, and `#[cfg(any(test, feature = "mocks"))] pub use super::mock::*;`

  * `[✅]`   `adapters/random/tests/lookup_random_declaration_integration_test.rs`
    * `[✅]`   The file is gated by `#![cfg(feature = "mocks")]`, so it compiles only where the crate's builders exist; no module-level `allow`
    * `[✅]`   Imports from the crate's public surface and its `mocks` feature: `lookup_random_declaration`, `build_lookup_random_declaration_deps`, `LookupRandomDeclarationDepsOverrides`, `build_lookup_random_declaration_params`, `LookupRandomDeclarationParamsOverrides`, `build_lookup_random_declaration_payload`, `LookupRandomDeclarationPayloadOverrides`, `RandomSourceKind`, `MockIRandomSourceAdapterFailureMode`, `RANDOM_SOURCE_INTERFACE_VERSION`, and `RANDOM_SOURCE_KINDS`
    * `[✅]`   Each block builds its deps, params, and payload through their builders, makes each call to `lookup_random_declaration` through the crate's public surface, reads the declaration from the `Ok` arm through the irrefutable pattern `let Ok(success) = result;`, and reads it only through its public fields; no block names a member's declaration constant
    * `[✅]`   `lookup_random_declaration_returns_the_operating_system_source_its_own_declaration_for_an_outside_caller`
      * `[✅]`   Contract: own entry `the_operating_system_source_is_looked_up_to_the_declaration_it_owns`
      * `[✅]`   Arrange: the identifier `RandomSourceKind::OperatingSystem`
      * `[✅]`   Act: `lookup_random_declaration` over the identifier
      * `[✅]`   Assert: the result is `Ok`, the declaration's `source` equals `RandomSourceKind::OperatingSystem`, its `adapter_version` equals the literal 1, and its `interface_version` equals `RANDOM_SOURCE_INTERFACE_VERSION`
      * `[✅]`   Boundary: the public `lookup_random_declaration`
      * `[✅]`   Mocked: nothing
    * `[✅]`   `lookup_random_declaration_returns_the_mock_source_a_declaration_naming_its_identifier_for_an_outside_caller`
      * `[✅]`   Contract: own entry `the_mock_source_is_looked_up_to_a_declaration_naming_the_identifier_it_was_given`
      * `[✅]`   Arrange: the identifier `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)`, and the first identifier of `RANDOM_SOURCE_KINDS`
      * `[✅]`   Act: `lookup_random_declaration` over each
      * `[✅]`   Assert: the mock result is `Ok` whose declaration's `source` equals `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)` and which is not equal to the declaration the declared member's result holds
      * `[✅]`   Boundary: the public `lookup_random_declaration`
      * `[✅]`   Mocked: nothing, because the mock member is the family's own mock concrete
    * `[✅]`   Reports its enumeration: each entry of the spec's public surface, by the block above that proves it

  * `[✅]`   `directionality`
    * `[✅]`   `lookup_random_declaration` depends on the concrete's declaration through the family's private path and on the family's `factory` module; the concrete and the modules beneath it name nothing in this module; no cycle
    * `[✅]`   The dependency map's `random/admit_random_selection` row depend on this function and its types

  * `[✅]`   `requirements`
    * `[✅]`   `adapters/random/src/lib.rs` carries the lines the elements above state, `factory/interface.rs` carries the derives, and the files of `lookup_random_declaration/` carry the elements above
    * `[✅]`   `cargo check --workspace --all-targets --all-features`, `cargo fmt --all --check`, and `cargo deny check` complete without error; `cargo clippy --workspace --all-targets --all-features` reports no warning
    * `[✅]`   `the_operating_system_source_returns_the_declaration_its_member_owns` and `the_mock_source_returns_a_declaration_naming_the_identifier_it_was_given` pass, each asserting the whole result by `assert_eq!`
    * `[✅]`   In `lookup_random_declaration_integration_test.rs`, `lookup_random_declaration_returns_the_operating_system_source_its_own_declaration_for_an_outside_caller` and `lookup_random_declaration_returns_the_mock_source_a_declaration_naming_its_identifier_for_an_outside_caller` pass
    * `[✅]`   `RandomSourceDeclaration.source` remains the member's typed `RandomSourceKind` and the declaration carries no duplicate text name

* `[✅]`   `random/admit_random_selection` **The randomness family's selection admission: receives the admitted member identifier and the untrusted settings document, refuses a document that supplies any setting with the family's own error naming the member, since no member has a settable value, and returns the selection that names the member; creates the `admit_random_selection` module and adds the family's dependency on `serde_json`**

  * `[✅]`   `objective`
    * `[✅]`   Problem: configuration names a randomness member, the family's owner admits that name into the member's identifier, and the family turns the identifier and the untrusted settings document into the selection every later consumer receives, so no configuration function, layer, or composition root builds a selection from its own knowledge of a member; no randomness member has a settable value, so the admission carries no settings and refuses a document that supplies one
      * `[✅]`   Sources, the planning documents: the dependency map's `random/admit_random_selection` row, "receives the admitted member identifier and the untrusted settings document, refuses a document that supplies any setting, with the family's own error naming the member, since no member has a settable value, and returns the selection naming the member; every callee error is carried unchanged", with its proof "each declared member's selection returned, and a document supplying any setting refused with the family's own error naming the member"; the Workspace and discipline bootstrap milestone's scope entry, "`random/admit_random_selection` refusing every supplied setting and returning the selection without settings"; the technical requirements' Randomness family register row, "each concrete's settings-document guard and settings admission where the concrete has settable values, with no settings module otherwise, and the family's declaration lookup, selection admission, and composition admission stand ahead of the factory"; the product requirements' Feature Details on adapter composition, "family selection admission narrows the untrusted settings document through the concrete's guard and settings admission only for concretes with settable values, refuses every supplied setting for concretes with none"
      * `[✅]`   Sources, `random/os`: its `role` element, "does not carry a settings module, settings node, or settings table; the concrete has no settable value, so the real selection carries no settings"; its `RandomSourceKind` element, "the typed kind the family's declarations and selections carry ... the mock selection carrying the failure mode its configuration names"; its `RANDOM_SOURCE_KINDS` element, "containing only the real `OperatingSystem` member"; its `build_random_source_kind` element, "the family's declared-selection builder ... whose real `OperatingSystem` arm returns the selection with no configuration"
      * `[✅]`   Sources, `random/lookup_random_declaration`: its `factory/interface.rs` element, the derives `RandomSourceKind`, `RandomSourceDeclaration`, and `MockIRandomSourceAdapterFailureMode` carry, `Clone`, `Copy`, `Debug`, `PartialEq`, and `Eq`
      * `[✅]`   Sources, `families.md`, "Selecting a concrete from configuration", "A concrete's settings-document type is its settings type with every field optional; the guard narrows to it and refuses a document carrying a key that names no field of the settings type", and "Where a concrete's settings live in a document", "A concrete with no settings module has no table": a setting is supplied only as an entry of an object, a member with no settings has no table, so a document that is `Value::Null` or an object holding no entry supplies none, and a value that is not an object is not a settings document and is refused
      * `[✅]`   Sources, the topics: `families.md`, "Selecting a concrete from configuration" and "A concrete's settings precede the concrete"; `composition.md`, "The signature" and "Parameter jurisdiction — what is trusted vs. proven"; `types.md`, "`unknown` — forbidden as a dodge, required at a boundary"; `errors-and-returns.md`, "A family's errors"
    * `[✅]`   Functional: the params carry the member identifier `RandomSourceKind` and nothing else, the payload is the untrusted settings document `serde_json::Value`, and the success return carries the selection, a `RandomSourceKind` equal to the identifier the params carry
    * `[✅]`   Functional: a document supplies no setting when it is `Value::Null` or a `Value::Object` that holds no entry; every other document, an object holding an entry or a value that is not an object, supplies a setting and is refused with the family's own error `SettingsSupplied`, which carries the identifier looked at
    * `[✅]`   Functional: the refusal and the selection are the same for every member of `RandomSourceKind`, the mock member included under the `mocks` feature, which returns the selection `RandomSourceKind::Mock(mode)` carrying the failure mode it was given
    * `[✅]`   Functional: the function constructs no adapter, reads no declaration, calls no function, and has no side effect; a callee error does not exist because no callee is called
    * `[✅]`   Non-functional: adding a further randomness concrete adds an arm to this function's exhaustive `match` on the identifier, and a variant to `AdmitRandomSelectionErrorReturn` where the concrete's own settings can be refused; this node's types name no concrete, and the function holds no literal of a member's name, version, or kind
  * `[✅]`   `role`
    * `[✅]`   Adapter-ring family-owned function of the randomness family: its own module beside `factory`, `os`, and `lookup_random_declaration`, ahead of the family's composition admission and factory, in the validating form because its payload is the untrusted settings document; it reads no clock, no environment variable, no network, and no filesystem
    * `[✅]`   Owns `AdmitRandomSelectionDeps`, `AdmitRandomSelectionParams`, `AdmitRandomSelectionSuccessReturn`, `AdmitRandomSelectionErrorReturn`, `AdmitRandomSelectionReturn`, `AdmitRandomSelectionFn`, and the function; every other type the function names is a member's or the family's, and the payload type is `serde_json::Value`
    * `[✅]`   Does not admit configuration text into a member identifier, which the family's owner does before this function's params exist; does not construct an adapter, read a declaration, narrow a document through a settings guard, admit a member's settings, admit a composition, or build a list of members
    * `[✅]`   Does not carry a settings module, settings node, or settings table
    * `[✅]`   Does not carry a commit

  * `[✅]`   `module`
    * `[✅]`   Bounded context: the `admit_random_selection` module of `adapters/random`, holding the function's deps, params, success, error, return, and signature types, the function, and their builders and function mock
    * `[✅]`   Files: `adapters/random/Cargo.toml`; `adapters/random/src/lib.rs`; `adapters/random/src/admit_random_selection/interface.rs`, `interaction.spec.md`, `mock.rs`, `test.rs`, `mod.rs`, and `provides.rs`; `adapters/random/tests/admit_random_selection_integration_test.rs`
    * `[✅]`   Outside: the members' declarations and what they state, the family's declared set and declared-selection builder, the declaration lookup, the composition admission, the factory function, the admission of configuration text into an identifier, and the configuration

  * `[✅]`   `deps`
    * `[✅]`   `factory`, same crate, through `crate::factory::provides`, the family's `factory` module: `RandomSourceKind`, as `random/os` authors it in `factory/interface.rs` with the derives `random/lookup_random_declaration` adds, and `RANDOM_SOURCE_KINDS`, as `random/os` authors it; under the `mocks` feature `MockIRandomSourceAdapterFailureMode` and `build_random_source_kind`, as `random/os` authors them in `factory/mock.rs`; direction inward, the family's function reading the family's own types
    * `[✅]`   `serde_json`, external crate, MIT OR Apache-2.0, pinned `1.0.154`, runtime dependency; supplies `Value`, the boundary type of the untrusted settings document, and `Map`, the object a document is built from in tests
    * `[✅]`   No item of `os` or of `lookup_random_declaration` is named by this module; the function reads no declaration
    * `[✅]`   No reverse dependency: no module of the concrete or of the declaration lookup names this module, and no later module of the family is named by it

  * `[✅]`   `context_slice`
    * `[✅]`   From `factory`: `RandomSourceKind` with the variant `OperatingSystem` and, under `#[cfg(any(test, feature = "mocks"))]`, the variant `Mock(MockIRandomSourceAdapterFailureMode)`; `RANDOM_SOURCE_KINDS: &[RandomSourceKind]`; under the `mocks` feature `MockIRandomSourceAdapterFailureMode` with the variants `Succeeds` and `FillBytesRefused`, and `build_random_source_kind`, the declared-selection builder returning a `RandomSourceKind`
    * `[✅]`   From `serde_json`: `Value` with the variants `Null`, `Bool`, `Number`, `String`, `Array`, and `Object`; `Map<String, Value>` with `new`, `insert`, and `is_empty`

  * `[✅]`   `adapters/random/Cargo.toml`
    * `[✅]`   `[dependencies]` gains `serde_json = "1.0.154"`, default features, beside the entries the manifest already holds
    * `[✅]`   Every other table and key of the manifest keeps its value

  * `[✅]`   `adapters/random/src/lib.rs`
    * `[✅]`   The crate barrel gains `mod admit_random_selection;` beside the module declarations and `pub use admit_random_selection::provides::*;` beside the re-exports

  * `[✅]`   `adapters/random/src/admit_random_selection/interface.rs`
    * `[✅]`   `AdmitRandomSelectionDeps`, the fieldless struct `pub struct AdmitRandomSelectionDeps;`, the deps slot, which carries no concrete's value and no collaborator
    * `[✅]`   `AdmitRandomSelectionParams`, a struct with `#[derive(Clone, Copy, Debug, PartialEq, Eq)]` and the field `pub identifier: RandomSourceKind`, the member admitted from configuration
    * `[✅]`   `AdmitRandomSelectionSuccessReturn`, a struct with `#[derive(Clone, Copy, Debug, PartialEq, Eq)]` and the field `pub selection: RandomSourceKind`, the selection naming the member, which carries no settings
    * `[✅]`   `AdmitRandomSelectionErrorReturn`, an enum with `#[derive(Clone, Copy, Debug, PartialEq, Eq)]` and the variant `SettingsSupplied { identifier: RandomSourceKind }`, the family's own error: the document supplied a setting and the member the identifier names has none
    * `[✅]`   `AdmitRandomSelectionReturn`, the alias `Result<AdmitRandomSelectionSuccessReturn, AdmitRandomSelectionErrorReturn>`; `AdmitRandomSelectionFn`, the alias `fn(&AdmitRandomSelectionDeps, AdmitRandomSelectionParams, serde_json::Value) -> AdmitRandomSelectionReturn`, the validating form, the payload slot the untrusted type
    * `[✅]`   No `Default` on any type in this file
    * `[✅]`   Imports `RandomSourceKind` from `crate::factory::provides`; names `serde_json::Value` by its full path in the signature alias

  * `[✅]`   `adapters/random/src/admit_random_selection/interaction.spec.md`
    * `[✅]`   `admit_random_selection(deps: &AdmitRandomSelectionDeps, params: AdmitRandomSelectionParams, payload: serde_json::Value) -> AdmitRandomSelectionReturn`, the validating form; `deps` is not read; the decision is an exhaustive `match` on `params.identifier`, so a further member adds an arm; in every arm the document supplies no setting when the payload is `Value::Null` or a `Value::Object` that holds no entry, and supplies a setting for every other payload
    * `[✅]`   Operating system, no setting supplied: condition `params.identifier` is `RandomSourceKind::OperatingSystem` and the payload is `Value::Null` or an empty `Value::Object`; no dependency call; outcome `Ok(AdmitRandomSelectionSuccessReturn { selection: RandomSourceKind::OperatingSystem })`
    * `[✅]`   Operating system, a setting supplied: condition `params.identifier` is `RandomSourceKind::OperatingSystem` and the payload is a `Value::Object` that holds an entry, or a `Value::Bool`, `Value::Number`, `Value::String`, or `Value::Array`; no dependency call; outcome `Err(AdmitRandomSelectionErrorReturn::SettingsSupplied { identifier: RandomSourceKind::OperatingSystem })`
    * `[✅]`   Mock, under `#[cfg(any(test, feature = "mocks"))]`, no setting supplied: condition `params.identifier` is `RandomSourceKind::Mock(mode)` and the payload is `Value::Null` or an empty `Value::Object`; no dependency call; outcome `Ok(AdmitRandomSelectionSuccessReturn { selection: RandomSourceKind::Mock(mode) })`, the selection carrying the failure mode it was given
    * `[✅]`   Mock, under `#[cfg(any(test, feature = "mocks"))]`, a setting supplied: condition `params.identifier` is `RandomSourceKind::Mock(mode)` and the payload supplies a setting; no dependency call; outcome `Err(AdmitRandomSelectionErrorReturn::SettingsSupplied { identifier: RandomSourceKind::Mock(mode) })`
    * `[✅]`   Ordering: the matched branch returns; no side effect; the payload is inspected and not retained
    * `[✅]`   Callee dispositions: no entry of `random/os` or of `random/lookup_random_declaration` passes through `admit_random_selection` or ends at one of its tests, since the function calls none of their functions and names none of their items
    * `[✅]`   Own entry, `every_declared_member_is_admitted_to_the_selection_naming_it`: condition `admit_random_selection` is called by a caller outside the crate over each identifier of `RANDOM_SOURCE_KINDS`, with a document that is `Value::Null` and with a document that is an empty `Value::Object`; outcome `Ok` whose selection equals the identifier; variation the documents that supply no setting, `Value::Null` and an empty `Value::Object`, so a check that refuses either fails; edge that must survive the composition: the selection reaches the outside caller through this function alone, and the declared set holds no mock member
    * `[✅]`   Own entry, `the_mock_member_is_admitted_to_the_selection_carrying_its_failure_mode`: condition `admit_random_selection` is called by a caller outside the crate under the `mocks` feature over `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)` and over `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::Succeeds)`, with a document that is `Value::Null` and with a document that is an empty `Value::Object`; outcome `Ok` whose selection is the identifier looked at, its failure mode included; variation the failure modes `FillBytesRefused` and `Succeeds`, and the documents `Value::Null` and an empty `Value::Object`, so a branch that returns one fixed mode, or the real member's selection, fails; edge that must survive the composition: the mock selection keeps the failure mode its configuration names
    * `[✅]`   Own entry, `a_document_supplying_a_setting_is_refused_naming_the_member`: condition `admit_random_selection` is called by a caller outside the crate over each identifier of `RANDOM_SOURCE_KINDS` and, under the `mocks` feature, over `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)`, with a document that is a `Value::Object` holding an entry, with a document that is an empty `Value::Array`, with a document that is a `Value::Bool`, with a document that is a `Value::Number`, and with a document that is a `Value::String`; outcome `Err(AdmitRandomSelectionErrorReturn::SettingsSupplied { identifier })` naming the identifier looked at; variation the object holding an entry, the empty array, a boolean, a number, and a string, where the empty array is a value an emptiness check on the document would admit, so a check that refuses only objects, or that omits one of these kinds, fails; edge that must survive the composition: the refusal names the member, the mock member's failure mode included
    * `[✅]`   Private surface: the real function `admit_random_selection`, which calls no function; the observable result is the `AdmitRandomSelectionReturn`; the branches are proven by the unit blocks of `test.rs`, and the node carries no private integration test
    * `[✅]`   Public surface: the crate's `lib.rs` re-exports `admit_random_selection::provides`, so an outside caller invokes `admit_random_selection` over a member identifier and a settings document
      * `[✅]`   It observes the `AdmitRandomSelectionReturn`: the `Ok` arm's selection and the `Err` arm's identifier through their public fields
      * `[✅]`   The entries proven are `every_declared_member_is_admitted_to_the_selection_naming_it`, `the_mock_member_is_admitted_to_the_selection_carrying_its_failure_mode`, and `a_document_supplying_a_setting_is_refused_naming_the_member`
      * `[✅]`   The route is `admit_random_selection`, with no outer-edge collaborator

  * `[✅]`   `adapters/random/src/admit_random_selection/mock.rs`
    * `[✅]`   Module-level `#![allow(dead_code, clippy::too_many_arguments)]`, compiled under `#[cfg(any(test, feature = "mocks"))]`
    * `[✅]`   `AdmitRandomSelectionDepsOverrides`, the fieldless unit struct with `#[derive(Default)]`; `build_admit_random_selection_deps(_overrides: AdmitRandomSelectionDepsOverrides) -> AdmitRandomSelectionDeps`; no corruptions type and no invalidator, since deps are never serialized
    * `[✅]`   `AdmitRandomSelectionParamsOverrides`, `#[derive(Default)]`, the field `pub identifier: Option<RandomSourceKind>`; `build_admit_random_selection_params(overrides) -> AdmitRandomSelectionParams`, the identifier defaulting to `RandomSourceKind::OperatingSystem`; no corruptions type and no invalidator, since params are assembled in-crate from an admitted identifier
    * `[✅]`   `AdmitRandomSelectionSuccessReturnOverrides`, `#[derive(Default)]`, the field `pub selection: Option<RandomSourceKind>`; `build_admit_random_selection_success_return(overrides) -> AdmitRandomSelectionSuccessReturn`, the selection defaulting to `build_random_source_kind(RandomSourceKind::OperatingSystem, Default::default())`; no corruptions type and no invalidator, since a selection is never untrusted data
    * `[✅]`   `mock_admit_random_selection(_deps: &AdmitRandomSelectionDeps, _params: AdmitRandomSelectionParams, _payload: serde_json::Value) -> AdmitRandomSelectionReturn`, the production type `AdmitRandomSelectionFn`, returning `Ok(build_admit_random_selection_success_return(Default::default()))`
    * `[✅]`   Dispositions: the settings document is `serde_json::Value`, an imported type with no builder here, so a test writes it through `Value`'s own variants; `AdmitRandomSelectionErrorReturn` is an enum used by its production value; `AdmitRandomSelectionFn` is the alias the function mock is typed by
    * `[✅]`   Imports this module's types from `super::interface`, `RandomSourceKind` and `build_random_source_kind` from `crate::factory::provides`, and `serde_json::Value` by its full path

  * `[✅]`   `adapters/random/src/admit_random_selection/test.rs`
    * `[✅]`   No module-level `allow`; the blocks call no `unwrap`, `expect`, or `panic!`; imports `admit_random_selection` from `super`, every type of `super::interface`, every builder and overrides type of `super::mock`, `RandomSourceKind` and `MockIRandomSourceAdapterFailureMode` from `crate::factory::provides`, and `Map` and `Value` from `serde_json`; the file names no member's declaration constant
    * `[✅]`   Each block builds its deps with `build_admit_random_selection_deps`, its params with `build_admit_random_selection_params` naming the identifier the block names, writes its payload through `Value`'s own variants, and makes the single call to `admit_random_selection`; a document holding an entry is `Value::Object` over a `Map` into which the key `unsupported_setting` is inserted with the value `Value::Bool(true)`; a boolean document is `Value::Bool(true)`, a number document is `Value::from(7_u64)`, and a string document is `Value::String(String::from("text"))`; no block constructs an adapter
    * `[✅]`   Each block asserts the whole result by `assert_eq!` against an independent expectation: a success against `Ok(build_admit_random_selection_success_return(...))` built with the selection override the block names, and a refusal against `Err(AdmitRandomSelectionErrorReturn::SettingsSupplied { identifier })` naming the identifier the block names
    * `[✅]`   `the_operating_system_member_with_an_absent_document_returns_its_own_selection`
      * `[✅]`   Contract: the identifier `RandomSourceKind::OperatingSystem` with the payload `Value::Null` → `Ok` holding the selection `RandomSourceKind::OperatingSystem`
      * `[✅]`   Arrange: the identifier `OperatingSystem` and the payload `Value::Null`, a document that supplies no setting and is not an object, so a check that refuses a document that is not an object fails
      * `[✅]`   Act: `admit_random_selection`
      * `[✅]`   Assert: the result equals `Ok(build_admit_random_selection_success_return(...))` built with the selection override `RandomSourceKind::OperatingSystem`
    * `[✅]`   `the_operating_system_member_with_an_empty_object_document_returns_its_own_selection`
      * `[✅]`   Contract: the identifier `RandomSourceKind::OperatingSystem` with the payload an empty `Value::Object` → `Ok` holding the selection `RandomSourceKind::OperatingSystem`
      * `[✅]`   Arrange: the identifier `OperatingSystem` and the payload `Value::Object` over `Map::new()`, a document that supplies no setting and is not null, so a check that refuses every document but `Value::Null` fails
      * `[✅]`   Act: `admit_random_selection`
      * `[✅]`   Assert: the result equals `Ok(build_admit_random_selection_success_return(...))` built with the selection override `RandomSourceKind::OperatingSystem`
    * `[✅]`   `the_mock_member_with_an_absent_document_returns_a_selection_carrying_its_failure_mode`
      * `[✅]`   Contract: the identifier `RandomSourceKind::Mock(mode)` with the payload `Value::Null` → `Ok` holding the selection `RandomSourceKind::Mock(mode)`
      * `[✅]`   Arrange: the identifier `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)` and the payload `Value::Null`, a mode and a member that differ from the operating-system member, so a branch that returns the real member's selection fails
      * `[✅]`   Act: `admit_random_selection`
      * `[✅]`   Assert: the result equals `Ok(build_admit_random_selection_success_return(...))` built with the selection override `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)`
    * `[✅]`   `the_operating_system_member_with_a_document_holding_an_entry_is_refused_naming_the_member`
      * `[✅]`   Contract: the identifier `RandomSourceKind::OperatingSystem` with a payload that is a `Value::Object` holding an entry → `Err` `SettingsSupplied` naming `RandomSourceKind::OperatingSystem`
      * `[✅]`   Arrange: the identifier `OperatingSystem` and the payload `Value::Object` over a `Map` holding the key `unsupported_setting`, a document that supplies a setting, so a branch that admits a document that is an object fails
      * `[✅]`   Act: `admit_random_selection`
      * `[✅]`   Assert: the result equals `Err(AdmitRandomSelectionErrorReturn::SettingsSupplied { identifier: RandomSourceKind::OperatingSystem })`
    * `[✅]`   `the_operating_system_member_with_an_empty_array_document_is_refused_naming_the_member`
      * `[✅]`   Contract: the identifier `RandomSourceKind::OperatingSystem` with a payload that is not an object and is not null → `Err` `SettingsSupplied` naming `RandomSourceKind::OperatingSystem`
      * `[✅]`   Arrange: the identifier `OperatingSystem` and the payload `Value::Array` over an empty `Vec`, a value an emptiness check on the document would admit, so a branch that admits an empty document of any shape fails
      * `[✅]`   Act: `admit_random_selection`
      * `[✅]`   Assert: the result equals `Err(AdmitRandomSelectionErrorReturn::SettingsSupplied { identifier: RandomSourceKind::OperatingSystem })`
    * `[✅]`   `the_mock_member_with_a_document_holding_an_entry_is_refused_naming_the_member_and_its_failure_mode`
      * `[✅]`   Contract: the identifier `RandomSourceKind::Mock(mode)` with a payload that is a `Value::Object` holding an entry → `Err` `SettingsSupplied` naming `RandomSourceKind::Mock(mode)`
      * `[✅]`   Arrange: the identifier `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)` and the payload `Value::Object` over a `Map` holding the key `unsupported_setting`, so a refusal that names the real member fails
      * `[✅]`   Act: `admit_random_selection`
      * `[✅]`   Assert: the result equals `Err(AdmitRandomSelectionErrorReturn::SettingsSupplied { identifier: RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused) })`
    * `[✅]`   `the_operating_system_member_with_a_boolean_document_is_refused_naming_the_member`
      * `[✅]`   Contract: the identifier `RandomSourceKind::OperatingSystem` with the payload `Value::Bool` → `Err` `SettingsSupplied` naming `RandomSourceKind::OperatingSystem`
      * `[✅]`   Arrange: the identifier `OperatingSystem` and the payload `Value::Bool(true)`, a value that is not an object and holds no entry, so a branch that admits a boolean fails
      * `[✅]`   Act: `admit_random_selection`
      * `[✅]`   Assert: the result equals `Err(AdmitRandomSelectionErrorReturn::SettingsSupplied { identifier: RandomSourceKind::OperatingSystem })`
    * `[✅]`   `the_operating_system_member_with_a_number_document_is_refused_naming_the_member`
      * `[✅]`   Contract: the identifier `RandomSourceKind::OperatingSystem` with the payload `Value::Number` → `Err` `SettingsSupplied` naming `RandomSourceKind::OperatingSystem`
      * `[✅]`   Arrange: the identifier `OperatingSystem` and the payload `Value::from(7_u64)`, a value that is not an object and holds no entry, so a branch that admits a number fails
      * `[✅]`   Act: `admit_random_selection`
      * `[✅]`   Assert: the result equals `Err(AdmitRandomSelectionErrorReturn::SettingsSupplied { identifier: RandomSourceKind::OperatingSystem })`
    * `[✅]`   `the_operating_system_member_with_a_string_document_is_refused_naming_the_member`
      * `[✅]`   Contract: the identifier `RandomSourceKind::OperatingSystem` with the payload `Value::String` → `Err` `SettingsSupplied` naming `RandomSourceKind::OperatingSystem`
      * `[✅]`   Arrange: the identifier `OperatingSystem` and the payload `Value::String(String::from("text"))`, a value that is not an object and holds no entry, so a branch that admits a string fails
      * `[✅]`   Act: `admit_random_selection`
      * `[✅]`   Assert: the result equals `Err(AdmitRandomSelectionErrorReturn::SettingsSupplied { identifier: RandomSourceKind::OperatingSystem })`
    * `[✅]`   `the_mock_member_with_an_empty_object_document_returns_a_selection_carrying_its_failure_mode`
      * `[✅]`   Contract: the identifier `RandomSourceKind::Mock(mode)` with the payload an empty `Value::Object` → `Ok` holding the selection `RandomSourceKind::Mock(mode)`
      * `[✅]`   Arrange: the identifier `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)` and the payload `Value::Object` over `Map::new()`, a document that supplies no setting and is not null, so a mock arm that refuses every document but `Value::Null` fails
      * `[✅]`   Act: `admit_random_selection`
      * `[✅]`   Assert: the result equals `Ok(build_admit_random_selection_success_return(...))` built with the selection override `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)`
    * `[✅]`   `the_mock_member_with_the_succeeds_mode_returns_a_selection_carrying_that_mode`
      * `[✅]`   Contract: the identifier `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::Succeeds)` with the payload `Value::Null` → `Ok` holding the selection `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::Succeeds)`
      * `[✅]`   Arrange: the identifier `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::Succeeds)` and the payload `Value::Null`, a mode that differs from `FillBytesRefused`, which the other mock blocks arrange, so a mock arm that returns one fixed mode fails
      * `[✅]`   Act: `admit_random_selection`
      * `[✅]`   Assert: the result equals `Ok(build_admit_random_selection_success_return(...))` built with the selection override `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::Succeeds)`
    * `[✅]`   Every test block carries the full `Contract`, `Arrange`, `Act`, `Assert` header and the inline markers, and ends with a comment stating the test's name

  * `[✅]`   `construction`
    * `[✅]`   `AdmitRandomSelectionSuccessReturn` is produced only by `admit_random_selection` and its builder; a selection reaches a consumer through this function, the family's declared-selection builder, or the factory's own configuration, never from a caller's knowledge of a member
    * `[✅]`   A caller holding an admitted identifier and a settings document calls `admit_random_selection` and receives the selection or the family's refusal
  * `[✅]`   `adapters/random/src/admit_random_selection/mod.rs`
    * `[✅]`   Module declarations: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, `pub mod provides;`, and `#[cfg(test)] mod test;`
    * `[✅]`   `pub fn admit_random_selection(_deps: &AdmitRandomSelectionDeps, params: AdmitRandomSelectionParams, payload: serde_json::Value) -> AdmitRandomSelectionReturn`: binds `supplies_a_setting` from an exhaustive `match` on `&payload`, `Value::Null` to `false`, `Value::Object(entries)` to `!entries.is_empty()`, and each of `Value::Bool(_)`, `Value::Number(_)`, `Value::String(_)`, and `Value::Array(_)` to `true`; then an exhaustive `match` on `params.identifier`, each arm returning `Err(AdmitRandomSelectionErrorReturn::SettingsSupplied { identifier: params.identifier })` where `supplies_a_setting` holds and otherwise `Ok(AdmitRandomSelectionSuccessReturn { selection })`: the `RandomSourceKind::OperatingSystem` arm with `selection` `RandomSourceKind::OperatingSystem`, and the `RandomSourceKind::Mock(mode)` arm, under `#[cfg(any(test, feature = "mocks"))]`, with `selection` `RandomSourceKind::Mock(mode)`; no `unwrap`, `expect`, `panic!`, numeric `as`, or `unsafe`
    * `[✅]`   Imports this module's names from `interface`, `RandomSourceKind` from `crate::factory::provides`, and `serde_json::Value` by its full path

  * `[✅]`   `adapters/random/src/admit_random_selection/provides.rs`
    * `[✅]`   `pub use super::admit_random_selection;`, the function at public visibility since the crate's barrel re-exports it, `pub use super::interface::*;`, and `#[cfg(any(test, feature = "mocks"))] pub use super::mock::*;`

  * `[✅]`   `adapters/random/tests/admit_random_selection_integration_test.rs`
    * `[✅]`   The file is gated by `#![cfg(feature = "mocks")]`, so it compiles only where the crate's builders exist; no module-level `allow`
    * `[✅]`   Imports from the crate's public surface and its `mocks` feature: `admit_random_selection`, `build_admit_random_selection_deps`, `AdmitRandomSelectionDepsOverrides`, `build_admit_random_selection_params`, `AdmitRandomSelectionParamsOverrides`, `AdmitRandomSelectionErrorReturn`, `RandomSourceKind`, `MockIRandomSourceAdapterFailureMode`, and `RANDOM_SOURCE_KINDS`, and `Map` and `Value` from `serde_json`
    * `[✅]`   Each block builds its deps and params through their builders, writes its payload through `Value`'s own variants, makes each call to `admit_random_selection` through the crate's public surface, and asserts the whole result by `assert_eq!`; a block over the declared set writes its body once and runs it for each identifier of `RANDOM_SOURCE_KINDS`, so the test names no concrete
    * `[✅]`   `admit_random_selection_returns_each_declared_member_its_own_selection_for_an_outside_caller`
      * `[✅]`   Contract: own entry `every_declared_member_is_admitted_to_the_selection_naming_it`
      * `[✅]`   Arrange: each identifier of `RANDOM_SOURCE_KINDS`, with the payload `Value::Null` and with the payload an empty `Value::Object`
      * `[✅]`   Act: `admit_random_selection` over each identifier and each payload
      * `[✅]`   Assert: each result equals `Ok` holding the selection that equals the identifier it was called over
      * `[✅]`   Boundary: the public `admit_random_selection`
      * `[✅]`   Mocked: nothing
    * `[✅]`   `admit_random_selection_returns_the_mock_member_a_selection_carrying_its_failure_mode_for_an_outside_caller`
      * `[✅]`   Contract: own entry `the_mock_member_is_admitted_to_the_selection_carrying_its_failure_mode`
      * `[✅]`   Arrange: the identifiers `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)` and `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::Succeeds)`, each with the payload `Value::Null` and with the payload an empty `Value::Object`
      * `[✅]`   Act: `admit_random_selection` over each identifier and each payload
      * `[✅]`   Assert: each result for the `FillBytesRefused` identifier equals `Ok` holding the selection `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)`, and each result for the `Succeeds` identifier equals `Ok` holding the selection `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::Succeeds)`, each stated as an independent literal
      * `[✅]`   Boundary: the public `admit_random_selection`
      * `[✅]`   Mocked: nothing, because the mock member is the family's own mock concrete
    * `[✅]`   `admit_random_selection_refuses_a_document_supplying_a_setting_naming_the_member_for_an_outside_caller`
      * `[✅]`   Contract: own entry `a_document_supplying_a_setting_is_refused_naming_the_member`
      * `[✅]`   Arrange: each identifier of `RANDOM_SOURCE_KINDS` and `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)`, each with the payload `Value::Object` over a `Map` holding the key `unsupported_setting`, with the payload `Value::Array` over an empty `Vec`, with the payload `Value::Bool(true)`, with the payload `Value::from(7_u64)`, and with the payload `Value::String(String::from("text"))`
      * `[✅]`   Act: `admit_random_selection` over each identifier and each payload
      * `[✅]`   Assert: each result equals `Err(AdmitRandomSelectionErrorReturn::SettingsSupplied { identifier })` naming the identifier it was called over
      * `[✅]`   Boundary: the public `admit_random_selection`
      * `[✅]`   Mocked: nothing
    * `[✅]`   Reports its enumeration: each entry of the spec's public surface, by the block above that proves it

  * `[✅]`   `directionality`
    * `[✅]`   `admit_random_selection` depends on the family's `factory` module for the identifier and the declared set, and on `serde_json` for the boundary type; the concrete, the declaration lookup, and the modules beneath them name nothing in this module; no cycle
    * `[✅]`   The dependency map's `random/factory` rows depend on this function and its types where they need an admitted selection

  * `[✅]`   `requirements`
    * `[✅]`   `adapters/random/Cargo.toml` carries `serde_json = "1.0.154"` under `[dependencies]`, `adapters/random/src/lib.rs` carries the lines the elements above state, and the files of `admit_random_selection/` carry the elements above
    * `[✅]`   `cargo check --workspace --all-targets --all-features`, `cargo fmt --all --check`, and `cargo deny check` complete without error; `cargo clippy --workspace --all-targets --all-features` reports no warning
    * `[✅]`   The unit blocks of `test.rs` pass, each asserting the whole result by `assert_eq!`: `the_operating_system_member_with_an_absent_document_returns_its_own_selection`, `the_operating_system_member_with_an_empty_object_document_returns_its_own_selection`, `the_mock_member_with_an_absent_document_returns_a_selection_carrying_its_failure_mode`, `the_operating_system_member_with_a_document_holding_an_entry_is_refused_naming_the_member`, `the_operating_system_member_with_an_empty_array_document_is_refused_naming_the_member`, `the_mock_member_with_a_document_holding_an_entry_is_refused_naming_the_member_and_its_failure_mode`, `the_operating_system_member_with_a_boolean_document_is_refused_naming_the_member`, `the_operating_system_member_with_a_number_document_is_refused_naming_the_member`, `the_operating_system_member_with_a_string_document_is_refused_naming_the_member`, `the_mock_member_with_an_empty_object_document_returns_a_selection_carrying_its_failure_mode`, and `the_mock_member_with_the_succeeds_mode_returns_a_selection_carrying_that_mode`
    * `[✅]`   In `admit_random_selection_integration_test.rs`, `admit_random_selection_returns_each_declared_member_its_own_selection_for_an_outside_caller`, `admit_random_selection_returns_the_mock_member_a_selection_carrying_its_failure_mode_for_an_outside_caller`, and `admit_random_selection_refuses_a_document_supplying_a_setting_naming_the_member_for_an_outside_caller` pass
    * `[✅]`   `AdmitRandomSelectionErrorReturn` derives `Debug`, `PartialEq`, and `Eq`, and carries the identifier as `RandomSourceKind`, never as text

* `[✅]`   `random/factory` **Randomness factory constructing the concrete the selection `random/admit_random_selection` returns names and returning it behind the family's trait; carries the family's integration test and the grouping's commit**

  * `[✅]`   `objective`
    * `[✅]`   Problem: a consumer obtains a randomness source only through the family's generic surface, never by naming a concrete, and a caller reads a member's declaration through `random/lookup_random_declaration`, never from the constructed source (CR-05; Composition Boundary)
      * `[✅]`   Sources, the planning documents: the dependency map's `random/factory` row, "constructs the member the admitted selection names" and "carries the family's integration test across factory and concrete", with its proof "CR-05: the one surface every production draw passes through", and its dependencies `random/os` and `random/admit_random_selection`
      * `[✅]`   Sources, `random/os`: its `IRandomSourceAdapter` element, "a member's declaration is read through `random/lookup_random_declaration` without constructing the member, never from the adapter"; its `RANDOM_SOURCE_KINDS` element, "containing only the real `OperatingSystem` member"; its `build_random_source_kind` element, "the family's declared-selection builder"
      * `[✅]`   Sources, `random/admit_random_selection`: its success return, "the selection naming the member, which carries no settings"; its own entries `every_declared_member_is_admitted_to_the_selection_naming_it` and `the_mock_member_is_admitted_to_the_selection_carrying_its_failure_mode`
      * `[✅]`   Sources, `random/lookup_random_declaration`: its objective, "a caller outside the `adapters/random` crate reads a declared member's declaration through this function alone"
      * `[✅]`   Sources, the topics: `families.md`, "Selecting a concrete from configuration", "Reading a declaration before construction", and "A further concrete extends its family"; `composition.md`, "Configuration follows ownership (Rust)"
    * `[✅]`   Functional: given the selection `random/admit_random_selection` returns, a `RandomSourceKind`, the factory constructs the concrete the selection names and returns it as `Box<dyn IRandomSourceAdapter>`; the declaration of the member is read through `random/lookup_random_declaration`, never from the returned source
    * `[✅]`   Functional: a concrete's constructor error is returned unchanged in the factory's error arm, one variant per concrete
    * `[✅]`   Functional: a source obtained from the factory draws the requested number of random bytes through the family's trait
    * `[✅]`   Non-functional: adding a concrete is its module, its variant in `RandomSourceKind` and in the family's error enums, its entry in `RANDOM_SOURCE_KINDS`, its arm in `random/admit_random_selection`, `random/lookup_random_declaration`, and `build_random_source_kind`, and its branch here; no consumer changes

  * `[✅]`   `role`
    * `[✅]`   Adapter family factory: the implementation of the `factory` module, which is the randomness family's construction point and the crate's public surface
    * `[✅]`   Does not admit configuration text or a settings document; the family's owner admits configuration text into the member identifier, and `random/admit_random_selection` admits the settings document into the selection the factory takes in `params.kind`
    * `[✅]`   Does not admit against upstream declarations; the randomness family consumes no other family
    * `[✅]`   Does not draw bytes; drawing is the concrete's
    * `[✅]`   Does not read or restate a member's declaration, which `random/lookup_random_declaration` reads
    * `[✅]`   Carries the family's integration test and the commit for the workspace and discipline bootstrap milestone

  * `[✅]`   `module`
    * `[✅]`   Bounded context: the `factory` module of `adapters/random`, holding the factory function, its deps, params, payload, and return types, its signature type, its function mock and builders, and the crate's integration test under `adapters/random/tests`
    * `[✅]`   Outside: every concrete's behavior, the family's declared set and declared-selection builder, the declaration lookup, the selection admission, the decoding of configuration, and every consumer of the family

  * `[✅]`   `deps`
    * `[✅]`   The `os` concrete, through `crate::os::provides`: `OsRandomSource`, `OsRandomSourceConstructorParams`, and `OsRandomSource::try_new`; the factory constructs its concretes, the family form's recorded cycle
    * `[✅]`   The `factory` module's own interface, as `random/os` authors it: `IRandomSourceAdapter`, `RandomSourceKind`, `FillBytesParams`, `FillBytesPayloadOverrides`, `build_fill_bytes_payload`, and `FillBytesErrorReturn`; in `mock.rs`, `test.rs`, `integration_test.rs`, and the public integration test, `build_random_source_kind`, `RandomSourceKindOverrides`, `RANDOM_SOURCE_KINDS`, and `MockIRandomSourceAdapterFailureMode`
    * `[✅]`   `random/lookup_random_declaration`, through the derives it adds to `RandomSourceKind` and `MockIRandomSourceAdapterFailureMode` in `factory/interface.rs`, `Clone`, `Copy`, `Debug`, `PartialEq`, and `Eq`, which the blocks use to pass a member of `RANDOM_SOURCE_KINDS` by value and to hold a selection
    * `[✅]`   `domain`, `crates/domain`, through the `Secret` the family's `FillBytesSuccessReturn` holds: `Secret::expose`, which every block reads a draw through
    * `[✅]`   `random/admit_random_selection`, the family's selection admission, through the crate's public surface in the integration test: `admit_random_selection`, `build_admit_random_selection_deps`, `build_admit_random_selection_params`, and `AdmitRandomSelectionParamsOverrides`, as that node authors them; the factory takes in `params.kind` the selection its success return holds, and the factory function names no item of it
    * `[✅]`   `serde_json`, external crate, as `random/admit_random_selection` adds it to `adapters/random/Cargo.toml`: `Value`, the settings document an integration test passes to `admit_random_selection`
    * `[✅]`   A member's declaration is read through `random/lookup_random_declaration` alone; no function and no test of this node reads one
    * `[✅]`   `core::convert::Infallible`, standard library, the operating-system constructor's error carried in the factory's error arm
    * `[✅]`   `std::collections::HashSet`, standard library, in the public integration test only
    * `[✅]`   The external crates are those `random/os` and `random/admit_random_selection` declare in `adapters/random/Cargo.toml`

  * `[✅]`   `context_slice`
    * `[✅]`   From `os`: `OsRandomSource::try_new(OsRandomSourceConstructorParams) -> Result<OsRandomSource, Infallible>` and `OsRandomSource`'s implementation of `IRandomSourceAdapter`
    * `[✅]`   From `random/admit_random_selection`: `admit_random_selection(&AdmitRandomSelectionDeps, AdmitRandomSelectionParams, serde_json::Value) -> Result<AdmitRandomSelectionSuccessReturn, AdmitRandomSelectionErrorReturn>`, `AdmitRandomSelectionSuccessReturn`'s field `selection: RandomSourceKind`, and `AdmitRandomSelectionParams`'s field `identifier: RandomSourceKind`
    * `[✅]`   From the `factory` module: `build_random_source_kind(kind: RandomSourceKind, overrides: RandomSourceKindOverrides) -> RandomSourceKind`, `RandomSourceKindOverrides`, the fieldless unit struct with `Default`, and `RANDOM_SOURCE_KINDS: &[RandomSourceKind]`
    * `[✅]`   From the `factory` module: `IRandomSourceAdapter::fill_bytes(&self, FillBytesParams, FillBytesPayload) -> FillBytesReturn`, `FillBytesParams`, the fieldless struct, `FillBytesPayloadOverrides` with its field `length: Option<usize>`, `build_fill_bytes_payload(FillBytesPayloadOverrides) -> FillBytesPayload`, and `FillBytesErrorReturn` with its variant `MockIRandomSourceAdapter`
    * `[✅]`   From `domain`: `Secret::expose(&self) -> &T`, read on the `bytes` of `FillBytesSuccessReturn`

  * `[✅]`   `adapters/random/src/factory/interface.rs`
    * `[✅]`   `CreateRandomSourceDeps`, the fieldless struct `pub struct CreateRandomSourceDeps;`
    * `[✅]`   `CreateRandomSourceParams`, a struct with `pub kind: RandomSourceKind`, the selection of the concrete to construct, the selection `random/admit_random_selection` returns
    * `[✅]`   `CreateRandomSourcePayload`, the fieldless struct `pub struct CreateRandomSourcePayload;`, since the factory operates on no data
    * `[✅]`   `CreateRandomSourceSuccessReturn`, a struct with `pub adapter: Box<dyn IRandomSourceAdapter>`; it pairs no declaration with the boxed source, the declaration of the member being read through `random/lookup_random_declaration`
    * `[✅]`   `CreateRandomSourceErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]`, which `Infallible` satisfies, and the variant `OperatingSystem(Infallible)`, the operating-system concrete's constructor error carried unchanged, and, under `#[cfg(any(test, feature = "mocks"))]`, the variant `MockIRandomSourceAdapter(Infallible)`, the mock concrete's constructor error carried unchanged; each concrete's constructor error is its own variant
    * `[✅]`   `CreateRandomSourceReturn`, the type alias `Result<CreateRandomSourceSuccessReturn, CreateRandomSourceErrorReturn>`
    * `[✅]`   `CreateRandomSourceFn`, the type alias `fn(&CreateRandomSourceDeps, CreateRandomSourceParams, CreateRandomSourcePayload) -> CreateRandomSourceReturn`
    * `[✅]`   Imports `core::convert::Infallible` for the error variants

  * `[✅]`   `adapters/random/src/factory/interaction.spec.md`
    * `[✅]`   `create_random_source`, mock, under `#[cfg(any(test, feature = "mocks"))]`: condition: `params.kind` is `RandomSourceKind::Mock(mode)`; decision: the same `match` on `params.kind`; dependency call: `MockIRandomSourceAdapter::try_new(MockIRandomSourceAdapterConstructorParams { failure_mode: mode })`, exactly once; outcome: `Ok(CreateRandomSourceSuccessReturn { adapter: Box::new(source) })`; the boxed source carries the failure mode `mode`, and its constructor error arm is uninhabited
    * `[✅]`   `create_random_source`, operating system: condition: `params.kind` is `RandomSourceKind::OperatingSystem`; decision: a `match` on `params.kind`; dependency call: `OsRandomSource::try_new(OsRandomSourceConstructorParams)`, exactly once; outcome: `Ok(CreateRandomSourceSuccessReturn { adapter: Box::new(source) })`
    * `[✅]`   The operating-system constructor's error arm is uninhabited, so its success is destructured irrefutably and that branch has no failure outcome; `CreateRandomSourceErrorReturn::OperatingSystem` carries its error type in the return union
    * `[✅]`   `params.kind` selects the concrete; `deps` and `payload` carry nothing and are not read; the `match` is exhaustive over `RandomSourceKind`, so a kind with no branch fails to compile
    * `[✅]`   Callee dispositions, `random/os`: its interaction spec states no integration entry, so no entry passes through `create_random_source` or ends at one of its tests
    * `[✅]`   Callee dispositions, `random/lookup_random_declaration`: this node calls none of its functions, so no entry of it passes through `create_random_source` or ends at one of its tests
    * `[✅]`   Callee dispositions, `random/admit_random_selection`: its entries `every_declared_member_is_admitted_to_the_selection_naming_it` and `the_mock_member_is_admitted_to_the_selection_carrying_its_failure_mode` are carried below, each reaching a test of this node through its selection; its entry `a_document_supplying_a_setting_is_refused_naming_the_member` yields no selection and so reaches neither `create_random_source` nor a test of this node
    * `[✅]`   Own entry, `a_source_the_factory_constructs_for_each_declared_member_draws_the_requested_bytes_from_the_generator`: condition `create_random_source` is called by a caller outside the crate with the selection admitted for each member of `RANDOM_SOURCE_KINDS`, and `fill_bytes` is called on the returned source for a length of 1024; outcome `Ok` whose `bytes` hold exactly 1024 bytes, whose 32-byte segments are pairwise distinct; variation the length 1024, which differs from the builder's default of 32 and splits into segments, so a source that returns the default width, or a zero-filled or repeating buffer, fails; edge that must survive the composition: the generator's output reaches the caller through the family's trait, and the test does not prove the generator's quality
    * `[✅]`   Own entry, `a_source_the_factory_constructs_draws_an_empty_buffer_for_a_zero_length`: condition `create_random_source` is called by a caller outside the crate with the selection admitted for each member of `RANDOM_SOURCE_KINDS`, and `fill_bytes` is called on the returned source for a length of zero; outcome `Ok` whose `bytes` are empty; variation the length zero, which differs from the builder's default of 32, so a source that ignores the length fails; edge that must survive the composition: a zero-length draw is a success through the factory's surface
    * `[✅]`   Carried entry, `every_declared_member_is_admitted_to_the_selection_naming_it` of `random/admit_random_selection`, restated in this node's terms: the selection admitted for each member of `RANDOM_SOURCE_KINDS`, from a document that is `Value::Null` and from a document that is an empty `Value::Object`, is the selection `create_random_source` constructs that member's real source from, so two sources built from the two admitted selections draw 32-byte buffers that differ, which the mock concrete's fixed counter does not; route: `admit_random_selection` → `create_random_source` → the member's `try_new` → `IRandomSourceAdapter::fill_bytes` → the member's generator; the outer-edge collaborator is the member's generator, which runs real
    * `[✅]`   Carried entry, `the_mock_member_is_admitted_to_the_selection_carrying_its_failure_mode` of `random/admit_random_selection`, restated in this node's terms: the selection admitted for `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)` constructs a source whose `fill_bytes` returns `Err(FillBytesErrorReturn::MockIRandomSourceAdapter)`, and the selection admitted for `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::Succeeds)` constructs a source whose `fill_bytes` returns the bytes requested, so the failure mode the configuration names survives the factory; route: `admit_random_selection` → `create_random_source` → the mock selection's `try_new` → `IRandomSourceAdapter::fill_bytes`; no outer-edge collaborator, the mock concrete being the family's own
    * `[✅]`   Own entry, `every_source_the_factory_constructs_draws_exactly_the_bytes_requested`: condition `fill_bytes` is called on the source `create_random_source` returns for each selection `build_random_source_kind` returns for a member of `RANDOM_SOURCE_KINDS` and for `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::Succeeds)`, for a length of 48; outcome `Ok` whose `bytes` hold exactly 48 bytes; variation the length 48, which differs from the builder's default of 32, so a source that returns the default width fails; edge that must survive the composition: every concrete of the family, the mock concrete included, answers the trait's length contract
    * `[✅]`   Own entry, `every_source_the_factory_constructs_draws_an_empty_buffer_for_a_zero_length`: condition `fill_bytes` is called on the source `create_random_source` returns for each selection `build_random_source_kind` returns for a member of `RANDOM_SOURCE_KINDS` and for `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::Succeeds)`, for a length of zero; outcome `Ok` whose `bytes` are empty; variation the length zero, which differs from the builder's default of 32, so a source that ignores the length fails; edge that must survive the composition: a zero-length draw is a success for every concrete
    * `[✅]`   Own entry, `the_mock_source_returns_the_family_mock_error_when_its_failure_mode_refuses`: condition `fill_bytes` is called on the source `create_random_source` returns for `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)`; outcome `Err(FillBytesErrorReturn::MockIRandomSourceAdapter)`; variation the failure mode `FillBytesRefused`, which differs from the builder's default `Succeeds`, so a source that ignores the mode fails; edge that must survive the composition: the failure mode the selection names reaches the draw, and the declared set cannot reach this arm
    * `[✅]`   Private surface: the real chain `create_random_source` → the selection's `try_new` → `IRandomSourceAdapter::fill_bytes`; the outer-edge collaborator is the operating system's generator, which runs real; the entries proven are `every_source_the_factory_constructs_draws_exactly_the_bytes_requested`, `every_source_the_factory_constructs_draws_an_empty_buffer_for_a_zero_length`, and `the_mock_source_returns_the_family_mock_error_when_its_failure_mode_refuses`, by `integration_test.rs`; the observable result is the `FillBytesReturn`; the branches of `create_random_source` are proven by the unit blocks of `test.rs`
    * `[✅]`   Public surface: the crate's `lib.rs` re-exports `factory::provides`, so an outside caller invokes `create_random_source` over a selection and `fill_bytes` on the source it returns
      * `[✅]`   It observes the `CreateRandomSourceReturn` and the source's `FillBytesReturn` through their public fields
      * `[✅]`   The entries proven are `a_source_the_factory_constructs_for_each_declared_member_draws_the_requested_bytes_from_the_generator`, `a_source_the_factory_constructs_draws_an_empty_buffer_for_a_zero_length`, `every_declared_member_is_admitted_to_the_selection_naming_it`, and `the_mock_member_is_admitted_to_the_selection_carrying_its_failure_mode`
      * `[✅]`   The routes are those the entries state

  * `[✅]`   `adapters/random/src/factory/mock.rs`
    * `[✅]`   `CreateRandomSourceParamsOverrides`, `#[derive(Default)]`, one field `pub kind: Option<RandomSourceKind>`; `build_create_random_source_params(overrides: CreateRandomSourceParamsOverrides) -> CreateRandomSourceParams`, the kind defaulting to `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::Succeeds)`, so a consumer's test obtains the mock concrete through `create_random_source` without naming it, and a test proving a refused draw overrides the kind with the mode that refuses
    * `[✅]`   `CreateRandomSourceSuccessReturnOverrides`, the fieldless unit struct with `#[derive(Default)]`, since the adapter is handed; `build_create_random_source_success_return(adapter: Box<dyn IRandomSourceAdapter>, overrides: CreateRandomSourceSuccessReturnOverrides) -> CreateRandomSourceSuccessReturn`, the adapter its leading parameter
    * `[✅]`   `mock_create_random_source(_deps: &CreateRandomSourceDeps, _params: CreateRandomSourceParams, _payload: CreateRandomSourcePayload) -> CreateRandomSourceReturn`, standing in for the factory that constructs a concrete: it builds the family's mock concrete with `build_mock_i_random_source_adapter(Default::default())` and returns `Ok(build_create_random_source_success_return(Box::new(adapter), Default::default()))`
    * `[✅]`   `CreateRandomSourceDepsOverrides` and `CreateRandomSourcePayloadOverrides`, each the fieldless unit struct with `#[derive(Default)]`; `build_create_random_source_deps(_overrides: CreateRandomSourceDepsOverrides) -> CreateRandomSourceDeps` and `build_create_random_source_payload(_overrides: CreateRandomSourcePayloadOverrides) -> CreateRandomSourcePayload`, each returning its production value, with no corruptions and no invalidator
    * `[✅]`   No builder for the enum `CreateRandomSourceErrorReturn`, used by its production value

  * `[✅]`   `adapters/random/src/factory/test.rs`
    * `[✅]`   Module-level `#![allow(clippy::expect_used)]`; imports `create_random_source`, `build_create_random_source_deps`, `build_create_random_source_payload`, `CreateRandomSourceParamsOverrides`, `build_create_random_source_params`, `build_random_source_kind`, `MockIRandomSourceAdapterFailureMode`, `RandomSourceKind`, `RANDOM_SOURCE_KINDS`, `IRandomSourceAdapter`, `FillBytesParams`, `FillBytesPayloadOverrides`, `build_fill_bytes_payload`, and `FillBytesErrorReturn` from `super::provides`
    * `[✅]`   `every_member_of_the_declared_set_is_constructed_into_a_source_drawing_from_the_real_generator`
      * `[✅]`   Contract: `create_random_source` is called with `params.kind` the selection `build_random_source_kind` returns for a member of `RANDOM_SOURCE_KINDS` → `Ok(CreateRandomSourceSuccessReturn { adapter })` holding the real concrete that member names, whose draws come from its generator and not from the mock concrete's counter
      * `[✅]`   Collaborators: the `try_new` of the concrete each member names, which the factory constructs, runs for real, and the member's generator runs as the outer edge; fixtures `build_create_random_source_params`, `build_create_random_source_deps`, `build_create_random_source_payload`, `build_random_source_kind`, and `build_fill_bytes_payload`
      * `[✅]`   Arrange: the block's body written once and run over each member of `RANDOM_SOURCE_KINDS`, `build_create_random_source_params` with the kind override set to `build_random_source_kind(member, Default::default())`; two sources built from two calls to `create_random_source` over that selection, so a factory arm that constructs the mock concrete, whose fixed counter yields the same first draw from every fresh instance, fails
      * `[✅]`   Act: `create_random_source(&build_create_random_source_deps(Default::default()), params, build_create_random_source_payload(Default::default()))` once for each source, then `fill_bytes(FillBytesParams, build_fill_bytes_payload(Default::default()))` once on each
      * `[✅]`   Assert: each success arm is extracted with `expect`; the `bytes.expose()` of the first source differs from the `bytes.expose()` of the second under `assert_ne!`
    * `[✅]`   `create_random_source_returns_the_mock_source_failing_in_the_mode_its_kind_names`
      * `[✅]`   Contract: `params.kind` is `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)` → `Ok(CreateRandomSourceSuccessReturn { adapter })` holding the mock concrete, whose `fill_bytes` returns `Err(FillBytesErrorReturn::MockIRandomSourceAdapter)`
      * `[✅]`   Collaborators: `MockIRandomSourceAdapter::try_new`, the concrete the factory constructs, runs for real; fixtures `build_create_random_source_params`, `build_create_random_source_deps`, `build_create_random_source_payload`, and `build_fill_bytes_payload`
      * `[✅]`   Arrange: `build_create_random_source_params` with the kind override set to `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)`, a mode that differs from the builder's default `Succeeds`, so an arm that drops the mode, whose draw would succeed, fails
      * `[✅]`   Act: `create_random_source(&build_create_random_source_deps(Default::default()), params, build_create_random_source_payload(Default::default()))`, then `fill_bytes(FillBytesParams, build_fill_bytes_payload(Default::default()))` on the success arm's adapter
      * `[✅]`   Assert: the adapter is extracted from the success arm with `expect`; the `fill_bytes` result's `err()` equals `Some(FillBytesErrorReturn::MockIRandomSourceAdapter)` under `assert_eq!`
    * `[✅]`   `create_random_source_returns_the_mock_source_drawing_the_bytes_requested_in_the_succeeds_mode`
      * `[✅]`   Contract: `params.kind` is `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::Succeeds)` → `Ok(CreateRandomSourceSuccessReturn { adapter })` holding the mock concrete, whose `fill_bytes` returns `Ok` holding exactly `payload.length` bytes
      * `[✅]`   Collaborators: `MockIRandomSourceAdapter::try_new`, the concrete the factory constructs, runs for real; fixtures `build_create_random_source_params`, `build_create_random_source_deps`, `build_create_random_source_payload`, and `build_fill_bytes_payload`
      * `[✅]`   Arrange: `build_create_random_source_params` with the kind override set to `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::Succeeds)`, and `build_fill_bytes_payload` with the length override set to 48, a length that differs from the builder's default of 32, so an arm that fixes the mode to `FillBytesRefused` fails and a source that returns the default width fails
      * `[✅]`   Act: `create_random_source(&build_create_random_source_deps(Default::default()), params, build_create_random_source_payload(Default::default()))`, then `fill_bytes(FillBytesParams, payload)` on the success arm's adapter
      * `[✅]`   Assert: the adapter and the `fill_bytes` success arm are each extracted with `expect`; the length of `bytes.expose()` equals the literal 48 written in the assertion
    * `[✅]`   Every test block carries the full `Contract`, `Arrange`, `Act`, `Assert` header and the inline markers, and ends with a comment stating the test's name

  * `[✅]`   `construction`
    * `[✅]`   The composition root calls `admit_random_selection` over the identifier admitted from configuration and the untrusted settings document, then calls `create_random_source` with `&CreateRandomSourceDeps`, `CreateRandomSourceParams` holding the `selection` the admission returned, and `CreateRandomSourcePayload`, and places the returned `Box<dyn IRandomSourceAdapter>` in each consumer's deps; no consumer constructs a concrete, builds a selection, or reads a declaration from the source, a declaration being read through `lookup_random_declaration`

  * `[✅]`   `adapters/random/src/factory/mod.rs`
    * `[✅]`   Adds `#[cfg(test)] mod test;` and `#[cfg(test)] mod integration_test;` to the wiring `random/os` authored
    * `[✅]`   `pub fn create_random_source(_deps: &CreateRandomSourceDeps, params: CreateRandomSourceParams, _payload: CreateRandomSourcePayload) -> CreateRandomSourceReturn`, a `match` on `params.kind` whose `RandomSourceKind::OperatingSystem` arm binds the concrete by `let Ok(source) = OsRandomSource::try_new(OsRandomSourceConstructorParams);` and returns `Ok(CreateRandomSourceSuccessReturn { adapter: Box::new(source) })`, and, under `#[cfg(any(test, feature = "mocks"))]`, whose `RandomSourceKind::Mock(mode)` arm binds the mock concrete by `let Ok(source) = MockIRandomSourceAdapter::try_new(MockIRandomSourceAdapterConstructorParams { failure_mode: mode });` and returns `Ok(CreateRandomSourceSuccessReturn { adapter: Box::new(source) })`
    * `[✅]`   Imports `OsRandomSource` and `OsRandomSourceConstructorParams` from `crate::os::provides`, under the same `cfg` `MockIRandomSourceAdapter` and `MockIRandomSourceAdapterConstructorParams` from `crate::factory::mock`, and this module's types from `interface`
    * `[✅]`   No `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[✅]`   `adapters/random/src/factory/integration_test.rs`
    * `[✅]`   A `#[cfg(test)]` module of its own beside `test.rs`; module-level `#![allow(clippy::expect_used)]`; imports `create_random_source`, `build_create_random_source_deps`, `build_create_random_source_payload`, `CreateRandomSourceParamsOverrides`, `build_create_random_source_params`, `build_random_source_kind`, `MockIRandomSourceAdapterFailureMode`, `RandomSourceKind`, `RANDOM_SOURCE_KINDS`, `IRandomSourceAdapter`, `FillBytesParams`, `FillBytesPayloadOverrides`, `build_fill_bytes_payload`, and `FillBytesErrorReturn` from `super::provides`
    * `[✅]`   Each block builds its selection with `build_random_source_kind(member, Default::default())` for a member of `RANDOM_SOURCE_KINDS`, or names `RandomSourceKind::Mock(...)` with the failure mode the block names; a block over the declared set writes its body once and runs it for each member of `RANDOM_SOURCE_KINDS` and for `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::Succeeds)`, so a further concrete answers every entry without an edit to the test
    * `[✅]`   `every_source_the_factory_constructs_draws_exactly_the_bytes_requested`
      * `[✅]`   Contract: own entry `every_source_the_factory_constructs_draws_exactly_the_bytes_requested`
      * `[✅]`   Boundary: `create_random_source` → the selection's `try_new` → `IRandomSourceAdapter::fill_bytes`
      * `[✅]`   Mocked: nothing; the operating system's generator is the outer edge and runs real
      * `[✅]`   Arrange: `build_create_random_source_params` with the kind override set to each selection the block runs over; `build_fill_bytes_payload` with the length override set to 48, which differs from the builder's default of 32
      * `[✅]`   Act: `create_random_source` over `build_create_random_source_deps(Default::default())`, the params, and `build_create_random_source_payload(Default::default())`, then `fill_bytes(FillBytesParams, payload)` on the success arm's adapter
      * `[✅]`   Assert: the adapter and the `fill_bytes` success arm are each extracted with `expect`; the length of `bytes.expose()` equals the literal 48 written in the assertion
    * `[✅]`   `every_source_the_factory_constructs_draws_an_empty_buffer_for_a_zero_length`
      * `[✅]`   Contract: own entry `every_source_the_factory_constructs_draws_an_empty_buffer_for_a_zero_length`
      * `[✅]`   Boundary: `create_random_source` → the selection's `try_new` → `IRandomSourceAdapter::fill_bytes`
      * `[✅]`   Mocked: nothing; the operating system's generator is the outer edge and runs real
      * `[✅]`   Arrange: `build_create_random_source_params` with the kind override set to each selection the block runs over; `build_fill_bytes_payload` with the length override set to 0, which differs from the builder's default of 32
      * `[✅]`   Act: `create_random_source` over the deps, the params, and the payload, then `fill_bytes(FillBytesParams, payload)` on the success arm's adapter
      * `[✅]`   Assert: the adapter and the `fill_bytes` success arm are each extracted with `expect`; `bytes.expose()` is empty
    * `[✅]`   `the_mock_source_returns_the_family_mock_error_when_its_failure_mode_refuses`
      * `[✅]`   Contract: own entry `the_mock_source_returns_the_family_mock_error_when_its_failure_mode_refuses`
      * `[✅]`   Boundary: `create_random_source` → the selection's `try_new` → `IRandomSourceAdapter::fill_bytes`
      * `[✅]`   Mocked: nothing; the mock concrete is the family's own, and the declared set cannot reach this arm, so the block runs over the mock selection alone
      * `[✅]`   Arrange: `build_create_random_source_params` with the kind override set to `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)`, which differs from the builder's default `Succeeds`; `build_fill_bytes_payload` at its defaults
      * `[✅]`   Act: `create_random_source` over the deps, the params, and the payload, then `fill_bytes(FillBytesParams, payload)` on the success arm's adapter
      * `[✅]`   Assert: the adapter is extracted from the success arm with `expect`; the `fill_bytes` result's `err()` equals `Some(FillBytesErrorReturn::MockIRandomSourceAdapter)` under `assert_eq!`
    * `[✅]`   Every test block carries the full `Contract`, `Boundary`, `Mocked`, `Arrange`, `Act`, `Assert` header and the inline markers, and ends with a comment stating the test's name
    * `[✅]`   Reports its enumeration: each entry of the spec's private surface, by the block above that proves it

  * `[✅]`   `adapters/random/src/factory/provides.rs`
    * `[✅]`   Adds `pub use super::create_random_source;` to the re-exports `random/os` authored, `RANDOM_SOURCE_KINDS` being among them

  * `[✅]`   `adapters/random/tests/factory_integration_test.rs`
    * `[✅]`   The file is gated by `#![cfg(feature = "mocks")]`, so it compiles only where the crate's builders exist; outside the crate, so it reaches only the crate's public surface: module-level `#![allow(clippy::expect_used)]`; imports from the `random` crate `create_random_source`, `build_create_random_source_deps`, `build_create_random_source_payload`, `CreateRandomSourceParamsOverrides`, `build_create_random_source_params`, `admit_random_selection`, `build_admit_random_selection_deps`, `build_admit_random_selection_params`, `AdmitRandomSelectionParamsOverrides`, `IRandomSourceAdapter`, `FillBytesParams`, `FillBytesPayloadOverrides`, `build_fill_bytes_payload`, `FillBytesErrorReturn`, `RandomSourceKind`, `MockIRandomSourceAdapterFailureMode`, and `RANDOM_SOURCE_KINDS`, `HashSet` from `std::collections`, and `Map` and `Value` from `serde_json`
    * `[✅]`   Every test block carries the full `Contract`, `Boundary`, `Mocked`, `Arrange`, `Act`, `Assert` header and the inline markers, and ends with a comment stating the test's name
    * `[✅]`   Each block obtains the selection it constructs from by calling `admit_random_selection` over the identifier the block names and the document the block names, with deps from `build_admit_random_selection_deps` and params from `build_admit_random_selection_params` carrying the identifier override, extracting the success arm with `expect` and passing its `selection` as the kind override of `build_create_random_source_params`; each block's body over the declared set is written once and run for each member of `RANDOM_SOURCE_KINDS`, so the test names no concrete
    * `[✅]`   `a_source_from_the_factory_draws_random_bytes_through_the_family_trait`
      * `[✅]`   Contract: own entry `a_source_the_factory_constructs_for_each_declared_member_draws_the_requested_bytes_from_the_generator`
      * `[✅]`   Boundary: the crate's public surface; the real chain `admit_random_selection` → `create_random_source` → the member's `try_new` → `IRandomSourceAdapter::fill_bytes` → the member's generator
      * `[✅]`   Mocked: nothing; each member's generator is its outer edge and runs real, so this test proves the generator's output reaches the caller and does not prove the generator's quality
      * `[✅]`   Arrange: for each member of `RANDOM_SOURCE_KINDS` in turn, the selection admitted from the document `Value::Null`, the adapter returned by `create_random_source` for it with `build_create_random_source_deps(Default::default())` and `build_create_random_source_payload(Default::default())` as its deps and payload; `build_fill_bytes_payload` with the length override set to 1024, which splits into segments of 32 bytes and differs from the builder's default of 32
      * `[✅]`   Act: `fill_bytes(FillBytesParams, payload)` on the returned adapter
      * `[✅]`   Assert: the success arm is extracted with `expect`; the length of `bytes.expose()` equals the literal 1024 written in the assertion; the 32-byte segments of `bytes.expose()` collected into a `HashSet` number the literal 32 written in the assertion
    * `[✅]`   `a_source_from_the_factory_draws_an_empty_buffer_for_a_zero_length`
      * `[✅]`   Contract: own entry `a_source_the_factory_constructs_draws_an_empty_buffer_for_a_zero_length`
      * `[✅]`   Boundary: the crate's public surface; the real chain `admit_random_selection` → `create_random_source` → the member's `try_new` → `IRandomSourceAdapter::fill_bytes` → the member's generator
      * `[✅]`   Mocked: nothing; each member's generator is its outer edge and runs real
      * `[✅]`   Arrange: for each member of `RANDOM_SOURCE_KINDS` in turn, the selection admitted from the document `Value::Null`, the adapter returned by `create_random_source` for it with `build_create_random_source_deps(Default::default())` and `build_create_random_source_payload(Default::default())` as its deps and payload; `build_fill_bytes_payload` with the length override set to 0, which differs from the builder's default of 32
      * `[✅]`   Act: `fill_bytes(FillBytesParams, payload)` on the returned adapter
      * `[✅]`   Assert: the success arm is extracted with `expect`; `bytes.expose()` is empty
    * `[✅]`   `a_selection_admitted_for_each_declared_member_constructs_the_source_that_member_names`
      * `[✅]`   Contract: carried entry `every_declared_member_is_admitted_to_the_selection_naming_it` of `random/admit_random_selection`, as this node restates it
      * `[✅]`   Boundary: the crate's public surface; the real chain `admit_random_selection` → `create_random_source` → the member's `try_new` → `IRandomSourceAdapter::fill_bytes` → the member's generator
      * `[✅]`   Mocked: nothing; each member's generator is its outer edge and runs real
      * `[✅]`   Arrange: for each member of `RANDOM_SOURCE_KINDS` in turn, two selections admitted for it, one from the document `Value::Null` and one from the document `Value::Object` over an empty `Map`, and the adapter returned by `create_random_source` for each, a pair of fresh sources whose first draws a mock concrete's fixed counter would make equal; `build_fill_bytes_payload` at its default length of 32
      * `[✅]`   Act: `fill_bytes(FillBytesParams, payload)` once on each of the two adapters
      * `[✅]`   Assert: both success arms are extracted with `expect`; the `bytes.expose()` of the first adapter differs from the `bytes.expose()` of the second under `assert_ne!`
    * `[✅]`   `a_mock_selection_admitted_with_its_failure_mode_constructs_the_source_failing_in_that_mode`
      * `[✅]`   Contract: carried entry `the_mock_member_is_admitted_to_the_selection_carrying_its_failure_mode` of `random/admit_random_selection`, as this node restates it
      * `[✅]`   Boundary: the crate's public surface; the real chain `admit_random_selection` → `create_random_source` → the mock selection's `try_new` → `IRandomSourceAdapter::fill_bytes`
      * `[✅]`   Mocked: nothing; the mock concrete is the family's own
      * `[✅]`   Arrange: the selection admitted for `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)` from the document `Value::Null`, and the selection admitted for `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::Succeeds)` from the document `Value::Object` over an empty `Map`, the two modes differing so a factory that fixes one mode fails; the adapter returned by `create_random_source` for each; `build_fill_bytes_payload` with the length override set to 48, which differs from the builder's default of 32
      * `[✅]`   Act: `fill_bytes(FillBytesParams, payload)` on each of the two adapters
      * `[✅]`   Assert: the result of the `FillBytesRefused` adapter has an `err()` equal to `Some(FillBytesErrorReturn::MockIRandomSourceAdapter)` under `assert_eq!`; the `Succeeds` adapter's success arm is extracted with `expect` and the length of its `bytes.expose()` equals the literal 48 written in the assertion
    * `[✅]`   Reports its enumeration: each entry of the spec's public surface, by the block above that proves it

  * `[✅]`   `directionality`
    * `[✅]`   The `factory` module depends on `os` through `crate::os::provides` and on its own interface; `os` depends on the `factory` module's surface, the family form's recorded cycle; the factory function names no item of `admit_random_selection` or `lookup_random_declaration`, and the integration test reads `admit_random_selection` through the crate's public surface alone; the crate's public surface is the `provides` of the `factory`, `lookup_random_declaration`, and `admit_random_selection` modules

  * `[✅]`   `requirements`
    * `[✅]`   `every_member_of_the_declared_set_is_constructed_into_a_source_drawing_from_the_real_generator` passes
    * `[✅]`   `create_random_source_returns_the_mock_source_failing_in_the_mode_its_kind_names` and `create_random_source_returns_the_mock_source_drawing_the_bytes_requested_in_the_succeeds_mode` pass
    * `[✅]`   `every_source_the_factory_constructs_draws_exactly_the_bytes_requested`, `every_source_the_factory_constructs_draws_an_empty_buffer_for_a_zero_length`, and `the_mock_source_returns_the_family_mock_error_when_its_failure_mode_refuses` pass in `integration_test.rs`
    * `[✅]`   `CreateRandomSourceErrorReturn` derives `Debug`, `PartialEq`, and `Eq`
    * `[✅]`   `a_source_from_the_factory_draws_random_bytes_through_the_family_trait` and `a_source_from_the_factory_draws_an_empty_buffer_for_a_zero_length` pass (CR-05, a production draw passes through the factory's surface)
    * `[✅]`   `a_selection_admitted_for_each_declared_member_constructs_the_source_that_member_names` and `a_mock_selection_admitted_with_its_failure_mode_constructs_the_source_failing_in_that_mode` pass
    * `[✅]`   `RANDOM_SOURCE_KINDS` is declared by `random/os` alone, `CreateRandomSourceSuccessReturn` pairs no declaration with the adapter, and a member's declaration is read through `random/lookup_random_declaration` alone
    * `[✅]`   `cargo check --workspace --all-targets --all-features`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `cargo fmt --all --check` complete without error or warning in every target, the `os` concrete's unused-item warnings having no remaining cause
    * `[✅]`   `getrandom` is named nowhere outside `adapters/random/src/os`, and no code outside `adapters/random` can name `OsRandomSource`
    * `[✅]`   The workspace and discipline bootstrap milestone's exit holds: the workspace, `crates/domain`, and `adapters/random` build and pass the Rust CI definition on Windows, macOS, and Linux

  * `[✅]`   **Commit** `feat(foundation): workspace, Rust CI, secret type, and randomness family`
    * `[✅]`   Structural: the virtual workspace manifest with its lint table and `librqbit` overlay, `rust-toolchain.toml`, `deny.toml`, `.gitignore`, `.github/workflows/rust.yml`, the `domain` crate with its `secret` module, and the `random` crate with its `factory` module, `os` concrete, `lookup_random_declaration` module, and `admit_random_selection` module
    * `[✅]`   Behavioral: secrets zeroize on drop and cannot be formatted, cloned, or copied; random bytes are drawn from the operating system's generator into a `Secret` through the family's factory
    * `[✅]`   Contract: `Secret` with `try_new` and `expose`; `IRandomSourceAdapter` with `fill_bytes`, its params, payload, and return types, `RandomSourceDeclaration`, and `RANDOM_SOURCE_INTERFACE_VERSION`; `lookup_random_declaration` and `admit_random_selection` with their signatures and return types; `create_random_source` with its signature and return types

* `[✅]`   `workspace/ci` **Continuous-integration matrix over Windows, macOS, and Linux running the workspace checks, tests, and dependency audits**

  * `[✅]`   `objective`
    * `[✅]`   Problem: every ticket's proof, format, type check, lint, tests, license policy, and advisories, runs on each platform the completion boundary names
    * `[✅]`   Functional: every push to any branch and every pull request to `main` runs the workspace proof on `ubuntu-latest`, `macos-latest`, and `windows-latest`, and a failure on one runner does not cancel the others
    * `[✅]`   Functional: each runner installs the toolchain and components pinned by `rust-toolchain.toml`, never a toolchain named in the workflow
    * `[✅]`   Functional: each runner runs, in order, the format check, the type check, the lint, the tests, the dependency-policy check, and the advisory audit, and any failure fails that runner's job
    * `[✅]`   Non-functional: the workflow token is read-only, a newer run on the same ref cancels the older one, and no step exists beyond those listed here

  * `[✅]`   `role`
    * `[✅]`   Infrastructure: one configuration file with no types and no tests, exempt from the support-file structure
    * `[✅]`   Does not create the Solidity, TypeScript, fuzz, or end-to-end workflow definitions, each a separate file created once by the ticket that first needs it, `contracts/evm/ParameterSets` for `forge build`, `forge fmt --check`, and `forge test`, the first shell or webview ticket for the TypeScript linter, the first fuzz target for `cargo-fuzz`, and the first end-to-end scenario for the clean-runner job; no later ticket edits `rust.yml`
    * `[✅]`   Does not edit `.github/workflows/npm-publish.yml`
    * `[✅]`   Does not package, sign, release, or publish, and reads no secret
    * `[✅]`   Does not carry a commit; the grouping's integration test and commit are carried by `random/factory`

  * `[✅]`   `module`
    * `[✅]`   Bounded context: the Rust continuous-integration definition — triggers, token permissions, concurrency, the runner matrix, toolchain and tool installation, and the proof commands
    * `[✅]`   Outside: the workspace configuration it reads (`workspace/cargo`), release packaging and signing, and every non-Rust check

  * `[✅]`   `deps`
    * `[✅]`   `workspace/cargo`, repository-root configuration: `Cargo.toml` for the members and lint table, `rust-toolchain.toml` for the toolchain and components, `deny.toml` for the dependency policy; the workflow consumes them and nothing in them refers to the workflow
    * `[✅]`   `actions/checkout@v7`, external action, checks out the ref under test
    * `[✅]`   `actions-rust-lang/setup-rust-toolchain@v2`, external action, given no `toolchain` input so it installs the toolchain and components from `rust-toolchain.toml`, caches through `Swatinem/rust-cache` by its default, and sets build warnings to deny by its default
    * `[✅]`   `taiki-e/install-action@v2`, external action, installs prebuilt `cargo-deny` and `cargo-audit` on Linux, macOS, and Windows
    * `[✅]`   No reverse dependency: no repository file reads the workflow

  * `[✅]`   `.github/workflows/rust.yml`
    * `[✅]`   `on`: `push` with `branches: ["**"]`, and `pull_request` with `branches: ["main"]`
    * `[✅]`   `permissions`: `contents: read`
    * `[✅]`   `concurrency`: `group: ${{ github.workflow }}-${{ github.ref }}` and `cancel-in-progress: true`
    * `[✅]`   `name`: `Rust`
    * `[✅]`   `env`: `CARGO_TERM_COLOR: always`
    * `[✅]`   `jobs.build`: `strategy` with `fail-fast: false` and `matrix.os: [ubuntu-latest, macos-latest, windows-latest]`; `runs-on: ${{ matrix.os }}`
    * `[✅]`   `jobs.build.steps`, in this order: `uses: actions/checkout@v7`; `uses: actions-rust-lang/setup-rust-toolchain@v2` with no `with` block; `uses: taiki-e/install-action@v2` with `tool: cargo-deny,cargo-audit`; `run: cargo fmt --all --check`; `run: cargo check --workspace --all-targets --all-features`; `run: cargo clippy --workspace --all-targets --all-features -- -D warnings`; `run: cargo test --workspace --all-features`; `run: cargo deny check`; `run: cargo audit`

  * `[✅]`   `directionality`
    * `[✅]`   The workflow depends inward on the root configuration and on pinned external actions; nothing in the repository depends on it; no cycle

  * `[✅]`   `requirements`
    * `[✅]`   `.github/workflows/rust.yml` carries exactly the triggers, permissions, concurrency, environment, matrix, and steps stated above, in that order, under the workflow name `Rust` and the job id `build`
    * `[✅]`   On each runner, `cargo deny check` reads `deny.toml` without a configuration error
    * `[✅]`   A formatting deviation, a clippy warning or denied lint, a failing test, a dependency outside the license allowlist or from an unlisted source, or an unignored advisory fails the job on the runner where it occurs
    * `[✅]`   `cargo deny check` under `workspace/ci` reads `deny.toml` without a configuration error on Windows, macOS, and Linux

* **Cryptographic validation harness**

## Pairing adapters and key derivation

* `[ ]`   `pairing/bn254_arkworks` **BN254 pairing concrete on arkworks implementing the pairing family's generic interface, arithmetic trait, and reference trait over the EIP-196 and EIP-197 encodings and the `Bn254V1` target-group value; creates the `adapters/pairing` crate and authors the family's traits, sampling bound, member and value identifiers with one total spelling per real member, declared sets, declaration, selection, and mock**

  * `[✅]`   `objective`
    * `[✅]`   Problem: the credential KEM, the envelope, and the delivery proof compute in a Type-3 pairing group whose curve the launch chain's precompiles dictate, so every group operation, subgroup check, pairing-product check, and precompile encoding passes through one repo-owned interface and no module outside a concrete names a curve library (CR-10)
    * `[✅]`   Problem: the credential KEM encapsulates and decapsulates a target-group value `K` the key-derivation family consumes as bytes, checks validity and well-formedness as pairing equations whose sides are divided, and refuses a trivial identity element; the envelope decrypts by removing a masked source-group element and rejects identity-element keys; the delivery proof computes Schnorr responses in the scalar field; and the decrypted credential is a pair of source-group elements that must be zeroized; so each of these operations passes through the family (CR-04; CR-07; CR-08; CR-09; `docs/research/cryptography.md`'s Credential KEM, Key Agreement, and Delivery Proof statements)
    * `[✅]`   Problem: the bytes the KEM hands the KDF are a target-group element's encoding, and a library's reduced pairing is a convention: `ark-ec`'s BN254 final exponentiation returns the Miller loop's value raised to `2x(6x^2 + 3x + 1)` times the exact exponent `(p^12 - 1) / r` for the curve seed `x`, and `halo2curves`' returns it raised to the exact exponent, so two holders on different libraries derive different wrapping keys from one capsule; the protocol fixes the value by one identifier per curve naming the pairing map, the generators, the tower, the exact exponent, and the serialization, every concrete declares it, the composition admission admits by it before construction, and the hash-card pins it (CR-10; `docs/research/cryptography.md`'s pairing adapter capability declaration; the product requirements' canonical target-group value position)
    * `[✅]`   Problem: a contract recomputes every hash-to-scalar mapping as the digest reduced modulo the group order, and proves its own subgroup checks only against an on-curve point outside the subgroup; the group order is a constant of each curve library and such a point is found only with a library's curve internals, so neither is reachable by a consumer of the family, and a consumer that mirrors the Rust reference to a target suite names no curve library and holds no literal; so the family exposes both, each concrete reading them from the library it wraps (CR-10 on chain; CR-11; the dependency map's `pairing/bn254_arkworks`, `harness-crypto/generate/solidity/constants`, and `harness-crypto/generate/solidity/rejection_vectors` rows; the Pairing adapters and key derivation milestone's scope and exit; the technical requirements' generated-directory contents)
    * `[✅]`   Functional: the family's generic interface exposes the generators of both source groups, addition, scalar multiplication, and multi-scalar multiplication in either source group, the pairing-product check, and decoding and encoding of group elements and scalars in the target chain's precompile format
    * `[✅]`   Functional: the group-element, scalar, and encoded first-group, second-group, and scalar types are associated types of the generic interface, so a consumer names them through the family without knowing which concrete produced them; each encoder returns a distinct owned encoded type for each group and concrete, which preserves its group and curve, and a consumer reaches its wire bytes only by borrowing `as_ref()`
    * `[✅]`   Functional: every group element and scalar a concrete accepts from bytes is decoded by a fallible constructor that rejects a wrong length, a non-canonical field element, a point off the curve, and a point outside the prime-order subgroup, so no element that fails a check exists as a value
    * `[✅]`   Functional: a scalar type is sampled from uniform bytes the randomness family drew, by reduction of a fixed-width input modulo the group order, and the sampled scalar is returned inside a `Secret`
    * `[✅]`   Functional: every concrete's declaration carries its `PairingConcrete` member identifier, its curve, whether the target chain's verifier has second-group arithmetic, its precompile encoding, its target-group encoding identifier, its adapter version, and the interface version it implements, the declaration carrying the typed member identifier with no duplicate text name, read through `pairing/lookup_pairing_declaration` without an instance, never from a constructed adapter
    * `[✅]`   Functional: the member identifier and each value identifier spell each real member once and totally, the declared sets hold the real members alone, the real selection carries no settings, and the mock selection alone carries its failure mode
    * `[✅]`   Functional: the BN254 concrete encodes a first-group point as EIP-196's 64 bytes and a second-group point as EIP-197's 128 bytes, each coordinate a 32-byte big-endian integer, the second-group coordinates imaginary part first, and the point at infinity as all zero bytes; a scalar is 32 big-endian bytes
    * `[✅]`   Functional: the family's arithmetic trait extends the generic interface with the scalar field's addition, multiplication, and negation modulo the group order
    * `[✅]`   Functional: the arithmetic trait negates a point in either source group, the identity negating to the identity, and reports whether a point in either source group is the identity
    * `[✅]`   Functional: the arithmetic trait computes the product of the pairings of a list of first-group and second-group pairs as a value of the target group, an empty list yielding the target group's identity; division is realized by negating a first-group input, and no target-group arithmetic exists above the concrete
    * `[✅]`   Functional: the arithmetic trait encodes a target-group value as its twelve base-field coefficients in the tower order of the degree-twelve extension, `c0.c0.c0`, `c0.c0.c1`, `c0.c1.c0`, `c0.c1.c1`, `c0.c2.c0`, `c0.c2.c1`, `c1.c0.c0`, `c1.c0.c1`, `c1.c1.c0`, `c1.c1.c1`, `c1.c2.c0`, `c1.c2.c1`, each the coefficient's big-endian canonical integer at the base field's byte width, in a distinct owned fixed-width encoded type returned inside a `Secret`
    * `[✅]`   Functional: this concrete declares `TargetGroupEncodingIdentifier::Bn254V1`: the optimal ate pairing as EIP-197 fixes it, over the EIP-197 generators, on the tower `Fp2 = Fp[u] / (u^2 + 1)`, `Fp6 = Fp2[v] / (v^3 - (u + 9))`, `Fp12 = Fp6[w] / (w^2 - v)`, the Miller loop's value raised to the exact exponent `(p^12 - 1) / r`, serialized as the twelve 32-byte tower coefficients above
    * `[✅]`   Functional: `pairing_product` returns the identifier's exact value: the library's reduced pairing multiplied in the target group by `reduced_pairing_correction`, the inverse in the scalar field of `2x(6x^2 + 3x + 1)`, computed once in `try_new` by Fermat from the curve seed
    * `[✅]`   Functional: the concrete's test executes the identifier's definition, the library's Miller loop over the two generators raised to the integer `(p^12 - 1) / r` built by `num-bigint` from the library's moduli and serialized in the tower order, and requires `pairing_product` then `encode_gt` over the generators to yield the same bytes
    * `[✅]`   Functional: the arithmetic trait requires the source-group types and the target-group type to implement `Zeroize`, and this concrete's scalar, first-group, second-group, and target-group types zeroize their value through `Zeroize` and on drop
    * `[✅]`   Functional: the family's reference trait extends the generic interface with the scalar field's order as 32 big-endian bytes, the integer every scalar is reduced modulo and the least value `decode_scalar` refuses as non-canonical, and, for each source group, the precompile encoding of the family's outside-the-subgroup point of that group in the concrete's encoded type for that group, or its absence where the group is the whole curve
    * `[✅]`   Functional: the family's outside-the-subgroup point of a source group is the on-curve point outside the prime-order subgroup with the least first coordinate in an ascending search, `x = 1, 2, 3, …` in the first group and `x = (c0, 0)` with `c0 = 1, 2, 3, …` in the second, and, of the two points sharing that `x`, the one whose `y` is the lesser: in the first group the lesser integer, and in the second the `y` whose `c1` is the lesser integer, or whose `c0` is the lesser where both `c1` are zero; the rule is the family's, so every library of one curve yields the same bytes
    * `[✅]`   Functional: this concrete returns the group order arkworks holds for BN254's scalar field, no first-group point, and the second group's point under the rule, encoded as `encode_g2` encodes any point, an encoding its own `decode_g2` refuses as outside the subgroup
    * `[✅]`   Non-functional: `ark-bn254`, `ark-ec`, and `ark-ff` are named only inside the arkworks concretes; the arithmetic trait and the reference trait are each separate from the generic interface, so a concrete compiles against the generic interface alone; `num-bigint` is named only in `adapters/pairing/Cargo.toml` and in the concretes' `test.rs` files; no order and no point is written as a literal outside a test; the crate depends on no repository crate but `domain` at runtime

  * `[✅]`   `role`
    * `[✅]`   Adapter: the pairing family's concrete that authors, in the family's `factory` module and as their producer, the generic interface, the sampling bound, the member and value identifiers with their total spellings and declared sets, the declaration carrying the typed member identifier with no duplicate text name, the selection enum, the arithmetic trait, the reference trait, every method's types, and the family's mock
    * `[✅]`   Creates `factory/mod.rs` with the module wiring alone and `factory/provides.rs` with the interface and mock surface alone; the factory function, its types, its interaction spec, its unit test, its re-export, the consumer trait, and the family's integration test are `pairing/factory`'s
    * `[✅]`   Does not author the BN254 `halo2curves` concrete or either BLS12-381 concrete; this node declares only the `Bn254Arkworks` member and the values it provides, each further concrete adding its own member variant, spelling, declared-set entry, and provided value variants in its own increment, `pairing/bls12_381_arkworks` adding the BLS12-381 variants of `PairingCurve`, `VerifierGroupArithmetic`, and `PrecompileEncoding` and the `Bls12381V1` variant of `TargetGroupEncodingIdentifier`
    * `[✅]`   Does not hash to a group or to a scalar; `hash-to-scalar/keccak256` maps domain-tagged bytes to a scalar through this family's scalar type
    * `[✅]`   Does not draw randomness; a consumer draws `UNIFORM_BYTES_LENGTH` bytes through the randomness family and hands the draw to the sampling bound
    * `[✅]`   Does not compute in the target group beyond the pairing product, its correction to the identifier's value, and its encoding: no target-group multiplication, exponentiation, or inversion is exposed
    * `[✅]`   Does not map a target-group value to a key; the key-derivation family derives from the encoding
    * `[✅]`   Does not read the correction from configuration; it is a property of this library under this identifier and is computed in `try_new`
    * `[✅]`   Does not decode, classify, or emit a rejection vector; `harness-crypto/generate/solidity/rejection_vectors` emits the reference encodings it reads from the reference trait, and each decoder's refusal of them is proven by `pairing/factory`'s public integration test
    * `[✅]`   Does not carry a settings module, settings node, or settings table; the concrete has no settable value, so the real selection carries no settings
    * `[✅]`   Does not carry a commit

  * `[✅]`   `module`
    * `[✅]`   Bounded context: the `adapters/pairing` crate's `factory` module, holding the family's generic interface, arithmetic trait, and reference trait with their associated types and methods, every method's params, payload, success, error, and return types, the sampling bound with its types, the member and value identifiers with their total spellings and declared sets, the declaration carrying the typed member identifier, the selection, the interface version, and the family's mock; and its private `bn254_arkworks` concrete, holding the adapter over arkworks' BN254 with its correction field and seed constant, its declaration constant, its scalar, group-element, target-group, and encoded types over the library's elements, its constructor params, its implementations of the three traits, and the builder defaults for its owned types
    * `[✅]`   The node's files: `adapters/pairing/Cargo.toml`, `adapters/pairing/src/lib.rs`, the `factory` module's `interface.rs`, `mock.rs`, `mod.rs`, and `provides.rs`, and the `bn254_arkworks` module's `interface.rs`, `interaction.spec.md`, `mock.rs`, `test.rs`, `mod.rs`, and `provides.rs`
    * `[✅]`   Creates the crate at `adapters/pairing`, admitted by the workspace's `adapters/*` glob with no edit to the root manifest
    * `[✅]`   Outside: which scalars are secret and who produces them, hashing to a scalar, the KEM, envelope, and proof algebra built on the family, the key derivation from the target-group encoding, the Solidity mirror of the encodings, the cross-library agreement of the encodings, the decoders' verdicts as vectors, the family's declaration lookup and its selection and composition admissions, and the factory's construction of a concrete

  * `[✅]`   `deps`
    * `[✅]`   `domain`, `crates/domain`, protocol and domain ring, path dependency; supplies `Secret` and `SecretConstructorParams`, the wrapper for uniform input, sampled scalars, encoded scalars, and encoded target-group values; direction inward, adapter ring on domain ring
    * `[✅]`   `domain` with its `mocks` feature, as a dev-dependency and through this crate's `mocks` feature; supplies `build_secret` and `SecretConstructorParamsOverrides`
    * `[✅]`   `zeroize` `1.9.0`, external crate, Apache-2.0 OR MIT, runtime dependency; supplies the `Zeroize` and `ZeroizeOnDrop` traits the sampling bound and the arithmetic trait require, its implementation for `Vec<Z: Zeroize>`, and its implementations for arkworks' short-Weierstrass affine points and `PairingOutput`; zeroization is a property of the type, not an external touchpoint, so no adapter wraps it
    * `[✅]`   `ark-bn254` `0.6.0`, `ark-ec` `0.6.0`, and `ark-ff` `0.6.0`, external crates, MIT OR Apache-2.0, runtime dependencies named only in `bn254_arkworks`; supply the curve, the pairing, the Miller loop, the group arithmetic, and the field arithmetic
    * `[✅]`   `hex` `0.4.3`, external crate, MIT OR Apache-2.0, dev-dependency; supplies `hex::decode` for the test vectors, in `mock.rs`'s test-only fixtures
    * `[✅]`   `num-bigint` `0.4.8`, the version `Cargo.lock` resolves for `ark-ff`, external crate, MIT OR Apache-2.0, dev-dependency; used only in `mock.rs`'s test-only fixtures, to compute the integer exponent `(p^12 - 1) / r`, and in `test.rs`, to compare roots
    * `[✅]`   `core::convert::Infallible`, standard library, the error arm of every method that has no failure; `core::marker::PhantomData`, standard library, in `factory/mock.rs`
    * `[✅]`   Reverse dependencies: every other pairing concrete and `pairing/factory`, through the `factory` module's surface

  * `[✅]`   `context_slice`
    * `[✅]`   From `domain`: `Secret::try_new(SecretConstructorParams { value })` returning `Result<Secret<T>, Infallible>`, and `Secret::expose(&self) -> &T`
    * `[✅]`   From `domain`'s mocks: `build_secret(value: T, SecretConstructorParamsOverrides) -> Secret<T>` for `T: Zeroize`
    * `[✅]`   From `zeroize`: the `Zeroize` trait's `zeroize(&mut self)`, the `ZeroizeOnDrop` marker trait, and their implementations for `Vec<Z: Zeroize>`, `Affine`, and `PairingOutput`
    * `[✅]`   From `ark-bn254`: `Bn254`, `Fq`, `Fq2` with its public fields `c0` and `c1` and `Fq2::new(c0, c1)`, `Fq6` with `Fq6::new(c0, c1, c2)`, `Fq12` with `Fq12::new(c0, c1)`, `Fr`, `G1Affine`, `G1Projective`, `G2Affine`, and `G2Projective`
    * `[✅]`   From `ark-ec`: `AffineRepr` for `generator()`, `xy()`, and `is_zero()`, true exactly for the identity; the short-Weierstrass affine `identity()`, `new_unchecked(x, y)`, `is_on_curve()`, `is_in_correct_subgroup_assuming_on_curve()`, and `get_point_from_x_unchecked(x, greatest)`, an `Option` of an on-curve point with that `x`, unchecked for the subgroup; the affine `+` and `* Fr` producing projective points; unary `-` on the affine point, returning `(x, -y)` and the identity for the identity; `CurveGroup::into_affine`; `VariableBaseMSM::msm_unchecked(bases, scalars)`; `pairing::Pairing::multi_pairing(a, b)` over iterators of borrowed affine points, returning `PairingOutput<Bn254>`, whose public field `0` is the `Fq12` value and whose identity is `Fq12::one()`; `Pairing::multi_miller_loop(a, b)` over the same iterators, returning `MillerLoopOutput<Bn254>` whose public field `0` is the unreduced `Fq12`; `Mul<Fr> for PairingOutput<Bn254>`, the exponentiation of a target-group value by a scalar; `Pairing::pairing(p, q)`; `Zeroize` for the affine point, which zeroizes `x`, `y`, and `infinity`, so a zeroized point is `new_unchecked(0, 0)`, and for `PairingOutput`, which zeroizes its `Fq12`
    * `[✅]`   From `ark-ff`: `Fr`'s `+`, `*`, and unary `-` modulo the group order; `Fr::from(u64)` and `Fq::from(u64)`; `Field::ONE`; `Field::pow(&self, exp: impl AsRef<[u64]>)`, infallible exponentiation by little-endian limbs, on `Fr` and on `Fq12`; `PrimeField::MODULUS`, the `BigInt<4>` of `Fr` and of `Fq`; `BigInteger::sub_with_borrow(&mut self, other: &Self) -> bool` and `BigInt::<4>::from(u64)`; `From<BigInt<4>> for num_bigint::BigUint`; the public fields `c0` and `c1` of the degree-two and degree-twelve extensions and `c0`, `c1`, and `c2` of the degree-six extension; `PrimeField::from_be_bytes_mod_order(&[u8])` and `into_bigint()` on `Fq` and on `Fr`, the `BigInt<4>` of `Fq` ordering as the integer; `BigInteger::to_bytes_be()`, 32 bytes for a `BigInt<4>`; `Zero::is_zero()` on `Fq`, on `Fq12`, and on `PairingOutput`; unary `-` on `Fq2`
    * `[✅]`   From `num-bigint`, in `mock.rs`'s test-only fixtures and in `test.rs`: `BigUint::from(u32)`, `BigUint::from_bytes_be(&[u8])`, `BigUint::pow(&self, u32)`, `Sub`, `Div`, and `Rem` between `BigUint`s, and `BigUint::to_u64_digits(&self) -> Vec<u64>`, little-endian limbs

  * `[✅]`   `adapters/pairing/Cargo.toml`
    * `[✅]`   `[package]` with `name = "pairing"`, `edition.workspace = true`, `rust-version.workspace = true`, and `publish.workspace = true`; no `version` key
    * `[✅]`   `[dependencies]` with `domain = { path = "../../crates/domain" }`, `zeroize = "1.9.0"`, `ark-bn254 = "0.6.0"`, `ark-ec = "0.6.0"`, and `ark-ff = "0.6.0"`, each with default features
    * `[✅]`   `[dev-dependencies]` with `domain = { path = "../../crates/domain", features = ["mocks"] }`, `hex = "0.4.3"`, and `num-bigint = "0.4.8"`
    * `[✅]`   `[features]` with `mocks = ["domain/mocks"]`
    * `[✅]`   `[lints]` with `workspace = true`
    * `[✅]`   No other table

  * `[✅]`   `adapters/pairing/src/lib.rs`
    * `[✅]`   The crate barrel: `mod factory;`, `mod bn254_arkworks;`, and `pub use factory::provides::*;`; each further concrete node contributes its own module line

  * `[ ]`   `adapters/pairing/src/factory/interface.rs`
    * `[✅]`   Imports `core::convert::Infallible`, `domain::Secret`, and `zeroize::{Zeroize, ZeroizeOnDrop}`; names no vendor and no concrete
    * `[✅]`   `PAIRING_INTERFACE_VERSION`, a `pub const` of type `u32` with value `1`
    * `[✅]`   `PairingCurve`, an enum with the real variant `Bn254` spelled `bn254` by its `name()` mapping; `VerifierGroupArithmetic`, an enum with the real variant `FirstGroupOnly` spelled `first_group_only`; `PrecompileEncoding`, an enum with the real variant `Eip196Eip197` spelled `eip196_eip197`; each value identifier holds only real provided values, no mock-only variant
    * `[✅]`   `PairingConcrete`, the member identifier, an enum with `#[derive(Clone, Copy, Debug, PartialEq, Eq)]`, the real member `Bn254Arkworks`, an exhaustive `name()` mapping spelling `Bn254Arkworks` as `bn254_arkworks` and, under `#[cfg(any(test, feature = "mocks"))]`, the member `Mock(MockIPairingAdapterFailureMode)` spelled `mock`, the mock selection carrying the failure mode its configuration names; `PAIRING_CONCRETES`, a `pub const` of type `&[PairingConcrete]` containing only the real `Bn254Arkworks` member, so input admission accepts only declared real spellings and later concretes are added only by their own increments
    * `[✅]`   `MockIPairingAdapterFailureMode`, under `#[cfg(any(test, feature = "mocks"))]`, an enum with `#[derive(Clone, Copy, Debug, PartialEq, Eq)]` and the variants `Succeeds`, `DecodeG1NotOnCurve`, `DecodeG1NotInSubgroup`, `DecodeG2NotOnCurve`, `G1OutsideSubgroupSearchExhausted`, `G2OutsideSubgroupSearchExhausted`, and `SampleScalarWrongLength`, each failing variant standing for an error arm of the contract that no input reaches on the mock concrete
    * `[✅]`   `TargetGroupEncodingIdentifier`, an enum with `#[derive(Clone, Copy, Debug, PartialEq, Eq)]` and the real variant `Bn254V1` spelled `bn254_v1`, its doc comment reading "The optimal ate pairing as EIP-197 fixes it, over the EIP-197 generators, on the tower `Fp2 = Fp[u] / (u^2 + 1)`, `Fp6 = Fp2[v] / (v^3 - (u + 9))`, `Fp12 = Fp6[w] / (w^2 - v)`; the pairing value is the Miller loop's value raised to the exact exponent `(p^12 - 1) / r`, not a fixed multiple of it; a target-group element is serialized as its twelve base-field coefficients in tower order, each the coefficient's 32-byte big-endian canonical integer."; `Bls12381V1` arrives with its providing concrete's increment; `TARGET_GROUP_ENCODING_IDENTIFIERS`, a `pub const` of type `&[TargetGroupEncodingIdentifier]` containing only the real `Bn254V1` value, later real values added by their providing concretes
    * `[✅]`   `PairingDeclaration`, a struct with `pub concrete: PairingConcrete`, `pub curve: PairingCurve`, `pub verifier_group_arithmetic: VerifierGroupArithmetic`, `pub precompile_encoding: PrecompileEncoding`, `pub target_group_encoding: TargetGroupEncodingIdentifier`, `pub adapter_version: u32`, and `pub interface_version: u32`, the declaration carrying the typed member identifier with no duplicate text name
    * `[✅]`   The sampling bound's types: the fieldless `SampleUniformScalarParams`; `SampleUniformScalarPayload` with `pub uniform: Secret<Vec<u8>>`; `SampleUniformScalarSuccessReturn<S: Zeroize>` with `pub scalar: Secret<S>`; `SampleUniformScalarErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]` and the struct variant `WrongLength { expected: usize, actual: usize }`; `SampleUniformScalarReturn<S>`, the alias `Result<SampleUniformScalarSuccessReturn<S>, SampleUniformScalarErrorReturn>`
    * `[ ]`   `ISampleUniformScalar`, the sampling bound, `pub trait ISampleUniformScalar: Zeroize + ZeroizeOnDrop + Sized + 'static`, the `'static` supertrait letting `Box<dyn IHashToScalarAdapter<S>>` hold for every `S: ISampleUniformScalar` and for `P::Scalar` as a caller names it, with `const UNIFORM_BYTES_LENGTH: usize;` and `fn sample_from_uniform_bytes(params: SampleUniformScalarParams, payload: SampleUniformScalarPayload) -> SampleUniformScalarReturn<Self>;`
    * `[✅]`   `g1_generator`: the fieldless `G1GeneratorParams` and `G1GeneratorPayload`; `G1GeneratorSuccessReturn<G>` with `pub point: G`; `G1GeneratorReturn<G>`, the alias `Result<G1GeneratorSuccessReturn<G>, Infallible>`; `g2_generator`, the same shape under the `G2Generator` prefix
    * `[✅]`   `add_g1`: the fieldless `AddG1Params`; `AddG1Payload<G>` with `pub left: G` and `pub right: G`; `AddG1SuccessReturn<G>` with `pub sum: G`; `AddG1Return<G>`, the alias `Result<AddG1SuccessReturn<G>, Infallible>`; `add_g2`, the same shape under the `AddG2` prefix
    * `[✅]`   `mul_g1`: the fieldless `MulG1Params`; `MulG1Payload<G, S>` with `pub point: G` and `pub scalar: S`; `MulG1SuccessReturn<G>` with `pub product: G`; `MulG1Return<G>`, the alias `Result<MulG1SuccessReturn<G>, Infallible>`; `mul_g2`, the same shape under the `MulG2` prefix
    * `[✅]`   `msm_g1`: the fieldless `MsmG1Params`; `MsmG1Term<G, S>` with `pub base: G` and `pub scalar: S`; `MsmG1Payload<G, S>` with `pub terms: Vec<MsmG1Term<G, S>>`, so a base and its scalar cannot differ in count; `MsmG1SuccessReturn<G>` with `pub sum: G`; `MsmG1Return<G>`, the alias `Result<MsmG1SuccessReturn<G>, Infallible>`; `msm_g2`, the same shape under the `MsmG2` prefix with `MsmG2Term<G, S>`
    * `[✅]`   `pairing_product_is_one`: the fieldless `PairingProductIsOneParams`; `PairingProductTerm<G1, G2>` with `pub g1: G1` and `pub g2: G2`; `PairingProductIsOnePayload<G1, G2>` with `pub terms: Vec<PairingProductTerm<G1, G2>>`; `PairingProductIsOneSuccessReturn` with `pub is_one: bool`; `PairingProductIsOneReturn`, the alias `Result<PairingProductIsOneSuccessReturn, Infallible>`
    * `[✅]`   `decode_g1`: the fieldless `DecodeG1Params`; `DecodeG1SuccessReturn<G>` with `pub point: G`; `DecodeG1ErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]` and the variants `WrongLength { expected: usize, actual: usize }`, `NonCanonicalCoordinate`, `NotOnCurve`, and `NotInSubgroup`; `DecodeG1Return<G>`, the alias `Result<DecodeG1SuccessReturn<G>, DecodeG1ErrorReturn>`; `decode_g2`, the same shape under the `DecodeG2` prefix, `DecodeG2ErrorReturn` carrying the same derives and the same variants
    * `[✅]`   `decode_scalar`: the fieldless `DecodeScalarParams`; `DecodeScalarSuccessReturn<S>` with `pub scalar: S`; `DecodeScalarErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]` and the variants `WrongLength { expected: usize, actual: usize }` and `NonCanonical`; `DecodeScalarReturn<S>`, the alias `Result<DecodeScalarSuccessReturn<S>, DecodeScalarErrorReturn>`
    * `[✅]`   `encode_g1`: the fieldless `EncodeG1Params`; `EncodeG1Payload<G>` with `pub point: G`; `EncodeG1SuccessReturn<E>` with `pub bytes: E`; `EncodeG1Return<E>`, the alias `Result<EncodeG1SuccessReturn<E>, Infallible>`; `encode_g2`, the same shape under the `EncodeG2` prefix
    * `[✅]`   `encode_scalar`: the fieldless `EncodeScalarParams`; `EncodeScalarPayload<S>` with `pub scalar: S`; `EncodeScalarSuccessReturn<E: Zeroize>` with `pub bytes: Secret<E>`, since the scalar encoded may be secret; `EncodeScalarReturn<E>`, the alias `Result<EncodeScalarSuccessReturn<E>, Infallible>`
    * `[✅]`   `IPairingAdapter`, `pub trait IPairingAdapter` with `type Scalar: ISampleUniformScalar + Clone;`, `type G1: Clone;`, `type G2: Clone;`, `type EncodedG1: AsRef<[u8]> + Clone + PartialEq + Eq;`, `type EncodedG2: AsRef<[u8]> + Clone + PartialEq + Eq;`, and `type EncodedScalar: AsRef<[u8]> + Zeroize;`, the encoded types concrete-owned and distinct from each other and from another concrete's; a member's declaration, including its `concrete` identifier, is read through `pairing/lookup_pairing_declaration` without constructing the member, not exposed as a trait-constant surface; the methods `g1_generator(&self, params: G1GeneratorParams, payload: G1GeneratorPayload) -> G1GeneratorReturn<Self::G1>`, `g2_generator` over `Self::G2`, `add_g1(&self, params: AddG1Params, payload: AddG1Payload<Self::G1>) -> AddG1Return<Self::G1>`, `add_g2` over `Self::G2`, `mul_g1(&self, params: MulG1Params, payload: MulG1Payload<Self::G1, Self::Scalar>) -> MulG1Return<Self::G1>`, `mul_g2` over `Self::G2` and `Self::Scalar`, `msm_g1(&self, params: MsmG1Params, payload: MsmG1Payload<Self::G1, Self::Scalar>) -> MsmG1Return<Self::G1>`, `msm_g2` over `Self::G2` and `Self::Scalar`, `pairing_product_is_one(&self, params: PairingProductIsOneParams, payload: PairingProductIsOnePayload<Self::G1, Self::G2>) -> PairingProductIsOneReturn`, `decode_g1(&self, params: DecodeG1Params, payload: &[u8]) -> DecodeG1Return<Self::G1>`, `decode_g2` over `Self::G2`, `decode_scalar(&self, params: DecodeScalarParams, payload: &[u8]) -> DecodeScalarReturn<Self::Scalar>`, `encode_g1(&self, params: EncodeG1Params, payload: EncodeG1Payload<Self::G1>) -> EncodeG1Return<Self::EncodedG1>`, `encode_g2(&self, params: EncodeG2Params, payload: EncodeG2Payload<Self::G2>) -> EncodeG2Return<Self::EncodedG2>`, and `encode_scalar(&self, params: EncodeScalarParams, payload: EncodeScalarPayload<Self::Scalar>) -> EncodeScalarReturn<Self::EncodedScalar>`; each decoder's payload is the untrusted wire bytes and its narrowing target the associated type
    * `[✅]`   `add_scalar`: the fieldless `AddScalarParams`; `AddScalarPayload<S>` with `pub left: S` and `pub right: S`; `AddScalarSuccessReturn<S>` with `pub sum: S`; `AddScalarReturn<S>`, the alias `Result<AddScalarSuccessReturn<S>, Infallible>`; `mul_scalar`, the same shape under the `MulScalar` prefix, its success return holding `pub product: S`
    * `[✅]`   `neg_scalar`: the fieldless `NegScalarParams`; `NegScalarPayload<S>` with `pub scalar: S`; `NegScalarSuccessReturn<S>` with `pub negation: S`; `NegScalarReturn<S>`, the alias `Result<NegScalarSuccessReturn<S>, Infallible>`
    * `[✅]`   `neg_g1`: the fieldless `NegG1Params`; `NegG1Payload<G>` with `pub point: G`; `NegG1SuccessReturn<G>` with `pub negation: G`; `NegG1Return<G>`, the alias `Result<NegG1SuccessReturn<G>, Infallible>`; `neg_g2`, the same shape under the `NegG2` prefix
    * `[✅]`   `is_identity_g1`: the fieldless `IsIdentityG1Params`; `IsIdentityG1Payload<G>` with `pub point: G`; `IsIdentityG1SuccessReturn` with `pub is_identity: bool`; `IsIdentityG1Return`, the alias `Result<IsIdentityG1SuccessReturn, Infallible>`; `is_identity_g2`, the same shape under the `IsIdentityG2` prefix
    * `[✅]`   `pairing_product`: the fieldless `PairingProductParams`; `PairingProductPayload<G1, G2>` with `pub terms: Vec<PairingProductTerm<G1, G2>>`; `PairingProductSuccessReturn<T>` with `pub product: T`; `PairingProductReturn<T>`, the alias `Result<PairingProductSuccessReturn<T>, Infallible>`
    * `[✅]`   `encode_gt`: the fieldless `EncodeGtParams`; `EncodeGtPayload<T>` with `pub value: T`; `EncodeGtSuccessReturn<E: Zeroize>` with `pub bytes: Secret<E>`; `EncodeGtReturn<E>`, the alias `Result<EncodeGtSuccessReturn<E>, Infallible>`
    * `[✅]`   `IPairingArithmetic`, the arithmetic trait, `pub trait IPairingArithmetic: IPairingAdapter<G1: Zeroize, G2: Zeroize>` with `type Gt: Zeroize;` and `type EncodedGt: AsRef<[u8]> + Zeroize;`, the encoded target-group type concrete-owned and distinct from the encoded types `IPairingAdapter` supplies; the methods `add_scalar(&self, params: AddScalarParams, payload: AddScalarPayload<Self::Scalar>) -> AddScalarReturn<Self::Scalar>`, `mul_scalar` and `neg_scalar` over `Self::Scalar`, `neg_g1(&self, params: NegG1Params, payload: NegG1Payload<Self::G1>) -> NegG1Return<Self::G1>`, `neg_g2` over `Self::G2`, `is_identity_g1(&self, params: IsIdentityG1Params, payload: IsIdentityG1Payload<Self::G1>) -> IsIdentityG1Return`, `is_identity_g2` over `Self::G2`, `pairing_product(&self, params: PairingProductParams, payload: PairingProductPayload<Self::G1, Self::G2>) -> PairingProductReturn<Self::Gt>`, and `encode_gt(&self, params: EncodeGtParams, payload: EncodeGtPayload<Self::Gt>) -> EncodeGtReturn<Self::EncodedGt>`
    * `[✅]`   `scalar_field_order`: the fieldless `ScalarFieldOrderParams` and `ScalarFieldOrderPayload`; `ScalarFieldOrderSuccessReturn` with `pub bytes: Vec<u8>`, the order's 32 big-endian bytes; `ScalarFieldOrderReturn`, the alias `Result<ScalarFieldOrderSuccessReturn, Infallible>`
    * `[✅]`   `g1_outside_subgroup_encoding`: the fieldless `G1OutsideSubgroupEncodingParams` and `G1OutsideSubgroupEncodingPayload`; `G1OutsideSubgroupEncodingSuccessReturn<E>` with `pub bytes: Option<E>`, the point's precompile encoding, `None` where the first group is the whole curve; `G1OutsideSubgroupEncodingErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]` and the variant `SearchExhausted`, the ascending search ending with no such point; `G1OutsideSubgroupEncodingReturn<E>`, the alias `Result<G1OutsideSubgroupEncodingSuccessReturn<E>, G1OutsideSubgroupEncodingErrorReturn>`
    * `[✅]`   `g2_outside_subgroup_encoding`: the fieldless `G2OutsideSubgroupEncodingParams` and `G2OutsideSubgroupEncodingPayload`; `G2OutsideSubgroupEncodingSuccessReturn<E>` with `pub bytes: E`; `G2OutsideSubgroupEncodingErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]` and the variant `SearchExhausted`; `G2OutsideSubgroupEncodingReturn<E>`, the alias `Result<G2OutsideSubgroupEncodingSuccessReturn<E>, G2OutsideSubgroupEncodingErrorReturn>`
    * `[✅]`   `IPairingReference`, the reference trait, `pub trait IPairingReference: IPairingAdapter` with the methods `scalar_field_order(&self, params: ScalarFieldOrderParams, payload: ScalarFieldOrderPayload) -> ScalarFieldOrderReturn`, `g1_outside_subgroup_encoding(&self, params: G1OutsideSubgroupEncodingParams, payload: G1OutsideSubgroupEncodingPayload) -> G1OutsideSubgroupEncodingReturn<Self::EncodedG1>`, and `g2_outside_subgroup_encoding(&self, params: G2OutsideSubgroupEncodingParams, payload: G2OutsideSubgroupEncodingPayload) -> G2OutsideSubgroupEncodingReturn<Self::EncodedG2>`, with a doc comment on the trait stating the family's outside-the-subgroup point as the objective states it
    * `[✅]`   No derives on any type in this file beyond those stated

  * `[✅]`   `adapters/pairing/src/bn254_arkworks/interface.rs`
    * `[✅]`   Imports `core::convert::Infallible` and `PairingDeclaration`, `PairingConcrete`, `PairingCurve`, `VerifierGroupArithmetic`, `PrecompileEncoding`, `TargetGroupEncodingIdentifier`, and `PAIRING_INTERFACE_VERSION` from `crate::factory::provides`; declares nothing beyond the items below
    * `[✅]`   `Bn254ArkworksPairing`, a struct with no derives and one field, `pub(super) reduced_pairing_correction: ark_bn254::Fr`, with the doc comment "The inverse in the scalar field of the multiple by which the library's reduced pairing exceeds the identifier's exact value, `2x(6x^2 + 3x + 1)` for the curve seed `x`."
    * `[✅]`   `Bn254ArkworksPairingConstructorParams`, the unit struct `pub struct Bn254ArkworksPairingConstructorParams;`, the constructor's deps slot; `Bn254ArkworksPairingTryNewReturn`, the alias `Result<Bn254ArkworksPairing, Infallible>`, the error arm uninhabited because the adapter takes no configuration
    * `[✅]`   `Bn254ArkworksScalar`, `#[derive(Clone)]`, with one field `pub(super) value: ark_bn254::Fr`; `Bn254ArkworksG1`, `#[derive(Clone)]`, with one field `pub(super) value: ark_bn254::G1Affine`; `Bn254ArkworksG2`, `#[derive(Clone)]`, with one field `pub(super) value: ark_bn254::G2Affine`
    * `[✅]`   `Bn254ArkworksGt`, a struct with no derives and one field, `pub(super) value: ark_ec::pairing::PairingOutput<ark_bn254::Bn254>`, the target-group value
    * `[✅]`   `Bn254ArkworksEncodedG1` and `Bn254ArkworksEncodedG2`, `pub struct`s with `#[derive(Clone, PartialEq, Eq)]` and one field each, `pub(super) bytes: [u8; 64]` and `pub(super) bytes: [u8; 128]`
    * `[✅]`   `Bn254ArkworksEncodedScalar`, a `pub struct` with no derives and one field `pub(super) bytes: [u8; 32]`; `Bn254ArkworksEncodedGt`, a `pub struct` with no derives and one field `pub(super) bytes: [u8; 384]`; each is returned only inside a `Secret`
    * `[✅]`   The `pub(super)` fields admit construction only inside `bn254_arkworks` and its child modules, so no code outside the concrete constructs a value of these types or mutates an encoding's bytes
    * `[✅]`   `Bn254ArkworksPairing::DECLARATION`, an inherent `pub const` declared in this interface, `PairingDeclaration { concrete: PairingConcrete::Bn254Arkworks, curve: PairingCurve::Bn254, verifier_group_arithmetic: VerifierGroupArithmetic::FirstGroupOnly, precompile_encoding: PrecompileEncoding::Eip196Eip197, target_group_encoding: TargetGroupEncodingIdentifier::Bn254V1, adapter_version: 1, interface_version: PAIRING_INTERFACE_VERSION }`; the constructor stays settings-free and the concrete has no settings module

  * `[✅]`   `adapters/pairing/src/bn254_arkworks/interaction.spec.md`
    * `[✅]`   The title `` # `bn254_arkworks` — interaction spec `` and one opening sentence stating the file as the branch contract for the `bn254_arkworks` module of the `pairing` crate, the pairing adapter over arkworks' BN254 encoding group elements as EIP-196 and EIP-197 precompile input, each branch stating condition, decision, dependency call, and the exact return outcome; then one `##` section per constructor, constant, and method, each headed by its full signature and holding a table with the columns `Branch`, `Condition`, `Decision`, `Dependency call`, and `Outcome`, in this order: `try_new`, `DECLARATION`, `g1_generator`, `g2_generator`, `add_g1`, `add_g2`, `mul_g1`, `mul_g2`, `msm_g1`, `msm_g2`, `pairing_product_is_one`, `decode_g1`, `decode_g2`, `decode_scalar`, `encode_g1`, `encode_g2`, `encode_scalar`, `UNIFORM_BYTES_LENGTH`, `sample_from_uniform_bytes`, `add_scalar`, `mul_scalar`, `neg_scalar`, `neg_g1`, `neg_g2`, `is_identity_g1`, `is_identity_g2`, `pairing_product`, `encode_gt`, `scalar_field_order`, `g1_outside_subgroup_encoding`, `g2_outside_subgroup_encoding`, then `Ordering and edges`
    * `[✅]`   `try_new`: one branch, construct; condition any params; decision none; dependency calls, in order: `Fr::from(BN254_SEED)` as `seed`; `(seed * seed * Fr::from(6u64) + seed * Fr::from(3u64) + Fr::ONE) * seed * Fr::from(2u64)` as `multiple`; `let mut exponent = Fr::MODULUS;` then `let _ = exponent.sub_with_borrow(&BigInt::from(2u64));`, the group order minus two; `multiple.pow(exponent)` as `reduced_pairing_correction`; outcome `Ok(Bn254ArkworksPairing { reduced_pairing_correction })`; the error arm has no branch, `Infallible` being uninhabited
    * `[✅]`   `DECLARATION`: an inherent constant held by `bn254_arkworks/interface.rs`, `PairingDeclaration { concrete: PairingConcrete::Bn254Arkworks, curve: PairingCurve::Bn254, verifier_group_arithmetic: VerifierGroupArithmetic::FirstGroupOnly, precompile_encoding: PrecompileEncoding::Eip196Eip197, target_group_encoding: TargetGroupEncodingIdentifier::Bn254V1, adapter_version: 1, interface_version: PAIRING_INTERFACE_VERSION }`, read through `pairing/lookup_pairing_declaration` without an instance, never obtained from a constructed adapter
    * `[✅]`   `g1_generator`, `g2_generator`: one branch each, generated; condition any; decision none; dependency call `G1Affine::generator()` or `G2Affine::generator()`; outcome `Ok(G1GeneratorSuccessReturn { point })` or `Ok(G2GeneratorSuccessReturn { point })`, the generator in the owned group type
    * `[✅]`   `add_g1`, `add_g2`: one branch each, summed; dependency call the affine `+` of `payload.left.value` and `payload.right.value`, then `into_affine`; outcome `Ok(AddG1SuccessReturn { sum })` or `Ok(AddG2SuccessReturn { sum })`
    * `[✅]`   `mul_g1`, `mul_g2`: one branch each, multiplied; dependency call the affine `payload.point.value` `*` the scalar's `Fr`, then `into_affine`; outcome `Ok(MulG1SuccessReturn { product })` or `Ok(MulG2SuccessReturn { product })`; the payload, holding the scalar, drops at the end of the call and the scalar is zeroized
    * `[✅]`   `msm_g1`, `msm_g2`: one branch each, summed; dependency call split the terms into a `Vec` of affine bases and a `Vec<Fr>` of the same length, then `G1Projective::msm_unchecked` or `G2Projective::msm_unchecked` over them, then `into_affine`; outcome `Ok(MsmG1SuccessReturn { sum })` or `Ok(MsmG2SuccessReturn { sum })`; the `Vec<Fr>` is zeroized after the call; an empty term list yields the identity
    * `[✅]`   `pairing_product_is_one`: one branch, evaluated; dependency call `Bn254::multi_pairing` over the terms' first-group elements and second-group elements in term order; outcome `Ok(PairingProductIsOneSuccessReturn { is_one })`, `is_one` being the output's `is_zero()`, the identity of the target group in arkworks' additive notation; an empty term list yields `is_one: true`, as EIP-197 does for empty input
    * `[✅]`   `decode_g1`, its branches in order: wrong length, condition `payload.len() != 64`, decision the length check, no dependency call, outcome `Err(DecodeG1ErrorReturn::WrongLength { expected: 64, actual: payload.len() })`; non-canonical coordinate, condition either 32-byte half read by `Fq::from_be_bytes_mod_order` does not re-encode through `into_bigint().to_bytes_be()` to the same 32 bytes, that is, it is at least the base field modulus, decision the re-encoding comparison per half, outcome `Err(DecodeG1ErrorReturn::NonCanonicalCoordinate)`; identity, condition both coordinates zero, decision the zero check, no dependency call, outcome `Ok(DecodeG1SuccessReturn { point })` holding `G1Affine::identity()`; off the curve, condition `G1Affine::new_unchecked(x, y).is_on_curve()` is false, outcome `Err(DecodeG1ErrorReturn::NotOnCurve)`; outside the subgroup, condition `is_in_correct_subgroup_assuming_on_curve()` is false, outcome `Err(DecodeG1ErrorReturn::NotInSubgroup)`; valid, condition every check passes, outcome `Ok(DecodeG1SuccessReturn { point })`; the section states that BN254's first group has cofactor one, so no on-curve point takes the subgroup branch and it has no unit test, while the check runs on every decode
    * `[✅]`   `decode_g2`: the same branches in the same order over 128 bytes read as `x.c1`, `x.c0`, `y.c1`, `y.c0`, each 32 bytes, assembled by `Fq2::new(c0, c1)`, with `DecodeG2ErrorReturn` and `expected: 128`; the identity is all four coordinates zero; the section states that the subgroup branch is reachable, since BN254's second group has a nontrivial cofactor
    * `[✅]`   `decode_scalar`, its branches in order: wrong length, condition `payload.len() != 32`, outcome `Err(DecodeScalarErrorReturn::WrongLength { expected: 32, actual: payload.len() })`; non-canonical, condition the bytes read by `Fr::from_be_bytes_mod_order` do not re-encode to the same 32 bytes, that is, they are at least the group order, decision the re-encoding comparison, outcome `Err(DecodeScalarErrorReturn::NonCanonical)`; valid, outcome `Ok(DecodeScalarSuccessReturn { scalar })`
    * `[✅]`   `encode_g1`: one branch, encoded; decision the identity check; dependency call `xy()`, then `into_bigint().to_bytes_be()` per coordinate, each written to its fixed 32-byte position; outcome `Ok(EncodeG1SuccessReturn { bytes })` holding `Bn254ArkworksEncodedG1`, 64 zero bytes for the identity and otherwise `x` then `y`, each 32 bytes big-endian
    * `[✅]`   `encode_g2`: one branch, encoded; decision the identity check; dependency call `xy()`, then `into_bigint().to_bytes_be()` per coefficient, each written to its fixed 32-byte position; outcome `Ok(EncodeG2SuccessReturn { bytes })` holding `Bn254ArkworksEncodedG2`, 128 zero bytes for the identity and otherwise `x.c1`, `x.c0`, `y.c1`, `y.c0`, each 32 bytes big-endian
    * `[✅]`   `encode_scalar`: one branch, encoded; dependency call `into_bigint().to_bytes_be()`; outcome `Ok(EncodeScalarSuccessReturn { bytes })`, the scalar's 32 big-endian bytes in `Bn254ArkworksEncodedScalar` moved into a `Secret`
    * `[✅]`   `UNIFORM_BYTES_LENGTH`: `64`, twice the byte width of the group order, so the reduction's bias from uniform is below two to the minus two hundred fifty
    * `[✅]`   `sample_from_uniform_bytes`, its branches in order: wrong length, condition `payload.uniform.expose().len() != 64`, dependency call `Secret::expose`, outcome `Err(SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual })`; sampled, condition the length is 64, dependency call `Fr::from_be_bytes_mod_order` over the exposed bytes, outcome `Ok(SampleUniformScalarSuccessReturn { scalar })`, the scalar moved into a `Secret`; the payload's `Secret` zeroizes the input when it drops
    * `[✅]`   `add_scalar`: one branch; dependency call `Fr`'s `+` over `payload.left.value` and `payload.right.value`; outcome `Ok(AddScalarSuccessReturn { sum })`, the sum modulo the group order in the owned scalar type; the payload, holding both scalars, drops at the end of the call and both are zeroized
    * `[✅]`   `mul_scalar`: one branch; dependency call `Fr`'s `*`; outcome `Ok(MulScalarSuccessReturn { product })` modulo the group order; the payload's scalars are zeroized as it drops
    * `[✅]`   `neg_scalar`: one branch; dependency call `Fr`'s unary `-`; outcome `Ok(NegScalarSuccessReturn { negation })`, the group order minus the scalar, and zero for zero
    * `[✅]`   `neg_g1`, `neg_g2`: one branch each; dependency call the affine point's unary `-`; outcome `Ok(NegG1SuccessReturn { negation })` or `Ok(NegG2SuccessReturn { negation })`, `(x, p - y)` for a point `(x, y)` and the identity for the identity
    * `[✅]`   `is_identity_g1`, `is_identity_g2`: one branch each; dependency call `AffineRepr::is_zero()` on the payload point; outcome `Ok(IsIdentityG1SuccessReturn { is_identity })` or `Ok(IsIdentityG2SuccessReturn { is_identity })`, `true` exactly for the identity
    * `[✅]`   `pairing_product`: one branch, evaluated; dependency call split the terms into a `Vec<G1Affine>` and a `Vec<G2Affine>` in term order, then `Bn254::multi_pairing(&g1s, &g2s)`, then the `PairingOutput` `*` `self.reduced_pairing_correction`, the exponentiation that brings the library's reduced pairing to the identifier's exact value, then `zeroize` on both vectors; outcome `Ok(PairingProductSuccessReturn { product })` holding the corrected `PairingOutput` in the owned target-group type; an empty term list yields the target group's identity, which the exponentiation preserves
    * `[✅]`   `encode_gt`: one branch, encoded; dependency call `into_bigint().to_bytes_be()` on each of the twelve `Fq` coefficients of `payload.value.value.0` in the tower order `c0.c0.c0`, `c0.c0.c1`, `c0.c1.c0`, `c0.c1.c1`, `c0.c2.c0`, `c0.c2.c1`, `c1.c0.c0`, `c1.c0.c1`, `c1.c1.c0`, `c1.c1.c1`, `c1.c2.c0`, `c1.c2.c1`, each written to its fixed 32-byte position in a 384-byte buffer; outcome `Ok(EncodeGtSuccessReturn { bytes })`, a `Secret<Bn254ArkworksEncodedGt>` built from the complete buffer with no variable-width intermediate; the target group's identity encodes as 31 zero bytes, `01`, and 352 zero bytes
    * `[✅]`   `scalar_field_order`: one branch, read; condition any; decision none; dependency call `Fr::MODULUS.to_bytes_be()`; outcome `Ok(ScalarFieldOrderSuccessReturn { bytes })`, the group order's 32 big-endian bytes; the error arm has no branch
    * `[✅]`   `g1_outside_subgroup_encoding`: one branch, absent; condition any; decision none; dependency call none; outcome `Ok(G1OutsideSubgroupEncodingSuccessReturn { bytes: None })`; the section states that BN254's first group has cofactor one, so every on-curve point is in the subgroup, and that no branch produces `SearchExhausted`
    * `[✅]`   `g2_outside_subgroup_encoding`, exhausted: condition `(1u64..).find_map(…)` over the search below returns `None`; decision the search's result; outcome `Err(G2OutsideSubgroupEncodingErrorReturn::SearchExhausted)`; the section states that the second group's cofactor exceeds the group order, so an on-curve point is outside the subgroup with overwhelming probability and the search ends among the least values of `c0`, that no input takes this branch, and that it has no unit test
    * `[✅]`   `g2_outside_subgroup_encoding`, found: condition the search returns a point and its `y`; decision, per `c0` in ascending order, `G2Affine::get_point_from_x_unchecked(Fq2::new(Fq::from(c0), Fq::from(0u64)), false)`, kept when `is_in_correct_subgroup_assuming_on_curve()` is false, its `y` read through `xy()` inside the search so a value yielding no coordinates continues the search; then, with `negated = -y`, the point is kept when `(y.c1.into_bigint(), y.c0.into_bigint())` is not greater than `(negated.c1.into_bigint(), negated.c0.into_bigint())` and replaced by its affine negation otherwise; dependency call `self.encode_g2(EncodeG2Params, EncodeG2Payload { point: Bn254ArkworksG2 { value } })`, unpacked irrefutably; outcome `Ok(G2OutsideSubgroupEncodingSuccessReturn { bytes })`, the `Bn254ArkworksEncodedG2` `encode_g2` returns
    * `[✅]`   `Ordering and edges`, a bulleted section: every decoder checks in the stated order, length, then canonicality, then identity, then curve, then subgroup, and slices the payload only after the length check; an empty `msm` term list yields the identity, and an empty `pairing_product_is_one` term list yields `is_one: true`; `Bn254ArkworksScalar`, `Bn254ArkworksG1`, `Bn254ArkworksG2`, and `Bn254ArkworksGt` each zeroize their `value` through their `Zeroize` implementation and on drop, so every clone a consumer places in a payload is zeroized when the payload drops; the outside-the-subgroup search is ascending from `c0 = 1` and stops at the first on-curve point outside the subgroup, the choice between a point and its negation follows the search and precedes the encoding, and the same call always returns the same bytes; `params` carries no control and is not read in any method, and no reference method reads its payload

  * `[✅]`   `adapters/pairing/src/factory/mock.rs`
    * `[✅]`   Module-level `#![allow(dead_code, clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::too_many_arguments)]`; imports from `super::interface` every type the items below name, `core::convert::Infallible`, `domain::{Secret, SecretConstructorParams, build_secret}`, and `zeroize::{Zeroize, ZeroizeOnDrop}`
    * `[✅]`   `PairingDeclarationOverrides`, `#[derive(Default)]`, one `Option` per field of `PairingDeclaration`, `concrete` as `Option<PairingConcrete>`; `build_pairing_declaration(overrides: PairingDeclarationOverrides) -> PairingDeclaration`, defaulting `concrete` to `PairingConcrete::Bn254Arkworks`, `curve` to `PairingCurve::Bn254`, `verifier_group_arithmetic` to `VerifierGroupArithmetic::FirstGroupOnly`, `precompile_encoding` to `PrecompileEncoding::Eip196Eip197`, `target_group_encoding` to `TargetGroupEncodingIdentifier::Bn254V1`, `adapter_version` to `1`, and `interface_version` to `PAIRING_INTERFACE_VERSION`
    * `[✅]`   For each struct below that is generic over a group, scalar, encoded, or target-group type, an overrides struct named by the type with the suffix `Overrides`, one `Option` per field over the struct's type parameters, with a hand-written `impl Default` initializing every field to `None`, since a derived `Default` bounds the type parameter; and a builder `build_` followed by the type's name in snake case, `build_x<P: IPairingAdapter>(pairing: &P, overrides: XOverrides<…>) -> X<…>`, with `P: IPairingArithmetic` where a target-group or arithmetic type appears, typed over the pairing's associated types, bounded by no `Default`, and by `Zeroize` where the production type requires it; every call site passes the pairing it holds as the leading argument; an omitted field takes the default stated for it, computed from the pairing's own operations
    * `[✅]`   The pairing-sourced builders: `G1GeneratorSuccessReturn`, `G2GeneratorSuccessReturn`, `AddG1Payload`, `AddG1SuccessReturn`, `AddG2Payload`, `AddG2SuccessReturn`, `MulG1Payload`, `MulG1SuccessReturn`, `MulG2Payload`, `MulG2SuccessReturn`, `MsmG1Term`, `MsmG1Payload` with `terms` defaulting to an empty `Vec`, `MsmG1SuccessReturn`, `MsmG2Term`, `MsmG2Payload` with `terms` defaulting to an empty `Vec`, `MsmG2SuccessReturn`, `PairingProductTerm`, `PairingProductIsOnePayload` with `terms` defaulting to an empty `Vec`, `DecodeG1SuccessReturn`, `DecodeG2SuccessReturn`, `DecodeScalarSuccessReturn`, `EncodeG1Payload`, `EncodeG2Payload`, `EncodeScalarPayload`, `EncodeG1SuccessReturn`, `EncodeG2SuccessReturn`, `EncodeScalarSuccessReturn` with `bytes` the `Secret` of the pairing's `encode_scalar` of the default scalar, `SampleUniformScalarSuccessReturn` with `scalar` the `Secret` that `P::Scalar::sample_from_uniform_bytes` returns for uniform bytes of value one, `AddScalarPayload`, `AddScalarSuccessReturn`, `MulScalarPayload`, `MulScalarSuccessReturn`, `NegScalarPayload`, `NegScalarSuccessReturn`, `NegG1Payload`, `NegG1SuccessReturn`, `NegG2Payload`, `NegG2SuccessReturn`, `IsIdentityG1Payload`, `IsIdentityG2Payload`, `PairingProductPayload` with `terms` defaulting to an empty `Vec`, `PairingProductSuccessReturn`, `EncodeGtPayload`, `EncodeGtSuccessReturn` with `bytes` the `Secret` of the pairing's `encode_gt` of the default target-group value, `G1OutsideSubgroupEncodingSuccessReturn` with `bytes` defaulting to `None`, and `G2OutsideSubgroupEncodingSuccessReturn`; the defaults come from the pairing's own operations: a first-group point is `g1_generator`'s, a second-group point `g2_generator`'s, a scalar the one `P::Scalar::sample_from_uniform_bytes` returns for uniform bytes of value one, an encoded point `encode_g1` or `encode_g2` of that default, an encoded scalar `encode_scalar` of that default, a target-group value the `pairing_product` over the term of the two generators, and an encoded target-group value `encode_gt` of that product; each default composes the nested builder of the type it holds
    * `[✅]`   The non-generic builders: `PairingProductIsOneSuccessReturnOverrides` with `build_pairing_product_is_one_success_return`, `is_one` defaulting to `true`; `SampleUniformScalarPayloadOverrides` with `build_sample_uniform_scalar_payload`, `uniform` defaulting to `build_secret` holding `vec![0u8; 64]`; `IsIdentityG1SuccessReturnOverrides` with `build_is_identity_g1_success_return` and `IsIdentityG2SuccessReturnOverrides` with `build_is_identity_g2_success_return`, `is_identity` defaulting to `false`; `ScalarFieldOrderSuccessReturnOverrides` with `build_scalar_field_order_success_return`, `bytes` defaulting to an empty `Vec`
    * `[✅]`   The mock concrete's toy algebra, `pub(crate)` throughout: `MOCK_GROUP_ORDER`, a `const` of type `u64` with value `65521`, a prime; `MockScalar`, `MockG1`, and `MockGt`, each a struct with one `u64` residue modulo `MOCK_GROUP_ORDER`, and `MockG2`, a struct with one `u64` residue modulo twice `MOCK_GROUP_ORDER`, whose subgroup is the even residues; each derives `Clone`, implements `Zeroize` by clearing the residue and `ZeroizeOnDrop` through a `Drop` that does, and implements no `Default`; the generators are the residue `1` for the first group and the residue `2` for the second group
    * `[✅]`   `MockEncodedG1`, `MockEncodedG2`, and `MockEncodedGt`, distinct `pub(crate)` structs each holding a private `[u8; 8]`, the residue in big-endian, and `MockEncodedScalar`, a `pub(crate)` struct holding a private `[u8; 32]`, the residue in the last eight bytes in big-endian and the first twenty-four bytes zero, the scalar's 32-byte encoding every concrete states; each implements `AsRef<[u8]>`; `MockEncodedG1` and `MockEncodedG2` derive `Clone`, `PartialEq`, and `Eq`; `MockEncodedScalar` and `MockEncodedGt` implement `Zeroize`
    * `[✅]`   `MockIPairingAdapterConstructorParams`, a `pub(crate)` struct with `pub failure_mode: MockIPairingAdapterFailureMode`; `MockIPairingAdapterConstructorParamsOverrides`, `#[derive(Default)]`, one field `pub failure_mode: Option<MockIPairingAdapterFailureMode>`; `build_mock_i_pairing_adapter_constructor_params(overrides: MockIPairingAdapterConstructorParamsOverrides) -> MockIPairingAdapterConstructorParams`, the mode defaulting to `MockIPairingAdapterFailureMode::Succeeds`; no corruptions type and no invalidator, since the params arrive typed from the factory
    * `[✅]`   `MockIPairingAdapter`, the family's mock concrete, a `pub(crate) struct` with `pub(super) failure_mode: MockIPairingAdapterFailureMode`; `MockIPairingAdapterTryNewReturn`, the alias `Result<MockIPairingAdapter, Infallible>`; `impl MockIPairingAdapter` with `pub(crate) fn try_new(params: MockIPairingAdapterConstructorParams) -> MockIPairingAdapterTryNewReturn` returning `Ok` holding the params' mode and setting `MOCK_SAMPLING_MODE` to it; `build_mock_i_pairing_adapter(overrides: MockIPairingAdapterConstructorParamsOverrides) -> MockIPairingAdapter`, the real instance from `try_new` through the irrefutable pattern `let Ok(adapter) = …;`; the type is crate-visible and the public surface carries the configuration that selects it; it takes no concrete as a type parameter and borrows nothing from one
    * `[✅]`   `impl IPairingAdapter for MockIPairingAdapter`, self-contained and answering the family's contract by computing from its inputs: `Scalar`, `G1`, and `G2` are `MockScalar`, `MockG1`, and `MockG2`, and the encoded types are `MockEncodedG1`, `MockEncodedG2`, and `MockEncodedScalar`; the generators are the residues `1` and `2`; `add_g1` and `add_g2` add residues modulo the group's modulus; `mul_g1` and `mul_g2` multiply the point's residue by the scalar's modulo the group's modulus; `msm_g1` and `msm_g2` sum the products, an empty list yielding residue `0`, the identity; `pairing_product_is_one` is true exactly when the sum over the terms of the first-group residue times half the second-group residue, modulo `MOCK_GROUP_ORDER`, is `0`, an empty list yielding true; `encode_g1` and `encode_g2` write the residue as eight big-endian bytes, and `encode_scalar` writes it as the scalar's 32 big-endian bytes, the first twenty-four zero, moving the bytes into a `Secret`; `decode_g1` and `decode_g2` check in the contract's order: length against eight bytes with `WrongLength { expected: 8, actual }`; canonicality against the group's modulus with `NonCanonicalCoordinate`; identity at residue `0` with `Ok`; curve, where `NotOnCurve` is returned under the mode `DecodeG1NotOnCurve` or `DecodeG2NotOnCurve` and every residue is otherwise on the curve; subgroup, where `decode_g1` returns `NotInSubgroup` under the mode `DecodeG1NotInSubgroup` and `decode_g2` returns `NotInSubgroup` for an odd residue, an arm the input reaches; then `Ok`; `decode_scalar` checks length against 32 bytes with `WrongLength { expected: 32, actual }` and then canonicality of the 32-byte big-endian integer against `MOCK_GROUP_ORDER` with `NonCanonical`, a nonzero byte among the first twenty-four being at or above it
    * `[✅]`   `impl ISampleUniformScalar for MockScalar`: `UNIFORM_BYTES_LENGTH` is `64`, the mock concrete's sampling bound, the length every real scalar of the family states so a caller under the declared hash-to-scalar concretes' digest bounds is served; `sample_from_uniform_bytes` returns `Err(SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual })` for any other length, and otherwise `Ok` with the low sixteen big-endian bytes reduced modulo `MOCK_GROUP_ORDER`, moved into a `Secret`; under the mode `SampleScalarWrongLength` it returns `Err(SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual })` for a sixty-four-byte input as well, `actual` being the input's length; the sampler holds no adapter, so the mode reaches it through `MOCK_SAMPLING_MODE`, a `pub(crate)` `thread_local!` `Cell<MockIPairingAdapterFailureMode>` initialized to `Succeeds`, which `MockIPairingAdapter::try_new` sets to its params' mode and `sample_from_uniform_bytes` reads
    * `[✅]`   `impl IPairingArithmetic for MockIPairingAdapter` with `Gt` as `MockGt` and `EncodedGt` as `MockEncodedGt`: `add_scalar`, `mul_scalar`, and `neg_scalar` compute modulo `MOCK_GROUP_ORDER`; `neg_g1` and `neg_g2` return the modulus minus the residue, `0` for `0`; `is_identity_g1` and `is_identity_g2` are true exactly at residue `0`; `pairing_product` returns the `MockGt` residue that `pairing_product_is_one` sums, an empty list yielding `0`; `encode_gt` writes the residue as eight big-endian bytes inside a `Secret`
    * `[✅]`   `impl IPairingReference for MockIPairingAdapter`: `scalar_field_order` returns `MOCK_GROUP_ORDER` as 32 big-endian bytes, the first twenty-four zero; `g1_outside_subgroup_encoding` returns `Ok` with `bytes: None`, the first group having cofactor one, or `Err(G1OutsideSubgroupEncodingErrorReturn::SearchExhausted)` under the mode `G1OutsideSubgroupSearchExhausted`; `g2_outside_subgroup_encoding` returns `Ok` with the encoding of residue `1`, the least odd residue and so outside the second group's subgroup, or `Err(G2OutsideSubgroupEncodingErrorReturn::SearchExhausted)` under the mode `G2OutsideSubgroupSearchExhausted`
    * `[✅]`   `MOCK_I_PAIRING_ADAPTER_DECLARATION`, a `pub(crate)` constant under `#[cfg(any(test, feature = "mocks"))]`, `PairingDeclaration { concrete: PairingConcrete::Mock(MockIPairingAdapterFailureMode::Succeeds), curve: PairingCurve::Bn254, verifier_group_arithmetic: VerifierGroupArithmetic::FirstGroupOnly, precompile_encoding: PrecompileEncoding::Eip196Eip197, target_group_encoding: TargetGroupEncodingIdentifier::Bn254V1, adapter_version: 1, interface_version: PAIRING_INTERFACE_VERSION }`, carrying only real Bn254-provided values; `pairing/lookup_pairing_declaration` returns it for the mock member with `concrete` set to the identifier the caller looked up, as `random/lookup_random_declaration` does for its mock
    * `[✅]`   `PairingConcreteOverrides`, the fieldless overrides struct, and `build_pairing_concrete(concrete: PairingConcrete, overrides: PairingConcreteOverrides) -> PairingConcrete`, the family's declared-selection builder, an exhaustive `match` over `PAIRING_CONCRETES` whose real `Bn254Arkworks` arm returns the selection with no configuration; the failure mode stays only in the mock selection's configuration, outside the declared real set
    * `[✅]`   No builder for the fieldless params and payloads, for the error enums, or for the declaration's value enums, each used by its production value; no corruptions type and no invalidator, since no struct this interface owns arrives as untrusted data and the decoders take the untrusted bytes directly

  * `[✅]`   `adapters/pairing/src/bn254_arkworks/mock.rs`
    * `[✅]`   Module-level `#![allow(dead_code, clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::too_many_arguments)]`; imports `super::interface::{Bn254ArkworksEncodedGt, Bn254ArkworksEncodedScalar, Bn254ArkworksG1, Bn254ArkworksG2, Bn254ArkworksGt, Bn254ArkworksPairing, Bn254ArkworksPairingConstructorParams, Bn254ArkworksScalar}`, `super::BN254_SEED`, `ark_bn254::{Bn254, Fq, Fq2, Fq12, Fr, G1Affine, G2Affine}`, `ark_ec::{AffineRepr, pairing::{Pairing, PairingOutput}}`, and `ark_ff::{BigInteger, Field, PrimeField}`; and, `#[cfg(test)]`, `ark_bn254::Fq6`, `ark_ec::CurveGroup`, `hex::decode`, `num_bigint::BigUint`, and `zeroize::ZeroizeOnDrop`
    * `[✅]`   The defaults of the concrete's owned types, each stated in its builder, with no `Default` implementation on any of them: the scalar `Fr::from(1u64)`; the first-group point `G1Affine::generator()`; the second-group point `G2Affine::generator()`; the target-group value `Bn254::pairing(G1Affine::generator(), G2Affine::generator())`, a non-identity target-group value
    * `[✅]`   The builders for the concrete's owned types, each overrides struct `#[derive(Default)]` with one `Option` field and each omitted value taking the default stated above: `Bn254ArkworksScalarOverrides` with `pub value: Option<Fr>` and `build_bn254_arkworks_scalar(overrides: Bn254ArkworksScalarOverrides) -> Bn254ArkworksScalar`; `Bn254ArkworksG1Overrides` with `pub value: Option<G1Affine>` and `build_bn254_arkworks_g1`; `Bn254ArkworksG2Overrides` with `pub value: Option<G2Affine>` and `build_bn254_arkworks_g2`; `Bn254ArkworksGtOverrides` with `pub value: Option<PairingOutput<Bn254>>` and `build_bn254_arkworks_gt`; no corruptions type and no invalidator, since none of these arrives as untrusted data
    * `[✅]`   `build_bn254_arkworks_pairing() -> Bn254ArkworksPairing`, the real instance from `Bn254ArkworksPairing::try_new(Bn254ArkworksPairingConstructorParams)` through `let Ok(pairing) = …;`; the constructor params are fieldless, so the builder takes no overrides
    * `[✅]`   The builders for the concrete's encoded scalar and encoded target-group types, each overrides struct `#[derive(Default)]` with one `Option` field: `Bn254ArkworksEncodedScalarOverrides` with `pub bytes: Option<[u8; 32]>` and `build_bn254_arkworks_encoded_scalar(overrides: Bn254ArkworksEncodedScalarOverrides) -> Bn254ArkworksEncodedScalar`, the bytes defaulting to `[1u8; 32]`; `Bn254ArkworksEncodedGtOverrides` with `pub bytes: Option<[u8; 384]>` and `build_bn254_arkworks_encoded_gt`, the bytes defaulting to `[1u8; 384]`; the defaults are nonzero so a zeroized value differs from a built one; no corruptions type and no invalidator, since neither arrives as untrusted data
    * `[✅]`   The test fixtures, each `#[cfg(test)]`, since they use the crate's dev-dependencies; constants, each a `const … : &str` of hex: `BASE_FIELD_MODULUS_HEX` `30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47`; `GROUP_ORDER_HEX` `30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001`; `GROUP_ORDER_MINUS_ONE_HEX` `30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000000`; `GROUP_ORDER_MINUS_TWO_HEX` `30644e72e131a029b85045b68181585d2833e84879b9709143e1f593efffffff`; `G1_GENERATOR_HEX`, 31 zero bytes and `01` then 31 zero bytes and `02`; `G2_GENERATOR_HEX`, `198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c2` `1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed` `090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b` `12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa`; `G2_GENERATOR_OFF_CURVE_HEX`, the same with the last byte `aa` replaced by `ab`; `G2_X_C1_AT_MODULUS_HEX`, the 32 bytes of `BASE_FIELD_MODULUS_HEX` followed by the last three 32-byte quarters of `G2_GENERATOR_HEX`; `NEG_G1_GENERATOR_HEX`, 31 zero bytes and `01` then `30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd45`; `G1_X_AT_MODULUS_HEX`, the 32 bytes of `BASE_FIELD_MODULUS_HEX` then 31 zero bytes and `02`; `G1_OFF_CURVE_HEX`, 31 zero bytes and `01` then 31 zero bytes and `03`, the point `(1, 3)`, which satisfies `y^2 = x^3 + 3` on neither side, 9 against 4; `UNIFORM_GROUP_ORDER_HEX`, 32 zero bytes then the 32 bytes of `GROUP_ORDER_HEX`, the group order as a 64-byte big-endian integer
    * `[✅]`   The test-fixture helpers, each `#[cfg(test)]`: `vector_bytes(hex: &str) -> Vec<u8>`, `decode(hex)` unpacked by `let Ok(bytes) = … else { panic!("the vector decodes") };`; `zero_bytes(length: usize) -> Vec<u8>`, `vec![0u8; length]`; `gt_identity_encoding() -> Vec<u8>`, `vec![0u8; 384]` with index `31` set to `1`; `gt_with_sequential_coefficients() -> PairingOutput<Bn254>`, `PairingOutput(Fq12::new(Fq6::new(Fq2::new(Fq::from(1u64), Fq::from(2u64)), Fq2::new(Fq::from(3u64), Fq::from(4u64)), Fq2::new(Fq::from(5u64), Fq::from(6u64))), Fq6::new(Fq2::new(Fq::from(7u64), Fq::from(8u64)), Fq2::new(Fq::from(9u64), Fq::from(10u64)), Fq2::new(Fq::from(11u64), Fq::from(12u64)))))`, whose twelve coefficients in tower order are the integers one through twelve; `sequential_gt_encoding() -> Vec<u8>`, `vec![0u8; 384]` with, for each coefficient position `i` from zero through eleven in tower order, byte `32 * i + 31` set to `i + 1`; `base_field_modulus() -> BigUint`, `BigUint::from_bytes_be(&vector_bytes(BASE_FIELD_MODULUS_HEX))`; `scalar_value(hex: &str) -> Fr`, `Fr::from_be_bytes_mod_order(&vector_bytes(hex))`; `eip_196_generator() -> G1Affine`, `G1Affine::new_unchecked(Fq::from(1u64), Fq::from(2u64))`, the generator EIP-196 publishes; `eip_196_negated_generator() -> G1Affine`, `G1Affine::new_unchecked` over the two 32-byte halves of `vector_bytes(NEG_G1_GENERATOR_HEX)`, each read by `Fq::from_be_bytes_mod_order`; `eip_197_generator() -> G2Affine`, `G2Affine::new_unchecked(Fq2::new(x_c0, x_c1), Fq2::new(y_c0, y_c1))` over the four 32-byte quarters of `vector_bytes(G2_GENERATOR_HEX)`, read in order as `x_c1`, `x_c0`, `y_c1`, `y_c0` by `Fq::from_be_bytes_mod_order`; `g2_point_from_encoding(bytes: &[u8]) -> G2Affine`, the same reading over any 128 bytes; `g2_outside_subgroup_bytes() -> Vec<u8>`, the least on-curve point outside the subgroup, `(1u64..).find_map(|c0| G2Affine::get_point_from_x_unchecked(Fq2::new(Fq::from(c0), Fq::from(0u64)), false).filter(|point| !point.is_in_correct_subgroup_assuming_on_curve()))` unpacked by `let Some(point) = … else { panic!("an on-curve point outside the subgroup exists") };`, its `xy()` unpacked by `let Some((x, y)) = … else { panic!("the point has coordinates") };`, encoded by appending `into_bigint().to_bytes_be()` of `x.c1`, `x.c0`, `y.c1`, `y.c0`; `definition_exponent() -> Vec<u64>`, `let p = BigUint::from(Fq::MODULUS); let r = BigUint::from(Fr::MODULUS); ((p.pow(12) - BigUint::from(1u32)) / r).to_u64_digits()`; `definition_value(g1: G1Affine, g2: G2Affine) -> Fq12`, `Bn254::multi_miller_loop([g1], [g2]).0.pow(definition_exponent())`, the identifier's definition of the pairing value; `eip_196_doubled_generator() -> G1Affine`, `(eip_196_generator() + eip_196_generator()).into_affine()`; `definition_value_power(g1: G1Affine, g2: G2Affine, exponent: u64) -> Fq12`, `definition_value(g1, g2).pow([exponent])`; `library_multiple() -> Fr`, `let seed = Fr::from(BN254_SEED); (seed * seed * Fr::from(6u64) + seed * Fr::from(3u64) + Fr::ONE) * seed * Fr::from(2u64)`; `requires_zeroize_on_drop<T: ZeroizeOnDrop>() {}`
    * `[✅]`   No mock function: the concrete is built as a real instance and owns no free function

  * `[✅]`   `adapters/pairing/src/factory/mod.rs`
    * `[✅]`   The module wiring: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, and `pub mod provides;`; the factory function and its unit-test module are `pairing/factory`'s

  * `[✅]`   `adapters/pairing/src/factory/provides.rs`
    * `[✅]`   `pub use super::interface::*;` and `#[cfg(any(test, feature = "mocks"))] pub use super::mock::*;`; the factory function's re-export is `pairing/factory`'s

  * `[✅]`   `adapters/pairing/src/bn254_arkworks/test.rs`
    * `[✅]`   Module-level `#![allow(clippy::expect_used)]`; imports from `super::provides` the concrete, its owned types, and the mock's builders, constants, and helpers each block uses; from `crate::factory::provides` the params, payloads, errors, trait names, member and value identifiers, declared sets, and builders each block uses; `HashSet` from `std::collections`; and the `ark_bn254`, `ark_ec`, `ark_ff`, `num_bigint::BigUint`, and `zeroize::Zeroize` names each block uses
    * `[✅]`   Compile-time assertion over `Bn254ArkworksPairing::DECLARATION`
      * `[✅]`   Contract: the inherent constant is `PairingDeclaration { concrete: PairingConcrete::Bn254Arkworks, curve: PairingCurve::Bn254, verifier_group_arithmetic: VerifierGroupArithmetic::FirstGroupOnly, precompile_encoding: PrecompileEncoding::Eip196Eip197, target_group_encoding: TargetGroupEncodingIdentifier::Bn254V1, adapter_version: 1, interface_version: PAIRING_INTERFACE_VERSION }`, read through `pairing/lookup_pairing_declaration` without an instance
      * `[✅]`   Arrange: the type alone
      * `[✅]`   Act: a module-level `const _: () = assert!(…);` reading `Bn254ArkworksPairing::DECLARATION`, `concrete` and each enum field tested with `matches!` and each version with `==`, no text-name assertion
      * `[✅]`   Assert: the module compiles only if every field holds
    * `[✅]`   `pairing_concretes_declares_only_real_members`
      * `[✅]`   Contract: the declared set holds the real members alone → `PAIRING_CONCRETES` contains `PairingConcrete::Bn254Arkworks` and no `Mock(_)` member
      * `[✅]`   Collaborators: none; the declared set is read as a constant, no instance is built
      * `[✅]`   Arrange: none; the constant alone
      * `[✅]`   Act: reading `PAIRING_CONCRETES`
      * `[✅]`   Assert: every member is `PairingConcrete::Bn254Arkworks` and none is `Mock(_)`
    * `[✅]`   `target_group_encoding_identifiers_declares_only_real_values`
      * `[✅]`   Contract: the declared set holds the real provided values alone → `TARGET_GROUP_ENCODING_IDENTIFIERS` contains `TargetGroupEncodingIdentifier::Bn254V1` and no `Mock` member
      * `[✅]`   Collaborators: none; the declared set is read as a constant, no instance is built
      * `[✅]`   Arrange: none; the constant alone
      * `[✅]`   Act: reading `TARGET_GROUP_ENCODING_IDENTIFIERS`
      * `[✅]`   Assert: every member is `TargetGroupEncodingIdentifier::Bn254V1` and none is `Mock`
    * `[✅]`   `every_declared_identifier_spells_a_distinct_name`
      * `[✅]`   Contract: each declared member has one total spelling → the `name()` spellings of the members of `PAIRING_CONCRETES` are pairwise distinct, and so are those of `TARGET_GROUP_ENCODING_IDENTIFIERS`
      * `[✅]`   Collaborators: none; the declared sets are read as constants, no instance is built
      * `[✅]`   Arrange: none; the constants alone
      * `[✅]`   Act: collecting each member's `name()` into a `HashSet` per declared set
      * `[✅]`   Assert: each `HashSet`'s length equals its declared set's length
    * `[✅]`   `no_value_identifier_declares_a_mock_only_variant`
      * `[✅]`   Contract: a value identifier holds real provided values alone → `PairingCurve`, `VerifierGroupArithmetic`, `PrecompileEncoding`, and `TargetGroupEncodingIdentifier` have no mock-only variant
      * `[✅]`   Collaborators: none; the value identifiers are read as types, no instance is built
      * `[✅]`   Arrange: none; the types alone
      * `[✅]`   Act: reading `core::mem::variant_count` on each value identifier
      * `[✅]`   Assert: each variant count equals its real variants' count, so no mock-only variant exists
    * `[✅]`   `bn254_arkworks_scalar_is_zeroize_on_drop`
      * `[✅]`   Contract: `Bn254ArkworksScalar` implements `ZeroizeOnDrop`, the marker the sampling bound requires
      * `[✅]`   Collaborators: none
      * `[✅]`   Arrange: the type alone
      * `[✅]`   Act: `requires_zeroize_on_drop::<Bn254ArkworksScalar>()`
      * `[✅]`   Assert: the block compiles only if the type implements `ZeroizeOnDrop`
    * `[✅]`   `bn254_arkworks_scalar_zeroize_clears_its_value`
      * `[✅]`   Contract: `Zeroize::zeroize` on a living scalar → its `value` is zero
      * `[✅]`   Collaborators: arkworks' `Fr` zeroization, run for real as the vendor; fixture `build_bn254_arkworks_scalar`
      * `[✅]`   Arrange: `build_bn254_arkworks_scalar` with the value override `Fr::from(5u64)`, so a scalar left unchanged differs from the expected zero
      * `[✅]`   Act: `scalar.zeroize()`
      * `[✅]`   Assert: `scalar.value` equals `Fr::from(0u64)`
    * `[✅]`   `reduced_pairing_correction_inverts_the_library_multiple`
      * `[✅]`   Contract: any params → `Ok(Bn254ArkworksPairing)` whose `reduced_pairing_correction` is the inverse in the scalar field of `2x(6x^2 + 3x + 1)` for the curve seed `x`
      * `[✅]`   Collaborators: arkworks' `Fr` arithmetic, run for real as the vendor; the expectation is `library_multiple()`, computed in the fixtures from the seed by field multiplication and independent of the Fermat inversion `try_new` performs
      * `[✅]`   Arrange: `Bn254ArkworksPairingConstructorParams` by its production value
      * `[✅]`   Act: `Bn254ArkworksPairing::try_new(Bn254ArkworksPairingConstructorParams)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(pairing) = …;`; `pairing.reduced_pairing_correction * library_multiple()` equals `Fr::from(1u64)`
    * `[✅]`   `uniform_bytes_length_is_twice_the_group_order_width`
      * `[✅]`   Contract: `UNIFORM_BYTES_LENGTH` is twice the byte width of the group order
      * `[✅]`   Collaborators: none
      * `[✅]`   Arrange: the type alone
      * `[✅]`   Act: reading `<Bn254ArkworksScalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH`
      * `[✅]`   Assert: the constant equals the literal 64 written in the assertion
    * `[✅]`   `g1_generator_returns_the_eip_196_generator`
      * `[✅]`   Contract: any call → `Ok(G1GeneratorSuccessReturn { point })` holding the first group's generator
      * `[✅]`   Collaborators: arkworks, run for real as the vendor; fixture `build_bn254_arkworks_pairing`; `G1GeneratorParams` and `G1GeneratorPayload` by their production values
      * `[✅]`   Arrange: `build_bn254_arkworks_pairing()`
      * `[✅]`   Act: `g1_generator(G1GeneratorParams, G1GeneratorPayload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.point.value` equals `eip_196_generator()`
    * `[✅]`   `g2_generator_returns_the_eip_197_generator`
      * `[✅]`   Contract: any call → `Ok(G2GeneratorSuccessReturn { point })` holding the second group's generator
      * `[✅]`   Collaborators: arkworks, run for real as the vendor; fixture `build_bn254_arkworks_pairing`; `G2GeneratorParams` and `G2GeneratorPayload` by their production values
      * `[✅]`   Arrange: `build_bn254_arkworks_pairing()`
      * `[✅]`   Act: `g2_generator(G2GeneratorParams, G2GeneratorPayload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.point.value` equals `eip_197_generator()`
    * `[✅]`   `add_g1_of_a_point_and_its_negation_is_the_identity`
      * `[✅]`   Contract: `payload.left` and `payload.right` → `Ok(AddG1SuccessReturn { sum })` holding their sum
      * `[✅]`   Collaborators: arkworks' affine addition, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_add_g1_payload`, and `build_bn254_arkworks_g1`
      * `[✅]`   Arrange: `build_add_g1_payload` with the left override `build_bn254_arkworks_g1` at its default generator and the right override `build_bn254_arkworks_g1` with the value `eip_196_negated_generator()`, so a sum that returns either input differs from the identity
      * `[✅]`   Act: `add_g1(AddG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G1Affine::identity()`
    * `[✅]`   `add_g1_of_the_identity_and_a_point_is_the_point`
      * `[✅]`   Contract: `payload.left` and `payload.right` → `Ok(AddG1SuccessReturn { sum })` holding their sum
      * `[✅]`   Collaborators: arkworks' affine addition, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_add_g1_payload`, and `build_bn254_arkworks_g1`
      * `[✅]`   Arrange: `build_add_g1_payload` with the left override `build_bn254_arkworks_g1` with the value `G1Affine::identity()` and the right override `build_bn254_arkworks_g1` at its default generator, so a sum that returns the left input differs from the generator
      * `[✅]`   Act: `add_g1(AddG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `eip_196_generator()`
    * `[✅]`   `add_g2_of_a_point_and_its_negation_is_the_identity`
      * `[✅]`   Contract: `payload.left` and `payload.right` → `Ok(AddG2SuccessReturn { sum })` holding their sum
      * `[✅]`   Collaborators: arkworks' affine addition, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_add_g2_payload`, and `build_bn254_arkworks_g2`
      * `[✅]`   Arrange: `build_add_g2_payload` with the left override `build_bn254_arkworks_g2` at its default generator and the right override `build_bn254_arkworks_g2` with the value `-eip_197_generator()`, so a sum that returns either input differs from the identity
      * `[✅]`   Act: `add_g2(AddG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G2Affine::identity()`
    * `[✅]`   `add_g2_of_the_identity_and_a_point_is_the_point`
      * `[✅]`   Contract: `payload.left` and `payload.right` → `Ok(AddG2SuccessReturn { sum })` holding their sum
      * `[✅]`   Collaborators: arkworks' affine addition, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_add_g2_payload`, and `build_bn254_arkworks_g2`
      * `[✅]`   Arrange: `build_add_g2_payload` with the left override `build_bn254_arkworks_g2` with the value `G2Affine::identity()` and the right override `build_bn254_arkworks_g2` at its default generator, so a sum that returns the left input differs from the generator
      * `[✅]`   Act: `add_g2(AddG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `eip_197_generator()`
    * `[✅]`   `mul_g1_by_one_is_the_point`
      * `[✅]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG1SuccessReturn { product })` holding the point multiplied by the scalar
      * `[✅]`   Collaborators: arkworks' affine scalar multiplication, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_mul_g1_payload`, `build_bn254_arkworks_g1`, and `build_bn254_arkworks_scalar`
      * `[✅]`   Arrange: `build_mul_g1_payload` with the point override `build_bn254_arkworks_g1` at its default generator and the scalar override `build_bn254_arkworks_scalar` with the value `Fr::from(1u64)`
      * `[✅]`   Act: `mul_g1(MulG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `eip_196_generator()`
    * `[✅]`   `mul_g1_by_zero_is_the_identity`
      * `[✅]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG1SuccessReturn { product })` holding the point multiplied by the scalar
      * `[✅]`   Collaborators: arkworks' affine scalar multiplication, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_mul_g1_payload`, `build_bn254_arkworks_g1`, and `build_bn254_arkworks_scalar`
      * `[✅]`   Arrange: `build_mul_g1_payload` with the point override `build_bn254_arkworks_g1` at its default generator and the scalar override `build_bn254_arkworks_scalar` with the value `Fr::from(0u64)`, so a product that returns the point differs from the identity
      * `[✅]`   Act: `mul_g1(MulG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `G1Affine::identity()`
    * `[✅]`   `mul_g1_by_the_group_order_minus_one_is_the_negated_generator`
      * `[✅]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG1SuccessReturn { product })` holding the point multiplied by the scalar
      * `[✅]`   Collaborators: arkworks' affine scalar multiplication, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_mul_g1_payload`, `build_bn254_arkworks_g1`, and `build_bn254_arkworks_scalar`
      * `[✅]`   Arrange: `build_mul_g1_payload` with the point override `build_bn254_arkworks_g1` at its default generator and the scalar override `build_bn254_arkworks_scalar` with the value `scalar_value(GROUP_ORDER_MINUS_ONE_HEX)`, so a product that ignores the scalar differs from the negated generator
      * `[✅]`   Act: `mul_g1(MulG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `eip_196_negated_generator()`
    * `[✅]`   `mul_g2_by_one_is_the_point`
      * `[✅]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG2SuccessReturn { product })` holding the point multiplied by the scalar
      * `[✅]`   Collaborators: arkworks' affine scalar multiplication, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_mul_g2_payload`, `build_bn254_arkworks_g2`, and `build_bn254_arkworks_scalar`
      * `[✅]`   Arrange: `build_mul_g2_payload` with the point override `build_bn254_arkworks_g2` at its default generator and the scalar override `build_bn254_arkworks_scalar` with the value `Fr::from(1u64)`
      * `[✅]`   Act: `mul_g2(MulG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `eip_197_generator()`
    * `[✅]`   `mul_g2_by_zero_is_the_identity`
      * `[✅]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG2SuccessReturn { product })` holding the point multiplied by the scalar
      * `[✅]`   Collaborators: arkworks' affine scalar multiplication, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_mul_g2_payload`, `build_bn254_arkworks_g2`, and `build_bn254_arkworks_scalar`
      * `[✅]`   Arrange: `build_mul_g2_payload` with the point override `build_bn254_arkworks_g2` at its default generator and the scalar override `build_bn254_arkworks_scalar` with the value `Fr::from(0u64)`, so a product that returns the point differs from the identity
      * `[✅]`   Act: `mul_g2(MulG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `G2Affine::identity()`
    * `[✅]`   `mul_g2_by_the_group_order_minus_one_is_the_negated_point`
      * `[✅]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG2SuccessReturn { product })` holding the point multiplied by the scalar
      * `[✅]`   Collaborators: arkworks' affine scalar multiplication, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_mul_g2_payload`, `build_bn254_arkworks_g2`, and `build_bn254_arkworks_scalar`
      * `[✅]`   Arrange: `build_mul_g2_payload` with the point override `build_bn254_arkworks_g2` at its default generator and the scalar override `build_bn254_arkworks_scalar` with the value `scalar_value(GROUP_ORDER_MINUS_ONE_HEX)`, so a product that ignores the scalar differs from the negated generator
      * `[✅]`   Act: `mul_g2(MulG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `-eip_197_generator()`
    * `[✅]`   `msm_g1_of_no_terms_is_the_identity`
      * `[✅]`   Contract: an empty `payload.terms` → `Ok(MsmG1SuccessReturn { sum })` holding the identity
      * `[✅]`   Collaborators: arkworks' multi-scalar multiplication, run for real as the vendor; fixtures `build_bn254_arkworks_pairing` and `build_msm_g1_payload`
      * `[✅]`   Arrange: `build_msm_g1_payload` at its default of no terms
      * `[✅]`   Act: `msm_g1(MsmG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G1Affine::identity()`
    * `[✅]`   `msm_g1_pairs_each_base_with_its_own_scalar`
      * `[✅]`   Contract: `payload.terms` → `Ok(MsmG1SuccessReturn { sum })` holding the sum of each base multiplied by its own scalar
      * `[✅]`   Collaborators: arkworks' multi-scalar multiplication, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_msm_g1_payload`, `build_msm_g1_term`, `build_bn254_arkworks_g1`, and `build_bn254_arkworks_scalar`
      * `[✅]`   Arrange: `build_msm_g1_payload` with the terms override holding two `build_msm_g1_term` values: the generator with the scalar `Fr::from(1u64)`, and the base `eip_196_negated_generator()` with the scalar `Fr::from(0u64)`, so bases and scalars exchanged between terms yield the negated generator
      * `[✅]`   Act: `msm_g1(MsmG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `eip_196_generator()`
    * `[✅]`   `msm_g1_sums_its_terms`
      * `[✅]`   Contract: `payload.terms` → `Ok(MsmG1SuccessReturn { sum })` holding the sum of each base multiplied by its own scalar
      * `[✅]`   Collaborators: arkworks' multi-scalar multiplication, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_msm_g1_payload`, `build_msm_g1_term`, `build_bn254_arkworks_g1`, and `build_bn254_arkworks_scalar`
      * `[✅]`   Arrange: `build_msm_g1_payload` with the terms override holding two `build_msm_g1_term` values: the generator with the scalar `Fr::from(1u64)`, and the base `eip_196_negated_generator()` with the scalar `Fr::from(1u64)`, so a sum over the first term alone differs from the identity
      * `[✅]`   Act: `msm_g1(MsmG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G1Affine::identity()`
    * `[✅]`   `msm_g2_of_no_terms_is_the_identity`
      * `[✅]`   Contract: an empty `payload.terms` → `Ok(MsmG2SuccessReturn { sum })` holding the identity
      * `[✅]`   Collaborators: arkworks' multi-scalar multiplication, run for real as the vendor; fixtures `build_bn254_arkworks_pairing` and `build_msm_g2_payload`
      * `[✅]`   Arrange: `build_msm_g2_payload` at its default of no terms
      * `[✅]`   Act: `msm_g2(MsmG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G2Affine::identity()`
    * `[✅]`   `msm_g2_pairs_each_base_with_its_own_scalar`
      * `[✅]`   Contract: `payload.terms` → `Ok(MsmG2SuccessReturn { sum })` holding the sum of each base multiplied by its own scalar
      * `[✅]`   Collaborators: arkworks' multi-scalar multiplication, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_msm_g2_payload`, `build_msm_g2_term`, `build_bn254_arkworks_g2`, and `build_bn254_arkworks_scalar`
      * `[✅]`   Arrange: `build_msm_g2_payload` with the terms override holding two `build_msm_g2_term` values: the generator with the scalar `Fr::from(1u64)`, and the base `-eip_197_generator()` with the scalar `Fr::from(0u64)`, so bases and scalars exchanged between terms yield the negated generator
      * `[✅]`   Act: `msm_g2(MsmG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `eip_197_generator()`
    * `[✅]`   `msm_g2_sums_its_terms`
      * `[✅]`   Contract: `payload.terms` → `Ok(MsmG2SuccessReturn { sum })` holding the sum of each base multiplied by its own scalar
      * `[✅]`   Collaborators: arkworks' multi-scalar multiplication, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_msm_g2_payload`, `build_msm_g2_term`, `build_bn254_arkworks_g2`, and `build_bn254_arkworks_scalar`
      * `[✅]`   Arrange: `build_msm_g2_payload` with the terms override holding two `build_msm_g2_term` values: the generator with the scalar `Fr::from(1u64)`, and the base `-eip_197_generator()` with the scalar `Fr::from(1u64)`, so a sum over the first term alone differs from the identity
      * `[✅]`   Act: `msm_g2(MsmG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G2Affine::identity()`
    * `[✅]`   `pairing_product_is_one_of_no_terms_is_true`
      * `[✅]`   Contract: an empty `payload.terms` → `Ok(PairingProductIsOneSuccessReturn { is_one })` with `is_one` true
      * `[✅]`   Collaborators: arkworks' multi-pairing, run for real as the vendor; fixtures `build_bn254_arkworks_pairing` and `build_pairing_product_is_one_payload`
      * `[✅]`   Arrange: `build_pairing_product_is_one_payload` at its default of no terms
      * `[✅]`   Act: `pairing_product_is_one(PairingProductIsOneParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_one` is true
    * `[✅]`   `pairing_product_is_one_of_a_pairing_and_its_first_group_negation_is_true`
      * `[✅]`   Contract: `payload.terms` whose pairings multiply to the target group's identity → `Ok(PairingProductIsOneSuccessReturn { is_one })` with `is_one` true
      * `[✅]`   Collaborators: arkworks' multi-pairing, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_pairing_product_is_one_payload`, `build_pairing_product_term`, and `build_bn254_arkworks_g1`
      * `[✅]`   Arrange: `build_pairing_product_is_one_payload` with the terms override holding two `build_pairing_product_term` values: the generators, and the first-group override `build_bn254_arkworks_g1` with the value `eip_196_negated_generator()` over the second-group generator
      * `[✅]`   Act: `pairing_product_is_one(PairingProductIsOneParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_one` is true
    * `[✅]`   `pairing_product_is_one_of_the_generators_is_false`
      * `[✅]`   Contract: `payload.terms` whose pairings multiply to a value other than the target group's identity → `Ok(PairingProductIsOneSuccessReturn { is_one })` with `is_one` false
      * `[✅]`   Collaborators: arkworks' multi-pairing, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_pairing_product_is_one_payload`, and `build_pairing_product_term`
      * `[✅]`   Arrange: `build_pairing_product_is_one_payload` with the terms override holding one `build_pairing_product_term` at its default generators
      * `[✅]`   Act: `pairing_product_is_one(PairingProductIsOneParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_one` is false
    * `[✅]`   `decode_g1_rejects_a_wrong_length`
      * `[✅]`   Contract: `payload.len() != 64` → `Err(DecodeG1ErrorReturn::WrongLength { expected: 64, actual: payload.len() })`
      * `[✅]`   Collaborators: none called; fixture `build_bn254_arkworks_pairing`; the untrusted bytes from `zero_bytes`
      * `[✅]`   Arrange: the payload `zero_bytes(63)`, which differs from the required length
      * `[✅]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(DecodeG1ErrorReturn::WrongLength { expected: 64, actual: 63 })`, the whole expected error
    * `[✅]`   `decode_g1_rejects_a_non_canonical_coordinate`
      * `[✅]`   Contract: a 64-byte payload whose first half is at least the base field modulus → `Err(DecodeG1ErrorReturn::NonCanonicalCoordinate)`
      * `[✅]`   Collaborators: arkworks' field reduction, run for real as the vendor; fixture `build_bn254_arkworks_pairing`; the untrusted bytes from `vector_bytes(G1_X_AT_MODULUS_HEX)`
      * `[✅]`   Arrange: the payload `vector_bytes(G1_X_AT_MODULUS_HEX)`, whose reduced coordinates would otherwise fail the curve check, so a decoder that checks the curve first returns a different error
      * `[✅]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(DecodeG1ErrorReturn::NonCanonicalCoordinate)`
    * `[✅]`   `decode_g1_decodes_the_identity_from_zero_bytes`
      * `[✅]`   Contract: a 64-byte payload with both coordinates zero → `Ok(DecodeG1SuccessReturn { point })` holding the first group's identity
      * `[✅]`   Collaborators: arkworks, run for real as the vendor; fixture `build_bn254_arkworks_pairing`; the untrusted bytes from `zero_bytes`
      * `[✅]`   Arrange: the payload `zero_bytes(64)`
      * `[✅]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[✅]`   Assert: the success arm is extracted with `expect`; `success.point.value` equals `G1Affine::identity()`
    * `[✅]`   `decode_g1_rejects_a_point_off_the_curve`
      * `[✅]`   Contract: a canonical 64-byte payload that satisfies no curve equation → `Err(DecodeG1ErrorReturn::NotOnCurve)`
      * `[✅]`   Collaborators: arkworks' curve check, run for real as the vendor; fixture `build_bn254_arkworks_pairing`; the untrusted bytes from `vector_bytes(G1_OFF_CURVE_HEX)`
      * `[✅]`   Arrange: the payload `vector_bytes(G1_OFF_CURVE_HEX)`
      * `[✅]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(DecodeG1ErrorReturn::NotOnCurve)`
    * `[✅]`   `decode_g1_decodes_the_eip_196_generator`
      * `[✅]`   Contract: a canonical 64-byte payload on the curve → `Ok(DecodeG1SuccessReturn { point })` holding that point
      * `[✅]`   Collaborators: arkworks, run for real as the vendor; fixture `build_bn254_arkworks_pairing`; the untrusted bytes from `vector_bytes(G1_GENERATOR_HEX)`
      * `[✅]`   Arrange: the payload `vector_bytes(G1_GENERATOR_HEX)`
      * `[✅]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[✅]`   Assert: the success arm is extracted with `expect`; `success.point.value` equals `eip_196_generator()`
    * `[✅]`   `decode_g2_rejects_a_wrong_length`
      * `[✅]`   Contract: `payload.len() != 128` → `Err(DecodeG2ErrorReturn::WrongLength { expected: 128, actual: payload.len() })`
      * `[✅]`   Collaborators: none called; fixture `build_bn254_arkworks_pairing`; the untrusted bytes from `zero_bytes`
      * `[✅]`   Arrange: the payload `zero_bytes(127)`, which differs from the required length
      * `[✅]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(DecodeG2ErrorReturn::WrongLength { expected: 128, actual: 127 })`, the whole expected error
    * `[✅]`   `decode_g2_rejects_a_non_canonical_coordinate`
      * `[✅]`   Contract: a 128-byte payload whose first coordinate is at least the base field modulus → `Err(DecodeG2ErrorReturn::NonCanonicalCoordinate)`
      * `[✅]`   Collaborators: arkworks' field reduction, run for real as the vendor; fixture `build_bn254_arkworks_pairing`; the untrusted bytes from `vector_bytes(G2_X_C1_AT_MODULUS_HEX)`
      * `[✅]`   Arrange: the payload `vector_bytes(G2_X_C1_AT_MODULUS_HEX)`, whose reduced coordinates would otherwise fail the curve check, so a decoder that checks the curve first returns a different error
      * `[✅]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(DecodeG2ErrorReturn::NonCanonicalCoordinate)`
    * `[✅]`   `decode_g2_decodes_the_identity_from_zero_bytes`
      * `[✅]`   Contract: a 128-byte payload with every coordinate zero → `Ok(DecodeG2SuccessReturn { point })` holding the second group's identity
      * `[✅]`   Collaborators: arkworks, run for real as the vendor; fixture `build_bn254_arkworks_pairing`; the untrusted bytes from `zero_bytes`
      * `[✅]`   Arrange: the payload `zero_bytes(128)`
      * `[✅]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[✅]`   Assert: the success arm is extracted with `expect`; `success.point.value` equals `G2Affine::identity()`
    * `[✅]`   `decode_g2_rejects_a_point_off_the_curve`
      * `[✅]`   Contract: a canonical 128-byte payload that satisfies no curve equation → `Err(DecodeG2ErrorReturn::NotOnCurve)`
      * `[✅]`   Collaborators: arkworks' curve check, run for real as the vendor; fixture `build_bn254_arkworks_pairing`; the untrusted bytes from `vector_bytes(G2_GENERATOR_OFF_CURVE_HEX)`
      * `[✅]`   Arrange: the payload `vector_bytes(G2_GENERATOR_OFF_CURVE_HEX)`
      * `[✅]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(DecodeG2ErrorReturn::NotOnCurve)`
    * `[✅]`   `decode_g2_rejects_a_point_outside_the_subgroup`
      * `[✅]`   Contract: a canonical 128-byte payload on the curve and outside the prime-order subgroup → `Err(DecodeG2ErrorReturn::NotInSubgroup)`
      * `[✅]`   Collaborators: arkworks' subgroup check, run for real as the vendor; fixture `build_bn254_arkworks_pairing`; the untrusted bytes from `g2_outside_subgroup_bytes()`
      * `[✅]`   Arrange: the payload `g2_outside_subgroup_bytes()`
      * `[✅]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(DecodeG2ErrorReturn::NotInSubgroup)`
    * `[✅]`   `decode_g2_decodes_the_eip_197_generator`
      * `[✅]`   Contract: a canonical 128-byte payload on the curve and in the subgroup → `Ok(DecodeG2SuccessReturn { point })` holding that point
      * `[✅]`   Collaborators: arkworks, run for real as the vendor; fixture `build_bn254_arkworks_pairing`; the untrusted bytes from `vector_bytes(G2_GENERATOR_HEX)`
      * `[✅]`   Arrange: the payload `vector_bytes(G2_GENERATOR_HEX)`
      * `[✅]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[✅]`   Assert: the success arm is extracted with `expect`; `success.point.value` equals `eip_197_generator()`
    * `[✅]`   `decode_scalar_rejects_a_wrong_length`
      * `[✅]`   Contract: `payload.len() != 32` → `Err(DecodeScalarErrorReturn::WrongLength { expected: 32, actual: payload.len() })`
      * `[✅]`   Collaborators: none called; fixture `build_bn254_arkworks_pairing`; the untrusted bytes from `zero_bytes`
      * `[✅]`   Arrange: the payload `zero_bytes(31)`, which differs from the required length
      * `[✅]`   Act: `decode_scalar(DecodeScalarParams, &payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(DecodeScalarErrorReturn::WrongLength { expected: 32, actual: 31 })`, the whole expected error
    * `[✅]`   `decode_scalar_rejects_the_group_order`
      * `[✅]`   Contract: a 32-byte payload at least the group order → `Err(DecodeScalarErrorReturn::NonCanonical)`
      * `[✅]`   Collaborators: arkworks' field reduction, run for real as the vendor; fixture `build_bn254_arkworks_pairing`; the untrusted bytes from `vector_bytes(GROUP_ORDER_HEX)`
      * `[✅]`   Arrange: the payload `vector_bytes(GROUP_ORDER_HEX)`, whose reduction is zero and so differs from the bytes
      * `[✅]`   Act: `decode_scalar(DecodeScalarParams, &payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(DecodeScalarErrorReturn::NonCanonical)`
    * `[✅]`   `decode_scalar_decodes_the_largest_canonical_scalar`
      * `[✅]`   Contract: a canonical 32-byte payload → `Ok(DecodeScalarSuccessReturn { scalar })` holding that scalar
      * `[✅]`   Collaborators: arkworks' field reduction, run for real as the vendor; fixture `build_bn254_arkworks_pairing`; the untrusted bytes from `vector_bytes(GROUP_ORDER_MINUS_ONE_HEX)`
      * `[✅]`   Arrange: the payload `vector_bytes(GROUP_ORDER_MINUS_ONE_HEX)`
      * `[✅]`   Act: `decode_scalar(DecodeScalarParams, &payload)`
      * `[✅]`   Assert: the success arm is extracted with `expect`; `success.scalar.value` equals `-Fr::from(1u64)`
    * `[✅]`   `encode_g1_writes_the_eip_196_generator`
      * `[✅]`   Contract: a first-group point → `Ok(EncodeG1SuccessReturn { bytes })` holding `x` then `y`, each 32 bytes big-endian, in this concrete's `Bn254ArkworksEncodedG1`
      * `[✅]`   Collaborators: arkworks' coordinate encoding, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_encode_g1_payload`, and `build_bn254_arkworks_g1`
      * `[✅]`   Arrange: `build_encode_g1_payload` with the point override `build_bn254_arkworks_g1` at its default generator
      * `[✅]`   Act: `encode_g1(EncodeG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.bytes` binds to a `Bn254ArkworksEncodedG1` typed binding; its bytes equal `vector_bytes(G1_GENERATOR_HEX)` as a byte slice
    * `[✅]`   `encode_g1_writes_the_identity_as_zero_bytes`
      * `[✅]`   Contract: the first group's identity → `Ok(EncodeG1SuccessReturn { bytes })` holding 64 zero bytes
      * `[✅]`   Collaborators: arkworks, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_encode_g1_payload`, and `build_bn254_arkworks_g1`
      * `[✅]`   Arrange: `build_encode_g1_payload` with the point override `build_bn254_arkworks_g1` with the value `G1Affine::identity()`
      * `[✅]`   Act: `encode_g1(EncodeG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; the bytes of `success.bytes` equal `zero_bytes(64)` as a byte slice
    * `[✅]`   `encode_g2_writes_the_eip_197_generator`
      * `[✅]`   Contract: a second-group point → `Ok(EncodeG2SuccessReturn { bytes })` holding `x.c1`, `x.c0`, `y.c1`, `y.c0`, each 32 bytes big-endian, in this concrete's `Bn254ArkworksEncodedG2`
      * `[✅]`   Collaborators: arkworks' coordinate encoding, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_encode_g2_payload`, and `build_bn254_arkworks_g2`
      * `[✅]`   Arrange: `build_encode_g2_payload` with the point override `build_bn254_arkworks_g2` at its default generator
      * `[✅]`   Act: `encode_g2(EncodeG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.bytes` binds to a `Bn254ArkworksEncodedG2` typed binding; its bytes equal `vector_bytes(G2_GENERATOR_HEX)` as a byte slice
    * `[✅]`   `encode_g2_writes_the_identity_as_zero_bytes`
      * `[✅]`   Contract: the second group's identity → `Ok(EncodeG2SuccessReturn { bytes })` holding 128 zero bytes
      * `[✅]`   Collaborators: arkworks, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_encode_g2_payload`, and `build_bn254_arkworks_g2`
      * `[✅]`   Arrange: `build_encode_g2_payload` with the point override `build_bn254_arkworks_g2` with the value `G2Affine::identity()`
      * `[✅]`   Act: `encode_g2(EncodeG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; the bytes of `success.bytes` equal `zero_bytes(128)` as a byte slice
    * `[✅]`   `encode_scalar_writes_the_largest_canonical_scalar`
      * `[✅]`   Contract: a scalar → `Ok(EncodeScalarSuccessReturn { bytes })` holding its 32 big-endian bytes in this concrete's `Bn254ArkworksEncodedScalar`, inside a `Secret`
      * `[✅]`   Collaborators: arkworks' integer encoding, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_encode_scalar_payload`, and `build_bn254_arkworks_scalar`
      * `[✅]`   Arrange: `build_encode_scalar_payload` with the scalar override `build_bn254_arkworks_scalar` with the value `-Fr::from(1u64)`
      * `[✅]`   Act: `encode_scalar(EncodeScalarParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.bytes.expose()` binds to a `Bn254ArkworksEncodedScalar` typed binding; its bytes equal `vector_bytes(GROUP_ORDER_MINUS_ONE_HEX)` as a byte slice
    * `[✅]`   `sample_from_uniform_bytes_rejects_a_wrong_length`
      * `[✅]`   Contract: `payload.uniform.expose().len() != 64` → `Err(SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual })`
      * `[✅]`   Collaborators: `Secret::expose`, real; fixtures `build_sample_uniform_scalar_payload` and `build_secret`
      * `[✅]`   Arrange: `build_sample_uniform_scalar_payload` with the uniform override `build_secret` holding `zero_bytes(63)`
      * `[✅]`   Act: `Bn254ArkworksScalar::sample_from_uniform_bytes(SampleUniformScalarParams, payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual: 63 })`, the whole expected error
    * `[✅]`   `sample_from_uniform_bytes_reduces_the_group_order_to_zero`
      * `[✅]`   Contract: a 64-byte uniform input → `Ok(SampleUniformScalarSuccessReturn { scalar })` holding the input reduced modulo the group order, inside a `Secret`
      * `[✅]`   Collaborators: `Secret::expose` and arkworks' modular reduction, real; fixtures `build_sample_uniform_scalar_payload` and `build_secret`
      * `[✅]`   Arrange: `build_sample_uniform_scalar_payload` with the uniform override `build_secret` holding `vector_bytes(UNIFORM_GROUP_ORDER_HEX)`, whose reduction is zero and so differs from the input
      * `[✅]`   Act: `Bn254ArkworksScalar::sample_from_uniform_bytes(SampleUniformScalarParams, payload)`
      * `[✅]`   Assert: the success arm is extracted with `expect`; `success.scalar.expose().value` equals `Fr::from(0u64)`
    * `[✅]`   `add_scalar_of_two_and_three_is_five`
      * `[✅]`   Contract: `payload.left` and `payload.right` → `Ok(AddScalarSuccessReturn { sum })` holding their sum modulo the group order
      * `[✅]`   Collaborators: arkworks' `Fr` addition, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_add_scalar_payload`, and `build_bn254_arkworks_scalar`
      * `[✅]`   Arrange: `build_add_scalar_payload` with the left override `build_bn254_arkworks_scalar` with the value `Fr::from(2u64)` and the right override with the value `Fr::from(3u64)`
      * `[✅]`   Act: `add_scalar(AddScalarParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `Fr::from(5u64)`
    * `[✅]`   `add_scalar_reduces_modulo_the_group_order`
      * `[✅]`   Contract: `payload.left` and `payload.right` → `Ok(AddScalarSuccessReturn { sum })` holding their sum modulo the group order
      * `[✅]`   Collaborators: arkworks' `Fr` addition, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_add_scalar_payload`, and `build_bn254_arkworks_scalar`
      * `[✅]`   Arrange: `build_add_scalar_payload` with the left override `build_bn254_arkworks_scalar` with the value `scalar_value(GROUP_ORDER_MINUS_ONE_HEX)` and the right override with the value `Fr::from(2u64)`, whose integer sum exceeds the group order
      * `[✅]`   Act: `add_scalar(AddScalarParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `Fr::from(1u64)`
    * `[✅]`   `mul_scalar_of_two_and_three_is_six`
      * `[✅]`   Contract: `payload.left` and `payload.right` → `Ok(MulScalarSuccessReturn { product })` holding their product modulo the group order
      * `[✅]`   Collaborators: arkworks' `Fr` multiplication, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_mul_scalar_payload`, and `build_bn254_arkworks_scalar`
      * `[✅]`   Arrange: `build_mul_scalar_payload` with the left override `build_bn254_arkworks_scalar` with the value `Fr::from(2u64)` and the right override with the value `Fr::from(3u64)`
      * `[✅]`   Act: `mul_scalar(MulScalarParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `Fr::from(6u64)`
    * `[✅]`   `mul_scalar_reduces_modulo_the_group_order`
      * `[✅]`   Contract: `payload.left` and `payload.right` → `Ok(MulScalarSuccessReturn { product })` holding their product modulo the group order
      * `[✅]`   Collaborators: arkworks' `Fr` multiplication, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_mul_scalar_payload`, and `build_bn254_arkworks_scalar`
      * `[✅]`   Arrange: `build_mul_scalar_payload` with the left override `build_bn254_arkworks_scalar` with the value `scalar_value(GROUP_ORDER_MINUS_ONE_HEX)` and the right override with the value `Fr::from(2u64)`, whose integer product exceeds the group order
      * `[✅]`   Act: `mul_scalar(MulScalarParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `scalar_value(GROUP_ORDER_MINUS_TWO_HEX)`
    * `[✅]`   `neg_scalar_of_one_is_the_group_order_minus_one`
      * `[✅]`   Contract: `payload.scalar` → `Ok(NegScalarSuccessReturn { negation })` holding the group order minus the scalar
      * `[✅]`   Collaborators: arkworks' `Fr` negation, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_neg_scalar_payload`, and `build_bn254_arkworks_scalar`
      * `[✅]`   Arrange: `build_neg_scalar_payload` with the scalar override `build_bn254_arkworks_scalar` with the value `Fr::from(1u64)`
      * `[✅]`   Act: `neg_scalar(NegScalarParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.negation.value` equals `scalar_value(GROUP_ORDER_MINUS_ONE_HEX)`
    * `[✅]`   `neg_scalar_of_zero_is_zero`
      * `[✅]`   Contract: `payload.scalar` of zero → `Ok(NegScalarSuccessReturn { negation })` holding zero
      * `[✅]`   Collaborators: arkworks' `Fr` negation, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_neg_scalar_payload`, and `build_bn254_arkworks_scalar`
      * `[✅]`   Arrange: `build_neg_scalar_payload` with the scalar override `build_bn254_arkworks_scalar` with the value `Fr::from(0u64)`
      * `[✅]`   Act: `neg_scalar(NegScalarParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.negation.value` equals `Fr::from(0u64)`
    * `[✅]`   `neg_g1_of_the_generator_is_the_negated_generator`
      * `[✅]`   Contract: `payload.point` → `Ok(NegG1SuccessReturn { negation })` holding `(x, p - y)` for a point `(x, y)`
      * `[✅]`   Collaborators: arkworks' affine negation, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_neg_g1_payload`, and `build_bn254_arkworks_g1`
      * `[✅]`   Arrange: `build_neg_g1_payload` with the point override `build_bn254_arkworks_g1` at its default generator
      * `[✅]`   Act: `neg_g1(NegG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.negation.value` equals `eip_196_negated_generator()`
    * `[✅]`   `neg_g1_of_the_identity_is_the_identity`
      * `[✅]`   Contract: `payload.point` of the identity → `Ok(NegG1SuccessReturn { negation })` holding the identity
      * `[✅]`   Collaborators: arkworks' affine negation, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_neg_g1_payload`, and `build_bn254_arkworks_g1`
      * `[✅]`   Arrange: `build_neg_g1_payload` with the point override `build_bn254_arkworks_g1` with the value `G1Affine::identity()`
      * `[✅]`   Act: `neg_g1(NegG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.negation.value` equals `G1Affine::identity()`
    * `[✅]`   `neg_g2_negates_each_y_coefficient_modulo_the_base_field`
      * `[✅]`   Contract: `payload.point` → `Ok(NegG2SuccessReturn { negation })` holding `(x, p - y)` for a point `(x, y)`, each `y` coefficient negated
      * `[✅]`   Collaborators: arkworks' affine negation, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_neg_g2_payload`, and `build_bn254_arkworks_g2`
      * `[✅]`   Arrange: `build_neg_g2_payload` with the point override `build_bn254_arkworks_g2` at its default generator
      * `[✅]`   Act: `neg_g2(NegG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; the negation's `xy()` has an `x` equal to the generator's `x` from `eip_197_generator().xy()`; its `y.c0` and `y.c1`, each as a `BigUint`, equal `base_field_modulus()` minus the generator's corresponding coefficient as a `BigUint`
    * `[✅]`   `neg_g2_of_the_identity_is_the_identity`
      * `[✅]`   Contract: `payload.point` of the identity → `Ok(NegG2SuccessReturn { negation })` holding the identity
      * `[✅]`   Collaborators: arkworks' affine negation, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_neg_g2_payload`, and `build_bn254_arkworks_g2`
      * `[✅]`   Arrange: `build_neg_g2_payload` with the point override `build_bn254_arkworks_g2` with the value `G2Affine::identity()`
      * `[✅]`   Act: `neg_g2(NegG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.negation.value` equals `G2Affine::identity()`
    * `[✅]`   `is_identity_g1_is_true_for_the_identity`
      * `[✅]`   Contract: `payload.point` → `Ok(IsIdentityG1SuccessReturn { is_identity })`, true exactly for the identity
      * `[✅]`   Collaborators: arkworks, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_is_identity_g1_payload`, and `build_bn254_arkworks_g1`
      * `[✅]`   Arrange: `build_is_identity_g1_payload` with the point override `build_bn254_arkworks_g1` with the value `G1Affine::identity()`
      * `[✅]`   Act: `is_identity_g1(IsIdentityG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_identity` is true
    * `[✅]`   `is_identity_g1_is_false_for_the_generator`
      * `[✅]`   Contract: `payload.point` → `Ok(IsIdentityG1SuccessReturn { is_identity })`, true exactly for the identity
      * `[✅]`   Collaborators: arkworks, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_is_identity_g1_payload`, and `build_bn254_arkworks_g1`
      * `[✅]`   Arrange: `build_is_identity_g1_payload` with the point override `build_bn254_arkworks_g1` at its default generator
      * `[✅]`   Act: `is_identity_g1(IsIdentityG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_identity` is false
    * `[✅]`   `is_identity_g2_is_true_for_the_identity`
      * `[✅]`   Contract: `payload.point` → `Ok(IsIdentityG2SuccessReturn { is_identity })`, true exactly for the identity
      * `[✅]`   Collaborators: arkworks, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_is_identity_g2_payload`, and `build_bn254_arkworks_g2`
      * `[✅]`   Arrange: `build_is_identity_g2_payload` with the point override `build_bn254_arkworks_g2` with the value `G2Affine::identity()`
      * `[✅]`   Act: `is_identity_g2(IsIdentityG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_identity` is true
    * `[✅]`   `is_identity_g2_is_false_for_the_generator`
      * `[✅]`   Contract: `payload.point` → `Ok(IsIdentityG2SuccessReturn { is_identity })`, true exactly for the identity
      * `[✅]`   Collaborators: arkworks, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_is_identity_g2_payload`, and `build_bn254_arkworks_g2`
      * `[✅]`   Arrange: `build_is_identity_g2_payload` with the point override `build_bn254_arkworks_g2` at its default generator
      * `[✅]`   Act: `is_identity_g2(IsIdentityG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_identity` is false
    * `[✅]`   `pairing_product_of_no_terms_is_the_target_group_identity`
      * `[✅]`   Contract: an empty `payload.terms` → `Ok(PairingProductSuccessReturn { product })` holding the target group's identity
      * `[✅]`   Collaborators: arkworks' multi-pairing and target-group exponentiation, run for real as the vendor; fixtures `build_bn254_arkworks_pairing` and `build_pairing_product_payload`
      * `[✅]`   Arrange: `build_pairing_product_payload` at its default of no terms
      * `[✅]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` is the identity under `is_zero()`
    * `[✅]`   `pairing_product_of_the_generators_is_not_the_target_group_identity`
      * `[✅]`   Contract: a pairing of the generators → `Ok(PairingProductSuccessReturn { product })` holding a value other than the target group's identity
      * `[✅]`   Collaborators: arkworks' multi-pairing and target-group exponentiation, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_pairing_product_payload`, and `build_pairing_product_term`
      * `[✅]`   Arrange: `build_pairing_product_payload` with the terms override holding one `build_pairing_product_term` at its default generators
      * `[✅]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` is not the identity under `is_zero()`
    * `[✅]`   `pairing_product_is_bilinear`
      * `[✅]`   Contract: a term whose first-group element is twice the generator → `Ok(PairingProductSuccessReturn { product })` holding the generators' pairing raised to the power two
      * `[✅]`   Collaborators: arkworks' multi-pairing and target-group exponentiation, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_pairing_product_payload`, `build_pairing_product_term`, and `build_bn254_arkworks_g1`; the expectation is `definition_value_power`, the identifier's definition raised to a power in the fixtures, independent of the correction `pairing_product` applies
      * `[✅]`   Arrange: `build_pairing_product_payload` with the terms override holding one `build_pairing_product_term` with the first-group override `build_bn254_arkworks_g1` with the value `eip_196_doubled_generator()` over the second-group generator
      * `[✅]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value.0` equals `definition_value_power(eip_196_generator(), eip_197_generator(), 2)`
    * `[✅]`   `pairing_product_multiplies_its_terms`
      * `[✅]`   Contract: `payload.terms` → `Ok(PairingProductSuccessReturn { product })` holding the product of each term's pairing
      * `[✅]`   Collaborators: arkworks' multi-pairing and target-group exponentiation, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_pairing_product_payload`, and `build_pairing_product_term`; the expectation is `definition_value_power`
      * `[✅]`   Arrange: `build_pairing_product_payload` with the terms override holding two `build_pairing_product_term` values at their default generators, so a product over one term differs from the expected square
      * `[✅]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value.0` equals `definition_value_power(eip_196_generator(), eip_197_generator(), 2)`
    * `[✅]`   `pairing_product_of_a_pairing_and_its_first_group_negation_is_the_target_group_identity`
      * `[✅]`   Contract: `payload.terms` whose pairings multiply to the target group's identity → `Ok(PairingProductSuccessReturn { product })` holding the identity
      * `[✅]`   Collaborators: arkworks' multi-pairing and target-group exponentiation, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_pairing_product_payload`, `build_pairing_product_term`, and `build_bn254_arkworks_g1`
      * `[✅]`   Arrange: `build_pairing_product_payload` with the terms override holding two `build_pairing_product_term` values: the generators, and the first-group override `build_bn254_arkworks_g1` with the value `eip_196_negated_generator()` over the second-group generator
      * `[✅]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` is the identity under `is_zero()`
    * `[✅]`   `pairing_product_of_the_generators_equals_the_definition`
      * `[✅]`   Contract: a pairing of the generators → `Ok(PairingProductSuccessReturn { product })` holding the Miller loop's value raised to the exact exponent `(p^12 - 1) / r`, the value `TargetGroupEncodingIdentifier::Bn254V1` defines
      * `[✅]`   Collaborators: arkworks' multi-pairing and target-group exponentiation, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_pairing_product_payload`, and `build_pairing_product_term`; the expectation is `definition_value`, the Miller loop over the generators raised to `definition_exponent()` built by `num-bigint`, independent of the correction `pairing_product` applies
      * `[✅]`   Arrange: `build_pairing_product_payload` with the terms override holding one `build_pairing_product_term` at its default generators
      * `[✅]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value.0` equals `definition_value(eip_196_generator(), eip_197_generator())`
    * `[✅]`   `encode_gt_writes_the_target_group_identity_with_c0_c0_c0_first`
      * `[✅]`   Contract: a target-group value → `Ok(EncodeGtSuccessReturn { bytes })` holding its twelve coefficients in tower order, `c0.c0.c0` first, each 32 bytes big-endian, inside a `Secret`
      * `[✅]`   Collaborators: arkworks' integer encoding, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_encode_gt_payload`, and `build_bn254_arkworks_gt`; the expectation is `gt_identity_encoding()`
      * `[✅]`   Arrange: `build_encode_gt_payload` with the value override `build_bn254_arkworks_gt` with the value `PairingOutput(Fq12::ONE)`, the target group's identity
      * `[✅]`   Act: `encode_gt(EncodeGtParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.bytes.expose()` binds to a `Bn254ArkworksEncodedGt` typed binding; its bytes equal `gt_identity_encoding()` as a byte slice
    * `[✅]`   `scalar_field_order_is_the_group_order`
      * `[✅]`   Contract: any call → `Ok(ScalarFieldOrderSuccessReturn { bytes })` holding the group order's 32 big-endian bytes
      * `[✅]`   Collaborators: arkworks' scalar field modulus, run for real as the vendor; fixture `build_bn254_arkworks_pairing`; `ScalarFieldOrderParams` and `ScalarFieldOrderPayload` by their production values
      * `[✅]`   Arrange: `build_bn254_arkworks_pairing()`
      * `[✅]`   Act: `scalar_field_order(ScalarFieldOrderParams, ScalarFieldOrderPayload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.bytes` equals `vector_bytes(GROUP_ORDER_HEX)`
    * `[✅]`   `g1_outside_subgroup_encoding_is_absent_where_the_cofactor_is_one`
      * `[✅]`   Contract: any call → `Ok(G1OutsideSubgroupEncodingSuccessReturn { bytes: None })`
      * `[✅]`   Collaborators: none called; fixture `build_bn254_arkworks_pairing`; `G1OutsideSubgroupEncodingParams` and `G1OutsideSubgroupEncodingPayload` by their production values
      * `[✅]`   Arrange: `build_bn254_arkworks_pairing()`
      * `[✅]`   Act: `g1_outside_subgroup_encoding(G1OutsideSubgroupEncodingParams, G1OutsideSubgroupEncodingPayload)`
      * `[✅]`   Assert: the success arm is extracted with `expect`; `success.bytes.is_none()` is true
    * `[✅]`   `g2_outside_subgroup_encoding_encodes_an_on_curve_point_outside_the_subgroup`
      * `[✅]`   Contract: any call → `Ok(G2OutsideSubgroupEncodingSuccessReturn { bytes })` holding the precompile encoding of an on-curve point outside the prime-order subgroup, in this concrete's `Bn254ArkworksEncodedG2`
      * `[✅]`   Collaborators: arkworks' curve and subgroup checks, run for real as the vendor; fixture `build_bn254_arkworks_pairing`; `G2OutsideSubgroupEncodingParams` and `G2OutsideSubgroupEncodingPayload` by their production values; the point read back by `g2_point_from_encoding`
      * `[✅]`   Arrange: `build_bn254_arkworks_pairing()`
      * `[✅]`   Act: `g2_outside_subgroup_encoding(G2OutsideSubgroupEncodingParams, G2OutsideSubgroupEncodingPayload)`
      * `[✅]`   Assert: the success arm is extracted with `expect`; `success.bytes` binds to a `Bn254ArkworksEncodedG2` typed binding; the point `g2_point_from_encoding` reads from its bytes satisfies `is_on_curve()` and does not satisfy `is_in_correct_subgroup_assuming_on_curve()`
    * `[✅]`   `g2_outside_subgroup_encoding_has_a_real_first_coordinate_and_the_lesser_root`
      * `[✅]`   Contract: any call → the encoded point's first coordinate is `(c0, 0)` with the least `c0` of an on-curve point outside the subgroup, and its `y` is the lesser of the two roots, compared by `y.c1` and then `y.c0`
      * `[✅]`   Collaborators: arkworks' curve and subgroup checks, run for real as the vendor; fixture `build_bn254_arkworks_pairing`; `G2OutsideSubgroupEncodingParams` and `G2OutsideSubgroupEncodingPayload` by their production values; the expectations are `g2_outside_subgroup_bytes()`, the ascending search in the fixtures, and `base_field_modulus()`
      * `[✅]`   Arrange: `build_bn254_arkworks_pairing()`
      * `[✅]`   Act: `g2_outside_subgroup_encoding(G2OutsideSubgroupEncodingParams, G2OutsideSubgroupEncodingPayload)`
      * `[✅]`   Assert: the success arm is extracted with `expect`; the encoding's `x.c1` quarter, its first 32 bytes, is all zero; its `x.c0` quarter, its second 32 bytes, equals the `x.c0` quarter of `g2_outside_subgroup_bytes()`; the pair `y.c1`, `y.c0` read from its last 64 bytes as `BigUint` values is not greater than the pair of their negations, each `base_field_modulus()` minus the coefficient, reduced to zero where the coefficient is zero
    * `[✅]`   `encode_gt_writes_the_twelve_coefficients_in_tower_order`
      * `[✅]`   Contract: a target-group value → `Ok(EncodeGtSuccessReturn { bytes })` holding its twelve coefficients in tower order, `c0.c0.c0` through `c1.c2.c1`, each 32 bytes big-endian, inside a `Secret`
      * `[✅]`   Collaborators: arkworks' integer encoding, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_encode_gt_payload`, and `build_bn254_arkworks_gt`; the expectation is `sequential_gt_encoding()`
      * `[✅]`   Arrange: `build_encode_gt_payload` with the value override `build_bn254_arkworks_gt` with the value `gt_with_sequential_coefficients()`, whose twelve coefficients are distinct, so any exchange of two positions changes the bytes
      * `[✅]`   Act: `encode_gt(EncodeGtParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; the bytes of `success.bytes.expose()` equal `sequential_gt_encoding()` as a byte slice
    * `[✅]`   `bn254_arkworks_g1_zeroize_clears_its_value`
      * `[✅]`   Contract: `Zeroize::zeroize` on a living first-group element → its `value` is the point `new_unchecked(0, 0)`
      * `[✅]`   Collaborators: arkworks' affine zeroization, run for real as the vendor; fixture `build_bn254_arkworks_g1`
      * `[✅]`   Arrange: `build_bn254_arkworks_g1` at its default generator, which differs from the expected point
      * `[✅]`   Act: `g1.zeroize()`
      * `[✅]`   Assert: `g1.value` equals `G1Affine::new_unchecked(Fq::from(0u64), Fq::from(0u64))`
    * `[✅]`   `bn254_arkworks_g2_zeroize_clears_its_value`
      * `[✅]`   Contract: `Zeroize::zeroize` on a living second-group element → its `value` is the point `new_unchecked(0, 0)`
      * `[✅]`   Collaborators: arkworks' affine zeroization, run for real as the vendor; fixture `build_bn254_arkworks_g2`
      * `[✅]`   Arrange: `build_bn254_arkworks_g2` at its default generator, which differs from the expected point
      * `[✅]`   Act: `g2.zeroize()`
      * `[✅]`   Assert: `g2.value` equals `G2Affine::new_unchecked(Fq2::new(Fq::from(0u64), Fq::from(0u64)), Fq2::new(Fq::from(0u64), Fq::from(0u64)))`
    * `[✅]`   `bn254_arkworks_gt_zeroize_clears_its_value`
      * `[✅]`   Contract: `Zeroize::zeroize` on a living target-group value → every coefficient of its `Fq12` is zero
      * `[✅]`   Collaborators: arkworks' `PairingOutput` zeroization, run for real as the vendor; fixture `build_bn254_arkworks_gt`
      * `[✅]`   Arrange: `build_bn254_arkworks_gt` at its default pairing of the generators, whose `Fq12` is nonzero
      * `[✅]`   Act: `gt.zeroize()`
      * `[✅]`   Assert: `gt.value.0.is_zero()` is true
    * `[✅]`   `bn254_arkworks_encoded_scalar_zeroize_clears_its_bytes`
      * `[✅]`   Contract: `Zeroize::zeroize` on a living encoded scalar → every byte is zero
      * `[✅]`   Collaborators: none; fixture `build_bn254_arkworks_encoded_scalar`
      * `[✅]`   Arrange: `build_bn254_arkworks_encoded_scalar` at its nonzero default
      * `[✅]`   Act: `encoded.zeroize()`
      * `[✅]`   Assert: the bytes of `encoded` equal `zero_bytes(32)` as a byte slice
    * `[✅]`   `bn254_arkworks_encoded_gt_zeroize_clears_its_bytes`
      * `[✅]`   Contract: `Zeroize::zeroize` on a living encoded target-group value → every byte is zero
      * `[✅]`   Collaborators: none; fixture `build_bn254_arkworks_encoded_gt`
      * `[✅]`   Arrange: `build_bn254_arkworks_encoded_gt` at its nonzero default
      * `[✅]`   Act: `encoded.zeroize()`
      * `[✅]`   Assert: the bytes of `encoded` equal `zero_bytes(384)` as a byte slice

  * `[✅]`   `construction`
    * `[✅]`   `Bn254ArkworksPairing::try_new` is the concrete's only producer, computing its one field, and its only caller is the pairing factory, which obtains the declaration through `pairing/lookup_pairing_declaration` and completes `pairing/admit_pairing_composition` before calling it and before its correction computation
    * `[✅]`   A group element or scalar is produced only by the adapter's generators, arithmetic, and decoders, or by the scalar's sampling bound; a target-group value only by `pairing_product`; an encoded value only by the adapter's encoders and reference methods; no consumer constructs one from library values

  * `[✅]`   `adapters/pairing/src/bn254_arkworks/mod.rs`
    * `[✅]`   Module declarations: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, `pub(crate) mod provides;`, and `#[cfg(test)] mod test;`
    * `[✅]`   Imports: from `crate::factory::provides` every params, payload, return, success-return, and error type of the three traits and the sampling bound, `IPairingAdapter`, `IPairingArithmetic`, `IPairingReference`, and `ISampleUniformScalar`; `ark_bn254::{Bn254, Fq, Fq2, Fr, G1Affine, G1Projective, G2Affine, G2Projective}`; `ark_ec::{AffineRepr, CurveGroup, VariableBaseMSM, pairing::Pairing}`; `ark_ff::{BigInt, BigInteger, Field, PrimeField, Zero}`; `domain::{Secret, SecretConstructorParams}`; from `interface` every type it declares; `zeroize::{Zeroize, ZeroizeOnDrop}`
    * `[✅]`   `const BN254_SEED: u64 = 4_965_661_367_192_848_881;`, private, with the doc comment "The seed `x` of the BN254 curve EIP-197 fixes, from which the curve's `p` and `r` derive."
    * `[✅]`   `impl Bn254ArkworksPairing` holding `pub fn try_new(params: Bn254ArkworksPairingConstructorParams) -> Bn254ArkworksPairingTryNewReturn` realizing the interaction spec's construct branch; `DECLARATION` lives in `interface.rs`, the interface owning the real declaration
    * `[✅]`   `impl IPairingAdapter for Bn254ArkworksPairing` with `type Scalar = Bn254ArkworksScalar;`, `type G1 = Bn254ArkworksG1;`, `type G2 = Bn254ArkworksG2;`, `type EncodedG1 = Bn254ArkworksEncodedG1;`, `type EncodedG2 = Bn254ArkworksEncodedG2;`, and `type EncodedScalar = Bn254ArkworksEncodedScalar;`, every method realizing its branches in the interaction spec
    * `[✅]`   `impl IPairingArithmetic for Bn254ArkworksPairing` with `type Gt = Bn254ArkworksGt;` and `type EncodedGt = Bn254ArkworksEncodedGt;`, every method realizing its branches in the interaction spec, `pairing_product` multiplying the `PairingOutput` by `self.reduced_pairing_correction`
    * `[✅]`   `impl IPairingReference for Bn254ArkworksPairing`, every method realizing its branches in the interaction spec, the search as `(1u64..).find_map(…)` with the `SearchExhausted` return by `let … else`
    * `[✅]`   `impl AsRef<[u8]>` for `Bn254ArkworksEncodedG1`, `Bn254ArkworksEncodedG2`, `Bn254ArkworksEncodedScalar`, and `Bn254ArkworksEncodedGt`, each returning `&self.bytes`; `impl Zeroize` for `Bn254ArkworksEncodedScalar` and `Bn254ArkworksEncodedGt`, each calling `self.bytes.zeroize()`; no implementation returns a mutable reference to the bytes
    * `[✅]`   `impl Zeroize` and `impl Drop` for `Bn254ArkworksScalar`, `Bn254ArkworksG1`, `Bn254ArkworksG2`, and `Bn254ArkworksGt`, each calling `self.value.zeroize()`; `impl ZeroizeOnDrop for Bn254ArkworksScalar {}`, marking the drop behavior the sampling bound requires
    * `[✅]`   `impl ISampleUniformScalar for Bn254ArkworksScalar` with `const UNIFORM_BYTES_LENGTH: usize = 64;` and `sample_from_uniform_bytes` realizing its branches in the interaction spec, the scalar moved into a `Secret` by `let Ok(scalar) = Secret::try_new(SecretConstructorParams { value });`
    * `[✅]`   No `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`; the correction is computed in `try_new` and read from no configuration; neither the order nor a point is written as a literal

  * `[✅]`   `adapters/pairing/src/bn254_arkworks/provides.rs`
    * `[✅]`   `pub(crate) use super::interface::*;` and `#[cfg(test)] pub(crate) use super::mock::*;`, exposing every item of the concrete's interface to the crate and, in the crate's own test build, the concrete's builders, fixtures, and helpers, and nothing beyond them

  * `[✅]`   `directionality`
    * `[✅]`   `bn254_arkworks` depends on the `factory` module's surface through `crate::factory::provides`, on `domain`, on `zeroize`, and on the arkworks crates; the `factory` module depends on `domain` and `zeroize` and on no concrete; `IPairingArithmetic` and `IPairingReference` each depend on `IPairingAdapter` within the `factory` module; `pairing/lookup_pairing_declaration` reaches each existing member's interface-held declaration through the family's private path, and the factory receives declarations through that lookup and completes composition admission before construction; among repository crates the crate depends on `crates/domain` alone at runtime and on `adapters/random` for tests only, the dependency map's edge
    * `[✅]`   `pairing/factory` constructs each existing concrete, the family form's recorded cycle, admits by precompile encoding and identifier, and requires the arithmetic and reference traits of the concrete a consumer receives; each later concrete adds its own declaration, identifier variant, declared-set entry, and family-function branch in its own increment without changing consumers outside the family

  * `[✅]`   `requirements`
    * `[✅]`   `adapters/pairing/Cargo.toml` carries exactly the tables and keys stated above, and no `ark-` crate is named in the crate outside the arkworks concretes
    * `[✅]`   The family's encoders and this concrete return distinct owned fixed-width types for the first group, the second group, the scalar, and the target group; scalar and target-group bytes are held inside `Secret`, and every encoded type exposes only an immutable wire-byte view
    * `[✅]`   `SampleUniformScalarErrorReturn`, `DecodeG1ErrorReturn`, `DecodeG2ErrorReturn`, `DecodeScalarErrorReturn`, `G1OutsideSubgroupEncodingErrorReturn`, and `G2OutsideSubgroupEncodingErrorReturn` derive `Debug`, `PartialEq`, and `Eq`, and every refusal the concrete's tests assert is asserted by `assert_eq!` against the whole expected error
    * `[✅]`   `cargo check --all-targets --all-features`, `cargo fmt --check`, and `cargo deny check` complete without error; `cargo clippy --all-targets --all-features` reports nothing beyond the library target's unused-item warnings for the `bn254_arkworks` concrete, which `pairing/factory` constructs
    * `[✅]`   The `ZeroizeOnDrop` compile-time assertion compiles and `bn254_arkworks_scalar_zeroize_clears_its_value` passes (CR-07), and `encode_g1_writes_the_eip_196_generator`, `encode_g2_writes_the_eip_197_generator`, and `encode_scalar_writes_the_largest_canonical_scalar` bind their results as this concrete's own encoded types and pass (the typed encodings)
    * `[✅]`   The `g1_generator`, `g2_generator`, `encode_g1`, `encode_g2`, `decode_g1`, `decode_g2`, and `decode_scalar` blocks, the `add_g1`, `add_g2`, `mul_g1`, `mul_g2`, `msm_g1`, and `msm_g2` blocks, the `pairing_product_is_one` blocks, and the `sample_from_uniform_bytes` blocks pass, and the declaration compile-time assertion compiles (CR-10, the generic interface on BN254 over arkworks; CR-05 for the sampled scalar)
    * `[✅]`   `pairing_concretes_declares_only_real_members`, `target_group_encoding_identifiers_declares_only_real_values`, `every_declared_identifier_spells_a_distinct_name`, and `no_value_identifier_declares_a_mock_only_variant` pass, proving the declaration carries `PairingConcrete` rather than text, `PAIRING_CONCRETES` and `TARGET_GROUP_ENCODING_IDENTIFIERS` contain only real currently provided values with distinct spellings, and no value identifier has a mock-only variant
    * `[✅]`   The real selection carries no settings and this concrete carries no settings module, settings node, or settings table, and `build_pairing_concrete` returns the real `Bn254Arkworks` selection with no configuration
    * `[✅]`   The factory obtains the declaration through `pairing/lookup_pairing_declaration` and completes `pairing/admit_pairing_composition` before constructing the concrete
    * `[✅]`   `add_scalar_of_two_and_three_is_five`, `add_scalar_reduces_modulo_the_group_order`, `mul_scalar_of_two_and_three_is_six`, `mul_scalar_reduces_modulo_the_group_order`, `neg_scalar_of_one_is_the_group_order_minus_one`, and `neg_scalar_of_zero_is_zero` pass (CR-09, the scalar arithmetic the delivery proof's responses use)
    * `[✅]`   The `neg_g1`, `neg_g2`, `is_identity_g1`, and `is_identity_g2` blocks pass (CR-04 envelope decryption and identity-key rejection; CR-08 trivial identity-element refusal)
    * `[✅]`   `pairing_product_of_no_terms_is_the_target_group_identity`, `encode_gt_writes_the_target_group_identity_with_c0_c0_c0_first`, `pairing_product_of_the_generators_is_not_the_target_group_identity`, `pairing_product_is_bilinear`, `pairing_product_multiplies_its_terms`, and `pairing_product_of_a_pairing_and_its_first_group_negation_is_the_target_group_identity` pass (CR-08, the encapsulated value and the validity, well-formedness, and decapsulation equations)
    * `[✅]`   `pairing_product_of_the_generators_equals_the_definition` and `reduced_pairing_correction_inverts_the_library_multiple` pass (CR-10, the target-group value fixed by the identifier's definition, executed in the test)
    * `[✅]`   `scalar_field_order_is_the_group_order`, `g1_outside_subgroup_encoding_is_absent_where_the_cofactor_is_one`, `g2_outside_subgroup_encoding_encodes_an_on_curve_point_outside_the_subgroup`, `decode_g2_rejects_a_point_outside_the_subgroup`, and `g2_outside_subgroup_encoding_has_a_real_first_coordinate_and_the_lesser_root` pass (CR-10, the subgroup-rejection input a contract's own check is proven against; CR-11, the modulus a contract reduces every hash-to-scalar digest by)
    * `[✅]`   Code outside `adapters/pairing` naming `Bn254ArkworksPairing` or anything under `bn254_arkworks` fails to compile; the crate's public surface is the `factory` module's `provides`

* `[✅]`   `pairing/bn254_halo2curves` **BN254 pairing concrete on halo2curves implementing the pairing family's generic interface, arithmetic trait, and reference trait over the EIP-196 and EIP-197 encodings and the `Bn254V1` target-group value; the increment adding the `Bn254Halo2curves` member variant, its spelling, its declared-set entry, and its declared-selection builder arm, a further concrete beneath the pairing factory**

  * `[✅]`   `objective`
    * `[✅]`   Problem: the harness benchmark compares pairing libraries per curve through the factory, so BN254 needs a concrete over another library that satisfies the family's generic interface exactly as the arkworks concrete does (CR-10)
    * `[✅]`   Problem: the KEM, the envelope, and the delivery proof consume the pairing family's arithmetic trait through whichever concrete the factory resolves, and the bytes the KEM hands the KDF are the identifier's target-group value, so this concrete implements the trait with target-group values equal to the identifier's definition, and therefore to the arkworks concrete's, under the one identifier both declare; `halo2curves`' BN256 final exponentiation computes the exact exponent `(p^12 - 1) / r`, the Devegili–Scott–Dahab chain in its source, so the library's reduced pairing is the identifier's value and the concrete applies no correction (CR-04; CR-07; CR-08; CR-09; CR-10; `docs/research/cryptography.md`'s pairing adapter capability declaration)
    * `[✅]`   Problem: every concrete a consumer receives returns the group order and the family's outside-the-subgroup encodings, and the libraries of one curve return the same bytes, so this concrete implements the family's reference trait under the family's rule from its own library (CR-10 on chain; CR-11; the dependency map's `pairing/bn254_arkworks` row; the Pairing adapters and key derivation milestone's exit)
    * `[✅]`   Functional: the concrete implements `IPairingAdapter` over `halo2curves`' BN256, which is BN254, with its own scalar, group-element, and encoded types over the library's elements as the associated types
    * `[✅]`   Functional: it encodes and decodes a first-group point as EIP-196's 64 bytes and a second-group point as EIP-197's 128 bytes, each coordinate a 32-byte big-endian integer, the second-group coordinates imaginary part first, the point at infinity as all zero bytes, and a scalar as 32 big-endian bytes, rejecting a wrong length, a non-canonical field element, a point off the curve, and a point outside the prime-order subgroup
    * `[✅]`   Functional: its scalar type implements the family's sampling bound, reading the 64 uniform bytes as one big-endian integer reduced modulo the group order, so the same input bytes sample the same scalar under either BN254 concrete
    * `[✅]`   Functional: its declaration carries `PairingConcrete::Bn254Halo2curves`, the same real Bn254-provided values as the arkworks member — the BN254 curve, first-group-only arithmetic at the verifier, the EIP-196 and EIP-197 precompile encoding, and `TargetGroupEncodingIdentifier::Bn254V1` — its adapter version, and the interface version it implements, the typed member identifier with no duplicate text name, so no new value-identifier variant is needed; read through `pairing/lookup_pairing_declaration` without an instance, never from a constructed adapter
    * `[✅]`   Functional: `Bn254Halo2curvesPairing` implements `IPairingArithmetic` with every method's behavior as `pairing/bn254_arkworks` states it for the trait, its `pairing_product` returning the library's reduced pairing, which is the identifier's exact value
    * `[✅]`   Functional: the target-group encoding reads the `Fq12` through `Gt::inner` and writes its twelve coefficients in the tower order `c0.c0.c0`, `c0.c0.c1`, `c0.c1.c0`, `c0.c1.c1`, `c0.c2.c0`, `c0.c2.c1`, `c1.c0.c0`, `c1.c0.c1`, `c1.c1.c0`, `c1.c1.c1`, `c1.c2.c0`, `c1.c2.c1`, each 32 bytes big-endian, over halo2curves' BN256 tower `Fq2 = Fq[u] / (u^2 + 1)`, `Fq6 = Fq2[v] / (v^3 - (u + 9))`, `Fq12 = Fq6[w] / (w^2 - v)`
    * `[✅]`   Functional: the concrete's test executes the identifier's definition, the library's Miller loop over the two generators raised to the integer `(p^12 - 1) / r` built by `num-bigint` from the library's moduli and serialized in the tower order, and requires `pairing_product` then `encode_gt` over the generators to yield the same bytes
    * `[✅]`   Functional: `Bn254Halo2curvesScalar`, `Bn254Halo2curvesG1`, `Bn254Halo2curvesG2`, and `Bn254Halo2curvesGt` clear their value through `Zeroize` and on drop
    * `[✅]`   Functional: the concrete returns BN254's group order as 32 big-endian bytes, no first-group point, and the second group's point under the family's rule, encoded as `encode_g2` encodes any point, an encoding its own `decode_g2` refuses as outside the subgroup
    * `[✅]`   Non-functional: `halo2curves` is named only inside the halo2curves concretes; no order and no point is written as a literal outside a test

  * `[✅]`   `role`
    * `[✅]`   Adapter: a further concrete of the pairing family, implementing the generic interface, the sampling bound, the arithmetic trait, and the reference trait `pairing/bn254_arkworks` authors in the `factory` module, declaring the identifier it authors, and the increment adding the `Bn254Halo2curves` member variant, its `bn254_halo2curves` spelling, its `PAIRING_CONCRETES` entry, and its `build_pairing_concrete` arm
    * `[✅]`   Contributes its module line to `lib.rs` and its dependency line to the crate manifest; increments `factory/interface.rs` with the member `PairingConcrete::Bn254Halo2curves`, its `name()` spelling `bn254_halo2curves`, and its entry in `PAIRING_CONCRETES` beside `Bn254Arkworks`, the set staying real-only and ordered by the existing nodes, and increments `factory/mock.rs` with the `build_pairing_concrete` arm returning this real selection without configuration; `PairingCurve`, `VerifierGroupArithmetic`, `PrecompileEncoding`, `TargetGroupEncodingIdentifier`, and `TARGET_GROUP_ENCODING_IDENTIFIERS` are unchanged, this member providing no new value
    * `[✅]`   Does not select between the BN254 concretes; `pairing/factory` constructs a concrete by the composition's request and `harness-crypto/benchmark` records each concrete's measurements
    * `[✅]`   Does not name, import, or compare against another concrete; `pairing/factory`'s integration test proves agreement with the arkworks concrete
    * `[✅]`   Does not carry a correction; the library's reduced pairing is the identifier's value
    * `[✅]`   Does not carry a settings module, settings node, or settings table; the concrete has no settable value, so the real selection carries no settings
    * `[✅]`   Does not carry a commit

  * `[✅]`   `module`
    * `[✅]`   Bounded context: the private `bn254_halo2curves` module of `adapters/pairing`, holding the adapter over `halo2curves`' BN256, its scalar, group-element, target-group, and encoded types over the library's elements, its constructor params, its implementations of the three traits, its interface-held declaration constant carrying `PairingConcrete::Bn254Halo2curves`, its definition test, its clearing, and the builder defaults for its owned types
    * `[✅]`   The node's files: the `adapters/pairing/Cargo.toml` dependency line, the `adapters/pairing/src/lib.rs` module line, the `factory` module's `interface.rs` and `mock.rs`, and the `bn254_halo2curves` module's `interface.rs`, `interaction.spec.md`, `mock.rs`, `test.rs`, `mod.rs`, and `provides.rs`
    * `[✅]`   Outside: the family's traits and declaration, every other concrete, and the family's declaration lookup, selection admission, composition admission, and factory construction

  * `[✅]`   `deps`
    * `[✅]`   The `factory` module's surface, through `crate::factory::provides`: `IPairingAdapter`, `IPairingArithmetic`, `IPairingReference`, `ISampleUniformScalar`, `PairingDeclaration` with its typed `concrete` field and its real value enums `PairingCurve`, `VerifierGroupArithmetic`, `PrecompileEncoding`, and `TargetGroupEncodingIdentifier`, `PairingConcrete` with `Bn254Halo2curves` and its `name()` mapping, `PAIRING_CONCRETES` holding both real BN254 members, `PAIRING_INTERFACE_VERSION`, every params, payload, return, success-return, error type, and builder of the three traits and the sampling bound, and, under the `mocks` feature, `build_pairing_concrete` with this member's no-configuration arm
    * `[✅]`   `domain`, runtime: `Secret` and `SecretConstructorParams`; `build_secret` and `SecretConstructorParamsOverrides` through the `mocks` feature
    * `[✅]`   `zeroize` `1.9.0`, runtime: the `Zeroize` and `ZeroizeOnDrop` traits and the zeroization of the local 64-byte input copy
    * `[✅]`   `halo2curves` `0.10.0`, external crate, MIT/Apache-2.0, runtime, default features, resolved to the `gt-accessor` overlay, which supplies `Gt::inner(&self) -> &Fq12`; named only in the halo2curves concretes; supplies the curve, the pairing, the group arithmetic, the field arithmetic, and multi-scalar multiplication, and re-exports the `ff`, `group`, and `pairing` traits its types implement, so no separate trait crate is pinned
    * `[✅]`   `hex` `0.4.3` and `num-bigint` `0.4.8`, the crate's dev-dependencies, for the test vectors and the integer exponent in `mock.rs`'s test-only fixtures and the root comparison in `test.rs`
    * `[✅]`   `core::convert::Infallible`, standard library, the constructor's error arm; `core::hint::black_box`, standard library, which keeps each clearing from being removed as a dead store, since `halo2curves`' fields, points, and target-group values implement no `Zeroize`; `core::iter::successors`, standard library, the ascending search
    * `[✅]`   Reverse dependency: `pairing/factory`

  * `[✅]`   `context_slice`
    * `[✅]`   From `halo2curves::bn256`: `Bn256`, `Fq`, `Fq2` with `Fq2::new(c0, c1)` and the accessors `c0()` and `c1()`, `Fq12` with `c0()` and `c1()` and the degree-six accessors `c0()`, `c1()`, and `c2()`, `Fr`, `G1`, `G1Affine`, `G2`, `G2Affine`, and `Gt` with `Gt::identity()`, `Gt::inner()`, and `PartialEq`
    * `[✅]`   `Fr::from(u64)` and `Fq::from(u64)`, the scalars and coordinates the test fixtures and expectations name
    * `[✅]`   From `halo2curves::ff`: `Fr`'s `+`, `*`, and unary `-` modulo the group order; `Field` for `ZERO`, `ONE`, `is_zero()`, `square()`, `sqrt()` returning a `CtOption`, and `pow_vartime(&self, exp: impl AsRef<[u64]>)`, exponentiation by little-endian limbs; unary `-` on `Fq` and `Fq2` and binary `+` on `Fq2`; `PrimeField` for `from_repr(repr) -> CtOption<Self>`, which reads 32 little-endian bytes and is none at or above the modulus, `to_repr()`, 32 little-endian bytes read by `as_ref()`, and `MODULUS`, the `&'static str` hex modulus with the `0x` prefix, on `Fq` and on `Fr`; `FromUniformBytes::<64>::from_uniform_bytes(&[u8; 64])`, a little-endian wide reduction
    * `[✅]`   From `halo2curves::group`: `Curve::to_affine`, `Group::is_identity`, `prime::PrimeCurveAffine` for `generator()`, `identity()`, `is_identity()` returning a `Choice`, and `to_curve()`, `cofactor::CofactorGroup::is_torsion_free`, and the projective `+` and `* Fr`; unary `-` on `G1Affine` and `G2Affine`
    * `[✅]`   From `halo2curves`: `CurveAffine` for `from_xy(x, y) -> CtOption<Self>`, which is none off the curve, `coordinates() -> CtOption<Coordinates<Self>>`, and `b()`; `Coordinates` for `x()` and `y()`; `msm::msm_best(coeffs: &[C::Scalar], bases: &[C]) -> C::Curve`
    * `[✅]`   From `halo2curves::pairing`: `MultiMillerLoop::multi_miller_loop(&[(&G1Affine, &G2Affine)])` on `Bn256`, returning the unreduced `Fq12`, `MillerLoopResult::final_exponentiation` returning `Gt`, whose `is_identity()` is the check, and `Engine::pairing(&G1Affine, &G2Affine)` for the builder default
    * `[✅]`   A `Choice` becomes a `bool` by `bool::from`, and a `CtOption` becomes an `Option` by `Option::from`
    * `[✅]`   From `num-bigint`, in `mock.rs`'s test-only fixtures and in `test.rs`: `BigUint::parse_bytes(&[u8], u32) -> Option<BigUint>`, `BigUint::from(u32)`, `BigUint::from_bytes_be(&[u8])`, `BigUint::pow(&self, u32)`, `Sub`, `Div`, and `Rem` between `BigUint`s, and `to_u64_digits()`
    * `[✅]`   From `domain`: `Secret::try_new(SecretConstructorParams { value })` returning `Result<Secret<T>, Infallible>`, and `Secret::expose(&self) -> &T`

  * `[✅]`   `adapters/pairing/Cargo.toml`
    * `[✅]`   `[dependencies]` holds `halo2curves = "0.10.0"`, default features

  * `[✅]`   `adapters/pairing/src/lib.rs`
    * `[✅]`   The barrel holds `mod bn254_halo2curves;`

  * `[✅]`   `adapters/pairing/src/bn254_halo2curves/interface.rs`
    * `[✅]`   Imports `core::convert::Infallible`; `PairingDeclaration`, `PairingCurve`, `PairingConcrete`, `VerifierGroupArithmetic`, `PrecompileEncoding`, `TargetGroupEncodingIdentifier`, and `PAIRING_INTERFACE_VERSION` from `crate::factory::provides`; declares nothing beyond the items below
    * `[✅]`   `Bn254Halo2curvesPairing`, the unit struct `pub struct Bn254Halo2curvesPairing;` with no derives, the adapter over `halo2curves`' BN256; `Bn254Halo2curvesPairingConstructorParams`, a unit struct; `Bn254Halo2curvesPairingTryNewReturn`, the alias `Result<Bn254Halo2curvesPairing, Infallible>`, the error arm uninhabited because the adapter takes no configuration
    * `[✅]`   `Bn254Halo2curvesPairing::DECLARATION`, the inherent constant `pub const DECLARATION: PairingDeclaration = PairingDeclaration { concrete: PairingConcrete::Bn254Halo2curves, curve: PairingCurve::Bn254, verifier_group_arithmetic: VerifierGroupArithmetic::FirstGroupOnly, precompile_encoding: PrecompileEncoding::Eip196Eip197, target_group_encoding: TargetGroupEncodingIdentifier::Bn254V1, adapter_version: 1, interface_version: PAIRING_INTERFACE_VERSION }`, read by `pairing/lookup_pairing_declaration` before any instance exists; the settings-free concrete carries no settings module, and the real selection carries no settings
    * `[✅]`   `Bn254Halo2curvesScalar`, `#[derive(Clone)]`, with one field `pub(super) value: halo2curves::bn256::Fr`; `Bn254Halo2curvesG1`, `#[derive(Clone)]`, with one field `pub(super) value: halo2curves::bn256::G1Affine`; `Bn254Halo2curvesG2`, `#[derive(Clone)]`, with one field `pub(super) value: halo2curves::bn256::G2Affine`
    * `[✅]`   `Bn254Halo2curvesGt`, a struct with no derives and one field, `pub(super) value: halo2curves::bn256::Gt`, the target-group value
    * `[✅]`   `Bn254Halo2curvesEncodedG1` and `Bn254Halo2curvesEncodedG2`, `pub struct`s with `#[derive(Clone, PartialEq, Eq)]` and one field each, `pub(super) bytes: [u8; 64]` and `pub(super) bytes: [u8; 128]`; `Bn254Halo2curvesEncodedScalar`, a `pub struct` with no derives and one field `pub(super) bytes: [u8; 32]`; `Bn254Halo2curvesEncodedGt`, a `pub struct` with no derives and one field `pub(super) bytes: [u8; 384]`; the scalar and target-group encodings are returned only inside a `Secret`, and no encoded type has a public constructor or mutable byte access

  * `[✅]`   `adapters/pairing/src/bn254_halo2curves/interaction.spec.md`
    * `[✅]`   The title `` # `bn254_halo2curves` — interaction spec `` and one opening sentence stating the file as the branch contract for the `bn254_halo2curves` module of the `pairing` crate, the pairing adapter over halo2curves' BN256 (BN254) encoding group elements as EIP-196 and EIP-197 precompile input, each branch stating condition, decision, dependency call, and the exact return outcome; then one `##` section per constructor, constant, and method, each headed by its full signature and holding a table with the columns `Branch`, `Condition`, `Decision`, `Dependency call`, and `Outcome`, in the section order `pairing/bn254_arkworks` states for its spec, then `Ordering and edges`
    * `[✅]`   `try_new`: one branch, construct; condition any params; decision none; dependency call none; outcome `Ok(Bn254Halo2curvesPairing)`; the error arm has no branch, `Infallible` being uninhabited
    * `[✅]`   `DECLARATION`: the inherent constant `PairingDeclaration { concrete: PairingConcrete::Bn254Halo2curves, curve: PairingCurve::Bn254, verifier_group_arithmetic: VerifierGroupArithmetic::FirstGroupOnly, precompile_encoding: PrecompileEncoding::Eip196Eip197, target_group_encoding: TargetGroupEncodingIdentifier::Bn254V1, adapter_version: 1, interface_version: PAIRING_INTERFACE_VERSION }`, read by `pairing/lookup_pairing_declaration` before any instance exists
    * `[✅]`   `g1_generator`, `g2_generator`: one branch each, generated; condition any; decision none; dependency call `G1Affine::generator()` or `G2Affine::generator()`; outcome `Ok(G1GeneratorSuccessReturn { point })` or `Ok(G2GeneratorSuccessReturn { point })`, the generator in the owned group type
    * `[✅]`   `add_g1`, `add_g2`: one branch each, summed; dependency call `to_curve()` on each payload point, the projective `+`, then `to_affine()`; outcome `Ok(AddG1SuccessReturn { sum })` or `Ok(AddG2SuccessReturn { sum })`
    * `[✅]`   `mul_g1`, `mul_g2`: one branch each, multiplied; dependency call `to_curve()` on the point, the projective `*` the scalar's `Fr`, then `to_affine()`; outcome `Ok(MulG1SuccessReturn { product })` or `Ok(MulG2SuccessReturn { product })`; the payload, holding the scalar, drops at the end of the call and the scalar is cleared
    * `[✅]`   `msm_g1`, `msm_g2`: one branch each, summed; dependency call split the terms into a `Vec` of affine bases and a `Vec<Fr>` of the same length, then `msm_best(&scalars, &bases)`, then `to_affine()`; outcome `Ok(MsmG1SuccessReturn { sum })` or `Ok(MsmG2SuccessReturn { sum })`; every element of the `Vec<Fr>` is then set to `Fr::ZERO` and the vector passed to `black_box`; an empty term list yields the identity
    * `[✅]`   `pairing_product_is_one`: one branch, evaluated; dependency call `Bn256::multi_miller_loop` over the terms as a `Vec<(&G1Affine, &G2Affine)>` in term order, then `final_exponentiation()`; outcome `Ok(PairingProductIsOneSuccessReturn { is_one })`, `is_one` being `bool::from` of the result's `is_identity()`; an empty term list yields `is_one: true`, as EIP-197 does for empty input
    * `[✅]`   `decode_g1`, its branches in order: wrong length, condition `<[u8; 64]>::try_from(payload)` fails, decision the length check, no dependency call, outcome `Err(DecodeG1ErrorReturn::WrongLength { expected: 64, actual: payload.len() })`; non-canonical coordinate, condition either 32-byte half, copied out, reversed to little-endian, and read by `Fq::from_repr`, is none, that is, it is at least the base field modulus, decision the `CtOption` conversion per half, outcome `Err(DecodeG1ErrorReturn::NonCanonicalCoordinate)`; identity, condition both coordinates zero, decision the zero check, outcome `Ok(DecodeG1SuccessReturn { point })` holding `G1Affine::identity()`; off the curve, condition `G1Affine::from_xy(x, y)` is none, outcome `Err(DecodeG1ErrorReturn::NotOnCurve)`; outside the subgroup, condition `bool::from(point.to_curve().is_torsion_free())` is false, outcome `Err(DecodeG1ErrorReturn::NotInSubgroup)`; valid, condition every check passes, outcome `Ok(DecodeG1SuccessReturn { point })`; the section states that BN254's first group has cofactor one, so no on-curve point takes the subgroup branch and it has no unit test, while the check runs on every decode
    * `[✅]`   `decode_g2`: the same branches in the same order over 128 bytes, the length checked by `<[u8; 128]>::try_from(payload)`, read as `x.c1`, `x.c0`, `y.c1`, `y.c0`, each 32 bytes big-endian, assembled by `Fq2::new(c0, c1)`, with `DecodeG2ErrorReturn` and `expected: 128`; the identity is all four coordinates zero; the section states that the subgroup branch is reachable, since BN254's second group has a nontrivial cofactor
    * `[✅]`   `decode_scalar`, its branches in order: wrong length, condition `<[u8; 32]>::try_from(payload)` fails, outcome `Err(DecodeScalarErrorReturn::WrongLength { expected: 32, actual: payload.len() })`; non-canonical, condition the bytes reversed to little-endian and read by `Fr::from_repr` are none, that is, they are at least the group order, decision the `CtOption` conversion, outcome `Err(DecodeScalarErrorReturn::NonCanonical)`; valid, outcome `Ok(DecodeScalarSuccessReturn { scalar })`
    * `[✅]`   `encode_g1`, `encode_g2`: one branch each, encoded; decision the identity check, a point whose `coordinates()` is none; dependency call `coordinates()`, then `to_repr()` reversed to big-endian per coordinate, each written to its fixed 32-byte position; outcome `Ok(EncodeG1SuccessReturn { bytes })` holding `Bn254Halo2curvesEncodedG1`, 64 zero bytes for the identity and otherwise `x()` then `y()`, or `Ok(EncodeG2SuccessReturn { bytes })` holding `Bn254Halo2curvesEncodedG2`, 128 zero bytes for the identity and otherwise `x().c1()`, `x().c0()`, `y().c1()`, `y().c0()`, each 32 bytes big-endian
    * `[✅]`   `encode_scalar`: one branch, encoded; dependency call `to_repr()` reversed to 32 big-endian bytes; outcome `Ok(EncodeScalarSuccessReturn { bytes })`, the bytes in `Bn254Halo2curvesEncodedScalar` moved into a `Secret`
    * `[✅]`   `UNIFORM_BYTES_LENGTH`: `64`, twice the byte width of the group order
    * `[✅]`   `sample_from_uniform_bytes`, its branches in order: wrong length, condition `<&[u8; 64]>::try_from(payload.uniform.expose().as_slice())` fails, dependency call `Secret::expose`, outcome `Err(SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual })`; sampled, condition the length is 64, dependency call the 64 bytes copied into a local `[u8; 64]` and reversed, so the big-endian integer the arkworks concrete reads is the little-endian integer halo2curves reads, then `Fr::from_uniform_bytes` over the copy, which is then zeroized, outcome `Ok(SampleUniformScalarSuccessReturn { scalar })`, the scalar moved into a `Secret`; the payload's `Secret` zeroizes the input when it drops
    * `[✅]`   `add_scalar`, `mul_scalar`: one branch each; dependency call `Fr`'s `+` or `*` over the payload scalars' values; outcome `Ok(AddScalarSuccessReturn { sum })` or `Ok(MulScalarSuccessReturn { product })`, modulo the group order; the payload's scalars are cleared as it drops
    * `[✅]`   `neg_scalar`: one branch; dependency call `Fr`'s unary `-`; outcome `Ok(NegScalarSuccessReturn { negation })`, the group order minus the scalar, and zero for zero
    * `[✅]`   `neg_g1`, `neg_g2`: one branch each; dependency call the affine point's unary `-`; outcome `Ok(NegG1SuccessReturn { negation })` or `Ok(NegG2SuccessReturn { negation })`, the identity for the identity
    * `[✅]`   `is_identity_g1`, `is_identity_g2`: one branch each; dependency call `is_identity()` on the payload point; outcome `Ok(IsIdentityG1SuccessReturn { is_identity })` or `Ok(IsIdentityG2SuccessReturn { is_identity })`, `bool::from` of the `Choice`
    * `[✅]`   `pairing_product`: one branch, evaluated; dependency call `Bn256::multi_miller_loop` over the terms as a `Vec<(&G1Affine, &G2Affine)>` in term order, then `final_exponentiation()`; outcome `Ok(PairingProductSuccessReturn { product })` holding the `Gt` in the owned target-group type; an empty term list yields the target group's identity
    * `[✅]`   `encode_gt`: one branch, encoded; dependency call `inner()` on the payload's `Gt`, then each of the twelve `Fq` coefficients' `to_repr()` reversed to 32 big-endian bytes, written into its fixed position in the tower order `c0.c0.c0`, `c0.c0.c1`, `c0.c1.c0`, `c0.c1.c1`, `c0.c2.c0`, `c0.c2.c1`, `c1.c0.c0`, `c1.c0.c1`, `c1.c1.c0`, `c1.c1.c1`, `c1.c2.c0`, `c1.c2.c1` in a 384-byte buffer; outcome `Ok(EncodeGtSuccessReturn { bytes })`, a `Secret<Bn254Halo2curvesEncodedGt>` built from the complete buffer with no variable-width intermediate; the target group's identity encodes as 31 zero bytes, `01`, and 352 zero bytes
    * `[✅]`   `scalar_field_order`: one branch, read; dependency call `(-Fr::ONE).to_repr()`, the group order minus one in little-endian bytes; the bytes are reversed to big-endian and the last byte's lowest bit is set, the group order being odd and its predecessor even; outcome `Ok(ScalarFieldOrderSuccessReturn { bytes })`, 32 bytes; the error arm has no branch
    * `[✅]`   `g1_outside_subgroup_encoding`: one branch, absent; no dependency call; outcome `Ok(G1OutsideSubgroupEncodingSuccessReturn { bytes: None })`; the section states that BN254's first group has cofactor one and that no branch produces `SearchExhausted`
    * `[✅]`   `g2_outside_subgroup_encoding`, exhausted: condition the search below yields no point; outcome `Err(G2OutsideSubgroupEncodingErrorReturn::SearchExhausted)`; the section states that no input takes this branch and it has no unit test
    * `[✅]`   `g2_outside_subgroup_encoding`, found: decision, per `c0` from `successors(Some(Fq::ONE), |c0| Some(*c0 + Fq::ONE))`, `x = Fq2::new(c0, Fq::ZERO)`, `y` from `(x.square() * x + G2Affine::b()).sqrt()`, the point from `G2Affine::from_xy(x, y)`, kept when `to_curve().is_torsion_free()` is false; then, with `negated = -y`, the point is kept when the pair of the big-endian bytes of `y.c1()` and of `y.c0()`, each `to_repr()` reversed, is not greater than the same pair for `negated`, and replaced by its affine negation otherwise; dependency call `self.encode_g2(EncodeG2Params, EncodeG2Payload { point: Bn254Halo2curvesG2 { value } })`, unpacked irrefutably; outcome `Ok(G2OutsideSubgroupEncodingSuccessReturn { bytes })`, the `Bn254Halo2curvesEncodedG2` `encode_g2` returns
    * `[✅]`   `Ordering and edges`, a bulleted section: every decoder checks in the stated order, length, then canonicality, then identity, then curve, then subgroup, and copies each 32-byte coordinate out of the fixed-size array before reversing it to little-endian; an empty `msm` term list yields the identity, and an empty `pairing_product_is_one` term list yields `is_one: true`; halo2curves' `Fr` implements no `Zeroize`, so `Bn254Halo2curvesScalar`'s `Zeroize` implementation and its `Drop` set `value` to `Fr::ZERO` and pass `&self.value` to `black_box`, which keeps the clearing from being removed as a dead store, every clone a consumer places in a payload being cleared when the payload drops, and the same zero-then-`black_box` treatment clears the `Vec<Fr>` an `msm` builds; halo2curves' affine points and `Gt` implement no `Zeroize`, so `Bn254Halo2curvesG1`, `Bn254Halo2curvesG2`, and `Bn254Halo2curvesGt` clear by setting `value` to `G1Affine::identity()`, `G2Affine::identity()`, or `Gt::identity()` and passing `&self.value` to `black_box`, in their `Zeroize` implementations and their `Drop`; the outside-the-subgroup search is ascending from `c0 = 1` and stops at the first on-curve point outside the subgroup, the choice between a point and its negation follows the search and precedes the encoding, and the same call always returns the same bytes; `params` carries no control and is not read in any method, and no reference method reads its payload

  * `[✅]`   `adapters/pairing/src/bn254_halo2curves/mock.rs`
    * `[✅]`   The module-level `#![allow(dead_code, clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::too_many_arguments)]`; imports `super::interface::{Bn254Halo2curvesEncodedGt, Bn254Halo2curvesEncodedScalar, Bn254Halo2curvesG1, Bn254Halo2curvesG2, Bn254Halo2curvesGt, Bn254Halo2curvesPairing, Bn254Halo2curvesPairingConstructorParams, Bn254Halo2curvesScalar}`, `core::iter::successors`, `halo2curves::CurveAffine`, `halo2curves::bn256::{Bn256, Fq, Fq2, Fq12, Fr, G1Affine, G2Affine, Gt}`, `halo2curves::ff::{Field, PrimeField}`, `halo2curves::group::{cofactor::CofactorGroup, prime::PrimeCurveAffine}`, and `halo2curves::pairing::{Engine, MultiMillerLoop}`; and, `#[cfg(test)]`, `halo2curves::group::Curve`, `hex::decode`, `num_bigint::BigUint`, and `zeroize::ZeroizeOnDrop`
    * `[✅]`   The defaults of the concrete's owned types, each stated in its builder, with no `Default` implementation on any of them: the scalar `Fr::ONE`; the first-group point `G1Affine::generator()`; the second-group point `G2Affine::generator()`; the target-group value `Bn256::pairing(&G1Affine::generator(), &G2Affine::generator())`, a non-identity target-group value
    * `[✅]`   The builders for the concrete's owned types, each overrides struct `#[derive(Default)]` with one `Option` field and each omitted value taking the default stated above: `Bn254Halo2curvesScalarOverrides` with `pub value: Option<Fr>` and `build_bn254_halo2curves_scalar`; `Bn254Halo2curvesG1Overrides` with `pub value: Option<G1Affine>` and `build_bn254_halo2curves_g1`; `Bn254Halo2curvesG2Overrides` with `pub value: Option<G2Affine>` and `build_bn254_halo2curves_g2`; `Bn254Halo2curvesGtOverrides` with `pub value: Option<Gt>` and `build_bn254_halo2curves_gt`; no corruptions type and no invalidator, since none of these arrives as untrusted data
    * `[✅]`   `build_bn254_halo2curves_pairing() -> Bn254Halo2curvesPairing`, the real instance from `Bn254Halo2curvesPairing::try_new(Bn254Halo2curvesPairingConstructorParams)` through `let Ok(pairing) = …;`; the constructor params are fieldless, so the builder takes no overrides
    * `[✅]`   The builders for the concrete's encoded scalar and encoded target-group types, each overrides struct `#[derive(Default)]` with one `Option` field: `Bn254Halo2curvesEncodedScalarOverrides` with `pub bytes: Option<[u8; 32]>` and `build_bn254_halo2curves_encoded_scalar(overrides: Bn254Halo2curvesEncodedScalarOverrides) -> Bn254Halo2curvesEncodedScalar`, the bytes defaulting to `[1u8; 32]`; `Bn254Halo2curvesEncodedGtOverrides` with `pub bytes: Option<[u8; 384]>` and `build_bn254_halo2curves_encoded_gt`, the bytes defaulting to `[1u8; 384]`; the defaults are nonzero so a zeroized value differs from a built one; no corruptions type and no invalidator, since neither arrives as untrusted data
    * `[✅]`   The test fixtures, each `#[cfg(test)]`, since they use the crate's dev-dependencies: the constants `pairing/bn254_arkworks` states in its `mock.rs`, by the same names and values, and `UNIFORM_FIVE_HEX`, 63 zero bytes and `05`; the helpers `vector_bytes`, `zero_bytes`, `gt_identity_encoding`, `base_field_modulus`, and `requires_zeroize_on_drop`; `scalar_value(hex: &str) -> Fr`, the 32 bytes of `vector_bytes(hex)` reversed to little-endian and read by `Fr::from_repr`, unpacked by `let Some(scalar) = Option::from(…) else { panic!("the scalar is canonical") };`; `eip_196_generator() -> G1Affine`, `G1Affine::from_xy(Fq::from(1u64), Fq::from(2u64))` unpacked the same way; `eip_196_negated_generator() -> G1Affine`, `G1Affine::from_xy` over the two 32-byte halves of `vector_bytes(NEG_G1_GENERATOR_HEX)`, each reversed to little-endian and read by `Fq::from_repr`; `eip_197_generator() -> G2Affine`, `G2Affine::from_xy(Fq2::new(x_c0, x_c1), Fq2::new(y_c0, y_c1))` over the four 32-byte quarters of `vector_bytes(G2_GENERATOR_HEX)`, read in order as `x_c1`, `x_c0`, `y_c1`, `y_c0`, each reversed to little-endian and read by `Fq::from_repr`; `g2_point_from_encoding(bytes: &[u8]) -> Option<G2Affine>`, the same reading over any 128 bytes, `None` off the curve; `g2_outside_subgroup_bytes() -> Vec<u8>`, the least on-curve point outside the subgroup, `successors(Some(Fq::ONE), |c0| Some(*c0 + Fq::ONE)).find_map(|c0| { let x = Fq2::new(c0, Fq::ZERO); Option::<Fq2>::from((x.square() * x + G2Affine::b()).sqrt()).and_then(|y| Option::<G2Affine>::from(G2Affine::from_xy(x, y)).filter(|point| !bool::from(point.to_curve().is_torsion_free())).map(|_| (x, y))) })` unpacked by `let Some((x, y)) = … else { panic!("an on-curve point outside the subgroup exists") };`, encoded by extending with `to_repr().as_ref().iter().rev()` of `x.c1()`, `x.c0()`, `y.c1()`, `y.c0()`; `definition_exponent() -> Vec<u64>`, `let Some(p) = BigUint::parse_bytes(Fq::MODULUS.trim_start_matches("0x").as_bytes(), 16) else { panic!("the base field modulus parses") }; let Some(r) = BigUint::parse_bytes(Fr::MODULUS.trim_start_matches("0x").as_bytes(), 16) else { panic!("the group order parses") }; ((p.pow(12) - BigUint::from(1u32)) / r).to_u64_digits()`; `definition_value(g1: G1Affine, g2: G2Affine) -> Fq12`, `Bn256::multi_miller_loop(&[(&g1, &g2)]).pow_vartime(definition_exponent())`, the identifier's definition of the pairing value; `definition_value_power(g1: G1Affine, g2: G2Affine, exponent: u64) -> Fq12`, `definition_value(g1, g2).pow_vartime([exponent])`; `definition_encoding() -> Vec<u8>`, the twelve coefficients of `definition_value(eip_196_generator(), eip_197_generator())` read through the `c0()`, `c1()`, and `c2()` accessors in the tower order `c0.c0.c0`, `c0.c0.c1`, `c0.c1.c0`, `c0.c1.c1`, `c0.c2.c0`, `c0.c2.c1`, `c1.c0.c0`, `c1.c0.c1`, `c1.c1.c0`, `c1.c1.c1`, `c1.c2.c0`, `c1.c2.c1`, each `to_repr()` reversed to 32 big-endian bytes and appended; `eip_196_doubled_generator() -> G1Affine`, `(eip_196_generator().to_curve() + eip_196_generator().to_curve()).to_affine()`
    * `[✅]`   No mock function: the concrete is built as a real instance and owns no free function

  * `[✅]`   `adapters/pairing/src/bn254_halo2curves/test.rs`
    * `[✅]`   Module-level `#![allow(clippy::expect_used)]`; imports from `super::provides` the concrete, its owned types, and the mock's builders, constants, and helpers each block uses; from `crate::factory::provides` the params, payloads, errors, trait names, member and value identifiers, declared sets, and builders each block uses; `HashSet` from `std::collections`; and the `halo2curves`, `num_bigint::BigUint`, and `zeroize::Zeroize` names each block uses
    * `[✅]`   Compile-time assertion over `Bn254Halo2curvesPairing::DECLARATION`
      * `[✅]`   Contract: the inherent constant is `PairingDeclaration { concrete: PairingConcrete::Bn254Halo2curves, curve: PairingCurve::Bn254, verifier_group_arithmetic: VerifierGroupArithmetic::FirstGroupOnly, precompile_encoding: PrecompileEncoding::Eip196Eip197, target_group_encoding: TargetGroupEncodingIdentifier::Bn254V1, adapter_version: 1, interface_version: PAIRING_INTERFACE_VERSION }`, read through `pairing/lookup_pairing_declaration` without an instance
      * `[✅]`   Arrange: the type alone
      * `[✅]`   Act: a module-level `const _: () = assert!(…);` reading `Bn254Halo2curvesPairing::DECLARATION`, `concrete` and each enum field tested with `matches!` and each version with `==`, no text-name assertion
      * `[✅]`   Assert: the module compiles only if every field holds
    * `[✅]`   `pairing_concretes_declares_only_real_members`
      * `[✅]`   Contract: the declared set holds the real members alone → `PAIRING_CONCRETES` contains `PairingConcrete::Bn254Arkworks` and `PairingConcrete::Bn254Halo2curves` and no `Mock(_)` member
      * `[✅]`   Collaborators: none; the declared set is read as a constant, no instance is built
      * `[✅]`   Arrange: none; the constant alone
      * `[✅]`   Act: reading `PAIRING_CONCRETES`
      * `[✅]`   Assert: every member is `PairingConcrete::Bn254Arkworks` or `PairingConcrete::Bn254Halo2curves` and none is `Mock(_)`
    * `[✅]`   `target_group_encoding_identifiers_declares_only_real_values`
      * `[✅]`   Contract: the declared set holds the real provided values alone → `TARGET_GROUP_ENCODING_IDENTIFIERS` contains `TargetGroupEncodingIdentifier::Bn254V1` and no `Mock` member
      * `[✅]`   Collaborators: none; the declared set is read as a constant, no instance is built
      * `[✅]`   Arrange: none; the constant alone
      * `[✅]`   Act: reading `TARGET_GROUP_ENCODING_IDENTIFIERS`
      * `[✅]`   Assert: every member is `TargetGroupEncodingIdentifier::Bn254V1` and none is `Mock`
    * `[✅]`   `every_declared_identifier_spells_a_distinct_name`
      * `[✅]`   Contract: each declared member has one total spelling → the `name()` spellings of the members of `PAIRING_CONCRETES` are pairwise distinct, and so are those of `TARGET_GROUP_ENCODING_IDENTIFIERS`
      * `[✅]`   Collaborators: none; the declared sets are read as constants, no instance is built
      * `[✅]`   Arrange: none; the constants alone
      * `[✅]`   Act: collecting each member's `name()` into a `HashSet` per declared set
      * `[✅]`   Assert: each `HashSet`'s length equals its declared set's length
    * `[✅]`   `no_value_identifier_declares_a_mock_only_variant`
      * `[✅]`   Contract: a value identifier holds real provided values alone → `PairingCurve`, `VerifierGroupArithmetic`, `PrecompileEncoding`, and `TargetGroupEncodingIdentifier` have no mock-only variant
      * `[✅]`   Collaborators: none; the value identifiers are read as types, no instance is built
      * `[✅]`   Arrange: none; the types alone
      * `[✅]`   Act: reading `core::mem::variant_count` on each value identifier
      * `[✅]`   Assert: each variant count equals its real variants' count, so no mock-only variant exists
    * `[✅]`   `bn254_halo2curves_declared_selection_builds_without_configuration`
      * `[✅]`   Contract: the declared-selection builder's `Bn254Halo2curves` arm returns the real selection with no configuration → `build_pairing_concrete` over `PairingConcrete::Bn254Halo2curves` yields that member, the fieldless `PairingConcreteOverrides` carrying nothing
      * `[✅]`   Collaborators: none; the builder builds no adapter
      * `[✅]`   Arrange: `PairingConcreteOverrides` default
      * `[✅]`   Act: `build_pairing_concrete(PairingConcrete::Bn254Halo2curves, PairingConcreteOverrides::default())`
      * `[✅]`   Assert: the result is `PairingConcrete::Bn254Halo2curves`
    * `[✅]`   `bn254_halo2curves_scalar_is_zeroize_on_drop`
      * `[✅]`   Contract: `Bn254Halo2curvesScalar` implements `ZeroizeOnDrop`, the marker the sampling bound requires
      * `[✅]`   Collaborators: none
      * `[✅]`   Arrange: the type alone
      * `[✅]`   Act: `requires_zeroize_on_drop::<Bn254Halo2curvesScalar>()`
      * `[✅]`   Assert: the block compiles only if the type implements `ZeroizeOnDrop`
    * `[✅]`   `bn254_halo2curves_scalar_zeroize_clears_its_value`
      * `[✅]`   Contract: `Zeroize::zeroize` on a living scalar → its `value` is zero
      * `[✅]`   Collaborators: none; fixture `build_bn254_halo2curves_scalar`
      * `[✅]`   Arrange: `build_bn254_halo2curves_scalar` with the value override `Fr::from(5u64)`, so a scalar left unchanged differs from the expected zero
      * `[✅]`   Act: `scalar.zeroize()`
      * `[✅]`   Assert: `scalar.value` equals `Fr::ZERO`
    * `[✅]`   `bn254_halo2curves_g1_zeroize_clears_its_value`
      * `[✅]`   Contract: `Zeroize::zeroize` on a living first-group element → its `value` is the identity
      * `[✅]`   Collaborators: none; fixture `build_bn254_halo2curves_g1`
      * `[✅]`   Arrange: `build_bn254_halo2curves_g1` at its default generator, which differs from the identity
      * `[✅]`   Act: `g1.zeroize()`
      * `[✅]`   Assert: `g1.value` equals `G1Affine::identity()`
    * `[✅]`   `bn254_halo2curves_g2_zeroize_clears_its_value`
      * `[✅]`   Contract: `Zeroize::zeroize` on a living second-group element → its `value` is the identity
      * `[✅]`   Collaborators: none; fixture `build_bn254_halo2curves_g2`
      * `[✅]`   Arrange: `build_bn254_halo2curves_g2` at its default generator, which differs from the identity
      * `[✅]`   Act: `g2.zeroize()`
      * `[✅]`   Assert: `g2.value` equals `G2Affine::identity()`
    * `[✅]`   `bn254_halo2curves_gt_zeroize_clears_its_value`
      * `[✅]`   Contract: `Zeroize::zeroize` on a living target-group value → its `value` is the target group's identity
      * `[✅]`   Collaborators: none; fixture `build_bn254_halo2curves_gt`
      * `[✅]`   Arrange: `build_bn254_halo2curves_gt` at its default pairing of the generators, which differs from the identity
      * `[✅]`   Act: `gt.zeroize()`
      * `[✅]`   Assert: `gt.value` equals `Gt::identity()`
    * `[✅]`   `bn254_halo2curves_encoded_scalar_zeroize_clears_its_bytes`
      * `[✅]`   Contract: `Zeroize::zeroize` on a living encoded scalar → every byte is zero
      * `[✅]`   Collaborators: none; fixture `build_bn254_halo2curves_encoded_scalar`
      * `[✅]`   Arrange: `build_bn254_halo2curves_encoded_scalar` at its nonzero default
      * `[✅]`   Act: `encoded.zeroize()`
      * `[✅]`   Assert: the bytes of `encoded` equal `zero_bytes(32)` as a byte slice
    * `[✅]`   `bn254_halo2curves_encoded_gt_zeroize_clears_its_bytes`
      * `[✅]`   Contract: `Zeroize::zeroize` on a living encoded target-group value → every byte is zero
      * `[✅]`   Collaborators: none; fixture `build_bn254_halo2curves_encoded_gt`
      * `[✅]`   Arrange: `build_bn254_halo2curves_encoded_gt` at its nonzero default
      * `[✅]`   Act: `encoded.zeroize()`
      * `[✅]`   Assert: the bytes of `encoded` equal `zero_bytes(384)` as a byte slice
    * `[✅]`   `try_new_returns_the_halo2curves_pairing`
      * `[✅]`   Contract: any params → `Ok(Bn254Halo2curvesPairing)`
      * `[✅]`   Collaborators: none; `Bn254Halo2curvesPairingConstructorParams` by its production value
      * `[✅]`   Arrange: `Bn254Halo2curvesPairingConstructorParams` by its production value
      * `[✅]`   Act: `Bn254Halo2curvesPairing::try_new(Bn254Halo2curvesPairingConstructorParams)`
      * `[✅]`   Assert: `result.is_ok()` is true
    * `[✅]`   `uniform_bytes_length_is_twice_the_group_order_width`
      * `[✅]`   Contract: `UNIFORM_BYTES_LENGTH` is twice the byte width of the group order
      * `[✅]`   Collaborators: none
      * `[✅]`   Arrange: the type alone
      * `[✅]`   Act: reading `<Bn254Halo2curvesScalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH`
      * `[✅]`   Assert: the constant equals the literal 64 written in the assertion
    * `[✅]`   `g1_generator_returns_the_eip_196_generator`
      * `[✅]`   Contract: any call → `Ok(G1GeneratorSuccessReturn { point })` holding the first group's generator
      * `[✅]`   Collaborators: halo2curves, run for real as the vendor; fixture `build_bn254_halo2curves_pairing`; `G1GeneratorParams` and `G1GeneratorPayload` by their production values
      * `[✅]`   Arrange: `build_bn254_halo2curves_pairing()`
      * `[✅]`   Act: `g1_generator(G1GeneratorParams, G1GeneratorPayload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.point.value` equals `eip_196_generator()`
    * `[✅]`   `g2_generator_returns_the_eip_197_generator`
      * `[✅]`   Contract: any call → `Ok(G2GeneratorSuccessReturn { point })` holding the second group's generator
      * `[✅]`   Collaborators: halo2curves, run for real as the vendor; fixture `build_bn254_halo2curves_pairing`; `G2GeneratorParams` and `G2GeneratorPayload` by their production values
      * `[✅]`   Arrange: `build_bn254_halo2curves_pairing()`
      * `[✅]`   Act: `g2_generator(G2GeneratorParams, G2GeneratorPayload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.point.value` equals `eip_197_generator()`
    * `[✅]`   `add_g1_of_a_point_and_its_negation_is_the_identity`
      * `[✅]`   Contract: `payload.left` and `payload.right` → `Ok(AddG1SuccessReturn { sum })` holding their sum
      * `[✅]`   Collaborators: halo2curves' projective addition, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_add_g1_payload`, and `build_bn254_halo2curves_g1`
      * `[✅]`   Arrange: `build_add_g1_payload` with the left override `build_bn254_halo2curves_g1` at its default generator and the right override `build_bn254_halo2curves_g1` with the value `eip_196_negated_generator()`, so a sum that returns either input differs from the identity
      * `[✅]`   Act: `add_g1(AddG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G1Affine::identity()`
    * `[✅]`   `add_g1_of_the_identity_and_a_point_is_the_point`
      * `[✅]`   Contract: `payload.left` and `payload.right` → `Ok(AddG1SuccessReturn { sum })` holding their sum
      * `[✅]`   Collaborators: halo2curves' projective addition, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_add_g1_payload`, and `build_bn254_halo2curves_g1`
      * `[✅]`   Arrange: `build_add_g1_payload` with the left override `build_bn254_halo2curves_g1` with the value `G1Affine::identity()` and the right override `build_bn254_halo2curves_g1` at its default generator, so a sum that returns the left input differs from the generator
      * `[✅]`   Act: `add_g1(AddG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `eip_196_generator()`
    * `[✅]`   `add_g2_of_a_point_and_its_negation_is_the_identity`
      * `[✅]`   Contract: `payload.left` and `payload.right` → `Ok(AddG2SuccessReturn { sum })` holding their sum
      * `[✅]`   Collaborators: halo2curves' projective addition, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_add_g2_payload`, and `build_bn254_halo2curves_g2`
      * `[✅]`   Arrange: `build_add_g2_payload` with the left override `build_bn254_halo2curves_g2` at its default generator and the right override `build_bn254_halo2curves_g2` with the value `-eip_197_generator()`, so a sum that returns either input differs from the identity
      * `[✅]`   Act: `add_g2(AddG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G2Affine::identity()`
    * `[✅]`   `add_g2_of_the_identity_and_a_point_is_the_point`
      * `[✅]`   Contract: `payload.left` and `payload.right` → `Ok(AddG2SuccessReturn { sum })` holding their sum
      * `[✅]`   Collaborators: halo2curves' projective addition, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_add_g2_payload`, and `build_bn254_halo2curves_g2`
      * `[✅]`   Arrange: `build_add_g2_payload` with the left override `build_bn254_halo2curves_g2` with the value `G2Affine::identity()` and the right override `build_bn254_halo2curves_g2` at its default generator, so a sum that returns the left input differs from the generator
      * `[✅]`   Act: `add_g2(AddG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `eip_197_generator()`
    * `[✅]`   `mul_g1_by_one_is_the_point`
      * `[✅]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG1SuccessReturn { product })` holding the point multiplied by the scalar
      * `[✅]`   Collaborators: halo2curves' projective scalar multiplication, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_mul_g1_payload`, `build_bn254_halo2curves_g1`, and `build_bn254_halo2curves_scalar`
      * `[✅]`   Arrange: `build_mul_g1_payload` with the point override `build_bn254_halo2curves_g1` at its default generator and the scalar override `build_bn254_halo2curves_scalar` with the value `Fr::from(1u64)`
      * `[✅]`   Act: `mul_g1(MulG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `eip_196_generator()`
    * `[✅]`   `mul_g1_by_zero_is_the_identity`
      * `[✅]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG1SuccessReturn { product })` holding the point multiplied by the scalar
      * `[✅]`   Collaborators: halo2curves' projective scalar multiplication, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_mul_g1_payload`, `build_bn254_halo2curves_g1`, and `build_bn254_halo2curves_scalar`
      * `[✅]`   Arrange: `build_mul_g1_payload` with the point override `build_bn254_halo2curves_g1` at its default generator and the scalar override `build_bn254_halo2curves_scalar` with the value `Fr::ZERO`, so a product that returns the point differs from the identity
      * `[✅]`   Act: `mul_g1(MulG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `G1Affine::identity()`
    * `[✅]`   `mul_g1_by_the_group_order_minus_one_is_the_negated_generator`
      * `[✅]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG1SuccessReturn { product })` holding the point multiplied by the scalar
      * `[✅]`   Collaborators: halo2curves' projective scalar multiplication, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_mul_g1_payload`, `build_bn254_halo2curves_g1`, and `build_bn254_halo2curves_scalar`
      * `[✅]`   Arrange: `build_mul_g1_payload` with the point override `build_bn254_halo2curves_g1` at its default generator and the scalar override `build_bn254_halo2curves_scalar` with the value `scalar_value(GROUP_ORDER_MINUS_ONE_HEX)`, so a product that ignores the scalar differs from the negated generator
      * `[✅]`   Act: `mul_g1(MulG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `eip_196_negated_generator()`
    * `[✅]`   `mul_g2_by_one_is_the_point`
      * `[✅]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG2SuccessReturn { product })` holding the point multiplied by the scalar
      * `[✅]`   Collaborators: halo2curves' projective scalar multiplication, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_mul_g2_payload`, `build_bn254_halo2curves_g2`, and `build_bn254_halo2curves_scalar`
      * `[✅]`   Arrange: `build_mul_g2_payload` with the point override `build_bn254_halo2curves_g2` at its default generator and the scalar override `build_bn254_halo2curves_scalar` with the value `Fr::from(1u64)`
      * `[✅]`   Act: `mul_g2(MulG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `eip_197_generator()`
    * `[✅]`   `mul_g2_by_zero_is_the_identity`
      * `[✅]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG2SuccessReturn { product })` holding the point multiplied by the scalar
      * `[✅]`   Collaborators: halo2curves' projective scalar multiplication, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_mul_g2_payload`, `build_bn254_halo2curves_g2`, and `build_bn254_halo2curves_scalar`
      * `[✅]`   Arrange: `build_mul_g2_payload` with the point override `build_bn254_halo2curves_g2` at its default generator and the scalar override `build_bn254_halo2curves_scalar` with the value `Fr::ZERO`, so a product that returns the point differs from the identity
      * `[✅]`   Act: `mul_g2(MulG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `G2Affine::identity()`
    * `[✅]`   `mul_g2_by_the_group_order_minus_one_is_the_negated_point`
      * `[✅]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG2SuccessReturn { product })` holding the point multiplied by the scalar
      * `[✅]`   Collaborators: halo2curves' projective scalar multiplication, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_mul_g2_payload`, `build_bn254_halo2curves_g2`, and `build_bn254_halo2curves_scalar`
      * `[✅]`   Arrange: `build_mul_g2_payload` with the point override `build_bn254_halo2curves_g2` at its default generator and the scalar override `build_bn254_halo2curves_scalar` with the value `scalar_value(GROUP_ORDER_MINUS_ONE_HEX)`, so a product that ignores the scalar differs from the negated generator
      * `[✅]`   Act: `mul_g2(MulG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `-eip_197_generator()`
    * `[✅]`   `msm_g1_of_no_terms_is_the_identity`
      * `[✅]`   Contract: an empty `payload.terms` → `Ok(MsmG1SuccessReturn { sum })` holding the identity
      * `[✅]`   Collaborators: halo2curves' `msm_best`, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing` and `build_msm_g1_payload`
      * `[✅]`   Arrange: `build_msm_g1_payload` at its default of no terms
      * `[✅]`   Act: `msm_g1(MsmG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G1Affine::identity()`
    * `[✅]`   `msm_g1_pairs_each_base_with_its_own_scalar`
      * `[✅]`   Contract: `payload.terms` → `Ok(MsmG1SuccessReturn { sum })` holding the sum of each base multiplied by its own scalar
      * `[✅]`   Collaborators: halo2curves' `msm_best`, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_msm_g1_payload`, `build_msm_g1_term`, `build_bn254_halo2curves_g1`, and `build_bn254_halo2curves_scalar`
      * `[✅]`   Arrange: `build_msm_g1_payload` with the terms override holding two `build_msm_g1_term` values: the generator with the scalar `Fr::from(1u64)`, and the base `eip_196_negated_generator()` with the scalar `Fr::ZERO`, so bases and scalars exchanged between terms yield the negated generator
      * `[✅]`   Act: `msm_g1(MsmG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `eip_196_generator()`
    * `[✅]`   `msm_g1_sums_its_terms`
      * `[✅]`   Contract: `payload.terms` → `Ok(MsmG1SuccessReturn { sum })` holding the sum of each base multiplied by its own scalar
      * `[✅]`   Collaborators: halo2curves' `msm_best`, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_msm_g1_payload`, `build_msm_g1_term`, `build_bn254_halo2curves_g1`, and `build_bn254_halo2curves_scalar`
      * `[✅]`   Arrange: `build_msm_g1_payload` with the terms override holding two `build_msm_g1_term` values: the generator with the scalar `Fr::from(1u64)`, and the base `eip_196_negated_generator()` with the scalar `Fr::from(1u64)`, so a sum over the first term alone differs from the identity
      * `[✅]`   Act: `msm_g1(MsmG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G1Affine::identity()`
    * `[✅]`   `msm_g2_of_no_terms_is_the_identity`
      * `[✅]`   Contract: an empty `payload.terms` → `Ok(MsmG2SuccessReturn { sum })` holding the identity
      * `[✅]`   Collaborators: halo2curves' `msm_best`, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing` and `build_msm_g2_payload`
      * `[✅]`   Arrange: `build_msm_g2_payload` at its default of no terms
      * `[✅]`   Act: `msm_g2(MsmG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G2Affine::identity()`
    * `[✅]`   `msm_g2_pairs_each_base_with_its_own_scalar`
      * `[✅]`   Contract: `payload.terms` → `Ok(MsmG2SuccessReturn { sum })` holding the sum of each base multiplied by its own scalar
      * `[✅]`   Collaborators: halo2curves' `msm_best`, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_msm_g2_payload`, `build_msm_g2_term`, `build_bn254_halo2curves_g2`, and `build_bn254_halo2curves_scalar`
      * `[✅]`   Arrange: `build_msm_g2_payload` with the terms override holding two `build_msm_g2_term` values: the generator with the scalar `Fr::from(1u64)`, and the base `-eip_197_generator()` with the scalar `Fr::ZERO`, so bases and scalars exchanged between terms yield the negated generator
      * `[✅]`   Act: `msm_g2(MsmG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `eip_197_generator()`
    * `[✅]`   `msm_g2_sums_its_terms`
      * `[✅]`   Contract: `payload.terms` → `Ok(MsmG2SuccessReturn { sum })` holding the sum of each base multiplied by its own scalar
      * `[✅]`   Collaborators: halo2curves' `msm_best`, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_msm_g2_payload`, `build_msm_g2_term`, `build_bn254_halo2curves_g2`, and `build_bn254_halo2curves_scalar`
      * `[✅]`   Arrange: `build_msm_g2_payload` with the terms override holding two `build_msm_g2_term` values: the generator with the scalar `Fr::from(1u64)`, and the base `-eip_197_generator()` with the scalar `Fr::from(1u64)`, so a sum over the first term alone differs from the identity
      * `[✅]`   Act: `msm_g2(MsmG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G2Affine::identity()`
    * `[✅]`   `pairing_product_is_one_of_no_terms_is_true`
      * `[✅]`   Contract: an empty `payload.terms` → `Ok(PairingProductIsOneSuccessReturn { is_one })` with `is_one` true
      * `[✅]`   Collaborators: halo2curves' multi-Miller loop and final exponentiation, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing` and `build_pairing_product_is_one_payload`
      * `[✅]`   Arrange: `build_pairing_product_is_one_payload` at its default of no terms
      * `[✅]`   Act: `pairing_product_is_one(PairingProductIsOneParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_one` is true
    * `[✅]`   `pairing_product_is_one_of_a_pairing_and_its_first_group_negation_is_true`
      * `[✅]`   Contract: `payload.terms` whose pairings multiply to the target group's identity → `Ok(PairingProductIsOneSuccessReturn { is_one })` with `is_one` true
      * `[✅]`   Collaborators: halo2curves' multi-Miller loop and final exponentiation, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_pairing_product_is_one_payload`, `build_pairing_product_term`, and `build_bn254_halo2curves_g1`
      * `[✅]`   Arrange: `build_pairing_product_is_one_payload` with the terms override holding two `build_pairing_product_term` values: the generators, and the first-group override `build_bn254_halo2curves_g1` with the value `eip_196_negated_generator()` over the second-group generator
      * `[✅]`   Act: `pairing_product_is_one(PairingProductIsOneParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_one` is true
    * `[✅]`   `pairing_product_is_one_of_the_generators_is_false`
      * `[✅]`   Contract: `payload.terms` whose pairings multiply to a value other than the target group's identity → `Ok(PairingProductIsOneSuccessReturn { is_one })` with `is_one` false
      * `[✅]`   Collaborators: halo2curves' multi-Miller loop and final exponentiation, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_pairing_product_is_one_payload`, and `build_pairing_product_term`
      * `[✅]`   Arrange: `build_pairing_product_is_one_payload` with the terms override holding one `build_pairing_product_term` at its default generators
      * `[✅]`   Act: `pairing_product_is_one(PairingProductIsOneParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_one` is false
    * `[✅]`   `decode_g1_rejects_a_wrong_length`
      * `[✅]`   Contract: `payload.len() != 64` → `Err(DecodeG1ErrorReturn::WrongLength { expected: 64, actual: payload.len() })`
      * `[✅]`   Collaborators: none called; fixture `build_bn254_halo2curves_pairing`; the untrusted bytes from `zero_bytes`
      * `[✅]`   Arrange: the payload `zero_bytes(63)`, which differs from the required length
      * `[✅]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(DecodeG1ErrorReturn::WrongLength { expected: 64, actual: 63 })`, the whole expected error
    * `[✅]`   `decode_g1_rejects_a_non_canonical_coordinate`
      * `[✅]`   Contract: a 64-byte payload whose first half is at least the base field modulus → `Err(DecodeG1ErrorReturn::NonCanonicalCoordinate)`
      * `[✅]`   Collaborators: halo2curves' `Fq::from_repr`, run for real as the vendor; fixture `build_bn254_halo2curves_pairing`; the untrusted bytes from `vector_bytes(G1_X_AT_MODULUS_HEX)`
      * `[✅]`   Arrange: the payload `vector_bytes(G1_X_AT_MODULUS_HEX)`, whose coordinates would otherwise fail the curve check, so a decoder that checks the curve first returns a different error
      * `[✅]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(DecodeG1ErrorReturn::NonCanonicalCoordinate)`
    * `[✅]`   `decode_g1_decodes_the_identity_from_zero_bytes`
      * `[✅]`   Contract: a 64-byte payload with both coordinates zero → `Ok(DecodeG1SuccessReturn { point })` holding the first group's identity
      * `[✅]`   Collaborators: halo2curves, run for real as the vendor; fixture `build_bn254_halo2curves_pairing`; the untrusted bytes from `zero_bytes`
      * `[✅]`   Arrange: the payload `zero_bytes(64)`
      * `[✅]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[✅]`   Assert: the success arm is extracted with `expect`; `success.point.value` equals `G1Affine::identity()`
    * `[✅]`   `decode_g1_rejects_a_point_off_the_curve`
      * `[✅]`   Contract: a canonical 64-byte payload that satisfies no curve equation → `Err(DecodeG1ErrorReturn::NotOnCurve)`
      * `[✅]`   Collaborators: halo2curves' `from_xy`, run for real as the vendor; fixture `build_bn254_halo2curves_pairing`; the untrusted bytes from `vector_bytes(G1_OFF_CURVE_HEX)`
      * `[✅]`   Arrange: the payload `vector_bytes(G1_OFF_CURVE_HEX)`
      * `[✅]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(DecodeG1ErrorReturn::NotOnCurve)`
    * `[✅]`   `decode_g1_decodes_the_eip_196_generator`
      * `[✅]`   Contract: a canonical 64-byte payload on the curve → `Ok(DecodeG1SuccessReturn { point })` holding that point
      * `[✅]`   Collaborators: halo2curves, run for real as the vendor; fixture `build_bn254_halo2curves_pairing`; the untrusted bytes from `vector_bytes(G1_GENERATOR_HEX)`
      * `[✅]`   Arrange: the payload `vector_bytes(G1_GENERATOR_HEX)`
      * `[✅]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[✅]`   Assert: the success arm is extracted with `expect`; `success.point.value` equals `eip_196_generator()`
    * `[✅]`   `decode_g2_rejects_a_wrong_length`
      * `[✅]`   Contract: `payload.len() != 128` → `Err(DecodeG2ErrorReturn::WrongLength { expected: 128, actual: payload.len() })`
      * `[✅]`   Collaborators: none called; fixture `build_bn254_halo2curves_pairing`; the untrusted bytes from `zero_bytes`
      * `[✅]`   Arrange: the payload `zero_bytes(127)`, which differs from the required length
      * `[✅]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(DecodeG2ErrorReturn::WrongLength { expected: 128, actual: 127 })`, the whole expected error
    * `[✅]`   `decode_g2_rejects_a_non_canonical_coordinate`
      * `[✅]`   Contract: a 128-byte payload whose first coordinate is at least the base field modulus → `Err(DecodeG2ErrorReturn::NonCanonicalCoordinate)`
      * `[✅]`   Collaborators: halo2curves' `Fq::from_repr`, run for real as the vendor; fixture `build_bn254_halo2curves_pairing`; the untrusted bytes from `vector_bytes(G2_X_C1_AT_MODULUS_HEX)`
      * `[✅]`   Arrange: the payload `vector_bytes(G2_X_C1_AT_MODULUS_HEX)`, whose coordinates would otherwise fail the curve check, so a decoder that checks the curve first returns a different error
      * `[✅]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(DecodeG2ErrorReturn::NonCanonicalCoordinate)`
    * `[✅]`   `decode_g2_decodes_the_identity_from_zero_bytes`
      * `[✅]`   Contract: a 128-byte payload with every coordinate zero → `Ok(DecodeG2SuccessReturn { point })` holding the second group's identity
      * `[✅]`   Collaborators: halo2curves, run for real as the vendor; fixture `build_bn254_halo2curves_pairing`; the untrusted bytes from `zero_bytes`
      * `[✅]`   Arrange: the payload `zero_bytes(128)`
      * `[✅]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[✅]`   Assert: the success arm is extracted with `expect`; `success.point.value` equals `G2Affine::identity()`
    * `[✅]`   `decode_g2_rejects_a_point_off_the_curve`
      * `[✅]`   Contract: a canonical 128-byte payload that satisfies no curve equation → `Err(DecodeG2ErrorReturn::NotOnCurve)`
      * `[✅]`   Collaborators: halo2curves' `from_xy`, run for real as the vendor; fixture `build_bn254_halo2curves_pairing`; the untrusted bytes from `vector_bytes(G2_GENERATOR_OFF_CURVE_HEX)`
      * `[✅]`   Arrange: the payload `vector_bytes(G2_GENERATOR_OFF_CURVE_HEX)`
      * `[✅]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(DecodeG2ErrorReturn::NotOnCurve)`
    * `[✅]`   `decode_g2_rejects_a_point_outside_the_subgroup`
      * `[✅]`   Contract: a canonical 128-byte payload on the curve and outside the prime-order subgroup → `Err(DecodeG2ErrorReturn::NotInSubgroup)`
      * `[✅]`   Collaborators: halo2curves' torsion check, run for real as the vendor; fixture `build_bn254_halo2curves_pairing`; the untrusted bytes from `g2_outside_subgroup_bytes()`
      * `[✅]`   Arrange: the payload `g2_outside_subgroup_bytes()`
      * `[✅]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(DecodeG2ErrorReturn::NotInSubgroup)`
    * `[✅]`   `decode_g2_decodes_the_eip_197_generator`
      * `[✅]`   Contract: a canonical 128-byte payload on the curve and in the subgroup → `Ok(DecodeG2SuccessReturn { point })` holding that point
      * `[✅]`   Collaborators: halo2curves, run for real as the vendor; fixture `build_bn254_halo2curves_pairing`; the untrusted bytes from `vector_bytes(G2_GENERATOR_HEX)`
      * `[✅]`   Arrange: the payload `vector_bytes(G2_GENERATOR_HEX)`
      * `[✅]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[✅]`   Assert: the success arm is extracted with `expect`; `success.point.value` equals `eip_197_generator()`
    * `[✅]`   `decode_scalar_rejects_a_wrong_length`
      * `[✅]`   Contract: `payload.len() != 32` → `Err(DecodeScalarErrorReturn::WrongLength { expected: 32, actual: payload.len() })`
      * `[✅]`   Collaborators: none called; fixture `build_bn254_halo2curves_pairing`; the untrusted bytes from `zero_bytes`
      * `[✅]`   Arrange: the payload `zero_bytes(31)`, which differs from the required length
      * `[✅]`   Act: `decode_scalar(DecodeScalarParams, &payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(DecodeScalarErrorReturn::WrongLength { expected: 32, actual: 31 })`, the whole expected error
    * `[✅]`   `decode_scalar_rejects_the_group_order`
      * `[✅]`   Contract: a 32-byte payload at least the group order → `Err(DecodeScalarErrorReturn::NonCanonical)`
      * `[✅]`   Collaborators: halo2curves' `Fr::from_repr`, run for real as the vendor; fixture `build_bn254_halo2curves_pairing`; the untrusted bytes from `vector_bytes(GROUP_ORDER_HEX)`
      * `[✅]`   Arrange: the payload `vector_bytes(GROUP_ORDER_HEX)`
      * `[✅]`   Act: `decode_scalar(DecodeScalarParams, &payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(DecodeScalarErrorReturn::NonCanonical)`
    * `[✅]`   `decode_scalar_decodes_the_largest_canonical_scalar`
      * `[✅]`   Contract: a canonical 32-byte payload → `Ok(DecodeScalarSuccessReturn { scalar })` holding that scalar
      * `[✅]`   Collaborators: halo2curves' `Fr::from_repr`, run for real as the vendor; fixture `build_bn254_halo2curves_pairing`; the untrusted bytes from `vector_bytes(GROUP_ORDER_MINUS_ONE_HEX)`
      * `[✅]`   Arrange: the payload `vector_bytes(GROUP_ORDER_MINUS_ONE_HEX)`
      * `[✅]`   Act: `decode_scalar(DecodeScalarParams, &payload)`
      * `[✅]`   Assert: the success arm is extracted with `expect`; `success.scalar.value` equals `-Fr::from(1u64)`
    * `[✅]`   `encode_g1_writes_the_eip_196_generator`
      * `[✅]`   Contract: a first-group point → `Ok(EncodeG1SuccessReturn { bytes })` holding `x` then `y`, each 32 bytes big-endian, in this concrete's `Bn254Halo2curvesEncodedG1`
      * `[✅]`   Collaborators: halo2curves' coordinate encoding, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_encode_g1_payload`, and `build_bn254_halo2curves_g1`
      * `[✅]`   Arrange: `build_encode_g1_payload` with the point override `build_bn254_halo2curves_g1` at its default generator
      * `[✅]`   Act: `encode_g1(EncodeG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.bytes` binds to a `Bn254Halo2curvesEncodedG1` typed binding; its bytes equal `vector_bytes(G1_GENERATOR_HEX)` as a byte slice
    * `[✅]`   `encode_g1_writes_the_identity_as_zero_bytes`
      * `[✅]`   Contract: the first group's identity → `Ok(EncodeG1SuccessReturn { bytes })` holding 64 zero bytes
      * `[✅]`   Collaborators: halo2curves, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_encode_g1_payload`, and `build_bn254_halo2curves_g1`
      * `[✅]`   Arrange: `build_encode_g1_payload` with the point override `build_bn254_halo2curves_g1` with the value `G1Affine::identity()`
      * `[✅]`   Act: `encode_g1(EncodeG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; the bytes of `success.bytes` equal `zero_bytes(64)` as a byte slice
    * `[✅]`   `encode_g2_writes_the_eip_197_generator`
      * `[✅]`   Contract: a second-group point → `Ok(EncodeG2SuccessReturn { bytes })` holding `x.c1`, `x.c0`, `y.c1`, `y.c0`, each 32 bytes big-endian, in this concrete's `Bn254Halo2curvesEncodedG2`
      * `[✅]`   Collaborators: halo2curves' coordinate encoding, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_encode_g2_payload`, and `build_bn254_halo2curves_g2`
      * `[✅]`   Arrange: `build_encode_g2_payload` with the point override `build_bn254_halo2curves_g2` at its default generator
      * `[✅]`   Act: `encode_g2(EncodeG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.bytes` binds to a `Bn254Halo2curvesEncodedG2` typed binding; its bytes equal `vector_bytes(G2_GENERATOR_HEX)` as a byte slice
    * `[✅]`   `encode_g2_writes_the_identity_as_zero_bytes`
      * `[✅]`   Contract: the second group's identity → `Ok(EncodeG2SuccessReturn { bytes })` holding 128 zero bytes
      * `[✅]`   Collaborators: halo2curves, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_encode_g2_payload`, and `build_bn254_halo2curves_g2`
      * `[✅]`   Arrange: `build_encode_g2_payload` with the point override `build_bn254_halo2curves_g2` with the value `G2Affine::identity()`
      * `[✅]`   Act: `encode_g2(EncodeG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; the bytes of `success.bytes` equal `zero_bytes(128)` as a byte slice
    * `[✅]`   `encode_scalar_writes_the_largest_canonical_scalar`
      * `[✅]`   Contract: a scalar → `Ok(EncodeScalarSuccessReturn { bytes })` holding its 32 big-endian bytes in this concrete's `Bn254Halo2curvesEncodedScalar`, inside a `Secret`
      * `[✅]`   Collaborators: halo2curves' `to_repr`, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_encode_scalar_payload`, and `build_bn254_halo2curves_scalar`
      * `[✅]`   Arrange: `build_encode_scalar_payload` with the scalar override `build_bn254_halo2curves_scalar` with the value `-Fr::from(1u64)`
      * `[✅]`   Act: `encode_scalar(EncodeScalarParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.bytes.expose()` binds to a `Bn254Halo2curvesEncodedScalar` typed binding; its bytes equal `vector_bytes(GROUP_ORDER_MINUS_ONE_HEX)` as a byte slice
    * `[✅]`   `sample_from_uniform_bytes_rejects_a_wrong_length`
      * `[✅]`   Contract: `payload.uniform.expose().len() != 64` → `Err(SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual })`
      * `[✅]`   Collaborators: `Secret::expose`, real; fixtures `build_sample_uniform_scalar_payload` and `build_secret`
      * `[✅]`   Arrange: `build_sample_uniform_scalar_payload` with the uniform override `build_secret` holding `zero_bytes(63)`
      * `[✅]`   Act: `Bn254Halo2curvesScalar::sample_from_uniform_bytes(SampleUniformScalarParams, payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual: 63 })`, the whole expected error
    * `[✅]`   `sample_from_uniform_bytes_reads_the_input_as_a_big_endian_integer`
      * `[✅]`   Contract: a 64-byte uniform input → `Ok(SampleUniformScalarSuccessReturn { scalar })` holding the input read as one big-endian integer reduced modulo the group order, inside a `Secret`
      * `[✅]`   Collaborators: `Secret::expose` and halo2curves' wide reduction, real; fixtures `build_sample_uniform_scalar_payload` and `build_secret`
      * `[✅]`   Arrange: `build_sample_uniform_scalar_payload` with the uniform override `build_secret` holding `vector_bytes(UNIFORM_FIVE_HEX)`, whose last byte is five, so an input read as little-endian yields a different scalar
      * `[✅]`   Act: `Bn254Halo2curvesScalar::sample_from_uniform_bytes(SampleUniformScalarParams, payload)`
      * `[✅]`   Assert: the success arm is extracted with `expect`; `success.scalar.expose().value` equals `Fr::from(5u64)`
    * `[✅]`   `sample_from_uniform_bytes_reduces_the_group_order_to_zero`
      * `[✅]`   Contract: a 64-byte uniform input → `Ok(SampleUniformScalarSuccessReturn { scalar })` holding the input reduced modulo the group order, inside a `Secret`
      * `[✅]`   Collaborators: `Secret::expose` and halo2curves' wide reduction, real; fixtures `build_sample_uniform_scalar_payload` and `build_secret`
      * `[✅]`   Arrange: `build_sample_uniform_scalar_payload` with the uniform override `build_secret` holding `vector_bytes(UNIFORM_GROUP_ORDER_HEX)`, whose reduction is zero and so differs from the input
      * `[✅]`   Act: `Bn254Halo2curvesScalar::sample_from_uniform_bytes(SampleUniformScalarParams, payload)`
      * `[✅]`   Assert: the success arm is extracted with `expect`; `success.scalar.expose().value` equals `Fr::ZERO`
    * `[✅]`   `add_scalar_of_two_and_three_is_five`
      * `[✅]`   Contract: `payload.left` and `payload.right` → `Ok(AddScalarSuccessReturn { sum })` holding their sum modulo the group order
      * `[✅]`   Collaborators: halo2curves' `Fr` addition, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_add_scalar_payload`, and `build_bn254_halo2curves_scalar`
      * `[✅]`   Arrange: `build_add_scalar_payload` with the left override `build_bn254_halo2curves_scalar` with the value `Fr::from(2u64)` and the right override with the value `Fr::from(3u64)`
      * `[✅]`   Act: `add_scalar(AddScalarParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `Fr::from(5u64)`
    * `[✅]`   `add_scalar_reduces_modulo_the_group_order`
      * `[✅]`   Contract: `payload.left` and `payload.right` → `Ok(AddScalarSuccessReturn { sum })` holding their sum modulo the group order
      * `[✅]`   Collaborators: halo2curves' `Fr` addition, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_add_scalar_payload`, and `build_bn254_halo2curves_scalar`
      * `[✅]`   Arrange: `build_add_scalar_payload` with the left override `build_bn254_halo2curves_scalar` with the value `scalar_value(GROUP_ORDER_MINUS_ONE_HEX)` and the right override with the value `Fr::from(2u64)`, whose integer sum exceeds the group order
      * `[✅]`   Act: `add_scalar(AddScalarParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `Fr::from(1u64)`
    * `[✅]`   `mul_scalar_of_two_and_three_is_six`
      * `[✅]`   Contract: `payload.left` and `payload.right` → `Ok(MulScalarSuccessReturn { product })` holding their product modulo the group order
      * `[✅]`   Collaborators: halo2curves' `Fr` multiplication, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_mul_scalar_payload`, and `build_bn254_halo2curves_scalar`
      * `[✅]`   Arrange: `build_mul_scalar_payload` with the left override `build_bn254_halo2curves_scalar` with the value `Fr::from(2u64)` and the right override with the value `Fr::from(3u64)`
      * `[✅]`   Act: `mul_scalar(MulScalarParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `Fr::from(6u64)`
    * `[✅]`   `mul_scalar_reduces_modulo_the_group_order`
      * `[✅]`   Contract: `payload.left` and `payload.right` → `Ok(MulScalarSuccessReturn { product })` holding their product modulo the group order
      * `[✅]`   Collaborators: halo2curves' `Fr` multiplication, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_mul_scalar_payload`, and `build_bn254_halo2curves_scalar`
      * `[✅]`   Arrange: `build_mul_scalar_payload` with the left override `build_bn254_halo2curves_scalar` with the value `scalar_value(GROUP_ORDER_MINUS_ONE_HEX)` and the right override with the value `Fr::from(2u64)`, whose integer product exceeds the group order
      * `[✅]`   Act: `mul_scalar(MulScalarParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `scalar_value(GROUP_ORDER_MINUS_TWO_HEX)`
    * `[✅]`   `neg_scalar_of_one_is_the_group_order_minus_one`
      * `[✅]`   Contract: `payload.scalar` → `Ok(NegScalarSuccessReturn { negation })` holding the group order minus the scalar
      * `[✅]`   Collaborators: halo2curves' `Fr` negation, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_neg_scalar_payload`, and `build_bn254_halo2curves_scalar`
      * `[✅]`   Arrange: `build_neg_scalar_payload` with the scalar override `build_bn254_halo2curves_scalar` with the value `Fr::from(1u64)`
      * `[✅]`   Act: `neg_scalar(NegScalarParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.negation.value` equals `scalar_value(GROUP_ORDER_MINUS_ONE_HEX)`
    * `[✅]`   `neg_scalar_of_zero_is_zero`
      * `[✅]`   Contract: `payload.scalar` of zero → `Ok(NegScalarSuccessReturn { negation })` holding zero
      * `[✅]`   Collaborators: halo2curves' `Fr` negation, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_neg_scalar_payload`, and `build_bn254_halo2curves_scalar`
      * `[✅]`   Arrange: `build_neg_scalar_payload` with the scalar override `build_bn254_halo2curves_scalar` with the value `Fr::ZERO`
      * `[✅]`   Act: `neg_scalar(NegScalarParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.negation.value` equals `Fr::ZERO`
    * `[✅]`   `neg_g1_of_the_generator_is_the_negated_generator`
      * `[✅]`   Contract: `payload.point` → `Ok(NegG1SuccessReturn { negation })` holding `(x, p - y)` for a point `(x, y)`
      * `[✅]`   Collaborators: halo2curves' affine negation, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_neg_g1_payload`, and `build_bn254_halo2curves_g1`
      * `[✅]`   Arrange: `build_neg_g1_payload` with the point override `build_bn254_halo2curves_g1` at its default generator
      * `[✅]`   Act: `neg_g1(NegG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.negation.value` equals `eip_196_negated_generator()`
    * `[✅]`   `neg_g1_of_the_identity_is_the_identity`
      * `[✅]`   Contract: `payload.point` of the identity → `Ok(NegG1SuccessReturn { negation })` holding the identity
      * `[✅]`   Collaborators: halo2curves' affine negation, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_neg_g1_payload`, and `build_bn254_halo2curves_g1`
      * `[✅]`   Arrange: `build_neg_g1_payload` with the point override `build_bn254_halo2curves_g1` with the value `G1Affine::identity()`
      * `[✅]`   Act: `neg_g1(NegG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.negation.value` equals `G1Affine::identity()`
    * `[✅]`   `neg_g2_negates_the_y_coordinate_and_keeps_x`
      * `[✅]`   Contract: `payload.point` → `Ok(NegG2SuccessReturn { negation })` holding `(x, -y)` for a point `(x, y)`
      * `[✅]`   Collaborators: halo2curves' affine negation, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_neg_g2_payload`, and `build_bn254_halo2curves_g2`
      * `[✅]`   Arrange: `build_neg_g2_payload` with the point override `build_bn254_halo2curves_g2` at its default generator
      * `[✅]`   Act: `neg_g2(NegG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; the negation's `coordinates()` yields an `x` equal to the `x` of `eip_197_generator().coordinates()` and a `y` whose sum with the generator's `y` is zero under `is_zero()`
    * `[✅]`   `neg_g2_of_the_identity_is_the_identity`
      * `[✅]`   Contract: `payload.point` of the identity → `Ok(NegG2SuccessReturn { negation })` holding the identity
      * `[✅]`   Collaborators: halo2curves' affine negation, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_neg_g2_payload`, and `build_bn254_halo2curves_g2`
      * `[✅]`   Arrange: `build_neg_g2_payload` with the point override `build_bn254_halo2curves_g2` with the value `G2Affine::identity()`
      * `[✅]`   Act: `neg_g2(NegG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.negation.value` equals `G2Affine::identity()`
    * `[✅]`   `is_identity_g1_is_true_for_the_identity`
      * `[✅]`   Contract: `payload.point` → `Ok(IsIdentityG1SuccessReturn { is_identity })`, true exactly for the identity
      * `[✅]`   Collaborators: halo2curves, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_is_identity_g1_payload`, and `build_bn254_halo2curves_g1`
      * `[✅]`   Arrange: `build_is_identity_g1_payload` with the point override `build_bn254_halo2curves_g1` with the value `G1Affine::identity()`
      * `[✅]`   Act: `is_identity_g1(IsIdentityG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_identity` is true
    * `[✅]`   `is_identity_g1_is_false_for_the_generator`
      * `[✅]`   Contract: `payload.point` → `Ok(IsIdentityG1SuccessReturn { is_identity })`, true exactly for the identity
      * `[✅]`   Collaborators: halo2curves, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_is_identity_g1_payload`, and `build_bn254_halo2curves_g1`
      * `[✅]`   Arrange: `build_is_identity_g1_payload` with the point override `build_bn254_halo2curves_g1` at its default generator
      * `[✅]`   Act: `is_identity_g1(IsIdentityG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_identity` is false
    * `[✅]`   `is_identity_g2_is_true_for_the_identity`
      * `[✅]`   Contract: `payload.point` → `Ok(IsIdentityG2SuccessReturn { is_identity })`, true exactly for the identity
      * `[✅]`   Collaborators: halo2curves, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_is_identity_g2_payload`, and `build_bn254_halo2curves_g2`
      * `[✅]`   Arrange: `build_is_identity_g2_payload` with the point override `build_bn254_halo2curves_g2` with the value `G2Affine::identity()`
      * `[✅]`   Act: `is_identity_g2(IsIdentityG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_identity` is true
    * `[✅]`   `is_identity_g2_is_false_for_the_generator`
      * `[✅]`   Contract: `payload.point` → `Ok(IsIdentityG2SuccessReturn { is_identity })`, true exactly for the identity
      * `[✅]`   Collaborators: halo2curves, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_is_identity_g2_payload`, and `build_bn254_halo2curves_g2`
      * `[✅]`   Arrange: `build_is_identity_g2_payload` with the point override `build_bn254_halo2curves_g2` at its default generator
      * `[✅]`   Act: `is_identity_g2(IsIdentityG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_identity` is false
    * `[✅]`   `pairing_product_of_no_terms_is_the_target_group_identity`
      * `[✅]`   Contract: an empty `payload.terms` → `Ok(PairingProductSuccessReturn { product })` holding the target group's identity
      * `[✅]`   Collaborators: halo2curves' multi-Miller loop and final exponentiation, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing` and `build_pairing_product_payload`
      * `[✅]`   Arrange: `build_pairing_product_payload` at its default of no terms
      * `[✅]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `Gt::identity()`
    * `[✅]`   `pairing_product_of_the_generators_is_not_the_target_group_identity`
      * `[✅]`   Contract: a pairing of the generators → `Ok(PairingProductSuccessReturn { product })` holding a value other than the target group's identity
      * `[✅]`   Collaborators: halo2curves' multi-Miller loop and final exponentiation, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_pairing_product_payload`, and `build_pairing_product_term`
      * `[✅]`   Arrange: `build_pairing_product_payload` with the terms override holding one `build_pairing_product_term` at its default generators
      * `[✅]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` is not equal to `Gt::identity()`
    * `[✅]`   `pairing_product_is_bilinear`
      * `[✅]`   Contract: a term whose first-group element is twice the generator → `Ok(PairingProductSuccessReturn { product })` holding the generators' pairing raised to the power two
      * `[✅]`   Collaborators: halo2curves' multi-Miller loop and final exponentiation, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_pairing_product_payload`, `build_pairing_product_term`, and `build_bn254_halo2curves_g1`; the expectation is `definition_value_power`, the identifier's definition raised to a power in the fixtures
      * `[✅]`   Arrange: `build_pairing_product_payload` with the terms override holding one `build_pairing_product_term` with the first-group override `build_bn254_halo2curves_g1` with the value `eip_196_doubled_generator()` over the second-group generator
      * `[✅]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value.inner()` equals `&definition_value_power(eip_196_generator(), eip_197_generator(), 2)`
    * `[✅]`   `pairing_product_multiplies_its_terms`
      * `[✅]`   Contract: `payload.terms` → `Ok(PairingProductSuccessReturn { product })` holding the product of each term's pairing
      * `[✅]`   Collaborators: halo2curves' multi-Miller loop and final exponentiation, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_pairing_product_payload`, and `build_pairing_product_term`; the expectation is `definition_value_power`
      * `[✅]`   Arrange: `build_pairing_product_payload` with the terms override holding two `build_pairing_product_term` values at their default generators, so a product over one term differs from the expected square
      * `[✅]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value.inner()` equals `&definition_value_power(eip_196_generator(), eip_197_generator(), 2)`
    * `[✅]`   `pairing_product_of_a_pairing_and_its_first_group_negation_is_the_target_group_identity`
      * `[✅]`   Contract: `payload.terms` whose pairings multiply to the target group's identity → `Ok(PairingProductSuccessReturn { product })` holding the identity
      * `[✅]`   Collaborators: halo2curves' multi-Miller loop and final exponentiation, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_pairing_product_payload`, `build_pairing_product_term`, and `build_bn254_halo2curves_g1`
      * `[✅]`   Arrange: `build_pairing_product_payload` with the terms override holding two `build_pairing_product_term` values: the generators, and the first-group override `build_bn254_halo2curves_g1` with the value `eip_196_negated_generator()` over the second-group generator
      * `[✅]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `Gt::identity()`
    * `[✅]`   `pairing_product_of_the_generators_equals_the_definition`
      * `[✅]`   Contract: a pairing of the generators → `Ok(PairingProductSuccessReturn { product })` holding the Miller loop's value raised to the exact exponent `(p^12 - 1) / r`, the value `TargetGroupEncodingIdentifier::Bn254V1` defines, with no correction applied
      * `[✅]`   Collaborators: halo2curves' multi-Miller loop and final exponentiation, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_pairing_product_payload`, and `build_pairing_product_term`; the expectation is `definition_value`, the Miller loop over the generators raised to `definition_exponent()` built by `num-bigint`, independent of the library's final exponentiation
      * `[✅]`   Arrange: `build_pairing_product_payload` with the terms override holding one `build_pairing_product_term` at its default generators
      * `[✅]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value.inner()` equals `&definition_value(eip_196_generator(), eip_197_generator())`
    * `[✅]`   `encode_gt_writes_the_target_group_identity_with_c0_c0_c0_first`
      * `[✅]`   Contract: a target-group value → `Ok(EncodeGtSuccessReturn { bytes })` holding its twelve coefficients in tower order, `c0.c0.c0` first, each 32 bytes big-endian, inside a `Secret`
      * `[✅]`   Collaborators: halo2curves' `to_repr`, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_encode_gt_payload`, and `build_bn254_halo2curves_gt`; the expectation is `gt_identity_encoding()`
      * `[✅]`   Arrange: `build_encode_gt_payload` with the value override `build_bn254_halo2curves_gt` with the value `Gt::identity()`
      * `[✅]`   Act: `encode_gt(EncodeGtParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.bytes.expose()` binds to a `Bn254Halo2curvesEncodedGt` typed binding; its bytes equal `gt_identity_encoding()` as a byte slice
    * `[✅]`   `encode_gt_writes_the_definition_value_in_tower_order`
      * `[✅]`   Contract: a target-group value → `Ok(EncodeGtSuccessReturn { bytes })` holding its twelve coefficients in tower order, `c0.c0.c0` through `c1.c2.c1`, each 32 bytes big-endian, inside a `Secret`
      * `[✅]`   Collaborators: halo2curves' `to_repr`, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_encode_gt_payload`, and `build_bn254_halo2curves_gt`; the expectation is `definition_encoding()`, the serialization in the fixtures of the Miller loop over the generators raised to the exact exponent
      * `[✅]`   Arrange: `build_encode_gt_payload` at its default value, the pairing of the generators, whose twelve coefficients are distinct, so any exchange of two positions changes the bytes
      * `[✅]`   Act: `encode_gt(EncodeGtParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; the bytes of `success.bytes.expose()` equal `definition_encoding()` as a byte slice
    * `[✅]`   `scalar_field_order_is_the_group_order`
      * `[✅]`   Contract: any call → `Ok(ScalarFieldOrderSuccessReturn { bytes })` holding the group order's 32 big-endian bytes
      * `[✅]`   Collaborators: halo2curves' `Fr` negation and `to_repr`, run for real as the vendor; fixture `build_bn254_halo2curves_pairing`; `ScalarFieldOrderParams` and `ScalarFieldOrderPayload` by their production values
      * `[✅]`   Arrange: `build_bn254_halo2curves_pairing()`
      * `[✅]`   Act: `scalar_field_order(ScalarFieldOrderParams, ScalarFieldOrderPayload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.bytes` equals `vector_bytes(GROUP_ORDER_HEX)`
    * `[✅]`   `g1_outside_subgroup_encoding_is_absent_where_the_cofactor_is_one`
      * `[✅]`   Contract: any call → `Ok(G1OutsideSubgroupEncodingSuccessReturn { bytes: None })`
      * `[✅]`   Collaborators: none called; fixture `build_bn254_halo2curves_pairing`; `G1OutsideSubgroupEncodingParams` and `G1OutsideSubgroupEncodingPayload` by their production values
      * `[✅]`   Arrange: `build_bn254_halo2curves_pairing()`
      * `[✅]`   Act: `g1_outside_subgroup_encoding(G1OutsideSubgroupEncodingParams, G1OutsideSubgroupEncodingPayload)`
      * `[✅]`   Assert: the success arm is extracted with `expect`; `success.bytes.is_none()` is true
    * `[✅]`   `g2_outside_subgroup_encoding_encodes_an_on_curve_point_outside_the_subgroup`
      * `[✅]`   Contract: any call → `Ok(G2OutsideSubgroupEncodingSuccessReturn { bytes })` holding the precompile encoding of an on-curve point outside the prime-order subgroup, in this concrete's `Bn254Halo2curvesEncodedG2`
      * `[✅]`   Collaborators: halo2curves' curve and torsion checks, run for real as the vendor; fixture `build_bn254_halo2curves_pairing`; `G2OutsideSubgroupEncodingParams` and `G2OutsideSubgroupEncodingPayload` by their production values; the point read back by `g2_point_from_encoding`
      * `[✅]`   Arrange: `build_bn254_halo2curves_pairing()`
      * `[✅]`   Act: `g2_outside_subgroup_encoding(G2OutsideSubgroupEncodingParams, G2OutsideSubgroupEncodingPayload)`
      * `[✅]`   Assert: the success arm is extracted with `expect`; `success.bytes` binds to a `Bn254Halo2curvesEncodedG2` typed binding; `g2_point_from_encoding` reads a point from its bytes, which is `Some` because the point is on the curve, and that point's `to_curve().is_torsion_free()` is false
    * `[✅]`   `g2_outside_subgroup_encoding_has_a_real_first_coordinate_and_the_lesser_root`
      * `[✅]`   Contract: any call → the encoded point's first coordinate is `(c0, 0)` with the least `c0` of an on-curve point outside the subgroup, and its `y` is the lesser of the two roots, compared by `y.c1` and then `y.c0`
      * `[✅]`   Collaborators: halo2curves' curve and torsion checks, run for real as the vendor; fixture `build_bn254_halo2curves_pairing`; `G2OutsideSubgroupEncodingParams` and `G2OutsideSubgroupEncodingPayload` by their production values; the expectations are `g2_outside_subgroup_bytes()`, the ascending search in the fixtures, and `base_field_modulus()`
      * `[✅]`   Arrange: `build_bn254_halo2curves_pairing()`
      * `[✅]`   Act: `g2_outside_subgroup_encoding(G2OutsideSubgroupEncodingParams, G2OutsideSubgroupEncodingPayload)`
      * `[✅]`   Assert: the success arm is extracted with `expect`; the encoding's `x.c1` quarter, its first 32 bytes, is all zero; its `x.c0` quarter, its second 32 bytes, equals the `x.c0` quarter of `g2_outside_subgroup_bytes()`; the pair `y.c1`, `y.c0` read from its last 64 bytes as `BigUint` values is not greater than the pair of their negations, each `base_field_modulus()` minus the coefficient, reduced to zero where the coefficient is zero

  * `[✅]`   `construction`
    * `[✅]`   `Bn254Halo2curvesPairing::try_new` is the concrete's only producer, and its only caller is the pairing factory, which obtains the declaration through `pairing/lookup_pairing_declaration` and completes `pairing/admit_pairing_composition` before calling it
    * `[✅]`   A group element or scalar is produced only by the adapter's generators, arithmetic, and decoders, or by the scalar's sampling bound; a target-group value only by `pairing_product`; an encoded value only by the adapter's encoders and reference methods; no consumer constructs one from library values

  * `[✅]`   `adapters/pairing/src/bn254_halo2curves/mod.rs`
    * `[✅]`   Module declarations: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, `pub(crate) mod provides;`, and `#[cfg(test)] mod test;`
    * `[✅]`   Imports: from `crate::factory::provides` every params, payload, return, success-return, and error type of the three traits and the sampling bound, `IPairingAdapter`, `IPairingArithmetic`, `IPairingReference`, and `ISampleUniformScalar`; `core::hint::black_box`; `core::iter::successors`; `domain::{Secret, SecretConstructorParams}`; `halo2curves::bn256::{Bn256, Fq, Fq2, Fr, G1Affine, G2Affine, Gt}`; `halo2curves::ff::{Field, FromUniformBytes, PrimeField}`; `halo2curves::group::{Curve, Group, cofactor::CofactorGroup, prime::PrimeCurveAffine}`; `halo2curves::msm::msm_best`; `halo2curves::pairing::{MillerLoopResult, MultiMillerLoop}`; `halo2curves::{Coordinates, CurveAffine}`; from `interface` every type it declares; `zeroize::{Zeroize, ZeroizeOnDrop}`
    * `[✅]`   `impl Bn254Halo2curvesPairing` holding `pub fn try_new(params: Bn254Halo2curvesPairingConstructorParams) -> Bn254Halo2curvesPairingTryNewReturn` realizing the interaction spec's construct branch; `DECLARATION` lives in `interface.rs`, the interface owning the real declaration
    * `[✅]`   `impl IPairingAdapter for Bn254Halo2curvesPairing` with `type Scalar = Bn254Halo2curvesScalar;`, `type G1 = Bn254Halo2curvesG1;`, `type G2 = Bn254Halo2curvesG2;`, `type EncodedG1 = Bn254Halo2curvesEncodedG1;`, `type EncodedG2 = Bn254Halo2curvesEncodedG2;`, and `type EncodedScalar = Bn254Halo2curvesEncodedScalar;`, every method realizing its branches in the interaction spec
    * `[✅]`   `impl IPairingArithmetic for Bn254Halo2curvesPairing` with `type Gt = Bn254Halo2curvesGt;` and `type EncodedGt = Bn254Halo2curvesEncodedGt;`, every method realizing its branches in the interaction spec, `pairing_product` applying no correction
    * `[✅]`   `impl IPairingReference for Bn254Halo2curvesPairing`, every method realizing its branches in the interaction spec, the search by `find_map` with the `SearchExhausted` return by `let … else`
    * `[✅]`   `impl AsRef<[u8]>` for `Bn254Halo2curvesEncodedG1`, `Bn254Halo2curvesEncodedG2`, `Bn254Halo2curvesEncodedScalar`, and `Bn254Halo2curvesEncodedGt`, each returning `&self.bytes`; `impl Zeroize` for `Bn254Halo2curvesEncodedScalar` and `Bn254Halo2curvesEncodedGt`, each calling `self.bytes.zeroize()`; no implementation returns a mutable reference to the bytes
    * `[✅]`   `impl Zeroize` and `impl Drop` for `Bn254Halo2curvesScalar`, setting `value` to `Fr::ZERO` and calling `black_box(&self.value)`, and for `Bn254Halo2curvesG1`, `Bn254Halo2curvesG2`, and `Bn254Halo2curvesGt`, each setting `value` to its type's identity and calling `black_box(&self.value)`; `impl ZeroizeOnDrop for Bn254Halo2curvesScalar {}`, marking the drop behavior the sampling bound requires
    * `[✅]`   `impl ISampleUniformScalar for Bn254Halo2curvesScalar` with `const UNIFORM_BYTES_LENGTH: usize = 64;` and `sample_from_uniform_bytes` realizing its branches in the interaction spec, the scalar moved into a `Secret` by `let Ok(scalar) = Secret::try_new(SecretConstructorParams { value });`
    * `[✅]`   No `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`; neither the order nor a point is written as a literal

  * `[✅]`   `adapters/pairing/src/bn254_halo2curves/provides.rs`
    * `[✅]`   `pub(crate) use super::interface::*;` and `#[cfg(test)] pub(crate) use super::mock::*;`, exposing every item of the concrete's interface to the crate and, in the crate's own test build, the concrete's builders, fixtures, and helpers, and nothing beyond them

  * `[✅]`   `directionality`
    * `[✅]`   `bn254_halo2curves` depends on the `factory` module's surface through `crate::factory::provides`, on `domain`, on `zeroize`, and on `halo2curves`; it depends on no other concrete, and `pairing/lookup_pairing_declaration` reaches its interface-held declaration through the family's private path, the selection and composition admissions consume the corrected `PairingConcrete` identifier and declared set, and the factory completes those admissions before constructing this member; its increments to `factory/interface.rs` and `factory/mock.rs` introduce no dependency outside the pairing family; among repository crates the crate depends on `crates/domain` alone at runtime
    * `[✅]`   `pairing/factory` constructs this concrete, the family form's recorded cycle, and proves its target-group encoding, order, and outside-the-subgroup encodings equal the arkworks concrete's

  * `[✅]`   `requirements`
    * `[✅]`   `adapters/pairing/Cargo.toml` carries the `halo2curves` dependency, and `halo2curves` is named nowhere in the crate outside the halo2curves concretes
    * `[✅]`   This concrete returns its own distinct fixed-width encoded types for the first group, the second group, the scalar, and the target group through the family's associated types
    * `[✅]`   `cargo check --all-targets --all-features`, `cargo fmt --check`, and `cargo deny check` complete without error; `cargo clippy --all-targets --all-features` reports nothing beyond the library target's unused-item warnings for the pairing concretes, which `pairing/factory` constructs
    * `[✅]`   Every test in `bn254_halo2curves/test.rs` passes (CR-04, CR-08, CR-09, and CR-10 on BN254 over halo2curves; CR-05 for the sampled scalar; CR-07 for the clearing; CR-11 for the order)
    * `[✅]`   The declaration compile-time assertion compiles, and `pairing_product_of_the_generators_equals_the_definition` passes (CR-10, the target-group value fixed by the identifier's definition, executed in the test)
    * `[✅]`   `pairing_concretes_declares_only_real_members` and `every_declared_identifier_spells_a_distinct_name` pass, proving the declaration carries `PairingConcrete::Bn254Halo2curves` rather than text and `PAIRING_CONCRETES` contains both real BN254 members with distinct spellings and no mock member
    * `[✅]`   `target_group_encoding_identifiers_declares_only_real_values` and `no_value_identifier_declares_a_mock_only_variant` pass, proving `TARGET_GROUP_ENCODING_IDENTIFIERS` remains real-only
    * `[✅]`   `bn254_halo2curves_declared_selection_builds_without_configuration` passes, proving the new declared-selection arm returns the real selection with no configuration, and this concrete carries no settings module, settings node, or settings table
    * `[✅]`   The factory obtains this member's declaration through `pairing/lookup_pairing_declaration` and completes `pairing/admit_pairing_composition` before constructing the concrete
    * `[✅]`   The `ZeroizeOnDrop` compile-time assertion compiles and `bn254_halo2curves_scalar_zeroize_clears_its_value` passes (CR-07)
    * `[✅]`   Code outside `adapters/pairing` naming `Bn254Halo2curvesPairing` or anything under `bn254_halo2curves` fails to compile; the crate's public surface is the `factory` module's `provides`

* `[✅]`   `pairing/bls12_381_arkworks` **BLS12-381 pairing concrete on arkworks implementing the pairing family's generic interface, arithmetic trait, and reference trait over the EIP-2537 encodings with subgroup checks on every input and the `Bls12381V1` target-group value; the increment adding the `Bls12381Arkworks` member variant, the four new value variants and their spellings, both declared-set entries, and the declared-selection builder arm**

  * `[✅]`   `objective`
    * `[✅]`   Problem: BLS12-381 is the primary verifier form on Base through the EIP-2537 precompiles, so the pairing family needs a BLS12-381 concrete whose encodings and checks match those precompiles exactly (CR-10)
    * `[✅]`   Problem: the KEM, the envelope, and the delivery proof consume the pairing family's arithmetic trait through whichever concrete the factory resolves, so this concrete implements it; the bytes the KEM hands the KDF are the identifier's target-group value, and `ark-ec`'s BLS12 final exponentiation returns the Miller loop's value raised to three times the exact exponent `(p^12 - 1) / r`, the Hayashida–Hayasaka–Teruya chain in its source, so without correction the library's reduced pairing is a different element of the target group than the identifier's (CR-04; CR-07; CR-08; CR-09; CR-10; `docs/research/cryptography.md`'s pairing adapter capability declaration)
    * `[✅]`   Problem: on BLS12-381 both source groups have a cofactor above one, and EIP-2537's addition precompiles perform no subgroup check, so a contract's own check is proven against an on-curve point outside the subgroup in each group; this concrete returns the group order and both encodings under the family's rule from its own library (CR-10 on chain; CR-11; the dependency map's `pairing/bn254_arkworks` row; the Pairing adapters and key derivation milestone's exit)
    * `[✅]`   Functional: the concrete implements `IPairingAdapter` over arkworks' BLS12-381, with its own scalar, group-element, and encoded types over the library's elements as the associated types
    * `[✅]`   Functional: it encodes and decodes a base field element as EIP-2537's 64 bytes, sixteen zero bytes followed by the 48-byte big-endian integer, a first-group point as 128 bytes of `x` then `y`, a second-group point as 256 bytes of `x.c0`, `x.c1`, `y.c0`, `y.c1`, the point at infinity as all zero bytes, and a scalar as 32 big-endian bytes
    * `[✅]`   Functional: decoding rejects a wrong length, a nonzero padding byte, a field element at or above the modulus, a point off the curve, and a point outside the prime-order subgroup in either group, since BLS12-381's first group has a nontrivial cofactor as well as its second
    * `[✅]`   Functional: its scalar type implements the family's sampling bound, reading the 64 uniform bytes as one big-endian integer reduced modulo the group order
    * `[✅]`   Functional: its declaration carries `PairingConcrete::Bls12381Arkworks`, the real values this member provides — the BLS12-381 curve, both-group arithmetic at the verifier, the EIP-2537 precompile encoding, and `TargetGroupEncodingIdentifier::Bls12381V1` — its adapter version, and the interface version it implements, the typed member identifier with no duplicate text name; read through `pairing/lookup_pairing_declaration` without an instance, never from a constructed adapter, so the proof factory selects the verifier form with direct second-group equations
    * `[✅]`   Functional: `Bls12381ArkworksPairing` implements `IPairingArithmetic` with every method's behavior as `pairing/bn254_arkworks` states it for the trait; `Bls12381V1` is the optimal ate pairing as the CFRG pairing-friendly-curves draft fixes it, over the EIP-2537 generators, on the tower `Fp2 = Fp[u] / (u^2 + 1)`, `Fp6 = Fp2[v] / (v^3 - (u + 1))`, `Fp12 = Fp6[w] / (w^2 - v)`, the Miller loop's value raised to the exact exponent `(p^12 - 1) / r`
    * `[✅]`   Functional: `pairing_product` returns the identifier's exact value: the library's reduced pairing multiplied in the target group by `reduced_pairing_correction`, the inverse of three in the scalar field, computed once in `try_new` by Fermat
    * `[✅]`   Functional: the target-group encoding writes the twelve `Fq` coefficients of the `Fq12` in the tower order `c0.c0.c0`, `c0.c0.c1`, `c0.c1.c0`, `c0.c1.c1`, `c0.c2.c0`, `c0.c2.c1`, `c1.c0.c0`, `c1.c0.c1`, `c1.c1.c0`, `c1.c1.c1`, `c1.c2.c0`, `c1.c2.c1`, each 48 bytes big-endian, BLS12-381's base-field byte width, 576 bytes in all, without the 16-byte padding EIP-2537 adds to source-group coordinates
    * `[✅]`   Functional: the concrete's tests execute the identifier's definition, the library's Miller loop over the two generators raised to the integer `(p^12 - 1) / r` built by `num-bigint` from the library's moduli and serialized in the tower order, and assert the pairing of the base points the CFRG draft publishes in its test-vector appendix, requiring `pairing_product` then `encode_gt` over the generators to yield those bytes
    * `[✅]`   Functional: `Bls12381ArkworksScalar`, `Bls12381ArkworksG1`, `Bls12381ArkworksG2`, and `Bls12381ArkworksGt` zeroize their value through `Zeroize` and on drop
    * `[✅]`   Functional: the concrete returns BLS12-381's group order as 32 big-endian bytes and each group's point under the family's rule, encoded as `encode_g1` and `encode_g2` encode any point, each refused by its own decoder as outside the subgroup
    * `[✅]`   Non-functional: `ark-bls12-381` is named only inside `adapters/pairing/src/bls12_381_arkworks`, and `ark-ec` and `ark-ff` only inside the arkworks concretes; no order and no point is written as a literal outside a test

  * `[✅]`   `role`
    * `[✅]`   Adapter: a further concrete of the pairing family, implementing the generic interface, the sampling bound, the arithmetic trait, and the reference trait `pairing/bn254_arkworks` authors in the `factory` module, declaring the identifier it authors, and the increment adding the `Bls12381Arkworks` member variant, the new value variants `PairingCurve::Bls12381`, `VerifierGroupArithmetic::BothGroups`, `PrecompileEncoding::Eip2537`, and `TargetGroupEncodingIdentifier::Bls12381V1` with their spellings `bls12381`, `both_groups`, `eip2537`, and `bls12381_v1`, the `PAIRING_CONCRETES` and `TARGET_GROUP_ENCODING_IDENTIFIERS` entries, and the `build_pairing_concrete` arm
    * `[✅]`   Adds to `factory/interface.rs` the value variants `PairingCurve::Bls12381` with spelling `bls12381`, `VerifierGroupArithmetic::BothGroups` with spelling `both_groups`, `PrecompileEncoding::Eip2537` with spelling `eip2537`, and `TargetGroupEncodingIdentifier::Bls12381V1` with spelling `bls12381_v1` and its entry in `TARGET_GROUP_ENCODING_IDENTIFIERS` beside `Bn254V1`, and the member `PairingConcrete::Bls12381Arkworks` with spelling `bls12381_arkworks` and its entry in `PAIRING_CONCRETES`, and to `factory/mock.rs` the `build_pairing_concrete` arm returning this real selection without configuration, every mock-only value variant absent
    * `[✅]`   Contributes its module line to `lib.rs` and its dependency line to the crate manifest
    * `[✅]`   Does not select between concretes; `pairing/factory` constructs a concrete by the composition's request and `harness-crypto/benchmark` records each concrete's measurements
    * `[✅]`   Does not name, import, or compare against another concrete; `pairing/factory`'s integration test proves agreement with the halo2curves concrete
    * `[✅]`   Does not read the correction from configuration; it is a property of this library under this identifier and is computed in `try_new`
    * `[✅]`   Does not carry a settings module, settings node, or settings table; the concrete has no settable value, so the real selection carries no settings
    * `[✅]`   Does not carry a commit

  * `[✅]`   `module`
    * `[✅]`   Bounded context: the private `bls12_381_arkworks` module of `adapters/pairing`, holding the adapter over arkworks' BLS12-381 with its correction field, its scalar, group-element, target-group, and encoded types over the library's elements, its constructor params, its implementations of the three traits, its interface-held declaration constant carrying `PairingConcrete::Bls12381Arkworks`, its definition and published-vector tests, its zeroization, and the builder defaults for its owned types; and the node's identifier and declared-set increments in the `factory` module
    * `[✅]`   The node's files: the `adapters/pairing/Cargo.toml` dependency line, the `adapters/pairing/src/lib.rs` module line, the identifier variants and declared-set entries in `adapters/pairing/src/factory/interface.rs`, the declared-selection arm in `adapters/pairing/src/factory/mock.rs`, and the `bls12_381_arkworks` module's `interface.rs`, `interaction.spec.md`, `mock.rs`, `test.rs`, `mod.rs`, and `provides.rs`
    * `[✅]`   Outside: the rest of the family's traits and declaration, every other concrete, and the family's declaration lookup, selection admission, composition admission, and factory construction

  * `[✅]`   `deps`
    * `[✅]`   The `factory` module's surface, through `crate::factory::provides`: `IPairingAdapter`, `IPairingArithmetic`, `IPairingReference`, `ISampleUniformScalar`, `PairingDeclaration` with its typed `concrete` field and its real value enums `PairingCurve` with `Bls12381` spelled `bls12381`, `VerifierGroupArithmetic` with `BothGroups` spelled `both_groups`, `PrecompileEncoding` with `Eip2537` spelled `eip2537`, and `TargetGroupEncodingIdentifier` with `Bls12381V1` spelled `bls12381_v1`, `PairingConcrete` with `Bls12381Arkworks` and its `name()` mapping, `PAIRING_CONCRETES` with all three real members now present, `TARGET_GROUP_ENCODING_IDENTIFIERS` with both real encoding identifiers, `PAIRING_INTERFACE_VERSION`, every params, payload, return, success-return, error type, and builder of the three traits and the sampling bound, and, under the `mocks` feature, `build_pairing_concrete` with this member's no-configuration arm
    * `[✅]`   `domain`, runtime: `Secret` and `SecretConstructorParams`; `build_secret` and `SecretConstructorParamsOverrides` through the `mocks` feature
    * `[✅]`   `zeroize` `1.9.0`, runtime: the `Zeroize` and `ZeroizeOnDrop` traits and their implementations for arkworks' affine points and `PairingOutput`
    * `[✅]`   `ark-bls12-381` `0.6.0`, external crate, MIT OR Apache-2.0, runtime dependency named only in `bls12_381_arkworks`, default features; supplies the curve and the pairing
    * `[✅]`   `ark-ec` `0.6.0` and `ark-ff` `0.6.0`, the crate's runtime dependencies; supply the group and field arithmetic
    * `[✅]`   `hex` `0.4.3` and `num-bigint` `0.4.8`, the crate's dev-dependencies, for the test vectors and the integer exponent in `mock.rs`'s test-only fixtures and the root comparison in `test.rs`
    * `[✅]`   `core::convert::Infallible`, standard library, the constructor's error arm
    * `[✅]`   Reverse dependency: `pairing/factory`

  * `[✅]`   `context_slice`
    * `[✅]`   From `ark-bls12-381`: `Bls12_381`, `Fq`, `Fq2` with its public fields `c0` and `c1` and `Fq2::new(c0, c1)`, `Fq12`, `Fr`, `G1Affine`, `G1Projective`, `G2Affine`, and `G2Projective`
    * `[✅]`   From `ark-ec`: `AffineRepr` for `generator()`, `xy()`, and `is_zero()`; the short-Weierstrass affine `identity()`, `new_unchecked(x, y)`, `is_on_curve()`, `is_in_correct_subgroup_assuming_on_curve()`, and `get_point_from_x_unchecked(x, greatest)`; the affine `+` and `* Fr` producing projective points; unary `-` on the affine point; `CurveGroup::into_affine`; `VariableBaseMSM::msm_unchecked(bases, scalars)`; `pairing::Pairing::multi_pairing` over iterators of borrowed affine points, returning `PairingOutput<Bls12_381>` with public field `0`; `Pairing::multi_miller_loop` over the same iterators, returning `MillerLoopOutput<Bls12_381>` with public field `0`, the unreduced `Fq12`; `Mul<Fr> for PairingOutput<Bls12_381>`; `Pairing::pairing(p, q)`; `Zeroize` for the affine point, which zeroizes `x`, `y`, and `infinity`, so a zeroized point is `new_unchecked(0, 0)`, and for `PairingOutput`, which zeroizes its `Fq12`
    * `[✅]`   From `ark-ff`: `Fr`'s `+`, `*`, and unary `-` modulo the group order; `Fr::from(u64)` and `Fq::from(u64)`; `Field::ONE`; `Field::pow(&self, exp: impl AsRef<[u64]>)` on `Fr` and on `Fq12`; `PrimeField::MODULUS`, the `BigInt<4>` of `Fr` and the `BigInt<6>` of `Fq`; `BigInteger::sub_with_borrow(&mut self, other: &Self) -> bool` and `BigInt::<4>::from(u64)`; `From<BigInt<N>> for num_bigint::BigUint`; the public fields `c0` and `c1` of the degree-two and degree-twelve extensions and `c0`, `c1`, and `c2` of the degree-six extension; `PrimeField::from_be_bytes_mod_order(&[u8])` and `into_bigint()` on `Fq` and on `Fr`, the `BigInt<6>` of `Fq` ordering as the integer; `BigInteger::to_bytes_be()`, 48 bytes for `Fq` and 32 for `Fr`; `Zero::is_zero()` on `Fq`, on `Fq12`, and on `PairingOutput`; unary `-` on `Fq` and on `Fq2`
    * `[✅]`   From `num-bigint`, in `mock.rs`'s test-only fixtures and in `test.rs`: `BigUint::from(u32)`, `BigUint::from_bytes_be(&[u8])`, `BigUint::pow(&self, u32)`, `Sub`, `Div`, and `Rem` between `BigUint`s, and `to_u64_digits()`
    * `[✅]`   From `domain`: `Secret::try_new(SecretConstructorParams { value })` returning `Result<Secret<T>, Infallible>`, and `Secret::expose(&self) -> &T`

  * `[✅]`   `adapters/pairing/Cargo.toml`
    * `[✅]`   `[dependencies]` holds `ark-bls12-381 = "0.6.0"`, default features

  * `[✅]`   `adapters/pairing/src/lib.rs`
    * `[✅]`   The barrel holds `mod bls12_381_arkworks;`

  * `[✅]`   `adapters/pairing/src/factory/interface.rs`
    * `[✅]`   `PairingCurve` holds the variant `Bls12381` with `name()` spelling `bls12381`; `VerifierGroupArithmetic` holds the variant `BothGroups` with spelling `both_groups`; `PrecompileEncoding` holds the variant `Eip2537` with spelling `eip2537`; `TargetGroupEncodingIdentifier` holds the variant `Bls12381V1` with `name()` spelling `bls12381_v1` and `TARGET_GROUP_ENCODING_IDENTIFIERS` gains it beside `Bn254V1`; `PairingConcrete` holds the member `Bls12381Arkworks` with `name()` spelling `bls12381_arkworks` and `PAIRING_CONCRETES` gains it; every entry stays real and later members are added by their own increments

  * `[✅]`   `adapters/pairing/src/bls12_381_arkworks/interface.rs`
    * `[✅]`   Imports `core::convert::Infallible`; `PairingDeclaration`, `PairingCurve`, `PairingConcrete`, `VerifierGroupArithmetic`, `PrecompileEncoding`, `TargetGroupEncodingIdentifier`, and `PAIRING_INTERFACE_VERSION` from `crate::factory::provides`; declares nothing beyond the items below
    * `[✅]`   `Bls12381ArkworksPairing`, a struct with no derives and one field, `pub(super) reduced_pairing_correction: ark_bls12_381::Fr`, with the doc comment "The inverse of three in the scalar field, the multiple by which the library's reduced pairing exceeds the identifier's exact value."
    * `[✅]`   `Bls12381ArkworksPairingConstructorParams`, a unit struct; `Bls12381ArkworksPairingTryNewReturn`, the alias `Result<Bls12381ArkworksPairing, Infallible>`, the error arm uninhabited because the adapter takes no configuration
    * `[✅]`   `Bls12381ArkworksPairing::DECLARATION`, the inherent constant `pub const DECLARATION: PairingDeclaration = PairingDeclaration { concrete: PairingConcrete::Bls12381Arkworks, curve: PairingCurve::Bls12381, verifier_group_arithmetic: VerifierGroupArithmetic::BothGroups, precompile_encoding: PrecompileEncoding::Eip2537, target_group_encoding: TargetGroupEncodingIdentifier::Bls12381V1, adapter_version: 1, interface_version: PAIRING_INTERFACE_VERSION }`, read by `pairing/lookup_pairing_declaration` before any instance exists; the settings-free concrete carries no settings module, and the real selection carries no settings
    * `[✅]`   `Bls12381ArkworksScalar`, `#[derive(Clone)]`, with one field `pub(super) value: ark_bls12_381::Fr`; `Bls12381ArkworksG1`, `#[derive(Clone)]`, with one field `pub(super) value: ark_bls12_381::G1Affine`; `Bls12381ArkworksG2`, `#[derive(Clone)]`, with one field `pub(super) value: ark_bls12_381::G2Affine`
    * `[✅]`   `Bls12381ArkworksGt`, a struct with no derives and one field, `pub(super) value: ark_ec::pairing::PairingOutput<ark_bls12_381::Bls12_381>`, the target-group value
    * `[✅]`   `Bls12381ArkworksEncodedG1` and `Bls12381ArkworksEncodedG2`, `pub struct`s with `#[derive(Clone, PartialEq, Eq)]` and one field each, `pub(super) bytes: [u8; 128]` and `pub(super) bytes: [u8; 256]`; `Bls12381ArkworksEncodedScalar`, a `pub struct` with no derives and one field `pub(super) bytes: [u8; 32]`; `Bls12381ArkworksEncodedGt`, a `pub struct` with no derives and one field `pub(super) bytes: [u8; 576]`; the scalar and target-group encodings are returned only inside a `Secret`, and no encoded type has a public constructor or mutable byte access

  * `[✅]`   `adapters/pairing/src/bls12_381_arkworks/interaction.spec.md`
    * `[✅]`   The title `` # `bls12_381_arkworks` — interaction spec `` and one opening sentence stating the file as the branch contract for the `bls12_381_arkworks` module of the `pairing` crate, the pairing adapter over arkworks' BLS12-381 encoding group elements as EIP-2537 precompile input, each branch stating condition, decision, dependency call, and the exact return outcome; then one `##` section per constructor, constant, and method, each headed by its full signature and holding a table with the columns `Branch`, `Condition`, `Decision`, `Dependency call`, and `Outcome`, then `Ordering and edges`
    * `[✅]`   `try_new`: one branch, construct; condition any params; decision none; dependency calls, in order: `Fr::from(3u64)` as `multiple`; `let mut exponent = Fr::MODULUS;` then `let _ = exponent.sub_with_borrow(&BigInt::from(2u64));`, the group order minus two; `multiple.pow(exponent)` as `reduced_pairing_correction`; outcome `Ok(Bls12381ArkworksPairing { reduced_pairing_correction })`; the error arm has no branch, `Infallible` being uninhabited
    * `[✅]`   `DECLARATION`: the inherent constant `PairingDeclaration { concrete: PairingConcrete::Bls12381Arkworks, curve: PairingCurve::Bls12381, verifier_group_arithmetic: VerifierGroupArithmetic::BothGroups, precompile_encoding: PrecompileEncoding::Eip2537, target_group_encoding: TargetGroupEncodingIdentifier::Bls12381V1, adapter_version: 1, interface_version: PAIRING_INTERFACE_VERSION }`, read by `pairing/lookup_pairing_declaration` before any instance exists
    * `[✅]`   `g1_generator`, `g2_generator`: one branch each, generated; condition any; decision none; dependency call `G1Affine::generator()` or `G2Affine::generator()`; outcome `Ok(G1GeneratorSuccessReturn { point })` or `Ok(G2GeneratorSuccessReturn { point })`, the generator in the owned group type
    * `[✅]`   `add_g1`, `add_g2`: one branch each, summed; dependency call the affine `+` of `payload.left.value` and `payload.right.value`, then `into_affine`; outcome `Ok(AddG1SuccessReturn { sum })` or `Ok(AddG2SuccessReturn { sum })`
    * `[✅]`   `mul_g1`, `mul_g2`: one branch each, multiplied; dependency call the affine `payload.point.value` `*` the scalar's `Fr`, then `into_affine`; outcome `Ok(MulG1SuccessReturn { product })` or `Ok(MulG2SuccessReturn { product })`; the payload, holding the scalar, drops at the end of the call and the scalar is zeroized
    * `[✅]`   `msm_g1`, `msm_g2`: one branch each, summed; dependency call split the terms into a `Vec` of affine bases and a `Vec<Fr>` of the same length, then `G1Projective::msm_unchecked` or `G2Projective::msm_unchecked` over them, then `into_affine`; outcome `Ok(MsmG1SuccessReturn { sum })` or `Ok(MsmG2SuccessReturn { sum })`; the `Vec<Fr>` is zeroized after the call; an empty term list yields the identity
    * `[✅]`   `pairing_product_is_one`: one branch, evaluated; dependency call `Bls12_381::multi_pairing` over the terms' first-group elements and second-group elements in term order; outcome `Ok(PairingProductIsOneSuccessReturn { is_one })`, `is_one` being the output's `is_zero()`, the identity of the target group in arkworks' additive notation; an empty term list yields `is_one: true`
    * `[✅]`   `decode_g1`, its section opening with the coordinate rule, a coordinate being EIP-2537's 64 bytes, the first 16 zero and the last 48 the field element big-endian, and a nonzero byte among the first 16 or 48 bytes at or above the base field modulus being non-canonical; its branches in order: wrong length, condition `payload.len() != 128`, decision the length check, no dependency call, outcome `Err(DecodeG1ErrorReturn::WrongLength { expected: 128, actual: payload.len() })`; non-canonical coordinate, condition either 64-byte coordinate has a nonzero byte among its first 16 or its last 48 read by `Fq::from_be_bytes_mod_order` do not re-encode through `into_bigint().to_bytes_be()` to the same 48 bytes, decision the padding check and the re-encoding comparison per coordinate, outcome `Err(DecodeG1ErrorReturn::NonCanonicalCoordinate)`; identity, condition both coordinates zero, decision the zero check, outcome `Ok(DecodeG1SuccessReturn { point })` holding `G1Affine::identity()`; off the curve, condition `G1Affine::new_unchecked(x, y).is_on_curve()` is false, outcome `Err(DecodeG1ErrorReturn::NotOnCurve)`; outside the subgroup, condition `is_in_correct_subgroup_assuming_on_curve()` is false, outcome `Err(DecodeG1ErrorReturn::NotInSubgroup)`; valid, condition every check passes, outcome `Ok(DecodeG1SuccessReturn { point })`; the section states that BLS12-381's first group has a nontrivial cofactor, so the subgroup branch is reachable here as well as in the second group
    * `[✅]`   `decode_g2`: the same branches in the same order over 256 bytes read as `x.c0`, `x.c1`, `y.c0`, `y.c1`, each a 64-byte coordinate, assembled by `Fq2::new(c0, c1)`, with `DecodeG2ErrorReturn` and `expected: 256`; the identity is all four coordinates zero
    * `[✅]`   `decode_scalar`, its branches in order: wrong length, condition `payload.len() != 32`, outcome `Err(DecodeScalarErrorReturn::WrongLength { expected: 32, actual: payload.len() })`; non-canonical, condition the bytes read by `Fr::from_be_bytes_mod_order` do not re-encode to the same 32 bytes, decision the re-encoding comparison, outcome `Err(DecodeScalarErrorReturn::NonCanonical)`; valid, outcome `Ok(DecodeScalarSuccessReturn { scalar })`; the section states that EIP-2537's MSM accepts any 256-bit scalar while this decoder, which produces an owned scalar, admits only the canonical ones
    * `[✅]`   `encode_g1`, `encode_g2`: one branch each, encoded; decision the identity check; dependency call `xy()`, then `into_bigint().to_bytes_be()` per coordinate, each written after its 16 zero bytes to its fixed 64-byte position; outcome `Ok(EncodeG1SuccessReturn { bytes })` holding `Bls12381ArkworksEncodedG1`, 128 zero bytes for the identity and otherwise `x` then `y`, or `Ok(EncodeG2SuccessReturn { bytes })` holding `Bls12381ArkworksEncodedG2`, 256 zero bytes for the identity and otherwise `x.c0`, `x.c1`, `y.c0`, `y.c1`, each 16 zero bytes followed by its 48 big-endian bytes
    * `[✅]`   `encode_scalar`: one branch, encoded; dependency call `into_bigint().to_bytes_be()`; outcome `Ok(EncodeScalarSuccessReturn { bytes })`, the scalar's 32 big-endian bytes in `Bls12381ArkworksEncodedScalar` moved into a `Secret`
    * `[✅]`   `UNIFORM_BYTES_LENGTH`: `64`, twice the byte width of the group order, so the reduction's bias from uniform is below two to the minus two hundred fifty
    * `[✅]`   `sample_from_uniform_bytes`, its branches in order: wrong length, condition `payload.uniform.expose().len() != 64`, dependency call `Secret::expose`, outcome `Err(SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual })`; sampled, condition the length is 64, dependency call `Fr::from_be_bytes_mod_order` over the exposed bytes, outcome `Ok(SampleUniformScalarSuccessReturn { scalar })`, the scalar moved into a `Secret`; the payload's `Secret` zeroizes the input when it drops
    * `[✅]`   `add_scalar`, `mul_scalar`: one branch each; dependency call `Fr`'s `+` or `*` over `payload.left.value` and `payload.right.value`; outcome `Ok(AddScalarSuccessReturn { sum })` or `Ok(MulScalarSuccessReturn { product })`, modulo the group order in the owned scalar type; the payload's scalars are zeroized as it drops
    * `[✅]`   `neg_scalar`: one branch; dependency call `Fr`'s unary `-` over `payload.scalar.value`; outcome `Ok(NegScalarSuccessReturn { negation })`, the group order minus the scalar, and zero for zero
    * `[✅]`   `neg_g1`, `neg_g2`: one branch each; dependency call the affine point's unary `-` over `payload.point.value`; outcome `Ok(NegG1SuccessReturn { negation })` or `Ok(NegG2SuccessReturn { negation })`, `(x, p - y)` for a point `(x, y)` and the identity for the identity
    * `[✅]`   `is_identity_g1`, `is_identity_g2`: one branch each; dependency call `AffineRepr::is_zero()` on `payload.point.value`; outcome `Ok(IsIdentityG1SuccessReturn { is_identity })` or `Ok(IsIdentityG2SuccessReturn { is_identity })`, `true` exactly for the identity
    * `[✅]`   `pairing_product`: one branch, evaluated; dependency call split the terms into a `Vec<G1Affine>` and a `Vec<G2Affine>` in term order, then `Bls12_381::multi_pairing(&g1s, &g2s)`, then the `PairingOutput` `*` `self.reduced_pairing_correction`, the exponentiation that brings the library's cubed value to the identifier's exact value, then `zeroize` on both vectors; outcome `Ok(PairingProductSuccessReturn { product })` holding the corrected `PairingOutput` in the owned target-group type; an empty term list yields the target group's identity, which the exponentiation preserves
    * `[✅]`   `encode_gt`: one branch, encoded; dependency call `into_bigint().to_bytes_be()` on each of the twelve `Fq` coefficients of `payload.value.value.0` in the tower order `c0.c0.c0`, `c0.c0.c1`, `c0.c1.c0`, `c0.c1.c1`, `c0.c2.c0`, `c0.c2.c1`, `c1.c0.c0`, `c1.c0.c1`, `c1.c1.c0`, `c1.c1.c1`, `c1.c2.c0`, `c1.c2.c1`, each written to its fixed 48-byte position in a 576-byte buffer; outcome `Ok(EncodeGtSuccessReturn { bytes })`, a `Secret<Bls12381ArkworksEncodedGt>` built from the complete buffer with no variable-width intermediate; the target group's identity encodes as 47 zero bytes, `01`, and 528 zero bytes
    * `[✅]`   `scalar_field_order`: one branch, read; dependency call `Fr::MODULUS.to_bytes_be()`; outcome `Ok(ScalarFieldOrderSuccessReturn { bytes })`, the group order's 32 big-endian bytes; the error arm has no branch
    * `[✅]`   `g1_outside_subgroup_encoding`, exhausted: condition `(1u64..).find_map(…)` over the search below returns `None`; outcome `Err(G1OutsideSubgroupEncodingErrorReturn::SearchExhausted)`; the section states that the first group's cofactor exceeds one, so the search ends among the least values of `x`, that no input takes this branch, and that it has no unit test
    * `[✅]`   `g1_outside_subgroup_encoding`, found: decision, per `x` in ascending order, `G1Affine::get_point_from_x_unchecked(Fq::from(x), false)`, kept when `is_in_correct_subgroup_assuming_on_curve()` is false, its `y` read through `xy()` inside the search; then, with `negated = -y`, the point is kept when `y.into_bigint()` is not greater than `negated.into_bigint()` and replaced by its affine negation otherwise; dependency call `self.encode_g1(EncodeG1Params, EncodeG1Payload { point: Bls12381ArkworksG1 { value } })`, unpacked irrefutably; outcome `Ok(G1OutsideSubgroupEncodingSuccessReturn { bytes: Some(bytes) })`, the `Bls12381ArkworksEncodedG1` `encode_g1` returns
    * `[✅]`   `g2_outside_subgroup_encoding`, exhausted: as the first group's, with `G2OutsideSubgroupEncodingErrorReturn::SearchExhausted`
    * `[✅]`   `g2_outside_subgroup_encoding`, found: decision, per `c0` in ascending order, `G2Affine::get_point_from_x_unchecked(Fq2::new(Fq::from(c0), Fq::from(0u64)), false)`, kept when outside the subgroup, its `y` read through `xy()`; then, with `negated = -y`, the point is kept when `(y.c1.into_bigint(), y.c0.into_bigint())` is not greater than `(negated.c1.into_bigint(), negated.c0.into_bigint())` and replaced by its affine negation otherwise; dependency call `self.encode_g2(EncodeG2Params, EncodeG2Payload { point: Bls12381ArkworksG2 { value } })`, unpacked irrefutably; outcome `Ok(G2OutsideSubgroupEncodingSuccessReturn { bytes })`, the `Bls12381ArkworksEncodedG2` `encode_g2` returns
    * `[✅]`   `Ordering and edges`, a bulleted section: every decoder checks in the stated order, length, then canonicality, then identity, then curve, then subgroup, and slices the payload only after the length check; an empty `msm` term list yields the identity, and an empty `pairing_product_is_one` term list yields `is_one: true`, as EIP-2537 does for empty input; `Bls12381ArkworksScalar`, `Bls12381ArkworksG1`, `Bls12381ArkworksG2`, and `Bls12381ArkworksGt` each zeroize their `value` through their `Zeroize` implementation and on drop, so every clone a consumer places in a payload is zeroized when the payload drops; each outside-the-subgroup search is ascending from one and stops at the first on-curve point outside the subgroup, the choice between a point and its negation follows the search and precedes the encoding, and the same call always returns the same bytes; `params` carries no control and is not read in any method, and no reference method reads its payload

  * `[✅]`   `adapters/pairing/src/bls12_381_arkworks/mock.rs`
    * `[✅]`   The module-level `#![allow(dead_code, clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::too_many_arguments)]`; imports `super::interface::{Bls12381ArkworksEncodedGt, Bls12381ArkworksEncodedScalar, Bls12381ArkworksG1, Bls12381ArkworksG2, Bls12381ArkworksGt, Bls12381ArkworksPairing, Bls12381ArkworksPairingConstructorParams, Bls12381ArkworksScalar}`, `ark_bls12_381::{Bls12_381, Fq, Fq2, Fq12, Fr, G1Affine, G2Affine}`, `ark_ec::{AffineRepr, pairing::{Pairing, PairingOutput}}`, and `ark_ff::{BigInteger, Field, PrimeField}`; and, `#[cfg(test)]`, `ark_ec::CurveGroup`, `hex::decode`, `num_bigint::BigUint`, and `zeroize::ZeroizeOnDrop`
    * `[✅]`   The defaults of the concrete's owned types, each stated in its builder, with no `Default` implementation on any of them: the scalar `Fr::from(1u64)`; the first-group point `G1Affine::generator()`; the second-group point `G2Affine::generator()`; the target-group value `Bls12_381::pairing(G1Affine::generator(), G2Affine::generator())`, a non-identity target-group value
    * `[✅]`   The builders for the concrete's owned types, each overrides struct `#[derive(Default)]` with one `Option` field and each omitted value taking the default stated above: `Bls12381ArkworksScalarOverrides` with `pub value: Option<Fr>` and `build_bls12_381_arkworks_scalar`; `Bls12381ArkworksG1Overrides` with `pub value: Option<G1Affine>` and `build_bls12_381_arkworks_g1`; `Bls12381ArkworksG2Overrides` with `pub value: Option<G2Affine>` and `build_bls12_381_arkworks_g2`; `Bls12381ArkworksGtOverrides` with `pub value: Option<PairingOutput<Bls12_381>>` and `build_bls12_381_arkworks_gt`; no corruptions type and no invalidator, since none of these arrives as untrusted data
    * `[✅]`   `build_bls12_381_arkworks_pairing() -> Bls12381ArkworksPairing`, the real instance from `Bls12381ArkworksPairing::try_new(Bls12381ArkworksPairingConstructorParams)` through `let Ok(pairing) = …;`; the constructor params are fieldless, so the builder takes no overrides
    * `[✅]`   The builders for the concrete's encoded scalar and encoded target-group types, each overrides struct `#[derive(Default)]` with one `Option` field: `Bls12381ArkworksEncodedScalarOverrides` with `pub bytes: Option<[u8; 32]>` and `build_bls12_381_arkworks_encoded_scalar(overrides: Bls12381ArkworksEncodedScalarOverrides) -> Bls12381ArkworksEncodedScalar`, the bytes defaulting to `[1u8; 32]`; `Bls12381ArkworksEncodedGtOverrides` with `pub bytes: Option<[u8; 576]>` and `build_bls12_381_arkworks_encoded_gt`, the bytes defaulting to `[1u8; 576]`; the defaults are nonzero so a zeroized value differs from a built one; no corruptions type and no invalidator, since neither arrives as untrusted data
    * `[✅]`   The test fixtures, each `#[cfg(test)]`, since they use the crate's dev-dependencies; constants, each a `const … : &str` of hex: `BASE_FIELD_MODULUS_PADDED_HEX`, 16 zero bytes then `1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab`; `GROUP_ORDER_HEX` `73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001`; `GROUP_ORDER_MINUS_ONE_HEX` `73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000000`; `GROUP_ORDER_MINUS_TWO_HEX` `73eda753299d7d483339d80809a1d80553bda402fffe5bfefffffffeffffffff`; `G1_GENERATOR_HEX`, 16 zero bytes, `17f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb`, 16 zero bytes, `08b3f481e3aaa0f1a09e30ed741d8ae4fcf5e095d5d00af600db18cb2c04b3edd03cc744a2888ae40caa232946c5e7e1`; `G1_GENERATOR_OFF_CURVE_HEX`, the same with the last byte `e1` replaced by `e2`; `G2_GENERATOR_HEX`, 16 zero bytes and `024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8`, 16 zero bytes and `13e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e`, 16 zero bytes and `0ce5d527727d6e118cc9cdc6da2e351aadfd9baa8cbdd3a76d429a695160d12c923ac9cc3baca289e193548608b82801`, 16 zero bytes and `0606c4a02ea734cc32acd2b02bc28b99cb3e287e85a763af267492ab572e99ab3f370d275cec1da1aaa9075ff05f79be`; `G2_GENERATOR_OFF_CURVE_HEX`, the same with the last byte `be` replaced by `bf`; `NEG_G1_GENERATOR_HEX`, 16 zero bytes and `17f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb`, then 16 zero bytes and `114d1d6855d545a8aa7d76c8cf2e21f267816aef1db507c96655b9d5caac42364e6f38ba0ecb751bad54dcd6b939c2ca`; `G1_NONZERO_PADDING_HEX`, `G1_GENERATOR_HEX` with its first byte `00` replaced by `01`; `G1_X_AT_MODULUS_HEX`, the 64 bytes of `BASE_FIELD_MODULUS_PADDED_HEX` then the last 64 bytes of `G1_GENERATOR_HEX`; `G2_NONZERO_PADDING_HEX`, `G2_GENERATOR_HEX` with its first byte `00` replaced by `01`; `G2_X_C0_AT_MODULUS_HEX`, the 64 bytes of `BASE_FIELD_MODULUS_PADDED_HEX` then the last 192 bytes of `G2_GENERATOR_HEX`; `UNIFORM_GROUP_ORDER_HEX`, 32 zero bytes then the 32 bytes of `GROUP_ORDER_HEX`, the group order as a 64-byte big-endian integer; `UNIFORM_FIVE_HEX`, 63 zero bytes and `05`
    * `[✅]`   The test-fixture constant `CFRG_GENERATOR_PAIRING_HEX`, `#[cfg(test)]`, the 576-byte encoding of the pairing of the base points `BP` and `BP'`, which are the EIP-2537 generators, as Appendix B of `draft-irtf-cfrg-pairing-friendly-curves-11` publishes it, its twelve coefficients `e_0` through `e_11` concatenated in order, the draft's inductive octet rule being the tower order the objective states; the coefficients, each 96 hex characters: `11619b45f61edfe3b47a15fac19442526ff489dcda25e59121d9931438907dfd448299a87dde3a649bdba96e84d54558`, `153ce14a76a53e205ba8f275ef1137c56a566f638b52d34ba3bf3bf22f277d70f76316218c0dfd583a394b8448d2be7f`, `095668fb4a02fe930ed44767834c915b283b1c6ca98c047bd4c272e9ac3f3ba6ff0b05a93e59c71fba77bce995f04692`, `16deedaa683124fe7260085184d88f7d036b86f53bb5b7f1fc5e248814782065413e7d958d17960109ea006b2afdeb5f`, `09c92cf02f3cd3d2f9d34bc44eee0dd50314ed44ca5d30ce6a9ec0539be7a86b121edc61839ccc908c4bdde256cd6048`, `111061f398efc2a97ff825b04d21089e24fd8b93a47e41e60eae7e9b2a38d54fa4dedced0811c34ce528781ab9e929c7`, `01ecfcf31c86257ab00b4709c33f1c9c4e007659dd5ffc4a735192167ce197058cfb4c94225e7f1b6c26ad9ba68f63bc`, `08890726743a1f94a8193a166800b7787744a8ad8e2f9365db76863e894b7a11d83f90d873567e9d645ccf725b32d26f`, `0e61c752414ca5dfd258e9606bac08daec29b3e2c57062669556954fb227d3f1260eedf25446a086b0844bcd43646c10`, `0fe63f185f56dd29150fc498bbeea78969e7e783043620db33f75a05a0a2ce5c442beaff9da195ff15164c00ab66bdde`, `10900338a92ed0b47af211636f7cfdec717b7ee43900eee9b5fc24f0000c5874d4801372db478987691c566a8c474978`, `1454814f3085f0e6602247671bc408bbce2007201536818c901dbd4d2095dd86c1ec8b888e59611f60a301af7776be3d`
    * `[✅]`   The test-fixture helpers, each `#[cfg(test)]`: `vector_bytes`, `zero_bytes`, `scalar_value`, and `requires_zeroize_on_drop`; `base_field_modulus() -> BigUint`, `BigUint::from_bytes_be(&vector_bytes(BASE_FIELD_MODULUS_PADDED_HEX))`; `gt_identity_encoding() -> Vec<u8>`, `vec![0u8; 576]` with index `47` set to `1`; `g1_point_from_encoding(bytes: &[u8]) -> G1Affine`, `G1Affine::new_unchecked` over the two 64-byte coordinates of `bytes`, each its last 48 bytes read by `Fq::from_be_bytes_mod_order`; `g2_point_from_encoding(bytes: &[u8]) -> G2Affine`, `G2Affine::new_unchecked(Fq2::new(x_c0, x_c1), Fq2::new(y_c0, y_c1))` over the four 64-byte coordinates of `bytes`, read in order as `x_c0`, `x_c1`, `y_c0`, `y_c1`, each its last 48 bytes read by `Fq::from_be_bytes_mod_order`; `eip_2537_g1_generator() -> G1Affine`, `g1_point_from_encoding(&vector_bytes(G1_GENERATOR_HEX))`, the generator EIP-2537 publishes; `eip_2537_negated_g1_generator() -> G1Affine`, `g1_point_from_encoding(&vector_bytes(NEG_G1_GENERATOR_HEX))`; `eip_2537_g2_generator() -> G2Affine`, `g2_point_from_encoding(&vector_bytes(G2_GENERATOR_HEX))`; `g1_outside_subgroup_bytes() -> Vec<u8>`, the least on-curve point outside the subgroup, `(1u64..).find_map(|c| G1Affine::get_point_from_x_unchecked(Fq::from(c), false).filter(|point| !point.is_in_correct_subgroup_assuming_on_curve()))` unpacked with "an on-curve point outside the subgroup exists", its `xy()` unpacked with "the point has coordinates", encoded as 16 zero bytes, `x.into_bigint().to_bytes_be()`, 16 zero bytes, and `y.into_bigint().to_bytes_be()`; `g2_outside_subgroup_bytes() -> Vec<u8>`, the least on-curve point outside the subgroup, `(1u64..).find_map(|c0| G2Affine::get_point_from_x_unchecked(Fq2::new(Fq::from(c0), Fq::from(0u64)), false).filter(|point| !point.is_in_correct_subgroup_assuming_on_curve()))` unpacked with "an on-curve point outside the subgroup exists", its `xy()` unpacked with "the point has coordinates", encoded as `x.c0`, `x.c1`, `y.c0`, `y.c1`, each 16 zero bytes followed by `into_bigint().to_bytes_be()`; `definition_exponent() -> Vec<u64>`, `let p = BigUint::from(Fq::MODULUS); let r = BigUint::from(Fr::MODULUS); ((p.pow(12) - BigUint::from(1u32)) / r).to_u64_digits()`; `definition_value(g1: G1Affine, g2: G2Affine) -> Fq12`, `Bls12_381::multi_miller_loop([g1], [g2]).0.pow(definition_exponent())`, the identifier's definition of the pairing value; `definition_value_power(g1: G1Affine, g2: G2Affine, exponent: u64) -> Fq12`, `definition_value(g1, g2).pow([exponent])`; `eip_2537_doubled_g1_generator() -> G1Affine`, `(eip_2537_g1_generator() + eip_2537_g1_generator()).into_affine()`
    * `[✅]`   No mock function: the concrete is built as a real instance and owns no free function

  * `[✅]`   `adapters/pairing/src/bls12_381_arkworks/test.rs`
    * `[✅]`   Module-level `#![allow(clippy::expect_used)]`; imports from `super::provides` the concrete, its owned types, and the mock's builders, constants, and helpers each block uses; from `crate::factory::provides` the params, payloads, errors, trait names, member and value identifiers, declared sets, and builders each block uses; `HashSet` from `std::collections`; and the `ark_bls12_381`, `ark_ec`, `ark_ff`, `num_bigint::BigUint`, and `zeroize::Zeroize` names each block uses
    * `[✅]`   Compile-time assertion over `Bls12381ArkworksPairing::DECLARATION`
      * `[✅]`   Contract: the inherent constant is `PairingDeclaration { concrete: PairingConcrete::Bls12381Arkworks, curve: PairingCurve::Bls12381, verifier_group_arithmetic: VerifierGroupArithmetic::BothGroups, precompile_encoding: PrecompileEncoding::Eip2537, target_group_encoding: TargetGroupEncodingIdentifier::Bls12381V1, adapter_version: 1, interface_version: PAIRING_INTERFACE_VERSION }`, read through `pairing/lookup_pairing_declaration` without an instance
      * `[✅]`   Arrange: the type alone
      * `[✅]`   Act: a module-level `const _: () = assert!(…);` reading `Bls12381ArkworksPairing::DECLARATION`, `concrete` and each enum field tested with `matches!` and each version with `==`, no text-name assertion
      * `[✅]`   Assert: the module compiles only if every field holds
    * `[✅]`   `pairing_concretes_declares_only_real_members`
      * `[✅]`   Contract: the declared set holds the real members alone → `PAIRING_CONCRETES` contains `PairingConcrete::Bn254Arkworks`, `PairingConcrete::Bn254Halo2curves`, and `PairingConcrete::Bls12381Arkworks` and no `Mock(_)` member
      * `[✅]`   Collaborators: none; the declared set is read as a constant, no instance is built
      * `[✅]`   Arrange: none; the constant alone
      * `[✅]`   Act: reading `PAIRING_CONCRETES`
      * `[✅]`   Assert: every member is `PairingConcrete::Bn254Arkworks`, `PairingConcrete::Bn254Halo2curves`, or `PairingConcrete::Bls12381Arkworks` and none is `Mock(_)`
    * `[✅]`   `target_group_encoding_identifiers_declares_only_real_values`
      * `[✅]`   Contract: the declared set holds the real provided values alone → `TARGET_GROUP_ENCODING_IDENTIFIERS` contains `TargetGroupEncodingIdentifier::Bn254V1` and `TargetGroupEncodingIdentifier::Bls12381V1` and no `Mock` member
      * `[✅]`   Collaborators: none; the declared set is read as a constant, no instance is built
      * `[✅]`   Arrange: none; the constant alone
      * `[✅]`   Act: reading `TARGET_GROUP_ENCODING_IDENTIFIERS`
      * `[✅]`   Assert: every member is `TargetGroupEncodingIdentifier::Bn254V1` or `TargetGroupEncodingIdentifier::Bls12381V1` and none is `Mock`
    * `[✅]`   `every_declared_identifier_spells_a_distinct_name`
      * `[✅]`   Contract: each declared member has one total spelling → the `name()` spellings of the members of `PAIRING_CONCRETES` are pairwise distinct, and so are those of `TARGET_GROUP_ENCODING_IDENTIFIERS`
      * `[✅]`   Collaborators: none; the declared sets are read as constants, no instance is built
      * `[✅]`   Arrange: none; the constants alone
      * `[✅]`   Act: collecting each member's `name()` into a `HashSet` per declared set
      * `[✅]`   Assert: each `HashSet`'s length equals its declared set's length
    * `[✅]`   `no_value_identifier_declares_a_mock_only_variant`
      * `[✅]`   Contract: a value identifier holds real provided values alone → `PairingCurve`, `VerifierGroupArithmetic`, `PrecompileEncoding`, and `TargetGroupEncodingIdentifier` have no mock-only variant
      * `[✅]`   Collaborators: none; the value identifiers are read as types, no instance is built
      * `[✅]`   Arrange: none; the types alone
      * `[✅]`   Act: reading `core::mem::variant_count` on each value identifier
      * `[✅]`   Assert: each variant count equals its real variants' count, so no mock-only variant exists
    * `[✅]`   `bls12_381_arkworks_declared_selection_builds_without_configuration`
      * `[✅]`   Contract: the declared-selection builder's `Bls12381Arkworks` arm returns the real selection with no configuration → `build_pairing_concrete` over `PairingConcrete::Bls12381Arkworks` yields that member, the fieldless `PairingConcreteOverrides` carrying nothing
      * `[✅]`   Collaborators: none; the builder builds no adapter
      * `[✅]`   Arrange: `PairingConcreteOverrides` default
      * `[✅]`   Act: `build_pairing_concrete(PairingConcrete::Bls12381Arkworks, PairingConcreteOverrides::default())`
      * `[✅]`   Assert: the result is `PairingConcrete::Bls12381Arkworks`
    * `[✅]`   `bls12_381_arkworks_scalar_is_zeroize_on_drop`
      * `[✅]`   Contract: `Bls12381ArkworksScalar` implements `ZeroizeOnDrop`, the marker the sampling bound requires
      * `[✅]`   Collaborators: none
      * `[✅]`   Arrange: the type alone
      * `[✅]`   Act: `requires_zeroize_on_drop::<Bls12381ArkworksScalar>()`
      * `[✅]`   Assert: the block compiles only if the type implements `ZeroizeOnDrop`
    * `[✅]`   `bls12_381_arkworks_scalar_zeroize_clears_its_value`
      * `[✅]`   Contract: `Zeroize::zeroize` on a living scalar → its `value` is zero
      * `[✅]`   Collaborators: arkworks' `Fr` zeroization, run for real as the vendor; fixture `build_bls12_381_arkworks_scalar`
      * `[✅]`   Arrange: `build_bls12_381_arkworks_scalar` with the value override `Fr::from(5u64)`, so a scalar left unchanged differs from the expected zero
      * `[✅]`   Act: `scalar.zeroize()`
      * `[✅]`   Assert: `scalar.value` equals `Fr::from(0u64)`
    * `[✅]`   `bls12_381_arkworks_g1_zeroize_clears_its_value`
      * `[✅]`   Contract: `Zeroize::zeroize` on a living first-group element → its `value` is the point `new_unchecked(0, 0)`
      * `[✅]`   Collaborators: arkworks' affine zeroization, run for real as the vendor; fixture `build_bls12_381_arkworks_g1`
      * `[✅]`   Arrange: `build_bls12_381_arkworks_g1` at its default generator, which differs from the expected point
      * `[✅]`   Act: `g1.zeroize()`
      * `[✅]`   Assert: `g1.value` equals `G1Affine::new_unchecked(Fq::from(0u64), Fq::from(0u64))`
    * `[✅]`   `bls12_381_arkworks_g2_zeroize_clears_its_value`
      * `[✅]`   Contract: `Zeroize::zeroize` on a living second-group element → its `value` is the point `new_unchecked(0, 0)`
      * `[✅]`   Collaborators: arkworks' affine zeroization, run for real as the vendor; fixture `build_bls12_381_arkworks_g2`
      * `[✅]`   Arrange: `build_bls12_381_arkworks_g2` at its default generator, which differs from the expected point
      * `[✅]`   Act: `g2.zeroize()`
      * `[✅]`   Assert: `g2.value` equals `G2Affine::new_unchecked(Fq2::new(Fq::from(0u64), Fq::from(0u64)), Fq2::new(Fq::from(0u64), Fq::from(0u64)))`
    * `[✅]`   `bls12_381_arkworks_gt_zeroize_clears_its_value`
      * `[✅]`   Contract: `Zeroize::zeroize` on a living target-group value → every coefficient of its `Fq12` is zero
      * `[✅]`   Collaborators: arkworks' `PairingOutput` zeroization, run for real as the vendor; fixture `build_bls12_381_arkworks_gt`
      * `[✅]`   Arrange: `build_bls12_381_arkworks_gt` at its default pairing of the generators, whose `Fq12` is nonzero
      * `[✅]`   Act: `gt.zeroize()`
      * `[✅]`   Assert: `gt.value.0.is_zero()` is true
    * `[✅]`   `bls12_381_arkworks_encoded_scalar_zeroize_clears_its_bytes`
      * `[✅]`   Contract: `Zeroize::zeroize` on a living encoded scalar → every byte is zero
      * `[✅]`   Collaborators: none; fixture `build_bls12_381_arkworks_encoded_scalar`
      * `[✅]`   Arrange: `build_bls12_381_arkworks_encoded_scalar` at its nonzero default
      * `[✅]`   Act: `encoded.zeroize()`
      * `[✅]`   Assert: the bytes of `encoded` equal `zero_bytes(32)` as a byte slice
    * `[✅]`   `bls12_381_arkworks_encoded_gt_zeroize_clears_its_bytes`
      * `[✅]`   Contract: `Zeroize::zeroize` on a living encoded target-group value → every byte is zero
      * `[✅]`   Collaborators: none; fixture `build_bls12_381_arkworks_encoded_gt`
      * `[✅]`   Arrange: `build_bls12_381_arkworks_encoded_gt` at its nonzero default
      * `[✅]`   Act: `encoded.zeroize()`
      * `[✅]`   Assert: the bytes of `encoded` equal `zero_bytes(576)` as a byte slice
    * `[✅]`   `reduced_pairing_correction_inverts_three`
      * `[✅]`   Contract: any params → `Ok(Bls12381ArkworksPairing)` whose `reduced_pairing_correction` is the inverse of three in the scalar field
      * `[✅]`   Collaborators: arkworks' `Fr` arithmetic, run for real as the vendor; `Bls12381ArkworksPairingConstructorParams` by its production value
      * `[✅]`   Arrange: `Bls12381ArkworksPairingConstructorParams` by its production value
      * `[✅]`   Act: `Bls12381ArkworksPairing::try_new(Bls12381ArkworksPairingConstructorParams)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(pairing) = …;`; `pairing.reduced_pairing_correction * Fr::from(3u64)` equals `Fr::from(1u64)`
    * `[✅]`   `uniform_bytes_length_is_twice_the_group_order_width`
      * `[✅]`   Contract: `UNIFORM_BYTES_LENGTH` is twice the byte width of the group order
      * `[✅]`   Collaborators: none
      * `[✅]`   Arrange: the type alone
      * `[✅]`   Act: reading `<Bls12381ArkworksScalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH`
      * `[✅]`   Assert: the constant equals the literal 64 written in the assertion
    * `[✅]`   `g1_generator_returns_the_eip_2537_generator`
      * `[✅]`   Contract: any call → `Ok(G1GeneratorSuccessReturn { point })` holding the first group's generator
      * `[✅]`   Collaborators: arkworks, run for real as the vendor; fixture `build_bls12_381_arkworks_pairing`; `G1GeneratorParams` and `G1GeneratorPayload` by their production values
      * `[✅]`   Arrange: `build_bls12_381_arkworks_pairing()`
      * `[✅]`   Act: `g1_generator(G1GeneratorParams, G1GeneratorPayload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.point.value` equals `eip_2537_g1_generator()`
    * `[✅]`   `g2_generator_returns_the_eip_2537_generator`
      * `[✅]`   Contract: any call → `Ok(G2GeneratorSuccessReturn { point })` holding the second group's generator
      * `[✅]`   Collaborators: arkworks, run for real as the vendor; fixture `build_bls12_381_arkworks_pairing`; `G2GeneratorParams` and `G2GeneratorPayload` by their production values
      * `[✅]`   Arrange: `build_bls12_381_arkworks_pairing()`
      * `[✅]`   Act: `g2_generator(G2GeneratorParams, G2GeneratorPayload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.point.value` equals `eip_2537_g2_generator()`
    * `[✅]`   `add_g1_of_a_point_and_its_negation_is_the_identity`
      * `[✅]`   Contract: `payload.left` and `payload.right` → `Ok(AddG1SuccessReturn { sum })` holding their sum
      * `[✅]`   Collaborators: arkworks' affine addition, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_add_g1_payload`, and `build_bls12_381_arkworks_g1`
      * `[✅]`   Arrange: `build_add_g1_payload` with the left override `build_bls12_381_arkworks_g1` at its default generator and the right override `build_bls12_381_arkworks_g1` with the value `eip_2537_negated_g1_generator()`, so a sum that returns either input differs from the identity
      * `[✅]`   Act: `add_g1(AddG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G1Affine::identity()`
    * `[✅]`   `add_g1_of_the_identity_and_a_point_is_the_point`
      * `[✅]`   Contract: `payload.left` and `payload.right` → `Ok(AddG1SuccessReturn { sum })` holding their sum
      * `[✅]`   Collaborators: arkworks' affine addition, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_add_g1_payload`, and `build_bls12_381_arkworks_g1`
      * `[✅]`   Arrange: `build_add_g1_payload` with the left override `build_bls12_381_arkworks_g1` with the value `G1Affine::identity()` and the right override `build_bls12_381_arkworks_g1` at its default generator, so a sum that returns the left input differs from the generator
      * `[✅]`   Act: `add_g1(AddG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `eip_2537_g1_generator()`
    * `[✅]`   `add_g2_of_a_point_and_its_negation_is_the_identity`
      * `[✅]`   Contract: `payload.left` and `payload.right` → `Ok(AddG2SuccessReturn { sum })` holding their sum
      * `[✅]`   Collaborators: arkworks' affine addition, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_add_g2_payload`, and `build_bls12_381_arkworks_g2`
      * `[✅]`   Arrange: `build_add_g2_payload` with the left override `build_bls12_381_arkworks_g2` at its default generator and the right override `build_bls12_381_arkworks_g2` with the value `-eip_2537_g2_generator()`, so a sum that returns either input differs from the identity
      * `[✅]`   Act: `add_g2(AddG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G2Affine::identity()`
    * `[✅]`   `add_g2_of_the_identity_and_a_point_is_the_point`
      * `[✅]`   Contract: `payload.left` and `payload.right` → `Ok(AddG2SuccessReturn { sum })` holding their sum
      * `[✅]`   Collaborators: arkworks' affine addition, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_add_g2_payload`, and `build_bls12_381_arkworks_g2`
      * `[✅]`   Arrange: `build_add_g2_payload` with the left override `build_bls12_381_arkworks_g2` with the value `G2Affine::identity()` and the right override `build_bls12_381_arkworks_g2` at its default generator, so a sum that returns the left input differs from the generator
      * `[✅]`   Act: `add_g2(AddG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `eip_2537_g2_generator()`
    * `[✅]`   `mul_g1_by_one_is_the_point`
      * `[✅]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG1SuccessReturn { product })` holding the point multiplied by the scalar
      * `[✅]`   Collaborators: arkworks' affine scalar multiplication, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_mul_g1_payload`, `build_bls12_381_arkworks_g1`, and `build_bls12_381_arkworks_scalar`
      * `[✅]`   Arrange: `build_mul_g1_payload` with the point override `build_bls12_381_arkworks_g1` at its default generator and the scalar override `build_bls12_381_arkworks_scalar` with the value `Fr::from(1u64)`
      * `[✅]`   Act: `mul_g1(MulG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `eip_2537_g1_generator()`
    * `[✅]`   `mul_g1_by_zero_is_the_identity`
      * `[✅]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG1SuccessReturn { product })` holding the point multiplied by the scalar
      * `[✅]`   Collaborators: arkworks' affine scalar multiplication, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_mul_g1_payload`, `build_bls12_381_arkworks_g1`, and `build_bls12_381_arkworks_scalar`
      * `[✅]`   Arrange: `build_mul_g1_payload` with the point override `build_bls12_381_arkworks_g1` at its default generator and the scalar override `build_bls12_381_arkworks_scalar` with the value `Fr::from(0u64)`, so a product that returns the point differs from the identity
      * `[✅]`   Act: `mul_g1(MulG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `G1Affine::identity()`
    * `[✅]`   `mul_g1_by_the_group_order_minus_one_is_the_negated_generator`
      * `[✅]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG1SuccessReturn { product })` holding the point multiplied by the scalar
      * `[✅]`   Collaborators: arkworks' affine scalar multiplication, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_mul_g1_payload`, `build_bls12_381_arkworks_g1`, and `build_bls12_381_arkworks_scalar`
      * `[✅]`   Arrange: `build_mul_g1_payload` with the point override `build_bls12_381_arkworks_g1` at its default generator and the scalar override `build_bls12_381_arkworks_scalar` with the value `scalar_value(GROUP_ORDER_MINUS_ONE_HEX)`, so a product that ignores the scalar differs from the negated generator
      * `[✅]`   Act: `mul_g1(MulG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `eip_2537_negated_g1_generator()`
    * `[✅]`   `mul_g2_by_one_is_the_point`
      * `[✅]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG2SuccessReturn { product })` holding the point multiplied by the scalar
      * `[✅]`   Collaborators: arkworks' affine scalar multiplication, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_mul_g2_payload`, `build_bls12_381_arkworks_g2`, and `build_bls12_381_arkworks_scalar`
      * `[✅]`   Arrange: `build_mul_g2_payload` with the point override `build_bls12_381_arkworks_g2` at its default generator and the scalar override `build_bls12_381_arkworks_scalar` with the value `Fr::from(1u64)`
      * `[✅]`   Act: `mul_g2(MulG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `eip_2537_g2_generator()`
    * `[✅]`   `mul_g2_by_zero_is_the_identity`
      * `[✅]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG2SuccessReturn { product })` holding the point multiplied by the scalar
      * `[✅]`   Collaborators: arkworks' affine scalar multiplication, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_mul_g2_payload`, `build_bls12_381_arkworks_g2`, and `build_bls12_381_arkworks_scalar`
      * `[✅]`   Arrange: `build_mul_g2_payload` with the point override `build_bls12_381_arkworks_g2` at its default generator and the scalar override `build_bls12_381_arkworks_scalar` with the value `Fr::from(0u64)`, so a product that returns the point differs from the identity
      * `[✅]`   Act: `mul_g2(MulG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `G2Affine::identity()`
    * `[✅]`   `mul_g2_by_the_group_order_minus_one_is_the_negated_point`
      * `[✅]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG2SuccessReturn { product })` holding the point multiplied by the scalar
      * `[✅]`   Collaborators: arkworks' affine scalar multiplication, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_mul_g2_payload`, `build_bls12_381_arkworks_g2`, and `build_bls12_381_arkworks_scalar`
      * `[✅]`   Arrange: `build_mul_g2_payload` with the point override `build_bls12_381_arkworks_g2` at its default generator and the scalar override `build_bls12_381_arkworks_scalar` with the value `scalar_value(GROUP_ORDER_MINUS_ONE_HEX)`, so a product that ignores the scalar differs from the negated generator
      * `[✅]`   Act: `mul_g2(MulG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `-eip_2537_g2_generator()`
    * `[✅]`   `msm_g1_of_no_terms_is_the_identity`
      * `[✅]`   Contract: an empty `payload.terms` → `Ok(MsmG1SuccessReturn { sum })` holding the identity
      * `[✅]`   Collaborators: arkworks' multi-scalar multiplication, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing` and `build_msm_g1_payload`
      * `[✅]`   Arrange: `build_msm_g1_payload` at its default of no terms
      * `[✅]`   Act: `msm_g1(MsmG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G1Affine::identity()`
    * `[✅]`   `msm_g1_pairs_each_base_with_its_own_scalar`
      * `[✅]`   Contract: `payload.terms` → `Ok(MsmG1SuccessReturn { sum })` holding the sum of each base multiplied by its own scalar
      * `[✅]`   Collaborators: arkworks' multi-scalar multiplication, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_msm_g1_payload`, `build_msm_g1_term`, `build_bls12_381_arkworks_g1`, and `build_bls12_381_arkworks_scalar`
      * `[✅]`   Arrange: `build_msm_g1_payload` with the terms override holding two `build_msm_g1_term` values: the generator with the scalar `Fr::from(1u64)`, and the base `eip_2537_negated_g1_generator()` with the scalar `Fr::from(0u64)`, so bases and scalars exchanged between terms yield the negated generator
      * `[✅]`   Act: `msm_g1(MsmG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `eip_2537_g1_generator()`
    * `[✅]`   `msm_g1_sums_its_terms`
      * `[✅]`   Contract: `payload.terms` → `Ok(MsmG1SuccessReturn { sum })` holding the sum of each base multiplied by its own scalar
      * `[✅]`   Collaborators: arkworks' multi-scalar multiplication, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_msm_g1_payload`, `build_msm_g1_term`, `build_bls12_381_arkworks_g1`, and `build_bls12_381_arkworks_scalar`
      * `[✅]`   Arrange: `build_msm_g1_payload` with the terms override holding two `build_msm_g1_term` values: the generator with the scalar `Fr::from(1u64)`, and the base `eip_2537_negated_g1_generator()` with the scalar `Fr::from(1u64)`, so a sum over the first term alone differs from the identity
      * `[✅]`   Act: `msm_g1(MsmG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G1Affine::identity()`
    * `[✅]`   `msm_g2_of_no_terms_is_the_identity`
      * `[✅]`   Contract: an empty `payload.terms` → `Ok(MsmG2SuccessReturn { sum })` holding the identity
      * `[✅]`   Collaborators: arkworks' multi-scalar multiplication, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing` and `build_msm_g2_payload`
      * `[✅]`   Arrange: `build_msm_g2_payload` at its default of no terms
      * `[✅]`   Act: `msm_g2(MsmG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G2Affine::identity()`
    * `[✅]`   `msm_g2_pairs_each_base_with_its_own_scalar`
      * `[✅]`   Contract: `payload.terms` → `Ok(MsmG2SuccessReturn { sum })` holding the sum of each base multiplied by its own scalar
      * `[✅]`   Collaborators: arkworks' multi-scalar multiplication, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_msm_g2_payload`, `build_msm_g2_term`, `build_bls12_381_arkworks_g2`, and `build_bls12_381_arkworks_scalar`
      * `[✅]`   Arrange: `build_msm_g2_payload` with the terms override holding two `build_msm_g2_term` values: the generator with the scalar `Fr::from(1u64)`, and the base `-eip_2537_g2_generator()` with the scalar `Fr::from(0u64)`, so bases and scalars exchanged between terms yield the negated generator
      * `[✅]`   Act: `msm_g2(MsmG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `eip_2537_g2_generator()`
    * `[✅]`   `msm_g2_sums_its_terms`
      * `[✅]`   Contract: `payload.terms` → `Ok(MsmG2SuccessReturn { sum })` holding the sum of each base multiplied by its own scalar
      * `[✅]`   Collaborators: arkworks' multi-scalar multiplication, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_msm_g2_payload`, `build_msm_g2_term`, `build_bls12_381_arkworks_g2`, and `build_bls12_381_arkworks_scalar`
      * `[✅]`   Arrange: `build_msm_g2_payload` with the terms override holding two `build_msm_g2_term` values: the generator with the scalar `Fr::from(1u64)`, and the base `-eip_2537_g2_generator()` with the scalar `Fr::from(1u64)`, so a sum over the first term alone differs from the identity
      * `[✅]`   Act: `msm_g2(MsmG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G2Affine::identity()`
    * `[✅]`   `pairing_product_is_one_of_no_terms_is_true`
      * `[✅]`   Contract: an empty `payload.terms` → `Ok(PairingProductIsOneSuccessReturn { is_one })` with `is_one` true
      * `[✅]`   Collaborators: arkworks' multi-pairing, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing` and `build_pairing_product_is_one_payload`
      * `[✅]`   Arrange: `build_pairing_product_is_one_payload` at its default of no terms
      * `[✅]`   Act: `pairing_product_is_one(PairingProductIsOneParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_one` is true
    * `[✅]`   `pairing_product_is_one_of_a_pairing_and_its_first_group_negation_is_true`
      * `[✅]`   Contract: `payload.terms` whose pairings multiply to the target group's identity → `Ok(PairingProductIsOneSuccessReturn { is_one })` with `is_one` true
      * `[✅]`   Collaborators: arkworks' multi-pairing, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_pairing_product_is_one_payload`, `build_pairing_product_term`, and `build_bls12_381_arkworks_g1`
      * `[✅]`   Arrange: `build_pairing_product_is_one_payload` with the terms override holding two `build_pairing_product_term` values: the generators, and the first-group override `build_bls12_381_arkworks_g1` with the value `eip_2537_negated_g1_generator()` over the second-group generator
      * `[✅]`   Act: `pairing_product_is_one(PairingProductIsOneParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_one` is true
    * `[✅]`   `pairing_product_is_one_of_the_generators_is_false`
      * `[✅]`   Contract: `payload.terms` whose pairings multiply to a value other than the target group's identity → `Ok(PairingProductIsOneSuccessReturn { is_one })` with `is_one` false
      * `[✅]`   Collaborators: arkworks' multi-pairing, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_pairing_product_is_one_payload`, and `build_pairing_product_term`
      * `[✅]`   Arrange: `build_pairing_product_is_one_payload` with the terms override holding one `build_pairing_product_term` at its default generators
      * `[✅]`   Act: `pairing_product_is_one(PairingProductIsOneParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_one` is false
    * `[✅]`   `decode_g1_rejects_a_wrong_length`
      * `[✅]`   Contract: `payload.len() != 128` → `Err(DecodeG1ErrorReturn::WrongLength { expected: 128, actual: payload.len() })`
      * `[✅]`   Collaborators: none called; fixture `build_bls12_381_arkworks_pairing`; the untrusted bytes from `zero_bytes`
      * `[✅]`   Arrange: the payload `zero_bytes(127)`, which differs from the required length
      * `[✅]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(DecodeG1ErrorReturn::WrongLength { expected: 128, actual: 127 })`, the whole expected error
    * `[✅]`   `decode_g1_rejects_a_nonzero_padding_byte`
      * `[✅]`   Contract: a 128-byte payload with a nonzero byte among a coordinate's first 16 → `Err(DecodeG1ErrorReturn::NonCanonicalCoordinate)`
      * `[✅]`   Collaborators: none called; fixture `build_bls12_381_arkworks_pairing`; the untrusted bytes from `vector_bytes(G1_NONZERO_PADDING_HEX)`
      * `[✅]`   Arrange: the payload `vector_bytes(G1_NONZERO_PADDING_HEX)`, the generator with one padding byte changed, so a decoder that ignores the padding returns the generator
      * `[✅]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(DecodeG1ErrorReturn::NonCanonicalCoordinate)`
    * `[✅]`   `decode_g1_rejects_a_non_canonical_coordinate`
      * `[✅]`   Contract: a 128-byte payload whose first coordinate is at least the base field modulus → `Err(DecodeG1ErrorReturn::NonCanonicalCoordinate)`
      * `[✅]`   Collaborators: arkworks' field reduction, run for real as the vendor; fixture `build_bls12_381_arkworks_pairing`; the untrusted bytes from `vector_bytes(G1_X_AT_MODULUS_HEX)`
      * `[✅]`   Arrange: the payload `vector_bytes(G1_X_AT_MODULUS_HEX)`, whose reduced coordinate would otherwise fail the curve check, so a decoder that checks the curve first returns a different error
      * `[✅]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(DecodeG1ErrorReturn::NonCanonicalCoordinate)`
    * `[✅]`   `decode_g1_decodes_the_identity_from_zero_bytes`
      * `[✅]`   Contract: a 128-byte payload with both coordinates zero → `Ok(DecodeG1SuccessReturn { point })` holding the first group's identity
      * `[✅]`   Collaborators: arkworks, run for real as the vendor; fixture `build_bls12_381_arkworks_pairing`; the untrusted bytes from `zero_bytes`
      * `[✅]`   Arrange: the payload `zero_bytes(128)`
      * `[✅]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[✅]`   Assert: the success arm is extracted with `expect`; `success.point.value` equals `G1Affine::identity()`
    * `[✅]`   `decode_g1_rejects_a_point_off_the_curve`
      * `[✅]`   Contract: a canonical 128-byte payload that satisfies no curve equation → `Err(DecodeG1ErrorReturn::NotOnCurve)`
      * `[✅]`   Collaborators: arkworks' curve check, run for real as the vendor; fixture `build_bls12_381_arkworks_pairing`; the untrusted bytes from `vector_bytes(G1_GENERATOR_OFF_CURVE_HEX)`
      * `[✅]`   Arrange: the payload `vector_bytes(G1_GENERATOR_OFF_CURVE_HEX)`
      * `[✅]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(DecodeG1ErrorReturn::NotOnCurve)`
    * `[✅]`   `decode_g1_rejects_a_point_outside_the_subgroup`
      * `[✅]`   Contract: a canonical 128-byte payload on the curve and outside the prime-order subgroup → `Err(DecodeG1ErrorReturn::NotInSubgroup)`
      * `[✅]`   Collaborators: arkworks' subgroup check, run for real as the vendor; fixture `build_bls12_381_arkworks_pairing`; the untrusted bytes from `g1_outside_subgroup_bytes()`
      * `[✅]`   Arrange: the payload `g1_outside_subgroup_bytes()`
      * `[✅]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(DecodeG1ErrorReturn::NotInSubgroup)`
    * `[✅]`   `decode_g1_decodes_the_eip_2537_generator`
      * `[✅]`   Contract: a canonical 128-byte payload on the curve and in the subgroup → `Ok(DecodeG1SuccessReturn { point })` holding that point
      * `[✅]`   Collaborators: arkworks, run for real as the vendor; fixture `build_bls12_381_arkworks_pairing`; the untrusted bytes from `vector_bytes(G1_GENERATOR_HEX)`
      * `[✅]`   Arrange: the payload `vector_bytes(G1_GENERATOR_HEX)`
      * `[✅]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[✅]`   Assert: the success arm is extracted with `expect`; `success.point.value` equals `eip_2537_g1_generator()`
    * `[✅]`   `decode_g2_rejects_a_wrong_length`
      * `[✅]`   Contract: `payload.len() != 256` → `Err(DecodeG2ErrorReturn::WrongLength { expected: 256, actual: payload.len() })`
      * `[✅]`   Collaborators: none called; fixture `build_bls12_381_arkworks_pairing`; the untrusted bytes from `zero_bytes`
      * `[✅]`   Arrange: the payload `zero_bytes(255)`, which differs from the required length
      * `[✅]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(DecodeG2ErrorReturn::WrongLength { expected: 256, actual: 255 })`, the whole expected error
    * `[✅]`   `decode_g2_rejects_a_nonzero_padding_byte`
      * `[✅]`   Contract: a 256-byte payload with a nonzero byte among a coordinate's first 16 → `Err(DecodeG2ErrorReturn::NonCanonicalCoordinate)`
      * `[✅]`   Collaborators: none called; fixture `build_bls12_381_arkworks_pairing`; the untrusted bytes from `vector_bytes(G2_NONZERO_PADDING_HEX)`
      * `[✅]`   Arrange: the payload `vector_bytes(G2_NONZERO_PADDING_HEX)`, the generator with one padding byte changed, so a decoder that ignores the padding returns the generator
      * `[✅]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(DecodeG2ErrorReturn::NonCanonicalCoordinate)`
    * `[✅]`   `decode_g2_rejects_a_non_canonical_coordinate`
      * `[✅]`   Contract: a 256-byte payload whose first coordinate is at least the base field modulus → `Err(DecodeG2ErrorReturn::NonCanonicalCoordinate)`
      * `[✅]`   Collaborators: arkworks' field reduction, run for real as the vendor; fixture `build_bls12_381_arkworks_pairing`; the untrusted bytes from `vector_bytes(G2_X_C0_AT_MODULUS_HEX)`
      * `[✅]`   Arrange: the payload `vector_bytes(G2_X_C0_AT_MODULUS_HEX)`, whose reduced coordinate would otherwise fail the curve check, so a decoder that checks the curve first returns a different error
      * `[✅]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(DecodeG2ErrorReturn::NonCanonicalCoordinate)`
    * `[✅]`   `decode_g2_decodes_the_identity_from_zero_bytes`
      * `[✅]`   Contract: a 256-byte payload with every coordinate zero → `Ok(DecodeG2SuccessReturn { point })` holding the second group's identity
      * `[✅]`   Collaborators: arkworks, run for real as the vendor; fixture `build_bls12_381_arkworks_pairing`; the untrusted bytes from `zero_bytes`
      * `[✅]`   Arrange: the payload `zero_bytes(256)`
      * `[✅]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[✅]`   Assert: the success arm is extracted with `expect`; `success.point.value` equals `G2Affine::identity()`
    * `[✅]`   `decode_g2_rejects_a_point_off_the_curve`
      * `[✅]`   Contract: a canonical 256-byte payload that satisfies no curve equation → `Err(DecodeG2ErrorReturn::NotOnCurve)`
      * `[✅]`   Collaborators: arkworks' curve check, run for real as the vendor; fixture `build_bls12_381_arkworks_pairing`; the untrusted bytes from `vector_bytes(G2_GENERATOR_OFF_CURVE_HEX)`
      * `[✅]`   Arrange: the payload `vector_bytes(G2_GENERATOR_OFF_CURVE_HEX)`
      * `[✅]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(DecodeG2ErrorReturn::NotOnCurve)`
    * `[✅]`   `decode_g2_rejects_a_point_outside_the_subgroup`
      * `[✅]`   Contract: a canonical 256-byte payload on the curve and outside the prime-order subgroup → `Err(DecodeG2ErrorReturn::NotInSubgroup)`
      * `[✅]`   Collaborators: arkworks' subgroup check, run for real as the vendor; fixture `build_bls12_381_arkworks_pairing`; the untrusted bytes from `g2_outside_subgroup_bytes()`
      * `[✅]`   Arrange: the payload `g2_outside_subgroup_bytes()`
      * `[✅]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(DecodeG2ErrorReturn::NotInSubgroup)`
    * `[✅]`   `decode_g2_decodes_the_eip_2537_generator`
      * `[✅]`   Contract: a canonical 256-byte payload on the curve and in the subgroup → `Ok(DecodeG2SuccessReturn { point })` holding that point
      * `[✅]`   Collaborators: arkworks, run for real as the vendor; fixture `build_bls12_381_arkworks_pairing`; the untrusted bytes from `vector_bytes(G2_GENERATOR_HEX)`
      * `[✅]`   Arrange: the payload `vector_bytes(G2_GENERATOR_HEX)`
      * `[✅]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[✅]`   Assert: the success arm is extracted with `expect`; `success.point.value` equals `eip_2537_g2_generator()`
    * `[✅]`   `decode_scalar_rejects_a_wrong_length`
      * `[✅]`   Contract: `payload.len() != 32` → `Err(DecodeScalarErrorReturn::WrongLength { expected: 32, actual: payload.len() })`
      * `[✅]`   Collaborators: none called; fixture `build_bls12_381_arkworks_pairing`; the untrusted bytes from `zero_bytes`
      * `[✅]`   Arrange: the payload `zero_bytes(31)`, which differs from the required length
      * `[✅]`   Act: `decode_scalar(DecodeScalarParams, &payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(DecodeScalarErrorReturn::WrongLength { expected: 32, actual: 31 })`, the whole expected error
    * `[✅]`   `decode_scalar_rejects_the_group_order`
      * `[✅]`   Contract: a 32-byte payload at least the group order → `Err(DecodeScalarErrorReturn::NonCanonical)`
      * `[✅]`   Collaborators: arkworks' field reduction, run for real as the vendor; fixture `build_bls12_381_arkworks_pairing`; the untrusted bytes from `vector_bytes(GROUP_ORDER_HEX)`
      * `[✅]`   Arrange: the payload `vector_bytes(GROUP_ORDER_HEX)`, whose reduction is zero and so differs from the bytes
      * `[✅]`   Act: `decode_scalar(DecodeScalarParams, &payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(DecodeScalarErrorReturn::NonCanonical)`
    * `[✅]`   `decode_scalar_decodes_the_largest_canonical_scalar`
      * `[✅]`   Contract: a canonical 32-byte payload → `Ok(DecodeScalarSuccessReturn { scalar })` holding that scalar
      * `[✅]`   Collaborators: arkworks' field reduction, run for real as the vendor; fixture `build_bls12_381_arkworks_pairing`; the untrusted bytes from `vector_bytes(GROUP_ORDER_MINUS_ONE_HEX)`
      * `[✅]`   Arrange: the payload `vector_bytes(GROUP_ORDER_MINUS_ONE_HEX)`
      * `[✅]`   Act: `decode_scalar(DecodeScalarParams, &payload)`
      * `[✅]`   Assert: the success arm is extracted with `expect`; `success.scalar.value` equals `-Fr::from(1u64)`
    * `[✅]`   `encode_g1_writes_the_eip_2537_generator`
      * `[✅]`   Contract: a first-group point → `Ok(EncodeG1SuccessReturn { bytes })` holding `x` then `y`, each 16 zero bytes followed by 48 bytes big-endian, in this concrete's `Bls12381ArkworksEncodedG1`
      * `[✅]`   Collaborators: arkworks' coordinate encoding, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_encode_g1_payload`, and `build_bls12_381_arkworks_g1`
      * `[✅]`   Arrange: `build_encode_g1_payload` with the point override `build_bls12_381_arkworks_g1` at its default generator
      * `[✅]`   Act: `encode_g1(EncodeG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.bytes` binds to a `Bls12381ArkworksEncodedG1` typed binding; its bytes equal `vector_bytes(G1_GENERATOR_HEX)` as a byte slice
    * `[✅]`   `encode_g1_writes_the_identity_as_zero_bytes`
      * `[✅]`   Contract: the first group's identity → `Ok(EncodeG1SuccessReturn { bytes })` holding 128 zero bytes
      * `[✅]`   Collaborators: arkworks, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_encode_g1_payload`, and `build_bls12_381_arkworks_g1`
      * `[✅]`   Arrange: `build_encode_g1_payload` with the point override `build_bls12_381_arkworks_g1` with the value `G1Affine::identity()`
      * `[✅]`   Act: `encode_g1(EncodeG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; the bytes of `success.bytes` equal `zero_bytes(128)` as a byte slice
    * `[✅]`   `encode_g2_writes_the_eip_2537_generator`
      * `[✅]`   Contract: a second-group point → `Ok(EncodeG2SuccessReturn { bytes })` holding `x.c0`, `x.c1`, `y.c0`, `y.c1`, each 16 zero bytes followed by 48 bytes big-endian, in this concrete's `Bls12381ArkworksEncodedG2`
      * `[✅]`   Collaborators: arkworks' coordinate encoding, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_encode_g2_payload`, and `build_bls12_381_arkworks_g2`
      * `[✅]`   Arrange: `build_encode_g2_payload` with the point override `build_bls12_381_arkworks_g2` at its default generator
      * `[✅]`   Act: `encode_g2(EncodeG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.bytes` binds to a `Bls12381ArkworksEncodedG2` typed binding; its bytes equal `vector_bytes(G2_GENERATOR_HEX)` as a byte slice
    * `[✅]`   `encode_g2_writes_the_identity_as_zero_bytes`
      * `[✅]`   Contract: the second group's identity → `Ok(EncodeG2SuccessReturn { bytes })` holding 256 zero bytes
      * `[✅]`   Collaborators: arkworks, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_encode_g2_payload`, and `build_bls12_381_arkworks_g2`
      * `[✅]`   Arrange: `build_encode_g2_payload` with the point override `build_bls12_381_arkworks_g2` with the value `G2Affine::identity()`
      * `[✅]`   Act: `encode_g2(EncodeG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; the bytes of `success.bytes` equal `zero_bytes(256)` as a byte slice
    * `[✅]`   `encode_scalar_writes_the_largest_canonical_scalar`
      * `[✅]`   Contract: a scalar → `Ok(EncodeScalarSuccessReturn { bytes })` holding its 32 big-endian bytes in this concrete's `Bls12381ArkworksEncodedScalar`, inside a `Secret`
      * `[✅]`   Collaborators: arkworks' integer encoding, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_encode_scalar_payload`, and `build_bls12_381_arkworks_scalar`
      * `[✅]`   Arrange: `build_encode_scalar_payload` with the scalar override `build_bls12_381_arkworks_scalar` with the value `-Fr::from(1u64)`
      * `[✅]`   Act: `encode_scalar(EncodeScalarParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.bytes.expose()` binds to a `Bls12381ArkworksEncodedScalar` typed binding; its bytes equal `vector_bytes(GROUP_ORDER_MINUS_ONE_HEX)` as a byte slice
    * `[✅]`   `sample_from_uniform_bytes_rejects_a_wrong_length`
      * `[✅]`   Contract: `payload.uniform.expose().len() != 64` → `Err(SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual })`
      * `[✅]`   Collaborators: `Secret::expose`, real; fixtures `build_sample_uniform_scalar_payload` and `build_secret`
      * `[✅]`   Arrange: `build_sample_uniform_scalar_payload` with the uniform override `build_secret` holding `zero_bytes(63)`
      * `[✅]`   Act: `Bls12381ArkworksScalar::sample_from_uniform_bytes(SampleUniformScalarParams, payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual: 63 })`, the whole expected error
    * `[✅]`   `sample_from_uniform_bytes_reads_the_input_as_a_big_endian_integer`
      * `[✅]`   Contract: a 64-byte uniform input → `Ok(SampleUniformScalarSuccessReturn { scalar })` holding the input read as one big-endian integer reduced modulo the group order, inside a `Secret`
      * `[✅]`   Collaborators: `Secret::expose` and arkworks' modular reduction, real; fixtures `build_sample_uniform_scalar_payload` and `build_secret`
      * `[✅]`   Arrange: `build_sample_uniform_scalar_payload` with the uniform override `build_secret` holding `vector_bytes(UNIFORM_FIVE_HEX)`, whose last byte is five, so an input read as little-endian yields a different scalar
      * `[✅]`   Act: `Bls12381ArkworksScalar::sample_from_uniform_bytes(SampleUniformScalarParams, payload)`
      * `[✅]`   Assert: the success arm is extracted with `expect`; `success.scalar.expose().value` equals `Fr::from(5u64)`
    * `[✅]`   `sample_from_uniform_bytes_reduces_the_group_order_to_zero`
      * `[✅]`   Contract: a 64-byte uniform input → `Ok(SampleUniformScalarSuccessReturn { scalar })` holding the input reduced modulo the group order, inside a `Secret`
      * `[✅]`   Collaborators: `Secret::expose` and arkworks' modular reduction, real; fixtures `build_sample_uniform_scalar_payload` and `build_secret`
      * `[✅]`   Arrange: `build_sample_uniform_scalar_payload` with the uniform override `build_secret` holding `vector_bytes(UNIFORM_GROUP_ORDER_HEX)`, whose reduction is zero and so differs from the input
      * `[✅]`   Act: `Bls12381ArkworksScalar::sample_from_uniform_bytes(SampleUniformScalarParams, payload)`
      * `[✅]`   Assert: the success arm is extracted with `expect`; `success.scalar.expose().value` equals `Fr::from(0u64)`
    * `[✅]`   `add_scalar_of_two_and_three_is_five`
      * `[✅]`   Contract: `payload.left` and `payload.right` → `Ok(AddScalarSuccessReturn { sum })` holding their sum modulo the group order
      * `[✅]`   Collaborators: arkworks' `Fr` addition, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_add_scalar_payload`, and `build_bls12_381_arkworks_scalar`
      * `[✅]`   Arrange: `build_add_scalar_payload` with the left override `build_bls12_381_arkworks_scalar` with the value `Fr::from(2u64)` and the right override with the value `Fr::from(3u64)`
      * `[✅]`   Act: `add_scalar(AddScalarParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `Fr::from(5u64)`
    * `[✅]`   `add_scalar_reduces_modulo_the_group_order`
      * `[✅]`   Contract: `payload.left` and `payload.right` → `Ok(AddScalarSuccessReturn { sum })` holding their sum modulo the group order
      * `[✅]`   Collaborators: arkworks' `Fr` addition, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_add_scalar_payload`, and `build_bls12_381_arkworks_scalar`
      * `[✅]`   Arrange: `build_add_scalar_payload` with the left override `build_bls12_381_arkworks_scalar` with the value `scalar_value(GROUP_ORDER_MINUS_ONE_HEX)` and the right override with the value `Fr::from(2u64)`, whose integer sum exceeds the group order
      * `[✅]`   Act: `add_scalar(AddScalarParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `Fr::from(1u64)`
    * `[✅]`   `mul_scalar_of_two_and_three_is_six`
      * `[✅]`   Contract: `payload.left` and `payload.right` → `Ok(MulScalarSuccessReturn { product })` holding their product modulo the group order
      * `[✅]`   Collaborators: arkworks' `Fr` multiplication, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_mul_scalar_payload`, and `build_bls12_381_arkworks_scalar`
      * `[✅]`   Arrange: `build_mul_scalar_payload` with the left override `build_bls12_381_arkworks_scalar` with the value `Fr::from(2u64)` and the right override with the value `Fr::from(3u64)`
      * `[✅]`   Act: `mul_scalar(MulScalarParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `Fr::from(6u64)`
    * `[✅]`   `mul_scalar_reduces_modulo_the_group_order`
      * `[✅]`   Contract: `payload.left` and `payload.right` → `Ok(MulScalarSuccessReturn { product })` holding their product modulo the group order
      * `[✅]`   Collaborators: arkworks' `Fr` multiplication, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_mul_scalar_payload`, and `build_bls12_381_arkworks_scalar`
      * `[✅]`   Arrange: `build_mul_scalar_payload` with the left override `build_bls12_381_arkworks_scalar` with the value `scalar_value(GROUP_ORDER_MINUS_ONE_HEX)` and the right override with the value `Fr::from(2u64)`, whose integer product exceeds the group order
      * `[✅]`   Act: `mul_scalar(MulScalarParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `scalar_value(GROUP_ORDER_MINUS_TWO_HEX)`
    * `[✅]`   `neg_scalar_of_one_is_the_group_order_minus_one`
      * `[✅]`   Contract: `payload.scalar` → `Ok(NegScalarSuccessReturn { negation })` holding the group order minus the scalar
      * `[✅]`   Collaborators: arkworks' `Fr` negation, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_neg_scalar_payload`, and `build_bls12_381_arkworks_scalar`
      * `[✅]`   Arrange: `build_neg_scalar_payload` with the scalar override `build_bls12_381_arkworks_scalar` with the value `Fr::from(1u64)`
      * `[✅]`   Act: `neg_scalar(NegScalarParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.negation.value` equals `scalar_value(GROUP_ORDER_MINUS_ONE_HEX)`
    * `[✅]`   `neg_scalar_of_zero_is_zero`
      * `[✅]`   Contract: `payload.scalar` of zero → `Ok(NegScalarSuccessReturn { negation })` holding zero
      * `[✅]`   Collaborators: arkworks' `Fr` negation, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_neg_scalar_payload`, and `build_bls12_381_arkworks_scalar`
      * `[✅]`   Arrange: `build_neg_scalar_payload` with the scalar override `build_bls12_381_arkworks_scalar` with the value `Fr::from(0u64)`
      * `[✅]`   Act: `neg_scalar(NegScalarParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.negation.value` equals `Fr::from(0u64)`
    * `[✅]`   `neg_g1_of_the_generator_is_the_negated_generator`
      * `[✅]`   Contract: `payload.point` → `Ok(NegG1SuccessReturn { negation })` holding `(x, p - y)` for a point `(x, y)`
      * `[✅]`   Collaborators: arkworks' affine negation, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_neg_g1_payload`, and `build_bls12_381_arkworks_g1`
      * `[✅]`   Arrange: `build_neg_g1_payload` with the point override `build_bls12_381_arkworks_g1` at its default generator
      * `[✅]`   Act: `neg_g1(NegG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.negation.value` equals `eip_2537_negated_g1_generator()`
    * `[✅]`   `neg_g1_of_the_identity_is_the_identity`
      * `[✅]`   Contract: `payload.point` of the identity → `Ok(NegG1SuccessReturn { negation })` holding the identity
      * `[✅]`   Collaborators: arkworks' affine negation, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_neg_g1_payload`, and `build_bls12_381_arkworks_g1`
      * `[✅]`   Arrange: `build_neg_g1_payload` with the point override `build_bls12_381_arkworks_g1` with the value `G1Affine::identity()`
      * `[✅]`   Act: `neg_g1(NegG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.negation.value` equals `G1Affine::identity()`
    * `[✅]`   `neg_g2_negates_each_y_coefficient_modulo_the_base_field`
      * `[✅]`   Contract: `payload.point` → `Ok(NegG2SuccessReturn { negation })` holding `(x, p - y)` for a point `(x, y)`, each `y` coefficient negated
      * `[✅]`   Collaborators: arkworks' affine negation, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_neg_g2_payload`, and `build_bls12_381_arkworks_g2`
      * `[✅]`   Arrange: `build_neg_g2_payload` with the point override `build_bls12_381_arkworks_g2` at its default generator
      * `[✅]`   Act: `neg_g2(NegG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; the negation's `xy()` has an `x` equal to the generator's `x` from `eip_2537_g2_generator().xy()`; its `y.c0` and `y.c1`, each as a `BigUint`, equal `base_field_modulus()` minus the generator's corresponding coefficient as a `BigUint`
    * `[✅]`   `neg_g2_of_the_identity_is_the_identity`
      * `[✅]`   Contract: `payload.point` of the identity → `Ok(NegG2SuccessReturn { negation })` holding the identity
      * `[✅]`   Collaborators: arkworks' affine negation, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_neg_g2_payload`, and `build_bls12_381_arkworks_g2`
      * `[✅]`   Arrange: `build_neg_g2_payload` with the point override `build_bls12_381_arkworks_g2` with the value `G2Affine::identity()`
      * `[✅]`   Act: `neg_g2(NegG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.negation.value` equals `G2Affine::identity()`
    * `[✅]`   `is_identity_g1_is_true_for_the_identity`
      * `[✅]`   Contract: `payload.point` → `Ok(IsIdentityG1SuccessReturn { is_identity })`, true exactly for the identity
      * `[✅]`   Collaborators: arkworks, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_is_identity_g1_payload`, and `build_bls12_381_arkworks_g1`
      * `[✅]`   Arrange: `build_is_identity_g1_payload` with the point override `build_bls12_381_arkworks_g1` with the value `G1Affine::identity()`
      * `[✅]`   Act: `is_identity_g1(IsIdentityG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_identity` is true
    * `[✅]`   `is_identity_g1_is_false_for_the_generator`
      * `[✅]`   Contract: `payload.point` → `Ok(IsIdentityG1SuccessReturn { is_identity })`, true exactly for the identity
      * `[✅]`   Collaborators: arkworks, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_is_identity_g1_payload`, and `build_bls12_381_arkworks_g1`
      * `[✅]`   Arrange: `build_is_identity_g1_payload` with the point override `build_bls12_381_arkworks_g1` at its default generator
      * `[✅]`   Act: `is_identity_g1(IsIdentityG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_identity` is false
    * `[✅]`   `is_identity_g2_is_true_for_the_identity`
      * `[✅]`   Contract: `payload.point` → `Ok(IsIdentityG2SuccessReturn { is_identity })`, true exactly for the identity
      * `[✅]`   Collaborators: arkworks, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_is_identity_g2_payload`, and `build_bls12_381_arkworks_g2`
      * `[✅]`   Arrange: `build_is_identity_g2_payload` with the point override `build_bls12_381_arkworks_g2` with the value `G2Affine::identity()`
      * `[✅]`   Act: `is_identity_g2(IsIdentityG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_identity` is true
    * `[✅]`   `is_identity_g2_is_false_for_the_generator`
      * `[✅]`   Contract: `payload.point` → `Ok(IsIdentityG2SuccessReturn { is_identity })`, true exactly for the identity
      * `[✅]`   Collaborators: arkworks, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_is_identity_g2_payload`, and `build_bls12_381_arkworks_g2`
      * `[✅]`   Arrange: `build_is_identity_g2_payload` with the point override `build_bls12_381_arkworks_g2` at its default generator
      * `[✅]`   Act: `is_identity_g2(IsIdentityG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_identity` is false
    * `[✅]`   `pairing_product_of_no_terms_is_the_target_group_identity`
      * `[✅]`   Contract: an empty `payload.terms` → `Ok(PairingProductSuccessReturn { product })` holding the target group's identity
      * `[✅]`   Collaborators: arkworks' multi-pairing and target-group exponentiation, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing` and `build_pairing_product_payload`
      * `[✅]`   Arrange: `build_pairing_product_payload` at its default of no terms
      * `[✅]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` is the identity under `is_zero()`
    * `[✅]`   `pairing_product_of_the_generators_is_not_the_target_group_identity`
      * `[✅]`   Contract: a pairing of the generators → `Ok(PairingProductSuccessReturn { product })` holding a value other than the target group's identity
      * `[✅]`   Collaborators: arkworks' multi-pairing and target-group exponentiation, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_pairing_product_payload`, and `build_pairing_product_term`
      * `[✅]`   Arrange: `build_pairing_product_payload` with the terms override holding one `build_pairing_product_term` at its default generators
      * `[✅]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` is not the identity under `is_zero()`
    * `[✅]`   `pairing_product_is_bilinear`
      * `[✅]`   Contract: a term whose first-group element is twice the generator → `Ok(PairingProductSuccessReturn { product })` holding the generators' pairing raised to the power two
      * `[✅]`   Collaborators: arkworks' multi-pairing and target-group exponentiation, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_pairing_product_payload`, `build_pairing_product_term`, and `build_bls12_381_arkworks_g1`; the expectation is `definition_value_power`, the identifier's definition raised to a power in the fixtures, independent of the correction `pairing_product` applies
      * `[✅]`   Arrange: `build_pairing_product_payload` with the terms override holding one `build_pairing_product_term` with the first-group override `build_bls12_381_arkworks_g1` with the value `eip_2537_doubled_g1_generator()` over the second-group generator
      * `[✅]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value.0` equals `definition_value_power(eip_2537_g1_generator(), eip_2537_g2_generator(), 2)`
    * `[✅]`   `pairing_product_multiplies_its_terms`
      * `[✅]`   Contract: `payload.terms` → `Ok(PairingProductSuccessReturn { product })` holding the product of each term's pairing
      * `[✅]`   Collaborators: arkworks' multi-pairing and target-group exponentiation, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_pairing_product_payload`, and `build_pairing_product_term`; the expectation is `definition_value_power`
      * `[✅]`   Arrange: `build_pairing_product_payload` with the terms override holding two `build_pairing_product_term` values at their default generators, so a product over one term differs from the expected square
      * `[✅]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value.0` equals `definition_value_power(eip_2537_g1_generator(), eip_2537_g2_generator(), 2)`
    * `[✅]`   `pairing_product_of_a_pairing_and_its_first_group_negation_is_the_target_group_identity`
      * `[✅]`   Contract: `payload.terms` whose pairings multiply to the target group's identity → `Ok(PairingProductSuccessReturn { product })` holding the identity
      * `[✅]`   Collaborators: arkworks' multi-pairing and target-group exponentiation, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_pairing_product_payload`, `build_pairing_product_term`, and `build_bls12_381_arkworks_g1`
      * `[✅]`   Arrange: `build_pairing_product_payload` with the terms override holding two `build_pairing_product_term` values: the generators, and the first-group override `build_bls12_381_arkworks_g1` with the value `eip_2537_negated_g1_generator()` over the second-group generator
      * `[✅]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` is the identity under `is_zero()`
    * `[✅]`   `pairing_product_of_the_generators_equals_the_definition`
      * `[✅]`   Contract: a pairing of the generators → `Ok(PairingProductSuccessReturn { product })` holding the Miller loop's value raised to the exact exponent `(p^12 - 1) / r`, the value `TargetGroupEncodingIdentifier::Bls12381V1` defines, which is the library's reduced pairing with the cube removed
      * `[✅]`   Collaborators: arkworks' multi-pairing and target-group exponentiation, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_pairing_product_payload`, and `build_pairing_product_term`; the expectation is `definition_value`, the Miller loop over the generators raised to `definition_exponent()` built by `num-bigint`, independent of the correction `pairing_product` applies
      * `[✅]`   Arrange: `build_pairing_product_payload` with the terms override holding one `build_pairing_product_term` at its default generators
      * `[✅]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value.0` equals `definition_value(eip_2537_g1_generator(), eip_2537_g2_generator())`
    * `[✅]`   `encode_gt_writes_the_target_group_identity_with_c0_c0_c0_first`
      * `[✅]`   Contract: a target-group value → `Ok(EncodeGtSuccessReturn { bytes })` holding its twelve coefficients in tower order, `c0.c0.c0` first, each 48 bytes big-endian, inside a `Secret`
      * `[✅]`   Collaborators: arkworks' integer encoding, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_encode_gt_payload`, and `build_bls12_381_arkworks_gt`; the expectation is `gt_identity_encoding()`
      * `[✅]`   Arrange: `build_encode_gt_payload` with the value override `build_bls12_381_arkworks_gt` with the value `PairingOutput(Fq12::ONE)`, the target group's identity
      * `[✅]`   Act: `encode_gt(EncodeGtParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.bytes.expose()` binds to a `Bls12381ArkworksEncodedGt` typed binding; its bytes equal `gt_identity_encoding()` as a byte slice
    * `[✅]`   `encode_gt_writes_the_published_pairing_of_the_generators`
      * `[✅]`   Contract: a target-group value → `Ok(EncodeGtSuccessReturn { bytes })` holding its twelve coefficients in tower order, `c0.c0.c0` through `c1.c2.c1`, each 48 bytes big-endian, inside a `Secret`
      * `[✅]`   Collaborators: arkworks' integer encoding, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_encode_gt_payload`, and `build_bls12_381_arkworks_gt`; the expectation is `CFRG_GENERATOR_PAIRING_HEX`, the pairing of the base points published in the CFRG draft's test-vector appendix
      * `[✅]`   Arrange: `build_encode_gt_payload` with the value override `build_bls12_381_arkworks_gt` with the value `PairingOutput(definition_value(eip_2537_g1_generator(), eip_2537_g2_generator()))`, the identifier's value for the generators, whose twelve coefficients are distinct, so any exchange of two positions changes the bytes
      * `[✅]`   Act: `encode_gt(EncodeGtParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; the bytes of `success.bytes.expose()` equal `vector_bytes(CFRG_GENERATOR_PAIRING_HEX)` as a byte slice
    * `[✅]`   `scalar_field_order_is_the_group_order`
      * `[✅]`   Contract: any call → `Ok(ScalarFieldOrderSuccessReturn { bytes })` holding the group order's 32 big-endian bytes
      * `[✅]`   Collaborators: arkworks' scalar field modulus, run for real as the vendor; fixture `build_bls12_381_arkworks_pairing`; `ScalarFieldOrderParams` and `ScalarFieldOrderPayload` by their production values
      * `[✅]`   Arrange: `build_bls12_381_arkworks_pairing()`
      * `[✅]`   Act: `scalar_field_order(ScalarFieldOrderParams, ScalarFieldOrderPayload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.bytes` equals `vector_bytes(GROUP_ORDER_HEX)`
    * `[✅]`   `g1_outside_subgroup_encoding_encodes_an_on_curve_point_outside_the_subgroup`
      * `[✅]`   Contract: any call → `Ok(G1OutsideSubgroupEncodingSuccessReturn { bytes: Some(bytes) })` holding the precompile encoding of an on-curve point outside the prime-order subgroup, in this concrete's `Bls12381ArkworksEncodedG1`
      * `[✅]`   Collaborators: arkworks' curve and subgroup checks, run for real as the vendor; fixture `build_bls12_381_arkworks_pairing`; `G1OutsideSubgroupEncodingParams` and `G1OutsideSubgroupEncodingPayload` by their production values; the point read back by `g1_point_from_encoding`
      * `[✅]`   Arrange: `build_bls12_381_arkworks_pairing()`
      * `[✅]`   Act: `g1_outside_subgroup_encoding(G1OutsideSubgroupEncodingParams, G1OutsideSubgroupEncodingPayload)`
      * `[✅]`   Assert: the success arm is extracted with `expect`; `success.bytes` is `Some` and its content binds to a `Bls12381ArkworksEncodedG1` typed binding; the point `g1_point_from_encoding` reads from its bytes satisfies `is_on_curve()` and does not satisfy `is_in_correct_subgroup_assuming_on_curve()`
    * `[✅]`   `g1_outside_subgroup_encoding_uses_the_least_first_coordinate`
      * `[✅]`   Contract: any call → the encoded point's `x` is the least `x` of an on-curve point outside the subgroup in an ascending search
      * `[✅]`   Collaborators: arkworks' curve and subgroup checks, run for real as the vendor; fixture `build_bls12_381_arkworks_pairing`; `G1OutsideSubgroupEncodingParams` and `G1OutsideSubgroupEncodingPayload` by their production values; the expectation is `g1_outside_subgroup_bytes()`, the ascending search in the fixtures
      * `[✅]`   Arrange: `build_bls12_381_arkworks_pairing()`
      * `[✅]`   Act: `g1_outside_subgroup_encoding(G1OutsideSubgroupEncodingParams, G1OutsideSubgroupEncodingPayload)`
      * `[✅]`   Assert: the success arm is extracted with `expect`; the first 64 bytes of the encoding, the `x` coordinate, equal the first 64 bytes of `g1_outside_subgroup_bytes()`
    * `[✅]`   `g1_outside_subgroup_encoding_takes_the_lesser_root`
      * `[✅]`   Contract: any call → the encoded point's `y` is the lesser of the two roots sharing its `x`
      * `[✅]`   Collaborators: arkworks' curve and subgroup checks, run for real as the vendor; fixture `build_bls12_381_arkworks_pairing`; `G1OutsideSubgroupEncodingParams` and `G1OutsideSubgroupEncodingPayload` by their production values; the expectation is `base_field_modulus()`
      * `[✅]`   Arrange: `build_bls12_381_arkworks_pairing()`
      * `[✅]`   Act: `g1_outside_subgroup_encoding(G1OutsideSubgroupEncodingParams, G1OutsideSubgroupEncodingPayload)`
      * `[✅]`   Assert: the success arm is extracted with `expect`; the last 48 bytes of the encoding's `y` coordinate, bytes 80 through 127, read as a `BigUint`, are not greater than `base_field_modulus()` minus that value
    * `[✅]`   `g2_outside_subgroup_encoding_encodes_an_on_curve_point_outside_the_subgroup`
      * `[✅]`   Contract: any call → `Ok(G2OutsideSubgroupEncodingSuccessReturn { bytes })` holding the precompile encoding of an on-curve point outside the prime-order subgroup, in this concrete's `Bls12381ArkworksEncodedG2`
      * `[✅]`   Collaborators: arkworks' curve and subgroup checks, run for real as the vendor; fixture `build_bls12_381_arkworks_pairing`; `G2OutsideSubgroupEncodingParams` and `G2OutsideSubgroupEncodingPayload` by their production values; the point read back by `g2_point_from_encoding`
      * `[✅]`   Arrange: `build_bls12_381_arkworks_pairing()`
      * `[✅]`   Act: `g2_outside_subgroup_encoding(G2OutsideSubgroupEncodingParams, G2OutsideSubgroupEncodingPayload)`
      * `[✅]`   Assert: the success arm is extracted with `expect`; `success.bytes` binds to a `Bls12381ArkworksEncodedG2` typed binding; the point `g2_point_from_encoding` reads from its bytes satisfies `is_on_curve()` and does not satisfy `is_in_correct_subgroup_assuming_on_curve()`
    * `[✅]`   `g2_outside_subgroup_encoding_has_a_real_first_coordinate_and_the_lesser_root`
      * `[✅]`   Contract: any call → the encoded point's first coordinate is `(c0, 0)` with the least `c0` of an on-curve point outside the subgroup, and its `y` is the lesser of the two roots, compared by `y.c1` and then `y.c0`
      * `[✅]`   Collaborators: arkworks' curve and subgroup checks, run for real as the vendor; fixture `build_bls12_381_arkworks_pairing`; `G2OutsideSubgroupEncodingParams` and `G2OutsideSubgroupEncodingPayload` by their production values; the expectations are `g2_outside_subgroup_bytes()`, the ascending search in the fixtures, and `base_field_modulus()`
      * `[✅]`   Arrange: `build_bls12_381_arkworks_pairing()`
      * `[✅]`   Act: `g2_outside_subgroup_encoding(G2OutsideSubgroupEncodingParams, G2OutsideSubgroupEncodingPayload)`
      * `[✅]`   Assert: the success arm is extracted with `expect`; the encoding's `x.c1` coordinate, bytes 64 through 127, is all zero; its `x.c0` coordinate, bytes 0 through 63, equals the first 64 bytes of `g2_outside_subgroup_bytes()`; the pair `y.c1`, `y.c0`, each the last 48 bytes of its 64-byte coordinate read as a `BigUint`, is not greater than the pair of their negations, each `base_field_modulus()` minus the coefficient, reduced to zero where the coefficient is zero

  * `[✅]`   `construction`
    * `[✅]`   `Bls12381ArkworksPairing::try_new` is the concrete's only producer, computing its one field, and its only caller is the pairing factory, which obtains the declaration through `pairing/lookup_pairing_declaration` and completes `pairing/admit_pairing_composition` before calling it and before its correction computation
    * `[✅]`   A group element or scalar is produced only by the adapter's generators, arithmetic, and decoders, or by the scalar's sampling bound; a target-group value only by `pairing_product`; an encoded value only by the adapter's encoders and reference methods; no consumer constructs one from library values

  * `[✅]`   `adapters/pairing/src/bls12_381_arkworks/mod.rs`
    * `[✅]`   Module declarations: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, `pub(crate) mod provides;`, and `#[cfg(test)] mod test;`
    * `[✅]`   Imports: from `crate::factory::provides` every params, payload, return, success-return, and error type of the three traits and the sampling bound, `IPairingAdapter`, `IPairingArithmetic`, `IPairingReference`, and `ISampleUniformScalar`; `ark_bls12_381::{Bls12_381, Fq, Fq2, Fr, G1Affine, G1Projective, G2Affine, G2Projective}`; `ark_ec::{AffineRepr, CurveGroup, VariableBaseMSM, pairing::Pairing}`; `ark_ff::{BigInt, BigInteger, Field, PrimeField, Zero}`; `domain::{Secret, SecretConstructorParams}`; from `interface` every type it declares; `zeroize::{Zeroize, ZeroizeOnDrop}`
    * `[✅]`   `fn decode_coordinate(bytes: &[u8]) -> Option<Fq>`, private: `None` when any of `bytes[..16]` is nonzero; `Fq::from_be_bytes_mod_order(&bytes[16..64])` as the coordinate; `None` when `coordinate.into_bigint().to_bytes_be()` differs from `bytes[16..64]`; otherwise `Some(coordinate)`
    * `[✅]`   `impl Bls12381ArkworksPairing` holding `pub fn try_new(params: Bls12381ArkworksPairingConstructorParams) -> Bls12381ArkworksPairingTryNewReturn` realizing the interaction spec's construct branch; `DECLARATION` lives in `interface.rs`, the interface owning the real declaration
    * `[✅]`   `impl IPairingAdapter for Bls12381ArkworksPairing` with `type Scalar = Bls12381ArkworksScalar;`, `type G1 = Bls12381ArkworksG1;`, `type G2 = Bls12381ArkworksG2;`, `type EncodedG1 = Bls12381ArkworksEncodedG1;`, `type EncodedG2 = Bls12381ArkworksEncodedG2;`, and `type EncodedScalar = Bls12381ArkworksEncodedScalar;`, every method realizing its branches in the interaction spec, the decoders reading each coordinate through `decode_coordinate`
    * `[✅]`   `impl IPairingArithmetic for Bls12381ArkworksPairing` with `type Gt = Bls12381ArkworksGt;` and `type EncodedGt = Bls12381ArkworksEncodedGt;`, every method realizing its branches in the interaction spec, `pairing_product` multiplying the `PairingOutput` by `self.reduced_pairing_correction`
    * `[✅]`   `impl IPairingReference for Bls12381ArkworksPairing`, every method realizing its branches in the interaction spec, each search as `(1u64..).find_map(…)` with the `SearchExhausted` return by `let … else`
    * `[✅]`   `impl AsRef<[u8]>` for `Bls12381ArkworksEncodedG1`, `Bls12381ArkworksEncodedG2`, `Bls12381ArkworksEncodedScalar`, and `Bls12381ArkworksEncodedGt`, each returning `&self.bytes`; `impl Zeroize` for `Bls12381ArkworksEncodedScalar` and `Bls12381ArkworksEncodedGt`, each calling `self.bytes.zeroize()`; no implementation returns a mutable reference to the bytes
    * `[✅]`   `impl Zeroize` and `impl Drop` for `Bls12381ArkworksScalar`, `Bls12381ArkworksG1`, `Bls12381ArkworksG2`, and `Bls12381ArkworksGt`, each calling `self.value.zeroize()`; `impl ZeroizeOnDrop for Bls12381ArkworksScalar {}`, marking the drop behavior the sampling bound requires
    * `[✅]`   `impl ISampleUniformScalar for Bls12381ArkworksScalar` with `const UNIFORM_BYTES_LENGTH: usize = 64;` and `sample_from_uniform_bytes` realizing its branches in the interaction spec, the scalar moved into a `Secret` by `let Ok(scalar) = Secret::try_new(SecretConstructorParams { value });`
    * `[✅]`   No `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`; the correction is computed in `try_new` and read from no configuration; neither the order nor a point is written as a literal

  * `[✅]`   `adapters/pairing/src/bls12_381_arkworks/provides.rs`
    * `[✅]`   `pub(crate) use super::interface::*;` and `#[cfg(test)] pub(crate) use super::mock::*;`, exposing every item of the concrete's interface to the crate and, in the crate's own test build, the concrete's builders, fixtures, and helpers, and nothing beyond them

  * `[✅]`   `directionality`
    * `[✅]`   `bls12_381_arkworks` depends on the `factory` module's surface through `crate::factory::provides`, on `domain`, on `zeroize`, and on the arkworks crates; it depends on no other concrete, and `pairing/lookup_pairing_declaration` reaches its interface-held declaration through the family's private path, the selection and composition admissions consume the new member and value identifiers and declared sets, and the factory completes those admissions before constructing this member; its increments to `factory/interface.rs` and `factory/mock.rs` introduce no dependency outside the pairing family; among repository crates the crate depends on `crates/domain` alone at runtime
    * `[✅]`   `pairing/factory` constructs this concrete, the family form's recorded cycle, and proves its target-group encoding, order, and outside-the-subgroup encodings equal the halo2curves concrete's

  * `[✅]`   `requirements`
    * `[✅]`   `adapters/pairing/Cargo.toml` carries the `ark-bls12-381` dependency, and `ark-bls12-381` is named nowhere in the crate outside `adapters/pairing/src/bls12_381_arkworks`
    * `[✅]`   This concrete returns its own distinct fixed-width encoded types for the first group, the second group, the scalar, and the target group through the family's associated types
    * `[✅]`   `cargo check --all-targets --all-features`, `cargo fmt --check`, and `cargo deny check` complete without error; `cargo clippy --all-targets --all-features` reports nothing beyond the library target's unused-item warnings for the pairing concretes and the BLS12-381 variants, which `pairing/factory` constructs
    * `[✅]`   Every test in `bls12_381_arkworks/test.rs` passes: the generators and their encodings match EIP-2537, the decoders read the generators and the identity and reject a wrong length, a nonzero padding byte, a non-canonical coordinate or scalar, a point off the curve, and a point outside the subgroup in both groups, the arithmetic, multi-scalar multiplication, and pairing checks agree, and sampling rejects a wrong length, reads its input big-endian, and reduces modulo the group order (CR-04, CR-08, CR-09, and CR-10 on BLS12-381 over arkworks; CR-05 for the sampled scalar; CR-07 for the zeroization)
    * `[✅]`   The declaration compile-time assertion compiles, the `ZeroizeOnDrop` compile-time assertion compiles, `bls12_381_arkworks_scalar_zeroize_clears_its_value` passes (CR-07), and `pairing_product_of_the_generators_equals_the_definition`, `encode_gt_writes_the_published_pairing_of_the_generators`, and `reduced_pairing_correction_inverts_three` pass (CR-10, the target-group value fixed by the identifier's definition, executed in the test, and its serialization by the standard's published value)
    * `[✅]`   `pairing_concretes_declares_only_real_members` and `every_declared_identifier_spells_a_distinct_name` pass, proving the declaration carries `PairingConcrete::Bls12381Arkworks` rather than text and `PAIRING_CONCRETES` contains all three real members with distinct spellings and no mock member
    * `[✅]`   `target_group_encoding_identifiers_declares_only_real_values` and `no_value_identifier_declares_a_mock_only_variant` pass, proving `TARGET_GROUP_ENCODING_IDENTIFIERS` contains both real encodings and every new value identifier is real
    * `[✅]`   `bls12_381_arkworks_declared_selection_builds_without_configuration` passes, proving the new declared-selection arm returns the real selection with no configuration, and this concrete carries no settings module, settings node, or settings table
    * `[✅]`   The factory obtains this member's declaration through `pairing/lookup_pairing_declaration` and completes `pairing/admit_pairing_composition` before constructing the concrete
    * `[✅]`   `scalar_field_order_is_the_group_order`, `g1_outside_subgroup_encoding_encodes_an_on_curve_point_outside_the_subgroup`, `g1_outside_subgroup_encoding_takes_the_lesser_root`, `g2_outside_subgroup_encoding_encodes_an_on_curve_point_outside_the_subgroup`, and `g2_outside_subgroup_encoding_has_a_real_first_coordinate_and_the_lesser_root` pass (CR-10 and CR-11 on BLS12-381 over arkworks)
    * `[✅]`   Code outside `adapters/pairing` naming `Bls12381ArkworksPairing` or anything under `bls12_381_arkworks` fails to compile; the crate's public surface is the `factory` module's `provides`

* `[✅]`   `pairing/bls12_381_halo2curves` **BLS12-381 pairing concrete on halo2curves implementing the pairing family's generic interface, arithmetic trait, and reference trait over the EIP-2537 encodings with subgroup checks on every input and the `Bls12381V1` target-group value; the increment adding the `Bls12381Halo2curves` member variant, its spelling, its declared-set entry, and its declared-selection builder arm, a further concrete beneath the pairing factory**

  * `[✅]`   `objective`
    * `[✅]`   Problem: the harness benchmark compares pairing libraries per curve through the factory, so BLS12-381 needs a concrete over another library that satisfies the family's generic interface exactly as the arkworks BLS12-381 concrete does (CR-10)
    * `[✅]`   Problem: the KEM, the envelope, and the delivery proof consume the pairing family's arithmetic trait through whichever concrete the factory resolves, and the bytes the KEM hands the KDF are the identifier's target-group value, so this concrete implements the trait with target-group values equal to the identifier's definition, and therefore to the arkworks BLS12-381 concrete's, under the one identifier both declare; `halo2curves`' BLS12-381 final exponentiation returns the Miller loop's value raised to three times the exact exponent `(p^12 - 1) / r`, the Hayashida–Hayasaka–Teruya chain in its source, so without correction the library's reduced pairing is a different element of the target group than the identifier's (CR-04; CR-07; CR-08; CR-09; CR-10; `docs/research/cryptography.md`'s pairing adapter capability declaration)
    * `[✅]`   Problem: every concrete a consumer receives returns the group order and the family's outside-the-subgroup encodings, and the libraries of one curve return the same bytes, so this concrete implements the family's reference trait under the family's rule from its own library (CR-10 on chain; CR-11; the dependency map's `pairing/bn254_arkworks` row; the Pairing adapters and key derivation milestone's exit)
    * `[✅]`   Functional: the concrete implements `IPairingAdapter` over `halo2curves`' BLS12-381, with its own scalar, group-element, and encoded types over the library's elements as the associated types
    * `[✅]`   Functional: it encodes and decodes a base field element as EIP-2537's 64 bytes, sixteen zero bytes followed by the 48-byte big-endian integer, a first-group point as 128 bytes of `x` then `y`, a second-group point as 256 bytes of `x.c0`, `x.c1`, `y.c0`, `y.c1`, the point at infinity as all zero bytes, and a scalar as 32 big-endian bytes
    * `[✅]`   Functional: decoding rejects a wrong length, a nonzero padding byte, a field element at or above the modulus, a point off the curve, and a point outside the prime-order subgroup in either group
    * `[✅]`   Functional: its scalar type implements the family's sampling bound, reading the 64 uniform bytes as one big-endian integer reduced modulo the group order, so the same input bytes sample the same scalar under either BLS12-381 concrete
    * `[✅]`   Functional: its declaration carries `PairingConcrete::Bls12381Halo2curves`, the same real BLS12-381-provided values as the arkworks member — the BLS12-381 curve, both-group arithmetic at the verifier, the EIP-2537 precompile encoding, and `TargetGroupEncodingIdentifier::Bls12381V1` — its adapter version, and the interface version it implements, the typed member identifier with no duplicate text name, so no new value-identifier variant is needed; read through `pairing/lookup_pairing_declaration` without an instance, never from a constructed adapter
    * `[✅]`   Functional: `Bls12381Halo2curvesPairing` implements `IPairingArithmetic` with every method's behavior as `pairing/bn254_arkworks` states it for the trait; its `pairing_product` returns the identifier's exact value: the library's reduced pairing multiplied in the target group by `reduced_pairing_correction`, the inverse of three in the scalar field, computed once in `try_new` by Fermat with the group order minus two as little-endian limbs read from the library's own field arithmetic
    * `[✅]`   Functional: the target-group encoding reads the `Fq12` through `Gt::inner` and writes its twelve coefficients in the tower order `c0.c0.c0`, `c0.c0.c1`, `c0.c1.c0`, `c0.c1.c1`, `c0.c2.c0`, `c0.c2.c1`, `c1.c0.c0`, `c1.c0.c1`, `c1.c1.c0`, `c1.c1.c1`, `c1.c2.c0`, `c1.c2.c1`, each 48 bytes big-endian, 576 bytes in all, over halo2curves' BLS12-381 tower `Fq2 = Fq[u] / (u^2 + 1)`, `Fq6 = Fq2[v] / (v^3 - (u + 1))`, `Fq12 = Fq6[w] / (w^2 - v)`
    * `[✅]`   Functional: the concrete's tests execute the identifier's definition, the library's Miller loop over the two generators raised to the integer `(p^12 - 1) / r` built by `num-bigint` from the library's moduli and serialized in the tower order, and assert the pairing of the base points the CFRG draft publishes in its test-vector appendix, requiring `pairing_product` then `encode_gt` over the generators to yield those bytes
    * `[✅]`   Functional: `Bls12381Halo2curvesScalar`, `Bls12381Halo2curvesG1`, `Bls12381Halo2curvesG2`, and `Bls12381Halo2curvesGt` clear their value through `Zeroize` and on drop
    * `[✅]`   Functional: the concrete returns BLS12-381's group order as 32 big-endian bytes and each group's point under the family's rule, encoded as `encode_g1` and `encode_g2` encode any point, each refused by its own decoder as outside the subgroup
    * `[✅]`   Non-functional: `halo2curves` is named only inside the halo2curves concretes; no order and no point is written as a literal outside a test

  * `[✅]`   `role`
    * `[✅]`   Adapter: a further concrete of the pairing family, implementing the generic interface, the sampling bound, the arithmetic trait, and the reference trait `pairing/bn254_arkworks` authors, declaring the identifier it authors, using the real BLS12-381 values `pairing/bls12_381_arkworks` provides, and the increment adding the `Bls12381Halo2curves` member variant, its `bls12381_halo2curves` spelling, its `PAIRING_CONCRETES` entry, and its `build_pairing_concrete` arm
    * `[✅]`   Contributes its module line to `lib.rs`; the crate manifest's `halo2curves` dependency is `pairing/bn254_halo2curves`'s; increments `factory/interface.rs` with the member `PairingConcrete::Bls12381Halo2curves`, its `name()` spelling `bls12381_halo2curves`, and its entry in `PAIRING_CONCRETES` beside the three existing real members, and increments `factory/mock.rs` with the `build_pairing_concrete` arm returning this real selection without configuration; `PairingCurve`, `VerifierGroupArithmetic`, `PrecompileEncoding`, `TargetGroupEncodingIdentifier`, and `TARGET_GROUP_ENCODING_IDENTIFIERS` are unchanged, this member providing no new value
    * `[✅]`   Does not select between the BLS12-381 concretes; `pairing/factory` constructs a concrete by the composition's request and `harness-crypto/benchmark` records each concrete's measurements
    * `[✅]`   Does not name, import, or compare against another concrete; `pairing/factory`'s integration test proves agreement with the arkworks concrete
    * `[✅]`   Does not read the correction from configuration; it is a property of this library under this identifier and is computed in `try_new`
    * `[✅]`   Does not carry a settings module, settings node, or settings table; the concrete has no settable value, so the real selection carries no settings
    * `[✅]`   Does not carry a commit

  * `[✅]`   `module`
    * `[✅]`   Bounded context: the private `bls12_381_halo2curves` module of `adapters/pairing`, holding the adapter over `halo2curves`' BLS12-381 with its correction field, its scalar, group-element, target-group, and encoded types over the library's elements, its constructor params, its implementations of the three traits, its interface-held declaration constant carrying `PairingConcrete::Bls12381Halo2curves`, its definition and published-vector tests, its clearing, and the builder defaults for its owned types
    * `[✅]`   The node's files: the `adapters/pairing/src/lib.rs` module line, the `factory` module's `interface.rs` and `mock.rs`, and the `bls12_381_halo2curves` module's `interface.rs`, `interaction.spec.md`, `mock.rs`, `test.rs`, `mod.rs`, and `provides.rs`
    * `[✅]`   Outside: the family's traits and declaration, every other concrete, and the family's declaration lookup, selection admission, composition admission, and factory construction

  * `[✅]`   `deps`
    * `[✅]`   The `factory` module's surface, through `crate::factory::provides`: `IPairingAdapter`, `IPairingArithmetic`, `IPairingReference`, `ISampleUniformScalar`, `PairingDeclaration` with its typed `concrete` field and its real value enums `PairingCurve` with `Bls12381`, `VerifierGroupArithmetic` with `BothGroups`, `PrecompileEncoding` with `Eip2537`, and `TargetGroupEncodingIdentifier` with `Bls12381V1`, `PairingConcrete` with `Bls12381Halo2curves` and its `name()` mapping, `PAIRING_CONCRETES` with all four real members, `TARGET_GROUP_ENCODING_IDENTIFIERS` with both real encoding identifiers, `PAIRING_INTERFACE_VERSION`, every params, payload, return, success-return, error type, and builder of the three traits and the sampling bound, and, under the `mocks` feature, `build_pairing_concrete` with this member's no-configuration arm
    * `[✅]`   `domain`, runtime: `Secret` and `SecretConstructorParams`; `build_secret` and `SecretConstructorParamsOverrides` through the `mocks` feature
    * `[✅]`   `zeroize` `1.9.0`, runtime: the `Zeroize` and `ZeroizeOnDrop` traits and the zeroization of the local 64-byte input copy
    * `[✅]`   `halo2curves` `0.10.0`, the crate's runtime dependency as `pairing/bn254_halo2curves` states it, resolved to the `gt-accessor` overlay, which supplies `Gt::inner(&self) -> &Fq12`; supplies the curve, the pairing, the group arithmetic, the field arithmetic, and multi-scalar multiplication, and re-exports the `ff`, `group`, and `pairing` traits its types implement
    * `[✅]`   `hex` `0.4.3` and `num-bigint` `0.4.8`, the crate's dev-dependencies, for the test vectors and the integer exponent in `mock.rs`'s test-only fixtures and the root comparison in `test.rs`
    * `[✅]`   `core::convert::Infallible`, standard library, the constructor's error arm; `core::array::from_fn`, standard library, the Fermat exponent's limbs; `core::hint::black_box`, standard library, which keeps each clearing from being removed as a dead store, since `halo2curves`' fields, points, and target-group values implement no `Zeroize`; `core::iter::successors`, standard library, the ascending search
    * `[✅]`   Reverse dependency: `pairing/factory`

  * `[✅]`   `context_slice`
    * `[✅]`   From `halo2curves::bls12381`: `Bls12381`, the engine; `Fq`, whose representation is 48 bytes; `Fq2` with `Fq2::new(c0, c1)` and the accessors `c0()` and `c1()`; `Fq12` with `c0()` and `c1()` and the degree-six accessors `c0()`, `c1()`, and `c2()`; `Fr`, whose representation is 32 bytes; `G1`, `G1Affine`, `G2`, `G2Affine`; and `Gt` with `Gt::identity()`, `Gt::inner()`, `PartialEq`, and `Mul<&Fr> for &Gt`, the exponentiation of a target-group value by a scalar
    * `[✅]`   From `halo2curves::ff`: `Fr`'s `+`, `*`, and unary `-` modulo the group order; `Fr::from(u64)`; `Field` for `ZERO`, `ONE`, `is_zero()`, `square()`, `sqrt()` returning a `CtOption`, `invert()` returning a `CtOption`, and `pow_vartime(&self, exp: impl AsRef<[u64]>)`, exponentiation by little-endian limbs, on `Fr` and on `Fq12`; unary `-` on `Fq` and `Fq2` and binary `+` on `Fq2`; `PrimeField` for `from_repr(repr) -> CtOption<Self>`, which reads little-endian bytes and is none at or above the modulus, `to_repr()`, which writes little-endian bytes read by `as_ref()`, 48 for `Fq` and 32 for `Fr`, and `MODULUS`, the `&'static str` hex modulus with the `0x` prefix, on `Fq` and on `Fr`; `FromUniformBytes::<64>::from_uniform_bytes(&[u8; 64])` on `Fr`, a little-endian wide reduction
    * `[✅]`   From `halo2curves::group`: `Curve::to_affine`, `Group::is_identity`, `prime::PrimeCurveAffine` for `generator()`, `identity()`, `is_identity()` returning a `Choice`, and `to_curve()`, `cofactor::CofactorGroup::is_torsion_free` on `G1` and on `G2`, and the projective `+` and `* Fr`; unary `-` on `G1Affine` and `G2Affine`
    * `[✅]`   From `halo2curves`: `CurveAffine` for `from_xy(x, y) -> CtOption<Self>`, which is none off the curve, `coordinates() -> CtOption<Coordinates<Self>>`, and `b()`; `Coordinates` for `x()` and `y()`; `msm::msm_best(coeffs: &[C::Scalar], bases: &[C]) -> C::Curve`
    * `[✅]`   From `halo2curves::pairing`: `MultiMillerLoop::multi_miller_loop(&[(&G1Affine, &G2Affine)])` on `Bls12381`, returning the unreduced `Fq12`, `MillerLoopResult::final_exponentiation` returning `Gt`, whose `is_identity()` is the check, and `Engine::pairing(&G1Affine, &G2Affine)` for the builder default
    * `[✅]`   A `Choice` becomes a `bool` by `bool::from`, and a `CtOption` becomes an `Option` by `Option::from`; `u64::from(u8)` and `u64`'s `<<` and `|` fold a little-endian byte group into a limb
    * `[✅]`   From `num-bigint`, in `mock.rs`'s test-only fixtures and in `test.rs`: `BigUint::parse_bytes(&[u8], u32) -> Option<BigUint>`, `BigUint::from(u32)`, `BigUint::from_bytes_be(&[u8])`, `BigUint::pow(&self, u32)`, `Sub`, `Div`, and `Rem` between `BigUint`s, and `to_u64_digits()`
    * `[✅]`   From `domain`: `Secret::try_new(SecretConstructorParams { value })` returning `Result<Secret<T>, Infallible>`, and `Secret::expose(&self) -> &T`

  * `[✅]`   `adapters/pairing/src/lib.rs`
    * `[✅]`   The barrel holds `mod bls12_381_halo2curves;`

  * `[✅]`   `adapters/pairing/src/bls12_381_halo2curves/interface.rs`
    * `[✅]`   Imports `core::convert::Infallible`; `PairingDeclaration`, `PairingCurve`, `PairingConcrete`, `VerifierGroupArithmetic`, `PrecompileEncoding`, `TargetGroupEncodingIdentifier`, and `PAIRING_INTERFACE_VERSION` from `crate::factory::provides`; declares nothing beyond the items below
    * `[✅]`   `Bls12381Halo2curvesPairing`, a struct with no derives and one field, `pub(super) reduced_pairing_correction: halo2curves::bls12381::Fr`, with the doc comment "The inverse of three in the scalar field, the multiple by which the library's reduced pairing exceeds the identifier's exact value."
    * `[✅]`   `Bls12381Halo2curvesPairingConstructorParams`, a unit struct; `Bls12381Halo2curvesPairingTryNewReturn`, the alias `Result<Bls12381Halo2curvesPairing, Infallible>`, the error arm uninhabited because the adapter takes no configuration
    * `[✅]`   `Bls12381Halo2curvesPairing::DECLARATION`, the inherent constant `pub const DECLARATION: PairingDeclaration = PairingDeclaration { concrete: PairingConcrete::Bls12381Halo2curves, curve: PairingCurve::Bls12381, verifier_group_arithmetic: VerifierGroupArithmetic::BothGroups, precompile_encoding: PrecompileEncoding::Eip2537, target_group_encoding: TargetGroupEncodingIdentifier::Bls12381V1, adapter_version: 1, interface_version: PAIRING_INTERFACE_VERSION }`, read by `pairing/lookup_pairing_declaration` before any instance exists; the settings-free concrete carries no settings module, and the real selection carries no settings
    * `[✅]`   `Bls12381Halo2curvesScalar`, `#[derive(Clone)]`, with one field `pub(super) value: halo2curves::bls12381::Fr`; `Bls12381Halo2curvesG1`, `#[derive(Clone)]`, with one field `pub(super) value: halo2curves::bls12381::G1Affine`; `Bls12381Halo2curvesG2`, `#[derive(Clone)]`, with one field `pub(super) value: halo2curves::bls12381::G2Affine`
    * `[✅]`   `Bls12381Halo2curvesGt`, a struct with no derives and one field, `pub(super) value: halo2curves::bls12381::Gt`, the target-group value
    * `[✅]`   `Bls12381Halo2curvesEncodedG1` and `Bls12381Halo2curvesEncodedG2`, `pub struct`s with `#[derive(Clone, PartialEq, Eq)]` and one field each, `pub(super) bytes: [u8; 128]` and `pub(super) bytes: [u8; 256]`; `Bls12381Halo2curvesEncodedScalar`, a `pub struct` with no derives and one field `pub(super) bytes: [u8; 32]`; `Bls12381Halo2curvesEncodedGt`, a `pub struct` with no derives and one field `pub(super) bytes: [u8; 576]`; the scalar and target-group encodings are returned only inside a `Secret`, and no encoded type has a public constructor or mutable byte access

  * `[✅]`   `adapters/pairing/src/bls12_381_halo2curves/interaction.spec.md`
    * `[✅]`   The title `` # `bls12_381_halo2curves` — interaction spec `` and one opening sentence stating the file as the branch contract for the `bls12_381_halo2curves` module of the `pairing` crate, the pairing adapter over halo2curves' BLS12-381 encoding group elements as EIP-2537 precompile input, each branch stating condition, decision, dependency call, and the exact return outcome; then one `##` section per constructor, constant, and method, each headed by its full signature and holding a table with the columns `Branch`, `Condition`, `Decision`, `Dependency call`, and `Outcome`, in the section order `pairing/bn254_arkworks` states for its spec, then `Ordering and edges`
    * `[✅]`   `try_new`: one branch, construct; condition any params; decision none; dependency calls, in order: `(-Fr::from(2u64)).to_repr()`, the group order minus two as 32 little-endian bytes, read into four little-endian limbs by `core::array::from_fn`, each limb folding its eight bytes from the most significant by `(limb << 8) | u64::from(byte)`; then `Fr::from(3u64).pow_vartime(limbs)` as `reduced_pairing_correction`, the inverse of three by Fermat; outcome `Ok(Bls12381Halo2curvesPairing { reduced_pairing_correction })`; the error arm has no branch, `Infallible` being uninhabited
    * `[✅]`   `DECLARATION`: the inherent constant `PairingDeclaration { concrete: PairingConcrete::Bls12381Halo2curves, curve: PairingCurve::Bls12381, verifier_group_arithmetic: VerifierGroupArithmetic::BothGroups, precompile_encoding: PrecompileEncoding::Eip2537, target_group_encoding: TargetGroupEncodingIdentifier::Bls12381V1, adapter_version: 1, interface_version: PAIRING_INTERFACE_VERSION }`, read by `pairing/lookup_pairing_declaration` before any instance exists
    * `[✅]`   `g1_generator`, `g2_generator`: one branch each, generated; condition any; decision none; dependency call `G1Affine::generator()` or `G2Affine::generator()`; outcome `Ok(G1GeneratorSuccessReturn { point })` or `Ok(G2GeneratorSuccessReturn { point })`, the generator in the owned group type
    * `[✅]`   `add_g1`, `add_g2`: one branch each, summed; dependency call `to_curve()` on each payload point, the projective `+`, then `to_affine()`; outcome `Ok(AddG1SuccessReturn { sum })` or `Ok(AddG2SuccessReturn { sum })`
    * `[✅]`   `mul_g1`, `mul_g2`: one branch each, multiplied; dependency call `to_curve()` on the point, the projective `*` the scalar's `Fr`, then `to_affine()`; outcome `Ok(MulG1SuccessReturn { product })` or `Ok(MulG2SuccessReturn { product })`; the payload, holding the scalar, drops at the end of the call and the scalar is cleared
    * `[✅]`   `msm_g1`, `msm_g2`: one branch each, summed; dependency call split the terms into a `Vec` of affine bases and a `Vec<Fr>` of the same length, then `msm_best(&scalars, &bases)`, then `to_affine()`; outcome `Ok(MsmG1SuccessReturn { sum })` or `Ok(MsmG2SuccessReturn { sum })`; every element of the `Vec<Fr>` is then set to `Fr::ZERO` and the vector passed to `black_box`; an empty term list yields the identity
    * `[✅]`   `pairing_product_is_one`: one branch, evaluated; dependency call `Bls12381::multi_miller_loop` over the terms as a `Vec<(&G1Affine, &G2Affine)>` in term order, then `final_exponentiation()`; outcome `Ok(PairingProductIsOneSuccessReturn { is_one })`, `is_one` being `bool::from` of the result's `is_identity()`; an empty term list yields `is_one: true`
    * `[✅]`   `decode_g1`, its section opening with the coordinate rule, a coordinate being EIP-2537's 64 bytes, the first 16 zero and the last 48 the field element big-endian, copied into a `[u8; 48]`, reversed to little-endian, and read by `Fq::from_repr`, with a nonzero byte among the first 16 or 48 bytes at or above the base field modulus being non-canonical; its branches in order: wrong length, condition `<[u8; 128]>::try_from(payload)` fails, decision the length check, no dependency call, outcome `Err(DecodeG1ErrorReturn::WrongLength { expected: 128, actual: payload.len() })`; non-canonical coordinate, condition either 64-byte coordinate is non-canonical, decision the padding check and the `CtOption` conversion per coordinate, dependency call `Fq::from_repr`, outcome `Err(DecodeG1ErrorReturn::NonCanonicalCoordinate)`; identity, condition both coordinates zero, decision the zero check, outcome `Ok(DecodeG1SuccessReturn { point })` holding `G1Affine::identity()`; off the curve, condition `G1Affine::from_xy(x, y)` is none, outcome `Err(DecodeG1ErrorReturn::NotOnCurve)`; outside the subgroup, condition `bool::from(point.to_curve().is_torsion_free())` is false, outcome `Err(DecodeG1ErrorReturn::NotInSubgroup)`; valid, condition every check passes, outcome `Ok(DecodeG1SuccessReturn { point })`; the section states that BLS12-381's first group has a nontrivial cofactor, so the subgroup branch is reachable here as well as in the second group
    * `[✅]`   `decode_g2`: the same branches in the same order over 256 bytes, the length checked by `<[u8; 256]>::try_from(payload)`, read as `x.c0`, `x.c1`, `y.c0`, `y.c1`, each a 64-byte coordinate, assembled by `Fq2::new(c0, c1)`, with `DecodeG2ErrorReturn` and `expected: 256`; the identity is all four coordinates zero
    * `[✅]`   `decode_scalar`, its branches in order: wrong length, condition `<[u8; 32]>::try_from(payload)` fails, outcome `Err(DecodeScalarErrorReturn::WrongLength { expected: 32, actual: payload.len() })`; non-canonical, condition the bytes reversed to little-endian and read by `Fr::from_repr` are none, decision the `CtOption` conversion, outcome `Err(DecodeScalarErrorReturn::NonCanonical)`; valid, outcome `Ok(DecodeScalarSuccessReturn { scalar })`; the section states that EIP-2537's MSM accepts any 256-bit scalar while this decoder, which produces an owned scalar, admits only the canonical ones
    * `[✅]`   `encode_g1`, `encode_g2`: one branch each, encoded; decision the identity check, a point whose `coordinates()` is none; dependency call `coordinates()`, then `to_repr()` reversed to big-endian per coordinate, each written after its 16 zero bytes to its fixed 64-byte position; outcome `Ok(EncodeG1SuccessReturn { bytes })` holding `Bls12381Halo2curvesEncodedG1`, 128 zero bytes for the identity and otherwise `x()` then `y()`, or `Ok(EncodeG2SuccessReturn { bytes })` holding `Bls12381Halo2curvesEncodedG2`, 256 zero bytes for the identity and otherwise `x().c0()`, `x().c1()`, `y().c0()`, `y().c1()`, each 16 zero bytes followed by the 48 big-endian bytes
    * `[✅]`   `encode_scalar`: one branch, encoded; dependency call `to_repr()` reversed to 32 big-endian bytes; outcome `Ok(EncodeScalarSuccessReturn { bytes })`, the bytes in `Bls12381Halo2curvesEncodedScalar` moved into a `Secret`
    * `[✅]`   `UNIFORM_BYTES_LENGTH`: `64`, twice the byte width of the group order
    * `[✅]`   `sample_from_uniform_bytes`, its branches in order: wrong length, condition `<&[u8; 64]>::try_from(payload.uniform.expose().as_slice())` fails, dependency call `Secret::expose`, outcome `Err(SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual })`; sampled, condition the length is 64, dependency call the 64 bytes copied into a local `[u8; 64]` and reversed, so the big-endian integer the arkworks concrete reads is the little-endian integer halo2curves reads, then `Fr::from_uniform_bytes` over the copy, which is then zeroized, outcome `Ok(SampleUniformScalarSuccessReturn { scalar })`, the scalar moved into a `Secret`; the payload's `Secret` zeroizes the input when it drops
    * `[✅]`   `add_scalar`, `mul_scalar`: one branch each; dependency call `Fr`'s `+` or `*` over the payload scalars' values; outcome `Ok(AddScalarSuccessReturn { sum })` or `Ok(MulScalarSuccessReturn { product })`, modulo the group order; the payload's scalars are cleared as it drops
    * `[✅]`   `neg_scalar`: one branch; dependency call `Fr`'s unary `-`; outcome `Ok(NegScalarSuccessReturn { negation })`, the group order minus the scalar, and zero for zero
    * `[✅]`   `neg_g1`, `neg_g2`: one branch each; dependency call the affine point's unary `-`; outcome `Ok(NegG1SuccessReturn { negation })` or `Ok(NegG2SuccessReturn { negation })`, the identity for the identity
    * `[✅]`   `is_identity_g1`, `is_identity_g2`: one branch each; dependency call `is_identity()` on the payload point; outcome `Ok(IsIdentityG1SuccessReturn { is_identity })` or `Ok(IsIdentityG2SuccessReturn { is_identity })`, `bool::from` of the `Choice`
    * `[✅]`   `pairing_product`: one branch, evaluated; dependency call `Bls12381::multi_miller_loop` over the terms as a `Vec<(&G1Affine, &G2Affine)>` in term order, then `final_exponentiation()` as `gt`, then `&gt * &self.reduced_pairing_correction`, the exponentiation that brings the library's cubed value to the identifier's exact value; outcome `Ok(PairingProductSuccessReturn { product })` holding the corrected `Gt` in the owned target-group type; an empty term list yields the target group's identity, which the exponentiation preserves
    * `[✅]`   `encode_gt`: one branch, encoded; dependency call `inner()` on the payload's `Gt`, then each of the twelve `Fq` coefficients' `to_repr()` reversed to 48 big-endian bytes, written into its fixed position in the tower order `c0.c0.c0`, `c0.c0.c1`, `c0.c1.c0`, `c0.c1.c1`, `c0.c2.c0`, `c0.c2.c1`, `c1.c0.c0`, `c1.c0.c1`, `c1.c1.c0`, `c1.c1.c1`, `c1.c2.c0`, `c1.c2.c1` in a 576-byte buffer; outcome `Ok(EncodeGtSuccessReturn { bytes })`, a `Secret<Bls12381Halo2curvesEncodedGt>` built from the complete buffer with no variable-width intermediate; the target group's identity encodes as 47 zero bytes, `01`, and 528 zero bytes
    * `[✅]`   `scalar_field_order`: one branch, read; dependency call `(-Fr::ONE).to_repr()`; the bytes are reversed to big-endian and the last byte's lowest bit is set, the group order being odd and its predecessor even; outcome `Ok(ScalarFieldOrderSuccessReturn { bytes })`, 32 bytes; the error arm has no branch
    * `[✅]`   `g1_outside_subgroup_encoding`, exhausted: condition the search below yields no point; outcome `Err(G1OutsideSubgroupEncodingErrorReturn::SearchExhausted)`; the section states that no input takes this branch and it has no unit test
    * `[✅]`   `g1_outside_subgroup_encoding`, found: decision, per `x` from `successors(Some(Fq::ONE), |x| Some(*x + Fq::ONE))`, `y` from `(x.square() * x + G1Affine::b()).sqrt()`, the point from `G1Affine::from_xy(x, y)`, kept when `to_curve().is_torsion_free()` is false; then, with `negated = -y`, the point is kept when the big-endian bytes of `y`, its `to_repr()` reversed, are not greater than those of `negated`, and replaced by its affine negation otherwise; dependency call `self.encode_g1(EncodeG1Params, EncodeG1Payload { point: Bls12381Halo2curvesG1 { value } })`, unpacked irrefutably; outcome `Ok(G1OutsideSubgroupEncodingSuccessReturn { bytes: Some(bytes) })`, the `Bls12381Halo2curvesEncodedG1` `encode_g1` returns
    * `[✅]`   `g2_outside_subgroup_encoding`, exhausted: as the first group's, with `G2OutsideSubgroupEncodingErrorReturn::SearchExhausted`
    * `[✅]`   `g2_outside_subgroup_encoding`, found: decision, per `c0` from `successors(Some(Fq::ONE), |c0| Some(*c0 + Fq::ONE))`, `x = Fq2::new(c0, Fq::ZERO)`, `y` from `(x.square() * x + G2Affine::b()).sqrt()`, the point from `G2Affine::from_xy(x, y)`, kept when outside the subgroup; then, with `negated = -y`, the point is kept when the pair of the big-endian bytes of `y.c1()` and of `y.c0()` is not greater than the same pair for `negated`, and replaced by its affine negation otherwise; dependency call `self.encode_g2(EncodeG2Params, EncodeG2Payload { point: Bls12381Halo2curvesG2 { value } })`, unpacked irrefutably; outcome `Ok(G2OutsideSubgroupEncodingSuccessReturn { bytes })`, the `Bls12381Halo2curvesEncodedG2` `encode_g2` returns
    * `[✅]`   `Ordering and edges`, a bulleted section: every decoder checks in the stated order, length, then canonicality, then identity, then curve, then subgroup, and copies each 48-byte coordinate out of the fixed-size array before reversing it to little-endian; an empty `msm` term list yields the identity, and an empty `pairing_product_is_one` term list yields `is_one: true`, as EIP-2537 does for empty input; halo2curves' `Fr` implements no `Zeroize`, so `Bls12381Halo2curvesScalar`'s `Zeroize` implementation and its `Drop` set `value` to `Fr::ZERO` and pass `&self.value` to `black_box`, which keeps the clearing from being removed as a dead store, every clone a consumer places in a payload being cleared when the payload drops, and the same zero-then-`black_box` treatment clears the `Vec<Fr>` an `msm` builds; halo2curves' affine points and `Gt` implement no `Zeroize`, so `Bls12381Halo2curvesG1`, `Bls12381Halo2curvesG2`, and `Bls12381Halo2curvesGt` clear by setting `value` to `G1Affine::identity()`, `G2Affine::identity()`, or `Gt::identity()` and passing `&self.value` to `black_box`, in their `Zeroize` implementations and their `Drop`; each outside-the-subgroup search is ascending from one and stops at the first on-curve point outside the subgroup, the choice between a point and its negation follows the search and precedes the encoding, and the same call always returns the same bytes; `params` carries no control and is not read in any method, and no reference method reads its payload

  * `[✅]`   `adapters/pairing/src/bls12_381_halo2curves/mock.rs`
    * `[✅]`   The module-level `#![allow(dead_code, clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::too_many_arguments)]`; imports `super::interface::{Bls12381Halo2curvesEncodedGt, Bls12381Halo2curvesEncodedScalar, Bls12381Halo2curvesG1, Bls12381Halo2curvesG2, Bls12381Halo2curvesGt, Bls12381Halo2curvesPairing, Bls12381Halo2curvesPairingConstructorParams, Bls12381Halo2curvesScalar}`, `core::iter::successors`, `halo2curves::CurveAffine`, `halo2curves::bls12381::{Bls12381, Fq, Fq2, Fq12, Fr, G1Affine, G2Affine, Gt}`, `halo2curves::ff::{Field, PrimeField}`, `halo2curves::group::{cofactor::CofactorGroup, prime::PrimeCurveAffine}`, and `halo2curves::pairing::{Engine, MultiMillerLoop}`; and, `#[cfg(test)]`, `halo2curves::group::Curve`, `hex::decode`, `num_bigint::BigUint`, and `zeroize::ZeroizeOnDrop`
    * `[✅]`   The defaults of the concrete's owned types, each stated in its builder, with no `Default` implementation on any of them: the scalar `Fr::ONE`; the first-group point `G1Affine::generator()`; the second-group point `G2Affine::generator()`; the target-group value `Bls12381::pairing(&G1Affine::generator(), &G2Affine::generator())`, a non-identity target-group value
    * `[✅]`   The builders for the concrete's owned types, each overrides struct `#[derive(Default)]` with one `Option` field and each omitted value taking the default stated above: `Bls12381Halo2curvesScalarOverrides` with `pub value: Option<Fr>` and `build_bls12_381_halo2curves_scalar`; `Bls12381Halo2curvesG1Overrides` with `pub value: Option<G1Affine>` and `build_bls12_381_halo2curves_g1`; `Bls12381Halo2curvesG2Overrides` with `pub value: Option<G2Affine>` and `build_bls12_381_halo2curves_g2`; `Bls12381Halo2curvesGtOverrides` with `pub value: Option<Gt>` and `build_bls12_381_halo2curves_gt`; no corruptions type and no invalidator, since none of these arrives as untrusted data
    * `[✅]`   `build_bls12_381_halo2curves_pairing() -> Bls12381Halo2curvesPairing`, the real instance from `Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams)` through `let Ok(pairing) = …;`; the constructor params are fieldless, so the builder takes no overrides
    * `[✅]`   The builders for the concrete's encoded scalar and encoded target-group types, each overrides struct `#[derive(Default)]` with one `Option` field: `Bls12381Halo2curvesEncodedScalarOverrides` with `pub bytes: Option<[u8; 32]>` and `build_bls12_381_halo2curves_encoded_scalar(overrides: Bls12381Halo2curvesEncodedScalarOverrides) -> Bls12381Halo2curvesEncodedScalar`, the bytes defaulting to `[1u8; 32]`; `Bls12381Halo2curvesEncodedGtOverrides` with `pub bytes: Option<[u8; 576]>` and `build_bls12_381_halo2curves_encoded_gt`, the bytes defaulting to `[1u8; 576]`; the defaults are nonzero so a zeroized value differs from a built one; no corruptions type and no invalidator, since neither arrives as untrusted data
    * `[✅]`   The test fixtures, each `#[cfg(test)]`, since they use the crate's dev-dependencies: the constants `pairing/bls12_381_arkworks` states in its `mock.rs`, by the same names and values; the helpers `vector_bytes`, `zero_bytes`, `base_field_modulus`, `gt_identity_encoding`, and `requires_zeroize_on_drop` as `pairing/bls12_381_arkworks` states them; `scalar_value(hex: &str) -> Fr` as `pairing/bn254_halo2curves` states it; `g1_point_from_encoding(bytes: &[u8]) -> Option<G1Affine>`, `G1Affine::from_xy` over the two 64-byte coordinates of `bytes`, each its last 48 bytes copied into a `[u8; 48]`, reversed to little-endian, and read by `Fq::from_repr`, `None` off the curve; `g2_point_from_encoding(bytes: &[u8]) -> Option<G2Affine>`, `G2Affine::from_xy(Fq2::new(x_c0, x_c1), Fq2::new(y_c0, y_c1))` over the four 64-byte coordinates of `bytes`, read in order as `x_c0`, `x_c1`, `y_c0`, `y_c1` the same way; `eip_2537_g1_generator() -> G1Affine`, `eip_2537_negated_g1_generator() -> G1Affine`, and `eip_2537_g2_generator() -> G2Affine`, the point `g1_point_from_encoding` or `g2_point_from_encoding` returns over `vector_bytes(G1_GENERATOR_HEX)`, `vector_bytes(NEG_G1_GENERATOR_HEX)`, or `vector_bytes(G2_GENERATOR_HEX)`, unpacked by `let Some(point) = … else { panic!("the published point is on the curve") };`; `g1_outside_subgroup_bytes() -> Vec<u8>`, the least on-curve point outside the subgroup, `successors(Some(Fq::ONE), |x| Some(*x + Fq::ONE)).find_map(|x| Option::<Fq>::from((x.square() * x + G1Affine::b()).sqrt()).and_then(|y| Option::<G1Affine>::from(G1Affine::from_xy(x, y)).filter(|point| !bool::from(point.to_curve().is_torsion_free())).map(|_| (x, y))))` unpacked by `let Some((x, y)) = … else { panic!("an on-curve point outside the subgroup exists") };`, encoded as 16 zero bytes, `x.to_repr().as_ref().iter().rev()`, 16 zero bytes, and `y.to_repr().as_ref().iter().rev()`; `g2_outside_subgroup_bytes() -> Vec<u8>`, the least on-curve point outside the subgroup, `successors(Some(Fq::ONE), |c0| Some(*c0 + Fq::ONE)).find_map(|c0| { let x = Fq2::new(c0, Fq::ZERO); Option::<Fq2>::from((x.square() * x + G2Affine::b()).sqrt()).and_then(|y| Option::<G2Affine>::from(G2Affine::from_xy(x, y)).filter(|point| !bool::from(point.to_curve().is_torsion_free())).map(|_| (x, y))) })` unpacked the same way, encoded as `x.c0()`, `x.c1()`, `y.c0()`, `y.c1()`, each 16 zero bytes followed by `to_repr().as_ref().iter().rev()`; `definition_exponent() -> Vec<u64>`, `let Some(p) = BigUint::parse_bytes(Fq::MODULUS.trim_start_matches("0x").as_bytes(), 16) else { panic!("the base field modulus parses") }; let Some(r) = BigUint::parse_bytes(Fr::MODULUS.trim_start_matches("0x").as_bytes(), 16) else { panic!("the group order parses") }; ((p.pow(12) - BigUint::from(1u32)) / r).to_u64_digits()`; `definition_value(g1: G1Affine, g2: G2Affine) -> Fq12`, `Bls12381::multi_miller_loop(&[(&g1, &g2)]).pow_vartime(definition_exponent())`, the identifier's definition of the pairing value; `definition_value_power(g1: G1Affine, g2: G2Affine, exponent: u64) -> Fq12`, `definition_value(g1, g2).pow_vartime([exponent])`; `eip_2537_doubled_g1_generator() -> G1Affine`, `(eip_2537_g1_generator().to_curve() + eip_2537_g1_generator().to_curve()).to_affine()`; `corrected_generator_pairing() -> Gt`, the library's reduced pairing of the generators, `Bls12381::pairing(&G1Affine::generator(), &G2Affine::generator())`, exponentiated by the inverse of three, `&pairing * &inverse` with `inverse` the `Option::from` of `Fr::from(3u64).invert()` unpacked by `let Some(inverse) = … else { panic!("three is invertible") };`, the identifier's exact value as a `Gt`
    * `[✅]`   No mock function: the concrete is built as a real instance and owns no free function

  * `[✅]`   `adapters/pairing/src/bls12_381_halo2curves/test.rs`
    * `[✅]`   Module-level `#![allow(clippy::expect_used)]`; imports from `super::provides` the concrete, its owned types, and the mock's builders, constants, and helpers each block uses; from `crate::factory::provides` the params, payloads, errors, trait names, member and value identifiers, declared sets, and builders each block uses; `HashSet` from `std::collections`; and the `halo2curves`, `num_bigint::BigUint`, and `zeroize::Zeroize` names each block uses
    * `[✅]`   Compile-time assertion over `Bls12381Halo2curvesPairing::DECLARATION`
      * `[✅]`   Contract: the inherent constant is `PairingDeclaration { concrete: PairingConcrete::Bls12381Halo2curves, curve: PairingCurve::Bls12381, verifier_group_arithmetic: VerifierGroupArithmetic::BothGroups, precompile_encoding: PrecompileEncoding::Eip2537, target_group_encoding: TargetGroupEncodingIdentifier::Bls12381V1, adapter_version: 1, interface_version: PAIRING_INTERFACE_VERSION }`, read through `pairing/lookup_pairing_declaration` without an instance
      * `[✅]`   Arrange: the type alone
      * `[✅]`   Act: a module-level `const _: () = assert!(…);` reading `Bls12381Halo2curvesPairing::DECLARATION`, `concrete` and each enum field tested with `matches!` and each version with `==`, no text-name assertion
      * `[✅]`   Assert: the module compiles only if every field holds
    * `[✅]`   `pairing_concretes_declares_only_real_members`
      * `[✅]`   Contract: the declared set holds the real members alone → `PAIRING_CONCRETES` contains `PairingConcrete::Bn254Arkworks`, `PairingConcrete::Bn254Halo2curves`, `PairingConcrete::Bls12381Arkworks`, and `PairingConcrete::Bls12381Halo2curves` and no `Mock(_)` member
      * `[✅]`   Collaborators: none; the declared set is read as a constant, no instance is built
      * `[✅]`   Arrange: none; the constant alone
      * `[✅]`   Act: reading `PAIRING_CONCRETES`
      * `[✅]`   Assert: every member is one of the four real members and none is `Mock(_)`
    * `[✅]`   `target_group_encoding_identifiers_declares_only_real_values`
      * `[✅]`   Contract: the declared set holds the real provided values alone → `TARGET_GROUP_ENCODING_IDENTIFIERS` contains `TargetGroupEncodingIdentifier::Bn254V1` and `TargetGroupEncodingIdentifier::Bls12381V1` and no `Mock` member
      * `[✅]`   Collaborators: none; the declared set is read as a constant, no instance is built
      * `[✅]`   Arrange: none; the constant alone
      * `[✅]`   Act: reading `TARGET_GROUP_ENCODING_IDENTIFIERS`
      * `[✅]`   Assert: every member is `TargetGroupEncodingIdentifier::Bn254V1` or `TargetGroupEncodingIdentifier::Bls12381V1` and none is `Mock`
    * `[✅]`   `every_declared_identifier_spells_a_distinct_name`
      * `[✅]`   Contract: each declared member has one total spelling → the `name()` spellings of the members of `PAIRING_CONCRETES` are pairwise distinct, and so are those of `TARGET_GROUP_ENCODING_IDENTIFIERS`
      * `[✅]`   Collaborators: none; the declared sets are read as constants, no instance is built
      * `[✅]`   Arrange: none; the constants alone
      * `[✅]`   Act: collecting each member's `name()` into a `HashSet` per declared set
      * `[✅]`   Assert: each `HashSet`'s length equals its declared set's length
    * `[✅]`   `no_value_identifier_declares_a_mock_only_variant`
      * `[✅]`   Contract: a value identifier holds real provided values alone → `PairingCurve`, `VerifierGroupArithmetic`, `PrecompileEncoding`, and `TargetGroupEncodingIdentifier` have no mock-only variant
      * `[✅]`   Collaborators: none; the value identifiers are read as types, no instance is built
      * `[✅]`   Arrange: none; the types alone
      * `[✅]`   Act: reading `core::mem::variant_count` on each value identifier
      * `[✅]`   Assert: each variant count equals its real variants' count, so no mock-only variant exists
    * `[✅]`   `bls12_381_halo2curves_declared_selection_builds_without_configuration`
      * `[✅]`   Contract: the declared-selection builder's `Bls12381Halo2curves` arm returns the real selection with no configuration → `build_pairing_concrete` over `PairingConcrete::Bls12381Halo2curves` yields that member, the fieldless `PairingConcreteOverrides` carrying nothing
      * `[✅]`   Collaborators: none; the builder builds no adapter
      * `[✅]`   Arrange: `PairingConcreteOverrides` default
      * `[✅]`   Act: `build_pairing_concrete(PairingConcrete::Bls12381Halo2curves, PairingConcreteOverrides::default())`
      * `[✅]`   Assert: the result is `PairingConcrete::Bls12381Halo2curves`
    * `[✅]`   `bls12_381_halo2curves_scalar_is_zeroize_on_drop`
      * `[✅]`   Contract: `Bls12381Halo2curvesScalar` implements `ZeroizeOnDrop`, the marker the sampling bound requires
      * `[✅]`   Collaborators: none
      * `[✅]`   Arrange: the type alone
      * `[✅]`   Act: `requires_zeroize_on_drop::<Bls12381Halo2curvesScalar>()`
      * `[✅]`   Assert: the block compiles only if the type implements `ZeroizeOnDrop`
    * `[✅]`   `bls12_381_halo2curves_scalar_zeroize_clears_its_value`
      * `[✅]`   Contract: `Zeroize::zeroize` on a living scalar → its `value` is zero
      * `[✅]`   Collaborators: none; fixture `build_bls12_381_halo2curves_scalar`
      * `[✅]`   Arrange: `build_bls12_381_halo2curves_scalar` with the value override `Fr::from(5u64)`, so a scalar left unchanged differs from the expected zero
      * `[✅]`   Act: `scalar.zeroize()`
      * `[✅]`   Assert: `scalar.value` equals `Fr::ZERO`
    * `[✅]`   `bls12_381_halo2curves_g1_zeroize_clears_its_value`
      * `[✅]`   Contract: `Zeroize::zeroize` on a living first-group element → its `value` is the identity
      * `[✅]`   Collaborators: none; fixture `build_bls12_381_halo2curves_g1`
      * `[✅]`   Arrange: `build_bls12_381_halo2curves_g1` at its default generator, which differs from the identity
      * `[✅]`   Act: `g1.zeroize()`
      * `[✅]`   Assert: `g1.value` equals `G1Affine::identity()`
    * `[✅]`   `bls12_381_halo2curves_g2_zeroize_clears_its_value`
      * `[✅]`   Contract: `Zeroize::zeroize` on a living second-group element → its `value` is the identity
      * `[✅]`   Collaborators: none; fixture `build_bls12_381_halo2curves_g2`
      * `[✅]`   Arrange: `build_bls12_381_halo2curves_g2` at its default generator, which differs from the identity
      * `[✅]`   Act: `g2.zeroize()`
      * `[✅]`   Assert: `g2.value` equals `G2Affine::identity()`
    * `[✅]`   `bls12_381_halo2curves_gt_zeroize_clears_its_value`
      * `[✅]`   Contract: `Zeroize::zeroize` on a living target-group value → its `value` is the target group's identity
      * `[✅]`   Collaborators: none; fixture `build_bls12_381_halo2curves_gt`
      * `[✅]`   Arrange: `build_bls12_381_halo2curves_gt` at its default pairing of the generators, which differs from the identity
      * `[✅]`   Act: `gt.zeroize()`
      * `[✅]`   Assert: `gt.value` equals `Gt::identity()`
    * `[✅]`   `bls12_381_halo2curves_encoded_scalar_zeroize_clears_its_bytes`
      * `[✅]`   Contract: `Zeroize::zeroize` on a living encoded scalar → every byte is zero
      * `[✅]`   Collaborators: none; fixture `build_bls12_381_halo2curves_encoded_scalar`
      * `[✅]`   Arrange: `build_bls12_381_halo2curves_encoded_scalar` at its nonzero default
      * `[✅]`   Act: `encoded.zeroize()`
      * `[✅]`   Assert: the bytes of `encoded` equal `zero_bytes(32)` as a byte slice
    * `[✅]`   `bls12_381_halo2curves_encoded_gt_zeroize_clears_its_bytes`
      * `[✅]`   Contract: `Zeroize::zeroize` on a living encoded target-group value → every byte is zero
      * `[✅]`   Collaborators: none; fixture `build_bls12_381_halo2curves_encoded_gt`
      * `[✅]`   Arrange: `build_bls12_381_halo2curves_encoded_gt` at its nonzero default
      * `[✅]`   Act: `encoded.zeroize()`
      * `[✅]`   Assert: the bytes of `encoded` equal `zero_bytes(576)` as a byte slice
    * `[✅]`   `reduced_pairing_correction_inverts_three`
      * `[✅]`   Contract: any params → `Ok(Bls12381Halo2curvesPairing)` whose `reduced_pairing_correction` is the inverse of three in the scalar field
      * `[✅]`   Collaborators: halo2curves' `Fr` arithmetic, run for real as the vendor; `Bls12381Halo2curvesPairingConstructorParams` by its production value
      * `[✅]`   Arrange: `Bls12381Halo2curvesPairingConstructorParams` by its production value
      * `[✅]`   Act: `Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(pairing) = …;`; `pairing.reduced_pairing_correction * Fr::from(3u64)` equals `Fr::ONE`
    * `[✅]`   `uniform_bytes_length_is_twice_the_group_order_width`
      * `[✅]`   Contract: `UNIFORM_BYTES_LENGTH` is twice the byte width of the group order
      * `[✅]`   Collaborators: none
      * `[✅]`   Arrange: the type alone
      * `[✅]`   Act: reading `<Bls12381Halo2curvesScalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH`
      * `[✅]`   Assert: the constant equals the literal 64 written in the assertion
    * `[✅]`   `g1_generator_returns_the_eip_2537_generator`
      * `[✅]`   Contract: any call → `Ok(G1GeneratorSuccessReturn { point })` holding the first group's generator
      * `[✅]`   Collaborators: halo2curves, run for real as the vendor; fixture `build_bls12_381_halo2curves_pairing`; `G1GeneratorParams` and `G1GeneratorPayload` by their production values
      * `[✅]`   Arrange: `build_bls12_381_halo2curves_pairing()`
      * `[✅]`   Act: `g1_generator(G1GeneratorParams, G1GeneratorPayload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.point.value` equals `eip_2537_g1_generator()`
    * `[✅]`   `g2_generator_returns_the_eip_2537_generator`
      * `[✅]`   Contract: any call → `Ok(G2GeneratorSuccessReturn { point })` holding the second group's generator
      * `[✅]`   Collaborators: halo2curves, run for real as the vendor; fixture `build_bls12_381_halo2curves_pairing`; `G2GeneratorParams` and `G2GeneratorPayload` by their production values
      * `[✅]`   Arrange: `build_bls12_381_halo2curves_pairing()`
      * `[✅]`   Act: `g2_generator(G2GeneratorParams, G2GeneratorPayload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.point.value` equals `eip_2537_g2_generator()`
    * `[✅]`   `add_g1_of_a_point_and_its_negation_is_the_identity`
      * `[✅]`   Contract: `payload.left` and `payload.right` → `Ok(AddG1SuccessReturn { sum })` holding their sum
      * `[✅]`   Collaborators: halo2curves' projective addition, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_add_g1_payload`, and `build_bls12_381_halo2curves_g1`
      * `[✅]`   Arrange: `build_add_g1_payload` with the left override `build_bls12_381_halo2curves_g1` at its default generator and the right override `build_bls12_381_halo2curves_g1` with the value `eip_2537_negated_g1_generator()`, so a sum that returns either input differs from the identity
      * `[✅]`   Act: `add_g1(AddG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G1Affine::identity()`
    * `[✅]`   `add_g1_of_the_identity_and_a_point_is_the_point`
      * `[✅]`   Contract: `payload.left` and `payload.right` → `Ok(AddG1SuccessReturn { sum })` holding their sum
      * `[✅]`   Collaborators: halo2curves' projective addition, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_add_g1_payload`, and `build_bls12_381_halo2curves_g1`
      * `[✅]`   Arrange: `build_add_g1_payload` with the left override `build_bls12_381_halo2curves_g1` with the value `G1Affine::identity()` and the right override `build_bls12_381_halo2curves_g1` at its default generator, so a sum that returns the left input differs from the generator
      * `[✅]`   Act: `add_g1(AddG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `eip_2537_g1_generator()`
    * `[✅]`   `add_g2_of_a_point_and_its_negation_is_the_identity`
      * `[✅]`   Contract: `payload.left` and `payload.right` → `Ok(AddG2SuccessReturn { sum })` holding their sum
      * `[✅]`   Collaborators: halo2curves' projective addition, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_add_g2_payload`, and `build_bls12_381_halo2curves_g2`
      * `[✅]`   Arrange: `build_add_g2_payload` with the left override `build_bls12_381_halo2curves_g2` at its default generator and the right override `build_bls12_381_halo2curves_g2` with the value `-eip_2537_g2_generator()`, so a sum that returns either input differs from the identity
      * `[✅]`   Act: `add_g2(AddG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G2Affine::identity()`
    * `[✅]`   `add_g2_of_the_identity_and_a_point_is_the_point`
      * `[✅]`   Contract: `payload.left` and `payload.right` → `Ok(AddG2SuccessReturn { sum })` holding their sum
      * `[✅]`   Collaborators: halo2curves' projective addition, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_add_g2_payload`, and `build_bls12_381_halo2curves_g2`
      * `[✅]`   Arrange: `build_add_g2_payload` with the left override `build_bls12_381_halo2curves_g2` with the value `G2Affine::identity()` and the right override `build_bls12_381_halo2curves_g2` at its default generator, so a sum that returns the left input differs from the generator
      * `[✅]`   Act: `add_g2(AddG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `eip_2537_g2_generator()`
    * `[✅]`   `mul_g1_by_one_is_the_point`
      * `[✅]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG1SuccessReturn { product })` holding the point multiplied by the scalar
      * `[✅]`   Collaborators: halo2curves' projective scalar multiplication, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_mul_g1_payload`, `build_bls12_381_halo2curves_g1`, and `build_bls12_381_halo2curves_scalar`
      * `[✅]`   Arrange: `build_mul_g1_payload` with the point override `build_bls12_381_halo2curves_g1` at its default generator and the scalar override `build_bls12_381_halo2curves_scalar` with the value `Fr::from(1u64)`
      * `[✅]`   Act: `mul_g1(MulG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `eip_2537_g1_generator()`
    * `[✅]`   `mul_g1_by_zero_is_the_identity`
      * `[✅]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG1SuccessReturn { product })` holding the point multiplied by the scalar
      * `[✅]`   Collaborators: halo2curves' projective scalar multiplication, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_mul_g1_payload`, `build_bls12_381_halo2curves_g1`, and `build_bls12_381_halo2curves_scalar`
      * `[✅]`   Arrange: `build_mul_g1_payload` with the point override `build_bls12_381_halo2curves_g1` at its default generator and the scalar override `build_bls12_381_halo2curves_scalar` with the value `Fr::ZERO`, so a product that returns the point differs from the identity
      * `[✅]`   Act: `mul_g1(MulG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `G1Affine::identity()`
    * `[✅]`   `mul_g1_by_the_group_order_minus_one_is_the_negated_generator`
      * `[✅]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG1SuccessReturn { product })` holding the point multiplied by the scalar
      * `[✅]`   Collaborators: halo2curves' projective scalar multiplication, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_mul_g1_payload`, `build_bls12_381_halo2curves_g1`, and `build_bls12_381_halo2curves_scalar`
      * `[✅]`   Arrange: `build_mul_g1_payload` with the point override `build_bls12_381_halo2curves_g1` at its default generator and the scalar override `build_bls12_381_halo2curves_scalar` with the value `scalar_value(GROUP_ORDER_MINUS_ONE_HEX)`, so a product that ignores the scalar differs from the negated generator
      * `[✅]`   Act: `mul_g1(MulG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `eip_2537_negated_g1_generator()`
    * `[✅]`   `mul_g2_by_one_is_the_point`
      * `[✅]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG2SuccessReturn { product })` holding the point multiplied by the scalar
      * `[✅]`   Collaborators: halo2curves' projective scalar multiplication, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_mul_g2_payload`, `build_bls12_381_halo2curves_g2`, and `build_bls12_381_halo2curves_scalar`
      * `[✅]`   Arrange: `build_mul_g2_payload` with the point override `build_bls12_381_halo2curves_g2` at its default generator and the scalar override `build_bls12_381_halo2curves_scalar` with the value `Fr::from(1u64)`
      * `[✅]`   Act: `mul_g2(MulG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `eip_2537_g2_generator()`
    * `[✅]`   `mul_g2_by_zero_is_the_identity`
      * `[✅]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG2SuccessReturn { product })` holding the point multiplied by the scalar
      * `[✅]`   Collaborators: halo2curves' projective scalar multiplication, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_mul_g2_payload`, `build_bls12_381_halo2curves_g2`, and `build_bls12_381_halo2curves_scalar`
      * `[✅]`   Arrange: `build_mul_g2_payload` with the point override `build_bls12_381_halo2curves_g2` at its default generator and the scalar override `build_bls12_381_halo2curves_scalar` with the value `Fr::ZERO`, so a product that returns the point differs from the identity
      * `[✅]`   Act: `mul_g2(MulG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `G2Affine::identity()`
    * `[✅]`   `mul_g2_by_the_group_order_minus_one_is_the_negated_point`
      * `[✅]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG2SuccessReturn { product })` holding the point multiplied by the scalar
      * `[✅]`   Collaborators: halo2curves' projective scalar multiplication, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_mul_g2_payload`, `build_bls12_381_halo2curves_g2`, and `build_bls12_381_halo2curves_scalar`
      * `[✅]`   Arrange: `build_mul_g2_payload` with the point override `build_bls12_381_halo2curves_g2` at its default generator and the scalar override `build_bls12_381_halo2curves_scalar` with the value `scalar_value(GROUP_ORDER_MINUS_ONE_HEX)`, so a product that ignores the scalar differs from the negated generator
      * `[✅]`   Act: `mul_g2(MulG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `-eip_2537_g2_generator()`
    * `[✅]`   `msm_g1_of_no_terms_is_the_identity`
      * `[✅]`   Contract: an empty `payload.terms` → `Ok(MsmG1SuccessReturn { sum })` holding the identity
      * `[✅]`   Collaborators: halo2curves' `msm_best`, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing` and `build_msm_g1_payload`
      * `[✅]`   Arrange: `build_msm_g1_payload` at its default of no terms
      * `[✅]`   Act: `msm_g1(MsmG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G1Affine::identity()`
    * `[✅]`   `msm_g1_pairs_each_base_with_its_own_scalar`
      * `[✅]`   Contract: `payload.terms` → `Ok(MsmG1SuccessReturn { sum })` holding the sum of each base multiplied by its own scalar
      * `[✅]`   Collaborators: halo2curves' `msm_best`, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_msm_g1_payload`, `build_msm_g1_term`, `build_bls12_381_halo2curves_g1`, and `build_bls12_381_halo2curves_scalar`
      * `[✅]`   Arrange: `build_msm_g1_payload` with the terms override holding two `build_msm_g1_term` values: the generator with the scalar `Fr::from(1u64)`, and the base `eip_2537_negated_g1_generator()` with the scalar `Fr::ZERO`, so bases and scalars exchanged between terms yield the negated generator
      * `[✅]`   Act: `msm_g1(MsmG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `eip_2537_g1_generator()`
    * `[✅]`   `msm_g1_sums_its_terms`
      * `[✅]`   Contract: `payload.terms` → `Ok(MsmG1SuccessReturn { sum })` holding the sum of each base multiplied by its own scalar
      * `[✅]`   Collaborators: halo2curves' `msm_best`, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_msm_g1_payload`, `build_msm_g1_term`, `build_bls12_381_halo2curves_g1`, and `build_bls12_381_halo2curves_scalar`
      * `[✅]`   Arrange: `build_msm_g1_payload` with the terms override holding two `build_msm_g1_term` values: the generator with the scalar `Fr::from(1u64)`, and the base `eip_2537_negated_g1_generator()` with the scalar `Fr::from(1u64)`, so a sum over the first term alone differs from the identity
      * `[✅]`   Act: `msm_g1(MsmG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G1Affine::identity()`
    * `[✅]`   `msm_g2_of_no_terms_is_the_identity`
      * `[✅]`   Contract: an empty `payload.terms` → `Ok(MsmG2SuccessReturn { sum })` holding the identity
      * `[✅]`   Collaborators: halo2curves' `msm_best`, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing` and `build_msm_g2_payload`
      * `[✅]`   Arrange: `build_msm_g2_payload` at its default of no terms
      * `[✅]`   Act: `msm_g2(MsmG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G2Affine::identity()`
    * `[✅]`   `msm_g2_pairs_each_base_with_its_own_scalar`
      * `[✅]`   Contract: `payload.terms` → `Ok(MsmG2SuccessReturn { sum })` holding the sum of each base multiplied by its own scalar
      * `[✅]`   Collaborators: halo2curves' `msm_best`, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_msm_g2_payload`, `build_msm_g2_term`, `build_bls12_381_halo2curves_g2`, and `build_bls12_381_halo2curves_scalar`
      * `[✅]`   Arrange: `build_msm_g2_payload` with the terms override holding two `build_msm_g2_term` values: the generator with the scalar `Fr::from(1u64)`, and the base `-eip_2537_g2_generator()` with the scalar `Fr::ZERO`, so bases and scalars exchanged between terms yield the negated generator
      * `[✅]`   Act: `msm_g2(MsmG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `eip_2537_g2_generator()`
    * `[✅]`   `msm_g2_sums_its_terms`
      * `[✅]`   Contract: `payload.terms` → `Ok(MsmG2SuccessReturn { sum })` holding the sum of each base multiplied by its own scalar
      * `[✅]`   Collaborators: halo2curves' `msm_best`, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_msm_g2_payload`, `build_msm_g2_term`, `build_bls12_381_halo2curves_g2`, and `build_bls12_381_halo2curves_scalar`
      * `[✅]`   Arrange: `build_msm_g2_payload` with the terms override holding two `build_msm_g2_term` values: the generator with the scalar `Fr::from(1u64)`, and the base `-eip_2537_g2_generator()` with the scalar `Fr::from(1u64)`, so a sum over the first term alone differs from the identity
      * `[✅]`   Act: `msm_g2(MsmG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G2Affine::identity()`
    * `[✅]`   `pairing_product_is_one_of_no_terms_is_true`
      * `[✅]`   Contract: an empty `payload.terms` → `Ok(PairingProductIsOneSuccessReturn { is_one })` with `is_one` true
      * `[✅]`   Collaborators: halo2curves' multi-Miller loop and final exponentiation, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing` and `build_pairing_product_is_one_payload`
      * `[✅]`   Arrange: `build_pairing_product_is_one_payload` at its default of no terms
      * `[✅]`   Act: `pairing_product_is_one(PairingProductIsOneParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_one` is true
    * `[✅]`   `pairing_product_is_one_of_a_pairing_and_its_first_group_negation_is_true`
      * `[✅]`   Contract: `payload.terms` whose pairings multiply to the target group's identity → `Ok(PairingProductIsOneSuccessReturn { is_one })` with `is_one` true
      * `[✅]`   Collaborators: halo2curves' multi-Miller loop and final exponentiation, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_pairing_product_is_one_payload`, `build_pairing_product_term`, and `build_bls12_381_halo2curves_g1`
      * `[✅]`   Arrange: `build_pairing_product_is_one_payload` with the terms override holding two `build_pairing_product_term` values: the generators, and the first-group override `build_bls12_381_halo2curves_g1` with the value `eip_2537_negated_g1_generator()` over the second-group generator
      * `[✅]`   Act: `pairing_product_is_one(PairingProductIsOneParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_one` is true
    * `[✅]`   `pairing_product_is_one_of_the_generators_is_false`
      * `[✅]`   Contract: `payload.terms` whose pairings multiply to a value other than the target group's identity → `Ok(PairingProductIsOneSuccessReturn { is_one })` with `is_one` false
      * `[✅]`   Collaborators: halo2curves' multi-Miller loop and final exponentiation, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_pairing_product_is_one_payload`, and `build_pairing_product_term`
      * `[✅]`   Arrange: `build_pairing_product_is_one_payload` with the terms override holding one `build_pairing_product_term` at its default generators
      * `[✅]`   Act: `pairing_product_is_one(PairingProductIsOneParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_one` is false
    * `[✅]`   `decode_g1_rejects_a_wrong_length`
      * `[✅]`   Contract: `payload.len() != 128` → `Err(DecodeG1ErrorReturn::WrongLength { expected: 128, actual: payload.len() })`
      * `[✅]`   Collaborators: none called; fixture `build_bls12_381_halo2curves_pairing`; the untrusted bytes from `zero_bytes`
      * `[✅]`   Arrange: the payload `zero_bytes(127)`, which differs from the required length
      * `[✅]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(DecodeG1ErrorReturn::WrongLength { expected: 128, actual: 127 })`, the whole expected error
    * `[✅]`   `decode_g1_rejects_a_nonzero_padding_byte`
      * `[✅]`   Contract: a 128-byte payload with a nonzero byte among a coordinate's first 16 → `Err(DecodeG1ErrorReturn::NonCanonicalCoordinate)`
      * `[✅]`   Collaborators: none called; fixture `build_bls12_381_halo2curves_pairing`; the untrusted bytes from `vector_bytes(G1_NONZERO_PADDING_HEX)`
      * `[✅]`   Arrange: the payload `vector_bytes(G1_NONZERO_PADDING_HEX)`, the generator with one padding byte changed, so a decoder that ignores the padding returns the generator
      * `[✅]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(DecodeG1ErrorReturn::NonCanonicalCoordinate)`
    * `[✅]`   `decode_g1_rejects_a_non_canonical_coordinate`
      * `[✅]`   Contract: a 128-byte payload whose first coordinate is at least the base field modulus → `Err(DecodeG1ErrorReturn::NonCanonicalCoordinate)`
      * `[✅]`   Collaborators: halo2curves' `Fq::from_repr`, run for real as the vendor; fixture `build_bls12_381_halo2curves_pairing`; the untrusted bytes from `vector_bytes(G1_X_AT_MODULUS_HEX)`
      * `[✅]`   Arrange: the payload `vector_bytes(G1_X_AT_MODULUS_HEX)`, whose coordinates would otherwise fail the curve check, so a decoder that checks the curve first returns a different error
      * `[✅]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(DecodeG1ErrorReturn::NonCanonicalCoordinate)`
    * `[✅]`   `decode_g1_decodes_the_identity_from_zero_bytes`
      * `[✅]`   Contract: a 128-byte payload with both coordinates zero → `Ok(DecodeG1SuccessReturn { point })` holding the first group's identity
      * `[✅]`   Collaborators: halo2curves, run for real as the vendor; fixture `build_bls12_381_halo2curves_pairing`; the untrusted bytes from `zero_bytes`
      * `[✅]`   Arrange: the payload `zero_bytes(128)`
      * `[✅]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[✅]`   Assert: the success arm is extracted with `expect`; `success.point.value` equals `G1Affine::identity()`
    * `[✅]`   `decode_g1_rejects_a_point_off_the_curve`
      * `[✅]`   Contract: a canonical 128-byte payload that satisfies no curve equation → `Err(DecodeG1ErrorReturn::NotOnCurve)`
      * `[✅]`   Collaborators: halo2curves' `from_xy`, run for real as the vendor; fixture `build_bls12_381_halo2curves_pairing`; the untrusted bytes from `vector_bytes(G1_GENERATOR_OFF_CURVE_HEX)`
      * `[✅]`   Arrange: the payload `vector_bytes(G1_GENERATOR_OFF_CURVE_HEX)`
      * `[✅]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(DecodeG1ErrorReturn::NotOnCurve)`
    * `[✅]`   `decode_g1_rejects_a_point_outside_the_subgroup`
      * `[✅]`   Contract: a canonical 128-byte payload on the curve and outside the prime-order subgroup → `Err(DecodeG1ErrorReturn::NotInSubgroup)`
      * `[✅]`   Collaborators: halo2curves' torsion check, run for real as the vendor; fixture `build_bls12_381_halo2curves_pairing`; the untrusted bytes from `g1_outside_subgroup_bytes()`
      * `[✅]`   Arrange: the payload `g1_outside_subgroup_bytes()`
      * `[✅]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(DecodeG1ErrorReturn::NotInSubgroup)`
    * `[✅]`   `decode_g1_decodes_the_eip_2537_generator`
      * `[✅]`   Contract: a canonical 128-byte payload on the curve and in the subgroup → `Ok(DecodeG1SuccessReturn { point })` holding that point
      * `[✅]`   Collaborators: halo2curves, run for real as the vendor; fixture `build_bls12_381_halo2curves_pairing`; the untrusted bytes from `vector_bytes(G1_GENERATOR_HEX)`
      * `[✅]`   Arrange: the payload `vector_bytes(G1_GENERATOR_HEX)`
      * `[✅]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[✅]`   Assert: the success arm is extracted with `expect`; `success.point.value` equals `eip_2537_g1_generator()`
    * `[✅]`   `decode_g2_rejects_a_wrong_length`
      * `[✅]`   Contract: `payload.len() != 256` → `Err(DecodeG2ErrorReturn::WrongLength { expected: 256, actual: payload.len() })`
      * `[✅]`   Collaborators: none called; fixture `build_bls12_381_halo2curves_pairing`; the untrusted bytes from `zero_bytes`
      * `[✅]`   Arrange: the payload `zero_bytes(255)`, which differs from the required length
      * `[✅]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(DecodeG2ErrorReturn::WrongLength { expected: 256, actual: 255 })`, the whole expected error
    * `[✅]`   `decode_g2_rejects_a_nonzero_padding_byte`
      * `[✅]`   Contract: a 256-byte payload with a nonzero byte among a coordinate's first 16 → `Err(DecodeG2ErrorReturn::NonCanonicalCoordinate)`
      * `[✅]`   Collaborators: none called; fixture `build_bls12_381_halo2curves_pairing`; the untrusted bytes from `vector_bytes(G2_NONZERO_PADDING_HEX)`
      * `[✅]`   Arrange: the payload `vector_bytes(G2_NONZERO_PADDING_HEX)`, the generator with one padding byte changed, so a decoder that ignores the padding returns the generator
      * `[✅]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(DecodeG2ErrorReturn::NonCanonicalCoordinate)`
    * `[✅]`   `decode_g2_rejects_a_non_canonical_coordinate`
      * `[✅]`   Contract: a 256-byte payload whose first coordinate is at least the base field modulus → `Err(DecodeG2ErrorReturn::NonCanonicalCoordinate)`
      * `[✅]`   Collaborators: halo2curves' `Fq::from_repr`, run for real as the vendor; fixture `build_bls12_381_halo2curves_pairing`; the untrusted bytes from `vector_bytes(G2_X_C0_AT_MODULUS_HEX)`
      * `[✅]`   Arrange: the payload `vector_bytes(G2_X_C0_AT_MODULUS_HEX)`, whose coordinates would otherwise fail the curve check, so a decoder that checks the curve first returns a different error
      * `[✅]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(DecodeG2ErrorReturn::NonCanonicalCoordinate)`
    * `[✅]`   `decode_g2_decodes_the_identity_from_zero_bytes`
      * `[✅]`   Contract: a 256-byte payload with every coordinate zero → `Ok(DecodeG2SuccessReturn { point })` holding the second group's identity
      * `[✅]`   Collaborators: halo2curves, run for real as the vendor; fixture `build_bls12_381_halo2curves_pairing`; the untrusted bytes from `zero_bytes`
      * `[✅]`   Arrange: the payload `zero_bytes(256)`
      * `[✅]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[✅]`   Assert: the success arm is extracted with `expect`; `success.point.value` equals `G2Affine::identity()`
    * `[✅]`   `decode_g2_rejects_a_point_off_the_curve`
      * `[✅]`   Contract: a canonical 256-byte payload that satisfies no curve equation → `Err(DecodeG2ErrorReturn::NotOnCurve)`
      * `[✅]`   Collaborators: halo2curves' `from_xy`, run for real as the vendor; fixture `build_bls12_381_halo2curves_pairing`; the untrusted bytes from `vector_bytes(G2_GENERATOR_OFF_CURVE_HEX)`
      * `[✅]`   Arrange: the payload `vector_bytes(G2_GENERATOR_OFF_CURVE_HEX)`
      * `[✅]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(DecodeG2ErrorReturn::NotOnCurve)`
    * `[✅]`   `decode_g2_rejects_a_point_outside_the_subgroup`
      * `[✅]`   Contract: a canonical 256-byte payload on the curve and outside the prime-order subgroup → `Err(DecodeG2ErrorReturn::NotInSubgroup)`
      * `[✅]`   Collaborators: halo2curves' torsion check, run for real as the vendor; fixture `build_bls12_381_halo2curves_pairing`; the untrusted bytes from `g2_outside_subgroup_bytes()`
      * `[✅]`   Arrange: the payload `g2_outside_subgroup_bytes()`
      * `[✅]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(DecodeG2ErrorReturn::NotInSubgroup)`
    * `[✅]`   `decode_g2_decodes_the_eip_2537_generator`
      * `[✅]`   Contract: a canonical 256-byte payload on the curve and in the subgroup → `Ok(DecodeG2SuccessReturn { point })` holding that point
      * `[✅]`   Collaborators: halo2curves, run for real as the vendor; fixture `build_bls12_381_halo2curves_pairing`; the untrusted bytes from `vector_bytes(G2_GENERATOR_HEX)`
      * `[✅]`   Arrange: the payload `vector_bytes(G2_GENERATOR_HEX)`
      * `[✅]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[✅]`   Assert: the success arm is extracted with `expect`; `success.point.value` equals `eip_2537_g2_generator()`
    * `[✅]`   `decode_scalar_rejects_a_wrong_length`
      * `[✅]`   Contract: `payload.len() != 32` → `Err(DecodeScalarErrorReturn::WrongLength { expected: 32, actual: payload.len() })`
      * `[✅]`   Collaborators: none called; fixture `build_bls12_381_halo2curves_pairing`; the untrusted bytes from `zero_bytes`
      * `[✅]`   Arrange: the payload `zero_bytes(31)`, which differs from the required length
      * `[✅]`   Act: `decode_scalar(DecodeScalarParams, &payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(DecodeScalarErrorReturn::WrongLength { expected: 32, actual: 31 })`, the whole expected error
    * `[✅]`   `decode_scalar_rejects_the_group_order`
      * `[✅]`   Contract: a 32-byte payload at least the group order → `Err(DecodeScalarErrorReturn::NonCanonical)`
      * `[✅]`   Collaborators: halo2curves' `Fr::from_repr`, run for real as the vendor; fixture `build_bls12_381_halo2curves_pairing`; the untrusted bytes from `vector_bytes(GROUP_ORDER_HEX)`
      * `[✅]`   Arrange: the payload `vector_bytes(GROUP_ORDER_HEX)`
      * `[✅]`   Act: `decode_scalar(DecodeScalarParams, &payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(DecodeScalarErrorReturn::NonCanonical)`
    * `[✅]`   `decode_scalar_decodes_the_largest_canonical_scalar`
      * `[✅]`   Contract: a canonical 32-byte payload → `Ok(DecodeScalarSuccessReturn { scalar })` holding that scalar
      * `[✅]`   Collaborators: halo2curves' `Fr::from_repr`, run for real as the vendor; fixture `build_bls12_381_halo2curves_pairing`; the untrusted bytes from `vector_bytes(GROUP_ORDER_MINUS_ONE_HEX)`
      * `[✅]`   Arrange: the payload `vector_bytes(GROUP_ORDER_MINUS_ONE_HEX)`
      * `[✅]`   Act: `decode_scalar(DecodeScalarParams, &payload)`
      * `[✅]`   Assert: the success arm is extracted with `expect`; `success.scalar.value` equals `-Fr::from(1u64)`
    * `[✅]`   `encode_g1_writes_the_eip_2537_generator`
      * `[✅]`   Contract: a first-group point → `Ok(EncodeG1SuccessReturn { bytes })` holding `x` then `y`, each 16 zero bytes followed by 48 bytes big-endian, in this concrete's `Bls12381Halo2curvesEncodedG1`
      * `[✅]`   Collaborators: halo2curves' coordinate encoding, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_encode_g1_payload`, and `build_bls12_381_halo2curves_g1`
      * `[✅]`   Arrange: `build_encode_g1_payload` with the point override `build_bls12_381_halo2curves_g1` at its default generator
      * `[✅]`   Act: `encode_g1(EncodeG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.bytes` binds to a `Bls12381Halo2curvesEncodedG1` typed binding; its bytes equal `vector_bytes(G1_GENERATOR_HEX)` as a byte slice
    * `[✅]`   `encode_g1_writes_the_identity_as_zero_bytes`
      * `[✅]`   Contract: the first group's identity → `Ok(EncodeG1SuccessReturn { bytes })` holding 128 zero bytes
      * `[✅]`   Collaborators: halo2curves, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_encode_g1_payload`, and `build_bls12_381_halo2curves_g1`
      * `[✅]`   Arrange: `build_encode_g1_payload` with the point override `build_bls12_381_halo2curves_g1` with the value `G1Affine::identity()`
      * `[✅]`   Act: `encode_g1(EncodeG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; the bytes of `success.bytes` equal `zero_bytes(128)` as a byte slice
    * `[✅]`   `encode_g2_writes_the_eip_2537_generator`
      * `[✅]`   Contract: a second-group point → `Ok(EncodeG2SuccessReturn { bytes })` holding `x.c0`, `x.c1`, `y.c0`, `y.c1`, each 16 zero bytes followed by 48 bytes big-endian, in this concrete's `Bls12381Halo2curvesEncodedG2`
      * `[✅]`   Collaborators: halo2curves' coordinate encoding, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_encode_g2_payload`, and `build_bls12_381_halo2curves_g2`
      * `[✅]`   Arrange: `build_encode_g2_payload` with the point override `build_bls12_381_halo2curves_g2` at its default generator
      * `[✅]`   Act: `encode_g2(EncodeG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.bytes` binds to a `Bls12381Halo2curvesEncodedG2` typed binding; its bytes equal `vector_bytes(G2_GENERATOR_HEX)` as a byte slice
    * `[✅]`   `encode_g2_writes_the_identity_as_zero_bytes`
      * `[✅]`   Contract: the second group's identity → `Ok(EncodeG2SuccessReturn { bytes })` holding 256 zero bytes
      * `[✅]`   Collaborators: halo2curves, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_encode_g2_payload`, and `build_bls12_381_halo2curves_g2`
      * `[✅]`   Arrange: `build_encode_g2_payload` with the point override `build_bls12_381_halo2curves_g2` with the value `G2Affine::identity()`
      * `[✅]`   Act: `encode_g2(EncodeG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; the bytes of `success.bytes` equal `zero_bytes(256)` as a byte slice
    * `[✅]`   `encode_scalar_writes_the_largest_canonical_scalar`
      * `[✅]`   Contract: a scalar → `Ok(EncodeScalarSuccessReturn { bytes })` holding its 32 big-endian bytes in this concrete's `Bls12381Halo2curvesEncodedScalar`, inside a `Secret`
      * `[✅]`   Collaborators: halo2curves' `to_repr`, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_encode_scalar_payload`, and `build_bls12_381_halo2curves_scalar`
      * `[✅]`   Arrange: `build_encode_scalar_payload` with the scalar override `build_bls12_381_halo2curves_scalar` with the value `-Fr::from(1u64)`
      * `[✅]`   Act: `encode_scalar(EncodeScalarParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.bytes.expose()` binds to a `Bls12381Halo2curvesEncodedScalar` typed binding; its bytes equal `vector_bytes(GROUP_ORDER_MINUS_ONE_HEX)` as a byte slice
    * `[✅]`   `sample_from_uniform_bytes_rejects_a_wrong_length`
      * `[✅]`   Contract: `payload.uniform.expose().len() != 64` → `Err(SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual })`
      * `[✅]`   Collaborators: `Secret::expose`, real; fixtures `build_sample_uniform_scalar_payload` and `build_secret`
      * `[✅]`   Arrange: `build_sample_uniform_scalar_payload` with the uniform override `build_secret` holding `zero_bytes(63)`
      * `[✅]`   Act: `Bls12381Halo2curvesScalar::sample_from_uniform_bytes(SampleUniformScalarParams, payload)`
      * `[✅]`   Assert: the result's `.err()` equals `Some(SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual: 63 })`, the whole expected error
    * `[✅]`   `sample_from_uniform_bytes_reads_the_input_as_a_big_endian_integer`
      * `[✅]`   Contract: a 64-byte uniform input → `Ok(SampleUniformScalarSuccessReturn { scalar })` holding the input read as one big-endian integer reduced modulo the group order, inside a `Secret`
      * `[✅]`   Collaborators: `Secret::expose` and halo2curves' wide reduction, real; fixtures `build_sample_uniform_scalar_payload` and `build_secret`
      * `[✅]`   Arrange: `build_sample_uniform_scalar_payload` with the uniform override `build_secret` holding `vector_bytes(UNIFORM_FIVE_HEX)`, whose last byte is five, so an input read as little-endian yields a different scalar
      * `[✅]`   Act: `Bls12381Halo2curvesScalar::sample_from_uniform_bytes(SampleUniformScalarParams, payload)`
      * `[✅]`   Assert: the success arm is extracted with `expect`; `success.scalar.expose().value` equals `Fr::from(5u64)`
    * `[✅]`   `sample_from_uniform_bytes_reduces_the_group_order_to_zero`
      * `[✅]`   Contract: a 64-byte uniform input → `Ok(SampleUniformScalarSuccessReturn { scalar })` holding the input reduced modulo the group order, inside a `Secret`
      * `[✅]`   Collaborators: `Secret::expose` and halo2curves' wide reduction, real; fixtures `build_sample_uniform_scalar_payload` and `build_secret`
      * `[✅]`   Arrange: `build_sample_uniform_scalar_payload` with the uniform override `build_secret` holding `vector_bytes(UNIFORM_GROUP_ORDER_HEX)`, whose reduction is zero and so differs from the input
      * `[✅]`   Act: `Bls12381Halo2curvesScalar::sample_from_uniform_bytes(SampleUniformScalarParams, payload)`
      * `[✅]`   Assert: the success arm is extracted with `expect`; `success.scalar.expose().value` equals `Fr::ZERO`
    * `[✅]`   `add_scalar_of_two_and_three_is_five`
      * `[✅]`   Contract: `payload.left` and `payload.right` → `Ok(AddScalarSuccessReturn { sum })` holding their sum modulo the group order
      * `[✅]`   Collaborators: halo2curves' `Fr` addition, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_add_scalar_payload`, and `build_bls12_381_halo2curves_scalar`
      * `[✅]`   Arrange: `build_add_scalar_payload` with the left override `build_bls12_381_halo2curves_scalar` with the value `Fr::from(2u64)` and the right override with the value `Fr::from(3u64)`
      * `[✅]`   Act: `add_scalar(AddScalarParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `Fr::from(5u64)`
    * `[✅]`   `add_scalar_reduces_modulo_the_group_order`
      * `[✅]`   Contract: `payload.left` and `payload.right` → `Ok(AddScalarSuccessReturn { sum })` holding their sum modulo the group order
      * `[✅]`   Collaborators: halo2curves' `Fr` addition, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_add_scalar_payload`, and `build_bls12_381_halo2curves_scalar`
      * `[✅]`   Arrange: `build_add_scalar_payload` with the left override `build_bls12_381_halo2curves_scalar` with the value `scalar_value(GROUP_ORDER_MINUS_ONE_HEX)` and the right override with the value `Fr::from(2u64)`, whose integer sum exceeds the group order
      * `[✅]`   Act: `add_scalar(AddScalarParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `Fr::from(1u64)`
    * `[✅]`   `mul_scalar_of_two_and_three_is_six`
      * `[✅]`   Contract: `payload.left` and `payload.right` → `Ok(MulScalarSuccessReturn { product })` holding their product modulo the group order
      * `[✅]`   Collaborators: halo2curves' `Fr` multiplication, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_mul_scalar_payload`, and `build_bls12_381_halo2curves_scalar`
      * `[✅]`   Arrange: `build_mul_scalar_payload` with the left override `build_bls12_381_halo2curves_scalar` with the value `Fr::from(2u64)` and the right override with the value `Fr::from(3u64)`
      * `[✅]`   Act: `mul_scalar(MulScalarParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `Fr::from(6u64)`
    * `[✅]`   `mul_scalar_reduces_modulo_the_group_order`
      * `[✅]`   Contract: `payload.left` and `payload.right` → `Ok(MulScalarSuccessReturn { product })` holding their product modulo the group order
      * `[✅]`   Collaborators: halo2curves' `Fr` multiplication, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_mul_scalar_payload`, and `build_bls12_381_halo2curves_scalar`
      * `[✅]`   Arrange: `build_mul_scalar_payload` with the left override `build_bls12_381_halo2curves_scalar` with the value `scalar_value(GROUP_ORDER_MINUS_ONE_HEX)` and the right override with the value `Fr::from(2u64)`, whose integer product exceeds the group order
      * `[✅]`   Act: `mul_scalar(MulScalarParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `scalar_value(GROUP_ORDER_MINUS_TWO_HEX)`
    * `[✅]`   `neg_scalar_of_one_is_the_group_order_minus_one`
      * `[✅]`   Contract: `payload.scalar` → `Ok(NegScalarSuccessReturn { negation })` holding the group order minus the scalar
      * `[✅]`   Collaborators: halo2curves' `Fr` negation, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_neg_scalar_payload`, and `build_bls12_381_halo2curves_scalar`
      * `[✅]`   Arrange: `build_neg_scalar_payload` with the scalar override `build_bls12_381_halo2curves_scalar` with the value `Fr::from(1u64)`
      * `[✅]`   Act: `neg_scalar(NegScalarParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.negation.value` equals `scalar_value(GROUP_ORDER_MINUS_ONE_HEX)`
    * `[✅]`   `neg_scalar_of_zero_is_zero`
      * `[✅]`   Contract: `payload.scalar` of zero → `Ok(NegScalarSuccessReturn { negation })` holding zero
      * `[✅]`   Collaborators: halo2curves' `Fr` negation, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_neg_scalar_payload`, and `build_bls12_381_halo2curves_scalar`
      * `[✅]`   Arrange: `build_neg_scalar_payload` with the scalar override `build_bls12_381_halo2curves_scalar` with the value `Fr::ZERO`
      * `[✅]`   Act: `neg_scalar(NegScalarParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.negation.value` equals `Fr::ZERO`
    * `[✅]`   `neg_g1_of_the_generator_is_the_negated_generator`
      * `[✅]`   Contract: `payload.point` → `Ok(NegG1SuccessReturn { negation })` holding `(x, p - y)` for a point `(x, y)`
      * `[✅]`   Collaborators: halo2curves' affine negation, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_neg_g1_payload`, and `build_bls12_381_halo2curves_g1`
      * `[✅]`   Arrange: `build_neg_g1_payload` with the point override `build_bls12_381_halo2curves_g1` at its default generator
      * `[✅]`   Act: `neg_g1(NegG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.negation.value` equals `eip_2537_negated_g1_generator()`
    * `[✅]`   `neg_g1_of_the_identity_is_the_identity`
      * `[✅]`   Contract: `payload.point` of the identity → `Ok(NegG1SuccessReturn { negation })` holding the identity
      * `[✅]`   Collaborators: halo2curves' affine negation, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_neg_g1_payload`, and `build_bls12_381_halo2curves_g1`
      * `[✅]`   Arrange: `build_neg_g1_payload` with the point override `build_bls12_381_halo2curves_g1` with the value `G1Affine::identity()`
      * `[✅]`   Act: `neg_g1(NegG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.negation.value` equals `G1Affine::identity()`
    * `[✅]`   `neg_g2_negates_the_y_coordinate_and_keeps_x`
      * `[✅]`   Contract: `payload.point` → `Ok(NegG2SuccessReturn { negation })` holding `(x, -y)` for a point `(x, y)`
      * `[✅]`   Collaborators: halo2curves' affine negation, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_neg_g2_payload`, and `build_bls12_381_halo2curves_g2`
      * `[✅]`   Arrange: `build_neg_g2_payload` with the point override `build_bls12_381_halo2curves_g2` at its default generator
      * `[✅]`   Act: `neg_g2(NegG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; the negation's `coordinates()` yields an `x` equal to the `x` of `eip_2537_g2_generator().coordinates()` and a `y` whose sum with the generator's `y` is zero under `is_zero()`
    * `[✅]`   `neg_g2_of_the_identity_is_the_identity`
      * `[✅]`   Contract: `payload.point` of the identity → `Ok(NegG2SuccessReturn { negation })` holding the identity
      * `[✅]`   Collaborators: halo2curves' affine negation, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_neg_g2_payload`, and `build_bls12_381_halo2curves_g2`
      * `[✅]`   Arrange: `build_neg_g2_payload` with the point override `build_bls12_381_halo2curves_g2` with the value `G2Affine::identity()`
      * `[✅]`   Act: `neg_g2(NegG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.negation.value` equals `G2Affine::identity()`
    * `[✅]`   `is_identity_g1_is_true_for_the_identity`
      * `[✅]`   Contract: `payload.point` → `Ok(IsIdentityG1SuccessReturn { is_identity })`, true exactly for the identity
      * `[✅]`   Collaborators: halo2curves, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_is_identity_g1_payload`, and `build_bls12_381_halo2curves_g1`
      * `[✅]`   Arrange: `build_is_identity_g1_payload` with the point override `build_bls12_381_halo2curves_g1` with the value `G1Affine::identity()`
      * `[✅]`   Act: `is_identity_g1(IsIdentityG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_identity` is true
    * `[✅]`   `is_identity_g1_is_false_for_the_generator`
      * `[✅]`   Contract: `payload.point` → `Ok(IsIdentityG1SuccessReturn { is_identity })`, true exactly for the identity
      * `[✅]`   Collaborators: halo2curves, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_is_identity_g1_payload`, and `build_bls12_381_halo2curves_g1`
      * `[✅]`   Arrange: `build_is_identity_g1_payload` with the point override `build_bls12_381_halo2curves_g1` at its default generator
      * `[✅]`   Act: `is_identity_g1(IsIdentityG1Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_identity` is false
    * `[✅]`   `is_identity_g2_is_true_for_the_identity`
      * `[✅]`   Contract: `payload.point` → `Ok(IsIdentityG2SuccessReturn { is_identity })`, true exactly for the identity
      * `[✅]`   Collaborators: halo2curves, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_is_identity_g2_payload`, and `build_bls12_381_halo2curves_g2`
      * `[✅]`   Arrange: `build_is_identity_g2_payload` with the point override `build_bls12_381_halo2curves_g2` with the value `G2Affine::identity()`
      * `[✅]`   Act: `is_identity_g2(IsIdentityG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_identity` is true
    * `[✅]`   `is_identity_g2_is_false_for_the_generator`
      * `[✅]`   Contract: `payload.point` → `Ok(IsIdentityG2SuccessReturn { is_identity })`, true exactly for the identity
      * `[✅]`   Collaborators: halo2curves, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_is_identity_g2_payload`, and `build_bls12_381_halo2curves_g2`
      * `[✅]`   Arrange: `build_is_identity_g2_payload` with the point override `build_bls12_381_halo2curves_g2` at its default generator
      * `[✅]`   Act: `is_identity_g2(IsIdentityG2Params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_identity` is false
    * `[✅]`   `pairing_product_of_no_terms_is_the_target_group_identity`
      * `[✅]`   Contract: an empty `payload.terms` → `Ok(PairingProductSuccessReturn { product })` holding the target group's identity
      * `[✅]`   Collaborators: halo2curves' multi-Miller loop, final exponentiation, and target-group exponentiation, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing` and `build_pairing_product_payload`
      * `[✅]`   Arrange: `build_pairing_product_payload` at its default of no terms
      * `[✅]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `Gt::identity()`
    * `[✅]`   `pairing_product_of_the_generators_is_not_the_target_group_identity`
      * `[✅]`   Contract: a pairing of the generators → `Ok(PairingProductSuccessReturn { product })` holding a value other than the target group's identity
      * `[✅]`   Collaborators: halo2curves' multi-Miller loop, final exponentiation, and target-group exponentiation, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_pairing_product_payload`, and `build_pairing_product_term`
      * `[✅]`   Arrange: `build_pairing_product_payload` with the terms override holding one `build_pairing_product_term` at its default generators
      * `[✅]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` is not equal to `Gt::identity()`
    * `[✅]`   `pairing_product_is_bilinear`
      * `[✅]`   Contract: a term whose first-group element is twice the generator → `Ok(PairingProductSuccessReturn { product })` holding the generators' pairing raised to the power two
      * `[✅]`   Collaborators: halo2curves' multi-Miller loop, final exponentiation, and target-group exponentiation, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_pairing_product_payload`, `build_pairing_product_term`, and `build_bls12_381_halo2curves_g1`; the expectation is `definition_value_power`, the identifier's definition raised to a power in the fixtures, independent of the correction `pairing_product` applies
      * `[✅]`   Arrange: `build_pairing_product_payload` with the terms override holding one `build_pairing_product_term` with the first-group override `build_bls12_381_halo2curves_g1` with the value `eip_2537_doubled_g1_generator()` over the second-group generator
      * `[✅]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value.inner()` equals `&definition_value_power(eip_2537_g1_generator(), eip_2537_g2_generator(), 2)`
    * `[✅]`   `pairing_product_multiplies_its_terms`
      * `[✅]`   Contract: `payload.terms` → `Ok(PairingProductSuccessReturn { product })` holding the product of each term's pairing
      * `[✅]`   Collaborators: halo2curves' multi-Miller loop, final exponentiation, and target-group exponentiation, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_pairing_product_payload`, and `build_pairing_product_term`; the expectation is `definition_value_power`
      * `[✅]`   Arrange: `build_pairing_product_payload` with the terms override holding two `build_pairing_product_term` values at their default generators, so a product over one term differs from the expected square
      * `[✅]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value.inner()` equals `&definition_value_power(eip_2537_g1_generator(), eip_2537_g2_generator(), 2)`
    * `[✅]`   `pairing_product_of_a_pairing_and_its_first_group_negation_is_the_target_group_identity`
      * `[✅]`   Contract: `payload.terms` whose pairings multiply to the target group's identity → `Ok(PairingProductSuccessReturn { product })` holding the identity
      * `[✅]`   Collaborators: halo2curves' multi-Miller loop, final exponentiation, and target-group exponentiation, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_pairing_product_payload`, `build_pairing_product_term`, and `build_bls12_381_halo2curves_g1`
      * `[✅]`   Arrange: `build_pairing_product_payload` with the terms override holding two `build_pairing_product_term` values: the generators, and the first-group override `build_bls12_381_halo2curves_g1` with the value `eip_2537_negated_g1_generator()` over the second-group generator
      * `[✅]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `Gt::identity()`
    * `[✅]`   `pairing_product_of_the_generators_equals_the_definition`
      * `[✅]`   Contract: a pairing of the generators → `Ok(PairingProductSuccessReturn { product })` holding the Miller loop's value raised to the exact exponent `(p^12 - 1) / r`, the value `TargetGroupEncodingIdentifier::Bls12381V1` defines, which is the library's reduced pairing with the cube removed
      * `[✅]`   Collaborators: halo2curves' multi-Miller loop, final exponentiation, and target-group exponentiation, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_pairing_product_payload`, and `build_pairing_product_term`; the expectation is `definition_value`, the Miller loop over the generators raised to `definition_exponent()` built by `num-bigint`, independent of the correction `pairing_product` applies
      * `[✅]`   Arrange: `build_pairing_product_payload` with the terms override holding one `build_pairing_product_term` at its default generators
      * `[✅]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value.inner()` equals `&definition_value(eip_2537_g1_generator(), eip_2537_g2_generator())`
    * `[✅]`   `encode_gt_writes_the_target_group_identity_with_c0_c0_c0_first`
      * `[✅]`   Contract: a target-group value → `Ok(EncodeGtSuccessReturn { bytes })` holding its twelve coefficients in tower order, `c0.c0.c0` first, each 48 bytes big-endian, inside a `Secret`
      * `[✅]`   Collaborators: halo2curves' `to_repr`, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_encode_gt_payload`, and `build_bls12_381_halo2curves_gt`; the expectation is `gt_identity_encoding()`
      * `[✅]`   Arrange: `build_encode_gt_payload` with the value override `build_bls12_381_halo2curves_gt` with the value `Gt::identity()`
      * `[✅]`   Act: `encode_gt(EncodeGtParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.bytes.expose()` binds to a `Bls12381Halo2curvesEncodedGt` typed binding; its bytes equal `gt_identity_encoding()` as a byte slice
    * `[✅]`   `encode_gt_writes_the_published_pairing_of_the_generators`
      * `[✅]`   Contract: a target-group value → `Ok(EncodeGtSuccessReturn { bytes })` holding its twelve coefficients in tower order, `c0.c0.c0` through `c1.c2.c1`, each 48 bytes big-endian, inside a `Secret`
      * `[✅]`   Collaborators: halo2curves' `to_repr`, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_encode_gt_payload`, and `build_bls12_381_halo2curves_gt`; the expectation is `CFRG_GENERATOR_PAIRING_HEX`, the pairing of the base points published in the CFRG draft's test-vector appendix
      * `[✅]`   Arrange: `build_encode_gt_payload` with the value override `build_bls12_381_halo2curves_gt` with the value `corrected_generator_pairing()`, the identifier's value for the generators, whose twelve coefficients are distinct, so any exchange of two positions changes the bytes
      * `[✅]`   Act: `encode_gt(EncodeGtParams, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; the bytes of `success.bytes.expose()` equal `vector_bytes(CFRG_GENERATOR_PAIRING_HEX)` as a byte slice
    * `[✅]`   `scalar_field_order_is_the_group_order`
      * `[✅]`   Contract: any call → `Ok(ScalarFieldOrderSuccessReturn { bytes })` holding the group order's 32 big-endian bytes
      * `[✅]`   Collaborators: halo2curves' `Fr` negation and `to_repr`, run for real as the vendor; fixture `build_bls12_381_halo2curves_pairing`; `ScalarFieldOrderParams` and `ScalarFieldOrderPayload` by their production values
      * `[✅]`   Arrange: `build_bls12_381_halo2curves_pairing()`
      * `[✅]`   Act: `scalar_field_order(ScalarFieldOrderParams, ScalarFieldOrderPayload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.bytes` equals `vector_bytes(GROUP_ORDER_HEX)`
    * `[✅]`   `g1_outside_subgroup_encoding_encodes_an_on_curve_point_outside_the_subgroup`
      * `[✅]`   Contract: any call → `Ok(G1OutsideSubgroupEncodingSuccessReturn { bytes: Some(bytes) })` holding the precompile encoding of an on-curve point outside the prime-order subgroup, in this concrete's `Bls12381Halo2curvesEncodedG1`
      * `[✅]`   Collaborators: halo2curves' curve and torsion checks, run for real as the vendor; fixture `build_bls12_381_halo2curves_pairing`; `G1OutsideSubgroupEncodingParams` and `G1OutsideSubgroupEncodingPayload` by their production values; the point read back by `g1_point_from_encoding`
      * `[✅]`   Arrange: `build_bls12_381_halo2curves_pairing()`
      * `[✅]`   Act: `g1_outside_subgroup_encoding(G1OutsideSubgroupEncodingParams, G1OutsideSubgroupEncodingPayload)`
      * `[✅]`   Assert: the success arm is extracted with `expect`; `success.bytes` is `Some` and its content binds to a `Bls12381Halo2curvesEncodedG1` typed binding; `g1_point_from_encoding` reads a point from its bytes, which is `Some` because the point is on the curve, and that point's `to_curve().is_torsion_free()` is false
    * `[✅]`   `g1_outside_subgroup_encoding_uses_the_least_first_coordinate`
      * `[✅]`   Contract: any call → the encoded point's `x` is the least `x` of an on-curve point outside the subgroup in an ascending search
      * `[✅]`   Collaborators: halo2curves' curve and torsion checks, run for real as the vendor; fixture `build_bls12_381_halo2curves_pairing`; `G1OutsideSubgroupEncodingParams` and `G1OutsideSubgroupEncodingPayload` by their production values; the expectation is `g1_outside_subgroup_bytes()`, the ascending search in the fixtures
      * `[✅]`   Arrange: `build_bls12_381_halo2curves_pairing()`
      * `[✅]`   Act: `g1_outside_subgroup_encoding(G1OutsideSubgroupEncodingParams, G1OutsideSubgroupEncodingPayload)`
      * `[✅]`   Assert: the success arm is extracted with `expect`; the first 64 bytes of the encoding, the `x` coordinate, equal the first 64 bytes of `g1_outside_subgroup_bytes()`
    * `[✅]`   `g1_outside_subgroup_encoding_takes_the_lesser_root`
      * `[✅]`   Contract: any call → the encoded point's `y` is the lesser of the two roots sharing its `x`
      * `[✅]`   Collaborators: halo2curves' curve and torsion checks, run for real as the vendor; fixture `build_bls12_381_halo2curves_pairing`; `G1OutsideSubgroupEncodingParams` and `G1OutsideSubgroupEncodingPayload` by their production values; the expectation is `base_field_modulus()`
      * `[✅]`   Arrange: `build_bls12_381_halo2curves_pairing()`
      * `[✅]`   Act: `g1_outside_subgroup_encoding(G1OutsideSubgroupEncodingParams, G1OutsideSubgroupEncodingPayload)`
      * `[✅]`   Assert: the success arm is extracted with `expect`; the last 48 bytes of the encoding's `y` coordinate, bytes 80 through 127, read as a `BigUint`, are not greater than `base_field_modulus()` minus that value
    * `[✅]`   `g2_outside_subgroup_encoding_encodes_an_on_curve_point_outside_the_subgroup`
      * `[✅]`   Contract: any call → `Ok(G2OutsideSubgroupEncodingSuccessReturn { bytes })` holding the precompile encoding of an on-curve point outside the prime-order subgroup, in this concrete's `Bls12381Halo2curvesEncodedG2`
      * `[✅]`   Collaborators: halo2curves' curve and torsion checks, run for real as the vendor; fixture `build_bls12_381_halo2curves_pairing`; `G2OutsideSubgroupEncodingParams` and `G2OutsideSubgroupEncodingPayload` by their production values; the point read back by `g2_point_from_encoding`
      * `[✅]`   Arrange: `build_bls12_381_halo2curves_pairing()`
      * `[✅]`   Act: `g2_outside_subgroup_encoding(G2OutsideSubgroupEncodingParams, G2OutsideSubgroupEncodingPayload)`
      * `[✅]`   Assert: the success arm is extracted with `expect`; `success.bytes` binds to a `Bls12381Halo2curvesEncodedG2` typed binding; `g2_point_from_encoding` reads a point from its bytes, which is `Some` because the point is on the curve, and that point's `to_curve().is_torsion_free()` is false
    * `[✅]`   `g2_outside_subgroup_encoding_has_a_real_first_coordinate_and_the_lesser_root`
      * `[✅]`   Contract: any call → the encoded point's first coordinate is `(c0, 0)` with the least `c0` of an on-curve point outside the subgroup, and its `y` is the lesser of the two roots, compared by `y.c1` and then `y.c0`
      * `[✅]`   Collaborators: halo2curves' curve and torsion checks, run for real as the vendor; fixture `build_bls12_381_halo2curves_pairing`; `G2OutsideSubgroupEncodingParams` and `G2OutsideSubgroupEncodingPayload` by their production values; the expectations are `g2_outside_subgroup_bytes()`, the ascending search in the fixtures, and `base_field_modulus()`
      * `[✅]`   Arrange: `build_bls12_381_halo2curves_pairing()`
      * `[✅]`   Act: `g2_outside_subgroup_encoding(G2OutsideSubgroupEncodingParams, G2OutsideSubgroupEncodingPayload)`
      * `[✅]`   Assert: the success arm is extracted with `expect`; the encoding's `x.c1` coordinate, bytes 64 through 127, is all zero; its `x.c0` coordinate, bytes 0 through 63, equals the first 64 bytes of `g2_outside_subgroup_bytes()`; the pair `y.c1`, `y.c0`, each the last 48 bytes of its 64-byte coordinate read as a `BigUint`, is not greater than the pair of their negations, each `base_field_modulus()` minus the coefficient, reduced to zero where the coefficient is zero

  * `[✅]`   `construction`
    * `[✅]`   `Bls12381Halo2curvesPairing::try_new` is the concrete's only producer, computing its one field, and its only caller is the pairing factory, which obtains the declaration through `pairing/lookup_pairing_declaration` and completes `pairing/admit_pairing_composition` before calling it and before its correction computation
    * `[✅]`   A group element or scalar is produced only by the adapter's generators, arithmetic, and decoders, or by the scalar's sampling bound; a target-group value only by `pairing_product`; an encoded value only by the adapter's encoders and reference methods; no consumer constructs one from library values

  * `[✅]`   `adapters/pairing/src/bls12_381_halo2curves/mod.rs`
    * `[✅]`   Module declarations: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, `pub(crate) mod provides;`, and `#[cfg(test)] mod test;`
    * `[✅]`   Imports: from `crate::factory::provides` every params, payload, return, success-return, and error type of the three traits and the sampling bound, `IPairingAdapter`, `IPairingArithmetic`, `IPairingReference`, and `ISampleUniformScalar`; `core::array::from_fn`; `core::hint::black_box`; `core::iter::successors`; `domain::{Secret, SecretConstructorParams}`; `halo2curves::bls12381::{Bls12381, Fq, Fq2, Fr, G1Affine, G2Affine, Gt}`; `halo2curves::ff::{Field, FromUniformBytes, PrimeField}`; `halo2curves::group::{Curve, Group, cofactor::CofactorGroup, prime::PrimeCurveAffine}`; `halo2curves::msm::msm_best`; `halo2curves::pairing::{MillerLoopResult, MultiMillerLoop}`; `halo2curves::{Coordinates, CurveAffine}`; from `interface` every type it declares; `zeroize::{Zeroize, ZeroizeOnDrop}`
    * `[✅]`   `fn decode_coordinate(bytes: &[u8]) -> Option<Fq>`, private: `None` when any of `bytes[..16]` is nonzero; otherwise `bytes[16..64]` copied into a `[u8; 48]`, reversed, and read by `Fq::from_repr`, returned as `Option::<Fq>::from` of the `CtOption`
    * `[✅]`   `impl Bls12381Halo2curvesPairing` holding `pub fn try_new(params: Bls12381Halo2curvesPairingConstructorParams) -> Bls12381Halo2curvesPairingTryNewReturn` realizing the interaction spec's construct branch; `DECLARATION` lives in `interface.rs`, the interface owning the real declaration
    * `[✅]`   `impl IPairingAdapter for Bls12381Halo2curvesPairing` with `type Scalar = Bls12381Halo2curvesScalar;`, `type G1 = Bls12381Halo2curvesG1;`, `type G2 = Bls12381Halo2curvesG2;`, `type EncodedG1 = Bls12381Halo2curvesEncodedG1;`, `type EncodedG2 = Bls12381Halo2curvesEncodedG2;`, and `type EncodedScalar = Bls12381Halo2curvesEncodedScalar;`, every method realizing its branches in the interaction spec, the decoders reading each coordinate through `decode_coordinate`
    * `[✅]`   `impl IPairingArithmetic for Bls12381Halo2curvesPairing` with `type Gt = Bls12381Halo2curvesGt;` and `type EncodedGt = Bls12381Halo2curvesEncodedGt;`, every method realizing its branches in the interaction spec, `pairing_product` multiplying the `Gt` by `self.reduced_pairing_correction`
    * `[✅]`   `impl IPairingReference for Bls12381Halo2curvesPairing`, every method realizing its branches in the interaction spec, each search by `find_map` with the `SearchExhausted` return by `let … else`
    * `[✅]`   `impl AsRef<[u8]>` for `Bls12381Halo2curvesEncodedG1`, `Bls12381Halo2curvesEncodedG2`, `Bls12381Halo2curvesEncodedScalar`, and `Bls12381Halo2curvesEncodedGt`, each returning `&self.bytes`; `impl Zeroize` for `Bls12381Halo2curvesEncodedScalar` and `Bls12381Halo2curvesEncodedGt`, each calling `self.bytes.zeroize()`; no implementation returns a mutable reference to the bytes
    * `[✅]`   `impl Zeroize` and `impl Drop` for `Bls12381Halo2curvesScalar`, setting `value` to `Fr::ZERO` and calling `black_box(&self.value)`, and for `Bls12381Halo2curvesG1`, `Bls12381Halo2curvesG2`, and `Bls12381Halo2curvesGt`, each setting `value` to its type's identity and calling `black_box(&self.value)`; `impl ZeroizeOnDrop for Bls12381Halo2curvesScalar {}`, marking the drop behavior the sampling bound requires
    * `[✅]`   `impl ISampleUniformScalar for Bls12381Halo2curvesScalar` with `const UNIFORM_BYTES_LENGTH: usize = 64;` and `sample_from_uniform_bytes` realizing its branches in the interaction spec, the scalar moved into a `Secret` by `let Ok(scalar) = Secret::try_new(SecretConstructorParams { value });`
    * `[✅]`   No `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`; the correction is computed in `try_new` and read from no configuration; neither the order, the order minus two, nor a point is written as a literal

  * `[✅]`   `adapters/pairing/src/bls12_381_halo2curves/provides.rs`
    * `[✅]`   `pub(crate) use super::interface::*;` and `#[cfg(test)] pub(crate) use super::mock::*;`, exposing every item of the concrete's interface to the crate and, in the crate's own test build, the concrete's builders, fixtures, and helpers, and nothing beyond them

  * `[✅]`   `directionality`
    * `[✅]`   `bls12_381_halo2curves` depends on the `factory` module's surface through `crate::factory::provides`, on `domain`, on `zeroize`, and on `halo2curves`; it depends on no other concrete, and `pairing/lookup_pairing_declaration` reaches its interface-held declaration through the family's private path, the selection and composition admissions consume the corrected member identifier and declared set, and the factory completes those admissions before constructing this member; its increments to `factory/interface.rs` and `factory/mock.rs` introduce no dependency outside the pairing family; among repository crates the crate depends on `crates/domain` alone at runtime
    * `[✅]`   `pairing/factory` constructs this concrete, the family form's recorded cycle, and proves its target-group encoding, order, and outside-the-subgroup encodings equal the arkworks concrete's

  * `[✅]`   `requirements`
    * `[✅]`   `halo2curves` is named nowhere in the crate outside the halo2curves concretes
    * `[✅]`   This concrete returns its own distinct fixed-width encoded types for the first group, the second group, the scalar, and the target group through the family's associated types
    * `[✅]`   `cargo check --all-targets --all-features`, `cargo fmt --check`, and `cargo deny check` complete without error; `cargo clippy --all-targets --all-features` reports nothing beyond the library target's unused-item warnings for the pairing concretes, which `pairing/factory` constructs
    * `[✅]`   Every test in `bls12_381_halo2curves/test.rs` passes (CR-04, CR-08, CR-09, and CR-10 on BLS12-381 over halo2curves; CR-05 for the sampled scalar; CR-07 for the clearing; CR-11 for the order)
    * `[✅]`   The declaration compile-time assertion compiles, the `ZeroizeOnDrop` compile-time assertion compiles, `bls12_381_halo2curves_scalar_zeroize_clears_its_value` passes (CR-07), and `pairing_product_of_the_generators_equals_the_definition`, `encode_gt_writes_the_published_pairing_of_the_generators`, and `reduced_pairing_correction_inverts_three` pass (CR-10, the target-group value fixed by the identifier's definition, executed in the test, and its serialization by the standard's published value)
    * `[✅]`   `pairing_concretes_declares_only_real_members` and `every_declared_identifier_spells_a_distinct_name` pass, proving the declaration carries `PairingConcrete::Bls12381Halo2curves` rather than text and `PAIRING_CONCRETES` contains all four real members with distinct spellings and no mock member
    * `[✅]`   `target_group_encoding_identifiers_declares_only_real_values` and `no_value_identifier_declares_a_mock_only_variant` pass, proving `TARGET_GROUP_ENCODING_IDENTIFIERS` contains both real encodings with distinct spellings
    * `[✅]`   `bls12_381_halo2curves_declared_selection_builds_without_configuration` passes, proving the new declared-selection arm returns the real selection with no configuration, and this concrete carries no settings module, settings node, or settings table
    * `[✅]`   The factory obtains this member's declaration through `pairing/lookup_pairing_declaration` and completes `pairing/admit_pairing_composition` before constructing the concrete
    * `[✅]`   Code outside `adapters/pairing` naming `Bls12381Halo2curvesPairing` or anything under `bls12_381_halo2curves` fails to compile; the crate's public surface is the `factory` module's `provides`

* `[✅]`   `pairing/lookup_pairing_declaration` **Pairing declaration lookup returning the declaration of the member a `PairingConcrete` identifier names, read from the member's interface-held constant without constructing the member; creates the family-owned `lookup_pairing_declaration` module of `adapters/pairing`**

  * `[✅]`   `objective`
    * `[✅]`   Problem: every level that holds a `PairingConcrete` identifier must read what the member declares, its curve, its verifier-group arithmetic, its precompile encoding, its target-group encoding identifier, and its versions, before the member is constructed, so a composition is admitted from declarations alone; the family provides one function from the identifier to the declaration and no caller restates a member's declared value from its own knowledge of the member (the dependency map's `pairing/lookup_pairing_declaration` row; the technical requirements' pairing family register row; the product requirements' declaration-lookup position)
    * `[✅]`   Functional: given a `PairingConcrete` identifier, the function returns the `PairingDeclaration` that member declares: for each real member the value of that concrete's `DECLARATION` constant, and, under the `mocks` feature, for the mock member `MOCK_I_PAIRING_ADAPTER_DECLARATION` with its `concrete` field set to the identifier looked up
    * `[✅]`   Functional: the function holds no adapter and constructs no member; its deps are fieldless, its params carry the identifier alone, and its payload is fieldless, so the declaration is read with no concrete constructed
    * `[✅]`   Functional: the success return holds the declaration and the error arm is `Infallible`, since the exhaustive `match` over `PairingConcrete` returns a declaration for every member
    * `[✅]`   Non-functional: a further concrete adds its arm, its import, its spec branch, and its unit block to this module; no module outside the pairing family changes (the family measure)
    * `[✅]`   Non-functional: no declaration value is written as a literal in `mod.rs`; every field of a returned declaration is read from a concrete's `DECLARATION` or from `MOCK_I_PAIRING_ADAPTER_DECLARATION`

  * `[✅]`   `role`
    * `[✅]`   Adapter-ring family-owned function of the pairing family, beside its factory: the declaration lookup the family's selection admission, composition admission, and factory read declarations through
    * `[✅]`   Does not construct a concrete, call a concrete's `try_new`, or read an instance
    * `[✅]`   Does not admit a selection, narrow a settings document, or compare declarations; the family's selection admission and composition admission do
    * `[✅]`   Does not read configuration and does not carry a settings module, settings node, or settings table
    * `[✅]`   Does not carry a commit; the milestone's commit belongs to the family's factory node
    * `[✅]`   Is a public entry, since the crate barrel re-exports this module's `provides`; it hosts the public integration test of its own boundary

  * `[✅]`   `module`
    * `[✅]`   Bounded context: the `lookup_pairing_declaration` module of the `adapters/pairing` crate, holding the function, its deps, params, payload, success return, return alias, and function type, its builders and function mock, its interaction spec, and its unit test
    * `[✅]`   The node's files: `adapters/pairing/src/lookup_pairing_declaration/interface.rs`, `interaction.spec.md`, `mock.rs`, `test.rs`, `mod.rs`, and `provides.rs`, the barrel `adapters/pairing/src/lib.rs`, and the public integration test `adapters/pairing/tests/lookup_pairing_declaration_integration_test.rs`
    * `[✅]`   Outside: the member and value identifiers, the declaration type, the declared sets, and the family's mock, which `pairing/bn254_arkworks` authors in the `factory` module; each concrete's `DECLARATION` constant, which each concrete's interface holds; the selection admission, the composition admission, and the factory

  * `[✅]`   `deps`
    * `[✅]`   `pairing/bn254_arkworks`, provider of the `factory` module's surface, through `crate::factory::provides`: `PairingConcrete` with the variant `Bn254Arkworks` and, under `#[cfg(any(test, feature = "mocks"))]`, the variant `Mock(MockIPairingAdapterFailureMode)`; `PairingDeclaration` with `concrete`, `curve`, `verifier_group_arithmetic`, `precompile_encoding`, `target_group_encoding`, `adapter_version`, and `interface_version`; `PairingCurve::Bn254`, `VerifierGroupArithmetic::FirstGroupOnly`, `PrecompileEncoding::Eip196Eip197`, `TargetGroupEncodingIdentifier::Bn254V1`, and `PAIRING_CONCRETES`; direction inward, a family function on the family's factory module
    * `[✅]`   `pairing/bn254_halo2curves`, through `crate::factory::provides`: the variant `PairingConcrete::Bn254Halo2curves`, its increment to the member identifier
    * `[✅]`   `pairing/bls12_381_arkworks`, through `crate::factory::provides`: the variants `PairingConcrete::Bls12381Arkworks`, `PairingCurve::Bls12381`, `VerifierGroupArithmetic::BothGroups`, `PrecompileEncoding::Eip2537`, and `TargetGroupEncodingIdentifier::Bls12381V1`, its increments to the member and value identifiers
    * `[✅]`   `pairing/bls12_381_halo2curves`, through `crate::factory::provides`: the variant `PairingConcrete::Bls12381Halo2curves`, its increment to the member identifier
    * `[✅]`   `pairing/bn254_arkworks`, under the `mocks` feature and in the crate's own test build, through `crate::factory::provides`: `MOCK_I_PAIRING_ADAPTER_DECLARATION`, `MockIPairingAdapterFailureMode`, `PairingDeclarationOverrides`, and `build_pairing_declaration`
    * `[✅]`   `pairing/bn254_arkworks`, through `crate::bn254_arkworks::provides`: `Bn254ArkworksPairing`, whose inherent `DECLARATION` constant is read without an instance; direction inward through the family's private path
    * `[✅]`   `pairing/bn254_halo2curves`, through `crate::bn254_halo2curves::provides`: `Bn254Halo2curvesPairing`, whose inherent `DECLARATION` constant is read without an instance
    * `[✅]`   `pairing/bls12_381_arkworks`, through `crate::bls12_381_arkworks::provides`: `Bls12381ArkworksPairing`, whose inherent `DECLARATION` constant is read without an instance
    * `[✅]`   `pairing/bls12_381_halo2curves`, through `crate::bls12_381_halo2curves::provides`: `Bls12381Halo2curvesPairing`, whose inherent `DECLARATION` constant is read without an instance
    * `[✅]`   `core::convert::Infallible`, standard library, the error arm of the return
    * `[✅]`   Reverse dependencies: `pairing/admit_pairing_selection`, `pairing/admit_pairing_composition`, and `pairing/factory`, through this module's `provides`
    * `[✅]`   No reverse dependency from a concrete or from the `factory` module on this module

  * `[✅]`   `context_slice`
    * `[✅]`   From the `factory` module: the identifier `PairingConcrete`, matched exhaustively, and the struct `PairingDeclaration`, constructed from fields in the mock branch
    * `[✅]`   From each concrete's interface: the associated constant `DECLARATION: PairingDeclaration`, read as a value; no method, constructor, or field of a concrete is read
    * `[✅]`   From the `factory` module's mock, under the `mocks` feature: `MOCK_I_PAIRING_ADAPTER_DECLARATION`, whose `curve`, `verifier_group_arithmetic`, `precompile_encoding`, `target_group_encoding`, `adapter_version`, and `interface_version` fields are read, and `build_pairing_declaration` with `PairingDeclarationOverrides`, the builder of the declaration the success-return builder composes

  * `[✅]`   `adapters/pairing/src/lookup_pairing_declaration/interface.rs`
    * `[✅]`   Imports `core::convert::Infallible`; `PairingConcrete` and `PairingDeclaration` from `crate::factory::provides`; declares nothing beyond the items below
    * `[✅]`   `LookupPairingDeclarationDeps`, the fieldless unit struct `pub struct LookupPairingDeclarationDeps;`, the function's deps slot
    * `[✅]`   `LookupPairingDeclarationParams`, a struct with the field `pub concrete: PairingConcrete`, the identifier of the member whose declaration is read; the identifier is a selection, since no received type or adapter determines which member is looked up
    * `[✅]`   `LookupPairingDeclarationPayload`, the fieldless unit struct `pub struct LookupPairingDeclarationPayload;`
    * `[✅]`   `LookupPairingDeclarationSuccessReturn`, a struct with the field `pub declaration: PairingDeclaration`
    * `[✅]`   `LookupPairingDeclarationReturn`, the alias `Result<LookupPairingDeclarationSuccessReturn, Infallible>`, the error arm uninhabited because every member has a declaration
    * `[✅]`   `LookupPairingDeclarationFn`, the alias `fn(&LookupPairingDeclarationDeps, LookupPairingDeclarationParams, LookupPairingDeclarationPayload) -> LookupPairingDeclarationReturn`
    * `[✅]`   No derives on any type in this file; the family's declaration and identifier types are owned by the `factory` module and are not redeclared here

  * `[✅]`   `adapters/pairing/src/lookup_pairing_declaration/interaction.spec.md`
    * `[✅]`   The title `` # `lookup_pairing_declaration` — interaction spec `` and one opening sentence stating the file as the branch contract for the `lookup_pairing_declaration` module of the `pairing` crate, the family's declaration lookup returning a member's declaration without constructing the member, each branch stating condition, decision, dependency call, and the exact return outcome; then one `##` section headed by the full signature `lookup_pairing_declaration(deps: &LookupPairingDeclarationDeps, params: LookupPairingDeclarationParams, payload: LookupPairingDeclarationPayload) -> LookupPairingDeclarationReturn`, holding a table with the columns `Branch`, `Condition`, `Decision`, `Dependency call`, and `Outcome`, in the branch order below, then `Ordering and edges`
    * `[✅]`   `bn254_arkworks`: condition `params.concrete` is `PairingConcrete::Bn254Arkworks`; decision the exhaustive `match` on `params.concrete`; dependency call none, the constant `Bn254ArkworksPairing::DECLARATION` read as a value; outcome `Ok(LookupPairingDeclarationSuccessReturn { declaration })` holding that constant's value
    * `[✅]`   `bn254_halo2curves`: condition `params.concrete` is `PairingConcrete::Bn254Halo2curves`; decision the same `match`; dependency call none, the constant `Bn254Halo2curvesPairing::DECLARATION` read as a value; outcome `Ok(LookupPairingDeclarationSuccessReturn { declaration })` holding that constant's value
    * `[✅]`   `bls12_381_arkworks`: condition `params.concrete` is `PairingConcrete::Bls12381Arkworks`; decision the same `match`; dependency call none, the constant `Bls12381ArkworksPairing::DECLARATION` read as a value; outcome `Ok(LookupPairingDeclarationSuccessReturn { declaration })` holding that constant's value
    * `[✅]`   `bls12_381_halo2curves`: condition `params.concrete` is `PairingConcrete::Bls12381Halo2curves`; decision the same `match`; dependency call none, the constant `Bls12381Halo2curvesPairing::DECLARATION` read as a value; outcome `Ok(LookupPairingDeclarationSuccessReturn { declaration })` holding that constant's value
    * `[✅]`   `mock`, present only under `#[cfg(any(test, feature = "mocks"))]`: condition `params.concrete` is `PairingConcrete::Mock(_)`; decision the same `match`; dependency call none, `MOCK_I_PAIRING_ADAPTER_DECLARATION` read as a value; outcome `Ok(LookupPairingDeclarationSuccessReturn { declaration })` where `declaration.concrete` is `params.concrete`, the identifier the caller looked up, and every other field is that field of `MOCK_I_PAIRING_ADAPTER_DECLARATION`
    * `[✅]`   `Ordering and edges`, a bulleted section: the function is a single exhaustive `match` with one arm per member of `PairingConcrete`, so a variant without an arm does not compile; the function constructs no concrete and reads no instance, so the declaration of a member is read before the member exists; `deps` and `payload` carry no value and are not read; the error arm has no branch, `Infallible` being uninhabited; a further concrete adds one arm in its own increment
    * `[✅]`   `Own entries`, a section of the spec: `declaration_names_the_member_looked_up` — condition `params.concrete` is any member of `PAIRING_CONCRETES`; outcome `Ok` whose `declaration.concrete` is that member; input variation every member of the declared set, so a lookup that returns one fixed member's declaration fails; edge that must survive composition: the declaration is read through the crate's public surface with no concrete constructed. `mock_declaration_names_the_identifier_looked_up`, present only under `#[cfg(feature = "mocks")]` — condition `params.concrete` is `PairingConcrete::Mock(mode)` with a mode other than `Succeeds`; outcome `Ok` whose `declaration.concrete` is `PairingConcrete::Mock(mode)`; input variation the mode; edge: the mock member is absent from `PAIRING_CONCRETES` and reached only by naming it
    * `[✅]`   `Callee dispositions`, a section of the spec: the declaration constants of the concretes this node reads state no integration entry, so no entry is absorbed or carried
    * `[✅]`   `Route`, a section of the spec: `lookup_pairing_declaration` reads the `DECLARATION` constant of the concrete the identifier names, or `MOCK_I_PAIRING_ADAPTER_DECLARATION` for the mock member; no outer-edge collaborator is on the route
    * `[✅]`   `Public surface`, a section of the spec: an outside caller invokes `lookup_pairing_declaration` with fieldless deps, params carrying a `PairingConcrete`, and a fieldless payload, each built through the official mocks, and observes the `Ok` declaration's `concrete`; the entries `declaration_names_the_member_looked_up` and `mock_declaration_names_the_identifier_looked_up` are proven by the public integration test

  * `[✅]`   `adapters/pairing/src/lookup_pairing_declaration/mock.rs`
    * `[✅]`   Module-level `#![allow(dead_code, clippy::too_many_arguments)]`; imports from `super::interface` every type the items below name; `PairingConcrete` and `PairingDeclaration` from `crate::factory::provides`; and `build_pairing_declaration` and `PairingDeclarationOverrides` from `crate::factory::provides`
    * `[✅]`   `LookupPairingDeclarationDepsOverrides`, the fieldless `#[derive(Default)]` unit struct, and `build_lookup_pairing_declaration_deps(overrides: LookupPairingDeclarationDepsOverrides) -> LookupPairingDeclarationDeps`
    * `[✅]`   `LookupPairingDeclarationParamsOverrides`, `#[derive(Default)]`, with `pub concrete: Option<PairingConcrete>`; `build_lookup_pairing_declaration_params(overrides: LookupPairingDeclarationParamsOverrides) -> LookupPairingDeclarationParams`, `concrete` defaulting to `PairingConcrete::Bn254Arkworks`
    * `[✅]`   `LookupPairingDeclarationPayloadOverrides`, the fieldless `#[derive(Default)]` unit struct, and `build_lookup_pairing_declaration_payload(overrides: LookupPairingDeclarationPayloadOverrides) -> LookupPairingDeclarationPayload`
    * `[✅]`   `LookupPairingDeclarationSuccessReturnOverrides`, `#[derive(Default)]`, with `pub declaration: Option<PairingDeclaration>`; `build_lookup_pairing_declaration_success_return(overrides: LookupPairingDeclarationSuccessReturnOverrides) -> LookupPairingDeclarationSuccessReturn`, `declaration` defaulting to `build_pairing_declaration(PairingDeclarationOverrides::default())`
    * `[✅]`   `mock_lookup_pairing_declaration(_deps: &LookupPairingDeclarationDeps, _params: LookupPairingDeclarationParams, _payload: LookupPairingDeclarationPayload) -> LookupPairingDeclarationReturn`, returning `Ok` around `build_lookup_pairing_declaration_success_return` at its defaults; it is the production function type, so `let f: LookupPairingDeclarationFn = mock_lookup_pairing_declaration;` holds
    * `[✅]`   No corruptions type and no invalidator, since no type this interface owns arrives as untrusted data; no builder for the return alias, whose error arm is uninhabited

  * `[✅]`   `adapters/pairing/src/lookup_pairing_declaration/test.rs`
    * `[✅]`   Imports from `super::provides` `lookup_pairing_declaration`, `LookupPairingDeclarationParamsOverrides`, and the builders `build_lookup_pairing_declaration_deps`, `build_lookup_pairing_declaration_params`, and `build_lookup_pairing_declaration_payload`; from `crate::factory::provides` `PairingConcrete`, `PairingCurve`, `VerifierGroupArithmetic`, `PrecompileEncoding`, `TargetGroupEncodingIdentifier`, and `MockIPairingAdapterFailureMode`
    * `[✅]`   `lookup_returns_the_bn254_arkworks_declaration`
      * `[✅]`   Contract: `params.concrete` is `PairingConcrete::Bn254Arkworks` → `Ok` holding the declaration `PairingDeclaration { concrete: PairingConcrete::Bn254Arkworks, curve: PairingCurve::Bn254, verifier_group_arithmetic: VerifierGroupArithmetic::FirstGroupOnly, precompile_encoding: PrecompileEncoding::Eip196Eip197, target_group_encoding: TargetGroupEncodingIdentifier::Bn254V1, adapter_version: 1, interface_version: 1 }`
      * `[✅]`   Collaborators: the member's `DECLARATION` constant, read for real by the function; fixtures `build_lookup_pairing_declaration_deps`, `build_lookup_pairing_declaration_params`, and `build_lookup_pairing_declaration_payload`
      * `[✅]`   Arrange: `build_lookup_pairing_declaration_params` with the concrete override `PairingConcrete::Bn254Arkworks`, a member whose declaration differs from every other member's in `concrete`, so a lookup that returns another member's declaration fails
      * `[✅]`   Act: `lookup_pairing_declaration(&deps, params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `matches!` holds for `success.declaration.concrete`, `curve`, `verifier_group_arithmetic`, `precompile_encoding`, and `target_group_encoding` against the values in the contract, and `adapter_version` and `interface_version` each equal the literal `1`
    * `[✅]`   `lookup_returns_the_bn254_halo2curves_declaration`
      * `[✅]`   Contract: `params.concrete` is `PairingConcrete::Bn254Halo2curves` → `Ok` holding the declaration `PairingDeclaration { concrete: PairingConcrete::Bn254Halo2curves, curve: PairingCurve::Bn254, verifier_group_arithmetic: VerifierGroupArithmetic::FirstGroupOnly, precompile_encoding: PrecompileEncoding::Eip196Eip197, target_group_encoding: TargetGroupEncodingIdentifier::Bn254V1, adapter_version: 1, interface_version: 1 }`
      * `[✅]`   Collaborators: the member's `DECLARATION` constant, read for real by the function; the fixtures of the previous block
      * `[✅]`   Arrange: `build_lookup_pairing_declaration_params` with the concrete override `PairingConcrete::Bn254Halo2curves`, whose declaration differs from the arkworks member's in `concrete` alone, so a lookup that returns the arkworks member's declaration fails
      * `[✅]`   Act: `lookup_pairing_declaration(&deps, params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `matches!` holds for each enum field of `success.declaration` against the values in the contract, and `adapter_version` and `interface_version` each equal the literal `1`
    * `[✅]`   `lookup_returns_the_bls12_381_arkworks_declaration`
      * `[✅]`   Contract: `params.concrete` is `PairingConcrete::Bls12381Arkworks` → `Ok` holding the declaration `PairingDeclaration { concrete: PairingConcrete::Bls12381Arkworks, curve: PairingCurve::Bls12381, verifier_group_arithmetic: VerifierGroupArithmetic::BothGroups, precompile_encoding: PrecompileEncoding::Eip2537, target_group_encoding: TargetGroupEncodingIdentifier::Bls12381V1, adapter_version: 1, interface_version: 1 }`
      * `[✅]`   Collaborators: the member's `DECLARATION` constant, read for real by the function; the fixtures of the previous blocks
      * `[✅]`   Arrange: `build_lookup_pairing_declaration_params` with the concrete override `PairingConcrete::Bls12381Arkworks`, whose curve, arithmetic, precompile encoding, and target-group encoding differ from the BN254 members', so a lookup that returns a BN254 declaration fails
      * `[✅]`   Act: `lookup_pairing_declaration(&deps, params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `matches!` holds for each enum field of `success.declaration` against the values in the contract, and `adapter_version` and `interface_version` each equal the literal `1`
    * `[✅]`   `lookup_returns_the_bls12_381_halo2curves_declaration`
      * `[✅]`   Contract: `params.concrete` is `PairingConcrete::Bls12381Halo2curves` → `Ok` holding the declaration `PairingDeclaration { concrete: PairingConcrete::Bls12381Halo2curves, curve: PairingCurve::Bls12381, verifier_group_arithmetic: VerifierGroupArithmetic::BothGroups, precompile_encoding: PrecompileEncoding::Eip2537, target_group_encoding: TargetGroupEncodingIdentifier::Bls12381V1, adapter_version: 1, interface_version: 1 }`
      * `[✅]`   Collaborators: the member's `DECLARATION` constant, read for real by the function; the fixtures of the previous blocks
      * `[✅]`   Arrange: `build_lookup_pairing_declaration_params` with the concrete override `PairingConcrete::Bls12381Halo2curves`, whose declaration differs from the arkworks BLS12-381 member's in `concrete` alone, so a lookup that returns the arkworks member's declaration fails
      * `[✅]`   Act: `lookup_pairing_declaration(&deps, params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `matches!` holds for each enum field of `success.declaration` against the values in the contract, and `adapter_version` and `interface_version` each equal the literal `1`
    * `[✅]`   `lookup_returns_the_mock_declaration_carrying_the_identifier_looked_up`
      * `[✅]`   Contract: `params.concrete` is `PairingConcrete::Mock(mode)` → `Ok` holding the declaration of `MOCK_I_PAIRING_ADAPTER_DECLARATION` with `concrete` equal to `PairingConcrete::Mock(mode)`, whose provided values are `PairingCurve::Bn254`, `VerifierGroupArithmetic::FirstGroupOnly`, `PrecompileEncoding::Eip196Eip197`, `TargetGroupEncodingIdentifier::Bn254V1`, `adapter_version: 1`, and `interface_version: 1`
      * `[✅]`   Collaborators: `MOCK_I_PAIRING_ADAPTER_DECLARATION`, read for real by the function; the fixtures of the previous blocks
      * `[✅]`   Arrange: `build_lookup_pairing_declaration_params` with the concrete override `PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG1NotOnCurve)`, a mode other than the constant's `Succeeds`, so a lookup that returns the constant's own `concrete` fails
      * `[✅]`   Act: `lookup_pairing_declaration(&deps, params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `matches!` holds for `success.declaration.concrete` against `PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG1NotOnCurve)` and for each enum field against the values in the contract, and `adapter_version` and `interface_version` each equal the literal `1`

  * `[✅]`   `construction`
    * `[✅]`   The function is a standalone function with no constructor; its deps are the fieldless `LookupPairingDeclarationDeps`, built by the caller with a struct literal in production code and by `build_lookup_pairing_declaration_deps` in a mock or test
    * `[✅]`   The function constructs no concrete and no mock concrete; the value it returns is a constant of a concrete's interface or the mock declaration constant

  * `[✅]`   `adapters/pairing/src/lookup_pairing_declaration/mod.rs`
    * `[✅]`   Module declarations: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, `pub mod provides;`, and `#[cfg(test)] mod test;`
    * `[✅]`   Imports: `LookupPairingDeclarationDeps`, `LookupPairingDeclarationParams`, `LookupPairingDeclarationPayload`, `LookupPairingDeclarationReturn`, and `LookupPairingDeclarationSuccessReturn` from `interface`; `PairingConcrete` and `PairingDeclaration` from `crate::factory::provides`; `Bn254ArkworksPairing` from `crate::bn254_arkworks::provides`; `Bn254Halo2curvesPairing` from `crate::bn254_halo2curves::provides`; `Bls12381ArkworksPairing` from `crate::bls12_381_arkworks::provides`; `Bls12381Halo2curvesPairing` from `crate::bls12_381_halo2curves::provides`; and, under `#[cfg(any(test, feature = "mocks"))]`, `MOCK_I_PAIRING_ADAPTER_DECLARATION` from `crate::factory::provides`
    * `[✅]`   `pub fn lookup_pairing_declaration(_deps: &LookupPairingDeclarationDeps, params: LookupPairingDeclarationParams, _payload: LookupPairingDeclarationPayload) -> LookupPairingDeclarationReturn`, the trusted form, realizing the interaction spec's branches as one exhaustive `match` on `params.concrete` with an arm per real member returning `Ok(LookupPairingDeclarationSuccessReturn { declaration })` around that concrete's `DECLARATION`, and, under `#[cfg(any(test, feature = "mocks"))]`, an arm for `PairingConcrete::Mock(_)` returning a `PairingDeclaration` whose `concrete` is `params.concrete` and whose other fields are read from `MOCK_I_PAIRING_ADAPTER_DECLARATION`
    * `[✅]`   No `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`; no wildcard arm in the `match`; no declaration value is written as a literal

  * `[✅]`   `adapters/pairing/src/lookup_pairing_declaration/provides.rs`
    * `[✅]`   `pub use super::lookup_pairing_declaration;`, `pub use super::interface::*;`, and `#[cfg(any(test, feature = "mocks"))] pub use super::mock::*;`

  * `[✅]`   `adapters/pairing/src/lib.rs`
    * `[✅]`   The crate barrel gains `mod lookup_pairing_declaration;` and `pub use lookup_pairing_declaration::provides::*;`, so the crate's public surface carries this family function beside the `factory` module's `provides`; no concrete is re-exported

  * `[✅]`   `adapters/pairing/tests/lookup_pairing_declaration_integration_test.rs`
    * `[✅]`   Consumes the crate's public surface and the official mocks under the `mocks` feature: `lookup_pairing_declaration`, `PAIRING_CONCRETES`, `PairingConcrete`, `MockIPairingAdapterFailureMode`, and the builders `build_lookup_pairing_declaration_deps`, `build_lookup_pairing_declaration_params`, and `build_lookup_pairing_declaration_payload`; names no concrete
    * `[✅]`   `declaration_names_the_member_looked_up`
      * `[✅]`   Contract: the entry of the spec's own entries and public surface; each member of `PAIRING_CONCRETES` → `Ok` whose `declaration.concrete` equals that member
      * `[✅]`   Boundary: the public call `lookup_pairing_declaration`, the route reading the member's `DECLARATION` constant
      * `[✅]`   Mocked: nothing; the route has no outer-edge collaborator, so the test does not prove the mock member
      * `[✅]`   Arrange: the block body written once and run over every member of `PAIRING_CONCRETES`, params built by `build_lookup_pairing_declaration_params` with the member as the concrete override
      * `[✅]`   Act: `lookup_pairing_declaration(&deps, params, payload)` per member
      * `[✅]`   Assert: each result binds through the irrefutable `let Ok(success) = …;`; `success.declaration.concrete` equals the member it was looked up by
    * `[✅]`   `mock_declaration_names_the_identifier_looked_up`, under `#[cfg(feature = "mocks")]`
      * `[✅]`   Contract: the entry of the spec's own entries and public surface; `PairingConcrete::Mock(mode)` → `Ok` whose `declaration.concrete` is `PairingConcrete::Mock(mode)`
      * `[✅]`   Boundary: the public call `lookup_pairing_declaration`, the route reading `MOCK_I_PAIRING_ADAPTER_DECLARATION`
      * `[✅]`   Mocked: the mock member itself, so the test proves the identifier is carried and no real member's declaration
      * `[✅]`   Arrange: `build_lookup_pairing_declaration_params` with the concrete override `PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG1NotOnCurve)`, a mode other than `Succeeds`
      * `[✅]`   Act: `lookup_pairing_declaration(&deps, params, payload)`
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `matches!` holds for `success.declaration.concrete` against `PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG1NotOnCurve)`
    * `[✅]`   Reports its enumeration: each entry the public surface lists and the block that proves it

  * `[✅]`   `directionality`
    * `[✅]`   `lookup_pairing_declaration` depends on the `factory` module's surface through `crate::factory::provides` and on each concrete's `provides` through the family's private path, which no module outside the family names; neither the `factory` module nor any concrete depends on this module
    * `[✅]`   `pairing/admit_pairing_selection`, `pairing/admit_pairing_composition`, and `pairing/factory` consume this module through its `provides`; among repository crates the crate depends on `crates/domain` alone at runtime, and this module adds no dependency

  * `[✅]`   `requirements`
    * `[✅]`   `lookup_returns_the_bn254_arkworks_declaration`, `lookup_returns_the_bn254_halo2curves_declaration`, `lookup_returns_the_bls12_381_arkworks_declaration`, and `lookup_returns_the_bls12_381_halo2curves_declaration` pass, proving each real member's declaration is read from the member's constant and carries the member's `PairingConcrete` identifier (CR-10, the declaration the proof and chain compositions read)
    * `[✅]`   `lookup_returns_the_mock_declaration_carrying_the_identifier_looked_up` passes, proving the mock member's declaration carries the identifier looked up and only real Bn254-provided values
    * `[✅]`   `declaration_names_the_member_looked_up` and `mock_declaration_names_the_identifier_looked_up` pass, proving the public entry returns the declaration of the member looked up through the crate's public surface with no concrete constructed
    * `[✅]`   `cargo check --all-targets --all-features`, `cargo fmt --check`, and `cargo clippy --all-targets --all-features` complete without a warning in this module
    * `[✅]`   Code outside `adapters/pairing` naming a concrete's `DECLARATION`, or naming `Bn254ArkworksPairing`, `Bn254Halo2curvesPairing`, `Bls12381ArkworksPairing`, or `Bls12381Halo2curvesPairing`, fails to compile; the crate's public surface is the `factory` module's `provides` and this module's `provides`

* `[✅]`   `pairing/admit_pairing_selection` **Pairing selection admission receiving the `PairingConcrete` identifier admitted from configuration and the untrusted settings document, refusing a document that supplies any setting with the family's own error naming the member, since no pairing member has a settable value, and returning the selection that names the member; creates the family-owned `admit_pairing_selection` module of `adapters/pairing`**

  * `[✅]`   `objective`
    * `[✅]`   Problem: configuration names a pairing member and may carry a settings document beside it, and the family turns the two into the selection the factory constructs from; no pairing member has a settable value, so a document that supplies a setting is configuration for a value that does not exist and is refused before any construction, naming the member, and a member admitted with no setting is returned as the selection that names it (the dependency map's `pairing/admit_pairing_selection` row; the technical requirements' pairing family register row; the product requirements' family position on selection admission)
    * `[✅]`   Functional: given the identifier in params and the untrusted settings document as the payload, the function returns `Ok` holding the selection naming the member when the document supplies no setting, a document supplying no setting being `Value::Null` or a `Value::Object` with no entry
    * `[✅]`   Functional: given a document that supplies a setting, being a `Value::Object` with at least one entry or a `Value::Bool`, `Value::Number`, `Value::String`, or `Value::Array`, the function returns `Err(AdmitPairingSelectionErrorReturn::SettingsSupplied { identifier })` naming the member the params carry, for every real member and for the mock member
    * `[✅]`   Functional: the function holds no adapter, constructs no member, and reads no declaration; its deps are fieldless, its params carry the identifier alone, and its payload is the document in the validating form
    * `[✅]`   Functional: the selection the success return holds is the family's selection, `PairingConcrete`, the real selection carrying no settings and the mock selection carrying the failure mode its identifier names
    * `[✅]`   Non-functional: a further concrete adds its name to the real-member arm of this function's exhaustive `match`, its spec branches, and its unit blocks; no module outside the pairing family changes (the family measure); no member's name, spelling, or setting is written as a literal in `mod.rs`

  * `[✅]`   `role`
    * `[✅]`   Adapter-ring family-owned function of the pairing family, beside its factory: the selection admission the configuration admission and the factory's selection are produced through
    * `[✅]`   Does not construct a concrete, read a declaration, or compare a composition; the family's composition admission and factory do
    * `[✅]`   Does not carry a settings module, settings guard, or settings table, since no pairing member has a settable value, and does not read configuration beyond the document it is handed
    * `[✅]`   Does not carry a commit; the milestone's commit belongs to the family's factory node
    * `[✅]`   Is a public entry, since the crate barrel re-exports this module's `provides`; it hosts the public integration test of its own boundary

  * `[✅]`   `module`
    * `[✅]`   Bounded context: the `admit_pairing_selection` module of the `adapters/pairing` crate, holding the function, its deps, params, success return, error return, return alias, and function type, its builders and function mock, its interaction spec, and its unit test
    * `[✅]`   The node's files: `adapters/pairing/Cargo.toml`, `adapters/pairing/src/admit_pairing_selection/interface.rs`, `interaction.spec.md`, `mock.rs`, `test.rs`, `mod.rs`, and `provides.rs`, the barrel `adapters/pairing/src/lib.rs`, and the public integration test `adapters/pairing/tests/admit_pairing_selection_integration_test.rs`
    * `[✅]`   Outside: the member and value identifiers, the declaration type, the declared sets, the declared-selection builder, and the family's mock, which `pairing/bn254_arkworks` authors in the `factory` module; the declaration lookup; the composition admission; the factory

  * `[✅]`   `deps`
    * `[✅]`   `pairing/bn254_arkworks`, provider of the `factory` module's surface, through `crate::factory::provides`: `PairingConcrete` with the variant `Bn254Arkworks`, derived `Clone`, `Copy`, `Debug`, `PartialEq`, and `Eq`, and, under `#[cfg(any(test, feature = "mocks"))]`, the variant `Mock(MockIPairingAdapterFailureMode)`; `PAIRING_CONCRETES`; and, under the `mocks` feature and in the crate's own test build, `MockIPairingAdapterFailureMode`, derived `Clone`, `Copy`, `Debug`, `PartialEq`, and `Eq`, and `PairingConcreteOverrides` with `build_pairing_concrete`, the declared-selection builder; direction inward, a family function on the family's factory module
    * `[✅]`   `pairing/bn254_halo2curves`, through `crate::factory::provides`: the variant `PairingConcrete::Bn254Halo2curves`, its increment to the member identifier
    * `[✅]`   `pairing/bls12_381_arkworks`, through `crate::factory::provides`: the variant `PairingConcrete::Bls12381Arkworks`, its increment to the member identifier
    * `[✅]`   `pairing/bls12_381_halo2curves`, through `crate::factory::provides`: the variant `PairingConcrete::Bls12381Halo2curves`, its increment to the member identifier
    * `[✅]`   `pairing/lookup_pairing_declaration`, the dependency-map predecessor in the family's order; this node imports no symbol from it and calls it nowhere
    * `[✅]`   `serde_json` `1.0.154`, external crate, MIT OR Apache-2.0, default features, runtime dependency; supplies `serde_json::Value`, the untrusted settings document's type, named in this module's `interface.rs`, `mod.rs`, and `mock.rs`, and `serde_json::Map`, which `test.rs` builds an object document from; `mod.rs` reads an object document's entries through `Value::Object`; this node adds it to `adapters/pairing/Cargo.toml`
    * `[✅]`   Reverse dependencies: `pairing/admit_pairing_composition`, through this module's `provides`
    * `[✅]`   No reverse dependency from a concrete or from the `factory` module on this module

  * `[✅]`   `context_slice`
    * `[✅]`   From the `factory` module: the identifier `PairingConcrete`, matched exhaustively and carried unchanged into the success return and the error return; no field, constructor, or declaration of a member is read
    * `[✅]`   From the `factory` module's mock, under the `mocks` feature: `build_pairing_concrete` with `PairingConcreteOverrides`, the builder of the selection the success-return builder composes, and `MockIPairingAdapterFailureMode`, named by the mock branch
    * `[✅]`   From `serde_json`: the enum `Value` with the variants `Null`, `Bool`, `Number`, `String`, `Array`, and `Object`, and `Map`'s `is_empty` and `new`

  * `[✅]`   `adapters/pairing/Cargo.toml`
    * `[✅]`   `[dependencies]` gains `serde_json = "1.0.154"`

  * `[✅]`   `adapters/pairing/src/admit_pairing_selection/interface.rs`
    * `[✅]`   Imports `serde_json::Value` and `PairingConcrete` from `crate::factory::provides`; declares nothing beyond the items below
    * `[✅]`   `AdmitPairingSelectionDeps`, the fieldless unit struct `pub struct AdmitPairingSelectionDeps;`, the function's deps slot
    * `[✅]`   `AdmitPairingSelectionParams`, a struct with the field `pub identifier: PairingConcrete`, the member admitted from configuration; the identifier is a selection, since no received type or adapter determines which member configuration names
    * `[✅]`   `AdmitPairingSelectionSuccessReturn`, a struct with `#[derive(Debug, PartialEq, Eq)]` and the field `pub selection: PairingConcrete`, the selection naming the member
    * `[✅]`   `AdmitPairingSelectionErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]` and the single struct variant `SettingsSupplied { identifier: PairingConcrete }`, the family's own error: the document supplied a setting and the member the identifier names has none; the identifier is carried as `PairingConcrete`, never as text
    * `[✅]`   `AdmitPairingSelectionReturn`, the alias `Result<AdmitPairingSelectionSuccessReturn, AdmitPairingSelectionErrorReturn>`
    * `[✅]`   `AdmitPairingSelectionFn`, the alias `fn(&AdmitPairingSelectionDeps, AdmitPairingSelectionParams, Value) -> AdmitPairingSelectionReturn`, the validating form, the payload slot the untrusted type; the document has no payload struct, since it carries no data the function operates on beyond whether it supplies a setting
    * `[✅]`   The family's identifier types are owned by the `factory` module and are not redeclared here

  * `[✅]`   `adapters/pairing/src/admit_pairing_selection/interaction.spec.md`
    * `[✅]`   The title `` # `admit_pairing_selection` — interaction spec `` and one opening sentence stating the file as the branch contract for the `admit_pairing_selection` module of the `pairing` crate, the family's selection admission, each branch stating condition, decision, dependency call, and the exact return outcome; then one `##` section headed by the full signature `admit_pairing_selection(deps: &AdmitPairingSelectionDeps, params: AdmitPairingSelectionParams, payload: serde_json::Value) -> AdmitPairingSelectionReturn`, holding a table with the columns `Branch`, `Condition`, `Decision`, `Dependency call`, and `Outcome`, in the branch order below, then `Ordering and edges`
    * `[✅]`   `real member, no setting`: condition `params.identifier` is any real member of `PairingConcrete` and the payload is `Value::Null` or a `Value::Object` with no entry; decision the document supplies no setting, then the exhaustive `match` on `params.identifier`; dependency call none; outcome `Ok(AdmitPairingSelectionSuccessReturn { selection })` where `selection` is `params.identifier`
    * `[✅]`   `real member, setting supplied`: condition `params.identifier` is any real member and the payload is a `Value::Object` that holds an entry, or a `Value::Bool`, `Value::Number`, `Value::String`, or `Value::Array`; decision the document supplies a setting; dependency call none; outcome `Err(AdmitPairingSelectionErrorReturn::SettingsSupplied { identifier })` where `identifier` is `params.identifier`
    * `[✅]`   `mock, no setting`, present only under `#[cfg(any(test, feature = "mocks"))]`: condition `params.identifier` is `PairingConcrete::Mock(mode)` and the payload supplies no setting; decision the same `match`; dependency call none; outcome `Ok(AdmitPairingSelectionSuccessReturn { selection })` where `selection` is `PairingConcrete::Mock(mode)`
    * `[✅]`   `mock, setting supplied`, present only under `#[cfg(any(test, feature = "mocks"))]`: condition `params.identifier` is `PairingConcrete::Mock(mode)` and the payload supplies a setting; decision the document supplies a setting; dependency call none; outcome `Err(AdmitPairingSelectionErrorReturn::SettingsSupplied { identifier })` where `identifier` is `PairingConcrete::Mock(mode)`
    * `[✅]`   `Ordering and edges`, a bulleted section: whether the document supplies a setting is decided once, from the payload alone, before the `match`, with no dependency call; the `match` on `params.identifier` is exhaustive with an arm for the real members and, under `#[cfg(any(test, feature = "mocks"))]`, an arm for the mock member, so a variant without an arm does not compile; `deps` carries no value and is not read; no concrete is constructed and no declaration is read; the error arm has one flavor, naming the member; a further concrete adds its name to the real-member arm in its own increment
    * `[✅]`   `Own entries`, a section of the spec: `admission_returns_each_declared_member_for_a_document_without_a_setting` — condition `params.identifier` is any member of `PAIRING_CONCRETES` and the payload supplies no setting; outcome `Ok` whose `selection` is that member; input variation every member of the declared set, so an admission that returns one fixed member fails; edge that must survive composition: the selection is obtained through the crate's public surface with no concrete constructed.
    * `[✅]`   `admission_refuses_a_setting_for_each_declared_member` — condition `params.identifier` is any member of `PAIRING_CONCRETES` and the payload supplies a setting; outcome `Err` naming that member; input variation every member and a document that supplies a setting; edge: the refusal names the member the caller admitted.
    * `[✅]`   `mock_admission_names_the_identifier_admitted`, present only under `#[cfg(feature = "mocks")]` — condition `params.identifier` is `PairingConcrete::Mock(mode)` with a mode other than `Succeeds`; outcome `Ok` whose `selection` is `PairingConcrete::Mock(mode)` for a document without a setting and `Err` naming `PairingConcrete::Mock(mode)` for a document with one; input variation the mode; edge: the mock member is absent from `PAIRING_CONCRETES` and reached only by naming it.
    * `[✅]`   `admission_returns_each_declared_member_for_a_null_document` — condition `params.identifier` is any member of `PAIRING_CONCRETES` and the payload is `Value::Null`; outcome `Ok` whose `selection` is that member; input variation every member and the absent document, so an admission that treats only an empty object as no setting fails; edge: the absent document is admitted through the public surface.
    * `[✅]`   `admission_refuses_each_non_object_document_for_each_declared_member` — condition `params.identifier` is any member of `PAIRING_CONCRETES` and the payload is `Value::Bool`, `Value::Number`, `Value::String`, or `Value::Array`; outcome `Err` naming that member; input variation every member and each of the four scalar and array documents, each falsy or empty, so an admission that treats a falsy or empty value as absent fails; edge: each refusal names the member the caller admitted
    * `[✅]`   `Callee dispositions`, a section of the spec: no function of this node calls a dependency, and the entries of the declaration lookup and of the concretes this node's dependencies authored neither pass through a function of this node nor end at a test of this node, so no entry is absorbed or carried
    * `[✅]`   `Route`, a section of the spec: `admit_pairing_selection` reads the payload and the identifier alone; no real function of another module and no outer-edge collaborator is on the route
    * `[✅]`   `Public surface`, a section of the spec: an outside caller invokes `admit_pairing_selection` with fieldless deps, params carrying a `PairingConcrete`, and a `serde_json::Value` document, deps and params built through the official mocks and the document written through `Value`'s own variants, and observes the `Ok` selection or the `Err` refusal; the entries `admission_returns_each_declared_member_for_a_document_without_a_setting`, `admission_returns_each_declared_member_for_a_null_document`, `admission_refuses_a_setting_for_each_declared_member`, `admission_refuses_each_non_object_document_for_each_declared_member`, and `mock_admission_names_the_identifier_admitted` are proven by the public integration test

  * `[✅]`   `adapters/pairing/src/admit_pairing_selection/mock.rs`
    * `[✅]`   Module-level `#![allow(dead_code, clippy::too_many_arguments)]`; imports from `super::interface` every type the items below name; `PairingConcrete`, `PairingConcreteOverrides`, and `build_pairing_concrete` from `crate::factory::provides`; and `serde_json::Value`
    * `[✅]`   `AdmitPairingSelectionDepsOverrides`, the fieldless `#[derive(Default)]` unit struct, and `build_admit_pairing_selection_deps(overrides: AdmitPairingSelectionDepsOverrides) -> AdmitPairingSelectionDeps`
    * `[✅]`   `AdmitPairingSelectionParamsOverrides`, `#[derive(Default)]`, with `pub identifier: Option<PairingConcrete>`; `build_admit_pairing_selection_params(overrides: AdmitPairingSelectionParamsOverrides) -> AdmitPairingSelectionParams`, `identifier` defaulting to `PairingConcrete::Bn254Arkworks`
    * `[✅]`   `AdmitPairingSelectionSuccessReturnOverrides`, `#[derive(Default)]`, with `pub selection: Option<PairingConcrete>`; `build_admit_pairing_selection_success_return(overrides: AdmitPairingSelectionSuccessReturnOverrides) -> AdmitPairingSelectionSuccessReturn`, `selection` defaulting to `build_pairing_concrete(PairingConcrete::Bn254Arkworks, PairingConcreteOverrides::default())`
    * `[✅]`   `mock_admit_pairing_selection(_deps: &AdmitPairingSelectionDeps, _params: AdmitPairingSelectionParams, _payload: Value) -> AdmitPairingSelectionReturn`, returning `Ok` around `build_admit_pairing_selection_success_return` at its defaults; it is the production function type, so `let f: AdmitPairingSelectionFn = mock_admit_pairing_selection;` holds
    * `[✅]`   No builder for the settings document, an imported type written through its own variants; no builder for the error enum, used by its production value; no corruptions type and no invalidator, since the untrusted document is `serde_json::Value`, an imported type, and no struct this interface owns arrives as untrusted data

  * `[✅]`   `adapters/pairing/src/admit_pairing_selection/test.rs`
    * `[✅]`   Imports from `super::provides` `admit_pairing_selection`, `AdmitPairingSelectionErrorReturn`, `AdmitPairingSelectionParamsOverrides`, `AdmitPairingSelectionSuccessReturnOverrides`, and the builders `build_admit_pairing_selection_deps`, `build_admit_pairing_selection_params`, and `build_admit_pairing_selection_success_return`; from `crate::factory::provides` `PairingConcrete` and `MockIPairingAdapterFailureMode`; and `serde_json::{Map, Value}`
    * `[✅]`   `admit_returns_the_bn254_arkworks_selection_for_an_empty_document`
      * `[✅]`   Contract: `params.identifier` is `PairingConcrete::Bn254Arkworks` and the payload is an empty `Value::Object` → `Ok` whose `selection` is `PairingConcrete::Bn254Arkworks`
      * `[✅]`   Collaborators: none; fixtures `build_admit_pairing_selection_deps`, `build_admit_pairing_selection_params`, and `build_admit_pairing_selection_success_return`
      * `[✅]`   Arrange: `build_admit_pairing_selection_params` with the identifier override `PairingConcrete::Bn254Arkworks`, and `Value::Object(Map::new())`, a member whose selection differs from every other member's, so an admission that returns another member fails
      * `[✅]`   Act: `admit_pairing_selection(&deps, params, document)`
      * `[✅]`   Assert: the result equals `Ok(build_admit_pairing_selection_success_return(AdmitPairingSelectionSuccessReturnOverrides { selection: Some(PairingConcrete::Bn254Arkworks) }))`
    * `[✅]`   `admit_returns_the_bn254_halo2curves_selection_for_an_empty_document`
      * `[✅]`   Contract: `params.identifier` is `PairingConcrete::Bn254Halo2curves` and the payload is an empty `Value::Object` → `Ok` whose `selection` is `PairingConcrete::Bn254Halo2curves`
      * `[✅]`   Collaborators: none; the fixtures of the previous block
      * `[✅]`   Arrange: the identifier override `PairingConcrete::Bn254Halo2curves` and `Value::Object(Map::new())`, a member that differs from the arkworks member of the same curve, so an admission that returns the arkworks member fails
      * `[✅]`   Act: `admit_pairing_selection(&deps, params, document)`
      * `[✅]`   Assert: the result equals `Ok(build_admit_pairing_selection_success_return(AdmitPairingSelectionSuccessReturnOverrides { selection: Some(PairingConcrete::Bn254Halo2curves) }))`
    * `[✅]`   `admit_returns_the_bls12_381_arkworks_selection_for_an_empty_document`
      * `[✅]`   Contract: `params.identifier` is `PairingConcrete::Bls12381Arkworks` and the payload is an empty `Value::Object` → `Ok` whose `selection` is `PairingConcrete::Bls12381Arkworks`
      * `[✅]`   Collaborators: none; the fixtures of the previous blocks
      * `[✅]`   Arrange: the identifier override `PairingConcrete::Bls12381Arkworks` and `Value::Object(Map::new())`, a member of the other curve, so an admission that returns a BN254 member fails
      * `[✅]`   Act: `admit_pairing_selection(&deps, params, document)`
      * `[✅]`   Assert: the result equals `Ok(build_admit_pairing_selection_success_return(AdmitPairingSelectionSuccessReturnOverrides { selection: Some(PairingConcrete::Bls12381Arkworks) }))`
    * `[✅]`   `admit_returns_the_bls12_381_halo2curves_selection_for_an_empty_document`
      * `[✅]`   Contract: `params.identifier` is `PairingConcrete::Bls12381Halo2curves` and the payload is an empty `Value::Object` → `Ok` whose `selection` is `PairingConcrete::Bls12381Halo2curves`
      * `[✅]`   Collaborators: none; the fixtures of the previous blocks
      * `[✅]`   Arrange: the identifier override `PairingConcrete::Bls12381Halo2curves` and `Value::Object(Map::new())`, a member that differs from the arkworks member of the same curve, so an admission that returns the arkworks member fails
      * `[✅]`   Act: `admit_pairing_selection(&deps, params, document)`
      * `[✅]`   Assert: the result equals `Ok(build_admit_pairing_selection_success_return(AdmitPairingSelectionSuccessReturnOverrides { selection: Some(PairingConcrete::Bls12381Halo2curves) }))`
    * `[✅]`   `admit_returns_the_selection_for_a_null_document`
      * `[✅]`   Contract: `params.identifier` is `PairingConcrete::Bls12381Arkworks` and the payload is `Value::Null` → `Ok` whose `selection` is `PairingConcrete::Bls12381Arkworks`
      * `[✅]`   Collaborators: none; the fixtures of the previous blocks
      * `[✅]`   Arrange: the identifier override `PairingConcrete::Bls12381Arkworks` and `Value::Null`, the absent document, a form distinct from the empty object, so an admission that treats only an empty object as no setting fails
      * `[✅]`   Act: `admit_pairing_selection(&deps, params, document)`
      * `[✅]`   Assert: the result equals `Ok(build_admit_pairing_selection_success_return(AdmitPairingSelectionSuccessReturnOverrides { selection: Some(PairingConcrete::Bls12381Arkworks) }))`
    * `[✅]`   `admit_refuses_an_object_that_holds_an_entry`
      * `[✅]`   Contract: `params.identifier` is `PairingConcrete::Bn254Arkworks` and the payload is a `Value::Object` with one entry → `Err(AdmitPairingSelectionErrorReturn::SettingsSupplied { identifier: PairingConcrete::Bn254Arkworks })`
      * `[✅]`   Collaborators: none; the fixtures of the previous blocks
      * `[✅]`   Arrange: the identifier override `PairingConcrete::Bn254Arkworks` and a `Value::Object` built from a `Map` holding the key `endpoint` with the value `Value::Bool(true)`, a document that differs from the empty object by one entry, so an admission that ignores an object's entries fails
      * `[✅]`   Act: `admit_pairing_selection(&deps, params, document)`
      * `[✅]`   Assert: the result equals `Err(AdmitPairingSelectionErrorReturn::SettingsSupplied { identifier: PairingConcrete::Bn254Arkworks })`
    * `[✅]`   `admit_refuses_a_boolean_document`
      * `[✅]`   Contract: `params.identifier` is `PairingConcrete::Bn254Halo2curves` and the payload is `Value::Bool(false)` → `Err(AdmitPairingSelectionErrorReturn::SettingsSupplied { identifier: PairingConcrete::Bn254Halo2curves })`
      * `[✅]`   Collaborators: none; the fixtures of the previous blocks
      * `[✅]`   Arrange: the identifier override `PairingConcrete::Bn254Halo2curves` and `Value::Bool(false)`, a falsy scalar document, so an admission that treats a falsy value as absent fails
      * `[✅]`   Act: `admit_pairing_selection(&deps, params, document)`
      * `[✅]`   Assert: the result equals `Err(AdmitPairingSelectionErrorReturn::SettingsSupplied { identifier: PairingConcrete::Bn254Halo2curves })`
    * `[✅]`   `admit_refuses_a_number_document`
      * `[✅]`   Contract: `params.identifier` is `PairingConcrete::Bls12381Arkworks` and the payload is the number zero → `Err(AdmitPairingSelectionErrorReturn::SettingsSupplied { identifier: PairingConcrete::Bls12381Arkworks })`
      * `[✅]`   Collaborators: none; the fixtures of the previous blocks
      * `[✅]`   Arrange: the identifier override `PairingConcrete::Bls12381Arkworks` and `Value::Number` holding zero, a falsy number, so an admission that treats zero as absent fails
      * `[✅]`   Act: `admit_pairing_selection(&deps, params, document)`
      * `[✅]`   Assert: the result equals `Err(AdmitPairingSelectionErrorReturn::SettingsSupplied { identifier: PairingConcrete::Bls12381Arkworks })`
    * `[✅]`   `admit_refuses_a_string_document`
      * `[✅]`   Contract: `params.identifier` is `PairingConcrete::Bls12381Halo2curves` and the payload is the empty `Value::String` → `Err(AdmitPairingSelectionErrorReturn::SettingsSupplied { identifier: PairingConcrete::Bls12381Halo2curves })`
      * `[✅]`   Collaborators: none; the fixtures of the previous blocks
      * `[✅]`   Arrange: the identifier override `PairingConcrete::Bls12381Halo2curves` and `Value::String` holding the empty text, a falsy string, so an admission that treats an empty string as absent fails
      * `[✅]`   Act: `admit_pairing_selection(&deps, params, document)`
      * `[✅]`   Assert: the result equals `Err(AdmitPairingSelectionErrorReturn::SettingsSupplied { identifier: PairingConcrete::Bls12381Halo2curves })`
    * `[✅]`   `admit_refuses_an_array_document`
      * `[✅]`   Contract: `params.identifier` is `PairingConcrete::Bn254Arkworks` and the payload is the empty `Value::Array` → `Err(AdmitPairingSelectionErrorReturn::SettingsSupplied { identifier: PairingConcrete::Bn254Arkworks })`
      * `[✅]`   Collaborators: none; the fixtures of the previous blocks
      * `[✅]`   Arrange: the identifier override `PairingConcrete::Bn254Arkworks` and `Value::Array` holding no element, an empty array that differs from the empty object, so an admission that treats every empty container as absent fails
      * `[✅]`   Act: `admit_pairing_selection(&deps, params, document)`
      * `[✅]`   Assert: the result equals `Err(AdmitPairingSelectionErrorReturn::SettingsSupplied { identifier: PairingConcrete::Bn254Arkworks })`
    * `[✅]`   `admit_returns_the_mock_selection_carrying_its_failure_mode`
      * `[✅]`   Contract: `params.identifier` is `PairingConcrete::Mock(mode)` and the payload supplies no setting → `Ok` whose `selection` is `PairingConcrete::Mock(mode)`
      * `[✅]`   Collaborators: none; the fixtures of the previous blocks
      * `[✅]`   Arrange: the identifier override `PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG1NotOnCurve)`, a mode other than `Succeeds`, and `Value::Object(Map::new())`, so an admission that returns a mode other than the one named fails
      * `[✅]`   Act: `admit_pairing_selection(&deps, params, document)`
      * `[✅]`   Assert: the result equals `Ok(build_admit_pairing_selection_success_return(AdmitPairingSelectionSuccessReturnOverrides { selection: Some(PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG1NotOnCurve)) }))`
    * `[✅]`   `admit_refuses_a_setting_for_the_mock_member_naming_it`
      * `[✅]`   Contract: `params.identifier` is `PairingConcrete::Mock(mode)` and the payload supplies a setting → `Err(AdmitPairingSelectionErrorReturn::SettingsSupplied { identifier: PairingConcrete::Mock(mode) })`
      * `[✅]`   Collaborators: none; the fixtures of the previous blocks
      * `[✅]`   Arrange: the identifier override `PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG2NotOnCurve)` and a `Value::Object` holding one entry, so a refusal that names a real member or another mode fails
      * `[✅]`   Act: `admit_pairing_selection(&deps, params, document)`
      * `[✅]`   Assert: the result equals `Err(AdmitPairingSelectionErrorReturn::SettingsSupplied { identifier: PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG2NotOnCurve) })`

  * `[✅]`   `construction`
    * `[✅]`   The function is a standalone function with no constructor; its deps are the fieldless `AdmitPairingSelectionDeps`, built by the caller with a struct literal in production code and by `build_admit_pairing_selection_deps` in a mock or test
    * `[✅]`   The function constructs no concrete and no mock concrete; the selection it returns is the identifier it received

  * `[✅]`   `adapters/pairing/src/admit_pairing_selection/mod.rs`
    * `[✅]`   Module declarations: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, `pub mod provides;`, and `#[cfg(test)] mod test;`
    * `[✅]`   Imports: `AdmitPairingSelectionDeps`, `AdmitPairingSelectionErrorReturn`, `AdmitPairingSelectionParams`, `AdmitPairingSelectionReturn`, and `AdmitPairingSelectionSuccessReturn` from `interface`; `PairingConcrete` from `crate::factory::provides`; and `serde_json::Value`
    * `[✅]`   `pub fn admit_pairing_selection(_deps: &AdmitPairingSelectionDeps, params: AdmitPairingSelectionParams, payload: Value) -> AdmitPairingSelectionReturn`, the validating form, realizing the interaction spec's branches: a binding `supplies_setting` decided from `&payload` by a `match` that is `false` for `Value::Null`, `false` for a `Value::Object` whose `is_empty()` holds, and `true` for every other arm, with no wildcard arm that hides a variant; then one exhaustive `match` on `params.identifier` whose real-member arm, naming each real member of `PairingConcrete`, and, under `#[cfg(any(test, feature = "mocks"))]`, whose `PairingConcrete::Mock(_)` arm each return `Err(AdmitPairingSelectionErrorReturn::SettingsSupplied { identifier: params.identifier })` where `supplies_setting` holds and `Ok(AdmitPairingSelectionSuccessReturn { selection: params.identifier })` otherwise
    * `[✅]`   No `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`; no wildcard arm in the `match` on `params.identifier`; no member's name or spelling is written as a literal

  * `[✅]`   `adapters/pairing/src/admit_pairing_selection/provides.rs`
    * `[✅]`   `pub use super::admit_pairing_selection;`, `pub use super::interface::*;`, and `#[cfg(any(test, feature = "mocks"))] pub use super::mock::*;`

  * `[✅]`   `adapters/pairing/src/lib.rs`
    * `[✅]`   The crate barrel gains `mod admit_pairing_selection;` and `pub use admit_pairing_selection::provides::*;`, so the crate's public surface carries this family function beside the `factory` module's `provides`; no concrete is re-exported

  * `[✅]`   `adapters/pairing/tests/admit_pairing_selection_integration_test.rs`
    * `[✅]`   Consumes the crate's public surface and the official mocks under the `mocks` feature: `admit_pairing_selection`, `AdmitPairingSelectionErrorReturn`, `AdmitPairingSelectionSuccessReturn`, `PAIRING_CONCRETES`, `PairingConcrete`, `MockIPairingAdapterFailureMode`, and the builders `build_admit_pairing_selection_deps`, `build_admit_pairing_selection_params`, and `build_admit_pairing_selection_success_return` with their overrides; names no concrete; the document is written through `serde_json::Value`'s own variants
    * `[✅]`   `admission_returns_each_declared_member_for_a_document_without_a_setting`
      * `[✅]`   Contract: the entry of the spec's own entries and public surface; each member of `PAIRING_CONCRETES` with `Value::Object` holding no entry → `Ok` whose `selection` is that member
      * `[✅]`   Boundary: the public call `admit_pairing_selection`, the route reading the identifier and the payload
      * `[✅]`   Mocked: nothing; the route has no outer-edge collaborator, so the test does not prove the mock member
      * `[✅]`   Arrange: the block body written once and run over every member of `PAIRING_CONCRETES`, params built by `build_admit_pairing_selection_params` with the member as the identifier override
      * `[✅]`   Act: `admit_pairing_selection(&deps, params, document)` per member
      * `[✅]`   Assert: each result equals `Ok(build_admit_pairing_selection_success_return(...))` built with the member as the selection override
    * `[✅]`   `admission_refuses_a_setting_for_each_declared_member`
      * `[✅]`   Contract: the entry of the spec's own entries and public surface; each member of `PAIRING_CONCRETES` with a `Value::Object` holding one entry → `Err(AdmitPairingSelectionErrorReturn::SettingsSupplied { identifier })` naming that member
      * `[✅]`   Boundary: the public call `admit_pairing_selection`, the route reading the identifier and the payload
      * `[✅]`   Mocked: nothing; the route has no outer-edge collaborator
      * `[✅]`   Arrange: the block body written once and run over every member of `PAIRING_CONCRETES`, with a `Value::Object` holding the key `endpoint` and the value `Value::Bool(true)`
      * `[✅]`   Act: `admit_pairing_selection(&deps, params, document)` per member
      * `[✅]`   Assert: each result equals `Err(AdmitPairingSelectionErrorReturn::SettingsSupplied { identifier })` with the member it was called over
    * `[✅]`   `mock_admission_names_the_identifier_admitted`, under `#[cfg(feature = "mocks")]`
      * `[✅]`   Contract: the entry of the spec's own entries and public surface; `PairingConcrete::Mock(mode)` with no setting → `Ok` whose `selection` is `PairingConcrete::Mock(mode)`, and with a setting → `Err` naming `PairingConcrete::Mock(mode)`
      * `[✅]`   Boundary: the public call `admit_pairing_selection`, the route reading the identifier and the payload
      * `[✅]`   Mocked: the mock member itself, so the test proves the identifier is carried and no real member's admission
      * `[✅]`   Arrange: the identifier override `PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG1NotOnCurve)`, a mode other than `Succeeds`, called once with an empty `Value::Object` and once with a `Value::Object` holding one entry
      * `[✅]`   Act: `admit_pairing_selection(&deps, params, document)` per document
      * `[✅]`   Assert: the first result equals `Ok(build_admit_pairing_selection_success_return(...))` with the mock identifier as the selection override, and the second equals `Err(AdmitPairingSelectionErrorReturn::SettingsSupplied { identifier: PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG1NotOnCurve) })`
    * `[✅]`   `admission_returns_each_declared_member_for_a_null_document`
      * `[✅]`   Contract: the entry of the spec's own entries and public surface; each member of `PAIRING_CONCRETES` with `Value::Null` → `Ok` whose `selection` is that member
      * `[✅]`   Boundary: the public call `admit_pairing_selection`, the route reading the identifier and the payload
      * `[✅]`   Mocked: nothing; the route has no outer-edge collaborator
      * `[✅]`   Arrange: the block body written once and run over every member of `PAIRING_CONCRETES`, with `Value::Null`, the absent document, which differs from the empty object the first block uses
      * `[✅]`   Act: `admit_pairing_selection(&deps, params, document)` per member
      * `[✅]`   Assert: each result equals `Ok(build_admit_pairing_selection_success_return(...))` built with the member as the selection override
    * `[✅]`   `admission_refuses_each_non_object_document_for_each_declared_member`
      * `[✅]`   Contract: the entry of the spec's own entries and public surface; each member of `PAIRING_CONCRETES` with `Value::Bool(false)`, a `Value::Number` holding zero, a `Value::String` holding the empty text, and an empty `Value::Array` → `Err(AdmitPairingSelectionErrorReturn::SettingsSupplied { identifier })` naming that member
      * `[✅]`   Boundary: the public call `admit_pairing_selection`, the route reading the identifier and the payload
      * `[✅]`   Mocked: nothing; the route has no outer-edge collaborator
      * `[✅]`   Arrange: the block body written once and run over every member of `PAIRING_CONCRETES` and, within each, over the four documents named in the contract, each falsy or empty, so an admission that treats a falsy or empty document as no setting fails
      * `[✅]`   Act: `admit_pairing_selection(&deps, params, document)` per member and document
      * `[✅]`   Assert: each result equals `Err(AdmitPairingSelectionErrorReturn::SettingsSupplied { identifier })` with the member it was called over
    * `[✅]`   Reports its enumeration: each entry the public surface lists and the block that proves it

  * `[✅]`   `directionality`
    * `[✅]`   `admit_pairing_selection` depends on the `factory` module's surface through `crate::factory::provides` and on `serde_json`; it depends on no concrete; neither the `factory` module nor any concrete depends on this module
    * `[✅]`   `pairing/admit_pairing_composition` consumes this module through its `provides`; among repository crates the crate depends on `crates/domain` alone at runtime, and this module adds no repository dependency

  * `[✅]`   `requirements`
    * `[✅]`   `admit_returns_the_bn254_arkworks_selection_for_an_empty_document`, `admit_returns_the_bn254_halo2curves_selection_for_an_empty_document`, `admit_returns_the_bls12_381_arkworks_selection_for_an_empty_document`, and `admit_returns_the_bls12_381_halo2curves_selection_for_an_empty_document` pass, proving each real member is admitted as the selection naming it (CR-10, the selection the factory constructs from)
    * `[✅]`   `admit_returns_the_selection_for_a_null_document` passes, proving an absent document supplies no setting
    * `[✅]`   `admit_refuses_an_object_that_holds_an_entry`, `admit_refuses_a_boolean_document`, `admit_refuses_a_number_document`, `admit_refuses_a_string_document`, and `admit_refuses_an_array_document` pass, proving every form of document that supplies a setting is refused with the family's own error naming the member
    * `[✅]`   `admit_returns_the_mock_selection_carrying_its_failure_mode` and `admit_refuses_a_setting_for_the_mock_member_naming_it` pass, proving the mock member is admitted and refused with its failure mode carried
    * `[✅]`   `admission_returns_each_declared_member_for_a_document_without_a_setting`, `admission_returns_each_declared_member_for_a_null_document`, `admission_refuses_a_setting_for_each_declared_member`, `admission_refuses_each_non_object_document_for_each_declared_member`, and `mock_admission_names_the_identifier_admitted` pass, proving the public entry admits and refuses through the crate's public surface with no concrete constructed
    * `[✅]`   `cargo check --all-targets --all-features`, `cargo fmt --check`, and `cargo clippy --all-targets --all-features` complete without a warning in this module
    * `[✅]`   Code outside `adapters/pairing` naming a concrete fails to compile; the crate's public surface is the `factory` module's `provides` and the `provides` of this module and of the other family-owned functions

* `[✅]`   `pairing/admit_pairing_composition` **Pairing composition admission comparing the selected member's declared curve, verifier-group arithmetic, precompile encoding, and target-group encoding with the values its params require, reading the declaration through the declaration lookup its deps carry, and returning the family's `Incompatible` error naming the member and each disagreeing value before any member is constructed; creates the family-owned `admit_pairing_composition` module of `adapters/pairing`**

  * `[✅]`   `objective`
    * `[✅]`   Problem: a consumer of the pairing family states the values it requires of a member, the target-group encoding identifier the suite requires, the precompile encodings the chain works with, and the curve and verifier-group arithmetic the verifier requires; whether a member provides them is determinate from its declaration alone, so the family compares the declaration with the requirements before any member is constructed and refuses a disagreement as `Incompatible` before any effort or side effect (the dependency map's `pairing/admit_pairing_composition` row; the milestones' pairing scope, where composition admission refuses a selection whose required target-group encoding identifier no composed declaration provides; the technical requirements' chain family ownership, where precompile requirements and provisions are compared by the pairing family's composition admission; the families topic's compatibility section)
    * `[✅]`   Functional: given the member identifier and the required values in params, the function reads the member's declaration through the lookup its deps carry and returns `Ok` holding the identifier when the declared curve equals the required curve, the declared verifier-group arithmetic equals the required arithmetic, the declared precompile encoding is a member of the required precompile encodings, and the declared target-group encoding equals the required target-group encoding
    * `[✅]`   Functional: when any of those comparisons fails, the function returns `Err(AdmitPairingCompositionErrorReturn::Incompatible { identifier, disagreements })` naming the member the params carry and holding one `PairingDisagreement` for each provision that disagrees, in the order curve, verifier-group arithmetic, precompile encoding, target-group encoding, each carrying the declared value and the required value as identifiers, never as text
    * `[✅]`   Functional: the declared verifier-group arithmetic is the family's verifier-form provision, since the declaration carries one value that states whether the verifier works in the first group alone or in both groups, which the dependency map's row names as the verifier-form provision beside arithmetic and which the BLS12-381 member's declaration states as the form the proof factory selects; the function compares that value as the verifier-form provision
    * `[✅]`   Functional: the function holds no adapter, constructs no member, and causes no side effect; its one collaborator is the declaration lookup, so the refusal precedes any construction the factory performs
    * `[✅]`   Functional: the real and the mock member are admitted through the same comparisons, since the declaration the lookup returns for either is read the same way
    * `[✅]`   Non-functional: a further concrete changes no file of this module, since every declared value it provides is read through the lookup; no module outside the pairing family changes (the family measure)
    * `[✅]`   Non-functional: no member's name, spelling, or declared value is written as a literal in `mod.rs`; every compared value is read from the declaration the lookup returns or from params

  * `[✅]`   `role`
    * `[✅]`   Adapter-ring family-owned function of the pairing family, beside its factory: the composition admission the family's factory calls before it constructs anything
    * `[✅]`   Does not construct a concrete, narrow a settings document, admit a selection, or read configuration; the selection admission and the factory do
    * `[✅]`   Does not read a declaration except through the lookup its deps carry, and does not restate a member's declared value
    * `[✅]`   Does not carry a commit; the milestone's commit belongs to the family's factory node
    * `[✅]`   Is a public entry, since the crate barrel re-exports this module's `provides`; it hosts the public integration test of its own boundary

  * `[✅]`   `module`
    * `[✅]`   Bounded context: the `admit_pairing_composition` module of the `adapters/pairing` crate, holding the function, its deps, params, payload, success return, error return, return alias, and function type, the disagreement type its error carries, its builders and function mock, its interaction spec, its unit test, and its private integration test
    * `[✅]`   The node's files: `adapters/pairing/src/factory/interface.rs`, for the derives named below, `adapters/pairing/src/admit_pairing_composition/interface.rs`, `interaction.spec.md`, `mock.rs`, `test.rs`, `mod.rs`, `integration_test.rs`, and `provides.rs`, the barrel `adapters/pairing/src/lib.rs`, and the public integration test `adapters/pairing/tests/admit_pairing_composition_integration_test.rs`
    * `[✅]`   Outside: the member and value identifiers, the declaration type, the declared sets, and the declaration builders, which `pairing/bn254_arkworks` authors in the `factory` module and its increments extend; the declaration lookup; the selection admission; the factory

  * `[✅]`   `deps`
    * `[✅]`   `pairing/lookup_pairing_declaration`, through `crate::lookup_pairing_declaration::provides`: `LookupPairingDeclarationFn`, the function type of the handle the deps hold, `LookupPairingDeclarationDeps`, `LookupPairingDeclarationParams` with its field `concrete`, `LookupPairingDeclarationPayload`, and `LookupPairingDeclarationSuccessReturn` with its field `declaration`, which the function builds and reads; under the `mocks` feature and in the crate's own test build, `mock_lookup_pairing_declaration`, the function mock the unit test and the mock builder hand to the deps; `lookup_pairing_declaration`, the production function the private and public integration tests hand to the deps; direction inward, a family function on the family's factory module
    * `[✅]`   `pairing/admit_pairing_selection`, the dependency-map predecessor in the family's order; this node imports no symbol from it and calls it nowhere
    * `[✅]`   `pairing/bn254_arkworks`, provider of the `factory` module's surface, through `crate::factory::provides`: `PairingConcrete` with the variant `Bn254Arkworks`, derived `Clone`, `Copy`, `Debug`, `PartialEq`, and `Eq`, and, under `#[cfg(any(test, feature = "mocks"))]`, the variant `Mock(MockIPairingAdapterFailureMode)`; `PairingCurve::Bn254`, `VerifierGroupArithmetic::FirstGroupOnly`, `PrecompileEncoding::Eip196Eip197`, `TargetGroupEncodingIdentifier::Bn254V1`, the last derived `Clone`, `Copy`, `Debug`, `PartialEq`, and `Eq`; `PairingDeclaration`, read through the lookup's success return; `PAIRING_CONCRETES` and `TARGET_GROUP_ENCODING_IDENTIFIERS`; and, under the `mocks` feature and in the crate's own test build, `MockIPairingAdapterFailureMode`, `PairingDeclarationOverrides` with `build_pairing_declaration`, and `PairingConcreteOverrides` with `build_pairing_concrete`, the declared-selection builder
    * `[✅]`   `pairing/bn254_halo2curves`, through `crate::factory::provides`: the variant `PairingConcrete::Bn254Halo2curves`, its increment to the member identifier
    * `[✅]`   `pairing/bls12_381_arkworks`, through `crate::factory::provides`: the variants `PairingConcrete::Bls12381Arkworks`, `PairingCurve::Bls12381`, `VerifierGroupArithmetic::BothGroups`, `PrecompileEncoding::Eip2537`, and `TargetGroupEncodingIdentifier::Bls12381V1`, its increments to the member and value identifiers
    * `[✅]`   `pairing/bls12_381_halo2curves`, through `crate::factory::provides`: the variant `PairingConcrete::Bls12381Halo2curves`, its increment to the member identifier
    * `[✅]`   `core::convert::Infallible`, standard library, the error arm of the lookup's return, matched by the irrefutable pattern `let Ok(looked_up) = …;`
    * `[✅]`   Reverse dependencies: `pairing/factory`, through this module's `provides`
    * `[✅]`   No reverse dependency from a concrete, from the `factory` module, or from `pairing/lookup_pairing_declaration` on this module

  * `[✅]`   `context_slice`
    * `[✅]`   From the lookup: the function type `LookupPairingDeclarationFn`, held in deps and called once, with the fieldless `LookupPairingDeclarationDeps` and `LookupPairingDeclarationPayload` built by struct literal and `LookupPairingDeclarationParams { concrete }` carrying `params.identifier`; the success return's `declaration` field, whose `curve`, `verifier_group_arithmetic`, `precompile_encoding`, and `target_group_encoding` fields are read; no other field of the declaration is read
    * `[✅]`   From the `factory` module: the identifier `PairingConcrete`, carried unchanged into the success return and the error return; the value identifiers `PairingCurve`, `VerifierGroupArithmetic`, `PrecompileEncoding`, and `TargetGroupEncodingIdentifier`, compared by `==` and carried in the disagreements
    * `[✅]`   From the `factory` module's mock, under the `mocks` feature: `build_pairing_declaration` with `PairingDeclarationOverrides`, whose defaults the params builder reads, and `build_pairing_concrete` with `PairingConcreteOverrides`, the builder of the identifier the success-return builder composes

  * `[✅]`   `adapters/pairing/src/factory/interface.rs`
    * `[✅]`   `PairingCurve`, `VerifierGroupArithmetic`, and `PrecompileEncoding` each carry `#[derive(Clone, Copy, Debug, PartialEq, Eq)]`, so the admission compares a declared value with a required one by `==`, the params hold each by value, and an `Incompatible` error that carries them is printable and comparable by value; `TargetGroupEncodingIdentifier` and `PairingConcrete` carry those derives and are used as declared

  * `[✅]`   `adapters/pairing/src/admit_pairing_composition/interface.rs`
    * `[✅]`   Imports `PairingConcrete`, `PairingCurve`, `VerifierGroupArithmetic`, `PrecompileEncoding`, and `TargetGroupEncodingIdentifier` from `crate::factory::provides` and `LookupPairingDeclarationFn` from `crate::lookup_pairing_declaration::provides`; declares nothing beyond the items below
    * `[✅]`   `AdmitPairingCompositionDeps`, a struct with the field `pub lookup_pairing_declaration: LookupPairingDeclarationFn`, the declaration lookup the function reads the member's declaration through
    * `[✅]`   `AdmitPairingCompositionParams`, a struct with the fields `pub identifier: PairingConcrete`, the member the selection names, `pub required_curve: PairingCurve`, `pub required_verifier_group_arithmetic: VerifierGroupArithmetic`, `pub required_precompile_encodings: Vec<PrecompileEncoding>`, the encodings the consumer works with, any of which the member's declared encoding may be, and `pub required_target_group_encoding: TargetGroupEncodingIdentifier`; each is a selection the consumer states, since no received type or adapter of this function fixes it
    * `[✅]`   `AdmitPairingCompositionPayload`, the fieldless unit struct `pub struct AdmitPairingCompositionPayload;`, since the pairing family composes with no other family's declaration
    * `[✅]`   `AdmitPairingCompositionSuccessReturn`, a struct with `#[derive(Debug, PartialEq, Eq)]` and the field `pub identifier: PairingConcrete`, the member the admission passed
    * `[✅]`   `PairingDisagreement`, an enum with `#[derive(Debug, PartialEq, Eq)]` and the struct variants `Curve { declared: PairingCurve, required: PairingCurve }`, `VerifierGroupArithmetic { declared: VerifierGroupArithmetic, required: VerifierGroupArithmetic }`, `PrecompileEncoding { declared: PrecompileEncoding, required: Vec<PrecompileEncoding> }`, and `TargetGroupEncoding { declared: TargetGroupEncodingIdentifier, required: TargetGroupEncodingIdentifier }`, each variant named by the provision it names and carrying identifiers, never text
    * `[✅]`   `AdmitPairingCompositionErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]` and the single struct variant `Incompatible { identifier: PairingConcrete, disagreements: Vec<PairingDisagreement> }`, the family's `Incompatible` flavor: the member the params name and every provision that disagrees, carrying no concrete
    * `[✅]`   `AdmitPairingCompositionReturn`, the alias `Result<AdmitPairingCompositionSuccessReturn, AdmitPairingCompositionErrorReturn>`
    * `[✅]`   `AdmitPairingCompositionFn`, the alias `fn(&AdmitPairingCompositionDeps, AdmitPairingCompositionParams, AdmitPairingCompositionPayload) -> AdmitPairingCompositionReturn`, the trusted form, since every value arrives already typed from an in-crate caller
    * `[✅]`   The family's identifier and declaration types are owned by the `factory` module and the lookup's types by its own interface; none is redeclared here

  * `[✅]`   `adapters/pairing/src/admit_pairing_composition/interaction.spec.md`
    * `[✅]`   The title `` # `admit_pairing_composition` — interaction spec `` and one opening sentence stating the file as the branch contract for the `admit_pairing_composition` module of the `pairing` crate, the family's composition admission comparing a member's declared provisions with the values its params require, each branch stating condition, decision, dependency call, and the exact return outcome; then one `##` section headed by the full signature `admit_pairing_composition(deps: &AdmitPairingCompositionDeps, params: AdmitPairingCompositionParams, payload: AdmitPairingCompositionPayload) -> AdmitPairingCompositionReturn`, holding a table with the columns `Branch`, `Condition`, `Decision`, `Dependency call`, and `Outcome`, in the branch order below, then `Ordering and edges`
    * `[✅]`   `every provision agrees`: condition the declaration the lookup returns for `params.identifier` has `curve` equal to `params.required_curve`, `verifier_group_arithmetic` equal to `params.required_verifier_group_arithmetic`, `precompile_encoding` a member of `params.required_precompile_encodings`, and `target_group_encoding` equal to `params.required_target_group_encoding`; decision each comparison holds; dependency call `deps.lookup_pairing_declaration` with `LookupPairingDeclarationParams { concrete: params.identifier }`; outcome `Ok(AdmitPairingCompositionSuccessReturn { identifier })` where `identifier` is `params.identifier`
    * `[✅]`   `curve disagrees`: condition the declared `curve` differs from `params.required_curve`; decision the curve comparison fails; dependency call the same lookup call; outcome `Err(AdmitPairingCompositionErrorReturn::Incompatible { identifier, disagreements })` where `identifier` is `params.identifier` and `disagreements` holds `PairingDisagreement::Curve { declared, required }` with the declared curve and `params.required_curve`, together with the disagreement of every other provision that also disagrees
    * `[✅]`   `verifier-group arithmetic disagrees`: condition the declared `verifier_group_arithmetic` differs from `params.required_verifier_group_arithmetic`; decision the arithmetic comparison fails; dependency call the same lookup call; outcome the same `Incompatible` error whose `disagreements` holds `PairingDisagreement::VerifierGroupArithmetic { declared, required }`
    * `[✅]`   `precompile encoding disagrees`: condition the declared `precompile_encoding` is not a member of `params.required_precompile_encodings`; decision the membership check fails; dependency call the same lookup call; outcome the same `Incompatible` error whose `disagreements` holds `PairingDisagreement::PrecompileEncoding { declared, required }` with the declared encoding and `params.required_precompile_encodings`
    * `[✅]`   `target-group encoding disagrees`: condition the declared `target_group_encoding` differs from `params.required_target_group_encoding`; decision the target-group comparison fails; dependency call the same lookup call; outcome the same `Incompatible` error whose `disagreements` holds `PairingDisagreement::TargetGroupEncoding { declared, required }`
    * `[✅]`   `several provisions disagree`: condition more than one comparison fails; decision each comparison is evaluated independently of the others; dependency call the same lookup call; outcome the same `Incompatible` error whose `disagreements` holds one `PairingDisagreement` for each failed comparison, in the order curve, verifier-group arithmetic, precompile encoding, target-group encoding
    * `[✅]`   `Ordering and edges`, a bulleted section: the lookup is called once, before any comparison, and its `Infallible` error arm has no branch; the comparisons are evaluated independently so every disagreeing provision is named; the function does not match on `params.identifier`, so the real members and the mock member pass through the same branches; `payload` is not read; no concrete is constructed and no side effect occurs; the error arm has one flavor, carrying the member and every disagreement; a further concrete adds no branch
    * `[✅]`   `Own entries`, a section of the spec: `admission_compares_the_provisions_of_the_member_named` — condition `params.identifier` is a member of each curve and the required values are those of the member's own curve or of the other curve; outcome `Ok` naming the member where its declared provisions are the required ones, and `Incompatible` naming the member and every disagreeing provision, with the declared values of the member named, where they are not; input variation a member of each curve against the required values of each curve, so an admission that reads one fixed member's declaration, or compares against the requirements alone, fails; edge that must survive composition: the provisions compared are those the real lookup returns for the identifier the caller named. `mock_admission_compares_the_mock_declaration`, present only under `#[cfg(any(test, feature = "mocks"))]` — condition `params.identifier` is `PairingConcrete::Mock(mode)` with a mode other than `Succeeds`; outcome `Ok` naming `PairingConcrete::Mock(mode)` against the provisions the mock declaration states, and `Incompatible` naming `PairingConcrete::Mock(mode)` against provisions it does not state; input variation the mode and the required values; edge: the mock member is absent from `PAIRING_CONCRETES` and reached only by naming it. `admission_names_each_disagreeing_provision_for_each_declared_member` — condition `params.identifier` is any member of `PAIRING_CONCRETES` and one required value is that of a member of a different curve; outcome `Incompatible` naming the member and the one provision replaced, with its declared and required values; input variation every member and every provision, so an admission that omits or confuses a provision fails; edge: the refusal is observed through the crate's public surface. `admission_names_every_disagreeing_provision_in_order_for_each_declared_member` — condition `params.identifier` is any member of `PAIRING_CONCRETES` and every required value is that of a member of a different curve; outcome `Incompatible` naming the member and every provision in the order curve, verifier-group arithmetic, precompile encoding, target-group encoding; input variation every member; edge: the order survives the public surface
    * `[✅]`   `Callee dispositions`, a section of the spec: the entries `declaration_names_the_member_looked_up` and `mock_declaration_names_the_identifier_looked_up` of `pairing/lookup_pairing_declaration` state that a declaration's `concrete` is the member looked up; this function reads no `concrete` field of the declaration and returns none, so neither entry passes through a function of this node or ends at a test of this node, and neither is dispositioned; the provisions the function does read are proven against the real lookup by the private blocks of the own entries; no entry of `pairing/admit_pairing_selection` passes through a function of this node or ends at a test of this node, so none is dispositioned
    * `[✅]`   `Route`, a section of the spec: `admit_pairing_composition` calls `lookup_pairing_declaration`, which reads the `DECLARATION` constant of the concrete the identifier names, or `MOCK_I_PAIRING_ADAPTER_DECLARATION` for the mock member; no outer-edge collaborator is on the route
    * `[✅]`   `Private surface`, a section of the spec: the chain of real functions `admit_pairing_composition` then `lookup_pairing_declaration` then the declaration constant it reads; no outer-edge collaborator is mocked; the observable result is the `Ok` identifier or the `Incompatible` error; the entries `admission_compares_the_provisions_of_the_member_named` and `mock_admission_compares_the_mock_declaration` are proven by the private integration test
    * `[✅]`   `Public surface`, a section of the spec: an outside caller invokes `admit_pairing_composition` with deps holding the public `lookup_pairing_declaration`, params carrying a `PairingConcrete` and the required values, and the fieldless payload, each built through the official mocks, and observes the `Ok` identifier or the `Incompatible` error; the entries `admission_compares_the_provisions_of_the_member_named` and `mock_admission_compares_the_mock_declaration`, `admission_names_each_disagreeing_provision_for_each_declared_member`, and `admission_names_every_disagreeing_provision_in_order_for_each_declared_member` are proven by the public integration test

  * `[✅]`   `adapters/pairing/src/admit_pairing_composition/mock.rs`
    * `[✅]`   Module-level `#![allow(dead_code, clippy::too_many_arguments)]`; imports from `super::interface` every type the items below name; `PairingConcrete`, `PairingCurve`, `VerifierGroupArithmetic`, `PrecompileEncoding`, `TargetGroupEncodingIdentifier`, `PairingConcreteOverrides`, `build_pairing_concrete`, `PairingDeclarationOverrides`, and `build_pairing_declaration` from `crate::factory::provides`; and `LookupPairingDeclarationFn` from `crate::lookup_pairing_declaration::provides`
    * `[✅]`   `AdmitPairingCompositionDepsOverrides`, the fieldless `#[derive(Default)]` unit struct, and `build_admit_pairing_composition_deps(lookup_pairing_declaration: LookupPairingDeclarationFn, overrides: AdmitPairingCompositionDepsOverrides) -> AdmitPairingCompositionDeps`, the handle handed as the leading parameter since the deps hold a collaborator the builder cannot default
    * `[✅]`   `AdmitPairingCompositionParamsOverrides`, `#[derive(Default)]`, with `pub identifier: Option<PairingConcrete>`, `pub required_curve: Option<PairingCurve>`, `pub required_verifier_group_arithmetic: Option<VerifierGroupArithmetic>`, `pub required_precompile_encodings: Option<Vec<PrecompileEncoding>>`, and `pub required_target_group_encoding: Option<TargetGroupEncodingIdentifier>`; `build_admit_pairing_composition_params(overrides: AdmitPairingCompositionParamsOverrides) -> AdmitPairingCompositionParams`, `identifier` defaulting to `PairingConcrete::Bn254Arkworks` and each required value defaulting to the matching field of `build_pairing_declaration(PairingDeclarationOverrides::default())`, the precompile encodings as a vector holding that declaration's `precompile_encoding`
    * `[✅]`   `AdmitPairingCompositionPayloadOverrides`, the fieldless `#[derive(Default)]` unit struct, and `build_admit_pairing_composition_payload(overrides: AdmitPairingCompositionPayloadOverrides) -> AdmitPairingCompositionPayload`
    * `[✅]`   `AdmitPairingCompositionSuccessReturnOverrides`, `#[derive(Default)]`, with `pub identifier: Option<PairingConcrete>`; `build_admit_pairing_composition_success_return(overrides: AdmitPairingCompositionSuccessReturnOverrides) -> AdmitPairingCompositionSuccessReturn`, `identifier` defaulting to `build_pairing_concrete(PairingConcrete::Bn254Arkworks, PairingConcreteOverrides::default())`
    * `[✅]`   `mock_admit_pairing_composition(_deps: &AdmitPairingCompositionDeps, _params: AdmitPairingCompositionParams, _payload: AdmitPairingCompositionPayload) -> AdmitPairingCompositionReturn`, returning `Ok` around `build_admit_pairing_composition_success_return` at its defaults; it is the production function type, so `let f: AdmitPairingCompositionFn = mock_admit_pairing_composition;` holds
    * `[✅]`   No builder for `PairingDisagreement` or the error enum, used by their production values; no corruptions type and no invalidator, since no type this interface owns arrives as untrusted data

  * `[✅]`   `adapters/pairing/src/admit_pairing_composition/test.rs`
    * `[✅]`   Imports from `super::provides` `admit_pairing_composition`, `AdmitPairingCompositionErrorReturn`, `AdmitPairingCompositionParamsOverrides`, `AdmitPairingCompositionSuccessReturnOverrides`, `PairingDisagreement`, and the builders `build_admit_pairing_composition_deps`, `build_admit_pairing_composition_params`, `build_admit_pairing_composition_payload`, and `build_admit_pairing_composition_success_return`; from `crate::factory::provides` `PairingConcrete`, `PairingCurve`, `VerifierGroupArithmetic`, `PrecompileEncoding`, and `TargetGroupEncodingIdentifier`; and `mock_lookup_pairing_declaration` from `crate::lookup_pairing_declaration::provides`
    * `[✅]`   `admit_returns_the_identifier_when_every_provision_agrees`
      * `[✅]`   Contract: the declaration the lookup returns provides `PairingCurve::Bn254`, `VerifierGroupArithmetic::FirstGroupOnly`, `PrecompileEncoding::Eip196Eip197`, and `TargetGroupEncodingIdentifier::Bn254V1`, and the params require those values with a precompile set holding that encoding → `Ok` whose `identifier` is `params.identifier`
      * `[✅]`   Collaborators: `mock_lookup_pairing_declaration`, the official function mock of the direct dependency, returning the declaration built at the defaults of `build_pairing_declaration`; fixtures `build_admit_pairing_composition_deps`, `build_admit_pairing_composition_params`, `build_admit_pairing_composition_payload`, and `build_admit_pairing_composition_success_return`
      * `[✅]`   Arrange: `build_admit_pairing_composition_deps` handed `mock_lookup_pairing_declaration`; `build_admit_pairing_composition_params` with the identifier override `PairingConcrete::Bls12381Arkworks`, an identifier that differs from the one the mock declaration carries, so an admission that returns the declaration's `concrete` fails, and the precompile override `vec![PrecompileEncoding::Eip2537, PrecompileEncoding::Eip196Eip197]`, a set that holds the declared encoding beside another, so an admission that requires the set to equal the declared encoding fails
      * `[✅]`   Act: `admit_pairing_composition(&deps, params, payload)`
      * `[✅]`   Assert: the result equals `Ok(build_admit_pairing_composition_success_return(AdmitPairingCompositionSuccessReturnOverrides { identifier: Some(PairingConcrete::Bls12381Arkworks) }))`
    * `[✅]`   `admit_names_a_curve_disagreement`
      * `[✅]`   Contract: the declared curve is `PairingCurve::Bn254` and the params require `PairingCurve::Bls12381`, every other provision agreeing → `Err(AdmitPairingCompositionErrorReturn::Incompatible { identifier, disagreements })` whose `disagreements` holds `PairingDisagreement::Curve { declared: PairingCurve::Bn254, required: PairingCurve::Bls12381 }` alone
      * `[✅]`   Collaborators: the fixtures and `mock_lookup_pairing_declaration` of the previous block
      * `[✅]`   Arrange: the identifier override `PairingConcrete::Bls12381Halo2curves`, which the mock declaration does not carry, so a refusal that names the declaration's member fails, and the required-curve override `PairingCurve::Bls12381`, a value that differs from the declared one in this provision alone, so an admission that compares more or fewer provisions fails
      * `[✅]`   Act: `admit_pairing_composition(&deps, params, payload)`
      * `[✅]`   Assert: the result equals `Err(AdmitPairingCompositionErrorReturn::Incompatible { identifier: PairingConcrete::Bls12381Halo2curves, disagreements: vec![PairingDisagreement::Curve { declared: PairingCurve::Bn254, required: PairingCurve::Bls12381 }] })`
    * `[✅]`   `admit_names_a_verifier_group_arithmetic_disagreement`
      * `[✅]`   Contract: the declared arithmetic is `VerifierGroupArithmetic::FirstGroupOnly` and the params require `VerifierGroupArithmetic::BothGroups`, every other provision agreeing → `Err(Incompatible)` whose `disagreements` holds `PairingDisagreement::VerifierGroupArithmetic { declared: VerifierGroupArithmetic::FirstGroupOnly, required: VerifierGroupArithmetic::BothGroups }` alone
      * `[✅]`   Collaborators: the fixtures and `mock_lookup_pairing_declaration` of the previous blocks
      * `[✅]`   Arrange: the identifier override `PairingConcrete::Bn254Halo2curves` and the required-arithmetic override `VerifierGroupArithmetic::BothGroups`, a value that differs from the declared one in this provision alone
      * `[✅]`   Act: `admit_pairing_composition(&deps, params, payload)`
      * `[✅]`   Assert: the result equals `Err(AdmitPairingCompositionErrorReturn::Incompatible { identifier: PairingConcrete::Bn254Halo2curves, disagreements: vec![PairingDisagreement::VerifierGroupArithmetic { declared: VerifierGroupArithmetic::FirstGroupOnly, required: VerifierGroupArithmetic::BothGroups }] })`
    * `[✅]`   `admit_names_a_precompile_encoding_disagreement`
      * `[✅]`   Contract: the declared encoding is `PrecompileEncoding::Eip196Eip197` and the required set is `vec![PrecompileEncoding::Eip2537]`, every other provision agreeing → `Err(Incompatible)` whose `disagreements` holds `PairingDisagreement::PrecompileEncoding { declared: PrecompileEncoding::Eip196Eip197, required: vec![PrecompileEncoding::Eip2537] }` alone
      * `[✅]`   Collaborators: the fixtures and `mock_lookup_pairing_declaration` of the previous blocks
      * `[✅]`   Arrange: the identifier override `PairingConcrete::Bn254Arkworks` and the precompile override `vec![PrecompileEncoding::Eip2537]`, a set that lacks the declared encoding, so an admission that accepts any non-empty set fails
      * `[✅]`   Act: `admit_pairing_composition(&deps, params, payload)`
      * `[✅]`   Assert: the result equals `Err(AdmitPairingCompositionErrorReturn::Incompatible { identifier: PairingConcrete::Bn254Arkworks, disagreements: vec![PairingDisagreement::PrecompileEncoding { declared: PrecompileEncoding::Eip196Eip197, required: vec![PrecompileEncoding::Eip2537] }] })`
    * `[✅]`   `admit_names_a_target_group_encoding_disagreement`
      * `[✅]`   Contract: the declared target-group encoding is `TargetGroupEncodingIdentifier::Bn254V1` and the params require `TargetGroupEncodingIdentifier::Bls12381V1`, every other provision agreeing → `Err(Incompatible)` whose `disagreements` holds `PairingDisagreement::TargetGroupEncoding { declared: TargetGroupEncodingIdentifier::Bn254V1, required: TargetGroupEncodingIdentifier::Bls12381V1 }` alone
      * `[✅]`   Collaborators: the fixtures and `mock_lookup_pairing_declaration` of the previous blocks
      * `[✅]`   Arrange: the identifier override `PairingConcrete::Bls12381Arkworks` and the required-target-group override `TargetGroupEncodingIdentifier::Bls12381V1`, a value that differs from the declared one in this provision alone
      * `[✅]`   Act: `admit_pairing_composition(&deps, params, payload)`
      * `[✅]`   Assert: the result equals `Err(AdmitPairingCompositionErrorReturn::Incompatible { identifier: PairingConcrete::Bls12381Arkworks, disagreements: vec![PairingDisagreement::TargetGroupEncoding { declared: TargetGroupEncodingIdentifier::Bn254V1, required: TargetGroupEncodingIdentifier::Bls12381V1 }] })`
    * `[✅]`   `admit_names_every_disagreeing_provision_in_order`
      * `[✅]`   Contract: every provision disagrees, the params requiring `PairingCurve::Bls12381`, `VerifierGroupArithmetic::BothGroups`, the set `vec![PrecompileEncoding::Eip2537]`, and `TargetGroupEncodingIdentifier::Bls12381V1` against the declared Bn254 values → `Err(Incompatible)` whose `disagreements` holds the curve, the verifier-group arithmetic, the precompile encoding, and the target-group encoding disagreement, in that order
      * `[✅]`   Collaborators: the fixtures and `mock_lookup_pairing_declaration` of the previous blocks
      * `[✅]`   Arrange: the identifier override `PairingConcrete::Bn254Arkworks` and an override of each required value to the Bls12-381 value named in the contract, so an admission that stops at the first disagreement, or names the disagreements in another order, fails
      * `[✅]`   Act: `admit_pairing_composition(&deps, params, payload)`
      * `[✅]`   Assert: the result equals `Err(AdmitPairingCompositionErrorReturn::Incompatible { identifier: PairingConcrete::Bn254Arkworks, disagreements: vec![PairingDisagreement::Curve { declared: PairingCurve::Bn254, required: PairingCurve::Bls12381 }, PairingDisagreement::VerifierGroupArithmetic { declared: VerifierGroupArithmetic::FirstGroupOnly, required: VerifierGroupArithmetic::BothGroups }, PairingDisagreement::PrecompileEncoding { declared: PrecompileEncoding::Eip196Eip197, required: vec![PrecompileEncoding::Eip2537] }, PairingDisagreement::TargetGroupEncoding { declared: TargetGroupEncodingIdentifier::Bn254V1, required: TargetGroupEncodingIdentifier::Bls12381V1 }] })`

  * `[✅]`   `construction`
    * `[✅]`   The function is a standalone function with no constructor; its deps are `AdmitPairingCompositionDeps`, built by the caller with a struct literal holding `lookup_pairing_declaration` in production code and by `build_admit_pairing_composition_deps` handed a lookup in a mock or test
    * `[✅]`   The function constructs no concrete and no mock concrete; the lookup it calls constructs none either

  * `[✅]`   `adapters/pairing/src/admit_pairing_composition/mod.rs`
    * `[✅]`   Module declarations: `mod interface;`, `#[cfg(test)] mod integration_test;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, `pub mod provides;`, and `#[cfg(test)] mod test;`
    * `[✅]`   Imports: `AdmitPairingCompositionDeps`, `AdmitPairingCompositionErrorReturn`, `AdmitPairingCompositionParams`, `AdmitPairingCompositionPayload`, `AdmitPairingCompositionReturn`, `AdmitPairingCompositionSuccessReturn`, and `PairingDisagreement` from `interface`; and `LookupPairingDeclarationDeps`, `LookupPairingDeclarationParams`, and `LookupPairingDeclarationPayload` from `crate::lookup_pairing_declaration::provides`
    * `[✅]`   `pub fn admit_pairing_composition(deps: &AdmitPairingCompositionDeps, params: AdmitPairingCompositionParams, _payload: AdmitPairingCompositionPayload) -> AdmitPairingCompositionReturn`, the trusted form, realizing the interaction spec's branches: one call `(deps.lookup_pairing_declaration)(&LookupPairingDeclarationDeps, LookupPairingDeclarationParams { concrete: params.identifier }, LookupPairingDeclarationPayload)` bound by the irrefutable `let Ok(looked_up) = …;`; then a vector `disagreements` built by independent `if` checks in the order curve, verifier-group arithmetic, precompile encoding, target-group encoding, each pushing its `PairingDisagreement` carrying the declared value read from `looked_up.declaration` and the required value from `params`, the precompile check pushing when `params.required_precompile_encodings` does not `contains` the declared encoding and carrying that vector; then `Ok(AdmitPairingCompositionSuccessReturn { identifier: params.identifier })` where `disagreements` is empty and `Err(AdmitPairingCompositionErrorReturn::Incompatible { identifier: params.identifier, disagreements })` otherwise
    * `[✅]`   No `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`; no `match` on `params.identifier`; no member's name, spelling, or declared value is written as a literal

  * `[✅]`   `adapters/pairing/src/admit_pairing_composition/integration_test.rs`
    * `[✅]`   Imports from `super::provides` `admit_pairing_composition`, `AdmitPairingCompositionErrorReturn`, `AdmitPairingCompositionParamsOverrides`, `AdmitPairingCompositionSuccessReturnOverrides`, `PairingDisagreement`, and the builders `build_admit_pairing_composition_deps`, `build_admit_pairing_composition_params`, `build_admit_pairing_composition_payload`, and `build_admit_pairing_composition_success_return`; from `crate::factory::provides` `PairingConcrete`, `PairingCurve`, `VerifierGroupArithmetic`, `PrecompileEncoding`, `TargetGroupEncodingIdentifier`, and `MockIPairingAdapterFailureMode`; and `lookup_pairing_declaration` from `crate::lookup_pairing_declaration::provides`, the real function every block hands to the deps
    * `[✅]`   `a_bn254_member_is_admitted_against_bn254_requirements`
      * `[✅]`   Contract: the entry `admission_compares_the_provisions_of_the_member_named`; `PairingConcrete::Bn254Halo2curves` with the required values `PairingCurve::Bn254`, `VerifierGroupArithmetic::FirstGroupOnly`, the set `vec![PrecompileEncoding::Eip196Eip197]`, and `TargetGroupEncodingIdentifier::Bn254V1` → `Ok` whose `identifier` is `PairingConcrete::Bn254Halo2curves`
      * `[✅]`   Boundary: `admit_pairing_composition` then the real `lookup_pairing_declaration` then the declaration constant of the member named
      * `[✅]`   Mocked: nothing; the route has no outer-edge collaborator
      * `[✅]`   Arrange: `build_admit_pairing_composition_deps` handed the real `lookup_pairing_declaration`; the identifier override `PairingConcrete::Bn254Halo2curves` and the required values stated as independent literals in the contract, a member whose provisions are the Bn254 values, so an admission that reads another curve's declaration fails
      * `[✅]`   Act: `admit_pairing_composition(&deps, params, payload)`
      * `[✅]`   Assert: the result equals `Ok(build_admit_pairing_composition_success_return(AdmitPairingCompositionSuccessReturnOverrides { identifier: Some(PairingConcrete::Bn254Halo2curves) }))`
    * `[✅]`   `a_bls12_381_member_is_admitted_against_bls12_381_requirements`
      * `[✅]`   Contract: the entry `admission_compares_the_provisions_of_the_member_named`; `PairingConcrete::Bls12381Arkworks` with the required values `PairingCurve::Bls12381`, `VerifierGroupArithmetic::BothGroups`, the set `vec![PrecompileEncoding::Eip2537]`, and `TargetGroupEncodingIdentifier::Bls12381V1` → `Ok` whose `identifier` is `PairingConcrete::Bls12381Arkworks`
      * `[✅]`   Boundary: the route of the previous block
      * `[✅]`   Mocked: nothing; the route has no outer-edge collaborator
      * `[✅]`   Arrange: the real lookup in the deps; the identifier override `PairingConcrete::Bls12381Arkworks` and the required values stated as independent literals in the contract, a member of the other curve, so an admission that reads a Bn254 declaration fails
      * `[✅]`   Act: `admit_pairing_composition(&deps, params, payload)`
      * `[✅]`   Assert: the result equals `Ok(build_admit_pairing_composition_success_return(AdmitPairingCompositionSuccessReturnOverrides { identifier: Some(PairingConcrete::Bls12381Arkworks) }))`
    * `[✅]`   `a_bn254_member_is_refused_against_bls12_381_requirements`
      * `[✅]`   Contract: the entry `admission_compares_the_provisions_of_the_member_named`; `PairingConcrete::Bn254Arkworks` with the Bls12-381 required values of the previous block → `Err(Incompatible)` naming `PairingConcrete::Bn254Arkworks` whose `disagreements` holds the curve, verifier-group arithmetic, precompile encoding, and target-group encoding disagreement with the declared values `PairingCurve::Bn254`, `VerifierGroupArithmetic::FirstGroupOnly`, `PrecompileEncoding::Eip196Eip197`, and `TargetGroupEncodingIdentifier::Bn254V1`
      * `[✅]`   Boundary: the route of the previous blocks
      * `[✅]`   Mocked: nothing; the route has no outer-edge collaborator
      * `[✅]`   Arrange: the real lookup in the deps; the identifier override `PairingConcrete::Bn254Arkworks` and the Bls12-381 required values as independent literals, a member whose every provision differs from them, so an admission that reads another curve's declaration or compares fewer provisions fails
      * `[✅]`   Act: `admit_pairing_composition(&deps, params, payload)`
      * `[✅]`   Assert: the result equals the whole `Err(AdmitPairingCompositionErrorReturn::Incompatible { identifier: PairingConcrete::Bn254Arkworks, disagreements })` with `disagreements` holding every disagreement in order, each stated with its declared and required values as literals
    * `[✅]`   `a_bls12_381_member_is_refused_against_bn254_requirements`
      * `[✅]`   Contract: the entry `admission_compares_the_provisions_of_the_member_named`; `PairingConcrete::Bls12381Halo2curves` with the Bn254 required values of `a_bn254_member_is_admitted_against_bn254_requirements` → `Err(Incompatible)` naming `PairingConcrete::Bls12381Halo2curves` whose `disagreements` holds every disagreement with the declared values `PairingCurve::Bls12381`, `VerifierGroupArithmetic::BothGroups`, `PrecompileEncoding::Eip2537`, and `TargetGroupEncodingIdentifier::Bls12381V1`
      * `[✅]`   Boundary: the route of the previous blocks
      * `[✅]`   Mocked: nothing; the route has no outer-edge collaborator
      * `[✅]`   Arrange: the real lookup in the deps; the identifier override `PairingConcrete::Bls12381Halo2curves` and the Bn254 required values as independent literals, a member of the other curve from the previous block, so an admission that reads a fixed member's declaration fails a refusal block
      * `[✅]`   Act: `admit_pairing_composition(&deps, params, payload)`
      * `[✅]`   Assert: the result equals the whole `Err(AdmitPairingCompositionErrorReturn::Incompatible { identifier: PairingConcrete::Bls12381Halo2curves, disagreements })` with `disagreements` holding every disagreement in order, each stated with its declared and required values as literals
    * `[✅]`   `the_mock_member_is_admitted_against_the_provisions_its_declaration_states`
      * `[✅]`   Contract: the entry `mock_admission_compares_the_mock_declaration`; `PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG1NotOnCurve)` with the Bn254 required values of `a_bn254_member_is_admitted_against_bn254_requirements` → `Ok` whose `identifier` is that mock identifier
      * `[✅]`   Boundary: `admit_pairing_composition` then the real `lookup_pairing_declaration` then `MOCK_I_PAIRING_ADAPTER_DECLARATION`
      * `[✅]`   Mocked: the mock member itself, so the test proves the mock declaration is read and no real member's
      * `[✅]`   Arrange: the real lookup in the deps; the identifier override `PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG1NotOnCurve)`, a mode other than `Succeeds`, and the Bn254 required values as independent literals
      * `[✅]`   Act: `admit_pairing_composition(&deps, params, payload)`
      * `[✅]`   Assert: the result equals `Ok(build_admit_pairing_composition_success_return(AdmitPairingCompositionSuccessReturnOverrides { identifier: Some(PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG1NotOnCurve)) }))`
    * `[✅]`   `the_mock_member_is_refused_naming_its_identifier`
      * `[✅]`   Contract: the entry `mock_admission_compares_the_mock_declaration`; `PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG2NotOnCurve)` with the Bls12-381 required values of `a_bn254_member_is_refused_against_bls12_381_requirements` → `Err(Incompatible)` naming that mock identifier whose `disagreements` holds every disagreement with the declared Bn254 values the mock declaration states
      * `[✅]`   Boundary: the route of the previous block
      * `[✅]`   Mocked: the mock member itself, so the test proves the identifier is carried into the refusal
      * `[✅]`   Arrange: the real lookup in the deps; the identifier override `PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG2NotOnCurve)`, a mode that differs from the previous block's, and the Bls12-381 required values as independent literals, so a refusal that names another mode or a real member fails
      * `[✅]`   Act: `admit_pairing_composition(&deps, params, payload)`
      * `[✅]`   Assert: the result equals the whole `Err(AdmitPairingCompositionErrorReturn::Incompatible { identifier: PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG2NotOnCurve), disagreements })` with `disagreements` holding every disagreement in order, each stated with its declared and required values as literals
    * `[✅]`   Reports its enumeration: each entry the private surface lists and the block that proves it

  * `[✅]`   `adapters/pairing/src/admit_pairing_composition/provides.rs`
    * `[✅]`   `pub use super::admit_pairing_composition;`, `pub use super::interface::*;`, and `#[cfg(any(test, feature = "mocks"))] pub use super::mock::*;`

  * `[✅]`   `adapters/pairing/src/lib.rs`
    * `[✅]`   The crate barrel gains `mod admit_pairing_composition;` and `pub use admit_pairing_composition::provides::*;`, so the crate's public surface carries this family function beside the `factory` module's `provides`; no concrete is re-exported

  * `[✅]`   `adapters/pairing/tests/admit_pairing_composition_integration_test.rs`
    * `[✅]`   Consumes the crate's public surface and the official mocks under the `mocks` feature: `admit_pairing_composition`, `AdmitPairingCompositionErrorReturn`, `PairingDisagreement`, `lookup_pairing_declaration`, `PAIRING_CONCRETES`, `TARGET_GROUP_ENCODING_IDENTIFIERS`, `PairingConcrete`, `PairingCurve`, `VerifierGroupArithmetic`, `PrecompileEncoding`, `TargetGroupEncodingIdentifier`, `MockIPairingAdapterFailureMode`, and the builders `build_admit_pairing_composition_deps`, `build_admit_pairing_composition_params`, `build_admit_pairing_composition_payload`, `build_admit_pairing_composition_success_return`, and `build_lookup_pairing_declaration_deps`, `build_lookup_pairing_declaration_params`, and `build_lookup_pairing_declaration_payload` with their overrides; names no concrete
    * `[✅]`   `admission_compares_the_provisions_of_the_member_named`
      * `[✅]`   Contract: the own entry of the spec's own entries and public surface; each member of `PAIRING_CONCRETES` with the required values read from the declaration the public `lookup_pairing_declaration` returns for it → `Ok` whose `identifier` is that member; and, for each member, the required target-group encoding replaced by a value of `TARGET_GROUP_ENCODING_IDENTIFIERS` the member does not declare → `Err(Incompatible)` naming the member whose `disagreements` holds one `PairingDisagreement::TargetGroupEncoding` carrying the member's declared value and that required value
      * `[✅]`   Boundary: the public call `admit_pairing_composition`, the route extended with the real `lookup_pairing_declaration` and the member's declaration constant
      * `[✅]`   Mocked: nothing; the route has no outer-edge collaborator, so the test does not prove the mock member
      * `[✅]`   Arrange: the block body written once and run over every member of `PAIRING_CONCRETES`, the deps built with the public `lookup_pairing_declaration`, the requirements built from the declaration that function returns for the member, and, for the refusal, the target-group override chosen from `TARGET_GROUP_ENCODING_IDENTIFIERS` as a value unequal to the declared one, so an admission that reads one fixed member's declaration, or that ignores the target-group requirement, fails
      * `[✅]`   Act: `admit_pairing_composition(&deps, params, payload)` per member and per requirement set
      * `[✅]`   Assert: the first result for each member equals `Ok(build_admit_pairing_composition_success_return(...))` built with the member as the identifier override; the second result equals `Err(AdmitPairingCompositionErrorReturn::Incompatible { identifier, disagreements })` with the member as the identifier and `disagreements` holding a single `PairingDisagreement::TargetGroupEncoding { declared, required }`, where `declared` is the target-group encoding of the declaration the public lookup returned for the member and `required` is the chosen value
    * `[✅]`   `admission_names_each_disagreeing_provision_for_each_declared_member`
      * `[✅]`   Contract: the own entry of the spec's own entries and public surface; each member of `PAIRING_CONCRETES` with the required values read from the declaration the public `lookup_pairing_declaration` returns for it, one provision at a time replaced by the same provision of the declaration the public lookup returns for a member of `PAIRING_CONCRETES` whose declared curve differs → `Err(Incompatible)` naming the member whose `disagreements` holds the one `PairingDisagreement` of the replaced provision, carrying the member's declared value and the replaced required value, the precompile provision carrying the required set holding the other member's declared encoding
      * `[✅]`   Boundary: the public call `admit_pairing_composition`, the route extended with the real `lookup_pairing_declaration` and the declaration constants of the member and of the member of the other curve
      * `[✅]`   Mocked: nothing; the route has no outer-edge collaborator, so the test does not prove the mock member
      * `[✅]`   Arrange: the block body written once and run over every member of `PAIRING_CONCRETES` and, within each, over the curve, verifier-group arithmetic, precompile encoding, and target-group encoding provisions, the other-curve member chosen from `PAIRING_CONCRETES` by comparing the declared curves the public lookup returns, so an admission that omits a provision, names a provision that agrees, or names another provision fails
      * `[✅]`   Act: `admit_pairing_composition(&deps, params, payload)` per member and per replaced provision
      * `[✅]`   Assert: each result equals the whole `Err(AdmitPairingCompositionErrorReturn::Incompatible { identifier, disagreements })` with the member as the identifier and `disagreements` holding the one `PairingDisagreement` of the replaced provision, its `declared` read from the member's declaration and its `required` from the other-curve member's declaration
    * `[✅]`   `admission_names_every_disagreeing_provision_in_order_for_each_declared_member`
      * `[✅]`   Contract: the own entry of the spec's own entries and public surface; each member of `PAIRING_CONCRETES` with every required value taken from the declaration the public lookup returns for a member of `PAIRING_CONCRETES` whose declared curve differs → `Err(Incompatible)` naming the member whose `disagreements` holds the curve, verifier-group arithmetic, precompile encoding, and target-group encoding disagreement, in that order, each carrying the member's declared value and the other member's value
      * `[✅]`   Boundary: the route of the previous block
      * `[✅]`   Mocked: nothing; the route has no outer-edge collaborator
      * `[✅]`   Arrange: the block body written once and run over every member of `PAIRING_CONCRETES`, the requirements taken wholly from the other-curve member's declaration, so an admission that stops at the first disagreement, or names the disagreements in another order, fails
      * `[✅]`   Act: `admit_pairing_composition(&deps, params, payload)` per member
      * `[✅]`   Assert: each result equals the whole `Err(AdmitPairingCompositionErrorReturn::Incompatible { identifier, disagreements })` with the member as the identifier and `disagreements` holding every disagreement in order, each `declared` read from the member's declaration and each `required` from the other-curve member's declaration
    * `[✅]`   `mock_admission_compares_the_mock_declaration`, under `#[cfg(feature = "mocks")]`
      * `[✅]`   Contract: the own entry of the spec's own entries and public surface; `PairingConcrete::Mock(mode)` with the Bn254 required values → `Ok` whose `identifier` is `PairingConcrete::Mock(mode)`, and with a required curve of `PairingCurve::Bls12381` and the other Bn254 values → `Err(Incompatible)` naming `PairingConcrete::Mock(mode)` whose `disagreements` holds a single `PairingDisagreement::Curve` with the declared `PairingCurve::Bn254`
      * `[✅]`   Boundary: the public call `admit_pairing_composition`, the route extended with the real `lookup_pairing_declaration` and `MOCK_I_PAIRING_ADAPTER_DECLARATION`
      * `[✅]`   Mocked: the mock member itself, so the test proves the identifier is carried and the mock declaration is compared, and no real member's
      * `[✅]`   Arrange: the identifier override `PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG1NotOnCurve)`, a mode other than `Succeeds`, called once with the Bn254 required values and once with the required curve `PairingCurve::Bls12381`
      * `[✅]`   Act: `admit_pairing_composition(&deps, params, payload)` per requirement set
      * `[✅]`   Assert: the first result equals `Ok(build_admit_pairing_composition_success_return(...))` with the mock identifier as the identifier override, and the second equals `Err(AdmitPairingCompositionErrorReturn::Incompatible { identifier: PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG1NotOnCurve), disagreements: vec![PairingDisagreement::Curve { declared: PairingCurve::Bn254, required: PairingCurve::Bls12381 }] })`
    * `[✅]`   Reports its enumeration: each entry the public surface lists and the block that proves it

  * `[✅]`   `directionality`
    * `[✅]`   `admit_pairing_composition` depends on the `factory` module's surface through `crate::factory::provides` and on the lookup's `provides` through `crate::lookup_pairing_declaration::provides`; it depends on no concrete; neither the `factory` module, any concrete, nor the lookup depends on this module
    * `[✅]`   `pairing/factory` consumes this module through its `provides`; among repository crates the crate depends on `crates/domain` alone at runtime, and this module adds no dependency

  * `[✅]`   `requirements`
    * `[✅]`   `admit_returns_the_identifier_when_every_provision_agrees` passes, proving a member whose declared provisions are the required ones is admitted and named from the params (CR-10, the suite's required target-group encoding and verifier capabilities compared before construction)
    * `[✅]`   `admit_names_a_curve_disagreement`, `admit_names_a_verifier_group_arithmetic_disagreement`, `admit_names_a_precompile_encoding_disagreement`, and `admit_names_a_target_group_encoding_disagreement` pass, proving each disagreement is refused as `Incompatible` naming the member and the declared and required values
    * `[✅]`   `admit_names_every_disagreeing_provision_in_order` passes, proving every disagreeing provision is named in the fixed order
    * `[✅]`   `a_bn254_member_is_admitted_against_bn254_requirements`, `a_bls12_381_member_is_admitted_against_bls12_381_requirements`, `a_bn254_member_is_refused_against_bls12_381_requirements`, `a_bls12_381_member_is_refused_against_bn254_requirements`, `the_mock_member_is_admitted_against_the_provisions_its_declaration_states`, and `the_mock_member_is_refused_naming_its_identifier` pass, proving the provisions compared are those the real lookup returns for the member named
    * `[✅]`   `admission_compares_the_provisions_of_the_member_named`, `admission_names_each_disagreeing_provision_for_each_declared_member`, `admission_names_every_disagreeing_provision_in_order_for_each_declared_member`, and `mock_admission_compares_the_mock_declaration` pass, proving the public entry admits and refuses through the crate's public surface with no concrete constructed
    * `[✅]`   `cargo check --all-targets --all-features`, `cargo fmt --check`, and `cargo clippy --all-targets --all-features` complete without a warning in this module and in the `factory` module's interface
    * `[✅]`   Code outside `adapters/pairing` naming a concrete fails to compile; the crate's public surface is the `factory` module's `provides` and the `provides` of this module and of the other family-owned functions

* `[✅]`   `pairing/factory` **Pairing factory constructing the concrete the selection names after `admit_pairing_composition` admits it against the requirements its params carry, and handing it to a consumer generic over the family's arithmetic and reference traits; carries the family's integration test proving the libraries of each curve agree**

  * `[✅]`   `objective`
    * `[✅]`   Problem: a consumer obtains a pairing adapter only through the family's generic surface, never by naming a concrete, and a concrete whose declared provisions the requirements its params carry do not match is refused by the family's composition admission before anything is constructed (CR-10; Composition Boundary)
    * `[✅]`   Problem: the KEM, the envelope, and the delivery proof reach a pairing concrete only through `create_pairing` and `IPairingConsumer`; a capsule encapsulated under one library must decapsulate to the same wrapping key under the other, so both libraries on a curve encode a target-group value to the same bytes; and the bytes are fixed by the suite's target-group encoding identifier, so the factory admits a concrete only through `admit_pairing_composition`, which compares the identifier the concrete declares with the one the suite requires (CR-08; CR-10; CR-11; the product requirements' canonical target-group value position)
    * `[✅]`   Problem: a consumer that mirrors the reference to a target suite reaches the group order and the outside-the-subgroup encodings only through the concrete the factory hands it, so the consumer surface requires the reference trait of that concrete; and two libraries of one curve are interchangeable under one suite only if they return the same bytes for both, which agreement between each concrete and its own decoder does not prove (CR-10; CR-11; the dependency map's `pairing/factory` row; the Pairing adapters and key derivation milestone's exit)
    * `[✅]`   Functional: given the concrete the selection names and the curve, verifier-group arithmetic, precompile encodings, and target-group encoding identifier the consumer requires, the factory calls `admit_pairing_composition` once, before any construction, and a refusal it returns is carried whole in the factory's error arm, with no construction and no call to the consumer
    * `[✅]`   Functional: an admitted concrete is constructed and handed to a consumer generic over a concrete implementing `IPairingArithmetic` and `IPairingReference`, which reads `P::DECLARATION` and `P::CONCRETE`, and the consumer's output is returned; the consumer never names the concrete; a consumer whose `consume_pairing` is written against `IPairingAdapter` or `IPairingArithmetic` alone is a valid implementation, an implementation's bound being no stricter than the trait's
    * `[✅]`   Functional: a concrete's constructor error is returned unchanged in the factory's error arm, one variant per concrete
    * `[✅]`   Functional: every concrete obtained from the factory computes through the family's traits; the arkworks and halo2curves concretes on BN254 encode the pairing of the same inputs to the same bytes, and likewise on BLS12-381, each concrete's definition test in its own node passing
    * `[✅]`   Functional: through the family's surface alone, both concretes of BN254 return the same scalar field order, no first-group encoding, and the same second-group encoding, which `decode_g2` refuses as outside the subgroup; both concretes of BLS12-381 return the same scalar field order and the same encoding in each group, each of which its group's decoder refuses as outside the subgroup
    * `[✅]`   Non-functional: a further concrete changes this node by its variant in the error enum, its arm here, and its entry in `PRECOMPILE_ENCODINGS` where it provides an encoding no earlier member provides; its module, its selection variant, its entry in `PAIRING_CONCRETES`, its entry in `TARGET_GROUP_ENCODING_IDENTIFIERS`, and its arms in the lookup and the selection admission are its own increments, and no consumer changes

  * `[✅]`   `role`
    * `[✅]`   Adapter family factory: the implementation of the `factory` module, which is the pairing family's construction point and the crate's public surface; binds the arithmetic and reference traits into the consumer surface
    * `[✅]`   Hands the concrete to a consumer rather than returning it, because the family's traits carry associated types and so cannot be returned as one type across concretes; the consumer is written once, generic over the family's traits, and the factory instantiates it for the concrete it constructs
    * `[✅]`   Admits by calling `admit_pairing_composition` before any construction, and compares no declaration with a requirement itself, reads no declaration, and restates no member's declared value
    * `[✅]`   Does not choose a concrete when none is named; the composition names one from the configuration catalogue
    * `[✅]`   Does not read the chain family; the run reads the precompile encodings the configured chain member's declaration states through `chain/lookup_chain_declaration` and passes them as the `required_precompile_encodings` of `CreatePairingParams`, values of this family's `PrecompileEncoding`
    * `[✅]`   Does not decode or validate configuration; the configuration registry hands the factory typed params
    * `[✅]`   Does not edit any concrete
    * `[✅]`   Carries the family's integration test; does not carry a commit

  * `[✅]`   `module`
    * `[✅]`   Bounded context: the `factory` module of `adapters/pairing`, holding the factory function, its deps, params, payload, and return types, its signature type, the consumer trait with its params and payload types, the declared set `PRECOMPILE_ENCODINGS`, the function mock and builders, the unit test, the private integration test, and the crate's public integration test under `adapters/pairing/tests`
    * `[✅]`   The node's files: `adapters/pairing/src/factory/interface.rs`, `interaction.spec.md`, `mock.rs`, `test.rs`, `mod.rs`, `integration_test.rs`, and `provides.rs`, and `adapters/pairing/tests/factory_integration_test.rs`
    * `[✅]`   Outside: every concrete's behavior, the family's traits, declaration, member identifier, `PAIRING_CONCRETES`, and `TARGET_GROUP_ENCODING_IDENTIFIERS`, which `pairing/bn254_arkworks` authors and each further concrete increments, the declaration lookup, the selection admission, the composition admission, the chain family's declarations, the configuration catalogue, and every consumer of the family

  * `[✅]`   `deps`
    * `[✅]`   Each concrete, through its `provides`: `Bn254ArkworksPairing`, `Bn254Halo2curvesPairing`, `Bls12381ArkworksPairing`, and `Bls12381Halo2curvesPairing`, each with its constructor params, `try_new`, `DECLARATION`, `CONCRETE`, and implementations of `IPairingAdapter`, `IPairingArithmetic`, and `IPairingReference`, as `pairing/bn254_arkworks`, `pairing/bn254_halo2curves`, `pairing/bls12_381_arkworks`, and `pairing/bls12_381_halo2curves` state them; the factory constructs its concretes, the family form's recorded cycle
    * `[✅]`   The `factory` module's interface: `IPairingAdapter`, `IPairingArithmetic`, `IPairingReference`, `PairingDeclaration`, `PairingConcrete`, `PairingCurve`, `VerifierGroupArithmetic`, `PrecompileEncoding`, `TargetGroupEncodingIdentifier`, `PAIRING_CONCRETES`, `TARGET_GROUP_ENCODING_IDENTIFIERS`, and every method's types, as `pairing/bn254_arkworks` authors them and each further concrete increments them, and, under `#[cfg(any(test, feature = "mocks"))]`, `MockIPairingAdapter`, `MockIPairingAdapterConstructorParams`, and `MockIPairingAdapterFailureMode`
    * `[✅]`   `pairing/admit_pairing_composition`, through `crate::admit_pairing_composition::provides`: `AdmitPairingCompositionFn`, the type of the handle the deps hold; `AdmitPairingCompositionDeps`, `AdmitPairingCompositionParams` with its fields `identifier`, `required_curve`, `required_verifier_group_arithmetic`, `required_precompile_encodings`, and `required_target_group_encoding`, `AdmitPairingCompositionPayload`, and `AdmitPairingCompositionErrorReturn`, which the function builds, passes, and carries; and, under the `mocks` feature and in the crate's own test build, `mock_admit_pairing_composition`, `build_admit_pairing_composition_deps`, and `AdmitPairingCompositionDepsOverrides`, the function mock and the deps builder the unit test and the mock builder hand to the deps, and `admit_pairing_composition`, the production function the private and public integration tests hand to the deps; direction inward, a family function on the family's factory module
    * `[✅]`   `pairing/lookup_pairing_declaration`, through `crate::lookup_pairing_declaration::provides`: `LookupPairingDeclarationFn`, the type of the handle `build_admit_pairing_composition_deps` takes as its leading parameter, and `lookup_pairing_declaration` and, under the `mocks` feature and in the crate's own test build, `mock_lookup_pairing_declaration`, the handles the integration tests and the unit test hand to it; this node calls the lookup nowhere itself
    * `[✅]`   `core::convert::Infallible`, standard library, each concrete's constructor error carried in the factory's error arm
    * `[✅]`   `domain` with its `mocks` feature, the crate's dev-dependency, for `build_secret` and `SecretConstructorParamsOverrides` in `integration_test.rs` and `tests/factory_integration_test.rs`; `pairing` itself, as the public integration test's dependency through the crate's public surface
    * `[✅]`   No external crate beyond those `adapters/pairing/Cargo.toml` holds; this node edits no manifest
    * `[✅]`   Reverse dependencies: `kem/bb1_depth_one` and `harness-crypto/main`, each through `create_pairing` and an `IPairingConsumer`

  * `[✅]`   `context_slice`
    * `[✅]`   From `pairing/admit_pairing_composition`: the function type `AdmitPairingCompositionFn`, held in deps and called once, with `&deps.admit_pairing_composition_deps`, an `AdmitPairingCompositionParams` built from `params.concrete` and the four `required_*` fields, and the fieldless `AdmitPairingCompositionPayload`; its `Err` carried whole in the error arm and its `Ok` identifier not read
    * `[✅]`   From each concrete's `provides`: `DECLARATION`, a `PairingDeclaration`, which a consumer reads through `P::DECLARATION` and this function does not read, and `try_new(params) -> Result<Self, Infallible>` taking the concrete's fieldless constructor params; from its `IPairingAdapter` implementation, `CONCRETE`, a `PairingConcrete`
    * `[✅]`   From the `factory` module's mock: `MockIPairingConsumer`, implementing `IPairingConsumer` with `type Output = ();`, the official consumer mock the unit test hands the factory
    * `[✅]`   In the private and public integration tests: `ISampleUniformScalar::sample_from_uniform_bytes(SampleUniformScalarParams, SampleUniformScalarPayload { uniform })`, which reads `uniform` through `Secret::expose` and returns `scalar: Secret<P::Scalar>`; `IPairingAdapter::encode_scalar(&self, EncodeScalarParams, EncodeScalarPayload { scalar })` returning `bytes: Secret<P::EncodedScalar>`; `IPairingAdapter::decode_g1`, `decode_g2`, `g1_generator`, `g2_generator`, and `mul_g1` with their params, payload, and error types; `IPairingArithmetic::pairing_product` and `encode_gt`; `IPairingReference::scalar_field_order`, `g1_outside_subgroup_encoding`, and `g2_outside_subgroup_encoding`; `PAIRING_CONCRETES`, the declared set each block runs, and `PairingConcrete::Mock(MockIPairingAdapterFailureMode)`, the selection a failure-mode block makes; `Secret::expose(&self) -> &T`; and, from `domain`'s mocks, `build_secret(value: Vec<u8>, SecretConstructorParamsOverrides) -> Secret<Vec<u8>>`

  * `[✅]`   `adapters/pairing/src/factory/interface.rs`
    * `[✅]`   Imports `AdmitPairingCompositionDeps`, `AdmitPairingCompositionErrorReturn`, and `AdmitPairingCompositionFn` from `crate::admit_pairing_composition::provides`; declares only the items below, never an item `pairing/bn254_arkworks` or a later concrete authors
    * `[✅]`   `IPairingConsumer`, `pub trait IPairingConsumer` with `type Output;` and `fn consume_pairing<P: IPairingArithmetic + IPairingReference>(&self, params: ConsumePairingParams, payload: ConsumePairingPayload<P>) -> Self::Output;`, the work a composition performs with whichever concrete the factory constructs
    * `[✅]`   `ConsumePairingParams`, the fieldless struct `pub struct ConsumePairingParams;`
    * `[✅]`   `ConsumePairingPayload<P: IPairingAdapter>`, a struct with only `pub adapter: P`; the consumer reads `P::DECLARATION` and `P::CONCRETE`, so neither metadata value can be paired with another adapter
    * `[✅]`   `CreatePairingDeps<C>`, a struct with `pub consumer: C`, the collaborator the factory hands the concrete to, `pub admit_pairing_composition: AdmitPairingCompositionFn`, the composition admission the factory calls before it constructs anything, and `pub admit_pairing_composition_deps: AdmitPairingCompositionDeps`, the deps that admission is called with
    * `[✅]`   `CreatePairingParams`, a struct with `pub concrete: PairingConcrete`, the selection, and the values the composition admission compares the concrete's declaration with, each a selection the consumer states: `pub required_curve: PairingCurve`, `pub required_verifier_group_arithmetic: VerifierGroupArithmetic`, `pub required_precompile_encodings: Vec<PrecompileEncoding>`, and `pub required_target_group_encoding: TargetGroupEncodingIdentifier`
    * `[✅]`   `CreatePairingPayload`, the fieldless struct `pub struct CreatePairingPayload;`, since the factory operates on no data
    * `[✅]`   `CreatePairingSuccessReturn<O>`, a struct with `pub output: O`
    * `[✅]`   `CreatePairingErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]`, which `Infallible` and `AdmitPairingCompositionErrorReturn` satisfy, and the variants `Composition(AdmitPairingCompositionErrorReturn)`, the composition admission's refusal carried whole, `Bn254Arkworks(Infallible)`, `Bn254Halo2curves(Infallible)`, `Bls12381Arkworks(Infallible)`, and `Bls12381Halo2curves(Infallible)`, each concrete's constructor error carried unchanged in its own variant, and, under `#[cfg(any(test, feature = "mocks"))]`, the variant `MockIPairingAdapter(Infallible)`, the mock concrete's constructor error carried unchanged
    * `[✅]`   `CreatePairingReturn<O>`, the alias `Result<CreatePairingSuccessReturn<O>, CreatePairingErrorReturn>`
    * `[✅]`   `CreatePairingFn<C>`, the alias `fn(&CreatePairingDeps<C>, CreatePairingParams, CreatePairingPayload) -> CreatePairingReturn<<C as IPairingConsumer>::Output>`
    * `[✅]`   `PRECOMPILE_ENCODINGS`, the constant `pub const PRECOMPILE_ENCODINGS: &[PrecompileEncoding] = &[PrecompileEncoding::Eip196Eip197, PrecompileEncoding::Eip2537];`, the declared set of real encodings; it never holds the mock variant
    * `[✅]`   No derives on any type this node declares beyond those stated; the file names no vendor and no concrete

  * `[✅]`   `adapters/pairing/src/factory/interaction.spec.md`
    * `[✅]`   The title `` # `factory` — interaction spec `` and one opening sentence stating the file as the branch contract for the `factory` module of the `pairing` crate, the pairing family's construction point, admitting the concrete the selection names through `admit_pairing_composition` and handing it to a consumer generic over `IPairingArithmetic` and `IPairingReference`, each branch stating condition, decision, dependency call, and the exact return outcome; then one `##` section headed by the full signature `create_pairing<C: IPairingConsumer>(deps: &CreatePairingDeps<C>, params: CreatePairingParams, payload: CreatePairingPayload) -> CreatePairingReturn<C::Output>`, then `## Ordering and edges`
    * `[✅]`   The section opens by stating that the decision is a `match` on `params.concrete`, one arm per `PairingConcrete` variant, `Bn254Arkworks`, `Bn254Halo2curves`, `Bls12381Arkworks`, and `Bls12381Halo2curves`, exhaustive, so a variant with no arm fails to compile, each arm constructing the concrete whose `CONCRETE` is the arm's variant and running the same contract against it; then a table with the columns `Branch`, `Condition`, `Decision`, `Dependency call`, and `Outcome` holding, in order: composition refused, condition the composition admission returns `Err` for `params.concrete` and the four `required_*` values, decision the admission's result, read before any construction, dependency call `(deps.admit_pairing_composition)(&deps.admit_pairing_composition_deps, AdmitPairingCompositionParams { identifier: params.concrete, required_curve: params.required_curve, required_verifier_group_arithmetic: params.required_verifier_group_arithmetic, required_precompile_encodings: params.required_precompile_encodings, required_target_group_encoding: params.required_target_group_encoding }, AdmitPairingCompositionPayload)`, exactly once, outcome `Err(CreatePairingErrorReturn::Composition(error))` carrying that admission's error whole, with nothing constructed and the consumer not called; admitted, condition the admission returns `Ok`, decision the admission's result, dependency call the concrete's `try_new` with its fieldless constructor params, exactly once, its success destructured irrefutably because its error arm is uninhabited, then `deps.consumer.consume_pairing(ConsumePairingParams, ConsumePairingPayload { adapter })`, exactly once, the consumer reading `P::DECLARATION` and `P::CONCRETE`, outcome `Ok(CreatePairingSuccessReturn { output })` holding the consumer's output; the section closes by stating that each concrete's own variant of `CreatePairingErrorReturn` carries that concrete's constructor error in the return union unchanged and, every constructor error being `Infallible`, no branch produces one
    * `[✅]`   `Ordering and edges`, a bulleted section: `params.concrete` selects the concrete, the four `required_*` params are the requirements the composition admission compares its declaration with, and `payload` carries nothing and is not read; the composition admission is called once in every arm and precedes construction in every arm, so a refused concrete is never constructed and `deps.consumer` is never touched; the factory compares no declaration itself; the consumer is generic over `P: IPairingArithmetic + IPairingReference`, instantiated inside the arm for the concrete the arm constructs, and never names the concrete itself
    * `[✅]`   The mock arm, under `#[cfg(any(test, feature = "mocks"))]`: `params.concrete` of `PairingConcrete::Mock(mode)` takes the same decision, admission, and outcome as every arm, calling the composition admission with `PairingConcrete::Mock(mode)` as the identifier and constructing the concrete by `MockIPairingAdapter::try_new(MockIPairingAdapterConstructorParams { failure_mode: mode })`, exactly once
    * `[✅]`   `Integration: own entries`, a bulleted section, each entry named and proven by the private integration block of the same name in `integration_test.rs`, run per case over `PAIRING_CONCRETES` and `PairingConcrete::Mock(MockIPairingAdapterFailureMode::Succeeds)` — a failure-mode entry over the mock selection under its mode alone — and, where the public surface lists it, by the public integration block of the same name over `PAIRING_CONCRETES`; every entry's params come from `build_create_pairing_params` naming only `concrete` unless the entry states its requirement, so the builder's defaults, which are the requirements the concrete's own declaration states, admit, and a consumer written for the entry's work against the narrowest family trait that work needs
      * `[✅]`   `the_consumer_receives_the_concrete_the_params_name`: condition `params.concrete` is a variant of `PAIRING_CONCRETES` and the composition admission passes, the private block adding the case `PairingConcrete::Mock(MockIPairingAdapterFailureMode::Succeeds)`, whose `P::CONCRETE` is `Mock(Succeeds)` and so equals `params.concrete` as it does for every case; outcome `Ok` whose output is the consumer's reading of `P::CONCRETE`, equal to `params.concrete`; variation every variant in turn, so an arm that constructs a sibling concrete, including the other library of its curve, fails; edge a consumer written against `IPairingAdapter` alone is a valid implementation
      * `[✅]`   `a_sampled_scalar_encodes_through_secret`: condition the consumer builds a `Secret<Vec<u8>>` of `P::Scalar::UNIFORM_BYTES_LENGTH` bytes, every byte zero but the last, which is `7`, samples it with `sample_from_uniform_bytes`, and encodes the sampled scalar with `encode_scalar`; outcome `Ok` whose output is the encoded scalar read through `Secret::expose`, 31 zero bytes then `7`, the 32 big-endian bytes of the integer seven a scalar below the group order encodes to; variation the nonzero last byte, which separates a sampled input from a zero-filled input and from a dropped input; edge the uniform input, the sampled scalar, and the encoding each cross the family inside `Secret` and are read only through `Secret::expose`
      * `[✅]`   `a_sampling_refusal_reaches_the_caller_unchanged`: condition the consumer of `a_sampled_scalar_encodes_through_secret` given a uniform `Secret<Vec<u8>>` one byte shorter than `P::Scalar::UNIFORM_BYTES_LENGTH`; outcome `Ok` whose output is `Err(SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual: 63 })`, the consumer propagating the sampling error unchanged; variation the shortened input against the full-length input of `a_sampled_scalar_encodes_through_secret`; edge the error crosses the consumer and the factory whole
      * `[✅]`   `libraries_sharing_an_identifier_encode_the_pairing_product_to_the_same_bytes`: condition for every concrete, the consumer, written against `IPairingArithmetic`, samples a scalar from the uniform input of value `7`, multiplies the first-group generator by it with `mul_g1`, takes `pairing_product` over the terms of that point with the second-group generator and of the first-group generator with the second-group generator, and encodes the product with `encode_gt`; outcome the output pairs `P::DECLARATION.target_group_encoding` with the encoded product read through `Secret::expose`, and any two concretes whose identifiers are equal return equal bytes; variation a nonzero scalar and two terms, so the product is neither the target group's identity nor a single generator pairing; edge each library's correction of its reduced pairing to the identifier's exact value reaches the same serialized bytes
      * `[✅]`   `libraries_sharing_an_identifier_return_the_same_scalar_field_order`: condition for every concrete, the consumer, written against `IPairingReference`, reads `scalar_field_order`; outcome the output pairs the identifier with the order's bytes, and any two concretes whose identifiers are equal return equal bytes; variation every concrete of the declared set, which holds each library of each identifier; edge the order is read through the family's surface alone
      * `[✅]`   `libraries_sharing_an_identifier_return_the_same_first_group_reference_encoding`: condition for every concrete, the consumer, written against `IPairingReference`, reads `g1_outside_subgroup_encoding`, propagating its error unchanged; outcome the output pairs the identifier with the optional bytes, the bytes absent for `TargetGroupEncodingIdentifier::Bn254V1`, and present and equal across any two concretes whose identifiers are equal for `TargetGroupEncodingIdentifier::Bls12381V1`; variation both identifiers, so absence and presence are both exercised; edge the first group of BN254 has no point outside the subgroup, and the family returns no encoding for it
      * `[✅]`   `libraries_sharing_an_identifier_return_the_same_second_group_reference_encoding`: condition for every concrete, the consumer, written against `IPairingReference`, reads `g2_outside_subgroup_encoding`, propagating its error unchanged; outcome the output pairs the identifier with the bytes, and any two concretes whose identifiers are equal return equal bytes; variation every concrete of the declared set; edge the encoding is the ascending search's least point outside the subgroup, so two libraries agree only if they search in the same order and encode alike
      * `[✅]`   `each_second_group_decoder_refuses_the_second_group_reference_encoding`: condition for every concrete, the consumer, written against `IPairingReference`, reads `g2_outside_subgroup_encoding` and passes its bytes to the same concrete's `decode_g2`; outcome the output nests the reference's result around the decoder's result, each error carried unchanged, and equals `Ok(Err(DecodeG2ErrorReturn::NotInSubgroup))`; variation every concrete of the declared set; edge the decoder refuses a point on the curve and outside the subgroup, which its length, canonicality, and curve checks all pass
      * `[✅]`   `each_first_group_decoder_refuses_the_first_group_reference_encoding_where_one_exists`: condition for every concrete, the consumer, written against `IPairingReference`, reads `g1_outside_subgroup_encoding` and, when it holds bytes, passes them to the same concrete's `decode_g1`; outcome the output nests the optional decoder result inside the reference's result, each error carried unchanged, and whenever the decoder ran its result is `Err(DecodeG1ErrorReturn::NotInSubgroup)`; variation both identifiers, so a concrete with no first-group encoding and a concrete with one are both run; edge the decoder refuses a point on the curve and outside the subgroup, which its length, canonicality, and curve checks all pass
      * `[✅]`   `the_sample_scalar_wrong_length_mode_returns_the_sampling_refusal_whole`: condition the consumer samples `P::Scalar::UNIFORM_BYTES_LENGTH` uniform bytes on the concrete `PairingConcrete::Mock(MockIPairingAdapterFailureMode::SampleScalarWrongLength)` selects; outcome `Ok` whose output is `Err(SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual: 64 })`, the family's flavor the mode manufactures because no input reaches the arm; variation the correctly-sized input under the mode, a refusal no real concrete's correctly-sized input produces; edge the failure mode the selection carries reaches `sample_from_uniform_bytes` through `MOCK_SAMPLING_MODE`, and the refusal arrives at the caller whole
      * `[✅]`   `the_decode_g1_not_on_curve_mode_returns_the_curve_refusal_whole`: condition the consumer calls `decode_g1` on the eight-byte first-group encoding of residue `1`, on the curve and in the subgroup on the mock concrete, under `PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG1NotOnCurve)`; outcome `Ok` whose output is `Err(DecodeG1ErrorReturn::NotOnCurve)`; variation the mode against a decodable input, the arm no input reaches on the mock concrete; edge the refusal arrives at the caller whole in the family's flavor
      * `[✅]`   `the_decode_g1_not_in_subgroup_mode_returns_the_subgroup_refusal_whole`: condition the consumer calls `decode_g1` on the same first-group encoding under `PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG1NotInSubgroup)`; outcome `Ok` whose output is `Err(DecodeG1ErrorReturn::NotInSubgroup)`; variation and edge as the entry above
      * `[✅]`   `the_decode_g2_not_on_curve_mode_returns_the_curve_refusal_whole`: condition the consumer calls `decode_g2` on the eight-byte second-group encoding of residue `2`, even and so inside the mock second group's subgroup, under `PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG2NotOnCurve)`; outcome `Ok` whose output is `Err(DecodeG2ErrorReturn::NotOnCurve)`; variation the mode against a decodable input, an odd residue taking the input-reached `NotInSubgroup` arm instead, so the refusal observed is the mode's; edge the refusal arrives at the caller whole in the family's flavor
      * `[✅]`   `the_g1_outside_subgroup_search_exhausted_mode_returns_the_search_refusal_whole`: condition the consumer calls `g1_outside_subgroup_encoding` under `PairingConcrete::Mock(MockIPairingAdapterFailureMode::G1OutsideSubgroupSearchExhausted)`; outcome `Ok` whose output is `Err(G1OutsideSubgroupEncodingErrorReturn::SearchExhausted)`; variation the mode, the arm no input reaches on the mock concrete, whose first group is the whole curve; edge the refusal arrives at the caller whole in the family's flavor
      * `[✅]`   `the_g2_outside_subgroup_search_exhausted_mode_returns_the_search_refusal_whole`: condition the consumer calls `g2_outside_subgroup_encoding` under `PairingConcrete::Mock(MockIPairingAdapterFailureMode::G2OutsideSubgroupSearchExhausted)`; outcome `Ok` whose output is `Err(G2OutsideSubgroupEncodingErrorReturn::SearchExhausted)`; variation and edge as the entry above
      * `[✅]`   `a_concrete_whose_provisions_the_requirements_do_not_match_is_refused_whole_before_construction`: condition `params.concrete` is a variant of `PAIRING_CONCRETES`, or the mock selection `PairingConcrete::Mock(MockIPairingAdapterFailureMode::Succeeds)`, and the four `required_*` values are every provision the declaration the real `lookup_pairing_declaration` returns for a member of a different curve states, that member read from `PAIRING_CONCRETES` by comparing declared curves; outcome `Err(CreatePairingErrorReturn::Composition(AdmitPairingCompositionErrorReturn::Incompatible { identifier, disagreements }))` naming `params.concrete`, whose `disagreements` holds the curve, verifier-group arithmetic, precompile encoding, and target-group encoding disagreement in that order, each carrying the concrete's declared value and the other member's value, with nothing constructed and the consumer's output absent; variation every variant in turn and the mock concrete, so a factory that compares fewer provisions, names another member, or constructs before it admits fails; edge the refusal crosses the factory whole
      * `[✅]`   `a_single_disagreeing_provision_reaches_the_caller_whole`: condition `params.concrete` is a variant of `PAIRING_CONCRETES`, or the mock selection, and the four `required_*` values are the provisions its own declaration states except the target-group encoding, replaced by a value of `TARGET_GROUP_ENCODING_IDENTIFIERS` its declaration does not state; outcome `Err(CreatePairingErrorReturn::Composition(AdmitPairingCompositionErrorReturn::Incompatible { identifier, disagreements }))` whose `disagreements` holds the one `PairingDisagreement::TargetGroupEncoding` carrying the concrete's declared value and the replaced value; variation every variant in turn and the mock concrete; edge a provision that agrees is not named
    * `[✅]`   `Integration: callee dispositions`: `pairing/bn254_arkworks`, `pairing/bn254_halo2curves`, `pairing/bls12_381_arkworks`, and `pairing/bls12_381_halo2curves` state no integration entry, the agreement of two libraries of one curve being this node's own entries, so no concrete entry passes through `create_pairing` or ends at one of its tests; `pairing/lookup_pairing_declaration` is called by the admission and by no function of this node, so none of its entries passes through `create_pairing`; `pairing/admit_pairing_composition`'s entries `admission_compares_the_provisions_of_the_member_named`, `mock_admission_compares_the_mock_declaration`, and `admission_names_every_disagreeing_provision_in_order_for_each_declared_member` pass through `create_pairing`, which calls the admission and returns its error whole, and are absorbed by the private block `a_concrete_whose_provisions_the_requirements_do_not_match_is_refused_whole_before_construction`; its entry `admission_names_each_disagreeing_provision_for_each_declared_member` passes through `create_pairing` likewise and is absorbed by the private block `a_single_disagreeing_provision_reaches_the_caller_whole`
    * `[✅]`   `Integration: private surface`: the chain of real functions is `create_pairing`, the real `admit_pairing_composition`, the real `lookup_pairing_declaration`, the declaration constant of the concrete named, the arm's `try_new`, and the consumer's calls on the concrete through the family's traits, with `Secret::expose`, the `Secret` drop, and `AsRef<[u8]>` running for real; the cases are each variant of `PAIRING_CONCRETES` and `PairingConcrete::Mock(mode)`, `mode` `Succeeds` except where a failure-mode entry selects its own; no outer-edge collaborator is mocked, `domain` running as itself; the observable result is the `Result` `create_pairing` returns, `P::CONCRETE` read through the trait, each method's success and error values, and the encoded bytes; under the `libraries_sharing_*` entries the mock concrete contributes a `Mock`-identifier output that pairs with no other, and under `each_first_group_decoder_refuses_the_first_group_reference_encoding_where_one_exists` a `None`, so that entry's `Some(Err)` witness comes from the declared set; the entries proven are every own entry above
    * `[✅]`   `Integration: public surface`: an outside caller invokes `create_pairing` from the crate's public surface with `CreatePairingDeps` holding an `IPairingConsumer` it writes, the public `admit_pairing_composition`, and `AdmitPairingCompositionDeps` holding the public `lookup_pairing_declaration`, the `CreatePairingParams` configuration supplies, and `CreatePairingPayload`, for each concrete in `PAIRING_CONCRETES`, and observes the consumer's output in `CreatePairingSuccessReturn` or the refusal in `CreatePairingErrorReturn::Composition`; the entries proven are the own entries above that run over the declared set, the failure-mode entries and the entries naming the mock concrete excluded, no input reaching their arms on a real concrete; the route of each is `create_pairing`, the real `admit_pairing_composition` and `lookup_pairing_declaration`, the arm's concrete through its `try_new`, and the consumer's calls on that concrete through the family's traits; the outer-edge collaborators on that route are none, and the fixtures are `build_secret`, `build_create_pairing_params`, and `build_create_pairing_deps`

  * `[✅]`   `adapters/pairing/src/factory/mock.rs`
    * `[✅]`   Imports from `super::interface` the names the items below add; `AdmitPairingCompositionDeps` and `AdmitPairingCompositionFn` from `crate::admit_pairing_composition::provides`; and `lookup_pairing_declaration`, `LookupPairingDeclarationDeps`, `LookupPairingDeclarationParams`, and `LookupPairingDeclarationPayload` from `crate::lookup_pairing_declaration::provides`
    * `[✅]`   `CreatePairingParamsOverrides`, `#[derive(Default)]`, with `pub concrete: Option<PairingConcrete>`, `pub required_curve: Option<PairingCurve>`, `pub required_verifier_group_arithmetic: Option<VerifierGroupArithmetic>`, `pub required_precompile_encodings: Option<Vec<PrecompileEncoding>>`, and `pub required_target_group_encoding: Option<TargetGroupEncodingIdentifier>`; `build_create_pairing_params(overrides: CreatePairingParamsOverrides) -> CreatePairingParams` binding `let concrete = overrides.concrete.unwrap_or(PairingConcrete::Mock(MockIPairingAdapterFailureMode::Succeeds));`, then `let Ok(looked_up) = lookup_pairing_declaration(&LookupPairingDeclarationDeps, LookupPairingDeclarationParams { concrete }, LookupPairingDeclarationPayload);`, the declaration the real family lookup returns for the concrete, and defaulting each required value to the matching field of `looked_up.declaration`, `required_precompile_encodings` to a vector holding its `precompile_encoding`, so an arrangement naming nothing selects the mock concrete, an arrangement naming a concrete alone is admitted by the real composition admission, and only an arrangement proving refusal overrides a requirement; no `match` on the concrete and no member's declared value is written
    * `[✅]`   `CreatePairingSuccessReturnOverrides`, the fieldless unit struct with `#[derive(Default)]`, since the output is handed; `build_create_pairing_success_return<O>(output: O, overrides: CreatePairingSuccessReturnOverrides) -> CreatePairingSuccessReturn<O>`, the output its leading parameter
    * `[✅]`   `ConsumePairingPayloadOverrides`, the fieldless unit struct with `#[derive(Default)]`, since the adapter is handed; `build_consume_pairing_payload<P: IPairingAdapter>(adapter: P, overrides: ConsumePairingPayloadOverrides) -> ConsumePairingPayload<P>`, the adapter its leading parameter
    * `[✅]`   `MockIPairingConsumer`, the unit struct `pub struct MockIPairingConsumer;`, implementing `IPairingConsumer` with `type Output = ();` and `consume_pairing<P: IPairingAdapter>` returning `()`
    * `[✅]`   `CreatePairingDepsOverrides`, the fieldless unit struct with `#[derive(Default)]`, since the consumer and the composition admission's handle and deps are handed; `build_create_pairing_deps<C: IPairingConsumer>(consumer: C, admit_pairing_composition: AdmitPairingCompositionFn, admit_pairing_composition_deps: AdmitPairingCompositionDeps, overrides: CreatePairingDepsOverrides) -> CreatePairingDeps<C>`, the three handed fields its leading parameters in field order, so a unit test places `MockIPairingConsumer`, `mock_admit_pairing_composition`, and a `build_admit_pairing_composition_deps` result in the deps by passing them
    * `[✅]`   `mock_create_pairing<C: IPairingConsumer>(deps: &CreatePairingDeps<C>, _params: CreatePairingParams, _payload: CreatePairingPayload) -> CreatePairingReturn<C::Output>`, the production type `CreatePairingFn<C>` at every consumer, standing in for the factory that constructs a concrete: it builds the family's mock concrete with `build_mock_i_pairing_adapter(Default::default())`, calls `deps.consumer.consume_pairing(ConsumePairingParams, build_consume_pairing_payload(adapter, Default::default()))` exactly once, and returns `Ok(build_create_pairing_success_return(output, Default::default()))` around the consumer's output
    * `[✅]`   No builder for the fieldless `ConsumePairingParams` and `CreatePairingPayload`, used by their production values, or for the enums and the error return, used by their production values

  * `[✅]`   `adapters/pairing/src/factory/test.rs`
    * `[✅]`   Imports `create_pairing` from `super`, and `CreatePairingErrorReturn`, `CreatePairingParamsOverrides`, `CreatePairingPayload`, `MockIPairingAdapterFailureMode`, `MockIPairingConsumer`, `PairingConcrete`, `PairingCurve`, `PrecompileEncoding`, `TargetGroupEncodingIdentifier`, `VerifierGroupArithmetic`, `build_create_pairing_deps`, and `build_create_pairing_params` from `super::provides`; `AdmitPairingCompositionErrorReturn`, `AdmitPairingCompositionFn`, `PairingDisagreement`, `build_admit_pairing_composition_deps`, and `mock_admit_pairing_composition` from `crate::admit_pairing_composition::provides`; and `mock_lookup_pairing_declaration` from `crate::lookup_pairing_declaration::provides`
    * `[✅]`   Every block: the function under test is `create_pairing`; the consumer is `MockIPairingConsumer`, the family's official consumer mock, placed in the deps by `build_create_pairing_deps(MockIPairingConsumer, mock_admit_pairing_composition, build_admit_pairing_composition_deps(mock_lookup_pairing_declaration, Default::default()), Default::default())` except where the block declares its own composition admission; the arm's concrete `try_new` runs for real, since the factory constructs it itself; the payload is the production value `CreatePairingPayload`; params come from `build_create_pairing_params` with only the fields the block varies overridden
    * `[✅]`   `create_pairing_admits_the_<concrete>_concrete`, one block per `PairingConcrete` variant, `<concrete>` the variant in snake case: `bn254_arkworks` for `Bn254Arkworks`, `bn254_halo2curves` for `Bn254Halo2curves`, `bls12_381_arkworks` for `Bls12381Arkworks`, `bls12_381_halo2curves` for `Bls12381Halo2curves`, and `mock` for `Mock(MockIPairingAdapterFailureMode::Succeeds)`
      * `[✅]`   Contract: given `params.concrete` is the variant and the composition admission returns `Ok`, the factory returns the success arm
      * `[✅]`   Arrange: `build_create_pairing_params` with only `concrete` overridden to the variant; the deps of the block's lead line, whose admission is the official function mock returning `Ok`
      * `[✅]`   Act: `create_pairing(&deps, params, CreatePairingPayload)`
      * `[✅]`   Assert: `result.is_ok()`
    * `[✅]`   `create_pairing_returns_the_composition_refusal_whole_for_the_<concrete>_concrete`, one block per variant, `<concrete>` as above
      * `[✅]`   Contract: given the composition admission refuses, the factory returns `Err(CreatePairingErrorReturn::Composition(error))` carrying the admission's error whole, the admission having been called with `params.concrete` as the identifier and the four `required_*` values `params` carries
      * `[✅]`   Arrange: a composition admission the block declares, typed `AdmitPairingCompositionFn`, returning `Err(AdmitPairingCompositionErrorReturn::Incompatible { identifier: params.identifier, disagreements })` whose `disagreements` holds `PairingDisagreement::Curve { declared: PairingCurve::Bn254, required: params.required_curve }`, `PairingDisagreement::VerifierGroupArithmetic { declared: VerifierGroupArithmetic::FirstGroupOnly, required: params.required_verifier_group_arithmetic }`, `PairingDisagreement::PrecompileEncoding { declared: PrecompileEncoding::Eip196Eip197, required: params.required_precompile_encodings }`, and `PairingDisagreement::TargetGroupEncoding { declared: TargetGroupEncodingIdentifier::Bn254V1, required: params.required_target_group_encoding }`, so the error restates what the factory passed; `build_create_pairing_params` with `concrete` overridden to the variant and the four `required_*` overridden to the values of the curve the variant is not of, `PairingCurve::Bls12381`, `VerifierGroupArithmetic::BothGroups`, `vec![PrecompileEncoding::Eip2537]`, and `TargetGroupEncodingIdentifier::Bls12381V1` for the BN254 variants and the mock variant, and `PairingCurve::Bn254`, `VerifierGroupArithmetic::FirstGroupOnly`, `vec![PrecompileEncoding::Eip196Eip197]`, and `TargetGroupEncodingIdentifier::Bn254V1` for the BLS12-381 variants, each differing from the builder's default for the variant, so a factory that passes defaults or another concrete's identifier fails; deps built with that admission and `build_admit_pairing_composition_deps(mock_lookup_pairing_declaration, Default::default())`
      * `[✅]`   Act: `create_pairing(&deps, params, CreatePairingPayload)`
      * `[✅]`   Assert: `assert_eq!(result.err(), Some(CreatePairingErrorReturn::Composition(AdmitPairingCompositionErrorReturn::Incompatible { identifier: <the variant>, disagreements: <the four disagreements with the override values written as literals> })))`, the interaction spec's outcome as a literal

  * `[✅]`   `construction`
    * `[✅]`   The composition root writes its pairing-dependent work once as an `IPairingConsumer`, generic over `P: IPairingArithmetic + IPairingReference`, and calls `create_pairing` with `CreatePairingDeps` holding that consumer, `admit_pairing_composition`, and the `AdmitPairingCompositionDeps` holding `lookup_pairing_declaration`, `CreatePairingParams` holding the configured `PairingConcrete` and the four requirements, the precompile encodings among them read through `chain/lookup_chain_declaration` from the configured chain member's declaration, and `CreatePairingPayload`; no consumer constructs or names a concrete

  * `[✅]`   `adapters/pairing/src/factory/mod.rs`
    * `[✅]`   The module wiring holds `#[cfg(test)] mod test;` and `#[cfg(test)] mod integration_test;`; imports each concrete's adapter type and constructor params from `crate::bn254_arkworks::provides`, `crate::bn254_halo2curves::provides`, `crate::bls12_381_arkworks::provides`, and `crate::bls12_381_halo2curves::provides`, `AdmitPairingCompositionParams` and `AdmitPairingCompositionPayload` from `crate::admit_pairing_composition::provides`, under `#[cfg(any(test, feature = "mocks"))]` `MockIPairingAdapter` and `MockIPairingAdapterConstructorParams` from `self::mock`, and from `interface` `ConsumePairingParams`, `ConsumePairingPayload`, `CreatePairingDeps`, `CreatePairingErrorReturn`, `CreatePairingParams`, `CreatePairingPayload`, `CreatePairingReturn`, `CreatePairingSuccessReturn`, `IPairingConsumer`, and `PairingConcrete`
    * `[✅]`   `pub fn create_pairing<C: IPairingConsumer>(deps: &CreatePairingDeps<C>, params: CreatePairingParams, _payload: CreatePairingPayload) -> CreatePairingReturn<C::Output>`, realizing the interaction spec's branches: one call `(deps.admit_pairing_composition)(&deps.admit_pairing_composition_deps, AdmitPairingCompositionParams { identifier: params.concrete, required_curve: params.required_curve, required_verifier_group_arithmetic: params.required_verifier_group_arithmetic, required_precompile_encodings: params.required_precompile_encodings, required_target_group_encoding: params.required_target_group_encoding }, AdmitPairingCompositionPayload)` made before the `match`, whose `Err(error)` returns `Err(CreatePairingErrorReturn::Composition(error))`; then a `match` on `params.concrete`, one arm per variant, each binding the concrete by `let Ok(adapter) = <Concrete>::try_new(<Concrete>ConstructorParams);` and returning `Ok(CreatePairingSuccessReturn { output: deps.consumer.consume_pairing(ConsumePairingParams, ConsumePairingPayload { adapter }) })`; each arm's variant is the `CONCRETE` of the concrete it constructs, and, under `#[cfg(any(test, feature = "mocks"))]`, a `PairingConcrete::Mock(mode)` arm that binds the mock concrete by `let Ok(adapter) = MockIPairingAdapter::try_new(MockIPairingAdapterConstructorParams { failure_mode: mode });`; no arm reads a declaration or compares a requirement
    * `[✅]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[✅]`   `adapters/pairing/src/factory/integration_test.rs`
    * `[✅]`   Module-level `#![allow(clippy::expect_used)]`; imports `create_pairing` from `super`, `AdmitPairingCompositionErrorReturn`, `PairingDisagreement`, `admit_pairing_composition`, and `build_admit_pairing_composition_deps` from `crate::admit_pairing_composition::provides`, `lookup_pairing_declaration`, `LookupPairingDeclarationDeps`, `LookupPairingDeclarationParams`, and `LookupPairingDeclarationPayload` from `crate::lookup_pairing_declaration::provides`, `PairingCurve`, `PrecompileEncoding`, and `VerifierGroupArithmetic` from `super::provides`, and from `super::provides` `PAIRING_CONCRETES`, `TARGET_GROUP_ENCODING_IDENTIFIERS`, `PairingConcrete`, `MockIPairingAdapterFailureMode`, `CreatePairingParamsOverrides`, `CreatePairingPayload`, `ConsumePairingParams`, `ConsumePairingPayload`, `IPairingConsumer`, `IPairingAdapter`, `IPairingArithmetic`, `IPairingReference`, `ISampleUniformScalar`, `SampleUniformScalarErrorReturn`, `TargetGroupEncodingIdentifier`, `build_create_pairing_deps`, and `build_create_pairing_params`, plus the params, payload, and error types of each method a consumer calls; from `domain` `Secret` and `build_secret`
    * `[✅]`   Every block: its body is written once and runs over `PAIRING_CONCRETES` and, except a failure-mode block, `PairingConcrete::Mock(MockIPairingAdapterFailureMode::Succeeds)`, naming no concrete; per case, params are `build_create_pairing_params(CreatePairingParamsOverrides { concrete: Some(concrete), ..Default::default() })`, unless the block states its requirements, and deps are `build_create_pairing_deps(Consumer, admit_pairing_composition, build_admit_pairing_composition_deps(lookup_pairing_declaration, Default::default()), Default::default())`, the real admission over the real lookup, `Consumer` being a unit struct implementing `IPairingConsumer`, declared inside the block with the `Output` type and the narrowest bound its entry states; the call is `create_pairing(&deps, params, CreatePairingPayload)`, whose result is observed as `result.ok().map(|success| success.output)`; a failure-mode block runs the single selection `PairingConcrete::Mock(mode)` of its mode; nothing is mocked
    * `[✅]`   `the_consumer_receives_the_concrete_the_params_name`, `a_sampled_scalar_encodes_through_secret`, `a_sampling_refusal_reaches_the_caller_unchanged`, `each_second_group_decoder_refuses_the_second_group_reference_encoding`, and `each_first_group_decoder_refuses_the_first_group_reference_encoding_where_one_exists`: each the block the public element `tests/factory_integration_test.rs` states for the entry of the same name, run per case over the declared set and the mock concrete, the mock case taking the entry's outcome as every concrete's — `Mock(Succeeds)` for the first, the encoded `7` for the second, `WrongLength { expected: 64, actual: 63 }` for the third, `Err(DecodeG2ErrorReturn::NotInSubgroup)` for the fourth, and `Ok(None)` for the fifth
    * `[✅]`   `libraries_sharing_an_identifier_encode_the_pairing_product_to_the_same_bytes`, `libraries_sharing_an_identifier_return_the_same_scalar_field_order`, `libraries_sharing_an_identifier_return_the_same_first_group_reference_encoding`, and `libraries_sharing_an_identifier_return_the_same_second_group_reference_encoding`: each the block the public element `tests/factory_integration_test.rs` states for the entry of the same name, run per case over the declared set and the mock concrete, so `outputs.len()` equals `PAIRING_CONCRETES.len() + 1` and the mock concrete's `Mock`-identifier output pairs with no other
    * `[✅]`   `a_concrete_whose_provisions_the_requirements_do_not_match_is_refused_whole_before_construction`
      * `[✅]`   Contract: entry `a_concrete_whose_provisions_the_requirements_do_not_match_is_refused_whole_before_construction`; given the four `required_*` values are every provision the declaration the real lookup returns for a member of a different curve states, `create_pairing` returns `Err(CreatePairingErrorReturn::Composition(AdmitPairingCompositionErrorReturn::Incompatible { identifier, disagreements }))` naming `params.concrete`, whose `disagreements` holds the curve, verifier-group arithmetic, precompile encoding, and target-group encoding disagreement in that order, each carrying the concrete's declared value and the other member's value
      * `[✅]`   Arrange: `Consumer` with `type Output = ();` and `consume_pairing<P: IPairingAdapter>` returning `()`; per case, the other-curve member chosen from `PAIRING_CONCRETES` as the first member whose declaration, read through `lookup_pairing_declaration`, states a curve unequal to the case's declared curve, the requirements built from that member's declaration, a `Vec` holding its precompile encoding for the encodings; every concrete of the declared set and the mock concrete in turn
      * `[✅]`   Act: `create_pairing(&deps, params, CreatePairingPayload)` per case
      * `[✅]`   Assert: per case, `assert_eq!(result.err(), Some(CreatePairingErrorReturn::Composition(AdmitPairingCompositionErrorReturn::Incompatible { identifier, disagreements })))` with `identifier` the case's concrete and `disagreements` the four disagreements in order, each `declared` read from the case's declaration and each `required` from the other member's declaration through `lookup_pairing_declaration`
      * `[✅]`   Boundary: `create_pairing`, the real `admit_pairing_composition`, the real `lookup_pairing_declaration`, and the declaration constants they read
      * `[✅]`   Mocked: none
    * `[✅]`   `a_single_disagreeing_provision_reaches_the_caller_whole`
      * `[✅]`   Contract: entry `a_single_disagreeing_provision_reaches_the_caller_whole`; given the four `required_*` values are the provisions the concrete's own declaration states except the target-group encoding, replaced by a value of `TARGET_GROUP_ENCODING_IDENTIFIERS` its declaration does not state, `create_pairing` returns `Err(CreatePairingErrorReturn::Composition(AdmitPairingCompositionErrorReturn::Incompatible { identifier, disagreements }))` whose `disagreements` holds the one `PairingDisagreement::TargetGroupEncoding` carrying the declared value and the replaced value
      * `[✅]`   Arrange: the `Consumer` of the block above; per case, the requirements read from the case's declaration through `lookup_pairing_declaration`, the target-group encoding replaced by the first value of `TARGET_GROUP_ENCODING_IDENTIFIERS` unequal to the declared one; every concrete of the declared set and the mock concrete in turn
      * `[✅]`   Act: `create_pairing(&deps, params, CreatePairingPayload)` per case
      * `[✅]`   Assert: per case, `assert_eq!(result.err(), Some(CreatePairingErrorReturn::Composition(AdmitPairingCompositionErrorReturn::Incompatible { identifier, disagreements: vec![PairingDisagreement::TargetGroupEncoding { declared, required }] })))` with `declared` read from the case's declaration and `required` the replaced value
      * `[✅]`   Boundary: the boundary of the block above
      * `[✅]`   Mocked: none
    * `[✅]`   `the_sample_scalar_wrong_length_mode_returns_the_sampling_refusal_whole`
      * `[✅]`   Contract: entry `the_sample_scalar_wrong_length_mode_returns_the_sampling_refusal_whole`; given the mock selection under `SampleScalarWrongLength`, the consumer's sampling of a correctly-sized uniform input is refused `Err(SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual: 64 })`, whole
      * `[✅]`   Arrange: `Consumer` with `type Output = Result<(), SampleUniformScalarErrorReturn>;` and `consume_pairing<P: IPairingAdapter>` calling `P::Scalar::sample_from_uniform_bytes(SampleUniformScalarParams, SampleUniformScalarPayload { uniform })` over `uniform` from `build_secret(vec![0u8; P::Scalar::UNIFORM_BYTES_LENGTH], Default::default())` and returning its `Err`, the success erased; the selection `PairingConcrete::Mock(MockIPairingAdapterFailureMode::SampleScalarWrongLength)` alone
      * `[✅]`   Act: `create_pairing(&deps, params, CreatePairingPayload)`
      * `[✅]`   Assert: `assert_eq!(result.ok().map(|success| success.output), Some(Err(SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual: 64 })))`, the mode's manufactured refusal as a literal
      * `[✅]`   Boundary: `create_pairing`, the mock arm's `try_new`, `consume_pairing`, and `sample_from_uniform_bytes`, the mode reaching the sampler through `MOCK_SAMPLING_MODE`
      * `[✅]`   Mocked: none; `build_secret` builds the input
    * `[✅]`   `the_decode_g1_not_on_curve_mode_returns_the_curve_refusal_whole`
      * `[✅]`   Contract: entry `the_decode_g1_not_on_curve_mode_returns_the_curve_refusal_whole`; given the mock selection under `DecodeG1NotOnCurve`, `decode_g1` of a decodable encoding is refused `Err(DecodeG1ErrorReturn::NotOnCurve)`, whole
      * `[✅]`   Arrange: `Consumer` with `type Output = Result<(), DecodeG1ErrorReturn>;` and `consume_pairing<P: IPairingAdapter>` calling `adapter.decode_g1(DecodeG1Params, &[0, 0, 0, 0, 0, 0, 0, 1])`, the eight-byte encoding of residue `1`, and returning its `Err`, the success erased; the selection `PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG1NotOnCurve)` alone
      * `[✅]`   Act: `create_pairing(&deps, params, CreatePairingPayload)`
      * `[✅]`   Assert: `assert_eq!(result.ok().map(|success| success.output), Some(Err(DecodeG1ErrorReturn::NotOnCurve)))`, the mode's manufactured refusal as a literal
      * `[✅]`   Boundary: `create_pairing`, the mock arm's `try_new`, `consume_pairing`, and `decode_g1`
      * `[✅]`   Mocked: none
    * `[✅]`   `the_decode_g1_not_in_subgroup_mode_returns_the_subgroup_refusal_whole`
      * `[✅]`   Contract: entry `the_decode_g1_not_in_subgroup_mode_returns_the_subgroup_refusal_whole`; given the mock selection under `DecodeG1NotInSubgroup`, `decode_g1` of the same encoding is refused `Err(DecodeG1ErrorReturn::NotInSubgroup)`, whole
      * `[✅]`   Arrange: the `Consumer` of the block above under the selection `PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG1NotInSubgroup)` alone
      * `[✅]`   Act: `create_pairing(&deps, params, CreatePairingPayload)`
      * `[✅]`   Assert: `assert_eq!(result.ok().map(|success| success.output), Some(Err(DecodeG1ErrorReturn::NotInSubgroup)))`, the mode's manufactured refusal as a literal
      * `[✅]`   Boundary: `create_pairing`, the mock arm's `try_new`, `consume_pairing`, and `decode_g1`
      * `[✅]`   Mocked: none
    * `[✅]`   `the_decode_g2_not_on_curve_mode_returns_the_curve_refusal_whole`
      * `[✅]`   Contract: entry `the_decode_g2_not_on_curve_mode_returns_the_curve_refusal_whole`; given the mock selection under `DecodeG2NotOnCurve`, `decode_g2` of a decodable encoding is refused `Err(DecodeG2ErrorReturn::NotOnCurve)`, whole
      * `[✅]`   Arrange: `Consumer` with `type Output = Result<(), DecodeG2ErrorReturn>;` and `consume_pairing<P: IPairingAdapter>` calling `adapter.decode_g2(DecodeG2Params, &[0, 0, 0, 0, 0, 0, 0, 2])`, the eight-byte encoding of residue `2`, even and so inside the mock second group's subgroup, and returning its `Err`, the success erased; the selection `PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG2NotOnCurve)` alone
      * `[✅]`   Act: `create_pairing(&deps, params, CreatePairingPayload)`
      * `[✅]`   Assert: `assert_eq!(result.ok().map(|success| success.output), Some(Err(DecodeG2ErrorReturn::NotOnCurve)))`, the mode's manufactured refusal as a literal
      * `[✅]`   Boundary: `create_pairing`, the mock arm's `try_new`, `consume_pairing`, and `decode_g2`
      * `[✅]`   Mocked: none
    * `[✅]`   `the_g1_outside_subgroup_search_exhausted_mode_returns_the_search_refusal_whole`
      * `[✅]`   Contract: entry `the_g1_outside_subgroup_search_exhausted_mode_returns_the_search_refusal_whole`; given the mock selection under `G1OutsideSubgroupSearchExhausted`, `g1_outside_subgroup_encoding` is refused `Err(G1OutsideSubgroupEncodingErrorReturn::SearchExhausted)`, whole
      * `[✅]`   Arrange: `Consumer` with `type Output = Result<(), G1OutsideSubgroupEncodingErrorReturn>;` and `consume_pairing<P: IPairingReference>` calling `adapter.g1_outside_subgroup_encoding(G1OutsideSubgroupEncodingParams, G1OutsideSubgroupEncodingPayload)` and returning its `Err`, the success erased; the selection `PairingConcrete::Mock(MockIPairingAdapterFailureMode::G1OutsideSubgroupSearchExhausted)` alone
      * `[✅]`   Act: `create_pairing(&deps, params, CreatePairingPayload)`
      * `[✅]`   Assert: `assert_eq!(result.ok().map(|success| success.output), Some(Err(G1OutsideSubgroupEncodingErrorReturn::SearchExhausted)))`, the mode's manufactured refusal as a literal
      * `[✅]`   Boundary: `create_pairing`, the mock arm's `try_new`, `consume_pairing`, and `g1_outside_subgroup_encoding`
      * `[✅]`   Mocked: none
    * `[✅]`   `the_g2_outside_subgroup_search_exhausted_mode_returns_the_search_refusal_whole`
      * `[✅]`   Contract: entry `the_g2_outside_subgroup_search_exhausted_mode_returns_the_search_refusal_whole`; given the mock selection under `G2OutsideSubgroupSearchExhausted`, `g2_outside_subgroup_encoding` is refused `Err(G2OutsideSubgroupEncodingErrorReturn::SearchExhausted)`, whole
      * `[✅]`   Arrange: `Consumer` with `type Output = Result<(), G2OutsideSubgroupEncodingErrorReturn>;` and `consume_pairing<P: IPairingReference>` calling `adapter.g2_outside_subgroup_encoding(G2OutsideSubgroupEncodingParams, G2OutsideSubgroupEncodingPayload)` and returning its `Err`, the success erased; the selection `PairingConcrete::Mock(MockIPairingAdapterFailureMode::G2OutsideSubgroupSearchExhausted)` alone
      * `[✅]`   Act: `create_pairing(&deps, params, CreatePairingPayload)`
      * `[✅]`   Assert: `assert_eq!(result.ok().map(|success| success.output), Some(Err(G2OutsideSubgroupEncodingErrorReturn::SearchExhausted)))`, the mode's manufactured refusal as a literal
      * `[✅]`   Boundary: `create_pairing`, the mock arm's `try_new`, `consume_pairing`, and `g2_outside_subgroup_encoding`
      * `[✅]`   Mocked: none

  * `[✅]`   `adapters/pairing/src/factory/provides.rs`
    * `[✅]`   The line `pub use super::create_pairing;`, beside the interface and mock re-exports, the crate's public surface, which the barrel re-exports and nothing beneath it reaches

  * `[✅]`   `adapters/pairing/tests/factory_integration_test.rs`
    * `[✅]`   Outside the crate, so it reaches only the crate's public surface and the official mocks under the `mocks` feature; imports from `pairing` `create_pairing`, `admit_pairing_composition`, `lookup_pairing_declaration`, `build_admit_pairing_composition_deps`, `AdmitPairingCompositionErrorReturn`, `CreatePairingErrorReturn`, `PairingDisagreement`, `TARGET_GROUP_ENCODING_IDENTIFIERS`, `PAIRING_CONCRETES`, `build_create_pairing_deps`, `CreatePairingParamsOverrides`, `build_create_pairing_params`, `CreatePairingPayload`, `IPairingConsumer`, `ConsumePairingParams`, `ConsumePairingPayload`, `IPairingAdapter`, `IPairingArithmetic`, `IPairingReference`, `ISampleUniformScalar`, `TargetGroupEncodingIdentifier`, and the params, payload, and error types of each adapter method a block's consumer calls; and from `domain` `Secret` and `build_secret`
    * `[✅]`   Every block: its body is written once and runs over `PAIRING_CONCRETES`, naming no concrete; per concrete, params are `build_create_pairing_params(CreatePairingParamsOverrides { concrete: Some(concrete), ..Default::default() })`, unless the block states its requirements, and deps are `build_create_pairing_deps(Consumer, admit_pairing_composition, build_admit_pairing_composition_deps(lookup_pairing_declaration, Default::default()), Default::default())`, the public admission over the public lookup, `Consumer` being a unit struct implementing `IPairingConsumer`, declared inside the block with the `Output` type and the narrowest bound its entry states; the call is `create_pairing(&deps, params, CreatePairingPayload)`, whose result is observed as `result.ok().map(|success| success.output)`; nothing is mocked
    * `[✅]`   `the_consumer_receives_the_concrete_the_params_name`
      * `[✅]`   Contract: entry `the_consumer_receives_the_concrete_the_params_name`; given `params.concrete` is a variant of `PAIRING_CONCRETES` and the composition admission passes, the consumer's reading of `P::CONCRETE` equals `params.concrete`
      * `[✅]`   Arrange: `Consumer` with `type Output = PairingConcrete;` and `consume_pairing<P: IPairingAdapter>` returning `P::CONCRETE`; every variant of the declared set in turn, including both libraries of one curve
      * `[✅]`   Act: `create_pairing(&deps, params, CreatePairingPayload)` per concrete
      * `[✅]`   Assert: per concrete, `assert!(result.ok().map(|success| success.output) == Some(*concrete))`, the variant coming from the declared set and not from the call
      * `[✅]`   Boundary: the public `create_pairing`; the real chain is `create_pairing`, the arm's `try_new`, and `consume_pairing`
      * `[✅]`   Mocked: none
    * `[✅]`   `a_sampled_scalar_encodes_through_secret`
      * `[✅]`   Contract: entry `a_sampled_scalar_encodes_through_secret`; given a uniform `Secret` whose last byte is `7`, the encoded scalar read through `Secret::expose` is 31 zero bytes then `7`
      * `[✅]`   Arrange: `Consumer` with `type Output = Result<Vec<u8>, SampleUniformScalarErrorReturn>;` and `consume_pairing<P: IPairingAdapter>` building `build_secret(bytes, Default::default())` with `bytes` of `P::Scalar::UNIFORM_BYTES_LENGTH` zeros but for the last, `7`, calling `P::Scalar::sample_from_uniform_bytes(SampleUniformScalarParams, SampleUniformScalarPayload { uniform })` and propagating its error unchanged, calling `adapter.encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar })` with the sampled scalar's `Secret` exposed and cloned, and returning the encoded `Secret`'s bytes through `Secret::expose` and `AsRef<[u8]>`; the nonzero last byte separates a sampled input from a zero-filled input
      * `[✅]`   Act: `create_pairing(&deps, params, CreatePairingPayload)` per concrete
      * `[✅]`   Assert: per concrete, `assert_eq!(result.ok().map(|success| success.output), Some(Ok(expected)))` with `expected` the 32 bytes of 31 zeros then `7`, the integer seven in big-endian as the concretes' `sample_from_uniform_bytes` and `encode_scalar` state
      * `[✅]`   Boundary: the public `create_pairing`; the real chain is `create_pairing`, the arm's `try_new`, `consume_pairing`, `sample_from_uniform_bytes`, `encode_scalar`, and `Secret::expose`
      * `[✅]`   Mocked: none; `build_secret` builds the input
    * `[✅]`   `a_sampling_refusal_reaches_the_caller_unchanged`
      * `[✅]`   Contract: entry `a_sampling_refusal_reaches_the_caller_unchanged`; given a uniform `Secret` one byte shorter than the bound, the consumer's output is the sampling error, whole
      * `[✅]`   Arrange: the `Consumer` of `a_sampled_scalar_encodes_through_secret` with the uniform input one byte shorter than `P::Scalar::UNIFORM_BYTES_LENGTH`, the last byte `7`
      * `[✅]`   Act: `create_pairing(&deps, params, CreatePairingPayload)` per concrete
      * `[✅]`   Assert: per concrete, `assert_eq!(result.ok().map(|success| success.output), Some(Err(SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual: 63 })))`, the values the concretes' `sample_from_uniform_bytes` states
      * `[✅]`   Boundary: the public `create_pairing`; the real chain is `create_pairing`, the arm's `try_new`, `consume_pairing`, and `sample_from_uniform_bytes`
      * `[✅]`   Mocked: none; `build_secret` builds the input
    * `[✅]`   `libraries_sharing_an_identifier_encode_the_pairing_product_to_the_same_bytes`
      * `[✅]`   Contract: entry `libraries_sharing_an_identifier_encode_the_pairing_product_to_the_same_bytes`; given the same inputs, any two concretes whose declared target-group encoding identifiers are equal return equal encoded product bytes
      * `[✅]`   Arrange: `Consumer` with `type Output = Result<(TargetGroupEncodingIdentifier, Vec<u8>), SampleUniformScalarErrorReturn>;` and `consume_pairing<P: IPairingArithmetic>` sampling a scalar from the uniform input of value `7` as `a_sampled_scalar_encodes_through_secret` does, calling `mul_g1` on `g1_generator` with it, `pairing_product` over the terms of that point with `g2_generator` and of `g1_generator` with `g2_generator`, and `encode_gt` on the product, returning `P::DECLARATION.target_group_encoding` with the encoded `Secret`'s bytes; every concrete of the declared set runs, collected in order
      * `[✅]`   Act: `create_pairing(&deps, params, CreatePairingPayload)` per concrete
      * `[✅]`   Assert: the collected outputs hold an `Ok` for every concrete, `assert_eq!(outputs.len(), PAIRING_CONCRETES.len())`, and for every pair of outputs with equal identifiers the bytes are equal by `assert_eq!`
      * `[✅]`   Boundary: the public `create_pairing`; the real chain is `create_pairing`, the arm's `try_new`, `consume_pairing`, `sample_from_uniform_bytes`, `g1_generator`, `g2_generator`, `mul_g1`, `pairing_product`, `encode_gt`, and `Secret::expose`
      * `[✅]`   Mocked: none; `build_secret` builds the input
    * `[✅]`   `libraries_sharing_an_identifier_return_the_same_scalar_field_order`
      * `[✅]`   Contract: entry `libraries_sharing_an_identifier_return_the_same_scalar_field_order`; any two concretes whose identifiers are equal return equal order bytes
      * `[✅]`   Arrange: `Consumer` with `type Output = (TargetGroupEncodingIdentifier, Vec<u8>);` and `consume_pairing<P: IPairingReference>` calling `scalar_field_order` irrefutably and returning `P::DECLARATION.target_group_encoding` with the order's bytes; every concrete of the declared set runs, collected in order
      * `[✅]`   Act: `create_pairing(&deps, params, CreatePairingPayload)` per concrete
      * `[✅]`   Assert: `assert_eq!(outputs.len(), PAIRING_CONCRETES.len())` and for every pair of outputs with equal identifiers the bytes are equal by `assert_eq!`
      * `[✅]`   Boundary: the public `create_pairing`; the real chain is `create_pairing`, the arm's `try_new`, `consume_pairing`, and `scalar_field_order`
      * `[✅]`   Mocked: none
    * `[✅]`   `libraries_sharing_an_identifier_return_the_same_first_group_reference_encoding`
      * `[✅]`   Contract: entry `libraries_sharing_an_identifier_return_the_same_first_group_reference_encoding`; the encoding is absent for `Bn254V1`, and present and equal across any two concretes with equal identifiers for `Bls12381V1`
      * `[✅]`   Arrange: `Consumer` with `type Output = Result<(TargetGroupEncodingIdentifier, Option<Vec<u8>>), G1OutsideSubgroupEncodingErrorReturn>;` and `consume_pairing<P: IPairingReference>` calling `g1_outside_subgroup_encoding`, propagating its error unchanged, and returning `P::DECLARATION.target_group_encoding` with the optional bytes read through `AsRef<[u8]>`; every concrete of the declared set runs, collected in order, so both identifiers are exercised
      * `[✅]`   Act: `create_pairing(&deps, params, CreatePairingPayload)` per concrete
      * `[✅]`   Assert: `assert_eq!(outputs.len(), PAIRING_CONCRETES.len())`; every output with identifier `TargetGroupEncodingIdentifier::Bn254V1` holds `None` by `assert_eq!`; every output with identifier `TargetGroupEncodingIdentifier::Bls12381V1` holds `Some`; and for every pair of outputs with equal identifiers the optional bytes are equal by `assert_eq!`
      * `[✅]`   Boundary: the public `create_pairing`; the real chain is `create_pairing`, the arm's `try_new`, `consume_pairing`, and `g1_outside_subgroup_encoding`
      * `[✅]`   Mocked: none
    * `[✅]`   `libraries_sharing_an_identifier_return_the_same_second_group_reference_encoding`
      * `[✅]`   Contract: entry `libraries_sharing_an_identifier_return_the_same_second_group_reference_encoding`; any two concretes whose identifiers are equal return equal bytes
      * `[✅]`   Arrange: `Consumer` with `type Output = Result<(TargetGroupEncodingIdentifier, Vec<u8>), G2OutsideSubgroupEncodingErrorReturn>;` and `consume_pairing<P: IPairingReference>` calling `g2_outside_subgroup_encoding`, propagating its error unchanged, and returning `P::DECLARATION.target_group_encoding` with the bytes read through `AsRef<[u8]>`; every concrete of the declared set runs, collected in order
      * `[✅]`   Act: `create_pairing(&deps, params, CreatePairingPayload)` per concrete
      * `[✅]`   Assert: `assert_eq!(outputs.len(), PAIRING_CONCRETES.len())` and for every pair of outputs with equal identifiers the bytes are equal by `assert_eq!`
      * `[✅]`   Boundary: the public `create_pairing`; the real chain is `create_pairing`, the arm's `try_new`, `consume_pairing`, and `g2_outside_subgroup_encoding`
      * `[✅]`   Mocked: none
    * `[✅]`   `each_second_group_decoder_refuses_the_second_group_reference_encoding`
      * `[✅]`   Contract: entry `each_second_group_decoder_refuses_the_second_group_reference_encoding`; the concrete's `decode_g2` over its own second-group reference encoding returns `Err(DecodeG2ErrorReturn::NotInSubgroup)`
      * `[✅]`   Arrange: `Consumer` with `type Output = Result<Result<(), DecodeG2ErrorReturn>, G2OutsideSubgroupEncodingErrorReturn>;` and `consume_pairing<P: IPairingReference>` calling `g2_outside_subgroup_encoding`, propagating its error unchanged, passing the bytes to `decode_g2(DecodeG2Params, bytes.as_ref())`, and returning its result with the success value replaced by `()`; every concrete of the declared set in turn
      * `[✅]`   Act: `create_pairing(&deps, params, CreatePairingPayload)` per concrete
      * `[✅]`   Assert: per concrete, `assert_eq!(result.ok().map(|success| success.output), Some(Ok(Err(DecodeG2ErrorReturn::NotInSubgroup))))`, the variant the concretes' `decode_g2` states for a point outside the subgroup
      * `[✅]`   Boundary: the public `create_pairing`; the real chain is `create_pairing`, the arm's `try_new`, `consume_pairing`, `g2_outside_subgroup_encoding`, and `decode_g2`
      * `[✅]`   Mocked: none
    * `[✅]`   `a_concrete_whose_provisions_the_requirements_do_not_match_is_refused_whole_before_construction`
      * `[✅]`   Contract: entry `a_concrete_whose_provisions_the_requirements_do_not_match_is_refused_whole_before_construction`; given the four `required_*` values are every provision the declaration the public `lookup_pairing_declaration` returns for a member of a different curve states, the factory returns `Err(CreatePairingErrorReturn::Composition(AdmitPairingCompositionErrorReturn::Incompatible { identifier, disagreements }))` naming `params.concrete`, whose `disagreements` holds the curve, verifier-group arithmetic, precompile encoding, and target-group encoding disagreement in that order
      * `[✅]`   Arrange: `Consumer` with `type Output = ();` and `consume_pairing<P: IPairingAdapter>` returning `()`; per concrete, the other-curve member chosen from `PAIRING_CONCRETES` as the first member whose declared curve, read through the public lookup, differs from the concrete's, the requirements built from that member's declaration
      * `[✅]`   Act: `create_pairing(&deps, params, CreatePairingPayload)` per concrete
      * `[✅]`   Assert: per concrete, `assert_eq!(result.err(), Some(CreatePairingErrorReturn::Composition(AdmitPairingCompositionErrorReturn::Incompatible { identifier, disagreements })))` with `identifier` the concrete and `disagreements` the four in order, each `declared` read from the concrete's declaration and each `required` from the other member's declaration through the public lookup
      * `[✅]`   Boundary: the public `create_pairing`; the real chain is `create_pairing`, the public `admit_pairing_composition`, the public `lookup_pairing_declaration`, and the declaration constants they read
      * `[✅]`   Mocked: none
    * `[✅]`   `a_single_disagreeing_provision_reaches_the_caller_whole`
      * `[✅]`   Contract: entry `a_single_disagreeing_provision_reaches_the_caller_whole`; given the four `required_*` values are the concrete's own declared provisions except the target-group encoding, replaced by a value of `TARGET_GROUP_ENCODING_IDENTIFIERS` the concrete does not declare, the factory returns `Err(CreatePairingErrorReturn::Composition(AdmitPairingCompositionErrorReturn::Incompatible { identifier, disagreements }))` whose `disagreements` holds the one `PairingDisagreement::TargetGroupEncoding`
      * `[✅]`   Arrange: the `Consumer` of the block above; per concrete, the requirements read from its declaration through the public lookup, the target-group encoding replaced by the first value of `TARGET_GROUP_ENCODING_IDENTIFIERS` unequal to the declared one
      * `[✅]`   Act: `create_pairing(&deps, params, CreatePairingPayload)` per concrete
      * `[✅]`   Assert: per concrete, `assert_eq!(result.err(), Some(CreatePairingErrorReturn::Composition(AdmitPairingCompositionErrorReturn::Incompatible { identifier, disagreements: vec![PairingDisagreement::TargetGroupEncoding { declared, required }] })))` with `declared` read from the concrete's declaration and `required` the replaced value
      * `[✅]`   Boundary: the boundary of the block above
      * `[✅]`   Mocked: none
    * `[✅]`   `each_first_group_decoder_refuses_the_first_group_reference_encoding_where_one_exists`
      * `[✅]`   Contract: entry `each_first_group_decoder_refuses_the_first_group_reference_encoding_where_one_exists`; whenever the concrete's `g1_outside_subgroup_encoding` holds bytes, its `decode_g1` over them returns `Err(DecodeG1ErrorReturn::NotInSubgroup)`
      * `[✅]`   Arrange: `Consumer` with `type Output = Result<Option<Result<(), DecodeG1ErrorReturn>>, G1OutsideSubgroupEncodingErrorReturn>;` and `consume_pairing<P: IPairingReference>` calling `g1_outside_subgroup_encoding`, propagating its error unchanged, passing present bytes to `decode_g1(DecodeG1Params, bytes.as_ref())`, and returning its result with the success value replaced by `()`; every concrete of the declared set in turn, so a concrete with no first-group encoding and a concrete with one both run
      * `[✅]`   Act: `create_pairing(&deps, params, CreatePairingPayload)` per concrete
      * `[✅]`   Assert: per concrete, `matches!` of the output against `Some(Ok(None))` or `Some(Ok(Some(Err(DecodeG1ErrorReturn::NotInSubgroup))))`, and across the declared set at least one output is `Some(Ok(Some(_)))`
      * `[✅]`   Boundary: the public `create_pairing`; the real chain is `create_pairing`, the arm's `try_new`, `consume_pairing`, `g1_outside_subgroup_encoding`, and `decode_g1`
      * `[✅]`   Mocked: none

  * `[✅]`   `directionality`
    * `[✅]`   The `factory` module depends on each concrete through its `provides`, on `admit_pairing_composition` through its `provides`, in its mock, tests, and integration tests on `lookup_pairing_declaration` through its `provides`, and on its own interface; each concrete, the composition admission, and the lookup depend on the `factory` module's surface, the family form's recorded cycle and the crate's one cycle; `IPairingConsumer` names `IPairingArithmetic` and `IPairingReference` within the `factory` module; the crate's public surface is the `factory` module's `provides`; no edge between crates beyond `domain`, `zeroize`, and the curve libraries
    * `[✅]`   `kem/bb1_depth_one` consumes the family through `create_pairing` and an `IPairingConsumer`; `harness-crypto/main` calls `create_pairing` with an `IPairingConsumer` bounded by `IPairingArithmetic + IPairingReference` and hands the concrete to the generate concrete, whose `harness-crypto/generate/solidity/constants` and `harness-crypto/generate/solidity/rejection_vectors` read the order and the encodings from it

  * `[✅]`   `requirements`
    * `[✅]`   `cargo check --workspace --all-targets --all-features`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo fmt --all --check`, and `cargo deny check` complete without error or warning in every target, the pairing concretes' unused-item warnings having no cause once the factory constructs them
    * `[✅]`   `CreatePairingErrorReturn` derives `Debug`, `PartialEq`, and `Eq`; the refusal blocks assert the whole error by `assert_eq!`
    * `[✅]`   Every test in `adapters/pairing` passes
    * `[✅]`   The `create_pairing_admits_the_…_concrete` blocks pass, each proving its arm admits its concrete against the official consumer mock (CR-10, every concrete constructed through the family's surface)
    * `[✅]`   The `create_pairing_returns_the_composition_refusal_whole_for_the_…_concrete` blocks pass, each proving its arm calls the composition admission with its identifier and the requirements `params` carries and returns the refusal whole (CR-10, admission by the family's composition admission before construction)
    * `[✅]`   The private and public blocks `a_concrete_whose_provisions_the_requirements_do_not_match_is_refused_whole_before_construction` and `a_single_disagreeing_provision_reaches_the_caller_whole` pass over the real admission and lookup (CR-10, the refusal crosses the factory whole)
    * `[✅]`   Each `the_…_concrete_from_the_factory_samples_and_encodes_through_secret` integration test passes, each proving its `Secret → <Concrete> → create_pairing` chain with every link real (CR-07, secret material crosses the family inside `Secret`; CR-10, every concrete reached through the factory)
    * `[✅]`   No `ark-` crate or `halo2curves` is named outside its concretes, and no code outside `adapters/pairing` can name a concrete

* `[ ]`   `harness-crypto/benchmark` **Pairing benchmark timing scalar multiplication, multi-scalar multiplication, and the pairing-product check on whichever concrete the pairing factory hands it; creates the `apps/harness-crypto` crate**

  * `[✅]`   `objective`
    * `[✅]`   Problem: the run constructs it for each selected concrete with each selected identifier and records the measurements, so each concrete's cost for the operations the KEM, envelope, and proof perform is measured through the factory, with no module naming a library
    * `[✅]`   Functional: handed any concrete by `create_pairing`, the benchmark draws two scalars from the randomness family, samples them through the concrete's sampling bound, and times first-group and second-group scalar multiplication, first-group and second-group multi-scalar multiplication over two terms, and the pairing-product check over two terms
    * `[✅]`   Functional: each timing is the mean over a configured number of iterations, returned per operation with the exact `PairingConcrete` measured and the nonzero iteration count used to compute it
    * `[✅]`   Functional: a failed draw or a failed sampling is returned unchanged in the error arm, before anything is timed
    * `[✅]`   Non-functional: the benchmark names no curve library and no concrete; it reaches every concrete only through `create_pairing` and `IPairingAdapter`

  * `[✅]`   `role`
    * `[✅]`   App module: an `IPairingConsumer` the harness passes to `create_pairing` for each selected concrete with each selected identifier; it is a class in the adapter role, implementing a repo-owned trait and wrapping the operating system's monotonic clock
    * `[✅]`   Creates the `apps/harness-crypto` crate as a library crate and adds `"apps/*"` to the root manifest's `members`, the member glob `workspace/cargo` specifies, so the crate is admitted
    * `[✅]`   Returns the timings, concrete identity, and iteration count of the concrete it is handed and selects nothing; the harness run constructs it for each selected concrete with each selected identifier and records the measurements
    * `[✅]`   Does not author `main.rs`; `harness-crypto/main` authors the binary entry, reading `harness-crypto/config` and composing every family through its factory
    * `[✅]`   Does not time decoding, encoding, or sampling
    * `[✅]`   Does not carry a commit

  * `[✅]`   `module`
    * `[✅]`   Bounded context: the `benchmark` module of `apps/harness-crypto`, holding `PairingBenchmark`, its constructor params, its timings and return types, its implementation of `IPairingConsumer`, its case runner, and its builders
    * `[✅]`   Creates the crate at `apps/harness-crypto`, admitted by the `apps/*` glob this node adds
    * `[✅]`   Outside: the choice of which concretes to run, the iteration count's configured value, the recording of timings, and every other harness module

  * `[✅]`   `deps`
    * `[✅]`   `pairing`, `adapters/pairing`, adapter ring, path dependency; supplies `IPairingConsumer`, `ConsumePairingParams`, `ConsumePairingPayload`, `IPairingAdapter`, `ISampleUniformScalar`, `SampleUniformScalarParams`, `SampleUniformScalarPayload`, `SampleUniformScalarErrorReturn`, and every method's params and payload types used below; in `interface.rs`'s case types `IPairingArithmetic`, `IPairingReference`, `ConsumePairingParams`, `ConsumePairingPayload`, `CreatePairingParams`, and `CreatePairingErrorReturn`; in the case runner `create_pairing`, `IPairingConsumer`, `build_create_pairing_deps`, `build_create_pairing_params`, and `build_create_pairing_payload`; in the unit test and the integration test `IPairingConsumer`, `IPairingArithmetic`, `IPairingReference`, `ConsumePairingParams`, `ConsumePairingPayload`, `build_create_pairing_params`, and `CreatePairingParamsOverrides`; `PairingConcrete` and `MockIPairingAdapterFailureMode` in the mock, the unit test, and the integration test, and `PAIRING_CONCRETES` in the integration test; direction inward, app on adapter
    * `[✅]`   `random`, `adapters/random`, adapter ring, path dependency; supplies `IRandomSourceAdapter`, `FillBytesParams`, `FillBytesPayload`, and `FillBytesErrorReturn`; the source `create_random_source` returns for the `random_source` selection of the case params, the family's mock concrete by default, is the source the case runner's pairing level hands the block's consumer, which hands it the benchmark builder; in `interface.rs`'s case types `CreateRandomSourceParams` and `CreateRandomSourceErrorReturn`; in the case runner `create_random_source`, `build_create_random_source_deps`, `build_create_random_source_params`, `CreateRandomSourceParamsOverrides`, and `build_create_random_source_payload`; in the unit test and the integration test `IRandomSourceAdapter`, `build_create_random_source_params`, `CreateRandomSourceParamsOverrides`, and `RandomSourceKind`, in the integration test also `RANDOM_SOURCE_KINDS`, and in the unit test `MockIRandomSourceAdapterFailureMode` and `FillBytesErrorReturn`
    * `[✅]`   `pairing` and `random` with their `mocks` features, as dev-dependencies and through this crate's `mocks` feature
    * `[✅]`   `std::time::Instant` and `core::time::Duration`, standard library, the monotonic clock and its measure; `core::num::NonZeroU32`, standard library, the iteration count, so the mean never divides by zero; `core::convert::Infallible`, standard library, the constructor's error arm
    * `[✅]`   No external crate at runtime; no reverse dependency

  * `[ ]`   `context_slice`
    * `[ ]`   From `pairing`: `IPairingConsumer` with `type Output` and `consume_pairing<P: IPairingArithmetic + IPairingReference>(&self, ConsumePairingParams, ConsumePairingPayload<P>) -> Self::Output`; `ConsumePairingParams` carries `concrete`, the already-selected typed `PairingConcrete` identifier the measurement carries, so no declaration-lookup dependency is introduced; `ConsumePairingPayload<P>` with only `adapter`, held by value; the adapter methods `g1_generator`, `g2_generator`, `mul_g1`, `mul_g2`, `msm_g1`, `msm_g2`, and `pairing_product_is_one`, each returning `Result<_, Infallible>`; `ISampleUniformScalar` with `UNIFORM_BYTES_LENGTH` and `sample_from_uniform_bytes`
    * `[✅]`   From `pairing`'s mocks: `build_create_pairing_deps<C: IPairingConsumer>(consumer: C, CreatePairingDepsOverrides) -> CreatePairingDeps<C>` and `build_create_pairing_payload(CreatePairingPayloadOverrides) -> CreatePairingPayload`, the deps and payload of the runner's `create_pairing` call, and `build_create_pairing_params(CreatePairingParamsOverrides) -> CreatePairingParams`, whose default selects the family's mock concrete, the `pairing` field's default in the case params and the selection a block overrides
    * `[✅]`   From `random`: `create_random_source(&CreateRandomSourceDeps, CreateRandomSourceParams, CreateRandomSourcePayload) -> CreateRandomSourceReturn` with `CreateRandomSourceSuccessReturn { adapter }`, the leaf call the case runner's pairing level makes, `build_create_random_source_deps(CreateRandomSourceDepsOverrides) -> CreateRandomSourceDeps` and `build_create_random_source_payload(CreateRandomSourcePayloadOverrides) -> CreateRandomSourcePayload`, each returning its production value, `build_create_random_source_params(CreateRandomSourceParamsOverrides)` whose default selects the family's mock concrete and which the level rebuilds from the `Copy` `kind` of the selection it carries, `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode)`, the mode `FillBytesRefused` refusing every draw with `FillBytesErrorReturn::MockIRandomSourceAdapter`, and `RANDOM_SOURCE_KINDS`, the declared set of real kinds the public integration test's sources run over
    * `[✅]`   From `pairing`: `PAIRING_CONCRETES`, the declared set of real concretes the integration test runs over, and `PairingConcrete::Mock(MockIPairingAdapterFailureMode)`, whose default `Succeeds` is the selection `build_create_pairing_params` makes and whose mode `SampleScalarWrongLength` makes the mock concrete's sampling return `Err(SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual: 64 })`
    * `[✅]`   From `random`: `IRandomSourceAdapter::fill_bytes(&self, FillBytesParams, FillBytesPayload) -> Result<FillBytesSuccessReturn, FillBytesErrorReturn>`, the draw inside a `Secret`
    * `[✅]`   From the standard library: `Instant::now()` and `Instant::elapsed()`, `Duration`'s division by `u32`, and `NonZeroU32::get()`

  * `[✅]`   `Cargo.toml`
    * `[✅]`   Adds `"apps/*"` to `[workspace] members`, which reads `["crates/*", "adapters/*", "apps/*"]`; every other table and key is unchanged

  * `[✅]`   `apps/harness-crypto/Cargo.toml`
    * `[✅]`   `[package]` with `name = "harness-crypto"`, `edition.workspace = true`, `rust-version.workspace = true`, and `publish.workspace = true`; no `version` key
    * `[✅]`   `[dependencies]` with `pairing = { path = "../../adapters/pairing" }` and `random = { path = "../../adapters/random" }`
    * `[✅]`   `[dev-dependencies]` with `pairing = { path = "../../adapters/pairing", features = ["mocks"] }` and `random = { path = "../../adapters/random", features = ["mocks"] }`
    * `[✅]`   `[features]` with `mocks = ["pairing/mocks", "random/mocks"]`
    * `[✅]`   `[lints]` with `workspace = true`
    * `[✅]`   No other table

  * `[✅]`   `apps/harness-crypto/src/lib.rs`
    * `[✅]`   The crate barrel: `mod benchmark;` and `pub use benchmark::provides::*;`, nothing else
    * `[✅]`   Until `benchmark/mod.rs` exists, `cargo check` reports the unresolved module, which is the RED state for every element below that precedes it

  * `[✅]`   `apps/harness-crypto/src/benchmark/interface.rs`
    * `[✅]`   `PairingBenchmark`, a struct with `pub(super) random: Box<dyn IRandomSourceAdapter>` and `pub(super) iterations: NonZeroU32`
    * `[✅]`   `PairingBenchmarkConstructorParams`, a struct with `pub random: Box<dyn IRandomSourceAdapter>` and `pub iterations: NonZeroU32`, the constructor's deps slot
    * `[✅]`   `PairingBenchmarkTryNewReturn`, the alias `Result<PairingBenchmark, Infallible>`; the error arm is uninhabited because `NonZeroU32` already excludes the one invalid count
    * `[✅]`   `PairingOperationTimings`, a struct with `pub mul_g1: Duration`, `pub mul_g2: Duration`, `pub msm_g1: Duration`, `pub msm_g2: Duration`, and `pub pairing_product: Duration`, each the mean time of one call
    * `[✅]`   `PairingBenchmarkSuccessReturn`, a struct with `pub concrete: PairingConcrete`, `pub iterations: NonZeroU32`, and `pub timings: PairingOperationTimings`; a measurement carries the selected library and the count used to compute each mean
    * `[✅]`   `PairingBenchmarkErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]` and the variants `FillBytes(FillBytesErrorReturn)` and `SampleScalar(SampleUniformScalarErrorReturn)`, each carrying its callee's error unchanged with the callee's derives
    * `[✅]`   `PairingBenchmarkReturn`, the alias `Result<PairingBenchmarkSuccessReturn, PairingBenchmarkErrorReturn>`, the consumer's `Output`
    * `[✅]`   `IPairingBenchmarkCaseConsumer`, under `#[cfg(any(test, feature = "mocks"))]`, a trait with `type Output;` and `fn consume_pairing_benchmark_case<P: IPairingArithmetic + IPairingReference>(&self, random: Box<dyn IRandomSourceAdapter>, params: ConsumePairingParams, payload: ConsumePairingPayload<P>) -> Self::Output`, the source, the consume params, and the consume payload holding the adapter each arriving by value because the subject's call consumes them; the trait carries no type parameters, so the case runner is generic in the consumer alone and the case's pairing generic arrives with the method
    * `[✅]`   `PairingBenchmarkCaseDeps<C: IPairingBenchmarkCaseConsumer>`, under the same gate, a struct with the field `pub consumer: C`
    * `[✅]`   `PairingBenchmarkCaseParams`, under the same gate, a struct with the fields `pub pairing: CreatePairingParams` and `pub random_source: CreateRandomSourceParams`, each family's full selection
    * `[✅]`   `PairingBenchmarkCasePayload`, under the same gate, the fieldless struct
    * `[✅]`   `PairingBenchmarkCaseSuccessReturn<O>`, under the same gate, a struct with the field `pub output: O`
    * `[✅]`   `PairingBenchmarkCaseErrorReturn`, under the same gate, an enum with `#[derive(Debug, PartialEq, Eq)]` and a variant per factory the chain calls — `CreatePairing(CreatePairingErrorReturn)` and `CreateRandomSource(CreateRandomSourceErrorReturn)` — each factory's create error carried intact
    * `[✅]`   `PairingBenchmarkCaseReturn<O>`, under the same gate, the alias `Result<PairingBenchmarkCaseSuccessReturn<O>, PairingBenchmarkCaseErrorReturn>`
    * `[✅]`   `RunPairingBenchmarkCaseFn<C>`, under the same gate, the alias `fn(&PairingBenchmarkCaseDeps<C>, PairingBenchmarkCaseParams, PairingBenchmarkCasePayload) -> PairingBenchmarkCaseReturn<<C as IPairingBenchmarkCaseConsumer>::Output>`
    * `[✅]`   Imports `IRandomSourceAdapter` and `FillBytesErrorReturn` from `random`, `SampleUniformScalarErrorReturn` and `PairingConcrete` from `pairing`, `core::num::NonZeroU32`, `core::time::Duration`, and `core::convert::Infallible`; and, under `#[cfg(any(test, feature = "mocks"))]`, `IPairingArithmetic`, `IPairingReference`, `ConsumePairingParams`, `ConsumePairingPayload`, `CreatePairingParams`, and `CreatePairingErrorReturn` from `pairing` and `CreateRandomSourceParams` and `CreateRandomSourceErrorReturn` from `random`; no derives beyond those stated

  * `[ ]`   `apps/harness-crypto/src/benchmark/interaction.spec.md`
    * `[✅]`   `PairingBenchmark::try_new(params: PairingBenchmarkConstructorParams) -> PairingBenchmarkTryNewReturn`: one branch; outcome `Ok(PairingBenchmark)` holding the random source and the iteration count; the error arm has no branch
    * `[ ]`   `consume_pairing<P: IPairingAdapter>(&self, params: ConsumePairingParams, payload: ConsumePairingPayload<P>) -> PairingBenchmarkReturn`, draw failed: condition `self.random.fill_bytes(FillBytesParams, FillBytesPayload { length: P::Scalar::UNIFORM_BYTES_LENGTH })` returns `Err(error)` for either scalar; outcome `Err(PairingBenchmarkErrorReturn::FillBytes(error))`; nothing is timed
    * `[✅]`   Sampling failed: condition `P::Scalar::sample_from_uniform_bytes(SampleUniformScalarParams, SampleUniformScalarPayload { uniform: draw.bytes })` returns `Err(error)` for either scalar; outcome `Err(PairingBenchmarkErrorReturn::SampleScalar(error))`; nothing is timed
    * `[ ]`   Measured: both scalars `a` and `b` sampled; the setup takes the generators `g1` and `g2` and computes `p1 = g1 · a` and `p2 = g2 · b` through `mul_g1` and `mul_g2`, untimed; then, for each operation in turn, `Instant::now()` is read, the operation runs `self.iterations.get()` times, and `elapsed()` divided by `self.iterations.get()` is its mean: `mul_g1` of a clone of `p1` by a clone of `a`; `mul_g2` of a clone of `p2` by a clone of `b`; `msm_g1` over the terms `MsmG1Term { base: p1, scalar: a }` and `MsmG1Term { base: g1, scalar: b }`; `msm_g2` over the corresponding `MsmG2Term` values; `pairing_product_is_one` over the terms `(p1, g2)` and `(g1, p2)`, each term's elements cloned per call; outcome `Ok(PairingBenchmarkSuccessReturn { concrete: params.concrete, iterations: self.iterations, timings })`
    * `[✅]`   Every adapter call returns `Result<_, Infallible>` and is unpacked irrefutably; the scalars are cloned from their `Secret`s by `expose().clone()`, and each clone is cleared when the payload holding it drops
    * `[ ]`   Ordering: both draws and both samplings precede any timing, so a failure returns before the clock is read; `params.concrete`, the selected typed `PairingConcrete` identifier, is carried into the result; no independently supplied metadata is read
    * `[ ]`   `Integration: callee dispositions`, the `pairing/factory` entries this node's route reaches, each proven by the public integration block of the same name; the route of each is `create_pairing`, the arm's concrete through its `try_new`, `PairingBenchmark::consume_pairing`, and the calls it makes on that concrete and on the source `create_random_source` constructs; the outer-edge collaborators on that route are the operating system's generator and monotonic clock, both run real, except where an entry names the pairing family's mock concrete
      * `[ ]`   `the_consumer_receives_the_concrete_the_params_name`: carried; restated where this node transforms it: the `concrete` of the `Ok` measurement the benchmark returns equals `params.concrete`, the selected typed `PairingConcrete` identifier, read from the benchmark's output
      * `[✅]`   `a_sampled_scalar_encodes_through_secret`: carried; restated where this node transforms it: the benchmark draws `P::Scalar::UNIFORM_BYTES_LENGTH` bytes into a `Secret`, samples both scalars with `sample_from_uniform_bytes`, and consumes them in the group operations instead of `encode_scalar`, so the output is `Ok` for every concrete
      * `[✅]`   `a_sampling_refusal_reaches_the_caller_unchanged`: carried; restated where this node transforms it: a sampling refusal is the output's `Err(PairingBenchmarkErrorReturn::SampleScalar(error))`, the error whole; its concrete is the pairing family's mock concrete selected by `PairingConcrete::Mock(MockIPairingAdapterFailureMode::SampleScalarWrongLength)`
    * `[✅]`   `Integration: own entries`, a bulleted section, each entry named and proven by the public integration block of the same name
      * `[✅]`   `every_operation_is_timed_on_every_concrete`: condition the benchmark runs through `create_pairing` on a concrete of `PAIRING_CONCRETES` with the source `create_random_source` constructs for a member of `RANDOM_SOURCE_KINDS`; outcome `Ok` whose timings for `mul_g1`, `mul_g2`, `msm_g1`, `msm_g2`, and `pairing_product_is_one` are each greater than `Duration::ZERO`; variation every concrete of the declared set paired with each member of `RANDOM_SOURCE_KINDS`, whose operations and draws are real, so a timing left at its zero default fails; edge each operation runs on scalars sampled from the draw and on the concrete's own points, so none is skipped
    * `[✅]`   `Integration: public surface`: an outside caller constructs `PairingBenchmark` with `PairingBenchmark::try_new` from `PairingBenchmarkConstructorParams` holding the source `create_random_source` returns and an iteration count, passes it as `CreatePairingDeps { consumer }` to `create_pairing` from `pairing`'s public surface with `CreatePairingParams` naming a concrete and `CreatePairingPayload`, and observes the benchmark's `PairingBenchmarkReturn` in `CreatePairingSuccessReturn`; the test drives that composition through the case runner — a block declares one consumer implementing `IPairingBenchmarkCaseConsumer` and calls `run_pairing_benchmark_case` once per case, the runner's pairing level running `create_random_source` inward and handing the source, the consume params, and the consume payload to the consumer, whose method builds the benchmark and makes the single `consume_pairing` call; the entries proven are every carried entry and the own entry above; the route of each is as the callee dispositions state; the outer-edge collaborators are the operating system's generator and monotonic clock, run real, and, for `a_sampling_refusal_reaches_the_caller_unchanged`, the pairing family's mock concrete; the fixtures are `build_pairing_benchmark`, `build_pairing_benchmark_case_deps`, `build_pairing_benchmark_case_params`, `build_pairing_benchmark_case_payload`, `run_pairing_benchmark_case`, `build_create_pairing_params`, and `build_create_random_source_params`

  * `[✅]`   `apps/harness-crypto/src/benchmark/mock.rs`
    * `[✅]`   Module-level `#![allow(dead_code, clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::too_many_arguments)]`
    * `[✅]`   `PairingBenchmarkConstructorParamsOverrides`, `#[derive(Default)]`, the one field `pub iterations: Option<NonZeroU32>`, since the random source is handed; `build_pairing_benchmark_constructor_params(random: Box<dyn IRandomSourceAdapter>, overrides: PairingBenchmarkConstructorParamsOverrides) -> PairingBenchmarkConstructorParams`, the source its leading parameter, and `iterations` defaulting to `NonZeroU32::MIN`
    * `[✅]`   `build_pairing_benchmark(random: Box<dyn IRandomSourceAdapter>, overrides: PairingBenchmarkConstructorParamsOverrides) -> PairingBenchmark`, returning the real instance from `PairingBenchmark::try_new(build_pairing_benchmark_constructor_params(random, overrides))` through the irrefutable pattern `let Ok(benchmark) = …;`
    * `[✅]`   `PairingOperationTimingsOverrides`, `#[derive(Default)]`, one `Option<Duration>` per field; `build_pairing_operation_timings(overrides: PairingOperationTimingsOverrides) -> PairingOperationTimings`, each field defaulting to `Duration::ZERO`
    * `[✅]`   `PairingBenchmarkSuccessReturnOverrides`, `#[derive(Default)]`, fields `pub concrete: Option<PairingConcrete>`, `pub iterations: Option<NonZeroU32>`, and `pub timings: Option<PairingOperationTimings>`; `build_pairing_benchmark_success_return(overrides: PairingBenchmarkSuccessReturnOverrides) -> PairingBenchmarkSuccessReturn`, defaulting to `PairingConcrete::Mock(MockIPairingAdapterFailureMode::Succeeds)`, `NonZeroU32::MIN`, and `build_pairing_operation_timings(Default::default())`
    * `[✅]`   `PairingBenchmarkCaseParamsOverrides`, `#[derive(Default)]`, the same fields as `PairingBenchmarkCaseParams`'s as `Option`s; `build_pairing_benchmark_case_params(overrides: PairingBenchmarkCaseParamsOverrides) -> PairingBenchmarkCaseParams`, each field defaulting to its family's `build_create_*_params(Default::default())`; no corruptions type and no invalidator, the selections arriving admitted
    * `[✅]`   `PairingBenchmarkCaseDepsOverrides`, the fieldless unit struct with `#[derive(Default)]`, the consumer handed; `build_pairing_benchmark_case_deps<C: IPairingBenchmarkCaseConsumer>(consumer: C, _overrides: PairingBenchmarkCaseDepsOverrides) -> PairingBenchmarkCaseDeps<C>`, the consumer its leading parameter; no corruptions type and no invalidator
    * `[✅]`   `PairingBenchmarkCasePayloadOverrides`, the fieldless unit struct with `#[derive(Default)]`; `build_pairing_benchmark_case_payload(_overrides: PairingBenchmarkCasePayloadOverrides) -> PairingBenchmarkCasePayload`; no corruptions type and no invalidator
    * `[✅]`   `PairingBenchmarkCaseSuccessReturnOverrides`, the fieldless unit struct with `#[derive(Default)]`, the output handed; `build_pairing_benchmark_case_success_return<O>(output: O, _overrides: PairingBenchmarkCaseSuccessReturnOverrides) -> PairingBenchmarkCaseSuccessReturn<O>`, the output its leading parameter; no corruptions type and no invalidator
    * `[✅]`   Private `CaseAtPairing`, a struct implementing `IPairingConsumer` with `type Output = PairingBenchmarkCaseReturn<C::Output>`, carrying `&'a C` the case consumer and `CreateRandomSourceParams` the case's randomness selection; its `consume_pairing` builds the inner call through the randomness family's builders — `build_create_random_source_params` from the `Copy` `kind` of the selection it carries, `build_create_random_source_deps`, and `build_create_random_source_payload` — runs `create_random_source`, a refusal mapped to `PairingBenchmarkCaseErrorReturn::CreateRandomSource`, then calls `consumer.consume_pairing_benchmark_case(created.adapter, params, payload)`, its output wrapped `Ok(PairingBenchmarkCaseSuccessReturn { output })`; no factory result unpacked by `unwrap`, `expect`, or `let … else { panic!(…) }`
    * `[✅]`   `run_pairing_benchmark_case<C: IPairingBenchmarkCaseConsumer>(deps: &PairingBenchmarkCaseDeps<C>, params: PairingBenchmarkCaseParams, _payload: PairingBenchmarkCasePayload) -> PairingBenchmarkCaseReturn<C::Output>`, the production type `RunPairingBenchmarkCaseFn<C>` and nothing else, running `create_pairing` through the family's builders — `build_create_pairing_deps` handing `CaseAtPairing` carrying `&deps.consumer` and `params.random_source`, `build_create_pairing_payload` for the payload — over `params.pairing`, a success's `.output` relayed and a `CreatePairingErrorReturn` mapped to `PairingBenchmarkCaseErrorReturn::CreatePairing`
    * `[✅]`   Dispositions: `PairingBenchmarkConstructorParams`, `PairingOperationTimings`, and `PairingBenchmarkSuccessReturn` take an Overrides type and a builder, and no corruptions type and no invalidator; `PairingBenchmark` takes `build_pairing_benchmark` and no mock of the class; `PairingBenchmarkErrorReturn`, `PairingBenchmarkTryNewReturn`, and `PairingBenchmarkReturn` take no mock; `PairingBenchmarkCaseDeps`, `PairingBenchmarkCaseParams`, `PairingBenchmarkCasePayload`, and `PairingBenchmarkCaseSuccessReturn`, object types `interface.rs` declares, each with its builder above; `PairingBenchmarkCaseErrorReturn`, an enum, produced by the case level and the runner; `PairingBenchmarkCaseReturn`, an alias of `Result`, and `RunPairingBenchmarkCaseFn`, a function type whose function is `run_pairing_benchmark_case`; `IPairingBenchmarkCaseConsumer`, a trait `interface.rs` declares whose consumer `run_pairing_benchmark_case` drives; no function mock, `consume_pairing` being the method of the injected `IPairingConsumer`, whose mock `pairing` owns
    * `[✅]`   Imports `IRandomSourceAdapter` from `random`, `PairingConcrete` and `MockIPairingAdapterFailureMode` from `pairing`, the standard-library names above, and this module's types from `super::interface`; additionally, for the case level, the builders, and the runner: `IPairingConsumer`, `ConsumePairingParams`, `ConsumePairingPayload`, `IPairingArithmetic`, `IPairingReference`, `create_pairing`, `build_create_pairing_deps`, `build_create_pairing_params`, and `build_create_pairing_payload` from `pairing` and `create_random_source`, `build_create_random_source_deps`, `build_create_random_source_params`, `CreateRandomSourceParamsOverrides`, and `build_create_random_source_payload` from `random`

  * `[ ]`   `apps/harness-crypto/src/benchmark/test.rs`
    * `[✅]`   Module-level `#![allow(clippy::expect_used)]`; imports `IPairingBenchmarkCaseConsumer`, `PairingBenchmarkErrorReturn`, `PairingBenchmarkReturn`, `build_pairing_benchmark`, `build_pairing_benchmark_case_deps`, `build_pairing_benchmark_case_params`, `build_pairing_benchmark_case_payload`, and `run_pairing_benchmark_case` from `super::provides`; `IPairingConsumer`, `IPairingArithmetic`, `IPairingReference`, `ConsumePairingParams`, `ConsumePairingPayload`, `PairingConcrete`, `MockIPairingAdapterFailureMode`, `SampleUniformScalarErrorReturn`, `CreatePairingParamsOverrides`, and `build_create_pairing_params` from `pairing`; `IRandomSourceAdapter`, `RandomSourceKind`, `MockIRandomSourceAdapterFailureMode`, `FillBytesErrorReturn`, `CreateRandomSourceParamsOverrides`, and `build_create_random_source_params` from `random`
    * `[✅]`   Every block declares inside the block a unit struct `Consumer` implementing `IPairingBenchmarkCaseConsumer` with `type Output = PairingBenchmarkReturn` and `consume_pairing_benchmark_case<P: IPairingArithmetic + IPairingReference>(&self, random: Box<dyn IRandomSourceAdapter>, params: ConsumePairingParams, payload: ConsumePairingPayload<P>)`, whose body builds the benchmark `build_pairing_benchmark(random, Default::default())` over the received source and makes the block's single call `benchmark.consume_pairing(params, payload)`, returning its `PairingBenchmarkReturn` whole; the block calls `run_pairing_benchmark_case(&deps, params, build_pairing_benchmark_case_payload(Default::default()))` with `deps` from `build_pairing_benchmark_case_deps(Consumer, Default::default())`, observed as `result`, whose default `pairing` selection is the pairing family's mock concrete and whose default `random_source` selection is the randomness family's mock concrete, a `CreatePairingErrorReturn` reaching the block whole through `PairingBenchmarkCaseErrorReturn::CreatePairing` and a `CreateRandomSourceErrorReturn` through `PairingBenchmarkCaseErrorReturn::CreateRandomSource` had a factory refused; each block asserts `result.is_ok()` outside the consumer, proving the runner's level ran; the subject is `PairingBenchmark::consume_pairing`, the only function of its module that runs; no block names a concrete
    * `[✅]`   `consume_pairing_returns_the_draw_error_unchanged`
      * `[✅]`   Contract: `self.random.fill_bytes` returns `Err(error)` → `Err(PairingBenchmarkErrorReturn::FillBytes(error))`, with nothing timed
      * `[✅]`   Collaborators: the randomness family's mock concrete, selected through the case params' `random_source` selection naming the mode that refuses every draw; the pairing family's mock concrete through the case params' `pairing` selection at its default; fixtures `build_pairing_benchmark`, `build_pairing_benchmark_case_deps`, `build_pairing_benchmark_case_params`, `build_pairing_benchmark_case_payload`, `run_pairing_benchmark_case`, `build_create_pairing_params`, and `build_create_random_source_params`
      * `[✅]`   Arrange: `params` from `build_pairing_benchmark_case_params` with the `random_source` override set to `build_create_random_source_params` carrying the kind override `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)`, a mode that differs from the builder's default `Succeeds`, so a benchmark that ignores a refused draw fails, and the `pairing` selection at its default
      * `[✅]`   Act: `run_pairing_benchmark_case(&deps, params, build_pairing_benchmark_case_payload(Default::default()))`
      * `[✅]`   Assert: `assert_eq!(result.ok().and_then(|success| success.output.err()), Some(PairingBenchmarkErrorReturn::FillBytes(FillBytesErrorReturn::MockIRandomSourceAdapter)))`, the whole error, the `FillBytes` variant being the subject's wrapping and the carried error the mock concrete's own variant written as a literal
    * `[ ]`   `consume_pairing_draws_the_length_the_sampling_bound_requires`
      * `[ ]`   Contract: both scalars drawn with `P::Scalar::UNIFORM_BYTES_LENGTH` bytes and sampled → `Ok(PairingBenchmarkSuccessReturn { concrete: params.concrete, iterations: self.iterations, timings })`
      * `[✅]`   Collaborators: the randomness family's mock concrete through the case params' `random_source` selection at its default, drawing exactly the requested length; the pairing family's mock concrete through the case params' `pairing` selection at its default; fixtures `build_pairing_benchmark`, `build_pairing_benchmark_case_deps`, `build_pairing_benchmark_case_params`, `build_pairing_benchmark_case_payload`, `run_pairing_benchmark_case`, `build_create_pairing_params`, and `build_create_random_source_params`
      * `[✅]`   Arrange: `params` from `build_pairing_benchmark_case_params` with no override; the pairing mock concrete's sampling refuses an input whose length differs from its bound, so a benchmark that requests another length fails
      * `[✅]`   Act: `run_pairing_benchmark_case(&deps, params, build_pairing_benchmark_case_payload(Default::default()))`
      * `[ ]`   Assert: `assert_eq!(result.ok().and_then(|success| success.output.ok()).map(|success| success.concrete), Some(PairingConcrete::Mock(MockIPairingAdapterFailureMode::Succeeds)))`, the runner's success arm present and the measurement's `concrete` equal to the `params.concrete` the default `pairing` selection admits, proving the measurement carries the selected typed identifier with no read of `P::CONCRETE`; `iterations` and the timings are not asserted, no timing or mean being proven here
    * `[✅]`   `consume_pairing_returns_the_sampling_error_unchanged`
      * `[✅]`   Contract: `P::Scalar::sample_from_uniform_bytes` returns `Err(error)` → `Err(PairingBenchmarkErrorReturn::SampleScalar(error))`, with nothing timed
      * `[✅]`   Collaborators: the randomness family's mock concrete through the case params' `random_source` selection at its default; the pairing family's mock concrete through the case params' `pairing` selection naming the mode that refuses its sampling; fixtures `build_pairing_benchmark`, `build_pairing_benchmark_case_deps`, `build_pairing_benchmark_case_params`, `build_pairing_benchmark_case_payload`, `run_pairing_benchmark_case`, `build_create_pairing_params`, and `build_create_random_source_params`
      * `[✅]`   Arrange: `params` from `build_pairing_benchmark_case_params` with the `pairing` override set to `build_create_pairing_params` carrying the `concrete` override `PairingConcrete::Mock(MockIPairingAdapterFailureMode::SampleScalarWrongLength)`, a mode that differs from the builder's default `Succeeds`, so a benchmark that ignores a refused sampling fails, and the `random_source` selection at its default
      * `[✅]`   Act: `run_pairing_benchmark_case(&deps, params, build_pairing_benchmark_case_payload(Default::default()))`
      * `[✅]`   Assert: `assert_eq!(result.ok().and_then(|success| success.output.err()), Some(PairingBenchmarkErrorReturn::SampleScalar(SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual: 64 })))`, the whole error, the `SampleScalar` variant being the subject's wrapping and the carried error the mock concrete's own value for that mode written as a literal

  * `[✅]`   `construction`
    * `[✅]`   `PairingBenchmark::try_new` is the only producer; the harness run constructs one for each selected concrete with each selected identifier from the random source `create_random_source` returns and the configured iteration count, and passes it as `CreatePairingDeps { consumer }`

  * `[✅]`   `apps/harness-crypto/src/benchmark/mod.rs`
    * `[✅]`   Module declarations: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, `pub mod provides;`, and `#[cfg(test)] mod test;`
    * `[✅]`   `impl PairingBenchmark` with `pub fn try_new(params: PairingBenchmarkConstructorParams) -> PairingBenchmarkTryNewReturn` returning `Ok(PairingBenchmark { random: params.random, iterations: params.iterations })`
    * `[✅]`   `impl IPairingConsumer for PairingBenchmark` with `type Output = PairingBenchmarkReturn;` and `consume_pairing` realizing the branches and ordering of the interaction spec
    * `[✅]`   Imports the `pairing` and `random` names the context slice lists, `std::time::Instant`, and this module's types from `interface`
    * `[✅]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[✅]`   `apps/harness-crypto/src/benchmark/provides.rs`
    * `[✅]`   `pub use super::interface::*;` and `#[cfg(any(test, feature = "mocks"))] pub use super::mock::*;`, nothing else

  * `[✅]`   `apps/harness-crypto/tests/benchmark_integration_test.rs`
    * `[✅]`   Outside the crate, so it reaches only the crate's public surface and the official mocks under the `mocks` feature: module-level `#![allow(clippy::expect_used)]`; imports from `harness_crypto` `IPairingBenchmarkCaseConsumer`, `PairingBenchmarkErrorReturn`, `PairingBenchmarkReturn`, `build_pairing_benchmark`, `build_pairing_benchmark_case_deps`, `build_pairing_benchmark_case_params`, `build_pairing_benchmark_case_payload`, and `run_pairing_benchmark_case`; from `pairing` `IPairingConsumer`, `IPairingArithmetic`, `IPairingReference`, `ConsumePairingParams`, `ConsumePairingPayload`, `PAIRING_CONCRETES`, `PairingConcrete`, `MockIPairingAdapterFailureMode`, `SampleUniformScalarErrorReturn`, `CreatePairingParamsOverrides`, and `build_create_pairing_params`; from `random` `IRandomSourceAdapter`, `RandomSourceKind`, `RANDOM_SOURCE_KINDS`, `CreateRandomSourceParamsOverrides`, and `build_create_random_source_params`; and `Duration` from `core::time`
    * `[✅]`   Every block declares inside itself one consumer implementing `IPairingBenchmarkCaseConsumer` with `type Output = PairingBenchmarkReturn`, whose method builds the benchmark `build_pairing_benchmark(random, Default::default())` over the received source and makes the block's single call `benchmark.consume_pairing(params, payload)`; its body is written once and runs over `PAIRING_CONCRETES`, naming no concrete, except `a_sampling_refusal_reaches_the_caller_unchanged`, whose concrete is the configuration's mock selection; per concrete `params` is `build_pairing_benchmark_case_params` with the `pairing` override set to `build_create_pairing_params` carrying `concrete` and the `random_source` override set to `build_create_random_source_params` carrying the `kind` of a member of `RANDOM_SOURCE_KINDS`, each pairing member run with each member of the random family's declared set, `deps` is `build_pairing_benchmark_case_deps(Consumer, Default::default())`, and the call is `run_pairing_benchmark_case(&deps, params, build_pairing_benchmark_case_payload(Default::default()))`, whose result is observed as `result`, its `output` the `PairingBenchmarkReturn`; each block asserts `result.is_ok()`, proving the runner's level ran; the operating system's generator and monotonic clock run real
    * `[✅]`   `the_consumer_receives_the_concrete_the_params_name`
      * `[✅]`   Contract: entry `the_consumer_receives_the_concrete_the_params_name`; given `params.pairing`'s `concrete` is a variant of `PAIRING_CONCRETES` and the admissions pass, the `concrete` of the `Ok` measurement equals that variant
      * `[✅]`   Arrange: every variant of the declared set in turn, including both libraries of one curve, so a benchmark that returns a fixed concrete, or the factory constructing a sibling library, fails
      * `[✅]`   Act: `run_pairing_benchmark_case(&deps, params, build_pairing_benchmark_case_payload(Default::default()))` per concrete
      * `[✅]`   Assert: per concrete, `assert!(result.ok().and_then(|success| success.output.ok()).map(|measurement| measurement.concrete) == Some(*concrete))`, the variant coming from the declared set and not from the call
      * `[✅]`   Boundary: the public `create_pairing`, driven by the case runner; the real chain is `create_pairing`, the arm's `try_new`, `PairingBenchmark::consume_pairing`, the selected random member's `fill_bytes`, `sample_from_uniform_bytes`, and the concrete's group operations
      * `[✅]`   Mocked: none
    * `[✅]`   `a_sampled_scalar_encodes_through_secret`
      * `[✅]`   Contract: entry `a_sampled_scalar_encodes_through_secret`; the draw of `P::Scalar::UNIFORM_BYTES_LENGTH` bytes crosses inside a `Secret` into `sample_from_uniform_bytes`, and the sampled scalars are consumed by the concrete's group operations, so the output is `Ok`
      * `[✅]`   Arrange: every concrete of the declared set in turn, each sampling bound its own, so a benchmark that draws a length the bound does not name fails for the concrete whose bound differs
      * `[✅]`   Act: `run_pairing_benchmark_case(&deps, params, build_pairing_benchmark_case_payload(Default::default()))` per concrete
      * `[✅]`   Assert: per concrete, `assert_eq!(result.ok().map(|success| success.output.err()), Some(None))`, the runner's success arm present and the output's error arm empty, the whole error shown on failure
      * `[✅]`   Boundary: the public `create_pairing`, driven by the case runner; the real chain is `create_pairing`, the arm's `try_new`, `PairingBenchmark::consume_pairing`, the selected random member's `fill_bytes`, `sample_from_uniform_bytes`, and the concrete's group operations
      * `[✅]`   Mocked: none
    * `[✅]`   `a_sampling_refusal_reaches_the_caller_unchanged`
      * `[✅]`   Contract: entry `a_sampling_refusal_reaches_the_caller_unchanged`; a sampling refusal is the output's `Err(PairingBenchmarkErrorReturn::SampleScalar(error))`, the error whole
      * `[✅]`   Arrange: `params` from `build_pairing_benchmark_case_params` with the `pairing` override set to `build_create_pairing_params` carrying the `concrete` override `PairingConcrete::Mock(MockIPairingAdapterFailureMode::SampleScalarWrongLength)`, against the `Ok` outputs of the entries above, so a benchmark that ignores a refused sampling fails
      * `[✅]`   Act: `run_pairing_benchmark_case(&deps, params, build_pairing_benchmark_case_payload(Default::default()))`
      * `[✅]`   Assert: `assert_eq!(result.ok().and_then(|success| success.output.err()), Some(PairingBenchmarkErrorReturn::SampleScalar(SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual: 64 })))`, the whole error, the carried error the mock concrete's own value for that mode written as a literal
      * `[✅]`   Boundary: the public `create_pairing`, driven by the case runner; the real chain is `create_pairing`, the mock arm's `try_new`, `PairingBenchmark::consume_pairing`, the selected random member's `fill_bytes`, and `sample_from_uniform_bytes`
      * `[✅]`   Mocked: the pairing family's mock concrete, selected by configuration; this block does not prove a real concrete's sampling
    * `[✅]`   `every_operation_is_timed_on_every_concrete`
      * `[✅]`   Contract: entry `every_operation_is_timed_on_every_concrete`; the `Ok` measurement's timings for `mul_g1`, `mul_g2`, `msm_g1`, `msm_g2`, and `pairing_product_is_one` are each greater than `Duration::ZERO`
      * `[✅]`   Arrange: every concrete of the declared set in turn, whose operations are real, so a timing left at its zero default fails
      * `[✅]`   Act: `run_pairing_benchmark_case(&deps, params, build_pairing_benchmark_case_payload(Default::default()))` per concrete
      * `[✅]`   Assert: per concrete, the measurement is extracted with `expect` from `result.ok().and_then(|success| success.output.ok())`; each of the five timings is greater than `Duration::ZERO` under `assert!`; the exact means are not asserted
      * `[✅]`   Boundary: the public `create_pairing`, driven by the case runner; the real chain is `create_pairing`, the arm's `try_new`, `PairingBenchmark::consume_pairing`, the selected random member's `fill_bytes`, `sample_from_uniform_bytes`, and the concrete's group operations and clock reads
      * `[✅]`   Mocked: none

  * `[✅]`   `directionality`
    * `[✅]`   `benchmark` depends on `pairing`'s and `random`'s public surfaces and on the standard library; it names no concrete and no curve library; nothing depends on the crate yet; no cycle

  * `[ ]`   `requirements`
    * `[✅]`   The root `Cargo.toml` lists `members = ["crates/*", "adapters/*", "apps/*"]` and is otherwise unchanged; `apps/harness-crypto/Cargo.toml` carries exactly the tables and keys stated above
    * `[✅]`   `cargo check --workspace --all-targets --all-features`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo fmt --check`, and `cargo deny check` complete without error or warning
    * `[✅]`   `PairingBenchmarkErrorReturn` derives `Debug`, `PartialEq`, and `Eq`
    * `[✅]`   `consume_pairing_returns_the_draw_error_unchanged` passes, asserting the whole error by `assert_eq!`
    * `[✅]`   `consume_pairing_draws_the_length_the_sampling_bound_requires` passes
    * `[✅]`   `consume_pairing_returns_the_sampling_error_unchanged` passes, asserting the whole error by `assert_eq!`
    * `[✅]`   `the_consumer_receives_the_concrete_the_params_name` passes over every concrete of the declared set paired with each member of `RANDOM_SOURCE_KINDS` (CR-10, the concrete the factory constructs reaches the benchmark)
    * `[✅]`   `a_sampled_scalar_encodes_through_secret` passes over every concrete of the declared set paired with each member of `RANDOM_SOURCE_KINDS` (CR-05, a production draw passes through the randomness factory and is sampled by a concrete's sampling bound; CR-07, draws and scalars cross the families inside `Secret` with every link real)
    * `[✅]`   `a_sampling_refusal_reaches_the_caller_unchanged` passes, asserting the whole error by `assert_eq!`
    * `[✅]`   `every_operation_is_timed_on_every_concrete` passes over every concrete of the declared set paired with each member of `RANDOM_SOURCE_KINDS`
    * `[ ]`   Each `Ok` measurement carries `params.concrete`, the selected typed `PairingConcrete` identifier, with no read of `P::DECLARATION` or `P::CONCRETE` and no declaration-lookup dependency introduced merely to recover the selection the caller already holds
    * `[✅]`   `MockIPairingAdapterFailureMode` carries `SampleScalarWrongLength`, and the pairing mock concrete's sampling answers it
    * `[✅]`   No module of `apps/harness-crypto` names `ark-`, `halo2curves`, or a pairing concrete

* `[✅]`   `domain/deployment_identity` **Registry-assigned deployment identity, the 32 bytes of the Registry's `bytes32` deployment key, admitted only when it can name an assigned deployment**

  * `[✅]`   `objective`
    * `[✅]`   Problem: every derivation, capsule, sidecar, and escrow record is bound to one deployment by the Registry's globally unique, non-reusable `deployment_id`, the `bytes32` key its deployment and escrow records carry, so a value naming no deployment must be refused before anything is derived from it (CR-05; the nonce invariant of Deployment Cryptographic Setup)
    * `[✅]`   Functional: one type holds the deployment identity's 32 bytes, reachable only through a read accessor, and its only producer is a fallible constructor
    * `[✅]`   Functional: the constructor takes exactly 32 bytes, so the length is a fact of the params type and never a runtime check
    * `[✅]`   Functional: the constructor refuses the all-zero value, which is what an unassigned `bytes32` storage slot reads as, so it names no deployment the Registry assigned
    * `[✅]`   Functional: every other 32-byte value is admitted and read back unchanged
    * `[✅]`   Non-functional: the module depends on the standard library alone; the `domain` crate's dependencies are unchanged

  * `[✅]`   `role`
    * `[✅]`   Domain: an owned value type in the protocol and domain ring, the deployment every later derivation context, hash-card, and attempt context names
    * `[✅]`   Does not assign, generate, or check the uniqueness of a deployment identity; the Registry assigns it, and uniqueness is a property of consensus
    * `[✅]`   Does not decode a deployment identity from wire bytes of unknown length; the encoding family decodes a `bytes32` into the 32 bytes this constructor takes
    * `[✅]`   Does not create any other module of the `domain` crate
    * `[✅]`   Does not carry a commit

  * `[✅]`   `module`
    * `[✅]`   Bounded context: the `deployment_identity` module of the `domain` crate, holding `DeploymentIdentity`, its length, its constructor params, and its constructor's error and return types
    * `[✅]`   Adds the module to the existing `domain` crate at `crates/domain`; the crate's manifest is unchanged and its barrel gains this module's line
    * `[✅]`   Outside: the Registry's assignment of identities, the deployment record and hash-card the identity keys, and the encoding of the identity on the wire

  * `[✅]`   `deps`
    * `[✅]`   `domain/secret` created the `domain` crate this module joins; this module imports nothing from `secret`
    * `[✅]`   The standard library: `u8`, arrays, and `Iterator::all`, through the prelude
    * `[✅]`   No external crate and no repository crate; `crates/domain/Cargo.toml` is unchanged; direction inward, `domain` is the innermost ring
    * `[✅]`   No reverse dependency; `workflows/sidecar/derivation_context` is this module's first consumer

  * `[✅]`   `context_slice`
    * `[✅]`   From the standard library: `<[u8]>::iter` and `Iterator::all`

  * `[✅]`   `crates/domain/src/lib.rs`
    * `[✅]`   The crate barrel reads `mod deployment_identity;`, `mod secret;`, `pub use deployment_identity::provides::*;`, and `pub use secret::provides::*;`, nothing else
    * `[✅]`   Until `deployment_identity/mod.rs` exists, `cargo check` reports the unresolved `mod deployment_identity`, which is the RED state for every element below that precedes the implementation

  * `[✅]`   `crates/domain/src/deployment_identity/interface.rs`
    * `[✅]`   `DEPLOYMENT_IDENTITY_LENGTH`, a `pub const` of type `usize` with value `32`, the width of the Registry's `bytes32` deployment key
    * `[✅]`   `DeploymentIdentity`, a struct with `#[derive(Clone, Debug, PartialEq, Eq)]` and the one field `pub(super) bytes: [u8; DEPLOYMENT_IDENTITY_LENGTH]`, so only the `deployment_identity` module and its children reach the field
    * `[✅]`   `DeploymentIdentityConstructorParams`, a struct with the one field `pub bytes: [u8; DEPLOYMENT_IDENTITY_LENGTH]`; no derives
    * `[✅]`   `DeploymentIdentityTryNewErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]` and the one variant `AllZero`
    * `[✅]`   `DeploymentIdentityTryNewReturn`, the type alias `Result<DeploymentIdentity, DeploymentIdentityTryNewErrorReturn>`
    * `[✅]`   Imports nothing; declares nothing else

  * `[✅]`   `crates/domain/src/deployment_identity/interaction.spec.md`
    * `[✅]`   `DeploymentIdentity::try_new(params: DeploymentIdentityConstructorParams) -> DeploymentIdentityTryNewReturn`, all zero: condition every byte of `params.bytes` is `0`; decision `params.bytes.iter().all(…)` over the byte equal to `0`; dependency call none; outcome `Err(DeploymentIdentityTryNewErrorReturn::AllZero)`
    * `[✅]`   Admitted: condition some byte of `params.bytes` is nonzero; decision the same check; dependency call none; outcome `Ok(DeploymentIdentity { bytes })`, the array moved from the params
    * `[✅]`   `DeploymentIdentity::as_bytes(&self) -> &[u8; DEPLOYMENT_IDENTITY_LENGTH]`: one branch; outcome a shared reference to the held array, no copy, no side effect
    * `[✅]`   Invariants: every `DeploymentIdentity` holds exactly 32 bytes, not all zero; its only producer is `try_new`

  * `[✅]`   `crates/domain/src/deployment_identity/mock.rs`
    * `[✅]`   Module-level `#![allow(dead_code, clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::too_many_arguments)]`
    * `[✅]`   `DeploymentIdentityConstructorParamsOverrides`, `#[derive(Default)]`, one field `pub bytes: Option<[u8; DEPLOYMENT_IDENTITY_LENGTH]>`
    * `[✅]`   `build_deployment_identity_constructor_params(overrides: DeploymentIdentityConstructorParamsOverrides) -> DeploymentIdentityConstructorParams`, the bytes defaulting to `[0x11; DEPLOYMENT_IDENTITY_LENGTH]`
    * `[✅]`   `build_deployment_identity(overrides: DeploymentIdentityConstructorParamsOverrides) -> DeploymentIdentity`, returning the real instance from `DeploymentIdentity::try_new(build_deployment_identity_constructor_params(overrides))` through `.expect("built deployment identity constructor params are admitted")`
    * `[✅]`   No corruptions type and no invalidator: the constructor params are a typed 32-byte array, the one value the constructor refuses is an array the params builder's overrides carry, and the crate has no serialization dependency; no `DeploymentIdentity` overrides, invalidator, or mock function, since the type is built as a real instance and owns no free function
    * `[✅]`   Imports `DeploymentIdentity`, `DeploymentIdentityConstructorParams`, and `DEPLOYMENT_IDENTITY_LENGTH` from `super::interface`

  * `[✅]`   `crates/domain/src/deployment_identity/test.rs`
    * `[✅]`   Module-level `#![allow(clippy::expect_used)]`
    * `[✅]`   Assertions on the held bytes read the `bytes` field, which the `pub(super)` visibility admits to this child module
    * `[✅]`   `try_new_rejects_the_all_zero_identity`
      * `[✅]`   Contract: every byte of `params.bytes` is `0` → `Err(DeploymentIdentityTryNewErrorReturn::AllZero)`
      * `[✅]`   Arrange: `build_deployment_identity_constructor_params` overriding the bytes with the array of `DEPLOYMENT_IDENTITY_LENGTH` zero bytes, the value an unassigned `bytes32` slot reads as; the builder's default array is nonzero, so the override establishes the refused value
      * `[✅]`   Act: `DeploymentIdentity::try_new(params)`
      * `[✅]`   Assert: the result equals `Err(DeploymentIdentityTryNewErrorReturn::AllZero)` by `assert_eq!`
    * `[✅]`   `try_new_admits_an_identity_whose_only_nonzero_byte_is_the_last`
      * `[✅]`   Contract: some byte of `params.bytes` is nonzero → `Ok(DeploymentIdentity { bytes })`
      * `[✅]`   Arrange: `build_deployment_identity_constructor_params` overriding the bytes with an array of zeros whose last offset holds `0x01`, so a check of the first byte alone fails
      * `[✅]`   Act: `DeploymentIdentity::try_new(params)`
      * `[✅]`   Assert: the result is `Ok`; the identity's `bytes` field equals the array literal of zeros with `0x01` at the last offset
    * `[✅]`   `try_new_admits_an_identity_whose_only_nonzero_byte_is_the_first`
      * `[✅]`   Contract: some byte of `params.bytes` is nonzero → `Ok(DeploymentIdentity { bytes })`
      * `[✅]`   Arrange: `build_deployment_identity_constructor_params` overriding the bytes with an array of zeros whose first offset holds `0xFF`, so a check of the last byte alone fails
      * `[✅]`   Act: `DeploymentIdentity::try_new(params)`
      * `[✅]`   Assert: the result is `Ok`; the identity's `bytes` field equals the array literal of zeros with `0xFF` at the first offset
    * `[✅]`   `try_new_admits_an_identity_whose_only_nonzero_byte_is_interior`
      * `[✅]`   Contract: some byte of `params.bytes` is nonzero → `Ok(DeploymentIdentity { bytes })`
      * `[✅]`   Arrange: `build_deployment_identity_constructor_params` overriding the bytes with an array of zeros whose offset 16 holds `0x80`, so a check of the first and last bytes alone fails
      * `[✅]`   Act: `DeploymentIdentity::try_new(params)`
      * `[✅]`   Assert: the result is `Ok`; the identity's `bytes` field equals the array literal of zeros with `0x80` at offset 16
    * `[✅]`   `as_bytes_returns_the_held_bytes`
      * `[✅]`   Contract: `DeploymentIdentity::as_bytes(&self) -> &[u8; DEPLOYMENT_IDENTITY_LENGTH]` → a shared reference to the held array
      * `[✅]`   Arrange: `build_deployment_identity` overriding the bytes with the array of `0x01` through `0x20` in ascending order, each offset holding a value different from every other offset, so a reversed, rotated, or partial copy fails
      * `[✅]`   Act: `identity.as_bytes()`
      * `[✅]`   Assert: the returned array equals the array literal of `0x01` through `0x20` in ascending order by `assert_eq!`

  * `[✅]`   `construction`
    * `[✅]`   `DeploymentIdentity::try_new` is the only producer; no `Default`, `From`, or other constructor exists; a caller holding a decoded `bytes32` passes it as `DeploymentIdentityConstructorParams` and handles the refusal arm

  * `[✅]`   `crates/domain/src/deployment_identity/mod.rs`
    * `[✅]`   Module declarations: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, `pub mod provides;`, and `#[cfg(test)] mod test;`
    * `[✅]`   `impl DeploymentIdentity` with `pub fn try_new(params: DeploymentIdentityConstructorParams) -> DeploymentIdentityTryNewReturn` realizing the branches of the interaction spec, and `pub fn as_bytes(&self) -> &[u8; DEPLOYMENT_IDENTITY_LENGTH]` returning `&self.bytes`
    * `[✅]`   Imports `DeploymentIdentity`, `DeploymentIdentityConstructorParams`, `DeploymentIdentityTryNewErrorReturn`, `DeploymentIdentityTryNewReturn`, and `DEPLOYMENT_IDENTITY_LENGTH` from `interface`
    * `[✅]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[✅]`   `crates/domain/src/deployment_identity/provides.rs`
    * `[✅]`   `pub use super::interface::*;` and `#[cfg(any(test, feature = "mocks"))] pub use super::mock::*;`, nothing else, so a sibling module's test and mock reach this module's builders in the crate's own test build as well as under the `mocks` feature

  * `[✅]`   `directionality`
    * `[✅]`   `deployment_identity` depends on the standard library alone and on no other module of the crate; `domain` depends on no repository crate; later consumers reach it through `lib.rs`'s re-export of `deployment_identity::provides`; no cycle

  * `[✅]`   `requirements`
    * `[✅]`   `crates/domain/Cargo.toml` is unchanged, and `crates/domain/src/lib.rs` carries exactly the barrel stated above
    * `[✅]`   `cargo check --workspace --all-targets --all-features`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `cargo fmt --all --check` complete without error or warning
    * `[✅]`   `try_new_admits_an_identity_whose_only_nonzero_byte_is_the_last` and `try_new_admits_an_identity_whose_only_nonzero_byte_is_the_first` pass
    * `[✅]`   `try_new_admits_an_identity_whose_only_nonzero_byte_is_interior` passes
    * `[✅]`   `try_new_rejects_the_all_zero_identity` passes
    * `[✅]`   `as_bytes_returns_the_held_bytes` passes
    * `[✅]`   Code outside `crates/domain/src/deployment_identity` reading the `bytes` field fails to compile

* `[✅]`   `domain/suite_identifier` **Cryptographic suite identifier and version, the hash-card field that fixes a deployment's whole cryptographic composition, admitted only when both can name a registered suite**

  * `[✅]`   `objective`
    * `[✅]`   Problem: a deployment's hash-card names one immutable suite by its identifier and version, and that pair fixes the pairing, credential KEM, envelope, delivery proof, payload cipher, commitment scheme, KDF and hash-to-scalar mappings, delivery-statement version, and attempt-rule parameters every derivation and attempt resolves against, so a pair naming no suite must be refused before anything resolves from it (Adapter Composition; IC-09)
    * `[✅]`   Functional: one type holds the suite identifier and the suite version, each reachable only through a read accessor, and its only producer is a fallible constructor
    * `[✅]`   Functional: the identifier is exactly 32 bytes, the width of the Registry's `bytes32` suite key, so its length is a fact of the params type and never a runtime check
    * `[✅]`   Functional: the version is a `u16`, the width the hash-card gives its other version field, the delivery-statement version
    * `[✅]`   Functional: the constructor refuses an all-zero identifier and a zero version, each the value an unassigned storage slot reads as, so neither names a registered suite
    * `[✅]`   Functional: a refusal names the failed field, and the same input always yields the same refusal: the identifier is checked before the version
    * `[✅]`   Non-functional: the module depends on the standard library alone; the `domain` crate's dependencies are unchanged

  * `[✅]`   `role`
    * `[✅]`   Domain: an owned value type in the protocol and domain ring, the suite every later derivation context, hash-card, and compatibility statement names
    * `[✅]`   Does not resolve a suite to its adapters, decide compatibility, or hold a compatibility statement; the composition resolver and the release's compatibility statement do that from this value
    * `[✅]`   Does not assign or register suite identifiers or versions
    * `[✅]`   Does not decode a suite identifier or version from wire bytes; the encoding family decodes them into the typed values this constructor takes
    * `[✅]`   Does not create any other module of the `domain` crate
    * `[✅]`   Does not carry a commit

  * `[✅]`   `module`
    * `[✅]`   Bounded context: the `suite_identifier` module of the `domain` crate, holding `SuiteIdentifier`, the identifier's length, its constructor params, and its constructor's error and return types
    * `[✅]`   Adds the module to the existing `domain` crate at `crates/domain`; the crate's manifest is unchanged and its barrel gains this module's line
    * `[✅]`   Outside: the suite's composition, its registration, the compatibility statement, and the encoding of the pair on the wire

  * `[✅]`   `deps`
    * `[✅]`   `domain/secret` created the `domain` crate this module joins; this module imports nothing from `secret` or `deployment_identity`
    * `[✅]`   The standard library: `u8`, `u16`, arrays, and `Iterator::all`, through the prelude
    * `[✅]`   No external crate and no repository crate; `crates/domain/Cargo.toml` is unchanged; direction inward, `domain` is the innermost ring
    * `[✅]`   No reverse dependency; `workflows/sidecar/derivation_context` is this module's first consumer

  * `[✅]`   `context_slice`
    * `[✅]`   From the standard library: `<[u8]>::iter` and `Iterator::all`

  * `[✅]`   `crates/domain/src/lib.rs`
    * `[✅]`   The crate barrel reads `mod deployment_identity;`, `mod secret;`, `mod suite_identifier;`, `pub use deployment_identity::provides::*;`, `pub use secret::provides::*;`, and `pub use suite_identifier::provides::*;`, nothing else
    * `[✅]`   Until `suite_identifier/mod.rs` exists, `cargo check` reports the unresolved `mod suite_identifier`, which is the RED state for every element below that precedes the implementation

  * `[✅]`   `crates/domain/src/suite_identifier/interface.rs`
    * `[✅]`   `SUITE_IDENTIFIER_LENGTH`, a `pub const` of type `usize` with value `32`, the width of the Registry's `bytes32` suite key
    * `[✅]`   `SuiteIdentifier`, a struct with `#[derive(Clone, Debug, PartialEq, Eq)]` and the fields `pub(super) identifier: [u8; SUITE_IDENTIFIER_LENGTH]` and `pub(super) version: u16`, so only the `suite_identifier` module and its children reach the fields
    * `[✅]`   `SuiteIdentifierConstructorParams`, a struct with the fields `pub identifier: [u8; SUITE_IDENTIFIER_LENGTH]` and `pub version: u16`; no derives
    * `[✅]`   `SuiteIdentifierTryNewErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]` and the variants `AllZeroIdentifier` and `ZeroVersion`
    * `[✅]`   `SuiteIdentifierTryNewReturn`, the type alias `Result<SuiteIdentifier, SuiteIdentifierTryNewErrorReturn>`
    * `[✅]`   Imports nothing; declares nothing else

  * `[✅]`   `crates/domain/src/suite_identifier/interaction.spec.md`
    * `[✅]`   `SuiteIdentifier::try_new(params: SuiteIdentifierConstructorParams) -> SuiteIdentifierTryNewReturn`, all-zero identifier: condition every byte of `params.identifier` is `0`; decision `params.identifier.iter().all(…)` over the byte equal to `0`; dependency call none; outcome `Err(SuiteIdentifierTryNewErrorReturn::AllZeroIdentifier)`
    * `[✅]`   Zero version: condition the identifier passes and `params.version` is `0`; decision the equality check; dependency call none; outcome `Err(SuiteIdentifierTryNewErrorReturn::ZeroVersion)`
    * `[✅]`   Admitted: condition some byte of the identifier is nonzero and the version is nonzero; decision the same two checks; dependency call none; outcome `Ok(SuiteIdentifier { identifier, version })`, both moved from the params
    * `[✅]`   `SuiteIdentifier::identifier(&self) -> &[u8; SUITE_IDENTIFIER_LENGTH]`: one branch; outcome a shared reference to the held array, no copy, no side effect
    * `[✅]`   `SuiteIdentifier::version(&self) -> u16`: one branch; outcome the held version
    * `[✅]`   Ordering: the identifier's check precedes the version's; the same params always yield the same outcome
    * `[✅]`   Invariants: every `SuiteIdentifier` holds a 32-byte identifier, not all zero, and a nonzero version; its only producer is `try_new`

  * `[✅]`   `crates/domain/src/suite_identifier/mock.rs`
    * `[✅]`   Module-level `#![allow(dead_code, clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::too_many_arguments)]`
    * `[✅]`   `SuiteIdentifierConstructorParamsOverrides`, `#[derive(Default)]`, fields `pub identifier: Option<[u8; SUITE_IDENTIFIER_LENGTH]>` and `pub version: Option<u16>`
    * `[✅]`   `build_suite_identifier_constructor_params(overrides: SuiteIdentifierConstructorParamsOverrides) -> SuiteIdentifierConstructorParams`, the identifier defaulting to `[0x22; SUITE_IDENTIFIER_LENGTH]` and the version to `1`
    * `[✅]`   `build_suite_identifier(overrides: SuiteIdentifierConstructorParamsOverrides) -> SuiteIdentifier`, returning the real instance from `SuiteIdentifier::try_new(build_suite_identifier_constructor_params(overrides))` through `.expect("built suite identifier constructor params are admitted")`
    * `[✅]`   No corruptions type and no invalidator: the constructor params are a typed 32-byte array and a `u16`, every value the constructor refuses is one the params builder's overrides carry, and the crate has no serialization dependency; no `SuiteIdentifier` overrides, invalidator, or mock function, since the type is built as a real instance and owns no free function
    * `[✅]`   Imports `SuiteIdentifier`, `SuiteIdentifierConstructorParams`, and `SUITE_IDENTIFIER_LENGTH` from `super::interface`

  * `[✅]`   `crates/domain/src/suite_identifier/test.rs`
    * `[✅]`   Module-level `#![allow(clippy::expect_used)]`
    * `[✅]`   Assertions on the held identifier and the held version read the `identifier` and `version` fields, which the `pub(super)` visibility admits to this child module
    * `[✅]`   `try_new_rejects_an_all_zero_identifier`
      * `[✅]`   Contract: every byte of `params.identifier` is `0` → `Err(SuiteIdentifierTryNewErrorReturn::AllZeroIdentifier)`
      * `[✅]`   Arrange: `build_suite_identifier_constructor_params` overriding the identifier with the array of `SUITE_IDENTIFIER_LENGTH` zero bytes, the value an unassigned `bytes32` slot reads as; the version keeps its valid default, so the identifier is the only refusal the params can meet
      * `[✅]`   Act: `SuiteIdentifier::try_new(params)`
      * `[✅]`   Assert: the result equals `Err(SuiteIdentifierTryNewErrorReturn::AllZeroIdentifier)` by `assert_eq!`
    * `[✅]`   `try_new_rejects_a_zero_version`
      * `[✅]`   Contract: the identifier passes and `params.version` is `0` → `Err(SuiteIdentifierTryNewErrorReturn::ZeroVersion)`
      * `[✅]`   Arrange: `build_suite_identifier_constructor_params` overriding the version with `0`; the identifier keeps its valid nonzero default, so the version is the only refusal the params can meet
      * `[✅]`   Act: `SuiteIdentifier::try_new(params)`
      * `[✅]`   Assert: the result equals `Err(SuiteIdentifierTryNewErrorReturn::ZeroVersion)` by `assert_eq!`
    * `[✅]`   `try_new_reports_the_identifier_before_the_version`
      * `[✅]`   Contract: the identifier's check precedes the version's → an all-zero identifier with a zero version yields `Err(SuiteIdentifierTryNewErrorReturn::AllZeroIdentifier)`
      * `[✅]`   Arrange: `build_suite_identifier_constructor_params` overriding the identifier with the array of `SUITE_IDENTIFIER_LENGTH` zero bytes and the version with `0`, so a version check that runs ahead of the identifier check fails
      * `[✅]`   Act: `SuiteIdentifier::try_new(params)`
      * `[✅]`   Assert: the result equals `Err(SuiteIdentifierTryNewErrorReturn::AllZeroIdentifier)` by `assert_eq!`
    * `[✅]`   `try_new_admits_an_identifier_whose_only_nonzero_byte_is_the_last_with_version_one`
      * `[✅]`   Contract: some byte of the identifier is nonzero and the version is nonzero → `Ok(SuiteIdentifier { identifier, version })`
      * `[✅]`   Arrange: `build_suite_identifier_constructor_params` overriding the identifier with an array of zeros whose last offset holds `0x01` and the version with `1`, the smallest admitted version, so a check of the first identifier byte alone fails
      * `[✅]`   Act: `SuiteIdentifier::try_new(params)`
      * `[✅]`   Assert: the result is `Ok`; the suite identifier's `identifier` field equals the array literal of zeros with `0x01` at the last offset and its `version` field equals the literal `1`
    * `[✅]`   `try_new_admits_an_identifier_whose_only_nonzero_byte_is_the_first_with_the_largest_version`
      * `[✅]`   Contract: some byte of the identifier is nonzero and the version is nonzero → `Ok(SuiteIdentifier { identifier, version })`
      * `[✅]`   Arrange: `build_suite_identifier_constructor_params` overriding the identifier with an array of zeros whose first offset holds `0xFF` and the version with the largest `u16`, `0xFFFF`, so a check of the last identifier byte alone fails
      * `[✅]`   Act: `SuiteIdentifier::try_new(params)`
      * `[✅]`   Assert: the result is `Ok`; the suite identifier's `identifier` field equals the array literal of zeros with `0xFF` at the first offset and its `version` field equals the literal `0xFFFF`
    * `[✅]`   `try_new_admits_an_identifier_whose_only_nonzero_byte_is_interior_with_version_one`
      * `[✅]`   Contract: some byte of the identifier is nonzero and the version is nonzero → `Ok(SuiteIdentifier { identifier, version })`
      * `[✅]`   Arrange: `build_suite_identifier_constructor_params` overriding the identifier with an array of zeros whose offset 16 holds `0x80` and the version with `1`, so a check of the first and last identifier bytes alone fails
      * `[✅]`   Act: `SuiteIdentifier::try_new(params)`
      * `[✅]`   Assert: the result is `Ok`; the suite identifier's `identifier` field equals the array literal of zeros with `0x80` at offset 16 and its `version` field equals the literal `1`
    * `[✅]`   `identifier_returns_the_held_identifier`
      * `[✅]`   Contract: `SuiteIdentifier::identifier(&self) -> &[u8; SUITE_IDENTIFIER_LENGTH]` → a shared reference to the held array
      * `[✅]`   Arrange: `build_suite_identifier` overriding the identifier with the array of `0x01` through `0x20` in ascending order, each offset holding a value different from every other offset, so a reversed, rotated, or partial copy fails
      * `[✅]`   Act: `suite_identifier.identifier()`
      * `[✅]`   Assert: the returned array equals the array literal of `0x01` through `0x20` in ascending order by `assert_eq!`
    * `[✅]`   `version_returns_the_held_version`
      * `[✅]`   Contract: `SuiteIdentifier::version(&self) -> u16` → the held version
      * `[✅]`   Arrange: `build_suite_identifier` overriding the version with `0x0102`, a value whose two bytes differ, so a byte-swapped or truncated return fails
      * `[✅]`   Act: `suite_identifier.version()`
      * `[✅]`   Assert: the returned `u16` equals the literal `0x0102` by `assert_eq!`

  * `[✅]`   `construction`
    * `[✅]`   `SuiteIdentifier::try_new` is the only producer; no `Default`, `From`, or other constructor exists; a caller holding a decoded identifier and version passes them as `SuiteIdentifierConstructorParams` and handles the refusal arm

  * `[✅]`   `crates/domain/src/suite_identifier/mod.rs`
    * `[✅]`   Module declarations: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, `pub mod provides;`, and `#[cfg(test)] mod test;`
    * `[✅]`   `impl SuiteIdentifier` with `pub fn try_new(params: SuiteIdentifierConstructorParams) -> SuiteIdentifierTryNewReturn` realizing the branches and ordering of the interaction spec, `pub fn identifier(&self) -> &[u8; SUITE_IDENTIFIER_LENGTH]` returning `&self.identifier`, and `pub fn version(&self) -> u16` returning `self.version`
    * `[✅]`   Imports `SuiteIdentifier`, `SuiteIdentifierConstructorParams`, `SuiteIdentifierTryNewErrorReturn`, `SuiteIdentifierTryNewReturn`, and `SUITE_IDENTIFIER_LENGTH` from `interface`
    * `[✅]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[✅]`   `crates/domain/src/suite_identifier/provides.rs`
    * `[✅]`   `pub use super::interface::*;` and `#[cfg(any(test, feature = "mocks"))] pub use super::mock::*;`, nothing else, so a sibling module's test and mock reach this module's builders in the crate's own test build as well as under the `mocks` feature

  * `[✅]`   `directionality`
    * `[✅]`   `suite_identifier` depends on the standard library alone and on no other module of the crate; `domain` depends on no repository crate; later consumers reach it through `lib.rs`'s re-export of `suite_identifier::provides`; no cycle

  * `[✅]`   `requirements`
    * `[✅]`   `crates/domain/Cargo.toml` is unchanged, and `crates/domain/src/lib.rs` carries exactly the barrel stated above
    * `[✅]`   `cargo check --workspace --all-targets --all-features`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `cargo fmt --all --check` complete without error or warning
    * `[✅]`   `try_new_admits_an_identifier_whose_only_nonzero_byte_is_the_last_with_version_one` and `try_new_admits_an_identifier_whose_only_nonzero_byte_is_the_first_with_the_largest_version` pass
    * `[✅]`   `try_new_admits_an_identifier_whose_only_nonzero_byte_is_interior_with_version_one` passes
    * `[✅]`   `try_new_rejects_an_all_zero_identifier`, `try_new_rejects_a_zero_version`, and `try_new_reports_the_identifier_before_the_version` pass
    * `[✅]`   `identifier_returns_the_held_identifier` and `version_returns_the_held_version` pass
    * `[✅]`   Code outside `crates/domain/src/suite_identifier` reading the `identifier` or `version` field fails to compile

* `[✅]`   `domain/parameter_set_identifier` **Parameter-set identifier, the 32 bytes of the Registry's `bytes32` parameter-set key, admitted only when it can name a registered set**

  * `[✅]`   `objective`
    * `[✅]`   Problem: every capsule, sidecar, wrapping key, and entitlement is bound to one parameter set by the identifier the Registry keys it under, the `bytes32` that the parameter-set, entitlement, and escrow records and the sidecar list of a hash-card carry, so a value naming no set must be refused before anything is derived from it (CR-11; LC-09)
    * `[✅]`   Functional: one type holds the parameter-set identifier's 32 bytes, reachable only through a read accessor, and its only producer is a fallible constructor
    * `[✅]`   Functional: the constructor takes exactly 32 bytes, so the length is a fact of the params type and never a runtime check
    * `[✅]`   Functional: the constructor refuses the all-zero value, which is what an unassigned `bytes32` storage slot reads as, so it names no set the Registry holds
    * `[✅]`   Functional: every other 32-byte value is admitted and read back unchanged
    * `[✅]`   Non-functional: the module depends on the standard library alone; the `domain` crate's dependencies are unchanged

  * `[✅]`   `role`
    * `[✅]`   Domain: an owned value type in the protocol and domain ring, the parameter set every later derivation context, sidecar entry, and entitlement names
    * `[✅]`   Does not hold, generate, or register a parameter set's public elements or its master scalar; the credential KEM family owns the parameter set and the Registry records it
    * `[✅]`   Does not track whether a set is live or retired; the Registry holds liveness
    * `[✅]`   Does not decode a parameter-set identifier from wire bytes of unknown length; the encoding family decodes a `bytes32` into the 32 bytes this constructor takes
    * `[✅]`   Does not create any other module of the `domain` crate
    * `[✅]`   Does not carry a commit

  * `[✅]`   `module`
    * `[✅]`   Bounded context: the `parameter_set_identifier` module of the `domain` crate, holding `ParameterSetIdentifier`, its length, its constructor params, and its constructor's error and return types
    * `[✅]`   Adds the module to the existing `domain` crate at `crates/domain`; the crate's manifest is unchanged and its barrel gains this module's line
    * `[✅]`   Outside: the parameter set itself, its registration and liveness, and the encoding of the identifier on the wire

  * `[✅]`   `deps`
    * `[✅]`   `domain/secret` created the `domain` crate this module joins; this module imports nothing from `secret`, `deployment_identity`, or `suite_identifier`
    * `[✅]`   The standard library: `u8`, arrays, and `Iterator::all`, through the prelude
    * `[✅]`   No external crate and no repository crate; `crates/domain/Cargo.toml` is unchanged; direction inward, `domain` is the innermost ring
    * `[✅]`   No reverse dependency; `workflows/sidecar/derivation_context` is this module's first consumer

  * `[✅]`   `context_slice`
    * `[✅]`   From the standard library: `<[u8]>::iter` and `Iterator::all`

  * `[✅]`   `crates/domain/src/lib.rs`
    * `[✅]`   The crate barrel reads `mod deployment_identity;`, `mod parameter_set_identifier;`, `mod secret;`, `mod suite_identifier;`, `pub use deployment_identity::provides::*;`, `pub use parameter_set_identifier::provides::*;`, `pub use secret::provides::*;`, and `pub use suite_identifier::provides::*;`, nothing else
    * `[✅]`   Until `parameter_set_identifier/mod.rs` exists, `cargo check` reports the unresolved `mod parameter_set_identifier`, which is the RED state for every element below that precedes the implementation

  * `[✅]`   `crates/domain/src/parameter_set_identifier/interface.rs`
    * `[✅]`   `PARAMETER_SET_IDENTIFIER_LENGTH`, a `pub const` of type `usize` with value `32`, the width of the Registry's `bytes32` parameter-set key
    * `[✅]`   `ParameterSetIdentifier`, a struct with `#[derive(Clone, Debug, PartialEq, Eq)]` and the one field `pub(super) bytes: [u8; PARAMETER_SET_IDENTIFIER_LENGTH]`, so only the `parameter_set_identifier` module and its children reach the field
    * `[✅]`   `ParameterSetIdentifierConstructorParams`, a struct with the one field `pub bytes: [u8; PARAMETER_SET_IDENTIFIER_LENGTH]`; no derives
    * `[✅]`   `ParameterSetIdentifierTryNewErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]` and the one variant `AllZero`
    * `[✅]`   `ParameterSetIdentifierTryNewReturn`, the type alias `Result<ParameterSetIdentifier, ParameterSetIdentifierTryNewErrorReturn>`
    * `[✅]`   Imports nothing; declares nothing else

  * `[✅]`   `crates/domain/src/parameter_set_identifier/interaction.spec.md`
    * `[✅]`   `ParameterSetIdentifier::try_new(params: ParameterSetIdentifierConstructorParams) -> ParameterSetIdentifierTryNewReturn`, all zero: condition every byte of `params.bytes` is `0`; decision `params.bytes.iter().all(…)` over the byte equal to `0`; dependency call none; outcome `Err(ParameterSetIdentifierTryNewErrorReturn::AllZero)`
    * `[✅]`   Admitted: condition some byte of `params.bytes` is nonzero; decision the same check; dependency call none; outcome `Ok(ParameterSetIdentifier { bytes })`, the array moved from the params
    * `[✅]`   `ParameterSetIdentifier::as_bytes(&self) -> &[u8; PARAMETER_SET_IDENTIFIER_LENGTH]`: one branch; outcome a shared reference to the held array, no copy, no side effect
    * `[✅]`   Invariants: every `ParameterSetIdentifier` holds exactly 32 bytes, not all zero; its only producer is `try_new`

  * `[✅]`   `crates/domain/src/parameter_set_identifier/mock.rs`
    * `[✅]`   Module-level `#![allow(dead_code, clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::too_many_arguments)]`
    * `[✅]`   `ParameterSetIdentifierConstructorParamsOverrides`, `#[derive(Default)]`, one field `pub bytes: Option<[u8; PARAMETER_SET_IDENTIFIER_LENGTH]>`
    * `[✅]`   `build_parameter_set_identifier_constructor_params(overrides: ParameterSetIdentifierConstructorParamsOverrides) -> ParameterSetIdentifierConstructorParams`, the bytes defaulting to `[0x33; PARAMETER_SET_IDENTIFIER_LENGTH]`
    * `[✅]`   `build_parameter_set_identifier(overrides: ParameterSetIdentifierConstructorParamsOverrides) -> ParameterSetIdentifier`, returning the real instance from `ParameterSetIdentifier::try_new(build_parameter_set_identifier_constructor_params(overrides))` through `.expect("built parameter set identifier constructor params are admitted")`
    * `[✅]`   No corruptions type and no invalidator: the constructor params are a typed 32-byte array, the one value the constructor refuses is an array the params builder's overrides carry, and the crate has no serialization dependency; no `ParameterSetIdentifier` overrides, invalidator, or mock function, since the type is built as a real instance and owns no free function
    * `[✅]`   Imports `ParameterSetIdentifier`, `ParameterSetIdentifierConstructorParams`, and `PARAMETER_SET_IDENTIFIER_LENGTH` from `super::interface`

  * `[✅]`   `crates/domain/src/parameter_set_identifier/test.rs`
    * `[✅]`   Module-level `#![allow(clippy::expect_used)]`
    * `[✅]`   Assertions on the held bytes read the `bytes` field, which the `pub(super)` visibility admits to this child module
    * `[✅]`   `try_new_rejects_the_all_zero_identifier`
      * `[✅]`   Contract: every byte of `params.bytes` is `0` → `Err(ParameterSetIdentifierTryNewErrorReturn::AllZero)`
      * `[✅]`   Arrange: `build_parameter_set_identifier_constructor_params` overriding the bytes with the array of `PARAMETER_SET_IDENTIFIER_LENGTH` zero bytes, the value an unassigned `bytes32` slot reads as; the builder's default array is nonzero, so the override establishes the refused value
      * `[✅]`   Act: `ParameterSetIdentifier::try_new(params)`
      * `[✅]`   Assert: the result equals `Err(ParameterSetIdentifierTryNewErrorReturn::AllZero)` by `assert_eq!`
    * `[✅]`   `try_new_admits_an_identifier_whose_only_nonzero_byte_is_the_last`
      * `[✅]`   Contract: some byte of `params.bytes` is nonzero → `Ok(ParameterSetIdentifier { bytes })`
      * `[✅]`   Arrange: `build_parameter_set_identifier_constructor_params` overriding the bytes with an array of zeros whose last offset holds `0x01`, so a check of the first byte alone fails
      * `[✅]`   Act: `ParameterSetIdentifier::try_new(params)`
      * `[✅]`   Assert: the result is `Ok`; the parameter-set identifier's `bytes` field equals the array literal of zeros with `0x01` at the last offset
    * `[✅]`   `try_new_admits_an_identifier_whose_only_nonzero_byte_is_the_first`
      * `[✅]`   Contract: some byte of `params.bytes` is nonzero → `Ok(ParameterSetIdentifier { bytes })`
      * `[✅]`   Arrange: `build_parameter_set_identifier_constructor_params` overriding the bytes with an array of zeros whose first offset holds `0xFF`, so a check of the last byte alone fails
      * `[✅]`   Act: `ParameterSetIdentifier::try_new(params)`
      * `[✅]`   Assert: the result is `Ok`; the parameter-set identifier's `bytes` field equals the array literal of zeros with `0xFF` at the first offset
    * `[✅]`   `try_new_admits_an_identifier_whose_only_nonzero_byte_is_interior`
      * `[✅]`   Contract: some byte of `params.bytes` is nonzero → `Ok(ParameterSetIdentifier { bytes })`
      * `[✅]`   Arrange: `build_parameter_set_identifier_constructor_params` overriding the bytes with an array of zeros whose offset 16 holds `0x80`, so a check of the first and last bytes alone fails
      * `[✅]`   Act: `ParameterSetIdentifier::try_new(params)`
      * `[✅]`   Assert: the result is `Ok`; the parameter-set identifier's `bytes` field equals the array literal of zeros with `0x80` at offset 16
    * `[✅]`   `as_bytes_returns_the_held_bytes`
      * `[✅]`   Contract: `ParameterSetIdentifier::as_bytes(&self) -> &[u8; PARAMETER_SET_IDENTIFIER_LENGTH]` → a shared reference to the held array
      * `[✅]`   Arrange: `build_parameter_set_identifier` overriding the bytes with the array of `0x01` through `0x20` in ascending order, each offset holding a value different from every other offset, so a reversed, rotated, or partial copy fails
      * `[✅]`   Act: `parameter_set_identifier.as_bytes()`
      * `[✅]`   Assert: the returned array equals the array literal of `0x01` through `0x20` in ascending order by `assert_eq!`

  * `[✅]`   `construction`
    * `[✅]`   `ParameterSetIdentifier::try_new` is the only producer; no `Default`, `From`, or other constructor exists; a caller holding a decoded `bytes32` passes it as `ParameterSetIdentifierConstructorParams` and handles the refusal arm

  * `[✅]`   `crates/domain/src/parameter_set_identifier/mod.rs`
    * `[✅]`   Module declarations: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, `pub mod provides;`, and `#[cfg(test)] mod test;`
    * `[✅]`   `impl ParameterSetIdentifier` with `pub fn try_new(params: ParameterSetIdentifierConstructorParams) -> ParameterSetIdentifierTryNewReturn` realizing the branches of the interaction spec, and `pub fn as_bytes(&self) -> &[u8; PARAMETER_SET_IDENTIFIER_LENGTH]` returning `&self.bytes`
    * `[✅]`   Imports `ParameterSetIdentifier`, `ParameterSetIdentifierConstructorParams`, `ParameterSetIdentifierTryNewErrorReturn`, `ParameterSetIdentifierTryNewReturn`, and `PARAMETER_SET_IDENTIFIER_LENGTH` from `interface`
    * `[✅]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[✅]`   `crates/domain/src/parameter_set_identifier/provides.rs`
    * `[✅]`   `pub use super::interface::*;` and `#[cfg(any(test, feature = "mocks"))] pub use super::mock::*;`, nothing else, so a sibling module's test and mock reach this module's builders in the crate's own test build as well as under the `mocks` feature

  * `[✅]`   `directionality`
    * `[✅]`   `parameter_set_identifier` depends on the standard library alone and on no other module of the crate; `domain` depends on no repository crate; later consumers reach it through `lib.rs`'s re-export of `parameter_set_identifier::provides`; no cycle

  * `[✅]`   `requirements`
    * `[✅]`   `crates/domain/Cargo.toml` is unchanged, and `crates/domain/src/lib.rs` carries exactly the barrel stated above
    * `[✅]`   `cargo check --workspace --all-targets --all-features`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `cargo fmt --all --check` complete without error or warning
    * `[✅]`   `try_new_admits_an_identifier_whose_only_nonzero_byte_is_the_last` and `try_new_admits_an_identifier_whose_only_nonzero_byte_is_the_first` pass
    * `[✅]`   `try_new_admits_an_identifier_whose_only_nonzero_byte_is_interior` passes
    * `[✅]`   `try_new_rejects_the_all_zero_identifier` passes
    * `[✅]`   `as_bytes_returns_the_held_bytes` passes
    * `[✅]`   Code outside `crates/domain/src/parameter_set_identifier` reading the `bytes` field fails to compile

* `[✅]`   `domain/group_index` **Piece-group index, the position of one piece group within a deployment's continuous stream, held as a distinct type so no other integer stands in for it**

  * `[✅]`   `objective`
    * `[✅]`   Problem: every capsule's randomness, every piece-group key, and every wrapping key is domain-separated by the group it belongs to, so the group index enters each derivation context as its own type and cannot be confused with a piece index, an interval, a version, or a count (CR-05; CR-11; the nonce invariant of Deployment Cryptographic Setup)
    * `[✅]`   Functional: one type holds a piece-group index as a `u64`, reachable only through a read accessor, and its only producer is a constructor
    * `[✅]`   Functional: every `u64` is admitted; a piece group spans at least one cipher block and the MVP cipher addresses at most two to the sixty-four blocks, so every index a deployment can carry fits the type, and the index's bound against one deployment's group count is enforced where that deployment's geometry is held, by `workflows/sidecar/derivation_context`
    * `[✅]`   Functional: the admitted index is read back unchanged
    * `[✅]`   Non-functional: the module depends on the standard library alone; the `domain` crate's dependencies are unchanged

  * `[✅]`   `role`
    * `[✅]`   Domain: an owned value type in the protocol and domain ring, the group every later derivation context, sidecar entry, and decryption attempt names
    * `[✅]`   Does not know a deployment's piece size, piece-group size, extent, or group count, and does not bound the index against them; `domain/piece_geometry` holds the geometry and `workflows/sidecar/derivation_context` refuses an index outside its group count
    * `[✅]`   Does not compute an index from a byte offset or a piece index
    * `[✅]`   Does not create any other module of the `domain` crate
    * `[✅]`   Does not carry a commit

  * `[✅]`   `module`
    * `[✅]`   Bounded context: the `group_index` module of the `domain` crate, holding `GroupIndex`, its constructor params, and its constructor's return type
    * `[✅]`   Adds the module to the existing `domain` crate at `crates/domain`; the crate's manifest is unchanged and its barrel gains this module's line
    * `[✅]`   Outside: the deployment's geometry and group count, the mapping from offsets and pieces to groups, and the encoding of the index on the wire

  * `[✅]`   `deps`
    * `[✅]`   `domain/secret` created the `domain` crate this module joins; this module imports nothing from `secret`, `deployment_identity`, `suite_identifier`, or `parameter_set_identifier`
    * `[✅]`   `core::convert::Infallible`, standard library, the constructor's error arm
    * `[✅]`   No external crate and no repository crate; `crates/domain/Cargo.toml` is unchanged; direction inward, `domain` is the innermost ring
    * `[✅]`   No reverse dependency; `workflows/sidecar/derivation_context` is this module's first consumer

  * `[✅]`   `context_slice`
    * `[✅]`   From the standard library: `core::convert::Infallible` and `u64`; nothing else

  * `[✅]`   `crates/domain/src/lib.rs`
    * `[✅]`   The crate barrel reads `mod deployment_identity;`, `mod group_index;`, `mod parameter_set_identifier;`, `mod secret;`, `mod suite_identifier;`, `pub use deployment_identity::provides::*;`, `pub use group_index::provides::*;`, `pub use parameter_set_identifier::provides::*;`, `pub use secret::provides::*;`, and `pub use suite_identifier::provides::*;`, nothing else
    * `[✅]`   Until `group_index/mod.rs` exists, `cargo check` reports the unresolved `mod group_index`, which is the RED state for every element below that precedes the implementation

  * `[✅]`   `crates/domain/src/group_index/interface.rs`
    * `[✅]`   `GroupIndex`, a struct with `#[derive(Clone, Debug, PartialEq, Eq)]` and the one field `pub(super) value: u64`, so only the `group_index` module and its children reach the field
    * `[✅]`   `GroupIndexConstructorParams`, a struct with the one field `pub value: u64`; no derives
    * `[✅]`   `GroupIndexTryNewReturn`, the type alias `Result<GroupIndex, Infallible>`; the error arm is uninhabited because every `u64` is an index a deployment can carry
    * `[✅]`   Imports `core::convert::Infallible`; declares nothing else

  * `[✅]`   `crates/domain/src/group_index/interaction.spec.md`
    * `[✅]`   `GroupIndex::try_new(params: GroupIndexConstructorParams) -> GroupIndexTryNewReturn`: one branch; condition any params; decision none; dependency call none; outcome `Ok(GroupIndex { value })` holding `params.value`; the error arm has no branch
    * `[✅]`   `GroupIndex::value(&self) -> u64`: one branch; outcome the held index, no side effect
    * `[✅]`   Invariants: every `GroupIndex` holds one `u64`; its only producer is `try_new`; its bound against a deployment's group count is `workflows/sidecar/derivation_context`'s

  * `[✅]`   `crates/domain/src/group_index/mock.rs`
    * `[✅]`   Module-level `#![allow(dead_code, clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::too_many_arguments)]`
    * `[✅]`   `GroupIndexConstructorParamsOverrides`, `#[derive(Default)]`, one field `pub value: Option<u64>`
    * `[✅]`   `build_group_index_constructor_params(overrides: GroupIndexConstructorParamsOverrides) -> GroupIndexConstructorParams`, the value defaulting to `7`
    * `[✅]`   `build_group_index(overrides: GroupIndexConstructorParamsOverrides) -> GroupIndex`, returning the real instance from `GroupIndex::try_new(build_group_index_constructor_params(overrides))` through the irrefutable pattern `let Ok(index) = …;`
    * `[✅]`   No corruptions type and no invalidator: the constructor params are a typed `u64` and the constructor admits every value; no `GroupIndex` overrides, invalidator, or mock function, since the type is built as a real instance and owns no free function
    * `[✅]`   Imports `GroupIndex` and `GroupIndexConstructorParams` from `super::interface`

  * `[✅]`   `crates/domain/src/group_index/test.rs`
    * `[✅]`   The `Ok` arm is taken through the irrefutable pattern `let Ok(index) = …;`, the error arm being `Infallible`
    * `[✅]`   Assertions on the held index read the `value` field, which the `pub(super)` visibility admits to this child module
    * `[✅]`   `try_new_admits_the_lowest_index`
      * `[✅]`   Contract: any params → `Ok(GroupIndex { value })` holding `params.value`
      * `[✅]`   Arrange: `build_group_index_constructor_params` overriding the value with `0`, the lowest `u64` and not the builder's default, so an implementation that rewrites zero fails
      * `[✅]`   Act: `GroupIndex::try_new(params)`
      * `[✅]`   Assert: the index's `value` field equals the literal `0`
    * `[✅]`   `try_new_admits_the_highest_index`
      * `[✅]`   Contract: any params → `Ok(GroupIndex { value })` holding `params.value`
      * `[✅]`   Arrange: `build_group_index_constructor_params` overriding the value with `u64::MAX`, the highest `u64`, so an implementation that clamps or narrows the value fails
      * `[✅]`   Act: `GroupIndex::try_new(params)`
      * `[✅]`   Assert: the index's `value` field equals the literal `u64::MAX`
    * `[✅]`   `value_returns_the_held_index`
      * `[✅]`   Contract: `GroupIndex::value(&self) -> u64` → the held index
      * `[✅]`   Arrange: `build_group_index` overriding the value with `0x0102_0304_0506_0708`, a value whose eight bytes all differ, so a byte-swapped or truncated return fails
      * `[✅]`   Act: `index.value()`
      * `[✅]`   Assert: the returned `u64` equals the literal `0x0102_0304_0506_0708` by `assert_eq!`

  * `[✅]`   `construction`
    * `[✅]`   `GroupIndex::try_new` is the only producer; no `Default`, `From`, or other constructor exists; a caller holding a group's position passes it as `GroupIndexConstructorParams`, and the derivation context that composes it bounds it against the deployment's group count

  * `[✅]`   `crates/domain/src/group_index/mod.rs`
    * `[✅]`   Module declarations: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, `pub mod provides;`, and `#[cfg(test)] mod test;`
    * `[✅]`   `impl GroupIndex` with `pub fn try_new(params: GroupIndexConstructorParams) -> GroupIndexTryNewReturn` returning `Ok(GroupIndex { value: params.value })`, and `pub fn value(&self) -> u64` returning `self.value`
    * `[✅]`   Imports `GroupIndex`, `GroupIndexConstructorParams`, and `GroupIndexTryNewReturn` from `interface`
    * `[✅]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[✅]`   `crates/domain/src/group_index/provides.rs`
    * `[✅]`   `pub use super::interface::*;` and `#[cfg(any(test, feature = "mocks"))] pub use super::mock::*;`, nothing else, so a sibling module's test and mock reach this module's builders in the crate's own test build as well as under the `mocks` feature

  * `[✅]`   `directionality`
    * `[✅]`   `group_index` depends on the standard library alone and on no other module of the crate; `domain` depends on no repository crate; later consumers reach it through `lib.rs`'s re-export of `group_index::provides`; no cycle

  * `[✅]`   `requirements`
    * `[✅]`   `crates/domain/Cargo.toml` is unchanged, and `crates/domain/src/lib.rs` carries exactly the barrel stated above
    * `[✅]`   `cargo check --workspace --all-targets --all-features`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `cargo fmt --all --check` complete without error or warning
    * `[✅]`   `try_new_admits_the_lowest_index` and `try_new_admits_the_highest_index` pass, each proving the admitted index is read back unchanged
    * `[✅]`   `value_returns_the_held_index` passes
    * `[✅]`   Code outside `crates/domain/src/group_index` reading the `value` field fails to compile

* `[✅]`   `domain/piece_geometry` **Piece geometry, the piece size, piece-group size, and total extent a hash-card declares, admitted only when pieces align and groups are whole multiples of pieces, with the piece and group counts they imply**

  * `[✅]`   `objective`
    * `[✅]`   Problem: a hash-card's declared geometry drives every offset, counter, and key boundary, so it is untrusted until its sizes are mutually consistent, and a geometry that fails is refused before any piece-group key is derived (EC-01; CR-06; Manifest Bounds Validation)
    * `[✅]`   Functional: one type holds the piece size and piece-group size as `u32` and the total extent as `u64`, the widths the hash-card's `pieceSize`, `pieceGroupSize`, and `totalExtent` fields carry, each reachable only through a read accessor, and its only producer is a fallible constructor
    * `[✅]`   Functional: the constructor refuses a piece size that is not a power of two, zero included, and a piece size below 16 KiB
    * `[✅]`   Functional: the constructor refuses a zero piece-group size and a piece-group size that is not a whole multiple of the piece size, a group smaller than a piece included, so every group holds whole pieces
    * `[✅]`   Functional: the constructor refuses a zero total extent, so every admitted geometry has at least one piece and one group
    * `[✅]`   Functional: the admitted geometry reports its piece count and group count, each the total extent divided by the piece size or the piece-group size and rounded up, computed without overflow across the full width of the extent
    * `[✅]`   Functional: a refusal names the failed check and the values it failed on, and the same input always yields the same refusal: piece size, then piece-group size, then total extent
    * `[✅]`   Non-functional: the module depends on the standard library alone; the `domain` crate's dependencies are unchanged

  * `[✅]`   `role`
    * `[✅]`   Domain: an owned value type in the protocol and domain ring, the geometry every later derivation context, cipher call, and manifest gate reads
    * `[✅]`   Does not check the extent against a cipher's declared `maxAddressableBytes` or the piece size against a cipher's block size; both are declared by the resolved cipher, so the manifest gate checks them against that declaration; a power of two of at least 16 KiB is already a multiple of every power-of-two block size up to 16 KiB
    * `[✅]`   Does not bound a piece index or a group index; `workflows/sidecar/derivation_context` bounds the group index against this geometry's group count, and the transport bounds a piece index against its piece count
    * `[✅]`   Does not hold the IV, the counter layout, or the cipher identifier
    * `[✅]`   Does not choose the piece-group size; the hash-card declares it and the code reads it from the record
    * `[✅]`   Does not create any other module of the `domain` crate
    * `[✅]`   Does not carry a commit

  * `[✅]`   `module`
    * `[✅]`   Bounded context: the `piece_geometry` module of the `domain` crate, holding `PieceGeometry`, the minimum piece size, its constructor params, its constructor's error and return types, and the counts the geometry implies
    * `[✅]`   Adds the module to the existing `domain` crate at `crates/domain`; the crate's manifest is unchanged and its barrel gains this module's line
    * `[✅]`   Outside: every cipher's declared extent and block size, the offset and counter arithmetic, index ranges, and the encoding of the geometry on the wire

  * `[✅]`   `deps`
    * `[✅]`   `domain/secret` created the `domain` crate this module joins; this module imports nothing from any other module of the crate
    * `[✅]`   The standard library: `u32::is_power_of_two`, `u64::from` over `u32`, and `u64::div_ceil`, through the prelude
    * `[✅]`   No external crate and no repository crate; `crates/domain/Cargo.toml` is unchanged; direction inward, `domain` is the innermost ring
    * `[✅]`   No reverse dependency; `workflows/sidecar/derivation_context` is this module's first consumer

  * `[✅]`   `context_slice`
    * `[✅]`   From the standard library: `u32::is_power_of_two`, which is false for `0`; the `%` remainder over `u32`; `u64::from(u32)`, the lossless widening; and `u64::div_ceil`, rounding the quotient up

  * `[✅]`   `crates/domain/src/lib.rs`
    * `[✅]`   The crate barrel reads `mod deployment_identity;`, `mod group_index;`, `mod parameter_set_identifier;`, `mod piece_geometry;`, `mod secret;`, `mod suite_identifier;`, `pub use deployment_identity::provides::*;`, `pub use group_index::provides::*;`, `pub use parameter_set_identifier::provides::*;`, `pub use piece_geometry::provides::*;`, `pub use secret::provides::*;`, and `pub use suite_identifier::provides::*;`, nothing else
    * `[✅]`   Until `piece_geometry/mod.rs` exists, `cargo check` reports the unresolved `mod piece_geometry`, which is the RED state for every element below that precedes the implementation

  * `[✅]`   `crates/domain/src/piece_geometry/interface.rs`
    * `[✅]`   `MINIMUM_PIECE_SIZE`, a `pub const` of type `u32` with value `16384`, 16 KiB
    * `[✅]`   `PieceGeometry`, a struct with `#[derive(Clone, Debug, PartialEq, Eq)]` and the fields `pub(super) piece_size: u32`, `pub(super) piece_group_size: u32`, and `pub(super) total_extent: u64`, so only the `piece_geometry` module and its children reach the fields
    * `[✅]`   `PieceGeometryConstructorParams`, a struct with the fields `pub piece_size: u32`, `pub piece_group_size: u32`, and `pub total_extent: u64`; no derives
    * `[✅]`   `PieceGeometryTryNewErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]` and the variants `PieceSizeNotPowerOfTwo { piece_size: u32 }`, `PieceSizeBelowMinimum { piece_size: u32, minimum: u32 }`, `ZeroPieceGroupSize`, `PieceGroupSizeNotMultipleOfPieceSize { piece_group_size: u32, piece_size: u32 }`, and `ZeroTotalExtent`
    * `[✅]`   `PieceGeometryTryNewReturn`, the type alias `Result<PieceGeometry, PieceGeometryTryNewErrorReturn>`
    * `[✅]`   Imports nothing; declares nothing else

  * `[✅]`   `crates/domain/src/piece_geometry/interaction.spec.md`
    * `[✅]`   `PieceGeometry::try_new(params: PieceGeometryConstructorParams) -> PieceGeometryTryNewReturn`, piece size not a power of two: condition `!params.piece_size.is_power_of_two()`, zero included; decision the power-of-two check; dependency call none; outcome `Err(PieceGeometryTryNewErrorReturn::PieceSizeNotPowerOfTwo { piece_size })`
    * `[✅]`   Piece size below the minimum: condition the piece size is a power of two and `params.piece_size < MINIMUM_PIECE_SIZE`; decision the comparison; dependency call none; outcome `Err(PieceGeometryTryNewErrorReturn::PieceSizeBelowMinimum { piece_size, minimum: MINIMUM_PIECE_SIZE })`
    * `[✅]`   Zero piece-group size: condition the piece size passes and `params.piece_group_size == 0`; decision the equality check; dependency call none; outcome `Err(PieceGeometryTryNewErrorReturn::ZeroPieceGroupSize)`
    * `[✅]`   Piece-group size not a multiple of the piece size: condition the piece size passes, the piece-group size is nonzero, and `params.piece_group_size % params.piece_size != 0`; decision the remainder check; dependency call none; outcome `Err(PieceGeometryTryNewErrorReturn::PieceGroupSizeNotMultipleOfPieceSize { piece_group_size, piece_size })`
    * `[✅]`   Zero total extent: condition both sizes pass and `params.total_extent == 0`; decision the equality check; dependency call none; outcome `Err(PieceGeometryTryNewErrorReturn::ZeroTotalExtent)`
    * `[✅]`   Admitted: condition every check passes; decision none further; dependency call none; outcome `Ok(PieceGeometry { piece_size, piece_group_size, total_extent })`, each moved from the params
    * `[✅]`   `PieceGeometry::piece_size(&self) -> u32`, `PieceGeometry::piece_group_size(&self) -> u32`, and `PieceGeometry::total_extent(&self) -> u64`: one branch each; outcome the held value
    * `[✅]`   `PieceGeometry::piece_count(&self) -> u64`: one branch; outcome `self.total_extent.div_ceil(u64::from(self.piece_size))`, nonzero because both operands are
    * `[✅]`   `PieceGeometry::group_count(&self) -> u64`: one branch; outcome `self.total_extent.div_ceil(u64::from(self.piece_group_size))`, nonzero because both operands are
    * `[✅]`   Ordering: the piece size's checks precede the piece-group size's, which precede the total extent's; the same params always yield the same outcome
    * `[✅]`   Invariants: every `PieceGeometry` holds a power-of-two piece size of at least 16 KiB, a nonzero piece-group size that is a whole multiple of it, and a nonzero total extent; its only producer is `try_new`

  * `[✅]`   `crates/domain/src/piece_geometry/mock.rs`
    * `[✅]`   Module-level `#![allow(dead_code, clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::too_many_arguments)]`
    * `[✅]`   `PieceGeometryConstructorParamsOverrides`, `#[derive(Default)]`, fields `pub piece_size: Option<u32>`, `pub piece_group_size: Option<u32>`, and `pub total_extent: Option<u64>`
    * `[✅]`   `build_piece_geometry_constructor_params(overrides: PieceGeometryConstructorParamsOverrides) -> PieceGeometryConstructorParams`, the piece size defaulting to `MINIMUM_PIECE_SIZE`, the piece-group size to `16384`, the configured default of 16 KiB, and the total extent to `1048576`
    * `[✅]`   `build_piece_geometry(overrides: PieceGeometryConstructorParamsOverrides) -> PieceGeometry`, returning the real instance from `PieceGeometry::try_new(build_piece_geometry_constructor_params(overrides))` through `.expect("built piece geometry constructor params are admitted")`
    * `[✅]`   No corruptions type and no invalidator: the constructor params are typed integers, every geometry the constructor refuses is a value the params builder's overrides carry, and the crate has no serialization dependency; no `PieceGeometry` overrides, invalidator, or mock function, since the type is built as a real instance and owns no free function
    * `[✅]`   Imports `PieceGeometry`, `PieceGeometryConstructorParams`, and `MINIMUM_PIECE_SIZE` from `super::interface`

  * `[✅]`   `crates/domain/src/piece_geometry/test.rs`
    * `[✅]`   Module-level `#![allow(clippy::expect_used)]`
    * `[✅]`   Assertions on the held sizes and the held extent read the `piece_size`, `piece_group_size`, and `total_extent` fields, which the `pub(super)` visibility admits to this child module
    * `[✅]`   `try_new_rejects_a_piece_size_that_is_not_a_power_of_two`
      * `[✅]`   Contract: `!params.piece_size.is_power_of_two()` → `Err(PieceGeometryTryNewErrorReturn::PieceSizeNotPowerOfTwo { piece_size })`
      * `[✅]`   Arrange: `build_piece_geometry_constructor_params` overriding the piece size with `49152` and the piece-group size with `49152`; `49152` is a multiple of the minimum piece size and above it but not a power of two, so a check of the minimum or of a multiple of it fails, and the sizes agree, so the power-of-two check is the only refusal the params can meet; the extent keeps its valid default
      * `[✅]`   Act: `PieceGeometry::try_new(params)`
      * `[✅]`   Assert: the result equals `Err(PieceGeometryTryNewErrorReturn::PieceSizeNotPowerOfTwo { piece_size: 49152 })` by `assert_eq!`
    * `[✅]`   `try_new_rejects_a_zero_piece_size`
      * `[✅]`   Contract: `!params.piece_size.is_power_of_two()`, zero included → `Err(PieceGeometryTryNewErrorReturn::PieceSizeNotPowerOfTwo { piece_size })`
      * `[✅]`   Arrange: `build_piece_geometry_constructor_params` overriding the piece size with `0`; the piece-group size and the extent keep their valid defaults, so a remainder taken by the piece size before the piece size is checked fails
      * `[✅]`   Act: `PieceGeometry::try_new(params)`
      * `[✅]`   Assert: the result equals `Err(PieceGeometryTryNewErrorReturn::PieceSizeNotPowerOfTwo { piece_size: 0 })` by `assert_eq!`
    * `[✅]`   `try_new_rejects_a_piece_size_below_the_minimum`
      * `[✅]`   Contract: the piece size is a power of two and `params.piece_size < MINIMUM_PIECE_SIZE` → `Err(PieceGeometryTryNewErrorReturn::PieceSizeBelowMinimum { piece_size, minimum: MINIMUM_PIECE_SIZE })`
      * `[✅]`   Arrange: `build_piece_geometry_constructor_params` overriding the piece size with `8192`, the power of two just below 16 KiB, and the piece-group size with `8192`, so the sizes agree and the minimum is the only refusal the params can meet; the extent keeps its valid default
      * `[✅]`   Act: `PieceGeometry::try_new(params)`
      * `[✅]`   Assert: the result equals `Err(PieceGeometryTryNewErrorReturn::PieceSizeBelowMinimum { piece_size: 8192, minimum: 16384 })` by `assert_eq!`, the minimum being the 16 KiB the objective states
    * `[✅]`   `try_new_rejects_a_zero_piece_group_size`
      * `[✅]`   Contract: the piece size passes and `params.piece_group_size == 0` → `Err(PieceGeometryTryNewErrorReturn::ZeroPieceGroupSize)`
      * `[✅]`   Arrange: `build_piece_geometry_constructor_params` overriding the piece-group size with `0`; the piece size and the extent keep their valid defaults, so a remainder check alone, which reads zero as a multiple, fails
      * `[✅]`   Act: `PieceGeometry::try_new(params)`
      * `[✅]`   Assert: the result equals `Err(PieceGeometryTryNewErrorReturn::ZeroPieceGroupSize)` by `assert_eq!`
    * `[✅]`   `try_new_rejects_a_piece_group_size_that_is_not_a_multiple_of_the_piece_size`
      * `[✅]`   Contract: the piece size passes, the piece-group size is nonzero, and `params.piece_group_size % params.piece_size != 0` → `Err(PieceGeometryTryNewErrorReturn::PieceGroupSizeNotMultipleOfPieceSize { piece_group_size, piece_size })`
      * `[✅]`   Arrange: `build_piece_geometry_constructor_params` overriding the piece size with `16384` and the piece-group size with `24576`, a group larger than a piece and one and a half pieces long; the extent keeps its valid default
      * `[✅]`   Act: `PieceGeometry::try_new(params)`
      * `[✅]`   Assert: the result equals `Err(PieceGeometryTryNewErrorReturn::PieceGroupSizeNotMultipleOfPieceSize { piece_group_size: 24576, piece_size: 16384 })` by `assert_eq!`
    * `[✅]`   `try_new_rejects_a_piece_group_smaller_than_a_piece`
      * `[✅]`   Contract: the piece size passes, the piece-group size is nonzero, and `params.piece_group_size % params.piece_size != 0` → `Err(PieceGeometryTryNewErrorReturn::PieceGroupSizeNotMultipleOfPieceSize { piece_group_size, piece_size })`
      * `[✅]`   Arrange: `build_piece_geometry_constructor_params` overriding the piece size with `32768` and the piece-group size with `16384`, a group smaller than a piece, so a remainder taken with its operands exchanged, which reads zero, fails; the extent keeps its valid default
      * `[✅]`   Act: `PieceGeometry::try_new(params)`
      * `[✅]`   Assert: the result equals `Err(PieceGeometryTryNewErrorReturn::PieceGroupSizeNotMultipleOfPieceSize { piece_group_size: 16384, piece_size: 32768 })` by `assert_eq!`
    * `[✅]`   `try_new_rejects_a_zero_total_extent`
      * `[✅]`   Contract: both sizes pass and `params.total_extent == 0` → `Err(PieceGeometryTryNewErrorReturn::ZeroTotalExtent)`
      * `[✅]`   Arrange: `build_piece_geometry_constructor_params` overriding the extent with `0`; the sizes keep their valid defaults, so the extent is the only refusal the params can meet
      * `[✅]`   Act: `PieceGeometry::try_new(params)`
      * `[✅]`   Assert: the result equals `Err(PieceGeometryTryNewErrorReturn::ZeroTotalExtent)` by `assert_eq!`
    * `[✅]`   `try_new_reports_the_piece_size_before_the_piece_group_size`
      * `[✅]`   Contract: the piece size's checks precede the piece-group size's → a piece size that is not a power of two with a zero piece-group size yields `Err(PieceGeometryTryNewErrorReturn::PieceSizeNotPowerOfTwo { piece_size })`
      * `[✅]`   Arrange: `build_piece_geometry_constructor_params` overriding the piece size with `49152` and the piece-group size with `0`; the extent keeps its valid default, so a piece-group size check that runs ahead of the piece size check fails
      * `[✅]`   Act: `PieceGeometry::try_new(params)`
      * `[✅]`   Assert: the result equals `Err(PieceGeometryTryNewErrorReturn::PieceSizeNotPowerOfTwo { piece_size: 49152 })` by `assert_eq!`
    * `[✅]`   `try_new_reports_the_piece_group_size_before_the_total_extent`
      * `[✅]`   Contract: the piece-group size's checks precede the total extent's → a zero piece-group size with a zero extent yields `Err(PieceGeometryTryNewErrorReturn::ZeroPieceGroupSize)`
      * `[✅]`   Arrange: `build_piece_geometry_constructor_params` overriding the piece-group size with `0` and the extent with `0`; the piece size keeps its valid default, so an extent check that runs ahead of the piece-group size check fails
      * `[✅]`   Act: `PieceGeometry::try_new(params)`
      * `[✅]`   Assert: the result equals `Err(PieceGeometryTryNewErrorReturn::ZeroPieceGroupSize)` by `assert_eq!`
    * `[✅]`   `try_new_reports_the_piece_size_before_the_total_extent`
      * `[✅]`   Contract: the piece size's checks precede the total extent's → a piece size that is not a power of two with a zero extent yields `Err(PieceGeometryTryNewErrorReturn::PieceSizeNotPowerOfTwo { piece_size })`
      * `[✅]`   Arrange: `build_piece_geometry_constructor_params` overriding the piece size with `49152`, the piece-group size with `49152`, and the extent with `0`, so the sizes agree and an extent check that runs ahead of the piece size check fails
      * `[✅]`   Act: `PieceGeometry::try_new(params)`
      * `[✅]`   Assert: the result equals `Err(PieceGeometryTryNewErrorReturn::PieceSizeNotPowerOfTwo { piece_size: 49152 })` by `assert_eq!`
    * `[✅]`   `try_new_admits_the_smallest_geometry`
      * `[✅]`   Contract: every check passes → `Ok(PieceGeometry { piece_size, piece_group_size, total_extent })`
      * `[✅]`   Arrange: `build_piece_geometry_constructor_params` overriding the piece size with `16384`, the minimum, the piece-group size with `16384`, one piece, and the extent with `1`, the smallest nonzero extent, so a minimum check that refuses the minimum itself, a group check that requires more than one piece, and an extent check that requires more than one byte each fail
      * `[✅]`   Act: `PieceGeometry::try_new(params)`
      * `[✅]`   Assert: the result is `Ok`; the geometry's `piece_size` field equals the literal `16384`, its `piece_group_size` field equals the literal `16384`, and its `total_extent` field equals the literal `1`
    * `[✅]`   `try_new_admits_a_piece_group_size_that_is_an_odd_multiple_of_the_piece_size`
      * `[✅]`   Contract: every check passes → `Ok(PieceGeometry { piece_size, piece_group_size, total_extent })`
      * `[✅]`   Arrange: `build_piece_geometry_constructor_params` overriding the piece size with `16384` and the piece-group size with `49152`, an odd multiple of the piece size and not a power of two, so a group check that requires a power of two fails; the extent keeps its valid default
      * `[✅]`   Act: `PieceGeometry::try_new(params)`
      * `[✅]`   Assert: the result is `Ok`; the geometry's `piece_size` field equals the literal `16384` and its `piece_group_size` field equals the literal `49152`
    * `[✅]`   `piece_size_returns_the_held_piece_size`
      * `[✅]`   Contract: `PieceGeometry::piece_size(&self) -> u32` → the held piece size
      * `[✅]`   Arrange: `build_piece_geometry` overriding the piece size with `32768`, the piece-group size with `131072`, and the extent with `0x0102_0304_0506_0708`, three values that differ from one another, so an accessor that returns another field fails
      * `[✅]`   Act: `geometry.piece_size()`
      * `[✅]`   Assert: the returned `u32` equals the literal `32768` by `assert_eq!`
    * `[✅]`   `piece_group_size_returns_the_held_piece_group_size`
      * `[✅]`   Contract: `PieceGeometry::piece_group_size(&self) -> u32` → the held piece-group size
      * `[✅]`   Arrange: `build_piece_geometry` overriding the piece size with `32768`, the piece-group size with `131072`, and the extent with `0x0102_0304_0506_0708`, three values that differ from one another, so an accessor that returns another field fails
      * `[✅]`   Act: `geometry.piece_group_size()`
      * `[✅]`   Assert: the returned `u32` equals the literal `131072` by `assert_eq!`
    * `[✅]`   `total_extent_returns_the_held_total_extent`
      * `[✅]`   Contract: `PieceGeometry::total_extent(&self) -> u64` → the held total extent
      * `[✅]`   Arrange: `build_piece_geometry` overriding the piece size with `32768`, the piece-group size with `131072`, and the extent with `0x0102_0304_0506_0708`, a value whose eight bytes all differ, so a narrowed or byte-swapped return fails
      * `[✅]`   Act: `geometry.total_extent()`
      * `[✅]`   Assert: the returned `u64` equals the literal `0x0102_0304_0506_0708` by `assert_eq!`
    * `[✅]`   `piece_count_rounds_up_a_partial_piece`
      * `[✅]`   Contract: `PieceGeometry::piece_count(&self) -> u64` → `self.total_extent.div_ceil(u64::from(self.piece_size))`
      * `[✅]`   Arrange: `build_piece_geometry` overriding the piece size with `16384`, the piece-group size with `65536`, and the extent with `16385`, one byte past a whole piece, so a count that rounds down, and a count taken by the piece-group size, fail
      * `[✅]`   Act: `geometry.piece_count()`
      * `[✅]`   Assert: the returned `u64` equals the literal `2`, the pieces `16385` bytes need at `16384` bytes each, by `assert_eq!`
    * `[✅]`   `group_count_rounds_up_a_partial_group`
      * `[✅]`   Contract: `PieceGeometry::group_count(&self) -> u64` → `self.total_extent.div_ceil(u64::from(self.piece_group_size))`
      * `[✅]`   Arrange: `build_piece_geometry` overriding the piece size with `16384`, the piece-group size with `65536`, and the extent with `65537`, one byte past a whole group, so a count that rounds down, and a count taken by the piece size, fail
      * `[✅]`   Act: `geometry.group_count()`
      * `[✅]`   Assert: the returned `u64` equals the literal `2`, the groups `65537` bytes need at `65536` bytes each, by `assert_eq!`
    * `[✅]`   `piece_count_does_not_round_up_an_exact_extent`
      * `[✅]`   Contract: `PieceGeometry::piece_count(&self) -> u64` → `self.total_extent.div_ceil(u64::from(self.piece_size))`
      * `[✅]`   Arrange: `build_piece_geometry` overriding the piece size with `16384`, the piece-group size with `65536`, and the extent with `49152`, a whole number of pieces, so a count that adds one past the quotient, and a count taken by the piece-group size, fail
      * `[✅]`   Act: `geometry.piece_count()`
      * `[✅]`   Assert: the returned `u64` equals the literal `3`, the pieces `49152` bytes fill at `16384` bytes each, by `assert_eq!`
    * `[✅]`   `group_count_does_not_round_up_an_exact_extent`
      * `[✅]`   Contract: `PieceGeometry::group_count(&self) -> u64` → `self.total_extent.div_ceil(u64::from(self.piece_group_size))`
      * `[✅]`   Arrange: `build_piece_geometry` overriding the piece size with `16384`, the piece-group size with `65536`, and the extent with `131072`, a whole number of groups, so a count that adds one past the quotient, and a count taken by the piece size, fail
      * `[✅]`   Act: `geometry.group_count()`
      * `[✅]`   Assert: the returned `u64` equals the literal `2`, the groups `131072` bytes fill at `65536` bytes each, by `assert_eq!`
    * `[✅]`   `piece_count_holds_at_the_largest_extent`
      * `[✅]`   Contract: `PieceGeometry::piece_count(&self) -> u64` → `self.total_extent.div_ceil(u64::from(self.piece_size))`
      * `[✅]`   Arrange: `build_piece_geometry` overriding the piece size with `16384`, the piece-group size with `65536`, and the extent with `u64::MAX`, so a count that adds the divisor before dividing overflows
      * `[✅]`   Act: `geometry.piece_count()`
      * `[✅]`   Assert: the returned `u64` equals the literal `1 << 50`, the quotient of `2^64` by the piece size `2^14`, which rounding up `u64::MAX`, one below `2^64`, reaches, by `assert_eq!`
    * `[✅]`   `group_count_holds_at_the_largest_extent`
      * `[✅]`   Contract: `PieceGeometry::group_count(&self) -> u64` → `self.total_extent.div_ceil(u64::from(self.piece_group_size))`
      * `[✅]`   Arrange: `build_piece_geometry` overriding the piece size with `16384`, the piece-group size with `65536`, and the extent with `u64::MAX`, so a count that adds the divisor before dividing overflows
      * `[✅]`   Act: `geometry.group_count()`
      * `[✅]`   Assert: the returned `u64` equals the literal `1 << 48`, the quotient of `2^64` by the piece-group size `2^16`, which rounding up `u64::MAX`, one below `2^64`, reaches, by `assert_eq!`

  * `[✅]`   `construction`
    * `[✅]`   `PieceGeometry::try_new` is the only producer; no `Default`, `From`, or other constructor exists; a caller holding a hash-card's decoded `pieceSize`, `pieceGroupSize`, and `totalExtent` passes them as `PieceGeometryConstructorParams` and handles the refusal arm

  * `[✅]`   `crates/domain/src/piece_geometry/mod.rs`
    * `[✅]`   Module declarations: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, `pub mod provides;`, and `#[cfg(test)] mod test;`
    * `[✅]`   `impl PieceGeometry` with `pub fn try_new(params: PieceGeometryConstructorParams) -> PieceGeometryTryNewReturn` realizing the branches and ordering of the interaction spec, and the accessors `pub fn piece_size(&self) -> u32`, `pub fn piece_group_size(&self) -> u32`, `pub fn total_extent(&self) -> u64`, `pub fn piece_count(&self) -> u64`, and `pub fn group_count(&self) -> u64` as the interaction spec states
    * `[✅]`   Imports `PieceGeometry`, `PieceGeometryConstructorParams`, `PieceGeometryTryNewErrorReturn`, `PieceGeometryTryNewReturn`, and `MINIMUM_PIECE_SIZE` from `interface`
    * `[✅]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[✅]`   `crates/domain/src/piece_geometry/provides.rs`
    * `[✅]`   `pub use super::interface::*;` and `#[cfg(any(test, feature = "mocks"))] pub use super::mock::*;`, nothing else, so a sibling module's test and mock reach this module's builders in the crate's own test build as well as under the `mocks` feature

  * `[✅]`   `directionality`
    * `[✅]`   `piece_geometry` depends on the standard library alone and on no other module of the crate; `domain` depends on no repository crate; later consumers reach it through `lib.rs`'s re-export of `piece_geometry::provides`; no cycle

  * `[✅]`   `requirements`
    * `[✅]`   `crates/domain/Cargo.toml` is unchanged, and `crates/domain/src/lib.rs` carries exactly the barrel stated above
    * `[✅]`   `cargo check --workspace --all-targets --all-features`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `cargo fmt --all --check` complete without error or warning
    * `[✅]`   `try_new_admits_the_smallest_geometry`, `piece_count_rounds_up_a_partial_piece`, `group_count_rounds_up_a_partial_group`, `piece_count_does_not_round_up_an_exact_extent`, `group_count_does_not_round_up_an_exact_extent`, `piece_count_holds_at_the_largest_extent`, and `group_count_holds_at_the_largest_extent` pass
    * `[✅]`   `try_new_rejects_a_piece_size_that_is_not_a_power_of_two`, `try_new_rejects_a_zero_piece_size`, `try_new_rejects_a_piece_size_below_the_minimum`, `try_new_rejects_a_zero_piece_group_size`, `try_new_rejects_a_piece_group_size_that_is_not_a_multiple_of_the_piece_size`, `try_new_rejects_a_piece_group_smaller_than_a_piece`, `try_new_rejects_a_zero_total_extent`, and `try_new_reports_the_piece_size_before_the_total_extent` pass (EC-01, CR-06, a malformed geometry is refused before any key is derived)
    * `[✅]`   `try_new_reports_the_piece_size_before_the_piece_group_size` and `try_new_reports_the_piece_group_size_before_the_total_extent` pass (EC-01, CR-06, the refusal is deterministic)
    * `[✅]`   `try_new_admits_a_piece_group_size_that_is_an_odd_multiple_of_the_piece_size` passes
    * `[✅]`   `piece_size_returns_the_held_piece_size`, `piece_group_size_returns_the_held_piece_group_size`, and `total_extent_returns_the_held_total_extent` pass
    * `[✅]`   Code outside `crates/domain/src/piece_geometry` reading the `piece_size`, `piece_group_size`, or `total_extent` field fails to compile
