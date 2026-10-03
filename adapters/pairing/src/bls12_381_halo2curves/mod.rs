mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub(crate) mod provides;
#[cfg(test)]
mod test;

use crate::factory::provides::{
    AddG1Params, AddG1Payload, AddG1Return, AddG1SuccessReturn, AddG2Params, AddG2Payload,
    AddG2Return, AddG2SuccessReturn, AddScalarParams, AddScalarPayload, AddScalarReturn,
    AddScalarSuccessReturn, DecodeG1ErrorReturn, DecodeG1Params, DecodeG1Return,
    DecodeG1SuccessReturn, DecodeG2ErrorReturn, DecodeG2Params, DecodeG2Return,
    DecodeG2SuccessReturn, DecodeScalarErrorReturn, DecodeScalarParams, DecodeScalarReturn,
    DecodeScalarSuccessReturn, EncodeG1Params, EncodeG1Payload, EncodeG1Return,
    EncodeG1SuccessReturn, EncodeG2Params, EncodeG2Payload, EncodeG2Return, EncodeG2SuccessReturn,
    EncodeGtParams, EncodeGtPayload, EncodeGtReturn, EncodeGtSuccessReturn, EncodeScalarParams,
    EncodeScalarPayload, EncodeScalarReturn, EncodeScalarSuccessReturn, G1GeneratorParams,
    G1GeneratorPayload, G1GeneratorReturn, G1GeneratorSuccessReturn,
    G1OutsideSubgroupEncodingErrorReturn, G1OutsideSubgroupEncodingParams,
    G1OutsideSubgroupEncodingPayload, G1OutsideSubgroupEncodingReturn,
    G1OutsideSubgroupEncodingSuccessReturn, G2GeneratorParams, G2GeneratorPayload,
    G2GeneratorReturn, G2GeneratorSuccessReturn, G2OutsideSubgroupEncodingErrorReturn,
    G2OutsideSubgroupEncodingParams, G2OutsideSubgroupEncodingPayload,
    G2OutsideSubgroupEncodingReturn, G2OutsideSubgroupEncodingSuccessReturn, IPairingAdapter,
    IPairingArithmetic, IPairingReference, ISampleUniformScalar, IsIdentityG1Params,
    IsIdentityG1Payload, IsIdentityG1Return, IsIdentityG1SuccessReturn, IsIdentityG2Params,
    IsIdentityG2Payload, IsIdentityG2Return, IsIdentityG2SuccessReturn, MsmG1Params, MsmG1Payload,
    MsmG1Return, MsmG1SuccessReturn, MsmG2Params, MsmG2Payload, MsmG2Return, MsmG2SuccessReturn,
    MulG1Params, MulG1Payload, MulG1Return, MulG1SuccessReturn, MulG2Params, MulG2Payload,
    MulG2Return, MulG2SuccessReturn, MulScalarParams, MulScalarPayload, MulScalarReturn,
    MulScalarSuccessReturn, NegG1Params, NegG1Payload, NegG1Return, NegG1SuccessReturn,
    NegG2Params, NegG2Payload, NegG2Return, NegG2SuccessReturn, NegScalarParams, NegScalarPayload,
    NegScalarReturn, NegScalarSuccessReturn, PAIRING_INTERFACE_VERSION, PairingConcrete,
    PairingCurve, PairingDeclaration, PairingProductIsOneParams, PairingProductIsOnePayload,
    PairingProductIsOneReturn, PairingProductIsOneSuccessReturn, PairingProductParams,
    PairingProductPayload, PairingProductReturn, PairingProductSuccessReturn, PrecompileEncoding,
    SampleUniformScalarErrorReturn, SampleUniformScalarParams, SampleUniformScalarPayload,
    SampleUniformScalarReturn, SampleUniformScalarSuccessReturn, ScalarFieldOrderParams,
    ScalarFieldOrderPayload, ScalarFieldOrderReturn, ScalarFieldOrderSuccessReturn,
    TargetGroupEncodingIdentifier, VerifierGroupArithmetic,
};
use core::array::from_fn;
use core::hint::black_box;
use core::iter::successors;
use domain::{Secret, SecretConstructorParams};
use halo2curves::bls12381::{Bls12381, Fq, Fq2, Fr, G1Affine, G2Affine, Gt};
use halo2curves::ff::{Field, FromUniformBytes, PrimeField};
use halo2curves::group::{Curve, Group, cofactor::CofactorGroup, prime::PrimeCurveAffine};
use halo2curves::msm::msm_best;
use halo2curves::pairing::{MillerLoopResult, MultiMillerLoop};
use halo2curves::{Coordinates, CurveAffine};
use interface::{
    Bls12381Halo2curvesEncodedG1, Bls12381Halo2curvesEncodedG2, Bls12381Halo2curvesEncodedGt,
    Bls12381Halo2curvesEncodedScalar, Bls12381Halo2curvesG1, Bls12381Halo2curvesG2,
    Bls12381Halo2curvesGt, Bls12381Halo2curvesPairing, Bls12381Halo2curvesPairingConstructorParams,
    Bls12381Halo2curvesPairingTryNewReturn, Bls12381Halo2curvesScalar,
};
use zeroize::{Zeroize, ZeroizeOnDrop};

