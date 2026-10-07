# `bls12_381_arkworks` — interaction spec

Branch contract for the `bls12_381_arkworks` module of the `pairing` crate: the pairing adapter over arkworks' BLS12-381, encoding group elements as EIP-2537 precompile input. Each branch states condition, decision, dependency call, and the exact return outcome.

## `Bls12381ArkworksPairing::try_new(params: Bls12381ArkworksPairingConstructorParams) -> Bls12381ArkworksPairingTryNewReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| construct | any params | none | `Fr::from(3u64)` as `multiple`; `let mut exponent = Fr::MODULUS;` then `let _ = exponent.sub_with_borrow(&BigInt::from(2u64));`, the group order minus two; `multiple.pow(exponent)` as `reduced_pairing_correction` | `Ok(Bls12381ArkworksPairing { reduced_pairing_correction })` |

The error arm has no branch: the adapter takes no configuration, so `Infallible` is uninhabited.

## `Bls12381ArkworksPairing::DECLARATION`

An inherent constant, readable from the type before any instance exists:

`PairingDeclaration { curve: PairingCurve::Bls12381, verifier_group_arithmetic: VerifierGroupArithmetic::BothGroups, precompile_encoding: PrecompileEncoding::Eip2537, target_group_encoding: TargetGroupEncodingIdentifier::Bls12381V1, adapter_version: 1, interface_version: PAIRING_INTERFACE_VERSION }`

The trait constant `IPairingAdapter::DECLARATION` is this constant.

## `Bls12381ArkworksPairing::CONCRETE`

The trait constant `PairingConcrete::Bls12381Arkworks`.

## `g1_generator(&self, params: G1GeneratorParams, payload: G1GeneratorPayload) -> G1GeneratorReturn<Bls12381ArkworksG1>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| generated | any | none | `G1Affine::generator()` | `Ok(G1GeneratorSuccessReturn { point })`, the generator in the owned group type |

## `g2_generator(&self, params: G2GeneratorParams, payload: G2GeneratorPayload) -> G2GeneratorReturn<Bls12381ArkworksG2>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| generated | any | none | `G2Affine::generator()` | `Ok(G2GeneratorSuccessReturn { point })`, the generator in the owned group type |

## `add_g1(&self, params: AddG1Params, payload: AddG1Payload<Bls12381ArkworksG1>) -> AddG1Return<Bls12381ArkworksG1>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| summed | any | none | the affine `+` of `payload.left.value` and `payload.right.value`, then `into_affine` | `Ok(AddG1SuccessReturn { sum })` |

## `add_g2(&self, params: AddG2Params, payload: AddG2Payload<Bls12381ArkworksG2>) -> AddG2Return<Bls12381ArkworksG2>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| summed | any | none | the affine `+` of `payload.left.value` and `payload.right.value`, then `into_affine` | `Ok(AddG2SuccessReturn { sum })` |

## `mul_g1(&self, params: MulG1Params, payload: MulG1Payload<Bls12381ArkworksG1, Bls12381ArkworksScalar>) -> MulG1Return<Bls12381ArkworksG1>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| multiplied | any | none | the affine `payload.point.value` `*` the scalar's `Fr`, then `into_affine` | `Ok(MulG1SuccessReturn { product })`; the payload, holding the scalar, drops at the end of the call and the scalar is zeroized |

## `mul_g2(&self, params: MulG2Params, payload: MulG2Payload<Bls12381ArkworksG2, Bls12381ArkworksScalar>) -> MulG2Return<Bls12381ArkworksG2>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| multiplied | any | none | the affine `payload.point.value` `*` the scalar's `Fr`, then `into_affine` | `Ok(MulG2SuccessReturn { product })`; the payload, holding the scalar, drops at the end of the call and the scalar is zeroized |

