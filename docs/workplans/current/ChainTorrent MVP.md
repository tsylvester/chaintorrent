`[ ]`    // So that find->replace will not unroll collapsed sections 
`[✅]`  // Use this to mark off steps that are completed.  

# **ChainTorrent MVP**

## Problem Statement

JavaScript dependency distribution runs through a single centralized registry that is a availability chokepoint, a mutable source of truth, and an intermediary between the people who publish packages and the people who consume them. ChainTorrent's MVP replaces that path with a content-addressed swarm, a canonical on-chain identity registry, and transferable access entitlements priced at $0.00, so that the distribution and identity model can be proven before any monetization is introduced.

The design is specified in `docs/cryptography.md` and bounded in `docs/MVP Scope.md`, but several decisions that the build depends on are not yet determined. Those undetermined items are held in the To-Do list below rather than in either specification, because a specification states what is and a scope states where the boundary sits — neither can carry an argument that is still running.

## Objectives

Deliver a `torrent install` path for JavaScript dependencies that survives registry outages, serves popular packages from a peer swarm, and exercises the identity and entitlement pipeline end to end with all monetization set to $0.00.

Prove distribution and identity. Willingness to pay and seeder compensation are explicitly out of bounds for this increment and are recorded as open items rather than deferred silently.

Carry every undetermined decision in the To-Do list until it is resolved, scoped, and promoted into a node. The To-Do list is inherited by every workplan that stems from this one, so an item survives until it is built rather than until it is forgotten.

## Expected Outcome

A developer runs `torrent install` and receives a working `node_modules` assembled from a local content-addressed store, a peer swarm, or the NPM registry as fallback, with canonical identity resolved on-chain and an access entitlement minted to their wallet at no cost and no gas.

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

## Key custody for First Finder ingests

**What was found.** The DKMN was performing two distinct jobs and the sequencer wrap model replaces only one of them. It provisioned content keys to consumers, which is now handled by wrapping the content key to the recipient's public key at mint and at transfer. It also held First Finder content keys in escrow between ingest and the Web2 maintainer's eventual claim, and that job has no replacement.

An asymmetry that was invisible while the DKMN held everything sharpens the gap. Explicit publishers derive content keys deterministically through the layered chain `seed_phrase → publisher_root → master_key → SCK`, so they can regenerate any content key they have ever issued from the seed phrase, the asset identity hash, and the deployment id, storing nothing. First Finders generate random content keys, as the anti-derivability rule requires, so the key exists only where it was generated — on an arbitrary peer that is not a trustworthy custodian and may never be seen again.

A standing obligation constrains any answer. An entitlement against an old deployment remains transferable indefinitely, and every transfer requires a re-wrap for the new holder, so the ability to reproduce a content key is permanent rather than incidental. Deterministic derivation satisfies it for free. Random content keys satisfy it only if something stores them, and whatever stores them must not be able to deny a future re-wrap, because a party that can decline to re-wrap has revoked access by inaction, which the protocol's core thesis forbids.

**Where.** `docs/cryptography.md` Phase 1.3 and Phase 1.6; `docs/MVP Scope.md` In Scope item 4 and Deferred item 2. First Finder ingestion is the MVP's bootstrap mechanism, so this sits on the delivery path rather than beside it.

**What resolving it would take.** A custody or derivation mechanism for non-owner ingests that reintroduces no standing key-holding network and leaves no party able to deny a future re-wrap. Candidate shapes, none evaluated and none selected: deterministic derivation from a secret independent of the First Finder's own material; sequencer-held escrow offered as a service; wrapping the content key at ingest to a claimable escrow identity the maintainer later assumes.

## Replay protection under sequencer provisioning

**What was found.** The replay analysis is written entirely against the DKMN handshake — timestamped or nonce-based signatures from the consumer preventing an attacker from intercepting and replaying an authorization request. Under sequencer provisioning there is no recurring read-time handshake to replay, because the content key arrives wrapped at mint and is unwrapped locally thereafter.

The replay surface does not disappear, it moves to the sequencer interaction at mint and at transfer. That surface has a different shape: far lower frequency, materially higher value per event, and a settlement atomicity requirement, since a transfer does not complete until the wrap is produced.

**Where.** `docs/cryptography.md` §5.2. Shares an origin with the First Finder custody item and is expected to resolve alongside it.

**What resolving it would take.** Restating the replay threat against mint and transfer settlement rather than a read-time handshake, and stating how nonce or ordering guarantees compose with atomic settlement.

## Key provisioning adapter

**What was found.** Key provisioning is to be expressed as an adapter so that the sequencer wrap model and the DKMN are implementations behind one interface rather than a choice between architectures. This follows the reasoning behind the signature and identity adapters: the protocol intends its own chain and tokens eventually, with cross-chain and cross-token support, so every chain, curve, token, and provisioning touchpoint is abstracted from the start rather than retrofitted once a second implementation appears.

