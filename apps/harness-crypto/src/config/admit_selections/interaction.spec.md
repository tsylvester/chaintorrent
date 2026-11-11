# `admit_selections` — interaction spec

Branch contract for the `admit_selections` module of `apps/harness-crypto`: `admit_selections` turns each text of the document's `selections` table and of `network_profile.precompile_encodings` into the member of its family's declared set whose `Debug` text it equals — the admitted selection carrying the identifier, scope, and algebra that member declares — and refuses the first text no member names, with the document path that held it. Each branch states condition, decision, dependency call, and the exact return outcome.

## `admit_selections`

`admit_selections(deps: &AdmitSelectionsDeps, params: AdmitSelectionsParams, payload: AdmitSelectionsPayload) -> AdmitSelectionsReturn`, the trusted form: the document's `selections` table and the text list `network_profile.precompile_encodings` arrive as data in `payload`; the function reads no file, no environment variable, and no clock; `deps` and `params` carry nothing and are not read.

### Naming

Private `member_named`: given a text, a slice of members, and a function from a member to the value it is named by; decision the first member in declared order for which `format!("{:?}", &name_of(member))` equals the text, compared whole and case-sensitively; outcome that member, or none; dependency call none — each declared set is a constant the family declares, and a mock selection is never a member of a declared set, so its text names no member.

### Random source refused

Condition `member_named` over `RANDOM_SOURCE_KINDS` and `payload.selections.random_source` returns none; outcome `Err(AdmitSelectionsErrorReturn::RandomSource { text })`, `text` the refused text.

### Random source admitted

Condition a member returns; outcome the member as the admitted kind.

### Encoding refused

Condition `member_named` over `DECLARED_ENCODING_SELECTIONS`, each member named by its `concrete`, and `payload.selections.encoding` returns none; outcome `Err(AdmitSelectionsErrorReturn::Encoding { text })`.

### Encoding admitted

Condition a member returns; outcome `AdmittedEncoding { concrete, identifier }` read from that member.

### Hash-to-scalar refused

Condition `member_named` over `DECLARED_HASH_TO_SCALAR_SELECTIONS`, each member named by its `concrete`, and `payload.selections.hash_to_scalar` returns none; outcome `Err(AdmitSelectionsErrorReturn::HashToScalar { text })`.

### Hash-to-scalar admitted

Condition a member returns; outcome `AdmittedHashToScalar { concrete, identifier }` read from that member.

### Chain forms refused

Condition `member_named` over `DECLARED_CHAIN_FORMS_SELECTIONS`, each member named by its `concrete`, and `payload.selections.chain_forms` returns none; outcome `Err(AdmitSelectionsErrorReturn::ChainForms { text })`.

### Chain forms admitted

Condition a member returns; outcome `AdmittedChainForms { concrete, identifier }` read from that member.

### Verifier refused

Condition `member_named` over `VERIFIER_CONCRETES` and `payload.selections.verifier` returns none; outcome `Err(AdmitSelectionsErrorReturn::Verifier { text })`.

### Verifier admitted

Condition a member returns; outcome the member as the admitted kind.

### Identity scope refused

Condition `member_named` over `DECLARED_KEM_SELECTIONS`, each member named by its `scope`, and `payload.selections.identity_scope` returns none; outcome `Err(AdmitSelectionsErrorReturn::IdentityScope { text })`.

### Identity scope admitted

Condition a member returns; outcome that member's `scope` as the admitted scope, which the KEM decision reads.

### Envelope algebra refused

Condition `member_named` over `DECLARED_KEY_AGREEMENT_SELECTIONS`, each member named by its `algebra`, and `payload.selections.envelope_algebra` returns none; outcome `Err(AdmitSelectionsErrorReturn::EnvelopeAlgebra { text })`.

### Envelope algebra admitted

Condition a member returns; outcome that member's `algebra` as the admitted algebra, which the key agreement and delivery proof decisions read.

### KEM refused

Condition no member of `DECLARED_KEM_SELECTIONS` has a `concrete` whose `Debug` text equals `payload.selections.kem` and a `scope` equal to the admitted scope; outcome `Err(AdmitSelectionsErrorReturn::Kem { text })`.

### KEM admitted

Condition the first member in declared order meeting both; outcome `AdmittedKem { concrete, identifier, scope }` read from that member.

### Key agreement refused

Condition no member of `DECLARED_KEY_AGREEMENT_SELECTIONS` has a `concrete` whose `Debug` text equals `payload.selections.key_agreement` and an `algebra` equal to the admitted algebra; outcome `Err(AdmitSelectionsErrorReturn::KeyAgreement { text })`.

### Key agreement admitted

Condition the first member in declared order meeting both; outcome `AdmittedKeyAgreement { concrete, identifier, algebra }` read from that member.

### Delivery proof refused

Condition `member_named` over `DELIVERY_PROOF_CONCRETES` and `payload.selections.delivery_proof` returns none; outcome `Err(AdmitSelectionsErrorReturn::DeliveryProof { text })`.

### Delivery proof admitted

