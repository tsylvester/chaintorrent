<!-- Template: parenthesis_technical_requirements.md -->
# Index

Draft, 2026-09-23. The gating technical requirements for the ChainTorrent MVP, informing the master plan and milestones. New material in this document: the subsystem register, the API surfaces, the schemas, the proposed file tree with the Rust mapping of the workplan's node elements, the delta summary, and the iteration notes. Every other section states the position reached in an earlier planning document and cites it rather than repeating it.

- Executive Summary
- Subsystems
- APIs
- Database Schemas
- Proposed File Tree
- Architecture Overview
- Delta Summary
- Iteration Notes
- Feature Scope
- Feasibility Insights
- Non-Functional Alignment
- Outcome Alignment, North Star Metric, Primary KPIs, Guardrails, Measurement Plan
- Architecture Summary, Architecture, Services, Components, Data Flows, Interfaces, Integration Points, Dependency Resolution
- Security Measures, Observability Strategy, Scalability Plan, Resilience Strategy
- Frontend Stack, Backend Stack, Data Platform, DevOps Tooling, Security Tooling, Shared Libraries, Third Party Services

Sources: [cryptography.md](../research/cryptography.md), [MVP Scope](../research/MVP%20Scope.md), [MVP Application Requirements](../research/MVP%20Application%20Requirements.md), [MVP Execution Trace](../research/MVP%20Execution%20Trace.md), the [workplan](../workplans/current/ChainTorrent%20MVP.md), and the planning set: [product requirements](product-requirements.md), [system architecture](system-architecture.md), [technical approach](technical-approach.md), [tech stack](tech-stack.md), [dependency map](dependency-map.md), [feature spec](feature-spec.md), [success metrics](success-metrics.md), [risk register](risk-register.md), [non-functional review](non-functional-requirements.md), [feasibility assessment](technical-feasibility.md).

# Executive Summary

The MVP is a Rust protocol client and its supporting services. A single machine-level daemon serves npm's registry protocol to package managers from a plaintext cache, a peer swarm, and the upstream registry in that order; authorizes every decryption attempt from a fresh view of Base state; decrypts locally with a native credential delivered inside settlement; and seeds ciphertext it cannot read. A Solidity contract suite on Base holds canonical identities, deployments, parameter sets, envelope keys, entitlements with interval state, escrow, claims, and a delivery verifier over the BLS12-381 and BN254 precompiles. A relayer sponsors the free path, a claim verifier attests maintainers, and a project seed host and site carry the core closure and the education tier. Two onboarding shells, a desktop application, and a CLI attach to the daemon over authenticated IPC. A validation harness of thirty-two source files is built first and its delivery verifier is the shipped contract.

What must function, in one sentence per family: install on Windows, macOS, and Linux through either shell to one identical postcondition after only permitted consent; serve packages by the fixed resolution order with a verified atomic CAS; authenticate every descriptor before any credential is exercised and authorize every attempt from a multi-node state view; bootstrap absent packages in the background without blocking the initiating install and win or lose the registration race cleanly; move ciphertext by root across transports with non-authoritative discovery and a persistent seed host; deliver credentials at mint, grant, and sale with on-chain proof verification and atomic payment; hold identity under a custody adapter that derives keys from a holder seed and declares device roles; implement the cryptography exactly and measure its cost before fixing parameters; publish explicitly with dependency-closure ingestion and let maintainers claim escrowed records without the First Finder; sponsor free onboarding under rate limits; and observe every install without recording a secret. Completion is twenty-seven acceptance scenarios through packaged applications on clean machines.

# Subsystems

The [system architecture](system-architecture.md) breaks the daemon into twenty-one subsystems with responsibilities, ownership, and defining rows. This register lists every subsystem across all deployables, the ring it lives in, and the crate that will hold it under the proposed tree.

