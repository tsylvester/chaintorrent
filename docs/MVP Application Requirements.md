# MVP Application Requirements

## Purpose

This document translates [MVP Scope](MVP%20Scope.md) into the application capabilities required to deliver it. It defines the deployable applications, internal modules, external integrations, installation behavior, and observable proofs that together constitute the MVP application environment.

The cryptographic and protocol rules remain authoritative in [cryptography.md](cryptography.md). [MVP Scope](MVP%20Scope.md) remains authoritative for what the MVP includes and defers. This document is authoritative for how that scope is divided into applications and composable implementation modules. If an application requirement conflicts with either upstream document, the conflict must be resolved there rather than silently implemented here.

The intended adopter is an individual JavaScript developer. The normal entry points are installing the ChainTorrent Visual Studio Code extension and running `npm install chaintorrent`. After either path completes, and after any unavoidable operating-system or custody consent, the developer must have a working system rather than a list of components to install manually.

## Normative Language and Proof Model

The terms **MUST**, **MUST NOT**, **SHOULD**, and **MAY** are normative.

Every requirement in Self-Installation Requirements, every subsection in Composable Module Catalogue, and every MVP Acceptance Scenario is simultaneously:

1. an operational requirement for the application; and
2. a high-level integration or end-to-end test obligation.

Passing isolated unit tests does not satisfy a requirement whose proof column crosses a process, adapter, storage, network, custody, or chain boundary. The release evidence must exercise the named boundary using the packaged application path.

Requirement identifiers are stable references. They describe behavior, not filenames or a fixed crate layout. A later workplan may split or combine implementation units without changing a requirement, provided the same boundary and proof remain observable.

## Implementation Language and Runtime Policy

All deployable ChainTorrent applications MUST be implemented in Rust. A deployable application with a graphical user interface MUST use Tauri for that interface. Headless services and command-line applications MUST use Rust without adding Tauri solely for consistency.

Shared protocol rules, application workflows, cryptography, storage, networking, provisioning, and adapter contracts MUST be Rust libraries so that the CLI, desktop application, local daemon, and test harness execute the same behavior rather than reimplementing it.

TypeScript or JavaScript is permitted only where an external host imposes it:

- the Visual Studio Code extension uses TypeScript because the extension host requires a JavaScript-compatible extension;
- the npm bootstrap package may contain the minimum JavaScript or TypeScript required by npm lifecycle and package execution conventions;
- generated TypeScript bindings may exist when a host API requires them; and
- EVM contracts use Solidity because the target execution environment requires contract bytecode rather than a Rust application.

The TypeScript and JavaScript surfaces MUST remain thin host shells. They MUST NOT own protocol rules, cryptographic decisions, entitlement logic, package resolution, or workflow state. Any proposed additional TypeScript module must identify the external host constraint that prevents its implementation in Rust.

## Composition Boundary

The implementation is divided into three dependency rings. Dependencies point inward.

### Protocol and Domain

Pure Rust types and rules represent canonical identities, authenticated deployment suites and hash-cards, authorization contexts, entitlement state, custody state, manifest bounds, provisioning state, claims, and lifecycle transitions. This ring MUST NOT depend on Tauri, Visual Studio Code, npm, a particular chain SDK, a wallet product, a transport implementation, or a storage engine.

### Application Workflows

Rust use cases coordinate installation, package serving, cache resolution, encrypted acquisition, First Finder ingestion, authorization, decryption, seeding, publishing, claiming, transfer, repair, and recovery. A workflow depends on capability contracts and domain types, not on the identity of an adapter implementation.

Long-running operations MUST be represented as durable, restartable jobs. In particular, the initiating npm install MUST be allowed to complete after the upstream tarball is verified and committed to the plaintext CAS; First Finder encryption, threshold escrow, registration, and seeding continue asynchronously.

### Adapters and Host Shells

Rust adapters connect workflows to package-manager protocols, chains, wallets, key custody, ingest sources, swarm transports, discovery systems, seed hosts, provisioning participants, claim verifiers, local storage, and telemetry. Thin TypeScript or JavaScript shells connect Rust application surfaces to Visual Studio Code and npm where those hosts require it.

Every adapter MUST declare capabilities, version, and compatibility. Resolution MUST validate the complete composition before an operation begins and MUST fail closed before authorization traffic, decryption material, chain mutation, or swarm transfer occurs.

## Capability Disposition

“Existing” means an external system or standard exists for integration. It does not mean that a conforming ChainTorrent adapter, installer, or test already exists.