fn decode_coordinate(bytes: &[u8]) -> Option<Fq> {
    if bytes[..16].iter().any(|byte| *byte != 0) {
        return None;
    }
    let mut repr = [0u8; 48];
    repr.copy_from_slice(&bytes[16..64]);
    repr.reverse();
    Option::<Fq>::from(Fq::from_repr(repr.into()))
}

impl Bls12381Halo2curvesPairing {
    pub const DECLARATION: PairingDeclaration = PairingDeclaration {
        curve: PairingCurve::Bls12381,
        verifier_group_arithmetic: VerifierGroupArithmetic::BothGroups,
        precompile_encoding: PrecompileEncoding::Eip2537,
        target_group_encoding: TargetGroupEncodingIdentifier::Bls12381V1,
        adapter_version: 1,
        interface_version: PAIRING_INTERFACE_VERSION,
    };

    pub fn try_new(
        _params: Bls12381Halo2curvesPairingConstructorParams,
    ) -> Bls12381Halo2curvesPairingTryNewReturn {
        let order_minus_two = (-Fr::from(2u64)).to_repr();
        let limbs: [u64; 4] = from_fn(|index| {
            order_minus_two.as_ref()[index * 8..(index + 1) * 8]
                .iter()
                .rev()
                .fold(0u64, |limb, byte| (limb << 8) | u64::from(*byte))
        });
        let reduced_pairing_correction = Fr::from(3u64).pow_vartime(limbs);
        Ok(Bls12381Halo2curvesPairing {
            reduced_pairing_correction,
        })
    }
}

impl IPairingAdapter for Bls12381Halo2curvesPairing {
    const DECLARATION: PairingDeclaration = Bls12381Halo2curvesPairing::DECLARATION;
    const CONCRETE: PairingConcrete = PairingConcrete::Bls12381Halo2curves;

    type Scalar = Bls12381Halo2curvesScalar;
    type G1 = Bls12381Halo2curvesG1;
    type G2 = Bls12381Halo2curvesG2;
    type EncodedG1 = Bls12381Halo2curvesEncodedG1;
    type EncodedG2 = Bls12381Halo2curvesEncodedG2;
    type EncodedScalar = Bls12381Halo2curvesEncodedScalar;

    fn g1_generator(
        &self,
        _params: G1GeneratorParams,
        _payload: G1GeneratorPayload,
    ) -> G1GeneratorReturn<Self::G1> {
        Ok(G1GeneratorSuccessReturn {
            point: Bls12381Halo2curvesG1 {
                value: G1Affine::generator(),
            },
        })
    }

    fn g2_generator(
        &self,
        _params: G2GeneratorParams,
        _payload: G2GeneratorPayload,
    ) -> G2GeneratorReturn<Self::G2> {
        Ok(G2GeneratorSuccessReturn {
            point: Bls12381Halo2curvesG2 {
                value: G2Affine::generator(),
            },
        })
    }

