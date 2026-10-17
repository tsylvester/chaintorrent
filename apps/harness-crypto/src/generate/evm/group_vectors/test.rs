#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::super::render::provides::{
    SolidityMemberNameConstructorParamsOverrides, SolidityRecordMemberOverrides,
    SolidityRecordMembersConstructorParamsOverrides, build_solidity_member_name,
    build_solidity_record_member, build_solidity_record_members,
};
use super::group_vectors;
use super::interface::*;
use super::mock::*;
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFieldsOverrides, IEncodingContract,
    build_canonical_fields, build_from_fields_params, build_to_fields_params,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParamsOverrides,
    CreatePairingPayload, IPairingArithmetic, IPairingConsumer, IPairingReference,
    MockIPairingAdapterFailureMode, PairingConcrete, SampleUniformScalarErrorReturn,
    build_create_pairing_deps, build_create_pairing_params, create_pairing,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParamsOverrides, CreateRandomSourcePayload,
    FillBytesErrorReturn, IRandomSourceAdapter, MockIRandomSourceAdapterFailureMode,
    RandomSourceKind, build_create_random_source_params, create_random_source,
};

/// Contract: `NonZeroU32::new(params.count)` is `Some` → `Ok(VectorCount {
///   count })`, and `get()` returns the count.
/// Arrange:  `build_vector_count_constructor_params` with `count` `3`.
/// Act:      `VectorCount::try_new` over it.
/// Assert:   `value.get()` equals `3`.
/// Collaborators: none — the constructor is a leaf.
#[test]
fn vector_count_admits_a_positive_count() {
    // Arrange
    let params = build_vector_count_constructor_params(VectorCountConstructorParamsOverrides {
        count: Some(3),
    });

    // Act
    let value = VectorCount::try_new(params).expect("a positive count is admitted");

    // Assert
    assert_eq!(value.get(), 3);
    // vector_count_admits_a_positive_count
}

/// Contract: `NonZeroU32::new(params.count)` is `None` →
///   `Err(VectorCountTryNewErrorReturn::Zero)`.
/// Arrange:  `build_vector_count_constructor_params` with `count` `0`.
/// Act:      `VectorCount::try_new` over it.
/// Assert:   `error` equals `VectorCountTryNewErrorReturn::Zero`.
/// Collaborators: none — the constructor is a leaf.
#[test]
fn vector_count_refuses_zero() {
    // Arrange
    let params = build_vector_count_constructor_params(VectorCountConstructorParamsOverrides {
        count: Some(0),
    });

    // Act
    let result = VectorCount::try_new(params);

    // Assert
    assert_eq!(result, Err(VectorCountTryNewErrorReturn::Zero));
    // vector_count_refuses_zero
}

/// Contract: `NonZeroU32::new(params.count)` is `Some` → `Ok(MsmTermCount {
///   count })`, and `get()` returns the count.
/// Arrange:  `build_msm_term_count_constructor_params` with `count` `5`.
/// Act:      `MsmTermCount::try_new` over it.
/// Assert:   `value.get()` equals `5`.
/// Collaborators: none — the constructor is a leaf.
#[test]
fn msm_term_count_admits_a_positive_count() {
    // Arrange
    let params = build_msm_term_count_constructor_params(MsmTermCountConstructorParamsOverrides {
        count: Some(5),
    });

    // Act
    let value = MsmTermCount::try_new(params).expect("a positive count is admitted");

    // Assert
    assert_eq!(value.get(), 5);
    // msm_term_count_admits_a_positive_count
}

/// Contract: `NonZeroU32::new(params.count)` is `None` →
///   `Err(MsmTermCountTryNewErrorReturn::Zero)`.
/// Arrange:  `build_msm_term_count_constructor_params` with `count` `0`.
/// Act:      `MsmTermCount::try_new` over it.
/// Assert:   `error` equals `MsmTermCountTryNewErrorReturn::Zero`.
/// Collaborators: none — the constructor is a leaf.
#[test]
fn msm_term_count_refuses_zero() {
    // Arrange
    let params = build_msm_term_count_constructor_params(MsmTermCountConstructorParamsOverrides {
        count: Some(0),
    });

    // Act
    let result = MsmTermCount::try_new(params);

    // Assert
    assert_eq!(result, Err(MsmTermCountTryNewErrorReturn::Zero));
    // msm_term_count_refuses_zero
}