| Capability | Disposition for MVP | Application consequence |
| --- | --- | --- |
| npm dependency resolution, lockfiles, workspaces, and install behavior | Existing external system | Integrate through npm's registry protocol; do not replace the package manager. |
| Visual Studio Code extension host | Existing external system | Build a thin TypeScript onboarding and control shell over the Rust application interface. |
| Operating-system service managers, credential stores, firewall controls, and application directories | Existing platform facilities | Build signed installation, lifecycle, custody, and repair adapters per supported platform. |
| EVM-compatible chain, transaction settlement, and view calls | Existing external platform; launch chain unresolved | Build a chain adapter, contract bindings, settlement adapter, and entitlement-state adapter. |
| User wallets and signing providers | Existing external systems | Build wallet adapters and capability validation; provide a working default identity path. |
| npm registry and integrity metadata | Existing external system | Build the MVP ingest-source adapter and preserve upstream bytes and integrity attestations. |
| BitTorrent clients and protocol | Existing optional compatibility systems | Build transport, discovery, and delegated seed-host adapters without making them protocol requirements. |
| BLAKE3 and Bao, AES-256-CTR, Ed25519, secp256k1, and X25519 libraries | Existing cryptographic primitives | Build suite-constrained Rust wrappers, validation, domain separation, and lifecycle handling. |
| Tauri | Existing UI framework | Use it for ChainTorrent graphical applications; keep workflows in shared Rust libraries. |
| ChainTorrent protocol/domain model | New | Build the authoritative Rust types, validation rules, and state machines. |
| Package-serving and resolution orchestration | New | Build the local registry endpoint, CAS orchestration, encrypted acquisition, and fallback workflow. |
| First Finder lifecycle | New | Build immediate foreground serving plus durable asynchronous bootstrap. |
| Threshold custody and proactive resharing | New | Build real multi-participant custody; a single-node simulation does not satisfy the MVP. |
| Entitlement, registry, escrow, identity-binding, and claim contracts | New | Build and deploy the contract suite plus application adapters. |
| Provisioning, claim verification, relaying, and application observability services | New | Build Rust deployables and their operational controls. |
| Recipient- and window-bound decryption credentials, general monetization, seeder compensation, and other V2+ items | Deferred | Preserve the suite and adapter boundaries described by MVP Scope; do not implement them as hidden MVP dependencies. |
| Default key-custody implementation, launch chain, custody recovery UX, and published window values | Unresolved blocking selections | Resolve through the MVP workplan and prove the chosen composition before release. |

## Deployable Applications

Logical applications MAY share a packaged Rust binary when doing so simplifies installation, but their module contracts and independent lifecycle responsibilities MUST remain testable.

| Application | Language/runtime | Responsibility |
| --- | --- | --- |
| Visual Studio Code extension | Thin TypeScript extension host backed by Rust | Mandatory primary onboarding, status, consent, adapter configuration, health, repair, and links to richer controls. |
| npm bootstrap package | Minimal JavaScript/TypeScript host shim backed by signed Rust artifacts | Makes `npm install chaintorrent` a complete alternative onboarding path and starts or invokes the same installer used by the extension. |
| ChainTorrent desktop application | Rust with Tauri | Graphical configuration, identity and custody workflows, adapter selection, daemon and job status, diagnostics, repair, and recovery. |
| ChainTorrent CLI | Rust | Scriptable consent, configuration, diagnostics, package-host control, identity operations, jobs, repair, and uninstall. It does not replace npm's resolver. |
| Local daemon and package host | Rust service | Machine-level single instance providing local registry endpoints, CAS orchestration, encrypted acquisition, decryption, durable jobs, seed-host control, and local IPC/API. |
| Provisioning participant | Rust service | Holds threshold shares, validates authorization, agrees settlement references, provisions authenticated envelopes, and performs proactive resharing. |
| Relayer or paymaster | Rust service plus chain-specific contract support where required | Sponsors the free path and initial identity binding, enforces policy, and reports cost. |
| Claim verifier | Rust service | Implements the MVP publisher-proof flow and emits contract-verifiable, claimant-bound results. |
| Contract suite | Solidity for an EVM launch adapter | Implements canonical registry, authenticated deployment records, entitlements, issuance and transfer, escrow, identity binding, authority transfer, and claim settlement. |
| MVP demonstration harness | Rust | Creates controlled participants, services, wallets, chain state, and failures so packaged end-to-end scenarios are reproducible. |

## Self-Installation Requirements

Both supported entry points MUST converge on one Rust installation coordinator and the same post-installation state. Neither path may be a reduced demonstration path.

