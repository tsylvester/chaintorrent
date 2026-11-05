#![allow(clippy::expect_used)]

use domain::{
    ParameterSetIdentifierConstructorParamsOverrides, SuiteIdentifierConstructorParamsOverrides,
    build_parameter_set_identifier, build_suite_identifier,
};
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, DECLARED_FIXED_WIDTH_FIELD_KINDS,
    DECLARED_UNSIGNED_FIELD_KINDS,
};
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::IdentityScope;

use super::admit_statements;
use super::interface::AdmittedIdentity;
use super::mock::{
    AdmittedIdentitiesOverrides, build_admit_statements_deps, build_admit_statements_params,
    build_admit_statements_payload, build_admitted_identities, build_admitted_network_profile,
    build_admitted_statements, declared_name, representative_canonical_value,
};

/// Contract: Success → the default payload admits to the default admitted
///   network profile, statements, and identities.
/// Arrange: `build_admit_statements_payload(Default::default())`.
/// Act:     `admit_statements`, once.
/// Assert:  the result is `Ok`; its `network_profile` equals
///   `build_admitted_network_profile(Default::default())`; its
///   `statements.mint` and `transfer` equal the contexts
///   `build_admitted_statements(Default::default())` holds, and their
///   purposes' codes equal `1` and `2`; its `identities` equals
///   `build_admitted_identities(Default::default())`.
#[test]
fn admit_statements_admits_the_default_payload() {
    // Arrange
    let payload = build_admit_statements_payload(Default::default());
    let expected_profile = build_admitted_network_profile(Default::default());
    let expected_statements = build_admitted_statements(Default::default());
    let expected_identities = build_admitted_identities(Default::default());
    // Act
    let success = admit_statements(
        &build_admit_statements_deps(Default::default()),
        build_admit_statements_params(Default::default()),
        payload,
    )
    .expect("the admission succeeds");
    // Assert
    assert_eq!(success.network_profile, expected_profile);
    assert_eq!(success.statements.mint, expected_statements.mint);
    assert_eq!(success.statements.mint_purpose.code(), 1);
    assert_eq!(success.statements.transfer, expected_statements.transfer);
    assert_eq!(success.statements.transfer_purpose.code(), 2);
    assert_eq!(success.identities, expected_identities);
    // admit_statements_admits_the_default_payload
}

/// Contract: Network profile → a `0x` text of exactly the kind's width
///   admits to the kind's variant.
/// Arrange: for each member of `DECLARED_FIXED_WIDTH_FIELD_KINDS`, the
///   default payload with `chain_identifier` of that kind and the text and
///   variant `representative_canonical_value` returns.
/// Act:     `admit_statements`, once per kind.
/// Assert:  `network_profile.chain_identifier` equals the variant.
#[test]
fn admit_statements_admits_a_fixed_width_value_of_each_declared_kind() {
    // Arrange — the deps and params are invariant across every kind.
    let deps = build_admit_statements_deps(Default::default());
    let params = build_admit_statements_params(Default::default());
    // Act / Assert — every member of `DECLARED_FIXED_WIDTH_FIELD_KINDS` in turn.
    for kind in DECLARED_FIXED_WIDTH_FIELD_KINDS {
        let (text, variant) =
            representative_canonical_value(*kind).expect("the declared kind admits a value");
        let mut payload = build_admit_statements_payload(Default::default());
        payload.network_profile.chain_identifier.kind = declared_name(kind);
        payload.network_profile.chain_identifier.value = text;
        let admitted = admit_statements(&deps, params, payload).expect("the admission succeeds");
        assert_eq!(admitted.network_profile.chain_identifier, variant);
    }
    // admit_statements_admits_a_fixed_width_value_of_each_declared_kind
}

/// Contract: Hex digits → uppercase and lowercase digits name the same byte.
/// Arrange: the default payload with `entitlement_contract` of kind
///   `FixedBytes20` and the text `0x` followed by 20 bytes of `AB`.
/// Act:     `admit_statements`, once.
/// Assert:  `network_profile.entitlement_contract` equals
///   `CanonicalFieldValue::FixedBytes20([0xab; 20])`.
#[test]
fn admit_statements_admits_a_fixed_width_value_whatever_the_case_of_its_digits() {
    // Arrange
    let mut payload = build_admit_statements_payload(Default::default());
    payload.network_profile.entitlement_contract.kind =
        declared_name(&CanonicalFieldKind::FixedBytes20);
    payload.network_profile.entitlement_contract.value = format!("0x{}", "AB".repeat(20));
    // Act
    let admitted = admit_statements(
        &build_admit_statements_deps(Default::default()),
        build_admit_statements_params(Default::default()),
        payload,
    )
    .expect("the admission succeeds");
    // Assert
    assert_eq!(
        admitted.network_profile.entitlement_contract,
        CanonicalFieldValue::FixedBytes20([0xab; 20])
    );
    // admit_statements_admits_a_fixed_width_value_whatever_the_case_of_its_digits
}

