#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use encoding::{IEncodingContract, build_from_fields_params, build_to_fields_params};
use pairing::{
    AddScalarParams, AddScalarPayload, CreatePairingParamsOverrides, DecodeScalarParams,
    EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload, EncodeScalarParams,
    EncodeScalarPayload, G1GeneratorParams, G1GeneratorPayload, G2GeneratorParams,
    G2GeneratorPayload, IPairingArithmetic, IPairingReference, MulG1Params, MulG1Payload,
    MulG2Params, MulG2Payload, MulScalarParams, MulScalarPayload, PAIRING_CONCRETES,
    PairingConcrete, VerifierGroupArithmetic, build_create_pairing_params,
};
use random::{
    CreateRandomSourceParamsOverrides, IRandomSourceAdapter, RandomSourceKind,
    build_create_random_source_params,
};

use super::super::super::factory::provides::{
    MsmTermCountConstructorParamsOverrides, VectorCountConstructorParamsOverrides,
    build_msm_term_count, build_vector_count,
};
use super::super::render::provides::{
    SolidityMemberNameConstructorParamsOverrides, build_solidity_member_name,
};
use super::group_vectors;
use super::interface::{
    FirstGroupVectorCounts, GroupVectorsCaseParams, GroupVectorsCasePayload,
    GroupVectorsErrorReturn, IGroupVectorsCaseConsumer, SecondGroupVectorCounts,
};
use super::mock::{
    FirstGroupVectorCountsOverrides, GroupOperationVectorOverrides,
    GroupVectorsCaseParamsOverrides, GroupVectorsParamsOverrides, MsmVectorSettingsOverrides,
    PairingCheckVectorOverrides, SecondGroupVectorCountsOverrides,
    build_first_group_vector_counts, build_group_operation_vector,
    build_group_operation_vector_description, build_group_vectors_case_deps,
    build_group_vectors_case_params, build_group_vectors_case_payload,
    build_group_vectors_deps, build_group_vectors_params, build_group_vectors_payload,
    build_msm_vector_settings, build_pairing_check_vector,
    build_pairing_check_vector_description, build_second_group_vector_counts,
    run_group_vectors_case,
};

fn case_params(pairing_concrete: &PairingConcrete) -> GroupVectorsCaseParams {
    build_group_vectors_case_params(GroupVectorsCaseParamsOverrides {
        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*pairing_concrete),
            ..Default::default()
        })),
        random_source: Some(build_create_random_source_params(
            CreateRandomSourceParamsOverrides {
                kind: Some(RandomSourceKind::OperatingSystem),
            },
        )),
    })
}

/// Contract: each description's `fields_to_value`, called with the fields its
///   `to_fields` returns for a record whose three fields differ from one another in
///   value and length, returns `Ok` whose `described` equals the record.
/// Arrange: `build_group_operation_vector` with `draws` `vec![0x0A, 0x0B]`, `input`
///   `vec![0x0C]`, and `output` `vec![0x0D, 0x0E, 0x0F]`, and
///   `build_pairing_check_vector` with `draws` `vec![0x0A]`, `input`
///   `vec![0x0B, 0x0C, 0x0D]`, and `output` `[0x0E; 32]`, each with its description
///   from `build_group_operation_vector_description(Default::default())` or
///   `build_pairing_check_vector_description(Default::default())` and with
///   `build_to_fields_params(Default::default())` and
///   `build_from_fields_params(Default::default())`.
/// Act:    `to_fields` on each record, then `fields_to_value` on the fields it
///   returned.
/// Assert: each `described` equals the record it was built from.
/// Boundary: each description's `to_fields` then `fields_to_value`, both real.
/// Mocked:  nothing, because the chain has no outer-edge collaborator.
#[test]
fn a_record_survives_its_description_round_trip() {
    // Arrange
    let group_operation = build_group_operation_vector(GroupOperationVectorOverrides {
        draws: Some(vec![0x0A, 0x0B]),
        input: Some(vec![0x0C]),
        output: Some(vec![0x0D, 0x0E, 0x0F]),
    });
    let pairing_check = build_pairing_check_vector(PairingCheckVectorOverrides {
        draws: Some(vec![0x0A]),
        input: Some(vec![0x0B, 0x0C, 0x0D]),
        output: Some([0x0E; 32]),
    });
    let group_operation_description = build_group_operation_vector_description(Default::default());
    let pairing_check_description = build_pairing_check_vector_description(Default::default());

    // Act
    let group_operation_fields = group_operation_description
        .to_fields(build_to_fields_params(Default::default()), &group_operation)
        .expect("the description maps the record")
        .fields;
    let group_operation_described = group_operation_description
        .fields_to_value(
            build_from_fields_params(Default::default()),
            group_operation_fields,
        )
        .expect("the fields rebuild the record")
        .described;
    let pairing_check_fields = pairing_check_description
        .to_fields(build_to_fields_params(Default::default()), &pairing_check)
        .expect("the description maps the record")
        .fields;
    let pairing_check_described = pairing_check_description
        .fields_to_value(
            build_from_fields_params(Default::default()),
            pairing_check_fields,
        )
        .expect("the fields rebuild the record")
        .described;

    // Assert
    assert_eq!(group_operation_described, group_operation);
    assert_eq!(pairing_check_described, pairing_check);
}
// a_record_survives_its_description_round_trip

