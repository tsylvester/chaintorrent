# `bls12_381_halo2curves` — interaction spec

Branch contract for the `bls12_381_halo2curves` module of the `pairing` crate: the pairing adapter over halo2curves' BLS12-381, encoding group elements as EIP-2537 precompile input. Each branch states condition, decision, dependency call, and the exact return outcome.

## `Bls12381Halo2curvesPairing::try_new(params: Bls12381Halo2curvesPairingConstructorParams) -> Bls12381Halo2curvesPairingTryNewReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| construct | any params | none | `(-Fr::from(2u64)).to_repr()`, the group order minus two as 32 little-endian bytes, read into four little-endian limbs by `core::array::from_fn`, each limb folding its eight bytes from the most significant by `(limb << 8) \| u64::from(byte)`; then `Fr::from(3u64).pow_vartime(limbs)` as `reduced_pairing_correction`, the inverse of three by Fermat | `Ok(Bls12381Halo2curvesPairing { reduced_pairing_correction })` |

The error arm has no branch: the adapter takes no configuration, so `Infallible` is uninhabited.

## `Bls12381Halo2curvesPairing::DECLARATION`

An inherent constant, readable from the type before any instance exists:

`PairingDeclaration { curve: PairingCurve::Bls12381, verifier_group_arithmetic: VerifierGroupArithmetic::BothGroups, precompile_encoding: PrecompileEncoding::Eip2537, target_group_encoding: TargetGroupEncodingIdentifier::Bls12381V1, adapter_version: 1, interface_version: PAIRING_INTERFACE_VERSION }`

The trait constant `IPairingAdapter::DECLARATION` is this constant.

## `Bls12381Halo2curvesPairing::CONCRETE`

The trait constant `PairingConcrete::Bls12381Halo2curves`.

## `g1_generator(&self, params: G1GeneratorParams, payload: G1GeneratorPayload) -> G1GeneratorReturn<Bls12381Halo2curvesG1>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| generated | any | none | `G1Affine::generator()` | `Ok(G1GeneratorSuccessReturn { point })`, the generator in the owned group type |

## `g2_generator(&self, params: G2GeneratorParams, payload: G2GeneratorPayload) -> G2GeneratorReturn<Bls12381Halo2curvesG2>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| generated | any | none | `G2Affine::generator()` | `Ok(G2GeneratorSuccessReturn { point })`, the generator in the owned group type |

## `add_g1(&self, params: AddG1Params, payload: AddG1Payload<Bls12381Halo2curvesG1>) -> AddG1Return<Bls12381Halo2curvesG1>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| summed | any | none | `to_curve()` on each payload point, the projective `+`, then `to_affine()` | `Ok(AddG1SuccessReturn { sum })` |

## `add_g2(&self, params: AddG2Params, payload: AddG2Payload<Bls12381Halo2curvesG2>) -> AddG2Return<Bls12381Halo2curvesG2>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| summed | any | none | `to_curve()` on each payload point, the projective `+`, then `to_affine()` | `Ok(AddG2SuccessReturn { sum })` |

## `mul_g1(&self, params: MulG1Params, payload: MulG1Payload<Bls12381Halo2curvesG1, Bls12381Halo2curvesScalar>) -> MulG1Return<Bls12381Halo2curvesG1>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| multiplied | any | none | `to_curve()` on the point, the projective `*` the scalar's `Fr`, then `to_affine()` | `Ok(MulG1SuccessReturn { product })`; the payload, holding the scalar, drops at the end of the call and the scalar is cleared |

## `mul_g2(&self, params: MulG2Params, payload: MulG2Payload<Bls12381Halo2curvesG2, Bls12381Halo2curvesScalar>) -> MulG2Return<Bls12381Halo2curvesG2>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| multiplied | any | none | `to_curve()` on the point, the projective `*` the scalar's `Fr`, then `to_affine()` | `Ok(MulG2SuccessReturn { product })`; the payload, holding the scalar, drops at the end of the call and the scalar is cleared |

