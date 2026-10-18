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

/// Contract: `random.source` and `encoding.concrete` both outside their
///   vocabularies → `UnknownValue` with field `RandomSource`, the family
///   fields admitted in the order listed and the first refusal returning.
/// Arrange: `random` `build_random_document` with `source` `"system"` and
///   `encoding` `build_selection_document` with `concrete` `"json"`.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitSelectionsErrorReturn::UnknownValue {
///   field: SelectionField::RandomSource, text: "system".to_string() })`.
#[test]
fn admit_selections_refuses_the_random_source_before_the_encoding_concrete() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        random: Some(build_random_document(RandomDocumentOverrides {
            source: Some("system".to_string()),
        })),
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
            field: SelectionField::RandomSource,
            text: "system".to_string(),
        })
    );
}

/// Contract: `encoding.concrete` and `encoding.identifier` both outside
///   their vocabularies → `UnknownValue` with field `EncodingConcrete`.
/// Arrange: `encoding` `build_selection_document` with `concrete` `"json"`
///   and `identifier` `"json_v1"`.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitSelectionsErrorReturn::UnknownValue {
///   field: SelectionField::EncodingConcrete, text: "json".to_string() })`.
#[test]
fn admit_selections_refuses_the_encoding_concrete_before_the_encoding_identifier() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        encoding: Some(build_selection_document(SelectionDocumentOverrides {
            concrete: Some("json".to_string()),
            identifier: Some("json_v1".to_string()),
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

/// Contract: `encoding.identifier` and `hash_to_scalar.concrete` both
///   outside their vocabularies → `UnknownValue` with field
///   `EncodingIdentifier`.
/// Arrange: `encoding` `build_selection_document` with `identifier`
///   `"json_v1"` and `hash_to_scalar` `build_selection_document` with
///   `concrete` `"sha256"` and `identifier` `"keccak256_v1"`.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitSelectionsErrorReturn::UnknownValue {
///   field: SelectionField::EncodingIdentifier, text: "json_v1".to_string()
///   })`.
#[test]
fn admit_selections_refuses_the_encoding_identifier_before_the_hash_to_scalar_concrete() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        encoding: Some(build_selection_document(SelectionDocumentOverrides {
            identifier: Some("json_v1".to_string()),
            ..Default::default()
        })),
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
            field: SelectionField::EncodingIdentifier,
            text: "json_v1".to_string(),
        })
    );
}

/// Contract: `hash_to_scalar.concrete` and `hash_to_scalar.identifier` both
///   outside their vocabularies → `UnknownValue` with field
///   `HashToScalarConcrete`.
/// Arrange: `hash_to_scalar` `build_selection_document` with `concrete`
///   `"sha256"` and `identifier` `"sha256_v1"`.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitSelectionsErrorReturn::UnknownValue {
///   field: SelectionField::HashToScalarConcrete,
///   text: "sha256".to_string() })`.
#[test]
fn admit_selections_refuses_the_hash_to_scalar_concrete_before_the_hash_to_scalar_identifier() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        hash_to_scalar: Some(build_selection_document(SelectionDocumentOverrides {
            concrete: Some("sha256".to_string()),
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
            field: SelectionField::HashToScalarConcrete,
            text: "sha256".to_string(),
        })
    );
}

/// Contract: `hash_to_scalar.identifier` and `chain_forms.concrete` both
///   outside their vocabularies → `UnknownValue` with field
///   `HashToScalarIdentifier`.
/// Arrange: `hash_to_scalar` `build_selection_document` with `concrete`
///   `"keccak256"` and `identifier` `"sha256_v1"` and `chain_forms`
///   `build_selection_document` with `concrete` `"solana"` and `identifier`
///   `"evm_v1"`.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitSelectionsErrorReturn::UnknownValue {
///   field: SelectionField::HashToScalarIdentifier,
///   text: "sha256_v1".to_string() })`.
#[test]
fn admit_selections_refuses_the_hash_to_scalar_identifier_before_the_chain_forms_concrete() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        hash_to_scalar: Some(build_selection_document(SelectionDocumentOverrides {
            concrete: Some("keccak256".to_string()),
            identifier: Some("sha256_v1".to_string()),
        })),
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
            field: SelectionField::HashToScalarIdentifier,
            text: "sha256_v1".to_string(),
        })
    );
}

/// Contract: `chain_forms.concrete` and `chain_forms.identifier` both
///   outside their vocabularies → `UnknownValue` with field
///   `ChainFormsConcrete`.
/// Arrange: `chain_forms` `build_selection_document` with `concrete`
///   `"solana"` and `identifier` `"solana_v1"`.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitSelectionsErrorReturn::UnknownValue {
///   field: SelectionField::ChainFormsConcrete, text: "solana".to_string()
///   })`.
#[test]
fn admit_selections_refuses_the_chain_forms_concrete_before_the_chain_forms_identifier() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        chain_forms: Some(build_selection_document(SelectionDocumentOverrides {
            concrete: Some("solana".to_string()),
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
            field: SelectionField::ChainFormsConcrete,
            text: "solana".to_string(),
        })
    );
}

/// Contract: `chain_forms.identifier` and `kem.concrete` both outside their
///   vocabularies → `UnknownValue` with field `ChainFormsIdentifier`.
/// Arrange: `chain_forms` `build_selection_document` with `concrete` `"evm"`
///   and `identifier` `"solana_v1"` and `kem` `build_kem_document` with
///   `concrete` `"bb2_depth_two"`.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitSelectionsErrorReturn::UnknownValue {
///   field: SelectionField::ChainFormsIdentifier,
///   text: "solana_v1".to_string() })`.
#[test]
fn admit_selections_refuses_the_chain_forms_identifier_before_the_kem_concrete() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        chain_forms: Some(build_selection_document(SelectionDocumentOverrides {
            concrete: Some("evm".to_string()),
            identifier: Some("solana_v1".to_string()),
        })),
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
            field: SelectionField::ChainFormsIdentifier,
            text: "solana_v1".to_string(),
        })
    );
}

