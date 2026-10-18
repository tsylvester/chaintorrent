#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::super::layers::provides::{
    BenchmarkDocumentOverrides, CurveDocumentOverrides, HarnessConfigDocumentOverrides,
    NetworkDocumentOverrides, build_benchmark_document, build_curve_document,
    build_harness_config_document, build_network_document,
};
use super::admit_selections;
use super::interface::*;
use super::mock::*;

/// Contract: `curve.pairing` outside its vocabulary → `UnknownValue` with
///   field `CurvePairing { index }`.
/// Arrange: `curves` holding `build_curve_document(Default::default())` then
///   `build_curve_document` with `pairing` `"bn254"`, the refused element at
///   index `1`.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitSelectionsErrorReturn::UnknownValue {
///   field: SelectionField::CurvePairing { index: 1 },
///   text: "bn254".to_string() })`.
#[test]
fn admit_selections_refuses_a_curve_pairing_text_with_its_index() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        curves: Some(vec![
            build_curve_document(Default::default()),
            build_curve_document(CurveDocumentOverrides {
                pairing: Some("bn254".to_string()),
                ..Default::default()
            }),
        ]),
        ..Default::default()
    });

    // Act
    let result = admit_selections(
        &build_admit_selections_deps(Default::default()),
        build_admit_selections_params(Default::default()),
        build_admit_selections_payload(
            &document.random,
            &document.encoding,
            &document.hash_to_scalar,
            &document.chain_forms,
            &document.kem,
            &document.key_agreement,
            &document.delivery_proof,
            &document.curves,
            &document.benchmark.concretes,
            &document.network.precompile_encodings,
            Default::default(),
        ),
    );

    // Assert
    assert_eq!(
        result,
        Err(AdmitSelectionsErrorReturn::UnknownValue {
            field: SelectionField::CurvePairing { index: 1 },
            text: "bn254".to_string(),
        })
    );
}

/// Contract: `curve.target_group_encoding` outside its vocabulary →
///   `UnknownValue` with field `CurveTargetGroupEncoding { index }`.
/// Arrange: `curves` holding `build_curve_document(Default::default())` then
///   `build_curve_document` with `target_group_encoding` `"bn254"`, the
///   refused element at index `1`.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitSelectionsErrorReturn::UnknownValue {
///   field: SelectionField::CurveTargetGroupEncoding { index: 1 },
///   text: "bn254".to_string() })`.
#[test]
fn admit_selections_refuses_a_curve_target_group_encoding_text_with_its_index() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        curves: Some(vec![
            build_curve_document(Default::default()),
            build_curve_document(CurveDocumentOverrides {
                target_group_encoding: Some("bn254".to_string()),
                ..Default::default()
            }),
        ]),
        ..Default::default()
    });

    // Act
    let result = admit_selections(
        &build_admit_selections_deps(Default::default()),
        build_admit_selections_params(Default::default()),
        build_admit_selections_payload(
            &document.random,
            &document.encoding,
            &document.hash_to_scalar,
            &document.chain_forms,
            &document.kem,
            &document.key_agreement,
            &document.delivery_proof,
            &document.curves,
            &document.benchmark.concretes,
            &document.network.precompile_encodings,
            Default::default(),
        ),
    );

    // Assert
    assert_eq!(
        result,
        Err(AdmitSelectionsErrorReturn::UnknownValue {
            field: SelectionField::CurveTargetGroupEncoding { index: 1 },
            text: "bn254".to_string(),
        })
    );
}

/// Contract: `payload.curves` is empty → `Err(AdmitSelectionsErrorReturn::
///   NoCurves)`.
/// Arrange: `curves` `Vec::new()`, the default holding one curve.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitSelectionsErrorReturn::NoCurves)`.
#[test]
fn admit_selections_refuses_an_empty_curve_list() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        curves: Some(Vec::new()),
        ..Default::default()
    });

    // Act
    let result = admit_selections(
        &build_admit_selections_deps(Default::default()),
        build_admit_selections_params(Default::default()),
        build_admit_selections_payload(
            &document.random,
            &document.encoding,
            &document.hash_to_scalar,
            &document.chain_forms,
            &document.kem,
            &document.key_agreement,
            &document.delivery_proof,
            &document.curves,
            &document.benchmark.concretes,
            &document.network.precompile_encodings,
            Default::default(),
        ),
    );

    // Assert
    assert_eq!(result, Err(AdmitSelectionsErrorReturn::NoCurves));
}

