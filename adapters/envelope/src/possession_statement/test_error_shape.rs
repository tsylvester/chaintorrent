use super::provides::{
    PossessionG1StatementCorruptions, PossessionG2StatementCorruptions,
    PossessionStatementFromFieldsErrorReturn, build_possession_g1_statement_description,
    build_possession_g2_statement_description, invalidate_possession_g1_statement,
    invalidate_possession_g2_statement,
};
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFieldsOverrides, IEncodingContract,
    build_canonical_fields, build_from_fields_params,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingPayload, IPairingArithmetic,
    IPairingConsumer, IPairingReference, build_create_pairing_deps, build_create_pairing_params,
    create_pairing,
};

/// Contract: `fields_to_value`, wrong count: `payload.values` does not convert
///   into the two-field array → `Err(FieldCount { expected, actual })`,
///   `actual` the vector's length.
/// Arrange: `build_canonical_fields` overriding `values` with one
///   `CanonicalFieldValue::Bytes`, a value of the declared kind, so only the
///   count differs.
/// Act:     `description.fields_to_value(build_from_fields_params(
///   Default::default()), fields)` inside the consumer's method.
/// Assert:  in place, `assert_eq!(result.err(), Some(
///   PossessionStatementFromFieldsErrorReturn::FieldCount { expected: 2,
///   actual: 1 }))`, the field count stated as a literal; outside,
///   `created.is_ok()`.
/// Collaborators: the pairing family's mock concrete through `create_pairing`
///   under the default params.
#[test]
fn g1_fields_to_value_rejects_one_value_short() {
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let description =
                build_possession_g1_statement_description(&payload.adapter, Default::default());
            let fields = build_canonical_fields(CanonicalFieldsOverrides {
                values: Some(vec![CanonicalFieldValue::Bytes(vec![0x01])]),
            });
            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), fields);
            assert_eq!(
                result.err(),
                Some(PossessionStatementFromFieldsErrorReturn::FieldCount {
                    expected: 2,
                    actual: 1
                })
            );
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    assert!(created.is_ok());
}

/// Contract: `fields_to_value`, wrong count: `payload.values` does not convert
///   into the two-field array → `Err(FieldCount { expected, actual })`,
///   `actual` the vector's length.
/// Arrange: `build_canonical_fields` overriding `values` with one
///   `CanonicalFieldValue::Bytes`, a value of the declared kind, so only the
///   count differs.
/// Act:     `description.fields_to_value(build_from_fields_params(
///   Default::default()), fields)` inside the consumer's method.
/// Assert:  in place, `assert_eq!(result.err(), Some(
///   PossessionStatementFromFieldsErrorReturn::FieldCount { expected: 2,
///   actual: 1 }))`, the field count stated as a literal; outside,
///   `created.is_ok()`.
/// Collaborators: the pairing family's mock concrete through `create_pairing`
///   under the default params.
#[test]
fn g2_fields_to_value_rejects_one_value_short() {
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let description =
                build_possession_g2_statement_description(&payload.adapter, Default::default());
            let fields = build_canonical_fields(CanonicalFieldsOverrides {
                values: Some(vec![CanonicalFieldValue::Bytes(vec![0x01])]),
            });
            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), fields);
            assert_eq!(
                result.err(),
                Some(PossessionStatementFromFieldsErrorReturn::FieldCount {
                    expected: 2,
                    actual: 1
                })
            );
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    assert!(created.is_ok());
}

/// Contract: `fields_to_value`, wrong count: `payload.values` does not convert
///   into the two-field array → `Err(FieldCount { expected, actual })`,
///   `actual` the vector's length.
/// Arrange: `build_canonical_fields` overriding `values` with three
///   `CanonicalFieldValue::Bytes`, every value of the declared kind, so only
///   the count differs and `actual` is the vector's length.
/// Act:     `description.fields_to_value(build_from_fields_params(
///   Default::default()), fields)` inside the consumer's method.
/// Assert:  in place, `assert_eq!(result.err(), Some(
///   PossessionStatementFromFieldsErrorReturn::FieldCount { expected: 2,
///   actual: 3 }))`; outside, `created.is_ok()`.
/// Collaborators: the pairing family's mock concrete through `create_pairing`
///   under the default params.
#[test]
fn g1_fields_to_value_rejects_one_value_long() {
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let description =
                build_possession_g1_statement_description(&payload.adapter, Default::default());
            let fields = build_canonical_fields(CanonicalFieldsOverrides {
                values: Some(vec![
                    CanonicalFieldValue::Bytes(vec![0x01]),
                    CanonicalFieldValue::Bytes(vec![0x02]),
                    CanonicalFieldValue::Bytes(vec![0x03]),
                ]),
            });
            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), fields);
            assert_eq!(
                result.err(),
                Some(PossessionStatementFromFieldsErrorReturn::FieldCount {
                    expected: 2,
                    actual: 3
                })
            );
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    assert!(created.is_ok());
}

