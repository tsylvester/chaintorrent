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
use super::interface::*;
use super::mock::*;
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, FromFieldsParamsOverrides, IEncodingContract,
    ToFieldsParamsOverrides, build_from_fields_params, build_to_fields_params,
};
use pairing::{
    AddScalarParams, AddScalarPayload, ConsumePairingParams, ConsumePairingPayload,
    CreatePairingDepsOverrides, CreatePairingParamsOverrides, CreatePairingPayload,
    DecodeScalarParams, EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload,
    EncodeScalarParams, EncodeScalarPayload, G1GeneratorParams, G1GeneratorPayload,
    G2GeneratorParams, G2GeneratorPayload, IPairingArithmetic, IPairingConsumer, IPairingReference,
    ISampleUniformScalar, MulG1Params, MulG1Payload, MulG2Params, MulG2Payload, MulScalarParams,
    MulScalarPayload, PAIRING_CONCRETES, SampleUniformScalarParams, SampleUniformScalarPayload,
    VerifierGroupArithmetic, build_create_pairing_deps, build_create_pairing_params,
    create_pairing,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParamsOverrides, CreateRandomSourcePayload,
    FillBytesParams, FillBytesPayload, IRandomSourceAdapter, RandomSourceKind,
    build_create_random_source_params, create_random_source,
};

fn generator_g1<P: IPairingArithmetic>(pairing: &P) -> P::G1 {
    let Ok(success) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
    success.point
}

fn generator_g2<P: IPairingArithmetic>(pairing: &P) -> P::G2 {
    let Ok(success) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
    success.point
}

fn encoded_g1_bytes<P: IPairingArithmetic>(pairing: &P, point: &P::G1) -> Vec<u8> {
    let Ok(encoded) = pairing.encode_g1(
        EncodeG1Params,
        EncodeG1Payload {
            point: point.clone(),
        },
    );
    encoded.bytes.as_ref().to_vec()
}

fn encoded_g2_bytes<P: IPairingArithmetic>(pairing: &P, point: &P::G2) -> Vec<u8> {
    let Ok(encoded) = pairing.encode_g2(
        EncodeG2Params,
        EncodeG2Payload {
            point: point.clone(),
        },
    );
    encoded.bytes.as_ref().to_vec()
}

fn encoded_scalar_bytes<P: IPairingArithmetic>(pairing: &P, scalar: &P::Scalar) -> Vec<u8> {
    let Ok(encoded) = pairing.encode_scalar(
        EncodeScalarParams,
        EncodeScalarPayload {
            scalar: scalar.clone(),
        },
    );
    encoded.bytes.expose().as_ref().to_vec()
}

fn decoded_scalar<P: IPairingArithmetic>(pairing: &P, bytes: &[u8]) -> P::Scalar {
    pairing
        .decode_scalar(DecodeScalarParams, bytes)
        .expect("a recorded draw decodes")
        .scalar
}

fn sum_scalar<P: IPairingArithmetic>(pairing: &P, left: P::Scalar, right: P::Scalar) -> P::Scalar {
    let Ok(success) = pairing.add_scalar(AddScalarParams, AddScalarPayload { left, right });
    success.sum
}

fn product_scalar<P: IPairingArithmetic>(
    pairing: &P,
    left: P::Scalar,
    right: P::Scalar,
) -> P::Scalar {
    let Ok(success) = pairing.mul_scalar(MulScalarParams, MulScalarPayload { left, right });
    success.product
}

fn product_g1<P: IPairingArithmetic>(pairing: &P, point: P::G1, scalar: P::Scalar) -> P::G1 {
    let Ok(success) = pairing.mul_g1(MulG1Params, MulG1Payload { point, scalar });
    success.product
}

fn product_g2<P: IPairingArithmetic>(pairing: &P, point: P::G2, scalar: P::Scalar) -> P::G2 {
    let Ok(success) = pairing.mul_g2(MulG2Params, MulG2Payload { point, scalar });
    success.product
}

fn os_scalar<P: IPairingArithmetic>(random: &dyn IRandomSourceAdapter) -> P::Scalar {
    let filled = random
        .fill_bytes(
            FillBytesParams,
            FillBytesPayload {
                length: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
            },
        )
        .expect("the operating-system source fills the uniform bytes");
    let sampled = <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
        SampleUniformScalarParams,
        SampleUniformScalarPayload {
            uniform: filled.bytes,
        },
    )
    .expect("a filled uniform sample admits a scalar");
    sampled.scalar.expose().clone()
}

