#![allow(clippy::expect_used)]

use pairing::{PAIRING_CONCRETES, PRECOMPILE_ENCODINGS, TARGET_GROUP_ENCODING_IDENTIFIERS};
use random::RANDOM_SOURCE_KINDS;

use super::super::layers::provides::{
    BenchmarkConcreteTableOverrides, CurveTableOverrides, build_benchmark_concrete_table,
    build_curve_table,
};
use super::admit_selections;
use super::interface::{AdmitSelectionsErrorReturn, PairingField};
use super::mock::{
    build_admit_selections_deps, build_admit_selections_params, build_admit_selections_payload,
    declared_name,
};

/// Contract: Random source → `Err(AdmitSelectionsErrorReturn::RandomSource {
///   text })`.
/// Arrange: the default payload with `random_source` `"unknown"`.
/// Act:     `admit_selections`, once.
/// Assert:  the result equals `Err(AdmitSelectionsErrorReturn::RandomSource {
///   text: "unknown".to_string() })`.
#[test]
fn admit_selections_refuses_an_unknown_random_source() {
    // Arrange
    let mut payload = build_admit_selections_payload(Default::default());
    payload.selections.random_source = "unknown".to_string();
    // Act
    let result = admit_selections(
        &build_admit_selections_deps(Default::default()),
        build_admit_selections_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result,
        Err(AdmitSelectionsErrorReturn::RandomSource {
            text: "unknown".to_string()
        })
    );
    // admit_selections_refuses_an_unknown_random_source
}

/// Contract: Encoding → `Err(AdmitSelectionsErrorReturn::Encoding { text })`.
/// Arrange: the default payload with `encoding` `"unknown"`.
/// Act:     `admit_selections`, once.
/// Assert:  the result equals `Err(AdmitSelectionsErrorReturn::Encoding {
///   text: "unknown".to_string() })`.
#[test]
fn admit_selections_refuses_an_unknown_encoding() {
    // Arrange
    let mut payload = build_admit_selections_payload(Default::default());
    payload.selections.encoding = "unknown".to_string();
    // Act
    let result = admit_selections(
        &build_admit_selections_deps(Default::default()),
        build_admit_selections_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result,
        Err(AdmitSelectionsErrorReturn::Encoding {
            text: "unknown".to_string()
        })
    );
    // admit_selections_refuses_an_unknown_encoding
}

/// Contract: Hash-to-scalar → `Err(AdmitSelectionsErrorReturn::HashToScalar {
///   text })`.
/// Arrange: the default payload with `hash_to_scalar` `"unknown"`.
/// Act:     `admit_selections`, once.
/// Assert:  the result equals `Err(AdmitSelectionsErrorReturn::HashToScalar {
///   text: "unknown".to_string() })`.
#[test]
fn admit_selections_refuses_an_unknown_hash_to_scalar() {
    // Arrange
    let mut payload = build_admit_selections_payload(Default::default());
    payload.selections.hash_to_scalar = "unknown".to_string();
    // Act
    let result = admit_selections(
        &build_admit_selections_deps(Default::default()),
        build_admit_selections_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result,
        Err(AdmitSelectionsErrorReturn::HashToScalar {
            text: "unknown".to_string()
        })
    );
    // admit_selections_refuses_an_unknown_hash_to_scalar
}

/// Contract: Chain forms → `Err(AdmitSelectionsErrorReturn::ChainForms {
///   text })`.
/// Arrange: the default payload with `chain_forms` `"unknown"`.
/// Act:     `admit_selections`, once.
/// Assert:  the result equals `Err(AdmitSelectionsErrorReturn::ChainForms {
///   text: "unknown".to_string() })`.
#[test]
fn admit_selections_refuses_an_unknown_chain_forms() {
    // Arrange
    let mut payload = build_admit_selections_payload(Default::default());
    payload.selections.chain_forms = "unknown".to_string();
    // Act
    let result = admit_selections(
        &build_admit_selections_deps(Default::default()),
        build_admit_selections_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result,
        Err(AdmitSelectionsErrorReturn::ChainForms {
            text: "unknown".to_string()
        })
    );
    // admit_selections_refuses_an_unknown_chain_forms
}