| Subsystem | Deployable | Ring | Crate | Defined by |
| --- | --- | --- | --- | --- |
| Domain types and guards | all | Protocol and domain | `domain` | Composition Boundary |
| Package host endpoint | daemon | Adapter | `adapters/package-host-npm` | PR-01, PR-02 |
| IPC server | daemon | Adapter | `adapters/ipc` | XA-02, RO-05 |
| Resolution orchestrator | daemon | Workflow | `workflows` | PR-03, PR-06, PR-07 |
| Plaintext CAS | daemon | Adapter | `adapters/cas` | PR-04, PR-05 |
| Deployment gate | daemon | Workflow | `workflows` | EC-01, CR-06 |
| Swarm engine: transports, discovery, seed host controller | daemon, seed host | Adapter | `adapters/transport`, `adapters/discovery`, `adapters/seed-host` | SW-01 to SW-07 |
| Ledger client: chain, settlement, entitlement-state | daemon, relayer, verifier, harness | Adapter | `adapters/chain` | LC-04, LC-06, XA-06, CD-04 |
| Identity manager | daemon | Workflow | `workflows` | IW-01, IW-03, IW-05, IW-07 |
| Custody adapter | daemon | Adapter | `adapters/custody` | IW-02, IW-04 |
| Entitlement acquisition | daemon | Workflow | `workflows` | EC-03, CD-01, CD-02, CD-05 |
| Credential engine and secret region | daemon | Workflow | `workflows` | EC-06, CR-07 |
| Attempt engine | daemon | Workflow | `workflows` | EC-04, EC-05 |
| Decryption pipeline | daemon | Workflow | `workflows` | EC-07 |
| Interval-end watcher | daemon | Workflow | `workflows` | EC-08 |
| First Finder engine | daemon | Workflow | `workflows` | FF-01 to FF-08 |
| Publisher engine and issuance policy | daemon | Workflow | `workflows` | PC-01, FF-09 |
| Grant service | daemon | Workflow | `workflows` | CD-05, CD-08 |
| Job engine | daemon, installer | Adapter | `adapters/jobs` | XA-03, FF-03 |
| Composition resolver | daemon, installer | Workflow | `workflows` | IC-05, SI-10 |
| Configuration registry | daemon, installer | Adapter | `adapters/config` | IC-04 |
| Settings service and catalogue | daemon, installer, every control surface | Workflow | `workflows/settings` | IC-04, IC-06, SI-05, SI-07, RO-05; this document, Settings |
| Health and diagnostics | daemon | Workflow | `workflows` | IC-08, RO-05 |
| Telemetry | all | Adapter | `adapters/telemetry` | RO-03, RO-04, RO-06 |
| Lifecycle and single instance | daemon, installer | Adapter | `adapters/platform` | IC-03, SI-06 |
| Pairing adapters | harness, daemon, contracts mirror | Adapter | `adapters/pairing` | CR-10 |
| Credential KEM | harness, daemon | Adapter | `adapters/kem` | CR-08, CD-08 |
| Envelope | harness, daemon | Adapter | `adapters/envelope` | CR-04 |
| Delivery proof | harness, daemon | Adapter | `adapters/proof` | CR-09 |
| Payload cipher | daemon, harness | Adapter | `adapters/cipher` | CR-01 |
| Hashing and Bao | daemon, harness, site demo | Adapter | `adapters/hashing` | CR-02 |
| Signatures | daemon | Adapter | `adapters/signature` | CR-03 |
| KDF and hash-to-scalar | harness, daemon | Adapter | `adapters/kdf` | CR-05, CR-08 |
| Ingest source | daemon | Adapter | `adapters/ingest-npm` | FF-01 |
| Publisher proof and claim verifier client | daemon | Adapter | `adapters/identity-proof` | PC-03 |
| Installation coordinator | installer | Workflow | `workflows` | SI-01 to SI-19, IC-01 |
| Relayer service | relayer | App | `apps/relayer` | RO-01, LC-10 |
| Claim verifier service | verifier | App | `apps/claim-verifier` | PC-04 to PC-07 |
| Validation harness | harness | App | `apps/harness-crypto` | CD-07 |
| Demonstration harness | test | App | `apps/harness-demo` | XA-07 |
| Contract suite | chain | Contracts | `contracts` | LC-01 to LC-10, CD-03 |
| Desktop application | desktop | Shell | `apps/desktop` | IC-08 |
| CLI | cli | Shell | `apps/cli` | SI-16 |
| Visual Studio Code extension | extension | Shell | `shells/vscode-extension` | SI-01 |
| npm bootstrap package | npm | Shell | `shells/npm-bootstrap` | SI-02 |
| Site and WebAssembly demonstration | site | Shell | `site`, `apps/wasm-demo` | MVP Scope, Project Seed Host and Site |

## Settings service and catalogue

Every value a user may reasonably want to change is a setting: declared once in a catalogue, stored in the configuration registry with its default, its user override, and its scope, validated by the composition resolver before it takes effect, backed up before it changes and restorable exactly, and exposed identically through the CLI, the desktop application, the extension, and the IPC. The installer sets initial values from the catalogue's defaults and offers the path and consent items during installation. The MVP ships few user-facing settings and the full mechanism, because adding a setting to a catalogue is cheap and retrofitting a settings function into a system that hard-coded its values is not. This is the pattern mature torrent clients use, and it is the pattern the tables below assume.

**Rules that apply to every setting.**