/// Contract: Network profile → a decimal text admits to the kind's variant
///   over the integer.
/// Arrange: for each member of `DECLARED_UNSIGNED_FIELD_KINDS`, the default
///   payload with `chain_identifier` of that kind and the text and variant
///   `representative_canonical_value` returns.
/// Act:     `admit_statements`, once per kind.
/// Assert:  `network_profile.chain_identifier` equals the variant.
#[test]
fn admit_statements_admits_an_unsigned_value_of_each_declared_kind() {
    // Arrange — the deps and params are invariant across every kind.
    let deps = build_admit_statements_deps(Default::default());
    let params = build_admit_statements_params(Default::default());
    // Act / Assert — every member of `DECLARED_UNSIGNED_FIELD_KINDS` in turn.
    for kind in DECLARED_UNSIGNED_FIELD_KINDS {
        let (text, variant) =
            representative_canonical_value(*kind).expect("the declared kind admits a value");
        let mut payload = build_admit_statements_payload(Default::default());
        payload.network_profile.chain_identifier.kind = declared_name(kind);
        payload.network_profile.chain_identifier.value = text;
        let admitted = admit_statements(&deps, params, payload).expect("the admission succeeds");
        assert_eq!(admitted.network_profile.chain_identifier, variant);
    }
    // admit_statements_admits_an_unsigned_value_of_each_declared_kind
}

/// Contract: Range → zero and the kind's largest integer are admitted.
/// Arrange: the default payload with `chain_identifier` of kind `Unsigned16`
///   and text `"65535"`, then `Unsigned32` and `"4294967295"`, then
///   `Unsigned64` and `"18446744073709551615"`, then `Unsigned256` and the
///   256-bit maximum's decimal text, then `Unsigned256` and `"0"`.
/// Act:     `admit_statements`, once per payload.
/// Assert:  the admitted values equal `Unsigned16(u16::MAX)`,
///   `Unsigned32(u32::MAX)`, `Unsigned64(u64::MAX)`, `Unsigned256([0xff; 32])`,
///   and `Unsigned256([0x00; 32])`.
#[test]
fn admit_statements_admits_zero_and_the_maximum_of_each_unsigned_kind() {
    // Arrange — each case is the kind, its text, and the admitted variant.
    let cases: Vec<(CanonicalFieldKind, &str, CanonicalFieldValue)> = vec![
        (
            CanonicalFieldKind::Unsigned16,
            "65535",
            CanonicalFieldValue::Unsigned16(u16::MAX),
        ),
        (
            CanonicalFieldKind::Unsigned32,
            "4294967295",
            CanonicalFieldValue::Unsigned32(u32::MAX),
        ),
        (
            CanonicalFieldKind::Unsigned64,
            "18446744073709551615",
            CanonicalFieldValue::Unsigned64(u64::MAX),
        ),
        (
            CanonicalFieldKind::Unsigned256,
            "115792089237316195423570985008687907853269984665640564039457584007913129639935",
            CanonicalFieldValue::Unsigned256([0xff; 32]),
        ),
        (
            CanonicalFieldKind::Unsigned256,
            "0",
            CanonicalFieldValue::Unsigned256([0x00; 32]),
        ),
    ];
    let deps = build_admit_statements_deps(Default::default());
    let params = build_admit_statements_params(Default::default());
    for (kind, text, variant) in cases {
        let mut payload = build_admit_statements_payload(Default::default());
        payload.network_profile.chain_identifier.kind = declared_name(&kind);
        payload.network_profile.chain_identifier.value = text.to_string();
        // Act
        let admitted = admit_statements(&deps, params, payload).expect("the admission succeeds");
        // Assert
        assert_eq!(admitted.network_profile.chain_identifier, variant);
    }
    // admit_statements_admits_zero_and_the_maximum_of_each_unsigned_kind
}

/// Contract: Decimal → leading zeros do not change the integer.
/// Arrange: the default payload with `chain_identifier` of kind `Unsigned256`
///   and text `"0007"`.
/// Act:     `admit_statements`, once.
/// Assert:  the admitted value equals `Unsigned256` of 31 zero bytes then
///   `0x07`.
#[test]
fn admit_statements_admits_a_decimal_text_with_leading_zeros_as_its_integer() {
    // Arrange
    let mut payload = build_admit_statements_payload(Default::default());
    payload.network_profile.chain_identifier.kind = declared_name(&CanonicalFieldKind::Unsigned256);
    payload.network_profile.chain_identifier.value = "0007".to_string();
    let mut expected = [0u8; 32];
    expected[31] = 0x07;
    // Act
    let admitted = admit_statements(
        &build_admit_statements_deps(Default::default()),
        build_admit_statements_params(Default::default()),
        payload,
    )
    .expect("the admission succeeds");
    // Assert
    assert_eq!(
        admitted.network_profile.chain_identifier,
        CanonicalFieldValue::Unsigned256(expected)
    );
    // admit_statements_admits_a_decimal_text_with_leading_zeros_as_its_integer
}

