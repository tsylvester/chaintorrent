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

* `[ ]`   `workspace/cargo` **Virtual workspace manifest, toolchain pin, dependency policy, and build-output rules at the repository root**

  * `[ ]`   `objective`
    * `[ ]`   Problem: a crate is created, built, and linted only inside a Cargo workspace, and every crate is bound by one toolchain, one lint policy, and one dependency policy
    * `[ ]`   Functional: a virtual workspace manifest at the repository root admits every crate directory under `crates/`, `adapters/`, and `apps/` by glob, so a node that creates a crate never edits the root manifest
    * `[ ]`   Functional: the workspace lint table forbids `unsafe_code` and denies clippy's `unwrap_used`, `expect_used`, `panic`, and `as_conversions`, and every member inherits it through its own `[lints] workspace = true`
    * `[ ]`   Functional: the workspace declares the inheritable package keys every member takes through `edition.workspace = true`, `rust-version.workspace = true`, and `publish.workspace = true`
    * `[ ]`   Functional: every machine and every CI runner builds with one pinned toolchain carrying `clippy` and `rustfmt`
    * `[ ]`   Functional: `cargo-deny` admits only permissively licensed dependencies from crates.io, and denies yanked versions, wildcard version requirements on published dependencies, and any unlisted registry or git source
    * `[ ]`   Functional: build output is untracked and `Cargo.lock` is tracked
    * `[ ]`   Non-functional: no dependency is pinned, no crate is created, and no tool, target, or component is configured beyond what the four files below state

  * `[ ]`   `role`
    * `[ ]`   Infrastructure: four configuration files with no types and no tests, exempt from the support-file structure; the root of the build graph every later node compiles within
    * `[ ]`   Does not create any directory under `crates/`, `adapters/`, or `apps/`; the node that creates a crate writes that crate's `Cargo.toml`
    * `[ ]`   Does not declare a `[workspace.dependencies]` table or any dependency; each dependency is pinned by the node that first consumes it
    * `[ ]`   Does not create `clippy.toml`, `rustfmt.toml`, or `.cargo/config.toml`; test modules and `mock.rs` files carry their own module-level `allow` for the lint-table entries they are exempt from
    * `[ ]`   Does not add a compilation target or toolchain component beyond `clippy` and `rustfmt`; `wasm32-unknown-unknown` arrives with `apps/wasm-demo`
    * `[ ]`   Does not edit `.github/workflows/rust.yml`; the CI matrix, `cargo-audit`, and `cargo-deny` invocation belong to `workspace/ci`
    * `[ ]`   Does not carry a commit; the grouping's integration test and commit are carried by `random/factory`

  * `[ ]`   `module`
    * `[ ]`   Bounded context: repository-root build configuration — member layout, workspace lint table, inheritable package keys, the `librqbit` overlay, toolchain pin, dependency-policy file, and build-output ignore rules
    * `[ ]`   Member layout by ring: `crates/` holds `domain`, and `workflows`; `adapters/` holds one crate per adapter family, the crate path `adapters/<family>` the planning set names, each crate holding the family's `factory` module and its concretes as private modules beneath it; `apps/` holds one crate per deployable
    * `[ ]`   Outside: every member's own `Cargo.toml`, the CI definition, dependency versions, the Solidity workspace under `contracts/`, and the TypeScript shells under `shells/`, none of which is a Cargo member

  * `[ ]`   `deps`
    * `[ ]`   `rustup`, external toolchain manager, reads `rust-toolchain.toml` at the repository root; direction inward, the configuration names the toolchain and nothing names the configuration
    * `[ ]`   `cargo`, external build tool, reads `Cargo.toml`; resolver `3` requires the pinned toolchain, which satisfies it
    * `[ ]`   `cargo-deny`, external policy checker installed and invoked by `workspace/ci`, reads `deny.toml`
    * `[ ]`   No repository file is a dependency of this node, and no reverse dependency exists

  * `[ ]`   `Cargo.toml`
    * `[ ]`   `[workspace]` with `resolver = "3"` and `members = ["crates/*", "adapters/*", "apps/*"]`; no `exclude`, no `default-members`, no `[package]` table, so the manifest is virtual
    * `[ ]`   `[workspace.package]` with `edition = "2024"`, `rust-version = "1.98"`, and `publish = false`; no `license` key, since the license text is a release prerequisite
    * `[ ]`   `[workspace.lints.rust]` with `unsafe_code = "forbid"`
    * `[ ]`   `[workspace.lints.clippy]` with `unwrap_used = "deny"`, `expect_used = "deny"`, `panic = "deny"`, and `as_conversions = "deny"`
    * `[ ]`   `[patch.crates-io]` with `librqbit = { git = "https://github.com/tsylvester/rqbit", branch = "chaintorrent-overlay" }`, the branch existing at a tagged upstream release before this node is authored; `Cargo.lock` pins the commit
    * `[ ]`   No other table

  * `[ ]`   `rust-toolchain.toml`
    * `[ ]`   `[toolchain]` with `channel = "1.98.1"`, `components = ["clippy", "rustfmt"]`, and `profile = "minimal"`

  * `[ ]`   `deny.toml`
    * `[ ]`   `[graph]` with `all-features = true`
    * `[ ]`   `[advisories]` with `yanked = "deny"`, `unmaintained = "all"`, `unsound = "all"`, and `ignore = []`; no `version`, `vulnerability`, `notice`, or `severity-threshold` key
    * `[ ]`   `[licenses]` with `allow = ["Apache-2.0", "Apache-2.0 WITH LLVM-exception", "MIT", "BSD-2-Clause", "BSD-3-Clause", "ISC", "Zlib", "CC0-1.0", "BSL-1.0", "Unicode-3.0"]` and `confidence-threshold = 0.93`; no `version` key
    * `[ ]`   `[licenses.private]` with `ignore = true`, so the unpublished, unlicensed workspace members are not license-checked
    * `[ ]`   `[bans]` with `multiple-versions = "warn"`, `wildcards = "deny"`, and `allow-wildcard-paths = true`, so path dependencies between the private workspace members are admitted
    * `[ ]`   `[sources]` with `unknown-registry = "deny"`, `unknown-git = "deny"`, `allow-registry = ["https://github.com/rust-lang/crates.io-index"]`, and `allow-git = ["https://github.com/tsylvester/rqbit"]`, the source the `[patch.crates-io]` table names

  * `[ ]`   `.gitignore`
    * `[ ]`   One entry, `/target`; `Cargo.lock` is not ignored

  * `[ ]`   `directionality`
    * `[ ]`   The root manifest declares no dependency and depends on no member; members depend inward across rings, `apps/` on `adapters/` and `crates/`, `adapters/` on `crates/`, and `crates/domain` on nothing outside itself, each through its own manifest
    * `[ ]`   No cycle is introduced; the root is the build graph's container, not a node in it

  * `[ ]`   `requirements`
    * `[ ]`   `Cargo.toml`, `rust-toolchain.toml`, `deny.toml`, and `.gitignore` exist at the repository root with exactly the keys and values stated above and no others
    * `[ ]`   No directory exists under `crates/`, `adapters/`, or `apps/` when this node completes
    * `[ ]`   `cargo check`, `cargo clippy`, and `cargo fmt --check` at the repository root complete without error
    * `[ ]`   A production `unwrap`, `expect`, `panic!`, numeric `as`, or `unsafe` block in the first production crate, `domain/secret`, is rejected by `cargo clippy` under the inherited lint table
    * `[ ]`   `cargo deny check` under `workspace/ci` reads `deny.toml` without a configuration error on Windows, macOS, and Linux

