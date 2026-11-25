#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use domain::{Secret, build_secret};
use pairing::{
    AdmitPairingCompositionErrorReturn, ConsumePairingParams, ConsumePairingPayload,
    CreatePairingErrorReturn, CreatePairingParamsOverrides, CreatePairingPayload,
    DecodeG1ErrorReturn, DecodeG1Params, DecodeG2ErrorReturn, DecodeG2Params, EncodeGtParams,
    EncodeGtPayload, EncodeScalarParams, EncodeScalarPayload, G1GeneratorParams,
    G1GeneratorPayload, G1OutsideSubgroupEncodingErrorReturn, G1OutsideSubgroupEncodingParams,
    G1OutsideSubgroupEncodingPayload, G2GeneratorParams, G2GeneratorPayload,
    G2OutsideSubgroupEncodingErrorReturn, G2OutsideSubgroupEncodingParams,
    G2OutsideSubgroupEncodingPayload, IPairingAdapter, IPairingArithmetic, IPairingConsumer,
    IPairingReference, ISampleUniformScalar, LookupPairingDeclarationParamsOverrides, MulG1Params,
    MulG1Payload, PAIRING_CONCRETES, PairingConcrete, PairingDisagreement, PairingProductParams,
    PairingProductPayload, PairingProductTerm, SampleUniformScalarErrorReturn,
    SampleUniformScalarParams, SampleUniformScalarPayload, ScalarFieldOrderParams,
    ScalarFieldOrderPayload, TARGET_GROUP_ENCODING_IDENTIFIERS, TargetGroupEncodingIdentifier,
    admit_pairing_composition, build_admit_pairing_composition_deps, build_create_pairing_deps,
    build_create_pairing_params, build_lookup_pairing_declaration_deps,
    build_lookup_pairing_declaration_params, build_lookup_pairing_declaration_payload,
    create_pairing, lookup_pairing_declaration,
};

/// Contract: entry `the_consumer_receives_the_concrete_the_params_name`; given
///   `params.concrete` is a variant of `PAIRING_CONCRETES` and the composition
///   admission passes, the consumer's reading of
///   `payload.declaration.concrete` equals `params.concrete`.
/// Boundary: the public `create_pairing`; the real chain is `create_pairing`,
///   the real `admit_pairing_composition` and `lookup_pairing_declaration`,
///   the arm's `try_new`, and `consume_pairing`.
/// Mocked:   none.
/// Arrange:  a `Consumer` written against `IPairingAdapter` returning
///   `payload.declaration.concrete`; every variant of the declared set in
///   turn, including both libraries of one curve.
/// Act:      `create_pairing(&deps, params, CreatePairingPayload)` per
///   concrete.
/// Assert:   per concrete, the observed output is `Some(concrete)`, the
///   variant coming from the declared set and not from the call.
#[test]
fn the_consumer_receives_the_concrete_the_params_name() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = PairingConcrete;

        fn consume_pairing<P: IPairingAdapter>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            payload.declaration.concrete
        }
    }

    for concrete in PAIRING_CONCRETES.iter().copied() {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(concrete),
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
        assert!(result.ok().map(|success| success.output) == Some(concrete));
    }
}

/// Contract: entry `a_sampled_scalar_encodes_through_secret`; given a uniform
///   `Secret` whose last byte is `7`, the encoded scalar read through
///   `Secret::expose` is 31 zero bytes then `7`.
/// Boundary: the public `create_pairing`; the real chain is `create_pairing`,
///   the real `admit_pairing_composition` and `lookup_pairing_declaration`,
///   the arm's `try_new`, `consume_pairing`, `sample_from_uniform_bytes`,
///   `encode_scalar`, and `Secret::expose`.
/// Mocked:   none; `build_secret` builds the input.
/// Arrange:  a `Consumer` written against `IPairingAdapter` sampling a uniform
///   `Secret<Vec<u8>>` of `P::Scalar::UNIFORM_BYTES_LENGTH` bytes, every byte
///   zero but the last, `7`, and encoding the sampled scalar with
///   `encode_scalar`; the nonzero last byte separates a sampled input from
///   `build_secret`'s zero default.
/// Act:      `create_pairing(&deps, params, CreatePairingPayload)` per
///   concrete.
/// Assert:   per concrete, the observed output is `Ok` holding the 32
///   big-endian bytes of the integer seven.
#[test]
fn a_sampled_scalar_encodes_through_secret() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = Result<Vec<u8>, SampleUniformScalarErrorReturn>;

        fn consume_pairing<P: IPairingAdapter>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let mut bytes = vec![0u8; P::Scalar::UNIFORM_BYTES_LENGTH];
            bytes[P::Scalar::UNIFORM_BYTES_LENGTH - 1] = 7;
            let uniform: Secret<Vec<u8>> = build_secret(bytes, Default::default());
            let sampled = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload { uniform },
            )?;
            let scalar = sampled.scalar.expose().clone();
            let Ok(encoded) = payload
                .adapter
                .encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar });
            Ok(encoded.bytes.expose().as_ref().to_vec())
        }
    }

    let mut expected = vec![0u8; 32];
    expected[31] = 7;

    for concrete in PAIRING_CONCRETES.iter().copied() {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(concrete),
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
            Some(Ok(expected.clone()))
        );
    }
}

