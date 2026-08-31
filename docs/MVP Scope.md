# MVP Scope

## Objective

Deliver a working `torrent install` path for JavaScript dependencies that survives registry outages, serves popular packages from a peer swarm, and exercises the full identity/entitlement/encryption pipeline end to end — with all monetization set to $0.00.

This MVP validates **distribution and identity**. It deliberately does not validate willingness to pay or seeder compensation.

This document states the boundary of the work. Undetermined decisions are held in the To-Do list of [ChainTorrent MVP.md](workplans/current/ChainTorrent%20MVP.md).

Terminology follows [cryptography.md §1.2](cryptography.md) — entitlement, handshake key, content key, master key, escrowed key, transfer sequencer, provisioning layer — and the unqualified words "key" and "token" are not used for any of them.

---

# In Scope

1. **The CLI & Extension Proxy (`torrent install`)**:
* Reads `package.json`, builds a local dependency DAG.
* Checks the local CAS first; if missing, checks the P2P swarm + Ledger; if missing there, falls back to the ingest source.
* Two operations run in the install path and must not be conflated. **Entitlement acquisition** inspects the wallet and skips targets the identity already holds, because those need not be acquired again. **Decryption authorization** is per window and is never skipped, cached, or presumed.

2. **Canonical Identity & As-Is Ingestion**:
* The **identity hash is deterministic**: `BLAKE3(packageName@version)`. This is what the Registry is keyed on, and it is what guarantees every node resolves the same asset to the same canonical record.
* The **ciphertext is not deterministic, by design**. Content keys are randomly generated per deployment, per [cryptography.md §1.1 Anti-Derivability](cryptography.md). Deterministic ciphertext would let any holder of the plaintext recompute the content key and bypass escrow entirely — and for public NPM packages, *everyone* holds the plaintext.
* Ingestion runs through the **`IIngestSourceAdapter`** ([cryptography.md §4 Phase 1.2](cryptography.md)). **NPM is the MVP implementation, not the definition** — pnpm, Bun, PyPI, crates.io, or a publisher's own release endpoint are peer implementations behind the same interface, and none of them is a special case.
* The First Finder **does not repackage or normalize** the artifact. It fetches the bytes exactly as served and hashes them as-is. For the NPM implementation that is the immutable `.tgz`. Normalization would break the source's own integrity attestation, which is the only defense against a poisoned ingest becoming permanently canonical.
* The First Finder **must verify** the fetched artifact against the upstream integrity attestation before registering it, and record that attestation on-chain alongside the identity hash. For NPM that attestation is the published `integrity` sha512; the on-chain field is source-dependent, not NPM-shaped.
* Two nodes ingesting the same unlisted package produce different ciphertext. This is resolved at the identity layer, not the payload layer: **State-Locked Escrow Registration** means the first transaction to land wins, and the losing node discards its local ciphertext and content key and repoints at the winner's infohash.

3. **Per-Window Decryption Authorization**:
* Before any decryption, the client obtains authorization for a bounded window through the **key provisioning adapter**, evaluated against chain state at the deployment's declared settlement tier. An entitlement held for years authorizes nothing until the current window is established.
* **The MVP declares `INCLUDED`** ([cryptography.md §2 Settlement Adapter](cryptography.md)). Entitlements are $0.00 and packages are permissively licensed, so the value at risk in a reversal is zero, and the exposure it could produce — one window, one package, one identity — is the same bound the protocol already publishes for a seller's retained capacity. Requiring deeper settlement would make `torrent install` wait on chain finality for no benefit; a CI pipeline cannot tolerate that and gains nothing by paying it.
* **No local keystore.** The content key is held in memory for the window only and is never written to disk. There is no cache of prior authorization results and no exemption for assets decrypted previously.
* Window bounds are `k` (settlement-reference ceiling) and `M` (volume cap), expiring at whichever is reached first. Both are MVP configuration with published values; see the To-Do list for their selection and the capacity-versus-boundary tradeoff in [cryptography.md §8.1](cryptography.md). `M` is client-side and unobservable to the provisioner, so only `k` carries that tradeoff.
* On expiry the client discards the content key and requests a new window if decryption continues. Renewal is an ordinary authorization request and is refused if the entitlement no longer holds.
* The DKMN is the MVP implementation of the provisioning layer. It sits behind the adapter and is replaceable by any construction meeting the same two obligations — per-window authorization and no selective withholding.

4. **Local Content Addressable Storage (CAS) + Symlinker**:
* Single global cache directory storing decrypted packages by content hash.
* Hardlinks/symlinks from global CAS directly into the project's `node_modules`.
* Plaintext in the CAS **persists by design and is outside the authorization boundary**. This is the retained-plaintext commitment of [cryptography.md §1.1](cryptography.md), not a gap: a developer who installed a package keeps it and uses it with any toolchain.

