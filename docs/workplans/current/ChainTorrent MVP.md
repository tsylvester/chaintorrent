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
    * `[✅]`   Does not create the Solidity, TypeScript, fuzz, or end-to-end workflow definitions, each a separate file created once by the ticket that first needs it, `contracts/evm/PairingLib` for `forge build`, `forge fmt --check`, and `forge test`, the first shell or webview ticket for the TypeScript linter, the first fuzz target for `cargo-fuzz`, and the first end-to-end scenario for the clean-runner job; no later ticket edits `rust.yml`
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

* `[ ]`   `domain/secret` **Secret-typed value that cannot be formatted, cloned, or serialized, exposes its value only through an explicit accessor, and zeroizes on drop; creates the `domain` crate**

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
    * `[✅]`   Module-level `#![allow(clippy::panic)]`; imports `Secret`, `SecretConstructorParamsOverrides`, `build_secret`, and `build_secret_constructor_params` from `super::provides`, `Zeroize` from `zeroize`, and `assert_not_impl_any` from `static_assertions`; every test holds `Vec<u8>` except where it holds the block-local recorder named in that test
    * `[✅]`   Compile-time assertion over `Secret<Vec<u8>>`
      * `[✅]`   Contract: `Secret<T>` implements none of `core::fmt::Debug`, `core::fmt::Display`, `Clone`, or `Copy`
      * `[✅]`   Arrange: the type `Secret<Vec<u8>>`
      * `[✅]`   Act: `assert_not_impl_any!` over that type naming `core::fmt::Debug`, `core::fmt::Display`, `Clone`, and `Copy`
      * `[✅]`   Assert: the module compiles only if none of the four traits is implemented
    * `[✅]`   `try_new_holds_the_value_in_params`
      * `[✅]`   Contract: any params → `Ok(Secret)` holding `params.value`
      * `[✅]`   Collaborators: none; the fixture is `build_secret_constructor_params::<Vec<u8>>`
      * `[✅]`   Arrange: `build_secret_constructor_params` with the value override set to a nonempty byte sequence, so the default empty vector and the given value differ
      * `[✅]`   Act: `Secret::try_new` on the built params
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(secret) = …;`; the held `value`, read through the field the child test module reaches, equals the same byte sequence written as a literal in the assertion
    * `[✅]`   `expose_returns_a_reference_to_the_held_value`
      * `[✅]`   Contract: a living secret → a shared reference to the held value, no copy
      * `[✅]`   Collaborators: none; the fixture is `build_secret::<Vec<u8>>`
      * `[✅]`   Arrange: `build_secret` with the value override set to a nonempty byte sequence, so the default empty vector and the held value differ
      * `[✅]`   Act: `secret.expose()`
      * `[✅]`   Assert: the returned reference equals the same byte sequence written as a literal in the assertion; the returned reference is address-identical to the secret's held `value` field under `core::ptr::eq`
    * `[✅]`   `dropping_a_secret_zeroizes_its_value`
      * `[✅]`   Contract: the secret's lifetime ends by move into a consumer that drops it → `Zeroize::zeroize` is called on the held value exactly once
      * `[✅]`   Collaborators: `Zeroize::zeroize` on the held value, replaced by a block-local struct implementing `Zeroize` whose `zeroize` increments an `Arc<AtomicUsize>` counter shared with the test; the struct implements `Default` to meet the builder's bound
      * `[✅]`   Arrange: the shared counter at its initial value; the recorder holding a handle to it; `build_secret` with the value override set to the recorder
      * `[✅]`   Act: `drop(secret)`
      * `[✅]`   Assert: the shared counter equals the literal one
    * `[✅]`   `exposing_a_secret_does_not_zeroize_it`
      * `[✅]`   Contract: a living secret → `expose` returns a shared reference with no side effect on the held value
      * `[✅]`   Collaborators: `Zeroize::zeroize` on the held value, replaced by a block-local struct implementing `Zeroize` whose `zeroize` increments an `Arc<AtomicUsize>` counter shared with the test; the struct implements `Default` to meet the builder's bound
      * `[✅]`   Arrange: the shared counter at its initial value; the recorder holding a handle to it; `build_secret` with the value override set to the recorder
      * `[✅]`   Act: `secret.expose()`
      * `[✅]`   Assert: the shared counter equals the literal zero while the secret is alive
    * `[✅]`   `try_new_moves_the_value_without_copying_it`
      * `[✅]`   Contract: any params → `Ok(Secret)` holding `params.value`, moved without copy
      * `[✅]`   Collaborators: none; the fixture is `build_secret_constructor_params::<Vec<u8>>`
      * `[✅]`   Arrange: a nonempty byte vector bound to a local whose heap address is read before the call; `build_secret_constructor_params` with the value override set to that vector
      * `[✅]`   Act: `Secret::try_new` on the built params
      * `[✅]`   Assert: the result binds through the irrefutable `let Ok(secret) = …;`; the heap address of the held `value` equals the address read before the call
    * `[✅]`   `dropping_a_secret_during_unwinding_zeroizes_its_value`
      * `[✅]`   Contract: the secret's lifetime ends by unwinding → `Zeroize::zeroize` is called on the held value exactly once
      * `[✅]`   Collaborators: `Zeroize::zeroize` on the held value, replaced by a block-local struct implementing `Zeroize` whose `zeroize` increments an `Arc<AtomicUsize>` counter shared with the test; the struct implements `Default` to meet the builder's bound
      * `[✅]`   Arrange: the shared counter at its initial value; the recorder holding a handle to it; `build_secret` with the value override set to the recorder; a closure that takes the secret by move and panics
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

  * `[ ]`   `requirements`
    * `[✅]`   `crates/domain/Cargo.toml` carries exactly the tables and keys stated above, and no serialization dependency
    * `[✅]`   `cargo check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo fmt --check` complete without error with the `domain` crate as the workspace's only member
    * `[✅]`   The `assert_not_impl_any!` assertion compiles, proving `Secret` implements none of `Debug`, `Display`, `Clone`, or `Copy` (CR-07, formatting excluded at compile time)
    * `[ ]`   `expose_returns_a_reference_to_the_held_value` passes
    * `[✅]`   `dropping_a_secret_zeroizes_its_value` passes (CR-07 zeroization)
    * `[✅]`   Code outside `crates/domain/src/secret` reading the `value` field fails to compile
    * `[✅]`   A production `unwrap`, `expect`, `panic!`, numeric `as`, or `unsafe` block in the first production crate, `domain/secret`, is rejected by `cargo clippy` under the inherited lint table
    * `[✅]`   `cargo check`, `cargo clippy`, and `cargo fmt --check` at the repository root complete without error

* `[ ]`   `random/os` **Operating-system randomness concrete, the one source every production draw passes through; creates the `adapters/random` crate and authors the randomness family's generic interface, declaration, and mock**

  * `[✅]`   `objective`
    * `[✅]`   Problem: master scalars, capsule randomness, piece-group keys, and IVs come from a cryptographic random source (CR-05), and every production path draws through one repo-owned interface, so no module outside the concrete names the generator's library
    * `[✅]`   Functional: the family's generic interface draws a requested number of bytes and returns exactly that many bytes inside a `Secret`, including zero for a zero-length request, so every draw is zeroized when dropped whether or not the caller keeps it; every implementation, including the default mock, obeys the requested length
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
    * `[✅]`   `FillBytesErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]` and the one variant `OperatingSystem(OsRandomSourceFillBytesErrorReturn)`, the operating-system concrete's error carried unchanged; each concrete's error is its own variant
    * `[✅]`   `FillBytesReturn`, the type alias `Result<FillBytesSuccessReturn, FillBytesErrorReturn>`
    * `[✅]`   `IRandomSourceAdapter`, an object-safe trait with `fn declaration(&self) -> RandomSourceDeclaration;` and `fn fill_bytes(&self, params: FillBytesParams, payload: FillBytesPayload) -> FillBytesReturn;`; the declaration comes from the adapter itself, including through `Box<dyn IRandomSourceAdapter>`
    * `[✅]`   Imports `domain::Secret` and `OsRandomSourceFillBytesErrorReturn` from `crate::os::provides`; names no vendor

  * `[✅]`   `adapters/random/src/os/interface.rs`
    * `[✅]`   `OsRandomSource`, the unit struct `pub struct OsRandomSource;`, the adapter over the operating system's generator
    * `[✅]`   `OsRandomSourceConstructorParams`, the fieldless struct `pub struct OsRandomSourceConstructorParams;`, the constructor's deps slot
    * `[✅]`   `OsRandomSourceTryNewReturn`, the type alias `Result<OsRandomSource, Infallible>`; the error arm is uninhabited because the operating-system source takes no configuration
    * `[✅]`   `OsRandomSourceFillBytesErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]`, which `getrandom::Error` satisfies, and the one variant `OperatingSystem(getrandom::Error)`, the generator's failure carried unchanged
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
    * `[✅]`   `FillBytesSuccessReturnOverrides`, `#[derive(Default)]`, one field `pub bytes: Option<Secret<Vec<u8>>>`; `build_fill_bytes_success_return(overrides: FillBytesSuccessReturnOverrides) -> FillBytesSuccessReturn`, the bytes defaulting to `build_secret(SecretConstructorParamsOverrides { value: Some(vec![0u8; 32]) })`, matching the payload builder's default length; tests needing another length supply an override
    * `[✅]`   `MockIRandomSourceAdapter`, the unit struct `pub struct MockIRandomSourceAdapter;`, implementing `IRandomSourceAdapter` with `declaration()` returning the default `build_random_source_declaration(Default::default())` and `fill_bytes` returning `Ok(build_fill_bytes_success_return(FillBytesSuccessReturnOverrides { bytes: Some(build_secret(SecretConstructorParamsOverrides { value: Some(vec![0u8; payload.length]) })) }))`; the default mock returns exactly the requested length, including zero, and a test needing a failing or malformed source implements the trait on its own local struct
    * `[✅]`   No builder for `FillBytesParams`, which is fieldless and used by its production value, or for `RandomSourceKind`, an enum; no corruptions type and no invalidator, since no value this interface owns arrives as untrusted data
    * `[✅]`   Imports `Secret`, `build_secret`, and `SecretConstructorParamsOverrides` from `domain`, and this module's types from `super::interface`

  * `[✅]`   `adapters/random/src/factory/mod.rs`
    * `[✅]`   Module wiring only: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, and `pub mod provides;`, nothing else

  * `[✅]`   `adapters/random/src/factory/provides.rs`
    * `[✅]`   `pub use super::interface::*;` and `#[cfg(any(test, feature = "mocks"))] pub use super::mock::*;`, nothing else

  * `[ ]`   `adapters/random/src/os/mock.rs`
    * `[ ]`   `build_os_random_source() -> OsRandomSource`, returning the real instance from `OsRandomSource::try_new(OsRandomSourceConstructorParams)` through the irrefutable pattern `let Ok(source) = …;`
    * `[ ]`   No overrides type, corruptions type, or invalidator for `OsRandomSourceConstructorParams`, which is fieldless, used by its production value, and never arrives as untrusted data; no `OsRandomSource` overrides, invalidator, or mock function, since the concrete is built as a real instance and owns no free function
    * `[ ]`   Imports this module's types from `super::interface`

  * `[ ]`   `adapters/random/src/os/test.rs`
    * `[ ]`   Module-level `#![allow(clippy::expect_used)]`; imports `OsRandomSource` and `build_os_random_source` from `super::provides`, `FillBytesParams`, `FillBytesPayloadOverrides`, `IRandomSourceAdapter`, `RandomSourceKind`, `RANDOM_SOURCE_INTERFACE_VERSION`, and `build_fill_bytes_payload` from `crate::factory::provides`, and `HashSet` from `std::collections`
    * `[ ]`   `fill_bytes_returns_the_number_of_bytes_requested`
      * `[ ]`   Contract: `getrandom::fill` returns `Ok(())` over a buffer of `payload.length` bytes → `Ok(FillBytesSuccessReturn)` whose `Secret` holds exactly `payload.length` bytes
      * `[ ]`   Collaborators: `getrandom::fill`, the operating system's generator, called for real as the outer edge; fixtures `build_os_random_source` and `build_fill_bytes_payload`; `FillBytesParams` by its production value
      * `[ ]`   Arrange: `build_fill_bytes_payload` with the length override set to 48, a nonzero length that differs from the builder's default of 32, so a source returning the default width fails
      * `[ ]`   Act: `fill_bytes(FillBytesParams, payload)` on the built source
      * `[ ]`   Assert: the success arm is extracted with `expect`; the length of `bytes.expose()` equals the literal 48 written in the assertion
    * `[ ]`   `fill_bytes_returns_an_empty_draw_for_a_zero_length`
      * `[ ]`   Contract: `payload.length` of zero takes the drawn branch with an empty buffer → `Ok(FillBytesSuccessReturn)` whose `Secret` holds no bytes
      * `[ ]`   Collaborators: `getrandom::fill`, the operating system's generator, called for real as the outer edge; fixtures `build_os_random_source` and `build_fill_bytes_payload`; `FillBytesParams` by its production value
      * `[ ]`   Arrange: `build_fill_bytes_payload` with the length override set to 0, so a source that ignores the length and returns the builder's default width fails
      * `[ ]`   Act: `fill_bytes(FillBytesParams, payload)` on the built source
      * `[ ]`   Assert: the success arm is extracted with `expect`; `bytes.expose()` is empty
    * `[ ]`   `fill_bytes_draws_pairwise_distinct_segments_within_a_draw`
      * `[ ]`   Contract: a drawn buffer is filled by the generator → segments of a fixed width within one draw are pairwise distinct
      * `[ ]`   Collaborators: `getrandom::fill`, the operating system's generator, called for real as the outer edge; fixtures `build_os_random_source` and `build_fill_bytes_payload`; `FillBytesParams` by its production value
      * `[ ]`   Arrange: `build_fill_bytes_payload` with the length override set to 1024, which splits into segments of 32 bytes, so a zero-filled or repeating buffer yields fewer distinct segments than segments
      * `[ ]`   Act: `fill_bytes(FillBytesParams, payload)` on the built source
      * `[ ]`   Assert: the success arm is extracted with `expect`; the 32-byte segments of `bytes.expose()` collected into a `HashSet` number the literal 32 written in the assertion
    * `[ ]`   `fill_bytes_fills_a_draw_with_more_than_one_distinct_byte_value`
      * `[ ]`   Contract: a drawn buffer is filled by the generator → a draw of a fixed length holds more than one distinct byte value
      * `[ ]`   Collaborators: `getrandom::fill`, the operating system's generator, called for real as the outer edge; fixtures `build_os_random_source` and `build_fill_bytes_payload`; `FillBytesParams` by its production value
      * `[ ]`   Arrange: `build_fill_bytes_payload` with the length override set to 256, so a zero-filled buffer holds one distinct byte value
      * `[ ]`   Act: `fill_bytes(FillBytesParams, payload)` on the built source
      * `[ ]`   Assert: the success arm is extracted with `expect`; the bytes of `bytes.expose()` collected into a `HashSet` number more than the literal 1 written in the assertion
    * `[ ]`   `os_random_source_declares_its_adapter_and_interface_versions`
      * `[ ]`   Contract: `OsRandomSource::DECLARATION` is `RandomSourceDeclaration { source: RandomSourceKind::OperatingSystem, adapter_version: 1, interface_version: RANDOM_SOURCE_INTERFACE_VERSION }`, readable from the type before any instance exists
      * `[ ]`   Collaborators: none; no instance is built
      * `[ ]`   Arrange: none; the type alone
      * `[ ]`   Act: reading `OsRandomSource::DECLARATION`
      * `[ ]`   Assert: `source` matches `RandomSourceKind::OperatingSystem` under `matches!`; `adapter_version` equals the literal 1 written in the assertion; `interface_version` equals `RANDOM_SOURCE_INTERFACE_VERSION`
    * `[ ]`   `declaration_returns_the_associated_declaration`
      * `[ ]`   Contract: `declaration()` on a living source → `Self::DECLARATION`
      * `[ ]`   Collaborators: none; the fixture is `build_os_random_source`
      * `[ ]`   Arrange: `build_os_random_source()`
      * `[ ]`   Act: `declaration()` on the built source through `IRandomSourceAdapter`
      * `[ ]`   Assert: `source` matches `RandomSourceKind::OperatingSystem` under `matches!`; `adapter_version` equals the literal 1 written in the assertion; `interface_version` equals `RANDOM_SOURCE_INTERFACE_VERSION`
    * `[ ]`   `try_new_returns_the_operating_system_source`
      * `[ ]`   Contract: any params → `Ok(OsRandomSource)`
      * `[ ]`   Collaborators: none; `OsRandomSourceConstructorParams` by its production value
      * `[ ]`   Arrange: `OsRandomSourceConstructorParams` by its production value
      * `[ ]`   Act: `OsRandomSource::try_new(OsRandomSourceConstructorParams)`
      * `[ ]`   Assert: `result.is_ok()` is true

  * `[✅]`   `construction`
    * `[✅]`   `OsRandomSource::try_new` is the concrete's only producer, and its only caller is the randomness factory, which returns it as `Box<dyn IRandomSourceAdapter>`; the boxed adapter reports its declaration through `IRandomSourceAdapter::declaration`

  * `[✅]`   `adapters/random/src/os/mod.rs`
    * `[✅]`   Module declarations: `mod interface;`, `#[cfg(test)] mod mock;`, `pub(crate) mod provides;`, and `#[cfg(test)] mod test;`
    * `[✅]`   `impl OsRandomSource` with `pub const DECLARATION: RandomSourceDeclaration` as the interaction spec states, and `pub fn try_new(_params: OsRandomSourceConstructorParams) -> OsRandomSourceTryNewReturn` returning `Ok(OsRandomSource)`
    * `[✅]`   `impl IRandomSourceAdapter for OsRandomSource` with `declaration()` returning `Self::DECLARATION` and `fn fill_bytes(&self, _params: FillBytesParams, payload: FillBytesPayload) -> FillBytesReturn`, which allocates `vec![0u8; payload.length]`, calls `getrandom::fill` on it, moves the buffer into a `Secret` by `let Ok(bytes) = Secret::try_new(SecretConstructorParams { value: buffer });`, and then matches the fill result into the two branches of the interaction spec
    * `[✅]`   Imports the factory's names from `crate::factory::provides`, `Secret` and `SecretConstructorParams` from `domain`, and this module's types from `interface`
    * `[✅]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[✅]`   `adapters/random/src/os/provides.rs`
    * `[✅]`   `pub(crate) use super::interface::*;` and `#[cfg(test)] pub(crate) use super::mock::*;`, nothing else, so the concrete is visible to the crate's factory and to nothing outside the crate

  * `[✅]`   `directionality`
    * `[✅]`   `os` depends on the `factory` module's surface through `crate::factory::provides`, on `domain`, and on `getrandom`; the `factory` module depends on `domain` and on `os`'s error type through `crate::os::provides`; among repository crates the crate depends on `crates/domain` alone, inward; nothing depends on the crate yet
    * `[✅]`   The mutual dependency between the `factory` module and `os` is the family form's recorded cycle: a concrete implements the factory's trait, the factory's error enum carries the concrete's error, and the factory function constructs the concrete

  * `[ ]`   `requirements`
    * `[✅]`   `adapters/random/Cargo.toml` carries exactly the tables and keys stated above, and `getrandom` is named nowhere in the crate outside `adapters/random/src/os`
    * `[✅]`   `FillBytesErrorReturn` and `OsRandomSourceFillBytesErrorReturn` derive `Debug`, `PartialEq`, and `Eq`
    * `[✅]`   `cargo check --all-targets --all-features` and `cargo fmt --check` complete without error; `cargo clippy --all-targets --all-features` reports nothing beyond the library target's unused-item warnings for the `os` concrete, which `random/factory` resolves by constructing the concrete
    * `[✅]`   `fill_bytes_returns_the_number_of_bytes_requested` passes
    * `[✅]`   `fill_bytes_returns_an_empty_draw_for_a_zero_length` passes
    * `[ ]`   `fill_bytes_draws_pairwise_distinct_segments_within_a_draw` passes (CR-05, the source's non-repetition)
    * `[✅]`   `fill_bytes_fills_a_draw_with_more_than_one_distinct_byte_value` passes (CR-05, the source fills what it is asked to fill)
    * `[✅]`   `os_random_source_declares_its_adapter_and_interface_versions` passes
    * `[✅]`   A generator failure is returned as `FillBytesErrorReturn::OperatingSystem` holding `OsRandomSourceFillBytesErrorReturn::OperatingSystem` with the `getrandom::Error` unchanged, fixed by the error arm's type; the failure branch has no unit test, since the operating system's generator cannot be driven to fail from a test and the vendor is not mocked
    * `[✅]`   Code outside `adapters/random` naming `OsRandomSource` or anything under `os` fails to compile; the crate's public surface is the `factory` module's `provides`

* `[ ]`   `random/factory` **Randomness factory constructing the concrete a configuration names and returning it behind the family's trait, which reports its declaration; carries the family's integration test and the grouping's commit**

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
    * `[✅]`   `CreateRandomSourceSuccessReturn`, a struct with `pub adapter: Box<dyn IRandomSourceAdapter>`; callers read its declaration through `adapter.declaration()`, so no separate declaration can be paired with the boxed source
    * `[✅]`   `CreateRandomSourceErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]`, which `Infallible` satisfies, and the one variant `OperatingSystem(Infallible)`, the operating-system concrete's constructor error carried unchanged; each concrete's constructor error is its own variant
    * `[✅]`   `CreateRandomSourceReturn`, the type alias `Result<CreateRandomSourceSuccessReturn, CreateRandomSourceErrorReturn>`
    * `[✅]`   `CreateRandomSourceFn`, the type alias `fn(&CreateRandomSourceDeps, CreateRandomSourceParams, CreateRandomSourcePayload) -> CreateRandomSourceReturn`
    * `[✅]`   Adds the import of `core::convert::Infallible`; every item `random/os` authored in this file is unchanged

  * `[✅]`   `adapters/random/src/factory/interaction.spec.md`
    * `[✅]`   `create_random_source`, operating system: condition: `params.kind` is `RandomSourceKind::OperatingSystem`; decision: a `match` on `params.kind`; dependency call: `OsRandomSource::try_new(OsRandomSourceConstructorParams)`, exactly once; outcome: `Ok(CreateRandomSourceSuccessReturn { adapter: Box::new(source) })`; the boxed source reports `OsRandomSource::DECLARATION` through the trait method
    * `[✅]`   The operating-system constructor's error arm is uninhabited, so its success is destructured irrefutably and that branch has no failure outcome; `CreateRandomSourceErrorReturn::OperatingSystem` carries its error type in the return union
    * `[✅]`   `params.kind` selects the concrete; `deps` and `payload` carry nothing and are not read; the `match` is exhaustive over `RandomSourceKind`, so a kind with no branch fails to compile

  * `[✅]`   `adapters/random/src/factory/mock.rs`
    * `[✅]`   `CreateRandomSourceParamsOverrides`, `#[derive(Default)]`, one field `pub kind: Option<RandomSourceKind>`; `build_create_random_source_params(overrides: CreateRandomSourceParamsOverrides) -> CreateRandomSourceParams`, the kind defaulting to `RandomSourceKind::OperatingSystem`
    * `[✅]`   `CreateRandomSourceSuccessReturnOverrides`, `#[derive(Default)]`, with `pub adapter: Option<Box<dyn IRandomSourceAdapter>>`; `build_create_random_source_success_return(overrides: CreateRandomSourceSuccessReturnOverrides) -> CreateRandomSourceSuccessReturn`, the adapter defaulting to `Box::new(MockIRandomSourceAdapter)`; a test needing another declaration provides an adapter whose `declaration()` returns it
    * `[✅]`   `mock_create_random_source(_deps: &CreateRandomSourceDeps, _params: CreateRandomSourceParams, _payload: CreateRandomSourcePayload) -> CreateRandomSourceReturn`, returning `Ok(build_create_random_source_success_return(Default::default()))`
    * `[✅]`   No builder for the fieldless `CreateRandomSourceDeps` and `CreateRandomSourcePayload`, used by their production values, or for the enum `CreateRandomSourceErrorReturn`; every symbol `random/os` authored in this file is unchanged

  * `[ ]`   `adapters/random/src/factory/test.rs`
    * `[ ]`   Module-level `#![allow(clippy::expect_used)]`; imports `create_random_source`, `CreateRandomSourceDeps`, `CreateRandomSourcePayload`, `CreateRandomSourceParamsOverrides`, `build_create_random_source_params`, `RandomSourceKind`, and `RANDOM_SOURCE_INTERFACE_VERSION` from `super::provides`
    * `[ ]`   `create_random_source_returns_the_operating_system_source_for_its_kind`
      * `[ ]`   Contract: `params.kind` is `RandomSourceKind::OperatingSystem` → `Ok(CreateRandomSourceSuccessReturn { adapter })` holding the operating-system concrete, which reports `OsRandomSource::DECLARATION` through `declaration()`
      * `[ ]`   Collaborators: `OsRandomSource::try_new`, the concrete the factory constructs, runs for real; fixture `build_create_random_source_params`; `CreateRandomSourceDeps` and `CreateRandomSourcePayload` by their production values
      * `[ ]`   Arrange: `build_create_random_source_params` with the kind override set to `RandomSourceKind::OperatingSystem`
      * `[ ]`   Act: `create_random_source(&CreateRandomSourceDeps, params, CreateRandomSourcePayload)`
      * `[ ]`   Assert: the success arm is extracted with `expect`; `adapter.declaration()` has `source` matching `RandomSourceKind::OperatingSystem` under `matches!`, `adapter_version` equal to the literal 1 written in the assertion, and `interface_version` equal to `RANDOM_SOURCE_INTERFACE_VERSION`

  * `[✅]`   `construction`
    * `[✅]`   The composition root calls `create_random_source` with `&CreateRandomSourceDeps`, `CreateRandomSourceParams` holding the configured `RandomSourceKind`, and `CreateRandomSourcePayload`, and places the returned `Box<dyn IRandomSourceAdapter>` in each consumer's deps; no consumer constructs a concrete

  * `[✅]`   `adapters/random/src/factory/mod.rs`
    * `[✅]`   Adds `#[cfg(test)] mod test;` to the wiring `random/os` authored
    * `[✅]`   `pub fn create_random_source(_deps: &CreateRandomSourceDeps, params: CreateRandomSourceParams, _payload: CreateRandomSourcePayload) -> CreateRandomSourceReturn`, a `match` on `params.kind` whose `RandomSourceKind::OperatingSystem` arm binds the concrete by `let Ok(source) = OsRandomSource::try_new(OsRandomSourceConstructorParams);` and returns `Ok(CreateRandomSourceSuccessReturn { adapter: Box::new(source) })`
    * `[✅]`   Imports `OsRandomSource` and `OsRandomSourceConstructorParams` from `crate::os::provides`, and this module's types from `interface`
    * `[✅]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[✅]`   `adapters/random/src/factory/provides.rs`
    * `[✅]`   Adds `pub use super::create_random_source;` to the re-exports `random/os` authored

  * `[ ]`   `adapters/random/tests/integration_test.rs`
    * `[ ]`   Outside the crate, so it reaches only the crate's public surface: module-level `#![allow(clippy::expect_used)]`; imports from the `random` crate `create_random_source`, `CreateRandomSourceDeps`, `CreateRandomSourcePayload`, `CreateRandomSourceParamsOverrides`, `build_create_random_source_params`, `FillBytesParams`, `FillBytesPayloadOverrides`, `build_fill_bytes_payload`, `RandomSourceKind`, and `RANDOM_SOURCE_INTERFACE_VERSION`, and `HashSet` from `std::collections`
    * `[ ]`   `a_source_from_the_factory_draws_random_bytes_through_the_family_trait`
      * `[ ]`   Contract: a source obtained from `create_random_source` for `RandomSourceKind::OperatingSystem` draws the requested number of bytes through `fill_bytes`, and the draw is filled by the operating system's generator
      * `[ ]`   Boundary: the crate's public surface; the real chain `create_random_source` → `OsRandomSource::try_new` → `OsRandomSource::fill_bytes` → `getrandom::fill`
      * `[ ]`   Mocked: nothing; the operating system's generator is the outer edge and runs real, so this test proves the generator's output reaches the caller and does not prove the generator's quality
      * `[ ]`   Arrange: the adapter returned by `create_random_source` with `build_create_random_source_params` carrying the kind override `RandomSourceKind::OperatingSystem`, `CreateRandomSourceDeps` and `CreateRandomSourcePayload` by their production values; `build_fill_bytes_payload` with the length override set to 1024, which splits into segments of 32 bytes and differs from the builder's default of 32
      * `[ ]`   Act: `fill_bytes(FillBytesParams, payload)` on the returned adapter
      * `[ ]`   Assert: the success arm is extracted with `expect`; the length of `bytes.expose()` equals the literal 1024 written in the assertion; the 32-byte segments of `bytes.expose()` collected into a `HashSet` number the literal 32 written in the assertion
    * `[ ]`   `a_source_from_the_factory_draws_an_empty_buffer_for_a_zero_length`
      * `[ ]`   Contract: a source obtained from `create_random_source` for `RandomSourceKind::OperatingSystem` draws zero bytes for a `payload.length` of zero through `fill_bytes`
      * `[ ]`   Boundary: the crate's public surface; the real chain `create_random_source` → `OsRandomSource::try_new` → `OsRandomSource::fill_bytes` → `getrandom::fill`
      * `[ ]`   Mocked: nothing; the operating system's generator is the outer edge and runs real
      * `[ ]`   Arrange: the adapter returned by `create_random_source` with `build_create_random_source_params` carrying the kind override `RandomSourceKind::OperatingSystem`, `CreateRandomSourceDeps` and `CreateRandomSourcePayload` by their production values; `build_fill_bytes_payload` with the length override set to 0, which differs from the builder's default of 32
      * `[ ]`   Act: `fill_bytes(FillBytesParams, payload)` on the returned adapter
      * `[ ]`   Assert: the success arm is extracted with `expect`; `bytes.expose()` is empty
    * `[ ]`   `a_source_from_the_factory_reports_the_operating_system_declaration`
      * `[ ]`   Contract: a source obtained from `create_random_source` for `RandomSourceKind::OperatingSystem` reports `OsRandomSource::DECLARATION` through `declaration()`
      * `[ ]`   Boundary: the crate's public surface; the real chain `create_random_source` → `OsRandomSource::try_new` → `OsRandomSource::declaration`
      * `[ ]`   Mocked: nothing; the chain has no outer edge beyond the standard library
      * `[ ]`   Arrange: the adapter returned by `create_random_source` with `build_create_random_source_params` carrying the kind override `RandomSourceKind::OperatingSystem`, `CreateRandomSourceDeps` and `CreateRandomSourcePayload` by their production values
      * `[ ]`   Act: `declaration()` on the returned adapter
      * `[ ]`   Assert: `source` matches `RandomSourceKind::OperatingSystem` under `matches!`; `adapter_version` equals the literal 1 written in the assertion; `interface_version` equals `RANDOM_SOURCE_INTERFACE_VERSION`

  * `[✅]`   `directionality`
    * `[✅]`   The `factory` module depends on `os` through `crate::os::provides` and on its own interface; `os` depends on the `factory` module's surface, the family form's recorded cycle; the crate's public surface is the `factory` module's `provides`; nothing depends on the crate yet

  * `[✅]`   `requirements`
    * `[✅]`   `create_random_source_returns_the_operating_system_source_for_its_kind` passes
    * `[✅]`   `CreateRandomSourceErrorReturn` derives `Debug`, `PartialEq`, and `Eq`
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

* `[ ]`   `pairing/bn254_arkworks` **BN254 pairing concrete on arkworks implementing the pairing family's generic interface, arithmetic trait, and reference trait over the EIP-196 and EIP-197 encodings and the `Bn254V1` target-group value; creates the `adapters/pairing` crate and authors the family's traits, sampling bound, declaration, target-group encoding identifier, selection enum, and mock**

  * `[✅]`   `objective`
    * `[✅]`   Problem: the credential KEM, the envelope, and the delivery proof compute in a Type-3 pairing group whose curve the launch chain's precompiles dictate, so every group operation, subgroup check, pairing-product check, and precompile encoding passes through one repo-owned interface and no module outside a concrete names a curve library (CR-10)
    * `[✅]`   Problem: the credential KEM encapsulates and decapsulates a target-group value `K` the key-derivation family consumes as bytes, checks validity and well-formedness as pairing equations whose sides are divided, and refuses a trivial identity element; the envelope decrypts by removing a masked source-group element and rejects identity-element keys; the delivery proof computes Schnorr responses in the scalar field; and the decrypted credential is a pair of source-group elements that must be zeroized; so each of these operations passes through the family (CR-04; CR-07; CR-08; CR-09; `docs/research/cryptography.md`'s Credential KEM, Key Agreement, and Delivery Proof statements)
    * `[✅]`   Problem: the bytes the KEM hands the KDF are a target-group element's encoding, and a library's reduced pairing is a convention: `ark-ec`'s BN254 final exponentiation returns the Miller loop's value raised to `2x(6x^2 + 3x + 1)` times the exact exponent `(p^12 - 1) / r` for the curve seed `x`, and `halo2curves`' returns it raised to the exact exponent, so two holders on different libraries derive different wrapping keys from one capsule; the protocol fixes the value by one identifier per curve naming the pairing map, the generators, the tower, the exact exponent, and the serialization, every concrete declares it, the factory admits by it, and the hash-card pins it (CR-10; `docs/research/cryptography.md`'s pairing adapter capability declaration; the product requirements' canonical target-group value position)
    * `[✅]`   Problem: a contract recomputes every hash-to-scalar mapping as the digest reduced modulo the group order, and proves its own subgroup checks only against an on-curve point outside the subgroup; the group order is a constant of each curve library and such a point is found only with a library's curve internals, so neither is reachable by a consumer of the family, and a consumer that mirrors the Rust reference to a target suite names no curve library and holds no literal; so the family exposes both, each concrete reading them from the library it wraps (CR-10 on chain; CR-11; the dependency map's `pairing/bn254_arkworks`, `harness-crypto/generate/evm/constants`, and `harness-crypto/generate/evm/rejection_vectors` rows; the Pairing adapters and key derivation milestone's scope and exit; the technical requirements' generated-directory contents)
    * `[✅]`   Functional: the family's generic interface exposes the generators of both source groups, addition, scalar multiplication, and multi-scalar multiplication in either source group, the pairing-product check, and decoding and encoding of group elements and scalars in the target chain's precompile format
    * `[✅]`   Functional: the group-element, scalar, and encoded first-group, second-group, and scalar types are associated types of the generic interface, so a consumer names them through the family without knowing which concrete produced them; each encoder returns a distinct owned encoded type for each group and concrete, which preserves its group and curve, and a consumer reaches its wire bytes only by borrowing `as_ref()`
    * `[✅]`   Functional: every group element and scalar a concrete accepts from bytes is decoded by a fallible constructor that rejects a wrong length, a non-canonical field element, a point off the curve, and a point outside the prime-order subgroup, so no element that fails a check exists as a value
    * `[✅]`   Functional: a scalar type is sampled from uniform bytes the randomness family drew, by reduction of a fixed-width input modulo the group order, and the sampled scalar is returned inside a `Secret`
    * `[✅]`   Functional: every concrete declares its curve, whether the target chain's verifier has second-group arithmetic, its precompile encoding, its target-group encoding identifier, its adapter version, and the interface version it implements, readable before any instance exists, and names its own `PairingConcrete` variant
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
    * `[✅]`   Adapter: the pairing family's concrete that authors, in the family's `factory` module and as their producer, the generic interface, the sampling bound, the declaration and its enums, the target-group encoding identifier, the selection enum, the arithmetic trait, the reference trait, every method's types, and the family's mock
    * `[✅]`   Creates `factory/mod.rs` with the module wiring alone and `factory/provides.rs` with the interface and mock surface alone; the factory function, its types, its interaction spec, its unit test, its re-export, the consumer trait, and the family's integration test are `pairing/factory`'s
    * `[✅]`   Does not author the BN254 `halo2curves` concrete or either BLS12-381 concrete; each is its own node beneath the factory, and `pairing/bls12_381_arkworks` adds the BLS12-381 variants of `PairingCurve`, `VerifierGroupArithmetic`, and `PrecompileEncoding`
    * `[✅]`   Does not hash to a group or to a scalar; `hash-to-scalar/keccak256` maps domain-tagged bytes to a scalar through this family's scalar type
    * `[✅]`   Does not draw randomness; a consumer draws `UNIFORM_BYTES_LENGTH` bytes through the randomness family and hands the draw to the sampling bound
    * `[✅]`   Does not compute in the target group beyond the pairing product, its correction to the identifier's value, and its encoding: no target-group multiplication, exponentiation, or inversion is exposed
    * `[✅]`   Does not map a target-group value to a key; the key-derivation family derives from the encoding
    * `[✅]`   Does not read the correction from configuration; it is a property of this library under this identifier and is computed in `try_new`
    * `[✅]`   Does not decode, classify, or emit a rejection vector; `harness-crypto/generate/evm/rejection_vectors` runs the encoding through the family's decoders and emits it with their verdict
    * `[✅]`   Does not carry a commit

  * `[✅]`   `module`
    * `[✅]`   Bounded context: the `adapters/pairing` crate's `factory` module, holding the family's generic interface, arithmetic trait, and reference trait with their associated types and methods, every method's params, payload, success, error, and return types, the sampling bound with its types, the declaration and its enums, the target-group encoding identifier, the selection enum, the interface version, and the family's mock; and its private `bn254_arkworks` concrete, holding the adapter over arkworks' BN254 with its correction field and seed constant, its scalar, group-element, target-group, and encoded types over the library's elements, its constructor params, its implementations of the three traits, and the builder defaults for its owned types
    * `[✅]`   The node's files: `adapters/pairing/Cargo.toml`, `adapters/pairing/src/lib.rs`, the `factory` module's `interface.rs`, `mock.rs`, `mod.rs`, and `provides.rs`, and the `bn254_arkworks` module's `interface.rs`, `interaction.spec.md`, `mock.rs`, `test.rs`, `mod.rs`, and `provides.rs`
    * `[✅]`   Creates the crate at `adapters/pairing`, admitted by the workspace's `adapters/*` glob with no edit to the root manifest
    * `[✅]`   Outside: which scalars are secret and who produces them, hashing to a scalar, the KEM, envelope, and proof algebra built on the family, the key derivation from the target-group encoding, the Solidity mirror of the encodings, the cross-library agreement of the encodings, the decoders' verdicts as vectors, and the factory's selection and admission of a concrete

  * `[ ]`   `deps`
    * `[✅]`   `domain`, `crates/domain`, protocol and domain ring, path dependency; supplies `Secret` and `SecretConstructorParams`, the wrapper for uniform input, sampled scalars, encoded scalars, and encoded target-group values; direction inward, adapter ring on domain ring
    * `[✅]`   `domain` with its `mocks` feature, as a dev-dependency and through this crate's `mocks` feature; supplies `build_secret` and `SecretConstructorParamsOverrides`
    * `[✅]`   `zeroize` `1.9.0`, external crate, Apache-2.0 OR MIT, runtime dependency; supplies the `Zeroize` and `ZeroizeOnDrop` traits the sampling bound and the arithmetic trait require, its implementation for `Vec<Z: Zeroize>`, and its implementations for arkworks' short-Weierstrass affine points and `PairingOutput`; zeroization is a property of the type, not an external touchpoint, so no adapter wraps it
    * `[✅]`   `ark-bn254` `0.6.0`, `ark-ec` `0.6.0`, and `ark-ff` `0.6.0`, external crates, MIT OR Apache-2.0, runtime dependencies named only in `bn254_arkworks`; supply the curve, the pairing, the Miller loop, the group arithmetic, and the field arithmetic
    * `[ ]`   `hex` `0.4.3`, external crate, MIT OR Apache-2.0, dev-dependency; supplies `hex::decode` for the test vectors, in `mock.rs`'s test-only fixtures
    * `[ ]`   `num-bigint` `0.4.8`, the version `Cargo.lock` resolves for `ark-ff`, external crate, MIT OR Apache-2.0, dev-dependency; used only in `mock.rs`'s test-only fixtures, to compute the integer exponent `(p^12 - 1) / r`, and in `test.rs`, to compare roots
    * `[✅]`   `core::convert::Infallible`, standard library, the error arm of every method that has no failure; `core::marker::PhantomData`, standard library, in `factory/mock.rs`
    * `[✅]`   Reverse dependencies: every other pairing concrete and `pairing/factory`, through the `factory` module's surface

  * `[ ]`   `context_slice`
    * `[✅]`   From `domain`: `Secret::try_new(SecretConstructorParams { value })` returning `Result<Secret<T>, Infallible>`, and `Secret::expose(&self) -> &T`
    * `[✅]`   From `domain`'s mocks: `build_secret(SecretConstructorParamsOverrides<T>) -> Secret<T>` for `T: Zeroize + Default`
    * `[✅]`   From `zeroize`: the `Zeroize` trait's `zeroize(&mut self)`, the `ZeroizeOnDrop` marker trait, and their implementations for `Vec<Z: Zeroize>`, `Affine`, and `PairingOutput`
    * `[✅]`   From `ark-bn254`: `Bn254`, `Fq`, `Fq2` with its public fields `c0` and `c1` and `Fq2::new(c0, c1)`, `Fq6` with `Fq6::new(c0, c1, c2)`, `Fq12` with `Fq12::new(c0, c1)`, `Fr`, `G1Affine`, `G1Projective`, `G2Affine`, and `G2Projective`
    * `[✅]`   From `ark-ec`: `AffineRepr` for `generator()`, `xy()`, and `is_zero()`, true exactly for the identity; the short-Weierstrass affine `identity()`, `new_unchecked(x, y)`, `is_on_curve()`, `is_in_correct_subgroup_assuming_on_curve()`, and `get_point_from_x_unchecked(x, greatest)`, an `Option` of an on-curve point with that `x`, unchecked for the subgroup; the affine `+` and `* Fr` producing projective points; unary `-` on the affine point, returning `(x, -y)` and the identity for the identity; `CurveGroup::into_affine`; `VariableBaseMSM::msm_unchecked(bases, scalars)`; `pairing::Pairing::multi_pairing(a, b)` over iterators of borrowed affine points, returning `PairingOutput<Bn254>`, whose public field `0` is the `Fq12` value and whose identity is `Fq12::one()`; `Pairing::multi_miller_loop(a, b)` over the same iterators, returning `MillerLoopOutput<Bn254>` whose public field `0` is the unreduced `Fq12`; `Mul<Fr> for PairingOutput<Bn254>`, the exponentiation of a target-group value by a scalar; `Pairing::pairing(p, q)`; `Zeroize` for the affine point, which zeroizes `x`, `y`, and `infinity`, so a zeroized point is `new_unchecked(0, 0)`, and for `PairingOutput`, which zeroizes its `Fq12`
    * `[✅]`   From `ark-ff`: `Fr`'s `+`, `*`, and unary `-` modulo the group order; `Fr::from(u64)` and `Fq::from(u64)`; `Field::ONE`; `Field::pow(&self, exp: impl AsRef<[u64]>)`, infallible exponentiation by little-endian limbs, on `Fr` and on `Fq12`; `PrimeField::MODULUS`, the `BigInt<4>` of `Fr` and of `Fq`; `BigInteger::sub_with_borrow(&mut self, other: &Self) -> bool` and `BigInt::<4>::from(u64)`; `From<BigInt<4>> for num_bigint::BigUint`; the public fields `c0` and `c1` of the degree-two and degree-twelve extensions and `c0`, `c1`, and `c2` of the degree-six extension; `PrimeField::from_be_bytes_mod_order(&[u8])` and `into_bigint()` on `Fq` and on `Fr`, the `BigInt<4>` of `Fq` ordering as the integer; `BigInteger::to_bytes_be()`, 32 bytes for a `BigInt<4>`; `Zero::is_zero()` on `Fq`, on `Fq12`, and on `PairingOutput`; unary `-` on `Fq2`
    * `[ ]`   From `num-bigint`, in `mock.rs`'s test-only fixtures and in `test.rs`: `BigUint::from(u32)`, `BigUint::from_bytes_be(&[u8])`, `BigUint::pow(&self, u32)`, `Sub`, `Div`, and `Rem` between `BigUint`s, and `BigUint::to_u64_digits(&self) -> Vec<u64>`, little-endian limbs

  * `[ ]`   `adapters/pairing/Cargo.toml`
    * `[✅]`   `[package]` with `name = "pairing"`, `edition.workspace = true`, `rust-version.workspace = true`, and `publish.workspace = true`; no `version` key
    * `[✅]`   `[dependencies]` with `domain = { path = "../../crates/domain" }`, `zeroize = "1.9.0"`, `ark-bn254 = "0.6.0"`, `ark-ec = "0.6.0"`, and `ark-ff = "0.6.0"`, each with default features
    * `[ ]`   `[dev-dependencies]` with `domain = { path = "../../crates/domain", features = ["mocks"] }`, `hex = "0.4.3"`, and `num-bigint = "0.4.8"`
    * `[✅]`   `[features]` with `mocks = ["domain/mocks"]`
    * `[✅]`   `[lints]` with `workspace = true`
    * `[✅]`   No other table

  * `[✅]`   `adapters/pairing/src/lib.rs`
    * `[✅]`   The crate barrel: `mod factory;`, `mod bn254_arkworks;`, and `pub use factory::provides::*;`; each further concrete node contributes its own module line

  * `[✅]`   `adapters/pairing/src/factory/interface.rs`
    * `[✅]`   Imports `core::convert::Infallible`, `domain::Secret`, and `zeroize::{Zeroize, ZeroizeOnDrop}`; names no vendor and no concrete
    * `[✅]`   `PAIRING_INTERFACE_VERSION`, a `pub const` of type `u32` with value `1`
    * `[✅]`   `PairingCurve`, an enum with the variant `Bn254`; `VerifierGroupArithmetic`, an enum with the variant `FirstGroupOnly`; `PrecompileEncoding`, an enum with the variant `Eip196Eip197`
    * `[✅]`   `PairingConcrete`, an enum with `#[derive(Clone, Copy, PartialEq, Eq)]` and the variants `Bn254Arkworks`, `Bn254Halo2curves`, `Bls12381Arkworks`, and `Bls12381Halo2curves`, so `IPairingAdapter::CONCRETE` names a type this node produces
    * `[✅]`   `TargetGroupEncodingIdentifier`, an enum with `#[derive(Clone, Copy, Debug, PartialEq, Eq)]` and the variants `Bn254V1` and `Bls12381V1`, each with a doc comment; `Bn254V1`'s reads "The optimal ate pairing as EIP-197 fixes it, over the EIP-197 generators, on the tower `Fp2 = Fp[u] / (u^2 + 1)`, `Fp6 = Fp2[v] / (v^3 - (u + 9))`, `Fp12 = Fp6[w] / (w^2 - v)`; the pairing value is the Miller loop's value raised to the exact exponent `(p^12 - 1) / r`, not a fixed multiple of it; a target-group element is serialized as its twelve base-field coefficients in tower order, each the coefficient's 32-byte big-endian canonical integer."; `Bls12381V1`'s reads the same with "the CFRG pairing-friendly-curves draft" as the standard, "the EIP-2537 generators", the tower `Fp6 = Fp2[v] / (v^3 - (u + 1))`, and "48-byte"
    * `[✅]`   `PairingDeclaration`, a struct with `pub curve: PairingCurve`, `pub verifier_group_arithmetic: VerifierGroupArithmetic`, `pub precompile_encoding: PrecompileEncoding`, `pub target_group_encoding: TargetGroupEncodingIdentifier`, `pub adapter_version: u32`, and `pub interface_version: u32`
    * `[✅]`   The sampling bound's types: the fieldless `SampleUniformScalarParams`; `SampleUniformScalarPayload` with `pub uniform: Secret<Vec<u8>>`; `SampleUniformScalarSuccessReturn<S: Zeroize>` with `pub scalar: Secret<S>`; `SampleUniformScalarErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]` and the struct variant `WrongLength { expected: usize, actual: usize }`; `SampleUniformScalarReturn<S>`, the alias `Result<SampleUniformScalarSuccessReturn<S>, SampleUniformScalarErrorReturn>`
    * `[✅]`   `ISampleUniformScalar`, the sampling bound, `pub trait ISampleUniformScalar: Zeroize + ZeroizeOnDrop + Sized` with `const UNIFORM_BYTES_LENGTH: usize;` and `fn sample_from_uniform_bytes(params: SampleUniformScalarParams, payload: SampleUniformScalarPayload) -> SampleUniformScalarReturn<Self>;`
    * `[✅]`   `g1_generator`: the fieldless `G1GeneratorParams` and `G1GeneratorPayload`; `G1GeneratorSuccessReturn<G>` with `pub point: G`; `G1GeneratorReturn<G>`, the alias `Result<G1GeneratorSuccessReturn<G>, Infallible>`; `g2_generator`, the same shape under the `G2Generator` prefix
    * `[✅]`   `add_g1`: the fieldless `AddG1Params`; `AddG1Payload<G>` with `pub left: G` and `pub right: G`; `AddG1SuccessReturn<G>` with `pub sum: G`; `AddG1Return<G>`, the alias `Result<AddG1SuccessReturn<G>, Infallible>`; `add_g2`, the same shape under the `AddG2` prefix
    * `[✅]`   `mul_g1`: the fieldless `MulG1Params`; `MulG1Payload<G, S>` with `pub point: G` and `pub scalar: S`; `MulG1SuccessReturn<G>` with `pub product: G`; `MulG1Return<G>`, the alias `Result<MulG1SuccessReturn<G>, Infallible>`; `mul_g2`, the same shape under the `MulG2` prefix
    * `[✅]`   `msm_g1`: the fieldless `MsmG1Params`; `MsmG1Term<G, S>` with `pub base: G` and `pub scalar: S`; `MsmG1Payload<G, S>` with `pub terms: Vec<MsmG1Term<G, S>>`, so a base and its scalar cannot differ in count; `MsmG1SuccessReturn<G>` with `pub sum: G`; `MsmG1Return<G>`, the alias `Result<MsmG1SuccessReturn<G>, Infallible>`; `msm_g2`, the same shape under the `MsmG2` prefix with `MsmG2Term<G, S>`
    * `[✅]`   `pairing_product_is_one`: the fieldless `PairingProductIsOneParams`; `PairingProductTerm<G1, G2>` with `pub g1: G1` and `pub g2: G2`; `PairingProductIsOnePayload<G1, G2>` with `pub terms: Vec<PairingProductTerm<G1, G2>>`; `PairingProductIsOneSuccessReturn` with `pub is_one: bool`; `PairingProductIsOneReturn`, the alias `Result<PairingProductIsOneSuccessReturn, Infallible>`
    * `[✅]`   `decode_g1`: the fieldless `DecodeG1Params`; `DecodeG1SuccessReturn<G>` with `pub point: G`; `DecodeG1ErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]` and the variants `WrongLength { expected: usize, actual: usize }`, `NonCanonicalCoordinate`, `NotOnCurve`, and `NotInSubgroup`; `DecodeG1Return<G>`, the alias `Result<DecodeG1SuccessReturn<G>, DecodeG1ErrorReturn>`; `decode_g2`, the same shape under the `DecodeG2` prefix, `DecodeG2ErrorReturn` carrying the same derives and the same variants
    * `[✅]`   `decode_scalar`: the fieldless `DecodeScalarParams`; `DecodeScalarSuccessReturn<S>` with `pub scalar: S`; `DecodeScalarErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]` and the variants `WrongLength { expected: usize, actual: usize }` and `NonCanonical`; `DecodeScalarReturn<S>`, the alias `Result<DecodeScalarSuccessReturn<S>, DecodeScalarErrorReturn>`
    * `[✅]`   `encode_g1`: the fieldless `EncodeG1Params`; `EncodeG1Payload<G>` with `pub point: G`; `EncodeG1SuccessReturn<E>` with `pub bytes: E`; `EncodeG1Return<E>`, the alias `Result<EncodeG1SuccessReturn<E>, Infallible>`; `encode_g2`, the same shape under the `EncodeG2` prefix
    * `[✅]`   `encode_scalar`: the fieldless `EncodeScalarParams`; `EncodeScalarPayload<S>` with `pub scalar: S`; `EncodeScalarSuccessReturn<E: Zeroize>` with `pub bytes: Secret<E>`, since the scalar encoded may be secret; `EncodeScalarReturn<E>`, the alias `Result<EncodeScalarSuccessReturn<E>, Infallible>`
    * `[✅]`   `IPairingAdapter`, `pub trait IPairingAdapter` with `const DECLARATION: PairingDeclaration;` and `const CONCRETE: PairingConcrete;`, each supplied by its implementing concrete; `type Scalar: ISampleUniformScalar + Clone;`, `type G1: Clone;`, `type G2: Clone;`, `type EncodedG1: AsRef<[u8]> + Clone + PartialEq + Eq;`, `type EncodedG2: AsRef<[u8]> + Clone + PartialEq + Eq;`, and `type EncodedScalar: AsRef<[u8]> + Zeroize;`, the encoded types concrete-owned and distinct from each other and from another concrete's; the methods `g1_generator(&self, params: G1GeneratorParams, payload: G1GeneratorPayload) -> G1GeneratorReturn<Self::G1>`, `g2_generator` over `Self::G2`, `add_g1(&self, params: AddG1Params, payload: AddG1Payload<Self::G1>) -> AddG1Return<Self::G1>`, `add_g2` over `Self::G2`, `mul_g1(&self, params: MulG1Params, payload: MulG1Payload<Self::G1, Self::Scalar>) -> MulG1Return<Self::G1>`, `mul_g2` over `Self::G2` and `Self::Scalar`, `msm_g1(&self, params: MsmG1Params, payload: MsmG1Payload<Self::G1, Self::Scalar>) -> MsmG1Return<Self::G1>`, `msm_g2` over `Self::G2` and `Self::Scalar`, `pairing_product_is_one(&self, params: PairingProductIsOneParams, payload: PairingProductIsOnePayload<Self::G1, Self::G2>) -> PairingProductIsOneReturn`, `decode_g1(&self, params: DecodeG1Params, payload: &[u8]) -> DecodeG1Return<Self::G1>`, `decode_g2` over `Self::G2`, `decode_scalar(&self, params: DecodeScalarParams, payload: &[u8]) -> DecodeScalarReturn<Self::Scalar>`, `encode_g1(&self, params: EncodeG1Params, payload: EncodeG1Payload<Self::G1>) -> EncodeG1Return<Self::EncodedG1>`, `encode_g2(&self, params: EncodeG2Params, payload: EncodeG2Payload<Self::G2>) -> EncodeG2Return<Self::EncodedG2>`, and `encode_scalar(&self, params: EncodeScalarParams, payload: EncodeScalarPayload<Self::Scalar>) -> EncodeScalarReturn<Self::EncodedScalar>`; each decoder's payload is the untrusted wire bytes and its narrowing target the associated type
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
    * `[✅]`   Imports `core::convert::Infallible`; declares nothing beyond the items below
    * `[✅]`   `Bn254ArkworksPairing`, a struct with no derives and one field, `pub(super) reduced_pairing_correction: ark_bn254::Fr`, with the doc comment "The inverse in the scalar field of the multiple by which the library's reduced pairing exceeds the identifier's exact value, `2x(6x^2 + 3x + 1)` for the curve seed `x`."
    * `[✅]`   `Bn254ArkworksPairingConstructorParams`, the unit struct `pub struct Bn254ArkworksPairingConstructorParams;`, the constructor's deps slot; `Bn254ArkworksPairingTryNewReturn`, the alias `Result<Bn254ArkworksPairing, Infallible>`, the error arm uninhabited because the adapter takes no configuration
    * `[✅]`   `Bn254ArkworksScalar`, `#[derive(Clone)]`, with one field `pub(super) value: ark_bn254::Fr`; `Bn254ArkworksG1`, `#[derive(Clone)]`, with one field `pub(super) value: ark_bn254::G1Affine`; `Bn254ArkworksG2`, `#[derive(Clone)]`, with one field `pub(super) value: ark_bn254::G2Affine`
    * `[✅]`   `Bn254ArkworksGt`, a struct with no derives and one field, `pub(super) value: ark_ec::pairing::PairingOutput<ark_bn254::Bn254>`, the target-group value
    * `[✅]`   `Bn254ArkworksEncodedG1` and `Bn254ArkworksEncodedG2`, `pub struct`s with `#[derive(Clone, PartialEq, Eq)]` and one field each, `pub(super) bytes: [u8; 64]` and `pub(super) bytes: [u8; 128]`
    * `[✅]`   `Bn254ArkworksEncodedScalar`, a `pub struct` with no derives and one field `pub(super) bytes: [u8; 32]`; `Bn254ArkworksEncodedGt`, a `pub struct` with no derives and one field `pub(super) bytes: [u8; 384]`; each is returned only inside a `Secret`
    * `[✅]`   The `pub(super)` fields admit construction only inside `bn254_arkworks` and its child modules, so no code outside the concrete constructs a value of these types or mutates an encoding's bytes

  * `[✅]`   `adapters/pairing/src/bn254_arkworks/interaction.spec.md`
    * `[✅]`   The title `` # `bn254_arkworks` — interaction spec `` and one opening sentence stating the file as the branch contract for the `bn254_arkworks` module of the `pairing` crate, the pairing adapter over arkworks' BN254 encoding group elements as EIP-196 and EIP-197 precompile input, each branch stating condition, decision, dependency call, and the exact return outcome; then one `##` section per constructor, constant, and method, each headed by its full signature and holding a table with the columns `Branch`, `Condition`, `Decision`, `Dependency call`, and `Outcome`, in this order: `try_new`, `DECLARATION`, `CONCRETE`, `g1_generator`, `g2_generator`, `add_g1`, `add_g2`, `mul_g1`, `mul_g2`, `msm_g1`, `msm_g2`, `pairing_product_is_one`, `decode_g1`, `decode_g2`, `decode_scalar`, `encode_g1`, `encode_g2`, `encode_scalar`, `UNIFORM_BYTES_LENGTH`, `sample_from_uniform_bytes`, `add_scalar`, `mul_scalar`, `neg_scalar`, `neg_g1`, `neg_g2`, `is_identity_g1`, `is_identity_g2`, `pairing_product`, `encode_gt`, `scalar_field_order`, `g1_outside_subgroup_encoding`, `g2_outside_subgroup_encoding`, then `Ordering and edges`
    * `[✅]`   `try_new`: one branch, construct; condition any params; decision none; dependency calls, in order: `Fr::from(BN254_SEED)` as `seed`; `(seed * seed * Fr::from(6u64) + seed * Fr::from(3u64) + Fr::ONE) * seed * Fr::from(2u64)` as `multiple`; `let mut exponent = Fr::MODULUS;` then `let _ = exponent.sub_with_borrow(&BigInt::from(2u64));`, the group order minus two; `multiple.pow(exponent)` as `reduced_pairing_correction`; outcome `Ok(Bn254ArkworksPairing { reduced_pairing_correction })`; the error arm has no branch, `Infallible` being uninhabited
    * `[✅]`   `DECLARATION`: the inherent constant `PairingDeclaration { curve: PairingCurve::Bn254, verifier_group_arithmetic: VerifierGroupArithmetic::FirstGroupOnly, precompile_encoding: PrecompileEncoding::Eip196Eip197, target_group_encoding: TargetGroupEncodingIdentifier::Bn254V1, adapter_version: 1, interface_version: PAIRING_INTERFACE_VERSION }`, readable from the type before any instance exists; the trait constant `IPairingAdapter::DECLARATION` is this constant
    * `[✅]`   `CONCRETE`: the trait constant `PairingConcrete::Bn254Arkworks`
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
    * `[✅]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`; imports from `super::interface` every type the items below name, `core::marker::PhantomData`, `domain::{Secret, SecretConstructorParamsOverrides, build_secret}`, and `zeroize::Zeroize`
    * `[✅]`   `PairingDeclarationOverrides`, `#[derive(Default)]`, one `Option` per field of `PairingDeclaration`; `build_pairing_declaration(overrides: PairingDeclarationOverrides) -> PairingDeclaration`, defaulting `curve` to `PairingCurve::Bn254`, `verifier_group_arithmetic` to `VerifierGroupArithmetic::FirstGroupOnly`, `precompile_encoding` to `PrecompileEncoding::Eip196Eip197`, `target_group_encoding` to `TargetGroupEncodingIdentifier::Bn254V1`, `adapter_version` to `1`, and `interface_version` to `PAIRING_INTERFACE_VERSION`
    * `[✅]`   For each generic struct below, an overrides struct named by the type with the suffix `Overrides`, `#[derive(Default)]`, one `Option` per field over the struct's type parameters, and a builder `build_` followed by the type's name in snake case, taking the overrides and returning the production type, each type parameter bounded by `Default`, and by `Zeroize` where the production type requires it; an omitted field takes `Default::default()` of its type parameter unless stated
    * `[✅]`   The generic builders: `G1GeneratorSuccessReturn`, `G2GeneratorSuccessReturn`, `AddG1Payload`, `AddG1SuccessReturn`, `AddG2Payload`, `AddG2SuccessReturn`, `MulG1Payload`, `MulG1SuccessReturn`, `MulG2Payload`, `MulG2SuccessReturn`, `MsmG1Term`, `MsmG1Payload` with `terms` defaulting to an empty `Vec`, `MsmG1SuccessReturn`, `MsmG2Term`, `MsmG2Payload` with `terms` defaulting to an empty `Vec`, `MsmG2SuccessReturn`, `PairingProductTerm`, `PairingProductIsOnePayload` with `terms` defaulting to an empty `Vec`, `DecodeG1SuccessReturn`, `DecodeG2SuccessReturn`, `DecodeScalarSuccessReturn`, `EncodeG1Payload`, `EncodeG2Payload`, `EncodeScalarPayload`, `EncodeG1SuccessReturn`, `EncodeG2SuccessReturn`, `EncodeScalarSuccessReturn` with `bytes` defaulting to `build_secret` holding `E::default()`, `SampleUniformScalarSuccessReturn` with `scalar` defaulting to `build_secret(SecretConstructorParamsOverrides::default())`, `AddScalarPayload`, `AddScalarSuccessReturn`, `MulScalarPayload`, `MulScalarSuccessReturn`, `NegScalarPayload`, `NegScalarSuccessReturn`, `NegG1Payload`, `NegG1SuccessReturn`, `NegG2Payload`, `NegG2SuccessReturn`, `IsIdentityG1Payload`, `IsIdentityG2Payload`, `PairingProductPayload` with `terms` defaulting to an empty `Vec`, `PairingProductSuccessReturn`, `EncodeGtPayload`, `EncodeGtSuccessReturn` with `bytes` defaulting to `build_secret` holding `E::default()`, `G1OutsideSubgroupEncodingSuccessReturn` with `bytes` defaulting to `None`, and `G2OutsideSubgroupEncodingSuccessReturn`
    * `[✅]`   The non-generic builders: `PairingProductIsOneSuccessReturnOverrides` with `build_pairing_product_is_one_success_return`, `is_one` defaulting to `true`; `SampleUniformScalarPayloadOverrides` with `build_sample_uniform_scalar_payload`, `uniform` defaulting to `build_secret` holding `vec![0u8; 64]`; `IsIdentityG1SuccessReturnOverrides` with `build_is_identity_g1_success_return` and `IsIdentityG2SuccessReturnOverrides` with `build_is_identity_g2_success_return`, `is_identity` defaulting to `false`; `ScalarFieldOrderSuccessReturnOverrides` with `build_scalar_field_order_success_return`, `bytes` defaulting to an empty `Vec`
    * `[✅]`   `MockEncodedG1`, `MockEncodedG2`, `MockEncodedScalar`, and `MockEncodedGt`, distinct `pub struct` mock encodings with private fields `[u8; 64]`, `[u8; 128]`, `[u8; 32]`, and `[u8; 384]`; each implements `Default` by a hand-written `impl` returning the all-zero array, since the standard library implements `Default` for arrays no longer than 32, and `AsRef<[u8]>`; `MockEncodedG1` and `MockEncodedG2` derive `Clone`, `PartialEq`, and `Eq`; `MockEncodedScalar` and `MockEncodedGt` implement `Zeroize`; they satisfy the mock adapter's associated encoding types without erasing their roles, are fixtures, and are not evidence of any concrete's byte format
    * `[✅]`   `MockIPairingAdapter<P: IPairingAdapter>`, a struct with `pub adapter: PhantomData<P>`, implementing `IPairingAdapter` for `P: IPairingAdapter` whose `Scalar`, `G1`, and `G2` implement `Default`, with `const DECLARATION: PairingDeclaration = P::DECLARATION;`, `const CONCRETE: PairingConcrete = P::CONCRETE;`, `type Scalar = P::Scalar;`, `type G1 = P::G1;`, `type G2 = P::G2;`, `type EncodedG1 = MockEncodedG1;`, `type EncodedG2 = MockEncodedG2;`, and `type EncodedScalar = MockEncodedScalar;`; implementing `IPairingArithmetic` under `P: IPairingArithmetic` with `P::Gt: Default`, `type Gt = P::Gt;` and `type EncodedGt = MockEncodedGt;`; and implementing `IPairingReference` under the same bounds as its `IPairingAdapter` implementation; every method returns `Ok` holding its success return's builder called with `Default::default()`, the decoders for any payload; a test needing other behavior implements the trait on its own local struct
    * `[✅]`   No builder for the fieldless params and payloads, for the error enums, or for the declaration's enums, each used by its production value; no corruptions type and no invalidator, since no struct this interface owns arrives as untrusted data and the decoders take the untrusted bytes directly

  * `[ ]`   `adapters/pairing/src/bn254_arkworks/mock.rs`
    * `[ ]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`; imports `super::interface::{Bn254ArkworksEncodedGt, Bn254ArkworksEncodedScalar, Bn254ArkworksG1, Bn254ArkworksG2, Bn254ArkworksGt, Bn254ArkworksPairing, Bn254ArkworksPairingConstructorParams, Bn254ArkworksScalar}`, `super::BN254_SEED`, `ark_bn254::{Bn254, Fq, Fq2, Fq12, Fr, G1Affine, G2Affine}`, `ark_ec::{AffineRepr, pairing::{Pairing, PairingOutput}}`, and `ark_ff::{BigInteger, Field, PrimeField}`; and, `#[cfg(test)]`, `ark_bn254::Fq6`, `ark_ec::CurveGroup`, `hex::decode`, `num_bigint::BigUint`, and `zeroize::ZeroizeOnDrop`
    * `[✅]`   The builder defaults for the concrete's owned types, which the family's generic builders and `MockIPairingAdapter` read through `Default`: `impl Default for Bn254ArkworksScalar` returning `value: Fr::from(1u64)`; `impl Default for Bn254ArkworksG1` returning `value: G1Affine::generator()`; `impl Default for Bn254ArkworksG2` returning `value: G2Affine::generator()`; `impl Default for Bn254ArkworksGt` returning `value: Bn254::pairing(G1Affine::generator(), G2Affine::generator())`, a non-identity target-group value
    * `[ ]`   The builders for the concrete's owned types, each overrides struct `#[derive(Default)]` with one `Option` field and each omitted value taking the type's `Default` above: `Bn254ArkworksScalarOverrides` with `pub value: Option<Fr>` and `build_bn254_arkworks_scalar(overrides: Bn254ArkworksScalarOverrides) -> Bn254ArkworksScalar`; `Bn254ArkworksG1Overrides` with `pub value: Option<G1Affine>` and `build_bn254_arkworks_g1`; `Bn254ArkworksG2Overrides` with `pub value: Option<G2Affine>` and `build_bn254_arkworks_g2`; `Bn254ArkworksGtOverrides` with `pub value: Option<PairingOutput<Bn254>>` and `build_bn254_arkworks_gt`; no corruptions type and no invalidator, since none of these arrives as untrusted data
    * `[ ]`   `build_bn254_arkworks_pairing() -> Bn254ArkworksPairing`, the real instance from `Bn254ArkworksPairing::try_new(Bn254ArkworksPairingConstructorParams)` through `let Ok(pairing) = …;`; the constructor params are fieldless, so the builder takes no overrides
    * `[ ]`   The builders for the concrete's encoded scalar and encoded target-group types, each overrides struct `#[derive(Default)]` with one `Option` field: `Bn254ArkworksEncodedScalarOverrides` with `pub bytes: Option<[u8; 32]>` and `build_bn254_arkworks_encoded_scalar(overrides: Bn254ArkworksEncodedScalarOverrides) -> Bn254ArkworksEncodedScalar`, the bytes defaulting to `[1u8; 32]`; `Bn254ArkworksEncodedGtOverrides` with `pub bytes: Option<[u8; 384]>` and `build_bn254_arkworks_encoded_gt`, the bytes defaulting to `[1u8; 384]`; the defaults are nonzero so a zeroized value differs from a built one; no corruptions type and no invalidator, since neither arrives as untrusted data
    * `[ ]`   The test fixtures, each `#[cfg(test)]`, since they use the crate's dev-dependencies; constants, each a `const … : &str` of hex: `BASE_FIELD_MODULUS_HEX` `30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47`; `GROUP_ORDER_HEX` `30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001`; `GROUP_ORDER_MINUS_ONE_HEX` `30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000000`; `GROUP_ORDER_MINUS_TWO_HEX` `30644e72e131a029b85045b68181585d2833e84879b9709143e1f593efffffff`; `G1_GENERATOR_HEX`, 31 zero bytes and `01` then 31 zero bytes and `02`; `G2_GENERATOR_HEX`, `198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c2` `1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed` `090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b` `12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa`; `G2_GENERATOR_OFF_CURVE_HEX`, the same with the last byte `aa` replaced by `ab`; `G2_X_C1_AT_MODULUS_HEX`, the 32 bytes of `BASE_FIELD_MODULUS_HEX` followed by the last three 32-byte quarters of `G2_GENERATOR_HEX`; `NEG_G1_GENERATOR_HEX`, 31 zero bytes and `01` then `30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd45`; `G1_X_AT_MODULUS_HEX`, the 32 bytes of `BASE_FIELD_MODULUS_HEX` then 31 zero bytes and `02`; `G1_OFF_CURVE_HEX`, 31 zero bytes and `01` then 31 zero bytes and `03`, the point `(1, 3)`, which satisfies `y^2 = x^3 + 3` on neither side, 9 against 4; `UNIFORM_GROUP_ORDER_HEX`, 32 zero bytes then the 32 bytes of `GROUP_ORDER_HEX`, the group order as a 64-byte big-endian integer
    * `[ ]`   The test-fixture helpers, each `#[cfg(test)]`: `vector_bytes(hex: &str) -> Vec<u8>`, `decode(hex)` unpacked by `let Ok(bytes) = … else { panic!("the vector decodes") };`; `zero_bytes(length: usize) -> Vec<u8>`, `vec![0u8; length]`; `gt_identity_encoding() -> Vec<u8>`, `vec![0u8; 384]` with index `31` set to `1`; `gt_with_sequential_coefficients() -> PairingOutput<Bn254>`, `PairingOutput(Fq12::new(Fq6::new(Fq2::new(Fq::from(1u64), Fq::from(2u64)), Fq2::new(Fq::from(3u64), Fq::from(4u64)), Fq2::new(Fq::from(5u64), Fq::from(6u64))), Fq6::new(Fq2::new(Fq::from(7u64), Fq::from(8u64)), Fq2::new(Fq::from(9u64), Fq::from(10u64)), Fq2::new(Fq::from(11u64), Fq::from(12u64)))))`, whose twelve coefficients in tower order are the integers one through twelve; `sequential_gt_encoding() -> Vec<u8>`, `vec![0u8; 384]` with, for each coefficient position `i` from zero through eleven in tower order, byte `32 * i + 31` set to `i + 1`; `base_field_modulus() -> BigUint`, `BigUint::from_bytes_be(&vector_bytes(BASE_FIELD_MODULUS_HEX))`; `scalar_value(hex: &str) -> Fr`, `Fr::from_be_bytes_mod_order(&vector_bytes(hex))`; `eip_196_generator() -> G1Affine`, `G1Affine::new_unchecked(Fq::from(1u64), Fq::from(2u64))`, the generator EIP-196 publishes; `eip_196_negated_generator() -> G1Affine`, `G1Affine::new_unchecked` over the two 32-byte halves of `vector_bytes(NEG_G1_GENERATOR_HEX)`, each read by `Fq::from_be_bytes_mod_order`; `eip_197_generator() -> G2Affine`, `G2Affine::new_unchecked(Fq2::new(x_c0, x_c1), Fq2::new(y_c0, y_c1))` over the four 32-byte quarters of `vector_bytes(G2_GENERATOR_HEX)`, read in order as `x_c1`, `x_c0`, `y_c1`, `y_c0` by `Fq::from_be_bytes_mod_order`; `g2_point_from_encoding(bytes: &[u8]) -> G2Affine`, the same reading over any 128 bytes; `g2_outside_subgroup_bytes() -> Vec<u8>`, the least on-curve point outside the subgroup, `(1u64..).find_map(|c0| G2Affine::get_point_from_x_unchecked(Fq2::new(Fq::from(c0), Fq::from(0u64)), false).filter(|point| !point.is_in_correct_subgroup_assuming_on_curve()))` unpacked by `let Some(point) = … else { panic!("an on-curve point outside the subgroup exists") };`, its `xy()` unpacked by `let Some((x, y)) = … else { panic!("the point has coordinates") };`, encoded by appending `into_bigint().to_bytes_be()` of `x.c1`, `x.c0`, `y.c1`, `y.c0`; `definition_exponent() -> Vec<u64>`, `let p = BigUint::from(Fq::MODULUS); let r = BigUint::from(Fr::MODULUS); ((p.pow(12) - BigUint::from(1u32)) / r).to_u64_digits()`; `definition_value(g1: G1Affine, g2: G2Affine) -> Fq12`, `Bn254::multi_miller_loop([g1], [g2]).0.pow(definition_exponent())`, the identifier's definition of the pairing value; `eip_196_doubled_generator() -> G1Affine`, `(eip_196_generator() + eip_196_generator()).into_affine()`; `definition_value_power(g1: G1Affine, g2: G2Affine, exponent: u64) -> Fq12`, `definition_value(g1, g2).pow([exponent])`; `library_multiple() -> Fr`, `let seed = Fr::from(BN254_SEED); (seed * seed * Fr::from(6u64) + seed * Fr::from(3u64) + Fr::ONE) * seed * Fr::from(2u64)`; `requires_zeroize_on_drop<T: ZeroizeOnDrop>() {}`
    * `[ ]`   No mock function: the concrete is built as a real instance and owns no free function

  * `[✅]`   `adapters/pairing/src/factory/mod.rs`
    * `[✅]`   The module wiring: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, and `pub mod provides;`; the factory function and its unit-test module are `pairing/factory`'s

  * `[✅]`   `adapters/pairing/src/factory/provides.rs`
    * `[✅]`   `pub use super::interface::*;` and `#[cfg(any(test, feature = "mocks"))] pub use super::mock::*;`; the factory function's re-export is `pairing/factory`'s

  * `[ ]`   `adapters/pairing/src/bn254_arkworks/test.rs`
    * `[ ]`   Module-level `#![allow(clippy::expect_used)]`; imports from `super::provides` the concrete, its owned types, and the mock's builders, constants, and helpers each block uses; from `crate::factory::provides` the params, payloads, errors, trait names, and builders each block uses; and the `ark_bn254`, `ark_ec`, `ark_ff`, `num_bigint::BigUint`, and `zeroize::Zeroize` names each block uses
    * `[ ]`   Compile-time assertion over `Bn254ArkworksPairing::DECLARATION`
      * `[ ]`   Contract: the inherent constant is `PairingDeclaration { curve: PairingCurve::Bn254, verifier_group_arithmetic: VerifierGroupArithmetic::FirstGroupOnly, precompile_encoding: PrecompileEncoding::Eip196Eip197, target_group_encoding: TargetGroupEncodingIdentifier::Bn254V1, adapter_version: 1, interface_version: PAIRING_INTERFACE_VERSION }`, readable before any instance exists
      * `[ ]`   Arrange: the type alone
      * `[ ]`   Act: a module-level `const _: () = assert!(…);` reading `Bn254ArkworksPairing::DECLARATION`, each enum field tested with `matches!` and each version with `==`
      * `[ ]`   Assert: the module compiles only if every field holds
    * `[ ]`   Compile-time assertion over `<Bn254ArkworksPairing as IPairingAdapter>::DECLARATION`
      * `[ ]`   Contract: the trait constant carries the same fields as the inherent constant
      * `[ ]`   Arrange: the type alone
      * `[ ]`   Act: a module-level `const _: () = assert!(…);` reading the trait constant, each enum field tested with `matches!` and each version with `==`
      * `[ ]`   Assert: the module compiles only if every field holds
    * `[ ]`   Compile-time assertion over `<Bn254ArkworksPairing as IPairingAdapter>::CONCRETE`
      * `[ ]`   Contract: the trait constant is `PairingConcrete::Bn254Arkworks`
      * `[ ]`   Arrange: the type alone
      * `[ ]`   Act: a module-level `const _: () = assert!(…);` reading the trait constant under `matches!`
      * `[ ]`   Assert: the module compiles only if the constant is `PairingConcrete::Bn254Arkworks`
    * `[ ]`   `bn254_arkworks_scalar_is_zeroize_on_drop`
      * `[ ]`   Contract: `Bn254ArkworksScalar` implements `ZeroizeOnDrop`, the marker the sampling bound requires
      * `[ ]`   Collaborators: none
      * `[ ]`   Arrange: the type alone
      * `[ ]`   Act: `requires_zeroize_on_drop::<Bn254ArkworksScalar>()`
      * `[ ]`   Assert: the block compiles only if the type implements `ZeroizeOnDrop`
    * `[ ]`   `bn254_arkworks_scalar_zeroize_clears_its_value`
      * `[ ]`   Contract: `Zeroize::zeroize` on a living scalar → its `value` is zero
      * `[ ]`   Collaborators: arkworks' `Fr` zeroization, run for real as the vendor; fixture `build_bn254_arkworks_scalar`
      * `[ ]`   Arrange: `build_bn254_arkworks_scalar` with the value override `Fr::from(5u64)`, so a scalar left unchanged differs from the expected zero
      * `[ ]`   Act: `scalar.zeroize()`
      * `[ ]`   Assert: `scalar.value` equals `Fr::from(0u64)`
    * `[ ]`   `reduced_pairing_correction_inverts_the_library_multiple`
      * `[ ]`   Contract: any params → `Ok(Bn254ArkworksPairing)` whose `reduced_pairing_correction` is the inverse in the scalar field of `2x(6x^2 + 3x + 1)` for the curve seed `x`
      * `[ ]`   Collaborators: arkworks' `Fr` arithmetic, run for real as the vendor; the expectation is `library_multiple()`, computed in the fixtures from the seed by field multiplication and independent of the Fermat inversion `try_new` performs
      * `[ ]`   Arrange: `Bn254ArkworksPairingConstructorParams` by its production value
      * `[ ]`   Act: `Bn254ArkworksPairing::try_new(Bn254ArkworksPairingConstructorParams)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(pairing) = …;`; `pairing.reduced_pairing_correction * library_multiple()` equals `Fr::from(1u64)`
    * `[ ]`   `uniform_bytes_length_is_twice_the_group_order_width`
      * `[ ]`   Contract: `UNIFORM_BYTES_LENGTH` is twice the byte width of the group order
      * `[ ]`   Collaborators: none
      * `[ ]`   Arrange: the type alone
      * `[ ]`   Act: reading `<Bn254ArkworksScalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH`
      * `[ ]`   Assert: the constant equals the literal 64 written in the assertion
    * `[ ]`   `g1_generator_returns_the_eip_196_generator`
      * `[ ]`   Contract: any call → `Ok(G1GeneratorSuccessReturn { point })` holding the first group's generator
      * `[ ]`   Collaborators: arkworks, run for real as the vendor; fixture `build_bn254_arkworks_pairing`; `G1GeneratorParams` and `G1GeneratorPayload` by their production values
      * `[ ]`   Arrange: `build_bn254_arkworks_pairing()`
      * `[ ]`   Act: `g1_generator(G1GeneratorParams, G1GeneratorPayload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.point.value` equals `eip_196_generator()`
    * `[ ]`   `g2_generator_returns_the_eip_197_generator`
      * `[ ]`   Contract: any call → `Ok(G2GeneratorSuccessReturn { point })` holding the second group's generator
      * `[ ]`   Collaborators: arkworks, run for real as the vendor; fixture `build_bn254_arkworks_pairing`; `G2GeneratorParams` and `G2GeneratorPayload` by their production values
      * `[ ]`   Arrange: `build_bn254_arkworks_pairing()`
      * `[ ]`   Act: `g2_generator(G2GeneratorParams, G2GeneratorPayload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.point.value` equals `eip_197_generator()`
    * `[ ]`   `add_g1_of_a_point_and_its_negation_is_the_identity`
      * `[ ]`   Contract: `payload.left` and `payload.right` → `Ok(AddG1SuccessReturn { sum })` holding their sum
      * `[ ]`   Collaborators: arkworks' affine addition, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_add_g1_payload`, and `build_bn254_arkworks_g1`
      * `[ ]`   Arrange: `build_add_g1_payload` with the left override `build_bn254_arkworks_g1` at its default generator and the right override `build_bn254_arkworks_g1` with the value `eip_196_negated_generator()`, so a sum that returns either input differs from the identity
      * `[ ]`   Act: `add_g1(AddG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G1Affine::identity()`
    * `[ ]`   `add_g1_of_the_identity_and_a_point_is_the_point`
      * `[ ]`   Contract: `payload.left` and `payload.right` → `Ok(AddG1SuccessReturn { sum })` holding their sum
      * `[ ]`   Collaborators: arkworks' affine addition, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_add_g1_payload`, and `build_bn254_arkworks_g1`
      * `[ ]`   Arrange: `build_add_g1_payload` with the left override `build_bn254_arkworks_g1` with the value `G1Affine::identity()` and the right override `build_bn254_arkworks_g1` at its default generator, so a sum that returns the left input differs from the generator
      * `[ ]`   Act: `add_g1(AddG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `eip_196_generator()`
    * `[ ]`   `add_g2_of_a_point_and_its_negation_is_the_identity`
      * `[ ]`   Contract: `payload.left` and `payload.right` → `Ok(AddG2SuccessReturn { sum })` holding their sum
      * `[ ]`   Collaborators: arkworks' affine addition, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_add_g2_payload`, and `build_bn254_arkworks_g2`
      * `[ ]`   Arrange: `build_add_g2_payload` with the left override `build_bn254_arkworks_g2` at its default generator and the right override `build_bn254_arkworks_g2` with the value `-eip_197_generator()`, so a sum that returns either input differs from the identity
      * `[ ]`   Act: `add_g2(AddG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G2Affine::identity()`
    * `[ ]`   `add_g2_of_the_identity_and_a_point_is_the_point`
      * `[ ]`   Contract: `payload.left` and `payload.right` → `Ok(AddG2SuccessReturn { sum })` holding their sum
      * `[ ]`   Collaborators: arkworks' affine addition, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_add_g2_payload`, and `build_bn254_arkworks_g2`
      * `[ ]`   Arrange: `build_add_g2_payload` with the left override `build_bn254_arkworks_g2` with the value `G2Affine::identity()` and the right override `build_bn254_arkworks_g2` at its default generator, so a sum that returns the left input differs from the generator
      * `[ ]`   Act: `add_g2(AddG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `eip_197_generator()`
    * `[ ]`   `mul_g1_by_one_is_the_point`
      * `[ ]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG1SuccessReturn { product })` holding the point multiplied by the scalar
      * `[ ]`   Collaborators: arkworks' affine scalar multiplication, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_mul_g1_payload`, `build_bn254_arkworks_g1`, and `build_bn254_arkworks_scalar`
      * `[ ]`   Arrange: `build_mul_g1_payload` with the point override `build_bn254_arkworks_g1` at its default generator and the scalar override `build_bn254_arkworks_scalar` with the value `Fr::from(1u64)`
      * `[ ]`   Act: `mul_g1(MulG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `eip_196_generator()`
    * `[ ]`   `mul_g1_by_zero_is_the_identity`
      * `[ ]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG1SuccessReturn { product })` holding the point multiplied by the scalar
      * `[ ]`   Collaborators: arkworks' affine scalar multiplication, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_mul_g1_payload`, `build_bn254_arkworks_g1`, and `build_bn254_arkworks_scalar`
      * `[ ]`   Arrange: `build_mul_g1_payload` with the point override `build_bn254_arkworks_g1` at its default generator and the scalar override `build_bn254_arkworks_scalar` with the value `Fr::from(0u64)`, so a product that returns the point differs from the identity
      * `[ ]`   Act: `mul_g1(MulG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `G1Affine::identity()`
    * `[ ]`   `mul_g1_by_the_group_order_minus_one_is_the_negated_generator`
      * `[ ]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG1SuccessReturn { product })` holding the point multiplied by the scalar
      * `[ ]`   Collaborators: arkworks' affine scalar multiplication, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_mul_g1_payload`, `build_bn254_arkworks_g1`, and `build_bn254_arkworks_scalar`
      * `[ ]`   Arrange: `build_mul_g1_payload` with the point override `build_bn254_arkworks_g1` at its default generator and the scalar override `build_bn254_arkworks_scalar` with the value `scalar_value(GROUP_ORDER_MINUS_ONE_HEX)`, so a product that ignores the scalar differs from the negated generator
      * `[ ]`   Act: `mul_g1(MulG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `eip_196_negated_generator()`
    * `[ ]`   `mul_g2_by_one_is_the_point`
      * `[ ]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG2SuccessReturn { product })` holding the point multiplied by the scalar
      * `[ ]`   Collaborators: arkworks' affine scalar multiplication, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_mul_g2_payload`, `build_bn254_arkworks_g2`, and `build_bn254_arkworks_scalar`
      * `[ ]`   Arrange: `build_mul_g2_payload` with the point override `build_bn254_arkworks_g2` at its default generator and the scalar override `build_bn254_arkworks_scalar` with the value `Fr::from(1u64)`
      * `[ ]`   Act: `mul_g2(MulG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `eip_197_generator()`
    * `[ ]`   `mul_g2_by_zero_is_the_identity`
      * `[ ]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG2SuccessReturn { product })` holding the point multiplied by the scalar
      * `[ ]`   Collaborators: arkworks' affine scalar multiplication, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_mul_g2_payload`, `build_bn254_arkworks_g2`, and `build_bn254_arkworks_scalar`
      * `[ ]`   Arrange: `build_mul_g2_payload` with the point override `build_bn254_arkworks_g2` at its default generator and the scalar override `build_bn254_arkworks_scalar` with the value `Fr::from(0u64)`, so a product that returns the point differs from the identity
      * `[ ]`   Act: `mul_g2(MulG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `G2Affine::identity()`
    * `[ ]`   `mul_g2_by_the_group_order_minus_one_is_the_negated_point`
      * `[ ]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG2SuccessReturn { product })` holding the point multiplied by the scalar
      * `[ ]`   Collaborators: arkworks' affine scalar multiplication, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_mul_g2_payload`, `build_bn254_arkworks_g2`, and `build_bn254_arkworks_scalar`
      * `[ ]`   Arrange: `build_mul_g2_payload` with the point override `build_bn254_arkworks_g2` at its default generator and the scalar override `build_bn254_arkworks_scalar` with the value `scalar_value(GROUP_ORDER_MINUS_ONE_HEX)`, so a product that ignores the scalar differs from the negated generator
      * `[ ]`   Act: `mul_g2(MulG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `-eip_197_generator()`
    * `[ ]`   `msm_g1_of_no_terms_is_the_identity`
      * `[ ]`   Contract: an empty `payload.terms` → `Ok(MsmG1SuccessReturn { sum })` holding the identity
      * `[ ]`   Collaborators: arkworks' multi-scalar multiplication, run for real as the vendor; fixtures `build_bn254_arkworks_pairing` and `build_msm_g1_payload`
      * `[ ]`   Arrange: `build_msm_g1_payload` at its default of no terms
      * `[ ]`   Act: `msm_g1(MsmG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G1Affine::identity()`
    * `[ ]`   `msm_g1_pairs_each_base_with_its_own_scalar`
      * `[ ]`   Contract: `payload.terms` → `Ok(MsmG1SuccessReturn { sum })` holding the sum of each base multiplied by its own scalar
      * `[ ]`   Collaborators: arkworks' multi-scalar multiplication, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_msm_g1_payload`, `build_msm_g1_term`, `build_bn254_arkworks_g1`, and `build_bn254_arkworks_scalar`
      * `[ ]`   Arrange: `build_msm_g1_payload` with the terms override holding two `build_msm_g1_term` values: the generator with the scalar `Fr::from(1u64)`, and the base `eip_196_negated_generator()` with the scalar `Fr::from(0u64)`, so bases and scalars exchanged between terms yield the negated generator
      * `[ ]`   Act: `msm_g1(MsmG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `eip_196_generator()`
    * `[ ]`   `msm_g1_sums_its_terms`
      * `[ ]`   Contract: `payload.terms` → `Ok(MsmG1SuccessReturn { sum })` holding the sum of each base multiplied by its own scalar
      * `[ ]`   Collaborators: arkworks' multi-scalar multiplication, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_msm_g1_payload`, `build_msm_g1_term`, `build_bn254_arkworks_g1`, and `build_bn254_arkworks_scalar`
      * `[ ]`   Arrange: `build_msm_g1_payload` with the terms override holding two `build_msm_g1_term` values: the generator with the scalar `Fr::from(1u64)`, and the base `eip_196_negated_generator()` with the scalar `Fr::from(1u64)`, so a sum over the first term alone differs from the identity
      * `[ ]`   Act: `msm_g1(MsmG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G1Affine::identity()`
    * `[ ]`   `msm_g2_of_no_terms_is_the_identity`
      * `[ ]`   Contract: an empty `payload.terms` → `Ok(MsmG2SuccessReturn { sum })` holding the identity
      * `[ ]`   Collaborators: arkworks' multi-scalar multiplication, run for real as the vendor; fixtures `build_bn254_arkworks_pairing` and `build_msm_g2_payload`
      * `[ ]`   Arrange: `build_msm_g2_payload` at its default of no terms
      * `[ ]`   Act: `msm_g2(MsmG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G2Affine::identity()`
    * `[ ]`   `msm_g2_pairs_each_base_with_its_own_scalar`
      * `[ ]`   Contract: `payload.terms` → `Ok(MsmG2SuccessReturn { sum })` holding the sum of each base multiplied by its own scalar
      * `[ ]`   Collaborators: arkworks' multi-scalar multiplication, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_msm_g2_payload`, `build_msm_g2_term`, `build_bn254_arkworks_g2`, and `build_bn254_arkworks_scalar`
      * `[ ]`   Arrange: `build_msm_g2_payload` with the terms override holding two `build_msm_g2_term` values: the generator with the scalar `Fr::from(1u64)`, and the base `-eip_197_generator()` with the scalar `Fr::from(0u64)`, so bases and scalars exchanged between terms yield the negated generator
      * `[ ]`   Act: `msm_g2(MsmG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `eip_197_generator()`
    * `[ ]`   `msm_g2_sums_its_terms`
      * `[ ]`   Contract: `payload.terms` → `Ok(MsmG2SuccessReturn { sum })` holding the sum of each base multiplied by its own scalar
      * `[ ]`   Collaborators: arkworks' multi-scalar multiplication, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_msm_g2_payload`, `build_msm_g2_term`, `build_bn254_arkworks_g2`, and `build_bn254_arkworks_scalar`
      * `[ ]`   Arrange: `build_msm_g2_payload` with the terms override holding two `build_msm_g2_term` values: the generator with the scalar `Fr::from(1u64)`, and the base `-eip_197_generator()` with the scalar `Fr::from(1u64)`, so a sum over the first term alone differs from the identity
      * `[ ]`   Act: `msm_g2(MsmG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G2Affine::identity()`
    * `[ ]`   `pairing_product_is_one_of_no_terms_is_true`
      * `[ ]`   Contract: an empty `payload.terms` → `Ok(PairingProductIsOneSuccessReturn { is_one })` with `is_one` true
      * `[ ]`   Collaborators: arkworks' multi-pairing, run for real as the vendor; fixtures `build_bn254_arkworks_pairing` and `build_pairing_product_is_one_payload`
      * `[ ]`   Arrange: `build_pairing_product_is_one_payload` at its default of no terms
      * `[ ]`   Act: `pairing_product_is_one(PairingProductIsOneParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_one` is true
    * `[ ]`   `pairing_product_is_one_of_a_pairing_and_its_first_group_negation_is_true`
      * `[ ]`   Contract: `payload.terms` whose pairings multiply to the target group's identity → `Ok(PairingProductIsOneSuccessReturn { is_one })` with `is_one` true
      * `[ ]`   Collaborators: arkworks' multi-pairing, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_pairing_product_is_one_payload`, `build_pairing_product_term`, and `build_bn254_arkworks_g1`
      * `[ ]`   Arrange: `build_pairing_product_is_one_payload` with the terms override holding two `build_pairing_product_term` values: the generators, and the first-group override `build_bn254_arkworks_g1` with the value `eip_196_negated_generator()` over the second-group generator
      * `[ ]`   Act: `pairing_product_is_one(PairingProductIsOneParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_one` is true
    * `[ ]`   `pairing_product_is_one_of_the_generators_is_false`
      * `[ ]`   Contract: `payload.terms` whose pairings multiply to a value other than the target group's identity → `Ok(PairingProductIsOneSuccessReturn { is_one })` with `is_one` false
      * `[ ]`   Collaborators: arkworks' multi-pairing, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_pairing_product_is_one_payload`, and `build_pairing_product_term`
      * `[ ]`   Arrange: `build_pairing_product_is_one_payload` with the terms override holding one `build_pairing_product_term` at its default generators
      * `[ ]`   Act: `pairing_product_is_one(PairingProductIsOneParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_one` is false
    * `[ ]`   `decode_g1_rejects_a_wrong_length`
      * `[ ]`   Contract: `payload.len() != 64` → `Err(DecodeG1ErrorReturn::WrongLength { expected: 64, actual: payload.len() })`
      * `[ ]`   Collaborators: none called; fixture `build_bn254_arkworks_pairing`; the untrusted bytes from `zero_bytes`
      * `[ ]`   Arrange: the payload `zero_bytes(63)`, which differs from the required length
      * `[ ]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(DecodeG1ErrorReturn::WrongLength { expected: 64, actual: 63 })`, the whole expected error
    * `[ ]`   `decode_g1_rejects_a_non_canonical_coordinate`
      * `[ ]`   Contract: a 64-byte payload whose first half is at least the base field modulus → `Err(DecodeG1ErrorReturn::NonCanonicalCoordinate)`
      * `[ ]`   Collaborators: arkworks' field reduction, run for real as the vendor; fixture `build_bn254_arkworks_pairing`; the untrusted bytes from `vector_bytes(G1_X_AT_MODULUS_HEX)`
      * `[ ]`   Arrange: the payload `vector_bytes(G1_X_AT_MODULUS_HEX)`, whose reduced coordinates would otherwise fail the curve check, so a decoder that checks the curve first returns a different error
      * `[ ]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(DecodeG1ErrorReturn::NonCanonicalCoordinate)`
    * `[ ]`   `decode_g1_decodes_the_identity_from_zero_bytes`
      * `[ ]`   Contract: a 64-byte payload with both coordinates zero → `Ok(DecodeG1SuccessReturn { point })` holding the first group's identity
      * `[ ]`   Collaborators: arkworks, run for real as the vendor; fixture `build_bn254_arkworks_pairing`; the untrusted bytes from `zero_bytes`
      * `[ ]`   Arrange: the payload `zero_bytes(64)`
      * `[ ]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[ ]`   Assert: the success arm is extracted with `expect`; `success.point.value` equals `G1Affine::identity()`
    * `[ ]`   `decode_g1_rejects_a_point_off_the_curve`
      * `[ ]`   Contract: a canonical 64-byte payload that satisfies no curve equation → `Err(DecodeG1ErrorReturn::NotOnCurve)`
      * `[ ]`   Collaborators: arkworks' curve check, run for real as the vendor; fixture `build_bn254_arkworks_pairing`; the untrusted bytes from `vector_bytes(G1_OFF_CURVE_HEX)`
      * `[ ]`   Arrange: the payload `vector_bytes(G1_OFF_CURVE_HEX)`
      * `[ ]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(DecodeG1ErrorReturn::NotOnCurve)`
    * `[ ]`   `decode_g1_decodes_the_eip_196_generator`
      * `[ ]`   Contract: a canonical 64-byte payload on the curve → `Ok(DecodeG1SuccessReturn { point })` holding that point
      * `[ ]`   Collaborators: arkworks, run for real as the vendor; fixture `build_bn254_arkworks_pairing`; the untrusted bytes from `vector_bytes(G1_GENERATOR_HEX)`
      * `[ ]`   Arrange: the payload `vector_bytes(G1_GENERATOR_HEX)`
      * `[ ]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[ ]`   Assert: the success arm is extracted with `expect`; `success.point.value` equals `eip_196_generator()`
    * `[ ]`   `decode_g2_rejects_a_wrong_length`
      * `[ ]`   Contract: `payload.len() != 128` → `Err(DecodeG2ErrorReturn::WrongLength { expected: 128, actual: payload.len() })`
      * `[ ]`   Collaborators: none called; fixture `build_bn254_arkworks_pairing`; the untrusted bytes from `zero_bytes`
      * `[ ]`   Arrange: the payload `zero_bytes(127)`, which differs from the required length
      * `[ ]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(DecodeG2ErrorReturn::WrongLength { expected: 128, actual: 127 })`, the whole expected error
    * `[ ]`   `decode_g2_rejects_a_non_canonical_coordinate`
      * `[ ]`   Contract: a 128-byte payload whose first coordinate is at least the base field modulus → `Err(DecodeG2ErrorReturn::NonCanonicalCoordinate)`
      * `[ ]`   Collaborators: arkworks' field reduction, run for real as the vendor; fixture `build_bn254_arkworks_pairing`; the untrusted bytes from `vector_bytes(G2_X_C1_AT_MODULUS_HEX)`
      * `[ ]`   Arrange: the payload `vector_bytes(G2_X_C1_AT_MODULUS_HEX)`, whose reduced coordinates would otherwise fail the curve check, so a decoder that checks the curve first returns a different error
      * `[ ]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(DecodeG2ErrorReturn::NonCanonicalCoordinate)`
    * `[ ]`   `decode_g2_decodes_the_identity_from_zero_bytes`
      * `[ ]`   Contract: a 128-byte payload with every coordinate zero → `Ok(DecodeG2SuccessReturn { point })` holding the second group's identity
      * `[ ]`   Collaborators: arkworks, run for real as the vendor; fixture `build_bn254_arkworks_pairing`; the untrusted bytes from `zero_bytes`
      * `[ ]`   Arrange: the payload `zero_bytes(128)`
      * `[ ]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[ ]`   Assert: the success arm is extracted with `expect`; `success.point.value` equals `G2Affine::identity()`
    * `[ ]`   `decode_g2_rejects_a_point_off_the_curve`
      * `[ ]`   Contract: a canonical 128-byte payload that satisfies no curve equation → `Err(DecodeG2ErrorReturn::NotOnCurve)`
      * `[ ]`   Collaborators: arkworks' curve check, run for real as the vendor; fixture `build_bn254_arkworks_pairing`; the untrusted bytes from `vector_bytes(G2_GENERATOR_OFF_CURVE_HEX)`
      * `[ ]`   Arrange: the payload `vector_bytes(G2_GENERATOR_OFF_CURVE_HEX)`
      * `[ ]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(DecodeG2ErrorReturn::NotOnCurve)`
    * `[ ]`   `decode_g2_rejects_a_point_outside_the_subgroup`
      * `[ ]`   Contract: a canonical 128-byte payload on the curve and outside the prime-order subgroup → `Err(DecodeG2ErrorReturn::NotInSubgroup)`
      * `[ ]`   Collaborators: arkworks' subgroup check, run for real as the vendor; fixture `build_bn254_arkworks_pairing`; the untrusted bytes from `g2_outside_subgroup_bytes()`
      * `[ ]`   Arrange: the payload `g2_outside_subgroup_bytes()`
      * `[ ]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(DecodeG2ErrorReturn::NotInSubgroup)`
    * `[ ]`   `decode_g2_decodes_the_eip_197_generator`
      * `[ ]`   Contract: a canonical 128-byte payload on the curve and in the subgroup → `Ok(DecodeG2SuccessReturn { point })` holding that point
      * `[ ]`   Collaborators: arkworks, run for real as the vendor; fixture `build_bn254_arkworks_pairing`; the untrusted bytes from `vector_bytes(G2_GENERATOR_HEX)`
      * `[ ]`   Arrange: the payload `vector_bytes(G2_GENERATOR_HEX)`
      * `[ ]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[ ]`   Assert: the success arm is extracted with `expect`; `success.point.value` equals `eip_197_generator()`
    * `[ ]`   `decode_scalar_rejects_a_wrong_length`
      * `[ ]`   Contract: `payload.len() != 32` → `Err(DecodeScalarErrorReturn::WrongLength { expected: 32, actual: payload.len() })`
      * `[ ]`   Collaborators: none called; fixture `build_bn254_arkworks_pairing`; the untrusted bytes from `zero_bytes`
      * `[ ]`   Arrange: the payload `zero_bytes(31)`, which differs from the required length
      * `[ ]`   Act: `decode_scalar(DecodeScalarParams, &payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(DecodeScalarErrorReturn::WrongLength { expected: 32, actual: 31 })`, the whole expected error
    * `[ ]`   `decode_scalar_rejects_the_group_order`
      * `[ ]`   Contract: a 32-byte payload at least the group order → `Err(DecodeScalarErrorReturn::NonCanonical)`
      * `[ ]`   Collaborators: arkworks' field reduction, run for real as the vendor; fixture `build_bn254_arkworks_pairing`; the untrusted bytes from `vector_bytes(GROUP_ORDER_HEX)`
      * `[ ]`   Arrange: the payload `vector_bytes(GROUP_ORDER_HEX)`, whose reduction is zero and so differs from the bytes
      * `[ ]`   Act: `decode_scalar(DecodeScalarParams, &payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(DecodeScalarErrorReturn::NonCanonical)`
    * `[ ]`   `decode_scalar_decodes_the_largest_canonical_scalar`
      * `[ ]`   Contract: a canonical 32-byte payload → `Ok(DecodeScalarSuccessReturn { scalar })` holding that scalar
      * `[ ]`   Collaborators: arkworks' field reduction, run for real as the vendor; fixture `build_bn254_arkworks_pairing`; the untrusted bytes from `vector_bytes(GROUP_ORDER_MINUS_ONE_HEX)`
      * `[ ]`   Arrange: the payload `vector_bytes(GROUP_ORDER_MINUS_ONE_HEX)`
      * `[ ]`   Act: `decode_scalar(DecodeScalarParams, &payload)`
      * `[ ]`   Assert: the success arm is extracted with `expect`; `success.scalar.value` equals `-Fr::from(1u64)`
    * `[ ]`   `encode_g1_writes_the_eip_196_generator`
      * `[ ]`   Contract: a first-group point → `Ok(EncodeG1SuccessReturn { bytes })` holding `x` then `y`, each 32 bytes big-endian, in this concrete's `Bn254ArkworksEncodedG1`
      * `[ ]`   Collaborators: arkworks' coordinate encoding, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_encode_g1_payload`, and `build_bn254_arkworks_g1`
      * `[ ]`   Arrange: `build_encode_g1_payload` with the point override `build_bn254_arkworks_g1` at its default generator
      * `[ ]`   Act: `encode_g1(EncodeG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.bytes` binds to a `Bn254ArkworksEncodedG1` typed binding; its bytes equal `vector_bytes(G1_GENERATOR_HEX)` as a byte slice
    * `[ ]`   `encode_g1_writes_the_identity_as_zero_bytes`
      * `[ ]`   Contract: the first group's identity → `Ok(EncodeG1SuccessReturn { bytes })` holding 64 zero bytes
      * `[ ]`   Collaborators: arkworks, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_encode_g1_payload`, and `build_bn254_arkworks_g1`
      * `[ ]`   Arrange: `build_encode_g1_payload` with the point override `build_bn254_arkworks_g1` with the value `G1Affine::identity()`
      * `[ ]`   Act: `encode_g1(EncodeG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; the bytes of `success.bytes` equal `zero_bytes(64)` as a byte slice
    * `[ ]`   `encode_g2_writes_the_eip_197_generator`
      * `[ ]`   Contract: a second-group point → `Ok(EncodeG2SuccessReturn { bytes })` holding `x.c1`, `x.c0`, `y.c1`, `y.c0`, each 32 bytes big-endian, in this concrete's `Bn254ArkworksEncodedG2`
      * `[ ]`   Collaborators: arkworks' coordinate encoding, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_encode_g2_payload`, and `build_bn254_arkworks_g2`
      * `[ ]`   Arrange: `build_encode_g2_payload` with the point override `build_bn254_arkworks_g2` at its default generator
      * `[ ]`   Act: `encode_g2(EncodeG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.bytes` binds to a `Bn254ArkworksEncodedG2` typed binding; its bytes equal `vector_bytes(G2_GENERATOR_HEX)` as a byte slice
    * `[ ]`   `encode_g2_writes_the_identity_as_zero_bytes`
      * `[ ]`   Contract: the second group's identity → `Ok(EncodeG2SuccessReturn { bytes })` holding 128 zero bytes
      * `[ ]`   Collaborators: arkworks, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_encode_g2_payload`, and `build_bn254_arkworks_g2`
      * `[ ]`   Arrange: `build_encode_g2_payload` with the point override `build_bn254_arkworks_g2` with the value `G2Affine::identity()`
      * `[ ]`   Act: `encode_g2(EncodeG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; the bytes of `success.bytes` equal `zero_bytes(128)` as a byte slice
    * `[ ]`   `encode_scalar_writes_the_largest_canonical_scalar`
      * `[ ]`   Contract: a scalar → `Ok(EncodeScalarSuccessReturn { bytes })` holding its 32 big-endian bytes in this concrete's `Bn254ArkworksEncodedScalar`, inside a `Secret`
      * `[ ]`   Collaborators: arkworks' integer encoding, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_encode_scalar_payload`, and `build_bn254_arkworks_scalar`
      * `[ ]`   Arrange: `build_encode_scalar_payload` with the scalar override `build_bn254_arkworks_scalar` with the value `-Fr::from(1u64)`
      * `[ ]`   Act: `encode_scalar(EncodeScalarParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.bytes.expose()` binds to a `Bn254ArkworksEncodedScalar` typed binding; its bytes equal `vector_bytes(GROUP_ORDER_MINUS_ONE_HEX)` as a byte slice
    * `[ ]`   `sample_from_uniform_bytes_rejects_a_wrong_length`
      * `[ ]`   Contract: `payload.uniform.expose().len() != 64` → `Err(SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual })`
      * `[ ]`   Collaborators: `Secret::expose`, real; fixtures `build_sample_uniform_scalar_payload` and `build_secret`
      * `[ ]`   Arrange: `build_sample_uniform_scalar_payload` with the uniform override `build_secret` holding `zero_bytes(63)`
      * `[ ]`   Act: `Bn254ArkworksScalar::sample_from_uniform_bytes(SampleUniformScalarParams, payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual: 63 })`, the whole expected error
    * `[ ]`   `sample_from_uniform_bytes_reduces_the_group_order_to_zero`
      * `[ ]`   Contract: a 64-byte uniform input → `Ok(SampleUniformScalarSuccessReturn { scalar })` holding the input reduced modulo the group order, inside a `Secret`
      * `[ ]`   Collaborators: `Secret::expose` and arkworks' modular reduction, real; fixtures `build_sample_uniform_scalar_payload` and `build_secret`
      * `[ ]`   Arrange: `build_sample_uniform_scalar_payload` with the uniform override `build_secret` holding `vector_bytes(UNIFORM_GROUP_ORDER_HEX)`, whose reduction is zero and so differs from the input
      * `[ ]`   Act: `Bn254ArkworksScalar::sample_from_uniform_bytes(SampleUniformScalarParams, payload)`
      * `[ ]`   Assert: the success arm is extracted with `expect`; `success.scalar.expose().value` equals `Fr::from(0u64)`
    * `[ ]`   `add_scalar_of_two_and_three_is_five`
      * `[ ]`   Contract: `payload.left` and `payload.right` → `Ok(AddScalarSuccessReturn { sum })` holding their sum modulo the group order
      * `[ ]`   Collaborators: arkworks' `Fr` addition, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_add_scalar_payload`, and `build_bn254_arkworks_scalar`
      * `[ ]`   Arrange: `build_add_scalar_payload` with the left override `build_bn254_arkworks_scalar` with the value `Fr::from(2u64)` and the right override with the value `Fr::from(3u64)`
      * `[ ]`   Act: `add_scalar(AddScalarParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `Fr::from(5u64)`
    * `[ ]`   `add_scalar_reduces_modulo_the_group_order`
      * `[ ]`   Contract: `payload.left` and `payload.right` → `Ok(AddScalarSuccessReturn { sum })` holding their sum modulo the group order
      * `[ ]`   Collaborators: arkworks' `Fr` addition, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_add_scalar_payload`, and `build_bn254_arkworks_scalar`
      * `[ ]`   Arrange: `build_add_scalar_payload` with the left override `build_bn254_arkworks_scalar` with the value `scalar_value(GROUP_ORDER_MINUS_ONE_HEX)` and the right override with the value `Fr::from(2u64)`, whose integer sum exceeds the group order
      * `[ ]`   Act: `add_scalar(AddScalarParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `Fr::from(1u64)`
    * `[ ]`   `mul_scalar_of_two_and_three_is_six`
      * `[ ]`   Contract: `payload.left` and `payload.right` → `Ok(MulScalarSuccessReturn { product })` holding their product modulo the group order
      * `[ ]`   Collaborators: arkworks' `Fr` multiplication, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_mul_scalar_payload`, and `build_bn254_arkworks_scalar`
      * `[ ]`   Arrange: `build_mul_scalar_payload` with the left override `build_bn254_arkworks_scalar` with the value `Fr::from(2u64)` and the right override with the value `Fr::from(3u64)`
      * `[ ]`   Act: `mul_scalar(MulScalarParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `Fr::from(6u64)`
    * `[ ]`   `mul_scalar_reduces_modulo_the_group_order`
      * `[ ]`   Contract: `payload.left` and `payload.right` → `Ok(MulScalarSuccessReturn { product })` holding their product modulo the group order
      * `[ ]`   Collaborators: arkworks' `Fr` multiplication, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_mul_scalar_payload`, and `build_bn254_arkworks_scalar`
      * `[ ]`   Arrange: `build_mul_scalar_payload` with the left override `build_bn254_arkworks_scalar` with the value `scalar_value(GROUP_ORDER_MINUS_ONE_HEX)` and the right override with the value `Fr::from(2u64)`, whose integer product exceeds the group order
      * `[ ]`   Act: `mul_scalar(MulScalarParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `scalar_value(GROUP_ORDER_MINUS_TWO_HEX)`
    * `[ ]`   `neg_scalar_of_one_is_the_group_order_minus_one`
      * `[ ]`   Contract: `payload.scalar` → `Ok(NegScalarSuccessReturn { negation })` holding the group order minus the scalar
      * `[ ]`   Collaborators: arkworks' `Fr` negation, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_neg_scalar_payload`, and `build_bn254_arkworks_scalar`
      * `[ ]`   Arrange: `build_neg_scalar_payload` with the scalar override `build_bn254_arkworks_scalar` with the value `Fr::from(1u64)`
      * `[ ]`   Act: `neg_scalar(NegScalarParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.negation.value` equals `scalar_value(GROUP_ORDER_MINUS_ONE_HEX)`
    * `[ ]`   `neg_scalar_of_zero_is_zero`
      * `[ ]`   Contract: `payload.scalar` of zero → `Ok(NegScalarSuccessReturn { negation })` holding zero
      * `[ ]`   Collaborators: arkworks' `Fr` negation, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_neg_scalar_payload`, and `build_bn254_arkworks_scalar`
      * `[ ]`   Arrange: `build_neg_scalar_payload` with the scalar override `build_bn254_arkworks_scalar` with the value `Fr::from(0u64)`
      * `[ ]`   Act: `neg_scalar(NegScalarParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.negation.value` equals `Fr::from(0u64)`
    * `[ ]`   `neg_g1_of_the_generator_is_the_negated_generator`
      * `[ ]`   Contract: `payload.point` → `Ok(NegG1SuccessReturn { negation })` holding `(x, p - y)` for a point `(x, y)`
      * `[ ]`   Collaborators: arkworks' affine negation, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_neg_g1_payload`, and `build_bn254_arkworks_g1`
      * `[ ]`   Arrange: `build_neg_g1_payload` with the point override `build_bn254_arkworks_g1` at its default generator
      * `[ ]`   Act: `neg_g1(NegG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.negation.value` equals `eip_196_negated_generator()`
    * `[ ]`   `neg_g1_of_the_identity_is_the_identity`
      * `[ ]`   Contract: `payload.point` of the identity → `Ok(NegG1SuccessReturn { negation })` holding the identity
      * `[ ]`   Collaborators: arkworks' affine negation, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_neg_g1_payload`, and `build_bn254_arkworks_g1`
      * `[ ]`   Arrange: `build_neg_g1_payload` with the point override `build_bn254_arkworks_g1` with the value `G1Affine::identity()`
      * `[ ]`   Act: `neg_g1(NegG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.negation.value` equals `G1Affine::identity()`
    * `[ ]`   `neg_g2_negates_each_y_coefficient_modulo_the_base_field`
      * `[ ]`   Contract: `payload.point` → `Ok(NegG2SuccessReturn { negation })` holding `(x, p - y)` for a point `(x, y)`, each `y` coefficient negated
      * `[ ]`   Collaborators: arkworks' affine negation, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_neg_g2_payload`, and `build_bn254_arkworks_g2`
      * `[ ]`   Arrange: `build_neg_g2_payload` with the point override `build_bn254_arkworks_g2` at its default generator
      * `[ ]`   Act: `neg_g2(NegG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; the negation's `xy()` has an `x` equal to the generator's `x` from `eip_197_generator().xy()`; its `y.c0` and `y.c1`, each as a `BigUint`, equal `base_field_modulus()` minus the generator's corresponding coefficient as a `BigUint`
    * `[ ]`   `neg_g2_of_the_identity_is_the_identity`
      * `[ ]`   Contract: `payload.point` of the identity → `Ok(NegG2SuccessReturn { negation })` holding the identity
      * `[ ]`   Collaborators: arkworks' affine negation, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_neg_g2_payload`, and `build_bn254_arkworks_g2`
      * `[ ]`   Arrange: `build_neg_g2_payload` with the point override `build_bn254_arkworks_g2` with the value `G2Affine::identity()`
      * `[ ]`   Act: `neg_g2(NegG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.negation.value` equals `G2Affine::identity()`
    * `[ ]`   `is_identity_g1_is_true_for_the_identity`
      * `[ ]`   Contract: `payload.point` → `Ok(IsIdentityG1SuccessReturn { is_identity })`, true exactly for the identity
      * `[ ]`   Collaborators: arkworks, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_is_identity_g1_payload`, and `build_bn254_arkworks_g1`
      * `[ ]`   Arrange: `build_is_identity_g1_payload` with the point override `build_bn254_arkworks_g1` with the value `G1Affine::identity()`
      * `[ ]`   Act: `is_identity_g1(IsIdentityG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_identity` is true
    * `[ ]`   `is_identity_g1_is_false_for_the_generator`
      * `[ ]`   Contract: `payload.point` → `Ok(IsIdentityG1SuccessReturn { is_identity })`, true exactly for the identity
      * `[ ]`   Collaborators: arkworks, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_is_identity_g1_payload`, and `build_bn254_arkworks_g1`
      * `[ ]`   Arrange: `build_is_identity_g1_payload` with the point override `build_bn254_arkworks_g1` at its default generator
      * `[ ]`   Act: `is_identity_g1(IsIdentityG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_identity` is false
    * `[ ]`   `is_identity_g2_is_true_for_the_identity`
      * `[ ]`   Contract: `payload.point` → `Ok(IsIdentityG2SuccessReturn { is_identity })`, true exactly for the identity
      * `[ ]`   Collaborators: arkworks, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_is_identity_g2_payload`, and `build_bn254_arkworks_g2`
      * `[ ]`   Arrange: `build_is_identity_g2_payload` with the point override `build_bn254_arkworks_g2` with the value `G2Affine::identity()`
      * `[ ]`   Act: `is_identity_g2(IsIdentityG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_identity` is true
    * `[ ]`   `is_identity_g2_is_false_for_the_generator`
      * `[ ]`   Contract: `payload.point` → `Ok(IsIdentityG2SuccessReturn { is_identity })`, true exactly for the identity
      * `[ ]`   Collaborators: arkworks, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_is_identity_g2_payload`, and `build_bn254_arkworks_g2`
      * `[ ]`   Arrange: `build_is_identity_g2_payload` with the point override `build_bn254_arkworks_g2` at its default generator
      * `[ ]`   Act: `is_identity_g2(IsIdentityG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_identity` is false
    * `[ ]`   `pairing_product_of_no_terms_is_the_target_group_identity`
      * `[ ]`   Contract: an empty `payload.terms` → `Ok(PairingProductSuccessReturn { product })` holding the target group's identity
      * `[ ]`   Collaborators: arkworks' multi-pairing and target-group exponentiation, run for real as the vendor; fixtures `build_bn254_arkworks_pairing` and `build_pairing_product_payload`
      * `[ ]`   Arrange: `build_pairing_product_payload` at its default of no terms
      * `[ ]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` is the identity under `is_zero()`
    * `[ ]`   `pairing_product_of_the_generators_is_not_the_target_group_identity`
      * `[ ]`   Contract: a pairing of the generators → `Ok(PairingProductSuccessReturn { product })` holding a value other than the target group's identity
      * `[ ]`   Collaborators: arkworks' multi-pairing and target-group exponentiation, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_pairing_product_payload`, and `build_pairing_product_term`
      * `[ ]`   Arrange: `build_pairing_product_payload` with the terms override holding one `build_pairing_product_term` at its default generators
      * `[ ]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` is not the identity under `is_zero()`
    * `[ ]`   `pairing_product_is_bilinear`
      * `[ ]`   Contract: a term whose first-group element is twice the generator → `Ok(PairingProductSuccessReturn { product })` holding the generators' pairing raised to the power two
      * `[ ]`   Collaborators: arkworks' multi-pairing and target-group exponentiation, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_pairing_product_payload`, `build_pairing_product_term`, and `build_bn254_arkworks_g1`; the expectation is `definition_value_power`, the identifier's definition raised to a power in the fixtures, independent of the correction `pairing_product` applies
      * `[ ]`   Arrange: `build_pairing_product_payload` with the terms override holding one `build_pairing_product_term` with the first-group override `build_bn254_arkworks_g1` with the value `eip_196_doubled_generator()` over the second-group generator
      * `[ ]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value.0` equals `definition_value_power(eip_196_generator(), eip_197_generator(), 2)`
    * `[ ]`   `pairing_product_multiplies_its_terms`
      * `[ ]`   Contract: `payload.terms` → `Ok(PairingProductSuccessReturn { product })` holding the product of each term's pairing
      * `[ ]`   Collaborators: arkworks' multi-pairing and target-group exponentiation, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_pairing_product_payload`, and `build_pairing_product_term`; the expectation is `definition_value_power`
      * `[ ]`   Arrange: `build_pairing_product_payload` with the terms override holding two `build_pairing_product_term` values at their default generators, so a product over one term differs from the expected square
      * `[ ]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value.0` equals `definition_value_power(eip_196_generator(), eip_197_generator(), 2)`
    * `[ ]`   `pairing_product_of_a_pairing_and_its_first_group_negation_is_the_target_group_identity`
      * `[ ]`   Contract: `payload.terms` whose pairings multiply to the target group's identity → `Ok(PairingProductSuccessReturn { product })` holding the identity
      * `[ ]`   Collaborators: arkworks' multi-pairing and target-group exponentiation, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_pairing_product_payload`, `build_pairing_product_term`, and `build_bn254_arkworks_g1`
      * `[ ]`   Arrange: `build_pairing_product_payload` with the terms override holding two `build_pairing_product_term` values: the generators, and the first-group override `build_bn254_arkworks_g1` with the value `eip_196_negated_generator()` over the second-group generator
      * `[ ]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` is the identity under `is_zero()`
    * `[ ]`   `pairing_product_of_the_generators_equals_the_definition`
      * `[ ]`   Contract: a pairing of the generators → `Ok(PairingProductSuccessReturn { product })` holding the Miller loop's value raised to the exact exponent `(p^12 - 1) / r`, the value `TargetGroupEncodingIdentifier::Bn254V1` defines
      * `[ ]`   Collaborators: arkworks' multi-pairing and target-group exponentiation, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_pairing_product_payload`, and `build_pairing_product_term`; the expectation is `definition_value`, the Miller loop over the generators raised to `definition_exponent()` built by `num-bigint`, independent of the correction `pairing_product` applies
      * `[ ]`   Arrange: `build_pairing_product_payload` with the terms override holding one `build_pairing_product_term` at its default generators
      * `[ ]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value.0` equals `definition_value(eip_196_generator(), eip_197_generator())`
    * `[ ]`   `encode_gt_writes_the_target_group_identity_with_c0_c0_c0_first`
      * `[ ]`   Contract: a target-group value → `Ok(EncodeGtSuccessReturn { bytes })` holding its twelve coefficients in tower order, `c0.c0.c0` first, each 32 bytes big-endian, inside a `Secret`
      * `[ ]`   Collaborators: arkworks' integer encoding, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_encode_gt_payload`, and `build_bn254_arkworks_gt`; the expectation is `gt_identity_encoding()`
      * `[ ]`   Arrange: `build_encode_gt_payload` with the value override `build_bn254_arkworks_gt` with the value `PairingOutput(Fq12::ONE)`, the target group's identity
      * `[ ]`   Act: `encode_gt(EncodeGtParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.bytes.expose()` binds to a `Bn254ArkworksEncodedGt` typed binding; its bytes equal `gt_identity_encoding()` as a byte slice
    * `[ ]`   `scalar_field_order_is_the_group_order`
      * `[ ]`   Contract: any call → `Ok(ScalarFieldOrderSuccessReturn { bytes })` holding the group order's 32 big-endian bytes
      * `[ ]`   Collaborators: arkworks' scalar field modulus, run for real as the vendor; fixture `build_bn254_arkworks_pairing`; `ScalarFieldOrderParams` and `ScalarFieldOrderPayload` by their production values
      * `[ ]`   Arrange: `build_bn254_arkworks_pairing()`
      * `[ ]`   Act: `scalar_field_order(ScalarFieldOrderParams, ScalarFieldOrderPayload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.bytes` equals `vector_bytes(GROUP_ORDER_HEX)`
    * `[ ]`   `g1_outside_subgroup_encoding_is_absent_where_the_cofactor_is_one`
      * `[ ]`   Contract: any call → `Ok(G1OutsideSubgroupEncodingSuccessReturn { bytes: None })`
      * `[ ]`   Collaborators: none called; fixture `build_bn254_arkworks_pairing`; `G1OutsideSubgroupEncodingParams` and `G1OutsideSubgroupEncodingPayload` by their production values
      * `[ ]`   Arrange: `build_bn254_arkworks_pairing()`
      * `[ ]`   Act: `g1_outside_subgroup_encoding(G1OutsideSubgroupEncodingParams, G1OutsideSubgroupEncodingPayload)`
      * `[ ]`   Assert: the success arm is extracted with `expect`; `success.bytes.is_none()` is true
    * `[ ]`   `g2_outside_subgroup_encoding_encodes_an_on_curve_point_outside_the_subgroup`
      * `[ ]`   Contract: any call → `Ok(G2OutsideSubgroupEncodingSuccessReturn { bytes })` holding the precompile encoding of an on-curve point outside the prime-order subgroup, in this concrete's `Bn254ArkworksEncodedG2`
      * `[ ]`   Collaborators: arkworks' curve and subgroup checks, run for real as the vendor; fixture `build_bn254_arkworks_pairing`; `G2OutsideSubgroupEncodingParams` and `G2OutsideSubgroupEncodingPayload` by their production values; the point read back by `g2_point_from_encoding`
      * `[ ]`   Arrange: `build_bn254_arkworks_pairing()`
      * `[ ]`   Act: `g2_outside_subgroup_encoding(G2OutsideSubgroupEncodingParams, G2OutsideSubgroupEncodingPayload)`
      * `[ ]`   Assert: the success arm is extracted with `expect`; `success.bytes` binds to a `Bn254ArkworksEncodedG2` typed binding; the point `g2_point_from_encoding` reads from its bytes satisfies `is_on_curve()` and does not satisfy `is_in_correct_subgroup_assuming_on_curve()`
    * `[ ]`   `g2_outside_subgroup_encoding_has_a_real_first_coordinate_and_the_lesser_root`
      * `[ ]`   Contract: any call → the encoded point's first coordinate is `(c0, 0)` with the least `c0` of an on-curve point outside the subgroup, and its `y` is the lesser of the two roots, compared by `y.c1` and then `y.c0`
      * `[ ]`   Collaborators: arkworks' curve and subgroup checks, run for real as the vendor; fixture `build_bn254_arkworks_pairing`; `G2OutsideSubgroupEncodingParams` and `G2OutsideSubgroupEncodingPayload` by their production values; the expectations are `g2_outside_subgroup_bytes()`, the ascending search in the fixtures, and `base_field_modulus()`
      * `[ ]`   Arrange: `build_bn254_arkworks_pairing()`
      * `[ ]`   Act: `g2_outside_subgroup_encoding(G2OutsideSubgroupEncodingParams, G2OutsideSubgroupEncodingPayload)`
      * `[ ]`   Assert: the success arm is extracted with `expect`; the encoding's `x.c1` quarter, its first 32 bytes, is all zero; its `x.c0` quarter, its second 32 bytes, equals the `x.c0` quarter of `g2_outside_subgroup_bytes()`; the pair `y.c1`, `y.c0` read from its last 64 bytes as `BigUint` values is not greater than the pair of their negations, each `base_field_modulus()` minus the coefficient, reduced to zero where the coefficient is zero
    * `[ ]`   `encode_gt_writes_the_twelve_coefficients_in_tower_order`
      * `[ ]`   Contract: a target-group value → `Ok(EncodeGtSuccessReturn { bytes })` holding its twelve coefficients in tower order, `c0.c0.c0` through `c1.c2.c1`, each 32 bytes big-endian, inside a `Secret`
      * `[ ]`   Collaborators: arkworks' integer encoding, run for real as the vendor; fixtures `build_bn254_arkworks_pairing`, `build_encode_gt_payload`, and `build_bn254_arkworks_gt`; the expectation is `sequential_gt_encoding()`
      * `[ ]`   Arrange: `build_encode_gt_payload` with the value override `build_bn254_arkworks_gt` with the value `gt_with_sequential_coefficients()`, whose twelve coefficients are distinct, so any exchange of two positions changes the bytes
      * `[ ]`   Act: `encode_gt(EncodeGtParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; the bytes of `success.bytes.expose()` equal `sequential_gt_encoding()` as a byte slice
    * `[ ]`   `bn254_arkworks_g1_zeroize_clears_its_value`
      * `[ ]`   Contract: `Zeroize::zeroize` on a living first-group element → its `value` is the point `new_unchecked(0, 0)`
      * `[ ]`   Collaborators: arkworks' affine zeroization, run for real as the vendor; fixture `build_bn254_arkworks_g1`
      * `[ ]`   Arrange: `build_bn254_arkworks_g1` at its default generator, which differs from the expected point
      * `[ ]`   Act: `g1.zeroize()`
      * `[ ]`   Assert: `g1.value` equals `G1Affine::new_unchecked(Fq::from(0u64), Fq::from(0u64))`
    * `[ ]`   `bn254_arkworks_g2_zeroize_clears_its_value`
      * `[ ]`   Contract: `Zeroize::zeroize` on a living second-group element → its `value` is the point `new_unchecked(0, 0)`
      * `[ ]`   Collaborators: arkworks' affine zeroization, run for real as the vendor; fixture `build_bn254_arkworks_g2`
      * `[ ]`   Arrange: `build_bn254_arkworks_g2` at its default generator, which differs from the expected point
      * `[ ]`   Act: `g2.zeroize()`
      * `[ ]`   Assert: `g2.value` equals `G2Affine::new_unchecked(Fq2::new(Fq::from(0u64), Fq::from(0u64)), Fq2::new(Fq::from(0u64), Fq::from(0u64)))`
    * `[ ]`   `bn254_arkworks_gt_zeroize_clears_its_value`
      * `[ ]`   Contract: `Zeroize::zeroize` on a living target-group value → every coefficient of its `Fq12` is zero
      * `[ ]`   Collaborators: arkworks' `PairingOutput` zeroization, run for real as the vendor; fixture `build_bn254_arkworks_gt`
      * `[ ]`   Arrange: `build_bn254_arkworks_gt` at its default pairing of the generators, whose `Fq12` is nonzero
      * `[ ]`   Act: `gt.zeroize()`
      * `[ ]`   Assert: `gt.value.0.is_zero()` is true
    * `[ ]`   `bn254_arkworks_encoded_scalar_zeroize_clears_its_bytes`
      * `[ ]`   Contract: `Zeroize::zeroize` on a living encoded scalar → every byte is zero
      * `[ ]`   Collaborators: none; fixture `build_bn254_arkworks_encoded_scalar`
      * `[ ]`   Arrange: `build_bn254_arkworks_encoded_scalar` at its nonzero default
      * `[ ]`   Act: `encoded.zeroize()`
      * `[ ]`   Assert: the bytes of `encoded` equal `zero_bytes(32)` as a byte slice
    * `[ ]`   `bn254_arkworks_encoded_gt_zeroize_clears_its_bytes`
      * `[ ]`   Contract: `Zeroize::zeroize` on a living encoded target-group value → every byte is zero
      * `[ ]`   Collaborators: none; fixture `build_bn254_arkworks_encoded_gt`
      * `[ ]`   Arrange: `build_bn254_arkworks_encoded_gt` at its nonzero default
      * `[ ]`   Act: `encoded.zeroize()`
      * `[ ]`   Assert: the bytes of `encoded` equal `zero_bytes(384)` as a byte slice

  * `[✅]`   `construction`
    * `[✅]`   `Bn254ArkworksPairing::try_new` is the concrete's only producer, computing its one field, and its only caller is the pairing factory, which reads `Bn254ArkworksPairing::DECLARATION` before constructing
    * `[✅]`   A group element or scalar is produced only by the adapter's generators, arithmetic, and decoders, or by the scalar's sampling bound; a target-group value only by `pairing_product`; an encoded value only by the adapter's encoders and reference methods; no consumer constructs one from library values

  * `[✅]`   `adapters/pairing/src/bn254_arkworks/mod.rs`
    * `[✅]`   Module declarations: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, `pub(crate) mod provides;`, and `#[cfg(test)] mod test;`
    * `[✅]`   Imports: from `crate::factory::provides` every params, payload, return, success-return, and error type of the three traits and the sampling bound, `IPairingAdapter`, `IPairingArithmetic`, `IPairingReference`, `ISampleUniformScalar`, `PAIRING_INTERFACE_VERSION`, `PairingConcrete`, `PairingCurve`, `PairingDeclaration`, `PrecompileEncoding`, `TargetGroupEncodingIdentifier`, and `VerifierGroupArithmetic`; `ark_bn254::{Bn254, Fq, Fq2, Fr, G1Affine, G1Projective, G2Affine, G2Projective}`; `ark_ec::{AffineRepr, CurveGroup, VariableBaseMSM, pairing::Pairing}`; `ark_ff::{BigInt, BigInteger, Field, PrimeField, Zero}`; `domain::{Secret, SecretConstructorParams}`; from `interface` every type it declares; `zeroize::{Zeroize, ZeroizeOnDrop}`
    * `[✅]`   `const BN254_SEED: u64 = 4_965_661_367_192_848_881;`, private, with the doc comment "The seed `x` of the BN254 curve EIP-197 fixes, from which the curve's `p` and `r` derive."
    * `[✅]`   `impl Bn254ArkworksPairing` holding `pub const DECLARATION: PairingDeclaration`, the constant the interaction spec states, and `pub fn try_new(params: Bn254ArkworksPairingConstructorParams) -> Bn254ArkworksPairingTryNewReturn` realizing the interaction spec's construct branch
    * `[✅]`   `impl IPairingAdapter for Bn254ArkworksPairing` with `const DECLARATION: PairingDeclaration = Bn254ArkworksPairing::DECLARATION;`, `const CONCRETE: PairingConcrete = PairingConcrete::Bn254Arkworks;`, `type Scalar = Bn254ArkworksScalar;`, `type G1 = Bn254ArkworksG1;`, `type G2 = Bn254ArkworksG2;`, `type EncodedG1 = Bn254ArkworksEncodedG1;`, `type EncodedG2 = Bn254ArkworksEncodedG2;`, and `type EncodedScalar = Bn254ArkworksEncodedScalar;`, every method realizing its branches in the interaction spec
    * `[✅]`   `impl IPairingArithmetic for Bn254ArkworksPairing` with `type Gt = Bn254ArkworksGt;` and `type EncodedGt = Bn254ArkworksEncodedGt;`, every method realizing its branches in the interaction spec, `pairing_product` multiplying the `PairingOutput` by `self.reduced_pairing_correction`
    * `[✅]`   `impl IPairingReference for Bn254ArkworksPairing`, every method realizing its branches in the interaction spec, the search as `(1u64..).find_map(…)` with the `SearchExhausted` return by `let … else`
    * `[✅]`   `impl AsRef<[u8]>` for `Bn254ArkworksEncodedG1`, `Bn254ArkworksEncodedG2`, `Bn254ArkworksEncodedScalar`, and `Bn254ArkworksEncodedGt`, each returning `&self.bytes`; `impl Zeroize` for `Bn254ArkworksEncodedScalar` and `Bn254ArkworksEncodedGt`, each calling `self.bytes.zeroize()`; no implementation returns a mutable reference to the bytes
    * `[✅]`   `impl Zeroize` and `impl Drop` for `Bn254ArkworksScalar`, `Bn254ArkworksG1`, `Bn254ArkworksG2`, and `Bn254ArkworksGt`, each calling `self.value.zeroize()`; `impl ZeroizeOnDrop for Bn254ArkworksScalar {}`, marking the drop behavior the sampling bound requires
    * `[✅]`   `impl ISampleUniformScalar for Bn254ArkworksScalar` with `const UNIFORM_BYTES_LENGTH: usize = 64;` and `sample_from_uniform_bytes` realizing its branches in the interaction spec, the scalar moved into a `Secret` by `let Ok(scalar) = Secret::try_new(SecretConstructorParams { value });`
    * `[✅]`   No `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`; the correction is computed in `try_new` and read from no configuration; neither the order nor a point is written as a literal

  * `[✅]`   `adapters/pairing/src/bn254_arkworks/provides.rs`
    * `[✅]`   `pub(crate) use super::interface::*;` and `#[cfg(test)] pub(crate) use super::mock::*;`, exposing every item of the concrete's interface to the crate and, in the crate's own test build, the concrete's builders, fixtures, and helpers, and nothing beyond them

  * `[✅]`   `directionality`
    * `[✅]`   `bn254_arkworks` depends on the `factory` module's surface through `crate::factory::provides`, on `domain`, on `zeroize`, and on the arkworks crates; the `factory` module depends on `domain` and `zeroize` and on no concrete; `IPairingArithmetic` and `IPairingReference` each depend on `IPairingAdapter` within the `factory` module; among repository crates the crate depends on `crates/domain` alone at runtime and on `adapters/random` for tests only, the dependency map's edge
    * `[✅]`   `pairing/bn254_halo2curves`, `pairing/bls12_381_arkworks`, and `pairing/bls12_381_halo2curves` each implement the three traits and declare their identifier; `pairing/factory` constructs each concrete, the family form's recorded cycle, admits by precompile encoding and identifier, and requires the arithmetic and reference traits of the concrete a consumer receives

  * `[ ]`   `requirements`
    * `[✅]`   `adapters/pairing/Cargo.toml` carries exactly the tables and keys stated above, and no `ark-` crate is named in the crate outside the arkworks concretes
    * `[✅]`   The family's encoders and this concrete return distinct owned fixed-width types for the first group, the second group, the scalar, and the target group; scalar and target-group bytes are held inside `Secret`, and every encoded type exposes only an immutable wire-byte view
    * `[✅]`   `SampleUniformScalarErrorReturn`, `DecodeG1ErrorReturn`, `DecodeG2ErrorReturn`, `DecodeScalarErrorReturn`, `G1OutsideSubgroupEncodingErrorReturn`, and `G2OutsideSubgroupEncodingErrorReturn` derive `Debug`, `PartialEq`, and `Eq`, and every refusal the concrete's tests assert is asserted by `assert_eq!` against the whole expected error
    * `[✅]`   `cargo check --all-targets --all-features`, `cargo fmt --check`, and `cargo deny check` complete without error; `cargo clippy --all-targets --all-features` reports nothing beyond the library target's unused-item warnings for the `bn254_arkworks` concrete, which `pairing/factory` constructs
    * `[ ]`   The `ZeroizeOnDrop` compile-time assertion compiles and `bn254_arkworks_scalar_zeroize_clears_its_value` passes (CR-07), and `encode_g1_writes_the_eip_196_generator`, `encode_g2_writes_the_eip_197_generator`, and `encode_scalar_writes_the_largest_canonical_scalar` bind their results as this concrete's own encoded types and pass (the typed encodings)
    * `[ ]`   The `g1_generator`, `g2_generator`, `encode_g1`, `encode_g2`, `decode_g1`, `decode_g2`, and `decode_scalar` blocks, the `add_g1`, `add_g2`, `mul_g1`, `mul_g2`, `msm_g1`, and `msm_g2` blocks, the `pairing_product_is_one` blocks, and the `sample_from_uniform_bytes` blocks pass, and the declaration and concrete-identity compile-time assertions compile (CR-10, the generic interface on BN254 over arkworks; CR-05 for the sampled scalar)
    * `[✅]`   `add_scalar_of_two_and_three_is_five`, `add_scalar_reduces_modulo_the_group_order`, `mul_scalar_of_two_and_three_is_six`, `mul_scalar_reduces_modulo_the_group_order`, `neg_scalar_of_one_is_the_group_order_minus_one`, and `neg_scalar_of_zero_is_zero` pass (CR-09, the scalar arithmetic the delivery proof's responses use)
    * `[✅]`   The `neg_g1`, `neg_g2`, `is_identity_g1`, and `is_identity_g2` blocks pass (CR-04 envelope decryption and identity-key rejection; CR-08 trivial identity-element refusal)
    * `[ ]`   `pairing_product_of_no_terms_is_the_target_group_identity`, `encode_gt_writes_the_target_group_identity_with_c0_c0_c0_first`, `pairing_product_of_the_generators_is_not_the_target_group_identity`, `pairing_product_is_bilinear`, `pairing_product_multiplies_its_terms`, and `pairing_product_of_a_pairing_and_its_first_group_negation_is_the_target_group_identity` pass (CR-08, the encapsulated value and the validity, well-formedness, and decapsulation equations)
    * `[ ]`   `pairing_product_of_the_generators_equals_the_definition` and `reduced_pairing_correction_inverts_the_library_multiple` pass (CR-10, the target-group value fixed by the identifier's definition, executed in the test)
    * `[ ]`   `scalar_field_order_is_the_group_order`, `g1_outside_subgroup_encoding_is_absent_where_the_cofactor_is_one`, `g2_outside_subgroup_encoding_encodes_an_on_curve_point_outside_the_subgroup`, `decode_g2_rejects_a_point_outside_the_subgroup`, and `g2_outside_subgroup_encoding_has_a_real_first_coordinate_and_the_lesser_root` pass (CR-10, the subgroup-rejection input a contract's own check is proven against; CR-11, the modulus a contract reduces every hash-to-scalar digest by)
    * `[✅]`   Code outside `adapters/pairing` naming `Bn254ArkworksPairing` or anything under `bn254_arkworks` fails to compile; the crate's public surface is the `factory` module's `provides`

* `[ ]`   `pairing/bn254_halo2curves` **BN254 pairing concrete on halo2curves implementing the pairing family's generic interface, arithmetic trait, and reference trait over the EIP-196 and EIP-197 encodings and the `Bn254V1` target-group value, a further concrete beneath the pairing factory**

  * `[✅]`   `objective`
    * `[✅]`   Problem: the harness benchmark compares pairing libraries per curve through the factory, so BN254 needs a concrete over another library that satisfies the family's generic interface exactly as the arkworks concrete does (CR-10)
    * `[✅]`   Problem: the KEM, the envelope, and the delivery proof consume the pairing family's arithmetic trait through whichever concrete the factory resolves, and the bytes the KEM hands the KDF are the identifier's target-group value, so this concrete implements the trait with target-group values equal to the identifier's definition, and therefore to the arkworks concrete's, under the one identifier both declare; `halo2curves`' BN256 final exponentiation computes the exact exponent `(p^12 - 1) / r`, the Devegili–Scott–Dahab chain in its source, so the library's reduced pairing is the identifier's value and the concrete applies no correction (CR-04; CR-07; CR-08; CR-09; CR-10; `docs/research/cryptography.md`'s pairing adapter capability declaration)
    * `[✅]`   Problem: every concrete a consumer receives returns the group order and the family's outside-the-subgroup encodings, and the libraries of one curve return the same bytes, so this concrete implements the family's reference trait under the family's rule from its own library (CR-10 on chain; CR-11; the dependency map's `pairing/bn254_arkworks` row; the Pairing adapters and key derivation milestone's exit)
    * `[✅]`   Functional: the concrete implements `IPairingAdapter` over `halo2curves`' BN256, which is BN254, with its own scalar, group-element, and encoded types over the library's elements as the associated types
    * `[✅]`   Functional: it encodes and decodes a first-group point as EIP-196's 64 bytes and a second-group point as EIP-197's 128 bytes, each coordinate a 32-byte big-endian integer, the second-group coordinates imaginary part first, the point at infinity as all zero bytes, and a scalar as 32 big-endian bytes, rejecting a wrong length, a non-canonical field element, a point off the curve, and a point outside the prime-order subgroup
    * `[✅]`   Functional: its scalar type implements the family's sampling bound, reading the 64 uniform bytes as one big-endian integer reduced modulo the group order, so the same input bytes sample the same scalar under either BN254 concrete
    * `[✅]`   Functional: it declares the BN254 curve, first-group-only arithmetic at the verifier, the EIP-196 and EIP-197 encoding, `TargetGroupEncodingIdentifier::Bn254V1`, its adapter version, and the interface version it implements, and names `PairingConcrete::Bn254Halo2curves`
    * `[✅]`   Functional: `Bn254Halo2curvesPairing` implements `IPairingArithmetic` with every method's behavior as `pairing/bn254_arkworks` states it for the trait, its `pairing_product` returning the library's reduced pairing, which is the identifier's exact value
    * `[✅]`   Functional: the target-group encoding reads the `Fq12` through `Gt::inner` and writes its twelve coefficients in the tower order `c0.c0.c0`, `c0.c0.c1`, `c0.c1.c0`, `c0.c1.c1`, `c0.c2.c0`, `c0.c2.c1`, `c1.c0.c0`, `c1.c0.c1`, `c1.c1.c0`, `c1.c1.c1`, `c1.c2.c0`, `c1.c2.c1`, each 32 bytes big-endian, over halo2curves' BN256 tower `Fq2 = Fq[u] / (u^2 + 1)`, `Fq6 = Fq2[v] / (v^3 - (u + 9))`, `Fq12 = Fq6[w] / (w^2 - v)`
    * `[✅]`   Functional: the concrete's test executes the identifier's definition, the library's Miller loop over the two generators raised to the integer `(p^12 - 1) / r` built by `num-bigint` from the library's moduli and serialized in the tower order, and requires `pairing_product` then `encode_gt` over the generators to yield the same bytes
    * `[✅]`   Functional: `Bn254Halo2curvesScalar`, `Bn254Halo2curvesG1`, `Bn254Halo2curvesG2`, and `Bn254Halo2curvesGt` clear their value through `Zeroize` and on drop
    * `[✅]`   Functional: the concrete returns BN254's group order as 32 big-endian bytes, no first-group point, and the second group's point under the family's rule, encoded as `encode_g2` encodes any point, an encoding its own `decode_g2` refuses as outside the subgroup
    * `[✅]`   Non-functional: `halo2curves` is named only inside the halo2curves concretes; no order and no point is written as a literal outside a test

  * `[✅]`   `role`
    * `[✅]`   Adapter: a further concrete of the pairing family, implementing the generic interface, the sampling bound, the arithmetic trait, and the reference trait `pairing/bn254_arkworks` authors in the `factory` module, and declaring the identifier it authors
    * `[✅]`   Contributes its module line to `lib.rs` and its dependency line to the crate manifest; edits no file of the `factory` module, since its declaration and concrete identity use variants `pairing/bn254_arkworks` declares
    * `[✅]`   Does not select between the BN254 concretes; `pairing/factory` constructs a concrete by the composition's request and `harness-crypto/benchmark` records the default per curve
    * `[✅]`   Does not name, import, or compare against another concrete; `pairing/factory`'s integration test proves agreement with the arkworks concrete
    * `[✅]`   Does not carry a correction; the library's reduced pairing is the identifier's value
    * `[✅]`   Does not carry a commit

  * `[✅]`   `module`
    * `[✅]`   Bounded context: the private `bn254_halo2curves` module of `adapters/pairing`, holding the adapter over `halo2curves`' BN256, its scalar, group-element, target-group, and encoded types over the library's elements, its constructor params, its implementations of the three traits, its declared identifier, its definition test, its clearing, and the builder defaults for its owned types
    * `[✅]`   The node's files: the `adapters/pairing/Cargo.toml` dependency line, the `adapters/pairing/src/lib.rs` module line, and the `bn254_halo2curves` module's `interface.rs`, `interaction.spec.md`, `mock.rs`, `test.rs`, `mod.rs`, and `provides.rs`
    * `[✅]`   Outside: the family's traits and declaration, every other concrete, and the factory's selection and admission

  * `[ ]`   `deps`
    * `[✅]`   The `factory` module's surface, through `crate::factory::provides`: `IPairingAdapter`, `IPairingArithmetic`, `IPairingReference`, `ISampleUniformScalar`, `PairingDeclaration` and its enums, `PairingConcrete`, `TargetGroupEncodingIdentifier`, `PAIRING_INTERFACE_VERSION`, and every params, payload, return, success-return, error type, and builder of the three traits and the sampling bound, as `pairing/bn254_arkworks` states them
    * `[✅]`   `domain`, runtime: `Secret` and `SecretConstructorParams`; `build_secret` and `SecretConstructorParamsOverrides` through the `mocks` feature
    * `[✅]`   `zeroize` `1.9.0`, runtime: the `Zeroize` and `ZeroizeOnDrop` traits and the zeroization of the local 64-byte input copy
    * `[✅]`   `halo2curves` `0.10.0`, external crate, MIT/Apache-2.0, runtime, default features, resolved to the `gt-accessor` overlay, which supplies `Gt::inner(&self) -> &Fq12`; named only in the halo2curves concretes; supplies the curve, the pairing, the group arithmetic, the field arithmetic, and multi-scalar multiplication, and re-exports the `ff`, `group`, and `pairing` traits its types implement, so no separate trait crate is pinned
    * `[ ]`   `hex` `0.4.3` and `num-bigint` `0.4.8`, the crate's dev-dependencies, for the test vectors and the integer exponent in `mock.rs`'s test-only fixtures and the root comparison in `test.rs`
    * `[✅]`   `core::convert::Infallible`, standard library, the constructor's error arm; `core::hint::black_box`, standard library, which keeps each clearing from being removed as a dead store, since `halo2curves`' fields, points, and target-group values implement no `Zeroize`; `core::iter::successors`, standard library, the ascending search
    * `[✅]`   Reverse dependency: `pairing/factory`

  * `[ ]`   `context_slice`
    * `[✅]`   From `halo2curves::bn256`: `Bn256`, `Fq`, `Fq2` with `Fq2::new(c0, c1)` and the accessors `c0()` and `c1()`, `Fq12` with `c0()` and `c1()` and the degree-six accessors `c0()`, `c1()`, and `c2()`, `Fr`, `G1`, `G1Affine`, `G2`, `G2Affine`, and `Gt` with `Gt::identity()`, `Gt::inner()`, and `PartialEq`
    * `[ ]`   `Fr::from(u64)` and `Fq::from(u64)`, the scalars and coordinates the test fixtures and expectations name
    * `[✅]`   From `halo2curves::ff`: `Fr`'s `+`, `*`, and unary `-` modulo the group order; `Field` for `ZERO`, `ONE`, `is_zero()`, `square()`, `sqrt()` returning a `CtOption`, and `pow_vartime(&self, exp: impl AsRef<[u64]>)`, exponentiation by little-endian limbs; unary `-` on `Fq` and `Fq2` and binary `+` on `Fq2`; `PrimeField` for `from_repr(repr) -> CtOption<Self>`, which reads 32 little-endian bytes and is none at or above the modulus, `to_repr()`, 32 little-endian bytes read by `as_ref()`, and `MODULUS`, the `&'static str` hex modulus with the `0x` prefix, on `Fq` and on `Fr`; `FromUniformBytes::<64>::from_uniform_bytes(&[u8; 64])`, a little-endian wide reduction
    * `[✅]`   From `halo2curves::group`: `Curve::to_affine`, `Group::is_identity`, `prime::PrimeCurveAffine` for `generator()`, `identity()`, `is_identity()` returning a `Choice`, and `to_curve()`, `cofactor::CofactorGroup::is_torsion_free`, and the projective `+` and `* Fr`; unary `-` on `G1Affine` and `G2Affine`
    * `[✅]`   From `halo2curves`: `CurveAffine` for `from_xy(x, y) -> CtOption<Self>`, which is none off the curve, `coordinates() -> CtOption<Coordinates<Self>>`, and `b()`; `Coordinates` for `x()` and `y()`; `msm::msm_best(coeffs: &[C::Scalar], bases: &[C]) -> C::Curve`
    * `[✅]`   From `halo2curves::pairing`: `MultiMillerLoop::multi_miller_loop(&[(&G1Affine, &G2Affine)])` on `Bn256`, returning the unreduced `Fq12`, `MillerLoopResult::final_exponentiation` returning `Gt`, whose `is_identity()` is the check, and `Engine::pairing(&G1Affine, &G2Affine)` for the builder default
    * `[✅]`   A `Choice` becomes a `bool` by `bool::from`, and a `CtOption` becomes an `Option` by `Option::from`
    * `[ ]`   From `num-bigint`, in `mock.rs`'s test-only fixtures and in `test.rs`: `BigUint::parse_bytes(&[u8], u32) -> Option<BigUint>`, `BigUint::from(u32)`, `BigUint::from_bytes_be(&[u8])`, `BigUint::pow(&self, u32)`, `Sub`, `Div`, and `Rem` between `BigUint`s, and `to_u64_digits()`
    * `[✅]`   From `domain`: `Secret::try_new(SecretConstructorParams { value })` returning `Result<Secret<T>, Infallible>`, and `Secret::expose(&self) -> &T`

  * `[✅]`   `adapters/pairing/Cargo.toml`
    * `[✅]`   `[dependencies]` holds `halo2curves = "0.10.0"`, default features

  * `[✅]`   `adapters/pairing/src/lib.rs`
    * `[✅]`   The barrel holds `mod bn254_halo2curves;`

  * `[✅]`   `adapters/pairing/src/bn254_halo2curves/interface.rs`
    * `[✅]`   Imports `core::convert::Infallible`; declares nothing beyond the items below
    * `[✅]`   `Bn254Halo2curvesPairing`, the unit struct `pub struct Bn254Halo2curvesPairing;` with no derives, the adapter over `halo2curves`' BN256; `Bn254Halo2curvesPairingConstructorParams`, a unit struct; `Bn254Halo2curvesPairingTryNewReturn`, the alias `Result<Bn254Halo2curvesPairing, Infallible>`, the error arm uninhabited because the adapter takes no configuration
    * `[✅]`   `Bn254Halo2curvesScalar`, `#[derive(Clone)]`, with one field `pub(super) value: halo2curves::bn256::Fr`; `Bn254Halo2curvesG1`, `#[derive(Clone)]`, with one field `pub(super) value: halo2curves::bn256::G1Affine`; `Bn254Halo2curvesG2`, `#[derive(Clone)]`, with one field `pub(super) value: halo2curves::bn256::G2Affine`
    * `[✅]`   `Bn254Halo2curvesGt`, a struct with no derives and one field, `pub(super) value: halo2curves::bn256::Gt`, the target-group value
    * `[✅]`   `Bn254Halo2curvesEncodedG1` and `Bn254Halo2curvesEncodedG2`, `pub struct`s with `#[derive(Clone, PartialEq, Eq)]` and one field each, `pub(super) bytes: [u8; 64]` and `pub(super) bytes: [u8; 128]`; `Bn254Halo2curvesEncodedScalar`, a `pub struct` with no derives and one field `pub(super) bytes: [u8; 32]`; `Bn254Halo2curvesEncodedGt`, a `pub struct` with no derives and one field `pub(super) bytes: [u8; 384]`; the scalar and target-group encodings are returned only inside a `Secret`, and no encoded type has a public constructor or mutable byte access

  * `[✅]`   `adapters/pairing/src/bn254_halo2curves/interaction.spec.md`
    * `[✅]`   The title `` # `bn254_halo2curves` — interaction spec `` and one opening sentence stating the file as the branch contract for the `bn254_halo2curves` module of the `pairing` crate, the pairing adapter over halo2curves' BN256 (BN254) encoding group elements as EIP-196 and EIP-197 precompile input, each branch stating condition, decision, dependency call, and the exact return outcome; then one `##` section per constructor, constant, and method, each headed by its full signature and holding a table with the columns `Branch`, `Condition`, `Decision`, `Dependency call`, and `Outcome`, in the section order `pairing/bn254_arkworks` states for its spec, then `Ordering and edges`
    * `[✅]`   `try_new`: one branch, construct; condition any params; decision none; dependency call none; outcome `Ok(Bn254Halo2curvesPairing)`; the error arm has no branch, `Infallible` being uninhabited
    * `[✅]`   `DECLARATION`: the inherent constant `PairingDeclaration { curve: PairingCurve::Bn254, verifier_group_arithmetic: VerifierGroupArithmetic::FirstGroupOnly, precompile_encoding: PrecompileEncoding::Eip196Eip197, target_group_encoding: TargetGroupEncodingIdentifier::Bn254V1, adapter_version: 1, interface_version: PAIRING_INTERFACE_VERSION }`, readable from the type before any instance exists; the trait constant `IPairingAdapter::DECLARATION` is this constant
    * `[✅]`   `CONCRETE`: the trait constant `PairingConcrete::Bn254Halo2curves`
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

  * `[ ]`   `adapters/pairing/src/bn254_halo2curves/mock.rs`
    * `[ ]`   The module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`; imports `super::interface::{Bn254Halo2curvesEncodedGt, Bn254Halo2curvesEncodedScalar, Bn254Halo2curvesG1, Bn254Halo2curvesG2, Bn254Halo2curvesGt, Bn254Halo2curvesPairing, Bn254Halo2curvesPairingConstructorParams, Bn254Halo2curvesScalar}`, `core::iter::successors`, `halo2curves::CurveAffine`, `halo2curves::bn256::{Bn256, Fq, Fq2, Fq12, Fr, G1Affine, G2Affine, Gt}`, `halo2curves::ff::{Field, PrimeField}`, `halo2curves::group::{cofactor::CofactorGroup, prime::PrimeCurveAffine}`, and `halo2curves::pairing::{Engine, MultiMillerLoop}`; and, `#[cfg(test)]`, `halo2curves::group::Curve`, `hex::decode`, `num_bigint::BigUint`, and `zeroize::ZeroizeOnDrop`
    * `[✅]`   `impl Default for Bn254Halo2curvesScalar` returning `value: Fr::ONE`; `impl Default for Bn254Halo2curvesG1` returning `value: G1Affine::generator()`; `impl Default for Bn254Halo2curvesG2` returning `value: G2Affine::generator()`; `impl Default for Bn254Halo2curvesGt` returning `value: Bn256::pairing(&G1Affine::generator(), &G2Affine::generator())`, a non-identity target-group value; the family's generic builders and `MockIPairingAdapter` read these through `Default`
    * `[ ]`   The builders for the concrete's owned types, each overrides struct `#[derive(Default)]` with one `Option` field and each omitted value taking the type's `Default` above: `Bn254Halo2curvesScalarOverrides` with `pub value: Option<Fr>` and `build_bn254_halo2curves_scalar`; `Bn254Halo2curvesG1Overrides` with `pub value: Option<G1Affine>` and `build_bn254_halo2curves_g1`; `Bn254Halo2curvesG2Overrides` with `pub value: Option<G2Affine>` and `build_bn254_halo2curves_g2`; `Bn254Halo2curvesGtOverrides` with `pub value: Option<Gt>` and `build_bn254_halo2curves_gt`; no corruptions type and no invalidator, since none of these arrives as untrusted data
    * `[ ]`   `build_bn254_halo2curves_pairing() -> Bn254Halo2curvesPairing`, the real instance from `Bn254Halo2curvesPairing::try_new(Bn254Halo2curvesPairingConstructorParams)` through `let Ok(pairing) = …;`; the constructor params are fieldless, so the builder takes no overrides
    * `[ ]`   The builders for the concrete's encoded scalar and encoded target-group types, each overrides struct `#[derive(Default)]` with one `Option` field: `Bn254Halo2curvesEncodedScalarOverrides` with `pub bytes: Option<[u8; 32]>` and `build_bn254_halo2curves_encoded_scalar(overrides: Bn254Halo2curvesEncodedScalarOverrides) -> Bn254Halo2curvesEncodedScalar`, the bytes defaulting to `[1u8; 32]`; `Bn254Halo2curvesEncodedGtOverrides` with `pub bytes: Option<[u8; 384]>` and `build_bn254_halo2curves_encoded_gt`, the bytes defaulting to `[1u8; 384]`; the defaults are nonzero so a zeroized value differs from a built one; no corruptions type and no invalidator, since neither arrives as untrusted data
    * `[ ]`   The test fixtures, each `#[cfg(test)]`, since they use the crate's dev-dependencies: the constants `pairing/bn254_arkworks` states in its `mock.rs`, by the same names and values, and `UNIFORM_FIVE_HEX`, 63 zero bytes and `05`; the helpers `vector_bytes`, `zero_bytes`, `gt_identity_encoding`, `base_field_modulus`, and `requires_zeroize_on_drop` as `pairing/bn254_arkworks` states them; `scalar_value(hex: &str) -> Fr`, the 32 bytes of `vector_bytes(hex)` reversed to little-endian and read by `Fr::from_repr`, unpacked by `let Some(scalar) = Option::from(…) else { panic!("the scalar is canonical") };`; `eip_196_generator() -> G1Affine`, `G1Affine::from_xy(Fq::from(1u64), Fq::from(2u64))` unpacked the same way; `eip_196_negated_generator() -> G1Affine`, `G1Affine::from_xy` over the two 32-byte halves of `vector_bytes(NEG_G1_GENERATOR_HEX)`, each reversed to little-endian and read by `Fq::from_repr`; `eip_197_generator() -> G2Affine`, `G2Affine::from_xy(Fq2::new(x_c0, x_c1), Fq2::new(y_c0, y_c1))` over the four 32-byte quarters of `vector_bytes(G2_GENERATOR_HEX)`, read in order as `x_c1`, `x_c0`, `y_c1`, `y_c0`, each reversed to little-endian and read by `Fq::from_repr`; `g2_point_from_encoding(bytes: &[u8]) -> Option<G2Affine>`, the same reading over any 128 bytes, `None` off the curve; `g2_outside_subgroup_bytes() -> Vec<u8>`, the least on-curve point outside the subgroup, `successors(Some(Fq::ONE), |c0| Some(*c0 + Fq::ONE)).find_map(|c0| { let x = Fq2::new(c0, Fq::ZERO); Option::<Fq2>::from((x.square() * x + G2Affine::b()).sqrt()).and_then(|y| Option::<G2Affine>::from(G2Affine::from_xy(x, y)).filter(|point| !bool::from(point.to_curve().is_torsion_free())).map(|_| (x, y))) })` unpacked by `let Some((x, y)) = … else { panic!("an on-curve point outside the subgroup exists") };`, encoded by extending with `to_repr().as_ref().iter().rev()` of `x.c1()`, `x.c0()`, `y.c1()`, `y.c0()`; `definition_exponent() -> Vec<u64>`, `let Some(p) = BigUint::parse_bytes(Fq::MODULUS.trim_start_matches("0x").as_bytes(), 16) else { panic!("the base field modulus parses") }; let Some(r) = BigUint::parse_bytes(Fr::MODULUS.trim_start_matches("0x").as_bytes(), 16) else { panic!("the group order parses") }; ((p.pow(12) - BigUint::from(1u32)) / r).to_u64_digits()`; `definition_value(g1: G1Affine, g2: G2Affine) -> Fq12`, `Bn256::multi_miller_loop(&[(&g1, &g2)]).pow_vartime(definition_exponent())`, the identifier's definition of the pairing value; `definition_value_power(g1: G1Affine, g2: G2Affine, exponent: u64) -> Fq12`, `definition_value(g1, g2).pow_vartime([exponent])`; `definition_encoding() -> Vec<u8>`, the twelve coefficients of `definition_value(eip_196_generator(), eip_197_generator())` read through the `c0()`, `c1()`, and `c2()` accessors in the tower order `c0.c0.c0`, `c0.c0.c1`, `c0.c1.c0`, `c0.c1.c1`, `c0.c2.c0`, `c0.c2.c1`, `c1.c0.c0`, `c1.c0.c1`, `c1.c1.c0`, `c1.c1.c1`, `c1.c2.c0`, `c1.c2.c1`, each `to_repr()` reversed to 32 big-endian bytes and appended; `eip_196_doubled_generator() -> G1Affine`, `(eip_196_generator().to_curve() + eip_196_generator().to_curve()).to_affine()`
    * `[ ]`   No mock function: the concrete is built as a real instance and owns no free function

  * `[ ]`   `adapters/pairing/src/bn254_halo2curves/test.rs`
    * `[ ]`   Module-level `#![allow(clippy::expect_used)]`; imports from `super::provides` the concrete, its owned types, and the mock's builders, constants, and helpers each block uses; from `crate::factory::provides` the params, payloads, errors, trait names, and builders each block uses; and the `halo2curves`, `num_bigint::BigUint`, and `zeroize::Zeroize` names each block uses
    * `[ ]`   Compile-time assertion over `Bn254Halo2curvesPairing::DECLARATION`
      * `[ ]`   Contract: the inherent constant is `PairingDeclaration { curve: PairingCurve::Bn254, verifier_group_arithmetic: VerifierGroupArithmetic::FirstGroupOnly, precompile_encoding: PrecompileEncoding::Eip196Eip197, target_group_encoding: TargetGroupEncodingIdentifier::Bn254V1, adapter_version: 1, interface_version: PAIRING_INTERFACE_VERSION }`, readable before any instance exists
      * `[ ]`   Arrange: the type alone
      * `[ ]`   Act: a module-level `const _: () = assert!(…);` reading `Bn254Halo2curvesPairing::DECLARATION`, each enum field tested with `matches!` and each version with `==`
      * `[ ]`   Assert: the module compiles only if every field holds
    * `[ ]`   Compile-time assertion over `<Bn254Halo2curvesPairing as IPairingAdapter>::DECLARATION`
      * `[ ]`   Contract: the trait constant carries the same fields as the inherent constant
      * `[ ]`   Arrange: the type alone
      * `[ ]`   Act: a module-level `const _: () = assert!(…);` reading the trait constant, each enum field tested with `matches!` and each version with `==`
      * `[ ]`   Assert: the module compiles only if every field holds
    * `[ ]`   Compile-time assertion over `<Bn254Halo2curvesPairing as IPairingAdapter>::CONCRETE`
      * `[ ]`   Contract: the trait constant is `PairingConcrete::Bn254Halo2curves`
      * `[ ]`   Arrange: the type alone
      * `[ ]`   Act: a module-level `const _: () = assert!(…);` reading the trait constant under `matches!`
      * `[ ]`   Assert: the module compiles only if the constant is `PairingConcrete::Bn254Halo2curves`
    * `[ ]`   `bn254_halo2curves_scalar_is_zeroize_on_drop`
      * `[ ]`   Contract: `Bn254Halo2curvesScalar` implements `ZeroizeOnDrop`, the marker the sampling bound requires
      * `[ ]`   Collaborators: none
      * `[ ]`   Arrange: the type alone
      * `[ ]`   Act: `requires_zeroize_on_drop::<Bn254Halo2curvesScalar>()`
      * `[ ]`   Assert: the block compiles only if the type implements `ZeroizeOnDrop`
    * `[ ]`   `bn254_halo2curves_scalar_zeroize_clears_its_value`
      * `[ ]`   Contract: `Zeroize::zeroize` on a living scalar → its `value` is zero
      * `[ ]`   Collaborators: none; fixture `build_bn254_halo2curves_scalar`
      * `[ ]`   Arrange: `build_bn254_halo2curves_scalar` with the value override `Fr::from(5u64)`, so a scalar left unchanged differs from the expected zero
      * `[ ]`   Act: `scalar.zeroize()`
      * `[ ]`   Assert: `scalar.value` equals `Fr::ZERO`
    * `[ ]`   `bn254_halo2curves_g1_zeroize_clears_its_value`
      * `[ ]`   Contract: `Zeroize::zeroize` on a living first-group element → its `value` is the identity
      * `[ ]`   Collaborators: none; fixture `build_bn254_halo2curves_g1`
      * `[ ]`   Arrange: `build_bn254_halo2curves_g1` at its default generator, which differs from the identity
      * `[ ]`   Act: `g1.zeroize()`
      * `[ ]`   Assert: `g1.value` equals `G1Affine::identity()`
    * `[ ]`   `bn254_halo2curves_g2_zeroize_clears_its_value`
      * `[ ]`   Contract: `Zeroize::zeroize` on a living second-group element → its `value` is the identity
      * `[ ]`   Collaborators: none; fixture `build_bn254_halo2curves_g2`
      * `[ ]`   Arrange: `build_bn254_halo2curves_g2` at its default generator, which differs from the identity
      * `[ ]`   Act: `g2.zeroize()`
      * `[ ]`   Assert: `g2.value` equals `G2Affine::identity()`
    * `[ ]`   `bn254_halo2curves_gt_zeroize_clears_its_value`
      * `[ ]`   Contract: `Zeroize::zeroize` on a living target-group value → its `value` is the target group's identity
      * `[ ]`   Collaborators: none; fixture `build_bn254_halo2curves_gt`
      * `[ ]`   Arrange: `build_bn254_halo2curves_gt` at its default pairing of the generators, which differs from the identity
      * `[ ]`   Act: `gt.zeroize()`
      * `[ ]`   Assert: `gt.value` equals `Gt::identity()`
    * `[ ]`   `bn254_halo2curves_encoded_scalar_zeroize_clears_its_bytes`
      * `[ ]`   Contract: `Zeroize::zeroize` on a living encoded scalar → every byte is zero
      * `[ ]`   Collaborators: none; fixture `build_bn254_halo2curves_encoded_scalar`
      * `[ ]`   Arrange: `build_bn254_halo2curves_encoded_scalar` at its nonzero default
      * `[ ]`   Act: `encoded.zeroize()`
      * `[ ]`   Assert: the bytes of `encoded` equal `zero_bytes(32)` as a byte slice
    * `[ ]`   `bn254_halo2curves_encoded_gt_zeroize_clears_its_bytes`
      * `[ ]`   Contract: `Zeroize::zeroize` on a living encoded target-group value → every byte is zero
      * `[ ]`   Collaborators: none; fixture `build_bn254_halo2curves_encoded_gt`
      * `[ ]`   Arrange: `build_bn254_halo2curves_encoded_gt` at its nonzero default
      * `[ ]`   Act: `encoded.zeroize()`
      * `[ ]`   Assert: the bytes of `encoded` equal `zero_bytes(384)` as a byte slice
    * `[ ]`   `try_new_returns_the_halo2curves_pairing`
      * `[ ]`   Contract: any params → `Ok(Bn254Halo2curvesPairing)`
      * `[ ]`   Collaborators: none; `Bn254Halo2curvesPairingConstructorParams` by its production value
      * `[ ]`   Arrange: `Bn254Halo2curvesPairingConstructorParams` by its production value
      * `[ ]`   Act: `Bn254Halo2curvesPairing::try_new(Bn254Halo2curvesPairingConstructorParams)`
      * `[ ]`   Assert: `result.is_ok()` is true
    * `[ ]`   `uniform_bytes_length_is_twice_the_group_order_width`
      * `[ ]`   Contract: `UNIFORM_BYTES_LENGTH` is twice the byte width of the group order
      * `[ ]`   Collaborators: none
      * `[ ]`   Arrange: the type alone
      * `[ ]`   Act: reading `<Bn254Halo2curvesScalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH`
      * `[ ]`   Assert: the constant equals the literal 64 written in the assertion
    * `[ ]`   `g1_generator_returns_the_eip_196_generator`
      * `[ ]`   Contract: any call → `Ok(G1GeneratorSuccessReturn { point })` holding the first group's generator
      * `[ ]`   Collaborators: halo2curves, run for real as the vendor; fixture `build_bn254_halo2curves_pairing`; `G1GeneratorParams` and `G1GeneratorPayload` by their production values
      * `[ ]`   Arrange: `build_bn254_halo2curves_pairing()`
      * `[ ]`   Act: `g1_generator(G1GeneratorParams, G1GeneratorPayload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.point.value` equals `eip_196_generator()`
    * `[ ]`   `g2_generator_returns_the_eip_197_generator`
      * `[ ]`   Contract: any call → `Ok(G2GeneratorSuccessReturn { point })` holding the second group's generator
      * `[ ]`   Collaborators: halo2curves, run for real as the vendor; fixture `build_bn254_halo2curves_pairing`; `G2GeneratorParams` and `G2GeneratorPayload` by their production values
      * `[ ]`   Arrange: `build_bn254_halo2curves_pairing()`
      * `[ ]`   Act: `g2_generator(G2GeneratorParams, G2GeneratorPayload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.point.value` equals `eip_197_generator()`
    * `[ ]`   `add_g1_of_a_point_and_its_negation_is_the_identity`
      * `[ ]`   Contract: `payload.left` and `payload.right` → `Ok(AddG1SuccessReturn { sum })` holding their sum
      * `[ ]`   Collaborators: halo2curves' projective addition, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_add_g1_payload`, and `build_bn254_halo2curves_g1`
      * `[ ]`   Arrange: `build_add_g1_payload` with the left override `build_bn254_halo2curves_g1` at its default generator and the right override `build_bn254_halo2curves_g1` with the value `eip_196_negated_generator()`, so a sum that returns either input differs from the identity
      * `[ ]`   Act: `add_g1(AddG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G1Affine::identity()`
    * `[ ]`   `add_g1_of_the_identity_and_a_point_is_the_point`
      * `[ ]`   Contract: `payload.left` and `payload.right` → `Ok(AddG1SuccessReturn { sum })` holding their sum
      * `[ ]`   Collaborators: halo2curves' projective addition, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_add_g1_payload`, and `build_bn254_halo2curves_g1`
      * `[ ]`   Arrange: `build_add_g1_payload` with the left override `build_bn254_halo2curves_g1` with the value `G1Affine::identity()` and the right override `build_bn254_halo2curves_g1` at its default generator, so a sum that returns the left input differs from the generator
      * `[ ]`   Act: `add_g1(AddG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `eip_196_generator()`
    * `[ ]`   `add_g2_of_a_point_and_its_negation_is_the_identity`
      * `[ ]`   Contract: `payload.left` and `payload.right` → `Ok(AddG2SuccessReturn { sum })` holding their sum
      * `[ ]`   Collaborators: halo2curves' projective addition, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_add_g2_payload`, and `build_bn254_halo2curves_g2`
      * `[ ]`   Arrange: `build_add_g2_payload` with the left override `build_bn254_halo2curves_g2` at its default generator and the right override `build_bn254_halo2curves_g2` with the value `-eip_197_generator()`, so a sum that returns either input differs from the identity
      * `[ ]`   Act: `add_g2(AddG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G2Affine::identity()`
    * `[ ]`   `add_g2_of_the_identity_and_a_point_is_the_point`
      * `[ ]`   Contract: `payload.left` and `payload.right` → `Ok(AddG2SuccessReturn { sum })` holding their sum
      * `[ ]`   Collaborators: halo2curves' projective addition, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_add_g2_payload`, and `build_bn254_halo2curves_g2`
      * `[ ]`   Arrange: `build_add_g2_payload` with the left override `build_bn254_halo2curves_g2` with the value `G2Affine::identity()` and the right override `build_bn254_halo2curves_g2` at its default generator, so a sum that returns the left input differs from the generator
      * `[ ]`   Act: `add_g2(AddG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `eip_197_generator()`
    * `[ ]`   `mul_g1_by_one_is_the_point`
      * `[ ]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG1SuccessReturn { product })` holding the point multiplied by the scalar
      * `[ ]`   Collaborators: halo2curves' projective scalar multiplication, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_mul_g1_payload`, `build_bn254_halo2curves_g1`, and `build_bn254_halo2curves_scalar`
      * `[ ]`   Arrange: `build_mul_g1_payload` with the point override `build_bn254_halo2curves_g1` at its default generator and the scalar override `build_bn254_halo2curves_scalar` with the value `Fr::from(1u64)`
      * `[ ]`   Act: `mul_g1(MulG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `eip_196_generator()`
    * `[ ]`   `mul_g1_by_zero_is_the_identity`
      * `[ ]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG1SuccessReturn { product })` holding the point multiplied by the scalar
      * `[ ]`   Collaborators: halo2curves' projective scalar multiplication, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_mul_g1_payload`, `build_bn254_halo2curves_g1`, and `build_bn254_halo2curves_scalar`
      * `[ ]`   Arrange: `build_mul_g1_payload` with the point override `build_bn254_halo2curves_g1` at its default generator and the scalar override `build_bn254_halo2curves_scalar` with the value `Fr::ZERO`, so a product that returns the point differs from the identity
      * `[ ]`   Act: `mul_g1(MulG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `G1Affine::identity()`
    * `[ ]`   `mul_g1_by_the_group_order_minus_one_is_the_negated_generator`
      * `[ ]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG1SuccessReturn { product })` holding the point multiplied by the scalar
      * `[ ]`   Collaborators: halo2curves' projective scalar multiplication, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_mul_g1_payload`, `build_bn254_halo2curves_g1`, and `build_bn254_halo2curves_scalar`
      * `[ ]`   Arrange: `build_mul_g1_payload` with the point override `build_bn254_halo2curves_g1` at its default generator and the scalar override `build_bn254_halo2curves_scalar` with the value `scalar_value(GROUP_ORDER_MINUS_ONE_HEX)`, so a product that ignores the scalar differs from the negated generator
      * `[ ]`   Act: `mul_g1(MulG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `eip_196_negated_generator()`
    * `[ ]`   `mul_g2_by_one_is_the_point`
      * `[ ]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG2SuccessReturn { product })` holding the point multiplied by the scalar
      * `[ ]`   Collaborators: halo2curves' projective scalar multiplication, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_mul_g2_payload`, `build_bn254_halo2curves_g2`, and `build_bn254_halo2curves_scalar`
      * `[ ]`   Arrange: `build_mul_g2_payload` with the point override `build_bn254_halo2curves_g2` at its default generator and the scalar override `build_bn254_halo2curves_scalar` with the value `Fr::from(1u64)`
      * `[ ]`   Act: `mul_g2(MulG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `eip_197_generator()`
    * `[ ]`   `mul_g2_by_zero_is_the_identity`
      * `[ ]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG2SuccessReturn { product })` holding the point multiplied by the scalar
      * `[ ]`   Collaborators: halo2curves' projective scalar multiplication, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_mul_g2_payload`, `build_bn254_halo2curves_g2`, and `build_bn254_halo2curves_scalar`
      * `[ ]`   Arrange: `build_mul_g2_payload` with the point override `build_bn254_halo2curves_g2` at its default generator and the scalar override `build_bn254_halo2curves_scalar` with the value `Fr::ZERO`, so a product that returns the point differs from the identity
      * `[ ]`   Act: `mul_g2(MulG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `G2Affine::identity()`
    * `[ ]`   `mul_g2_by_the_group_order_minus_one_is_the_negated_point`
      * `[ ]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG2SuccessReturn { product })` holding the point multiplied by the scalar
      * `[ ]`   Collaborators: halo2curves' projective scalar multiplication, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_mul_g2_payload`, `build_bn254_halo2curves_g2`, and `build_bn254_halo2curves_scalar`
      * `[ ]`   Arrange: `build_mul_g2_payload` with the point override `build_bn254_halo2curves_g2` at its default generator and the scalar override `build_bn254_halo2curves_scalar` with the value `scalar_value(GROUP_ORDER_MINUS_ONE_HEX)`, so a product that ignores the scalar differs from the negated generator
      * `[ ]`   Act: `mul_g2(MulG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `-eip_197_generator()`
    * `[ ]`   `msm_g1_of_no_terms_is_the_identity`
      * `[ ]`   Contract: an empty `payload.terms` → `Ok(MsmG1SuccessReturn { sum })` holding the identity
      * `[ ]`   Collaborators: halo2curves' `msm_best`, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing` and `build_msm_g1_payload`
      * `[ ]`   Arrange: `build_msm_g1_payload` at its default of no terms
      * `[ ]`   Act: `msm_g1(MsmG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G1Affine::identity()`
    * `[ ]`   `msm_g1_pairs_each_base_with_its_own_scalar`
      * `[ ]`   Contract: `payload.terms` → `Ok(MsmG1SuccessReturn { sum })` holding the sum of each base multiplied by its own scalar
      * `[ ]`   Collaborators: halo2curves' `msm_best`, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_msm_g1_payload`, `build_msm_g1_term`, `build_bn254_halo2curves_g1`, and `build_bn254_halo2curves_scalar`
      * `[ ]`   Arrange: `build_msm_g1_payload` with the terms override holding two `build_msm_g1_term` values: the generator with the scalar `Fr::from(1u64)`, and the base `eip_196_negated_generator()` with the scalar `Fr::ZERO`, so bases and scalars exchanged between terms yield the negated generator
      * `[ ]`   Act: `msm_g1(MsmG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `eip_196_generator()`
    * `[ ]`   `msm_g1_sums_its_terms`
      * `[ ]`   Contract: `payload.terms` → `Ok(MsmG1SuccessReturn { sum })` holding the sum of each base multiplied by its own scalar
      * `[ ]`   Collaborators: halo2curves' `msm_best`, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_msm_g1_payload`, `build_msm_g1_term`, `build_bn254_halo2curves_g1`, and `build_bn254_halo2curves_scalar`
      * `[ ]`   Arrange: `build_msm_g1_payload` with the terms override holding two `build_msm_g1_term` values: the generator with the scalar `Fr::from(1u64)`, and the base `eip_196_negated_generator()` with the scalar `Fr::from(1u64)`, so a sum over the first term alone differs from the identity
      * `[ ]`   Act: `msm_g1(MsmG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G1Affine::identity()`
    * `[ ]`   `msm_g2_of_no_terms_is_the_identity`
      * `[ ]`   Contract: an empty `payload.terms` → `Ok(MsmG2SuccessReturn { sum })` holding the identity
      * `[ ]`   Collaborators: halo2curves' `msm_best`, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing` and `build_msm_g2_payload`
      * `[ ]`   Arrange: `build_msm_g2_payload` at its default of no terms
      * `[ ]`   Act: `msm_g2(MsmG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G2Affine::identity()`
    * `[ ]`   `msm_g2_pairs_each_base_with_its_own_scalar`
      * `[ ]`   Contract: `payload.terms` → `Ok(MsmG2SuccessReturn { sum })` holding the sum of each base multiplied by its own scalar
      * `[ ]`   Collaborators: halo2curves' `msm_best`, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_msm_g2_payload`, `build_msm_g2_term`, `build_bn254_halo2curves_g2`, and `build_bn254_halo2curves_scalar`
      * `[ ]`   Arrange: `build_msm_g2_payload` with the terms override holding two `build_msm_g2_term` values: the generator with the scalar `Fr::from(1u64)`, and the base `-eip_197_generator()` with the scalar `Fr::ZERO`, so bases and scalars exchanged between terms yield the negated generator
      * `[ ]`   Act: `msm_g2(MsmG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `eip_197_generator()`
    * `[ ]`   `msm_g2_sums_its_terms`
      * `[ ]`   Contract: `payload.terms` → `Ok(MsmG2SuccessReturn { sum })` holding the sum of each base multiplied by its own scalar
      * `[ ]`   Collaborators: halo2curves' `msm_best`, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_msm_g2_payload`, `build_msm_g2_term`, `build_bn254_halo2curves_g2`, and `build_bn254_halo2curves_scalar`
      * `[ ]`   Arrange: `build_msm_g2_payload` with the terms override holding two `build_msm_g2_term` values: the generator with the scalar `Fr::from(1u64)`, and the base `-eip_197_generator()` with the scalar `Fr::from(1u64)`, so a sum over the first term alone differs from the identity
      * `[ ]`   Act: `msm_g2(MsmG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G2Affine::identity()`
    * `[ ]`   `pairing_product_is_one_of_no_terms_is_true`
      * `[ ]`   Contract: an empty `payload.terms` → `Ok(PairingProductIsOneSuccessReturn { is_one })` with `is_one` true
      * `[ ]`   Collaborators: halo2curves' multi-Miller loop and final exponentiation, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing` and `build_pairing_product_is_one_payload`
      * `[ ]`   Arrange: `build_pairing_product_is_one_payload` at its default of no terms
      * `[ ]`   Act: `pairing_product_is_one(PairingProductIsOneParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_one` is true
    * `[ ]`   `pairing_product_is_one_of_a_pairing_and_its_first_group_negation_is_true`
      * `[ ]`   Contract: `payload.terms` whose pairings multiply to the target group's identity → `Ok(PairingProductIsOneSuccessReturn { is_one })` with `is_one` true
      * `[ ]`   Collaborators: halo2curves' multi-Miller loop and final exponentiation, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_pairing_product_is_one_payload`, `build_pairing_product_term`, and `build_bn254_halo2curves_g1`
      * `[ ]`   Arrange: `build_pairing_product_is_one_payload` with the terms override holding two `build_pairing_product_term` values: the generators, and the first-group override `build_bn254_halo2curves_g1` with the value `eip_196_negated_generator()` over the second-group generator
      * `[ ]`   Act: `pairing_product_is_one(PairingProductIsOneParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_one` is true
    * `[ ]`   `pairing_product_is_one_of_the_generators_is_false`
      * `[ ]`   Contract: `payload.terms` whose pairings multiply to a value other than the target group's identity → `Ok(PairingProductIsOneSuccessReturn { is_one })` with `is_one` false
      * `[ ]`   Collaborators: halo2curves' multi-Miller loop and final exponentiation, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_pairing_product_is_one_payload`, and `build_pairing_product_term`
      * `[ ]`   Arrange: `build_pairing_product_is_one_payload` with the terms override holding one `build_pairing_product_term` at its default generators
      * `[ ]`   Act: `pairing_product_is_one(PairingProductIsOneParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_one` is false
    * `[ ]`   `decode_g1_rejects_a_wrong_length`
      * `[ ]`   Contract: `payload.len() != 64` → `Err(DecodeG1ErrorReturn::WrongLength { expected: 64, actual: payload.len() })`
      * `[ ]`   Collaborators: none called; fixture `build_bn254_halo2curves_pairing`; the untrusted bytes from `zero_bytes`
      * `[ ]`   Arrange: the payload `zero_bytes(63)`, which differs from the required length
      * `[ ]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(DecodeG1ErrorReturn::WrongLength { expected: 64, actual: 63 })`, the whole expected error
    * `[ ]`   `decode_g1_rejects_a_non_canonical_coordinate`
      * `[ ]`   Contract: a 64-byte payload whose first half is at least the base field modulus → `Err(DecodeG1ErrorReturn::NonCanonicalCoordinate)`
      * `[ ]`   Collaborators: halo2curves' `Fq::from_repr`, run for real as the vendor; fixture `build_bn254_halo2curves_pairing`; the untrusted bytes from `vector_bytes(G1_X_AT_MODULUS_HEX)`
      * `[ ]`   Arrange: the payload `vector_bytes(G1_X_AT_MODULUS_HEX)`, whose coordinates would otherwise fail the curve check, so a decoder that checks the curve first returns a different error
      * `[ ]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(DecodeG1ErrorReturn::NonCanonicalCoordinate)`
    * `[ ]`   `decode_g1_decodes_the_identity_from_zero_bytes`
      * `[ ]`   Contract: a 64-byte payload with both coordinates zero → `Ok(DecodeG1SuccessReturn { point })` holding the first group's identity
      * `[ ]`   Collaborators: halo2curves, run for real as the vendor; fixture `build_bn254_halo2curves_pairing`; the untrusted bytes from `zero_bytes`
      * `[ ]`   Arrange: the payload `zero_bytes(64)`
      * `[ ]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[ ]`   Assert: the success arm is extracted with `expect`; `success.point.value` equals `G1Affine::identity()`
    * `[ ]`   `decode_g1_rejects_a_point_off_the_curve`
      * `[ ]`   Contract: a canonical 64-byte payload that satisfies no curve equation → `Err(DecodeG1ErrorReturn::NotOnCurve)`
      * `[ ]`   Collaborators: halo2curves' `from_xy`, run for real as the vendor; fixture `build_bn254_halo2curves_pairing`; the untrusted bytes from `vector_bytes(G1_OFF_CURVE_HEX)`
      * `[ ]`   Arrange: the payload `vector_bytes(G1_OFF_CURVE_HEX)`
      * `[ ]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(DecodeG1ErrorReturn::NotOnCurve)`
    * `[ ]`   `decode_g1_decodes_the_eip_196_generator`
      * `[ ]`   Contract: a canonical 64-byte payload on the curve → `Ok(DecodeG1SuccessReturn { point })` holding that point
      * `[ ]`   Collaborators: halo2curves, run for real as the vendor; fixture `build_bn254_halo2curves_pairing`; the untrusted bytes from `vector_bytes(G1_GENERATOR_HEX)`
      * `[ ]`   Arrange: the payload `vector_bytes(G1_GENERATOR_HEX)`
      * `[ ]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[ ]`   Assert: the success arm is extracted with `expect`; `success.point.value` equals `eip_196_generator()`
    * `[ ]`   `decode_g2_rejects_a_wrong_length`
      * `[ ]`   Contract: `payload.len() != 128` → `Err(DecodeG2ErrorReturn::WrongLength { expected: 128, actual: payload.len() })`
      * `[ ]`   Collaborators: none called; fixture `build_bn254_halo2curves_pairing`; the untrusted bytes from `zero_bytes`
      * `[ ]`   Arrange: the payload `zero_bytes(127)`, which differs from the required length
      * `[ ]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(DecodeG2ErrorReturn::WrongLength { expected: 128, actual: 127 })`, the whole expected error
    * `[ ]`   `decode_g2_rejects_a_non_canonical_coordinate`
      * `[ ]`   Contract: a 128-byte payload whose first coordinate is at least the base field modulus → `Err(DecodeG2ErrorReturn::NonCanonicalCoordinate)`
      * `[ ]`   Collaborators: halo2curves' `Fq::from_repr`, run for real as the vendor; fixture `build_bn254_halo2curves_pairing`; the untrusted bytes from `vector_bytes(G2_X_C1_AT_MODULUS_HEX)`
      * `[ ]`   Arrange: the payload `vector_bytes(G2_X_C1_AT_MODULUS_HEX)`, whose coordinates would otherwise fail the curve check, so a decoder that checks the curve first returns a different error
      * `[ ]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(DecodeG2ErrorReturn::NonCanonicalCoordinate)`
    * `[ ]`   `decode_g2_decodes_the_identity_from_zero_bytes`
      * `[ ]`   Contract: a 128-byte payload with every coordinate zero → `Ok(DecodeG2SuccessReturn { point })` holding the second group's identity
      * `[ ]`   Collaborators: halo2curves, run for real as the vendor; fixture `build_bn254_halo2curves_pairing`; the untrusted bytes from `zero_bytes`
      * `[ ]`   Arrange: the payload `zero_bytes(128)`
      * `[ ]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[ ]`   Assert: the success arm is extracted with `expect`; `success.point.value` equals `G2Affine::identity()`
    * `[ ]`   `decode_g2_rejects_a_point_off_the_curve`
      * `[ ]`   Contract: a canonical 128-byte payload that satisfies no curve equation → `Err(DecodeG2ErrorReturn::NotOnCurve)`
      * `[ ]`   Collaborators: halo2curves' `from_xy`, run for real as the vendor; fixture `build_bn254_halo2curves_pairing`; the untrusted bytes from `vector_bytes(G2_GENERATOR_OFF_CURVE_HEX)`
      * `[ ]`   Arrange: the payload `vector_bytes(G2_GENERATOR_OFF_CURVE_HEX)`
      * `[ ]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(DecodeG2ErrorReturn::NotOnCurve)`
    * `[ ]`   `decode_g2_rejects_a_point_outside_the_subgroup`
      * `[ ]`   Contract: a canonical 128-byte payload on the curve and outside the prime-order subgroup → `Err(DecodeG2ErrorReturn::NotInSubgroup)`
      * `[ ]`   Collaborators: halo2curves' torsion check, run for real as the vendor; fixture `build_bn254_halo2curves_pairing`; the untrusted bytes from `g2_outside_subgroup_bytes()`
      * `[ ]`   Arrange: the payload `g2_outside_subgroup_bytes()`
      * `[ ]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(DecodeG2ErrorReturn::NotInSubgroup)`
    * `[ ]`   `decode_g2_decodes_the_eip_197_generator`
      * `[ ]`   Contract: a canonical 128-byte payload on the curve and in the subgroup → `Ok(DecodeG2SuccessReturn { point })` holding that point
      * `[ ]`   Collaborators: halo2curves, run for real as the vendor; fixture `build_bn254_halo2curves_pairing`; the untrusted bytes from `vector_bytes(G2_GENERATOR_HEX)`
      * `[ ]`   Arrange: the payload `vector_bytes(G2_GENERATOR_HEX)`
      * `[ ]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[ ]`   Assert: the success arm is extracted with `expect`; `success.point.value` equals `eip_197_generator()`
    * `[ ]`   `decode_scalar_rejects_a_wrong_length`
      * `[ ]`   Contract: `payload.len() != 32` → `Err(DecodeScalarErrorReturn::WrongLength { expected: 32, actual: payload.len() })`
      * `[ ]`   Collaborators: none called; fixture `build_bn254_halo2curves_pairing`; the untrusted bytes from `zero_bytes`
      * `[ ]`   Arrange: the payload `zero_bytes(31)`, which differs from the required length
      * `[ ]`   Act: `decode_scalar(DecodeScalarParams, &payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(DecodeScalarErrorReturn::WrongLength { expected: 32, actual: 31 })`, the whole expected error
    * `[ ]`   `decode_scalar_rejects_the_group_order`
      * `[ ]`   Contract: a 32-byte payload at least the group order → `Err(DecodeScalarErrorReturn::NonCanonical)`
      * `[ ]`   Collaborators: halo2curves' `Fr::from_repr`, run for real as the vendor; fixture `build_bn254_halo2curves_pairing`; the untrusted bytes from `vector_bytes(GROUP_ORDER_HEX)`
      * `[ ]`   Arrange: the payload `vector_bytes(GROUP_ORDER_HEX)`
      * `[ ]`   Act: `decode_scalar(DecodeScalarParams, &payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(DecodeScalarErrorReturn::NonCanonical)`
    * `[ ]`   `decode_scalar_decodes_the_largest_canonical_scalar`
      * `[ ]`   Contract: a canonical 32-byte payload → `Ok(DecodeScalarSuccessReturn { scalar })` holding that scalar
      * `[ ]`   Collaborators: halo2curves' `Fr::from_repr`, run for real as the vendor; fixture `build_bn254_halo2curves_pairing`; the untrusted bytes from `vector_bytes(GROUP_ORDER_MINUS_ONE_HEX)`
      * `[ ]`   Arrange: the payload `vector_bytes(GROUP_ORDER_MINUS_ONE_HEX)`
      * `[ ]`   Act: `decode_scalar(DecodeScalarParams, &payload)`
      * `[ ]`   Assert: the success arm is extracted with `expect`; `success.scalar.value` equals `-Fr::from(1u64)`
    * `[ ]`   `encode_g1_writes_the_eip_196_generator`
      * `[ ]`   Contract: a first-group point → `Ok(EncodeG1SuccessReturn { bytes })` holding `x` then `y`, each 32 bytes big-endian, in this concrete's `Bn254Halo2curvesEncodedG1`
      * `[ ]`   Collaborators: halo2curves' coordinate encoding, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_encode_g1_payload`, and `build_bn254_halo2curves_g1`
      * `[ ]`   Arrange: `build_encode_g1_payload` with the point override `build_bn254_halo2curves_g1` at its default generator
      * `[ ]`   Act: `encode_g1(EncodeG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.bytes` binds to a `Bn254Halo2curvesEncodedG1` typed binding; its bytes equal `vector_bytes(G1_GENERATOR_HEX)` as a byte slice
    * `[ ]`   `encode_g1_writes_the_identity_as_zero_bytes`
      * `[ ]`   Contract: the first group's identity → `Ok(EncodeG1SuccessReturn { bytes })` holding 64 zero bytes
      * `[ ]`   Collaborators: halo2curves, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_encode_g1_payload`, and `build_bn254_halo2curves_g1`
      * `[ ]`   Arrange: `build_encode_g1_payload` with the point override `build_bn254_halo2curves_g1` with the value `G1Affine::identity()`
      * `[ ]`   Act: `encode_g1(EncodeG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; the bytes of `success.bytes` equal `zero_bytes(64)` as a byte slice
    * `[ ]`   `encode_g2_writes_the_eip_197_generator`
      * `[ ]`   Contract: a second-group point → `Ok(EncodeG2SuccessReturn { bytes })` holding `x.c1`, `x.c0`, `y.c1`, `y.c0`, each 32 bytes big-endian, in this concrete's `Bn254Halo2curvesEncodedG2`
      * `[ ]`   Collaborators: halo2curves' coordinate encoding, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_encode_g2_payload`, and `build_bn254_halo2curves_g2`
      * `[ ]`   Arrange: `build_encode_g2_payload` with the point override `build_bn254_halo2curves_g2` at its default generator
      * `[ ]`   Act: `encode_g2(EncodeG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.bytes` binds to a `Bn254Halo2curvesEncodedG2` typed binding; its bytes equal `vector_bytes(G2_GENERATOR_HEX)` as a byte slice
    * `[ ]`   `encode_g2_writes_the_identity_as_zero_bytes`
      * `[ ]`   Contract: the second group's identity → `Ok(EncodeG2SuccessReturn { bytes })` holding 128 zero bytes
      * `[ ]`   Collaborators: halo2curves, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_encode_g2_payload`, and `build_bn254_halo2curves_g2`
      * `[ ]`   Arrange: `build_encode_g2_payload` with the point override `build_bn254_halo2curves_g2` with the value `G2Affine::identity()`
      * `[ ]`   Act: `encode_g2(EncodeG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; the bytes of `success.bytes` equal `zero_bytes(128)` as a byte slice
    * `[ ]`   `encode_scalar_writes_the_largest_canonical_scalar`
      * `[ ]`   Contract: a scalar → `Ok(EncodeScalarSuccessReturn { bytes })` holding its 32 big-endian bytes in this concrete's `Bn254Halo2curvesEncodedScalar`, inside a `Secret`
      * `[ ]`   Collaborators: halo2curves' `to_repr`, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_encode_scalar_payload`, and `build_bn254_halo2curves_scalar`
      * `[ ]`   Arrange: `build_encode_scalar_payload` with the scalar override `build_bn254_halo2curves_scalar` with the value `-Fr::from(1u64)`
      * `[ ]`   Act: `encode_scalar(EncodeScalarParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.bytes.expose()` binds to a `Bn254Halo2curvesEncodedScalar` typed binding; its bytes equal `vector_bytes(GROUP_ORDER_MINUS_ONE_HEX)` as a byte slice
    * `[ ]`   `sample_from_uniform_bytes_rejects_a_wrong_length`
      * `[ ]`   Contract: `payload.uniform.expose().len() != 64` → `Err(SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual })`
      * `[ ]`   Collaborators: `Secret::expose`, real; fixtures `build_sample_uniform_scalar_payload` and `build_secret`
      * `[ ]`   Arrange: `build_sample_uniform_scalar_payload` with the uniform override `build_secret` holding `zero_bytes(63)`
      * `[ ]`   Act: `Bn254Halo2curvesScalar::sample_from_uniform_bytes(SampleUniformScalarParams, payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual: 63 })`, the whole expected error
    * `[ ]`   `sample_from_uniform_bytes_reads_the_input_as_a_big_endian_integer`
      * `[ ]`   Contract: a 64-byte uniform input → `Ok(SampleUniformScalarSuccessReturn { scalar })` holding the input read as one big-endian integer reduced modulo the group order, inside a `Secret`
      * `[ ]`   Collaborators: `Secret::expose` and halo2curves' wide reduction, real; fixtures `build_sample_uniform_scalar_payload` and `build_secret`
      * `[ ]`   Arrange: `build_sample_uniform_scalar_payload` with the uniform override `build_secret` holding `vector_bytes(UNIFORM_FIVE_HEX)`, whose last byte is five, so an input read as little-endian yields a different scalar
      * `[ ]`   Act: `Bn254Halo2curvesScalar::sample_from_uniform_bytes(SampleUniformScalarParams, payload)`
      * `[ ]`   Assert: the success arm is extracted with `expect`; `success.scalar.expose().value` equals `Fr::from(5u64)`
    * `[ ]`   `sample_from_uniform_bytes_reduces_the_group_order_to_zero`
      * `[ ]`   Contract: a 64-byte uniform input → `Ok(SampleUniformScalarSuccessReturn { scalar })` holding the input reduced modulo the group order, inside a `Secret`
      * `[ ]`   Collaborators: `Secret::expose` and halo2curves' wide reduction, real; fixtures `build_sample_uniform_scalar_payload` and `build_secret`
      * `[ ]`   Arrange: `build_sample_uniform_scalar_payload` with the uniform override `build_secret` holding `vector_bytes(UNIFORM_GROUP_ORDER_HEX)`, whose reduction is zero and so differs from the input
      * `[ ]`   Act: `Bn254Halo2curvesScalar::sample_from_uniform_bytes(SampleUniformScalarParams, payload)`
      * `[ ]`   Assert: the success arm is extracted with `expect`; `success.scalar.expose().value` equals `Fr::ZERO`
    * `[ ]`   `add_scalar_of_two_and_three_is_five`
      * `[ ]`   Contract: `payload.left` and `payload.right` → `Ok(AddScalarSuccessReturn { sum })` holding their sum modulo the group order
      * `[ ]`   Collaborators: halo2curves' `Fr` addition, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_add_scalar_payload`, and `build_bn254_halo2curves_scalar`
      * `[ ]`   Arrange: `build_add_scalar_payload` with the left override `build_bn254_halo2curves_scalar` with the value `Fr::from(2u64)` and the right override with the value `Fr::from(3u64)`
      * `[ ]`   Act: `add_scalar(AddScalarParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `Fr::from(5u64)`
    * `[ ]`   `add_scalar_reduces_modulo_the_group_order`
      * `[ ]`   Contract: `payload.left` and `payload.right` → `Ok(AddScalarSuccessReturn { sum })` holding their sum modulo the group order
      * `[ ]`   Collaborators: halo2curves' `Fr` addition, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_add_scalar_payload`, and `build_bn254_halo2curves_scalar`
      * `[ ]`   Arrange: `build_add_scalar_payload` with the left override `build_bn254_halo2curves_scalar` with the value `scalar_value(GROUP_ORDER_MINUS_ONE_HEX)` and the right override with the value `Fr::from(2u64)`, whose integer sum exceeds the group order
      * `[ ]`   Act: `add_scalar(AddScalarParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `Fr::from(1u64)`
    * `[ ]`   `mul_scalar_of_two_and_three_is_six`
      * `[ ]`   Contract: `payload.left` and `payload.right` → `Ok(MulScalarSuccessReturn { product })` holding their product modulo the group order
      * `[ ]`   Collaborators: halo2curves' `Fr` multiplication, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_mul_scalar_payload`, and `build_bn254_halo2curves_scalar`
      * `[ ]`   Arrange: `build_mul_scalar_payload` with the left override `build_bn254_halo2curves_scalar` with the value `Fr::from(2u64)` and the right override with the value `Fr::from(3u64)`
      * `[ ]`   Act: `mul_scalar(MulScalarParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `Fr::from(6u64)`
    * `[ ]`   `mul_scalar_reduces_modulo_the_group_order`
      * `[ ]`   Contract: `payload.left` and `payload.right` → `Ok(MulScalarSuccessReturn { product })` holding their product modulo the group order
      * `[ ]`   Collaborators: halo2curves' `Fr` multiplication, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_mul_scalar_payload`, and `build_bn254_halo2curves_scalar`
      * `[ ]`   Arrange: `build_mul_scalar_payload` with the left override `build_bn254_halo2curves_scalar` with the value `scalar_value(GROUP_ORDER_MINUS_ONE_HEX)` and the right override with the value `Fr::from(2u64)`, whose integer product exceeds the group order
      * `[ ]`   Act: `mul_scalar(MulScalarParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `scalar_value(GROUP_ORDER_MINUS_TWO_HEX)`
    * `[ ]`   `neg_scalar_of_one_is_the_group_order_minus_one`
      * `[ ]`   Contract: `payload.scalar` → `Ok(NegScalarSuccessReturn { negation })` holding the group order minus the scalar
      * `[ ]`   Collaborators: halo2curves' `Fr` negation, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_neg_scalar_payload`, and `build_bn254_halo2curves_scalar`
      * `[ ]`   Arrange: `build_neg_scalar_payload` with the scalar override `build_bn254_halo2curves_scalar` with the value `Fr::from(1u64)`
      * `[ ]`   Act: `neg_scalar(NegScalarParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.negation.value` equals `scalar_value(GROUP_ORDER_MINUS_ONE_HEX)`
    * `[ ]`   `neg_scalar_of_zero_is_zero`
      * `[ ]`   Contract: `payload.scalar` of zero → `Ok(NegScalarSuccessReturn { negation })` holding zero
      * `[ ]`   Collaborators: halo2curves' `Fr` negation, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_neg_scalar_payload`, and `build_bn254_halo2curves_scalar`
      * `[ ]`   Arrange: `build_neg_scalar_payload` with the scalar override `build_bn254_halo2curves_scalar` with the value `Fr::ZERO`
      * `[ ]`   Act: `neg_scalar(NegScalarParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.negation.value` equals `Fr::ZERO`
    * `[ ]`   `neg_g1_of_the_generator_is_the_negated_generator`
      * `[ ]`   Contract: `payload.point` → `Ok(NegG1SuccessReturn { negation })` holding `(x, p - y)` for a point `(x, y)`
      * `[ ]`   Collaborators: halo2curves' affine negation, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_neg_g1_payload`, and `build_bn254_halo2curves_g1`
      * `[ ]`   Arrange: `build_neg_g1_payload` with the point override `build_bn254_halo2curves_g1` at its default generator
      * `[ ]`   Act: `neg_g1(NegG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.negation.value` equals `eip_196_negated_generator()`
    * `[ ]`   `neg_g1_of_the_identity_is_the_identity`
      * `[ ]`   Contract: `payload.point` of the identity → `Ok(NegG1SuccessReturn { negation })` holding the identity
      * `[ ]`   Collaborators: halo2curves' affine negation, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_neg_g1_payload`, and `build_bn254_halo2curves_g1`
      * `[ ]`   Arrange: `build_neg_g1_payload` with the point override `build_bn254_halo2curves_g1` with the value `G1Affine::identity()`
      * `[ ]`   Act: `neg_g1(NegG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.negation.value` equals `G1Affine::identity()`
    * `[ ]`   `neg_g2_negates_the_y_coordinate_and_keeps_x`
      * `[ ]`   Contract: `payload.point` → `Ok(NegG2SuccessReturn { negation })` holding `(x, -y)` for a point `(x, y)`
      * `[ ]`   Collaborators: halo2curves' affine negation, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_neg_g2_payload`, and `build_bn254_halo2curves_g2`
      * `[ ]`   Arrange: `build_neg_g2_payload` with the point override `build_bn254_halo2curves_g2` at its default generator
      * `[ ]`   Act: `neg_g2(NegG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; the negation's `coordinates()` yields an `x` equal to the `x` of `eip_197_generator().coordinates()` and a `y` whose sum with the generator's `y` is zero under `is_zero()`
    * `[ ]`   `neg_g2_of_the_identity_is_the_identity`
      * `[ ]`   Contract: `payload.point` of the identity → `Ok(NegG2SuccessReturn { negation })` holding the identity
      * `[ ]`   Collaborators: halo2curves' affine negation, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_neg_g2_payload`, and `build_bn254_halo2curves_g2`
      * `[ ]`   Arrange: `build_neg_g2_payload` with the point override `build_bn254_halo2curves_g2` with the value `G2Affine::identity()`
      * `[ ]`   Act: `neg_g2(NegG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.negation.value` equals `G2Affine::identity()`
    * `[ ]`   `is_identity_g1_is_true_for_the_identity`
      * `[ ]`   Contract: `payload.point` → `Ok(IsIdentityG1SuccessReturn { is_identity })`, true exactly for the identity
      * `[ ]`   Collaborators: halo2curves, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_is_identity_g1_payload`, and `build_bn254_halo2curves_g1`
      * `[ ]`   Arrange: `build_is_identity_g1_payload` with the point override `build_bn254_halo2curves_g1` with the value `G1Affine::identity()`
      * `[ ]`   Act: `is_identity_g1(IsIdentityG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_identity` is true
    * `[ ]`   `is_identity_g1_is_false_for_the_generator`
      * `[ ]`   Contract: `payload.point` → `Ok(IsIdentityG1SuccessReturn { is_identity })`, true exactly for the identity
      * `[ ]`   Collaborators: halo2curves, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_is_identity_g1_payload`, and `build_bn254_halo2curves_g1`
      * `[ ]`   Arrange: `build_is_identity_g1_payload` with the point override `build_bn254_halo2curves_g1` at its default generator
      * `[ ]`   Act: `is_identity_g1(IsIdentityG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_identity` is false
    * `[ ]`   `is_identity_g2_is_true_for_the_identity`
      * `[ ]`   Contract: `payload.point` → `Ok(IsIdentityG2SuccessReturn { is_identity })`, true exactly for the identity
      * `[ ]`   Collaborators: halo2curves, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_is_identity_g2_payload`, and `build_bn254_halo2curves_g2`
      * `[ ]`   Arrange: `build_is_identity_g2_payload` with the point override `build_bn254_halo2curves_g2` with the value `G2Affine::identity()`
      * `[ ]`   Act: `is_identity_g2(IsIdentityG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_identity` is true
    * `[ ]`   `is_identity_g2_is_false_for_the_generator`
      * `[ ]`   Contract: `payload.point` → `Ok(IsIdentityG2SuccessReturn { is_identity })`, true exactly for the identity
      * `[ ]`   Collaborators: halo2curves, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_is_identity_g2_payload`, and `build_bn254_halo2curves_g2`
      * `[ ]`   Arrange: `build_is_identity_g2_payload` with the point override `build_bn254_halo2curves_g2` at its default generator
      * `[ ]`   Act: `is_identity_g2(IsIdentityG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_identity` is false
    * `[ ]`   `pairing_product_of_no_terms_is_the_target_group_identity`
      * `[ ]`   Contract: an empty `payload.terms` → `Ok(PairingProductSuccessReturn { product })` holding the target group's identity
      * `[ ]`   Collaborators: halo2curves' multi-Miller loop and final exponentiation, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing` and `build_pairing_product_payload`
      * `[ ]`   Arrange: `build_pairing_product_payload` at its default of no terms
      * `[ ]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `Gt::identity()`
    * `[ ]`   `pairing_product_of_the_generators_is_not_the_target_group_identity`
      * `[ ]`   Contract: a pairing of the generators → `Ok(PairingProductSuccessReturn { product })` holding a value other than the target group's identity
      * `[ ]`   Collaborators: halo2curves' multi-Miller loop and final exponentiation, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_pairing_product_payload`, and `build_pairing_product_term`
      * `[ ]`   Arrange: `build_pairing_product_payload` with the terms override holding one `build_pairing_product_term` at its default generators
      * `[ ]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` is not equal to `Gt::identity()`
    * `[ ]`   `pairing_product_is_bilinear`
      * `[ ]`   Contract: a term whose first-group element is twice the generator → `Ok(PairingProductSuccessReturn { product })` holding the generators' pairing raised to the power two
      * `[ ]`   Collaborators: halo2curves' multi-Miller loop and final exponentiation, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_pairing_product_payload`, `build_pairing_product_term`, and `build_bn254_halo2curves_g1`; the expectation is `definition_value_power`, the identifier's definition raised to a power in the fixtures
      * `[ ]`   Arrange: `build_pairing_product_payload` with the terms override holding one `build_pairing_product_term` with the first-group override `build_bn254_halo2curves_g1` with the value `eip_196_doubled_generator()` over the second-group generator
      * `[ ]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value.inner()` equals `&definition_value_power(eip_196_generator(), eip_197_generator(), 2)`
    * `[ ]`   `pairing_product_multiplies_its_terms`
      * `[ ]`   Contract: `payload.terms` → `Ok(PairingProductSuccessReturn { product })` holding the product of each term's pairing
      * `[ ]`   Collaborators: halo2curves' multi-Miller loop and final exponentiation, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_pairing_product_payload`, and `build_pairing_product_term`; the expectation is `definition_value_power`
      * `[ ]`   Arrange: `build_pairing_product_payload` with the terms override holding two `build_pairing_product_term` values at their default generators, so a product over one term differs from the expected square
      * `[ ]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value.inner()` equals `&definition_value_power(eip_196_generator(), eip_197_generator(), 2)`
    * `[ ]`   `pairing_product_of_a_pairing_and_its_first_group_negation_is_the_target_group_identity`
      * `[ ]`   Contract: `payload.terms` whose pairings multiply to the target group's identity → `Ok(PairingProductSuccessReturn { product })` holding the identity
      * `[ ]`   Collaborators: halo2curves' multi-Miller loop and final exponentiation, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_pairing_product_payload`, `build_pairing_product_term`, and `build_bn254_halo2curves_g1`
      * `[ ]`   Arrange: `build_pairing_product_payload` with the terms override holding two `build_pairing_product_term` values: the generators, and the first-group override `build_bn254_halo2curves_g1` with the value `eip_196_negated_generator()` over the second-group generator
      * `[ ]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `Gt::identity()`
    * `[ ]`   `pairing_product_of_the_generators_equals_the_definition`
      * `[ ]`   Contract: a pairing of the generators → `Ok(PairingProductSuccessReturn { product })` holding the Miller loop's value raised to the exact exponent `(p^12 - 1) / r`, the value `TargetGroupEncodingIdentifier::Bn254V1` defines, with no correction applied
      * `[ ]`   Collaborators: halo2curves' multi-Miller loop and final exponentiation, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_pairing_product_payload`, and `build_pairing_product_term`; the expectation is `definition_value`, the Miller loop over the generators raised to `definition_exponent()` built by `num-bigint`, independent of the library's final exponentiation
      * `[ ]`   Arrange: `build_pairing_product_payload` with the terms override holding one `build_pairing_product_term` at its default generators
      * `[ ]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value.inner()` equals `&definition_value(eip_196_generator(), eip_197_generator())`
    * `[ ]`   `encode_gt_writes_the_target_group_identity_with_c0_c0_c0_first`
      * `[ ]`   Contract: a target-group value → `Ok(EncodeGtSuccessReturn { bytes })` holding its twelve coefficients in tower order, `c0.c0.c0` first, each 32 bytes big-endian, inside a `Secret`
      * `[ ]`   Collaborators: halo2curves' `to_repr`, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_encode_gt_payload`, and `build_bn254_halo2curves_gt`; the expectation is `gt_identity_encoding()`
      * `[ ]`   Arrange: `build_encode_gt_payload` with the value override `build_bn254_halo2curves_gt` with the value `Gt::identity()`
      * `[ ]`   Act: `encode_gt(EncodeGtParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.bytes.expose()` binds to a `Bn254Halo2curvesEncodedGt` typed binding; its bytes equal `gt_identity_encoding()` as a byte slice
    * `[ ]`   `encode_gt_writes_the_definition_value_in_tower_order`
      * `[ ]`   Contract: a target-group value → `Ok(EncodeGtSuccessReturn { bytes })` holding its twelve coefficients in tower order, `c0.c0.c0` through `c1.c2.c1`, each 32 bytes big-endian, inside a `Secret`
      * `[ ]`   Collaborators: halo2curves' `to_repr`, run for real as the vendor; fixtures `build_bn254_halo2curves_pairing`, `build_encode_gt_payload`, and `build_bn254_halo2curves_gt`; the expectation is `definition_encoding()`, the serialization in the fixtures of the Miller loop over the generators raised to the exact exponent
      * `[ ]`   Arrange: `build_encode_gt_payload` at its default value, the pairing of the generators, whose twelve coefficients are distinct, so any exchange of two positions changes the bytes
      * `[ ]`   Act: `encode_gt(EncodeGtParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; the bytes of `success.bytes.expose()` equal `definition_encoding()` as a byte slice
    * `[ ]`   `scalar_field_order_is_the_group_order`
      * `[ ]`   Contract: any call → `Ok(ScalarFieldOrderSuccessReturn { bytes })` holding the group order's 32 big-endian bytes
      * `[ ]`   Collaborators: halo2curves' `Fr` negation and `to_repr`, run for real as the vendor; fixture `build_bn254_halo2curves_pairing`; `ScalarFieldOrderParams` and `ScalarFieldOrderPayload` by their production values
      * `[ ]`   Arrange: `build_bn254_halo2curves_pairing()`
      * `[ ]`   Act: `scalar_field_order(ScalarFieldOrderParams, ScalarFieldOrderPayload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.bytes` equals `vector_bytes(GROUP_ORDER_HEX)`
    * `[ ]`   `g1_outside_subgroup_encoding_is_absent_where_the_cofactor_is_one`
      * `[ ]`   Contract: any call → `Ok(G1OutsideSubgroupEncodingSuccessReturn { bytes: None })`
      * `[ ]`   Collaborators: none called; fixture `build_bn254_halo2curves_pairing`; `G1OutsideSubgroupEncodingParams` and `G1OutsideSubgroupEncodingPayload` by their production values
      * `[ ]`   Arrange: `build_bn254_halo2curves_pairing()`
      * `[ ]`   Act: `g1_outside_subgroup_encoding(G1OutsideSubgroupEncodingParams, G1OutsideSubgroupEncodingPayload)`
      * `[ ]`   Assert: the success arm is extracted with `expect`; `success.bytes.is_none()` is true
    * `[ ]`   `g2_outside_subgroup_encoding_encodes_an_on_curve_point_outside_the_subgroup`
      * `[ ]`   Contract: any call → `Ok(G2OutsideSubgroupEncodingSuccessReturn { bytes })` holding the precompile encoding of an on-curve point outside the prime-order subgroup, in this concrete's `Bn254Halo2curvesEncodedG2`
      * `[ ]`   Collaborators: halo2curves' curve and torsion checks, run for real as the vendor; fixture `build_bn254_halo2curves_pairing`; `G2OutsideSubgroupEncodingParams` and `G2OutsideSubgroupEncodingPayload` by their production values; the point read back by `g2_point_from_encoding`
      * `[ ]`   Arrange: `build_bn254_halo2curves_pairing()`
      * `[ ]`   Act: `g2_outside_subgroup_encoding(G2OutsideSubgroupEncodingParams, G2OutsideSubgroupEncodingPayload)`
      * `[ ]`   Assert: the success arm is extracted with `expect`; `success.bytes` binds to a `Bn254Halo2curvesEncodedG2` typed binding; `g2_point_from_encoding` reads a point from its bytes, which is `Some` because the point is on the curve, and that point's `to_curve().is_torsion_free()` is false
    * `[ ]`   `g2_outside_subgroup_encoding_has_a_real_first_coordinate_and_the_lesser_root`
      * `[ ]`   Contract: any call → the encoded point's first coordinate is `(c0, 0)` with the least `c0` of an on-curve point outside the subgroup, and its `y` is the lesser of the two roots, compared by `y.c1` and then `y.c0`
      * `[ ]`   Collaborators: halo2curves' curve and torsion checks, run for real as the vendor; fixture `build_bn254_halo2curves_pairing`; `G2OutsideSubgroupEncodingParams` and `G2OutsideSubgroupEncodingPayload` by their production values; the expectations are `g2_outside_subgroup_bytes()`, the ascending search in the fixtures, and `base_field_modulus()`
      * `[ ]`   Arrange: `build_bn254_halo2curves_pairing()`
      * `[ ]`   Act: `g2_outside_subgroup_encoding(G2OutsideSubgroupEncodingParams, G2OutsideSubgroupEncodingPayload)`
      * `[ ]`   Assert: the success arm is extracted with `expect`; the encoding's `x.c1` quarter, its first 32 bytes, is all zero; its `x.c0` quarter, its second 32 bytes, equals the `x.c0` quarter of `g2_outside_subgroup_bytes()`; the pair `y.c1`, `y.c0` read from its last 64 bytes as `BigUint` values is not greater than the pair of their negations, each `base_field_modulus()` minus the coefficient, reduced to zero where the coefficient is zero

  * `[✅]`   `construction`
    * `[✅]`   `Bn254Halo2curvesPairing::try_new` is the concrete's only producer, and its only caller is the pairing factory, which reads `Bn254Halo2curvesPairing::DECLARATION` before constructing
    * `[✅]`   A group element or scalar is produced only by the adapter's generators, arithmetic, and decoders, or by the scalar's sampling bound; a target-group value only by `pairing_product`; an encoded value only by the adapter's encoders and reference methods; no consumer constructs one from library values

  * `[✅]`   `adapters/pairing/src/bn254_halo2curves/mod.rs`
    * `[✅]`   Module declarations: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, `pub(crate) mod provides;`, and `#[cfg(test)] mod test;`
    * `[✅]`   Imports: from `crate::factory::provides` every params, payload, return, success-return, and error type of the three traits and the sampling bound, `IPairingAdapter`, `IPairingArithmetic`, `IPairingReference`, `ISampleUniformScalar`, `PAIRING_INTERFACE_VERSION`, `PairingConcrete`, `PairingCurve`, `PairingDeclaration`, `PrecompileEncoding`, `TargetGroupEncodingIdentifier`, and `VerifierGroupArithmetic`; `core::hint::black_box`; `core::iter::successors`; `domain::{Secret, SecretConstructorParams}`; `halo2curves::bn256::{Bn256, Fq, Fq2, Fr, G1Affine, G2Affine, Gt}`; `halo2curves::ff::{Field, FromUniformBytes, PrimeField}`; `halo2curves::group::{Curve, Group, cofactor::CofactorGroup, prime::PrimeCurveAffine}`; `halo2curves::msm::msm_best`; `halo2curves::pairing::{MillerLoopResult, MultiMillerLoop}`; `halo2curves::{Coordinates, CurveAffine}`; from `interface` every type it declares; `zeroize::{Zeroize, ZeroizeOnDrop}`
    * `[✅]`   `impl Bn254Halo2curvesPairing` holding `pub const DECLARATION: PairingDeclaration`, the constant the interaction spec states, and `pub fn try_new(params: Bn254Halo2curvesPairingConstructorParams) -> Bn254Halo2curvesPairingTryNewReturn` realizing the interaction spec's construct branch
    * `[✅]`   `impl IPairingAdapter for Bn254Halo2curvesPairing` with `const DECLARATION: PairingDeclaration = Bn254Halo2curvesPairing::DECLARATION;`, `const CONCRETE: PairingConcrete = PairingConcrete::Bn254Halo2curves;`, `type Scalar = Bn254Halo2curvesScalar;`, `type G1 = Bn254Halo2curvesG1;`, `type G2 = Bn254Halo2curvesG2;`, `type EncodedG1 = Bn254Halo2curvesEncodedG1;`, `type EncodedG2 = Bn254Halo2curvesEncodedG2;`, and `type EncodedScalar = Bn254Halo2curvesEncodedScalar;`, every method realizing its branches in the interaction spec
    * `[✅]`   `impl IPairingArithmetic for Bn254Halo2curvesPairing` with `type Gt = Bn254Halo2curvesGt;` and `type EncodedGt = Bn254Halo2curvesEncodedGt;`, every method realizing its branches in the interaction spec, `pairing_product` applying no correction
    * `[✅]`   `impl IPairingReference for Bn254Halo2curvesPairing`, every method realizing its branches in the interaction spec, the search by `find_map` with the `SearchExhausted` return by `let … else`
    * `[✅]`   `impl AsRef<[u8]>` for `Bn254Halo2curvesEncodedG1`, `Bn254Halo2curvesEncodedG2`, `Bn254Halo2curvesEncodedScalar`, and `Bn254Halo2curvesEncodedGt`, each returning `&self.bytes`; `impl Zeroize` for `Bn254Halo2curvesEncodedScalar` and `Bn254Halo2curvesEncodedGt`, each calling `self.bytes.zeroize()`; no implementation returns a mutable reference to the bytes
    * `[✅]`   `impl Zeroize` and `impl Drop` for `Bn254Halo2curvesScalar`, setting `value` to `Fr::ZERO` and calling `black_box(&self.value)`, and for `Bn254Halo2curvesG1`, `Bn254Halo2curvesG2`, and `Bn254Halo2curvesGt`, each setting `value` to its type's identity and calling `black_box(&self.value)`; `impl ZeroizeOnDrop for Bn254Halo2curvesScalar {}`, marking the drop behavior the sampling bound requires
    * `[✅]`   `impl ISampleUniformScalar for Bn254Halo2curvesScalar` with `const UNIFORM_BYTES_LENGTH: usize = 64;` and `sample_from_uniform_bytes` realizing its branches in the interaction spec, the scalar moved into a `Secret` by `let Ok(scalar) = Secret::try_new(SecretConstructorParams { value });`
    * `[✅]`   No `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`; neither the order nor a point is written as a literal

  * `[✅]`   `adapters/pairing/src/bn254_halo2curves/provides.rs`
    * `[✅]`   `pub(crate) use super::interface::*;` and `#[cfg(test)] pub(crate) use super::mock::*;`, exposing every item of the concrete's interface to the crate and, in the crate's own test build, the concrete's builders, fixtures, and helpers, and nothing beyond them

  * `[✅]`   `directionality`
    * `[✅]`   `bn254_halo2curves` depends on the `factory` module's surface through `crate::factory::provides`, on `domain`, on `zeroize`, and on `halo2curves`; it depends on no other concrete and no concrete depends on it; among repository crates the crate depends on `crates/domain` alone at runtime
    * `[✅]`   `pairing/factory` constructs this concrete, the family form's recorded cycle, and proves its target-group encoding, order, and outside-the-subgroup encodings equal the arkworks concrete's

  * `[ ]`   `requirements`
    * `[✅]`   `adapters/pairing/Cargo.toml` carries the `halo2curves` dependency, and `halo2curves` is named nowhere in the crate outside the halo2curves concretes
    * `[✅]`   This concrete returns its own distinct fixed-width encoded types for the first group, the second group, the scalar, and the target group through the family's associated types
    * `[✅]`   `cargo check --all-targets --all-features`, `cargo fmt --check`, and `cargo deny check` complete without error; `cargo clippy --all-targets --all-features` reports nothing beyond the library target's unused-item warnings for the pairing concretes, which `pairing/factory` constructs
    * `[✅]`   Every test in `bn254_halo2curves/test.rs` passes (CR-04, CR-08, CR-09, and CR-10 on BN254 over halo2curves; CR-05 for the sampled scalar; CR-07 for the clearing; CR-11 for the order)
    * `[ ]`   The declaration and concrete-identity compile-time assertions compile, and `pairing_product_of_the_generators_equals_the_definition` passes (CR-10, the target-group value fixed by the identifier's definition, executed in the test)
    * `[ ]`   The `ZeroizeOnDrop` compile-time assertion compiles and `bn254_halo2curves_scalar_zeroize_clears_its_value` passes (CR-07)
    * `[✅]`   Code outside `adapters/pairing` naming `Bn254Halo2curvesPairing` or anything under `bn254_halo2curves` fails to compile; the crate's public surface is the `factory` module's `provides`

* `[ ]`   `pairing/bls12_381_arkworks` **BLS12-381 pairing concrete on arkworks implementing the pairing family's generic interface, arithmetic trait, and reference trait over the EIP-2537 encodings with subgroup checks on every input and the `Bls12381V1` target-group value; adds the BLS12-381 variants to the family's declaration**

  * `[✅]`   `objective`
    * `[✅]`   Problem: BLS12-381 is the primary verifier form on Base through the EIP-2537 precompiles, so the pairing family needs a BLS12-381 concrete whose encodings and checks match those precompiles exactly (CR-10)
    * `[✅]`   Problem: the KEM, the envelope, and the delivery proof consume the pairing family's arithmetic trait through whichever concrete the factory resolves, so this concrete implements it; the bytes the KEM hands the KDF are the identifier's target-group value, and `ark-ec`'s BLS12 final exponentiation returns the Miller loop's value raised to three times the exact exponent `(p^12 - 1) / r`, the Hayashida–Hayasaka–Teruya chain in its source, so without correction the library's reduced pairing is a different element of the target group than the identifier's (CR-04; CR-07; CR-08; CR-09; CR-10; `docs/research/cryptography.md`'s pairing adapter capability declaration)
    * `[✅]`   Problem: on BLS12-381 both source groups have a cofactor above one, and EIP-2537's addition precompiles perform no subgroup check, so a contract's own check is proven against an on-curve point outside the subgroup in each group; this concrete returns the group order and both encodings under the family's rule from its own library (CR-10 on chain; CR-11; the dependency map's `pairing/bn254_arkworks` row; the Pairing adapters and key derivation milestone's exit)
    * `[✅]`   Functional: the concrete implements `IPairingAdapter` over arkworks' BLS12-381, with its own scalar, group-element, and encoded types over the library's elements as the associated types
    * `[✅]`   Functional: it encodes and decodes a base field element as EIP-2537's 64 bytes, sixteen zero bytes followed by the 48-byte big-endian integer, a first-group point as 128 bytes of `x` then `y`, a second-group point as 256 bytes of `x.c0`, `x.c1`, `y.c0`, `y.c1`, the point at infinity as all zero bytes, and a scalar as 32 big-endian bytes
    * `[✅]`   Functional: decoding rejects a wrong length, a nonzero padding byte, a field element at or above the modulus, a point off the curve, and a point outside the prime-order subgroup in either group, since BLS12-381's first group has a nontrivial cofactor as well as its second
    * `[✅]`   Functional: its scalar type implements the family's sampling bound, reading the 64 uniform bytes as one big-endian integer reduced modulo the group order
    * `[✅]`   Functional: it declares the BLS12-381 curve, second-group arithmetic at the verifier, the EIP-2537 encoding, `TargetGroupEncodingIdentifier::Bls12381V1`, its adapter version, and the interface version it implements, and names `PairingConcrete::Bls12381Arkworks`, so the proof factory selects the verifier form with direct second-group equations
    * `[✅]`   Functional: `Bls12381ArkworksPairing` implements `IPairingArithmetic` with every method's behavior as `pairing/bn254_arkworks` states it for the trait; `Bls12381V1` is the optimal ate pairing as the CFRG pairing-friendly-curves draft fixes it, over the EIP-2537 generators, on the tower `Fp2 = Fp[u] / (u^2 + 1)`, `Fp6 = Fp2[v] / (v^3 - (u + 1))`, `Fp12 = Fp6[w] / (w^2 - v)`, the Miller loop's value raised to the exact exponent `(p^12 - 1) / r`
    * `[✅]`   Functional: `pairing_product` returns the identifier's exact value: the library's reduced pairing multiplied in the target group by `reduced_pairing_correction`, the inverse of three in the scalar field, computed once in `try_new` by Fermat
    * `[✅]`   Functional: the target-group encoding writes the twelve `Fq` coefficients of the `Fq12` in the tower order `c0.c0.c0`, `c0.c0.c1`, `c0.c1.c0`, `c0.c1.c1`, `c0.c2.c0`, `c0.c2.c1`, `c1.c0.c0`, `c1.c0.c1`, `c1.c1.c0`, `c1.c1.c1`, `c1.c2.c0`, `c1.c2.c1`, each 48 bytes big-endian, BLS12-381's base-field byte width, 576 bytes in all, without the 16-byte padding EIP-2537 adds to source-group coordinates
    * `[✅]`   Functional: the concrete's tests execute the identifier's definition, the library's Miller loop over the two generators raised to the integer `(p^12 - 1) / r` built by `num-bigint` from the library's moduli and serialized in the tower order, and assert the pairing of the base points the CFRG draft publishes in its test-vector appendix, requiring `pairing_product` then `encode_gt` over the generators to yield those bytes
    * `[✅]`   Functional: `Bls12381ArkworksScalar`, `Bls12381ArkworksG1`, `Bls12381ArkworksG2`, and `Bls12381ArkworksGt` zeroize their value through `Zeroize` and on drop
    * `[✅]`   Functional: the concrete returns BLS12-381's group order as 32 big-endian bytes and each group's point under the family's rule, encoded as `encode_g1` and `encode_g2` encode any point, each refused by its own decoder as outside the subgroup
    * `[✅]`   Non-functional: `ark-bls12-381` is named only inside `adapters/pairing/src/bls12_381_arkworks`, and `ark-ec` and `ark-ff` only inside the arkworks concretes; no order and no point is written as a literal outside a test

  * `[✅]`   `role`
    * `[✅]`   Adapter: a further concrete of the pairing family, implementing the generic interface, the sampling bound, the arithmetic trait, and the reference trait `pairing/bn254_arkworks` authors in the `factory` module, and declaring the identifier it authors
    * `[✅]`   Adds the variants `PairingCurve::Bls12381`, `VerifierGroupArithmetic::BothGroups`, and `PrecompileEncoding::Eip2537` to `factory/interface.rs`, the producers its declaration needs; edits no other item of the `factory` module
    * `[✅]`   Contributes its module line to `lib.rs` and its dependency line to the crate manifest
    * `[✅]`   Does not select between concretes; `pairing/factory` constructs a concrete by the composition's request and `harness-crypto/benchmark` records the default per curve
    * `[✅]`   Does not name, import, or compare against another concrete; `pairing/factory`'s integration test proves agreement with the halo2curves concrete
    * `[✅]`   Does not read the correction from configuration; it is a property of this library under this identifier and is computed in `try_new`
    * `[✅]`   Does not carry a commit

  * `[✅]`   `module`
    * `[✅]`   Bounded context: the private `bls12_381_arkworks` module of `adapters/pairing`, holding the adapter over arkworks' BLS12-381 with its correction field, its scalar, group-element, target-group, and encoded types over the library's elements, its constructor params, its implementations of the three traits, its declared identifier, its definition and published-vector tests, its zeroization, and the builder defaults for its owned types; and the three declaration variants in the `factory` module
    * `[✅]`   The node's files: the `adapters/pairing/Cargo.toml` dependency line, the `adapters/pairing/src/lib.rs` module line, the declaration variants in `adapters/pairing/src/factory/interface.rs`, and the `bls12_381_arkworks` module's `interface.rs`, `interaction.spec.md`, `mock.rs`, `test.rs`, `mod.rs`, and `provides.rs`
    * `[✅]`   Outside: the rest of the family's traits and declaration, every other concrete, and the factory's selection and admission

  * `[ ]`   `deps`
    * `[✅]`   The `factory` module's surface, through `crate::factory::provides`: `IPairingAdapter`, `IPairingArithmetic`, `IPairingReference`, `ISampleUniformScalar`, `PairingDeclaration` and its enums, `PairingConcrete`, `TargetGroupEncodingIdentifier`, `PAIRING_INTERFACE_VERSION`, and every params, payload, return, success-return, error type, and builder of the three traits and the sampling bound, as `pairing/bn254_arkworks` states them
    * `[✅]`   `domain`, runtime: `Secret` and `SecretConstructorParams`; `build_secret` and `SecretConstructorParamsOverrides` through the `mocks` feature
    * `[✅]`   `zeroize` `1.9.0`, runtime: the `Zeroize` and `ZeroizeOnDrop` traits and their implementations for arkworks' affine points and `PairingOutput`
    * `[✅]`   `ark-bls12-381` `0.6.0`, external crate, MIT OR Apache-2.0, runtime dependency named only in `bls12_381_arkworks`, default features; supplies the curve and the pairing
    * `[✅]`   `ark-ec` `0.6.0` and `ark-ff` `0.6.0`, the crate's runtime dependencies; supply the group and field arithmetic
    * `[ ]`   `hex` `0.4.3` and `num-bigint` `0.4.8`, the crate's dev-dependencies, for the test vectors and the integer exponent in `mock.rs`'s test-only fixtures and the root comparison in `test.rs`
    * `[✅]`   `core::convert::Infallible`, standard library, the constructor's error arm
    * `[✅]`   Reverse dependency: `pairing/factory`

  * `[ ]`   `context_slice`
    * `[✅]`   From `ark-bls12-381`: `Bls12_381`, `Fq`, `Fq2` with its public fields `c0` and `c1` and `Fq2::new(c0, c1)`, `Fq12`, `Fr`, `G1Affine`, `G1Projective`, `G2Affine`, and `G2Projective`
    * `[✅]`   From `ark-ec`: `AffineRepr` for `generator()`, `xy()`, and `is_zero()`; the short-Weierstrass affine `identity()`, `new_unchecked(x, y)`, `is_on_curve()`, `is_in_correct_subgroup_assuming_on_curve()`, and `get_point_from_x_unchecked(x, greatest)`; the affine `+` and `* Fr` producing projective points; unary `-` on the affine point; `CurveGroup::into_affine`; `VariableBaseMSM::msm_unchecked(bases, scalars)`; `pairing::Pairing::multi_pairing` over iterators of borrowed affine points, returning `PairingOutput<Bls12_381>` with public field `0`; `Pairing::multi_miller_loop` over the same iterators, returning `MillerLoopOutput<Bls12_381>` with public field `0`, the unreduced `Fq12`; `Mul<Fr> for PairingOutput<Bls12_381>`; `Pairing::pairing(p, q)`; `Zeroize` for the affine point, which zeroizes `x`, `y`, and `infinity`, so a zeroized point is `new_unchecked(0, 0)`, and for `PairingOutput`, which zeroizes its `Fq12`
    * `[✅]`   From `ark-ff`: `Fr`'s `+`, `*`, and unary `-` modulo the group order; `Fr::from(u64)` and `Fq::from(u64)`; `Field::ONE`; `Field::pow(&self, exp: impl AsRef<[u64]>)` on `Fr` and on `Fq12`; `PrimeField::MODULUS`, the `BigInt<4>` of `Fr` and the `BigInt<6>` of `Fq`; `BigInteger::sub_with_borrow(&mut self, other: &Self) -> bool` and `BigInt::<4>::from(u64)`; `From<BigInt<N>> for num_bigint::BigUint`; the public fields `c0` and `c1` of the degree-two and degree-twelve extensions and `c0`, `c1`, and `c2` of the degree-six extension; `PrimeField::from_be_bytes_mod_order(&[u8])` and `into_bigint()` on `Fq` and on `Fr`, the `BigInt<6>` of `Fq` ordering as the integer; `BigInteger::to_bytes_be()`, 48 bytes for `Fq` and 32 for `Fr`; `Zero::is_zero()` on `Fq`, on `Fq12`, and on `PairingOutput`; unary `-` on `Fq` and on `Fq2`
    * `[ ]`   From `num-bigint`, in `mock.rs`'s test-only fixtures and in `test.rs`: `BigUint::from(u32)`, `BigUint::from_bytes_be(&[u8])`, `BigUint::pow(&self, u32)`, `Sub`, `Div`, and `Rem` between `BigUint`s, and `to_u64_digits()`
    * `[✅]`   From `domain`: `Secret::try_new(SecretConstructorParams { value })` returning `Result<Secret<T>, Infallible>`, and `Secret::expose(&self) -> &T`

  * `[✅]`   `adapters/pairing/Cargo.toml`
    * `[✅]`   `[dependencies]` holds `ark-bls12-381 = "0.6.0"`, default features

  * `[✅]`   `adapters/pairing/src/lib.rs`
    * `[✅]`   The barrel holds `mod bls12_381_arkworks;`

  * `[✅]`   `adapters/pairing/src/factory/interface.rs`
    * `[✅]`   `PairingCurve` holds the variant `Bls12381`; `VerifierGroupArithmetic` holds the variant `BothGroups`; `PrecompileEncoding` holds the variant `Eip2537`

  * `[✅]`   `adapters/pairing/src/bls12_381_arkworks/interface.rs`
    * `[✅]`   Imports `core::convert::Infallible`; declares nothing beyond the items below
    * `[✅]`   `Bls12381ArkworksPairing`, a struct with no derives and one field, `pub(super) reduced_pairing_correction: ark_bls12_381::Fr`, with the doc comment "The inverse of three in the scalar field, the multiple by which the library's reduced pairing exceeds the identifier's exact value."
    * `[✅]`   `Bls12381ArkworksPairingConstructorParams`, a unit struct; `Bls12381ArkworksPairingTryNewReturn`, the alias `Result<Bls12381ArkworksPairing, Infallible>`, the error arm uninhabited because the adapter takes no configuration
    * `[✅]`   `Bls12381ArkworksScalar`, `#[derive(Clone)]`, with one field `pub(super) value: ark_bls12_381::Fr`; `Bls12381ArkworksG1`, `#[derive(Clone)]`, with one field `pub(super) value: ark_bls12_381::G1Affine`; `Bls12381ArkworksG2`, `#[derive(Clone)]`, with one field `pub(super) value: ark_bls12_381::G2Affine`
    * `[✅]`   `Bls12381ArkworksGt`, a struct with no derives and one field, `pub(super) value: ark_ec::pairing::PairingOutput<ark_bls12_381::Bls12_381>`, the target-group value
    * `[✅]`   `Bls12381ArkworksEncodedG1` and `Bls12381ArkworksEncodedG2`, `pub struct`s with `#[derive(Clone, PartialEq, Eq)]` and one field each, `pub(super) bytes: [u8; 128]` and `pub(super) bytes: [u8; 256]`; `Bls12381ArkworksEncodedScalar`, a `pub struct` with no derives and one field `pub(super) bytes: [u8; 32]`; `Bls12381ArkworksEncodedGt`, a `pub struct` with no derives and one field `pub(super) bytes: [u8; 576]`; the scalar and target-group encodings are returned only inside a `Secret`, and no encoded type has a public constructor or mutable byte access

  * `[✅]`   `adapters/pairing/src/bls12_381_arkworks/interaction.spec.md`
    * `[✅]`   The title `` # `bls12_381_arkworks` — interaction spec `` and one opening sentence stating the file as the branch contract for the `bls12_381_arkworks` module of the `pairing` crate, the pairing adapter over arkworks' BLS12-381 encoding group elements as EIP-2537 precompile input, each branch stating condition, decision, dependency call, and the exact return outcome; then one `##` section per constructor, constant, and method, each headed by its full signature and holding a table with the columns `Branch`, `Condition`, `Decision`, `Dependency call`, and `Outcome`, in the section order `pairing/bn254_arkworks` states for its spec, then `Ordering and edges`
    * `[✅]`   `try_new`: one branch, construct; condition any params; decision none; dependency calls, in order: `Fr::from(3u64)` as `multiple`; `let mut exponent = Fr::MODULUS;` then `let _ = exponent.sub_with_borrow(&BigInt::from(2u64));`, the group order minus two; `multiple.pow(exponent)` as `reduced_pairing_correction`; outcome `Ok(Bls12381ArkworksPairing { reduced_pairing_correction })`; the error arm has no branch, `Infallible` being uninhabited
    * `[✅]`   `DECLARATION`: the inherent constant `PairingDeclaration { curve: PairingCurve::Bls12381, verifier_group_arithmetic: VerifierGroupArithmetic::BothGroups, precompile_encoding: PrecompileEncoding::Eip2537, target_group_encoding: TargetGroupEncodingIdentifier::Bls12381V1, adapter_version: 1, interface_version: PAIRING_INTERFACE_VERSION }`, readable from the type before any instance exists; the trait constant `IPairingAdapter::DECLARATION` is this constant
    * `[✅]`   `CONCRETE`: the trait constant `PairingConcrete::Bls12381Arkworks`
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

  * `[ ]`   `adapters/pairing/src/bls12_381_arkworks/mock.rs`
    * `[ ]`   The module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`; imports `super::interface::{Bls12381ArkworksEncodedGt, Bls12381ArkworksEncodedScalar, Bls12381ArkworksG1, Bls12381ArkworksG2, Bls12381ArkworksGt, Bls12381ArkworksPairing, Bls12381ArkworksPairingConstructorParams, Bls12381ArkworksScalar}`, `ark_bls12_381::{Bls12_381, Fq, Fq2, Fq12, Fr, G1Affine, G2Affine}`, `ark_ec::{AffineRepr, pairing::{Pairing, PairingOutput}}`, and `ark_ff::{BigInteger, Field, PrimeField}`; and, `#[cfg(test)]`, `ark_ec::CurveGroup`, `hex::decode`, `num_bigint::BigUint`, and `zeroize::ZeroizeOnDrop`
    * `[✅]`   `impl Default for Bls12381ArkworksScalar` returning `value: Fr::from(1u64)`; `impl Default for Bls12381ArkworksG1` returning `value: G1Affine::generator()`; `impl Default for Bls12381ArkworksG2` returning `value: G2Affine::generator()`; `impl Default for Bls12381ArkworksGt` returning `value: Bls12_381::pairing(G1Affine::generator(), G2Affine::generator())`, a non-identity target-group value; the family's generic builders and `MockIPairingAdapter` read these through `Default`
    * `[ ]`   The builders for the concrete's owned types, each overrides struct `#[derive(Default)]` with one `Option` field and each omitted value taking the type's `Default` above: `Bls12381ArkworksScalarOverrides` with `pub value: Option<Fr>` and `build_bls12_381_arkworks_scalar`; `Bls12381ArkworksG1Overrides` with `pub value: Option<G1Affine>` and `build_bls12_381_arkworks_g1`; `Bls12381ArkworksG2Overrides` with `pub value: Option<G2Affine>` and `build_bls12_381_arkworks_g2`; `Bls12381ArkworksGtOverrides` with `pub value: Option<PairingOutput<Bls12_381>>` and `build_bls12_381_arkworks_gt`; no corruptions type and no invalidator, since none of these arrives as untrusted data
    * `[ ]`   `build_bls12_381_arkworks_pairing() -> Bls12381ArkworksPairing`, the real instance from `Bls12381ArkworksPairing::try_new(Bls12381ArkworksPairingConstructorParams)` through `let Ok(pairing) = …;`; the constructor params are fieldless, so the builder takes no overrides
    * `[ ]`   The builders for the concrete's encoded scalar and encoded target-group types, each overrides struct `#[derive(Default)]` with one `Option` field: `Bls12381ArkworksEncodedScalarOverrides` with `pub bytes: Option<[u8; 32]>` and `build_bls12_381_arkworks_encoded_scalar(overrides: Bls12381ArkworksEncodedScalarOverrides) -> Bls12381ArkworksEncodedScalar`, the bytes defaulting to `[1u8; 32]`; `Bls12381ArkworksEncodedGtOverrides` with `pub bytes: Option<[u8; 576]>` and `build_bls12_381_arkworks_encoded_gt`, the bytes defaulting to `[1u8; 576]`; the defaults are nonzero so a zeroized value differs from a built one; no corruptions type and no invalidator, since neither arrives as untrusted data
    * `[ ]`   The test fixtures, each `#[cfg(test)]`, since they use the crate's dev-dependencies; constants, each a `const … : &str` of hex: `BASE_FIELD_MODULUS_PADDED_HEX`, 16 zero bytes then `1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab`; `GROUP_ORDER_HEX` `73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001`; `GROUP_ORDER_MINUS_ONE_HEX` `73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000000`; `GROUP_ORDER_MINUS_TWO_HEX` `73eda753299d7d483339d80809a1d80553bda402fffe5bfefffffffeffffffff`; `G1_GENERATOR_HEX`, 16 zero bytes, `17f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb`, 16 zero bytes, `08b3f481e3aaa0f1a09e30ed741d8ae4fcf5e095d5d00af600db18cb2c04b3edd03cc744a2888ae40caa232946c5e7e1`; `G1_GENERATOR_OFF_CURVE_HEX`, the same with the last byte `e1` replaced by `e2`; `G2_GENERATOR_HEX`, 16 zero bytes and `024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8`, 16 zero bytes and `13e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e`, 16 zero bytes and `0ce5d527727d6e118cc9cdc6da2e351aadfd9baa8cbdd3a76d429a695160d12c923ac9cc3baca289e193548608b82801`, 16 zero bytes and `0606c4a02ea734cc32acd2b02bc28b99cb3e287e85a763af267492ab572e99ab3f370d275cec1da1aaa9075ff05f79be`; `G2_GENERATOR_OFF_CURVE_HEX`, the same with the last byte `be` replaced by `bf`; `NEG_G1_GENERATOR_HEX`, 16 zero bytes and `17f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb`, then 16 zero bytes and `114d1d6855d545a8aa7d76c8cf2e21f267816aef1db507c96655b9d5caac42364e6f38ba0ecb751bad54dcd6b939c2ca`; `G1_NONZERO_PADDING_HEX`, `G1_GENERATOR_HEX` with its first byte `00` replaced by `01`; `G1_X_AT_MODULUS_HEX`, the 64 bytes of `BASE_FIELD_MODULUS_PADDED_HEX` then the last 64 bytes of `G1_GENERATOR_HEX`; `G2_NONZERO_PADDING_HEX`, `G2_GENERATOR_HEX` with its first byte `00` replaced by `01`; `G2_X_C0_AT_MODULUS_HEX`, the 64 bytes of `BASE_FIELD_MODULUS_PADDED_HEX` then the last 192 bytes of `G2_GENERATOR_HEX`; `UNIFORM_GROUP_ORDER_HEX`, 32 zero bytes then the 32 bytes of `GROUP_ORDER_HEX`, the group order as a 64-byte big-endian integer; `UNIFORM_FIVE_HEX`, 63 zero bytes and `05`
    * `[ ]`   The test-fixture constant `CFRG_GENERATOR_PAIRING_HEX`, `#[cfg(test)]`, the 576-byte encoding of the pairing of the base points `BP` and `BP'`, which are the EIP-2537 generators, as Appendix B of `draft-irtf-cfrg-pairing-friendly-curves-11` publishes it, its twelve coefficients `e_0` through `e_11` concatenated in order, the draft's inductive octet rule being the tower order the objective states; the coefficients, each 96 hex characters: `11619b45f61edfe3b47a15fac19442526ff489dcda25e59121d9931438907dfd448299a87dde3a649bdba96e84d54558`, `153ce14a76a53e205ba8f275ef1137c56a566f638b52d34ba3bf3bf22f277d70f76316218c0dfd583a394b8448d2be7f`, `095668fb4a02fe930ed44767834c915b283b1c6ca98c047bd4c272e9ac3f3ba6ff0b05a93e59c71fba77bce995f04692`, `16deedaa683124fe7260085184d88f7d036b86f53bb5b7f1fc5e248814782065413e7d958d17960109ea006b2afdeb5f`, `09c92cf02f3cd3d2f9d34bc44eee0dd50314ed44ca5d30ce6a9ec0539be7a86b121edc61839ccc908c4bdde256cd6048`, `111061f398efc2a97ff825b04d21089e24fd8b93a47e41e60eae7e9b2a38d54fa4dedced0811c34ce528781ab9e929c7`, `01ecfcf31c86257ab00b4709c33f1c9c4e007659dd5ffc4a735192167ce197058cfb4c94225e7f1b6c26ad9ba68f63bc`, `08890726743a1f94a8193a166800b7787744a8ad8e2f9365db76863e894b7a11d83f90d873567e9d645ccf725b32d26f`, `0e61c752414ca5dfd258e9606bac08daec29b3e2c57062669556954fb227d3f1260eedf25446a086b0844bcd43646c10`, `0fe63f185f56dd29150fc498bbeea78969e7e783043620db33f75a05a0a2ce5c442beaff9da195ff15164c00ab66bdde`, `10900338a92ed0b47af211636f7cfdec717b7ee43900eee9b5fc24f0000c5874d4801372db478987691c566a8c474978`, `1454814f3085f0e6602247671bc408bbce2007201536818c901dbd4d2095dd86c1ec8b888e59611f60a301af7776be3d`
    * `[ ]`   The test-fixture helpers, each `#[cfg(test)]`: `vector_bytes`, `zero_bytes`, `scalar_value`, and `requires_zeroize_on_drop` as `pairing/bn254_arkworks` states them; `base_field_modulus() -> BigUint`, `BigUint::from_bytes_be(&vector_bytes(BASE_FIELD_MODULUS_PADDED_HEX))`; `gt_identity_encoding() -> Vec<u8>`, `vec![0u8; 576]` with index `47` set to `1`; `g1_point_from_encoding(bytes: &[u8]) -> G1Affine`, `G1Affine::new_unchecked` over the two 64-byte coordinates of `bytes`, each its last 48 bytes read by `Fq::from_be_bytes_mod_order`; `g2_point_from_encoding(bytes: &[u8]) -> G2Affine`, `G2Affine::new_unchecked(Fq2::new(x_c0, x_c1), Fq2::new(y_c0, y_c1))` over the four 64-byte coordinates of `bytes`, read in order as `x_c0`, `x_c1`, `y_c0`, `y_c1`, each its last 48 bytes read by `Fq::from_be_bytes_mod_order`; `eip_2537_g1_generator() -> G1Affine`, `g1_point_from_encoding(&vector_bytes(G1_GENERATOR_HEX))`, the generator EIP-2537 publishes; `eip_2537_negated_g1_generator() -> G1Affine`, `g1_point_from_encoding(&vector_bytes(NEG_G1_GENERATOR_HEX))`; `eip_2537_g2_generator() -> G2Affine`, `g2_point_from_encoding(&vector_bytes(G2_GENERATOR_HEX))`; `g1_outside_subgroup_bytes() -> Vec<u8>`, the least on-curve point outside the subgroup, `(1u64..).find_map(|c| G1Affine::get_point_from_x_unchecked(Fq::from(c), false).filter(|point| !point.is_in_correct_subgroup_assuming_on_curve()))` unpacked with "an on-curve point outside the subgroup exists", its `xy()` unpacked with "the point has coordinates", encoded as 16 zero bytes, `x.into_bigint().to_bytes_be()`, 16 zero bytes, and `y.into_bigint().to_bytes_be()`; `g2_outside_subgroup_bytes() -> Vec<u8>`, the least on-curve point outside the subgroup, `(1u64..).find_map(|c0| G2Affine::get_point_from_x_unchecked(Fq2::new(Fq::from(c0), Fq::from(0u64)), false).filter(|point| !point.is_in_correct_subgroup_assuming_on_curve()))` unpacked with "an on-curve point outside the subgroup exists", its `xy()` unpacked with "the point has coordinates", encoded as `x.c0`, `x.c1`, `y.c0`, `y.c1`, each 16 zero bytes followed by `into_bigint().to_bytes_be()`; `definition_exponent() -> Vec<u64>`, `let p = BigUint::from(Fq::MODULUS); let r = BigUint::from(Fr::MODULUS); ((p.pow(12) - BigUint::from(1u32)) / r).to_u64_digits()`; `definition_value(g1: G1Affine, g2: G2Affine) -> Fq12`, `Bls12_381::multi_miller_loop([g1], [g2]).0.pow(definition_exponent())`, the identifier's definition of the pairing value; `definition_value_power(g1: G1Affine, g2: G2Affine, exponent: u64) -> Fq12`, `definition_value(g1, g2).pow([exponent])`; `eip_2537_doubled_g1_generator() -> G1Affine`, `(eip_2537_g1_generator() + eip_2537_g1_generator()).into_affine()`
    * `[ ]`   No mock function: the concrete is built as a real instance and owns no free function

  * `[ ]`   `adapters/pairing/src/bls12_381_arkworks/test.rs`
    * `[ ]`   Module-level `#![allow(clippy::expect_used)]`; imports from `super::provides` the concrete, its owned types, and the mock's builders, constants, and helpers each block uses; from `crate::factory::provides` the params, payloads, errors, trait names, and builders each block uses; and the `ark_bls12_381`, `ark_ec`, `ark_ff`, `num_bigint::BigUint`, and `zeroize::Zeroize` names each block uses
    * `[ ]`   Compile-time assertion over `Bls12381ArkworksPairing::DECLARATION`
      * `[ ]`   Contract: the inherent constant is `PairingDeclaration { curve: PairingCurve::Bls12381, verifier_group_arithmetic: VerifierGroupArithmetic::BothGroups, precompile_encoding: PrecompileEncoding::Eip2537, target_group_encoding: TargetGroupEncodingIdentifier::Bls12381V1, adapter_version: 1, interface_version: PAIRING_INTERFACE_VERSION }`, readable before any instance exists
      * `[ ]`   Arrange: the type alone
      * `[ ]`   Act: a module-level `const _: () = assert!(…);` reading `Bls12381ArkworksPairing::DECLARATION`, each enum field tested with `matches!` and each version with `==`
      * `[ ]`   Assert: the module compiles only if every field holds
    * `[ ]`   Compile-time assertion over `<Bls12381ArkworksPairing as IPairingAdapter>::DECLARATION`
      * `[ ]`   Contract: the trait constant carries the same fields as the inherent constant
      * `[ ]`   Arrange: the type alone
      * `[ ]`   Act: a module-level `const _: () = assert!(…);` reading the trait constant, each enum field tested with `matches!` and each version with `==`
      * `[ ]`   Assert: the module compiles only if every field holds
    * `[ ]`   Compile-time assertion over `<Bls12381ArkworksPairing as IPairingAdapter>::CONCRETE`
      * `[ ]`   Contract: the trait constant is `PairingConcrete::Bls12381Arkworks`
      * `[ ]`   Arrange: the type alone
      * `[ ]`   Act: a module-level `const _: () = assert!(…);` reading the trait constant under `matches!`
      * `[ ]`   Assert: the module compiles only if the constant is `PairingConcrete::Bls12381Arkworks`
    * `[ ]`   `bls12_381_arkworks_scalar_is_zeroize_on_drop`
      * `[ ]`   Contract: `Bls12381ArkworksScalar` implements `ZeroizeOnDrop`, the marker the sampling bound requires
      * `[ ]`   Collaborators: none
      * `[ ]`   Arrange: the type alone
      * `[ ]`   Act: `requires_zeroize_on_drop::<Bls12381ArkworksScalar>()`
      * `[ ]`   Assert: the block compiles only if the type implements `ZeroizeOnDrop`
    * `[ ]`   `bls12_381_arkworks_scalar_zeroize_clears_its_value`
      * `[ ]`   Contract: `Zeroize::zeroize` on a living scalar → its `value` is zero
      * `[ ]`   Collaborators: arkworks' `Fr` zeroization, run for real as the vendor; fixture `build_bls12_381_arkworks_scalar`
      * `[ ]`   Arrange: `build_bls12_381_arkworks_scalar` with the value override `Fr::from(5u64)`, so a scalar left unchanged differs from the expected zero
      * `[ ]`   Act: `scalar.zeroize()`
      * `[ ]`   Assert: `scalar.value` equals `Fr::from(0u64)`
    * `[ ]`   `bls12_381_arkworks_g1_zeroize_clears_its_value`
      * `[ ]`   Contract: `Zeroize::zeroize` on a living first-group element → its `value` is the point `new_unchecked(0, 0)`
      * `[ ]`   Collaborators: arkworks' affine zeroization, run for real as the vendor; fixture `build_bls12_381_arkworks_g1`
      * `[ ]`   Arrange: `build_bls12_381_arkworks_g1` at its default generator, which differs from the expected point
      * `[ ]`   Act: `g1.zeroize()`
      * `[ ]`   Assert: `g1.value` equals `G1Affine::new_unchecked(Fq::from(0u64), Fq::from(0u64))`
    * `[ ]`   `bls12_381_arkworks_g2_zeroize_clears_its_value`
      * `[ ]`   Contract: `Zeroize::zeroize` on a living second-group element → its `value` is the point `new_unchecked(0, 0)`
      * `[ ]`   Collaborators: arkworks' affine zeroization, run for real as the vendor; fixture `build_bls12_381_arkworks_g2`
      * `[ ]`   Arrange: `build_bls12_381_arkworks_g2` at its default generator, which differs from the expected point
      * `[ ]`   Act: `g2.zeroize()`
      * `[ ]`   Assert: `g2.value` equals `G2Affine::new_unchecked(Fq2::new(Fq::from(0u64), Fq::from(0u64)), Fq2::new(Fq::from(0u64), Fq::from(0u64)))`
    * `[ ]`   `bls12_381_arkworks_gt_zeroize_clears_its_value`
      * `[ ]`   Contract: `Zeroize::zeroize` on a living target-group value → every coefficient of its `Fq12` is zero
      * `[ ]`   Collaborators: arkworks' `PairingOutput` zeroization, run for real as the vendor; fixture `build_bls12_381_arkworks_gt`
      * `[ ]`   Arrange: `build_bls12_381_arkworks_gt` at its default pairing of the generators, whose `Fq12` is nonzero
      * `[ ]`   Act: `gt.zeroize()`
      * `[ ]`   Assert: `gt.value.0.is_zero()` is true
    * `[ ]`   `bls12_381_arkworks_encoded_scalar_zeroize_clears_its_bytes`
      * `[ ]`   Contract: `Zeroize::zeroize` on a living encoded scalar → every byte is zero
      * `[ ]`   Collaborators: none; fixture `build_bls12_381_arkworks_encoded_scalar`
      * `[ ]`   Arrange: `build_bls12_381_arkworks_encoded_scalar` at its nonzero default
      * `[ ]`   Act: `encoded.zeroize()`
      * `[ ]`   Assert: the bytes of `encoded` equal `zero_bytes(32)` as a byte slice
    * `[ ]`   `bls12_381_arkworks_encoded_gt_zeroize_clears_its_bytes`
      * `[ ]`   Contract: `Zeroize::zeroize` on a living encoded target-group value → every byte is zero
      * `[ ]`   Collaborators: none; fixture `build_bls12_381_arkworks_encoded_gt`
      * `[ ]`   Arrange: `build_bls12_381_arkworks_encoded_gt` at its nonzero default
      * `[ ]`   Act: `encoded.zeroize()`
      * `[ ]`   Assert: the bytes of `encoded` equal `zero_bytes(576)` as a byte slice
    * `[ ]`   `reduced_pairing_correction_inverts_three`
      * `[ ]`   Contract: any params → `Ok(Bls12381ArkworksPairing)` whose `reduced_pairing_correction` is the inverse of three in the scalar field
      * `[ ]`   Collaborators: arkworks' `Fr` arithmetic, run for real as the vendor; `Bls12381ArkworksPairingConstructorParams` by its production value
      * `[ ]`   Arrange: `Bls12381ArkworksPairingConstructorParams` by its production value
      * `[ ]`   Act: `Bls12381ArkworksPairing::try_new(Bls12381ArkworksPairingConstructorParams)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(pairing) = …;`; `pairing.reduced_pairing_correction * Fr::from(3u64)` equals `Fr::from(1u64)`
    * `[ ]`   `uniform_bytes_length_is_twice_the_group_order_width`
      * `[ ]`   Contract: `UNIFORM_BYTES_LENGTH` is twice the byte width of the group order
      * `[ ]`   Collaborators: none
      * `[ ]`   Arrange: the type alone
      * `[ ]`   Act: reading `<Bls12381ArkworksScalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH`
      * `[ ]`   Assert: the constant equals the literal 64 written in the assertion
    * `[ ]`   `g1_generator_returns_the_eip_2537_generator`
      * `[ ]`   Contract: any call → `Ok(G1GeneratorSuccessReturn { point })` holding the first group's generator
      * `[ ]`   Collaborators: arkworks, run for real as the vendor; fixture `build_bls12_381_arkworks_pairing`; `G1GeneratorParams` and `G1GeneratorPayload` by their production values
      * `[ ]`   Arrange: `build_bls12_381_arkworks_pairing()`
      * `[ ]`   Act: `g1_generator(G1GeneratorParams, G1GeneratorPayload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.point.value` equals `eip_2537_g1_generator()`
    * `[ ]`   `g2_generator_returns_the_eip_2537_generator`
      * `[ ]`   Contract: any call → `Ok(G2GeneratorSuccessReturn { point })` holding the second group's generator
      * `[ ]`   Collaborators: arkworks, run for real as the vendor; fixture `build_bls12_381_arkworks_pairing`; `G2GeneratorParams` and `G2GeneratorPayload` by their production values
      * `[ ]`   Arrange: `build_bls12_381_arkworks_pairing()`
      * `[ ]`   Act: `g2_generator(G2GeneratorParams, G2GeneratorPayload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.point.value` equals `eip_2537_g2_generator()`
    * `[ ]`   `add_g1_of_a_point_and_its_negation_is_the_identity`
      * `[ ]`   Contract: `payload.left` and `payload.right` → `Ok(AddG1SuccessReturn { sum })` holding their sum
      * `[ ]`   Collaborators: arkworks' affine addition, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_add_g1_payload`, and `build_bls12_381_arkworks_g1`
      * `[ ]`   Arrange: `build_add_g1_payload` with the left override `build_bls12_381_arkworks_g1` at its default generator and the right override `build_bls12_381_arkworks_g1` with the value `eip_2537_negated_g1_generator()`, so a sum that returns either input differs from the identity
      * `[ ]`   Act: `add_g1(AddG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G1Affine::identity()`
    * `[ ]`   `add_g1_of_the_identity_and_a_point_is_the_point`
      * `[ ]`   Contract: `payload.left` and `payload.right` → `Ok(AddG1SuccessReturn { sum })` holding their sum
      * `[ ]`   Collaborators: arkworks' affine addition, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_add_g1_payload`, and `build_bls12_381_arkworks_g1`
      * `[ ]`   Arrange: `build_add_g1_payload` with the left override `build_bls12_381_arkworks_g1` with the value `G1Affine::identity()` and the right override `build_bls12_381_arkworks_g1` at its default generator, so a sum that returns the left input differs from the generator
      * `[ ]`   Act: `add_g1(AddG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `eip_2537_g1_generator()`
    * `[ ]`   `add_g2_of_a_point_and_its_negation_is_the_identity`
      * `[ ]`   Contract: `payload.left` and `payload.right` → `Ok(AddG2SuccessReturn { sum })` holding their sum
      * `[ ]`   Collaborators: arkworks' affine addition, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_add_g2_payload`, and `build_bls12_381_arkworks_g2`
      * `[ ]`   Arrange: `build_add_g2_payload` with the left override `build_bls12_381_arkworks_g2` at its default generator and the right override `build_bls12_381_arkworks_g2` with the value `-eip_2537_g2_generator()`, so a sum that returns either input differs from the identity
      * `[ ]`   Act: `add_g2(AddG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G2Affine::identity()`
    * `[ ]`   `add_g2_of_the_identity_and_a_point_is_the_point`
      * `[ ]`   Contract: `payload.left` and `payload.right` → `Ok(AddG2SuccessReturn { sum })` holding their sum
      * `[ ]`   Collaborators: arkworks' affine addition, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_add_g2_payload`, and `build_bls12_381_arkworks_g2`
      * `[ ]`   Arrange: `build_add_g2_payload` with the left override `build_bls12_381_arkworks_g2` with the value `G2Affine::identity()` and the right override `build_bls12_381_arkworks_g2` at its default generator, so a sum that returns the left input differs from the generator
      * `[ ]`   Act: `add_g2(AddG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `eip_2537_g2_generator()`
    * `[ ]`   `mul_g1_by_one_is_the_point`
      * `[ ]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG1SuccessReturn { product })` holding the point multiplied by the scalar
      * `[ ]`   Collaborators: arkworks' affine scalar multiplication, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_mul_g1_payload`, `build_bls12_381_arkworks_g1`, and `build_bls12_381_arkworks_scalar`
      * `[ ]`   Arrange: `build_mul_g1_payload` with the point override `build_bls12_381_arkworks_g1` at its default generator and the scalar override `build_bls12_381_arkworks_scalar` with the value `Fr::from(1u64)`
      * `[ ]`   Act: `mul_g1(MulG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `eip_2537_g1_generator()`
    * `[ ]`   `mul_g1_by_zero_is_the_identity`
      * `[ ]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG1SuccessReturn { product })` holding the point multiplied by the scalar
      * `[ ]`   Collaborators: arkworks' affine scalar multiplication, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_mul_g1_payload`, `build_bls12_381_arkworks_g1`, and `build_bls12_381_arkworks_scalar`
      * `[ ]`   Arrange: `build_mul_g1_payload` with the point override `build_bls12_381_arkworks_g1` at its default generator and the scalar override `build_bls12_381_arkworks_scalar` with the value `Fr::from(0u64)`, so a product that returns the point differs from the identity
      * `[ ]`   Act: `mul_g1(MulG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `G1Affine::identity()`
    * `[ ]`   `mul_g1_by_the_group_order_minus_one_is_the_negated_generator`
      * `[ ]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG1SuccessReturn { product })` holding the point multiplied by the scalar
      * `[ ]`   Collaborators: arkworks' affine scalar multiplication, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_mul_g1_payload`, `build_bls12_381_arkworks_g1`, and `build_bls12_381_arkworks_scalar`
      * `[ ]`   Arrange: `build_mul_g1_payload` with the point override `build_bls12_381_arkworks_g1` at its default generator and the scalar override `build_bls12_381_arkworks_scalar` with the value `scalar_value(GROUP_ORDER_MINUS_ONE_HEX)`, so a product that ignores the scalar differs from the negated generator
      * `[ ]`   Act: `mul_g1(MulG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `eip_2537_negated_g1_generator()`
    * `[ ]`   `mul_g2_by_one_is_the_point`
      * `[ ]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG2SuccessReturn { product })` holding the point multiplied by the scalar
      * `[ ]`   Collaborators: arkworks' affine scalar multiplication, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_mul_g2_payload`, `build_bls12_381_arkworks_g2`, and `build_bls12_381_arkworks_scalar`
      * `[ ]`   Arrange: `build_mul_g2_payload` with the point override `build_bls12_381_arkworks_g2` at its default generator and the scalar override `build_bls12_381_arkworks_scalar` with the value `Fr::from(1u64)`
      * `[ ]`   Act: `mul_g2(MulG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `eip_2537_g2_generator()`
    * `[ ]`   `mul_g2_by_zero_is_the_identity`
      * `[ ]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG2SuccessReturn { product })` holding the point multiplied by the scalar
      * `[ ]`   Collaborators: arkworks' affine scalar multiplication, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_mul_g2_payload`, `build_bls12_381_arkworks_g2`, and `build_bls12_381_arkworks_scalar`
      * `[ ]`   Arrange: `build_mul_g2_payload` with the point override `build_bls12_381_arkworks_g2` at its default generator and the scalar override `build_bls12_381_arkworks_scalar` with the value `Fr::from(0u64)`, so a product that returns the point differs from the identity
      * `[ ]`   Act: `mul_g2(MulG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `G2Affine::identity()`
    * `[ ]`   `mul_g2_by_the_group_order_minus_one_is_the_negated_point`
      * `[ ]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG2SuccessReturn { product })` holding the point multiplied by the scalar
      * `[ ]`   Collaborators: arkworks' affine scalar multiplication, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_mul_g2_payload`, `build_bls12_381_arkworks_g2`, and `build_bls12_381_arkworks_scalar`
      * `[ ]`   Arrange: `build_mul_g2_payload` with the point override `build_bls12_381_arkworks_g2` at its default generator and the scalar override `build_bls12_381_arkworks_scalar` with the value `scalar_value(GROUP_ORDER_MINUS_ONE_HEX)`, so a product that ignores the scalar differs from the negated generator
      * `[ ]`   Act: `mul_g2(MulG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `-eip_2537_g2_generator()`
    * `[ ]`   `msm_g1_of_no_terms_is_the_identity`
      * `[ ]`   Contract: an empty `payload.terms` → `Ok(MsmG1SuccessReturn { sum })` holding the identity
      * `[ ]`   Collaborators: arkworks' multi-scalar multiplication, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing` and `build_msm_g1_payload`
      * `[ ]`   Arrange: `build_msm_g1_payload` at its default of no terms
      * `[ ]`   Act: `msm_g1(MsmG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G1Affine::identity()`
    * `[ ]`   `msm_g1_pairs_each_base_with_its_own_scalar`
      * `[ ]`   Contract: `payload.terms` → `Ok(MsmG1SuccessReturn { sum })` holding the sum of each base multiplied by its own scalar
      * `[ ]`   Collaborators: arkworks' multi-scalar multiplication, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_msm_g1_payload`, `build_msm_g1_term`, `build_bls12_381_arkworks_g1`, and `build_bls12_381_arkworks_scalar`
      * `[ ]`   Arrange: `build_msm_g1_payload` with the terms override holding two `build_msm_g1_term` values: the generator with the scalar `Fr::from(1u64)`, and the base `eip_2537_negated_g1_generator()` with the scalar `Fr::from(0u64)`, so bases and scalars exchanged between terms yield the negated generator
      * `[ ]`   Act: `msm_g1(MsmG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `eip_2537_g1_generator()`
    * `[ ]`   `msm_g1_sums_its_terms`
      * `[ ]`   Contract: `payload.terms` → `Ok(MsmG1SuccessReturn { sum })` holding the sum of each base multiplied by its own scalar
      * `[ ]`   Collaborators: arkworks' multi-scalar multiplication, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_msm_g1_payload`, `build_msm_g1_term`, `build_bls12_381_arkworks_g1`, and `build_bls12_381_arkworks_scalar`
      * `[ ]`   Arrange: `build_msm_g1_payload` with the terms override holding two `build_msm_g1_term` values: the generator with the scalar `Fr::from(1u64)`, and the base `eip_2537_negated_g1_generator()` with the scalar `Fr::from(1u64)`, so a sum over the first term alone differs from the identity
      * `[ ]`   Act: `msm_g1(MsmG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G1Affine::identity()`
    * `[ ]`   `msm_g2_of_no_terms_is_the_identity`
      * `[ ]`   Contract: an empty `payload.terms` → `Ok(MsmG2SuccessReturn { sum })` holding the identity
      * `[ ]`   Collaborators: arkworks' multi-scalar multiplication, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing` and `build_msm_g2_payload`
      * `[ ]`   Arrange: `build_msm_g2_payload` at its default of no terms
      * `[ ]`   Act: `msm_g2(MsmG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G2Affine::identity()`
    * `[ ]`   `msm_g2_pairs_each_base_with_its_own_scalar`
      * `[ ]`   Contract: `payload.terms` → `Ok(MsmG2SuccessReturn { sum })` holding the sum of each base multiplied by its own scalar
      * `[ ]`   Collaborators: arkworks' multi-scalar multiplication, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_msm_g2_payload`, `build_msm_g2_term`, `build_bls12_381_arkworks_g2`, and `build_bls12_381_arkworks_scalar`
      * `[ ]`   Arrange: `build_msm_g2_payload` with the terms override holding two `build_msm_g2_term` values: the generator with the scalar `Fr::from(1u64)`, and the base `-eip_2537_g2_generator()` with the scalar `Fr::from(0u64)`, so bases and scalars exchanged between terms yield the negated generator
      * `[ ]`   Act: `msm_g2(MsmG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `eip_2537_g2_generator()`
    * `[ ]`   `msm_g2_sums_its_terms`
      * `[ ]`   Contract: `payload.terms` → `Ok(MsmG2SuccessReturn { sum })` holding the sum of each base multiplied by its own scalar
      * `[ ]`   Collaborators: arkworks' multi-scalar multiplication, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_msm_g2_payload`, `build_msm_g2_term`, `build_bls12_381_arkworks_g2`, and `build_bls12_381_arkworks_scalar`
      * `[ ]`   Arrange: `build_msm_g2_payload` with the terms override holding two `build_msm_g2_term` values: the generator with the scalar `Fr::from(1u64)`, and the base `-eip_2537_g2_generator()` with the scalar `Fr::from(1u64)`, so a sum over the first term alone differs from the identity
      * `[ ]`   Act: `msm_g2(MsmG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G2Affine::identity()`
    * `[ ]`   `pairing_product_is_one_of_no_terms_is_true`
      * `[ ]`   Contract: an empty `payload.terms` → `Ok(PairingProductIsOneSuccessReturn { is_one })` with `is_one` true
      * `[ ]`   Collaborators: arkworks' multi-pairing, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing` and `build_pairing_product_is_one_payload`
      * `[ ]`   Arrange: `build_pairing_product_is_one_payload` at its default of no terms
      * `[ ]`   Act: `pairing_product_is_one(PairingProductIsOneParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_one` is true
    * `[ ]`   `pairing_product_is_one_of_a_pairing_and_its_first_group_negation_is_true`
      * `[ ]`   Contract: `payload.terms` whose pairings multiply to the target group's identity → `Ok(PairingProductIsOneSuccessReturn { is_one })` with `is_one` true
      * `[ ]`   Collaborators: arkworks' multi-pairing, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_pairing_product_is_one_payload`, `build_pairing_product_term`, and `build_bls12_381_arkworks_g1`
      * `[ ]`   Arrange: `build_pairing_product_is_one_payload` with the terms override holding two `build_pairing_product_term` values: the generators, and the first-group override `build_bls12_381_arkworks_g1` with the value `eip_2537_negated_g1_generator()` over the second-group generator
      * `[ ]`   Act: `pairing_product_is_one(PairingProductIsOneParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_one` is true
    * `[ ]`   `pairing_product_is_one_of_the_generators_is_false`
      * `[ ]`   Contract: `payload.terms` whose pairings multiply to a value other than the target group's identity → `Ok(PairingProductIsOneSuccessReturn { is_one })` with `is_one` false
      * `[ ]`   Collaborators: arkworks' multi-pairing, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_pairing_product_is_one_payload`, and `build_pairing_product_term`
      * `[ ]`   Arrange: `build_pairing_product_is_one_payload` with the terms override holding one `build_pairing_product_term` at its default generators
      * `[ ]`   Act: `pairing_product_is_one(PairingProductIsOneParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_one` is false
    * `[ ]`   `decode_g1_rejects_a_wrong_length`
      * `[ ]`   Contract: `payload.len() != 128` → `Err(DecodeG1ErrorReturn::WrongLength { expected: 128, actual: payload.len() })`
      * `[ ]`   Collaborators: none called; fixture `build_bls12_381_arkworks_pairing`; the untrusted bytes from `zero_bytes`
      * `[ ]`   Arrange: the payload `zero_bytes(127)`, which differs from the required length
      * `[ ]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(DecodeG1ErrorReturn::WrongLength { expected: 128, actual: 127 })`, the whole expected error
    * `[ ]`   `decode_g1_rejects_a_nonzero_padding_byte`
      * `[ ]`   Contract: a 128-byte payload with a nonzero byte among a coordinate's first 16 → `Err(DecodeG1ErrorReturn::NonCanonicalCoordinate)`
      * `[ ]`   Collaborators: none called; fixture `build_bls12_381_arkworks_pairing`; the untrusted bytes from `vector_bytes(G1_NONZERO_PADDING_HEX)`
      * `[ ]`   Arrange: the payload `vector_bytes(G1_NONZERO_PADDING_HEX)`, the generator with one padding byte changed, so a decoder that ignores the padding returns the generator
      * `[ ]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(DecodeG1ErrorReturn::NonCanonicalCoordinate)`
    * `[ ]`   `decode_g1_rejects_a_non_canonical_coordinate`
      * `[ ]`   Contract: a 128-byte payload whose first coordinate is at least the base field modulus → `Err(DecodeG1ErrorReturn::NonCanonicalCoordinate)`
      * `[ ]`   Collaborators: arkworks' field reduction, run for real as the vendor; fixture `build_bls12_381_arkworks_pairing`; the untrusted bytes from `vector_bytes(G1_X_AT_MODULUS_HEX)`
      * `[ ]`   Arrange: the payload `vector_bytes(G1_X_AT_MODULUS_HEX)`, whose reduced coordinate would otherwise fail the curve check, so a decoder that checks the curve first returns a different error
      * `[ ]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(DecodeG1ErrorReturn::NonCanonicalCoordinate)`
    * `[ ]`   `decode_g1_decodes_the_identity_from_zero_bytes`
      * `[ ]`   Contract: a 128-byte payload with both coordinates zero → `Ok(DecodeG1SuccessReturn { point })` holding the first group's identity
      * `[ ]`   Collaborators: arkworks, run for real as the vendor; fixture `build_bls12_381_arkworks_pairing`; the untrusted bytes from `zero_bytes`
      * `[ ]`   Arrange: the payload `zero_bytes(128)`
      * `[ ]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[ ]`   Assert: the success arm is extracted with `expect`; `success.point.value` equals `G1Affine::identity()`
    * `[ ]`   `decode_g1_rejects_a_point_off_the_curve`
      * `[ ]`   Contract: a canonical 128-byte payload that satisfies no curve equation → `Err(DecodeG1ErrorReturn::NotOnCurve)`
      * `[ ]`   Collaborators: arkworks' curve check, run for real as the vendor; fixture `build_bls12_381_arkworks_pairing`; the untrusted bytes from `vector_bytes(G1_GENERATOR_OFF_CURVE_HEX)`
      * `[ ]`   Arrange: the payload `vector_bytes(G1_GENERATOR_OFF_CURVE_HEX)`
      * `[ ]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(DecodeG1ErrorReturn::NotOnCurve)`
    * `[ ]`   `decode_g1_rejects_a_point_outside_the_subgroup`
      * `[ ]`   Contract: a canonical 128-byte payload on the curve and outside the prime-order subgroup → `Err(DecodeG1ErrorReturn::NotInSubgroup)`
      * `[ ]`   Collaborators: arkworks' subgroup check, run for real as the vendor; fixture `build_bls12_381_arkworks_pairing`; the untrusted bytes from `g1_outside_subgroup_bytes()`
      * `[ ]`   Arrange: the payload `g1_outside_subgroup_bytes()`
      * `[ ]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(DecodeG1ErrorReturn::NotInSubgroup)`
    * `[ ]`   `decode_g1_decodes_the_eip_2537_generator`
      * `[ ]`   Contract: a canonical 128-byte payload on the curve and in the subgroup → `Ok(DecodeG1SuccessReturn { point })` holding that point
      * `[ ]`   Collaborators: arkworks, run for real as the vendor; fixture `build_bls12_381_arkworks_pairing`; the untrusted bytes from `vector_bytes(G1_GENERATOR_HEX)`
      * `[ ]`   Arrange: the payload `vector_bytes(G1_GENERATOR_HEX)`
      * `[ ]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[ ]`   Assert: the success arm is extracted with `expect`; `success.point.value` equals `eip_2537_g1_generator()`
    * `[ ]`   `decode_g2_rejects_a_wrong_length`
      * `[ ]`   Contract: `payload.len() != 256` → `Err(DecodeG2ErrorReturn::WrongLength { expected: 256, actual: payload.len() })`
      * `[ ]`   Collaborators: none called; fixture `build_bls12_381_arkworks_pairing`; the untrusted bytes from `zero_bytes`
      * `[ ]`   Arrange: the payload `zero_bytes(255)`, which differs from the required length
      * `[ ]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(DecodeG2ErrorReturn::WrongLength { expected: 256, actual: 255 })`, the whole expected error
    * `[ ]`   `decode_g2_rejects_a_nonzero_padding_byte`
      * `[ ]`   Contract: a 256-byte payload with a nonzero byte among a coordinate's first 16 → `Err(DecodeG2ErrorReturn::NonCanonicalCoordinate)`
      * `[ ]`   Collaborators: none called; fixture `build_bls12_381_arkworks_pairing`; the untrusted bytes from `vector_bytes(G2_NONZERO_PADDING_HEX)`
      * `[ ]`   Arrange: the payload `vector_bytes(G2_NONZERO_PADDING_HEX)`, the generator with one padding byte changed, so a decoder that ignores the padding returns the generator
      * `[ ]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(DecodeG2ErrorReturn::NonCanonicalCoordinate)`
    * `[ ]`   `decode_g2_rejects_a_non_canonical_coordinate`
      * `[ ]`   Contract: a 256-byte payload whose first coordinate is at least the base field modulus → `Err(DecodeG2ErrorReturn::NonCanonicalCoordinate)`
      * `[ ]`   Collaborators: arkworks' field reduction, run for real as the vendor; fixture `build_bls12_381_arkworks_pairing`; the untrusted bytes from `vector_bytes(G2_X_C0_AT_MODULUS_HEX)`
      * `[ ]`   Arrange: the payload `vector_bytes(G2_X_C0_AT_MODULUS_HEX)`, whose reduced coordinate would otherwise fail the curve check, so a decoder that checks the curve first returns a different error
      * `[ ]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(DecodeG2ErrorReturn::NonCanonicalCoordinate)`
    * `[ ]`   `decode_g2_decodes_the_identity_from_zero_bytes`
      * `[ ]`   Contract: a 256-byte payload with every coordinate zero → `Ok(DecodeG2SuccessReturn { point })` holding the second group's identity
      * `[ ]`   Collaborators: arkworks, run for real as the vendor; fixture `build_bls12_381_arkworks_pairing`; the untrusted bytes from `zero_bytes`
      * `[ ]`   Arrange: the payload `zero_bytes(256)`
      * `[ ]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[ ]`   Assert: the success arm is extracted with `expect`; `success.point.value` equals `G2Affine::identity()`
    * `[ ]`   `decode_g2_rejects_a_point_off_the_curve`
      * `[ ]`   Contract: a canonical 256-byte payload that satisfies no curve equation → `Err(DecodeG2ErrorReturn::NotOnCurve)`
      * `[ ]`   Collaborators: arkworks' curve check, run for real as the vendor; fixture `build_bls12_381_arkworks_pairing`; the untrusted bytes from `vector_bytes(G2_GENERATOR_OFF_CURVE_HEX)`
      * `[ ]`   Arrange: the payload `vector_bytes(G2_GENERATOR_OFF_CURVE_HEX)`
      * `[ ]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(DecodeG2ErrorReturn::NotOnCurve)`
    * `[ ]`   `decode_g2_rejects_a_point_outside_the_subgroup`
      * `[ ]`   Contract: a canonical 256-byte payload on the curve and outside the prime-order subgroup → `Err(DecodeG2ErrorReturn::NotInSubgroup)`
      * `[ ]`   Collaborators: arkworks' subgroup check, run for real as the vendor; fixture `build_bls12_381_arkworks_pairing`; the untrusted bytes from `g2_outside_subgroup_bytes()`
      * `[ ]`   Arrange: the payload `g2_outside_subgroup_bytes()`
      * `[ ]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(DecodeG2ErrorReturn::NotInSubgroup)`
    * `[ ]`   `decode_g2_decodes_the_eip_2537_generator`
      * `[ ]`   Contract: a canonical 256-byte payload on the curve and in the subgroup → `Ok(DecodeG2SuccessReturn { point })` holding that point
      * `[ ]`   Collaborators: arkworks, run for real as the vendor; fixture `build_bls12_381_arkworks_pairing`; the untrusted bytes from `vector_bytes(G2_GENERATOR_HEX)`
      * `[ ]`   Arrange: the payload `vector_bytes(G2_GENERATOR_HEX)`
      * `[ ]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[ ]`   Assert: the success arm is extracted with `expect`; `success.point.value` equals `eip_2537_g2_generator()`
    * `[ ]`   `decode_scalar_rejects_a_wrong_length`
      * `[ ]`   Contract: `payload.len() != 32` → `Err(DecodeScalarErrorReturn::WrongLength { expected: 32, actual: payload.len() })`
      * `[ ]`   Collaborators: none called; fixture `build_bls12_381_arkworks_pairing`; the untrusted bytes from `zero_bytes`
      * `[ ]`   Arrange: the payload `zero_bytes(31)`, which differs from the required length
      * `[ ]`   Act: `decode_scalar(DecodeScalarParams, &payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(DecodeScalarErrorReturn::WrongLength { expected: 32, actual: 31 })`, the whole expected error
    * `[ ]`   `decode_scalar_rejects_the_group_order`
      * `[ ]`   Contract: a 32-byte payload at least the group order → `Err(DecodeScalarErrorReturn::NonCanonical)`
      * `[ ]`   Collaborators: arkworks' field reduction, run for real as the vendor; fixture `build_bls12_381_arkworks_pairing`; the untrusted bytes from `vector_bytes(GROUP_ORDER_HEX)`
      * `[ ]`   Arrange: the payload `vector_bytes(GROUP_ORDER_HEX)`, whose reduction is zero and so differs from the bytes
      * `[ ]`   Act: `decode_scalar(DecodeScalarParams, &payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(DecodeScalarErrorReturn::NonCanonical)`
    * `[ ]`   `decode_scalar_decodes_the_largest_canonical_scalar`
      * `[ ]`   Contract: a canonical 32-byte payload → `Ok(DecodeScalarSuccessReturn { scalar })` holding that scalar
      * `[ ]`   Collaborators: arkworks' field reduction, run for real as the vendor; fixture `build_bls12_381_arkworks_pairing`; the untrusted bytes from `vector_bytes(GROUP_ORDER_MINUS_ONE_HEX)`
      * `[ ]`   Arrange: the payload `vector_bytes(GROUP_ORDER_MINUS_ONE_HEX)`
      * `[ ]`   Act: `decode_scalar(DecodeScalarParams, &payload)`
      * `[ ]`   Assert: the success arm is extracted with `expect`; `success.scalar.value` equals `-Fr::from(1u64)`
    * `[ ]`   `encode_g1_writes_the_eip_2537_generator`
      * `[ ]`   Contract: a first-group point → `Ok(EncodeG1SuccessReturn { bytes })` holding `x` then `y`, each 16 zero bytes followed by 48 bytes big-endian, in this concrete's `Bls12381ArkworksEncodedG1`
      * `[ ]`   Collaborators: arkworks' coordinate encoding, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_encode_g1_payload`, and `build_bls12_381_arkworks_g1`
      * `[ ]`   Arrange: `build_encode_g1_payload` with the point override `build_bls12_381_arkworks_g1` at its default generator
      * `[ ]`   Act: `encode_g1(EncodeG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.bytes` binds to a `Bls12381ArkworksEncodedG1` typed binding; its bytes equal `vector_bytes(G1_GENERATOR_HEX)` as a byte slice
    * `[ ]`   `encode_g1_writes_the_identity_as_zero_bytes`
      * `[ ]`   Contract: the first group's identity → `Ok(EncodeG1SuccessReturn { bytes })` holding 128 zero bytes
      * `[ ]`   Collaborators: arkworks, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_encode_g1_payload`, and `build_bls12_381_arkworks_g1`
      * `[ ]`   Arrange: `build_encode_g1_payload` with the point override `build_bls12_381_arkworks_g1` with the value `G1Affine::identity()`
      * `[ ]`   Act: `encode_g1(EncodeG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; the bytes of `success.bytes` equal `zero_bytes(128)` as a byte slice
    * `[ ]`   `encode_g2_writes_the_eip_2537_generator`
      * `[ ]`   Contract: a second-group point → `Ok(EncodeG2SuccessReturn { bytes })` holding `x.c0`, `x.c1`, `y.c0`, `y.c1`, each 16 zero bytes followed by 48 bytes big-endian, in this concrete's `Bls12381ArkworksEncodedG2`
      * `[ ]`   Collaborators: arkworks' coordinate encoding, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_encode_g2_payload`, and `build_bls12_381_arkworks_g2`
      * `[ ]`   Arrange: `build_encode_g2_payload` with the point override `build_bls12_381_arkworks_g2` at its default generator
      * `[ ]`   Act: `encode_g2(EncodeG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.bytes` binds to a `Bls12381ArkworksEncodedG2` typed binding; its bytes equal `vector_bytes(G2_GENERATOR_HEX)` as a byte slice
    * `[ ]`   `encode_g2_writes_the_identity_as_zero_bytes`
      * `[ ]`   Contract: the second group's identity → `Ok(EncodeG2SuccessReturn { bytes })` holding 256 zero bytes
      * `[ ]`   Collaborators: arkworks, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_encode_g2_payload`, and `build_bls12_381_arkworks_g2`
      * `[ ]`   Arrange: `build_encode_g2_payload` with the point override `build_bls12_381_arkworks_g2` with the value `G2Affine::identity()`
      * `[ ]`   Act: `encode_g2(EncodeG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; the bytes of `success.bytes` equal `zero_bytes(256)` as a byte slice
    * `[ ]`   `encode_scalar_writes_the_largest_canonical_scalar`
      * `[ ]`   Contract: a scalar → `Ok(EncodeScalarSuccessReturn { bytes })` holding its 32 big-endian bytes in this concrete's `Bls12381ArkworksEncodedScalar`, inside a `Secret`
      * `[ ]`   Collaborators: arkworks' integer encoding, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_encode_scalar_payload`, and `build_bls12_381_arkworks_scalar`
      * `[ ]`   Arrange: `build_encode_scalar_payload` with the scalar override `build_bls12_381_arkworks_scalar` with the value `-Fr::from(1u64)`
      * `[ ]`   Act: `encode_scalar(EncodeScalarParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.bytes.expose()` binds to a `Bls12381ArkworksEncodedScalar` typed binding; its bytes equal `vector_bytes(GROUP_ORDER_MINUS_ONE_HEX)` as a byte slice
    * `[ ]`   `sample_from_uniform_bytes_rejects_a_wrong_length`
      * `[ ]`   Contract: `payload.uniform.expose().len() != 64` → `Err(SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual })`
      * `[ ]`   Collaborators: `Secret::expose`, real; fixtures `build_sample_uniform_scalar_payload` and `build_secret`
      * `[ ]`   Arrange: `build_sample_uniform_scalar_payload` with the uniform override `build_secret` holding `zero_bytes(63)`
      * `[ ]`   Act: `Bls12381ArkworksScalar::sample_from_uniform_bytes(SampleUniformScalarParams, payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual: 63 })`, the whole expected error
    * `[ ]`   `sample_from_uniform_bytes_reads_the_input_as_a_big_endian_integer`
      * `[ ]`   Contract: a 64-byte uniform input → `Ok(SampleUniformScalarSuccessReturn { scalar })` holding the input read as one big-endian integer reduced modulo the group order, inside a `Secret`
      * `[ ]`   Collaborators: `Secret::expose` and arkworks' modular reduction, real; fixtures `build_sample_uniform_scalar_payload` and `build_secret`
      * `[ ]`   Arrange: `build_sample_uniform_scalar_payload` with the uniform override `build_secret` holding `vector_bytes(UNIFORM_FIVE_HEX)`, whose last byte is five, so an input read as little-endian yields a different scalar
      * `[ ]`   Act: `Bls12381ArkworksScalar::sample_from_uniform_bytes(SampleUniformScalarParams, payload)`
      * `[ ]`   Assert: the success arm is extracted with `expect`; `success.scalar.expose().value` equals `Fr::from(5u64)`
    * `[ ]`   `sample_from_uniform_bytes_reduces_the_group_order_to_zero`
      * `[ ]`   Contract: a 64-byte uniform input → `Ok(SampleUniformScalarSuccessReturn { scalar })` holding the input reduced modulo the group order, inside a `Secret`
      * `[ ]`   Collaborators: `Secret::expose` and arkworks' modular reduction, real; fixtures `build_sample_uniform_scalar_payload` and `build_secret`
      * `[ ]`   Arrange: `build_sample_uniform_scalar_payload` with the uniform override `build_secret` holding `vector_bytes(UNIFORM_GROUP_ORDER_HEX)`, whose reduction is zero and so differs from the input
      * `[ ]`   Act: `Bls12381ArkworksScalar::sample_from_uniform_bytes(SampleUniformScalarParams, payload)`
      * `[ ]`   Assert: the success arm is extracted with `expect`; `success.scalar.expose().value` equals `Fr::from(0u64)`
    * `[ ]`   `add_scalar_of_two_and_three_is_five`
      * `[ ]`   Contract: `payload.left` and `payload.right` → `Ok(AddScalarSuccessReturn { sum })` holding their sum modulo the group order
      * `[ ]`   Collaborators: arkworks' `Fr` addition, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_add_scalar_payload`, and `build_bls12_381_arkworks_scalar`
      * `[ ]`   Arrange: `build_add_scalar_payload` with the left override `build_bls12_381_arkworks_scalar` with the value `Fr::from(2u64)` and the right override with the value `Fr::from(3u64)`
      * `[ ]`   Act: `add_scalar(AddScalarParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `Fr::from(5u64)`
    * `[ ]`   `add_scalar_reduces_modulo_the_group_order`
      * `[ ]`   Contract: `payload.left` and `payload.right` → `Ok(AddScalarSuccessReturn { sum })` holding their sum modulo the group order
      * `[ ]`   Collaborators: arkworks' `Fr` addition, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_add_scalar_payload`, and `build_bls12_381_arkworks_scalar`
      * `[ ]`   Arrange: `build_add_scalar_payload` with the left override `build_bls12_381_arkworks_scalar` with the value `scalar_value(GROUP_ORDER_MINUS_ONE_HEX)` and the right override with the value `Fr::from(2u64)`, whose integer sum exceeds the group order
      * `[ ]`   Act: `add_scalar(AddScalarParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `Fr::from(1u64)`
    * `[ ]`   `mul_scalar_of_two_and_three_is_six`
      * `[ ]`   Contract: `payload.left` and `payload.right` → `Ok(MulScalarSuccessReturn { product })` holding their product modulo the group order
      * `[ ]`   Collaborators: arkworks' `Fr` multiplication, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_mul_scalar_payload`, and `build_bls12_381_arkworks_scalar`
      * `[ ]`   Arrange: `build_mul_scalar_payload` with the left override `build_bls12_381_arkworks_scalar` with the value `Fr::from(2u64)` and the right override with the value `Fr::from(3u64)`
      * `[ ]`   Act: `mul_scalar(MulScalarParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `Fr::from(6u64)`
    * `[ ]`   `mul_scalar_reduces_modulo_the_group_order`
      * `[ ]`   Contract: `payload.left` and `payload.right` → `Ok(MulScalarSuccessReturn { product })` holding their product modulo the group order
      * `[ ]`   Collaborators: arkworks' `Fr` multiplication, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_mul_scalar_payload`, and `build_bls12_381_arkworks_scalar`
      * `[ ]`   Arrange: `build_mul_scalar_payload` with the left override `build_bls12_381_arkworks_scalar` with the value `scalar_value(GROUP_ORDER_MINUS_ONE_HEX)` and the right override with the value `Fr::from(2u64)`, whose integer product exceeds the group order
      * `[ ]`   Act: `mul_scalar(MulScalarParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `scalar_value(GROUP_ORDER_MINUS_TWO_HEX)`
    * `[ ]`   `neg_scalar_of_one_is_the_group_order_minus_one`
      * `[ ]`   Contract: `payload.scalar` → `Ok(NegScalarSuccessReturn { negation })` holding the group order minus the scalar
      * `[ ]`   Collaborators: arkworks' `Fr` negation, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_neg_scalar_payload`, and `build_bls12_381_arkworks_scalar`
      * `[ ]`   Arrange: `build_neg_scalar_payload` with the scalar override `build_bls12_381_arkworks_scalar` with the value `Fr::from(1u64)`
      * `[ ]`   Act: `neg_scalar(NegScalarParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.negation.value` equals `scalar_value(GROUP_ORDER_MINUS_ONE_HEX)`
    * `[ ]`   `neg_scalar_of_zero_is_zero`
      * `[ ]`   Contract: `payload.scalar` of zero → `Ok(NegScalarSuccessReturn { negation })` holding zero
      * `[ ]`   Collaborators: arkworks' `Fr` negation, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_neg_scalar_payload`, and `build_bls12_381_arkworks_scalar`
      * `[ ]`   Arrange: `build_neg_scalar_payload` with the scalar override `build_bls12_381_arkworks_scalar` with the value `Fr::from(0u64)`
      * `[ ]`   Act: `neg_scalar(NegScalarParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.negation.value` equals `Fr::from(0u64)`
    * `[ ]`   `neg_g1_of_the_generator_is_the_negated_generator`
      * `[ ]`   Contract: `payload.point` → `Ok(NegG1SuccessReturn { negation })` holding `(x, p - y)` for a point `(x, y)`
      * `[ ]`   Collaborators: arkworks' affine negation, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_neg_g1_payload`, and `build_bls12_381_arkworks_g1`
      * `[ ]`   Arrange: `build_neg_g1_payload` with the point override `build_bls12_381_arkworks_g1` at its default generator
      * `[ ]`   Act: `neg_g1(NegG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.negation.value` equals `eip_2537_negated_g1_generator()`
    * `[ ]`   `neg_g1_of_the_identity_is_the_identity`
      * `[ ]`   Contract: `payload.point` of the identity → `Ok(NegG1SuccessReturn { negation })` holding the identity
      * `[ ]`   Collaborators: arkworks' affine negation, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_neg_g1_payload`, and `build_bls12_381_arkworks_g1`
      * `[ ]`   Arrange: `build_neg_g1_payload` with the point override `build_bls12_381_arkworks_g1` with the value `G1Affine::identity()`
      * `[ ]`   Act: `neg_g1(NegG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.negation.value` equals `G1Affine::identity()`
    * `[ ]`   `neg_g2_negates_each_y_coefficient_modulo_the_base_field`
      * `[ ]`   Contract: `payload.point` → `Ok(NegG2SuccessReturn { negation })` holding `(x, p - y)` for a point `(x, y)`, each `y` coefficient negated
      * `[ ]`   Collaborators: arkworks' affine negation, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_neg_g2_payload`, and `build_bls12_381_arkworks_g2`
      * `[ ]`   Arrange: `build_neg_g2_payload` with the point override `build_bls12_381_arkworks_g2` at its default generator
      * `[ ]`   Act: `neg_g2(NegG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; the negation's `xy()` has an `x` equal to the generator's `x` from `eip_2537_g2_generator().xy()`; its `y.c0` and `y.c1`, each as a `BigUint`, equal `base_field_modulus()` minus the generator's corresponding coefficient as a `BigUint`
    * `[ ]`   `neg_g2_of_the_identity_is_the_identity`
      * `[ ]`   Contract: `payload.point` of the identity → `Ok(NegG2SuccessReturn { negation })` holding the identity
      * `[ ]`   Collaborators: arkworks' affine negation, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_neg_g2_payload`, and `build_bls12_381_arkworks_g2`
      * `[ ]`   Arrange: `build_neg_g2_payload` with the point override `build_bls12_381_arkworks_g2` with the value `G2Affine::identity()`
      * `[ ]`   Act: `neg_g2(NegG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.negation.value` equals `G2Affine::identity()`
    * `[ ]`   `is_identity_g1_is_true_for_the_identity`
      * `[ ]`   Contract: `payload.point` → `Ok(IsIdentityG1SuccessReturn { is_identity })`, true exactly for the identity
      * `[ ]`   Collaborators: arkworks, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_is_identity_g1_payload`, and `build_bls12_381_arkworks_g1`
      * `[ ]`   Arrange: `build_is_identity_g1_payload` with the point override `build_bls12_381_arkworks_g1` with the value `G1Affine::identity()`
      * `[ ]`   Act: `is_identity_g1(IsIdentityG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_identity` is true
    * `[ ]`   `is_identity_g1_is_false_for_the_generator`
      * `[ ]`   Contract: `payload.point` → `Ok(IsIdentityG1SuccessReturn { is_identity })`, true exactly for the identity
      * `[ ]`   Collaborators: arkworks, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_is_identity_g1_payload`, and `build_bls12_381_arkworks_g1`
      * `[ ]`   Arrange: `build_is_identity_g1_payload` with the point override `build_bls12_381_arkworks_g1` at its default generator
      * `[ ]`   Act: `is_identity_g1(IsIdentityG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_identity` is false
    * `[ ]`   `is_identity_g2_is_true_for_the_identity`
      * `[ ]`   Contract: `payload.point` → `Ok(IsIdentityG2SuccessReturn { is_identity })`, true exactly for the identity
      * `[ ]`   Collaborators: arkworks, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_is_identity_g2_payload`, and `build_bls12_381_arkworks_g2`
      * `[ ]`   Arrange: `build_is_identity_g2_payload` with the point override `build_bls12_381_arkworks_g2` with the value `G2Affine::identity()`
      * `[ ]`   Act: `is_identity_g2(IsIdentityG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_identity` is true
    * `[ ]`   `is_identity_g2_is_false_for_the_generator`
      * `[ ]`   Contract: `payload.point` → `Ok(IsIdentityG2SuccessReturn { is_identity })`, true exactly for the identity
      * `[ ]`   Collaborators: arkworks, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_is_identity_g2_payload`, and `build_bls12_381_arkworks_g2`
      * `[ ]`   Arrange: `build_is_identity_g2_payload` with the point override `build_bls12_381_arkworks_g2` at its default generator
      * `[ ]`   Act: `is_identity_g2(IsIdentityG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_identity` is false
    * `[ ]`   `pairing_product_of_no_terms_is_the_target_group_identity`
      * `[ ]`   Contract: an empty `payload.terms` → `Ok(PairingProductSuccessReturn { product })` holding the target group's identity
      * `[ ]`   Collaborators: arkworks' multi-pairing and target-group exponentiation, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing` and `build_pairing_product_payload`
      * `[ ]`   Arrange: `build_pairing_product_payload` at its default of no terms
      * `[ ]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` is the identity under `is_zero()`
    * `[ ]`   `pairing_product_of_the_generators_is_not_the_target_group_identity`
      * `[ ]`   Contract: a pairing of the generators → `Ok(PairingProductSuccessReturn { product })` holding a value other than the target group's identity
      * `[ ]`   Collaborators: arkworks' multi-pairing and target-group exponentiation, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_pairing_product_payload`, and `build_pairing_product_term`
      * `[ ]`   Arrange: `build_pairing_product_payload` with the terms override holding one `build_pairing_product_term` at its default generators
      * `[ ]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` is not the identity under `is_zero()`
    * `[ ]`   `pairing_product_is_bilinear`
      * `[ ]`   Contract: a term whose first-group element is twice the generator → `Ok(PairingProductSuccessReturn { product })` holding the generators' pairing raised to the power two
      * `[ ]`   Collaborators: arkworks' multi-pairing and target-group exponentiation, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_pairing_product_payload`, `build_pairing_product_term`, and `build_bls12_381_arkworks_g1`; the expectation is `definition_value_power`, the identifier's definition raised to a power in the fixtures, independent of the correction `pairing_product` applies
      * `[ ]`   Arrange: `build_pairing_product_payload` with the terms override holding one `build_pairing_product_term` with the first-group override `build_bls12_381_arkworks_g1` with the value `eip_2537_doubled_g1_generator()` over the second-group generator
      * `[ ]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value.0` equals `definition_value_power(eip_2537_g1_generator(), eip_2537_g2_generator(), 2)`
    * `[ ]`   `pairing_product_multiplies_its_terms`
      * `[ ]`   Contract: `payload.terms` → `Ok(PairingProductSuccessReturn { product })` holding the product of each term's pairing
      * `[ ]`   Collaborators: arkworks' multi-pairing and target-group exponentiation, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_pairing_product_payload`, and `build_pairing_product_term`; the expectation is `definition_value_power`
      * `[ ]`   Arrange: `build_pairing_product_payload` with the terms override holding two `build_pairing_product_term` values at their default generators, so a product over one term differs from the expected square
      * `[ ]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value.0` equals `definition_value_power(eip_2537_g1_generator(), eip_2537_g2_generator(), 2)`
    * `[ ]`   `pairing_product_of_a_pairing_and_its_first_group_negation_is_the_target_group_identity`
      * `[ ]`   Contract: `payload.terms` whose pairings multiply to the target group's identity → `Ok(PairingProductSuccessReturn { product })` holding the identity
      * `[ ]`   Collaborators: arkworks' multi-pairing and target-group exponentiation, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_pairing_product_payload`, `build_pairing_product_term`, and `build_bls12_381_arkworks_g1`
      * `[ ]`   Arrange: `build_pairing_product_payload` with the terms override holding two `build_pairing_product_term` values: the generators, and the first-group override `build_bls12_381_arkworks_g1` with the value `eip_2537_negated_g1_generator()` over the second-group generator
      * `[ ]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` is the identity under `is_zero()`
    * `[ ]`   `pairing_product_of_the_generators_equals_the_definition`
      * `[ ]`   Contract: a pairing of the generators → `Ok(PairingProductSuccessReturn { product })` holding the Miller loop's value raised to the exact exponent `(p^12 - 1) / r`, the value `TargetGroupEncodingIdentifier::Bls12381V1` defines, which is the library's reduced pairing with the cube removed
      * `[ ]`   Collaborators: arkworks' multi-pairing and target-group exponentiation, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_pairing_product_payload`, and `build_pairing_product_term`; the expectation is `definition_value`, the Miller loop over the generators raised to `definition_exponent()` built by `num-bigint`, independent of the correction `pairing_product` applies
      * `[ ]`   Arrange: `build_pairing_product_payload` with the terms override holding one `build_pairing_product_term` at its default generators
      * `[ ]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value.0` equals `definition_value(eip_2537_g1_generator(), eip_2537_g2_generator())`
    * `[ ]`   `encode_gt_writes_the_target_group_identity_with_c0_c0_c0_first`
      * `[ ]`   Contract: a target-group value → `Ok(EncodeGtSuccessReturn { bytes })` holding its twelve coefficients in tower order, `c0.c0.c0` first, each 48 bytes big-endian, inside a `Secret`
      * `[ ]`   Collaborators: arkworks' integer encoding, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_encode_gt_payload`, and `build_bls12_381_arkworks_gt`; the expectation is `gt_identity_encoding()`
      * `[ ]`   Arrange: `build_encode_gt_payload` with the value override `build_bls12_381_arkworks_gt` with the value `PairingOutput(Fq12::ONE)`, the target group's identity
      * `[ ]`   Act: `encode_gt(EncodeGtParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.bytes.expose()` binds to a `Bls12381ArkworksEncodedGt` typed binding; its bytes equal `gt_identity_encoding()` as a byte slice
    * `[ ]`   `encode_gt_writes_the_published_pairing_of_the_generators`
      * `[ ]`   Contract: a target-group value → `Ok(EncodeGtSuccessReturn { bytes })` holding its twelve coefficients in tower order, `c0.c0.c0` through `c1.c2.c1`, each 48 bytes big-endian, inside a `Secret`
      * `[ ]`   Collaborators: arkworks' integer encoding, run for real as the vendor; fixtures `build_bls12_381_arkworks_pairing`, `build_encode_gt_payload`, and `build_bls12_381_arkworks_gt`; the expectation is `CFRG_GENERATOR_PAIRING_HEX`, the pairing of the base points published in the CFRG draft's test-vector appendix
      * `[ ]`   Arrange: `build_encode_gt_payload` with the value override `build_bls12_381_arkworks_gt` with the value `PairingOutput(definition_value(eip_2537_g1_generator(), eip_2537_g2_generator()))`, the identifier's value for the generators, whose twelve coefficients are distinct, so any exchange of two positions changes the bytes
      * `[ ]`   Act: `encode_gt(EncodeGtParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; the bytes of `success.bytes.expose()` equal `vector_bytes(CFRG_GENERATOR_PAIRING_HEX)` as a byte slice
    * `[ ]`   `scalar_field_order_is_the_group_order`
      * `[ ]`   Contract: any call → `Ok(ScalarFieldOrderSuccessReturn { bytes })` holding the group order's 32 big-endian bytes
      * `[ ]`   Collaborators: arkworks' scalar field modulus, run for real as the vendor; fixture `build_bls12_381_arkworks_pairing`; `ScalarFieldOrderParams` and `ScalarFieldOrderPayload` by their production values
      * `[ ]`   Arrange: `build_bls12_381_arkworks_pairing()`
      * `[ ]`   Act: `scalar_field_order(ScalarFieldOrderParams, ScalarFieldOrderPayload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.bytes` equals `vector_bytes(GROUP_ORDER_HEX)`
    * `[ ]`   `g1_outside_subgroup_encoding_encodes_an_on_curve_point_outside_the_subgroup`
      * `[ ]`   Contract: any call → `Ok(G1OutsideSubgroupEncodingSuccessReturn { bytes: Some(bytes) })` holding the precompile encoding of an on-curve point outside the prime-order subgroup, in this concrete's `Bls12381ArkworksEncodedG1`
      * `[ ]`   Collaborators: arkworks' curve and subgroup checks, run for real as the vendor; fixture `build_bls12_381_arkworks_pairing`; `G1OutsideSubgroupEncodingParams` and `G1OutsideSubgroupEncodingPayload` by their production values; the point read back by `g1_point_from_encoding`
      * `[ ]`   Arrange: `build_bls12_381_arkworks_pairing()`
      * `[ ]`   Act: `g1_outside_subgroup_encoding(G1OutsideSubgroupEncodingParams, G1OutsideSubgroupEncodingPayload)`
      * `[ ]`   Assert: the success arm is extracted with `expect`; `success.bytes` is `Some` and its content binds to a `Bls12381ArkworksEncodedG1` typed binding; the point `g1_point_from_encoding` reads from its bytes satisfies `is_on_curve()` and does not satisfy `is_in_correct_subgroup_assuming_on_curve()`
    * `[ ]`   `g1_outside_subgroup_encoding_uses_the_least_first_coordinate`
      * `[ ]`   Contract: any call → the encoded point's `x` is the least `x` of an on-curve point outside the subgroup in an ascending search
      * `[ ]`   Collaborators: arkworks' curve and subgroup checks, run for real as the vendor; fixture `build_bls12_381_arkworks_pairing`; `G1OutsideSubgroupEncodingParams` and `G1OutsideSubgroupEncodingPayload` by their production values; the expectation is `g1_outside_subgroup_bytes()`, the ascending search in the fixtures
      * `[ ]`   Arrange: `build_bls12_381_arkworks_pairing()`
      * `[ ]`   Act: `g1_outside_subgroup_encoding(G1OutsideSubgroupEncodingParams, G1OutsideSubgroupEncodingPayload)`
      * `[ ]`   Assert: the success arm is extracted with `expect`; the first 64 bytes of the encoding, the `x` coordinate, equal the first 64 bytes of `g1_outside_subgroup_bytes()`
    * `[ ]`   `g1_outside_subgroup_encoding_takes_the_lesser_root`
      * `[ ]`   Contract: any call → the encoded point's `y` is the lesser of the two roots sharing its `x`
      * `[ ]`   Collaborators: arkworks' curve and subgroup checks, run for real as the vendor; fixture `build_bls12_381_arkworks_pairing`; `G1OutsideSubgroupEncodingParams` and `G1OutsideSubgroupEncodingPayload` by their production values; the expectation is `base_field_modulus()`
      * `[ ]`   Arrange: `build_bls12_381_arkworks_pairing()`
      * `[ ]`   Act: `g1_outside_subgroup_encoding(G1OutsideSubgroupEncodingParams, G1OutsideSubgroupEncodingPayload)`
      * `[ ]`   Assert: the success arm is extracted with `expect`; the last 48 bytes of the encoding's `y` coordinate, bytes 80 through 127, read as a `BigUint`, are not greater than `base_field_modulus()` minus that value
    * `[ ]`   `g2_outside_subgroup_encoding_encodes_an_on_curve_point_outside_the_subgroup`
      * `[ ]`   Contract: any call → `Ok(G2OutsideSubgroupEncodingSuccessReturn { bytes })` holding the precompile encoding of an on-curve point outside the prime-order subgroup, in this concrete's `Bls12381ArkworksEncodedG2`
      * `[ ]`   Collaborators: arkworks' curve and subgroup checks, run for real as the vendor; fixture `build_bls12_381_arkworks_pairing`; `G2OutsideSubgroupEncodingParams` and `G2OutsideSubgroupEncodingPayload` by their production values; the point read back by `g2_point_from_encoding`
      * `[ ]`   Arrange: `build_bls12_381_arkworks_pairing()`
      * `[ ]`   Act: `g2_outside_subgroup_encoding(G2OutsideSubgroupEncodingParams, G2OutsideSubgroupEncodingPayload)`
      * `[ ]`   Assert: the success arm is extracted with `expect`; `success.bytes` binds to a `Bls12381ArkworksEncodedG2` typed binding; the point `g2_point_from_encoding` reads from its bytes satisfies `is_on_curve()` and does not satisfy `is_in_correct_subgroup_assuming_on_curve()`
    * `[ ]`   `g2_outside_subgroup_encoding_has_a_real_first_coordinate_and_the_lesser_root`
      * `[ ]`   Contract: any call → the encoded point's first coordinate is `(c0, 0)` with the least `c0` of an on-curve point outside the subgroup, and its `y` is the lesser of the two roots, compared by `y.c1` and then `y.c0`
      * `[ ]`   Collaborators: arkworks' curve and subgroup checks, run for real as the vendor; fixture `build_bls12_381_arkworks_pairing`; `G2OutsideSubgroupEncodingParams` and `G2OutsideSubgroupEncodingPayload` by their production values; the expectations are `g2_outside_subgroup_bytes()`, the ascending search in the fixtures, and `base_field_modulus()`
      * `[ ]`   Arrange: `build_bls12_381_arkworks_pairing()`
      * `[ ]`   Act: `g2_outside_subgroup_encoding(G2OutsideSubgroupEncodingParams, G2OutsideSubgroupEncodingPayload)`
      * `[ ]`   Assert: the success arm is extracted with `expect`; the encoding's `x.c1` coordinate, bytes 64 through 127, is all zero; its `x.c0` coordinate, bytes 0 through 63, equals the first 64 bytes of `g2_outside_subgroup_bytes()`; the pair `y.c1`, `y.c0`, each the last 48 bytes of its 64-byte coordinate read as a `BigUint`, is not greater than the pair of their negations, each `base_field_modulus()` minus the coefficient, reduced to zero where the coefficient is zero

  * `[✅]`   `construction`
    * `[✅]`   `Bls12381ArkworksPairing::try_new` is the concrete's only producer, computing its one field, and its only caller is the pairing factory, which reads `Bls12381ArkworksPairing::DECLARATION` before constructing
    * `[✅]`   A group element or scalar is produced only by the adapter's generators, arithmetic, and decoders, or by the scalar's sampling bound; a target-group value only by `pairing_product`; an encoded value only by the adapter's encoders and reference methods; no consumer constructs one from library values

  * `[✅]`   `adapters/pairing/src/bls12_381_arkworks/mod.rs`
    * `[✅]`   Module declarations: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, `pub(crate) mod provides;`, and `#[cfg(test)] mod test;`
    * `[✅]`   Imports: from `crate::factory::provides` every params, payload, return, success-return, and error type of the three traits and the sampling bound, `IPairingAdapter`, `IPairingArithmetic`, `IPairingReference`, `ISampleUniformScalar`, `PAIRING_INTERFACE_VERSION`, `PairingConcrete`, `PairingCurve`, `PairingDeclaration`, `PrecompileEncoding`, `TargetGroupEncodingIdentifier`, and `VerifierGroupArithmetic`; `ark_bls12_381::{Bls12_381, Fq, Fq2, Fr, G1Affine, G1Projective, G2Affine, G2Projective}`; `ark_ec::{AffineRepr, CurveGroup, VariableBaseMSM, pairing::Pairing}`; `ark_ff::{BigInt, BigInteger, Field, PrimeField, Zero}`; `domain::{Secret, SecretConstructorParams}`; from `interface` every type it declares; `zeroize::{Zeroize, ZeroizeOnDrop}`
    * `[✅]`   `fn decode_coordinate(bytes: &[u8]) -> Option<Fq>`, private: `None` when any of `bytes[..16]` is nonzero; `Fq::from_be_bytes_mod_order(&bytes[16..64])` as the coordinate; `None` when `coordinate.into_bigint().to_bytes_be()` differs from `bytes[16..64]`; otherwise `Some(coordinate)`
    * `[✅]`   `impl Bls12381ArkworksPairing` holding `pub const DECLARATION: PairingDeclaration`, the constant the interaction spec states, and `pub fn try_new(params: Bls12381ArkworksPairingConstructorParams) -> Bls12381ArkworksPairingTryNewReturn` realizing the interaction spec's construct branch
    * `[✅]`   `impl IPairingAdapter for Bls12381ArkworksPairing` with `const DECLARATION: PairingDeclaration = Bls12381ArkworksPairing::DECLARATION;`, `const CONCRETE: PairingConcrete = PairingConcrete::Bls12381Arkworks;`, `type Scalar = Bls12381ArkworksScalar;`, `type G1 = Bls12381ArkworksG1;`, `type G2 = Bls12381ArkworksG2;`, `type EncodedG1 = Bls12381ArkworksEncodedG1;`, `type EncodedG2 = Bls12381ArkworksEncodedG2;`, and `type EncodedScalar = Bls12381ArkworksEncodedScalar;`, every method realizing its branches in the interaction spec, the decoders reading each coordinate through `decode_coordinate`
    * `[✅]`   `impl IPairingArithmetic for Bls12381ArkworksPairing` with `type Gt = Bls12381ArkworksGt;` and `type EncodedGt = Bls12381ArkworksEncodedGt;`, every method realizing its branches in the interaction spec, `pairing_product` multiplying the `PairingOutput` by `self.reduced_pairing_correction`
    * `[✅]`   `impl IPairingReference for Bls12381ArkworksPairing`, every method realizing its branches in the interaction spec, each search as `(1u64..).find_map(…)` with the `SearchExhausted` return by `let … else`
    * `[✅]`   `impl AsRef<[u8]>` for `Bls12381ArkworksEncodedG1`, `Bls12381ArkworksEncodedG2`, `Bls12381ArkworksEncodedScalar`, and `Bls12381ArkworksEncodedGt`, each returning `&self.bytes`; `impl Zeroize` for `Bls12381ArkworksEncodedScalar` and `Bls12381ArkworksEncodedGt`, each calling `self.bytes.zeroize()`; no implementation returns a mutable reference to the bytes
    * `[✅]`   `impl Zeroize` and `impl Drop` for `Bls12381ArkworksScalar`, `Bls12381ArkworksG1`, `Bls12381ArkworksG2`, and `Bls12381ArkworksGt`, each calling `self.value.zeroize()`; `impl ZeroizeOnDrop for Bls12381ArkworksScalar {}`, marking the drop behavior the sampling bound requires
    * `[✅]`   `impl ISampleUniformScalar for Bls12381ArkworksScalar` with `const UNIFORM_BYTES_LENGTH: usize = 64;` and `sample_from_uniform_bytes` realizing its branches in the interaction spec, the scalar moved into a `Secret` by `let Ok(scalar) = Secret::try_new(SecretConstructorParams { value });`
    * `[✅]`   No `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`; the correction is computed in `try_new` and read from no configuration; neither the order nor a point is written as a literal

  * `[✅]`   `adapters/pairing/src/bls12_381_arkworks/provides.rs`
    * `[✅]`   `pub(crate) use super::interface::*;` and `#[cfg(test)] pub(crate) use super::mock::*;`, exposing every item of the concrete's interface to the crate and, in the crate's own test build, the concrete's builders, fixtures, and helpers, and nothing beyond them

  * `[✅]`   `directionality`
    * `[✅]`   `bls12_381_arkworks` depends on the `factory` module's surface through `crate::factory::provides`, on `domain`, on `zeroize`, and on the arkworks crates; it depends on no other concrete and no concrete depends on it; the `factory` module depends on no concrete; among repository crates the crate depends on `crates/domain` alone at runtime
    * `[✅]`   `pairing/factory` constructs this concrete, the family form's recorded cycle, and proves its target-group encoding, order, and outside-the-subgroup encodings equal the halo2curves concrete's

  * `[ ]`   `requirements`
    * `[✅]`   `adapters/pairing/Cargo.toml` carries the `ark-bls12-381` dependency, and `ark-bls12-381` is named nowhere in the crate outside `adapters/pairing/src/bls12_381_arkworks`
    * `[✅]`   This concrete returns its own distinct fixed-width encoded types for the first group, the second group, the scalar, and the target group through the family's associated types
    * `[✅]`   `cargo check --all-targets --all-features`, `cargo fmt --check`, and `cargo deny check` complete without error; `cargo clippy --all-targets --all-features` reports nothing beyond the library target's unused-item warnings for the pairing concretes and the BLS12-381 variants, which `pairing/factory` constructs
    * `[ ]`   Every test in `bls12_381_arkworks/test.rs` passes: the generators and their encodings match EIP-2537, the decoders read the generators and the identity and reject a wrong length, a nonzero padding byte, a non-canonical coordinate or scalar, a point off the curve, and a point outside the subgroup in both groups, the arithmetic, multi-scalar multiplication, and pairing checks agree, and sampling rejects a wrong length, reads its input big-endian, and reduces modulo the group order (CR-04, CR-08, CR-09, and CR-10 on BLS12-381 over arkworks; CR-05 for the sampled scalar; CR-07 for the zeroization)
    * `[ ]`   The declaration and concrete-identity compile-time assertions compile, the `ZeroizeOnDrop` compile-time assertion compiles, `bls12_381_arkworks_scalar_zeroize_clears_its_value` passes (CR-07), and `pairing_product_of_the_generators_equals_the_definition`, `encode_gt_writes_the_published_pairing_of_the_generators`, and `reduced_pairing_correction_inverts_three` pass (CR-10, the target-group value fixed by the identifier's definition, executed in the test, and its serialization by the standard's published value)
    * `[ ]`   `scalar_field_order_is_the_group_order`, `g1_outside_subgroup_encoding_encodes_an_on_curve_point_outside_the_subgroup`, `g1_outside_subgroup_encoding_takes_the_lesser_root`, `g2_outside_subgroup_encoding_encodes_an_on_curve_point_outside_the_subgroup`, and `g2_outside_subgroup_encoding_has_a_real_first_coordinate_and_the_lesser_root` pass (CR-10 and CR-11 on BLS12-381 over arkworks)
    * `[✅]`   Code outside `adapters/pairing` naming `Bls12381ArkworksPairing` or anything under `bls12_381_arkworks` fails to compile; the crate's public surface is the `factory` module's `provides`

* `[ ]`   `pairing/bls12_381_halo2curves` **BLS12-381 pairing concrete on halo2curves implementing the pairing family's generic interface, arithmetic trait, and reference trait over the EIP-2537 encodings with subgroup checks on every input and the `Bls12381V1` target-group value, a further concrete beneath the pairing factory**

  * `[✅]`   `objective`
    * `[✅]`   Problem: the harness benchmark compares pairing libraries per curve through the factory, so BLS12-381 needs a concrete over another library that satisfies the family's generic interface exactly as the arkworks BLS12-381 concrete does (CR-10)
    * `[✅]`   Problem: the KEM, the envelope, and the delivery proof consume the pairing family's arithmetic trait through whichever concrete the factory resolves, and the bytes the KEM hands the KDF are the identifier's target-group value, so this concrete implements the trait with target-group values equal to the identifier's definition, and therefore to the arkworks BLS12-381 concrete's, under the one identifier both declare; `halo2curves`' BLS12-381 final exponentiation returns the Miller loop's value raised to three times the exact exponent `(p^12 - 1) / r`, the Hayashida–Hayasaka–Teruya chain in its source, so without correction the library's reduced pairing is a different element of the target group than the identifier's (CR-04; CR-07; CR-08; CR-09; CR-10; `docs/research/cryptography.md`'s pairing adapter capability declaration)
    * `[✅]`   Problem: every concrete a consumer receives returns the group order and the family's outside-the-subgroup encodings, and the libraries of one curve return the same bytes, so this concrete implements the family's reference trait under the family's rule from its own library (CR-10 on chain; CR-11; the dependency map's `pairing/bn254_arkworks` row; the Pairing adapters and key derivation milestone's exit)
    * `[✅]`   Functional: the concrete implements `IPairingAdapter` over `halo2curves`' BLS12-381, with its own scalar, group-element, and encoded types over the library's elements as the associated types
    * `[✅]`   Functional: it encodes and decodes a base field element as EIP-2537's 64 bytes, sixteen zero bytes followed by the 48-byte big-endian integer, a first-group point as 128 bytes of `x` then `y`, a second-group point as 256 bytes of `x.c0`, `x.c1`, `y.c0`, `y.c1`, the point at infinity as all zero bytes, and a scalar as 32 big-endian bytes
    * `[✅]`   Functional: decoding rejects a wrong length, a nonzero padding byte, a field element at or above the modulus, a point off the curve, and a point outside the prime-order subgroup in either group
    * `[✅]`   Functional: its scalar type implements the family's sampling bound, reading the 64 uniform bytes as one big-endian integer reduced modulo the group order, so the same input bytes sample the same scalar under either BLS12-381 concrete
    * `[✅]`   Functional: it declares the BLS12-381 curve, second-group arithmetic at the verifier, the EIP-2537 encoding, `TargetGroupEncodingIdentifier::Bls12381V1`, its adapter version, and the interface version it implements, and names `PairingConcrete::Bls12381Halo2curves`
    * `[✅]`   Functional: `Bls12381Halo2curvesPairing` implements `IPairingArithmetic` with every method's behavior as `pairing/bn254_arkworks` states it for the trait; its `pairing_product` returns the identifier's exact value: the library's reduced pairing multiplied in the target group by `reduced_pairing_correction`, the inverse of three in the scalar field, computed once in `try_new` by Fermat with the group order minus two as little-endian limbs read from the library's own field arithmetic
    * `[✅]`   Functional: the target-group encoding reads the `Fq12` through `Gt::inner` and writes its twelve coefficients in the tower order `c0.c0.c0`, `c0.c0.c1`, `c0.c1.c0`, `c0.c1.c1`, `c0.c2.c0`, `c0.c2.c1`, `c1.c0.c0`, `c1.c0.c1`, `c1.c1.c0`, `c1.c1.c1`, `c1.c2.c0`, `c1.c2.c1`, each 48 bytes big-endian, 576 bytes in all, over halo2curves' BLS12-381 tower `Fq2 = Fq[u] / (u^2 + 1)`, `Fq6 = Fq2[v] / (v^3 - (u + 1))`, `Fq12 = Fq6[w] / (w^2 - v)`
    * `[✅]`   Functional: the concrete's tests execute the identifier's definition, the library's Miller loop over the two generators raised to the integer `(p^12 - 1) / r` built by `num-bigint` from the library's moduli and serialized in the tower order, and assert the pairing of the base points the CFRG draft publishes in its test-vector appendix, requiring `pairing_product` then `encode_gt` over the generators to yield those bytes
    * `[✅]`   Functional: `Bls12381Halo2curvesScalar`, `Bls12381Halo2curvesG1`, `Bls12381Halo2curvesG2`, and `Bls12381Halo2curvesGt` clear their value through `Zeroize` and on drop
    * `[✅]`   Functional: the concrete returns BLS12-381's group order as 32 big-endian bytes and each group's point under the family's rule, encoded as `encode_g1` and `encode_g2` encode any point, each refused by its own decoder as outside the subgroup
    * `[✅]`   Non-functional: `halo2curves` is named only inside the halo2curves concretes; no order and no point is written as a literal outside a test

  * `[✅]`   `role`
    * `[✅]`   Adapter: a further concrete of the pairing family, implementing the generic interface, the sampling bound, the arithmetic trait, and the reference trait `pairing/bn254_arkworks` authors, declaring the identifier it authors, and using the BLS12-381 declaration variants `pairing/bls12_381_arkworks` adds
    * `[✅]`   Contributes its module line to `lib.rs`; the crate manifest's `halo2curves` dependency is `pairing/bn254_halo2curves`'s, and no file of the `factory` module changes
    * `[✅]`   Does not select between the BLS12-381 concretes; `pairing/factory` constructs a concrete by the composition's request and `harness-crypto/benchmark` records the default per curve
    * `[✅]`   Does not name, import, or compare against another concrete; `pairing/factory`'s integration test proves agreement with the arkworks concrete
    * `[✅]`   Does not read the correction from configuration; it is a property of this library under this identifier and is computed in `try_new`
    * `[✅]`   Does not carry a commit

  * `[✅]`   `module`
    * `[✅]`   Bounded context: the private `bls12_381_halo2curves` module of `adapters/pairing`, holding the adapter over `halo2curves`' BLS12-381 with its correction field, its scalar, group-element, target-group, and encoded types over the library's elements, its constructor params, its implementations of the three traits, its declared identifier, its definition and published-vector tests, its clearing, and the builder defaults for its owned types
    * `[✅]`   The node's files: the `adapters/pairing/src/lib.rs` module line and the `bls12_381_halo2curves` module's `interface.rs`, `interaction.spec.md`, `mock.rs`, `test.rs`, `mod.rs`, and `provides.rs`
    * `[✅]`   Outside: the family's traits and declaration, every other concrete, and the factory's selection and admission

  * `[ ]`   `deps`
    * `[✅]`   The `factory` module's surface, through `crate::factory::provides`: `IPairingAdapter`, `IPairingArithmetic`, `IPairingReference`, `ISampleUniformScalar`, `PairingDeclaration` and its enums, `PairingConcrete`, `TargetGroupEncodingIdentifier`, `PAIRING_INTERFACE_VERSION`, and every params, payload, return, success-return, error type, and builder of the three traits and the sampling bound, as `pairing/bn254_arkworks` states them
    * `[✅]`   `domain`, runtime: `Secret` and `SecretConstructorParams`; `build_secret` and `SecretConstructorParamsOverrides` through the `mocks` feature
    * `[✅]`   `zeroize` `1.9.0`, runtime: the `Zeroize` and `ZeroizeOnDrop` traits and the zeroization of the local 64-byte input copy
    * `[✅]`   `halo2curves` `0.10.0`, the crate's runtime dependency as `pairing/bn254_halo2curves` states it, resolved to the `gt-accessor` overlay, which supplies `Gt::inner(&self) -> &Fq12`; supplies the curve, the pairing, the group arithmetic, the field arithmetic, and multi-scalar multiplication, and re-exports the `ff`, `group`, and `pairing` traits its types implement
    * `[ ]`   `hex` `0.4.3` and `num-bigint` `0.4.8`, the crate's dev-dependencies, for the test vectors and the integer exponent in `mock.rs`'s test-only fixtures and the root comparison in `test.rs`
    * `[✅]`   `core::convert::Infallible`, standard library, the constructor's error arm; `core::array::from_fn`, standard library, the Fermat exponent's limbs; `core::hint::black_box`, standard library, which keeps each clearing from being removed as a dead store, since `halo2curves`' fields, points, and target-group values implement no `Zeroize`; `core::iter::successors`, standard library, the ascending search
    * `[✅]`   Reverse dependency: `pairing/factory`

  * `[ ]`   `context_slice`
    * `[✅]`   From `halo2curves::bls12381`: `Bls12381`, the engine; `Fq`, whose representation is 48 bytes; `Fq2` with `Fq2::new(c0, c1)` and the accessors `c0()` and `c1()`; `Fq12` with `c0()` and `c1()` and the degree-six accessors `c0()`, `c1()`, and `c2()`; `Fr`, whose representation is 32 bytes; `G1`, `G1Affine`, `G2`, `G2Affine`; and `Gt` with `Gt::identity()`, `Gt::inner()`, `PartialEq`, and `Mul<&Fr> for &Gt`, the exponentiation of a target-group value by a scalar
    * `[✅]`   From `halo2curves::ff`: `Fr`'s `+`, `*`, and unary `-` modulo the group order; `Fr::from(u64)`; `Field` for `ZERO`, `ONE`, `is_zero()`, `square()`, `sqrt()` returning a `CtOption`, `invert()` returning a `CtOption`, and `pow_vartime(&self, exp: impl AsRef<[u64]>)`, exponentiation by little-endian limbs, on `Fr` and on `Fq12`; unary `-` on `Fq` and `Fq2` and binary `+` on `Fq2`; `PrimeField` for `from_repr(repr) -> CtOption<Self>`, which reads little-endian bytes and is none at or above the modulus, `to_repr()`, which writes little-endian bytes read by `as_ref()`, 48 for `Fq` and 32 for `Fr`, and `MODULUS`, the `&'static str` hex modulus with the `0x` prefix, on `Fq` and on `Fr`; `FromUniformBytes::<64>::from_uniform_bytes(&[u8; 64])` on `Fr`, a little-endian wide reduction
    * `[✅]`   From `halo2curves::group`: `Curve::to_affine`, `Group::is_identity`, `prime::PrimeCurveAffine` for `generator()`, `identity()`, `is_identity()` returning a `Choice`, and `to_curve()`, `cofactor::CofactorGroup::is_torsion_free` on `G1` and on `G2`, and the projective `+` and `* Fr`; unary `-` on `G1Affine` and `G2Affine`
    * `[✅]`   From `halo2curves`: `CurveAffine` for `from_xy(x, y) -> CtOption<Self>`, which is none off the curve, `coordinates() -> CtOption<Coordinates<Self>>`, and `b()`; `Coordinates` for `x()` and `y()`; `msm::msm_best(coeffs: &[C::Scalar], bases: &[C]) -> C::Curve`
    * `[✅]`   From `halo2curves::pairing`: `MultiMillerLoop::multi_miller_loop(&[(&G1Affine, &G2Affine)])` on `Bls12381`, returning the unreduced `Fq12`, `MillerLoopResult::final_exponentiation` returning `Gt`, whose `is_identity()` is the check, and `Engine::pairing(&G1Affine, &G2Affine)` for the builder default
    * `[✅]`   A `Choice` becomes a `bool` by `bool::from`, and a `CtOption` becomes an `Option` by `Option::from`; `u64::from(u8)` and `u64`'s `<<` and `|` fold a little-endian byte group into a limb
    * `[ ]`   From `num-bigint`, in `mock.rs`'s test-only fixtures and in `test.rs`: `BigUint::parse_bytes(&[u8], u32) -> Option<BigUint>`, `BigUint::from(u32)`, `BigUint::from_bytes_be(&[u8])`, `BigUint::pow(&self, u32)`, `Sub`, `Div`, and `Rem` between `BigUint`s, and `to_u64_digits()`
    * `[✅]`   From `domain`: `Secret::try_new(SecretConstructorParams { value })` returning `Result<Secret<T>, Infallible>`, and `Secret::expose(&self) -> &T`

  * `[✅]`   `adapters/pairing/src/lib.rs`
    * `[✅]`   The barrel holds `mod bls12_381_halo2curves;`

  * `[✅]`   `adapters/pairing/src/bls12_381_halo2curves/interface.rs`
    * `[✅]`   Imports `core::convert::Infallible`; declares nothing beyond the items below
    * `[✅]`   `Bls12381Halo2curvesPairing`, a struct with no derives and one field, `pub(super) reduced_pairing_correction: halo2curves::bls12381::Fr`, with the doc comment "The inverse of three in the scalar field, the multiple by which the library's reduced pairing exceeds the identifier's exact value."
    * `[✅]`   `Bls12381Halo2curvesPairingConstructorParams`, a unit struct; `Bls12381Halo2curvesPairingTryNewReturn`, the alias `Result<Bls12381Halo2curvesPairing, Infallible>`, the error arm uninhabited because the adapter takes no configuration
    * `[✅]`   `Bls12381Halo2curvesScalar`, `#[derive(Clone)]`, with one field `pub(super) value: halo2curves::bls12381::Fr`; `Bls12381Halo2curvesG1`, `#[derive(Clone)]`, with one field `pub(super) value: halo2curves::bls12381::G1Affine`; `Bls12381Halo2curvesG2`, `#[derive(Clone)]`, with one field `pub(super) value: halo2curves::bls12381::G2Affine`
    * `[✅]`   `Bls12381Halo2curvesGt`, a struct with no derives and one field, `pub(super) value: halo2curves::bls12381::Gt`, the target-group value
    * `[✅]`   `Bls12381Halo2curvesEncodedG1` and `Bls12381Halo2curvesEncodedG2`, `pub struct`s with `#[derive(Clone, PartialEq, Eq)]` and one field each, `pub(super) bytes: [u8; 128]` and `pub(super) bytes: [u8; 256]`; `Bls12381Halo2curvesEncodedScalar`, a `pub struct` with no derives and one field `pub(super) bytes: [u8; 32]`; `Bls12381Halo2curvesEncodedGt`, a `pub struct` with no derives and one field `pub(super) bytes: [u8; 576]`; the scalar and target-group encodings are returned only inside a `Secret`, and no encoded type has a public constructor or mutable byte access

  * `[✅]`   `adapters/pairing/src/bls12_381_halo2curves/interaction.spec.md`
    * `[✅]`   The title `` # `bls12_381_halo2curves` — interaction spec `` and one opening sentence stating the file as the branch contract for the `bls12_381_halo2curves` module of the `pairing` crate, the pairing adapter over halo2curves' BLS12-381 encoding group elements as EIP-2537 precompile input, each branch stating condition, decision, dependency call, and the exact return outcome; then one `##` section per constructor, constant, and method, each headed by its full signature and holding a table with the columns `Branch`, `Condition`, `Decision`, `Dependency call`, and `Outcome`, in the section order `pairing/bn254_arkworks` states for its spec, then `Ordering and edges`
    * `[✅]`   `try_new`: one branch, construct; condition any params; decision none; dependency calls, in order: `(-Fr::from(2u64)).to_repr()`, the group order minus two as 32 little-endian bytes, read into four little-endian limbs by `core::array::from_fn`, each limb folding its eight bytes from the most significant by `(limb << 8) | u64::from(byte)`; then `Fr::from(3u64).pow_vartime(limbs)` as `reduced_pairing_correction`, the inverse of three by Fermat; outcome `Ok(Bls12381Halo2curvesPairing { reduced_pairing_correction })`; the error arm has no branch, `Infallible` being uninhabited
    * `[✅]`   `DECLARATION`: the inherent constant `PairingDeclaration { curve: PairingCurve::Bls12381, verifier_group_arithmetic: VerifierGroupArithmetic::BothGroups, precompile_encoding: PrecompileEncoding::Eip2537, target_group_encoding: TargetGroupEncodingIdentifier::Bls12381V1, adapter_version: 1, interface_version: PAIRING_INTERFACE_VERSION }`, readable from the type before any instance exists; the trait constant `IPairingAdapter::DECLARATION` is this constant
    * `[✅]`   `CONCRETE`: the trait constant `PairingConcrete::Bls12381Halo2curves`
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

  * `[ ]`   `adapters/pairing/src/bls12_381_halo2curves/mock.rs`
    * `[ ]`   The module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`; imports `super::interface::{Bls12381Halo2curvesEncodedGt, Bls12381Halo2curvesEncodedScalar, Bls12381Halo2curvesG1, Bls12381Halo2curvesG2, Bls12381Halo2curvesGt, Bls12381Halo2curvesPairing, Bls12381Halo2curvesPairingConstructorParams, Bls12381Halo2curvesScalar}`, `core::iter::successors`, `halo2curves::CurveAffine`, `halo2curves::bls12381::{Bls12381, Fq, Fq2, Fq12, Fr, G1Affine, G2Affine, Gt}`, `halo2curves::ff::{Field, PrimeField}`, `halo2curves::group::{cofactor::CofactorGroup, prime::PrimeCurveAffine}`, and `halo2curves::pairing::{Engine, MultiMillerLoop}`; and, `#[cfg(test)]`, `halo2curves::group::Curve`, `hex::decode`, `num_bigint::BigUint`, and `zeroize::ZeroizeOnDrop`
    * `[✅]`   `impl Default for Bls12381Halo2curvesScalar` returning `value: Fr::ONE`; `impl Default for Bls12381Halo2curvesG1` returning `value: G1Affine::generator()`; `impl Default for Bls12381Halo2curvesG2` returning `value: G2Affine::generator()`; `impl Default for Bls12381Halo2curvesGt` returning `value: Bls12381::pairing(&G1Affine::generator(), &G2Affine::generator())`, a non-identity target-group value; the family's generic builders and `MockIPairingAdapter` read these through `Default`
    * `[ ]`   The builders for the concrete's owned types, each overrides struct `#[derive(Default)]` with one `Option` field and each omitted value taking the type's `Default` above: `Bls12381Halo2curvesScalarOverrides` with `pub value: Option<Fr>` and `build_bls12_381_halo2curves_scalar`; `Bls12381Halo2curvesG1Overrides` with `pub value: Option<G1Affine>` and `build_bls12_381_halo2curves_g1`; `Bls12381Halo2curvesG2Overrides` with `pub value: Option<G2Affine>` and `build_bls12_381_halo2curves_g2`; `Bls12381Halo2curvesGtOverrides` with `pub value: Option<Gt>` and `build_bls12_381_halo2curves_gt`; no corruptions type and no invalidator, since none of these arrives as untrusted data
    * `[ ]`   `build_bls12_381_halo2curves_pairing() -> Bls12381Halo2curvesPairing`, the real instance from `Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams)` through `let Ok(pairing) = …;`; the constructor params are fieldless, so the builder takes no overrides
    * `[ ]`   The builders for the concrete's encoded scalar and encoded target-group types, each overrides struct `#[derive(Default)]` with one `Option` field: `Bls12381Halo2curvesEncodedScalarOverrides` with `pub bytes: Option<[u8; 32]>` and `build_bls12_381_halo2curves_encoded_scalar(overrides: Bls12381Halo2curvesEncodedScalarOverrides) -> Bls12381Halo2curvesEncodedScalar`, the bytes defaulting to `[1u8; 32]`; `Bls12381Halo2curvesEncodedGtOverrides` with `pub bytes: Option<[u8; 576]>` and `build_bls12_381_halo2curves_encoded_gt`, the bytes defaulting to `[1u8; 576]`; the defaults are nonzero so a zeroized value differs from a built one; no corruptions type and no invalidator, since neither arrives as untrusted data
    * `[ ]`   The test fixtures, each `#[cfg(test)]`, since they use the crate's dev-dependencies: the constants `pairing/bls12_381_arkworks` states in its `mock.rs`, by the same names and values; the helpers `vector_bytes`, `zero_bytes`, `base_field_modulus`, `gt_identity_encoding`, and `requires_zeroize_on_drop` as `pairing/bls12_381_arkworks` states them; `scalar_value(hex: &str) -> Fr` as `pairing/bn254_halo2curves` states it; `g1_point_from_encoding(bytes: &[u8]) -> Option<G1Affine>`, `G1Affine::from_xy` over the two 64-byte coordinates of `bytes`, each its last 48 bytes copied into a `[u8; 48]`, reversed to little-endian, and read by `Fq::from_repr`, `None` off the curve; `g2_point_from_encoding(bytes: &[u8]) -> Option<G2Affine>`, `G2Affine::from_xy(Fq2::new(x_c0, x_c1), Fq2::new(y_c0, y_c1))` over the four 64-byte coordinates of `bytes`, read in order as `x_c0`, `x_c1`, `y_c0`, `y_c1` the same way; `eip_2537_g1_generator() -> G1Affine`, `eip_2537_negated_g1_generator() -> G1Affine`, and `eip_2537_g2_generator() -> G2Affine`, the point `g1_point_from_encoding` or `g2_point_from_encoding` returns over `vector_bytes(G1_GENERATOR_HEX)`, `vector_bytes(NEG_G1_GENERATOR_HEX)`, or `vector_bytes(G2_GENERATOR_HEX)`, unpacked by `let Some(point) = … else { panic!("the published point is on the curve") };`; `g1_outside_subgroup_bytes() -> Vec<u8>`, the least on-curve point outside the subgroup, `successors(Some(Fq::ONE), |x| Some(*x + Fq::ONE)).find_map(|x| Option::<Fq>::from((x.square() * x + G1Affine::b()).sqrt()).and_then(|y| Option::<G1Affine>::from(G1Affine::from_xy(x, y)).filter(|point| !bool::from(point.to_curve().is_torsion_free())).map(|_| (x, y))))` unpacked by `let Some((x, y)) = … else { panic!("an on-curve point outside the subgroup exists") };`, encoded as 16 zero bytes, `x.to_repr().as_ref().iter().rev()`, 16 zero bytes, and `y.to_repr().as_ref().iter().rev()`; `g2_outside_subgroup_bytes() -> Vec<u8>`, the least on-curve point outside the subgroup, `successors(Some(Fq::ONE), |c0| Some(*c0 + Fq::ONE)).find_map(|c0| { let x = Fq2::new(c0, Fq::ZERO); Option::<Fq2>::from((x.square() * x + G2Affine::b()).sqrt()).and_then(|y| Option::<G2Affine>::from(G2Affine::from_xy(x, y)).filter(|point| !bool::from(point.to_curve().is_torsion_free())).map(|_| (x, y))) })` unpacked the same way, encoded as `x.c0()`, `x.c1()`, `y.c0()`, `y.c1()`, each 16 zero bytes followed by `to_repr().as_ref().iter().rev()`; `definition_exponent() -> Vec<u64>`, `let Some(p) = BigUint::parse_bytes(Fq::MODULUS.trim_start_matches("0x").as_bytes(), 16) else { panic!("the base field modulus parses") }; let Some(r) = BigUint::parse_bytes(Fr::MODULUS.trim_start_matches("0x").as_bytes(), 16) else { panic!("the group order parses") }; ((p.pow(12) - BigUint::from(1u32)) / r).to_u64_digits()`; `definition_value(g1: G1Affine, g2: G2Affine) -> Fq12`, `Bls12381::multi_miller_loop(&[(&g1, &g2)]).pow_vartime(definition_exponent())`, the identifier's definition of the pairing value; `definition_value_power(g1: G1Affine, g2: G2Affine, exponent: u64) -> Fq12`, `definition_value(g1, g2).pow_vartime([exponent])`; `eip_2537_doubled_g1_generator() -> G1Affine`, `(eip_2537_g1_generator().to_curve() + eip_2537_g1_generator().to_curve()).to_affine()`; `corrected_generator_pairing() -> Gt`, the library's reduced pairing of the generators, `Bls12381::pairing(&G1Affine::generator(), &G2Affine::generator())`, exponentiated by the inverse of three, `&pairing * &inverse` with `inverse` the `Option::from` of `Fr::from(3u64).invert()` unpacked by `let Some(inverse) = … else { panic!("three is invertible") };`, the identifier's exact value as a `Gt`
    * `[ ]`   No mock function: the concrete is built as a real instance and owns no free function

  * `[ ]`   `adapters/pairing/src/bls12_381_halo2curves/test.rs`
    * `[ ]`   Module-level `#![allow(clippy::expect_used)]`; imports from `super::provides` the concrete, its owned types, and the mock's builders, constants, and helpers each block uses; from `crate::factory::provides` the params, payloads, errors, trait names, and builders each block uses; and the `halo2curves`, `num_bigint::BigUint`, and `zeroize::Zeroize` names each block uses
    * `[ ]`   Compile-time assertion over `Bls12381Halo2curvesPairing::DECLARATION`
      * `[ ]`   Contract: the inherent constant is `PairingDeclaration { curve: PairingCurve::Bls12381, verifier_group_arithmetic: VerifierGroupArithmetic::BothGroups, precompile_encoding: PrecompileEncoding::Eip2537, target_group_encoding: TargetGroupEncodingIdentifier::Bls12381V1, adapter_version: 1, interface_version: PAIRING_INTERFACE_VERSION }`, readable before any instance exists
      * `[ ]`   Arrange: the type alone
      * `[ ]`   Act: a module-level `const _: () = assert!(…);` reading `Bls12381Halo2curvesPairing::DECLARATION`, each enum field tested with `matches!` and each version with `==`
      * `[ ]`   Assert: the module compiles only if every field holds
    * `[ ]`   Compile-time assertion over `<Bls12381Halo2curvesPairing as IPairingAdapter>::DECLARATION`
      * `[ ]`   Contract: the trait constant carries the same fields as the inherent constant
      * `[ ]`   Arrange: the type alone
      * `[ ]`   Act: a module-level `const _: () = assert!(…);` reading the trait constant, each enum field tested with `matches!` and each version with `==`
      * `[ ]`   Assert: the module compiles only if every field holds
    * `[ ]`   Compile-time assertion over `<Bls12381Halo2curvesPairing as IPairingAdapter>::CONCRETE`
      * `[ ]`   Contract: the trait constant is `PairingConcrete::Bls12381Halo2curves`
      * `[ ]`   Arrange: the type alone
      * `[ ]`   Act: a module-level `const _: () = assert!(…);` reading the trait constant under `matches!`
      * `[ ]`   Assert: the module compiles only if the constant is `PairingConcrete::Bls12381Halo2curves`
    * `[ ]`   `bls12_381_halo2curves_scalar_is_zeroize_on_drop`
      * `[ ]`   Contract: `Bls12381Halo2curvesScalar` implements `ZeroizeOnDrop`, the marker the sampling bound requires
      * `[ ]`   Collaborators: none
      * `[ ]`   Arrange: the type alone
      * `[ ]`   Act: `requires_zeroize_on_drop::<Bls12381Halo2curvesScalar>()`
      * `[ ]`   Assert: the block compiles only if the type implements `ZeroizeOnDrop`
    * `[ ]`   `bls12_381_halo2curves_scalar_zeroize_clears_its_value`
      * `[ ]`   Contract: `Zeroize::zeroize` on a living scalar → its `value` is zero
      * `[ ]`   Collaborators: none; fixture `build_bls12_381_halo2curves_scalar`
      * `[ ]`   Arrange: `build_bls12_381_halo2curves_scalar` with the value override `Fr::from(5u64)`, so a scalar left unchanged differs from the expected zero
      * `[ ]`   Act: `scalar.zeroize()`
      * `[ ]`   Assert: `scalar.value` equals `Fr::ZERO`
    * `[ ]`   `bls12_381_halo2curves_g1_zeroize_clears_its_value`
      * `[ ]`   Contract: `Zeroize::zeroize` on a living first-group element → its `value` is the identity
      * `[ ]`   Collaborators: none; fixture `build_bls12_381_halo2curves_g1`
      * `[ ]`   Arrange: `build_bls12_381_halo2curves_g1` at its default generator, which differs from the identity
      * `[ ]`   Act: `g1.zeroize()`
      * `[ ]`   Assert: `g1.value` equals `G1Affine::identity()`
    * `[ ]`   `bls12_381_halo2curves_g2_zeroize_clears_its_value`
      * `[ ]`   Contract: `Zeroize::zeroize` on a living second-group element → its `value` is the identity
      * `[ ]`   Collaborators: none; fixture `build_bls12_381_halo2curves_g2`
      * `[ ]`   Arrange: `build_bls12_381_halo2curves_g2` at its default generator, which differs from the identity
      * `[ ]`   Act: `g2.zeroize()`
      * `[ ]`   Assert: `g2.value` equals `G2Affine::identity()`
    * `[ ]`   `bls12_381_halo2curves_gt_zeroize_clears_its_value`
      * `[ ]`   Contract: `Zeroize::zeroize` on a living target-group value → its `value` is the target group's identity
      * `[ ]`   Collaborators: none; fixture `build_bls12_381_halo2curves_gt`
      * `[ ]`   Arrange: `build_bls12_381_halo2curves_gt` at its default pairing of the generators, which differs from the identity
      * `[ ]`   Act: `gt.zeroize()`
      * `[ ]`   Assert: `gt.value` equals `Gt::identity()`
    * `[ ]`   `bls12_381_halo2curves_encoded_scalar_zeroize_clears_its_bytes`
      * `[ ]`   Contract: `Zeroize::zeroize` on a living encoded scalar → every byte is zero
      * `[ ]`   Collaborators: none; fixture `build_bls12_381_halo2curves_encoded_scalar`
      * `[ ]`   Arrange: `build_bls12_381_halo2curves_encoded_scalar` at its nonzero default
      * `[ ]`   Act: `encoded.zeroize()`
      * `[ ]`   Assert: the bytes of `encoded` equal `zero_bytes(32)` as a byte slice
    * `[ ]`   `bls12_381_halo2curves_encoded_gt_zeroize_clears_its_bytes`
      * `[ ]`   Contract: `Zeroize::zeroize` on a living encoded target-group value → every byte is zero
      * `[ ]`   Collaborators: none; fixture `build_bls12_381_halo2curves_encoded_gt`
      * `[ ]`   Arrange: `build_bls12_381_halo2curves_encoded_gt` at its nonzero default
      * `[ ]`   Act: `encoded.zeroize()`
      * `[ ]`   Assert: the bytes of `encoded` equal `zero_bytes(576)` as a byte slice
    * `[ ]`   `reduced_pairing_correction_inverts_three`
      * `[ ]`   Contract: any params → `Ok(Bls12381Halo2curvesPairing)` whose `reduced_pairing_correction` is the inverse of three in the scalar field
      * `[ ]`   Collaborators: halo2curves' `Fr` arithmetic, run for real as the vendor; `Bls12381Halo2curvesPairingConstructorParams` by its production value
      * `[ ]`   Arrange: `Bls12381Halo2curvesPairingConstructorParams` by its production value
      * `[ ]`   Act: `Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(pairing) = …;`; `pairing.reduced_pairing_correction * Fr::from(3u64)` equals `Fr::ONE`
    * `[ ]`   `uniform_bytes_length_is_twice_the_group_order_width`
      * `[ ]`   Contract: `UNIFORM_BYTES_LENGTH` is twice the byte width of the group order
      * `[ ]`   Collaborators: none
      * `[ ]`   Arrange: the type alone
      * `[ ]`   Act: reading `<Bls12381Halo2curvesScalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH`
      * `[ ]`   Assert: the constant equals the literal 64 written in the assertion
    * `[ ]`   `g1_generator_returns_the_eip_2537_generator`
      * `[ ]`   Contract: any call → `Ok(G1GeneratorSuccessReturn { point })` holding the first group's generator
      * `[ ]`   Collaborators: halo2curves, run for real as the vendor; fixture `build_bls12_381_halo2curves_pairing`; `G1GeneratorParams` and `G1GeneratorPayload` by their production values
      * `[ ]`   Arrange: `build_bls12_381_halo2curves_pairing()`
      * `[ ]`   Act: `g1_generator(G1GeneratorParams, G1GeneratorPayload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.point.value` equals `eip_2537_g1_generator()`
    * `[ ]`   `g2_generator_returns_the_eip_2537_generator`
      * `[ ]`   Contract: any call → `Ok(G2GeneratorSuccessReturn { point })` holding the second group's generator
      * `[ ]`   Collaborators: halo2curves, run for real as the vendor; fixture `build_bls12_381_halo2curves_pairing`; `G2GeneratorParams` and `G2GeneratorPayload` by their production values
      * `[ ]`   Arrange: `build_bls12_381_halo2curves_pairing()`
      * `[ ]`   Act: `g2_generator(G2GeneratorParams, G2GeneratorPayload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.point.value` equals `eip_2537_g2_generator()`
    * `[ ]`   `add_g1_of_a_point_and_its_negation_is_the_identity`
      * `[ ]`   Contract: `payload.left` and `payload.right` → `Ok(AddG1SuccessReturn { sum })` holding their sum
      * `[ ]`   Collaborators: halo2curves' projective addition, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_add_g1_payload`, and `build_bls12_381_halo2curves_g1`
      * `[ ]`   Arrange: `build_add_g1_payload` with the left override `build_bls12_381_halo2curves_g1` at its default generator and the right override `build_bls12_381_halo2curves_g1` with the value `eip_2537_negated_g1_generator()`, so a sum that returns either input differs from the identity
      * `[ ]`   Act: `add_g1(AddG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G1Affine::identity()`
    * `[ ]`   `add_g1_of_the_identity_and_a_point_is_the_point`
      * `[ ]`   Contract: `payload.left` and `payload.right` → `Ok(AddG1SuccessReturn { sum })` holding their sum
      * `[ ]`   Collaborators: halo2curves' projective addition, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_add_g1_payload`, and `build_bls12_381_halo2curves_g1`
      * `[ ]`   Arrange: `build_add_g1_payload` with the left override `build_bls12_381_halo2curves_g1` with the value `G1Affine::identity()` and the right override `build_bls12_381_halo2curves_g1` at its default generator, so a sum that returns the left input differs from the generator
      * `[ ]`   Act: `add_g1(AddG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `eip_2537_g1_generator()`
    * `[ ]`   `add_g2_of_a_point_and_its_negation_is_the_identity`
      * `[ ]`   Contract: `payload.left` and `payload.right` → `Ok(AddG2SuccessReturn { sum })` holding their sum
      * `[ ]`   Collaborators: halo2curves' projective addition, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_add_g2_payload`, and `build_bls12_381_halo2curves_g2`
      * `[ ]`   Arrange: `build_add_g2_payload` with the left override `build_bls12_381_halo2curves_g2` at its default generator and the right override `build_bls12_381_halo2curves_g2` with the value `-eip_2537_g2_generator()`, so a sum that returns either input differs from the identity
      * `[ ]`   Act: `add_g2(AddG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G2Affine::identity()`
    * `[ ]`   `add_g2_of_the_identity_and_a_point_is_the_point`
      * `[ ]`   Contract: `payload.left` and `payload.right` → `Ok(AddG2SuccessReturn { sum })` holding their sum
      * `[ ]`   Collaborators: halo2curves' projective addition, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_add_g2_payload`, and `build_bls12_381_halo2curves_g2`
      * `[ ]`   Arrange: `build_add_g2_payload` with the left override `build_bls12_381_halo2curves_g2` with the value `G2Affine::identity()` and the right override `build_bls12_381_halo2curves_g2` at its default generator, so a sum that returns the left input differs from the generator
      * `[ ]`   Act: `add_g2(AddG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `eip_2537_g2_generator()`
    * `[ ]`   `mul_g1_by_one_is_the_point`
      * `[ ]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG1SuccessReturn { product })` holding the point multiplied by the scalar
      * `[ ]`   Collaborators: halo2curves' projective scalar multiplication, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_mul_g1_payload`, `build_bls12_381_halo2curves_g1`, and `build_bls12_381_halo2curves_scalar`
      * `[ ]`   Arrange: `build_mul_g1_payload` with the point override `build_bls12_381_halo2curves_g1` at its default generator and the scalar override `build_bls12_381_halo2curves_scalar` with the value `Fr::from(1u64)`
      * `[ ]`   Act: `mul_g1(MulG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `eip_2537_g1_generator()`
    * `[ ]`   `mul_g1_by_zero_is_the_identity`
      * `[ ]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG1SuccessReturn { product })` holding the point multiplied by the scalar
      * `[ ]`   Collaborators: halo2curves' projective scalar multiplication, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_mul_g1_payload`, `build_bls12_381_halo2curves_g1`, and `build_bls12_381_halo2curves_scalar`
      * `[ ]`   Arrange: `build_mul_g1_payload` with the point override `build_bls12_381_halo2curves_g1` at its default generator and the scalar override `build_bls12_381_halo2curves_scalar` with the value `Fr::ZERO`, so a product that returns the point differs from the identity
      * `[ ]`   Act: `mul_g1(MulG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `G1Affine::identity()`
    * `[ ]`   `mul_g1_by_the_group_order_minus_one_is_the_negated_generator`
      * `[ ]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG1SuccessReturn { product })` holding the point multiplied by the scalar
      * `[ ]`   Collaborators: halo2curves' projective scalar multiplication, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_mul_g1_payload`, `build_bls12_381_halo2curves_g1`, and `build_bls12_381_halo2curves_scalar`
      * `[ ]`   Arrange: `build_mul_g1_payload` with the point override `build_bls12_381_halo2curves_g1` at its default generator and the scalar override `build_bls12_381_halo2curves_scalar` with the value `scalar_value(GROUP_ORDER_MINUS_ONE_HEX)`, so a product that ignores the scalar differs from the negated generator
      * `[ ]`   Act: `mul_g1(MulG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `eip_2537_negated_g1_generator()`
    * `[ ]`   `mul_g2_by_one_is_the_point`
      * `[ ]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG2SuccessReturn { product })` holding the point multiplied by the scalar
      * `[ ]`   Collaborators: halo2curves' projective scalar multiplication, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_mul_g2_payload`, `build_bls12_381_halo2curves_g2`, and `build_bls12_381_halo2curves_scalar`
      * `[ ]`   Arrange: `build_mul_g2_payload` with the point override `build_bls12_381_halo2curves_g2` at its default generator and the scalar override `build_bls12_381_halo2curves_scalar` with the value `Fr::from(1u64)`
      * `[ ]`   Act: `mul_g2(MulG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `eip_2537_g2_generator()`
    * `[ ]`   `mul_g2_by_zero_is_the_identity`
      * `[ ]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG2SuccessReturn { product })` holding the point multiplied by the scalar
      * `[ ]`   Collaborators: halo2curves' projective scalar multiplication, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_mul_g2_payload`, `build_bls12_381_halo2curves_g2`, and `build_bls12_381_halo2curves_scalar`
      * `[ ]`   Arrange: `build_mul_g2_payload` with the point override `build_bls12_381_halo2curves_g2` at its default generator and the scalar override `build_bls12_381_halo2curves_scalar` with the value `Fr::ZERO`, so a product that returns the point differs from the identity
      * `[ ]`   Act: `mul_g2(MulG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `G2Affine::identity()`
    * `[ ]`   `mul_g2_by_the_group_order_minus_one_is_the_negated_point`
      * `[ ]`   Contract: `payload.point` and `payload.scalar` → `Ok(MulG2SuccessReturn { product })` holding the point multiplied by the scalar
      * `[ ]`   Collaborators: halo2curves' projective scalar multiplication, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_mul_g2_payload`, `build_bls12_381_halo2curves_g2`, and `build_bls12_381_halo2curves_scalar`
      * `[ ]`   Arrange: `build_mul_g2_payload` with the point override `build_bls12_381_halo2curves_g2` at its default generator and the scalar override `build_bls12_381_halo2curves_scalar` with the value `scalar_value(GROUP_ORDER_MINUS_ONE_HEX)`, so a product that ignores the scalar differs from the negated generator
      * `[ ]`   Act: `mul_g2(MulG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `-eip_2537_g2_generator()`
    * `[ ]`   `msm_g1_of_no_terms_is_the_identity`
      * `[ ]`   Contract: an empty `payload.terms` → `Ok(MsmG1SuccessReturn { sum })` holding the identity
      * `[ ]`   Collaborators: halo2curves' `msm_best`, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing` and `build_msm_g1_payload`
      * `[ ]`   Arrange: `build_msm_g1_payload` at its default of no terms
      * `[ ]`   Act: `msm_g1(MsmG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G1Affine::identity()`
    * `[ ]`   `msm_g1_pairs_each_base_with_its_own_scalar`
      * `[ ]`   Contract: `payload.terms` → `Ok(MsmG1SuccessReturn { sum })` holding the sum of each base multiplied by its own scalar
      * `[ ]`   Collaborators: halo2curves' `msm_best`, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_msm_g1_payload`, `build_msm_g1_term`, `build_bls12_381_halo2curves_g1`, and `build_bls12_381_halo2curves_scalar`
      * `[ ]`   Arrange: `build_msm_g1_payload` with the terms override holding two `build_msm_g1_term` values: the generator with the scalar `Fr::from(1u64)`, and the base `eip_2537_negated_g1_generator()` with the scalar `Fr::ZERO`, so bases and scalars exchanged between terms yield the negated generator
      * `[ ]`   Act: `msm_g1(MsmG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `eip_2537_g1_generator()`
    * `[ ]`   `msm_g1_sums_its_terms`
      * `[ ]`   Contract: `payload.terms` → `Ok(MsmG1SuccessReturn { sum })` holding the sum of each base multiplied by its own scalar
      * `[ ]`   Collaborators: halo2curves' `msm_best`, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_msm_g1_payload`, `build_msm_g1_term`, `build_bls12_381_halo2curves_g1`, and `build_bls12_381_halo2curves_scalar`
      * `[ ]`   Arrange: `build_msm_g1_payload` with the terms override holding two `build_msm_g1_term` values: the generator with the scalar `Fr::from(1u64)`, and the base `eip_2537_negated_g1_generator()` with the scalar `Fr::from(1u64)`, so a sum over the first term alone differs from the identity
      * `[ ]`   Act: `msm_g1(MsmG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G1Affine::identity()`
    * `[ ]`   `msm_g2_of_no_terms_is_the_identity`
      * `[ ]`   Contract: an empty `payload.terms` → `Ok(MsmG2SuccessReturn { sum })` holding the identity
      * `[ ]`   Collaborators: halo2curves' `msm_best`, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing` and `build_msm_g2_payload`
      * `[ ]`   Arrange: `build_msm_g2_payload` at its default of no terms
      * `[ ]`   Act: `msm_g2(MsmG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G2Affine::identity()`
    * `[ ]`   `msm_g2_pairs_each_base_with_its_own_scalar`
      * `[ ]`   Contract: `payload.terms` → `Ok(MsmG2SuccessReturn { sum })` holding the sum of each base multiplied by its own scalar
      * `[ ]`   Collaborators: halo2curves' `msm_best`, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_msm_g2_payload`, `build_msm_g2_term`, `build_bls12_381_halo2curves_g2`, and `build_bls12_381_halo2curves_scalar`
      * `[ ]`   Arrange: `build_msm_g2_payload` with the terms override holding two `build_msm_g2_term` values: the generator with the scalar `Fr::from(1u64)`, and the base `-eip_2537_g2_generator()` with the scalar `Fr::ZERO`, so bases and scalars exchanged between terms yield the negated generator
      * `[ ]`   Act: `msm_g2(MsmG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `eip_2537_g2_generator()`
    * `[ ]`   `msm_g2_sums_its_terms`
      * `[ ]`   Contract: `payload.terms` → `Ok(MsmG2SuccessReturn { sum })` holding the sum of each base multiplied by its own scalar
      * `[ ]`   Collaborators: halo2curves' `msm_best`, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_msm_g2_payload`, `build_msm_g2_term`, `build_bls12_381_halo2curves_g2`, and `build_bls12_381_halo2curves_scalar`
      * `[ ]`   Arrange: `build_msm_g2_payload` with the terms override holding two `build_msm_g2_term` values: the generator with the scalar `Fr::from(1u64)`, and the base `-eip_2537_g2_generator()` with the scalar `Fr::from(1u64)`, so a sum over the first term alone differs from the identity
      * `[ ]`   Act: `msm_g2(MsmG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `G2Affine::identity()`
    * `[ ]`   `pairing_product_is_one_of_no_terms_is_true`
      * `[ ]`   Contract: an empty `payload.terms` → `Ok(PairingProductIsOneSuccessReturn { is_one })` with `is_one` true
      * `[ ]`   Collaborators: halo2curves' multi-Miller loop and final exponentiation, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing` and `build_pairing_product_is_one_payload`
      * `[ ]`   Arrange: `build_pairing_product_is_one_payload` at its default of no terms
      * `[ ]`   Act: `pairing_product_is_one(PairingProductIsOneParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_one` is true
    * `[ ]`   `pairing_product_is_one_of_a_pairing_and_its_first_group_negation_is_true`
      * `[ ]`   Contract: `payload.terms` whose pairings multiply to the target group's identity → `Ok(PairingProductIsOneSuccessReturn { is_one })` with `is_one` true
      * `[ ]`   Collaborators: halo2curves' multi-Miller loop and final exponentiation, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_pairing_product_is_one_payload`, `build_pairing_product_term`, and `build_bls12_381_halo2curves_g1`
      * `[ ]`   Arrange: `build_pairing_product_is_one_payload` with the terms override holding two `build_pairing_product_term` values: the generators, and the first-group override `build_bls12_381_halo2curves_g1` with the value `eip_2537_negated_g1_generator()` over the second-group generator
      * `[ ]`   Act: `pairing_product_is_one(PairingProductIsOneParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_one` is true
    * `[ ]`   `pairing_product_is_one_of_the_generators_is_false`
      * `[ ]`   Contract: `payload.terms` whose pairings multiply to a value other than the target group's identity → `Ok(PairingProductIsOneSuccessReturn { is_one })` with `is_one` false
      * `[ ]`   Collaborators: halo2curves' multi-Miller loop and final exponentiation, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_pairing_product_is_one_payload`, and `build_pairing_product_term`
      * `[ ]`   Arrange: `build_pairing_product_is_one_payload` with the terms override holding one `build_pairing_product_term` at its default generators
      * `[ ]`   Act: `pairing_product_is_one(PairingProductIsOneParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_one` is false
    * `[ ]`   `decode_g1_rejects_a_wrong_length`
      * `[ ]`   Contract: `payload.len() != 128` → `Err(DecodeG1ErrorReturn::WrongLength { expected: 128, actual: payload.len() })`
      * `[ ]`   Collaborators: none called; fixture `build_bls12_381_halo2curves_pairing`; the untrusted bytes from `zero_bytes`
      * `[ ]`   Arrange: the payload `zero_bytes(127)`, which differs from the required length
      * `[ ]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(DecodeG1ErrorReturn::WrongLength { expected: 128, actual: 127 })`, the whole expected error
    * `[ ]`   `decode_g1_rejects_a_nonzero_padding_byte`
      * `[ ]`   Contract: a 128-byte payload with a nonzero byte among a coordinate's first 16 → `Err(DecodeG1ErrorReturn::NonCanonicalCoordinate)`
      * `[ ]`   Collaborators: none called; fixture `build_bls12_381_halo2curves_pairing`; the untrusted bytes from `vector_bytes(G1_NONZERO_PADDING_HEX)`
      * `[ ]`   Arrange: the payload `vector_bytes(G1_NONZERO_PADDING_HEX)`, the generator with one padding byte changed, so a decoder that ignores the padding returns the generator
      * `[ ]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(DecodeG1ErrorReturn::NonCanonicalCoordinate)`
    * `[ ]`   `decode_g1_rejects_a_non_canonical_coordinate`
      * `[ ]`   Contract: a 128-byte payload whose first coordinate is at least the base field modulus → `Err(DecodeG1ErrorReturn::NonCanonicalCoordinate)`
      * `[ ]`   Collaborators: halo2curves' `Fq::from_repr`, run for real as the vendor; fixture `build_bls12_381_halo2curves_pairing`; the untrusted bytes from `vector_bytes(G1_X_AT_MODULUS_HEX)`
      * `[ ]`   Arrange: the payload `vector_bytes(G1_X_AT_MODULUS_HEX)`, whose coordinates would otherwise fail the curve check, so a decoder that checks the curve first returns a different error
      * `[ ]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(DecodeG1ErrorReturn::NonCanonicalCoordinate)`
    * `[ ]`   `decode_g1_decodes_the_identity_from_zero_bytes`
      * `[ ]`   Contract: a 128-byte payload with both coordinates zero → `Ok(DecodeG1SuccessReturn { point })` holding the first group's identity
      * `[ ]`   Collaborators: halo2curves, run for real as the vendor; fixture `build_bls12_381_halo2curves_pairing`; the untrusted bytes from `zero_bytes`
      * `[ ]`   Arrange: the payload `zero_bytes(128)`
      * `[ ]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[ ]`   Assert: the success arm is extracted with `expect`; `success.point.value` equals `G1Affine::identity()`
    * `[ ]`   `decode_g1_rejects_a_point_off_the_curve`
      * `[ ]`   Contract: a canonical 128-byte payload that satisfies no curve equation → `Err(DecodeG1ErrorReturn::NotOnCurve)`
      * `[ ]`   Collaborators: halo2curves' `from_xy`, run for real as the vendor; fixture `build_bls12_381_halo2curves_pairing`; the untrusted bytes from `vector_bytes(G1_GENERATOR_OFF_CURVE_HEX)`
      * `[ ]`   Arrange: the payload `vector_bytes(G1_GENERATOR_OFF_CURVE_HEX)`
      * `[ ]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(DecodeG1ErrorReturn::NotOnCurve)`
    * `[ ]`   `decode_g1_rejects_a_point_outside_the_subgroup`
      * `[ ]`   Contract: a canonical 128-byte payload on the curve and outside the prime-order subgroup → `Err(DecodeG1ErrorReturn::NotInSubgroup)`
      * `[ ]`   Collaborators: halo2curves' torsion check, run for real as the vendor; fixture `build_bls12_381_halo2curves_pairing`; the untrusted bytes from `g1_outside_subgroup_bytes()`
      * `[ ]`   Arrange: the payload `g1_outside_subgroup_bytes()`
      * `[ ]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(DecodeG1ErrorReturn::NotInSubgroup)`
    * `[ ]`   `decode_g1_decodes_the_eip_2537_generator`
      * `[ ]`   Contract: a canonical 128-byte payload on the curve and in the subgroup → `Ok(DecodeG1SuccessReturn { point })` holding that point
      * `[ ]`   Collaborators: halo2curves, run for real as the vendor; fixture `build_bls12_381_halo2curves_pairing`; the untrusted bytes from `vector_bytes(G1_GENERATOR_HEX)`
      * `[ ]`   Arrange: the payload `vector_bytes(G1_GENERATOR_HEX)`
      * `[ ]`   Act: `decode_g1(DecodeG1Params, &payload)`
      * `[ ]`   Assert: the success arm is extracted with `expect`; `success.point.value` equals `eip_2537_g1_generator()`
    * `[ ]`   `decode_g2_rejects_a_wrong_length`
      * `[ ]`   Contract: `payload.len() != 256` → `Err(DecodeG2ErrorReturn::WrongLength { expected: 256, actual: payload.len() })`
      * `[ ]`   Collaborators: none called; fixture `build_bls12_381_halo2curves_pairing`; the untrusted bytes from `zero_bytes`
      * `[ ]`   Arrange: the payload `zero_bytes(255)`, which differs from the required length
      * `[ ]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(DecodeG2ErrorReturn::WrongLength { expected: 256, actual: 255 })`, the whole expected error
    * `[ ]`   `decode_g2_rejects_a_nonzero_padding_byte`
      * `[ ]`   Contract: a 256-byte payload with a nonzero byte among a coordinate's first 16 → `Err(DecodeG2ErrorReturn::NonCanonicalCoordinate)`
      * `[ ]`   Collaborators: none called; fixture `build_bls12_381_halo2curves_pairing`; the untrusted bytes from `vector_bytes(G2_NONZERO_PADDING_HEX)`
      * `[ ]`   Arrange: the payload `vector_bytes(G2_NONZERO_PADDING_HEX)`, the generator with one padding byte changed, so a decoder that ignores the padding returns the generator
      * `[ ]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(DecodeG2ErrorReturn::NonCanonicalCoordinate)`
    * `[ ]`   `decode_g2_rejects_a_non_canonical_coordinate`
      * `[ ]`   Contract: a 256-byte payload whose first coordinate is at least the base field modulus → `Err(DecodeG2ErrorReturn::NonCanonicalCoordinate)`
      * `[ ]`   Collaborators: halo2curves' `Fq::from_repr`, run for real as the vendor; fixture `build_bls12_381_halo2curves_pairing`; the untrusted bytes from `vector_bytes(G2_X_C0_AT_MODULUS_HEX)`
      * `[ ]`   Arrange: the payload `vector_bytes(G2_X_C0_AT_MODULUS_HEX)`, whose coordinates would otherwise fail the curve check, so a decoder that checks the curve first returns a different error
      * `[ ]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(DecodeG2ErrorReturn::NonCanonicalCoordinate)`
    * `[ ]`   `decode_g2_decodes_the_identity_from_zero_bytes`
      * `[ ]`   Contract: a 256-byte payload with every coordinate zero → `Ok(DecodeG2SuccessReturn { point })` holding the second group's identity
      * `[ ]`   Collaborators: halo2curves, run for real as the vendor; fixture `build_bls12_381_halo2curves_pairing`; the untrusted bytes from `zero_bytes`
      * `[ ]`   Arrange: the payload `zero_bytes(256)`
      * `[ ]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[ ]`   Assert: the success arm is extracted with `expect`; `success.point.value` equals `G2Affine::identity()`
    * `[ ]`   `decode_g2_rejects_a_point_off_the_curve`
      * `[ ]`   Contract: a canonical 256-byte payload that satisfies no curve equation → `Err(DecodeG2ErrorReturn::NotOnCurve)`
      * `[ ]`   Collaborators: halo2curves' `from_xy`, run for real as the vendor; fixture `build_bls12_381_halo2curves_pairing`; the untrusted bytes from `vector_bytes(G2_GENERATOR_OFF_CURVE_HEX)`
      * `[ ]`   Arrange: the payload `vector_bytes(G2_GENERATOR_OFF_CURVE_HEX)`
      * `[ ]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(DecodeG2ErrorReturn::NotOnCurve)`
    * `[ ]`   `decode_g2_rejects_a_point_outside_the_subgroup`
      * `[ ]`   Contract: a canonical 256-byte payload on the curve and outside the prime-order subgroup → `Err(DecodeG2ErrorReturn::NotInSubgroup)`
      * `[ ]`   Collaborators: halo2curves' torsion check, run for real as the vendor; fixture `build_bls12_381_halo2curves_pairing`; the untrusted bytes from `g2_outside_subgroup_bytes()`
      * `[ ]`   Arrange: the payload `g2_outside_subgroup_bytes()`
      * `[ ]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(DecodeG2ErrorReturn::NotInSubgroup)`
    * `[ ]`   `decode_g2_decodes_the_eip_2537_generator`
      * `[ ]`   Contract: a canonical 256-byte payload on the curve and in the subgroup → `Ok(DecodeG2SuccessReturn { point })` holding that point
      * `[ ]`   Collaborators: halo2curves, run for real as the vendor; fixture `build_bls12_381_halo2curves_pairing`; the untrusted bytes from `vector_bytes(G2_GENERATOR_HEX)`
      * `[ ]`   Arrange: the payload `vector_bytes(G2_GENERATOR_HEX)`
      * `[ ]`   Act: `decode_g2(DecodeG2Params, &payload)`
      * `[ ]`   Assert: the success arm is extracted with `expect`; `success.point.value` equals `eip_2537_g2_generator()`
    * `[ ]`   `decode_scalar_rejects_a_wrong_length`
      * `[ ]`   Contract: `payload.len() != 32` → `Err(DecodeScalarErrorReturn::WrongLength { expected: 32, actual: payload.len() })`
      * `[ ]`   Collaborators: none called; fixture `build_bls12_381_halo2curves_pairing`; the untrusted bytes from `zero_bytes`
      * `[ ]`   Arrange: the payload `zero_bytes(31)`, which differs from the required length
      * `[ ]`   Act: `decode_scalar(DecodeScalarParams, &payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(DecodeScalarErrorReturn::WrongLength { expected: 32, actual: 31 })`, the whole expected error
    * `[ ]`   `decode_scalar_rejects_the_group_order`
      * `[ ]`   Contract: a 32-byte payload at least the group order → `Err(DecodeScalarErrorReturn::NonCanonical)`
      * `[ ]`   Collaborators: halo2curves' `Fr::from_repr`, run for real as the vendor; fixture `build_bls12_381_halo2curves_pairing`; the untrusted bytes from `vector_bytes(GROUP_ORDER_HEX)`
      * `[ ]`   Arrange: the payload `vector_bytes(GROUP_ORDER_HEX)`
      * `[ ]`   Act: `decode_scalar(DecodeScalarParams, &payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(DecodeScalarErrorReturn::NonCanonical)`
    * `[ ]`   `decode_scalar_decodes_the_largest_canonical_scalar`
      * `[ ]`   Contract: a canonical 32-byte payload → `Ok(DecodeScalarSuccessReturn { scalar })` holding that scalar
      * `[ ]`   Collaborators: halo2curves' `Fr::from_repr`, run for real as the vendor; fixture `build_bls12_381_halo2curves_pairing`; the untrusted bytes from `vector_bytes(GROUP_ORDER_MINUS_ONE_HEX)`
      * `[ ]`   Arrange: the payload `vector_bytes(GROUP_ORDER_MINUS_ONE_HEX)`
      * `[ ]`   Act: `decode_scalar(DecodeScalarParams, &payload)`
      * `[ ]`   Assert: the success arm is extracted with `expect`; `success.scalar.value` equals `-Fr::from(1u64)`
    * `[ ]`   `encode_g1_writes_the_eip_2537_generator`
      * `[ ]`   Contract: a first-group point → `Ok(EncodeG1SuccessReturn { bytes })` holding `x` then `y`, each 16 zero bytes followed by 48 bytes big-endian, in this concrete's `Bls12381Halo2curvesEncodedG1`
      * `[ ]`   Collaborators: halo2curves' coordinate encoding, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_encode_g1_payload`, and `build_bls12_381_halo2curves_g1`
      * `[ ]`   Arrange: `build_encode_g1_payload` with the point override `build_bls12_381_halo2curves_g1` at its default generator
      * `[ ]`   Act: `encode_g1(EncodeG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.bytes` binds to a `Bls12381Halo2curvesEncodedG1` typed binding; its bytes equal `vector_bytes(G1_GENERATOR_HEX)` as a byte slice
    * `[ ]`   `encode_g1_writes_the_identity_as_zero_bytes`
      * `[ ]`   Contract: the first group's identity → `Ok(EncodeG1SuccessReturn { bytes })` holding 128 zero bytes
      * `[ ]`   Collaborators: halo2curves, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_encode_g1_payload`, and `build_bls12_381_halo2curves_g1`
      * `[ ]`   Arrange: `build_encode_g1_payload` with the point override `build_bls12_381_halo2curves_g1` with the value `G1Affine::identity()`
      * `[ ]`   Act: `encode_g1(EncodeG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; the bytes of `success.bytes` equal `zero_bytes(128)` as a byte slice
    * `[ ]`   `encode_g2_writes_the_eip_2537_generator`
      * `[ ]`   Contract: a second-group point → `Ok(EncodeG2SuccessReturn { bytes })` holding `x.c0`, `x.c1`, `y.c0`, `y.c1`, each 16 zero bytes followed by 48 bytes big-endian, in this concrete's `Bls12381Halo2curvesEncodedG2`
      * `[ ]`   Collaborators: halo2curves' coordinate encoding, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_encode_g2_payload`, and `build_bls12_381_halo2curves_g2`
      * `[ ]`   Arrange: `build_encode_g2_payload` with the point override `build_bls12_381_halo2curves_g2` at its default generator
      * `[ ]`   Act: `encode_g2(EncodeG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.bytes` binds to a `Bls12381Halo2curvesEncodedG2` typed binding; its bytes equal `vector_bytes(G2_GENERATOR_HEX)` as a byte slice
    * `[ ]`   `encode_g2_writes_the_identity_as_zero_bytes`
      * `[ ]`   Contract: the second group's identity → `Ok(EncodeG2SuccessReturn { bytes })` holding 256 zero bytes
      * `[ ]`   Collaborators: halo2curves, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_encode_g2_payload`, and `build_bls12_381_halo2curves_g2`
      * `[ ]`   Arrange: `build_encode_g2_payload` with the point override `build_bls12_381_halo2curves_g2` with the value `G2Affine::identity()`
      * `[ ]`   Act: `encode_g2(EncodeG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; the bytes of `success.bytes` equal `zero_bytes(256)` as a byte slice
    * `[ ]`   `encode_scalar_writes_the_largest_canonical_scalar`
      * `[ ]`   Contract: a scalar → `Ok(EncodeScalarSuccessReturn { bytes })` holding its 32 big-endian bytes in this concrete's `Bls12381Halo2curvesEncodedScalar`, inside a `Secret`
      * `[ ]`   Collaborators: halo2curves' `to_repr`, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_encode_scalar_payload`, and `build_bls12_381_halo2curves_scalar`
      * `[ ]`   Arrange: `build_encode_scalar_payload` with the scalar override `build_bls12_381_halo2curves_scalar` with the value `-Fr::from(1u64)`
      * `[ ]`   Act: `encode_scalar(EncodeScalarParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.bytes.expose()` binds to a `Bls12381Halo2curvesEncodedScalar` typed binding; its bytes equal `vector_bytes(GROUP_ORDER_MINUS_ONE_HEX)` as a byte slice
    * `[ ]`   `sample_from_uniform_bytes_rejects_a_wrong_length`
      * `[ ]`   Contract: `payload.uniform.expose().len() != 64` → `Err(SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual })`
      * `[ ]`   Collaborators: `Secret::expose`, real; fixtures `build_sample_uniform_scalar_payload` and `build_secret`
      * `[ ]`   Arrange: `build_sample_uniform_scalar_payload` with the uniform override `build_secret` holding `zero_bytes(63)`
      * `[ ]`   Act: `Bls12381Halo2curvesScalar::sample_from_uniform_bytes(SampleUniformScalarParams, payload)`
      * `[ ]`   Assert: the result's `.err()` equals `Some(SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual: 63 })`, the whole expected error
    * `[ ]`   `sample_from_uniform_bytes_reads_the_input_as_a_big_endian_integer`
      * `[ ]`   Contract: a 64-byte uniform input → `Ok(SampleUniformScalarSuccessReturn { scalar })` holding the input read as one big-endian integer reduced modulo the group order, inside a `Secret`
      * `[ ]`   Collaborators: `Secret::expose` and halo2curves' wide reduction, real; fixtures `build_sample_uniform_scalar_payload` and `build_secret`
      * `[ ]`   Arrange: `build_sample_uniform_scalar_payload` with the uniform override `build_secret` holding `vector_bytes(UNIFORM_FIVE_HEX)`, whose last byte is five, so an input read as little-endian yields a different scalar
      * `[ ]`   Act: `Bls12381Halo2curvesScalar::sample_from_uniform_bytes(SampleUniformScalarParams, payload)`
      * `[ ]`   Assert: the success arm is extracted with `expect`; `success.scalar.expose().value` equals `Fr::from(5u64)`
    * `[ ]`   `sample_from_uniform_bytes_reduces_the_group_order_to_zero`
      * `[ ]`   Contract: a 64-byte uniform input → `Ok(SampleUniformScalarSuccessReturn { scalar })` holding the input reduced modulo the group order, inside a `Secret`
      * `[ ]`   Collaborators: `Secret::expose` and halo2curves' wide reduction, real; fixtures `build_sample_uniform_scalar_payload` and `build_secret`
      * `[ ]`   Arrange: `build_sample_uniform_scalar_payload` with the uniform override `build_secret` holding `vector_bytes(UNIFORM_GROUP_ORDER_HEX)`, whose reduction is zero and so differs from the input
      * `[ ]`   Act: `Bls12381Halo2curvesScalar::sample_from_uniform_bytes(SampleUniformScalarParams, payload)`
      * `[ ]`   Assert: the success arm is extracted with `expect`; `success.scalar.expose().value` equals `Fr::ZERO`
    * `[ ]`   `add_scalar_of_two_and_three_is_five`
      * `[ ]`   Contract: `payload.left` and `payload.right` → `Ok(AddScalarSuccessReturn { sum })` holding their sum modulo the group order
      * `[ ]`   Collaborators: halo2curves' `Fr` addition, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_add_scalar_payload`, and `build_bls12_381_halo2curves_scalar`
      * `[ ]`   Arrange: `build_add_scalar_payload` with the left override `build_bls12_381_halo2curves_scalar` with the value `Fr::from(2u64)` and the right override with the value `Fr::from(3u64)`
      * `[ ]`   Act: `add_scalar(AddScalarParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `Fr::from(5u64)`
    * `[ ]`   `add_scalar_reduces_modulo_the_group_order`
      * `[ ]`   Contract: `payload.left` and `payload.right` → `Ok(AddScalarSuccessReturn { sum })` holding their sum modulo the group order
      * `[ ]`   Collaborators: halo2curves' `Fr` addition, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_add_scalar_payload`, and `build_bls12_381_halo2curves_scalar`
      * `[ ]`   Arrange: `build_add_scalar_payload` with the left override `build_bls12_381_halo2curves_scalar` with the value `scalar_value(GROUP_ORDER_MINUS_ONE_HEX)` and the right override with the value `Fr::from(2u64)`, whose integer sum exceeds the group order
      * `[ ]`   Act: `add_scalar(AddScalarParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.sum.value` equals `Fr::from(1u64)`
    * `[ ]`   `mul_scalar_of_two_and_three_is_six`
      * `[ ]`   Contract: `payload.left` and `payload.right` → `Ok(MulScalarSuccessReturn { product })` holding their product modulo the group order
      * `[ ]`   Collaborators: halo2curves' `Fr` multiplication, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_mul_scalar_payload`, and `build_bls12_381_halo2curves_scalar`
      * `[ ]`   Arrange: `build_mul_scalar_payload` with the left override `build_bls12_381_halo2curves_scalar` with the value `Fr::from(2u64)` and the right override with the value `Fr::from(3u64)`
      * `[ ]`   Act: `mul_scalar(MulScalarParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `Fr::from(6u64)`
    * `[ ]`   `mul_scalar_reduces_modulo_the_group_order`
      * `[ ]`   Contract: `payload.left` and `payload.right` → `Ok(MulScalarSuccessReturn { product })` holding their product modulo the group order
      * `[ ]`   Collaborators: halo2curves' `Fr` multiplication, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_mul_scalar_payload`, and `build_bls12_381_halo2curves_scalar`
      * `[ ]`   Arrange: `build_mul_scalar_payload` with the left override `build_bls12_381_halo2curves_scalar` with the value `scalar_value(GROUP_ORDER_MINUS_ONE_HEX)` and the right override with the value `Fr::from(2u64)`, whose integer product exceeds the group order
      * `[ ]`   Act: `mul_scalar(MulScalarParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `scalar_value(GROUP_ORDER_MINUS_TWO_HEX)`
    * `[ ]`   `neg_scalar_of_one_is_the_group_order_minus_one`
      * `[ ]`   Contract: `payload.scalar` → `Ok(NegScalarSuccessReturn { negation })` holding the group order minus the scalar
      * `[ ]`   Collaborators: halo2curves' `Fr` negation, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_neg_scalar_payload`, and `build_bls12_381_halo2curves_scalar`
      * `[ ]`   Arrange: `build_neg_scalar_payload` with the scalar override `build_bls12_381_halo2curves_scalar` with the value `Fr::from(1u64)`
      * `[ ]`   Act: `neg_scalar(NegScalarParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.negation.value` equals `scalar_value(GROUP_ORDER_MINUS_ONE_HEX)`
    * `[ ]`   `neg_scalar_of_zero_is_zero`
      * `[ ]`   Contract: `payload.scalar` of zero → `Ok(NegScalarSuccessReturn { negation })` holding zero
      * `[ ]`   Collaborators: halo2curves' `Fr` negation, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_neg_scalar_payload`, and `build_bls12_381_halo2curves_scalar`
      * `[ ]`   Arrange: `build_neg_scalar_payload` with the scalar override `build_bls12_381_halo2curves_scalar` with the value `Fr::ZERO`
      * `[ ]`   Act: `neg_scalar(NegScalarParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.negation.value` equals `Fr::ZERO`
    * `[ ]`   `neg_g1_of_the_generator_is_the_negated_generator`
      * `[ ]`   Contract: `payload.point` → `Ok(NegG1SuccessReturn { negation })` holding `(x, p - y)` for a point `(x, y)`
      * `[ ]`   Collaborators: halo2curves' affine negation, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_neg_g1_payload`, and `build_bls12_381_halo2curves_g1`
      * `[ ]`   Arrange: `build_neg_g1_payload` with the point override `build_bls12_381_halo2curves_g1` at its default generator
      * `[ ]`   Act: `neg_g1(NegG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.negation.value` equals `eip_2537_negated_g1_generator()`
    * `[ ]`   `neg_g1_of_the_identity_is_the_identity`
      * `[ ]`   Contract: `payload.point` of the identity → `Ok(NegG1SuccessReturn { negation })` holding the identity
      * `[ ]`   Collaborators: halo2curves' affine negation, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_neg_g1_payload`, and `build_bls12_381_halo2curves_g1`
      * `[ ]`   Arrange: `build_neg_g1_payload` with the point override `build_bls12_381_halo2curves_g1` with the value `G1Affine::identity()`
      * `[ ]`   Act: `neg_g1(NegG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.negation.value` equals `G1Affine::identity()`
    * `[ ]`   `neg_g2_negates_the_y_coordinate_and_keeps_x`
      * `[ ]`   Contract: `payload.point` → `Ok(NegG2SuccessReturn { negation })` holding `(x, -y)` for a point `(x, y)`
      * `[ ]`   Collaborators: halo2curves' affine negation, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_neg_g2_payload`, and `build_bls12_381_halo2curves_g2`
      * `[ ]`   Arrange: `build_neg_g2_payload` with the point override `build_bls12_381_halo2curves_g2` at its default generator
      * `[ ]`   Act: `neg_g2(NegG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; the negation's `coordinates()` yields an `x` equal to the `x` of `eip_2537_g2_generator().coordinates()` and a `y` whose sum with the generator's `y` is zero under `is_zero()`
    * `[ ]`   `neg_g2_of_the_identity_is_the_identity`
      * `[ ]`   Contract: `payload.point` of the identity → `Ok(NegG2SuccessReturn { negation })` holding the identity
      * `[ ]`   Collaborators: halo2curves' affine negation, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_neg_g2_payload`, and `build_bls12_381_halo2curves_g2`
      * `[ ]`   Arrange: `build_neg_g2_payload` with the point override `build_bls12_381_halo2curves_g2` with the value `G2Affine::identity()`
      * `[ ]`   Act: `neg_g2(NegG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.negation.value` equals `G2Affine::identity()`
    * `[ ]`   `is_identity_g1_is_true_for_the_identity`
      * `[ ]`   Contract: `payload.point` → `Ok(IsIdentityG1SuccessReturn { is_identity })`, true exactly for the identity
      * `[ ]`   Collaborators: halo2curves, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_is_identity_g1_payload`, and `build_bls12_381_halo2curves_g1`
      * `[ ]`   Arrange: `build_is_identity_g1_payload` with the point override `build_bls12_381_halo2curves_g1` with the value `G1Affine::identity()`
      * `[ ]`   Act: `is_identity_g1(IsIdentityG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_identity` is true
    * `[ ]`   `is_identity_g1_is_false_for_the_generator`
      * `[ ]`   Contract: `payload.point` → `Ok(IsIdentityG1SuccessReturn { is_identity })`, true exactly for the identity
      * `[ ]`   Collaborators: halo2curves, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_is_identity_g1_payload`, and `build_bls12_381_halo2curves_g1`
      * `[ ]`   Arrange: `build_is_identity_g1_payload` with the point override `build_bls12_381_halo2curves_g1` at its default generator
      * `[ ]`   Act: `is_identity_g1(IsIdentityG1Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_identity` is false
    * `[ ]`   `is_identity_g2_is_true_for_the_identity`
      * `[ ]`   Contract: `payload.point` → `Ok(IsIdentityG2SuccessReturn { is_identity })`, true exactly for the identity
      * `[ ]`   Collaborators: halo2curves, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_is_identity_g2_payload`, and `build_bls12_381_halo2curves_g2`
      * `[ ]`   Arrange: `build_is_identity_g2_payload` with the point override `build_bls12_381_halo2curves_g2` with the value `G2Affine::identity()`
      * `[ ]`   Act: `is_identity_g2(IsIdentityG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_identity` is true
    * `[ ]`   `is_identity_g2_is_false_for_the_generator`
      * `[ ]`   Contract: `payload.point` → `Ok(IsIdentityG2SuccessReturn { is_identity })`, true exactly for the identity
      * `[ ]`   Collaborators: halo2curves, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_is_identity_g2_payload`, and `build_bls12_381_halo2curves_g2`
      * `[ ]`   Arrange: `build_is_identity_g2_payload` with the point override `build_bls12_381_halo2curves_g2` at its default generator
      * `[ ]`   Act: `is_identity_g2(IsIdentityG2Params, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.is_identity` is false
    * `[ ]`   `pairing_product_of_no_terms_is_the_target_group_identity`
      * `[ ]`   Contract: an empty `payload.terms` → `Ok(PairingProductSuccessReturn { product })` holding the target group's identity
      * `[ ]`   Collaborators: halo2curves' multi-Miller loop, final exponentiation, and target-group exponentiation, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing` and `build_pairing_product_payload`
      * `[ ]`   Arrange: `build_pairing_product_payload` at its default of no terms
      * `[ ]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `Gt::identity()`
    * `[ ]`   `pairing_product_of_the_generators_is_not_the_target_group_identity`
      * `[ ]`   Contract: a pairing of the generators → `Ok(PairingProductSuccessReturn { product })` holding a value other than the target group's identity
      * `[ ]`   Collaborators: halo2curves' multi-Miller loop, final exponentiation, and target-group exponentiation, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_pairing_product_payload`, and `build_pairing_product_term`
      * `[ ]`   Arrange: `build_pairing_product_payload` with the terms override holding one `build_pairing_product_term` at its default generators
      * `[ ]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` is not equal to `Gt::identity()`
    * `[ ]`   `pairing_product_is_bilinear`
      * `[ ]`   Contract: a term whose first-group element is twice the generator → `Ok(PairingProductSuccessReturn { product })` holding the generators' pairing raised to the power two
      * `[ ]`   Collaborators: halo2curves' multi-Miller loop, final exponentiation, and target-group exponentiation, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_pairing_product_payload`, `build_pairing_product_term`, and `build_bls12_381_halo2curves_g1`; the expectation is `definition_value_power`, the identifier's definition raised to a power in the fixtures, independent of the correction `pairing_product` applies
      * `[ ]`   Arrange: `build_pairing_product_payload` with the terms override holding one `build_pairing_product_term` with the first-group override `build_bls12_381_halo2curves_g1` with the value `eip_2537_doubled_g1_generator()` over the second-group generator
      * `[ ]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value.inner()` equals `&definition_value_power(eip_2537_g1_generator(), eip_2537_g2_generator(), 2)`
    * `[ ]`   `pairing_product_multiplies_its_terms`
      * `[ ]`   Contract: `payload.terms` → `Ok(PairingProductSuccessReturn { product })` holding the product of each term's pairing
      * `[ ]`   Collaborators: halo2curves' multi-Miller loop, final exponentiation, and target-group exponentiation, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_pairing_product_payload`, and `build_pairing_product_term`; the expectation is `definition_value_power`
      * `[ ]`   Arrange: `build_pairing_product_payload` with the terms override holding two `build_pairing_product_term` values at their default generators, so a product over one term differs from the expected square
      * `[ ]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value.inner()` equals `&definition_value_power(eip_2537_g1_generator(), eip_2537_g2_generator(), 2)`
    * `[ ]`   `pairing_product_of_a_pairing_and_its_first_group_negation_is_the_target_group_identity`
      * `[ ]`   Contract: `payload.terms` whose pairings multiply to the target group's identity → `Ok(PairingProductSuccessReturn { product })` holding the identity
      * `[ ]`   Collaborators: halo2curves' multi-Miller loop, final exponentiation, and target-group exponentiation, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_pairing_product_payload`, `build_pairing_product_term`, and `build_bls12_381_halo2curves_g1`
      * `[ ]`   Arrange: `build_pairing_product_payload` with the terms override holding two `build_pairing_product_term` values: the generators, and the first-group override `build_bls12_381_halo2curves_g1` with the value `eip_2537_negated_g1_generator()` over the second-group generator
      * `[ ]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value` equals `Gt::identity()`
    * `[ ]`   `pairing_product_of_the_generators_equals_the_definition`
      * `[ ]`   Contract: a pairing of the generators → `Ok(PairingProductSuccessReturn { product })` holding the Miller loop's value raised to the exact exponent `(p^12 - 1) / r`, the value `TargetGroupEncodingIdentifier::Bls12381V1` defines, which is the library's reduced pairing with the cube removed
      * `[ ]`   Collaborators: halo2curves' multi-Miller loop, final exponentiation, and target-group exponentiation, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_pairing_product_payload`, and `build_pairing_product_term`; the expectation is `definition_value`, the Miller loop over the generators raised to `definition_exponent()` built by `num-bigint`, independent of the correction `pairing_product` applies
      * `[ ]`   Arrange: `build_pairing_product_payload` with the terms override holding one `build_pairing_product_term` at its default generators
      * `[ ]`   Act: `pairing_product(PairingProductParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.product.value.inner()` equals `&definition_value(eip_2537_g1_generator(), eip_2537_g2_generator())`
    * `[ ]`   `encode_gt_writes_the_target_group_identity_with_c0_c0_c0_first`
      * `[ ]`   Contract: a target-group value → `Ok(EncodeGtSuccessReturn { bytes })` holding its twelve coefficients in tower order, `c0.c0.c0` first, each 48 bytes big-endian, inside a `Secret`
      * `[ ]`   Collaborators: halo2curves' `to_repr`, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_encode_gt_payload`, and `build_bls12_381_halo2curves_gt`; the expectation is `gt_identity_encoding()`
      * `[ ]`   Arrange: `build_encode_gt_payload` with the value override `build_bls12_381_halo2curves_gt` with the value `Gt::identity()`
      * `[ ]`   Act: `encode_gt(EncodeGtParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.bytes.expose()` binds to a `Bls12381Halo2curvesEncodedGt` typed binding; its bytes equal `gt_identity_encoding()` as a byte slice
    * `[ ]`   `encode_gt_writes_the_published_pairing_of_the_generators`
      * `[ ]`   Contract: a target-group value → `Ok(EncodeGtSuccessReturn { bytes })` holding its twelve coefficients in tower order, `c0.c0.c0` through `c1.c2.c1`, each 48 bytes big-endian, inside a `Secret`
      * `[ ]`   Collaborators: halo2curves' `to_repr`, run for real as the vendor; fixtures `build_bls12_381_halo2curves_pairing`, `build_encode_gt_payload`, and `build_bls12_381_halo2curves_gt`; the expectation is `CFRG_GENERATOR_PAIRING_HEX`, the pairing of the base points published in the CFRG draft's test-vector appendix
      * `[ ]`   Arrange: `build_encode_gt_payload` with the value override `build_bls12_381_halo2curves_gt` with the value `corrected_generator_pairing()`, the identifier's value for the generators, whose twelve coefficients are distinct, so any exchange of two positions changes the bytes
      * `[ ]`   Act: `encode_gt(EncodeGtParams, payload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; the bytes of `success.bytes.expose()` equal `vector_bytes(CFRG_GENERATOR_PAIRING_HEX)` as a byte slice
    * `[ ]`   `scalar_field_order_is_the_group_order`
      * `[ ]`   Contract: any call → `Ok(ScalarFieldOrderSuccessReturn { bytes })` holding the group order's 32 big-endian bytes
      * `[ ]`   Collaborators: halo2curves' `Fr` negation and `to_repr`, run for real as the vendor; fixture `build_bls12_381_halo2curves_pairing`; `ScalarFieldOrderParams` and `ScalarFieldOrderPayload` by their production values
      * `[ ]`   Arrange: `build_bls12_381_halo2curves_pairing()`
      * `[ ]`   Act: `scalar_field_order(ScalarFieldOrderParams, ScalarFieldOrderPayload)`
      * `[ ]`   Assert: the result binds through the irrefutable `let Ok(success) = …;`; `success.bytes` equals `vector_bytes(GROUP_ORDER_HEX)`
    * `[ ]`   `g1_outside_subgroup_encoding_encodes_an_on_curve_point_outside_the_subgroup`
      * `[ ]`   Contract: any call → `Ok(G1OutsideSubgroupEncodingSuccessReturn { bytes: Some(bytes) })` holding the precompile encoding of an on-curve point outside the prime-order subgroup, in this concrete's `Bls12381Halo2curvesEncodedG1`
      * `[ ]`   Collaborators: halo2curves' curve and torsion checks, run for real as the vendor; fixture `build_bls12_381_halo2curves_pairing`; `G1OutsideSubgroupEncodingParams` and `G1OutsideSubgroupEncodingPayload` by their production values; the point read back by `g1_point_from_encoding`
      * `[ ]`   Arrange: `build_bls12_381_halo2curves_pairing()`
      * `[ ]`   Act: `g1_outside_subgroup_encoding(G1OutsideSubgroupEncodingParams, G1OutsideSubgroupEncodingPayload)`
      * `[ ]`   Assert: the success arm is extracted with `expect`; `success.bytes` is `Some` and its content binds to a `Bls12381Halo2curvesEncodedG1` typed binding; `g1_point_from_encoding` reads a point from its bytes, which is `Some` because the point is on the curve, and that point's `to_curve().is_torsion_free()` is false
    * `[ ]`   `g1_outside_subgroup_encoding_uses_the_least_first_coordinate`
      * `[ ]`   Contract: any call → the encoded point's `x` is the least `x` of an on-curve point outside the subgroup in an ascending search
      * `[ ]`   Collaborators: halo2curves' curve and torsion checks, run for real as the vendor; fixture `build_bls12_381_halo2curves_pairing`; `G1OutsideSubgroupEncodingParams` and `G1OutsideSubgroupEncodingPayload` by their production values; the expectation is `g1_outside_subgroup_bytes()`, the ascending search in the fixtures
      * `[ ]`   Arrange: `build_bls12_381_halo2curves_pairing()`
      * `[ ]`   Act: `g1_outside_subgroup_encoding(G1OutsideSubgroupEncodingParams, G1OutsideSubgroupEncodingPayload)`
      * `[ ]`   Assert: the success arm is extracted with `expect`; the first 64 bytes of the encoding, the `x` coordinate, equal the first 64 bytes of `g1_outside_subgroup_bytes()`
    * `[ ]`   `g1_outside_subgroup_encoding_takes_the_lesser_root`
      * `[ ]`   Contract: any call → the encoded point's `y` is the lesser of the two roots sharing its `x`
      * `[ ]`   Collaborators: halo2curves' curve and torsion checks, run for real as the vendor; fixture `build_bls12_381_halo2curves_pairing`; `G1OutsideSubgroupEncodingParams` and `G1OutsideSubgroupEncodingPayload` by their production values; the expectation is `base_field_modulus()`
      * `[ ]`   Arrange: `build_bls12_381_halo2curves_pairing()`
      * `[ ]`   Act: `g1_outside_subgroup_encoding(G1OutsideSubgroupEncodingParams, G1OutsideSubgroupEncodingPayload)`
      * `[ ]`   Assert: the success arm is extracted with `expect`; the last 48 bytes of the encoding's `y` coordinate, bytes 80 through 127, read as a `BigUint`, are not greater than `base_field_modulus()` minus that value
    * `[ ]`   `g2_outside_subgroup_encoding_encodes_an_on_curve_point_outside_the_subgroup`
      * `[ ]`   Contract: any call → `Ok(G2OutsideSubgroupEncodingSuccessReturn { bytes })` holding the precompile encoding of an on-curve point outside the prime-order subgroup, in this concrete's `Bls12381Halo2curvesEncodedG2`
      * `[ ]`   Collaborators: halo2curves' curve and torsion checks, run for real as the vendor; fixture `build_bls12_381_halo2curves_pairing`; `G2OutsideSubgroupEncodingParams` and `G2OutsideSubgroupEncodingPayload` by their production values; the point read back by `g2_point_from_encoding`
      * `[ ]`   Arrange: `build_bls12_381_halo2curves_pairing()`
      * `[ ]`   Act: `g2_outside_subgroup_encoding(G2OutsideSubgroupEncodingParams, G2OutsideSubgroupEncodingPayload)`
      * `[ ]`   Assert: the success arm is extracted with `expect`; `success.bytes` binds to a `Bls12381Halo2curvesEncodedG2` typed binding; `g2_point_from_encoding` reads a point from its bytes, which is `Some` because the point is on the curve, and that point's `to_curve().is_torsion_free()` is false
    * `[ ]`   `g2_outside_subgroup_encoding_has_a_real_first_coordinate_and_the_lesser_root`
      * `[ ]`   Contract: any call → the encoded point's first coordinate is `(c0, 0)` with the least `c0` of an on-curve point outside the subgroup, and its `y` is the lesser of the two roots, compared by `y.c1` and then `y.c0`
      * `[ ]`   Collaborators: halo2curves' curve and torsion checks, run for real as the vendor; fixture `build_bls12_381_halo2curves_pairing`; `G2OutsideSubgroupEncodingParams` and `G2OutsideSubgroupEncodingPayload` by their production values; the expectations are `g2_outside_subgroup_bytes()`, the ascending search in the fixtures, and `base_field_modulus()`
      * `[ ]`   Arrange: `build_bls12_381_halo2curves_pairing()`
      * `[ ]`   Act: `g2_outside_subgroup_encoding(G2OutsideSubgroupEncodingParams, G2OutsideSubgroupEncodingPayload)`
      * `[ ]`   Assert: the success arm is extracted with `expect`; the encoding's `x.c1` coordinate, bytes 64 through 127, is all zero; its `x.c0` coordinate, bytes 0 through 63, equals the first 64 bytes of `g2_outside_subgroup_bytes()`; the pair `y.c1`, `y.c0`, each the last 48 bytes of its 64-byte coordinate read as a `BigUint`, is not greater than the pair of their negations, each `base_field_modulus()` minus the coefficient, reduced to zero where the coefficient is zero

  * `[✅]`   `construction`
    * `[✅]`   `Bls12381Halo2curvesPairing::try_new` is the concrete's only producer, computing its one field, and its only caller is the pairing factory, which reads `Bls12381Halo2curvesPairing::DECLARATION` before constructing
    * `[✅]`   A group element or scalar is produced only by the adapter's generators, arithmetic, and decoders, or by the scalar's sampling bound; a target-group value only by `pairing_product`; an encoded value only by the adapter's encoders and reference methods; no consumer constructs one from library values

  * `[✅]`   `adapters/pairing/src/bls12_381_halo2curves/mod.rs`
    * `[✅]`   Module declarations: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, `pub(crate) mod provides;`, and `#[cfg(test)] mod test;`
    * `[✅]`   Imports: from `crate::factory::provides` every params, payload, return, success-return, and error type of the three traits and the sampling bound, `IPairingAdapter`, `IPairingArithmetic`, `IPairingReference`, `ISampleUniformScalar`, `PAIRING_INTERFACE_VERSION`, `PairingConcrete`, `PairingCurve`, `PairingDeclaration`, `PrecompileEncoding`, `TargetGroupEncodingIdentifier`, and `VerifierGroupArithmetic`; `core::array::from_fn`; `core::hint::black_box`; `core::iter::successors`; `domain::{Secret, SecretConstructorParams}`; `halo2curves::bls12381::{Bls12381, Fq, Fq2, Fr, G1Affine, G2Affine, Gt}`; `halo2curves::ff::{Field, FromUniformBytes, PrimeField}`; `halo2curves::group::{Curve, Group, cofactor::CofactorGroup, prime::PrimeCurveAffine}`; `halo2curves::msm::msm_best`; `halo2curves::pairing::{MillerLoopResult, MultiMillerLoop}`; `halo2curves::{Coordinates, CurveAffine}`; from `interface` every type it declares; `zeroize::{Zeroize, ZeroizeOnDrop}`
    * `[✅]`   `fn decode_coordinate(bytes: &[u8]) -> Option<Fq>`, private: `None` when any of `bytes[..16]` is nonzero; otherwise `bytes[16..64]` copied into a `[u8; 48]`, reversed, and read by `Fq::from_repr`, returned as `Option::<Fq>::from` of the `CtOption`
    * `[✅]`   `impl Bls12381Halo2curvesPairing` holding `pub const DECLARATION: PairingDeclaration`, the constant the interaction spec states, and `pub fn try_new(params: Bls12381Halo2curvesPairingConstructorParams) -> Bls12381Halo2curvesPairingTryNewReturn` realizing the interaction spec's construct branch
    * `[✅]`   `impl IPairingAdapter for Bls12381Halo2curvesPairing` with `const DECLARATION: PairingDeclaration = Bls12381Halo2curvesPairing::DECLARATION;`, `const CONCRETE: PairingConcrete = PairingConcrete::Bls12381Halo2curves;`, `type Scalar = Bls12381Halo2curvesScalar;`, `type G1 = Bls12381Halo2curvesG1;`, `type G2 = Bls12381Halo2curvesG2;`, `type EncodedG1 = Bls12381Halo2curvesEncodedG1;`, `type EncodedG2 = Bls12381Halo2curvesEncodedG2;`, and `type EncodedScalar = Bls12381Halo2curvesEncodedScalar;`, every method realizing its branches in the interaction spec, the decoders reading each coordinate through `decode_coordinate`
    * `[✅]`   `impl IPairingArithmetic for Bls12381Halo2curvesPairing` with `type Gt = Bls12381Halo2curvesGt;` and `type EncodedGt = Bls12381Halo2curvesEncodedGt;`, every method realizing its branches in the interaction spec, `pairing_product` multiplying the `Gt` by `self.reduced_pairing_correction`
    * `[✅]`   `impl IPairingReference for Bls12381Halo2curvesPairing`, every method realizing its branches in the interaction spec, each search by `find_map` with the `SearchExhausted` return by `let … else`
    * `[✅]`   `impl AsRef<[u8]>` for `Bls12381Halo2curvesEncodedG1`, `Bls12381Halo2curvesEncodedG2`, `Bls12381Halo2curvesEncodedScalar`, and `Bls12381Halo2curvesEncodedGt`, each returning `&self.bytes`; `impl Zeroize` for `Bls12381Halo2curvesEncodedScalar` and `Bls12381Halo2curvesEncodedGt`, each calling `self.bytes.zeroize()`; no implementation returns a mutable reference to the bytes
    * `[✅]`   `impl Zeroize` and `impl Drop` for `Bls12381Halo2curvesScalar`, setting `value` to `Fr::ZERO` and calling `black_box(&self.value)`, and for `Bls12381Halo2curvesG1`, `Bls12381Halo2curvesG2`, and `Bls12381Halo2curvesGt`, each setting `value` to its type's identity and calling `black_box(&self.value)`; `impl ZeroizeOnDrop for Bls12381Halo2curvesScalar {}`, marking the drop behavior the sampling bound requires
    * `[✅]`   `impl ISampleUniformScalar for Bls12381Halo2curvesScalar` with `const UNIFORM_BYTES_LENGTH: usize = 64;` and `sample_from_uniform_bytes` realizing its branches in the interaction spec, the scalar moved into a `Secret` by `let Ok(scalar) = Secret::try_new(SecretConstructorParams { value });`
    * `[✅]`   No `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`; the correction is computed in `try_new` and read from no configuration; neither the order, the order minus two, nor a point is written as a literal

  * `[✅]`   `adapters/pairing/src/bls12_381_halo2curves/provides.rs`
    * `[✅]`   `pub(crate) use super::interface::*;` and `#[cfg(test)] pub(crate) use super::mock::*;`, exposing every item of the concrete's interface to the crate and, in the crate's own test build, the concrete's builders, fixtures, and helpers, and nothing beyond them

  * `[✅]`   `directionality`
    * `[✅]`   `bls12_381_halo2curves` depends on the `factory` module's surface through `crate::factory::provides`, on `domain`, on `zeroize`, and on `halo2curves`; it depends on no other concrete and no concrete depends on it; among repository crates the crate depends on `crates/domain` alone at runtime
    * `[✅]`   `pairing/factory` constructs this concrete, the family form's recorded cycle, and proves its target-group encoding, order, and outside-the-subgroup encodings equal the arkworks concrete's

  * `[ ]`   `requirements`
    * `[✅]`   `halo2curves` is named nowhere in the crate outside the halo2curves concretes
    * `[✅]`   This concrete returns its own distinct fixed-width encoded types for the first group, the second group, the scalar, and the target group through the family's associated types
    * `[✅]`   `cargo check --all-targets --all-features`, `cargo fmt --check`, and `cargo deny check` complete without error; `cargo clippy --all-targets --all-features` reports nothing beyond the library target's unused-item warnings for the pairing concretes, which `pairing/factory` constructs
    * `[✅]`   Every test in `bls12_381_halo2curves/test.rs` passes (CR-04, CR-08, CR-09, and CR-10 on BLS12-381 over halo2curves; CR-05 for the sampled scalar; CR-07 for the clearing; CR-11 for the order)
    * `[ ]`   The declaration and concrete-identity compile-time assertions compile, the `ZeroizeOnDrop` compile-time assertion compiles, `bls12_381_halo2curves_scalar_zeroize_clears_its_value` passes (CR-07), and `pairing_product_of_the_generators_equals_the_definition`, `encode_gt_writes_the_published_pairing_of_the_generators`, and `reduced_pairing_correction_inverts_three` pass (CR-10, the target-group value fixed by the identifier's definition, executed in the test, and its serialization by the standard's published value)
    * `[✅]`   Code outside `adapters/pairing` naming `Bls12381Halo2curvesPairing` or anything under `bls12_381_halo2curves` fails to compile; the crate's public surface is the `factory` module's `provides`

* `[ ]`   `pairing/factory` **Pairing factory constructing the concrete the composition names, admitted against the chain's precompile encodings and the suite's target-group encoding identifier, and handing it to a consumer generic over the family's arithmetic and reference traits; carries the family's integration test proving the libraries of each curve agree**

  * `[✅]`   `objective`
    * `[✅]`   Problem: a consumer obtains a pairing adapter only through the family's generic surface, never by naming a concrete, and a concrete whose precompile encoding the target chain does not deploy is refused before anything is constructed (CR-10; Composition Boundary)
    * `[✅]`   Problem: the KEM, the envelope, and the delivery proof reach a pairing concrete only through `create_pairing` and `IPairingConsumer`; a capsule encapsulated under one library must decapsulate to the same wrapping key under the other, so both libraries on a curve encode a target-group value to the same bytes; and the bytes are fixed by the suite's target-group encoding identifier, so the factory admits a concrete only when it declares that identifier, as the KEM factory admits by scope and the delivery-proof factory by algebra, form, and version (CR-08; CR-10; CR-11; the product requirements' canonical target-group value position)
    * `[✅]`   Problem: a consumer that mirrors the reference to a target suite reaches the group order and the outside-the-subgroup encodings only through the concrete the factory hands it, so the consumer surface requires the reference trait of that concrete; and two libraries of one curve are interchangeable under one suite only if they return the same bytes for both, which agreement between each concrete and its own decoder does not prove (CR-10; CR-11; the dependency map's `pairing/factory` row; the Pairing adapters and key derivation milestone's exit)
    * `[✅]`   Functional: given the concrete the composition names, the precompile encodings the chain declares, and the target-group encoding identifier the suite requires, the factory refuses a concrete whose declared encoding is not among the chain's, or whose declared identifier is not the suite's, with no construction and no call to the consumer
    * `[✅]`   Functional: an admitted concrete is constructed and handed to a consumer generic over a concrete implementing `IPairingArithmetic` and `IPairingReference`, which reads `P::DECLARATION` and `P::CONCRETE`, and the consumer's output is returned; the consumer never names the concrete; a consumer whose `consume_pairing` is written against `IPairingAdapter` or `IPairingArithmetic` alone is a valid implementation, an implementation's bound being no stricter than the trait's
    * `[✅]`   Functional: a concrete's constructor error is returned unchanged in the factory's error arm, one variant per concrete
    * `[✅]`   Functional: every concrete obtained from the factory computes through the family's traits; the arkworks and halo2curves concretes on BN254 encode the pairing of the same inputs to the same bytes, and likewise on BLS12-381, each concrete's definition test in its own node passing
    * `[✅]`   Functional: through the family's surface alone, both concretes of BN254 return the same scalar field order, no first-group encoding, and the same second-group encoding, which `decode_g2` refuses as outside the subgroup; both concretes of BLS12-381 return the same scalar field order and the same encoding in each group, each of which its group's decoder refuses as outside the subgroup
    * `[✅]`   Non-functional: adding a concrete is its module, its variant in the selection enum and in the error enum, its entry in `PAIRING_CONCRETES`, and its arm here; no consumer changes

  * `[✅]`   `role`
    * `[✅]`   Adapter family factory: the implementation of the `factory` module, which is the pairing family's construction point and the crate's public surface; binds the arithmetic and reference traits into the consumer surface
    * `[✅]`   Hands the concrete to a consumer rather than returning it, because the family's traits carry associated types and so cannot be returned as one type across concretes; the consumer is written once, generic over the family's traits, and the factory instantiates it for the concrete it constructs
    * `[✅]`   Admits by the chain's precompile encodings and by the suite's target-group encoding identifier, both read before any construction
    * `[✅]`   Does not choose a concrete when none is named; the composition names one from the configuration catalogue, whose shipped value per curve is the default `harness-crypto/benchmark` records
    * `[✅]`   Does not read the chain family; the composition resolver passes the chain's declared encodings as values of this family's `PrecompileEncoding`
    * `[✅]`   Does not decode or validate configuration; the configuration registry hands the factory typed params
    * `[✅]`   Does not edit any concrete
    * `[✅]`   Carries the family's integration test; does not carry a commit

  * `[✅]`   `module`
    * `[✅]`   Bounded context: the `factory` module of `adapters/pairing`, holding the factory function, its deps, params, payload, and return types, its signature type, the consumer trait with its params and payload types, the derive on `PrecompileEncoding` the admission compares by, the function mock and builders, the unit test, and the crate's integration test under `adapters/pairing/tests`
    * `[✅]`   The node's files: `adapters/pairing/src/factory/interface.rs`, `interaction.spec.md`, `mock.rs`, `test.rs`, `mod.rs`, and `provides.rs`, and `adapters/pairing/tests/factory_integration_test.rs`
    * `[✅]`   Outside: every concrete's behavior, the family's traits and declaration, the chain family's declarations, the configuration catalogue, and every consumer of the family

  * `[ ]`   `deps`
    * `[✅]`   Each concrete, through its `provides`: `Bn254ArkworksPairing`, `Bn254Halo2curvesPairing`, `Bls12381ArkworksPairing`, and `Bls12381Halo2curvesPairing`, each with its constructor params, `try_new`, `DECLARATION`, `CONCRETE`, and implementations of `IPairingAdapter`, `IPairingArithmetic`, and `IPairingReference`, as `pairing/bn254_arkworks`, `pairing/bn254_halo2curves`, `pairing/bls12_381_arkworks`, and `pairing/bls12_381_halo2curves` state them; the factory constructs its concretes, the family form's recorded cycle
    * `[✅]`   The `factory` module's interface: `IPairingAdapter`, `IPairingArithmetic`, `IPairingReference`, `PairingDeclaration`, `PairingConcrete`, `PrecompileEncoding`, `TargetGroupEncodingIdentifier`, and every method's types, as `pairing/bn254_arkworks` authors them
    * `[ ]`   `core::convert::Infallible`, standard library, each concrete's constructor error carried in the factory's error arm
    * `[ ]`   `domain` with its `mocks` feature, the crate's dev-dependency, for `build_secret` and `SecretConstructorParamsOverrides` in the integration test; `pairing` itself, as the integration test's dependency through the crate's public surface
    * `[✅]`   No external crate beyond those `adapters/pairing/Cargo.toml` holds; this node edits no manifest
    * `[✅]`   Reverse dependencies: `kem/bb1_depth_one` and `harness-crypto/main`, each through `create_pairing` and an `IPairingConsumer`

  * `[ ]`   `context_slice`
    * `[✅]`   From each concrete's `provides`: `DECLARATION`, a `PairingDeclaration`, and `try_new(params) -> Result<Self, Infallible>` taking the concrete's fieldless constructor params; from its `IPairingAdapter` implementation, `CONCRETE`, a `PairingConcrete`
    * `[ ]`   From the `factory` module's mock: `MockIPairingConsumer`, implementing `IPairingConsumer` with `type Output = ();`, the official consumer mock the unit test hands the factory
    * `[ ]`   In the integration test: `ISampleUniformScalar::sample_from_uniform_bytes(SampleUniformScalarParams, SampleUniformScalarPayload { uniform })`, which reads `uniform` through `Secret::expose` and returns `scalar: Secret<P::Scalar>`; `IPairingAdapter::encode_scalar(&self, EncodeScalarParams, EncodeScalarPayload { scalar })` returning `bytes: Secret<P::EncodedScalar>`; `Secret::expose(&self) -> &T`; and, from `domain`'s mocks, `build_secret(SecretConstructorParamsOverrides<Vec<u8>>) -> Secret<Vec<u8>>`

  * `[✅]`   `adapters/pairing/src/factory/interface.rs`
    * `[✅]`   `PrecompileEncoding` derives `PartialEq` and `Eq`, so the admission compares a declared encoding with the chain's
    * `[✅]`   `IPairingConsumer`, `pub trait IPairingConsumer` with `type Output;` and `fn consume_pairing<P: IPairingArithmetic + IPairingReference>(&self, params: ConsumePairingParams, payload: ConsumePairingPayload<P>) -> Self::Output;`, the work a composition performs with whichever concrete the factory constructs
    * `[✅]`   `ConsumePairingParams`, the fieldless struct `pub struct ConsumePairingParams;`
    * `[✅]`   `ConsumePairingPayload<P: IPairingAdapter>`, a struct with only `pub adapter: P`; the consumer reads `P::DECLARATION` and `P::CONCRETE`, so neither metadata value can be paired with another adapter
    * `[✅]`   `CreatePairingDeps<C>`, a struct with `pub consumer: C`, the collaborator the factory hands the concrete to
    * `[✅]`   `CreatePairingParams`, a struct with `pub concrete: PairingConcrete`, `pub supported_encodings: Vec<PrecompileEncoding>`, and `pub target_group_encoding: TargetGroupEncodingIdentifier`, the selection, the encodings the chain declares, and the identifier the suite requires
    * `[✅]`   `CreatePairingPayload`, the fieldless struct `pub struct CreatePairingPayload;`, since the factory operates on no data
    * `[✅]`   `CreatePairingSuccessReturn<O>`, a struct with `pub output: O`
    * `[✅]`   `CreatePairingErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]`, which `Infallible` satisfies, and the variants `UnsupportedPrecompileEncoding`, `UnsupportedTargetGroupEncoding`, `Bn254Arkworks(Infallible)`, `Bn254Halo2curves(Infallible)`, `Bls12381Arkworks(Infallible)`, and `Bls12381Halo2curves(Infallible)`, each concrete's constructor error carried unchanged in its own variant
    * `[✅]`   `CreatePairingReturn<O>`, the alias `Result<CreatePairingSuccessReturn<O>, CreatePairingErrorReturn>`
    * `[✅]`   `CreatePairingFn<C>`, the alias `fn(&CreatePairingDeps<C>, CreatePairingParams, CreatePairingPayload) -> CreatePairingReturn<<C as IPairingConsumer>::Output>`
    * `[ ]`   `PAIRING_CONCRETES`, a `pub const` slice of `PairingConcrete` holding each variant of the selection enum, the family's declared set of selectable concretes
    * `[✅]`   No derives on any type this node declares beyond those stated; the file names no vendor and no concrete

  * `[✅]`   `adapters/pairing/src/factory/interaction.spec.md`
    * `[✅]`   The title `` # `factory` — interaction spec `` and one opening sentence stating the file as the branch contract for the `factory` module of the `pairing` crate, the pairing family's construction point, admitting the concrete the composition names against the chain's declared precompile encodings and the suite's target-group encoding identifier and handing it to a consumer generic over `IPairingArithmetic` and `IPairingReference`, each branch stating condition, decision, dependency call, and the exact return outcome; then one `##` section headed by the full signature `create_pairing<C: IPairingConsumer>(deps: &CreatePairingDeps<C>, params: CreatePairingParams, payload: CreatePairingPayload) -> CreatePairingReturn<C::Output>`, then `## Ordering and edges`
    * `[✅]`   The section opens by stating that the decision is a `match` on `params.concrete`, one arm per `PairingConcrete` variant, `Bn254Arkworks`, `Bn254Halo2curves`, `Bls12381Arkworks`, and `Bls12381Halo2curves`, exhaustive, so a variant with no arm fails to compile, each arm constructing the concrete whose `CONCRETE` is the arm's variant and running the same contract against it; then a table with the columns `Branch`, `Condition`, `Decision`, `Dependency call`, and `Outcome` holding, in order: unsupported encoding, condition the arm's concrete's `DECLARATION.precompile_encoding` is not in `params.supported_encodings`, decision `contains`, read before any construction, dependency call none, outcome `Err(CreatePairingErrorReturn::UnsupportedPrecompileEncoding)` with nothing constructed and the consumer not called; unsupported target-group encoding, condition the arm's concrete's `DECLARATION.target_group_encoding` is not equal to `params.target_group_encoding`, decision equality, read after the encoding admission and before any construction, dependency call none, outcome `Err(CreatePairingErrorReturn::UnsupportedTargetGroupEncoding)` with nothing constructed and the consumer not called; admitted, condition both admissions pass, decision both checks, dependency call the concrete's `try_new` with its fieldless constructor params, exactly once, its success destructured irrefutably because its error arm is uninhabited, then `deps.consumer.consume_pairing(ConsumePairingParams, ConsumePairingPayload { adapter })`, exactly once, the consumer reading `P::DECLARATION` and `P::CONCRETE`, outcome `Ok(CreatePairingSuccessReturn { output })` holding the consumer's output; the section closes by stating that each concrete's own variant of `CreatePairingErrorReturn` carries that concrete's constructor error in the return union unchanged and, every constructor error being `Infallible`, no branch produces one
    * `[✅]`   `Ordering and edges`, a bulleted section: `params.concrete` selects the concrete, `params.supported_encodings` and `params.target_group_encoding` admit or refuse it, and `payload` carries nothing and is not read; the encoding admission precedes the target-group admission and both precede construction in every arm, so a refused concrete is never constructed and `deps.consumer` is never touched; the consumer is generic over `P: IPairingArithmetic + IPairingReference`, instantiated inside the arm for the concrete the arm constructs, and never names the concrete itself

  * `[ ]`   `adapters/pairing/src/factory/mock.rs`
    * `[✅]`   `CreatePairingParamsOverrides`, `#[derive(Default)]`, with `pub concrete: Option<PairingConcrete>`, `pub supported_encodings: Option<Vec<PrecompileEncoding>>`, and `pub target_group_encoding: Option<TargetGroupEncodingIdentifier>`; `build_create_pairing_params(overrides: CreatePairingParamsOverrides) -> CreatePairingParams` binding `let concrete = overrides.concrete.unwrap_or(PairingConcrete::Bn254Arkworks);`, defaulting `supported_encodings` to `vec![PrecompileEncoding::Eip196Eip197, PrecompileEncoding::Eip2537]`, and defaulting `target_group_encoding` to `match concrete { PairingConcrete::Bn254Arkworks | PairingConcrete::Bn254Halo2curves => TargetGroupEncodingIdentifier::Bn254V1, PairingConcrete::Bls12381Arkworks | PairingConcrete::Bls12381Halo2curves => TargetGroupEncodingIdentifier::Bls12381V1 }`, so an arrangement naming a concrete alone admits and only an arrangement proving refusal overrides the identifier
    * `[✅]`   `CreatePairingSuccessReturnOverrides<O>`, `#[derive(Default)]`, one field `pub output: Option<O>`; `build_create_pairing_success_return<O: Default>(overrides: CreatePairingSuccessReturnOverrides<O>) -> CreatePairingSuccessReturn<O>`
    * `[✅]`   `ConsumePairingPayloadOverrides<P>`, with `impl<P: IPairingAdapter> Default` initializing its only field to `None`, and only `pub adapter: Option<MockIPairingAdapter<P>>`; `build_consume_pairing_payload<P: IPairingAdapter>(overrides: ConsumePairingPayloadOverrides<P>) -> ConsumePairingPayload<MockIPairingAdapter<P>>` under the mock adapter's `Default` bounds, the adapter defaulting to `MockIPairingAdapter { adapter: PhantomData }`; its declaration, concrete identity, scalar, and group types come from the one adapter type `P`, with no independent metadata or value-type overrides
    * `[ ]`   `MockIPairingConsumer`, the unit struct with `#[derive(Default)]`, `pub struct MockIPairingConsumer;`, implementing `IPairingConsumer` with `type Output = ();` and `consume_pairing<P: IPairingAdapter>` returning `()`; a test needing other behavior implements the trait on its own local struct
    * `[ ]`   `CreatePairingDepsOverrides<C: IPairingConsumer>`, with `impl<C: IPairingConsumer> Default` initializing its only field to `None`, and only `pub consumer: Option<C>`; `build_create_pairing_deps<C: IPairingConsumer + Default>(overrides: CreatePairingDepsOverrides<C>) -> CreatePairingDeps<C>`, the consumer defaulting to `C::default()`, so a unit test places `MockIPairingConsumer` in the deps by naming it as the type parameter
    * `[✅]`   `mock_create_pairing<C: IPairingConsumer>(_deps: &CreatePairingDeps<C>, _params: CreatePairingParams, _payload: CreatePairingPayload) -> CreatePairingReturn<C::Output>` for `C::Output: Default`, returning `Ok(build_create_pairing_success_return(Default::default()))`
    * `[ ]`   No builder for the fieldless `ConsumePairingParams` and `CreatePairingPayload`, used by their production values, or for the enums; the module imports from `super::interface` the names these items add

  * `[ ]`   `adapters/pairing/src/factory/test.rs`
    * `[ ]`   Imports `create_pairing` from `super`, and `CreatePairingDepsOverrides`, `CreatePairingErrorReturn`, `CreatePairingParamsOverrides`, `CreatePairingPayload`, `MockIPairingConsumer`, `PairingConcrete`, `PrecompileEncoding`, `TargetGroupEncodingIdentifier`, `build_create_pairing_deps`, and `build_create_pairing_params` from `super::provides`
    * `[ ]`   Every block: the function under test is `create_pairing`; the consumer is `MockIPairingConsumer`, the family's official consumer mock, placed in the deps by `build_create_pairing_deps::<MockIPairingConsumer>(CreatePairingDepsOverrides::default())`; the arm's concrete `DECLARATION` and `try_new` run for real, since the factory reads and constructs them itself; the payload is the production value `CreatePairingPayload`; params come from `build_create_pairing_params` with only the fields the block varies overridden
    * `[ ]`   `create_pairing_admits_the_<concrete>_concrete`, one block per `PairingConcrete` variant, `<concrete>` the variant in snake case: `bn254_arkworks` for `Bn254Arkworks`, `bn254_halo2curves` for `Bn254Halo2curves`, `bls12_381_arkworks` for `Bls12381Arkworks`, and `bls12_381_halo2curves` for `Bls12381Halo2curves`
      * `[ ]`   Contract: given `params.concrete` is the variant, its declared precompile encoding is among `params.supported_encodings`, and its declared target-group encoding equals `params.target_group_encoding`, the factory returns the success arm
      * `[ ]`   Arrange: `build_create_pairing_params` with only `concrete` overridden to the variant, so `supported_encodings` holds the encoding of each curve and `target_group_encoding` is the identifier of the variant's curve; deps with no override
      * `[ ]`   Act: `create_pairing(&deps, params, CreatePairingPayload)`
      * `[ ]`   Assert: `result.is_ok()`
    * `[ ]`   `create_pairing_refuses_the_<concrete>_concrete_whose_encoding_the_chain_does_not_deploy`, one block per variant, `<concrete>` as above
      * `[ ]`   Contract: given the variant's declared precompile encoding is not in `params.supported_encodings`, the factory returns `Err(CreatePairingErrorReturn::UnsupportedPrecompileEncoding)`
      * `[ ]`   Arrange: `build_create_pairing_params` with `concrete` overridden to the variant and `supported_encodings` overridden to a set holding only the encoding of the other curve, `PrecompileEncoding::Eip2537` for the BN254 variants and `PrecompileEncoding::Eip196Eip197` for the BLS12-381 variants, each variant's own encoding being the one its `DECLARATION` states; `target_group_encoding` left at the builder's default for the variant's curve, so only the encoding admission fails; deps with no override
      * `[ ]`   Act: `create_pairing(&deps, params, CreatePairingPayload)`
      * `[ ]`   Assert: `assert_eq!(result.err(), Some(CreatePairingErrorReturn::UnsupportedPrecompileEncoding))`, the interaction spec's outcome as a literal
    * `[ ]`   `create_pairing_refuses_the_<concrete>_concrete_whose_target_group_encoding_the_suite_does_not_require`, one block per variant, `<concrete>` as above
      * `[ ]`   Contract: given the variant's declared precompile encoding is in `params.supported_encodings` and its declared target-group encoding differs from `params.target_group_encoding`, the factory returns `Err(CreatePairingErrorReturn::UnsupportedTargetGroupEncoding)`
      * `[ ]`   Arrange: `build_create_pairing_params` with `concrete` overridden to the variant and `target_group_encoding` overridden to the identifier of the other curve, `TargetGroupEncodingIdentifier::Bls12381V1` for the BN254 variants and `TargetGroupEncodingIdentifier::Bn254V1` for the BLS12-381 variants, each variant's own identifier being the one its `DECLARATION` states; `supported_encodings` left at the builder's default, so only the target-group admission fails; deps with no override
      * `[ ]`   Act: `create_pairing(&deps, params, CreatePairingPayload)`
      * `[ ]`   Assert: `assert_eq!(result.err(), Some(CreatePairingErrorReturn::UnsupportedTargetGroupEncoding))`, the interaction spec's outcome as a literal
    * `[ ]`   `create_pairing_refuses_the_encoding_before_the_target_group_encoding_for_the_<concrete>_concrete`, one block per variant, `<concrete>` as above
      * `[ ]`   Contract: given both the variant's declared precompile encoding is not in `params.supported_encodings` and its declared target-group encoding differs from `params.target_group_encoding`, the factory returns `Err(CreatePairingErrorReturn::UnsupportedPrecompileEncoding)`, the encoding admission preceding the target-group admission
      * `[ ]`   Arrange: `build_create_pairing_params` with `concrete` overridden to the variant, `supported_encodings` overridden as in the encoding refusal block, and `target_group_encoding` overridden as in the target-group refusal block, so both admissions fail; deps with no override
      * `[ ]`   Act: `create_pairing(&deps, params, CreatePairingPayload)`
      * `[ ]`   Assert: `assert_eq!(result.err(), Some(CreatePairingErrorReturn::UnsupportedPrecompileEncoding))`, the interaction spec's outcome as a literal

  * `[✅]`   `construction`
    * `[✅]`   The composition root writes its pairing-dependent work once as an `IPairingConsumer`, generic over `P: IPairingArithmetic + IPairingReference`, and calls `create_pairing` with `CreatePairingDeps { consumer }`, `CreatePairingParams` holding the configured `PairingConcrete`, the chain's declared encodings, and the suite's identifier, and `CreatePairingPayload`; no consumer constructs or names a concrete

  * `[✅]`   `adapters/pairing/src/factory/mod.rs`
    * `[✅]`   The module wiring holds `#[cfg(test)] mod test;`; imports each concrete's adapter type and constructor params from `crate::bn254_arkworks::provides`, `crate::bn254_halo2curves::provides`, `crate::bls12_381_arkworks::provides`, and `crate::bls12_381_halo2curves::provides`, and from `interface` `ConsumePairingParams`, `ConsumePairingPayload`, `CreatePairingDeps`, `CreatePairingErrorReturn`, `CreatePairingParams`, `CreatePairingPayload`, `CreatePairingReturn`, `CreatePairingSuccessReturn`, `IPairingConsumer`, and `PairingConcrete`
    * `[✅]`   `pub fn create_pairing<C: IPairingConsumer>(deps: &CreatePairingDeps<C>, params: CreatePairingParams, _payload: CreatePairingPayload) -> CreatePairingReturn<C::Output>`, a `match` on `params.concrete` whose each arm checks the concrete's `DECLARATION.precompile_encoding` against `params.supported_encodings` and its `DECLARATION.target_group_encoding` against `params.target_group_encoding`, returning the matching refusal, then binds the concrete by `let Ok(adapter) = <Concrete>::try_new(<Concrete>ConstructorParams);` and returns `Ok(CreatePairingSuccessReturn { output: deps.consumer.consume_pairing(ConsumePairingParams, ConsumePairingPayload { adapter }) })`; each arm's variant is the `CONCRETE` of the concrete it constructs
    * `[✅]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[✅]`   `adapters/pairing/src/factory/provides.rs`
    * `[✅]`   The line `pub use super::create_pairing;`, beside the interface and mock re-exports, the crate's public surface, which the barrel re-exports and nothing beneath it reaches

  * `[ ]`   `adapters/pairing/tests/integration_test.rs`

  * `[✅]`   `directionality`
    * `[✅]`   The `factory` module depends on each concrete through its `provides` and on its own interface; each concrete depends on the `factory` module's surface, the family form's recorded cycle and the crate's one cycle; `IPairingConsumer` names `IPairingArithmetic` and `IPairingReference` within the `factory` module; the crate's public surface is the `factory` module's `provides`; no edge between crates beyond `domain`, `zeroize`, and the curve libraries
    * `[✅]`   `kem/bb1_depth_one` consumes the family through `create_pairing` and an `IPairingConsumer`; `harness-crypto/main` calls `create_pairing` with an `IPairingConsumer` bounded by `IPairingArithmetic + IPairingReference` and hands the concrete to the generate concrete, whose `harness-crypto/generate/evm/constants` and `harness-crypto/generate/evm/rejection_vectors` read the order and the encodings from it

  * `[ ]`   `requirements`
    * `[✅]`   `cargo check --workspace --all-targets --all-features`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo fmt --all --check`, and `cargo deny check` complete without error or warning in every target, the pairing concretes' unused-item warnings having no cause once the factory constructs them
    * `[ ]`   `CreatePairingErrorReturn` derives `Debug`, `PartialEq`, and `Eq`; the refusal blocks assert the whole error by `assert_eq!`
    * `[✅]`   Every test in `adapters/pairing` passes
    * `[ ]`   The `create_pairing_admits_the_…_concrete` blocks pass, each proving its arm admits its concrete against the official consumer mock (CR-10, every concrete constructed through the family's surface)
    * `[ ]`   The `create_pairing_refuses_the_…_concrete_whose_encoding_the_chain_does_not_deploy` blocks pass (CR-10, admission by the chain's precompile encodings)
    * `[ ]`   The `create_pairing_refuses_the_…_concrete_whose_target_group_encoding_the_suite_does_not_require` blocks pass (CR-10, admission by the suite's identifier)
    * `[ ]`   The `create_pairing_refuses_the_encoding_before_the_target_group_encoding_for_the_…_concrete` blocks pass (CR-10, admission by the chain's precompile encodings precedes admission by the suite's identifier)
    * `[ ]`   Each `the_…_concrete_from_the_factory_samples_and_encodes_through_secret` integration test passes, each proving its `Secret → <Concrete> → create_pairing` chain with every link real (CR-07, secret material crosses the family inside `Secret`; CR-10, every concrete reached through the factory)
    * `[✅]`   No `ark-` crate or `halo2curves` is named outside its concretes, and no code outside `adapters/pairing` can name a concrete

* `[ ]`   `harness-crypto/benchmark` **Pairing benchmark timing scalar multiplication, multi-scalar multiplication, and the pairing-product check on whichever concrete the pairing factory hands it; creates the `apps/harness-crypto` crate**

  * `[✅]`   `objective`
    * `[✅]`   Problem: the default pairing concrete per curve is a configured value set from measurement, so each concrete's cost for the operations the KEM, envelope, and proof perform is measured through the factory, with no module naming a library
    * `[✅]`   Functional: handed any concrete by `create_pairing`, the benchmark draws two scalars from the randomness family, samples them through the concrete's sampling bound, and times first-group and second-group scalar multiplication, first-group and second-group multi-scalar multiplication over two terms, and the pairing-product check over two terms
    * `[✅]`   Functional: each timing is the mean over a configured number of iterations, returned per operation with the exact `PairingConcrete` measured and the nonzero iteration count used to compute it
    * `[✅]`   Functional: a failed draw or a failed sampling is returned unchanged in the error arm, before anything is timed
    * `[✅]`   Non-functional: the benchmark names no curve library and no concrete; it reaches every concrete only through `create_pairing` and `IPairingAdapter`

  * `[✅]`   `role`
    * `[✅]`   App module: an `IPairingConsumer` the harness passes to `create_pairing` once per `PairingConcrete`; it is a class in the adapter role, implementing a repo-owned trait and wrapping the operating system's monotonic clock
    * `[✅]`   Creates the `apps/harness-crypto` crate as a library crate and adds `"apps/*"` to the root manifest's `members`, the member glob `workspace/cargo` specifies, so the crate is admitted
    * `[✅]`   Returns the timings, concrete identity, and iteration count of the concrete it is handed and selects nothing; the harness run constructs it for each concrete and records the measurements, and the configured default per curve is set from them
    * `[✅]`   Does not author `main.rs`; `harness-crypto/main` authors the binary entry, reading `harness-crypto/config` and composing every family through its factory
    * `[✅]`   Does not time decoding, encoding, or sampling
    * `[✅]`   Does not carry a commit

  * `[✅]`   `module`
    * `[✅]`   Bounded context: the `benchmark` module of `apps/harness-crypto`, holding `PairingBenchmark`, its constructor params, its timings and return types, its implementation of `IPairingConsumer`, and its builders
    * `[✅]`   Creates the crate at `apps/harness-crypto`, admitted by the `apps/*` glob this node adds
    * `[✅]`   Outside: the choice of which concretes to run, the iteration count's configured value, the recording of timings, and every other harness module

  * `[ ]`   `deps`
    * `[ ]`   `pairing`, `adapters/pairing`, adapter ring, path dependency; supplies `IPairingConsumer`, `ConsumePairingParams`, `ConsumePairingPayload`, `IPairingAdapter`, `ISampleUniformScalar`, `SampleUniformScalarParams`, `SampleUniformScalarPayload`, `SampleUniformScalarErrorReturn`, and every method's params and payload types used below; in the unit test the family's mock surface, `build_consume_pairing_payload`, `ConsumePairingPayloadOverrides`, every family builder, `MockEncodedG1`, `MockEncodedG2`, `MockEncodedScalar`, and `MockEncodedGt`, and the items of `IPairingAdapter`, `IPairingArithmetic`, and `IPairingReference` with the declaration's types, for the test-local pairing double; in the integration test `create_pairing`, `CreatePairingDeps`, `CreatePairingPayload`, `PairingConcrete`, `build_create_pairing_params`, and `CreatePairingParamsOverrides`; direction inward, app on adapter
    * `[ ]`   `random`, `adapters/random`, adapter ring, path dependency; supplies `IRandomSourceAdapter`, `FillBytesParams`, `FillBytesPayload`, and `FillBytesErrorReturn`; `MockIRandomSourceAdapter`, the family's official mock, is the benchmark builder's default source, which the unit test and the pairing-chain integration tests use; in the integration test `create_random_source`, `CreateRandomSourceDeps`, `CreateRandomSourcePayload`, `build_create_random_source_params`, `CreateRandomSourceParamsOverrides`, and `RandomSourceKind`
    * `[✅]`   `pairing` and `random` with their `mocks` features, as dev-dependencies and through this crate's `mocks` feature
    * `[ ]`   `zeroize` `1.9.0`, external crate, Apache-2.0 OR MIT, dev-dependency; supplies `Zeroize` and `ZeroizeOnDrop`, which the unit test's test-local scalar and group types implement to satisfy the family's bounds
    * `[✅]`   `std::time::Instant` and `core::time::Duration`, standard library, the monotonic clock and its measure; `core::num::NonZeroU32`, standard library, the iteration count, so the mean never divides by zero; `core::convert::Infallible`, standard library, the constructor's error arm
    * `[ ]`   No external crate at runtime; no reverse dependency

  * `[ ]`   `context_slice`
    * `[✅]`   From `pairing`: `IPairingConsumer` with `type Output` and `consume_pairing<P: IPairingAdapter>(&self, ConsumePairingParams, ConsumePairingPayload<P>) -> Self::Output`; `ConsumePairingPayload<P>` with only `adapter`, and `P::DECLARATION` and `P::CONCRETE` read from the trait; the adapter methods `g1_generator`, `g2_generator`, `mul_g1`, `mul_g2`, `msm_g1`, `msm_g2`, and `pairing_product_is_one`, each returning `Result<_, Infallible>`; `ISampleUniformScalar` with `UNIFORM_BYTES_LENGTH` and `sample_from_uniform_bytes`
    * `[ ]`   From `pairing`'s mocks, in the unit test: `build_consume_pairing_payload::<P>(ConsumePairingPayloadOverrides<P>) -> ConsumePairingPayload<MockIPairingAdapter<P>>`, whose adapter takes its constants and its scalar and group types from `P`, which therefore implements `IPairingAdapter`, `IPairingArithmetic`, and `IPairingReference` with `Default` scalar, group, and target-group types, as `pairing/bn254_arkworks` states for `MockIPairingAdapter`
    * `[✅]`   From `random`: `IRandomSourceAdapter::fill_bytes(&self, FillBytesParams, FillBytesPayload) -> Result<FillBytesSuccessReturn, FillBytesErrorReturn>`, the draw inside a `Secret`
    * `[✅]`   From the standard library: `Instant::now()` and `Instant::elapsed()`, `Duration`'s division by `u32`, and `NonZeroU32::get()`

  * `[✅]`   `Cargo.toml`
    * `[✅]`   Adds `"apps/*"` to `[workspace] members`, which reads `["crates/*", "adapters/*", "apps/*"]`; every other table and key is unchanged

  * `[ ]`   `apps/harness-crypto/Cargo.toml`
    * `[✅]`   `[package]` with `name = "harness-crypto"`, `edition.workspace = true`, `rust-version.workspace = true`, and `publish.workspace = true`; no `version` key
    * `[✅]`   `[dependencies]` with `pairing = { path = "../../adapters/pairing" }` and `random = { path = "../../adapters/random" }`
    * `[ ]`   `[dev-dependencies]` with `pairing = { path = "../../adapters/pairing", features = ["mocks"] }`, `random = { path = "../../adapters/random", features = ["mocks"] }`, and `zeroize = "1.9.0"`
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
    * `[✅]`   Imports `IRandomSourceAdapter` and `FillBytesErrorReturn` from `random`, `SampleUniformScalarErrorReturn` and `PairingConcrete` from `pairing`, `core::num::NonZeroU32`, `core::time::Duration`, and `core::convert::Infallible`; no derives beyond those stated

  * `[✅]`   `apps/harness-crypto/src/benchmark/interaction.spec.md`
    * `[✅]`   `PairingBenchmark::try_new(params: PairingBenchmarkConstructorParams) -> PairingBenchmarkTryNewReturn`: one branch; outcome `Ok(PairingBenchmark)` holding the random source and the iteration count; the error arm has no branch
    * `[✅]`   `consume_pairing<P: IPairingAdapter>(&self, _params: ConsumePairingParams, payload: ConsumePairingPayload<P>) -> PairingBenchmarkReturn`, draw failed: condition `self.random.fill_bytes(FillBytesParams, FillBytesPayload { length: P::Scalar::UNIFORM_BYTES_LENGTH })` returns `Err(error)` for either scalar; outcome `Err(PairingBenchmarkErrorReturn::FillBytes(error))`; nothing is timed
    * `[✅]`   Sampling failed: condition `P::Scalar::sample_from_uniform_bytes(SampleUniformScalarParams, SampleUniformScalarPayload { uniform: draw.bytes })` returns `Err(error)` for either scalar; outcome `Err(PairingBenchmarkErrorReturn::SampleScalar(error))`; nothing is timed
    * `[✅]`   Measured: both scalars `a` and `b` sampled; the setup takes the generators `g1` and `g2` and computes `p1 = g1 · a` and `p2 = g2 · b` through `mul_g1` and `mul_g2`, untimed; then, for each operation in turn, `Instant::now()` is read, the operation runs `self.iterations.get()` times, and `elapsed()` divided by `self.iterations.get()` is its mean: `mul_g1` of a clone of `p1` by a clone of `a`; `mul_g2` of a clone of `p2` by a clone of `b`; `msm_g1` over the terms `MsmG1Term { base: p1, scalar: a }` and `MsmG1Term { base: g1, scalar: b }`; `msm_g2` over the corresponding `MsmG2Term` values; `pairing_product_is_one` over the terms `(p1, g2)` and `(g1, p2)`, each term's elements cloned per call; outcome `Ok(PairingBenchmarkSuccessReturn { concrete: P::CONCRETE, iterations: self.iterations, timings })`
    * `[✅]`   Every adapter call returns `Result<_, Infallible>` and is unpacked irrefutably; the scalars are cloned from their `Secret`s by `expose().clone()`, and each clone is cleared when the payload holding it drops
    * `[✅]`   Ordering: both draws and both samplings precede any timing, so a failure returns before the clock is read; `P::CONCRETE` is carried into the result; no independently supplied metadata is read

  * `[✅]`   `apps/harness-crypto/src/benchmark/mock.rs`
    * `[✅]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`
    * `[✅]`   `PairingBenchmarkConstructorParamsOverrides`, `#[derive(Default)]`, fields `pub random: Option<Box<dyn IRandomSourceAdapter>>` and `pub iterations: Option<NonZeroU32>`; `build_pairing_benchmark_constructor_params(overrides: PairingBenchmarkConstructorParamsOverrides) -> PairingBenchmarkConstructorParams`, defaulting to `Box::new(MockIRandomSourceAdapter)` and `NonZeroU32::MIN`
    * `[✅]`   `build_pairing_benchmark(overrides: PairingBenchmarkConstructorParamsOverrides) -> PairingBenchmark`, returning the real instance from `PairingBenchmark::try_new(build_pairing_benchmark_constructor_params(overrides))` through the irrefutable pattern `let Ok(benchmark) = …;`
    * `[✅]`   `PairingOperationTimingsOverrides`, `#[derive(Default)]`, one `Option<Duration>` per field; `build_pairing_operation_timings(overrides: PairingOperationTimingsOverrides) -> PairingOperationTimings`, each field defaulting to `Duration::ZERO`
    * `[✅]`   `PairingBenchmarkSuccessReturnOverrides`, `#[derive(Default)]`, fields `pub concrete: Option<PairingConcrete>`, `pub iterations: Option<NonZeroU32>`, and `pub timings: Option<PairingOperationTimings>`; `build_pairing_benchmark_success_return(overrides: PairingBenchmarkSuccessReturnOverrides) -> PairingBenchmarkSuccessReturn`, defaulting to `PairingConcrete::Bn254Arkworks`, `NonZeroU32::MIN`, and `build_pairing_operation_timings(Default::default())`
    * `[✅]`   No mock of `PairingBenchmark` or of `consume_pairing`: it is injected as an `IPairingConsumer`, whose mock `pairing` owns; no corruptions type and no invalidator, since nothing this interface owns arrives as untrusted data
    * `[✅]`   Imports `IRandomSourceAdapter` and `MockIRandomSourceAdapter` from `random`, the standard-library names above, and this module's types from `super::interface`

  * `[ ]`   `apps/harness-crypto/src/benchmark/test.rs`

  * `[✅]`   `construction`
    * `[✅]`   `PairingBenchmark::try_new` is the only producer; the harness run constructs one per concrete from the random source `create_random_source` returns and the configured iteration count, and passes it as `CreatePairingDeps { consumer }`

  * `[✅]`   `apps/harness-crypto/src/benchmark/mod.rs`
    * `[✅]`   Module declarations: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, `pub mod provides;`, and `#[cfg(test)] mod test;`
    * `[✅]`   `impl PairingBenchmark` with `pub fn try_new(params: PairingBenchmarkConstructorParams) -> PairingBenchmarkTryNewReturn` returning `Ok(PairingBenchmark { random: params.random, iterations: params.iterations })`
    * `[✅]`   `impl IPairingConsumer for PairingBenchmark` with `type Output = PairingBenchmarkReturn;` and `consume_pairing` realizing the branches and ordering of the interaction spec
    * `[✅]`   Imports the `pairing` and `random` names the context slice lists, `std::time::Instant`, and this module's types from `interface`
    * `[✅]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[✅]`   `apps/harness-crypto/src/benchmark/provides.rs`
    * `[✅]`   `pub use super::interface::*;` and `#[cfg(any(test, feature = "mocks"))] pub use super::mock::*;`, nothing else

  * `[ ]`   `apps/harness-crypto/tests/integration_test.rs`

  * `[✅]`   `directionality`
    * `[✅]`   `benchmark` depends on `pairing`'s and `random`'s public surfaces and on the standard library; it names no concrete and no curve library; nothing depends on the crate yet; no cycle

  * `[ ]`   `requirements`
    * `[✅]`   The root `Cargo.toml` lists `members = ["crates/*", "adapters/*", "apps/*"]` and is otherwise unchanged; `apps/harness-crypto/Cargo.toml` carries exactly the tables and keys stated above
    * `[✅]`   `cargo check --workspace --all-targets --all-features`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo fmt --check`, and `cargo deny check` complete without error or warning
    * `[ ]`   `consume_pairing_returns_the_sampling_error_unchanged` passes, asserting the whole error by `assert_eq!`; `PairingBenchmarkErrorReturn` derives `Debug`, `PartialEq`, and `Eq`
    * `[ ]`   Each `the_factory_hands_the_…_concrete_to_the_benchmark` integration test passes, each proving the concrete the factory constructs reaches the benchmark and every operation is timed on it (CR-10)
    * `[ ]`   `the_benchmark_samples_a_draw_from_the_operating_system_source_the_factory_constructs` passes (CR-05, a production draw passes through the randomness factory and is sampled by a concrete's sampling bound)
    * `[ ]`   Each `the_benchmark_samples_through_the_…_scalar_inside_secret` integration test and `the_benchmark_receives_each_operating_system_draw_inside_secret` pass, each proving its chain through `Secret` with every link real (CR-07, draws and scalars cross the families inside `Secret`)
    * `[✅]`   No module of `apps/harness-crypto` names `ark-`, `halo2curves`, or a pairing concrete

* `[ ]`   `domain/asset_identity` **Canonical asset identity, the package name and version whose `name@version` join is the input to the Registry's identity hash, admitted only when that join names exactly one coordinate in exactly one byte form**

  * `[✅]`   `objective`
    * `[✅]`   Problem: every record, hash-card, and derivation names an asset, and the Registry keys the asset by `BLAKE3(name@version)`, so equivalent coordinates must yield one identity and a malformed coordinate must be refused deterministically, in every process, before anything is derived from it (PR-02)
    * `[✅]`   Functional: one type holds an asset's name and version, reachable only through read accessors, and its only producer is a fallible constructor
    * `[✅]`   Functional: the constructor refuses an empty name and an empty version
    * `[✅]`   Functional: the constructor refuses any byte of the name or the version outside visible ASCII, `0x21` through `0x7E`, so no coordinate has a second byte form through whitespace, control characters, or Unicode normalization
    * `[✅]`   Functional: the constructor refuses a version containing the separator `@`, so the join `name@version` splits at its last `@` into exactly one name and one version; a name may contain `@`, as a scoped npm name does
    * `[✅]`   Functional: a refusal names the failed check and the index and byte where it failed, and the same input always yields the same refusal: the name is checked before the version, and within each string the lowest offending index decides
    * `[✅]`   Functional: `AssetIdentity` is the sole producer of the canonical `name@version` hash preimage, returned as an owned `AssetCoordinate`; a consumer borrows its bytes when hashing and never repeats the join
    * `[✅]`   Non-functional: the module depends on the standard library alone; the `domain` crate's dependencies are unchanged

  * `[✅]`   `role`
    * `[✅]`   Domain: an owned value type in the protocol and domain ring, the asset identity every later record, hash-card, and derivation context names
    * `[✅]`   Does not compute the identity hash; the domain crate names no hash library, and `BLAKE3(name@version)` is computed through the hashing family
    * `[✅]`   Does not apply any ecosystem's naming rules, case folding, trimming, or normalization; it refuses a coordinate outside its invariants and never repairs one, and an ecosystem's own naming rules belong to that ecosystem's adapters
    * `[✅]`   Takes the name and the version as separate fields and never splits a joined coordinate
    * `[✅]`   Does not create any other module of the `domain` crate
    * `[✅]`   Does not carry a commit

  * `[✅]`   `module`
    * `[✅]`   Bounded context: the `asset_identity` module of the `domain` crate, holding `AssetIdentity`, its constructor params, its constructor's error and return types, its derived `AssetCoordinate`, the coordinate separator, and the invariants a coordinate satisfies in every ecosystem
    * `[✅]`   Adds the module to the existing `domain` crate at `crates/domain`; the crate's manifest is unchanged and its barrel gains this module's line
    * `[✅]`   Outside: the identity hash, the Registry record the hash keys, the resolution of a package-manager request into a name and a version, and every ecosystem's own naming rules

  * `[✅]`   `deps`
    * `[✅]`   `domain/secret` created the `domain` crate this module joins; this module imports nothing from `secret`
    * `[✅]`   The standard library: `String`, `str`, and `u8`, through the prelude
    * `[✅]`   No external crate and no repository crate; `crates/domain/Cargo.toml` is unchanged; direction inward, `domain` is the innermost ring
    * `[✅]`   No reverse dependency; `domain/derivation_context` is this module's first consumer

  * `[✅]`   `context_slice`
    * `[✅]`   From the standard library: `str::is_empty`, `str::bytes`, `Iterator::enumerate`, `Iterator::find`, and `u8::is_ascii_graphic`, which is true exactly for `0x21` through `0x7E`

  * `[✅]`   `crates/domain/src/lib.rs`
    * `[✅]`   The crate barrel reads `mod asset_identity;`, `mod secret;`, `pub use asset_identity::provides::*;`, and `pub use secret::provides::*;`, nothing else
    * `[✅]`   Until `asset_identity/mod.rs` exists, `cargo check` reports the unresolved `mod asset_identity`, which is the RED state for every element below that precedes the implementation

  * `[✅]`   `crates/domain/src/asset_identity/interface.rs`
    * `[✅]`   `ASSET_COORDINATE_SEPARATOR`, a `pub const` of type `u8` with value `b'@'`, the byte that joins name and version in the identity hash's input
    * `[✅]`   `AssetIdentity`, a struct with `#[derive(Clone, Debug, PartialEq, Eq)]` and the fields `pub(super) name: String` and `pub(super) version: String`, so only the `asset_identity` module and its children reach the fields; raw strings are admitted only by its constructor, and display accessors remain borrowed `&str`
    * `[✅]`   `AssetCoordinate`, a struct with `pub(super) bytes: Vec<u8>` and no public constructor or mutable byte access, deriving `Clone`, `Debug`, `PartialEq`, and `Eq` and implementing `AsRef<[u8]>`; the field is visible only to the owning module and its children, and the type denotes the canonical Registry identity-hash preimage
    * `[✅]`   `AssetIdentityConstructorParams`, a struct with the fields `pub name: String` and `pub version: String`; no derives
    * `[✅]`   `AssetIdentityTryNewErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]` and the variants `EmptyName`, `NameByteOutsideVisibleAscii { index: usize, byte: u8 }`, `EmptyVersion`, `VersionByteOutsideVisibleAscii { index: usize, byte: u8 }`, and `VersionContainsSeparator { index: usize }`
    * `[✅]`   `AssetIdentityTryNewReturn`, the type alias `Result<AssetIdentity, AssetIdentityTryNewErrorReturn>`
    * `[✅]`   Imports nothing; declares nothing else

  * `[✅]`   `crates/domain/src/asset_identity/interaction.spec.md`
    * `[✅]`   `AssetIdentity::try_new(params: AssetIdentityConstructorParams) -> AssetIdentityTryNewReturn`, empty name: condition `params.name.is_empty()`; decision the emptiness check; dependency call none; outcome `Err(AssetIdentityTryNewErrorReturn::EmptyName)`
    * `[✅]`   Name byte outside visible ASCII: condition the name is non-empty and some byte of `params.name.bytes()` fails `is_ascii_graphic`; decision the first such byte by index; dependency call none; outcome `Err(AssetIdentityTryNewErrorReturn::NameByteOutsideVisibleAscii { index, byte })` for the lowest such index
    * `[✅]`   Empty version: condition the name passes and `params.version.is_empty()`; decision the emptiness check; dependency call none; outcome `Err(AssetIdentityTryNewErrorReturn::EmptyVersion)`
    * `[✅]`   Version byte outside visible ASCII or equal to the separator: condition the name passes, the version is non-empty, and some byte of `params.version.bytes()` fails `is_ascii_graphic` or equals `ASSET_COORDINATE_SEPARATOR`; decision the first such byte by index, scanned once left to right; dependency call none; outcome `Err(AssetIdentityTryNewErrorReturn::VersionByteOutsideVisibleAscii { index, byte })` when that byte fails `is_ascii_graphic`, and `Err(AssetIdentityTryNewErrorReturn::VersionContainsSeparator { index })` when it is the separator
    * `[✅]`   Admitted: condition every check passes; decision none further; dependency call none; outcome `Ok(AssetIdentity { name, version })`, both strings moved from the params without copy
    * `[✅]`   `AssetIdentity::name(&self) -> &str` and `AssetIdentity::version(&self) -> &str`: one branch each; outcome a shared reference to the held string, no copy, no side effect; `AssetIdentity::coordinate(&self) -> AssetCoordinate` allocates exactly `name.len() + 1 + version.len()` bytes and appends the name's ASCII bytes, `ASSET_COORDINATE_SEPARATOR`, and the version's ASCII bytes in that order, with no alternate join or normalization
    * `[✅]`   Ordering: the name's checks precede the version's; within each string the lowest offending index decides; the same params always yield the same outcome
    * `[✅]`   Invariants: every `AssetIdentity` holds a non-empty name and a non-empty version of visible ASCII, the version free of `@`; its only producer is `try_new`; every `AssetCoordinate` comes from an admitted `AssetIdentity` and has exactly one `name@version` byte form

  * `[✅]`   `crates/domain/src/asset_identity/mock.rs`
    * `[✅]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`
    * `[✅]`   `AssetIdentityConstructorParamsOverrides`, `#[derive(Default)]`, fields `pub name: Option<String>` and `pub version: Option<String>`
    * `[✅]`   `build_asset_identity_constructor_params(overrides: AssetIdentityConstructorParamsOverrides) -> AssetIdentityConstructorParams`, the name defaulting to `"example-package"` and the version to `"1.0.0"`
    * `[✅]`   `build_asset_identity(overrides: AssetIdentityConstructorParamsOverrides) -> AssetIdentity`, returning the real instance from `AssetIdentity::try_new(build_asset_identity_constructor_params(overrides))` through `.expect("built asset identity constructor params are admitted")`
    * `[✅]`   No corruptions type and no invalidator: the constructor params are typed strings, every coordinate the constructor refuses is a string value the params builder's overrides carry, and the crate has no serialization dependency; no `AssetIdentity` overrides, invalidator, or mock function, since the type is built as a real instance and owns no free function
    * `[✅]`   Imports `AssetIdentity` and `AssetIdentityConstructorParams` from `super::interface`

  * `[ ]`   `crates/domain/src/asset_identity/test.rs`

  * `[✅]`   `construction`
    * `[✅]`   `AssetIdentity::try_new` is the only producer of `AssetIdentity`; no `Default`, `From`, `FromStr`, or other constructor exists; a caller holding a name and a version from any source passes them as `AssetIdentityConstructorParams` and handles the refusal arm; `AssetIdentity::coordinate` is the only producer of `AssetCoordinate`

  * `[✅]`   `crates/domain/src/asset_identity/mod.rs`
    * `[✅]`   Module declarations: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, `pub mod provides;`, and `#[cfg(test)] mod test;`
    * `[✅]`   `impl AssetIdentity` with `pub fn try_new(params: AssetIdentityConstructorParams) -> AssetIdentityTryNewReturn` realizing the branches and ordering of the interaction spec, each scan by `bytes().enumerate().find(…)`, `pub fn name(&self) -> &str` returning `&self.name`, `pub fn version(&self) -> &str` returning `&self.version`, and `pub fn coordinate(&self) -> AssetCoordinate` realizing the one canonical join; `impl AsRef<[u8]> for AssetCoordinate` borrows its private bytes
    * `[✅]`   Imports `AssetIdentity`, `AssetCoordinate`, `AssetIdentityConstructorParams`, `AssetIdentityTryNewErrorReturn`, `AssetIdentityTryNewReturn`, and `ASSET_COORDINATE_SEPARATOR` from `interface`
    * `[✅]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[✅]`   `crates/domain/src/asset_identity/provides.rs`
    * `[✅]`   `pub use super::interface::*;` and `#[cfg(any(test, feature = "mocks"))] pub use super::mock::*;`, nothing else, so a sibling module's test and mock reach this module's builders in the crate's own test build as well as under the `mocks` feature

  * `[✅]`   `directionality`
    * `[✅]`   `asset_identity` depends on the standard library alone and on no other module of the crate; `domain` depends on no repository crate; later consumers reach it through `lib.rs`'s re-export of `asset_identity::provides`; no cycle

  * `[✅]`   `requirements`
    * `[✅]`   `crates/domain/Cargo.toml` is unchanged, and `crates/domain/src/lib.rs` carries exactly the barrel stated above
    * `[✅]`   `coordinate_writes_the_only_registry_hash_preimage` passes, and downstream Registry hashing consumes `AssetCoordinate::as_ref()` instead of joining the two strings again
    * `[✅]`   `cargo check --workspace --all-targets --all-features`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `cargo fmt --all --check` complete without error or warning
    * `[✅]`   `try_new_admits_a_scoped_name_and_a_prerelease_version` passes
    * `[✅]`   `try_new_rejects_an_empty_name`, `try_new_rejects_the_lowest_name_byte_outside_visible_ascii`, `try_new_rejects_a_non_ascii_name`, `try_new_rejects_an_empty_version`, `try_new_rejects_a_version_byte_outside_visible_ascii`, and `try_new_rejects_a_version_containing_the_separator` pass (PR-02, a malformed coordinate is refused)
    * `[✅]`   `try_new_reports_the_lowest_offending_version_byte_whatever_its_kind` and `try_new_reports_the_name_before_the_version` pass (PR-02, the refusal is deterministic)
    * `[✅]`   Code outside `crates/domain/src/asset_identity` reading the `name` or `version` field fails to compile

* `[ ]`   `domain/deployment_identity` **Registry-assigned deployment identity, the 32 bytes of the Registry's `bytes32` deployment key, admitted only when it can name an assigned deployment**

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
    * `[✅]`   `domain/secret` created the `domain` crate this module joins; this module imports nothing from `secret` or from `asset_identity`
    * `[✅]`   The standard library: `u8`, arrays, and `Iterator::all`, through the prelude
    * `[✅]`   No external crate and no repository crate; `crates/domain/Cargo.toml` is unchanged; direction inward, `domain` is the innermost ring
    * `[✅]`   No reverse dependency; `domain/derivation_context` is this module's first consumer

  * `[✅]`   `context_slice`
    * `[✅]`   From the standard library: `<[u8]>::iter` and `Iterator::all`

  * `[✅]`   `crates/domain/src/lib.rs`
    * `[✅]`   The crate barrel reads `mod asset_identity;`, `mod deployment_identity;`, `mod secret;`, `pub use asset_identity::provides::*;`, `pub use deployment_identity::provides::*;`, and `pub use secret::provides::*;`, nothing else
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
    * `[✅]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`
    * `[✅]`   `DeploymentIdentityConstructorParamsOverrides`, `#[derive(Default)]`, one field `pub bytes: Option<[u8; DEPLOYMENT_IDENTITY_LENGTH]>`
    * `[✅]`   `build_deployment_identity_constructor_params(overrides: DeploymentIdentityConstructorParamsOverrides) -> DeploymentIdentityConstructorParams`, the bytes defaulting to `[0x11; DEPLOYMENT_IDENTITY_LENGTH]`
    * `[✅]`   `build_deployment_identity(overrides: DeploymentIdentityConstructorParamsOverrides) -> DeploymentIdentity`, returning the real instance from `DeploymentIdentity::try_new(build_deployment_identity_constructor_params(overrides))` through `.expect("built deployment identity constructor params are admitted")`
    * `[✅]`   No corruptions type and no invalidator: the constructor params are a typed 32-byte array, the one value the constructor refuses is an array the params builder's overrides carry, and the crate has no serialization dependency; no `DeploymentIdentity` overrides, invalidator, or mock function, since the type is built as a real instance and owns no free function
    * `[✅]`   Imports `DeploymentIdentity`, `DeploymentIdentityConstructorParams`, and `DEPLOYMENT_IDENTITY_LENGTH` from `super::interface`

  * `[ ]`   `crates/domain/src/deployment_identity/test.rs`

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
    * `[✅]`   `try_new_rejects_the_all_zero_identity` passes
    * `[✅]`   Code outside `crates/domain/src/deployment_identity` reading the `bytes` field fails to compile

* `[ ]`   `domain/suite_identifier` **Cryptographic suite identifier and version, the hash-card field that fixes a deployment's whole cryptographic composition, admitted only when both can name a registered suite**

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
    * `[✅]`   `domain/secret` created the `domain` crate this module joins; this module imports nothing from `secret`, `asset_identity`, or `deployment_identity`
    * `[✅]`   The standard library: `u8`, `u16`, arrays, and `Iterator::all`, through the prelude
    * `[✅]`   No external crate and no repository crate; `crates/domain/Cargo.toml` is unchanged; direction inward, `domain` is the innermost ring
    * `[✅]`   No reverse dependency; `domain/derivation_context` is this module's first consumer

  * `[✅]`   `context_slice`
    * `[✅]`   From the standard library: `<[u8]>::iter` and `Iterator::all`

  * `[✅]`   `crates/domain/src/lib.rs`
    * `[✅]`   The crate barrel reads `mod asset_identity;`, `mod deployment_identity;`, `mod secret;`, `mod suite_identifier;`, `pub use asset_identity::provides::*;`, `pub use deployment_identity::provides::*;`, `pub use secret::provides::*;`, and `pub use suite_identifier::provides::*;`, nothing else
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
    * `[✅]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`
    * `[✅]`   `SuiteIdentifierConstructorParamsOverrides`, `#[derive(Default)]`, fields `pub identifier: Option<[u8; SUITE_IDENTIFIER_LENGTH]>` and `pub version: Option<u16>`
    * `[✅]`   `build_suite_identifier_constructor_params(overrides: SuiteIdentifierConstructorParamsOverrides) -> SuiteIdentifierConstructorParams`, the identifier defaulting to `[0x22; SUITE_IDENTIFIER_LENGTH]` and the version to `1`
    * `[✅]`   `build_suite_identifier(overrides: SuiteIdentifierConstructorParamsOverrides) -> SuiteIdentifier`, returning the real instance from `SuiteIdentifier::try_new(build_suite_identifier_constructor_params(overrides))` through `.expect("built suite identifier constructor params are admitted")`
    * `[✅]`   No corruptions type and no invalidator: the constructor params are a typed 32-byte array and a `u16`, every value the constructor refuses is one the params builder's overrides carry, and the crate has no serialization dependency; no `SuiteIdentifier` overrides, invalidator, or mock function, since the type is built as a real instance and owns no free function
    * `[✅]`   Imports `SuiteIdentifier`, `SuiteIdentifierConstructorParams`, and `SUITE_IDENTIFIER_LENGTH` from `super::interface`

  * `[ ]`   `crates/domain/src/suite_identifier/test.rs`

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
    * `[✅]`   `try_new_rejects_an_all_zero_identifier`, `try_new_rejects_a_zero_version`, and `try_new_reports_the_identifier_before_the_version` pass
    * `[✅]`   Code outside `crates/domain/src/suite_identifier` reading the `identifier` or `version` field fails to compile

* `[ ]`   `domain/parameter_set_identifier` **Parameter-set identifier, the 32 bytes of the Registry's `bytes32` parameter-set key, admitted only when it can name a registered set**

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
    * `[✅]`   `domain/secret` created the `domain` crate this module joins; this module imports nothing from `secret`, `asset_identity`, `deployment_identity`, or `suite_identifier`
    * `[✅]`   The standard library: `u8`, arrays, and `Iterator::all`, through the prelude
    * `[✅]`   No external crate and no repository crate; `crates/domain/Cargo.toml` is unchanged; direction inward, `domain` is the innermost ring
    * `[✅]`   No reverse dependency; `domain/derivation_context` is this module's first consumer

  * `[✅]`   `context_slice`
    * `[✅]`   From the standard library: `<[u8]>::iter` and `Iterator::all`

  * `[✅]`   `crates/domain/src/lib.rs`
    * `[✅]`   The crate barrel reads `mod asset_identity;`, `mod deployment_identity;`, `mod parameter_set_identifier;`, `mod secret;`, `mod suite_identifier;`, `pub use asset_identity::provides::*;`, `pub use deployment_identity::provides::*;`, `pub use parameter_set_identifier::provides::*;`, `pub use secret::provides::*;`, and `pub use suite_identifier::provides::*;`, nothing else
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
    * `[✅]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`
    * `[✅]`   `ParameterSetIdentifierConstructorParamsOverrides`, `#[derive(Default)]`, one field `pub bytes: Option<[u8; PARAMETER_SET_IDENTIFIER_LENGTH]>`
    * `[✅]`   `build_parameter_set_identifier_constructor_params(overrides: ParameterSetIdentifierConstructorParamsOverrides) -> ParameterSetIdentifierConstructorParams`, the bytes defaulting to `[0x33; PARAMETER_SET_IDENTIFIER_LENGTH]`
    * `[✅]`   `build_parameter_set_identifier(overrides: ParameterSetIdentifierConstructorParamsOverrides) -> ParameterSetIdentifier`, returning the real instance from `ParameterSetIdentifier::try_new(build_parameter_set_identifier_constructor_params(overrides))` through `.expect("built parameter set identifier constructor params are admitted")`
    * `[✅]`   No corruptions type and no invalidator: the constructor params are a typed 32-byte array, the one value the constructor refuses is an array the params builder's overrides carry, and the crate has no serialization dependency; no `ParameterSetIdentifier` overrides, invalidator, or mock function, since the type is built as a real instance and owns no free function
    * `[✅]`   Imports `ParameterSetIdentifier`, `ParameterSetIdentifierConstructorParams`, and `PARAMETER_SET_IDENTIFIER_LENGTH` from `super::interface`

  * `[ ]`   `crates/domain/src/parameter_set_identifier/test.rs`

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
    * `[✅]`   `try_new_rejects_the_all_zero_identifier` passes
    * `[✅]`   Code outside `crates/domain/src/parameter_set_identifier` reading the `bytes` field fails to compile

* `[ ]`   `domain/group_index` **Piece-group index, the position of one piece group within a deployment's continuous stream, held as a distinct type so no other integer stands in for it**

  * `[✅]`   `objective`
    * `[✅]`   Problem: every capsule's randomness, every piece-group key, and every wrapping key is domain-separated by the group it belongs to, so the group index enters each derivation context as its own type and cannot be confused with a piece index, an interval, a version, or a count (CR-05; CR-11; the nonce invariant of Deployment Cryptographic Setup)
    * `[✅]`   Functional: one type holds a piece-group index as a `u64`, reachable only through a read accessor, and its only producer is a constructor
    * `[✅]`   Functional: every `u64` is admitted; a piece group spans at least one cipher block and the MVP cipher addresses at most two to the sixty-four blocks, so every index a deployment can carry fits the type, and the index's bound against one deployment's group count is enforced where that deployment's geometry is held, by `domain/derivation_context`
    * `[✅]`   Functional: the admitted index is read back unchanged
    * `[✅]`   Non-functional: the module depends on the standard library alone; the `domain` crate's dependencies are unchanged

  * `[✅]`   `role`
    * `[✅]`   Domain: an owned value type in the protocol and domain ring, the group every later derivation context, sidecar entry, and decryption attempt names
    * `[✅]`   Does not know a deployment's piece size, piece-group size, extent, or group count, and does not bound the index against them; `domain/piece_geometry` holds the geometry and `domain/derivation_context` refuses an index outside its group count
    * `[✅]`   Does not compute an index from a byte offset or a piece index
    * `[✅]`   Does not create any other module of the `domain` crate
    * `[✅]`   Does not carry a commit

  * `[✅]`   `module`
    * `[✅]`   Bounded context: the `group_index` module of the `domain` crate, holding `GroupIndex`, its constructor params, and its constructor's return type
    * `[✅]`   Adds the module to the existing `domain` crate at `crates/domain`; the crate's manifest is unchanged and its barrel gains this module's line
    * `[✅]`   Outside: the deployment's geometry and group count, the mapping from offsets and pieces to groups, and the encoding of the index on the wire

  * `[✅]`   `deps`
    * `[✅]`   `domain/secret` created the `domain` crate this module joins; this module imports nothing from `secret`, `asset_identity`, `deployment_identity`, `suite_identifier`, or `parameter_set_identifier`
    * `[✅]`   `core::convert::Infallible`, standard library, the constructor's error arm
    * `[✅]`   No external crate and no repository crate; `crates/domain/Cargo.toml` is unchanged; direction inward, `domain` is the innermost ring
    * `[✅]`   No reverse dependency; `domain/derivation_context` is this module's first consumer

  * `[✅]`   `context_slice`
    * `[✅]`   From the standard library: `core::convert::Infallible` and `u64`; nothing else

  * `[✅]`   `crates/domain/src/lib.rs`
    * `[✅]`   The crate barrel reads `mod asset_identity;`, `mod deployment_identity;`, `mod group_index;`, `mod parameter_set_identifier;`, `mod secret;`, `mod suite_identifier;`, `pub use asset_identity::provides::*;`, `pub use deployment_identity::provides::*;`, `pub use group_index::provides::*;`, `pub use parameter_set_identifier::provides::*;`, `pub use secret::provides::*;`, and `pub use suite_identifier::provides::*;`, nothing else
    * `[✅]`   Until `group_index/mod.rs` exists, `cargo check` reports the unresolved `mod group_index`, which is the RED state for every element below that precedes the implementation

  * `[✅]`   `crates/domain/src/group_index/interface.rs`
    * `[✅]`   `GroupIndex`, a struct with `#[derive(Clone, Debug, PartialEq, Eq)]` and the one field `pub(super) value: u64`, so only the `group_index` module and its children reach the field
    * `[✅]`   `GroupIndexConstructorParams`, a struct with the one field `pub value: u64`; no derives
    * `[✅]`   `GroupIndexTryNewReturn`, the type alias `Result<GroupIndex, Infallible>`; the error arm is uninhabited because every `u64` is an index a deployment can carry
    * `[✅]`   Imports `core::convert::Infallible`; declares nothing else

  * `[✅]`   `crates/domain/src/group_index/interaction.spec.md`
    * `[✅]`   `GroupIndex::try_new(params: GroupIndexConstructorParams) -> GroupIndexTryNewReturn`: one branch; condition any params; decision none; dependency call none; outcome `Ok(GroupIndex { value })` holding `params.value`; the error arm has no branch
    * `[✅]`   `GroupIndex::value(&self) -> u64`: one branch; outcome the held index, no side effect
    * `[✅]`   Invariants: every `GroupIndex` holds one `u64`; its only producer is `try_new`; its bound against a deployment's group count is `domain/derivation_context`'s

  * `[✅]`   `crates/domain/src/group_index/mock.rs`
    * `[✅]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`
    * `[✅]`   `GroupIndexConstructorParamsOverrides`, `#[derive(Default)]`, one field `pub value: Option<u64>`
    * `[✅]`   `build_group_index_constructor_params(overrides: GroupIndexConstructorParamsOverrides) -> GroupIndexConstructorParams`, the value defaulting to `7`
    * `[✅]`   `build_group_index(overrides: GroupIndexConstructorParamsOverrides) -> GroupIndex`, returning the real instance from `GroupIndex::try_new(build_group_index_constructor_params(overrides))` through the irrefutable pattern `let Ok(index) = …;`
    * `[✅]`   No corruptions type and no invalidator: the constructor params are a typed `u64` and the constructor admits every value; no `GroupIndex` overrides, invalidator, or mock function, since the type is built as a real instance and owns no free function
    * `[✅]`   Imports `GroupIndex` and `GroupIndexConstructorParams` from `super::interface`

  * `[ ]`   `crates/domain/src/group_index/test.rs`

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

  * `[ ]`   `requirements`
    * `[✅]`   `crates/domain/Cargo.toml` is unchanged, and `crates/domain/src/lib.rs` carries exactly the barrel stated above
    * `[✅]`   `cargo check --workspace --all-targets --all-features`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `cargo fmt --all --check` complete without error or warning
    * `[✅]`   Code outside `crates/domain/src/group_index` reading the `value` field fails to compile

* `[ ]`   `domain/piece_geometry` **Piece geometry, the piece size, piece-group size, and total extent a hash-card declares, admitted only when pieces align and groups are whole multiples of pieces, with the piece and group counts they imply**

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
    * `[✅]`   Does not bound a piece index or a group index; `domain/derivation_context` bounds the group index against this geometry's group count, and the transport bounds a piece index against its piece count
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
    * `[✅]`   No reverse dependency; `domain/derivation_context` is this module's first consumer

  * `[✅]`   `context_slice`
    * `[✅]`   From the standard library: `u32::is_power_of_two`, which is false for `0`; the `%` remainder over `u32`; `u64::from(u32)`, the lossless widening; and `u64::div_ceil`, rounding the quotient up

  * `[✅]`   `crates/domain/src/lib.rs`
    * `[✅]`   The crate barrel reads `mod asset_identity;`, `mod deployment_identity;`, `mod group_index;`, `mod parameter_set_identifier;`, `mod piece_geometry;`, `mod secret;`, `mod suite_identifier;`, `pub use asset_identity::provides::*;`, `pub use deployment_identity::provides::*;`, `pub use group_index::provides::*;`, `pub use parameter_set_identifier::provides::*;`, `pub use piece_geometry::provides::*;`, `pub use secret::provides::*;`, and `pub use suite_identifier::provides::*;`, nothing else
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
    * `[✅]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`
    * `[✅]`   `PieceGeometryConstructorParamsOverrides`, `#[derive(Default)]`, fields `pub piece_size: Option<u32>`, `pub piece_group_size: Option<u32>`, and `pub total_extent: Option<u64>`
    * `[✅]`   `build_piece_geometry_constructor_params(overrides: PieceGeometryConstructorParamsOverrides) -> PieceGeometryConstructorParams`, the piece size defaulting to `MINIMUM_PIECE_SIZE`, the piece-group size to `16384`, the configured default of 16 KiB, and the total extent to `1048576`
    * `[✅]`   `build_piece_geometry(overrides: PieceGeometryConstructorParamsOverrides) -> PieceGeometry`, returning the real instance from `PieceGeometry::try_new(build_piece_geometry_constructor_params(overrides))` through `.expect("built piece geometry constructor params are admitted")`
    * `[✅]`   No corruptions type and no invalidator: the constructor params are typed integers, every geometry the constructor refuses is a value the params builder's overrides carry, and the crate has no serialization dependency; no `PieceGeometry` overrides, invalidator, or mock function, since the type is built as a real instance and owns no free function
    * `[✅]`   Imports `PieceGeometry`, `PieceGeometryConstructorParams`, and `MINIMUM_PIECE_SIZE` from `super::interface`

  * `[ ]`   `crates/domain/src/piece_geometry/test.rs`

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

  * `[ ]`   `requirements`
    * `[✅]`   `crates/domain/Cargo.toml` is unchanged, and `crates/domain/src/lib.rs` carries exactly the barrel stated above
    * `[✅]`   `cargo check --workspace --all-targets --all-features`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `cargo fmt --all --check` complete without error or warning
    * `[ ]`   `try_new_admits_the_smallest_geometry`, `piece_count_rounds_up_a_partial_piece`, `group_count_rounds_up_a_partial_group`, `piece_count_does_not_round_up_an_exact_extent`, `group_count_does_not_round_up_an_exact_extent`, `piece_count_holds_at_the_largest_extent`, and `group_count_holds_at_the_largest_extent` pass
    * `[✅]`   `try_new_rejects_a_piece_size_that_is_not_a_power_of_two`, `try_new_rejects_a_zero_piece_size`, `try_new_rejects_a_piece_size_below_the_minimum`, `try_new_rejects_a_zero_piece_group_size`, `try_new_rejects_a_piece_group_size_that_is_not_a_multiple_of_the_piece_size`, `try_new_rejects_a_piece_group_smaller_than_a_piece`, `try_new_rejects_a_zero_total_extent`, and `try_new_reports_the_piece_size_before_the_total_extent` pass (EC-01, CR-06, a malformed geometry is refused before any key is derived)
    * `[✅]`   Code outside `crates/domain/src/piece_geometry` reading the `piece_size`, `piece_group_size`, or `total_extent` field fails to compile

* `[ ]`   `domain/derivation_context` **Derivation context, the asset, deployment, suite, parameter set, group index, and geometry every wrapping key and lineage derivation is domain-separated by, admitted only when the group index falls within the geometry's group count; the first encodable domain type**

  * `[✅]`   `objective`
    * `[✅]`   Problem: each parameter set's wrapping key is a domain-separated KDF of the encapsulated value and the context, asset, deployment, suite, parameter set, group index, and geometry, so those six values travel together as one type, and a context naming a group the deployment does not have is refused before any key is derived from it (CR-11; Credential KEM; Manifest Bounds Validation)
    * `[✅]`   Functional: one type holds an `AssetIdentity`, a `DeploymentIdentity`, a `SuiteIdentifier`, a `ParameterSetIdentifier`, a `GroupIndex`, and a `PieceGeometry`, each reachable only through a read accessor, and its only producer is a fallible constructor
    * `[✅]`   Functional: the constructor refuses a group index at or beyond the geometry's group count, naming both values
    * `[✅]`   Functional: every other combination of admitted components is admitted and read back unchanged
    * `[✅]`   Non-functional: the module depends on the `domain` crate's own identifier and geometry modules and the standard library alone; the `domain` crate's dependencies are unchanged

  * `[✅]`   `role`
    * `[✅]`   Domain: an owned value type in the protocol and domain ring composing the domain's identifier and geometry types, the first encodable domain type, whose canonical description the encoding family authors next
    * `[✅]`   Does not re-check any component's own invariants; each arrives as an admitted instance of its type
    * `[✅]`   Does not encode itself, derive a key, or hash; `encoding/derivation_context` describes its canonical field sequence and `kdf/blake3_keyed` derives from the encoding
    * `[✅]`   Does not check that the parameter set is live for the asset, that the deployment belongs to the asset, or that the suite admits the geometry; those are Registry state and suite declarations the manifest gate reads
    * `[✅]`   Does not create any other module of the `domain` crate
    * `[✅]`   Does not carry a commit

  * `[✅]`   `module`
    * `[✅]`   Bounded context: the `derivation_context` module of the `domain` crate, holding `DerivationContext`, its constructor params, its constructor's error and return types, and the one cross-field rule between the group index and the geometry
    * `[✅]`   Adds the module to the existing `domain` crate at `crates/domain`; the crate's manifest is unchanged and its barrel gains this module's line
    * `[✅]`   Outside: each component's own invariants, the context's encoding, every derivation computed from it, and every Registry and suite rule

  * `[✅]`   `deps`
    * `[✅]`   `domain/asset_identity`, same crate, protocol and domain ring, through `crate::asset_identity::provides`; supplies `AssetIdentity`, and in the crate's test build and under the `mocks` feature `build_asset_identity` and `AssetIdentityConstructorParamsOverrides`
    * `[✅]`   `domain/deployment_identity`, through `crate::deployment_identity::provides`; supplies `DeploymentIdentity`, and in the crate's test build and under the `mocks` feature `build_deployment_identity` and `DeploymentIdentityConstructorParamsOverrides`
    * `[✅]`   `domain/suite_identifier`, through `crate::suite_identifier::provides`; supplies `SuiteIdentifier`, and in the crate's test build and under the `mocks` feature `build_suite_identifier` and `SuiteIdentifierConstructorParamsOverrides`
    * `[✅]`   `domain/parameter_set_identifier`, through `crate::parameter_set_identifier::provides`; supplies `ParameterSetIdentifier`, and in the crate's test build and under the `mocks` feature `build_parameter_set_identifier` and `ParameterSetIdentifierConstructorParamsOverrides`
    * `[✅]`   `domain/group_index`, through `crate::group_index::provides`; supplies `GroupIndex` with `value()`, and in the crate's test build and under the `mocks` feature `build_group_index` and `GroupIndexConstructorParamsOverrides`
    * `[✅]`   `domain/piece_geometry`, through `crate::piece_geometry::provides`; supplies `PieceGeometry` with `group_count()`, and in the crate's test build and under the `mocks` feature `build_piece_geometry` and `PieceGeometryConstructorParamsOverrides`
    * `[✅]`   Each is a module of the same ring that this type composes; none depends on `derivation_context`, so no cycle forms; no external crate and no other repository crate; `crates/domain/Cargo.toml` is unchanged
    * `[✅]`   No reverse dependency; `encoding/derivation_context` is this module's first consumer

  * `[✅]`   `context_slice`
    * `[✅]`   From `group_index`: `GroupIndex::value(&self) -> u64`
    * `[✅]`   From `piece_geometry`: `PieceGeometry::group_count(&self) -> u64`
    * `[✅]`   From the other components: their types, moved into the context and returned by reference; no method is called on them
    * `[✅]`   From each component's mocks: its builder, taking its constructor-params overrides and returning a real instance

  * `[✅]`   `crates/domain/src/lib.rs`
    * `[✅]`   The crate barrel reads `mod asset_identity;`, `mod deployment_identity;`, `mod derivation_context;`, `mod group_index;`, `mod parameter_set_identifier;`, `mod piece_geometry;`, `mod secret;`, `mod suite_identifier;`, `pub use asset_identity::provides::*;`, `pub use deployment_identity::provides::*;`, `pub use derivation_context::provides::*;`, `pub use group_index::provides::*;`, `pub use parameter_set_identifier::provides::*;`, `pub use piece_geometry::provides::*;`, `pub use secret::provides::*;`, and `pub use suite_identifier::provides::*;`, nothing else
    * `[✅]`   Until `derivation_context/mod.rs` exists, `cargo check` reports the unresolved `mod derivation_context`, which is the RED state for every element below that precedes the implementation

  * `[✅]`   `crates/domain/src/derivation_context/interface.rs`
    * `[✅]`   `DerivationContext`, a struct with `#[derive(Clone, Debug, PartialEq, Eq)]` and the fields, in this order, `pub(super) asset: AssetIdentity`, `pub(super) deployment: DeploymentIdentity`, `pub(super) suite: SuiteIdentifier`, `pub(super) parameter_set: ParameterSetIdentifier`, `pub(super) group_index: GroupIndex`, and `pub(super) geometry: PieceGeometry`, so only the `derivation_context` module and its children reach the fields
    * `[✅]`   `DerivationContextConstructorParams`, a struct with the same fields in the same order, each `pub`; no derives
    * `[✅]`   `DerivationContextTryNewErrorReturn`, an enum with `#[derive(Debug, PartialEq, Eq)]` and the one variant `GroupIndexOutOfRange { group_index: u64, group_count: u64 }`
    * `[✅]`   `DerivationContextTryNewReturn`, the type alias `Result<DerivationContext, DerivationContextTryNewErrorReturn>`
    * `[✅]`   Imports `AssetIdentity`, `DeploymentIdentity`, `SuiteIdentifier`, `ParameterSetIdentifier`, `GroupIndex`, and `PieceGeometry` from their modules' `provides`; declares nothing else

  * `[✅]`   `crates/domain/src/derivation_context/interaction.spec.md`
    * `[✅]`   `DerivationContext::try_new(params: DerivationContextConstructorParams) -> DerivationContextTryNewReturn`, group index out of range: condition `params.group_index.value() >= params.geometry.group_count()`; decision the comparison; dependency call `GroupIndex::value` and `PieceGeometry::group_count`, once each; outcome `Err(DerivationContextTryNewErrorReturn::GroupIndexOutOfRange { group_index, group_count })` holding the two values compared
    * `[✅]`   Admitted: condition the group index is below the group count; decision the same comparison; dependency call the same two reads; outcome `Ok(DerivationContext { asset, deployment, suite, parameter_set, group_index, geometry })`, every component moved from the params
    * `[✅]`   `DerivationContext::asset(&self) -> &AssetIdentity`, `deployment(&self) -> &DeploymentIdentity`, `suite(&self) -> &SuiteIdentifier`, `parameter_set(&self) -> &ParameterSetIdentifier`, `group_index(&self) -> &GroupIndex`, and `geometry(&self) -> &PieceGeometry`: one branch each; outcome a shared reference to the held component, no copy, no side effect
    * `[✅]`   Invariants: every `DerivationContext` holds six admitted components and a group index below its geometry's group count; its only producer is `try_new`

  * `[✅]`   `crates/domain/src/derivation_context/mock.rs`
    * `[✅]`   Module-level `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::as_conversions)]`
    * `[✅]`   `DerivationContextConstructorParamsOverrides`, `#[derive(Default)]`, fields `pub asset: Option<AssetIdentity>`, `pub deployment: Option<DeploymentIdentity>`, `pub suite: Option<SuiteIdentifier>`, `pub parameter_set: Option<ParameterSetIdentifier>`, `pub group_index: Option<GroupIndex>`, and `pub geometry: Option<PieceGeometry>`
    * `[✅]`   `build_derivation_context_constructor_params(overrides: DerivationContextConstructorParamsOverrides) -> DerivationContextConstructorParams`, each omitted field defaulting to its component's builder called with `Default::default()`: `build_asset_identity`, `build_deployment_identity`, `build_suite_identifier`, `build_parameter_set_identifier`, `build_group_index`, and `build_piece_geometry`; the default group index `7` falls within the default geometry's `64` groups
    * `[✅]`   `build_derivation_context(overrides: DerivationContextConstructorParamsOverrides) -> DerivationContext`, returning the real instance from `DerivationContext::try_new(build_derivation_context_constructor_params(overrides))` through `.expect("built derivation context constructor params are admitted")`
    * `[✅]`   No corruptions type and no invalidator: every component arrives as an admitted instance, the one context the constructor refuses is a combination the params builder's overrides carry, and the crate has no serialization dependency; no `DerivationContext` overrides, invalidator, or mock function, since the type is built as a real instance and owns no free function
    * `[✅]`   Imports `DerivationContext` and `DerivationContextConstructorParams` from `super::interface`, and each component's type, builder, and constructor-params overrides from its module's `provides`, which re-exports its mocks in the crate's test build and under the `mocks` feature

  * `[ ]`   `crates/domain/src/derivation_context/test.rs`

  * `[✅]`   `construction`
    * `[✅]`   `DerivationContext::try_new` is the only producer; no `Default`, `From`, or other constructor exists; a caller holding admitted components passes them as `DerivationContextConstructorParams` and handles the refusal arm

  * `[✅]`   `crates/domain/src/derivation_context/mod.rs`
    * `[✅]`   Module declarations: `mod interface;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, `pub mod provides;`, and `#[cfg(test)] mod test;`
    * `[✅]`   `impl DerivationContext` with `pub fn try_new(params: DerivationContextConstructorParams) -> DerivationContextTryNewReturn` realizing the branches of the interaction spec, and the six accessors the interaction spec states, each returning `&self.` its field
    * `[✅]`   Imports `DerivationContext`, `DerivationContextConstructorParams`, `DerivationContextTryNewErrorReturn`, and `DerivationContextTryNewReturn` from `interface`, and the component types from their modules' `provides`
    * `[✅]`   No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`

  * `[✅]`   `crates/domain/src/derivation_context/provides.rs`
    * `[✅]`   `pub use super::interface::*;` and `#[cfg(any(test, feature = "mocks"))] pub use super::mock::*;`, nothing else, so a sibling module's test and mock reach this module's builders in the crate's own test build as well as under the `mocks` feature

  * `[✅]`   `directionality`
    * `[✅]`   `derivation_context` depends on `asset_identity`, `deployment_identity`, `suite_identifier`, `parameter_set_identifier`, `group_index`, and `piece_geometry` through their `provides`, and on the standard library; none of them depends on it; `domain` depends on no repository crate; `encoding/derivation_context` reaches it through `lib.rs`'s re-export of `derivation_context::provides`; no cycle

  * `[✅]`   `requirements`
    * `[✅]`   `crates/domain/Cargo.toml` is unchanged, and `crates/domain/src/lib.rs` carries exactly the barrel stated above
    * `[✅]`   `cargo check --workspace --all-targets --all-features`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `cargo fmt --all --check` complete without error or warning
    * `[✅]`   `try_new_admits_the_last_group_of_the_geometry` and `try_new_admits_index_zero_of_a_one_group_geometry` pass
    * `[✅]`   `try_new_rejects_a_group_index_equal_to_the_group_count` and `try_new_rejects_the_largest_group_index` pass (CR-11, a context names only a group its deployment has)
    * `[✅]`   Code outside `crates/domain/src/derivation_context` reading any field of `DerivationContext` fails to compile

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
