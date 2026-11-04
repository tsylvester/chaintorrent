#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use core::num::NonZeroU32;
use encoding::{
    CanonicalFieldKind, CanonicalFieldReader, CanonicalFieldValue, CanonicalFields,
    FromFieldsParams, FromFieldsReturn, FromFieldsSuccessReturn, IEncodingContract, ToFieldsParams,
    ToFieldsReturn, ToFieldsSuccessReturn,
};
use pairing::{
    AddG1Params, AddG1Payload, AddG2Params, AddG2Payload, EncodeG1Params, EncodeG1Payload,
    EncodeG2Params, EncodeG2Payload, EncodeScalarParams, EncodeScalarPayload, G1GeneratorParams,
    G1GeneratorPayload, G2GeneratorParams, G2GeneratorPayload, IPairingArithmetic,
    ISampleUniformScalar, MsmG1Params, MsmG1Payload, MsmG1Term, MsmG2Params, MsmG2Payload,
    MsmG2Term, MulG1Params, MulG1Payload, MulG2Params, MulG2Payload, MulScalarParams,
    MulScalarPayload, NegScalarParams, NegScalarPayload, PairingProductIsOneParams,
    PairingProductIsOnePayload, PairingProductTerm, SampleUniformScalarParams,
    SampleUniformScalarPayload, VerifierGroupArithmetic,
};
use random::{FillBytesParams, FillBytesPayload};

use self::interface::{
    FirstGroupVectors, GROUP_OPERATION_VECTOR_FIELD_KINDS, GroupOperationVector,
    GroupOperationVectorDescription, GroupOperationVectorDescriptionConstructorParams,
    GroupOperationVectorDescriptionTryNewReturn, GroupOperationVectorFromFieldsErrorReturn,
    GroupVectorsDeps, GroupVectorsErrorReturn, GroupVectorsParams, GroupVectorsPayload,
    GroupVectorsReturn, GroupVectorsSuccessReturn, MsmTermCount, MsmTermCountConstructorParams,
    MsmTermCountTryNewErrorReturn, MsmTermCountTryNewReturn, PAIRING_CHECK_VECTOR_FIELD_KINDS,
    PairingCheckVector, PairingCheckVectorDescription,
    PairingCheckVectorDescriptionConstructorParams, PairingCheckVectorDescriptionTryNewReturn,
    PairingCheckVectorFromFieldsErrorReturn, SecondGroupVectors, VECTOR_RECORD_FIELD_COUNT,
    VECTOR_RECORD_MEMBER_NAMES, VectorCount, VectorCountConstructorParams,
    VectorCountTryNewErrorReturn, VectorCountTryNewReturn, VectorRecordMemberNamesErrorReturn,
    VectorRecordMemberNamesReturn,
};
use super::render::provides::{SolidityMemberName, SolidityMemberNameConstructorParams};

impl VectorCount {
    pub fn try_new(params: VectorCountConstructorParams) -> VectorCountTryNewReturn {
        match NonZeroU32::new(params.count) {
            Some(count) => Ok(VectorCount { count }),
            None => Err(VectorCountTryNewErrorReturn::Zero),
        }
    }

    pub fn get(&self) -> u32 {
        self.count.get()
    }
}

impl MsmTermCount {
    pub fn try_new(params: MsmTermCountConstructorParams) -> MsmTermCountTryNewReturn {
        match NonZeroU32::new(params.count) {
            Some(count) => Ok(MsmTermCount { count }),
            None => Err(MsmTermCountTryNewErrorReturn::Zero),
        }
    }

    pub fn get(&self) -> u32 {
        self.count.get()
    }
}

impl GroupOperationVectorDescription {
    pub fn try_new(
        _params: GroupOperationVectorDescriptionConstructorParams,
    ) -> GroupOperationVectorDescriptionTryNewReturn {
        Ok(GroupOperationVectorDescription)
    }

    pub fn member_names(&self) -> VectorRecordMemberNamesReturn {
        let mut names = Vec::with_capacity(VECTOR_RECORD_FIELD_COUNT);
        for text in VECTOR_RECORD_MEMBER_NAMES {
            match SolidityMemberName::try_new(SolidityMemberNameConstructorParams {
                text: text.to_string(),
            }) {
                Ok(name) => names.push(name),
                Err(error) => {
                    return Err(VectorRecordMemberNamesErrorReturn::MemberName(error));
                }
            }
        }
        Ok(names)
    }
}

