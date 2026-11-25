# `lookup_encoding_declaration` — interaction spec

Branch contract for the `lookup_encoding_declaration` module of the `encoding` crate: `lookup_encoding_declaration`, the encoding family's declaration lookup, returning a member's declaration from its identifier without constructing the member. Each branch states condition, decision, dependency call, and the exact return outcome.

## `lookup_encoding_declaration(deps: &LookupEncodingDeclarationDeps, params: LookupEncodingDeclarationParams, payload: LookupEncodingDeclarationPayload) -> LookupEncodingDeclarationReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| `Abi` member | `params.concrete` is `EncodingConcrete::Abi` | an exhaustive `match` on `params.concrete` with no wildcard arm, so a member with no arm fails to compile | none, the arm reads the constant `AbiEncoding::DECLARATION` and calls no constructor | `Ok(LookupEncodingDeclarationSuccessReturn { declaration: AbiEncoding::DECLARATION })` |
| `Mock` member | `params.concrete` is `EncodingConcrete::Mock`, an arm compiled under `#[cfg(any(test, feature = "mocks"))]` alone | the same `match` | none, the arm reads the constant `MOCK_I_ENCODER_ADAPTER_DECLARATION` | `Ok(LookupEncodingDeclarationSuccessReturn { declaration: MOCK_I_ENCODER_ADAPTER_DECLARATION })` |

The error arm has no branch: every member identifier has a declaration, so `Result`'s `Err` carries `Infallible`.

## Ordering

The function reads `params.concrete`; `deps` and `payload` hold no field and are not read; no adapter is constructed, no file is touched, and no side effect occurs.

## Integration: own entries and callee dispositions

Callee dispositions: none; the function's callees are the constants `AbiEncoding::DECLARATION` and `MOCK_I_ENCODER_ADAPTER_DECLARATION`, and the specs of `encoding/abi` and `encoding/canonical_field_reader` declare no entry.

Own entry `declared_member_declaration_through_the_public_surface`: an outside caller that holds a member of `ENCODING_CONCRETES` and calls `lookup_encoding_declaration` through the crate's public surface reads that member's own declaration, whose `concrete` is the member passed, whose `identifier` is a member of `ENCODING_IDENTIFIERS`, and whose `interface_version` is `ENCODING_INTERFACE_VERSION`; the input variation that separates a pass from a fail is each member of the declared set, so a lookup that returns another member's declaration, or one for a member the barrel does not export, fails; the edge that must survive composition is that no adapter is constructed and no concrete's name, type, or constructor is reachable from outside the crate.

Private surface: none; every branch is proven by this node's unit tests.

Public surface: the outside caller invokes `lookup_encoding_declaration` from the crate root with builder-made deps, params, and payload, where the params' `concrete` is each member of `ENCODING_CONCRETES`, and observes the returned declaration; the entry the public integration test proves is `declared_member_declaration_through_the_public_surface`; the route is the crate barrel, then `lookup_encoding_declaration`, ending at the member's declaration constant; the outer-edge collaborators are none.
