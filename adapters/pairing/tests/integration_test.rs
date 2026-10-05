#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use hex::decode;
use pairing::{
    AddG1Params, AddG1Payload, ConsumePairingParams, ConsumePairingPayload, CreatePairingDeps,
    CreatePairingParamsOverrides, CreatePairingPayload, DecodeG1ErrorReturn, DecodeG1Params,
    DecodeG2ErrorReturn, DecodeG2Params, DecodeScalarParams, EncodeG1Params, EncodeG1Payload,
    EncodeGtParams, EncodeGtPayload, G1GeneratorParams, G1GeneratorPayload,
    G1OutsideSubgroupEncodingParams, G1OutsideSubgroupEncodingPayload, G2GeneratorParams,
    G2GeneratorPayload, G2OutsideSubgroupEncodingParams, G2OutsideSubgroupEncodingPayload,
    IPairingAdapter, IPairingArithmetic, IPairingConsumer, IPairingReference, MulG1Params,
    MulG1Payload, MulG2Params, MulG2Payload, PairingConcrete, PairingProductIsOneParams,
    PairingProductIsOnePayload, PairingProductParams, PairingProductPayload, PairingProductTerm,
    ScalarFieldOrderParams, ScalarFieldOrderPayload, build_create_pairing_params, create_pairing,
};

struct FamilyCheckResult {
    doubling_agrees: bool,
    lone_pairing_is_one: bool,
    empty_product_is_one: bool,
}

struct FamilyCheck;

impl IPairingConsumer for FamilyCheck {
    type Output = FamilyCheckResult;

    fn consume_pairing<P: IPairingAdapter>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let adapter = payload.adapter;

        let Ok(g1) = adapter.g1_generator(G1GeneratorParams, G1GeneratorPayload);
        let Ok(g2) = adapter.g2_generator(G2GeneratorParams, G2GeneratorPayload);
        let mut two_bytes = [0u8; 32];
        two_bytes[31] = 2;
        let Ok(two) = adapter.decode_scalar(DecodeScalarParams, &two_bytes) else {
            panic!("the scalar two decodes from its big-endian bytes");
        };

        let Ok(doubled) = adapter.add_g1(
            AddG1Params,
            AddG1Payload {
                left: g1.point.clone(),
                right: g1.point.clone(),
            },
        );
        let Ok(product) = adapter.mul_g1(
            MulG1Params,
            MulG1Payload {
                point: g1.point.clone(),
                scalar: two.scalar,
            },
        );
        let Ok(sum_encoding) =
            adapter.encode_g1(EncodeG1Params, EncodeG1Payload { point: doubled.sum });
        let Ok(product_encoding) = adapter.encode_g1(
            EncodeG1Params,
            EncodeG1Payload {
                point: product.product,
            },
        );
        let doubling_agrees = sum_encoding.bytes == product_encoding.bytes;

        let Ok(lone) = adapter.pairing_product_is_one(
            PairingProductIsOneParams,
            PairingProductIsOnePayload {
                terms: vec![PairingProductTerm {
                    g1: g1.point,
                    g2: g2.point,
                }],
            },
        );
        let Ok(empty) = adapter.pairing_product_is_one(
            PairingProductIsOneParams,
            PairingProductIsOnePayload { terms: Vec::new() },
        );

        FamilyCheckResult {
            doubling_agrees,
            lone_pairing_is_one: lone.is_one,
            empty_product_is_one: empty.is_one,
        }
    }
}

/// Contract: admitted — the named concrete's declared encoding is among
///   `params.supported_encodings`, so the concrete is constructed and handed,
///   with its `DECLARATION`, to the consumer, whose output is returned in
///   `Ok(CreatePairingSuccessReturn { output })`.
/// Arrange: params naming `PairingConcrete::Bn254Arkworks` with the default
///   admitted encodings; a `FamilyCheck` consumer.
/// Act:     `create_pairing` over the deps and params.
/// Assert:  `doubling_agrees` is `true`, `lone_pairing_is_one` is `false`, and
///   `empty_product_is_one` is `true`.
/// Boundary: the crate's public surface — `create_pairing` constructs the real
///   arkworks BN254 concrete and the consumer exercises it through
///   `IPairingAdapter` alone.
/// Mocked:   nothing; the curve library is the outer edge.
#[test]
fn the_bn254_arkworks_concrete_from_the_factory_computes_through_the_family_trait() {
    // Arrange
    let deps = CreatePairingDeps {
        consumer: FamilyCheck,
    };
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bn254Arkworks),
        ..Default::default()
    });

    // Act
    let Ok(success) = create_pairing(&deps, params, CreatePairingPayload) else {
        panic!("an admitted concrete is constructed and consumed");
    };

    // Assert
    assert!(success.output.doubling_agrees);
    assert!(!success.output.lone_pairing_is_one);
    assert!(success.output.empty_product_is_one);
}

