#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::super::layers::provides::{
    DeliveryProofDocumentOverrides, HarnessConfigDocumentOverrides, KemDocumentOverrides,
    KeyAgreementDocumentOverrides, RandomDocumentOverrides, SelectionDocumentOverrides,
    build_delivery_proof_document, build_harness_config_document, build_kem_document,
    build_key_agreement_document, build_random_document, build_selection_document,
};
use super::admit_selections;
use super::interface::*;
use super::mock::*;

/// Contract: `random.source` outside its vocabulary →
///   `Err(AdmitSelectionsErrorReturn::UnknownValue { field, text })`, every
///   other field holding its family's vocabulary text.
/// Arrange: `random` `build_random_document` with `source` `"system"`.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitSelectionsErrorReturn::UnknownValue {
///   field: SelectionField::RandomSource, text: "system".to_string() })`.
#[test]
fn admit_selections_refuses_an_unknown_random_source() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        random: Some(build_random_document(RandomDocumentOverrides {
            source: Some("system".to_string()),
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
            field: SelectionField::RandomSource,
            text: "system".to_string(),
        })
    );
}

/// Contract: `encoding.concrete` outside its vocabulary → `UnknownValue`
///   with field `EncodingConcrete`, every other field holding its family's
///   vocabulary text.
/// Arrange: `encoding` `build_selection_document` with `concrete` `"json"`.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitSelectionsErrorReturn::UnknownValue {
///   field: SelectionField::EncodingConcrete, text: "json".to_string() })`.
#[test]
fn admit_selections_refuses_an_unknown_encoding_concrete() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        encoding: Some(build_selection_document(SelectionDocumentOverrides {
            concrete: Some("json".to_string()),
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
            field: SelectionField::EncodingConcrete,
            text: "json".to_string(),
        })
    );
}

/// Contract: `encoding.identifier` outside its vocabulary → `UnknownValue`
///   with field `EncodingIdentifier`, every other field holding its family's
///   vocabulary text.
/// Arrange: `encoding` `build_selection_document` with `identifier`
///   `"json_v1"`.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitSelectionsErrorReturn::UnknownValue {
///   field: SelectionField::EncodingIdentifier, text: "json_v1".to_string()
///   })`.
#[test]
fn admit_selections_refuses_an_unknown_encoding_identifier() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        encoding: Some(build_selection_document(SelectionDocumentOverrides {
            identifier: Some("json_v1".to_string()),
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
            field: SelectionField::EncodingIdentifier,
            text: "json_v1".to_string(),
        })
    );
}

/// Contract: `hash_to_scalar.concrete` outside its vocabulary →
///   `UnknownValue` with field `HashToScalarConcrete`, every other field
///   holding its family's vocabulary text.
/// Arrange: `hash_to_scalar` `build_selection_document` with `concrete`
///   `"sha256"` and `identifier` `"keccak256_v1"`.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitSelectionsErrorReturn::UnknownValue {
///   field: SelectionField::HashToScalarConcrete, text: "sha256".to_string()
///   })`.
#[test]
fn admit_selections_refuses_an_unknown_hash_to_scalar_concrete() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        hash_to_scalar: Some(build_selection_document(SelectionDocumentOverrides {
            concrete: Some("sha256".to_string()),
            identifier: Some("keccak256_v1".to_string()),
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
            field: SelectionField::HashToScalarConcrete,
            text: "sha256".to_string(),
        })
    );
}

/// Contract: `hash_to_scalar.identifier` outside its vocabulary →
///   `UnknownValue` with field `HashToScalarIdentifier`, every other field
///   holding its family's vocabulary text.
/// Arrange: `hash_to_scalar` `build_selection_document` with `concrete`
///   `"keccak256"` and `identifier` `"sha256_v1"`.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitSelectionsErrorReturn::UnknownValue {
///   field: SelectionField::HashToScalarIdentifier,
///   text: "sha256_v1".to_string() })`.
#[test]
fn admit_selections_refuses_an_unknown_hash_to_scalar_identifier() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        hash_to_scalar: Some(build_selection_document(SelectionDocumentOverrides {
            concrete: Some("keccak256".to_string()),
            identifier: Some("sha256_v1".to_string()),
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
            field: SelectionField::HashToScalarIdentifier,
            text: "sha256_v1".to_string(),
        })
    );
}

