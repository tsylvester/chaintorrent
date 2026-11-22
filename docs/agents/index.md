# Agent Instruction Index

Single source of truth for how agents work in this repo. Every rule lives in exactly one topic file below (DRY / SSOT / SRP). Nothing here is duplicated into a harness's native path — each harness entry file (`CLAUDE.md`, `AGENTS.md`, `.github/copilot-instructions.md`, `.cursor/rules/rules.mdc`) is a thin pointer to this index.

## How to use this index

- **Process topics** govern every turn regardless of which file you touch. Read them first, every turn.
- **Standards topics** govern the content of one file/element. A workplan node element and its implementer prompt cite the **same** standards topic(s) — the two views never diverge.

## Process topics — govern every turn

- [precedence](precedence.md) — authority order, single-turn completeness, rejection-attribution
- [scope](scope.md) — what a scope is; drawing, respecting, and stating it in full
- [loop](loop.md) — Read → Analyze → Explain → Propose → (Edit → Lint) → Halt; one file per turn
- [discovery-halt](discovery-halt.md) — multi-file / underspecified → report, propose node, halt
- [modes](modes.md) — Builder vs Reviewer; EO&D review
- [traceability](traceability.md) — model signature, required report elements, canary mechanism
- [output](output.md) — no code in chat, no unbidden writes, safe edit boundaries
- [fidelity](fidelity.md) — no whole-file rewrites, preserve behavior on refactor, no unrequested renames
- [environment](environment.md) — tools / MCPs, language-server, worktrees, never run tests
- [terminal-policy](terminal-policy.md) — terminal denied by default; two-gate allowlist; per-env enforcement
- [linting-proof](linting-proof.md) — lint after edit, no silencing, linter error = RED proof
- [logging](logging.md) — debug by logging, believe failures literally
- [insanity-loop](insanity-loop.md) — break repetitive failure and recover by starting over
- [line-length](line-length.md) — do not wrap lines, do not write long lines 

## Standards topics — govern file content

- [tdd-ordering](tdd-ordering.md) — TDD cycle + bottom-up dependency order
- [workplan-structure](workplan-structure.md) — node anatomy, one file per node
- [types](types.md) — strict typing, narrowest type, `unknown` only at a boundary
- [composition](composition.md) — function signature shape (deps / params / payload / return union)
- [dependency-injection](dependency-injection.md) — DI at the boundary, context factory
- [families](families.md) — adapter families in production code: factory, declaration, selection and its admission, declared set, compatibility admission, family surface, concretes unreachable outside their family
- [mocks](mocks.md) — builders, invalidators, function mocks, values the mock cannot construct, family mock concretes and their failures
- [guards](guards.md) — guard everything, authorship, boundary litmus
- [tests](tests.md) — test authoring, contract headers, derangement audit; anchors `#interface` `#guard` `#unit` `#integration` `#integration-private` `#integration-public` `#e2e` `#audit`
- [errors-and-returns](errors-and-returns.md) — `Success | Error` handling
- [boundaries](boundaries.md) — provides / barrels, directionality

## Views

- **Construction view** (workplan author): the [workplan-structure](workplan-structure.md) node template names, per element, the standards topic(s) that element must conform to.
- **Implementation view** (file author): each element prompt points at the same standards topic(s) as its node element, plus all Process topics.

## Implementation routing

