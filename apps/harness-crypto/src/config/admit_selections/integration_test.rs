#![allow(clippy::expect_used)]

use chain::{ChainFormsConcrete, DECLARED_CHAIN_FORMS_SELECTIONS};
use encoding::{DECLARED_ENCODING_SELECTIONS, EncodingConcrete};
use envelope::DECLARED_KEY_AGREEMENT_SELECTIONS;
use hash_to_scalar::{
    DECLARED_HASH_TO_SCALAR_SELECTIONS, HashToScalarConcrete, MockIHashToScalarAdapterFailureMode,
};
use kem::{DECLARED_KEM_SELECTIONS, KemConcrete};
use pairing::{PAIRING_CONCRETES, PRECOMPILE_ENCODINGS, TARGET_GROUP_ENCODING_IDENTIFIERS};
use proof::DELIVERY_PROOF_CONCRETES;
use random::{MockIRandomSourceAdapterFailureMode, RANDOM_SOURCE_KINDS, RandomSourceKind};

use super::super::layers::provides::{
    BenchmarkConcreteTableOverrides, CurveTableOverrides, SelectionsTable,
    build_benchmark_concrete_table, build_curve_table,
};
use super::admit_selections;
use super::interface::{
    AdmitSelectionsErrorReturn, AdmittedChainForms, AdmittedCurve, AdmittedDeliveryProof,
    AdmittedEncoding, AdmittedHashToScalar, AdmittedKem, AdmittedKeyAgreement, AdmittedPairing,
};
use super::mock::{
    build_admit_selections_deps, build_admit_selections_params, build_admit_selections_payload,
    declared_name,
};
use crate::verifier::provides::VERIFIER_CONCRETES;