- **Nothing protocol-fixed is a setting.** A value the deployment's hash-card fixes, the piece-group size, the IV, `τ_soft`, `τ_wallet`, `minSettlementTier`, the suite, is read from the record and never from configuration. A client may apply a *stricter* local policy where the specification says a limit is conforming-client policy, never a looser one.
- **Scope is declared.** Machine, identity, or project. Machine settings need local presence and a control principal; identity settings follow the identity across devices where the custody adapter's sync carries them; project settings live with the package-manager redirect scope.
- **Paths are repointable.** Every store root is a setting with a platform default; changing one is a durable job that migrates or relinks contents, is checkpointed and reversible, and refuses to point two stores at one location. Nothing in this document is a fixed path; every path shown is the default.
- **Validation before effect.** A change is validated against the catalogue's constraints and, where it touches an adapter, against the composition resolver; an invalid value is refused with the reason and no side effect.
- **Restart requirements are declared.** A setting states whether it applies live, on the next job, or after daemon restart, and the surface says so.
- **Secrets are never settings.** A setting may reference custody; it never holds a key, a seed, or a credential.
- **Export and import** produce a versioned document without secrets, for backup and for a second machine, subject to the same validation on import.
- **Every change is traced**: surface, principal, old and new value, in the consent trace and diagnostics, so a rerouted toolchain is always explainable.

**Catalogue, by category.** Columns: setting, default, scope, applies, MVP exposure. "Exposed" means visible and editable in the MVP surfaces; "mechanism only" means declared and stored but shown only under advanced or diagnostics, so it exists to be exposed later.

| Category | Setting | Default | Scope | Applies | MVP |
| --- | --- | --- | --- | --- | --- |
| Paths | Configuration directory | Platform application-config directory | Machine | Restart | Exposed at install |
| Paths | Plaintext CAS root | Platform application-data directory, `cas` | Machine | Migration job | Exposed at install and settings |
| Paths | Ciphertext store root, owned mode | Platform application-data directory, `swarm` | Machine | Migration job | Exposed at install and settings |
| Paths | Job store, logs and metrics, temporary directory | Platform defaults under the data directory | Machine | Restart | Exposed |
| Storage | Plaintext CAS quota | Unbounded | Machine | Live | Exposed |
| Storage | Ciphertext store capacity | Unbounded | Machine | Live | Exposed |
| Storage | Pin linked artifacts; warn before evicting anything not re-authorizable | On; on | Machine | Live | Exposed |
| Storage | Voluntary holdings policy: keep own closure, keep everything fetched, keep nothing beyond obligation | Keep own closure | Identity | Live | Exposed |
| Seeding | Upload bandwidth limit; download bandwidth limit | Unlimited; unlimited | Machine | Live | Exposed |
| Seeding | Idle CPU limit; schedule windows; pause on battery; pause on metered connection | None; none; on; on | Machine | Live | Exposed |
| Seeding | Listening port and interface; port mapping | Random; all; on | Machine | Restart | Exposed |
| Seeding | Seed host mode: owned or delegated, and the delegated client's endpoint | Owned | Machine | Restart | Exposed |
| Seeding | Per-object seed or do not seed; obligated holdings visible and not evictable | Seed all held | Identity | Live | Exposed |
| Network | Transport adapters enabled | BitTorrent, the only MVP transport | Machine | Restart | Mechanism only until a second transport exists |
| Network | Discovery sources enabled: DHT, tracker, PEX, local, on-chain seeder map; tracker URLs | All; project tracker | Machine | Restart | Exposed |
| Network | Peer connection limits; encryption requirement for compatibility peers | Adapter defaults | Machine | Live | Mechanism only |
| Chain | Network profile: Base mainnet, Base Sepolia, local Anvil | Base mainnet | Machine | Restart | Exposed under advanced |
| Chain | RPC endpoints, at least two; archive endpoint for envelope recovery; prefer own node | Two providers; provider; off | Machine | Live | Exposed |
| Chain | Contract addresses per network profile | From the release's deployment record | Machine | Restart | Mechanism only |
| Chain | Relayer endpoint | Project relayer | Machine | Live | Exposed under advanced |
| Chain | Stricter local settlement policy: read no earlier than a tier above the deployment's declared tier | Off | Identity | Live | Mechanism only |
| Package host | Loopback port; redirect scope: user, project, workspace; package managers redirected; upstream registry URL for fallback; scoped or private registries passed through untouched | Random fixed at install; user; detected; npm; passthrough | Machine and project | Restart for port, live for scope | Exposed at install and settings |
| Package host | Upstream fallback allowed; First Finder bootstrap allowed | On; on | Identity | Live | Exposed |
| Identity | Custody adapter; device role; multi-device pairing | Local keystore; full; paired transfer | Machine | Restart | Exposed |
| Identity | External wallet provider: none, EIP-1193 provider, WalletConnect | None | Identity | Live | Exposed |
| Identity | Recovery method as the custody adapter declares | Adapter default | Identity | Live | Exposed |
| Publisher | Issuance policy: available for sale, price, limits, per-request approval | Not available | Identity | Live | Mechanism only; the dogfood publisher uses it |
| Publisher | Publisher seed custody and proof method preference | Custody adapter; strongest available | Identity | Live | Mechanism only |
| Privacy | Prefer own-node state reads; telemetry level: off, local only, export | Off; local only | Identity | Live | Exposed |
| Privacy | Show first-run disclosure again | — | Machine | — | Exposed |
| Updates | Update channel; automatic update; artifact verification policy | Stable; on; required | Machine | Live | Exposed |
| Consent | Account-link step | Unavailable in the MVP | Identity | — | Shown, not editable |
| Diagnostics | Log level; trace export target; metrics retention | Info; none; adapter default | Machine | Live | Exposed |
| Advanced | Attempt worker pool size; decryption pool size; job concurrency; backoff ceilings per adapter; IPC socket path | Adapter defaults | Machine | Restart | Mechanism only |
| Advanced | Latency budget declaration for instrumentation runs | Unset | Machine | Live | Exposed for the dogfood run |

