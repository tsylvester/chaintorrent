#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use encoding::{
    CanonicalFieldReaderErrorReturn, CanonicalFieldValue, CanonicalFieldsOverrides,
    IEncodingContract, build_canonical_fields, build_from_fields_params, build_to_fields_params,
};
use pairing::{
    CreatePairingParamsOverrides, IPairingArithmetic, IPairingReference,
    MockIPairingAdapterFailureMode, PairingConcrete, SampleUniformScalarErrorReturn,
    build_create_pairing_params,
};
use random::{
    CreateRandomSourceParamsOverrides, FillBytesErrorReturn, IRandomSourceAdapter,
    MockIRandomSourceAdapterFailureMode, RandomSourceKind, build_create_random_source_params,
};

use super::super::render::provides::{
    SolidityMemberNameConstructorParamsOverrides, build_solidity_member_name,
};
use super::group_vectors;
use super::interface::{
    GroupOperationVectorFromFieldsErrorReturn, GroupVectorsCasePayload, GroupVectorsErrorReturn,
    GroupVectorsParams, GroupVectorsReturn, IGroupVectorsCaseConsumer,
    PairingCheckVectorFromFieldsErrorReturn,
};
use super::mock::{
    FirstGroupVectorCountsOverrides, GroupOperationVectorOverrides,
    GroupVectorsCaseParamsOverrides, GroupVectorsParamsOverrides,
    PairingCheckVectorOverrides, build_first_group_vector_counts,
    build_group_operation_vector, build_group_operation_vector_description,
    build_group_vectors_case_deps, build_group_vectors_case_params,
    build_group_vectors_case_payload, build_group_vectors_deps, build_group_vectors_params,
    build_group_vectors_payload, build_pairing_check_vector,
    build_pairing_check_vector_description, build_second_group_vector_counts,
    run_group_vectors_case,
};
use super::super::super::factory::provides::{
    MsmTermCount, MsmTermCountConstructorParamsOverrides, MsmTermCountTryNewErrorReturn,
    VectorCount, VectorCountConstructorParamsOverrides, VectorCountTryNewErrorReturn,
    build_msm_term_count_constructor_params, build_vector_count,
    build_vector_count_constructor_params,
};

/// Contract: `NonZeroU32::new(params.count)` is `Some` → `Ok(VectorCount { count })`,
///   and `get()` returns the count.
/// Arrange: `build_vector_count_constructor_params` with `count` `3`.
/// Act:    `VectorCount::try_new` over it.
/// Assert: `value.get()` equals `3`.
#[test]
fn vector_count_admits_a_positive_count() {
    // Arrange
    let params = build_vector_count_constructor_params(VectorCountConstructorParamsOverrides {
        count: Some(3),
    });

    // Act
    let value = VectorCount::try_new(params).expect("a positive count admits");

    // Assert
    assert_eq!(value.get(), 3);
}
// vector_count_admits_a_positive_count

/// Contract: `NonZeroU32::new(params.count)` is `None` →
///   `Err(VectorCountTryNewErrorReturn::Zero)`.
/// Arrange: `build_vector_count_constructor_params` with `count` `0`.
/// Act:    `VectorCount::try_new` over it.
/// Assert: `error` equals `VectorCountTryNewErrorReturn::Zero`.
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
}
// vector_count_refuses_zero

/// Contract: `NonZeroU32::new(params.count)` is `Some` → `Ok(MsmTermCount { count })`,
///   and `get()` returns the count.
/// Arrange: `build_msm_term_count_constructor_params` with `count` `5`.
/// Act:    `MsmTermCount::try_new` over it.
/// Assert: `value.get()` equals `5`.
#[test]
fn msm_term_count_admits_a_positive_count() {
    // Arrange
    let params = build_msm_term_count_constructor_params(MsmTermCountConstructorParamsOverrides {
        count: Some(5),
    });

    // Act
    let value = MsmTermCount::try_new(params).expect("a positive count admits");

    // Assert
    assert_eq!(value.get(), 5);
}
// msm_term_count_admits_a_positive_count

