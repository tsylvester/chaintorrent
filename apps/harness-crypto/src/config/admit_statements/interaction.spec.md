# `admit_statements` — interaction spec

Branch contract for the `admit_statements` module of `apps/harness-crypto`: `admit_statements` turns the document's `network_profile`, `statements`, and `identities` tables, the labels of the configured curves, and the identity scope the payload carries into the admitted network profile, statements, and identities the later modules take — each canonical value's kind text mapped to the member of the encoding family's declared field-kind sets it names and its value text read as the kind declares, each context's identifier, root, and version admitted through the domain and hashing constructors, each purpose through its relation's declared code, and each identity admitted under the one scope — and refuses the first text any step refuses, carrying the document path that held it and the callee's refusal whole. Each branch states condition, decision, dependency call, and the exact return outcome.

## `admit_statements`

`admit_statements(deps: &AdmitStatementsDeps, params: AdmitStatementsParams, payload: AdmitStatementsPayload) -> AdmitStatementsReturn`, the trusted form: the document's `network_profile`, `statements`, and `identities` tables, the labels of the configured curves in curve order, and the `IdentityScope` arrive as data in `payload`; the function reads no file, no environment variable, and no clock; `deps` and `params` carry nothing and are not read.

### `kind_named`

Private `kind_named`: given a text, decision the first member in declared order of `DECLARED_FIXED_WIDTH_FIELD_KINDS` and then of `DECLARED_UNSIGNED_FIELD_KINDS` whose `format!("{:?}", &kind)` equals the text, compared whole and case-sensitively; outcome that member, or none; dependency call none — the declared sets are constants the encoding family declares.

### `bytes_from_hex`

Private `bytes_from_hex`: given a text, decision a text that lacks the `0x` prefix, holds an odd count of digits after it, or holds a character that is not an ASCII hexadecimal digit of either case returns `Err(HexRefusal::Malformed { text })`; outcome otherwise the bytes the digit pairs name, in order, and none for the empty digit string.

### `fixed_from_hex`

Private `fixed_from_hex<const N: usize>`: given a text, decision `bytes_from_hex` first, its refusal returned unchanged, and a byte count other than `N` returns `Err(HexRefusal::WrongWidth { expected: N, actual })`; outcome otherwise the array.

### `unsigned256_from_decimal`

Private `unsigned256_from_decimal`: given a digit text already known to be non-empty ASCII digits, an accumulator of 32 zero bytes, and for each digit in text order the accumulator multiplied by ten and increased by the digit, byte by byte from the least significant with a carry; decision a carry out of the most significant byte returns `Err(CanonicalValueRefusal::OutOfRange { text })`; outcome otherwise the accumulator.

### `canonical_value`

Private `canonical_value`: given a `CanonicalValueTable`, decision `kind_named` over its `kind`, no member returning `Err(CanonicalValueRefusal::UnknownKind { text })`; a member continues by its variant: `FixedBytes32` and `FixedBytes20` admit `fixed_from_hex` over the variant's array width, a refusal returned as `Err(CanonicalValueRefusal::Hex(refusal))` and a success as the variant over the array; `Unsigned16`, `Unsigned32`, and `Unsigned64` check the text is non-empty ASCII digits, a text that is not returning `Err(CanonicalValueRefusal::MalformedDecimal { text })`, then `str::parse` into the variant's integer, an overflow returning `Err(CanonicalValueRefusal::OutOfRange { text })`; `Unsigned256` checks the digits the same way and continues with `unsigned256_from_decimal`; `Text` and `Bytes` return `Err(CanonicalValueRefusal::UnknownKind { text })`.

### Network profile refused

Condition `canonical_value` over `payload.network_profile.chain_identifier`, then `entitlement_contract`, then `gas_price_oracle` refuses; `verifier_endpoint` and `precompile_encodings` are not read; outcome `Err(AdmitStatementsErrorReturn::NetworkProfile { field, refusal })` with the field of the refused entry and the refusal unchanged.

### Deployment records refused

Condition, after the three values above and before the statements, for each label of `payload.curve_labels` in order, the entry of `payload.network_profile.deployment_records` under that label: no entry returns `Err(AdmitStatementsErrorReturn::MissingDeploymentRecord { label })`; an entry is admitted by `canonical_value`, a refusal returning `Err(AdmitStatementsErrorReturn::DeploymentRecord { label, refusal })` unchanged; then, for each key of `deployment_records` in key order that is no label of `payload.curve_labels`, `Err(AdmitStatementsErrorReturn::UndeclaredDeploymentRecord { label: key })`.

