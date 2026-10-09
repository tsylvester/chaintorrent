#![allow(clippy::expect_used)]

use super::provides::{
    Bls12381ArkworksEncodedG1, Bls12381ArkworksEncodedG2, Bls12381ArkworksEncodedGt,
    Bls12381ArkworksEncodedScalar, Bls12381ArkworksG1, Bls12381ArkworksG1Overrides,
    Bls12381ArkworksG2, Bls12381ArkworksG2Overrides, Bls12381ArkworksGtOverrides,
    Bls12381ArkworksPairing, Bls12381ArkworksPairingConstructorParams, Bls12381ArkworksScalar,
    Bls12381ArkworksScalarOverrides, CFRG_GENERATOR_PAIRING_HEX, G1_GENERATOR_HEX,
    G1_GENERATOR_OFF_CURVE_HEX, G1_NONZERO_PADDING_HEX, G1_X_AT_MODULUS_HEX, G2_GENERATOR_HEX,
    G2_GENERATOR_OFF_CURVE_HEX, G2_NONZERO_PADDING_HEX, G2_X_C0_AT_MODULUS_HEX, GROUP_ORDER_HEX,
    GROUP_ORDER_MINUS_ONE_HEX, GROUP_ORDER_MINUS_TWO_HEX, UNIFORM_FIVE_HEX,
    UNIFORM_GROUP_ORDER_HEX, base_field_modulus, build_bls12_381_arkworks_encoded_gt,
    build_bls12_381_arkworks_encoded_scalar, build_bls12_381_arkworks_g1,
    build_bls12_381_arkworks_g2, build_bls12_381_arkworks_gt, build_bls12_381_arkworks_pairing,
    build_bls12_381_arkworks_scalar, definition_value, definition_value_power,
    eip_2537_doubled_g1_generator, eip_2537_g1_generator, eip_2537_g2_generator,
    eip_2537_negated_g1_generator, g1_outside_subgroup_bytes, g1_point_from_encoding,
    g2_outside_subgroup_bytes, g2_point_from_encoding, gt_identity_encoding,
    requires_zeroize_on_drop, scalar_value, vector_bytes, zero_bytes,
};
use crate::factory::provides::{
    AddG1Params, AddG1PayloadOverrides, AddG2Params, AddG2PayloadOverrides, AddScalarParams,
    AddScalarPayloadOverrides, DecodeG1ErrorReturn, DecodeG1Params, DecodeG2ErrorReturn,
    DecodeG2Params, DecodeScalarErrorReturn, DecodeScalarParams, EncodeG1Params,
    EncodeG1PayloadOverrides, EncodeG2Params, EncodeG2PayloadOverrides, EncodeGtParams,
    EncodeGtPayloadOverrides, EncodeScalarParams, EncodeScalarPayloadOverrides, G1GeneratorParams,
    G1GeneratorPayload, G1OutsideSubgroupEncodingParams, G1OutsideSubgroupEncodingPayload,
    G2GeneratorParams, G2GeneratorPayload, G2OutsideSubgroupEncodingParams,
    G2OutsideSubgroupEncodingPayload, IPairingAdapter, IPairingArithmetic, IPairingReference,
    ISampleUniformScalar, IsIdentityG1Params, IsIdentityG1PayloadOverrides, IsIdentityG2Params,
    IsIdentityG2PayloadOverrides, MsmG1Params, MsmG1PayloadOverrides, MsmG1TermOverrides,
    MsmG2Params, MsmG2PayloadOverrides, MsmG2TermOverrides, MulG1Params, MulG1PayloadOverrides,
    MulG2Params, MulG2PayloadOverrides, MulScalarParams, MulScalarPayloadOverrides, NegG1Params,
    NegG1PayloadOverrides, NegG2Params, NegG2PayloadOverrides, NegScalarParams,
    NegScalarPayloadOverrides, PAIRING_INTERFACE_VERSION, PairingConcrete, PairingCurve,
    PairingProductIsOneParams, PairingProductIsOnePayloadOverrides, PairingProductParams,
    PairingProductPayloadOverrides, PairingProductTermOverrides, PrecompileEncoding,
    SampleUniformScalarErrorReturn, SampleUniformScalarParams, SampleUniformScalarPayloadOverrides,
    ScalarFieldOrderParams, ScalarFieldOrderPayload, TargetGroupEncodingIdentifier,
    VerifierGroupArithmetic, build_add_g1_payload, build_add_g2_payload, build_add_scalar_payload,
    build_encode_g1_payload, build_encode_g2_payload, build_encode_gt_payload,
    build_encode_scalar_payload, build_is_identity_g1_payload, build_is_identity_g2_payload,
    build_msm_g1_payload, build_msm_g1_term, build_msm_g2_payload, build_msm_g2_term,
    build_mul_g1_payload, build_mul_g2_payload, build_mul_scalar_payload, build_neg_g1_payload,
    build_neg_g2_payload, build_neg_scalar_payload, build_pairing_product_is_one_payload,
    build_pairing_product_payload, build_pairing_product_term, build_sample_uniform_scalar_payload,
};
use ark_bls12_381::{Fq, Fq2, Fq12, Fr, G1Affine, G2Affine};
use ark_ec::{AffineRepr, pairing::PairingOutput};
use ark_ff::{Field, PrimeField, Zero};
use domain::{SecretConstructorParamsOverrides, build_secret};
use num_bigint::BigUint;
use zeroize::Zeroize;

const _: () = assert!(
    matches!(
        Bls12381ArkworksPairing::DECLARATION.curve,
        PairingCurve::Bls12381
    ) && matches!(
        Bls12381ArkworksPairing::DECLARATION.verifier_group_arithmetic,
        VerifierGroupArithmetic::BothGroups
    ) && matches!(
        Bls12381ArkworksPairing::DECLARATION.precompile_encoding,
        PrecompileEncoding::Eip2537
    ) && matches!(
        Bls12381ArkworksPairing::DECLARATION.target_group_encoding,
        TargetGroupEncodingIdentifier::Bls12381V1
    ) && Bls12381ArkworksPairing::DECLARATION.adapter_version == 1
        && Bls12381ArkworksPairing::DECLARATION.interface_version == PAIRING_INTERFACE_VERSION
);

