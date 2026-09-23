`[ ]`    // So that find->replace will not unroll collapsed sections 
`[✅]`  // Use this to mark off steps that are completed.  

# **ChainTorrent MVP**

## Problem Statement

JavaScript dependency distribution runs through a single centralized registry that is an availability chokepoint, a mutable source of truth, and an intermediary between the people who publish packages and the people who consume them. ChainTorrent's MVP replaces that path with a content-addressed swarm, a canonical on-chain identity registry, and transferable access entitlements, so that the distribution and identity model can be proven before monetization is introduced.

**Adoption runs individual developer first, and everything else follows from that.** A developer at a terminal gains cross-project reuse of whatever their machine has already fetched, swarm retrieval on popular packages, and installs that survive a registry outage. Build platforms adopt next and for their own reasons: a service running the same few thousand popular installs continuously, on always-on machines with real bandwidth, holds the ideal cache as a by-product of its own economics, and seeding that cache costs almost nothing once it exists. That makes them natural superseeders rather than participants anyone has to recruit, and it is why no privileged bootstrap host is part of this plan. CI and enterprise adoption arrive as a consequence of that sequence rather than as targets to be won early.

The sequence matters to this workplan because it determines what the MVP measures. Latency is judged against interactive local installs rather than pipeline tolerance, since a developer notices a doubled install where a pipeline does not. Cache reuse is the headline adoption metric, because it is the benefit the adopting population actually experiences. Registry-outage survival remains true throughout and is simply not the lead, since it matters most to the population that adopts last.

The design is specified in `docs/cryptography.md` and bounded in `docs/MVP Scope.md`, but several decisions that the build depends on are not yet determined. Those undetermined items are held in the To-Do list below rather than in either specification, because a specification states what is and a scope states where the boundary sits — neither can carry an argument that is still running.

The cryptographic construction is specified; its research record is `docs/cryptography-research-notebook.md`, with the current statement in `docs/cryptography-critical-path.md`. A validation harness on the resolved pairing curve produces the sizes, timings, and gas that fix the piece-group size, and no node that encrypts is scoped ahead of it.

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

**This is one problem with retention assignment, not two adjacent ones.** Both bottom out in Sybil-resistant stake over the same participant set: a design answering who may be rewarded for storing is most of a design for who may be assigned to store, and the reverse holds as well. A proposal addressing one and not the other is incomplete rather than partial, and the two are resolved together or not at all.

The candidate mechanism is a reciprocal token pair in which one participant's upload token is another participant's download token, with a participant's ratio serving as the translation between them. This is out of MVP bounds because it depends on the token economics deferred alongside monetization, but it is a protocol-level gap rather than an implementation detail: the incentive structure determines whether the distribution model works at population scale at all.