/// Contract: entry `a_sampling_refusal_reaches_the_caller_unchanged`; given a
///   uniform `Secret` one byte shorter than the bound, the consumer's output is
///   the sampling error, whole.
/// Boundary: the public `create_pairing`; the real chain is `create_pairing`,
///   the real `admit_pairing_composition` and `lookup_pairing_declaration`,
///   the arm's `try_new`, `consume_pairing`, and `sample_from_uniform_bytes`.
/// Mocked:   none; `build_secret` builds the input.
/// Arrange:  the `Consumer` of `a_sampled_scalar_encodes_through_secret` with
///   the uniform input one byte shorter than
///   `P::Scalar::UNIFORM_BYTES_LENGTH`, the last byte `7`, against the
///   full-length input of the entry it varies.
/// Act:      `create_pairing(&deps, params, CreatePairingPayload)` per
///   concrete.
/// Assert:   per concrete, the observed output is `Err(
///   SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual: 63 })`.
#[test]
fn a_sampling_refusal_reaches_the_caller_unchanged() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = Result<Vec<u8>, SampleUniformScalarErrorReturn>;

        fn consume_pairing<P: IPairingAdapter>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let mut bytes = vec![0u8; P::Scalar::UNIFORM_BYTES_LENGTH - 1];
            bytes[P::Scalar::UNIFORM_BYTES_LENGTH - 2] = 7;
            let uniform: Secret<Vec<u8>> = build_secret(bytes, Default::default());
            let sampled = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload { uniform },
            )?;
            let scalar = sampled.scalar.expose().clone();
            let Ok(encoded) = payload
                .adapter
                .encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar });
            Ok(encoded.bytes.expose().as_ref().to_vec())
        }
    }

    for concrete in PAIRING_CONCRETES.iter().copied() {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(concrete),
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
            Some(Err(SampleUniformScalarErrorReturn::WrongLength {
                expected: 64,
                actual: 63,
            }))
        );
    }
}

