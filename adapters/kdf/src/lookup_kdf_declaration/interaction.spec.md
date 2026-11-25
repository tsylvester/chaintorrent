# lookup_kdf_declaration interactions

`lookup_kdf_declaration` is the key-derivation family's declaration lookup: it reads the `KdfDeclaration` of the member `params.concrete` names, from that member's interface-held declaration constant, with no member constructed. A caller that holds an admitted member identifier learns what the member declares — its typed member identity, the `KdfIdentifier` a deployment's hash-card carries, its adapter version, and its interface version — before any construction, and no caller restates a member's declared value from its own knowledge of the member.

## `lookup_kdf_declaration`

`lookup_kdf_declaration(deps: &LookupKdfDeclarationDeps, params: LookupKdfDeclarationParams, payload: LookupKdfDeclarationPayload) -> LookupKdfDeclarationReturn`

The decision is a `match` on `params.concrete`, one arm per `KdfConcrete` variant, exhaustive so a variant with no arm fails to compile and a further concrete of the key-derivation family adds its own arm.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| blake3_keyed | `params.concrete` is `KdfConcrete::Blake3Keyed` | the exhaustive `match` on `params.concrete` | none; the arm reads the constant `Blake3KeyedKdf::DECLARATION` and `Blake3KeyedKdf::try_new` is not called | `Ok(LookupKdfDeclarationSuccessReturn { declaration })`, `declaration` equal to `Blake3KeyedKdf::DECLARATION` |
| mock | `params.concrete` is `KdfConcrete::Mock(failure_mode)`, under `#[cfg(any(test, feature = "mocks"))]` | the same `match` | none; the arm reads the constant `MOCK_I_KEY_DERIVATION_ADAPTER_DECLARATION` | `Ok(LookupKdfDeclarationSuccessReturn { declaration })`, `declaration.concrete` the selected `KdfConcrete::Mock(failure_mode)` and `declaration.identifier`, `declaration.adapter_version`, `declaration.interface_version` the fields of `MOCK_I_KEY_DERIVATION_ADAPTER_DECLARATION` |

The error arm `LookupKdfDeclarationErrorReturn` has no variant, so no branch returns `Err`.

Ordering and effects: the `match` is the whole body; no member is constructed, nothing is read from configuration, and the same params always yield the same declaration.

## Integration: callee dispositions

None. The function calls no collaborator; each arm reads a constant, which the unit tests prove.

## Integration: own entries

- `lookup_kdf_declaration_is_reachable_through_the_crate_surface_without_a_constructed_member`: condition a caller outside the crate calls `lookup_kdf_declaration` with the member identifier of each member of `KDF_CONCRETES` and with no key-derivation member constructed anywhere in the call; outcome `Ok` carrying a declaration whose `identifier` is a member of `KDF_IDENTIFIERS`, whose `adapter_version` is `1`, and whose `interface_version` is `1`; variation the call crosses the crate boundary through the re-exported surface alone, with no member and no factory involved; the failure that must survive composition is the function or its types being absent from the crate's re-exported surface, which no unit test inside the module observes.
- `lookup_kdf_declaration_carries_the_selected_mock_member_through_the_crate_surface`: condition a caller outside the crate, under the `mocks` feature, calls `lookup_kdf_declaration` with `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable)`; outcome `Ok` carrying a declaration whose `concrete` is that selected mock member, whose `identifier` is `KdfIdentifier::Blake3KeyedV1`, whose `adapter_version` is `1`, and whose `interface_version` is `1`; variation the selected failure mode differs from the `NoFailure` member the mock constant names, so a lookup that returns the mock constant unchanged differs from a lookup that carries the selection; the failure that must survive composition is the mock member or its failure mode being absent from the surface the `mocks` feature exposes.

Route: the chain of real functions is the crate's re-exported `lookup_kdf_declaration`, then the `match` on the member identifier, then, for a declared member, the read of the member's interface-held declaration through the family's private path, and, for the mock member, the read of `MOCK_I_KEY_DERIVATION_ADAPTER_DECLARATION`; the outer-edge collaborators are none.

## Integration: private surface

None. The function composes no collaborator beyond the constants it reads, which the unit tests prove, so the module has no `integration_test.rs`.

## Integration: public surface

An outside caller invokes `lookup_kdf_declaration` through the `kdf` crate's re-exports, with `LookupKdfDeclarationDeps`, `LookupKdfDeclarationParams` carrying a member of `KDF_CONCRETES` built from `build_kdf_concrete` or, under the `mocks` feature, a mock member, and `LookupKdfDeclarationPayload`; it observes the success return's `declaration`. The entries the public integration test proves are `lookup_kdf_declaration_is_reachable_through_the_crate_surface_without_a_constructed_member` and `lookup_kdf_declaration_carries_the_selected_mock_member_through_the_crate_surface`.