const _: () = assert!(
    matches!(
        <Bls12381ArkworksPairing as IPairingAdapter>::DECLARATION.curve,
        PairingCurve::Bls12381
    ) && matches!(
        <Bls12381ArkworksPairing as IPairingAdapter>::DECLARATION.verifier_group_arithmetic,
        VerifierGroupArithmetic::BothGroups
    ) && matches!(
        <Bls12381ArkworksPairing as IPairingAdapter>::DECLARATION.precompile_encoding,
        PrecompileEncoding::Eip2537
    ) && matches!(
        <Bls12381ArkworksPairing as IPairingAdapter>::DECLARATION.target_group_encoding,
        TargetGroupEncodingIdentifier::Bls12381V1
    ) && <Bls12381ArkworksPairing as IPairingAdapter>::DECLARATION.adapter_version == 1
        && <Bls12381ArkworksPairing as IPairingAdapter>::DECLARATION.interface_version
            == PAIRING_INTERFACE_VERSION
);

const _: () = assert!(matches!(
    <Bls12381ArkworksPairing as IPairingAdapter>::CONCRETE,
    PairingConcrete::Bls12381Arkworks
));

#[test]
fn bls12_381_arkworks_scalar_is_zeroize_on_drop() {
    requires_zeroize_on_drop::<Bls12381ArkworksScalar>();
}

#[test]
fn bls12_381_arkworks_scalar_zeroize_clears_its_value() {
    let mut scalar = build_bls12_381_arkworks_scalar(Bls12381ArkworksScalarOverrides {
        value: Some(Fr::from(5u64)),
    });
    scalar.zeroize();
    assert_eq!(scalar.value, Fr::from(0u64));
}

#[test]
fn bls12_381_arkworks_g1_zeroize_clears_its_value() {
    let mut g1 = build_bls12_381_arkworks_g1(Default::default());
    g1.zeroize();
    assert_eq!(
        g1.value,
        G1Affine::new_unchecked(Fq::from(0u64), Fq::from(0u64))
    );
}

#[test]
fn bls12_381_arkworks_g2_zeroize_clears_its_value() {
    let mut g2 = build_bls12_381_arkworks_g2(Default::default());
    g2.zeroize();
    assert_eq!(
        g2.value,
        G2Affine::new_unchecked(
            Fq2::new(Fq::from(0u64), Fq::from(0u64)),
            Fq2::new(Fq::from(0u64), Fq::from(0u64))
        )
    );
}

#[test]
fn bls12_381_arkworks_gt_zeroize_clears_its_value() {
    let mut gt = build_bls12_381_arkworks_gt(Default::default());
    gt.zeroize();
    assert!(gt.value.0.is_zero());
}

#[test]
fn bls12_381_arkworks_encoded_scalar_zeroize_clears_its_bytes() {
    let mut encoded = build_bls12_381_arkworks_encoded_scalar(Default::default());
    encoded.zeroize();
    assert_eq!(encoded.as_ref(), zero_bytes(32).as_slice());
}

#[test]
fn bls12_381_arkworks_encoded_gt_zeroize_clears_its_bytes() {
    let mut encoded = build_bls12_381_arkworks_encoded_gt(Default::default());
    encoded.zeroize();
    assert_eq!(encoded.as_ref(), zero_bytes(576).as_slice());
}

#[test]
fn reduced_pairing_correction_inverts_three() {
    let Ok(pairing) = Bls12381ArkworksPairing::try_new(Bls12381ArkworksPairingConstructorParams);
    assert_eq!(
        pairing.reduced_pairing_correction * Fr::from(3u64),
        Fr::from(1u64)
    );
}

#[test]
fn uniform_bytes_length_is_twice_the_group_order_width() {
    assert_eq!(
        <Bls12381ArkworksScalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
        64
    );
}

#[test]
fn g1_generator_returns_the_eip_2537_generator() {
    let pairing = build_bls12_381_arkworks_pairing();
    let Ok(success) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
    assert_eq!(success.point.value, eip_2537_g1_generator());
}

#[test]
fn g2_generator_returns_the_eip_2537_generator() {
    let pairing = build_bls12_381_arkworks_pairing();
    let Ok(success) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
    assert_eq!(success.point.value, eip_2537_g2_generator());
}

#[test]
fn add_g1_of_a_point_and_its_negation_is_the_identity() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = build_add_g1_payload(AddG1PayloadOverrides {
        left: Some(build_bls12_381_arkworks_g1(Default::default())),
        right: Some(build_bls12_381_arkworks_g1(Bls12381ArkworksG1Overrides {
            value: Some(eip_2537_negated_g1_generator()),
        })),
    });
    let Ok(success) = pairing.add_g1(AddG1Params, payload);
    assert_eq!(success.sum.value, G1Affine::identity());
}

#[test]
fn add_g1_of_the_identity_and_a_point_is_the_point() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = build_add_g1_payload(AddG1PayloadOverrides {
        left: Some(build_bls12_381_arkworks_g1(Bls12381ArkworksG1Overrides {
            value: Some(G1Affine::identity()),
        })),
        right: Some(build_bls12_381_arkworks_g1(Default::default())),
    });
    let Ok(success) = pairing.add_g1(AddG1Params, payload);
    assert_eq!(success.sum.value, eip_2537_g1_generator());
}

#[test]
fn add_g2_of_a_point_and_its_negation_is_the_identity() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = build_add_g2_payload(AddG2PayloadOverrides {
        left: Some(build_bls12_381_arkworks_g2(Default::default())),
        right: Some(build_bls12_381_arkworks_g2(Bls12381ArkworksG2Overrides {
            value: Some(-eip_2537_g2_generator()),
        })),
    });
    let Ok(success) = pairing.add_g2(AddG2Params, payload);
    assert_eq!(success.sum.value, G2Affine::identity());
}

