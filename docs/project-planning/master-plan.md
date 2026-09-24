<!-- Template: parenthesis_master_plan.md -->
# Index

Draft, 2026-09-23. The master plan for the ChainTorrent MVP: the high-level view of the build that summarizes every planning document before it into one cohesive statement of what is built, in what order, under what rules, and with what status. The [milestones](milestones.md) hold the definitions this plan summarizes; the workplan's Work Breakdown Structure will hold the nodes; this plan holds the state. Groupings are addressed by delivery role.

- Executive Summary
- Implementation Phases, as delivery groupings by role
- Status Summary and Status Markers
- Dependency Rules and Generation Limits
- Feature Scope, Features, MVP Description
- Market Opportunity and Competitive Analysis
- Technical Context, Implementation Context, Test Framework, Component Mapping
- Architecture Summary, Architecture, Services, Components, Integration Points, Dependency Resolution
- Frontend Stack, Backend Stack, Data Platform, DevOps Tooling, Security Tooling, Shared Libraries, Third Party Services

Sources, in the order the pipeline produced them: [revised business case](business-case-revised.md), [feature spec](feature-spec.md), [success metrics](success-metrics.md), [technical approach](technical-approach.md), [risk register](risk-register.md), [non-functional review](non-functional-requirements.md), [dependency map](dependency-map.md), [feasibility assessment](technical-feasibility.md), [product requirements](product-requirements.md), [tech stack](tech-stack.md), [system architecture](system-architecture.md), [technical requirements](technical-requirements.md), [milestones](milestones.md); and the specifications under [docs/research](../research/) with the [workplan](../workplans/current/ChainTorrent%20MVP.md).

# Executive Summary

ChainTorrent is a protocol that distributes encrypted assets over a peer swarm and lets the holder of a transferable, irrevocable access right decrypt them from nothing but their own credential and a fresh view of public ledger state, with no service on the read path. The MVP applies it to JavaScript dependencies: a developer installs one extension or one npm package, consents once, and thereafter installs from whichever of a machine-wide cache, a peer swarm, or the registry delivers within the declared budget, never slower than the registry alone by more than the hedge delay, at $0.00, with a first run leaving the machine independent of the registry for everything it installed and the swarm holding what anyone fetched and serving when the registry does not. The cryptographic problem that once made the design impossible is closed and adopted; technical feasibility is high; delivery feasibility is calibrated by the first delivery grouping.

The build runs through delivery groupings addressed by role. The foundation grouping establishes the workspace, the canonical encoding, the redaction layer, and continuous integration. The cryptographic validation harness grouping builds the pairing adapters, credential KEM, envelope, delivery proof, and the shipped Solidity verifier, measures them on Base Sepolia, fixes the piece-group size, confirms the curve, and calibrates throughput. The protocol core grouping builds hashing, signatures, and the swarm transport over the `librqbit` soft fork beside the harness, and the cipher and sidecar layer, the contract suite, and the chain adapters after it. The daemon grouping builds the single machine-level process every requirement family meets, closes the read path with credential delivery and per-attempt authorization, completing a first run's independence, and reaches the demonstrable milestone where an ordinary `npm install` is served at the registry's speed, registers and prefetches, and a second identity resolves against the swarm with the registry down. The shells and services grouping builds installation on Windows, macOS, and Linux, the control surfaces, the relayer, the publisher path, the claim verifier, the site with its WebAssembly demonstration, observability, and the demonstration harness. The acceptance and release grouping runs the priced transaction flow proof on the Base mainnet pilot, closes external review and legal prerequisites, passes every acceptance scenario on clean machines, and releases with the dogfood baseline recorded.

Nothing is started. The workplan names its opening node when authored, from the foundation tickets. The stop criteria sit where they can first be evaluated, and the plan halts at any of them.

# Implementation Phases

The template's word is phases; here they are groupings addressed by dependency role. Each grouping below names its milestones in dependency order, its entry, its exit, and the stop criterion it carries, if any. Definitions are in the [milestones](milestones.md).

## Foundation grouping

**Milestones.** Workspace and discipline bootstrap.

**Entry.** The empty repository.

