mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use core::num::NonZeroU32;
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFields, FromFieldsParams, FromFieldsReturn,
    FromFieldsSuccessReturn, IEncodingContract, ToFieldsParams, ToFieldsReturn,
    ToFieldsSuccessReturn,
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

use super::render::provides::{
    SolidityMemberName, SolidityMemberNameConstructorParams, SolidityRecordMember,
    SolidityRecordMembers, SolidityRecordMembersConstructorParams,
};
use interface::{
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
    VectorCountTryNewErrorReturn, VectorCountTryNewReturn, VectorRecordMembersErrorReturn,
    VectorRecordMembersReturn,
};

impl VectorCount {
    pub fn try_new(params: VectorCountConstructorParams) -> VectorCountTryNewReturn {
        let Some(count) = NonZeroU32::new(params.count) else {
            return Err(VectorCountTryNewErrorReturn::Zero);
        };
        Ok(VectorCount { count })
    }

    pub fn get(&self) -> u32 {
        self.count.get()
    }
}

impl MsmTermCount {
    pub fn try_new(params: MsmTermCountConstructorParams) -> MsmTermCountTryNewReturn {
        let Some(count) = NonZeroU32::new(params.count) else {
            return Err(MsmTermCountTryNewErrorReturn::Zero);
        };
        Ok(MsmTermCount { count })
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

    pub fn members(&self) -> VectorRecordMembersReturn {
        let mut members = Vec::new();
        for (index, text) in VECTOR_RECORD_MEMBER_NAMES.iter().enumerate() {
            let name = SolidityMemberName::try_new(SolidityMemberNameConstructorParams {
                text: text.to_string(),
            })
            .map_err(VectorRecordMembersErrorReturn::MemberName)?;
            members.push(SolidityRecordMember {
                name,
                kind: GROUP_OPERATION_VECTOR_FIELD_KINDS[index],
            });
        }
        SolidityRecordMembers::try_new(SolidityRecordMembersConstructorParams { members })
            .map_err(VectorRecordMembersErrorReturn::RecordMembers)
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
        let actual = payload.values.len();
        let Ok(fields) =
            <[CanonicalFieldValue; VECTOR_RECORD_FIELD_COUNT]>::try_from(payload.values)
        else {
            return Err(GroupOperationVectorFromFieldsErrorReturn::FieldCount {
                expected: VECTOR_RECORD_FIELD_COUNT,
                actual,
            });
        };
        let [draws, input, output] = fields;
        let CanonicalFieldValue::Bytes(draws) = draws else {
            return Err(GroupOperationVectorFromFieldsErrorReturn::FieldKind {
                index: 0,
                expected: CanonicalFieldKind::Bytes,
            });
        };
        let CanonicalFieldValue::Bytes(input) = input else {
            return Err(GroupOperationVectorFromFieldsErrorReturn::FieldKind {
                index: 1,
                expected: CanonicalFieldKind::Bytes,
            });
        };
        let CanonicalFieldValue::Bytes(output) = output else {
            return Err(GroupOperationVectorFromFieldsErrorReturn::FieldKind {
                index: 2,
                expected: CanonicalFieldKind::Bytes,
            });
        };
        Ok(FromFieldsSuccessReturn {
            described: GroupOperationVector {
                draws,
                input,
                output,
            },
        })
    }
}

impl PairingCheckVectorDescription {
    pub fn try_new(
        _params: PairingCheckVectorDescriptionConstructorParams,
    ) -> PairingCheckVectorDescriptionTryNewReturn {
        Ok(PairingCheckVectorDescription)
    }