/// Contract: on each description over a built record, `member_names()` returns `Ok`
///   of the names `draws`, `input`, and `output` in order, and their count equals
///   the number of values `to_fields` returns.
/// Arrange: `build_group_operation_vector` and `build_pairing_check_vector` at
///   their defaults, each with its description and `build_to_fields_params`.
/// Act:    `to_fields` and `member_names()` on each description.
/// Assert: each `member_names()` result equals `Ok` of the names `draws`, `input`,
///   and `output` through `build_solidity_member_name`, and its length equals the
///   length of the values `to_fields` returned.
/// Boundary: `member_names()` over the real `SolidityMemberName::try_new`, and
///   `to_fields`.
/// Mocked:  nothing, because the chain has no outer-edge collaborator.
#[test]
fn member_names_declare_one_name_per_field() {
    // Arrange
    let group_operation = build_group_operation_vector(Default::default());
    let pairing_check = build_pairing_check_vector(Default::default());
    let group_operation_description = build_group_operation_vector_description(Default::default());
    let pairing_check_description = build_pairing_check_vector_description(Default::default());
    let expected = vec![
        build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
            text: Some("draws".to_string()),
        }),
        build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
            text: Some("input".to_string()),
        }),
        build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
            text: Some("output".to_string()),
        }),
    ];

    // Act
    let group_operation_fields = group_operation_description
        .to_fields(build_to_fields_params(Default::default()), &group_operation)
        .expect("the description maps the record")
        .fields;
    let group_operation_names = group_operation_description
        .member_names()
        .expect("the names admit");
    let pairing_check_fields = pairing_check_description
        .to_fields(build_to_fields_params(Default::default()), &pairing_check)
        .expect("the description maps the record")
        .fields;
    let pairing_check_names = pairing_check_description
        .member_names()
        .expect("the names admit");

    // Assert
    assert_eq!(group_operation_names, expected);
    assert_eq!(
        group_operation_names.len(),
        group_operation_fields.values.len()
    );
    assert_eq!(pairing_check_names, expected);
    assert_eq!(pairing_check_names.len(), pairing_check_fields.values.len());
}
// member_names_declare_one_name_per_field

/// Contract: `group_vectors` called with `second_group` absent returns
///   `Err(GroupVectorsErrorReturn::SecondGroupSettingsMissing)` where
///   `P::DECLARATION.verifier_group_arithmetic` is `BothGroups`, and `Ok` with
///   `second_group` absent where it is `FirstGroupOnly`.
/// Arrange: the consumer's `second_group` absent whatever the declaration,
///   `build_group_vectors_params` with `second_group` overridden to `None`, run
///   once per member of `PAIRING_CONCRETES`.
/// Act:    `run_group_vectors_case` per member, the consumer's method calling
///   `group_vectors::<P>`.
/// Assert: in place, where the declaration is `BothGroups` the result equals
///   `Err(GroupVectorsErrorReturn::SecondGroupSettingsMissing)`, and where it is
///   `FirstGroupOnly` the result is `Ok` whose `second_group` is `None`.
/// Boundary: `create_pairing`, the arm's real concrete's declaration, and
///   `group_vectors`.
/// Mocked:  nothing, because the operating system's generator is the outer edge
///   and runs real.
#[test]
fn a_both_groups_pairing_requires_its_second_group_settings() {
    // Arrange
    struct Consumer;

    impl IGroupVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_group_vectors_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            random: &dyn IRandomSourceAdapter,
            _payload: &GroupVectorsCasePayload,
        ) -> Self::Output {
            // Arrange
            let params = build_group_vectors_params(GroupVectorsParamsOverrides {
                second_group: Some(None),
                ..Default::default()
            });

            // Act
            let result = group_vectors::<P>(
                &build_group_vectors_deps(pairing, random, Default::default()),
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
                    assert!(
                        result
                            .expect("absent second-group settings admit")
                            .second_group
                            .is_none()
                    );
                }
            }
        }
    }

    // Act — over the declared set
    for pairing_concrete in PAIRING_CONCRETES {
        run_group_vectors_case(
            &build_group_vectors_case_deps(Consumer, Default::default()),
            case_params(pairing_concrete),
            build_group_vectors_case_payload(Default::default()),
        )
        .expect("every factory ran");
    }
}
// a_both_groups_pairing_requires_its_second_group_settings