/// Contract: `NonZeroU32::new(params.count)` is `None` →
///   `Err(MsmTermCountTryNewErrorReturn::Zero)`.
/// Arrange: `build_msm_term_count_constructor_params` with `count` `0`.
/// Act:    `MsmTermCount::try_new` over it.
/// Assert: `error` equals `MsmTermCountTryNewErrorReturn::Zero`.
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
}
// msm_term_count_refuses_zero

/// Contract: `to_fields` → `Ok(ToFieldsSuccessReturn { fields })` whose `values` are
///   `CanonicalFieldValue::Bytes` of clones of `draws`, `input`, and `output` in that
///   order.
/// Arrange: `build_group_operation_vector` with `draws` `vec![0x0A]`, `input`
///   `vec![0x0B, 0x0C]`, and `output` `vec![0x0D]`,
///   `build_group_operation_vector_description(Default::default())`, and
///   `build_to_fields_params(Default::default())`.
/// Act:    `to_fields`.
/// Assert: the values equal `CanonicalFieldValue::Bytes(vec![0x0A])`,
///   `CanonicalFieldValue::Bytes(vec![0x0B, 0x0C])`, and
///   `CanonicalFieldValue::Bytes(vec![0x0D])` in that order.
#[test]
fn group_operation_vector_description_maps_draws_input_and_output_to_byte_strings_in_order() {
    // Arrange
    let vector = build_group_operation_vector(GroupOperationVectorOverrides {
        draws: Some(vec![0x0A]),
        input: Some(vec![0x0B, 0x0C]),
        output: Some(vec![0x0D]),
    });
    let description = build_group_operation_vector_description(Default::default());

    // Act
    let fields = description
        .to_fields(build_to_fields_params(Default::default()), &vector)
        .expect("the description maps the record")
        .fields;

    // Assert
    assert_eq!(
        fields.values,
        vec![
            CanonicalFieldValue::Bytes(vec![0x0A]),
            CanonicalFieldValue::Bytes(vec![0x0B, 0x0C]),
            CanonicalFieldValue::Bytes(vec![0x0D]),
        ]
    );
}
// group_operation_vector_description_maps_draws_input_and_output_to_byte_strings_in_order

/// Contract: the reader admits every field → `Ok(FromFieldsSuccessReturn {
///   described: GroupOperationVector { draws, input, output } })`, each moved
///   from its field.
/// Arrange: `build_canonical_fields` with `values` `Bytes(vec![0x0A])`,
///   `Bytes(vec![0x0B, 0x0C])`, and `Bytes(vec![0x0D])`, the description from
///   `build_group_operation_vector_description(Default::default())`, and
///   `build_from_fields_params(Default::default())`.
/// Act:    `fields_to_value`.
/// Assert: `described` equals `build_group_operation_vector` with those three values.
#[test]
fn group_operation_vector_description_rebuilds_the_record_from_its_fields() {
    // Arrange
    let payload = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Bytes(vec![0x0A]),
            CanonicalFieldValue::Bytes(vec![0x0B, 0x0C]),
            CanonicalFieldValue::Bytes(vec![0x0D]),
        ]),
    });
    let description = build_group_operation_vector_description(Default::default());

    // Act
    let described = description
        .fields_to_value(build_from_fields_params(Default::default()), payload)
        .expect("the fields rebuild the record")
        .described;

    // Assert
    assert_eq!(
        described,
        build_group_operation_vector(GroupOperationVectorOverrides {
            draws: Some(vec![0x0A]),
            input: Some(vec![0x0B, 0x0C]),
            output: Some(vec![0x0D]),
        })
    );
}
// group_operation_vector_description_rebuilds_the_record_from_its_fields

