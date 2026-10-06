#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{
    DELIVERY_PURPOSE_MINT, DELIVERY_PURPOSE_REPLACEMENT, DeliveryPurpose, DeliveryRelation,
    MintPurpose, MintStatement, MintStatementDescription,
    MintStatementDescriptionConstructorParams, MintStatementFromFieldsErrorReturn, TransferPurpose,
    UndeclaredDeliveryPurposeCode,
};
use super::mock::{MintStatementOverrides, build_mint_statement};
use chain::{
    CHAIN_FORMS_INTERFACE_VERSION, ChainFormsDeclaration, ChainFormsIdentifier, IChainForms,
};
use domain::{
    AssetIdentityHashTryNewErrorReturn, ParameterSetIdentifierTryNewErrorReturn,
    SuiteIdentifierTryNewErrorReturn,
};
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFieldsOverrides, FromFieldParams,
    FromFieldReturn, FromFieldSuccessReturn, FromFieldsParams, ICanonicalField, IEncodingContract,
    ToFieldParams, ToFieldReturn, ToFieldSuccessReturn, ToFieldsParams, build_canonical_fields,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingDeps, CreatePairingPayload,
    DecodeG1ErrorReturn, DecodeG2ErrorReturn, EncodeG1Params, EncodeG1Payload, EncodeG2Params,
    EncodeG2Payload, IPairingAdapter, IPairingConsumer, build_create_pairing_params,
    create_pairing,
};

#[derive(Debug, PartialEq, Eq)]
enum TestFormFromFieldErrorReturn {
    WrongKind,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct TestAddress {
    bytes: [u8; 20],
}

impl ICanonicalField for TestAddress {
    type FromFieldErrorReturn = TestFormFromFieldErrorReturn;
    const KIND: CanonicalFieldKind = CanonicalFieldKind::FixedBytes20;

    fn to_field(_params: ToFieldParams, payload: &Self) -> ToFieldReturn {
        Ok(ToFieldSuccessReturn {
            field: CanonicalFieldValue::FixedBytes20(payload.bytes),
        })
    }

