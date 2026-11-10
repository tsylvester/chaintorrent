#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::num::NonZeroU32;
use pairing::{MockIPairingAdapterFailureMode, PairingConcrete};
use serde_json::{Value, json};
use std::env::temp_dir;
use std::path::PathBuf;

use super::mock::{RunPayloadOverrides, build_run_deps, build_run_params, build_run_payload};
use super::run;
use crate::generate::provides::{EmittedFileOverrides, VectorKind, build_emitted_file};
use crate::report::provides::{
    ReportDefaultsOverrides, ReportDestination, ReportTextConstructorParamsOverrides,
    build_report_defaults, build_report_text,
};

/// Contract: Benchmark measured → the benchmark of the configured concrete
///   runs with the configured count and its measurement is the outcome's
///   benchmark.
/// Arrange: the default payload with `benchmark_iterations` of `3`, a count
///   other than the builder's default.
/// Act:     `run`, once.
/// Assert:  the result is `Ok`; the benchmark's `concrete` equals the
///   configured concrete and its `iterations.get()` equals `3`.
#[test]
fn run_returns_the_measurement_of_the_configured_benchmark_concrete() {
    // Arrange
    let mut payload = build_run_payload(Default::default());
    payload.configuration.benchmark_iterations = NonZeroU32::new(3).unwrap();

    // Act
    let result = run(
        &build_run_deps(Default::default()),
        build_run_params(Default::default()),
        payload,
    );

    // Assert
    let success = result.expect("the run composes the mock concretes");
    assert_eq!(success.benchmarks.len(), 1);
    assert_eq!(
        success.benchmarks[0].concrete,
        PairingConcrete::Mock(MockIPairingAdapterFailureMode::Succeeds)
    );
    assert_eq!(success.benchmarks[0].iterations.get(), 3);
    // run_returns_the_measurement_of_the_configured_benchmark_concrete
}

/// Contract: Curve outcome → each curve's outcome holds that curve's label
///   and the measurement its consumer returned, in curve order.
/// Arrange: the default payload, two curves of distinct labels `"curve-a"`
///   and `"curve-b"`, with `measurement.iterations` of `5`.
/// Act:     `run`, once.
/// Assert:  the outcomes' labels equal `"curve-a"` then `"curve-b"` as
///   written literals, and each outcome's `measurement.iterations.get()`
///   equals `5`.
#[test]
fn run_returns_each_curves_label_and_measurement_in_curve_order() {
    // Arrange
    let mut payload = build_run_payload(Default::default());
    payload.configuration.measurement.iterations = NonZeroU32::new(5).unwrap();

    // Act
    let result = run(
        &build_run_deps(Default::default()),
        build_run_params(Default::default()),
        payload,
    );

    // Assert
    let success = result.expect("the run composes the mock concretes");
    assert_eq!(success.curves.len(), 2);
    assert_eq!(success.curves[0].label.as_str(), "curve-a");
    assert_eq!(success.curves[1].label.as_str(), "curve-b");
    assert_eq!(success.curves[0].measurement.iterations.get(), 5);
    assert_eq!(success.curves[1].measurement.iterations.get(), 5);
    // run_returns_each_curves_label_and_measurement_in_curve_order
}

/// Contract: Curve reported → the report is called with the configured
///   destinations, the curve's label, the configured conditions, and the
///   configured defaults, and its document is the outcome's report.
/// Arrange: the default payload with `output` overridden so
///   `report_destinations` holds one file destination, `measurement`
///   conditions `"conditions-a"`, and `report` defaults of a piece-group
///   size of `16384` and the primary curve `"curve-b"`.
/// Act:     `run`, once.
/// Assert:  the file read back equals the last curve's `report.document`,
///   and the document parsed with `serde_json::from_str::<Value>` holds
///   `curve` `"curve-b"`, `conditions` `"conditions-a"`, and
///   `defaults_in_force` `{ "piece_group_size_bytes": 16384, "primary_curve": "curve-b" }`.
#[test]
fn run_reports_each_curve_to_the_configured_destination_with_its_label() {
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
    payload.configuration.measurement.conditions =
        build_report_text(ReportTextConstructorParamsOverrides {
            text: Some("conditions-a".to_string()),
        });
    payload.configuration.output.report_destinations = vec![ReportDestination::File {
        path: report_path.clone(),
    }];
    payload.configuration.report.defaults = build_report_defaults(ReportDefaultsOverrides {
        piece_group_size: NonZeroU32::new(16_384),
        primary_curve: Some(build_report_text(ReportTextConstructorParamsOverrides {
            text: Some("curve-b".to_string()),
        })),
    });

    // Act
    let result = run(
        &build_run_deps(Default::default()),
        build_run_params(Default::default()),
        payload,
    );

    // Assert
    let success = result.expect("the run composes the mock concretes");
    let written = std::fs::read_to_string(&report_path).expect("the report file is written");
    assert_eq!(written, success.curves[1].report.document);
    let document: Value = serde_json::from_str(&written).expect("the document is JSON");
    assert_eq!(document["curve"], Value::from("curve-b"));
    assert_eq!(document["conditions"], Value::from("conditions-a"));
    assert_eq!(
        document["defaults_in_force"],
        json!({ "piece_group_size_bytes": 16384, "primary_curve": "curve-b" })
    );
    // run_reports_each_curve_to_the_configured_destination_with_its_label
}

/// Contract: Curve outcome → the files in a curve's outcome are the files
///   the consumer returned for that curve, in write order.
/// Arrange: the default payload, whose mock generate concrete returns the
///   file `build_emitted_file` builds for each kind its declaration
///   carries, `kind` the kind and `path` the kind's name as text.
/// Act:     `run`, once.
/// Assert:  each curve's `files` equals the expected files in that order
///   by `assert_eq!`.
#[test]
fn run_returns_the_files_the_curves_consumer_returned() {
    // Arrange
    let payload = build_run_payload(RunPayloadOverrides::default());
    let expected_files = vec![
        build_emitted_file(EmittedFileOverrides {
            kind: Some(Some(VectorKind::G1Addition)),
            path: Some(PathBuf::from("G1Addition")),
        }),
        build_emitted_file(EmittedFileOverrides {
            kind: Some(Some(VectorKind::G1MultiScalarMultiplication)),
            path: Some(PathBuf::from("G1MultiScalarMultiplication")),
        }),
        build_emitted_file(EmittedFileOverrides {
            kind: Some(Some(VectorKind::HashToScalar)),
            path: Some(PathBuf::from("HashToScalar")),
        }),
        build_emitted_file(EmittedFileOverrides {
            kind: Some(Some(VectorKind::DecodeRejection)),
            path: Some(PathBuf::from("DecodeRejection")),
        }),
        build_emitted_file(EmittedFileOverrides {
            kind: Some(Some(VectorKind::IdentityMapping)),
            path: Some(PathBuf::from("IdentityMapping")),
        }),
    ];

    // Act
    let result = run(
        &build_run_deps(Default::default()),
        build_run_params(Default::default()),
        payload,
    );

    // Assert
    let success = result.expect("the run composes the mock concretes");
    assert_eq!(success.curves[0].files, expected_files);
    assert_eq!(success.curves[1].files, expected_files);
    // run_returns_the_files_the_curves_consumer_returned
}
