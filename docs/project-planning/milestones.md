<!-- Template: parenthesis_milestone_schema.md -->
# Index

Draft, 2026-09-23. The milestones for the ChainTorrent MVP: the middle view between the plan and the workplan's nodes, spanning the empty repository to the delivered MVP. Milestones are addressed by their dependency role and never by ordinal; each names what precedes it and what it unblocks, so insertion and reordering leave every other milestone's text valid.

- Executive Summary
- Pipeline Context
- Selection Criteria
- Shared Infrastructure
- Milestones, grouped by delivery role: foundation; cryptographic validation harness; protocol core and contract suite; local daemon and package serving; onboarding shells and services; acceptance and release
- Iteration Semantics
- Features Context
- Feasibility Insights
- Non-Functional Alignment
- Architecture Summary, Services, Components, Dependency Resolution, Component Details, Integration Requirements
- Migration Context

Sources: [product requirements](product-requirements.md), [technical requirements](technical-requirements.md), [system architecture](system-architecture.md), [dependency map](dependency-map.md), [tech stack](tech-stack.md), [feature spec](feature-spec.md), [success metrics](success-metrics.md), [risk register](risk-register.md), and the specifications under [docs/research](../research/) and the [workplan](../workplans/current/ChainTorrent%20MVP.md).

# Executive Summary

The milestones, in groupings by delivery role, carry the work from an empty repository to a released MVP. The foundation grouping establishes the workspace, the canonical encoding, the redaction layer, the build on Windows, macOS, and Linux, and the terminal proof surface. The cryptographic validation harness grouping builds the pairing adapters, the credential KEM, the envelope, the delivery proof, and the Solidity verifier, measures them on Base Sepolia, fixes the piece-group size, confirms the curve, and yields the project's throughput calibration. The protocol core grouping builds hashing, signatures, and the swarm transport over the `librqbit` soft fork at ticket resolution beside the harness, without any chain, and the cipher and sidecar layer, the contract suite, and the chain adapters after the harness report. The daemon grouping builds the process every requirement family meets, closes the read path with credential delivery and per-attempt authorization, and then reaches the demonstrable milestone at which an ordinary `npm install` is served at the registry's speed, registers and prefetches, a second identity resolves against the swarm with the registry down, and the north star, first-run independence, and the latency guardrail are observed. The shells and services grouping builds installation on Windows, macOS, and Linux, the control surfaces, the relayer, the publisher path, the claim verifier, the site with its WebAssembly demonstration, observability reconciliation, and the demonstration harness. The acceptance and release grouping runs the transaction flow proof on the Base mainnet pilot, closes external review and legal prerequisites, runs every acceptance scenario on clean machines, records the dogfood baseline, and releases.

Each milestone states its purpose, scope by subsystem and ticket, entry conditions, exit proof by requirement and scenario identifier, deliverables, owner role, and what it unblocks. Resolution follows the dependency map's decay: the foundation and harness groupings and the hashing, signature, and swarm milestones are at ticket resolution now; the remaining milestones are re-mapped to tickets as the harness closes and outward from there. The stop criteria from the product requirements sit at the milestones where they can first be evaluated.

# Pipeline Context

This document sits between the product and technical requirements above it and the workplan's nodes below it. The [technical requirements](technical-requirements.md) fix what the system must contain; the [dependency map](dependency-map.md) fixes what depends on what and holds the harness at ticket resolution; this document segments that into milestones with entry and exit conditions; the workplan's Work Breakdown Structure then holds one node per source file, authored through the repository's ordinary path from the tickets a milestone names. The master plan, which the pipeline places before this document, summarizes these milestones into the implementation view and tracks their status; it can be produced from this document without loss. No node exists yet; the workplan names its opening node when authored, from the foundation tickets.

Two repository rules govern everything below. Nodes and groupings are addressed relationally, never numbered. Each node is one source file with its full support system, authored test-first in the fixed element order, one file per turn, with a commit only at the end of a chain that can be integration-tested.

# Selection Criteria

A milestone is drawn where all of the following hold.

- **It ends at a provable boundary.** Its exit is a set of requirement rows and, where one exists, an acceptance scenario, proven through the boundary the requirement names, never by a unit test where the proof crosses a process, adapter, storage, network, custody, or chain boundary.
- **Its dependencies are complete before it starts.** Entry conditions name the milestones that must have closed; nothing inside a milestone waits on something outside it.
- **It is the smallest span that yields something the next milestone consumes.** A milestone that produces nothing consumable is merged into its consumer; one that produces two independent things is split.
- **It carries a commit.** The last node in a milestone's chain holds the integration test and the commit, per the workplan-structure rule that neither is ever a node of its own.
- **Where a stop criterion can first be evaluated, the milestone names it.** The harness report, the post-harness re-map, the demonstrable milestone, the acceptance run, and review closure each carry one.
- **It has one owner by role.** Roles are those the product requirements and success metrics use.

# Shared Infrastructure

Built once in the foundation grouping and consumed by every later milestone.

| Infrastructure | Provides | Built in |
| --- | --- | --- |
| Cargo workspace with rings as crates | A domain crate, a workflows crate, one crate per adapter family, apps; a ring violation is a compile error | Workspace and discipline bootstrap |
| Continuous integration on clean ephemeral runners for Windows, macOS, and Linux | `cargo check`, `cargo clippy`, `cargo fmt --check`, `forge build`, `forge fmt --check`, the TypeScript linter, `cargo-audit`, `cargo-deny` on every push; end-to-end scenarios on clean runners | Workspace and discipline bootstrap |
| Terminal proof surface | The allowlist bound so an agent can lint after every edit as the linting-proof topic requires | Done, 2026-09-23 |
| Canonical binary encoding | One encoder for everything hashed, signed, or stored, shared by Rust and mirrored in Solidity | Workspace and discipline bootstrap |
| Tracing with redaction | Secret-typed values unformattable; per-request correlation | Workspace and discipline bootstrap |
| Local chain and test chain | Anvil in the developer loop; Base Sepolia for deployment; Foundry scripts emitting addresses into configuration | Solidity verifier |
| Generated verifier constants and vectors | Solidity constants and test vectors produced from the Rust reference | Delivery proof and Solidity verifier |
| Code signing | Apple, Authenticode, and Sigstore identities enrolled during the harness grouping so they exist before the onboarding grouping | Workspace and discipline bootstrap, enrollment started; installation coordinator, first use |
| Demonstration harness | Controlled participants, wallets, chain state, and fault injection; load-bearing under the completion boundary | Its own milestone in the shells and services grouping |
| Metrics store and release-evidence format | The RO-03 record and the harness output document | Daemon skeleton; harness report |