    fn add_g1(
        &self,
        _params: AddG1Params,
        payload: AddG1Payload<Self::G1>,
    ) -> AddG1Return<Self::G1> {
        Ok(AddG1SuccessReturn {
            sum: Bls12381Halo2curvesG1 {
                value: (payload.left.value.to_curve() + payload.right.value.to_curve()).to_affine(),
            },
        })
    }

    fn add_g2(
        &self,
        _params: AddG2Params,
        payload: AddG2Payload<Self::G2>,
    ) -> AddG2Return<Self::G2> {
        Ok(AddG2SuccessReturn {
            sum: Bls12381Halo2curvesG2 {
                value: (payload.left.value.to_curve() + payload.right.value.to_curve()).to_affine(),
            },
        })
    }

    fn mul_g1(
        &self,
        _params: MulG1Params,
        payload: MulG1Payload<Self::G1, Self::Scalar>,
    ) -> MulG1Return<Self::G1> {
        Ok(MulG1SuccessReturn {
            product: Bls12381Halo2curvesG1 {
                value: (payload.point.value.to_curve() * payload.scalar.value).to_affine(),
            },
        })
    }

    fn mul_g2(
        &self,
        _params: MulG2Params,
        payload: MulG2Payload<Self::G2, Self::Scalar>,
    ) -> MulG2Return<Self::G2> {
        Ok(MulG2SuccessReturn {
            product: Bls12381Halo2curvesG2 {
                value: (payload.point.value.to_curve() * payload.scalar.value).to_affine(),
            },
        })
    }

    fn msm_g1(
        &self,
        _params: MsmG1Params,
        payload: MsmG1Payload<Self::G1, Self::Scalar>,
    ) -> MsmG1Return<Self::G1> {
        let (bases, mut scalars): (Vec<G1Affine>, Vec<Fr>) = payload
            .terms
            .iter()
            .map(|term| (term.base.value, term.scalar.value))
            .unzip();
        let sum = msm_best(&scalars, &bases).to_affine();
        scalars.fill(Fr::ZERO);
        black_box(&scalars);
        Ok(MsmG1SuccessReturn {
            sum: Bls12381Halo2curvesG1 { value: sum },
        })
    }

    fn msm_g2(
        &self,
        _params: MsmG2Params,
        payload: MsmG2Payload<Self::G2, Self::Scalar>,
    ) -> MsmG2Return<Self::G2> {
        let (bases, mut scalars): (Vec<G2Affine>, Vec<Fr>) = payload
            .terms
            .iter()
            .map(|term| (term.base.value, term.scalar.value))
            .unzip();
        let sum = msm_best(&scalars, &bases).to_affine();
        scalars.fill(Fr::ZERO);
        black_box(&scalars);
        Ok(MsmG2SuccessReturn {
            sum: Bls12381Halo2curvesG2 { value: sum },
        })
    }

    fn pairing_product_is_one(
        &self,
        _params: PairingProductIsOneParams,
        payload: PairingProductIsOnePayload<Self::G1, Self::G2>,
    ) -> PairingProductIsOneReturn {
        let terms: Vec<(&G1Affine, &G2Affine)> = payload
            .terms
            .iter()
            .map(|term| (&term.g1.value, &term.g2.value))
            .collect();
        let product = Bls12381::multi_miller_loop(&terms).final_exponentiation();
        Ok(PairingProductIsOneSuccessReturn {
            is_one: bool::from(product.is_identity()),
        })
    }

    fn decode_g1(&self, _params: DecodeG1Params, payload: &[u8]) -> DecodeG1Return<Self::G1> {
        let Ok(bytes) = <[u8; 128]>::try_from(payload) else {
            return Err(DecodeG1ErrorReturn::WrongLength {
                expected: 128,
                actual: payload.len(),
            });
        };
        let (Some(x), Some(y)) = (
            decode_coordinate(&bytes[0..64]),
            decode_coordinate(&bytes[64..128]),
        ) else {
            return Err(DecodeG1ErrorReturn::NonCanonicalCoordinate);
        };
        if bool::from(x.is_zero()) && bool::from(y.is_zero()) {
            return Ok(DecodeG1SuccessReturn {
                point: Bls12381Halo2curvesG1 {
                    value: G1Affine::identity(),
                },
            });
        }
        let Some(point) = Option::<G1Affine>::from(G1Affine::from_xy(x, y)) else {
            return Err(DecodeG1ErrorReturn::NotOnCurve);
        };
        if !bool::from(point.to_curve().is_torsion_free()) {
            return Err(DecodeG1ErrorReturn::NotInSubgroup);
        }
        Ok(DecodeG1SuccessReturn {
            point: Bls12381Halo2curvesG1 { value: point },
        })
    }