/// Contract: a refusal of `CanonicalFieldReader::try_new` or of any read →
///   `Err(GroupOperationVectorFromFieldsErrorReturn::Fields(error))`, the error
///   unchanged.
/// Arrange: `build_canonical_fields` with `values` two `Bytes` values, the
///   description from
///   `build_group_operation_vector_description(Default::default())`, and
///   `build_from_fields_params(Default::default())`.
/// Act:    `fields_to_value`.
/// Assert: `result.err()` equals
///   `Some(GroupOperationVectorFromFieldsErrorReturn::Fields(
///   CanonicalFieldReaderErrorReturn::Count { expected: 3, actual: 2 }))`.
#[test]
fn group_operation_vector_description_carries_the_readers_refusal() {
    // Arrange
    let payload = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Bytes(vec![0x0A]),
            CanonicalFieldValue::Bytes(vec![0x0B]),
        ]),
    });
    let description = build_group_operation_vector_description(Default::default());

    // Act
    let result = description.fields_to_value(build_from_fields_params(Default::default()), payload);

    // Assert
    assert_eq!(
        result.err(),
        Some(GroupOperationVectorFromFieldsErrorReturn::Fields(
            CanonicalFieldReaderErrorReturn::Count {
                expected: 3,
                actual: 2,
            }
        ))
    );
}
// group_operation_vector_description_carries_the_readers_refusal

/// Contract: `to_fields` → `Ok(ToFieldsSuccessReturn { fields })` whose `values` are
///   `CanonicalFieldValue::Bytes` of clones of `draws` and `input` and
///   `CanonicalFieldValue::FixedBytes32` of `output`.
/// Arrange: `build_pairing_check_vector` with `draws` `vec![0x0A]`, `input`
///   `vec![0x0B]`, and `output` `[0x0C; 32]`,
///   `build_pairing_check_vector_description(Default::default())`, and
///   `build_to_fields_params(Default::default())`.
/// Act:    `to_fields`.
/// Assert: the values equal `CanonicalFieldValue::Bytes(vec![0x0A])`,
///   `CanonicalFieldValue::Bytes(vec![0x0B])`, and
///   `CanonicalFieldValue::FixedBytes32([0x0C; 32])` in that order.
#[test]
fn pairing_check_vector_description_maps_its_output_to_a_fixed_32_byte_field() {
    // Arrange
    let vector = build_pairing_check_vector(PairingCheckVectorOverrides {
        draws: Some(vec![0x0A]),
        input: Some(vec![0x0B]),
        output: Some([0x0C; 32]),
    });
    let description = build_pairing_check_vector_description(Default::default());

    // Act
    let fields = description
        .to_fields(build_to_fields_params(Default::default()), &vector)
        .expect("the description maps the record")
        .fields;

    // Assert
    assert_eq!(
        fields.values,
        vec![
            CanonicalFieldValue::Bytes(vec![0x0A]),
            CanonicalFieldValue::Bytes(vec![0x0B]),
            CanonicalFieldValue::FixedBytes32([0x0C; 32]),
        ]
    );
}
// pairing_check_vector_description_maps_its_output_to_a_fixed_32_byte_field

/// Contract: the reader admits every field → `Ok(FromFieldsSuccessReturn {
///   described: PairingCheckVector { draws, input, output } })`, each moved from
///   its field.
/// Arrange: `build_canonical_fields` with `values` `Bytes(vec![0x0A])`,
///   `Bytes(vec![0x0B])`, and `FixedBytes32([0x0C; 32])`, the description from
///   `build_pairing_check_vector_description(Default::default())`, and
///   `build_from_fields_params(Default::default())`.
/// Act:    `fields_to_value`.
/// Assert: `described` equals `build_pairing_check_vector` with those three values.
#[test]
fn pairing_check_vector_description_rebuilds_the_record_from_its_fields() {
    // Arrange
    let payload = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Bytes(vec![0x0A]),
            CanonicalFieldValue::Bytes(vec![0x0B]),
            CanonicalFieldValue::FixedBytes32([0x0C; 32]),
        ]),
    });
    let description = build_pairing_check_vector_description(Default::default());

    // Act
    let described = description
        .fields_to_value(build_from_fields_params(Default::default()), payload)
        .expect("the fields rebuild the record")
        .described;

    // Assert
    assert_eq!(
        described,
        build_pairing_check_vector(PairingCheckVectorOverrides {
            draws: Some(vec![0x0A]),
            input: Some(vec![0x0B]),
            output: Some([0x0C; 32]),
        })
    );
}
// pairing_check_vector_description_rebuilds_the_record_from_its_fields

