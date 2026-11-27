# `lookup_envelope_declaration` — interaction spec

Branch contract for the `lookup_envelope_declaration` module of the `envelope` crate: a family-owned function that reads the declaration the member selection `params.concrete` names, before any instance exists. Each branch states condition, decision, dependency call, and the exact return outcome.

## `lookup_envelope_declaration(deps: &LookupEnvelopeDeclarationDeps, params: LookupEnvelopeDeclarationParams, payload: LookupEnvelopeDeclarationPayload) -> LookupEnvelopeDeclarationReturn`

The trusted form: every input is an in-crate typed value, so no guard narrows anything, and `deps` and `payload` are fieldless and are not read. The decision is an exhaustive `match` on `params.concrete`, one arm per `KeyAgreementConcrete` variant — `PairingElGamal` and, under `#[cfg(any(test, feature = "mocks"))]`, `Mock` — so a variant with no arm fails to compile. No branch refuses: the error arm of the return is uninhabited.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| real member | `params.concrete` is `KeyAgreementConcrete::PairingElGamal` | an exhaustive `match` on `params.concrete` | none; `PAIRING_ELGAMAL_DECLARATION` is read as a constant through `crate::pairing_elgamal::provides` | `Ok(LookupEnvelopeDeclarationSuccessReturn { declaration: PAIRING_ELGAMAL_DECLARATION })` |
| mock member, under `#[cfg(any(test, feature = "mocks"))]` | `params.concrete` is `KeyAgreementConcrete::Mock(failure_mode)` | the same `match` | none; `MOCK_I_KEY_AGREEMENT_ADAPTER_DECLARATION` is read as a constant through `crate::factory::provides` | `Ok(LookupEnvelopeDeclarationSuccessReturn { declaration })`, `declaration` that constant with `concrete` replaced by `KeyAgreementConcrete::Mock(failure_mode)` and `identifier`, `algebra`, `possession_g1_tag`, `possession_g2_tag`, `adapter_version`, and `interface_version` preserved |

## Ordering and edges

- The decision is the `match` on `params.concrete`, and each arm yields a declaration; no arm returns the error arm.
- Side effects none; the function constructs no adapter, draws no bytes, and encodes nothing. The `match` is exhaustive over `KeyAgreementConcrete`.
- `deps` holds no collaborator and `payload` carries nothing; neither is read.
- An arm no input reaches is not an entry.

## Integration: callee dispositions

`lookup_envelope_declaration` reads constants and calls no function, so no callee's entry passes through it and none is absorbed or carried here.

## Integration: own entries

Each entry is proven by the block of the same name in this node's public integration test element `tests/lookup_envelope_declaration_integration_test.rs`.

- `every_declared_key_agreement_member_declares_itself`: condition `lookup_envelope_declaration` called with a `LookupEnvelopeDeclarationParams` carrying each member of `KEY_AGREEMENT_CONCRETES`, the selection `build_key_agreement_concrete` returns for that member; outcome the returned declaration's `concrete` equals the member looked up, its `identifier` is a member of `KEY_AGREEMENT_IDENTIFIERS`, and its `algebra` is a member of `ENVELOPE_ALGEBRAS`; variation every member of `KEY_AGREEMENT_CONCRETES`, so a lookup that returns one member's declaration for another, or a declaration whose identifier or algebra names no declared value, fails; edge that must survive the composition: the declaration is read through the family's private path with no member constructed and no type parameter chosen; route `lookup_envelope_declaration`, whose real arm reads `PAIRING_ELGAMAL_DECLARATION`; the route reaches no outer-edge collaborator.

## Integration: private surface

This spec lists no private-surface entry: `lookup_envelope_declaration` reads a constant and calls no function, so no chain of real functions runs inside the crate.

## Integration: public surface

An outside caller invokes `lookup_envelope_declaration` with `build_lookup_envelope_declaration_deps`, `build_lookup_envelope_declaration_params` carrying the member's selection in `concrete`, and `build_lookup_envelope_declaration_payload`, and observes the returned declaration's `concrete`, `identifier`, and `algebra`; the entry proven is `every_declared_key_agreement_member_declares_itself`, by the block of the same name in `lookup_envelope_declaration_integration_test.rs`; the fixtures are `KEY_AGREEMENT_CONCRETES`, `KEY_AGREEMENT_IDENTIFIERS`, `ENVELOPE_ALGEBRAS`, `KeyAgreementConcreteOverrides`, `build_key_agreement_concrete`, `LookupEnvelopeDeclarationParamsOverrides`, `build_lookup_envelope_declaration_deps`, `build_lookup_envelope_declaration_params`, and `build_lookup_envelope_declaration_payload`; the route is the public call alone.
