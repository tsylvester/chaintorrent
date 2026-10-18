#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use kem::IdentityScope;
use pairing::{PairingConcrete, PrecompileEncoding, TargetGroupEncodingIdentifier};

use super::super::layers::provides::{
    BenchmarkDocumentOverrides, CurveDocumentOverrides, DeliveryProofDocumentOverrides,
    HarnessConfigDocumentOverrides, KemDocumentOverrides, NetworkDocumentOverrides,
    build_benchmark_document, build_curve_document, build_delivery_proof_document,
    build_harness_config_document, build_kem_document, build_network_document,
};
use super::admit_selections;
use super::interface::*;
use super::mock::*;

/// Contract: every text of the document is one its vocabulary lists →
///   `Ok(AdmitSelectionsSuccessReturn { selections })`, each field the
///   variant its text names, the curves, the benchmarked concretes, and the
///   precompile encodings in the order held.
/// Arrange: the document with `kem` `build_kem_document` with `scope`
///   `"asset"`, `delivery_proof` `build_delivery_proof_document` with
///   `statement_version` `7`, `curves` holding `build_curve_document` over
///   `"bls12_381_halo2curves"` and `"bls12_381_v1"` then over
///   `"bn254_halo2curves"` and `"bn254_v1"`, `benchmark`
///   `build_benchmark_document` with `concretes` holding
///   `build_curve_document` over `"bls12_381_arkworks"` and `"bls12_381_v1"`,
///   and `network` `build_network_document` with `precompile_encodings`
///   `"eip2537"` then `"eip196_eip197"`, every other part its default.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: `selections` equals `build_harness_selections` with `kem`
///   `build_kem_selection` with `scope` `IdentityScope::Asset`,
///   `delivery_proof` `build_delivery_proof_selection` with
///   `statement_version` `7`, `curves` `build_curve_selection` over
///   `PairingConcrete::Bls12381Halo2curves` and
///   `TargetGroupEncodingIdentifier::Bls12381V1` then over
///   `PairingConcrete::Bn254Halo2curves` and
///   `TargetGroupEncodingIdentifier::Bn254V1`, `benchmark_concretes`
///   `build_curve_selection` over `PairingConcrete::Bls12381Arkworks` and
///   `TargetGroupEncodingIdentifier::Bls12381V1`, and
///   `precompile_encodings`
///   `vec![PrecompileEncoding::Eip2537, PrecompileEncoding::Eip196Eip197]`,
///   every other field its default.
#[test]
fn admit_selections_computes_the_selections_from_every_document_part() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        kem: Some(build_kem_document(KemDocumentOverrides {
            scope: Some("asset".to_string()),
            ..Default::default()
        })),
        delivery_proof: Some(build_delivery_proof_document(
            DeliveryProofDocumentOverrides {
                statement_version: Some(7),
                ..Default::default()
            },
        )),
        curves: Some(vec![
            build_curve_document(CurveDocumentOverrides {
                pairing: Some("bls12_381_halo2curves".to_string()),
                target_group_encoding: Some("bls12_381_v1".to_string()),
            }),
            build_curve_document(CurveDocumentOverrides {
                pairing: Some("bn254_halo2curves".to_string()),
                target_group_encoding: Some("bn254_v1".to_string()),
            }),
        ]),
        benchmark: Some(build_benchmark_document(BenchmarkDocumentOverrides {
            concretes: Some(vec![build_curve_document(CurveDocumentOverrides {
                pairing: Some("bls12_381_arkworks".to_string()),
                target_group_encoding: Some("bls12_381_v1".to_string()),
            })]),
            ..Default::default()
        })),
        network: Some(build_network_document(NetworkDocumentOverrides {
            precompile_encodings: Some(vec!["eip2537".to_string(), "eip196_eip197".to_string()]),
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
    let Ok(AdmitSelectionsSuccessReturn { selections }) = result else {
        panic!("every document part admits");
    };
    assert_eq!(
        selections,
        build_harness_selections(HarnessSelectionsOverrides {
            kem: Some(build_kem_selection(KemSelectionOverrides {
                scope: Some(IdentityScope::Asset),
                ..Default::default()
            })),
            delivery_proof: Some(build_delivery_proof_selection(
                DeliveryProofSelectionOverrides {
                    statement_version: Some(7),
                    ..Default::default()
                },
            )),
            curves: Some(vec![
                build_curve_selection(CurveSelectionOverrides {
                    pairing: Some(PairingConcrete::Bls12381Halo2curves),
                    target_group_encoding: Some(TargetGroupEncodingIdentifier::Bls12381V1),
                }),
                build_curve_selection(CurveSelectionOverrides {
                    pairing: Some(PairingConcrete::Bn254Halo2curves),
                    target_group_encoding: Some(TargetGroupEncodingIdentifier::Bn254V1),
                }),
            ]),
            benchmark_concretes: Some(vec![build_curve_selection(CurveSelectionOverrides {
                pairing: Some(PairingConcrete::Bls12381Arkworks),
                target_group_encoding: Some(TargetGroupEncodingIdentifier::Bls12381V1),
            })]),
            precompile_encodings: Some(vec![
                PrecompileEncoding::Eip2537,
                PrecompileEncoding::Eip196Eip197,
            ]),
            ..Default::default()
        })
    );
}

/// Contract: `curve.pairing` `"bn254_arkworks"`, `"bn254_halo2curves"`,
///   `"bls12_381_arkworks"`, and `"bls12_381_halo2curves"` each →
///   `PairingConcrete::Bn254Arkworks`, `Bn254Halo2curves`,
///   `Bls12381Arkworks`, and `Bls12381Halo2curves`, the curves admitted in
///   order.
/// Arrange: `curves` holding `build_curve_document` with `pairing`
///   `"bls12_381_halo2curves"`, then `"bn254_arkworks"`, then
///   `"bls12_381_arkworks"`, then `"bn254_halo2curves"`, every other field
///   its default.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: `selections.curves` equals, in that order, `build_curve_selection`
///   with `pairing` `PairingConcrete::Bls12381Halo2curves`, `Bn254Arkworks`,
///   `Bls12381Arkworks`, and `Bn254Halo2curves`, every other field its
///   default.
#[test]
fn admit_selections_maps_every_curve_pairing_text_to_its_concrete() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        curves: Some(vec![
            build_curve_document(CurveDocumentOverrides {
                pairing: Some("bls12_381_halo2curves".to_string()),
                ..Default::default()
            }),
            build_curve_document(CurveDocumentOverrides {
                pairing: Some("bn254_arkworks".to_string()),
                ..Default::default()
            }),
            build_curve_document(CurveDocumentOverrides {
                pairing: Some("bls12_381_arkworks".to_string()),
                ..Default::default()
            }),
            build_curve_document(CurveDocumentOverrides {
                pairing: Some("bn254_halo2curves".to_string()),
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
    let Ok(AdmitSelectionsSuccessReturn { selections }) = result else {
        panic!("every curve pairing admits");
    };
    assert_eq!(
        selections.curves,
        vec![
            build_curve_selection(CurveSelectionOverrides {
                pairing: Some(PairingConcrete::Bls12381Halo2curves),
                ..Default::default()
            }),
            build_curve_selection(CurveSelectionOverrides {
                pairing: Some(PairingConcrete::Bn254Arkworks),
                ..Default::default()
            }),
            build_curve_selection(CurveSelectionOverrides {
                pairing: Some(PairingConcrete::Bls12381Arkworks),
                ..Default::default()
            }),
            build_curve_selection(CurveSelectionOverrides {
                pairing: Some(PairingConcrete::Bn254Halo2curves),
                ..Default::default()
            }),
        ]
    );
}

/// Contract: `curve.target_group_encoding` `"bn254_v1"` →
///   `TargetGroupEncodingIdentifier::Bn254V1` and `"bls12_381_v1"` →
///   `Bls12381V1`, the curves admitted in order.
/// Arrange: `curves` holding `build_curve_document` with
///   `target_group_encoding` `"bls12_381_v1"` then `build_curve_document`
///   with `target_group_encoding` `"bn254_v1"`, every other field its
///   default.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: `selections.curves` equals, in that order, `build_curve_selection`
///   with `target_group_encoding` `TargetGroupEncodingIdentifier::Bls12381V1`
///   then `Bn254V1`, every other field its default.
#[test]
fn admit_selections_maps_every_curve_target_group_encoding_text_to_its_identifier() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        curves: Some(vec![
            build_curve_document(CurveDocumentOverrides {
                target_group_encoding: Some("bls12_381_v1".to_string()),
                ..Default::default()
            }),
            build_curve_document(CurveDocumentOverrides {
                target_group_encoding: Some("bn254_v1".to_string()),
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
    let Ok(AdmitSelectionsSuccessReturn { selections }) = result else {
        panic!("every curve target-group encoding admits");
    };
    assert_eq!(
        selections.curves,
        vec![
            build_curve_selection(CurveSelectionOverrides {
                target_group_encoding: Some(TargetGroupEncodingIdentifier::Bls12381V1),
                ..Default::default()
            }),
            build_curve_selection(CurveSelectionOverrides {
                target_group_encoding: Some(TargetGroupEncodingIdentifier::Bn254V1),
                ..Default::default()
            }),
        ]
    );
}

/// Contract: `concrete.pairing` of each benchmarked concrete maps over the
///   vocabulary the curves use, the concretes admitted in order.
/// Arrange: `benchmark` `build_benchmark_document` with `concretes` holding
///   `build_curve_document` with `pairing` `"bn254_halo2curves"`, then
///   `"bls12_381_arkworks"`, then `"bn254_arkworks"`, then
///   `"bls12_381_halo2curves"`, every other field its default.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: `selections.benchmark_concretes` equals, in that order,
///   `build_curve_selection` with `pairing`
///   `PairingConcrete::Bn254Halo2curves`, `Bls12381Arkworks`,
///   `Bn254Arkworks`, and `Bls12381Halo2curves`, every other field its
///   default.
#[test]
fn admit_selections_maps_every_benchmarked_pairing_text_to_its_concrete() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        benchmark: Some(build_benchmark_document(BenchmarkDocumentOverrides {
            concretes: Some(vec![
                build_curve_document(CurveDocumentOverrides {
                    pairing: Some("bn254_halo2curves".to_string()),
                    ..Default::default()
                }),
                build_curve_document(CurveDocumentOverrides {
                    pairing: Some("bls12_381_arkworks".to_string()),
                    ..Default::default()
                }),
                build_curve_document(CurveDocumentOverrides {
                    pairing: Some("bn254_arkworks".to_string()),
                    ..Default::default()
                }),
                build_curve_document(CurveDocumentOverrides {
                    pairing: Some("bls12_381_halo2curves".to_string()),
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
    let Ok(AdmitSelectionsSuccessReturn { selections }) = result else {
        panic!("every benchmarked pairing admits");
    };
    assert_eq!(
        selections.benchmark_concretes,
        vec![
            build_curve_selection(CurveSelectionOverrides {
                pairing: Some(PairingConcrete::Bn254Halo2curves),
                ..Default::default()
            }),
            build_curve_selection(CurveSelectionOverrides {
                pairing: Some(PairingConcrete::Bls12381Arkworks),
                ..Default::default()
            }),
            build_curve_selection(CurveSelectionOverrides {
                pairing: Some(PairingConcrete::Bn254Arkworks),
                ..Default::default()
            }),
            build_curve_selection(CurveSelectionOverrides {
                pairing: Some(PairingConcrete::Bls12381Halo2curves),
                ..Default::default()
            }),
        ]
    );
}

/// Contract: `concrete.target_group_encoding` of each benchmarked concrete
///   maps over the vocabulary the curves use, the concretes admitted in
///   order.
/// Arrange: `benchmark` `build_benchmark_document` with `concretes` holding
///   `build_curve_document` with `target_group_encoding` `"bls12_381_v1"`
///   then `build_curve_document` with `target_group_encoding` `"bn254_v1"`,
///   every other field its default.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: `selections.benchmark_concretes` equals, in that order,
///   `build_curve_selection` with `target_group_encoding`
///   `TargetGroupEncodingIdentifier::Bls12381V1` then `Bn254V1`, every other
///   field its default.
#[test]
fn admit_selections_maps_every_benchmarked_target_group_encoding_text_to_its_identifier() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        benchmark: Some(build_benchmark_document(BenchmarkDocumentOverrides {
            concretes: Some(vec![
                build_curve_document(CurveDocumentOverrides {
                    target_group_encoding: Some("bls12_381_v1".to_string()),
                    ..Default::default()
                }),
                build_curve_document(CurveDocumentOverrides {
                    target_group_encoding: Some("bn254_v1".to_string()),
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
    let Ok(AdmitSelectionsSuccessReturn { selections }) = result else {
        panic!("every benchmarked target-group encoding admits");
    };
    assert_eq!(
        selections.benchmark_concretes,
        vec![
            build_curve_selection(CurveSelectionOverrides {
                target_group_encoding: Some(TargetGroupEncodingIdentifier::Bls12381V1),
                ..Default::default()
            }),
            build_curve_selection(CurveSelectionOverrides {
                target_group_encoding: Some(TargetGroupEncodingIdentifier::Bn254V1),
                ..Default::default()
            }),
        ]
    );
}

/// Contract: an empty `payload.benchmark_concretes` → admitted as an empty
///   `benchmark_concretes`.
/// Arrange: `benchmark` `build_benchmark_document` with `concretes`
///   `Vec::new()`, the default holding one concrete.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: `selections.benchmark_concretes` is empty.
#[test]
fn admit_selections_admits_an_empty_benchmark_list() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        benchmark: Some(build_benchmark_document(BenchmarkDocumentOverrides {
            concretes: Some(Vec::new()),
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
    let Ok(AdmitSelectionsSuccessReturn { selections }) = result else {
        panic!("an empty benchmark list admits");
    };
    assert!(selections.benchmark_concretes.is_empty());
}

/// Contract: `kem.scope` `"asset"` → `IdentityScope::Asset`.
/// Arrange: `kem` `build_kem_document` with `scope` `"asset"`, the default
///   `"entitlement"`.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: `selections.kem` equals `build_kem_selection` with `scope`
///   `IdentityScope::Asset`.
#[test]
fn admit_selections_maps_the_asset_scope() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        kem: Some(build_kem_document(KemDocumentOverrides {
            scope: Some("asset".to_string()),
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
    let Ok(AdmitSelectionsSuccessReturn { selections }) = result else {
        panic!("the asset scope admits");
    };
    assert_eq!(
        selections.kem,
        build_kem_selection(KemSelectionOverrides {
            scope: Some(IdentityScope::Asset),
            ..Default::default()
        })
    );
}

/// Contract: `"eip196_eip197"` → `PrecompileEncoding::Eip196Eip197` and
///   `"eip2537"` → `PrecompileEncoding::Eip2537`, in order.
/// Arrange: `network` `build_network_document` with `precompile_encodings`
///   `"eip2537"` then `"eip196_eip197"`.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: `selections.precompile_encodings` equals
///   `vec![PrecompileEncoding::Eip2537, PrecompileEncoding::Eip196Eip197]`.
#[test]
fn admit_selections_maps_the_precompile_encodings_in_order() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        network: Some(build_network_document(NetworkDocumentOverrides {
            precompile_encodings: Some(vec!["eip2537".to_string(), "eip196_eip197".to_string()]),
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
    let Ok(AdmitSelectionsSuccessReturn { selections }) = result else {
        panic!("the precompile encodings admit");
    };
    assert_eq!(
        selections.precompile_encodings,
        vec![
            PrecompileEncoding::Eip2537,
            PrecompileEncoding::Eip196Eip197,
        ]
    );
}

/// Contract: `delivery_proof.statement_version` carried as it is.
/// Arrange: `delivery_proof` `build_delivery_proof_document` with
///   `statement_version` `7`, the default `1`.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: `selections.delivery_proof` equals `build_delivery_proof_selection`
///   with `statement_version` `7`.
#[test]
fn admit_selections_carries_the_delivery_statement_version_unchanged() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        delivery_proof: Some(build_delivery_proof_document(
            DeliveryProofDocumentOverrides {
                statement_version: Some(7),
                ..Default::default()
            },
        )),
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
    let Ok(AdmitSelectionsSuccessReturn { selections }) = result else {
        panic!("the delivery-statement version admits");
    };
    assert_eq!(
        selections.delivery_proof,
        build_delivery_proof_selection(DeliveryProofSelectionOverrides {
            statement_version: Some(7),
            ..Default::default()
        })
    );
}