/// Contract: `to_fields` → `Ok(ToFieldsSuccessReturn { fields })` whose
///   `values` are `CanonicalFieldValue::Bytes` of clones of `draws`, `input`,
///   and `output` in that order.
/// Arrange:  `build_group_operation_vector` with `draws` `vec![0x0A]`,
///   `input` `vec![0x0B, 0x0C]`, and `output` `vec![0x0D]`,
///   `build_group_operation_vector_description(Default::default())`, and
///   `build_to_fields_params(Default::default())`.
/// Act:      `to_fields`.
/// Assert:   the values equal `CanonicalFieldValue::Bytes(vec![0x0A])`,
///   `CanonicalFieldValue::Bytes(vec![0x0B, 0x0C])`, and
///   `CanonicalFieldValue::Bytes(vec![0x0D])` in that order.
/// Collaborators: none — the description is a leaf.
#[test]
fn group_operation_vector_description_maps_draws_input_and_output_to_byte_strings_in_order() {
    // Arrange
    let vector = build_group_operation_vector(GroupOperationVectorOverrides {
        draws: Some(vec![0x0A]),
        input: Some(vec![0x0B, 0x0C]),
        output: Some(vec![0x0D]),
    });
    let description = build_group_operation_vector_description(Default::default());
    let params = build_to_fields_params(Default::default());

    // Act
    let Ok(success) = description.to_fields(params, &vector);

    // Assert
    assert_eq!(
        success.fields.values,
        vec![
            CanonicalFieldValue::Bytes(vec![0x0A]),
            CanonicalFieldValue::Bytes(vec![0x0B, 0x0C]),
            CanonicalFieldValue::Bytes(vec![0x0D]),
        ]
    );
    // group_operation_vector_description_maps_draws_input_and_output_to_byte_strings_in_order
}

/// Contract: `payload.values` converts into the field array and every value is
///   `CanonicalFieldValue::Bytes` → `Ok(FromFieldsSuccessReturn { described:
///   GroupOperationVector { draws, input, output } })`, each moved from its
///   field.
/// Arrange:  `build_canonical_fields` with `values` `Bytes(vec![0x0A])`,
///   `Bytes(vec![0x0B, 0x0C])`, and `Bytes(vec![0x0D])`, the description from
///   `build_group_operation_vector_description(Default::default())`, and
///   `build_from_fields_params(Default::default())`.
/// Act:      `fields_to_value`.
/// Assert:   `described` equals `build_group_operation_vector` with those
///   three values.
/// Collaborators: none — the description is a leaf.
#[test]
fn group_operation_vector_description_rebuilds_the_record_from_its_fields() {
    // Arrange
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Bytes(vec![0x0A]),
            CanonicalFieldValue::Bytes(vec![0x0B, 0x0C]),
            CanonicalFieldValue::Bytes(vec![0x0D]),
        ]),
    });
    let description = build_group_operation_vector_description(Default::default());
    let params = build_from_fields_params(Default::default());

    // Act
    let success = description
        .fields_to_value(params, fields)
        .expect("fields of the declared kinds rebuild the record");

    // Assert
    assert_eq!(
        success.described,
        build_group_operation_vector(GroupOperationVectorOverrides {
            draws: Some(vec![0x0A]),
            input: Some(vec![0x0B, 0x0C]),
            output: Some(vec![0x0D]),
        })
    );
    // group_operation_vector_description_rebuilds_the_record_from_its_fields
}

/// Contract: `payload.values` does not convert into the field array →
///   `Err(GroupOperationVectorFromFieldsErrorReturn::FieldCount { expected:
///   VECTOR_RECORD_FIELD_COUNT, actual })`.
/// Arrange:  `build_canonical_fields` with `values` two `Bytes` values.
/// Act:      `fields_to_value`.
/// Assert:   `error` equals `GroupOperationVectorFromFieldsErrorReturn::
///   FieldCount { expected: 3, actual: 2 }`.
/// Collaborators: none — the description is a leaf.
#[test]
fn group_operation_vector_description_refuses_a_wrong_field_count() {
    // Arrange
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Bytes(vec![0x0A]),
            CanonicalFieldValue::Bytes(vec![0x0B]),
        ]),
    });
    let description = build_group_operation_vector_description(Default::default());
    let params = build_from_fields_params(Default::default());

    // Act
    let error = description
        .fields_to_value(params, fields)
        .err()
        .expect("a wrong field count is refused");

    // Assert
    assert_eq!(
        error,
        GroupOperationVectorFromFieldsErrorReturn::FieldCount {
            expected: 3,
            actual: 2,
        }
    );
    // group_operation_vector_description_refuses_a_wrong_field_count
}

