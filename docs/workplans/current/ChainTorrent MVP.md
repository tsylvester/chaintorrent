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

## Key provisioning adapter

**What was found.** Key provisioning is to be expressed as an adapter so that the DKMN implementation is an interface rather than an implementation detail that binds a specific architecture. This follows the reasoning behind the signature and identity adapters: the protocol intends its own chain and tokens eventually, with cross-chain and cross-token support, so every chain, curve, token, and provisioning touchpoint is abstracted from the start rather than retrofitted once a second implementation appears.

The adapter is now declared, and two obligations bind any implementation behind it rather than being properties of a particular construction: authorization is established per decryption window against finalized chain state, and provisioning for a valid unexpired entitlement is non-discretionary. Adapters may vary how authorization is established; they may never vary whether it is. What remains undetermined is the operation set itself, which the window model now largely determines — a request carrying an aggregate authorization payload, a response carrying a content key wrapped to the requesting identity and bounded by the window it was issued for.

**Where.** `docs/cryptography.md` §2, alongside `ISignatureAdapter`, `IIdentityAdapter`, and `IIngestSourceAdapter`.

**What resolving it would take.** Naming the interface operations against the window model, then documenting a DKMN implementation as the MVP path.

## Collusion parameter selection

**What was found.** Variant seeds are assigned using a collusion-resistant fingerprinting code providing provable tracing up to a chosen collusion size *c*. Codeword length grows with *c* and with the accused-population size, so *c* is an economic parameter with no principled default. The cost curve has been assumed rather than measured.

Because the variant object is a seed rather than an enumeration of replacement content, the cost of *c* is carried in codeword length rather than in object size, and is independent of asset size. That makes the parameter cheaper than it was under the enumerated model, but no less undetermined.

Three related properties are settled rather than open, and are stated in the specification rather than held here: the MVP ships with no attribution at all, since per-window provisioning stops casual sharing but not a determined leaker; attribution is evidence rather than enforcement, since identifying a leaker does not recall content, making the response to a traced leak a governance and legal question; and the fingerprint is cooperative rather than forensic, since the swarm object is complete and a modified client can render canonical plaintext carrying no fingerprint at all.

**Where.** `docs/cryptography.md` §7.5.

**What resolving it would take.** Modelling *c* against asset value and expected adversary resources, and measuring the codeword-length cost curve rather than assuming it. Neither is needed before per-entitlement variance ships.

## Variant object availability

**What was found.** Both halves are now specified rather than open. Seeding is unrestricted, because the variant object is encrypted to the entitlement identity and is therefore opaque to anyone without that entitlement in their wallet at the point of an attempted decryption — so any participant may colocate any other participant's variant object harmlessly, and reciprocal colocation gives every object a swarm larger than one user's device set. Reciprocal seeding additionally acts as a mixing mechanism, since a large seeder set means serving an infohash no longer implies owning the corresponding entitlement.

What remains is the regeneration path, which stands as the complement to colocation rather than as its alternative: a holder must be able to regenerate their own variant seed when no peer is serving it. Regeneration inputs must be domain-separated with the same discipline as content key derivation, or two entitlements collide onto the same variant seed and attribution silently fails.

**Where.** `docs/cryptography.md` §7.3.

**What resolving it would take.** Specifying the regeneration derivation and its domain separation. Blocked behind variant seed authorship, which determines what material regeneration operates on.

## Window bounds

**What was found.** Authorization is established for one bounded decryption window, expiring at a block-height ceiling `k` or a volume cap `M`, whichever is reached first. Both are published parameters and neither has a value.

They cannot be chosen independently of one another or of the transfer boundary. Widening `k` reduces provisioning load and lengthens the interval during which a seller retains capacity after settlement, so any capacity argument for widening the window is an argument for weakening the transfer boundary and has to be made as one. The load side is also unmeasured: authorization traffic scales with decryption activity across the whole swarm, and no baseline exists because nothing has run.

One related property is settled rather than open and is stated in the specification: the maximum transfer lag has two components, the window ceiling that the protocol sets and the target chain's finality that it does not, which makes the lag a per-deployment observable rather than a protocol constant.

