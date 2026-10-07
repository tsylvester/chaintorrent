#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::super::render::provides::{
    SolidityMemberNameConstructorParamsOverrides, SolidityRecordMemberOverrides,
    SolidityRecordMembersConstructorParamsOverrides, build_solidity_member_name,
    build_solidity_record_member, build_solidity_record_members,
};
use super::interface::{
    DecodeTarget, DecodeTargetConstructorParams, DecodeTargetTryNewErrorReturn,
    RejectionVectorFromFieldsErrorReturn, RejectionVectorsDeps, RejectionVectorsParams,
    RejectionVectorsPayload, RejectionVectorsReturn,
};
use super::mock::{
    RejectionVectorOverrides, build_rejection_vector, build_rejection_vector_description,
};
use super::rejection_vectors;
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFields, FromFieldsParams, IEncodingContract,
    ToFieldsParams,
};
use hex::decode;
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingDeps, CreatePairingParamsOverrides,
    CreatePairingPayload, IPairingArithmetic, IPairingConsumer, IPairingReference, PairingConcrete,
    build_create_pairing_params, create_pairing,
};

struct RejectionVectorsPairingProbe;

impl IPairingConsumer for RejectionVectorsPairingProbe {
    type Output = RejectionVectorsReturn;

    fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        rejection_vectors::<P>(
            &RejectionVectorsDeps {
                pairing: &payload.adapter,
            },
            RejectionVectorsParams,
            RejectionVectorsPayload,
        )
    }
}

fn run_rejection_vectors(concrete: PairingConcrete) -> RejectionVectorsReturn {
    let Ok(success) = create_pairing(
        &CreatePairingDeps {
            consumer: RejectionVectorsPairingProbe,
        },
        build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(concrete),
            ..Default::default()
        }),
        CreatePairingPayload,
    ) else {
        panic!("the pairing concrete is admitted")
    };
    success.output
}

/// Contract: text equal to a variant's `as_str()` is admitted to that variant.
/// Arrange: the three admitted texts, "G1", "G2", and "Scalar".
/// Act:     DecodeTarget::try_new over each text.
/// Assert:  the admitted values are DecodeTarget::G1, DecodeTarget::G2, and
///   DecodeTarget::Scalar, and each value's as_str() is the text it was
///   admitted from.
#[test]
fn decode_target_admits_each_name_and_names_each_target() {
    // Arrange
    let texts = ["G1", "G2", "Scalar"];

    // Act
    let g1 = DecodeTarget::try_new(DecodeTargetConstructorParams {
        text: texts[0].to_string(),
    });
    let g2 = DecodeTarget::try_new(DecodeTargetConstructorParams {
        text: texts[1].to_string(),
    });
    let scalar = DecodeTarget::try_new(DecodeTargetConstructorParams {
        text: texts[2].to_string(),
    });

    // Assert
    let Ok(g1) = g1 else { panic!("G1 is admitted") };
    let Ok(g2) = g2 else { panic!("G2 is admitted") };
    let Ok(scalar) = scalar else {
        panic!("Scalar is admitted")
    };
    assert_eq!(g1, DecodeTarget::G1);
    assert_eq!(g1.as_str(), "G1");
    assert_eq!(g2, DecodeTarget::G2);
    assert_eq!(g2.as_str(), "G2");
    assert_eq!(scalar, DecodeTarget::Scalar);
    assert_eq!(scalar.as_str(), "Scalar");
}

/// Contract: text naming no variant takes the unknown branch with the text.
/// Arrange: the text "G3".
/// Act:     DecodeTarget::try_new over the text.
/// Assert:  the error is DecodeTargetTryNewErrorReturn::Unknown { text: "G3" }.
#[test]
fn decode_target_refuses_an_unknown_name() {
    // Arrange
    let params = DecodeTargetConstructorParams {
        text: "G3".to_string(),
    };

    // Act
    let result = DecodeTarget::try_new(params);

    // Assert
    assert_eq!(
        result,
        Err(DecodeTargetTryNewErrorReturn::Unknown { text: "G3".into() })
    );
}

/// Contract: the description's mapped branch carries the target's name as a
///   Text field and the input as a Bytes field, in member order.
/// Arrange: a rejection vector with target DecodeTarget::Scalar and input
///   [0x0A, 0x0B].
/// Act:     the description's to_fields over the record.
/// Assert:  the values are Text("Scalar") and Bytes([0x0A, 0x0B]) in that
///   order.
#[test]
fn rejection_vector_description_maps_target_and_input_in_order() {
    // Arrange
    let vector = build_rejection_vector(RejectionVectorOverrides {
        target: Some(DecodeTarget::Scalar),
        input: Some(vec![0x0A, 0x0B]),
    });
    let description = build_rejection_vector_description();

    // Act
    let Ok(success) = description.to_fields(ToFieldsParams, &vector);

    // Assert
    assert_eq!(
        success.fields.values,
        vec![
            CanonicalFieldValue::Text("Scalar".into()),
            CanonicalFieldValue::Bytes(vec![0x0A, 0x0B]),
        ]
    );
}