/// Contract: admitted — the named concrete's declared encoding is among
///   `params.supported_encodings`, so the concrete is constructed and handed,
///   with its `DECLARATION`, to the consumer, whose output is returned in
///   `Ok(CreatePairingSuccessReturn { output })`.
/// Arrange: params naming `PairingConcrete::Bn254Halo2curves` with the default
///   admitted encodings; a `FamilyCheck` consumer.
/// Act:     `create_pairing` over the deps and params.
/// Assert:  `doubling_agrees` is `true`, `lone_pairing_is_one` is `false`, and
///   `empty_product_is_one` is `true`.
/// Boundary: the crate's public surface — `create_pairing` constructs the real
///   halo2curves BN254 concrete and the consumer exercises it through
///   `IPairingAdapter` alone.
/// Mocked:   nothing; the curve library is the outer edge.
#[test]
fn the_bn254_halo2curves_concrete_from_the_factory_computes_through_the_family_trait() {
    // Arrange
    let deps = CreatePairingDeps {
        consumer: FamilyCheck,
    };
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bn254Halo2curves),
        ..Default::default()
    });

    // Act
    let Ok(success) = create_pairing(&deps, params, CreatePairingPayload) else {
        panic!("an admitted concrete is constructed and consumed");
    };

    // Assert
    assert!(success.output.doubling_agrees);
    assert!(!success.output.lone_pairing_is_one);
    assert!(success.output.empty_product_is_one);
}

/// Contract: admitted — the named concrete's declared encoding is among
///   `params.supported_encodings`, so the concrete is constructed and handed,
///   with its `DECLARATION`, to the consumer, whose output is returned in
///   `Ok(CreatePairingSuccessReturn { output })`.
/// Arrange: params naming `PairingConcrete::Bls12381Arkworks` with the default
///   admitted encodings; a `FamilyCheck` consumer.
/// Act:     `create_pairing` over the deps and params.
/// Assert:  `doubling_agrees` is `true`, `lone_pairing_is_one` is `false`, and
///   `empty_product_is_one` is `true`.
/// Boundary: the crate's public surface — `create_pairing` constructs the real
///   arkworks BLS12-381 concrete and the consumer exercises it through
///   `IPairingAdapter` alone.
/// Mocked:   nothing; the curve library is the outer edge.
#[test]
fn the_bls12_381_arkworks_concrete_from_the_factory_computes_through_the_family_trait() {
    // Arrange
    let deps = CreatePairingDeps {
        consumer: FamilyCheck,
    };
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bls12381Arkworks),
        ..Default::default()
    });

    // Act
    let Ok(success) = create_pairing(&deps, params, CreatePairingPayload) else {
        panic!("an admitted concrete is constructed and consumed");
    };

    // Assert
    assert!(success.output.doubling_agrees);
    assert!(!success.output.lone_pairing_is_one);
    assert!(success.output.empty_product_is_one);
}

/// Contract: admitted — the named concrete's declared encoding is among
///   `params.supported_encodings`, so the concrete is constructed and handed,
///   with its `DECLARATION`, to the consumer, whose output is returned in
///   `Ok(CreatePairingSuccessReturn { output })`.
/// Arrange: params naming `PairingConcrete::Bls12381Halo2curves` with the
///   default admitted encodings; a `FamilyCheck` consumer.
/// Act:     `create_pairing` over the deps and params.
/// Assert:  `doubling_agrees` is `true`, `lone_pairing_is_one` is `false`, and
///   `empty_product_is_one` is `true`.
/// Boundary: the crate's public surface — `create_pairing` constructs the real
///   halo2curves BLS12-381 concrete and the consumer exercises it through
///   `IPairingAdapter` alone.
/// Mocked:   nothing; the curve library is the outer edge.
#[test]
fn the_bls12_381_halo2curves_concrete_from_the_factory_computes_through_the_family_trait() {
    // Arrange
    let deps = CreatePairingDeps {
        consumer: FamilyCheck,
    };
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bls12381Halo2curves),
        ..Default::default()
    });

    // Act
    let Ok(success) = create_pairing(&deps, params, CreatePairingPayload) else {
        panic!("an admitted concrete is constructed and consumed");
    };

    // Assert
    assert!(success.output.doubling_agrees);
    assert!(!success.output.lone_pairing_is_one);
    assert!(success.output.empty_product_is_one);
}

