<!-- Template: thesis_feature_spec.md -->
# ChainTorrent MVP Feature Specifications

Draft, 2026-09-23. One instance of the feature spec template per feature. Features are derived from the requirement families and acceptance scenarios of [MVP Application Requirements](../research/MVP%20Application%20Requirements.md), the boundary set by [MVP Scope](../research/MVP%20Scope.md), and the state machine in [MVP Execution Trace](../research/MVP%20Execution%20Trace.md). Acceptance criteria cite requirement identifiers (SI, IC, PR, EC, FF, SW, LC, IW, CR, CD, PC, LI, RO, XA) and release scenarios (AS) so that each criterion is traceable to a proof obligation. Terminology follows [cryptography.md, Terminology](../research/cryptography.md#terminology). This document adds no requirement; where a requirement is ambiguous the source documents govern.

**Personas used in the user stories.** *Developer*: an individual JavaScript developer at a terminal, the adopting population. *CI operator*: someone running unattended builds. *Participant*: any identity running the daemon, who seeds what it holds. *Publisher*: a rights holder registering their own asset explicitly; in the MVP, the project itself. *Maintainer*: the upstream owner of a package that a First Finder escrowed, arriving to claim it. *Buyer* and *seller*: identities on either side of an entitlement transfer. *Operator*: whoever runs the relayer, seed host, or claim verifier. *First Finder*: the automatic role a participant's daemon assumes when it ingests an asset absent from the ledger; it is software, not a person.

---

# Feature Name
Self-Installing Onboarding

## Feature Objective
Turn either supported entry point, the Visual Studio Code extension or `npx chaintorrent`, into a complete working environment on a clean machine with no manual component installation, converging on one Rust installation coordinator and the same post-installation state, with reversible package-manager redirect, durable daemon, idempotent reinstall, repair, signed update, and uninstall.

## User Stories
- As a developer, I install the extension or run one npm command and can install a package through ChainTorrent without downloading, configuring, or starting anything else.
- As a developer, I am asked for consent before my package manager is redirected, I can see what changed, and I can reverse it exactly.
- As a developer, when I reinstall or upgrade, my identity, cached packages, seeding obligations, and background jobs survive.
- As a developer, when something breaks I run one repair operation from the CLI, the desktop app, or the extension and service is restored without losing my state.
- As a developer, when I uninstall, npm works against its prior registry and my identity and plaintext are preserved unless I explicitly ask for their removal.
- As a CI operator, the daemon and its jobs survive reboot without an editor open.
- As a developer, I choose at install whether the daemon requests entitlements and prefetches ciphertext for my dependencies, I am told that a first run roughly doubles disk and bandwidth, and I can choose a cache-only mode with no chain identity if I want only the local cache and the registry.

## Acceptance Criteria
- On a clean supported machine either entry point yields a ready environment and a successful package install after only permitted consent (SI-01, SI-02, SI-19; AS-01, AS-02).
- Installation offers a cache-only mode that creates no chain identity, registers no request, and prefetches nothing; presents automatic requests and prefetch as consent items; and discloses the first-run disk and bandwidth cost, all recorded in the consent trace (SI-19, SI-20; AS-30).
- Platform, architecture, service capabilities, package managers, and custody and wallet integrations are detected and produce deterministic adapter selection or an actionable fail-closed result (SI-03).
- Every downloaded native artifact and update is authenticated before execution; tampering halts installation without replacing the working version (SI-04, IC-02).
- Configuration store, durable job store, plaintext CAS, ciphertext store, IPC endpoint, and logs are created with correct permissions and survive restart (SI-05).
- The daemon is installed, started, and enabled across reboot, or an equivalent persistent user service is used where elevation is unavailable (SI-06, IC-03; AS-18).
- Package-manager redirect is explicit, visible, reversible, backed up, and scoped to the user's choice across user, project, workspace, proxy, and custom-registry cases (SI-07, IC-06).
- Every configuration change is backed up and partial changes roll back on later failure; termination at every durable checkpoint resumes or restores one coherent installation (SI-08, SI-15, IC-07; AS-04).
- The default install includes an owned seed host and working discovery and transport; delegation to third-party software is optional (SI-09).
- Readiness is determined by an active end-to-end health probe and reported consistently across CLI, Tauri, extension, and daemon API (SI-13, IC-08).
- Reinstall is idempotent and preserves identities, custody, CAS contents, ciphertext obligations, and jobs (SI-14; AS-03).
- One repair operation restores missing binaries, services, permissions, configuration, and adapters without discarding user state (SI-16; AS-05).
- Updates are signed, atomic, schema-aware, and reversible with active jobs and populated stores (SI-17; AS-04).
- Uninstall restores package-manager configuration, removes owned services, and preserves identity and plaintext by default (SI-18; AS-05).

## Dependencies
- Adapter composition and capability resolution (SI-10, IC-05), because readiness requires a compatible composition.
- Settings and configuration (ST-07), because installation initializes from the catalogue's defaults and offers path and consent items.
- Identity, wallet, and custody (IW-03), because first run creates or imports an identity.
- Local daemon and package host, seed host, and relayer endpoints, because readiness probes them.
- Platform facilities: service managers, credential stores, firewall controls, application directories.

## Success Metrics
- Clean-install success rate on every supported OS and architecture, both entry points.
- Count of manual interactions per clean install, target equal to the permitted set only.
- Interrupted-install recovery rate at every durable checkpoint.
- Time from entry-point invocation to ready.

---

# Feature Name
Adapter Composition and Capability Resolution

## Feature Objective
Resolve every protocol layer against declared adapter capabilities rather than implementation identity, validate the complete composition before any operation begins, and fail closed before any credential is exercised, chain mutation sent, or swarm transfer started.

## User Stories
- As a developer, an incompatible configuration is refused at startup with a message naming the capability that failed, never discovered mid-install.
- As a publisher, I can swap a chain, curve, transport, custody, or seed-host implementation for another that declares the same capabilities without touching anything above it.
- As an operator, I can see the resolved capability graph and compare it across machines.

## Acceptance Criteria
- Every adapter declares capabilities, version, and compatibility (Composition Boundary).
- Installation resolves a compatible package-host, ingest, chain, pairing, credential-KEM, key-agreement, delivery-proof, wallet, identity, custody, settlement, entitlement, transport, discovery, and seed-host composition before reporting ready; substituting one incompatible declaration fails readiness before protected operations begin (SI-10; AS-17).
- Invalid compositions create no network, chain, authorization, or secret side effects (IC-05).
- The immutable deployment suite fixes the pairing adapter, credential-KEM adapter and live parameter sets, payload cipher and piece-group size, key-agreement adapter, delivery-proof adapter, KDF and hash-to-scalar mappings, delivery-statement version, attempt-rule parameters, and settlement tier; a client resolves only implementations declared compatible with every field, and a delivery-proof adapter only if it declares the chosen envelope algebra (MVP Scope, Adapter Composition).
- The credential KEM's identity scope is a declared capability fixed by the suite, entitlement scope for explicit-publisher deployments and asset scope for escrow deployments (CD-08).
- The configuration registry owns defaults, overrides, secret references, and adapter capabilities without storing raw custody secrets, and round-trips identically through every control surface (IC-04).

## Dependencies
- The protocol and domain ring's authenticated suite and hash-card types.
- Every adapter family, since each must declare before it can be resolved.

## Success Metrics
- Zero protected side effects observed across the invalid-composition matrix.
- Every incompatible pair in the compatibility matrix is refused at resolution, none in operation.

---

# Feature Name
Settings and Configuration

## Feature Objective
Declare every user-configurable value in one catalogue with its default, scope, constraints, apply mode, and exposure; store it in the configuration registry; validate every change before it takes effect; back up before and restore exactly after; make every store root a repointable default whose change is a checkpointed migration; and expose the same catalogue with identical behavior through the CLI, desktop application, extension, and IPC, so that the MVP ships few user-facing settings and the whole mechanism, and every later setting is a catalogue entry rather than a retrofit.

## User Stories
- As a developer, I choose where the plaintext cache and the ciphertext store live at install, and I can move either later without losing anything or breaking my projects.
- As a developer, I cap the daemon's upload bandwidth, pause seeding on battery or on a metered connection, set a schedule, and pick a listening port, and the daemon honours each immediately.
- As a developer, I set a quota on the plaintext cache and am warned before anything I could not re-authorize is evicted.
- As a developer, I change a setting in the desktop application, the CLI, or the extension and get the same result and the same validation from each.
- As a developer, I export my settings without any secret and import them on a second machine.
- As a developer, a change that would break the installation is refused with a reason rather than applied.
- As a developer, I can see which surface changed which setting and when.
- As a publisher, my issuance policy is a setting I configure once and the client serves automatically.
- As the project, no setting exists that weakens the protocol's guarantees, and the catalogue proves it.

## Acceptance Criteria
- The catalogue declares every configurable value with default, scope, constraints, apply mode, and exposure; nothing fixed by a hash-card or the specification is a setting; local policy may only be stricter (ST-01).
- Every store root is a repointable default; a change is a checkpointed reversible migration that moves contents and refuses coinciding roots; nothing links into a store (ST-02).
- Changes are validated against constraints and the composition resolver before effect; invalid values are refused with no side effect (ST-03).
- Every change is backed up and restorable; export contains no secret and import validates (ST-04).
- All surfaces expose the same catalogue identically; mutations require a control principal and local presence for machine scope; every change is in the consent trace (ST-05).
- Seeding controls are honoured as declared and obligated holdings remain non-evictable (ST-06).
- Installation initializes from defaults, offers path and consent items, and reinstall preserves overrides (ST-07).
- No setting skips the state view, weakens a freshness bound, persists decrypt-capable material, disables destruction at `HARD`, or serves unverified plaintext (ST-08).
- The settings round-trip and store migration scenario passes through every packaged surface (AS-28).

## Dependencies
- Configuration registry (IC-04) and package-manager configuration (IC-06).
- Composition resolver (IC-05), for validation of adapter-touching changes.
- Durable jobs (XA-03), for store migrations.
- The consent trace (SI-19).
- Custody, since a setting may reference custody and never hold a secret.
- The catalogue's contents are specified in the technical requirements' Settings section.

## Success Metrics
- Surface parity: identical daemon state after the same change from each surface, across the whole catalogue.
- Migration recovery at every checkpoint, with prior projects still installing from the CAS afterward.
- Zero secrets in any export.
- Zero forbidden settings reachable through any surface or the registry store.

---

# Feature Name
Package Serving and Local Content Addressable Storage

## Feature Objective
Serve npm's own registry protocol from a local package host backed by a machine-wide plaintext CAS, trying sources in order and hedged, local plaintext, local or swarm ciphertext under a held credential, then the upstream ingest source, so that a package fetched once is reused by every project on the machine, an install is never slower than npm alone by more than the hedge delay, and a first run leaves the machine independent of the registry and of every other participant for everything it installed.

## User Stories
- As a developer, a package I installed in one project installs into a second project from local plaintext with the registry, the chain, and the swarm all unreachable.
- As a developer, my first install of a project is served at npm's speed, and afterwards the same project installs with npm and every other participant unreachable.
- As a developer, I can see for each dependency whether I am independent of the registry for it or still waiting on a grant, and why.
- As a developer, npm resolves my dependency graph exactly as before; only where the bytes come from has changed.
- As a developer, I can see cache capacity, pins, quota, eviction, and hit rate, and I am warned before evicting anything I could not re-authorize.
- As a developer, `node_modules` is disposable; rebuilding it touches neither the network nor the ledger.

## Acceptance Criteria
- The npm package-host adapter serves compatible metadata and tarball responses while leaving resolution, peer dependencies, workspaces, overrides, and lockfiles to npm; representative projects produce the same resolved graphs and lockfile behavior as upstream (PR-01).
- Package coordinates resolve deterministically to the canonical identity hash `BLAKE3(packageName@version)` and authenticated deployment records; malformed requests are rejected deterministically (PR-02).
- Sources are tried in order, local plaintext, local or swarm ciphertext under a held credential, the ingest source, and swarm ciphertext under a grant obtained within a bounded wait only when the ingest source is unavailable, each hedged by starting the next in parallel after its configured delay and serving the first plaintext to verify, failing when exhausted with an actionable result that leaves the request pending; an identity without a credential is served by the ingest source while it is available; the path and its reason are recorded (PR-03; AS-08, AS-23, AS-24, AS-25).
- For every ledger-known asset the identity does not hold, a durable grant request is registered before or alongside the upstream fetch, batched per install into one sponsored operation, queued behind key registration and while the chain or relayer is unavailable, and fulfillable while the requester is offline; no request is made for held, absent, or dead-deployment assets, an explicit-publisher request goes to its issuance policy, and a priced asset becomes a notice (PR-10; AS-29).
- While a request is pending the deployment's ciphertext and sidecar are prefetched in the background under the bandwidth, metered, and battery settings and the ciphertext quota, pinned until the grant lands, seeded meanwhile, and matched to the granted set afterwards (PR-11; AS-29).
- Every control surface shows per-asset independence, plaintext, ciphertext, and credential held, or pending with its reason, and a project's independence as every lockfile asset being independent (PR-12; AS-29).
- The initiating install waits only for the source that served it, never for bootstrap, request registration, prefetch, or grant pickup (PR-08).
- The plaintext CAS uses verified content addressing and atomic commit; incomplete or mismatched artifacts are never served under interruption, racing writers, or corruption (PR-04).
- Cache policy exposes capacity, pinning, quota, eviction, hit rate, and cross-project reuse, never deletes ciphertext held under a seed obligation, and pins artifacts a live project resolved (PR-05; MVP Scope, Local CAS).
- A plaintext CAS hit serves without swarm, chain, entitlement acquisition, or decryption authorization (PR-06; AS-06).
- Reuse is at the tarball: the CAS holds verified tarballs, package managers extract per project, and nothing links into the store (PR-04; technical requirements, Plaintext CAS layout).
- Ciphertext, local or remote, enters the authorization and decryption workflow before plaintext is committed or served (PR-07).
- Package metadata is captured at every upstream resolution into an authenticated local store; tarball URLs stay on the canonical registry host so lockfiles are portable; the outage guarantee is a pinned closure, with range resolution against the ledger's version index marked best effort (PR-09; AS-11, AS-24).
- The local API is authenticated and least-privileged; the package host confers resolution and serving only; a project process gains no custody, credentials, keys, or authority by requesting a package or by running as the user, and sensitive operations complete only after an interactive confirmation no IPC message can supply (XA-02).

## Dependencies
- Encrypted content consumption, for the encrypted-source paths.
- First Finder bootstrap, for the upstream fallback.
- Self-installing onboarding, for the redirect that points npm at the host.

## Success Metrics
- Plaintext CAS hit rate and cross-project reuse count, the MVP's headline adoption metric (RO-03).
- Fraction of a closure independent at the end of its first run, and time to independence for the rest (RO-03; AS-29).
- Interactive install wall-clock against the pre-declared latency budget (RO-06).
- Lockfile and resolved-graph parity with upstream across the representative project set.

---

# Feature Name
Encrypted Content Consumption and Per-Attempt Authorization

## Feature Objective
Authenticate a deployment's record, hash-card, suite, manifest bounds, and header sidecar before any credential is exercised; acquire and Bao-verify ciphertext and sidecar; hold a fresh state view at the deployment's declared tier before every piece-group key derivation; decapsulate, decrypt, verify against the plaintext root, commit to CAS, and serve; and stop attempts and destroy decrypt-capable material when the entitlement transfers away.

## User Stories
- As a developer, a package that exists in the swarm installs from peers with my own credential and nothing requested from any service, whenever I already hold the credential and whenever the registry is unavailable.
- As a developer, an entitlement I have held for years still installs, and it never installs on a stale view.
- As a seller, once my transfer settles my client stops decrypting and destroys its keys, and a reorganized transfer never strands me while I still own the entitlement.
- As a developer, a malformed or hostile manifest halts before my credential is touched.

## Acceptance Criteria
- Deployment record, hash-card, complete immutable suite, manifest bounds, and header sidecar root are authenticated before any credential is exercised; mutating any authenticated field, capability, bound, count, group size, index, or capsule fails closed early (EC-01, CR-06).
- Ciphertext and sidecar acquisition accepts local and remote sources, verifies Bao data against authenticated roots, and shares one object across compatible transport locators; only authenticated bytes enter storage (EC-02).
- Entitlement acquisition and per-attempt authorization are separate; an existing entitlement skips acquisition and never skips the per-attempt view (EC-03).
- Every attempt holds a state view at the declared tier no older than `τ_soft` over the complete `AttemptContext`, and a wallet-control assertion no older than `τ_wallet` by a device key the identity's contract account admits in that view; omitting or aging any element, or asserting with a revoked device key, fails closed (EC-04).
- `AUTHORIZED`, `PENDING_SETTLEMENT`, and `DENIED` lead to decryption, bounded wait and retry, and terminal refusal respectively, in single and paginated batch evaluation without losing per-context bindings (EC-05, LC-06).
- The envelope is decrypted only under the identity's own registered envelope keys, the credential is verified against the parameter set's validity equation, and each capsule's well-formedness is verified before decapsulation; substituted envelopes, credentials, and capsules are rejected (EC-06).
- Decryption decapsulates each group's capsule, derives the set's wrapping key, unwraps the piece-group key, implements AES-256-CTR addressing, verifies plaintext against the authenticated root, commits atomically, and serves the exact upstream tarball across random ranges and boundaries (EC-07, CR-01).
- Decrypted credential, piece-group keys, cipher state, and keystream are memory-only; attempts stop on a transfer out observed at the declared tier and everything decrypt-capable is destroyed at `HARD`; the persistent credential is stored only under custody (EC-08, CR-07; AS-19).
- The state view is a quorum of two of three configured nodes agreeing at a common reference within the freshness bound; a stale or unreachable node does not deny, divergent state at one reference and an over-age view fail closed (XA-06).
- The local encrypted hit, the remote encrypted hit with the ingest source unavailable, and the interval-end scenarios pass with their denial, stale-view, and interval-end variants (AS-07, AS-08, AS-19).

## Dependencies
- Cryptographic services (CR-01 through CR-10).
- Credential delivery, for the credential that decrypts.
- Ledger, contracts, and settlement, for `evaluateAuthorization` and tier references.
- Swarm transport, discovery, and storage.

## Success Metrics
- State-read volume and latency per install (RO-03).
- Decapsulation time per piece group (RO-03).
- Attempt latency, state read plus decapsulation, as a pass or fail against the declared budget (RO-06).
- Zero persisted decrypt-capable material found in storage or crash artifacts across the lifecycle matrix.

---

# Feature Name
First Finder Bootstrap and Escrow

## Feature Objective
When an asset is absent from the ledger, fetch the exact upstream artifact, verify its integrity attestation, commit and serve it to the initiating install immediately, then as a durable background job generate a random master scalar and parameter set, encrypt per piece group, construct the sidecar and hash-card, win or lose a state-locked escrow registration, retain the master scalar as escrow custodian, and seed the ciphertext and sidecar, so that later seekers install without contacting the registry.

## User Stories
- As a developer, installing a package nobody has ingested before is no slower than installing from the registry; the swarm work happens after my install returns.
- As a developer, if I close my editor or my machine restarts mid-bootstrap, the job finishes once and exactly once.
- As a participant whose daemon lost a First Finder race, my copy is destroyed and I follow the winner automatically.
- As a later seeker, I install a package that was ingested by someone else while the registry is down.
- As a publisher, publishing my package puts its entire dependency closure in the swarm.

## Acceptance Criteria
- The npm ingest adapter fetches the immutable tarball as-is and validates the published integrity attestation before commit; mismatched, mutable, truncated, and unavailable responses commit nothing (FF-01).
- Verified upstream plaintext is atomically committed and served to the initiating request before background bootstrap completes, with every background dependency unavailable (FF-02, PR-08; AS-09).
- A durable idempotent job continues parameter-set creation, deployment creation, encryption, registration, and seeding after npm, the editor, the CLI, or the daemon exits, reaching single completion after restart (FF-03; AS-10, AS-18).
- Deployment creation generates a unique deployment identifier, a random master scalar and parameter set where none is live, a random piece-group key and random capsule randomness per piece group and a random IV, encrypts as one continuous stream keyed per group, constructs a sidecar per live set carrying that set's capsule and the wrapped piece-group key as its own object, commitments, and hash-card, and never derives a key from public material (FF-04, CR-05).
- State-locked escrow registration yields one canonical winner; a loser destroys ciphertext, sidecar, master scalar, derivatives, and keystream, then resolves the winner (FF-05, LC-07; AS-12).
- The winner retains the master scalar under custody as escrow custodian, authors credentials only for grants the escrow contract authorizes, and erases only after acknowledged handover (FF-06).
- Bootstrap completion requires authenticated parameter-set and deployment registration, sidecar publication, persistent seeding, and the finder's grant of the asset's first entitlement to itself; partial success is a recoverable job with no duplicate deployments, sets, or self-grants (FF-07).
- A later seeker discovers the canonical deployment, leeches ciphertext and sidecar, obtains an entitlement with a credential from any online holder, decrypts, and installs without contacting npm (FF-08; AS-11, AS-24).
- The escrow record carries source provenance, the upstream attestation, and the escrow parameter set, carries no maintainer commitment, and the finder gains no publisher rights (PC-02).
- Explicit publication ingests the dependency closure, bootstrapping absent dependencies and verifying present ones (FF-09; AS-22, AS-23).

## Dependencies
- Cryptographic services, for the KEM setup, cipher, and Bao commitments.
- Ledger, contracts, and settlement, for registration and the race.
- Swarm, discovery, and seed hosting.

## Success Metrics
- Foreground install latency ends with the upstream response and CAS commit, independent of background progress.
- Bootstrap completion rate after induced termination at every checkpoint, target one completion per asset.
- Ingest bytes per install and swarm bytes per install (RO-03).
- Registry-outage install success for ingested assets (AS-11, AS-24).

---

# Feature Name
Swarm Transport, Peer Discovery, and Seed Hosting

## Feature Objective
Move ciphertext and sidecars between participants by authenticated root through replaceable transport adapters, with BitTorrent compatibility as one implementation; discover peers through multiple concurrent non-authoritative discovery adapters; and seed persistently from a machine-level host that survives editor and terminal sessions, holding a ciphertext store keyed by root and separately budgeted from the plaintext CAS.

## User Stories
- As a participant, what I seed keeps seeding after I close my editor and after my machine restarts.
- As a participant, I can see my seeding archive, objects, sidecars, sizes, and whether each is obligated or voluntary, as my contribution to the swarm.
- As a participant who already runs a BitTorrent client, I can delegate seeding to it and the protocol verifies what it actually holds rather than trusting its report.
- As a developer, the same deployment is retrievable over more than one transport and it is one object, not two.

## Acceptance Criteria
- Transport adapters transfer ciphertext by authenticated root; the owned implementation and the BitTorrent compatibility adapter retrieve identical ciphertext roots (SW-01).
- Multiple discovery adapters run concurrently, union and deduplicate, and treat no source as authoritative; overlapping, conflicting, unavailable, and malicious results aggregate deterministically and safely (SW-02).
- The owned seed host runs independently of editor and CLI sessions and resumes transfers and seeding after restart (SW-03; AS-18).
- A delegated seed host translates roots to external identifiers and verifies custody with random Bao challenges; a false possession report is detected (SW-04).
- Ciphertext storage is keyed by ciphertext root, shared across transports, separately budgeted from plaintext, and distinguishes voluntary from obligated holdings (SW-05).
- Transport and storage failures leave resumable jobs without exposing unauthenticated partial data (SW-06).
- The seeding archive is visible through CLI, desktop, and extension and matches the seed host's verified contents (SW-07).
- The canonical record's transport locator set is contributed per serving adapter and a multi-homed deployment carries several locators as one object (MVP Scope, Swarm Transport).

## Dependencies
- Cryptographic services, for Bao verification and challenges.
- Self-installing onboarding, for daemon lifecycle.
- The ledger's transport locator set and on-chain seeder map.

## Success Metrics
- Seeding continuity across restart, measured as another participant's retrieval success during and after the host's restart.
- Detection rate of false possession reports under delegation.
- Swarm bytes served and received per identity.
- Ciphertext-root identity across transports, target identical.

---

# Feature Name
Entitlement Ledger, Contracts, and Settlement

## Feature Objective
Deploy a Solidity contract suite on the Ethereum launch network representing canonical identities, authenticated deployments and hash-cards, parameter sets and their live or retired state, envelope-key registration, asset-bound entitlements with interval counters and envelope digests, issuance and transfer with delivery-proof verification through the pairing precompiles, escrow, identity binding, claims, and administrative authority, exposed to clients through chain, settlement, and entitlement-state adapters.

## User Stories
- As a buyer, my payment is locked against the exact entitlement, seller, price, and expiry, released only when the seller's delivery proof verifies, and refunded if no valid delivery arrives.
- As a seller, I am paid in the same transaction that delivers the buyer's credential; I cannot be paid without delivering and the buyer cannot obtain the credential before locking payment.
- As a developer, my client reads authorization as a view call at whatever settlement tier the deployment declares, and a dependency closure of a thousand packages is one paginated traversal.
- As a publisher, I mint as many entitlements as I choose at whatever price I set, and nobody can price an asset they do not own.

## Acceptance Criteria
- The contract suite represents every state listed in the objective and each transition is exercised through the Rust chain adapter with asserted events and view state (LC-01).
- Entitlements remain bound to one asset; cross-asset identifier reuse is rejected by contract and by authorization evaluation (LC-02).
- First Finder issuance is $0.00 and escrowed; only explicitly published project-owned assets may use the nonzero path (LC-03).
- Settlement adapters expose the deployment's tier, the references at which the attempt rule reads and destroys, and reference age; included, pending, reorganized, stale, and insufficient-tier references drive the attempt rule correctly (LC-04).
- Transfer verifies the seller's delivery proof against the stored envelope digest, seller keys, and identity element, and in one transaction advances the interval counter, records the buyer's keys and digest, emits the envelope, transfers the entitlement, and releases payment; a lock with no valid delivery refunds at expiry (LC-05; AS-13, AS-15).
- Batch and paginated reads preserve every attempt context and never broaden authorization (LC-06).
- State-locked registration and claim settlement are race safe, replay safe, and bound to contract, chain, claimant, asset, and expiry (LC-07).
- Envelope-key registration requires a proof of possession per key under a wallet signature and rejects identity-element keys; mint rejects a trivial identity element (LC-08).
- Parameter sets are registered per asset, marked live at bootstrap and claim, retired only when no live entitlement remains, and a deployment record is rejected unless it carries a sidecar root for every live set; the asset's authority may add a sidecar for a set made live later (LC-09; AS-16).
- Payment locks carry a bounded published expiry; relayer-paid mints and zero-price grants are admitted under a global budget and per-identity limits (LC-10; AS-27).
- Ownership changes only through mint, grant, and delivery; the token's standard transfer and approval entry points are disabled and receiver callbacks run after state is final (LC-11; AS-27).
- The identity is a contract account whose signer set is its device registry; every function that mutates identity-bound state takes the acting identity and a signed intent verified through the account's ERC-1271 check against a signer whose permissions cover the call, so sponsored and self-funded callers share one entry point, and the contracts enforce lock expiry and no volume limit (LC-13).
- All three content roots and the plaintext-root disclosure mode are registered; public registries use Public mode (MVP Scope, Entitlement Ledger).
- The per-identity binding record stores the bound handshake key and scheme on-chain with an anchor hash to the DID Document (MVP Scope, Entitlement Ledger).

## Dependencies
- Cryptographic services, for the on-chain verifier through `IPairingAdapter` (CR-09, CR-10).
- Credential delivery, whose proofs the contract verifies.
- Base as the launch network, with BLS12-381 as the primary verifier form and BN254 retained.

## Success Metrics
- Delivery-verification gas for a mint and a transfer on the resolved curve (CD-07, RO-03).
- Relayer gas per install (RO-03).
- Zero accepted invalid proofs across the malformed, replayed, cross-entitlement, and record-contradicting sets.
- Refund rate for lapsed locks, target complete.

---

# Feature Name
Identity, Wallet, and Key Custody

## Feature Objective
Create or import a protocol identity on first run as a contract account, bind its handshake key on-chain once through a relayer-paid transaction, register its envelope key pair with proofs of possession, keep the holder seed on root devices while admitting every other device under its own key as a signer on the identity's contract account, and hold identity keys, envelope keys, issuance material, publisher seeds, and per-entitlement envelopes under a capability-declaring custody adapter that upgrades in place, with external wallets connecting through EIP-1193 or WalletConnect to sign only.

## User Stories
- As a developer, I get a working identity on first run with no wallet, no gas, and no account creation.
- As a developer with an external wallet, I connect it to sign and my envelope keys are never my wallet keys.
- As a developer adding a laptop, I pair it with my workstation and it reads my entitlements on its own, recovering envelopes from chain history, while my seed stays on the workstation.
- As a developer adding a phone, it approves operations for my identity without holding my seed or any envelope material.
- As a developer who loses a device, I revoke it from a root and it can decrypt nothing further once it next reads chain state.
- As a developer whose root is lost, I recover it on a fresh machine from my seed without becoming a different principal.
- As a developer, when the custody adapter is upgraded my identity is unchanged, and a failed upgrade leaves the old custody working.
- As a publisher, rotating my authority key changes nothing about my parameter sets, my deployments, or my holders' credentials.

## Acceptance Criteria
- Identity, chain signing, handshake signing, envelope keys, custody, and recovery are separate capabilities bindable to one principal without matching keys; distinct schemes compose and survive restart (IW-01).
- The custody adapter declares signing, envelope-key, issuance-material, external-key protection, multi-device, recovery, user-presence, and publisher-seed capabilities, and unsupported combinations fail closed before identity creation (IW-02).
- First run creates or imports an identity, completes the relayer-funded handshake-key binding once per identity, and registers envelope keys with proofs of possession before the first acquisition; restarts, reinstalls, and multiple installs yield one identity, one binding, one registration (IW-03; AS-14).
- Custody protects identity keys, envelope keys, master scalars, and derivation seeds, persists the envelope and interval index per entitlement, and never persists the decrypted credential or piece-group keys (IW-04).
- Recovery and multi-device use preserve or explicitly rotate bindings, never silently create a different principal, and recover a missing envelope from chain history (IW-05, CD-04).
- The holder seed stays on root devices; its control branch holds the handshake key and authority over binding, rotation, and the device set, and its envelope branch holds the envelope secrets. A new device generates its own key, pairs with a root, is admitted as a signer on the identity's contract account with per-device permissions, and receives a role delegation under the handshake key; a reader receives the envelope secrets, never the seed. A further root is added only by a deliberate seed transfer (IW-05).
- Every attempt's wallet-control assertion is made by a device key the contract account admits in the same state view as the authorization; a revoked device fails its next attempt and its conforming client purges its envelopes and envelope secrets (EC-04, EC-08).
- Custody blobs carry the adapter version, wrapping-key identifier, and derivation parameters; an upgrade rewraps beside the old blob, verifies the re-derived public keys against the on-chain binding and envelope-key registration, and only then retires the old blob, rolling back on mismatch (IW-02, IW-04).
- Publisher and administrative authority rotation leaves parameter sets, plaintext identity, and asset binding unchanged; existing credentials still decrypt (IW-06).
- External wallets connect through EIP-1193 or WalletConnect and only sign; registrations and bindings are EIP-712 typed data; the default identity is a local custody-held key; envelope keys are never wallet keys (IW-07).
- Signature services support Ed25519 for protocol layers and secp256k1 for the EVM chain layer with domain separation and complete-message binding (CR-03).

## Dependencies
- The local keystore under the OS credential store as the default `IKeyCustodyAdapter`.
- Relaying, for the binding transaction and device admission and revocation.
- Ledger contracts, for the binding record and envelope-key registry.

## Success Metrics
- One binding transaction and one envelope-key registration per identity across the restart and reinstall matrix.
- A reader device reading alone on a second device profile with the seed never leaving the root, and purging at its next state view after revocation.
- Root recovery on a fresh profile with correct identity and envelope recovery.
- Principal unchanged across an in-place custody upgrade.
- Zero decrypted credentials or piece-group keys found in any custody surface or recovery export.

---

# Feature Name
Cryptographic Services and Validation Harness

## Feature Objective
Implement the suite-constrained Rust cryptography, the AES-256-CTR payload cipher, BLAKE3/Bao commitments, signatures, the pairing adapter on BN254 and BLS12-381, the credential KEM, the pairing ElGamal envelope, and the Schnorr delivery proof, and ship a validation harness that measures their sizes, timings, and gas on each candidate curve against a deployed verifier so that the piece-group size is chosen from measurement and the curve confirmed before release.

## User Stories
- As the project, I know the byte, latency, and gas cost of every cryptographic step on each candidate curve before I fix the piece-group size or confirm the curve.
- As a developer, the cryptography I run is the one the specification defines, verified by test vectors, not an approximation.
- As an auditor, the Rust verifier and the deployed on-chain verifier agree bit for bit.

## Acceptance Criteria
- The payload cipher implements AES-256-CTR with a random 64-bit IV, 64-bit block counter, one continuous stream keyed per piece group, and the address and extent rules; specification vectors, boundaries, maximum counters, and overflow rejection pass (CR-01).
- BLAKE3/Bao services construct and verify plaintext, ciphertext, and sidecar commitments, streaming chunks, and random challenges; corruption of roots, paths, chunks, lengths, and order is detected (CR-02).
- Envelope services encrypt one credential to a recipient's two registered keys with independent coins, reject shared or identity secrets, and round-trip exactly (CR-04).
- The credential KEM implements setup, issue, rerandomize, validity, encapsulate, well-formedness, and decapsulate exactly as specified: every valid credential recovers the same value from every well-formed capsule, rerandomization needs no master scalar, credentials are non-convertible across entitlements, malformed capsules are rejected, trivial identity elements refused (CR-08).
- The delivery proof proves and verifies the mint and transfer relations over the full statement in both verifier forms, and the Rust verifier matches the deployed on-chain verifier bit for bit; each statement field and response mutation is rejected and replay across settlements fails (CR-09).
- The pairing adapter exposes source-group operations, subgroup checks, and the pairing-product check on both curves with encodings matching the target chain's precompiles (CR-10).
- Sidecar and derivation services derive each set's wrapping key, wrap and unwrap the piece-group key as specified, use BLAKE3 keyed derivation off chain and keccak256 under domain tags for the identity mapping and the challenge, and freeze contexts with known-answer vectors; two independently generated sets unwrap one piece-group key (CR-11).
- Manifest, hash-card, and sidecar validation authenticate all suite fields and capsules and reject malformed bounds before any credential is exercised, with ordering proven by chain and custody spies (CR-06).
- Secret lifecycle services minimize copies, exclude secrets from logs, keep decrypted credentials and keys memory-only, and zeroize at every transition (CR-07).
- The harness produces capsule, envelope, and proof sizes, decapsulation time per piece group, proof generation and verification time, and delivery-verification gas for a mint and a transfer on each candidate curve, and the piece-group size is chosen from them and recorded in release evidence (CD-07; AS-21).

## Dependencies
- Existing BLAKE3, Bao, AES, Ed25519, secp256k1, and pairing-curve libraries.
- A test-chain deployment of the contract verifier, for the gas and bit-for-bit measurements.
- None on other MVP features.

## Success Metrics
- Measured capsule, envelope, and proof bytes on each curve.
- Measured decapsulation time per piece group, proof generation and verification time.
- Measured delivery-verification gas per mint and per transfer.
- A recorded piece-group size and curve confirmation derived from those measurements.
- Vector pass rate, target complete, across every service.

---

# Feature Name
Credential Delivery at Mint, Grant, and Sale

## Feature Objective
Deliver each holder's native credential exactly once per ownership interval inside the settlement that creates the interval: the issuer authors it under its master scalar at mint, any current holder authors it from its own credential at an escrow grant, and the seller rerandomizes and re-encrypts its own at a sale, each posting an envelope and a proof the contract verifies before recording the interval.

## User Stories
- As a buyer, I receive a credential that decrypts every live deployment of the asset, delivered in the same transaction that transfers me the entitlement.
- As a seller, I deliver from a fresh decryption of my own envelope after a restart, with no in-memory state and no persisted offset.
- As a new identity, I obtain a free grant for an escrowed package from any online holder, including while I am offline, since my first run registered the request and my daemon picks the credential up from chain history; the First Finder's permanent absence stops nothing.
- As a holder, my daemon fulfils pending requests for the assets I hold as it seeds them, sponsored by the relayer, without my involvement.
- As a maintainer who claimed my package, my escrow-era holders keep working, my successor deployments carry a sidecar for every live set, and a holder can migrate to my parameter set voluntarily.

## Acceptance Criteria
- Mint delivers the first credential in the mint transaction with a mint proof verified before interval zero is recorded; malformed, replayed, and unregistered-recipient cases record nothing (CD-01; AS-14).
- Sale delivers the buyer's credential in the settlement from the seller's fresh decryption, rerandomized with a private offset, with a transfer proof against the previous envelope; the seller's persisted offset is never required (CD-02; AS-15).
- The on-chain verifier verifies mint and transfer proofs through the pairing adapter's precompile interface in the declared form and rejects any statement field the record contradicts (CD-03; AS-13).
- The persistent credential survives restart and is recoverable from chain history; lost envelope secrets end transfer but not reading while the decrypted credential is held (CD-04).
- Under an escrow suite any current holder authors the credential for a newly minted zero-price entitlement, the contract records it as interval zero, the conforming client serves pending grant requests automatically under the relayer's policy including for requesters that are offline, and the First Finder's absence affects nothing (CD-05; AS-26, AS-29).
- The registry holds durable, batched, withdrawable grant requests readable by holders, open until a grant settles and authorizing nothing by themselves (LC-12; AS-29).
- A grant fulfilled in the requester's absence is loaded on its next run from chain history, and the requester then confirms it holds a deployment carrying the granted set (PR-10, PR-11, CD-04; AS-29).
- Holder-assisted authorship is refused under the entitlement scope (CD-08; AS-26).
- Claim registers the claimant's parameter set, leaves escrow-era credentials working, allows optional encrypted handover with a compatibility proof, requires later bodies to carry a sidecar per live set, and supports voluntary replacement of escrow-era credentials (CD-06; AS-16).

## Dependencies
- Cryptographic services, for KEM, envelope, and proof.
- Ledger, contracts, and settlement, for verification and interval state.
- Identity, wallet, and custody, for registered envelope keys and the persistent credential.

## Success Metrics
- Proof generation and verification time and gas per mint and per transfer (RO-03).
- Grant fulfilment rate with the First Finder offline, and fulfilment latency from request to credential loaded, measured separately from swarm health.
- Zero accepted deliveries whose envelope fails the recipient's local validity check.

---

# Feature Name
Explicit Publishing and Escrow Claim

## Feature Objective
Let a rights holder register their own asset directly with publisher authority proven by the strongest available proof, deriving keys from their recoverable seed hierarchy, and let an upstream maintainer claim every escrowed record their proof covers in one transaction, receiving complete protocol-level ownership with no rights drift and no dependency on the First Finder.

## User Stories
- As a publisher, I publish a package with proof of my authority, price it, and consumers acquire it through a second identity.
- As a publisher, I recover every parameter set I have ever created from my seed phrase alone.
- As a maintainer of ninety escrowed versions, I make one claim and receive control of all ninety.
- As a maintainer, my claim succeeds with the First Finder gone, I issue under my own parameter set, and my successor deployment is readable by both escrow-era and new credentials.
- As a claimant, an intercepted proof cannot be used by another address.

## Acceptance Criteria
- Explicit publishing proves publisher authority, creates a unique deployment, derives parameter set and capsule randomness from the explicit-publisher hierarchy, registers authenticated metadata including the sidecar root, and seeds; invalid authority is rejected and a second identity consumes the result (PC-01; AS-22).
- Publisher proof resolves by strength per package: provenance attestation with repository control, then maintainer OAuth, with DNS framed and not implemented, and the proof class is recorded on-chain per record (MVP Scope, Explicit Publisher Path).
- Publisher-proof and claim-verifier adapters bind results to claimant, package or claim set, contract, chain, nonce, and expiry; replay and redirection fail (PC-03).
- Claim-set verification depends on no party that can be unavailable at claim and on no stored maintainer commitment; the verifier establishes the claim set from upstream metadata at verification time (PC-04).
- A successful claim transfers complete ownership and authority, registers the claimant's parameter set, leaves escrow-era credentials working, and allows successor deployments with the same plaintext root and a sidecar per live set (PC-05; AS-16).
- One verified maintainer proves every covered record without exposing reusable raw proof material; ineligible assets are excluded and replays fail (PC-06).
- The verifier's attestation key is registered on-chain with rotation and revocation; a revoked key's vouchers are rejected (PC-07; AS-27).
- Claims are scoped per package and per version and apply to the proof-derived set, not a claimant-chosen set (MVP Scope, Escrow Claim).

## Dependencies
- First Finder bootstrap, which creates the escrow records claims act on.
- Credential delivery, for the claimant's parameter set and voluntary migration.
- Ledger contracts, for claim settlement and the verifier-key registry.
- The claim verifier service.

## Success Metrics
- Claim success with the First Finder absent, target complete.
- Successor deployment readable under every live parameter set.
- Zero accepted redirected or replayed proofs.

---

# Feature Name
Transaction Flow Proof

## Feature Objective
Prove that a nonzero transaction settles by publishing the project's own package through the explicit-publisher path priced trivially above gas, having a funded buyer acquire it, and having that holder resell it, asserting the settlement boundary end to end while the entitlement graph is two identities.

## User Stories
- As the project, I know the price parameter, the payment leg, and the ordering of settlement against authorization actually work before anything real is priced.
- As a buyer, I pay with real funds, decrypt at the declared tier, and resell to another identity.
- As a seller, my client destroys its credential when the transfer reaches hard finality and not before.

## Acceptance Criteria
- Paid acquisition requires real buyer funding and is not presented as covered by the free subsidy; the unfunded attempt fails and the funded one completes primary and secondary settlement (RO-02; AS-15).
- Only project-owned explicitly published assets carry a nonzero price; First Finder content refuses pricing (LC-03).
- The priced deployment declares and justifies its own settlement tier rather than inheriting the free-path default (MVP Scope, Native Credentials).
- Primary issuance mints priced entitlements and a consumer acquires one; secondary transfer verifies the seller's proof and releases payment in one transaction, the buyer decrypts at the declared tier, and the seller destroys at `HARD` (MVP Scope, Transaction Flow Proof; AS-15).
- The project's package record carries content terms obligating an entitlement for use and a license per copy sold or bundled; publication without terms is refused (LI-01).

## Dependencies
- Explicit publishing, credential delivery, ledger and settlement, encrypted consumption.
- Licensing decisions, held in the workplan To-Do.

## Success Metrics
- One completed primary and one completed secondary priced settlement with every boundary assertion passing.
- Measured gas for the priced mint and transfer.

---

# Feature Name
Relaying and Free-Path Subsidy

## Feature Objective
Sponsor free entitlement acquisition and the one-time identity binding through a relayer or ERC-4337 paymaster funded by a grant pool, with a global per-window budget, a maximum sponsored liability, per-identity rate limits as one layer, explicit failure states, and cost reporting, so a first-run user is never blocked on acquiring gas.

## User Stories
- As a developer, I install free packages without holding any cryptocurrency and without knowing a relayer exists.
- As a developer, my first install registers one request for everything I do not hold, in one sponsored operation, and holders fulfil it while I am offline.
- As an operator, I see what the subsidy costs per install and can deny or exhaust it without producing false authorizations.
- As an operator, a flood of free mints from many fresh identities cannot spend beyond the subsidy budget I configured, and when it is exhausted onboarding fails truthfully rather than authorizing falsely.

## Acceptance Criteria
- The relayer sponsors free entitlement acquisition, batched grant requests and their fulfilling grants, and the single initial binding transaction with abuse controls and explicit failure states; exhaustion or denial yields actionable recovery without false authorization, with requests queued locally until sponsorship returns (RO-01; AS-14, AS-29).
- Zero-price grants, batched requests, and relayer-paid mints are admitted under a global per-window budget and maximum sponsored liability as well as per-identity limits that admit a realistic dependency closure in one batch; a flood of fresh identities never spends beyond the configured budget and exhaustion is an explicit failure (LC-10; AS-27).
- Per-attempt state reads never pass through the relayer (MVP Scope, Gas Relayer).
- Relayer gas is recorded per install (RO-03).

## Dependencies
- Ledger contracts and the launch network's account-abstraction support.
- Grant pool funding, held as a decision.

## Success Metrics
- Relayer gas per install.
- Sponsored spend under flood, never above the configured budget.
- Free onboarding success rate.

---

# Feature Name
Observability, Diagnostics, and Cost Instrumentation

## Feature Objective
Record the metrics that make the MVP's economics measurable, correlate one package request across every subsystem, expose degraded dependencies and recovery actions consistently through every control surface, and evaluate attempt latency against a latency budget declared before the run, all without recording secrets.

## User Stories
- As the project, I obtain cost per install, state-read cost, cryptographic cost, and CAS reuse as measured numbers rather than estimates.
- As a developer, when something fails I am told which capability failed and what to do, never a secret and never an insecure bypass.
- As an operator, I reconstruct any single install's path across package host, CAS, swarm, chain, custody, decryption, and background work.

## Acceptance Criteria
- Metrics record CAS hit and cross-project reuse, resolution path and its reason, request and grant timing, the fraction of a closure independent after its first run and time to independence, install wall-clock, state-read volume and latency, decapsulation time per group, proof generation and verification time and gas, swarm bytes, ingest bytes, relayer gas, job outcome, and adapter health without secrets; every acceptance path reconciles with induced activity (RO-03; AS-20).
- Structured logs and traces correlate one request across all subsystems under concurrent installs (RO-04).
- Health and diagnostics expose degraded dependencies, compatibility decisions, job state, and recovery actions consistently through CLI, Tauri, extension, and API (RO-05).
- Attempt latency is measured against a pre-declared interactive budget and yields a pass or failure (RO-06).
- Failure messages identify the failed capability and safe recovery without exposing secrets or suggesting bypasses (XA-04).
- All external inputs are untrusted until validated at their boundary; fuzzing yields bounded failure with no protected side effects (XA-01).

## Dependencies
- Every other feature, since each emits the metrics and traces this feature collects.
- The declared latency budget, set before the measurement run.

## Success Metrics
- Every metric in RO-03 reconciled against induced activity.
- Zero sensitive fields found in telemetry scans.
- Attempt-latency pass against the declared budget.

---

# Feature Name
Test Facilities and Demonstration Harness

## Feature Objective
Ship unit, integration, and end-to-end test facilities including a demonstration harness that creates controlled participants, services, wallets, chain state, and failures, so that every requirement's proof class maps to a facility and every acceptance scenario runs reproducibly through the packaged applications.

## User Stories
- As the project, every requirement identifier reconciles with a passing proof in CI on a clean machine.
- As a reviewer, I can rerun any acceptance scenario, including induced failures and races, and get the same result.
- As a user, the packaged MVP runs on a machine with no Rust, no Solidity tooling, and no repository source.

## Acceptance Criteria
- The repository ships unit, integration, and end-to-end facilities including the demonstration harness, and every requirement's proof class maps to one (XA-07).
- Durable workflows are idempotent, observable, cancellable where safe, and recoverable, proven by termination at every persisted transition (XA-03).
- The packaged MVP runs without a developer toolchain on supported machines (XA-05).
- Every acceptance scenario passes through the packaged applications with no test-only bypass of onboarding, the package host, capability resolution, or the deployed verifier (MVP Acceptance Scenarios, Completion Boundary).
- The demonstration harness supplies the deliberately incompatible adapter declarations the incompatibility scenario rejects (XA-07; AS-17).

## Dependencies
- Every feature above; this is the completion gate.

## Success Metrics
- Requirement-to-proof reconciliation, target every identifier.
- Acceptance scenario pass rate on clean machines, target every scenario.

---

# Feature Name
Project Seed Host and Site

## Feature Objective
Run a persistent first seeder of the core packages and their dependency closure, optionally hosting the relayer and a discovery endpoint, with a site carrying the education tier, explanation, a browser demonstration built from the client's own Rust crates compiled to WebAssembly against a sample deployment, and the download, so the swarm has a seed from day one and a prospective developer can see the protocol work before installing, while no client depends on any of it and no account exists.

## User Stories
- As a new participant, the core packages are available in the swarm the day I install, before any other participant is online.
- As a prospective developer, I read what the protocol does and watch real resolution, verification, and decapsulation run in my browser against sample content, with nothing to sign up for.
- As a developer, the installer shows me an account-link step that is unavailable in the MVP, so I know it is coming and know it is not required.
- As the project, nothing I run is on any client's critical path; a build platform can supersede it.

## Acceptance Criteria
- The seed host persistently seeds the core packages and their dependency closure after swarm-native publication (FF-09; AS-22).
- Any API it exposes is a convenience a client may use and never a component a client must reach; the protocol resolves without it (MVP Scope, Project Seed Host and Site).
- The seed host holds an entitlement, and so a credential, for every asset it seeds; a new identity obtains a grant from it with every other holder offline, and resolution proceeds with it unreachable (SW-08).
- The browser demonstration is built from the same crates as the client, runs against a sample deployment with a sample credential, performs no network, chain, or swarm access on the read path, and states that seeding and the CAS do not run in a browser (MVP Scope, Project Seed Host and Site).
- The installer's consent flow presents the account-link step as reserved and unavailable, and the consent trace records that it was shown and not acted on (MVP Scope, Website, Account, and Remote Head).
- Every asset record carries content terms and packaged applications ship under the chosen license (LI-01).

## Dependencies
- Swarm transport and seed hosting.
- Explicit publishing, for the dogfood publication that produces the content it seeds.
- License text, held in the workplan To-Do.

## Success Metrics
- Core dependency closure availability in the swarm from first publication.
- Zero client dependencies on the host, proven by resolution with it unreachable.

---

## Additional Content

**Cross-feature dependencies and build order.** The Application Requirements fix the dependency direction: protocol and domain, then application workflows, then adapters and host shells. The validation harness precedes every node that encrypts a registered deployment, because the piece-group size and attempt-rule parameters gate them. The payload cipher, the contracts, and the chain adapters follow, with hashing, signatures, and swarm hosting in parallel from the foundation onward; then the daemon, identity, package serving, First Finder bootstrap, credential delivery, and the demonstrable milestone; then onboarding shells, relaying, publishing and claims, the seed host and site, observability, and the demonstration harness; then the transaction flow proof and the completion gate.

**Features deliberately absent.** Git commit wrapping, an email bot for escrow, general paid monetization, a version alignment engine, arbitrary media and streaming, content flagging and advisory surfaces, per-entitlement variance, composable container objects, partial encryption, seeder compensation, retention enforcement, Sybil-resistant stake, and the website account, remote head, and hosted instance are deferred to V2 and architecturally protected as recorded in MVP Scope. None is a hidden dependency of any feature above.

**Deferred feature, recorded for its protection: website account, remote head, and hosted instance.** Objective: let a person create a website account with a familiar sign-in, see and manage their local installation from that account, and, as a separate product mode, use a hosted instance without a local install. What the account owns: onboarding state, preferences, a roster of linked identities and devices, and optionally an end-to-end encrypted recovery backup of the holder seed under a user-held secret; never an entitlement. User stories it would serve: as a developer, I link my installation to my account by a signed challenge and see its status, seeding archive, jobs, footprint, and diagnostics from any browser, and I start a repair remotely; as a developer, transfer, key export, binding changes, and uninstall require me to be at the machine; as a developer, I see and revoke my identity's admitted devices from any browser, while admission itself requires a root. Acceptance criteria it would carry: outbound-only rendezvous connection; end-to-end encryption under a daemon-pinned key; commands to the daemon with keys never leaving it; capability tiers with local presence for value-moving operations; the consent trace recording the authorizing surface; the account-to-identity mapping held only by the account service, never on chain, and optional; a hosted instance declared as its own adapter with a service on its read path. Dependencies: the daemon's authenticated IPC (XA-02), the custody adapter's declared multi-device capability (IW-02, IW-05), and the reserved link step in the installer. Protection in the MVP: the reserved consent step, the holder seed's control and envelope branches with device admission on the identity's contract account, and a browser demonstration built from the client's own crates.

**Open selections that features depend on.** The attempt-rule parameters and piece-group size, custody recovery UX, and the license text remain open. Each feature that depends on one names it under Dependencies.

**Anticipated capability, unimplemented in the MVP: the travelling developer.** Recorded early so that no MVP choice accidentally blocks or complicates it. Nothing below is an MVP requirement; every item under "what the MVP must preserve" is already a recorded decision, and the list exists so an implementer can check a choice against the story.

The story. A developer's workstation hosts their seeded packages, including their git repositories. Travelling, they work on a laptop already paired to the same identity; the laptop seeds too, but less, because it is often on battery and not always available. They finish a sprint and commit; the commit syncs to the workstation and to their git host and starts a CI pipeline. At dinner, away from every machine, the pipeline fails: the commit imports a package it never used before, the package is priced, and the identity holds no entitlement to it. The developer's phone, running the mobile client, shows the alert with the price. They approve. The entitlement is minted and its credential delivered to their identity. The pipeline resumes and completes, and the phone shows the success.

What each step needs, and where it is recorded. The workstation is the identity's root and holds the holder seed; the laptop is a reader, holding its own device key admitted as a signer on the identity's contract account and the envelope secrets the workstation sent over the paired channel, so it reads alone while the seed never leaves the workstation: the multi-device position in the product requirements. The laptop's lighter seeding is a per-device footprint setting; retention standing is per identity. Git repositories as swarm objects are the deferred Git commit wrapping item. The CI runner is a read-delegate device: it holds its own admitted device key and only decrypted credentials in memory, supplied by the always-on workstation under a delegation signed by the identity's bound handshake key, and reads without ever being able to transfer; the specification permits this by separating the persistent credential from the decrypted one, and the role is a declared custody capability. The phone is a signer device holding only its own device key, admitted with permission to lock payment; approving the purchase is a signed intent through the identity's contract account that locks payment against the exact entitlement, seller, price, and expiry, and local presence means presence at an admitted device whose permissions cover the operation, which the phone is. The mint completes without a person present because publisher discretion is an issuance policy served automatically by the publisher's client: MVP Scope, Paid Monetization. The credential is delivered to the identity's registered envelope keys, which the workstation opens and supplies to the runner. The alert and approval travel over the identity-scoped notification channel of the deferred account and rendezvous tier, raised by the workstation as the runner's delegating device.

What the MVP must preserve so the story stays possible. The holder seed's control branch, holding the handshake key, and its envelope branch, holding the envelope secrets, are domain-separated, so a device is added by admitting its own key and handing it the envelope secrets, never by moving the seed and never by registering fresh envelope keys, which would be a sale to oneself per entitlement. The custody adapter's capability declaration is versioned and admits roles beyond root and reader, so read-delegate and signer can be added without a protocol change. The decrypted credential stays a memory-only object distinct from the persistent credential, so a device can hold one without the other. The per-attempt wallet-control assertion is made by any device key the identity's contract account admits, read in the same state view as the authorization, so every admitted device satisfies it with its own key and a revoked one fails it. The publisher authority adapter's capability declaration admits an issuance policy served automatically, so a priced mint does not require an interactive publisher. The installer reserves the account-link step, so the notification channel arrives as a feature rather than a redesign. Daemon IPC is authenticated and local, so a remote head or a delegate session is a further capability on the same surface rather than a second one. Per-device footprint limits exist, so a laptop can seed less than a workstation under one identity.

What would block it, stated so it is not chosen by accident. Per-device envelope key registration. Requiring the seed on a device in order to admit it. An identity held as an externally owned account, which has no signer set to admit devices to. A custody adapter whose capability declaration is a fixed enum rather than a versioned set. Persisting the decrypted credential as part of the persistent credential, or requiring the persistent credential for reading. A wallet-control check that requires the identity's handshake key on every device rather than an admitted device key. A mint path that requires a person to act. A consent or presence rule that names the workstation rather than any device holding the identity. Retention standing computed per device rather than per identity.
