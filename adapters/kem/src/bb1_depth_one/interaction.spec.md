# `bb1_depth_one` — interaction spec

Branch contract for the `bb1_depth_one` module of the `kem` crate: the depth-one, all-wildcard Boneh–Boyen credential KEM concrete over any pairing concrete, under either identity scope. Each branch states condition, decision, dependency call, and the exact return outcome.

## `Bb1DepthOneKem::try_new(params: Bb1DepthOneKemConstructorParams<'a, P>) -> Bb1DepthOneKemTryNewReturn<'a, P>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| tag refused | `DomainTag::try_new(DomainTagConstructorParams { bytes: BB1_DEPTH_ONE_IDENTITY_TAG.to_vec() })` returns `Err(error)` | the constructor's result | `DomainTag::try_new` once | `Err(Bb1DepthOneKemTryNewErrorReturn::DomainTag(error))`, the refusal unchanged; the tag is 28 visible-ASCII bytes, so no input takes this branch and it has no unit test |
| admitted | the tag constructor succeeds | none | `DomainTag::try_new` once | `Ok(Bb1DepthOneKem { pairing, hash_to_scalar, tag })`, holding the params' borrows |

## `Bb1DepthOneKem::<'_, P>::DECLARATION`

The inherent constant `KemDeclaration = BB1_DEPTH_ONE_DECLARATION`, its tag the constant `try_new` constructs the adapter's `DomainTag` from; readable before any instance exists.

## Shared steps

**Sampling** — every sampling branch below names this step: `P::Scalar::sample_from_uniform_bytes(SampleUniformScalarParams, SampleUniformScalarPayload { uniform })` with the payload's uniform bytes moved in, its `Err(error)` returned in the method's sampling flavor and its `Ok` yielding a `Secret<P::Scalar>` whose exposed value is cloned into each pairing payload that needs it.

**Identity mapping** — `setup` and `derive_identity` name this step: borrow `canonical` from `KemIdentity::Entitlement` or call `root.as_bytes()` on `KemIdentity::Asset` only when building `HashToScalarPayload`, the setup asset root being the latter; `self.hash_to_scalar.hash_to_scalar(HashToScalarParams { tag: &self.tag }, HashToScalarPayload { message: bytes })` yields `I`, its `Err(error)` returned in the method's `HashToScalar` flavor; then `F = add_g1(u0, mul_g1(u1, I))`; then `is_identity_g1(F)`, `true` returned in the method's `TrivialIdentityElement` flavor.

## `setup(&self, params: SetupParams<'_>, payload: SetupPayload) -> SetupReturn<Self::ParameterSet, Self::MasterScalar>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| sampling refused | the master, `u0`, or `u1` uniform bytes are refused, sampled in that order | the sampler's result, for the first refused | `P::Scalar::sample_from_uniform_bytes` up to three times | `Err(SetupErrorReturn::MasterScalarSampling(error))`, `Err(SetupErrorReturn::U0Sampling(error))`, or `Err(SetupErrorReturn::U1Sampling(error))`, the refusal unchanged, for the first refused |
| entitlement scope | `params.scope` is `SetupScope::Entitlement` and all three samplings succeed | none | `g1_generator`, `g2_generator`, `mul_g1(g1, a)` for `u0`, `mul_g1(g1, b)` for `u1`, and `mul_g2(g2, α)` for `hpub`, each once | `Ok(SetupSuccessReturn { parameter_set, master_scalar })` with the scope `Bb1DepthOneParameterSetScope::Entitlement` and the master scalar the sampled `Secret` of `α` in `Bb1DepthOneMasterScalar`; the sampled `a` and `b` drop and zeroize |
| asset scope refused | `params.scope` is `SetupScope::Asset { root }` and the identity mapping over `KemIdentity::Asset { root }` and the new `u0`, `u1` fails, by the hash adapter returning `Err(error)` or by `F` being the identity element, which zero uniform bytes for `a` and `b` produce by making `u0` and `u1` the identity element | the mapping's result | `g1_generator`, `g2_generator`, the two `mul_g1`, `mul_g2(g2, α)`, then the identity mapping | `Err(SetupErrorReturn::HashToScalar(error))`, the refusal unchanged, or `Err(SetupErrorReturn::TrivialIdentityElement)` |
| asset scope | `params.scope` is `SetupScope::Asset { root }` and the identity mapping succeeds | none | as above | `Ok` as the entitlement scope with the scope `Bb1DepthOneParameterSetScope::Asset { identity_element: F }` |