/// Contract: `every_member_of_every_declared_set_is_admitted_by_its_own_name`
///   → `admit_selections` over a payload whose every text is the `Debug`
///   text of one member of the family's real declared set returns `Ok`
///   whose admitted value for each text equals the member the text names,
///   with the identifier, scope, and algebra that member carries.
/// Arrange: for each member of each declared set a payload from
///   `build_admit_selections_payload` whose text for that family is
///   `declared_name` of the member, the KEM and key agreement members
///   carrying their scope and algebra, and every member of
///   `PAIRING_CONCRETES` crossed with every member of
///   `TARGET_GROUP_ENCODING_IDENTIFIERS` as a curve and as a benchmark
///   concrete, and every member of `PRECOMPILE_ENCODINGS`.
/// Act:     `admit_selections`, once per payload.
/// Assert:  the result is `Ok` and the admitted value for that family equals
///   the expected value built from the member.
/// Boundary: the real `admit_selections`, `member_named`, and the families'
///   real declared sets.
/// Mocked:  none.
#[test]
fn every_member_of_every_declared_set_is_admitted_by_its_own_name() {
    // Arrange — the deps and params are invariant across every payload.
    let deps = build_admit_selections_deps(Default::default());
    let params = build_admit_selections_params(Default::default());

    // Act / Assert — every member of `RANDOM_SOURCE_KINDS` in turn.
    for member in RANDOM_SOURCE_KINDS {
        let mut payload = build_admit_selections_payload(Default::default());
        payload.selections.random_source = declared_name(member);
        let admitted = admit_selections(&deps, params, payload)
            .expect("the member is admitted")
            .admitted;
        assert_eq!(admitted.random_source, *member);
    }

    // Act / Assert — every member of `DECLARED_ENCODING_SELECTIONS` in turn.
    for member in DECLARED_ENCODING_SELECTIONS {
        let mut payload = build_admit_selections_payload(Default::default());
        payload.selections.encoding = declared_name(&member.concrete);
        let admitted = admit_selections(&deps, params, payload)
            .expect("the member is admitted")
            .admitted;
        assert_eq!(
            admitted.encoding,
            AdmittedEncoding {
                concrete: member.concrete,
                identifier: member.identifier,
            }
        );
    }

    // Act / Assert — every member of `DECLARED_HASH_TO_SCALAR_SELECTIONS` in turn.
    for member in DECLARED_HASH_TO_SCALAR_SELECTIONS {
        let mut payload = build_admit_selections_payload(Default::default());
        payload.selections.hash_to_scalar = declared_name(&member.concrete);
        let admitted = admit_selections(&deps, params, payload)
            .expect("the member is admitted")
            .admitted;
        assert_eq!(
            admitted.hash_to_scalar,
            AdmittedHashToScalar {
                concrete: member.concrete,
                identifier: member.identifier,
            }
        );
    }

    // Act / Assert — every member of `DECLARED_CHAIN_FORMS_SELECTIONS` in turn.
    for member in DECLARED_CHAIN_FORMS_SELECTIONS {
        let mut payload = build_admit_selections_payload(Default::default());
        payload.selections.chain_forms = declared_name(&member.concrete);
        let admitted = admit_selections(&deps, params, payload)
            .expect("the member is admitted")
            .admitted;
        assert_eq!(
            admitted.chain_forms,
            AdmittedChainForms {
                concrete: member.concrete,
                identifier: member.identifier,
            }
        );
    }

    // Act / Assert — every member of `VERIFIER_CONCRETES` in turn.
    for member in VERIFIER_CONCRETES {
        let mut payload = build_admit_selections_payload(Default::default());
        payload.selections.verifier = declared_name(member);
        let admitted = admit_selections(&deps, params, payload)
            .expect("the member is admitted")
            .admitted;
        assert_eq!(admitted.verifier, *member);
    }

    // Act / Assert — every member of `DECLARED_KEM_SELECTIONS` in turn, its
    // concrete named and its scope the admitted scope.
    for member in DECLARED_KEM_SELECTIONS {
        let mut payload = build_admit_selections_payload(Default::default());
        payload.selections.kem = declared_name(&member.concrete);
        payload.selections.identity_scope = declared_name(&member.scope);
        let admitted = admit_selections(&deps, params, payload)
            .expect("the member is admitted")
            .admitted;
        assert_eq!(
            admitted.kem,
            AdmittedKem {
                concrete: member.concrete,
                identifier: member.identifier,
                scope: member.scope,
            }
        );
    }

    // Act / Assert — every member of `DECLARED_KEY_AGREEMENT_SELECTIONS` in
    // turn, its concrete named and its algebra the admitted algebra.
    for member in DECLARED_KEY_AGREEMENT_SELECTIONS {
        let mut payload = build_admit_selections_payload(Default::default());
        payload.selections.key_agreement = declared_name(&member.concrete);
        payload.selections.envelope_algebra = declared_name(&member.algebra);
        let admitted = admit_selections(&deps, params, payload)
            .expect("the member is admitted")
            .admitted;
        assert_eq!(
            admitted.key_agreement,
            AdmittedKeyAgreement {
                concrete: member.concrete,
                identifier: member.identifier,
                algebra: member.algebra,
            }
        );
    }

    // Act / Assert — every member of `DELIVERY_PROOF_CONCRETES` in turn, the
    // algebra and statement version the default payload's.
    for member in DELIVERY_PROOF_CONCRETES {
        let mut payload = build_admit_selections_payload(Default::default());
        payload.selections.delivery_proof = declared_name(member);
        let admitted = admit_selections(&deps, params, payload)
            .expect("the member is admitted")
            .admitted;
        assert_eq!(
            admitted.delivery_proof,
            AdmittedDeliveryProof {
                concrete: *member,
                algebra: DECLARED_KEY_AGREEMENT_SELECTIONS[0].algebra,
                statement_version: 1,
            }
        );
    }

    // Act / Assert — every `PAIRING_CONCRETES` member crossed with every
    // `TARGET_GROUP_ENCODING_IDENTIFIERS` member, as a curve and as a
    // benchmark concrete.
    for concrete in PAIRING_CONCRETES {
        for target_group_encoding in TARGET_GROUP_ENCODING_IDENTIFIERS {
            let mut payload = build_admit_selections_payload(Default::default());
            payload.selections.curves = vec![build_curve_table(CurveTableOverrides {
                pairing_concrete: Some(declared_name(concrete)),
                target_group_encoding: Some(declared_name(target_group_encoding)),
                ..Default::default()
            })];
            payload.selections.benchmark_concretes = vec![build_benchmark_concrete_table(
                BenchmarkConcreteTableOverrides {
                    pairing_concrete: Some(declared_name(concrete)),
                    target_group_encoding: Some(declared_name(target_group_encoding)),
                },
            )];
            let admitted = admit_selections(&deps, params, payload)
                .expect("the member pair is admitted")
                .admitted;
            let expected_pairing = AdmittedPairing {
                concrete: *concrete,
                target_group_encoding: *target_group_encoding,
            };
            assert_eq!(
                admitted.curves,
                vec![AdmittedCurve {
                    label: "curve-a".to_string(),
                    pairing: expected_pairing,
                }]
            );
            assert_eq!(admitted.benchmark_concretes, vec![expected_pairing]);
        }
    }

    // Act / Assert — every member of `PRECOMPILE_ENCODINGS` in turn.
    for member in PRECOMPILE_ENCODINGS {
        let mut payload = build_admit_selections_payload(Default::default());
        payload.precompile_encodings = vec![declared_name(member)];
        let admitted = admit_selections(&deps, params, payload)
            .expect("the member is admitted")
            .admitted;
        assert_eq!(admitted.precompile_encodings, vec![*member]);
    }
    // every_member_of_every_declared_set_is_admitted_by_its_own_name
}