/// Contract: a refusal of `CanonicalFieldReader::try_new` or of any read →
///   `Err(PairingCheckVectorFromFieldsErrorReturn::Fields(error))`, the error
///   unchanged.
/// Arrange: `build_canonical_fields` with `values` two `Bytes` values, the
///   description from
///   `build_pairing_check_vector_description(Default::default())`, and
///   `build_from_fields_params(Default::default())`.
/// Act:    `fields_to_value`.
/// Assert: `result.err()` equals
///   `Some(PairingCheckVectorFromFieldsErrorReturn::Fields(
///   CanonicalFieldReaderErrorReturn::Count { expected: 3, actual: 2 }))`.
#[test]
fn pairing_check_vector_description_carries_the_readers_refusal() {
    // Arrange
    let payload = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Bytes(vec![0x0A]),
            CanonicalFieldValue::Bytes(vec![0x0B]),
        ]),
    });
    let description = build_pairing_check_vector_description(Default::default());

    // Act
    let result = description.fields_to_value(build_from_fields_params(Default::default()), payload);

    // Assert
    assert_eq!(
        result.err(),
        Some(PairingCheckVectorFromFieldsErrorReturn::Fields(
            CanonicalFieldReaderErrorReturn::Count {
                expected: 3,
                actual: 2,
            }
        ))
    );
}
// pairing_check_vector_description_carries_the_readers_refusal

/// Contract: each name in `VECTOR_RECORD_MEMBER_NAMES` that
///   `SolidityMemberName::try_new` admits → `Ok` of the names in order.
/// Arrange: `build_group_operation_vector_description(Default::default())`.
/// Act:    `member_names()`.
/// Assert: the result equals `Ok` of the names `draws`, `input`, and `output`, each
///   through `build_solidity_member_name`.
#[test]
fn group_operation_vector_description_declares_draws_input_and_output_member_names() {
    // Arrange
    let description = build_group_operation_vector_description(Default::default());

    // Act
    let result = description.member_names();

    // Assert
    assert_eq!(
        result,
        Ok(vec![
            build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                text: Some("draws".to_string()),
            }),
            build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                text: Some("input".to_string()),
            }),
            build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                text: Some("output".to_string()),
            }),
        ])
    );
}
// group_operation_vector_description_declares_draws_input_and_output_member_names

/// Contract: each name in `VECTOR_RECORD_MEMBER_NAMES` that
///   `SolidityMemberName::try_new` admits → `Ok` of the names in order.
/// Arrange: `build_pairing_check_vector_description(Default::default())`.
/// Act:    `member_names()`.
/// Assert: the result equals `Ok` of the names `draws`, `input`, and `output`, each
///   through `build_solidity_member_name`.
#[test]
fn pairing_check_vector_description_declares_draws_input_and_output_member_names() {
    // Arrange
    let description = build_pairing_check_vector_description(Default::default());

    // Act
    let result = description.member_names();

    // Assert
    assert_eq!(
        result,
        Ok(vec![
            build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                text: Some("draws".to_string()),
            }),
            build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                text: Some("input".to_string()),
            }),
            build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                text: Some("output".to_string()),
            }),
        ])
    );
}
// pairing_check_vector_description_declares_draws_input_and_output_member_names

