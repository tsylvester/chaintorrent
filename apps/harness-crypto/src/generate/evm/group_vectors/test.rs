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
use super::group_vectors;
use super::interface::{
    GroupOperationVectorFromFieldsErrorReturn, GroupVectorsDeps, GroupVectorsErrorReturn,
    GroupVectorsParams, GroupVectorsPayload, GroupVectorsReturn, MsmTermCount,
    MsmTermCountTryNewErrorReturn, PairingCheckVectorFromFieldsErrorReturn, VectorCount,
    VectorCountTryNewErrorReturn,
};
use super::mock::{
    FirstGroupVectorCountsOverrides, GroupOperationVectorOverrides, GroupVectorsParamsOverrides,
    MsmTermCountConstructorParamsOverrides, MsmVectorSettingsOverrides,
    PairingCheckVectorOverrides, SecondGroupVectorCountsOverrides,
    VectorCountConstructorParamsOverrides, build_first_group_vector_counts,
    build_group_operation_vector, build_group_operation_vector_description,
    build_group_vectors_params, build_msm_term_count, build_msm_term_count_constructor_params,
    build_msm_vector_settings, build_pairing_check_vector, build_pairing_check_vector_description,
    build_second_group_vector_counts, build_vector_count, build_vector_count_constructor_params,
};
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFields, FromFieldsParams, IEncodingContract,
    ToFieldsParams,
};
use pairing::{
    AddScalarParams, AddScalarPayload, ConsumePairingParams, ConsumePairingPayload,
    CreatePairingDeps, CreatePairingParamsOverrides, CreatePairingPayload, DecodeScalarParams,
    EncodeG1Params, EncodeG1Payload, G1GeneratorParams, G1GeneratorPayload, IPairingArithmetic,
    IPairingConsumer, IPairingReference, MulG1Params, MulG1Payload, MulScalarParams,
    MulScalarPayload, NegScalarParams, NegScalarPayload, PairingConcrete,
    SampleUniformScalarErrorReturn, build_create_pairing_params, create_pairing,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParamsOverrides, CreateRandomSourcePayload,
    FillBytesParams, FillBytesPayload, FillBytesReturn, IRandomSourceAdapter,
    MockIRandomSourceAdapter, RandomSourceDeclaration, RandomSourceKind,
    build_create_random_source_params, build_random_source_declaration, create_random_source,
};

struct WrongLengthRandomSource;

impl IRandomSourceAdapter for WrongLengthRandomSource {
    fn declaration(&self) -> RandomSourceDeclaration {
        build_random_source_declaration(Default::default())
    }

    fn fill_bytes(&self, params: FillBytesParams, _payload: FillBytesPayload) -> FillBytesReturn {
        MockIRandomSourceAdapter.fill_bytes(params, FillBytesPayload { length: 0 })
    }
}

struct GroupVectorsProbeOutput {
    result: GroupVectorsReturn,
    g1_add_references: Vec<Vec<u8>>,
    g1_mul_references: Vec<Vec<u8>>,
    pairing_cancelling_terms: Vec<Vec<u8>>,
    g1_msm_references: Vec<Vec<u8>>,
}

struct GroupVectorsPairingProbe {
    params: GroupVectorsParams,
    random: Box<dyn IRandomSourceAdapter>,
}

impl IPairingConsumer for GroupVectorsPairingProbe {
    type Output = GroupVectorsProbeOutput;

    fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let mut output = GroupVectorsProbeOutput {
            result: group_vectors::<P>(
                &GroupVectorsDeps {
                    pairing: &payload.adapter,
                    random: self.random.as_ref(),
                },
                self.params,
                GroupVectorsPayload,
            ),
            g1_add_references: Vec::new(),
            g1_mul_references: Vec::new(),
            pairing_cancelling_terms: Vec::new(),
            g1_msm_references: Vec::new(),
        };
        let Ok(success) = &output.result else {
            return output;
        };
        let Ok(generator) = payload
            .adapter
            .g1_generator(G1GeneratorParams, G1GeneratorPayload);
        let g1 = generator.point;
        let decode = |draws: &[u8], index: usize| -> P::Scalar {
            payload
                .adapter
                .decode_scalar(DecodeScalarParams, &draws[index * 32..(index + 1) * 32])
                .expect("each draw is a canonical scalar")
                .scalar
        };
        let encode_g1 = |point: P::G1| -> Vec<u8> {
            let Ok(encoded) = payload
                .adapter
                .encode_g1(EncodeG1Params, EncodeG1Payload { point });
            encoded.bytes.as_ref().to_vec()
        };
        for record in &success.first_group.g1_add {
            let a = decode(&record.draws, 0);
            let b = decode(&record.draws, 1);
            let Ok(sum) = payload
                .adapter
                .add_scalar(AddScalarParams, AddScalarPayload { left: a, right: b });
            let Ok(product) = payload.adapter.mul_g1(
                MulG1Params,
                MulG1Payload {
                    point: g1.clone(),
                    scalar: sum.sum,
                },
            );
            output.g1_add_references.push(encode_g1(product.product));
        }
        for record in &success.first_group.g1_mul {
            let a = decode(&record.draws, 0);
            let s = decode(&record.draws, 1);
            let Ok(product) = payload
                .adapter
                .mul_scalar(MulScalarParams, MulScalarPayload { left: a, right: s });
            let Ok(point) = payload.adapter.mul_g1(
                MulG1Params,
                MulG1Payload {
                    point: g1.clone(),
                    scalar: product.product,
                },
            );
            output.g1_mul_references.push(encode_g1(point.product));
        }
        for (index, record) in success.first_group.pairing_check.iter().enumerate() {
            if index % 2 != 0 {
                continue;
            }
            let a = decode(&record.draws, 0);
            let b = decode(&record.draws, 1);
            let Ok(product) = payload
                .adapter
                .mul_scalar(MulScalarParams, MulScalarPayload { left: a, right: b });
            let Ok(negation) = payload.adapter.neg_scalar(
                NegScalarParams,
                NegScalarPayload {
                    scalar: product.product,
                },
            );
            let Ok(point) = payload.adapter.mul_g1(
                MulG1Params,
                MulG1Payload {
                    point: g1.clone(),
                    scalar: negation.negation,
                },
            );
            output
                .pairing_cancelling_terms
                .push(encode_g1(point.product));
        }
        if let Some(second_group) = &success.second_group {
            for record in &second_group.g1_msm {
                let mut total: Option<P::Scalar> = None;
                for term in 0..record.draws.len() / 64 {
                    let a = decode(&record.draws, term * 2);
                    let s = decode(&record.draws, term * 2 + 1);
                    let Ok(product) = payload
                        .adapter
                        .mul_scalar(MulScalarParams, MulScalarPayload { left: a, right: s });
                    total = Some(match total {
                        None => product.product,
                        Some(accumulated) => {
                            let Ok(sum) = payload.adapter.add_scalar(
                                AddScalarParams,
                                AddScalarPayload {
                                    left: accumulated,
                                    right: product.product,
                                },
                            );
                            sum.sum
                        }
                    });
                }
                let Some(total) = total else {
                    panic!("each record holds at least one term")
                };
                let Ok(point) = payload.adapter.mul_g1(
                    MulG1Params,
                    MulG1Payload {
                        point: g1.clone(),
                        scalar: total,
                    },
                );
                output.g1_msm_references.push(encode_g1(point.product));
            }
        }
        output
    }
}