/// Contract: the lowest index `i` whose value is not `CanonicalFieldValue::
///   Bytes` → `Err(GroupOperationVectorFromFieldsErrorReturn::FieldKind {
///   index: i, expected: CanonicalFieldKind::Bytes })`.
/// Arrange:  `build_canonical_fields` with `values` `Bytes(vec![0x0A])`,
///   `FixedBytes32([0x0B; 32])`, and `FixedBytes32([0x0C; 32])`, so the
///   indexes `1` and `2` both hold a wrong kind.
/// Act:      `fields_to_value`.
/// Assert:   `error` equals `GroupOperationVectorFromFieldsErrorReturn::
///   FieldKind { index: 1, expected: CanonicalFieldKind::Bytes }`.
/// Collaborators: none — the description is a leaf.
#[test]
fn group_operation_vector_description_refuses_the_lowest_index_of_the_wrong_kind() {
    // Arrange
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Bytes(vec![0x0A]),
            CanonicalFieldValue::FixedBytes32([0x0B; 32]),
            CanonicalFieldValue::FixedBytes32([0x0C; 32]),
        ]),
    });
    let description = build_group_operation_vector_description(Default::default());
    let params = build_from_fields_params(Default::default());

    // Act
    let error = description
        .fields_to_value(params, fields)
        .err()
        .expect("a field of the wrong kind is refused");

    // Assert
    assert_eq!(
        error,
        GroupOperationVectorFromFieldsErrorReturn::FieldKind {
            index: 1,
            expected: CanonicalFieldKind::Bytes,
        }
    );
    // group_operation_vector_description_refuses_the_lowest_index_of_the_wrong_kind
}

/// Contract: `to_fields` → `Ok(ToFieldsSuccessReturn { fields })` whose
///   `values` are `CanonicalFieldValue::Bytes` of clones of `draws` and
///   `input` and `CanonicalFieldValue::FixedBytes32` of `output`.
/// Arrange:  `build_pairing_check_vector` with `draws` `vec![0x0A]`, `input`
///   `vec![0x0B]`, and `output` `[0x0C; 32]`,
///   `build_pairing_check_vector_description(Default::default())`, and
///   `build_to_fields_params(Default::default())`.
/// Act:      `to_fields`.
/// Assert:   the values equal `CanonicalFieldValue::Bytes(vec![0x0A])`,
///   `CanonicalFieldValue::Bytes(vec![0x0B])`, and
///   `CanonicalFieldValue::FixedBytes32([0x0C; 32])` in that order.
/// Collaborators: none — the description is a leaf.
#[test]
fn pairing_check_vector_description_maps_its_output_to_a_fixed_32_byte_field() {
    // Arrange
    let vector = build_pairing_check_vector(PairingCheckVectorOverrides {
        draws: Some(vec![0x0A]),
        input: Some(vec![0x0B]),
        output: Some([0x0C; 32]),
    });
    let description = build_pairing_check_vector_description(Default::default());
    let params = build_to_fields_params(Default::default());

    // Act
    let Ok(success) = description.to_fields(params, &vector);

    // Assert
    assert_eq!(
        success.fields.values,
        vec![
            CanonicalFieldValue::Bytes(vec![0x0A]),
            CanonicalFieldValue::Bytes(vec![0x0B]),
            CanonicalFieldValue::FixedBytes32([0x0C; 32]),
        ]
    );
    // pairing_check_vector_description_maps_its_output_to_a_fixed_32_byte_field
}

/// Contract: `payload.values` converts into the field array and each value is
///   the kind `PAIRING_CHECK_VECTOR_FIELD_KINDS` holds at its index →
///   `Ok(FromFieldsSuccessReturn { described: PairingCheckVector { draws,
///   input, output } })`, each moved from its field.
/// Arrange:  `build_canonical_fields` with `values` `Bytes(vec![0x0A])`,
///   `Bytes(vec![0x0B])`, and `FixedBytes32([0x0C; 32])`, the description from
///   `build_pairing_check_vector_description(Default::default())`, and
///   `build_from_fields_params(Default::default())`.
/// Act:      `fields_to_value`.
/// Assert:   `described` equals `build_pairing_check_vector` with those three
///   values.
/// Collaborators: none — the description is a leaf.
#[test]
fn pairing_check_vector_description_rebuilds_the_record_from_its_fields() {
    // Arrange
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Bytes(vec![0x0A]),
            CanonicalFieldValue::Bytes(vec![0x0B]),
            CanonicalFieldValue::FixedBytes32([0x0C; 32]),
        ]),
    });
    let description = build_pairing_check_vector_description(Default::default());
    let params = build_from_fields_params(Default::default());

    // Act
    let success = description
        .fields_to_value(params, fields)
        .expect("fields of the declared kinds rebuild the record");

    // Assert
    assert_eq!(
        success.described,
        build_pairing_check_vector(PairingCheckVectorOverrides {
            draws: Some(vec![0x0A]),
            input: Some(vec![0x0B]),
            output: Some([0x0C; 32]),
        })
    );
    // pairing_check_vector_description_rebuilds_the_record_from_its_fields
}

