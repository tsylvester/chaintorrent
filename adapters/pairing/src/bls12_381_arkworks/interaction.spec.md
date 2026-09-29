# `bls12_381_arkworks` — interaction spec

Branch contract for the `bls12_381_arkworks` module of the `pairing` crate: the pairing adapter over arkworks' BLS12-381, encoding group elements as EIP-2537 precompile input. Each branch states condition, decision, dependency call, and the exact return outcome.

## `Bls12381ArkworksPairing::try_new(params: Bls12381ArkworksPairingConstructorParams) -> Bls12381ArkworksPairingTryNewReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| construct | any params | none | none | `Ok(Bls12381ArkworksPairing)` |

The error arm has no branch: the adapter takes no configuration, so `Infallible` is uninhabited.

## `Bls12381ArkworksPairing::DECLARATION`

An inherent constant, readable from the type before any instance exists:

`PairingDeclaration { curve: PairingCurve::Bls12381, verifier_group_arithmetic: VerifierGroupArithmetic::BothGroups, precompile_encoding: PrecompileEncoding::Eip2537, adapter_version: 1, interface_version: PAIRING_INTERFACE_VERSION }`

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

## `encode_g1(&self, params: EncodeG1Params, payload: EncodeG1Payload<Bls12381ArkworksG1>) -> EncodeG1Return`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| encoded | any | the identity check: the identity encodes to 128 zero bytes | `xy()`, then `into_bigint().to_bytes_be()` per coordinate | `Ok(EncodeG1SuccessReturn { bytes })` — 128 zero bytes for the identity; otherwise `x` then `y`, each 16 zero bytes followed by its 48 big-endian bytes |

## `encode_g2(&self, params: EncodeG2Params, payload: EncodeG2Payload<Bls12381ArkworksG2>) -> EncodeG2Return`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| encoded | any | the identity check: the identity encodes to 256 zero bytes | `xy()`, then `into_bigint().to_bytes_be()` per coordinate | `Ok(EncodeG2SuccessReturn { bytes })` — 256 zero bytes for the identity; otherwise `x.c0`, `x.c1`, `y.c0`, `y.c1`, each 16 zero bytes followed by its 48 big-endian bytes |

## `encode_scalar(&self, params: EncodeScalarParams, payload: EncodeScalarPayload<Bls12381ArkworksScalar>) -> EncodeScalarReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| encoded | any | none | `into_bigint().to_bytes_be()` | `Ok(EncodeScalarSuccessReturn { bytes })`, the scalar's 32 big-endian bytes moved into a `Secret` |

## `Bls12381ArkworksScalar::UNIFORM_BYTES_LENGTH`

`64` — twice the byte width of the group order, so the reduction's bias from uniform is below two to the minus two hundred fifty.

## `Bls12381ArkworksScalar::sample_from_uniform_bytes(params: SampleUniformScalarParams, payload: SampleUniformScalarPayload) -> SampleUniformScalarReturn<Bls12381ArkworksScalar>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| wrong length | `payload.uniform.expose().len() != 64` | the length check | `Secret::expose` | `Err(SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual })` |
| sampled | the length is 64 | none | `Fr::from_be_bytes_mod_order` over the exposed bytes | `Ok(SampleUniformScalarSuccessReturn { scalar })`, the scalar moved into a `Secret`; the payload's `Secret` zeroizes the input when it drops |

## Ordering and edges

- Every decoder checks in the stated order — length, then canonicality, then identity, then curve, then subgroup — and slices the payload only after the length check.
- An empty `msm` term list yields the identity; an empty `pairing_product_is_one` term list yields `is_one: true`, as EIP-2537 does for empty input.
- Zeroization: `Bls12381ArkworksScalar` zeroizes its `Fr` through its `Zeroize` implementation and on drop, so every clone a consumer places in a payload is zeroized when the payload drops.
- `params` carries no control and is not read in any method.
