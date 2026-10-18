# `admit_selections` — interaction spec

Branch contract for the `admit_selections` module of the `harness-crypto` crate: `admit_selections`, the function the configuration document's selection texts are admitted into the adapter families' own types by — the randomness source, each family's concrete and identifier, the identity scope, the envelope algebra, the delivery-statement version, each curve's pairing concrete and target-group encoding identifier, each benchmarked pairing concrete and its target-group encoding identifier, and the network profile's precompile encodings — each text matched against its field's vocabulary and every admitted selection collected into the `HarnessSelections` the run composes from. Each branch states condition, decision, dependency call, and the exact return outcome.

## `admit_selections`

`admit_selections(deps: &AdmitSelectionsDeps, params: AdmitSelectionsParams, payload: AdmitSelectionsPayload<'_>) -> AdmitSelectionsReturn`: the payload is the document the deserializer proved the shape of, and this function proves its selections' content; `deps` and `params` carry nothing and are not read.

### Mapping

For every field in the vocabularies below, the decision is a `match` on the text as a `&str`, each listed text to its variant; any other text is refused with outcome `Err(AdmitSelectionsErrorReturn::UnknownValue { field, text })`, `field` the field's `SelectionField` variant and `text` a clone of the text.

| Field | `SelectionField` | Text | Variant |
|---|---|---|---|
| `random.source` | `RandomSource` | `"operating_system"` | `RandomSourceKind::OperatingSystem` |
| `encoding.concrete` | `EncodingConcrete` | `"abi"` | `EncodingConcrete::Abi` |
| `encoding.identifier` | `EncodingIdentifier` | `"ethereum_abi_v1"` | `EncodingIdentifier::EthereumAbiV1` |
| `hash_to_scalar.concrete` | `HashToScalarConcrete` | `"keccak256"` | `HashToScalarConcrete::Keccak256` |
| `hash_to_scalar.identifier` | `HashToScalarIdentifier` | `"keccak256_v1"` | `HashToScalarIdentifier::Keccak256V1` |
| `chain_forms.concrete` | `ChainFormsConcrete` | `"evm"` | `ChainFormsConcrete::Evm` |
| `chain_forms.identifier` | `ChainFormsIdentifier` | `"evm_v1"` | `ChainFormsIdentifier::EvmV1` |
| `kem.concrete` | `KemConcrete` | `"bb1_depth_one"` | `KemConcrete::Bb1DepthOne` |
| `kem.identifier` | `KemIdentifier` | `"bb1_depth_one_v1"` | `KemIdentifier::Bb1DepthOneV1` |
| `kem.scope` | `KemScope` | `"entitlement"` | `IdentityScope::Entitlement` |
| `kem.scope` | `KemScope` | `"asset"` | `IdentityScope::Asset` |
| `key_agreement.concrete` | `KeyAgreementConcrete` | `"pairing_elgamal"` | `KeyAgreementConcrete::PairingElGamal` |
| `key_agreement.identifier` | `KeyAgreementIdentifier` | `"pairing_elgamal_v1"` | `KeyAgreementIdentifier::PairingElGamalV1` |
| `key_agreement.algebra` | `KeyAgreementAlgebra` | `"pairing_elgamal"` | `EnvelopeAlgebra::PairingElGamal` |
| `delivery_proof.concrete` | `DeliveryProofConcrete` | `"schnorr_fs"` | `DeliveryProofConcrete::SchnorrFs` |

`delivery_proof.statement_version` is carried as it is; whether a version is supported is the delivery proof factory's admission.

| Field | `SelectionField` | Text | Variant |
|---|---|---|---|
| `curve.pairing` at `(index, curve)` of `payload.curves` | `CurvePairing { index }` | `"bn254_arkworks"` | `PairingConcrete::Bn254Arkworks` |
| `curve.pairing` at `(index, curve)` of `payload.curves` | `CurvePairing { index }` | `"bn254_halo2curves"` | `PairingConcrete::Bn254Halo2curves` |
| `curve.pairing` at `(index, curve)` of `payload.curves` | `CurvePairing { index }` | `"bls12_381_arkworks"` | `PairingConcrete::Bls12381Arkworks` |
| `curve.pairing` at `(index, curve)` of `payload.curves` | `CurvePairing { index }` | `"bls12_381_halo2curves"` | `PairingConcrete::Bls12381Halo2curves` |
| `curve.target_group_encoding` at `(index, curve)` of `payload.curves` | `CurveTargetGroupEncoding { index }` | `"bn254_v1"` | `TargetGroupEncodingIdentifier::Bn254V1` |
| `curve.target_group_encoding` at `(index, curve)` of `payload.curves` | `CurveTargetGroupEncoding { index }` | `"bls12_381_v1"` | `TargetGroupEncodingIdentifier::Bls12381V1` |
| `concrete.pairing` at `(index, concrete)` of `payload.benchmark_concretes` | `BenchmarkPairing { index }` | the four pairing texts the curves use | the four `PairingConcrete` variants |
| `concrete.target_group_encoding` at `(index, concrete)` of `payload.benchmark_concretes` | `BenchmarkTargetGroupEncoding { index }` | the two target-group encoding texts the curves use | the two `TargetGroupEncodingIdentifier` variants |
| `text` at `(index, text)` of `payload.precompile_encodings` | `PrecompileEncoding { index }` | `"eip196_eip197"` | `PrecompileEncoding::Eip196Eip197` |
| `text` at `(index, text)` of `payload.precompile_encodings` | `PrecompileEncoding { index }` | `"eip2537"` | `PrecompileEncoding::Eip2537` |

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| no curves | `payload.curves` is empty | the empty check | none | `Err(AdmitSelectionsErrorReturn::NoCurves)` |
| curves admitted | `payload.curves` holds at least one curve | the iteration over each `(index, curve)` in order | none | each admitted curve a `CurveSelection`, collected in order |
| benchmarked concretes admitted | `payload.benchmark_concretes`, each `(index, concrete)` taken in order | the iteration | none | each admitted concrete a `CurveSelection`, collected in order, an empty list admitted as an empty one |
| precompile encodings admitted | `payload.precompile_encodings`, each `(index, text)` taken in order | the iteration | none | each mapped `PrecompileEncoding` collected in order |
| computed | every field admitted | none | none | `Ok(AdmitSelectionsSuccessReturn { selections: HarnessSelections { random, encoding, hash_to_scalar, chain_forms, kem, key_agreement, delivery_proof, curves, benchmark_concretes, precompile_encodings } })` |

### Ordering

The family fields in the order listed above, then the empty-curve check, then the curves in index order with each curve's pairing before its target-group encoding, then the benchmarked concretes in index order with each concrete's pairing before its target-group encoding, then the precompile encodings in index order; the first refusal returns; no side effect.
