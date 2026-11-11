#![allow(clippy::expect_used)]

use super::create_pairing;
use super::provides::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParamsOverrides,
    CreatePairingPayload, DecodeG1ErrorReturn, DecodeG1Params, DecodeG2ErrorReturn, DecodeG2Params,
    EncodeGtParams, EncodeGtPayload, EncodeScalarParams, EncodeScalarPayload, G1GeneratorParams,
    G1GeneratorPayload, G1OutsideSubgroupEncodingErrorReturn, G1OutsideSubgroupEncodingParams,
    G1OutsideSubgroupEncodingPayload, G2GeneratorParams, G2GeneratorPayload,
    G2OutsideSubgroupEncodingErrorReturn, G2OutsideSubgroupEncodingParams,
    G2OutsideSubgroupEncodingPayload, IPairingAdapter, IPairingArithmetic, IPairingConsumer,
    IPairingReference, ISampleUniformScalar, MockIPairingAdapterFailureMode, MulG1Params,
    MulG1Payload, PAIRING_CONCRETES, PairingConcrete, PairingProductParams, PairingProductPayload,
    PairingProductTerm, SampleUniformScalarErrorReturn, SampleUniformScalarParams,
    SampleUniformScalarPayload, ScalarFieldOrderParams, ScalarFieldOrderPayload,
    TargetGroupEncodingIdentifier, build_create_pairing_deps, build_create_pairing_params,
};
use domain::{Secret, build_secret};

/// Contract: entry `the_consumer_receives_the_concrete_the_params_name`; given
///   `params.concrete` is a variant of `PAIRING_CONCRETES` and the admissions
///   pass, the consumer's reading of `P::CONCRETE` equals `params.concrete`;
///   the private case adds `PairingConcrete::Mock(Succeeds)`, whose
///   `P::CONCRETE` is `Mock(Succeeds)` and so equals `params.concrete` as it
///   does for every case.
/// Boundary: `create_pairing`, the arm's `try_new`, and `consume_pairing`.
/// Mocked:   none.
/// Arrange:  a `Consumer` written against `IPairingAdapter` alone returning
///   `P::CONCRETE`; every variant of the declared set in turn, including both
///   libraries of one curve, plus the mock concrete under `Succeeds`, so an
///   arm constructing a sibling concrete fails.
/// Act:      `create_pairing(&deps, params, CreatePairingPayload)` per case.
/// Assert:   per case, the observed output equals the case's variant.
#[test]
fn the_consumer_receives_the_concrete_the_params_name() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = PairingConcrete;

        fn consume_pairing<P: IPairingAdapter>(
            &self,
            _params: ConsumePairingParams,
            _payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            P::CONCRETE
        }
    }

    for concrete in
        PAIRING_CONCRETES
            .iter()
            .copied()
            .chain(core::iter::once(PairingConcrete::Mock(
                MockIPairingAdapterFailureMode::Succeeds,
            )))
    {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(Consumer, Default::default());

        // Act
        let result = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(result.ok().map(|success| success.output) == Some(concrete));
    }
}