## `msm_g1(&self, params: MsmG1Params, payload: MsmG1Payload<Bls12381ArkworksG1, Bls12381ArkworksScalar>) -> MsmG1Return<Bls12381ArkworksG1>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| summed | any | none | split the terms into a `Vec` of affine bases and a `Vec<Fr>` of the same length, then `G1Projective::msm_unchecked` over them, then `into_affine` | `Ok(MsmG1SuccessReturn { sum })`; the `Vec<Fr>` is zeroized after the call |

## `msm_g2(&self, params: MsmG2Params, payload: MsmG2Payload<Bls12381ArkworksG2, Bls12381ArkworksScalar>) -> MsmG2Return<Bls12381ArkworksG2>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| summed | any | none | split the terms into a `Vec` of affine bases and a `Vec<Fr>` of the same length, then `G2Projective::msm_unchecked` over them, then `into_affine` | `Ok(MsmG2SuccessReturn { sum })`; the `Vec<Fr>` is zeroized after the call |

## `pairing_product_is_one(&self, params: PairingProductIsOneParams, payload: PairingProductIsOnePayload<Bls12381ArkworksG1, Bls12381ArkworksG2>) -> PairingProductIsOneReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| evaluated | any | none | `Bls12_381::multi_pairing` over the terms' first-group elements and second-group elements in term order | `Ok(PairingProductIsOneSuccessReturn { is_one })`, where `is_one` is the output's `is_zero()` — the identity of the target group in arkworks' additive notation |

## `decode_g1(&self, params: DecodeG1Params, payload: &[u8]) -> DecodeG1Return<Bls12381ArkworksG1>`

A coordinate is EIP-2537's 64 bytes: the first 16 zero, the last 48 the field element big-endian. A nonzero byte among the first 16, or 48 bytes at or above the base field modulus, is non-canonical.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| wrong length | `payload.len() != 128` | the length check | none | `Err(DecodeG1ErrorReturn::WrongLength { expected: 128, actual: payload.len() })` |
| non-canonical coordinate | either 64-byte coordinate has a nonzero byte among its first 16, or its last 48, read by `Fq::from_be_bytes_mod_order`, do not re-encode through `into_bigint().to_bytes_be()` to the same 48 bytes — that is, they are at least the base field modulus | the padding check and the re-encoding comparison, per coordinate | `Fq::from_be_bytes_mod_order`, `into_bigint().to_bytes_be()` | `Err(DecodeG1ErrorReturn::NonCanonicalCoordinate)` |
| identity | both coordinates are zero | the zero check | none | `Ok(DecodeG1SuccessReturn { point })` holding `G1Affine::identity()` |
| off the curve | `G1Affine::new_unchecked(x, y).is_on_curve()` is false | the curve check | `G1Affine::new_unchecked`, `is_on_curve` | `Err(DecodeG1ErrorReturn::NotOnCurve)` |
| outside the subgroup | `is_in_correct_subgroup_assuming_on_curve()` is false | the subgroup check | `is_in_correct_subgroup_assuming_on_curve` | `Err(DecodeG1ErrorReturn::NotInSubgroup)` |
| valid | every check passes | all of the above | as above | `Ok(DecodeG1SuccessReturn { point })` |

BLS12-381's first group has a nontrivial cofactor, so the subgroup branch is reachable here as well as in the second group.

## `decode_g2(&self, params: DecodeG2Params, payload: &[u8]) -> DecodeG2Return<Bls12381ArkworksG2>`

The same branches in the same order over 256 bytes read as `x.c0`, `x.c1`, `y.c0`, `y.c1`, each a 64-byte coordinate, assembled by `Fq2::new(c0, c1)`, with `DecodeG2ErrorReturn` and `expected: 256`. The identity is all four coordinates zero.