impl PairingCheckVectorDescription {
    pub fn try_new(
        _params: PairingCheckVectorDescriptionConstructorParams,
    ) -> PairingCheckVectorDescriptionTryNewReturn {
        Ok(PairingCheckVectorDescription)
    }

    pub fn member_names(&self) -> VectorRecordMemberNamesReturn {
        let mut names = Vec::with_capacity(VECTOR_RECORD_FIELD_COUNT);
        for text in VECTOR_RECORD_MEMBER_NAMES {
            match SolidityMemberName::try_new(SolidityMemberNameConstructorParams {
                text: text.to_string(),
            }) {
                Ok(name) => names.push(name),
                Err(error) => {
                    return Err(VectorRecordMemberNamesErrorReturn::MemberName(error));
                }
            }
        }
        Ok(names)
    }
}

impl IEncodingContract for GroupOperationVectorDescription {
    type Described = GroupOperationVector;
    type FromFieldsErrorReturn = GroupOperationVectorFromFieldsErrorReturn;
    const FIELDS: &'static [CanonicalFieldKind] = &GROUP_OPERATION_VECTOR_FIELD_KINDS;

    fn to_fields(&self, _params: ToFieldsParams, payload: &Self::Described) -> ToFieldsReturn {
        Ok(ToFieldsSuccessReturn {
            fields: CanonicalFields {
                values: vec![
                    CanonicalFieldValue::Bytes(payload.draws.clone()),
                    CanonicalFieldValue::Bytes(payload.input.clone()),
                    CanonicalFieldValue::Bytes(payload.output.clone()),
                ],
            },
        })
    }

    fn fields_to_value(
        &self,
        _params: FromFieldsParams,
        payload: CanonicalFields,
    ) -> FromFieldsReturn<Self::Described, Self::FromFieldsErrorReturn> {
        let mut reader = CanonicalFieldReader::try_new(payload, Self::FIELDS)
            .map_err(GroupOperationVectorFromFieldsErrorReturn::Fields)?;
        let draws = reader
            .read_bytes()
            .map_err(GroupOperationVectorFromFieldsErrorReturn::Fields)?;
        let input = reader
            .read_bytes()
            .map_err(GroupOperationVectorFromFieldsErrorReturn::Fields)?;
        let output = reader
            .read_bytes()
            .map_err(GroupOperationVectorFromFieldsErrorReturn::Fields)?;
        Ok(FromFieldsSuccessReturn {
            described: GroupOperationVector {
                draws,
                input,
                output,
            },
        })
    }
}

impl IEncodingContract for PairingCheckVectorDescription {
    type Described = PairingCheckVector;
    type FromFieldsErrorReturn = PairingCheckVectorFromFieldsErrorReturn;
    const FIELDS: &'static [CanonicalFieldKind] = &PAIRING_CHECK_VECTOR_FIELD_KINDS;

    fn to_fields(&self, _params: ToFieldsParams, payload: &Self::Described) -> ToFieldsReturn {
        Ok(ToFieldsSuccessReturn {
            fields: CanonicalFields {
                values: vec![
                    CanonicalFieldValue::Bytes(payload.draws.clone()),
                    CanonicalFieldValue::Bytes(payload.input.clone()),
                    CanonicalFieldValue::FixedBytes32(payload.output),
                ],
            },
        })
    }

    fn fields_to_value(
        &self,
        _params: FromFieldsParams,
        payload: CanonicalFields,
    ) -> FromFieldsReturn<Self::Described, Self::FromFieldsErrorReturn> {
        let mut reader = CanonicalFieldReader::try_new(payload, Self::FIELDS)
            .map_err(PairingCheckVectorFromFieldsErrorReturn::Fields)?;
        let draws = reader
            .read_bytes()
            .map_err(PairingCheckVectorFromFieldsErrorReturn::Fields)?;
        let input = reader
            .read_bytes()
            .map_err(PairingCheckVectorFromFieldsErrorReturn::Fields)?;
        let output = reader
            .read_fixed_bytes32()
            .map_err(PairingCheckVectorFromFieldsErrorReturn::Fields)?;
        Ok(FromFieldsSuccessReturn {
            described: PairingCheckVector {
                draws,
                input,
                output,
            },
        })
    }
}