    fn from_field(
        _params: FromFieldParams,
        payload: CanonicalFieldValue,
    ) -> FromFieldReturn<Self, Self::FromFieldErrorReturn> {
        match payload {
            CanonicalFieldValue::FixedBytes20(bytes) => Ok(FromFieldSuccessReturn {
                value: TestAddress { bytes },
            }),
            _ => Err(TestFormFromFieldErrorReturn::WrongKind),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct TestWord {
    bytes: [u8; 32],
}

impl ICanonicalField for TestWord {
    type FromFieldErrorReturn = TestFormFromFieldErrorReturn;
    const KIND: CanonicalFieldKind = CanonicalFieldKind::Unsigned256;

    fn to_field(_params: ToFieldParams, payload: &Self) -> ToFieldReturn {
        Ok(ToFieldSuccessReturn {
            field: CanonicalFieldValue::Unsigned256(payload.bytes),
        })
    }

    fn from_field(
        _params: FromFieldParams,
        payload: CanonicalFieldValue,
    ) -> FromFieldReturn<Self, Self::FromFieldErrorReturn> {
        match payload {
            CanonicalFieldValue::Unsigned256(bytes) => Ok(FromFieldSuccessReturn {
                value: TestWord { bytes },
            }),
            _ => Err(TestFormFromFieldErrorReturn::WrongKind),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct TestForms;

impl IChainForms for TestForms {
    const DECLARATION: ChainFormsDeclaration = ChainFormsDeclaration {
        identifier: ChainFormsIdentifier::EvmV1,
        adapter_version: 1,
        interface_version: CHAIN_FORMS_INTERFACE_VERSION,
    };

    type Identity = TestAddress;
    type Entitlement = TestWord;
    type Interval = TestWord;
    type ChainIdentifier = TestWord;
}

type TestErrorReturn = MintStatementFromFieldsErrorReturn<TestForms>;

fn reference_statement<P: IPairingAdapter>(pairing: &P) -> MintStatement<P, TestForms> {
    build_mint_statement::<P, TestForms>(
        pairing,
        MintStatementOverrides {
            chain: Some(TestWord { bytes: [0x21; 32] }),
            entitlement_contract: Some(TestAddress { bytes: [0x31; 20] }),
            source_entitlement: Some(TestWord { bytes: [0x41; 32] }),
            target_entitlement: Some(TestWord { bytes: [0x42; 32] }),
            old_interval: Some(TestWord { bytes: [0x51; 32] }),
            new_interval: Some(TestWord { bytes: [0x52; 32] }),
            purpose: Some(DELIVERY_PURPOSE_REPLACEMENT),
            seller: Some(TestAddress { bytes: [0x61; 20] }),
            buyer: Some(TestAddress { bytes: [0x62; 20] }),
            expiry: Some(1_800_000_000),
            ..Default::default()
        },
    )
}

fn encode_g1_bytes<P: IPairingAdapter>(pairing: &P, point: P::G1) -> Vec<u8> {
    let Ok(encoded) = pairing.encode_g1(EncodeG1Params, EncodeG1Payload { point });
    encoded.bytes.as_ref().to_vec()
}

fn encode_g2_bytes<P: IPairingAdapter>(pairing: &P, point: P::G2) -> Vec<u8> {
    let Ok(encoded) = pairing.encode_g2(EncodeG2Params, EncodeG2Payload { point });
    encoded.bytes.as_ref().to_vec()
}

fn reference_fields<P: IPairingAdapter>(
    pairing: &P,
    statement: &MintStatement<P, TestForms>,
) -> Vec<CanonicalFieldValue> {
    vec![
        CanonicalFieldValue::FixedBytes32(*statement.suite_identifier.identifier()),
        CanonicalFieldValue::Unsigned16(statement.suite_identifier.version()),
        CanonicalFieldValue::Unsigned256(statement.chain.bytes),
        CanonicalFieldValue::FixedBytes20(statement.entitlement_contract.bytes),
        CanonicalFieldValue::FixedBytes32(*statement.asset_identity_hash.as_bytes()),
        CanonicalFieldValue::FixedBytes32(*statement.parameter_set_identifier.as_bytes()),
        CanonicalFieldValue::Unsigned256(statement.source_entitlement.bytes),
        CanonicalFieldValue::Unsigned256(statement.target_entitlement.bytes),
        CanonicalFieldValue::Unsigned256(statement.old_interval.bytes),
        CanonicalFieldValue::Unsigned256(statement.new_interval.bytes),
        CanonicalFieldValue::Unsigned16(statement.purpose.code()),
        CanonicalFieldValue::FixedBytes20(statement.seller.bytes),
        CanonicalFieldValue::FixedBytes20(statement.buyer.bytes),
        CanonicalFieldValue::Bytes(encode_g1_bytes(pairing, statement.buyer_keys.pk1.clone())),
        CanonicalFieldValue::Bytes(encode_g2_bytes(pairing, statement.buyer_keys.pk2.clone())),
        CanonicalFieldValue::Bytes(encode_g1_bytes(pairing, statement.envelope.c1.clone())),
        CanonicalFieldValue::Bytes(encode_g1_bytes(pairing, statement.envelope.c2.clone())),
        CanonicalFieldValue::Bytes(encode_g2_bytes(pairing, statement.envelope.d1.clone())),
        CanonicalFieldValue::Bytes(encode_g2_bytes(pairing, statement.envelope.d2.clone())),
        CanonicalFieldValue::Unsigned64(statement.expiry),
        CanonicalFieldValue::Bytes(encode_g2_bytes(
            pairing,
            statement.first_messages.hpub.clone(),
        )),
        CanonicalFieldValue::Bytes(encode_g1_bytes(
            pairing,
            statement.first_messages.c1.clone(),
        )),
        CanonicalFieldValue::Bytes(encode_g1_bytes(
            pairing,
            statement.first_messages.c2.clone(),
        )),
        CanonicalFieldValue::Bytes(encode_g2_bytes(
            pairing,
            statement.first_messages.d1.clone(),
        )),
        CanonicalFieldValue::Bytes(encode_g2_bytes(
            pairing,
            statement.first_messages.d2.clone(),
        )),
    ]
}

struct ToFieldsProbe;

impl IPairingConsumer for ToFieldsProbe {
    type Output = (Vec<CanonicalFieldValue>, Vec<CanonicalFieldValue>);

    fn consume_pairing<P: IPairingAdapter>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let pairing = &payload.adapter;
        let Ok(description) = MintStatementDescription::<P, TestForms>::try_new(
            MintStatementDescriptionConstructorParams { pairing },
        );
        let statement = reference_statement(pairing);
        let expected = reference_fields(pairing, &statement);

        let Ok(success) = description.to_fields(ToFieldsParams, &statement);

        (success.fields.values, expected)
    }
}

struct FromFieldsProbe;

impl IPairingConsumer for FromFieldsProbe {
    type Output = (Vec<CanonicalFieldValue>, Vec<CanonicalFieldValue>);

    fn consume_pairing<P: IPairingAdapter>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let pairing = &payload.adapter;
        let Ok(description) = MintStatementDescription::<P, TestForms>::try_new(
            MintStatementDescriptionConstructorParams { pairing },
        );
        let statement = reference_statement(pairing);
        let expected = reference_fields(pairing, &statement);
        let fields = build_canonical_fields(CanonicalFieldsOverrides {
            values: Some(expected.clone()),
        });

        let Ok(success) = description.fields_to_value(FromFieldsParams, fields) else {
            panic!("the reference fields describe a statement")
        };
        let Ok(round_trip) = description.to_fields(ToFieldsParams, &success.described);

        (round_trip.fields.values, expected)
    }
}

struct ShortFieldsProbe;

impl IPairingConsumer for ShortFieldsProbe {
    type Output = TestErrorReturn;

    fn consume_pairing<P: IPairingAdapter>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let pairing = &payload.adapter;
        let Ok(description) = MintStatementDescription::<P, TestForms>::try_new(
            MintStatementDescriptionConstructorParams { pairing },
        );
        let mut values = reference_fields(pairing, &reference_statement(pairing));
        values.pop();
        let fields = build_canonical_fields(CanonicalFieldsOverrides {
            values: Some(values),
        });

        let Err(error) = description.fields_to_value(FromFieldsParams, fields) else {
            panic!("a field list one short is refused")
        };
        error
    }
}

struct LongFieldsProbe;

impl IPairingConsumer for LongFieldsProbe {
    type Output = TestErrorReturn;

    fn consume_pairing<P: IPairingAdapter>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let pairing = &payload.adapter;
        let Ok(description) = MintStatementDescription::<P, TestForms>::try_new(
            MintStatementDescriptionConstructorParams { pairing },
        );
        let mut values = reference_fields(pairing, &reference_statement(pairing));
        values.push(CanonicalFieldValue::Bytes(vec![0x15]));
        let fields = build_canonical_fields(CanonicalFieldsOverrides {
            values: Some(values),
        });

        let Err(error) = description.fields_to_value(FromFieldsParams, fields) else {
            panic!("a field list one long is refused")
        };
        error
    }
}

struct CorruptIndexProbe {
    index: usize,
    value: CanonicalFieldValue,
}

impl IPairingConsumer for CorruptIndexProbe {
    type Output = TestErrorReturn;

    fn consume_pairing<P: IPairingAdapter>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let pairing = &payload.adapter;
        let Ok(description) = MintStatementDescription::<P, TestForms>::try_new(
            MintStatementDescriptionConstructorParams { pairing },
        );
        let mut values = reference_fields(pairing, &reference_statement(pairing));
        values[self.index] = self.value.clone();
        let fields = build_canonical_fields(CanonicalFieldsOverrides {
            values: Some(values),
        });

        let Err(error) = description.fields_to_value(FromFieldsParams, fields) else {
            panic!("a corrupted field list is refused")
        };
        error
    }
}

struct CorruptIndicesProbe {
    first: usize,
    second: usize,
    value: CanonicalFieldValue,
}

impl IPairingConsumer for CorruptIndicesProbe {
    type Output = TestErrorReturn;