/// Contract: `group_vectors` called with settings where the declaration is
///   `BothGroups` returns `second_group` holding exactly the configured records of
///   each kind, each multi-scalar-multiplication record's `input` holding its
///   configured terms, each a base encoding and a multiplier encoding, and with
///   `second_group` absent where it is `FirstGroupOnly` returns `second_group`
///   absent.
/// Arrange: settings from `build_second_group_vector_counts` with `g2_add` from
///   `build_vector_count` over `2`, `g1_msm` from `build_msm_vector_settings` with
///   `count` `2` and `terms` `3`, and `g2_msm` from `build_msm_vector_settings`
///   with `count` `1` and `terms` `2`; the consumer reads the widths `encode_g1`,
///   `encode_g2`, and `encode_scalar` write.
/// Act:    `run_group_vectors_case` per member, the consumer's method calling
///   `group_vectors::<P>`.
/// Assert: in place, where the declaration is `BothGroups` `second_group` is `Some`
///   holding `2`, `2`, and `1` records, each `g1_msm` record's `input` holds `3`
///   terms of one `encode_g1` width plus one `encode_scalar` width, and each
///   `g2_msm` record's `input` holds `2` terms of one `encode_g2` width plus one
///   `encode_scalar` width, and where it is `FirstGroupOnly` `second_group` is
///   `None`.
/// Boundary: `create_pairing`, the arm's real concrete's operations, and
///   `group_vectors`.
/// Mocked:  nothing, because the operating system's generator is the outer edge
///   and runs real.
#[test]
fn a_both_groups_pairing_emits_the_second_group_kinds_at_their_configured_counts() {
    // Arrange
    struct Consumer {
        settings: SecondGroupVectorCounts,
    }

    impl IGroupVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_group_vectors_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            random: &dyn IRandomSourceAdapter,
            _payload: &GroupVectorsCasePayload,
        ) -> Self::Output {
            // Arrange
            let params = build_group_vectors_params(GroupVectorsParamsOverrides {
                second_group: Some(match P::DECLARATION.verifier_group_arithmetic {
                    VerifierGroupArithmetic::BothGroups => Some(self.settings),
                    VerifierGroupArithmetic::FirstGroupOnly => None,
                }),
                ..Default::default()
            });

            // Act
            let result = group_vectors::<P>(
                &build_group_vectors_deps(pairing, random, Default::default()),
                params,
                build_group_vectors_payload(Default::default()),
            );

            // Assert
            match P::DECLARATION.verifier_group_arithmetic {
                VerifierGroupArithmetic::BothGroups => {
                    let second_group = result
                        .expect("the settings admit")
                        .second_group
                        .expect("a BothGroups declaration emits the second-group kinds");
                    assert_eq!(second_group.g2_add.len(), 2);
                    assert_eq!(second_group.g1_msm.len(), 2);
                    assert_eq!(second_group.g2_msm.len(), 1);
                    let Ok(g1) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
                    let Ok(encoded_g1) = pairing.encode_g1(
                        EncodeG1Params,
                        EncodeG1Payload {
                            point: g1.point.clone(),
                        },
                    );
                    let g1_width = encoded_g1.bytes.as_ref().len();
                    let Ok(g2) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
                    let Ok(encoded_g2) = pairing.encode_g2(
                        EncodeG2Params,
                        EncodeG2Payload {
                            point: g2.point.clone(),
                        },
                    );
                    let g2_width = encoded_g2.bytes.as_ref().len();
                    let scalar_width = second_group.g2_add[0].draws.len() / 2;
                    let decoded = pairing
                        .decode_scalar(
                            DecodeScalarParams,
                            &second_group.g2_add[0].draws[..scalar_width],
                        )
                        .expect("a drawn scalar decodes");
                    let Ok(encoded_scalar) = pairing.encode_scalar(
                        EncodeScalarParams,
                        EncodeScalarPayload {
                            scalar: decoded.scalar,
                        },
                    );
                    let scalar_width = encoded_scalar.bytes.expose().as_ref().len();
                    for record in &second_group.g1_msm {
                        assert_eq!(record.input.len(), 3 * (g1_width + scalar_width));
                    }
                    for record in &second_group.g2_msm {
                        assert_eq!(record.input.len(), 2 * (g2_width + scalar_width));
                    }
                }
                VerifierGroupArithmetic::FirstGroupOnly => {
                    assert!(
                        result
                            .expect("absent second-group settings admit")
                            .second_group
                            .is_none()
                    );
                }
            }
        }
    }

    let consumer = Consumer {
        settings: build_second_group_vector_counts(SecondGroupVectorCountsOverrides {
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
        }),
    };

    // Act — over the declared set
    for pairing_concrete in PAIRING_CONCRETES {
        run_group_vectors_case(
            &build_group_vectors_case_deps(
                Consumer {
                    settings: consumer.settings,
                },
                Default::default(),
            ),
            case_params(pairing_concrete),
            build_group_vectors_case_payload(Default::default()),
        )
        .expect("every factory ran");
    }
}
// a_both_groups_pairing_emits_the_second_group_kinds_at_their_configured_counts

