#![allow(clippy::expect_used)]

use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, DECLARED_FIXED_WIDTH_FIELD_KINDS,
    DECLARED_UNSIGNED_FIELD_KINDS,
};
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::IdentityScope;

use super::admit_statements;
use super::interface::{
    AdmitStatementsErrorReturn, AdmittedIdentity, CanonicalValueRefusal, NetworkProfileField,
};
use super::mock::{
    AdmittedIdentitiesOverrides, build_admit_statements_deps, build_admit_statements_params,
    build_admit_statements_payload, build_admitted_identities, declared_name,
    representative_canonical_value,
};

/// Contract: `every_declared_kind_admits_its_value_text_to_its_own_variant` →
///   `admit_statements` over a payload whose canonical values name, in turn,
///   a member of `DECLARED_FIXED_WIDTH_FIELD_KINDS` or of
///   `DECLARED_UNSIGNED_FIELD_KINDS` by its `Debug` text with a value text
///   the kind admits returns `Ok` whose admitted value equals the variant of
///   that kind over the bytes or the integer the text names.
/// Arrange: for each member of both declared sets a payload from
///   `build_admit_statements_payload` whose `chain_identifier` kind text is
///   `declared_name` of the member and whose value text and expected variant
///   are those `representative_canonical_value` returns, then the same in a
///   context's `source_entitlement`.
/// Act:     `admit_statements`, once per payload.
/// Assert:  the result is `Ok` and the admitted value equals the expected
///   variant.
/// Boundary: the real `admit_statements`, `kind_named`, `canonical_value`,
///   and the hex and decimal functions over the encoding family's real
///   declared sets.
/// Mocked:  none.
#[test]
fn every_declared_kind_admits_its_value_text_to_its_own_variant() {
    // Arrange — the deps and params are invariant across every member.
    let deps = build_admit_statements_deps(Default::default());
    let params = build_admit_statements_params(Default::default());

    // Act / Assert — every member of both declared sets in `chain_identifier`.
    for kind in DECLARED_FIXED_WIDTH_FIELD_KINDS
        .iter()
        .chain(DECLARED_UNSIGNED_FIELD_KINDS)
    {
        let (text, variant) =
            representative_canonical_value(*kind).expect("the declared kind admits a value");
        let mut payload = build_admit_statements_payload(Default::default());
        payload.network_profile.chain_identifier.kind = declared_name(kind);
        payload.network_profile.chain_identifier.value = text;
        let admitted = admit_statements(&deps, params, payload).expect("the member is admitted");
        assert_eq!(admitted.network_profile.chain_identifier, variant);
    }

    // Act / Assert — every member of both declared sets in a context's
    // `source_entitlement`.
    for kind in DECLARED_FIXED_WIDTH_FIELD_KINDS
        .iter()
        .chain(DECLARED_UNSIGNED_FIELD_KINDS)
    {
        let (text, variant) =
            representative_canonical_value(*kind).expect("the declared kind admits a value");
        let mut payload = build_admit_statements_payload(Default::default());
        payload.statements.mint.source_entitlement.kind = declared_name(kind);
        payload.statements.mint.source_entitlement.value = text;
        let admitted = admit_statements(&deps, params, payload).expect("the member is admitted");
        assert_eq!(admitted.statements.mint.source_entitlement, variant);
    }
    // every_declared_kind_admits_its_value_text_to_its_own_variant
}

/// Contract: `a_kind_outside_the_declared_sets_is_never_admitted` →
///   `admit_statements` over a payload whose `chain_identifier` kind text is
///   the `Debug` text of `CanonicalFieldKind::Text` and then of
///   `CanonicalFieldKind::Bytes`, and a text no variant is named by, returns
///   `Err(AdmitStatementsErrorReturn::NetworkProfile { field:
///   NetworkProfileField::ChainIdentifier, refusal:
///   CanonicalValueRefusal::UnknownKind { text } })`.
/// Arrange: a payload per text — the `Debug` text of `Text`, of `Bytes`, and
///   `"unknown"`.
/// Act:     `admit_statements`, once per payload.
/// Assert:  the result's error equals the `NetworkProfile` refusal at
///   `ChainIdentifier` with `UnknownKind` carrying the text.
/// Boundary: the real `admit_statements`, `kind_named`, `canonical_value`,
///   and the hex and decimal functions over the encoding family's real
///   declared sets.
/// Mocked:  none.
#[test]
fn a_kind_outside_the_declared_sets_is_never_admitted() {
    // Arrange — one payload per kind text outside the declared sets.
    let texts = [
        declared_name(&CanonicalFieldKind::Text),
        declared_name(&CanonicalFieldKind::Bytes),
        "unknown".to_string(),
    ];
    let deps = build_admit_statements_deps(Default::default());
    let params = build_admit_statements_params(Default::default());
    for text in texts {
        let mut payload = build_admit_statements_payload(Default::default());
        payload.network_profile.chain_identifier.kind = text.clone();
        // Act
        let result = admit_statements(&deps, params, payload);
        // Assert
        assert_eq!(
            result.err(),
            Some(AdmitStatementsErrorReturn::NetworkProfile {
                field: NetworkProfileField::ChainIdentifier,
                refusal: CanonicalValueRefusal::UnknownKind { text }
            })
        );
    }
    // a_kind_outside_the_declared_sets_is_never_admitted
}