/// Contract: `payload.values` does not convert into the field array →
///   `Err(PairingCheckVectorFromFieldsErrorReturn::FieldCount { expected:
///   VECTOR_RECORD_FIELD_COUNT, actual })`.
/// Arrange:  `build_canonical_fields` with `values` two `Bytes` values.
/// Act:      `fields_to_value`.
/// Assert:   `error` equals `PairingCheckVectorFromFieldsErrorReturn::
///   FieldCount { expected: 3, actual: 2 }`.
/// Collaborators: none — the description is a leaf.
#[test]
fn pairing_check_vector_description_refuses_a_wrong_field_count() {
    // Arrange
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Bytes(vec![0x0A]),
            CanonicalFieldValue::Bytes(vec![0x0B]),
        ]),
    });
    let description = build_pairing_check_vector_description(Default::default());
    let params = build_from_fields_params(Default::default());

    // Act
    let error = description
        .fields_to_value(params, fields)
        .err()
        .expect("a wrong field count is refused");

    // Assert
    assert_eq!(
        error,
        PairingCheckVectorFromFieldsErrorReturn::FieldCount {
            expected: 3,
            actual: 2,
        }
    );
    // pairing_check_vector_description_refuses_a_wrong_field_count
}

/// Contract: the lowest index `i` whose value is not the kind
///   `PAIRING_CHECK_VECTOR_FIELD_KINDS` holds at `i` →
///   `Err(PairingCheckVectorFromFieldsErrorReturn::FieldKind { index: i,
///   expected })`, `expected` that kind.
/// Arrange:  `build_canonical_fields` with `values` `Bytes(vec![0x0A])`,
///   `Bytes(vec![0x0B])`, and `Bytes(vec![0x0C])`.
/// Act:      `fields_to_value`.
/// Assert:   `error` equals `PairingCheckVectorFromFieldsErrorReturn::
///   FieldKind { index: 2, expected: CanonicalFieldKind::FixedBytes32 }`.
/// Collaborators: none — the description is a leaf.
#[test]
fn pairing_check_vector_description_refuses_a_byte_string_output() {
    // Arrange
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Bytes(vec![0x0A]),
            CanonicalFieldValue::Bytes(vec![0x0B]),
            CanonicalFieldValue::Bytes(vec![0x0C]),
        ]),
    });
    let description = build_pairing_check_vector_description(Default::default());
    let params = build_from_fields_params(Default::default());

    // Act
    let error = description
        .fields_to_value(params, fields)
        .err()
        .expect("an output of the wrong kind is refused");

    // Assert
    assert_eq!(
        error,
        PairingCheckVectorFromFieldsErrorReturn::FieldKind {
            index: 2,
            expected: CanonicalFieldKind::FixedBytes32,
        }
    );
    // pairing_check_vector_description_refuses_a_byte_string_output
}

/// Contract: the lowest index `i` whose value is not the kind
///   `PAIRING_CHECK_VECTOR_FIELD_KINDS` holds at `i` →
///   `Err(PairingCheckVectorFromFieldsErrorReturn::FieldKind { index: i,
///   expected })`, `expected` that kind.
/// Arrange:  `build_canonical_fields` with `values` `Bytes(vec![0x0A])`,
///   `FixedBytes32([0x0B; 32])`, and `Bytes(vec![0x0C])`, so the indexes `1`
///   and `2` both hold a wrong kind.
/// Act:      `fields_to_value`.
/// Assert:   `error` equals `PairingCheckVectorFromFieldsErrorReturn::
///   FieldKind { index: 1, expected: CanonicalFieldKind::Bytes }`.
/// Collaborators: none — the description is a leaf.
#[test]
fn pairing_check_vector_description_refuses_the_lowest_index_of_the_wrong_kind() {
    // Arrange
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Bytes(vec![0x0A]),
            CanonicalFieldValue::FixedBytes32([0x0B; 32]),
            CanonicalFieldValue::Bytes(vec![0x0C]),
        ]),
    });
    let description = build_pairing_check_vector_description(Default::default());
    let params = build_from_fields_params(Default::default());

    // Act
    let error = description
        .fields_to_value(params, fields)
        .err()
        .expect("a field of the wrong kind is refused");

    // Assert
    assert_eq!(
        error,
        PairingCheckVectorFromFieldsErrorReturn::FieldKind {
            index: 1,
            expected: CanonicalFieldKind::Bytes,
        }
    );
    // pairing_check_vector_description_refuses_the_lowest_index_of_the_wrong_kind
}