/// Contract: `FirstGroupOnly` with `params.second_group` `Some(_)` →
///   `Err(GroupVectorsErrorReturn::SecondGroupSettingsForFirstGroupOnly)`.
/// Arrange: the default mock pairing concrete, whose declaration is `FirstGroupOnly`,
///   and params from `build_group_vectors_params` with `second_group`
///   `Some(Some(build_second_group_vector_counts(Default::default())))`.
/// Act:    `group_vectors` in the consumer's method.
/// Assert: `result` equals `Err(GroupVectorsErrorReturn::SecondGroupSettingsForFirstGroupOnly)`.
#[test]
fn group_vectors_refuse_second_group_settings_on_a_first_group_only_pairing() {
    // Arrange
    struct Consumer {
        params: GroupVectorsParams,
    }

    impl IGroupVectorsCaseConsumer for Consumer {
        type Output = GroupVectorsReturn;

        fn consume_group_vectors_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            random: &dyn IRandomSourceAdapter,
            _payload: &GroupVectorsCasePayload,
        ) -> Self::Output {
            group_vectors::<P>(
                &build_group_vectors_deps(pairing, random, Default::default()),
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
    };

    // Act
    let created = run_group_vectors_case(
        &build_group_vectors_case_deps(consumer, Default::default()),
        build_group_vectors_case_params(Default::default()),
        build_group_vectors_case_payload(Default::default()),
    );

    // Assert
    let result = created.expect("every factory ran").output;
    assert_eq!(
        result,
        Err(GroupVectorsErrorReturn::SecondGroupSettingsForFirstGroupOnly)
    );
}
// group_vectors_refuse_second_group_settings_on_a_first_group_only_pairing

/// Contract: `deps.random.fill_bytes` fails →
///   `Err(GroupVectorsErrorReturn::FillBytes(error))`, the error unchanged.
/// Arrange: the case params' `random_source` overridden to
///   `build_create_random_source_params` with `kind`
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)`.
/// Act:    `group_vectors` in the consumer's method.
/// Assert: `result` equals
///   `Err(GroupVectorsErrorReturn::FillBytes(FillBytesErrorReturn::MockIRandomSourceAdapter))`.
#[test]
fn group_vectors_return_the_fill_refusal_unchanged() {
    // Arrange
    struct Consumer {
        params: GroupVectorsParams,
    }

    impl IGroupVectorsCaseConsumer for Consumer {
        type Output = GroupVectorsReturn;

        fn consume_group_vectors_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            random: &dyn IRandomSourceAdapter,
            _payload: &GroupVectorsCasePayload,
        ) -> Self::Output {
            group_vectors::<P>(
                &build_group_vectors_deps(pairing, random, Default::default()),
                self.params,
                build_group_vectors_payload(Default::default()),
            )
        }
    }

    let consumer = Consumer {
        params: build_group_vectors_params(Default::default()),
    };

    // Act
    let created = run_group_vectors_case(
        &build_group_vectors_case_deps(consumer, Default::default()),
        build_group_vectors_case_params(GroupVectorsCaseParamsOverrides {
            random_source: Some(build_create_random_source_params(
                CreateRandomSourceParamsOverrides {
                    kind: Some(RandomSourceKind::Mock(
                        MockIRandomSourceAdapterFailureMode::FillBytesRefused,
                    )),
                },
            )),
            ..Default::default()
        }),
        build_group_vectors_case_payload(Default::default()),
    );

    // Assert
    let result = created.expect("every factory ran").output;
    assert_eq!(
        result,
        Err(GroupVectorsErrorReturn::FillBytes(
            FillBytesErrorReturn::MockIRandomSourceAdapter
        ))
    );
}
// group_vectors_return_the_fill_refusal_unchanged

/// Contract: `sample_from_uniform_bytes` fails →
///   `Err(GroupVectorsErrorReturn::SampleScalar(error))`, the error unchanged.
/// Arrange: the case params' `pairing` overridden to `build_create_pairing_params`
///   with `concrete`
///   `PairingConcrete::Mock(MockIPairingAdapterFailureMode::SampleScalarWrongLength)`.
/// Act:    `group_vectors` in the consumer's method.
/// Assert: `result` equals `Err(GroupVectorsErrorReturn::SampleScalar(
///   SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual: 64 }))`, the
///   error the pairing mock concrete's contract states for that mode.
#[test]
fn group_vectors_return_the_sampling_refusal_unchanged() {
    // Arrange
    struct Consumer {
        params: GroupVectorsParams,
    }

    impl IGroupVectorsCaseConsumer for Consumer {
        type Output = GroupVectorsReturn;

        fn consume_group_vectors_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            random: &dyn IRandomSourceAdapter,
            _payload: &GroupVectorsCasePayload,
        ) -> Self::Output {
            group_vectors::<P>(
                &build_group_vectors_deps(pairing, random, Default::default()),
                self.params,
                build_group_vectors_payload(Default::default()),
            )
        }
    }

    let consumer = Consumer {
        params: build_group_vectors_params(Default::default()),
    };

    // Act
    let created = run_group_vectors_case(
        &build_group_vectors_case_deps(consumer, Default::default()),
        build_group_vectors_case_params(GroupVectorsCaseParamsOverrides {
            pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(PairingConcrete::Mock(
                    MockIPairingAdapterFailureMode::SampleScalarWrongLength,
                )),
                ..Default::default()
            })),
            ..Default::default()
        }),
        build_group_vectors_case_payload(Default::default()),
    );

    // Assert
    let result = created.expect("every factory ran").output;
    assert_eq!(
        result,
        Err(GroupVectorsErrorReturn::SampleScalar(
            SampleUniformScalarErrorReturn::WrongLength {
                expected: 64,
                actual: 64,
            }
        ))
    );
}
// group_vectors_return_the_sampling_refusal_unchanged