A map from the file you are building to the topic that governs it, the template or example to copy, and its most common halt trigger. Every element is also governed by all Process topics. [workplan-structure](workplan-structure.md#element-files-by-language) maps the elements to their files per language; copy the form of the language you are building in.

| Building | Conforms to | Copy from | Halt if |
|---|---|---|---|
| interface test (TypeScript, Solidity) | [tests](tests.md#interface), [composition](composition.md), [types](types.md), [errors-and-returns](errors-and-returns.md) | typed-assignment example ([tests#interface](tests.md#interface)) | a type it needs lives in another interface that does not exist yet |
| interface | [composition](composition.md), [types](types.md), [errors-and-returns](errors-and-returns.md), [dependency-injection](dependency-injection.md), [families](families.md) | signature template ([composition](composition.md)); a family's identifier, selection, declaration, and declared set ([families](families.md#what-a-family-owns)); a family's description ([families](families.md#a-familys-surface-carries-no-concretes-model)); the consumer-generic return template ([composition](composition.md#a-return-that-carries-the-callers-type-rust)); the case runner's types ([mocks](mocks.md#the-case-runner--composing-families-for-a-subject-rust)) | a required type has no locatable definition, or the type needs a concrete's name, field, or vocabulary outside its family |
| mock (TypeScript) | [mocks](mocks.md), [families](families.md) | builder / invalidator / function-mock templates ([mocks](mocks.md)); the family's interface builder, the factory's function mock, and the declared-selection builder ([mocks](mocks.md#families--the-typescript-form)) | an imported type's mock is missing from its home package, or the family's interface builder is missing from the factory module's mock file |
| mock (Rust) | [mocks](mocks.md), [families](families.md) | builder / invalidator / function-mock templates ([mocks](mocks.md)); mock concrete and declared-selection builder ([mocks](mocks.md#families--the-mock-is-a-concrete-of-the-family-rust)); handed, produced, consumer-invoking, and independent forms ([mocks](mocks.md#fields-with-no-default-rust), [mocks](mocks.md#a-return-that-carries-the-consumers-type-rust)); type parameters and configuration ([composition](composition.md#type-parameters-follow-dependency-rust), [composition](composition.md#configuration-follows-ownership-rust)); the case runner ([mocks](mocks.md#the-case-runner--composing-families-for-a-subject-rust)) and inputs by what a level receives ([composition](composition.md#nested-calls-build-their-inputs-from-what-they-receive-rust)) | an imported type's mock, or a composed family's consumer trait or params builder, is missing from its home package, or the family's selection lacks its mock concrete |
| mock (Solidity) | [mocks](mocks.md#the-solidity-form) | builder and interface-mock templates ([mocks](mocks.md#the-solidity-form)) | an imported struct's builder or an imported interface's mock is missing from its home mock file |
| guard test (TypeScript, Solidity) | [tests](tests.md#guard), [guards](guards.md), [mocks](mocks.md) | case-checklist example ([tests#guard](tests.md#guard)) | a needed builder or invalidator is absent from its mock file |
| guard (TypeScript, Solidity) | [guards](guards.md) | guard skeleton + forbidden substitutes ([guards](guards.md)) | an imported type's guard is absent after the predicate search |
| unit test | [tests](tests.md#unit), [errors-and-returns](errors-and-returns.md), [composition](composition.md) | no positive template — the branch contract ([interaction spec](workplan-structure.md#the-interaction-spec-is-a-branch-contract)) and the forbidden catalog | the implementation's producer does not exist yet, or a family collaborator's mock concrete is not selectable through its factory |
| implementation | [composition](composition.md), [dependency-injection](dependency-injection.md), [types](types.md), [errors-and-returns](errors-and-returns.md), [guards](guards.md), [logging](logging.md), [families](families.md) | guard-on-entry skeleton ([guards](guards.md)); body from the branch contract | an undeclared dependency or a missing guard is required, or the body needs a concrete outside its family |
| provides | [boundaries](boundaries.md), [families](families.md) | export-surface example ([boundaries](boundaries.md)); a family module's surface ([families](families.md#concretes-are-unreachable-outside-their-family)) | a symbol a consumer needs is not available to export, or a consumer outside the family needs a concrete's symbol |
| integration test (TypeScript, Solidity) | [tests](tests.md#integration) | boundary-mocking example ([tests#integration](tests.md#integration)) | a function in the integrated chain is not built yet |
| private integration test (Rust) | [tests](tests.md#integration-private), [workplan-structure](workplan-structure.md#the-interaction-spec-carries-the-integration-elements) | the spec's private surface and the entries it lists | a function in the integrated chain is not built yet |
| public integration test (Rust) | [tests](tests.md#integration-public), [boundaries](boundaries.md#public-entries), [workplan-structure](workplan-structure.md#the-interaction-spec-carries-the-integration-elements) | the spec's public surface and the entries it lists | a carried entry cannot be observed through the public API, or a function in its route is not built yet |

A file whose module plays a role in a family is governed by its element's row above and by its role's row below.

| Role | Conforms to | Copy from | Halt if |
|---|---|---|---|
| a family's factory | [families](families.md), [composition](composition.md#configuration-follows-ownership-rust), [errors-and-returns](errors-and-returns.md#a-familys-errors), [mocks](mocks.md#families--the-mock-is-a-concrete-of-the-family-rust) | the family-level types ([families](families.md#what-a-family-owns)); the compatibility refusal ([families](families.md#compatibility-is-admitted-before-construction-and-before-effort)) | a member's declaration lacks a requirement the composition admission must compare, or the surface needs one member's model |
| a concrete | [families](families.md), [composition](composition.md#configuration-follows-ownership-rust), [errors-and-returns](errors-and-returns.md#a-familys-errors) | the declaration ([families](families.md#the-declaration)); the settings admission ([families](families.md#selecting-a-concrete-from-configuration)); the settings ordering ([families](families.md#a-concretes-settings-precede-the-concrete)) | a setting has no bounds or default in its declaration, or the concrete is constructed from a value configuration sets and has no settings module |
| a family-owned function | [families](families.md), [composition](composition.md) | the selection admission ([families](families.md#selecting-a-concrete-from-configuration)); the declaration lookup ([families](families.md#reading-a-declaration-before-construction)); the composition admission ([families](families.md#compatibility-is-admitted-before-construction-and-before-effort)) | it needs a concrete outside its family's private path |
| a family's consumer | [families](families.md), [composition](composition.md#nested-calls-build-their-inputs-from-what-they-receive-rust) | the consumer-generic return ([composition](composition.md#a-return-that-carries-the-callers-type-rust)); the family's description ([families](families.md#a-familys-surface-carries-no-concretes-model)) | it needs a concrete's name, field, or error variant |
| a configuration admission | [families](families.md), [composition](composition.md#parameter-jurisdiction--what-is-trusted-vs-proven), [guards](guards.md) | the selection admission and the settings table ([families](families.md#selecting-a-concrete-from-configuration)) | it needs a concrete's type, constant, constructor, or setting key |
| a composition root | [families](families.md), [dependency-injection](dependency-injection.md) | the composition admission ([families](families.md#compatibility-is-admitted-before-construction-and-before-effort)) | a side effect precedes the admission of a member it composes |