| ID | Operational requirement | Required integration or end-to-end proof | Test class |
| --- | --- | --- | --- |
| SI-01 | Installing the Visual Studio Code extension MUST discover or install the complete supported local environment. | On a clean supported machine, install the published extension, grant only unavoidable consent, then install a supported package through ChainTorrent. | End-to-end |
| SI-02 | Running `npm install chaintorrent` MUST discover or install the same complete environment without requiring Visual Studio Code. | On a clean supported machine, run the npm command and complete the same package-install scenario as SI-01. | End-to-end |
| SI-03 | The bootstrapper MUST detect the operating system, architecture, service capabilities, package managers, and available custody and wallet integrations. | Feed supported and unsupported platform combinations and verify deterministic adapter selection or an actionable fail-closed result. | Integration |
| SI-04 | Downloaded native artifacts and application updates MUST be authenticated before execution. | Tamper with an artifact and signature metadata and verify installation halts without executing or replacing the working version. | Security integration |
| SI-05 | Installation MUST create and permission the configuration store, durable job store, plaintext CAS, owned ciphertext store, IPC endpoint, and logs/metrics store. | Install as an ordinary user and verify each store is usable only by its intended principal and survives restart. | Platform integration |
| SI-06 | Installation MUST install, start, and enable the local daemon across reboot, or use an equivalently persistent supported user service when elevation is unavailable. | Reboot or restart the service environment and prove the package host and durable jobs recover without opening an editor. | End-to-end |
| SI-07 | Installation MUST detect supported package managers and offer an explicit, visible, reversible redirect to the local package host. | Accept and reject consent in separate runs; verify only accepted package-manager configuration changes and that restoration is exact. | End-to-end |
| SI-08 | Installation MUST back up every configuration it changes and MUST roll back partial changes if any later required step fails. | Inject a failure after each mutating installation stage and compare the resulting host configuration to the pre-install snapshot. | Failure integration |
| SI-09 | The default install MUST include an owned seed host and working discovery and transport implementations. Delegation to third-party software MAY be offered but MUST NOT be required. | Complete clean installation on a machine with no compatible third-party swarm client and seed then retrieve an object. | End-to-end |
| SI-10 | Installation MUST resolve a compatible package-host, ingest, chain, wallet, identity, custody, settlement, entitlement, provisioning, transport, discovery, and seed-host composition before reporting ready. | Substitute one incompatible capability declaration at a time and verify readiness fails before protected operations begin. | Composition integration |
| SI-11 | Installation MUST create or import a protocol identity, bind required keys, initialize the selected custody and recovery path, and configure the free relayer without exposing secret material. | Complete first run, restart every local process, and perform a signed, relayed identity-bound free acquisition. | End-to-end |
| SI-12 | Installation MUST configure usable default chain, provisioning, discovery, relay, and ingest endpoints, while allowing later replacement through capability resolution. | Remove all prior user configuration, install, and verify health checks and a complete free acquisition against the configured environment. | End-to-end |
| SI-13 | Readiness MUST be determined by an active end-to-end health probe, not by the presence of files or processes alone. | Return syntactically valid but nonfunctional adapter endpoints and verify the installer does not report ready. | Integration |
| SI-14 | Reinstallation MUST be idempotent and MUST preserve identities, custody state, plaintext CAS contents, ciphertext obligations, and durable jobs. | Run both onboarding paths repeatedly over populated state and compare identities, stored objects, jobs, and configuration. | End-to-end |
| SI-15 | An interrupted installation MUST resume safely or restore the previous working version without duplicating identities, services, or chain bindings. | Terminate installation at each durable checkpoint, restart it, and prove one coherent final installation. | Failure end-to-end |
| SI-16 | The CLI, Tauri application, and extension MUST expose one repair operation that diagnoses and restores missing binaries, services, permissions, configuration, and adapters without discarding user state. | Corrupt each owned installation component independently and verify repair restores service while preserving custody and stored content. | Recovery end-to-end |
| SI-17 | Updates MUST be signed, atomic, schema-aware, and reversible across application and configuration migrations. | Exercise forward update, interrupted update, incompatible schema, and rollback with active durable jobs and populated stores. | Upgrade end-to-end |
| SI-18 | Uninstall MUST restore package-manager configuration and stop/remove owned services. It MUST preserve identity/recovery material and plaintext content by default unless the user explicitly requests their removal. | Uninstall from a populated machine, verify npm works against its prior registry, and verify the stated preservation and explicit-removal choices. | End-to-end |
| SI-19 | User interaction MUST be limited to unavoidable operating-system permission or firewall approval, explicit package-manager redirect consent, custody user presence, recovery confirmation, and funding required for the paid dogfood proof. | Record a clean-install interaction trace and fail it for any unexplained manual download, account creation, endpoint entry, or component-install instruction. | UX end-to-end |

## Composable Module Catalogue

Each subsection defines modules that may be split into crates or processes later. The proof is part of the module requirement, not optional test guidance.

### Installation and Configuration

| ID | Operational requirement | Required integration proof |
| --- | --- | --- |
| IC-01 | A Rust bootstrap coordinator MUST execute a durable installation plan used by both onboarding shells. | Run both entry points against the same fixtures and compare their resulting capability graph and machine state. |
| IC-02 | Platform detection and signed-artifact installation MUST select only compatible binaries and MUST authenticate them before activation. | Cover every supported OS/architecture matrix member plus wrong-architecture, tampered, missing, and downgrade artifacts. |
| IC-03 | Service lifecycle management MUST install, start, stop, restart, enable, inspect, and remove the local daemon using supported platform facilities. | Exercise every lifecycle operation across process failure and host restart. |
| IC-04 | A versioned configuration registry MUST own defaults, user overrides, secret references, adapter capabilities, and endpoint configuration without storing raw custody secrets. | Round-trip configuration through CLI, Tauri, and extension surfaces and verify identical daemon behavior and redacted diagnostics. |
| IC-05 | Adapter resolution MUST validate required capabilities and immutable suite compatibility before activation. | Attempt valid and invalid compositions and prove invalid ones create no network, chain, authorization, or secret side effects. |
| IC-06 | Package-manager configuration MUST be explicit, backed up, reversible, and scoped according to the user's choice. | Configure and restore npm across user, project, workspace, proxy, and pre-existing custom-registry cases. |
| IC-07 | Migration, rollback, repair, and uninstall MUST be durable operations with checkpointed recovery. | Inject termination and storage errors at every checkpoint and prove recovery reaches the prior or new coherent state. |
| IC-08 | Health reporting MUST distinguish installed, configured, degraded, incompatible, and ready states and expose the same facts to all control surfaces. | Compare CLI, Tauri, extension, and daemon API results while adapters and services are independently failed. |

