# `lookup_pairing_declaration` — interaction spec

Branch contract for the `lookup_pairing_declaration` module of the `pairing` crate: the family's declaration lookup, returning the declaration of the member a `PairingConcrete` identifier names, read from the member's interface-held constant without constructing the member. Each branch states condition, decision, dependency call, and the exact return outcome.

## `lookup_pairing_declaration(deps: &LookupPairingDeclarationDeps, params: LookupPairingDeclarationParams, payload: LookupPairingDeclarationPayload) -> LookupPairingDeclarationReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| `bn254_arkworks` | `params.concrete` is `PairingConcrete::Bn254Arkworks` | the exhaustive `match` on `params.concrete` | none; the constant `Bn254ArkworksPairing::DECLARATION` is read as a value | `Ok(LookupPairingDeclarationSuccessReturn { declaration })` holding that constant's value |
| `bn254_halo2curves` | `params.concrete` is `PairingConcrete::Bn254Halo2curves` | the same `match` | none; the constant `Bn254Halo2curvesPairing::DECLARATION` is read as a value | `Ok(LookupPairingDeclarationSuccessReturn { declaration })` holding that constant's value |
| `bls12_381_arkworks` | `params.concrete` is `PairingConcrete::Bls12381Arkworks` | the same `match` | none; the constant `Bls12381ArkworksPairing::DECLARATION` is read as a value | `Ok(LookupPairingDeclarationSuccessReturn { declaration })` holding that constant's value |
| `bls12_381_halo2curves` | `params.concrete` is `PairingConcrete::Bls12381Halo2curves` | the same `match` | none; the constant `Bls12381Halo2curvesPairing::DECLARATION` is read as a value | `Ok(LookupPairingDeclarationSuccessReturn { declaration })` holding that constant's value |
| `mock` — present only under `#[cfg(any(test, feature = "mocks"))]` | `params.concrete` is `PairingConcrete::Mock(_)` | the same `match` | none; `MOCK_I_PAIRING_ADAPTER_DECLARATION` is read as a value | `Ok(LookupPairingDeclarationSuccessReturn { declaration })` where `declaration.concrete` is `params.concrete`, the identifier the caller looked up, and every other field is that field of `MOCK_I_PAIRING_ADAPTER_DECLARATION` |

## Ordering and edges

- The function is a single exhaustive `match` with one arm per member of `PairingConcrete`, so a variant without an arm does not compile.
- The function constructs no concrete and reads no instance, so the declaration of a member is read before the member exists.
- `deps` and `payload` carry no value and are not read.
- The error arm has no branch, `Infallible` being uninhabited.
- A further concrete adds one arm in its own increment.

## Own entries

- `declaration_names_the_member_looked_up`: condition `params.concrete` is any member of `PAIRING_CONCRETES`; outcome `Ok` whose `declaration.concrete` is that member; input variation every member of the declared set, so a lookup that returns one fixed member's declaration fails; edge that must survive composition: the declaration is read through the crate's public surface with no concrete constructed.
- `mock_declaration_names_the_identifier_looked_up`, present only under `#[cfg(feature = "mocks")]`: condition `params.concrete` is `PairingConcrete::Mock(mode)` with a mode other than `Succeeds`; outcome `Ok` whose `declaration.concrete` is `PairingConcrete::Mock(mode)`; input variation the mode; edge: the mock member is absent from `PAIRING_CONCRETES` and reached only by naming it.

## Callee dispositions

The declaration constants of the concretes this node reads state no integration entry, so no entry is absorbed or carried.

## Route

`lookup_pairing_declaration` reads the `DECLARATION` constant of the concrete the identifier names, or `MOCK_I_PAIRING_ADAPTER_DECLARATION` for the mock member; no outer-edge collaborator is on the route.

## Public surface

An outside caller invokes `lookup_pairing_declaration` with fieldless deps, params carrying a `PairingConcrete`, and a fieldless payload, each built through the official mocks, and observes the `Ok` declaration's `concrete`. The entries `declaration_names_the_member_looked_up` and `mock_declaration_names_the_identifier_looked_up` are proven by the public integration test.