Settings the specification forbids the client from offering: anything that skips the per-attempt state view, weakens a freshness bound below the deployment's, persists decrypt-capable material, disables destruction at HARD, or serves ciphertext-derived plaintext before verification. These are not "advanced"; they do not exist.

# APIs

Five API surfaces exist. The specification fixes the adapter interface signatures, reproduced in the system architecture's Interfaces section; this section covers the surfaces that cross a process or network boundary.

**Package host, npm registry protocol, served by the daemon on a loopback port.** `GET /{package}` returns the packument with versions and `dist.tarball` URLs pointing at the local host; `GET /{package}/-/{name}-{version}.tgz` returns the exact upstream bytes from the CAS after resolution; `GET /-/ping` for readiness. No write endpoints; publishing goes through the CLI, not the registry protocol. Responses carry the upstream integrity value so npm's own verification passes unchanged.

**Daemon IPC, authenticated, local-only.** Request classes and the capability each requires: `package.resolve`, `package.status`, any principal; `status.health`, `status.jobs`, `status.archive`, `status.footprint`, `status.diagnostics`, control principals; `job.cancel`, `job.repair`, control principals; `settings.schema` returning the catalogue with defaults, scopes, constraints, and restart requirements, any control principal; `settings.get`, control principals; `settings.set`, `settings.reset`, `settings.import` with validation and the migration job for path changes, control principals with local presence; `settings.export` without secrets, control principals; `identity.create`, `identity.import`, `identity.registerEnvelopeKeys`, `identity.connectWallet`, `identity.export` under local presence; `entitlement.acquire`, `entitlement.transfer` under local presence; `publish.package`, `claim.submit` under local presence; `admin.uninstall` under local presence. Every request carries a principal, a correlation identifier, and an idempotency key where it mutates. Framing is length-prefixed canonical binary messages, the same encoding used for anything hashed or signed, over a Unix domain socket on macOS and Linux and a named pipe on Windows, at a path under the configuration directory that is a machine-scoped setting. Authentication is by peer credentials where the platform supplies them, the connecting process's user identity read from the socket or pipe, mapped to a principal by the capability table; where peer credentials are unavailable, a per-installation token file readable only by the installing user is presented on connect. Control principals are the installing user's own processes; a project process spawned by a package manager is a package principal and nothing more. The consent trace records the surface and principal for every mutating call.

**Contract suite, Solidity on Base.** Registry: `registerAsset`, `registerDeployment` with the sidecar-per-live-set check, `registerParameterSet`, `retireParameterSet`, `resolve` and `resolveBatch` views. Envelope keys: `registerEnvelopeKeys(pk1, pk2, pop1, pop2, walletSig)`, rejecting identity elements. Entitlements: `mint(entitlement, recipient, envelope, mintProof)`, `lock(entitlement, seller, price, expiry)`, `deliver(entitlement, previousEnvelope, newEnvelope, transferProof)` which advances the interval, records keys and digest, emits the envelope, transfers, and releases payment, `withdrawLapsed(lock)`, `grant(entitlement, recipient, authorEnvelope, envelope, transferProof)` under the escrow suite. Authorization: `evaluateAuthorization(context)` and `evaluateAuthorizationBatch(contexts, page)` views returning the three-state result. Escrow and claims: `registerEscrow`, `submitClaim(claimSet, voucher)`, `transferAuthority`, `rotateVerifierKey`, `revokeVerifierKey`. Identity: `bindHandshakeKey(pubkey, scheme, anchorHash, walletSig)`. Every mutating function is bound to chain, contract, and expiry where a proof or voucher is involved. Exact ABI is authored at the contract nodes.

**Relayer service.** `POST /sponsor/binding` and `POST /sponsor/mint` accepting a signed user operation for the one binding and for a free mint, applying per-identity rate limits, returning the sponsored transaction hash or an explicit failure state; `GET /cost` reporting gas spent by category. No endpoint sponsors a paid acquisition.

**Claim verifier service.** `POST /claim/verify` accepting a claimant address, a package or claim set, and the proof inputs, running provenance-then-OAuth verification, and returning a voucher over the committed claim set, claimant, contract, chain, nonce, and expiry, signed under the on-chain registered key. `GET /key` returning the current attestation key.

