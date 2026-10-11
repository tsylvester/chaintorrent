#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use core::time::Duration;
use harness_crypto::{PairingBenchmarkErrorReturn, build_pairing_benchmark};
use pairing::{
    CreatePairingParamsOverrides, CreatePairingPayload, MockIPairingAdapterFailureMode,
    PAIRING_CONCRETES, PairingConcrete, SampleUniformScalarErrorReturn, build_create_pairing_deps,
    build_create_pairing_params, create_pairing,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParamsOverrides, CreateRandomSourcePayload,
    RandomSourceKind, build_create_random_source_params, create_random_source,
};

/// Contract: entry `the_consumer_receives_the_concrete_the_params_name`; given
///   `params.concrete` is a variant of `PAIRING_CONCRETES` and the admissions
///   pass, the `concrete` of the `Ok` measurement equals `params.concrete`.
/// Boundary: the public `create_pairing`; the real chain is `create_pairing`,
///   the arm's `try_new`, `PairingBenchmark::consume_pairing`,
///   `OsRandomSource::fill_bytes`, `sample_from_uniform_bytes`, and the
///   concrete's group operations.
/// Mocked:   none.
/// Arrange:  every variant of the declared set in turn, including both
///   libraries of one curve, so a benchmark that returns a fixed concrete, or
///   the factory constructing a sibling library, fails; the source
///   `create_random_source` constructs for `RandomSourceKind::OperatingSystem`.
/// Act:      `create_pairing(&deps, params, CreatePairingPayload)` per concrete.
/// Assert:   per concrete, the measured `concrete` equals the variant from the
///   declared set, not the value the call happened to construct.
#[test]
fn the_consumer_receives_the_concrete_the_params_name() {
    for concrete in PAIRING_CONCRETES {
        // Arrange
        let Ok(created) = create_random_source(
            &CreateRandomSourceDeps,
            build_create_random_source_params(CreateRandomSourceParamsOverrides {
                kind: Some(RandomSourceKind::OperatingSystem),
            }),
            CreateRandomSourcePayload,
        );
        let benchmark = build_pairing_benchmark(created.adapter, Default::default());
        let deps = build_create_pairing_deps(benchmark, Default::default());
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });

        // Act
        let result = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
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
/// Boundary: the public `create_pairing`; the real chain is `create_pairing`,
///   the arm's `try_new`, `PairingBenchmark::consume_pairing`,
///   `OsRandomSource::fill_bytes`, `sample_from_uniform_bytes`, and the
///   concrete's group operations.
/// Mocked:   none.
/// Arrange:  every concrete of the declared set in turn, each sampling bound
///   its own, so a benchmark that draws a length the bound does not name fails
///   for the concrete whose bound differs; the source `create_random_source`
///   constructs for `RandomSourceKind::OperatingSystem`.
/// Act:      `create_pairing(&deps, params, CreatePairingPayload)` per concrete.
/// Assert:   per concrete, the factory's success arm is present and the
///   output's error arm is empty, the whole error shown on failure.
#[test]
fn a_sampled_scalar_encodes_through_secret() {
    for concrete in PAIRING_CONCRETES {
        // Arrange
        let Ok(created) = create_random_source(
            &CreateRandomSourceDeps,
            build_create_random_source_params(CreateRandomSourceParamsOverrides {
                kind: Some(RandomSourceKind::OperatingSystem),
            }),
            CreateRandomSourcePayload,
        );
        let benchmark = build_pairing_benchmark(created.adapter, Default::default());
        let deps = build_create_pairing_deps(benchmark, Default::default());
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });

        // Act
        let result = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert_eq!(result.ok().map(|success| success.output.err()), Some(None));
    }
}

/// Contract: entry `a_sampling_refusal_reaches_the_caller_unchanged`; a
///   sampling refusal is the output's
///   `Err(PairingBenchmarkErrorReturn::SampleScalar(error))`, the error whole.
/// Boundary: the public `create_pairing`; the real chain is `create_pairing`,
///   the mock arm's `try_new`, `PairingBenchmark::consume_pairing`,
///   `OsRandomSource::fill_bytes`, and `sample_from_uniform_bytes`.
/// Mocked:   the pairing family's mock concrete, selected by configuration;
///   this block does not prove a real concrete's sampling.
/// Arrange:  `params` from `build_create_pairing_params` with the `concrete`
///   override set to `PairingConcrete::Mock(
///   MockIPairingAdapterFailureMode::SampleScalarWrongLength)`, against the
///   `Ok` outputs of the entries above, so a benchmark that ignores a refused
///   sampling fails; the source `create_random_source` constructs for
///   `RandomSourceKind::OperatingSystem`.
/// Act:      `create_pairing(&deps, params, CreatePairingPayload)`.
/// Assert:   the observed output error equals
///   `PairingBenchmarkErrorReturn::SampleScalar(
///   SampleUniformScalarErrorReturn::WrongLength { expected: 16, actual: 16 })`,
///   the whole error, the carried error the mock concrete's own value for that
///   mode written as a literal.
#[test]
fn a_sampling_refusal_reaches_the_caller_unchanged() {
    // Arrange
    let Ok(created) = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
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

/// Contract: entry `every_operation_is_timed_on_every_concrete`; the `Ok`
///   measurement's timings for `mul_g1`, `mul_g2`, `msm_g1`, `msm_g2`, and
///   `pairing_product_is_one` are each greater than `Duration::ZERO`.
/// Boundary: the public `create_pairing`; the real chain is `create_pairing`,
///   the arm's `try_new`, `PairingBenchmark::consume_pairing`,
///   `OsRandomSource::fill_bytes`, `sample_from_uniform_bytes`, and the
///   concrete's group operations and clock reads.
/// Mocked:   none.
/// Arrange:  every concrete of the declared set in turn, whose operations are
///   real, so a timing left at its zero default fails; the source
///   `create_random_source` constructs for `RandomSourceKind::OperatingSystem`.
/// Act:      `create_pairing(&deps, params, CreatePairingPayload)` per concrete.
/// Assert:   per concrete, each of the five timings is greater than
///   `Duration::ZERO`; the exact means are not asserted.
#[test]
fn every_operation_is_timed_on_every_concrete() {
    for concrete in PAIRING_CONCRETES {
        // Arrange
        let Ok(created) = create_random_source(
            &CreateRandomSourceDeps,
            build_create_random_source_params(CreateRandomSourceParamsOverrides {
                kind: Some(RandomSourceKind::OperatingSystem),
            }),
            CreateRandomSourcePayload,
        );
        let benchmark = build_pairing_benchmark(created.adapter, Default::default());
        let deps = build_create_pairing_deps(benchmark, Default::default());
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });

        // Act
        let result = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
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
