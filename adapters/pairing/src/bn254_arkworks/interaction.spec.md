# `bn254_arkworks` — interaction spec

Branch contract for the `bn254_arkworks` module of the `pairing` crate: the pairing adapter over arkworks' BN254, encoding group elements as EIP-196 and EIP-197 precompile input. Each branch states condition, decision, dependency call, and the exact return outcome.

## `Bn254ArkworksPairing::try_new(params: Bn254ArkworksPairingConstructorParams) -> Bn254ArkworksPairingTryNewReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| construct | any params | none | `Fr::from(BN254_SEED)` as `seed`; `(seed * seed * Fr::from(6u64) + seed * Fr::from(3u64) + Fr::ONE) * seed * Fr::from(2u64)` as `multiple`; `let mut exponent = Fr::MODULUS;` then `let _ = exponent.sub_with_borrow(&BigInt::from(2u64));`, the group order minus two; `multiple.pow(exponent)` as `reduced_pairing_correction` | `Ok(Bn254ArkworksPairing { reduced_pairing_correction })` |

The error arm has no branch: the adapter takes no configuration, so `Infallible` is uninhabited.

## `Bn254ArkworksPairing::DECLARATION`

An inherent constant, readable from the type before any instance exists:

`PairingDeclaration { curve: PairingCurve::Bn254, verifier_group_arithmetic: VerifierGroupArithmetic::FirstGroupOnly, precompile_encoding: PrecompileEncoding::Eip196Eip197, target_group_encoding: TargetGroupEncodingIdentifier::Bn254V1, adapter_version: 1, interface_version: PAIRING_INTERFACE_VERSION }`

## `g1_generator(&self, params: G1GeneratorParams, payload: G1GeneratorPayload) -> G1GeneratorReturn<Bn254ArkworksG1>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| generated | any | none | `G1Affine::generator()` | `Ok(G1GeneratorSuccessReturn { point })`, the generator in the owned group type |

## `g2_generator(&self, params: G2GeneratorParams, payload: G2GeneratorPayload) -> G2GeneratorReturn<Bn254ArkworksG2>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| generated | any | none | `G2Affine::generator()` | `Ok(G2GeneratorSuccessReturn { point })`, the generator in the owned group type |

## `add_g1(&self, params: AddG1Params, payload: AddG1Payload<Bn254ArkworksG1>) -> AddG1Return<Bn254ArkworksG1>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| summed | any | none | the affine `+` of `payload.left.value` and `payload.right.value`, then `into_affine` | `Ok(AddG1SuccessReturn { sum })` |

## `add_g2(&self, params: AddG2Params, payload: AddG2Payload<Bn254ArkworksG2>) -> AddG2Return<Bn254ArkworksG2>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| summed | any | none | the affine `+` of `payload.left.value` and `payload.right.value`, then `into_affine` | `Ok(AddG2SuccessReturn { sum })` |

## `mul_g1(&self, params: MulG1Params, payload: MulG1Payload<Bn254ArkworksG1, Bn254ArkworksScalar>) -> MulG1Return<Bn254ArkworksG1>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| multiplied | any | none | the affine `payload.point.value` `*` the scalar's `Fr`, then `into_affine` | `Ok(MulG1SuccessReturn { product })`; the payload, holding the scalar, drops at the end of the call and the scalar is zeroized |

## `mul_g2(&self, params: MulG2Params, payload: MulG2Payload<Bn254ArkworksG2, Bn254ArkworksScalar>) -> MulG2Return<Bn254ArkworksG2>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| multiplied | any | none | the affine `payload.point.value` `*` the scalar's `Fr`, then `into_affine` | `Ok(MulG2SuccessReturn { product })`; the payload, holding the scalar, drops at the end of the call and the scalar is zeroized |

## `msm_g1(&self, params: MsmG1Params, payload: MsmG1Payload<Bn254ArkworksG1, Bn254ArkworksScalar>) -> MsmG1Return<Bn254ArkworksG1>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| summed | any | none | split the terms into a `Vec` of affine bases and a `Vec<Fr>` of the same length, then `G1Projective::msm_unchecked` over them, then `into_affine` | `Ok(MsmG1SuccessReturn { sum })`; the `Vec<Fr>` is zeroized after the call |

## `msm_g2(&self, params: MsmG2Params, payload: MsmG2Payload<Bn254ArkworksG2, Bn254ArkworksScalar>) -> MsmG2Return<Bn254ArkworksG2>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| summed | any | none | split the terms into a `Vec` of affine bases and a `Vec<Fr>` of the same length, then `G2Projective::msm_unchecked` over them, then `into_affine` | `Ok(MsmG2SuccessReturn { sum })`; the `Vec<Fr>` is zeroized after the call |

## `pairing_product_is_one(&self, params: PairingProductIsOneParams, payload: PairingProductIsOnePayload<Bn254ArkworksG1, Bn254ArkworksG2>) -> PairingProductIsOneReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| evaluated | any | none | `Bn254::multi_pairing` over the terms' first-group elements and second-group elements in term order | `Ok(PairingProductIsOneSuccessReturn { is_one })`, where `is_one` is the output's `is_zero()` — the identity of the target group in arkworks' additive notation |

