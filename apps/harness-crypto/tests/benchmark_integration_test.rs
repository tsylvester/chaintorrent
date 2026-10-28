#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use core::time::Duration;
use harness_crypto::{
    IPairingBenchmarkCaseConsumer, PairingBenchmarkCaseParamsOverrides,
    PairingBenchmarkErrorReturn, PairingBenchmarkReturn, build_pairing_benchmark,
    build_pairing_benchmark_case_deps, build_pairing_benchmark_case_params,
    build_pairing_benchmark_case_payload, run_pairing_benchmark_case,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParamsOverrides, IPairingArithmetic,
    IPairingConsumer, IPairingReference, MockIPairingAdapterFailureMode, PAIRING_CONCRETES,
    PairingConcrete, SampleUniformScalarErrorReturn, build_create_pairing_params,
};
use random::{
    CreateRandomSourceParamsOverrides, IRandomSourceAdapter, RandomSourceKind,
    build_create_random_source_params,
};

/// Contract: entry `the_consumer_receives_the_concrete_the_params_name`; given
///   `params.pairing`'s `concrete` is a variant of `PAIRING_CONCRETES` and the
///   admissions pass, the `concrete` of the `Ok` measurement equals that
///   variant.
/// Boundary: the public `create_pairing`, driven by the case runner; the real
///   chain is `create_pairing`, the arm's `try_new`,
///   `PairingBenchmark::consume_pairing`, `OsRandomSource::fill_bytes`,
///   `sample_from_uniform_bytes`, and the concrete's group operations.
/// Mocked:   none.
/// Arrange:  every variant of the declared set in turn, including both
///   libraries of one curve, so a benchmark that returns a fixed concrete, or
///   the factory constructing a sibling library, fails; the source
///   `create_random_source` constructs for `RandomSourceKind::OperatingSystem`.
/// Act:      `run_pairing_benchmark_case(&deps, params,
///   build_pairing_benchmark_case_payload(Default::default()))` per concrete.
/// Assert:   per concrete, `result.is_ok()`, the runner's level having run, and
///   the measured `concrete` equals the variant from the declared set, not the
///   value the call happened to construct.
#[test]
fn the_consumer_receives_the_concrete_the_params_name() {
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

    for concrete in PAIRING_CONCRETES {
        // Arrange
        let deps = build_pairing_benchmark_case_deps(Consumer, Default::default());
        let params = build_pairing_benchmark_case_params(PairingBenchmarkCaseParamsOverrides {
            pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            })),
            random_source: Some(build_create_random_source_params(
                CreateRandomSourceParamsOverrides {
                    kind: Some(RandomSourceKind::OperatingSystem),
                },
            )),
        });

        // Act
        let result = run_pairing_benchmark_case(
            &deps,
            params,
            build_pairing_benchmark_case_payload(Default::default()),
        );

        // Assert
        assert!(result.is_ok());
        assert!(
            result
                .ok()
                .and_then(|success| success.output.ok())
                .map(|measurement| measurement.concrete)
                == Some(*concrete)
        );
    }
}

/// Contract: entry `a_sampled_scalar_encodes_through_secret`; the draw of
///   `P::Scalar::UNIFORM_BYTES_LENGTH` bytes crosses inside a `Secret` into
///   `sample_from_uniform_bytes`, and the sampled scalars are consumed by the
///   concrete's group operations, so the output is `Ok`.
/// Boundary: the public `create_pairing`, driven by the case runner; the real
///   chain is `create_pairing`, the arm's `try_new`,
///   `PairingBenchmark::consume_pairing`, `OsRandomSource::fill_bytes`,
///   `sample_from_uniform_bytes`, and the concrete's group operations.
/// Mocked:   none.
/// Arrange:  every concrete of the declared set in turn, each sampling bound
///   its own, so a benchmark that draws a length the bound does not name fails
///   for the concrete whose bound differs; the source `create_random_source`
///   constructs for `RandomSourceKind::OperatingSystem`.
/// Act:      `run_pairing_benchmark_case(&deps, params,
///   build_pairing_benchmark_case_payload(Default::default()))` per concrete.
/// Assert:   per concrete, `result.is_ok()`, the runner's level having run, and
///   `result.ok().map(|success| success.output.err())` equals `Some(None)`, the
///   runner's success arm present and the output's error arm empty, the whole
///   error shown on failure.
#[test]
fn a_sampled_scalar_encodes_through_secret() {
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

    for concrete in PAIRING_CONCRETES {
        // Arrange
        let deps = build_pairing_benchmark_case_deps(Consumer, Default::default());
        let params = build_pairing_benchmark_case_params(PairingBenchmarkCaseParamsOverrides {
            pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            })),
            random_source: Some(build_create_random_source_params(
                CreateRandomSourceParamsOverrides {
                    kind: Some(RandomSourceKind::OperatingSystem),
                },
            )),
        });

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
}