struct TargetGroupEncoding;

impl IPairingConsumer for TargetGroupEncoding {
    type Output = Vec<u8>;

    fn consume_pairing<P: IPairingArithmetic>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let adapter = payload.adapter;

        let Ok(g1) = adapter.g1_generator(G1GeneratorParams, G1GeneratorPayload);
        let Ok(g2) = adapter.g2_generator(G2GeneratorParams, G2GeneratorPayload);
        let mut two_bytes = [0u8; 32];
        two_bytes[31] = 2;
        let mut three_bytes = [0u8; 32];
        three_bytes[31] = 3;
        let Ok(two) = adapter.decode_scalar(DecodeScalarParams, &two_bytes) else {
            panic!("the scalar two decodes from its big-endian bytes");
        };
        let Ok(three) = adapter.decode_scalar(DecodeScalarParams, &three_bytes) else {
            panic!("the scalar three decodes from its big-endian bytes");
        };
        let Ok(g1_two) = adapter.mul_g1(
            MulG1Params,
            MulG1Payload {
                point: g1.point,
                scalar: two.scalar,
            },
        );
        let Ok(g2_three) = adapter.mul_g2(
            MulG2Params,
            MulG2Payload {
                point: g2.point,
                scalar: three.scalar,
            },
        );
        let Ok(product) = adapter.pairing_product(
            PairingProductParams,
            PairingProductPayload {
                terms: vec![PairingProductTerm {
                    g1: g1_two.product,
                    g2: g2_three.product,
                }],
            },
        );
        let Ok(encoded) = adapter.encode_gt(
            EncodeGtParams,
            EncodeGtPayload {
                value: product.product,
            },
        );
        encoded.bytes.expose().as_ref().to_vec()
    }
}

/// Contract: the arkworks and halo2curves BN254 concretes, each constructed by
///   the factory and used only through the family traits, encode
///   `e(g1 · 2, g2 · 3)` to identical bytes.
/// Arrange: params naming `PairingConcrete::Bn254Arkworks` and then
///   `PairingConcrete::Bn254Halo2curves` with the default admitted encodings; a
///   `TargetGroupEncoding` consumer.
/// Act:     `create_pairing` for each concrete.
/// Assert:  the two outputs are equal, are 384 bytes, and differ from the
///   target group's identity encoding.
/// Boundary: the crate's public surface — `create_pairing` constructs the real
///   arkworks and halo2curves BN254 concretes and the consumer exercises them
///   through `IPairingArithmetic` and its supertrait.
/// Mocked:   nothing; the curve libraries are the outer edge.
#[test]
fn the_bn254_concretes_encode_the_same_pairing_to_the_same_bytes() {
    // Arrange
    let deps = CreatePairingDeps {
        consumer: TargetGroupEncoding,
    };
    let arkworks_params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bn254Arkworks),
        ..Default::default()
    });
    let halo2curves_params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bn254Halo2curves),
        ..Default::default()
    });

    // Act
    let Ok(arkworks) = create_pairing(&deps, arkworks_params, CreatePairingPayload) else {
        panic!("an admitted concrete is constructed and consumed");
    };
    let Ok(halo2curves) = create_pairing(&deps, halo2curves_params, CreatePairingPayload) else {
        panic!("an admitted concrete is constructed and consumed");
    };

    // Assert
    assert_eq!(arkworks.output, halo2curves.output);
    assert_eq!(arkworks.output.len(), 384);
    let mut identity = vec![0u8; 384];
    identity[31] = 1;
    assert_ne!(arkworks.output, identity);
}

/// Contract: the arkworks and halo2curves BLS12-381 concretes, each constructed
///   by the factory and used only through the family traits, encode
///   `e(g1 · 2, g2 · 3)` to identical bytes.
/// Arrange: params naming `PairingConcrete::Bls12381Arkworks` and then
///   `PairingConcrete::Bls12381Halo2curves` with the default admitted
///   encodings; a `TargetGroupEncoding` consumer.
/// Act:     `create_pairing` for each concrete.
/// Assert:  the two outputs are equal, are 576 bytes, and differ from the
///   target group's identity encoding.
/// Boundary: the crate's public surface — `create_pairing` constructs the real
///   arkworks and halo2curves BLS12-381 concretes and the consumer exercises
///   them through `IPairingArithmetic` and its supertrait.
/// Mocked:   nothing; the curve libraries are the outer edge.
#[test]
fn the_bls12_381_concretes_encode_the_same_pairing_to_the_same_bytes() {
    // Arrange
    let deps = CreatePairingDeps {
        consumer: TargetGroupEncoding,
    };
    let arkworks_params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bls12381Arkworks),
        ..Default::default()
    });
    let halo2curves_params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bls12381Halo2curves),
        ..Default::default()
    });

    // Act
    let Ok(arkworks) = create_pairing(&deps, arkworks_params, CreatePairingPayload) else {
        panic!("an admitted concrete is constructed and consumed");
    };
    let Ok(halo2curves) = create_pairing(&deps, halo2curves_params, CreatePairingPayload) else {
        panic!("an admitted concrete is constructed and consumed");
    };

    // Assert
    assert_eq!(arkworks.output, halo2curves.output);
    assert_eq!(arkworks.output.len(), 576);
    let mut identity = vec![0u8; 576];
    identity[47] = 1;
    assert_ne!(arkworks.output, identity);
}