fn run_group_vectors(
    probe: GroupVectorsPairingProbe,
    concrete: PairingConcrete,
) -> GroupVectorsProbeOutput {
    let Ok(success) = create_pairing(
        &CreatePairingDeps { consumer: probe },
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

/// Contract: a positive count is admitted, decided by
///   `NonZeroU32::new(params.count)` returning `Some`.
/// Arrange: constructor params whose count is 3.
/// Act:     VectorCount::try_new over those params.
/// Assert:  the admitted value's get() returns 3.
#[test]
fn vector_count_admits_a_positive_count() {
    // Arrange
    let params = build_vector_count_constructor_params(VectorCountConstructorParamsOverrides {
        count: Some(3),
    });

    // Act
    let result = VectorCount::try_new(params);

    // Assert
    let Ok(value) = result else {
        panic!("a positive count is admitted")
    };
    assert_eq!(value.get(), 3);
}

/// Contract: a zero count is refused, decided by `NonZeroU32::new(params.count)`
///   returning `None`.
/// Arrange: constructor params whose count is 0.
/// Act:     VectorCount::try_new over those params.
/// Assert:  the return is Err(VectorCountTryNewErrorReturn::Zero).
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

/// Contract: a positive term count is admitted, decided by
///   `NonZeroU32::new(params.count)` returning `Some`.
/// Arrange: constructor params whose count is 5.
/// Act:     MsmTermCount::try_new over those params.
/// Assert:  the admitted value's get() returns 5.
#[test]
fn msm_term_count_admits_a_positive_count() {
    // Arrange
    let params = build_msm_term_count_constructor_params(MsmTermCountConstructorParamsOverrides {
        count: Some(5),
    });

    // Act
    let result = MsmTermCount::try_new(params);

    // Assert
    let Ok(value) = result else {
        panic!("a positive count is admitted")
    };
    assert_eq!(value.get(), 5);
}

/// Contract: a zero term count is refused, decided by
///   `NonZeroU32::new(params.count)` returning `None`.
/// Arrange: constructor params whose count is 0.
/// Act:     MsmTermCount::try_new over those params.
/// Assert:  the return is Err(MsmTermCountTryNewErrorReturn::Zero).
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

/// Contract: the group-operation description's mapped branch carries draws,
///   input, and output each as a Bytes field, in member order.
/// Arrange: a group-operation vector with draws [0x0A], input [0x0B, 0x0C],
///   and output [0x0D].
/// Act:     the description's to_fields over the record.
/// Assert:  the values are Bytes([0x0A]), Bytes([0x0B, 0x0C]), and
///   Bytes([0x0D]) in that order.
#[test]
fn group_operation_vector_description_maps_draws_input_and_output_to_byte_strings_in_order() {
    // Arrange
    let vector = build_group_operation_vector(GroupOperationVectorOverrides {
        draws: Some(vec![0x0A]),
        input: Some(vec![0x0B, 0x0C]),
        output: Some(vec![0x0D]),
    });
    let description = build_group_operation_vector_description();

    // Act
    let Ok(success) = description.to_fields(ToFieldsParams, &vector);

    // Assert
    assert_eq!(
        success.fields.values,
        vec![
            CanonicalFieldValue::Bytes(vec![0x0A]),
            CanonicalFieldValue::Bytes(vec![0x0B, 0x0C]),
            CanonicalFieldValue::Bytes(vec![0x0D]),
        ]
    );
}

/// Contract: the group-operation description's rebuilt branch moves each Bytes
///   field back into its record member in order.
/// Arrange: canonical fields Bytes([0x0A]), Bytes([0x0B, 0x0C]), and
///   Bytes([0x0D]).
/// Act:     the description's fields_to_value over the fields.
/// Assert:  the described record holds those three byte strings.
#[test]
fn group_operation_vector_description_rebuilds_the_record_from_its_fields() {
    // Arrange
    let fields = CanonicalFields {
        values: vec![
            CanonicalFieldValue::Bytes(vec![0x0A]),
            CanonicalFieldValue::Bytes(vec![0x0B, 0x0C]),
            CanonicalFieldValue::Bytes(vec![0x0D]),
        ],
    };
    let description = build_group_operation_vector_description();

    // Act
    let result = description.fields_to_value(FromFieldsParams, fields);

    // Assert
    let Ok(success) = result else {
        panic!("the fields rebuild the record")
    };
    assert_eq!(
        success.described,
        build_group_operation_vector(GroupOperationVectorOverrides {
            draws: Some(vec![0x0A]),
            input: Some(vec![0x0B, 0x0C]),
            output: Some(vec![0x0D]),
        })
    );
}

/// Contract: a field vector that does not convert into three values is refused
///   on the count branch.
/// Arrange: canonical fields of two Bytes values.
/// Act:     the description's fields_to_value over the fields.
/// Assert:  the error is FieldCount { expected: 3, actual: 2 }.
#[test]
fn group_operation_vector_description_refuses_a_wrong_field_count() {
    // Arrange
    let fields = CanonicalFields {
        values: vec![
            CanonicalFieldValue::Bytes(vec![0x0A]),
            CanonicalFieldValue::Bytes(vec![0x0B]),
        ],
    };
    let description = build_group_operation_vector_description();

    // Act
    let result = description.fields_to_value(FromFieldsParams, fields);

    // Assert
    let Err(error) = result else {
        panic!("the wrong field count is refused")
    };
    assert_eq!(
        error,
        GroupOperationVectorFromFieldsErrorReturn::FieldCount {
            expected: 3,
            actual: 2
        }
    );
}

/// Contract: the lowest index whose value is not Bytes is refused on the kind
///   branch, expecting Bytes at that index.
/// Arrange: canonical fields Bytes([0x0A]), Bytes([0x0B]), and
///   FixedBytes32([0x0C; 32]).
/// Act:     the description's fields_to_value over the fields.
/// Assert:  the error is FieldKind { index: 2, expected:
///   CanonicalFieldKind::Bytes }.
#[test]
fn group_operation_vector_description_refuses_a_field_of_the_wrong_kind() {
    // Arrange
    let fields = CanonicalFields {
        values: vec![
            CanonicalFieldValue::Bytes(vec![0x0A]),
            CanonicalFieldValue::Bytes(vec![0x0B]),
            CanonicalFieldValue::FixedBytes32([0x0C; 32]),
        ],
    };
    let description = build_group_operation_vector_description();

    // Act
    let result = description.fields_to_value(FromFieldsParams, fields);

    // Assert
    let Err(error) = result else {
        panic!("the wrong field kind is refused")
    };
    assert_eq!(
        error,
        GroupOperationVectorFromFieldsErrorReturn::FieldKind {
            index: 2,
            expected: CanonicalFieldKind::Bytes,
        }
    );
}

/// Contract: the pairing-check description's mapped branch carries draws and
///   input as Bytes fields and output as a FixedBytes32 field.
/// Arrange: a pairing-check vector with draws [0x0A], input [0x0B], and output
///   [0x0C; 32].
/// Act:     the description's to_fields over the record.
/// Assert:  the values are Bytes([0x0A]), Bytes([0x0B]), and
///   FixedBytes32([0x0C; 32]) in that order.
#[test]
fn pairing_check_vector_description_maps_its_output_to_a_fixed_32_byte_field() {
    // Arrange
    let vector = build_pairing_check_vector(PairingCheckVectorOverrides {
        draws: Some(vec![0x0A]),
        input: Some(vec![0x0B]),
        output: Some([0x0C; 32]),
    });
    let description = build_pairing_check_vector_description();

    // Act
    let Ok(success) = description.to_fields(ToFieldsParams, &vector);

    // Assert
    assert_eq!(
        success.fields.values,
        vec![
            CanonicalFieldValue::Bytes(vec![0x0A]),
            CanonicalFieldValue::Bytes(vec![0x0B]),
            CanonicalFieldValue::FixedBytes32([0x0C; 32]),
        ]
    );
}

/// Contract: the pairing-check description's rebuilt branch moves each field
///   back into its record member, and a Bytes output field takes the kind
///   branch expecting FixedBytes32.
/// Arrange: canonical fields Bytes([0x0A]), Bytes([0x0B]), and
///   FixedBytes32([0x0C; 32]); and fields Bytes([0x0A]), Bytes([0x0B]), and
///   Bytes([0x0C]).
/// Act:     the description's fields_to_value over each field set.
/// Assert:  the first described record holds the three values; the second
///   error is FieldKind { index: 2, expected: CanonicalFieldKind::FixedBytes32
///   }.
#[test]
fn pairing_check_vector_description_rebuilds_the_record_and_refuses_a_byte_string_output() {
    // Arrange
    let admitted = CanonicalFields {
        values: vec![
            CanonicalFieldValue::Bytes(vec![0x0A]),
            CanonicalFieldValue::Bytes(vec![0x0B]),
            CanonicalFieldValue::FixedBytes32([0x0C; 32]),
        ],
    };
    let refused = CanonicalFields {
        values: vec![
            CanonicalFieldValue::Bytes(vec![0x0A]),
            CanonicalFieldValue::Bytes(vec![0x0B]),
            CanonicalFieldValue::Bytes(vec![0x0C]),
        ],
    };
    let description = build_pairing_check_vector_description();

    // Act
    let rebuilt = description.fields_to_value(FromFieldsParams, admitted);
    let wrong_kind = description.fields_to_value(FromFieldsParams, refused);

    // Assert
    let Ok(success) = rebuilt else {
        panic!("the fields rebuild the record")
    };
    assert_eq!(
        success.described,
        build_pairing_check_vector(PairingCheckVectorOverrides {
            draws: Some(vec![0x0A]),
            input: Some(vec![0x0B]),
            output: Some([0x0C; 32]),
        })
    );
    let Err(error) = wrong_kind else {
        panic!("the byte-string output is refused")
    };
    assert_eq!(
        error,
        PairingCheckVectorFromFieldsErrorReturn::FieldKind {
            index: 2,
            expected: CanonicalFieldKind::FixedBytes32,
        }
    );
}

/// Contract: each description's members branch pairs the member names draws,
///   input, and output with the description's FIELDS kind at the same index.
/// Arrange: the group-operation and pairing-check descriptions.
/// Act:     members() on each description.
/// Assert:  the first equals members draws, input, and output each of kind
///   Bytes; the second equals draws and input of kind Bytes and output of
///   kind FixedBytes32.
#[test]
fn vector_record_descriptions_declare_draws_input_and_output_members() {
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

    // Act
    let Ok(group_operation) = build_group_operation_vector_description().members() else {
        panic!("the member names are admitted")
    };
    let Ok(pairing_check) = build_pairing_check_vector_description().members() else {
        panic!("the member names are admitted")
    };

    // Assert
    assert_eq!(
        group_operation,
        build_solidity_record_members(SolidityRecordMembersConstructorParamsOverrides {
            members: Some(vec![
                member("draws", CanonicalFieldKind::Bytes),
                member("input", CanonicalFieldKind::Bytes),
                member("output", CanonicalFieldKind::Bytes),
            ]),
        })
    );
    assert_eq!(
        pairing_check,
        build_solidity_record_members(SolidityRecordMembersConstructorParamsOverrides {
            members: Some(vec![
                member("draws", CanonicalFieldKind::Bytes),
                member("input", CanonicalFieldKind::Bytes),
                member("output", CanonicalFieldKind::FixedBytes32),
            ]),
        })
    );
}

/// Contract: second-group settings handed where the pairing declares
///   FirstGroupOnly take the gate's settings-on-first-group-only branch,
///   before any draw.
/// Arrange: params whose second_group holds settings, over the operating
///   system random source.
/// Act:     group_vectors through the probe over
///   PairingConcrete::Bn254Arkworks.
/// Assert:  the result is
///   Err(GroupVectorsErrorReturn::SecondGroupSettingsForFirstGroupOnly).
#[test]
fn group_vectors_refuse_second_group_settings_on_a_first_group_only_pairing() {
    // Arrange
    let Ok(random) = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    );
    let probe = GroupVectorsPairingProbe {
        params: build_group_vectors_params(GroupVectorsParamsOverrides {
            second_group: Some(Some(build_second_group_vector_counts(Default::default()))),
            ..Default::default()
        }),
        random: random.adapter,
    };

    // Act
    let output = run_group_vectors(probe, PairingConcrete::Bn254Arkworks);

    // Assert
    assert_eq!(
        output.result,
        Err(GroupVectorsErrorReturn::SecondGroupSettingsForFirstGroupOnly)
    );
}

/// Contract: a pairing declaring BothGroups handed no second-group settings
///   takes the gate's settings-missing branch, before any draw.
/// Arrange: the default params, over the operating system random source.
/// Act:     group_vectors through the probe over
///   PairingConcrete::Bls12381Arkworks.
/// Assert:  the result is
///   Err(GroupVectorsErrorReturn::SecondGroupSettingsMissing).
#[test]
fn group_vectors_require_second_group_settings_on_bls12_381() {
    // Arrange
    let Ok(random) = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    );
    let probe = GroupVectorsPairingProbe {
        params: build_group_vectors_params(Default::default()),
        random: random.adapter,
    };

    // Act
    let output = run_group_vectors(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert_eq!(
        output.result,
        Err(GroupVectorsErrorReturn::SecondGroupSettingsMissing)
    );
}

/// Contract: a uniform draw of the wrong length takes the sampling-failed
///   branch, the sampler's refusal carried unchanged.
/// Arrange: a probe whose random source fills zero bytes.
/// Act:     group_vectors through the probe over
///   PairingConcrete::Bn254Arkworks.
/// Assert:  the result is Err(GroupVectorsErrorReturn::SampleScalar(
///   SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual: 0 })).
#[test]
fn group_vectors_return_the_sampling_error_for_a_draw_of_the_wrong_length() {
    // Arrange
    let probe = GroupVectorsPairingProbe {
        params: build_group_vectors_params(Default::default()),
        random: Box::new(WrongLengthRandomSource),
    };

    // Act
    let output = run_group_vectors(probe, PairingConcrete::Bn254Arkworks);

    // Assert
    assert_eq!(
        output.result,
        Err(GroupVectorsErrorReturn::SampleScalar(
            SampleUniformScalarErrorReturn::WrongLength {
                expected: 64,
                actual: 0
            }
        ))
    );
}

/// Contract: the computed branch emits each first-group kind at exactly its
///   configured count, and no second group on a first-group-only pairing.
/// Arrange: first-group counts of 3 g1_add, 2 g1_mul, and 4 pairing_check,
///   over the operating system random source.
/// Act:     group_vectors through the probe over
///   PairingConcrete::Bn254Arkworks.
/// Assert:  first_group.g1_add, g1_mul, and pairing_check hold 3, 2, and 4
///   records and second_group is None.
#[test]
fn group_vectors_emit_each_first_group_kind_at_its_configured_count() {
    // Arrange
    let Ok(random) = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    );
    let probe = GroupVectorsPairingProbe {
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
        random: random.adapter,
    };

    // Act
    let output = run_group_vectors(probe, PairingConcrete::Bn254Arkworks);

    // Assert
    let Ok(success) = output.result else {
        panic!("the vectors are computed")
    };
    assert_eq!(success.first_group.g1_add.len(), 3);
    assert_eq!(success.first_group.g1_mul.len(), 2);
    assert_eq!(success.first_group.pairing_check.len(), 4);
    assert_eq!(success.second_group, None);
}

