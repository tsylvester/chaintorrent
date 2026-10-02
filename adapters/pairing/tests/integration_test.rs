#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use pairing::{
    AddG1Params, AddG1Payload, ConsumePairingParams, ConsumePairingPayload, CreatePairingDeps,
    CreatePairingParamsOverrides, CreatePairingPayload, DecodeScalarParams, EncodeG1Params,
    EncodeG1Payload, EncodeGtParams, EncodeGtPayload, G1GeneratorParams, G1GeneratorPayload,
    G2GeneratorParams, G2GeneratorPayload, IPairingAdapter, IPairingArithmetic, IPairingConsumer,
    MulG1Params, MulG1Payload, MulG2Params, MulG2Payload, PairingConcrete,
    PairingProductIsOneParams, PairingProductIsOnePayload, PairingProductParams,
    PairingProductPayload, PairingProductTerm, build_create_pairing_params, create_pairing,
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
        encoded.bytes.expose().clone()
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