/// Contract: entry `a_sampled_scalar_encodes_through_secret`; given a uniform
///   `Secret` whose last byte is `7`, the encoded scalar read through
///   `Secret::expose` is 31 zero bytes then `7`.
/// Boundary: `create_pairing`, the arm's `try_new`, `consume_pairing`,
///   `sample_from_uniform_bytes`, `encode_scalar`, and `Secret::expose`.
/// Mocked:   none; `build_secret` builds the input.
/// Arrange:  a `Consumer` written against `IPairingAdapter` sampling a uniform
///   `Secret<Vec<u8>>` of `P::Scalar::UNIFORM_BYTES_LENGTH` bytes, every byte
///   zero but the last, `7`, and encoding the sampled scalar with
///   `encode_scalar`; the nonzero last byte separates a sampled input from
///   `build_secret`'s zero default; every concrete of the declared set plus
///   the mock concrete under `Succeeds` runs.
/// Act:      `create_pairing(&deps, params, CreatePairingPayload)` per case.
/// Assert:   per case, the observed output is `Ok` holding the 32 big-endian
///   bytes of the integer seven.
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

    for concrete in
        PAIRING_CONCRETES
            .iter()
            .copied()
            .chain(core::iter::once(PairingConcrete::Mock(
                MockIPairingAdapterFailureMode::Succeeds,
            )))
    {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(Consumer, Default::default());

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
/// Boundary: `create_pairing`, the arm's `try_new`, `consume_pairing`, and
///   `sample_from_uniform_bytes`.
/// Mocked:   none; `build_secret` builds the input.
/// Arrange:  the `Consumer` of `a_sampled_scalar_encodes_through_secret` with
///   the uniform input one byte shorter than
///   `P::Scalar::UNIFORM_BYTES_LENGTH`, the last byte `7`, against the
///   full-length input of the entry it varies; every concrete of the declared
///   set plus the mock concrete under `Succeeds` runs.
/// Act:      `create_pairing(&deps, params, CreatePairingPayload)` per case.
/// Assert:   per case, the observed output is `Err(
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

    for concrete in
        PAIRING_CONCRETES
            .iter()
            .copied()
            .chain(core::iter::once(PairingConcrete::Mock(
                MockIPairingAdapterFailureMode::Succeeds,
            )))
    {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(Consumer, Default::default());

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
/// Boundary: `create_pairing`, the arm's `try_new`, `consume_pairing`,
///   `sample_from_uniform_bytes`, `g1_generator`, `g2_generator`, `mul_g1`,
///   `pairing_product`, `encode_gt`, and `Secret::expose`.
/// Mocked:   none; `build_secret` builds the input.
/// Arrange:  a `Consumer` written against `IPairingArithmetic` sampling a
///   scalar of value `7`, multiplying the first-group generator by it, taking
///   `pairing_product` over the terms of that point with the second-group
///   generator and of the first-group generator with the second-group
///   generator, and encoding the product with `encode_gt`; a nonzero scalar
///   and two terms, so the product is neither the identity nor a single
///   generator pairing; every concrete of the declared set plus the mock
///   concrete under `Succeeds` runs, collected in order, the mock's
///   `Mock`-identifier output pairing with no other.
/// Act:      `create_pairing(&deps, params, CreatePairingPayload)` per case.
/// Assert:   an `Ok` output for every case; any two outputs with equal
///   identifiers hold equal bytes.
#[test]
fn libraries_sharing_an_identifier_encode_the_pairing_product_to_the_same_bytes() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output =
            Result<(TargetGroupEncodingIdentifier, Vec<u8>), SampleUniformScalarErrorReturn>;

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
                P::DECLARATION.target_group_encoding,
                encoded.bytes.expose().as_ref().to_vec(),
            ))
        }
    }

    let mut outputs = Vec::new();
    for concrete in
        PAIRING_CONCRETES
            .iter()
            .copied()
            .chain(core::iter::once(PairingConcrete::Mock(
                MockIPairingAdapterFailureMode::Succeeds,
            )))
    {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(Consumer, Default::default());

        // Act
        let result = create_pairing(&deps, params, CreatePairingPayload);

        outputs.push(result.ok().map(|success| success.output));
    }

    // Assert
    assert_eq!(outputs.len(), PAIRING_CONCRETES.len() + 1);
    for output in &outputs {
        assert!(output.is_some());
        assert!(output.as_ref().expect("an output per concrete").is_ok());
    }
    for left in &outputs {
        for right in &outputs {
            let left_output = left.as_ref().expect("an output per concrete");
            let right_output = right.as_ref().expect("an output per concrete");
            let (left_identifier, left_bytes) = left_output.as_ref().expect("an admitted concrete");
            let (right_identifier, right_bytes) =
                right_output.as_ref().expect("an admitted concrete");
            if left_identifier == right_identifier {
                assert_eq!(left_bytes, right_bytes);
            }
        }
    }
}