#[test]
fn add_g2_of_the_identity_and_a_point_is_the_point() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = build_add_g2_payload(AddG2PayloadOverrides {
        left: Some(build_bls12_381_arkworks_g2(Bls12381ArkworksG2Overrides {
            value: Some(G2Affine::identity()),
        })),
        right: Some(build_bls12_381_arkworks_g2(Default::default())),
    });
    let Ok(success) = pairing.add_g2(AddG2Params, payload);
    assert_eq!(success.sum.value, eip_2537_g2_generator());
}

#[test]
fn mul_g1_by_one_is_the_point() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = build_mul_g1_payload(MulG1PayloadOverrides {
        point: Some(build_bls12_381_arkworks_g1(Default::default())),
        scalar: Some(build_bls12_381_arkworks_scalar(
            Bls12381ArkworksScalarOverrides {
                value: Some(Fr::from(1u64)),
            },
        )),
    });
    let Ok(success) = pairing.mul_g1(MulG1Params, payload);
    assert_eq!(success.product.value, eip_2537_g1_generator());
}

#[test]
fn mul_g1_by_zero_is_the_identity() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = build_mul_g1_payload(MulG1PayloadOverrides {
        point: Some(build_bls12_381_arkworks_g1(Default::default())),
        scalar: Some(build_bls12_381_arkworks_scalar(
            Bls12381ArkworksScalarOverrides {
                value: Some(Fr::from(0u64)),
            },
        )),
    });
    let Ok(success) = pairing.mul_g1(MulG1Params, payload);
    assert_eq!(success.product.value, G1Affine::identity());
}

#[test]
fn mul_g1_by_the_group_order_minus_one_is_the_negated_generator() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = build_mul_g1_payload(MulG1PayloadOverrides {
        point: Some(build_bls12_381_arkworks_g1(Default::default())),
        scalar: Some(build_bls12_381_arkworks_scalar(
            Bls12381ArkworksScalarOverrides {
                value: Some(scalar_value(GROUP_ORDER_MINUS_ONE_HEX)),
            },
        )),
    });
    let Ok(success) = pairing.mul_g1(MulG1Params, payload);
    assert_eq!(success.product.value, eip_2537_negated_g1_generator());
}

#[test]
fn mul_g2_by_one_is_the_point() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = build_mul_g2_payload(MulG2PayloadOverrides {
        point: Some(build_bls12_381_arkworks_g2(Default::default())),
        scalar: Some(build_bls12_381_arkworks_scalar(
            Bls12381ArkworksScalarOverrides {
                value: Some(Fr::from(1u64)),
            },
        )),
    });
    let Ok(success) = pairing.mul_g2(MulG2Params, payload);
    assert_eq!(success.product.value, eip_2537_g2_generator());
}

#[test]
fn mul_g2_by_zero_is_the_identity() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = build_mul_g2_payload(MulG2PayloadOverrides {
        point: Some(build_bls12_381_arkworks_g2(Default::default())),
        scalar: Some(build_bls12_381_arkworks_scalar(
            Bls12381ArkworksScalarOverrides {
                value: Some(Fr::from(0u64)),
            },
        )),
    });
    let Ok(success) = pairing.mul_g2(MulG2Params, payload);
    assert_eq!(success.product.value, G2Affine::identity());
}

#[test]
fn mul_g2_by_the_group_order_minus_one_is_the_negated_point() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = build_mul_g2_payload(MulG2PayloadOverrides {
        point: Some(build_bls12_381_arkworks_g2(Default::default())),
        scalar: Some(build_bls12_381_arkworks_scalar(
            Bls12381ArkworksScalarOverrides {
                value: Some(scalar_value(GROUP_ORDER_MINUS_ONE_HEX)),
            },
        )),
    });
    let Ok(success) = pairing.mul_g2(MulG2Params, payload);
    assert_eq!(success.product.value, -eip_2537_g2_generator());
}

#[test]
fn msm_g1_of_no_terms_is_the_identity() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload =
        build_msm_g1_payload::<Bls12381ArkworksG1, Bls12381ArkworksScalar>(Default::default());
    let Ok(success) = pairing.msm_g1(MsmG1Params, payload);
    assert_eq!(success.sum.value, G1Affine::identity());
}

#[test]
fn msm_g1_pairs_each_base_with_its_own_scalar() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload =
        build_msm_g1_payload::<Bls12381ArkworksG1, Bls12381ArkworksScalar>(MsmG1PayloadOverrides {
            terms: Some(vec![
                build_msm_g1_term(MsmG1TermOverrides {
                    base: Some(build_bls12_381_arkworks_g1(Default::default())),
                    scalar: Some(build_bls12_381_arkworks_scalar(
                        Bls12381ArkworksScalarOverrides {
                            value: Some(Fr::from(1u64)),
                        },
                    )),
                }),
                build_msm_g1_term(MsmG1TermOverrides {
                    base: Some(build_bls12_381_arkworks_g1(Bls12381ArkworksG1Overrides {
                        value: Some(eip_2537_negated_g1_generator()),
                    })),
                    scalar: Some(build_bls12_381_arkworks_scalar(
                        Bls12381ArkworksScalarOverrides {
                            value: Some(Fr::from(0u64)),
                        },
                    )),
                }),
            ]),
        });
    let Ok(success) = pairing.msm_g1(MsmG1Params, payload);
    assert_eq!(success.sum.value, eip_2537_g1_generator());
}

#[test]
fn msm_g1_sums_its_terms() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload =
        build_msm_g1_payload::<Bls12381ArkworksG1, Bls12381ArkworksScalar>(MsmG1PayloadOverrides {
            terms: Some(vec![
                build_msm_g1_term(MsmG1TermOverrides {
                    base: Some(build_bls12_381_arkworks_g1(Default::default())),
                    scalar: Some(build_bls12_381_arkworks_scalar(
                        Bls12381ArkworksScalarOverrides {
                            value: Some(Fr::from(1u64)),
                        },
                    )),
                }),
                build_msm_g1_term(MsmG1TermOverrides {
                    base: Some(build_bls12_381_arkworks_g1(Bls12381ArkworksG1Overrides {
                        value: Some(eip_2537_negated_g1_generator()),
                    })),
                    scalar: Some(build_bls12_381_arkworks_scalar(
                        Bls12381ArkworksScalarOverrides {
                            value: Some(Fr::from(1u64)),
                        },
                    )),
                }),
            ]),
        });
    let Ok(success) = pairing.msm_g1(MsmG1Params, payload);
    assert_eq!(success.sum.value, G1Affine::identity());
}

