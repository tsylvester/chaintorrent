#![allow(clippy::expect_used)]

use super::provides::{PairingBenchmarkErrorReturn, build_pairing_benchmark};
use pairing::{
    CreatePairingParamsOverrides, CreatePairingPayload, MockIPairingAdapterFailureMode,
    PairingConcrete, SampleUniformScalarErrorReturn, build_create_pairing_deps,
    build_create_pairing_params, create_pairing,
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
    let benchmark = build_pairing_benchmark(created.adapter, Default::default());
    let deps = build_create_pairing_deps(benchmark, Default::default());
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

/// Contract: both scalars drawn with `P::Scalar::UNIFORM_BYTES_LENGTH` bytes
///   and sampled → `Ok(PairingBenchmarkSuccessReturn { concrete: P::CONCRETE,
///   iterations: self.iterations, timings })`.
/// Arrange: `created` the success arm of `create_random_source` under
///   `build_create_random_source_params` with no override, the randomness
///   family's mock concrete drawing exactly the requested length;
///   `build_pairing_benchmark` with `created.adapter` handed as the random
///   source; `params` from `build_create_pairing_params` with no override,
///   selecting the pairing family's mock concrete, whose sampling refuses an
///   input whose length differs from its bound, so a benchmark that requests
///   another length fails.
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
    let random_params = build_create_random_source_params(Default::default());
    let Ok(created) = create_random_source(
        &CreateRandomSourceDeps,
        random_params,
        CreateRandomSourcePayload,
    );
    let benchmark = build_pairing_benchmark(created.adapter, Default::default());
    let deps = build_create_pairing_deps(benchmark, Default::default());
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
///   ignores a refused sampling fails; `build_pairing_benchmark` with
///   `created.adapter` handed as the random source, `created` the success arm
///   of `create_random_source` under `build_create_random_source_params` with
///   no override, the randomness family's mock concrete.
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
    let random_params = build_create_random_source_params(Default::default());
    let Ok(created) = create_random_source(
        &CreateRandomSourceDeps,
        random_params,
        CreateRandomSourcePayload,
    );
    let benchmark = build_pairing_benchmark(created.adapter, Default::default());
    let deps = build_create_pairing_deps(benchmark, Default::default());
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