**Exit.** The workspace builds and passes checks on all three platforms; the domain crate skeleton depends on nothing outside itself, its types authored later in the nodes that first consume them.

**Yields.** The substrate every later grouping consumes, and signing enrollment started so certificates exist before onboarding.

## Cryptographic validation harness grouping

**Milestones.** Pairing adapters and key derivation; credential KEM; envelope; delivery proof and Solidity verifier; harness vectors, measurement, and report.

**Entry.** Foundation grouping closed. External cryptographic review starts on the composition claims, including the multi-set sidecar layer and the provenance boundary, at this grouping's start. The latency budget and its measurement conditions are declared before the report milestone selects anything.

**Exit.** CD-07 and AS-21: sizes, timings, and delivery cost as L2 execution and L1 data fee on both curves; the piece-group size selected and the curve confirmed against the declared budget and recorded in release evidence; cross-set agreement among the vectors; the verifier deployed on Base Sepolia as shipped code; throughput recorded.

**Stop criteria.** No parameter within the specification's bounds meets the latency budget; or the post-harness estimate of remaining work, produced by re-mapping the next grouping to tickets from measured throughput, is beyond what the project can fund.

## Protocol core and contract suite grouping

**Milestones.** Hashing and commitments, signature services and binding schema, and swarm transport and seed host, at ticket resolution beside the harness with no chain dependency; payload cipher and sidecar layer; registry and entitlement contracts; chain, settlement, and entitlement-state adapters.

**Entry.** The hashing, signature, and swarm milestones enter on the foundation grouping; the remaining milestones enter when the harness report closes and are re-mapped to tickets.

**Exit.** CR-01, CR-02, CR-03, CR-06, with a two-set sidecar opening one ciphertext and the sample deployment bundled; LC-01 through LC-13 on Base Sepolia; XA-06 quorum views through the decision table; SW-01 through SW-07 with retrieval through the owned host and a delegated client and Bao rejection of a piece that passes SHA-1.

**Yields.** A ledger, a way to read it from a quorum of nodes, and bytes moving between machines.

## Local daemon and package serving grouping

**Milestones.** Daemon skeleton with jobs, configuration, settings, resolver, health, telemetry, and IPC; identity and custody; plaintext CAS, resolution orchestrator, and package host; First Finder engine; credential delivery and per-attempt authorization; the demonstrable milestone.

**Entry.** Protocol core grouping closed; this grouping re-mapped to tickets.

**Exit.** XA-02 under the same-user threat model, XA-03, XA-10; ST-01 through ST-05 and ST-08; IW-01 through IW-07; PR-01 through PR-12 and AS-06, AS-29; FF-01 through FF-08 and AS-09 through AS-12; EC-01 through EC-08 and AS-07, AS-08, AS-19; CD-01 through CD-05, CD-08 and AS-14, AS-26.

**Stop criterion.** At the demonstrable milestone, the north star and latency guardrail on the dogfood population show the benefit is not felt.

**Yields.** A daemon that serves packages from whichever of cache, swarm, and registry delivers within budget, never slower than the registry alone by more than the hedge delay, registers and prefetches in the background so a first run ends independent, bootstraps absent packages, and decrypts with nothing on the read path.

## Onboarding shells and services grouping

**Milestones.** Installation coordinator and platform adapters; relayer or paymaster, in parallel; CLI and desktop application; Visual Studio Code extension and npm bootstrap package; explicit publisher path and issuance policy, in parallel; claim verifier and escrow claim; project seed host, site, and WebAssembly demonstration; observability reconciliation; demonstration harness, running alongside from the daemon grouping and closing here.

**Entry.** Daemon grouping closed, except that the relayer milestone enters on the chain adapters alone and may start earlier; this grouping re-mapped to tickets; signing certificates in hand under XA-08's custody discipline.

**Exit.** SI-01 through SI-20, IC-01 through IC-10, ST-06, ST-07 and AS-01 through AS-05, AS-18, AS-30; RO-01, RO-02, LC-10 and AS-27; PC-01 through PC-07, CD-06, FF-09 and AS-16, AS-22, AS-23; RO-03 through RO-08, XA-04, XA-09; XA-07 with every proof class mapped to a facility.

