# Chain Prerequisites Handoff

## Topic

The planning documents, and the workplans built from them, do not state the application's real dependency order. The harness's later tickets consume the chain family, and the chain family's concrete and factory depend on work the planning documents place after the harness closes. This handoff corrects the planning documents so the order is the dependency order, and carries the work until the requirements for `chain/base` and `chain/factory` are delivered at ticket resolution. The Concrete Couplings Handoff (`docs/workplans/current/Concrete Couplings Handoff.md`) then resumes from its current point.

## The governing requirement

A ticket depends on another only when its file needs the other's file in order to be written (the dependency map's Overview). A family's nodes are authored once, complete, in the node of their first consumer, and where that consumer precedes the family's concretes in the work breakdown, the family's nodes are inserted ahead of it (`docs/agents/families.md`, "Ownership at the first consumer"). A caller reaches a concrete only through its family's factory (`families.md`, "The rule"). The order of the work is the order these produce; no ticket is reordered, deferred, split, or authored partially to avoid a dependency.

## The finding

- The harness tickets from `harness-crypto/verifier/evm` onward consume the chain's network values: the endpoint, the gas price oracle, the chain identifier, the entitlement contract, and the precompile encodings. Under the Concrete Couplings Handoff's problem "the chain's network values", these belong to the adapter that provides the chain and are reached through the chain family's factory. The harness therefore receives a chain concrete constructed by `chain/factory` through a chain consumer, so `harness-crypto/verifier/evm`, the configuration and run tickets, and every ticket after them depend on the chain family's interface and factory.
- `chain/factory` follows its concrete, `chain/base`, which is one concrete authored once, complete.
- The dependency map's Chain and submission families sprint row states the sprint's dependencies on the registry and entitlement contracts, `signature/factory`, and `discovery/factory`. At ticket resolution `chain/base`'s file needs the contracts it binds and their producers; `discovery/factory` is needed by `discovery/seeder_map`, which follows `chain/factory`, and no ticket on the path to `chain/factory` needs `signature/factory`. The registry and entitlement contracts depend on `contracts/evm/IDeliveryVerifier`; a Solidity contract imports no Rust file, so the domain model ticket on the path is the one `chain/base` needs.
- The dependency map placed these as sprints after the harness closes. The signature and swarm milestones still depend on nothing the harness measures and run beside it, since they are not on the path.
- The workplans followed the planning order: no nodes existed for the contracts `chain/base` binds, `chain/base`, or `chain/factory`, and the harness's verifier, configuration, and run nodes preceded none of them.

## The corrected order

The dependency map's columns give this order, verified under the first item of "Work".

- The harness tickets up to and including `harness-crypto/generate/factory` and `contracts/evm/IDeliveryVerifier`, unchanged. `contracts/evm/PairingLib`, `contracts/evm/DeliveryVerifier`, and `contracts/evm/deploy` depend only on these and on each other, and nothing on the path needs them; the generated directory, the deployed verifier, and the deployment records are runtime inputs and order nothing.
- The contract tickets `chain/base` binds and their producers: `contracts/evm/ISignatureVerifier`, `contracts/evm/IdentityAccount`, `contracts/evm/Binding`, `contracts/evm/IEntitlement`, `contracts/evm/IIdentityAdapter`, `contracts/evm/IAttestationVerifier`, `contracts/evm/ParameterSets`, and `contracts/evm/Registry`.
- `domain/handshake_key_commitment`, the domain type the attempt context `chain/base` authors carries.
- `chain/base`, then `chain/admit_chain_selection`, `chain/lookup_chain_declaration`, and `chain/admit_chain_composition`, then `chain/factory`.
- `harness-crypto/verifier/evm` and every harness ticket after it.
- The signature, transport, discovery, storage, and seed-host families are not on the path and keep their place.

## Work

Each step is one document or one decision. Planning documents are corrected in the pipeline order of `docs/workplans/correction-template.md`, each once, complete.