**Where.** [`docs/cryptography.md`, Provisioning Liveness](../../cryptography.md#provisioning-liveness-centralization-and-participation) and [Seeder Compensation](../../cryptography.md#seeder-compensation); [`docs/MVP Scope.md`, Seeder Compensation](../../MVP%20Scope.md#seeder-compensation).

**What resolving it would take.** Specifying the token pair and the ratio translation, then deciding whether the mechanism is enforced or emergent. The specification now assigns retention by obligation and audits it rather than leaving it to goodwill, so the open question has narrowed: compensation is what makes the discretionary tier worth offering, while the obligated tier does not depend on it. Which tier a token accounting system is meant to drive needs to be a decision rather than a side effect.

### Retention parameters

**What was found.** This entry supersedes the colocation multiplier it previously held, which asked what multiple of their own footprint a participant should set aside for other participants' variant objects. That question no longer exists in that form: variant seeds are carried by the entitlement and are neither stored nor served, so nothing is colocated on their account. What survives is the general case — retention of swarm ciphertext and header sidecars is the participation obligation attached to every identity, assigned by random rotation over the participant set and audited on one surface; the provisioning committee that once shared that rotation no longer exists. The multiplier's successor is therefore the retention floor over ciphertext, and it is one of four coupled parameters rather than a standalone question. What was previously conceded as emergent and probabilistic availability is now a constructed replication factor, which is what makes a number necessary rather than merely desirable.

Four values are undetermined and none can be chosen independently of the others. The **rotation period** trades capture resistance against reassignment churn, since an assignment that rotates faster is harder to game and more expensive to keep synchronized. The **replication factor** sets how many identities each object is assigned to, and is what converts availability from an emergent property into a constructed one. The **retention floor** is the ciphertext storage each identity carries to remain in good standing, and it is the multiplier's successor. The **audit challenge frequency** determines how quickly a non-compliant holder is detected, and costs bandwidth on every participant to raise.

**Where.** [`docs/cryptography.md`, Provisioning Liveness](../../cryptography.md#provisioning-liveness-centralization-and-participation), whose surviving content is the participation obligation; [`docs/MVP Scope.md`, Retention Obligation](../../MVP%20Scope.md#retention-obligation) and [Seed Hosting and the Ciphertext Store](../../MVP%20Scope.md#seed-hosting-and-the-ciphertext-store).

**What resolving it would take.** Choosing the four jointly against a stated availability target and a stated storage cost per participant. None of them is enforceable before the identity and stake layer exists, so values can be selected during the MVP but not exercised by it.

### Boundary decisions under this gate

* **Paid monetization** — general pricing above nominal, and the fiat and token rails it requires. Deferred in [`docs/MVP Scope.md`, Paid Monetization](../../MVP%20Scope.md#paid-monetization). Note that [Transaction Flow Proof](../../MVP%20Scope.md#transaction-flow-proof) does not release this: that path is a test instrument on assets the project publishes itself, touching no third-party publisher's revenue.
* **Enforcement of retention assignment and obligation** — the shape ships in the MVP and nothing is enforced. Specified in [`docs/MVP Scope.md`, Retention Obligation](../../MVP%20Scope.md#retention-obligation).
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
* **Composable container objects** — reference nesting with per-member roots in the manner of BitTorrent v2, so that seeding a collection seeds its members; noted, not designed. Deferred in [`docs/MVP Scope.md`, Composable Container Objects](../../MVP%20Scope.md#composable-container-objects).

## Releases when a design question is answered

Arguments still running. Each blocks work that cannot be authored around it.

### Attempt-rule parameters and piece-group size

**What was found.** The attempt rule replaces the former decryption windows. A conforming client reads a state view at the deployment's declared tier no older than `τ_soft` before every piece-group key derivation, renews its wallet-control assertion every `τ_wallet`, and destroys decrypt-capable material only when a transfer out reaches `HARD`. The piece-group size fixes how many pieces one capsule covers: smaller groups bound what one leaked key unlocks, larger groups bound the pairing work per byte. All three are published per deployment and none has a value.

Unlike the window bounds they replace, they carry no security-versus-load tradeoff against the transfer boundary: the seller's loss of capability is fixed at `HARD` regardless of their values. What they need is measurement, because the acceptable `τ_soft` depends on how much a state read adds to install time, and the piece-group size depends on decapsulation cost per group on the resolved curve.

**Where.** [`docs/cryptography.md`, Invariant Requirements](../../cryptography.md#overview-and-invariant-requirements) (Authorization is Per-Attempt), [Phase 2](../../cryptography.md#phase-2-consumption-and-decryption), and [Authorization Height Agreement](../../cryptography.md#authorization-height-agreement); [`docs/MVP Scope.md`, Native Credentials and Per-Attempt Authorization](../../MVP%20Scope.md#native-credentials-and-per-attempt-authorization) and [Cost Instrumentation](../../MVP%20Scope.md#cost-instrumentation); [`docs/MVP Application Requirements.md`, CD-07](../../MVP%20Application%20Requirements.md#credential-delivery).

**What resolving it would take.** Running the cryptographic validation harness on the resolved pairing curve to obtain capsule, envelope, and proof sizes, decapsulation time per group, proof generation and verification time, and delivery-verification gas, then choosing the piece-group size from those numbers against a declared latency budget; and instrumenting state-read latency during the MVP to choose `τ_soft` and `τ_wallet`. The harness is the first cryptographic thing built and is gated on the launch chain only for its curve; it runs on BN254 until the chain is chosen.

### Variant seed authorship

**What was found.** Variance is an overlay seed bound to the entitlement, but who authors the seed, from what material, and under what constraints is undetermined.

Two constraints shape any answer. First Finder escrow means the publisher is absent by definition, so no scheme requiring the publisher online at issuance or transfer can work at all; and a First Finder that authors seeds must escrow and discard the authoring material exactly as it does the content key, or a non-owner retains permanent framing capability over an asset they do not own. Separately, a seed can select among alternatives but cannot author them — if both the varied positions and their replacement values derive from the seed alone, the rendering is a corruption rather than a variant: a glitched frame, or a tarball that no longer installs. Semantically valid renderings require alternatives authored with knowledge of the content.

Whoever holds the authoring material can generate any holder's seed and therefore frame any holder. Siting that material with the credential author — the issuer at mint and the seller at sale, who already post the envelope the seed would ride in — is the cheapest available answer, but it would let a seller frame its buyer, which the framing rule forbids. Delivery itself is solved: the seed rides the credential envelope.

**Where.** [`docs/cryptography.md`, Variance Belongs to the Entitlement](../../cryptography.md#variance-belongs-to-the-entitlement-not-the-content), [The Variant Seed Lives in the Entitlement](../../cryptography.md#the-variant-seed-lives-in-the-entitlement), and [Variant Seed Authorship](../../cryptography.md#variant-seed-authorship); [`docs/MVP Scope.md`, Per-Entitlement Variance and Forensic Attribution](../../MVP%20Scope.md#per-entitlement-variance-and-forensic-attribution).

**What resolving it would take.** Choosing an authorship mechanism and specifying it behind the variance interface. Solutions exist in forensic watermarking; selecting one is beyond present scope, and because the implementation sits behind an interface the decision can be made later without disturbing the layers around it.

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

**What was found.** The Cryptographic Authorization Invariants are written as unconditional statements, but they bind the settlement contract, credential authors, and conforming clients only. They cannot bind arbitrary software on a user-controlled device, and the protocol deliberately commits to plaintext running in any compatible software, which guarantees such software exists.

That scope is currently stated under Modified Clients as a security consideration rather than among the invariants where they are declared, so a reader taking the invariant list on its own would overread its guarantees.

**Where.** [`docs/cryptography.md`, Cryptographic Authorization Invariants](../../cryptography.md#cryptographic-authorization-invariants); the scope statement presently under [Modified Clients](../../cryptography.md#modified-clients-the-honesty-assumption).

**What resolving it would take.** Deciding whether the invariant list carries its own scope sentence or whether the Modified Clients statement suffices. This is a question of where the statement belongs, not whether it is true.

### Entitlement migration across chain adapters

**What was found.** The chain sits behind an adapter and the protocol intends its own chain eventually, but an adapter swap moves nothing: entitlements, interval state, envelope digests, parameter-set liveness, deployment records, and identity bindings live in one chain's contract state, and a later chain, whether Ethereum mainnet, another L2, or the project's own, starts empty. Every invariant the protocol makes about an entitlement, asset binding, irrevocability, transferability, and the delivery chain that authenticates its credential history, is a statement about one ledger. No document specifies how an entitlement and its interval history become valid on a second ledger without a party acquiring discretion over the move, which the No Party May Selectively Withhold invariant forbids.

**Where.** [`docs/MVP Scope.md`, Adapter Composition and Capability Declaration](../../MVP%20Scope.md#adapter-composition-and-capability-declaration) and [Entitlement Ledger and Escrow Contract](../../MVP%20Scope.md#entitlement-ledger-and-escrow-contract); [`docs/cryptography.md`, Architectural Invariants](../../cryptography.md#overview-and-invariant-requirements) and [Credential Delivery](../../cryptography.md#cryptographic-primitives); the launch network decision recorded in [`docs/project-planning/product-requirements.md`](../../project-planning/product-requirements.md).

**What resolving it would take.** A migration mechanism with the same non-deniability the transfer path has: a snapshot, claim, or bridge construction under which a holder proves its entitlement and interval on the source ledger and obtains the equivalent record on the target ledger with no party able to withhold it, the source record retired or frozen so scarcity is not doubled, the envelope history carried or re-anchored so the next sale's proof has a base case, and parameter-set liveness reproduced so existing credentials keep decrypting. None of it is needed before the MVP, which lives on one chain. It is needed before any second chain adapter carries live entitlements, and it is the item that decides whether the first-party chain is a migration or a fresh start.

## Releases on a deliberate policy line

Not blocked by engineering. Blocked by a line the project draws on purpose, so that crossing it is a decision rather than a drift.

### Ingest adapter eligibility

**What was found.** The adapter interface is source-independent, but the *selection* of sources is not a free choice. Exposure concentrates in the acquisition path rather than in distribution, escrow is deferred ownership identification rather than an acquisition licence, and a free archive is what makes the post-claim pricing paradox inert. Adapters therefore currently target archives whose content is already free to use.

Two crossings are held behind separate answers: an adapter aimed at licensed content waits on the post-claim pricing paradox, and an adapter aimed at private or internal content waits on consumption privacy.

**Where.** [`docs/cryptography.md`, Ingest Source Eligibility](../../cryptography.md#ingest-source-eligibility).

**What resolving it would take.** Answering the gating item for the crossing in question, then making the adapter selection a recorded policy decision rather than an engineering convenience.

### Distribution and client license

**What was found.** The software must permit forking and modification while making a non-conforming client a license violation, and each asset record must carry the rights-holder's content terms, with the project's own packages obligating an entitlement for use and a license per copy sold or bundled. A conformance clause makes the software license source-available rather than OSI-open, and it binds forks of the code, not a clean-room client written from the specification.

**Where.** [`docs/MVP Scope.md`, Distribution and Client License](../../MVP%20Scope.md#distribution-and-client-license); [`docs/MVP Application Requirements.md`, LI-01](../../MVP%20Application%20Requirements.md#licensing); [`docs/cryptography.md`, Core Philosophy](../../cryptography.md#core-philosophy).

**What resolving it would take.** Legal drafting of the license text; a decision on the OSI question against Core Philosophy, including whether the protocol libraries and the reference client carry different licenses; and the record field for content terms.

### Intentional identity fragmentation

**What was found.** The retention obligation is scoped per identity so that Sybil resistance is inherent rather than an overlay. One operator deliberately splitting into many identities to dilute that obligation is a distinct problem, and the specification explicitly declines to answer it.

**Where.** [`docs/cryptography.md`, Provisioning Liveness](../../cryptography.md#provisioning-liveness-centralization-and-participation).

**What resolving it would take.** A cost on identity creation that fragmentation cannot amortize, which is the same stake layer the compensation and enforcement items wait on — but the question is separable, because stake makes fragmentation expensive without making it incoherent.

## Releases on sequence alone

Nothing is unresolved here. These wait for their turn.

### Boundary decisions under this gate

* **Git commit wrapping** — mutable DAGs and merge complexity against static immutable tarballs; packages first. Deferred in [`docs/MVP Scope.md`, Git Commit Wrapping](../../MVP%20Scope.md#git-commit-wrapping).
* **Active email bot for First Finder escrow** — cut for spam and domain reputation, and made costless by the claim-set mechanism. Deferred in [`docs/MVP Scope.md`, Active Email Bot for First Finder Escrow](../../MVP%20Scope.md#active-email-bot-for-first-finder-escrow).
* **Version alignment engine** — resolution belongs to the package manager; surfaces later as a diagnostic. Deferred in [`docs/MVP Scope.md`, Version Alignment Engine](../../MVP%20Scope.md#version-alignment-engine).

## Proposed method

### Build sequence

**What was proposed.** An authoring order for the Work Breakdown Structure, derived from what depends on what rather than from what is most interesting to build. Recorded here as a proposal, not a decision — no node has been authored against it.

* **Decisions before nodes.** The credential construction, key custody, host adapter approach, claim granularity, launch network, and the absence of a maintainer commitment in the escrow record are settled; the license is not settled and is a release prerequisite rather than a node blocker.
* **Cryptographic validation harness.** The credential KEM, envelope, and delivery proof on the resolved pairing adapter, with the verifier deployed to a test chain, producing the measurements that fix the piece-group size. This precedes any node that encrypts, because piece geometry is a suite parameter every later deployment carries.
* **Swarm transport, seed host, and the ciphertext store.** Everything downstream needs bytes to move, and this is provable end to end with no chain and no cryptography: seed an object and its sidecar, fetch them elsewhere, verify Bao paths.
* **Registry contract**, carrying batch resolve, batch authorize, pagination, envelope-key registration, the parameter-set registry with the sidecar-coverage rule, per-entitlement interval state, delivery verification through the precompiles, and the per-`(package, version)` claim-set state layout.
* **First Finder ingest** — fetch, verify attestation, generate the parameter set, encrypt per group, build the sidecar, register, seed. This is the spine both publisher paths reuse.
* **Package host adapter**, at which point an ordinary `npm install` resolves against the swarm and the MVP has something to demonstrate.
* **Credential delivery and per-attempt authorization**, closing the read path: mint delivery through the relayer, local decryption under the attempt rule, interval-end destruction.
* **Explicit publisher path** as branches off First Finder, then **transaction flow proof**, which is where transfer delivery is first exercised, then **escrow claim** last, since nothing else depends on it.

**Where.** No node exists. The Work Breakdown Structure above is empty.

**What resolving it would take.** Ratifying or replacing the order, then authoring the first node against it.