    fn decode_g2(&self, _params: DecodeG2Params, payload: &[u8]) -> DecodeG2Return<Self::G2> {
        let Ok(bytes) = <[u8; 256]>::try_from(payload) else {
            return Err(DecodeG2ErrorReturn::WrongLength {
                expected: 256,
                actual: payload.len(),
            });
        };
        let (Some(x_c0), Some(x_c1), Some(y_c0), Some(y_c1)) = (
            decode_coordinate(&bytes[0..64]),
            decode_coordinate(&bytes[64..128]),
            decode_coordinate(&bytes[128..192]),
            decode_coordinate(&bytes[192..256]),
        ) else {
            return Err(DecodeG2ErrorReturn::NonCanonicalCoordinate);
        };
        if bool::from(x_c0.is_zero())
            && bool::from(x_c1.is_zero())
            && bool::from(y_c0.is_zero())
            && bool::from(y_c1.is_zero())
        {
            return Ok(DecodeG2SuccessReturn {
                point: Bls12381Halo2curvesG2 {
                    value: G2Affine::identity(),
                },
            });
        }
        let x = Fq2::new(x_c0, x_c1);
        let y = Fq2::new(y_c0, y_c1);
        let Some(point) = Option::<G2Affine>::from(G2Affine::from_xy(x, y)) else {
            return Err(DecodeG2ErrorReturn::NotOnCurve);
        };
        if !bool::from(point.to_curve().is_torsion_free()) {
            return Err(DecodeG2ErrorReturn::NotInSubgroup);
        }
        Ok(DecodeG2SuccessReturn {
            point: Bls12381Halo2curvesG2 { value: point },
        })
    }

    fn decode_scalar(
        &self,
        _params: DecodeScalarParams,
        payload: &[u8],
    ) -> DecodeScalarReturn<Self::Scalar> {
        let Ok(bytes) = <[u8; 32]>::try_from(payload) else {
            return Err(DecodeScalarErrorReturn::WrongLength {
                expected: 32,
                actual: payload.len(),
            });
        };
        let mut repr = bytes;
        repr.reverse();
        let Some(scalar) = Option::<Fr>::from(Fr::from_repr(repr.into())) else {
            return Err(DecodeScalarErrorReturn::NonCanonical);
        };
        Ok(DecodeScalarSuccessReturn {
            scalar: Bls12381Halo2curvesScalar { value: scalar },
        })
    }

    fn encode_g1(
        &self,
        _params: EncodeG1Params,
        payload: EncodeG1Payload<Self::G1>,
    ) -> EncodeG1Return<Self::EncodedG1> {
        let mut bytes = [0u8; 128];
        if let Some(coordinates) =
            Option::<Coordinates<G1Affine>>::from(payload.point.value.coordinates())
        {
            let mut x = coordinates.x().to_repr();
            x.as_mut().reverse();
            bytes[16..64].copy_from_slice(x.as_ref());
            let mut y = coordinates.y().to_repr();
            y.as_mut().reverse();
            bytes[80..128].copy_from_slice(y.as_ref());
        }
        Ok(EncodeG1SuccessReturn {
            bytes: Bls12381Halo2curvesEncodedG1 { bytes },
        })
    }