**Harness output.** A release-evidence JSON document per curve: sizes of capsule, envelope, and proof; decapsulation time per group; proof generation and verification time; delivery cost as L2 execution gas and L1 data fee for a mint and a transfer; the chosen piece-group size and curve.

# Database Schemas

**On-chain state.** Fields per record; types are Solidity's.

- `Asset`: `identityHash bytes32`, `attestation bytes` or explicit absence flag, `authority address`, `proofClass uint8`, `entitlementAdapter address`, `issuanceAuthority address`, `liveParameterSets bytes32[]`, `deployments bytes32[]`.
- `Deployment`: `deploymentId bytes32`, `suiteId bytes32`, `cipherId uint8`, `counterLayout uint8`, `iv bytes8`, `pieceSize uint32`, `pieceGroupSize uint32`, `totalExtent uint64`, `sidecars (parameterSetId bytes32, root bytes32, locator bytes)[]`, `kdfId uint8`, `hashToScalarId uint8`, `encodingId uint8`, `statementVersion uint16`, `tauSoft uint32`, `tauWallet uint32`, `minSettlementTier uint8`, `ciphertextRoot bytes32`, `plaintextRootMode uint8`, `plaintextRoot bytes32`, `locators bytes[]`, `advisoryRoot bytes32`.
- `ParameterSet`: `id bytes32`, `asset bytes32`, `g1, u0, u1` first-group points, `g2, hpub` second-group points, `identityScope uint8`, `live bool`.
- `EnvelopeKeys`: `identity address`, `pk1` first-group point, `pk2` second-group point.
- `Entitlement`: ERC-721 token; `asset bytes32`, `parameterSet bytes32`, `interval uint64`, `holderKeys (pk1, pk2)`, `envelopeDigest bytes32`, `identityElement` first-group point.
- `Lock`: `entitlement uint256`, `buyer address`, `seller address`, `price uint256`, `expiry uint64`.
- `Escrow`: `asset bytes32`, `provenance bytes`, `parameterSet bytes32`; no maintainer commitment by the accepted default.
- `Binding`: `identity address`, `handshakeKey bytes32`, `scheme uint8`, `anchorHash bytes32`.
- `VerifierKey`: `key bytes32`, `activeFrom uint64`, `revokedAt uint64`.
- Events carry the full envelope and proof at every mint, grant, and transfer.

**Daemon local stores, `redb` tables.** Keys and values as canonical binary.

- `jobs`: key job id; value kind, state, checkpoint, idempotency key, created, updated, error.
- `resolutions`: key request correlation id; value path chosen, timings, outcome.
- `ciphertext_index`: key ciphertext root; value size, transports, locators, holding reason obligated or voluntary, sidecar roots held.
- `cas_index`: key content hash; value size, upstream integrity, linked projects, pinned, last used.
- `entitlements`: key entitlement id; value asset, parameter set, interval index, envelope digest, custody reference to the persistent credential; never the decrypted credential.
- `state_views`: key node id and reference; value tier, timestamp, result cache bounded by `τ_soft`.
- `settings`: key catalogue path; value schema version, default, user override, scope, constraint reference, restart requirement, last changed by surface and principal; secrets only as custody references.
- `health`: key subsystem; value state and last probe.
- `consent_trace`: key event id; value surface, principal, operation, shown-and-acted flags, timestamp.

**Plaintext CAS layout.** Under the configured CAS root, whose default is the platform application-data directory's `cas` and which the user repoints at install or in settings: `{cas_root}/{hash[0..2]}/{hash[2..4]}/{hash}/` holding the tarball and a manifest; `{cas_root}/tmp/` for uncommitted writes on the same volume so commit is a rename. Links from `node_modules` into the CAS entry; a root change is a migration job that relinks.

**Ciphertext store layout, owned mode.** Under the configured store root, default the platform application-data directory's `swarm`, repointable the same way: `{swarm_root}/{root[0..2]}/{root}/` holding pieces, the Bao outboard, and sidecars by parameter set. Under delegation the layout is the external client's, whose download directory is itself a setting of that client, and the index translates. The two roots may not coincide.

**Custody store.** Under the OS credential store, a wrapping key per identity; on disk, an encrypted blob holding the holder seed or the derived keys, the chain key if local, per-entitlement persistent credentials as envelope plus interval index, publisher seed if any, master scalars if any. Device role recorded per device. Never the decrypted credential or a piece-group key.

**Metrics schema.** One record per install with correlation id: resolution path, CAS hit, cross-project reuse count, wall-clock, authorization fraction, state-read count and latency, decapsulation time per group and group count, swarm bytes, ingest bytes, relayer gas, L2 execution gas and L1 data fee for any settlement, job outcomes, adapter health snapshot. No identifying data beyond what the ledger publishes.