/// Contract: entry
///   `libraries_sharing_an_identifier_encode_the_pairing_product_to_the_same_bytes`;
///   given the same inputs, any two concretes whose declared target-group
///   encoding identifiers are equal return equal encoded product bytes.
/// Boundary: the public `create_pairing`; the real chain is `create_pairing`,
///   the real `admit_pairing_composition` and `lookup_pairing_declaration`,
///   the arm's `try_new`, `consume_pairing`, `sample_from_uniform_bytes`,
///   `g1_generator`, `g2_generator`, `mul_g1`, `pairing_product`, `encode_gt`,
///   and `Secret::expose`.
/// Mocked:   none; `build_secret` builds the input.
/// Arrange:  a `Consumer` written against `IPairingArithmetic` sampling a
///   scalar of value `7`, multiplying the first-group generator by it, taking
///   `pairing_product` over the terms of that point with the second-group
///   generator and of the first-group generator with the second-group
///   generator, and encoding the product with `encode_gt`; a nonzero scalar
///   and two terms, so the product is neither the identity nor a single
///   generator pairing; every concrete of the declared set runs, collected in
///   order, each output carrying `payload.declaration.target_group_encoding`
///   with the encoded `Secret`'s bytes.
/// Act:      `create_pairing(&deps, params, CreatePairingPayload)` per
///   concrete.
/// Assert:   an `Ok` output for every concrete; any two outputs with equal
///   identifiers hold equal bytes.
#[test]
fn libraries_sharing_an_identifier_encode_the_pairing_product_to_the_same_bytes() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = Result<(TargetGroupEncodingIdentifier, Vec<u8>), SampleUniformScalarErrorReturn>;

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let mut bytes = vec![0u8; P::Scalar::UNIFORM_BYTES_LENGTH];
            bytes[P::Scalar::UNIFORM_BYTES_LENGTH - 1] = 7;
            let uniform: Secret<Vec<u8>> = build_secret(bytes, Default::default());
            let sampled = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload { uniform },
            )?;
            let scalar = sampled.scalar.expose().clone();
            let adapter = payload.adapter;
            let Ok(g1_generator) = adapter.g1_generator(G1GeneratorParams, G1GeneratorPayload);
            let Ok(g2_generator) = adapter.g2_generator(G2GeneratorParams, G2GeneratorPayload);
            let Ok(product_of_generator) = adapter.mul_g1(
                MulG1Params,
                MulG1Payload {
                    point: g1_generator.point.clone(),
                    scalar,
                },
            );
            let Ok(product) = adapter.pairing_product(
                PairingProductParams,
                PairingProductPayload {
                    terms: vec![
                        PairingProductTerm {
                            g1: product_of_generator.product,
                            g2: g2_generator.point.clone(),
                        },
                        PairingProductTerm {
                            g1: g1_generator.point,
                            g2: g2_generator.point,
                        },
                    ],
                },
            );
            let Ok(encoded) = adapter.encode_gt(
                EncodeGtParams,
                EncodeGtPayload {
                    value: product.product,
                },
            );
            Ok((
                payload.declaration.target_group_encoding,
                encoded.bytes.expose().as_ref().to_vec(),
            ))
        }
    }

    let mut outputs = Vec::new();
    for concrete in PAIRING_CONCRETES.iter().copied() {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(concrete),
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

        outputs.push(result.ok().map(|success| success.output));
    }

    // Assert
    assert_eq!(outputs.len(), PAIRING_CONCRETES.len());
    for output in &outputs {
        assert!(output.is_some());
        assert!(output.as_ref().expect("an output per concrete").is_ok());
    }
    for left in &outputs {
        for right in &outputs {
            let (left_identifier, left_bytes) = left
                .as_ref()
                .expect("an output per concrete")
                .as_ref()
                .expect("an admitted concrete");
            let (right_identifier, right_bytes) = right
                .as_ref()
                .expect("an output per concrete")
                .as_ref()
                .expect("an admitted concrete");
            if left_identifier == right_identifier {
                assert_eq!(left_bytes, right_bytes);
            }
        }
    }
}

/// Contract: entry
///   `libraries_sharing_an_identifier_return_the_same_scalar_field_order`; any
///   two concretes whose identifiers are equal return equal order bytes.
/// Boundary: the public `create_pairing`; the real chain is `create_pairing`,
///   the real `admit_pairing_composition` and `lookup_pairing_declaration`,
///   the arm's `try_new`, `consume_pairing`, and `scalar_field_order`.
/// Mocked:   none.
/// Arrange:  a `Consumer` written against `IPairingReference` calling
///   `scalar_field_order` irrefutably and returning
///   `payload.declaration.target_group_encoding` with the order's bytes;
///   every concrete of the declared set, which holds each library of each
///   identifier, runs, collected in order.
/// Act:      `create_pairing(&deps, params, CreatePairingPayload)` per
///   concrete.
/// Assert:   an output for every concrete; any two outputs with equal
///   identifiers hold equal bytes.
#[test]
fn libraries_sharing_an_identifier_return_the_same_scalar_field_order() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = (TargetGroupEncodingIdentifier, Vec<u8>);

        fn consume_pairing<P: IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let Ok(order) = payload
                .adapter
                .scalar_field_order(ScalarFieldOrderParams, ScalarFieldOrderPayload);
            (payload.declaration.target_group_encoding, order.bytes)
        }
    }

    let mut outputs = Vec::new();
    for concrete in PAIRING_CONCRETES.iter().copied() {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(concrete),
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

        outputs.push(result.ok().map(|success| success.output));
    }

    // Assert
    assert_eq!(outputs.len(), PAIRING_CONCRETES.len());
    for left in &outputs {
        for right in &outputs {
            let (left_identifier, left_bytes) =
                left.as_ref().expect("an output per concrete");
            let (right_identifier, right_bytes) =
                right.as_ref().expect("an output per concrete");
            if left_identifier == right_identifier {
                assert_eq!(left_bytes, right_bytes);
            }
        }
    }
}