* `[ ]`   `workspace/ci` **Continuous-integration matrix over Windows, macOS, and Linux running the workspace checks, tests, and dependency audits**

  * `[ ]`   `objective`
    * `[ ]`   Problem: every ticket's proof, format, type check, lint, tests, license policy, and advisories, runs on each platform the completion boundary names
    * `[ ]`   Functional: every push to any branch and every pull request to `main` runs the workspace proof on `ubuntu-latest`, `macos-latest`, and `windows-latest`, and a failure on one runner does not cancel the others
    * `[ ]`   Functional: each runner installs the toolchain and components pinned by `rust-toolchain.toml`, never a toolchain named in the workflow
    * `[ ]`   Functional: each runner runs, in order, the format check, the type check, the lint, the tests, the dependency-policy check, and the advisory audit, and any failure fails that runner's job
    * `[ ]`   Non-functional: the workflow token is read-only, a newer run on the same ref cancels the older one, and no step exists beyond those listed here

  * `[ ]`   `role`
    * `[ ]`   Infrastructure: one configuration file with no types and no tests, exempt from the support-file structure
    * `[ ]`   Does not create the Solidity, TypeScript, fuzz, or end-to-end workflow definitions, each a separate file created once by the ticket that first needs it, `contracts/evm/PairingLib` for `forge build` and `forge fmt --check`, the first shell or webview ticket for the TypeScript linter, the first fuzz target for `cargo-fuzz`, and the first end-to-end scenario for the clean-runner job; no later ticket edits `rust.yml`
    * `[ ]`   Does not edit `.github/workflows/npm-publish.yml`
    * `[ ]`   Does not package, sign, release, or publish, and reads no secret
    * `[ ]`   Does not carry a commit; the grouping's integration test and commit are carried by `random/factory`

  * `[ ]`   `module`
    * `[ ]`   Bounded context: the Rust continuous-integration definition — triggers, token permissions, concurrency, the runner matrix, toolchain and tool installation, and the proof commands
    * `[ ]`   Outside: the workspace configuration it reads (`workspace/cargo`), release packaging and signing, and every non-Rust check

  * `[ ]`   `deps`
    * `[ ]`   `workspace/cargo`, repository-root configuration: `Cargo.toml` for the members and lint table, `rust-toolchain.toml` for the toolchain and components, `deny.toml` for the dependency policy; the workflow consumes them and nothing in them refers to the workflow
    * `[ ]`   `actions/checkout@v7`, external action, checks out the ref under test
    * `[ ]`   `actions-rust-lang/setup-rust-toolchain@v2`, external action, given no `toolchain` input so it installs the toolchain and components from `rust-toolchain.toml`, caches through `Swatinem/rust-cache` by its default, and sets build warnings to deny by its default
    * `[ ]`   `taiki-e/install-action@v2`, external action, installs prebuilt `cargo-deny` and `cargo-audit` on Linux, macOS, and Windows
    * `[ ]`   No reverse dependency: no repository file reads the workflow

  * `[ ]`   `.github/workflows/rust.yml`
    * `[ ]`   `on`: `push` with `branches: ["**"]`, and `pull_request` with `branches: ["main"]`
    * `[ ]`   `permissions`: `contents: read`
    * `[ ]`   `concurrency`: `group: ${{ github.workflow }}-${{ github.ref }}` and `cancel-in-progress: true`
    * `[ ]`   `name`: `Rust`
    * `[ ]`   `env`: `CARGO_TERM_COLOR: always`
    * `[ ]`   `jobs.build`: `strategy` with `fail-fast: false` and `matrix.os: [ubuntu-latest, macos-latest, windows-latest]`; `runs-on: ${{ matrix.os }}`
    * `[ ]`   `jobs.build.steps`, in this order: `uses: actions/checkout@v7`; `uses: actions-rust-lang/setup-rust-toolchain@v2` with no `with` block; `uses: taiki-e/install-action@v2` with `tool: cargo-deny,cargo-audit`; `run: cargo fmt --all --check`; `run: cargo check --workspace --all-targets --all-features`; `run: cargo clippy --workspace --all-targets --all-features -- -D warnings`; `run: cargo test --workspace --all-features`; `run: cargo deny check`; `run: cargo audit`

  * `[ ]`   `directionality`
    * `[ ]`   The workflow depends inward on the root configuration and on pinned external actions; nothing in the repository depends on it; no cycle

  * `[ ]`   `requirements`
    * `[ ]`   `.github/workflows/rust.yml` carries exactly the triggers, permissions, concurrency, environment, matrix, and steps stated above, in that order, under the workflow name `Rust` and the job id `build`
    * `[ ]`   On each runner, `cargo deny check` reads `deny.toml` without a configuration error
    * `[ ]`   A formatting deviation, a clippy warning or denied lint, a failing test, a dependency outside the license allowlist or from an unlisted source, or an unignored advisory fails the job on the runner where it occurs