## `msm_g1(&self, params: MsmG1Params, payload: MsmG1Payload<Bls12381Halo2curvesG1, Bls12381Halo2curvesScalar>) -> MsmG1Return<Bls12381Halo2curvesG1>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| summed | any | none | split the terms into a `Vec` of affine bases and a `Vec<Fr>` of the same length, then `msm_best(&scalars, &bases)`, then `to_affine()` | `Ok(MsmG1SuccessReturn { sum })`; every element of the `Vec<Fr>` is then set to `Fr::ZERO` and the vector passed to `black_box` |

## `msm_g2(&self, params: MsmG2Params, payload: MsmG2Payload<Bls12381Halo2curvesG2, Bls12381Halo2curvesScalar>) -> MsmG2Return<Bls12381Halo2curvesG2>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| summed | any | none | split the terms into a `Vec` of affine bases and a `Vec<Fr>` of the same length, then `msm_best(&scalars, &bases)`, then `to_affine()` | `Ok(MsmG2SuccessReturn { sum })`; every element of the `Vec<Fr>` is then set to `Fr::ZERO` and the vector passed to `black_box` |

## `pairing_product_is_one(&self, params: PairingProductIsOneParams, payload: PairingProductIsOnePayload<Bls12381Halo2curvesG1, Bls12381Halo2curvesG2>) -> PairingProductIsOneReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| evaluated | any | none | `Bls12381::multi_miller_loop` over the terms as a `Vec<(&G1Affine, &G2Affine)>` in term order, then `final_exponentiation()` | `Ok(PairingProductIsOneSuccessReturn { is_one })`, where `is_one` is `bool::from` of the result's `is_identity()` |

## `decode_g1(&self, params: DecodeG1Params, payload: &[u8]) -> DecodeG1Return<Bls12381Halo2curvesG1>`

A coordinate is EIP-2537's 64 bytes: the first 16 zero, the last 48 the field element big-endian. Copied into a `[u8; 48]`, reversed to little-endian, and read by `Fq::from_repr`, the last 48 are some below the modulus. A nonzero byte among the first 16, or 48 bytes at or above the base field modulus, is non-canonical.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| wrong length | `<[u8; 128]>::try_from(payload)` fails | the length check | none | `Err(DecodeG1ErrorReturn::WrongLength { expected: 128, actual: payload.len() })` |
| non-canonical coordinate | either 64-byte coordinate is non-canonical | the padding check and the `CtOption` conversion, per coordinate | `Fq::from_repr` | `Err(DecodeG1ErrorReturn::NonCanonicalCoordinate)` |
| identity | both coordinates are zero | the zero check | none | `Ok(DecodeG1SuccessReturn { point })` holding `G1Affine::identity()` |
| off the curve | `G1Affine::from_xy(x, y)` is none | the curve check | `G1Affine::from_xy` | `Err(DecodeG1ErrorReturn::NotOnCurve)` |
| outside the subgroup | `bool::from(point.to_curve().is_torsion_free())` is false | the subgroup check | `to_curve`, `is_torsion_free` | `Err(DecodeG1ErrorReturn::NotInSubgroup)` |
| valid | every check passes | all of the above | as above | `Ok(DecodeG1SuccessReturn { point })` |

BLS12-381's first group has a nontrivial cofactor, so the subgroup branch is reachable here as well as in the second group.

## `decode_g2(&self, params: DecodeG2Params, payload: &[u8]) -> DecodeG2Return<Bls12381Halo2curvesG2>`

The same branches in the same order over 256 bytes, the length checked by `<[u8; 256]>::try_from(payload)`, read as `x.c0`, `x.c1`, `y.c0`, `y.c1`, each a 64-byte coordinate, assembled by `Fq2::new(c0, c1)`, with `DecodeG2ErrorReturn` and `expected: 256`. The identity is all four coordinates zero.

