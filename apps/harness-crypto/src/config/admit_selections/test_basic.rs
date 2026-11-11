#![allow(clippy::expect_used)]

use chain::DECLARED_CHAIN_FORMS_SELECTIONS;
use encoding::DECLARED_ENCODING_SELECTIONS;
use envelope::DECLARED_KEY_AGREEMENT_SELECTIONS;
use hash_to_scalar::DECLARED_HASH_TO_SCALAR_SELECTIONS;
use kem::DECLARED_KEM_SELECTIONS;
use pairing::{PAIRING_CONCRETES, PRECOMPILE_ENCODINGS, TARGET_GROUP_ENCODING_IDENTIFIERS};
use proof::DELIVERY_PROOF_CONCRETES;
use random::RANDOM_SOURCE_KINDS;

use super::super::layers::provides::{
    BenchmarkConcreteTableOverrides, CurveTableOverrides, build_benchmark_concrete_table,
    build_curve_table,
};
use super::admit_selections;
use super::interface::{
    AdmittedCurve, AdmittedDeliveryProof, AdmittedKem, AdmittedKeyAgreement, AdmittedPairing,
};
use super::mock::{
    AdmittedSelectionsOverrides, build_admit_selections_deps, build_admit_selections_params,
    build_admit_selections_payload, build_admitted_selections, declared_name,
};
use crate::verifier::provides::VERIFIER_CONCRETES;