# Milestones

Within each grouping, milestones are listed in dependency order; where two are independent, the text says so.

## Foundation grouping

### Workspace and discipline bootstrap

**Purpose.** Turn the empty repository into a workspace where a node can be authored and proven on Windows, macOS, and Linux.

**Scope.** The tickets `workspace/cargo`, `domain/encoding`, `telemetry/redaction`, and `workspace/ci`: the workspace manifest and crate skeletons for `domain`, `workflows`, every `adapters/*` family, and every `apps/*` entry from the technical requirements' file tree, the `domain` crate as an empty skeleton beyond its encoding module; `rust-toolchain.toml`, `deny.toml` with the license allowlist, `foundry.toml`; the element mapping applied as one module directory per function with one file per element; the canonical encoding module; `tracing` with the redaction layer; the GitHub Actions matrix over Windows, macOS, and Linux running the allowlisted checks and audits; Apple, Microsoft, and Sigstore signing enrollment started. The manifests and the CI definition are configuration files with no types and no tests.

**Entry.** None; this is the first milestone.

**Exit.** The empty workspace builds and passes checks on Windows, macOS, and Linux; a node authored in the harness grouping's form compiles against the skeleton; `cargo-deny` enforces the allowlist; the redaction layer refuses to format a secret-typed value in a test. Requirement rows: XA-07 for the facilities, NF-M07 as the authoring discipline.

**Deliverables.** The workspace; the CI definition; the encoding and tracing crates; the signing enrollment requests filed.

**Owner.** Project lead with the platform implementer.

**Unblocks.** Everything.

## Cryptographic validation harness grouping

Mapped at ticket resolution in the dependency map. The independent starting points are the BN254 pairing adapter and the Solidity pairing library; every other ticket consumes one of them. External cryptographic review starts on the composition claims when this grouping starts.

### Pairing adapters and key derivation

**Purpose.** The group arithmetic and derivation every cryptographic ticket calls, on both curves, with encodings that match the precompiles.

**Scope.** `pairing/bn254`, which owns the pairing interface and types, `pairing/bls12_381`, `pairing/benchmark`, which selects the library, and `kdf/hash_to_scalar`, which owns BLAKE3 keyed derivation for every off-chain use, keccak256 hash-to-scalar for the identity mapping and the challenge, and the piece-group-key wrap and unwrap; capability declaration for second-group arithmetic at the verifier.

**Entry.** Workspace and discipline bootstrap.

**Exit.** CR-10 vectors against precompile behavior including subgroup rejection and encoding edge cases on both curves; CR-11's derivation, hash-to-scalar, and wrap produce the specification's outputs with context strings, serialization, and output lengths frozen, matching a Solidity mirror and known-answer vectors from an independent implementation; the benchmark recorded and the library chosen.

**Deliverables.** The pairing and KDF crates; the benchmark record.

**Owner.** Cryptography implementer.

**Unblocks.** Credential KEM, envelope, delivery proof.

### Credential KEM

**Purpose.** The construction that makes decryption capability native and distinct per interval.

**Scope.** `kem/setup`, which owns the KEM interface with the declared identity scope and the parameter-set types, `kem/issue`, `kem/rerandomize`, `kem/validity`, `kem/encapsulate`, `kem/well_formed`, `kem/decapsulate`.

**Entry.** Pairing adapters and key derivation.

**Exit.** CR-08 algebraic vectors: cross-holder agreement on the encapsulated value, cross-set agreement with two independently generated parameter sets unwrapping one piece-group key, rerandomized-credential validity, cross-entitlement non-convertibility under the entitlement scope, holder-authored grants under the asset scope, malformed-capsule rejection, trivial identity-element refusal; CD-08's scope declaration refused where it must be.

**Deliverables.** The KEM crate.

**Owner.** Cryptography implementer.

**Unblocks.** Envelope unwrap, delivery proof.

### Envelope

**Purpose.** Delivery of a credential to a recipient's registered keys and nothing else's.

**Scope.** `envelope/keygen`, which owns the key-agreement interface declaring the envelope algebra and the envelope types, with proofs of possession and identity-element rejection, `envelope/wrap`, `envelope/unwrap` with the local validity check.

**Entry.** Credential KEM, for the validity check.

**Exit.** CR-04: independent coins, shared or identity secrets rejected, exact round trip, swapped keys or elements yield decryption failure or an invalid credential; LC-08's registration hygiene at the algebra level.

**Deliverables.** The envelope crate.

**Owner.** Cryptography implementer.

**Unblocks.** Delivery proof.

### Delivery proof and Solidity verifier

**Purpose.** The proof the contract verifies before it records an interval, in Rust as the reference and in Solidity as the shipped contract, bit for bit.

**Scope.** `proof/challenge`, which owns the delivery-proof interface and the statement types, over the full context schema under keccak256, `proof/prove_mint`, `proof/prove_transfer`, `proof/verify` in both forms; `contracts/PairingLib` over EIP-196, EIP-197, and EIP-2537 with contract-side subgroup checks, `harness-crypto/generate` emitting the Solidity constants and vectors from the Rust reference, `contracts/DeliveryVerifier`, `contracts/deploy` to Anvil and Base Sepolia; `chain/verifier_client`.

**Entry.** Credential KEM and envelope; the Solidity pairing library may start at the workspace bootstrap and joins here.

**Exit.** CR-09 in both verifier forms with every statement field and response mutated and replay across settlements rejected; CD-03 on each curve form with record-contradicting statements rejected; the Rust verifier and the deployed contract agree on every vector; the verifier deployed on Base Sepolia with its address recorded.

**Deliverables.** The proof crate, the Solidity pairing library and verifier, the generator, the chain verifier client, the Base Sepolia deployment record.

**Owner.** Cryptography implementer with the contract implementer.

**Unblocks.** Harness measurement; registry and entitlement contracts.

### Harness vectors, measurement, and report

**Purpose.** Measure what nothing has measured, fix the parameters that gate every node that encrypts, and calibrate throughput.