/// Contract: entry
///   `libraries_sharing_an_identifier_return_the_same_scalar_field_order`; any
///   two concretes whose identifiers are equal return equal order bytes.
/// Boundary: `create_pairing`, the arm's `try_new`, `consume_pairing`, and
///   `scalar_field_order`.
/// Mocked:   none.
/// Arrange:  a `Consumer` written against `IPairingReference` calling
///   `scalar_field_order` irrefutably and pairing
///   `P::DECLARATION.target_group_encoding` with the order's bytes; every
///   concrete of the declared set, which holds each library of each
///   identifier, plus the mock concrete under `Succeeds` runs, collected in
///   order, the mock's `Mock`-identifier output pairing with no other.
/// Act:      `create_pairing(&deps, params, CreatePairingPayload)` per case.
/// Assert:   an output for every case; any two outputs with equal identifiers
///   hold equal bytes.
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
            (P::DECLARATION.target_group_encoding, order.bytes)
        }
    }

    let mut outputs = Vec::new();
    for concrete in
        PAIRING_CONCRETES
            .iter()
            .copied()
            .chain(core::iter::once(PairingConcrete::Mock(
                MockIPairingAdapterFailureMode::Succeeds,
            )))
    {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(Consumer, Default::default());

        // Act
        let result = create_pairing(&deps, params, CreatePairingPayload);

        outputs.push(result.ok().map(|success| success.output));
    }

    // Assert
    assert_eq!(outputs.len(), PAIRING_CONCRETES.len() + 1);
    for left in &outputs {
        for right in &outputs {
            let (left_identifier, left_bytes) = left.as_ref().expect("an output per concrete");
            let (right_identifier, right_bytes) = right.as_ref().expect("an output per concrete");
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
/// Boundary: `create_pairing`, the arm's `try_new`, `consume_pairing`, and
///   `g1_outside_subgroup_encoding`.
/// Mocked:   none.
/// Arrange:  a `Consumer` written against `IPairingReference` calling
///   `g1_outside_subgroup_encoding`, propagating its error unchanged, and
///   pairing `P::DECLARATION.target_group_encoding` with the optional bytes;
///   every concrete of the declared set plus the mock concrete under
///   `Succeeds` runs, collected in order, so both identifiers are exercised
///   and the mock's `Mock`-identifier output pairs with no other.
/// Act:      `create_pairing(&deps, params, CreatePairingPayload)` per case.
/// Assert:   an output for every case; every `Bn254V1` output holds `None`;
///   every `Bls12381V1` output holds `Some`; any two outputs with equal
///   identifiers hold equal optional bytes.
#[test]
fn libraries_sharing_an_identifier_return_the_same_first_group_reference_encoding() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = Result<
            (TargetGroupEncodingIdentifier, Option<Vec<u8>>),
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
            Ok((
                P::DECLARATION.target_group_encoding,
                success.bytes.map(|bytes| bytes.as_ref().to_vec()),
            ))
        }
    }

    let mut outputs = Vec::new();
    for concrete in
        PAIRING_CONCRETES
            .iter()
            .copied()
            .chain(core::iter::once(PairingConcrete::Mock(
                MockIPairingAdapterFailureMode::Succeeds,
            )))
    {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(Consumer, Default::default());

        // Act
        let result = create_pairing(&deps, params, CreatePairingPayload);

        outputs.push(result.ok().map(|success| success.output));
    }

    // Assert
    assert_eq!(outputs.len(), PAIRING_CONCRETES.len() + 1);
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
/// Boundary: `create_pairing`, the arm's `try_new`, `consume_pairing`, and
///   `g2_outside_subgroup_encoding`.
/// Mocked:   none.
/// Arrange:  a `Consumer` written against `IPairingReference` calling
///   `g2_outside_subgroup_encoding`, propagating its error unchanged, and
///   pairing `P::DECLARATION.target_group_encoding` with the bytes; every
///   concrete of the declared set plus the mock concrete under `Succeeds`
///   runs, collected in order, the mock's `Mock`-identifier output pairing
///   with no other.
/// Act:      `create_pairing(&deps, params, CreatePairingPayload)` per case.
/// Assert:   an output for every case; any two outputs with equal identifiers
///   hold equal bytes.
#[test]
fn libraries_sharing_an_identifier_return_the_same_second_group_reference_encoding() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output =
            Result<(TargetGroupEncodingIdentifier, Vec<u8>), G2OutsideSubgroupEncodingErrorReturn>;

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
                P::DECLARATION.target_group_encoding,
                success.bytes.as_ref().to_vec(),
            ))
        }
    }

    let mut outputs = Vec::new();
    for concrete in
        PAIRING_CONCRETES
            .iter()
            .copied()
            .chain(core::iter::once(PairingConcrete::Mock(
                MockIPairingAdapterFailureMode::Succeeds,
            )))
    {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(Consumer, Default::default());

        // Act
        let result = create_pairing(&deps, params, CreatePairingPayload);

        outputs.push(result.ok().map(|success| success.output));
    }

    // Assert
    assert_eq!(outputs.len(), PAIRING_CONCRETES.len() + 1);
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
/// Boundary: `create_pairing`, the arm's `try_new`, `consume_pairing`,
///   `g2_outside_subgroup_encoding`, and `decode_g2`.
/// Mocked:   none.
/// Arrange:  a `Consumer` written against `IPairingReference` calling
///   `g2_outside_subgroup_encoding`, propagating its error unchanged, passing
///   the bytes to `decode_g2`, and returning its result with the success value
///   replaced by `()`; every concrete of the declared set plus the mock
///   concrete under `Succeeds` in turn, the mock taking the entry's outcome as
///   every concrete's.
/// Act:      `create_pairing(&deps, params, CreatePairingPayload)` per case.
/// Assert:   per case, the observed output is
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

    for concrete in
        PAIRING_CONCRETES
            .iter()
            .copied()
            .chain(core::iter::once(PairingConcrete::Mock(
                MockIPairingAdapterFailureMode::Succeeds,
            )))
    {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(Consumer, Default::default());

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
/// Boundary: `create_pairing`, the arm's `try_new`, `consume_pairing`,
///   `g1_outside_subgroup_encoding`, and `decode_g1`.
/// Mocked:   none.
/// Arrange:  a `Consumer` written against `IPairingReference` calling
///   `g1_outside_subgroup_encoding`, propagating its error unchanged, passing
///   present bytes to `decode_g1`, and returning its result with the success
///   value replaced by `()`; every concrete of the declared set plus the mock
///   concrete under `Succeeds` in turn, the mock holding no first-group
///   encoding, so a concrete with none and a concrete with one both run.
/// Act:      `create_pairing(&deps, params, CreatePairingPayload)` per case.
/// Assert:   per case, the observed output matches `Some(Ok(None))` or
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
    for concrete in
        PAIRING_CONCRETES
            .iter()
            .copied()
            .chain(core::iter::once(PairingConcrete::Mock(
                MockIPairingAdapterFailureMode::Succeeds,
            )))
    {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(Consumer, Default::default());

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

/// Contract: entry `the_sample_scalar_wrong_length_mode_returns_the_sampling_refusal_whole`;
///   given the mock selection under `SampleScalarWrongLength`, the consumer's
///   sampling of a correctly-sized uniform input is refused `Err(
///   SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual: 64 })`,
///   whole.
/// Boundary: `create_pairing`, the mock arm's `try_new`, `consume_pairing`,
///   and `sample_from_uniform_bytes`, the mode reaching the sampler through
///   `MOCK_SAMPLING_MODE`.
/// Mocked:   none; `build_secret` builds the input.
/// Arrange:  a `Consumer` written against `IPairingAdapter` sampling a uniform
///   `Secret<Vec<u8>>` of `P::Scalar::UNIFORM_BYTES_LENGTH` zero bytes and
///   returning the sampling `Err` with the success erased; the selection
///   `PairingConcrete::Mock(MockIPairingAdapterFailureMode::
///   SampleScalarWrongLength)` alone, the correctly-sized input under the
///   mode being a refusal no real concrete's correctly-sized input produces.
/// Act:      `create_pairing(&deps, params, CreatePairingPayload)`.
/// Assert:   the observed output is `Err(
///   SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual: 64 })`,
///   the mode's manufactured refusal.
#[test]
fn the_sample_scalar_wrong_length_mode_returns_the_sampling_refusal_whole() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = Result<(), SampleUniformScalarErrorReturn>;

        fn consume_pairing<P: IPairingAdapter>(
            &self,
            _params: ConsumePairingParams,
            _payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let uniform: Secret<Vec<u8>> = build_secret(
                vec![0u8; P::Scalar::UNIFORM_BYTES_LENGTH],
                Default::default(),
            );
            P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload { uniform },
            )
            .map(|_| ())
        }
    }

    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Mock(
            MockIPairingAdapterFailureMode::SampleScalarWrongLength,
        )),
        ..Default::default()
    });
    let deps = build_create_pairing_deps(Consumer, Default::default());

    // Act
    let result = create_pairing(&deps, params, CreatePairingPayload);

    // Assert
    assert_eq!(
        result.ok().map(|success| success.output),
        Some(Err(SampleUniformScalarErrorReturn::WrongLength {
            expected: 64,
            actual: 64,
        }))
    );
}

