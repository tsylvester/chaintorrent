#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::super::layers::provides::{
    BenchmarkDocumentOverrides, CurveDocumentOverrides, DeliveryProofDocumentOverrides,
    HarnessConfigDocumentOverrides, NetworkDocumentOverrides, build_benchmark_document,
    build_curve_document, build_delivery_proof_document, build_harness_config_document,
    build_network_document,
};
use super::admit_selections;
use super::interface::*;
use super::mock::*;

/// Contract: `delivery_proof.concrete` outside its vocabulary and
///   `payload.curves` empty → `UnknownValue` with field
///   `DeliveryProofConcrete`, the family fields admitted before the
///   empty-curve check and the first refusal returning.
/// Arrange: `delivery_proof` `build_delivery_proof_document` with `concrete`
///   `"ed25519"` and `curves` `Vec::new()`.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitSelectionsErrorReturn::UnknownValue {
///   field: SelectionField::DeliveryProofConcrete,
///   text: "ed25519".to_string() })`.
#[test]
fn admit_selections_refuses_the_delivery_proof_concrete_before_an_empty_curve_list() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        delivery_proof: Some(build_delivery_proof_document(
            DeliveryProofDocumentOverrides {
                concrete: Some("ed25519".to_string()),
                ..Default::default()
            },
        )),
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
    assert_eq!(
        result,
        Err(AdmitSelectionsErrorReturn::UnknownValue {
            field: SelectionField::DeliveryProofConcrete,
            text: "ed25519".to_string(),
        })
    );
}

/// Contract: `payload.curves` empty and a benchmarked `concrete.pairing`
///   outside its vocabulary → `Err(AdmitSelectionsErrorReturn::NoCurves)`,
///   the empty-curve check before the benchmarked concretes.
/// Arrange: `curves` `Vec::new()` and `benchmark` `build_benchmark_document`
///   with `concretes` holding `build_curve_document` with `pairing`
///   `"bn254"`.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitSelectionsErrorReturn::NoCurves)`.
#[test]
fn admit_selections_refuses_an_empty_curve_list_before_a_benchmarked_pairing() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        curves: Some(Vec::new()),
        benchmark: Some(build_benchmark_document(BenchmarkDocumentOverrides {
            concretes: Some(vec![build_curve_document(CurveDocumentOverrides {
                pairing: Some("bn254".to_string()),
                ..Default::default()
            })]),
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
    assert_eq!(result, Err(AdmitSelectionsErrorReturn::NoCurves));
}

/// Contract: `curve.pairing` and `curve.target_group_encoding` of one curve
///   both outside their vocabularies → `UnknownValue` with field
///   `CurvePairing { index }`, each curve's pairing before its target-group
///   encoding.
/// Arrange: `curves` holding one `build_curve_document` with `pairing`
///   `"bn254"` and `target_group_encoding` `"bn254"`.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitSelectionsErrorReturn::UnknownValue {
///   field: SelectionField::CurvePairing { index: 0 },
///   text: "bn254".to_string() })`.
#[test]
fn admit_selections_refuses_a_curve_pairing_before_its_target_group_encoding() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        curves: Some(vec![build_curve_document(CurveDocumentOverrides {
            pairing: Some("bn254".to_string()),
            target_group_encoding: Some("bn254".to_string()),
        })]),
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
            field: SelectionField::CurvePairing { index: 0 },
            text: "bn254".to_string(),
        })
    );
}

/// Contract: `curve.target_group_encoding` of the curve at index `0` and
///   `curve.pairing` of the curve at index `1` both outside their
///   vocabularies → `UnknownValue` with field
///   `CurveTargetGroupEncoding { index: 0 }`, each curve admitted whole
///   before the next.
/// Arrange: `curves` holding `build_curve_document` with
///   `target_group_encoding` `"bn254"` then `build_curve_document` with
///   `pairing` `"bn254"`.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitSelectionsErrorReturn::UnknownValue {
///   field: SelectionField::CurveTargetGroupEncoding { index: 0 },
///   text: "bn254".to_string() })`.
#[test]
fn admit_selections_refuses_an_earlier_curve_before_a_later_curve() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        curves: Some(vec![
            build_curve_document(CurveDocumentOverrides {
                target_group_encoding: Some("bn254".to_string()),
                ..Default::default()
            }),
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
            field: SelectionField::CurveTargetGroupEncoding { index: 0 },
            text: "bn254".to_string(),
        })
    );
}

