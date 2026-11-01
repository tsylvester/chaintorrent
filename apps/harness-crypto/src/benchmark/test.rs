#![allow(clippy::expect_used)]

use super::provides::{
    IPairingBenchmarkCaseConsumer, PairingBenchmarkCaseParamsOverrides,
    PairingBenchmarkErrorReturn, PairingBenchmarkReturn, build_pairing_benchmark,
    build_pairing_benchmark_case_deps, build_pairing_benchmark_case_params,
    build_pairing_benchmark_case_payload, run_pairing_benchmark_case,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParamsOverrides, IPairingArithmetic,
    IPairingConsumer, IPairingReference, MockIPairingAdapterFailureMode, PairingConcrete,
    SampleUniformScalarErrorReturn, build_create_pairing_params,
};
use random::{
    CreateRandomSourceParamsOverrides, FillBytesErrorReturn, IRandomSourceAdapter,
    MockIRandomSourceAdapterFailureMode, RandomSourceKind, build_create_random_source_params,
};

/// Contract: `self.random.fill_bytes` returns `Err(error)` for either scalar →
///   `Err(PairingBenchmarkErrorReturn::FillBytes(error))`, with nothing timed.
/// Arrange: `params` from `build_pairing_benchmark_case_params` with the
///   `random_source` override set to `build_create_random_source_params`
///   carrying the kind override `RandomSourceKind::Mock(
///   MockIRandomSourceAdapterFailureMode::FillBytesRefused)`, the randomness
///   family's mock concrete refusing every draw, a mode that differs from the
///   builder's default `Succeeds`, so a benchmark that ignores a refused draw
///   fails; the `pairing` selection at its default, the pairing family's mock
///   concrete.
/// Act:    `run_pairing_benchmark_case(&deps, params,
///   build_pairing_benchmark_case_payload(Default::default()))`, whose consumer
///   builds the benchmark over the received source and makes the single call
///   `benchmark.consume_pairing(params, payload)`, the only function of its
///   module that runs.
/// Assert: `result.is_ok()`, the runner's level having run;
///   `result.ok().and_then(|success| success.output.err())` equals
///   `Some(PairingBenchmarkErrorReturn::FillBytes(
///   FillBytesErrorReturn::MockIRandomSourceAdapter))`, the whole error — the
///   `FillBytes` variant is the subject's wrapping and the carried error is the
///   mock concrete's own variant written as a literal.
#[test]
fn consume_pairing_returns_the_draw_error_unchanged() {
    struct Consumer;

    impl IPairingBenchmarkCaseConsumer for Consumer {
        type Output = PairingBenchmarkReturn;

        fn consume_pairing_benchmark_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            random: Box<dyn IRandomSourceAdapter>,
            params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let benchmark = build_pairing_benchmark(random, Default::default());
            benchmark.consume_pairing(params, payload)
        }
    }

    // Arrange
    let deps = build_pairing_benchmark_case_deps(Consumer, Default::default());
    let params = build_pairing_benchmark_case_params(PairingBenchmarkCaseParamsOverrides {
        random_source: Some(build_create_random_source_params(
            CreateRandomSourceParamsOverrides {
                kind: Some(RandomSourceKind::Mock(
                    MockIRandomSourceAdapterFailureMode::FillBytesRefused,
                )),
            },
        )),
        ..Default::default()
    });

    // Act
    let result = run_pairing_benchmark_case(
        &deps,
        params,
        build_pairing_benchmark_case_payload(Default::default()),
    );

    // Assert
    assert!(result.is_ok());
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
/// Arrange: `params` from `build_pairing_benchmark_case_params` with no
///   override — the `random_source` selection at its default, the randomness
///   family's mock concrete drawing exactly the requested length, and the
///   `pairing` selection at its default, the pairing family's mock concrete,
///   whose sampling refuses an input whose length differs from its bound, so a
///   benchmark that requests another length fails.
/// Act:    `run_pairing_benchmark_case(&deps, params,
///   build_pairing_benchmark_case_payload(Default::default()))`, whose consumer
///   builds the benchmark over the received source and makes the single call
///   `benchmark.consume_pairing(params, payload)`, the only function of its
///   module that runs.
/// Assert: `result.is_ok()`, the runner's level having run;
///   `result.ok().map(|success| success.output.err())` equals `Some(None)`,
///   the runner's success arm present and the output's error arm empty, the
///   whole error shown on failure; the measurement's `concrete`, `iterations`,
///   and timings are not asserted.
#[test]
fn consume_pairing_draws_the_length_the_sampling_bound_requires() {
    struct Consumer;

    impl IPairingBenchmarkCaseConsumer for Consumer {
        type Output = PairingBenchmarkReturn;

        fn consume_pairing_benchmark_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            random: Box<dyn IRandomSourceAdapter>,
            params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let benchmark = build_pairing_benchmark(random, Default::default());
            benchmark.consume_pairing(params, payload)
        }
    }

    // Arrange
    let deps = build_pairing_benchmark_case_deps(Consumer, Default::default());
    let params = build_pairing_benchmark_case_params(Default::default());

    // Act
    let result = run_pairing_benchmark_case(
        &deps,
        params,
        build_pairing_benchmark_case_payload(Default::default()),
    );

    // Assert
    assert!(result.is_ok());
    assert_eq!(result.ok().map(|success| success.output.err()), Some(None));
}

