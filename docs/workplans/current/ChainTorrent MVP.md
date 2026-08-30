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

**What was found.** Key provisioning is to be expressed as an adapter so that the  DKMN implementation is an interface rather than an implementation detail that binds a specific architecture. This follows the reasoning behind the signature and identity adapters: the protocol intends its own chain and tokens eventually, with cross-chain and cross-token support, so every chain, curve, token, and provisioning touchpoint is abstracted from the start rather than retrofitted once a second implementation appears.

Authoring the interface is deferred rather than blocked. The custody and replay items both determine what operations the interface must expose, so naming them now would fix the abstraction around an incomplete picture.

**Where.** `docs/cryptography.md` §2, alongside `ISignatureAdapter` and `IIdentityAdapter`.

**What resolving it would take.** Naming the interface operations, then documenting a DKMN implementation as the MVP path.

## Collusion parameter selection

**What was found.** Variant sets are assigned using a collusion-resistant fingerprinting code providing provable tracing up to a chosen collusion size *c*. Variant-set size grows with *c* and with the accused-population size, so *c* is an economic parameter with no principled default. The cost curve has been assumed rather than measured.

Two related properties are settled rather than open, and are stated in the specification rather than held here: the MVP ships with no attribution at all, since wrapped keys stop casual sharing but not a determined leaker; and attribution is evidence rather than enforcement, since identifying a leaker does not recall content, making the response to a traced leak a governance and legal question.

**Where.** `docs/cryptography.md` §7.5; the eliminated §8.2.

**What resolving it would take.** Modelling *c* against asset value and expected adversary resources, and measuring the variant-size cost curve rather than assuming it. Neither is needed before per-entitlement variance ships.

## Variant object availability

**What was found.** A per-entitlement variant object has a swarm the size of one user's device set, so it may be unavailable when those devices are offline. Options: the variant set is made deterministically regenerable from the invariant content, the entitlement id, and a publisher secret; or the variant object can be shared into the swarm but is useless without possession of its specific NFT in the variant object possessor's wallet at the point of an attempted decryption. Both should be implemented so that users can colocate variants for one another and regenerate their own variant as needed. Regeneration inputs must be separated with the same discipline as content key derivation, or two entitlements collide onto the same variant set and attribution silently fails.


**Where.** `docs/cryptography.md` §7.3; the eliminated §8.2.

**What resolving it would take.** Specifying the regeneration derivation.