## `decode_g1(&self, params: DecodeG1Params, payload: &[u8]) -> DecodeG1Return<Bn254ArkworksG1>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| wrong length | `payload.len() != 64` | the length check | none | `Err(DecodeG1ErrorReturn::WrongLength { expected: 64, actual: payload.len() })` |
| non-canonical coordinate | either 32-byte half, read by `Fq::from_be_bytes_mod_order`, does not re-encode through `into_bigint().to_bytes_be()` to the same 32 bytes — that is, it is at least the base field modulus | the re-encoding comparison, per half | `Fq::from_be_bytes_mod_order`, `into_bigint().to_bytes_be()` | `Err(DecodeG1ErrorReturn::NonCanonicalCoordinate)` |
| identity | both coordinates are zero | the zero check | none | `Ok(DecodeG1SuccessReturn { point })` holding `G1Affine::identity()` |
| off the curve | `G1Affine::new_unchecked(x, y).is_on_curve()` is false | the curve check | `G1Affine::new_unchecked`, `is_on_curve` | `Err(DecodeG1ErrorReturn::NotOnCurve)` |
| outside the subgroup | `is_in_correct_subgroup_assuming_on_curve()` is false | the subgroup check | `is_in_correct_subgroup_assuming_on_curve` | `Err(DecodeG1ErrorReturn::NotInSubgroup)` |
| valid | every check passes | all of the above | as above | `Ok(DecodeG1SuccessReturn { point })` |

BN254's first group has cofactor one, so no on-curve point takes the subgroup branch and it has no unit test, but the check runs on every decode.

## `decode_g2(&self, params: DecodeG2Params, payload: &[u8]) -> DecodeG2Return<Bn254ArkworksG2>`

The same branches in the same order over 128 bytes read as `x.c1`, `x.c0`, `y.c1`, `y.c0`, each 32 bytes, assembled by `Fq2::new(c0, c1)`, with `DecodeG2ErrorReturn` and `expected: 128`. The identity is all four coordinates zero. The subgroup branch is reachable, since BN254's second group has a nontrivial cofactor.

## `decode_scalar(&self, params: DecodeScalarParams, payload: &[u8]) -> DecodeScalarReturn<Bn254ArkworksScalar>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| wrong length | `payload.len() != 32` | the length check | none | `Err(DecodeScalarErrorReturn::WrongLength { expected: 32, actual: payload.len() })` |
| non-canonical | the bytes, read by `Fr::from_be_bytes_mod_order`, do not re-encode to the same 32 bytes — that is, they are at least the group order | the re-encoding comparison | `Fr::from_be_bytes_mod_order`, `into_bigint().to_bytes_be()` | `Err(DecodeScalarErrorReturn::NonCanonical)` |
| valid | every check passes | all of the above | as above | `Ok(DecodeScalarSuccessReturn { scalar })` |

## `encode_g1(&self, params: EncodeG1Params, payload: EncodeG1Payload<Bn254ArkworksG1>) -> EncodeG1Return`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| encoded | any | the identity check: the identity encodes to 64 zero bytes | `xy()`, then `into_bigint().to_bytes_be()` per coordinate | `Ok(EncodeG1SuccessReturn { bytes })` — 64 zero bytes for the identity; otherwise `x` then `y`, each 32 bytes big-endian |

## `encode_g2(&self, params: EncodeG2Params, payload: EncodeG2Payload<Bn254ArkworksG2>) -> EncodeG2Return`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| encoded | any | the identity check: the identity encodes to 128 zero bytes | `xy()`, then `into_bigint().to_bytes_be()` per coordinate | `Ok(EncodeG2SuccessReturn { bytes })` — 128 zero bytes for the identity; otherwise `x.c1`, `x.c0`, `y.c1`, `y.c0`, each 32 bytes big-endian |

## `encode_scalar(&self, params: EncodeScalarParams, payload: EncodeScalarPayload<Bn254ArkworksScalar>) -> EncodeScalarReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| encoded | any | none | `into_bigint().to_bytes_be()` | `Ok(EncodeScalarSuccessReturn { bytes })`, the scalar's 32 big-endian bytes moved into a `Secret` |

## `Bn254ArkworksScalar::UNIFORM_BYTES_LENGTH`

`64` — twice the byte width of the group order, so the reduction's bias from uniform is below two to the minus two hundred fifty.

## `Bn254ArkworksScalar::sample_from_uniform_bytes(params: SampleUniformScalarParams, payload: SampleUniformScalarPayload) -> SampleUniformScalarReturn<Bn254ArkworksScalar>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| wrong length | `payload.uniform.expose().len() != 64` | the length check | `Secret::expose` | `Err(SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual })` |
| sampled | the length is 64 | none | `Fr::from_be_bytes_mod_order` over the exposed bytes | `Ok(SampleUniformScalarSuccessReturn { scalar })`, the scalar moved into a `Secret`; the payload's `Secret` zeroizes the input when it drops |