#[test]
fn msm_g2_of_no_terms_is_the_identity() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload =
        build_msm_g2_payload::<Bls12381ArkworksG2, Bls12381ArkworksScalar>(Default::default());
    let Ok(success) = pairing.msm_g2(MsmG2Params, payload);
    assert_eq!(success.sum.value, G2Affine::identity());
}

#[test]
fn msm_g2_pairs_each_base_with_its_own_scalar() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload =
        build_msm_g2_payload::<Bls12381ArkworksG2, Bls12381ArkworksScalar>(MsmG2PayloadOverrides {
            terms: Some(vec![
                build_msm_g2_term(MsmG2TermOverrides {
                    base: Some(build_bls12_381_arkworks_g2(Default::default())),
                    scalar: Some(build_bls12_381_arkworks_scalar(
                        Bls12381ArkworksScalarOverrides {
                            value: Some(Fr::from(1u64)),
                        },
                    )),
                }),
                build_msm_g2_term(MsmG2TermOverrides {
                    base: Some(build_bls12_381_arkworks_g2(Bls12381ArkworksG2Overrides {
                        value: Some(-eip_2537_g2_generator()),
                    })),
                    scalar: Some(build_bls12_381_arkworks_scalar(
                        Bls12381ArkworksScalarOverrides {
                            value: Some(Fr::from(0u64)),
                        },
                    )),
                }),
            ]),
        });
    let Ok(success) = pairing.msm_g2(MsmG2Params, payload);
    assert_eq!(success.sum.value, eip_2537_g2_generator());
}

#[test]
fn msm_g2_sums_its_terms() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload =
        build_msm_g2_payload::<Bls12381ArkworksG2, Bls12381ArkworksScalar>(MsmG2PayloadOverrides {
            terms: Some(vec![
                build_msm_g2_term(MsmG2TermOverrides {
                    base: Some(build_bls12_381_arkworks_g2(Default::default())),
                    scalar: Some(build_bls12_381_arkworks_scalar(
                        Bls12381ArkworksScalarOverrides {
                            value: Some(Fr::from(1u64)),
                        },
                    )),
                }),
                build_msm_g2_term(MsmG2TermOverrides {
                    base: Some(build_bls12_381_arkworks_g2(Bls12381ArkworksG2Overrides {
                        value: Some(-eip_2537_g2_generator()),
                    })),
                    scalar: Some(build_bls12_381_arkworks_scalar(
                        Bls12381ArkworksScalarOverrides {
                            value: Some(Fr::from(1u64)),
                        },
                    )),
                }),
            ]),
        });
    let Ok(success) = pairing.msm_g2(MsmG2Params, payload);
    assert_eq!(success.sum.value, G2Affine::identity());
}

#[test]
fn pairing_product_is_one_of_no_terms_is_true() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = build_pairing_product_is_one_payload::<Bls12381ArkworksG1, Bls12381ArkworksG2>(
        Default::default(),
    );
    let Ok(success) = pairing.pairing_product_is_one(PairingProductIsOneParams, payload);
    assert!(success.is_one);
}

#[test]
fn pairing_product_is_one_of_a_pairing_and_its_first_group_negation_is_true() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = build_pairing_product_is_one_payload::<Bls12381ArkworksG1, Bls12381ArkworksG2>(
        PairingProductIsOnePayloadOverrides {
            terms: Some(vec![
                build_pairing_product_term(Default::default()),
                build_pairing_product_term(PairingProductTermOverrides {
                    g1: Some(build_bls12_381_arkworks_g1(Bls12381ArkworksG1Overrides {
                        value: Some(eip_2537_negated_g1_generator()),
                    })),
                    g2: Some(build_bls12_381_arkworks_g2(Default::default())),
                }),
            ]),
        },
    );
    let Ok(success) = pairing.pairing_product_is_one(PairingProductIsOneParams, payload);
    assert!(success.is_one);
}

#[test]
fn pairing_product_is_one_of_the_generators_is_false() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = build_pairing_product_is_one_payload::<Bls12381ArkworksG1, Bls12381ArkworksG2>(
        PairingProductIsOnePayloadOverrides {
            terms: Some(vec![build_pairing_product_term(Default::default())]),
        },
    );
    let Ok(success) = pairing.pairing_product_is_one(PairingProductIsOneParams, payload);
    assert!(!success.is_one);
}

#[test]
fn decode_g1_rejects_a_wrong_length() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = zero_bytes(127);
    let result = pairing.decode_g1(DecodeG1Params, &payload);
    assert_eq!(
        result.err(),
        Some(DecodeG1ErrorReturn::WrongLength {
            expected: 128,
            actual: 127
        })
    );
}

#[test]
fn decode_g1_rejects_a_nonzero_padding_byte() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = vector_bytes(G1_NONZERO_PADDING_HEX);
    let result = pairing.decode_g1(DecodeG1Params, &payload);
    assert_eq!(
        result.err(),
        Some(DecodeG1ErrorReturn::NonCanonicalCoordinate)
    );
}

#[test]
fn decode_g1_rejects_a_non_canonical_coordinate() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = vector_bytes(G1_X_AT_MODULUS_HEX);
    let result = pairing.decode_g1(DecodeG1Params, &payload);
    assert_eq!(
        result.err(),
        Some(DecodeG1ErrorReturn::NonCanonicalCoordinate)
    );
}

#[test]
fn decode_g1_decodes_the_identity_from_zero_bytes() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = zero_bytes(128);
    let success = pairing
        .decode_g1(DecodeG1Params, &payload)
        .expect("zero bytes decode to the identity");
    assert_eq!(success.point.value, G1Affine::identity());
}

#[test]
fn decode_g1_rejects_a_point_off_the_curve() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = vector_bytes(G1_GENERATOR_OFF_CURVE_HEX);
    let result = pairing.decode_g1(DecodeG1Params, &payload);
    assert_eq!(result.err(), Some(DecodeG1ErrorReturn::NotOnCurve));
}

