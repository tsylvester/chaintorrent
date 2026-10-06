#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{
    PossessionG1StatementDescription, PossessionG1StatementDescriptionConstructorParams,
    PossessionG2StatementDescription, PossessionG2StatementDescriptionConstructorParams,
    PossessionStatementFromFieldsErrorReturn,
};
use super::mock::{build_possession_g1_statement, build_possession_g2_statement};
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFields, CanonicalFieldsOverrides,
    FromFieldsParams, IEncodingContract, ToFieldsParams, build_canonical_fields,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingDeps, CreatePairingPayload,
    DecodeG1ErrorReturn, DecodeG2ErrorReturn, EncodeG1Params, EncodeG1Payload, EncodeG2Params,
    EncodeG2Payload, IPairingAdapter, IPairingConsumer, build_create_pairing_params,
    create_pairing,
};

fn reference_g1_fields<P: IPairingAdapter>(pairing: &P) -> Vec<CanonicalFieldValue> {
    let statement = build_possession_g1_statement(pairing, Default::default());
    let Ok(key) = pairing.encode_g1(
        EncodeG1Params,
        EncodeG1Payload {
            point: statement.key.clone(),
        },
    );
    let Ok(commitment) = pairing.encode_g1(
        EncodeG1Params,
        EncodeG1Payload {
            point: statement.commitment.clone(),
        },
    );
    vec![
        CanonicalFieldValue::Bytes(key.bytes.as_ref().to_vec()),
        CanonicalFieldValue::Bytes(commitment.bytes.as_ref().to_vec()),
    ]
}

fn reference_g2_fields<P: IPairingAdapter>(pairing: &P) -> Vec<CanonicalFieldValue> {
    let statement = build_possession_g2_statement(pairing, Default::default());
    let Ok(key) = pairing.encode_g2(
        EncodeG2Params,
        EncodeG2Payload {
            point: statement.key.clone(),
        },
    );
    let Ok(commitment) = pairing.encode_g2(
        EncodeG2Params,
        EncodeG2Payload {
            point: statement.commitment.clone(),
        },
    );
    vec![
        CanonicalFieldValue::Bytes(key.bytes.as_ref().to_vec()),
        CanonicalFieldValue::Bytes(commitment.bytes.as_ref().to_vec()),
    ]
}

type FieldsProbeOutput = (
    Vec<CanonicalFieldValue>,
    Vec<CanonicalFieldValue>,
    Vec<CanonicalFieldValue>,
    Vec<CanonicalFieldValue>,
);

struct ToFieldsProbe;

impl IPairingConsumer for ToFieldsProbe {
    type Output = FieldsProbeOutput;

    fn consume_pairing<P: IPairingAdapter>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let pairing = &payload.adapter;
        let Ok(g1_description) = PossessionG1StatementDescription::try_new(
            PossessionG1StatementDescriptionConstructorParams { pairing },
        );
        let Ok(g2_description) = PossessionG2StatementDescription::try_new(
            PossessionG2StatementDescriptionConstructorParams { pairing },
        );
        let g1_statement = build_possession_g1_statement(pairing, Default::default());
        let g2_statement = build_possession_g2_statement(pairing, Default::default());

        let Ok(g1_fields) = g1_description.to_fields(ToFieldsParams, &g1_statement);
        let Ok(g2_fields) = g2_description.to_fields(ToFieldsParams, &g2_statement);

        (
            g1_fields.fields.values,
            g2_fields.fields.values,
            reference_g1_fields(pairing),
            reference_g2_fields(pairing),
        )
    }
}

struct FromFieldsProbe;

impl IPairingConsumer for FromFieldsProbe {
    type Output = FieldsProbeOutput;

