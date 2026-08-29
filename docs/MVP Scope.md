# MVP Scope

## Objective

Deliver a working `torrent install` path for JavaScript dependencies that survives registry outages, serves popular packages from a peer swarm, and exercises the full identity/entitlement/encryption pipeline end to end — with all monetization set to $0.00.

This MVP validates **distribution and identity**. It deliberately does not validate willingness to pay or seeder compensation.

This document states the boundary of the work. Undetermined decisions — including the evidence path for monetization, the first-user value proposition, ingestion consent, and whether this increment is correctly sized — are held in the To-Do list of [ChainTorrent MVP.md](workplans/current/ChainTorrent%20MVP.md).

---

# In Scope

1. **The CLI & Extension Proxy (`torrent install`)**:
* Reads `package.json`, builds a local dependency DAG.
* Checks the local CAS first; if missing, checks the P2P swarm + Ledger; if missing there, falls back to NPM registry.

2. **Canonical Identity & As-Is Ingestion**:
* The **identity hash is deterministic**: `BLAKE3(packageName@version)`. This is what the Registry is keyed on, and it is what guarantees every node resolves the same asset to the same canonical record.
* The **ciphertext is not deterministic, by design**. Symmetric Content Keys are randomly generated per deployment, per [cryptography.md §1.1 Anti-Derivability](cryptography.md). Deterministic ciphertext would let any holder of the plaintext recompute the SCK and bypass escrow entirely — and for public NPM packages, *everyone* holds the plaintext.
* The First Finder **does not repackage or normalize** the tarball. It fetches the immutable `.tgz` exactly as served by the NPM registry and hashes it as-is. Normalization would break the registry's own `integrity` (sha512) attestation, which is the only defense against a poisoned ingest becoming permanently canonical.
* The First Finder **must verify** the fetched tarball against the NPM-published `integrity` hash before registering it, and record that hash on-chain alongside the identity hash.
* Two nodes ingesting the same unlisted package produce different ciphertext. This is resolved at the identity layer, not the payload layer: **State-Locked Escrow Registration** means the first transaction to land wins, and the losing node discards its local ciphertext/SCK and repoints at the winner's infohash.

3. **Local Content Addressable Storage (CAS) + Symlinker**:
* Single global cache directory storing decrypted packages by content hash.
* Hardlinks/symlinks from global CAS directly into the project's `node_modules`.

4. **$0.00 Key Ledger & Escrow Contract (EVM Testnet / L2)**:
* Smart contract that registers `(PackageIdentityHash, NpmIntegrityHash, CanInfohash, OwnerIdentifierHash)`.
* Mints $0.00 access tokens to requesting user wallets as transferable bearer assets.
* If a package is ingested via NPM fallback ("First Finder"), the contract creates an escrow record tagged with the maintainer's public NPM email hash.
* **Per-identity binding record.** The contract stores the bound handshake public key and its scheme on-chain — the fields on-chain logic must read — plus an anchor hash committing to the full DID Document, which resolves off-chain. Contracts cannot read an off-chain document without an oracle or a proof system, so the fields consulted during escrow claim verification and authorization checks are not among the anchored ones. See [cryptography.md §2 Binding Schema](cryptography.md).

5. **Gas Relayer / Dev Grant Pool**:
* A dead-simple backend relayer (or Paymaster) funded by a developer grant that signs $0.00 key mint transactions on behalf of newly generated user wallets. No user-funded gas, no fiat deposits, no friction.
* **One binding attestation transaction per new wallet.** At wallet creation the relayer pays for a single transaction registering the identity's handshake public key, signed by the chain identity. It runs once per wallet, never per install, and is invisible to the user.
* This is acknowledged scaffolding: it is a centralized chokepoint inside a decentralization thesis, and it is an unmetered subsidy. It exists to remove first-run friction, and it is expected to be replaced.

6. **Cost Instrumentation**:
* Record relayer gas spend and swarm bandwidth per install. Cost-per-install is the first real unit-economics datapoint the project can obtain, and it is nearly free to capture at this stage.

## Signature Scheme (MVP)

The MVP resolves signatures through the **`ISignatureAdapter`** abstraction defined in [cryptography.md §2](cryptography.md), resolved per layer, and ships two implementations.