### Package Serving and Resolution

| ID | Operational requirement | Required integration proof |
| --- | --- | --- |
| PR-01 | The npm `IPackageHostAdapter` MUST serve npm-compatible metadata and tarball responses while leaving dependency resolution, peer dependency handling, workspaces, overrides, and lockfiles to npm. | Run representative npm projects through the local endpoint and compare resolved graphs and lockfile behavior with the upstream registry. |
| PR-02 | Package coordinates MUST resolve deterministically to the canonical identity hash and authenticated deployment records. | Resolve equivalent and malformed name/version requests across processes and verify identical identities or deterministic rejection. |
| PR-03 | The resolution orchestrator MUST use this order: local plaintext CAS, local encrypted store, remote encrypted swarm, then supported ingest source. | Populate one source at a time and assert the chosen path, forbidden fall-throughs, and resulting metrics. |
| PR-04 | The plaintext CAS MUST use verified content addressing and atomic commit; incomplete or mismatched artifacts MUST never become serveable. | Interrupt writes, race writers, corrupt bytes, and request concurrent reads while verifying only a complete validated tarball is served. |
| PR-05 | Cache policy MUST expose capacity, pinning, quota, eviction, hit rate, and cross-project reuse without deleting ciphertext held under a seed obligation. | Fill both stores independently, exercise eviction, and verify plaintext and ciphertext lifecycles remain separate. |
| PR-06 | A plaintext CAS hit MUST serve without swarm access, chain access, entitlement acquisition, or decryption authorization. | Disable all remote systems and install the same package into a second project from a populated plaintext CAS. |
| PR-07 | Local or remote ciphertext MUST enter the authenticated authorization and decryption workflow before plaintext is committed or served. | Attempt to serve encrypted content with absent, pending, denied, mismatched, and valid authorization. |
| PR-08 | If plaintext and encrypted sources miss, verified upstream bytes MUST be committed and served to the initiating npm request without waiting for First Finder bootstrap completion. | Stall encryption, chain, committee, and seeding stages after upstream verification and prove the initiating install still completes correctly. |

### Existing Encrypted Content Consumption

| ID | Operational requirement | Required integration proof |
| --- | --- | --- |
| EC-01 | The client MUST authenticate the deployment record and hash-card, resolve the complete immutable suite, and validate manifest bounds before entitlement or provisioning activity. | Mutate every authenticated field, suite capability, counter bound, piece count, and piece index and verify early fail-closed behavior. |
| EC-02 | Ciphertext acquisition MUST accept local and remote sources, verify Bao data against the authenticated ciphertext root, and share one object across compatible transport locators. | Corrupt chunks and locators across local, remote, and multi-homed retrievals and verify only authenticated bytes enter storage. |
| EC-03 | Entitlement acquisition and decryption authorization MUST be separate operations; an existing entitlement skips acquisition but never skips per-window authorization. | Repeat installs with and without an entitlement and assert chain mutations and authorization requests independently. |
| EC-04 | Authorization MUST carry the complete versioned context and obtain agreement on one exact qualifying settlement reference. | Omit or alter each context field and create conflicting participant references; every case must fail closed. |
| EC-05 | `AUTHORIZED`, `PENDING_SETTLEMENT`, and `DENIED` MUST lead respectively to provisioning, bounded wait/retry, and terminal refusal. | Drive all three adapter results through single and paginated batch evaluation without losing per-context bindings. |
| EC-06 | The client MUST verify the response envelope, aggregate commitment, suite, keyset, recipient binding, material type, context hash, and window bounds before accepting decryption material. | Substitute a valid field from another request, identity, deployment, keyset, or window and verify rejection. |
| EC-07 | Decryption MUST implement the suite-declared AES-256-CTR addressing rules, verify plaintext against the authenticated root, atomically commit to CAS, and serve the exact upstream tarball. | Decrypt random ranges and complete artifacts at counter boundaries, then compare upstream integrity and plaintext root. |
| EC-08 | Content keys, expanded cipher state, buffered keystream, and other transient decryption material MUST remain memory-only and be destroyed when `k` or `M` expires, whichever comes first. | Exercise both expiry boundaries, cancellation, crash recovery, and diagnostics while checking that no reusable material is persisted. |

