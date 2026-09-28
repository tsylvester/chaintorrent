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

The build runs through delivery groupings addressed by role. Every external touchpoint is an adapter family: a factory crate that owns the generic interface and its capability declaration, with each implementation a private concrete beneath it, so that a consumer names the factory and never an implementation and a further implementation is one concrete and nothing else. The foundation grouping establishes the workspace and its lint table, the encoding family with its ABI concrete, the secret type, the randomness family, and continuous integration. The cryptographic validation harness grouping builds the pairing, key-derivation, hash-to-scalar, credential KEM, envelope, and delivery proof families, each factory before its concretes, together with the shipped Solidity verifier and the harness's own generate and verifier families, measures them on Base Sepolia through the factories, fixes the piece-group size, confirms the curve, and calibrates throughput. The protocol core grouping builds the hashing, signature, transport, discovery, storage, and seed-host families beside the harness, with `librqbit` as the transport concrete, and the cipher family with the sidecar layer, the EVM contract suite as the chain family's on-chain concrete, and the chain and submission families after it. The daemon grouping builds the single machine-level process every requirement family meets, closes the read path with credential delivery and per-attempt authorization, completing a first run's independence, and reaches the demonstrable milestone where an ordinary `npm install` is served at the registry's speed, registers and prefetches, and a second identity resolves against the swarm with the registry down. The shells and services grouping builds installation on Windows, macOS, and Linux, the control surfaces, the relayer, the publisher path, the claim verifier, the site with its WebAssembly demonstration, observability, and the demonstration harness. The acceptance and release grouping runs the priced transaction flow proof on the Base mainnet pilot, closes external review and legal prerequisites, passes every acceptance scenario on clean machines, and releases with the dogfood baseline recorded.

Nothing is started. The workplan names its opening node when authored, from the foundation tickets. The stop criteria sit where they can first be evaluated, and the plan halts at any of them.

# Implementation Phases

The template's word is phases; here they are groupings addressed by dependency role. Each grouping below names its milestones in dependency order, its entry, its exit, and the stop criterion it carries, if any. Definitions are in the [milestones](milestones.md).

## Foundation grouping

**Milestones.** Workspace and discipline bootstrap.

**Entry.** The empty repository.

**Exit.** The workspace and every crate the foundation creates build and pass checks on Windows, macOS, and Linux; the domain crate depends on no host, chain SDK, wallet, transport, or storage engine; every family crate exposes its factory's surface and no concrete; no crate exists ahead of the module that first lives in it, and every later type is authored in the node that first consumes it.

**Yields.** The substrate every later grouping consumes, and signing enrollment started so certificates exist before onboarding.

## Cryptographic validation harness grouping

**Milestones.** Pairing adapters and key derivation; credential KEM; envelope; delivery proof and Solidity verifier; harness vectors, measurement, and report.

**Entry.** Foundation grouping closed. External cryptographic review starts on the composition claims, including the multi-set sidecar layer and the provenance boundary, at this grouping's start. The latency budget and its measurement conditions are declared before the report milestone selects anything.

**Exit.** CD-07 and AS-21: sizes, timings, and delivery cost as L2 execution and L1 data fee on both curves; the piece-group size selected and the curve confirmed against the declared budget and recorded in release evidence; cross-set agreement among the vectors; the verifier deployed on Base Sepolia as shipped code; throughput recorded.

**Stop criteria.** No parameter within the specification's bounds meets the latency budget; or the post-harness estimate of remaining work, produced by re-mapping the next grouping to tickets from measured throughput, is beyond what the project can fund.

## Protocol core and contract suite grouping

**Milestones.** Hashing and commitments, signature services, and swarm transport and seed host, at ticket resolution beside the harness with no chain dependency; payload cipher and sidecar layer; registry and entitlement contracts, the EVM suite; the chain and submission families.

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

**Entry.** Daemon grouping closed, except that the relayer milestone enters on the chain and submission families alone and may start earlier; this grouping re-mapped to tickets; signing certificates in hand under XA-08's custody discipline.

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
| Protocol core and contract suite | Hashing and commitments; signature services; swarm transport and seed host; payload cipher and sidecar layer; registry and entitlement contracts; chain and submission families | Unstarted; hashing, signatures, and swarm at ticket resolution, the rest as sprints |
| Local daemon and package serving | Daemon skeleton; identity and custody; CAS and package host; First Finder engine; credential delivery and per-attempt authorization; demonstrable milestone | Unstarted; mapped as epics |
| Onboarding shells and services | Installation coordinator; relayer; CLI and desktop; extension and npm bootstrap; explicit publisher path; claim verifier and escrow claim; seed host, site, and demonstration; observability reconciliation; demonstration harness | Unstarted; mapped as milestones |
| Acceptance and release | Transaction flow proof; review and legal closed; acceptance run; dogfood baseline and release | Unstarted; mapped as objectives |