/// Contract: `curve.target_group_encoding` of the curve at index `1` and
///   `concrete.pairing` of the benchmarked concrete at index `0` both
///   outside their vocabularies → `UnknownValue` with field
///   `CurveTargetGroupEncoding { index: 1 }`, the curves admitted before the
///   benchmarked concretes.
/// Arrange: `curves` holding `build_curve_document(Default::default())` then
///   `build_curve_document` with `target_group_encoding` `"bn254"`, and
///   `benchmark` `build_benchmark_document` with `concretes` holding
///   `build_curve_document` with `pairing` `"bn254"`.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitSelectionsErrorReturn::UnknownValue {
///   field: SelectionField::CurveTargetGroupEncoding { index: 1 },
///   text: "bn254".to_string() })`.
#[test]
fn admit_selections_refuses_the_last_curve_before_a_benchmarked_pairing() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        curves: Some(vec![
            build_curve_document(Default::default()),
            build_curve_document(CurveDocumentOverrides {
                target_group_encoding: Some("bn254".to_string()),
                ..Default::default()
            }),
        ]),
        benchmark: Some(build_benchmark_document(BenchmarkDocumentOverrides {
            concretes: Some(vec![build_curve_document(CurveDocumentOverrides {
                pairing: Some("bn254".to_string()),
                ..Default::default()
            })]),
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
            field: SelectionField::CurveTargetGroupEncoding { index: 1 },
            text: "bn254".to_string(),
        })
    );
}

/// Contract: `concrete.pairing` and `concrete.target_group_encoding` of one
///   benchmarked concrete both outside their vocabularies → `UnknownValue`
///   with field `BenchmarkPairing { index }`, each concrete's pairing before
///   its target-group encoding.
/// Arrange: `benchmark` `build_benchmark_document` with `concretes` holding
///   one `build_curve_document` with `pairing` `"bn254"` and
///   `target_group_encoding` `"bn254"`.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitSelectionsErrorReturn::UnknownValue {
///   field: SelectionField::BenchmarkPairing { index: 0 },
///   text: "bn254".to_string() })`.
#[test]
fn admit_selections_refuses_a_benchmarked_pairing_before_its_target_group_encoding() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        benchmark: Some(build_benchmark_document(BenchmarkDocumentOverrides {
            concretes: Some(vec![build_curve_document(CurveDocumentOverrides {
                pairing: Some("bn254".to_string()),
                target_group_encoding: Some("bn254".to_string()),
            })]),
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
            field: SelectionField::BenchmarkPairing { index: 0 },
            text: "bn254".to_string(),
        })
    );
}

/// Contract: `concrete.target_group_encoding` of the benchmarked concrete at
///   index `0` and `concrete.pairing` of the one at index `1` both outside
///   their vocabularies → `UnknownValue` with field
///   `BenchmarkTargetGroupEncoding { index: 0 }`, each concrete admitted
///   whole before the next.
/// Arrange: `benchmark` `build_benchmark_document` with `concretes` holding
///   `build_curve_document` with `target_group_encoding` `"bn254"` then
///   `build_curve_document` with `pairing` `"bn254"`.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitSelectionsErrorReturn::UnknownValue {
///   field: SelectionField::BenchmarkTargetGroupEncoding { index: 0 },
///   text: "bn254".to_string() })`.
#[test]
fn admit_selections_refuses_an_earlier_benchmarked_concrete_before_a_later_one() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        benchmark: Some(build_benchmark_document(BenchmarkDocumentOverrides {
            concretes: Some(vec![
                build_curve_document(CurveDocumentOverrides {
                    target_group_encoding: Some("bn254".to_string()),
                    ..Default::default()
                }),
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
            field: SelectionField::BenchmarkTargetGroupEncoding { index: 0 },
            text: "bn254".to_string(),
        })
    );
}

/// Contract: `concrete.target_group_encoding` of the benchmarked concrete at
///   index `1` and the precompile encoding at index `0` both outside their
///   vocabularies → `UnknownValue` with field
///   `BenchmarkTargetGroupEncoding { index: 1 }`, the benchmarked concretes
///   admitted before the precompile encodings.
/// Arrange: `benchmark` `build_benchmark_document` with `concretes` holding
///   `build_curve_document(Default::default())` then `build_curve_document`
///   with `target_group_encoding` `"bn254"`, and `network`
///   `build_network_document` with `precompile_encodings` `"eip-2537"`.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitSelectionsErrorReturn::UnknownValue {
///   field: SelectionField::BenchmarkTargetGroupEncoding { index: 1 },
///   text: "bn254".to_string() })`.
#[test]
fn admit_selections_refuses_the_last_benchmarked_concrete_before_a_precompile_encoding() {
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
        network: Some(build_network_document(NetworkDocumentOverrides {
            precompile_encodings: Some(vec!["eip-2537".to_string()]),
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

/// Contract: the precompile encodings at index `0` and index `1` both
///   outside their vocabulary → `UnknownValue` with field
///   `PrecompileEncoding { index: 0 }`, the encodings admitted in index
///   order.
/// Arrange: `network` `build_network_document` with `precompile_encodings`
///   `"eip-2537"` then `"eip_2537"`.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitSelectionsErrorReturn::UnknownValue {
///   field: SelectionField::PrecompileEncoding { index: 0 },
///   text: "eip-2537".to_string() })`.
#[test]
fn admit_selections_refuses_an_earlier_precompile_encoding_before_a_later_one() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        network: Some(build_network_document(NetworkDocumentOverrides {
            precompile_encodings: Some(vec!["eip-2537".to_string(), "eip_2537".to_string()]),
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
            field: SelectionField::PrecompileEncoding { index: 0 },
            text: "eip-2537".to_string(),
        })
    );
}