/// Contract: `chain_forms.concrete` outside its vocabulary → `UnknownValue`
///   with field `ChainFormsConcrete`, every other field holding its family's
///   vocabulary text.
/// Arrange: `chain_forms` `build_selection_document` with `concrete`
///   `"solana"` and `identifier` `"evm_v1"`.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitSelectionsErrorReturn::UnknownValue {
///   field: SelectionField::ChainFormsConcrete, text: "solana".to_string()
///   })`.
#[test]
fn admit_selections_refuses_an_unknown_chain_forms_concrete() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        chain_forms: Some(build_selection_document(SelectionDocumentOverrides {
            concrete: Some("solana".to_string()),
            identifier: Some("evm_v1".to_string()),
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
            field: SelectionField::ChainFormsConcrete,
            text: "solana".to_string(),
        })
    );
}

/// Contract: `chain_forms.identifier` outside its vocabulary → `UnknownValue`
///   with field `ChainFormsIdentifier`, every other field holding its
///   family's vocabulary text.
/// Arrange: `chain_forms` `build_selection_document` with `concrete` `"evm"`
///   and `identifier` `"solana_v1"`.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitSelectionsErrorReturn::UnknownValue {
///   field: SelectionField::ChainFormsIdentifier,
///   text: "solana_v1".to_string() })`.
#[test]
fn admit_selections_refuses_an_unknown_chain_forms_identifier() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        chain_forms: Some(build_selection_document(SelectionDocumentOverrides {
            concrete: Some("evm".to_string()),
            identifier: Some("solana_v1".to_string()),
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
            field: SelectionField::ChainFormsIdentifier,
            text: "solana_v1".to_string(),
        })
    );
}

/// Contract: `kem.concrete` outside its vocabulary → `UnknownValue` with
///   field `KemConcrete`, every other field holding its family's vocabulary
///   text.
/// Arrange: `kem` `build_kem_document` with `concrete` `"bb2_depth_two"`.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitSelectionsErrorReturn::UnknownValue {
///   field: SelectionField::KemConcrete, text: "bb2_depth_two".to_string()
///   })`.
#[test]
fn admit_selections_refuses_an_unknown_kem_concrete() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        kem: Some(build_kem_document(KemDocumentOverrides {
            concrete: Some("bb2_depth_two".to_string()),
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
            field: SelectionField::KemConcrete,
            text: "bb2_depth_two".to_string(),
        })
    );
}

/// Contract: `kem.identifier` outside its vocabulary → `UnknownValue` with
///   field `KemIdentifier`, every other field holding its family's vocabulary
///   text.
/// Arrange: `kem` `build_kem_document` with `identifier`
///   `"bb2_depth_two_v1"`.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitSelectionsErrorReturn::UnknownValue {
///   field: SelectionField::KemIdentifier,
///   text: "bb2_depth_two_v1".to_string() })`.
#[test]
fn admit_selections_refuses_an_unknown_kem_identifier() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        kem: Some(build_kem_document(KemDocumentOverrides {
            identifier: Some("bb2_depth_two_v1".to_string()),
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
            field: SelectionField::KemIdentifier,
            text: "bb2_depth_two_v1".to_string(),
        })
    );
}

/// Contract: `kem.scope` outside its vocabulary → `UnknownValue` with field
///   `KemScope`, every other field holding its family's vocabulary text.
/// Arrange: `kem` `build_kem_document` with `scope` `"holder"`.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitSelectionsErrorReturn::UnknownValue {
///   field: SelectionField::KemScope, text: "holder".to_string() })`.
#[test]
fn admit_selections_refuses_a_scope_it_does_not_hold() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        kem: Some(build_kem_document(KemDocumentOverrides {
            scope: Some("holder".to_string()),
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
            field: SelectionField::KemScope,
            text: "holder".to_string(),
        })
    );
}