/// Contract: entry
///   `libraries_sharing_an_identifier_return_the_same_first_group_reference_encoding`;
///   the encoding is absent for `Bn254V1`, and present and equal across any
///   two concretes with equal identifiers for `Bls12381V1`.
/// Boundary: the public `create_pairing`; the real chain is `create_pairing`,
///   the real `admit_pairing_composition` and `lookup_pairing_declaration`,
///   the arm's `try_new`, `consume_pairing`, and
///   `g1_outside_subgroup_encoding`.
/// Mocked:   none.
/// Arrange:  a `Consumer` written against `IPairingReference` calling
///   `g1_outside_subgroup_encoding`, propagating its error unchanged, and
///   returning `payload.declaration.target_group_encoding` with the
///   optional bytes; every concrete of the declared set runs, collected in
///   order, so both identifiers are exercised.
/// Act:      `create_pairing(&deps, params, CreatePairingPayload)` per
///   concrete.
/// Assert:   an output for every concrete; every `Bn254V1` output holds
///   `None`; every `Bls12381V1` output holds `Some`; any two outputs with
///   equal identifiers hold equal optional bytes.
#[test]
fn libraries_sharing_an_identifier_return_the_same_first_group_reference_encoding() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output =
            Result<(TargetGroupEncodingIdentifier, Option<Vec<u8>>), G1OutsideSubgroupEncodingErrorReturn>;

        fn consume_pairing<P: IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let success = payload.adapter.g1_outside_subgroup_encoding(
                G1OutsideSubgroupEncodingParams,
                G1OutsideSubgroupEncodingPayload,
            )?;
            Ok((
                payload.declaration.target_group_encoding,
                success.bytes.map(|bytes| bytes.as_ref().to_vec()),
            ))
        }
    }

    let mut outputs = Vec::new();
    for concrete in PAIRING_CONCRETES.iter().copied() {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(concrete),
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

        outputs.push(result.ok().map(|success| success.output));
    }

    // Assert
    assert_eq!(outputs.len(), PAIRING_CONCRETES.len());
    for output in &outputs {
        let (identifier, bytes) = output
            .as_ref()
            .expect("an output per concrete")
            .as_ref()
            .expect("each concrete's encoding succeeds");
        if *identifier == TargetGroupEncodingIdentifier::Bn254V1 {
            assert_eq!(bytes, &None);
        }
        if *identifier == TargetGroupEncodingIdentifier::Bls12381V1 {
            assert!(bytes.is_some());
        }
    }
    for left in &outputs {
        for right in &outputs {
            let (left_identifier, left_bytes) = left
                .as_ref()
                .expect("an output per concrete")
                .as_ref()
                .expect("an admitted concrete");
            let (right_identifier, right_bytes) = right
                .as_ref()
                .expect("an output per concrete")
                .as_ref()
                .expect("an admitted concrete");
            if left_identifier == right_identifier {
                assert_eq!(left_bytes, right_bytes);
            }
        }
    }
}