## `decode_scalar(&self, params: DecodeScalarParams, payload: &[u8]) -> DecodeScalarReturn<Bls12381Halo2curvesScalar>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| wrong length | `<[u8; 32]>::try_from(payload)` fails | the length check | none | `Err(DecodeScalarErrorReturn::WrongLength { expected: 32, actual: payload.len() })` |
| non-canonical | the bytes, reversed to little-endian and read by `Fr::from_repr`, are none — that is, they are at least the group order | the `CtOption` conversion | `Fr::from_repr` | `Err(DecodeScalarErrorReturn::NonCanonical)` |
| valid | every check passes | all of the above | as above | `Ok(DecodeScalarSuccessReturn { scalar })` |

EIP-2537's MSM accepts any 256-bit scalar; this decoder, which produces an owned scalar, admits only the canonical ones.

## `encode_g1(&self, params: EncodeG1Params, payload: EncodeG1Payload<Bls12381Halo2curvesG1>) -> EncodeG1Return<Bls12381Halo2curvesEncodedG1>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| encoded | any | the identity check: a point whose `coordinates()` is none encodes to 128 zero bytes | `coordinates()`, then `to_repr()` reversed to big-endian per coordinate, each written after its 16 zero bytes to its fixed 64-byte position | `Ok(EncodeG1SuccessReturn { bytes })` holding `Bls12381Halo2curvesEncodedG1` — 128 zero bytes for the identity; otherwise `x()` then `y()`, each 16 zero bytes followed by the 48 big-endian bytes |

## `encode_g2(&self, params: EncodeG2Params, payload: EncodeG2Payload<Bls12381Halo2curvesG2>) -> EncodeG2Return<Bls12381Halo2curvesEncodedG2>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| encoded | any | the identity check: a point whose `coordinates()` is none encodes to 256 zero bytes | `coordinates()`, then `to_repr()` reversed to big-endian per coordinate, each written after its 16 zero bytes to its fixed 64-byte position | `Ok(EncodeG2SuccessReturn { bytes })` holding `Bls12381Halo2curvesEncodedG2` — 256 zero bytes for the identity; otherwise `x().c0()`, `x().c1()`, `y().c0()`, `y().c1()`, each 16 zero bytes followed by the 48 big-endian bytes |

## `encode_scalar(&self, params: EncodeScalarParams, payload: EncodeScalarPayload<Bls12381Halo2curvesScalar>) -> EncodeScalarReturn<Bls12381Halo2curvesEncodedScalar>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| encoded | any | none | `to_repr()` reversed to 32 big-endian bytes | `Ok(EncodeScalarSuccessReturn { bytes })`, the bytes in `Bls12381Halo2curvesEncodedScalar` moved into a `Secret` |

## `Bls12381Halo2curvesScalar::UNIFORM_BYTES_LENGTH`

`64` — twice the byte width of the group order.

## `Bls12381Halo2curvesScalar::sample_from_uniform_bytes(params: SampleUniformScalarParams, payload: SampleUniformScalarPayload) -> SampleUniformScalarReturn<Bls12381Halo2curvesScalar>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| wrong length | `<&[u8; 64]>::try_from(payload.uniform.expose().as_slice())` fails | the length check | `Secret::expose` | `Err(SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual })` |
| sampled | the length is 64 | none | the 64 bytes are copied into a local `[u8; 64]` and reversed, so the big-endian integer the arkworks concrete reads is the little-endian integer halo2curves reads; `Fr::from_uniform_bytes` over the copy, which is then zeroized | `Ok(SampleUniformScalarSuccessReturn { scalar })`, the scalar moved into a `Secret`; the payload's `Secret` zeroizes the input when it drops |

## `add_scalar(&self, params: AddScalarParams, payload: AddScalarPayload<Bls12381Halo2curvesScalar>) -> AddScalarReturn<Bls12381Halo2curvesScalar>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| summed | any | none | `Fr`'s `+` over `payload.left.value` and `payload.right.value` | `Ok(AddScalarSuccessReturn { sum })`, the sum modulo the group order in the owned scalar type; the payload's scalars are cleared as it drops |

## `mul_scalar(&self, params: MulScalarParams, payload: MulScalarPayload<Bls12381Halo2curvesScalar>) -> MulScalarReturn<Bls12381Halo2curvesScalar>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| multiplied | any | none | `Fr`'s `*` over `payload.left.value` and `payload.right.value` | `Ok(MulScalarSuccessReturn { product })`, the product modulo the group order in the owned scalar type; the payload's scalars are cleared as it drops |