/// Contract: each name `SolidityMemberName::try_new` admits is paired with the
///   description's `FIELDS` kind at its index and
///   `SolidityRecordMembers::try_new` admits the members → `Ok` of the
///   members.
/// Arrange:  `build_group_operation_vector_description(Default::default())`.
/// Act:      `members()`.
/// Assert:   the result equals `Ok` of `build_solidity_record_members` over
///   the members `draws`, `input`, and `output`, each of kind
///   `CanonicalFieldKind::Bytes`, each member through
///   `build_solidity_record_member` with its name from
///   `build_solidity_member_name`.
/// Collaborators: none — the description is a leaf.
#[test]
fn group_operation_vector_description_declares_draws_input_and_output_members() {
    // Arrange
    let description = build_group_operation_vector_description(Default::default());
    let expected = build_solidity_record_members(SolidityRecordMembersConstructorParamsOverrides {
        members: Some(vec![
            build_solidity_record_member(SolidityRecordMemberOverrides {
                name: Some(build_solidity_member_name(
                    SolidityMemberNameConstructorParamsOverrides {
                        text: Some("draws".to_string()),
                    },
                )),
                kind: Some(CanonicalFieldKind::Bytes),
            }),
            build_solidity_record_member(SolidityRecordMemberOverrides {
                name: Some(build_solidity_member_name(
                    SolidityMemberNameConstructorParamsOverrides {
                        text: Some("input".to_string()),
                    },
                )),
                kind: Some(CanonicalFieldKind::Bytes),
            }),
            build_solidity_record_member(SolidityRecordMemberOverrides {
                name: Some(build_solidity_member_name(
                    SolidityMemberNameConstructorParamsOverrides {
                        text: Some("output".to_string()),
                    },
                )),
                kind: Some(CanonicalFieldKind::Bytes),
            }),
        ]),
    });

    // Act
    let result = description.members();

    // Assert
    assert_eq!(result, Ok(expected));
    // group_operation_vector_description_declares_draws_input_and_output_members
}

/// Contract: each name `SolidityMemberName::try_new` admits is paired with the
///   description's `FIELDS` kind at its index and
///   `SolidityRecordMembers::try_new` admits the members → `Ok` of the
///   members.
/// Arrange:  `build_pairing_check_vector_description(Default::default())`.
/// Act:      `members()`.
/// Assert:   the result equals `Ok` of `build_solidity_record_members` over
///   the members `draws` and `input` of kind `CanonicalFieldKind::Bytes` and
///   `output` of kind `CanonicalFieldKind::FixedBytes32`, each member through
///   `build_solidity_record_member` with its name from
///   `build_solidity_member_name`.
/// Collaborators: none — the description is a leaf.
#[test]
fn pairing_check_vector_description_declares_draws_input_and_output_members() {
    // Arrange
    let description = build_pairing_check_vector_description(Default::default());
    let expected = build_solidity_record_members(SolidityRecordMembersConstructorParamsOverrides {
        members: Some(vec![
            build_solidity_record_member(SolidityRecordMemberOverrides {
                name: Some(build_solidity_member_name(
                    SolidityMemberNameConstructorParamsOverrides {
                        text: Some("draws".to_string()),
                    },
                )),
                kind: Some(CanonicalFieldKind::Bytes),
            }),
            build_solidity_record_member(SolidityRecordMemberOverrides {
                name: Some(build_solidity_member_name(
                    SolidityMemberNameConstructorParamsOverrides {
                        text: Some("input".to_string()),
                    },
                )),
                kind: Some(CanonicalFieldKind::Bytes),
            }),
            build_solidity_record_member(SolidityRecordMemberOverrides {
                name: Some(build_solidity_member_name(
                    SolidityMemberNameConstructorParamsOverrides {
                        text: Some("output".to_string()),
                    },
                )),
                kind: Some(CanonicalFieldKind::FixedBytes32),
            }),
        ]),
    });

    // Act
    let result = description.members();

    // Assert
    assert_eq!(result, Ok(expected));
    // pairing_check_vector_description_declares_draws_input_and_output_members
}

