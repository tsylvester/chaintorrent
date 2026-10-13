#![allow(clippy::expect_used)]

// Reference fields, the expectations' source, stated as literals and never
// read from a builder or from the pairing: the pairing mock concrete's
// first-group generator is residue `1` and its second-group generator residue
// `2`, `add_g1` and `add_g2` add residues modulo the group's modulus, and a
// point's encoding is its residue as eight big-endian bytes; the first
// group's reference fields are `Bytes` of the encoding of residue `1` then
// `Bytes` of the encoding of that generator added to itself, and the second
// group's are `Bytes` of the encoding of residue `2` then `Bytes` of the
// encoding of that generator added to itself.

use super::provides::{
    PossessionG1StatementDescription, PossessionG2StatementDescription,
    build_possession_g1_statement, build_possession_g1_statement_description,
    build_possession_g1_statement_description_constructor_params, build_possession_g2_statement,
    build_possession_g2_statement_description,
    build_possession_g2_statement_description_constructor_params,
    invalidate_possession_g1_statement, invalidate_possession_g2_statement,
};
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFieldsOverrides, IEncodingContract,
    build_canonical_fields, build_from_fields_params, build_to_fields_params,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingPayload, EncodeG1Params,
    EncodeG1PayloadOverrides, EncodeG2Params, EncodeG2PayloadOverrides, IPairingArithmetic,
    IPairingConsumer, IPairingReference, build_create_pairing_deps, build_create_pairing_params,
    build_encode_g1_payload, build_encode_g2_payload, create_pairing,
};

/// Contract: `try_new` stores the supplied `&P` and returns `Ok`.
/// Arrange: the params from
///   `build_possession_g1_statement_description_constructor_params(
///   &payload.adapter, Default::default())`, borrowing the adapter the
///   consumer received.
/// Act:     `PossessionG1StatementDescription::try_new(params)` inside the
///   consumer's method.
/// Assert:  in place, the result is `Ok` and the description's `pairing` is
///   `payload.adapter` by `core::ptr::eq`; outside, `created.is_ok()`.
/// Collaborators: the pairing family's mock concrete through `create_pairing`
///   under the default params.
#[test]
fn g1_try_new_stores_the_supplied_pairing() {
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let params = build_possession_g1_statement_description_constructor_params(
                &payload.adapter,
                Default::default(),
            );
            let description = PossessionG1StatementDescription::try_new(params);
            assert!(description.is_ok());
            let Ok(description) = description;
            assert!(core::ptr::eq(description.pairing, &payload.adapter));
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    assert!(created.is_ok());
}

/// Contract: `try_new` stores the supplied `&P` and returns `Ok`.
/// Arrange: the params from
///   `build_possession_g2_statement_description_constructor_params(
///   &payload.adapter, Default::default())`, borrowing the adapter the
///   consumer received.
/// Act:     `PossessionG2StatementDescription::try_new(params)` inside the
///   consumer's method.
/// Assert:  in place, the result is `Ok` and the description's `pairing` is
///   `payload.adapter` by `core::ptr::eq`; outside, `created.is_ok()`.
/// Collaborators: the pairing family's mock concrete through `create_pairing`
///   under the default params.
#[test]
fn g2_try_new_stores_the_supplied_pairing() {
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let params = build_possession_g2_statement_description_constructor_params(
                &payload.adapter,
                Default::default(),
            );
            let description = PossessionG2StatementDescription::try_new(params);
            assert!(description.is_ok());
            let Ok(description) = description;
            assert!(core::ptr::eq(description.pairing, &payload.adapter));
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    assert!(created.is_ok());
}

/// Contract: `PossessionG1StatementDescription` implements `IEncodingContract`
///   for `PossessionG1Statement` with `FIELDS` the key's encoding then the
///   commitment's encoding, each a byte string.
/// Arrange: the description's type received through the consumer; the read
///   names no instance.
/// Act:     `<PossessionG1StatementDescription<'_, P> as
///   IEncodingContract>::FIELDS` inside the consumer's method, returned whole.
/// Assert:  `assert_eq!` of the output against `[CanonicalFieldKind::Bytes,
///   CanonicalFieldKind::Bytes]`; `created.is_ok()`, proving the consumer ran.
/// Collaborators: the pairing family's mock concrete through `create_pairing`
///   under the default params.
#[test]
fn g1_fields_declares_the_key_and_the_commitment_as_byte_strings() {
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = &'static [CanonicalFieldKind];

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            _payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            <PossessionG1StatementDescription<'_, P> as IEncodingContract>::FIELDS
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    assert!(created.is_ok());
    assert_eq!(
        created
            .expect("the pairing factory ran the consumer")
            .output,
        [CanonicalFieldKind::Bytes, CanonicalFieldKind::Bytes]
    );
}