#[test]
fn decode_g1_rejects_a_point_outside_the_subgroup() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = g1_outside_subgroup_bytes();
    let result = pairing.decode_g1(DecodeG1Params, &payload);
    assert_eq!(result.err(), Some(DecodeG1ErrorReturn::NotInSubgroup));
}

#[test]
fn decode_g1_decodes_the_eip_2537_generator() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = vector_bytes(G1_GENERATOR_HEX);
    let success = pairing
        .decode_g1(DecodeG1Params, &payload)
        .expect("the generator decodes");
    assert_eq!(success.point.value, eip_2537_g1_generator());
}

#[test]
fn decode_g2_rejects_a_wrong_length() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = zero_bytes(255);
    let result = pairing.decode_g2(DecodeG2Params, &payload);
    assert_eq!(
        result.err(),
        Some(DecodeG2ErrorReturn::WrongLength {
            expected: 256,
            actual: 255
        })
    );
}

#[test]
fn decode_g2_rejects_a_nonzero_padding_byte() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = vector_bytes(G2_NONZERO_PADDING_HEX);
    let result = pairing.decode_g2(DecodeG2Params, &payload);
    assert_eq!(
        result.err(),
        Some(DecodeG2ErrorReturn::NonCanonicalCoordinate)
    );
}

#[test]
fn decode_g2_rejects_a_non_canonical_coordinate() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = vector_bytes(G2_X_C0_AT_MODULUS_HEX);
    let result = pairing.decode_g2(DecodeG2Params, &payload);
    assert_eq!(
        result.err(),
        Some(DecodeG2ErrorReturn::NonCanonicalCoordinate)
    );
}

#[test]
fn decode_g2_decodes_the_identity_from_zero_bytes() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = zero_bytes(256);
    let success = pairing
        .decode_g2(DecodeG2Params, &payload)
        .expect("zero bytes decode to the identity");
    assert_eq!(success.point.value, G2Affine::identity());
}

#[test]
fn decode_g2_rejects_a_point_off_the_curve() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = vector_bytes(G2_GENERATOR_OFF_CURVE_HEX);
    let result = pairing.decode_g2(DecodeG2Params, &payload);
    assert_eq!(result.err(), Some(DecodeG2ErrorReturn::NotOnCurve));
}

#[test]
fn decode_g2_rejects_a_point_outside_the_subgroup() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = g2_outside_subgroup_bytes();
    let result = pairing.decode_g2(DecodeG2Params, &payload);
    assert_eq!(result.err(), Some(DecodeG2ErrorReturn::NotInSubgroup));
}

#[test]
fn decode_g2_decodes_the_eip_2537_generator() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = vector_bytes(G2_GENERATOR_HEX);
    let success = pairing
        .decode_g2(DecodeG2Params, &payload)
        .expect("the generator decodes");
    assert_eq!(success.point.value, eip_2537_g2_generator());
}

#[test]
fn decode_scalar_rejects_a_wrong_length() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = zero_bytes(31);
    let result = pairing.decode_scalar(DecodeScalarParams, &payload);
    assert_eq!(
        result.err(),
        Some(DecodeScalarErrorReturn::WrongLength {
            expected: 32,
            actual: 31
        })
    );
}

#[test]
fn decode_scalar_rejects_the_group_order() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = vector_bytes(GROUP_ORDER_HEX);
    let result = pairing.decode_scalar(DecodeScalarParams, &payload);
    assert_eq!(result.err(), Some(DecodeScalarErrorReturn::NonCanonical));
}

#[test]
fn decode_scalar_decodes_the_largest_canonical_scalar() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = vector_bytes(GROUP_ORDER_MINUS_ONE_HEX);
    let success = pairing
        .decode_scalar(DecodeScalarParams, &payload)
        .expect("the largest canonical scalar decodes");
    assert_eq!(success.scalar.value, -Fr::from(1u64));
}

#[test]
fn encode_g1_writes_the_eip_2537_generator() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = build_encode_g1_payload(EncodeG1PayloadOverrides {
        point: Some(build_bls12_381_arkworks_g1(Default::default())),
    });
    let Ok(success) = pairing.encode_g1(EncodeG1Params, payload);
    let bytes: Bls12381ArkworksEncodedG1 = success.bytes;
    assert_eq!(bytes.as_ref(), vector_bytes(G1_GENERATOR_HEX).as_slice());
}

#[test]
fn encode_g1_writes_the_identity_as_zero_bytes() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = build_encode_g1_payload(EncodeG1PayloadOverrides {
        point: Some(build_bls12_381_arkworks_g1(Bls12381ArkworksG1Overrides {
            value: Some(G1Affine::identity()),
        })),
    });
    let Ok(success) = pairing.encode_g1(EncodeG1Params, payload);
    assert_eq!(success.bytes.as_ref(), zero_bytes(128).as_slice());
}

#[test]
fn encode_g2_writes_the_eip_2537_generator() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = build_encode_g2_payload(EncodeG2PayloadOverrides {
        point: Some(build_bls12_381_arkworks_g2(Default::default())),
    });
    let Ok(success) = pairing.encode_g2(EncodeG2Params, payload);
    let bytes: Bls12381ArkworksEncodedG2 = success.bytes;
    assert_eq!(bytes.as_ref(), vector_bytes(G2_GENERATOR_HEX).as_slice());
}

#[test]
fn encode_g2_writes_the_identity_as_zero_bytes() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = build_encode_g2_payload(EncodeG2PayloadOverrides {
        point: Some(build_bls12_381_arkworks_g2(Bls12381ArkworksG2Overrides {
            value: Some(G2Affine::identity()),
        })),
    });
    let Ok(success) = pairing.encode_g2(EncodeG2Params, payload);
    assert_eq!(success.bytes.as_ref(), zero_bytes(256).as_slice());
}

