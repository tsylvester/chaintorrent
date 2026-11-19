# `lookup_chain_declaration` — interaction spec

Branch contract for the `lookup_chain_declaration` module of the `chain` crate: the chain family's declaration lookup — a total function from a member identifier to that member's declaration, so a level that holds an identifier reads the forms identifier, the precompile encodings, the fee entries, and every value the member fixes, without constructing the member. Each branch states condition, decision, dependency call, and the exact return outcome.

## `lookup_chain_declaration(deps: &LookupChainDeclarationDeps, params: LookupChainDeclarationParams, payload: LookupChainDeclarationPayload) -> LookupChainDeclarationReturn`

The trusted form: `deps` and `payload` are not read. Decision: an exhaustive `match` on `params.identifier`, one arm per `ChainMemberIdentifier` variant, so a further member adds an arm.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| base | `params.identifier` is `ChainMemberIdentifier::Base` | the discriminant | none | `Ok(LookupChainDeclarationSuccessReturn { declaration: BASE_CHAIN_DECLARATION })` |
| mock — `#[cfg(any(test, feature = "mocks"))]` | `params.identifier` is `ChainMemberIdentifier::Mock` | the discriminant | none | `Ok(LookupChainDeclarationSuccessReturn { declaration: MOCK_I_CHAIN_ADAPTER_DECLARATION })` |

The error arm is uninhabited: `LookupChainDeclarationReturn` is `Result<LookupChainDeclarationSuccessReturn, Infallible>`, the identifier being a closed enum the `match` covers.

## Ordering and invariants

- The matched branch returns; no side effect — no clock, no environment variable, no network, no filesystem.

## Integration: callee dispositions

No entry of the interaction specs of `chain/base` or `chain/base/settings` passes through a function of this node or ends at one of its tests, this node calling none of their functions and reading their constants alone:

- the `chain/base` entries `each_tier_reads_its_own_head_and_orders_the_references`, `a_context_reaches_the_registry_with_every_field`, `batch_evaluation_agrees_with_individual_evaluation`, `a_reorganized_reference_is_refused_whatever_its_number`, `a_delivered_envelope_is_recovered_from_the_settlement_that_created_the_interval`, and `an_estimated_fee_carries_its_execution_gas_and_its_l1_data_fee_in_the_declared_order`, and the `chain/base/settings` entry `the_declared_settings_are_admitted_through_the_real_constructors`, end at those nodes' tests;
- the entries `chain/base` carries, `chain_forms_identity_survives_its_field_round_trip`, `chain_forms_entitlement_survives_its_field_round_trip`, `chain_forms_interval_survives_its_field_round_trip`, `chain_forms_chain_identifier_survives_its_field_round_trip`, and `an_admitted_commitment_reads_back_its_bytes_and_its_scheme`, neither pass through a function of this node nor end at one of its tests, this node reading no form and no commitment.

## Integration: own entries

- `each_declared_member_is_looked_up_to_the_declaration_it_owns`: condition `lookup_chain_declaration` is called once for each identifier in `DECLARED_CHAIN_MEMBERS` by a caller outside the crate; outcome `Ok` whose declaration's `identifier` is the identifier looked up and whose `name` is a nonempty text; variation each member of the declared set, so a lookup that returns one member's declaration for every identifier, or a declaration built from the identifier, fails; edge that must survive the composition: the declaration reaches the outside caller through this function alone, the member's constant being unnameable outside the crate.
- `the_mock_member_is_looked_up_to_the_declaration_the_mock_concrete_owns`: condition `lookup_chain_declaration` is called over `ChainMemberIdentifier::Mock` by a caller outside the crate under the `mocks` feature, and over the first identifier of `DECLARED_CHAIN_MEMBERS`; outcome `Ok` whose declaration's `identifier` is `ChainMemberIdentifier::Mock` and which differs from the declaration the declared member returns; variation the mock identifier beside a declared member's, so a branch that returns the declared member's declaration for the mock identifier fails; edge that must survive the composition: the mock member's declaration is its own and the declared set never holds it.

## Integration: private surface

The real function `lookup_chain_declaration`, which calls no function; the observable result is the `LookupChainDeclarationReturn`; the branches are proven by the unit blocks of `test.rs`, and the node carries no private integration test.

## Integration: public surface

The crate's `lib.rs` re-exports `lookup_chain_declaration::provides`, so an outside caller invokes `lookup_chain_declaration` over a member identifier. It observes the `LookupChainDeclarationReturn`: the `Ok` arm's declaration through its public fields. The entries proven are `each_declared_member_is_looked_up_to_the_declaration_it_owns` and `the_mock_member_is_looked_up_to_the_declaration_the_mock_concrete_owns`, along the route `lookup_chain_declaration`, with no outer-edge collaborator.
