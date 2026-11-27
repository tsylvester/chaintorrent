# admit_envelope_selection interactions

`admit_envelope_selection` is the key-agreement family's selection admission: it receives the key-agreement member the selection configuration names and that member's untrusted settings document, refuses a document that supplies any setting with the family's own error naming the member — no key-agreement member has a settable value — and returns the selection that names the member. No member is constructed and no declaration is read.

## `admit_envelope_selection`

`admit_envelope_selection(deps: &AdmitEnvelopeSelectionDeps, params: AdmitEnvelopeSelectionParams, payload: serde_json::Value) -> AdmitEnvelopeSelectionReturn`

The payload is an untrusted document, and `deps` is fieldless and is not read. The decision is an exhaustive `match` on `params.concrete`, one arm per `KeyAgreementConcrete` variant, so a further concrete of the key-agreement family adds its own arm and a variant with no arm fails to compile. Each arm applies the document decision once on entry: `serde_json::Value::Null` or an empty `Value::Object` supplies no setting, and every other `serde_json::Value` supplies a setting.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| real member, no setting supplied | `params.concrete` is `KeyAgreementConcrete::PairingElGamal` and the payload is `Value::Null` or `Value::Object(map)` with `map.is_empty()` | the exhaustive `match` on `params.concrete` | none | `Ok(AdmitEnvelopeSelectionSuccessReturn { selection: KeyAgreementConcrete::PairingElGamal })` |
| real member, setting supplied | `params.concrete` is `KeyAgreementConcrete::PairingElGamal` and the payload is any other `serde_json::Value` | the same `match` | none | `Err(AdmitEnvelopeSelectionErrorReturn::SettingSupplied { concrete: KeyAgreementConcrete::PairingElGamal })` |
| mock member, no setting supplied | `params.concrete` is `KeyAgreementConcrete::Mock(failure_mode)` and the payload is `Value::Null` or `Value::Object(map)` with `map.is_empty()`, under `#[cfg(any(test, feature = "mocks"))]` | the same `match` on `params.concrete` | none | `Ok(AdmitEnvelopeSelectionSuccessReturn { selection: KeyAgreementConcrete::Mock(failure_mode) })`, the failure mode carried |
| mock member, setting supplied | `params.concrete` is `KeyAgreementConcrete::Mock(failure_mode)` and the payload is any other `serde_json::Value`, under `#[cfg(any(test, feature = "mocks"))]` | the same `match` | none | `Err(AdmitEnvelopeSelectionErrorReturn::SettingSupplied { concrete: KeyAgreementConcrete::Mock(failure_mode) })` |

Ordering and effects: the decision is the `match` on `params.concrete`; in the selected arm the document decision runs before any outcome is returned; the function constructs no adapter, reads no declaration, draws no bytes, encodes nothing, and has no side effect; an arm no input reaches is not an entry.

## Integration: own entries

- `every_admitted_key_agreement_selection_declares_the_member_it_names`: condition `admit_envelope_selection` called with each member of `KEY_AGREEMENT_CONCRETES` and a document that supplies no setting; outcome `Ok` whose `selection` equals the member the params carried; variation every member of `KEY_AGREEMENT_CONCRETES`, so an admission that returns one member's selection for another fails; edge that must survive the composition: the selection the admission returns is `params.concrete` unchanged, with no member constructed and no setting carried; route `admit_envelope_selection`; the route reaches no outer-edge collaborator.
- `every_declared_key_agreement_member_refuses_a_supplied_setting`: condition `admit_envelope_selection` called with each member of `KEY_AGREEMENT_CONCRETES` and a document that supplies a setting; outcome `Err(AdmitEnvelopeSelectionErrorReturn::SettingSupplied { concrete: member })`, and no selection returned; variation every member of `KEY_AGREEMENT_CONCRETES`, a document supplying a setting against one supplying none under which the admission returns `Ok`; edge that must survive the composition: the refusal names the member that was admitted and returns no selection; route `admit_envelope_selection`; the route reaches no outer-edge collaborator.

## Integration: public surface

An outside caller invokes `admit_envelope_selection` with `build_admit_envelope_selection_deps`, `build_admit_envelope_selection_params` carrying the member in `concrete`, and a `serde_json::Value` document written directly, observing the returned selection or refusal. The entries proven are `every_admitted_key_agreement_selection_declares_the_member_it_names` and `every_declared_key_agreement_member_refuses_a_supplied_setting`, each by the block of the same name in `admit_envelope_selection_integration_test.rs`; the fixtures are `KEY_AGREEMENT_CONCRETES`, `KeyAgreementConcreteOverrides`, `build_key_agreement_concrete`, `AdmitEnvelopeSelectionParamsOverrides`, `build_admit_envelope_selection_deps`, `build_admit_envelope_selection_params`, and `serde_json::Value`; the route is the public call.
