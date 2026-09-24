<!-- Template: thesis_technical_approach.md -->
# ChainTorrent MVP Technical Approach

Draft, 2026-09-23. An overview of how the MVP is built, drawn from [cryptography.md](../research/cryptography.md), [MVP Scope](../research/MVP%20Scope.md), [MVP Application Requirements](../research/MVP%20Application%20Requirements.md), [MVP Execution Trace](../research/MVP%20Execution%20Trace.md), and the To-Do list and build sequence in the [ChainTorrent MVP workplan](../workplans/current/ChainTorrent%20MVP.md). Those documents are authoritative; this one arranges them for a reader deciding how to build, and it adds no rule they do not already carry. Decisions on the MVP path are stated as decided; the parameters the harness measures carry a Feedback block, and a blank answer means the build proceeds on the assumption stated there.

# Architecture

## The protocol in one paragraph

A deployment is one canonical ciphertext plus one public header sidecar per live parameter set, seeded to a content-addressed swarm and committed on chain by three BLAKE3/Bao roots: ciphertext, sidecar, and plaintext. An entitlement is a single-owner bearer asset on the ledger, bound to the asset rather than to any deployment. Each ownership interval receives a native credential, delivered inside the settlement that creates the interval and verified by a proof the contract checks through the chain's pairing precompiles. A conforming client authorizes each decryption attempt itself, from a fresh view of consensus state at the deployment's declared settlement tier, and decrypts locally by decapsulating one capsule per piece group. No service sits on the read path. When the entitlement transfers away, the client stops at the declared tier and destroys its decrypt-capable material at hard finality.

## Layers and decentralization boundary

The specification defines decentralization per layer rather than as one property. Content distribution, entitlement ownership and transfer, transaction ordering, decryption authorization, content governance, and discovery are each decentralized in a stated sense; content identity is rights-holder asserted. The rule that follows is the architectural boundary: a centralized service may exist as a convenience, business, gateway, indexer, or application, but no service may become an unavoidable authority over a protocol property the protocol defines as decentralized. The relayer, the claim verifier, and the project seed host are all permitted by that rule and all named as scaffolding.

## Implementation rings

The Application Requirements fix three dependency rings with dependencies pointing inward.

| Ring | Contents | Must not depend on |
| --- | --- | --- |
| Protocol and domain | Pure Rust types and rules: canonical identities, authenticated suites and hash-cards, attempt contexts, delivery statements, entitlement and interval state, custody state, manifest and sidecar bounds, parameter-set state, claims, lifecycle transitions | Tauri, Visual Studio Code, npm, any chain SDK, wallet product, transport, or storage engine |
| Application workflows | Rust use cases: installation, package serving, cache resolution, grant requests and ciphertext prefetch, encrypted acquisition, First Finder ingestion, credential delivery, per-attempt authorization, decryption, seeding, publishing, claiming, transfer, repair, recovery; long-running operations as durable restartable jobs | The identity of any adapter implementation |
| Adapters and host shells | Rust adapters for package-manager protocols, chains, pairing backends, wallets, custody, ingest sources, transports, discovery, seed hosts, claim verifiers, storage, telemetry; thin TypeScript or JavaScript shells for Visual Studio Code and npm | Nothing above them; they are the outermost ring |

## Adapter composition

Every adapter declares capabilities, version, and compatibility. Consumers resolve against declarations, never against an implementation's name. Resolution validates the complete composition before any operation begins and fails closed before any credential is exercised, chain mutation sent, or swarm transfer started. The unit of cryptographic composition is an immutable deployment suite fixed in the authenticated hash-card: pairing adapter, credential-KEM adapter and live parameter sets with sidecar roots, payload-cipher adapter and piece-group size, key-agreement adapter, delivery-proof adapter, KDF and hash-to-scalar mappings, delivery-statement version, attempt-rule parameters, settlement tier, content commitments, and transport locators. Changing an incompatible cryptographic component is a successor deployment whose body carries a sidecar for every live parameter set, never a reinterpretation of existing ciphertext.

The adapter surfaces the MVP resolves, with their MVP implementations:

