mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub(crate) mod provides;
#[cfg(test)]
mod test;

use crate::factory::provides::{
    AddG1Params, AddG1Payload, AddG1Return, AddG1SuccessReturn, AddG2Params, AddG2Payload,
    AddG2Return, AddG2SuccessReturn, DecodeG1ErrorReturn, DecodeG1Params, DecodeG1Return,
    DecodeG1SuccessReturn, DecodeG2ErrorReturn, DecodeG2Params, DecodeG2Return,
    DecodeG2SuccessReturn, DecodeScalarErrorReturn, DecodeScalarParams, DecodeScalarReturn,
    DecodeScalarSuccessReturn, EncodeG1Params, EncodeG1Payload, EncodeG1Return,
    EncodeG1SuccessReturn, EncodeG2Params, EncodeG2Payload, EncodeG2Return, EncodeG2SuccessReturn,
    EncodeScalarParams, EncodeScalarPayload, EncodeScalarReturn, EncodeScalarSuccessReturn,
    G1GeneratorParams, G1GeneratorPayload, G1GeneratorReturn, G1GeneratorSuccessReturn,
    G2GeneratorParams, G2GeneratorPayload, G2GeneratorReturn, G2GeneratorSuccessReturn,
    IPairingAdapter, ISampleUniformScalar, MsmG1Params, MsmG1Payload, MsmG1Return,
    MsmG1SuccessReturn, MsmG2Params, MsmG2Payload, MsmG2Return, MsmG2SuccessReturn, MulG1Params,
    MulG1Payload, MulG1Return, MulG1SuccessReturn, MulG2Params, MulG2Payload, MulG2Return,
    MulG2SuccessReturn, PAIRING_INTERFACE_VERSION, PairingCurve, PairingDeclaration,
    PairingProductIsOneParams, PairingProductIsOnePayload, PairingProductIsOneReturn,
    PairingProductIsOneSuccessReturn, PrecompileEncoding, SampleUniformScalarErrorReturn,
    SampleUniformScalarParams, SampleUniformScalarPayload, SampleUniformScalarReturn,
    SampleUniformScalarSuccessReturn, VerifierGroupArithmetic,
};
use core::hint::black_box;
use domain::{Secret, SecretConstructorParams};
use halo2curves::bn256::{Bn256, Fq, Fq2, Fr, G1Affine, G2Affine};
use halo2curves::ff::{Field, FromUniformBytes, PrimeField};
use halo2curves::group::{Curve, Group, cofactor::CofactorGroup, prime::PrimeCurveAffine};
use halo2curves::msm::msm_best;
use halo2curves::pairing::{MillerLoopResult, MultiMillerLoop};
use halo2curves::{Coordinates, CurveAffine};
use interface::{
    Bn254Halo2curvesG1, Bn254Halo2curvesG2, Bn254Halo2curvesPairing,
    Bn254Halo2curvesPairingConstructorParams, Bn254Halo2curvesPairingTryNewReturn,
    Bn254Halo2curvesScalar,
};
use zeroize::Zeroize;

impl Bn254Halo2curvesPairing {
    pub const DECLARATION: PairingDeclaration = PairingDeclaration {
        curve: PairingCurve::Bn254,
        verifier_group_arithmetic: VerifierGroupArithmetic::FirstGroupOnly,
        precompile_encoding: PrecompileEncoding::Eip196Eip197,
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
    type Scalar = Bn254Halo2curvesScalar;
    type G1 = Bn254Halo2curvesG1;
    type G2 = Bn254Halo2curvesG2;

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
    ) -> EncodeG1Return {
        let mut bytes = Vec::with_capacity(64);
        match Option::<Coordinates<G1Affine>>::from(payload.point.value.coordinates()) {
            None => bytes.extend_from_slice(&[0u8; 64]),
            Some(coordinates) => {
                bytes.extend(coordinates.x().to_repr().as_ref().iter().rev());
                bytes.extend(coordinates.y().to_repr().as_ref().iter().rev());
            }
        }
        Ok(EncodeG1SuccessReturn { bytes })
    }

    fn encode_g2(
        &self,
        _params: EncodeG2Params,
        payload: EncodeG2Payload<Self::G2>,
    ) -> EncodeG2Return {
        let mut bytes = Vec::with_capacity(128);
        match Option::<Coordinates<G2Affine>>::from(payload.point.value.coordinates()) {
            None => bytes.extend_from_slice(&[0u8; 128]),
            Some(coordinates) => {
                bytes.extend(coordinates.x().c1().to_repr().as_ref().iter().rev());
                bytes.extend(coordinates.x().c0().to_repr().as_ref().iter().rev());
                bytes.extend(coordinates.y().c1().to_repr().as_ref().iter().rev());
                bytes.extend(coordinates.y().c0().to_repr().as_ref().iter().rev());
            }
        }
        Ok(EncodeG2SuccessReturn { bytes })
    }

    fn encode_scalar(
        &self,
        _params: EncodeScalarParams,
        payload: EncodeScalarPayload<Self::Scalar>,
    ) -> EncodeScalarReturn {
        let bytes: Vec<u8> = payload
            .scalar
            .value
            .to_repr()
            .as_ref()
            .iter()
            .rev()
            .copied()
            .collect();
        let Ok(bytes) = Secret::try_new(SecretConstructorParams { value: bytes });
        Ok(EncodeScalarSuccessReturn { bytes })
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
        let Ok(scalar) = Secret::try_new(SecretConstructorParams {
            value: Bn254Halo2curvesScalar { value },
        });
        Ok(SampleUniformScalarSuccessReturn { scalar })
    }
}