* `[ ]`   `domain/secret` **Secret-typed value that cannot be formatted, cloned, or serialized, exposes its value only through an explicit accessor, and zeroizes on drop; creates the `domain` crate**

  * `[ ]`   `objective`
    * `[ ]`   Problem: master scalars, credentials, piece-group keys, envelope secrets, and seeds are ordinary values unless a type prevents them from reaching a log, a diagnostic, a serialized record, or a stray copy, and from outliving their use in memory (CR-07)
    * `[ ]`   Functional: one generic type wraps any secret value whose type implements `zeroize::Zeroize`
    * `[ ]`   Functional: the wrapped value is reachable only through one explicit accessor returning a shared reference, never by field access outside the `secret` module
    * `[ ]`   Functional: the wrapped value is zeroized when the secret is dropped, on every path that ends its lifetime
    * `[ ]`   Functional: the type implements none of `core::fmt::Debug`, `core::fmt::Display`, `Clone`, or `Copy`, proven by a compile-time assertion
    * `[ ]`   Functional: the type cannot implement a serialization trait, because the `domain` crate has no serialization dependency
    * `[ ]`   Non-functional: the `domain` crate depends on `zeroize` alone at runtime and on no repository crate, host, chain SDK, wallet, transport, or storage engine

  * `[ ]`   `role`
    * `[ ]`   Domain: an owned value type in the protocol and domain ring; every secret-producing module in later nodes wraps its material in this type
    * `[ ]`   Does not generate, derive, compare, encrypt, or store secrets; producers and custody do that and hand the value to `Secret::try_new`
    * `[ ]`   Does not offer a mutable accessor, an owning extractor, equality, or ordering
    * `[ ]`   Does not create any other module of the `domain` crate; each further module is created by the node of the type it implements
    * `[ ]`   Does not carry a commit; the grouping's integration test and commit are carried by `random/factory`

  * `[ ]`   `module`
    * `[ ]`   Bounded context: the `secret` module of the `domain` crate, holding the `Secret<T>` type, its constructor-params type, and its lifecycle: construction by move, shared read access, zeroization on drop
    * `[ ]`   Creates the `domain` crate at `crates/domain`, admitted by the workspace's `crates/*` glob with no edit to the root manifest
    * `[ ]`   Outside: which values are secret, how they are produced, how custody wraps them for storage, and the memory-only secret region the daemon builds from them

  * `[ ]`   `deps`
    * `[ ]`   `zeroize` `1.9.0`, external crate, Apache-2.0 OR MIT, runtime dependency of `domain`; supplies the `Zeroize` trait the wrapped type is bound by and whose `zeroize` the drop calls; zeroization on drop is a property of the type, not an external touchpoint, so no adapter wraps it
    * `[ ]`   `static_assertions` `1.1.0`, external crate, MIT OR Apache-2.0, dev-dependency of `domain` only; supplies `assert_not_impl_any!`, the compile-time proof that the type implements no formatting, cloning, or copying trait
    * `[ ]`   `core::convert::Infallible`, standard library, the constructor's error arm
    * `[ ]`   No repository crate is a dependency; nothing depends on this module yet; direction inward, `domain` is the innermost ring

  * `[ ]`   `context_slice`
    * `[ ]`   From `zeroize`: the `Zeroize` trait's `zeroize(&mut self)` as a bound on `T`; nothing else
    * `[ ]`   From `static_assertions`: the `assert_not_impl_any!` macro, in `test.rs` only

  * `[ ]`   `crates/domain/Cargo.toml`
    * `[ ]`   `[package]` with `name = "domain"`, `edition.workspace = true`, `rust-version.workspace = true`, and `publish.workspace = true`; no `version` key
    * `[ ]`   `[dependencies]` with `zeroize = "1.9.0"`, default features
    * `[ ]`   `[dev-dependencies]` with `static_assertions = "1.1.0"`
    * `[ ]`   `[features]` with `mocks = []`
    * `[ ]`   `[lints]` with `workspace = true`
    * `[ ]`   No other table and no serialization dependency

  * `[ ]`   `crates/domain/src/lib.rs`
    * `[ ]`   The crate barrel: `mod secret;` and `pub use secret::provides::*;`, nothing else
    * `[ ]`   Until `secret/mod.rs` exists, `cargo check` reports the unresolved `mod secret`, which is the RED state for every element below that precedes the implementation

  * `[ ]`   `crates/domain/src/secret/interface.rs`
    * `[ ]`   `Secret<T: Zeroize>`, a struct with one field `pub(super) value: T`, so only the `secret` module and its children reach the field; no derives
    * `[ ]`   `SecretConstructorParams<T: Zeroize>`, a struct with one field `pub value: T`; no derives
    * `[ ]`   `SecretTryNewReturn<T>`, the type alias `Result<Secret<T>, Infallible>`, the two-arm return of the constructor; the error arm is uninhabited because moving a value into the wrapper has no failure
    * `[ ]`   Imports `zeroize::Zeroize` and `core::convert::Infallible`; declares nothing else

  * `[ ]`   `crates/domain/src/secret/interaction.spec.md`
    * `[ ]`   `Secret::try_new(params: SecretConstructorParams<T>) -> SecretTryNewReturn<T>`: one branch; condition: any params; decision: none; dependency call: none; outcome: `Ok(Secret)` holding `params.value`, moved without copy; the error arm has no branch
    * `[ ]`   `Secret::expose(&self) -> &T`: one branch; condition: a living secret; decision: none; dependency call: none; outcome: a shared reference to the held value, no copy, no side effect
    * `[ ]`   Drop: one branch; condition: the secret's lifetime ends, by scope end, move into a consumer that drops it, or unwinding; decision: none; dependency call: `Zeroize::zeroize` on the held value, exactly once; outcome: the held value's memory is zeroized before release
    * `[ ]`   Invariants: `Secret<T>` implements none of `Debug`, `Display`, `Clone`, or `Copy`, and cannot implement a serialization trait; its only producer is `try_new`

  * `[ ]`   `crates/domain/src/secret/mock.rs`
    * `[ ]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`
    * `[ ]`   `SecretConstructorParamsOverrides<T>`, `#[derive(Default)]`, one field `pub value: Option<T>`
    * `[ ]`   `build_secret_constructor_params<T: Zeroize + Default>(overrides: SecretConstructorParamsOverrides<T>) -> SecretConstructorParams<T>`, the value defaulting to `T::default()`
    * `[ ]`   `build_secret<T: Zeroize + Default>(overrides: SecretConstructorParamsOverrides<T>) -> Secret<T>`, returning the real instance from `Secret::try_new(build_secret_constructor_params(overrides))` through the irrefutable pattern `let Ok(secret) = …;`
    * `[ ]`   No corruptions type and no invalidator: the params never arrive as untrusted data, and the crate carries no serialization dependency to express corruption; no `Secret` overrides, invalidator, or mock function, since the class is built as a real instance and owns no free function

  * `[ ]`   `crates/domain/src/secret/test.rs`
    * `[ ]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`; imports `Secret` and `SecretConstructorParamsOverrides` from `super::interface` and `super::mock`, `build_secret` from `super::mock`, `assert_not_impl_any` from `static_assertions`, `zeroize::Zeroize`, `std::cell::Cell`, and `std::rc::Rc`
    * `[ ]`   Compile-time assertion with its contract header: `assert_not_impl_any!(Secret<[u8; 32]>: core::fmt::Debug, core::fmt::Display, Clone, Copy);`; contract: a secret has no formatting, cloning, or copying implementation
    * `[ ]`   `expose_returns_the_value_the_secret_was_constructed_with`: arrange `build_secret` with `value: Some([7u8; 32])`, differing from the builder's all-zero default; act `secret.expose()`; assert the reference equals `&[7u8; 32]`
    * `[ ]`   `dropping_a_secret_zeroizes_its_value`: a test-local `ZeroizeProbe` struct with `#[derive(Default)]` and one field `zeroized: Rc<Cell<bool>>`, implementing `Zeroize` by setting the flag to `true`; arrange a flag `Rc::new(Cell::new(false))` and `build_secret` with a probe holding a clone of it; act `drop(secret)`; assert the flag reads `true`
    * `[ ]`   Every test block carries the full `Contract`, `Arrange`, `Act`, `Assert` header and the inline markers; the compile-time assertion carries the header without markers, since it has no body

  * `[ ]`   `construction`
    * `[ ]`   `Secret::try_new` is the only producer; no `Default`, `From`, or other constructor exists; each later producer of secret material constructs the secret where the material is produced and passes it by move

  * `[ ]`   `crates/domain/src/secret/mod.rs`
    * `[ ]`   Module declarations: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, `pub mod provides;`, `#[cfg(test)] mod test;`
    * `[ ]`   `impl<T: Zeroize> Secret<T>` with `pub fn try_new(params: SecretConstructorParams<T>) -> SecretTryNewReturn<T>` returning `Ok(Secret { value: params.value })`, and `pub fn expose(&self) -> &T` returning `&self.value`
    * `[ ]`   `impl<T: Zeroize> Drop for Secret<T>` whose `drop` calls `self.value.zeroize()`
    * `[ ]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[ ]`   `crates/domain/src/secret/provides.rs`
    * `[ ]`   `pub use super::interface::*;` and `#[cfg(feature = "mocks")] pub use super::mock::*;`, nothing else

  * `[ ]`   `directionality`
    * `[ ]`   `secret` depends on `zeroize` and `core` only; `domain` depends on no repository crate; later consumers in the adapter and workflow rings depend on `domain` through `lib.rs`'s re-export of `secret::provides`; no cycle

  * `[ ]`   `requirements`
    * `[ ]`   `crates/domain/Cargo.toml` carries exactly the tables and keys stated above, and no serialization dependency
    * `[ ]`   `cargo check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo fmt --check` complete without error with the `domain` crate as the workspace's only member
    * `[ ]`   The `assert_not_impl_any!` assertion compiles, proving `Secret` implements none of `Debug`, `Display`, `Clone`, or `Copy` (CR-07, formatting excluded at compile time)
    * `[ ]`   `expose_returns_the_value_the_secret_was_constructed_with` passes
    * `[ ]`   `dropping_a_secret_zeroizes_its_value` passes (CR-07 zeroization)
    * `[ ]`   Code outside `crates/domain/src/secret` reading the `value` field fails to compile