## `neg_scalar(&self, params: NegScalarParams, payload: NegScalarPayload<Bls12381Halo2curvesScalar>) -> NegScalarReturn<Bls12381Halo2curvesScalar>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| negated | any | none | `Fr`'s unary `-` over `payload.scalar.value` | `Ok(NegScalarSuccessReturn { negation })`, the group order minus the scalar, and zero for zero |

## `neg_g1(&self, params: NegG1Params, payload: NegG1Payload<Bls12381Halo2curvesG1>) -> NegG1Return<Bls12381Halo2curvesG1>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| negated | any | none | the affine point's unary `-` over `payload.point.value` | `Ok(NegG1SuccessReturn { negation })`, `(x, p - y)` for a point `(x, y)` and the identity for the identity |

## `neg_g2(&self, params: NegG2Params, payload: NegG2Payload<Bls12381Halo2curvesG2>) -> NegG2Return<Bls12381Halo2curvesG2>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| negated | any | none | the affine point's unary `-` over `payload.point.value` | `Ok(NegG2SuccessReturn { negation })`, `(x, p - y)` for a point `(x, y)` and the identity for the identity |

## `is_identity_g1(&self, params: IsIdentityG1Params, payload: IsIdentityG1Payload<Bls12381Halo2curvesG1>) -> IsIdentityG1Return`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| tested | any | none | `is_identity()` on `payload.point.value` | `Ok(IsIdentityG1SuccessReturn { is_identity })`, `bool::from` of the returned `Choice` — `true` exactly for the identity |

## `is_identity_g2(&self, params: IsIdentityG2Params, payload: IsIdentityG2Payload<Bls12381Halo2curvesG2>) -> IsIdentityG2Return`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| tested | any | none | `is_identity()` on `payload.point.value` | `Ok(IsIdentityG2SuccessReturn { is_identity })`, `bool::from` of the returned `Choice` — `true` exactly for the identity |

## `pairing_product(&self, params: PairingProductParams, payload: PairingProductPayload<Bls12381Halo2curvesG1, Bls12381Halo2curvesG2>) -> PairingProductReturn<Bls12381Halo2curvesGt>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| evaluated | any | none | `Bls12381::multi_miller_loop` over the terms as a `Vec<(&G1Affine, &G2Affine)>` in term order, then `final_exponentiation()` as `gt`, then `&gt * &self.reduced_pairing_correction`, the exponentiation that brings the library's cubed value to the identifier's exact value | `Ok(PairingProductSuccessReturn { product })` holding the corrected `Gt` in the owned target-group type; an empty term list yields the target group's identity, which the exponentiation preserves |

## `encode_gt(&self, params: EncodeGtParams, payload: EncodeGtPayload<Bls12381Halo2curvesGt>) -> EncodeGtReturn<Bls12381Halo2curvesEncodedGt>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| encoded | any | none | `inner()` on `payload.value.value`, then each of the twelve `Fq` coefficients' `to_repr()` reversed to 48 big-endian bytes, written into its fixed position in the tower order `c0.c0.c0`, `c0.c0.c1`, `c0.c1.c0`, `c0.c1.c1`, `c0.c2.c0`, `c0.c2.c1`, `c1.c0.c0`, `c1.c0.c1`, `c1.c1.c0`, `c1.c1.c1`, `c1.c2.c0`, `c1.c2.c1` in a 576-byte buffer | `Ok(EncodeGtSuccessReturn { bytes })`, a `Secret<Bls12381Halo2curvesEncodedGt>` built from the complete buffer with no variable-width intermediate; the target group's identity encodes as 47 zero bytes, `01`, and 528 zero bytes |

## `scalar_field_order(&self, params: ScalarFieldOrderParams, payload: ScalarFieldOrderPayload) -> ScalarFieldOrderReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| read | any | none | `(-Fr::ONE).to_repr()`; the bytes are reversed to big-endian and the last byte's lowest bit is set, the group order being odd and its predecessor even | `Ok(ScalarFieldOrderSuccessReturn { bytes })`, 32 bytes |