**`Ed25519Adapter` is the protocol's preferred scheme** and is used at the DKMN handshake and content-signing layers — the layers where signature volume actually lives, since Phase 2.2 signs aggregate payloads in batches across whole dependency trees. Ed25519 is faster, is highly resistant to side-channel attacks, and is the general preference for new protocols.

**`Secp256k1Adapter` is used at the chain layer only**, because the MVP targets an EVM L2 where the account *is* a secp256k1 keypair and the chain permits no alternative. This is a constraint imposed by the target chain, not a protocol preference.

The two keys are bound rather than matched: the Ed25519 handshake public key is registered in the Registry once at wallet creation, signed by the chain identity, in a relayer-paid attestation that is thereafter verifiable by any party. Additional chains and additional layers are new adapters, not protocol changes.

---

# Deferred to V2+, but Architecturally Protected

1. **Git Commit Wrapping (`git commit` as torrents)**:
* **Why cut:** Git introduces mutable DAGs, branch delta tracking, and merge conflict complexity. Packages are static, immutable tarballs. Master package distribution first; tackle Git after the package network is live.

2. **Active Email Bot for First Finder Escrow**:
* **Why cut:** Sending automated emails every time an un-registered package is ingested creates spam risks, domain reputation hurdles, and unnecessary backend state.
* **Architectural protection:** Simply hash the package maintainer's email from `package.json` and store it in the smart contract state (`escrowOwnerHash`). When the maintainer eventually shows up on a web portal, they authenticate via OAuth/email magic-link to claim all escrowed master keys matching their email hash in one shot.

3. **App Tokens & Paid Monetization ($ > $0.00)**:
* **Why cut:** Introducing token economics, pricing, or fiat-to-crypto rails introduces immense regulatory and implementation drag.
* **Architectural protection:** The contract validates *tokens as permission keys*. Changing key price parameters from `0` to `X` later requires zero client refactoring.

4. **Version Alignment Engine / Dependency Tree Warnings**:
* **Why cut:** Calculating and warning about multi-version package bloat adds UI/UX complexity to the installer.
* **Architectural protection:** The DAG parser still resolves exact hash matches; version alignment can be surfaced later as a diagnostic feature (`torrent audit`).

5. **Arbitrary Media & Streaming Engine**:
* **Why cut:** Video byte-range seeking requires a completely different client pipeline. Stay 100% focused on JS developer dependencies.

6. **Content Flagging & Deprecation Surface**:
* **Why cut:** The backlink metadata layer is not built in the MVP.
* **Architectural protection:** Canonical records are immutable, but *advisory* — see [cryptography.md §6 Deprecation, Advisory Flags, and Content Governance](cryptography.md). Deprecation, malware advisories, and unwanted-content flags are backlink metadata parsed against the client's trust set, not registry mutations. Framing the handler now costs nothing; implementing it can wait.

7. **Per-Entitlement Variance & Forensic Attribution**:
* **Why cut:** Variant sets, re-minting on transfer, and collusion-resistant fingerprinting are meaningless while keys are $0.00 and content is permissively licensed.
* **Architectural protection:** The MVP ships the **wrapped-key posture only** ([cryptography.md §7.1](cryptography.md)) — distinct key bytes per identity, common SCK underneath. Enabling variance later adds a per-entitlement variant root and a variant object, and requires no change to the identity, registry, or entitlement layers. See [cryptography.md §7](cryptography.md).

---

# The MVP Execution Trace

When a developer runs `torrent install lodash`:

```
[1. Local Check]  ──> Package in CAS? ──(Yes)──> Symlink to node_modules ──> DONE
                          │ (No)
                          ▼
[2. Swarm Check]  ──> Identity hash on Ledger? ──(Yes)──> Pull ciphertext via Swarm
                          │                             │
                          │ (No - First Finder)         ▼
                          │                     Verify BLAKE3/Bao root
                          │                             │
                          ▼                             ▼
                  Fetch .tgz from NPM             Mint $0.00 Key
                          │                             │
                          ▼                             ▼
                  Verify NPM integrity           Request SCK from DKMN
                  (sha512) — abort on mismatch          │
                          │                             ▼
                          ▼                     Decrypt to Local CAS ──> Symlink
                  Encrypt with random SCK
                          │
                          ▼
             Register Identity Hash on Ledger
             (first tx wins; loser discards
              ciphertext and repoints)
             Escrow SCK (Maintainer Hash)
             Mint $0.00 Key to User Wallet
                          │
                          ▼
              Save to CAS & Seed Swarm

```
