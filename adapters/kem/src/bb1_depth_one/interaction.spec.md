# `bb1_depth_one` — interaction spec

Branch contract for the `bb1_depth_one` module of the `kem` crate: the depth-one, all-wildcard Boneh–Boyen credential KEM concrete over any pairing concrete, under either identity scope. Each branch states condition, decision, dependency call, and the exact return outcome.

## `Bb1DepthOneKem::try_new(params: Bb1DepthOneKemConstructorParams<'a, P>) -> Bb1DepthOneKemTryNewReturn<'a, P>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| tag refused | `DomainTag::try_new(DomainTagConstructorParams { bytes: BB1_DEPTH_ONE_IDENTITY_TAG.to_vec() })` returns `Err(error)` | the constructor's result | `DomainTag::try_new` once | `Err(Bb1DepthOneKemTryNewErrorReturn::DomainTag(error))`, the refusal unchanged; the tag is 28 visible-ASCII bytes, so no input takes this branch and it has no unit test |
| admitted | the tag constructor succeeds | none | `DomainTag::try_new` once | `Ok(Bb1DepthOneKem { pairing, hash_to_scalar, tag })`, holding the params' borrows |

## `Bb1DepthOneKem::<'_, P>::DECLARATION`

The inherent constant `KemDeclaration { identifier: KemIdentifier::Bb1DepthOneV1, identity_scopes: &[IdentityScope::Entitlement, IdentityScope::Asset], identity_tag: BB1_DEPTH_ONE_IDENTITY_TAG, adapter_version: 1, interface_version: KEM_INTERFACE_VERSION }`, its tag the constant `try_new` constructs the adapter's `DomainTag` from; readable before any instance exists.

## Shared steps

**Sampling** — every sampling branch below names this step: `P::Scalar::sample_from_uniform_bytes(SampleUniformScalarParams, SampleUniformScalarPayload { uniform })` with the payload's uniform bytes moved in, its `Err(error)` returned in the method's own variant and its `Ok` yielding a `Secret<P::Scalar>` whose exposed value is cloned into each pairing payload that needs it.

**Identity mapping** — `setup` and `derive_identity` name this step: `self.hash_to_scalar.hash_to_scalar(HashToScalarParams { tag: &self.tag }, HashToScalarPayload { message: identity })` yielding `I`, its `Err(error)` returned in the method's `HashToScalar` variant; then `F = add_g1(u0, mul_g1(u1, I))`; then `is_identity_g1(F)`, `true` returned in the method's `TrivialIdentityElement` variant.

## `setup(&self, params: SetupParams<'_>, payload: SetupPayload) -> SetupReturn<Self::ParameterSet, Self::MasterScalar>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| sampling refused | the master, `u0`, or `u1` uniform bytes are refused, sampled in that order | the sampler's result, for the first refused | `P::Scalar::sample_from_uniform_bytes` up to three times | `Err(SetupErrorReturn::Bb1DepthOne(…))` holding `MasterScalarSampling`, `U0Sampling`, or `U1Sampling` with the refusal unchanged |
| entitlement scope | `params.scope` is `SetupScope::Entitlement` and all three samplings succeed | none | `g1_generator`, `g2_generator`, `mul_g1(g1, a)` for `u0`, `mul_g1(g1, b)` for `u1`, and `mul_g2(g2, α)` for `hpub`, each once | `Ok(SetupSuccessReturn { parameter_set, master_scalar })` with the scope `Bb1DepthOneParameterSetScope::Entitlement` and the master scalar the sampled `Secret` of `α` in `Bb1DepthOneMasterScalar`; the sampled `a` and `b` drop and zeroize |
| asset scope refused | `params.scope` is `SetupScope::Asset { identity }` and the identity mapping over `identity` and the new `u0`, `u1` fails | the mapping's result | `g1_generator`, `g2_generator`, the two `mul_g1`, `mul_g2(g2, α)`, then the identity mapping | `Err(SetupErrorReturn::Bb1DepthOne(Bb1DepthOneSetupErrorReturn::HashToScalar(error)))` or `Err(SetupErrorReturn::Bb1DepthOne(Bb1DepthOneSetupErrorReturn::TrivialIdentityElement))`; `a` and `b` are uniform, so `F` is trivial with negligible probability and the hash refuses no tag this concrete holds, so neither branch has a unit test |
| asset scope | `params.scope` is `SetupScope::Asset { identity }` and the identity mapping succeeds | none | as above | `Ok` as the entitlement scope with the scope `Bb1DepthOneParameterSetScope::Asset { identity_element: F }` |

## `derive_identity(&self, params: DeriveIdentityParams, payload: DeriveIdentityPayload<'_, Self::ParameterSet>) -> DeriveIdentityReturn<Self::IdentityElement>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| refused by the hash or trivial | the identity mapping over `payload.identity` and the parameter set's `u0`, `u1` fails | the mapping's result | the identity mapping | `Err(DeriveIdentityErrorReturn::Bb1DepthOne(…))` holding `HashToScalar(error)` unchanged or `TrivialIdentityElement` (LC-08) |
| outside the asset scope | the parameter set's scope is `Asset { identity_element }` and `encode_g1(F)` differs from `encode_g1(identity_element)` | byte equality of the two public encodings | the identity mapping, then `encode_g1` twice | `Err(DeriveIdentityErrorReturn::Bb1DepthOne(Bb1DepthOneDeriveIdentityErrorReturn::OutsideAssetScope))` |
| derived | the mapping succeeds and, under the asset scope, the encodings are equal | none | the identity mapping, then `encode_g1` twice under the asset scope | `Ok(DeriveIdentitySuccessReturn { identity_element: Bb1DepthOneIdentityElement { scalar: I, element: F } })` |