Decisions: every blocking selection is resolved except the measured parameters, which the harness fixes. Compliance items on the release path are unstarted and run in parallel with the harness. The terminal proof surface is bound. No node exists.

# Status Markers

The plan uses the workplan's own vocabulary so that the master plan and the Work Breakdown Structure never disagree about state.

- `[ ]` unstarted: no node of the milestone has been authored or begun.
- `[~]` in progress: at least one node authored or under implementation; the milestone's chain has not committed.
- `[✅]` closed: the last node of the milestone's chain has committed with its integration test green, and the milestone's exit rows are proven.
- `[!]` halted: a stop criterion was met or a discovery revised the map; the milestone's scope is restated in full and the plan waits on the decision.

A milestone is closed only by its commit. A grouping is closed when every milestone in it is closed. The updated master plan, produced when a grouping closes, marks the next grouping's milestones as the next work and records the re-map.

# Dependency Rules

- **Producers before consumers, always.** A milestone starts only when every milestone its entry names has closed; within a milestone, nodes are dependency-ordered and a dependent node moves after its provider.
- **Dependencies point inward.** The domain crate depends on no host, chain SDK, wallet, transport, or storage engine; workflows depend on domain and on the adapter families' factory surfaces only; adapters and shells are outermost. A violation fails to compile.
- **Consumers name factories, never concretes.** A family crate exposes its factory's generic surface alone, its concretes are private modules beneath it, and a vendor library is named only by the concrete that wraps it; every family's factory ticket precedes its concretes, so the capability declarations exist before any resolver reads them.
- **Nothing depends on an undecided selection.** No node is authored against an open decision; the sponsorship mechanism for the identity contract account waits for the relayer milestone under LC-13; every node that encrypts a registered deployment waits on the harness report for the piece-group size.
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
| Entitlement ledger, contracts, and settlement | Registry and entitlement contracts; chain and submission families |
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

Rust throughout, Tauri for GUI surfaces, TypeScript only where Visual Studio Code and npm impose it, Solidity for the EVM suite. Three inward-pointing rings as crates. Every external touchpoint an adapter family whose factory crate owns the generic interface and capability declaration and whose concretes are private beneath it, the EVM contract suite included as the chain family's on-chain concrete; composition validated at each factory and at the resolver and failing closed; an immutable deployment suite as the unit of cryptographic composition. Base as the launch network with Base Sepolia for testing; BLS12-381 primary through EIP-2537 with BN254 retained; settlement tiers mapped to Base. A depth-one Boneh–Boyen KEM in Type-3 groups with seller-side rerandomization, ElGamal envelopes, and chained Schnorr delivery proofs, secure under SXDH, decisional BDH-3b, and the random-oracle model, closed at the research level and adopted; the piece-group key chosen by the encryptor and carried wrapped in every live set's sidecar, each sidecar its own swarm object; BLAKE3 keyed derivation off chain and keccak256 for what the contract recomputes; every identity-bound contract mutation a signed intent. BitTorrent through the `librqbit` soft fork as the sole MVP transport concrete, its DHT, PEX, and tracker sources concretes of the discovery family. A local keystore under the OS credential store with version-headed blobs upgraded in place as the custody family's first concrete; the holder seed on root devices only, every other device admitted under its own key as a signer on the identity's contract account, whose signer set every attempt reads, with versioned device roles. No maintainer commitment in escrow records. Every store root a repointable default under a settings catalogue. The [technical requirements](technical-requirements.md), [system architecture](system-architecture.md), and [tech stack](tech-stack.md) hold the detail.

# Implementation Context

The repository's agent process topics govern every implementation turn, with the Rust and Solidity element mapping the product requirements record and a lint proof for every file through the bound allowlist: `cargo check`, `cargo clippy`, `cargo fmt --check`, `forge build`, `forge fmt --check`, and the TypeScript linter for the shells and the webview. Nodes are authored from the milestones' tickets through the workplan's ordinary path, one source file per node with its full support system, in a module directory per function with one file per element; a family's factory is one node, a concrete adapter is one node whose operations are its methods, and a family's factory node precedes its concretes. Commits land only at the end of a chain that can be integration-tested. The workplan's To-Do list holds found-but-unscheduled debt, currently the chain-migration mechanism and the deferred design questions.

