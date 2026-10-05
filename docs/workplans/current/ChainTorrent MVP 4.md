`[ ]`    // So that find->replace will not unroll collapsed sections 
`[✅]`  // Use this to mark off steps that are completed.  

# **ChainTorrent MVP**

## Problem Statement

JavaScript dependency distribution runs through a single centralized registry that is an availability chokepoint, a mutable source of truth, and an intermediary between the people who publish packages and the people who consume them. ChainTorrent's MVP replaces that path with a content-addressed swarm, a canonical on-chain identity registry, and transferable access entitlements, so that the distribution and identity model is proven with monetization out of bounds.

**Adoption runs individual developer first, and everything else follows from that.** A developer at a terminal gains cross-project reuse of whatever their machine has already fetched, swarm retrieval on popular packages, and installs that survive a registry outage. Build platforms adopt next and for their own reasons: a service running the same few thousand popular installs continuously, on always-on machines with real bandwidth, holds the ideal cache as a by-product of its own economics, and seeding that cache costs almost nothing once it exists. That makes them natural superseeders rather than participants anyone has to recruit, and it is why no privileged bootstrap host is part of this plan. CI and enterprise adoption arrive as a consequence of that sequence rather than as targets to be won early.

The sequence matters to this workplan because it determines what the MVP measures. Latency is judged against interactive local installs rather than pipeline tolerance, since a developer notices a doubled install where a pipeline does not. Cache reuse is the headline adoption metric, because it is the benefit the adopting population actually experiences. Registry-outage survival remains true throughout and is simply not the lead, since it matters most to the population that adopts last.

The design is specified in `docs/research/cryptography.md` and bounded in `docs/research/MVP Scope.md`. Undetermined decisions are held in the To-Do list below.

The cryptographic construction is specified; its research record is `docs/research/cryptography-research-notebook.md`, and its statement is `docs/research/cryptography-critical-path.md`. A validation harness on both pairing curves produces the sizes, timings, and gas against the configured piece-group size and curve defaults, and nothing waits on it: every node reads the piece-group size from the deployment record and depends only on the code it calls.

## Objectives

Deliver an install path for JavaScript dependencies that survives registry outages, serves popular packages from a peer swarm, and exercises the identity and entitlement pipeline end to end.

Prove distribution and identity. Willingness to pay and seeder compensation are out of bounds and are held in the To-Do list; the transaction flow is proven at a nominal price on assets the project publishes, as a test instrument rather than a monetization step.

Every undetermined decision is carried in the To-Do list, which every workplan that stems from this one inherits.

## Expected Outcome

A developer installs dependencies and receives a working `node_modules` assembled from a local content-addressed store, a peer swarm, or the ingest source as fallback, with canonical identity resolved on-chain and an access entitlement minted to their wallet.

# Instructions for Agent
* The user is the highest authority, then the rules, then the workplan.
* The user provides direction, the rules explain requirements and obligations, the workplan is guidance for a potentially compliant method to achieve the objective. 
* Never obey the workplan if it contradicts the user or rules. 
* Read `docs/agents/index.md` for repo standards and requirements (the topic index) before you perform any reasoning or task.
* Read `.cursor/commands/*.prompt.md` for task-specific direction. 

# Work Breakdown Structure

Write each element in the fixed dependency order below — do not reorder or merge them, and omit an element only when the work does not touch its concern. Before writing an element, obey the topics that govern it — its `Conforms to:` list in `docs/agents/workplan-structure.md` and the routing matrix in `docs/agents/index.md` — but do not print those citations into the plan; they are authoring guidance, not node content. Name groupings by their dependency role.

* **Foundation**

* **Cryptographic validation harness**

## Delivery proof and Solidity verifier

# To-Do List

Every held item, grouped by **what releases it** rather than by topic.

**Open questions** carry what was found, where it lives, and what resolving it would take. **Boundary decisions** specified in `docs/research/MVP Scope.md` or `docs/research/cryptography.md` appear as a single line under their gate, pointing at where they are specified.

## Releases with the stake and token layer

These bottom out in the same prerequisite: identity that carries stake, over a participant set that cannot be captured cheaply. They are resolved together or not at all.

### Seeder compensation mechanism

**What was found.** The swarm distribution the protocol depends on presumes that seeding is rewarded, and no mechanism is specified. Without one, contribution is voluntary, free-riding is rational, and aggregate resilience decays toward whatever altruism sustains.