struct ReferenceValuesResult {
    scalar_field_order: Vec<u8>,
    g1_outside_subgroup: Option<Vec<u8>>,
    g2_outside_subgroup: Vec<u8>,
    g1_decoder_refusal: Option<DecodeG1ErrorReturn>,
    g2_decoder_refusal: Option<DecodeG2ErrorReturn>,
}

struct ReferenceValues;

impl IPairingConsumer for ReferenceValues {
    type Output = ReferenceValuesResult;

    fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let adapter = payload.adapter;

        let Ok(order) = adapter.scalar_field_order(ScalarFieldOrderParams, ScalarFieldOrderPayload);
        let Ok(g1_encoding) = adapter.g1_outside_subgroup_encoding(
            G1OutsideSubgroupEncodingParams,
            G1OutsideSubgroupEncodingPayload,
        ) else {
            panic!("the first group's outside-the-subgroup encoding is produced");
        };
        let Ok(g2_encoding) = adapter.g2_outside_subgroup_encoding(
            G2OutsideSubgroupEncodingParams,
            G2OutsideSubgroupEncodingPayload,
        ) else {
            panic!("the second group's outside-the-subgroup encoding is produced");
        };

        let g1_decoder_refusal = match g1_encoding.bytes.as_ref() {
            None => None,
            Some(bytes) => adapter.decode_g1(DecodeG1Params, bytes.as_ref()).err(),
        };
        let g2_decoder_refusal = adapter
            .decode_g2(DecodeG2Params, g2_encoding.bytes.as_ref())
            .err();

        ReferenceValuesResult {
            scalar_field_order: order.bytes,
            g1_outside_subgroup: g1_encoding.bytes.map(|bytes| bytes.as_ref().to_vec()),
            g2_outside_subgroup: g2_encoding.bytes.as_ref().to_vec(),
            g1_decoder_refusal,
            g2_decoder_refusal,
        }
    }
}

/// Contract: the arkworks and halo2curves BN254 concretes, each constructed by
///   the factory and used only through the family traits, return BN254's group
///   order and the same outside-the-subgroup encodings, absent in the first
///   group and refused by `decode_g2` in the second.
/// Arrange: params naming `PairingConcrete::Bn254Arkworks` and then
///   `PairingConcrete::Bn254Halo2curves` with the default admitted encodings; a
///   `ReferenceValues` consumer.
/// Act:     `create_pairing` for each concrete.
/// Assert:  the `scalar_field_order` values are equal and equal the published
///   BN254 group order, both `g1_outside_subgroup` values are `None`, the
///   `g2_outside_subgroup` values are equal and 128 bytes, and
///   `g1_decoder_refusal` is `None` and `g2_decoder_refusal` is
///   `Some(DecodeG2ErrorReturn::NotInSubgroup)` for each.
/// Boundary: the crate's public surface — `create_pairing` constructs the real
///   arkworks and halo2curves BN254 concretes and the consumer exercises them
///   through `IPairingArithmetic`, `IPairingReference`, and their supertrait.
/// Mocked:   nothing; the curve libraries are the outer edge.
#[test]
fn the_bn254_concretes_return_the_same_scalar_field_order_and_outside_subgroup_encodings() {
    // Arrange
    let deps = CreatePairingDeps {
        consumer: ReferenceValues,
    };
    let arkworks_params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bn254Arkworks),
        ..Default::default()
    });
    let halo2curves_params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bn254Halo2curves),
        ..Default::default()
    });

    // Act
    let Ok(arkworks) = create_pairing(&deps, arkworks_params, CreatePairingPayload) else {
        panic!("an admitted concrete is constructed and consumed");
    };
    let Ok(halo2curves) = create_pairing(&deps, halo2curves_params, CreatePairingPayload) else {
        panic!("an admitted concrete is constructed and consumed");
    };

    // Assert
    assert_eq!(
        arkworks.output.scalar_field_order,
        halo2curves.output.scalar_field_order
    );
    let Ok(expected_order) =
        decode("30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001")
    else {
        panic!("the published BN254 group order decodes from its hex");
    };
    assert_eq!(arkworks.output.scalar_field_order, expected_order);
    assert!(arkworks.output.g1_outside_subgroup.is_none());
    assert!(halo2curves.output.g1_outside_subgroup.is_none());
    assert_eq!(
        arkworks.output.g2_outside_subgroup,
        halo2curves.output.g2_outside_subgroup
    );
    assert_eq!(arkworks.output.g2_outside_subgroup.len(), 128);
    assert_eq!(arkworks.output.g1_decoder_refusal, None);
    assert_eq!(halo2curves.output.g1_decoder_refusal, None);
    assert_eq!(
        arkworks.output.g2_decoder_refusal,
        Some(DecodeG2ErrorReturn::NotInSubgroup)
    );
    assert_eq!(
        halo2curves.output.g2_decoder_refusal,
        Some(DecodeG2ErrorReturn::NotInSubgroup)
    );
}