**Harness sample deployment for the site demonstration.** A generated asset with a small plaintext, its ciphertext and sidecar, a parameter set, and one sample credential, bundled with the WebAssembly build.

# Proposed File Tree

The repository's node template names its elements with TypeScript suffixes, `[function].interface.ts`, `[function].mock.ts`, and so on, and the MVP is Rust. The mapping proposed here places each function in its own module directory with one file per element, which keeps one source file per node and one function per file as the dependency map assumed, and is flagged under Iteration Notes as a decision for the workplan author. Crate-per-adapter-family and a domain crate are the accepted default; paths are role names until nodes fix them.

```
ChainTorrent/
  Cargo.toml                          workspace
  rust-toolchain.toml
  deny.toml                           cargo-deny license allowlist
  crates/
    domain/                           protocol and domain ring; no host or chain dependency
      src/
        lib.rs
        parameter_set/                one function or type family per module
          interface.rs                types and signatures
          interface_test.rs
          interaction.spec.md         branch contract, declarative
          mock.rs                     builders, invalidators, function mocks
          guard.rs
          guard_test.rs
          test.rs
          provides.rs                 public surface
        credential/
        capsule/
        delivery_statement/
        attempt_context/
        hash_card/
        entitlement_state/
        custody_state/
        manifest_bounds/
        claim_set/
    workflows/                        application ring; depends on domain and adapter interfaces only
      src/
        resolve/  gate/  acquire/  attempt/  decrypt/  interval_end/
        first_finder/  publish/  grant/  identity/  compose/  health/
        settings/                     catalogue, validation, migration jobs, export and import
        install/                      installation coordinator plan; initial values from the catalogue
    adapters/
      pairing/        interface/  bn254/  bls12_381/
      kem/            interface/  setup/  issue/  rerandomize/  validity/  encapsulate/  well_formed/  decapsulate/
      envelope/       interface/  keygen/  wrap/  unwrap/
      proof/          interface/  challenge/  prove_mint/  prove_transfer/  verify/
      kdf/            hash_to_scalar/  piece_group_key/
      cipher/         aes_ctr/
      hashing/        blake3_root/  bao_verify/  bao_challenge/
      signature/      ed25519/  secp256k1/
      chain/          client/  settlement/  entitlement_state/  events/  verifier_client/
      ingest_npm/
      package_host_npm/
      transport/      bittorrent_rqbit/   adapter over pinned librqbit; translation, Bao post-verification, peer injection
      discovery/      dht/  tracker/  pex/  local/  seeder_map/  aggregate/
      seed_host/      owned/  delegated/
      cas/
      custody/        local_keystore/  holder_seed/  device_roles/
      identity_proof/ provenance/  maintainer_oauth/  verifier_client/
      jobs/
      config/         registry store; defaults, overrides, scopes, versions
      ipc/            server/  principals/  framing/
      telemetry/      metrics/  tracing/  redaction/
      platform/       service_linux/  service_macos/  service_windows/  credential_store/  install_paths/
  apps/
    daemon/           binary: package host, IPC, engines, workflows
    cli/
    desktop/          Tauri 2 project; src-tauri/ and a small TS UI
    installer/        installation coordinator entry; may be linked into daemon binary
    harness-crypto/   validation harness; emits release-evidence JSON
    harness-demo/     demonstration harness; participants, wallets, chain, faults
    relayer/
    claim-verifier/
    wasm-demo/        wasm-bindgen build of domain, hashing, pairing, kem, cipher for the site
  contracts/
    foundry.toml
    src/              Registry, ParameterSets, EnvelopeKeys, Entitlements, Locks, Escrow, Claims, Binding, VerifierKeys, PairingLib, DeliveryVerifier
    test/             mutation, replay, cross-entitlement, record-contradiction vectors
    script/           deploy to Anvil and Base Sepolia; addresses emitted to config
    generated/        constants and vectors generated from the Rust reference
  shells/
    vscode-extension/ TypeScript; IPC client; no protocol logic
    npm-bootstrap/    minimal JS bin invoking the signed installer; no postinstall
  site/
    static site; embeds apps/wasm-demo output and a harness-generated sample deployment
  docs/               unchanged
  .github/workflows/  three-platform matrix; clean-runner end-to-end; cargo-audit, cargo-deny, cargo-fuzz smoke
```

# Architecture Overview

Three inward-pointing rings; every external touchpoint an adapter with declared capabilities; composition validated at resolution and failing closed; an immutable deployment suite as the unit of cryptographic composition; no service on the read path; a single machine-level daemon with two ingress surfaces; a contract suite on Base verifying credential delivery through pairing precompiles; project services as scaffolding. The [system architecture](system-architecture.md) holds the context view, the daemon breakout, the credential-delivery sequence, and the device-role graph.

# Delta Summary

What changed across the planning cycle relative to the source documents as they stood at ledger log 36, in the order the changes were made.