/// Contract: on every concrete of `PAIRING_CONCRETES`, each first-group addition
///   record's `output` is the encoding of the first-group generator times the sum
///   of its two draws, and each first-group multiplication record's `output` is the
///   encoding of the first-group generator times the product of its two draws.
/// Arrange: `build_first_group_vector_counts` with `g1_add` and `g1_mul` from
///   `build_vector_count` over `3`, so more than one record of each kind is drawn,
///   and `pairing_check` at its default; the consumer reads the width
///   `encode_scalar` writes, reads each record's draws back through
///   `decode_scalar`, and builds the expected outputs from them: for each `g1_add`
///   record the encoding of `mul_g1(g1, add_scalar(a, b))` through `encode_g1`, and
///   for each `g1_mul` record the encoding of `mul_g1(g1, mul_scalar(a, s))`
///   through `encode_g1`.
/// Act:    `run_group_vectors_case` per member, the consumer's method calling
///   `group_vectors::<P>`.
/// Assert: in place, each `g1_add` and `g1_mul` record's `output` equals its
///   expected output.
/// Boundary: `create_pairing`, the arm's real concrete's operations, and
///   `group_vectors`.
/// Mocked:  nothing, because the operating system's generator is the outer edge
///   and runs real.
#[test]
fn each_first_group_output_is_the_references_result_for_its_drawn_operands() {
    // Arrange
    struct Consumer {
        first_group: FirstGroupVectorCounts,
        settings: SecondGroupVectorCounts,
    }

    impl IGroupVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_group_vectors_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            random: &dyn IRandomSourceAdapter,
            _payload: &GroupVectorsCasePayload,
        ) -> Self::Output {
            // Arrange
            let params = build_group_vectors_params(GroupVectorsParamsOverrides {
                first_group: Some(self.first_group),
                second_group: Some(match P::DECLARATION.verifier_group_arithmetic {
                    VerifierGroupArithmetic::BothGroups => Some(self.settings),
                    VerifierGroupArithmetic::FirstGroupOnly => None,
                }),
            });

            // Act
            let result = group_vectors::<P>(
                &build_group_vectors_deps(pairing, random, Default::default()),
                params,
                build_group_vectors_payload(Default::default()),
            );

            // Assert
            let success = result.expect("the params admit");
            let scalar_width = success.first_group.g1_add[0].draws.len() / 2;
            let Ok(g1) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
            for record in &success.first_group.g1_add {
                let a = pairing
                    .decode_scalar(DecodeScalarParams, &record.draws[..scalar_width])
                    .expect("the first draw decodes")
                    .scalar;
                let b = pairing
                    .decode_scalar(DecodeScalarParams, &record.draws[scalar_width..])
                    .expect("the second draw decodes")
                    .scalar;
                let Ok(sum) =
                    pairing.add_scalar(AddScalarParams, AddScalarPayload { left: a, right: b });
                let Ok(product) = pairing.mul_g1(
                    MulG1Params,
                    MulG1Payload {
                        point: g1.point.clone(),
                        scalar: sum.sum,
                    },
                );
                let Ok(expected) = pairing.encode_g1(
                    EncodeG1Params,
                    EncodeG1Payload {
                        point: product.product,
                    },
                );
                assert_eq!(record.output.as_slice(), expected.bytes.as_ref());
            }
            for record in &success.first_group.g1_mul {
                let a = pairing
                    .decode_scalar(DecodeScalarParams, &record.draws[..scalar_width])
                    .expect("the first draw decodes")
                    .scalar;
                let s = pairing
                    .decode_scalar(DecodeScalarParams, &record.draws[scalar_width..])
                    .expect("the second draw decodes")
                    .scalar;
                let Ok(product) =
                    pairing.mul_scalar(MulScalarParams, MulScalarPayload { left: a, right: s });
                let Ok(point) = pairing.mul_g1(
                    MulG1Params,
                    MulG1Payload {
                        point: g1.point.clone(),
                        scalar: product.product,
                    },
                );
                let Ok(expected) = pairing.encode_g1(
                    EncodeG1Params,
                    EncodeG1Payload {
                        point: point.product,
                    },
                );
                assert_eq!(record.output.as_slice(), expected.bytes.as_ref());
            }
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
        settings: build_second_group_vector_counts(Default::default()),
    };

    // Act — over the declared set
    for pairing_concrete in PAIRING_CONCRETES {
        run_group_vectors_case(
            &build_group_vectors_case_deps(
                Consumer {
                    first_group: consumer.first_group,
                    settings: consumer.settings,
                },
                Default::default(),
            ),
            case_params(pairing_concrete),
            build_group_vectors_case_payload(Default::default()),
        )
        .expect("every factory ran");
    }
}
// each_first_group_output_is_the_references_result_for_its_drawn_operands

