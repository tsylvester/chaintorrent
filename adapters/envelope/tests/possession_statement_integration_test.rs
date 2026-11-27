#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use encoding::{
    CanonicalFieldValue, CanonicalFields, IEncodingContract, build_from_fields_params,
    build_to_fields_params,
};
use envelope::{
    PossessionG1StatementCorruptions, PossessionG2StatementCorruptions,
    PossessionStatementFromFieldsErrorReturn, build_possession_g1_statement,
    build_possession_g1_statement_description, build_possession_g2_statement,
    build_possession_g2_statement_description, invalidate_possession_g1_statement,
    invalidate_possession_g2_statement,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParamsOverrides,
    CreatePairingPayload, DecodeG1ErrorReturn, DecodeG2ErrorReturn,
    G1OutsideSubgroupEncodingErrorReturn, G1OutsideSubgroupEncodingParams,
    G1OutsideSubgroupEncodingPayload, G2OutsideSubgroupEncodingErrorReturn,
    G2OutsideSubgroupEncodingParams, G2OutsideSubgroupEncodingPayload, IPairingAdapter,
    IPairingConsumer, IPairingReference, LookupPairingDeclarationParamsOverrides,
    PAIRING_CONCRETES, PairingConcreteOverrides, TargetGroupEncodingIdentifier,
    admit_pairing_composition, build_admit_pairing_composition_deps, build_create_pairing_deps,
    build_create_pairing_params, build_lookup_pairing_declaration_deps,
    build_lookup_pairing_declaration_params, build_lookup_pairing_declaration_payload,
    build_pairing_concrete, create_pairing, lookup_pairing_declaration,
};

/// Contract: entry `a_first_group_statement_survives_the_round_trip_through_its_fields`;
///   given a statement of a concrete's generator and that generator added to
///   itself, the fields the description flattens the decoded statement to equal
///   the fields it flattened the original to.
/// Boundary: the public `create_pairing`; the real chain is `create_pairing`,
///   the arm's `try_new`, `consume_pairing`, the description's `try_new`,
///   `to_fields`, `encode_g1`, `fields_to_value`, `decode_g1`, and `to_fields`
///   again.
/// Mocked:   none.
/// Arrange:  a `Consumer` written against `IPairingAdapter` building the
///   description and the statement through the official builders, flattening
///   the statement, decoding a clone of the fields, flattening the decoded
///   statement, and returning both field lists; every concrete of the declared
///   set in turn; the key differs from the commitment, so a swapped or repeated
///   field fails.
/// Act:      `create_pairing(&deps, params, CreatePairingPayload)` per concrete.
/// Assert:   per concrete, the output's second member equals `Ok` of its first
///   member, and the first member's two values differ, so the variation held.
#[test]
fn a_first_group_statement_survives_the_round_trip_through_its_fields() {
    // Arrange
    #[derive(Default)]
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = (
            CanonicalFields,
            Result<CanonicalFields, PossessionStatementFromFieldsErrorReturn>,
        );

        fn consume_pairing<P: IPairingAdapter>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let description =
                build_possession_g1_statement_description(&payload.adapter, Default::default());
            let statement = build_possession_g1_statement(&payload.adapter, Default::default());
            let Ok(original) =
                description.to_fields(build_to_fields_params(Default::default()), &statement);
            let second = description
                .fields_to_value(
                    build_from_fields_params(Default::default()),
                    original.fields.clone(),
                )
                .map(|success| {
                    let Ok(flattened) = description.to_fields(
                        build_to_fields_params(Default::default()),
                        &success.described,
                    );
                    flattened.fields
                });
            (original.fields, second)
        }
    }

    for concrete in PAIRING_CONCRETES {
        let selection = build_pairing_concrete(*concrete, PairingConcreteOverrides);
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(selection),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(
            Consumer,
            lookup_pairing_declaration,
            admit_pairing_composition,
            build_admit_pairing_composition_deps(lookup_pairing_declaration, Default::default()),
            Default::default(),
        );

        // Act
        let result = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        let output = result
            .ok()
            .map(|success| success.output)
            .expect("the factory admits each declared concrete");
        assert_eq!(output.1, Ok(output.0.clone()));
        assert_ne!(output.0.values[0], output.0.values[1]);
    }
}

