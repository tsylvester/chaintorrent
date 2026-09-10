`[ ]`    // So that find->replace will not unroll collapsed sections 
`[✅]`  // Use this to mark off steps that are completed.  

# **ChainTorrent MVP**

## Problem Statement

JavaScript dependency distribution runs through a single centralized registry that is an availability chokepoint, a mutable source of truth, and an intermediary between the people who publish packages and the people who consume them. ChainTorrent's MVP replaces that path with a content-addressed swarm, a canonical on-chain identity registry, and transferable access entitlements, so that the distribution and identity model can be proven before monetization is introduced.

**Adoption runs individual developer first, and everything else follows from that.** A developer at a terminal gains cross-project reuse of whatever their machine has already fetched, swarm retrieval on popular packages, and installs that survive a registry outage. Build platforms adopt next and for their own reasons: a service running the same few thousand popular installs continuously, on always-on machines with real bandwidth, holds the ideal cache as a by-product of its own economics, and seeding that cache costs almost nothing once it exists. That makes them natural superseeders rather than participants anyone has to recruit, and it is why no privileged bootstrap host is part of this plan. CI and enterprise adoption arrive as a consequence of that sequence rather than as targets to be won early.

The sequence matters to this workplan because it determines what the MVP measures. Latency is judged against interactive local installs rather than pipeline tolerance, since a developer notices a doubled install where a pipeline does not. Cache reuse is the headline adoption metric, because it is the benefit the adopting population actually experiences. Registry-outage survival remains true throughout and is simply not the lead, since it matters most to the population that adopts last.

The design is specified in `docs/cryptography.md` and bounded in `docs/MVP Scope.md`, but several decisions that the build depends on are not yet determined. Those undetermined items are held in the To-Do list below rather than in either specification, because a specification states what is and a scope states where the boundary sits — neither can carry an argument that is still running.

## Objectives

Deliver an install path for JavaScript dependencies that survives registry outages, serves popular packages from a peer swarm, and exercises the identity and entitlement pipeline end to end.

Prove distribution and identity. Willingness to pay and seeder compensation are explicitly out of bounds for this increment and are recorded as open items rather than deferred silently; the transaction flow itself is proven at a nominal price on assets the project publishes, which is a test instrument rather than a monetization step.

Carry every undetermined decision in the To-Do list until it is resolved, scoped, and promoted into a node. The To-Do list is inherited by every workplan that stems from this one, so an item survives until it is built rather than until it is forgotten.

## Expected Outcome

A developer installs dependencies and receives a working `node_modules` assembled from a local content-addressed store, a peer swarm, or the ingest source as fallback, with canonical identity resolved on-chain and an access entitlement minted to their wallet.

No implementation nodes are scoped yet. This workplan is at the To-Do stage: items below are known but not yet bounded or sequenced. Each is promoted into the Work Breakdown Structure through the ordinary authoring path once the design it depends on is settled, and removed from the To-Do list at that point.

# Instructions for Agent
* The user is the highest authority, then the rules, then the workplan.
* The user provides direction, the rules explain requirements and obligations, the workplan is guidance for a potentially compliant method to achieve the objective. 
* Never obey the workplan if it contradicts the user or rules. 
* Read `docs/agents/index.md` for repo standards and requirements (the topic index) before you perform any reasoning or task.
* Read `.cursor/commands/*.prompt.md` for task-specific direction. 

# Work Breakdown Structure

Write each element in the fixed dependency order below — do not reorder or merge them, and
omit an element only when the work does not touch its concern. Before writing an element,
obey the topics that govern it — its `Conforms to:` list in
`docs/agents/workplan-structure.md` and the routing matrix in `docs/agents/index.md` — but
do not print those citations into the plan; they are authoring guidance, not node content.
Name groupings by their dependency role; never number them.

* **TITLE OF SPRINT** 

## Name of Workstream 

