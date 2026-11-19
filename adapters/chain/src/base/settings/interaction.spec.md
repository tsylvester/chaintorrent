# `admit_base_chain_settings` — interaction spec

Branch contract for the `settings` module of `base` in the `chain` crate: the Base concrete's own settings admission — the document guard narrows the member's settings table to `BaseChainSettingsDocument`, and `admit_base_chain_settings` turns that narrowed document into `BaseChainSettings`, reading every bound and default from the declaration the caller hands it in `deps`. Each branch states condition, decision, dependency call, and the exact return outcome.

## `BaseChainSettingsDocument::try_from(value: serde_json::Value) -> BaseChainSettingsDocumentTryFromReturn`

The guard on entry. The member's table is the document's root object; its keys are `endpoint`, `settlement_endpoint`, `chain_identifier`, `registry`, `entitlement_contract`, `gas_price_oracle`, `anchor_state_registry`, and `batch_page_limit`, and every value is text.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| not a table | the value is not an object | `serde_json::Value` variant check | none | `Err(BaseChainSettingsDocumentErrorReturn::NotATable { path: "" })` |
| unknown key | a key outside the eight named keys | the lowest key in key order, decided before any value is read | none | `Err(BaseChainSettingsDocumentErrorReturn::UnknownKey { path: "", key })` |
| not text | a present key whose value is not a string | the lowest key in key order, decided after the unknown-key scan | none | `Err(BaseChainSettingsDocumentErrorReturn::NotText { path })`, the path the key |
| narrowed | an object of declared keys and text values | every check passes | none | `Ok` of the document holding each present value and `None` for each absent one |

## `admit_base_chain_settings(deps: &AdmitBaseChainSettingsDeps, params: AdmitBaseChainSettingsParams, payload: AdmitBaseChainSettingsPayload) -> AdmitBaseChainSettingsReturn`

The trusted form: the payload arrives already narrowed by the guard; `params` is not read. No dependency call other than `Url::parse` in the endpoint branch.

### Resolving a setting's text

Shared by every per-setting branch below; dependency none. The statement is the first of `deps.declaration.settings` whose `name` equals the setting's name.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| undeclared | no statement of that name | the settings' names scanned in declaration order | none | `Err(AdmitBaseChainSettingsErrorReturn::UndeclaredSetting { setting })` |
| fixed, supplied | `Fixed { value }` and the document supplies a text | the disposition discriminant | none | `Err(AdmitBaseChainSettingsErrorReturn::FixedSetting { setting, supplied })` |
| fixed, absent | `Fixed { value }` and the document supplies nothing | the disposition discriminant | none | the text is `value`; a `Fixed` text is not matched against bounds |
| settable, supplied | `Settable { bounds, default }` and the document supplies a text | the disposition discriminant | none | the text is the supplied text |
| settable, absent | `Settable { bounds, default }` and the document supplies nothing | the disposition discriminant | none | the text is `default` |

Then the bounds of a `Settable` statement, an exhaustive `match` decided before the text is admitted: `Unbounded`, the text is admitted; `TextPrefixedBy { prefixes }`, condition the text begins with none of the prefixes, outcome `Err(AdmitBaseChainSettingsErrorReturn::OutOfBounds { setting, supplied: text })`.

### Per-setting branches

| Setting | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| `endpoint`, then `settlement_endpoint` | the text resolved above | `Url::parse` admits the text | `Url::parse` over the text | the parser's refusal → `Err(AdmitBaseChainSettingsErrorReturn::Endpoint { setting, error })` carrying the parser's error intact; admitted → `ChainEndpoint { text }` |
| `chain_identifier` | the text resolved above | the text is nonempty, every byte in `b'0'..=b'9'`, and the value less than two to the 256th power | none | any condition unmet → `Err(AdmitBaseChainSettingsErrorReturn::ChainIdentifier { setting, text })`; admitted → the value as 32 big-endian bytes |
| `registry`, `entitlement_contract`, `gas_price_oracle`, then `anchor_state_registry` | the text resolved above | the text is `0x` followed by forty hexadecimal digits of either case | none | any condition unmet → `Err(AdmitBaseChainSettingsErrorReturn::Address { setting, text })`; admitted → the twenty bytes the digits state |
| `batch_page_limit` | the text resolved above | the text is nonempty, every byte in `b'0'..=b'9'`, and the value a nonzero 64-bit unsigned value | none | any condition unmet → `Err(AdmitBaseChainSettingsErrorReturn::BatchPageLimit { setting, text })`; admitted → the value |

### Computed

Outcome `Ok(AdmitBaseChainSettingsSuccessReturn { settings })`, the settings holding each admitted value.

## Ordering and invariants

- The settings are read in this order: `endpoint`, `settlement_endpoint`, `chain_identifier`, `registry`, `entitlement_contract`, `gas_price_oracle`, `anchor_state_registry`, then `batch_page_limit`. The first refusal returns and no later value is read.
- For each setting the order within the branch is: resolve the statement, apply the disposition, apply the bounds, then admit the text by its form. A `Fixed` statement's text is never matched against bounds.
- `Url::parse` is the only dependency call, reached at most twice per call, once per endpoint setting.
- No side effect: the admission reads no clock and touches no network or filesystem.

## Integration: callee dispositions

No entry of `chain/evm_forms` or `pairing/factory` passes through a function of this node or ends at one of its tests: this node reads `ChainFormsIdentifier::EvmV1`, `PrecompileEncoding`, and `PRECOMPILE_ENCODINGS` as values and calls none of their functions, so there is nothing to disposition.

## Integration: own entries

- `the_declared_settings_are_admitted_through_the_real_constructors`: condition `admit_base_chain_settings` is called over `BASE_CHAIN_DECLARATION` with a document that supplies nothing; outcome `Ok` whose settings hold the endpoint `https://mainnet.base.org`, the settlement endpoint `https://ethereum-rpc.publicnode.com`, the chain identifier `8453` as 32 big-endian bytes, the registry, the entitlement contract, the oracle, and the anchor state registry as the twenty bytes of the texts the declaration states, and the batch page limit `64`; variation each setting's default differing from every other, so a default used for another setting, or a text its real admission refuses, fails; edge that must survive the composition: each value is the real admission's value for its text.

## Integration: private surface

The chain of real functions is `admit_base_chain_settings`, running the real `Url::parse`; no outer-edge collaborator is mocked; the observable result is the `AdmitBaseChainSettingsReturn` and the admitted values; the entry proven is the own entry above.

## Integration: public surface

None. This node's boundary is not a public entry, so it carries no public integration test.
