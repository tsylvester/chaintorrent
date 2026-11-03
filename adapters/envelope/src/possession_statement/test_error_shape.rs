use super::provides::{
    PossessionStatementFromFieldsErrorReturn, build_possession_g1_statement_description,
    build_possession_g2_statement_description,
};
use encoding::{
    CanonicalFieldReaderErrorReturn, CanonicalFieldValue, CanonicalFieldsOverrides,
    IEncodingContract, build_canonical_fields, build_from_fields_params,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingPayload, IPairingArithmetic,
    IPairingConsumer, IPairingReference, build_create_pairing_deps, build_create_pairing_params,
    create_pairing,
};

/// Contract: `fields_to_value`, fields refused: the canonical field reader
///   refuses the list → `Err(PossessionStatementFromFieldsErrorReturn::
///   Fields(error))`, `error` unchanged.
/// Arrange: `build_canonical_fields` overriding `values` with one
///   `CanonicalFieldValue::Bytes`, so the reader refuses the count.
/// Act:     `description.fields_to_value(build_from_fields_params(
///   Default::default()), fields)` inside the consumer's method.
/// Assert:  in place, `assert_eq!(result.err(), Some(
///   PossessionStatementFromFieldsErrorReturn::Fields(
///   CanonicalFieldReaderErrorReturn::Count { expected: 2,
///   actual: 1 })))`, the field count stated as literals; outside,
///   `created.is_ok()`.
/// Collaborators: the pairing family's mock concrete through `create_pairing`
///   under the default params.
#[test]
fn g1_fields_to_value_carries_the_readers_refusal() {
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
                Some(PossessionStatementFromFieldsErrorReturn::Fields(
                    CanonicalFieldReaderErrorReturn::Count {
                        expected: 2,
                        actual: 1
                    }
                ))
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

/// Contract: `fields_to_value`, fields refused: the canonical field reader
///   refuses the list → `Err(PossessionStatementFromFieldsErrorReturn::
///   Fields(error))`, `error` unchanged.
/// Arrange: `build_canonical_fields` overriding `values` with one
///   `CanonicalFieldValue::Bytes`, so the reader refuses the count.
/// Act:     `description.fields_to_value(build_from_fields_params(
///   Default::default()), fields)` inside the consumer's method.
/// Assert:  in place, `assert_eq!(result.err(), Some(
///   PossessionStatementFromFieldsErrorReturn::Fields(
///   CanonicalFieldReaderErrorReturn::Count { expected: 2,
///   actual: 1 })))`, the field count stated as literals; outside,
///   `created.is_ok()`.
/// Collaborators: the pairing family's mock concrete through `create_pairing`
///   under the default params.
#[test]
fn g2_fields_to_value_carries_the_readers_refusal() {
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
                Some(PossessionStatementFromFieldsErrorReturn::Fields(
                    CanonicalFieldReaderErrorReturn::Count {
                        expected: 2,
                        actual: 1
                    }
                ))
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
