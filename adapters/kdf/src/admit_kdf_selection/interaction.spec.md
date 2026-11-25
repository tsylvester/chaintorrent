# admit_kdf_selection interactions

`admit_kdf_selection` is the key-derivation family's selection admission: it narrows the untrusted settings document that configuration may carry beside the member identifier, refuses a document that supplies any setting with the family's own error naming the member — no key-derivation member has a settable value — and returns the selection that names the member alone. No member is constructed and nothing is read from configuration.

## `admit_kdf_selection`

`admit_kdf_selection(deps: &AdmitKdfSelectionDeps, params: AdmitKdfSelectionParams, payload: serde_json::Value) -> AdmitKdfSelectionReturn`

The function computes the document decision once on `payload`: `serde_json::Value::Null` or an empty `Value::Object` supplies no setting, and every other `serde_json::Value` supplies a setting. The second decision is an exhaustive `match` on `params.concrete`, each arm applying the document decision, one arm per `KdfConcrete` variant, so a further concrete of the key-derivation family adds its own arm and a variant with no arm fails to compile.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| real member | `params.concrete` is `KdfConcrete::Blake3Keyed` | the exhaustive `match` on `params.concrete`, each arm applying the document decision | none | `Ok(AdmitKdfSelectionSuccessReturn { selection: KdfConcrete::Blake3Keyed })` when the document supplies no setting, and `Err(AdmitKdfSelectionErrorReturn::SettingSupplied { concrete: KdfConcrete::Blake3Keyed })` when it does |
| mock member | `params.concrete` is `KdfConcrete::Mock(failure_mode)`, under `#[cfg(any(test, feature = "mocks"))]` | the same `match` and the same document decision | none | `Ok(AdmitKdfSelectionSuccessReturn { selection: KdfConcrete::Mock(failure_mode) })` when the document supplies no setting, and `Err(AdmitKdfSelectionErrorReturn::SettingSupplied { concrete: KdfConcrete::Mock(failure_mode) })` when it does |
| supplied setting | the document is neither `Value::Null` nor an empty `Value::Object` — a nonempty object, a scalar, an array, a boolean, a string, or a number | the document decision | none | `Err(AdmitKdfSelectionErrorReturn::SettingSupplied { concrete: params.concrete })`, the sole refusal |

Ordering and effects: the document decision is computed once before the `match`; no member is constructed, nothing is read from configuration, and the same params and document always yield the same outcome.

## Integration: callee dispositions

None. No entry of `kdf/blake3_keyed` or `kdf/lookup_kdf_declaration` passes through one of this node's functions or ends at one of its tests, since this node calls neither `derive_key` nor `lookup_kdf_declaration`.

## Integration: own entries

- `admit_kdf_selection_returns_the_selection_naming_each_declared_member_through_the_crate_surface`: condition a caller outside the crate calls `admit_kdf_selection` with the member identifier of each member of `KDF_CONCRETES` and an empty settings document with no key-derivation member constructed anywhere in the call; outcome `Ok` whose `selection` is that member; variation the call crosses the crate boundary through the re-exported surface alone, with no member and no factory involved; the failure that must survive composition is the function or its types being absent from the crate's re-exported surface, which no unit test inside the module observes.
- `admit_kdf_selection_carries_the_selected_mock_member_through_the_crate_surface`: condition a caller outside the crate, under the `mocks` feature, calls `admit_kdf_selection` with `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable)` and an empty settings document; outcome `Ok` whose `selection` is that selected mock member; variation the selected failure mode differs from `NoFailure`, so a selection that substitutes a default mock member differs from one that carries the selection; the failure that must survive composition is the mock member or its failure mode being absent from the surface the `mocks` feature exposes.
- `admit_kdf_selection_refuses_a_supplied_setting_naming_each_declared_member_through_the_crate_surface`: condition a caller outside the crate calls `admit_kdf_selection` with the member identifier of each member of `KDF_CONCRETES` and a settings document that is a table holding one key; outcome `Err(AdmitKdfSelectionErrorReturn::SettingSupplied { concrete })` with `concrete` that member; variation the document supplies a setting where the first entry's document supplies none; the failure that must survive composition is the error type or its member-valued field being absent from the crate's re-exported surface.
- `admit_kdf_selection_refuses_a_supplied_setting_naming_the_selected_mock_member_through_the_crate_surface`: condition a caller outside the crate, under the `mocks` feature, calls `admit_kdf_selection` with `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable)` and a settings document that is a table holding one key; outcome `Err(AdmitKdfSelectionErrorReturn::SettingSupplied { concrete })` with `concrete` that selected mock member; variation the selected failure mode differs from `NoFailure`; the failure that must survive composition is the refusal naming a default mock member in place of the selected one.
- `admit_kdf_selection_refuses_a_non_object_document_as_a_supplied_setting_through_the_crate_surface`: condition a caller outside the crate calls `admit_kdf_selection` with a settings document that is the JSON number `7`; outcome `Err(AdmitKdfSelectionErrorReturn::SettingSupplied { concrete })` with `concrete` that member, since a value that is neither `Value::Null` nor an empty `Value::Object` supplies a setting; variation the document is a scalar where the earlier entries' documents are null, an empty object, or a nonempty object; the failure that must survive composition is the refusal type or its member-valued field being absent from the crate's re-exported surface.
- `admit_kdf_selection_returns_the_selection_for_an_absent_document_through_the_crate_surface`: condition a caller outside the crate calls `admit_kdf_selection` with the member identifier of each member of `KDF_CONCRETES` and the JSON null as the settings document; outcome `Ok` whose `selection` is that member; variation the document is absent where the first entry's document is an empty table; the failure that must survive composition is the null document being refused as a non-table.

Route, for every own entry: the chain of real functions is the crate's re-exported `admit_kdf_selection` and the `match` on the member identifier under its document decision; the outer-edge collaborators are none.

## Integration: private surface

None; every branch is proven by this node's unit tests.

## Integration: public surface

An outside caller invokes `admit_kdf_selection` through the `kdf` crate's re-exports, with `AdmitKdfSelectionDeps`, `AdmitKdfSelectionParams` carrying a member of `KDF_CONCRETES` built from `build_kdf_concrete` or, under the `mocks` feature, a mock member, and the untrusted document as the payload; it observes the success return's `selection` and the error return. The entries the public integration test proves are `admit_kdf_selection_returns_the_selection_naming_each_declared_member_through_the_crate_surface`, `admit_kdf_selection_carries_the_selected_mock_member_through_the_crate_surface`, `admit_kdf_selection_refuses_a_supplied_setting_naming_each_declared_member_through_the_crate_surface`, `admit_kdf_selection_refuses_a_supplied_setting_naming_the_selected_mock_member_through_the_crate_surface`, `admit_kdf_selection_refuses_a_non_object_document_as_a_supplied_setting_through_the_crate_surface`, and `admit_kdf_selection_returns_the_selection_for_an_absent_document_through_the_crate_surface`, each by the block of the same name.