/// Contract: `P::Scalar::sample_from_uniform_bytes` returns `Err(error)` →
///   `Err(PairingBenchmarkErrorReturn::SampleScalar(error))`, with nothing
///   timed.
/// Arrange: `params` from `build_pairing_benchmark_case_params` with the
///   `pairing` override set to `build_create_pairing_params` carrying the
///   `concrete` override `PairingConcrete::Mock(
///   MockIPairingAdapterFailureMode::SampleScalarWrongLength)`, the pairing
///   family's mock concrete refusing its sampling, a mode that differs from
///   the builder's default `Succeeds`, so a benchmark that ignores a refused
///   sampling fails; the `random_source` selection at its default, the
///   randomness family's mock concrete.
/// Act:    `run_pairing_benchmark_case(&deps, params,
///   build_pairing_benchmark_case_payload(Default::default()))`, whose consumer
///   builds the benchmark over the received source and makes the single call
///   `benchmark.consume_pairing(params, payload)`, the only function of its
///   module that runs.
/// Assert: `result.is_ok()`, the runner's level having run;
///   `result.ok().and_then(|success| success.output.err())` equals
///   `Some(PairingBenchmarkErrorReturn::SampleScalar(
///   SampleUniformScalarErrorReturn::WrongLength { expected: 16, actual: 16
///   }))`, the whole error — the `SampleScalar` variant is the subject's
///   wrapping and the carried error is the mock concrete's own value for that
///   mode written as a literal.
#[test]
fn consume_pairing_returns_the_sampling_error_unchanged() {
    struct Consumer;

    impl IPairingBenchmarkCaseConsumer for Consumer {
        type Output = PairingBenchmarkReturn;

        fn consume_pairing_benchmark_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            random: Box<dyn IRandomSourceAdapter>,
            params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let benchmark = build_pairing_benchmark(random, Default::default());
            benchmark.consume_pairing(params, payload)
        }
    }

    // Arrange
    let deps = build_pairing_benchmark_case_deps(Consumer, Default::default());
    let params = build_pairing_benchmark_case_params(PairingBenchmarkCaseParamsOverrides {
        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(PairingConcrete::Mock(
                MockIPairingAdapterFailureMode::SampleScalarWrongLength,
            )),
            ..Default::default()
        })),
        ..Default::default()
    });

    // Act
    let result = run_pairing_benchmark_case(
        &deps,
        params,
        build_pairing_benchmark_case_payload(Default::default()),
    );

    // Assert
    assert!(result.is_ok());
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