/// Contract: on every concrete whose declaration is `BothGroups`, each
///   second-group addition record's `output` is the encoding of the second-group
///   generator times the sum of its two draws, and each multi-scalar-multiplication
///   record's `output` is the encoding of the group's generator times the sum over
///   its terms of the product of the term's two draws.
/// Arrange: the settings of
///   `a_both_groups_pairing_emits_the_second_group_kinds_at_their_configured_counts`;
///   the consumer reads each record's draws back through `decode_scalar` and builds
///   the expected outputs from them, each sum through `add_scalar`.
/// Act:    `run_group_vectors_case` per member, the consumer's method calling
///   `group_vectors::<P>`.
/// Assert: in place, each second-group record's `output` equals its expected
///   output.
/// Boundary: `create_pairing`, the arm's real concrete's operations, and
///   `group_vectors`.
/// Mocked:  nothing, because the operating system's generator is the outer edge
///   and runs real.
#[test]
fn each_second_group_output_is_the_references_result_for_its_drawn_operands() {
    // Arrange
    struct Consumer {
        settings: SecondGroupVectorCounts,
    }

    impl IGroupVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_group_vectors_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            random: &dyn IRandomSourceAdapter,
            _payload: &GroupVectorsCasePayload,
        ) -> Self::Output {
            // Arrange
            let params = build_group_vectors_params(GroupVectorsParamsOverrides {
                second_group: Some(match P::DECLARATION.verifier_group_arithmetic {
                    VerifierGroupArithmetic::BothGroups => Some(self.settings),
                    VerifierGroupArithmetic::FirstGroupOnly => None,
                }),
                ..Default::default()
            });

            // Act
            let result = group_vectors::<P>(
                &build_group_vectors_deps(pairing, random, Default::default()),
                params,
                build_group_vectors_payload(Default::default()),
            );

            // Assert
            if let VerifierGroupArithmetic::BothGroups = P::DECLARATION.verifier_group_arithmetic {
                let second_group = result
                    .expect("the settings admit")
                    .second_group
                    .expect("a BothGroups declaration emits the second-group kinds");
                let Ok(g1) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
                let Ok(g2) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
                for record in &second_group.g2_add {
                    let scalar_width = record.draws.len() / 2;
                    let a = pairing
                        .decode_scalar(DecodeScalarParams, &record.draws[..scalar_width])
                        .expect("the first draw decodes")
                        .scalar;
                    let b = pairing
                        .decode_scalar(DecodeScalarParams, &record.draws[scalar_width..])
                        .expect("the second draw decodes")
                        .scalar;
                    let Ok(sum) =
                        pairing.add_scalar(AddScalarParams, AddScalarPayload { left: a, right: b });
                    let Ok(product) = pairing.mul_g2(
                        MulG2Params,
                        MulG2Payload {
                            point: g2.point.clone(),
                            scalar: sum.sum,
                        },
                    );
                    let Ok(expected) = pairing.encode_g2(
                        EncodeG2Params,
                        EncodeG2Payload {
                            point: product.product,
                        },
                    );
                    assert_eq!(record.output.as_slice(), expected.bytes.as_ref());
                }
                for record in &second_group.g1_msm {
                    let scalar_width = record.draws.len() / 6;
                    let mut sum = None;
                    for term in record.draws.chunks(2 * scalar_width) {
                        let a = pairing
                            .decode_scalar(DecodeScalarParams, &term[..scalar_width])
                            .expect("the term's first draw decodes")
                            .scalar;
                        let s = pairing
                            .decode_scalar(DecodeScalarParams, &term[scalar_width..])
                            .expect("the term's second draw decodes")
                            .scalar;
                        let Ok(product) = pairing
                            .mul_scalar(MulScalarParams, MulScalarPayload { left: a, right: s });
                        sum = Some(match sum {
                            Some(acc) => {
                                let Ok(next) = pairing.add_scalar(
                                    AddScalarParams,
                                    AddScalarPayload {
                                        left: acc,
                                        right: product.product,
                                    },
                                );
                                next.sum
                            }
                            None => product.product,
                        });
                    }
                    let Ok(product) = pairing.mul_g1(
                        MulG1Params,
                        MulG1Payload {
                            point: g1.point.clone(),
                            scalar: sum.expect("each record holds at least one term"),
                        },
                    );
                    let Ok(expected) = pairing.encode_g1(
                        EncodeG1Params,
                        EncodeG1Payload {
                            point: product.product,
                        },
                    );
                    assert_eq!(record.output.as_slice(), expected.bytes.as_ref());
                }
                for record in &second_group.g2_msm {
                    let scalar_width = record.draws.len() / 4;
                    let mut sum = None;
                    for term in record.draws.chunks(2 * scalar_width) {
                        let a = pairing
                            .decode_scalar(DecodeScalarParams, &term[..scalar_width])
                            .expect("the term's first draw decodes")
                            .scalar;
                        let s = pairing
                            .decode_scalar(DecodeScalarParams, &term[scalar_width..])
                            .expect("the term's second draw decodes")
                            .scalar;
                        let Ok(product) = pairing
                            .mul_scalar(MulScalarParams, MulScalarPayload { left: a, right: s });
                        sum = Some(match sum {
                            Some(acc) => {
                                let Ok(next) = pairing.add_scalar(
                                    AddScalarParams,
                                    AddScalarPayload {
                                        left: acc,
                                        right: product.product,
                                    },
                                );
                                next.sum
                            }
                            None => product.product,
                        });
                    }
                    let Ok(product) = pairing.mul_g2(
                        MulG2Params,
                        MulG2Payload {
                            point: g2.point.clone(),
                            scalar: sum.expect("each record holds at least one term"),
                        },
                    );
                    let Ok(expected) = pairing.encode_g2(
                        EncodeG2Params,
                        EncodeG2Payload {
                            point: product.product,
                        },
                    );
                    assert_eq!(record.output.as_slice(), expected.bytes.as_ref());
                }
            }
        }
    }

    let consumer = Consumer {
        settings: build_second_group_vector_counts(SecondGroupVectorCountsOverrides {
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
        }),
    };

    // Act — over the declared set
    for pairing_concrete in PAIRING_CONCRETES {
        run_group_vectors_case(
            &build_group_vectors_case_deps(
                Consumer {
                    settings: consumer.settings,
                },
                Default::default(),
            ),
            case_params(pairing_concrete),
            build_group_vectors_case_payload(Default::default()),
        )
        .expect("every factory ran");
    }
}
// each_second_group_output_is_the_references_result_for_its_drawn_operands