/// Contract: `kem.concrete` and `kem.identifier` both outside their
///   vocabularies → `UnknownValue` with field `KemConcrete`.
/// Arrange: `kem` `build_kem_document` with `concrete` `"bb2_depth_two"` and
///   `identifier` `"bb2_depth_two_v1"`.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitSelectionsErrorReturn::UnknownValue {
///   field: SelectionField::KemConcrete, text: "bb2_depth_two".to_string()
///   })`.
#[test]
fn admit_selections_refuses_the_kem_concrete_before_the_kem_identifier() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        kem: Some(build_kem_document(KemDocumentOverrides {
            concrete: Some("bb2_depth_two".to_string()),
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
            field: SelectionField::KemConcrete,
            text: "bb2_depth_two".to_string(),
        })
    );
}

/// Contract: `kem.identifier` and `kem.scope` both outside their
///   vocabularies → `UnknownValue` with field `KemIdentifier`.
/// Arrange: `kem` `build_kem_document` with `identifier`
///   `"bb2_depth_two_v1"` and `scope` `"holder"`.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitSelectionsErrorReturn::UnknownValue {
///   field: SelectionField::KemIdentifier,
///   text: "bb2_depth_two_v1".to_string() })`.
#[test]
fn admit_selections_refuses_the_kem_identifier_before_the_kem_scope() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        kem: Some(build_kem_document(KemDocumentOverrides {
            identifier: Some("bb2_depth_two_v1".to_string()),
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
            field: SelectionField::KemIdentifier,
            text: "bb2_depth_two_v1".to_string(),
        })
    );
}

/// Contract: `kem.scope` and `key_agreement.concrete` both outside their
///   vocabularies → `UnknownValue` with field `KemScope`.
/// Arrange: `kem` `build_kem_document` with `scope` `"holder"` and
///   `key_agreement` `build_key_agreement_document` with `concrete`
///   `"elgamal"`.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitSelectionsErrorReturn::UnknownValue {
///   field: SelectionField::KemScope, text: "holder".to_string() })`.
#[test]
fn admit_selections_refuses_the_kem_scope_before_the_key_agreement_concrete() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        kem: Some(build_kem_document(KemDocumentOverrides {
            scope: Some("holder".to_string()),
            ..Default::default()
        })),
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
            field: SelectionField::KemScope,
            text: "holder".to_string(),
        })
    );
}

/// Contract: `key_agreement.concrete` and `key_agreement.identifier` both
///   outside their vocabularies → `UnknownValue` with field
///   `KeyAgreementConcrete`.
/// Arrange: `key_agreement` `build_key_agreement_document` with `concrete`
///   `"elgamal"` and `identifier` `"elgamal_v1"`.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitSelectionsErrorReturn::UnknownValue {
///   field: SelectionField::KeyAgreementConcrete,
///   text: "elgamal".to_string() })`.
#[test]
fn admit_selections_refuses_the_key_agreement_concrete_before_the_key_agreement_identifier() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        key_agreement: Some(build_key_agreement_document(
            KeyAgreementDocumentOverrides {
                concrete: Some("elgamal".to_string()),
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
            field: SelectionField::KeyAgreementConcrete,
            text: "elgamal".to_string(),
        })
    );
}

/// Contract: `key_agreement.identifier` and `key_agreement.algebra` both
///   outside their vocabularies → `UnknownValue` with field
///   `KeyAgreementIdentifier`.
/// Arrange: `key_agreement` `build_key_agreement_document` with `identifier`
///   `"elgamal_v1"` and `algebra` `"lattice"`.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitSelectionsErrorReturn::UnknownValue {
///   field: SelectionField::KeyAgreementIdentifier,
///   text: "elgamal_v1".to_string() })`.
#[test]
fn admit_selections_refuses_the_key_agreement_identifier_before_the_key_agreement_algebra() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        key_agreement: Some(build_key_agreement_document(
            KeyAgreementDocumentOverrides {
                identifier: Some("elgamal_v1".to_string()),
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
            field: SelectionField::KeyAgreementIdentifier,
            text: "elgamal_v1".to_string(),
        })
    );
}

/// Contract: `key_agreement.algebra` and `delivery_proof.concrete` both
///   outside their vocabularies → `UnknownValue` with field
///   `KeyAgreementAlgebra`.
/// Arrange: `key_agreement` `build_key_agreement_document` with `algebra`
///   `"lattice"` and `delivery_proof` `build_delivery_proof_document` with
///   `concrete` `"ed25519"`.
/// Act:    `admit_selections(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitSelectionsErrorReturn::UnknownValue {
///   field: SelectionField::KeyAgreementAlgebra, text: "lattice".to_string()
///   })`.
#[test]
fn admit_selections_refuses_the_key_agreement_algebra_before_the_delivery_proof_concrete() {
    // Arrange
    let document = build_harness_config_document(HarnessConfigDocumentOverrides {
        key_agreement: Some(build_key_agreement_document(
            KeyAgreementDocumentOverrides {
                algebra: Some("lattice".to_string()),
                ..Default::default()
            },
        )),
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
            field: SelectionField::KeyAgreementAlgebra,
            text: "lattice".to_string(),
        })
    );
}