/// Contract: `FirstGroupOnly` with `params.second_group` `Some(_)` →
///   `Err(GroupVectorsErrorReturn::SecondGroupSettingsForFirstGroupOnly)`.
/// Arrange:  the default mock pairing concrete, whose declaration is
///   `FirstGroupOnly`, and params from `build_group_vectors_params` with
///   `second_group` `Some(Some(build_second_group_vector_counts(Default::
///   default())))`.
/// Act:      `group_vectors` in the consumer.
/// Assert:   `result` equals `Err(GroupVectorsErrorReturn::
///   SecondGroupSettingsForFirstGroupOnly)`.
/// Collaborators: the random source's and pairing's mock concretes in their
///   succeeding modes through `create_random_source` and `create_pairing`.
#[test]
fn group_vectors_refuse_second_group_settings_on_a_first_group_only_pairing() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides { kind: None }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;

    struct Consumer {
        params: GroupVectorsParams,
        random: Box<dyn IRandomSourceAdapter>,
    }

    impl IPairingConsumer for Consumer {
        type Output = GroupVectorsReturn;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Act
            group_vectors::<P>(
                &build_group_vectors_deps(
                    &payload.adapter,
                    self.random.as_ref(),
                    Default::default(),
                ),
                self.params,
                build_group_vectors_payload(Default::default()),
            )
        }
    }

    let consumer = Consumer {
        params: build_group_vectors_params(GroupVectorsParamsOverrides {
            second_group: Some(Some(build_second_group_vector_counts(Default::default()))),
            ..Default::default()
        }),
        random: source,
    };

    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: None,
            ..Default::default()
        }),
        CreatePairingPayload,
    );
    let success = created.expect("the pairing factory ran the consumer");

    // Assert
    assert_eq!(
        success.output,
        Err(GroupVectorsErrorReturn::SecondGroupSettingsForFirstGroupOnly)
    );
    // group_vectors_refuse_second_group_settings_on_a_first_group_only_pairing
}

/// Contract: `deps.random.fill_bytes` fails → `Err(GroupVectorsErrorReturn::
///   FillBytes(error))`, the error unchanged.
/// Arrange:  the random source with `kind` `RandomSourceKind::Mock(
///   MockIRandomSourceAdapterFailureMode::FillBytesRefused)`.
/// Act:      `group_vectors` in the consumer.
/// Assert:   `result` equals `Err(GroupVectorsErrorReturn::FillBytes(
///   FillBytesErrorReturn::MockIRandomSourceAdapter))`.
/// Collaborators: the random source's mock concrete in its `FillBytesRefused`
///   mode through `create_random_source`, and the pairing's mock concrete in
///   its succeeding mode through `create_pairing`.
#[test]
fn group_vectors_return_the_fill_refusal_unchanged() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::Mock(
                MockIRandomSourceAdapterFailureMode::FillBytesRefused,
            )),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;

    struct Consumer {
        params: GroupVectorsParams,
        random: Box<dyn IRandomSourceAdapter>,
    }

    impl IPairingConsumer for Consumer {
        type Output = GroupVectorsReturn;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Act
            group_vectors::<P>(
                &build_group_vectors_deps(
                    &payload.adapter,
                    self.random.as_ref(),
                    Default::default(),
                ),
                self.params,
                build_group_vectors_payload(Default::default()),
            )
        }
    }

    let consumer = Consumer {
        params: build_group_vectors_params(Default::default()),
        random: source,
    };

    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: None,
            ..Default::default()
        }),
        CreatePairingPayload,
    );
    let success = created.expect("the pairing factory ran the consumer");

    // Assert
    assert_eq!(
        success.output,
        Err(GroupVectorsErrorReturn::FillBytes(
            FillBytesErrorReturn::MockIRandomSourceAdapter
        ))
    );
    // group_vectors_return_the_fill_refusal_unchanged
}

/// Contract: `sample_from_uniform_bytes` fails → `Err(GroupVectorsErrorReturn
///   ::SampleScalar(error))`, the error unchanged.
/// Arrange:  the pairing with `concrete` `PairingConcrete::Mock(
///   MockIPairingAdapterFailureMode::SampleScalarWrongLength)`.
/// Act:      `group_vectors` in the consumer.
/// Assert:   `result` equals `Err(GroupVectorsErrorReturn::SampleScalar(
///   SampleUniformScalarErrorReturn::WrongLength { expected: 16, actual: 16
///   }))`, the error the pairing mock concrete's contract states for that
///   mode.
/// Collaborators: the random source's mock concrete in its succeeding mode
///   through `create_random_source`, and the pairing's mock concrete in its
///   `SampleScalarWrongLength` mode through `create_pairing`.
#[test]
fn group_vectors_return_the_sampling_refusal_unchanged() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides { kind: None }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;

    struct Consumer {
        params: GroupVectorsParams,
        random: Box<dyn IRandomSourceAdapter>,
    }

    impl IPairingConsumer for Consumer {
        type Output = GroupVectorsReturn;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Act
            group_vectors::<P>(
                &build_group_vectors_deps(
                    &payload.adapter,
                    self.random.as_ref(),
                    Default::default(),
                ),
                self.params,
                build_group_vectors_payload(Default::default()),
            )
        }
    }

    let consumer = Consumer {
        params: build_group_vectors_params(Default::default()),
        random: source,
    };

    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(PairingConcrete::Mock(
                MockIPairingAdapterFailureMode::SampleScalarWrongLength,
            )),
            ..Default::default()
        }),
        CreatePairingPayload,
    );
    let success = created.expect("the pairing factory ran the consumer");

    // Assert
    assert_eq!(
        success.output,
        Err(GroupVectorsErrorReturn::SampleScalar(
            SampleUniformScalarErrorReturn::WrongLength {
                expected: 16,
                actual: 16,
            }
        ))
    );
    // group_vectors_return_the_sampling_refusal_unchanged
}