/// Contract: the arkworks and halo2curves BLS12-381 concretes, each constructed
///   by the factory and used only through the family traits, return
///   BLS12-381's group order and the same outside-the-subgroup encodings, each
///   refused by its group's decoder.
/// Arrange: params naming `PairingConcrete::Bls12381Arkworks` and then
///   `PairingConcrete::Bls12381Halo2curves` with the default admitted
///   encodings; a `ReferenceValues` consumer.
/// Act:     `create_pairing` for each concrete.
/// Assert:  the `scalar_field_order` values are equal and equal the published
///   BLS12-381 group order, the `g1_outside_subgroup` values are equal,
///   `Some`, and 128 bytes, the `g2_outside_subgroup` values are equal and 256
///   bytes, and `g1_decoder_refusal` is
///   `Some(DecodeG1ErrorReturn::NotInSubgroup)` and `g2_decoder_refusal` is
///   `Some(DecodeG2ErrorReturn::NotInSubgroup)` for each.
/// Boundary: the crate's public surface — `create_pairing` constructs the real
///   arkworks and halo2curves BLS12-381 concretes and the consumer exercises
///   them through `IPairingArithmetic`, `IPairingReference`, and their
///   supertrait.
/// Mocked:   nothing; the curve libraries are the outer edge.
#[test]
fn the_bls12_381_concretes_return_the_same_scalar_field_order_and_outside_subgroup_encodings() {
    // Arrange
    let deps = CreatePairingDeps {
        consumer: ReferenceValues,
    };
    let arkworks_params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bls12381Arkworks),
        ..Default::default()
    });
    let halo2curves_params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bls12381Halo2curves),
        ..Default::default()
    });

    // Act
    let Ok(arkworks) = create_pairing(&deps, arkworks_params, CreatePairingPayload) else {
        panic!("an admitted concrete is constructed and consumed");
    };
    let Ok(halo2curves) = create_pairing(&deps, halo2curves_params, CreatePairingPayload) else {
        panic!("an admitted concrete is constructed and consumed");
    };

    // Assert
    assert_eq!(
        arkworks.output.scalar_field_order,
        halo2curves.output.scalar_field_order
    );
    let Ok(expected_order) =
        decode("73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001")
    else {
        panic!("the published BLS12-381 group order decodes from its hex");
    };
    assert_eq!(arkworks.output.scalar_field_order, expected_order);
    assert_eq!(
        arkworks.output.g1_outside_subgroup,
        halo2curves.output.g1_outside_subgroup
    );
    assert_eq!(
        arkworks.output.g1_outside_subgroup.as_ref().map(Vec::len),
        Some(128)
    );
    assert_eq!(
        arkworks.output.g2_outside_subgroup,
        halo2curves.output.g2_outside_subgroup
    );
    assert_eq!(arkworks.output.g2_outside_subgroup.len(), 256);
    assert_eq!(
        arkworks.output.g1_decoder_refusal,
        Some(DecodeG1ErrorReturn::NotInSubgroup)
    );
    assert_eq!(
        halo2curves.output.g1_decoder_refusal,
        Some(DecodeG1ErrorReturn::NotInSubgroup)
    );
    assert_eq!(
        arkworks.output.g2_decoder_refusal,
        Some(DecodeG2ErrorReturn::NotInSubgroup)
    );
    assert_eq!(
        halo2curves.output.g2_decoder_refusal,
        Some(DecodeG2ErrorReturn::NotInSubgroup)
    );
}