/// Contract: `fields_to_value`, wrong count: `payload.values` does not convert
///   into the two-field array → `Err(FieldCount { expected, actual })`,
///   `actual` the vector's length.
/// Arrange: `build_canonical_fields` overriding `values` with three
///   `CanonicalFieldValue::Bytes`, every value of the declared kind, so only
///   the count differs and `actual` is the vector's length.
/// Act:     `description.fields_to_value(build_from_fields_params(
///   Default::default()), fields)` inside the consumer's method.
/// Assert:  in place, `assert_eq!(result.err(), Some(
///   PossessionStatementFromFieldsErrorReturn::FieldCount { expected: 2,
///   actual: 3 }))`; outside, `created.is_ok()`.
/// Collaborators: the pairing family's mock concrete through `create_pairing`
///   under the default params.
#[test]
fn g2_fields_to_value_rejects_one_value_long() {
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let description =
                build_possession_g2_statement_description(&payload.adapter, Default::default());
            let fields = build_canonical_fields(CanonicalFieldsOverrides {
                values: Some(vec![
                    CanonicalFieldValue::Bytes(vec![0x01]),
                    CanonicalFieldValue::Bytes(vec![0x02]),
                    CanonicalFieldValue::Bytes(vec![0x03]),
                ]),
            });
            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), fields);
            assert_eq!(
                result.err(),
                Some(PossessionStatementFromFieldsErrorReturn::FieldCount {
                    expected: 2,
                    actual: 3
                })
            );
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    assert!(created.is_ok());
}

/// Contract: ordering: the count check precedes the kind check.
/// Arrange: `build_canonical_fields` overriding `values` with one
///   `CanonicalFieldValue::Text`, a payload both short and of the wrong kind,
///   so a kind check ahead of the count reports the kind.
/// Act:     `description.fields_to_value(build_from_fields_params(
///   Default::default()), fields)` inside the consumer's method.
/// Assert:  in place, `assert_eq!(result.err(), Some(
///   PossessionStatementFromFieldsErrorReturn::FieldCount { expected: 2,
///   actual: 1 }))`; outside, `created.is_ok()`.
/// Collaborators: the pairing family's mock concrete through `create_pairing`
///   under the default params.
#[test]
fn g1_fields_to_value_checks_the_count_before_any_kind() {
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let description =
                build_possession_g1_statement_description(&payload.adapter, Default::default());
            let fields = build_canonical_fields(CanonicalFieldsOverrides {
                values: Some(vec![CanonicalFieldValue::Text("not-bytes".to_string())]),
            });
            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), fields);
            assert_eq!(
                result.err(),
                Some(PossessionStatementFromFieldsErrorReturn::FieldCount {
                    expected: 2,
                    actual: 1
                })
            );
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    assert!(created.is_ok());
}

/// Contract: ordering: the count check precedes the kind check.
/// Arrange: `build_canonical_fields` overriding `values` with one
///   `CanonicalFieldValue::Text`, a payload both short and of the wrong kind,
///   so a kind check ahead of the count reports the kind.
/// Act:     `description.fields_to_value(build_from_fields_params(
///   Default::default()), fields)` inside the consumer's method.
/// Assert:  in place, `assert_eq!(result.err(), Some(
///   PossessionStatementFromFieldsErrorReturn::FieldCount { expected: 2,
///   actual: 1 }))`; outside, `created.is_ok()`.
/// Collaborators: the pairing family's mock concrete through `create_pairing`
///   under the default params.
#[test]
fn g2_fields_to_value_checks_the_count_before_any_kind() {
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let description =
                build_possession_g2_statement_description(&payload.adapter, Default::default());
            let fields = build_canonical_fields(CanonicalFieldsOverrides {
                values: Some(vec![CanonicalFieldValue::Text("not-bytes".to_string())]),
            });
            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), fields);
            assert_eq!(
                result.err(),
                Some(PossessionStatementFromFieldsErrorReturn::FieldCount {
                    expected: 2,
                    actual: 1
                })
            );
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    assert!(created.is_ok());
}

/// Contract: `fields_to_value`, wrong kind: the count matches and the value at
///   an index is not `CanonicalFieldValue::Bytes` → `Err(FieldKind { index,
///   expected: CanonicalFieldKind::Bytes })`.
/// Arrange: `invalidate_possession_g1_statement` with `commitment` corrupted
///   to `CanonicalFieldValue::Text`, the key left valid, so only index `1`
///   differs.
/// Act:     `description.fields_to_value(build_from_fields_params(
///   Default::default()), fields)` inside the consumer's method.
/// Assert:  in place, `assert_eq!(result.err(), Some(
///   PossessionStatementFromFieldsErrorReturn::FieldKind { index: 1,
///   expected: CanonicalFieldKind::Bytes }))`; outside, `created.is_ok()`.
/// Collaborators: the pairing family's mock concrete through `create_pairing`
///   under the default params.
#[test]
fn g1_fields_to_value_rejects_a_wrong_kind_at_the_commitment() {
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let description =
                build_possession_g1_statement_description(&payload.adapter, Default::default());
            let fields = invalidate_possession_g1_statement(
                &payload.adapter,
                PossessionG1StatementCorruptions {
                    commitment: Some(CanonicalFieldValue::Text("not-bytes".to_string())),
                    ..Default::default()
                },
            );
            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), fields);
            assert_eq!(
                result.err(),
                Some(PossessionStatementFromFieldsErrorReturn::FieldKind {
                    index: 1,
                    expected: CanonicalFieldKind::Bytes
                })
            );
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    assert!(created.is_ok());
}