    fn consume_pairing<P: IPairingAdapter>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let pairing = &payload.adapter;
        let Ok(g1_description) = PossessionG1StatementDescription::try_new(
            PossessionG1StatementDescriptionConstructorParams { pairing },
        );
        let Ok(g2_description) = PossessionG2StatementDescription::try_new(
            PossessionG2StatementDescriptionConstructorParams { pairing },
        );
        let g1_reference = reference_g1_fields(pairing);
        let g2_reference = reference_g2_fields(pairing);
        let g1_fields = build_canonical_fields(CanonicalFieldsOverrides {
            values: Some(g1_reference.clone()),
        });
        let g2_fields = build_canonical_fields(CanonicalFieldsOverrides {
            values: Some(g2_reference.clone()),
        });

        let Ok(g1_success) = g1_description.fields_to_value(FromFieldsParams, g1_fields) else {
            panic!("the reference fields describe a statement")
        };
        let Ok(g2_success) = g2_description.fields_to_value(FromFieldsParams, g2_fields) else {
            panic!("the reference fields describe a statement")
        };
        let Ok(g1_round_trip) = g1_description.to_fields(ToFieldsParams, &g1_success.described);
        let Ok(g2_round_trip) = g2_description.to_fields(ToFieldsParams, &g2_success.described);

        (
            g1_round_trip.fields.values,
            g2_round_trip.fields.values,
            g1_reference,
            g2_reference,
        )
    }
}

struct InvalidEncodingProbe {
    corrupt_index: usize,
}

type ErrorProbeOutput = (
    PossessionStatementFromFieldsErrorReturn,
    PossessionStatementFromFieldsErrorReturn,
);

impl IPairingConsumer for InvalidEncodingProbe {
    type Output = ErrorProbeOutput;

    fn consume_pairing<P: IPairingAdapter>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let pairing = &payload.adapter;
        let Ok(g1_description) = PossessionG1StatementDescription::try_new(
            PossessionG1StatementDescriptionConstructorParams { pairing },
        );
        let Ok(g2_description) = PossessionG2StatementDescription::try_new(
            PossessionG2StatementDescriptionConstructorParams { pairing },
        );
        let mut g1_values = reference_g1_fields(pairing);
        let mut g2_values = reference_g2_fields(pairing);
        g1_values[self.corrupt_index] = CanonicalFieldValue::Bytes(Vec::new());
        g2_values[self.corrupt_index] = CanonicalFieldValue::Bytes(Vec::new());
        let g1_fields = build_canonical_fields(CanonicalFieldsOverrides {
            values: Some(g1_values),
        });
        let g2_fields = build_canonical_fields(CanonicalFieldsOverrides {
            values: Some(g2_values),
        });

        let Err(g1_error) = g1_description.fields_to_value(FromFieldsParams, g1_fields) else {
            panic!("an empty group encoding is refused")
        };
        let Err(g2_error) = g2_description.fields_to_value(FromFieldsParams, g2_fields) else {
            panic!("an empty group encoding is refused")
        };
        (g1_error, g2_error)
    }
}

struct ErrorProbe {
    fields: CanonicalFields,
}

impl IPairingConsumer for ErrorProbe {
    type Output = ErrorProbeOutput;

    fn consume_pairing<P: IPairingAdapter>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let pairing = &payload.adapter;
        let Ok(g1_description) = PossessionG1StatementDescription::try_new(
            PossessionG1StatementDescriptionConstructorParams { pairing },
        );
        let Ok(g2_description) = PossessionG2StatementDescription::try_new(
            PossessionG2StatementDescriptionConstructorParams { pairing },
        );
        let g1_fields = build_canonical_fields(CanonicalFieldsOverrides {
            values: Some(self.fields.values.clone()),
        });
        let g2_fields = build_canonical_fields(CanonicalFieldsOverrides {
            values: Some(self.fields.values.clone()),
        });

        let Err(g1_error) = g1_description.fields_to_value(FromFieldsParams, g1_fields) else {
            panic!("a malformed field list is refused")
        };
        let Err(g2_error) = g2_description.fields_to_value(FromFieldsParams, g2_fields) else {
            panic!("a malformed field list is refused")
        };
        (g1_error, g2_error)
    }
}