The error arm has no branch.

## `g1_outside_subgroup_encoding(&self, params: G1OutsideSubgroupEncodingParams, payload: G1OutsideSubgroupEncodingPayload) -> G1OutsideSubgroupEncodingReturn<Bls12381Halo2curvesEncodedG1>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| exhausted | the search below yields no point | the search's result | the search | `Err(G1OutsideSubgroupEncodingErrorReturn::SearchExhausted)` |
| found | the search returns a point | per `x` from `successors(Some(Fq::ONE), \|x\| Some(*x + Fq::ONE))`, `y` from `(x.square() * x + G1Affine::b()).sqrt()`, the point from `G1Affine::from_xy(x, y)`, kept when `to_curve().is_torsion_free()` is false; then, with `negated = -y`, the point is kept when the big-endian bytes of `y`, its `to_repr()` reversed, are not greater than those of `negated`, and replaced by its affine negation otherwise | `self.encode_g1(EncodeG1Params, EncodeG1Payload { point: Bls12381Halo2curvesG1 { value } })`, unpacked irrefutably | `Ok(G1OutsideSubgroupEncodingSuccessReturn { bytes: Some(bytes) })`, the `Bls12381Halo2curvesEncodedG1` `encode_g1` returns |

No input takes the exhausted branch and it has no unit test.

## `g2_outside_subgroup_encoding(&self, params: G2OutsideSubgroupEncodingParams, payload: G2OutsideSubgroupEncodingPayload) -> G2OutsideSubgroupEncodingReturn<Bls12381Halo2curvesEncodedG2>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| exhausted | the search below yields no point | the search's result | the search | `Err(G2OutsideSubgroupEncodingErrorReturn::SearchExhausted)` |
| found | the search returns a point | per `c0` from `successors(Some(Fq::ONE), \|c0\| Some(*c0 + Fq::ONE))`, `x = Fq2::new(c0, Fq::ZERO)`, `y` from `(x.square() * x + G2Affine::b()).sqrt()`, the point from `G2Affine::from_xy(x, y)`, kept when outside the subgroup; then, with `negated = -y`, the point is kept when the pair of the big-endian bytes of `y.c1()` and of `y.c0()` is not greater than the same pair for `negated`, and replaced by its affine negation otherwise | `self.encode_g2(EncodeG2Params, EncodeG2Payload { point: Bls12381Halo2curvesG2 { value } })`, unpacked irrefutably | `Ok(G2OutsideSubgroupEncodingSuccessReturn { bytes })`, the `Bls12381Halo2curvesEncodedG2` `encode_g2` returns |

## Ordering and edges

- Every decoder checks in the stated order — length, then canonicality, then identity, then curve, then subgroup — and copies each 48-byte coordinate out of the fixed-size array before reversing it to little-endian.
- An empty `msm` term list yields the identity; an empty `pairing_product_is_one` term list yields `is_one: true`, as EIP-2537 does for empty input.
- Clearing: halo2curves' `Fr` implements no `Zeroize`, so `Bls12381Halo2curvesScalar`'s `Zeroize` implementation and its `Drop` set `value` to `Fr::ZERO` and pass `&self.value` to `black_box`, which keeps the clearing from being removed as a dead store; every clone a consumer places in a payload is cleared when the payload drops. The same zero-then-`black_box` treatment clears the `Vec<Fr>` an `msm` builds. halo2curves' affine points and `Gt` likewise implement no `Zeroize`, so `Bls12381Halo2curvesG1`, `Bls12381Halo2curvesG2`, and `Bls12381Halo2curvesGt` clear by setting `value` to `G1Affine::identity()`, `G2Affine::identity()`, or `Gt::identity()` and passing `&self.value` to `black_box`, in their `Zeroize` implementations and their `Drop`.
- Each outside-the-subgroup search is ascending from one and stops at the first on-curve point outside the subgroup; the choice between a point and its negation follows the search and precedes the encoding, and the same call always returns the same bytes.
- `params` carries no control and is not read in any method, and no reference method reads its payload.