**Yields.** Both onboarding paths to one postcondition, every control surface, the free path sponsored, the project's own package published, maintainers able to claim, the site live, and a harness that can run every scenario unattended.

## Acceptance and release grouping

**Milestones.** Transaction flow proof on the Base mainnet pilot; external review and legal prerequisites closed; acceptance run; dogfood baseline and release.

**Entry.** Every prior grouping closed; review dispositioned before any priced deployment.

**Exit.** LC-03, LC-05, CD-02, RO-02 and AS-15; every finding and legal item recorded as release evidence; every acceptance scenario through packaged applications on clean machines with every guardrail holding; release evidence complete under the completion boundary.

**Stop criteria.** A composition or verifier finding not dispositioned halts before the priced deployment; any guardrail breach in the acceptance run blocks release and no guardrail may be waived by adjusting the metric.

# Status Summary

| Grouping | Milestones | Status |
| --- | --- | --- |
| Foundation | Workspace and discipline bootstrap | Unstarted |
| Cryptographic validation harness | Pairing adapters and key derivation; credential KEM; envelope; delivery proof and Solidity verifier; harness vectors, measurement, and report | Unstarted; mapped at ticket resolution |
| Protocol core and contract suite | Hashing and commitments; signature services; swarm transport and seed host; payload cipher and sidecar layer; registry and entitlement contracts; chain adapters | Unstarted; hashing, signatures, and swarm at ticket resolution, the rest as sprints |
| Local daemon and package serving | Daemon skeleton; identity and custody; CAS and package host; First Finder engine; credential delivery and per-attempt authorization; demonstrable milestone | Unstarted; mapped as epics |
| Onboarding shells and services | Installation coordinator; relayer; CLI and desktop; extension and npm bootstrap; explicit publisher path; claim verifier and escrow claim; seed host, site, and demonstration; observability reconciliation; demonstration harness | Unstarted; mapped as milestones |
| Acceptance and release | Transaction flow proof; review and legal closed; acceptance run; dogfood baseline and release | Unstarted; mapped as objectives |

Decisions: every blocking selection is resolved except the multi-device custody sync mechanism, which proceeds on its recorded recommendation if unconfirmed, and the measured parameters, which the harness fixes. Compliance items on the release path are unstarted and run in parallel with the harness. The terminal proof surface is bound. No node exists.

# Status Markers

The plan uses the workplan's own vocabulary so that the master plan and the Work Breakdown Structure never disagree about state.

- `[ ]` unstarted: no node of the milestone has been authored or begun.
- `[~]` in progress: at least one node authored or under implementation; the milestone's chain has not committed.
- `[✅]` closed: the last node of the milestone's chain has committed with its integration test green, and the milestone's exit rows are proven.
- `[!]` halted: a stop criterion was met or a discovery revised the map; the milestone's scope is restated in full and the plan waits on the decision.

A milestone is closed only by its commit. A grouping is closed when every milestone in it is closed. The updated master plan, produced when a grouping closes, marks the next grouping's milestones as the next work and records the re-map.

# Dependency Rules

- **Producers before consumers, always.** A milestone starts only when every milestone its entry names has closed; within a milestone, nodes are dependency-ordered and a dependent node moves after its provider.
- **Dependencies point inward.** The domain crate depends on nothing outside itself; workflows depend on domain and adapter interfaces only; adapters and shells are outermost. A violation fails to compile.
- **Nothing depends on an undecided selection.** No node is authored against an open decision; the multi-device sync mechanism proceeds on its recorded recommendation; the identity's chain-level form waits for the relayer milestone under LC-13; every node that encrypts a registered deployment waits on the harness report for the piece-group size.
- **Independent milestones may run together.** Hashing, signatures, and the swarm transport run beside the harness and the contract milestones; the relayer and the publisher path run beside the installer; the demonstration harness runs beside the daemon grouping. The map's edges decide, not a calendar.
- **External work starts at the harness.** Cryptographic review, legal drafting and review, and signing enrollment all begin when the harness grouping begins, so they gate nothing late.
- **Deferred items are not dependencies.** Nothing in scope depends on any deferred feature; the deferred tier's constraints are recorded so that in-scope choices do not block it.
- **Resolution decays outward.** The foundation, the harness, and the milestones that depend on nothing it measures are at ticket resolution; the next grouping is re-mapped when the current one closes; the plan and the map are revised in place.