/// Contract: every first-group kind holds exactly its configured count of records,
///   and `second_group` is absent where `params.second_group` is `None`.
/// Arrange: `first_group` from `build_first_group_vector_counts` with `g1_add` of
///   `3`, `g1_mul` of `2`, and `pairing_check` of `4`, each from `build_vector_count`.
/// Act:    `group_vectors` in the consumer's method.
/// Assert: `first_group.g1_add`, `g1_mul`, and `pairing_check` hold `3`, `2`, and `4`
///   records and `second_group` is `None`.
#[test]
fn group_vectors_emit_each_first_group_kind_at_its_configured_count() {
    // Arrange
    struct Consumer {
        params: GroupVectorsParams,
    }

    impl IGroupVectorsCaseConsumer for Consumer {
        type Output = GroupVectorsReturn;

        fn consume_group_vectors_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            random: &dyn IRandomSourceAdapter,
            _payload: &GroupVectorsCasePayload,
        ) -> Self::Output {
            group_vectors::<P>(
                &build_group_vectors_deps(pairing, random, Default::default()),
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
    };

    // Act
    let created = run_group_vectors_case(
        &build_group_vectors_case_deps(consumer, Default::default()),
        build_group_vectors_case_params(Default::default()),
        build_group_vectors_case_payload(Default::default()),
    );

    // Assert
    let success = created
        .expect("every factory ran")
        .output
        .expect("the params admit");
    assert_eq!(success.first_group.g1_add.len(), 3);
    assert_eq!(success.first_group.g1_mul.len(), 2);
    assert_eq!(success.first_group.pairing_check.len(), 4);
    assert!(success.second_group.is_none());
}
// group_vectors_emit_each_first_group_kind_at_its_configured_count

/// Contract: first-group multiplication draws `a` then `s`, and `input` is the
///   encoding of `point` then the encoding of `s`.
/// Arrange: the default counts, so each `g1_mul` record holds draws `a` and `s` of
///   one width each.
/// Act:    `group_vectors` in the consumer's method.
/// Assert: each `g1_mul` record's `input` ends with the second half of its `draws`.
#[test]
fn group_vectors_g1_multiplication_ends_its_input_with_its_multiplier() {
    // Arrange
    struct Consumer {
        params: GroupVectorsParams,
    }

    impl IGroupVectorsCaseConsumer for Consumer {
        type Output = GroupVectorsReturn;

        fn consume_group_vectors_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            random: &dyn IRandomSourceAdapter,
            _payload: &GroupVectorsCasePayload,
        ) -> Self::Output {
            group_vectors::<P>(
                &build_group_vectors_deps(pairing, random, Default::default()),
                self.params,
                build_group_vectors_payload(Default::default()),
            )
        }
    }

    let consumer = Consumer {
        params: build_group_vectors_params(Default::default()),
    };

    // Act
    let created = run_group_vectors_case(
        &build_group_vectors_case_deps(consumer, Default::default()),
        build_group_vectors_case_params(Default::default()),
        build_group_vectors_case_payload(Default::default()),
    );

    // Assert
    let success = created
        .expect("every factory ran")
        .output
        .expect("the params admit");
    for record in &success.first_group.g1_mul {
        assert!(
            record
                .input
                .ends_with(&record.draws[record.draws.len() / 2..])
        );
    }
}
// group_vectors_g1_multiplication_ends_its_input_with_its_multiplier