### First Finder Bootstrap

| ID | Operational requirement | Required integration proof |
| --- | --- | --- |
| FF-01 | On a local and remote miss, the NPM ingest adapter MUST fetch the immutable tarball as-is and validate the published integrity attestation before CAS commit. | Supply valid, mismatched, mutable, truncated, and unavailable upstream responses and verify only exact validated bytes are committed. |
| FF-02 | The foreground workflow MUST atomically commit and serve validated upstream plaintext to the initiating npm request before asynchronous bootstrap completes. | Hold every background dependency unavailable and verify foreground installation latency ends with the upstream response and CAS commit. |
| FF-03 | A durable idempotent job MUST continue deployment creation, encryption, threshold escrow, registration, and seeding after the initiating process or editor exits. | Terminate npm, Visual Studio Code, the CLI, and the daemon at each checkpoint and prove eventual single completion after restart. |
| FF-04 | Deployment creation MUST generate a unique deployment identifier, random content key and IV, encrypt as one continuous stream, construct authenticated commitments and hash-card data, and never derive the content key from public material. | Use deterministic test vectors around the cipher while statistically and structurally validating production randomness and deployment uniqueness. |
| FF-05 | State-locked escrow registration MUST produce one canonical winner when finders race. A loser MUST destroy its ciphertext, content key, derivatives, and buffered keystream, then resolve the winner. | Race independent finders and inspect chain state, stores, secret lifecycle, and final locator selection. |
| FF-06 | The winning First Finder MAY retain the content key only until threshold shares are durably acknowledged. It MUST then destroy every local copy and derivative before marking bootstrap durable. | Crash before, during, and after acknowledgements and prove recovery neither loses the only key nor leaves a First Finder copy after durable escrow. |
| FF-07 | Bootstrap completion MUST require authenticated deployment registration, durable threshold custody, and persistent seeding; partial success remains a recoverable job. | Independently fail chain, committee, transport, and seed storage and verify accurate state plus eventual recovery without duplicate deployments. |
| FF-08 | A later seeker MUST discover the canonical deployment, leech authenticated ciphertext, obtain rights and authorization, decrypt, and install without contacting NPM. | Disable NPM after FF-07 and complete installation from a clean second identity and machine profile. |

### Swarm, Discovery, and Storage

| ID | Operational requirement | Required integration proof |
| --- | --- | --- |
| SW-01 | `ISwarmTransportAdapter` implementations MUST transfer ciphertext by authenticated root; BitTorrent compatibility MUST be one implementation rather than a protocol dependency. | Retrieve the same deployment through the owned implementation and the BitTorrent compatibility adapter and verify identical ciphertext roots. |
| SW-02 | Multiple `IPeerDiscoveryAdapter` implementations MUST run concurrently, union and deduplicate results, and treat no discovery source as authoritative. | Return overlapping, conflicting, unavailable, and malicious discovery results and verify deterministic safe aggregation. |
| SW-03 | The owned seed host MUST run independently of editor and CLI sessions and resume persisted transfers and seeding after restart. | Close all clients and reboot or restart the host while another participant continues to discover and retrieve the object. |
| SW-04 | A delegated `ISeedHostAdapter` MUST translate ciphertext roots to external identifiers and verify external custody with random Bao authentication challenges rather than trusting reports. | Make a delegated client falsely report possession and verify the adapter detects the missing or corrupt object. |
| SW-05 | Ciphertext storage MUST be keyed by ciphertext root, shared across transports, separately budgeted from plaintext CAS, and distinguish voluntary from obligated holdings. | Multi-home one object, vary both capacity policies, and inspect one stored copy with preserved holding reason. |
| SW-06 | Transport and storage failures MUST leave resumable acquisition and seeding jobs without exposing unauthenticated partial data. | Interrupt peers, network, disk, and daemon independently and prove verified resume or safe restart. |

### Ledger, Contracts, and Settlement

| ID | Operational requirement | Required integration proof |
| --- | --- | --- |
| LC-01 | The contract suite MUST represent canonical identities, authenticated deployments and hash-cards, asset-bound entitlements, issuance, transfer, escrow, identity bindings, claims, and administrative authority. | Deploy the suite and exercise each state transition through the Rust chain adapter while asserting emitted events and view state. |
| LC-02 | Entitlements MUST remain bound to one asset across issuance and transfer; no serial or identifier may authorize another asset. | Attempt cross-asset identifier reuse and verify both contract and authorization evaluation reject it. |
| LC-03 | First Finder issuance MUST remain $0.00 and escrowed; only explicitly published project-owned dogfood assets may use the nonzero MVP path. | Attempt to price First Finder content and verify refusal, then complete priced primary issuance and secondary transfer for an explicit publisher. |
| LC-04 | Settlement adapters MUST expose the deployment's tier and a verifiable exact reference suitable for participant agreement. | Exercise included, pending, conflicting, reorganized, and insufficient-tier references through authorization. |
| LC-05 | Transfer MUST let the buyer authorize at the qualifying settlement reference, refuse the seller's next renewal, and leave only the seller's already-issued bounded window until `k` or `M`. | Transfer during active use and assert buyer, seller renewal, and both expiry boundaries end to end. |
| LC-06 | Batch and paginated chain reads MUST preserve every complete authorization context and MUST NOT broaden authorization. | Compare individual and batched evaluations across mixed assets, identities, settlements, and outcomes. |
| LC-07 | State-locked registration and claim settlement MUST be race safe, replay safe, and bound to expected contract, chain, claimant, asset, and expiry. | Submit reordered, replayed, front-run, wrong-chain, and concurrent transactions and verify the intended single outcome. |