Authoring the interface is deferred rather than blocked. The custody and replay items both determine what operations the interface must expose, so naming them now would fix the abstraction around an incomplete picture.

**Where.** `docs/cryptography.md` §2, alongside `ISignatureAdapter` and `IIdentityAdapter`.

**What resolving it would take.** Naming the interface operations once custody and replay are settled, then documenting a sequencer wrap implementation as the MVP path and a DKMN implementation as a documented alternative.

## DKMN references throughout the specification

**What was found.** The DKMN is woven through more of the specification than the provisioning sections alone. Removing or demoting it touches the anti-derivability rationale in §1.1, the primitives list in §2, the condition-locking step in Phase 1.6, the whole of Phase 2 — the chunked batch handshake, sliding-window batching, and the finalized-block-height consensus that exists solely to prevent split-brain across DKMN nodes — the replay analysis in §5.2, the claim in §5.4 that the network provisions the same content key to every wallet, the open problem in §8.1, and the `[ DKMN Escrows SCK via Escrow Adapter ]` node in the Identity Resolution Lifecycle diagram.

Most of these are mechanical once the design is settled. Phase 2 is not: under sequencer provisioning the client unwraps locally and none of the batch handshake machinery runs, so that phase is a rewrite rather than an edit. This reduces MVP scope rather than expanding it.

One phrase is smaller than the rest and should not be lost among them. §5.3 states that unauthorized wallets cannot obtain the content key *from the network*, which remains true under sequencer provisioning but is phrased around an architecture that will no longer be primary.

**Where.** `docs/cryptography.md` §1.1, §2, Phase 1.6, Phase 2 in full, the Identity Resolution Lifecycle diagram, §5.2, §5.3, §5.4, §8.1.

**What resolving it would take.** A single pass once the custody, replay, and adapter items are settled, so the specification is not edited three times against three partial answers.

## Rotation as versioning

**What was found.** The rule follows from the protocol's revocation thesis but has not been confirmed. If a publisher rotating a content key required re-wrapping for every existing holder, a publisher who declined to perform that fanout would have revoked access by inaction, which the protocol forbids. The conclusion is that rotation is versioning: issuing a new deployment id creates a new deployment, the previous deployment stays live and permanently readable, existing wrapped keys never expire, and no re-wrap fanout exists to withhold.

**Where.** `docs/cryptography.md` Phase 1.3 and §7.4.

**What resolving it would take.** Confirmation or rejection of the derived rule. If confirmed it becomes a plain statement of design in the specification; the standing regenerability obligation it creates is already recorded under First Finder key custody.

## Trustless key release without a provisioning party

**What was found.** The sequencer wrap model removes the intermediary from the read path entirely — a wrapped content key needs no network to use, so a participant contacts a provisioning party to acquire or transfer, never to install. That resolves the liveness objection that motivated eliminating the DKMN, and it relocates trust from a standing third-party network holding every content key in existence to the publisher, who knows their own content key by definition.

What remains is narrower than the original problem but is not closed. Acquisition still depends on a party being available at mint and at transfer. The fully trustless case — the chain alone releasing a content key with no party involved — stays blocked on the obstacle that a public ledger cannot hold a secret only an entitled identity can unwrap, since any value the chain can compute or reveal is readable by every observer.

Candidate directions, none adopted and none demonstrated adequate at this protocol's cost and latency targets: witness or identity-based encryption against a chain-derived witness, where finality itself releases the decryption capability to the entitled party; threshold or timelock encryption anchored to consensus randomness rather than to a standing committee; proxy re-encryption keyed to the entitlement transfer, moving trust from a persistent network to the transfer event.

**Where.** `docs/cryptography.md` §7.1, which now specifies the sequencer wrap; the eliminated §8.1.

**What resolving it would take.** Either a construction that removes the acquisition-time dependency, or an explicit decision that acquisition-time dependency is acceptable and the ambition is retired.

## Collusion parameter selection

**What was found.** Variant sets are assigned using a collusion-resistant fingerprinting code providing provable tracing up to a chosen collusion size *c*. Variant-set size grows with *c* and with the accused-population size, so *c* is an economic parameter with no principled default. The cost curve has been assumed rather than measured.

Two related properties are settled rather than open, and are stated in the specification rather than held here: the MVP ships with no attribution at all, since wrapped keys stop casual sharing but not a determined leaker; and attribution is evidence rather than enforcement, since identifying a leaker does not recall content, making the response to a traced leak a governance and legal question.

**Where.** `docs/cryptography.md` §7.5; the eliminated §8.2.

**What resolving it would take.** Modelling *c* against asset value and expected adversary resources, and measuring the variant-size cost curve rather than assuming it. Neither is needed before per-entitlement variance ships.