# Generation Limits

What this plan and the documents under it deliberately do not generate.

- **No dates, headcount, or budget.** No source supports them; the harness grouping's measured throughput is the calibration that produces the first estimate, and the funding decision is made then.
- **No ordinals on groupings, milestones, sprints, epics, or nodes.** Everything is addressed by role and relation.
- **No nodes.** Nodes are authored through the workplan's ordinary path from the tickets a milestone names, one source file per node, in the fixed element order; this plan holds none.
- **No tickets beyond the grouping in progress and the milestones that depend on nothing it measures.** Detail for distant groupings is authored when they are re-mapped, never before.
- **No thresholds the sources leave unmeasured.** The latency budget is the only threshold that fails a run, declared before measurement; every cost and cryptographic metric is an observed value whose first measurement is the baseline.
- **No settings that weaken the protocol.** The catalogue admits nothing that skips the state view, weakens a freshness bound, persists decrypt-capable material, disables destruction at HARD, or serves unverified plaintext.
- **No claims from outside the repository unlabelled.** Where general knowledge is used, it is marked external context.
- **The repository's process topics govern every implementation turn.**

# Feature Scope

The features in scope, specified with objective, user stories, acceptance criteria, dependencies, and success metrics in the [feature spec](feature-spec.md) and mapped to milestones in the milestones document's Features Context. Deferred and architecturally protected: Git commit wrapping, an escrow email bot, general paid monetization, a version alignment engine, media and streaming, content flagging, per-entitlement variance, composable containers, partial encryption, seeder compensation, retention enforcement, Sybil-resistant stake, and the website account, remote head, and hosted instance. The travelling-developer story is recorded as an anticipated capability with the list of MVP choices that must not block it.

# Features

| Feature | Milestone that delivers it |
| --- | --- |
| Cryptographic services and validation harness | Harness grouping; hashing and payload cipher milestones |
| Adapter composition and capability resolution | Daemon skeleton |
| Entitlement ledger, contracts, and settlement | Registry and entitlement contracts; chain adapters |
| Swarm transport, peer discovery, and seed hosting | Swarm transport and seed host; project seed host and site |
| Package serving and local CAS | Plaintext CAS, resolution orchestrator, and package host |
| Encrypted content consumption and per-attempt authorization | Credential delivery and per-attempt authorization |
| First Finder bootstrap and escrow | First Finder engine |
| Credential delivery at mint, grant, and sale | Credential delivery and per-attempt authorization; claim verifier and escrow claim |
| Identity, wallet, and key custody | Identity and custody |
| Settings and configuration | Daemon skeleton; CLI and desktop for the surfaces |
| Self-installing onboarding | Installation coordinator; extension and npm bootstrap |
| Relaying and free-path subsidy | Relayer or paymaster |
| Explicit publishing and escrow claim | Explicit publisher path; claim verifier and escrow claim |
| Project seed host and site | Project seed host, site, and WebAssembly demonstration |
| Observability, diagnostics, and cost instrumentation | Daemon skeleton; observability reconciliation |
| Test facilities and demonstration harness | Demonstration harness; acceptance run |
| Transaction flow proof | Transaction flow proof on the Base mainnet pilot |

# MVP Description

A working install path for JavaScript dependencies that serves packages from whichever of the local cache, a peer swarm, or the registry delivers within budget, never slower than the registry alone by more than the hedge delay, reuses across every project on a machine whatever that machine has fetched, leaves a machine independent of the registry and of every other participant for everything its first run installed, survives an upstream registry outage for packages already ingested, and exercises the full identity, entitlement, credential-delivery, encryption, and transfer lifecycle end to end. It validates distribution and identity and does not validate willingness to pay or seeder compensation. $0.00 throughout, with one priced transaction on the project's own package to prove settlement. Deployables: the onboarding shells converging on one Rust installer, a Tauri desktop application, a CLI, a single-instance daemon and package host, a validation harness, a relayer, a claim verifier, a Solidity contract suite on Base, a demonstration harness, and a project seed host and site with a WebAssembly demonstration. Completion is every acceptance scenario through packaged applications on clean Windows, macOS, and Linux machines with no test-only bypass. The [product requirements](product-requirements.md) hold the full description.