| Adapter | MVP implementation | Resolved by |
| --- | --- | --- |
| `IPayloadCipherAdapter` | `AesCtrAdapter`: AES-256-CTR, 64-bit IV, 64-bit counter, keyed per piece group | Suite |
| `IPairingAdapter` | `Bls12381PairingAdapter` primary, `Bn254PairingAdapter` retained | Chain adapter, from Base's precompiles |
| `ICredentialKemAdapter` | `Bb1DepthOneKemAdapter`, entitlement scope for explicit publishers, asset scope for escrow | Suite |
| `IKeyAgreementAdapter` | `PairingElGamalAdapter` for credential envelopes | Suite |
| `IDeliveryProofAdapter` | `SchnorrFsDeliveryProofAdapter`, in the verifier form the pairing adapter declares | Suite |
| `ISignatureAdapter` | `Ed25519Adapter` at handshake and content layers; `Secp256k1Adapter` at the EVM chain layer | Per layer |
| `ISettlementAdapter` | EVM tier mapping onto `INCLUDED`, `SOFT`, `HARD`, `SETTLED` | Chain adapter |
| `IEntitlementStateAdapter` | Registry views `evaluateAuthorization` and paginated batch | Chain adapter |
| `IIdentityAdapter`, `IPublisherProofAdapter`, `IClaimVerifierAdapter` | Publisher authority and escrow identity adapters; provenance-then-maintainer-OAuth proof ordering; attestor claim verifier with an on-chain revocable key | Per asset |
| `IIngestSourceAdapter` | npm registry, as-is tarball with its release attestation or recorded absence | Policy: content its publisher distributes to the public at no charge; npm availability is the test |
| `IPackageHostAdapter` | npm registry protocol served locally | Package manager |
| `ISwarmTransportAdapter` | Embedded `librqbit` as the sole MVP transport, a soft fork pinned by commit with its hooks submitted upstream | Deployment, multi-homed |
| `IPeerDiscoveryAdapter` | DHT, tracker, PEX, local, on-chain seeder map, concurrent and non-authoritative | Client |
| `ISeedHostAdapter` | Owned daemon; delegation to an existing BitTorrent client | Installation |
| `IKeyCustodyAdapter` | Local keystore under the OS credential store, keys derived from a holder seed, device roles declared as versioned capabilities; default identity is a local key | Installation |

## Cryptographic construction, stated once

Type-3 pairing groups of prime order on BLS12-381, with BN254 retained as the second verifier form. A parameter set is `g1, u0, u1` in the first group and `g2, hpub = g2^α` in the second, with master scalar `α` held by the issuer. The entitlement's identity element is `F_I = u0 · u1^I`. A credential is `(g1^α · F_I^r, g2^r)`; the seller rerandomizes it as `(A · F_I^s, B · g2^s)` without the master scalar; validity is the public pairing check. A capsule is `(g2^t, u0^t, u1^t)`, well-formed under two pairing checks, encapsulating `e(g1, hpub)^t`; decapsulation is two pairings and one scalar multiplication. A domain-separated KDF of that value and the context yields the set's wrapping key; the piece-group key, chosen by the encryptor, travels in the sidecar wrapped under it, so independently generated sets open one ciphertext. Envelopes are ElGamal in each source group to the recipient's two independently keyed envelope keys. Delivery proofs are generalized Schnorr under Fiat–Shamir over the full statement, six scalars, plus three second-group elements where the verifier has only first-group arithmetic. Security rests on SXDH, decisional BDH-3b, and the random-oracle model. Under the escrow suite the identity element is fixed per asset, the capsule shrinks to two elements, and any current holder can author a grant's credential.

# Components

## Deployable applications

