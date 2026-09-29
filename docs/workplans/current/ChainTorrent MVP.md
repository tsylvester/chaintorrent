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
    * `[✅]`   Functional: the workspace lint table forbids `unsafe_code` and denies clippy's `unwrap_used`, `expect_used`, `panic`, and `as_conversions`, and every member inherits it through its own `[lints] workspace = true`
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
    * `[✅]`   `[workspace.lints.rust]` with `unsafe_code = "forbid"`
    * `[✅]`   `[workspace.lints.clippy]` with `unwrap_used = "deny"`, `expect_used = "deny"`, `panic = "deny"`, and `as_conversions = "deny"`
    * `[✅]`   `[patch.crates-io]` with `librqbit = { git = "https://github.com/tsylvester/rqbit", branch = "chaintorrent-overlay" }`, the branch existing at a tagged upstream release before this node is authored; `Cargo.lock` pins the commit
    * `[✅]`   No other table

  * `[✅]`   `rust-toolchain.toml`
    * `[✅]`   `[toolchain]` with `channel = "1.98.1"`, `components = ["clippy", "rustfmt"]`, and `profile = "minimal"`

  * `[✅]`   `deny.toml`
    * `[✅]`   `[graph]` with `all-features = true`
    * `[✅]`   `[advisories]` with `yanked = "deny"`, `unmaintained = "all"`, `unsound = "all"`, and `ignore = []`; no `version`, `vulnerability`, `notice`, or `severity-threshold` key
    * `[✅]`   `[licenses]` with `allow = ["Apache-2.0", "Apache-2.0 WITH LLVM-exception", "MIT", "BSD-2-Clause", "BSD-3-Clause", "ISC", "Zlib", "CC0-1.0", "BSL-1.0", "Unicode-3.0"]` and `confidence-threshold = 0.93`; no `version` key
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

* `[✅]`   `workspace/ci` **Continuous-integration matrix over Windows, macOS, and Linux running the workspace checks, tests, and dependency audits**

  * `[✅]`   `objective`
    * `[✅]`   Problem: every ticket's proof, format, type check, lint, tests, license policy, and advisories, runs on each platform the completion boundary names
    * `[✅]`   Functional: every push to any branch and every pull request to `main` runs the workspace proof on `ubuntu-latest`, `macos-latest`, and `windows-latest`, and a failure on one runner does not cancel the others
    * `[✅]`   Functional: each runner installs the toolchain and components pinned by `rust-toolchain.toml`, never a toolchain named in the workflow
    * `[✅]`   Functional: each runner runs, in order, the format check, the type check, the lint, the tests, the dependency-policy check, and the advisory audit, and any failure fails that runner's job
    * `[✅]`   Non-functional: the workflow token is read-only, a newer run on the same ref cancels the older one, and no step exists beyond those listed here

  * `[✅]`   `role`
    * `[✅]`   Infrastructure: one configuration file with no types and no tests, exempt from the support-file structure
    * `[✅]`   Does not create the Solidity, TypeScript, fuzz, or end-to-end workflow definitions, each a separate file created once by the ticket that first needs it, `contracts/evm/PairingLib` for `forge build` and `forge fmt --check`, the first shell or webview ticket for the TypeScript linter, the first fuzz target for `cargo-fuzz`, and the first end-to-end scenario for the clean-runner job; no later ticket edits `rust.yml`
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
    * `[✅]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`
    * `[✅]`   `SecretConstructorParamsOverrides<T>`, `#[derive(Default)]`, one field `pub value: Option<T>`
    * `[✅]`   `build_secret_constructor_params<T: Zeroize + Default>(overrides: SecretConstructorParamsOverrides<T>) -> SecretConstructorParams<T>`, the value defaulting to `T::default()`
    * `[✅]`   `build_secret<T: Zeroize + Default>(overrides: SecretConstructorParamsOverrides<T>) -> Secret<T>`, returning the real instance from `Secret::try_new(build_secret_constructor_params(overrides))` through the irrefutable pattern `let Ok(secret) = …;`
    * `[✅]`   No corruptions type and no invalidator: the params never arrive as untrusted data, and the crate carries no serialization dependency to express corruption; no `Secret` overrides, invalidator, or mock function, since the class is built as a real instance and owns no free function

  * `[✅]`   `crates/domain/src/secret/test.rs`
    * `[✅]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`; imports `Secret` and `SecretConstructorParamsOverrides` from `super::interface` and `super::mock`, `build_secret` from `super::mock`, `assert_not_impl_any` from `static_assertions`, `zeroize::Zeroize`, `std::cell::Cell`, and `std::rc::Rc`
    * `[✅]`   Compile-time assertion with its contract header: `assert_not_impl_any!(Secret<[u8; 32]>: core::fmt::Debug, core::fmt::Display, Clone, Copy);`; contract: a secret has no formatting, cloning, or copying implementation
    * `[✅]`   `expose_returns_the_value_the_secret_was_constructed_with`: arrange `build_secret` with `value: Some([7u8; 32])`, differing from the builder's all-zero default; act `secret.expose()`; assert the reference equals `&[7u8; 32]`
    * `[✅]`   `dropping_a_secret_zeroizes_its_value`: a test-local `ZeroizeProbe` struct with `#[derive(Default)]` and one field `zeroized: Rc<Cell<bool>>`, implementing `Zeroize` by setting the flag to `true`; arrange a flag `Rc::new(Cell::new(false))` and `build_secret` with a probe holding a clone of it; act `drop(secret)`; assert the flag reads `true`
    * `[✅]`   Every test block carries the full `Contract`, `Arrange`, `Act`, `Assert` header and the inline markers; the compile-time assertion carries the header without markers, since it has no body

  * `[✅]`   `construction`
    * `[✅]`   `Secret::try_new` is the only producer; no `Default`, `From`, or other constructor exists; each later producer of secret material constructs the secret where the material is produced and passes it by move

  * `[✅]`   `crates/domain/src/secret/mod.rs`
    * `[✅]`   Module declarations: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, `pub mod provides;`, `#[cfg(test)] mod test;`
    * `[✅]`   `impl<T: Zeroize> Secret<T>` with `pub fn try_new(params: SecretConstructorParams<T>) -> SecretTryNewReturn<T>` returning `Ok(Secret { value: params.value })`, and `pub fn expose(&self) -> &T` returning `&self.value`
    * `[✅]`   `impl<T: Zeroize> Drop for Secret<T>` whose `drop` calls `self.value.zeroize()`
    * `[✅]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[✅]`   `crates/domain/src/secret/provides.rs`
    * `[✅]`   `pub use super::interface::*;` and `#[cfg(feature = "mocks")] pub use super::mock::*;`, nothing else

  * `[✅]`   `directionality`
    * `[✅]`   `secret` depends on `zeroize` and `core` only; `domain` depends on no repository crate; later consumers in the adapter and workflow rings depend on `domain` through `lib.rs`'s re-export of `secret::provides`; no cycle

  * `[✅]`   `requirements`
    * `[✅]`   `crates/domain/Cargo.toml` carries exactly the tables and keys stated above, and no serialization dependency
    * `[✅]`   `cargo check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo fmt --check` complete without error with the `domain` crate as the workspace's only member
    * `[✅]`   The `assert_not_impl_any!` assertion compiles, proving `Secret` implements none of `Debug`, `Display`, `Clone`, or `Copy` (CR-07, formatting excluded at compile time)
    * `[✅]`   `expose_returns_the_value_the_secret_was_constructed_with` passes
    * `[✅]`   `dropping_a_secret_zeroizes_its_value` passes (CR-07 zeroization)
    * `[✅]`   Code outside `crates/domain/src/secret` reading the `value` field fails to compile
    * `[✅]`   A production `unwrap`, `expect`, `panic!`, numeric `as`, or `unsafe` block in the first production crate, `domain/secret`, is rejected by `cargo clippy` under the inherited lint table
    * `[✅]`   `cargo check`, `cargo clippy`, and `cargo fmt --check` at the repository root complete without error

