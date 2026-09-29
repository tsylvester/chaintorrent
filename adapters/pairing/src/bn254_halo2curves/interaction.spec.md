# `bn254_halo2curves` — interaction spec

Branch contract for the `bn254_halo2curves` module of the `pairing` crate: the pairing adapter over halo2curves' BN256 (BN254), encoding group elements as EIP-196 and EIP-197 precompile input. Each branch states condition, decision, dependency call, and the exact return outcome.

## `Bn254Halo2curvesPairing::try_new(params: Bn254Halo2curvesPairingConstructorParams) -> Bn254Halo2curvesPairingTryNewReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| construct | any params | none | none | `Ok(Bn254Halo2curvesPairing)` |

The error arm has no branch: the adapter takes no configuration, so `Infallible` is uninhabited.

## `Bn254Halo2curvesPairing::DECLARATION`

An inherent constant, readable from the type before any instance exists:

`PairingDeclaration { curve: PairingCurve::Bn254, verifier_group_arithmetic: VerifierGroupArithmetic::FirstGroupOnly, precompile_encoding: PrecompileEncoding::Eip196Eip197, adapter_version: 1, interface_version: PAIRING_INTERFACE_VERSION }`

## `g1_generator(&self, params: G1GeneratorParams, payload: G1GeneratorPayload) -> G1GeneratorReturn<Bn254Halo2curvesG1>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| generated | any | none | `G1Affine::generator()` | `Ok(G1GeneratorSuccessReturn { point })`, the generator in the owned group type |

## `g2_generator(&self, params: G2GeneratorParams, payload: G2GeneratorPayload) -> G2GeneratorReturn<Bn254Halo2curvesG2>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| generated | any | none | `G2Affine::generator()` | `Ok(G2GeneratorSuccessReturn { point })`, the generator in the owned group type |

## `add_g1(&self, params: AddG1Params, payload: AddG1Payload<Bn254Halo2curvesG1>) -> AddG1Return<Bn254Halo2curvesG1>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| summed | any | none | `to_curve()` on each payload point, the projective `+`, then `to_affine()` | `Ok(AddG1SuccessReturn { sum })` |

## `add_g2(&self, params: AddG2Params, payload: AddG2Payload<Bn254Halo2curvesG2>) -> AddG2Return<Bn254Halo2curvesG2>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| summed | any | none | `to_curve()` on each payload point, the projective `+`, then `to_affine()` | `Ok(AddG2SuccessReturn { sum })` |

## `mul_g1(&self, params: MulG1Params, payload: MulG1Payload<Bn254Halo2curvesG1, Bn254Halo2curvesScalar>) -> MulG1Return<Bn254Halo2curvesG1>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| multiplied | any | none | `to_curve()` on the point, the projective `*` the scalar's `Fr`, then `to_affine()` | `Ok(MulG1SuccessReturn { product })`; the payload, holding the scalar, drops at the end of the call and the scalar is cleared |

## `mul_g2(&self, params: MulG2Params, payload: MulG2Payload<Bn254Halo2curvesG2, Bn254Halo2curvesScalar>) -> MulG2Return<Bn254Halo2curvesG2>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| multiplied | any | none | `to_curve()` on the point, the projective `*` the scalar's `Fr`, then `to_affine()` | `Ok(MulG2SuccessReturn { product })`; the payload, holding the scalar, drops at the end of the call and the scalar is cleared |

## `msm_g1(&self, params: MsmG1Params, payload: MsmG1Payload<Bn254Halo2curvesG1, Bn254Halo2curvesScalar>) -> MsmG1Return<Bn254Halo2curvesG1>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| summed | any | none | split the terms into a `Vec` of affine bases and a `Vec<Fr>` of the same length, then `msm_best(&scalars, &bases)`, then `to_affine()` | `Ok(MsmG1SuccessReturn { sum })`; every element of the `Vec<Fr>` is then set to `Fr::ZERO` and the vector passed to `black_box` |

## `msm_g2(&self, params: MsmG2Params, payload: MsmG2Payload<Bn254Halo2curvesG2, Bn254Halo2curvesScalar>) -> MsmG2Return<Bn254Halo2curvesG2>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| summed | any | none | split the terms into a `Vec` of affine bases and a `Vec<Fr>` of the same length, then `msm_best(&scalars, &bases)`, then `to_affine()` | `Ok(MsmG2SuccessReturn { sum })`; every element of the `Vec<Fr>` is then set to `Fr::ZERO` and the vector passed to `black_box` |

## `pairing_product_is_one(&self, params: PairingProductIsOneParams, payload: PairingProductIsOnePayload<Bn254Halo2curvesG1, Bn254Halo2curvesG2>) -> PairingProductIsOneReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| evaluated | any | none | `Bn256::multi_miller_loop` over the terms as a `Vec<(&G1Affine, &G2Affine)>` in term order, then `final_exponentiation()` | `Ok(PairingProductIsOneSuccessReturn { is_one })`, where `is_one` is `bool::from` of the result's `is_identity()` |