## Variant object availability

**What was found.** A per-entitlement variant object has a swarm the size of one user's device set, so it may be unavailable when those devices are offline. Two mechanisms are identified: the sequencer pins variant objects, or the variant set is made deterministically regenerable from the invariant content, the entitlement id, and a publisher secret. Regeneration is preferred because it bounds publisher storage to zero, and its domain-separation requirement is already specified — regeneration inputs must be separated with the same discipline as content key derivation, or two entitlements collide onto the same variant set and attribution silently fails.

The mechanism is preferred but unspecified. Which of the two is normative has not been decided.

**Where.** `docs/cryptography.md` §7.3; the eliminated §8.2.

**What resolving it would take.** Selecting pinning or regeneration as normative, and specifying the regeneration derivation if regeneration is chosen.

## Open problems register boundary

**What was found.** The boundary is settled and stated in both documents: the specification holds protocol *properties* that are unresolved by design and that a reader should encounter in place; this list holds undetermined *authorship and design decisions* that block scoping. Applying it moved DKMN centralization and residual traceability gaps out of `docs/cryptography.md` §8 and into this list, leaving dependency graph privacy as the only Open Problem the specification still carries.

What remains unverified is whether that single survivor is correctly classified. Dependency graph privacy is a genuine protocol property, but it also has candidate mitigations that would each be a design decision, so it may belong in both places with different framings rather than in one.

**Where.** `docs/cryptography.md` §8.1; this list.

**What resolving it would take.** A read of §8.1 against the boundary now that only it remains, and a split of its mitigation directions into this list if they are decisions rather than properties.

## Evidence for the monetization assumption

**What was found.** Ranked by what kills the product if false, the assumptions are: whether anyone will pay for a transferable access right and whether anyone will seed for compensation; whether developers will adopt an alternative installer at all; and whether the crypto and distribution pipeline can be built. The MVP explicitly defers the first, largely assumes the second, and spends most of its effort on the third, which is the assumption nearest to already proven.

Deferring monetization for regulatory reasons is a deliberate and defensible trade, but the consequence is that a fully successful MVP tells us very little about whether the business works.

**Where.** `docs/MVP Scope.md`, In Scope item 4 and Deferred item 3.

**What resolving it would take.** A parallel means of gathering evidence for willingness to pay and seeder compensation that does not depend on shipping the paid tier — the shape of that evidence is undetermined.

## First-user value proposition

**What was found.** A developer adopting this gets a wallet they did not ask for, a gas relayer they must trust, a key provisioning dependency inside their install path, and public on-chain publication of their dependency graph. In exchange they get a global content-addressed store with symlinks into `node_modules`, which pnpm already provides for free with no new concepts.

What remains as genuine differentiation is resilience, meaning installs that survive registry outages, and swarm-accelerated fetches for popular packages. Both are real and both are modest against an incumbent that is faster today and requires no wallet. The honest pitch at $0.00 is resilience rather than ownership.

Related: for permissively-licensed packages, encryption adds latency, cost, and a liveness dependency while conferring nothing on the person installing. Its justification is that it exercises the pipeline the paid tier requires, which is a project-facing reason rather than a user-facing one.

**Where.** `docs/MVP Scope.md`, In Scope items 1 and 3.

**What resolving it would take.** A positioning decision on what the MVP claims to a first user, and a determination of whether encryption stays on the default install path or applies to a deliberate subset.

## First Finder ingestion consent and registry terms

**What was found.** Licenses generally permit the redistribution. The exposure is in the framing: encrypting someone else's package, registering it as canonical, and escrowing keys against their email hash reads as an ownership claim over work the protocol does not own.

**Where.** `docs/MVP Scope.md`, In Scope items 2 and 4; `docs/cryptography.md` Phase 1.1 and Phase 1.6.

**What resolving it would take.** Review of NPM's Terms of Service by counsel before ingesting at scale, and a decision on whether bulk seeding is run as a deliberate and publicly explained campaign rather than emerging as a side effect of user installs.

## MVP increment size

**What was found.** An alternative and thinner cut has been identified but not chosen: canonical identity registry, torrent distribution, and content-addressed store with symlinks, with encryption behind a flag applied to a small deliberate subset. That cut would validate the swarm and identity layer under real load with ordinary failure modes, still exercise the crypto pipeline end to end on a handful of packages, and avoid making key provisioning liveness a prerequisite for any developer's install succeeding.

The sequencer wrap model already removes part of the pressure that motivated this, since reads no longer contact a network, but the question of whether the first increment is still too large is open.

**Where.** `docs/MVP Scope.md`, the In Scope list in full.

**What resolving it would take.** A decision on the first increment's boundary, taken after the key custody item settles, since custody determines how much machinery the ingest path actually carries.
