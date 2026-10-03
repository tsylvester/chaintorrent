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
    G1OutsideSubgroupEncodingParams, G1OutsideSubgroupEncodingPayload,
    G1OutsideSubgroupEncodingReturn, G1OutsideSubgroupEncodingSuccessReturn, G2GeneratorParams,
    G2GeneratorPayload, G2GeneratorReturn, G2GeneratorSuccessReturn,
    G2OutsideSubgroupEncodingErrorReturn, G2OutsideSubgroupEncodingParams,
    G2OutsideSubgroupEncodingPayload, G2OutsideSubgroupEncodingReturn,
    G2OutsideSubgroupEncodingSuccessReturn, IPairingAdapter, IPairingArithmetic, IPairingReference,
    ISampleUniformScalar, IsIdentityG1Params, IsIdentityG1Payload, IsIdentityG1Return,
    IsIdentityG1SuccessReturn, IsIdentityG2Params, IsIdentityG2Payload, IsIdentityG2Return,
    IsIdentityG2SuccessReturn, MsmG1Params, MsmG1Payload, MsmG1Return, MsmG1SuccessReturn,
    MsmG2Params, MsmG2Payload, MsmG2Return, MsmG2SuccessReturn, MulG1Params, MulG1Payload,
    MulG1Return, MulG1SuccessReturn, MulG2Params, MulG2Payload, MulG2Return, MulG2SuccessReturn,
    MulScalarParams, MulScalarPayload, MulScalarReturn, MulScalarSuccessReturn, NegG1Params,
    NegG1Payload, NegG1Return, NegG1SuccessReturn, NegG2Params, NegG2Payload, NegG2Return,
    NegG2SuccessReturn, NegScalarParams, NegScalarPayload, NegScalarReturn, NegScalarSuccessReturn,
    PAIRING_INTERFACE_VERSION, PairingConcrete, PairingCurve, PairingDeclaration,
    PairingProductIsOneParams, PairingProductIsOnePayload, PairingProductIsOneReturn,
    PairingProductIsOneSuccessReturn, PairingProductParams, PairingProductPayload,
    PairingProductReturn, PairingProductSuccessReturn, PrecompileEncoding,
    SampleUniformScalarErrorReturn, SampleUniformScalarParams, SampleUniformScalarPayload,
    SampleUniformScalarReturn, SampleUniformScalarSuccessReturn, ScalarFieldOrderParams,
    ScalarFieldOrderPayload, ScalarFieldOrderReturn, ScalarFieldOrderSuccessReturn,
    TargetGroupEncodingIdentifier, VerifierGroupArithmetic,
};
use core::hint::black_box;
use core::iter::successors;
use domain::{Secret, SecretConstructorParams};
use halo2curves::bn256::{Bn256, Fq, Fq2, Fr, G1Affine, G2Affine, Gt};
use halo2curves::ff::{Field, FromUniformBytes, PrimeField};
use halo2curves::group::{Curve, Group, cofactor::CofactorGroup, prime::PrimeCurveAffine};
use halo2curves::msm::msm_best;
use halo2curves::pairing::{MillerLoopResult, MultiMillerLoop};
use halo2curves::{Coordinates, CurveAffine};
use interface::{
    Bn254Halo2curvesEncodedG1, Bn254Halo2curvesEncodedG2, Bn254Halo2curvesEncodedGt,
    Bn254Halo2curvesEncodedScalar, Bn254Halo2curvesG1, Bn254Halo2curvesG2, Bn254Halo2curvesGt,
    Bn254Halo2curvesPairing, Bn254Halo2curvesPairingConstructorParams,
    Bn254Halo2curvesPairingTryNewReturn, Bn254Halo2curvesScalar,
};
use zeroize::{Zeroize, ZeroizeOnDrop};