fn scalar_encoding_width<P: IPairingArithmetic>(
    pairing: &P,
    random: &dyn IRandomSourceAdapter,
) -> usize {
    encoded_scalar_bytes(pairing, &os_scalar::<P>(random)).len()
}

fn field_kind(value: &CanonicalFieldValue) -> CanonicalFieldKind {
    match value {
        CanonicalFieldValue::Bytes(_) => CanonicalFieldKind::Bytes,
        CanonicalFieldValue::FixedBytes32(_) => CanonicalFieldKind::FixedBytes32,
        _ => panic!("the descriptions emit only Bytes and FixedBytes32"),
    }
}

/// Contract: entry `a_record_survives_its_description_round_trip`; each
///   description's `fields_to_value`, called with the fields its `to_fields`
///   returns for a record whose three fields differ from one another in
///   value and length, returns `Ok` whose `described` equals the record.
/// Arrange: `build_group_operation_vector` with `draws` `vec![0x0A, 0x0B]`,
///   `input` `vec![0x0C]`, and `output` `vec![0x0D, 0x0E, 0x0F]`, and
///   `build_pairing_check_vector` with `draws` `vec![0x0A]`, `input`
///   `vec![0x0B, 0x0C, 0x0D]`, and `output` `[0x0E; 32]`, each with its
///   description and `build_to_fields_params`/`build_from_fields_params` at
///   their defaults.
/// Act:     `to_fields` on each record, then `fields_to_value` on the fields
///   it returned.
/// Assert:  each `described` equals the record it was built from.
/// Boundary: each description's `to_fields` then `fields_to_value`, both
///   real.
/// Mocked:  nothing, because the chain has no outer-edge collaborator.
#[test]
fn a_record_survives_its_description_round_trip() {
    // Arrange
    let operation = build_group_operation_vector(GroupOperationVectorOverrides {
        draws: Some(vec![0x0A, 0x0B]),
        input: Some(vec![0x0C]),
        output: Some(vec![0x0D, 0x0E, 0x0F]),
    });
    let operation_description = build_group_operation_vector_description(Default::default());
    let check = build_pairing_check_vector(PairingCheckVectorOverrides {
        draws: Some(vec![0x0A]),
        input: Some(vec![0x0B, 0x0C, 0x0D]),
        output: Some([0x0E; 32]),
    });
    let check_description = build_pairing_check_vector_description(Default::default());

    // Act
    let Ok(operation_fields) =
        operation_description.to_fields(build_to_fields_params(Default::default()), &operation);
    let Ok(check_fields) =
        check_description.to_fields(build_to_fields_params(Default::default()), &check);
    let operation_rebuilt = operation_description
        .fields_to_value(
            build_from_fields_params(Default::default()),
            operation_fields.fields,
        )
        .expect("the record's fields rebuild it");
    let check_rebuilt = check_description
        .fields_to_value(
            build_from_fields_params(Default::default()),
            check_fields.fields,
        )
        .expect("the record's fields rebuild it");

    // Assert
    assert_eq!(operation_rebuilt.described, operation);
    assert_eq!(check_rebuilt.described, check);
    // a_record_survives_its_description_round_trip
}