/// Contract: `key_agreement.concrete` outside its vocabulary → `UnknownValue`
///   with field `KeyAgreementConcrete`, every other field holding its
///   family's vocabulary text.
/// Arrange: `key_agreement` `build_key_agreement_document` with `concrete`
///   `"elgamal"`.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitSelectionsErrorReturn::UnknownValue {
///   field: SelectionField::KeyAgreementConcrete,
///   text: "elgamal".to_string() })`.
#[test]
fn admit_selections_refuses_an_unknown_key_agreement_concrete() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        key_agreement: Some(build_key_agreement_document(
            KeyAgreementDocumentOverrides {
                concrete: Some("elgamal".to_string()),
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
    assert_eq!(
        result,
        Err(AdmitSelectionsErrorReturn::UnknownValue {
            field: SelectionField::KeyAgreementConcrete,
            text: "elgamal".to_string(),
        })
    );
}

/// Contract: `key_agreement.identifier` outside its vocabulary →
///   `UnknownValue` with field `KeyAgreementIdentifier`, every other field
///   holding its family's vocabulary text.
/// Arrange: `key_agreement` `build_key_agreement_document` with `identifier`
///   `"elgamal_v1"`.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitSelectionsErrorReturn::UnknownValue {
///   field: SelectionField::KeyAgreementIdentifier,
///   text: "elgamal_v1".to_string() })`.
#[test]
fn admit_selections_refuses_an_unknown_key_agreement_identifier() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        key_agreement: Some(build_key_agreement_document(
            KeyAgreementDocumentOverrides {
                identifier: Some("elgamal_v1".to_string()),
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
    assert_eq!(
        result,
        Err(AdmitSelectionsErrorReturn::UnknownValue {
            field: SelectionField::KeyAgreementIdentifier,
            text: "elgamal_v1".to_string(),
        })
    );
}

/// Contract: `key_agreement.algebra` outside its vocabulary → `UnknownValue`
///   with field `KeyAgreementAlgebra`, every other field holding its family's
///   vocabulary text.
/// Arrange: `key_agreement` `build_key_agreement_document` with `algebra`
///   `"lattice"`.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitSelectionsErrorReturn::UnknownValue {
///   field: SelectionField::KeyAgreementAlgebra, text: "lattice".to_string()
///   })`.
#[test]
fn admit_selections_refuses_an_unknown_key_agreement_algebra() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        key_agreement: Some(build_key_agreement_document(
            KeyAgreementDocumentOverrides {
                algebra: Some("lattice".to_string()),
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
    assert_eq!(
        result,
        Err(AdmitSelectionsErrorReturn::UnknownValue {
            field: SelectionField::KeyAgreementAlgebra,
            text: "lattice".to_string(),
        })
    );
}

/// Contract: `delivery_proof.concrete` outside its vocabulary →
///   `UnknownValue` with field `DeliveryProofConcrete`, every other field
///   holding its family's vocabulary text.
/// Arrange: `delivery_proof` `build_delivery_proof_document` with `concrete`
///   `"ed25519"`.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitSelectionsErrorReturn::UnknownValue {
///   field: SelectionField::DeliveryProofConcrete,
///   text: "ed25519".to_string() })`.
#[test]
fn admit_selections_refuses_an_unknown_delivery_proof_concrete() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        delivery_proof: Some(build_delivery_proof_document(
            DeliveryProofDocumentOverrides {
                concrete: Some("ed25519".to_string()),
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
    assert_eq!(
        result,
        Err(AdmitSelectionsErrorReturn::UnknownValue {
            field: SelectionField::DeliveryProofConcrete,
            text: "ed25519".to_string(),
        })
    );
}