| Application | Runtime | Responsibility |
| --- | --- | --- |
| Visual Studio Code extension | Thin TypeScript over Rust | Primary onboarding, status, consent, adapter configuration, health, repair |
| npm bootstrap package | Minimal JavaScript shim over signed Rust artifacts | `npx chaintorrent` as a complete one-command alternative onboarding path invoking the same installer |
| Desktop application | Rust with Tauri | Configuration, identity and custody workflows, adapter selection, job status, diagnostics, repair, recovery |
| CLI | Rust | Scriptable consent, configuration, diagnostics, package-host control, identity, jobs, repair, uninstall; not a resolver |
| Local daemon and package host | Rust service, single instance per machine | Local registry endpoints, CAS orchestration, encrypted acquisition, credential custody, per-attempt authorization, decryption, durable jobs, seed-host control, authenticated IPC |
| Cryptographic validation harness | Rust | KEM, envelope, and proof on the resolved pairing adapter against a deployed verifier; the measurements that fix piece-group size and curve |
| Relayer or paymaster | Rust service plus ERC-4337 support where available | Sponsors the free path, batched grant requests and their fulfilling grants, and the one-time identity binding; a global budget with per-identity limits sized to a closure; cost and budget reporting |
| Claim verifier | Rust service | Authenticates claimants off chain and signs vouchers over committed claim sets under an on-chain registered, revocable key |
| Contract suite | Solidity | Registry, deployments and hash-cards, parameter sets, envelope-key registry, entitlements with interval state, delivery verification, escrow, identity binding, authority transfer, claims |
| Demonstration harness | Rust | Controlled participants, services, wallets, chain state, and failures for reproducible acceptance scenarios |
| Project seed host and site | Rust service plus static site | Persistent first seeder of the core packages and their closure; optional relayer and discovery host; a convenience no client depends on |

Logical applications may share a packaged binary; their module contracts and lifecycle responsibilities remain independently testable.

## Module families

Each family below is a composable module group in the Application Requirements, and each requirement in it carries an integration or end-to-end proof.

- **Installation and configuration** (IC): durable bootstrap coordinator shared by both shells; platform detection and signed-artifact installation; service lifecycle; versioned configuration registry without raw secrets; capability resolution; explicit reversible package-manager configuration; checkpointed migration, rollback, repair, uninstall; health states exposed identically on every surface.
- **Package serving and resolution** (PR): npm-compatible host leaving resolution to npm; deterministic coordinate-to-identity mapping; plaintext from the first source that delivers it within the budget, with the ingest source serving any identity that holds no credential while it is available; verified atomic plaintext CAS of tarballs; cache policy separating plaintext and ciphertext lifecycles; plaintext hits with no remote access; ciphertext through authorization before serving; upstream bytes served without waiting for any background stage; metadata capture and canonical tarball URLs; a durable batched grant request for every unheld asset, fulfillable while the requester is offline; background prefetch of ciphertext under the seeding settings; per-asset independence visible on every surface.
- **Encrypted content consumption** (EC): authentication of record, hash-card, suite, bounds, and sidecar before any credential is exercised; Bao-verified acquisition across locators; separate acquisition and per-attempt authorization; the attempt rule over `AttemptContext` with `τ_soft` and `τ_wallet`; three-state result handling; envelope decryption, credential validity, capsule well-formedness; per-group decapsulation and AES-CTR addressing; memory-only decrypt-capable material with interval-end destruction.
- **First Finder bootstrap** (FF): as-is ingest with attestation validation; foreground serve before background bootstrap; durable idempotent bootstrap job; random master scalar, parameter set, capsule randomness, IV; state-locked escrow race with loser destruction; escrow custody of the master scalar; completion requiring registration and persistent seeding; later-seeker install without npm; dependency-closure ingestion on publication.
- **Swarm, discovery, and storage** (SW): transport by authenticated root with BitTorrent as one implementation; concurrent non-authoritative discovery; session-independent seed host; delegated host with Bao custody challenges; root-keyed ciphertext store distinguishing obligated from voluntary; resumable failure handling; visible seeding archive.
- **Ledger, contracts, and settlement** (LC): the full contract state model; asset binding; $0.00 escrow issuance with the priced dogfood exception; tier and reference exposure; transfer with delivery verification, atomic interval advance, and payment release; batch reads that never broaden authorization; race- and replay-safe registration and claim; envelope-key proofs of possession; parameter-set liveness and the sidecar-coverage rule; lock expiry and rate limits.
- **Identity, wallet, and custody** (IW): separable capabilities bound to one principal; declared custody capabilities; first-run identity, one binding, one envelope-key registration; custody of identity, envelope, issuance, and seed material with per-entitlement envelopes; recovery and multi-device without silent principal change; authority rotation that versions nothing; EIP-1193, WalletConnect, EIP-712, local default identity.
- **Cryptographic services** (CR): payload cipher, Bao commitments and challenges, signatures, envelopes, randomness and derivation lineages, descriptor validation, secret lifecycle, credential KEM, delivery proof, pairing adapter, sidecar wrap and hash assignment.
- **Credential delivery** (CD): delivery at mint, at sale, and at escrow grant; on-chain verification; persistent credential recovery; declared identity scope; claim with optional handover and sidecar-per-live-set; the validation harness.
- **Publishing and claims** (PC): explicit publisher path with derived keys; escrow record contents; proof binding; claim verification independent of any unavailable party; complete authority transfer on claim; claim-set vouchers; verifier key rotation.
- **Licensing, relaying, and observability** (LI, RO): license and content terms; free-path sponsorship with abuse controls; real funding for paid dogfood; metrics, correlated traces, consistent health, and the pre-declared latency budget.
- **Cross-cutting** (XA): untrusted-until-validated inputs; authenticated least-privilege local API; idempotent recoverable workflows; safe failure messages; no developer toolchain on user machines; multi-node state views; test facilities for every proof class.