### Identity, Wallet, and Custody

| ID | Operational requirement | Required integration proof |
| --- | --- | --- |
| IW-01 | Identity, chain signing, handshake signing, key agreement, custody, and recovery MUST be separate capabilities that can be bound to the same principal without requiring matching keys. | Compose supported distinct schemes, register their binding, and verify signatures and recipient wrapping across process restart. |
| IW-02 | The selected MVP `IKeyCustodyAdapter` MUST declare signing, key-agreement, external-key protection, multi-device, recovery, user-presence, and publisher-seed capabilities. | Resolve every supported and unsupported capability combination and verify fail-closed selection before identity creation. |
| IW-03 | First run MUST create or import a usable identity and complete the relayer-funded on-chain handshake-key binding once per identity. | Start from no identity, restart, reinstall, and install multiple packages while asserting one stable identity and one binding transaction. |
| IW-04 | Custody MUST protect identity keys and any publisher derivation seed but MUST NOT persist consumer content keys or conflate them with identity material. | Inspect supported storage surfaces and recovery exports before and after authorized decryption. |
| IW-05 | Recovery and multi-device use MUST preserve or explicitly rotate identity bindings according to declared capability; neither may silently create a different principal. | Recover on a second device profile and verify identity, binding, access, and user-visible rotation semantics. |
| IW-06 | Publisher and administrative authority rotation MUST not change content-key lineage, plaintext identity, or existing entitlement asset binding. | Rotate authority and verify existing deployment authorization plus successor publishing behavior. |

### Cryptographic Services

| ID | Operational requirement | Required integration proof |
| --- | --- | --- |
| CR-01 | The MVP payload cipher MUST implement AES-256-CTR with a random 64-bit IV, 64-bit block counter, one continuous deployment stream, and the address and extent rules in the cryptographic specification. | Run specification vectors, random-access ranges, piece boundaries, maximum-valid counters, and overflow rejection. |
| CR-02 | BLAKE3/Bao services MUST construct and verify plaintext and ciphertext commitments, streaming chunks, and random storage challenges against authenticated roots. | Corrupt roots, paths, chunks, lengths, and order across encryption, acquisition, and delegated storage. |
| CR-03 | Signature services MUST support Ed25519 for protocol layers and secp256k1 for the EVM chain adapter with explicit domain separation and complete-message binding. | Verify valid signatures and reject cross-domain, cross-layer, truncated-context, wrong-scheme, and replay substitutions. |
| CR-04 | X25519 key agreement and response-envelope services MUST bind the recipient, authorization context, suite, keyset, material type, and window. | Swap each bound value independently and verify authenticated unwrap fails. |
| CR-05 | Content-key generation MUST use a cryptographic random source for First Finder deployments; explicit publisher derivation MUST use the secret hierarchy and unique non-reusable deployment identifier defined by the specification. | Prove First Finder non-determinism and publisher reproducibility only with the correct secret hierarchy and deployment identifier. |
| CR-06 | Manifest and hash-card validation MUST authenticate all suite fields and reject malformed bounds before entitlement acquisition or provisioning. | Fuzz authenticated descriptors and assert ordering with chain and committee spies. |
| CR-07 | Secret lifecycle services MUST minimize copies, exclude secrets from logs and diagnostics, keep consumer material memory-only, and zeroize it at every required success, expiry, cancellation, loss, and error transition. | Instrument lifecycle boundaries and crash artifacts while exercising all terminal paths. |

### Threshold Provisioning

| ID | Operational requirement | Required integration proof |
| --- | --- | --- |
| TP-01 | The MVP MUST run a real multi-participant threshold committee. No one participant or ordinary client may reconstruct or persist a deployment content key. | Provision with threshold participants, attempt with each sub-threshold subset, and inspect participant persistence. |
| TP-02 | Share distribution MUST authenticate the deployment suite and keyset, produce durable acknowledgements, and make First Finder destruction safe. | Lose and restart participants during distribution and verify the First Finder retains then destroys material at the exact durable boundary. |
| TP-03 | Participants MUST validate the complete authorization context, authenticate requester bindings, agree one exact settlement reference, and evaluate current entitlement state. | Give participants incomplete contexts, conflicting references, inconsistent chain views, and unauthorized identities and verify no response. |
| TP-04 | An authorized quorum MUST produce a fresh non-selective response envelope bound to the requester's declared key-agreement public key and complete context. | Combine honest, dishonest, duplicated, stale, and unavailable participant responses and verify only a valid quorum succeeds. |
| TP-05 | Proactive resharing MUST rotate shares and membership without changing the deployment content key, losing availability, or allowing old and new sub-threshold shares to combine into authority. | Reshare under active provisioning load, participant loss, and adversarial old-share retention. |
| TP-06 | Committee bootstrap and growth MUST reach the configured real threshold from clean deployment and recover from participant failure. | Start the packaged MVP environment from no committee state, add participants, fail each role, and complete authorization after recovery. |
| TP-07 | On a successful escrow claim, operational share custody MUST continue uninterrupted while custodial, administrative, publishing, and issuance authority transfer to the claimant. The raw content key MUST NOT be delivered to the claimant. | Claim an escrowed deployment during active committee operation and verify pre/post-claim authorization, authority, shares, and successor deployment standing. |