/// Contract: on BN254 the computed branch's records carry the EIP-196 and
///   EIP-197 input and output encodings, each record's draws its scalars' 32
///   big-endian bytes.
/// Arrange: the default counts, over the operating system random source.
/// Act:     group_vectors through the probe over
///   PairingConcrete::Bn254Arkworks.
/// Assert:  each g1_add record's draws, input, and output hold 64, 128, and
///   64 bytes; each g1_mul record's 64, 96, and 64; the even-indexed
///   pairing_check record's draws and input 64 and 384, and the odd-indexed
///   record's draws 96.
#[test]
fn group_vectors_size_bn254_records_in_the_eip_196_and_eip_197_encodings() {
    // Arrange
    let Ok(random) = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    );
    let probe = GroupVectorsPairingProbe {
        params: build_group_vectors_params(Default::default()),
        random: random.adapter,
    };

    // Act
    let output = run_group_vectors(probe, PairingConcrete::Bn254Arkworks);

    // Assert
    let Ok(success) = output.result else {
        panic!("the vectors are computed")
    };
    for record in &success.first_group.g1_add {
        assert_eq!(record.draws.len(), 64);
        assert_eq!(record.input.len(), 128);
        assert_eq!(record.output.len(), 64);
    }
    for record in &success.first_group.g1_mul {
        assert_eq!(record.draws.len(), 64);
        assert_eq!(record.input.len(), 96);
        assert_eq!(record.output.len(), 64);
    }
    for (index, record) in success.first_group.pairing_check.iter().enumerate() {
        if index % 2 == 0 {
            assert_eq!(record.draws.len(), 64);
            assert_eq!(record.input.len(), 384);
        } else {
            assert_eq!(record.draws.len(), 96);
        }
    }
}