struct KindsProbe;

impl IPairingConsumer for KindsProbe {
    type Output = (&'static [CanonicalFieldKind], &'static [CanonicalFieldKind]);

    fn consume_pairing<P: IPairingAdapter>(
        &self,
        _params: ConsumePairingParams,
        _payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        (
            <PossessionG1StatementDescription<'_, P> as IEncodingContract>::FIELDS,
            <PossessionG2StatementDescription<'_, P> as IEncodingContract>::FIELDS,
        )
    }
}

/// Contract: a statement maps to its key's bytes then its commitment's bytes,
///   each unchanged.
/// Arrange: the reference statement — the pairing's generator as key and its
///   nonidentity double as commitment, in each group.
/// Act:     `description.to_fields(ToFieldsParams, &statement)` for the G1 and
///   G2 descriptions under a real BN254 arkworks pairing.
/// Assert:  `success.fields.values` equals the reference fields — the same
///   pairing's `encode_g1`/`encode_g2` output copied through `as_ref()`.
#[test]
fn to_fields_lists_the_key_then_the_commitment() {
    // Arrange
    let consumer = ToFieldsProbe;
    let params = build_create_pairing_params(Default::default());

    // Act
    let Ok(success) = create_pairing(
        &CreatePairingDeps { consumer },
        params,
        CreatePairingPayload,
    ) else {
        panic!("the pairing factory resolves the concrete")
    };

    // Assert
    let (g1_values, g2_values, g1_expected, g2_expected) = success.output;
    assert_eq!(g1_values, g1_expected);
    assert_eq!(g2_values, g2_expected);
}

/// Contract: the canonical fields map back to the statement they describe.
/// Arrange: `build_canonical_fields` with the reference fields for each group.
/// Act:     `description.fields_to_value(FromFieldsParams, fields)`, then
///   `description.to_fields(ToFieldsParams, &success.described)` on the rebuilt
///   statements.
/// Assert:  `round_trip.fields.values` equals the reference fields for each
///   group — the encodings agree without `P::G1`/`P::G2` `PartialEq`.
#[test]
fn fields_to_value_returns_the_statement_the_reference_fields_describe() {
    // Arrange
    let consumer = FromFieldsProbe;
    let params = build_create_pairing_params(Default::default());

    // Act
    let Ok(success) = create_pairing(
        &CreatePairingDeps { consumer },
        params,
        CreatePairingPayload,
    ) else {
        panic!("the pairing factory resolves the concrete")
    };

    // Assert
    let (g1_round_trip, g2_round_trip, g1_expected, g2_expected) = success.output;
    assert_eq!(g1_round_trip, g1_expected);
    assert_eq!(g2_round_trip, g2_expected);
}