### Publishing and Claims

| ID | Operational requirement | Required integration proof |
| --- | --- | --- |
| PC-01 | Explicit publishing MUST prove publisher authority, create a unique deployment, apply the explicit-publisher content-key hierarchy, register authenticated metadata, and seed the ciphertext. | Publish a project-owned package, reject invalid authority, and consume it through a second identity. |
| PC-02 | First Finder escrow MUST record source provenance, the upstream integrity attestation, and a salted publisher commitment without granting the finder publisher rights. | Inspect registry and entitlement state after ingest and attempt finder pricing, administration, issuance, and claim actions. |
| PC-03 | `IPublisherProofAdapter` and `IClaimVerifierAdapter` MUST bind proof results to claimant, package or claim set, contract, chain, nonce, and expiry. | Replay and redirect valid proofs across claimants, packages, contracts, chains, and time. |
| PC-04 | The provisioning layer MUST retain and release the claim salt only through an authorized claim workflow; loss of a separate portal MUST not be a protocol dependency. | Complete and reject claims while portal-like components are absent and while participants are below threshold. |
| PC-05 | A successful claim MUST transfer complete protocol-level ownership and authority, preserve uninterrupted provisioning custody, and allow claimant-issued successor deployments with the same plaintext root. | Claim a First Finder record, administer it, issue entitlements, continue provisioning, and publish then verify a successor deployment. |
| PC-06 | Claim-set and voucher handling MUST support one verified maintainer proving every covered escrowed record without exposing reusable raw proof material. | Claim multiple eligible versions, exclude an ineligible asset, replay the voucher, and inspect logs and storage. |

### Relaying and Observability

| ID | Operational requirement | Required integration proof |
| --- | --- | --- |
| RO-01 | The relayer or paymaster MUST sponsor free entitlement acquisition and the single initial identity-binding transaction, with abuse controls and explicit failure states. | Complete clean free onboarding and acquisition, exhaust or deny subsidy, and verify actionable recovery without false authorization. |
| RO-02 | Paid dogfood acquisition MUST require real buyer funding and MUST NOT be presented as covered by the free subsidy. | Attempt without funds, then fund the test identity and complete primary and secondary priced settlement. |
| RO-03 | Metrics MUST record CAS hit and cross-project reuse, install wall-clock, authorization volume and latency, swarm bytes, ingest bytes, relayer gas, job outcome, and adapter health without recording secrets. | Run every acceptance path, reconcile metrics with induced activity, and scan telemetry for sensitive fields. |
| RO-04 | Structured logs and traces MUST correlate one package request across package host, CAS, swarm, chain, provisioning, decryption, and background First Finder work. | Execute concurrent installs and reconstruct each path without confusing identities, assets, or jobs. |
| RO-05 | Health and diagnostics MUST expose degraded dependencies, compatibility decisions, durable job state, and recovery actions consistently through CLI, Tauri, extension, and machine-readable API. | Fail each dependency independently and compare all control surfaces and repair recommendations. |
| RO-06 | Authorization latency MUST be measured against a declared pre-run interactive latency budget for the local developer population. | Configure the budget, inject latency below and above it, and verify the result is a pass or failure rather than an unqualified measurement. |

## Cross-Cutting Application Requirements

| ID | Operational requirement | Required integration proof |
| --- | --- | --- |
| XA-01 | All external inputs, authenticated descriptors, adapter responses, IPC requests, network messages, and chain events MUST be treated as untrusted until validated at their boundary. | Fuzz and substitute inputs at every adapter contract and verify bounded failure without protected side effects. |
| XA-02 | Local APIs MUST be authenticated and least-privileged; a project process MUST NOT gain custody, raw content keys, or administrative authority merely because it can request a package. | Call daemon surfaces from untrusted local users and project scripts and verify capability separation. |
| XA-03 | Durable workflows MUST be idempotent, observable, cancellable where safe, and recoverable after process or machine failure. | Terminate every workflow at persisted transitions and verify exactly-once effects or safe compensation. |
| XA-04 | Failure messages MUST identify the failed capability and safe recovery action without exposing secret material or suggesting insecure bypasses. | Exercise incompatible adapters, denied authorization, corrupt content, unavailable committee, chain failure, and custody failure through every UI. |
| XA-05 | The packaged MVP MUST run without a developer toolchain on supported user machines. | Install published artifacts on clean machines lacking Rust, Node development tools beyond npm, Solidity tooling, and repository source. |