/// Contract: the description's rebuilt branch moves the Text field's admitted
///   target and the Bytes field's input back into the record in order.
/// Arrange: canonical fields Text("G1") and Bytes([0x0C]).
/// Act:     the description's fields_to_value over the fields.
/// Assert:  the described record holds DecodeTarget::G1 and [0x0C].
#[test]
fn rejection_vector_description_rebuilds_the_record_from_its_fields() {
    // Arrange
    let fields = CanonicalFields {
        values: vec![
            CanonicalFieldValue::Text("G1".into()),
            CanonicalFieldValue::Bytes(vec![0x0C]),
        ],
    };
    let description = build_rejection_vector_description();

    // Act
    let result = description.fields_to_value(FromFieldsParams, fields);

    // Assert
    let Ok(success) = result else {
        panic!("the fields rebuild the record")
    };
    assert_eq!(
        success.described,
        build_rejection_vector(RejectionVectorOverrides {
            target: Some(DecodeTarget::G1),
            input: Some(vec![0x0C]),
        })
    );
}

/// Contract: a field vector that does not convert into two values is refused
///   on the count branch.
/// Arrange: canonical fields of one Text value.
/// Act:     the description's fields_to_value over the fields.
/// Assert:  the error is FieldCount { expected: 2, actual: 1 }.
#[test]
fn rejection_vector_description_refuses_a_wrong_field_count() {
    // Arrange
    let fields = CanonicalFields {
        values: vec![CanonicalFieldValue::Text("G1".into())],
    };
    let description = build_rejection_vector_description();

    // Act
    let result = description.fields_to_value(FromFieldsParams, fields);

    // Assert
    let Err(error) = result else {
        panic!("the wrong field count is refused")
    };
    assert_eq!(
        error,
        RejectionVectorFromFieldsErrorReturn::FieldCount {
            expected: 2,
            actual: 1
        }
    );
}

/// Contract: the lowest index whose value is not the declared kind is refused
///   on the kind branch, expecting that kind at that index.
/// Arrange: canonical fields Text("G1") and Text("input").
/// Act:     the description's fields_to_value over the fields.
/// Assert:  the error is FieldKind { index: 1, expected:
///   CanonicalFieldKind::Bytes }.
#[test]
fn rejection_vector_description_refuses_a_field_of_the_wrong_kind() {
    // Arrange
    let fields = CanonicalFields {
        values: vec![
            CanonicalFieldValue::Text("G1".into()),
            CanonicalFieldValue::Text("input".into()),
        ],
    };
    let description = build_rejection_vector_description();

    // Act
    let result = description.fields_to_value(FromFieldsParams, fields);

    // Assert
    let Err(error) = result else {
        panic!("the wrong field kind is refused")
    };
    assert_eq!(
        error,
        RejectionVectorFromFieldsErrorReturn::FieldKind {
            index: 1,
            expected: CanonicalFieldKind::Bytes,
        }
    );
}

/// Contract: a Text field naming no decode target takes the target branch,
///   the try_new refusal carried unchanged.
/// Arrange: canonical fields Text("G3") and Bytes([]).
/// Act:     the description's fields_to_value over the fields.
/// Assert:  the error is Target(DecodeTargetTryNewErrorReturn::Unknown
///   { text: "G3" }).
#[test]
fn rejection_vector_description_refuses_an_unknown_target() {
    // Arrange
    let fields = CanonicalFields {
        values: vec![
            CanonicalFieldValue::Text("G3".into()),
            CanonicalFieldValue::Bytes(vec![]),
        ],
    };
    let description = build_rejection_vector_description();

    // Act
    let result = description.fields_to_value(FromFieldsParams, fields);

    // Assert
    let Err(error) = result else {
        panic!("the unknown target is refused")
    };
    assert_eq!(
        error,
        RejectionVectorFromFieldsErrorReturn::Target(DecodeTargetTryNewErrorReturn::Unknown {
            text: "G3".into()
        })
    );
}