/// Contract: `fields_to_value`, wrong kind: the count matches and the value at
///   an index is not `CanonicalFieldValue::Bytes` → `Err(FieldKind { index,
///   expected: CanonicalFieldKind::Bytes })`.
/// Arrange: `invalidate_possession_g2_statement` with `commitment` corrupted
///   to `CanonicalFieldValue::Text`, the key left valid, so only index `1`
///   differs.
/// Act:     `description.fields_to_value(build_from_fields_params(
///   Default::default()), fields)` inside the consumer's method.
/// Assert:  in place, `assert_eq!(result.err(), Some(
///   PossessionStatementFromFieldsErrorReturn::FieldKind { index: 1,
///   expected: CanonicalFieldKind::Bytes }))`; outside, `created.is_ok()`.
/// Collaborators: the pairing family's mock concrete through `create_pairing`
///   under the default params.
#[test]
fn g2_fields_to_value_rejects_a_wrong_kind_at_the_commitment() {
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let description =
                build_possession_g2_statement_description(&payload.adapter, Default::default());
            let fields = invalidate_possession_g2_statement(
                &payload.adapter,
                PossessionG2StatementCorruptions {
                    commitment: Some(CanonicalFieldValue::Text("not-bytes".to_string())),
                    ..Default::default()
                },
            );
            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), fields);
            assert_eq!(
                result.err(),
                Some(PossessionStatementFromFieldsErrorReturn::FieldKind {
                    index: 1,
                    expected: CanonicalFieldKind::Bytes
                })
            );
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    assert!(created.is_ok());
}

/// Contract: `fields_to_value`, wrong kind: more than one value is not
///   `CanonicalFieldValue::Bytes` → `Err(FieldKind { index, expected:
///   CanonicalFieldKind::Bytes })` for the lowest such index.
/// Arrange: `invalidate_possession_g1_statement` with `key` and `commitment`
///   both corrupted to `CanonicalFieldValue::Text`, so a check that runs from
///   the highest index reports index `1`.
/// Act:     `description.fields_to_value(build_from_fields_params(
///   Default::default()), fields)` inside the consumer's method.
/// Assert:  in place, `assert_eq!(result.err(), Some(
///   PossessionStatementFromFieldsErrorReturn::FieldKind { index: 0,
///   expected: CanonicalFieldKind::Bytes }))`; outside, `created.is_ok()`.
/// Collaborators: the pairing family's mock concrete through `create_pairing`
///   under the default params.
#[test]
fn g1_fields_to_value_reports_the_lowest_field_of_the_wrong_kind() {
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let description =
                build_possession_g1_statement_description(&payload.adapter, Default::default());
            let fields = invalidate_possession_g1_statement(
                &payload.adapter,
                PossessionG1StatementCorruptions {
                    key: Some(CanonicalFieldValue::Text("not-bytes".to_string())),
                    commitment: Some(CanonicalFieldValue::Text("not-bytes".to_string())),
                },
            );
            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), fields);
            assert_eq!(
                result.err(),
                Some(PossessionStatementFromFieldsErrorReturn::FieldKind {
                    index: 0,
                    expected: CanonicalFieldKind::Bytes
                })
            );
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    assert!(created.is_ok());
}

/// Contract: `fields_to_value`, wrong kind: more than one value is not
///   `CanonicalFieldValue::Bytes` → `Err(FieldKind { index, expected:
///   CanonicalFieldKind::Bytes })` for the lowest such index.
/// Arrange: `invalidate_possession_g2_statement` with `key` and `commitment`
///   both corrupted to `CanonicalFieldValue::Text`, so a check that runs from
///   the highest index reports index `1`.
/// Act:     `description.fields_to_value(build_from_fields_params(
///   Default::default()), fields)` inside the consumer's method.
/// Assert:  in place, `assert_eq!(result.err(), Some(
///   PossessionStatementFromFieldsErrorReturn::FieldKind { index: 0,
///   expected: CanonicalFieldKind::Bytes }))`; outside, `created.is_ok()`.
/// Collaborators: the pairing family's mock concrete through `create_pairing`
///   under the default params.
#[test]
fn g2_fields_to_value_reports_the_lowest_field_of_the_wrong_kind() {
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let description =
                build_possession_g2_statement_description(&payload.adapter, Default::default());
            let fields = invalidate_possession_g2_statement(
                &payload.adapter,
                PossessionG2StatementCorruptions {
                    key: Some(CanonicalFieldValue::Text("not-bytes".to_string())),
                    commitment: Some(CanonicalFieldValue::Text("not-bytes".to_string())),
                },
            );
            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), fields);
            assert_eq!(
                result.err(),
                Some(PossessionStatementFromFieldsErrorReturn::FieldKind {
                    index: 0,
                    expected: CanonicalFieldKind::Bytes
                })
            );
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    assert!(created.is_ok());
}