## `derive_identity(&self, params: DeriveIdentityParams, payload: DeriveIdentityPayload<'_, Self::ParameterSet>) -> DeriveIdentityReturn<Self::IdentityElement>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| wrong role | the parameter set's scope is `Entitlement` and `payload.identity` is `KemIdentity::Asset`, or its scope is `Asset` and the identity is `KemIdentity::Entitlement` | the mismatch | none | `Err(DeriveIdentityErrorReturn::WrongIdentityScope)` before hashing |
| refused by the hash or trivial | the roles match and the identity mapping over `payload.identity` and the parameter set's `u0`, `u1` fails | the mapping's result | the identity mapping | `Err(DeriveIdentityErrorReturn::HashToScalar(error))` unchanged or `Err(DeriveIdentityErrorReturn::TrivialIdentityElement)` (LC-08) |
| outside the asset scope | the parameter set's scope is `Asset { identity_element }` and `encode_g1(F)` differs from `encode_g1(identity_element)` | equality of the two `P::EncodedG1` values, without discarding their type into byte vectors | the identity mapping, then `encode_g1` twice | `Err(DeriveIdentityErrorReturn::OutsideAssetScope)` |
| derived | the mapping succeeds and, under the asset scope, the encodings are equal | none | the identity mapping, then `encode_g1` twice under the asset scope | `Ok(DeriveIdentitySuccessReturn { identity_element: Bb1DepthOneIdentityElement { scalar: I, element: F } })` |

## `issue(&self, params: IssueParams, payload: IssuePayload<'_, Self::ParameterSet, Self::MasterScalar, Self::IdentityElement>) -> IssueReturn<Self::Credential, S>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| sampling refused | `payload.uniform` is refused | the sampler's result | `P::Scalar::sample_from_uniform_bytes` once | `Err(IssueErrorReturn::Sampling(error))`, the refusal unchanged |
| issued | sampling succeeds | none | `msm_g1` over the terms `(g1, α)` and `(F, r)` for `A` and `mul_g2(g2, r)` for `B`, each once | `Ok(IssueSuccessReturn { credential: Bb1DepthOneCredential { a, b }, randomness })`, `randomness` the sampled `Secret` of `r` moved without copy |

## `rerandomize(&self, params: RerandomizeParams, payload: RerandomizePayload<'_, Self::ParameterSet, Self::IdentityElement, Self::Credential>) -> RerandomizeReturn<Self::Credential, S>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| sampling refused | `payload.uniform` is refused | the sampler's result | `P::Scalar::sample_from_uniform_bytes` once | `Err(RerandomizeErrorReturn::Sampling(error))`, the refusal unchanged |
| rerandomized | sampling succeeds | none | `mul_g1(F, s)`, then `add_g1(A, s·F)`, `mul_g2(g2, s)`, then `add_g2(B, s·g2)` | `Ok(RerandomizeSuccessReturn { credential, offset })` holding the new credential and the sampled `Secret` of `s` moved without copy; the master scalar is not an input; the given credential is unchanged |

## `is_valid(&self, params: IsValidParams, payload: IsValidPayload<'_, Self::ParameterSet, Self::IdentityElement, Self::Credential>) -> IsValidReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| checked | any params and payload | none | `neg_g1(g1)`, `neg_g1(F)`, then `pairing_product_is_one` over the terms `(A, g2)`, `(-g1, hpub)`, and `(-F, B)`, once | `Ok(IsValidSuccessReturn { is_valid })`, `is_one` unchanged |

