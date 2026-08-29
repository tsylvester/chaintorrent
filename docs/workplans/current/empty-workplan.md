`[ ]`    // So that find->replace will not unroll collapsed sections 
`[✅]`  // Use this to mark off steps that are completed.  

# **TITLE**

## Problem Statement

## Objectives

## Expected Outcome

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

## Name of deferred work item
