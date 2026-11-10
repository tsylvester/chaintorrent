#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pairing::{
    CreatePairingErrorReturn, MockIPairingAdapterFailureMode, PRECOMPILE_ENCODINGS,
    PairingConcrete, SampleUniformScalarErrorReturn,
};
use proof::DeliveryRelation;
use random::{FillBytesErrorReturn, MockIRandomSourceAdapterFailureMode, RandomSourceKind};
use std::env::temp_dir;

use super::interface::RunErrorReturn;
use super::mock::{build_run_deps, build_run_params, build_run_payload};
use super::run;
use crate::benchmark::provides::PairingBenchmarkErrorReturn;
use crate::measure::provides::MeasureErrorReturn;
use crate::report::provides::{ReportDestination, ReportErrorReturn};
use crate::run::provides::{
    ChainFormsConsumerErrorReturn, DeliveryProofConsumerErrorReturn, EncodingConsumerErrorReturn,
    KemConsumerErrorReturn, KeyAgreementConsumerErrorReturn, PairingConsumerErrorReturn,
};
use crate::verifier::provides::{
    MockIVerifierAdapterFailureMode, MockIVerifierAdapterVerifyOnChainErrorReturn,
    VerifierConcrete, VerifyOnChainErrorReturn,
};

/// Contract: Benchmark pairing refused → the refusal `create_pairing`
///   returns is the outcome whole under `CreatePairing`, and no curve runs.
/// Arrange: the default payload with `precompile_encodings` overridden to
///   the real member of `PRECOMPILE_ENCODINGS` alone, an encoding the mock
///   pairing concrete does not declare.
/// Act:     `run`, once.
/// Assert:  the result matches `Err(RunErrorReturn::CreatePairing(
///   CreatePairingErrorReturn::UnsupportedPrecompileEncoding))`.
#[test]
fn run_returns_the_pairing_factorys_refusal_of_an_encoding_the_concrete_does_not_declare() {
    // Arrange
    let mut payload = build_run_payload(Default::default());
    payload.configuration.selections.precompile_encodings = vec![PRECOMPILE_ENCODINGS[0]];

    // Act
    let result = run(
        &build_run_deps(Default::default()),
        build_run_params(Default::default()),
        payload,
    );

    // Assert
    assert!(matches!(
        result,
        Err(RunErrorReturn::CreatePairing(
            CreatePairingErrorReturn::UnsupportedPrecompileEncoding
        ))
    ));
    // run_returns_the_pairing_factorys_refusal_of_an_encoding_the_concrete_does_not_declare
}

/// Contract: Benchmark refused → the refusal the benchmark returns is the
///   outcome whole under `Benchmark`, and no curve runs.
/// Arrange: the default payload with `benchmark_concretes` holding an
///   `AdmittedPairing` whose concrete is `PairingConcrete::Mock(
///   MockIPairingAdapterFailureMode::SampleScalarWrongLength)`.
/// Act:     `run`, once.
/// Assert:  the result matches `Err(RunErrorReturn::Benchmark(
///   PairingBenchmarkErrorReturn::SampleScalar(
///   SampleUniformScalarErrorReturn::WrongLength { expected: 64,
///   actual: 64 })))`.
#[test]
fn run_returns_the_benchmarks_sampling_refusal_whole() {
    // Arrange
    let mut payload = build_run_payload(Default::default());
    payload.configuration.selections.benchmark_concretes[0].concrete =
        PairingConcrete::Mock(MockIPairingAdapterFailureMode::SampleScalarWrongLength);

    // Act
    let result = run(
        &build_run_deps(Default::default()),
        build_run_params(Default::default()),
        payload,
    );

    // Assert
    assert!(matches!(
        result,
        Err(RunErrorReturn::Benchmark(
            PairingBenchmarkErrorReturn::SampleScalar(
                SampleUniformScalarErrorReturn::WrongLength {
                    expected: 64,
                    actual: 64,
                }
            )
        ))
    ));
    // run_returns_the_benchmarks_sampling_refusal_whole
}

/// Contract: Benchmark refused → a refused draw is the outcome whole under
///   `Benchmark`.
/// Arrange: the default payload with `selections.random_source` overridden
///   to `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::
///   FillBytesRefused)`.
/// Act:     `run`, once.
/// Assert:  the result matches `Err(RunErrorReturn::Benchmark(
///   PairingBenchmarkErrorReturn::FillBytes(
///   FillBytesErrorReturn::MockIRandomSourceAdapter)))`.
#[test]
fn run_returns_the_benchmarks_draw_refusal_whole() {
    // Arrange
    let mut payload = build_run_payload(Default::default());
    payload.configuration.selections.random_source =
        RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused);

    // Act
    let result = run(
        &build_run_deps(Default::default()),
        build_run_params(Default::default()),
        payload,
    );

    // Assert
    assert!(matches!(
        result,
        Err(RunErrorReturn::Benchmark(
            PairingBenchmarkErrorReturn::FillBytes(FillBytesErrorReturn::MockIRandomSourceAdapter)
        ))
    ));
    // run_returns_the_benchmarks_draw_refusal_whole
}