## `issue(&self, params: IssueParams, payload: IssuePayload<'_, Self::ParameterSet, Self::MasterScalar, Self::IdentityElement>) -> IssueReturn<Self::Credential, S>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| sampling refused | `payload.uniform` is refused | the sampler's result | `P::Scalar::sample_from_uniform_bytes` once | `Err(IssueErrorReturn::Bb1DepthOne(Bb1DepthOneIssueErrorReturn::Sampling(error)))`, the refusal unchanged |
| issued | sampling succeeds | none | `msm_g1` over the terms `(g1, α)` and `(F, r)` for `A` and `mul_g2(g2, r)` for `B`, each once | `Ok(IssueSuccessReturn { credential: Bb1DepthOneCredential { a, b }, randomness })`, `randomness` the sampled `Secret` of `r` moved without copy |

## `rerandomize(&self, params: RerandomizeParams, payload: RerandomizePayload<'_, Self::ParameterSet, Self::IdentityElement, Self::Credential>) -> RerandomizeReturn<Self::Credential, S>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| sampling refused | `payload.uniform` is refused | the sampler's result | `P::Scalar::sample_from_uniform_bytes` once | `Err(RerandomizeErrorReturn::Bb1DepthOne(Bb1DepthOneRerandomizeErrorReturn::Sampling(error)))`, the refusal unchanged |
| rerandomized | sampling succeeds | none | `mul_g1(F, s)`, then `add_g1(A, s·F)`, `mul_g2(g2, s)`, then `add_g2(B, s·g2)` | `Ok(RerandomizeSuccessReturn { credential, offset })` holding the new credential and the sampled `Secret` of `s` moved without copy; the master scalar is not an input; the given credential is unchanged |

## `is_valid(&self, params: IsValidParams, payload: IsValidPayload<'_, Self::ParameterSet, Self::IdentityElement, Self::Credential>) -> IsValidReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| checked | any params and payload | none | `neg_g1(g1)`, `neg_g1(F)`, then `pairing_product_is_one` over the terms `(A, g2)`, `(-g1, hpub)`, and `(-F, B)`, once | `Ok(IsValidSuccessReturn { is_valid })`, `is_one` unchanged |

## `encapsulate(&self, params: EncapsulateParams, payload: EncapsulatePayload<'_, Self::ParameterSet>) -> EncapsulateReturn<Self::Capsule>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| sampling refused | `payload.uniform` is refused | the sampler's result | `P::Scalar::sample_from_uniform_bytes` once | `Err(EncapsulateErrorReturn::Bb1DepthOne(Bb1DepthOneEncapsulateErrorReturn::Sampling(error)))`, the refusal unchanged |
| encapsulated | sampling succeeds | the parameter set's scope selects the capsule form | `mul_g2(g2, t)` for `U`; under the entitlement scope `mul_g1(u0, t)` for `V` and `mul_g1(u1, t)` for `W`; under the asset scope `mul_g1(F, t)` for `V`; then `mul_g1(g1, t)`, `pairing_product` over the one term `(t·g1, hpub)`, and `encode_gt` | `Ok(EncapsulateSuccessReturn { capsule, encapsulated: EncapsulatedValue { bytes } })`, the capsule `Bb1DepthOneCapsule::Entitlement { u, v, w }` or `Bb1DepthOneCapsule::Asset { u, v }` by scope; the sampled `t` and the target-group value drop and zeroize |

## `is_well_formed(&self, params: IsWellFormedParams, payload: IsWellFormedPayload<'_, Self::ParameterSet, Self::Capsule>) -> IsWellFormedReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| entitlement | the parameter set's scope is `Entitlement` and the capsule is `Entitlement { u, v, w }` | the conjunction of both `is_one` | `neg_g1(u0)`, `pairing_product_is_one` over `(v, g2)` and `(-u0, u)`, then `neg_g1(u1)`, `pairing_product_is_one` over `(w, g2)` and `(-u1, u)` | `Ok(IsWellFormedSuccessReturn { is_well_formed })` |
| asset | the scope is `Asset { identity_element }` and the capsule is `Asset { u, v }` | `is_one` | `neg_g1(identity_element)`, `pairing_product_is_one` over `(v, g2)` and `(-identity_element, u)` | `Ok(IsWellFormedSuccessReturn { is_well_formed })` |
| scope mismatch | the capsule's variant is not the parameter set's scope | the mismatch | none | `Ok(IsWellFormedSuccessReturn { is_well_formed: false })` |

## `decapsulate(&self, params: DecapsulateParams, payload: DecapsulatePayload<'_, Self::IdentityElement, Self::Credential, Self::Capsule>) -> DecapsulateReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| entitlement | the capsule is `Entitlement { u, v, w }` | none | `mul_g1(w, I)`, `add_g1(v, I·w)`, `neg_g1` of that sum, `pairing_product` over `(A, u)` and `(-(v + I·w), B)`, and `encode_gt` | `Ok(DecapsulateSuccessReturn { encapsulated: EncapsulatedValue { bytes } })` |
| asset | the capsule is `Asset { u, v }` | none | `neg_g1(v)`, `pairing_product` over `(A, u)` and `(-v, B)`, and `encode_gt` | `Ok(DecapsulateSuccessReturn { encapsulated: EncapsulatedValue { bytes } })`; the identity element is not read, since the asset's `F` is fixed in `v` |

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