/// Contract: Context assembly → `chain` and `entitlement_contract` of each
///   context are the network profile's admitted values.
/// Arrange: the default payload.
/// Act:     `admit_statements`, once.
/// Assert:  for `mint` and `transfer`, `chain` equals
///   `network_profile.chain_identifier` and `entitlement_contract` equals
///   `network_profile.entitlement_contract`.
#[test]
fn admit_statements_places_the_network_profile_in_every_context() {
    // Arrange
    let payload = build_admit_statements_payload(Default::default());
    let expected = build_admitted_network_profile(Default::default());
    // Act
    let admitted = admit_statements(
        &build_admit_statements_deps(Default::default()),
        build_admit_statements_params(Default::default()),
        payload,
    )
    .expect("the admission succeeds");
    // Assert
    for context in [&admitted.statements.mint, &admitted.statements.transfer] {
        assert_eq!(context.chain, expected.chain_identifier);
        assert_eq!(context.entitlement_contract, expected.entitlement_contract);
    }
    // admit_statements_places_the_network_profile_in_every_context
}

/// Contract: Context → the suite identifier with its narrowed version, the
///   root, and the parameter-set identifier are the domain values the texts
///   name.
/// Arrange: the default payload with the `transfer` table's
///   `suite_identifier` `0x` followed by 32 bytes each `0x55`, `suite_version`
///   `7`, `asset_root` 32 bytes each `0x56`, and `parameter_set_identifier`
///   32 bytes each `0x57`.
/// Act:     `admit_statements`, once.
/// Assert:  `statements.transfer` equals the default context with
///   `build_suite_identifier` of those bytes and version `7`, `build_root` of
///   the root bytes, and `build_parameter_set_identifier` of the set bytes,
///   and `statements.mint` still equals the default.
#[test]
fn admit_statements_admits_the_domain_values_of_each_context() {
    // Arrange
    let mut payload = build_admit_statements_payload(Default::default());
    payload.statements.transfer.suite_identifier = format!("0x{}", "55".repeat(32));
    payload.statements.transfer.suite_version = 7;
    payload.statements.transfer.asset_root = format!("0x{}", "56".repeat(32));
    payload.statements.transfer.parameter_set_identifier = format!("0x{}", "57".repeat(32));
    let mut expected_transfer = build_admitted_statements(Default::default()).transfer;
    expected_transfer.suite_identifier =
        build_suite_identifier(SuiteIdentifierConstructorParamsOverrides {
            identifier: Some([0x55; 32]),
            version: Some(7),
        });
    expected_transfer.asset_root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x56; 32]),
    });
    expected_transfer.parameter_set_identifier =
        build_parameter_set_identifier(ParameterSetIdentifierConstructorParamsOverrides {
            bytes: Some([0x57; 32]),
        });
    // Act
    let admitted = admit_statements(
        &build_admit_statements_deps(Default::default()),
        build_admit_statements_params(Default::default()),
        payload,
    )
    .expect("the admission succeeds");
    // Assert
    assert_eq!(admitted.statements.transfer, expected_transfer);
    assert_eq!(
        admitted.statements.mint,
        build_admitted_statements(Default::default()).mint
    );
    // admit_statements_admits_the_domain_values_of_each_context
}

/// Contract: Context → `expiry` is carried unchanged.
/// Arrange: the default payload with the `transfer` table's `expiry` `42`.
/// Act:     `admit_statements`, once.
/// Assert:  `statements.transfer.expiry` equals `42` and the mint context's
///   equals the default.
#[test]
fn admit_statements_carries_each_contexts_expiry_as_the_document_holds_it() {
    // Arrange
    let mut payload = build_admit_statements_payload(Default::default());
    payload.statements.transfer.expiry = 42;
    // Act
    let admitted = admit_statements(
        &build_admit_statements_deps(Default::default()),
        build_admit_statements_params(Default::default()),
        payload,
    )
    .expect("the admission succeeds");
    // Assert
    assert_eq!(admitted.statements.transfer.expiry, 42);
    assert_eq!(
        admitted.statements.mint.expiry,
        build_admitted_statements(Default::default()).mint.expiry
    );
    // admit_statements_carries_each_contexts_expiry_as_the_document_holds_it
}