#[test]
fn encode_scalar_writes_the_largest_canonical_scalar() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = build_encode_scalar_payload(EncodeScalarPayloadOverrides {
        scalar: Some(build_bls12_381_arkworks_scalar(
            Bls12381ArkworksScalarOverrides {
                value: Some(-Fr::from(1u64)),
            },
        )),
    });
    let Ok(success) = pairing.encode_scalar(EncodeScalarParams, payload);
    let bytes: &Bls12381ArkworksEncodedScalar = success.bytes.expose();
    assert_eq!(
        bytes.as_ref(),
        vector_bytes(GROUP_ORDER_MINUS_ONE_HEX).as_slice()
    );
}

#[test]
fn sample_from_uniform_bytes_rejects_a_wrong_length() {
    let payload = build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
        uniform: Some(build_secret(SecretConstructorParamsOverrides {
            value: Some(zero_bytes(63)),
        })),
    });
    let result =
        Bls12381ArkworksScalar::sample_from_uniform_bytes(SampleUniformScalarParams, payload);
    assert_eq!(
        result.err(),
        Some(SampleUniformScalarErrorReturn::WrongLength {
            expected: 64,
            actual: 63
        })
    );
}

#[test]
fn sample_from_uniform_bytes_reads_the_input_as_a_big_endian_integer() {
    let payload = build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
        uniform: Some(build_secret(SecretConstructorParamsOverrides {
            value: Some(vector_bytes(UNIFORM_FIVE_HEX)),
        })),
    });
    let success =
        Bls12381ArkworksScalar::sample_from_uniform_bytes(SampleUniformScalarParams, payload)
            .expect("the uniform input samples");
    assert_eq!(success.scalar.expose().value, Fr::from(5u64));
}

#[test]
fn sample_from_uniform_bytes_reduces_the_group_order_to_zero() {
    let payload = build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
        uniform: Some(build_secret(SecretConstructorParamsOverrides {
            value: Some(vector_bytes(UNIFORM_GROUP_ORDER_HEX)),
        })),
    });
    let success =
        Bls12381ArkworksScalar::sample_from_uniform_bytes(SampleUniformScalarParams, payload)
            .expect("the uniform input samples");
    assert_eq!(success.scalar.expose().value, Fr::from(0u64));
}

#[test]
fn add_scalar_of_two_and_three_is_five() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = build_add_scalar_payload(AddScalarPayloadOverrides {
        left: Some(build_bls12_381_arkworks_scalar(
            Bls12381ArkworksScalarOverrides {
                value: Some(Fr::from(2u64)),
            },
        )),
        right: Some(build_bls12_381_arkworks_scalar(
            Bls12381ArkworksScalarOverrides {
                value: Some(Fr::from(3u64)),
            },
        )),
    });
    let Ok(success) = pairing.add_scalar(AddScalarParams, payload);
    assert_eq!(success.sum.value, Fr::from(5u64));
}

#[test]
fn add_scalar_reduces_modulo_the_group_order() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = build_add_scalar_payload(AddScalarPayloadOverrides {
        left: Some(build_bls12_381_arkworks_scalar(
            Bls12381ArkworksScalarOverrides {
                value: Some(scalar_value(GROUP_ORDER_MINUS_ONE_HEX)),
            },
        )),
        right: Some(build_bls12_381_arkworks_scalar(
            Bls12381ArkworksScalarOverrides {
                value: Some(Fr::from(2u64)),
            },
        )),
    });
    let Ok(success) = pairing.add_scalar(AddScalarParams, payload);
    assert_eq!(success.sum.value, Fr::from(1u64));
}

#[test]
fn mul_scalar_of_two_and_three_is_six() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = build_mul_scalar_payload(MulScalarPayloadOverrides {
        left: Some(build_bls12_381_arkworks_scalar(
            Bls12381ArkworksScalarOverrides {
                value: Some(Fr::from(2u64)),
            },
        )),
        right: Some(build_bls12_381_arkworks_scalar(
            Bls12381ArkworksScalarOverrides {
                value: Some(Fr::from(3u64)),
            },
        )),
    });
    let Ok(success) = pairing.mul_scalar(MulScalarParams, payload);
    assert_eq!(success.product.value, Fr::from(6u64));
}

#[test]
fn mul_scalar_reduces_modulo_the_group_order() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = build_mul_scalar_payload(MulScalarPayloadOverrides {
        left: Some(build_bls12_381_arkworks_scalar(
            Bls12381ArkworksScalarOverrides {
                value: Some(scalar_value(GROUP_ORDER_MINUS_ONE_HEX)),
            },
        )),
        right: Some(build_bls12_381_arkworks_scalar(
            Bls12381ArkworksScalarOverrides {
                value: Some(Fr::from(2u64)),
            },
        )),
    });
    let Ok(success) = pairing.mul_scalar(MulScalarParams, payload);
    assert_eq!(
        success.product.value,
        scalar_value(GROUP_ORDER_MINUS_TWO_HEX)
    );
}

#[test]
fn neg_scalar_of_one_is_the_group_order_minus_one() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = build_neg_scalar_payload(NegScalarPayloadOverrides {
        scalar: Some(build_bls12_381_arkworks_scalar(
            Bls12381ArkworksScalarOverrides {
                value: Some(Fr::from(1u64)),
            },
        )),
    });
    let Ok(success) = pairing.neg_scalar(NegScalarParams, payload);
    assert_eq!(
        success.negation.value,
        scalar_value(GROUP_ORDER_MINUS_ONE_HEX)
    );
}

#[test]
fn neg_scalar_of_zero_is_zero() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = build_neg_scalar_payload(NegScalarPayloadOverrides {
        scalar: Some(build_bls12_381_arkworks_scalar(
            Bls12381ArkworksScalarOverrides {
                value: Some(Fr::from(0u64)),
            },
        )),
    });
    let Ok(success) = pairing.neg_scalar(NegScalarParams, payload);
    assert_eq!(success.negation.value, Fr::from(0u64));
}

#[test]
fn neg_g1_of_the_generator_is_the_negated_generator() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = build_neg_g1_payload(NegG1PayloadOverrides {
        point: Some(build_bls12_381_arkworks_g1(Default::default())),
    });
    let Ok(success) = pairing.neg_g1(NegG1Params, payload);
    assert_eq!(success.negation.value, eip_2537_negated_g1_generator());
}