/// Contract: entry `the_decode_g1_not_on_curve_mode_returns_the_curve_refusal_whole`;
///   given the mock selection under `DecodeG1NotOnCurve`, `decode_g1` of a
///   decodable encoding is refused `Err(DecodeG1ErrorReturn::NotOnCurve)`,
///   whole.
/// Boundary: `create_pairing`, the mock arm's `try_new`, `consume_pairing`,
///   and `decode_g1`.
/// Mocked:   none.
/// Arrange:  a `Consumer` written against `IPairingAdapter` calling
///   `adapter.decode_g1(DecodeG1Params, &[0, 0, 0, 0, 0, 0, 0, 1])`, the
///   eight-byte encoding of residue `1`, on the curve and in the subgroup on
///   the mock concrete, and returning the `Err` with the success erased; the
///   selection `PairingConcrete::Mock(MockIPairingAdapterFailureMode::
///   DecodeG1NotOnCurve)` alone, the mode against a decodable input.
/// Act:      `create_pairing(&deps, params, CreatePairingPayload)`.
/// Assert:   the observed output is `Err(DecodeG1ErrorReturn::NotOnCurve)`,
///   the mode's manufactured refusal.
#[test]
fn the_decode_g1_not_on_curve_mode_returns_the_curve_refusal_whole() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = Result<(), DecodeG1ErrorReturn>;

        fn consume_pairing<P: IPairingAdapter>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            payload
                .adapter
                .decode_g1(DecodeG1Params, &[0, 0, 0, 0, 0, 0, 0, 1])
                .map(|_| ())
        }
    }

    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Mock(
            MockIPairingAdapterFailureMode::DecodeG1NotOnCurve,
        )),
        ..Default::default()
    });
    let deps = build_create_pairing_deps(Consumer, Default::default());

    // Act
    let result = create_pairing(&deps, params, CreatePairingPayload);

    // Assert
    assert_eq!(
        result.ok().map(|success| success.output),
        Some(Err(DecodeG1ErrorReturn::NotOnCurve))
    );
}

