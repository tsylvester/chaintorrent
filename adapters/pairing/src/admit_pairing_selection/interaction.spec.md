# `admit_pairing_selection` — interaction spec

Branch contract for the `admit_pairing_selection` module of the `pairing` crate: the family's selection admission, turning the `PairingConcrete` identifier admitted from configuration and the untrusted settings document into the selection the factory constructs from. Each branch states condition, decision, dependency call, and the exact return outcome.

## `admit_pairing_selection(deps: &AdmitPairingSelectionDeps, params: AdmitPairingSelectionParams, payload: serde_json::Value) -> AdmitPairingSelectionReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| `real member, no setting` | `params.identifier` is any real member of `PairingConcrete` and the payload is `Value::Null` or a `Value::Object` with no entry | the document supplies no setting, then the exhaustive `match` on `params.identifier` | none | `Ok(AdmitPairingSelectionSuccessReturn { selection })` where `selection` is `params.identifier` |
| `real member, setting supplied` | `params.identifier` is any real member and the payload is a `Value::Object` that holds an entry, or a `Value::Bool`, `Value::Number`, `Value::String`, or `Value::Array` | the document supplies a setting | none | `Err(AdmitPairingSelectionErrorReturn::SettingsSupplied { identifier })` where `identifier` is `params.identifier` |
| `mock, no setting` — present only under `#[cfg(any(test, feature = "mocks"))]` | `params.identifier` is `PairingConcrete::Mock(mode)` and the payload supplies no setting | the same `match` | none | `Ok(AdmitPairingSelectionSuccessReturn { selection })` where `selection` is `PairingConcrete::Mock(mode)` |
| `mock, setting supplied` — present only under `#[cfg(any(test, feature = "mocks"))]` | `params.identifier` is `PairingConcrete::Mock(mode)` and the payload supplies a setting | the document supplies a setting | none | `Err(AdmitPairingSelectionErrorReturn::SettingsSupplied { identifier })` where `identifier` is `PairingConcrete::Mock(mode)` |

## Ordering and edges

- Whether the document supplies a setting is decided once, from the payload alone, before the `match`, with no dependency call.
- The `match` on `params.identifier` is exhaustive with an arm for the real members and, under `#[cfg(any(test, feature = "mocks"))]`, an arm for the mock member, so a variant without an arm does not compile.
- `deps` carries no value and is not read.
- No concrete is constructed and no declaration is read.
- The error arm has one flavor, naming the member.
- A further concrete adds its name to the real-member arm in its own increment.

## Own entries

- `admission_returns_each_declared_member_for_a_document_without_a_setting`: condition `params.identifier` is any member of `PAIRING_CONCRETES` and the payload supplies no setting; outcome `Ok` whose `selection` is that member; input variation every member of the declared set, so an admission that returns one fixed member fails; edge that must survive composition: the selection is obtained through the crate's public surface with no concrete constructed.
- `admission_refuses_a_setting_for_each_declared_member`: condition `params.identifier` is any member of `PAIRING_CONCRETES` and the payload supplies a setting; outcome `Err` naming that member; input variation every member and a document that supplies a setting; edge: the refusal names the member the caller admitted.
- `mock_admission_names_the_identifier_admitted`, present only under `#[cfg(feature = "mocks")]`: condition `params.identifier` is `PairingConcrete::Mock(mode)` with a mode other than `Succeeds`; outcome `Ok` whose `selection` is `PairingConcrete::Mock(mode)` for a document without a setting and `Err` naming `PairingConcrete::Mock(mode)` for a document with one; input variation the mode; edge: the mock member is absent from `PAIRING_CONCRETES` and reached only by naming it.
- `admission_returns_each_declared_member_for_a_null_document`: condition `params.identifier` is any member of `PAIRING_CONCRETES` and the payload is `Value::Null`; outcome `Ok` whose `selection` is that member; input variation every member and the absent document, so an admission that treats only an empty object as no setting fails; edge: the absent document is admitted through the public surface.
- `admission_refuses_each_non_object_document_for_each_declared_member`: condition `params.identifier` is any member of `PAIRING_CONCRETES` and the payload is `Value::Bool`, `Value::Number`, `Value::String`, or `Value::Array`; outcome `Err` naming that member; input variation every member and each of the four scalar and array documents, each falsy or empty, so an admission that treats a falsy or empty value as absent fails; edge: each refusal names the member the caller admitted.

## Callee dispositions

No function of this node calls a dependency, and the entries of the declaration lookup and of the concretes this node's dependencies authored neither pass through a function of this node nor end at a test of this node, so no entry is absorbed or carried.

## Route

`admit_pairing_selection` reads the payload and the identifier alone; no real function of another module and no outer-edge collaborator is on the route.

## Public surface

An outside caller invokes `admit_pairing_selection` with fieldless deps, params carrying a `PairingConcrete`, and a `serde_json::Value` document, deps and params built through the official mocks and the document written through `Value`'s own variants, and observes the `Ok` selection or the `Err` refusal. The entries `admission_returns_each_declared_member_for_a_document_without_a_setting`, `admission_returns_each_declared_member_for_a_null_document`, `admission_refuses_a_setting_for_each_declared_member`, `admission_refuses_each_non_object_document_for_each_declared_member`, and `mock_admission_names_the_identifier_admitted` are proven by the public integration test.