#[test]
fn neg_g1_of_the_identity_is_the_identity() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = build_neg_g1_payload(NegG1PayloadOverrides {
        point: Some(build_bls12_381_arkworks_g1(Bls12381ArkworksG1Overrides {
            value: Some(G1Affine::identity()),
        })),
    });
    let Ok(success) = pairing.neg_g1(NegG1Params, payload);
    assert_eq!(success.negation.value, G1Affine::identity());
}

#[test]
fn neg_g2_negates_each_y_coefficient_modulo_the_base_field() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = build_neg_g2_payload(NegG2PayloadOverrides {
        point: Some(build_bls12_381_arkworks_g2(Default::default())),
    });
    let Ok(success) = pairing.neg_g2(NegG2Params, payload);
    let (x, y) = success
        .negation
        .value
        .xy()
        .expect("the negation has coordinates");
    let (generator_x, generator_y) = eip_2537_g2_generator()
        .xy()
        .expect("the generator has coordinates");
    assert_eq!(x, generator_x);
    assert_eq!(
        BigUint::from(y.c0.into_bigint()),
        base_field_modulus() - BigUint::from(generator_y.c0.into_bigint())
    );
    assert_eq!(
        BigUint::from(y.c1.into_bigint()),
        base_field_modulus() - BigUint::from(generator_y.c1.into_bigint())
    );
}

#[test]
fn neg_g2_of_the_identity_is_the_identity() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = build_neg_g2_payload(NegG2PayloadOverrides {
        point: Some(build_bls12_381_arkworks_g2(Bls12381ArkworksG2Overrides {
            value: Some(G2Affine::identity()),
        })),
    });
    let Ok(success) = pairing.neg_g2(NegG2Params, payload);
    assert_eq!(success.negation.value, G2Affine::identity());
}

#[test]
fn is_identity_g1_is_true_for_the_identity() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = build_is_identity_g1_payload(IsIdentityG1PayloadOverrides {
        point: Some(build_bls12_381_arkworks_g1(Bls12381ArkworksG1Overrides {
            value: Some(G1Affine::identity()),
        })),
    });
    let Ok(success) = pairing.is_identity_g1(IsIdentityG1Params, payload);
    assert!(success.is_identity);
}

#[test]
fn is_identity_g1_is_false_for_the_generator() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = build_is_identity_g1_payload(IsIdentityG1PayloadOverrides {
        point: Some(build_bls12_381_arkworks_g1(Default::default())),
    });
    let Ok(success) = pairing.is_identity_g1(IsIdentityG1Params, payload);
    assert!(!success.is_identity);
}

#[test]
fn is_identity_g2_is_true_for_the_identity() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = build_is_identity_g2_payload(IsIdentityG2PayloadOverrides {
        point: Some(build_bls12_381_arkworks_g2(Bls12381ArkworksG2Overrides {
            value: Some(G2Affine::identity()),
        })),
    });
    let Ok(success) = pairing.is_identity_g2(IsIdentityG2Params, payload);
    assert!(success.is_identity);
}

#[test]
fn is_identity_g2_is_false_for_the_generator() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = build_is_identity_g2_payload(IsIdentityG2PayloadOverrides {
        point: Some(build_bls12_381_arkworks_g2(Default::default())),
    });
    let Ok(success) = pairing.is_identity_g2(IsIdentityG2Params, payload);
    assert!(!success.is_identity);
}

#[test]
fn pairing_product_of_no_terms_is_the_target_group_identity() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload =
        build_pairing_product_payload::<Bls12381ArkworksG1, Bls12381ArkworksG2>(Default::default());
    let Ok(success) = pairing.pairing_product(PairingProductParams, payload);
    assert!(success.product.value.is_zero());
}

#[test]
fn pairing_product_of_the_generators_is_not_the_target_group_identity() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = build_pairing_product_payload::<Bls12381ArkworksG1, Bls12381ArkworksG2>(
        PairingProductPayloadOverrides {
            terms: Some(vec![build_pairing_product_term(Default::default())]),
        },
    );
    let Ok(success) = pairing.pairing_product(PairingProductParams, payload);
    assert!(!success.product.value.is_zero());
}

#[test]
fn pairing_product_is_bilinear() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = build_pairing_product_payload::<Bls12381ArkworksG1, Bls12381ArkworksG2>(
        PairingProductPayloadOverrides {
            terms: Some(vec![build_pairing_product_term(
                PairingProductTermOverrides {
                    g1: Some(build_bls12_381_arkworks_g1(Bls12381ArkworksG1Overrides {
                        value: Some(eip_2537_doubled_g1_generator()),
                    })),
                    g2: Some(build_bls12_381_arkworks_g2(Default::default())),
                },
            )]),
        },
    );
    let Ok(success) = pairing.pairing_product(PairingProductParams, payload);
    assert_eq!(
        success.product.value.0,
        definition_value_power(eip_2537_g1_generator(), eip_2537_g2_generator(), 2)
    );
}

#[test]
fn pairing_product_multiplies_its_terms() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = build_pairing_product_payload::<Bls12381ArkworksG1, Bls12381ArkworksG2>(
        PairingProductPayloadOverrides {
            terms: Some(vec![
                build_pairing_product_term(Default::default()),
                build_pairing_product_term(Default::default()),
            ]),
        },
    );
    let Ok(success) = pairing.pairing_product(PairingProductParams, payload);
    assert_eq!(
        success.product.value.0,
        definition_value_power(eip_2537_g1_generator(), eip_2537_g2_generator(), 2)
    );
}

#[test]
fn pairing_product_of_a_pairing_and_its_first_group_negation_is_the_target_group_identity() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = build_pairing_product_payload::<Bls12381ArkworksG1, Bls12381ArkworksG2>(
        PairingProductPayloadOverrides {
            terms: Some(vec![
                build_pairing_product_term(Default::default()),
                build_pairing_product_term(PairingProductTermOverrides {
                    g1: Some(build_bls12_381_arkworks_g1(Bls12381ArkworksG1Overrides {
                        value: Some(eip_2537_negated_g1_generator()),
                    })),
                    g2: Some(build_bls12_381_arkworks_g2(Default::default())),
                }),
            ]),
        },
    );
    let Ok(success) = pairing.pairing_product(PairingProductParams, payload);
    assert!(success.product.value.is_zero());
}