## `encapsulate(&self, params: EncapsulateParams, payload: EncapsulatePayload<'_, Self::ParameterSet>) -> EncapsulateReturn<Self::Capsule>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| sampling refused | `payload.uniform` is refused | the sampler's result | `P::Scalar::sample_from_uniform_bytes` once | `Err(EncapsulateErrorReturn::Sampling(error))`, the refusal unchanged |
| encapsulated | sampling succeeds | the parameter set's scope selects the capsule form | `mul_g2(g2, t)` for `U`; under the entitlement scope `mul_g1(u0, t)` for `V` and `mul_g1(u1, t)` for `W`; under the asset scope `mul_g1(F, t)` for `V`; then `mul_g1(g1, t)`, `pairing_product` over the one term `(t·g1, hpub)`, and `encode_gt`; copy `Secret<P::EncodedGt>::expose().as_ref()` once into the private `EncapsulatedValue` secret buffer | `Ok(EncapsulateSuccessReturn { capsule, encapsulated })`, the capsule `Bb1DepthOneCapsule::Entitlement { u, v, w }` or `Bb1DepthOneCapsule::Asset { u, v }` by scope; the sampled `t`, the encoded target-group value, and the target-group value drop and zeroize |

## `is_well_formed(&self, params: IsWellFormedParams, payload: IsWellFormedPayload<'_, Self::ParameterSet, Self::Capsule>) -> IsWellFormedReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| entitlement | the parameter set's scope is `Entitlement` and the capsule is `Entitlement { u, v, w }` | the conjunction of both `is_one` | `neg_g1(u0)`, `pairing_product_is_one` over `(v, g2)` and `(-u0, u)`, then `neg_g1(u1)`, `pairing_product_is_one` over `(w, g2)` and `(-u1, u)` | `Ok(IsWellFormedSuccessReturn { is_well_formed })` |
| asset | the scope is `Asset { identity_element }` and the capsule is `Asset { u, v }` | `is_one` | `neg_g1(identity_element)`, `pairing_product_is_one` over `(v, g2)` and `(-identity_element, u)` | `Ok(IsWellFormedSuccessReturn { is_well_formed })` |
| scope mismatch | the capsule's variant is not the parameter set's scope | the mismatch | none | `Ok(IsWellFormedSuccessReturn { is_well_formed: false })` |

## `decapsulate(&self, params: DecapsulateParams, payload: DecapsulatePayload<'_, Self::IdentityElement, Self::Credential, Self::Capsule>) -> DecapsulateReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| entitlement | the capsule is `Entitlement { u, v, w }` | none | `mul_g1(w, I)`, `add_g1(v, I·w)`, `neg_g1` of that sum, `pairing_product` over `(A, u)` and `(-(v + I·w), B)`, and `encode_gt`; copy the typed encoded target-group bytes into the private `EncapsulatedValue` as in `encapsulate` | `Ok(DecapsulateSuccessReturn { encapsulated })` |
| asset | the capsule is `Asset { u, v }` | none | `neg_g1(v)`, `pairing_product` over `(A, u)` and `(-v, B)`, and `encode_gt`; the copy as in `encapsulate` | `Ok(DecapsulateSuccessReturn { encapsulated })`; the identity element is not read, since the asset's `F` is fixed in `v` |

## Component access

Each method is one branch with no dependency call and each outcome `Ok`:

| Method | Outcome |
|---|---|
| `credential_components` | `Ok` holding `CredentialComponents { a, b }`, clones of the credential's `a` and `b` |
| `credential_from_components` | `Ok` holding `Bb1DepthOneCredential { a, b }`, the components moved |
| `parameter_set_components` | `Ok` holding clones of `g1`, `u0`, `u1`, `g2`, `hpub`, and the scope, `Bb1DepthOneParameterSetScope::Entitlement` as `ParameterSetScopeComponents::Entitlement` and `Asset { identity_element }` as `ParameterSetScopeComponents::Asset { identity_element }` with a clone |
| `parameter_set_from_components` | `Ok` holding the parameter set holding the components moved, the scope mapped back the same way |
| `identity_element_components` | `Ok` holding clones of `scalar` and `element` |
| `capsule_components` | `Ok` holding clones of the capsule's elements in the like-named `CapsuleComponents` variant |
| `capsule_from_components` | `Ok` holding the `Bb1DepthOneCapsule` variant holding the components moved |
| `master_scalar_components` | `Ok` holding `MasterScalarComponents { value }`, `value` a new `Secret` from `Secret::try_new(SecretConstructorParams { value: self.value.expose().clone() })` unpacked irrefutably |
| `master_scalar_from_components` | `Ok` holding `Bb1DepthOneMasterScalar { value }`, the `Secret` moved |

## Ordering and invariants

- Every sampling precedes the arithmetic it feeds, and setup samples `α`, `a`, then `b`; the identity mapping hashes before it multiplies, and the trivial test precedes the asset-scope comparison; every point and scalar a pairing payload consumes is a clone of a held value, and the held values are unchanged; `params` carries no control and is not read in any method but `setup`.
- The same inputs always yield the same outputs; every credential `issue` or `rerandomize` returns passes `is_valid` for its identity element under its parameter set; every capsule `encapsulate` returns passes `is_well_formed` under its parameter set, and every valid credential decapsulates it to `encapsulate`'s bytes; every value rebuilt from its own components behaves as the value it was read from; `B` is `randomness·g2` for an issued credential and grows by `offset·g2` under rerandomization.

## Integration: callee dispositions

Each entry of `pairing/factory`, `hash-to-scalar/factory`, and `hash-to-scalar/domain_tag` whose route passes through a method of this node or through `Bb1DepthOneKem::try_new` is carried; every route ends at a method of this node or at `Bb1DepthOneKem::try_new`, which holds the borrowed collaborator, and no outer-edge collaborator is on any route, every random scalar being caller-supplied uniform bytes.