## `decode_scalar(&self, params: DecodeScalarParams, payload: &[u8]) -> DecodeScalarReturn<Bls12381ArkworksScalar>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| wrong length | `payload.len() != 32` | the length check | none | `Err(DecodeScalarErrorReturn::WrongLength { expected: 32, actual: payload.len() })` |
| non-canonical | the bytes, read by `Fr::from_be_bytes_mod_order`, do not re-encode to the same 32 bytes — that is, they are at least the group order | the re-encoding comparison | `Fr::from_be_bytes_mod_order`, `into_bigint().to_bytes_be()` | `Err(DecodeScalarErrorReturn::NonCanonical)` |
| valid | every check passes | all of the above | as above | `Ok(DecodeScalarSuccessReturn { scalar })` |

EIP-2537's MSM accepts any 256-bit scalar; this decoder, which produces an owned scalar, admits only the canonical ones.

## `encode_g1(&self, params: EncodeG1Params, payload: EncodeG1Payload<Bls12381ArkworksG1>) -> EncodeG1Return<Bls12381ArkworksEncodedG1>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| encoded | any | the identity check: the identity encodes to 128 zero bytes | `xy()`, then `into_bigint().to_bytes_be()` per coordinate, each written after its 16 zero bytes to its fixed 64-byte position | `Ok(EncodeG1SuccessReturn { bytes })` holding `Bls12381ArkworksEncodedG1`— 128 zero bytes for the identity; otherwise `x` then `y`, each 16 zero bytes followed by its 48 big-endian bytes |

## `encode_g2(&self, params: EncodeG2Params, payload: EncodeG2Payload<Bls12381ArkworksG2>) -> EncodeG2Return<Bls12381ArkworksEncodedG2>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| encoded | any | the identity check: the identity encodes to 256 zero bytes | `xy()`, then `into_bigint().to_bytes_be()` per coordinate, each written after its 16 zero bytes to its fixed 64-byte position | `Ok(EncodeG2SuccessReturn { bytes })` holding `Bls12381ArkworksEncodedG2`— 256 zero bytes for the identity; otherwise `x.c0`, `x.c1`, `y.c0`, `y.c1`, each 16 zero bytes followed by its 48 big-endian bytes |

## `encode_scalar(&self, params: EncodeScalarParams, payload: EncodeScalarPayload<Bls12381ArkworksScalar>) -> EncodeScalarReturn<Bls12381ArkworksEncodedScalar>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| encoded | any | none | `into_bigint().to_bytes_be()` | `Ok(EncodeScalarSuccessReturn { bytes })`, the scalar's 32 big-endian bytes in `Bls12381ArkworksEncodedScalar` moved into a `Secret` |

## `Bls12381ArkworksScalar::UNIFORM_BYTES_LENGTH`

`64` — twice the byte width of the group order, so the reduction's bias from uniform is below two to the minus two hundred fifty.

## `Bls12381ArkworksScalar::sample_from_uniform_bytes(params: SampleUniformScalarParams, payload: SampleUniformScalarPayload) -> SampleUniformScalarReturn<Bls12381ArkworksScalar>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| wrong length | `payload.uniform.expose().len() != 64` | the length check | `Secret::expose` | `Err(SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual })` |
| sampled | the length is 64 | none | `Fr::from_be_bytes_mod_order` over the exposed bytes | `Ok(SampleUniformScalarSuccessReturn { scalar })`, the scalar moved into a `Secret`; the payload's `Secret` zeroizes the input when it drops |

## `add_scalar(&self, params: AddScalarParams, payload: AddScalarPayload<Bls12381ArkworksScalar>) -> AddScalarReturn<Bls12381ArkworksScalar>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| summed | any | none | `Fr`'s `+` over `payload.left.value` and `payload.right.value` | `Ok(AddScalarSuccessReturn { sum })`, the sum modulo the group order in the owned scalar type; the payload, holding both scalars, drops at the end of the call and both are zeroized |

