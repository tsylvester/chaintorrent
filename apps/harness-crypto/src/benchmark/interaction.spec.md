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
| measured | both scalars `a` and `b` sampled | none | untimed setup: `g1_generator` and `g2_generator` for `g1` and `g2`, then `mul_g1(g1, a)` and `mul_g2(g2, b)` for `p1` and `p2`; then, per operation, `Instant::now()` is read, the operation runs `self.iterations.get()` times, and `elapsed()` divided by `self.iterations.get()` is its mean: `mul_g1` of a clone of `p1` by a clone of `a`; `mul_g2` of a clone of `p2` by a clone of `b`; `msm_g1` over the terms `(p1, a)` and `(g1, b)`; `msm_g2` over the terms `(p2, b)` and `(g2, a)`; `pairing_product_is_one` over the terms `(p1, g2)` and `(g1, p2)`, each term's elements cloned per call | `Ok(PairingBenchmarkSuccessReturn { concrete: P::CONCRETE, iterations: self.iterations, timings })`, the measured concrete and count carried with each field's mean time of one call |

## Ordering and edges

- Both draws and both samplings precede any timing, so a failure returns before the clock is read; nothing is timed on either error branch. `P::CONCRETE` is carried into the result; no independently supplied metadata is read.
- Every adapter call returns `Result<_, Infallible>` and is unpacked irrefutably; the error arm exists in the signature but no branch produces it.
- The scalars are cloned from their `Secret`s by `expose().clone()`, and each clone is cleared when the payload holding it drops.
- `payload.declaration` is not read; `_params` carries nothing and is not read.
- The body names no curve library and no concrete: `P` is bound only by `IPairingAdapter`, and `P::Scalar` only by `ISampleUniformScalar`.
- Decoding, encoding, and sampling itself are not timed.

## Integration: callee dispositions

The `pairing/factory` entries this node's route reaches, each proven by the public integration block of the same name. The route of each is `create_pairing`, the arm's concrete through its `try_new`, `PairingBenchmark::consume_pairing`, and the calls it makes on that concrete and on the source `create_random_source` constructs; the outer-edge collaborators on that route are the operating system's generator and monotonic clock, both run real, except where an entry names the pairing family's mock concrete.

- `the_consumer_receives_the_concrete_the_params_name`: carried; restated where this node transforms it: the `concrete` of the `Ok` measurement the benchmark returns equals `params.concrete`, read from the benchmark's output instead of a consumer's own reading of `P::CONCRETE`.
- `a_sampled_scalar_encodes_through_secret`: carried; restated where this node transforms it: the benchmark draws `P::Scalar::UNIFORM_BYTES_LENGTH` bytes into a `Secret`, samples both scalars with `sample_from_uniform_bytes`, and consumes them in the group operations instead of `encode_scalar`, so the output is `Ok` for every concrete.
- `a_sampling_refusal_reaches_the_caller_unchanged`: carried; restated where this node transforms it: a sampling refusal is the output's `Err(PairingBenchmarkErrorReturn::SampleScalar(error))`, the error whole; its concrete is the pairing family's mock concrete selected by `PairingConcrete::Mock(MockIPairingAdapterFailureMode::SampleScalarWrongLength)`.

## Integration: own entries

Each entry is named and proven by the public integration block of the same name.

- `every_operation_is_timed_on_every_concrete`: condition the benchmark runs through `create_pairing` on a concrete of `PAIRING_CONCRETES` with the source `create_random_source` constructs for `RandomSourceKind::OperatingSystem`; outcome `Ok` whose timings for `mul_g1`, `mul_g2`, `msm_g1`, `msm_g2`, and `pairing_product_is_one` are each greater than `Duration::ZERO`; variation every concrete of the declared set, whose operations are real, so a timing left at its zero default fails; edge each operation runs on scalars sampled from the draw and on the concrete's own points, so none is skipped.

## Integration: public surface

An outside caller constructs `PairingBenchmark` with `PairingBenchmark::try_new` from `PairingBenchmarkConstructorParams` holding the source `create_random_source` returns and an iteration count, passes it as `CreatePairingDeps { consumer }` to `create_pairing` from `pairing`'s public surface with `CreatePairingParams` naming a concrete and `CreatePairingPayload`, and observes the benchmark's `PairingBenchmarkReturn` in `CreatePairingSuccessReturn`. The entries proven are every carried entry and the own entry above. The route of each is as the callee dispositions state; the outer-edge collaborators are the operating system's generator and monotonic clock, run real, and, for `a_sampling_refusal_reaches_the_caller_unchanged`, the pairing family's mock concrete. The fixtures are `build_pairing_benchmark`, `build_create_random_source_params`, `build_create_pairing_deps`, and `build_create_pairing_params`.