## Execution trace

The Execution Trace fixes the state machine a package request follows. Local plaintext hit serves immediately. Otherwise the ledger is consulted: an absent asset takes the First Finder path, upstream fetch, verify, commit, serve, then asynchronous build, race, register, custody, seed. A present asset loads the authenticated descriptor, passes the manifest gate, acquires and verifies ciphertext and sidecar, seeds them, checks for an entitlement, registers envelope keys and acquires one if absent with delivery verified on chain, loads the credential into memory, and then for each piece group constructs an attempt context, reads a state view, waits on pending settlement, halts on denial, and on authorization decapsulates, unwraps the group key, and decrypts, finally verifying the plaintext root and committing to the CAS. A later transfer out ends the interval; a later claim transfers authority and registers the claimant's parameter set while escrow-era credentials continue.

# Data

## On-chain state

| Record | Contents | Mutability |
| --- | --- | --- |
| Asset record | Identity hash `BLAKE3(name@version)`; upstream attestation or its explicit absence; publisher or escrow authority and proof class; entitlement adapter; issuance authority; parameter sets marked live or retired; every live deployment | Append-only; authority transfers on claim |
| Deployment hash-card | Registry-assigned deployment identifier; immutable suite and version; payload cipher, counter layout, IV, piece geometry, piece-group size, extent; one sidecar root and locator per live parameter set; KDF, hash-to-scalar, and encoding identifiers; delivery-statement version; `τ_soft`, `τ_wallet`, `minSettlementTier`; ciphertext root; plaintext root and disclosure mode; transport locator set; advisory backlink root | Immutable; changes are successor deployments |
| Parameter set | Public elements of one issuance authority, registered per asset, live or retired | Retired only when no live entitlement remains under it |
| Entitlement | ERC-721 token bound to the asset; interval counter; current holder's envelope keys; current envelope digest | Advances at every mint and transfer |
| Envelope-key registration | Two envelope public keys per identity with proofs of possession under a wallet signature | Reusable across entitlements |
| Identity binding | Bound handshake public key and scheme on chain; anchor hash of the full DID Document resolved off chain | Once per identity, relayer-paid |
| Escrow record | Source provenance, attestation, escrow parameter set; no maintainer commitment | Until claimed |
| Settlement calldata and events | Envelope and delivery proof at each mint, grant, and transfer | Chain history; the holder's envelope is recoverable from it |
| Claim verifier key | Registered attestation key with rotation and revocation | Rotated or revoked on chain |

Three content roots are registered per deployment, and public registries use Public plaintext-root disclosure mode without exception.

## Swarm objects

- **Ciphertext**: one continuous stream per deployment under one IV, keyed per piece group, piece-aligned to the cipher block, committed by the ciphertext root; opaque to seeders.
- **Header sidecar**: one per live parameter set, its own object with its own locator, one entry per piece group holding the set's capsule and the wrapped piece-group key, committed by its own root; useless without a credential; seeded like any object.
- **Manifest**: the transport's descriptor, of which the hash-card is the superset; legacy piece hashes carried only by the compatibility adapter.