impl Bn254Halo2curvesPairing {
    pub const DECLARATION: PairingDeclaration = PairingDeclaration {
        curve: PairingCurve::Bn254,
        verifier_group_arithmetic: VerifierGroupArithmetic::FirstGroupOnly,
        precompile_encoding: PrecompileEncoding::Eip196Eip197,
        target_group_encoding: TargetGroupEncodingIdentifier::Bn254V1,
        adapter_version: 1,
        interface_version: PAIRING_INTERFACE_VERSION,
    };

    pub fn try_new(
        _params: Bn254Halo2curvesPairingConstructorParams,
    ) -> Bn254Halo2curvesPairingTryNewReturn {
        Ok(Bn254Halo2curvesPairing)
    }
}

impl IPairingAdapter for Bn254Halo2curvesPairing {
    const DECLARATION: PairingDeclaration = Bn254Halo2curvesPairing::DECLARATION;
    const CONCRETE: PairingConcrete = PairingConcrete::Bn254Halo2curves;
    type Scalar = Bn254Halo2curvesScalar;
    type G1 = Bn254Halo2curvesG1;
    type G2 = Bn254Halo2curvesG2;
    type EncodedG1 = Bn254Halo2curvesEncodedG1;
    type EncodedG2 = Bn254Halo2curvesEncodedG2;
    type EncodedScalar = Bn254Halo2curvesEncodedScalar;