/// Contract: for each term of each multi-scalar-multiplication record, the
///   multiplier's encoding in `input` equals the encoding of the term's second
///   draw in `draws`.
/// Arrange: the settings of
///   `a_both_groups_pairing_emits_the_second_group_kinds_at_their_configured_counts`;
///   the consumer reads the widths `encode_g1`, `encode_g2`, and `encode_scalar`
///   write.
/// Act:    `run_group_vectors_case` per member, the consumer's method calling
///   `group_vectors::<P>`.
/// Assert: in place, for each term `i` of each `g1_msm` and `g2_msm` record, the
///   multiplier's encoding at the end of the term's slice of `input` equals the
///   encoding of the term's second draw, the bytes of `draws` that follow the
///   term's first draw.
/// Boundary: `create_pairing`, the arm's real concrete's operations, and
///   `group_vectors`.
/// Mocked:  nothing, because the operating system's generator is the outer edge
///   and runs real.
#[test]
fn each_msm_term_carries_its_multiplier_in_its_input() {
    // Arrange
    struct Consumer {
        settings: SecondGroupVectorCounts,
    }

    impl IGroupVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_group_vectors_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            random: &dyn IRandomSourceAdapter,
            _payload: &GroupVectorsCasePayload,
        ) -> Self::Output {
            // Arrange
            let params = build_group_vectors_params(GroupVectorsParamsOverrides {
                second_group: Some(match P::DECLARATION.verifier_group_arithmetic {
                    VerifierGroupArithmetic::BothGroups => Some(self.settings),
                    VerifierGroupArithmetic::FirstGroupOnly => None,
                }),
                ..Default::default()
            });

            // Act
            let result = group_vectors::<P>(
                &build_group_vectors_deps(pairing, random, Default::default()),
                params,
                build_group_vectors_payload(Default::default()),
            );

            // Assert
            if let VerifierGroupArithmetic::BothGroups = P::DECLARATION.verifier_group_arithmetic {
                let second_group = result
                    .expect("the settings admit")
                    .second_group
                    .expect("a BothGroups declaration emits the second-group kinds");
                let Ok(g1) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
                let Ok(encoded_g1) = pairing.encode_g1(
                    EncodeG1Params,
                    EncodeG1Payload {
                        point: g1.point.clone(),
                    },
                );
                let g1_width = encoded_g1.bytes.as_ref().len();
                let Ok(g2) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
                let Ok(encoded_g2) = pairing.encode_g2(
                    EncodeG2Params,
                    EncodeG2Payload {
                        point: g2.point.clone(),
                    },
                );
                let g2_width = encoded_g2.bytes.as_ref().len();
                let scalar_width = second_group.g2_add[0].draws.len() / 2;
                let decoded = pairing
                    .decode_scalar(
                        DecodeScalarParams,
                        &second_group.g2_add[0].draws[..scalar_width],
                    )
                    .expect("a drawn scalar decodes");
                let Ok(encoded_scalar) = pairing.encode_scalar(
                    EncodeScalarParams,
                    EncodeScalarPayload {
                        scalar: decoded.scalar,
                    },
                );
                let scalar_width = encoded_scalar.bytes.expose().as_ref().len();
                for (records, base_width, terms) in [
                    (&second_group.g1_msm, g1_width, 3usize),
                    (&second_group.g2_msm, g2_width, 2usize),
                ] {
                    for record in records {
                        for index in 0..terms {
                            let term_input = &record.input[index * (base_width + scalar_width)
                                ..(index + 1) * (base_width + scalar_width)];
                            assert_eq!(
                                &term_input[base_width..],
                                &record.draws[(2 * index + 1) * scalar_width
                                    ..(2 * index + 2) * scalar_width]
                            );
                        }
                    }
                }
            }
        }
    }

    let consumer = Consumer {
        settings: build_second_group_vector_counts(SecondGroupVectorCountsOverrides {
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
        }),
    };

    // Act — over the declared set
    for pairing_concrete in PAIRING_CONCRETES {
        run_group_vectors_case(
            &build_group_vectors_case_deps(
                Consumer {
                    settings: consumer.settings,
                },
                Default::default(),
            ),
            case_params(pairing_concrete),
            build_group_vectors_case_payload(Default::default()),
        )
        .expect("every factory ran");
    }
}
// each_msm_term_carries_its_multiplier_in_its_input