**Scope.** `harness-crypto/vectors`, `harness-crypto/measure`, `harness-crypto/report`; the report node carries the grouping's integration test across the whole chain and its commit.

**Entry.** Delivery proof and Solidity verifier.

**Exit.** CD-07 and AS-21: capsule, envelope, and proof sizes; decapsulation time per piece group; proof generation and verification time; delivery cost as L2 execution gas and L1 data fee for a mint and a transfer, on both curves, the L1 fee noted as Sepolia's; the piece-group size chosen and the curve confirmed against the budget declared before this milestone and recorded in release evidence; tickets closed per unit of time recorded across the grouping.

**Stop criterion.** If no parameter within the specification's bounds meets the latency budget declared before this milestone, the finding is reported against the design and the plan halts here.

**Deliverables.** The release-evidence document; the throughput record; the first evidence-based estimate of the remaining work, produced by re-mapping the remaining protocol core milestones to tickets.

**Owner.** Cryptography implementer; project lead for the budget declaration, the re-map, and the funding decision.

**Unblocks.** Every node that encrypts a registered deployment; the funding stop criterion.

## Protocol core and contract suite grouping

The hashing, signature, and swarm milestones are at ticket resolution and run beside the harness, since they depend on nothing it measures; the remaining milestones are re-mapped to tickets when the harness report closes.

### Hashing and commitments

**Purpose.** The integrity layer the specification separates from confidentiality deliberately.

**Scope.** `hashing/blake3_root`, `hashing/bao_verify`, `hashing/bao_challenge`: BLAKE3 roots, Bao outboard construction, streaming and random-access verification, random custody challenges; the challenge node carries the milestone's integration test and commit.

**Entry.** Workspace and discipline bootstrap. Independent of the harness.

**Exit.** CR-02 corruption of roots, paths, chunks, lengths, and order detected across construction, verification, and challenge.

**Deliverables.** The hashing crate.

**Owner.** Cryptography implementer.

**Unblocks.** Swarm transport and seed host; payload cipher and sidecar layer; the plaintext CAS.

### Payload cipher and sidecar layer

**Purpose.** The symmetric layer, and the sidecar that lets every live parameter set open one ciphertext.

**Scope.** `AesCtrAdapter` with the 64/64 counter layout, continuous-stream addressing, extent and index bounds; the sidecar layer, per live parameter set a capsule and wrapped piece-group key per group committed by that set's Bao root, each sidecar its own object; manifest, hash-card, and sidecar validation in the ordering the specification requires; the decapsulation-to-cipher seam; the `sample_deployment` generator for the site demonstration, which needs the cipher and so lives here rather than in the harness.

**Entry.** Hashing and commitments; `kdf/hash_to_scalar` for the wrap; the harness report, for the piece-group size the `sample_deployment` generator fixes.

**Exit.** CR-01 vectors including maximum-valid counters and overflow rejection; CR-06 fuzzing with chain and custody spies proving validation precedes any credential exercise; a two-set sidecar opening one ciphertext under a credential of either set; the sample deployment bundled for the WebAssembly build.

**Deliverables.** The cipher crate; the validation workflow; the sample deployment.

**Owner.** Cryptography implementer.

**Unblocks.** Decryption pipeline, First Finder engine, the site demonstration.

### Signature services and binding schema

**Purpose.** Per-layer signatures and the identity binding the chain reads two words of.

**Scope.** `signature/ed25519`, `signature/secp256k1`, `signature/binding_schema`: `Ed25519Adapter`, `Secp256k1Adapter` through `alloy`, domain separation and complete-message binding; DID Document types with the anchor hash; EIP-712 typed data for registrations, bindings, and signed intents; the binding-schema node carries the milestone's integration test and commit.

**Entry.** Workspace and discipline bootstrap. Independent of the harness and the cipher milestone.

**Exit.** CR-03: valid signatures verified; cross-domain, cross-layer, truncated-context, wrong-scheme, and replay substitutions rejected.

**Deliverables.** The signature crate and DID types.

**Owner.** Cryptography implementer.

**Unblocks.** Identity and custody; contract binding surface.

### Registry and entitlement contracts

**Purpose.** The ledger the protocol's decentralized properties rest on.

**Scope.** The contract suite from the technical requirements' API surface: registry with asset records and the per-name version index, deployments carrying every hash-card field, the sidecar-per-live-set rule, sidecar addition for later sets, on-chain attestation verification through the P-256 precompile against the source-key table or a recorded absence, and later escrow deployments of an existing asset; parameter-set liveness and retirement; envelope-key registry with proofs of possession; ERC-721 entitlements with interval state and envelope digests and the ownership override that disables the standard transfer and approval entry points; batched grant requests readable by holders and closed by grant or withdrawal; locks with expiry and refund; mint, deliver, grant calling the harness's verifier; the identity contract account under ERC-1271 with its device registry, admission, re-scoping, revocation, and the `isDeviceAdmitted` view; every identity-bound mutation taking the acting identity and a signed intent verified through the account against a signer whose permissions cover it, with lock expiry enforced and no volume limit; `evaluateAuthorization` and its paginated batch; escrow records per deployment with provenance, attestation or its absence, and no maintainer commitment; identity binding; claim settlement state layout and verifier-key registry; the adapter registry and factory with constructor-injected, immutable bindings under the project-held governance key. Deployment scripts to Anvil and Base Sepolia.

**Entry.** Delivery proof and Solidity verifier.

**Exit.** Every transition of LC-01 exercised through Foundry tests with asserted events and views, the Rust-adapter form of that proof belonging to the chain adapters milestone; LC-02, LC-05, LC-07, LC-08, LC-09 including sidecar addition, LC-10, LC-11 with owner, approved address, and operator each refused an ordinary transfer, LC-12 with batched requests opened, fulfilled, withdrawn, and shown to authorize nothing, LC-13 through the identity contract account as paymaster-sponsored, relayer-submitted, and self-funded calls, with foreign, expired, replayed, revoked-signer, and out-of-permission intents refused; AS-13's proof-set acceptance and rejection on Base Sepolia.

**Deliverables.** The contract suite deployed on Base Sepolia; addresses in configuration; Foundry fuzz and invariant tests; static analysis in CI.

**Owner.** Contract implementer.

**Unblocks.** Chain adapters; First Finder engine; credential delivery; claim.

### Chain, settlement, and entitlement-state adapters

**Purpose.** The daemon's only view of consensus, from a quorum of configured nodes.