    fn consume_pairing<P: IPairingAdapter>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let pairing = &payload.adapter;
        let Ok(description) = MintStatementDescription::<P, TestForms>::try_new(
            MintStatementDescriptionConstructorParams { pairing },
        );
        let mut values = reference_fields(pairing, &reference_statement(pairing));
        values[self.first] = self.value.clone();
        values[self.second] = self.value.clone();
        let fields = build_canonical_fields(CanonicalFieldsOverrides {
            values: Some(values),
        });

        let Err(error) = description.fields_to_value(FromFieldsParams, fields) else {
            panic!("a corrupted field list is refused")
        };
        error
    }
}

struct PurposeProbe {
    code: u16,
}

impl IPairingConsumer for PurposeProbe {
    type Output = Result<MintPurpose, TestErrorReturn>;

    fn consume_pairing<P: IPairingAdapter>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let pairing = &payload.adapter;
        let Ok(description) = MintStatementDescription::<P, TestForms>::try_new(
            MintStatementDescriptionConstructorParams { pairing },
        );
        let mut values = reference_fields(pairing, &reference_statement(pairing));
        values[10] = CanonicalFieldValue::Unsigned16(self.code);
        let fields = build_canonical_fields(CanonicalFieldsOverrides {
            values: Some(values),
        });