/// Contract: entry `a_second_group_statement_survives_the_round_trip_through_its_fields`;
///   given a statement of a concrete's second-group generator and that
///   generator added to itself, the fields the description flattens the decoded
///   statement to equal the fields it flattened the original to.
/// Boundary: the public `create_pairing`; the real chain is `create_pairing`,
///   the arm's `try_new`, `consume_pairing`, the description's `try_new`,
///   `to_fields`, `encode_g2`, `fields_to_value`, `decode_g2`, and `to_fields`
///   again.
/// Mocked:   none.
/// Arrange:  the `Consumer` of the first-group round trip over
///   `build_possession_g2_statement_description` and
///   `build_possession_g2_statement`; every concrete of the declared set in
///   turn; the key differs from the commitment, so a swapped or repeated field
///   fails.
/// Act:      `create_pairing(&deps, params, CreatePairingPayload)` per concrete.
/// Assert:   per concrete, the output's second member equals `Ok` of its first
///   member, and the first member's two values differ, so the variation held.
#[test]
fn a_second_group_statement_survives_the_round_trip_through_its_fields() {
    // Arrange
    #[derive(Default)]
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = (
            CanonicalFields,
            Result<CanonicalFields, PossessionStatementFromFieldsErrorReturn>,
        );

        fn consume_pairing<P: IPairingAdapter>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let description =
                build_possession_g2_statement_description(&payload.adapter, Default::default());
            let statement = build_possession_g2_statement(&payload.adapter, Default::default());
            let Ok(original) =
                description.to_fields(build_to_fields_params(Default::default()), &statement);
            let second = description
                .fields_to_value(
                    build_from_fields_params(Default::default()),
                    original.fields.clone(),
                )
                .map(|success| {
                    let Ok(flattened) = description.to_fields(
                        build_to_fields_params(Default::default()),
                        &success.described,
                    );
                    flattened.fields
                });
            (original.fields, second)
        }
    }

    for concrete in PAIRING_CONCRETES {
        let selection = build_pairing_concrete(*concrete, PairingConcreteOverrides);
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(selection),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(
            Consumer,
            lookup_pairing_declaration,
            admit_pairing_composition,
            build_admit_pairing_composition_deps(lookup_pairing_declaration, Default::default()),
            Default::default(),
        );

        // Act
        let result = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        let output = result
            .ok()
            .map(|success| success.output)
            .expect("the factory admits each declared concrete");
        assert_eq!(output.1, Ok(output.0.clone()));
        assert_ne!(output.0.values[0], output.0.values[1]);
    }
}