**Scope.** `chain/client` bindings through `alloy`; `chain/settlement` mapping Base's tiers to INCLUDED, SOFT, HARD, and SETTLED with reference age; `chain/entitlement_state` single and paginated views aggregated as a quorum, two of three configured nodes agreeing at a common reference within `τ_soft`, stale nodes ignored, divergence and over-age views failing closed; `chain/events` for envelope recovery from calldata and events; intent signing and the relayer submission path.

**Entry.** Registry and entitlement contracts.

**Exit.** LC-01 every transition exercised through the Rust chain adapter with asserted events and views; LC-04 included, pending, reorganized, stale, and insufficient-tier references driving the attempt rule for a zero-price and a priced deployment; LC-06 batch parity with individual evaluation; XA-06's decision table, one timeout, one stale provider, honest heads at different heights, divergent state at one reference, a reorganization, and a prolonged stall, each with its single defined result; CD-04 recovery from chain history.

**Deliverables.** The chain adapter crate.

**Owner.** Chain adapter implementer.

**Unblocks.** Credential delivery; identity; relayer; every state read.

### Swarm transport and seed host

**Purpose.** Bytes moving between machines with nothing readable in transit, provable with no chain and no KEM.

**Scope.** `transport/rqbit_overlay`, the `[patch.crates-io]` overlay onto tagged `librqbit` with the adapter's hooks and the upstream issues opened; `transport/bittorrent_rqbit` with root-to-infohash translation, torrent creation per object with the ciphertext and each sidecar as its own object and locator, Bao verification of every completed piece after the library's SHA-1 check, seeder-map peer injection; `discovery/aggregate`, `discovery/local`, and `discovery/seeder_map` over the library's DHT, PEX, and trackers plus local and the on-chain seeder map; `seed-host/store`, the root-keyed ciphertext store with holding reason, quota, and eviction; `seed-host/owned` and `seed-host/delegated` with Bao custody challenges; settings passthrough for the controls the library has and enforcement of the ones it does not; the delegated node carries the milestone's integration test and commit.

**Entry.** Hashing and commitments. Independent of the harness and of every contract milestone; `discovery/seeder_map` joins when the chain adapters exist.

**Exit.** SW-01 through SW-07 and EC-02 as amended: retrieval through the owned host and a delegated external client with identical roots; a corrupted piece passing SHA-1 rejected by Bao before storage; discovery aggregation deterministic under conflicting and malicious results; seeding continuity across host restart while another participant retrieves; delegated custody challenges detecting false reports; resumable failure handling; the archive visible.

**Deliverables.** The transport, discovery, and seed-host crates; the overlay branch; the upstream issues.

**Owner.** Daemon implementer.

**Unblocks.** First Finder engine; package serving from the swarm; the project seed host.

## Local daemon and package serving grouping

Re-mapped to tickets when the protocol core grouping closes. The identity milestone runs early because the installation coordinator needs it.

### Daemon skeleton: jobs, configuration, settings, resolver, health, telemetry, IPC

**Purpose.** The process that owns the stores and the two ingress surfaces, with every cross-cutting mechanism present before any workflow lands.

**Scope.** `adapters/jobs` over `redb` with checkpoints, idempotency keys, restart, and cancellation; `adapters/config` and `workflows/settings` with the catalogue including its presence-required marks and the hedge delay, source deadline, and grant wait, validation, migration jobs, backup and restore, export and import; `workflows/compose` the composition resolver; `workflows/health`; `adapters/telemetry` with the RO-03 record and the trace exporter; `adapters/ipc` with the decided framing, per-platform authentication, the control-principal rule, and the local-presence confirmation; single-instance lock; `apps/daemon` binary skeleton.

**Entry.** Workspace and discipline bootstrap; signature services for the settings that reference custody. Independent of the swarm and contract milestones.

**Exit.** XA-03 termination at every persisted transition with exactly-once effects; IC-04, IC-05 with no side effects from invalid compositions, IC-08; ST-01 through ST-04 and ST-08 through the IPC; XA-02 other local users gain nothing, a lifecycle script running as the installing user obtains resolution only, and every local-presence operation and presence-required setting waits on the daemon-owned interactive confirmation; XA-10 backoff within declared ceilings; RO-04 correlation across the skeleton.

**Deliverables.** A daemon that starts, resolves a composition, holds settings, reports health, and refuses what it must.

**Owner.** Daemon implementer.

**Unblocks.** Every daemon workflow; the installation coordinator.

### Identity and custody

**Purpose.** The principal, its keys, and where they live.

**Scope.** `adapters/custody/interface`; `adapters/custody/local_keystore` over `keyring` with a wrapping key and version-headed encrypted blobs; `holder_seed` derivation of the control branch, holding the handshake key, and the envelope branch, holding the envelope secrets, on a root only; `device_key` generation on every device; `device_roles` declaring root and reader within the versioned capability set that also names read-delegate and signer; `upgrade`, the in-place migration job verified against the on-chain binding and envelope-key registration; `workflows/identity` create or import, relayer-paid binding and identity contract account creation through the chain adapter, envelope-key registration with proofs of possession, device pairing by QR code or short authentication string, signer admission, re-scoping, and revocation on the contract account, role delegations under the handshake key, envelope secrets delivered to a reader over the paired channel, EIP-1193 and WalletConnect signing-only connections; the persistent credential store per entitlement; the secret region types.

**Entry.** Daemon skeleton; signature services; chain adapters, for binding and registration on Base Sepolia.

**Exit.** IW-01 through IW-07: distinct schemes composed and surviving restart; capability combinations failing closed; one identity, one binding, one registration across restart and reinstall; no decrypted credential or key in any custody surface; a reader device paired to a root on a second device profile reading alone with the seed never leaving the root, then revoked and purging its envelopes and envelope secrets at its next state view; recovery of the root on a fresh profile from the seed; an in-place custody upgrade preserving the principal, and a tampered rewrap rolled back; wallets signing only; envelope keys never wallet keys.

**Deliverables.** The custody and identity crates.

**Owner.** Daemon implementer with the cryptography implementer for derivation.

**Unblocks.** Entitlement acquisition; the installation coordinator; the publisher engine.

### Plaintext CAS, resolution orchestrator, and package host

**Purpose.** The benefit a developer feels, served before any cryptography is exercised on the read path.