        match description.fields_to_value(FromFieldsParams, fields) {
            Ok(success) => Ok(success.described.purpose),
            Err(error) => Err(error),
        }
    }
}

struct KindsProbe;

impl IPairingConsumer for KindsProbe {
    type Output = &'static [CanonicalFieldKind];

    fn consume_pairing<P: IPairingAdapter>(
        &self,
        _params: ConsumePairingParams,
        _payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        <MintStatementDescription<'_, P, TestForms> as IEncodingContract>::FIELDS
    }
}

/// Contract: a transcript maps to its canonical fields in transcript order,
///   each form through its own field.
/// Arrange: the reference transcript — fixed chain, contract, entitlements,
///   intervals, identities, the replacement purpose, and a fixed expiry, with
///   the builder's domain defaults and typed points from the pairing.
/// Act:     `description.to_fields(ToFieldsParams, &statement)` under a real
///   BN254 arkworks pairing.
/// Assert:  `success.fields.values` equals the reference fields — the same
///   pairing's `encode_g1`/`encode_g2` output copied through `as_ref()`.
#[test]
fn to_fields_lists_the_mint_transcript_in_transcript_order() {
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
    let (actual, expected) = success.output;
    assert_eq!(actual, expected);
}

/// Contract: the canonical fields map back to the transcript they describe.
/// Arrange: `build_canonical_fields` with the reference fields.
/// Act:     `description.fields_to_value(FromFieldsParams, fields)`, then
///   `description.to_fields(ToFieldsParams, &success.described)` on the rebuilt
///   statement under a real BN254 arkworks pairing.
/// Assert:  `round_trip.fields.values` equals the reference fields — the
///   encodings agree without `P::G1`/`P::G2` `PartialEq`.
#[test]
fn fields_to_value_returns_the_transcript_the_reference_fields_describe() {
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
    let (round_trip, expected) = success.output;
    assert_eq!(round_trip, expected);
}

/// Contract: a field list that does not convert into
///   `[CanonicalFieldValue; MINT_STATEMENT_FIELD_COUNT]` is refused with the
///   expected and actual counts before any field is read.
/// Arrange: the reference fields without their last entry.
/// Act:     `description.fields_to_value(FromFieldsParams, fields)` under a
///   real BN254 arkworks pairing.
/// Assert:  `error` equals `FieldCount { expected: 25, actual: 24 }`.
#[test]
fn fields_to_value_rejects_a_field_list_one_short() {
    // Arrange
    let consumer = ShortFieldsProbe;
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
        success.output,
        MintStatementFromFieldsErrorReturn::FieldCount {
            expected: 25,
            actual: 24
        }
    );
}

/// Contract: a field list that does not convert into
///   `[CanonicalFieldValue; MINT_STATEMENT_FIELD_COUNT]` is refused with the
///   expected and actual counts before any field is read.
/// Arrange: the reference fields followed by `CanonicalFieldValue::Bytes`.
/// Act:     `description.fields_to_value(FromFieldsParams, fields)` under a
///   real BN254 arkworks pairing.
/// Assert:  `error` equals `FieldCount { expected: 25, actual: 26 }`.
#[test]
fn fields_to_value_rejects_a_field_list_one_long() {
    // Arrange
    let consumer = LongFieldsProbe;
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
        success.output,
        MintStatementFromFieldsErrorReturn::FieldCount {
            expected: 25,
            actual: 26
        }
    );
}