### Network profile admitted

Condition all three admit and the deployment records admit; outcome `AdmittedNetworkProfile` of the admitted values, its `deployment_records` holding the admitted records in curve order.

### Context refused

Per relation in the order `Mint`, `Transfer`, each over its `ContextTable`, the steps in this order and the first refusal returning `Err(AdmitStatementsErrorReturn::Context { relation, field, refusal })`:

- suite identifier text through `fixed_from_hex::<SUITE_IDENTIFIER_LENGTH>` — a refusal `ContextRefusal::Hex` at `ContextField::SuiteIdentifier`;
- `suite_version` through `u16::try_from` — a failure `ContextRefusal::SuiteVersion { version }` at `ContextField::SuiteVersion`;
- `SuiteIdentifier::try_new` over the array and the version — a refusal `ContextRefusal::Suite` at `ContextField::SuiteIdentifier`;
- asset root text through `fixed_from_hex::<32>` — a refusal `ContextRefusal::Hex` at `ContextField::AssetRoot`, then `Root::try_new` — a refusal `ContextRefusal::Root` at `ContextField::AssetRoot`;
- parameter set text through `fixed_from_hex::<PARAMETER_SET_IDENTIFIER_LENGTH>` — a refusal `ContextRefusal::Hex` at `ContextField::ParameterSetIdentifier`, then `ParameterSetIdentifier::try_new` — a refusal `ContextRefusal::ParameterSet` at `ContextField::ParameterSetIdentifier`;
- `canonical_value` over `source_entitlement`, `target_entitlement`, `old_interval`, `new_interval`, `seller`, and `buyer` in that order — a refusal `ContextRefusal::Value` at the value's field;
- purpose through `MintPurpose::try_from(code)` for `Mint` and `TransferPurpose::try_from(code)` for `Transfer` — a refusal `ContextRefusal::Purpose` at `ContextField::Purpose`.

### Context admitted

Outcome `CanonicalDeliveryContext` holding the admitted suite identifier, the network profile's admitted `chain_identifier` as `chain` and `entitlement_contract` cloned, the admitted root, the admitted parameter-set identifier, the six admitted canonical values, and `expiry` as the table holds it; the relation's purpose field holds the admitted purpose.

### Identity refused

Per text in the order `proof`, then each `mapping` entry in the order held, under `payload.scope`: `IdentityScope::Asset` through `fixed_from_hex::<32>` — a refusal `Err(AdmitStatementsErrorReturn::Identity { identity, refusal: IdentityRefusal::Hex(refusal) })`, then `Root::try_new` — a refusal `Err(AdmitStatementsErrorReturn::Identity { identity, refusal: IdentityRefusal::Root(error) })`; `IdentityScope::Entitlement` through `bytes_from_hex` — a refusal `Err(AdmitStatementsErrorReturn::Identity { identity, refusal: IdentityRefusal::Hex(refusal) })`; `identity` `IdentityField::Proof` or `IdentityField::Mapping { index }`, the index the entry's position.

### Identity admitted

Outcome `AdmittedIdentity::Asset { root }` under the asset scope and `AdmittedIdentity::Entitlement { canonical }` under the entitlement scope.

### Success

Outcome `Ok(AdmitStatementsSuccessReturn { network_profile, statements, identities })`.

### Unreached branches

- A `Text` or `Bytes` kind in a declared set: neither set holds either, so no input reaches the `canonical_value` arm that refuses them with a member, and that arm has no unit block; the `UnknownKind` refusal is reached by a text that names no member.

### Ordering

The network profile, then each context in the relation order above, then the identities; the first refusal returns and every later text is unread; `deps` and `params` carry nothing and are not read; no side effect.

## Integration: own entries