**Scope.** `adapters/cas` with the content-hash layout of verified tarballs under the repointable root, atomic commit, quota, pinning, eviction warnings, nothing linking into the store; `workflows/resolve` trying sources in order and hedged under the catalogue's hedge delay, source deadline, and grant wait, upstream for any identity without a credential, and path and reason metrics; `workflows/health` independence status per asset and per project; `adapters/package-host-npm` serving packuments from the metadata store and tarballs on the loopback port, with canonical tarball URLs for lockfile portability and the metadata capture at every upstream resolution; the upstream plaintext-root check against the canonical record; store migration jobs for ST-02.

**Entry.** Daemon skeleton.

**Exit.** PR-01 parity with upstream across representative projects; PR-02 deterministic mapping; PR-03 each source chosen and its reason recorded with and without a held credential and with upstream available and unavailable, the hedge start, the bounded wait, and the pending failure asserted; PR-04 under interruption, races, and corruption; PR-05 lifecycles separate; PR-06 and AS-06 plaintext hit with everything else unreachable; PR-08 the foreground install's latency unchanged with every background stage stalled; PR-09 a lockfile committed on one machine installing on another with a different host port, and the metadata store serving with a staleness marker when upstream is unavailable; PR-12 independence shown identically on every surface; ST-02 migration with interruption and prior projects still installing afterward.

**Deliverables.** The CAS and package-host crates; the resolution workflow.

**Owner.** Daemon implementer.

**Unblocks.** First Finder engine.

### First Finder engine

**Purpose.** Absent packages enter the swarm without slowing the install that fetched them.

**Scope.** `adapters/ingest-npm` with as-is fetch, registry-signature attestation validation against the registry's published keys or recorded absence, metadata capture, and public-availability eligibility; `workflows/first_finder` foreground serve and background durable job: deployment identity under state lock, master scalar and parameter set through the KEM, a piece-group key and capsule randomness per group and the IV, encryption per group, one sidecar per live set and the hash-card, registration race with loser destruction, escrow custody of the master scalar, seed handoff, and the finder's grant of the asset's first entitlement to itself; the grant service under the asset scope.

**Entry.** Plaintext CAS and package host; swarm transport and seed host; registry contracts; chain adapters; identity and custody; the credential KEM, envelope, and mint-proof tickets for the self-grant.

**Exit.** FF-01 through FF-07 and AS-09, AS-10, AS-12: verified bytes served with every background dependency unavailable, forged and mismatched attestations refused and a missing one recorded; single completion after termination at every checkpoint; production randomness validated; race with one winner and clean losers, and a later escrow deployment of an existing asset admitted; the master scalar retained under custody; the self-grant recorded once. The grant service under the asset scope is built here and proven at the credential delivery milestone, whose producers it needs.

**Deliverables.** The ingest crate and First Finder workflow.

**Owner.** Daemon implementer.

**Unblocks.** Credential delivery; explicit publishing.

### Credential delivery and per-attempt authorization

**Purpose.** Close the read path: nothing requested from anyone at read time, and nothing decrypted without a current view.

**Scope.** `workflows/request`, one batched sponsored request per install for every unheld ledger-known asset, queued until submittable, with pickup of grants from chain events on any later run; `workflows/prefetch`, ciphertext and sidecar while a request is pending, under the seeding settings and quota, pinned, seeded, and matched to the granted set; `workflows/acquire` with relayer-paid mint, escrow grant, and funded purchase with lock; the grant service fulfilling open requests for held escrow assets as it seeds, requesters present or absent; the credential engine decrypting envelopes into the secret region, validity check, in-memory rerandomization, zeroization; `workflows/attempt` with `AttemptContext`, freshness clocks, the wallet-control assertion by a device key the identity's contract account admits in the same state view, three-state handling, batch form; `workflows/decrypt` decapsulation, unwrap, AES-CTR addressing, Bao verification, CAS commit, streaming; the interval-end watcher; the deployment gate; sale delivery from a fresh decryption with the transfer proof.

**Entry.** Identity and custody; chain adapters; KEM, envelope, proof; payload cipher and sidecar layer; registry contracts; First Finder engine; the relayer's request endpoint, or self-funded submission on Base Sepolia until it exists.

**Exit.** EC-01 through EC-08 and AS-07, AS-08 with the ingest source disabled, AS-19; CD-01, CD-02, CD-04, CD-08 and AS-14; CD-05's automatic grant service with the First Finder offline, a requester offline when its grant is authored and loading it on its next run, and a swarm of only non-holder seeders reporting no grant author truthfully, AS-26; PR-10 and PR-11 through AS-29, the first run ending independent with every branch exercised, held, absent, dead-deployment, explicit-publisher, and priced assets; the delivery scenario AS-13 through the daemon rather than the harness; the grant-request and grant-pickup integration points crossed; storage and crash artifacts free of decrypt-capable material; destruction at HARD and a reorganized transfer leaving the seller able to read; a reader device denied and purging at its next state view after its signer is revoked.

**Deliverables.** The acquisition, credential, attempt, decryption, and interval workflows.

**Owner.** Daemon implementer with the cryptography implementer.

**Unblocks.** The demonstrable milestone; the transaction flow proof.

### Demonstrable milestone: an install resolves against the swarm

**Purpose.** The earliest point the north star and the latency guardrail can be observed, and the first thing the project can show.

**Scope.** No new subsystem. A second clean identity on a second machine profile installs a pinned closure the first ingested, at the registry's speed with the requests registered and the ciphertext prefetched, and then, with the registry unavailable, through the packaged daemon and package host over the swarm, on a grant from the first identity fulfilled while the second was offline. The latency budget has been declared before this milestone.

**Entry.** Credential delivery and per-attempt authorization; plaintext CAS and package host; First Finder engine; swarm transport and seed host.

**Exit.** FF-08 and AS-11, AS-24, and AS-29 in substance; the north star, cross-project reuse, and the fraction of the closure independent after the first run measured on the dogfood population; whole-command elapsed time and authorization fraction evaluated against the declared budget as a pass or fail.

**Stop criterion.** If the benefit is not felt on the dogfood population, the case does not proceed to the adopting population.

**Deliverables.** The dogfood measurement record.

**Owner.** Project lead.

**Unblocks.** Confidence; nothing technical.

## Onboarding shells and services grouping