## `mul_scalar(&self, params: MulScalarParams, payload: MulScalarPayload<Bls12381ArkworksScalar>) -> MulScalarReturn<Bls12381ArkworksScalar>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| multiplied | any | none | `Fr`'s `*` over `payload.left.value` and `payload.right.value` | `Ok(MulScalarSuccessReturn { product })`, the product modulo the group order in the owned scalar type; the payload's scalars are zeroized as it drops |

## `neg_scalar(&self, params: NegScalarParams, payload: NegScalarPayload<Bls12381ArkworksScalar>) -> NegScalarReturn<Bls12381ArkworksScalar>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| negated | any | none | `Fr`'s unary `-` over `payload.scalar.value` | `Ok(NegScalarSuccessReturn { negation })`, the group order minus the scalar, and zero for zero |

## `neg_g1(&self, params: NegG1Params, payload: NegG1Payload<Bls12381ArkworksG1>) -> NegG1Return<Bls12381ArkworksG1>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| negated | any | none | the affine point's unary `-` over `payload.point.value` | `Ok(NegG1SuccessReturn { negation })`, `(x, p - y)` for a point `(x, y)` and the identity for the identity |

## `neg_g2(&self, params: NegG2Params, payload: NegG2Payload<Bls12381ArkworksG2>) -> NegG2Return<Bls12381ArkworksG2>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| negated | any | none | the affine point's unary `-` over `payload.point.value` | `Ok(NegG2SuccessReturn { negation })`, `(x, p - y)` for a point `(x, y)` and the identity for the identity |

## `is_identity_g1(&self, params: IsIdentityG1Params, payload: IsIdentityG1Payload<Bls12381ArkworksG1>) -> IsIdentityG1Return`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| tested | any | none | `AffineRepr::is_zero()` on `payload.point.value` | `Ok(IsIdentityG1SuccessReturn { is_identity })`, `true` exactly for the identity |

## `is_identity_g2(&self, params: IsIdentityG2Params, payload: IsIdentityG2Payload<Bls12381ArkworksG2>) -> IsIdentityG2Return`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| tested | any | none | `AffineRepr::is_zero()` on `payload.point.value` | `Ok(IsIdentityG2SuccessReturn { is_identity })`, `true` exactly for the identity |

## `pairing_product(&self, params: PairingProductParams, payload: PairingProductPayload<Bls12381ArkworksG1, Bls12381ArkworksG2>) -> PairingProductReturn<Bls12381ArkworksGt>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| evaluated | any | none | split the terms into a `Vec<G1Affine>` and a `Vec<G2Affine>` in term order, then `Bls12_381::multi_pairing(&g1s, &g2s)`, then the `PairingOutput` `*` `self.reduced_pairing_correction`, the exponentiation that brings the library's cubed value to the identifier's exact value, then `zeroize` on both vectors | `Ok(PairingProductSuccessReturn { product })` holding the corrected `PairingOutput` in the owned target-group type; an empty term list yields the target group's identity, which the exponentiation preserves |

## `encode_gt(&self, params: EncodeGtParams, payload: EncodeGtPayload<Bls12381ArkworksGt>) -> EncodeGtReturn<Bls12381ArkworksEncodedGt>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| encoded | any | none | `into_bigint().to_bytes_be()` on each of the twelve `Fq` coefficients of `payload.value.value.0` in the tower order `c0.c0.c0`, `c0.c0.c1`, `c0.c1.c0`, `c0.c1.c1`, `c0.c2.c0`, `c0.c2.c1`, `c1.c0.c0`, `c1.c0.c1`, `c1.c1.c0`, `c1.c1.c1`, `c1.c2.c0`, `c1.c2.c1`, each written to its fixed 48-byte position in a 576-byte buffer | `Ok(EncodeGtSuccessReturn { bytes })`, a `Secret<Bls12381ArkworksEncodedGt>` built from the complete buffer with no variable-width intermediate; the target group's identity encodes as 47 zero bytes, `01`, and 528 zero bytes |