/// Contract: entry
///   `libraries_sharing_an_identifier_flatten_a_first_group_statement_to_the_same_fields`;
///   any two concretes whose identifiers are equal return equal fields for the
///   same statement.
/// Boundary: the public `lookup_pairing_declaration` and `create_pairing`; the
///   real chain is `lookup_pairing_declaration`, `create_pairing`, the arm's
///   `try_new`, `consume_pairing`, the description's `try_new`, `to_fields`,
///   and `encode_g1`.
/// Mocked:   none.
/// Arrange:  a `Consumer` written against `IPairingAdapter`, holding the
///   `TargetGroupEncodingIdentifier` the looked-up declaration carries as its
///   arrangement, building the description and the statement as the
///   first-group round trip does, calling `to_fields`, and returning that
///   typed identifier with the fields; every concrete of the declared set
///   runs, collected in order, so each library of each identifier is present;
///   the same selection is used in the lookup params and the infallible
///   `lookup_pairing_declaration` result is unpacked before `create_pairing`.
/// Act:      `create_pairing(&deps, params, CreatePairingPayload)` per concrete.
/// Assert:   an output for every concrete; any two outputs with equal
///   identifiers hold equal fields.
#[test]
fn libraries_sharing_an_identifier_flatten_a_first_group_statement_to_the_same_fields() {
    // Arrange
    struct Consumer {
        identifier: TargetGroupEncodingIdentifier,
    }

    impl IPairingConsumer for Consumer {
        type Output = (TargetGroupEncodingIdentifier, CanonicalFields);

        fn consume_pairing<P: IPairingAdapter>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let description =
                build_possession_g1_statement_description(&payload.adapter, Default::default());
            let statement = build_possession_g1_statement(&payload.adapter, Default::default());
            let Ok(success) =
                description.to_fields(build_to_fields_params(Default::default()), &statement);
            (self.identifier, success.fields)
        }
    }

    let mut outputs = Vec::new();
    for concrete in PAIRING_CONCRETES {
        let selection = build_pairing_concrete(*concrete, PairingConcreteOverrides);
        let Ok(looked_up) = lookup_pairing_declaration(
            &build_lookup_pairing_declaration_deps(Default::default()),
            build_lookup_pairing_declaration_params(LookupPairingDeclarationParamsOverrides {
                concrete: Some(selection),
            }),
            build_lookup_pairing_declaration_payload(Default::default()),
        );
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(selection),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(
            Consumer {
                identifier: looked_up.declaration.target_group_encoding,
            },
            lookup_pairing_declaration,
            admit_pairing_composition,
            build_admit_pairing_composition_deps(lookup_pairing_declaration, Default::default()),
            Default::default(),
        );

        // Act
        let result = create_pairing(&deps, params, CreatePairingPayload);

        outputs.push(result.ok().map(|success| success.output));
    }

    // Assert
    assert_eq!(outputs.len(), PAIRING_CONCRETES.len());
    for left in &outputs {
        for right in &outputs {
            let (left_identifier, left_fields) = left.as_ref().expect("an output per concrete");
            let (right_identifier, right_fields) = right.as_ref().expect("an output per concrete");
            if left_identifier == right_identifier {
                assert_eq!(left_fields, right_fields);
            }
        }
    }
}

/// Contract: entry
///   `libraries_sharing_an_identifier_flatten_a_second_group_statement_to_the_same_fields`;
///   any two concretes whose identifiers are equal return equal fields for the
///   same statement.
/// Boundary: the public `lookup_pairing_declaration` and `create_pairing`; the
///   real chain is `lookup_pairing_declaration`, `create_pairing`, the arm's
///   `try_new`, `consume_pairing`, the description's `try_new`, `to_fields`,
///   and `encode_g2`.
/// Mocked:   none.
/// Arrange:  the `Consumer` of the first-group flatten block, holding its
///   looked-up `TargetGroupEncodingIdentifier` arrangement the same way, over
///   `build_possession_g2_statement_description` and
///   `build_possession_g2_statement`; every concrete of the declared set runs,
///   collected in order, so each library of each identifier is present; the
///   same selection is used in the lookup params and the infallible
///   `lookup_pairing_declaration` result is unpacked before `create_pairing`.
/// Act:      `create_pairing(&deps, params, CreatePairingPayload)` per concrete.
/// Assert:   an output for every concrete; any two outputs with equal
///   identifiers hold equal fields.
#[test]
fn libraries_sharing_an_identifier_flatten_a_second_group_statement_to_the_same_fields() {
    // Arrange
    struct Consumer {
        identifier: TargetGroupEncodingIdentifier,
    }

    impl IPairingConsumer for Consumer {
        type Output = (TargetGroupEncodingIdentifier, CanonicalFields);

        fn consume_pairing<P: IPairingAdapter>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let description =
                build_possession_g2_statement_description(&payload.adapter, Default::default());
            let statement = build_possession_g2_statement(&payload.adapter, Default::default());
            let Ok(success) =
                description.to_fields(build_to_fields_params(Default::default()), &statement);
            (self.identifier, success.fields)
        }
    }

    let mut outputs = Vec::new();
    for concrete in PAIRING_CONCRETES {
        let selection = build_pairing_concrete(*concrete, PairingConcreteOverrides);
        let Ok(looked_up) = lookup_pairing_declaration(
            &build_lookup_pairing_declaration_deps(Default::default()),
            build_lookup_pairing_declaration_params(LookupPairingDeclarationParamsOverrides {
                concrete: Some(selection),
            }),
            build_lookup_pairing_declaration_payload(Default::default()),
        );
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(selection),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(
            Consumer {
                identifier: looked_up.declaration.target_group_encoding,
            },
            lookup_pairing_declaration,
            admit_pairing_composition,
            build_admit_pairing_composition_deps(lookup_pairing_declaration, Default::default()),
            Default::default(),
        );

        // Act
        let result = create_pairing(&deps, params, CreatePairingPayload);

        outputs.push(result.ok().map(|success| success.output));
    }

    // Assert
    assert_eq!(outputs.len(), PAIRING_CONCRETES.len());
    for left in &outputs {
        for right in &outputs {
            let (left_identifier, left_fields) = left.as_ref().expect("an output per concrete");
            let (right_identifier, right_fields) = right.as_ref().expect("an output per concrete");
            if left_identifier == right_identifier {
                assert_eq!(left_fields, right_fields);
            }
        }
    }
}