**Where.** `docs/cryptography.md` §1 (Atomic and Bounded; Authorization is Per-Window), §3, §4 Phase 3.2, §8.1; `docs/MVP Scope.md` in-scope items 3 and 7.

**What resolving it would take.** Instrumenting authorization request volume and latency during the MVP to obtain a load baseline, then choosing `k` and `M` jointly against that baseline and against a stated maximum acceptable seller-retention interval. The economic side has no principled default and needs a declared tolerance rather than a derived number.

## Variant seed authorship

**What was found.** Variance is an overlay seed bound to the entitlement, but who authors the seed, from what material, and under what constraints is undetermined.

Two constraints shape any answer. First Finder escrow means the publisher is absent by definition, so no scheme requiring the publisher online at issuance or transfer can work at all; and a First Finder that authors seeds must escrow and discard the authoring material exactly as it does the content key, or a non-owner retains permanent framing capability over an asset they do not own. Separately, a seed can select among alternatives but cannot author them — if both the varied positions and their replacement values derive from the seed alone, the rendering is a corruption rather than a variant: a glitched frame, or a tarball that no longer installs. Semantically valid renderings require alternatives authored with knowledge of the content.

Whoever holds the authoring material can generate any holder's seed and therefore frame any holder. Siting that material with the party that already provisions content keys would add no capability that party does not already have, which is the cheapest available answer but inherits the provisioning trust concerns wholesale.

**Where.** `docs/cryptography.md` §7.2, §7.3, §8.3; `docs/MVP Scope.md` deferred item 7.

**What resolving it would take.** Choosing an authorship mechanism and specifying it behind the variance interface. Solutions exist in forensic watermarking; selecting one is beyond present scope, and because the implementation sits behind an interface the decision can be made later without disturbing the layers around it.

## Seeder compensation mechanism

**What was found.** The reciprocal colocation of variant objects and the swarm distribution the protocol depends on both presume that seeding is rewarded, and no mechanism is specified. Without one, contribution is voluntary, free-riding is rational, and aggregate resilience decays toward whatever altruism sustains.

The candidate mechanism is a reciprocal token pair in which one participant's upload token is another participant's download token, with a participant's ratio serving as the translation between them. This is out of MVP bounds because it depends on the token economics deferred alongside monetization, but it is a protocol-level gap rather than an implementation detail: the incentive structure determines whether the distribution model works at population scale at all.

**Where.** `docs/cryptography.md` §7.3, §8.4; `docs/MVP Scope.md` deferred item 8.

**What resolving it would take.** Specifying the token pair and the ratio translation, then deciding whether the mechanism is enforced or emergent. The specification currently states that reliability is emergent and that the protocol imposes no contractual obligation on third parties; a token accounting system may or may not preserve that, and which it does needs to be a decision rather than a side effect.

## Colocation multiplier

**What was found.** Variant object availability rests on participants setting aside a multiple of their own variant footprint to colocate other participants' objects, with resilience emerging from the incentive — greater aggregate colocation means greater resilience of the participant's own access — rather than from obligation. No default multiple is stated, and no replication target is stated either. "Emergent and probabilistic" is honest but gives an implementer no number to ship.

**Where.** `docs/cryptography.md` §7.3.

**What resolving it would take.** Choosing a default multiplier and stating the replication behaviour it is expected to produce, or deciding deliberately that the client ships with a user-set value and no protocol default. Not needed before per-entitlement variance ships.

## Scope of the authorization invariants

**What was found.** The Cryptographic Authorization Invariants are written as unconditional statements, but they bind the provisioning layer and conforming clients only. They cannot bind arbitrary software on a user-controlled device, and the protocol deliberately commits to plaintext running in any compatible software, which guarantees such software exists.

That scope is currently stated in §5.3 as a security consideration rather than in §1 where the invariants are declared, so a reader taking the invariant list on its own would overread its guarantees.

**Where.** `docs/cryptography.md` §1 Cryptographic Authorization Invariants; the scope statement presently in §5.3.

**What resolving it would take.** Deciding whether the invariant list carries its own scope sentence or whether the §5.3 statement suffices. This is a question of where the statement belongs, not whether it is true.