/// Contract: `a_mock_selections_name_is_never_admitted` → `admit_selections`
///   over a payload whose `random_source`, `encoding`, `chain_forms`, `kem`,
///   and `hash_to_scalar` texts are each the `Debug` text of the family's
///   mock selection returns `Err` carrying the refused family's variant and
///   that text.
/// Arrange: for each of `random_source`, `encoding`, `chain_forms`, `kem`,
///   and `hash_to_scalar` a payload whose text for that family is
///   `declared_name` of the family's mock selection and whose other texts
///   name real members.
/// Act:     `admit_selections`, once per payload.
/// Assert:  the result equals `Err` of that family's variant carrying the
///   text.
/// Boundary: the real `admit_selections`, `member_named`, and the families'
///   real declared sets.
/// Mocked:  none.
#[test]
fn a_mock_selections_name_is_never_admitted() {
    // Arrange — one case per family: the mock selection's `Debug` text, the
    // field it is written to, and the variant the refusal carries.
    type MockSelectionCase = (
        String,
        fn(&mut SelectionsTable, String),
        fn(String) -> AdmitSelectionsErrorReturn,
    );
    let cases: Vec<MockSelectionCase> = vec![
        (
            declared_name(&RandomSourceKind::Mock(
                MockIRandomSourceAdapterFailureMode::Succeeds,
            )),
            |selections, text| selections.random_source = text,
            |text| AdmitSelectionsErrorReturn::RandomSource { text },
        ),
        (
            declared_name(&EncodingConcrete::Mock),
            |selections, text| selections.encoding = text,
            |text| AdmitSelectionsErrorReturn::Encoding { text },
        ),
        (
            declared_name(&ChainFormsConcrete::Mock),
            |selections, text| selections.chain_forms = text,
            |text| AdmitSelectionsErrorReturn::ChainForms { text },
        ),
        (
            declared_name(&KemConcrete::Mock),
            |selections, text| selections.kem = text,
            |text| AdmitSelectionsErrorReturn::Kem { text },
        ),
        (
            declared_name(&HashToScalarConcrete::Mock(
                MockIHashToScalarAdapterFailureMode::Succeeds,
            )),
            |selections, text| selections.hash_to_scalar = text,
            |text| AdmitSelectionsErrorReturn::HashToScalar { text },
        ),
    ];
    for (text, set_text, variant) in cases {
        let mut payload = build_admit_selections_payload(Default::default());
        set_text(&mut payload.selections, text.clone());
        // Act
        let result = admit_selections(
            &build_admit_selections_deps(Default::default()),
            build_admit_selections_params(Default::default()),
            payload,
        );
        // Assert
        assert_eq!(result, Err(variant(text)));
    }
    // a_mock_selections_name_is_never_admitted
}