Condition a member returns; outcome `AdmittedDeliveryProof { concrete, algebra, statement_version }`, `algebra` the admitted algebra and `statement_version` `payload.selections.delivery_statement_version`.

### Curve refused

Per entry of `payload.selections.curves` in order: condition `member_named` over `PAIRING_CONCRETES` and the entry's `pairing_concrete` returns none, or `member_named` over `TARGET_GROUP_ENCODING_IDENTIFIERS` and the entry's `target_group_encoding` returns none; outcome `Err(AdmitSelectionsErrorReturn::Curve { index, field, text })`, `index` the entry's position, `field` `PairingField::PairingConcrete` or `PairingField::TargetGroupEncoding` by which text was refused.

### Curve admitted

Condition both members return; outcome `AdmittedCurve { label, pairing }`, `label` the entry's label cloned and `pairing` `AdmittedPairing { concrete, target_group_encoding }` read from the two members.

### Benchmark concrete refused

Per entry of `payload.selections.benchmark_concretes` in order: condition as a curve's; outcome `Err(AdmitSelectionsErrorReturn::BenchmarkConcrete { index, field, text })` with the same `field` rule.

### Benchmark concrete admitted

Condition both members return; outcome `AdmittedPairing { concrete, target_group_encoding }` read from the two members.

### Precompile encoding refused

Per entry of `payload.precompile_encodings` in order: condition `member_named` over `PRECOMPILE_ENCODINGS` and the entry returns none; outcome `Err(AdmitSelectionsErrorReturn::PrecompileEncoding { index, text })`, `index` the entry's position.

### Precompile encoding admitted

Condition a member returns; outcome the member as the admitted encoding, in the list's order.

### Success

Outcome `Ok(AdmitSelectionsSuccessReturn { admitted })`, `admitted` the `AdmittedSelections` the admitted values compose.

### Unreached branches

- A scope no declared KEM member carries together with the named concrete: the scope is admitted from a member of `DECLARED_KEM_SELECTIONS` and the declared set holds one concrete whose declaration lists both scopes, so no document makes the concrete named and the scope admitted disagree; the `Kem` refusal is reached by an unnamed concrete, and the branch for a named concrete with a scope it does not carry has no unit block; the same holds for the key agreement's algebra.

### Ordering

Random source, encoding, hash-to-scalar, chain forms, verifier, identity scope, envelope algebra, KEM, key agreement, delivery proof, each curve, each benchmark concrete, then each precompile encoding; the first refusal returns and every later text is unread; no side effect.

## Integration: own entries

- `every_member_of_every_declared_set_is_admitted_by_its_own_name`: condition `admit_selections` over a payload whose every text is the `Debug` text of one member of the family's real declared set, every collaborator real, the declared sets the families and the verifier module declare; outcome `Ok` whose admitted value for each text equals the member the text names, with the identifier, scope, and algebra that member carries; variation every member of `RANDOM_SOURCE_KINDS`, `DECLARED_ENCODING_SELECTIONS`, `DECLARED_HASH_TO_SCALAR_SELECTIONS`, `DECLARED_CHAIN_FORMS_SELECTIONS`, `VERIFIER_CONCRETES`, `DECLARED_KEM_SELECTIONS`, `DECLARED_KEY_AGREEMENT_SELECTIONS`, and `DELIVERY_PROOF_CONCRETES` in turn, and every member of `PAIRING_CONCRETES` crossed with every member of `TARGET_GROUP_ENCODING_IDENTIFIERS` as a curve and as a benchmark concrete, and every member of `PRECOMPILE_ENCODINGS`; edge a text matched to a neighbouring member, an identifier, scope, or algebra the declared member does not carry, or a list reordered, differs from the member built in Arrange.
- `a_mock_selections_name_is_never_admitted`: condition `admit_selections` over a payload whose `random_source`, `encoding`, `chain_forms`, `kem`, and `hash_to_scalar` texts are each the `Debug` text of the family's mock selection and whose other texts name real members; outcome `Err` carrying the refused family's variant and that text; variation each of those families in turn, the other texts naming real members; edge a declared set that held its mock selection would admit it.

## Integration: callee dispositions

No entry of a direct dependency passes through `admit_selections` or ends at its tests, because it calls no factory and reads each family's declared set as a constant.

## Integration: private surface

The chain of real functions `admit_selections`, `member_named`, and the `Debug` text of each declared member, all running for real over the real declared sets; no outer-edge collaborator; the observable result is the `AdmitSelectionsReturn`; the entries proven are `every_member_of_every_declared_set_is_admitted_by_its_own_name` and `a_mock_selections_name_is_never_admitted`.

## Integration: public surface

An outside caller invokes `admit_selections` through the crate root with an `AdmitSelectionsDeps`, an `AdmitSelectionsParams`, and an `AdmitSelectionsPayload` holding a `SelectionsTable` and a text list, and observes the `AdmitSelectionsReturn` and the `AdmittedSelections` in its success; the entries proven are `every_member_of_every_declared_set_is_admitted_by_its_own_name` and `a_mock_selections_name_is_never_admitted`; there is no outer-edge collaborator.