    fn encode_g2(
        &self,
        _params: EncodeG2Params,
        payload: EncodeG2Payload<Self::G2>,
    ) -> EncodeG2Return<Self::EncodedG2> {
        let mut bytes = [0u8; 256];
        if let Some(coordinates) =
            Option::<Coordinates<G2Affine>>::from(payload.point.value.coordinates())
        {
            let mut x_c0 = coordinates.x().c0().to_repr();
            x_c0.as_mut().reverse();
            bytes[16..64].copy_from_slice(x_c0.as_ref());
            let mut x_c1 = coordinates.x().c1().to_repr();
            x_c1.as_mut().reverse();
            bytes[80..128].copy_from_slice(x_c1.as_ref());
            let mut y_c0 = coordinates.y().c0().to_repr();
            y_c0.as_mut().reverse();
            bytes[144..192].copy_from_slice(y_c0.as_ref());
            let mut y_c1 = coordinates.y().c1().to_repr();
            y_c1.as_mut().reverse();
            bytes[208..256].copy_from_slice(y_c1.as_ref());
        }
        Ok(EncodeG2SuccessReturn {
            bytes: Bls12381Halo2curvesEncodedG2 { bytes },
        })
    }

    fn encode_scalar(
        &self,
        _params: EncodeScalarParams,
        payload: EncodeScalarPayload<Self::Scalar>,
    ) -> EncodeScalarReturn<Self::EncodedScalar> {
        let mut repr = payload.scalar.value.to_repr();
        repr.as_mut().reverse();
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(repr.as_ref());
        let Ok(bytes) = Secret::try_new(SecretConstructorParams {
            value: Bls12381Halo2curvesEncodedScalar { bytes },
        });
        Ok(EncodeScalarSuccessReturn { bytes })
    }
}

impl IPairingArithmetic for Bls12381Halo2curvesPairing {
    type Gt = Bls12381Halo2curvesGt;
    type EncodedGt = Bls12381Halo2curvesEncodedGt;

    fn add_scalar(
        &self,
        _params: AddScalarParams,
        payload: AddScalarPayload<Self::Scalar>,
    ) -> AddScalarReturn<Self::Scalar> {
        Ok(AddScalarSuccessReturn {
            sum: Bls12381Halo2curvesScalar {
                value: payload.left.value + payload.right.value,
            },
        })
    }

    fn mul_scalar(
        &self,
        _params: MulScalarParams,
        payload: MulScalarPayload<Self::Scalar>,
    ) -> MulScalarReturn<Self::Scalar> {
        Ok(MulScalarSuccessReturn {
            product: Bls12381Halo2curvesScalar {
                value: payload.left.value * payload.right.value,
            },
        })
    }

    fn neg_scalar(
        &self,
        _params: NegScalarParams,
        payload: NegScalarPayload<Self::Scalar>,
    ) -> NegScalarReturn<Self::Scalar> {
        Ok(NegScalarSuccessReturn {
            negation: Bls12381Halo2curvesScalar {
                value: -payload.scalar.value,
            },
        })
    }

    fn neg_g1(
        &self,
        _params: NegG1Params,
        payload: NegG1Payload<Self::G1>,
    ) -> NegG1Return<Self::G1> {
        Ok(NegG1SuccessReturn {
            negation: Bls12381Halo2curvesG1 {
                value: -payload.point.value,
            },
        })
    }

    fn neg_g2(
        &self,
        _params: NegG2Params,
        payload: NegG2Payload<Self::G2>,
    ) -> NegG2Return<Self::G2> {
        Ok(NegG2SuccessReturn {
            negation: Bls12381Halo2curvesG2 {
                value: -payload.point.value,
            },
        })
    }

    fn is_identity_g1(
        &self,
        _params: IsIdentityG1Params,
        payload: IsIdentityG1Payload<Self::G1>,
    ) -> IsIdentityG1Return {
        Ok(IsIdentityG1SuccessReturn {
            is_identity: bool::from(payload.point.value.is_identity()),
        })
    }

    fn is_identity_g2(
        &self,
        _params: IsIdentityG2Params,
        payload: IsIdentityG2Payload<Self::G2>,
    ) -> IsIdentityG2Return {
        Ok(IsIdentityG2SuccessReturn {
            is_identity: bool::from(payload.point.value.is_identity()),
        })
    }

    fn pairing_product(
        &self,
        _params: PairingProductParams,
        payload: PairingProductPayload<Self::G1, Self::G2>,
    ) -> PairingProductReturn<Self::Gt> {
        let terms: Vec<(&G1Affine, &G2Affine)> = payload
            .terms
            .iter()
            .map(|term| (&term.g1.value, &term.g2.value))
            .collect();
        let gt = Bls12381::multi_miller_loop(&terms).final_exponentiation();
        Ok(PairingProductSuccessReturn {
            product: Bls12381Halo2curvesGt {
                value: gt * self.reduced_pairing_correction,
            },
        })
    }