/// Contract: the description's members branch pairs the member names target
///   and input with the FIELD_KINDS kind at the same index.
/// Arrange: the rejection-vector description.
/// Act:     members() on the description.
/// Assert:  the members equal target of kind CanonicalFieldKind::Text and
///   input of kind CanonicalFieldKind::Bytes.
#[test]
fn rejection_vector_description_declares_target_and_input_members() {
    // Arrange
    let member = |text: &str, kind: CanonicalFieldKind| {
        build_solidity_record_member(SolidityRecordMemberOverrides {
            name: Some(build_solidity_member_name(
                SolidityMemberNameConstructorParamsOverrides {
                    text: Some(text.to_string()),
                },
            )),
            kind: Some(kind),
        })
    };
    let description = build_rejection_vector_description();

    // Act
    let Ok(members) = description.members() else {
        panic!("the member names are admitted")
    };

    // Assert
    assert_eq!(
        members,
        build_solidity_record_members(SolidityRecordMembersConstructorParamsOverrides {
            members: Some(vec![
                member("target", CanonicalFieldKind::Text),
                member("input", CanonicalFieldKind::Bytes),
            ]),
        })
    );
}

/// Contract: on a pairing whose first group is the whole curve, the computed
///   branch emits no first-group record — only the second group's
///   outside-the-subgroup encoding and the scalar field's order.
/// Arrange: none; the kind is enumerated and holds no count.
/// Act:     rejection_vectors through the probe over
///   PairingConcrete::Bn254Arkworks.
/// Assert:  records holds two records whose targets are DecodeTarget::G2 and
///   DecodeTarget::Scalar in that order.
#[test]
fn rejection_vectors_on_bn254_enumerate_the_second_group_and_the_scalar_only() {
    // Arrange
    let concrete = PairingConcrete::Bn254Arkworks;

    // Act
    let result = run_rejection_vectors(concrete);

    // Assert
    let Ok(success) = result else {
        panic!("the candidates are enumerated")
    };
    assert_eq!(
        success
            .records
            .iter()
            .map(|record| record.target)
            .collect::<Vec<DecodeTarget>>(),
        vec![DecodeTarget::G2, DecodeTarget::Scalar]
    );
}

/// Contract: on BN254 the computed branch carries the second group's
///   outside-the-subgroup point in its EIP-197 encoding and the scalar
///   field's order as the non-canonical scalar.
/// Arrange: none; the kind is enumerated and holds no count.
/// Act:     rejection_vectors through the probe over
///   PairingConcrete::Bn254Arkworks.
/// Assert:  the G2 record's input holds 128 bytes and the Scalar record's
///   input equals
///   0x30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001.
#[test]
fn rejection_vectors_on_bn254_carry_each_candidate_in_its_precompile_encoding() {
    // Arrange
    let concrete = PairingConcrete::Bn254Arkworks;

    // Act
    let result = run_rejection_vectors(concrete);

    // Assert
    let Ok(success) = result else {
        panic!("the candidates are enumerated")
    };
    assert_eq!(success.records[0].input.len(), 128);
    assert_eq!(
        success.records[1].input,
        decode("30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001")
            .expect("the field order is hex")
    );
}

/// Contract: on a pairing whose first group is not the whole curve, the
///   computed branch emits each source group's outside-the-subgroup encoding
///   and the scalar field's order.
/// Arrange: none; the kind is enumerated and holds no count.
/// Act:     rejection_vectors through the probe over
///   PairingConcrete::Bls12381Arkworks.
/// Assert:  records holds three records whose targets are DecodeTarget::G1,
///   DecodeTarget::G2, and DecodeTarget::Scalar in that order.
#[test]
fn rejection_vectors_on_bls12_381_enumerate_both_groups_and_the_scalar() {
    // Arrange
    let concrete = PairingConcrete::Bls12381Arkworks;

    // Act
    let result = run_rejection_vectors(concrete);

    // Assert
    let Ok(success) = result else {
        panic!("the candidates are enumerated")
    };
    assert_eq!(
        success
            .records
            .iter()
            .map(|record| record.target)
            .collect::<Vec<DecodeTarget>>(),
        vec![DecodeTarget::G1, DecodeTarget::G2, DecodeTarget::Scalar]
    );
}

/// Contract: on BLS12-381 the computed branch carries each source group's
///   outside-the-subgroup point in its EIP-2537 encoding and the scalar
///   field's order as the non-canonical scalar.
/// Arrange: none; the kind is enumerated and holds no count.
/// Act:     rejection_vectors through the probe over
///   PairingConcrete::Bls12381Arkworks.
/// Assert:  the G1 record's input holds 128 bytes, the G2 record's 256, and
///   the Scalar record's input equals
///   0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001.
#[test]
fn rejection_vectors_on_bls12_381_carry_each_candidate_in_its_precompile_encoding() {
    // Arrange
    let concrete = PairingConcrete::Bls12381Arkworks;

    // Act
    let result = run_rejection_vectors(concrete);

    // Assert
    let Ok(success) = result else {
        panic!("the candidates are enumerated")
    };
    assert_eq!(success.records[0].input.len(), 128);
    assert_eq!(success.records[1].input.len(), 256);
    assert_eq!(
        success.records[2].input,
        decode("73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001")
            .expect("the field order is hex")
    );
}