/// Contract: entry `members_declare_the_kinds_to_fields_emits`; on each
///   description over a built record, the kinds of `members()` in order are
///   the kinds of the values `to_fields` returns, under the names `draws`,
///   `input`, and `output`.
/// Arrange: `build_group_operation_vector` and `build_pairing_check_vector`
///   at their defaults, each with its description and
///   `build_to_fields_params` at its default.
/// Act:     `to_fields` and `members()` on each description.
/// Assert:  each `members()` result equals `Ok` of
///   `build_solidity_record_members` over `build_solidity_record_member`
///   values named `draws`, `input`, and `output` through
///   `build_solidity_member_name`, whose kinds are, in order,
///   `CanonicalFieldKind::Bytes` for each `CanonicalFieldValue::Bytes` and
///   `CanonicalFieldKind::FixedBytes32` for each
///   `CanonicalFieldValue::FixedBytes32` among the values `to_fields`
///   returned.
/// Boundary: `members()` over the real `SolidityMemberName::try_new` and
///   `SolidityRecordMembers::try_new`, and `to_fields`.
/// Mocked:  nothing, because the chain has no outer-edge collaborator.
#[test]
fn members_declare_the_kinds_to_fields_emits() {
    // Arrange
    let operation = build_group_operation_vector(Default::default());
    let operation_description = build_group_operation_vector_description(Default::default());
    let check = build_pairing_check_vector(Default::default());
    let check_description = build_pairing_check_vector_description(Default::default());

    // Act
    let Ok(operation_fields) =
        operation_description.to_fields(build_to_fields_params(Default::default()), &operation);
    let operation_members = operation_description.members();
    let Ok(check_fields) =
        check_description.to_fields(build_to_fields_params(Default::default()), &check);
    let check_members = check_description.members();

    // Assert
    for (fields, members) in [
        (&operation_fields.fields.values, operation_members),
        (&check_fields.fields.values, check_members),
    ] {
        let expected =
            build_solidity_record_members(SolidityRecordMembersConstructorParamsOverrides {
                members: Some(
                    VECTOR_RECORD_MEMBER_NAMES
                        .iter()
                        .zip(fields.iter())
                        .map(|(name, value)| {
                            build_solidity_record_member(SolidityRecordMemberOverrides {
                                name: Some(build_solidity_member_name(
                                    SolidityMemberNameConstructorParamsOverrides {
                                        text: Some((*name).to_string()),
                                    },
                                )),
                                kind: Some(field_kind(value)),
                            })
                        })
                        .collect(),
                ),
            });
        assert_eq!(members, Ok(expected));
    }
    // members_declare_the_kinds_to_fields_emits
}

/// Contract: entry `a_both_groups_pairing_requires_its_second_group_
///   settings`; `group_vectors` called with `second_group` absent returns
///   `Err(GroupVectorsErrorReturn::SecondGroupSettingsMissing)` where
///   `P::DECLARATION.verifier_group_arithmetic` is `BothGroups`, and `Ok`
///   with `second_group` absent where it is `FirstGroupOnly`.
/// Arrange: the consumer's `second_group` absent whatever the declaration,
///   `build_group_vectors_params` with `second_group` overridden to `None`,
///   run once per member of `PAIRING_CONCRETES`.
/// Act:     `group_vectors::<P>` once in the consumer.
/// Assert:  in place, where `P::DECLARATION.verifier_group_arithmetic` is
///   `BothGroups` the result equals `Err(GroupVectorsErrorReturn::
///   SecondGroupSettingsMissing)`, and where it is `FirstGroupOnly` the
///   result is `Ok` whose `second_group` is `None`.
/// Boundary: `create_pairing`, the arm's real concrete's declaration, and
///   `group_vectors`.
/// Mocked:  nothing, because the operating system's generator is the outer
///   edge and runs real.
#[test]
fn a_both_groups_pairing_requires_its_second_group_settings() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;

    struct Consumer {
        second_group: Option<SecondGroupVectorCounts>,
        random: Box<dyn IRandomSourceAdapter>,
    }

    impl IPairingConsumer for &Consumer {
        type Output = GroupVectorsReturn;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let second_group = match P::DECLARATION.verifier_group_arithmetic {
                VerifierGroupArithmetic::BothGroups => self.second_group,
                VerifierGroupArithmetic::FirstGroupOnly => None,
            };
            let params = build_group_vectors_params(GroupVectorsParamsOverrides {
                second_group: Some(second_group),
                ..Default::default()
            });

            // Act
            let result = group_vectors::<P>(
                &build_group_vectors_deps(
                    &payload.adapter,
                    self.random.as_ref(),
                    Default::default(),
                ),
                params,
                build_group_vectors_payload(Default::default()),
            );

            // Assert
            match P::DECLARATION.verifier_group_arithmetic {
                VerifierGroupArithmetic::BothGroups => {
                    assert_eq!(
                        result,
                        Err(GroupVectorsErrorReturn::SecondGroupSettingsMissing)
                    );
                }
                VerifierGroupArithmetic::FirstGroupOnly => {
                    let Ok(vectors) = &result else {
                        panic!("a first-group-only pairing is served without settings")
                    };
                    assert_eq!(vectors.second_group, None);
                }
            }
            result
        }
    }

    let consumer = Consumer {
        second_group: None,
        random: source,
    };
    for concrete in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(&consumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        created.expect("the pairing factory ran the consumer");
    }
    // a_both_groups_pairing_requires_its_second_group_settings
}