    pub fn members(&self) -> VectorRecordMembersReturn {
        let mut members = Vec::new();
        for (index, text) in VECTOR_RECORD_MEMBER_NAMES.iter().enumerate() {
            let name = SolidityMemberName::try_new(SolidityMemberNameConstructorParams {
                text: text.to_string(),
            })
            .map_err(VectorRecordMembersErrorReturn::MemberName)?;
            members.push(SolidityRecordMember {
                name,
                kind: PAIRING_CHECK_VECTOR_FIELD_KINDS[index],
            });
        }
        SolidityRecordMembers::try_new(SolidityRecordMembersConstructorParams { members })
            .map_err(VectorRecordMembersErrorReturn::RecordMembers)
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
        let actual = payload.values.len();
        let Ok(fields) =
            <[CanonicalFieldValue; VECTOR_RECORD_FIELD_COUNT]>::try_from(payload.values)
        else {
            return Err(PairingCheckVectorFromFieldsErrorReturn::FieldCount {
                expected: VECTOR_RECORD_FIELD_COUNT,
                actual,
            });
        };
        let [draws, input, output] = fields;
        let CanonicalFieldValue::Bytes(draws) = draws else {
            return Err(PairingCheckVectorFromFieldsErrorReturn::FieldKind {
                index: 0,
                expected: PAIRING_CHECK_VECTOR_FIELD_KINDS[0],
            });
        };
        let CanonicalFieldValue::Bytes(input) = input else {
            return Err(PairingCheckVectorFromFieldsErrorReturn::FieldKind {
                index: 1,
                expected: PAIRING_CHECK_VECTOR_FIELD_KINDS[1],
            });
        };
        let CanonicalFieldValue::FixedBytes32(output) = output else {
            return Err(PairingCheckVectorFromFieldsErrorReturn::FieldKind {
                index: 2,
                expected: PAIRING_CHECK_VECTOR_FIELD_KINDS[2],
            });
        };
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

    let draw_scalar = |draws: &mut Vec<u8>| -> Result<P::Scalar, GroupVectorsErrorReturn> {
        let filled = deps
            .random
            .fill_bytes(
                FillBytesParams,
                FillBytesPayload {
                    length: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                },
            )
            .map_err(GroupVectorsErrorReturn::FillBytes)?;
        let sampled = <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
            SampleUniformScalarParams,
            SampleUniformScalarPayload {
                uniform: filled.bytes,
            },
        )
        .map_err(GroupVectorsErrorReturn::SampleScalar)?;
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
    let g1 = g1.point;
    let Ok(g2) = deps
        .pairing
        .g2_generator(G2GeneratorParams, G2GeneratorPayload);
    let g2 = g2.point;

    let encode_g1 = |point: P::G1| -> Vec<u8> {
        let Ok(encoded) = deps
            .pairing
            .encode_g1(EncodeG1Params, EncodeG1Payload { point });
        encoded.bytes.as_ref().to_vec()
    };
    let encode_g2 = |point: P::G2| -> Vec<u8> {
        let Ok(encoded) = deps
            .pairing
            .encode_g2(EncodeG2Params, EncodeG2Payload { point });
        encoded.bytes.as_ref().to_vec()
    };
    let encode_scalar = |scalar: P::Scalar| -> Vec<u8> {
        let Ok(encoded) = deps
            .pairing
            .encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar });
        encoded.bytes.expose().as_ref().to_vec()
    };

    let mut g1_add = Vec::new();
    for _ in 0..params.first_group.g1_add.get() {
        let mut draws = Vec::new();
        let a = draw_scalar(&mut draws)?;
        let b = draw_scalar(&mut draws)?;
        let Ok(left) = deps.pairing.mul_g1(
            MulG1Params,
            MulG1Payload {
                point: g1.clone(),
                scalar: a,
            },
        );
        let Ok(right) = deps.pairing.mul_g1(
            MulG1Params,
            MulG1Payload {
                point: g1.clone(),
                scalar: b,
            },
        );
        let Ok(sum) = deps.pairing.add_g1(
            AddG1Params,
            AddG1Payload {
                left: left.product.clone(),
                right: right.product.clone(),
            },
        );
        let mut input = encode_g1(left.product);
        input.extend_from_slice(&encode_g1(right.product));
        g1_add.push(GroupOperationVector {
            draws,
            input,
            output: encode_g1(sum.sum),
        });
    }