/// Contract: a `Bytes` field whose bytes are not the pairing's encoding of a
///   group element is refused, naming the index of the first invalid encoding;
///   no empty encoding is admitted.
/// Arrange: the reference fields with index `0`, then index `1`, replaced by
///   `CanonicalFieldValue::Bytes(Vec::new())`.
/// Act:     `description.fields_to_value(FromFieldsParams, fields)` for the G1
///   and G2 descriptions under a real BN254 arkworks pairing.
/// Assert:  `error` is `InvalidG1 { index }` / `InvalidG2 { index }` with the
///   decoder's refusal, at the corrupted index each time.
#[test]
fn fields_to_value_rejects_invalid_group_encodings() {
    // Arrange
    let params = build_create_pairing_params(Default::default());

    // Act
    let consumer = InvalidEncodingProbe { corrupt_index: 0 };
    let Ok(at_key) = create_pairing(
        &CreatePairingDeps { consumer },
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    ) else {
        panic!("the pairing factory resolves the concrete")
    };
    let consumer = InvalidEncodingProbe { corrupt_index: 1 };
    let Ok(at_commitment) = create_pairing(
        &CreatePairingDeps { consumer },
        params,
        CreatePairingPayload,
    ) else {
        panic!("the pairing factory resolves the concrete")
    };

    // Assert
    assert_eq!(
        at_key.output.0,
        PossessionStatementFromFieldsErrorReturn::InvalidG1 {
            index: 0,
            error: DecodeG1ErrorReturn::WrongLength {
                expected: 64,
                actual: 0,
            },
        }
    );
    assert_eq!(
        at_key.output.1,
        PossessionStatementFromFieldsErrorReturn::InvalidG2 {
            index: 0,
            error: DecodeG2ErrorReturn::WrongLength {
                expected: 128,
                actual: 0,
            },
        }
    );
    assert_eq!(
        at_commitment.output.0,
        PossessionStatementFromFieldsErrorReturn::InvalidG1 {
            index: 1,
            error: DecodeG1ErrorReturn::WrongLength {
                expected: 64,
                actual: 0,
            },
        }
    );
    assert_eq!(
        at_commitment.output.1,
        PossessionStatementFromFieldsErrorReturn::InvalidG2 {
            index: 1,
            error: DecodeG2ErrorReturn::WrongLength {
                expected: 128,
                actual: 0,
            },
        }
    );
}

/// Contract: a field list shorter than the canonical sequence is refused with
///   both lengths.
/// Arrange: the reference fields without their last entry.
/// Act:     `description.fields_to_value(FromFieldsParams, fields)` for the G1
///   and G2 descriptions.
/// Assert:  `error` is `PossessionStatementFromFieldsErrorReturn::FieldCount
///   { expected: 2, actual: 1 }` for each.
#[test]
fn fields_to_value_rejects_a_field_list_one_short() {
    // Arrange
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![CanonicalFieldValue::Bytes(vec![0x05; 64])]),
    });
    let consumer = ErrorProbe { fields };
    let params = build_create_pairing_params(Default::default());

    // Act
    let Ok(success) = create_pairing(
        &CreatePairingDeps { consumer },
        params,
        CreatePairingPayload,
    ) else {
        panic!("the pairing factory resolves the concrete")
    };

    // Assert
    assert_eq!(
        success.output.0,
        PossessionStatementFromFieldsErrorReturn::FieldCount {
            expected: 2,
            actual: 1
        }
    );
    assert_eq!(
        success.output.1,
        PossessionStatementFromFieldsErrorReturn::FieldCount {
            expected: 2,
            actual: 1
        }
    );
}

/// Contract: a field list longer than the canonical sequence is refused with
///   both lengths.
/// Arrange: the reference fields followed by
///   `CanonicalFieldValue::Bytes(vec![0x06])`.
/// Act:     `description.fields_to_value(FromFieldsParams, fields)` for the G1
///   and G2 descriptions.
/// Assert:  `error` is `PossessionStatementFromFieldsErrorReturn::FieldCount
///   { expected: 2, actual: 3 }` for each.
#[test]
fn fields_to_value_rejects_a_field_list_one_long() {
    // Arrange
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Bytes(vec![0x05; 64]),
            CanonicalFieldValue::Bytes(vec![0x06; 64]),
            CanonicalFieldValue::Bytes(vec![0x06]),
        ]),
    });
    let consumer = ErrorProbe { fields };
    let params = build_create_pairing_params(Default::default());

    // Act
    let Ok(success) = create_pairing(
        &CreatePairingDeps { consumer },
        params,
        CreatePairingPayload,
    ) else {
        panic!("the pairing factory resolves the concrete")
    };

    // Assert
    assert_eq!(
        success.output.0,
        PossessionStatementFromFieldsErrorReturn::FieldCount {
            expected: 2,
            actual: 3
        }
    );
    assert_eq!(
        success.output.1,
        PossessionStatementFromFieldsErrorReturn::FieldCount {
            expected: 2,
            actual: 3
        }
    );
}