5. **$0.00 Entitlement Ledger & Escrow Contract (EVM Testnet / L2)**:
* Smart contract that registers `(PackageIdentityHash, UpstreamAttestation, CiphertextRoot, PlaintextRoot, CanInfohash, OwnerIdentifierHash)`.
* **Both content roots are registered, not just the infohash, and the plaintext root is registered in Public disclosure mode** ([cryptography.md §2](cryptography.md)). Provenance and independent verifiability are the point of the package path, and NPM plaintext is openly distributed regardless, so there is nothing to mask. The infohash establishes what the swarm carries; the plaintext root establishes what a correct decryption must produce. Without it the `Verify plaintext root` step of the execution trace has nothing to verify against, and a client provisioned an incorrect content key produces garbage that passes every other check the protocol performs — see [cryptography.md §2 Content Commitments](cryptography.md).
* Mints $0.00 entitlements to requesting identities as transferable bearer assets.
* Exposes `isAuthorizedBatch([packageHashes], requestingIdentity)` as a multicall view function, evaluated by provisioning nodes at an agreed settlement reference meeting the declared tier. This is the authorization basis, not merely a concurrency device.
* **The view returns three states** — `AUTHORIZED`, `PENDING_SETTLEMENT`, `DENIED`. Authorization is transactional against evolving state, and during an install the entitlement is minted seconds before it is read, so "not settled yet" and "no" are different answers requiring different client behavior. See [cryptography.md Phase 2.4](cryptography.md).
* If a package is ingested via First Finder fallback, the contract creates an escrow record tagged with a **salted commitment** to the maintainer's public NPM email, not a bare hash. Email addresses are low-entropy and enumerable, so a bare hash publishes a permanently testable package-to-maintainer mapping — an on-chain disclosure the upstream source itself no longer makes, and [cryptography.md §8.2](cryptography.md)'s concern arriving in a form the MVP actually ships. The salt is held by the claim portal and released to a claimant on successful verification, so the claim flow of deferred item 2 is unchanged.
* **Per-identity binding record.** The contract stores the bound handshake public key and its scheme on-chain — the fields on-chain logic must read — plus an anchor hash committing to the full DID Document, which resolves off-chain. Contracts cannot read an off-chain document without an oracle or a proof system, so the fields consulted during escrow claim verification and authorization checks are not among the anchored ones. See [cryptography.md §2 Binding Schema](cryptography.md).

6. **Gas Relayer / Dev Grant Pool**:
* A dead-simple backend relayer (or Paymaster) funded by a developer grant that signs $0.00 entitlement mint transactions on behalf of newly generated identities. No user-funded gas, no fiat deposits, no friction.
* **One binding attestation transaction per new identity.** At wallet creation the relayer pays for a single transaction registering the identity's handshake public key, signed by the chain identity. It runs once per identity, never per install, and is invisible to the user.
* Authorization requests are **not** chain transactions and do not pass through the relayer. They are off-chain requests to the provisioning layer reading an on-chain view function, so per-window authorization adds provisioning traffic rather than gas cost.
* This is acknowledged scaffolding: it is a centralized chokepoint inside a decentralization thesis, and it is an unmetered subsidy. It exists to remove first-run friction, and it is expected to be replaced.

7. **Cost Instrumentation**:
* Record relayer gas spend and swarm bandwidth per install. Cost-per-install is the first real unit-economics datapoint the project can obtain, and it is nearly free to capture at this stage.
* Record **authorization request volume and latency** per install, separately from gas. This is the primary load driver on the provisioning layer and the empirical input to the capacity question in [cryptography.md §8.1](cryptography.md); `k` and `M` cannot be calibrated without it.

## Signature Scheme (MVP)

The MVP resolves signatures through the **`ISignatureAdapter`** abstraction defined in [cryptography.md §2](cryptography.md), resolved per layer, and ships two implementations.

**`Ed25519Adapter` is the protocol's preferred scheme** and is used at the provisioning handshake and content-signing layers — the layers where signature volume actually lives, since Phase 2.3 signs an aggregate authorization payload per window across whole dependency trees. Ed25519 is faster, is highly resistant to side-channel attacks, and is the general preference for new protocols.

**`Secp256k1Adapter` is used at the chain layer only**, because the MVP targets an EVM L2 where the account *is* a secp256k1 keypair and the chain permits no alternative. This is a constraint imposed by the target chain, not a protocol preference.

The two keys are bound rather than matched: the Ed25519 handshake public key is registered in the Registry once at wallet creation, signed by the chain identity, in a relayer-paid attestation that is thereafter verifiable by any party. Additional chains and additional layers are new adapters, not protocol changes.

## What the MVP Does and Does Not Exercise

Per-window authorization is built in full, but JavaScript dependencies exercise it lightly. A package is decrypted once at install, its plaintext lands in the CAS, and it is used from there indefinitely without further decryption — so the authorization boundary is crossed rarely for this content class. That is the expected behavior of install-once content under the retained-plaintext commitment, not a defect in the mechanism.

The consequence for the MVP is that the *mechanism* is validated while its *load profile* is not. Streaming and large-media content classes, which re-authorize continuously, will exercise it far harder. Authorization instrumentation (item 7) exists so that the MVP still produces a usable baseline rather than none.

