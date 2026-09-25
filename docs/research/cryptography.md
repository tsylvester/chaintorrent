# Transactable Key Protocol: Cryptographic Specification

## Overview and Invariant Requirements

The Transactable Key Protocol provides a decentralized, frictionless architecture for distributing encrypted digital assets and resolving access from blockchain state and the holder's own credential.

### Statement Classes

Every rule in this specification belongs to exactly one of four classes, and the class is stated wherever it is not obvious from context. The distinction exists because the failure this document is most exposed to is not a wrong rule but a **misclassified** one: an implementation choice that hardens into an architectural requirement because it was written down, or an unresolved mechanism that reads as settled because the prose around it is confident.

| Class | What it means | What an implementer may do |
| --- | --- | --- |
| **Normative invariant** | a property the protocol is defined by; violating it produces a non-conforming system regardless of other merits | nothing — satisfy it |
| **Defined behavior** | a mechanism this specification fixes, because interoperability requires one answer and this is it | implement exactly as written |
| **Required but unresolved** | the protocol requires the property, and the mechanism achieving it is not yet chosen | do not invent one silently; a choice made here is a protocol decision, not an implementation detail |
| **Implementation choice** | a conforming adapter may satisfy the contract however it likes | choose freely among conforming mechanisms |

Two consequences follow. A **required but unresolved** item is not a licence to improvise: two honest implementations that each fill the same gap differently produce systems that disagree about what a valid deployment or a valid decryption is, which is worse than an acknowledged hole. And an **implementation choice** never becomes normative by being the only one anyone has built — the adapter boundaries exist precisely so that the first implementation does not become the definition.

Undetermined items are tracked in the To-Do list of `docs/workplans/current/ChainTorrent MVP.md` rather than resolved here.