# Test Framework

Every requirement is simultaneously an operational requirement and an integration or end-to-end test obligation; a unit test does not satisfy a requirement whose proof crosses a process, adapter, storage, network, custody, or chain boundary. Facilities: interface tests and guard tests per node; unit tests against the interaction spec; integration tests at each integration point in the milestone that first joins the two sides; Foundry fuzz, invariant, mutation, and replay tests for the contracts with constants and vectors generated from the Rust reference; `cargo-fuzz` at every adapter boundary; the demonstration harness for participants, wallets, chain state, fault injection, and the incompatible adapter declarations the incompatibility scenario rejects; a CI matrix over Windows, macOS, and Linux with clean ephemeral runners for end-to-end scenarios; a telemetry scan as a release gate. Completion evidence is only what passes through the packaged applications on clean machines; a mocked adapter, an off-chain-only verifier, or a manually prepared machine is development evidence.

# Component Mapping

Subsystem to crate or module to milestone, from the technical requirements' register and the milestones' scope lines. An entry is a crate path, or a module of a crate named as `crate/module`, so `workflows/settings` is the `settings` module of the `workflows` crate. An adapter family's crate is its factory, and the concretes named beside it are private modules beneath that factory.

| Crate or module | Subsystems | Milestone |
| --- | --- | --- |
| `domain` | The types its own modules implement, with their guards; the secret type | Workspace and discipline bootstrap for the secret type, which creates the crate; every milestone thereafter |
| `adapters/encoding`, with `encoding/abi` | The encoding factory owning `IEncoderAdapter` and `IDecoderAdapter` under one versioned encoding identifier, the encoding contract, and the declaration; the ABI concrete | Workspace and discipline bootstrap |
| `adapters/random`, with `random/os` | The randomness factory with its deterministic mock; the operating-system concrete | Workspace and discipline bootstrap |
| `adapters/telemetry`, with `telemetry/tracing`, `telemetry/local_metrics`, and `telemetry/opentelemetry` | The exporter factory; tracing with per-request correlation; the local metrics store and the OpenTelemetry exporter | Daemon skeleton for the factory, tracing, and the local metrics store; observability reconciliation for OpenTelemetry |
| `adapters/pairing`, with `pairing/bn254_arkworks`, `pairing/bn254_halo2curves`, `pairing/bls12_381_arkworks`, and `pairing/bls12_381_halo2curves`; `adapters/kdf`, with `kdf/blake3_keyed`; `adapters/hash-to-scalar`, with `hash-to-scalar/keccak256`; `apps/harness-crypto` (benchmark) | The pairing factory and one concrete per curve per library; the KDF factory and its concrete; the hash-to-scalar factory and its concrete; the benchmark over every pairing concrete through the factory | Pairing adapters and key derivation |
| `adapters/kem`, with `kem/bb1_depth_one`; `workflows/sidecar` (wrap, unwrap) | The credential KEM factory and its concrete; the wrap and unwrap of the piece-group key | Credential KEM |
| `adapters/envelope`, with `envelope/pairing_elgamal` | The envelope factory and its concrete | Envelope |
| `adapters/proof`, with `proof/schnorr_fs`; `contracts/evm` (PairingLib, DeliveryVerifier, deploy); `apps/harness-crypto` (generate with `generate/evm`, verifier with `verifier/evm`) | The delivery proof factory and its concrete with its challenge; the Solidity pairing library and verifier; the harness's generate and verifier families | Delivery proof and Solidity verifier |
| `apps/harness-crypto` (vectors, measure, report) | Vectors, measurement, report | Harness vectors, measurement, and report |
| `adapters/hashing`, with `hashing/blake3_bao` | The hashing factory owning the commitment interface; the BLAKE3/Bao concrete | Hashing and commitments |
| `adapters/cipher`, with `cipher/aes_ctr`; `workflows/sidecar` (build, validate); `apps/wasm-demo` (sample_deployment) | The payload cipher factory and its concrete; sidecar construction and validation ordering; the sample deployment | Payload cipher and sidecar layer |
| `adapters/signature`, with `signature/ed25519` and `signature/secp256k1` | The signature factory resolving per layer; both scheme concretes | Signature services |
| `contracts/evm` | The EVM suite: registry, parameter sets, envelope keys, `IEntitlement` with its ERC-721 concrete, locks, escrow, claims, binding, the identity account, verifier keys, `IAttestationVerifier` per source, `IIdentityAdapter` with its adapters, the adapter registry and factory | Registry and entitlement contracts |
| `adapters/chain`, with `chain/quorum_view` and `chain/base`; `adapters/submission`, with `submission/self_funded`, `submission/relayer`, and `submission/paymaster` | The chain factory owning the chain interface, `ISettlementAdapter`, and `IEntitlementStateAdapter`; the quorum view; the Base concrete with client, tier mapping, entitlement-state views, and events; the submission factory and its call-path concretes | Chain and submission families for the factories, the Base concrete, and self-funded submission; relayer or paymaster for the sponsored concretes |
| `adapters/transport`, with `transport/rqbit`; `adapters/discovery`, with `discovery/aggregate` and the `dht`, `pex`, `tracker`, `local`, and `seeder_map` concretes; `adapters/storage`, with `storage/redb`; `adapters/seed-host`, with `seed-host/owned` and `seed-host/rqbit` | Swarm engine, discovery aggregation, the storage engine, the ciphertext store | Swarm transport and seed host |
| `workflows/jobs`, `workflows/config`, `workflows/settings`, `workflows/compose`, `workflows/health`, `adapters/platform-paths` with its `linux`, `macos`, and `windows` concretes, `adapters/ipc` with `ipc/framing`, `ipc/principals`, `ipc/server`, `ipc/unix_socket`, and `ipc/named_pipe`, `adapters/lifecycle` for its factory and single-instance lock, `apps/daemon` | Daemon skeleton | Daemon skeleton |
| `adapters/custody`, with `custody/holder_seed`, `custody/device_key`, `custody/upgrade`, and `custody/local_keystore`; `adapters/wallet`, with `wallet/eip1193` and `wallet/walletconnect`; `adapters/identity`, with `identity/publisher_authority` and `identity/escrow`; `workflows/identity` | Custody with its family-owned derivation, device keys, and upgrade; the wallet family; the identity family with the binding schema; the identity manager; the secret region | Identity and custody |
| `adapters/cas`, with `cas/filesystem`; `workflows/resolve`; `adapters/package-host`, with `package-host/npm` | The CAS factory and its concrete; the orchestrator; the package host factory and its npm concrete with the metadata store | Plaintext CAS, resolution orchestrator, and package host |
| `adapters/ingest`, with `ingest/npm`; `workflows/first_finder`; `workflows/grant` | The ingest factory and its npm concrete; the First Finder engine; the grant service | First Finder engine |
| `workflows/request`, `workflows/prefetch`, `workflows/acquire`, `workflows/attempt`, `workflows/decrypt`, `workflows/interval_end`, `workflows/gate` | Request and prefetch jobs, acquisition, credential engine, attempt engine, decryption pipeline, interval-end watcher, deployment gate | Credential delivery and per-attempt authorization |
| `workflows/install`; `adapters/lifecycle` with `lifecycle/systemd`, `lifecycle/launchd`, `lifecycle/windows_service`, and `lifecycle/scheduled_task`; `adapters/artifact-verifier` with `artifact-verifier/sigstore`, `artifact-verifier/authenticode`, and `artifact-verifier/apple_notarization`; `adapters/redirect` with `redirect/npm`; `apps/installer` | Installation coordinator, service lifecycle concretes, artifact verification, package-manager redirect | Installation coordinator and platform adapters |
| `apps/relayer`, with its `bundler` factory and one concrete per provider | Relayer | Relayer or paymaster |
| `apps/cli`, `apps/desktop` | Control surfaces | CLI and desktop application |
| `shells/vscode-extension`, `shells/npm-bootstrap` | Onboarding shells | Visual Studio Code extension and npm bootstrap package |
| `workflows/publish`; `adapters/publisher-proof`, with `publisher-proof/provenance` and `publisher-proof/maintainer_oauth` | Publisher engine, issuance policy, the publisher-proof family | Explicit publisher path and issuance policy |
| `apps/claim-verifier`; `adapters/claim-verifier`, with `claim-verifier/attestor`; `workflows/claim` | Claim verifier service, the claim-verifier client family, claim workflow | Claim verifier and escrow claim |
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