/// Contract: the computed branch's g1_add output is the generator times the
///   sum of the record's two drawn scalars.
/// Arrange: the default counts, over the operating system random source.
/// Act:     group_vectors through the probe over
///   PairingConcrete::Bn254Arkworks.
/// Assert:  each g1_add record's output equals the reference computed from its
///   own draws by an independent path.
#[test]
fn group_vectors_g1_addition_outputs_the_generator_times_the_sum_of_its_draws() {
    // Arrange
    let Ok(random) = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    );
    let probe = GroupVectorsPairingProbe {
        params: build_group_vectors_params(Default::default()),
        random: random.adapter,
    };

    // Act
    let output = run_group_vectors(probe, PairingConcrete::Bn254Arkworks);

    // Assert
    let Ok(success) = &output.result else {
        panic!("the vectors are computed")
    };
    for (index, record) in success.first_group.g1_add.iter().enumerate() {
        assert_eq!(record.output, output.g1_add_references[index]);
    }
}

/// Contract: the computed branch's g1_mul input ends with the multiplier's
///   scalar encoding and its output is the generator times the product of the
///   record's draws.
/// Arrange: the default counts, over the operating system random source.
/// Act:     group_vectors through the probe over
///   PairingConcrete::Bn254Arkworks.
/// Assert:  each g1_mul record's input[64..96] equals its draws[32..64] and its
///   output equals the reference computed from its own draws.
#[test]
fn group_vectors_g1_multiplication_ends_its_input_with_its_multiplier_and_outputs_the_product() {
    // Arrange
    let Ok(random) = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    );
    let probe = GroupVectorsPairingProbe {
        params: build_group_vectors_params(Default::default()),
        random: random.adapter,
    };

    // Act
    let output = run_group_vectors(probe, PairingConcrete::Bn254Arkworks);

    // Assert
    let Ok(success) = &output.result else {
        panic!("the vectors are computed")
    };
    for (index, record) in success.first_group.g1_mul.iter().enumerate() {
        assert_eq!(record.input[64..96], record.draws[32..64]);
        assert_eq!(record.output, output.g1_mul_references[index]);
    }
}

