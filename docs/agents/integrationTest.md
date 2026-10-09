# Integration test

Part of the [Tests](tests.md) topic; the [shared standards](tests.md#shared-standards-all-test-files) and [fixture rules](tests.md#fixtures-call-the-builder-directly) apply here.

Cited by: construction view (workplan node `integration.test` element) and implementation view (integrate prompt). Governed by all Process topics.

Exercises real code across an approved boundary (API, service, repository, external adapter). In TypeScript and Solidity this is a single test. In Rust it has a private tier and a public tier, described below; the standards in this section apply to both.

- Mock **only** at the outer boundary of the integrated scope. Use the **real** implementation for every function being integrated — never mocks, mock factories, or utilities for those.
- For an integration covering `f(x) → … → f(z)` where `f(x)` consumes `f(a)` and `f(z)` calls `f(b)`: mock `f(a)` and `f(b)`, run the real `f(x) … f(z)`.
- Integration fixtures are built from the mock file's builders (see [mocks](mocks.md)).

### The contract header here

This scope takes the **full four-field header and the inline markers** (see [tests](tests.md#every-test-states-its-contract)), plus two fields the other scopes do not have:

- **Boundary** — the approved boundary this test crosses, and the chain of real functions it runs.
- **Mocked** — what is mocked at the outer edge, and therefore what this test does **not** prove.

Naming what is mocked is what lets an audit catch a test that proves a mock instead of the integrated chain — an asserted value that originated in a mock and was only relayed is a tautological pass (see [tests](tests.md#audit)).

### A block transcribes an entry (Rust)

In Rust the `Contract` of every block transcribes one entry of the node's `interaction.spec.md` (see [workplan-structure](workplan-structure.md#the-interaction-spec-carries-the-integration-elements)), and `Arrange` names the input variation the entry declares. A block whose arrangement lacks that variation is incidental (see [tests](tests.md#audit)).

<a id="integration-private"></a>
## Private integration test (Rust)

The private integration test lives in the function's module directory as `integration_test.rs`, a `#[cfg(test)]` module of its own. It is never part of the unit test files: a block that runs more than its subject is an integration block and belongs here, and a unit test file holds no such block.

- It proves every own entry and every absorbed callee entry the spec places on its private surface, one block per entry.
- It runs the real chain through crate-internal paths and mocks only the outer-edge collaborators the private surface names. Fixtures come from the mock file's builders.
- In a family's factory module it proves every contract entry of the family's trait, each block's body written once and run over the declared set and the mock concrete (see [mocks](mocks.md#families--the-mock-is-a-concrete-of-the-family-rust)), so a further concrete answers every entry without an edit to the test.
- A function in the chain that is not built yet is a halt (see [discovery-halt](discovery-halt.md)).
- It reports its enumeration: every entry the private surface lists and the block that proves it. A file proving fewer entries than the spec lists is incomplete (see [scope](scope.md#coverage-canary)).

<a id="integration-public"></a>
## Public integration test (Rust)

The public integration test lives under the crate's `tests/` as `[module]_integration_test.rs`, hosted by a node whose boundary is a public entry (see [boundaries](boundaries.md#public-entries)). It proves the entries, own and carried, that the spec's public surface lists.

- It consumes only the crate's public surface and the official mocks under the `mocks` feature. The scenario is set up the way an outside caller reaches the behavior; when the behavior seems to need a private member, the scenario changes and the access never does.
- Its concretes come from the factory's declared set and configuration. Each block's body is written once and run over the declared set, so the test names no concrete and a new adapter adds a case without editing the test. Where the route receives a family's types, that body is the test's consumer (see [mocks](mocks.md#families--the-mock-is-a-concrete-of-the-family-rust)).
- Its `Boundary` is the public call and the route the entry carries, and its `Mocked` is the outer-edge collaborators on that route.
- A carried entry that no public call can observe is a discovery, not a gap to fill: the report names the entry, its route, and the consumer node whose disposition carried it, and proposes the correction to that disposition (see [discovery-halt](discovery-halt.md)). The test never drops the entry and never widens its access to reach it.
- A function in an entry's route that is not built yet is a halt.
- It reports its enumeration: every entry the public surface lists and the block that proves it (see [scope](scope.md#coverage-canary)).