Re-mapped to tickets when the daemon grouping closes. The relayer and the publisher path are independent of the installer milestones and may run in parallel with them, and the relayer's entry is the chain adapters alone, so it may start before the daemon grouping closes; the grouping gate in the master plan says the same.

### Installation coordinator and platform adapters

**Purpose.** Either entry point produces one identical working system on a clean machine after only permitted consent.

**Scope.** `workflows/install` durable checkpointed plan; `adapters/platform` service lifecycle for `systemd`, `launchd`, and Windows Service or per-user scheduled task, credential stores, install paths; signed-artifact download and verification with Sigstore and platform signatures; reversible package-manager redirect across user, project, workspace, proxy, and custom-registry cases; backup and rollback; idempotent reinstall; repair; signed reversible update; uninstall with preservation; initial settings from the catalogue with path and consent items offered, including the request and prefetch items with the first-run cost disclosure and the cache-only mode; the reserved account-link step shown as unavailable; the first-run disclosure.

**Entry.** Daemon skeleton; identity and custody; the signing identities enrolled at bootstrap.

**Exit.** SI-03 through SI-18 and SI-20 and IC-01 through IC-03, IC-06, IC-07; ST-07; AS-03, AS-04, AS-05, AS-18, AS-30 on every supported platform; the consent trace clean under SI-19.

**Deliverables.** The installer binary and platform adapters; per-platform packages through `cargo-dist`.

**Owner.** Platform implementer.

**Unblocks.** The shells; acceptance.

### Relayer or paymaster

**Purpose.** A first-run user never acquires gas to install a free package.

**Scope.** `apps/relayer` over `alloy` and an ERC-4337 bundler and paymaster provider on Base behind a relayer adapter; the sponsorship mechanism for the identity contract account chosen here under LC-13's signed-intent rule, with sponsorship overhead measured; sponsor endpoints for the one binding, device admission and revocation, one batched request per install, free mints, and fulfilling grants; admission policy with a global per-window budget and maximum sponsored liability first and per-identity rate limits sized to a realistic closure as one layer; explicit failure states naming exhaustion and denial; cost and budget reporting with L2 execution and L1 data fee separated; the grant pool sized from harness and dogfood gas.

**Entry.** Chain adapters; registry contracts. Independent of the daemon grouping's closure.

**Exit.** RO-01 clean free onboarding, a device admission and a revocation sponsored, a batched request sponsored and its fulfilling grants sponsored, subsidy exhausted and denied with actionable recovery, requests queued locally, and no false authorization; LC-10 and AS-27 a flood of fresh identities never spending beyond the configured budget and exhaustion reported truthfully; RO-02 paid acquisition refused without funds.

**Deliverables.** The relayer service and its operating runbook.

**Owner.** Relayer operator.

**Unblocks.** Free onboarding end to end; the transaction flow proof.

### CLI and desktop application

**Purpose.** The two control surfaces the project owns outright, over the daemon's IPC.

**Scope.** `apps/cli` with `clap`: consent, settings, diagnostics, package-host control, identity, jobs, repair, uninstall; `apps/desktop` on Tauri 2: configuration, identity and custody workflows including device pairing, admission, and revocation, the custody upgrade, and root recovery, adapter selection, jobs, the seeding archive, the settings catalogue, diagnostics, repair, recovery.

**Entry.** Installation coordinator; daemon skeleton.

**Exit.** IC-08 and RO-05 identical facts across surfaces; ST-05 surface parity for every catalogue entry; SW-07 the archive view matching the host; SI-16 repair from each surface; accessibility baselines as adopted.

**Deliverables.** The CLI and desktop packages.

**Owner.** Platform implementer.

**Unblocks.** Acceptance scenarios that exercise every surface.

### Visual Studio Code extension and npm bootstrap package

**Purpose.** The two onboarding paths the specification names, as thin shells.

**Scope.** `shells/vscode-extension` in TypeScript over the IPC client, bundling or fetching the signed installer, status and consent views, no protocol logic; `shells/npm-bootstrap` as a `postinstall`-free package whose `bin` invokes the signed installer.

**Entry.** Installation coordinator; CLI and desktop, for the IPC client patterns.

**Exit.** SI-01 and AS-01; SI-02 and AS-02; both paths converging on one postcondition, IC-01.

**Deliverables.** The extension and the npm package, published to their hosts' test channels.

**Owner.** Platform implementer.

**Unblocks.** Acceptance.

### Explicit publisher path and issuance policy

**Purpose.** The project publishes its own package, seeding the swarm with its first content and supplying the only assets that may carry a price.

**Scope.** `workflows/publish` with seed-derived parameter sets and capsule randomness, `adapters/identity-proof` provenance and maintainer-OAuth adapters in strength order, proof class recorded on chain, dependency-closure ingestion as First Finder, the issuance policy service serving mints automatically under configuration with per-request approval as a declared capability; the publisher seed under custody.

**Entry.** First Finder engine; identity and custody; registry contracts.

**Exit.** PC-01, PC-03 replay and redirection rejected, FF-09 and AS-22, AS-23; a second identity consumes the published package; the issuance policy serves a mint with no person present.

**Deliverables.** The publish workflow and proof adapters; ChainTorrent's own package published on Base Sepolia.

**Owner.** Daemon implementer.

**Unblocks.** The transaction flow proof; the project seed host's content; the claim verifier's inputs.

### Claim verifier and escrow claim

**Purpose.** A maintainer takes complete ownership of escrowed records without the First Finder and without any stored maintainer binding.

**Scope.** `apps/claim-verifier` running provenance-then-OAuth verification, establishing the claim set from upstream metadata at verification time, signing vouchers bound to claimant, set, contract, chain, nonce, and expiry under an on-chain registered key with rotation and revocation; the escrow claim contract surface in the registry; `workflows/claim` in the daemon; claimant parameter-set registration, optional encrypted handover with compatibility proof, successor deployments with a sidecar per live set, sidecar addition to live escrow deployments from an escrow-era credential, voluntary migration and retirement.

**Entry.** Explicit publisher path; registry contracts; credential delivery.

**Exit.** PC-04 through PC-07 and CD-06; AS-16 claim with the First Finder absent, successor readable under both sets, the escrow deployment opening under the claimant's set after sidecar addition, one holder migrated; AS-27's revoked-key voucher rejected.

**Deliverables.** The verifier service; the claim workflow; the contract surface deployed.

**Owner.** Claim verifier operator with the contract implementer.