Declaration, composition, suite binding, fail closed, and the on-chain factory under a project-held governance key recorded as scaffolding. Each family's factory admits a concrete by its declaration against every downstream requirement, and `workflows/compose` works back through the chain of factories. Built in each factory, in the daemon skeleton, and in the registry contracts; proven in the harness for the cryptographic families and under IC-05, SI-10, and AS-17 for the whole composition.

# Frontend Stack

Tauri 2 stable for the desktop; plain TypeScript with a small component library for the webview; TypeScript against the VS Code extension API; a `postinstall`-free npm bootstrap package; Rust with `clap` for the CLI; a static site with a `wasm-bindgen` build of the client crates for the demonstration. In the tech stack.

# Backend Stack

Rust throughout; `tokio`; one workspace with a domain crate, a workflows crate, and a crate per adapter family, each the family's factory with private concretes beneath it; authenticated local IPC through the `ipc` family's Unix domain socket and named pipe concretes with peer-credential or token authentication; `alloy` inside the chain, encoding, and secp256k1 concretes only; Foundry for the EVM suite; the `encoding` family with its ABI concrete under one versioned encoding identifier; `tracing` with per-request correlation in the `telemetry` factory; versioned configuration under the settings catalogue over the storage family. In the tech stack and technical requirements.

# Data Platform

