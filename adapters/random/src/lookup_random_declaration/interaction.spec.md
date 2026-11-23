# `lookup_random_declaration` — interaction spec

Branch contract for the `lookup_random_declaration` module of the `random` crate: the randomness family's declaration lookup — a total function from a member identifier to that member's declaration, so a level that holds an identifier reads the source kind, the adapter version, and the interface version the member fixes, without constructing the member. Each branch states condition, decision, dependency call, and the exact return outcome.

## `lookup_random_declaration(deps: &LookupRandomDeclarationDeps, params: LookupRandomDeclarationParams, payload: LookupRandomDeclarationPayload) -> LookupRandomDeclarationReturn`

The trusted form: `deps` and `payload` are not read. Decision: an exhaustive `match` on `params.identifier`, one arm per `RandomSourceKind` variant, so a further member adds an arm.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| operating system | `params.identifier` is `RandomSourceKind::OperatingSystem` | the discriminant | none | `Ok(LookupRandomDeclarationSuccessReturn { declaration: OsRandomSource::DECLARATION })` |
| mock — `#[cfg(any(test, feature = "mocks"))]` | `params.identifier` is `RandomSourceKind::Mock(mode)` | the discriminant | none | `Ok(LookupRandomDeclarationSuccessReturn { declaration: RandomSourceDeclaration { source: params.identifier, ..MOCK_I_RANDOM_SOURCE_ADAPTER_DECLARATION } })`, the declaration naming the identifier looked up, the mode included, and carrying the versions the mock concrete's constant holds |

The error arm is uninhabited: `LookupRandomDeclarationReturn` is `Result<LookupRandomDeclarationSuccessReturn, Infallible>`, the identifier being a closed enum the `match` covers.

## Ordering and invariants

- The matched branch returns; no side effect — no clock, no environment variable, no network, no filesystem.

## Integration: callee dispositions

The interaction spec of `random/os` states no integration entry, so no entry passes through a function of this node or ends at one of its tests, this node calling none of the concrete's functions and reading `OsRandomSource::DECLARATION` alone.

## Integration: own entries

- `the_operating_system_source_is_looked_up_to_the_declaration_it_owns`: condition `lookup_random_declaration` is called over `RandomSourceKind::OperatingSystem` by a caller outside the crate; outcome `Ok` whose declaration's `source` is `RandomSourceKind::OperatingSystem`, whose `adapter_version` is the literal 1, and whose `interface_version` is `RANDOM_SOURCE_INTERFACE_VERSION`; variation the operating-system identifier, whose declaration names a source that differs from any `Mock` kind, so a lookup that returns the mock member's declaration fails; edge that must survive the composition: the declaration reaches the outside caller through this function alone, the member's constant being unnameable outside the crate.
- `the_mock_source_is_looked_up_to_a_declaration_naming_the_identifier_it_was_given`: condition `lookup_random_declaration` is called by a caller outside the crate under the `mocks` feature over `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)` and over the first identifier of `RANDOM_SOURCE_KINDS`; outcome `Ok` whose declaration's `source` is the mock identifier with its failure mode and which differs from the declaration the declared member returns; variation the mock identifier carrying a mode other than the mode the mock concrete's constant holds, beside a declared member's identifier, so a branch that returns the constant's own `source`, or the declared member's declaration, for the mock identifier fails; edge that must survive the composition: the mock member's declaration is its own and the declared set never holds it.

## Integration: private surface

The real function `lookup_random_declaration`, which calls no function; the observable result is the `LookupRandomDeclarationReturn`; the branches are proven by the unit blocks of `test.rs`, and the node carries no private integration test.

## Integration: public surface

The crate's `lib.rs` re-exports `lookup_random_declaration::provides`, so an outside caller invokes `lookup_random_declaration` over a member identifier. It observes the `LookupRandomDeclarationReturn`: the `Ok` arm's declaration through its public fields. The entries proven are `the_operating_system_source_is_looked_up_to_the_declaration_it_owns` and `the_mock_source_is_looked_up_to_a_declaration_naming_the_identifier_it_was_given`, along the route `lookup_random_declaration`, with no outer-edge collaborator.
