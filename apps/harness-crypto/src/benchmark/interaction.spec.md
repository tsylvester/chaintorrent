# `benchmark` — interaction spec

Branch contract for the `benchmark` module of the `harness-crypto` crate: `PairingBenchmark`, an `IPairingConsumer` the harness passes to `create_pairing` once per `PairingConcrete`, timing the pairing family's operations on whichever concrete it is handed. Each branch states condition, decision, dependency call, and the exact return outcome.

## `PairingBenchmark::try_new(params: PairingBenchmarkConstructorParams) -> PairingBenchmarkTryNewReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| constructed | always | none | none | `Ok(PairingBenchmark)` holding `params.random` and `params.iterations` |

The error arm is uninhabited (`Result<_, Infallible>`): `NonZeroU32` already excludes the one invalid count, so no branch produces an error.

## `consume_pairing<P: IPairingAdapter>(&self, _params: ConsumePairingParams, payload: ConsumePairingPayload<P>) -> PairingBenchmarkReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| draw failed | `self.random.fill_bytes(FillBytesParams, FillBytesPayload { length: P::Scalar::UNIFORM_BYTES_LENGTH })` returns `Err(error)` for either scalar | the `Err` arm of the draw's return | `fill_bytes`, once per scalar, before any sampling | `Err(PairingBenchmarkErrorReturn::FillBytes(error))`, the callee's error unchanged; nothing is timed |
| sampling failed | `P::Scalar::sample_from_uniform_bytes(SampleUniformScalarParams, SampleUniformScalarPayload { uniform: draw.bytes })` returns `Err(error)` for either scalar | the `Err` arm of the sampling return | `sample_from_uniform_bytes`, once per drawn scalar, both draws taken first | `Err(PairingBenchmarkErrorReturn::SampleScalar(error))`, the callee's error unchanged; nothing is timed |
| measured | both scalars `a` and `b` sampled | none | untimed setup: `g1_generator` and `g2_generator` for `g1` and `g2`, then `mul_g1(g1, a)` and `mul_g2(g2, b)` for `p1` and `p2`; then, per operation, `Instant::now()` is read, the operation runs `self.iterations.get()` times, and `elapsed()` divided by `self.iterations.get()` is its mean: `mul_g1` of a clone of `p1` by a clone of `a`; `mul_g2` of a clone of `p2` by a clone of `b`; `msm_g1` over the terms `(p1, a)` and `(g1, b)`; `msm_g2` over the terms `(p2, b)` and `(g2, a)`; `pairing_product_is_one` over the terms `(p1, g2)` and `(g1, p2)`, each term's elements cloned per call | `Ok(PairingBenchmarkSuccessReturn { timings })`, each field the mean time of one call of its operation |

## Ordering and edges

- Both draws and both samplings precede any timing, so a failure returns before the clock is read; nothing is timed on either error branch.
- Every adapter call returns `Result<_, Infallible>` and is unpacked irrefutably; the error arm exists in the signature but no branch produces it.
- The scalars are cloned from their `Secret`s by `expose().clone()`, and each clone is cleared when the payload holding it drops.
- `payload.declaration` is not read; `_params` carries nothing and is not read.
- The body names no curve library and no concrete: `P` is bound only by `IPairingAdapter`, and `P::Scalar` only by `ISampleUniformScalar`.
- Decoding, encoding, and sampling itself are not timed.