- **Launch network**: from an unselected Ethereum network to Base, with BLS12-381 primary and BN254 retained, tiers mapped to Base, and delivery cost measured as L2 execution plus L1 data fee. Recorded in the product requirements; the workplan's launch-chain entry is stale and awaits revision in place.
- **Platform matrix**: Windows, macOS, and Linux, previously unlisted.
- **Ingest eligibility**: restated as free public distribution by the rights holder's choice with npm availability as the test and the residual named as redistribution and irrevocability rather than denied revenue; the critique's High rating re-rated Medium.
- **Custody**: local keystore under the OS credential store as the first adapter; keys derived from a holder seed; device roles full, read-delegate, and signer-only as versioned custody capabilities; paired local transfer as the MVP's declared multi-device mechanism.
- **Escrow salt**: decided; no maintainer commitment is stored and no custodian exists; the verifier establishes the claim set at verification time; reopened only with a ZK-Email verifier. MVP Scope, PC-02, and PC-04 amended and the workplan entry removed.
- **Publisher discretion**: an issuance policy served automatically, with per-request approval as a declared capability; added to MVP Scope under Paid Monetization.
- **Website tier**: the site carries the education tier with a WebAssembly demonstration built from the client's crates; the account, rendezvous, remote head, and hosted instance deferred with protection recorded in a new MVP Scope section, including what a pre-install account owns and never owns.
- **Chain migration**: identified as unspecified and recorded as a workplan To-Do entry.
- **Nine non-functional requirements** proposed and accepted for adoption into the Application Requirements.
- **Harness delivery verifier** established as production code consumed by the registry contract.
- **Delivery groupings** renamed by role; the business case's numbered phases superseded.
- **Factual corrections**: fifteen research requirements; precompile availability per network; one hundred fifteen operational requirements and twenty-seven scenarios.

# Iteration Notes

None outstanding.

# Feature Scope

Sixteen features in scope, each with objective, stories, acceptance criteria, dependencies, and metrics in the [feature spec](feature-spec.md); the deferred set, including the website account tier, recorded there and in MVP Scope with architectural protection; the travelling-developer story recorded as an anticipated capability with the list of MVP choices that must not block it.

# Feasibility Insights

Technical feasibility high; delivery feasibility undetermined until the harness phase calibrates throughput; platform breadth the largest delivery risk; the demonstration harness load-bearing under the completion boundary; the Solidity verifier generated from the Rust reference. Detail in the [feasibility assessment](technical-feasibility.md).

# Non-Functional Alignment

Sixty-seven extracted requirements with dispositions in the [non-functional review](non-functional-requirements.md); nine gaps accepted for adoption: signing-key custody, the package-host threat model, daemon footprint limits, cold-start time, backoff bounds, a per-release suite compatibility statement, regulatory review, export constraints, accessibility baselines, and the first-run disclosure.

# Outcome Alignment

Three outcomes: the benefit is felt, the lifecycle is proven, the economics are measurable. Per the [success metrics](success-metrics.md).

# North Star Metric

Plaintext CAS reuse: the fraction of installs served from local plaintext with no network, chain, swarm, or authorization work, and projects per machine sharing each artifact. First dogfood measurement is the baseline; no target stated.

# Primary KPIs

Interactive install wall-clock and authorization fraction against a budget declared before the run; clean-install success on every platform through both paths; registry-outage install success for the ingested set; all twenty-seven scenarios passing; cost per install; delivery cost as L2 execution and L1 data fee per mint and transfer; cross-project reuse.

# Guardrails

Latency budget pass; no protected material anywhere it should not be; no authorization without a current view and no accepted invalid delivery; no side effects from invalid compositions or untrusted input; package-manager parity; consent boundary and exact configuration restoration; free path rate-limited; obligated ciphertext never evicted; foreground install independent of bootstrap; no priced deployment before external review and legal review.

# Measurement Plan

Five stages: harness on both curves against Base Sepolia; budget declaration; acceptance run with reconciliation and telemetry scan; dogfood baseline; adopting population against baseline. Security proven as scenarios, reliability per platform, compliance as release-evidence items, review findings tracked to disposition.

# Architecture Summary

As stated in the [system architecture](system-architecture.md): one canonical ciphertext and sidecar per deployment on the swarm, entitlements on Base, credentials delivered inside settlement and verified by proof, per-attempt local authorization, and a daemon that serves npm's protocol from cache, swarm, and registry in that order.

# Architecture

Layers and the decentralization boundary, implementation rings, the context view, and the cryptographic construction, in the system architecture.

# Services

The daemon, relayer, claim verifier, seed host and site, validation harness, and demonstration harness, each with what it must never become; the deferred account, rendezvous, remote head, and hosted instance. In the system architecture.

# Components