pub fn group_vectors<'a, P: IPairingArithmetic>(
    deps: &GroupVectorsDeps<'a, P>,
    params: GroupVectorsParams,
    _payload: GroupVectorsPayload,
) -> GroupVectorsReturn {
    let settings = match (
        P::DECLARATION.verifier_group_arithmetic,
        params.second_group,
    ) {
        (VerifierGroupArithmetic::FirstGroupOnly, Some(_)) => {
            return Err(GroupVectorsErrorReturn::SecondGroupSettingsForFirstGroupOnly);
        }
        (VerifierGroupArithmetic::BothGroups, None) => {
            return Err(GroupVectorsErrorReturn::SecondGroupSettingsMissing);
        }
        (VerifierGroupArithmetic::FirstGroupOnly, None) => None,
        (VerifierGroupArithmetic::BothGroups, Some(settings)) => Some(settings),
    };

    let draw = |draws: &mut Vec<u8>| -> Result<P::Scalar, GroupVectorsErrorReturn> {
        let uniform = match deps.random.fill_bytes(
            FillBytesParams,
            FillBytesPayload {
                length: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
            },
        ) {
            Ok(uniform) => uniform,
            Err(error) => return Err(GroupVectorsErrorReturn::FillBytes(error)),
        };
        let sampled = match <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
            SampleUniformScalarParams,
            SampleUniformScalarPayload {
                uniform: uniform.bytes,
            },
        ) {
            Ok(sampled) => sampled,
            Err(error) => return Err(GroupVectorsErrorReturn::SampleScalar(error)),
        };
        let scalar = sampled.scalar.expose().clone();
        let Ok(encoded) = deps.pairing.encode_scalar(
            EncodeScalarParams,
            EncodeScalarPayload {
                scalar: scalar.clone(),
            },
        );
        draws.extend_from_slice(encoded.bytes.expose().as_ref());
        Ok(scalar)
    };

    let Ok(g1) = deps
        .pairing
        .g1_generator(G1GeneratorParams, G1GeneratorPayload);
    let Ok(g2) = deps
        .pairing
        .g2_generator(G2GeneratorParams, G2GeneratorPayload);

    let mut g1_add = Vec::new();
    for _ in 0..params.first_group.g1_add.get() {
        let mut draws = Vec::new();
        let a = draw(&mut draws)?;
        let b = draw(&mut draws)?;
        let Ok(left) = deps.pairing.mul_g1(
            MulG1Params,
            MulG1Payload {
                point: g1.point.clone(),
                scalar: a,
            },
        );
        let Ok(right) = deps.pairing.mul_g1(
            MulG1Params,
            MulG1Payload {
                point: g1.point.clone(),
                scalar: b,
            },
        );
        let mut input = Vec::new();
        let Ok(encoded_left) = deps.pairing.encode_g1(
            EncodeG1Params,
            EncodeG1Payload {
                point: left.product.clone(),
            },
        );
        input.extend_from_slice(encoded_left.bytes.as_ref());
        let Ok(encoded_right) = deps.pairing.encode_g1(
            EncodeG1Params,
            EncodeG1Payload {
                point: right.product.clone(),
            },
        );
        input.extend_from_slice(encoded_right.bytes.as_ref());
        let Ok(sum) = deps.pairing.add_g1(
            AddG1Params,
            AddG1Payload {
                left: left.product,
                right: right.product,
            },
        );
        let Ok(output) = deps
            .pairing
            .encode_g1(EncodeG1Params, EncodeG1Payload { point: sum.sum });
        g1_add.push(GroupOperationVector {
            draws,
            input,
            output: output.bytes.as_ref().to_vec(),
        });
    }

    let mut g1_mul = Vec::new();
    for _ in 0..params.first_group.g1_mul.get() {
        let mut draws = Vec::new();
        let a = draw(&mut draws)?;
        let s = draw(&mut draws)?;
        let Ok(point) = deps.pairing.mul_g1(
            MulG1Params,
            MulG1Payload {
                point: g1.point.clone(),
                scalar: a,
            },
        );
        let Ok(product) = deps.pairing.mul_g1(
            MulG1Params,
            MulG1Payload {
                point: point.product.clone(),
                scalar: s.clone(),
            },
        );
        let mut input = Vec::new();
        let Ok(encoded_point) = deps.pairing.encode_g1(
            EncodeG1Params,
            EncodeG1Payload {
                point: point.product,
            },
        );
        input.extend_from_slice(encoded_point.bytes.as_ref());
        let Ok(encoded_s) = deps
            .pairing
            .encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar: s });
        input.extend_from_slice(encoded_s.bytes.expose().as_ref());
        let Ok(output) = deps.pairing.encode_g1(
            EncodeG1Params,
            EncodeG1Payload {
                point: product.product,
            },
        );
        g1_mul.push(GroupOperationVector {
            draws,
            input,
            output: output.bytes.as_ref().to_vec(),
        });
    }

    let mut pairing_check = Vec::new();
    for index in 0..params.first_group.pairing_check.get() {
        let mut draws = Vec::new();
        let a = draw(&mut draws)?;
        let b = draw(&mut draws)?;
        let t = if index % 2 == 0 {
            let Ok(product) = deps.pairing.mul_scalar(
                MulScalarParams,
                MulScalarPayload {
                    left: a.clone(),
                    right: b.clone(),
                },
            );
            let Ok(negation) = deps.pairing.neg_scalar(
                NegScalarParams,
                NegScalarPayload {
                    scalar: product.product,
                },
            );
            negation.negation
        } else {
            draw(&mut draws)?
        };
        let Ok(ag1) = deps.pairing.mul_g1(
            MulG1Params,
            MulG1Payload {
                point: g1.point.clone(),
                scalar: a,
            },
        );
        let Ok(bg2) = deps.pairing.mul_g2(
            MulG2Params,
            MulG2Payload {
                point: g2.point.clone(),
                scalar: b,
            },
        );
        let Ok(tg1) = deps.pairing.mul_g1(
            MulG1Params,
            MulG1Payload {
                point: g1.point.clone(),
                scalar: t,
            },
        );
        let Ok(verdict) = deps.pairing.pairing_product_is_one(
            PairingProductIsOneParams,
            PairingProductIsOnePayload {
                terms: vec![
                    PairingProductTerm {
                        g1: ag1.product.clone(),
                        g2: bg2.product.clone(),
                    },
                    PairingProductTerm {
                        g1: tg1.product.clone(),
                        g2: g2.point.clone(),
                    },
                ],
            },
        );
        let mut input = Vec::new();
        let Ok(encoded_ag1) = deps
            .pairing
            .encode_g1(EncodeG1Params, EncodeG1Payload { point: ag1.product });
        input.extend_from_slice(encoded_ag1.bytes.as_ref());
        let Ok(encoded_bg2) = deps
            .pairing
            .encode_g2(EncodeG2Params, EncodeG2Payload { point: bg2.product });
        input.extend_from_slice(encoded_bg2.bytes.as_ref());
        let Ok(encoded_tg1) = deps
            .pairing
            .encode_g1(EncodeG1Params, EncodeG1Payload { point: tg1.product });
        input.extend_from_slice(encoded_tg1.bytes.as_ref());
        let Ok(encoded_g2) = deps.pairing.encode_g2(
            EncodeG2Params,
            EncodeG2Payload {
                point: g2.point.clone(),
            },
        );
        input.extend_from_slice(encoded_g2.bytes.as_ref());
        let mut output = [0x00; 32];
        if verdict.is_one {
            output[31] = 0x01;
        }
        pairing_check.push(PairingCheckVector {
            draws,
            input,
            output,
        });
    }

    let second_group = match settings {
        Some(settings) => {
            let mut g2_add = Vec::new();
            for _ in 0..settings.g2_add.get() {
                let mut draws = Vec::new();
                let a = draw(&mut draws)?;
                let b = draw(&mut draws)?;
                let Ok(left) = deps.pairing.mul_g2(
                    MulG2Params,
                    MulG2Payload {
                        point: g2.point.clone(),
                        scalar: a,
                    },
                );
                let Ok(right) = deps.pairing.mul_g2(
                    MulG2Params,
                    MulG2Payload {
                        point: g2.point.clone(),
                        scalar: b,
                    },
                );
                let mut input = Vec::new();
                let Ok(encoded_left) = deps.pairing.encode_g2(
                    EncodeG2Params,
                    EncodeG2Payload {
                        point: left.product.clone(),
                    },
                );
                input.extend_from_slice(encoded_left.bytes.as_ref());
                let Ok(encoded_right) = deps.pairing.encode_g2(
                    EncodeG2Params,
                    EncodeG2Payload {
                        point: right.product.clone(),
                    },
                );
                input.extend_from_slice(encoded_right.bytes.as_ref());
                let Ok(sum) = deps.pairing.add_g2(
                    AddG2Params,
                    AddG2Payload {
                        left: left.product,
                        right: right.product,
                    },
                );
                let Ok(output) = deps
                    .pairing
                    .encode_g2(EncodeG2Params, EncodeG2Payload { point: sum.sum });
                g2_add.push(GroupOperationVector {
                    draws,
                    input,
                    output: output.bytes.as_ref().to_vec(),
                });
            }
            let mut g1_msm = Vec::new();
            for _ in 0..settings.g1_msm.count.get() {
                let mut draws = Vec::new();
                let mut input = Vec::new();
                let mut terms = Vec::new();
                for _ in 0..settings.g1_msm.terms.get() {
                    let a_i = draw(&mut draws)?;
                    let s_i = draw(&mut draws)?;
                    let Ok(base) = deps.pairing.mul_g1(
                        MulG1Params,
                        MulG1Payload {
                            point: g1.point.clone(),
                            scalar: a_i,
                        },
                    );
                    let Ok(encoded_base) = deps.pairing.encode_g1(
                        EncodeG1Params,
                        EncodeG1Payload {
                            point: base.product.clone(),
                        },
                    );
                    input.extend_from_slice(encoded_base.bytes.as_ref());
                    let Ok(encoded_s) = deps.pairing.encode_scalar(
                        EncodeScalarParams,
                        EncodeScalarPayload {
                            scalar: s_i.clone(),
                        },
                    );
                    input.extend_from_slice(encoded_s.bytes.expose().as_ref());
                    terms.push(MsmG1Term {
                        base: base.product,
                        scalar: s_i,
                    });
                }
                let Ok(sum) = deps.pairing.msm_g1(MsmG1Params, MsmG1Payload { terms });
                let Ok(output) = deps
                    .pairing
                    .encode_g1(EncodeG1Params, EncodeG1Payload { point: sum.sum });
                g1_msm.push(GroupOperationVector {
                    draws,
                    input,
                    output: output.bytes.as_ref().to_vec(),
                });
            }
            let mut g2_msm = Vec::new();
            for _ in 0..settings.g2_msm.count.get() {
                let mut draws = Vec::new();
                let mut input = Vec::new();
                let mut terms = Vec::new();
                for _ in 0..settings.g2_msm.terms.get() {
                    let a_i = draw(&mut draws)?;
                    let s_i = draw(&mut draws)?;
                    let Ok(base) = deps.pairing.mul_g2(
                        MulG2Params,
                        MulG2Payload {
                            point: g2.point.clone(),
                            scalar: a_i,
                        },
                    );
                    let Ok(encoded_base) = deps.pairing.encode_g2(
                        EncodeG2Params,
                        EncodeG2Payload {
                            point: base.product.clone(),
                        },
                    );
                    input.extend_from_slice(encoded_base.bytes.as_ref());
                    let Ok(encoded_s) = deps.pairing.encode_scalar(
                        EncodeScalarParams,
                        EncodeScalarPayload {
                            scalar: s_i.clone(),
                        },
                    );
                    input.extend_from_slice(encoded_s.bytes.expose().as_ref());
                    terms.push(MsmG2Term {
                        base: base.product,
                        scalar: s_i,
                    });
                }
                let Ok(sum) = deps.pairing.msm_g2(MsmG2Params, MsmG2Payload { terms });
                let Ok(output) = deps
                    .pairing
                    .encode_g2(EncodeG2Params, EncodeG2Payload { point: sum.sum });
                g2_msm.push(GroupOperationVector {
                    draws,
                    input,
                    output: output.bytes.as_ref().to_vec(),
                });
            }
            Some(SecondGroupVectors {
                g2_add,
                g1_msm,
                g2_msm,
            })
        }
        None => None,
    };

    Ok(GroupVectorsSuccessReturn {
        first_group: FirstGroupVectors {
            g1_add,
            g1_mul,
            pairing_check,
        },
        second_group,
    })
}