/// Contract: Purposes → a mint context admits the mint and replacement codes
///   and a transfer context the transfer and grant codes.
/// Arrange: the default payload with `mint.purpose` `4` and `transfer.purpose`
///   `3`.
/// Act:     `admit_statements`, once.
/// Assert:  `mint_purpose.code()` equals `4` and `transfer_purpose.code()`
///   equals `3`.
#[test]
fn admit_statements_admits_each_relations_declared_purpose_codes() {
    // Arrange
    let mut payload = build_admit_statements_payload(Default::default());
    payload.statements.mint.purpose = 4;
    payload.statements.transfer.purpose = 3;
    // Act
    let admitted = admit_statements(
        &build_admit_statements_deps(Default::default()),
        build_admit_statements_params(Default::default()),
        payload,
    )
    .expect("the admission succeeds");
    // Assert
    assert_eq!(admitted.statements.mint_purpose.code(), 4);
    assert_eq!(admitted.statements.transfer_purpose.code(), 3);
    // admit_statements_admits_each_relations_declared_purpose_codes
}

/// Contract: Identities → under `IdentityScope::Asset` each text of 32 bytes
///   admits to `AdmittedIdentity::Asset` over a `Root`.
/// Arrange: the default payload with `scope` `IdentityScope::Asset` and
///   `identities` of `proof` 32 bytes each `0x71`, and `mapping` of 32 bytes
///   each `0x72` then 32 bytes each `0x73`.
/// Act:     `admit_statements`, once.
/// Assert:  `identities` equals `AdmittedIdentities` holding
///   `AdmittedIdentity::Asset` over `build_root` of those bytes, the mapping
///   in that order.
#[test]
fn admit_statements_admits_the_asset_scope_identities_as_roots() {
    // Arrange
    let mut payload = build_admit_statements_payload(Default::default());
    payload.scope = IdentityScope::Asset;
    payload.identities.proof = format!("0x{}", "71".repeat(32));
    payload.identities.mapping = vec![
        format!("0x{}", "72".repeat(32)),
        format!("0x{}", "73".repeat(32)),
    ];
    let expected = build_admitted_identities(AdmittedIdentitiesOverrides {
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
    });
    // Act
    let admitted = admit_statements(
        &build_admit_statements_deps(Default::default()),
        build_admit_statements_params(Default::default()),
        payload,
    )
    .expect("the admission succeeds");
    // Assert
    assert_eq!(admitted.identities, expected);
    // admit_statements_admits_the_asset_scope_identities_as_roots
}

/// Contract: Identities → under `IdentityScope::Entitlement` each text admits
///   to its decoded bytes whatever the length, an empty digit string included.
/// Arrange: the default payload with `identities` of `proof` `0x` and
///   `mapping` of `0x0102` and `0x`.
/// Act:     `admit_statements`, once.
/// Assert:  `identities` equals `AdmittedIdentity::Entitlement` over `vec![]`
///   and over `vec![0x01, 0x02]` and `vec![]`.
#[test]
fn admit_statements_admits_the_entitlement_scope_identities_as_canonical_bytes() {
    // Arrange
    let mut payload = build_admit_statements_payload(Default::default());
    payload.identities.proof = "0x".to_string();
    payload.identities.mapping = vec!["0x0102".to_string(), "0x".to_string()];
    let expected = build_admitted_identities(AdmittedIdentitiesOverrides {
        proof: Some(AdmittedIdentity::Entitlement {
            canonical: Vec::new(),
        }),
        mapping: Some(vec![
            AdmittedIdentity::Entitlement {
                canonical: vec![0x01, 0x02],
            },
            AdmittedIdentity::Entitlement {
                canonical: Vec::new(),
            },
        ]),
    });
    // Act
    let admitted = admit_statements(
        &build_admit_statements_deps(Default::default()),
        build_admit_statements_params(Default::default()),
        payload,
    )
    .expect("the admission succeeds");
    // Assert
    assert_eq!(admitted.identities, expected);
    // admit_statements_admits_the_entitlement_scope_identities_as_canonical_bytes
}

/// Contract: Identities → an empty mapping list admits to an empty list.
/// Arrange: the default payload with `mapping` empty.
/// Act:     `admit_statements`, once.
/// Assert:  `identities.mapping` is empty and `proof` equals the default's.
#[test]
fn admit_statements_admits_an_empty_mapping_list_as_empty() {
    // Arrange
    let mut payload = build_admit_statements_payload(Default::default());
    payload.identities.mapping = Vec::new();
    // Act
    let admitted = admit_statements(
        &build_admit_statements_deps(Default::default()),
        build_admit_statements_params(Default::default()),
        payload,
    )
    .expect("the admission succeeds");
    // Assert
    assert!(admitted.identities.mapping.is_empty());
    assert_eq!(
        admitted.identities.proof,
        build_admitted_identities(Default::default()).proof
    );
    // admit_statements_admits_an_empty_mapping_list_as_empty
}
