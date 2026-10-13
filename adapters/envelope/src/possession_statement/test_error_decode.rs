use super::provides::{
    PossessionG1StatementCorruptions, PossessionG2StatementCorruptions,
    PossessionStatementFromFieldsErrorReturn, build_possession_g1_statement_description,
    build_possession_g2_statement_description, invalidate_possession_g1_statement,
    invalidate_possession_g2_statement,
};
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, IEncodingContract, build_from_fields_params,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingPayload, DecodeG1ErrorReturn,
    DecodeG2ErrorReturn, IPairingArithmetic, IPairingConsumer, IPairingReference,
    build_create_pairing_deps, build_create_pairing_params, create_pairing,
};

/// Contract: ordering: the kinds are checked in index order before any group
///   element decodes.
/// Arrange: `invalidate_possession_g1_statement` with `key` corrupted to
///   `CanonicalFieldValue::Bytes` of seven bytes, a length the pairing
///   refuses, and `commitment` corrupted to `CanonicalFieldValue::Text`, so a
///   decode ahead of the kind check reports the key's refusal.
/// Act:     `description.fields_to_value(build_from_fields_params(
///   Default::default()), fields)` inside the consumer's method.
/// Assert:  in place, `assert_eq!(result.err(), Some(
///   PossessionStatementFromFieldsErrorReturn::FieldKind { index: 1,
///   expected: CanonicalFieldKind::Bytes }))`; outside, `created.is_ok()`.
/// Collaborators: the pairing family's mock concrete through `create_pairing`
///   under the default params.
#[test]
fn g1_fields_to_value_checks_the_kinds_before_decoding_any_group_element() {
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
                    key: Some(CanonicalFieldValue::Bytes(vec![0x01; 7])),
                    commitment: Some(CanonicalFieldValue::Text("not-bytes".to_string())),
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

/// Contract: ordering: the kinds are checked in index order before any group
///   element decodes.
/// Arrange: `invalidate_possession_g2_statement` with `key` corrupted to
///   `CanonicalFieldValue::Bytes` of seven bytes, a length the pairing
///   refuses, and `commitment` corrupted to `CanonicalFieldValue::Text`, so a
///   decode ahead of the kind check reports the key's refusal.
/// Act:     `description.fields_to_value(build_from_fields_params(
///   Default::default()), fields)` inside the consumer's method.
/// Assert:  in place, `assert_eq!(result.err(), Some(
///   PossessionStatementFromFieldsErrorReturn::FieldKind { index: 1,
///   expected: CanonicalFieldKind::Bytes }))`; outside, `created.is_ok()`.
/// Collaborators: the pairing family's mock concrete through `create_pairing`
///   under the default params.
#[test]
fn g2_fields_to_value_checks_the_kinds_before_decoding_any_group_element() {
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
                    key: Some(CanonicalFieldValue::Bytes(vec![0x01; 7])),
                    commitment: Some(CanonicalFieldValue::Text("not-bytes".to_string())),
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

/// Contract: `fields_to_value`, admitted kinds: the commitment's `decode_g1`
///   refuses its bytes → `Err(PossessionStatementFromFieldsErrorReturn::
///   InvalidG1 { index, error })`, the decoder's refusal carried whole with
///   the field's index.
/// Arrange: `invalidate_possession_g1_statement` with `commitment` corrupted
///   to `CanonicalFieldValue::Bytes` of seven bytes and the key left valid, so
///   only the commitment is refused and a hard-coded index fails.
/// Act:     `description.fields_to_value(build_from_fields_params(
///   Default::default()), fields)` inside the consumer's method.
/// Assert:  in place, `assert_eq!(result.err(), Some(
///   PossessionStatementFromFieldsErrorReturn::InvalidG1 { index: 1,
///   error: DecodeG1ErrorReturn::WrongLength { expected: 8, actual: 7 } }))`,
///   the eight bytes the mock concrete's encoding states; outside,
///   `created.is_ok()`.
/// Collaborators: the pairing family's mock concrete through `create_pairing`
///   under the default params.
#[test]
fn g1_fields_to_value_rejects_a_commitment_the_pairing_refuses() {
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
                    commitment: Some(CanonicalFieldValue::Bytes(vec![0x01; 7])),
                    ..Default::default()
                },
            );
            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), fields);
            assert_eq!(
                result.err(),
                Some(PossessionStatementFromFieldsErrorReturn::InvalidG1 {
                    index: 1,
                    error: DecodeG1ErrorReturn::WrongLength {
                        expected: 8,
                        actual: 7
                    }
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

/// Contract: `fields_to_value`, admitted kinds: the commitment's `decode_g2`
///   refuses its bytes → `Err(PossessionStatementFromFieldsErrorReturn::
///   InvalidG2 { index, error })`, the decoder's refusal carried whole with
///   the field's index.
/// Arrange: `invalidate_possession_g2_statement` with `commitment` corrupted
///   to `CanonicalFieldValue::Bytes` of seven bytes and the key left valid, so
///   only the commitment is refused and a hard-coded index fails.
/// Act:     `description.fields_to_value(build_from_fields_params(
///   Default::default()), fields)` inside the consumer's method.
/// Assert:  in place, `assert_eq!(result.err(), Some(
///   PossessionStatementFromFieldsErrorReturn::InvalidG2 { index: 1,
///   error: DecodeG2ErrorReturn::WrongLength { expected: 8, actual: 7 } }))`,
///   the eight bytes the mock concrete's encoding states; outside,
///   `created.is_ok()`.
/// Collaborators: the pairing family's mock concrete through `create_pairing`
///   under the default params.
#[test]
fn g2_fields_to_value_rejects_a_commitment_the_pairing_refuses() {
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
                    commitment: Some(CanonicalFieldValue::Bytes(vec![0x01; 7])),
                    ..Default::default()
                },
            );
            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), fields);
            assert_eq!(
                result.err(),
                Some(PossessionStatementFromFieldsErrorReturn::InvalidG2 {
                    index: 1,
                    error: DecodeG2ErrorReturn::WrongLength {
                        expected: 8,
                        actual: 7
                    }
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

/// Contract: ordering: the key then the commitment decode, and the first
///   refusal is returned.
/// Arrange: `invalidate_possession_g1_statement` with `key` corrupted to
///   `CanonicalFieldValue::Bytes` of seven bytes and `commitment` to
///   `CanonicalFieldValue::Bytes` of six bytes, so the two refusals differ and
///   a decode that runs from the commitment, or stops at the second field,
///   reports the commitment's.
/// Act:     `description.fields_to_value(build_from_fields_params(
///   Default::default()), fields)` inside the consumer's method.
/// Assert:  in place, `assert_eq!(result.err(), Some(
///   PossessionStatementFromFieldsErrorReturn::InvalidG1 { index: 0,
///   error: DecodeG1ErrorReturn::WrongLength { expected: 8, actual: 7 } }))`;
///   outside, `created.is_ok()`.
/// Collaborators: the pairing family's mock concrete through `create_pairing`
///   under the default params.
#[test]
fn g1_fields_to_value_reports_the_first_refusal() {
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
                    key: Some(CanonicalFieldValue::Bytes(vec![0x01; 7])),
                    commitment: Some(CanonicalFieldValue::Bytes(vec![0x02; 6])),
                },
            );
            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), fields);
            assert_eq!(
                result.err(),
                Some(PossessionStatementFromFieldsErrorReturn::InvalidG1 {
                    index: 0,
                    error: DecodeG1ErrorReturn::WrongLength {
                        expected: 8,
                        actual: 7
                    }
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

/// Contract: ordering: the key then the commitment decode, and the first
///   refusal is returned.
/// Arrange: `invalidate_possession_g2_statement` with `key` corrupted to
///   `CanonicalFieldValue::Bytes` of seven bytes and `commitment` to
///   `CanonicalFieldValue::Bytes` of six bytes, so the two refusals differ and
///   a decode that runs from the commitment, or stops at the second field,
///   reports the commitment's.
/// Act:     `description.fields_to_value(build_from_fields_params(
///   Default::default()), fields)` inside the consumer's method.
/// Assert:  in place, `assert_eq!(result.err(), Some(
///   PossessionStatementFromFieldsErrorReturn::InvalidG2 { index: 0,
///   error: DecodeG2ErrorReturn::WrongLength { expected: 8, actual: 7 } }))`;
///   outside, `created.is_ok()`.
/// Collaborators: the pairing family's mock concrete through `create_pairing`
///   under the default params.
#[test]
fn g2_fields_to_value_reports_the_first_refusal() {
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
                    key: Some(CanonicalFieldValue::Bytes(vec![0x01; 7])),
                    commitment: Some(CanonicalFieldValue::Bytes(vec![0x02; 6])),
                },
            );
            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), fields);
            assert_eq!(
                result.err(),
                Some(PossessionStatementFromFieldsErrorReturn::InvalidG2 {
                    index: 0,
                    error: DecodeG2ErrorReturn::WrongLength {
                        expected: 8,
                        actual: 7
                    }
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