## Local stores

| Store | Owner | Keyed by | Lifecycle |
| --- | --- | --- | --- |
| Plaintext CAS | Client | Content hash | Persists by design; outside the authorization boundary; user quota with pinning of artifacts a live project resolved and warning before evicting anything not re-authorizable |
| Ciphertext store | Seed host adapter | Ciphertext root | Shared across transports; separately budgeted; obligated versus voluntary holdings recorded; never evicted by plaintext policy |
| Custody store | `IKeyCustodyAdapter` | Identity | Handshake, chain, and envelope keys; master scalars and derivation seeds; per entitlement the envelope and interval index; never the decrypted credential or piece-group keys |
| Durable job store | Daemon | Job | Checkpointed, idempotent, restartable across process and machine failure |
| Configuration registry | Daemon | Version | Defaults, overrides, secret references, capabilities, endpoints; no raw secrets |
| Logs and metrics | Daemon | Request correlation | RO-03 metrics and RO-04 traces; no secrets |

## Memory-only material

Decrypted credential, expanded cipher state, derived piece-group keys, buffered keystream, and every live decryption context. Zeroized at success, interval end, cancellation, loss, and error; never written to disk, logs, diagnostics, or crash artifacts.

## Secrets and their holders

| Secret | Holder | Lifetime |
| --- | --- | --- |
| Master scalar `α` | Issuer: publisher, seed-derived; First Finder, random, retained as escrow custodian for optional handover | Per parameter set; loss recovered by registering a new set |
| Credential exponent `r` and offsets `s` | Never disclosed to anyone; offsets private to the seller and buyer | Discarded after use |
| Envelope coins | Credential author | Discarded after posting |
| Envelope secrets `x, y` | Holder | Persistent for the interval; loss ends transfer, not reading |
| Capsule randomness `t` | Encryptor | Discarded after encapsulation |
| Publisher seed phrase | Publisher | Recovery secret, not rotatable; never leaves the client |

# Deployment

**Target platforms.** Windows, macOS, and Linux for the daemon, CLI, desktop application, and extension, with signed native artifacts per platform. The packaged MVP runs without Rust, Solidity tooling, or repository source; npm itself is the only developer tool assumed.

**Installation.** Both shells invoke one Rust installation coordinator executing a durable plan: detect platform and capabilities; download and authenticate artifacts; create and permission stores and the IPC endpoint; install, start, and enable the daemon or an equivalent persistent user service; offer explicit reversible package-manager redirect; create or import identity and complete the relayer-paid binding and envelope-key registration, or select cache-only mode with no identity; take the request and prefetch consent items with the first-run cost disclosure; configure default chain, discovery, relay, and ingest endpoints; resolve the full adapter composition; and report ready only on an active end-to-end health probe. Every mutating stage is backed up and rolls back on later failure.

**Chain deployment.** The contract suite deploys to Base through an on-chain factory that injects constructor-bound adapter addresses from an adapter registry governed by a single project-held key, immutable once bound, with optional EIP-1967 proxies for adapter logic. The validation harness deploys the verifier to Base Sepolia. Delivery verification uses EIP-2537 on BLS12-381 as the primary form and EIP-196 and EIP-197 on BN254 as the retained form; both precompile sets are live on Base. Every function that mutates identity-bound state takes the acting identity and a signed intent verified by ECDSA or ERC-1271, so the identity's chain-level form and the sponsorship mechanism are chosen at the relayer milestone without contract rework.

**Services.** The relayer or paymaster, the claim verifier, and the project seed host and site run as Rust services with operational controls, explicit failure states, and cost reporting. Each is replaceable and none is on any client's critical path.

**Updates.** Signed, atomic, schema-aware, and reversible across application and configuration migrations, with active durable jobs and populated stores.

**Uninstall.** Restores package-manager configuration, stops and removes owned services, and preserves identity, recovery material, and plaintext unless removal is explicitly requested.

# Sequencing

The build order, by dependency role. The [dependency map](dependency-map.md) holds it at decaying resolution and the [milestones](milestones.md) hold each step's entry and exit.