## `add_scalar(&self, params: AddScalarParams, payload: AddScalarPayload<Bn254ArkworksScalar>) -> AddScalarReturn<Bn254ArkworksScalar>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| summed | any | none | `Fr`'s `+` over `payload.left.value` and `payload.right.value` | `Ok(AddScalarSuccessReturn { sum })`, the sum modulo the group order in the owned scalar type; the payload, holding both scalars, drops at the end of the call and both are zeroized |

## `mul_scalar(&self, params: MulScalarParams, payload: MulScalarPayload<Bn254ArkworksScalar>) -> MulScalarReturn<Bn254ArkworksScalar>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| multiplied | any | none | `Fr`'s `*` over `payload.left.value` and `payload.right.value` | `Ok(MulScalarSuccessReturn { product })`, the product modulo the group order in the owned scalar type; the payload's scalars are zeroized as it drops |

## `neg_scalar(&self, params: NegScalarParams, payload: NegScalarPayload<Bn254ArkworksScalar>) -> NegScalarReturn<Bn254ArkworksScalar>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| negated | any | none | `Fr`'s unary `-` over `payload.scalar.value` | `Ok(NegScalarSuccessReturn { negation })`, the group order minus the scalar, and zero for zero |

## `neg_g1(&self, params: NegG1Params, payload: NegG1Payload<Bn254ArkworksG1>) -> NegG1Return<Bn254ArkworksG1>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| negated | any | none | the affine point's unary `-` over `payload.point.value` | `Ok(NegG1SuccessReturn { negation })`, `(x, p - y)` for a point `(x, y)` and the identity for the identity |

## `neg_g2(&self, params: NegG2Params, payload: NegG2Payload<Bn254ArkworksG2>) -> NegG2Return<Bn254ArkworksG2>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| negated | any | none | the affine point's unary `-` over `payload.point.value` | `Ok(NegG2SuccessReturn { negation })`, `(x, p - y)` for a point `(x, y)` and the identity for the identity |

## `is_identity_g1(&self, params: IsIdentityG1Params, payload: IsIdentityG1Payload<Bn254ArkworksG1>) -> IsIdentityG1Return`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| tested | any | none | `AffineRepr::is_zero()` on `payload.point.value` | `Ok(IsIdentityG1SuccessReturn { is_identity })`, `true` exactly for the identity |

## `is_identity_g2(&self, params: IsIdentityG2Params, payload: IsIdentityG2Payload<Bn254ArkworksG2>) -> IsIdentityG2Return`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| tested | any | none | `AffineRepr::is_zero()` on `payload.point.value` | `Ok(IsIdentityG2SuccessReturn { is_identity })`, `true` exactly for the identity |

## `pairing_product(&self, params: PairingProductParams, payload: PairingProductPayload<Bn254ArkworksG1, Bn254ArkworksG2>) -> PairingProductReturn<Bn254ArkworksGt>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| evaluated | any | none | split the terms into a `Vec<G1Affine>` and a `Vec<G2Affine>` in term order, then `Bn254::multi_pairing(&g1s, &g2s)`, then the `PairingOutput` `*` `self.reduced_pairing_correction`, the exponentiation that brings the library's reduced pairing to the identifier's exact value, then `zeroize` on both vectors | `Ok(PairingProductSuccessReturn { product })` holding the corrected `PairingOutput` in the owned target-group type; an empty term list yields the target group's identity, which the exponentiation preserves |

## `encode_gt(&self, params: EncodeGtParams, payload: EncodeGtPayload<Bn254ArkworksGt>) -> EncodeGtReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| encoded | any | none | `into_bigint().to_bytes_be()` on each of the twelve `Fq` coefficients of `payload.value.value.0` in the tower order `c0.c0.c0`, `c0.c0.c1`, `c0.c1.c0`, `c0.c1.c1`, `c0.c2.c0`, `c0.c2.c1`, `c1.c0.c0`, `c1.c0.c1`, `c1.c1.c0`, `c1.c1.c1`, `c1.c2.c0`, `c1.c2.c1`, appended in that order into one buffer of 384 bytes | `Ok(EncodeGtSuccessReturn { bytes })`, the buffer moved into a `Secret` by `let Ok(bytes) = Secret::try_new(SecretConstructorParams { value: buffer });`; the target group's identity encodes as 31 zero bytes, `01`, and 352 zero bytes |

## Ordering and edges

- Every decoder checks in the stated order — length, then canonicality, then identity, then curve, then subgroup — and slices the payload only after the length check.
- An empty `msm` term list yields the identity; an empty `pairing_product_is_one` term list yields `is_one: true`, as EIP-197 does for empty input.
- Zeroization: `Bn254ArkworksScalar`, `Bn254ArkworksG1`, `Bn254ArkworksG2`, and `Bn254ArkworksGt` each zeroize their `value` through their `Zeroize` implementation and on drop, so every clone a consumer places in a payload is zeroized when the payload drops.
- `params` carries no control and is not read in any method.