**Unblocks.** Acceptance.

### Project seed host, site, and WebAssembly demonstration

**Purpose.** A seed of the core closure from day one and an education tier a prospective developer can try with nothing to sign up for.

**Scope.** The daemon's seed host configured for persistence, seeded by the dogfood publication, and holding an entitlement and credential for every asset it seeds; the static site; `apps/wasm-demo` built with `wasm-bindgen` from the domain, hashing, pairing, KEM, and cipher crates, running against the sample deployment generated at the payload cipher milestone; the download links; the statement that seeding and the CAS do not run in a browser.

**Entry.** Explicit publisher path; swarm transport; payload cipher and sidecar layer, for the sample deployment.

**Exit.** The core closure retrievable from the seed host after publication, FF-09; SW-08, a new identity obtaining a grant for a core-closure asset from the seed host with every other holder offline; the demonstration performing real resolution, verification, and decapsulation with no network access on the read path; the protocol resolving with the seed host unreachable.

**Deliverables.** The seed host deployment; the site; the demonstration bundle.

**Owner.** Project lead with the daemon implementer.

**Unblocks.** Cold start; the prospective-developer audience.

### Observability reconciliation

**Purpose.** Every metric the economics depend on, reconciled against induced activity, with no secret anywhere.

**Scope.** Completion of the RO-03 record across every emitting subsystem; RO-04 correlation across concurrent installs; RO-06 attempt latency against the declared budget as pass or fail; the telemetry scan; secret-free failure messages under XA-04.

**Entry.** Every daemon and service milestone, since each emits.

**Exit.** RO-03 through RO-06 and XA-04; AS-20 in rehearsal.

**Deliverables.** The reconciled metrics schema and scan.

**Owner.** Observability implementer.

**Unblocks.** The acceptance run.

### Demonstration harness

**Purpose.** Reproducible participants, wallets, chain state, and failures, without which the completion boundary cannot be met.

**Scope.** `apps/harness-demo`: spawned daemons under distinct identities and machine profiles, Anvil and Base Sepolia chain state, wallets funded and unfunded, fault injection at process, network, disk, and chain, the race and reorganization drivers, the deliberately incompatible adapter declarations the incompatibility scenario rejects, the clean-machine runner integration for CI.

**Entry.** Daemon skeleton; runs alongside every milestone from the daemon grouping onward and closes here.

**Exit.** XA-07 every requirement's proof class mapped to a facility; XA-03 termination at every persisted transition driven by the harness; every scenario from AS-06 onward runnable unattended.

**Deliverables.** The harness and its scenario definitions.

**Owner.** Test infrastructure implementer.

**Unblocks.** The acceptance run.

## Acceptance and release grouping

### Transaction flow proof on the Base mainnet pilot

**Purpose.** Prove a nonzero transaction settles, on the one asset class where pricing is legitimate, while the entitlement graph is two identities.

**Scope.** Contract suite deployed to Base mainnet; ChainTorrent's package published and priced trivially above gas under the issuance policy; a funded buyer acquires; the holder resells; the settlement boundary asserted; every guardrail live.

**Entry.** Explicit publisher path; credential delivery; relayer; registry contracts on mainnet; external cryptographic review dispositioned, which gates any priced deployment; legal review of priced entitlements and the paymaster.

**Exit.** LC-03 First Finder pricing refused; LC-05 and CD-02 the seller's proof verified and payment released in one transaction; RO-02 real funds required; AS-15 the buyer decrypting at the declared tier and the seller destroying at HARD.

**Stop criterion.** No priced deployment before review and legal closure; a composition or verifier finding not dispositioned halts here.

**Deliverables.** The mainnet deployment record; the proof's settlement records.

**Owner.** Project lead with the contract implementer and relayer operator.

**Unblocks.** The acceptance run's priced scenario.

### External review and legal prerequisites closed

**Purpose.** The two release conditions engineering does not control, started at the harness and closed here.

**Scope.** Cryptographic review findings on the composition claims and the verifier tracked to disposition; license text adopted and the content-terms field populated for the project's packages, LI-01; the eligibility principle and residual restated in MVP Scope; regulatory review of priced entitlements and the paymaster; export constraints on the packaged binary confirmed; code-signing certificates in hand.

**Entry.** Harness report, for review inputs; nothing else technical.

**Exit.** Every finding dispositioned and recorded in release evidence; every legal item recorded as a release-evidence document.

**Deliverables.** Release-evidence documents.

**Owner.** Project lead.

**Unblocks.** Release.

### Acceptance run

**Purpose.** The completion boundary, met or not.

**Scope.** Every acceptance scenario through the packaged applications on clean machines on every supported platform, driven by the demonstration harness against Base Sepolia, the priced scenario against the Base mainnet pilot, with every metric reconciled against induced activity and the telemetry scanned; no test-only bypass of onboarding, the package host, capability resolution, or the deployed verifier.

**Entry.** Every milestone above.

**Exit.** Every acceptance scenario passes; every guardrail holds; the requirement-to-proof reconciliation is complete.

**Stop criterion.** Any guardrail breach blocks release, and no guardrail may be waived by adjusting the metric.

**Deliverables.** The acceptance record.

**Owner.** Project lead.

**Unblocks.** Release.

### Dogfood baseline and release

**Purpose.** Ship, with the numbers that make the next decision possible.

**Scope.** The dogfood baseline for the north star, primary KPIs, and leading indicators recorded on the Base mainnet pilot alongside the harness measurements, the mainnet re-take of the L1 data fee, the chosen piece-group size, curve, and attempt-rule parameters; signed packages published; the extension and npm package published; the site live with the demonstration; the reporting cadence set; the next planning cycle's re-map of deferred items begun.

**Entry.** Acceptance run; external review and legal prerequisites closed; transaction flow proof.

**Exit.** Release evidence complete under the Application Requirements' completion boundary; both onboarding paths reaching the same postcondition from published artifacts.

**Deliverables.** The release and its evidence.

**Owner.** Project lead.

**Unblocks.** The adopting population.

# Iteration Semantics

**Resolution decays and re-maps.** The foundation and harness groupings and the hashing, signature, and swarm milestones are at ticket resolution now. When the harness report node commits, the remaining protocol core milestones are re-mapped from sprints to tickets using what the harness taught and its measured throughput, the daemon grouping from epics to sprints, and so on outward; this document and the dependency map are revised in place.