- `pairing/factory` entry `the_consumer_receives_the_concrete_the_params_name`: carried — restated in this node's terms: the pairing `Bb1DepthOneKem::try_new` holds is the concrete the params name, every `pairing` call a method makes landing on it. Route: `create_pairing`, the arm's concrete through its `try_new`, `Bb1DepthOneKem::try_new`.
- `pairing/factory` entry `a_sampled_scalar_encodes_through_secret`: carried — restated in this node's terms: `setup`, `issue`, `rerandomize`, and `encapsulate` move the payload's uniform `Secret<Vec<u8>>` into `sample_from_uniform_bytes` and hold each sampled scalar in a `Secret`, the master scalar, the issuance `randomness`, and the rerandomization `offset` reaching the caller as `Secret<P::Scalar>`. Route: `create_pairing`, the arm's concrete through its `try_new`, `Bb1DepthOneKem::try_new`, then the method.
- `pairing/factory` entry `a_sampling_refusal_reaches_the_caller_unchanged`: carried — restated in this node's terms: the refusal `WrongLength { expected, actual }` reaches the caller whole inside the method's flavor, `MasterScalarSampling`, `U0Sampling`, `U1Sampling`, or `Sampling`. Route: as `a_sampled_scalar_encodes_through_secret`.
- `pairing/factory` entry `libraries_sharing_an_identifier_encode_the_pairing_product_to_the_same_bytes`: carried — restated in this node's terms: `setup` then `encapsulate` over identical uniform bytes under any two concretes whose declared target-group encoding identifiers are equal return `EncapsulatedValue`s whose `key_material()` bytes are equal. Route: `create_pairing`, the arm's concrete through its `try_new`, `Bb1DepthOneKem::try_new`, `setup`, `encapsulate`, `pairing_product`, and `encode_gt`.
- `hash-to-scalar/factory` entry `every_concrete_declaring_the_keccak256_identifier_hashes_the_reference_inputs_to_the_oracle`: carried — restated in this node's terms: the scalar `identity_element_components` returns for an identity encodes to the encoding of the scalar the received scalar type samples from `expected_uniform_bytes(&tag, identity bytes, S::UNIFORM_BYTES_LENGTH)`, `tag` the tag of `BB1_DEPTH_ONE_IDENTITY_TAG`. Route: `create_pairing`, the arm's concrete through its `try_new`, `create_hash_to_scalar`, the selected concrete's `try_new`, `Bb1DepthOneKem::try_new`, `derive_identity`, and `identity_element_components`.
- `hash-to-scalar/factory` entry `every_concrete_returns_the_sampling_bound_refusal_whole`: carried — restated in this node's terms: the refusal reaches the caller of `setup` under the asset scope and of `derive_identity` whole inside the method's `HashToScalar` flavor. Route: `create_pairing`, the arm's concrete through its `try_new`, `create_hash_to_scalar` on the mock selection under `SamplingRefused`, `Bb1DepthOneKem::try_new`, then the method.
- `hash-to-scalar/domain_tag` entry `an_admitted_tag_reads_back_through_the_crate_root`: carried — restated in this node's terms: the bytes `as_bytes()` returns for the tag `Bb1DepthOneKem::try_new` builds are the bytes of `BB1_DEPTH_ONE_DECLARATION.identity_tag`. Route: the crate barrel `lib.rs`, `domain_tag::provides`, `DomainTag::try_new`, `DomainTag::as_bytes`, and `Bb1DepthOneKem::try_new`.
- `hash-to-scalar/factory` entry `every_concrete_declaring_the_keccak256_identifier_hashes_the_context_the_encoding_factory_encodes_to_the_oracle`: carried — restated in this node's terms: `derive_identity` hashes the canonical identity bytes the caller supplies unchanged, so the scalar `identity_element_components` returns for the bytes `create_encoding` returns for a derivation context encodes to the encoding of the scalar the received scalar type samples from `expected_uniform_bytes(&tag, bytes, S::UNIFORM_BYTES_LENGTH)`. Route: `create_encoding`, `create_pairing`, the arm's concrete through its `try_new`, `create_hash_to_scalar`, the selected concrete's `try_new`, `Bb1DepthOneKem::try_new`, `derive_identity`, and `identity_element_components`.

## Integration: own entries

Each entry is named and proven by the private integration block of the same name in `integration_test.rs`, each run over every concrete of `PAIRING_CONCRETES` and every case of `DECLARED_HASH_TO_SCALAR_SELECTIONS`, with every random scalar supplied as uniform bytes, a distinct value for each sampler so a reused or reordered scalar breaks the relation.