/// Contract: entry `a_both_groups_pairing_emits_the_second_group_kinds_at_
///   their_configured_counts`; `group_vectors` called with settings where
///   the declaration is `BothGroups` returns `second_group` holding exactly
///   the configured records of each kind, each multi-scalar-multiplication
///   record's `input` holding its configured terms, each a base encoding and
///   a multiplier encoding, and with `second_group` absent where it is
///   `FirstGroupOnly` returns `second_group` absent.
/// Arrange: settings from `build_second_group_vector_counts` with `g2_add`
///   from `build_vector_count` over `2`, `g1_msm` from
///   `build_msm_vector_settings` with `count` `2` and `terms` `3`, and
///   `g2_msm` from `build_msm_vector_settings` with `count` `1` and `terms`
///   `2`; the consumer reads the widths `encode_g1`, `encode_g2`, and
///   `encode_scalar` write; every member of `PAIRING_CONCRETES` is the
///   variation.
/// Act:     `group_vectors::<P>` once in the consumer.
/// Assert:  in place, where the declaration is `BothGroups` `second_group`
///   is `Some` holding `2`, `2`, and `1` records, each `g1_msm` record's
///   `input` holds `3` terms of one `encode_g1` width plus one
///   `encode_scalar` width, and each `g2_msm` record's `input` holds `2`
///   terms of one `encode_g2` width plus one `encode_scalar` width, and
///   where it is `FirstGroupOnly` `second_group` is `None`.
/// Boundary: `create_pairing`, the arm's real concrete's operations, and
///   `group_vectors`.
/// Mocked:  nothing, because the operating system's generator is the outer
///   edge and runs real.
#[test]
fn a_both_groups_pairing_emits_the_second_group_kinds_at_their_configured_counts() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;

    struct Consumer {
        second_group: Option<SecondGroupVectorCounts>,
        random: Box<dyn IRandomSourceAdapter>,
    }

    impl IPairingConsumer for &Consumer {
        type Output = GroupVectorsReturn;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let second_group = match P::DECLARATION.verifier_group_arithmetic {
                VerifierGroupArithmetic::BothGroups => self.second_group,
                VerifierGroupArithmetic::FirstGroupOnly => None,
            };
            let params = build_group_vectors_params(GroupVectorsParamsOverrides {
                second_group: Some(second_group),
                ..Default::default()
            });
            let g1_width =
                encoded_g1_bytes(&payload.adapter, &generator_g1(&payload.adapter)).len();
            let g2_width =
                encoded_g2_bytes(&payload.adapter, &generator_g2(&payload.adapter)).len();
            let scalar_width = scalar_encoding_width(&payload.adapter, self.random.as_ref());

            // Act
            let result = group_vectors::<P>(
                &build_group_vectors_deps(
                    &payload.adapter,
                    self.random.as_ref(),
                    Default::default(),
                ),
                params,
                build_group_vectors_payload(Default::default()),
            );

            // Assert
            let Ok(vectors) = &result else {
                panic!("the configured counts are admitted")
            };
            match P::DECLARATION.verifier_group_arithmetic {
                VerifierGroupArithmetic::BothGroups => {
                    let second_group = vectors
                        .second_group
                        .as_ref()
                        .expect("a both-groups pairing emits its second group");
                    assert_eq!(second_group.g2_add.len(), 2);
                    assert_eq!(second_group.g1_msm.len(), 2);
                    assert_eq!(second_group.g2_msm.len(), 1);
                    for record in &second_group.g1_msm {
                        assert_eq!(record.input.len(), 3 * (g1_width + scalar_width));
                    }
                    for record in &second_group.g2_msm {
                        assert_eq!(record.input.len(), 2 * (g2_width + scalar_width));
                    }
                }
                VerifierGroupArithmetic::FirstGroupOnly => {
                    assert_eq!(vectors.second_group, None);
                }
            }
            result
        }
    }

    let consumer = Consumer {
        second_group: Some(build_second_group_vector_counts(
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
        )),
        random: source,
    };
    for concrete in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(&consumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        created.expect("the pairing factory ran the consumer");
    }
    // a_both_groups_pairing_emits_the_second_group_kinds_at_their_configured_counts
}