/// Contract: `every_context_carries_the_network_profiles_chain_identifier_and_entitlement_contract`
///   → `admit_statements` over a payload whose network profile holds a chain
///   identifier and an entitlement contract that differ from every context
///   value returns `Ok` whose mint and transfer contexts each hold the
///   admitted `chain_identifier` as `chain` and the admitted
///   `entitlement_contract`.
/// Arrange: the default payload with `chain_identifier` `"7"` and
///   `entitlement_contract` `0x` followed by 20 bytes each `0x99`.
/// Act:     `admit_statements`, once.
/// Assert:  `chain` and `entitlement_contract` of `mint` and `transfer` equal
///   the admitted network profile's.
/// Boundary: the real `admit_statements`, `kind_named`, `canonical_value`,
///   and the hex and decimal functions over the encoding family's real
///   declared sets.
/// Mocked:  none.
#[test]
fn every_context_carries_the_network_profiles_chain_identifier_and_entitlement_contract() {
    // Arrange — values that differ from every context value.
    let mut payload = build_admit_statements_payload(Default::default());
    payload.network_profile.chain_identifier.value = "7".to_string();
    payload.network_profile.entitlement_contract.value = format!("0x{}", "99".repeat(20));
    let mut chain = [0u8; 32];
    chain[31] = 0x07;
    let expected_chain = CanonicalFieldValue::Unsigned256(chain);
    let expected_contract = CanonicalFieldValue::FixedBytes20([0x99; 20]);
    // Act
    let admitted = admit_statements(
        &build_admit_statements_deps(Default::default()),
        build_admit_statements_params(Default::default()),
        payload,
    )
    .expect("the admission succeeds");
    // Assert
    for context in [&admitted.statements.mint, &admitted.statements.transfer] {
        assert_eq!(context.chain, expected_chain);
        assert_eq!(context.entitlement_contract, expected_contract);
    }
    // every_context_carries_the_network_profiles_chain_identifier_and_entitlement_contract
}

/// Contract: `identities_are_admitted_under_the_scope_the_payload_names` →
///   `admit_statements` over a payload whose identities are 32-byte hex texts
///   and whose `scope` is `IdentityScope::Asset`, and over identities of
///   differing lengths with `IdentityScope::Entitlement`, returns the asset
///   scope's `AdmittedIdentity::Asset` over a `Root` holding the text's bytes
///   for the proof and each mapping entry in the document's order, and the
///   entitlement scope's `AdmittedIdentity::Entitlement` over the decoded
///   bytes.
/// Arrange: a payload per scope — the asset payloads holding 32-byte
///   identities and an empty mapping list, the entitlement payload holding
///   identities of differing lengths and an empty digit string.
/// Act:     `admit_statements`, once per payload.
/// Assert:  the result is `Ok` and each admitted identity equals the shape
///   its scope names over the bytes the text holds, the mapping in order.
/// Boundary: the real `admit_statements`, `kind_named`, `canonical_value`,
///   `fixed_from_hex`, `bytes_from_hex`, and `Root::try_new` over the real
///   declared sets.
/// Mocked:  none.
#[test]
fn identities_are_admitted_under_the_scope_the_payload_names() {
    // Arrange — the deps and params are invariant across every payload.
    let deps = build_admit_statements_deps(Default::default());
    let params = build_admit_statements_params(Default::default());

    // Asset scope — a proof and two mapping entries of 32 bytes, in order.
    let mut asset = build_admit_statements_payload(Default::default());
    asset.scope = IdentityScope::Asset;
    asset.identities.proof = format!("0x{}", "71".repeat(32));
    asset.identities.mapping = vec![
        format!("0x{}", "72".repeat(32)),
        format!("0x{}", "73".repeat(32)),
    ];
    // Act
    let admitted = admit_statements(&deps, params, asset).expect("the admission succeeds");
    // Assert
    assert_eq!(
        admitted.identities,
        build_admitted_identities(AdmittedIdentitiesOverrides {
            proof: Some(AdmittedIdentity::Asset {
                root: build_root(RootConstructorParamsOverrides {
                    bytes: Some([0x71; 32]),
                }),
            }),
            mapping: Some(vec![
                AdmittedIdentity::Asset {
                    root: build_root(RootConstructorParamsOverrides {
                        bytes: Some([0x72; 32]),
                    }),
                },
                AdmittedIdentity::Asset {
                    root: build_root(RootConstructorParamsOverrides {
                        bytes: Some([0x73; 32]),
                    }),
                },
            ]),
        })
    );

    // Asset scope — an empty mapping list.
    let mut empty_mapping = build_admit_statements_payload(Default::default());
    empty_mapping.scope = IdentityScope::Asset;
    empty_mapping.identities.proof = format!("0x{}", "71".repeat(32));
    empty_mapping.identities.mapping = Vec::new();
    // Act
    let admitted = admit_statements(&deps, params, empty_mapping).expect("the admission succeeds");
    // Assert
    assert!(admitted.identities.mapping.is_empty());

    // Entitlement scope — identities of differing lengths and an empty digit
    // string.
    let mut entitlement = build_admit_statements_payload(Default::default());
    entitlement.identities.proof = "0xaa01".to_string();
    entitlement.identities.mapping = vec!["0x0102".to_string(), "0x".to_string()];
    // Act
    let admitted = admit_statements(&deps, params, entitlement).expect("the admission succeeds");
    // Assert
    assert_eq!(
        admitted.identities,
        build_admitted_identities(AdmittedIdentitiesOverrides {
            proof: Some(AdmittedIdentity::Entitlement {
                canonical: vec![0xaa, 0x01],
            }),
            mapping: Some(vec![
                AdmittedIdentity::Entitlement {
                    canonical: vec![0x01, 0x02],
                },
                AdmittedIdentity::Entitlement { canonical: vec![] },
            ]),
        })
    );
    // identities_are_admitted_under_the_scope_the_payload_names
}