/// Contract: Curve consumer refused → the refusal the consumer returns for
///   a curve is the outcome whole under `PairingConsumer`, and the later
///   curve's pairing is not created.
/// Arrange: the default payload with `verifiers` holding
///   `VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure)`
///   at the first index and `VerifierConcrete::Mock(
///   MockIVerifierAdapterFailureMode::RpcRefused)` at the second, and a
///   file report destination.
/// Act:     `run`, once.
/// Assert:  the result matches `Err(RunErrorReturn::PairingConsumer(
///   PairingConsumerErrorReturn::Encoding(EncodingConsumerErrorReturn::
///   ChainForms(ChainFormsConsumerErrorReturn::Kem(
///   KemConsumerErrorReturn::KeyAgreement(KeyAgreementConsumerErrorReturn::
///   DeliveryProof(DeliveryProofConsumerErrorReturn::Measure(
///   MeasureErrorReturn::OnChain { relation: DeliveryRelation::Mint,
///   error: VerifyOnChainErrorReturn::Mock(
///   MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused) }))))))))`.
#[test]
fn run_returns_the_consumers_refusal_for_the_curve_whose_verifier_refuses() {
    // Arrange
    let directory = temp_dir().join(format!(
        "chaintorrent-run-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory).expect("the report directory is created");
    let report_path = directory.join("report.txt");
    let mut payload = build_run_payload(Default::default());
    payload.configuration.output.report_destinations =
        vec![ReportDestination::File { path: report_path }];
    payload.configuration.verifiers = vec![
        VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure),
        VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::RpcRefused),
    ];

    // Act
    let result = run(
        &build_run_deps(Default::default()),
        build_run_params(Default::default()),
        payload,
    );

    // Assert
    assert!(matches!(
        result,
        Err(RunErrorReturn::PairingConsumer(
            PairingConsumerErrorReturn::Encoding(EncodingConsumerErrorReturn::ChainForms(
                ChainFormsConsumerErrorReturn::Kem(KemConsumerErrorReturn::KeyAgreement(
                    KeyAgreementConsumerErrorReturn::DeliveryProof(
                        DeliveryProofConsumerErrorReturn::Measure(MeasureErrorReturn::OnChain {
                            relation: DeliveryRelation::Mint,
                            error: VerifyOnChainErrorReturn::Mock(
                                MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused
                            ),
                        })
                    )
                ))
            ))
        ))
    ));
    // run_returns_the_consumers_refusal_for_the_curve_whose_verifier_refuses
}

/// Contract: Curve reported → the refusal `report` returns is the outcome
///   whole under `Report`, and the next curve is not run.
/// Arrange: the default payload with `report_destinations` holding a file
///   destination whose path lies in a directory that does not exist.
/// Act:     `run`, once.
/// Assert:  the result matches `Err(RunErrorReturn::Report(
///   ReportErrorReturn::WriteFile { path, error }))`, `path` equal to the
///   destination's path and `error.kind()` equal to
///   `std::io::ErrorKind::NotFound`.
#[test]
fn run_returns_the_reports_refusal_whole() {
    // Arrange
    let missing = temp_dir().join(format!(
        "chaintorrent-run-missing-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = std::fs::remove_dir_all(&missing);
    let report_path = missing.join("report.txt");
    let mut payload = build_run_payload(Default::default());
    payload.configuration.output.report_destinations = vec![ReportDestination::File {
        path: report_path.clone(),
    }];

    // Act
    let result = run(
        &build_run_deps(Default::default()),
        build_run_params(Default::default()),
        payload,
    );

    // Assert
    match result {
        Err(RunErrorReturn::Report(ReportErrorReturn::WriteFile { path, error })) => {
            assert_eq!(path, report_path);
            assert_eq!(error.kind(), std::io::ErrorKind::NotFound);
        }
        _ => panic!("the report refusal is the outcome whole under `Report`"),
    }
    // run_returns_the_reports_refusal_whole
}

/// Contract: Ordering → with a benchmark refusal and a curve refusal
///   together the benchmark's is the outcome, so the benchmark entries run
///   before the curves.
/// Arrange: the default payload with the benchmark concrete of the block
///   above and `verifiers` refusing at the first index.
/// Act:     `run`, once.
/// Assert:  the result matches `Err(RunErrorReturn::Benchmark(
///   PairingBenchmarkErrorReturn::SampleScalar(
///   SampleUniformScalarErrorReturn::WrongLength { expected: 64,
///   actual: 64 })))`.
#[test]
fn run_returns_the_benchmark_refusal_before_the_curve_refusal() {
    // Arrange
    let mut payload = build_run_payload(Default::default());
    payload.configuration.selections.benchmark_concretes[0].concrete =
        PairingConcrete::Mock(MockIPairingAdapterFailureMode::SampleScalarWrongLength);
    payload.configuration.verifiers[0] =
        VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::RpcRefused);

    // Act
    let result = run(
        &build_run_deps(Default::default()),
        build_run_params(Default::default()),
        payload,
    );

    // Assert
    assert!(matches!(
        result,
        Err(RunErrorReturn::Benchmark(
            PairingBenchmarkErrorReturn::SampleScalar(
                SampleUniformScalarErrorReturn::WrongLength {
                    expected: 64,
                    actual: 64,
                }
            )
        ))
    ));
    // run_returns_the_benchmark_refusal_before_the_curve_refusal
}