**Nodes are authored from tickets through the ordinary path.** A ticket names a source file's role, dependencies, and proof; the workplan author writes the node in the fixed element order, resolving every path and symbol so the implementer instantiates rather than invents; the implementer builds it under the repository's process topics.

**A milestone closes on its commit.** The last node in a milestone's chain carries the integration test across the chain and the commit; the milestone's exit rows are the requirements that test proves.

**Discovery revises the map, not the node.** An implementer who finds that a node needs a second file, a missing producer, or a decision the node did not make reports and halts; the author revises the map and the node, and the milestone's scope is restated in full, never as a delta.

**Stop criteria are evaluated where named.** At the harness report, the post-harness re-map, the demonstrable milestone, the acceptance run, and review closure; each halts the plan at that point when met.

**Parallelism is by dependency only.** Independent milestones may run together when the team allows; the map's edges, not a calendar, decide.

**Status is recorded in the master plan**, which summarizes these milestones and marks each as unstarted, in progress, or closed; this document holds the definitions and the master plan holds the state.

# Features Context

The in-scope features of the [feature spec](feature-spec.md) map onto milestones as follows. Cryptographic services and validation harness: the harness grouping and the hashing and payload cipher milestones. Adapter composition: the daemon skeleton. Entitlement ledger and settlement: registry contracts and chain adapters. Swarm, discovery, and seed hosting: swarm transport and seed host, and the project seed host. Package serving and CAS: plaintext CAS and package host. Encrypted content consumption: credential delivery and per-attempt authorization. First Finder bootstrap: First Finder engine. Credential delivery: credential delivery and per-attempt authorization, and claim. Identity, wallet, and custody: identity and custody. Settings and configuration: daemon skeleton, with surfaces in CLI and desktop. Self-installing onboarding: installation coordinator, the extension and npm bootstrap. Relaying: relayer. Explicit publishing and escrow claim: explicit publisher path, claim verifier and escrow claim. Project seed host and site: its milestone. Observability: observability reconciliation. Test facilities and demonstration harness: demonstration harness and the acceptance run. Transaction flow proof: its milestone. The deferred features and the travelling-developer story constrain choices in identity and custody, the daemon skeleton's IPC, and the publisher path's issuance policy, and consume no milestone.

# Feasibility Insights

From the [feasibility assessment](technical-feasibility.md): technical feasibility high, delivery feasibility undetermined until the harness report calibrates throughput, which is why that milestone carries the re-map and the funding stop criterion. Platform breadth is the largest delivery risk and is concentrated in the installation coordinator milestone, whose scope is bounded by the decided platform matrix. The demonstration harness is load-bearing and is its own milestone. The Solidity verifier is generated from the Rust reference in the delivery-proof milestone. Transfer is proven last, in the transaction flow proof, and its external gates start at the harness so they do not arrive at the end.

# Non-Functional Alignment

Each non-functional requirement of the [review](non-functional-requirements.md) is proven inside the milestone that owns its subsystem: security rows in the harness, contracts, and credential-delivery milestones; performance rows at the harness report, the demonstrable milestone, and observability reconciliation; reliability rows in the daemon skeleton, installation coordinator, and swarm milestones; maintainability rows in the workspace bootstrap and the tickets that own each type; compliance rows in external review and legal prerequisites. The requirements the review added, XA-08, XA-09, XA-10, IC-09, IC-10, RO-07, RO-08, and LI-02, are carried where they land: signing-key custody in the bootstrap and installer; the package-host threat model in the daemon skeleton's IPC; footprint limits and cold-start in the daemon skeleton and settings; backoff bounds in the chain and swarm adapters; the suite compatibility statement at release; regulatory and export review in legal prerequisites; accessibility in CLI and desktop; the first-run cost disclosure in the installer.

# Architecture Summary

As the [system architecture](system-architecture.md) states: one canonical ciphertext per deployment and one sidecar per live parameter set, each its own object, on the swarm, entitlements on Base, credentials delivered inside settlement and verified by proof, per-attempt local authorization, and a daemon serving npm's protocol from whichever of cache, swarm, and registry delivers within budget, never slower than the registry alone by more than the hedge delay, leaving a first run independent, with nothing on the read path.

# Services

The daemon, relayer, claim verifier, seed host and site, validation harness, and demonstration harness, each built in the milestone named above and each bounded by what it must never become.

# Components

The rings and the adapter table of the system architecture; the subsystem register of the technical requirements assigns each to its crate and, through this document, to its milestone.

# Dependency Resolution

Declaration, composition, suite binding, fail closed, and the on-chain factory, built in the daemon skeleton and the registry contracts and proven under IC-05, SI-10, and AS-17 in the acceptance run.

# Component Details

The daemon's subsystems, their ownership, trust boundaries, process model, and storage ownership are in the system architecture's daemon breakout; their APIs, schemas, and file tree are in the technical requirements. This document adds only their milestone assignment, which the Milestones section states per scope line.

# Integration Requirements

The integration points of the dependency map are each crossed by the integration test of the milestone that first joins the two sides: verifier parity and precompile encoding in the delivery-proof milestone; the decapsulation-to-cipher seam and sidecar commitment in payload cipher and sidecar layer; envelope-in-settlement and the signed intent in registry contracts; attempt context in chain adapters; resolution order and the package-manager protocol in the CAS and package host; the persistent credential and device admission in identity and custody; grant request to holder and grant pickup from events in credential delivery; installer-to-daemon and shells-to-coordinator in the installer and shells; relayer-to-contracts in the relayer; voucher-to-contract in claim; the site demonstration in the seed host and site milestone.

# Migration Context

Migrations in the MVP's own lifetime, and the one beyond it. Settings and store roots migrate through checkpointed jobs under ST-02. Configuration schema migrates under IC-04 and SI-17 with reversible updates. Custody blobs migrate in place between adapter versions by a checkpointed job that verifies the re-derived public keys against the chain before retiring the old blob. A deployment suite changes only by a successor deployment carrying a sidecar per live parameter set, and a parameter set retires only when no live entitlement remains under it. The `librqbit` overlay migrates onto each upstream release by rebase, with the hard-fork trigger ending that track. Beyond the MVP, entitlements and contract state do not move between chain adapters, no mechanism exists, and one is required before any second chain, including the project's own, carries live entitlements; it is held in the workplan To-Do and is not scheduled here.
