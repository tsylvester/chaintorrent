# lookup_hash_to_scalar_declaration interactions

## `lookup_hash_to_scalar_declaration`

`lookup_hash_to_scalar_declaration(deps: &LookupHashToScalarDeclarationDeps, params: LookupHashToScalarDeclarationParams, payload: LookupHashToScalarDeclarationPayload) -> LookupHashToScalarDeclarationReturn`

The decision is an exhaustive `match` on `params.concrete`, one arm per `HashToScalarConcrete` variant; `deps` and `payload` carry nothing and are not read.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| keccak256 | `params.concrete` is `HashToScalarConcrete::Keccak256` | the `match` arm | none; the arm reads the associated constant `Keccak256HashToScalar::DECLARATION` | `Ok(LookupHashToScalarDeclarationSuccessReturn { declaration })`, `declaration` that constant |
| mock | `params.concrete` is `HashToScalarConcrete::Mock(mode)`, under `#[cfg(any(test, feature = "mocks"))]` | the `match` arm | none; the arm reads the constant `MOCK_I_HASH_TO_SCALAR_ADAPTER_DECLARATION` | `Ok(LookupHashToScalarDeclarationSuccessReturn { declaration })`, `declaration.concrete` equal to `HashToScalarConcrete::Mock(mode)` and `declaration.identifier`, `declaration.adapter_version`, `declaration.interface_version` taken from that constant |

The error arm has no branch that returns it; `LookupHashToScalarDeclarationErrorReturn` has no variant.

Ordering and side effects: none; the function reads constants, constructs no adapter, and touches no operating-system facility.

## Integration: own entries

- `every_declared_member_declares_itself_through_the_public_surface`: condition an outside caller invokes the lookup through the crate's public surface with each member of `HASH_TO_SCALAR_CONCRETES`; outcome `Ok` whose `declaration.interface_version` is `HASH_TO_SCALAR_INTERFACE_VERSION`; variation every member of the declared set; edge that must survive: no adapter, no scalar type, and no pairing is in scope at the call, so the declaration is read from the family's private path with nothing constructed.
- `a_mock_member_declares_the_member_selected_through_the_public_surface`: condition an outside caller invokes the lookup through the crate's public surface under the `mocks` feature with `HashToScalarConcrete::Mock(mode)` for a `mode` other than the `Succeeds` mode `MOCK_I_HASH_TO_SCALAR_ADAPTER_DECLARATION` names; outcome `Ok` whose `declaration.concrete` is `HashToScalarConcrete::Mock(mode)` and whose `declaration.interface_version` is `HASH_TO_SCALAR_INTERFACE_VERSION`; variation `SamplingRefused` against the constant's `Succeeds`; edge that must survive: the substituted identifier reaches the outside caller, so a lookup that returned the constant unchanged yields the `Succeeds` member.

## Integration: public surface

An outside caller invokes `lookup_hash_to_scalar_declaration` with `LookupHashToScalarDeclarationDeps`, `LookupHashToScalarDeclarationParams { concrete }`, and `LookupHashToScalarDeclarationPayload`, and observes `LookupHashToScalarDeclarationSuccessReturn { declaration }`; the entries the public integration test proves are `every_declared_member_declares_itself_through_the_public_surface` and `a_mock_member_declares_the_member_selected_through_the_public_surface`.