* `[ ]`   `random/os` **Operating-system randomness concrete, the one source every production draw passes through; creates the `adapters/random` crate and authors the randomness family's generic interface, declaration, and mock**

  * `[ ]`   `objective`
    * `[ ]`   Problem: master scalars, capsule randomness, piece-group keys, and IVs come from a cryptographic random source (CR-05), and every production path draws through one repo-owned interface, so no module outside the concrete names the generator's library
    * `[ ]`   Functional: the family's generic interface draws a requested number of bytes and returns them inside a `Secret`, so every draw is zeroized when dropped whether or not the caller keeps it
    * `[ ]`   Functional: every concrete declares its source kind, its adapter version, and the interface version it implements, readable from the type before any instance exists
    * `[ ]`   Functional: the operating-system concrete draws from the operating system's generator through `getrandom` and returns the generator's error unchanged in its error arm
    * `[ ]`   Functional: repeated draws of a fixed width are pairwise distinct across a fixed count, and a draw of a fixed length holds more than one distinct byte value
    * `[ ]`   Non-functional: `getrandom` is named only inside `adapters/random/src/os`; the crate depends on no repository crate but `domain`

  * `[ ]`   `role`
    * `[ ]`   Adapter: the randomness family's first concrete, and the first source file that requires the family's generic interface, declaration, and mock, which it authors in the family's `factory` module as its producers
    * `[ ]`   Creates `factory/mod.rs` with the module wiring alone and `factory/provides.rs` with the interface and mock surface alone; the factory function with its types, interaction spec, mock, builders, unit test, and re-export, and the family's integration test, are `random/factory`'s
    * `[ ]`   Does not author the sampling trait a scalar type implements to be drawn from uniform bytes; `pairing/bn254_arkworks`, its first consumer, adds it to the family's interface
    * `[ ]`   Does not provide a deterministic source, a seeded source, or any source but the operating system's
    * `[ ]`   Does not configure a `getrandom` backend for `wasm32-unknown-unknown`; that arrives with `apps/wasm-demo`
    * `[ ]`   Does not carry a commit; the grouping's integration test and commit are carried by `random/factory`

  * `[ ]`   `module`
    * `[ ]`   Bounded context: the `adapters/random` crate's `factory` module, holding the family's generic trait, its method's params, payload, success, and error types, the declaration, the interface version, and the family's mock; and its private `os` concrete, holding the adapter over the operating system's generator, its constructor params, and its error type
    * `[ ]`   Creates the crate at `adapters/random`, admitted by the workspace's `adapters/*` glob with no edit to the root manifest
    * `[ ]`   Outside: which values a caller draws and what it builds from them, the sampling of scalars from uniform bytes, and the factory's admission rule

  * `[ ]`   `deps`
    * `[ ]`   `domain`, `crates/domain`, protocol and domain ring, path dependency; supplies `Secret` and `SecretConstructorParams`, the wrapper every draw is returned in; direction inward, adapter ring on domain ring
    * `[ ]`   `domain` with its `mocks` feature, as a dev-dependency and through this crate's `mocks` feature; supplies `build_secret` and `SecretConstructorParamsOverrides` for the success-return builder
    * `[ ]`   `getrandom` `0.4.3`, external crate, MIT OR Apache-2.0, runtime dependency named only in `os`; supplies `getrandom::fill` and `getrandom::Error`
    * `[ ]`   `core::convert::Infallible`, standard library, the constructor's error arm
    * `[ ]`   `std::collections::HashSet`, standard library, in `os/test.rs` only
    * `[ ]`   No reverse dependency; nothing depends on this crate yet

  * `[ ]`   `context_slice`
    * `[ ]`   From `domain`: `Secret::try_new(SecretConstructorParams { value })` returning `Result<Secret<T>, Infallible>`, and `Secret::expose(&self) -> &T` in tests
    * `[ ]`   From `domain`'s mocks: `build_secret(SecretConstructorParamsOverrides<T>) -> Secret<T>` for `T: Zeroize + Default`, instantiated at `Vec<u8>`
    * `[ ]`   From `getrandom`: `fill(dest: &mut [u8]) -> Result<(), Error>`, and `Error`, which implements `Debug`

  * `[ ]`   `adapters/random/Cargo.toml`
    * `[ ]`   `[package]` with `name = "random"`, `edition.workspace = true`, `rust-version.workspace = true`, and `publish.workspace = true`; no `version` key
    * `[ ]`   `[dependencies]` with `domain = { path = "../../crates/domain" }` and `getrandom = "0.4.3"`, default features
    * `[ ]`   `[dev-dependencies]` with `domain = { path = "../../crates/domain", features = ["mocks"] }`
    * `[ ]`   `[features]` with `mocks = ["domain/mocks"]`
    * `[ ]`   `[lints]` with `workspace = true`
    * `[ ]`   No other table

  * `[ ]`   `adapters/random/src/lib.rs`
    * `[ ]`   The crate barrel: `mod factory;`, `mod os;`, and `pub use factory::provides::*;`, nothing else
    * `[ ]`   Until `factory/mod.rs` and `os/mod.rs` exist, `cargo check` reports the unresolved modules, which is the RED state for every element below that precedes them

  * `[ ]`   `adapters/random/src/factory/interface.rs`
    * `[ ]`   `RANDOM_SOURCE_INTERFACE_VERSION`, a `pub const` of type `u32` with value `1`, the version of this interface a concrete declares it implements
    * `[ ]`   `RandomSourceKind`, an enum with the one variant `OperatingSystem`; no derives
    * `[ ]`   `RandomSourceDeclaration`, a struct with `pub source: RandomSourceKind`, `pub adapter_version: u32`, and `pub interface_version: u32`; no derives
    * `[ ]`   `FillBytesParams`, the fieldless struct `pub struct FillBytesParams;`, the per-call control slot of `fill_bytes`
    * `[ ]`   `FillBytesPayload`, a struct with `pub length: usize`, the number of bytes to draw
    * `[ ]`   `FillBytesSuccessReturn`, a struct with `pub bytes: Secret<Vec<u8>>`
    * `[ ]`   `FillBytesErrorReturn`, an enum with the one variant `OperatingSystem(OsRandomSourceFillBytesErrorReturn)`, the operating-system concrete's error carried unchanged; each concrete's error is its own variant
    * `[ ]`   `FillBytesReturn`, the type alias `Result<FillBytesSuccessReturn, FillBytesErrorReturn>`
    * `[ ]`   `IRandomSourceAdapter`, a trait with the one method `fn fill_bytes(&self, params: FillBytesParams, payload: FillBytesPayload) -> FillBytesReturn;`
    * `[ ]`   Imports `domain::Secret` and `OsRandomSourceFillBytesErrorReturn` from `crate::os::provides`; names no vendor

  * `[ ]`   `adapters/random/src/os/interface.rs`
    * `[ ]`   `OsRandomSource`, the unit struct `pub struct OsRandomSource;`, the adapter over the operating system's generator
    * `[ ]`   `OsRandomSourceConstructorParams`, the fieldless struct `pub struct OsRandomSourceConstructorParams;`, the constructor's deps slot
    * `[ ]`   `OsRandomSourceTryNewReturn`, the type alias `Result<OsRandomSource, Infallible>`; the error arm is uninhabited because the operating-system source takes no configuration
    * `[ ]`   `OsRandomSourceFillBytesErrorReturn`, an enum with the one variant `OperatingSystem(getrandom::Error)`, the generator's failure carried unchanged
    * `[ ]`   Imports `core::convert::Infallible`; names `getrandom::Error` by its full path; declares nothing else

  * `[ ]`   `adapters/random/src/os/interaction.spec.md`
    * `[ ]`   `OsRandomSource::try_new(params: OsRandomSourceConstructorParams) -> OsRandomSourceTryNewReturn`: one branch; condition: any params; decision: none; dependency call: none; outcome: `Ok(OsRandomSource)`; the error arm has no branch
    * `[ ]`   `OsRandomSource::DECLARATION`: an inherent constant, `RandomSourceDeclaration { source: RandomSourceKind::OperatingSystem, adapter_version: 1, interface_version: RANDOM_SOURCE_INTERFACE_VERSION }`
    * `[ ]`   `fill_bytes`, drawn: condition: `getrandom::fill` returns `Ok(())` over a zero-initialized buffer of `payload.length` bytes; decision: the fill result; dependency call: `getrandom::fill`, exactly once; outcome: `Ok(FillBytesSuccessReturn { bytes })`, the filled buffer moved into a `Secret` without copy
    * `[ ]`   `fill_bytes`, generator failed: condition: `getrandom::fill` returns `Err(error)`; decision: the fill result; dependency call: `getrandom::fill`, exactly once; outcome: `Err(FillBytesErrorReturn::OperatingSystem(OsRandomSourceFillBytesErrorReturn::OperatingSystem(error)))`, the error unchanged; the buffer, already moved into a `Secret`, is zeroized when it drops
    * `[ ]`   Ordering: the buffer is moved into a `Secret` after `getrandom::fill` returns and before its result is inspected, so the buffer is zeroized on both branches
    * `[ ]`   A `payload.length` of zero takes the drawn branch with an empty buffer; `params` carries no control and is not read

  * `[ ]`   `adapters/random/src/factory/mock.rs`
    * `[ ]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`
    * `[ ]`   `RandomSourceDeclarationOverrides`, `#[derive(Default)]`, fields `pub source: Option<RandomSourceKind>`, `pub adapter_version: Option<u32>`, and `pub interface_version: Option<u32>`; `build_random_source_declaration(overrides: RandomSourceDeclarationOverrides) -> RandomSourceDeclaration`, defaulting to `RandomSourceKind::OperatingSystem`, `1`, and `RANDOM_SOURCE_INTERFACE_VERSION`
    * `[ ]`   `FillBytesPayloadOverrides`, `#[derive(Default)]`, one field `pub length: Option<usize>`; `build_fill_bytes_payload(overrides: FillBytesPayloadOverrides) -> FillBytesPayload`, the length defaulting to `32`
    * `[ ]`   `FillBytesSuccessReturnOverrides`, `#[derive(Default)]`, one field `pub bytes: Option<Secret<Vec<u8>>>`; `build_fill_bytes_success_return(overrides: FillBytesSuccessReturnOverrides) -> FillBytesSuccessReturn`, the bytes defaulting to `build_secret::<Vec<u8>>(SecretConstructorParamsOverrides::default())`, an empty draw
    * `[ ]`   `MockIRandomSourceAdapter`, the unit struct `pub struct MockIRandomSourceAdapter;`, implementing `IRandomSourceAdapter` with `fill_bytes` returning `Ok(build_fill_bytes_success_return(Default::default()))` for any params and payload; a test needing a failing source implements the trait on its own local struct
    * `[ ]`   No builder for `FillBytesParams`, which is fieldless and used by its production value, or for `RandomSourceKind`, an enum; no corruptions type and no invalidator, since no value this interface owns arrives as untrusted data
    * `[ ]`   Imports `Secret`, `build_secret`, and `SecretConstructorParamsOverrides` from `domain`, and this module's types from `super::interface`

  * `[ ]`   `adapters/random/src/factory/mod.rs`
    * `[ ]`   Module wiring only: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, and `pub mod provides;`, nothing else

  * `[ ]`   `adapters/random/src/factory/provides.rs`
    * `[ ]`   `pub use super::interface::*;` and `#[cfg(any(test, feature = "mocks"))] pub use super::mock::*;`, nothing else

  * `[ ]`   `adapters/random/src/os/test.rs`
    * `[ ]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`; imports `OsRandomSource` and `OsRandomSourceConstructorParams` from `super::interface`, `IRandomSourceAdapter`, `FillBytesParams`, `FillBytesPayloadOverrides`, `build_fill_bytes_payload`, and `RANDOM_SOURCE_INTERFACE_VERSION` from `crate::factory::provides`, and `HashSet` from `std::collections`; each test constructs the subject by `let Ok(source) = OsRandomSource::try_new(OsRandomSourceConstructorParams);` in its arrangement and unpacks a draw by `let Ok(success) = … else { panic!(…) };`
    * `[ ]`   `fill_bytes_returns_the_number_of_bytes_requested`: contract: a payload length selects the draw's length; arrange the subject and `build_fill_bytes_payload` with `length: Some(48)`, differing from the builder's default; act `source.fill_bytes(FillBytesParams, payload)`; assert `success.bytes.expose().len()` equals `48`
    * `[ ]`   `fill_bytes_returns_an_empty_draw_for_a_zero_length`: contract: a zero length takes the drawn branch with an empty buffer; arrange the subject and `build_fill_bytes_payload` with `length: Some(0)`; act `source.fill_bytes(FillBytesParams, payload)`; assert the call returns `Ok` and `success.bytes.expose().is_empty()`
    * `[ ]`   `fill_bytes_draws_pairwise_distinct_values_across_repeated_draws`: contract: draws from the generator do not repeat (CR-05); arrange the subject and an empty `HashSet<Vec<u8>>`; act `source.fill_bytes(FillBytesParams, build_fill_bytes_payload(Default::default()))` sixteen times, inserting a copy of each exposed draw into the set; assert the set holds sixteen entries
    * `[ ]`   `fill_bytes_fills_a_draw_with_more_than_one_distinct_byte_value`: contract: a draw is filled by the generator rather than left at its zero initialization (CR-05); arrange the subject and `build_fill_bytes_payload` with `length: Some(1024)`; act `source.fill_bytes(FillBytesParams, payload)`; assert the `HashSet<u8>` of the exposed draw's bytes holds more than one value
    * `[ ]`   `os_random_source_declares_its_adapter_and_interface_versions`: contract: the concrete's declaration names its adapter version and the interface version it implements; arrange nothing; act read `OsRandomSource::DECLARATION`; assert `adapter_version` equals `1` and `interface_version` equals `RANDOM_SOURCE_INTERFACE_VERSION`
    * `[ ]`   Every test block carries the full `Contract`, `Arrange`, `Act`, `Assert` header and the inline markers

  * `[ ]`   `construction`
    * `[ ]`   `OsRandomSource::try_new` is the concrete's only producer, and its only caller is the randomness factory, which reads `OsRandomSource::DECLARATION` before constructing and returns the concrete to consumers as `Box<dyn IRandomSourceAdapter>` beside its declaration

  * `[ ]`   `adapters/random/src/os/mod.rs`
    * `[ ]`   Module declarations: `mod interface;`, `pub(crate) mod provides;`, and `#[cfg(test)] mod test;`
    * `[ ]`   `impl OsRandomSource` with `pub const DECLARATION: RandomSourceDeclaration` as the interaction spec states, and `pub fn try_new(_params: OsRandomSourceConstructorParams) -> OsRandomSourceTryNewReturn` returning `Ok(OsRandomSource)`
    * `[ ]`   `impl IRandomSourceAdapter for OsRandomSource` with `fn fill_bytes(&self, _params: FillBytesParams, payload: FillBytesPayload) -> FillBytesReturn`, which allocates `vec![0u8; payload.length]`, calls `getrandom::fill` on it, moves the buffer into a `Secret` by `let Ok(bytes) = Secret::try_new(SecretConstructorParams { value: buffer });`, and then matches the fill result into the two branches of the interaction spec
    * `[ ]`   Imports the factory's names from `crate::factory::provides`, `Secret` and `SecretConstructorParams` from `domain`, and this module's types from `interface`
    * `[ ]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[ ]`   `adapters/random/src/os/provides.rs`
    * `[ ]`   `pub(crate) use super::interface::*;`, nothing else, so the concrete is visible to the crate's factory and to nothing outside the crate

  * `[ ]`   `directionality`
    * `[ ]`   `os` depends on the `factory` module's surface through `crate::factory::provides`, on `domain`, and on `getrandom`; the `factory` module depends on `domain` and on `os`'s error type through `crate::os::provides`; among repository crates the crate depends on `crates/domain` alone, inward; nothing depends on the crate yet
    * `[ ]`   The mutual dependency between the `factory` module and `os` is the family form's recorded cycle: a concrete implements the factory's trait, the factory's error enum carries the concrete's error, and the factory function constructs the concrete

  * `[ ]`   `requirements`
    * `[ ]`   `adapters/random/Cargo.toml` carries exactly the tables and keys stated above, and `getrandom` is named nowhere in the crate outside `adapters/random/src/os`
    * `[ ]`   `cargo check --all-targets --all-features` and `cargo fmt --check` complete without error; `cargo clippy --all-targets --all-features` reports nothing beyond the library target's unused-item warnings for the `os` concrete, which `random/factory` resolves by constructing the concrete
    * `[ ]`   `fill_bytes_returns_the_number_of_bytes_requested` passes
    * `[ ]`   `fill_bytes_returns_an_empty_draw_for_a_zero_length` passes
    * `[ ]`   `fill_bytes_draws_pairwise_distinct_values_across_repeated_draws` passes (CR-05, the source's non-repetition)
    * `[ ]`   `fill_bytes_fills_a_draw_with_more_than_one_distinct_byte_value` passes (CR-05, the source fills what it is asked to fill)
    * `[ ]`   `os_random_source_declares_its_adapter_and_interface_versions` passes
    * `[ ]`   A generator failure is returned as `FillBytesErrorReturn::OperatingSystem` holding `OsRandomSourceFillBytesErrorReturn::OperatingSystem` with the `getrandom::Error` unchanged, fixed by the error arm's type; the failure branch has no unit test, since the operating system's generator cannot be driven to fail from a test and the vendor is not mocked
    * `[ ]`   Code outside `adapters/random` naming `OsRandomSource` or anything under `os` fails to compile; the crate's public surface is the `factory` module's `provides`

* `[ ]`   `random/factory` **Randomness factory constructing the concrete a configuration names and returning it behind the family's trait with its declaration; carries the family's integration test and the grouping's commit**

  * `[ ]`   `objective`
    * `[ ]`   Problem: a consumer obtains a randomness source only through the family's generic surface, never by naming a concrete, and the composition reads what was constructed from its declaration (CR-05; Composition Boundary)
    * `[ ]`   Functional: given a requested `RandomSourceKind`, the factory constructs the matching concrete and returns it as `Box<dyn IRandomSourceAdapter>` together with that concrete's declaration
    * `[ ]`   Functional: a concrete's constructor error is returned unchanged in the factory's error arm, one variant per concrete
    * `[ ]`   Functional: a source obtained from the factory draws the requested number of random bytes through the family's trait
    * `[ ]`   Non-functional: adding a concrete is its module, its variant in the family's error enums, and its branch here; no consumer changes

  * `[ ]`   `role`
    * `[ ]`   Adapter family factory: the implementation of the `factory` module, which is the randomness family's construction point and the crate's public surface
    * `[ ]`   Does not decode or validate the requested kind; the configuration registry decodes and validates configuration and hands the factory a typed `RandomSourceKind`
    * `[ ]`   Does not admit against upstream declarations; the randomness family consumes no other family
    * `[ ]`   Does not draw bytes; drawing is the concrete's
    * `[ ]`   Carries the family's integration test and the commit for the workspace and discipline bootstrap milestone

  * `[ ]`   `module`
    * `[ ]`   Bounded context: the `factory` module of `adapters/random`, holding the factory function, its deps, params, payload, and return types, its signature type, its function mock and builders, and the crate's integration test under `adapters/random/tests`
    * `[ ]`   Outside: every concrete's behavior, the decoding of configuration, and every consumer of the family

  * `[ ]`   `deps`
    * `[ ]`   The `os` concrete, through `crate::os::provides`: `OsRandomSource`, `OsRandomSourceConstructorParams`, `OsRandomSource::try_new`, and `OsRandomSource::DECLARATION`; the factory constructs its concretes, the family form's recorded cycle
    * `[ ]`   The `factory` module's own interface: `IRandomSourceAdapter`, `RandomSourceKind`, and `RandomSourceDeclaration`
    * `[ ]`   `core::convert::Infallible`, standard library, the operating-system constructor's error carried in the factory's error arm
    * `[ ]`   `std::collections::HashSet`, standard library, in the integration test only
    * `[ ]`   No new external crate; `adapters/random/Cargo.toml` is unchanged

  * `[ ]`   `context_slice`
    * `[ ]`   From `os`: `OsRandomSource::try_new(OsRandomSourceConstructorParams) -> Result<OsRandomSource, Infallible>`, the inherent constant `OsRandomSource::DECLARATION: RandomSourceDeclaration`, and `OsRandomSource`'s implementation of `IRandomSourceAdapter`

  * `[ ]`   `adapters/random/src/factory/interface.rs`
    * `[ ]`   `CreateRandomSourceDeps`, the fieldless struct `pub struct CreateRandomSourceDeps;`
    * `[ ]`   `CreateRandomSourceParams`, the fieldless struct `pub struct CreateRandomSourceParams;`
    * `[ ]`   `CreateRandomSourcePayload`, a struct with `pub kind: RandomSourceKind`
    * `[ ]`   `CreateRandomSourceSuccessReturn`, a struct with `pub adapter: Box<dyn IRandomSourceAdapter>` and `pub declaration: RandomSourceDeclaration`
    * `[ ]`   `CreateRandomSourceErrorReturn`, an enum with the one variant `OperatingSystem(Infallible)`, the operating-system concrete's constructor error carried unchanged; each concrete's constructor error is its own variant
    * `[ ]`   `CreateRandomSourceReturn`, the type alias `Result<CreateRandomSourceSuccessReturn, CreateRandomSourceErrorReturn>`
    * `[ ]`   `CreateRandomSourceFn`, the type alias `fn(&CreateRandomSourceDeps, CreateRandomSourceParams, CreateRandomSourcePayload) -> CreateRandomSourceReturn`
    * `[ ]`   Adds the import of `core::convert::Infallible`; every item `random/os` authored in this file is unchanged

  * `[ ]`   `adapters/random/src/factory/interaction.spec.md`
    * `[ ]`   `create_random_source`, operating system: condition: `payload.kind` is `RandomSourceKind::OperatingSystem`; decision: a `match` on `payload.kind`; dependency call: `OsRandomSource::try_new(OsRandomSourceConstructorParams)`, exactly once; outcome: `Ok(CreateRandomSourceSuccessReturn { adapter: Box::new(source), declaration: OsRandomSource::DECLARATION })`
    * `[ ]`   The operating-system constructor's error arm is uninhabited, so its success is destructured irrefutably and that branch has no failure outcome; `CreateRandomSourceErrorReturn::OperatingSystem` carries its error type in the return union
    * `[ ]`   `deps` and `params` carry nothing and are not read; the `match` is exhaustive over `RandomSourceKind`, so a kind with no branch fails to compile

  * `[ ]`   `adapters/random/src/factory/mock.rs`
    * `[ ]`   `CreateRandomSourcePayloadOverrides`, `#[derive(Default)]`, one field `pub kind: Option<RandomSourceKind>`; `build_create_random_source_payload(overrides: CreateRandomSourcePayloadOverrides) -> CreateRandomSourcePayload`, the kind defaulting to `RandomSourceKind::OperatingSystem`
    * `[ ]`   `CreateRandomSourceSuccessReturnOverrides`, `#[derive(Default)]`, fields `pub adapter: Option<Box<dyn IRandomSourceAdapter>>` and `pub declaration: Option<RandomSourceDeclaration>`; `build_create_random_source_success_return(overrides: CreateRandomSourceSuccessReturnOverrides) -> CreateRandomSourceSuccessReturn`, the adapter defaulting to `Box::new(MockIRandomSourceAdapter)` and the declaration to `build_random_source_declaration(Default::default())`
    * `[ ]`   `mock_create_random_source(_deps: &CreateRandomSourceDeps, _params: CreateRandomSourceParams, _payload: CreateRandomSourcePayload) -> CreateRandomSourceReturn`, returning `Ok(build_create_random_source_success_return(Default::default()))`
    * `[ ]`   No builder for the fieldless `CreateRandomSourceDeps` and `CreateRandomSourceParams`, used by their production values, or for the enum `CreateRandomSourceErrorReturn`; every symbol `random/os` authored in this file is unchanged

  * `[ ]`   `adapters/random/src/factory/test.rs`
    * `[ ]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`; imports `create_random_source` from `super`, `CreateRandomSourceDeps`, `CreateRandomSourceParams`, `RandomSourceKind`, and `RANDOM_SOURCE_INTERFACE_VERSION` from `super::interface`, and `build_create_random_source_payload` and `CreateRandomSourcePayloadOverrides` from `super::mock`
    * `[ ]`   `create_random_source_returns_the_operating_system_source_for_its_kind`: contract: the operating-system kind returns `Ok` with the operating-system concrete's declaration; arrange `build_create_random_source_payload` with `kind: Some(RandomSourceKind::OperatingSystem)`; act `create_random_source(&CreateRandomSourceDeps, CreateRandomSourceParams, payload)`, unpacked by `let Ok(success) = … else { panic!(…) };`; assert `success.declaration.adapter_version` equals `1` and `success.declaration.interface_version` equals `RANDOM_SOURCE_INTERFACE_VERSION`
    * `[ ]`   The test block carries the full `Contract`, `Arrange`, `Act`, `Assert` header and the inline markers

  * `[ ]`   `construction`
    * `[ ]`   The composition root calls `create_random_source` with `&CreateRandomSourceDeps`, `CreateRandomSourceParams`, and the configured `RandomSourceKind`, and places the returned `Box<dyn IRandomSourceAdapter>` in each consumer's deps; no consumer constructs a concrete

  * `[ ]`   `adapters/random/src/factory/mod.rs`
    * `[ ]`   Adds `#[cfg(test)] mod test;` to the wiring `random/os` authored
    * `[ ]`   `pub fn create_random_source(_deps: &CreateRandomSourceDeps, _params: CreateRandomSourceParams, payload: CreateRandomSourcePayload) -> CreateRandomSourceReturn`, a `match` on `payload.kind` whose `RandomSourceKind::OperatingSystem` arm binds the concrete by `let Ok(source) = OsRandomSource::try_new(OsRandomSourceConstructorParams);` and returns `Ok(CreateRandomSourceSuccessReturn { adapter: Box::new(source), declaration: OsRandomSource::DECLARATION })`
    * `[ ]`   Imports `OsRandomSource` and `OsRandomSourceConstructorParams` from `crate::os::provides`, and this module's types from `interface`
    * `[ ]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[ ]`   `adapters/random/src/factory/provides.rs`
    * `[ ]`   Adds `pub use super::create_random_source;` to the re-exports `random/os` authored

  * `[ ]`   `adapters/random/tests/integration_test.rs`
    * `[ ]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`; imports `create_random_source`, `CreateRandomSourceDeps`, `CreateRandomSourceParams`, `RandomSourceKind`, `build_create_random_source_payload`, `CreateRandomSourcePayloadOverrides`, `FillBytesParams`, `build_fill_bytes_payload`, and `FillBytesPayloadOverrides` from `random`, and `HashSet` from `std::collections`; the builders are reached through the crate's `mocks` feature, which the workspace's test and check commands enable with `--all-features`
    * `[ ]`   `a_source_from_the_factory_draws_random_bytes_through_the_family_trait`: contract: the factory's operating-system source, used only through `Box<dyn IRandomSourceAdapter>`, returns a draw of the requested length filled by the generator; arrange `build_create_random_source_payload` with `kind: Some(RandomSourceKind::OperatingSystem)` and `build_fill_bytes_payload` with `length: Some(64)`; act `create_random_source` and then `fill_bytes` on the returned adapter, each unpacked by `let Ok(…) = … else { panic!(…) };`; assert the exposed draw's length equals `64` and the `HashSet<u8>` of its bytes holds more than one value
    * `[ ]`   The test block carries the full `Contract`, `Arrange`, `Act`, `Assert` header and the inline markers; nothing is mocked, since the operating system's generator is the outer edge

  * `[ ]`   `directionality`
    * `[ ]`   The `factory` module depends on `os` through `crate::os::provides` and on its own interface; `os` depends on the `factory` module's surface, the family form's recorded cycle; the crate's public surface is the `factory` module's `provides`; nothing depends on the crate yet

  * `[ ]`   `requirements`
    * `[ ]`   `create_random_source_returns_the_operating_system_source_for_its_kind` passes
    * `[ ]`   `a_source_from_the_factory_draws_random_bytes_through_the_family_trait` passes (CR-05, a production draw passes through the factory's surface)
    * `[ ]`   `cargo check --workspace --all-targets --all-features`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `cargo fmt --all --check` complete without error or warning in every target, the `os` concrete's unused-item warnings having no remaining cause
    * `[ ]`   `getrandom` is named nowhere outside `adapters/random/src/os`, and no code outside `adapters/random` can name `OsRandomSource`
    * `[ ]`   The workspace and discipline bootstrap milestone's exit holds: the workspace, `crates/domain`, and `adapters/random` build and pass the Rust CI definition on Windows, macOS, and Linux

  * `[ ]`   **Commit** `feat(foundation): workspace, Rust CI, secret type, and randomness family`
    * `[ ]`   Structural: the virtual workspace manifest with its lint table and `librqbit` overlay, `rust-toolchain.toml`, `deny.toml`, `.gitignore`, `.github/workflows/rust.yml`, the `domain` crate with its `secret` module, and the `random` crate with its `factory` module and `os` concrete
    * `[ ]`   Behavioral: secrets zeroize on drop and cannot be formatted, cloned, or copied; random bytes are drawn from the operating system's generator into a `Secret` through the family's factory
    * `[ ]`   Contract: `Secret` with `try_new` and `expose`; `IRandomSourceAdapter` with `fill_bytes`, its params, payload, and return types, `RandomSourceDeclaration`, and `RANDOM_SOURCE_INTERFACE_VERSION`; `create_random_source` with its signature and return types

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