    fn g1_generator(
        &self,
        _params: G1GeneratorParams,
        _payload: G1GeneratorPayload,
    ) -> G1GeneratorReturn<Self::G1> {
        Ok(G1GeneratorSuccessReturn {
            point: Bn254Halo2curvesG1 {
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
            point: Bn254Halo2curvesG2 {
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
            sum: Bn254Halo2curvesG1 {
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
            sum: Bn254Halo2curvesG2 {
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
            product: Bn254Halo2curvesG1 {
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
            product: Bn254Halo2curvesG2 {
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
            sum: Bn254Halo2curvesG1 { value: sum },
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
            sum: Bn254Halo2curvesG2 { value: sum },
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
        let product = Bn256::multi_miller_loop(&terms).final_exponentiation();
        Ok(PairingProductIsOneSuccessReturn {
            is_one: bool::from(product.is_identity()),
        })
    }

    fn decode_g1(&self, _params: DecodeG1Params, payload: &[u8]) -> DecodeG1Return<Self::G1> {
        let Ok(bytes) = <[u8; 64]>::try_from(payload) else {
            return Err(DecodeG1ErrorReturn::WrongLength {
                expected: 64,
                actual: payload.len(),
            });
        };
        let mut x_repr = [0u8; 32];
        x_repr.copy_from_slice(&bytes[0..32]);
        x_repr.reverse();
        let mut y_repr = [0u8; 32];
        y_repr.copy_from_slice(&bytes[32..64]);
        y_repr.reverse();
        let Some(x) = Option::<Fq>::from(Fq::from_repr(x_repr.into())) else {
            return Err(DecodeG1ErrorReturn::NonCanonicalCoordinate);
        };
        let Some(y) = Option::<Fq>::from(Fq::from_repr(y_repr.into())) else {
            return Err(DecodeG1ErrorReturn::NonCanonicalCoordinate);
        };
        if bool::from(x.is_zero()) && bool::from(y.is_zero()) {
            return Ok(DecodeG1SuccessReturn {
                point: Bn254Halo2curvesG1 {
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
            point: Bn254Halo2curvesG1 { value: point },
        })
    }

    fn decode_g2(&self, _params: DecodeG2Params, payload: &[u8]) -> DecodeG2Return<Self::G2> {
        let Ok(bytes) = <[u8; 128]>::try_from(payload) else {
            return Err(DecodeG2ErrorReturn::WrongLength {
                expected: 128,
                actual: payload.len(),
            });
        };
        let mut x_c1_repr = [0u8; 32];
        x_c1_repr.copy_from_slice(&bytes[0..32]);
        x_c1_repr.reverse();
        let mut x_c0_repr = [0u8; 32];
        x_c0_repr.copy_from_slice(&bytes[32..64]);
        x_c0_repr.reverse();
        let mut y_c1_repr = [0u8; 32];
        y_c1_repr.copy_from_slice(&bytes[64..96]);
        y_c1_repr.reverse();
        let mut y_c0_repr = [0u8; 32];
        y_c0_repr.copy_from_slice(&bytes[96..128]);
        y_c0_repr.reverse();
        let Some(x_c1) = Option::<Fq>::from(Fq::from_repr(x_c1_repr.into())) else {
            return Err(DecodeG2ErrorReturn::NonCanonicalCoordinate);
        };
        let Some(x_c0) = Option::<Fq>::from(Fq::from_repr(x_c0_repr.into())) else {
            return Err(DecodeG2ErrorReturn::NonCanonicalCoordinate);
        };
        let Some(y_c1) = Option::<Fq>::from(Fq::from_repr(y_c1_repr.into())) else {
            return Err(DecodeG2ErrorReturn::NonCanonicalCoordinate);
        };
        let Some(y_c0) = Option::<Fq>::from(Fq::from_repr(y_c0_repr.into())) else {
            return Err(DecodeG2ErrorReturn::NonCanonicalCoordinate);
        };
        if bool::from(x_c1.is_zero())
            && bool::from(x_c0.is_zero())
            && bool::from(y_c1.is_zero())
            && bool::from(y_c0.is_zero())
        {
            return Ok(DecodeG2SuccessReturn {
                point: Bn254Halo2curvesG2 {
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
            point: Bn254Halo2curvesG2 { value: point },
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
            scalar: Bn254Halo2curvesScalar { value: scalar },
        })
    }

    fn encode_g1(
        &self,
        _params: EncodeG1Params,
        payload: EncodeG1Payload<Self::G1>,
    ) -> EncodeG1Return<Self::EncodedG1> {
        let mut bytes = [0u8; 64];
        if let Some(coordinates) =
            Option::<Coordinates<G1Affine>>::from(payload.point.value.coordinates())
        {
            let mut x = coordinates.x().to_repr();
            x.as_mut().reverse();
            bytes[0..32].copy_from_slice(x.as_ref());
            let mut y = coordinates.y().to_repr();
            y.as_mut().reverse();
            bytes[32..64].copy_from_slice(y.as_ref());
        }
        Ok(EncodeG1SuccessReturn {
            bytes: Bn254Halo2curvesEncodedG1 { bytes },
        })
    }

    fn encode_g2(
        &self,
        _params: EncodeG2Params,
        payload: EncodeG2Payload<Self::G2>,
    ) -> EncodeG2Return<Self::EncodedG2> {
        let mut bytes = [0u8; 128];
        if let Some(coordinates) =
            Option::<Coordinates<G2Affine>>::from(payload.point.value.coordinates())
        {
            let mut x_c1 = coordinates.x().c1().to_repr();
            x_c1.as_mut().reverse();
            bytes[0..32].copy_from_slice(x_c1.as_ref());
            let mut x_c0 = coordinates.x().c0().to_repr();
            x_c0.as_mut().reverse();
            bytes[32..64].copy_from_slice(x_c0.as_ref());
            let mut y_c1 = coordinates.y().c1().to_repr();
            y_c1.as_mut().reverse();
            bytes[64..96].copy_from_slice(y_c1.as_ref());
            let mut y_c0 = coordinates.y().c0().to_repr();
            y_c0.as_mut().reverse();
            bytes[96..128].copy_from_slice(y_c0.as_ref());
        }
        Ok(EncodeG2SuccessReturn {
            bytes: Bn254Halo2curvesEncodedG2 { bytes },
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
            value: Bn254Halo2curvesEncodedScalar { bytes },
        });
        Ok(EncodeScalarSuccessReturn { bytes })
    }
}

impl IPairingArithmetic for Bn254Halo2curvesPairing {
    type Gt = Bn254Halo2curvesGt;
    type EncodedGt = Bn254Halo2curvesEncodedGt;

    fn add_scalar(
        &self,
        _params: AddScalarParams,
        payload: AddScalarPayload<Self::Scalar>,
    ) -> AddScalarReturn<Self::Scalar> {
        Ok(AddScalarSuccessReturn {
            sum: Bn254Halo2curvesScalar {
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
            product: Bn254Halo2curvesScalar {
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
            negation: Bn254Halo2curvesScalar {
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
            negation: Bn254Halo2curvesG1 {
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
            negation: Bn254Halo2curvesG2 {
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
        let product = Bn256::multi_miller_loop(&terms).final_exponentiation();
        Ok(PairingProductSuccessReturn {
            product: Bn254Halo2curvesGt { value: product },
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
        let mut bytes = [0u8; 384];
        for (position, coefficient) in coefficients.iter().enumerate() {
            let mut repr = coefficient.to_repr();
            repr.as_mut().reverse();
            bytes[position * 32..(position + 1) * 32].copy_from_slice(repr.as_ref());
        }
        let Ok(bytes) = Secret::try_new(SecretConstructorParams {
            value: Bn254Halo2curvesEncodedGt { bytes },
        });
        Ok(EncodeGtSuccessReturn { bytes })
    }
}

impl IPairingReference for Bn254Halo2curvesPairing {
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
        Ok(G1OutsideSubgroupEncodingSuccessReturn { bytes: None })
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
                point: Bn254Halo2curvesG2 { value },
            },
        );
        Ok(G2OutsideSubgroupEncodingSuccessReturn {
            bytes: encoded.bytes,
        })
    }
}

impl AsRef<[u8]> for Bn254Halo2curvesEncodedG1 {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

impl AsRef<[u8]> for Bn254Halo2curvesEncodedG2 {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

impl AsRef<[u8]> for Bn254Halo2curvesEncodedScalar {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

impl AsRef<[u8]> for Bn254Halo2curvesEncodedGt {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

impl Zeroize for Bn254Halo2curvesEncodedScalar {
    fn zeroize(&mut self) {
        self.bytes.zeroize();
    }
}

impl Zeroize for Bn254Halo2curvesEncodedGt {
    fn zeroize(&mut self) {
        self.bytes.zeroize();
    }
}

impl Zeroize for Bn254Halo2curvesScalar {
    fn zeroize(&mut self) {
        self.value = Fr::ZERO;
        black_box(&self.value);
    }
}

impl Drop for Bn254Halo2curvesScalar {
    fn drop(&mut self) {
        self.value = Fr::ZERO;
        black_box(&self.value);
    }
}

impl ZeroizeOnDrop for Bn254Halo2curvesScalar {}

impl Zeroize for Bn254Halo2curvesG1 {
    fn zeroize(&mut self) {
        self.value = G1Affine::identity();
        black_box(&self.value);
    }
}

impl Drop for Bn254Halo2curvesG1 {
    fn drop(&mut self) {
        self.value = G1Affine::identity();
        black_box(&self.value);
    }
}

impl Zeroize for Bn254Halo2curvesG2 {
    fn zeroize(&mut self) {
        self.value = G2Affine::identity();
        black_box(&self.value);
    }
}

impl Drop for Bn254Halo2curvesG2 {
    fn drop(&mut self) {
        self.value = G2Affine::identity();
        black_box(&self.value);
    }
}

impl Zeroize for Bn254Halo2curvesGt {
    fn zeroize(&mut self) {
        self.value = Gt::identity();
        black_box(&self.value);
    }
}

impl Drop for Bn254Halo2curvesGt {
    fn drop(&mut self) {
        self.value = Gt::identity();
        black_box(&self.value);
    }
}

impl ISampleUniformScalar for Bn254Halo2curvesScalar {
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
        let value = Bn254Halo2curvesScalar { value };
        let Ok(scalar) = Secret::try_new(SecretConstructorParams { value });
        Ok(SampleUniformScalarSuccessReturn { scalar })
    }
}