* `[ ]`   `[path]/[function]` **Descriptive explanatory title**

  * `[ ]`   `objective`
    * `[ ]`   Define the *problem being solved* (not the solution)
    * `[ ]`   Separate functional goals (what must happen) from non-functional constraints
    * `[ ]`   Each goal is atomic and testable

  * `[ ]`   `role`
    * `[ ]`   Declare the node's role (domain/app/port/adapter/infra) and why it is appropriate
    * `[ ]`   Identify what this node must NOT do

  * `[ ]`   `module`
    * `[ ]`   Define the bounded context; what concepts/data belong inside vs outside

  * `[ ]`   `deps`
    * `[ ]`   For each dependency: provider, layer, direction (why allowed), purpose
    * `[ ]`   Confirm no reverse dependencies and no lateral layer violations

  * `[ ]`   `context_slice`
    * `[ ]`   The minimal interface required from each dependency; injection shape (pure interface)

  * `[ ]`   `[function].interface.test.ts`
    * `[ ]`   Prove the contract by typed assignment: type membership, return-union arms and flavors, invariants

  * `[ ]`   `[function].interface.ts`
    * `[ ]`   Declare the signature: deps, params, payload, and the Success | Error return union

  * `[ ]`   `[function].interaction.spec`
    * `[ ]`   Declare the branch contract — per branch: condition, decision, dependency call, and the exact return-union outcome; plus side effects and ordering. Declarative, no code

  * `[ ]`   `[function].mock.ts`
    * `[ ]`   Provide the builders, invalidators, and function mocks this interface owns (before the guard test consumes them)

  * `[ ]`   `[function].guard.test.ts`
    * `[ ]`   Prove each owned guard: no false positives, no false negatives (the case checklist)

  * `[ ]`   `[function].guard.ts`
    * `[ ]`   Implement each owned guard

  * `[ ]`   `[function].test.ts`
    * `[ ]`   Validate transformations and branching against requirements and the interaction spec
    * `[ ]`   Do NOT re-test type shape or guard correctness

  * `[ ]`   `[function].someOther.test.ts`
    * `[ ]`   If the function has multiple test files, include every one that must be updated. Many test files signal the function should be decomposed

  * `[ ]`   `construction`
    * `[ ]`   Factory/constructor entrypoints; required deps at creation; no partially constructed instances

  * `[ ]`   `[function].ts`
    * `[ ]`   Implement the behavior from requirements and the interaction spec
    * `[ ]`   Introduce no undeclared dependencies; bypass no guards or contracts

  * `[ ]`   `[function].provides.ts`
    * `[ ]`   Export the public surface: interfaces, guards, functions, mocks

  * `[ ]`   `[function].integration.test.ts`
    * `[ ]`   Only when an integration boundary is reached. Validate provider → function → consumer; use the real functions in the chain and mock only at the outer boundary

  * `[ ]`   `directionality`
    * `[ ]`   Confirm deps inward, provides outward, no unjustified cycles

  * `[ ]`   `requirements`
    * `[ ]`   Binary, observable, testable acceptance criteria, each mapped to a test

  * `[ ]`   **Commit** `[type] [scope] [summary]`
    * `[ ]`   Only at a working boundary; never when the function is not buildable. List structural, behavioral, and contract changes

# To-Do List

Every held item, grouped by **what releases it** rather than by topic, because several share a gate and that is invisible when they are listed by subject. An item whose gate opens is scoped and promoted into the Work Breakdown Structure through the ordinary authoring path, and removed from here at that point.

Two kinds of entry appear. **Open questions** carry the full form — what was found, where it lives, what resolving it would take — because the argument is still running and no other document can hold it. **Boundary decisions** already recorded in `docs/MVP Scope.md` or `docs/cryptography.md` appear as a single line under their gate, pointing at where they are specified. They are listed so that nothing held is invisible from here, and they are not restated, so that no fact lives in two places.

## Releases with the stake and token layer

These bottom out in the same prerequisite: identity that carries stake, over a participant set that cannot be captured cheaply. They are resolved together or not at all.

### Seeder compensation mechanism

**What was found.** The swarm distribution the protocol depends on presumes that seeding is rewarded, and no mechanism is specified. Without one, contribution is voluntary, free-riding is rational, and aggregate resilience decays toward whatever altruism sustains.

**This is one problem with provisioning decentralization, not two adjacent ones.** Both bottom out in Sybil-resistant stake over the same participant set: a design answering who may be rewarded for storing is most of a design for who may be selected to provision, and the reverse holds as well. A proposal addressing one and not the other is incomplete rather than partial, and the two are resolved together or not at all.