/// Contract: entry
///   `each_first_group_output_is_the_references_result_for_its_drawn_
///   operands`; on every concrete of `PAIRING_CONCRETES`, each first-group
///   addition record's `output` is the encoding of the first-group
///   generator times the sum of its two draws, and each first-group
///   multiplication record's `output` is the encoding of the first-group
///   generator times the product of its two draws.
/// Arrange: `build_first_group_vector_counts` with `g1_add` and `g1_mul`
///   from `build_vector_count` over `3`, so more than one record of each
///   kind is drawn, and `pairing_check` at its default; the consumer reads
///   the width `encode_scalar` writes, reads each record's draws back
///   through `decode_scalar`, and builds the expected outputs from them:
///   for each `g1_add` record the encoding of `mul_g1(g1, add_scalar(a,
///   b))` through `encode_g1`, and for each `g1_mul` record the encoding of
///   `mul_g1(g1, mul_scalar(a, s))` through `encode_g1`.
/// Act:     `group_vectors::<P>` once in the consumer.
/// Assert:  in place, each `g1_add` and `g1_mul` record's `output` equals
///   its expected output.
/// Boundary: `create_pairing`, the arm's real concrete's operations, and
///   `group_vectors`.
/// Mocked:  nothing, because the operating system's generator is the outer
///   edge and runs real.
#[test]
fn each_first_group_output_is_the_references_result_for_its_drawn_operands() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;

    struct Consumer {
        first_group: FirstGroupVectorCounts,
        second_group: Option<SecondGroupVectorCounts>,
        random: Box<dyn IRandomSourceAdapter>,
    }

    impl IPairingConsumer for &Consumer {
        type Output = GroupVectorsReturn;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let second_group = match P::DECLARATION.verifier_group_arithmetic {
                VerifierGroupArithmetic::BothGroups => self.second_group,
                VerifierGroupArithmetic::FirstGroupOnly => None,
            };
            let params = build_group_vectors_params(GroupVectorsParamsOverrides {
                first_group: Some(self.first_group),
                second_group: Some(second_group),
            });
            let scalar_width = scalar_encoding_width(&payload.adapter, self.random.as_ref());
            let g1 = generator_g1(&payload.adapter);

            // Act
            let result = group_vectors::<P>(
                &build_group_vectors_deps(
                    &payload.adapter,
                    self.random.as_ref(),
                    Default::default(),
                ),
                params,
                build_group_vectors_payload(Default::default()),
            );

            // Assert
            let Ok(vectors) = &result else {
                panic!("the configured counts are admitted")
            };
            for record in &vectors.first_group.g1_add {
                let a = decoded_scalar(&payload.adapter, &record.draws[..scalar_width]);
                let b = decoded_scalar(&payload.adapter, &record.draws[scalar_width..]);
                let expected = encoded_g1_bytes(
                    &payload.adapter,
                    &product_g1(
                        &payload.adapter,
                        g1.clone(),
                        sum_scalar(&payload.adapter, a, b),
                    ),
                );
                assert_eq!(record.output, expected);
            }
            for record in &vectors.first_group.g1_mul {
                let a = decoded_scalar(&payload.adapter, &record.draws[..scalar_width]);
                let s = decoded_scalar(&payload.adapter, &record.draws[scalar_width..]);
                let expected = encoded_g1_bytes(
                    &payload.adapter,
                    &product_g1(
                        &payload.adapter,
                        g1.clone(),
                        product_scalar(&payload.adapter, a, s),
                    ),
                );
                assert_eq!(record.output, expected);
            }
            result
        }
    }

    let consumer = Consumer {
        first_group: build_first_group_vector_counts(FirstGroupVectorCountsOverrides {
            g1_add: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                count: Some(3),
            })),
            g1_mul: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                count: Some(3),
            })),
            ..Default::default()
        }),
        second_group: Some(build_second_group_vector_counts(Default::default())),
        random: source,
    };
    for concrete in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(&consumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        created.expect("the pairing factory ran the consumer");
    }
    // each_first_group_output_is_the_references_result_for_its_drawn_operands
}