/// Contract: entry `the_decode_g1_not_in_subgroup_mode_returns_the_subgroup_refusal_whole`;
///   given the mock selection under `DecodeG1NotInSubgroup`, `decode_g1` of
///   the same encoding is refused `Err(DecodeG1ErrorReturn::NotInSubgroup)`,
///   whole.
/// Boundary: `create_pairing`, the mock arm's `try_new`, `consume_pairing`,
///   and `decode_g1`.
/// Mocked:   none.
/// Arrange:  the `Consumer` of
///   `the_decode_g1_not_on_curve_mode_returns_the_curve_refusal_whole` under
///   the selection `PairingConcrete::Mock(MockIPairingAdapterFailureMode::
///   DecodeG1NotInSubgroup)` alone.
/// Act:      `create_pairing(&deps, params, CreatePairingPayload)`.
/// Assert:   the observed output is `Err(DecodeG1ErrorReturn::NotInSubgroup)`,
///   the mode's manufactured refusal.
#[test]
fn the_decode_g1_not_in_subgroup_mode_returns_the_subgroup_refusal_whole() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = Result<(), DecodeG1ErrorReturn>;

        fn consume_pairing<P: IPairingAdapter>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            payload
                .adapter
                .decode_g1(DecodeG1Params, &[0, 0, 0, 0, 0, 0, 0, 1])
                .map(|_| ())
        }
    }

    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Mock(
            MockIPairingAdapterFailureMode::DecodeG1NotInSubgroup,
        )),
        ..Default::default()
    });
    let deps = build_create_pairing_deps(Consumer, Default::default());

    // Act
    let result = create_pairing(&deps, params, CreatePairingPayload);

    // Assert
    assert_eq!(
        result.ok().map(|success| success.output),
        Some(Err(DecodeG1ErrorReturn::NotInSubgroup))
    );
}