/// Contract: every first-group kind holds exactly its configured count of
///   records, and `second_group` is absent where `params.second_group` is
///   `None`.
/// Arrange:  `first_group` from `build_first_group_vector_counts` with
///   `g1_add` of `3`, `g1_mul` of `2`, and `pairing_check` of `4`, each from
///   `build_vector_count`.
/// Act:      `group_vectors` in the consumer.
/// Assert:   `first_group.g1_add`, `g1_mul`, and `pairing_check` hold `3`,
///   `2`, and `4` records and `second_group` is `None`.
/// Collaborators: the random source's and pairing's mock concretes in their
///   succeeding modes through `create_random_source` and `create_pairing`.
#[test]
fn group_vectors_emit_each_first_group_kind_at_its_configured_count() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides { kind: None }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;

    struct Consumer {
        params: GroupVectorsParams,
        random: Box<dyn IRandomSourceAdapter>,
    }

    impl IPairingConsumer for Consumer {
        type Output = GroupVectorsReturn;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Act
            group_vectors::<P>(
                &build_group_vectors_deps(
                    &payload.adapter,
                    self.random.as_ref(),
                    Default::default(),
                ),
                self.params,
                build_group_vectors_payload(Default::default()),
            )
        }
    }

    let consumer = Consumer {
        params: build_group_vectors_params(GroupVectorsParamsOverrides {
            first_group: Some(build_first_group_vector_counts(
                FirstGroupVectorCountsOverrides {
                    g1_add: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(3),
                    })),
                    g1_mul: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(2),
                    })),
                    pairing_check: Some(build_vector_count(
                        VectorCountConstructorParamsOverrides { count: Some(4) },
                    )),
                },
            )),
            ..Default::default()
        }),
        random: source,
    };

    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: None,
            ..Default::default()
        }),
        CreatePairingPayload,
    );
    let success = created.expect("the pairing factory ran the consumer");

    // Assert
    let vectors = success.output.expect("the counts are admitted");
    assert_eq!(vectors.first_group.g1_add.len(), 3);
    assert_eq!(vectors.first_group.g1_mul.len(), 2);
    assert_eq!(vectors.first_group.pairing_check.len(), 4);
    assert_eq!(vectors.second_group, None);
    // group_vectors_emit_each_first_group_kind_at_its_configured_count
}

/// Contract: first-group multiplication draws `a` then `s`, and `input` is
///   the encoding of `point` then the encoding of `s`.
/// Arrange:  the default counts, so each `g1_mul` record holds draws `a` and
///   `s` of one width each.
/// Act:      `group_vectors` in the consumer.
/// Assert:   each `g1_mul` record's `input` ends with the second half of its
///   `draws`.
/// Collaborators: the random source's and pairing's mock concretes in their
///   succeeding modes through `create_random_source` and `create_pairing`.
#[test]
fn group_vectors_g1_multiplication_ends_its_input_with_its_multiplier() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides { kind: None }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;

    struct Consumer {
        params: GroupVectorsParams,
        random: Box<dyn IRandomSourceAdapter>,
    }

    impl IPairingConsumer for Consumer {
        type Output = GroupVectorsReturn;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Act
            group_vectors::<P>(
                &build_group_vectors_deps(
                    &payload.adapter,
                    self.random.as_ref(),
                    Default::default(),
                ),
                self.params,
                build_group_vectors_payload(Default::default()),
            )
        }
    }

    let consumer = Consumer {
        params: build_group_vectors_params(Default::default()),
        random: source,
    };

    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: None,
            ..Default::default()
        }),
        CreatePairingPayload,
    );
    let success = created.expect("the pairing factory ran the consumer");

    // Assert
    let vectors = success.output.expect("the counts are admitted");
    for record in &vectors.first_group.g1_mul {
        let half = record.draws.len() / 2;
        assert!(record.input.ends_with(&record.draws[half..]));
    }
    // group_vectors_g1_multiplication_ends_its_input_with_its_multiplier
}

