# Transactable Key Protocol: Cryptographic Specification

## 1. Overview and Invariant Requirements

The Transactable Key Protocol provides a decentralized, frictionless architecture for distributing encrypted digital assets and dynamically provisioning access based on blockchain state.

### Distribution Invariants: 
* **Swarm Content is Encrypted:** No plaintext objects are exchanged in the swarm. 
* **Swarm Cyphertext Doesn't Mutate or Splinter Within a Deployment:** A deployment has exactly one cyphertext, byte-identical for every seeder and every entitlement holder, and no participant's authorization state produces a different swarm object. Multiple deployments of the same plaintext content version may coexist — a publisher re-deploying, or claiming an escrowed asset (§8.5), produces new cyphertext under a new content key while the previous deployment remains live. These are distinct swarm objects sharing a plaintext root, which is what proves they carry the same content (§2); they are not a splintering of one object, and the plaintext root is the identifier that survives the distinction.
* **Access Control is Orthogonal to Distribution and Does Not Alter the Swarm Object:** Authorization determines whether a participant may transfer canonical cyphertext into plaintext, it does not create a different canonical swarm object for that participant. 
* **Incremental / Random-Access Decryption Does Not Require Reconstructing the Entire Object:** Objects are encrypted per-piece so that video can be streamed, audio can be seeked, and files can be downloaded in parts. Only a piece must be completed for its Bao chunks to be verified and decrypted. 
* **Content Identity is Independent of Encryption Identity:** The plaintext content version must remain identifiable as the same content even when publisher identity, publisher authority, encryption keys, and entitlements change, or when deployments are reissued. 
* **Platform Independence:** No platform, application, publisher, provisioner, indexer, or other intermediary is required to discover, distribute, acquire, transfer, or exercise an entitlement beyond the protocol-defined interfaces and consensus state. Applications may provide discovery, presentation, commerce, or other value-added services, but no application may become authoritative over the underlying content, entitlement, or transfer state.
* **Intermediary Replaceability:** Any application or service providing discovery, presentation, commerce, indexing, storage, or other value-added functionality may be replaced without invalidating content identity, entitlement ownership, or the ability to obtain authorized content from the protocol.

### Economic Invariants: 
* **Access is Transactable:** The publisher mints and transfers access entitlements to users, and users can transfer the access entitlement to other users.
* **Access Entitlement Transfer is Atomic and Bounded:** The access entitlement transfer is atomic. On settlement the buyer may authorize immediately, and the seller obtains no further authorization. The seller's capability ends no later than the expiry of their current authorization window, and that maximum lag is published and bounded for the deployment.
* **Access Entitlement Transfer Does Not Transfer Decryption Capability:** The access entitlement seller's previously acquired decryption capability does not become the buyer's capability, and the seller's retained cryptographic material does not constitute continued authorization. Authorization follows entitlement; cryptographic possession does not. 
* **Non-Publisher Bootstrap:** Any member of the swarm can discover and provision content they don't own, whose ownership and access control methods are held in escrow for the owner to prove and claim at a later time. 
* **Non-Publisher Escrow of Rights:** The protocol must be able to populate new content and provision entitlements into the swarm prior to the owner arriving to claim the content, take control of its escrowed content keys, and claim its economic rights. 
* **Content Bootstrap Does Not Confer Publisher Rights to Content Bootstrapper:** Discovery, encryption, seeding, escrow creation, or maintenance of an unclaimed asset confers neither publisher ownership nor consumer access rights upon the bootstrap participant.
* **Escrow Must Preserve the Same Rights that Would Exist if the Publisher Had Been Present:** The escrow mechanism must preserve all the same rights and access patterns that would exist if the publisher had been present and in control from the beginning. Escrow is deferred ownership identification, not a substitute for ownership. 
* **Intermediary Independence:** The protocol must not require an intermediary to control discovery, distribution, access, entitlement transfer, or settlement. Intermediaries may provide value-added services, but those services must remain replaceable without invalidating content identity, access rights, or economic ownership.

### Cryptographic Authorization Invariants: 
* **Cryptography Serves Economics and Distribution:** The cryptographic mechanisms are subordinate to the protocol's economic and distribution invariants. A cryptographic construction that satisfies confidentiality while violating transactable access, canonical distribution, platform independence, or non-hostile access is non-conforming.
* **Authorization is Controlled:** Before any decryption may occur, the system cryptographically establishes that the requesting identity currently satisfies the access condition, evaluated against chain state at or above the deployment's declared settlement tier (§2). No decryption occurs that is not covered by a valid, unexpired authorization.
* **Authorization is Current:** A previous successful authorization, previous entitlement state, previously issued key, cached authorization result beyond its stated window, or prior decryption session does not prove access entitlement is current. An authorization result is current only within the bounded decryption window for which it was issued.
* **Authorization is Per-Window:** Authorization is established for exactly one bounded decryption window and authorizes no decryption outside it. The window is bounded by a settlement-reference ceiling and a volume cap, expiring at whichever is reached first. The two bounds are enforced differently and the difference is material: the **reference ceiling is externally determinable**, since the provisioning layer computes it from the settlement reference the authorization was established against, while the **volume cap is client-side**, because a provisioner cannot observe how many bytes a client decrypted with material it already holds. The cap therefore bounds keystream exposure per issuance for a conforming client and is not a boundary against a modified one — the same posture as §5.3. Both values are published protocol parameters, not implementation choices, and the security tradeoff of §8.1 attaches to the ceiling alone. Authorization for one window does not authorize any subsequent window.
* **Authorization is a Condition of Decryption, Not a Property of the Content Key:** The SCK is the cryptographic material necessary to decrypt the cyphertext, not a user's right to decrypt the cyphertext. Current authorization is a precondition to using the SCK, and a user may decrypt only while holding a valid, unexpired authorization covering that attempt — never on the basis of an expired authorization, an authorization issued for a different window, or possession of the content key itself.

### Architectural Invariants: 
* **Adapter/Interface Construction:** The protocol can and will support multiple chains, tokens, encryption schema, smart contract controllers, curves, swarm models, and more. Everything is an adapter to an interface, never hard-bound to a specific implementation detail. 
* **No Publisher or Provisioner May Selectively Withhold an Entitlement Holder's Ability to Decrypt:** Once the publisher mints and transfers an entitlement, the publisher cannot revoke the entitlement, deny its use, or prevent its transfer, and no provisioner may withhold the entitlement holder's ability to decrypt if the holder proves their possession of a valid entitlement.
* **Publisher Identity/Authority Key Rotation Does Not Version Cyphertext:** Publisher cryptographic identity is not consumer authorization state. When the publisher rotates their key, the swarm cyphertext does not change, and existing access entitlement holders do not lose their entitlement to access. 

Decisions that remain undetermined are held in the To-Do list of `docs/workplans/current/ChainTorrent MVP.md` rather than here.

### 1.1 Core Philosophy

This protocol optimizes for **frictionless distribution and access**, not hostile digital rights management (DRM).

* **The Distribution Problem:** Piracy is fundamentally a distribution and friction problem. By making legitimate, highest-quality access seamless and inexpensive, the incentive for piracy is mitigated.
* **No Hardware Enclaves:** The protocol strictly avoids proprietary Trusted Execution Environments (TEEs) or hardware-level DRM. Such mechanisms introduce platform friction, violate open-source ethos, and create centralized failure points.
* **The DRM Boundary (Content Key vs. Plaintext):** The protocol strictly governs the lifecycle of the **content key (SCK)**. The protocol accepts the "analog hole" and acknowledges that attempting adversarial, OS-level plaintext enforcement on a user-controlled device is hostile and futile.
* **Retained Plaintext is a Deliverable, Not a Leak:** A user who has decrypted content holds it, keeps it, and uses it in whatever software they choose. This is the protocol's purpose rather than a limit on it — the transient copy that evaporates when a subscription lapses is precisely the platform behavior this design exists to replace. The protocol governs the transition from cyphertext to plaintext and makes no claim over plaintext thereafter, deliberately and permanently.
* **The Application-Layer Contract:** The authorization boundary is enforced by per-window authorization, not by client behavior: when a window expires, the next window is refused unless the requesting identity still satisfies the access condition. The reference client's obligation is to discard the content key at window expiry and to hold it in memory only, never persisting it. That obligation is hygiene reinforcing the boundary, not the boundary itself. The client explicitly does *not* attempt to track, flush, or purge plaintext that has already been rendered or exported.
* **Anti-Derivability & Random Content Keys:** Content keys **must** be generated randomly per asset deployment, never derived deterministically from plaintext payloads. While deterministic key derivation would trivially solve swarm fragmentation, it completely destroys economic enforcement by allowing any possessor of plaintext to independently compute the key and bypass escrow entirely. Security and economic viability supersede naive payload deduplication.
* **Incremental Verification:** The protocol provides incremental, random-access content verification. A client can cryptographically verify an individual Bao chunk using that chunk and its Merkle authentication path, without downloading the remainder of the asset. The Merkle root is independently authenticated by the protocol's content/deployment commitment.

#### Decentralization Is Layer-Specific

"Decentralized" is not a single property of the protocol. It describes the distribution of authority and dependency at a particular layer. A system may be decentralized at one layer while necessarily relying on coordination or service at another. The protocol therefore makes no blanket claim that every function is equally decentralized; each layer has a distinct decentralization requirement.

The protocol defines decentralization across the following layers:

**Content Distribution — Decentralized**

Content distribution is decentralized when no particular server, publisher, platform, or storage provider is required to supply the content. Encrypted swarm objects are replicated among independent participants and may be obtained from any available peers. Any participant may seed an object, and loss or withdrawal of any particular seeder must not invalidate the object's identity or the rights associated with it.

The protocol does not designate an authoritative content host.

**Content Identity — Rights-Holder Asserted**

Content identity is asserted by the party claiming rights over the content. The protocol may incorporate identifiers issued by existing registries, publishers, distributors, standards organizations, or other intermediaries, including identifiers such as ISBN, IMDB identifiers, package names and versions, and similar established references.

Such identifiers are references to an identity assertion, not sources of protocol authority. The continued operation, availability, or agreement of the external registry is not required for the protocol to maintain the content identity once that identity has been established.

Multiple external identifiers may identify the same content, and the protocol may associate those identifiers with a common content identity. Conversely, an external registry's identifier does not by itself establish ownership, entitlement, or authority within the protocol.

The protocol therefore does not attempt to replace or decentralize existing identity registries. It incorporates rights-holder-provided identity into a decentralized content and entitlement system while preserving the independence of the protocol's canonical state from those registries.