    fn encode_gt(
        &self,
        _params: EncodeGtParams,
        payload: EncodeGtPayload<Self::Gt>,
    ) -> EncodeGtReturn<Self::EncodedGt> {
        let value = payload.value.value.inner();
        let coefficients = [
            value.c0().c0().c0(),
            value.c0().c0().c1(),
            value.c0().c1().c0(),
            value.c0().c1().c1(),
            value.c0().c2().c0(),
            value.c0().c2().c1(),
            value.c1().c0().c0(),
            value.c1().c0().c1(),
            value.c1().c1().c0(),
            value.c1().c1().c1(),
            value.c1().c2().c0(),
            value.c1().c2().c1(),
        ];
        let mut bytes = [0u8; 576];
        for (position, coefficient) in coefficients.iter().enumerate() {
            let mut repr = coefficient.to_repr();
            repr.as_mut().reverse();
            bytes[position * 48..(position + 1) * 48].copy_from_slice(repr.as_ref());
        }
        let Ok(bytes) = Secret::try_new(SecretConstructorParams {
            value: Bls12381Halo2curvesEncodedGt { bytes },
        });
        Ok(EncodeGtSuccessReturn { bytes })
    }
}

impl IPairingReference for Bls12381Halo2curvesPairing {
    fn scalar_field_order(
        &self,
        _params: ScalarFieldOrderParams,
        _payload: ScalarFieldOrderPayload,
    ) -> ScalarFieldOrderReturn {
        let mut bytes = (-Fr::ONE).to_repr();
        bytes.as_mut().reverse();
        bytes.as_mut()[31] |= 1;
        Ok(ScalarFieldOrderSuccessReturn {
            bytes: bytes.as_ref().to_vec(),
        })
    }

    fn g1_outside_subgroup_encoding(
        &self,
        _params: G1OutsideSubgroupEncodingParams,
        _payload: G1OutsideSubgroupEncodingPayload,
    ) -> G1OutsideSubgroupEncodingReturn<Self::EncodedG1> {
        let Some(value) = successors(Some(Fq::ONE), |x| Some(*x + Fq::ONE)).find_map(|x| {
            let y = Option::<Fq>::from((x.square() * x + G1Affine::b()).sqrt())?;
            let point = Option::<G1Affine>::from(G1Affine::from_xy(x, y))?;
            if bool::from(point.to_curve().is_torsion_free()) {
                return None;
            }
            let negated = -y;
            let mut y_bytes = y.to_repr();
            y_bytes.as_mut().reverse();
            let mut negated_bytes = negated.to_repr();
            negated_bytes.as_mut().reverse();
            if y_bytes.as_ref() <= negated_bytes.as_ref() {
                Some(point)
            } else {
                Some(-point)
            }
        }) else {
            return Err(G1OutsideSubgroupEncodingErrorReturn::SearchExhausted);
        };
        let Ok(encoded) = self.encode_g1(
            EncodeG1Params,
            EncodeG1Payload {
                point: Bls12381Halo2curvesG1 { value },
            },
        );
        Ok(G1OutsideSubgroupEncodingSuccessReturn {
            bytes: Some(encoded.bytes),
        })
    }

