# Transactable Key Protocol: Cryptographic Specification

## 1. Overview and Threat Model

The Transactable Key Protocol provides a decentralized, frictionless architecture for distributing encrypted digital assets and dynamically provisioning access based on blockchain state.

### 1.1 Core Philosophy

This protocol optimizes for **frictionless distribution and access**, not hostile digital rights management (DRM).

* **The Distribution Problem:** Piracy is fundamentally a distribution and friction problem. By making legitimate, highest-quality access seamless and inexpensive, the incentive for piracy is mitigated.
* **No Hardware Enclaves:** The protocol strictly avoids proprietary Trusted Execution Environments (TEEs) or hardware-level DRM. Such mechanisms introduce platform friction, violate open-source ethos, and create centralized failure points.
* **The DRM Boundary (Key vs. Plaintext):** The protocol strictly governs the lifecycle of the **Symmetric Content Key (SCK)**. The protocol accepts the "analog hole" and acknowledges that attempting adversarial, OS-level plaintext enforcement on a user-controlled device is hostile and futile.
* **The Application-Layer Contract:** Enforcement relies on the state transitions of the open-source reference client. When token ownership changes, the client’s contractual obligation is to **destroy the key**, ceasing all further decryption. It explicitly does *not* attempt to track, flush, or purge plaintext that has already been rendered or exported.
* **Anti-Derivability & Random SCKs:** Symmetric Content Keys **must** be generated randomly per asset deployment, never derived deterministically from plaintext payloads. While deterministic key derivation would trivially solve swarm fragmentation, it completely destroys economic enforcement by allowing any possessor of plaintext to independently compute the key and bypass the DKMN escrow. Security and economic viability supersede naive payload deduplication.
* **Incremental Verification:** The protocol provides incremental, random-access content verification. A client can cryptographically verify an individual chunk using that chunk and its Merkle authentication path, without downloading the remainder of the asset. The Merkle root is independently authenticated by the protocol's content/deployment commitment.

## 2. Cryptographic Primitives

To ensure high performance, enable out-of-order streaming, and maintain native compatibility with Content-Addressable Networks (CAN) like BitTorrent and IPFS, the protocol utilizes:

**Symmetric Encryption (Payload):** `AES-CTR` utilizing a randomized 96-bit Initialization Vector (IV) per file alongside a 64-bit block counter. To support out-of-order chunk decryption and byte-range seeking without custom nonce hashing, the counter offset for any specific chunk is calculated directly via standard block arithmetic: $\text{Counter}_{\text{offset}} = \text{IV} + \left(\frac{\text{ChunkIndex} \times \text{ChunkSize}}{16}\right)$.

* **Chunk Alignment:** The protocol enforces strict Chunk Alignment, mandating that all CAN piece sizes are perfect multiples of the 16-byte AES block size, completely eliminating partial-block padding complexities across chunk boundaries.
* **Nonce Invariants:** The 96-bit IV is generated randomly once per deployment and stored in the clear within the CAN manifest. This establishes a strict nonce invariant for the lifetime of that specific file, guaranteeing the block arithmetic holds across the swarm.
* Payload integrity and verification are handled natively by the BLAKE3-Bao Merkle tree layer, removing the need for redundant AEAD tag overhead while allowing instantaneous seek-and-decrypt capabilities.

**Integrity & Verification:** `BLAKE3` (utilizing a Bao-style verified streaming structure) provides a native Merkle tree for chunk verification. BLAKE3/Bao provides cryptographic integrity proofs for individual chunks and binds those chunks to the authenticated content root, eliminating manual MAC tree management overhead and mapping directly to Merkle DAG structures (e.g., Git repositories).

**Confidentiality & Integrity Separation:** The protocol deliberately separates confidentiality from content integrity. AES-CTR provides encryption; BLAKE3/Bao provides independently verifiable content integrity and random-access authentication proofs.

**Content Commitments (Ciphertext, Plaintext, and Variant Roots):** The canonical on-chain record commits to **two** BLAKE3/Bao roots, with a **third** added per-entitlement when per-entitlement variance is enabled (§7):

* the **ciphertext root**, which lets seeders and downloaders verify opaque chunks they cannot read;
* the **plaintext root**, which lets a decrypting client verify that what it produced is the canonical plaintext — computed over the *invariant* portion of the asset when variance is in use; and
* the **variant root** (per-entitlement, carried in the entitlement record rather than the asset record), which commits to the recipient-specific chunk set. Absent under the MVP's uniform-content posture.

The ciphertext root alone is insufficient. Integrity checking happens entirely at the ciphertext layer, so a client that is provisioned an incorrect key — through oracle fault, compromise, or a substituted response — produces garbage plaintext that passes every check the protocol otherwise performs. The plaintext root closes that gap.