/// Contract: entry `the_decode_g2_not_on_curve_mode_returns_the_curve_refusal_whole`;
///   given the mock selection under `DecodeG2NotOnCurve`, `decode_g2` of a
///   decodable encoding is refused `Err(DecodeG2ErrorReturn::NotOnCurve)`,
///   whole.
/// Boundary: `create_pairing`, the mock arm's `try_new`, `consume_pairing`,
///   and `decode_g2`.
/// Mocked:   none.
/// Arrange:  a `Consumer` written against `IPairingAdapter` calling
///   `adapter.decode_g2(DecodeG2Params, &[0, 0, 0, 0, 0, 0, 0, 2])`, the
///   eight-byte encoding of residue `2`, even and so inside the mock second
///   group's subgroup, and returning the `Err` with the success erased; the
///   selection `PairingConcrete::Mock(MockIPairingAdapterFailureMode::
///   DecodeG2NotOnCurve)` alone, an odd residue taking the input-reached
///   `NotInSubgroup` arm instead, so the refusal observed is the mode's.
/// Act:      `create_pairing(&deps, params, CreatePairingPayload)`.
/// Assert:   the observed output is `Err(DecodeG2ErrorReturn::NotOnCurve)`,
///   the mode's manufactured refusal.
#[test]
fn the_decode_g2_not_on_curve_mode_returns_the_curve_refusal_whole() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = Result<(), DecodeG2ErrorReturn>;

        fn consume_pairing<P: IPairingAdapter>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            payload
                .adapter
                .decode_g2(DecodeG2Params, &[0, 0, 0, 0, 0, 0, 0, 2])
                .map(|_| ())
        }
    }

    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Mock(
            MockIPairingAdapterFailureMode::DecodeG2NotOnCurve,
        )),
        ..Default::default()
    });
    let deps = build_create_pairing_deps(Consumer, Default::default());

    // Act
    let result = create_pairing(&deps, params, CreatePairingPayload);

    // Assert
    assert_eq!(
        result.ok().map(|success| success.output),
        Some(Err(DecodeG2ErrorReturn::NotOnCurve))
    );
}