/// Contract: a value that is not a byte string is refused, naming its index.
/// Arrange: the reference fields with index `1` replaced by
///   `CanonicalFieldValue::FixedBytes32([0x04; 32])`.
/// Act:     `description.fields_to_value(FromFieldsParams, fields)` for the G1
///   and G2 descriptions.
/// Assert:  `error` is `PossessionStatementFromFieldsErrorReturn::FieldKind
///   { index: 1, expected: CanonicalFieldKind::Bytes }` for each.
#[test]
fn fields_to_value_rejects_a_field_of_the_wrong_kind() {
    // Arrange
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Bytes(vec![0x05; 64]),
            CanonicalFieldValue::FixedBytes32([0x04; 32]),
        ]),
    });
    let consumer = ErrorProbe { fields };
    let params = build_create_pairing_params(Default::default());

    // Act
    let Ok(success) = create_pairing(
        &CreatePairingDeps { consumer },
        params,
        CreatePairingPayload,
    ) else {
        panic!("the pairing factory resolves the concrete")
    };

    // Assert
    assert_eq!(
        success.output.0,
        PossessionStatementFromFieldsErrorReturn::FieldKind {
            index: 1,
            expected: CanonicalFieldKind::Bytes
        }
    );
    assert_eq!(
        success.output.1,
        PossessionStatementFromFieldsErrorReturn::FieldKind {
            index: 1,
            expected: CanonicalFieldKind::Bytes
        }
    );
}

/// Contract: of both fields misplaced, the lower index decides.
/// Arrange: the fields `vec![CanonicalFieldValue::Text("x".to_string()),
///   CanonicalFieldValue::Unsigned64(1)]`.
/// Act:     `description.fields_to_value(FromFieldsParams, fields)` for the G1
///   and G2 descriptions.
/// Assert:  `error` is `PossessionStatementFromFieldsErrorReturn::FieldKind
///   { index: 0, expected: CanonicalFieldKind::Bytes }` for each.
#[test]
fn fields_to_value_reports_the_lowest_field_of_the_wrong_kind() {
    // Arrange
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Text("x".to_string()),
            CanonicalFieldValue::Unsigned64(1),
        ]),
    });
    let consumer = ErrorProbe { fields };
    let params = build_create_pairing_params(Default::default());

    // Act
    let Ok(success) = create_pairing(
        &CreatePairingDeps { consumer },
        params,
        CreatePairingPayload,
    ) else {
        panic!("the pairing factory resolves the concrete")
    };

    // Assert
    assert_eq!(
        success.output.0,
        PossessionStatementFromFieldsErrorReturn::FieldKind {
            index: 0,
            expected: CanonicalFieldKind::Bytes
        }
    );
    assert_eq!(
        success.output.1,
        PossessionStatementFromFieldsErrorReturn::FieldKind {
            index: 0,
            expected: CanonicalFieldKind::Bytes
        }
    );
}

/// Contract: the description's declared kinds are the sequence an encoding
///   concrete decodes against.
/// Arrange: each G1 and G2 description under a real BN254 arkworks pairing.
/// Act:     read each description's `FIELDS`.
/// Assert:  `FIELDS` equals `[CanonicalFieldKind::Bytes,
///   CanonicalFieldKind::Bytes]` as a slice for each.
#[test]
fn possession_statement_description_declares_its_field_kinds_in_canonical_order() {
    // Arrange
    let consumer = KindsProbe;
    let params = build_create_pairing_params(Default::default());

    // Act
    let Ok(success) = create_pairing(
        &CreatePairingDeps { consumer },
        params,
        CreatePairingPayload,
    ) else {
        panic!("the pairing factory resolves the concrete")
    };

    // Assert
    assert_eq!(
        success.output.0,
        [CanonicalFieldKind::Bytes, CanonicalFieldKind::Bytes]
    );
    assert_eq!(
        success.output.1,
        [CanonicalFieldKind::Bytes, CanonicalFieldKind::Bytes]
    );
}