# Market Opportunity

Layered and reached through adapters, unsized by design. Beachhead: JavaScript dependencies, adopted by individual developers, with build platforms following as superseeders for their own economic reasons and CI and enterprise following them. Adjacent: every other package ecosystem as a peer adapter. Expansion: priced content classes where an irrevocable, resellable entitlement is the product. Platform: the project's own chain, tokens, keystore, and swarm client, arriving as adapters. The [revised business case](business-case-revised.md) holds the argument and its labelled external context.

# Competitive Analysis

No existing system combines one canonical encrypted swarm object, transferable irrevocable on-chain entitlements, per-holder decryption with nothing on the read path, and plaintext the user keeps; every system that enforces rights puts a service or device on the read path, and every system that avoids the read path enforces no rights. At the beachhead the honest position is parity for machine-wide reuse and for enterprise outage insulation, with the advantage in the long tail, swarm accumulation, and a rights layer no developer notices at $0.00. Discovery, presentation, commerce, indexing, and storage are invited to compete on top of the protocol under a replaceability requirement. The revised business case holds the table.

# Technical Context

Rust throughout, Tauri for GUI surfaces, TypeScript only where Visual Studio Code and npm impose it, Solidity for the EVM. Three inward-pointing rings as crates. Every external touchpoint an adapter with declared capabilities; composition validated at resolution and failing closed; an immutable deployment suite as the unit of cryptographic composition. Base as the launch network with Base Sepolia for testing; BLS12-381 primary through EIP-2537 with BN254 retained; settlement tiers mapped to Base. A depth-one Boneh–Boyen KEM in Type-3 groups with seller-side rerandomization, ElGamal envelopes, and chained Schnorr delivery proofs, secure under SXDH, decisional BDH-3b, and the random-oracle model, closed at the research level and adopted; the piece-group key chosen by the encryptor and carried wrapped in every live set's sidecar, each sidecar its own swarm object; BLAKE3 keyed derivation off chain and keccak256 for what the contract recomputes; every identity-bound contract mutation a signed intent. BitTorrent through the `librqbit` soft fork as the sole MVP transport. A local keystore under the OS credential store with holder-seed derivation and versioned device roles. No maintainer commitment in escrow records. Every store root a repointable default under a settings catalogue. The [technical requirements](technical-requirements.md), [system architecture](system-architecture.md), and [tech stack](tech-stack.md) hold the detail.

# Implementation Context

The repository's agent process topics govern every implementation turn, with the Rust and Solidity element mapping the product requirements record and a lint proof for every file through the bound allowlist: `cargo check`, `cargo clippy`, `cargo fmt --check`, `forge build`, `forge fmt --check`, and the TypeScript linter for the shells and the webview. Nodes are authored from the milestones' tickets through the workplan's ordinary path, one source file per node with its full support system, in a module directory per function with one file per element. Commits land only at the end of a chain that can be integration-tested. The workplan's To-Do list holds found-but-unscheduled debt, currently the chain-migration mechanism and the deferred design questions.

# Test Framework

Every requirement is simultaneously an operational requirement and an integration or end-to-end test obligation; a unit test does not satisfy a requirement whose proof crosses a process, adapter, storage, network, custody, or chain boundary. Facilities: interface tests and guard tests per node; unit tests against the interaction spec; integration tests at each integration point in the milestone that first joins the two sides; Foundry fuzz, invariant, mutation, and replay tests for the contracts with constants and vectors generated from the Rust reference; `cargo-fuzz` at every adapter boundary; the demonstration harness for participants, wallets, chain state, fault injection, and the incompatible adapter declarations the incompatibility scenario rejects; a CI matrix over Windows, macOS, and Linux with clean ephemeral runners for end-to-end scenarios; a telemetry scan as a release gate. Completion evidence is only what passes through the packaged applications on clean machines; a mocked adapter, an off-chain-only verifier, or a manually prepared machine is development evidence.

