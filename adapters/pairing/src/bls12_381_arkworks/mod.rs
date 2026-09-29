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
use ark_bls12_381::{Bls12_381, Fq, Fq2, Fr, G1Affine, G1Projective, G2Affine, G2Projective};
use ark_ec::{AffineRepr, CurveGroup, VariableBaseMSM, pairing::Pairing};
use ark_ff::{BigInteger, PrimeField, Zero};
use domain::{Secret, SecretConstructorParams};
use interface::{
    Bls12381ArkworksG1, Bls12381ArkworksG2, Bls12381ArkworksPairing,
    Bls12381ArkworksPairingConstructorParams, Bls12381ArkworksPairingTryNewReturn,
    Bls12381ArkworksScalar,
};
use zeroize::Zeroize;

fn decode_coordinate(bytes: &[u8]) -> Option<Fq> {
    if bytes[..16].iter().any(|byte| *byte != 0) {
        return None;
    }
    let coordinate = Fq::from_be_bytes_mod_order(&bytes[16..64]);
    if coordinate.into_bigint().to_bytes_be().as_slice() != &bytes[16..64] {
        return None;
    }
    Some(coordinate)
}

impl Bls12381ArkworksPairing {
    pub const DECLARATION: PairingDeclaration = PairingDeclaration {
        curve: PairingCurve::Bls12381,
        verifier_group_arithmetic: VerifierGroupArithmetic::BothGroups,
        precompile_encoding: PrecompileEncoding::Eip2537,
        adapter_version: 1,
        interface_version: PAIRING_INTERFACE_VERSION,
    };

    pub fn try_new(
        _params: Bls12381ArkworksPairingConstructorParams,
    ) -> Bls12381ArkworksPairingTryNewReturn {
        Ok(Bls12381ArkworksPairing)
    }
}

impl IPairingAdapter for Bls12381ArkworksPairing {
    type Scalar = Bls12381ArkworksScalar;
    type G1 = Bls12381ArkworksG1;
    type G2 = Bls12381ArkworksG2;