/// Contract: entry `the_g1_outside_subgroup_search_exhausted_mode_returns_the_search_refusal_whole`;
///   given the mock selection under `G1OutsideSubgroupSearchExhausted`,
///   `g1_outside_subgroup_encoding` is refused `Err(
///   G1OutsideSubgroupEncodingErrorReturn::SearchExhausted)`, whole.
/// Boundary: `create_pairing`, the mock arm's `try_new`, `consume_pairing`,
///   and `g1_outside_subgroup_encoding`.
/// Mocked:   none.
/// Arrange:  a `Consumer` written against `IPairingReference` calling
///   `adapter.g1_outside_subgroup_encoding(G1OutsideSubgroupEncodingParams,
///   G1OutsideSubgroupEncodingPayload)` and returning the `Err` with the
///   success erased; the selection `PairingConcrete::Mock(
///   MockIPairingAdapterFailureMode::G1OutsideSubgroupSearchExhausted)` alone,
///   the arm no input reaches on the mock concrete, whose first group is the
///   whole curve.
/// Act:      `create_pairing(&deps, params, CreatePairingPayload)`.
/// Assert:   the observed output is `Err(
///   G1OutsideSubgroupEncodingErrorReturn::SearchExhausted)`, the mode's
///   manufactured refusal.
#[test]
fn the_g1_outside_subgroup_search_exhausted_mode_returns_the_search_refusal_whole() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = Result<(), G1OutsideSubgroupEncodingErrorReturn>;

        fn consume_pairing<P: IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            payload
                .adapter
                .g1_outside_subgroup_encoding(
                    G1OutsideSubgroupEncodingParams,
                    G1OutsideSubgroupEncodingPayload,
                )
                .map(|_| ())
        }
    }

    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Mock(
            MockIPairingAdapterFailureMode::G1OutsideSubgroupSearchExhausted,
        )),
        ..Default::default()
    });
    let deps = build_create_pairing_deps(Consumer, Default::default());

    // Act
    let result = create_pairing(&deps, params, CreatePairingPayload);

    // Assert
    assert_eq!(
        result.ok().map(|success| success.output),
        Some(Err(G1OutsideSubgroupEncodingErrorReturn::SearchExhausted))
    );
}

/// Contract: entry `the_g2_outside_subgroup_search_exhausted_mode_returns_the_search_refusal_whole`;
///   given the mock selection under `G2OutsideSubgroupSearchExhausted`,
///   `g2_outside_subgroup_encoding` is refused `Err(
///   G2OutsideSubgroupEncodingErrorReturn::SearchExhausted)`, whole.
/// Boundary: `create_pairing`, the mock arm's `try_new`, `consume_pairing`,
///   and `g2_outside_subgroup_encoding`.
/// Mocked:   none.
/// Arrange:  a `Consumer` written against `IPairingReference` calling
///   `adapter.g2_outside_subgroup_encoding(G2OutsideSubgroupEncodingParams,
///   G2OutsideSubgroupEncodingPayload)` and returning the `Err` with the
///   success erased; the selection `PairingConcrete::Mock(
///   MockIPairingAdapterFailureMode::G2OutsideSubgroupSearchExhausted)` alone.
/// Act:      `create_pairing(&deps, params, CreatePairingPayload)`.
/// Assert:   the observed output is `Err(
///   G2OutsideSubgroupEncodingErrorReturn::SearchExhausted)`, the mode's
///   manufactured refusal.
#[test]
fn the_g2_outside_subgroup_search_exhausted_mode_returns_the_search_refusal_whole() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = Result<(), G2OutsideSubgroupEncodingErrorReturn>;

        fn consume_pairing<P: IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            payload
                .adapter
                .g2_outside_subgroup_encoding(
                    G2OutsideSubgroupEncodingParams,
                    G2OutsideSubgroupEncodingPayload,
                )
                .map(|_| ())
        }
    }

    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Mock(
            MockIPairingAdapterFailureMode::G2OutsideSubgroupSearchExhausted,
        )),
        ..Default::default()
    });
    let deps = build_create_pairing_deps(Consumer, Default::default());

    // Act
    let result = create_pairing(&deps, params, CreatePairingPayload);

    // Assert
    assert_eq!(
        result.ok().map(|success| success.output),
        Some(Err(G2OutsideSubgroupEncodingErrorReturn::SearchExhausted))
    );
}