- **Decisions before nodes.** The credential construction, key custody, host adapter approach, claim granularity, launch network and curve, and the escrow record's form are decided; the license is a release prerequisite rather than a node blocker.
- **Foundation.** The workspace, the canonical encoding, the tracing redaction layer, and the continuous-integration matrix, on which every node builds.
- **Cryptographic validation harness.** The credential KEM, envelope, and delivery proof on both curves with the verifier deployed to Base Sepolia, producing the measurements that fix the piece-group size and confirm the curve; it precedes any node that encrypts a registered deployment, because piece geometry is a suite parameter every deployment carries.
- **Hashing, signatures, swarm transport, seed host, and the ciphertext store.** Provable end to end with no chain and no KEM, so they run beside the harness and the contracts.
- **Registry contract**, carrying batch resolve, batch authorize, pagination, envelope-key registration, the parameter-set registry with the sidecar-coverage rule, per-entitlement interval state, delivery verification through the precompiles, signed intents under LC-13, and the per-package-and-version claim-set state layout; then the chain, settlement, and entitlement-state adapters.
- **Daemon skeleton and identity**: jobs, configuration, settings, resolver, health, telemetry, IPC; custody and identity early because the installer needs them.
- **Plaintext CAS, resolution orchestrator, and package host**, at which point an ordinary `npm install` is served at the registry's speed, registers its requests, and prefetches its ciphertext.
- **First Finder ingest**: fetch, verify or record the attestation, generate the parameter set, encrypt per group, build the sidecars, register, seed, grant itself the asset's first entitlement. This is the spine both publisher paths reuse.
- **Credential delivery and per-attempt authorization**, closing the read path: mint delivery through the relayer, holders fulfilling requests for requesters present or absent, local decryption under the attempt rule, interval-end destruction, completing a first run's independence.
- **The demonstrable milestone**: with the registry down, a second identity installs a closure against the swarm on a grant fulfilled in its absence, and the north star and the latency guardrail are observed on the dogfood population.
- **Onboarding shells and services**: the installation coordinator and platform adapters, the extension and npm bootstrap, the desktop application and CLI, the explicit publisher path, the claim verifier and escrow claim last since nothing else depends on it, the project seed host and site, and observability reconciliation; the relayer enters on the chain adapters and the demonstration harness runs beside the daemon grouping.
- **Transaction flow proof**, where transfer delivery is exercised above zero, then acceptance and release.

Within every node, the workplan's fixed element order applies: interface test, interface, interaction spec, mock, guard test, guard, unit tests, construction, implementation, provides, integration test, directionality, requirements, commit. Nodes are authored bottom-up in dependency order.

# Risk Mitigation

| Risk | Mitigation in the design |
| --- | --- |
| Parameters fixed before they are measured | The harness is the first cryptographic artifact built; piece-group size and curve are chosen from its output; the latency budget is declared before the acceptance run so the instrumentation can fail |
| A wrong cryptographic assumption or verifier form | Both curves implemented behind `IPairingAdapter`; Rust verifier cross-verified bit for bit against the deployed contract; external review before any priced deployment |
| A hostile manifest or sidecar exercising a credential | Record, hash-card, suite, bounds, and sidecar authenticated before any credential is touched; ordering proven with chain and custody spies |
| Incompatible adapter pairs discovered in operation | Capability declaration and fail-closed resolution before any side effect; the immutable suite fixes every cryptographic component together |
| Secret leakage through storage, logs, or crashes | Decrypt-capable material memory-only and zeroized at every transition; telemetry scanned; custody surfaces inspected before and after decryption and transfer |
| Installation leaving a machine half-configured or its toolchain silently rerouted | Explicit visible reversible consent; every mutation backed up; checkpointed rollback; idempotent reinstall; one repair operation |
| First Finder disappearing | Escrow suites use the asset identity scope so any holder authors grants; a claim never needs the First Finder; handover is an optional shortcut |
| A registration race producing two ciphertexts | State-locked escrow registration; the loser destroys its ciphertext, sidecar, master scalar, and derivatives and follows the winner; the plaintext root proves equivalence |
| Settlement reversal | Reads at the declared tier, destruction at `HARD`; the exposure is one entitlement's credential, bounded and self-healing; the tier declared per deployment |
| Free-path abuse | Per-identity rate limits, short published lock expiries with refund, multi-node state views failing closed on inconsistency, on-chain verifier-key rotation and revocation |
| A single RPC node censoring reads | The per-attempt state view is a quorum of two of three configured nodes at a common reference; one unreachable or stale node does not deny, and fewer than two lying in concert cannot forge a view |
| The relayer, seed host, or verifier becoming authoritative | Each is behind an interface, none is on a client's critical path, and the specification names them as scaffolding to be replaced |
| Modified clients retaining credentials; common piece-group keys | Accepted residuals stated in the specification; piece-group size bounds exposure; leaked credentials name their entitlement; the license makes a non-conforming client a violation; the decapsulation-to-cipher seam is preserved for a future multi-key suite |
| Load profile unobserved for streaming content | Decapsulation time per group and state-read cost recorded so a later content class has a baseline |
| Scope creep into deferred items | Deferred items are architecturally protected and listed in MVP Scope; none is a hidden dependency of any MVP feature |