    fn g2_outside_subgroup_encoding(
        &self,
        _params: G2OutsideSubgroupEncodingParams,
        _payload: G2OutsideSubgroupEncodingPayload,
    ) -> G2OutsideSubgroupEncodingReturn<Self::EncodedG2> {
        let Some(value) = successors(Some(Fq::ONE), |c0| Some(*c0 + Fq::ONE)).find_map(|c0| {
            let x = Fq2::new(c0, Fq::ZERO);
            let y = Option::<Fq2>::from((x.square() * x + G2Affine::b()).sqrt())?;
            let point = Option::<G2Affine>::from(G2Affine::from_xy(x, y))?;
            if bool::from(point.to_curve().is_torsion_free()) {
                return None;
            }
            let negated = -y;
            let mut y_c1 = y.c1().to_repr();
            y_c1.as_mut().reverse();
            let mut y_c0 = y.c0().to_repr();
            y_c0.as_mut().reverse();
            let mut negated_c1 = negated.c1().to_repr();
            negated_c1.as_mut().reverse();
            let mut negated_c0 = negated.c0().to_repr();
            negated_c0.as_mut().reverse();
            if (y_c1.as_ref(), y_c0.as_ref()) <= (negated_c1.as_ref(), negated_c0.as_ref()) {
                Some(point)
            } else {
                Some(-point)
            }
        }) else {
            return Err(G2OutsideSubgroupEncodingErrorReturn::SearchExhausted);
        };
        let Ok(encoded) = self.encode_g2(
            EncodeG2Params,
            EncodeG2Payload {
                point: Bls12381Halo2curvesG2 { value },
            },
        );
        Ok(G2OutsideSubgroupEncodingSuccessReturn {
            bytes: encoded.bytes,
        })
    }
}

impl AsRef<[u8]> for Bls12381Halo2curvesEncodedG1 {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

impl AsRef<[u8]> for Bls12381Halo2curvesEncodedG2 {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

impl AsRef<[u8]> for Bls12381Halo2curvesEncodedScalar {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

impl AsRef<[u8]> for Bls12381Halo2curvesEncodedGt {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

impl Zeroize for Bls12381Halo2curvesEncodedScalar {
    fn zeroize(&mut self) {
        self.bytes.zeroize();
    }
}

impl Zeroize for Bls12381Halo2curvesEncodedGt {
    fn zeroize(&mut self) {
        self.bytes.zeroize();
    }
}

impl Zeroize for Bls12381Halo2curvesScalar {
    fn zeroize(&mut self) {
        self.value = Fr::ZERO;
        black_box(&self.value);
    }
}

impl Drop for Bls12381Halo2curvesScalar {
    fn drop(&mut self) {
        self.value = Fr::ZERO;
        black_box(&self.value);
    }
}

impl ZeroizeOnDrop for Bls12381Halo2curvesScalar {}

impl Zeroize for Bls12381Halo2curvesG1 {
    fn zeroize(&mut self) {
        self.value = G1Affine::identity();
        black_box(&self.value);
    }
}

impl Drop for Bls12381Halo2curvesG1 {
    fn drop(&mut self) {
        self.value = G1Affine::identity();
        black_box(&self.value);
    }
}

impl Zeroize for Bls12381Halo2curvesG2 {
    fn zeroize(&mut self) {
        self.value = G2Affine::identity();
        black_box(&self.value);
    }
}

impl Drop for Bls12381Halo2curvesG2 {
    fn drop(&mut self) {
        self.value = G2Affine::identity();
        black_box(&self.value);
    }
}

impl Zeroize for Bls12381Halo2curvesGt {
    fn zeroize(&mut self) {
        self.value = Gt::identity();
        black_box(&self.value);
    }
}

impl Drop for Bls12381Halo2curvesGt {
    fn drop(&mut self) {
        self.value = Gt::identity();
        black_box(&self.value);
    }
}

impl ISampleUniformScalar for Bls12381Halo2curvesScalar {
    const UNIFORM_BYTES_LENGTH: usize = 64;

    fn sample_from_uniform_bytes(
        _params: SampleUniformScalarParams,
        payload: SampleUniformScalarPayload,
    ) -> SampleUniformScalarReturn<Self> {
        let actual = payload.uniform.expose().len();
        let Ok(uniform) = <&[u8; 64]>::try_from(payload.uniform.expose().as_slice()) else {
            return Err(SampleUniformScalarErrorReturn::WrongLength {
                expected: Self::UNIFORM_BYTES_LENGTH,
                actual,
            });
        };
        let mut copy = *uniform;
        copy.reverse();
        let value = Fr::from_uniform_bytes(&copy);
        copy.zeroize();
        let Ok(scalar) = Secret::try_new(SecretConstructorParams {
            value: Bls12381Halo2curvesScalar { value },
        });
        Ok(SampleUniformScalarSuccessReturn { scalar })
    }
}