/// Contract: entry
///   `each_first_group_decoder_refuses_the_first_group_reference_encoding_where_one_exists`;
///   whenever the concrete's `g1_outside_subgroup_encoding` holds bytes,
///   `fields_to_value` over a valid key and those bytes as the commitment
///   returns `Err(PossessionStatementFromFieldsErrorReturn::InvalidG1 {
///   index: 1, error: DecodeG1ErrorReturn::NotInSubgroup })`.
/// Boundary: the public `create_pairing`; the real chain is `create_pairing`,
///   the arm's `try_new`, `consume_pairing`, `g1_outside_subgroup_encoding`,
///   the description's `try_new`, `fields_to_value`, and `decode_g1`.
/// Mocked:   none.
/// Arrange:  a `Consumer` written against `IPairingReference` calling
///   `g1_outside_subgroup_encoding`, propagating its error unchanged, and,
///   where it holds bytes, placing them at the commitment of a valid rendering
///   through `invalidate_possession_g1_statement`, calling `fields_to_value`,
///   and returning its result with the success value replaced by `()`; every
///   concrete of the declared set in turn, so a concrete with no first-group
///   encoding and a concrete with one both run; the commitment position
///   separates a decoder that reports a hard-coded index.
/// Act:      `create_pairing(&deps, params, CreatePairingPayload)` per concrete.
/// Assert:   per concrete, the output equals `Some(Ok(None))` or
///   `Some(Ok(Some(Err(PossessionStatementFromFieldsErrorReturn::InvalidG1 {
///   index: 1, error: DecodeG1ErrorReturn::NotInSubgroup }))))`; across the
///   declared set at least one output is `Some(Ok(Some(_)))`.
#[test]
fn each_first_group_decoder_refuses_the_first_group_reference_encoding_where_one_exists() {
    // Arrange
    #[derive(Default)]
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = Result<
            Option<Result<(), PossessionStatementFromFieldsErrorReturn>>,
            G1OutsideSubgroupEncodingErrorReturn,
        >;

        fn consume_pairing<P: IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let success = payload.adapter.g1_outside_subgroup_encoding(
                G1OutsideSubgroupEncodingParams,
                G1OutsideSubgroupEncodingPayload,
            )?;
            Ok(success.bytes.map(|bytes| {
                let description =
                    build_possession_g1_statement_description(&payload.adapter, Default::default());
                let fields = invalidate_possession_g1_statement(
                    &payload.adapter,
                    PossessionG1StatementCorruptions {
                        commitment: Some(CanonicalFieldValue::Bytes(bytes.as_ref().to_vec())),
                        ..Default::default()
                    },
                );
                description
                    .fields_to_value(build_from_fields_params(Default::default()), fields)
                    .map(|_| ())
            }))
        }
    }

    let mut outputs = Vec::new();
    for concrete in PAIRING_CONCRETES {
        let selection = build_pairing_concrete(*concrete, PairingConcreteOverrides);
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(selection),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(
            Consumer,
            lookup_pairing_declaration,
            admit_pairing_composition,
            build_admit_pairing_composition_deps(lookup_pairing_declaration, Default::default()),
            Default::default(),
        );

        // Act
        let result = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        let output = result.ok().map(|success| success.output);
        assert!(
            output == Some(Ok(None))
                || output
                    == Some(Ok(Some(Err(
                        PossessionStatementFromFieldsErrorReturn::InvalidG1 {
                            index: 1,
                            error: DecodeG1ErrorReturn::NotInSubgroup
                        }
                    ))))
        );
        outputs.push(output);
    }

    assert!(
        outputs
            .iter()
            .any(|output| matches!(output, Some(Ok(Some(_)))))
    );
}