/// Contract: Verifier → `Err(AdmitSelectionsErrorReturn::Verifier { text })`.
/// Arrange: the default payload with `verifier` `"unknown"`.
/// Act:     `admit_selections`, once.
/// Assert:  the result equals `Err(AdmitSelectionsErrorReturn::Verifier {
///   text: "unknown".to_string() })`.
#[test]
fn admit_selections_refuses_an_unknown_verifier() {
    // Arrange
    let mut payload = build_admit_selections_payload(Default::default());
    payload.selections.verifier = "unknown".to_string();
    // Act
    let result = admit_selections(
        &build_admit_selections_deps(Default::default()),
        build_admit_selections_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result,
        Err(AdmitSelectionsErrorReturn::Verifier {
            text: "unknown".to_string()
        })
    );
    // admit_selections_refuses_an_unknown_verifier
}

/// Contract: Identity scope → `Err(AdmitSelectionsErrorReturn::IdentityScope {
///   text })`.
/// Arrange: the default payload with `identity_scope` `"unknown"`.
/// Act:     `admit_selections`, once.
/// Assert:  the result equals `Err(AdmitSelectionsErrorReturn::IdentityScope {
///   text: "unknown".to_string() })`.
#[test]
fn admit_selections_refuses_an_unknown_identity_scope() {
    // Arrange
    let mut payload = build_admit_selections_payload(Default::default());
    payload.selections.identity_scope = "unknown".to_string();
    // Act
    let result = admit_selections(
        &build_admit_selections_deps(Default::default()),
        build_admit_selections_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result,
        Err(AdmitSelectionsErrorReturn::IdentityScope {
            text: "unknown".to_string()
        })
    );
    // admit_selections_refuses_an_unknown_identity_scope
}

/// Contract: Envelope algebra →
///   `Err(AdmitSelectionsErrorReturn::EnvelopeAlgebra { text })`.
/// Arrange: the default payload with `envelope_algebra` `"unknown"`.
/// Act:     `admit_selections`, once.
/// Assert:  the result equals `Err(AdmitSelectionsErrorReturn::EnvelopeAlgebra
///   { text: "unknown".to_string() })`.
#[test]
fn admit_selections_refuses_an_unknown_envelope_algebra() {
    // Arrange
    let mut payload = build_admit_selections_payload(Default::default());
    payload.selections.envelope_algebra = "unknown".to_string();
    // Act
    let result = admit_selections(
        &build_admit_selections_deps(Default::default()),
        build_admit_selections_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result,
        Err(AdmitSelectionsErrorReturn::EnvelopeAlgebra {
            text: "unknown".to_string()
        })
    );
    // admit_selections_refuses_an_unknown_envelope_algebra
}

/// Contract: KEM → `Err(AdmitSelectionsErrorReturn::Kem { text })`.
/// Arrange: the default payload with `kem` `"unknown"`.
/// Act:     `admit_selections`, once.
/// Assert:  the result equals `Err(AdmitSelectionsErrorReturn::Kem {
///   text: "unknown".to_string() })`.
#[test]
fn admit_selections_refuses_an_unknown_kem() {
    // Arrange
    let mut payload = build_admit_selections_payload(Default::default());
    payload.selections.kem = "unknown".to_string();
    // Act
    let result = admit_selections(
        &build_admit_selections_deps(Default::default()),
        build_admit_selections_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result,
        Err(AdmitSelectionsErrorReturn::Kem {
            text: "unknown".to_string()
        })
    );
    // admit_selections_refuses_an_unknown_kem
}

/// Contract: Key agreement → `Err(AdmitSelectionsErrorReturn::KeyAgreement {
///   text })`.
/// Arrange: the default payload with `key_agreement` `"unknown"`.
/// Act:     `admit_selections`, once.
/// Assert:  the result equals `Err(AdmitSelectionsErrorReturn::KeyAgreement {
///   text: "unknown".to_string() })`.
#[test]
fn admit_selections_refuses_an_unknown_key_agreement() {
    // Arrange
    let mut payload = build_admit_selections_payload(Default::default());
    payload.selections.key_agreement = "unknown".to_string();
    // Act
    let result = admit_selections(
        &build_admit_selections_deps(Default::default()),
        build_admit_selections_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result,
        Err(AdmitSelectionsErrorReturn::KeyAgreement {
            text: "unknown".to_string()
        })
    );
    // admit_selections_refuses_an_unknown_key_agreement
}

