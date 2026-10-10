#![allow(clippy::expect_used)]

use super::provides::{
    PairingBenchmarkConstructorParamsOverrides, PairingBenchmarkErrorReturn,
    build_pairing_benchmark,
};
use core::num::NonZeroU32;
use pairing::{
    CreatePairingDepsOverrides, CreatePairingParamsOverrides, CreatePairingPayload,
    MockIPairingAdapterFailureMode, PairingConcrete, SampleUniformScalarErrorReturn,
    build_create_pairing_deps, build_create_pairing_params, create_pairing,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParamsOverrides, CreateRandomSourcePayload,
    FillBytesErrorReturn, MockIRandomSourceAdapterFailureMode, RandomSourceKind,
    build_create_random_source_params, create_random_source,
};

/// Contract: `self.random.fill_bytes` returns `Err(error)` for either scalar →
///   `Err(PairingBenchmarkErrorReturn::FillBytes(error))`, with nothing timed.
/// Arrange: the randomness family's mock concrete selected by
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)`
///   through `create_random_source`, a mode that refuses every draw and differs
///   from the builder's default `Succeeds`, so a benchmark that ignores a
///   refused draw fails; the benchmark built with `random` set to that source;
///   the pairing family's mock concrete selected by the params' default.
/// Act:    `create_pairing(&deps, params, CreatePairingPayload)`, whose consumer
///   is the benchmark, so the only function of its module that runs is
///   `PairingBenchmark::consume_pairing`.
/// Assert: `result.ok().and_then(|success| success.output.err())` equals
///   `Some(PairingBenchmarkErrorReturn::FillBytes(
///   FillBytesErrorReturn::MockIRandomSourceAdapter))`, the whole error — the
///   `FillBytes` variant is the subject's wrapping and the carried error is the
///   mock concrete's own variant written as a literal.
#[test]
fn consume_pairing_returns_the_draw_error_unchanged() {
    // Arrange
    let random_params = build_create_random_source_params(CreateRandomSourceParamsOverrides {
        kind: Some(RandomSourceKind::Mock(
            MockIRandomSourceAdapterFailureMode::FillBytesRefused,
        )),
    });
    let Ok(created) = create_random_source(
        &CreateRandomSourceDeps,
        random_params,
        CreateRandomSourcePayload,
    );
    let benchmark = build_pairing_benchmark(PairingBenchmarkConstructorParamsOverrides {
        random: Some(created.adapter),
        ..Default::default()
    });
    let deps = build_create_pairing_deps(CreatePairingDepsOverrides {
        consumer: Some(benchmark),
    });
    let params = build_create_pairing_params(CreatePairingParamsOverrides::default());

    // Act
    let result = create_pairing(&deps, params, CreatePairingPayload);

    // Assert
    assert_eq!(
        result.ok().and_then(|success| success.output.err()),
        Some(PairingBenchmarkErrorReturn::FillBytes(
            FillBytesErrorReturn::MockIRandomSourceAdapter
        ))
    );
}

/// Contract: both scalars drawn and sampled → `Ok(PairingBenchmarkSuccessReturn
///   { concrete: P::CONCRETE, iterations: self.iterations, timings })`.
/// Arrange: the randomness family's mock concrete from the builder's default
///   source; the benchmark built with `iterations` set to
///   `NonZeroU32::MIN.saturating_add(2)`, which is 3 and differs from the
///   builder's default of `NonZeroU32::MIN`, so a benchmark that returns a
///   fixed count fails; `params.concrete` read into `selected` before the call.
/// Act:    `create_pairing(&deps, params, CreatePairingPayload)`, whose consumer
///   is the benchmark, so the only function of its module that runs is
///   `PairingBenchmark::consume_pairing`.
/// Assert: the measurement extracted by `expect` carries `concrete` equal to
///   `selected` and `iterations.get()` equal to the literal `3`; the timings
///   are not asserted.
#[test]
fn consume_pairing_carries_the_selected_concrete_and_the_iteration_count() {
    // Arrange
    let benchmark = build_pairing_benchmark(PairingBenchmarkConstructorParamsOverrides {
        iterations: Some(NonZeroU32::MIN.saturating_add(2)),
        ..Default::default()
    });
    let deps = build_create_pairing_deps(CreatePairingDepsOverrides {
        consumer: Some(benchmark),
    });
    let params = build_create_pairing_params(CreatePairingParamsOverrides::default());
    let selected = params.concrete;

    // Act
    let result = create_pairing(&deps, params, CreatePairingPayload);

    // Assert
    let measurement = result
        .ok()
        .and_then(|success| success.output.ok())
        .expect("the measured arm");
    assert!(measurement.concrete == selected);
    assert_eq!(measurement.iterations.get(), 3);
}

/// Contract: both scalars drawn with `P::Scalar::UNIFORM_BYTES_LENGTH` bytes
///   and sampled → `Ok(PairingBenchmarkSuccessReturn { concrete: P::CONCRETE,
///   iterations: self.iterations, timings })`.
/// Arrange: `build_pairing_benchmark` with no override, whose source is the
///   randomness family's mock concrete drawing exactly the requested length;
///   `params` from `build_create_pairing_params` with no override, selecting
///   the pairing family's mock concrete, whose sampling refuses an input whose
///   length differs from its bound, so a benchmark that requests another
///   length fails.
/// Act:    `create_pairing(&deps, params, CreatePairingPayload)`, whose consumer
///   is the benchmark, so the only function of its module that runs is
///   `PairingBenchmark::consume_pairing`.
/// Assert: `result.ok().map(|success| success.output.err())` equals
///   `Some(None)`, the factory's success arm present and the output's error
///   arm empty; the measurement's `concrete`, `iterations`, and timings are
///   not asserted.
#[test]
fn consume_pairing_draws_the_length_the_sampling_bound_requires() {
    // Arrange
    let benchmark = build_pairing_benchmark(Default::default());
    let deps = build_create_pairing_deps(CreatePairingDepsOverrides {
        consumer: Some(benchmark),
    });
    let params = build_create_pairing_params(CreatePairingParamsOverrides::default());

    // Act
    let result = create_pairing(&deps, params, CreatePairingPayload);

    // Assert
    assert_eq!(result.ok().map(|success| success.output.err()), Some(None));
}

/// Contract: `P::Scalar::sample_from_uniform_bytes` returns `Err(error)` →
///   `Err(PairingBenchmarkErrorReturn::SampleScalar(error))`, with nothing
///   timed.
/// Arrange: `params` from `build_create_pairing_params` with the `concrete`
///   override set to `PairingConcrete::Mock(
///   MockIPairingAdapterFailureMode::SampleScalarWrongLength)`, a mode that
///   differs from the builder's default `Succeeds`, so a benchmark that
///   ignores a refused sampling fails; `build_pairing_benchmark` with no
///   override, its source the randomness family's mock concrete.
/// Act:    `create_pairing(&deps, params, CreatePairingPayload)`, whose consumer
///   is the benchmark, so the only function of its module that runs is
///   `PairingBenchmark::consume_pairing`.
/// Assert: `result.ok().and_then(|success| success.output.err())` equals
///   `Some(PairingBenchmarkErrorReturn::SampleScalar(
///   SampleUniformScalarErrorReturn::WrongLength { expected: 16, actual: 16
///   }))`, the whole error — the `SampleScalar` variant is the subject's
///   wrapping and the carried error is the mock concrete's own value for that
///   mode written as a literal.
#[test]
fn consume_pairing_returns_the_sampling_error_unchanged() {
    // Arrange
    let benchmark = build_pairing_benchmark(Default::default());
    let deps = build_create_pairing_deps(CreatePairingDepsOverrides {
        consumer: Some(benchmark),
    });
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Mock(
            MockIPairingAdapterFailureMode::SampleScalarWrongLength,
        )),
        ..Default::default()
    });

    // Act
    let result = create_pairing(&deps, params, CreatePairingPayload);

    // Assert
    assert_eq!(
        result.ok().and_then(|success| success.output.err()),
        Some(PairingBenchmarkErrorReturn::SampleScalar(
            SampleUniformScalarErrorReturn::WrongLength {
                expected: 16,
                actual: 16,
            }
        ))
    );
}