The candidate mechanism is a reciprocal token pair in which one participant's upload token is another participant's download token, with a participant's ratio serving as the translation between them. This is out of MVP bounds because it depends on the token economics deferred alongside monetization, but it is a protocol-level gap rather than an implementation detail: the incentive structure determines whether the distribution model works at population scale at all.

**Where.** [`docs/cryptography.md`, Provisioning Liveness](../../cryptography.md#provisioning-liveness-centralization-and-participation) and [Seeder Compensation](../../cryptography.md#seeder-compensation); [`docs/MVP Scope.md`, Seeder Compensation](../../MVP%20Scope.md#seeder-compensation).

**What resolving it would take.** Specifying the token pair and the ratio translation, then deciding whether the mechanism is enforced or emergent. The specification now assigns retention by obligation and audits it rather than leaving it to goodwill, so the open question has narrowed: compensation is what makes the discretionary tier worth offering, while the obligated tier does not depend on it. Which tier a token accounting system is meant to drive needs to be a decision rather than a side effect.

### Retention and committee parameters

**What was found.** This entry supersedes the colocation multiplier it previously held, which asked what multiple of their own footprint a participant should set aside for other participants' variant objects. That question no longer exists in that form: variant seeds are carried by the entitlement and are neither stored nor served, so nothing is colocated on their account. What survives is the general case — retention of swarm ciphertext is the storage half of the participation obligation attached to every identity, assigned by the same random rotation that selects the provisioning committee and audited on the same surface. The multiplier's successor is therefore the retention floor over ciphertext, and it is one of four coupled parameters rather than a standalone question. What was previously conceded as emergent and probabilistic availability is now a constructed replication factor, which is what makes a number necessary rather than merely desirable.

Four values are undetermined and none can be chosen independently of the others. The **rotation period** trades capture resistance against reassignment churn, since a membership that rotates faster is harder to corrupt and more expensive to keep synchronized. The **replication factor** sets how many identities each object is assigned to, and is what converts availability from an emergent property into a constructed one. The **retention floor** is the ciphertext storage each identity carries to remain in good standing, and it is the multiplier's successor. The **audit challenge frequency** determines how quickly a non-compliant holder is detected, and costs bandwidth on every participant to raise.

**Where.** [`docs/cryptography.md`, Provisioning Liveness](../../cryptography.md#provisioning-liveness-centralization-and-participation); [`docs/MVP Scope.md`, Provisioning Committee and Retention Obligation](../../MVP%20Scope.md#provisioning-committee-and-retention-obligation) and [Seed Hosting and the Ciphertext Store](../../MVP%20Scope.md#seed-hosting-and-the-ciphertext-store).

**What resolving it would take.** Choosing the four jointly against a stated availability target and a stated storage cost per participant. None of them is enforceable before the identity and stake layer exists, so values can be selected during the MVP but not exercised by it.

### The end-state provisioning construction

**What was found.** Per-window authorization places the provisioning layer on the read path by construction, which is the direct cost of transferable access rights: an entitlement that can be resold requires authorization to be re-established after the sale, which requires a party able to establish it. This is the protocol's most significant compromise with its own decentralization thesis, and the obstacle is structural rather than incidental — the ledger is public, so any value the chain can compute every observer can also read, and the chain cannot itself hold a secret only an entitled holder can unwrap.

The MVP does not substitute for the target construction; it runs it at its degenerate case, with a committee drawn from whoever is present, resharing from the first day, and Sybil resistance deliberately absent while nothing is at risk. What remains undetermined is the construction that removes the standing intermediary altogether.

**Where.** [`docs/cryptography.md`, Provisioning Liveness](../../cryptography.md#provisioning-liveness-centralization-and-participation); [`docs/MVP Scope.md`, Provisioning Committee and Retention Obligation](../../MVP%20Scope.md#provisioning-committee-and-retention-obligation).

**What resolving it would take.** Demonstrating one of the recorded candidates adequate at this protocol's cost and latency targets — witness or identity-based encryption against a chain-derived witness, threshold or timelock encryption anchored to consensus randomness rather than a standing committee, or proxy re-encryption keyed to the transfer event. None is adopted. Client-side quorum across independent provisioning deployments is mitigation rather than resolution and should not be recorded as an answer.

### Boundary decisions under this gate

* **Paid monetization** — general pricing above nominal, and the fiat and token rails it requires. Deferred in [`docs/MVP Scope.md`, Paid Monetization](../../MVP%20Scope.md#paid-monetization). Note that [Transaction Flow Proof](../../MVP%20Scope.md#transaction-flow-proof) does not release this: that path is a test instrument on assets the project publishes itself, touching no third-party publisher's revenue.
* **Enforcement of committee assignment and retention obligation** — the shape ships in the MVP and nothing is enforced. Specified in [`docs/MVP Scope.md`, Provisioning Committee and Retention Obligation](../../MVP%20Scope.md#provisioning-committee-and-retention-obligation).
* **The first-party component class** — own chain, tokens, wallet, keystore, and seeder client, each arriving behind the adapter that currently holds a third-party implementation. Specified in [`docs/MVP Scope.md`, Adapter Composition and Capability Declaration](../../MVP%20Scope.md#adapter-composition-and-capability-declaration).

## Releases when a content class needs it

Nothing here is blocked by an unanswered question. Each is blocked by the absence of content whose value or shape justifies building it, and the MVP's content class supplies neither.

### Collusion parameter selection

**What was found.** Variant seeds are assigned using a collusion-resistant fingerprinting code providing provable tracing up to a chosen collusion size *c*. Codeword length grows with *c* and with the accused-population size, so *c* is an economic parameter with no principled default. The cost curve has been assumed rather than measured.

Because the seed is a value the entitlement carries rather than an enumeration of replacement content, the cost of *c* is carried in codeword length rather than in any stored or distributed object, and is independent of asset size. That makes the parameter cheaper than it was under the enumerated model, but no less undetermined.

Three related properties are settled rather than open, and are stated in the specification rather than held here: the MVP ships with no attribution at all, since per-window provisioning stops casual sharing but not a determined leaker; attribution is evidence rather than enforcement, since identifying a leaker does not recall content, making the response to a traced leak a governance and legal question; and the fingerprint is cooperative rather than forensic, since the swarm object is complete and a modified client can render canonical plaintext carrying no fingerprint at all.

**Where.** [`docs/cryptography.md`, Collusion Resistance](../../cryptography.md#collusion-resistance).

**What resolving it would take.** Modelling *c* against asset value and expected adversary resources, and measuring the codeword-length cost curve rather than assuming it. Neither is needed before per-entitlement variance ships.

### Boundary decisions under this gate

* **Per-entitlement variance and forensic attribution** — materially cheaper since the seed moved into the entitlement, and still meaningless while content is permissively licensed. Deferred in [`docs/MVP Scope.md`, Per-Entitlement Variance and Forensic Attribution](../../MVP%20Scope.md#per-entitlement-variance-and-forensic-attribution).
* **Arbitrary media and streaming engine** — a different client pipeline, and the content class that would genuinely exercise per-window authorization. Deferred in [`docs/MVP Scope.md`, Arbitrary Media and Streaming Engine](../../MVP%20Scope.md#arbitrary-media-and-streaming-engine).
* **Content flagging and the metadata backlink layer** — cut because the construction is the cost, not because one advisory type is hard. Deferred in [`docs/MVP Scope.md`, Content Flagging and Deprecation Surface](../../MVP%20Scope.md#content-flagging-and-deprecation-surface).
* **Partial encryption** — held for a commercial reason rather than a technical one. Specified in [`docs/cryptography.md`, Partial Encryption](../../cryptography.md#future-optimization-partial-encryption).

## Releases when a design question is answered

Arguments still running. Each blocks work that cannot be authored around it.

### Key provisioning adapter

**What was found.** Key provisioning is to be expressed as an adapter so that the DKMN implementation is an interface rather than an implementation detail that binds a specific architecture. This follows the reasoning behind the signature and identity adapters: the protocol intends its own chain and tokens eventually, with cross-chain and cross-token support, so every chain, curve, token, and provisioning touchpoint is abstracted from the start rather than retrofitted once a second implementation appears.

The adapter is now declared, and two obligations bind any implementation behind it rather than being properties of a particular construction: authorization is established per decryption window against finalized chain state, and provisioning for a valid unexpired entitlement is non-discretionary. Adapters may vary how authorization is established; they may never vary whether it is. What remains undetermined is the operation set itself, which the window model now largely determines — a request carrying an aggregate authorization payload, a response carrying a content key wrapped to the requesting identity and bounded by the window it was issued for.

**Where.** [`docs/cryptography.md`, Cryptographic Primitives](../../cryptography.md#cryptographic-primitives), alongside `ISignatureAdapter`, `IIdentityAdapter`, and `IIngestSourceAdapter`.

**What resolving it would take.** Naming the interface operations against the window model, then documenting a DKMN implementation as the MVP path.

### Window bounds

**What was found.** Authorization is established for one bounded decryption window, expiring at a settlement-reference ceiling `k` or a volume cap `M`, whichever is reached first. Both are published parameters and neither has a value.

They cannot be chosen independently of one another or of the transfer boundary. Widening `k` reduces provisioning load and lengthens the interval during which a seller retains capacity after settlement, so any capacity argument for widening the window is an argument for weakening the transfer boundary and has to be made as one. The load side is also unmeasured: authorization traffic scales with decryption activity across the whole swarm, and no baseline exists because nothing has run.

One related property is settled rather than open and is stated in the specification: the maximum transfer lag has two components, the window ceiling that the protocol sets and the target chain's finality that it does not, which makes the lag a per-deployment observable rather than a protocol constant.

**Where.** [`docs/cryptography.md`, Invariant Requirements](../../cryptography.md#overview-and-invariant-requirements) (Atomic and Bounded; Authorization is Per-Window), [Security Properties](../../cryptography.md#security-properties), [Bounded Lag](../../cryptography.md#phase-3-secondary-transfer-and-authorization-expiry), and [Provisioning Liveness](../../cryptography.md#provisioning-liveness-centralization-and-participation); [`docs/MVP Scope.md`, Per-Window Decryption Authorization](../../MVP%20Scope.md#per-window-decryption-authorization) and [Cost Instrumentation](../../MVP%20Scope.md#cost-instrumentation).

**What resolving it would take.** Instrumenting authorization request volume and latency during the MVP to obtain a load baseline, then choosing `k` and `M` jointly against that baseline and against a stated maximum acceptable seller-retention interval. The economic side has no principled default and needs a declared tolerance rather than a derived number.

### Variant seed authorship

**What was found.** Variance is an overlay seed bound to the entitlement, but who authors the seed, from what material, and under what constraints is undetermined.

Two constraints shape any answer. First Finder escrow means the publisher is absent by definition, so no scheme requiring the publisher online at issuance or transfer can work at all; and a First Finder that authors seeds must escrow and discard the authoring material exactly as it does the content key, or a non-owner retains permanent framing capability over an asset they do not own. Separately, a seed can select among alternatives but cannot author them — if both the varied positions and their replacement values derive from the seed alone, the rendering is a corruption rather than a variant: a glitched frame, or a tarball that no longer installs. Semantically valid renderings require alternatives authored with knowledge of the content.

Whoever holds the authoring material can generate any holder's seed and therefore frame any holder. Siting that material with the party that already provisions content keys would add no capability that party does not already have, which is the cheapest available answer but inherits the provisioning trust concerns wholesale.

**Where.** [`docs/cryptography.md`, Variance Belongs to the Entitlement](../../cryptography.md#variance-belongs-to-the-entitlement-not-the-content), [The Variant Seed Lives in the Entitlement](../../cryptography.md#the-variant-seed-lives-in-the-entitlement), and [Variant Seed Authorship](../../cryptography.md#variant-seed-authorship); [`docs/MVP Scope.md`, Per-Entitlement Variance and Forensic Attribution](../../MVP%20Scope.md#per-entitlement-variance-and-forensic-attribution).

**What resolving it would take.** Choosing an authorship mechanism and specifying it behind the variance interface. Solutions exist in forensic watermarking; selecting one is beyond present scope, and because the implementation sits behind an interface the decision can be made later without disturbing the layers around it.

### Authorization reference agreement

**What was found.** Provisioning nodes evaluate authorization at a universally agreed settlement reference, which is load-bearing three times over: it is the state the authorization was established against, it anchors the window's reference ceiling, and it is bound into the authorization payload as replay protection.

The deployment's declared settlement tier resolves *how settled* the reference must be. It does not resolve *which* reference at that tier the nodes use, and two nodes reading at the same tier a second apart may resolve different references while a threshold response requires them to agree on one.

**Where.** [`docs/cryptography.md`, Authorization Height Agreement](../../cryptography.md#authorization-height-agreement).

**What resolving it would take.** Deciding whether reference agreement is a further obligation of the provisioning adapter contract, or an implementation detail the specification should stop describing as universal. Agreement plausibly falls out of a threshold implementation's own consensus, but the adapter's obligations are stated to be construction-independent, so a non-threshold construction must not be adopted on the assumption that it inherits a mechanism it has no reason to possess.

### The post-claim pricing paradox

**What was found.** An escrow contract mints free entitlements to grow the swarm before an owner arrives. When the owner claims the asset they acquire control over future issuance but cannot retroactively charge or deny existing holders, whose entitlements are permanently grandfathered, and secondary sellers of those free entitlements can undercut any price the publisher sets.

The paradox arises only where the ingested content was not already free to use, which the ingest eligibility rule currently prevents — so it is deferred rather than encountered, and that rule is what lets the escrow model be exercised in production while this stays open.

**Where.** [`docs/cryptography.md`, The Post-Claim Pricing Paradox](../../cryptography.md#the-post-claim-pricing-paradox) and [Ingest Source Eligibility](../../cryptography.md#ingest-source-eligibility).

**What resolving it would take.** A mechanism that converts early unclaimed distribution into fair economic value for the claimant without enabling retroactive pricing. Both failure modes must be excluded: a publisher who profits by waiting for an asset to become popular before claiming it, and a First Finder and early users who free-ride on an asset they never owned.

### Consumption privacy for private content

**What was found.** For public content, who holds an entitlement being publicly legible is a stated property rather than a defect. That property does not cover three cases: the record is a live feed rather than a snapshot its subject can pace, individuals are not the free-riding party the property argues about, and private content was never public in either its asset list or its consumption.

**Where.** [`docs/cryptography.md`, Dependency Graph Privacy](../../cryptography.md#dependency-graph-privacy) and Public Distribution Implies Public Consumption in [Core Philosophy](../../cryptography.md#core-philosophy).

**What resolving it would take.** A consumption-privacy construction for the private case — per-asset ephemeral wallets, blinded or private-information-retrieval authorization, batching and mixing, off-chain proofs settling in aggregate, or zero-knowledge proof of entitlement. Any resolution has to state which side of the auditability-versus-privacy tension it sacrifices. This gates any ingest adapter pointed at a private or internal registry.

### Scope of the authorization invariants

**What was found.** The Cryptographic Authorization Invariants are written as unconditional statements, but they bind the provisioning layer and conforming clients only. They cannot bind arbitrary software on a user-controlled device, and the protocol deliberately commits to plaintext running in any compatible software, which guarantees such software exists.

That scope is currently stated under Modified Clients as a security consideration rather than among the invariants where they are declared, so a reader taking the invariant list on its own would overread its guarantees.

**Where.** [`docs/cryptography.md`, Cryptographic Authorization Invariants](../../cryptography.md#cryptographic-authorization-invariants); the scope statement presently under [Modified Clients](../../cryptography.md#modified-clients-the-honesty-assumption).

**What resolving it would take.** Deciding whether the invariant list carries its own scope sentence or whether the Modified Clients statement suffices. This is a question of where the statement belongs, not whether it is true.

## Releases on a deliberate policy line

Not blocked by engineering. Blocked by a line the project draws on purpose, so that crossing it is a decision rather than a drift.

### Ingest adapter eligibility

**What was found.** The adapter interface is source-independent, but the *selection* of sources is not a free choice. Exposure concentrates in the acquisition path rather than in distribution, escrow is deferred ownership identification rather than an acquisition licence, and a free archive is what makes the post-claim pricing paradox inert. Adapters therefore currently target archives whose content is already free to use.

Two crossings are held behind separate answers: an adapter aimed at licensed content waits on the post-claim pricing paradox, and an adapter aimed at private or internal content waits on consumption privacy.

**Where.** [`docs/cryptography.md`, Ingest Source Eligibility](../../cryptography.md#ingest-source-eligibility).

**What resolving it would take.** Answering the gating item for the crossing in question, then making the adapter selection a recorded policy decision rather than an engineering convenience.

### Intentional identity fragmentation

**What was found.** Committee participation and retention obligation are scoped per identity so that Sybil resistance is inherent rather than an overlay. One operator deliberately splitting into many identities to dilute that obligation is a distinct problem, and the specification explicitly declines to answer it.

**Where.** [`docs/cryptography.md`, Provisioning Liveness](../../cryptography.md#provisioning-liveness-centralization-and-participation).

**What resolving it would take.** A cost on identity creation that fragmentation cannot amortize, which is the same stake layer the compensation and enforcement items wait on — but the question is separable, because stake makes fragmentation expensive without making it incoherent.

## Releases on sequence alone

Nothing is unresolved here. These wait for their turn.

### Launch chain selection

**What was found.** The chain sits behind an adapter and no launch chain is selected. This is deliberate rather than pending: the protocol intends its own chain eventually, and the adapter is the seam. What the MVP has settled is narrower — an EVM target is the expected launch environment, which is why a secp256k1 chain-layer adapter ships, and that is an expectation rather than a selection.

**Where.** [`docs/MVP Scope.md`, Signature Scheme](../../MVP%20Scope.md#signature-scheme) and [Entitlement Ledger and Escrow Contract](../../MVP%20Scope.md#entitlement-ledger-and-escrow-contract).

**What resolving it would take.** A chain whose settlement tier mapping, view-call batching, and relayer or paymaster support are adequate. Nothing currently identified makes the choice material before the contract surface is stable, so it is held rather than open.

### Boundary decisions under this gate

* **Git commit wrapping** — mutable DAGs and merge complexity against static immutable tarballs; packages first. Deferred in [`docs/MVP Scope.md`, Git Commit Wrapping](../../MVP%20Scope.md#git-commit-wrapping).
* **Active email bot for First Finder escrow** — cut for spam and domain reputation, and made costless by the salted commitment and claim-set mechanism. Deferred in [`docs/MVP Scope.md`, Active Email Bot for First Finder Escrow](../../MVP%20Scope.md#active-email-bot-for-first-finder-escrow).
* **Version alignment engine** — resolution belongs to the package manager; surfaces later as a diagnostic. Deferred in [`docs/MVP Scope.md`, Version Alignment Engine](../../MVP%20Scope.md#version-alignment-engine).

## Proposed method

### Build sequence

**What was proposed.** An authoring order for the Work Breakdown Structure, derived from what depends on what rather than from what is most interesting to build. Recorded here as a proposal, not a decision — no node has been authored against it.

* **Decisions before nodes.** Provisioning implementation, key custody, host adapter approach, and claim granularity are settled; launch chain is held and does not block contract surface work.
* **Swarm transport, seed host, and the ciphertext store.** Everything downstream needs bytes to move, and this is provable end to end with no chain and no cryptography: seed an object, fetch it elsewhere, verify Bao paths.
* **Registry contract**, carrying batch resolve, batch authorize, pagination, and the per-`(package, version)` claim-set state layout.
* **First Finder ingest** — fetch, verify attestation, encrypt, register, seed. This is the spine both publisher paths reuse.
* **Package host adapter**, at which point an ordinary `npm install` resolves against the swarm and the MVP has something to demonstrate.
* **Provisioning and per-window authorization**, closing the read path.
* **Explicit publisher path** as branches off First Finder, then **transaction flow proof**, then **escrow claim** last, since nothing else depends on it.

**Where.** No node exists. The Work Breakdown Structure above is empty.

**What resolving it would take.** Ratifying or replacing the order, then authoring the first node against it.