* `[✅]`   `random/os` **Operating-system randomness concrete, the one source every production draw passes through; creates the `adapters/random` crate and authors the randomness family's generic interface, declaration, and mock**

  * `[✅]`   `objective`
    * `[✅]`   Problem: master scalars, capsule randomness, piece-group keys, and IVs come from a cryptographic random source (CR-05), and every production path draws through one repo-owned interface, so no module outside the concrete names the generator's library
    * `[✅]`   Functional: the family's generic interface draws a requested number of bytes and returns them inside a `Secret`, so every draw is zeroized when dropped whether or not the caller keeps it
    * `[✅]`   Functional: every concrete declares its source kind, its adapter version, and the interface version it implements, readable from the type before any instance exists
    * `[✅]`   Functional: the operating-system concrete draws from the operating system's generator through `getrandom` and returns the generator's error unchanged in its error arm
    * `[✅]`   Functional: repeated draws of a fixed width are pairwise distinct across a fixed count, and a draw of a fixed length holds more than one distinct byte value
    * `[✅]`   Non-functional: `getrandom` is named only inside `adapters/random/src/os`; the crate depends on no repository crate but `domain`

  * `[✅]`   `role`
    * `[✅]`   Adapter: the randomness family's first concrete, and the first source file that requires the family's generic interface, declaration, and mock, which it authors in the family's `factory` module as its producers
    * `[✅]`   Creates `factory/mod.rs` with the module wiring alone and `factory/provides.rs` with the interface and mock surface alone; the factory function with its types, interaction spec, mock, builders, unit test, and re-export, and the family's integration test, are `random/factory`'s
    * `[✅]`   Does not author the sampling trait a scalar type implements to be drawn from uniform bytes; `pairing/bn254_arkworks`, its first consumer, adds it to the family's interface
    * `[✅]`   Does not provide a deterministic source, a seeded source, or any source but the operating system's
    * `[✅]`   Does not configure a `getrandom` backend for `wasm32-unknown-unknown`; that arrives with `apps/wasm-demo`
    * `[✅]`   Does not carry a commit; the grouping's integration test and commit are carried by `random/factory`

  * `[✅]`   `module`
    * `[✅]`   Bounded context: the `adapters/random` crate's `factory` module, holding the family's generic trait, its method's params, payload, success, and error types, the declaration, the interface version, and the family's mock; and its private `os` concrete, holding the adapter over the operating system's generator, its constructor params, and its error type
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
    * `[✅]`   From `domain`'s mocks: `build_secret(SecretConstructorParamsOverrides<T>) -> Secret<T>` for `T: Zeroize + Default`, instantiated at `Vec<u8>`
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
    * `[✅]`   `RandomSourceKind`, an enum with the one variant `OperatingSystem`; no derives
    * `[✅]`   `RandomSourceDeclaration`, a struct with `pub source: RandomSourceKind`, `pub adapter_version: u32`, and `pub interface_version: u32`; no derives
    * `[✅]`   `FillBytesParams`, the fieldless struct `pub struct FillBytesParams;`, the per-call control slot of `fill_bytes`
    * `[✅]`   `FillBytesPayload`, a struct with `pub length: usize`, the number of bytes to draw
    * `[✅]`   `FillBytesSuccessReturn`, a struct with `pub bytes: Secret<Vec<u8>>`
    * `[✅]`   `FillBytesErrorReturn`, an enum with the one variant `OperatingSystem(OsRandomSourceFillBytesErrorReturn)`, the operating-system concrete's error carried unchanged; each concrete's error is its own variant
    * `[✅]`   `FillBytesReturn`, the type alias `Result<FillBytesSuccessReturn, FillBytesErrorReturn>`
    * `[✅]`   `IRandomSourceAdapter`, a trait with the one method `fn fill_bytes(&self, params: FillBytesParams, payload: FillBytesPayload) -> FillBytesReturn;`
    * `[✅]`   Imports `domain::Secret` and `OsRandomSourceFillBytesErrorReturn` from `crate::os::provides`; names no vendor

  * `[✅]`   `adapters/random/src/os/interface.rs`
    * `[✅]`   `OsRandomSource`, the unit struct `pub struct OsRandomSource;`, the adapter over the operating system's generator
    * `[✅]`   `OsRandomSourceConstructorParams`, the fieldless struct `pub struct OsRandomSourceConstructorParams;`, the constructor's deps slot
    * `[✅]`   `OsRandomSourceTryNewReturn`, the type alias `Result<OsRandomSource, Infallible>`; the error arm is uninhabited because the operating-system source takes no configuration
    * `[✅]`   `OsRandomSourceFillBytesErrorReturn`, an enum with the one variant `OperatingSystem(getrandom::Error)`, the generator's failure carried unchanged
    * `[✅]`   Imports `core::convert::Infallible`; names `getrandom::Error` by its full path; declares nothing else

  * `[✅]`   `adapters/random/src/os/interaction.spec.md`
    * `[✅]`   `OsRandomSource::try_new(params: OsRandomSourceConstructorParams) -> OsRandomSourceTryNewReturn`: one branch; condition: any params; decision: none; dependency call: none; outcome: `Ok(OsRandomSource)`; the error arm has no branch
    * `[✅]`   `OsRandomSource::DECLARATION`: an inherent constant, `RandomSourceDeclaration { source: RandomSourceKind::OperatingSystem, adapter_version: 1, interface_version: RANDOM_SOURCE_INTERFACE_VERSION }`
    * `[✅]`   `fill_bytes`, drawn: condition: `getrandom::fill` returns `Ok(())` over a zero-initialized buffer of `payload.length` bytes; decision: the fill result; dependency call: `getrandom::fill`, exactly once; outcome: `Ok(FillBytesSuccessReturn { bytes })`, the filled buffer moved into a `Secret` without copy
    * `[✅]`   `fill_bytes`, generator failed: condition: `getrandom::fill` returns `Err(error)`; decision: the fill result; dependency call: `getrandom::fill`, exactly once; outcome: `Err(FillBytesErrorReturn::OperatingSystem(OsRandomSourceFillBytesErrorReturn::OperatingSystem(error)))`, the error unchanged; the buffer, already moved into a `Secret`, is zeroized when it drops
    * `[✅]`   Ordering: the buffer is moved into a `Secret` after `getrandom::fill` returns and before its result is inspected, so the buffer is zeroized on both branches
    * `[✅]`   A `payload.length` of zero takes the drawn branch with an empty buffer; `params` carries no control and is not read

  * `[✅]`   `adapters/random/src/factory/mock.rs`
    * `[✅]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`
    * `[✅]`   `RandomSourceDeclarationOverrides`, `#[derive(Default)]`, fields `pub source: Option<RandomSourceKind>`, `pub adapter_version: Option<u32>`, and `pub interface_version: Option<u32>`; `build_random_source_declaration(overrides: RandomSourceDeclarationOverrides) -> RandomSourceDeclaration`, defaulting to `RandomSourceKind::OperatingSystem`, `1`, and `RANDOM_SOURCE_INTERFACE_VERSION`
    * `[✅]`   `FillBytesPayloadOverrides`, `#[derive(Default)]`, one field `pub length: Option<usize>`; `build_fill_bytes_payload(overrides: FillBytesPayloadOverrides) -> FillBytesPayload`, the length defaulting to `32`
    * `[✅]`   `FillBytesSuccessReturnOverrides`, `#[derive(Default)]`, one field `pub bytes: Option<Secret<Vec<u8>>>`; `build_fill_bytes_success_return(overrides: FillBytesSuccessReturnOverrides) -> FillBytesSuccessReturn`, the bytes defaulting to `build_secret::<Vec<u8>>(SecretConstructorParamsOverrides::default())`, an empty draw
    * `[✅]`   `MockIRandomSourceAdapter`, the unit struct `pub struct MockIRandomSourceAdapter;`, implementing `IRandomSourceAdapter` with `fill_bytes` returning `Ok(build_fill_bytes_success_return(Default::default()))` for any params and payload; a test needing a failing source implements the trait on its own local struct
    * `[✅]`   No builder for `FillBytesParams`, which is fieldless and used by its production value, or for `RandomSourceKind`, an enum; no corruptions type and no invalidator, since no value this interface owns arrives as untrusted data
    * `[✅]`   Imports `Secret`, `build_secret`, and `SecretConstructorParamsOverrides` from `domain`, and this module's types from `super::interface`

  * `[✅]`   `adapters/random/src/factory/mod.rs`
    * `[✅]`   Module wiring only: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, and `pub mod provides;`, nothing else

  * `[✅]`   `adapters/random/src/factory/provides.rs`
    * `[✅]`   `pub use super::interface::*;` and `#[cfg(any(test, feature = "mocks"))] pub use super::mock::*;`, nothing else

  * `[✅]`   `adapters/random/src/os/test.rs`
    * `[✅]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`; imports `OsRandomSource` and `OsRandomSourceConstructorParams` from `super::interface`, `IRandomSourceAdapter`, `FillBytesParams`, `FillBytesPayloadOverrides`, `build_fill_bytes_payload`, and `RANDOM_SOURCE_INTERFACE_VERSION` from `crate::factory::provides`, and `HashSet` from `std::collections`; each test constructs the subject by `let Ok(source) = OsRandomSource::try_new(OsRandomSourceConstructorParams);` in its arrangement and unpacks a draw by `let Ok(success) = … else { panic!(…) };`
    * `[✅]`   `fill_bytes_returns_the_number_of_bytes_requested`: contract: a payload length selects the draw's length; arrange the subject and `build_fill_bytes_payload` with `length: Some(48)`, differing from the builder's default; act `source.fill_bytes(FillBytesParams, payload)`; assert `success.bytes.expose().len()` equals `48`
    * `[✅]`   `fill_bytes_returns_an_empty_draw_for_a_zero_length`: contract: a zero length takes the drawn branch with an empty buffer; arrange the subject and `build_fill_bytes_payload` with `length: Some(0)`; act `source.fill_bytes(FillBytesParams, payload)`; assert the call returns `Ok` and `success.bytes.expose().is_empty()`
    * `[✅]`   `fill_bytes_draws_pairwise_distinct_values_across_repeated_draws`: contract: draws from the generator do not repeat (CR-05); arrange the subject and an empty `HashSet<Vec<u8>>`; act `source.fill_bytes(FillBytesParams, build_fill_bytes_payload(Default::default()))` sixteen times, inserting a copy of each exposed draw into the set; assert the set holds sixteen entries
    * `[✅]`   `fill_bytes_fills_a_draw_with_more_than_one_distinct_byte_value`: contract: a draw is filled by the generator rather than left at its zero initialization (CR-05); arrange the subject and `build_fill_bytes_payload` with `length: Some(1024)`; act `source.fill_bytes(FillBytesParams, payload)`; assert the `HashSet<u8>` of the exposed draw's bytes holds more than one value
    * `[✅]`   `os_random_source_declares_its_adapter_and_interface_versions`: contract: the concrete's declaration names its adapter version and the interface version it implements; arrange nothing; act read `OsRandomSource::DECLARATION`; assert `adapter_version` equals `1` and `interface_version` equals `RANDOM_SOURCE_INTERFACE_VERSION`
    * `[✅]`   Every test block carries the full `Contract`, `Arrange`, `Act`, `Assert` header and the inline markers

  * `[✅]`   `construction`
    * `[✅]`   `OsRandomSource::try_new` is the concrete's only producer, and its only caller is the randomness factory, which reads `OsRandomSource::DECLARATION` before constructing and returns the concrete to consumers as `Box<dyn IRandomSourceAdapter>` beside its declaration

  * `[✅]`   `adapters/random/src/os/mod.rs`
    * `[✅]`   Module declarations: `mod interface;`, `pub(crate) mod provides;`, and `#[cfg(test)] mod test;`
    * `[✅]`   `impl OsRandomSource` with `pub const DECLARATION: RandomSourceDeclaration` as the interaction spec states, and `pub fn try_new(_params: OsRandomSourceConstructorParams) -> OsRandomSourceTryNewReturn` returning `Ok(OsRandomSource)`
    * `[✅]`   `impl IRandomSourceAdapter for OsRandomSource` with `fn fill_bytes(&self, _params: FillBytesParams, payload: FillBytesPayload) -> FillBytesReturn`, which allocates `vec![0u8; payload.length]`, calls `getrandom::fill` on it, moves the buffer into a `Secret` by `let Ok(bytes) = Secret::try_new(SecretConstructorParams { value: buffer });`, and then matches the fill result into the two branches of the interaction spec
    * `[✅]`   Imports the factory's names from `crate::factory::provides`, `Secret` and `SecretConstructorParams` from `domain`, and this module's types from `interface`
    * `[✅]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[✅]`   `adapters/random/src/os/provides.rs`
    * `[✅]`   `pub(crate) use super::interface::*;`, nothing else, so the concrete is visible to the crate's factory and to nothing outside the crate

  * `[✅]`   `directionality`
    * `[✅]`   `os` depends on the `factory` module's surface through `crate::factory::provides`, on `domain`, and on `getrandom`; the `factory` module depends on `domain` and on `os`'s error type through `crate::os::provides`; among repository crates the crate depends on `crates/domain` alone, inward; nothing depends on the crate yet
    * `[✅]`   The mutual dependency between the `factory` module and `os` is the family form's recorded cycle: a concrete implements the factory's trait, the factory's error enum carries the concrete's error, and the factory function constructs the concrete

  * `[✅]`   `requirements`
    * `[✅]`   `adapters/random/Cargo.toml` carries exactly the tables and keys stated above, and `getrandom` is named nowhere in the crate outside `adapters/random/src/os`
    * `[✅]`   `cargo check --all-targets --all-features` and `cargo fmt --check` complete without error; `cargo clippy --all-targets --all-features` reports nothing beyond the library target's unused-item warnings for the `os` concrete, which `random/factory` resolves by constructing the concrete
    * `[✅]`   `fill_bytes_returns_the_number_of_bytes_requested` passes
    * `[✅]`   `fill_bytes_returns_an_empty_draw_for_a_zero_length` passes
    * `[✅]`   `fill_bytes_draws_pairwise_distinct_values_across_repeated_draws` passes (CR-05, the source's non-repetition)
    * `[✅]`   `fill_bytes_fills_a_draw_with_more_than_one_distinct_byte_value` passes (CR-05, the source fills what it is asked to fill)
    * `[✅]`   `os_random_source_declares_its_adapter_and_interface_versions` passes
    * `[✅]`   A generator failure is returned as `FillBytesErrorReturn::OperatingSystem` holding `OsRandomSourceFillBytesErrorReturn::OperatingSystem` with the `getrandom::Error` unchanged, fixed by the error arm's type; the failure branch has no unit test, since the operating system's generator cannot be driven to fail from a test and the vendor is not mocked
    * `[✅]`   Code outside `adapters/random` naming `OsRandomSource` or anything under `os` fails to compile; the crate's public surface is the `factory` module's `provides`

* `[✅]`   `random/factory` **Randomness factory constructing the concrete a configuration names and returning it behind the family's trait with its declaration; carries the family's integration test and the grouping's commit**

  * `[✅]`   `objective`
    * `[✅]`   Problem: a consumer obtains a randomness source only through the family's generic surface, never by naming a concrete, and the composition reads what was constructed from its declaration (CR-05; Composition Boundary)
    * `[✅]`   Functional: given a requested `RandomSourceKind`, the factory constructs the matching concrete and returns it as `Box<dyn IRandomSourceAdapter>` together with that concrete's declaration
    * `[✅]`   Functional: a concrete's constructor error is returned unchanged in the factory's error arm, one variant per concrete
    * `[✅]`   Functional: a source obtained from the factory draws the requested number of random bytes through the family's trait
    * `[✅]`   Non-functional: adding a concrete is its module, its variant in the family's error enums, and its branch here; no consumer changes

  * `[✅]`   `role`
    * `[✅]`   Adapter family factory: the implementation of the `factory` module, which is the randomness family's construction point and the crate's public surface
    * `[✅]`   Does not decode or validate the requested kind; the configuration registry decodes and validates configuration and hands the factory a typed `RandomSourceKind`
    * `[✅]`   Does not admit against upstream declarations; the randomness family consumes no other family
    * `[✅]`   Does not draw bytes; drawing is the concrete's
    * `[✅]`   Carries the family's integration test and the commit for the workspace and discipline bootstrap milestone

  * `[✅]`   `module`
    * `[✅]`   Bounded context: the `factory` module of `adapters/random`, holding the factory function, its deps, params, payload, and return types, its signature type, its function mock and builders, and the crate's integration test under `adapters/random/tests`
    * `[✅]`   Outside: every concrete's behavior, the decoding of configuration, and every consumer of the family

  * `[✅]`   `deps`
    * `[✅]`   The `os` concrete, through `crate::os::provides`: `OsRandomSource`, `OsRandomSourceConstructorParams`, `OsRandomSource::try_new`, and `OsRandomSource::DECLARATION`; the factory constructs its concretes, the family form's recorded cycle
    * `[✅]`   The `factory` module's own interface: `IRandomSourceAdapter`, `RandomSourceKind`, and `RandomSourceDeclaration`
    * `[✅]`   `core::convert::Infallible`, standard library, the operating-system constructor's error carried in the factory's error arm
    * `[✅]`   `std::collections::HashSet`, standard library, in the integration test only
    * `[✅]`   No new external crate; `adapters/random/Cargo.toml` is unchanged

  * `[✅]`   `context_slice`
    * `[✅]`   From `os`: `OsRandomSource::try_new(OsRandomSourceConstructorParams) -> Result<OsRandomSource, Infallible>`, the inherent constant `OsRandomSource::DECLARATION: RandomSourceDeclaration`, and `OsRandomSource`'s implementation of `IRandomSourceAdapter`

  * `[✅]`   `adapters/random/src/factory/interface.rs`
    * `[✅]`   `CreateRandomSourceDeps`, the fieldless struct `pub struct CreateRandomSourceDeps;`
    * `[✅]`   `CreateRandomSourceParams`, a struct with `pub kind: RandomSourceKind`, the selection of the concrete to construct
    * `[✅]`   `CreateRandomSourcePayload`, the fieldless struct `pub struct CreateRandomSourcePayload;`, since the factory operates on no data
    * `[✅]`   `CreateRandomSourceSuccessReturn`, a struct with `pub adapter: Box<dyn IRandomSourceAdapter>` and `pub declaration: RandomSourceDeclaration`
    * `[✅]`   `CreateRandomSourceErrorReturn`, an enum with the one variant `OperatingSystem(Infallible)`, the operating-system concrete's constructor error carried unchanged; each concrete's constructor error is its own variant
    * `[✅]`   `CreateRandomSourceReturn`, the type alias `Result<CreateRandomSourceSuccessReturn, CreateRandomSourceErrorReturn>`
    * `[✅]`   `CreateRandomSourceFn`, the type alias `fn(&CreateRandomSourceDeps, CreateRandomSourceParams, CreateRandomSourcePayload) -> CreateRandomSourceReturn`
    * `[✅]`   Adds the import of `core::convert::Infallible`; every item `random/os` authored in this file is unchanged

  * `[✅]`   `adapters/random/src/factory/interaction.spec.md`
    * `[✅]`   `create_random_source`, operating system: condition: `params.kind` is `RandomSourceKind::OperatingSystem`; decision: a `match` on `params.kind`; dependency call: `OsRandomSource::try_new(OsRandomSourceConstructorParams)`, exactly once; outcome: `Ok(CreateRandomSourceSuccessReturn { adapter: Box::new(source), declaration: OsRandomSource::DECLARATION })`
    * `[✅]`   The operating-system constructor's error arm is uninhabited, so its success is destructured irrefutably and that branch has no failure outcome; `CreateRandomSourceErrorReturn::OperatingSystem` carries its error type in the return union
    * `[✅]`   `params.kind` selects the concrete; `deps` and `payload` carry nothing and are not read; the `match` is exhaustive over `RandomSourceKind`, so a kind with no branch fails to compile

  * `[✅]`   `adapters/random/src/factory/mock.rs`
    * `[✅]`   `CreateRandomSourceParamsOverrides`, `#[derive(Default)]`, one field `pub kind: Option<RandomSourceKind>`; `build_create_random_source_params(overrides: CreateRandomSourceParamsOverrides) -> CreateRandomSourceParams`, the kind defaulting to `RandomSourceKind::OperatingSystem`
    * `[✅]`   `CreateRandomSourceSuccessReturnOverrides`, `#[derive(Default)]`, fields `pub adapter: Option<Box<dyn IRandomSourceAdapter>>` and `pub declaration: Option<RandomSourceDeclaration>`; `build_create_random_source_success_return(overrides: CreateRandomSourceSuccessReturnOverrides) -> CreateRandomSourceSuccessReturn`, the adapter defaulting to `Box::new(MockIRandomSourceAdapter)` and the declaration to `build_random_source_declaration(Default::default())`
    * `[✅]`   `mock_create_random_source(_deps: &CreateRandomSourceDeps, _params: CreateRandomSourceParams, _payload: CreateRandomSourcePayload) -> CreateRandomSourceReturn`, returning `Ok(build_create_random_source_success_return(Default::default()))`
    * `[✅]`   No builder for the fieldless `CreateRandomSourceDeps` and `CreateRandomSourcePayload`, used by their production values, or for the enum `CreateRandomSourceErrorReturn`; every symbol `random/os` authored in this file is unchanged

  * `[✅]`   `adapters/random/src/factory/test.rs`
    * `[✅]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`; imports `create_random_source` from `super`, `CreateRandomSourceDeps`, `CreateRandomSourcePayload`, `RandomSourceKind`, and `RANDOM_SOURCE_INTERFACE_VERSION` from `super::interface`, and `build_create_random_source_params` and `CreateRandomSourceParamsOverrides` from `super::mock`
    * `[✅]`   `create_random_source_returns_the_operating_system_source_for_its_kind`: contract: the operating-system kind returns `Ok` with the operating-system concrete's declaration; arrange `build_create_random_source_params` with `kind: Some(RandomSourceKind::OperatingSystem)`; act `create_random_source(&CreateRandomSourceDeps, params, CreateRandomSourcePayload)`, unpacked by `let Ok(success) = … else { panic!(…) };`; assert `success.declaration.adapter_version` equals `1` and `success.declaration.interface_version` equals `RANDOM_SOURCE_INTERFACE_VERSION`
    * `[✅]`   The test block carries the full `Contract`, `Arrange`, `Act`, `Assert` header and the inline markers

  * `[✅]`   `construction`
    * `[✅]`   The composition root calls `create_random_source` with `&CreateRandomSourceDeps`, `CreateRandomSourceParams` holding the configured `RandomSourceKind`, and `CreateRandomSourcePayload`, and places the returned `Box<dyn IRandomSourceAdapter>` in each consumer's deps; no consumer constructs a concrete

  * `[✅]`   `adapters/random/src/factory/mod.rs`
    * `[✅]`   Adds `#[cfg(test)] mod test;` to the wiring `random/os` authored
    * `[✅]`   `pub fn create_random_source(_deps: &CreateRandomSourceDeps, params: CreateRandomSourceParams, _payload: CreateRandomSourcePayload) -> CreateRandomSourceReturn`, a `match` on `params.kind` whose `RandomSourceKind::OperatingSystem` arm binds the concrete by `let Ok(source) = OsRandomSource::try_new(OsRandomSourceConstructorParams);` and returns `Ok(CreateRandomSourceSuccessReturn { adapter: Box::new(source), declaration: OsRandomSource::DECLARATION })`
    * `[✅]`   Imports `OsRandomSource` and `OsRandomSourceConstructorParams` from `crate::os::provides`, and this module's types from `interface`
    * `[✅]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[✅]`   `adapters/random/src/factory/provides.rs`
    * `[✅]`   Adds `pub use super::create_random_source;` to the re-exports `random/os` authored

  * `[✅]`   `adapters/random/tests/integration_test.rs`
    * `[✅]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`; imports `create_random_source`, `CreateRandomSourceDeps`, `CreateRandomSourcePayload`, `RandomSourceKind`, `build_create_random_source_params`, `CreateRandomSourceParamsOverrides`, `FillBytesParams`, `build_fill_bytes_payload`, and `FillBytesPayloadOverrides` from `random`, and `HashSet` from `std::collections`; the builders are reached through the crate's `mocks` feature, which the workspace's test and check commands enable with `--all-features`
    * `[✅]`   `a_source_from_the_factory_draws_random_bytes_through_the_family_trait`: contract: the factory's operating-system source, used only through `Box<dyn IRandomSourceAdapter>`, returns a draw of the requested length filled by the generator; arrange `build_create_random_source_params` with `kind: Some(RandomSourceKind::OperatingSystem)` and `build_fill_bytes_payload` with `length: Some(64)`; act `create_random_source` and then `fill_bytes` on the returned adapter, each unpacked by `let Ok(…) = … else { panic!(…) };`; assert the exposed draw's length equals `64` and the `HashSet<u8>` of its bytes holds more than one value
    * `[✅]`   The test block carries the full `Contract`, `Arrange`, `Act`, `Assert` header and the inline markers; nothing is mocked, since the operating system's generator is the outer edge

  * `[✅]`   `directionality`
    * `[✅]`   The `factory` module depends on `os` through `crate::os::provides` and on its own interface; `os` depends on the `factory` module's surface, the family form's recorded cycle; the crate's public surface is the `factory` module's `provides`; nothing depends on the crate yet

  * `[✅]`   `requirements`
    * `[✅]`   `create_random_source_returns_the_operating_system_source_for_its_kind` passes
    * `[✅]`   `a_source_from_the_factory_draws_random_bytes_through_the_family_trait` passes (CR-05, a production draw passes through the factory's surface)
    * `[✅]`   `cargo check --workspace --all-targets --all-features`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `cargo fmt --all --check` complete without error or warning in every target, the `os` concrete's unused-item warnings having no remaining cause
    * `[✅]`   `getrandom` is named nowhere outside `adapters/random/src/os`, and no code outside `adapters/random` can name `OsRandomSource`
    * `[✅]`   The workspace and discipline bootstrap milestone's exit holds: the workspace, `crates/domain`, and `adapters/random` build and pass the Rust CI definition on Windows, macOS, and Linux

  * `[✅]`   **Commit** `feat(foundation): workspace, Rust CI, secret type, and randomness family`
    * `[✅]`   Structural: the virtual workspace manifest with its lint table and `librqbit` overlay, `rust-toolchain.toml`, `deny.toml`, `.gitignore`, `.github/workflows/rust.yml`, the `domain` crate with its `secret` module, and the `random` crate with its `factory` module and `os` concrete
    * `[✅]`   Behavioral: secrets zeroize on drop and cannot be formatted, cloned, or copied; random bytes are drawn from the operating system's generator into a `Secret` through the family's factory
    * `[✅]`   Contract: `Secret` with `try_new` and `expose`; `IRandomSourceAdapter` with `fill_bytes`, its params, payload, and return types, `RandomSourceDeclaration`, and `RANDOM_SOURCE_INTERFACE_VERSION`; `create_random_source` with its signature and return types

* **Cryptographic validation harness**

## Pairing adapters and key derivation

* `[ ]`   `pairing/bn254_arkworks` **BN254 pairing concrete on arkworks with EIP-196 and EIP-197 encodings; creates the `adapters/pairing` crate and authors the pairing family's generic interface, the scalar sampling bound, the declaration, and the mock**

  * `[ ]`   `objective`
    * `[ ]`   Problem: the credential KEM, the envelope, and the delivery proof compute in a Type-3 pairing group whose curve the launch chain's precompiles dictate, so every group operation, subgroup check, pairing-product check, and precompile encoding passes through one repo-owned interface and no module outside a concrete names a curve library (CR-10)
    * `[ ]`   Functional: the family's generic interface exposes the generators of both source groups, addition, scalar multiplication, and multi-scalar multiplication in either source group, the pairing-product check, and decoding and encoding of group elements and scalars in the target chain's precompile format
    * `[ ]`   Functional: the group-element and scalar types are associated types of the generic interface, so a consumer names them through the family without knowing which concrete produced them
    * `[ ]`   Functional: every group element and scalar a concrete accepts from bytes is decoded by a fallible constructor that rejects a wrong length, a non-canonical field element, a point off the curve, and a point outside the prime-order subgroup, so no element that fails a check exists as a value
    * `[ ]`   Functional: a scalar type is sampled from uniform bytes the randomness family drew, by reduction of a fixed-width input modulo the group order, and the sampled scalar is returned inside a `Secret`
    * `[ ]`   Functional: every concrete declares its curve, whether the target chain's verifier has second-group arithmetic, its precompile encoding, its adapter version, and the interface version it implements, readable before any instance exists
    * `[ ]`   Functional: the BN254 concrete encodes a first-group point as EIP-196's 64 bytes and a second-group point as EIP-197's 128 bytes, each coordinate a 32-byte big-endian integer, the second-group coordinates imaginary part first, and the point at infinity as all zero bytes; a scalar is 32 big-endian bytes
    * `[ ]`   Non-functional: `ark-bn254`, `ark-ec`, and `ark-ff` are named only inside `adapters/pairing/src/bn254_arkworks`; the crate depends on no repository crate but `domain` at runtime

  * `[ ]`   `role`
    * `[ ]`   Adapter: the pairing family's first concrete, and the first source file that requires the family's generic interface, the scalar sampling bound, the declaration, and the family's mock, which it authors in the family's `factory` module as its producers
    * `[ ]`   Creates `factory/mod.rs` with the module wiring alone and `factory/provides.rs` with the interface and mock surface alone; the factory function, its types, its interaction spec, its unit test, its re-export, and the family's integration test are `pairing/factory`'s
    * `[ ]`   Does not author the BN254 `halo2curves` concrete or either BLS12-381 concrete; each is its own node beneath this factory, and each adds its variants to the declaration's enums
    * `[ ]`   Does not hash to a group or to a scalar; `hash-to-scalar/keccak256` maps domain-tagged bytes to a scalar through this family's scalar type
    * `[ ]`   Does not draw randomness; a consumer draws `UNIFORM_BYTES_LENGTH` bytes through the randomness family and hands the draw to the sampling bound
    * `[ ]`   Does not compute in the target group except through the pairing-product check
    * `[ ]`   Does not carry a commit

  * `[ ]`   `module`
    * `[ ]`   Bounded context: the `adapters/pairing` crate's `factory` module, holding the family's generic trait with its associated types and methods, every method's params, payload, success, error, and return types, the scalar sampling bound with its types, the declaration and its enums, the interface version, and the family's mock; and its private `bn254_arkworks` concrete, holding the adapter over arkworks' BN254, its scalar and group-element types over the library's elements, its constructor params, and the builder defaults for its owned types
    * `[ ]`   Creates the crate at `adapters/pairing`, admitted by the workspace's `adapters/*` glob with no edit to the root manifest
    * `[ ]`   Outside: which scalars are secret and who produces them, hashing to a scalar, the KEM, envelope, and proof algebra built on the family, the Solidity mirror of the encodings, and the factory's selection of a concrete

  * `[ ]`   `deps`
    * `[ ]`   `domain`, `crates/domain`, protocol and domain ring, path dependency; supplies `Secret` and `SecretConstructorParams`, the wrapper for uniform input, sampled scalars, and encoded scalars; direction inward, adapter ring on domain ring
    * `[ ]`   `domain` with its `mocks` feature, as a dev-dependency and through this crate's `mocks` feature; supplies `build_secret` and `SecretConstructorParamsOverrides`
    * `[ ]`   `zeroize` `1.9.0`, external crate, Apache-2.0 OR MIT, runtime dependency; supplies the `Zeroize` trait the sampling bound requires and the scalar implements; zeroization is a property of the type, not an external touchpoint, so no adapter wraps it
    * `[ ]`   `ark-bn254` `0.6.0`, `ark-ec` `0.6.0`, and `ark-ff` `0.6.0`, external crates, MIT OR Apache-2.0, runtime dependencies named only in `bn254_arkworks`; supply the curve, the pairing, the group arithmetic, and the field arithmetic
    * `[ ]`   `random`, `adapters/random`, adapter ring, dev-dependency with its `mocks` feature; supplies `create_random_source` and `fill_bytes` for the in-range draw test, the dependency map's edge from `random/factory` to this node; nothing at runtime
    * `[ ]`   `hex` `0.4.3`, external crate, MIT OR Apache-2.0, dev-dependency only; supplies `hex::decode` for the test vectors
    * `[ ]`   `core::convert::Infallible`, standard library, the error arm of every method that has no failure; `core::marker::PhantomData`, standard library, in `factory/mock.rs` only
    * `[ ]`   No reverse dependency; nothing depends on this crate yet

  * `[ ]`   `context_slice`
    * `[ ]`   From `domain`: `Secret::try_new(SecretConstructorParams { value })` returning `Result<Secret<T>, Infallible>`, and `Secret::expose(&self) -> &T`
    * `[ ]`   From `domain`'s mocks: `build_secret(SecretConstructorParamsOverrides<T>) -> Secret<T>` for `T: Zeroize + Default`
    * `[ ]`   From `zeroize`: the `Zeroize` trait's `zeroize(&mut self)`, and its implementation for `Vec<Z: Zeroize>`
    * `[ ]`   From `ark-bn254`: `Bn254`, `Fq`, `Fq2` with its public fields `c0` and `c1` and `Fq2::new(c0, c1)`, `Fr`, `G1Affine`, `G1Projective`, `G2Affine`, and `G2Projective`
    * `[ ]`   From `ark-ec`: `AffineRepr` for `generator()` and `xy()`; the short-Weierstrass affine `identity()`, `new_unchecked(x, y)`, `is_on_curve()`, `is_in_correct_subgroup_assuming_on_curve()`, and `get_point_from_x_unchecked(x, greatest)`; the affine `+` and `* Fr` producing projective points; `CurveGroup::into_affine`; `VariableBaseMSM::msm_unchecked(bases, scalars)`; `pairing::Pairing::multi_pairing(a, b)` returning `PairingOutput`
    * `[ ]`   From `ark-ff`: `PrimeField::from_be_bytes_mod_order(&[u8])` and `into_bigint()`, `BigInteger::to_bytes_be()`, `Zero::is_zero()` on `PairingOutput`, and `Fr::from(u64)` and `Fq::from(u64)`
    * `[ ]`   From `random`: `create_random_source`, `CreateRandomSourceDeps`, `CreateRandomSourcePayload`, `build_create_random_source_params`, `CreateRandomSourceParamsOverrides`, `RandomSourceKind`, `IRandomSourceAdapter`, `FillBytesParams`, `build_fill_bytes_payload`, and `FillBytesPayloadOverrides`, in `bn254_arkworks/test.rs` only

  * `[ ]`   `adapters/pairing/Cargo.toml`
    * `[ ]`   `[package]` with `name = "pairing"`, `edition.workspace = true`, `rust-version.workspace = true`, and `publish.workspace = true`; no `version` key
    * `[ ]`   `[dependencies]` with `domain = { path = "../../crates/domain" }`, `zeroize = "1.9.0"`, `ark-bn254 = "0.6.0"`, `ark-ec = "0.6.0"`, and `ark-ff = "0.6.0"`, each with default features
    * `[ ]`   `[dev-dependencies]` with `domain = { path = "../../crates/domain", features = ["mocks"] }`, `random = { path = "../random", features = ["mocks"] }`, and `hex = "0.4.3"`
    * `[ ]`   `[features]` with `mocks = ["domain/mocks"]`
    * `[ ]`   `[lints]` with `workspace = true`
    * `[ ]`   No other table

  * `[ ]`   `adapters/pairing/src/lib.rs`
    * `[ ]`   The crate barrel: `mod factory;`, `mod bn254_arkworks;`, and `pub use factory::provides::*;`, nothing else
    * `[ ]`   Until `factory/mod.rs` and `bn254_arkworks/mod.rs` exist, `cargo check` reports the unresolved modules, which is the RED state for every element below that precedes them

  * `[ ]`   `adapters/pairing/src/factory/interface.rs`
    * `[ ]`   `PAIRING_INTERFACE_VERSION`, a `pub const` of type `u32` with value `1`
    * `[ ]`   `PairingCurve`, an enum with the one variant `Bn254`; `VerifierGroupArithmetic`, an enum with the one variant `FirstGroupOnly`; `PrecompileEncoding`, an enum with the one variant `Eip196Eip197`
    * `[ ]`   `PairingDeclaration`, a struct with `pub curve: PairingCurve`, `pub verifier_group_arithmetic: VerifierGroupArithmetic`, `pub precompile_encoding: PrecompileEncoding`, `pub adapter_version: u32`, and `pub interface_version: u32`
    * `[ ]`   `ISampleUniformScalar`, the sampling bound, `pub trait ISampleUniformScalar: Zeroize + Sized` with `const UNIFORM_BYTES_LENGTH: usize;` and `fn sample_from_uniform_bytes(params: SampleUniformScalarParams, payload: SampleUniformScalarPayload) -> SampleUniformScalarReturn<Self>;`
    * `[ ]`   The sampling bound's types: the fieldless `SampleUniformScalarParams`; `SampleUniformScalarPayload` with `pub uniform: Secret<Vec<u8>>`; `SampleUniformScalarSuccessReturn<S: Zeroize>` with `pub scalar: Secret<S>`; `SampleUniformScalarErrorReturn`, an enum with the one struct variant `WrongLength { expected: usize, actual: usize }`; `SampleUniformScalarReturn<S>`, the alias `Result<SampleUniformScalarSuccessReturn<S>, SampleUniformScalarErrorReturn>`
    * `[ ]`   `IPairingAdapter`, a trait with `type Scalar: ISampleUniformScalar + Clone;`, `type G1: Clone;`, `type G2: Clone;`, and the methods below, each taking `&self`, its params, and its payload, and returning its own return alias
    * `[ ]`   `g1_generator(&self, params: G1GeneratorParams, payload: G1GeneratorPayload) -> G1GeneratorReturn<Self::G1>`: the fieldless `G1GeneratorParams` and `G1GeneratorPayload`; `G1GeneratorSuccessReturn<G>` with `pub point: G`; `G1GeneratorReturn<G>`, the alias `Result<G1GeneratorSuccessReturn<G>, Infallible>`
    * `[ ]`   `g2_generator`, the same shape under the `G2Generator` prefix over `Self::G2`
    * `[ ]`   `add_g1(&self, params: AddG1Params, payload: AddG1Payload<Self::G1>) -> AddG1Return<Self::G1>`: the fieldless `AddG1Params`; `AddG1Payload<G>` with `pub left: G` and `pub right: G`; `AddG1SuccessReturn<G>` with `pub sum: G`; `AddG1Return<G>`, the alias `Result<AddG1SuccessReturn<G>, Infallible>`
    * `[ ]`   `add_g2`, the same shape under the `AddG2` prefix over `Self::G2`
    * `[ ]`   `mul_g1(&self, params: MulG1Params, payload: MulG1Payload<Self::G1, Self::Scalar>) -> MulG1Return<Self::G1>`: the fieldless `MulG1Params`; `MulG1Payload<G, S>` with `pub point: G` and `pub scalar: S`; `MulG1SuccessReturn<G>` with `pub product: G`; `MulG1Return<G>`, the alias `Result<MulG1SuccessReturn<G>, Infallible>`
    * `[ ]`   `mul_g2`, the same shape under the `MulG2` prefix over `Self::G2` and `Self::Scalar`
    * `[ ]`   `msm_g1(&self, params: MsmG1Params, payload: MsmG1Payload<Self::G1, Self::Scalar>) -> MsmG1Return<Self::G1>`: the fieldless `MsmG1Params`; `MsmG1Term<G, S>` with `pub base: G` and `pub scalar: S`; `MsmG1Payload<G, S>` with `pub terms: Vec<MsmG1Term<G, S>>`, so a base and its scalar cannot differ in count; `MsmG1SuccessReturn<G>` with `pub sum: G`; `MsmG1Return<G>`, the alias `Result<MsmG1SuccessReturn<G>, Infallible>`
    * `[ ]`   `msm_g2`, the same shape under the `MsmG2` prefix, with `MsmG2Term<G, S>`, over `Self::G2` and `Self::Scalar`
    * `[ ]`   `pairing_product_is_one(&self, params: PairingProductIsOneParams, payload: PairingProductIsOnePayload<Self::G1, Self::G2>) -> PairingProductIsOneReturn`: the fieldless `PairingProductIsOneParams`; `PairingProductTerm<G1, G2>` with `pub g1: G1` and `pub g2: G2`; `PairingProductIsOnePayload<G1, G2>` with `pub terms: Vec<PairingProductTerm<G1, G2>>`; `PairingProductIsOneSuccessReturn` with `pub is_one: bool`; `PairingProductIsOneReturn`, the alias `Result<PairingProductIsOneSuccessReturn, Infallible>`
    * `[ ]`   `decode_g1(&self, params: DecodeG1Params, payload: &[u8]) -> DecodeG1Return<Self::G1>`, the validating form, its payload the untrusted wire bytes and its narrowing target `Self::G1`: the fieldless `DecodeG1Params`; `DecodeG1SuccessReturn<G>` with `pub point: G`; `DecodeG1ErrorReturn`, an enum with the variants `WrongLength { expected: usize, actual: usize }`, `NonCanonicalCoordinate`, `NotOnCurve`, and `NotInSubgroup`; `DecodeG1Return<G>`, the alias `Result<DecodeG1SuccessReturn<G>, DecodeG1ErrorReturn>`
    * `[ ]`   `decode_g2`, the same shape under the `DecodeG2` prefix over `Self::G2`, with `DecodeG2ErrorReturn` carrying the same four variants
    * `[ ]`   `decode_scalar(&self, params: DecodeScalarParams, payload: &[u8]) -> DecodeScalarReturn<Self::Scalar>`, the validating form: the fieldless `DecodeScalarParams`; `DecodeScalarSuccessReturn<S>` with `pub scalar: S`; `DecodeScalarErrorReturn`, an enum with the variants `WrongLength { expected: usize, actual: usize }` and `NonCanonical`; `DecodeScalarReturn<S>`, the alias `Result<DecodeScalarSuccessReturn<S>, DecodeScalarErrorReturn>`
    * `[ ]`   `encode_g1(&self, params: EncodeG1Params, payload: EncodeG1Payload<Self::G1>) -> EncodeG1Return`: the fieldless `EncodeG1Params`; `EncodeG1Payload<G>` with `pub point: G`; `EncodeG1SuccessReturn` with `pub bytes: Vec<u8>`; `EncodeG1Return`, the alias `Result<EncodeG1SuccessReturn, Infallible>`
    * `[ ]`   `encode_g2`, the same shape under the `EncodeG2` prefix over `Self::G2`
    * `[ ]`   `encode_scalar(&self, params: EncodeScalarParams, payload: EncodeScalarPayload<Self::Scalar>) -> EncodeScalarReturn`: the fieldless `EncodeScalarParams`; `EncodeScalarPayload<S>` with `pub scalar: S`; `EncodeScalarSuccessReturn` with `pub bytes: Secret<Vec<u8>>`, since the scalar encoded may be secret; `EncodeScalarReturn`, the alias `Result<EncodeScalarSuccessReturn, Infallible>`
    * `[ ]`   No derives on any type in this file; imports `domain::Secret`, `zeroize::Zeroize`, and `core::convert::Infallible`; names no vendor and no concrete

  * `[ ]`   `adapters/pairing/src/bn254_arkworks/interface.rs`
    * `[ ]`   `Bn254ArkworksPairing`, the unit struct `pub struct Bn254ArkworksPairing;`, the adapter over arkworks' BN254
    * `[ ]`   `Bn254ArkworksPairingConstructorParams`, the fieldless struct `pub struct Bn254ArkworksPairingConstructorParams;`, the constructor's deps slot
    * `[ ]`   `Bn254ArkworksPairingTryNewReturn`, the alias `Result<Bn254ArkworksPairing, Infallible>`; the error arm is uninhabited because the adapter takes no configuration
    * `[ ]`   `Bn254ArkworksScalar`, a struct with `#[derive(Clone)]` and one field `pub(super) value: ark_bn254::Fr`
    * `[ ]`   `Bn254ArkworksG1`, a struct with `#[derive(Clone)]` and one field `pub(super) value: ark_bn254::G1Affine`
    * `[ ]`   `Bn254ArkworksG2`, a struct with `#[derive(Clone)]` and one field `pub(super) value: ark_bn254::G2Affine`
    * `[ ]`   The `pub(super)` fields admit construction only inside `bn254_arkworks` and its child modules; imports `core::convert::Infallible`; declares nothing else

  * `[ ]`   `adapters/pairing/src/bn254_arkworks/interaction.spec.md`
    * `[ ]`   `Bn254ArkworksPairing::try_new(params: Bn254ArkworksPairingConstructorParams) -> Bn254ArkworksPairingTryNewReturn`: one branch; outcome `Ok(Bn254ArkworksPairing)`; the error arm has no branch
    * `[ ]`   `Bn254ArkworksPairing::DECLARATION`: the inherent constant `PairingDeclaration { curve: PairingCurve::Bn254, verifier_group_arithmetic: VerifierGroupArithmetic::FirstGroupOnly, precompile_encoding: PrecompileEncoding::Eip196Eip197, adapter_version: 1, interface_version: PAIRING_INTERFACE_VERSION }`
    * `[ ]`   `g1_generator`, `g2_generator`: one branch each; dependency call `G1Affine::generator()` or `G2Affine::generator()`; outcome `Ok` holding the generator in the owned group type
    * `[ ]`   `add_g1`, `add_g2`: one branch each; dependency call the affine `+` of the two payload points and `into_affine`; outcome `Ok` holding the sum
    * `[ ]`   `mul_g1`, `mul_g2`: one branch each; dependency call the affine point `*` the scalar's `Fr` and `into_affine`; outcome `Ok` holding the product; the payload, holding the scalar, drops at the end of the call and the scalar is zeroized
    * `[ ]`   `msm_g1`, `msm_g2`: one branch each; the terms are split into a `Vec` of affine bases and a `Vec<Fr>` of the same length; dependency call `G1Projective::msm_unchecked` or `G2Projective::msm_unchecked` over them and `into_affine`; the `Vec<Fr>` is zeroized after the call; outcome `Ok` holding the sum; an empty term list yields the identity
    * `[ ]`   `pairing_product_is_one`: one branch; dependency call `Bn254::multi_pairing` over the terms' first-group elements and second-group elements in term order; outcome `Ok(PairingProductIsOneSuccessReturn { is_one })` where `is_one` is the output's `is_zero()`, the identity of the target group in arkworks' additive notation; an empty term list yields `is_one: true`, as EIP-197 does for empty input
    * `[ ]`   `decode_g1`, wrong length: condition `payload.len() != 64`; outcome `Err(DecodeG1ErrorReturn::WrongLength { expected: 64, actual: payload.len() })`
    * `[ ]`   `decode_g1`, non-canonical coordinate: condition either 32-byte half, read by `Fq::from_be_bytes_mod_order`, does not re-encode through `into_bigint().to_bytes_be()` to the same 32 bytes, that is, it is at least the base field modulus; outcome `Err(DecodeG1ErrorReturn::NonCanonicalCoordinate)`
    * `[ ]`   `decode_g1`, identity: condition both coordinates are zero; outcome `Ok` holding `G1Affine::identity()`
    * `[ ]`   `decode_g1`, off the curve: condition `G1Affine::new_unchecked(x, y).is_on_curve()` is false; outcome `Err(DecodeG1ErrorReturn::NotOnCurve)`
    * `[ ]`   `decode_g1`, outside the subgroup: condition `is_in_correct_subgroup_assuming_on_curve()` is false; outcome `Err(DecodeG1ErrorReturn::NotInSubgroup)`; BN254's first group has cofactor one, so no on-curve point takes this branch and it has no unit test, but the check runs on every decode
    * `[ ]`   `decode_g1`, valid: every check passes; outcome `Ok` holding the point
    * `[ ]`   `decode_g2`: the same branches in the same order over 128 bytes read as `x.c1`, `x.c0`, `y.c1`, `y.c0`, each 32 bytes, assembled by `Fq2::new(c0, c1)`, with `DecodeG2ErrorReturn`; the identity is all four coordinates zero; the subgroup branch is reachable, since BN254's second group has a nontrivial cofactor
    * `[ ]`   `decode_scalar`, wrong length: condition `payload.len() != 32`; outcome `Err(DecodeScalarErrorReturn::WrongLength { expected: 32, actual: payload.len() })`
    * `[ ]`   `decode_scalar`, non-canonical: condition the bytes, read by `Fr::from_be_bytes_mod_order`, do not re-encode to the same 32 bytes, that is, they are at least the group order; outcome `Err(DecodeScalarErrorReturn::NonCanonical)`
    * `[ ]`   `decode_scalar`, valid: outcome `Ok` holding the scalar
    * `[ ]`   `encode_g1`: one branch; the identity encodes to 64 zero bytes; any other point encodes `x` then `y`, each `into_bigint().to_bytes_be()`, from `xy()`
    * `[ ]`   `encode_g2`: one branch; the identity encodes to 128 zero bytes; any other point encodes `x.c1`, `x.c0`, `y.c1`, `y.c0`, each 32 bytes big-endian
    * `[ ]`   `encode_scalar`: one branch; outcome `Ok` holding the scalar's 32 big-endian bytes moved into a `Secret`
    * `[ ]`   `Bn254ArkworksScalar::UNIFORM_BYTES_LENGTH`: `64`, twice the byte width of the group order, so the reduction's bias from uniform is below two to the minus two hundred fifty
    * `[ ]`   `Bn254ArkworksScalar::sample_from_uniform_bytes`, wrong length: condition `payload.uniform.expose().len() != 64`; outcome `Err(SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual })`
    * `[ ]`   `Bn254ArkworksScalar::sample_from_uniform_bytes`, sampled: dependency call `Fr::from_be_bytes_mod_order` over the exposed bytes; outcome `Ok` holding the scalar moved into a `Secret`; the payload's `Secret` zeroizes the input when it drops
    * `[ ]`   Zeroization: `Bn254ArkworksScalar` zeroizes its `Fr` through its `Zeroize` implementation and on drop, so every clone a consumer places in a payload is zeroized when the payload drops

  * `[ ]`   `adapters/pairing/src/factory/mock.rs`
    * `[ ]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`
    * `[ ]`   `PairingDeclarationOverrides`, `#[derive(Default)]`, one `Option` per field; `build_pairing_declaration(overrides: PairingDeclarationOverrides) -> PairingDeclaration`, defaulting to `PairingCurve::Bn254`, `VerifierGroupArithmetic::FirstGroupOnly`, `PrecompileEncoding::Eip196Eip197`, `1`, and `PAIRING_INTERFACE_VERSION`
    * `[ ]`   For each generic struct below, an overrides struct named by the type with the suffix `Overrides`, `#[derive(Default)]`, one `Option` per field over the struct's type parameters, and a builder `build_` followed by the type's name in snake case, taking the overrides and returning the production type, each type parameter bounded by `Default`, and by `Zeroize` where the production type requires it; an omitted field takes `Default::default()` of its type parameter unless stated
    * `[ ]`   The generic builders: `G1GeneratorSuccessReturn`, `G2GeneratorSuccessReturn`, `AddG1Payload`, `AddG1SuccessReturn`, `AddG2Payload`, `AddG2SuccessReturn`, `MulG1Payload`, `MulG1SuccessReturn`, `MulG2Payload`, `MulG2SuccessReturn`, `MsmG1Term`, `MsmG1Payload` with `terms` defaulting to an empty `Vec`, `MsmG1SuccessReturn`, `MsmG2Term`, `MsmG2Payload` with `terms` defaulting to an empty `Vec`, `MsmG2SuccessReturn`, `PairingProductTerm`, `PairingProductIsOnePayload` with `terms` defaulting to an empty `Vec`, `DecodeG1SuccessReturn`, `DecodeG2SuccessReturn`, `DecodeScalarSuccessReturn`, `EncodeG1Payload`, `EncodeG2Payload`, `EncodeScalarPayload`, and `SampleUniformScalarSuccessReturn` with `scalar` defaulting to `build_secret(SecretConstructorParamsOverrides::default())`
    * `[ ]`   The non-generic builders: `PairingProductIsOneSuccessReturnOverrides` with `build_pairing_product_is_one_success_return`, `is_one` defaulting to `true`; `EncodeG1SuccessReturnOverrides` with `build_encode_g1_success_return` and `EncodeG2SuccessReturnOverrides` with `build_encode_g2_success_return`, `bytes` defaulting to an empty `Vec`; `EncodeScalarSuccessReturnOverrides` with `build_encode_scalar_success_return`, `bytes` defaulting to `build_secret(SecretConstructorParamsOverrides::default())`; `SampleUniformScalarPayloadOverrides` with `build_sample_uniform_scalar_payload`, `uniform` defaulting to `build_secret` holding `vec![0u8; 64]`
    * `[ ]`   `MockIPairingAdapter<S, G1, G2>`, a struct with `pub scalar: PhantomData<S>`, `pub g1: PhantomData<G1>`, and `pub g2: PhantomData<G2>`, implementing `IPairingAdapter` for `S: ISampleUniformScalar + Clone + Default`, `G1: Clone + Default`, and `G2: Clone + Default` with those as its associated types; every method returns `Ok` holding its success return's builder called with `Default::default()`, the decoders for any payload; a test needing other behavior implements the trait on its own local struct
    * `[ ]`   No builder for the fieldless params and payloads or for the enums; no corruptions type and no invalidator, since no struct this interface owns arrives as untrusted data and the decoders take the untrusted bytes directly
    * `[ ]`   Imports `Secret`, `build_secret`, and `SecretConstructorParamsOverrides` from `domain`, `zeroize::Zeroize`, `core::marker::PhantomData`, and this module's types from `super::interface`

  * `[ ]`   `adapters/pairing/src/bn254_arkworks/mock.rs`
    * `[ ]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`
    * `[ ]`   The builder defaults for the concrete's owned types, which the family's generic builders and `MockIPairingAdapter` read through `Default`: `impl Default for Bn254ArkworksScalar` returning the scalar `Fr::from(1u64)`; `impl Default for Bn254ArkworksG1` returning `G1Affine::generator()`; `impl Default for Bn254ArkworksG2` returning `G2Affine::generator()`
    * `[ ]`   Nothing else; the types are built as real values, so there is no overrides type, invalidator, or mock function here

  * `[ ]`   `adapters/pairing/src/factory/mod.rs`
    * `[ ]`   Module wiring only: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, and `pub mod provides;`, nothing else

  * `[ ]`   `adapters/pairing/src/factory/provides.rs`
    * `[ ]`   `pub use super::interface::*;` and `#[cfg(any(test, feature = "mocks"))] pub use super::mock::*;`, nothing else

  * `[ ]`   `adapters/pairing/src/bn254_arkworks/test.rs`
    * `[ ]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`; imports this module's types from `super::interface`, the family's names and builders from `crate::factory::provides`, `build_secret` and `SecretConstructorParamsOverrides` from `domain`, the `random` names the context slice lists, `hex::decode`, and `ark_bn254::{Fq, Fq2, G2Affine}` with `ark_ec::AffineRepr` and `ark_ff::{BigInteger, PrimeField}` for the non-subgroup vector; each test constructs the subject by `let Ok(pairing) = Bn254ArkworksPairing::try_new(Bn254ArkworksPairingConstructorParams);`, decodes hex by `let Ok(bytes) = decode(…) else { panic!(…) };`, and unpacks each call by `let Ok(…) = … else { panic!(…) };`
    * `[ ]`   The vectors, as hex: the base field modulus `p` = `30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47`; the group order `r` = `30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001`; `r - 1` = `30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000000`; `r - 2` = `30644e72e131a029b85045b68181585d2833e84879b9709143e1f593efffffff`; the EIP-196 first-group generator, 31 zero bytes and `01` followed by 31 zero bytes and `02`; the EIP-197 second-group generator, `198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c2` then `1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed` then `090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b` then `12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa`; the scalars two, three, and five as 32 big-endian bytes
    * `[ ]`   `g1_generator_encodes_to_the_eip_196_generator`: contract: the first-group generator's encoding is EIP-196's; act `encode_g1` over `g1_generator`; assert the bytes equal the first-group generator vector
    * `[ ]`   `g2_generator_encodes_to_the_eip_197_generator`: contract: the second-group generator's encoding is EIP-197's, imaginary parts first; act `encode_g2` over `g2_generator`; assert the bytes equal the second-group generator vector
    * `[ ]`   `decode_g1_round_trips_the_eip_196_generator`: arrange the first-group generator vector; act `decode_g1` then `encode_g1`; assert the bytes equal the vector
    * `[ ]`   `decode_g2_round_trips_the_eip_197_generator`: arrange the second-group generator vector; act `decode_g2` then `encode_g2`; assert the bytes equal the vector
    * `[ ]`   `decode_g1_reads_the_all_zero_encoding_as_the_identity`: arrange 64 zero bytes and the generator; act `decode_g1`, then `add_g1` of the generator and the decoded point, then `encode_g1`; assert the bytes equal the first-group generator vector
    * `[ ]`   `decode_g2_reads_the_all_zero_encoding_as_the_identity`: the same over 128 zero bytes, `add_g2`, and the second-group generator vector
    * `[ ]`   `decode_g1_rejects_a_wrong_length`: arrange 63 bytes; act `decode_g1`; assert `Err(DecodeG1ErrorReturn::WrongLength { expected: 64, actual: 63 })`
    * `[ ]`   `decode_g1_rejects_a_coordinate_equal_to_the_base_field_modulus`: arrange `p` followed by the 32-byte encoding of two; act `decode_g1`; assert `Err(DecodeG1ErrorReturn::NonCanonicalCoordinate)`
    * `[ ]`   `decode_g1_rejects_a_point_off_the_curve`: arrange the 32-byte encodings of one and three; act `decode_g1`; assert `Err(DecodeG1ErrorReturn::NotOnCurve)`
    * `[ ]`   `decode_g2_rejects_a_wrong_length`: arrange 127 bytes; act `decode_g2`; assert `Err(DecodeG2ErrorReturn::WrongLength { expected: 128, actual: 127 })`
    * `[ ]`   `decode_g2_rejects_a_point_off_the_curve`: arrange the second-group generator vector with its last byte `aa` replaced by `ab`; act `decode_g2`; assert `Err(DecodeG2ErrorReturn::NotOnCurve)`
    * `[ ]`   `decode_g2_rejects_a_point_outside_the_subgroup`: arrange the first on-curve point outside the subgroup found by `(1u64..).find_map` over `G2Affine::get_point_from_x_unchecked(Fq2::new(Fq::from(c0), Fq::from(0u64)), false)` filtered by `!is_in_correct_subgroup_assuming_on_curve()`, encoded from `xy()` as `x.c1`, `x.c0`, `y.c1`, `y.c0`, each `into_bigint().to_bytes_be()`; act `decode_g2`; assert `Err(DecodeG2ErrorReturn::NotInSubgroup)`
    * `[ ]`   `decode_scalar_rejects_the_group_order`: arrange `r`; act `decode_scalar`; assert `Err(DecodeScalarErrorReturn::NonCanonical)`
    * `[ ]`   `decode_scalar_round_trips_the_largest_canonical_scalar`: arrange `r - 1`; act `decode_scalar` then `encode_scalar`; assert the exposed bytes equal `r - 1`
    * `[ ]`   `decode_scalar_rejects_a_wrong_length`: arrange 31 bytes; act `decode_scalar`; assert `Err(DecodeScalarErrorReturn::WrongLength { expected: 32, actual: 31 })`
    * `[ ]`   `add_g1_of_the_generator_to_itself_equals_its_multiple_by_two`: arrange the generator and the decoded scalar two, payloads from `build_add_g1_payload` and `build_mul_g1_payload`; act `add_g1` and `mul_g1`, each encoded; assert the two encodings are equal and differ from the generator's
    * `[ ]`   `add_g2_of_the_generator_to_itself_equals_its_multiple_by_two`: the same over the second group
    * `[ ]`   `msm_g1_equals_the_multiple_by_the_sum_of_its_scalars`: arrange terms from `build_msm_g1_term` pairing the generator with two and with three, and the scalar five; act `msm_g1` and `mul_g1` by five, each encoded; assert the encodings are equal
    * `[ ]`   `msm_g2_equals_the_multiple_by_the_sum_of_its_scalars`: the same over the second group
    * `[ ]`   `pairing_product_is_one_for_a_pairing_and_its_inverse`: arrange terms `(g1, g2)` and `(g1 · (r - 1), g2)`; act `pairing_product_is_one`; assert `is_one` is `true`
    * `[ ]`   `pairing_product_is_not_one_for_a_single_generator_pairing`: arrange the one term `(g1, g2)`; act `pairing_product_is_one`; assert `is_one` is `false`
    * `[ ]`   `pairing_product_is_one_across_the_bilinear_exchange`: arrange terms `(g1 · 2, g2)` and `(g1, g2 · (r - 2))`; act `pairing_product_is_one`; assert `is_one` is `true`
    * `[ ]`   `sample_from_uniform_bytes_rejects_a_wrong_length`: arrange `build_sample_uniform_scalar_payload` with `uniform` holding 63 bytes; act `Bn254ArkworksScalar::sample_from_uniform_bytes`; assert `Err(SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual: 63 })`
    * `[ ]`   `sample_from_uniform_bytes_reduces_the_largest_input_into_the_scalar_field`: arrange `uniform` holding 64 bytes of `ff`; act `sample_from_uniform_bytes`, then `encode_scalar` of a clone of the exposed scalar, then `decode_scalar`; assert the decode returns `Ok`, the sampled scalar being below `r`
    * `[ ]`   `sample_from_uniform_bytes_draws_an_in_range_scalar_from_the_operating_system_source`: arrange `create_random_source` for `RandomSourceKind::OperatingSystem` and `fill_bytes` with `length: Some(Bn254ArkworksScalar::UNIFORM_BYTES_LENGTH)`; act `sample_from_uniform_bytes` over the draw, then `encode_scalar` and `decode_scalar`; assert the decode returns `Ok`
    * `[ ]`   `bn254_arkworks_pairing_declares_its_curve_arithmetic_encoding_and_versions`: act read `Bn254ArkworksPairing::DECLARATION`; assert `curve` matches `PairingCurve::Bn254`, `verifier_group_arithmetic` matches `VerifierGroupArithmetic::FirstGroupOnly`, `precompile_encoding` matches `PrecompileEncoding::Eip196Eip197`, `adapter_version` equals `1`, and `interface_version` equals `PAIRING_INTERFACE_VERSION`
    * `[ ]`   Every test block carries the full `Contract`, `Arrange`, `Act`, `Assert` header and the inline markers; every payload is built through its family builder with only the overrides the test depends on

  * `[ ]`   `construction`
    * `[ ]`   `Bn254ArkworksPairing::try_new` is the concrete's only producer, and its only caller is the pairing factory, which reads `Bn254ArkworksPairing::DECLARATION` before constructing
    * `[ ]`   A group element or scalar is produced only by the adapter's generators, arithmetic, and decoders, or by the scalar's sampling bound; no consumer constructs one from library values

  * `[ ]`   `adapters/pairing/src/bn254_arkworks/mod.rs`
    * `[ ]`   Module declarations: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, `pub(crate) mod provides;`, and `#[cfg(test)] mod test;`
    * `[ ]`   `impl Bn254ArkworksPairing` with `pub const DECLARATION: PairingDeclaration` and `pub fn try_new(_params: Bn254ArkworksPairingConstructorParams) -> Bn254ArkworksPairingTryNewReturn` returning `Ok(Bn254ArkworksPairing)`
    * `[ ]`   `impl IPairingAdapter for Bn254ArkworksPairing` with `type Scalar = Bn254ArkworksScalar;`, `type G1 = Bn254ArkworksG1;`, `type G2 = Bn254ArkworksG2;`, and every method realizing its branches in the interaction spec, the decoders checking in the stated order and slicing the payload only after the length check
    * `[ ]`   `impl Zeroize for Bn254ArkworksScalar` calling `self.value.zeroize()`; `impl Drop for Bn254ArkworksScalar` calling `self.value.zeroize()`
    * `[ ]`   `impl ISampleUniformScalar for Bn254ArkworksScalar` with `const UNIFORM_BYTES_LENGTH: usize = 64;` and `sample_from_uniform_bytes` realizing its branches, the scalar moved into a `Secret` by `let Ok(scalar) = Secret::try_new(SecretConstructorParams { value });`
    * `[ ]`   Imports the family's names from `crate::factory::provides`, `Secret` and `SecretConstructorParams` from `domain`, `zeroize::Zeroize`, the arkworks names the context slice lists, and this module's types from `interface`
    * `[ ]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[ ]`   `adapters/pairing/src/bn254_arkworks/provides.rs`
    * `[ ]`   `pub(crate) use super::interface::*;`, nothing else, so the concrete is visible to the crate's factory and to nothing outside the crate

  * `[ ]`   `directionality`
    * `[ ]`   `bn254_arkworks` depends on the `factory` module's surface through `crate::factory::provides`, on `domain`, on `zeroize`, and on the arkworks crates; the `factory` module depends on `domain` and `zeroize` and on no concrete; among repository crates the crate depends on `crates/domain` alone at runtime and on `adapters/random` for tests only, the dependency map's edge; nothing depends on the crate yet
    * `[ ]`   `pairing/factory` adds the family form's recorded cycle when the factory function constructs this concrete

  * `[ ]`   `requirements`
    * `[ ]`   `adapters/pairing/Cargo.toml` carries exactly the tables and keys stated above, and no `ark-` crate is named in the crate outside `adapters/pairing/src/bn254_arkworks`
    * `[ ]`   `cargo check --all-targets --all-features`, `cargo fmt --check`, and `cargo deny check` complete without error; `cargo clippy --all-targets --all-features` reports nothing beyond the library target's unused-item warnings for the `bn254_arkworks` concrete, which `pairing/factory` resolves by constructing the concrete
    * `[ ]`   Every test in `bn254_arkworks/test.rs` passes: the generator encodings match EIP-196 and EIP-197, the decoders round-trip and reject a wrong length, a non-canonical coordinate or scalar, a point off the curve, and a second-group point outside the subgroup, the arithmetic and multi-scalar multiplication agree, the pairing-product check accepts a pairing with its inverse and the bilinear exchange and rejects a lone pairing, and sampling rejects a wrong length and returns a scalar below the group order from a production draw (CR-10 on BN254; CR-05 for the sampled scalar)
    * `[ ]`   Code outside `adapters/pairing` naming `Bn254ArkworksPairing` or anything under `bn254_arkworks` fails to compile; the crate's public surface is the `factory` module's `provides`

* `[ ]`   `pairing/bn254_halo2curves` **BN254 pairing concrete on halo2curves with EIP-196 and EIP-197 encodings, a further concrete beneath the pairing factory**

  * `[ ]`   `objective`
    * `[ ]`   Problem: the harness benchmark compares pairing libraries per curve through the factory, so BN254 needs a second concrete over a second library that satisfies the family's generic interface exactly as the arkworks concrete does (CR-10)
    * `[ ]`   Functional: the concrete implements `IPairingAdapter` over `halo2curves`' BN256, which is BN254, with its own scalar and group-element types over the library's elements as the associated types
    * `[ ]`   Functional: it encodes and decodes a first-group point as EIP-196's 64 bytes and a second-group point as EIP-197's 128 bytes, each coordinate a 32-byte big-endian integer, the second-group coordinates imaginary part first, the point at infinity as all zero bytes, and a scalar as 32 big-endian bytes, rejecting a wrong length, a non-canonical field element, a point off the curve, and a point outside the prime-order subgroup
    * `[ ]`   Functional: its scalar type implements the family's sampling bound, reading the 64 uniform bytes as one big-endian integer reduced modulo the group order, so the same input bytes sample the same scalar under either BN254 concrete
    * `[ ]`   Functional: it declares the BN254 curve, first-group-only arithmetic at the verifier, the EIP-196 and EIP-197 encoding, its adapter version, and the interface version it implements
    * `[ ]`   Non-functional: `halo2curves` is named only inside `adapters/pairing/src/bn254_halo2curves`; the family's interface, declaration, builders, and mock are unchanged

  * `[ ]`   `role`
    * `[ ]`   Adapter: a further concrete of the pairing family, consuming the generic interface, sampling bound, declaration, and builders `pairing/bn254_arkworks` authored in the `factory` module
    * `[ ]`   Adds its module line to `lib.rs` and its dependency line to the crate manifest; edits no file of the `factory` module, since its declaration uses the variants that already exist
    * `[ ]`   Does not select between the BN254 concretes; `pairing/factory` constructs a concrete by the composition's request and `harness-crypto/benchmark` records the default per curve
    * `[ ]`   Does not name, import, or compare against the arkworks concrete
    * `[ ]`   Does not carry a commit

  * `[ ]`   `module`
    * `[ ]`   Bounded context: the private `bn254_halo2curves` module of `adapters/pairing`, holding the adapter over `halo2curves`' BN256, its scalar and group-element types over the library's elements, its constructor params, and the builder defaults for its owned types
    * `[ ]`   Outside: the family's generic interface and declaration, the arkworks concrete, and the factory's selection

  * `[ ]`   `deps`
    * `[ ]`   The `factory` module's surface, through `crate::factory::provides`: `IPairingAdapter` with its associated types and methods and every method's params, payload, success, error, and return types; `ISampleUniformScalar` and its types; `PairingDeclaration`, `PairingCurve`, `VerifierGroupArithmetic`, `PrecompileEncoding`, and `PAIRING_INTERFACE_VERSION`; and, in tests, the family's builders
    * `[ ]`   `domain`, `crates/domain`, path dependency the crate already carries; supplies `Secret` and `SecretConstructorParams`; in tests `build_secret` and `SecretConstructorParamsOverrides`
    * `[ ]`   `zeroize` `1.9.0`, which the crate already carries; supplies the `Zeroize` trait the sampling bound requires and the zeroization of the local 64-byte input copy
    * `[ ]`   `halo2curves` `0.10.0`, external crate, MIT/Apache-2.0, runtime dependency named only in `bn254_halo2curves`, default features; supplies the curve, the pairing, the group arithmetic, the field arithmetic, and multi-scalar multiplication, and re-exports the `ff`, `group`, and `pairing` traits its types implement, so no separate trait crate is pinned
    * `[ ]`   `random` with its `mocks` feature and `hex` `0.4.3`, dev-dependencies the crate already carries; supply the production draw and the test vectors
    * `[ ]`   `core::convert::Infallible`, standard library, the constructor's error arm; `core::hint::black_box`, standard library, which keeps the clearing of a scalar from being removed as a dead store, since `halo2curves`' fields implement no `Zeroize`; `core::iter::successors`, standard library, in `test.rs` only
    * `[ ]`   No reverse dependency; nothing depends on this concrete until `pairing/factory`

  * `[ ]`   `context_slice`
    * `[ ]`   From `halo2curves::bn256`: `Bn256`, `Fq`, `Fq2` with `Fq2::new(c0, c1)` and the accessors `c0()` and `c1()`, `Fr`, `G1`, `G1Affine`, `G2`, and `G2Affine`
    * `[ ]`   From `halo2curves::ff`: `Field` for `ZERO`, `ONE`, `is_zero()`, `square()`, and `sqrt()`; `PrimeField` for `from_repr(repr) -> CtOption<Self>`, which reads 32 little-endian bytes and is none at or above the modulus, and `to_repr()`, the representation built from a `[u8; 32]` by `into()` and read by `as_ref()`; `FromUniformBytes::<64>::from_uniform_bytes(&[u8; 64])`, a little-endian wide reduction
    * `[ ]`   From `halo2curves::group`: `Curve::to_affine`, `Group::is_identity`, `prime::PrimeCurveAffine` for `generator()`, `identity()`, and `to_curve()`, `cofactor::CofactorGroup::is_torsion_free`, and the projective `+` and `* Fr`
    * `[ ]`   From `halo2curves`: `CurveAffine` for `from_xy(x, y) -> CtOption<Self>`, which is none off the curve, `coordinates() -> CtOption<Coordinates<Self>>`, and `b()`; `Coordinates` for `x()` and `y()`; `msm::msm_best(coeffs: &[C::Scalar], bases: &[C]) -> C::Curve`
    * `[ ]`   From `halo2curves::pairing`: `MultiMillerLoop::multi_miller_loop(&[(&G1Affine, &G2Affine)])` on `Bn256`, and `MillerLoopResult::final_exponentiation` returning the target-group element, whose `is_identity()` is the check
    * `[ ]`   A `Choice` becomes a `bool` by `bool::from`, and a `CtOption` becomes an `Option` by `Option::from`

  * `[ ]`   `adapters/pairing/Cargo.toml`
    * `[ ]`   Adds `halo2curves = "0.10.0"`, default features, to `[dependencies]`; every other table and key is unchanged

  * `[ ]`   `adapters/pairing/src/lib.rs`
    * `[ ]`   Adds `mod bn254_halo2curves;` to the barrel; every other line is unchanged
    * `[ ]`   Until `bn254_halo2curves/mod.rs` exists, `cargo check` reports the unresolved module, which is the RED state for every element below that precedes it

  * `[ ]`   `adapters/pairing/src/bn254_halo2curves/interface.rs`
    * `[ ]`   `Bn254Halo2curvesPairing`, the unit struct `pub struct Bn254Halo2curvesPairing;`, the adapter over `halo2curves`' BN256
    * `[ ]`   `Bn254Halo2curvesPairingConstructorParams`, the fieldless struct `pub struct Bn254Halo2curvesPairingConstructorParams;`
    * `[ ]`   `Bn254Halo2curvesPairingTryNewReturn`, the alias `Result<Bn254Halo2curvesPairing, Infallible>`; the error arm is uninhabited because the adapter takes no configuration
    * `[ ]`   `Bn254Halo2curvesScalar`, a struct with `#[derive(Clone)]` and one field `pub(super) value: halo2curves::bn256::Fr`
    * `[ ]`   `Bn254Halo2curvesG1`, a struct with `#[derive(Clone)]` and one field `pub(super) value: halo2curves::bn256::G1Affine`
    * `[ ]`   `Bn254Halo2curvesG2`, a struct with `#[derive(Clone)]` and one field `pub(super) value: halo2curves::bn256::G2Affine`
    * `[ ]`   Imports `core::convert::Infallible`; declares nothing else

  * `[ ]`   `adapters/pairing/src/bn254_halo2curves/interaction.spec.md`
    * `[ ]`   `Bn254Halo2curvesPairing::try_new(params: Bn254Halo2curvesPairingConstructorParams) -> Bn254Halo2curvesPairingTryNewReturn`: one branch; outcome `Ok(Bn254Halo2curvesPairing)`; the error arm has no branch
    * `[ ]`   `Bn254Halo2curvesPairing::DECLARATION`: the inherent constant `PairingDeclaration { curve: PairingCurve::Bn254, verifier_group_arithmetic: VerifierGroupArithmetic::FirstGroupOnly, precompile_encoding: PrecompileEncoding::Eip196Eip197, adapter_version: 1, interface_version: PAIRING_INTERFACE_VERSION }`
    * `[ ]`   `g1_generator`, `g2_generator`: one branch each; dependency call `G1Affine::generator()` or `G2Affine::generator()`; outcome `Ok` holding the generator in the owned group type
    * `[ ]`   `add_g1`, `add_g2`: one branch each; dependency call `to_curve()` on each payload point, the projective `+`, and `to_affine()`; outcome `Ok` holding the sum
    * `[ ]`   `mul_g1`, `mul_g2`: one branch each; dependency call `to_curve()` on the point, the projective `*` the scalar's `Fr`, and `to_affine()`; outcome `Ok` holding the product; the payload, holding the scalar, drops at the end of the call and the scalar is cleared
    * `[ ]`   `msm_g1`, `msm_g2`: one branch each; the terms are split into a `Vec` of affine bases and a `Vec<Fr>` of the same length; dependency call `msm_best(&scalars, &bases)` and `to_affine()`; every element of the `Vec<Fr>` is then set to `Fr::ZERO` and the vector passed to `black_box`; outcome `Ok` holding the sum; an empty term list yields the identity
    * `[ ]`   `pairing_product_is_one`: one branch; dependency call `Bn256::multi_miller_loop` over the terms as a `Vec<(&G1Affine, &G2Affine)>` in term order, then `final_exponentiation()`; outcome `Ok(PairingProductIsOneSuccessReturn { is_one })` where `is_one` is `bool::from(is_identity())`; an empty term list yields `is_one: true`, as EIP-197 does for empty input
    * `[ ]`   `decode_g1`, wrong length: condition `<[u8; 64]>::try_from(payload)` fails; outcome `Err(DecodeG1ErrorReturn::WrongLength { expected: 64, actual: payload.len() })`
    * `[ ]`   `decode_g1`, non-canonical coordinate: condition either 32-byte half, reversed to little-endian and read by `Fq::from_repr`, is none, that is, it is at least the base field modulus; outcome `Err(DecodeG1ErrorReturn::NonCanonicalCoordinate)`
    * `[ ]`   `decode_g1`, identity: condition both coordinates are zero; outcome `Ok` holding `G1Affine::identity()`
    * `[ ]`   `decode_g1`, off the curve: condition `G1Affine::from_xy(x, y)` is none; outcome `Err(DecodeG1ErrorReturn::NotOnCurve)`
    * `[ ]`   `decode_g1`, outside the subgroup: condition `bool::from(point.to_curve().is_torsion_free())` is false; outcome `Err(DecodeG1ErrorReturn::NotInSubgroup)`; BN254's first group has cofactor one, so no on-curve point takes this branch and it has no unit test, but the check runs on every decode
    * `[ ]`   `decode_g1`, valid: every check passes; outcome `Ok` holding the point
    * `[ ]`   `decode_g2`: the same branches in the same order over 128 bytes read as `x.c1`, `x.c0`, `y.c1`, `y.c0`, each 32 bytes big-endian, assembled by `Fq2::new(c0, c1)`, with `DecodeG2ErrorReturn`; the identity is all four coordinates zero; the subgroup branch is reachable, since BN254's second group has a nontrivial cofactor
    * `[ ]`   `decode_scalar`, wrong length: condition `<[u8; 32]>::try_from(payload)` fails; outcome `Err(DecodeScalarErrorReturn::WrongLength { expected: 32, actual: payload.len() })`
    * `[ ]`   `decode_scalar`, non-canonical: condition the bytes, reversed to little-endian and read by `Fr::from_repr`, are none, that is, they are at least the group order; outcome `Err(DecodeScalarErrorReturn::NonCanonical)`
    * `[ ]`   `decode_scalar`, valid: outcome `Ok` holding the scalar
    * `[ ]`   `encode_g1`: one branch; a point whose `coordinates()` is none, the identity, encodes to 64 zero bytes; any other point encodes `x()` then `y()`, each `to_repr()` reversed to big-endian
    * `[ ]`   `encode_g2`: one branch; the identity encodes to 128 zero bytes; any other point encodes `x().c1()`, `x().c0()`, `y().c1()`, `y().c0()`, each `to_repr()` reversed to big-endian
    * `[ ]`   `encode_scalar`: one branch; outcome `Ok` holding the scalar's `to_repr()` reversed to 32 big-endian bytes, moved into a `Secret`
    * `[ ]`   `Bn254Halo2curvesScalar::UNIFORM_BYTES_LENGTH`: `64`, twice the byte width of the group order
    * `[ ]`   `Bn254Halo2curvesScalar::sample_from_uniform_bytes`, wrong length: condition `<&[u8; 64]>::try_from(payload.uniform.expose().as_slice())` fails; outcome `Err(SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual })`
    * `[ ]`   `Bn254Halo2curvesScalar::sample_from_uniform_bytes`, sampled: the 64 bytes are copied into a local `[u8; 64]` and reversed, so the big-endian integer the arkworks concrete reads is the little-endian integer `halo2curves` reads; dependency call `Fr::from_uniform_bytes` over the copy, which is then zeroized; outcome `Ok` holding the scalar moved into a `Secret`; the payload's `Secret` zeroizes the input when it drops
    * `[ ]`   Clearing: `halo2curves`' `Fr` implements no `Zeroize`, so `Bn254Halo2curvesScalar`'s `Zeroize` implementation and its `Drop` set `value` to `Fr::ZERO` and pass `&self.value` to `black_box`; every clone a consumer places in a payload is cleared when the payload drops

  * `[ ]`   `adapters/pairing/src/bn254_halo2curves/mock.rs`
    * `[ ]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`
    * `[ ]`   The builder defaults for the concrete's owned types, which the family's generic builders and `MockIPairingAdapter` read through `Default`: `impl Default for Bn254Halo2curvesScalar` returning `Fr::ONE`; `impl Default for Bn254Halo2curvesG1` returning `G1Affine::generator()`; `impl Default for Bn254Halo2curvesG2` returning `G2Affine::generator()`
    * `[ ]`   Nothing else; the types are built as real values, so there is no overrides type, invalidator, or mock function here

  * `[ ]`   `adapters/pairing/src/bn254_halo2curves/test.rs`
    * `[ ]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`; imports this module's types from `super::interface`, the family's names and builders from `crate::factory::provides`, `build_secret` and `SecretConstructorParamsOverrides` from `domain`, the `random` names `pairing/bn254_arkworks`'s test imports, `hex::decode`, `core::iter::successors`, and, for the non-subgroup vector, `halo2curves::bn256::{Fq, Fq2, G2Affine}`, `halo2curves::ff::{Field, PrimeField}`, `halo2curves::group::{cofactor::CofactorGroup, prime::PrimeCurveAffine}`, and `halo2curves::CurveAffine`; each test constructs the subject by `let Ok(pairing) = Bn254Halo2curvesPairing::try_new(Bn254Halo2curvesPairingConstructorParams);`, decodes hex by `let Ok(bytes) = decode(…) else { panic!(…) };`, and unpacks each call by `let Ok(…) = … else { panic!(…) };`
    * `[ ]`   The vectors are `pairing/bn254_arkworks`'s, restated here as hex: `p` = `30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47`; `r` = `30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001`; `r - 1` = `30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000000`; `r - 2` = `30644e72e131a029b85045b68181585d2833e84879b9709143e1f593efffffff`; the EIP-196 first-group generator, 31 zero bytes and `01` followed by 31 zero bytes and `02`; the EIP-197 second-group generator, `198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c2` then `1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed` then `090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b` then `12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa`; the scalars two, three, and five as 32 big-endian bytes
    * `[ ]`   `g1_generator_encodes_to_the_eip_196_generator`: contract: the first-group generator's encoding is EIP-196's; act `encode_g1` over `g1_generator`; assert the bytes equal the first-group generator vector
    * `[ ]`   `g2_generator_encodes_to_the_eip_197_generator`: contract: the second-group generator's encoding is EIP-197's, imaginary parts first; act `encode_g2` over `g2_generator`; assert the bytes equal the second-group generator vector
    * `[ ]`   `decode_g1_round_trips_the_eip_196_generator`: arrange the first-group generator vector; act `decode_g1` then `encode_g1`; assert the bytes equal the vector
    * `[ ]`   `decode_g2_round_trips_the_eip_197_generator`: arrange the second-group generator vector; act `decode_g2` then `encode_g2`; assert the bytes equal the vector
    * `[ ]`   `decode_g1_reads_the_all_zero_encoding_as_the_identity`: arrange 64 zero bytes and the generator; act `decode_g1`, then `add_g1` of the generator and the decoded point, then `encode_g1`; assert the bytes equal the first-group generator vector
    * `[ ]`   `decode_g2_reads_the_all_zero_encoding_as_the_identity`: the same over 128 zero bytes, `add_g2`, and the second-group generator vector
    * `[ ]`   `encode_g1_writes_the_identity_as_all_zero_bytes`: arrange the identity as `msm_g1` over an empty `build_msm_g1_payload`; act `encode_g1`; assert the bytes equal 64 zero bytes
    * `[ ]`   `decode_g1_rejects_a_wrong_length`: arrange 63 bytes; act `decode_g1`; assert `Err(DecodeG1ErrorReturn::WrongLength { expected: 64, actual: 63 })`
    * `[ ]`   `decode_g1_rejects_a_coordinate_equal_to_the_base_field_modulus`: arrange `p` followed by the 32-byte encoding of two; act `decode_g1`; assert `Err(DecodeG1ErrorReturn::NonCanonicalCoordinate)`
    * `[ ]`   `decode_g1_rejects_a_point_off_the_curve`: arrange the 32-byte encodings of one and three; act `decode_g1`; assert `Err(DecodeG1ErrorReturn::NotOnCurve)`
    * `[ ]`   `decode_g2_rejects_a_wrong_length`: arrange 127 bytes; act `decode_g2`; assert `Err(DecodeG2ErrorReturn::WrongLength { expected: 128, actual: 127 })`
    * `[ ]`   `decode_g2_rejects_a_point_off_the_curve`: arrange the second-group generator vector with its last byte `aa` replaced by `ab`; act `decode_g2`; assert `Err(DecodeG2ErrorReturn::NotOnCurve)`
    * `[ ]`   `decode_g2_rejects_a_point_outside_the_subgroup`: arrange the first on-curve point outside the subgroup found by `successors(Some(Fq::ONE), |c0| Some(*c0 + Fq::ONE))` and `find_map`, taking `x = Fq2::new(c0, Fq::ZERO)`, `y` from `Option::from((x.square() * x + G2Affine::b()).sqrt())`, the point from `Option::from(G2Affine::from_xy(x, y))`, and keeping it when `bool::from(point.to_curve().is_torsion_free())` is false, encoded as `x.c1()`, `x.c0()`, `y.c1()`, `y.c0()`, each `to_repr()` reversed to big-endian; act `decode_g2`; assert `Err(DecodeG2ErrorReturn::NotInSubgroup)`
    * `[ ]`   `decode_scalar_rejects_the_group_order`: arrange `r`; act `decode_scalar`; assert `Err(DecodeScalarErrorReturn::NonCanonical)`
    * `[ ]`   `decode_scalar_round_trips_the_largest_canonical_scalar`: arrange `r - 1`; act `decode_scalar` then `encode_scalar`; assert the exposed bytes equal `r - 1`
    * `[ ]`   `decode_scalar_rejects_a_wrong_length`: arrange 31 bytes; act `decode_scalar`; assert `Err(DecodeScalarErrorReturn::WrongLength { expected: 32, actual: 31 })`
    * `[ ]`   `add_g1_of_the_generator_to_itself_equals_its_multiple_by_two`: arrange the generator and the decoded scalar two, payloads from `build_add_g1_payload` and `build_mul_g1_payload`; act `add_g1` and `mul_g1`, each encoded; assert the two encodings are equal and differ from the generator's
    * `[ ]`   `add_g2_of_the_generator_to_itself_equals_its_multiple_by_two`: the same over the second group
    * `[ ]`   `msm_g1_equals_the_multiple_by_the_sum_of_its_scalars`: arrange terms from `build_msm_g1_term` pairing the generator with two and with three, and the scalar five; act `msm_g1` and `mul_g1` by five, each encoded; assert the encodings are equal
    * `[ ]`   `msm_g2_equals_the_multiple_by_the_sum_of_its_scalars`: the same over the second group
    * `[ ]`   `pairing_product_is_one_for_a_pairing_and_its_inverse`: arrange terms `(g1, g2)` and `(g1 · (r - 1), g2)`; act `pairing_product_is_one`; assert `is_one` is `true`
    * `[ ]`   `pairing_product_is_not_one_for_a_single_generator_pairing`: arrange the one term `(g1, g2)`; act `pairing_product_is_one`; assert `is_one` is `false`
    * `[ ]`   `pairing_product_is_one_across_the_bilinear_exchange`: arrange terms `(g1 · 2, g2)` and `(g1, g2 · (r - 2))`; act `pairing_product_is_one`; assert `is_one` is `true`
    * `[ ]`   `sample_from_uniform_bytes_rejects_a_wrong_length`: arrange `build_sample_uniform_scalar_payload` with `uniform` holding 63 bytes; act `Bn254Halo2curvesScalar::sample_from_uniform_bytes`; assert `Err(SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual: 63 })`
    * `[ ]`   `sample_from_uniform_bytes_reads_its_input_as_a_big_endian_integer`: arrange `uniform` holding 63 zero bytes followed by `05`; act `sample_from_uniform_bytes`, then `encode_scalar` of a clone of the exposed scalar; assert the exposed bytes equal the 32-byte big-endian encoding of five
    * `[ ]`   `sample_from_uniform_bytes_reduces_the_largest_input_into_the_scalar_field`: arrange `uniform` holding 64 bytes of `ff`; act `sample_from_uniform_bytes`, then `encode_scalar` of a clone of the exposed scalar, then `decode_scalar`; assert the decode returns `Ok`, the sampled scalar being below `r`
    * `[ ]`   `sample_from_uniform_bytes_draws_an_in_range_scalar_from_the_operating_system_source`: arrange `create_random_source` for `RandomSourceKind::OperatingSystem` and `fill_bytes` with `length: Some(Bn254Halo2curvesScalar::UNIFORM_BYTES_LENGTH)`; act `sample_from_uniform_bytes` over the draw, then `encode_scalar` and `decode_scalar`; assert the decode returns `Ok`
    * `[ ]`   `bn254_halo2curves_pairing_declares_its_curve_arithmetic_encoding_and_versions`: act read `Bn254Halo2curvesPairing::DECLARATION`; assert `curve` matches `PairingCurve::Bn254`, `verifier_group_arithmetic` matches `VerifierGroupArithmetic::FirstGroupOnly`, `precompile_encoding` matches `PrecompileEncoding::Eip196Eip197`, `adapter_version` equals `1`, and `interface_version` equals `PAIRING_INTERFACE_VERSION`
    * `[ ]`   Every test block carries the full `Contract`, `Arrange`, `Act`, `Assert` header and the inline markers; every payload is built through its family builder with only the overrides the test depends on

  * `[ ]`   `construction`
    * `[ ]`   `Bn254Halo2curvesPairing::try_new` is the concrete's only producer, and its only caller is the pairing factory, which reads `Bn254Halo2curvesPairing::DECLARATION` before constructing
    * `[ ]`   A group element or scalar is produced only by the adapter's generators, arithmetic, and decoders, or by the scalar's sampling bound; no consumer constructs one from library values

  * `[ ]`   `adapters/pairing/src/bn254_halo2curves/mod.rs`
    * `[ ]`   Module declarations: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, `pub(crate) mod provides;`, and `#[cfg(test)] mod test;`
    * `[ ]`   `impl Bn254Halo2curvesPairing` with `pub const DECLARATION: PairingDeclaration` and `pub fn try_new(_params: Bn254Halo2curvesPairingConstructorParams) -> Bn254Halo2curvesPairingTryNewReturn` returning `Ok(Bn254Halo2curvesPairing)`
    * `[ ]`   `impl IPairingAdapter for Bn254Halo2curvesPairing` with `type Scalar = Bn254Halo2curvesScalar;`, `type G1 = Bn254Halo2curvesG1;`, `type G2 = Bn254Halo2curvesG2;`, and every method realizing its branches in the interaction spec, the decoders checking in the stated order and copying each 32-byte coordinate out of the fixed-size array before reversing it
    * `[ ]`   `impl Zeroize for Bn254Halo2curvesScalar` and `impl Drop for Bn254Halo2curvesScalar`, each setting `self.value` to `Fr::ZERO` and calling `black_box(&self.value)`
    * `[ ]`   `impl ISampleUniformScalar for Bn254Halo2curvesScalar` with `const UNIFORM_BYTES_LENGTH: usize = 64;` and `sample_from_uniform_bytes` realizing its branches, the scalar moved into a `Secret` by `let Ok(scalar) = Secret::try_new(SecretConstructorParams { value });`
    * `[ ]`   Imports the family's names from `crate::factory::provides`, `Secret` and `SecretConstructorParams` from `domain`, `zeroize::Zeroize`, `core::hint::black_box`, the `halo2curves` names the context slice lists, and this module's types from `interface`
    * `[ ]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[ ]`   `adapters/pairing/src/bn254_halo2curves/provides.rs`
    * `[ ]`   `pub(crate) use super::interface::*;`, nothing else, so the concrete is visible to the crate's factory and to nothing outside the crate

  * `[ ]`   `directionality`
    * `[ ]`   `bn254_halo2curves` depends on the `factory` module's surface through `crate::factory::provides`, on `domain`, on `zeroize`, and on `halo2curves`; it depends on no other concrete and no concrete depends on it; the `factory` module is unchanged and depends on no concrete; among repository crates the crate still depends on `crates/domain` alone at runtime
    * `[ ]`   `pairing/factory` adds the family form's recorded cycle when the factory function constructs this concrete

  * `[ ]`   `requirements`
    * `[ ]`   `adapters/pairing/Cargo.toml` differs from `pairing/bn254_arkworks`'s only by the `halo2curves` dependency, and `halo2curves` is named nowhere in the crate outside `adapters/pairing/src/bn254_halo2curves`
    * `[ ]`   `cargo check --all-targets --all-features`, `cargo fmt --check`, and `cargo deny check` complete without error; `cargo clippy --all-targets --all-features` reports nothing beyond the library target's unused-item warnings for the two BN254 concretes, which `pairing/factory` resolves by constructing them
    * `[ ]`   Every test in `bn254_halo2curves/test.rs` passes: the generator encodings match EIP-196 and EIP-197 and the identity encodes to zero bytes, the decoders round-trip and reject a wrong length, a non-canonical coordinate or scalar, a point off the curve, and a second-group point outside the subgroup, the arithmetic and multi-scalar multiplication agree, the pairing-product check accepts a pairing with its inverse and the bilinear exchange and rejects a lone pairing, and sampling rejects a wrong length, reads its input big-endian, and returns a scalar below the group order from a production draw (CR-10 on BN254; CR-05 for the sampled scalar)
    * `[ ]`   Code outside `adapters/pairing` naming `Bn254Halo2curvesPairing` or anything under `bn254_halo2curves` fails to compile; the crate's public surface is the `factory` module's `provides`

* `[ ]`   `pairing/bls12_381_arkworks` **BLS12-381 pairing concrete on arkworks with EIP-2537 encodings and subgroup checks on every input; adds the BLS12-381 variants to the family's declaration**

  * `[ ]`   `objective`
    * `[ ]`   Problem: BLS12-381 is the primary verifier form on Base through the EIP-2537 precompiles, so the pairing family needs a BLS12-381 concrete whose encodings and checks match those precompiles exactly (CR-10)
    * `[ ]`   Functional: the concrete implements `IPairingAdapter` over arkworks' BLS12-381, with its own scalar and group-element types over the library's elements as the associated types
    * `[ ]`   Functional: it encodes and decodes a base field element as EIP-2537's 64 bytes, sixteen zero bytes followed by the 48-byte big-endian integer, a first-group point as 128 bytes of `x` then `y`, a second-group point as 256 bytes of `x.c0`, `x.c1`, `y.c0`, `y.c1`, the point at infinity as all zero bytes, and a scalar as 32 big-endian bytes
    * `[ ]`   Functional: decoding rejects a wrong length, a nonzero padding byte, a field element at or above the modulus, a point off the curve, and a point outside the prime-order subgroup in either group, since BLS12-381's first group has a nontrivial cofactor as well as its second
    * `[ ]`   Functional: its scalar type implements the family's sampling bound, reading the 64 uniform bytes as one big-endian integer reduced modulo the group order
    * `[ ]`   Functional: it declares the BLS12-381 curve, second-group arithmetic at the verifier, the EIP-2537 encoding, its adapter version, and the interface version it implements, so the proof factory selects the verifier form with direct second-group equations
    * `[ ]`   Non-functional: `ark-bls12-381` is named only inside `adapters/pairing/src/bls12_381_arkworks`, and `ark-ec` and `ark-ff` only inside the arkworks concretes

  * `[ ]`   `role`
    * `[ ]`   Adapter: a further concrete of the pairing family, consuming the generic interface, sampling bound, declaration, and builders `pairing/bn254_arkworks` authored in the `factory` module
    * `[ ]`   Adds the variants `PairingCurve::Bls12381`, `VerifierGroupArithmetic::BothGroups`, and `PrecompileEncoding::Eip2537` to `factory/interface.rs`, the producers its declaration needs; changes no other item of the `factory` module
    * `[ ]`   Adds its module line to `lib.rs` and its dependency line to the crate manifest
    * `[ ]`   Does not select between concretes; `pairing/factory` constructs a concrete by the composition's request and `harness-crypto/benchmark` records the default per curve
    * `[ ]`   Does not name, import, or compare against another concrete
    * `[ ]`   Does not carry a commit

  * `[ ]`   `module`
    * `[ ]`   Bounded context: the private `bls12_381_arkworks` module of `adapters/pairing`, holding the adapter over arkworks' BLS12-381, its scalar and group-element types over the library's elements, its constructor params, and the builder defaults for its owned types; and the three declaration variants in the `factory` module
    * `[ ]`   Outside: the rest of the family's generic interface, every other concrete, and the factory's selection

  * `[ ]`   `deps`
    * `[ ]`   The `factory` module's surface, through `crate::factory::provides`: `IPairingAdapter` with its associated types and methods and every method's params, payload, success, error, and return types; `ISampleUniformScalar` and its types; `PairingDeclaration`, `PairingCurve`, `VerifierGroupArithmetic`, `PrecompileEncoding`, and `PAIRING_INTERFACE_VERSION`; and, in tests, the family's builders
    * `[ ]`   `domain`, `crates/domain`, path dependency the crate already carries; supplies `Secret` and `SecretConstructorParams`; in tests `build_secret` and `SecretConstructorParamsOverrides`
    * `[ ]`   `zeroize` `1.9.0`, which the crate already carries; supplies the `Zeroize` trait the sampling bound requires and the scalar implements
    * `[ ]`   `ark-bls12-381` `0.6.0`, external crate, MIT OR Apache-2.0, runtime dependency named only in `bls12_381_arkworks`, default features; supplies the curve and the pairing
    * `[ ]`   `ark-ec` `0.6.0` and `ark-ff` `0.6.0`, which the crate already carries; supply the group and field arithmetic
    * `[ ]`   `random` with its `mocks` feature and `hex` `0.4.3`, dev-dependencies the crate already carries; supply the production draw and the test vectors
    * `[ ]`   `core::convert::Infallible`, standard library, the constructor's error arm
    * `[ ]`   No reverse dependency; nothing depends on this concrete until `pairing/factory`

  * `[ ]`   `context_slice`
    * `[ ]`   From `ark-bls12-381`: `Bls12_381`, `Fq`, `Fq2` with its public fields `c0` and `c1` and `Fq2::new(c0, c1)`, `Fr`, `G1Affine`, `G1Projective`, `G2Affine`, and `G2Projective`
    * `[ ]`   From `ark-ec`: `AffineRepr` for `generator()` and `xy()`; the short-Weierstrass affine `identity()`, `new_unchecked(x, y)`, `is_on_curve()`, `is_in_correct_subgroup_assuming_on_curve()`, and `get_point_from_x_unchecked(x, greatest)`; the affine `+` and `* Fr` producing projective points; `CurveGroup::into_affine`; `VariableBaseMSM::msm_unchecked(bases, scalars)`; `pairing::Pairing::multi_pairing(a, b)` returning `PairingOutput`
    * `[ ]`   From `ark-ff`: `PrimeField::from_be_bytes_mod_order(&[u8])` and `into_bigint()`, `BigInteger::to_bytes_be()`, which yields 48 bytes for `Fq` and 32 for `Fr`, `Zero::is_zero()` on `PairingOutput`, and `Fr::from(u64)` and `Fq::from(u64)`

  * `[ ]`   `adapters/pairing/Cargo.toml`
    * `[ ]`   Adds `ark-bls12-381 = "0.6.0"`, default features, to `[dependencies]`; every other table and key is unchanged

  * `[ ]`   `adapters/pairing/src/lib.rs`
    * `[ ]`   Adds `mod bls12_381_arkworks;` to the barrel; every other line is unchanged
    * `[ ]`   Until `bls12_381_arkworks/mod.rs` exists, `cargo check` reports the unresolved module, which is the RED state for every element below that precedes it

  * `[ ]`   `adapters/pairing/src/factory/interface.rs`
    * `[ ]`   `PairingCurve` gains the variant `Bls12381`; `VerifierGroupArithmetic` gains the variant `BothGroups`; `PrecompileEncoding` gains the variant `Eip2537`
    * `[ ]`   Every other item is unchanged

  * `[ ]`   `adapters/pairing/src/bls12_381_arkworks/interface.rs`
    * `[ ]`   `Bls12381ArkworksPairing`, the unit struct `pub struct Bls12381ArkworksPairing;`, the adapter over arkworks' BLS12-381
    * `[ ]`   `Bls12381ArkworksPairingConstructorParams`, the fieldless struct `pub struct Bls12381ArkworksPairingConstructorParams;`
    * `[ ]`   `Bls12381ArkworksPairingTryNewReturn`, the alias `Result<Bls12381ArkworksPairing, Infallible>`; the error arm is uninhabited because the adapter takes no configuration
    * `[ ]`   `Bls12381ArkworksScalar`, a struct with `#[derive(Clone)]` and one field `pub(super) value: ark_bls12_381::Fr`
    * `[ ]`   `Bls12381ArkworksG1`, a struct with `#[derive(Clone)]` and one field `pub(super) value: ark_bls12_381::G1Affine`
    * `[ ]`   `Bls12381ArkworksG2`, a struct with `#[derive(Clone)]` and one field `pub(super) value: ark_bls12_381::G2Affine`
    * `[ ]`   Imports `core::convert::Infallible`; declares nothing else

  * `[ ]`   `adapters/pairing/src/bls12_381_arkworks/interaction.spec.md`
    * `[ ]`   `Bls12381ArkworksPairing::try_new(params: Bls12381ArkworksPairingConstructorParams) -> Bls12381ArkworksPairingTryNewReturn`: one branch; outcome `Ok(Bls12381ArkworksPairing)`; the error arm has no branch
    * `[ ]`   `Bls12381ArkworksPairing::DECLARATION`: the inherent constant `PairingDeclaration { curve: PairingCurve::Bls12381, verifier_group_arithmetic: VerifierGroupArithmetic::BothGroups, precompile_encoding: PrecompileEncoding::Eip2537, adapter_version: 1, interface_version: PAIRING_INTERFACE_VERSION }`
    * `[ ]`   `g1_generator`, `g2_generator`: one branch each; dependency call `G1Affine::generator()` or `G2Affine::generator()`; outcome `Ok` holding the generator in the owned group type
    * `[ ]`   `add_g1`, `add_g2`: one branch each; dependency call the affine `+` of the two payload points and `into_affine`; outcome `Ok` holding the sum
    * `[ ]`   `mul_g1`, `mul_g2`: one branch each; dependency call the affine point `*` the scalar's `Fr` and `into_affine`; outcome `Ok` holding the product; the payload, holding the scalar, drops at the end of the call and the scalar is zeroized
    * `[ ]`   `msm_g1`, `msm_g2`: one branch each; the terms are split into a `Vec` of affine bases and a `Vec<Fr>` of the same length; dependency call `G1Projective::msm_unchecked` or `G2Projective::msm_unchecked` over them and `into_affine`; the `Vec<Fr>` is zeroized after the call; outcome `Ok` holding the sum; an empty term list yields the identity
    * `[ ]`   `pairing_product_is_one`: one branch; dependency call `Bls12_381::multi_pairing` over the terms' first-group elements and second-group elements in term order; outcome `Ok(PairingProductIsOneSuccessReturn { is_one })` where `is_one` is the output's `is_zero()`; an empty term list yields `is_one: true`
    * `[ ]`   A coordinate, as read by every decoder below, is 64 bytes whose first 16 are zero and whose last 48, read by `Fq::from_be_bytes_mod_order`, re-encode through `into_bigint().to_bytes_be()` to the same 48 bytes; a nonzero byte among the first 16, or 48 bytes at or above the base field modulus, is non-canonical
    * `[ ]`   `decode_g1`, wrong length: condition `payload.len() != 128`; outcome `Err(DecodeG1ErrorReturn::WrongLength { expected: 128, actual: payload.len() })`
    * `[ ]`   `decode_g1`, non-canonical coordinate: condition either 64-byte coordinate is non-canonical; outcome `Err(DecodeG1ErrorReturn::NonCanonicalCoordinate)`
    * `[ ]`   `decode_g1`, identity: condition both coordinates are zero; outcome `Ok` holding `G1Affine::identity()`
    * `[ ]`   `decode_g1`, off the curve: condition `G1Affine::new_unchecked(x, y).is_on_curve()` is false; outcome `Err(DecodeG1ErrorReturn::NotOnCurve)`
    * `[ ]`   `decode_g1`, outside the subgroup: condition `is_in_correct_subgroup_assuming_on_curve()` is false; outcome `Err(DecodeG1ErrorReturn::NotInSubgroup)`; reachable, since BLS12-381's first group has a nontrivial cofactor
    * `[ ]`   `decode_g1`, valid: every check passes; outcome `Ok` holding the point
    * `[ ]`   `decode_g2`: the same branches in the same order over 256 bytes read as `x.c0`, `x.c1`, `y.c0`, `y.c1`, each a 64-byte coordinate, assembled by `Fq2::new(c0, c1)`, with `DecodeG2ErrorReturn` and an expected length of 256; the identity is all four coordinates zero
    * `[ ]`   `decode_scalar`, wrong length: condition `payload.len() != 32`; outcome `Err(DecodeScalarErrorReturn::WrongLength { expected: 32, actual: payload.len() })`
    * `[ ]`   `decode_scalar`, non-canonical: condition the bytes, read by `Fr::from_be_bytes_mod_order`, do not re-encode to the same 32 bytes, that is, they are at least the group order; outcome `Err(DecodeScalarErrorReturn::NonCanonical)`; EIP-2537's MSM accepts any 256-bit scalar, and this decoder, which produces an owned scalar, admits only the canonical ones
    * `[ ]`   `decode_scalar`, valid: outcome `Ok` holding the scalar
    * `[ ]`   `encode_g1`: one branch; the identity encodes to 128 zero bytes; any other point encodes `x` then `y` from `xy()`, each as 16 zero bytes followed by `into_bigint().to_bytes_be()`
    * `[ ]`   `encode_g2`: one branch; the identity encodes to 256 zero bytes; any other point encodes `x.c0`, `x.c1`, `y.c0`, `y.c1`, each as 16 zero bytes followed by its 48 big-endian bytes
    * `[ ]`   `encode_scalar`: one branch; outcome `Ok` holding the scalar's 32 big-endian bytes moved into a `Secret`
    * `[ ]`   `Bls12381ArkworksScalar::UNIFORM_BYTES_LENGTH`: `64`, twice the byte width of the group order
    * `[ ]`   `Bls12381ArkworksScalar::sample_from_uniform_bytes`, wrong length: condition `payload.uniform.expose().len() != 64`; outcome `Err(SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual })`
    * `[ ]`   `Bls12381ArkworksScalar::sample_from_uniform_bytes`, sampled: dependency call `Fr::from_be_bytes_mod_order` over the exposed bytes; outcome `Ok` holding the scalar moved into a `Secret`; the payload's `Secret` zeroizes the input when it drops
    * `[ ]`   Zeroization: `Bls12381ArkworksScalar` zeroizes its `Fr` through its `Zeroize` implementation and on drop, so every clone a consumer places in a payload is zeroized when the payload drops

  * `[ ]`   `adapters/pairing/src/bls12_381_arkworks/mock.rs`
    * `[ ]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`
    * `[ ]`   The builder defaults for the concrete's owned types, which the family's generic builders and `MockIPairingAdapter` read through `Default`: `impl Default for Bls12381ArkworksScalar` returning `Fr::from(1u64)`; `impl Default for Bls12381ArkworksG1` returning `G1Affine::generator()`; `impl Default for Bls12381ArkworksG2` returning `G2Affine::generator()`
    * `[ ]`   Nothing else; the types are built as real values, so there is no overrides type, invalidator, or mock function here

  * `[ ]`   `adapters/pairing/src/bls12_381_arkworks/test.rs`
    * `[ ]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`; imports this module's types from `super::interface`, the family's names and builders from `crate::factory::provides`, `build_secret` and `SecretConstructorParamsOverrides` from `domain`, the `random` names `pairing/bn254_arkworks`'s test imports, `hex::decode`, and, for the non-subgroup vectors, `ark_bls12_381::{Fq, Fq2, G1Affine, G2Affine}` with `ark_ec::AffineRepr` and `ark_ff::{BigInteger, PrimeField}`; each test constructs the subject by `let Ok(pairing) = Bls12381ArkworksPairing::try_new(Bls12381ArkworksPairingConstructorParams);`, decodes hex by `let Ok(bytes) = decode(…) else { panic!(…) };`, and unpacks each call by `let Ok(…) = … else { panic!(…) };`
    * `[ ]`   The vectors, from EIP-2537, as hex, each coordinate written below as its 48 bytes and encoded in a vector as 16 zero bytes followed by them: the base field modulus `p` = `1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab`; the group order `r` = `73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001`; `r - 1` = `73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000000`; `r - 2` = `73eda753299d7d483339d80809a1d80553bda402fffe5bfefffffffeffffffff`; the first-group generator `x` = `17f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb` and `y` = `08b3f481e3aaa0f1a09e30ed741d8ae4fcf5e095d5d00af600db18cb2c04b3edd03cc744a2888ae40caa232946c5e7e1`; the second-group generator `x.c0` = `024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8`, `x.c1` = `13e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e`, `y.c0` = `0ce5d527727d6e118cc9cdc6da2e351aadfd9baa8cbdd3a76d429a695160d12c923ac9cc3baca289e193548608b82801`, and `y.c1` = `0606c4a02ea734cc32acd2b02bc28b99cb3e287e85a763af267492ab572e99ab3f370d275cec1da1aaa9075ff05f79be`; the scalars two, three, and five as 32 big-endian bytes
    * `[ ]`   `g1_generator_encodes_to_the_eip_2537_generator`: contract: the first-group generator's encoding is EIP-2537's; act `encode_g1` over `g1_generator`; assert the bytes equal the padded first-group generator vector
    * `[ ]`   `g2_generator_encodes_to_the_eip_2537_generator`: contract: the second-group generator's encoding is EIP-2537's, `c0` before `c1`; act `encode_g2` over `g2_generator`; assert the bytes equal the padded second-group generator vector
    * `[ ]`   `decode_g1_round_trips_the_eip_2537_generator`: arrange the first-group generator vector; act `decode_g1` then `encode_g1`; assert the bytes equal the vector
    * `[ ]`   `decode_g2_round_trips_the_eip_2537_generator`: arrange the second-group generator vector; act `decode_g2` then `encode_g2`; assert the bytes equal the vector
    * `[ ]`   `decode_g1_reads_the_all_zero_encoding_as_the_identity`: arrange 128 zero bytes and the generator; act `decode_g1`, then `add_g1` of the generator and the decoded point, then `encode_g1`; assert the bytes equal the first-group generator vector
    * `[ ]`   `decode_g2_reads_the_all_zero_encoding_as_the_identity`: the same over 256 zero bytes, `add_g2`, and the second-group generator vector
    * `[ ]`   `decode_g1_rejects_a_wrong_length`: arrange 127 bytes; act `decode_g1`; assert `Err(DecodeG1ErrorReturn::WrongLength { expected: 128, actual: 127 })`
    * `[ ]`   `decode_g1_rejects_a_nonzero_padding_byte`: arrange the first-group generator vector with its first byte set to `01`; act `decode_g1`; assert `Err(DecodeG1ErrorReturn::NonCanonicalCoordinate)`
    * `[ ]`   `decode_g1_rejects_a_coordinate_equal_to_the_base_field_modulus`: arrange `p` padded, followed by the generator's `y` padded; act `decode_g1`; assert `Err(DecodeG1ErrorReturn::NonCanonicalCoordinate)`
    * `[ ]`   `decode_g1_rejects_a_point_off_the_curve`: arrange the first-group generator vector with its last byte `e1` replaced by `e2`; act `decode_g1`; assert `Err(DecodeG1ErrorReturn::NotOnCurve)`
    * `[ ]`   `decode_g1_rejects_a_point_outside_the_subgroup`: arrange the first on-curve point outside the subgroup found by `(1u64..).find_map` over `G1Affine::get_point_from_x_unchecked(Fq::from(c), false)` filtered by `!is_in_correct_subgroup_assuming_on_curve()`, encoded from `xy()` as `x` then `y`, each as 16 zero bytes followed by `into_bigint().to_bytes_be()`; act `decode_g1`; assert `Err(DecodeG1ErrorReturn::NotInSubgroup)`
    * `[ ]`   `decode_g2_rejects_a_wrong_length`: arrange 255 bytes; act `decode_g2`; assert `Err(DecodeG2ErrorReturn::WrongLength { expected: 256, actual: 255 })`
    * `[ ]`   `decode_g2_rejects_a_point_off_the_curve`: arrange the second-group generator vector with its last byte `be` replaced by `bf`; act `decode_g2`; assert `Err(DecodeG2ErrorReturn::NotOnCurve)`
    * `[ ]`   `decode_g2_rejects_a_point_outside_the_subgroup`: arrange the first on-curve point outside the subgroup found by `(1u64..).find_map` over `G2Affine::get_point_from_x_unchecked(Fq2::new(Fq::from(c0), Fq::from(0u64)), false)` filtered by `!is_in_correct_subgroup_assuming_on_curve()`, encoded as `x.c0`, `x.c1`, `y.c0`, `y.c1`, each as 16 zero bytes followed by `into_bigint().to_bytes_be()`; act `decode_g2`; assert `Err(DecodeG2ErrorReturn::NotInSubgroup)`
    * `[ ]`   `decode_scalar_rejects_the_group_order`: arrange `r`; act `decode_scalar`; assert `Err(DecodeScalarErrorReturn::NonCanonical)`
    * `[ ]`   `decode_scalar_round_trips_the_largest_canonical_scalar`: arrange `r - 1`; act `decode_scalar` then `encode_scalar`; assert the exposed bytes equal `r - 1`
    * `[ ]`   `decode_scalar_rejects_a_wrong_length`: arrange 31 bytes; act `decode_scalar`; assert `Err(DecodeScalarErrorReturn::WrongLength { expected: 32, actual: 31 })`
    * `[ ]`   `add_g1_of_the_generator_to_itself_equals_its_multiple_by_two`: arrange the generator and the decoded scalar two, payloads from `build_add_g1_payload` and `build_mul_g1_payload`; act `add_g1` and `mul_g1`, each encoded; assert the two encodings are equal and differ from the generator's
    * `[ ]`   `add_g2_of_the_generator_to_itself_equals_its_multiple_by_two`: the same over the second group
    * `[ ]`   `msm_g1_equals_the_multiple_by_the_sum_of_its_scalars`: arrange terms from `build_msm_g1_term` pairing the generator with two and with three, and the scalar five; act `msm_g1` and `mul_g1` by five, each encoded; assert the encodings are equal
    * `[ ]`   `msm_g2_equals_the_multiple_by_the_sum_of_its_scalars`: the same over the second group
    * `[ ]`   `pairing_product_is_one_for_a_pairing_and_its_inverse`: arrange terms `(g1, g2)` and `(g1 · (r - 1), g2)`; act `pairing_product_is_one`; assert `is_one` is `true`
    * `[ ]`   `pairing_product_is_not_one_for_a_single_generator_pairing`: arrange the one term `(g1, g2)`; act `pairing_product_is_one`; assert `is_one` is `false`
    * `[ ]`   `pairing_product_is_one_across_the_bilinear_exchange`: arrange terms `(g1 · 2, g2)` and `(g1, g2 · (r - 2))`; act `pairing_product_is_one`; assert `is_one` is `true`
    * `[ ]`   `sample_from_uniform_bytes_rejects_a_wrong_length`: arrange `build_sample_uniform_scalar_payload` with `uniform` holding 63 bytes; act `Bls12381ArkworksScalar::sample_from_uniform_bytes`; assert `Err(SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual: 63 })`
    * `[ ]`   `sample_from_uniform_bytes_reads_its_input_as_a_big_endian_integer`: arrange `uniform` holding 63 zero bytes followed by `05`; act `sample_from_uniform_bytes`, then `encode_scalar` of a clone of the exposed scalar; assert the exposed bytes equal the 32-byte big-endian encoding of five
    * `[ ]`   `sample_from_uniform_bytes_reduces_the_largest_input_into_the_scalar_field`: arrange `uniform` holding 64 bytes of `ff`; act `sample_from_uniform_bytes`, then `encode_scalar` of a clone of the exposed scalar, then `decode_scalar`; assert the decode returns `Ok`, the sampled scalar being below `r`
    * `[ ]`   `sample_from_uniform_bytes_draws_an_in_range_scalar_from_the_operating_system_source`: arrange `create_random_source` for `RandomSourceKind::OperatingSystem` and `fill_bytes` with `length: Some(Bls12381ArkworksScalar::UNIFORM_BYTES_LENGTH)`; act `sample_from_uniform_bytes` over the draw, then `encode_scalar` and `decode_scalar`; assert the decode returns `Ok`
    * `[ ]`   `bls12_381_arkworks_pairing_declares_its_curve_arithmetic_encoding_and_versions`: act read `Bls12381ArkworksPairing::DECLARATION`; assert `curve` matches `PairingCurve::Bls12381`, `verifier_group_arithmetic` matches `VerifierGroupArithmetic::BothGroups`, `precompile_encoding` matches `PrecompileEncoding::Eip2537`, `adapter_version` equals `1`, and `interface_version` equals `PAIRING_INTERFACE_VERSION`
    * `[ ]`   Every test block carries the full `Contract`, `Arrange`, `Act`, `Assert` header and the inline markers; every payload is built through its family builder with only the overrides the test depends on

  * `[ ]`   `construction`
    * `[ ]`   `Bls12381ArkworksPairing::try_new` is the concrete's only producer, and its only caller is the pairing factory, which reads `Bls12381ArkworksPairing::DECLARATION` before constructing
    * `[ ]`   A group element or scalar is produced only by the adapter's generators, arithmetic, and decoders, or by the scalar's sampling bound; no consumer constructs one from library values

  * `[ ]`   `adapters/pairing/src/bls12_381_arkworks/mod.rs`
    * `[ ]`   Module declarations: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, `pub(crate) mod provides;`, and `#[cfg(test)] mod test;`
    * `[ ]`   `impl Bls12381ArkworksPairing` with `pub const DECLARATION: PairingDeclaration` and `pub fn try_new(_params: Bls12381ArkworksPairingConstructorParams) -> Bls12381ArkworksPairingTryNewReturn` returning `Ok(Bls12381ArkworksPairing)`
    * `[ ]`   `impl IPairingAdapter for Bls12381ArkworksPairing` with `type Scalar = Bls12381ArkworksScalar;`, `type G1 = Bls12381ArkworksG1;`, `type G2 = Bls12381ArkworksG2;`, and every method realizing its branches in the interaction spec, the decoders checking in the stated order and slicing the payload only after the length check
    * `[ ]`   `impl Zeroize for Bls12381ArkworksScalar` calling `self.value.zeroize()`; `impl Drop for Bls12381ArkworksScalar` calling `self.value.zeroize()`
    * `[ ]`   `impl ISampleUniformScalar for Bls12381ArkworksScalar` with `const UNIFORM_BYTES_LENGTH: usize = 64;` and `sample_from_uniform_bytes` realizing its branches, the scalar moved into a `Secret` by `let Ok(scalar) = Secret::try_new(SecretConstructorParams { value });`
    * `[ ]`   Imports the family's names from `crate::factory::provides`, `Secret` and `SecretConstructorParams` from `domain`, `zeroize::Zeroize`, the arkworks names the context slice lists, and this module's types from `interface`
    * `[ ]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[ ]`   `adapters/pairing/src/bls12_381_arkworks/provides.rs`
    * `[ ]`   `pub(crate) use super::interface::*;`, nothing else, so the concrete is visible to the crate's factory and to nothing outside the crate

  * `[ ]`   `directionality`
    * `[ ]`   `bls12_381_arkworks` depends on the `factory` module's surface through `crate::factory::provides`, on `domain`, on `zeroize`, and on the arkworks crates; it depends on no other concrete and no concrete depends on it; the `factory` module gains three enum variants and still depends on no concrete; among repository crates the crate still depends on `crates/domain` alone at runtime
    * `[ ]`   `pairing/factory` adds the family form's recorded cycle when the factory function constructs this concrete

  * `[ ]`   `requirements`
    * `[ ]`   `adapters/pairing/Cargo.toml` gains only the `ark-bls12-381` dependency, and `ark-bls12-381` is named nowhere in the crate outside `adapters/pairing/src/bls12_381_arkworks`
    * `[ ]`   `factory/interface.rs` differs from its prior state only by the three variants
    * `[ ]`   `cargo check --all-targets --all-features`, `cargo fmt --check`, and `cargo deny check` complete without error; `cargo clippy --all-targets --all-features` reports nothing beyond the library target's unused-item warnings for the pairing concretes and the new variants, which `pairing/factory` resolves by constructing the concretes
    * `[ ]`   Every test in `bls12_381_arkworks/test.rs` passes: the generator encodings match EIP-2537, the decoders round-trip and reject a wrong length, a nonzero padding byte, a non-canonical coordinate or scalar, a point off the curve, and a point outside the subgroup in both groups, the arithmetic and multi-scalar multiplication agree, the pairing-product check accepts a pairing with its inverse and the bilinear exchange and rejects a lone pairing, and sampling rejects a wrong length, reads its input big-endian, and returns a scalar below the group order from a production draw (CR-10 on BLS12-381; CR-05 for the sampled scalar)
    * `[ ]`   Code outside `adapters/pairing` naming `Bls12381ArkworksPairing` or anything under `bls12_381_arkworks` fails to compile; the crate's public surface is the `factory` module's `provides`

* `[ ]`   `pairing/bls12_381_halo2curves` **BLS12-381 pairing concrete on halo2curves with EIP-2537 encodings and subgroup checks on every input, a further concrete beneath the pairing factory**

  * `[ ]`   `objective`
    * `[ ]`   Problem: the harness benchmark compares pairing libraries per curve through the factory, so BLS12-381 needs a second concrete over a second library that satisfies the family's generic interface exactly as the arkworks BLS12-381 concrete does (CR-10)
    * `[ ]`   Functional: the concrete implements `IPairingAdapter` over `halo2curves`' BLS12-381, with its own scalar and group-element types over the library's elements as the associated types
    * `[ ]`   Functional: it encodes and decodes a base field element as EIP-2537's 64 bytes, sixteen zero bytes followed by the 48-byte big-endian integer, a first-group point as 128 bytes of `x` then `y`, a second-group point as 256 bytes of `x.c0`, `x.c1`, `y.c0`, `y.c1`, the point at infinity as all zero bytes, and a scalar as 32 big-endian bytes
    * `[ ]`   Functional: decoding rejects a wrong length, a nonzero padding byte, a field element at or above the modulus, a point off the curve, and a point outside the prime-order subgroup in either group
    * `[ ]`   Functional: its scalar type implements the family's sampling bound, reading the 64 uniform bytes as one big-endian integer reduced modulo the group order, so the same input bytes sample the same scalar under either BLS12-381 concrete
    * `[ ]`   Functional: it declares the BLS12-381 curve, second-group arithmetic at the verifier, the EIP-2537 encoding, its adapter version, and the interface version it implements
    * `[ ]`   Non-functional: `halo2curves` is named only inside the `halo2curves` concretes; the family's interface, declaration, builders, and mock are unchanged

  * `[ ]`   `role`
    * `[ ]`   Adapter: a further concrete of the pairing family, consuming the generic interface, sampling bound, declaration, and builders `pairing/bn254_arkworks` authored and the BLS12-381 declaration variants `pairing/bls12_381_arkworks` added
    * `[ ]`   Adds its module line to `lib.rs`; the crate manifest already carries `halo2curves`, and no file of the `factory` module changes
    * `[ ]`   Does not select between the BLS12-381 concretes; `pairing/factory` constructs a concrete by the composition's request and `harness-crypto/benchmark` records the default per curve
    * `[ ]`   Does not name, import, or compare against another concrete
    * `[ ]`   Does not carry a commit

  * `[ ]`   `module`
    * `[ ]`   Bounded context: the private `bls12_381_halo2curves` module of `adapters/pairing`, holding the adapter over `halo2curves`' BLS12-381, its scalar and group-element types over the library's elements, its constructor params, and the builder defaults for its owned types
    * `[ ]`   Outside: the family's generic interface and declaration, every other concrete, and the factory's selection

  * `[ ]`   `deps`
    * `[ ]`   The `factory` module's surface, through `crate::factory::provides`: `IPairingAdapter` with its associated types and methods and every method's params, payload, success, error, and return types; `ISampleUniformScalar` and its types; `PairingDeclaration`, `PairingCurve`, `VerifierGroupArithmetic`, `PrecompileEncoding`, and `PAIRING_INTERFACE_VERSION`; and, in tests, the family's builders
    * `[ ]`   `domain`, `crates/domain`, path dependency the crate already carries; supplies `Secret` and `SecretConstructorParams`; in tests `build_secret` and `SecretConstructorParamsOverrides`
    * `[ ]`   `zeroize` `1.9.0`, which the crate already carries; supplies the `Zeroize` trait the sampling bound requires and the zeroization of the local 64-byte input copy
    * `[ ]`   `halo2curves` `0.10.0`, which the crate already carries; supplies the curve, the pairing, the group arithmetic, the field arithmetic, and multi-scalar multiplication, and re-exports the `ff`, `group`, and `pairing` traits its types implement
    * `[ ]`   `random` with its `mocks` feature and `hex` `0.4.3`, dev-dependencies the crate already carries; supply the production draw and the test vectors
    * `[ ]`   `core::convert::Infallible`, standard library, the constructor's error arm; `core::hint::black_box`, standard library, which keeps the clearing of a scalar from being removed as a dead store, since `halo2curves`' fields implement no `Zeroize`; `core::iter::successors`, standard library, in `test.rs` only
    * `[ ]`   No reverse dependency; nothing depends on this concrete until `pairing/factory`

  * `[ ]`   `context_slice`
    * `[ ]`   From `halo2curves::bls12381`: `Bls12381`, the engine; `Fq`, whose representation is 48 bytes; `Fq2` with `Fq2::new(c0, c1)` and the accessors `c0()` and `c1()`; `Fr`, whose representation is 32 bytes; `G1`, `G1Affine`, `G2`, and `G2Affine`
    * `[ ]`   From `halo2curves::ff`: `Field` for `ZERO`, `ONE`, `is_zero()`, `square()`, and `sqrt()`; `PrimeField` for `from_repr(repr) -> CtOption<Self>`, which reads little-endian bytes and is none at or above the modulus, and `to_repr()`, which writes little-endian bytes, the representation built from a `[u8; 48]` for `Fq` or a `[u8; 32]` for `Fr` by `into()` and read by `as_ref()`; `FromUniformBytes::<64>::from_uniform_bytes(&[u8; 64])` on `Fr`, a little-endian wide reduction
    * `[ ]`   From `halo2curves::group`: `Curve::to_affine`, `Group::is_identity`, `prime::PrimeCurveAffine` for `generator()`, `identity()`, and `to_curve()`, `cofactor::CofactorGroup::is_torsion_free` on `G1` and on `G2`, and the projective `+` and `* Fr`
    * `[ ]`   From `halo2curves`: `CurveAffine` for `from_xy(x, y) -> CtOption<Self>`, which is none off the curve, `coordinates() -> CtOption<Coordinates<Self>>`, and `b()`; `Coordinates` for `x()` and `y()`; `msm::msm_best(coeffs: &[C::Scalar], bases: &[C]) -> C::Curve`
    * `[ ]`   From `halo2curves::pairing`: `MultiMillerLoop::multi_miller_loop(&[(&G1Affine, &G2Affine)])` on `Bls12381`, and `MillerLoopResult::final_exponentiation` returning the target-group element, whose `is_identity()` is the check
    * `[ ]`   A `Choice` becomes a `bool` by `bool::from`, and a `CtOption` becomes an `Option` by `Option::from`

  * `[ ]`   `adapters/pairing/src/lib.rs`
    * `[ ]`   Adds `mod bls12_381_halo2curves;` to the barrel; every other line is unchanged
    * `[ ]`   Until `bls12_381_halo2curves/mod.rs` exists, `cargo check` reports the unresolved module, which is the RED state for every element below that precedes it

  * `[ ]`   `adapters/pairing/src/bls12_381_halo2curves/interface.rs`
    * `[ ]`   `Bls12381Halo2curvesPairing`, the unit struct `pub struct Bls12381Halo2curvesPairing;`, the adapter over `halo2curves`' BLS12-381
    * `[ ]`   `Bls12381Halo2curvesPairingConstructorParams`, the fieldless struct `pub struct Bls12381Halo2curvesPairingConstructorParams;`
    * `[ ]`   `Bls12381Halo2curvesPairingTryNewReturn`, the alias `Result<Bls12381Halo2curvesPairing, Infallible>`; the error arm is uninhabited because the adapter takes no configuration
    * `[ ]`   `Bls12381Halo2curvesScalar`, a struct with `#[derive(Clone)]` and one field `pub(super) value: halo2curves::bls12381::Fr`
    * `[ ]`   `Bls12381Halo2curvesG1`, a struct with `#[derive(Clone)]` and one field `pub(super) value: halo2curves::bls12381::G1Affine`
    * `[ ]`   `Bls12381Halo2curvesG2`, a struct with `#[derive(Clone)]` and one field `pub(super) value: halo2curves::bls12381::G2Affine`
    * `[ ]`   Imports `core::convert::Infallible`; declares nothing else

  * `[ ]`   `adapters/pairing/src/bls12_381_halo2curves/interaction.spec.md`
    * `[ ]`   `Bls12381Halo2curvesPairing::try_new(params: Bls12381Halo2curvesPairingConstructorParams) -> Bls12381Halo2curvesPairingTryNewReturn`: one branch; outcome `Ok(Bls12381Halo2curvesPairing)`; the error arm has no branch
    * `[ ]`   `Bls12381Halo2curvesPairing::DECLARATION`: the inherent constant `PairingDeclaration { curve: PairingCurve::Bls12381, verifier_group_arithmetic: VerifierGroupArithmetic::BothGroups, precompile_encoding: PrecompileEncoding::Eip2537, adapter_version: 1, interface_version: PAIRING_INTERFACE_VERSION }`
    * `[ ]`   `g1_generator`, `g2_generator`: one branch each; dependency call `G1Affine::generator()` or `G2Affine::generator()`; outcome `Ok` holding the generator in the owned group type
    * `[ ]`   `add_g1`, `add_g2`: one branch each; dependency call `to_curve()` on each payload point, the projective `+`, and `to_affine()`; outcome `Ok` holding the sum
    * `[ ]`   `mul_g1`, `mul_g2`: one branch each; dependency call `to_curve()` on the point, the projective `*` the scalar's `Fr`, and `to_affine()`; outcome `Ok` holding the product; the payload, holding the scalar, drops at the end of the call and the scalar is cleared
    * `[ ]`   `msm_g1`, `msm_g2`: one branch each; the terms are split into a `Vec` of affine bases and a `Vec<Fr>` of the same length; dependency call `msm_best(&scalars, &bases)` and `to_affine()`; every element of the `Vec<Fr>` is then set to `Fr::ZERO` and the vector passed to `black_box`; outcome `Ok` holding the sum; an empty term list yields the identity
    * `[ ]`   `pairing_product_is_one`: one branch; dependency call `Bls12381::multi_miller_loop` over the terms as a `Vec<(&G1Affine, &G2Affine)>` in term order, then `final_exponentiation()`; outcome `Ok(PairingProductIsOneSuccessReturn { is_one })` where `is_one` is `bool::from(is_identity())`; an empty term list yields `is_one: true`
    * `[ ]`   A coordinate, as read by every decoder below, is 64 bytes whose first 16 are zero and whose last 48, copied into a `[u8; 48]`, reversed to little-endian, and read by `Fq::from_repr`, are some; a nonzero byte among the first 16, or 48 bytes at or above the base field modulus, is non-canonical
    * `[ ]`   `decode_g1`, wrong length: condition `<[u8; 128]>::try_from(payload)` fails; outcome `Err(DecodeG1ErrorReturn::WrongLength { expected: 128, actual: payload.len() })`
    * `[ ]`   `decode_g1`, non-canonical coordinate: condition either 64-byte coordinate is non-canonical; outcome `Err(DecodeG1ErrorReturn::NonCanonicalCoordinate)`
    * `[ ]`   `decode_g1`, identity: condition both coordinates are zero; outcome `Ok` holding `G1Affine::identity()`
    * `[ ]`   `decode_g1`, off the curve: condition `G1Affine::from_xy(x, y)` is none; outcome `Err(DecodeG1ErrorReturn::NotOnCurve)`
    * `[ ]`   `decode_g1`, outside the subgroup: condition `bool::from(point.to_curve().is_torsion_free())` is false; outcome `Err(DecodeG1ErrorReturn::NotInSubgroup)`; reachable, since BLS12-381's first group has a nontrivial cofactor
    * `[ ]`   `decode_g1`, valid: every check passes; outcome `Ok` holding the point
    * `[ ]`   `decode_g2`: the same branches in the same order over 256 bytes, the length checked by `<[u8; 256]>::try_from(payload)`, read as `x.c0`, `x.c1`, `y.c0`, `y.c1`, each a 64-byte coordinate, assembled by `Fq2::new(c0, c1)`, with `DecodeG2ErrorReturn` and an expected length of 256; the identity is all four coordinates zero
    * `[ ]`   `decode_scalar`, wrong length: condition `<[u8; 32]>::try_from(payload)` fails; outcome `Err(DecodeScalarErrorReturn::WrongLength { expected: 32, actual: payload.len() })`
    * `[ ]`   `decode_scalar`, non-canonical: condition the bytes, reversed to little-endian and read by `Fr::from_repr`, are none, that is, they are at least the group order; outcome `Err(DecodeScalarErrorReturn::NonCanonical)`; EIP-2537's MSM accepts any 256-bit scalar, and this decoder, which produces an owned scalar, admits only the canonical ones
    * `[ ]`   `decode_scalar`, valid: outcome `Ok` holding the scalar
    * `[ ]`   `encode_g1`: one branch; a point whose `coordinates()` is none, the identity, encodes to 128 zero bytes; any other point encodes `x()` then `y()`, each as 16 zero bytes followed by `to_repr()` reversed to 48 big-endian bytes
    * `[ ]`   `encode_g2`: one branch; the identity encodes to 256 zero bytes; any other point encodes `x().c0()`, `x().c1()`, `y().c0()`, `y().c1()`, each as 16 zero bytes followed by `to_repr()` reversed to 48 big-endian bytes
    * `[ ]`   `encode_scalar`: one branch; outcome `Ok` holding the scalar's `to_repr()` reversed to 32 big-endian bytes, moved into a `Secret`
    * `[ ]`   `Bls12381Halo2curvesScalar::UNIFORM_BYTES_LENGTH`: `64`, twice the byte width of the group order
    * `[ ]`   `Bls12381Halo2curvesScalar::sample_from_uniform_bytes`, wrong length: condition `<&[u8; 64]>::try_from(payload.uniform.expose().as_slice())` fails; outcome `Err(SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual })`
    * `[ ]`   `Bls12381Halo2curvesScalar::sample_from_uniform_bytes`, sampled: the 64 bytes are copied into a local `[u8; 64]` and reversed, so the big-endian integer the arkworks concrete reads is the little-endian integer `halo2curves` reads; dependency call `Fr::from_uniform_bytes` over the copy, which is then zeroized; outcome `Ok` holding the scalar moved into a `Secret`; the payload's `Secret` zeroizes the input when it drops
    * `[ ]`   Clearing: `halo2curves`' `Fr` implements no `Zeroize`, so `Bls12381Halo2curvesScalar`'s `Zeroize` implementation and its `Drop` set `value` to `Fr::ZERO` and pass `&self.value` to `black_box`; every clone a consumer places in a payload is cleared when the payload drops

  * `[ ]`   `adapters/pairing/src/bls12_381_halo2curves/mock.rs`
    * `[ ]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`
    * `[ ]`   The builder defaults for the concrete's owned types, which the family's generic builders and `MockIPairingAdapter` read through `Default`: `impl Default for Bls12381Halo2curvesScalar` returning `Fr::ONE`; `impl Default for Bls12381Halo2curvesG1` returning `G1Affine::generator()`; `impl Default for Bls12381Halo2curvesG2` returning `G2Affine::generator()`
    * `[ ]`   Nothing else; the types are built as real values, so there is no overrides type, invalidator, or mock function here

  * `[ ]`   `adapters/pairing/src/bls12_381_halo2curves/test.rs`
    * `[ ]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`; imports this module's types from `super::interface`, the family's names and builders from `crate::factory::provides`, `build_secret` and `SecretConstructorParamsOverrides` from `domain`, the `random` names `pairing/bn254_arkworks`'s test imports, `hex::decode`, `core::iter::successors`, and, for the non-subgroup vectors, `halo2curves::bls12381::{Fq, Fq2, G1Affine, G2Affine}`, `halo2curves::ff::{Field, PrimeField}`, `halo2curves::group::{cofactor::CofactorGroup, prime::PrimeCurveAffine}`, and `halo2curves::CurveAffine`; each test constructs the subject by `let Ok(pairing) = Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);`, decodes hex by `let Ok(bytes) = decode(…) else { panic!(…) };`, and unpacks each call by `let Ok(…) = … else { panic!(…) };`
    * `[ ]`   The vectors are `pairing/bls12_381_arkworks`'s, from EIP-2537, restated here as hex, each coordinate written below as its 48 bytes and encoded in a vector as 16 zero bytes followed by them: `p` = `1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab`; `r` = `73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001`; `r - 1` = `73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000000`; `r - 2` = `73eda753299d7d483339d80809a1d80553bda402fffe5bfefffffffeffffffff`; the first-group generator `x` = `17f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb` and `y` = `08b3f481e3aaa0f1a09e30ed741d8ae4fcf5e095d5d00af600db18cb2c04b3edd03cc744a2888ae40caa232946c5e7e1`; the second-group generator `x.c0` = `024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8`, `x.c1` = `13e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e`, `y.c0` = `0ce5d527727d6e118cc9cdc6da2e351aadfd9baa8cbdd3a76d429a695160d12c923ac9cc3baca289e193548608b82801`, and `y.c1` = `0606c4a02ea734cc32acd2b02bc28b99cb3e287e85a763af267492ab572e99ab3f370d275cec1da1aaa9075ff05f79be`; the scalars two, three, and five as 32 big-endian bytes
    * `[ ]`   `g1_generator_encodes_to_the_eip_2537_generator`: contract: the first-group generator's encoding is EIP-2537's; act `encode_g1` over `g1_generator`; assert the bytes equal the padded first-group generator vector
    * `[ ]`   `g2_generator_encodes_to_the_eip_2537_generator`: contract: the second-group generator's encoding is EIP-2537's, `c0` before `c1`; act `encode_g2` over `g2_generator`; assert the bytes equal the padded second-group generator vector
    * `[ ]`   `decode_g1_round_trips_the_eip_2537_generator`: arrange the first-group generator vector; act `decode_g1` then `encode_g1`; assert the bytes equal the vector
    * `[ ]`   `decode_g2_round_trips_the_eip_2537_generator`: arrange the second-group generator vector; act `decode_g2` then `encode_g2`; assert the bytes equal the vector
    * `[ ]`   `decode_g1_reads_the_all_zero_encoding_as_the_identity`: arrange 128 zero bytes and the generator; act `decode_g1`, then `add_g1` of the generator and the decoded point, then `encode_g1`; assert the bytes equal the first-group generator vector
    * `[ ]`   `decode_g2_reads_the_all_zero_encoding_as_the_identity`: the same over 256 zero bytes, `add_g2`, and the second-group generator vector
    * `[ ]`   `encode_g1_writes_the_identity_as_all_zero_bytes`: arrange the identity as `msm_g1` over an empty `build_msm_g1_payload`; act `encode_g1`; assert the bytes equal 128 zero bytes
    * `[ ]`   `decode_g1_rejects_a_wrong_length`: arrange 127 bytes; act `decode_g1`; assert `Err(DecodeG1ErrorReturn::WrongLength { expected: 128, actual: 127 })`
    * `[ ]`   `decode_g1_rejects_a_nonzero_padding_byte`: arrange the first-group generator vector with its first byte set to `01`; act `decode_g1`; assert `Err(DecodeG1ErrorReturn::NonCanonicalCoordinate)`
    * `[ ]`   `decode_g1_rejects_a_coordinate_equal_to_the_base_field_modulus`: arrange `p` padded, followed by the generator's `y` padded; act `decode_g1`; assert `Err(DecodeG1ErrorReturn::NonCanonicalCoordinate)`
    * `[ ]`   `decode_g1_rejects_a_point_off_the_curve`: arrange the first-group generator vector with its last byte `e1` replaced by `e2`; act `decode_g1`; assert `Err(DecodeG1ErrorReturn::NotOnCurve)`
    * `[ ]`   `decode_g1_rejects_a_point_outside_the_subgroup`: arrange the first on-curve point outside the subgroup found by `successors(Some(Fq::ONE), |x| Some(*x + Fq::ONE))` and `find_map`, taking `y` from `Option::from((x.square() * x + G1Affine::b()).sqrt())`, the point from `Option::from(G1Affine::from_xy(x, y))`, and keeping it when `bool::from(point.to_curve().is_torsion_free())` is false, encoded as `x` then `y`, each as 16 zero bytes followed by `to_repr()` reversed to big-endian; act `decode_g1`; assert `Err(DecodeG1ErrorReturn::NotInSubgroup)`
    * `[ ]`   `decode_g2_rejects_a_wrong_length`: arrange 255 bytes; act `decode_g2`; assert `Err(DecodeG2ErrorReturn::WrongLength { expected: 256, actual: 255 })`
    * `[ ]`   `decode_g2_rejects_a_point_off_the_curve`: arrange the second-group generator vector with its last byte `be` replaced by `bf`; act `decode_g2`; assert `Err(DecodeG2ErrorReturn::NotOnCurve)`
    * `[ ]`   `decode_g2_rejects_a_point_outside_the_subgroup`: arrange the first on-curve point outside the subgroup found by `successors(Some(Fq::ONE), |c0| Some(*c0 + Fq::ONE))` and `find_map`, taking `x = Fq2::new(c0, Fq::ZERO)`, `y` from `Option::from((x.square() * x + G2Affine::b()).sqrt())`, the point from `Option::from(G2Affine::from_xy(x, y))`, and keeping it when `bool::from(point.to_curve().is_torsion_free())` is false, encoded as `x.c0()`, `x.c1()`, `y.c0()`, `y.c1()`, each as 16 zero bytes followed by `to_repr()` reversed to big-endian; act `decode_g2`; assert `Err(DecodeG2ErrorReturn::NotInSubgroup)`
    * `[ ]`   `decode_scalar_rejects_the_group_order`: arrange `r`; act `decode_scalar`; assert `Err(DecodeScalarErrorReturn::NonCanonical)`
    * `[ ]`   `decode_scalar_round_trips_the_largest_canonical_scalar`: arrange `r - 1`; act `decode_scalar` then `encode_scalar`; assert the exposed bytes equal `r - 1`
    * `[ ]`   `decode_scalar_rejects_a_wrong_length`: arrange 31 bytes; act `decode_scalar`; assert `Err(DecodeScalarErrorReturn::WrongLength { expected: 32, actual: 31 })`
    * `[ ]`   `add_g1_of_the_generator_to_itself_equals_its_multiple_by_two`: arrange the generator and the decoded scalar two, payloads from `build_add_g1_payload` and `build_mul_g1_payload`; act `add_g1` and `mul_g1`, each encoded; assert the two encodings are equal and differ from the generator's
    * `[ ]`   `add_g2_of_the_generator_to_itself_equals_its_multiple_by_two`: the same over the second group
    * `[ ]`   `msm_g1_equals_the_multiple_by_the_sum_of_its_scalars`: arrange terms from `build_msm_g1_term` pairing the generator with two and with three, and the scalar five; act `msm_g1` and `mul_g1` by five, each encoded; assert the encodings are equal
    * `[ ]`   `msm_g2_equals_the_multiple_by_the_sum_of_its_scalars`: the same over the second group
    * `[ ]`   `pairing_product_is_one_for_a_pairing_and_its_inverse`: arrange terms `(g1, g2)` and `(g1 · (r - 1), g2)`; act `pairing_product_is_one`; assert `is_one` is `true`
    * `[ ]`   `pairing_product_is_not_one_for_a_single_generator_pairing`: arrange the one term `(g1, g2)`; act `pairing_product_is_one`; assert `is_one` is `false`
    * `[ ]`   `pairing_product_is_one_across_the_bilinear_exchange`: arrange terms `(g1 · 2, g2)` and `(g1, g2 · (r - 2))`; act `pairing_product_is_one`; assert `is_one` is `true`
    * `[ ]`   `sample_from_uniform_bytes_rejects_a_wrong_length`: arrange `build_sample_uniform_scalar_payload` with `uniform` holding 63 bytes; act `Bls12381Halo2curvesScalar::sample_from_uniform_bytes`; assert `Err(SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual: 63 })`
    * `[ ]`   `sample_from_uniform_bytes_reads_its_input_as_a_big_endian_integer`: arrange `uniform` holding 63 zero bytes followed by `05`; act `sample_from_uniform_bytes`, then `encode_scalar` of a clone of the exposed scalar; assert the exposed bytes equal the 32-byte big-endian encoding of five
    * `[ ]`   `sample_from_uniform_bytes_reduces_the_largest_input_into_the_scalar_field`: arrange `uniform` holding 64 bytes of `ff`; act `sample_from_uniform_bytes`, then `encode_scalar` of a clone of the exposed scalar, then `decode_scalar`; assert the decode returns `Ok`, the sampled scalar being below `r`
    * `[ ]`   `sample_from_uniform_bytes_draws_an_in_range_scalar_from_the_operating_system_source`: arrange `create_random_source` for `RandomSourceKind::OperatingSystem` and `fill_bytes` with `length: Some(Bls12381Halo2curvesScalar::UNIFORM_BYTES_LENGTH)`; act `sample_from_uniform_bytes` over the draw, then `encode_scalar` and `decode_scalar`; assert the decode returns `Ok`
    * `[ ]`   `bls12_381_halo2curves_pairing_declares_its_curve_arithmetic_encoding_and_versions`: act read `Bls12381Halo2curvesPairing::DECLARATION`; assert `curve` matches `PairingCurve::Bls12381`, `verifier_group_arithmetic` matches `VerifierGroupArithmetic::BothGroups`, `precompile_encoding` matches `PrecompileEncoding::Eip2537`, `adapter_version` equals `1`, and `interface_version` equals `PAIRING_INTERFACE_VERSION`
    * `[ ]`   Every test block carries the full `Contract`, `Arrange`, `Act`, `Assert` header and the inline markers; every payload is built through its family builder with only the overrides the test depends on

  * `[ ]`   `construction`
    * `[ ]`   `Bls12381Halo2curvesPairing::try_new` is the concrete's only producer, and its only caller is the pairing factory, which reads `Bls12381Halo2curvesPairing::DECLARATION` before constructing
    * `[ ]`   A group element or scalar is produced only by the adapter's generators, arithmetic, and decoders, or by the scalar's sampling bound; no consumer constructs one from library values

  * `[ ]`   `adapters/pairing/src/bls12_381_halo2curves/mod.rs`
    * `[ ]`   Module declarations: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, `pub(crate) mod provides;`, and `#[cfg(test)] mod test;`
    * `[ ]`   `impl Bls12381Halo2curvesPairing` with `pub const DECLARATION: PairingDeclaration` and `pub fn try_new(_params: Bls12381Halo2curvesPairingConstructorParams) -> Bls12381Halo2curvesPairingTryNewReturn` returning `Ok(Bls12381Halo2curvesPairing)`
    * `[ ]`   `impl IPairingAdapter for Bls12381Halo2curvesPairing` with `type Scalar = Bls12381Halo2curvesScalar;`, `type G1 = Bls12381Halo2curvesG1;`, `type G2 = Bls12381Halo2curvesG2;`, and every method realizing its branches in the interaction spec, the decoders checking in the stated order and copying each 48-byte coordinate out of the fixed-size array before reversing it
    * `[ ]`   `impl Zeroize for Bls12381Halo2curvesScalar` and `impl Drop for Bls12381Halo2curvesScalar`, each setting `self.value` to `Fr::ZERO` and calling `black_box(&self.value)`
    * `[ ]`   `impl ISampleUniformScalar for Bls12381Halo2curvesScalar` with `const UNIFORM_BYTES_LENGTH: usize = 64;` and `sample_from_uniform_bytes` realizing its branches, the scalar moved into a `Secret` by `let Ok(scalar) = Secret::try_new(SecretConstructorParams { value });`
    * `[ ]`   Imports the family's names from `crate::factory::provides`, `Secret` and `SecretConstructorParams` from `domain`, `zeroize::Zeroize`, `core::hint::black_box`, the `halo2curves` names the context slice lists, and this module's types from `interface`
    * `[ ]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[ ]`   `adapters/pairing/src/bls12_381_halo2curves/provides.rs`
    * `[ ]`   `pub(crate) use super::interface::*;`, nothing else, so the concrete is visible to the crate's factory and to nothing outside the crate

  * `[ ]`   `directionality`
    * `[ ]`   `bls12_381_halo2curves` depends on the `factory` module's surface through `crate::factory::provides`, on `domain`, on `zeroize`, and on `halo2curves`; it depends on no other concrete and no concrete depends on it; the `factory` module is unchanged and depends on no concrete; among repository crates the crate still depends on `crates/domain` alone at runtime
    * `[ ]`   `pairing/factory` adds the family form's recorded cycle when the factory function constructs this concrete

  * `[ ]`   `requirements`
    * `[ ]`   `adapters/pairing/Cargo.toml` is unchanged, and `halo2curves` is named nowhere in the crate outside `adapters/pairing/src/bn254_halo2curves` and `adapters/pairing/src/bls12_381_halo2curves`
    * `[ ]`   `cargo check --all-targets --all-features`, `cargo fmt --check`, and `cargo deny check` complete without error; `cargo clippy --all-targets --all-features` reports nothing beyond the library target's unused-item warnings for the pairing concretes, which `pairing/factory` resolves by constructing them
    * `[ ]`   Every test in `bls12_381_halo2curves/test.rs` passes: the generator encodings match EIP-2537 and the identity encodes to zero bytes, the decoders round-trip and reject a wrong length, a nonzero padding byte, a non-canonical coordinate or scalar, a point off the curve, and a point outside the subgroup in both groups, the arithmetic and multi-scalar multiplication agree, the pairing-product check accepts a pairing with its inverse and the bilinear exchange and rejects a lone pairing, and sampling rejects a wrong length, reads its input big-endian, and returns a scalar below the group order from a production draw (CR-10 on BLS12-381; CR-05 for the sampled scalar)
    * `[ ]`   Code outside `adapters/pairing` naming `Bls12381Halo2curvesPairing` or anything under `bls12_381_halo2curves` fails to compile; the crate's public surface is the `factory` module's `provides`

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