## `decode_g1(&self, params: DecodeG1Params, payload: &[u8]) -> DecodeG1Return<Bn254Halo2curvesG1>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| wrong length | `<[u8; 64]>::try_from(payload)` fails | the length check | none | `Err(DecodeG1ErrorReturn::WrongLength { expected: 64, actual: payload.len() })` |
| non-canonical coordinate | either 32-byte half, reversed to little-endian and read by `Fq::from_repr`, is none — that is, it is at least the base field modulus | the `CtOption` conversion, per half | `Fq::from_repr` | `Err(DecodeG1ErrorReturn::NonCanonicalCoordinate)` |
| identity | both coordinates are zero | the zero check | none | `Ok(DecodeG1SuccessReturn { point })` holding `G1Affine::identity()` |
| off the curve | `G1Affine::from_xy(x, y)` is none | the curve check | `G1Affine::from_xy` | `Err(DecodeG1ErrorReturn::NotOnCurve)` |
| outside the subgroup | `bool::from(point.to_curve().is_torsion_free())` is false | the subgroup check | `to_curve`, `is_torsion_free` | `Err(DecodeG1ErrorReturn::NotInSubgroup)` |
| valid | every check passes | all of the above | as above | `Ok(DecodeG1SuccessReturn { point })` |

BN254's first group has cofactor one, so no on-curve point takes the subgroup branch and it has no unit test, but the check runs on every decode.

## `decode_g2(&self, params: DecodeG2Params, payload: &[u8]) -> DecodeG2Return<Bn254Halo2curvesG2>`

The same branches in the same order over 128 bytes read as `x.c1`, `x.c0`, `y.c1`, `y.c0`, each 32 bytes big-endian, assembled by `Fq2::new(c0, c1)`, with `DecodeG2ErrorReturn` and `expected: 128`. The identity is all four coordinates zero. The subgroup branch is reachable, since BN254's second group has a nontrivial cofactor.

## `decode_scalar(&self, params: DecodeScalarParams, payload: &[u8]) -> DecodeScalarReturn<Bn254Halo2curvesScalar>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| wrong length | `<[u8; 32]>::try_from(payload)` fails | the length check | none | `Err(DecodeScalarErrorReturn::WrongLength { expected: 32, actual: payload.len() })` |
| non-canonical | the bytes, reversed to little-endian and read by `Fr::from_repr`, are none — that is, they are at least the group order | the `CtOption` conversion | `Fr::from_repr` | `Err(DecodeScalarErrorReturn::NonCanonical)` |
| valid | every check passes | all of the above | as above | `Ok(DecodeScalarSuccessReturn { scalar })` |

## `encode_g1(&self, params: EncodeG1Params, payload: EncodeG1Payload<Bn254Halo2curvesG1>) -> EncodeG1Return`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| encoded | any | the identity check: a point whose `coordinates()` is none encodes to 64 zero bytes | `coordinates()`, then `to_repr()` reversed to big-endian per coordinate | `Ok(EncodeG1SuccessReturn { bytes })` — 64 zero bytes for the identity; otherwise `x()` then `y()`, each 32 bytes big-endian |

## `encode_g2(&self, params: EncodeG2Params, payload: EncodeG2Payload<Bn254Halo2curvesG2>) -> EncodeG2Return`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| encoded | any | the identity check: a point whose `coordinates()` is none encodes to 128 zero bytes | `coordinates()`, then `to_repr()` reversed to big-endian per coordinate | `Ok(EncodeG2SuccessReturn { bytes })` — 128 zero bytes for the identity; otherwise `x().c1()`, `x().c0()`, `y().c1()`, `y().c0()`, each 32 bytes big-endian |

## `encode_scalar(&self, params: EncodeScalarParams, payload: EncodeScalarPayload<Bn254Halo2curvesScalar>) -> EncodeScalarReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| encoded | any | none | `to_repr()` reversed to 32 big-endian bytes | `Ok(EncodeScalarSuccessReturn { bytes })`, the scalar's 32 big-endian bytes moved into a `Secret` |

## `Bn254Halo2curvesScalar::UNIFORM_BYTES_LENGTH`

`64` — twice the byte width of the group order.

## `Bn254Halo2curvesScalar::sample_from_uniform_bytes(params: SampleUniformScalarParams, payload: SampleUniformScalarPayload) -> SampleUniformScalarReturn<Bn254Halo2curvesScalar>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| wrong length | `<&[u8; 64]>::try_from(payload.uniform.expose().as_slice())` fails | the length check | `Secret::expose` | `Err(SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual })` |
| sampled | the length is 64 | none | the 64 bytes are copied into a local `[u8; 64]` and reversed, so the big-endian integer the arkworks concrete reads is the little-endian integer halo2curves reads; `Fr::from_uniform_bytes` over the copy, which is then zeroized | `Ok(SampleUniformScalarSuccessReturn { scalar })`, the scalar moved into a `Secret`; the payload's `Secret` zeroizes the input when it drops |

## Ordering and edges

- Every decoder checks in the stated order — length, then canonicality, then identity, then curve, then subgroup — and copies each 32-byte coordinate out of the fixed-size array before reversing it to little-endian.
- An empty `msm` term list yields the identity; an empty `pairing_product_is_one` term list yields `is_one: true`, as EIP-197 does for empty input.
- Clearing: halo2curves' `Fr` implements no `Zeroize`, so `Bn254Halo2curvesScalar`'s `Zeroize` implementation and its `Drop` set `value` to `Fr::ZERO` and pass `&self.value` to `black_box`, which keeps the clearing from being removed as a dead store; every clone a consumer places in a payload is cleared when the payload drops. The same zero-then-`black_box` treatment clears the `Vec<Fr>` an `msm` builds.
- `params` carries no control and is not read in any method.