/// Contract: entry
///   `libraries_sharing_an_identifier_return_the_same_second_group_reference_encoding`;
///   any two concretes whose identifiers are equal return equal bytes.
/// Boundary: the public `create_pairing`; the real chain is `create_pairing`,
///   the real `admit_pairing_composition` and `lookup_pairing_declaration`,
///   the arm's `try_new`, `consume_pairing`, and
///   `g2_outside_subgroup_encoding`.
/// Mocked:   none.
/// Arrange:  a `Consumer` written against `IPairingReference` calling
///   `g2_outside_subgroup_encoding`, propagating its error unchanged, and
///   returning `payload.declaration.target_group_encoding` with the bytes;
///   every concrete of the declared set runs, collected in order.
/// Act:      `create_pairing(&deps, params, CreatePairingPayload)` per
///   concrete.
/// Assert:   an output for every concrete; any two outputs with equal
///   identifiers hold equal bytes.
#[test]
fn libraries_sharing_an_identifier_return_the_same_second_group_reference_encoding() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = Result<(TargetGroupEncodingIdentifier, Vec<u8>), G2OutsideSubgroupEncodingErrorReturn>;

        fn consume_pairing<P: IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let success = payload.adapter.g2_outside_subgroup_encoding(
                G2OutsideSubgroupEncodingParams,
                G2OutsideSubgroupEncodingPayload,
            )?;
            Ok((
                payload.declaration.target_group_encoding,
                success.bytes.as_ref().to_vec(),
            ))
        }
    }

    let mut outputs = Vec::new();
    for concrete in PAIRING_CONCRETES.iter().copied() {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(concrete),
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

        outputs.push(result.ok().map(|success| success.output));
    }

    // Assert
    assert_eq!(outputs.len(), PAIRING_CONCRETES.len());
    for left in &outputs {
        for right in &outputs {
            let (left_identifier, left_bytes) = left
                .as_ref()
                .expect("an output per concrete")
                .as_ref()
                .expect("an admitted concrete");
            let (right_identifier, right_bytes) = right
                .as_ref()
                .expect("an output per concrete")
                .as_ref()
                .expect("an admitted concrete");
            if left_identifier == right_identifier {
                assert_eq!(left_bytes, right_bytes);
            }
        }
    }
}

/// Contract: entry
///   `each_second_group_decoder_refuses_the_second_group_reference_encoding`;
///   the concrete's `decode_g2` over its own second-group reference encoding
///   returns `Err(DecodeG2ErrorReturn::NotInSubgroup)`.
/// Boundary: the public `create_pairing`; the real chain is `create_pairing`,
///   the real `admit_pairing_composition` and `lookup_pairing_declaration`,
///   the arm's `try_new`, `consume_pairing`, `g2_outside_subgroup_encoding`,
///   and `decode_g2`.
/// Mocked:   none.
/// Arrange:  a `Consumer` written against `IPairingReference` calling
///   `g2_outside_subgroup_encoding`, propagating its error unchanged, passing
///   the bytes to `decode_g2`, and returning its result with the success
///   value replaced by `()`; every concrete of the declared set in turn.
/// Act:      `create_pairing(&deps, params, CreatePairingPayload)` per
///   concrete.
/// Assert:   per concrete, the observed output is
///   `Ok(Err(DecodeG2ErrorReturn::NotInSubgroup))`.
#[test]
fn each_second_group_decoder_refuses_the_second_group_reference_encoding() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = Result<Result<(), DecodeG2ErrorReturn>, G2OutsideSubgroupEncodingErrorReturn>;

        fn consume_pairing<P: IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let success = payload.adapter.g2_outside_subgroup_encoding(
                G2OutsideSubgroupEncodingParams,
                G2OutsideSubgroupEncodingPayload,
            )?;
            Ok(payload
                .adapter
                .decode_g2(DecodeG2Params, success.bytes.as_ref())
                .map(|_| ()))
        }
    }

    for concrete in PAIRING_CONCRETES.iter().copied() {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(concrete),
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
            Some(Ok(Err(DecodeG2ErrorReturn::NotInSubgroup)))
        );
    }
}