/// Contract: `every_curve_has_its_own_deployment_record_in_curve_order` →
///   `admit_statements` over a payload whose `curve_labels` are two labels and
///   whose `deployment_records` holds one distinct `FixedBytes20` record under
///   each returns `Ok` whose `network_profile.deployment_records` equals the
///   two values in curve order.
/// Arrange: the default payload, its `curve_labels` given in the reverse of
///   the records' key order, so a key-order reading differs from the curve
///   order the entry states.
/// Act:     `admit_statements`, once.
/// Assert:  the result is `Ok` and `deployment_records` equals the two
///   `FixedBytes20` values in curve order.
/// Boundary: the real `admit_statements`, `kind_named`, `canonical_value`,
///   and `fixed_from_hex` over the encoding family's real declared sets.
/// Mocked:  none.
#[test]
fn every_curve_has_its_own_deployment_record_in_curve_order() {
    // Arrange
    let mut payload = build_admit_statements_payload(Default::default());
    payload.curve_labels = vec!["curve-b".to_string(), "curve-a".to_string()];
    // Act
    let admitted = admit_statements(
        &build_admit_statements_deps(Default::default()),
        build_admit_statements_params(Default::default()),
        payload,
    )
    .expect("the admission succeeds");
    // Assert
    assert_eq!(
        admitted.network_profile.deployment_records,
        vec![
            CanonicalFieldValue::FixedBytes20([0x24; 20]),
            CanonicalFieldValue::FixedBytes20([0x22; 20]),
        ]
    );
    // every_curve_has_its_own_deployment_record_in_curve_order
}

/// Contract: `a_curve_without_a_record_and_a_record_without_a_curve_are_each_refused_with_the_label`
///   → `admit_statements` over a payload with a curve label whose record is
///   absent returns `Err(MissingDeploymentRecord { label })`, and over a
///   payload whose `deployment_records` holds a key no curve label names
///   returns `Err(UndeclaredDeploymentRecord { label })`, each carrying that
///   label.
/// Arrange: the default payload with `curve-c` appended to `curve_labels`,
///   then the default payload with a `curve-c` record inserted and no such
///   label.
/// Act:     `admit_statements`, once per payload.
/// Assert:  each result's error equals the refusal carrying `curve-c`.
/// Boundary: the real `admit_statements`, `kind_named`, and `canonical_value`
///   over the encoding family's real declared sets.
/// Mocked:  none.
#[test]
fn a_curve_without_a_record_and_a_record_without_a_curve_are_each_refused_with_the_label() {
    // Arrange — a curve label whose record is absent.
    let mut missing = build_admit_statements_payload(Default::default());
    missing.curve_labels = vec!["curve-a".to_string(), "curve-c".to_string()];
    // Act
    let result = admit_statements(
        &build_admit_statements_deps(Default::default()),
        build_admit_statements_params(Default::default()),
        missing,
    );
    // Assert
    assert_eq!(
        result.err(),
        Some(AdmitStatementsErrorReturn::MissingDeploymentRecord {
            label: "curve-c".to_string()
        })
    );

    // Arrange — a record key no curve label names.
    let mut undeclared = build_admit_statements_payload(Default::default());
    undeclared.network_profile.deployment_records.insert(
        "curve-c".to_string(),
        undeclared.network_profile.entitlement_contract.clone(),
    );
    // Act
    let result = admit_statements(
        &build_admit_statements_deps(Default::default()),
        build_admit_statements_params(Default::default()),
        undeclared,
    );
    // Assert
    assert_eq!(
        result.err(),
        Some(AdmitStatementsErrorReturn::UndeclaredDeploymentRecord {
            label: "curve-c".to_string()
        })
    );
    // a_curve_without_a_record_and_a_record_without_a_curve_are_each_refused_with_the_label
}