/// Contract: a directly mapped value that is not its declared kind is refused,
///   naming its index and kind.
/// Arrange: the reference fields with index `19` replaced by
///   `CanonicalFieldValue::Unsigned16(1)`.
/// Act:     `description.fields_to_value(FromFieldsParams, fields)` under a
///   real BN254 arkworks pairing.
/// Assert:  `error` equals `FieldKind { index: 19, expected:
///   CanonicalFieldKind::Unsigned64 }`.
#[test]
fn fields_to_value_rejects_a_directly_mapped_field_of_the_wrong_kind() {
    // Arrange
    let consumer = CorruptIndexProbe {
        index: 19,
        value: CanonicalFieldValue::Unsigned16(1),
    };
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
        success.output,
        MintStatementFromFieldsErrorReturn::FieldKind {
            index: 19,
            expected: CanonicalFieldKind::Unsigned64
        }
    );
}

/// Contract: the chain identifier form's refusal from `from_field` is carried
///   unchanged with its index.
/// Arrange: the reference fields with index `2` replaced by
///   `CanonicalFieldValue::Unsigned64(1)`.
/// Act:     `description.fields_to_value(FromFieldsParams, fields)` under a
///   real BN254 arkworks pairing.
/// Assert:  `error` equals `ChainIdentifier { index: 2, error: WrongKind }`.
#[test]
fn fields_to_value_carries_the_chain_identifiers_refusal_with_its_index() {
    // Arrange
    let consumer = CorruptIndexProbe {
        index: 2,
        value: CanonicalFieldValue::Unsigned64(1),
    };
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
        success.output,
        MintStatementFromFieldsErrorReturn::ChainIdentifier {
            index: 2,
            error: TestFormFromFieldErrorReturn::WrongKind
        }
    );
}

/// Contract: an identity form's refusal from `from_field` is carried unchanged
///   with its index.
/// Arrange: the reference fields with index `12` replaced by
///   `CanonicalFieldValue::Unsigned64(1)`.
/// Act:     `description.fields_to_value(FromFieldsParams, fields)` under a
///   real BN254 arkworks pairing.
/// Assert:  `error` equals `Identity { index: 12, error: WrongKind }`.
#[test]
fn fields_to_value_carries_an_identitys_refusal_with_its_index() {
    // Arrange
    let consumer = CorruptIndexProbe {
        index: 12,
        value: CanonicalFieldValue::Unsigned64(1),
    };
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
        success.output,
        MintStatementFromFieldsErrorReturn::Identity {
            index: 12,
            error: TestFormFromFieldErrorReturn::WrongKind
        }
    );
}

/// Contract: an entitlement form's refusal from `from_field` is carried
///   unchanged with its index.
/// Arrange: the reference fields with index `7` replaced by
///   `CanonicalFieldValue::Unsigned64(1)`.
/// Act:     `description.fields_to_value(FromFieldsParams, fields)` under a
///   real BN254 arkworks pairing.
/// Assert:  `error` equals `Entitlement { index: 7, error: WrongKind }`.
#[test]
fn fields_to_value_carries_an_entitlements_refusal_with_its_index() {
    // Arrange
    let consumer = CorruptIndexProbe {
        index: 7,
        value: CanonicalFieldValue::Unsigned64(1),
    };
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
        success.output,
        MintStatementFromFieldsErrorReturn::Entitlement {
            index: 7,
            error: TestFormFromFieldErrorReturn::WrongKind
        }
    );
}

/// Contract: an interval form's refusal from `from_field` is carried unchanged
///   with its index.
/// Arrange: the reference fields with index `9` replaced by
///   `CanonicalFieldValue::Unsigned64(1)`.
/// Act:     `description.fields_to_value(FromFieldsParams, fields)` under a
///   real BN254 arkworks pairing.
/// Assert:  `error` equals `Interval { index: 9, error: WrongKind }`.
#[test]
fn fields_to_value_carries_an_intervals_refusal_with_its_index() {
    // Arrange
    let consumer = CorruptIndexProbe {
        index: 9,
        value: CanonicalFieldValue::Unsigned64(1),
    };
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
        success.output,
        MintStatementFromFieldsErrorReturn::Interval {
            index: 9,
            error: TestFormFromFieldErrorReturn::WrongKind
        }
    );
}