- Verify the order. Read the dependency map's tables, the Registry and entitlement contracts ownership list, the Remaining milestones sprint rows, the Sequencing section, and the Runtime sequence. For every ticket on the path to `chain/factory`, list what its file needs in order to be written, and confirm that none needs a harness ticket at or after `harness-crypto/verifier/evm`. A cycle, or a dependency the columns do not state, is a discovery: report it and halt.
- Map the path at ticket resolution. Under the dependency map's decay rule, work moving into the near span is mapped at ticket resolution. Map, as tickets with Owns, Depends on, and Proves, every domain-model ticket the registry and entitlement contracts need, every contract ticket of the registry and entitlement suite, `chain/base`, and `chain/factory`. Include only tickets on the path to `chain/factory`; everything else in those sprints stays a sprint where it is.
- `chain/base`'s ticket carries what the Concrete Couplings Handoff requires of the chain adapter. Its declaration states the endpoint, the gas price oracle, the chain identifier, the entitlement contract, and the precompile encodings, each fixed by the chain or settable within declared bounds with a default, and the concrete owns its settings type, its settings-document type and guard, and the admission of its own settings (`families.md`, "The declaration", "Selecting a concrete from configuration"). The chain family's selection admission and declaration lookup are family-owned tickets ahead of `chain/factory`, which constructs from the selection and calls them; the harness receives the network values from the constructed chain concrete through the chain factory's consumer (`families.md`, "Reading a declaration before construction").
- Correct `dependency-map.md`: place the newly mapped tickets ahead of `harness-crypto/verifier/evm`; add the edges from `harness-crypto/verifier/evm`, the configuration tickets, and `harness-crypto/run` and its consumers to the chain family; correct the Protocol core paragraph, the Components groupings' ordering statements, the Sequencing section, and every Conflict Flag that states the old order.
- Correct the other planning documents that state the order or the groupings: `milestones.md` (each moved milestone's position and Entry, and the Delivery proof and on-chain verifier milestone's scope and ordering paragraph), `master-plan.md`, `technical-requirements.md`, `system-architecture.md`, and any other document a search for the moved tickets' and milestones' names finds stating their position.
- Correct the workplans' order. The signature, transport, and discovery nodes keep their position. Each newly mapped ticket needs a node: authoring a new node follows the node-authoring procedure, not a correction, so stop at each insertion point, report the tickets it holds, and halt.
- Stop when `chain/base` and `chain/factory` are mapped at ticket resolution with every prerequisite ticket mapped and placed, and every planning document states the corrected order. Then return to the Concrete Couplings Handoff.

## Status

Complete. Every item under "Work" is done:

- The order is verified: no cycle, and no ticket on the path to `chain/factory` needs a harness ticket at or after `harness-crypto/verifier/evm`.
- The path is mapped at ticket resolution in `dependency-map.md`, with the ownership lists `contracts/evm/Registry ownership` and `chain/base ownership`, and the Conflict Flag "The harness consumes the chain family".
- `chain/base`'s ticket carries the Concrete Couplings requirements, with the user's decision on the fee: every chain has a fee object in the abstract whose concrete form its adapter determines, so the chain family describes a transaction's estimated fee, from the request its caller hands it and with nothing signed, sent, or settled, as the family's description, and the gas price oracle stays in Base's declaration and settings and reaches no family surface.
- Every planning document states the corrected order: `business-case-revised.md`, `feature-spec.md`, `technical-approach.md`, `dependency-map.md`, `product-requirements.md`, `system-architecture.md`, `technical-requirements.md`, `milestones.md`, and `master-plan.md`; `risk-register.md`, `non-functional-requirements.md`, and `tech-stack.md` state no order the correction changes.
- The workplans hold a placeholder for each of the fourteen tickets after `contracts/evm/IDeliveryVerifier` and before `harness-crypto/verifier/evm`; authoring them follows the node-authoring procedure.

## Return to the Concrete Couplings Handoff

On return, the Concrete Couplings Handoff's mapping still names `chain/network_profile_of` and `chain/base/admit_settings` in entries where they appear as dependencies. Both are withdrawn: `chain/network_profile_of` reached a concrete's settings without its factory, and `chain/base/admit_settings` existed only to put Base's settings ahead of a concrete that now comes first. Each entry naming either is corrected to the real dependency, `chain/factory` and the chain family's consumer surface for the network values, and `chain/base` for its settings admission. The corrections apply once this handoff has delivered the tickets those entries depend on.

## Standing instructions

- A point that needs input or a decision is explained and the agent halts. It is never worked around, and no option that breaks a rule is offered.
- Code on disk is not a source and is not opened. Findings are evidenced by quoted text and its location, never by line number.
- Each file is authored once, complete. Nothing is parked with an interim owner to be moved later.
- A node marked complete whose content changes returns to unbuilt.

## How to acquire the context

Read `docs/agents/index.md` and every Process topic it lists; `docs/agents/families.md`; `docs/agents/composition.md`, "Configuration follows ownership" and "Nested calls build their inputs from what they receive"; `docs/workplans/correction-template.md`; the dependency map in full; `milestones.md` from the Delivery proof and on-chain verifier milestone through the Chain and submission families milestone; and the Concrete Couplings Handoff's problem "the chain's network values" and its mapping.