/// Contract: the computed branch's pairing_check output word's last byte is
///   0x01 where the product is one — the even-indexed records — and 0x00 where
///   it is not — the odd-indexed records.
/// Arrange: a pairing_check count of 4, over the operating system random
///   source.
/// Act:     group_vectors through the probe over
///   PairingConcrete::Bn254Arkworks.
/// Assert:  the records at indices 0 and 2 have output thirty-one zero bytes
///   followed by 0x01, and the records at indices 1 and 3 thirty-two zero
///   bytes.
#[test]
fn group_vectors_pairing_checks_alternate_passing_and_failing_verdicts() {
    // Arrange
    let Ok(random) = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    );
    let probe = GroupVectorsPairingProbe {
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
        random: random.adapter,
    };
    let mut passing = [0x00; 32];
    passing[31] = 0x01;
    let failing = [0x00; 32];

    // Act
    let output = run_group_vectors(probe, PairingConcrete::Bn254Arkworks);

    // Assert
    let Ok(success) = &output.result else {
        panic!("the vectors are computed")
    };
    assert_eq!(success.first_group.pairing_check[0].output, passing);
    assert_eq!(success.first_group.pairing_check[1].output, failing);
    assert_eq!(success.first_group.pairing_check[2].output, passing);
    assert_eq!(success.first_group.pairing_check[3].output, failing);
}