### Distribution Invariants: 
* **Swarm Content is Encrypted:** No plaintext objects are exchanged in the swarm.
* **Swarm Cyphertext Doesn't Mutate or Splinter Within a Deployment:** A deployment has exactly one cyphertext, byte-identical for every seeder and every entitlement holder, and no participant's authorization state produces a different swarm object. Multiple deployments of the same plaintext content version may coexist — a publisher re-deploying, or claiming an escrowed asset ([The Post-Claim Pricing Paradox](#the-post-claim-pricing-paradox)), produces new cyphertext under fresh capsule randomness while the previous deployment remains live. These are distinct swarm objects sharing a plaintext root, which is what proves they carry the same content ([Cryptographic Primitives](#cryptographic-primitives)); they are not a splintering of one object, and the plaintext root is the identifier that survives the distinction.
* **Access Control is Orthogonal to Distribution and Does Not Alter the Swarm Object:** Authorization determines whether a participant may transfer canonical cyphertext into plaintext, it does not create a different canonical swarm object for that participant.
* **Incremental / Random-Access Decryption Does Not Require Reconstructing the Entire Object:** Objects are encrypted per-piece so that video can be streamed, audio can be seeked, and files can be downloaded in parts. Only a piece must be completed for its Bao chunks to be verified and decrypted.
* **Content Identity is Independent of Encryption Identity:** The plaintext content version must remain identifiable as the same content even when publisher identity, publisher authority, encryption keys, and entitlements change, or when deployments are reissued.
* **Platform Independence:** No particular platform, application, publisher, provisioner, indexer, or other operator is authoritative over discovery, distribution, acquisition, transfer, or exercise of an entitlement. Nothing sits on the read path but the holder's own credential and a view of consensus state; no service establishes authorization on a holder's behalf, and no individual operator receives discretion over whether a holder may decrypt. Applications may provide discovery, presentation, commerce, or other value-added services, but no application may become authoritative over the underlying content, entitlement, or transfer state.
* **Intermediary Replaceability:** Any application or service providing discovery, presentation, commerce, indexing, storage, or other value-added functionality may be replaced without invalidating content identity or entitlement ownership. Replacing a deployment's cryptographic suite is a successor-deployment event under the suite's rules; an interface boundary alone does not make a different construction compatible with existing cyphertext, and a successor body must carry a header sidecar for every parameter set still live for the asset ([Escrow Key Lineage Across a Claim](#escrow-key-lineage-across-a-claim)).

### Economic Invariants: 
* **Access is Transactable:** The publisher mints and transfers access entitlements to users, and users can transfer the access entitlement to other users. **Supply is the publisher's to set, without a protocol-imposed ceiling.** A publisher may mint further entitlements at any time, at any price, in a market-making role over their own asset; the protocol constrains who may mint for an asset, never how many. Each entitlement remains a single-owner bearer asset, so unbounded supply is a statement about issuance and never about the 1:1 ownership of any individual right.
* **An Entitlement Binds to the Asset, Not to a Deployment:** An access right is held against the content, and it authorizes decryption of **any live deployment of that asset**, exercised through a credential under a parameter set the deployment carries a sidecar for, which the asset's authority keeps true of every live deployment by sidecar addition ([Escrow Key Lineage Across a Claim](#escrow-key-lineage-across-a-claim)). A deployment is the cryptographic realization a holder decrypts *through*; it is not the thing the right attaches *to*. This is what allows a holder to migrate across a re-deployment without reacquiring anything, and it is what forecloses **deployment-as-deniability** — a publisher cannot strand existing holders by issuing a successor deployment, because their entitlements reach it. The plaintext root is what establishes that two deployments are the same asset, which is why content identity must remain independent of encryption identity.
* **Access Entitlement Transfer is Atomic and Bounded:** The access entitlement transfer is atomic: the settlement contract verifies the buyer's credential envelope and transfers the entitlement in one settlement. The buyer may decrypt once that settlement reaches the deployment's declared tier, and the seller's conforming capability ends when the transfer reaches the `HARD` tier, at which point the reference client destroys its credential. That maximum lag is the chain adapter's latency from the declared tier to `HARD`, published and bounded for the deployment.
* **Access Entitlement Transfer Does Not Transfer Decryption Capability:** The access entitlement seller's previously acquired decryption capability does not become the buyer's capability, and the seller's retained cryptographic material does not constitute continued authorization. Authorization follows entitlement; cryptographic possession does not.
* **Non-Publisher Bootstrap:** Any member of the swarm can discover and provision content they don't own, whose ownership and access control methods are held in escrow for the owner to prove and claim at a later time.
* **Non-Publisher Escrow of Rights:** The protocol must be able to populate new content and provision entitlements into the swarm prior to the owner arriving to claim the content, transfer administrative authority over its escrow record without interrupting existing holders, and allow the owner to claim its economic rights.
* **Content Bootstrap Does Not Confer Publisher Rights to Content Bootstrapper:** Discovery, encryption, seeding, escrow creation, or maintenance of an unclaimed asset confers neither publisher ownership nor consumer access rights upon the bootstrap participant.
* **Escrow Must Preserve the Same Rights that Would Exist if the Publisher Had Been Present:** The escrow mechanism must preserve all the same rights and access patterns that would exist if the publisher had been present and in control from the beginning. Escrow is deferred ownership identification, not a substitute for ownership.
* **Escrow Claim Produces No Rights Drift:** A claimant who proves ownership of an escrowed asset obtains the same protocol-level rights and identity state that would have existed had they registered the asset themselves — publishing authority, issuance control, and the standing to supersede. **This is equivalence of rights, not of key lineage.** A claim does not re-key the escrow deployment: the escrow parameter set was generated by the First Finder and stays live for the credentials issued under it, and the claimant registers its own parameter set for all future issuance, optionally taking custody of the escrow master scalar as a shortcut and never as a dependency. Any difference in *rights* between a claimed record and a directly published one is a defect; the coexistence of two parameter sets, and the sole fact that the claimant did not price the escrow-era entitlements, are the expected consequences of the asset having been bootstrapped by someone else.
* **Intermediary Independence:** No particular intermediary may control discovery, distribution, access, entitlement transfer, or settlement. No service of any kind sits on the read path, so no publisher, committee, or service can exercise a read-path veto; the only live participants in a transfer are the seller and the buyer, and the settlement contract verifies rather than decides. Replacement of a cryptographic suite follows the successor-deployment boundary rather than being inferred from interface compatibility.

### Cryptographic Authorization Invariants: 
* **Cryptography Serves Economics and Distribution:** The cryptographic mechanisms are subordinate to the protocol's economic and distribution invariants. A cryptographic construction that satisfies confidentiality while violating transactable access, canonical distribution, platform independence, or non-hostile access is non-conforming.
* **Authorization is Controlled:** Before any decryption attempt, the conforming client establishes from a fresh view of chain state at or above the deployment's declared settlement tier ([Cryptographic Primitives](#cryptographic-primitives)) that its identity currently holds the exact entitlement at the interval its credential was issued for, and proves control of that identity's bound handshake key to its own session. No conforming decryption occurs that is not covered by such a check.
* **Authorization is Current:** A previous successful check, previous entitlement state, a credential issued for an earlier interval, a cached state view older than the deployment's freshness bound, or a prior decryption session does not prove access entitlement is current. A state view is current only within the freshness bound `τ_soft` declared for the deployment.
* **Authorization is Per-Attempt and Credentials are Per-Interval:** An attempt is one derivation of a piece-group key from one capsule, and each attempt requires a current state view as above. The native credential is issued for exactly one ownership interval of one entitlement and is cryptographically distinct from every other interval's credential. It does not acquire revocation from interval metadata: the reference client makes loss of capability operationally real by destroying the decrypted credential and every decrypt-capable derivative when the transfer out of the interval reaches the `HARD` tier. A modified client can preserve that material and is outside this guarantee ([Modified Clients](#modified-clients-the-honesty-assumption)). Any volume or time limit within an interval is conforming-client policy, not a cryptographic boundary, and the protocol publishes none.
* **Authorization is a Condition of Decryption, Not a Property of the Credential:** A native credential supplies cryptographic capability, not a user's right to use it. Current entitlement is a precondition to using that capability through the conforming protocol, and a conforming client may decrypt only while a current state view covers that attempt — never on the basis of a stale view, a credential issued for a different interval, or material retained after the interval ended. Retained decrypt-capable material may remain mathematically capable of driving its compatible cipher; treating that capability as current authorization is the modified-client case the protocol expressly does not claim to prevent.

### Architectural Invariants: 
* **Adapter/Interface Construction:** The protocol can and will support multiple chains, tokens, encryption schema, smart contract controllers, curves, swarm models, and more. Everything is an adapter to an interface, never hard-bound to a specific implementation detail.
* **Adapters Declare Capabilities and Compose Explicitly:** Because adapters are resolved independently at each layer, a resolvable pair is not necessarily a compatible one. Every adapter interface therefore declares what its implementations can do, consumers resolve against the declaration rather than against an implementation's identity, and an incompatible composition is refused at resolution time rather than discovered in operation.
* **No Party May Selectively Withhold an Entitlement Holder's Ability to Decrypt or Transfer:** Once an entitlement is minted and its credential delivered, the publisher cannot revoke it, deny its use, or prevent its transfer. Reading needs no party but the holder; a transfer needs no party but the seller and the buyer, with the seller's participation made non-deniable by the settlement ordering. Unavailability is indistinguishable from refusal, so no step of use or transfer may require the live action of any single third party.
* **Publisher Identity/Authority Key Rotation Does Not Version Cyphertext:** Publisher cryptographic identity is not consumer authorization state. When the publisher rotates their key, the swarm cyphertext does not change, and existing access entitlement holders do not lose their entitlement to access.

Decisions that remain undetermined are held in the To-Do list of `docs/workplans/current/ChainTorrent MVP.md` rather than here.

### Core Philosophy

This protocol optimizes for **frictionless distribution and access**, not hostile digital rights management (DRM).

* **The Distribution Problem:** Piracy is fundamentally a distribution and friction problem. By making legitimate, highest-quality access seamless and inexpensive, the incentive for piracy is mitigated.
* **No Hardware Enclaves:** The protocol strictly avoids proprietary Trusted Execution Environments (TEEs) or hardware-level DRM. Such mechanisms introduce platform friction, violate open-source ethos, and create centralized failure points.
* **The DRM Boundary (Credential vs. Plaintext):** The protocol strictly governs the lifecycle of the **native credential** and the piece-group keys derived through it. The protocol accepts the "analog hole" and acknowledges that attempting adversarial, OS-level plaintext enforcement on a user-controlled device is hostile and futile.
* **Retained Plaintext is a Deliverable, Not a Leak:** A user who has decrypted content holds it, keeps it, and uses it in whatever software they choose. This is the protocol's purpose rather than a limit on it — the transient copy that evaporates when a subscription lapses is precisely the platform behavior this design exists to replace. The protocol governs the transition from cyphertext to plaintext and makes no claim over plaintext thereafter, deliberately and permanently.
* **The Application-Layer Contract:** The conforming authorization boundary has two required halves. The settlement contract delivers a credential only to the identity that holds the entitlement at that settlement, verified by proof, and the reference client makes loss of the interval substantive by checking a fresh state view before every attempt and by destroying the decrypted credential, expanded cipher state, derived piece-group keys, buffered keystream, and every live decryption context when the transfer out reaches the `HARD` tier. The persistent credential — the envelope key pair, the envelope, and the interval index — is stored, because it is the holder's transfer witness; the decrypted credential and everything derived from it are held in memory only. A client that keeps using a credential after its interval has ended, while merely reporting the transfer, is non-conforming. Destruction supplies real loss in the reference composition; the ledger prevents reacquisition after entitlement loss. Neither is a secure-erasure claim against modified software. The client explicitly does *not* attempt to track, flush, or purge plaintext that has already been rendered or exported.
* **Public Distribution Implies Public Consumption:** Entitlements are on-chain records, so who holds access to a publicly distributed asset is publicly legible. This is a property of the design rather than a defect in it. The package list was already public; what the ledger adds is that *known* risk cannot be held privately — a consumer running a version with a published vulnerability is visible to anyone who looks, which is the direction disclosure regulation has been travelling for years. The cost therefore falls almost entirely on a participant who wants the benefit of public content while concealing the flaws inherited with it, and the protocol does not treat that as a party it owes concealment to. The net effect on the ecosystem is more security rather than less, because unpatched exposure stops being cheap to hide. Private content is a different case and is treated as one — see [Dependency Graph Privacy](#dependency-graph-privacy).
* **Anti-Derivability and Counter-Block Uniqueness (normative invariant):** Every key the payload cipher sees — under the native-credential suite, one piece-group key per piece group, chosen by the encryptor and carried in the header sidecar wrapped under each live parameter set — is subject to three requirements, and generation strategy is subordinate to them. It **must be computationally unpredictable** to any party lacking the issuer's master scalar or a valid credential. It **must be independent of the plaintext payload**, because deterministic derivation from plaintext would let any possessor of the content compute the key and bypass escrow entirely — which would trivially solve swarm fragmentation and destroy economic enforcement doing it. And every capsule **must encapsulate a cryptographically distinct value**, expressed at the level the cipher actually cares about: **for any given key, no two encryptions may ever produce the same `(IV, counter)` block.** Keystream reuse under a shared key is the failure this prevents, and it is the requirement AES-CTR imposes; a fresh piece-group key and fresh capsule randomness satisfy it per piece group, and a fresh deployment satisfies it per body.
* **Two conforming key-generation strategies (implementation choice within the invariant above):** they apply to the parameter set's master scalar, to capsule randomness, and to the piece-group keys alike. *Random generation* is **mandatory for First Finder escrow** because no publisher secret exists to derive from; *domain-separated deterministic derivation* is permitted for explicit publishers because it buys recoverability from a seed phrase without ever taking the plaintext as an input. Neither is preferred by the protocol. Each is a strategy for satisfying the invariant above, and a derivation that satisfies it is as conforming as a random draw that does.
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

**Decryption Authorization — Decentralized**

Decryption authorization is decentralized when the ability to decrypt depends on nothing but the holder's own credential and the consensus state that records the entitlement, with no intermediary able to grant, deny, or selectively withhold access.

The protocol meets this by construction under the native-credential suite: a holder's credential is delivered once, at mint or at purchase, inside the settlement that records the entitlement, and every decryption thereafter is a local operation gated by the client's own check of consensus state. No provisioning network exists on the read path. The interface boundary that once held a provisioning mechanism is retained as the credential-delivery boundary, so that a different credential construction can be introduced as a successor deployment without changing content identity, entitlement ownership, transfer semantics, or the canonical swarm object.

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

Where the protocol currently requires an intermediary for a function, that dependency is identified explicitly rather than described as decentralized. An adapter separates the protocol contract from that intermediary's implementation; replacement of a cryptographic suite follows the successor-deployment path.

**Decentralization is therefore measured not by whether individual components have operators, but by whether those operators possess irreplaceable authority over the protocol's canonical state or whether users are forced to depend upon them to exercise rights the protocol itself is intended to provide.**

### Terminology

The protocol carries several distinct objects that plain English collapses into the single word "key", and two distinct roles that plain English collapses into "the network" or "the sequencer". They are owned by different parties, live for different durations, and fail in different ways, so the unqualified words are not used in this specification.

#### Objects

| Term | What it is | What happens to it |
| --- | --- | --- |
| **Entitlement** | the transferable bearer asset recording who currently holds an access right **to an asset**, not to any one deployment of it; an ERC-721 non-fungible token in the MVP implementation | **checked**, on-chain; it signs nothing |
| **Handshake key** | the private key a party signs with to prove control of the identity that holds an entitlement | **proven**, verified off-chain via the signature adapter |
| **Parameter set** | the public parameters of one issuance authority for an asset — the pairing generators, the identity bases, and the master public element — registered on-chain and marked live per asset | **registered** at bootstrap or claim; **retired** when no entitlement issued under it remains live |
| **Master scalar** | the secret behind a parameter set, from which the issuer authors native credentials | **held** by the issuer; random for a First Finder, seed-derived for a publisher; never transmitted except in an optional escrow handover |
| **Native credential** | the two-group-element tuple that decrypts every capsule of every deployment under its parameter set; issued for exactly one ownership interval of one entitlement and rerandomized by the seller at each sale | **delivered** inside a settlement, **decrypted** into memory, **destroyed** when the interval ends |
| **Envelope key pair** | the holder's two encryption public keys, one per pairing source group, to which credentials are delivered | **registered** on-chain with proofs of possession; **persistent**, because it is the holder's transfer witness |
| **Envelope** | the credential encrypted to the recipient's envelope keys, stored by digest in the settlement record and carried in full in its calldata | **posted** at mint or sale; **kept** by the holder as the statement of its next sale |
| **Header sidecar** | the deployment's common, public sequence of entries under one parameter set, one per piece group, each holding that set's capsule and the piece-group key wrapped under it; seeded beside the ciphertext as its own object with its own locator and committed by its own root in the hash-card; useless without a credential | **published** once per deployment per live parameter set; **seeded** like any other swarm object |
| **Piece-group key** | the symmetric key the encryptor chooses for one piece group and the payload-cipher adapter uses for it; common to every holder and carried wrapped in every live set's sidecar | **unwrapped** per attempt, used, and discarded with the decryption context |
| **Variant seed** | the per-entitlement value from which a holder's fingerprint codeword is generated; committed to by the entitlement record and never published in the clear | **obtained** as confidential auxiliary material in the credential envelope; re-issued when the entitlement transfers (V2) |

An entitlement cannot sign, and a credential cannot be owned on-chain. A consumer proves control of a **handshake key** to its own session and checks current ownership of an **entitlement** against consensus state before every attempt; it asks no party for anything at read time.

#### Roles

| Role | What it does | What it does **not** do |
| --- | --- | --- |
| **Transfer sequencer** | orders entitlement transfers so that scarcity is strictly sequenced and double-sale is impossible; the settlement contract that verifies a credential envelope and advances the interval counter is part of this ordering | it does not author credentials, hold master scalars, or participate in any part of the read path |
| **Credential author** | the issuer at mint, under its master scalar, and the seller at a voluntary sale, from its own credential; each posts an envelope and a proof that the settlement contract verifies | it does not establish authorization for reads and is never contacted after the settlement completes |

These two are separate by design and must not be merged. Ordering scarcity is a write-path concern that touches the ledger; authorship of a credential is a one-time act inside that write, made non-deniable by the settlement ordering. Nothing touches the read path but the holder, which is what the Cryptographic Authorization Invariants require.

#### Units

Plain English collapses two different granularities into "chunk". They differ by three orders of magnitude and are set by different layers, so the unqualified word is not used.

| Term | Size | Set by | Role |
| --- | --- | --- | --- |
| **Piece** | configurable, power of two, ≥ 16 KiB | the CAN | unit of transport, request, and encryption offset arithmetic |
| **Bao chunk** | 1 KiB, fixed | BLAKE3/Bao | leaf of the verification tree; the granularity at which an authentication path resolves |
| **Piece group** | configurable multiple of the piece size | the deployment suite | unit of key encapsulation: one capsule in the header sidecar, one piece-group key |

Verification granularity is the Bao chunk; transport and offset granularity is the piece; key granularity is the piece group. Claims elsewhere in this specification about verifying "a chunk" without downloading the remainder are claims at Bao granularity, and are satisfied once the containing piece has arrived. The piece-group size trades pairing work per byte against how much content one derived key unlocks, and is fixed per deployment from measurement rather than by this specification.

## Cryptographic Primitives

To ensure high performance, enable out-of-order streaming, and maintain native compatibility with Content-Addressable Networks (CAN) like BitTorrent and IPFS, the protocol utilizes:

**Symmetric Encryption (Payload — Cipher Adapter):** payload encryption resolves an abstract **`IPayloadCipherAdapter`** rather than a pinned cipher, on the same reasoning that governs `ISignatureAdapter`. The interface constrains the properties the rest of the protocol depends on; it does not constrain the construction. Under the native-credential suite the adapter is keyed **per piece group**, with each group's key unwrapped from its header-sidecar entry under the holder's credential ([Credential KEM](#credential-kem-deployment-encryption-adapter)); the IV, block arithmetic, and alignment rules below are unchanged, because keys differ across groups and no key is ever reused across deployments.

The adapter contract:

* **Seekable and order-independent.** Any piece decrypts from its index alone, without the preceding stream. This is what makes byte-range seeking and out-of-order swarm delivery possible.
* **Length-preserving.** Cyphertext length equals plaintext length, with no per-piece expansion. Integrity is supplied by the BLAKE3/Bao layer, so an AEAD tag per piece would be redundant overhead that also breaks the offset arithmetic.
* **Nonce-invariant per deployment.** One IV per deployment, fixed for its lifetime and published in the clear in the manifest, so the offset arithmetic holds identically for every participant.
* **Declared addressable extent.** Every adapter publishes `maxAddressableBytes`, the largest object it can encrypt under a single `(key, IV)` pair without exhausting its counter field. This is a property of the construction, not a protocol constant, and it is what the manifest bounds check below is evaluated against.

| Adapter | Construction | Counter layout | `maxAddressableBytes` | Notes |
| --- | --- | --- | --- | --- |
| `AesCtrAdapter` | AES-256-CTR | 64-bit IV ‖ 64-bit block counter | 2⁶⁸ (≈256 EiB) | MVP implementation |
| `XChaCha20Adapter` | XChaCha20 | 192-bit nonce ‖ 64-bit block counter | 2⁶⁶ (≈64 EiB) | candidate; wider nonce, no hardware dependency |

**MVP implementation (`AesCtrAdapter`).** AES-256-CTR with a randomly generated **64-bit IV** per deployment and a **64-bit block counter**, together forming the 128-bit counter block AES requires. The counter block for a given piece is `(IV << 64) | block_index`, where `block_index = (PieceIndex × PieceSize) / 16`. The counter layout is a declared parameter of the adapter rather than a constant: the hash-card carries the layout a deployment uses, chosen at registration within the layouts the adapter declares, each with its own `maxAddressableBytes`, and 64/64 is the default for the reason below.

* **Why 64/64 rather than 96/32.** A 32-bit counter caps a deployment at 2³² blocks — 64 GiB — which is adequate for a package tarball and inadequate for the media classes this protocol targets. A 64-bit counter removes the ceiling at the cost of IV width, which is acceptable because piece-group keys are fresh per capsule and never shared across deployments ([Core Philosophy](#core-philosophy)): an IV collision across two deployments, and the one shared IV across the groups of a single deployment, are harmless when the keys differ, and there is only ever one IV within a deployment.
* **Fields, not integer addition.** The IV occupies the high 64 bits and the counter the low 64; the block index is written into the counter field and never added to the counter block as a whole. Whole-block addition would carry into the IV field, which is a different construction with different collision behavior.
* **Piece Alignment.** All CAN piece sizes are multiples of the 16-byte AES block size, which every legal BitTorrent piece size already satisfies. Only the final block of a deployment is partial, and CTR mode handles it by keystream truncation, so no padding exists anywhere in the object.
* **Nonce Invariants.** The IV is generated randomly **once per deployment**, not per file, and is stored in the clear within the CAN manifest. A deployment containing multiple files is encrypted as a single contiguous byte stream under one IV, keyed per piece group, so file boundaries have no cryptographic meaning and the block arithmetic is continuous across them. **Per-file IVs under a shared key are a protocol violation**, and so is any construction deriving a second IV from the first: AES-CTR leaks the XOR of plaintexts under keystream reuse, and a deployment is the unit at which nonce invariance is guaranteed. Where a fresh IV is needed, a fresh deployment is the mechanism ([Content Key Generation](#phase-1-encryption-minting-and-seeding), `deployment_id`). This rule governs the protocol's own encryption of a deployment and says nothing about what the plaintext contains.
* **Payload contents are opaque to this layer, and nesting is permitted.** A deployment's plaintext may itself hold separately encrypted material — an encrypted archive, a file carrying its own protection, or **another ChainTorrent object with its own deployment, parameter set, and entitlements**. The protocol encrypts a deployment as one contiguous stream and imposes no constraint whatever on structure inside that stream; independent encryption within the payload is not a second IV under this deployment's keys and does not implicate the nonce invariant above. Each nested object is a deployment in its own right, and the invariant applies independently at each layer. The entitlement consequence is the operative one: **an outer entitlement authorizes decryption of the outer cyphertext only.** It neither confers nor implies authorization for anything independently encrypted within, which resolves under its own entitlement or does not resolve at all. A container deployment can therefore be distributed, held, and seeded by parties who cannot read its constituents, and holding the container is not a claim on them. Byte-embedding shares nothing with the outer object, since the inner ciphertext is encrypted again; sharing pieces between a collection and its members needs **reference nesting** — a container object whose hash-card lists member deployments by their own roots, in the manner of BitTorrent v2's per-file trees — which is recorded as future work in the To-Do list and not specified here.
* Payload integrity and verification are handled natively by the BLAKE3-Bao Merkle tree layer, removing the need for redundant AEAD tag overhead while allowing instantaneous seek-and-decrypt capabilities.

#### Manifest Bounds Validation

A manifest is untrusted input until it has been authenticated, and its declared geometry drives the block arithmetic above. The client therefore validates it in full **before deriving any piece-group key**, and a failure is a hard cryptographic error that halts the operation:

* **Piece geometry.** Piece size is a power of two, at least 16 KiB, and a multiple of the cipher's block size. Declared total size, piece size, and piece count are mutually consistent.
* **Addressable extent.** The highest block index the object can address — `ceil(total_bytes / 16) - 1` for a 16-byte block cipher — falls within the resolved adapter's `maxAddressableBytes`. A manifest declaring an object the adapter cannot address under one `(key, IV)` pair is rejected outright rather than silently wrapping the counter, which would reuse keystream.
* **Index range.** Every piece index requested or served is less than the declared piece count. Out-of-range indices are refused at the transport layer, not resolved into offsets.

**The ordering is the security property, not the arithmetic.** Validating before decryption means a malformed or hostile manifest can never cause a credential to be exercised, so no key material is derived for a session that was going to fail, and a manifest cannot be used as a probe against the client's credential handling. The same rule covers the header sidecar: its capsules are authenticated against the sidecar root and checked for well-formedness before any is decapsulated.

Under `AesCtrAdapter` the extent bound is unreachable in practice — 2⁶⁸ bytes exceeds any object that could be assembled, and such a manifest fails on piece count and allocation long before the counter is implicated — so for the MVP this check is dormant. It is specified as an adapter-declared bound rather than a constant because narrower counter fields make it live: a 32-bit counter caps an object at 64 GiB, which feature-length video reaches, and an adapter carrying that layout must fail closed rather than wrap.

**Integrity & Verification:** `BLAKE3` (utilizing a Bao-style verified streaming structure) provides a native Merkle tree for Bao chunk verification. BLAKE3/Bao provides cryptographic integrity proofs for individual Bao chunks and binds those chunks to the authenticated content root, eliminating manual MAC tree management overhead and mapping directly to Merkle DAG structures (e.g., Git repositories).

**Confidentiality & Integrity Separation:** The protocol deliberately separates confidentiality from content integrity. AES-CTR provides encryption; BLAKE3/Bao provides independently verifiable content integrity and random-access authentication proofs.

**Swarm Transport (Transport Adapter):** how encrypted pieces move between participants resolves an abstract **`ISwarmTransportAdapter`** rather than any one network. Pinning a single transport would be a commitment no other layer of this protocol makes, and it would foreclose transports whose native verification structure is the one specified here.

| Adapter | Construction | Integrity on the wire | Notes |
| --- | --- | --- | --- |
| `BitTorrentAdapter` | BitTorrent v1 swarm, magnet/BTIH addressing | legacy piece hashes, carried alongside the Bao tree | compatibility implementation; unmodified clients participate with no protocol integration |
| `Blake3NativeAdapter` | verified-streaming transport addressing content by BLAKE3/Bao root | the Bao tree itself | no duplicate integrity structure; the protocol's own commitment is the wire format |

* **Compatibility is available, not obligatory.** A deployment that wants unmodified BitTorrent clients in its swarm resolves the compatibility adapter and pays its costs. A deployment that does not, does not.
* **A deployment may be multi-homed.** Ciphertext and plaintext roots are transport-independent, so the same bytes served over two transports are one swarm object carrying two locators rather than two objects. No transport's absence invalidates a deployment, and no transport is authoritative over one.
* **Peer discovery resolves separately from transport.** Locating peers holding an object — by DHT, tracker, peer exchange, local discovery, or a seeder registry carried in consensus state — is independent of the transport carrying the bytes, and multiple discovery mechanisms may be active at once with none authoritative. Because seeder agnosticism ([Seeder Agnosticism](#seeder-agnosticism)) means possession of encrypted pieces confers no access, publishing which participants serve which objects discloses nothing about who holds an entitlement — which is what makes a public seeder registry a safe construction where a public entitlement registry is not ([Dependency Graph Privacy](#dependency-graph-privacy)).

**Content Commitments (Ciphertext, Plaintext, Sidecar, and Variant Roots):** The canonical on-chain record commits to **three** BLAKE3/Bao roots per deployment, with a **fourth** added per-entitlement when per-entitlement variance is enabled ([Per-Entitlement Variance](#traceability-and-per-entitlement-variance)):

* the **ciphertext root**, which lets seeders and downloaders verify opaque pieces they cannot read;
* the **plaintext root**, which lets a decrypting client verify that what it produced is the canonical plaintext, published in one of the three disclosure modes below;
* the **header sidecar root**, one per live parameter set, which authenticates the public capsules a credential is exercised against, so that no party can substitute a capsule that selects among honest holders or leaks through a malformed component; and
* the **variant root** (per-entitlement, carried in the entitlement record rather than the asset record), which commits to the current holder's variant seed without disclosing it. Absent under the MVP's uniform-content posture.

The ciphertext root alone is insufficient. Integrity checking happens entirely at the ciphertext layer, so a client holding an incorrect credential, or exercising it against a substituted sidecar, produces garbage plaintext that passes every check the protocol otherwise performs. The plaintext root closes that gap.

Critically, the commitment is to the **plaintext, not to the key**. Committing to a key would hard-code a single universal value and foreclose per-identity credentials. Committing to the plaintext constrains the *output* rather than the mechanism, so it holds unchanged under native credentials, per-identity subkeys, traceable decryption keys, or any future credential scheme. It also provides plaintext-side incremental verification that mirrors the ciphertext tree: a single decrypted Bao chunk can be validated against its authentication path without decrypting the remainder of the asset.

Because the swarm object remains complete and canonical under per-entitlement variance ([Variance Belongs to the Entitlement](#variance-belongs-to-the-entitlement-not-the-content)), the plaintext root commits to the canonical plaintext for every holder. The overlay is applied after verification, so variance does not fork the commitment.

#### Plaintext Root Disclosure Modes

Publishing a plaintext root in the clear permits confirmation-of-content attacks against low-entropy or guessable assets: an observer holding a candidate plaintext can confirm it without holding an entitlement. This is a non-issue for public assets such as NPM packages, whose plaintext is openly distributed regardless, and it is a real exposure for short, personal, or private content.

The plaintext root is therefore published in one of three **disclosure modes**, declared in the canonical record. The mode is a property of the deployment, fixed at registration, and its absence is stated explicitly rather than inferred — the same posture the record takes toward a missing upstream attestation ([Ingest Source Eligibility](#ingest-source-eligibility)). A consumer always knows which verification is available to it before it requests anything.

| Mode | Root published as | Who can verify | Confirmation attack |
| --- | --- | --- | --- |
| **Public** | bare BLAKE3/Bao root | anyone | possible against guessable plaintext |
| **Keyed** | BLAKE3 keyed-mode root under `KDF(verification_secret, "plaintext-root-v1")` | current entitlement holders | not possible without the verification secret |
| **Absent** | omitted | no one | not possible |

**Keyed is the recommended default for private content.** The cryptographic suite defines the verification secret established at deployment. Under the native-credential suite it is the encapsulated value of a reserved capsule in the header sidecar, so every holder derives it locally from its credential and no delivery step exists; the plaintext-root interface does not require the verification secret and any piece-group key to be the same value. Possession of the verification secret permits confirmation of candidate plaintext but need not confer decryption capability.

**What lapses under Keyed.** Verification becomes holder-only, so the properties that depend on *third parties* checking equivalence no longer hold: deterministic First Finder race resolution is not externally auditable, re-deployment identity continuity is asserted rather than provable to outsiders, advisories can no longer follow the content across re-encryption by targeting the fingerprint, and the binding to a Web2 source of truth is not independently checkable. Decryption correctness — the property the root primarily exists to deliver — is fully preserved.

**What lapses under Absent.** Everything above, plus decryption correctness itself. A client holding an incorrect credential produces garbage that passes every remaining check the protocol performs, exactly the gap this section opens by describing. The trust model shifts to the issuer, which is the ordinary expectation for personal cloud storage and is a genuine reduction relative to the rest of this specification. Absent is appropriate where the publisher and the consumer are the same party, or where masking outweighs verification; it should not be selected by default.

**MVP posture:** public registries are Public mode without exception. Provenance and independent verifiability are the entire point of the NPM path, and the plaintext is openly distributed regardless, so there is nothing to mask.

**Credential Delivery (Credential Delivery Boundary):** The protocol-facing boundary delivers a native credential exactly once per ownership interval, inside the settlement that creates the interval. The credential author — the issuer at mint, the seller at a sale — encrypts the credential to the recipient's registered envelope keys through the key-agreement adapter and posts the envelope together with a delivery proof ([Delivery Proof](#delivery-proof-delivery-proof-adapter)). The settlement contract verifies the proof against the entitlement's own record, and in the same transaction advances the interval counter, records the recipient's envelope keys and the envelope digest, transfers the entitlement, and releases payment. The proof's statement binds the asset, deployment parameter-set digest, exact entitlement contract and token, both interval counters, purpose, both parties, all four envelope keys, both envelopes, and an expiry. No confidential value is ever delivered outside this settlement, and nothing is delivered at read time.

The native-credential suite realizes this boundary with the credential KEM below. A suite with a different credential construction delivers its own compatible credential through the same boundary as a successor deployment.

The phrase **"the NFT is present in the requester's wallet"** has one protocol meaning, now evaluated by the conforming client before each attempt rather than by any service: at a settlement reference no older than the deployment's freshness bound, the exact entitlement has valid issuance lineage to the asset's registered publisher or escrow authority, consensus state names the client's identity as its current owner at the interval the credential was issued for, and the client's session proves control of a device key that the same consensus state shows the identity currently admits. A wallet application's local report or a preserved historical header is not an authorization input.

**Settlement (Settlement Adapter):** every attempt is gated on chain state, and *how settled that state must be* is a declared parameter rather than a protocol constant. Confirmation counts are not comparable across chains, so the protocol expresses settlement as a small ordered tier vocabulary that every chain maps onto, resolved through an abstract **`ISettlementAdapter`**.

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

**Depth is proportional to value at risk, and the protocol already accepts the failure it admits.** The attempt rule reads at the declared tier and destroys at `HARD`, so a reversal between them has exactly two effects: a buyer who read its envelope from a settlement that was then reverted holds one entitlement's credential without a settled transfer, and a seller whose transfer reverted still holds its credential and its entitlement. The first is the same bounded quantity the seller could have produced by leaking its own credential; the second is why destruction waits for `HARD`. A shallow `minSettlementTier` therefore introduces no new class of risk; it marginally increases the frequency of a failure the protocol already tolerates by design, and the failure is forward-only, non-compounding, and self-healing, because scarcity is ordered by the entitlement ledger and never by a credential.

The consequence is that a shallow tier is a legitimate choice rather than a concession to impatience. Requiring deep settlement universally would make decryption latency a function of chain finality, which is unusable for the interactive cases this protocol targets — a CI pipeline installing dependencies cannot wait for an L1 challenge period, while a viewer starting a high-value film reasonably can.

---

Any suite behind the credential-delivery boundary is bound by three obligations that are not optional properties of a particular construction:

* **Delivery inside settlement.** A credential is delivered only within the settlement that creates its interval, bound by proof to the complete statement above, and never outside one. The contract verifies at the deployment's declared settlement tier **exactly** — not weaker, which would violate the declaration, and not stronger, because settling deeper for one holder than another is discretion on the write path. Suites may vary *what* credential they deliver and *how* its validity is proved; they may never vary *whether* delivery is verified. A suite that delivers without proof, or whose contract accepts substitution of another entitlement, owner, interval, deployment, or envelope, is non-conforming regardless of its other merits.
* **No selective withholding.** A transfer needs no party but the seller and the buyer, and once payment is locked and the envelope posted, no live action by anyone completes or blocks it. Minting is the publisher's discretion; the use and transfer of an existing entitlement are not, and no construction may put a third party's availability on that path.
* **Per-attempt authorization by the client.** The conforming client, not the suite, establishes current entitlement before every attempt from a fresh state view, and a suite may not presume authorization forward from delivery.

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

[^enclave]: Using a platform authenticator to hold a signing key is unrelated to [Core Philosophy](#core-philosophy)'s rejection of hardware DRM. The protocol never requires special hardware to decrypt or render content; where a user's identity key happens to live is their choice and constrains no one else.

**Pairing Groups (Pairing Adapter):** the credential KEM, the credential envelope, and the delivery proof all live in a Type-3 pairing group `e: G1 × G2 → GT` of prime order, resolved through an abstract **`IPairingAdapter`**. The adapter is selected by the chain adapter, not by the protocol, because the delivery proof is verified inside a contract and is therefore a chain-layer artifact under the same rule that governs escrow-claim proofs ([Escrow Claim Front-Running](#escrow-claim-front-running--identity-proof-binding)): a curve the target chain has no precompile for is verifiable only at prohibitive cost.

```
    g1, g2                                  -> generators of G1, G2
    mul(P, k), add(P, Q)                    -> in either source group
    pairingProductIsOne([(P_i, Q_i)])       -> bool, the only target-group operation the verifier needs
    subgroupCheck(P)                        -> bool, per group
```

| Adapter | Curve | Verifier interface | Notes |
| --- | --- | --- | --- |
| `Bn254PairingAdapter` | BN254 | `G1` add and scalar multiplication, pairing-product check; the pairing precompile enforces the `G2` subgroup check and `G1` has cofactor one | available on every EVM chain; second-group equations are checked as pairing products |
| `Bls12381PairingAdapter` | BLS12-381 | `G1` and `G2` add, multi-scalar multiplication, pairing-product check; every MSM and pairing input is subgroup-checked | where the newer precompiles are deployed; second-group equations are checked directly, with no pairing |

* **Capability declaration.** The adapter declares whether the verifier has second-group arithmetic, because the delivery-proof adapter's on-chain form depends on it ([Delivery Proof](#delivery-proof-delivery-proof-adapter)).
* **No target-group arithmetic.** Nothing above the adapter computes in `GT` except by pairing; the verifier interface exposes only a product-of-pairings check, and every proof form is written against that constraint.

**Key Agreement (Key Agreement Adapter):** wrapping a credential to a recipient is an *encryption* operation and resolves an abstract **`IKeyAgreementAdapter`**, never the signature adapter. The two are distinct primitives over distinct keys: a signature key proves authorship and cannot receive a wrapped secret, and treating them as interchangeable is a category error rather than an implementation shortcut. Ed25519 signing keys are not encryption keys; secp256k1 requires an integrated encryption scheme. The binding is already expressed in the Binding Schema below, where a DID Document distinguishes `authentication` from `keyAgreement` — the signature adapter resolves the former, the key agreement adapter the latter.

```
    wrapTo(recipientPubkey, plaintext)   -> cyphertext
    unwrap(recipientPrivkey, cyphertext) -> plaintext
```

| Adapter | Scheme | Paired signature scheme |
| --- | --- | --- |
| `X25519Adapter` | X25519 + HKDF + AEAD (HPKE-style) | Ed25519 |
| `EciesSecp256k1Adapter` | ECIES over secp256k1 | secp256k1 |
| `PairingElGamalAdapter` | ElGamal in each source group of the pairing adapter, under two independent recipient secrets `x, y` with `pk1 = g1^x`, `pk2 = g2^y`; `wrapTo` encrypts one `G1` and one `G2` element as `(g1^ρ, A · pk1^ρ, g2^σ, B · pk2^σ)` | any; the envelope keys are bound to the identity by the `keyAgreement` relationship and registered with proofs of possession |

* **The credential envelope resolves this adapter with a capability the delivery proof requires.** A delivery-proof adapter declares which envelope algebra it can prove statements about; `PairingElGamalAdapter` is the one the MVP's proof adapter supports, and the suite binds the two together. Two hazards are part of the adapter's contract: the two recipient secrets must be independent, because a shared secret across the groups gives every observer a pairing-based DDH test on the `G1` ciphertext; and identity-element keys are rejected at registration.

#### Credential KEM (Deployment-Encryption Adapter)

The credential construction resolves an abstract **`ICredentialKemAdapter`**, the suite's deployment-encryption adapter. It is what makes a decryption capability native and distinct per ownership interval while every holder decrypts one canonical ciphertext. The interface fixes what the rest of the protocol depends on; the construction is an implementation behind it.

```
    setup()                                   -> (parameterSet, masterScalar)
    issue(masterScalar, entitlementId)        -> credential           // at mint, under a parameter set
    rerandomize(credential)                   -> credential           // at sale, by the seller, no master scalar
    isValid(parameterSet, entitlementId, credential) -> bool         // public-parameter check
    encapsulate(parameterSet)                 -> (capsule, K)         // once per piece group, at encryption
    isWellFormed(parameterSet, capsule)       -> bool
    decapsulate(credential, entitlementId, capsule) -> K             // local, per attempt
```

| Adapter | Construction | Assumptions | Notes |
| --- | --- | --- | --- |
| `Bb1DepthOneKemAdapter` | the depth-one, all-wildcard specialization of Boneh–Boyen selective-identity IBE in Type-3 groups, with a declared **identity scope** of one cryptographic identity per entitlement or one per asset, and seller-side rerandomization | decisional BDH-3b in the pairing adapter's groups | MVP implementation; explicit-publisher suites fix the entitlement scope, escrow suites fix the asset scope |

**MVP implementation (`Bb1DepthOneKemAdapter`).** A parameter set is `g1, u0, u1 ∈ G1` and `g2, hpub = g2^α ∈ G2`; the master scalar is `α`. The entitlement's identity element is `F_I = u0 · u1^I` for `I` an authenticated, domain-separated scalar mapping of the exact entitlement identifier, and a mint whose `F_I` is the identity element is refused. A credential is `(A, B) = (g1^α · F_I^r, g2^r)`, with `r` disclosed to nobody; the seller rerandomizes it as `(A · F_I^s, B · g2^s)` with a private `s`, which yields a fresh credential of the same identity without the master scalar, and the buyer may rerandomize again. Validity is the public check `e(A, g2) = e(g1, hpub) · e(F_I, B)`, with subgroup validation. A capsule is `(U, V, W) = (g2^t, u0^t, u1^t)`, well-formed when `e(V, g2) = e(u0, U)` and `e(W, g2) = e(u1, U)`, and encapsulates `K = e(g1, hpub)^t`; decapsulation is `K = e(A, U) / e(V · W^I, B)`, two pairings and one scalar multiplication. A domain-separated KDF of `K` and the context — asset, deployment, suite, parameter set, group index, geometry — yields the group's **wrapping key** under that parameter set. The piece-group key itself is chosen by the encryptor independently of any parameter set, and the sidecar entry for the group under each live set carries that set's capsule together with the piece-group key wrapped under that set's wrapping key ([Payload Encryption & Hashing](#phase-1-encryption-minting-and-seeding)); a KEM output is never used as a piece-group key directly, because two independently generated parameter sets encapsulate different values and a ciphertext must decrypt under every live set. The wrap is defined behavior: the wrapping key is the KDF output at the piece-group key's length, the wrapped value is their XOR, and no tag is carried, because the sidecar's Bao root already authenticates every entry. In the MVP suite every derivation a client performs off chain, the wrapping key, the publisher lineage's master scalar, identity bases, capsule randomness, and piece-group keys, and the keyed plaintext-root mode, is BLAKE3 keyed derivation under a fixed context string per use; every value a contract recomputes, the identity mapping `I` and the delivery proof's challenge, is keccak256 under a domain tag; the hash-card's KDF and hash-to-scalar identifiers name both. Three properties follow from the algebra: every valid credential for an identity recovers the same `K` from every well-formed capsule, so one sidecar entry serves every holder under its set, and because the wrapped value is one piece-group key, every live set opens the same ciphertext; no holder learns its own exponent, so no coalition of holders can recover the master scalar or strip a credential; and a credential for one entitlement cannot be converted into one for another, so a leaked usable credential attributes to its entitlement.

**Identity scope is a declared capability, fixed by the suite.** Under the **entitlement scope**, `F_I` is per entitlement: a leaked credential names its entitlement and cannot be converted to another's, and only the master scalar can author a credential for a newly minted entitlement. Under the **asset scope**, `F` is one fixed element per parameter set shared by every credential of the asset, and any holder can author a credential for a newly minted entitlement of the same asset from its own credential, by the same rerandomization and the same transfer proof, with no master scalar; the capsule shrinks to `(g2^t, F^t)`. The two are exclusive by construction — holder authorship means any holder could have made any credential, so attribution below the asset is gone — and the suite chooses. Explicit-publisher suites fix the entitlement scope. Escrow suites fix the asset scope, because an escrowed asset has no publisher and must not depend on the First Finder's continued presence for new grants ([Phase 1](#phase-1-encryption-minting-and-seeding)).

**What the interface constrains.** Every implementation must deliver constant-size credentials across any number of sales, a capsule cost independent of the number of entitlements, a public validity check, rerandomization without the master scalar, and, under the entitlement scope, non-convertibility across entitlements. A construction with a transfer-depth bound, a setup-time entitlement count, or a common decryption-equivalent secret held by any party other than a holder is non-conforming.

#### Delivery Proof (Delivery Proof Adapter)

Delivery is verified, not trusted. The settlement contract accepts an envelope only with a proof, resolved through an abstract **`IDeliveryProofAdapter`**, that the envelope decrypts under the recipient's registered keys to a valid credential for the exact entitlement. The proof is a chain-layer artifact and its on-chain form follows the pairing adapter's verifier interface.

```
    proveMint(masterScalar, coins, statement)          -> proof
    proveTransfer(sellerSecrets, offset, coins, statement) -> proof
    verify(statement, proof)                            -> bool     // on-chain, precompile-shaped
```

| Adapter | Construction | Proof size | Verifier work | Notes |
| --- | --- | --- | --- | --- |
| `SchnorrFsDeliveryProofAdapter` | generalized Schnorr argument over linear representation equations in both source groups, made non-interactive by Fiat–Shamir over the full statement | six scalars; on a curve without second-group arithmetic at the verifier, three further `G2` elements | with `G2` arithmetic: sixteen scalar multiplications in six multi-scalar multiplications, no pairing; without: one hash-weighted pairing product of about ten pairs | MVP implementation; random-oracle model, simulation-sound and weakly simulation-extractable |
| `GrothSahaiDeliveryProofAdapter` | Groth–Sahai proof of the pairing-product validity statement under a public-coin CRS | constant, several times larger | dozens of pairings | recorded alternative; not built |

**MVP implementation (`SchnorrFsDeliveryProofAdapter`).** The mint proof shows knowledge of `(α, r, ρ, σ)` with `hpub = g2^α`, `C1 = g1^ρ`, `C2 = g1^α · F_I^r · pk1^ρ`, `D1 = g2^σ`, `D2 = g2^r · pk2^σ`. The transfer proof shows, against the previous interval's envelope `E_o` and the seller's registered keys, knowledge of `(x_s, y_s, s, ρ, σ)` with `pk1_s = g1^x_s`, `C1_n = g1^ρ`, `C2_n / C2_o = C1_o^(−x_s) · F_I^s · pk1_b^ρ`, `pk2_s = g2^y_s`, `D1_n = g2^σ`, `D2_n / D2_o = D1_o^(−y_s) · g2^s · pk2_b^σ`. That is, the seller proves its new envelope is its own decryption of the previous envelope shifted by a secret offset, and soundness runs by induction from the mint proof along the entitlement's envelope history, which the contract supplies from the entitlement's own record. Every equation lies in a source group, which is what the precompile interface requires; a direct proof of the validity equation would need target-group arithmetic, and the obvious source-group substitute for it leaks the delivered credential. The challenge is keccak256 over the complete statement under a domain tag, which the contract recomputes, so a proof is valid for exactly one settlement.

#### Binding Schema

The binding between an identity's per-layer keys is expressed as a **W3C DID Document**. A DID Document already carries a `verificationMethod` set holding multiple keys of different types, and *verification relationships* — `authentication`, `assertionMethod`, `keyAgreement`, `capabilityInvocation` — that state which key serves which purpose. That is per-layer key resolution as a standard, multi-key by construction and chain-agnostic by design, so the binding is not a bespoke registry field and layer selection is a verification relationship rather than an invention of this protocol.

Storage is hybrid, because the binding is consulted from two places with opposite cost profiles:

* **On-chain:** the fields on-chain logic must read — the bound handshake public key and its scheme — are stored on-chain, roughly two storage words. Escrow claim verification ([Escrow Claim Front-Running](#escrow-claim-front-running--identity-proof-binding)) and authorization checks run inside contracts, and a contract cannot read an off-chain document without an oracle or a proof system, which would reintroduce the intermediary class this design removes.
* **Anchored:** the full DID Document is committed by hash on-chain and resolved off-chain. This keeps the schema extensible without contract changes and keeps storage cost at one word.

The chain holds state and pointers to actuals; large objects that are merely *identified* stay off-chain.

Alternatives considered, recorded so the choice can be attacked on review:

| Alternative | Fit | Why not primary |
| --- | --- | --- |
| UCAN / ZCAP-LD | capability delegation chains; the chain key delegates handshake authority | closer to the transferable-rights framing, but delegation semantics exceed what a key binding needs |
| Session keys (AA wallets) | a subordinate key authorized by a master key for a scope and duration | the ergonomics are well-trodden and this is effectively what the handshake key is; lacks a standard document format for multiple simultaneous purposes |
| Sigstore (Fulcio + Rekor) | ephemeral key bound to an identity, recorded in an append-only transparency log | philosophically close to the append-only registry, and the model the software supply chain is converging on; oriented to short-lived signing identities rather than durable multi-layer bindings |
| ERC-1271 / ERC-4337 | the chain identity is a contract declaring arbitrary signature validity | solves a different problem — validity rather than binding — and is EVM-specific; the EVM chain adapter uses it beside the binding, as the identity's contract account whose signer set admits and revokes devices |
| X.509 / SSH CA certificates | subordinate key signed by an authority | the structural ancestor of the attestation, but the PKI trust model does not fit a bearer-asset protocol |

#### Adapter Composition and Capability Declaration

Independent per-layer resolution is what keeps a chain, a curve, a transport, or a keystore from becoming a protocol commitment. It is also what makes it possible to resolve a pair that cannot compose: a key custody implementation that cannot produce what a chain adapter verifies, a transport that cannot carry what a discovery adapter advertises, a seed host that cannot satisfy a retention obligation. Each of those pairs resolves, and none of them works.

**Capability is part of the interface, not knowledge about implementations.** Every adapter interface declares the properties a consumer must resolve against — what an implementation can hold, produce, carry, or guarantee. A consumer that instead branches on an implementation's identity has pulled implementation detail into the layer above it, which is the coupling the adapter existed to prevent.

**Composition is checked at resolution and fails closed.** An incompatible pair is refused at the point of resolution, before any operation is attempted. This is the same ordering property as Manifest Bounds Validation above: validating before proceeding means no key material is exposed, no authorization traffic is spent, and no swarm activity is begun on a composition that was never going to complete.

**The unit of cryptographic composition is an immutable deployment suite.** Its identifier fixes the pairing adapter, the credential KEM adapter and its parameter set, the payload-cipher adapter and piece-group size, the key-agreement adapter used for envelopes, the delivery-proof adapter, the KDF and hash-to-scalar mappings, the verification-material contract, the delivery-statement version, and the attempt-rule parameters. The header sidecar is the suite's recipient-independent public object. Those components are resolved together rather than independently mixed after registration, and a delivery-proof adapter is admissible only if it declares the envelope algebra of the chosen key-agreement adapter. An implementation may be replaced in place only when it preserves the suite's byte-level contracts. A construction requiring different ciphertext formation or a different credential is introduced as a successor deployment, whose body carries a header sidecar for every parameter set still live for the asset. Because entitlements bind to the asset, holders retain the same entitlement across that transition.

Most conflicts never arise, because binding-not-matching already permits two layers to use different schemes provided they are provably the same principal. That is why incompatible pairs are rare rather than absent — and rare-and-unhandled is precisely how a protocol acquires an undocumented compatibility matrix after the fact.

### The Hash-Card (Deployment Descriptor)

The content commitments are fields of a larger descriptor. The `.torrent` file is the present-day carrier; the **hash-card** is its intended successor. The canonical deployment record authenticates the complete hash-card, so an attacker cannot substitute a weaker suite, parameter set, sidecar, or attempt policy while retaining the deployment identity.

**Compatibility posture.** Compatibility with existing BitTorrent clients is a property of the `BitTorrentAdapter`, not an obligation of the protocol. A deployment resolving that adapter exposes a standardized magnet link and BTIH in its canonical record (see [Canonical Identity Pre-Check](#phase-1-encryption-minting-and-seeding)), and unmodified clients participate in its swarm with no blockchain integration. A deployment resolving a BLAKE3-native transport exposes that transport's locator instead, and a multi-homed deployment carries both. The hash-card is therefore a *superset* of whatever descriptor a transport carries: everything that format holds, plus the fields it has no place to put. Legacy clients read the subset they understand; protocol clients read the whole card.

**Two integrity structures where a transport requires it, one authority always.** A legacy BitTorrent client verifies pieces against the v1 SHA-1 piece hashes and cannot join a swarm without them, so a deployment served over the compatibility adapter publishes both those hashes and the BLAKE3/Bao tree over the same bytes. **That duplication is a cost the compatibility adapter carries, not a property of the protocol** — a BLAKE3-native transport carries the Bao tree alone, because the protocol's own commitment is already its wire format. Where the duplication exists it is off-chain and small: twenty bytes per piece in the `.torrent`, with the on-chain record carrying only the locator. **The protocol does not inherit SHA-1's collision weakness.** The Bao cyphertext root is the sole authority for what the canonical object is under every transport; legacy piece hashes are transport-layer compatibility and nothing else. A piece satisfying SHA-1 but failing its Bao authentication path is rejected by every protocol client, so a legacy client can circulate bad bytes but can never establish canonical ones. Alignment imposes no additional constraint, since every legal BitTorrent piece size is already a multiple of the 16-byte cipher block.

| Field | Present in `.torrent`? | Purpose |
| --- | --- | --- |
| Identity hash — `BLAKE3(name @ version)` | no | canonical Registry key |
| Ciphertext Bao root | partly (piece hashes) | verify opaque pieces while seeding |
| **Plaintext fingerprint (Bao root)** | **no** | verify decryption; identify content across encryptions |
| Cryptographic suite identifier and version | no | resolve one compatible pairing, credential-KEM, envelope, delivery-proof, payload-cipher, and verification composition |
| Payload cipher identifier, counter layout, IV, piece size, piece-group size, and alignment | no | deterministic block arithmetic and key granularity across the swarm; each is a per-deployment choice within what the adapter declares |
| KDF, hash-to-scalar, and encoding identifiers | no | interpret every derivation, identity mapping, and proof identically; suite defaults, overridable at registration within what the adapters declare |
| Parameter-set identifiers, one per live set, each with its header sidecar root and locator | no | resolve which credentials decrypt this body and authenticate the capsules they are exercised against |
| Delivery-statement version, attempt-rule parameters `τ_soft` and `τ_wallet`, and `minSettlementTier` | no | interpret and validate every delivery and every attempt identically |
| Registry pointer / entitlement-state adapter | no | resolve publisher lineage, exact entitlement ownership, and ownership interval |
| Upstream attestation (source-dependent) | no | bind to the artifact the ingest source served, where that source provides one |
| Advisory backlink root | no | entry point for [Content Governance](#deprecation-advisory-flags-and-content-governance) governance metadata |
| Transport locator set | partly (one transport's own address) | where the object can be obtained; each serving transport adapter contributes its own locator, and a multi-homed deployment carries several |

#### What the Plaintext Fingerprint Buys

The plaintext fingerprint is referenced elsewhere in this specification as a means of proving that ciphertext and plaintext correspond. That is the smallest of its uses. Because it is an identifier for *content* rather than for any particular encryption of that content, it provides:

* **Decryption correctness.** The client can prove it holds the right credential, exercised against the right sidecar, and produced canonical output, incrementally and per Bao chunk (see above).
* **Encryption-independent content identity.** Two deployments with different capsule randomness produce entirely different ciphertext but the *same* plaintext root. Identical content is therefore provably identical across independent encryptions.
* **Deterministic resolution of First Finder races.** When two nodes ingest the same asset concurrently, the loser of State-Locked Escrow Registration can *prove* its discarded object was the same content rather than trusting the identity hash alone — and any third party can verify the equivalence.
* **Re-deployment without loss of identity.** A publisher issuing a new deployment of an asset produces new ciphertext whose plaintext root is unchanged, demonstrating that nothing about the content changed. Because rotation is versioning rather than replacement ([Content Key Generation](#phase-1-encryption-minting-and-seeding)), the previous deployment remains live alongside it, and the shared plaintext root is what proves the two carry the same content. Migration becomes verifiable rather than asserted.
* **Advisories that follow the content.** A malware or deprecation flag ([Content Governance](#deprecation-advisory-flags-and-content-governance)) targeting the plaintext fingerprint remains attached across re-encryptions, re-deployments, and re-uploads under new identities. Flags cannot be shed by repackaging — which is the single most common evasion in existing package ecosystems.
* **Binding to Web2 sources of truth.** The plaintext root is checkable against the upstream artifact (the NPM `integrity` sha512, an ISBN/DOI record, a publisher's own release hash), anchoring on-chain identity to the artifact the world already recognizes.
* **Deduplication at the identity layer.** Randomized piece-group keys make ciphertext deduplication impossible by design ([Core Philosophy](#core-philosophy)). Plaintext fingerprints restore the ability to *recognize* duplicate content without weakening key secrecy — dedup of knowledge, not of bytes.

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
      ┌───────┼───────┐                    ▼
      ▼       ▼       ▼            credential for THIS interval
 CIPHERTEXT SIDECAR PLAINTEXT      + current state view
   root      root     root                 │
      │       │       │                    │
┌─────┼─────┐ │       │                    │
▼     ▼     ▼ ▼       │                    │
proof proof proof     │                    │
│     │     │ capsule │                    │
▼     ▼     ▼ │       │                    │
piece piece piece     │                    │
A     B     C │       │                    │
│     │     │ │       │                    │
└─────┴─────┴─┼───────┼────────────────────┘
            │ │       │
            ▼ ▼       │
   payload cipher ◄───┼──── piece-group key, unwrapped with the
            │         │      credential from this group's sidecar entry
            ▼         │
        plaintext ────┘
            │
            ▼
    verified against
     PLAINTEXT root

```

Three roots and one gate. The ciphertext root authenticates pieces a seeder cannot read; the sidecar root authenticates the capsules a credential is exercised against; the plaintext root authenticates what a decryption produced. Between them sits the holder's credential together with a current state view for the attempt — possession of a credential is not a substitute for the view ([Invariant Requirements](#overview-and-invariant-requirements)).

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

        current state view covers
        this attempt at my interval?

                 ↓

        decapsulate this group's capsule
        with my credential, unwrap the key,
        decrypt piece

                 ↓

        verify BLAKE3/Bao proof
        against PLAINTEXT root

                 ↓

       "I was given valid material,
        and this is the canonical
        plaintext."

```

## Security Properties

* Confidentiality — parties holding no credential cannot derive decryption capability from the ciphertext, the header sidecar, the ledger's envelopes, or the delivery proofs. For the MVP suite this holds under SXDH, decisional BDH-3b, and the random-oracle model ([Cryptographic Primitives](#cryptographic-primitives)).
* Incremental integrity — individual Bao chunks can be verified independently without possessing the complete asset.
* Content authenticity — the expected Merkle root is bound to the canonical deployment identity.
* Decryption correctness — a client can verify that the plaintext it obtained is the canonical plaintext, independently of which credential produced it. Available under the Public and Keyed plaintext root disclosure modes ([Cryptographic Primitives](#cryptographic-primitives)); forgone under Absent, which is the cost that mode pays for masking.
* Authorization — only the identity that holds the entitlement at a settlement receives that interval's credential, verified by proof, and a conforming client exercises it only under a current state view.
* Per-interval credentials — every ownership interval's credential is cryptographically distinct from every other's, so the protocol can name the authorized credential of the open interval and treat any other as stale; possession of retained material is not authorization and is outside the modified-client threat boundary.
* Delivery soundness — a settlement the contract accepts has delivered a valid credential for the exact entitlement to the registered buyer keys. A seller cannot be paid without delivering, and a buyer cannot obtain the envelope before payment is locked.
* Bounded lag — the seller's conforming capability ends when the transfer reaches `HARD`, so the maximum interval between a transfer and the seller's loss of capability is the chain adapter's latency from the declared tier to `HARD`, stated per deployment rather than left as an implementation artifact.
* Bounded reversal exposure — a settlement reverted after the buyer read its envelope leaves that buyer one entitlement's credential and no entitlement, the same bounded quantity as a seller leaking its own credential; the seller keeps both credential and entitlement. See [Replay Attacks](#replay-attacks).
* Transferability — the credential follows the on-chain entitlement: the seller authors the buyer's credential from its own, and no publisher or service is involved.
* Enforced revocation — after entitlement loss the ledger delivers no further credential to the former holder, and mandatory client-side destruction at `HARD` removes the reference client's existing capability. Neither claim reaches material deliberately retained by modified software.
* Entitlement attribution — a leaked usable credential identifies the entitlement it was issued for and cannot be converted to another entitlement's. It does not identify the interval, and a leaker can rerandomize before leaking.
* Non-selective availability — no party sits on the read path, and a transfer needs no party but the seller and the buyer.
* Seeder agnosticism — possession of encrypted pieces, or of the header sidecar, does not confer content access.
* Plaintext non-revocability — the protocol makes no claim to erase plaintext already obtained by a user.

## Protocol Flow

### Phase 1: Encryption, Minting, and Seeding

The protocol supports both explicit content creators (Publishers) and automated, non-owner proxies ("First Finders").

1. **Canonical Identity Pre-Check:** Before performing any local cryptographic operations, the client queries the on-chain `Registry` using the asset's deterministic Web2 metadata hash.

* **If a record exists:** The client halts the First Finder workflow, fetches the record's transport locator set and live parameter sets from the ledger, and joins the existing swarm as a standard consumer/seeder. Each locator is contributed by a transport adapter serving the deployment, so what the record exposes follows from which adapters resolved rather than from any protocol-level transport commitment. Where the compatibility adapter is among them, the locator set includes a standardized magnet link and its exact BTIH, and traditional BitTorrent clients participate in the swarm without any blockchain integration.
* **If no record exists:** The client proceeds as the authorized First Finder, establishing that this is the network's initial ingestion point for the asset.

2. **Ingest & Deterministic Normalization (Dual-Hash):** The First Finder acquires the asset through an **ingest source adapter** (`IIngestSourceAdapter`), which resolves where the bytes originate. NPM is the MVP implementation; pnpm, Bun, PyPI, crates.io, a DOI resolver, or a publisher's own release endpoint are peer implementations rather than special cases.

The adapter's contract is source-independent, and it is the contract rather than any particular source that the protocol depends on:

* **Immutable, deterministically-hashable bytes.** The First Finder does not repackage the asset; it fetches and strictly hashes the artifact exactly as served. For the NPM implementation that is the immutable `.tgz` tarball, taken as-is.
* **Upstream release attestation, where the source provides one.** The attestation is whatever the source publishes that binds the package identity to the artifact's digest under a key the source controls. It is the digest's provenance, never the digest itself: a bare integrity value establishes only that some bytes hash to it, and a registrant that substitutes bytes can publish a matching one. For NPM the attestation is the registry signature — ECDSA over P-256 on the string `name@version:integrity`, published in the packument's `dist.signatures` and verifiable against the keys the registry serves at `/-/npm/v1/keys` — together with the `integrity` sha512 it covers. An ISBN/DOI record, a signed release hash, or a transparency-log entry serve the same role for other sources. Where a source offers none, the canonical record states its absence rather than implying an attestation exists. The attestation is checked by the First Finder before registering; by the registry on chain at registration, against source keys held in a governance-updated table, wherever the launch network can verify the source's scheme — Base exposes P-256 verification through the RIP-7212 precompile — so a registration that declares an attestation is refused unless it verifies, while a registration that declares absence is admitted with the absence recorded; and by every conforming client, which verifies the bytes it decrypts against the attested digest, where the record carries one, before committing or serving them. A registration carrying a genuine attestation but commitments over other bytes passes the chain, which cannot see the bytes, and is refused by every client that decrypts it, which can; what happens to such a deployment is defined under State-Locked Escrow Registration below.

A bootstrapping event necessarily originates from some pre-existing source. That is a property of bootstrapping itself, not a dependency on any particular registry, and **once the object is in the swarm its canonical reference is the on-chain record**. The ingest relationship ends at registration and is never consulted again.

The adapter faithfully ingests what the source served. Whether that content is *safe* is an advisory-layer question ([Content Governance](#deprecation-advisory-flags-and-content-governance)) and never an ingest-layer one: the attestation binds the canonical record to the artifact the upstream source published, and makes no claim whatever about that artifact's contents. The protocol has no control over what a source publishes and does not pretend otherwise.

#### Ingest Source Eligibility

The adapter interface is source-independent, but the *selection* of sources is not a free choice. Non-Publisher Bootstrap ([Invariant Requirements](#overview-and-invariant-requirements)) permits a First Finder to ingest an asset whose owner has not arrived, and Content Bootstrap Does Not Confer Publisher Rights ([Invariant Requirements](#overview-and-invariant-requirements)) settles what the bootstrapper gains by doing so — nothing. Neither invariant addresses whether the bootstrapper was entitled to *acquire* the bytes in the first place. Escrow is deferred ownership identification; it is not an acquisition license.

**Until [The Post-Claim Pricing Paradox](#the-post-claim-pricing-paradox) is resolved, ingest adapters target content its rights holder has distributed to the public at no charge; npm availability is the MVP's test of that choice, and the residual it leaves is redistribution without the publisher's grant and irrevocability, never denied revenue.** This is a constraint on adapter selection, not on adapter design: the interface, the attestation contract, and the as-is hashing rule are unchanged, and a future adapter pointed at a licensed catalog is a policy decision rather than an engineering one.

Two independent reasons hold the line:

* **Exposure concentrates in the acquisition path.** The swarm carries opaque ciphertext ([Seeder Agnosticism](#seeder-agnosticism)), entitlements are auditable bearer assets ([Entitlement Auditability](#entitlement-auditability-vs-content-key-traceability)), and transfer is ordered and irrevocable — none of these is where a rights holder's claim lands. The claim lands on how the bytes were obtained. This is the distinction courts have already drawn against the machine-learning corpora built over the last decade: transformative *use* of lawfully acquired works has been sustained, while wholesale acquisition from shadow libraries has not. A protocol whose entire ingest surface is one adapter interface should not discover this after the interface has shipped against a paid catalog.
* **A free archive makes the post-claim pricing paradox inert.** [The Post-Claim Pricing Paradox](#the-post-claim-pricing-paradox) turns on a claimant arriving to find their asset already circulating under grandfathered $0.00 entitlements they cannot reprice. Where the content was free to use before ingest, the arriving maintainer was denied no revenue, so the paradox has no economic content and the escrow model can be proven in production without it. Bootstrapping against a permissively-licensed archive is therefore load-bearing rather than incidental to the MVP's choice of NPM.

The reciprocal follows: an adapter aimed at content that is *not* already free to use may not ship until [The Post-Claim Pricing Paradox](#the-post-claim-pricing-paradox) has an answer, because such an adapter converts an unresolved economic question into an unbargained taking. The eligibility rule is what keeps that crossing deliberate rather than emergent.

The acquired archive is run through a dual-hash pipeline: an identity hash (e.g., `BLAKE3(packageName @ version)`) is generated to represent the asset on the Registry, while the primary payload undergoes a separate structural hash to build the CAN streaming manifest.
3. **Deployment Cryptographic Setup:** The selected cryptographic suite produces the parameter set and master scalar of the issuing authority if none is live for the asset yet, the payload-encryption parameters, the header sidecar, and the verification material committed by the deployment descriptor. A parameter set belongs to the asset, not to the deployment: every deployment of the asset carries a sidecar under every set live for the asset at its registration, and a sidecar for a set made live later is added to a live deployment by the asset's authority ([Escrow Key Lineage Across a Claim](#escrow-key-lineage-across-a-claim)), which is what lets one credential reach every live deployment. A credential opens only deployments carrying a sidecar for its set, and the conforming client selects among live deployments accordingly. The suite has two secret-generation lineages; they are not interchangeable, and which one applies follows from who is registering rather than from any preference. Both satisfy the invariant of [Core Philosophy](#core-philosophy); neither derives anything from the plaintext.

**First Finder lineage — random, retained as escrow custodian.** A First Finder generates the master scalar by cryptographically secure random draw and **no publisher hierarchy exists above it.** This is not an omission to be repaired later: the First Finder has no publisher secret to derive from, and manufacturing one would leave a non-owner holding derivation authority over an asset they do not own. The First Finder authors the first grant under the master scalar and **retains the scalar as escrow custodian** only for the optional handover at claim; it is not the asset's sole credential source, because the escrow suite's asset identity scope lets every holder author grants ([Credential KEM](#credential-kem-deployment-encryption-adapter)). It issues nothing except against grants the escrow contract authorizes, and erases its copy after an acknowledged handover to the claimant — a shortcut, never a dependency, because a claim never needs it ([Escrow Key Lineage Across a Claim](#escrow-key-lineage-across-a-claim)). Retention confers no right: the escrow contract, not the scalar, determines legal issuance, and a copy the First Finder keeps beyond that point can author decrypting credentials but no entitlements, which is plaintext-equivalent capability it necessarily had from the moment it encrypted the object.

**Publisher lineage — derived, recoverable.** An explicit publisher derives deterministically from their own private seed phrase, **layered and domain-separated by asset**, never a bare seed-to-key mapping:

```text
    seed_phrase                       (publisher's recoverable secret, never leaves the client)
         │
         ▼
    publisher_root = KDF(seed_phrase)
         │
         ├──── asset_identity_hash    (BLAKE3(name @ version) — the canonical Registry key)
         ▼
    asset_root = KDF(publisher_root, asset_identity_hash)
         │
         ├──── "master-scalar"        ──▶  α, the parameter set's master scalar (per asset, all deployments)
         ├──── "identity-bases"       ──▶  u0, u1 of the parameter set
         │
         ├──── deployment_id          (distinguishes re-deployments of the same identity)
         ├──── group index
         ▼
    t_j = KDF(asset_root, deployment_id, group index)     capsule randomness, one per piece group
    k_j = KDF(asset_root, deployment_id, group index, "piece-group-key")   the piece-group key, wrapped under every live set
```

* **Recoverability:** a publisher who retains only the seed phrase can regenerate the parameter set and master scalar for every asset they have ever published, and therefore mint under them again, without any stored key material.
* **Per-asset isolation:** because `asset_identity_hash` is mixed in, every asset a publisher releases has an independent parameter set. Compromise of one asset's master scalar does not expose any other asset by the same publisher, and does not expose the root.
* **Nonce-invariant safety (mandatory):** the `deployment_id` and group-index layers exist to guarantee that no capsule randomness, and hence no piece-group key, is ever reused with the deployment IV. AES-CTR catastrophically leaks plaintext XOR under keystream reuse, so a publisher re-encrypting the same asset **must** derive fresh capsule randomness via a new `deployment_id`, even when the plaintext and identity hash are unchanged. Deriving capsule randomness without the deployment and group layers is a protocol violation.
* **Anti-derivability is preserved:** the derivation inputs are the publisher's private secret and public identifiers — never the plaintext payload. Possession of the plaintext confers no ability to compute any key (see [Core Philosophy](#core-philosophy)).
* **`deployment_id` is assigned by the Registry, not chosen by the publisher (defined behavior).** A publisher-chosen identifier invites deliberate collision — reusing an identifier reproduces capsule randomness, and a publisher who can reproduce it can construct two deployments that are indistinguishable in the derivation while differing elsewhere, which is deployment-as-deniability. Ledger assignment makes uniqueness a property of consensus rather than of publisher good behavior, and it makes the registry authoritative over how many deployments an asset has. The normative requirement underneath it is narrower and holds for any assignment scheme: **`deployment_id` must be unique for every `(asset_root, deployment)` pair**, without exception and without reuse after a deployment is retired.
* **The derivation seed is not the publisher's identity key.** The seed phrase is a local secret that never leaves the client and exists solely to derive the parameter set and capsule randomness; the publisher's chain identity and publishing authority are separate objects held by the identity adapter and transferable independently ([Parameter-Set Registration](#phase-1-encryption-minting-and-seeding)). Rotating the identity or authority key therefore changes nothing about key derivation and does not version cyphertext, which is what the corresponding [Invariant Requirements](#overview-and-invariant-requirements) invariant asserts. The converse is the real constraint: **the seed is a recovery secret, not a rotatable credential.** A publisher who discards a seed loses the ability to re-derive the master scalar for every asset derived under it and can mint again only by registering a new parameter set ([Escrow Key Lineage Across a Claim](#escrow-key-lineage-across-a-claim)); a publisher adopting a new seed derives future assets only. Neither operation affects existing entitlements, deployments, or swarm objects.

4. **Payload Encryption & Hashing:** The asset is divided into pieces aligned to the CAN piece size and the resolved payload-cipher adapter's block boundaries, and into piece groups of the suite's declared size. For each group the encryptor draws the piece-group key, random under the First Finder lineage and derived as `k_j` under the publisher lineage, and encrypts the group's pieces through the payload-cipher adapter ([Cryptographic Primitives](#cryptographic-primitives)). Then, for each parameter set live for the asset, the credential KEM produces one capsule and its encapsulated value per group, the set's wrapping key is derived from that value and the context, and the piece-group key is wrapped under it. The header sidecar under a set is the sequence of capsule and wrapped key per group, one sidecar per live set, each committed by its own BLAKE3/Bao root; a single-set deployment uses the same layout, so there is one format and one code path. The piece-group keys are common to every holder under every set, which is the accepted residual of [Entitlement Auditability](#entitlement-auditability-vs-content-key-traceability); they exist at the encryptor, which necessarily holds the plaintext, and at holders, and nowhere else. The ciphertext is structurally hashed to build the verified streaming manifest.
5. **Trustless Seeding:** The encrypted pieces, the manifest, and the header sidecar are seeded to public, unauthenticated CANs. Seeders host opaque bytes blindly; the sidecar is public and useless without a credential.
6. **Parameter-Set Registration:** The deployment record registers the suite identifier, the parameter set as live for the asset, and the sidecar root under that set. Publisher and escrow authority resolve through the Abstract Identity Adapter, while the conforming client's per-attempt check resolves separately through the entitlement-state interface defined below; neither authority path is substituted for the other.

* *Explicit Publisher:* Registers via a Publisher Authority Adapter (e.g., a transferable bearer asset/ERC-721 token or cryptographic DID adapter). Copyright and publishing ownership are transferred simply by transferring the underlying authority token or updating identity resolution, without requiring protocol or asset state rewrites.
* *First Finder Escrow:* Registers via an Escrow Identity Adapter (e.g., `PackageJsonAdapter`), under an escrow suite whose credential KEM fixes the asset identity scope. The escrow contract authorizes zero-price grants and mints each entitlement; the credential for a grant is authored by **any current holder of a credential under the escrow set**, from its own credential, with the transfer proof against its own current envelope, and the conforming client serves pending grant requests for assets it holds automatically, as it seeds them, under the relayer's policy, including requests from identities that are offline, since the credential is encrypted to the requester's registered envelope keys and the envelope is recovered from chain history. The First Finder is only the first such holder: it authors the first grant under its master scalar with a mint proof and is thereafter one holder among many. No grant depends on any particular party, and an asset whose First Finder vanishes stays open to new participants for as long as one holder is online. That is a stricter condition than its swarm having a seeder: a conforming holder seeds, so a holder online implies a seeder online, but a blind seeder holds no credential and can author no grant, and a swarm can be healthy while nobody in it can grant. The project seed host therefore holds an entitlement, and so a credential, for every asset it seeds, so that its availability covers grants and not only bytes ([MVP Scope, Project Seed Host and Site](MVP%20Scope.md#project-seed-host-and-site)). This continues until the Web2 maintainer authenticates via the adapter and claims administrative control or swaps to a Publisher Authority Adapter, after which the escrow contract authorizes no further grants. Existing holders read and transfer with no involvement of anyone.

#### Escrow Key Lineage Across a Claim

A claim does not re-key a deployment, and it does not need to. The escrow parameter set was generated by the First Finder and sits under no publisher hierarchy, but the credentials issued under it decrypt every deployment that carries a sidecar for that set, and encryption under any published parameter set is a public operation. The claim therefore performs three things, and it is worth being explicit that none of them depends on the First Finder:

* **Administrative authority transfers without touching custody.** Deployment₀ remains live and decryptable by every escrow-era credential. The claim changes who controls publishing and issuance; it neither gives the claimant an exclusive read-path key nor gives anyone a veto over existing holders, who never contact any party to read or transfer.
* **Publishing authority transfers, under the claimant's own parameter set.** The claimant registers a parameter set of its own as live for the asset, and future issuance resolves to its Publisher Authority Adapter under that set. If the First Finder is reachable it may additionally hand over the escrow master scalar, encrypted to the claimant's registered envelope keys with a proof that the encrypted scalar matches the escrow set's public element; this is a convenience that lets the claimant also mint under the escrow set, and a claim completes identically without it.
* **The claimant acquires the standing to supersede, and every later body covers every live set.** The claimant may issue deployment₁ priced and managed as any asset they published themselves. Its body carries one header sidecar per parameter set still live for the asset, so escrow-era holders follow it without re-issuance. Deployment₀ does not become invalid — the Distribution Invariants permit deployments to coexist, and the shared plaintext root is what proves the two carry the same content.
* **Existing deployments gain the claimant's set by sidecar addition, never by re-encryption.** A credential under the claimant's set cannot open deployment₀ until deployment₀ carries a sidecar under that set, and the sidecars a deployment carried at registration are immutable. The claimant, holding the asset's authority, adds one: it obtains an escrow-era credential like any other participant, a zero-price grant from any holder taken before the claim closes escrow grants, recovers every piece-group key from deployment₀'s escrow sidecar, wraps each under its own set, and registers the new sidecar's root against deployment₀ through the registry's sidecar-addition function. The ciphertext is unchanged, so nothing is re-seeded, and no First Finder is involved. Where no holder is reachable the claimant, who holds the plaintext, issues deployment₁ from it under both sets instead. Either way every live deployment comes to carry every live set; until it does, a holder whose credential is under the claimant's set resolves only to deployments carrying that set, and the conforming client never selects a deployment its credential cannot open.

**Entitlements survive the transition without action, because they bind to the asset rather than to a deployment.** A holder reads deployment₁ with the credential it already holds, whether it migrates at transfer or voluntarily. An escrow-era holder may at any time request a replacement credential under the claimant's set, which the claimant mints for the same entitlement in one settlement that supersedes the escrow-era credential; refusal leaves the holder whole, so this step needs no non-deniability, and it is how the escrow set's population shrinks. A parameter set is retired, and later bodies stop carrying its sidecar, only when the registry shows no live entitlement under it. The swarm reorganizes around the successor over time rather than at an instant, and a `superseded-by` advisory ([Advisory Backlinks](#advisory-backlinks)) is the signal that recommends it — advisory and never obligate, so a participant seeding or reading deployment₀ is never broken by the existence of deployment₁.

The same mechanism is a publisher's recovery from a lost master scalar: register a new parameter set and continue, at the cost of one more sidecar per body while the old set has live entitlements. Custody of the master scalar is the publisher's liability, as key custody is every participant's.

The consequence for the no-drift invariant is the one stated in [Invariant Requirements](#overview-and-invariant-requirements): equivalence of *rights*, not of key lineage. A claimed asset carries a parameter set the claimant did not originate. That is the visible trace of having been bootstrapped by someone else, and it is the price of the asset having been distributed at all before its owner arrived.
* **State-Locked Escrow Registration:** If two independent non-owner nodes ("First Finders") attempt to ingest and register the same unlisted package simultaneously, the on-chain `Registry` enforces a strict State-Locked Escrow Registration on the asset record. The transaction that lands first in the block wins, establishing the canonical identity, its first escrow deployment, and that deployment's escrow binding and parameter set. The losing node's client catches the on-chain revert, discards its locally generated ciphertext, sidecar, and master scalar, and automatically switches to pointing its installation pipeline to the winning node's registered CAN infohash. The lock is on the asset record, not on how many deployments the asset has: the Distribution Invariants let deployments coexist, and a later First Finder may register a further escrow deployment of the same asset under its own escrow set and binding. That is what keeps an identity recoverable when a first deployment's bytes fail the attestation check at every client that decrypts them: such a deployment is dead, not the identity poisoned. Conforming clients refuse it, record an unverifiable advisory against it, and resolve to a deployment whose bytes they have verified; escrow grants and the claim resolve against the asset across all of its escrow sets, and the claimant retires a dead deployment's set as it retires any other.

7. **Minting:** Entitlements are registered on-chain as standard, transferable bearer assets, **against the asset rather than against this deployment**. **The MVP implementation is an ERC-721 non-fungible token on the target chain**, which makes minting, holding, and transfer ordinary operations with well-understood tooling rather than bespoke ones. As everywhere else in this specification the implementation is named and is not the definition: an entitlement is whatever the chain adapter can mint, own, and transfer under single-owner semantics, and a chain whose native construction differs supplies its own adapter. Ownership changes only through the settlement operations that carry delivery — mint, grant, and transfer with a verified envelope — and the token's standard transfer and approval entry points are disabled, so no ownership change can leave its recipient without a credential or the interval state behind the ownership; a receiver callback, where the token standard has one, runs after the contract's state is final and under a reentrancy guard.

**Every mint delivers the first credential.** The recipient has registered its envelope key pair; the issuer authors a credential for the exact entitlement under its master scalar, encrypts it to those keys, and posts the envelope with a mint proof through the credential-delivery boundary ([Cryptographic Primitives](#cryptographic-primitives)). The contract verifies the proof and records interval zero, the recipient's keys, and the envelope digest in the same transaction that creates the entitlement. Minting is the issuer's discretionary act, so an issuer that does not post has simply not minted; nothing about an existing entitlement depends on it.

**Supply is unbounded and publisher-controlled.** There is no maximum, no protocol-imposed schedule, and no requirement that supply only increase — a publisher may mint further entitlements whenever they choose, acting as market maker over an asset they own. What the protocol fixes is *who* may mint for a given asset: the party the identity adapter resolves as holding publishing authority, which for an escrowed asset is the escrow binding until a claim moves it. Escrow-issued entitlements are therefore a supply the arriving claimant did not authorize, which is the substance of [The Post-Claim Pricing Paradox](#the-post-claim-pricing-paradox) rather than a defect in this rule.

**Variant seed authorship is deliberately absent from this phase.** Under per-entitlement variance ([Variance Belongs to the Entitlement](#variance-belongs-to-the-entitlement-not-the-content)) a deployment would also establish how each holder's variant seed is authored. Who authors it, from what material, and how the escrow case is served when the publisher is absent are unresolved and recorded in [Variant Seed Authorship](#variant-seed-authorship). Nothing in Phase 1 should be read as settling them.

#### Abstract Identity Adapter & Generic Content Identity

To support generic content (movies, ebooks, audio, binaries, software), publisher and escrow authority are decoupled via an abstract verification interface (`IIdentityAdapter`). It answers whether a proved identity may register, claim, mint, or administer an asset. The conforming client's per-attempt check is a separate Registry *view* over an `AttemptContext`, evaluated through the deployment's entitlement-state adapter; publisher authority is necessary to establish valid issuance lineage and is never sufficient to authorize a consumer's attempt.

`AttemptContext` contains the asset and deployment identifiers, exact entitlement contract and token, the identity and its bound handshake key, the interval index the client's credential was issued for, and the settlement reference of the view. `Registry.evaluateAuthorization(context)` returns `AUTHORIZED`, `PENDING_SETTLEMENT`, or `DENIED`; `evaluateAuthorizationBatch` is only a batching form of that identical operation. It is a view the client reads for itself; no party calls it on the client's behalf.

The interval index is consensus-derived and changes on every mint or transfer, including a transfer back to a previous owner; it is the same counter the credential-delivery boundary advances at each settlement, and the credential, the envelope, and the registered envelope keys are all recorded against it. The MVP entitlement contract exposes it as a monotonically increasing per-token transfer counter; another entitlement-state adapter may expose a canonical transition identifier with the same uniqueness and historical-resolution properties.

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

[ First Finder Pushes Package ] ---> [ Registry Records Escrow Parameter Set via Escrow Adapter; First Finder Serves Zero-Price Grants ] | v [ Maintainer Proves Identity (ZK-Email/DNS) ] -> [ Registry Updates Authorization ] | v [ Maintainer Upgrades Adapter ] --------------> [ Full IP Control (Transferable Asset) ]

### Phase 2: Consumption and Decryption

Access is local: the holder already has its credential from the settlement that delivered it, and nothing is requested from anyone at read time. Two distinct operations run in this phase and must not be conflated: **entitlement acquisition**, which may be skipped when the identity already holds the entitlement, and **per-attempt authorization**, which may never be skipped, presumed, or cached beyond the deployment's freshness bound.

1. **The Download:** The consumer fetches the CAN manifest, the header sidecars, and encrypted pieces, in any order, from available peers. None of this needs a credential, which is why seeders hold all of it; a consumer that already holds a credential may fetch only the sidecar of its parameter set. Each sidecar is authenticated against its root and each capsule is checked for well-formedness before use.
2. **Entitlement Acquisition Check:** The client inspects its own **wallet** and filters out every target for which it already holds an entitlement, because those entitlements need not be acquired again. This filter concerns *acquisition only*. It never determines whether decryption may proceed.
3. **Credential Availability:** For each entitlement the client holds its persistent credential — the envelope key pair, the envelope from the settlement that delivered it, and the interval index. At start-up it decrypts the envelope into memory through the key-agreement adapter and checks the credential's validity against the parameter set; a client that has lost the envelope recovers it from chain history through any full-history node, and a client that has lost its envelope secrets can still read while it holds the decrypted credential but can no longer transfer. The decrypted credential and everything derived from it are never written to disk.
4. **Per-Attempt Authorization:** Before unwrapping any piece-group key, the client holds a state view at the deployment's declared tier or above, no older than `τ_soft`, obtained by reading `Registry.evaluateAuthorization` over its `AttemptContext` through the entitlement-state adapter. The view confirms valid issuance lineage to the registered publisher or escrow authority, ownership of the exact token by the client's identity at the interval its credential was issued for, and the deployment binding; the client's session separately proves control of a device key the identity admits in that same view, renewed every `τ_wallet`; a view that shows the device key revoked denies the attempt, and the conforming client purges its envelopes and envelope secrets as it does at interval end. There is no exemption for assets decrypted previously — an entitlement held for years authorizes nothing until a current view covers the attempt. Batching applies to state *reads* across dependency trees; it never substitutes for them.

The view returns three states because authorization is transactional against evolving state and "not settled yet" is not the same answer as "no". During an install the entitlement is frequently minted seconds before it is read, so the client is reading state it has just written:

| Result | Meaning | Client behavior |
| --- | --- | --- |
| `AUTHORIZED` | the entitlement resolves to the client's identity at its credential's interval, at the required tier | proceed to decryption |
| `PENDING_SETTLEMENT` | the entitlement or its latest transfer is visible but has not attained the required tier | retry under defined backoff; not an error |
| `DENIED` | the entitlement does not resolve to the client's identity at that interval | terminal; no retry |

Collapsing `PENDING_SETTLEMENT` into `DENIED` fails valid installs; collapsing it into `AUTHORIZED` authorizes against state below the declared tier. A client that cannot distinguish them either aborts a legitimate acquisition or retries an invalid one indefinitely.
5. **Decryption & Asynchronous Streaming:** For each piece group the client decapsulates the group's capsule with its credential, derives the set's wrapping key, unwraps the piece-group key, decrypts the group's pieces through the payload-cipher adapter, validates them against the BLAKE3 roots, and commits them to the local CAS while further pieces arrive. Piece-group keys, expanded cipher state, and buffered keystream live only in the decryption context. Plaintext written to the local CAS persists by design and is outside the authorization boundary ([Modified Clients](#modified-clients-the-honesty-assumption), [Partial Encryption](#future-optimization-partial-encryption)).
6. **Interval End:** When the client observes a transfer of the entitlement out of its identity at the deployment's declared tier, it stops new attempts; when that transfer reaches `HARD`, it actively destroys the decrypted credential, all piece-group keys, expanded cipher state, buffered keystream, and every live decryption context, so that a reorganized transfer never strands a still-owner. The persistent envelope and keys may be kept, since they decrypt nothing the ledger still authorizes. Retaining decrypt-capable state past this boundary is non-conforming.

### Phase 3: Secondary Transfer and Authorization Expiry

A transfer is the one moment a credential changes hands, and it happens inside the settlement, between the seller and the buyer alone. The seller's capacity in the conforming protocol ends when the transfer reaches `HARD`; the transfer requires no client-side revocation signal, because the ledger no longer resolves the entitlement to the seller and no party will ever deliver the seller another credential for it.

1. **Buyer Registration:** The buyer registers its envelope key pair, if it has not already, with a proof of possession for each key under a wallet signature; a key pair may be reused across entitlements and is rejected if either key is the identity element.
2. **Payment Lock:** The buyer locks payment against the exact entitlement, the seller, the price, and an expiry.
3. **Delivery:** Before the expiry the seller decrypts its own envelope afresh, rerandomizes the credential with a private offset, encrypts the result to the buyer's registered keys, and posts the envelope with a transfer proof, supplying the previous envelope from the entitlement's record as the proof's statement. The contract checks that envelope against the stored digest, loads the seller's registered keys and the entitlement's identity element from the record, verifies the proof, and in the same transaction advances the interval counter, records the buyer's keys and the new envelope digest, emits the envelope, transfers the entitlement, and releases payment. Step three is deterministic, so a seller cannot be paid without delivering and a buyer cannot obtain the envelope before payment is locked.
4. **Expiry:** If no valid delivery arrives by the expiry, the buyer withdraws the lock. No seller forfeit is required by the protocol; a listing bond against sellers who let locks expire is a market-design choice outside this specification.
5. **Bounded Lag:** Between the settlement reaching the declared tier and reaching `HARD`, both the buyer and the seller can decrypt: the buyer because its credential is delivered, the seller because the reference client destroys only at `HARD`, so that a reorganized transfer never strands a still-owner. This interval is the maximum lag referenced by the Atomic and Bounded invariant ([Invariant Requirements](#overview-and-invariant-requirements)), and it is the chain adapter's latency from the declared tier to `HARD` — a property of the chain the deployment's adapters resolve to, published through the declared `minSettlementTier` ([Cryptographic Primitives](#cryptographic-primitives)) as a **per-deployment observable** rather than an artifact of whichever chain was used.
6. **Material Discard:** At `HARD` the reference client actively destroys the decrypted credential and every decrypt-capable derivative or context. The persistent envelope and keys need no destruction, because the ledger authorizes nothing they decrypt. This destruction is a substantive requirement of the reference composition; it is not a cosmetic validity check and is not claimed to defeat modified software.
7. **Optional State Monitoring:** A client may observe entitlement transfers via an RPC node in order to stop attempts early and to present accurate state to the user. This is a user-experience affordance; correctness depends only on the per-attempt state view.
8. **Plaintext Agnosticism:** The client does not act as malware. It does not attempt to flush RAM buffers of already-rendered frames, hunt down exported files, or delete user-saved plaintext.
9. **Continued Network Support:** The user deliberately retains the *encrypted* pieces and the header sidecar in their local CAN storage. The client continues to act as a seeder, strengthening the swarm, despite the user no longer holding a credential the ledger authorizes.

The protocol provides that no further credential is ever delivered to a former holder and, for conforming clients that destroy at `HARD`, loss of the ability to decrypt cyphertext. It makes no claim to revoke previously decrypted plaintext or decrypt-capable material deliberately retained by modified software; a prior holder's retained credential is protocol-invalid and distinguishable from the open interval's, not cryptographically dead.

Reference-client destruction and the ledger's refusal to deliver again are complementary boundaries. Destruction makes the reference client's loss of capability real at `HARD`; the ledger prevents reacquisition after entitlement loss. The modified-client limit applies to both claims exactly as stated in [Modified Clients](#modified-clients-the-honesty-assumption).

#### Asset Ownership Transfer

Selling the asset itself — an author's rights sale to a publisher, a corporate IP transfer — is a transfer of publisher authority, not of an entitlement, and it follows the claim path's custody mechanics rather than the steps above. The authority token or identity resolution transfers as Phase 1 describes. The new owner then either registers its own parameter set as live for the asset and issues under it, with later bodies carrying a sidecar per live set, or receives the master scalar from the previous owner encrypted to its registered envelope keys with the compatibility proof, after which it issues under the existing set. Publishers therefore register envelope keys like any holder. Existing entitlements, credentials, and deployments are untouched either way.

## Security Considerations

### Seeder Agnosticism

Because payloads are piece-encrypted and bound by a BLAKE3 tree, data at rest is opaque. Seeders (including former entitlement holders) cannot access the content, allowing the encrypted files to scale horizontally as public infrastructure.

### Replay Attacks

Every delivery proof hashes its complete statement — both envelopes, both parties, all four envelope keys, both interval counters, purpose, and expiry — into its challenge, so a captured envelope and proof verify for exactly one settlement, and the contract rejects any counter it has already advanced. A captured envelope is useless to any identity but its recipient, and the recipient's own session assertions are bound to its handshake key and renewed on the deployment's schedule.

Replay protection is load-bearing rather than merely prudent. A proof that could be transplanted to another statement would let a seller be paid twice for one delivery, or let an envelope for one entitlement be recorded against another; binding the statement into the challenge is what makes the settlement record an authoritative history of who was delivered what.

#### Settlement Reversal Is Not Replay

Two adjacent failures are frequently conflated and must not be. **Replay** is an adversary resubmitting a captured payload to obtain a capability they were never issued, and it is defeated by the binding above. **Settlement reversal** is a reference below `SETTLED` being reverted after a settlement was validly included at it. Nothing is captured and nothing is resubmitted; the delivery was correctly verified against state that subsequently ceased to exist.

The distinction matters because the remedies differ entirely. Tighter payload binding shrinks the replay surface and does nothing whatever to the reversal surface, which is governed only by the declared settlement tier ([Cryptographic Primitives](#cryptographic-primitives)).

Reversal is not recallable — a buyer who read its envelope holds the credential in memory — and the protocol does not attempt to recall it. It is instead bounded: the exposure is one entitlement's credential, held by an identity that does not hold the entitlement, which is protocol-invalid material of the same class as a seller leaking its own credential; the seller may re-post to complete the sale, and because the reference client destroys only at `HARD`, the reverted seller loses nothing. A deployment selecting a shallow tier accepts a higher frequency of an already-accepted failure rather than a novel one.

The same reasoning distinguishes reversal from the settlement lag of [Bounded Lag](#phase-3-secondary-transfer-and-authorization-expiry), which is likewise not a vulnerability but a published property: in that case the *legitimate prior holder* still holds a credential the reference client has not yet destroyed. Neither is an attack, and neither belongs in the same class as replay.

### Modified Clients (The "Honesty" Assumption)

A user can compile a modified version of the open-source client that retains its decrypted credential past the end of its interval and uses it outside the conforming lifecycle, allowing them to keep decrypting after selling the entitlement. The protocol accepts this edge case. The system's primary directive is ensuring that *an identity holding no entitlement is never delivered a credential*, that the open interval's credential is distinguishable from every stale one, and that the path of least resistance for honest users effortlessly honors creator rights without intrusive friction.

Defeating the unmodified path requires deliberate preservation or extraction of decrypt-capable material and its use outside the conforming lifecycle. The user need not leak that material to another party. The intended enforcement posture is narrower and exact: complying users have no default path past the interval, while bypass requires consciously retaining or exporting material the reference client destroys.

This scope is a design commitment, not a concession. **Decrypted plaintext must run on any compatible device and its existing software** — a video plays in any player, a package installs with any toolchain, a book opens in any reader. The protocol will not require a bespoke runtime, and [Core Philosophy](#core-philosophy)'s rejection of hardware enclaves is the same commitment stated at the hardware layer. Content that only functions inside an environment the protocol controls is precisely the walled garden this design exists to dismantle.

The consequence follows directly: because the plaintext is deliberately released into general-purpose software the protocol does not control, the invariants of [Invariant Requirements](#overview-and-invariant-requirements) bind the settlement contract, credential authors, and conforming clients, and nothing else. They do not bind arbitrary software running on a user-controlled device, and no claim is made that they could.

### Entitlement Auditability vs. Content Key Traceability

These are two different properties and the protocol delivers them to different degrees. Conflating them overstates the protocol's guarantees.

* **Entitlements are 1:1 and fully auditable.** Every access right is a distinct transferable bearer asset with a unique on-chain owner and a complete transfer history. Accounting, residuals, resale, and audit of *who holds a right* are exact and trivially verifiable. This is a genuine and unusual strength. The 1:1 property is a statement about *ownership*, not about instantaneous decryption capacity: because a seller retains capacity until the transfer reaches `HARD` ([Bounded Lag](#phase-3-secondary-transfer-and-authorization-expiry)) while the buyer decrypts from the declared tier, the number of parties able to decrypt can transiently exceed the number of entitlement holders, bounded by the chain adapter's published tier latency. The ledger remains exact; the capacity overlap is a settlement property, not an accounting one.
* **Credentials are 1:1 per interval; capsule values are 1:many.** A deployment is encrypted once and seeded to the swarm as a single ciphertext with one header sidecar per live parameter set ([Payload Encryption and Trustless Seeding](#phase-1-encryption-minting-and-seeding)). Every holder's credential is distinct and bound to its entitlement; every credential under a set recovers the same encapsulated value from every capsule of that set, and every set unwraps the same piece-group keys, which are therefore common to every holder under every set.

Four leak classes with four different outcomes:

| What leaks | Traceable? | Why |
| --- | --- | --- |
| An envelope | **Yes, and useless** | it is encrypted to one identity's registered keys and recorded against one interval; nobody else can open it |
| A decrypted credential | **Yes, to the entitlement** | a usable credential is bound to one entitlement's identity element and cannot be converted to another's; it does not identify the interval, because any holder in that entitlement's chain could have rerandomized it |
| Assembled plaintext | **Yes, against unmodified clients only** | it carries the overlay fingerprint of the holder at assembly time ([Variance Belongs to the Entitlement](#variance-belongs-to-the-entitlement-not-the-content)); a modified client can render canonical plaintext instead and carries no fingerprint |
| A piece-group key, or the set of them, extracted from memory | **No** | it is the same value for every holder of the deployment, and the full set decrypts the whole deployment |

The residual gap is therefore narrower than it was under a shared content key — a leaked credential names its entitlement, and leaked *plaintext*, which is what actually circulates in practice, is attributable against the honest path — but it is real. If a holder extracts and publishes the piece-group keys, those bytes decrypt the universally-distributed swarm copy for everyone at zero marginal cost, and because every holder unwraps the same values, publication does not attribute the leak to any one of them. **On-chain provenance narrows who *obtained* a credential; a published key set narrows nothing.** The piece-group size bounds how much one leaked key unlocks and is chosen with that in view.

This is materially different from, and worse than, the analog hole. The analog hole leaks a rendering; a key-set leak hands over the canonical asset in perpetuity.

### Blast Radius Containment

Because parameter sets are per asset and capsule randomness is per deployment and per group ([Deployment Cryptographic Setup](#phase-1-encryption-minting-and-seeding)), a leaked credential exposes exactly one asset — every live deployment of it, which is what asset-bound entitlement requires — and a leaked piece-group key exposes one group of one deployment. Neither exposes other assets by the same publisher, the publisher's master scalar, or the publisher's seed phrase, and no number of credentials lets their holders recover the master scalar. Leak damage is bounded to the asset, not the catalog.

### Escrow Claim Front-Running & Identity Proof Binding

To prevent front-running attacks during escrow settlement (where an attacker intercepts a maintainer's off-chain verification proof and submits it to claim ownership), state transitions on the `Registry` require identity proofs or ZK-nullifiers to be cryptographically bound to the claimant's target chain identity. Any settlement proof generated for `Address_A` will revert on-chain if executed by or directed to `Address_B`.

Because these proofs are verified inside a contract, they are **chain-layer artifacts by definition** and are produced with the chain layer's signature adapter ([Cryptographic Primitives](#cryptographic-primitives)), never with the protocol's preferred scheme. A proof produced under a scheme the target chain has no precompile for is verifiable only at prohibitive cost, so scheme selection for on-chain verification follows the chain rather than the protocol's preference.

## Deprecation, Advisory Flags, and Content Governance

Canonical records are immutable and swarm content cannot be recalled. The protocol therefore does not attempt takedown; it attempts **informed refusal**. Governance is advisory metadata parsed against the user's own trust set, not registry mutation.

*Framed here for architectural intent. Not implemented in the MVP.*

### Why Not Registry Mutation

Deleting or rewriting a canonical record would break the guarantees the rest of the protocol depends on: reproducible dependency resolution, verifiable provenance, and the irrevocability of purchased access rights. A publisher who could unpublish could also revoke what users paid for — the exact behavior this protocol exists to prevent. The registry stays append-only.

### Advisory Backlinks

Deprecation notices, malware advisories, license disputes, and content classification are expressed as **append-only metadata objects that backlink to the target content hash** — the same mechanism as comments, ratings, and reactions. A flag is a signed assertion by some identity that a given asset has some property. It carries exactly the weight of the identity that signed it.

```text
    content_hash ◄──── advisory backlink { type, severity, signer, evidence_ref }
                 ◄──── advisory backlink { ... }
                 ◄──── counter-assertion  { disputes: <advisory_id>, signer, ... }
```

Advisory types anticipated: `deprecated`, `superseded-by`, `security-advisory`, `malware`, `license-dispute`, `content-classification`, `disputed`.

Consistent with the rest of this section, every signal is **advisory and never obligate** — a client may act on a flag, retain the content anyway, or ignore the assertion entirely, and no participant is required to act on any flag.

### Client Resolution

The client fetches backlinks alongside the asset and evaluates them against a **locally-configured trust set** — signers the user or their organization has chosen to heed. There is no global arbiter, and the protocol does not appoint one.

* Advisories from trusted signers surface as warnings, or block installation, per local policy.
* Enterprises point their trust set at their own security team or a vendor feed.
* Ecosystems may converge on well-known signers (a registry security team, a CVE feed) without any of them gaining protocol-level authority.
* Counter-assertions are visible too. Disputes are surfaced, not silently resolved.

### Unwanted and Illegal Content

The same mechanism is the intended handler for content classification generally, including material that is unwanted, age-restricted, or illegal in a given jurisdiction. The protocol's position is that it cannot and should not adjudicate this globally: it can carry signed assertions, and clients can act on the assertions they trust — including refusing to fetch, refusing to seed, or refusing to display.

Two limits are stated plainly rather than papered over:

* **Seeders host opaque ciphertext and cannot inspect what they carry** ([Seeder Agnosticism](#seeder-agnosticism)). This is a deliberate privacy and scalability property, and it means seeders cannot content-moderate by inspection. Client-side refusal-to-seed based on trusted advisories against the *identity hash* is the available lever.
* **Advisory flags do not remove content from the swarm.** Anyone running a client that ignores the trust set retains access. This is the same honesty assumption as [Modified Clients](#modified-clients-the-honesty-assumption), and the same limit applies.

Jurisdictional obligations for operators of gateways, indexers, and default trust sets are a legal question this specification does not attempt to answer. It is raised here so that the architecture is not later retrofitted under pressure.

## Traceability and Per-Entitlement Variance

*The MVP ships [MVP Posture](#mvp-posture-native-credentials) only. [Variance Belongs to the Entitlement](#variance-belongs-to-the-entitlement-not-the-content) onward is the V2 path and is architecturally protected by the commitment structure in [Cryptographic Primitives](#cryptographic-primitives).*

### MVP Posture: Native Credentials

Under the MVP, content is uniform across all holders. Each holder decrypts with its own native credential, delivered once per ownership interval through the credential-delivery boundary ([Cryptographic Primitives](#cryptographic-primitives)) and exercised locally under the per-attempt state check.

Each credential is **encrypted to the recipient's registered envelope keys**, resolved through the key-agreement adapter and bound to the identity by the `keyAgreement` relationship of its DID Document. The envelope is a delivery property; the credential inside is the grant, and it is distinct per interval by construction rather than by wrapping. It is emphatically not possession-as-right: a credential decrypts nothing under the conforming protocol without a current state view covering the attempt, which the invariants of [Invariant Requirements](#overview-and-invariant-requirements) require.

This buys more than the wrapped-key posture it replaces: no two intervals ever hold the same credential, a copied envelope is useless to any other identity, and a leaked credential names its entitlement. What it does not buy is interval attribution or cryptographic expiry: a holder that extracts its decrypted credential keeps a working key, and can rerandomize it before leaking. Defeating the conforming lifecycle requires deliberate preservation or extraction of decrypt-capable material, consistent with [Modified Clients](#modified-clients-the-honesty-assumption).

Availability is a property of the swarm and of the ledger's state view, not of any service. A participant needs the ciphertext, the header sidecar, its own credential, and a fresh view of consensus state to decrypt, and needs no party's cooperation to obtain any of them; this is what removes the liveness concern the wrapped-key posture carried ([Provisioning Liveness](#provisioning-liveness-centralization-and-participation)).

Multi-device follows without special handling of credentials: an identity's reading devices share its persistent credential and each authorizes independently against the same entitlement. The holder seed stays on the identity's root devices; every other device acts under its own device key, which the identity admits and revokes in consensus state, and receives the envelope secrets, never the seed, when its role reads. Revocation takes effect at the device's next state view, by the same conforming-client purge that ends an interval.

### Variance Belongs to the Entitlement, Not the Content

Forensic traceability requires that the plaintext a user renders be recipient-specific. The naive construction varies pieces within the distributed asset, which fragments the swarm and destroys the distribution economics the protocol depends on.

The protocol instead binds variance to the **entitlement layer**, where 1:1 identity already exists by construction, and applies it as an **overlay at decryption time**:

* The **swarm object remains complete and canonical.** One ciphertext, one infohash, byte-identical for every seeder, decrypting to the canonical plaintext committed in [Cryptographic Primitives](#cryptographic-primitives). Zero fragmentation, distribution invariants intact, and unmodified BitTorrent clients continue to retrieve a complete asset.
* The **variant seed belongs to the entitlement**, committed to by the variant root in the entitlement record and delivered as confidential auxiliary material in the holder's credential envelope. It is a value from which the holder's permutation is generated, not a copy of any content, and it is small enough that the entitlement carries its commitment rather than the swarm.
* At decryption the holder applies the permutation over the canonical plaintext, producing a rendering that is fingerprinted to them.

**The fingerprint is a cooperative mechanism, not a forensic guarantee.** Because the swarm object is complete, a holder whose client omits the overlay can render canonical, unfingerprinted plaintext. Attribution therefore holds against unmodified clients and casual redistribution — which is the great majority of leakage — and does not hold against a deliberately modified client. This is the same posture as [Core Philosophy](#core-philosophy)'s non-DRM stance and [Modified Clients](#modified-clients-the-honesty-assumption)'s honesty assumption, and it is stated here rather than overclaimed: the mechanism raises the cost and traceability of casual violation, and does not attempt to make violation impossible.

### The Variant Seed Lives in the Entitlement

A seed is a value of a couple of hundred bits from which a fingerprint codeword is generated, not an enumeration of replacement content. It carries no fraction of the asset and its size is independent of asset size — which means **the entitlement can carry it, and no swarm object is required at all.** Everything that would otherwise attach to distributing a per-entitlement object disappears with it: no separate infohash, no colocation obligation, no availability problem, no regeneration path for a holder whose peers stopped serving, and no pruning signal for a superseded object.

The binding that has to hold is confidentiality, and it survives the move intact:

* **The entitlement carries a commitment, never the seed.** An on-chain token is world-readable, so a seed published in it would be readable by everyone. The entitlement record holds the variant root ([Cryptographic Primitives](#cryptographic-primitives)); the seed itself is delivered in the confidential payload of the same credential envelope, at mint and at each transfer.
* **Readable seeds would be a framing weapon.** If any party could read another holder's seed, they could render a copy carrying someone else's fingerprint. Unilateral framing is a far worse failure than collusion, which is why the split between an on-chain commitment and a confidentially delivered seed is mandatory rather than an optimization.
* **The commitment is what makes attribution checkable.** A traced rendering is matched against the variant root committed for that entitlement, and the ledger's transfer history says who held the entitlement then. Attribution resolves to an identity *and* an ownership interval, with no party retaining a private record to be trusted or subpoenaed.

**Separate distribution availability ceases to be a question.** There is no per-entitlement seed object to seed, lose, or reconstruct. Confidential seed delivery rides the credential envelope and inherits no availability assumption beyond settlement itself; the authoring mechanism remains unresolved ([Variant Seed Authorship](#variant-seed-authorship)).

**Multi-device resolves without special handling.** One identity, *n* reading devices: all *n* share the same credential envelope and therefore the same ownership-interval seed, with no per-device ceremony at the entitlement layer; device admission is a property of the identity, not of any entitlement.

### Re-Minting on Transfer

When an entitlement transfers, the protocol requires the variant seed to be **re-issued** for the new holder. Without this, a buyer's rendering would carry the seller's fingerprint and attribution would point at the wrong party. This is required V2 behavior, not a claim that the confidential seed-authoring mechanism has been selected.

Because the seed lives in the entitlement rather than in the swarm, the public result of re-issuance is a field update at settlement: the entitlement's variant root is replaced, and the transfer envelope delivers the confidential seed it commits to. There is no new object to distribute, no old object to retire, and no seeder to notify. Transfers are already ordered by the **transfer sequencer**, so that sequencer orders the entitlement transition and associated commitment replacement. It does not author or learn the confidential seed and has no role in authorization.

The contract can require and commit the state transition, but a public contract cannot by itself privately author and deliver the seed. The unresolved authoring mechanism must therefore make seed production non-discretionary without handing the publisher or another single party a veto. Until that mechanism is selected, the contract does not foreclose selective withholding merely by requiring a replacement commitment; this is the open problem recorded in [Variant Seed Authorship](#variant-seed-authorship).

The seller retaining their previous seed and any rendering made under it is expected and harmless — it is the accepted plaintext-non-revocability case ([Core Philosophy](#core-philosophy)), and their fingerprint correctly identifies them.

*Note on double-sale:* re-issuance is not itself a double-sale prevention mechanism — entitlements are single-owner bearer assets and the ledger already prevents double-sale by construction. What re-issuance adds is **forensic precision**: each seed is bound to one ownership interval and the ledger's transfer history is what dates it, so leaked content is attributable not merely to an identity but to *when* that identity held the right.

### Collusion Resistance

Two or more holders can diff their renderings to locate variant positions and splice a copy whose fingerprint matches neither. Variant seeds are therefore assigned using a **collusion-resistant fingerprinting code** (Tardos or equivalent), which provides provable tracing up to a chosen collusion size *c* with a false-accusation probability bounded by a security parameter.

The cost is that the codeword length grows with *c* and with the accused-population size, so *c* is an explicit economic parameter — chosen against asset value and expected adversary resources — rather than a fixed constant.

### Future Optimization: Partial Encryption

A participant currently stores an asset twice — the ciphertext, retained to keep seeding ([Continued Network Support](#phase-3-secondary-transfer-and-authorization-expiry)), and the plaintext, retained to use. Roughly 2x local storage per asset held.

Partial encryption would collapse that. If a small set of pieces chosen to be *structurally essential* — headers, codec-critical frames, module entry points — were withheld from the swarm, the remainder could be distributed as plaintext. The asset stays useless without the withheld piece, the main swarm becomes plaintext with the attendant gains in deduplication and legacy client compatibility, and the key-handling problem shrinks from bulk encryption to small-object delivery.

This is a **storage and distribution optimization only**. It is unrelated to the per-entitlement variance of [Variance Belongs to the Entitlement](#variance-belongs-to-the-entitlement-not-the-content), which operates as an overlay over a complete swarm object and does not withhold anything; the two mechanisms should not be conflated, and adopting one does not imply the other.

**Recorded as a future optimization and explicitly not a launch capability.** The reason is commercial rather than technical: a distribution model whose plaintext is largely freely available is not a proposition sophisticated rights holders will entertain before the fully-encrypted model has been proven in production. The technical question — how degraded the content actually is with a strategically chosen fraction withheld — is also asymmetric across content classes, since a package missing its entry point is inert while a film missing 1% of its pieces may remain watchable.

## Open Problems

Protocol properties that are unresolved by design rather than by omission, recorded so they are neither forgotten nor discovered late. Undetermined *authorship and design decisions* are held separately, in the To-Do list of `docs/workplans/current/ChainTorrent MVP.md`.

### Provisioning Liveness, Centralization, and Participation

**Resolved for the read path by the native-credential suite.** The concern this section recorded — a standing provisioning layer on the read path, with its liveness, centralization, trust, and load costs — arose from re-establishing authorization after every sale through a party able to establish it. The credential KEM removes that party: a sale delivers the buyer's credential from the seller's own, the contract verifies rather than decides, and reading is local against a state view ([MVP Posture](#mvp-posture-native-credentials)). No committee holds any content secret, no service fields read-time traffic, and the former tradeoff between window width and the transfer boundary no longer exists, because there are no windows. The only liveness dependency that remains is the First Finder's during escrow, and it is bounded to new zero-price grants: existing holders read and transfer without it, and a claim never needs it ([Escrow Key Lineage Across a Claim](#escrow-key-lineage-across-a-claim)).

The obstacle this section previously stated — that a public ledger cannot hold a secret only an entitled holder can unwrap — was real and remains true. The resolution does not contradict it: the ledger holds no secret. It holds ciphertexts addressed to registered keys and proofs that they are well-formed, and the secret work happens once, at mint or sale, by a party that is present and non-deniable by construction.

What survives of this section is the participation obligation below, now confined to retention.

#### Participation as an Obligation of Identity

Seeder compensation requires a participant set that cannot be captured cheaply ([Seeder Compensation](#seeder-compensation)), and retention of swarm ciphertext is the storage obligation the protocol attaches to identity. The construction the protocol targets:

* **Participation is an obligation of holding an identity**, not a voluntary contribution offered by whoever chooses to run infrastructure.
* **Participation is scoped per identity, never per device.** Sybil resistance is then a property of the protocol rather than an overlay some other layer must manage, because running more machines buys no additional weight. Deliberate fragmentation of one operator into many identities is a distinct problem and is not answered here.
* **Retention is assigned in three tiers**: *obligated*, assigned by random rotation over the participant set and held until reassignment; *self-interested*, what a participant holds because they use it, governed by nothing but their own choice; and *discretionary*, capacity offered above the floor, which is the tier compensation attaches to.
* **The obligation is attested and audited, not enforced.** A participant delegating storage to software the protocol does not control cannot be compelled, so non-compliance is made visible and costly rather than impossible — the same posture as [Modified Clients](#modified-clients-the-honesty-assumption). The audit is a challenge to produce a BLAKE3/Bao authentication path for a random chunk of an assigned object, which introduces no cryptography the protocol does not already carry.

Sybil resistance is deliberately absent while nothing is at risk, and the absence is harmless: capturing retention assignments over $0.00 entitlements to permissively-licensed content obtains nothing. Value arrives with network size and with parties explicitly publishing priced assets, which is the same point at which stake becomes available to condition assignment on.

Rotation period, replication factor, retention floor, and audit challenge frequency have no values. They are held in the To-Do list rather than here, and none of them is enforceable before the stake layer exists.

### Recipient-Bound Decryption Credentials

**Resolved by the credential KEM, with three recorded residuals.** The desired end state this section described — a cryptographically distinct decryption capability per holder that decrypts one canonical cyphertext without exposing a common decryption-equivalent secret, bound to the exact entitlement, interval, and identity, resolved from current state, with First Finder escrow, no publisher on the read path, and non-selective availability — is what [Credential KEM](#credential-kem-deployment-encryption-adapter) and [Credential Delivery](#cryptographic-primitives) deliver, at the granularity of the ownership interval rather than of a window within it, because windows no longer exist. The constraints that made the target difficult are all preserved: one byte-identical and randomly seekable swarm object per deployment, unbounded issuance, re-binding on transfer by seller-side rerandomization, contemporaneous entitlement-state and wallet-control checks by the client, First Finder escrow, active client-side destruction, no publisher on the read path, and availability that depends on no party.

The residuals are stated so that this section is not read as more than it is. First, the capability is distinct per *interval*, and a modified prior holder's retained credential still decrypts; what the protocol gains is that the open interval's credential is named in the settlement record and every other is protocol-invalid, not that stale material stops working. Second, every credential recovers the same encapsulated values, so the unwrapped piece-group keys are a common object any holder can export; their granularity bounds the damage, and their leak is unattributable ([Entitlement Auditability](#entitlement-auditability-vs-content-key-traceability)). Third, attribution of a leaked credential reaches the entitlement and not the interval, and a leaker can rerandomize before leaking; the interval-level binding this section asked for is optional tracing that a rerandomizable credential cannot supply. Cryptographic loss at transfer against a modified client is not available under one canonical cyphertext without trusted hardware, and the protocol records that as a preference it does not meet rather than as an open problem.

### Common Piece-Group Keys

Every credential recovers the same encapsulated value from a capsule, so the piece-group keys the payload cipher consumes are common to every holder, and a holder can export them as a payload-proportional key file that unlocks the swarm copy for anyone ([Entitlement Auditability](#entitlement-auditability-vs-content-key-traceability)). This is a consequence of one canonical ciphertext under a symmetric cipher — the keystream is the same for everyone — rather than of the credential construction, and it is accepted as a residual: the deployment-wide content key no longer exists, no party ever holds or delivers the common values, the export is bounded per group and per deployment, and the compact object a holder actually receives is distinct and, under the entitlement scope, names its entitlement.

Removing the compact common object entirely is not available with known practical methods. It would require the final symmetric step itself to be keyed by the holder — a multi-key pseudorandom function in which distinct holder keys compute one keystream, so that the only common object is the keystream, which is the plaintext's size — and the known constructions are lattice-based, need a master issuer, and are orders of magnitude slower than AES per block. The seam is preserved for that future: the credential KEM's decapsulation and the payload-cipher adapter compose through the suite, and a future suite may fuse them behind the same consumption interface without touching commitments, entitlements, or delivery. Until then the piece-group size is the lever, chosen from measurement against how much one leaked key unlocks.

### Dependency Graph Privacy

Every entitlement is an on-chain record binding an identity to an asset, which in aggregate publishes a permanent, correlatable map of which software each participant runs at which version. For public content that is a stated property rather than a problem — see Public Distribution Implies Public Consumption in [Core Philosophy](#core-philosophy) — and what remains open is only what the property does not cover.

* **The record is a live feed, not a snapshot.** A published inventory is a disclosure its author controls and dates. This is continuous, and it reveals adoption timing, patch latency, and evaluation activity that a static inventory does not. Per-attempt state reads sharpen this rather than softening it: nothing beyond the entitlement is published at issuance, but the client's state reads form a traffic stream that shows whichever node serves them the same associations in real time, with decryption timing a ledger record would never carry, and each read reaches the nodes the client configures and no others.
* **Individuals are not the party the property argues about.** The free-rider case concerns an organization capturing the benefit of public content while concealing the risk inherited with it. A solo participant's install history is a behavioral profile, pseudonymity weakens it only partially because a dependency set approaches a fingerprint, and one deanonymizing transaction retroactively unmasks the history behind it.
* **Private content is a different case entirely.** Ingest source eligibility ([Ingest Source Eligibility](#ingest-source-eligibility)) currently confines adapters to archives whose content is already free to use, so every asset the protocol carries is public and the property covers it. An adapter pointed at a private or internal registry breaks that, because neither the asset list nor its consumption was ever public. That crossing must be deliberate rather than emergent: an adapter for private content needs a consumption-privacy answer before it ships, on the same reasoning that holds a licensed-catalog adapter behind [The Post-Claim Pricing Paradox](#the-post-claim-pricing-paradox).

Directions to evaluate for the cases above, none adopted: per-asset ephemeral wallets; blinded or private-information-retrieval authorization requests; batching and mixing to break the link between requester and asset; off-chain entitlement proofs that settle on-chain only in aggregate; zero-knowledge proof of entitlement that reveals neither identity nor asset. The tension with [Entitlement Auditability](#entitlement-auditability-vs-content-key-traceability)'s auditability is direct — entitlement accounting wants a legible record, and privacy wants an illegible one — and any resolution has to state which property it is sacrificing.

### Variant Seed Authorship

[Variance Belongs to the Entitlement](#variance-belongs-to-the-entitlement-not-the-content) establishes that variance is an overlay seed bound to the entitlement. It does not establish who authors the seed, from what, and under what constraints. Open.

The constraint that shapes the answer is **First Finder escrow**: the publisher is absent by definition in the escrow case, so any scheme requiring the publisher to be online at issuance or transfer fails outright. Whatever is chosen must work identically for an escrowed asset whose maintainer has never appeared, and must not confer publisher rights on the bootstrapper ([Invariant Requirements](#overview-and-invariant-requirements)). A First Finder that authors seeds must hold the authoring material under the same escrow rules as the master scalar and erase it after handover; retaining it would leave a non-owner with permanent framing capability over an asset they do not own.

A second constraint is content validity. **A seed can select among alternatives; it cannot author them.** If both the varied positions and their replacement values are derived from the seed alone, the resulting rendering is a corruption — for a video a glitched frame, for a tarball a syntax error and a package that does not install. Producing renderings that are *semantically valid* requires alternatives authored with knowledge of the content, at which point the seed selects among them rather than generating them. Solutions to this exist and are well understood in forensic watermarking; choosing and specifying one is beyond present scope, and implementation sits behind the variance interface, so the decision can be made later without disturbing the layers around it.

Whoever holds the authoring material can generate any holder's seed and therefore frame any holder. Siting that material with the credential author — the issuer at mint and the seller at sale, who already post the envelope the seed would ride in — is the cheapest available answer but not the only one, and it would let a seller frame its own buyer, which the framing analysis above forbids.

Until this problem is resolved, [Re-Minting on Transfer](#re-minting-on-transfer) states the required ownership-interval transition rather than an implemented generation procedure. The transfer contract can order the transition and store or verify a replacement commitment, but it cannot privately author the committed seed on a public ledger. Delivery is solved, since the seed rides the credential envelope; a V2 implementation therefore requires a confidential authoring construction that works for First Finder assets, makes valid issuance non-selective, and places no party on the read path.

### Seeder Compensation

**This is the participation obligation of [Provisioning Liveness](#provisioning-liveness-centralization-and-participation) in a different hat, and it is recorded separately only because it is encountered separately.** The swarm distribution the protocol depends on presumes that seeding is rewarded, and no compensation mechanism is specified. Without one, storage contribution is voluntary, free-riding is rational, and aggregate resilience decays toward the level sustained by altruism alone.

The obligation model of [Provisioning Liveness](#provisioning-liveness-centralization-and-participation) is what replaces altruism with assignment, and it cannot be enforced without a Sybil-resistant participant set — the same prerequisite, over the same population, as retention assignment. Compensation is what makes the discretionary tier worth offering; the obligated tier does not depend on it. A proposal addressing one of the two and not the other should be treated as incomplete rather than as partial progress.

This is a protocol-level gap rather than an implementation detail, because the incentive structure determines whether the distribution model works at population scale at all. Candidate mechanisms are recorded in the To-Do list rather than here, per this section's scope.

### The Post-Claim Pricing Paradox

When a First Finder bootstraps an unlisted object, the Escrow contract unconditionally mints $0.00 entitlements to grow the swarm. When the Web2 owner arrives to claim the asset, they acquire control over future entitlement issuance.

If the owner wishes to price the asset >$0.00, they cannot retroactively charge or deny existing users who hold the escrow-issued entitlements. The existing entitlements are permanently grandfathered. If the owner imposes a new price on the claimed object, secondary market sellers of the free entitlements can undercut the publisher, establishing a market ceiling.

**Scope of the paradox.** It arises only where the ingested content was not already free to use. Under the ingest source eligibility rule ([Ingest Source Eligibility](#ingest-source-eligibility)), the arriving maintainer of a permissively-licensed asset was denied no revenue by the escrow-issued entitlements, so the question below is deferred rather than encountered. That rule is what permits the escrow model to be exercised in production while this section remains open.

**Open Question (Post-MVP):** How can the protocol abstract early, unclaimed downloads into fair economic value for the claimant once they arrive? If a publisher benefits from waiting for an object to become popular via free entitlements before claiming and versioning it, there must be an equitable mechanism to reward the claimant without enabling retrospective deniability. If the publisher can retroactively impose an arbitrary price and force it on existing entitlement holders, that rewards the publisher for delaying until the object is popular and/or enables deniability for the publisher. If there is no fair economic value, the First Finder and early users benefit from free-riding against an object they don't own or have rights to.

### Authorization Height Agreement

**Resolved by the attempt rule.** The problem this section recorded — several provisioning nodes having to agree on one settlement reference for a threshold response — no longer arises, because no party evaluates authorization on a holder's behalf. The conforming client reads its own state view at the deployment's declared tier or above, no older than its freshness bound `τ_soft` ([Per-Attempt Authorization](#phase-2-consumption-and-decryption)), and the reference of that view binds nothing but the client's own attempt. Delivery proofs bind a settlement's own reference by being verified inside it. What remains is the choice of `τ_soft` and `τ_wallet` per deployment, which is a parameter selection held in the To-Do list rather than an open design question.