/// Contract: Success → a payload whose every text names the first member of
///   its declared set admits to the selections built from those members.
/// Arrange: `build_admit_selections_payload(Default::default())`.
/// Act:     `admit_selections`, once.
/// Assert:  the result is `Ok` and its `admitted` equals
///   `build_admitted_selections(Default::default())`.
#[test]
fn admit_selections_admits_the_default_payload() {
    // Arrange
    let payload = build_admit_selections_payload(Default::default());
    let expected = build_admitted_selections(Default::default());
    // Act
    let result = admit_selections(
        &build_admit_selections_deps(Default::default()),
        build_admit_selections_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(result.expect("the admission succeeds").admitted, expected);
    // admit_selections_admits_the_default_payload
}

/// Contract: Random source and Verifier → each text names its declared member
///   and the admitted value is that member.
/// Arrange: the default payload.
/// Act:     `admit_selections`, once.
/// Assert:  `admitted.random_source` equals `RANDOM_SOURCE_KINDS[0]` and
///   `admitted.verifier` equals `VERIFIER_CONCRETES[0]`.
#[test]
fn admit_selections_admits_the_random_source_and_the_verifier_the_texts_name() {
    // Arrange
    let payload = build_admit_selections_payload(Default::default());
    // Act
    let admitted = admit_selections(
        &build_admit_selections_deps(Default::default()),
        build_admit_selections_params(Default::default()),
        payload,
    )
    .expect("the admission succeeds")
    .admitted;
    // Assert
    assert_eq!(admitted.random_source, RANDOM_SOURCE_KINDS[0]);
    assert_eq!(admitted.verifier, VERIFIER_CONCRETES[0]);
    // admit_selections_admits_the_random_source_and_the_verifier_the_texts_name
}

/// Contract: Encoding, Hash-to-scalar, and Chain forms → the admitted
///   selection holds the identifier of the member whose concrete the text
///   names, which the document does not carry.
/// Arrange: the default payload.
/// Act:     `admit_selections`, once.
/// Assert:  `admitted.encoding.identifier` equals
///   `DECLARED_ENCODING_SELECTIONS[0].identifier`,
///   `admitted.hash_to_scalar.identifier` equals
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS[0].identifier`, and
///   `admitted.chain_forms.identifier` equals
///   `DECLARED_CHAIN_FORMS_SELECTIONS[0].identifier`.
#[test]
fn admit_selections_carries_the_identifier_its_declared_member_carries() {
    // Arrange
    let payload = build_admit_selections_payload(Default::default());
    // Act
    let admitted = admit_selections(
        &build_admit_selections_deps(Default::default()),
        build_admit_selections_params(Default::default()),
        payload,
    )
    .expect("the admission succeeds")
    .admitted;
    // Assert
    assert_eq!(
        admitted.encoding.identifier,
        DECLARED_ENCODING_SELECTIONS[0].identifier
    );
    assert_eq!(
        admitted.hash_to_scalar.identifier,
        DECLARED_HASH_TO_SCALAR_SELECTIONS[0].identifier
    );
    assert_eq!(
        admitted.chain_forms.identifier,
        DECLARED_CHAIN_FORMS_SELECTIONS[0].identifier
    );
    // admit_selections_carries_the_identifier_its_declared_member_carries
}

/// Contract: Identity scope and KEM → the admitted selection holds the
///   admitted scope and the identifier of the member carrying the named
///   concrete and that scope.
/// Arrange: the default payload.
/// Act:     `admit_selections`, once.
/// Assert:  `admitted.kem` equals `AdmittedKem` built from
///   `DECLARED_KEM_SELECTIONS[0]`.
#[test]
fn admit_selections_admits_the_kem_with_the_scope_the_text_names() {
    // Arrange
    let payload = build_admit_selections_payload(Default::default());
    let member = DECLARED_KEM_SELECTIONS[0];
    let expected = AdmittedKem {
        concrete: member.concrete,
        identifier: member.identifier,
        scope: member.scope,
    };
    // Act
    let admitted = admit_selections(
        &build_admit_selections_deps(Default::default()),
        build_admit_selections_params(Default::default()),
        payload,
    )
    .expect("the admission succeeds")
    .admitted;
    // Assert
    assert_eq!(admitted.kem, expected);
    // admit_selections_admits_the_kem_with_the_scope_the_text_names
}

/// Contract: Envelope algebra and Key agreement → the admitted selection
///   holds the admitted algebra and the member's identifier.
/// Arrange: the default payload.
/// Act:     `admit_selections`, once.
/// Assert:  `admitted.key_agreement` equals `AdmittedKeyAgreement` built from
///   `DECLARED_KEY_AGREEMENT_SELECTIONS[0]`.
#[test]
fn admit_selections_admits_the_key_agreement_with_the_algebra_the_text_names() {
    // Arrange
    let payload = build_admit_selections_payload(Default::default());
    let member = DECLARED_KEY_AGREEMENT_SELECTIONS[0];
    let expected = AdmittedKeyAgreement {
        concrete: member.concrete,
        identifier: member.identifier,
        algebra: member.algebra,
    };
    // Act
    let admitted = admit_selections(
        &build_admit_selections_deps(Default::default()),
        build_admit_selections_params(Default::default()),
        payload,
    )
    .expect("the admission succeeds")
    .admitted;
    // Assert
    assert_eq!(admitted.key_agreement, expected);
    // admit_selections_admits_the_key_agreement_with_the_algebra_the_text_names
}

/// Contract: Delivery proof → the selection holds the member, the admitted
///   algebra, and the statement version as the document carries it.
/// Arrange: the default payload with `delivery_statement_version` `7` in its
///   `SelectionsTable`.
/// Act:     `admit_selections`, once.
/// Assert:  `admitted.delivery_proof` equals `AdmittedDeliveryProof` of the
///   first member, the admitted algebra, and `7`.
#[test]
fn admit_selections_admits_the_delivery_proof_with_the_admitted_algebra_and_the_documents_version()
{
    // Arrange
    let mut payload = build_admit_selections_payload(Default::default());
    payload.selections.delivery_statement_version = 7;
    let expected = AdmittedDeliveryProof {
        concrete: DELIVERY_PROOF_CONCRETES[0],
        algebra: DECLARED_KEY_AGREEMENT_SELECTIONS[0].algebra,
        statement_version: 7,
    };
    // Act
    let admitted = admit_selections(
        &build_admit_selections_deps(Default::default()),
        build_admit_selections_params(Default::default()),
        payload,
    )
    .expect("the admission succeeds")
    .admitted;
    // Assert
    assert_eq!(admitted.delivery_proof, expected);
    // admit_selections_admits_the_delivery_proof_with_the_admitted_algebra_and_the_documents_version
}

/// Contract: Curves → each entry admitted with its label, in the document's
///   order.
/// Arrange: two `curves` entries of labels `"curve-a"` and `"curve-b"`, the
///   first naming the first members of `PAIRING_CONCRETES` and
///   `TARGET_GROUP_ENCODING_IDENTIFIERS` and the second the last members of
///   each.
/// Act:     `admit_selections`, once.
/// Assert:  `admitted.curves` equals two `AdmittedCurve` values in that order
///   with those labels and members.
#[test]
fn admit_selections_admits_each_curve_with_its_label_in_order() {
    // Arrange
    let mut payload = build_admit_selections_payload(Default::default());
    payload.selections.curves = vec![
        build_curve_table(CurveTableOverrides {
            label: Some("curve-a".to_string()),
            pairing_concrete: Some(declared_name(&PAIRING_CONCRETES[0])),
            target_group_encoding: Some(declared_name(&TARGET_GROUP_ENCODING_IDENTIFIERS[0])),
        }),
        build_curve_table(CurveTableOverrides {
            label: Some("curve-b".to_string()),
            pairing_concrete: Some(declared_name(
                &PAIRING_CONCRETES[PAIRING_CONCRETES.len() - 1],
            )),
            target_group_encoding: Some(declared_name(
                &TARGET_GROUP_ENCODING_IDENTIFIERS[TARGET_GROUP_ENCODING_IDENTIFIERS.len() - 1],
            )),
        }),
    ];
    let expected = vec![
        AdmittedCurve {
            label: "curve-a".to_string(),
            pairing: AdmittedPairing {
                concrete: PAIRING_CONCRETES[0],
                target_group_encoding: TARGET_GROUP_ENCODING_IDENTIFIERS[0],
            },
        },
        AdmittedCurve {
            label: "curve-b".to_string(),
            pairing: AdmittedPairing {
                concrete: PAIRING_CONCRETES[PAIRING_CONCRETES.len() - 1],
                target_group_encoding: TARGET_GROUP_ENCODING_IDENTIFIERS
                    [TARGET_GROUP_ENCODING_IDENTIFIERS.len() - 1],
            },
        },
    ];
    // Act
    let admitted = admit_selections(
        &build_admit_selections_deps(Default::default()),
        build_admit_selections_params(Default::default()),
        payload,
    )
    .expect("the admission succeeds")
    .admitted;
    // Assert
    assert_eq!(admitted.curves, expected);
    // admit_selections_admits_each_curve_with_its_label_in_order
}

/// Contract: Benchmark concretes → each entry admitted in the document's
///   order.
/// Arrange: two `benchmark_concretes` entries naming the first and the last
///   members of each declared set.
/// Act:     `admit_selections`, once.
/// Assert:  `admitted.benchmark_concretes` equals two `AdmittedPairing` values
///   in that order.
#[test]
fn admit_selections_admits_each_benchmark_concrete_in_order() {
    // Arrange
    let mut payload = build_admit_selections_payload(Default::default());
    payload.selections.benchmark_concretes = vec![
        build_benchmark_concrete_table(BenchmarkConcreteTableOverrides {
            pairing_concrete: Some(declared_name(&PAIRING_CONCRETES[0])),
            target_group_encoding: Some(declared_name(&TARGET_GROUP_ENCODING_IDENTIFIERS[0])),
        }),
        build_benchmark_concrete_table(BenchmarkConcreteTableOverrides {
            pairing_concrete: Some(declared_name(
                &PAIRING_CONCRETES[PAIRING_CONCRETES.len() - 1],
            )),
            target_group_encoding: Some(declared_name(
                &TARGET_GROUP_ENCODING_IDENTIFIERS[TARGET_GROUP_ENCODING_IDENTIFIERS.len() - 1],
            )),
        }),
    ];
    let expected = vec![
        AdmittedPairing {
            concrete: PAIRING_CONCRETES[0],
            target_group_encoding: TARGET_GROUP_ENCODING_IDENTIFIERS[0],
        },
        AdmittedPairing {
            concrete: PAIRING_CONCRETES[PAIRING_CONCRETES.len() - 1],
            target_group_encoding: TARGET_GROUP_ENCODING_IDENTIFIERS
                [TARGET_GROUP_ENCODING_IDENTIFIERS.len() - 1],
        },
    ];
    // Act
    let admitted = admit_selections(
        &build_admit_selections_deps(Default::default()),
        build_admit_selections_params(Default::default()),
        payload,
    )
    .expect("the admission succeeds")
    .admitted;
    // Assert
    assert_eq!(admitted.benchmark_concretes, expected);
    // admit_selections_admits_each_benchmark_concrete_in_order
}

/// Contract: Precompile encodings → each text admitted in the list's order.
/// Arrange: a `precompile_encodings` list of the last member's text then the
///   first member's text of `PRECOMPILE_ENCODINGS`.
/// Act:     `admit_selections`, once.
/// Assert:  `admitted.precompile_encodings` equals those two members in that
///   order.
#[test]
fn admit_selections_admits_each_precompile_encoding_in_order() {
    // Arrange
    let mut payload = build_admit_selections_payload(Default::default());
    payload.precompile_encodings = vec![
        declared_name(&PRECOMPILE_ENCODINGS[PRECOMPILE_ENCODINGS.len() - 1]),
        declared_name(&PRECOMPILE_ENCODINGS[0]),
    ];
    let expected = vec![
        PRECOMPILE_ENCODINGS[PRECOMPILE_ENCODINGS.len() - 1],
        PRECOMPILE_ENCODINGS[0],
    ];
    // Act
    let admitted = admit_selections(
        &build_admit_selections_deps(Default::default()),
        build_admit_selections_params(Default::default()),
        payload,
    )
    .expect("the admission succeeds")
    .admitted;
    // Assert
    assert_eq!(admitted.precompile_encodings, expected);
    // admit_selections_admits_each_precompile_encoding_in_order
}

/// Contract: Curves, Benchmark concretes, and Precompile encodings → an empty
///   list admits to an empty list.
/// Arrange: the default payload with `curves` and `benchmark_concretes` empty
///   and `precompile_encodings` empty.
/// Act:     `admit_selections`, once.
/// Assert:  the three admitted lists are empty and every other admitted value
///   equals the default's.
#[test]
fn admit_selections_admits_empty_lists_as_empty() {
    // Arrange
    let mut payload = build_admit_selections_payload(Default::default());
    payload.selections.curves = Vec::new();
    payload.selections.benchmark_concretes = Vec::new();
    payload.precompile_encodings = Vec::new();
    let expected = build_admitted_selections(AdmittedSelectionsOverrides {
        curves: Some(Vec::new()),
        benchmark_concretes: Some(Vec::new()),
        precompile_encodings: Some(Vec::new()),
        ..Default::default()
    });
    // Act
    let admitted = admit_selections(
        &build_admit_selections_deps(Default::default()),
        build_admit_selections_params(Default::default()),
        payload,
    )
    .expect("the admission succeeds")
    .admitted;
    // Assert
    assert!(admitted.curves.is_empty());
    assert!(admitted.benchmark_concretes.is_empty());
    assert!(admitted.precompile_encodings.is_empty());
    assert_eq!(admitted, expected);
    // admit_selections_admits_empty_lists_as_empty
}