/// Contract: the computed branch's even-indexed pairing_check record's second
///   input term is the generator times the negated product of its draws, so
///   the product is one.
/// Arrange: a pairing_check count of 2, over the operating system random
///   source.
/// Act:     group_vectors through the probe over
///   PairingConcrete::Bn254Arkworks.
/// Assert:  the record at index 0 has input[192..256] equal to the reference
///   computed from its own draws by an independent path.
#[test]
fn group_vectors_passing_pairing_check_cancels_its_first_pair() {
    // Arrange
    let Ok(random) = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    );
    let probe = GroupVectorsPairingProbe {
        params: build_group_vectors_params(GroupVectorsParamsOverrides {
            first_group: Some(build_first_group_vector_counts(
                FirstGroupVectorCountsOverrides {
                    pairing_check: Some(build_vector_count(
                        VectorCountConstructorParamsOverrides { count: Some(2) },
                    )),
                    ..Default::default()
                },
            )),
            ..Default::default()
        }),
        random: random.adapter,
    };

    // Act
    let output = run_group_vectors(probe, PairingConcrete::Bn254Arkworks);

    // Assert
    let Ok(success) = &output.result else {
        panic!("the vectors are computed")
    };
    assert_eq!(
        success.first_group.pairing_check[0].input[192..256],
        output.pairing_cancelling_terms[0]
    );
}