- `every_credential_issued_under_a_setup_parameter_set_is_valid_for_its_identity`: condition `setup` under a scope, `derive_identity` over an identity of that scope, and `issue` with the master scalar `setup` returned; outcome `is_valid` returns `Ok(IsValidSuccessReturn { is_valid: true })`; variation the entitlement scope with `KemIdentity::Entitlement { canonical }` and the asset scope with `KemIdentity::Asset { root }`; edge `α·g1`, `hpub`, `F`, and `r` agree across the methods and the pairing-product check.
- `a_rerandomized_credential_is_valid_for_its_identity`: condition `rerandomize` over a credential `issue` returned, with no master scalar; outcome `is_valid` returns `true` for the rerandomized credential and for the credential it was made from; variation each scope; edge the offset `s` is the one `rerandomize` returned and differs from `r`.
- `every_valid_credential_decapsulates_a_well_formed_capsule_to_the_encapsulated_bytes`: condition `encapsulate` under the parameter set, then `decapsulate` with the issued credential and with the rerandomized credential; outcome `is_well_formed` returns `true` for the capsule, and each `decapsulate` returns `key_material()` bytes equal to the bytes `encapsulate` returned; variation each scope, and under the asset scope an identity element of another asset identity passed to `decapsulate`, which the asset branch does not read; edge `K` is encoded alike on both paths.
- `credentials_of_different_entitlements_under_one_parameter_set_recover_the_same_bytes`: condition the entitlement scope, entitlements each `issue`d a credential, and `decapsulate` of one capsule with each; outcome each returns `key_material()` bytes equal to the bytes `encapsulate` returned; variation distinct canonical identity encodings; edge the identity scalar `I` differs between the entitlements, so a decapsulation that dropped `I·W` fails.
- `a_credential_is_invalid_for_another_identity_and_under_another_parameter_set`: condition `is_valid` over a credential `issue` returned, with the identity element of another identity and, separately, under a parameter set `setup` returned for other uniform bytes; outcome `Ok(IsValidSuccessReturn { is_valid: false })` for each of them; variation each scope, the other identity another canonical encoding or another asset root; edge the same credential is valid for its own identity under its own parameter set.
- `a_capsule_with_exchanged_identity_base_elements_is_ill_formed`: condition the entitlement scope, a capsule `encapsulate` returned, read by `capsule_components`, its `v` and `w` exchanged, and rebuilt by `capsule_from_components`; outcome `is_well_formed` returns `false` for the rebuilt capsule and `true` for the original; variation the exchanged and the unexchanged capsule; edge the exchange is the only difference between them.
- `a_capsule_is_ill_formed_under_another_parameter_set`: condition `is_well_formed` over a capsule `encapsulate` returned, under a parameter set `setup` returned for other uniform bytes; outcome `false`, and `true` under its own parameter set; variation each scope; edge the same capsule is well formed under its own parameter set.
- `a_capsule_of_the_other_scope_is_ill_formed`: condition `is_well_formed` over an asset capsule under an entitlement parameter set and over an entitlement capsule under an asset parameter set, each `encapsulate` having returned its capsule under a parameter set of its own scope; outcome `Ok(IsWellFormedSuccessReturn { is_well_formed: false })`; variation each direction; edge each capsule is well formed under its own scope's set.
- `derive_identity_refuses_a_trivial_identity_element`: condition an entitlement parameter set `setup` returned for zero uniform bytes for `u0` and `u1`, so `u0` and `u1` are the identity element, and `derive_identity` over an entitlement identity; outcome `Err(DeriveIdentityErrorReturn::TrivialIdentityElement)`; variation zero bytes against the default bytes, under which the same call returns `Ok`; edge `F` is the identity element whatever `I` is.
- `derive_identity_derives_under_the_asset_scope_only_the_identity_the_set_fixes`: condition an asset parameter set `setup` returned for an asset root, and `derive_identity` over `KemIdentity::Asset` of that root and of another root; outcome `Ok(DeriveIdentitySuccessReturn { identity_element })` for the root the set fixes and `Err(DeriveIdentityErrorReturn::OutsideAssetScope)` for the other; variation the root the set fixes and another root; edge the encodings compared are the pairing's own `P::EncodedG1` values.
- `every_value_rebuilt_from_its_components_behaves_as_the_value_it_was_read_from`: condition the parameter set, credential, capsule, and master scalar each read by its components method and rebuilt by its from-components method; outcome `issue` under the rebuilt parameter set with the rebuilt master scalar returns a credential `is_valid` accepts under the original set, the rebuilt credential is valid and decapsulates to the encapsulated bytes, and the rebuilt capsule is well formed under the original set; variation each scope; edge a swapped field of any component breaks the relation.

## Integration: private surface

The chain of real functions `create_pairing`, the arm's concrete through its `try_new`, `create_hash_to_scalar`, the selected concrete's `try_new` and `hash_to_scalar`, `Bb1DepthOneKem::try_new`, and every method of `Bb1DepthOneKem`, with `Secret` of `domain` running for real; the cases are every concrete of `PAIRING_CONCRETES` over every case of `DECLARED_HASH_TO_SCALAR_SELECTIONS`; no outer-edge collaborator is mocked, the uniform bytes being caller-supplied values; the observable result is each method's `Result`, the verdict of `is_valid` and `is_well_formed`, and the bytes `key_material()` exposes; the entries proven are the own entries above.