/// Contract: `PossessionG2StatementDescription` implements `IEncodingContract`
///   for `PossessionG2Statement` with `FIELDS` the key's encoding then the
///   commitment's encoding, each a byte string.
/// Arrange: the description's type received through the consumer; the read
///   names no instance.
/// Act:     `<PossessionG2StatementDescription<'_, P> as
///   IEncodingContract>::FIELDS` inside the consumer's method, returned whole.
/// Assert:  `assert_eq!` of the output against `[CanonicalFieldKind::Bytes,
///   CanonicalFieldKind::Bytes]`; `created.is_ok()`, proving the consumer ran.
/// Collaborators: the pairing family's mock concrete through `create_pairing`
///   under the default params.
#[test]
fn g2_fields_declares_the_key_and_the_commitment_as_byte_strings() {
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = &'static [CanonicalFieldKind];

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            _payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            <PossessionG2StatementDescription<'_, P> as IEncodingContract>::FIELDS
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    assert!(created.is_ok());
    assert_eq!(
        created
            .expect("the pairing factory ran the consumer")
            .output,
        [CanonicalFieldKind::Bytes, CanonicalFieldKind::Bytes]
    );
}

/// Contract: `to_fields` encodes the key then the commitment through the
///   selected pairing's `encode_g1` and copies each encoding's bytes into
///   `CanonicalFieldValue::Bytes`.
/// Arrange: the description from
///   `build_possession_g1_statement_description(&payload.adapter,
///   Default::default())` and the statement from
///   `build_possession_g1_statement(&payload.adapter, Default::default())`,
///   whose key and commitment differ, so a swapped field, a repeated field, or
///   a field of another kind changes the fields.
/// Act:     `description.to_fields(build_to_fields_params(Default::default()),
///   &statement)` inside the consumer's method, returned whole.
/// Assert:  the success return's `fields` equal `build_canonical_fields` whose
///   `values` are the first group's reference fields — `Bytes` of the
///   encoding of residue `1`, then `Bytes` of the encoding of residue `2` —
///   by `assert_eq!`; `created.is_ok()`, proving the consumer ran.
/// Collaborators: the pairing family's mock concrete through `create_pairing`
///   under the default params.
#[test]
fn g1_to_fields_lists_the_key_then_the_commitment() {
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = encoding::ToFieldsReturn;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let description =
                build_possession_g1_statement_description(&payload.adapter, Default::default());
            let statement = build_possession_g1_statement(&payload.adapter, Default::default());
            description.to_fields(build_to_fields_params(Default::default()), &statement)
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    assert!(created.is_ok());
    let Ok(success) = created
        .expect("the pairing factory ran the consumer")
        .output;
    assert_eq!(
        success.fields,
        build_canonical_fields(CanonicalFieldsOverrides {
            values: Some(vec![
                CanonicalFieldValue::Bytes(vec![0, 0, 0, 0, 0, 0, 0, 1]),
                CanonicalFieldValue::Bytes(vec![0, 0, 0, 0, 0, 0, 0, 2]),
            ]),
        })
    );
}

/// Contract: `to_fields` encodes the key then the commitment through the
///   selected pairing's `encode_g2` and copies each encoding's bytes into
///   `CanonicalFieldValue::Bytes`.
/// Arrange: the description from
///   `build_possession_g2_statement_description(&payload.adapter,
///   Default::default())` and the statement from
///   `build_possession_g2_statement(&payload.adapter, Default::default())`,
///   whose key and commitment differ, so a swapped field, a repeated field, or
///   a field of another kind changes the fields.
/// Act:     `description.to_fields(build_to_fields_params(Default::default()),
///   &statement)` inside the consumer's method, returned whole.
/// Assert:  the success return's `fields` equal `build_canonical_fields` whose
///   `values` are the second group's reference fields — `Bytes` of the
///   encoding of residue `2`, then `Bytes` of the encoding of residue `4` —
///   by `assert_eq!`; `created.is_ok()`, proving the consumer ran.
/// Collaborators: the pairing family's mock concrete through `create_pairing`
///   under the default params.
#[test]
fn g2_to_fields_lists_the_key_then_the_commitment() {
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = encoding::ToFieldsReturn;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let description =
                build_possession_g2_statement_description(&payload.adapter, Default::default());
            let statement = build_possession_g2_statement(&payload.adapter, Default::default());
            description.to_fields(build_to_fields_params(Default::default()), &statement)
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    assert!(created.is_ok());
    let Ok(success) = created
        .expect("the pairing factory ran the consumer")
        .output;
    assert_eq!(
        success.fields,
        build_canonical_fields(CanonicalFieldsOverrides {
            values: Some(vec![
                CanonicalFieldValue::Bytes(vec![0, 0, 0, 0, 0, 0, 0, 2]),
                CanonicalFieldValue::Bytes(vec![0, 0, 0, 0, 0, 0, 0, 4]),
            ]),
        })
    );
}