/// Contract: `concrete.pairing` outside its vocabulary → `UnknownValue` with
///   field `BenchmarkPairing { index }`.
/// Arrange: `benchmark` `build_benchmark_document` with `concretes` holding
///   `build_curve_document(Default::default())` then `build_curve_document`
///   with `pairing` `"bn254"`, the refused element at index `1`.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitSelectionsErrorReturn::UnknownValue {
///   field: SelectionField::BenchmarkPairing { index: 1 },
///   text: "bn254".to_string() })`.
#[test]
fn admit_selections_refuses_a_benchmarked_pairing_text_with_its_index() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        benchmark: Some(build_benchmark_document(BenchmarkDocumentOverrides {
            concretes: Some(vec![
                build_curve_document(Default::default()),
                build_curve_document(CurveDocumentOverrides {
                    pairing: Some("bn254".to_string()),
                    ..Default::default()
                }),
            ]),
            ..Default::default()
        })),
        ..Default::default()
    });

    // Act
    let result = admit_selections(
        &build_admit_selections_deps(Default::default()),
        build_admit_selections_params(Default::default()),
        build_admit_selections_payload(
            &document.random,
            &document.encoding,
            &document.hash_to_scalar,
            &document.chain_forms,
            &document.kem,
            &document.key_agreement,
            &document.delivery_proof,
            &document.curves,
            &document.benchmark.concretes,
            &document.network.precompile_encodings,
            Default::default(),
        ),
    );

    // Assert
    assert_eq!(
        result,
        Err(AdmitSelectionsErrorReturn::UnknownValue {
            field: SelectionField::BenchmarkPairing { index: 1 },
            text: "bn254".to_string(),
        })
    );
}

/// Contract: `concrete.target_group_encoding` outside its vocabulary →
///   `UnknownValue` with field `BenchmarkTargetGroupEncoding { index }`.
/// Arrange: `benchmark` `build_benchmark_document` with `concretes` holding
///   `build_curve_document(Default::default())` then `build_curve_document`
///   with `target_group_encoding` `"bn254"`, the refused element at
///   index `1`.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitSelectionsErrorReturn::UnknownValue {
///   field: SelectionField::BenchmarkTargetGroupEncoding { index: 1 },
///   text: "bn254".to_string() })`.
#[test]
fn admit_selections_refuses_a_benchmarked_target_group_encoding_text_with_its_index() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        benchmark: Some(build_benchmark_document(BenchmarkDocumentOverrides {
            concretes: Some(vec![
                build_curve_document(Default::default()),
                build_curve_document(CurveDocumentOverrides {
                    target_group_encoding: Some("bn254".to_string()),
                    ..Default::default()
                }),
            ]),
            ..Default::default()
        })),
        ..Default::default()
    });

    // Act
    let result = admit_selections(
        &build_admit_selections_deps(Default::default()),
        build_admit_selections_params(Default::default()),
        build_admit_selections_payload(
            &document.random,
            &document.encoding,
            &document.hash_to_scalar,
            &document.chain_forms,
            &document.kem,
            &document.key_agreement,
            &document.delivery_proof,
            &document.curves,
            &document.benchmark.concretes,
            &document.network.precompile_encodings,
            Default::default(),
        ),
    );

    // Assert
    assert_eq!(
        result,
        Err(AdmitSelectionsErrorReturn::UnknownValue {
            field: SelectionField::BenchmarkTargetGroupEncoding { index: 1 },
            text: "bn254".to_string(),
        })
    );
}

/// Contract: a precompile encoding text outside its vocabulary →
///   `UnknownValue` with field `PrecompileEncoding { index }`.
/// Arrange: `network` `build_network_document` with `precompile_encodings`
///   `"eip196_eip197"` then `"eip-2537"`, the refused element at index `1`.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitSelectionsErrorReturn::UnknownValue {
///   field: SelectionField::PrecompileEncoding { index: 1 },
///   text: "eip-2537".to_string() })`.
#[test]
fn admit_selections_refuses_a_precompile_encoding_with_its_index() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        network: Some(build_network_document(NetworkDocumentOverrides {
            precompile_encodings: Some(vec!["eip196_eip197".to_string(), "eip-2537".to_string()]),
            ..Default::default()
        })),
        ..Default::default()
    });

    // Act
    let result = admit_selections(
        &build_admit_selections_deps(Default::default()),
        build_admit_selections_params(Default::default()),
        build_admit_selections_payload(
            &document.random,
            &document.encoding,
            &document.hash_to_scalar,
            &document.chain_forms,
            &document.kem,
            &document.key_agreement,
            &document.delivery_proof,
            &document.curves,
            &document.benchmark.concretes,
            &document.network.precompile_encodings,
            Default::default(),
        ),
    );

    // Assert
    assert_eq!(
        result,
        Err(AdmitSelectionsErrorReturn::UnknownValue {
            field: SelectionField::PrecompileEncoding { index: 1 },
            text: "eip-2537".to_string(),
        })
    );
}