# Component Mapping

Subsystem to crate to milestone, from the technical requirements' register and the milestones' scope lines.

| Crate | Subsystems | Milestone |
| --- | --- | --- |
| `domain` | Types and guards; the canonical encoding | Workspace and discipline bootstrap for the skeleton and the encoding; every milestone thereafter |
| `adapters/telemetry` | Redaction layer; metrics and traces | Workspace and discipline bootstrap for the redaction layer; daemon skeleton |
| `adapters/pairing`, `adapters/kdf` | Pairing adapters, the benchmark, KDF, hash-to-scalar, the sidecar wrap | Pairing adapters and key derivation |
| `adapters/kem` | Credential KEM | Credential KEM |
| `adapters/envelope` | Envelope | Envelope |
| `adapters/proof`, `contracts`, `apps/harness-crypto` (generate), `adapters/chain/verifier_client` | Delivery proof, the Solidity generator and verifier, verifier client | Delivery proof and Solidity verifier |
| `apps/harness-crypto` | Vectors, measurement, report | Harness vectors, measurement, and report |
| `adapters/hashing` | BLAKE3 roots, Bao verification and challenges | Hashing and commitments |
| `adapters/cipher`, `apps/wasm-demo` (sample_deployment) | Payload cipher, sidecar layer, validation, the sample deployment | Payload cipher and sidecar layer |
| `adapters/signature` | Signatures, DID types | Signature services and binding schema |
| `contracts` | Registry, parameter sets, envelope keys, entitlements, locks, escrow, claims, binding, verifier keys, adapter registry | Registry and entitlement contracts |
| `adapters/chain` | Client, settlement, entitlement-state, events | Chain, settlement, and entitlement-state adapters |
| `adapters/transport`, `adapters/discovery`, `adapters/seed-host` | Swarm engine, ciphertext store | Swarm transport and seed host |
| `adapters/jobs`, `adapters/config`, `workflows/settings`, `workflows/compose`, `workflows/health`, `adapters/ipc`, `apps/daemon` | Daemon skeleton | Daemon skeleton |
| `adapters/custody`, `workflows/identity` | Custody, identity manager, secret region | Identity and custody |
| `adapters/cas`, `workflows/resolve`, `adapters/package-host-npm` | CAS, orchestrator, package host | Plaintext CAS, resolution orchestrator, and package host |
| `adapters/ingest-npm`, `workflows/first_finder`, `workflows/grant` | First Finder engine, grant service | First Finder engine |
| `workflows/acquire`, `workflows/attempt`, `workflows/decrypt`, `workflows/interval_end`, `workflows/gate` | Acquisition, credential engine, attempt engine, decryption pipeline, interval-end watcher, deployment gate | Credential delivery and per-attempt authorization |
| `workflows/install`, `adapters/platform`, `apps/installer` | Installation coordinator, lifecycle, platform adapters | Installation coordinator and platform adapters |
| `apps/relayer` | Relayer | Relayer or paymaster |
| `apps/cli`, `apps/desktop` | Control surfaces | CLI and desktop application |
| `shells/vscode-extension`, `shells/npm-bootstrap` | Onboarding shells | Visual Studio Code extension and npm bootstrap package |
| `workflows/publish`, `adapters/identity-proof` | Publisher engine, issuance policy, proof adapters | Explicit publisher path and issuance policy |
| `apps/claim-verifier`, `workflows/claim` | Claim verifier, claim workflow | Claim verifier and escrow claim |
| `site`, `apps/wasm-demo` | Site, demonstration | Project seed host, site, and WebAssembly demonstration |
| `apps/harness-demo` | Demonstration harness | Demonstration harness |

# Architecture Summary

