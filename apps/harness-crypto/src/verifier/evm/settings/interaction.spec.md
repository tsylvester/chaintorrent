# `settings` — interaction spec

Branch contract for the `settings` module of the `evm` module of `apps/harness-crypto`: `settings` owns the EVM concrete's admitted value types — the settings the concrete carries, of which it takes none, and the member's settings document, the concrete's own table beneath the family's, empty because the settings take no field — its `try_from` guard narrowing the member's untrusted document, and `admit_evm_verifier_settings` admitting the narrowed document into `EvmVerifierSettings`. Each function's branches state condition, decision, dependency call, and the exact return outcome.

## `EvmVerifierSettingsDocument::try_from`

`EvmVerifierSettingsDocument::try_from(value: serde_json::Value) -> EvmVerifierSettingsDocumentTryFromReturn`, the guard on entry; the concrete's table is the empty path.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| not a table | the value is not an object | none | none | `Err(NotATable { path: "" })` |
| unknown key | the object holds a key, the lowest in key order | none | none | `Err(UnknownKey { path: "", key })` |
| narrowed | the object holds no key | none | none | `Ok(EvmVerifierSettingsDocument)` |

## `admit_evm_verifier_settings`

`admit_evm_verifier_settings<'a>(deps: &AdmitEvmVerifierSettingsDeps<'a>, params: AdmitEvmVerifierSettingsParams, payload: AdmitEvmVerifierSettingsPayload) -> AdmitEvmVerifierSettingsReturn`, the trusted form; the payload arrives narrowed by the guard; `deps.declaration` is the declaration whose `settings` statements the admission applies, and the EVM declaration's `settings` is empty, so no statement is applied; `params` and `payload` are not read.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| admitted | always | none | none | `Ok(AdmitEvmVerifierSettingsSuccessReturn { settings: EvmVerifierSettings })` |

One branch; no side effect.

## Integration: callee dispositions

- `proof/factory`, `encoding/factory`, `chain/create_chain_forms`, and `chain/factory`: no entry of theirs passes through a function of this node or ends at one of its tests, this node reading the values `VerifierGroupArithmetic`, `DELIVERY_STATEMENT_VERSION_ONE`, `ChainFormsIdentifier`, `ChainMemberIdentifier`, and `EncodingIdentifier` and calling none of their functions.

## Integration: own entries

None; the guard and the admission each call no collaborator, so each property of this node is proven by the unit blocks of its function, and the work removes the concern of a private integration test.

## Integration: private surface

None, and the node carries no private integration test.

## Integration: public surface

None; this node's boundary is not a public entry, so it carries no public integration test.