/// Contract: Delivery proof → `Err(AdmitSelectionsErrorReturn::DeliveryProof {
///   text })`.
/// Arrange: the default payload with `delivery_proof` `"unknown"`.
/// Act:     `admit_selections`, once.
/// Assert:  the result equals `Err(AdmitSelectionsErrorReturn::DeliveryProof {
///   text: "unknown".to_string() })`.
#[test]
fn admit_selections_refuses_an_unknown_delivery_proof() {
    // Arrange
    let mut payload = build_admit_selections_payload(Default::default());
    payload.selections.delivery_proof = "unknown".to_string();
    // Act
    let result = admit_selections(
        &build_admit_selections_deps(Default::default()),
        build_admit_selections_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result,
        Err(AdmitSelectionsErrorReturn::DeliveryProof {
            text: "unknown".to_string()
        })
    );
    // admit_selections_refuses_an_unknown_delivery_proof
}

/// Contract: Curves → `Err(AdmitSelectionsErrorReturn::Curve { index, field,
///   text })`, the index of the entry and `PairingField::PairingConcrete`.
/// Arrange: two curves, the first admitted and the second with
///   `pairing_concrete` `"unknown"`.
/// Act:     `admit_selections`, once.
/// Assert:  the result equals `Err(AdmitSelectionsErrorReturn::Curve {
///   index: 1, field: PairingField::PairingConcrete,
///   text: "unknown".to_string() })`.
#[test]
fn admit_selections_refuses_an_unknown_curve_pairing_concrete_with_its_index_and_field() {
    // Arrange
    let mut payload = build_admit_selections_payload(Default::default());
    payload.selections.curves = vec![
        build_curve_table(CurveTableOverrides {
            pairing_concrete: Some(declared_name(&PAIRING_CONCRETES[0])),
            target_group_encoding: Some(declared_name(&TARGET_GROUP_ENCODING_IDENTIFIERS[0])),
            ..Default::default()
        }),
        build_curve_table(CurveTableOverrides {
            pairing_concrete: Some("unknown".to_string()),
            target_group_encoding: Some(declared_name(&TARGET_GROUP_ENCODING_IDENTIFIERS[0])),
            ..Default::default()
        }),
    ];
    // Act
    let result = admit_selections(
        &build_admit_selections_deps(Default::default()),
        build_admit_selections_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result,
        Err(AdmitSelectionsErrorReturn::Curve {
            index: 1,
            field: PairingField::PairingConcrete,
            text: "unknown".to_string()
        })
    );
    // admit_selections_refuses_an_unknown_curve_pairing_concrete_with_its_index_and_field
}

/// Contract: Curves → the refusal names `PairingField::TargetGroupEncoding`
///   when the concrete is named and the encoding is not.
/// Arrange: one curve with `target_group_encoding` `"unknown"`.
/// Act:     `admit_selections`, once.
/// Assert:  the result equals `Err(AdmitSelectionsErrorReturn::Curve {
///   index: 0, field: PairingField::TargetGroupEncoding,
///   text: "unknown".to_string() })`.
#[test]
fn admit_selections_refuses_an_unknown_curve_target_group_encoding_with_its_index_and_field() {
    // Arrange
    let mut payload = build_admit_selections_payload(Default::default());
    payload.selections.curves = vec![build_curve_table(CurveTableOverrides {
        pairing_concrete: Some(declared_name(&PAIRING_CONCRETES[0])),
        target_group_encoding: Some("unknown".to_string()),
        ..Default::default()
    })];
    // Act
    let result = admit_selections(
        &build_admit_selections_deps(Default::default()),
        build_admit_selections_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result,
        Err(AdmitSelectionsErrorReturn::Curve {
            index: 0,
            field: PairingField::TargetGroupEncoding,
            text: "unknown".to_string()
        })
    );
    // admit_selections_refuses_an_unknown_curve_target_group_encoding_with_its_index_and_field
}