    let mut g1_mul = Vec::new();
    for _ in 0..params.first_group.g1_mul.get() {
        let mut draws = Vec::new();
        let a = draw_scalar(&mut draws)?;
        let s = draw_scalar(&mut draws)?;
        let Ok(point) = deps.pairing.mul_g1(
            MulG1Params,
            MulG1Payload {
                point: g1.clone(),
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
        let mut input = encode_g1(point.product);
        input.extend_from_slice(&encode_scalar(s));
        g1_mul.push(GroupOperationVector {
            draws,
            input,
            output: encode_g1(product.product),
        });
    }

    let mut pairing_check = Vec::new();
    for index in 0..params.first_group.pairing_check.get() {
        let mut draws = Vec::new();
        let a = draw_scalar(&mut draws)?;
        let b = draw_scalar(&mut draws)?;
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
            draw_scalar(&mut draws)?
        };
        let Ok(a_g1) = deps.pairing.mul_g1(
            MulG1Params,
            MulG1Payload {
                point: g1.clone(),
                scalar: a,
            },
        );
        let Ok(b_g2) = deps.pairing.mul_g2(
            MulG2Params,
            MulG2Payload {
                point: g2.clone(),
                scalar: b,
            },
        );
        let Ok(t_g1) = deps.pairing.mul_g1(
            MulG1Params,
            MulG1Payload {
                point: g1.clone(),
                scalar: t,
            },
        );
        let Ok(is_one) = deps.pairing.pairing_product_is_one(
            PairingProductIsOneParams,
            PairingProductIsOnePayload {
                terms: vec![
                    PairingProductTerm {
                        g1: a_g1.product.clone(),
                        g2: b_g2.product.clone(),
                    },
                    PairingProductTerm {
                        g1: t_g1.product.clone(),
                        g2: g2.clone(),
                    },
                ],
            },
        );
        let mut input = encode_g1(a_g1.product);
        input.extend_from_slice(&encode_g2(b_g2.product));
        input.extend_from_slice(&encode_g1(t_g1.product));
        input.extend_from_slice(&encode_g2(g2.clone()));
        let mut output = [0x00; 32];
        output[31] = if is_one.is_one { 0x01 } else { 0x00 };
        pairing_check.push(PairingCheckVector {
            draws,
            input,
            output,
        });
    }

    let second_group = if let Some(settings) = settings {
        let mut g2_add = Vec::new();
        for _ in 0..settings.g2_add.get() {
            let mut draws = Vec::new();
            let a = draw_scalar(&mut draws)?;
            let b = draw_scalar(&mut draws)?;
            let Ok(left) = deps.pairing.mul_g2(
                MulG2Params,
                MulG2Payload {
                    point: g2.clone(),
                    scalar: a,
                },
            );
            let Ok(right) = deps.pairing.mul_g2(
                MulG2Params,
                MulG2Payload {
                    point: g2.clone(),
                    scalar: b,
                },
            );
            let Ok(sum) = deps.pairing.add_g2(
                AddG2Params,
                AddG2Payload {
                    left: left.product.clone(),
                    right: right.product.clone(),
                },
            );
            let mut input = encode_g2(left.product);
            input.extend_from_slice(&encode_g2(right.product));
            g2_add.push(GroupOperationVector {
                draws,
                input,
                output: encode_g2(sum.sum),
            });
        }

        let mut g1_msm = Vec::new();
        for _ in 0..settings.g1_msm.count.get() {
            let mut draws = Vec::new();
            let mut input = Vec::new();
            let mut terms = Vec::new();
            for _ in 0..settings.g1_msm.terms.get() {
                let a = draw_scalar(&mut draws)?;
                let s = draw_scalar(&mut draws)?;
                let Ok(base) = deps.pairing.mul_g1(
                    MulG1Params,
                    MulG1Payload {
                        point: g1.clone(),
                        scalar: a,
                    },
                );
                input.extend_from_slice(&encode_g1(base.product.clone()));
                input.extend_from_slice(&encode_scalar(s.clone()));
                terms.push(MsmG1Term {
                    base: base.product,
                    scalar: s,
                });
            }
            let Ok(sum) = deps.pairing.msm_g1(MsmG1Params, MsmG1Payload { terms });
            g1_msm.push(GroupOperationVector {
                draws,
                input,
                output: encode_g1(sum.sum),
            });
        }

        let mut g2_msm = Vec::new();
        for _ in 0..settings.g2_msm.count.get() {
            let mut draws = Vec::new();
            let mut input = Vec::new();
            let mut terms = Vec::new();
            for _ in 0..settings.g2_msm.terms.get() {
                let a = draw_scalar(&mut draws)?;
                let s = draw_scalar(&mut draws)?;
                let Ok(base) = deps.pairing.mul_g2(
                    MulG2Params,
                    MulG2Payload {
                        point: g2.clone(),
                        scalar: a,
                    },
                );
                input.extend_from_slice(&encode_g2(base.product.clone()));
                input.extend_from_slice(&encode_scalar(s.clone()));
                terms.push(MsmG2Term {
                    base: base.product,
                    scalar: s,
                });
            }
            let Ok(sum) = deps.pairing.msm_g2(MsmG2Params, MsmG2Payload { terms });
            g2_msm.push(GroupOperationVector {
                draws,
                input,
                output: encode_g2(sum.sum),
            });
        }

        Some(SecondGroupVectors {
            g2_add,
            g1_msm,
            g2_msm,
        })
    } else {
        None
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