/// Contract: of a form refusal and a later wrong kind, the lower index
///   decides — the fields are read in ascending index.
/// Arrange: the reference fields with index `3` and index `4` each replaced by
///   `CanonicalFieldValue::Unsigned64(1)`.
/// Act:     `description.fields_to_value(FromFieldsParams, fields)` under a
///   real BN254 arkworks pairing.
/// Assert:  `error` equals `Identity { index: 3, error: WrongKind }`, not the
///   index-4 refusal.
#[test]
fn fields_to_value_reports_the_lowest_failing_field() {
    // Arrange
    let consumer = CorruptIndicesProbe {
        first: 3,
        second: 4,
        value: CanonicalFieldValue::Unsigned64(1),
    };
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
        success.output,
        MintStatementFromFieldsErrorReturn::Identity {
            index: 3,
            error: TestFormFromFieldErrorReturn::WrongKind
        }
    );
}

/// Contract: `SuiteIdentifier::try_new` over fields `0` and `1` returns
///   `Err(error)`, and the refusal is carried unchanged.
/// Arrange: the reference fields with index `1` replaced by
///   `CanonicalFieldValue::Unsigned16(0)`.
/// Act:     `description.fields_to_value(FromFieldsParams, fields)` under a
///   real BN254 arkworks pairing.
/// Assert:  `error` equals `SuiteIdentifier(ZeroVersion)`.
#[test]
fn fields_to_value_carries_the_suite_identifiers_refusal() {
    // Arrange
    let consumer = CorruptIndexProbe {
        index: 1,
        value: CanonicalFieldValue::Unsigned16(0),
    };
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
        success.output,
        MintStatementFromFieldsErrorReturn::SuiteIdentifier(
            SuiteIdentifierTryNewErrorReturn::ZeroVersion
        )
    );
}

/// Contract: `AssetIdentityHash::try_new` over field `4` returns `Err(error)`,
///   and the refusal is carried unchanged.
/// Arrange: the reference fields with index `4` replaced by
///   `CanonicalFieldValue::FixedBytes32([0; 32])`.
/// Act:     `description.fields_to_value(FromFieldsParams, fields)` under a
///   real BN254 arkworks pairing.
/// Assert:  `error` equals `AssetIdentityHash(AllZero)`.
#[test]
fn fields_to_value_carries_the_asset_identity_hashs_refusal() {
    // Arrange
    let consumer = CorruptIndexProbe {
        index: 4,
        value: CanonicalFieldValue::FixedBytes32([0; 32]),
    };
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
        success.output,
        MintStatementFromFieldsErrorReturn::AssetIdentityHash(
            AssetIdentityHashTryNewErrorReturn::AllZero
        )
    );
}

/// Contract: `ParameterSetIdentifier::try_new` over field `5` returns
///   `Err(error)`, and the refusal is carried unchanged.
/// Arrange: the reference fields with index `5` replaced by
///   `CanonicalFieldValue::FixedBytes32([0; 32])`.
/// Act:     `description.fields_to_value(FromFieldsParams, fields)` under a
///   real BN254 arkworks pairing.
/// Assert:  `error` equals `ParameterSetIdentifier(AllZero)`.
#[test]
fn fields_to_value_carries_the_parameter_set_digests_refusal() {
    // Arrange
    let consumer = CorruptIndexProbe {
        index: 5,
        value: CanonicalFieldValue::FixedBytes32([0; 32]),
    };
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
        success.output,
        MintStatementFromFieldsErrorReturn::ParameterSetIdentifier(
            ParameterSetIdentifierTryNewErrorReturn::AllZero
        )
    );
}

/// Contract: `MintPurpose::try_from(code)` rejects a code no declared purpose
///   carries with `PurposeCode` naming the index and the code.
/// Arrange: the reference fields with index `10` replaced by
///   `CanonicalFieldValue::Unsigned16(5)`.
/// Act:     `description.fields_to_value(FromFieldsParams, fields)` under a
///   real BN254 arkworks pairing.
/// Assert:  `error` equals `PurposeCode { index: 10, code: 5 }`.
#[test]
fn fields_to_value_rejects_a_purpose_code_no_declared_purpose_carries() {
    // Arrange
    let consumer = CorruptIndexProbe {
        index: 10,
        value: CanonicalFieldValue::Unsigned16(5),
    };
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
        success.output,
        MintStatementFromFieldsErrorReturn::PurposeCode { index: 10, code: 5 }
    );
}