**Entitlement Ownership and Transfer — Decentralized**

Entitlement ownership is decentralized when ownership and transfer are determined by the protocol's authoritative consensus state rather than by a platform, publisher, marketplace, or service provider maintaining a private database.

An entitlement may have exactly one current owner, but no intermediary owns the authority to declare who that owner is independently of the consensus state. Transfer must therefore remain valid regardless of which application, marketplace, wallet, or interface initiated or displays it.

**Transaction Ordering — Decentralized**

Transaction ordering is decentralized when no single participant can unilaterally establish the authoritative ordering of conflicting entitlement state transitions. The ordering mechanism must derive its authority from the consensus mechanism of the underlying ledger rather than from an application, marketplace, publisher, or provisioning service.

A sequencer may provide an implementation mechanism for ordering operations, but it must not thereby acquire authority over entitlement ownership or decryption authorization beyond the state transitions explicitly assigned to it by the protocol.

**Decryption Authorization — Decentralization-Targeted**

Decryption authorization is decentralized when the ability to authorize access does not depend permanently upon a single trusted intermediary or a party possessing unilateral authority to grant, deny, or selectively withhold access.

The protocol currently places authorization behind a replaceable provisioning interface. A deployment may therefore use a threshold or other provisioning network to establish authorization, but such a network is infrastructure rather than an authority over entitlement ownership. The desired end state is for authorization to derive directly from the decentralized consensus state and proof of current entitlement, eliminating the additional trusted provisioning intermediary.

Until that construction exists, the protocol must preserve the interface boundary so that the provisioning mechanism can be replaced without changing content identity, entitlement ownership, transfer semantics, or the canonical swarm object.

**Content Governance and Trust — Decentralized**

Content governance is decentralized when no global authority is required to determine whether content is acceptable, safe, deprecated, disputed, or appropriate for a particular user or jurisdiction.

The protocol therefore treats governance assertions as signed, append-only metadata rather than authoritative mutation of canonical content or entitlement state. Clients and organizations may choose which identities they trust and what actions those assertions cause locally.

A trusted signer may therefore influence a participant's behavior without acquiring protocol-level authority over the underlying content or ownership state.

**Discovery and Presentation — Decentralized**

Discovery and presentation are decentralized when no particular application, indexer, marketplace, search engine, or user interface is required to locate, interpret, access, or present protocol content.

Applications may provide discovery, recommendation, search, presentation, analytics, commerce, or other value-added services. These services are intentionally permitted to be centralized businesses. Their decentralization requirement is instead **replaceability**: no such service may become authoritative over the content, entitlement, or transaction state merely by providing an interface to it.

A user must therefore be able to move between independent applications without surrendering the underlying content identity or entitlement.

#### The Decentralization Boundary

These distinctions produce an important rule:

**The protocol does not require every participant or service to be decentralized. It requires that no participant or service become an unavoidable authority over a protocol property that the protocol defines as decentralized.**

A centralized service may therefore exist as a convenience, business, gateway, indexer, storage provider, marketplace, or application. What it may not become is the authority whose continued operation is necessary to establish canonical content identity, entitlement ownership, transaction validity, or the user's underlying rights.

Where the protocol currently requires an intermediary for a function, that dependency is identified explicitly rather than described as decentralized. Where the intermediary is placed behind a replaceable adapter, the dependency is an implementation choice rather than a protocol-level authority.

**Decentralization is therefore measured not by whether individual components have operators, but by whether those operators possess irreplaceable authority over the protocol's canonical state or whether users are forced to depend upon them to exercise rights the protocol itself is intended to provide.**

### 1.2 Terminology

The protocol carries several distinct objects that plain English collapses into the single word "key", and two distinct roles that plain English collapses into "the network" or "the sequencer". They are owned by different parties, live for different durations, and fail in different ways, so the unqualified words are not used in this specification.

#### Objects

| Term | What it is | What happens to it |
| --- | --- | --- |
| **Entitlement** | the transferable bearer asset recording who currently holds an access right to a deployment | **checked**, on-chain; it signs nothing |
| **Handshake key** | the private key a party signs with to prove control of the identity that holds an entitlement | **proven**, verified off-chain via the signature adapter |
| **Content key (SCK)** | the symmetric key that decrypts a deployment's ciphertext | **obtained** per authorized decryption window; held in memory for that window only, never persisted |
| **Master key** | the per-asset node of a publisher's derivation hierarchy, from which that asset's deployment content keys derive | **derived**, never transmitted |
| **Escrowed key** | a content key held on behalf of an absent Web2 maintainer following a First Finder ingest, pending claim | **held**, then claimed |
| **Variant object key** | the key decrypting a per-entitlement variant object; bound to the entitlement identity, never to the asset | **obtained** per authorized decryption window, under the same authorization as the content key |

An entitlement cannot sign, and a key cannot be owned on-chain. A consumer proves control of a **handshake key** to demonstrate that they hold an **entitlement**, in order to obtain a **content key** for one bounded decryption window.

#### Roles

| Role | What it does | What it does **not** do |
| --- | --- | --- |
| **Transfer sequencer** | orders entitlement transfers so that scarcity is strictly sequenced and double-sale is impossible; maintains the binding between an entitlement and its current variant object | it does not authorize decryption, hold or provision content keys, or participate in any part of the read path |
| **Provisioning layer** | establishes current authorization against chain state at the deployment's declared settlement tier and provisions content keys for one bounded decryption window; holds escrowed keys pending claim | it does not order transfers, hold entitlements, or determine who may hold one |

These two are separate by design and must not be merged. Ordering scarcity is a write-path concern that touches the ledger; establishing authorization is a read-path concern that touches every decryption. A single component doing both invites the conclusion that authorization can be settled at transfer time and presumed thereafter, which the Cryptographic Authorization Invariants forbid. The DKMN is the current implementation of the provisioning layer, never its definition.

#### Units

Plain English collapses two different granularities into "chunk". They differ by three orders of magnitude and are set by different layers, so the unqualified word is not used.

| Term | Size | Set by | Role |
| --- | --- | --- | --- |
| **Piece** | configurable, power of two, ≥ 16 KiB | the CAN | unit of transport, request, and encryption offset arithmetic |
| **Bao chunk** | 1 KiB, fixed | BLAKE3/Bao | leaf of the verification tree; the granularity at which an authentication path resolves |

Verification granularity is the Bao chunk; transport and offset granularity is the piece. Claims elsewhere in this specification about verifying "a chunk" without downloading the remainder are claims at Bao granularity, and are satisfied once the containing piece has arrived.

## 2. Cryptographic Primitives

To ensure high performance, enable out-of-order streaming, and maintain native compatibility with Content-Addressable Networks (CAN) like BitTorrent and IPFS, the protocol utilizes:

**Symmetric Encryption (Payload — Cipher Adapter):** payload encryption resolves an abstract **`IPayloadCipherAdapter`** rather than a pinned cipher, on the same reasoning that governs `ISignatureAdapter`. The interface constrains the properties the rest of the protocol depends on; it does not constrain the construction.

The adapter contract:

* **Seekable and order-independent.** Any piece decrypts from its index alone, without the preceding stream. This is what makes byte-range seeking and out-of-order swarm delivery possible.
* **Length-preserving.** Cyphertext length equals plaintext length, with no per-piece expansion. Integrity is supplied by the BLAKE3/Bao layer, so an AEAD tag per piece would be redundant overhead that also breaks the offset arithmetic.
* **Nonce-invariant per deployment.** One IV per deployment, fixed for its lifetime and published in the clear in the manifest, so the offset arithmetic holds identically for every participant.
* **Declared addressable extent.** Every adapter publishes `maxAddressableBytes`, the largest object it can encrypt under a single `(key, IV)` pair without exhausting its counter field. This is a property of the construction, not a protocol constant, and it is what the manifest bounds check below is evaluated against.

| Adapter | Construction | Counter layout | `maxAddressableBytes` | Notes |
| --- | --- | --- | --- | --- |
| `AesCtrAdapter` | AES-256-CTR | 64-bit IV ‖ 64-bit block counter | 2⁶⁸ (≈256 EiB) | MVP implementation |
| `XChaCha20Adapter` | XChaCha20 | 192-bit nonce ‖ 64-bit block counter | 2⁶⁶ (≈64 EiB) | candidate; wider nonce, no hardware dependency |

**MVP implementation (`AesCtrAdapter`).** AES-256-CTR with a randomly generated **64-bit IV** per deployment and a **64-bit block counter**, together forming the 128-bit counter block AES requires. The counter block for a given piece is `(IV << 64) | block_index`, where `block_index = (PieceIndex × PieceSize) / 16`.

* **Why 64/64 rather than 96/32.** A 32-bit counter caps a deployment at 2³² blocks — 64 GiB — which is adequate for a package tarball and inadequate for the media classes this protocol targets. A 64-bit counter removes the ceiling at the cost of IV width, which is acceptable because content keys are random per deployment (§1.1): an IV collision across two deployments is harmless when the keys differ, and there is only ever one IV within a deployment.
* **Fields, not integer addition.** The IV occupies the high 64 bits and the counter the low 64; the block index is written into the counter field and never added to the counter block as a whole. Whole-block addition would carry into the IV field, which is a different construction with different collision behavior.
* **Piece Alignment.** All CAN piece sizes are multiples of the 16-byte AES block size, which every legal BitTorrent piece size already satisfies. Only the final block of a deployment is partial, and CTR mode handles it by keystream truncation, so no padding exists anywhere in the object.
* **Nonce Invariants.** The IV is generated randomly **once per deployment**, not per file, and is stored in the clear within the CAN manifest. A deployment containing multiple files is encrypted as a single contiguous byte stream under one `(content key, IV)` pair, so file boundaries have no cryptographic meaning and the block arithmetic is continuous across them. **Per-file IVs under a shared content key are a protocol violation**, and so is any construction deriving a second IV from the first: AES-CTR leaks the XOR of plaintexts under keystream reuse, and a deployment is the unit at which nonce invariance is guaranteed. Where a fresh IV is needed, a fresh deployment is the mechanism (Phase 1.3, `deployment_id`). This rule governs the protocol's own encryption of a deployment and says nothing about what the plaintext contains.
* **Payload contents are opaque to this layer, and nesting is permitted.** A deployment's plaintext may itself hold separately encrypted material — an encrypted archive, a file carrying its own protection, or **another ChainTorrent object with its own deployment, content key, and entitlements**. The protocol encrypts a deployment as one contiguous stream and imposes no constraint whatever on structure inside that stream; independent encryption within the payload is not a second IV under this deployment's content key and does not implicate the nonce invariant above. Each nested object is a deployment in its own right, and the invariant applies independently at each layer. The entitlement consequence is the operative one: **an outer entitlement authorizes decryption of the outer cyphertext only.** It neither confers nor implies authorization for anything independently encrypted within, which resolves under its own entitlement or does not resolve at all. A container deployment can therefore be distributed, held, and seeded by parties who cannot read its constituents, and holding the container is not a claim on them.
* Payload integrity and verification are handled natively by the BLAKE3-Bao Merkle tree layer, removing the need for redundant AEAD tag overhead while allowing instantaneous seek-and-decrypt capabilities.