/// Contract: entry
///   `each_first_group_decoder_refuses_the_first_group_reference_encoding_where_one_exists`;
///   whenever the concrete's `g1_outside_subgroup_encoding` holds bytes, its
///   `decode_g1` over them returns `Err(DecodeG1ErrorReturn::NotInSubgroup)`.
/// Boundary: the public `create_pairing`; the real chain is `create_pairing`,
///   the real `admit_pairing_composition` and `lookup_pairing_declaration`,
///   the arm's `try_new`, `consume_pairing`, `g1_outside_subgroup_encoding`,
///   and `decode_g1`.
/// Mocked:   none.
/// Arrange:  a `Consumer` written against `IPairingReference` calling
///   `g1_outside_subgroup_encoding`, propagating its error unchanged, passing
///   present bytes to `decode_g1`, and returning its result with the success
///   value replaced by `()`; every concrete of the declared set in turn, so a
///   concrete with no first-group encoding and a concrete with one both run.
/// Act:      `create_pairing(&deps, params, CreatePairingPayload)` per
///   concrete.
/// Assert:   per concrete, the observed output matches `Some(Ok(None))` or
///   `Some(Ok(Some(Err(DecodeG1ErrorReturn::NotInSubgroup))))`; across the
///   declared set at least one output is `Some(Ok(Some(_)))`.
#[test]
fn each_first_group_decoder_refuses_the_first_group_reference_encoding_where_one_exists() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output =
            Result<Option<Result<(), DecodeG1ErrorReturn>>, G1OutsideSubgroupEncodingErrorReturn>;

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
                payload
                    .adapter
                    .decode_g1(DecodeG1Params, bytes.as_ref())
                    .map(|_| ())
            }))
        }
    }

    let mut outputs = Vec::new();
    for concrete in PAIRING_CONCRETES.iter().copied() {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(concrete),
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

        let output = result.ok().map(|success| success.output);
        assert!(
            matches!(output, Some(Ok(None)))
                || matches!(
                    output,
                    Some(Ok(Some(Err(DecodeG1ErrorReturn::NotInSubgroup))))
                )
        );
        outputs.push(output);
    }

    // Assert
    assert!(
        outputs
            .iter()
            .any(|output| matches!(output, Some(Ok(Some(_)))))
    );
}

/// Contract: entry
///   `a_concrete_whose_provisions_the_requirements_do_not_match_is_refused_whole_before_construction`;
///   given the four `required_*` values are every provision the declaration
///   the public `lookup_pairing_declaration` returns for a member of a
///   different curve states, the factory returns `Err(CreatePairingErrorReturn::
///   Composition(AdmitPairingCompositionErrorReturn::Incompatible { identifier,
///   disagreements }))` naming `params.concrete`, `disagreements` holding the
///   curve, verifier-group arithmetic, precompile encoding, and target-group
///   encoding disagreement in that order.
/// Boundary: the public `create_pairing`; the real chain is `create_pairing`,
///   the public `admit_pairing_composition`, the public
///   `lookup_pairing_declaration`, and the declaration constants they read.
/// Mocked:   none.
/// Arrange:  a `Consumer` written against `IPairingAdapter` with
///   `type Output = ()`; per concrete, the other-curve member chosen from
///   `PAIRING_CONCRETES` as the first member whose declared curve, read
///   through the public lookup, differs from the concrete's, the
///   requirements built from that member's declaration.
/// Act:      `create_pairing(&deps, params, CreatePairingPayload)` per
///   concrete.
/// Assert:   per concrete, the observed `Err` is `Composition(Incompatible {
///   identifier, disagreements })` with `identifier` the concrete and
///   `disagreements` the four in order, each `declared` read from the
///   concrete's declaration and each `required` from the other member's
///   declaration through the public lookup.
#[test]
fn a_concrete_whose_provisions_the_requirements_do_not_match_is_refused_whole_before_construction()
{
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingAdapter>(
            &self,
            _params: ConsumePairingParams,
            _payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
        }
    }

    for concrete in PAIRING_CONCRETES.iter().copied() {
        let Ok(declaration) = lookup_pairing_declaration(
            &build_lookup_pairing_declaration_deps(Default::default()),
            build_lookup_pairing_declaration_params(LookupPairingDeclarationParamsOverrides {
                concrete: Some(concrete),
            }),
            build_lookup_pairing_declaration_payload(Default::default()),
        );
        let other = PAIRING_CONCRETES
            .iter()
            .copied()
            .find(|other| {
                let Ok(other_declaration) = lookup_pairing_declaration(
                    &build_lookup_pairing_declaration_deps(Default::default()),
                    build_lookup_pairing_declaration_params(
                        LookupPairingDeclarationParamsOverrides {
                            concrete: Some(*other),
                        },
                    ),
                    build_lookup_pairing_declaration_payload(Default::default()),
                );
                other_declaration.declaration.curve != declaration.declaration.curve
            })
            .expect("a member of a different curve");
        let Ok(other_declaration) = lookup_pairing_declaration(
            &build_lookup_pairing_declaration_deps(Default::default()),
            build_lookup_pairing_declaration_params(LookupPairingDeclarationParamsOverrides {
                concrete: Some(other),
            }),
            build_lookup_pairing_declaration_payload(Default::default()),
        );
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(concrete),
            required_curve: Some(other_declaration.declaration.curve),
            required_verifier_group_arithmetic: Some(
                other_declaration.declaration.verifier_group_arithmetic,
            ),
            required_precompile_encodings: Some(vec![
                other_declaration.declaration.precompile_encoding,
            ]),
            required_target_group_encoding: Some(
                other_declaration.declaration.target_group_encoding,
            ),
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
            result.err(),
            Some(CreatePairingErrorReturn::Composition(
                AdmitPairingCompositionErrorReturn::Incompatible {
                    identifier: concrete,
                    disagreements: vec![
                        PairingDisagreement::Curve {
                            declared: declaration.declaration.curve,
                            required: other_declaration.declaration.curve,
                        },
                        PairingDisagreement::VerifierGroupArithmetic {
                            declared: declaration.declaration.verifier_group_arithmetic,
                            required: other_declaration.declaration.verifier_group_arithmetic,
                        },
                        PairingDisagreement::PrecompileEncoding {
                            declared: declaration.declaration.precompile_encoding,
                            required: vec![other_declaration.declaration.precompile_encoding],
                        },
                        PairingDisagreement::TargetGroupEncoding {
                            declared: declaration.declaration.target_group_encoding,
                            required: other_declaration.declaration.target_group_encoding,
                        },
                    ],
                }
            ))
        );
    }
}