/// Contract: entry `a_sampling_refusal_reaches_the_caller_unchanged`; a
///   sampling refusal is the output's
///   `Err(PairingBenchmarkErrorReturn::SampleScalar(error))`, the error whole.
/// Boundary: the public `create_pairing`, driven by the case runner; the real
///   chain is `create_pairing`, the mock arm's `try_new`,
///   `PairingBenchmark::consume_pairing`, `OsRandomSource::fill_bytes`, and
///   `sample_from_uniform_bytes`.
/// Mocked:   the pairing family's mock concrete, selected by configuration;
///   this block does not prove a real concrete's sampling.
/// Arrange:  `params` from `build_pairing_benchmark_case_params` with the
///   `pairing` override set to `build_create_pairing_params` carrying the
///   `concrete` override `PairingConcrete::Mock(
///   MockIPairingAdapterFailureMode::SampleScalarWrongLength)`, against the
///   `Ok` outputs of the entries above, so a benchmark that ignores a refused
///   sampling fails; the source `create_random_source` constructs for
///   `RandomSourceKind::OperatingSystem`.
/// Act:      `run_pairing_benchmark_case(&deps, params,
///   build_pairing_benchmark_case_payload(Default::default()))`.
/// Assert:   `result.is_ok()`, the runner's level having run, and the observed
///   output error equals `PairingBenchmarkErrorReturn::SampleScalar(
///   SampleUniformScalarErrorReturn::WrongLength { expected: 16, actual: 16 })`,
///   the whole error, the carried error the mock concrete's own value for that
///   mode written as a literal.
#[test]
fn a_sampling_refusal_reaches_the_caller_unchanged() {
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
        random_source: Some(build_create_random_source_params(
            CreateRandomSourceParamsOverrides {
                kind: Some(RandomSourceKind::OperatingSystem),
            },
        )),
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

/// Contract: entry `every_operation_is_timed_on_every_concrete`; the `Ok`
///   measurement's timings for `mul_g1`, `mul_g2`, `msm_g1`, `msm_g2`, and
///   `pairing_product_is_one` are each greater than `Duration::ZERO`.
/// Boundary: the public `create_pairing`, driven by the case runner; the real
///   chain is `create_pairing`, the arm's `try_new`,
///   `PairingBenchmark::consume_pairing`, `OsRandomSource::fill_bytes`,
///   `sample_from_uniform_bytes`, and the concrete's group operations and clock
///   reads.
/// Mocked:   none.
/// Arrange:  every concrete of the declared set in turn, whose operations are
///   real, so a timing left at its zero default fails; the source
///   `create_random_source` constructs for `RandomSourceKind::OperatingSystem`.
/// Act:      `run_pairing_benchmark_case(&deps, params,
///   build_pairing_benchmark_case_payload(Default::default()))` per concrete.
/// Assert:   per concrete, `result.is_ok()`, the runner's level having run, and
///   each of the five timings is greater than `Duration::ZERO`; the exact
///   means are not asserted.
#[test]
fn every_operation_is_timed_on_every_concrete() {
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

    for concrete in PAIRING_CONCRETES {
        // Arrange
        let deps = build_pairing_benchmark_case_deps(Consumer, Default::default());
        let params = build_pairing_benchmark_case_params(PairingBenchmarkCaseParamsOverrides {
            pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            })),
            random_source: Some(build_create_random_source_params(
                CreateRandomSourceParamsOverrides {
                    kind: Some(RandomSourceKind::OperatingSystem),
                },
            )),
        });

        // Act
        let result = run_pairing_benchmark_case(
            &deps,
            params,
            build_pairing_benchmark_case_payload(Default::default()),
        );

        // Assert
        assert!(result.is_ok());
        let measurement = result
            .ok()
            .and_then(|success| success.output.ok())
            .expect("the measured arm");
        assert!(measurement.timings.mul_g1 > Duration::ZERO);
        assert!(measurement.timings.mul_g2 > Duration::ZERO);
        assert!(measurement.timings.msm_g1 > Duration::ZERO);
        assert!(measurement.timings.msm_g2 > Duration::ZERO);
        assert!(measurement.timings.pairing_product > Duration::ZERO);
    }
}