Critically, the commitment is to the **plaintext, not to the key**. Committing to a key (e.g. publishing `BLAKE3(SCK)`) would hard-code a single universal content key and foreclose per-identity key provisioning. Committing to the plaintext constrains the *output* rather than the mechanism, so it holds unchanged under wrapped keys, per-identity subkeys, traceable decryption keys, or any future provisioning scheme. It also provides plaintext-side incremental verification that mirrors the ciphertext tree: a single decrypted chunk can be validated against its authentication path without decrypting the remainder of the asset.

*Caveat:* publishing a plaintext root permits confirmation-of-content attacks against low-entropy or guessable assets — an observer holding a candidate plaintext can confirm it. This is a non-issue for public assets such as NPM packages, whose plaintext is openly distributed regardless, and should be weighed for short or private content.

**Key Management Network (DKMN):** A decentralized threshold cryptography network (Multi-Party Computation, e.g., Lit Protocol) to lock, escrow, and provision the SCK based on on-chain conditions without ever exposing plaintext keys to blockchain validators.

**Wallet Signatures (Signature Adapter):** proof-of-possession is performed with the wallet's native curve, resolved through an abstract **`ISignatureAdapter`** rather than pinned to any one scheme. The protocol is chain-agnostic by construction; the curve is an implementation detail of the target chain, not a property of the protocol.

```
    isValidSignature(pubkey, message, signature) -> bool
    recoverSigner(message, signature)            -> address
    canonicalAddress(pubkey)                     -> address
```

| Adapter | Curve / Scheme | Target |
| --- | --- | --- |
| `Secp256k1Adapter` | secp256k1 / ECDSA | EVM chains — chain layer only (MVP) |
| `Ed25519Adapter` | Ed25519 / EdDSA | protocol-preferred scheme; handshake and content-signing layers (MVP); chain layer on Solana, Near, Cosmos-family |
| `Secp256r1Adapter` | secp256r1 / P-256 | passkeys, WebAuthn, secure enclaves |
| `BlsAdapter` | BLS12-381 | aggregate/threshold signatures |

* **Per-layer resolution:** the signature adapter is resolved independently at each layer where a signature is produced or verified. A deployment is not restricted to a single scheme — the chain layer uses whatever the target chain mandates, while the handshake and content-signing layers retain the protocol's preferred scheme regardless of chain.
* **Binding, not matching:** where two layers use different keys, those keys must be provably the same principal. The Registry holds a binding attestation — the subordinate key signed by the authoritative chain identity — established once and thereafter verifiable by any party. Keys unbound across layers are a protocol violation.
* **Relationship to `IIdentityAdapter`:** the two are distinct and compose. The signature adapter answers *"is this signature cryptographically valid?"*; the identity adapter answers *"is this validated claimant authorized for this asset?"* Authentication and authorization stay separable, so a new chain requires a new signature adapter without touching authorization logic, and a new authorization model requires no cryptographic changes.
* Adapter resolution follows the same on-chain patterns as identity adapters — constructor injection via factory, resolved from a governance-controlled `AdapterRegistry`, immutable once bound.

### The Hash-Card (Deployment Descriptor)

The content commitments are fields of a larger descriptor. The `.torrent` file is the present-day carrier; the **hash-card** is its intended successor.

**Compatibility posture.** Full backwards compatibility with existing BitTorrent clients is a hard MVP requirement, so the canonical on-chain record natively exposes a standardized magnet link and BTIH (see Phase 1.1). Unmodified clients participate in the swarm with no blockchain integration. The hash-card is therefore introduced as a *superset*: everything a `.torrent` carries, plus the fields a legacy format has no place to put. Legacy clients read the subset they understand; protocol clients read the whole card.

| Field | Present in `.torrent`? | Purpose |
| --- | --- | --- |
| Identity hash — `BLAKE3(name @ version)` | no | canonical Registry key |
| Ciphertext Bao root | partly (piece hashes) | verify opaque chunks while seeding |
| **Plaintext fingerprint (Bao root)** | **no** | verify decryption; identify content across encryptions |
| AES-CTR IV, chunk size, alignment | no | deterministic block arithmetic across the swarm |
| Registry pointer / ACC reference | no | where authorization is resolved |
| Upstream attestation (e.g. NPM sha512) | no | bind to the Web2 source of truth |
| Advisory backlink root | no | entry point for §6 governance metadata |
| Magnet / BTIH | yes | legacy client interoperability |

#### What the Plaintext Fingerprint Buys

The plaintext fingerprint is referenced elsewhere in this specification as a means of proving that ciphertext and plaintext correspond. That is the smallest of its uses. Because it is an identifier for *content* rather than for any particular encryption of that content, it provides:

* **Decryption correctness.** The client can prove it was provisioned the right key and produced canonical output, incrementally and per-chunk (see above).
* **Encryption-independent content identity.** Two deployments with different SCKs produce entirely different ciphertext but the *same* plaintext root. Identical content is therefore provably identical across independent encryptions.
* **Deterministic resolution of First Finder races.** When two nodes ingest the same asset concurrently, the loser of State-Locked Escrow Registration can *prove* its discarded object was the same content rather than trusting the identity hash alone — and any third party can verify the equivalence.
* **Re-encryption and key rotation without loss of identity.** A publisher rotating an SCK or re-deploying an asset produces new ciphertext whose plaintext root is unchanged, demonstrating that nothing about the content changed. Migration becomes verifiable rather than asserted.
* **Advisories that follow the content.** A malware or deprecation flag (§6) targeting the plaintext fingerprint remains attached across re-encryptions, re-deployments, and re-uploads under new identities. Flags cannot be shed by repackaging — which is the single most common evasion in existing package ecosystems.
* **Binding to Web2 sources of truth.** The plaintext root is checkable against the upstream artifact (the NPM `integrity` sha512, an ISBN/DOI record, a publisher's own release hash), anchoring on-chain identity to the artifact the world already recognizes.
* **Deduplication at the identity layer.** Randomized SCKs make ciphertext deduplication impossible by design (§1.1). Plaintext fingerprints restore the ability to *recognize* duplicate content without weakening key secrecy — dedup of knowledge, not of bytes.

### Authentication and Verification Tree

```
                 BLOCKCHAIN
                     │
               authenticates
                     │
                     ▼
        Deployment ID (torrent-style hash)
                     │
                     ▼
                 Merkle Root
                     │
           ┌─────────┼─────────┐
           ▼         ▼         ▼
         proof     proof     proof
           │         │         │
           ▼         ▼         ▼
        chunk A   chunk B   chunk C
           │         │         │
           ▼         ▼         ▼
        AES-CTR   AES-CTR   AES-CTR
           │         │         │
           ▼         ▼         ▼
        plaintext plaintext plaintext

```

```
Peer gives me:
    encrypted chunk #N
    +
    proof path for chunk #N
    +
    authenticated manifest/root

                 ↓

        verify BLAKE3/Bao proof
        against CIPHERTEXT root

                 ↓

       "This chunk belongs to
        the canonical object."

                 ↓

        decrypt chunk

                 ↓

        verify BLAKE3/Bao proof
        against PLAINTEXT root

                 ↓

       "I was given the right key,
        and this is the canonical
        plaintext."

```

## 3. Security Considerations

* Confidentiality — unauthorized parties cannot derive the SCK from ciphertext.
* Incremental integrity — individual chunks can be verified independently without possessing the complete asset.
* Content authenticity — the expected Merkle root is bound to the canonical deployment identity.
* Decryption correctness — a client can verify that the plaintext it obtained is the canonical plaintext, independently of which key or key-provisioning path produced it.
* Authorization — only wallets satisfying the current identity/ownership policy may obtain the SCK.
* Transferability — authorization follows the on-chain entitlement rather than a permanently bound wallet.
* Cooperative revocation — the reference client ceases key use after entitlement loss.
* Seeder agnosticism — possession of encrypted chunks does not confer content access.
* Plaintext non-revocability — the protocol makes no claim to erase plaintext already obtained by a user.

## 4. Protocol Flow

### Phase 1: Encryption, Minting, and Seeding

The protocol supports both explicit content creators (Publishers) and automated, non-owner proxies ("First Finders").

1. **Canonical Identity Pre-Check:** Before performing any local cryptographic operations, the client queries the on-chain `Registry` using the asset's deterministic Web2 metadata hash.

* **If a record exists:** The client halts the First Finder workflow, fetches the official CAN infohash and SCK access conditions from the ledger, and joins the existing swarm as a standard consumer/seeder. To guarantee interoperability, the canonical on-chain record natively exposes a standardized magnet link (including the exact BTIH), allowing traditional BitTorrent clients to access the package registry and participate in the swarm without requiring custom blockchain integration.
* **If no record exists:** The client proceeds as the authorized First Finder, establishing that this is the network's initial ingestion point for the asset.

2. **Deterministic Normalization (Tarball & Dual-Hash):** For the initial NPM interception and ingestion, the First Finder does not repackage the asset; it fetches and strictly hashes the immutable `.tgz` tarball provided directly by the NPM registry as-is. While this acknowledges that a First Finder bootstrapping event relies on the centralized registry to deliver the initial object, it guarantees deterministic bytes. This canonical archive is run through a dual-hash pipeline: an identity hash (e.g., `BLAKE3(packageName @ version)`) is generated to represent the asset on the Registry, while the primary payload undergoes a separate structural hash to build the CAN streaming manifest.
3. **SCK Generation:** The symmetric content key generation is bifurcated based on the publisher's role:

* *First Finders* utilize cryptographically secure, random generation for the SCK, as the object is held in escrow and will be claimed by its rightful Web2 maintainer later.
* *Explicit Publishers / IP Owners* may derive keys deterministically from their own secure, private seed phrases for simplified key recovery and management. This derivation is **layered and domain-separated by asset**, never a bare seed-to-key mapping:

```text
    seed_phrase                       (publisher's recoverable secret, never leaves the client)
         │
         ▼
    publisher_root = KDF(seed_phrase)
         │
         ├──── asset_identity_hash    (BLAKE3(name @ version) — the canonical Registry key)
         ▼
    master_key = KDF(publisher_root, asset_identity_hash)
         │
         ├──── deployment_id          (distinguishes re-deployments of the same identity)
         ▼
    SCK = KDF(master_key, deployment_id)
```

* **Recoverability:** a publisher who retains only the seed phrase can regenerate the master key and SCK for every asset they have ever published, without any stored key material.
* **Per-asset isolation:** because `asset_identity_hash` is mixed in, every asset a publisher releases has an independent master key. Compromise of one asset's SCK does not expose any other asset by the same publisher, and does not expose the root.
* **Nonce-invariant safety (mandatory):** the `deployment_id` layer exists to guarantee the `(SCK, IV)` pair is never reused. AES-CTR catastrophically leaks plaintext XOR under keystream reuse, so a publisher re-encrypting the same asset **must** derive a fresh SCK via a new `deployment_id`, even when the plaintext and identity hash are unchanged. Deriving an SCK from `seed_phrase` alone, or from `seed_phrase` plus identity without a deployment layer, is a protocol violation.
* **Anti-derivability is preserved:** the derivation inputs are the publisher's private secret and public identifiers — never the plaintext payload. Possession of the plaintext confers no ability to compute the key (see §1.1).

4. **Payload Encryption & Hashing:** The asset is chunked to perfectly align with the CAN piece size and 16-byte block boundaries. Each chunk is encrypted using AES-CTR and structurally hashed using BLAKE3 to build the verified streaming manifest.
5. **Trustless Seeding:** The encrypted chunks and the manifest are seeded to public, unauthenticated CANs. Seeders host opaque bytes blindly.
6. **Condition-Locking & Universal Adapter Escrow:** The SCK is encrypted using the DKMN's public key and bound to an Access Control Condition (ACC). To ensure complete transferability of rights, key rotation, identity migration, and future identity resolution upgrades, **all publishers—whether explicit creators or automated First Finders—route authorization through an Abstract Identity Adapter** queried by the on-chain Identity Registry Contract (`Registry.isAuthorized(packageId, requestingWallet)`). This architecture ensures the protocol can seamlessly introduce native protocol-level identity adapters in the future with 100% backwards compatibility.

* *Explicit Publisher:* Registers via a Publisher Authority Adapter (e.g., a transferable bearer asset/ERC-721 token or cryptographic DID adapter). Copyright and publishing ownership are transferred simply by transferring the underlying authority token or updating identity resolution, without requiring protocol or asset state rewrites.
* *First Finder Escrow:* Registers via an Escrow Identity Adapter (e.g., `PackageJsonAdapter`). The DKMN holds the SCK in escrow, resolving authorization against the registry until the Web2 maintainer authenticates via the adapter and updates the registry state to claim administrative control or swap to a Publisher Authority Adapter.
* **State-Locked Escrow Registration:** If two independent non-owner nodes ("First Finders") attempt to ingest and register the same unlisted package simultaneously, the on-chain `Registry` enforces a strict State-Locked Escrow Registration. The transaction that lands first in the block wins, establishing the official package hash and escrow binding. The losing node's client catches the on-chain revert, discards its locally generated ciphertext/SCK, and automatically switches to pointing its installation pipeline to the winning node's registered CAN infohash.

7. **Minting:** Tokens or access permissions are registered on-chain as standard, transferable bearer assets.

#### Abstract Identity Adapter & Generic Content Identity

To support generic content (movies, ebooks, audio, binaries, software), the identity verification layer is decoupled via an abstract verification interface (`IIdentityAdapter`). This interface normalizes both *authentication mechanisms* (e.g., DNS, Web3 signatures) and *authorization models* (e.g., ERC-721 ownership, Multi-Sig) into a single standard: `isAuthorized(address claimant, bytes context) -> bool`.

```text
                                        +------------------------------------------------+
                                        |            Identity Registry Contract          |
                                        +------------------------------------------------+
                                                                |
                                              Calls IIdentityAdapter.isAuthorized()
                                                                |
      +----------------------------+----------------------------+-------------------------------+------------------------------------+
      |                            |                            |                               |                                    |
+-----+--------------------+ +-----+--------------------+ +-----+--------------------+ +--------+------------------+ +--------+------------------+
| Software Package Adapter | |    Web/DNS Adapter       | | Public Key / DID Adapter | | Publisher Rights Adapter  | | Enterprise DID / MultiSig |
| (NPM/git/package.json)   | | (DNSSEC / ZK-Email)      | | (Web3/Cryptographic ID)  | | (Asset NFT / Bearer Token)| | Adapter (Corporate IP)    |
+--------------------------+ +--------------------------+ +--------------------------+ +---------------------------+ +---------------------------+
   Escrow via Package ID          Escrow via Web/Email        Escrow via Blockchain             Explicit Wallet            Corporate Catalog
         (?)                             (?)                         (?)                   (Owns Publisher ERC-721)    (Governed by Multi-Sig / DNS)


```

The intended strategy for populating the Escrow pairs relies on deferred on-chain binding to keep the base architecture protocol-agnostic until deployment. The escrow models leave these targets empty (`?`) to establish structural intent without locking into a specific standard, whereas explicit publishers provide concrete implementations at execution time (despite the identity being abstracted through the adapter and transferable later).

#### Population Strategy Breakdown

To ensure security and compatibility within a smart contract environment, populating the escrow variables must utilize strict on-chain patterns rather than mutable off-chain configurations:

* **Constructor-Injected Adapters:** The specific targets (e.g., an Oracle endpoint for DNS validation or a specific cryptographic DID registry) are passed as immutable arguments to the Escrow contract's constructor via an On-Chain Factory pattern at deployment.
* **On-Chain Adapter Registry:** Rather than hardcoding target resolutions in the foundational escrow logic, the Factory queries a trusted, governance-controlled `AdapterRegistry` to resolve the current implementation address for a given protocol (NPM, DNS, Web3) before injecting it into the new Escrow. Once injected, the binding is strictly immutable to prevent registry-manipulation attacks.
* **Proxy-Based Upgradability (Optional):** If adapter logic must evolve post-deployment without forcing a migration of Escrow funds, the adapter addresses injected into the Escrow point to EIP-1967 Proxy contracts rather than static implementations.

#### Protocol Abstraction Strategy

* **v0.0.1 (Bootstrap Spec):** Implement `PackageJsonAdapter` (NPM/Git maintainer email verification).
* **v1.0+ (Generic Content Spec):**
* **Ebooks/Publications:** `ISBN/DOI` or author DNS domain (`author.com` via ZK-Email/TLSNotary proof).
* **Media (Music/Film):** ISRC/ISAN identifier registries, signature from an established artist public key, or DID.
* **Generic Domain Claim:** DNSSEC proof linking content hash to domain root.

#### Identity Resolution Lifecycle

[ First Finder Pushes Package ] ---> [ DKMN Escrows SCK via Escrow Adapter ]
|
v
[ Maintainer Proves Identity (ZK-Email/DNS) ] -> [ Registry Updates Authorization ]
|
v
[ Maintainer Upgrades Adapter ] --------------> [ Full IP Control (Transferable Asset) ]

### Phase 2: Consumption and Decryption

Access is provisioned dynamically via proof-of-possession, optimized for batch execution across complex dependency trees.

1. **The Download:** The consumer fetches the CAN manifest and begins downloading encrypted chunks, in any order, from available peers.
2. **The Chunked Batch Handshake:** To respect threshold network payload limits and prevent MPC consensus timeouts over massive dependency trees, the client checks its local keystore first, filters out already-unlocked packages, and groups remaining target hashes into optimized sliding-window batches (e.g., 20–50 items per request). The consumer signs a single aggregate authorization payload for each batch.
3. **State Verification, Concurrency, & Throttled Fetching:** DKMN nodes process each batch request against the on-chain Identity Registry via an optimized multicall view function (`Registry.isAuthorizedBatch([packageHashes], requestingWallet)`). To prevent **State Concurrency** race conditions (e.g., a user transferring a token while a batch request is inflight), DKMN nodes evaluate the view function at a universally agreed-upon, recently finalized block height. This guarantees that **DKMN Resolution** achieves threshold consensus deterministically without split-brain failures.
4. **Provisioning & Asynchronous Streaming:** As each batch of SCKs is returned by the DKMN, the client unlocks matching local CAN chunks, validates them against the BLAKE3 root hash, and feeds them into the local CAS symlink chain concurrently while the next batch resolves.

### Phase 3: Secondary Transfer and Key Destruction

When a consumer transfers the token on-chain, their entitlement to decrypt the content is revoked locally.

1. **State Monitoring:** The open-source client monitors token balances via an RPC node.
2. **Key Destruction:** Upon detecting a transfer event where the balance hits zero, the client executes its core contractual obligation: **deleting the SCK from local memory and storage.**
3. **No More Decryption:** Without the SCK, the client is mathematically incapable of decrypting any further chunks. The application’s access to the encrypted data stream halts.
4. **Plaintext Agnosticism:** The client does not act as malware. It does not attempt to flush RAM buffers of already-rendered frames, hunt down exported files, or delete user-saved plaintext.
5. **Continued Network Support:** The user deliberately retains the *encrypted* chunks in their local CAN storage. The client continues to act as a seeder, strengthening the swarm, despite the user no longer possessing the key to read the data themselves.

The protocol provides revocation of future key provisioning, not guaranteed revocation of previously provisioned plaintext or secret keys.

Reference-client key destruction is a cooperative enforcement mechanism and is not a cryptographic security boundary against modified clients.

## 5. Security Considerations

### 5.1 Seeder Agnosticism

Because payloads are chunk-encrypted and bound by a BLAKE3 tree, data at rest is opaque. Seeders (including former token holders) cannot access the content, allowing the encrypted files to scale horizontally as public infrastructure.

### 5.2 Replay Attacks

The DKMN handshake relies on timestamped or nonce-based signatures from the consumer's wallet to prevent bad actors from intercepting and replaying authorization requests.

### 5.3 Modified Clients (The "Honesty" Assumption)

A user can theoretically compile a modified version of the open-source client that disables Phase 3's Key Destruction, allowing them to save the SCK to disk indefinitely after selling the token. The protocol accepts this edge case. The system's primary directive is ensuring that *unauthorized wallets cannot obtain the SCK from the network*, and that the path of least resistance for honest users effortlessly honors creator rights without intrusive friction.

Defeating the unmodified path requires **both** a leaked key and a modified client. Two independent barriers is a meaningfully stronger position than either alone, and it is the intended enforcement posture: honest users are never inconvenienced, and dishonest users must take deliberate, visible steps.

### 5.4 Entitlement Auditability vs. SCK Traceability

These are two different properties and the protocol delivers them to different degrees. Conflating them overstates the protocol's guarantees.

* **Entitlements are 1:1 and fully auditable.** Every access right is a distinct transferable bearer asset with a unique on-chain owner and a complete transfer history. Accounting, residuals, resale, and audit of *who holds a right* are exact and trivially verifiable. This is a genuine and unusual strength.
* **The SCK is 1:many and is not, by itself, traceable.** A deployment is encrypted once with one SCK and seeded to the swarm as a single ciphertext (Phase 1.4–1.5). The DKMN provisions *that same SCK* to every wallet that satisfies the access condition (Phase 2.4). Every entitlement holder therefore receives identical key bytes.

The consequence: if a single holder publishes their SCK, those bytes decrypt the universally-distributed swarm copy for everyone, at zero marginal cost — and because many wallets legitimately hold the same value, publication of the key alone does not attribute the leak to any one of them. **On-chain provenance narrows who *requested* a key; it does not narrow who *published* one.**

This is materially different from, and worse than, the analog hole. The analog hole leaks a rendering; an SCK leak hands over the canonical asset in perpetuity.

Genuine traitor tracing requires the rendered plaintext to be recipient-specific. Applying that variation to the distributed asset would fragment the single-swarm model the distribution economics depend on. The protocol resolves this by binding variance to the **entitlement** rather than to the content, leaving the main swarm byte-identical for every holder — see §7.

### 5.5 Blast Radius Containment

Because SCK derivation is layered and domain-separated per asset and per deployment (Phase 1.3), a leaked or compromised SCK exposes exactly one deployment of one asset. It does not expose other versions of the same asset, other assets by the same publisher, the publisher's master key, or the publisher's seed phrase. Leak damage is bounded to the asset, not the catalog.

### 5.6 Escrow Claim Front-Running & Identity Proof Binding

To prevent front-running attacks during escrow settlement (where an attacker intercepts a maintainer's off-chain verification proof and submits it to claim ownership), state transitions on the `Registry` require identity proofs or ZK-nullifiers to be cryptographically bound to the claimant's target wallet address. Any settlement proof generated for `Address_A` will revert on-chain if executed by or directed to `Address_B`.

## 6. Deprecation, Advisory Flags, and Content Governance

Canonical records are immutable and swarm content cannot be recalled. The protocol therefore does not attempt takedown; it attempts **informed refusal**. Governance is advisory metadata parsed against the user's own trust set, not registry mutation.

*Framed here for architectural intent. Not implemented in the MVP.*

### 6.1 Why Not Registry Mutation

Deleting or rewriting a canonical record would break the guarantees the rest of the protocol depends on: reproducible dependency resolution, verifiable provenance, and the irrevocability of purchased access rights. A publisher who could unpublish could also revoke what users paid for — the exact behavior this protocol exists to prevent. The registry stays append-only.

### 6.2 Advisory Backlinks

Deprecation notices, malware advisories, license disputes, and content classification are expressed as **append-only metadata objects that backlink to the target content hash** — the same mechanism as comments, ratings, and reactions. A flag is a signed assertion by some identity that a given asset has some property. It carries exactly the weight of the identity that signed it.

```text
    content_hash ◄──── advisory backlink { type, severity, signer, evidence_ref }
                 ◄──── advisory backlink { ... }
                 ◄──── counter-assertion  { disputes: <advisory_id>, signer, ... }
```

Advisory types anticipated: `deprecated`, `superseded-by`, `security-advisory`, `malware`, `license-dispute`, `content-classification`, `disputed`.

### 6.3 Client Resolution

The client fetches backlinks alongside the asset and evaluates them against a **locally-configured trust set** — signers the user or their organization has chosen to heed. There is no global arbiter, and the protocol does not appoint one.

* Advisories from trusted signers surface as warnings, or block installation, per local policy.
* Enterprises point their trust set at their own security team or a vendor feed.
* Ecosystems may converge on well-known signers (a registry security team, a CVE feed) without any of them gaining protocol-level authority.
* Counter-assertions are visible too. Disputes are surfaced, not silently resolved.

### 6.4 Unwanted and Illegal Content

The same mechanism is the intended handler for content classification generally, including material that is unwanted, age-restricted, or illegal in a given jurisdiction. The protocol's position is that it cannot and should not adjudicate this globally: it can carry signed assertions, and clients can act on the assertions they trust — including refusing to fetch, refusing to seed, or refusing to display.

Two limits are stated plainly rather than papered over:

* **Seeders host opaque ciphertext and cannot inspect what they carry** (§5.1). This is a deliberate privacy and scalability property, and it means seeders cannot content-moderate by inspection. Client-side refusal-to-seed based on trusted advisories against the *identity hash* is the available lever.
* **Advisory flags do not remove content from the swarm.** Anyone running a client that ignores the trust set retains access. This is the same honesty assumption as §5.3, and the same limit applies.

Jurisdictional obligations for operators of gateways, indexers, and default trust sets are a legal question this specification does not attempt to answer. It is raised here so that the architecture is not later retrofitted under pressure.

## 7. Traceability and Per-Entitlement Variance

*Design intent. The MVP ships §7.1 only; §7.2 onward is the V2 path and is architecturally protected by the commitment structure in §2.*

### 7.1 MVP Posture: Wrapped Keys

Under the MVP, content is uniform across all holders and the per-identity key is a **wrapped SCK** — the content key encapsulated under the recipient's public key, resolved through the signature adapter. Distinct bytes per identity, unwrapping to a common SCK.

This buys a real, cheap barrier: the oracle never emits identical bytes to two identities, and a user who copies and publishes their key file gives away nothing, because no one else can unwrap it. Casual key sharing is eliminated outright. It does **not** buy attribution — a holder who extracts the unwrapped SCK from client memory produces a universal, unattributable key. Defeating the protocol therefore requires a leaked key *and* a modified client, consistent with §5.3.

### 7.2 Variance Belongs to the Entitlement, Not the Content

Forensic traceability requires that the plaintext a user renders be recipient-specific. The naive construction varies chunks within the distributed asset, which fragments the swarm and destroys the distribution economics the protocol depends on.

The protocol instead binds variance to the **entitlement layer**, where 1:1 identity already exists by construction:

* The **main swarm carries exactly one ciphertext with one infohash.** Zero fragmentation. Every seeder in the network serves byte-identical content. The distribution economics are fully preserved.
* A small **variant set** — on the order of 1% of the asset — is committed to by the entitlement record and delivered as a separate object.
* The recipient's plaintext is the invariant portion assembled with their variant set. That assembled plaintext carries their fingerprint, so attribution survives redistribution of the plaintext *and* the analog hole.

Variant chunks are load-bearing: the asset is incomplete without them, so a leaked copy necessarily carries the fingerprint of whoever it came from.

### 7.3 The Variant Object Is an Ordinary Swarm Object

The variant set is **not** stored on-chain — at ~1% of a multi-gigabyte asset it is far too large. The entitlement record commits to it by root and points at it; the bytes are a normal content-addressed torrent object, seeded and fetched like any other.

It requires no special transport, but it does require a specific key binding:

* **Encrypted to the entitlement, not to the asset.** The variant object is encrypted under a key bound to the entitlement identity, *not* under the asset's SCK. This is mandatory. If variant objects were readable by any SCK holder, any holder could read another's variant set and assemble a copy carrying someone else's fingerprint — turning a forensic mechanism into a framing weapon. Unilateral framing is a far worse failure than collusion.
* **Seeder agnosticism carries over unchanged.** Because the object is opaque to everyone but its entitlement holder, anyone may seed it harmlessly, exactly as with the main asset (§5.1). Sharing the object confers nothing without the corresponding entitlement.

**Multi-device is the motivating case and it resolves cleanly.** One wallet, *n* devices: all *n* resolve to the same entitlement identity, so all *n* unwrap the same variant object. The user's devices form a natural micro-swarm and sync among themselves over ordinary transport, with no per-device key ceremony, no re-minting, and no distinction between a user's second device and any other peer.

Two consequences to plan for:

* **Availability.** A per-entitlement object has a swarm the size of one user's device set. If those devices are offline the object may be unavailable. Either the sequencer pins variant objects, or the variant set is made deterministically regenerable from `(invariant content, entitlement id, publisher secret)` so it can be re-served on demand rather than stored in perpetuity. Regeneration is preferred; it bounds publisher storage to zero. Regeneration inputs **must** be domain-separated with the same discipline as SCK derivation (Phase 1.3), or two entitlements can collide onto the same variant set and attribution silently fails.
* **Network-level privacy.** Variant object infohashes are per-entitlement, so observing who requests or seeds a given infohash links a network address to a specific entitlement — a correlation surface distinct from, and more immediate than, the on-chain one in §8.3. Tracked there.

### 7.4 Re-Minting on Transfer

When an entitlement transfers, the variant set is **re-minted** for the new holder. Without this, a buyer's plaintext would carry the seller's fingerprint and attribution would point at the wrong party.

This is feasible because transfers are already publisher-sequenced, so the sequencing point can issue a fresh variant set as part of settlement. The concession is that resale is no longer a pure ledger write — the sequencer must serve an object. This weakens the "publisher cannot interfere with resale" property, though only to the degree that a sequencer can be *slow*; it still cannot deny a transfer, and the on-chain entitlement moves regardless.

The seller retaining their retired variant set is expected and harmless — it is the accepted plaintext-non-revocability case (§1.1), and their fingerprint correctly identifies them.

*Note on double-sale:* re-minting is not itself a double-sale prevention mechanism — entitlements are single-owner bearer assets and the ledger already prevents double-sale by construction. What re-minting adds is **forensic cleanup**: retired variant sets are bound to a specific ownership interval, so leaked content is attributable not merely to an identity but to *when* that identity held the right.

### 7.5 Collusion Resistance

Two or more holders can diff their copies to locate variant positions and splice a copy whose fingerprint matches neither. Variant sets are therefore assigned using a **collusion-resistant fingerprinting code** (Tardos or equivalent), which provides provable tracing up to a chosen collusion size *c* with a false-accusation probability bounded by a security parameter.

The cost is that variant-set size grows with *c* and with the accused-population size, so *c* is an explicit economic parameter — chosen against asset value and expected adversary resources — rather than a fixed constant.

## 8. Open Problems

Unresolved by design rather than by omission. Recorded so they are neither forgotten nor discovered late.

### 8.1 DKMN Centralization and Liveness

The threshold key-management network is the protocol's most significant compromise with its own decentralization thesis, and it is an external dependency:

* **Liveness:** if the DKMN is unavailable, nothing decrypts. For a package manager this means CI fails to install — a far lower tolerance for downtime than media playback has.
* **Collusion:** a colluding threshold of DKMN nodes can recover every SCK ever escrowed.
* **Trust:** the protocol inherits the security and governance of a network it does not control.

**The desired end state** is to eliminate the intermediary network entirely: derive or release the SCK from proof of wallet-possession-at-a-given-block, with Byzantine fault tolerance supplied by the consensus layer that already exists, rather than by a second overlay network. This is precisely the class of problem distributed consensus was built to solve, and it is the natural home for the guarantee.

No satisfactory construction is known to the authors. The obstacle is that the ledger is public: any value the chain can compute or reveal, every observer can also read, so the chain cannot itself hold a secret that only an entitled wallet can unwrap. Candidate directions worth evaluating — none adopted, none yet demonstrated adequate at this protocol's cost and latency targets:

* Witness/identity-based encryption against a chain-derived witness, where finality itself releases the decryption capability to the entitled party.
* Threshold or timelock encryption anchored to consensus randomness rather than to a standing key-holding committee.
* Proxy re-encryption keyed to the entitlement transfer, moving the trust from a persistent network to the transfer event.
* Trust-minimized fallback: multiple independent DKMN deployments with client-side quorum, treating any single network as replaceable infrastructure rather than a protocol component. This is mitigation, not a solution.

### 8.2 Residual Traceability Gaps

The traceability/swarm-coherence trade is addressed by §7 and is no longer open. What remains:

* **The MVP has no attribution at all.** §7.1's wrapped keys stop casual sharing but not a determined leaker. Anything shipped before §7.2 lands is unattributable by design.
* **Collusion parameter selection is unresolved.** *c* is an economic choice with no principled default; it needs modelling against asset value, and the variant-size cost curve needs measuring rather than assuming.
* **Variant object availability** depends on either publisher pinning or deterministic regeneration (§7.3). Regeneration is preferred but unspecified.
* **Attribution is evidence, not enforcement.** Identifying a leaker does not recall the content. The protocol's response to a traced leak is a governance and legal question (§6), not a cryptographic one.

### 8.3 Dependency Graph Privacy

Every key request is an on-chain record binding a wallet to an asset. In aggregate this publishes a permanent, correlatable map of exactly which software each participant runs, at which version.

* For individuals, it is a persistent behavioral profile that pseudonymity weakens only partially — a dependency set is close to a fingerprint, and one deanonymizing transaction retroactively unmasks the entire history.
* For organizations, it is a public inventory of their internal stack, including versions with known vulnerabilities. This is plausibly a disclosure hazard on its own and is an adoption blocker for most enterprises.

Note that this cuts against the protocol's own premise: escaping platform surveillance should not mean substituting a permanent public ledger of the same behavior for a private corporate one.

Directions to evaluate, none adopted: per-asset ephemeral wallets; blinded or private-information-retrieval key requests; batching and mixing to break the link between requester and asset; off-chain entitlement proofs that settle on-chain only in aggregate; zero-knowledge proof of entitlement that reveals neither wallet nor asset. The tension with §5.4's auditability is direct — entitlement accounting wants a legible record, and privacy wants an illegible one — and any resolution has to state which property it is sacrificing.