#### Manifest Bounds Validation

A manifest is untrusted input until it has been authenticated, and its declared geometry drives the block arithmetic above. The client therefore validates it in full **before issuing any authorization request**, and a failure is a hard cryptographic error that halts the operation:

* **Piece geometry.** Piece size is a power of two, at least 16 KiB, and a multiple of the cipher's block size. Declared total size, piece size, and piece count are mutually consistent.
* **Addressable extent.** The highest block index the object can address — `ceil(total_bytes / 16) - 1` for a 16-byte block cipher — falls within the resolved adapter's `maxAddressableBytes`. A manifest declaring an object the adapter cannot address under one `(key, IV)` pair is rejected outright rather than silently wrapping the counter, which would reuse keystream.
* **Index range.** Every piece index requested or served is less than the declared piece count. Out-of-range indices are refused at the transport layer, not resolved into offsets.

**The ordering is the security property, not the arithmetic.** Validating before provisioning means a malformed or hostile manifest can never cause a content key to be requested, so no key material is exposed to a session that was going to fail, no authorization traffic is spent on invalid input, and a manifest cannot be used as a probe against the provisioning layer.

Under `AesCtrAdapter` the extent bound is unreachable in practice — 2⁶⁸ bytes exceeds any object that could be assembled, and such a manifest fails on piece count and allocation long before the counter is implicated — so for the MVP this check is dormant. It is specified as an adapter-declared bound rather than a constant because narrower counter fields make it live: a 32-bit counter caps an object at 64 GiB, which feature-length video reaches, and an adapter carrying that layout must fail closed rather than wrap.

**Integrity & Verification:** `BLAKE3` (utilizing a Bao-style verified streaming structure) provides a native Merkle tree for Bao chunk verification. BLAKE3/Bao provides cryptographic integrity proofs for individual Bao chunks and binds those chunks to the authenticated content root, eliminating manual MAC tree management overhead and mapping directly to Merkle DAG structures (e.g., Git repositories).

**Confidentiality & Integrity Separation:** The protocol deliberately separates confidentiality from content integrity. AES-CTR provides encryption; BLAKE3/Bao provides independently verifiable content integrity and random-access authentication proofs.

**Content Commitments (Ciphertext, Plaintext, and Variant Roots):** The canonical on-chain record commits to **two** BLAKE3/Bao roots, with a **third** added per-entitlement when per-entitlement variance is enabled (§7):

* the **ciphertext root**, which lets seeders and downloaders verify opaque pieces they cannot read;
* the **plaintext root**, which lets a decrypting client verify that what it produced is the canonical plaintext, published in one of the three disclosure modes below; and
* the **variant root** (per-entitlement, carried in the entitlement record rather than the asset record), which commits to the recipient's overlay seed. Absent under the MVP's uniform-content posture.

The ciphertext root alone is insufficient. Integrity checking happens entirely at the ciphertext layer, so a client provisioned an incorrect content key — through provisioning fault, compromise, or a substituted response — produces garbage plaintext that passes every check the protocol otherwise performs. The plaintext root closes that gap.

Critically, the commitment is to the **plaintext, not to the key**. Committing to a key (e.g. publishing `BLAKE3(SCK)`) would hard-code a single universal content key and foreclose per-identity key provisioning. Committing to the plaintext constrains the *output* rather than the mechanism, so it holds unchanged under wrapped provisioning responses, per-identity subkeys, traceable decryption keys, or any future provisioning scheme. It also provides plaintext-side incremental verification that mirrors the ciphertext tree: a single decrypted Bao chunk can be validated against its authentication path without decrypting the remainder of the asset.

Because the swarm object remains complete and canonical under per-entitlement variance (§7.2), the plaintext root commits to the canonical plaintext for every holder. The overlay is applied after verification, so variance does not fork the commitment.

#### Plaintext Root Disclosure Modes

Publishing a plaintext root in the clear permits confirmation-of-content attacks against low-entropy or guessable assets: an observer holding a candidate plaintext can confirm it without holding an entitlement. This is a non-issue for public assets such as NPM packages, whose plaintext is openly distributed regardless, and it is a real exposure for short, personal, or private content.

The plaintext root is therefore published in one of three **disclosure modes**, declared in the canonical record. The mode is a property of the deployment, fixed at registration, and its absence is stated explicitly rather than inferred — the same posture the record takes toward a missing upstream attestation (§4, Phase 1.2). A consumer always knows which verification is available to it before it requests anything.

| Mode | Root published as | Who can verify | Confirmation attack |
| --- | --- | --- | --- |
| **Public** | bare BLAKE3/Bao root | anyone | possible against guessable plaintext |
| **Keyed** | BLAKE3 keyed-mode root under `KDF(content_key, "plaintext-root-v1")` | current entitlement holders | not possible without the content key |
| **Absent** | omitted | no one | not possible |

**Keyed is the recommended default for private content, and it is free.** BLAKE3's native keyed mode requires no additional field, no separate delivery, and no change to provisioning: the verification key is derived from the content key the holder already receives for the window. It concedes nothing, because any party able to compute the keyed root could already decrypt and read the plaintext directly. What it removes is precisely the outsider's ability to test a candidate.

**What lapses under Keyed.** Verification becomes holder-only, so the properties that depend on *third parties* checking equivalence no longer hold: deterministic First Finder race resolution is not externally auditable, re-deployment identity continuity is asserted rather than provable to outsiders, advisories can no longer follow the content across re-encryption by targeting the fingerprint, and the binding to a Web2 source of truth is not independently checkable. Decryption correctness — the property the root primarily exists to deliver — is fully preserved.

**What lapses under Absent.** Everything above, plus decryption correctness itself. A client provisioned an incorrect content key produces garbage that passes every remaining check the protocol performs, exactly the gap this section opens by describing. The trust model shifts to the publisher and the provisioning layer, which is the ordinary expectation for personal cloud storage and is a genuine reduction relative to the rest of this specification. Absent is appropriate where the publisher and the consumer are the same party, or where masking outweighs verification; it should not be selected by default.

**MVP posture:** public registries are Public mode without exception. Provenance and independent verifiability are the entire point of the NPM path, and the plaintext is openly distributed regardless, so there is nothing to mask.

**Key Management Network (DKMN):** A decentralized threshold cryptography network (Multi-Party Computation, e.g., Lit Protocol) to lock, escrow, and provision the content key based on on-chain conditions without ever exposing content keys to blockchain validators. It is expressed behind a key provisioning adapter so that it can be replaced if a non-threshold alternative is ever found, on the same reasoning that governs `ISignatureAdapter` and `IIdentityAdapter`.

**Settlement (Settlement Adapter):** authorization is established against chain state, and *how settled that state must be* is a declared parameter rather than a protocol constant. Confirmation counts are not comparable across chains, so the protocol expresses settlement as a small ordered tier vocabulary that every chain maps onto, resolved through an abstract **`ISettlementAdapter`**.

| Tier | Meaning |
| --- | --- |
| `INCLUDED` | in a block at the chain's own tip |
| `SOFT` | the chain's own liveness commitment — an L2 sequencer commitment, Solana `confirmed`, a committed Tendermint block |
| `HARD` | the chain's strongest self-guarantee — an EVM finalized checkpoint, Solana `rooted` |
| `SETTLED` | settled against an external security root, where one exists — an L2's proof or challenge resolution on its L1 |

```
    currentReferenceAt(tier) -> ref     // most recent reference having attained tier
    tierOf(ref)              -> tier    // how settled a given reference is now
```

A **settlement reference** is opaque and orderable — a height, a slot, a checkpoint identifier — and no chain's vocabulary for it appears above this adapter. Chains with instant finality collapse `SOFT`, `HARD`, and `SETTLED` onto one tier, which is the correct answer for such a chain rather than a modelling failure. There is deliberately no tier below `INCLUDED`: nothing authorizes off an unincluded transaction.

**Each deployment declares a `minSettlementTier` in its canonical record**, fixed at registration exactly as the plaintext root disclosure mode is. It is immutable because raising it later would silently degrade the access latency existing holders acquired; changing it means issuing a new deployment, which is how this specification handles every other change to a published object.

**Depth is proportional to value at risk, and the protocol already accepts the failure it admits.** A reversal below `SETTLED` can invalidate an authorization already issued. The resulting exposure is exactly one decryption window, of one deployment, by one identity — the next renewal is evaluated against post-reversal state and refused. That is the *same bounded quantity* §1's Atomic and Bounded invariant already publishes as acceptable when a seller retains capacity after settlement. A shallow tier therefore introduces no new class of risk; it marginally increases the frequency of a failure the protocol already tolerates by design, and the failure is forward-only, non-compounding, and self-healing, because scarcity is ordered by the entitlement ledger and never by an authorization.

The consequence is that a shallow tier is a legitimate choice rather than a concession to impatience. Requiring deep settlement universally would make decryption latency a function of chain finality, which is unusable for the interactive cases this protocol targets — a CI pipeline installing dependencies cannot wait for an L1 challenge period, while a viewer starting a high-value film reasonably can.

---

Any implementation behind the provisioning adapter, threshold or otherwise, is bound by two obligations that are not optional properties of a particular construction:

* **Per-window authorization.** The implementation establishes current authorization for each decryption window it provisions, and provisions nothing outside a window. Authorization is established at the deployment's declared settlement tier **exactly** — not weaker, which would violate the declaration, and not stronger, because reading deeper for one holder than another is discretion on the read path, which is selective withholding under another name. Adapters may vary *how* authorization is established; they may never vary *whether* it is. An implementation that provisions once and presumes authorization forward is non-conforming regardless of its other merits.
* **No selective withholding.** The obligation to provision for a valid, unexpired entitlement is non-discretionary and mechanical. No provisioner may decline a particular holder or a particular transfer, which would constitute revocation by inaction.