/// Contract: entry
///   `each_second_group_output_is_the_references_result_for_its_drawn_
///   operands`; on every concrete whose declaration is `BothGroups`, each
///   second-group addition record's `output` is the encoding of the
///   second-group generator times the sum of its two draws, and each
///   multi-scalar-multiplication record's `output` is the encoding of the
///   group's generator times the sum over its terms of the product of the
///   term's two draws.
/// Arrange: the settings of `a_both_groups_pairing_emits_the_second_group_
///   kinds_at_their_configured_counts`; the consumer reads each record's
///   draws back through `decode_scalar` and builds the expected outputs
///   from them: for each `g2_add` record the encoding of `mul_g2(g2,
///   add_scalar(a, b))`, for each `g1_msm` record the encoding of
///   `mul_g1(g1, Σ mul_scalar(a_i, s_i))`, and for each `g2_msm` record the
///   encoding of `mul_g2(g2, Σ mul_scalar(a_i, s_i))`, each sum through
///   `add_scalar`.
/// Act:     `group_vectors::<P>` once in the consumer, only where the
///   declaration is `BothGroups`.
/// Assert:  in place, each second-group record's `output` equals its
///   expected output.
/// Boundary: `create_pairing`, the arm's real concrete's operations, and
///   `group_vectors`.
/// Mocked:  nothing, because the operating system's generator is the outer
///   edge and runs real.
#[test]
fn each_second_group_output_is_the_references_result_for_its_drawn_operands() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;

    struct Consumer {
        second_group: Option<SecondGroupVectorCounts>,
        random: Box<dyn IRandomSourceAdapter>,
    }

    impl IPairingConsumer for &Consumer {
        type Output = GroupVectorsReturn;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let second_group = match P::DECLARATION.verifier_group_arithmetic {
                VerifierGroupArithmetic::BothGroups => self.second_group,
                VerifierGroupArithmetic::FirstGroupOnly => None,
            };
            let params = build_group_vectors_params(GroupVectorsParamsOverrides {
                second_group: Some(second_group),
                ..Default::default()
            });
            let scalar_width = scalar_encoding_width(&payload.adapter, self.random.as_ref());
            let g1 = generator_g1(&payload.adapter);
            let g2 = generator_g2(&payload.adapter);

            // Act
            let result = group_vectors::<P>(
                &build_group_vectors_deps(
                    &payload.adapter,
                    self.random.as_ref(),
                    Default::default(),
                ),
                params,
                build_group_vectors_payload(Default::default()),
            );

            // Assert
            if let VerifierGroupArithmetic::BothGroups = P::DECLARATION.verifier_group_arithmetic {
                let Ok(vectors) = &result else {
                    panic!("the configured counts are admitted")
                };
                let second_group = vectors
                    .second_group
                    .as_ref()
                    .expect("a both-groups pairing emits its second group");
                for record in &second_group.g2_add {
                    let a = decoded_scalar(&payload.adapter, &record.draws[..scalar_width]);
                    let b = decoded_scalar(&payload.adapter, &record.draws[scalar_width..]);
                    let expected = encoded_g2_bytes(
                        &payload.adapter,
                        &product_g2(
                            &payload.adapter,
                            g2.clone(),
                            sum_scalar(&payload.adapter, a, b),
                        ),
                    );
                    assert_eq!(record.output, expected);
                }
                for record in &second_group.g1_msm {
                    let mut sum = None;
                    for term in record.draws.chunks_exact(2 * scalar_width) {
                        let a = decoded_scalar(&payload.adapter, &term[..scalar_width]);
                        let s = decoded_scalar(&payload.adapter, &term[scalar_width..]);
                        let product = product_scalar(&payload.adapter, a, s);
                        sum = Some(match sum {
                            Some(total) => sum_scalar(&payload.adapter, total, product),
                            None => product,
                        });
                    }
                    let expected = encoded_g1_bytes(
                        &payload.adapter,
                        &product_g1(
                            &payload.adapter,
                            g1.clone(),
                            sum.expect("each msm record holds at least one term"),
                        ),
                    );
                    assert_eq!(record.output, expected);
                }
                for record in &second_group.g2_msm {
                    let mut sum = None;
                    for term in record.draws.chunks_exact(2 * scalar_width) {
                        let a = decoded_scalar(&payload.adapter, &term[..scalar_width]);
                        let s = decoded_scalar(&payload.adapter, &term[scalar_width..]);
                        let product = product_scalar(&payload.adapter, a, s);
                        sum = Some(match sum {
                            Some(total) => sum_scalar(&payload.adapter, total, product),
                            None => product,
                        });
                    }
                    let expected = encoded_g2_bytes(
                        &payload.adapter,
                        &product_g2(
                            &payload.adapter,
                            g2.clone(),
                            sum.expect("each msm record holds at least one term"),
                        ),
                    );
                    assert_eq!(record.output, expected);
                }
            }
            result
        }
    }

    let consumer = Consumer {
        second_group: Some(build_second_group_vector_counts(
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
        )),
        random: source,
    };
    for concrete in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(&consumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        created.expect("the pairing factory ran the consumer");
    }
    // each_second_group_output_is_the_references_result_for_its_drawn_operands
}

