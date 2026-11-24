# `admit_pairing_composition` — interaction spec

Branch contract for the `admit_pairing_composition` module of the `pairing` crate: the family's composition admission comparing a member's declared provisions with the values its params require. Each branch states condition, decision, dependency call, and the exact return outcome.

## `admit_pairing_composition(deps: &AdmitPairingCompositionDeps, params: AdmitPairingCompositionParams, payload: AdmitPairingCompositionPayload) -> AdmitPairingCompositionReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| `every provision agrees` | the declaration the lookup returns for `params.identifier` has `curve` equal to `params.required_curve`, `verifier_group_arithmetic` equal to `params.required_verifier_group_arithmetic`, `precompile_encoding` a member of `params.required_precompile_encodings`, and `target_group_encoding` equal to `params.required_target_group_encoding` | each comparison holds | `deps.lookup_pairing_declaration` with `LookupPairingDeclarationParams { concrete: params.identifier }` | `Ok(AdmitPairingCompositionSuccessReturn { identifier })` where `identifier` is `params.identifier` |
| `curve disagrees` | the declared `curve` differs from `params.required_curve` | the curve comparison fails | the same lookup call | `Err(AdmitPairingCompositionErrorReturn::Incompatible { identifier, disagreements })` where `identifier` is `params.identifier` and `disagreements` holds `PairingDisagreement::Curve { declared, required }` with the declared curve and `params.required_curve`, together with the disagreement of every other provision that also disagrees |
| `verifier-group arithmetic disagrees` | the declared `verifier_group_arithmetic` differs from `params.required_verifier_group_arithmetic` | the arithmetic comparison fails | the same lookup call | the same `Incompatible` error whose `disagreements` holds `PairingDisagreement::VerifierGroupArithmetic { declared, required }` |
| `precompile encoding disagrees` | the declared `precompile_encoding` is not a member of `params.required_precompile_encodings` | the membership check fails | the same lookup call | the same `Incompatible` error whose `disagreements` holds `PairingDisagreement::PrecompileEncoding { declared, required }` with the declared encoding and `params.required_precompile_encodings` |
| `target-group encoding disagrees` | the declared `target_group_encoding` differs from `params.required_target_group_encoding` | the target-group comparison fails | the same lookup call | the same `Incompatible` error whose `disagreements` holds `PairingDisagreement::TargetGroupEncoding { declared, required }` |
| `several provisions disagree` | more than one comparison fails | each comparison is evaluated independently of the others | the same lookup call | the same `Incompatible` error whose `disagreements` holds one `PairingDisagreement` for each failed comparison, in the order curve, verifier-group arithmetic, precompile encoding, target-group encoding |

## Ordering and edges

- The lookup is called once, before any comparison, and its `Infallible` error arm has no branch.
- The comparisons are evaluated independently so every disagreeing provision is named.
- The function does not match on `params.identifier`, so the real members and the mock member pass through the same branches.
- `payload` is not read.
- No concrete is constructed and no side effect occurs.
- The error arm has one flavor, carrying the member and every disagreement.
- A further concrete adds no branch.

## Own entries

- `admission_compares_the_provisions_of_the_member_named`: condition `params.identifier` is a member of each curve and the required values are those of the member's own curve or of the other curve; outcome `Ok` naming the member where its declared provisions are the required ones, and `Incompatible` naming the member and every disagreeing provision, with the declared values of the member named, where they are not; input variation a member of each curve against the required values of each curve, so an admission that reads one fixed member's declaration, or compares against the requirements alone, fails; edge that must survive composition: the provisions compared are those the real lookup returns for the identifier the caller named.
- `mock_admission_compares_the_mock_declaration`, present only under `#[cfg(any(test, feature = "mocks"))]`: condition `params.identifier` is `PairingConcrete::Mock(mode)` with a mode other than `Succeeds`; outcome `Ok` naming `PairingConcrete::Mock(mode)` against the provisions the mock declaration states, and `Incompatible` naming `PairingConcrete::Mock(mode)` against provisions it does not state; input variation the mode and the required values; edge: the mock member is absent from `PAIRING_CONCRETES` and reached only by naming it.
- `admission_names_each_disagreeing_provision_for_each_declared_member`: condition `params.identifier` is any member of `PAIRING_CONCRETES` and one required value is that of a member of a different curve; outcome `Incompatible` naming the member and the one provision replaced, with its declared and required values; input variation every member and every provision, so an admission that omits or confuses a provision fails; edge: the refusal is observed through the crate's public surface.
- `admission_names_every_disagreeing_provision_in_order_for_each_declared_member`: condition `params.identifier` is any member of `PAIRING_CONCRETES` and every required value is that of a member of a different curve; outcome `Incompatible` naming the member and every provision in the order curve, verifier-group arithmetic, precompile encoding, target-group encoding; input variation every member; edge: the order survives the public surface.

## Callee dispositions

The entries `declaration_names_the_member_looked_up` and `mock_declaration_names_the_identifier_looked_up` of `pairing/lookup_pairing_declaration` state that a declaration's `concrete` is the member looked up; this function reads no `concrete` field of the declaration and returns none, so neither entry passes through a function of this node or ends at a test of this node, and neither is dispositioned. The provisions the function does read are proven against the real lookup by the private blocks of the own entries. No entry of `pairing/admit_pairing_selection` passes through a function of this node or ends at a test of this node, so none is dispositioned.

## Route

`admit_pairing_composition` calls `lookup_pairing_declaration`, which reads the `DECLARATION` constant of the concrete the identifier names, or `MOCK_I_PAIRING_ADAPTER_DECLARATION` for the mock member; no outer-edge collaborator is on the route.

## Private surface

The chain of real functions `admit_pairing_composition` then `lookup_pairing_declaration` then the declaration constant it reads; no outer-edge collaborator is mocked; the observable result is the `Ok` identifier or the `Incompatible` error. The entries `admission_compares_the_provisions_of_the_member_named` and `mock_admission_compares_the_mock_declaration` are proven by the private integration test.

## Public surface

An outside caller invokes `admit_pairing_composition` with deps holding the public `lookup_pairing_declaration`, params carrying a `PairingConcrete` and the required values, and the fieldless payload, each built through the official mocks, and observes the `Ok` identifier or the `Incompatible` error. The entries `admission_compares_the_provisions_of_the_member_named`, `mock_admission_compares_the_mock_declaration`, `admission_names_each_disagreeing_provision_for_each_declared_member`, and `admission_names_every_disagreeing_provision_in_order_for_each_declared_member` are proven by the public integration test.