/// Contract: entry `a_single_disagreeing_provision_reaches_the_caller_whole`;
///   given the four `required_*` values are the concrete's own declared
///   provisions except the target-group encoding, replaced by a value of
///   `TARGET_GROUP_ENCODING_IDENTIFIERS` the concrete does not declare, the
///   factory returns `Err(CreatePairingErrorReturn::Composition(
///   AdmitPairingCompositionErrorReturn::Incompatible { identifier,
///   disagreements }))` naming `params.concrete`, `disagreements` holding the
///   one `PairingDisagreement::TargetGroupEncoding` carrying the declared
///   value and the replaced value.
/// Boundary: the boundary of
///   `a_concrete_whose_provisions_the_requirements_do_not_match_is_refused_whole_before_construction`.
/// Mocked:   none.
/// Arrange:  the `Consumer` of
///   `a_concrete_whose_provisions_the_requirements_do_not_match_is_refused_whole_before_construction`;
///   per concrete, the requirements read from its declaration through the
///   public lookup, the target-group encoding replaced by the first value of
///   `TARGET_GROUP_ENCODING_IDENTIFIERS` unequal to the declared one.
/// Act:      `create_pairing(&deps, params, CreatePairingPayload)` per
///   concrete.
/// Assert:   per concrete, the observed `Err` is `Composition(Incompatible {
///   identifier, disagreements })` with `identifier` the concrete and
///   `disagreements` holding only `TargetGroupEncoding { declared, required }`.
#[test]
fn a_single_disagreeing_provision_reaches_the_caller_whole() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingAdapter>(
            &self,
            _params: ConsumePairingParams,
            _payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
        }
    }

    for concrete in PAIRING_CONCRETES.iter().copied() {
        let Ok(declaration) = lookup_pairing_declaration(
            &build_lookup_pairing_declaration_deps(Default::default()),
            build_lookup_pairing_declaration_params(LookupPairingDeclarationParamsOverrides {
                concrete: Some(concrete),
            }),
            build_lookup_pairing_declaration_payload(Default::default()),
        );
        let required_target_group_encoding = TARGET_GROUP_ENCODING_IDENTIFIERS
            .iter()
            .copied()
            .find(|identifier| *identifier != declaration.declaration.target_group_encoding)
            .expect("a value the declaration does not state");
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(concrete),
            required_curve: Some(declaration.declaration.curve),
            required_verifier_group_arithmetic: Some(
                declaration.declaration.verifier_group_arithmetic,
            ),
            required_precompile_encodings: Some(vec![declaration.declaration.precompile_encoding]),
            required_target_group_encoding: Some(required_target_group_encoding),
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
            result.err(),
            Some(CreatePairingErrorReturn::Composition(
                AdmitPairingCompositionErrorReturn::Incompatible {
                    identifier: concrete,
                    disagreements: vec![PairingDisagreement::TargetGroupEncoding {
                        declared: declaration.declaration.target_group_encoding,
                        required: required_target_group_encoding,
                    }],
                }
            ))
        );
    }
}