#[test]
fn pairing_product_of_the_generators_equals_the_definition() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = build_pairing_product_payload::<Bls12381ArkworksG1, Bls12381ArkworksG2>(
        PairingProductPayloadOverrides {
            terms: Some(vec![build_pairing_product_term(Default::default())]),
        },
    );
    let Ok(success) = pairing.pairing_product(PairingProductParams, payload);
    assert_eq!(
        success.product.value.0,
        definition_value(eip_2537_g1_generator(), eip_2537_g2_generator())
    );
}

#[test]
fn encode_gt_writes_the_target_group_identity_with_c0_c0_c0_first() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = build_encode_gt_payload(EncodeGtPayloadOverrides {
        value: Some(build_bls12_381_arkworks_gt(Bls12381ArkworksGtOverrides {
            value: Some(PairingOutput(Fq12::ONE)),
        })),
    });
    let Ok(success) = pairing.encode_gt(EncodeGtParams, payload);
    let bytes: &Bls12381ArkworksEncodedGt = success.bytes.expose();
    assert_eq!(bytes.as_ref(), gt_identity_encoding().as_slice());
}

#[test]
fn encode_gt_writes_the_published_pairing_of_the_generators() {
    let pairing = build_bls12_381_arkworks_pairing();
    let payload = build_encode_gt_payload(EncodeGtPayloadOverrides {
        value: Some(build_bls12_381_arkworks_gt(Bls12381ArkworksGtOverrides {
            value: Some(PairingOutput(definition_value(
                eip_2537_g1_generator(),
                eip_2537_g2_generator(),
            ))),
        })),
    });
    let Ok(success) = pairing.encode_gt(EncodeGtParams, payload);
    assert_eq!(
        success.bytes.expose().as_ref(),
        vector_bytes(CFRG_GENERATOR_PAIRING_HEX).as_slice()
    );
}

#[test]
fn scalar_field_order_is_the_group_order() {
    let pairing = build_bls12_381_arkworks_pairing();
    let Ok(success) = pairing.scalar_field_order(ScalarFieldOrderParams, ScalarFieldOrderPayload);
    assert_eq!(success.bytes, vector_bytes(GROUP_ORDER_HEX));
}

#[test]
fn g1_outside_subgroup_encoding_encodes_an_on_curve_point_outside_the_subgroup() {
    let pairing = build_bls12_381_arkworks_pairing();
    let success = pairing
        .g1_outside_subgroup_encoding(
            G1OutsideSubgroupEncodingParams,
            G1OutsideSubgroupEncodingPayload,
        )
        .expect("the first group reports an outside-subgroup point");
    let bytes: Bls12381ArkworksEncodedG1 = success.bytes.expect("the bytes are present");
    let point = g1_point_from_encoding(bytes.as_ref());
    assert!(point.is_on_curve());
    assert!(!point.is_in_correct_subgroup_assuming_on_curve());
}

#[test]
fn g1_outside_subgroup_encoding_uses_the_least_first_coordinate() {
    let pairing = build_bls12_381_arkworks_pairing();
    let success = pairing
        .g1_outside_subgroup_encoding(
            G1OutsideSubgroupEncodingParams,
            G1OutsideSubgroupEncodingPayload,
        )
        .expect("the first group reports an outside-subgroup point");
    let bytes = success
        .bytes
        .expect("the bytes are present")
        .as_ref()
        .to_vec();
    let expected = g1_outside_subgroup_bytes();
    assert_eq!(&bytes[0..64], &expected[0..64]);
}

#[test]
fn g1_outside_subgroup_encoding_takes_the_lesser_root() {
    let pairing = build_bls12_381_arkworks_pairing();
    let success = pairing
        .g1_outside_subgroup_encoding(
            G1OutsideSubgroupEncodingParams,
            G1OutsideSubgroupEncodingPayload,
        )
        .expect("the first group reports an outside-subgroup point");
    let bytes = success
        .bytes
        .expect("the bytes are present")
        .as_ref()
        .to_vec();
    let y = BigUint::from_bytes_be(&bytes[80..128]);
    assert!(y <= base_field_modulus() - &y);
}

#[test]
fn g2_outside_subgroup_encoding_encodes_an_on_curve_point_outside_the_subgroup() {
    let pairing = build_bls12_381_arkworks_pairing();
    let success = pairing
        .g2_outside_subgroup_encoding(
            G2OutsideSubgroupEncodingParams,
            G2OutsideSubgroupEncodingPayload,
        )
        .expect("the second group reports an outside-subgroup point");
    let bytes: Bls12381ArkworksEncodedG2 = success.bytes;
    let point = g2_point_from_encoding(bytes.as_ref());
    assert!(point.is_on_curve());
    assert!(!point.is_in_correct_subgroup_assuming_on_curve());
}

#[test]
fn g2_outside_subgroup_encoding_has_a_real_first_coordinate_and_the_lesser_root() {
    let pairing = build_bls12_381_arkworks_pairing();
    let success = pairing
        .g2_outside_subgroup_encoding(
            G2OutsideSubgroupEncodingParams,
            G2OutsideSubgroupEncodingPayload,
        )
        .expect("the second group reports an outside-subgroup point");
    let bytes = success.bytes.as_ref();
    let expected = g2_outside_subgroup_bytes();
    assert_eq!(&bytes[64..128], zero_bytes(64).as_slice());
    assert_eq!(&bytes[0..64], &expected[0..64]);
    let modulus = base_field_modulus();
    let y_c1 = BigUint::from_bytes_be(&bytes[208..256]);
    let y_c0 = BigUint::from_bytes_be(&bytes[144..192]);
    let negate = |c: &BigUint| {
        if c == &BigUint::from(0u32) {
            BigUint::from(0u32)
        } else {
            &modulus - c
        }
    };
    assert!((y_c1.clone(), y_c0.clone()) <= (negate(&y_c1), negate(&y_c0)));
}