**This is one problem with retention assignment, not two adjacent ones.** Both bottom out in Sybil-resistant stake over the same participant set: a design answering who may be rewarded for storing is most of a design for who may be assigned to store, and the reverse holds as well. A proposal addressing one and not the other is incomplete rather than partial, and the two are resolved together or not at all.

The candidate mechanism is a reciprocal token pair in which one participant's upload token is another participant's download token, with a participant's ratio serving as the translation between them. This is out of MVP bounds because it depends on the token economics deferred alongside monetization, but it is a protocol-level gap rather than an implementation detail: the incentive structure determines whether the distribution model works at population scale at all.

**Where.** [`docs/research/cryptography.md`, Provisioning Liveness](../../research/cryptography.md#provisioning-liveness-centralization-and-participation) and [Seeder Compensation](../../research/cryptography.md#seeder-compensation); [`docs/research/MVP Scope.md`, Seeder Compensation](../../research/MVP%20Scope.md#seeder-compensation).

**What resolving it would take.** Specifying the token pair and the ratio translation, then deciding whether the mechanism is enforced or emergent. The specification assigns retention by obligation and audits it, so compensation is what makes the discretionary tier worth offering, and the obligated tier does not depend on it. Which tier a token accounting system drives is a decision, not a side effect.

### Retention parameters

**What was found.** Retention of swarm ciphertext and header sidecars is the participation obligation attached to every identity, assigned by random rotation over the participant set and audited on one surface. Variant seeds are carried by the entitlement and are neither stored nor served, so retention covers ciphertext and sidecars only. Availability is a constructed replication factor, which makes a number necessary.

The coupled values below are undetermined and none can be chosen independently of the others. The **rotation period** trades capture resistance against reassignment churn, since an assignment that rotates faster is harder to game and more expensive to keep synchronized. The **replication factor** sets how many identities each object is assigned to, and is what converts availability from an emergent property into a constructed one. The **retention floor** is the ciphertext storage each identity carries to remain in good standing. The **audit challenge frequency** determines how quickly a non-compliant holder is detected, and costs bandwidth on every participant to raise.

**Where.** [`docs/research/cryptography.md`, Provisioning Liveness](../../research/cryptography.md#provisioning-liveness-centralization-and-participation), the participation obligation; [`docs/research/MVP Scope.md`, Retention Obligation](../../research/MVP%20Scope.md#retention-obligation) and [Seed Hosting and the Ciphertext Store](../../research/MVP%20Scope.md#seed-hosting-and-the-ciphertext-store).

**What resolving it would take.** Choosing them jointly against a stated availability target and a stated storage cost per participant. None is enforceable without the identity and stake layer, so the MVP can select values and does not exercise them.

### Boundary decisions under this gate

* **Paid monetization** — general pricing above nominal, and the fiat and token rails it requires. Deferred in [`docs/research/MVP Scope.md`, Paid Monetization](../../research/MVP%20Scope.md#paid-monetization). Note that [Transaction Flow Proof](../../research/MVP%20Scope.md#transaction-flow-proof) does not release this: that path is a test instrument on assets the project publishes itself, touching no third-party publisher's revenue.
* **Enforcement of retention assignment and obligation** — the shape ships in the MVP and nothing is enforced. Specified in [`docs/research/MVP Scope.md`, Retention Obligation](../../research/MVP%20Scope.md#retention-obligation).
* **The first-party component class** — own chain, tokens, wallet, keystore, and seeder client, each behind the adapter whose MVP implementation is third-party. Specified in [`docs/research/MVP Scope.md`, Adapter Composition and Capability Declaration](../../research/MVP%20Scope.md#adapter-composition-and-capability-declaration).

## Releases when a content class needs it

Nothing here is blocked by an unanswered question. Each is blocked by the absence of content whose value or shape justifies building it, and the MVP's content class supplies neither.

### Collusion parameter selection

**What was found.** Variant seeds are assigned using a collusion-resistant fingerprinting code providing provable tracing up to a chosen collusion size *c*. Codeword length grows with *c* and with the accused-population size, so *c* is an economic parameter with no principled default, and its cost curve is unmeasured.

Because the seed is a value the entitlement carries rather than an enumeration of replacement content, the cost of *c* is carried in codeword length rather than in any stored or distributed object, and is independent of asset size.

The specification states the related properties: the MVP ships with no attribution; attribution is evidence rather than enforcement, since identifying a leaker does not recall content, making the response to a traced leak a governance and legal question; and the fingerprint is cooperative rather than forensic, since the swarm object is complete and a modified client can render canonical plaintext carrying no fingerprint at all.

**Where.** [`docs/research/cryptography.md`, Collusion Resistance](../../research/cryptography.md#collusion-resistance).

**What resolving it would take.** Modelling *c* against asset value and expected adversary resources, and measuring the codeword-length cost curve rather than assuming it. Neither is needed before per-entitlement variance ships.

### Boundary decisions under this gate

* **Per-entitlement variance and forensic attribution** — meaningless while content is permissively licensed. Deferred in [`docs/research/MVP Scope.md`, Per-Entitlement Variance and Forensic Attribution](../../research/MVP%20Scope.md#per-entitlement-variance-and-forensic-attribution).
* **Arbitrary media and streaming engine** — a different client pipeline, and the content class that exercises per-attempt authorization continuously. Deferred in [`docs/research/MVP Scope.md`, Arbitrary Media and Streaming Engine](../../research/MVP%20Scope.md#arbitrary-media-and-streaming-engine).
* **Content flagging and the metadata backlink layer** — the construction is the cost, not any one advisory type. Deferred in [`docs/research/MVP Scope.md`, Content Flagging and Deprecation Surface](../../research/MVP%20Scope.md#content-flagging-and-deprecation-surface).
* **Partial encryption** — held for a commercial reason rather than a technical one. Specified in [`docs/research/cryptography.md`, Partial Encryption](../../research/cryptography.md#future-optimization-partial-encryption).
* **Composable container objects** — reference nesting with per-member roots in the manner of BitTorrent v2, so that seeding a collection seeds its members. Deferred in [`docs/research/MVP Scope.md`, Composable Container Objects](../../research/MVP%20Scope.md#composable-container-objects).

## Releases when a design question is answered

Open questions. Each blocks work that cannot be authored around it.

### Variant seed authorship

**What was found.** Variance is an overlay seed bound to the entitlement, but who authors the seed, from what material, and under what constraints is undetermined.

Constraints shape any answer. First Finder escrow means the publisher is absent by definition, so no scheme requiring the publisher online at issuance or transfer can work at all; and a First Finder that authors seeds must escrow and discard the authoring material exactly as it does the content key, or a non-owner retains permanent framing capability over an asset they do not own. Separately, a seed can select among alternatives but cannot author them — if both the varied positions and their replacement values derive from the seed alone, the rendering is a corruption rather than a variant: a glitched frame, or a tarball that no longer installs. Semantically valid renderings require alternatives authored with knowledge of the content.

Whoever holds the authoring material can generate any holder's seed and therefore frame any holder. Siting that material with the credential author — the issuer at mint and the seller at sale, who already post the envelope the seed would ride in — is the cheapest available answer, but it would let a seller frame its buyer, which the framing rule forbids. The seed rides the credential envelope.

**Where.** [`docs/research/cryptography.md`, Variance Belongs to the Entitlement](../../research/cryptography.md#variance-belongs-to-the-entitlement-not-the-content), [The Variant Seed Lives in the Entitlement](../../research/cryptography.md#the-variant-seed-lives-in-the-entitlement), and [Variant Seed Authorship](../../research/cryptography.md#variant-seed-authorship); [`docs/research/MVP Scope.md`, Per-Entitlement Variance and Forensic Attribution](../../research/MVP%20Scope.md#per-entitlement-variance-and-forensic-attribution).

**What resolving it would take.** Choosing an authorship mechanism from forensic watermarking and specifying it behind the variance interface, where the choice disturbs no layer around it.

### The post-claim pricing paradox

**What was found.** An escrow contract mints free entitlements to grow the swarm before an owner arrives. When the owner claims the asset they acquire control over future issuance but cannot retroactively charge or deny existing holders, whose entitlements are permanently grandfathered, and secondary sellers of those free entitlements can undercut any price the publisher sets.

The paradox arises only where ingested content is not free to use, which the ingest eligibility rule excludes, so the escrow model is exercised in production without encountering it.

**Where.** [`docs/research/cryptography.md`, The Post-Claim Pricing Paradox](../../research/cryptography.md#the-post-claim-pricing-paradox) and [Ingest Source Eligibility](../../research/cryptography.md#ingest-source-eligibility).

**What resolving it would take.** A mechanism that converts early unclaimed distribution into fair economic value for the claimant without enabling retroactive pricing. Both failure modes must be excluded: a publisher who profits by waiting for an asset to become popular before claiming it, and a First Finder and early users who free-ride on an asset they never owned.

### Consumption privacy for private content

**What was found.** For public content, who holds an entitlement being publicly legible is a stated property rather than a defect. That property does not cover these cases: the record is a live feed rather than a snapshot its subject can pace, individuals are not the free-riding party the property argues about, and private content was never public in either its asset list or its consumption.

**Where.** [`docs/research/cryptography.md`, Dependency Graph Privacy](../../research/cryptography.md#dependency-graph-privacy) and Public Distribution Implies Public Consumption in [Core Philosophy](../../research/cryptography.md#core-philosophy).

**What resolving it would take.** A consumption-privacy construction for the private case — per-asset ephemeral wallets, blinded or private-information-retrieval authorization, batching and mixing, off-chain proofs settling in aggregate, or zero-knowledge proof of entitlement. Any resolution has to state which side of the auditability-versus-privacy tension it sacrifices. This gates any ingest adapter pointed at a private or internal registry.

### Scope of the authorization invariants

**What was found.** The Cryptographic Authorization Invariants are written as unconditional statements, but they bind the settlement contract, credential authors, and conforming clients only. They cannot bind arbitrary software on a user-controlled device, and the protocol deliberately commits to plaintext running in any compatible software, which guarantees such software exists.

That scope is stated under Modified Clients as a security consideration rather than among the invariants where they are declared, so a reader taking the invariant list on its own would overread its guarantees.

**Where.** [`docs/research/cryptography.md`, Cryptographic Authorization Invariants](../../research/cryptography.md#cryptographic-authorization-invariants); the scope statement under [Modified Clients](../../research/cryptography.md#modified-clients-the-honesty-assumption).

**What resolving it would take.** Deciding whether the invariant list carries its own scope sentence or whether the Modified Clients statement suffices. This is a question of where the statement belongs, not whether it is true.

### Adapter registry governance

**What was found.** Adapters are resolved from a governance-controlled on-chain registry, immutable once bound: the identity adapters, the signature adapters, the attestation verifier per ingest source, and the entitlement token form; the source-key table the npm attestation verifier reads is updated under the same governance. The MVP governs both with a single project-held key, held under the same custody discipline as issuance material and recorded as scaffolding, so the registry's binding authority and the attestation check are only as strong as that key's custody.

**Where.** [`docs/research/cryptography.md`, Population Strategy](../../research/cryptography.md#population-strategy-breakdown) and [Ingest and Deterministic Normalization](../../research/cryptography.md#phase-1-encryption-minting-and-seeding); [`docs/research/MVP Scope.md`, Entitlement Ledger and Escrow Contract](../../research/MVP%20Scope.md#entitlement-ledger-and-escrow-contract); the product requirements' Resolved Positions.

**What resolving it would take.** A governance model that moves the key off a single holder, a multisig or a stake-conditioned process, with a migration of the registry's and source-key table's authority that no single party can withhold, and a recorded trigger for when the MVP's scaffolding is replaced.

### Entitlement migration across chain adapters

**What was found.** The chain sits behind an adapter and the protocol intends its own chain eventually, but an adapter swap moves nothing: entitlements, interval state, envelope digests, parameter-set liveness, deployment records, and identity bindings live in one chain's contract state, and a further chain, whether Ethereum mainnet, another L2, or the project's own, starts empty. Every invariant the protocol makes about an entitlement, asset binding, irrevocability, transferability, and the delivery chain that authenticates its credential history, is a statement about one ledger. How an entitlement and its interval history become valid on another ledger, without a party acquiring the discretion over the move that the No Party May Selectively Withhold invariant forbids, is unspecified.

**Where.** [`docs/research/MVP Scope.md`, Adapter Composition and Capability Declaration](../../research/MVP%20Scope.md#adapter-composition-and-capability-declaration) and [Entitlement Ledger and Escrow Contract](../../research/MVP%20Scope.md#entitlement-ledger-and-escrow-contract); [`docs/research/cryptography.md`, Architectural Invariants](../../research/cryptography.md#overview-and-invariant-requirements) and [Credential Delivery](../../research/cryptography.md#cryptographic-primitives); the launch network decision recorded in [`docs/project-planning/product-requirements.md`](../../project-planning/product-requirements.md).

**What resolving it would take.** A migration mechanism with the same non-deniability the transfer path has: a snapshot, claim, or bridge construction under which a holder proves its entitlement and interval on the source ledger and obtains the equivalent record on the target ledger with no party able to withhold it, the source record retired or frozen so scarcity is not doubled, the envelope history carried or re-anchored so the next sale's proof has a base case, and parameter-set liveness reproduced so existing credentials keep decrypting. The MVP lives on one chain and needs none of it; any further chain adapter needs it before carrying live entitlements, and it decides whether the first-party chain is a migration or a fresh start.

## Releases on a deliberate policy line

Not blocked by engineering. Blocked by a line the project draws on purpose, so that crossing it is a decision rather than a drift.

### Ingest adapter eligibility

**What was found.** The adapter interface is source-independent, but the *selection* of sources is not a free choice. Exposure concentrates in the acquisition path rather than in distribution, escrow is deferred ownership identification rather than an acquisition licence, and a free archive is what makes the post-claim pricing paradox inert. Adapters therefore target archives whose content is free to use.

Crossings are held behind separate answers: an adapter aimed at licensed content waits on the post-claim pricing paradox, and an adapter aimed at private or internal content waits on consumption privacy.

**Where.** [`docs/research/cryptography.md`, Ingest Source Eligibility](../../research/cryptography.md#ingest-source-eligibility).

**What resolving it would take.** Answering the gating item for the crossing in question, then making the adapter selection a recorded policy decision rather than an engineering convenience.

### Distribution and client license

**What was found.** The software must permit forking and modification while making a non-conforming client a license violation, and each asset record must carry the rights-holder's content terms, with the project's own packages obligating an entitlement for use and a license per copy sold or bundled. A conformance clause makes the software license source-available rather than OSI-open, and it binds forks of the code, not a clean-room client written from the specification.

**Where.** [`docs/research/MVP Scope.md`, Distribution and Client License](../../research/MVP%20Scope.md#distribution-and-client-license); [`docs/research/MVP Application Requirements.md`, LI-01](../../research/MVP%20Application%20Requirements.md#licensing); [`docs/research/cryptography.md`, Core Philosophy](../../research/cryptography.md#core-philosophy).

**What resolving it would take.** Legal drafting of the license text; a decision on the OSI question against Core Philosophy, including whether the protocol libraries and the reference client carry different licenses; and the record field for content terms.

### Intentional identity fragmentation

**What was found.** The retention obligation is scoped per identity so that Sybil resistance is inherent rather than an overlay. One operator deliberately splitting into many identities to dilute that obligation is a distinct problem, and the specification explicitly declines to answer it.

**Where.** [`docs/research/cryptography.md`, Provisioning Liveness](../../research/cryptography.md#provisioning-liveness-centralization-and-participation).

**What resolving it would take.** A cost on identity creation that fragmentation cannot amortize, which is the same stake layer the compensation and enforcement items wait on — but the question is separable, because stake makes fragmentation expensive without making it incoherent.

## Releases on sequence alone

Nothing here is unresolved; each waits on the build sequence.

### Release workflow for a package that does not exist

**What was found.** `.github/workflows/npm-publish.yml` runs `npm ci`, `npm test`, and `npm publish` with the `npm_token` secret on every GitHub release, on Node.js 16, from the repository root, which is not an npm package; the project's npm package is `shells/npm-bootstrap`.

**Where.** `.github/workflows/npm-publish.yml`.

**What resolving it would take.** Removing the workflow, or replacing it at the `shells/npm-bootstrap` milestone with a publish job for that package's directory, built and signed by the release process.

### Boundary decisions under this gate

* **Git commit wrapping** — mutable DAGs and merge complexity against static immutable tarballs; packages first. Deferred in [`docs/research/MVP Scope.md`, Git Commit Wrapping](../../research/MVP%20Scope.md#git-commit-wrapping).
* **Active email bot for First Finder escrow** — excluded for spam and domain reputation, and made costless by the claim-set mechanism. Deferred in [`docs/research/MVP Scope.md`, Active Email Bot for First Finder Escrow](../../research/MVP%20Scope.md#active-email-bot-for-first-finder-escrow).
* **Version alignment engine** — resolution belongs to the package manager. Deferred in [`docs/research/MVP Scope.md`, Version Alignment Engine](../../research/MVP%20Scope.md#version-alignment-engine).