    fn g1_generator(
        &self,
        _params: G1GeneratorParams,
        _payload: G1GeneratorPayload,
    ) -> G1GeneratorReturn<Self::G1> {
        Ok(G1GeneratorSuccessReturn {
            point: Bls12381ArkworksG1 {
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
            point: Bls12381ArkworksG2 {
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
            sum: Bls12381ArkworksG1 {
                value: (payload.left.value + payload.right.value).into_affine(),
            },
        })
    }

    fn add_g2(
        &self,
        _params: AddG2Params,
        payload: AddG2Payload<Self::G2>,
    ) -> AddG2Return<Self::G2> {
        Ok(AddG2SuccessReturn {
            sum: Bls12381ArkworksG2 {
                value: (payload.left.value + payload.right.value).into_affine(),
            },
        })
    }

    fn mul_g1(
        &self,
        _params: MulG1Params,
        payload: MulG1Payload<Self::G1, Self::Scalar>,
    ) -> MulG1Return<Self::G1> {
        Ok(MulG1SuccessReturn {
            product: Bls12381ArkworksG1 {
                value: (payload.point.value * payload.scalar.value).into_affine(),
            },
        })
    }

    fn mul_g2(
        &self,
        _params: MulG2Params,
        payload: MulG2Payload<Self::G2, Self::Scalar>,
    ) -> MulG2Return<Self::G2> {
        Ok(MulG2SuccessReturn {
            product: Bls12381ArkworksG2 {
                value: (payload.point.value * payload.scalar.value).into_affine(),
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
        let sum = G1Projective::msm_unchecked(&bases, &scalars).into_affine();
        scalars.zeroize();
        Ok(MsmG1SuccessReturn {
            sum: Bls12381ArkworksG1 { value: sum },
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
        let sum = G2Projective::msm_unchecked(&bases, &scalars).into_affine();
        scalars.zeroize();
        Ok(MsmG2SuccessReturn {
            sum: Bls12381ArkworksG2 { value: sum },
        })
    }

    fn pairing_product_is_one(
        &self,
        _params: PairingProductIsOneParams,
        payload: PairingProductIsOnePayload<Self::G1, Self::G2>,
    ) -> PairingProductIsOneReturn {
        let (g1s, g2s): (Vec<G1Affine>, Vec<G2Affine>) = payload
            .terms
            .iter()
            .map(|term| (term.g1.value, term.g2.value))
            .unzip();
        let product = Bls12_381::multi_pairing(g1s, g2s);
        Ok(PairingProductIsOneSuccessReturn {
            is_one: product.is_zero(),
        })
    }

    fn decode_g1(&self, _params: DecodeG1Params, payload: &[u8]) -> DecodeG1Return<Self::G1> {
        if payload.len() != 128 {
            return Err(DecodeG1ErrorReturn::WrongLength {
                expected: 128,
                actual: payload.len(),
            });
        }
        let (Some(x), Some(y)) = (
            decode_coordinate(&payload[0..64]),
            decode_coordinate(&payload[64..128]),
        ) else {
            return Err(DecodeG1ErrorReturn::NonCanonicalCoordinate);
        };
        if x.is_zero() && y.is_zero() {
            return Ok(DecodeG1SuccessReturn {
                point: Bls12381ArkworksG1 {
                    value: G1Affine::identity(),
                },
            });
        }
        let point = G1Affine::new_unchecked(x, y);
        if !point.is_on_curve() {
            return Err(DecodeG1ErrorReturn::NotOnCurve);
        }
        if !point.is_in_correct_subgroup_assuming_on_curve() {
            return Err(DecodeG1ErrorReturn::NotInSubgroup);
        }
        Ok(DecodeG1SuccessReturn {
            point: Bls12381ArkworksG1 { value: point },
        })
    }

    fn decode_g2(&self, _params: DecodeG2Params, payload: &[u8]) -> DecodeG2Return<Self::G2> {
        if payload.len() != 256 {
            return Err(DecodeG2ErrorReturn::WrongLength {
                expected: 256,
                actual: payload.len(),
            });
        }
        let (Some(x_c0), Some(x_c1), Some(y_c0), Some(y_c1)) = (
            decode_coordinate(&payload[0..64]),
            decode_coordinate(&payload[64..128]),
            decode_coordinate(&payload[128..192]),
            decode_coordinate(&payload[192..256]),
        ) else {
            return Err(DecodeG2ErrorReturn::NonCanonicalCoordinate);
        };
        if x_c0.is_zero() && x_c1.is_zero() && y_c0.is_zero() && y_c1.is_zero() {
            return Ok(DecodeG2SuccessReturn {
                point: Bls12381ArkworksG2 {
                    value: G2Affine::identity(),
                },
            });
        }
        let point = G2Affine::new_unchecked(Fq2::new(x_c0, x_c1), Fq2::new(y_c0, y_c1));
        if !point.is_on_curve() {
            return Err(DecodeG2ErrorReturn::NotOnCurve);
        }
        if !point.is_in_correct_subgroup_assuming_on_curve() {
            return Err(DecodeG2ErrorReturn::NotInSubgroup);
        }
        Ok(DecodeG2SuccessReturn {
            point: Bls12381ArkworksG2 { value: point },
        })
    }

    fn decode_scalar(
        &self,
        _params: DecodeScalarParams,
        payload: &[u8],
    ) -> DecodeScalarReturn<Self::Scalar> {
        if payload.len() != 32 {
            return Err(DecodeScalarErrorReturn::WrongLength {
                expected: 32,
                actual: payload.len(),
            });
        }
        let scalar = Fr::from_be_bytes_mod_order(payload);
        if scalar.into_bigint().to_bytes_be().as_slice() != payload {
            return Err(DecodeScalarErrorReturn::NonCanonical);
        }
        Ok(DecodeScalarSuccessReturn {
            scalar: Bls12381ArkworksScalar { value: scalar },
        })
    }

    fn encode_g1(
        &self,
        _params: EncodeG1Params,
        payload: EncodeG1Payload<Self::G1>,
    ) -> EncodeG1Return {
        let mut bytes = Vec::with_capacity(128);
        match payload.point.value.xy() {
            None => bytes.extend_from_slice(&[0u8; 128]),
            Some((x, y)) => {
                bytes.extend_from_slice(&[0u8; 16]);
                bytes.extend_from_slice(&x.into_bigint().to_bytes_be());
                bytes.extend_from_slice(&[0u8; 16]);
                bytes.extend_from_slice(&y.into_bigint().to_bytes_be());
            }
        }
        Ok(EncodeG1SuccessReturn { bytes })
    }

    fn encode_g2(
        &self,
        _params: EncodeG2Params,
        payload: EncodeG2Payload<Self::G2>,
    ) -> EncodeG2Return {
        let mut bytes = Vec::with_capacity(256);
        match payload.point.value.xy() {
            None => bytes.extend_from_slice(&[0u8; 256]),
            Some((x, y)) => {
                bytes.extend_from_slice(&[0u8; 16]);
                bytes.extend_from_slice(&x.c0.into_bigint().to_bytes_be());
                bytes.extend_from_slice(&[0u8; 16]);
                bytes.extend_from_slice(&x.c1.into_bigint().to_bytes_be());
                bytes.extend_from_slice(&[0u8; 16]);
                bytes.extend_from_slice(&y.c0.into_bigint().to_bytes_be());
                bytes.extend_from_slice(&[0u8; 16]);
                bytes.extend_from_slice(&y.c1.into_bigint().to_bytes_be());
            }
        }
        Ok(EncodeG2SuccessReturn { bytes })
    }

    fn encode_scalar(
        &self,
        _params: EncodeScalarParams,
        payload: EncodeScalarPayload<Self::Scalar>,
    ) -> EncodeScalarReturn {
        let Ok(bytes) = Secret::try_new(SecretConstructorParams {
            value: payload.scalar.value.into_bigint().to_bytes_be(),
        });
        Ok(EncodeScalarSuccessReturn { bytes })
    }
}

impl Zeroize for Bls12381ArkworksScalar {
    fn zeroize(&mut self) {
        self.value.zeroize();
    }
}

impl Drop for Bls12381ArkworksScalar {
    fn drop(&mut self) {
        self.value.zeroize();
    }
}

impl ISampleUniformScalar for Bls12381ArkworksScalar {
    const UNIFORM_BYTES_LENGTH: usize = 64;

    fn sample_from_uniform_bytes(
        _params: SampleUniformScalarParams,
        payload: SampleUniformScalarPayload,
    ) -> SampleUniformScalarReturn<Self> {
        let actual = payload.uniform.expose().len();
        if actual != Self::UNIFORM_BYTES_LENGTH {
            return Err(SampleUniformScalarErrorReturn::WrongLength {
                expected: Self::UNIFORM_BYTES_LENGTH,
                actual,
            });
        }
        let Ok(scalar) = Secret::try_new(SecretConstructorParams {
            value: Bls12381ArkworksScalar {
                value: Fr::from_be_bytes_mod_order(payload.uniform.expose()),
            },
        });
        Ok(SampleUniformScalarSuccessReturn { scalar })
    }
}