/// Contract: on a pairing declaring BothGroups the computed branch emits each
///   second-group kind at its configured count, every record in the EIP-2537
///   encodings.
/// Arrange: second-group settings of 2 g2_add, g1_msm of count 2 and terms 3,
///   and g2_msm of count 1 and terms 2, over the operating system random
///   source.
/// Act:     group_vectors through the probe over
///   PairingConcrete::Bls12381Arkworks.
/// Assert:  second_group holds 2, 2, and 1 records; each g2_add record's
///   draws, input, and output hold 64, 512, and 256 bytes; each g1_msm
///   record's 192, 480, and 128; the g2_msm record's 128, 576, and 256; each
///   first-group g1_add record's input and output 256 and 128, each g1_mul
///   record's input 160, and each even-indexed pairing_check record's input
///   768.
#[test]
fn group_vectors_emit_second_group_kinds_on_bls12_381() {
    // Arrange
    let Ok(random) = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    );
    let probe = GroupVectorsPairingProbe {
        params: build_group_vectors_params(GroupVectorsParamsOverrides {
            second_group: Some(Some(build_second_group_vector_counts(
                SecondGroupVectorCountsOverrides {
                    g2_add: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(2),
                    })),
                    g1_msm: Some(build_msm_vector_settings(MsmVectorSettingsOverrides {
                        count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                            count: Some(2),
                        })),
                        terms: Some(build_msm_term_count(
                            MsmTermCountConstructorParamsOverrides { count: Some(3) },
                        )),
                    })),
                    g2_msm: Some(build_msm_vector_settings(MsmVectorSettingsOverrides {
                        count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                            count: Some(1),
                        })),
                        terms: Some(build_msm_term_count(
                            MsmTermCountConstructorParamsOverrides { count: Some(2) },
                        )),
                    })),
                },
            ))),
            ..Default::default()
        }),
        random: random.adapter,
    };

    // Act
    let output = run_group_vectors(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    let Ok(success) = &output.result else {
        panic!("the vectors are computed")
    };
    let Some(second_group) = &success.second_group else {
        panic!("the second group is emitted")
    };
    assert_eq!(second_group.g2_add.len(), 2);
    assert_eq!(second_group.g1_msm.len(), 2);
    assert_eq!(second_group.g2_msm.len(), 1);
    for record in &second_group.g2_add {
        assert_eq!(record.draws.len(), 64);
        assert_eq!(record.input.len(), 512);
        assert_eq!(record.output.len(), 256);
    }
    for record in &second_group.g1_msm {
        assert_eq!(record.draws.len(), 192);
        assert_eq!(record.input.len(), 480);
        assert_eq!(record.output.len(), 128);
    }
    assert_eq!(second_group.g2_msm[0].draws.len(), 128);
    assert_eq!(second_group.g2_msm[0].input.len(), 576);
    assert_eq!(second_group.g2_msm[0].output.len(), 256);
    for record in &success.first_group.g1_add {
        assert_eq!(record.input.len(), 256);
        assert_eq!(record.output.len(), 128);
    }
    for record in &success.first_group.g1_mul {
        assert_eq!(record.input.len(), 160);
    }
    for (index, record) in success.first_group.pairing_check.iter().enumerate() {
        if index % 2 == 0 {
            assert_eq!(record.input.len(), 768);
        }
    }
}

/// Contract: the computed branch's g1_msm output is the generator times the
///   sum of each term's drawn base and multiplier product, and its input
///   carries each multiplier's scalar encoding after the term's base encoding.
/// Arrange: second-group settings of 2 g2_add, g1_msm of count 2 and terms 3,
///   and g2_msm of count 1 and terms 2, over the operating system random
///   source.
/// Act:     group_vectors through the probe over
///   PairingConcrete::Bls12381Arkworks.
/// Assert:  each g1_msm record's output equals the reference computed from its
///   own draws, and for each term i of 3, input[i*160+128..i*160+160] equals
///   draws[i*64+32..i*64+64].
#[test]
fn group_vectors_g1_msm_on_bls12_381_outputs_the_generator_times_the_sum_of_draw_products_and_carries_each_multiplier()
 {
    // Arrange
    let Ok(random) = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    );
    let probe = GroupVectorsPairingProbe {
        params: build_group_vectors_params(GroupVectorsParamsOverrides {
            second_group: Some(Some(build_second_group_vector_counts(
                SecondGroupVectorCountsOverrides {
                    g1_msm: Some(build_msm_vector_settings(MsmVectorSettingsOverrides {
                        count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                            count: Some(2),
                        })),
                        terms: Some(build_msm_term_count(
                            MsmTermCountConstructorParamsOverrides { count: Some(3) },
                        )),
                    })),
                    ..Default::default()
                },
            ))),
            ..Default::default()
        }),
        random: random.adapter,
    };

    // Act
    let output = run_group_vectors(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    let Ok(success) = &output.result else {
        panic!("the vectors are computed")
    };
    let Some(second_group) = &success.second_group else {
        panic!("the second group is emitted")
    };
    for (index, record) in second_group.g1_msm.iter().enumerate() {
        assert_eq!(record.output, output.g1_msm_references[index]);
        for term in 0..3 {
            assert_eq!(
                record.input[term * 160 + 128..term * 160 + 160],
                record.draws[term * 64 + 32..term * 64 + 64]
            );
        }
    }
}