## `scalar_field_order(&self, params: ScalarFieldOrderParams, payload: ScalarFieldOrderPayload) -> ScalarFieldOrderReturn`

|| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
|| read | any | none | `Fr::MODULUS.to_bytes_be()` | `Ok(ScalarFieldOrderSuccessReturn { bytes })`, the group order's 32 big-endian bytes |

The error arm has no branch.

## `g1_outside_subgroup_encoding(&self, params: G1OutsideSubgroupEncodingParams, payload: G1OutsideSubgroupEncodingPayload) -> G1OutsideSubgroupEncodingReturn<Bls12381ArkworksEncodedG1>`

|| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
|| exhausted | `(1u64..).find_map(…)` over the search below returns `None` | the search's result | the search | `Err(G1OutsideSubgroupEncodingErrorReturn::SearchExhausted)` |
|| found | the search returns a point and its `y` | per `x` in ascending order, `G1Affine::get_point_from_x_unchecked(Fq::from(x), false)`, kept when `is_in_correct_subgroup_assuming_on_curve()` is false, its `y` read through `xy()` inside the search so a value yielding no coordinates continues the search; then, with `negated = -y`, the point is kept when `y.into_bigint()` is not greater than `negated.into_bigint()` and replaced by its affine negation otherwise | `self.encode_g1(EncodeG1Params, EncodeG1Payload { point: Bls12381ArkworksG1 { value } })`, unpacked irrefutably | `Ok(G1OutsideSubgroupEncodingSuccessReturn { bytes: Some(bytes) })`, the `Bls12381ArkworksEncodedG1` `encode_g1` returns |

The first group's cofactor exceeds one, so the search ends among the least values of `x`; no input takes the exhausted branch and it has no unit test.

## `g2_outside_subgroup_encoding(&self, params: G2OutsideSubgroupEncodingParams, payload: G2OutsideSubgroupEncodingPayload) -> G2OutsideSubgroupEncodingReturn<Bls12381ArkworksEncodedG2>`

|| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
|| exhausted | `(1u64..).find_map(…)` over the search below returns `None` | the search's result | the search | `Err(G2OutsideSubgroupEncodingErrorReturn::SearchExhausted)` |
|| found | the search returns a point and its `y` | per `c0` in ascending order, `G2Affine::get_point_from_x_unchecked(Fq2::new(Fq::from(c0), Fq::from(0u64)), false)`, kept when `is_in_correct_subgroup_assuming_on_curve()` is false, its `y` read through `xy()` inside the search so a value yielding no coordinates continues the search; then, with `negated = -y`, the point is kept when `(y.c1.into_bigint(), y.c0.into_bigint())` is not greater than `(negated.c1.into_bigint(), negated.c0.into_bigint())` and replaced by its affine negation otherwise | `self.encode_g2(EncodeG2Params, EncodeG2Payload { point: Bls12381ArkworksG2 { value } })`, unpacked irrefutably | `Ok(G2OutsideSubgroupEncodingSuccessReturn { bytes })`, the `Bls12381ArkworksEncodedG2` `encode_g2` returns |

## Ordering and edges

- Every decoder checks in the stated order — length, then canonicality, then identity, then curve, then subgroup — and slices the payload only after the length check.
- An empty `msm` term list yields the identity; an empty `pairing_product_is_one` term list yields `is_one: true`, as EIP-2537 does for empty input.
- Zeroization: `Bls12381ArkworksScalar`, `Bls12381ArkworksG1`, `Bls12381ArkworksG2`, and `Bls12381ArkworksGt` each zeroize their `value` through their `Zeroize` implementation and on drop, so every clone a consumer places in a payload is zeroized when the payload drops.
- Each outside-the-subgroup search is ascending from one and stops at the first on-curve point outside the subgroup; the choice between a point and its negation follows the search and precedes the encoding, and the same call always returns the same bytes.
- `params` carries no control and is not read in any method, and no reference method reads its payload.