Protocol and domain ring, workflow ring, the adapter table with MVP implementations and declared capabilities, the host shells, and the daemon breakout with twenty-one subsystems, trust boundaries, process model, and storage ownership. In the system architecture.

# Data Flows

The package-request state machine in the [Execution Trace](../research/MVP%20Execution%20Trace.md); credential delivery at a sale as a sequence diagram, First Finder bootstrap, and device roles, in the system architecture.

# Interfaces

The specification's fixed signatures for `ISettlementAdapter`, `ISignatureAdapter`, `IPairingAdapter`, `IKeyAgreementAdapter`, `ICredentialKemAdapter`, `IDeliveryProofAdapter`, and `IEntitlementStateAdapter`, reproduced in the system architecture, plus the repository's function contract: injected dependencies through a context slice, deps, params, payload, and a `Success | Error` return union, guards on entry, `unknown` only at a boundary. The APIs section above covers the process and network surfaces.

# Integration Points

Fourteen boundaries an integration test must cross, from verifier parity through the site demonstration, in the system architecture and the dependency map.

# Dependency Resolution

Declaration, composition, suite binding, fail closed, and the on-chain factory with governance under a single project-held key recorded as scaffolding. In the system architecture.

# Security Measures

Confidentiality under the named assumptions; delivery soundness and replay resistance; per-attempt authorization; no selective withholding; secret lifecycle; boundary validation; authenticated least-privilege IPC; authenticated artifacts; envelope keys never wallet keys; claims bound to the claimant; blast radius; stated residuals. In the system architecture.

# Observability Strategy

RO-03 metrics without secrets, per-request correlation, consistent health across surfaces, attempt latency against the declared budget, telemetry scanning as a release gate, `tracing` with OpenTelemetry export recommended. In the system architecture.

# Scalability Plan

Constant credential size, capsule cost independent of entitlements, unbounded issuance, sidecar overhead proportional to payload and unmeasured, constant on-chain state per interval, paginated batch reads, multi-homed deployments, independent store sizing, relayer cost under rate limits sized from measured gas, streaming out of scope with decapsulation time recorded as baseline. In the system architecture.

# Resilience Strategy

Durable idempotent jobs, daemon and seed host across reboot, checkpointed lifecycle operations, resumable transfers, multi-node views failing closed, no single operator on any path, destruction at HARD, envelope recovery from chain history, sequencer outages stalling writes only, swarm availability voluntary beyond the core closure. In the system architecture.

# Frontend Stack

Tauri 2 stable for the desktop; plain TypeScript with a small component library for the webview; TypeScript against the VS Code extension API; a `postinstall`-free npm bootstrap package; Rust with `clap` for the CLI; a static site with a `wasm-bindgen` build of the client crates for the demonstration. In the [tech stack](tech-stack.md).

# Backend Stack

Rust throughout; `tokio`; one workspace with a domain crate and a crate per adapter family; authenticated local IPC over Unix domain sockets and named pipes; `alloy` for the EVM; Foundry for Solidity; `serde` with canonical binary encoding; `tracing`; versioned TOML configuration. In the tech stack.

# Data Platform

`redb` for the job store and indexes; filesystem CAS with atomic rename and links; root-keyed ciphertext store with holding reason; custody under the OS credential store through `keyring` with a wrapping key and encrypted blob; multi-node Base RPC views; archive access for envelope recovery; local metrics store. In the tech stack.

# DevOps Tooling

`cargo-dist` packaging; `systemd`, `launchd`, and Windows Service or per-user scheduled task lifecycle; Apple and Authenticode signing plus Sigstore; GitHub Actions three-platform matrix with clean ephemeral runners; Anvil locally and Base Sepolia as the test chain; Foundry deployment scripts emitting addresses to configuration. In the tech stack.

# Security Tooling

`zeroize` and `subtle`; `cargo-audit` and `cargo-deny`; `cargo-fuzz` at every boundary; Foundry fuzz and invariant tests plus a static analyzer; telemetry scanning; redaction at the tracing layer; release signing keys under custody discipline. In the tech stack.

# Shared Libraries

`blake3` and `bao`; RustCrypto `aes` and `ctr`; `ed25519-dalek`; `k256` through `alloy`; arkworks `ark-bn254` and `ark-bls12-381` with `halo2curves` benchmarked; BLAKE3 keyed-mode KDF; `rand_core` with the OS RNG; DID Core types; OpenZeppelin ERC-721 and ERC-4337 EntryPoint v0.7 on the contract side. Versions and verification dates in the tech stack.

# Third Party Services

More than one Base RPC provider plus a project node; archive access; an ERC-4337 bundler and paymaster provider behind a relayer adapter; WalletConnect and EIP-1193 providers with Frame evaluated; npm metadata, provenance attestations, and GitHub OAuth for claims; Apple, Microsoft, and Sigstore signing; external BitTorrent clients under delegation; public DHT and an optional project tracker; no sign-in providers in the MVP. In the tech stack.