## MVP Acceptance Scenarios

The MVP is releasable only when these scenarios pass through the packaged applications. Test-only calls that bypass onboarding, the local package host, capability resolution, or real threshold participants do not satisfy them.

| ID | Release scenario | Required observable outcome |
| --- | --- | --- |
| AS-01 | Visual Studio Code clean install | The extension produces a ready Rust environment and installs a supported npm package after only permitted consent. Proves SI-01, SI-03–SI-13, and SI-19. |
| AS-02 | npm clean install | `npm install chaintorrent` produces the same ready environment and completes the same package installation without Visual Studio Code. Proves SI-02–SI-13 and SI-19. |
| AS-03 | Idempotent reinstall | Repeating both entry points preserves identity, bindings, stores, jobs, and package-manager configuration. Proves SI-14. |
| AS-04 | Interrupted install and upgrade | Termination at every durable stage resumes or rolls back to one coherent signed installation. Proves SI-08, SI-15, and SI-17. |
| AS-05 | Repair and uninstall | Repair restores deliberately damaged components; uninstall restores prior npm configuration and honors state-preservation choices. Proves SI-16 and SI-18. |
| AS-06 | Plaintext cross-project hit | A second project installs from local plaintext with chain, provisioning, swarm, and NPM unavailable. Proves PR-05 and PR-06. |
| AS-07 | Local encrypted hit | Local ciphertext is authenticated, authorized, decrypted, verified, committed, and served; denial and expiry variants fail correctly. Proves PR-07 and EC-01–EC-08. |
| AS-08 | Remote encrypted hit | A clean client discovers, leeches, Bao-verifies, authorizes, decrypts, and installs an existing deployment. Proves EC-01–EC-08 and SW-01–SW-06. |
| AS-09 | NPM fallback is non-blocking on bootstrap | With plaintext and encrypted sources missing, npm supplies verified bytes and the initiating install completes while First Finder work is deliberately stalled. Proves PR-08 and FF-01–FF-03. |
| AS-10 | First Finder durable completion | The background job creates one deployment, establishes real threshold custody, destroys finder material at durable acknowledgement, registers, and persists seeding across restart. Proves FF-03–FF-07 and TP-01–TP-02. |
| AS-11 | Registry outage for ingested content | After AS-10, a second clean identity installs with NPM unavailable using only the canonical record, swarm, ledger, provisioning, and local application stack. Proves FF-08. |
| AS-12 | Concurrent First Finder race | Multiple finders ingest the same absent version; one wins and every loser cleans up protected state and follows the winner. Proves FF-05 and LC-07. |
| AS-13 | Threshold failure and resharing | Authorization succeeds at quorum, fails below quorum, rejects dishonest responses and reference disagreement, and continues across proactive resharing and member loss. Proves TP-01–TP-06. |
| AS-14 | Free entitlement lifecycle | A new identity is bound once, obtains a free asset-bound entitlement through the relayer, and performs repeated per-window authorization without additional binding transactions. Proves IW-01–IW-04, LC-01–LC-04, and RO-01. |
| AS-15 | Paid dogfood and transfer | A funded buyer acquires an explicitly published project-owned package above zero; secondary transfer gives the buyer immediate rights, refuses seller renewal, and respects the seller's existing window. Proves LC-03–LC-05 and RO-02. |
| AS-16 | Escrow claim and successor | A verified maintainer claims multiple eligible escrow records, gains complete authority without receiving raw content keys, provisioning continues, and a successor deployment shares the plaintext root. Proves TP-07 and PC-02–PC-06. |
| AS-17 | Adapter incompatibility | Every incompatible required capability combination is rejected during resolution before chain, swarm, authorization, or secret activity. Proves IC-05 and SI-10. |
| AS-18 | Daemon and job recovery | Closing editors and terminals and restarting the daemon or machine does not stop seeding or lose, duplicate, or corrupt durable work. Proves IC-03, FF-03, SW-03, and XA-03. |
| AS-19 | Decryption-window destruction | Both settlement-reference and volume expiry independently prevent further use of transient decryption material while retained plaintext remains reusable under the specified CAS rule. Proves EC-08 and LC-05. |
| AS-20 | Operational evidence | Metrics, traces, health, and diagnostics accurately describe every preceding scenario within the declared latency budget and reveal no protected material. Proves RO-03–RO-06 and XA-04. |

## Completion Boundary

The MVP application environment is complete only when all deployable applications required by the chosen composition are packaged, both onboarding paths reach the same working postcondition, every blocking selection has a compatible implementation, and all acceptance scenarios pass against real application boundaries.

An external product's existence, a mocked adapter, a single-process threshold simulation, a manually prepared developer machine, or a successful isolated crate test is useful development evidence but is not completion evidence for this document.