/// Contract: `MintPurpose::try_from` admits the mint relation's codes and
///   rejects the transfer relation's, which no `MintPurpose` carries.
/// Arrange: the reference fields with index `10` set to each of the codes
///   `1`, `4`, `2`, and `3`.
/// Act:     `description.fields_to_value(FromFieldsParams, fields)` for each
///   code under a real BN254 arkworks pairing.
/// Assert:  code `1` yields `MintPurpose::Mint`, code `4`
///   `MintPurpose::Replacement`, and codes `2` and `3` the `PurposeCode`
///   refusal naming index `10` and the code.
#[test]
fn fields_to_value_admits_each_mint_purpose() {
    // Arrange
    let params = build_create_pairing_params(Default::default());

    // Act
    let Ok(mint) = create_pairing(
        &CreatePairingDeps {
            consumer: PurposeProbe { code: 1 },
        },
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    ) else {
        panic!("the pairing factory resolves the concrete")
    };
    let Ok(replacement) = create_pairing(
        &CreatePairingDeps {
            consumer: PurposeProbe { code: 4 },
        },
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    ) else {
        panic!("the pairing factory resolves the concrete")
    };
    let Ok(transfer) = create_pairing(
        &CreatePairingDeps {
            consumer: PurposeProbe { code: 2 },
        },
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    ) else {
        panic!("the pairing factory resolves the concrete")
    };
    let Ok(grant) = create_pairing(
        &CreatePairingDeps {
            consumer: PurposeProbe { code: 3 },
        },
        params,
        CreatePairingPayload,
    ) else {
        panic!("the pairing factory resolves the concrete")
    };

    // Assert
    assert_eq!(mint.output, Ok(MintPurpose::Mint));
    assert_eq!(replacement.output, Ok(MintPurpose::Replacement));
    assert_eq!(
        transfer.output,
        Err(MintStatementFromFieldsErrorReturn::PurposeCode { index: 10, code: 2 })
    );
    assert_eq!(
        grant.output,
        Err(MintStatementFromFieldsErrorReturn::PurposeCode { index: 10, code: 3 })
    );
}

/// Contract: the declared purposes carry their unsigned sixteen-bit codes and
///   relations, and each relation-specific purpose type admits exactly its own
///   codes.
/// Arrange: the four `DeliveryPurpose` variants and the codes `1` through `4`.
/// Act:     `code()` and `relation()` on each variant; `MintPurpose::try_from`
///   and `TransferPurpose::try_from` over each code.
/// Assert:  mint, transfer, grant, and replacement carry codes `1`, `2`, `3`,
///   and `4`; mint and replacement the mint relation, transfer and grant the
///   transfer relation; `MintPurpose` admits `1` and `4` and refuses `2` and
///   `3`, `TransferPurpose` admits `2` and `3` and refuses `1` and `4`.
#[test]
fn the_declared_purposes_carry_their_codes_and_relations() {
    // Arrange
    let purposes = [
        DeliveryPurpose::Mint,
        DeliveryPurpose::Transfer,
        DeliveryPurpose::Grant,
        DeliveryPurpose::Replacement,
    ];

    // Act
    let codes: Vec<u16> = purposes.iter().map(|purpose| purpose.code()).collect();
    let relations: Vec<DeliveryRelation> =
        purposes.iter().map(|purpose| purpose.relation()).collect();

    // Assert
    assert_eq!(codes, vec![1, 2, 3, 4]);
    assert_eq!(
        relations,
        vec![
            DeliveryRelation::Mint,
            DeliveryRelation::Transfer,
            DeliveryRelation::Transfer,
            DeliveryRelation::Mint,
        ]
    );
    assert_eq!(DELIVERY_PURPOSE_MINT, MintPurpose::Mint);
    assert_eq!(DELIVERY_PURPOSE_REPLACEMENT, MintPurpose::Replacement);
    assert_eq!(MintPurpose::try_from(1), Ok(MintPurpose::Mint));
    assert_eq!(MintPurpose::try_from(4), Ok(MintPurpose::Replacement));
    assert_eq!(
        MintPurpose::try_from(2),
        Err(UndeclaredDeliveryPurposeCode { code: 2 })
    );
    assert_eq!(
        MintPurpose::try_from(3),
        Err(UndeclaredDeliveryPurposeCode { code: 3 })
    );
    assert_eq!(TransferPurpose::try_from(2), Ok(TransferPurpose::Transfer));
    assert_eq!(TransferPurpose::try_from(3), Ok(TransferPurpose::Grant));
    assert_eq!(
        TransferPurpose::try_from(1),
        Err(UndeclaredDeliveryPurposeCode { code: 1 })
    );
    assert_eq!(
        TransferPurpose::try_from(4),
        Err(UndeclaredDeliveryPurposeCode { code: 4 })
    );
}