---

# Deferred to V2+, but Architecturally Protected

1. **Git Commit Wrapping (`git commit` as torrents)**:
* **Why cut:** Git introduces mutable DAGs, branch delta tracking, and merge conflict complexity. Packages are static, immutable tarballs. Master package distribution first; tackle Git after the package network is live.

2. **Active Email Bot for First Finder Escrow**:
* **Why cut:** Sending automated emails every time an un-registered package is ingested creates spam risks, domain reputation hurdles, and unnecessary backend state.
* **Architectural protection:** Store a salted commitment to the package maintainer's email from `package.json` in the smart contract state (`escrowOwnerCommitment`), per item 5 of In Scope. When the maintainer eventually shows up on a web portal, they authenticate via OAuth/email magic-link, and the portal releases the salt so they can claim all escrowed content keys matching their commitment in one shot.

3. **Paid Monetization ($ > $0.00)**:
* **Why cut:** Introducing token economics, pricing, or fiat-to-crypto rails introduces immense regulatory and implementation drag.
* **Architectural protection:** The contract treats entitlements as transferable bearer assets whose price is a parameter. Changing that parameter from `0` to `X` later requires zero client refactoring.

4. **Version Alignment Engine / Dependency Tree Warnings**:
* **Why cut:** Calculating and warning about multi-version package bloat adds UI/UX complexity to the installer.
* **Architectural protection:** The DAG parser still resolves exact hash matches; version alignment can be surfaced later as a diagnostic feature (`torrent audit`).

5. **Arbitrary Media & Streaming Engine**:
* **Why cut:** Video byte-range seeking requires a completely different client pipeline. Stay 100% focused on JS developer dependencies.

6. **Content Flagging & Deprecation Surface**:
* **Why cut:** The backlink metadata layer is not built in the MVP.
* **Architectural protection:** Canonical records are immutable, but *advisory* — see [cryptography.md §6](cryptography.md). Deprecation, malware advisories, and unwanted-content flags are backlink metadata parsed against the client's trust set, not registry mutations. Framing the handler now costs nothing; implementing it can wait.

7. **Per-Entitlement Variance & Forensic Attribution**:
* **Why cut:** Variant seeds, re-minting on transfer, and collusion-resistant fingerprinting are meaningless while entitlements are $0.00 and content is permissively licensed. Seed authorship is itself unresolved ([cryptography.md §8.3](cryptography.md)).
* **Architectural protection:** The MVP ships **per-window provisioning only** ([cryptography.md §7.1](cryptography.md)) — content uniform across holders, each provisioning response wrapped to the requesting identity for transport. Enabling variance later adds a per-entitlement variant root and variant object and requires no change to the identity, registry, or entitlement layers. Because variance is an overlay over a complete swarm object, it does not alter the ciphertext the MVP distributes. See [cryptography.md §7](cryptography.md).

8. **Seeder Compensation**:
* **Why cut:** Rewarding seeding requires the token economics cut in item 3.
* **Architectural protection:** Seeding is already the network's distribution mechanism and needs no structural change to become compensated. The problem is recorded in [cryptography.md §8.4](cryptography.md) and candidate mechanisms in the To-Do list.

---

# The MVP Execution Trace

When a developer runs `torrent install lodash`:

```
[1. Local Check]   ──> Plaintext in CAS? ──(Yes)──> Symlink to node_modules ──> DONE
                            │ (No)                  (retained plaintext; no
                            ▼                        authorization required)
[2. Swarm Check]   ──> Identity hash on Ledger?
                            │
              ┌─────────────┴─────────────┐
           (Yes)                        (No — First Finder)
              │                                │
              ▼                                ▼
     Pull ciphertext via Swarm        Fetch bytes via IIngestSourceAdapter
              │                                │
              ▼                                ▼
     Verify ciphertext root           Verify upstream attestation
              │                       — abort on mismatch
              │                                │
              ▼                                ▼
[3. Acquisition]                       Encrypt with random content key
   Entitlement held? ──(Yes)──┐                │
              │ (No)          │                ▼
              ▼               │       Register Identity Hash on Ledger
   Mint $0.00 Entitlement     │       (first tx wins; loser discards
              │               │        ciphertext and repoints)
              └───────┬───────┘       Escrow content key (Maintainer Hash)
                      │               Mint $0.00 Entitlement to Identity
                      ▼                        │
[4. Authorization]                             ▼
   Request decryption window          Save to CAS & Seed Swarm
   — never skipped, never cached
                      │
                      ▼
   isAuthorizedBatch() at declared tier
   AUTHORIZED / PENDING_SETTLEMENT / DENIED
                      │
                      ▼
   Content key provisioned for window
   (memory only, never written to disk)
                      │
                      ▼
[5. Decrypt]  Verify plaintext root ──> CAS ──> Symlink
                      │
                      ▼
[6. Expiry]   Window lapses ──> content key discarded from memory
              Ciphertext retained ──> keep seeding

```