/// Contract: entry `each_msm_term_carries_its_multiplier_in_its_input`; for
///   each term of each multi-scalar-multiplication record, the multiplier's
///   encoding in `input` equals the encoding of the term's second draw in
///   `draws`.
/// Arrange: the settings of `a_both_groups_pairing_emits_the_second_group_
///   kinds_at_their_configured_counts`; the consumer reads the widths
///   `encode_g1`, `encode_g2`, and `encode_scalar` write.
/// Act:     `group_vectors::<P>` once in the consumer, only where the
///   declaration is `BothGroups`.
/// Assert:  in place, for each term `i` of each `g1_msm` and `g2_msm`
///   record, the multiplier's encoding at the end of the term's slice of
///   `input` equals the encoding of the term's second draw, the bytes of
///   `draws` that follow the term's first draw.
/// Boundary: `create_pairing`, the arm's real concrete's operations, and
///   `group_vectors`.
/// Mocked:  nothing, because the operating system's generator is the outer
///   edge and runs real.
#[test]
fn each_msm_term_carries_its_multiplier_in_its_input() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;

    struct Consumer {
        second_group: Option<SecondGroupVectorCounts>,
        random: Box<dyn IRandomSourceAdapter>,
    }

    impl IPairingConsumer for &Consumer {
        type Output = GroupVectorsReturn;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            let second_group = match P::DECLARATION.verifier_group_arithmetic {
                VerifierGroupArithmetic::BothGroups => self.second_group,
                VerifierGroupArithmetic::FirstGroupOnly => None,
            };
            let params = build_group_vectors_params(GroupVectorsParamsOverrides {
                second_group: Some(second_group),
                ..Default::default()
            });
            let g1_width =
                encoded_g1_bytes(&payload.adapter, &generator_g1(&payload.adapter)).len();
            let g2_width =
                encoded_g2_bytes(&payload.adapter, &generator_g2(&payload.adapter)).len();
            let scalar_width = scalar_encoding_width(&payload.adapter, self.random.as_ref());

            // Act
            let result = group_vectors::<P>(
                &build_group_vectors_deps(
                    &payload.adapter,
                    self.random.as_ref(),
                    Default::default(),
                ),
                params,
                build_group_vectors_payload(Default::default()),
            );

            // Assert
            if let VerifierGroupArithmetic::BothGroups = P::DECLARATION.verifier_group_arithmetic {
                let Ok(vectors) = &result else {
                    panic!("the configured counts are admitted")
                };
                let second_group = vectors
                    .second_group
                    .as_ref()
                    .expect("a both-groups pairing emits its second group");
                for record in &second_group.g1_msm {
                    let term_width = g1_width + scalar_width;
                    let terms = record.input.len() / term_width;
                    for index in 0..terms {
                        let multiplier =
                            &record.input[index * term_width + g1_width..(index + 1) * term_width];
                        let drawn = &record.draws
                            [(2 * index + 1) * scalar_width..(2 * index + 2) * scalar_width];
                        assert_eq!(
                            multiplier, drawn,
                            "each term's multiplier encoding is its second draw"
                        );
                    }
                }
                for record in &second_group.g2_msm {
                    let term_width = g2_width + scalar_width;
                    let terms = record.input.len() / term_width;
                    for index in 0..terms {
                        let multiplier =
                            &record.input[index * term_width + g2_width..(index + 1) * term_width];
                        let drawn = &record.draws
                            [(2 * index + 1) * scalar_width..(2 * index + 2) * scalar_width];
                        assert_eq!(
                            multiplier, drawn,
                            "each term's multiplier encoding is its second draw"
                        );
                    }
                }
            }
            result
        }
    }

    let consumer = Consumer {
        second_group: Some(build_second_group_vector_counts(
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
        )),
        random: source,
    };
    for concrete in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(&consumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        created.expect("the pairing factory ran the consumer");
    }
    // each_msm_term_carries_its_multiplier_in_its_input
}