/// Contract: `fields_to_value`, admitted: after the count and kind checks, the
///   key then the commitment decode through the selected pairing's `decode_g1`
///   and move into the trusted statement.
/// Arrange: the description; the fields from
///   `invalidate_possession_g1_statement(&payload.adapter, Default::default())`,
///   the valid rendering of a statement whose key and commitment differ, so a
///   swapped or repeated field changes the decoded statement.
/// Act:     `description.fields_to_value(build_from_fields_params(
///   Default::default()), fields)` inside the consumer's method.
/// Assert:  in place, the result is `Ok` and the decoded key and commitment,
///   each encoded through the received pairing's `encode_g1` with
///   `build_encode_g1_payload` carrying the point, equal in order the bytes of
///   the first group's reference fields — residue `1`'s encoding, then
///   residue `2`'s — by `assert_eq!`; outside, `created.is_ok()`.
/// Collaborators: the pairing family's mock concrete through `create_pairing`
///   under the default params.
#[test]
fn g1_fields_to_value_returns_the_statement_the_fields_describe() {
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
            let fields = invalidate_possession_g1_statement(&payload.adapter, Default::default());
            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), fields);
            assert!(result.is_ok());
            let success = result.expect("the reference fields decode");
            let Ok(key) = payload.adapter.encode_g1(
                EncodeG1Params,
                build_encode_g1_payload(
                    &payload.adapter,
                    EncodeG1PayloadOverrides {
                        point: Some(success.described.key),
                    },
                ),
            );
            let Ok(commitment) = payload.adapter.encode_g1(
                EncodeG1Params,
                build_encode_g1_payload(
                    &payload.adapter,
                    EncodeG1PayloadOverrides {
                        point: Some(success.described.commitment),
                    },
                ),
            );
            assert_eq!(key.bytes.as_ref(), [0, 0, 0, 0, 0, 0, 0, 1]);
            assert_eq!(commitment.bytes.as_ref(), [0, 0, 0, 0, 0, 0, 0, 2]);
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    assert!(created.is_ok());
}

/// Contract: `fields_to_value`, admitted: after the count and kind checks, the
///   key then the commitment decode through the selected pairing's `decode_g2`
///   and move into the trusted statement.
/// Arrange: the description; the fields from
///   `invalidate_possession_g2_statement(&payload.adapter, Default::default())`,
///   the valid rendering of a statement whose key and commitment differ, so a
///   swapped or repeated field changes the decoded statement.
/// Act:     `description.fields_to_value(build_from_fields_params(
///   Default::default()), fields)` inside the consumer's method.
/// Assert:  in place, the result is `Ok` and the decoded key and commitment,
///   each encoded through the received pairing's `encode_g2` with
///   `build_encode_g2_payload` carrying the point, equal in order the bytes of
///   the second group's reference fields — residue `2`'s encoding, then
///   residue `4`'s — by `assert_eq!`; outside, `created.is_ok()`.
/// Collaborators: the pairing family's mock concrete through `create_pairing`
///   under the default params.
#[test]
fn g2_fields_to_value_returns_the_statement_the_fields_describe() {
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
            let fields = invalidate_possession_g2_statement(&payload.adapter, Default::default());
            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), fields);
            assert!(result.is_ok());
            let success = result.expect("the reference fields decode");
            let Ok(key) = payload.adapter.encode_g2(
                EncodeG2Params,
                build_encode_g2_payload(
                    &payload.adapter,
                    EncodeG2PayloadOverrides {
                        point: Some(success.described.key),
                    },
                ),
            );
            let Ok(commitment) = payload.adapter.encode_g2(
                EncodeG2Params,
                build_encode_g2_payload(
                    &payload.adapter,
                    EncodeG2PayloadOverrides {
                        point: Some(success.described.commitment),
                    },
                ),
            );
            assert_eq!(key.bytes.as_ref(), [0, 0, 0, 0, 0, 0, 0, 2]);
            assert_eq!(commitment.bytes.as_ref(), [0, 0, 0, 0, 0, 0, 0, 4]);
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    assert!(created.is_ok());
}