The `storage` family with `storage/redb` for the job store, settings, and every index; the `cas` family with `cas/filesystem` and atomic rename under a repointable root; the root-keyed ciphertext store with holding reason under a repointable root inside `seed-host/owned`; custody under the OS credential store through `custody/local_keystore` over `keyring` with a wrapping key and encrypted blob; multi-node Base RPC views through the chain family's quorum view; archive access for envelope recovery through `chain/base`; the local metrics store as `telemetry/local_metrics`. In the tech stack and technical requirements.

# DevOps Tooling

`cargo-dist` packaging; the `lifecycle` family's concretes for systemd, launchd, a Windows Service, and a per-user scheduled task; Apple and Authenticode signing plus Sigstore, verified on the installing machine through the `artifact-verifier` family; a GitHub Actions matrix over Windows, macOS, and Linux with clean ephemeral runners; Anvil locally and Base Sepolia as the test chain; Foundry deployment scripts under `contracts/evm` emitting addresses to configuration; the bound terminal allowlist with the TypeScript linter. In the tech stack.

# Security Tooling

`zeroize` and `subtle`; `cargo-audit` and `cargo-deny`; `cargo-fuzz` at every boundary; Foundry fuzz and invariant tests plus a static analyzer; the telemetry scan; a secret type that cannot be formatted or serialized and zeroizes on drop; the workspace lint table; release signing keys under custody discipline. In the tech stack.

# Shared Libraries

`blake3` and `bao` inside `hashing/blake3_bao`; RustCrypto `aes` and `ctr` inside `cipher/aes_ctr`; `ed25519-dalek` inside `signature/ed25519`; `k256` through `alloy` inside `signature/secp256k1`; arkworks `ark-bn254` and `ark-bls12-381` inside `pairing/bn254_arkworks` and `pairing/bls12_381_arkworks`, and `halo2curves` inside `pairing/bn254_halo2curves` and `pairing/bls12_381_halo2curves`, each measured by the harness benchmark through the pairing factory; BLAKE3 keyed-mode KDF inside `kdf/blake3_keyed` and keccak256 inside `hash-to-scalar/keccak256`; `rand_core` with the OS RNG inside `random/os`; DID Core types in the identity factory; `librqbit` as a soft fork inside `transport/rqbit`; OpenZeppelin ERC-721 inside the EVM suite's entitlement concrete and ERC-4337 EntryPoint v0.7 on the contract side. Each library appears only inside the concrete that wraps it. Versions and verification dates in the tech stack.

# Third Party Services

More than one Base RPC provider plus a project node; archive access; an ERC-4337 bundler and paymaster provider behind the relayer's `bundler` factory; WalletConnect and EIP-1193 providers behind the `wallet` family, with Frame evaluated; npm metadata, provenance attestations, and GitHub OAuth behind the `publisher-proof` family; Apple, Microsoft, and Sigstore signing behind the `artifact-verifier` family; the rqbit application behind `seed-host/rqbit`, and any further external client behind its own seed-host concrete; public DHT and an optional project tracker behind the `discovery` family; no sign-in providers in the MVP. In the tech stack.