**Signatures (Signature Adapter):** signatures are produced and verified per layer, each layer resolving an abstract **`ISignatureAdapter`** rather than any one pinned scheme. The chain layer uses the adapter its target chain requires; the handshake and content-signing layers use the protocol's preferred scheme independently of that choice. The curve is an implementation detail of the layer and the chain, not a property of the protocol.

```
    isValidSignature(pubkey, message, signature) -> bool
    recoverSigner(message, signature)            -> address
    canonicalAddress(pubkey)                     -> address
```

| Adapter | Curve / Scheme | Target |
| --- | --- | --- |
| `Secp256k1Adapter` | secp256k1 / ECDSA | EVM chains — chain layer only (MVP) |
| `Ed25519Adapter` | Ed25519 / EdDSA | protocol-preferred scheme; handshake and content-signing layers (MVP); chain layer on Solana, Near, Cosmos-family |
| `Secp256r1Adapter` | secp256r1 / P-256 | passkeys, WebAuthn, platform authenticators [^enclave] |
| `BlsAdapter` | BLS12-381 | aggregate/threshold signatures |

* **Per-layer resolution:** the signature adapter is resolved independently at each layer where a signature is produced or verified. A deployment is not restricted to a single scheme — the chain layer uses whatever the target chain mandates, while the handshake and content-signing layers retain the protocol's preferred scheme regardless of chain.
* **Binding, not matching:** where two layers use different keys, those keys must be provably the same principal. The Registry holds a binding attestation — the subordinate key signed by the authoritative chain identity — established once and thereafter verifiable by any party. Keys unbound across layers are a protocol violation.
* **Relationship to `IIdentityAdapter`:** the two are distinct and compose. The signature adapter answers *"is this signature cryptographically valid?"*; the identity adapter answers *"is this validated claimant authorized for this asset?"* Authentication and authorization stay separable, so a new chain requires a new signature adapter without touching authorization logic, and a new authorization model requires no cryptographic changes.
* Adapter resolution follows the same on-chain patterns as identity adapters — constructor injection via factory, resolved from a governance-controlled `AdapterRegistry`, immutable once bound.

[^enclave]: Using a platform authenticator to hold a signing key is unrelated to §1.1's rejection of hardware DRM. The protocol never requires special hardware to decrypt or render content; where a user's identity key happens to live is their choice and constrains no one else.

**Key Agreement (Key Agreement Adapter):** wrapping a content key to a recipient is an *encryption* operation and resolves an abstract **`IKeyAgreementAdapter`**, never the signature adapter. The two are distinct primitives over distinct keys: a signature key proves authorship and cannot receive a wrapped secret, and treating them as interchangeable is a category error rather than an implementation shortcut. Ed25519 signing keys are not encryption keys; secp256k1 requires an integrated encryption scheme. The binding is already expressed in the Binding Schema below, where a DID Document distinguishes `authentication` from `keyAgreement` — the signature adapter resolves the former, the key agreement adapter the latter.

```
    wrapTo(recipientPubkey, plaintext)   -> cyphertext
    unwrap(recipientPrivkey, cyphertext) -> plaintext
```

| Adapter | Scheme | Paired signature scheme |
| --- | --- | --- |
| `X25519Adapter` | X25519 + HKDF + AEAD (HPKE-style) | Ed25519 |
| `EciesSecp256k1Adapter` | ECIES over secp256k1 | secp256k1 |

#### Binding Schema

The binding between an identity's per-layer keys is expressed as a **W3C DID Document**. A DID Document already carries a `verificationMethod` set holding multiple keys of different types, and *verification relationships* — `authentication`, `assertionMethod`, `keyAgreement`, `capabilityInvocation` — that state which key serves which purpose. That is per-layer key resolution as a standard, multi-key by construction and chain-agnostic by design, so the binding is not a bespoke registry field and layer selection is a verification relationship rather than an invention of this protocol.

Storage is hybrid, because the binding is consulted from two places with opposite cost profiles:

* **On-chain:** the fields on-chain logic must read — the bound handshake public key and its scheme — are stored on-chain, roughly two storage words. Escrow claim verification (§5.6) and authorization checks run inside contracts, and a contract cannot read an off-chain document without an oracle or a proof system, which would reintroduce the intermediary class this design removes.
* **Anchored:** the full DID Document is committed by hash on-chain and resolved off-chain. This keeps the schema extensible without contract changes and keeps storage cost at one word.

The chain holds state and pointers to actuals; large objects that are merely *identified* stay off-chain.

Alternatives considered, recorded so the choice can be attacked on review:

| Alternative | Fit | Why not primary |
| --- | --- | --- |
| UCAN / ZCAP-LD | capability delegation chains; the chain key delegates handshake authority | closer to the transferable-rights framing, but delegation semantics exceed what a key binding needs |
| Session keys (AA wallets) | a subordinate key authorized by a master key for a scope and duration | the ergonomics are well-trodden and this is effectively what the handshake key is; lacks a standard document format for multiple simultaneous purposes |
| Sigstore (Fulcio + Rekor) | ephemeral key bound to an identity, recorded in an append-only transparency log | philosophically close to the append-only registry, and the model the software supply chain is converging on; oriented to short-lived signing identities rather than durable multi-layer bindings |
| ERC-1271 / ERC-4337 | the chain identity is a contract declaring arbitrary signature validity | solves a different problem — validity rather than binding — and is EVM-specific |
| X.509 / SSH CA certificates | subordinate key signed by an authority | the structural ancestor of the attestation, but the PKI trust model does not fit a bearer-asset protocol |

### The Hash-Card (Deployment Descriptor)

The content commitments are fields of a larger descriptor. The `.torrent` file is the present-day carrier; the **hash-card** is its intended successor.

**Compatibility posture.** Full backwards compatibility with existing BitTorrent clients is a hard MVP requirement, so the canonical on-chain record natively exposes a standardized magnet link and BTIH (see Phase 1.1). Unmodified clients participate in the swarm with no blockchain integration. The hash-card is therefore introduced as a *superset*: everything a `.torrent` carries, plus the fields a legacy format has no place to put. Legacy clients read the subset they understand; protocol clients read the whole card.

**Two integrity structures, one authority.** A legacy client verifies pieces against the BitTorrent v1 SHA-1 piece hashes and cannot join a swarm without them, so a compatible deployment publishes both those hashes and the BLAKE3/Bao tree over the same bytes. The duplication is off-chain and small — twenty bytes per piece in the `.torrent` — and the on-chain record carries only the BTIH. **The protocol does not inherit SHA-1's collision weakness.** The Bao cyphertext root is the sole authority for what the canonical object is; legacy piece hashes are transport-layer compatibility and nothing else. A piece satisfying SHA-1 but failing its Bao authentication path is rejected by every protocol client, so a legacy client can circulate bad bytes but can never establish canonical ones. Alignment imposes no additional constraint, since every legal BitTorrent piece size is already a multiple of the 16-byte cipher block.

| Field | Present in `.torrent`? | Purpose |
| --- | --- | --- |
| Identity hash — `BLAKE3(name @ version)` | no | canonical Registry key |
| Ciphertext Bao root | partly (piece hashes) | verify opaque pieces while seeding |
| **Plaintext fingerprint (Bao root)** | **no** | verify decryption; identify content across encryptions |
| Payload cipher IV, piece size, alignment | no | deterministic block arithmetic across the swarm |
| Registry pointer / ACC reference | no | where authorization is resolved |
| Upstream attestation (source-dependent) | no | bind to the artifact the ingest source served, where that source provides one |
| Advisory backlink root | no | entry point for §6 governance metadata |
| Magnet / BTIH | yes | legacy client interoperability |

#### What the Plaintext Fingerprint Buys

The plaintext fingerprint is referenced elsewhere in this specification as a means of proving that ciphertext and plaintext correspond. That is the smallest of its uses. Because it is an identifier for *content* rather than for any particular encryption of that content, it provides:

* **Decryption correctness.** The client can prove it was provisioned the right key and produced canonical output, incrementally and per Bao chunk (see above).
* **Encryption-independent content identity.** Two deployments with different content keys produce entirely different ciphertext but the *same* plaintext root. Identical content is therefore provably identical across independent encryptions.
* **Deterministic resolution of First Finder races.** When two nodes ingest the same asset concurrently, the loser of State-Locked Escrow Registration can *prove* its discarded object was the same content rather than trusting the identity hash alone — and any third party can verify the equivalence.
* **Re-deployment without loss of identity.** A publisher issuing a new deployment of an asset produces new ciphertext whose plaintext root is unchanged, demonstrating that nothing about the content changed. Because rotation is versioning rather than replacement (Phase 1.3), the previous deployment remains live alongside it, and the shared plaintext root is what proves the two carry the same content. Migration becomes verifiable rather than asserted.
* **Advisories that follow the content.** A malware or deprecation flag (§6) targeting the plaintext fingerprint remains attached across re-encryptions, re-deployments, and re-uploads under new identities. Flags cannot be shed by repackaging — which is the single most common evasion in existing package ecosystems.
* **Binding to Web2 sources of truth.** The plaintext root is checkable against the upstream artifact (the NPM `integrity` sha512, an ISBN/DOI record, a publisher's own release hash), anchoring on-chain identity to the artifact the world already recognizes.
* **Deduplication at the identity layer.** Randomized content keys make ciphertext deduplication impossible by design (§1.1). Plaintext fingerprints restore the ability to *recognize* duplicate content without weakening key secrecy — dedup of knowledge, not of bytes.

### Authentication and Verification Tree

```
                        BLOCKCHAIN
                            │
              ┌─────────────┴─────────────┐
        authenticates                  records
              │                            │
              ▼                            ▼
    Deployment ID (torrent-style)     Entitlement
              │                            │
      ┌───────┴───────┐                    ▼
      ▼               ▼            current authorization
 CIPHERTEXT root  PLAINTEXT root    for THIS window
      │               │                    │
┌─────┼─────┐         │                    │
▼     ▼     ▼         │                    │
proof proof proof     │                    │
│     │     │         │                    │
▼     ▼     ▼         │                    │
piece piece piece     │                    │
A     B     C         │                    │
│     │     │         │                    │
└─────┴─────┴─────────┼────────────────────┘
            │         │
            ▼         │
        AES-CTR ◄─────┼──── content key, provisioned
            │         │      for this window only
            ▼         │
        plaintext ────┘
            │
            ▼
    verified against
     PLAINTEXT root

```

Two roots and one gate. The ciphertext root authenticates pieces a seeder cannot read; the plaintext root authenticates what a decryption produced. Between them sits current authorization for the window in which the attempt falls — possession of the content key is not a substitute for it (§1).

```
Peer gives me:
    encrypted piece #N
    +
    proof path for piece #N
    +
    authenticated manifest/root

                 ↓

        verify BLAKE3/Bao proof
        against CIPHERTEXT root

                 ↓

       "This piece belongs to
        the canonical object."

                 ↓

        current authorization for
        this decryption window?

                 ↓

        decrypt piece

                 ↓

        verify BLAKE3/Bao proof
        against PLAINTEXT root

                 ↓

       "I was given the right key,
        and this is the canonical
        plaintext."

```

## 3. Security Considerations

* Confidentiality — unauthorized parties cannot derive the content key from ciphertext.
* Incremental integrity — individual Bao chunks can be verified independently without possessing the complete asset.
* Content authenticity — the expected Merkle root is bound to the canonical deployment identity.
* Decryption correctness — a client can verify that the plaintext it obtained is the canonical plaintext, independently of which key or key-provisioning path produced it. Available under the Public and Keyed plaintext root disclosure modes (§2); forgone under Absent, which is the cost that mode pays for masking.
* Authorization — only identities satisfying the current identity/ownership policy may obtain **or use** the content key.
* Per-window authorization — authorization is established for one bounded decryption window and confers no capability outside it; possession of the content key is not authorization.
* Bounded windows — every authorization carries an explicit expiry, bounded by a settlement-reference ceiling and a volume cap, so the maximum interval between a transfer and the seller's loss of capability is stated and bounded per deployment rather than left as an implementation artifact.
* Bounded reversal exposure — where the declared settlement tier permits reversal, an authorization issued against reverted state exposes at most one window, of one deployment, to one identity, and the next renewal is refused. See §5.2.
* Transferability — authorization follows the on-chain entitlement rather than a permanently bound identity.
* Enforced revocation — after entitlement loss the next authorization request fails; revocation is a property of the provisioning boundary, not of client cooperation.
* Seeder agnosticism — possession of encrypted pieces does not confer content access.
* Plaintext non-revocability — the protocol makes no claim to erase plaintext already obtained by a user.

## 4. Protocol Flow

### Phase 1: Encryption, Minting, and Seeding

The protocol supports both explicit content creators (Publishers) and automated, non-owner proxies ("First Finders").

1. **Canonical Identity Pre-Check:** Before performing any local cryptographic operations, the client queries the on-chain `Registry` using the asset's deterministic Web2 metadata hash.

* **If a record exists:** The client halts the First Finder workflow, fetches the official CAN infohash and content key access conditions from the ledger, and joins the existing swarm as a standard consumer/seeder. To guarantee interoperability, the canonical on-chain record natively exposes a standardized magnet link (including the exact BTIH), allowing traditional BitTorrent clients to access the package registry and participate in the swarm without requiring custom blockchain integration.
* **If no record exists:** The client proceeds as the authorized First Finder, establishing that this is the network's initial ingestion point for the asset.

2. **Ingest & Deterministic Normalization (Dual-Hash):** The First Finder acquires the asset through an **ingest source adapter** (`IIngestSourceAdapter`), which resolves where the bytes originate. NPM is the MVP implementation; pnpm, Bun, PyPI, crates.io, a DOI resolver, or a publisher's own release endpoint are peer implementations rather than special cases.

The adapter's contract is source-independent, and it is the contract rather than any particular source that the protocol depends on:

* **Immutable, deterministically-hashable bytes.** The First Finder does not repackage the asset; it fetches and strictly hashes the artifact exactly as served. For the NPM implementation that is the immutable `.tgz` tarball, taken as-is.
* **Upstream integrity attestation, where the source provides one.** NPM's `integrity` sha512 is one instance of this field, not its definition — an ISBN/DOI record, a signed release hash, or a transparency-log entry serve the same role for other sources. Where a source offers none, the canonical record states its absence rather than implying an attestation exists.

A bootstrapping event necessarily originates from some pre-existing source. That is a property of bootstrapping itself, not a dependency on any particular registry, and **once the object is in the swarm its canonical reference is the on-chain record**. The ingest relationship ends at registration and is never consulted again.

The adapter faithfully ingests what the source served. Whether that content is *safe* is an advisory-layer question (§6) and never an ingest-layer one: the attestation binds the canonical record to the artifact the upstream source published, and makes no claim whatever about that artifact's contents. The protocol has no control over what a source publishes and does not pretend otherwise.

#### Ingest Source Eligibility

The adapter interface is source-independent, but the *selection* of sources is not a free choice. Non-Publisher Bootstrap (§1) permits a First Finder to ingest an asset whose owner has not arrived, and Content Bootstrap Does Not Confer Publisher Rights (§1) settles what the bootstrapper gains by doing so — nothing. Neither invariant addresses whether the bootstrapper was entitled to *acquire* the bytes in the first place. Escrow is deferred ownership identification; it is not an acquisition license.

**Until §8.5 is resolved, ingest adapters target archives whose content is already free to use.** This is a constraint on adapter selection, not on adapter design: the interface, the attestation contract, and the as-is hashing rule are unchanged, and a future adapter pointed at a licensed catalog is a policy decision rather than an engineering one.

Two independent reasons hold the line:

* **Exposure concentrates in the acquisition path.** The swarm carries opaque ciphertext (§5.1), entitlements are auditable bearer assets (§5.4), and transfer is ordered and irrevocable — none of these is where a rights holder's claim lands. The claim lands on how the bytes were obtained. This is the distinction courts have already drawn against the machine-learning corpora built over the last decade: transformative *use* of lawfully acquired works has been sustained, while wholesale acquisition from shadow libraries has not. A protocol whose entire ingest surface is one adapter interface should not discover this after the interface has shipped against a paid catalog.
* **A free archive makes the post-claim pricing paradox inert.** §8.5 turns on a claimant arriving to find their asset already circulating under grandfathered $0.00 entitlements they cannot reprice. Where the content was free to use before ingest, the arriving maintainer was denied no revenue, so the paradox has no economic content and the escrow model can be proven in production without it. Bootstrapping against a permissively-licensed archive is therefore load-bearing rather than incidental to the MVP's choice of NPM.

The reciprocal follows: an adapter aimed at content that is *not* already free to use may not ship until §8.5 has an answer, because such an adapter converts an unresolved economic question into an unbargained taking. The eligibility rule is what keeps that crossing deliberate rather than emergent.

The acquired archive is run through a dual-hash pipeline: an identity hash (e.g., `BLAKE3(packageName @ version)`) is generated to represent the asset on the Registry, while the primary payload undergoes a separate structural hash to build the CAN streaming manifest.
3. **Content Key Generation:** Content key generation is bifurcated based on the publisher's role:

* *First Finders* utilize cryptographically secure, random generation for the content key, as the object is held in escrow and will be claimed by its rightful Web2 maintainer later.
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
* **The derivation seed is not the publisher's identity key.** The seed phrase is a local secret that never leaves the client and exists solely to derive content keys; the publisher's chain identity and publishing authority are separate objects held by the identity adapter and transferable independently (Phase 1.6). Rotating the identity or authority key therefore changes nothing about key derivation and does not version cyphertext, which is what the corresponding §1 invariant asserts. The converse is the real constraint: **the seed is a recovery secret, not a rotatable credential.** A publisher who discards a seed loses the ability to re-derive content keys for every asset derived under it, and a publisher adopting a new seed derives future assets only. Neither operation affects existing entitlements, deployments, or swarm objects.

4. **Payload Encryption & Hashing:** The asset is divided into pieces aligned to the CAN piece size and 16-byte block boundaries. Each piece is encrypted through the payload cipher adapter (§2) and structurally hashed using BLAKE3 to build the verified streaming manifest.
5. **Trustless Seeding:** The encrypted pieces and the manifest are seeded to public, unauthenticated CANs. Seeders host opaque bytes blindly.
6. **Condition-Locking & Universal Adapter Escrow:** The content key is encrypted to the **provisioning adapter's** key-agreement public key, via the key agreement adapter (§2) — the DKMN being the provisioning adapter's current implementation, never its definition — and bound to an Access Control Condition (ACC). To ensure complete transferability of rights, key rotation, identity migration, and future identity resolution upgrades, **all publishers—whether explicit creators or automated First Finders—route authorization through an Abstract Identity Adapter** queried by the on-chain Identity Registry Contract (`Registry.isAuthorized(packageId, requestingIdentity)`). This architecture ensures the protocol can seamlessly introduce native protocol-level identity adapters in the future with 100% backwards compatibility.

* *Explicit Publisher:* Registers via a Publisher Authority Adapter (e.g., a transferable bearer asset/ERC-721 token or cryptographic DID adapter). Copyright and publishing ownership are transferred simply by transferring the underlying authority token or updating identity resolution, without requiring protocol or asset state rewrites.
* *First Finder Escrow:* Registers via an Escrow Identity Adapter (e.g., `PackageJsonAdapter`). The provisioning layer holds the escrowed key, resolving authorization against the registry until the Web2 maintainer authenticates via the adapter and updates the registry state to claim administrative control or swap to a Publisher Authority Adapter.
* **State-Locked Escrow Registration:** If two independent non-owner nodes ("First Finders") attempt to ingest and register the same unlisted package simultaneously, the on-chain `Registry` enforces a strict State-Locked Escrow Registration. The transaction that lands first in the block wins, establishing the official package hash and escrow binding. The losing node's client catches the on-chain revert, discards its locally generated ciphertext/SCK, and automatically switches to pointing its installation pipeline to the winning node's registered CAN infohash.

7. **Minting:** Entitlements are registered on-chain as standard, transferable bearer assets.

**Variant seed authorship is deliberately absent from this phase.** Under per-entitlement variance (§7.2) a deployment would also establish how each holder's variant seed is authored. Who authors it, from what material, and how the escrow case is served when the publisher is absent are unresolved and recorded in §8.3. Nothing in Phase 1 should be read as settling them.

#### Abstract Identity Adapter & Generic Content Identity

To support generic content (movies, ebooks, audio, binaries, software), the identity verification layer is decoupled via an abstract verification interface (`IIdentityAdapter`). This interface normalizes both *authentication mechanisms* (e.g., DNS, Web3 signatures) and *authorization models* (e.g., ERC-721 ownership, Multi-Sig) into a single standard: `isAuthorized(address claimant, bytes context) -> bool`.

```text
                                                 +------------------------------------------------+
                                                 |            Identity Registry Contract          |
                                                 +------------------------------------------------+
                                                                          |
                                                        Calls IIdentityAdapter.isAuthorized()
                                                                          |
              +-----------------------------+-----------------------------+-----------------------------+-----------------------------+
              |                             |                             |                             |                             |
+-----------------------------+ +-----------------------------+ +-----------------------------+ +-----------------------------+ +-----------------------------+
| Software Package Adapter    | |     Web/DNS Adapter         | | Public Key / DID Adapter    | | Publisher Rights Adapter    | | Enterprise DID / MultiSig   |
| (NPM/git/package.json)      | | (DNSSEC / ZK-Email)         | | (Web3/Cryptographic ID)     | | (Asset NFT / Bearer Token)  | | Adapter (Corporate IP)      |
+-----------------------------+ +-----------------------------+ +-----------------------------+ +-----------------------------+ +-----------------------------+
    Escrow via Package ID           Escrow via Web/Email          Escrow via Blockchain              Explicit Identity             Corporate Catalog
             (?)                             (?)                           (?)                   (Owns Publisher ERC-721)   (Governed by Multi-Sig / DNS)


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

[ First Finder Pushes Package ] ---> [ Provisioning Layer Escrows Content Key via Escrow Adapter ]
|
v
[ Maintainer Proves Identity (ZK-Email/DNS) ] -> [ Registry Updates Authorization ]
|
v
[ Maintainer Upgrades Adapter ] --------------> [ Full IP Control (Transferable Asset) ]

### Phase 2: Consumption and Decryption

Access is provisioned dynamically via proof-of-possession, optimized for batch execution across complex dependency trees. Two distinct operations run in this phase and must not be conflated: **entitlement acquisition**, which may be skipped when the identity already holds the entitlement, and **decryption authorization**, which may never be skipped, cached, or presumed.

1. **The Download:** The consumer fetches the CAN manifest and begins downloading encrypted pieces, in any order, from available peers.
2. **Entitlement Acquisition Check:** The client inspects its own **wallet** and filters out every target for which it already holds an entitlement, because those entitlements need not be acquired again. This filter concerns *acquisition only*. It never determines whether decryption may proceed.
3. **Per-Window Decryption Authorization:** For each decryption window, the client signs an aggregate authorization payload and requests provisioning. There is no local keystore, no cache of prior results, and no exemption for assets decrypted previously — an entitlement held for years authorizes nothing until the current window is established. Batching applies to authorization *requests*; it never substitutes for them. Requests are grouped into sliding-window batches across dependency trees; batch size is negotiated in the handshake against the provisioning implementation's payload and consensus limits rather than fixed by this specification, since those limits are properties of the implementation behind the adapter.
4. **State Verification & Window Determination:** Provisioning nodes evaluate each batch against the on-chain Identity Registry via an optimized multicall view function (`Registry.isAuthorizedBatch([packageHashes], requestingIdentity)`), at a universally agreed-upon settlement reference meeting the deployment's declared `minSettlementTier` (§2). That reference is the basis of the authorization itself, not merely a concurrency device: it fixes the state the authorization was established against, guarantees threshold consensus resolves deterministically without split-brain failures, and anchors the window's reference ceiling. The window expires at that ceiling or at the volume cap, whichever is reached first.

The view returns three states because authorization is transactional against evolving state and "not settled yet" is not the same answer as "no". During an install the entitlement is frequently minted seconds before it is read, so the client is reading state it has just written:

| Result | Meaning | Client behavior |
| --- | --- | --- |
| `AUTHORIZED` | the entitlement resolves to the requesting identity at the required tier | proceed to provisioning |
| `PENDING_SETTLEMENT` | the entitlement is visible but has not attained the required tier | retry under defined backoff; not an error |
| `DENIED` | the entitlement does not resolve to the requesting identity | terminal; no retry |

Collapsing `PENDING_SETTLEMENT` into `DENIED` fails valid installs; collapsing it into `AUTHORIZED` authorizes against state below the declared tier. A client that cannot distinguish them either aborts a legitimate acquisition or retries an invalid one indefinitely.
5. **Provisioning & Asynchronous Streaming:** As each batch is provisioned, the client unlocks matching local CAN pieces, validates them against the BLAKE3 roots, and feeds them into the local CAS symlink chain concurrently while the next batch resolves. The content key is held in memory for the window only and is never written to disk. Plaintext written to the local CAS persists by design and is outside the authorization boundary (§5.3, §7.6).
6. **Window Expiry and Renewal:** On expiry the client discards the content key and requests a new window if decryption is to continue. Renewal is an ordinary authorization request and is refused if the entitlement no longer holds.

### Phase 3: Secondary Transfer and Authorization Expiry

When a consumer transfers the entitlement on-chain, their capacity to decrypt ends at the expiry of their current authorization window. No local action is required for this, and none is relied upon.

1. **Enforced at the Boundary:** The former holder's next authorization request is refused because the on-chain entitlement no longer resolves to their identity. Revocation is a property of the provisioning boundary, not of client behavior, and is not detected, negotiated, or self-reported by the client.
2. **Bounded Lag:** Between settlement and expiry of the seller's outstanding window, the seller retains the capacity to decrypt within that window. This interval is the maximum lag referenced by the Atomic and Bounded invariant (§1). The buyer may authorize immediately on settlement and is not made to wait for the seller's window to lapse.

The lag has two components. The first is the window ceiling. The second is the latency of the deployment's declared `minSettlementTier` (§2) on the chain the adapters resolve to — a parameter the publisher chooses, not a property the protocol merely suffers. Because the tier is declared in the canonical record and the adapter maps it onto that chain, the maximum lag is a **published per-deployment observable** rather than an artifact of whichever chain was used. The protocol's obligation is that both components are published and bounded for a given deployment, not that either takes a particular value.
3. **Key Discard:** At window expiry the reference client discards the content key from memory. Because the key is never persisted, there is nothing to erase from storage. This is hygiene that narrows the window of exposure to memory extraction; it is not the boundary.
4. **Optional State Monitoring:** A client may observe entitlement transfers via an RPC node in order to discard early and to present accurate state to the user. This is a user-experience affordance and correctness does not depend on it.
5. **Plaintext Agnosticism:** The client does not act as malware. It does not attempt to flush RAM buffers of already-rendered frames, hunt down exported files, or delete user-saved plaintext.
6. **Continued Network Support:** The user deliberately retains the *encrypted* pieces in their local CAN storage. The client continues to act as a seeder, strengthening the swarm, despite the user no longer being able to obtain authorization to read the data themselves.

The protocol provides revocation of future content key provisioning and future decryption of cyphertext, bounded by the current window, and makes no claim to revoke previously decrypted plaintext retained by the user.

Reference-client key discard is a cooperative hygiene measure and is not the security boundary; the boundary is the refusal to provision a new window.

## 5. Security Considerations

### 5.1 Seeder Agnosticism

Because payloads are piece-encrypted and bound by a BLAKE3 tree, data at rest is opaque. Seeders (including former entitlement holders) cannot access the content, allowing the encrypted files to scale horizontally as public infrastructure.

### 5.2 Replay Attacks

The provisioning handshake relies on timestamped or nonce-based signatures from the consumer's handshake key to prevent bad actors from intercepting and replaying authorization requests.

Replay protection is load-bearing under per-window authorization rather than merely prudent. Each authorization payload **must** be bound to the specific window it establishes — its settlement reference and its expiry bounds — so that a captured payload cannot be replayed to manufacture a fresh window after the original has lapsed. A replayable authorization is indistinguishable from an unbounded one, and would silently reintroduce the presumption of forward authorization that the invariants forbid.

#### Settlement Reversal Is Not Replay

Two adjacent failures are frequently conflated and must not be. **Replay** is an adversary resubmitting a captured payload to obtain a capability they were never issued, and it is defeated by the binding above. **Settlement reversal** is a reference below `SETTLED` being reverted after an authorization was validly established against it. Nothing is captured and nothing is resubmitted; the authorization was correctly issued against state that subsequently ceased to exist.

The distinction matters because the remedies differ entirely. Tighter payload binding shrinks the replay surface and does nothing whatever to the reversal surface, which is governed only by the declared settlement tier (§2).

Reversal is not recallable — the content key is already in the holder's memory — and the protocol does not attempt to recall it. It is instead bounded: the exposure is one window, of one deployment, to one identity, and every subsequent renewal is evaluated against post-reversal state and refused. This is the identical bound §1's Atomic and Bounded invariant already publishes for a seller's retained capacity after transfer, so a deployment selecting a shallow tier accepts a higher frequency of an already-accepted failure rather than a novel one.

The same reasoning distinguishes reversal from the settlement lag of Phase 3.2, which is likewise not a vulnerability but a published property: in that case the *legitimate prior holder* uses an authorization that has not yet expired. Neither is an attack, and neither belongs in the same class as replay.

### 5.3 Modified Clients (The "Honesty" Assumption)

A user can theoretically compile a modified version of the open-source client that retains the content key past window expiry, allowing them to keep decrypting after selling the entitlement. The protocol accepts this edge case. The system's primary directive is ensuring that *an identity holding no entitlement cannot obtain the content key*, and that the path of least resistance for honest users effortlessly honors creator rights without intrusive friction.

Defeating the unmodified path requires **both** a leaked content key and a modified client. Two independent barriers is a meaningfully stronger position than either alone, and it is the intended enforcement posture: honest users are never inconvenienced, and dishonest users must take deliberate, visible steps.

This scope is a design commitment, not a concession. **Decrypted plaintext must run on any compatible device and its existing software** — a video plays in any player, a package installs with any toolchain, a book opens in any reader. The protocol will not require a bespoke runtime, and §1.1's rejection of hardware enclaves is the same commitment stated at the hardware layer. Content that only functions inside an environment the protocol controls is precisely the walled garden this design exists to dismantle.

The consequence follows directly: because the plaintext is deliberately released into general-purpose software the protocol does not control, the invariants of §1 bind the provisioning layer and conforming clients, and nothing else. They do not bind arbitrary software running on a user-controlled device, and no claim is made that they could.

### 5.4 Entitlement Auditability vs. Content Key Traceability

These are two different properties and the protocol delivers them to different degrees. Conflating them overstates the protocol's guarantees.

* **Entitlements are 1:1 and fully auditable.** Every access right is a distinct transferable bearer asset with a unique on-chain owner and a complete transfer history. Accounting, residuals, resale, and audit of *who holds a right* are exact and trivially verifiable. This is a genuine and unusual strength. The 1:1 property is a statement about *ownership*, not about instantaneous decryption capacity: because a seller retains capacity until their outstanding window lapses (Phase 3.2) while the buyer authorizes immediately, the number of parties able to decrypt can transiently exceed the number of entitlement holders, bounded by the same published window. The ledger remains exact; the capacity overlap is a settlement property, not an accounting one.
* **The content key is 1:many.** A deployment is encrypted once with one content key and seeded to the swarm as a single ciphertext (Phase 1.4–1.5). Every entitlement holder is provisioned that same content key, wrapped to their own public key for transport. The wrapping bytes differ per identity and per window; the key inside does not.

Three leak classes with three different outcomes:

| What leaks | Traceable? | Why |
| --- | --- | --- |
| A wrapped provisioning response | **Yes** | the wrap is per-identity and per-window, and is useless to anyone else regardless |
| Assembled plaintext | **Yes, against unmodified clients only** | it carries the overlay fingerprint of the holder at assembly time (§7.2); a modified client can render canonical plaintext instead and carries no fingerprint |
| A raw content key extracted from memory | **No** | it is the same value for every holder of the deployment |

The residual gap is therefore narrower than a shared content key alone implies — leaked *plaintext*, which is what actually circulates in practice, is attributable against the honest path — but it is real. If a holder extracts and publishes the raw content key, those bytes decrypt the universally-distributed swarm copy for everyone at zero marginal cost, and because many holders legitimately hold the same value, publication does not attribute the leak to any one of them. **On-chain provenance narrows who *obtained* a content key; it does not narrow who *published* one.**

This is materially different from, and worse than, the analog hole. The analog hole leaks a rendering; a content key leak hands over the canonical asset in perpetuity.

### 5.5 Blast Radius Containment

Because content key derivation is layered and domain-separated per asset and per deployment (Phase 1.3), a leaked or compromised content key exposes exactly one deployment of one asset. It does not expose other versions of the same asset, other assets by the same publisher, the publisher's master key, or the publisher's seed phrase. Leak damage is bounded to the asset, not the catalog.

### 5.6 Escrow Claim Front-Running & Identity Proof Binding

To prevent front-running attacks during escrow settlement (where an attacker intercepts a maintainer's off-chain verification proof and submits it to claim ownership), state transitions on the `Registry` require identity proofs or ZK-nullifiers to be cryptographically bound to the claimant's target chain identity. Any settlement proof generated for `Address_A` will revert on-chain if executed by or directed to `Address_B`.

Because these proofs are verified inside a contract, they are **chain-layer artifacts by definition** and are produced with the chain layer's signature adapter (§2), never with the protocol's preferred scheme. A proof produced under a scheme the target chain has no precompile for is verifiable only at prohibitive cost, so scheme selection for on-chain verification follows the chain rather than the protocol's preference.

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

Advisory types anticipated: `deprecated`, `superseded-by`, `security-advisory`, `malware`, `license-dispute`, `content-classification`, `disputed`, `retired`.

The `retired` type covers superseded per-entitlement variant objects (§7.4). When an entitlement transfers, the outgoing holder's variant object is re-minted and the previous one is no longer referenced by any entitlement, but reciprocal seeders holding it have no independent way to learn this. A `retired` advisory against the superseded variant object signals that pruning is recommended. Consistent with the rest of this section, the signal is **advisory and never obligate** — a seeder may prune, retain, or ignore it, and no participant is required to act on any flag.

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

*The MVP ships §7.1 only. §7.2 onward is the V2 path and is architecturally protected by the commitment structure in §2.*

### 7.1 MVP Posture: Per-Window Provisioning

Under the MVP, content is uniform across all holders. The content key is provisioned per authorization window through the key provisioning adapter (§2), against the current on-chain entitlement state, and is held in memory for that window only.

Each provisioning response is **wrapped to the requesting identity's key-agreement public key**, resolved through the key agreement adapter (§2) and bound to the identity by the `keyAgreement` relationship of its DID Document. Wrapping is a transport property of the response, not a grant: it secures delivery to the requester and produces distinct bytes per identity and per window. It is emphatically not a one-time issuance that substitutes for later authorization — a wrap performed once at entitlement issuance and relied upon thereafter would make possession the right, which the invariants of §1 forbid.

This buys a real, cheap barrier: no two identities are ever emitted identical bytes, and a user who copies and publishes a wrapped response gives away nothing, because no one else can unwrap it and it expires with its window regardless. Casual key sharing is eliminated outright. It does **not** buy attribution — a holder who extracts the unwrapped content key from client memory produces a universal, unattributable key. Defeating the protocol therefore requires a leaked content key *and* a modified client, consistent with §5.3.

Availability is a property of *reading*, because reading is where authorization is established. A participant needs the provisioning layer to decrypt, not merely to purchase. This is the direct cost of per-window authorization and it is the source of the liveness concern recorded in §8.1; it is accepted because transferable access rights cannot be delivered without it.

Multi-device follows without special handling: each of an identity's devices authorizes independently against the same entitlement, and no device is distinguishable from any other.

### 7.2 Variance Belongs to the Entitlement, Not the Content

Forensic traceability requires that the plaintext a user renders be recipient-specific. The naive construction varies pieces within the distributed asset, which fragments the swarm and destroys the distribution economics the protocol depends on.

The protocol instead binds variance to the **entitlement layer**, where 1:1 identity already exists by construction, and applies it as an **overlay at decryption time**:

* The **swarm object remains complete and canonical.** One ciphertext, one infohash, byte-identical for every seeder, decrypting to the canonical plaintext committed in §2. Zero fragmentation, distribution invariants intact, and unmodified BitTorrent clients continue to retrieve a complete asset.
* The **variant object is a per-entitlement seed**, referenced by the entitlement and committed to by the variant root. It carries the holder's specific permutation, not a copy of any content.
* At decryption the holder applies the permutation over the canonical plaintext, producing a rendering that is fingerprinted to them.

**The fingerprint is a cooperative mechanism, not a forensic guarantee.** Because the swarm object is complete, a holder whose client omits the overlay can render canonical, unfingerprinted plaintext. Attribution therefore holds against unmodified clients and casual redistribution — which is the great majority of leakage — and does not hold against a deliberately modified client. This is the same posture as §1.1's non-DRM stance and §5.3's honesty assumption, and it is stated here rather than overclaimed: the mechanism raises the cost and traceability of casual violation, and does not attempt to make violation impossible.

### 7.3 The Variant Object Is an Ordinary Swarm Object

The variant object is a seed rather than an enumeration of replacement content. It does not carry a fraction of the asset, and its size is therefore negligible and independent of asset size — which is what makes re-minting on transfer, reciprocal seeding, and per-window delivery cheap enough to be uninteresting.

It requires no special transport, but it does require a specific key binding:

* **Encrypted to the entitlement, not to the asset.** The variant object is encrypted under a **variant object key** bound to the entitlement identity, *not* under the asset's content key. This is mandatory. If variant objects were readable by any content key holder, any holder could read another's seed and render a copy carrying someone else's fingerprint — turning a forensic mechanism into a framing weapon. Unilateral framing is a far worse failure than collusion.
* **Seeder agnosticism carries over unchanged.** Because the object is opaque to everyone but its entitlement holder, anyone may seed it harmlessly, exactly as with the main asset (§5.1). Sharing the object confers nothing without the corresponding entitlement.

**Seeding is therefore unrestricted, and availability comes from reciprocity.** There is no reason to confine a variant object to its owner's devices. Each swarm participant sets aside a multiple of their own variant footprint to colocate other participants' variant objects; because greater aggregate colocation means greater resilience of the participant's *own* access, the incentive points toward more colocation rather than less. Reliability is emergent and probabilistic rather than guaranteed — the protocol imposes no contractual obligation on third parties, and offers none.

Reciprocal seeding also **improves** network-level privacy rather than degrading it. Variant object infohashes are per-entitlement, so a small seeder set would link a network address to a specific entitlement. With many peers holding any given object, serving an infohash no longer implies owning the corresponding entitlement, and the seeder set acts as a mixing mechanism. The residual correlation surface is tracked in §8.2.

**Multi-device is the motivating case and it resolves cleanly.** One identity, *n* devices: all *n* resolve to the same entitlement, so all *n* obtain the same variant object under their own per-window authorization, with no per-device ceremony and no distinction from any other peer.

Superseded variant objects are flagged `retired` (§6.2) so that reciprocal seeders may prune them. The signal is advisory; nothing obliges a seeder to act on it, and nothing depends on their doing so.

### 7.4 Re-Minting on Transfer

When an entitlement transfers, the variant seed is **re-minted** for the new holder. Without this, a buyer's rendering would carry the seller's fingerprint and attribution would point at the wrong party.

This is feasible because transfers are already ordered by the **transfer sequencer**, so a fresh seed is issued as part of settlement. The transfer sequencer maintains the binding between an entitlement and its current variant object — referenced by hash from the entitlement — so that the pairing survives transfer between owners. Note that this sequencer orders transfers and maintains bindings; it has no role in authorization, and provisioning does not pass through it.

That does not hand the publisher a veto. The obligation to produce the seed is **enforced by the contract that manages issuance**, not left to any party's discretion: declining is not an available move. Multi-party participation is unavoidable throughout this protocol and is not itself the hazard; the hazard would be a party able to withhold *selectively*, against a particular holder or a particular transfer, which is revocation by inaction and which the contract forecloses.

The seller retaining their retired seed and any rendering made under it is expected and harmless — it is the accepted plaintext-non-revocability case (§1.1), and their fingerprint correctly identifies them.

*Note on double-sale:* re-minting is not itself a double-sale prevention mechanism — entitlements are single-owner bearer assets and the ledger already prevents double-sale by construction. What re-minting adds is **forensic cleanup**: retired seeds are bound to a specific ownership interval, so leaked content is attributable not merely to an identity but to *when* that identity held the right.

### 7.5 Collusion Resistance

Two or more holders can diff their renderings to locate variant positions and splice a copy whose fingerprint matches neither. Variant seeds are therefore assigned using a **collusion-resistant fingerprinting code** (Tardos or equivalent), which provides provable tracing up to a chosen collusion size *c* with a false-accusation probability bounded by a security parameter.

The cost is that the codeword length grows with *c* and with the accused-population size, so *c* is an explicit economic parameter — chosen against asset value and expected adversary resources — rather than a fixed constant.

### 7.6 Future Optimization: Partial Encryption

A participant currently stores an asset twice — the ciphertext, retained to keep seeding (Phase 3.6), and the plaintext, retained to use. Roughly 2x local storage per asset held.

Partial encryption would collapse that. If a small set of pieces chosen to be *structurally essential* — headers, codec-critical frames, module entry points — were withheld from the swarm, the remainder could be distributed as plaintext. The asset stays useless without the withheld piece, the main swarm becomes plaintext with the attendant gains in deduplication and legacy client compatibility, and the key-handling problem shrinks from bulk encryption to small-object delivery.

This is a **storage and distribution optimization only**. It is unrelated to the per-entitlement variance of §7.2, which operates as an overlay over a complete swarm object and does not withhold anything; the two mechanisms should not be conflated, and adopting one does not imply the other.

**Recorded as a future optimization and explicitly not a launch capability.** The reason is commercial rather than technical: a distribution model whose plaintext is largely freely available is not a proposition sophisticated rights holders will entertain before the fully-encrypted model has been proven in production. The technical question — how degraded the content actually is with a strategically chosen fraction withheld — is also asymmetric across content classes, since a package missing its entry point is inert while a film missing 1% of its pieces may remain watchable.

## 8. Open Problems

Protocol properties that are unresolved by design rather than by omission, recorded so they are neither forgotten nor discovered late. Undetermined *authorship and design decisions* are held separately, in the To-Do list of `docs/workplans/current/ChainTorrent MVP.md`.

### 8.1 Provisioning Liveness and Centralization

Per-window authorization (§1, §7.1) places the provisioning layer on the read path by construction. This is the direct cost of transferable access rights: an entitlement that can be resold requires that authorization be re-established after the sale, which requires a party able to establish it. The threshold key-management network is the current implementation of that party, and it is the protocol's most significant compromise with its own decentralization thesis:

* **Liveness:** if provisioning is unavailable, nothing decrypts. For a package manager this means CI fails to install — a far lower tolerance for downtime than media playback has.
* **Centralization:** a colluding threshold of nodes can recover every content key ever escrowed.
* **Trust:** the protocol inherits the security and governance of a network it does not control.
* **Load:** in a populated network the provisioning layer fields authorization traffic proportional to decryption activity across the whole swarm.

**Window bounds are a security parameter that happens to govern load, and the two pull in opposite directions.** Widening the reference ceiling reduces authorization traffic and lengthens the interval during which a seller retains capacity after settlement (§1, Atomic and Bounded); narrowing it tightens the transfer boundary and multiplies provisioning load. They cannot be tuned independently, and load relief is not a free lever: any capacity argument for widening the window is an argument for weakening the transfer boundary, and must be made as such. Calibration is therefore a joint economic and security decision, unresolved, and its value belongs in the To-Do list rather than here.

This is a consequence of an unsatisfied constraint, not a design preference. **The desired end state** is to eliminate the intermediary entirely: derive or release the content key from proof of entitlement-possession-at-a-given-settlement-reference, with Byzantine fault tolerance supplied by the consensus layer that already exists rather than by a second overlay network. This is precisely the class of problem distributed consensus was built to solve, and it is the natural home for the guarantee.

No satisfactory construction is known to the authors. The obstacle is that the ledger is public: any value the chain can compute or reveal, every observer can also read, so the chain cannot itself hold a secret that only an entitled holder can unwrap. Candidate directions worth evaluating — none adopted, none yet demonstrated adequate at this protocol's cost and latency targets:

* Witness/identity-based encryption against a chain-derived witness, where finality itself releases the decryption capability to the entitled party.
* Threshold or timelock encryption anchored to consensus randomness rather than to a standing key-holding committee.
* Proxy re-encryption keyed to the entitlement transfer, moving the trust from a persistent network to the transfer event.
* Trust-minimized fallback: multiple independent provisioning deployments with client-side quorum, treating any single network as replaceable infrastructure rather than a protocol component. This is mitigation, not a solution.

**The mitigation actually adopted is architectural.** Provisioning is expressed behind an adapter (§2) so that any future construction satisfying the same two obligations — per-window authorization and no selective withholding — can replace the threshold network with backwards compatibility, without changes to identity, registry, entitlement, or content layers.

### 8.2 Dependency Graph Privacy

Every entitlement is an on-chain record binding an identity to an asset. In aggregate this publishes a permanent, correlatable map of exactly which software each participant runs, at which version. Permanence is what makes it serious: there is no point at which the association ages out, because the record *is* the entitlement.

Per-window provisioning changes the shape of the exposure without removing it. There is no permanent on-chain record of content key delivery, since nothing is published at issuance; but authorization requests form a **traffic stream** to the provisioning layer that reveals the same associations to that layer in real time, and reveals decryption *timing* that a static record would not. The permanent record is narrower and the observable behavior is wider.

* For individuals, it is a persistent behavioral profile that pseudonymity weakens only partially — a dependency set is close to a fingerprint, and one deanonymizing transaction retroactively unmasks the entire history.
* For organizations, it is a public inventory of their internal stack, including versions with known vulnerabilities. This is plausibly a disclosure hazard on its own and is an adoption blocker for most enterprises.
* Per-entitlement variant object infohashes are a further, network-level correlation surface, substantially mitigated but not eliminated by reciprocal seeding (§7.3).

Note that this cuts against the protocol's own premise: escaping platform surveillance should not mean substituting a permanent public ledger of the same behavior for a private corporate one.

Directions to evaluate, none adopted: per-asset ephemeral wallets; blinded or private-information-retrieval authorization requests; batching and mixing to break the link between requester and asset; off-chain entitlement proofs that settle on-chain only in aggregate; zero-knowledge proof of entitlement that reveals neither identity nor asset. The tension with §5.4's auditability is direct — entitlement accounting wants a legible record, and privacy wants an illegible one — and any resolution has to state which property it is sacrificing.

### 8.3 Variant Seed Authorship

§7.2 establishes that variance is an overlay seed bound to the entitlement. It does not establish who authors the seed, from what, and under what constraints. Open.

The constraint that shapes the answer is **First Finder escrow**: the publisher is absent by definition in the escrow case, so any scheme requiring the publisher to be online at issuance or transfer fails outright. Whatever is chosen must work identically for an escrowed asset whose maintainer has never appeared, and must not confer publisher rights on the bootstrapper (§1). A First Finder that authors seeds must escrow and discard the authoring material exactly as it does the content key; retaining it would leave a non-owner with permanent framing capability over an asset they do not own.

A second constraint is content validity. **A seed can select among alternatives; it cannot author them.** If both the varied positions and their replacement values are derived from the seed alone, the resulting rendering is a corruption — for a video a glitched frame, for a tarball a syntax error and a package that does not install. Producing renderings that are *semantically valid* requires alternatives authored with knowledge of the content, at which point the seed selects among them rather than generating them. Solutions to this exist and are well understood in forensic watermarking; choosing and specifying one is beyond present scope, and implementation sits behind the variance interface, so the decision can be made later without disturbing the layers around it.

Whoever holds the authoring material can generate any holder's seed and therefore frame any holder. Siting that material with the party that already provisions content keys would add no capability that party does not already have, which is the cheapest available answer but not the only one, and it inherits §8.1's trust concerns wholesale.

### 8.4 Seeder Compensation

The reciprocal colocation of §7.3 and the swarm distribution the protocol depends on both presume that seeding is rewarded. No compensation mechanism is specified. Without one, reciprocal storage degrades into the familiar ratio problem: contribution is voluntary, free-riding is rational, and aggregate resilience decays toward the level sustained by altruism alone.

This is a protocol-level gap rather than an implementation detail, because the incentive structure determines whether the distribution model works at population scale at all. Candidate mechanisms are recorded in the To-Do list rather than here, per this section's scope.

### 8.5 The Post-Claim Pricing Paradox

When a First Finder bootstraps an unlisted object, the Escrow contract unconditionally mints $0.00 entitlements to grow the swarm. When the Web2 owner arrives to claim the asset, they acquire control over future entitlement issuance. 

If the owner wishes to price the asset >$0.00, they cannot retroactively charge or deny existing users who hold the escrow-issued entitlements. The existing entitlements are permanently grandfathered. If the owner imposes a new price on the claimed object, secondary market sellers of the free entitlements can undercut the publisher, establishing a market ceiling.

**Scope of the paradox.** It arises only where the ingested content was not already free to use. Under the ingest source eligibility rule (§4, Phase 1.2), the arriving maintainer of a permissively-licensed asset was denied no revenue by the escrow-issued entitlements, so the question below is deferred rather than encountered. That rule is what permits the escrow model to be exercised in production while this section remains open.

**Resolution Strategy:** To escape the grandfathered entitlements, the publisher halts entitlement issuance on the original claimed object and issues a *new deployment version* derived from their own master key, priced at their discretion. 
        
**Open Question (Post-MVP):** How can the protocol abstract early, unclaimed downloads into fair economic value for the claimant once they arrive? If a publisher benefits from waiting for an object to become popular via free entitlements before claiming and versioning it, there must be an equitable mechanism to reward the claimant without enabling retrospective deniability. If the publisher can retroactively impose an arbitrary price and force it on existing entitlement holders, that rewards the publisher for delaying until the object is popular and/or enables deniability for the publisher. If there is no fair economic value, the First Finder and early users benefit from free-riding against an object they don't own or have rights to.

### 8.6 Authorization Height Agreement

Phase 2.4 requires provisioning nodes to evaluate `isAuthorizedBatch` at a universally agreed-upon settlement reference. That reference is load-bearing three times over: it is the state the authorization was established against, it anchors the window's reference ceiling, and it is bound into the authorization payload as replay protection (§5.2).

The deployment's declared `minSettlementTier` (§2) resolves *how settled* the reference must be, which was previously unspecified. It does not resolve *which* reference at that tier the nodes use, and that remains open. Two nodes reading at the same tier a second apart may resolve different references, and a threshold response requires them to agree on one.

Agreement is likely to fall out of the provisioning implementation's own consensus, since a threshold network must already agree on something to produce a threshold response. But that is an assumption about a particular construction, and the provisioning adapter's obligations (§2) are stated to be construction-independent. Either reference agreement is a further obligation of the adapter contract, or it is an implementation detail and the specification should stop describing it as universal. Unresolved, and recorded here so that a non-threshold construction is not adopted on the assumption that it inherits an agreement mechanism it has no reason to possess. 