/// Contract: the index is even → `t` cancels the first pair and `output` is the
///   32-byte word of zeros whose last byte is `0x01`; the index is odd → `t` is the
///   drawn `c` and `output` is thirty-two zero bytes.
/// Arrange: `pairing_check` of `4`.
/// Act:    `group_vectors` in the consumer's method.
/// Assert: the records at indices `0` and `2` have `output` thirty-one zero bytes
///   followed by `0x01`, and the records at indices `1` and `3` have `output`
///   thirty-two zero bytes.
#[test]
fn group_vectors_pairing_checks_alternate_passing_and_failing_verdicts() {
    // Arrange
    struct Consumer {
        params: GroupVectorsParams,
    }

    impl IGroupVectorsCaseConsumer for Consumer {
        type Output = GroupVectorsReturn;

        fn consume_group_vectors_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            random: &dyn IRandomSourceAdapter,
            _payload: &GroupVectorsCasePayload,
        ) -> Self::Output {
            group_vectors::<P>(
                &build_group_vectors_deps(pairing, random, Default::default()),
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
    };

    // Act
    let created = run_group_vectors_case(
        &build_group_vectors_case_deps(consumer, Default::default()),
        build_group_vectors_case_params(Default::default()),
        build_group_vectors_case_payload(Default::default()),
    );

    // Assert
    let success = created
        .expect("every factory ran")
        .output
        .expect("the params admit");
    let mut passing = [0x00; 32];
    passing[31] = 0x01;
    for index in [0, 2] {
        assert_eq!(success.first_group.pairing_check[index].output, passing);
    }
    for index in [1, 3] {
        assert_eq!(success.first_group.pairing_check[index].output, [0x00; 32]);
    }
}
// group_vectors_pairing_checks_alternate_passing_and_failing_verdicts

/// Contract: a pairing check draws `a` and `b`, and at an odd index also `c`.
/// Arrange: `pairing_check` of `4`.
/// Act:    `group_vectors` in the consumer's method.
/// Assert: twice the length of the `draws` of each record at indices `1` and `3`
///   equals three times the length of the `draws` of each record at indices `0`
///   and `2`.
#[test]
fn group_vectors_draw_a_third_scalar_for_each_odd_pairing_check() {
    // Arrange
    struct Consumer {
        params: GroupVectorsParams,
    }

    impl IGroupVectorsCaseConsumer for Consumer {
        type Output = GroupVectorsReturn;

        fn consume_group_vectors_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            random: &dyn IRandomSourceAdapter,
            _payload: &GroupVectorsCasePayload,
        ) -> Self::Output {
            group_vectors::<P>(
                &build_group_vectors_deps(pairing, random, Default::default()),
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
    };

    // Act
    let created = run_group_vectors_case(
        &build_group_vectors_case_deps(consumer, Default::default()),
        build_group_vectors_case_params(Default::default()),
        build_group_vectors_case_payload(Default::default()),
    );

    // Assert
    let success = created
        .expect("every factory ran")
        .output
        .expect("the params admit");
    for even in [0, 2] {
        for odd in [1, 3] {
            assert_eq!(
                2 * success.first_group.pairing_check[odd].draws.len(),
                3 * success.first_group.pairing_check[even].draws.len()
            );
        }
    }
}
// group_vectors_draw_a_third_scalar_for_each_odd_pairing_check