/// Contract: entry `each_second_group_decoder_refuses_the_second_group_reference_encoding`;
///   the concrete's `g2_outside_subgroup_encoding` bytes as the commitment
///   after a valid key return `Err(PossessionStatementFromFieldsErrorReturn::
///   InvalidG2 { index: 1, error: DecodeG2ErrorReturn::NotInSubgroup })`.
/// Boundary: the public `create_pairing`; the real chain is `create_pairing`,
///   the arm's `try_new`, `consume_pairing`, `g2_outside_subgroup_encoding`,
///   the description's `try_new`, `fields_to_value`, and `decode_g2`.
/// Mocked:   none.
/// Arrange:  a `Consumer` written against `IPairingReference` calling
///   `g2_outside_subgroup_encoding`, propagating its error unchanged, placing
///   the bytes at the commitment of a valid rendering through
///   `invalidate_possession_g2_statement`, calling `fields_to_value`, and
///   returning its result with the success value replaced by `()`; every
///   concrete of the declared set in turn; the commitment position separates a
///   decoder that reports a hard-coded index.
/// Act:      `create_pairing(&deps, params, CreatePairingPayload)` per concrete.
/// Assert:   per concrete, the observed output is
///   `Some(Ok(Err(PossessionStatementFromFieldsErrorReturn::InvalidG2 {
///   index: 1, error: DecodeG2ErrorReturn::NotInSubgroup })))`.
#[test]
fn each_second_group_decoder_refuses_the_second_group_reference_encoding() {
    // Arrange
    #[derive(Default)]
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = Result<
            Result<(), PossessionStatementFromFieldsErrorReturn>,
            G2OutsideSubgroupEncodingErrorReturn,
        >;

        fn consume_pairing<P: IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let success = payload.adapter.g2_outside_subgroup_encoding(
                G2OutsideSubgroupEncodingParams,
                G2OutsideSubgroupEncodingPayload,
            )?;
            let description =
                build_possession_g2_statement_description(&payload.adapter, Default::default());
            let fields = invalidate_possession_g2_statement(
                &payload.adapter,
                PossessionG2StatementCorruptions {
                    commitment: Some(CanonicalFieldValue::Bytes(success.bytes.as_ref().to_vec())),
                    ..Default::default()
                },
            );
            Ok(description
                .fields_to_value(build_from_fields_params(Default::default()), fields)
                .map(|_| ()))
        }
    }

    for concrete in PAIRING_CONCRETES {
        let selection = build_pairing_concrete(*concrete, PairingConcreteOverrides);
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(selection),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(
            Consumer,
            lookup_pairing_declaration,
            admit_pairing_composition,
            build_admit_pairing_composition_deps(lookup_pairing_declaration, Default::default()),
            Default::default(),
        );

        // Act
        let result = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert_eq!(
            result.ok().map(|success| success.output),
            Some(Ok(Err(
                PossessionStatementFromFieldsErrorReturn::InvalidG2 {
                    index: 1,
                    error: DecodeG2ErrorReturn::NotInSubgroup
                }
            )))
        );
    }
}