/// Contract: the index is even → `t` cancels the first pair and `output` is
///   the 32-byte word of zeros whose last byte is `0x01`; the index is odd →
///   `t` is the drawn `c` and `output` is thirty-two zero bytes.
/// Arrange:  `pairing_check` of `4`.
/// Act:      `group_vectors` in the consumer.
/// Assert:   the records at indices `0` and `2` have `output` thirty-one zero
///   bytes followed by `0x01`, and the records at indices `1` and `3` have
///   `output` thirty-two zero bytes.
/// Collaborators: the random source's and pairing's mock concretes in their
///   succeeding modes through `create_random_source` and `create_pairing`.
#[test]
fn group_vectors_pairing_checks_alternate_passing_and_failing_verdicts() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides { kind: None }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;

    struct Consumer {
        params: GroupVectorsParams,
        random: Box<dyn IRandomSourceAdapter>,
    }

    impl IPairingConsumer for Consumer {
        type Output = GroupVectorsReturn;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Act
            group_vectors::<P>(
                &build_group_vectors_deps(
                    &payload.adapter,
                    self.random.as_ref(),
                    Default::default(),
                ),
                self.params,
                build_group_vectors_payload(Default::default()),
            )
        }
    }

    let consumer = Consumer {
        params: build_group_vectors_params(GroupVectorsParamsOverrides {
            first_group: Some(build_first_group_vector_counts(
                FirstGroupVectorCountsOverrides {
                    pairing_check: Some(build_vector_count(
                        VectorCountConstructorParamsOverrides { count: Some(4) },
                    )),
                    ..Default::default()
                },
            )),
            ..Default::default()
        }),
        random: source,
    };

    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: None,
            ..Default::default()
        }),
        CreatePairingPayload,
    );
    let success = created.expect("the pairing factory ran the consumer");

    // Assert
    let vectors = success.output.expect("the counts are admitted");
    let mut passing = [0x00; 32];
    passing[31] = 0x01;
    assert_eq!(vectors.first_group.pairing_check[0].output, passing);
    assert_eq!(vectors.first_group.pairing_check[2].output, passing);
    assert_eq!(vectors.first_group.pairing_check[1].output, [0x00; 32]);
    assert_eq!(vectors.first_group.pairing_check[3].output, [0x00; 32]);
    // group_vectors_pairing_checks_alternate_passing_and_failing_verdicts
}

/// Contract: a pairing check draws `a` and `b`, and at an odd index also `c`.
/// Arrange:  `pairing_check` of `4`.
/// Act:      `group_vectors` in the consumer.
/// Assert:   twice the length of the `draws` of each record at indices `1`
///   and `3` equals three times the length of the `draws` of each record at
///   indices `0` and `2`.
/// Collaborators: the random source's and pairing's mock concretes in their
///   succeeding modes through `create_random_source` and `create_pairing`.
#[test]
fn group_vectors_draw_a_third_scalar_for_each_odd_pairing_check() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides { kind: None }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;

    struct Consumer {
        params: GroupVectorsParams,
        random: Box<dyn IRandomSourceAdapter>,
    }

    impl IPairingConsumer for Consumer {
        type Output = GroupVectorsReturn;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Act
            group_vectors::<P>(
                &build_group_vectors_deps(
                    &payload.adapter,
                    self.random.as_ref(),
                    Default::default(),
                ),
                self.params,
                build_group_vectors_payload(Default::default()),
            )
        }
    }

    let consumer = Consumer {
        params: build_group_vectors_params(GroupVectorsParamsOverrides {
            first_group: Some(build_first_group_vector_counts(
                FirstGroupVectorCountsOverrides {
                    pairing_check: Some(build_vector_count(
                        VectorCountConstructorParamsOverrides { count: Some(4) },
                    )),
                    ..Default::default()
                },
            )),
            ..Default::default()
        }),
        random: source,
    };

    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: None,
            ..Default::default()
        }),
        CreatePairingPayload,
    );
    let success = created.expect("the pairing factory ran the consumer");

    // Assert
    let vectors = success.output.expect("the counts are admitted");
    let pairing_check = &vectors.first_group.pairing_check;
    assert_eq!(
        2 * pairing_check[1].draws.len(),
        3 * pairing_check[0].draws.len()
    );
    assert_eq!(
        2 * pairing_check[3].draws.len(),
        3 * pairing_check[2].draws.len()
    );
    // group_vectors_draw_a_third_scalar_for_each_odd_pairing_check
}