- `every_declared_kind_admits_its_value_text_to_its_own_variant`: condition `admit_statements` over a payload whose canonical values name, in turn, a member of `DECLARED_FIXED_WIDTH_FIELD_KINDS` or of `DECLARED_UNSIGNED_FIELD_KINDS` by its `Debug` text with a value text the kind admits, every collaborator real; outcome `Ok` whose admitted value equals the variant of that kind over the bytes or the integer the text names; variation every member of both declared sets in the network profile's `chain_identifier` and in a context's `source_entitlement`, a fixed-width text of uppercase and lowercase digits, a decimal text at zero and at the kind's maximum; edge a value read as the wrong width, a decimal read as hexadecimal, a 256-bit text above 64 bits truncated, or a variant chosen by position instead of by kind differs from the value built in Arrange.
- `a_kind_outside_the_declared_sets_is_never_admitted`: condition `admit_statements` over a payload whose `chain_identifier` kind text is the `Debug` text of `CanonicalFieldKind::Text` and then of `CanonicalFieldKind::Bytes`, and a text no variant is named by; outcome `Err(AdmitStatementsErrorReturn::NetworkProfile { field: NetworkProfileField::ChainIdentifier, refusal: CanonicalValueRefusal::UnknownKind { text } })`; variation each of the three texts; edge a declared set that held `Text` or `Bytes` would admit it.
- `every_context_carries_the_network_profiles_chain_identifier_and_entitlement_contract`: condition `admit_statements` over a payload whose network profile holds a chain identifier and an entitlement contract that differ from every context value; outcome each of the mint and transfer contexts holds the admitted `chain_identifier` as `chain` and the admitted `entitlement_contract`; variation each relation; edge a context that read its chain from a default or from another relation's table differs.
- `identities_are_admitted_under_the_scope_the_payload_names`: condition `admit_statements` over a payload whose identities are 32-byte hex texts and whose `scope` is `IdentityScope::Asset`, and over identities of differing lengths with `IdentityScope::Entitlement`; outcome the asset scope returns `AdmittedIdentity::Asset` over a `Root` holding the text's bytes for the proof and each mapping entry in the document's order, the entitlement scope returns `AdmittedIdentity::Entitlement` over the decoded bytes; variation both scopes, an empty mapping list, and an entitlement text of no digits; edge an identity admitted under the other scope's shape, a mapping list reordered, or a 31-byte text admitted under the asset scope differs.
- `every_curve_has_its_own_deployment_record_in_curve_order`: condition `admit_statements` over a payload whose `curve_labels` are two labels and whose `deployment_records` holds one distinct `FixedBytes20` record under each; outcome `Ok` whose `network_profile.deployment_records` equals the two values in curve order; variation the labels given in either order, which changes the order of the admitted records and no value; edge a record read under another label, a single record shared by both curves, or the records returned in key order instead of curve order, differs from the values built in Arrange.
- `a_curve_without_a_record_and_a_record_without_a_curve_are_each_refused_with_the_label`: condition `admit_statements` over a payload with a curve label whose record is absent, then over a payload whose `deployment_records` holds a key no curve label names; outcome `Err(MissingDeploymentRecord { label })` and `Err(UndeclaredDeploymentRecord { label })` carrying that label; edge a refusal carrying another label or a missing label admitted as an empty record differs.

## Integration: callee dispositions

No entry of a direct dependency passes through `admit_statements` or ends at its tests, because it calls no factory and no adapter, and each constructor it calls is a leaf function proven by its own node.

## Integration: private surface

The chain of real functions `admit_statements`, `kind_named`, `canonical_value`, `fixed_from_hex`, `bytes_from_hex`, `unsigned256_from_decimal`, `SuiteIdentifier::try_new`, `ParameterSetIdentifier::try_new`, `Root::try_new`, and the purposes' `try_from`, all running for real over the real declared sets; no outer-edge collaborator; the observable result is the `AdmitStatementsReturn`; the entries proven are `every_declared_kind_admits_its_value_text_to_its_own_variant`, `a_kind_outside_the_declared_sets_is_never_admitted`, `every_context_carries_the_network_profiles_chain_identifier_and_entitlement_contract`, `identities_are_admitted_under_the_scope_the_payload_names`, `every_curve_has_its_own_deployment_record_in_curve_order`, and `a_curve_without_a_record_and_a_record_without_a_curve_are_each_refused_with_the_label`.

## Integration: public surface

An outside caller invokes `admit_statements` through the crate root with an `AdmitStatementsDeps`, an `AdmitStatementsParams`, and an `AdmitStatementsPayload` holding the three document tables, the curve labels, and an `IdentityScope`, and observes the `AdmitStatementsReturn` and the `AdmittedNetworkProfile`, `AdmittedStatements`, and `AdmittedIdentities` in its success; the entries proven are the own entries above; there is no outer-edge collaborator.