/// Contract: a group encoding the pairing's decoder refuses is refused with
///   its index and the decoder's refusal unchanged.
/// Arrange: under the BN254 arkworks concrete, the reference fields with
///   field `13`, then field `14`, replaced by
///   `CanonicalFieldValue::Bytes(Vec::new())`.
/// Act:     `description.fields_to_value(FromFieldsParams, fields)` over each.
/// Assert:  the first refusal equals `InvalidG1 { index: 13, error:
///   DecodeG1ErrorReturn::WrongLength { expected: 64, actual: 0 } }` and the
///   second `InvalidG2 { index: 14, error: DecodeG2ErrorReturn::WrongLength {
///   expected: 128, actual: 0 } }`.
#[test]
fn fields_to_value_rejects_invalid_mint_group_encodings() {
    // Arrange
    let params = build_create_pairing_params(Default::default());

    // Act
    let Ok(at_g1) = create_pairing(
        &CreatePairingDeps {
            consumer: CorruptIndexProbe {
                index: 13,
                value: CanonicalFieldValue::Bytes(Vec::new()),
            },
        },
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    ) else {
        panic!("the pairing factory resolves the concrete")
    };
    let Ok(at_g2) = create_pairing(
        &CreatePairingDeps {
            consumer: CorruptIndexProbe {
                index: 14,
                value: CanonicalFieldValue::Bytes(Vec::new()),
            },
        },
        params,
        CreatePairingPayload,
    ) else {
        panic!("the pairing factory resolves the concrete")
    };

    // Assert
    assert_eq!(
        at_g1.output,
        MintStatementFromFieldsErrorReturn::InvalidG1 {
            index: 13,
            error: DecodeG1ErrorReturn::WrongLength {
                expected: 64,
                actual: 0
            }
        }
    );
    assert_eq!(
        at_g2.output,
        MintStatementFromFieldsErrorReturn::InvalidG2 {
            index: 14,
            error: DecodeG2ErrorReturn::WrongLength {
                expected: 128,
                actual: 0
            }
        }
    );
}

/// Contract: the description's declared kinds are the sequence an encoding
///   concrete decodes against, each form's position carrying that form's kind.
/// Arrange: `TestForms`, whose chain, entitlement, and interval forms are
///   `Unsigned256` and whose identity form is `FixedBytes20`.
/// Act:     read `<MintStatementDescription<'_, P, TestForms> as
///   IEncodingContract>::FIELDS` under a real BN254 arkworks pairing.
/// Assert:  `FIELDS` equals, as a slice, `FixedBytes32`, `Unsigned16`,
///   `Unsigned256`, `FixedBytes20`, `FixedBytes32`, `FixedBytes32`,
///   `Unsigned256` four times, `Unsigned16`, `FixedBytes20` twice, `Bytes` six
///   times, `Unsigned64`, and `Bytes` five times.
#[test]
fn mint_statement_description_declares_its_field_kinds_in_transcript_order() {
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
        success.output,
        &[
            CanonicalFieldKind::FixedBytes32,
            CanonicalFieldKind::Unsigned16,
            CanonicalFieldKind::Unsigned256,
            CanonicalFieldKind::FixedBytes20,
            CanonicalFieldKind::FixedBytes32,
            CanonicalFieldKind::FixedBytes32,
            CanonicalFieldKind::Unsigned256,
            CanonicalFieldKind::Unsigned256,
            CanonicalFieldKind::Unsigned256,
            CanonicalFieldKind::Unsigned256,
            CanonicalFieldKind::Unsigned16,
            CanonicalFieldKind::FixedBytes20,
            CanonicalFieldKind::FixedBytes20,
            CanonicalFieldKind::Bytes,
            CanonicalFieldKind::Bytes,
            CanonicalFieldKind::Bytes,
            CanonicalFieldKind::Bytes,
            CanonicalFieldKind::Bytes,
            CanonicalFieldKind::Bytes,
            CanonicalFieldKind::Unsigned64,
            CanonicalFieldKind::Bytes,
            CanonicalFieldKind::Bytes,
            CanonicalFieldKind::Bytes,
            CanonicalFieldKind::Bytes,
            CanonicalFieldKind::Bytes,
        ]
    );
}