/// Contract: Benchmark concretes → `Err(AdmitSelectionsErrorReturn::
///   BenchmarkConcrete { index, field, text })`.
/// Arrange: one benchmark concrete with `pairing_concrete` `"unknown"`.
/// Act:     `admit_selections`, once.
/// Assert:  the result equals `Err(AdmitSelectionsErrorReturn::
///   BenchmarkConcrete { index: 0, field: PairingField::PairingConcrete,
///   text: "unknown".to_string() })`.
#[test]
fn admit_selections_refuses_an_unknown_benchmark_concrete_with_its_index_and_field() {
    // Arrange
    let mut payload = build_admit_selections_payload(Default::default());
    payload.selections.benchmark_concretes = vec![build_benchmark_concrete_table(
        BenchmarkConcreteTableOverrides {
            pairing_concrete: Some("unknown".to_string()),
            target_group_encoding: Some(declared_name(&TARGET_GROUP_ENCODING_IDENTIFIERS[0])),
        },
    )];
    // Act
    let result = admit_selections(
        &build_admit_selections_deps(Default::default()),
        build_admit_selections_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result,
        Err(AdmitSelectionsErrorReturn::BenchmarkConcrete {
            index: 0,
            field: PairingField::PairingConcrete,
            text: "unknown".to_string()
        })
    );
    // admit_selections_refuses_an_unknown_benchmark_concrete_with_its_index_and_field
}

/// Contract: Precompile encodings → `Err(AdmitSelectionsErrorReturn::
///   PrecompileEncoding { index, text })`.
/// Arrange: a list of an admitted text then `"unknown"`.
/// Act:     `admit_selections`, once.
/// Assert:  the result equals `Err(AdmitSelectionsErrorReturn::
///   PrecompileEncoding { index: 1, text: "unknown".to_string() })`.
#[test]
fn admit_selections_refuses_an_unknown_precompile_encoding_with_its_index() {
    // Arrange
    let mut payload = build_admit_selections_payload(Default::default());
    payload.precompile_encodings = vec![
        declared_name(&PRECOMPILE_ENCODINGS[0]),
        "unknown".to_string(),
    ];
    // Act
    let result = admit_selections(
        &build_admit_selections_deps(Default::default()),
        build_admit_selections_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result,
        Err(AdmitSelectionsErrorReturn::PrecompileEncoding {
            index: 1,
            text: "unknown".to_string()
        })
    );
    // admit_selections_refuses_an_unknown_precompile_encoding_with_its_index
}

/// Contract: the naming rule → a text equals a member's `Debug` text whole
///   and case-sensitively.
/// Arrange: the default payload with `random_source` the lowercase of
///   `declared_name(&RANDOM_SOURCE_KINDS[0])`.
/// Act:     `admit_selections`, once.
/// Assert:  the result equals `Err(AdmitSelectionsErrorReturn::RandomSource {
///   text })` with `text` the lowercase text.
#[test]
fn admit_selections_refuses_a_name_that_differs_only_in_case() {
    // Arrange
    let text = declared_name(&RANDOM_SOURCE_KINDS[0]).to_lowercase();
    let mut payload = build_admit_selections_payload(Default::default());
    payload.selections.random_source = text.clone();
    // Act
    let result = admit_selections(
        &build_admit_selections_deps(Default::default()),
        build_admit_selections_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result,
        Err(AdmitSelectionsErrorReturn::RandomSource { text })
    );
    // admit_selections_refuses_a_name_that_differs_only_in_case
}

/// Contract: Ordering → the first unknown text in the stated order returns
///   and later texts are unread.
/// Arrange: the default payload with `verifier` `"unknown"` and
///   `delivery_proof` `"unknown"` and an unknown `precompile_encodings`
///   entry.
/// Act:     `admit_selections`, once.
/// Assert:  the result equals `Err(AdmitSelectionsErrorReturn::Verifier {
///   text: "unknown".to_string() })`.
#[test]
fn admit_selections_returns_the_first_refusal_in_the_stated_order() {
    // Arrange
    let mut payload = build_admit_selections_payload(Default::default());
    payload.selections.verifier = "unknown".to_string();
    payload.selections.delivery_proof = "unknown".to_string();
    payload.precompile_encodings = vec!["unknown".to_string()];
    // Act
    let result = admit_selections(
        &build_admit_selections_deps(Default::default()),
        build_admit_selections_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result,
        Err(AdmitSelectionsErrorReturn::Verifier {
            text: "unknown".to_string()
        })
    );
    // admit_selections_returns_the_first_refusal_in_the_stated_order
}