One canonical ciphertext per deployment and one sidecar per live parameter set, each its own object, on the swarm, entitlements on Base, credentials delivered inside settlement and verified by proof, per-attempt local authorization, and a daemon serving npm's protocol from whichever of cache, swarm, and registry delivers within budget, never slower than the registry alone by more than the hedge delay, leaving a first run independent, with nothing on the read path. The [system architecture](system-architecture.md) holds the context view, the daemon breakout, the credential-delivery sequence, and the device-role graph.

# Architecture

Layers with a decentralization boundary; implementation rings; capability-declared adapters resolved at installation and failing closed; an immutable deployment suite; the cryptographic construction stated once. In the system architecture.

# Services

The daemon, relayer, claim verifier, seed host and site, validation harness, and demonstration harness, each with what it must never become; the deferred account service, rendezvous, remote head, and hosted instance with their constraints recorded. In the system architecture.

# Components

The rings, the adapter table with MVP implementations and declared capabilities, the host shells, and the daemon's subsystems with trust boundaries, process model, and storage ownership. In the system architecture; crate and milestone assignment in Component Mapping above.

# Integration Points

The boundaries an integration test must cross, from verifier parity to the site demonstration, each assigned to the milestone that first joins its two sides. In the dependency map and the milestones' Integration Requirements.

# Dependency Resolution

Declaration, composition, suite binding, fail closed, and the on-chain factory under a project-held governance key recorded as scaffolding. Built in the daemon skeleton and the registry contracts; proven under IC-05, SI-10, and AS-17.

# Frontend Stack

Tauri 2 stable for the desktop; plain TypeScript with a small component library for the webview; TypeScript against the VS Code extension API; a `postinstall`-free npm bootstrap package; Rust with `clap` for the CLI; a static site with a `wasm-bindgen` build of the client crates for the demonstration. In the tech stack.

# Backend Stack

Rust throughout; `tokio`; one workspace with a domain crate and a crate per adapter family; authenticated local IPC over Unix domain sockets and named pipes with peer-credential or token authentication; `alloy` for the EVM; Foundry for Solidity; `serde` with canonical binary encoding; `tracing` with redaction; versioned configuration under the settings catalogue. In the tech stack and technical requirements.

# Data Platform

`redb` for the job store, settings, and indexes; filesystem CAS with atomic rename under a repointable root; root-keyed ciphertext store with holding reason under a repointable root; custody under the OS credential store through `keyring` with a wrapping key and encrypted blob; multi-node Base RPC views; archive access for envelope recovery; a local metrics store. In the tech stack and technical requirements.

# DevOps Tooling

`cargo-dist` packaging; `systemd`, `launchd`, and Windows Service or per-user scheduled task lifecycle; Apple and Authenticode signing plus Sigstore; a GitHub Actions matrix over Windows, macOS, and Linux with clean ephemeral runners; Anvil locally and Base Sepolia as the test chain; Foundry deployment scripts emitting addresses to configuration; the bound terminal allowlist with the TypeScript linter. In the tech stack.

# Security Tooling

`zeroize` and `subtle`; `cargo-audit` and `cargo-deny`; `cargo-fuzz` at every boundary; Foundry fuzz and invariant tests plus a static analyzer; the telemetry scan; redaction at the tracing layer; release signing keys under custody discipline. In the tech stack.

# Shared Libraries

`blake3` and `bao`; RustCrypto `aes` and `ctr`; `ed25519-dalek`; `k256` through `alloy`; arkworks `ark-bn254` and `ark-bls12-381` with `halo2curves` benchmarked; BLAKE3 keyed-mode KDF off chain and keccak256 for what the contract recomputes; `rand_core` with the OS RNG; DID Core types; `librqbit` as a soft fork; OpenZeppelin ERC-721 and ERC-4337 EntryPoint v0.7 on the contract side. Versions and verification dates in the tech stack.

# Third Party Services

More than one Base RPC provider plus a project node; archive access; an ERC-4337 bundler and paymaster provider behind a relayer adapter; WalletConnect and EIP-1193 providers with Frame evaluated; npm metadata, provenance attestations, and GitHub OAuth for claims; Apple, Microsoft, and Sigstore signing; external BitTorrent clients under delegation; public DHT and an optional project tracker; no sign-in providers in the MVP. In the tech stack.