# Decisions and Open Questions

Decisions on the MVP path are stated; items held behind a policy line or released by a later design answer are summarized; the parameters the harness measures carry a Feedback block, and a blank block means the build proceeds on the stated assumption.

## Decided on the MVP path

**Launch network and pairing curve.** Base, with Base Sepolia as the test network; BLS12-381 primary through EIP-2537 and BN254 retained; tiers mapped to Base; delivery cost measured as L2 execution and L1 data fee.

**Default key custody.** A local keystore under the OS credential store as the first `IKeyCustodyAdapter`, keys derived from a holder seed, device roles as versioned custody capabilities with only the full role in the MVP, external wallets signing only, paired local transfer as the multi-device mechanism.

**Escrow record.** No maintainer commitment is stored and no salt custodian exists; the verifier establishes the claim set from upstream metadata at verification time; a commitment is reopened only with a ZK-Email verifier.

**Distribution and client license.** The build proceeds; the license text is a release prerequisite; the content-terms field is a string supplied at publication.

**Scope statement of the authorization invariants.** Left to the specification's next revision.

**Build sequence.** The order under Sequencing is ratified.

**Identity's chain-level form and sponsorship.** Chosen at the relayer milestone; LC-13's signed-intent rule keeps the contracts agnostic so neither answer is precluded.

## Measured by the harness

**Attempt-rule parameters and piece-group size.** Chosen from harness measurement against the declared latency budget.

Feedback:

## Held behind a policy line

**Ingest adapter eligibility.** Adapters target content its rights holder distributes to the public at no charge, npm availability being the MVP's test; the MVP ingests whatever npm serves, and a license check at ingest is a later policy decision. An adapter aimed at licensed content waits on the post-claim pricing paradox; one aimed at private or internal content waits on consumption privacy. Crossing either is a recorded policy decision, not an engineering convenience. No MVP build impact.

**Intentional identity fragmentation.** One operator splitting into many identities to dilute the retention obligation; waits on the stake layer. No MVP build impact.

## Released by a later design answer, outside the MVP build

**Variant seed authorship**, **the post-claim pricing paradox**, and **consumption privacy for private content** each block a V2 capability, are recorded in the workplan with what resolving them would take, and touch no MVP feature.

## Released with the stake and token layer or a later content class

Seeder compensation, retention parameters, paid monetization, retention enforcement, the first-party component class, collusion parameter selection, per-entitlement variance, media and streaming, content flagging, partial encryption, composable containers, Git commit wrapping, the escrow email bot, and the version alignment engine. All are deferred and architecturally protected in MVP Scope.

# Additional Content

**Relationship to the other planning documents.** The [revised business case](business-case-revised.md) and the [dependency map](dependency-map.md) address this sequence as groupings by role. The [feature spec](feature-spec.md) maps each module family here to a feature with acceptance criteria, and the [success metrics](success-metrics.md) define what the harness and the acceptance run must produce.

**Language, runtime, and proof.** The Application Requirements' Implementation Language and Runtime Policy and Normative Language and Proof Model govern every deliverable here.

**What is not decided here.** This document proposes nothing beyond what the specifications and workplan already carry. Where it states an assumption under a Feedback block, that assumption is the builder's default in the absence of an answer and is not a decision.
