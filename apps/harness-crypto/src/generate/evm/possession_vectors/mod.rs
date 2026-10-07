mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFields, FromFieldsParams, FromFieldsReturn,
    FromFieldsSuccessReturn, IEncodingContract, ToFieldsParams, ToFieldsReturn,
    ToFieldsSuccessReturn,
};
use envelope::{
    GenerateKeysParams, GenerateKeysPayload, IKeyAgreementAdapter, KeyPairComponentsParams,
    KeyPairComponentsPayload, KeyPairPublicKeysParams, KeyPairPublicKeysPayload,
    PossessionComponentsParams, PossessionComponentsPayload,
};
use pairing::{
    EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload, EncodeScalarParams,
    EncodeScalarPayload, IPairingArithmetic, ISampleUniformScalar,
};
use random::{FillBytesParams, FillBytesPayload};

use super::render::provides::{
    SolidityMemberName, SolidityMemberNameConstructorParams, SolidityRecordMember,
    SolidityRecordMembers, SolidityRecordMembersConstructorParams,
};
use interface::{
    POSSESSION_VECTOR_FIELD_COUNT, POSSESSION_VECTOR_FIELD_KINDS, POSSESSION_VECTOR_MEMBER_NAMES,
    PossessionVector, PossessionVectorDescription, PossessionVectorDescriptionConstructorParams,
    PossessionVectorDescriptionTryNewReturn, PossessionVectorFromFieldsErrorReturn,
    PossessionVectorMembersErrorReturn, PossessionVectorMembersReturn, PossessionVectorsDeps,
    PossessionVectorsErrorReturn, PossessionVectorsParams, PossessionVectorsPayload,
    PossessionVectorsReturn, PossessionVectorsSuccessReturn,
};

impl PossessionVectorDescription {
    pub fn try_new(
        _params: PossessionVectorDescriptionConstructorParams,
    ) -> PossessionVectorDescriptionTryNewReturn {
        Ok(PossessionVectorDescription)
    }

    pub fn members(&self) -> PossessionVectorMembersReturn {
        let mut members = Vec::new();
        for (index, text) in POSSESSION_VECTOR_MEMBER_NAMES.iter().enumerate() {
            let name = SolidityMemberName::try_new(SolidityMemberNameConstructorParams {
                text: text.to_string(),
            })
            .map_err(PossessionVectorMembersErrorReturn::MemberName)?;
            members.push(SolidityRecordMember {
                name,
                kind: POSSESSION_VECTOR_FIELD_KINDS[index],
            });
        }
        SolidityRecordMembers::try_new(SolidityRecordMembersConstructorParams { members })
            .map_err(PossessionVectorMembersErrorReturn::RecordMembers)
    }
}

impl IEncodingContract for PossessionVectorDescription {
    type Described = PossessionVector;
    type FromFieldsErrorReturn = PossessionVectorFromFieldsErrorReturn;
    const FIELDS: &'static [CanonicalFieldKind] = &POSSESSION_VECTOR_FIELD_KINDS;

    fn to_fields(&self, _params: ToFieldsParams, payload: &Self::Described) -> ToFieldsReturn {
        Ok(ToFieldsSuccessReturn {
            fields: CanonicalFields {
                values: vec![
                    CanonicalFieldValue::Unsigned256(payload.x),
                    CanonicalFieldValue::Unsigned256(payload.y),
                    CanonicalFieldValue::Bytes(payload.pk1.clone()),
                    CanonicalFieldValue::Bytes(payload.pk2.clone()),
                    CanonicalFieldValue::Bytes(payload.r1.clone()),
                    CanonicalFieldValue::Unsigned256(payload.z1),
                    CanonicalFieldValue::Bytes(payload.r2.clone()),
                    CanonicalFieldValue::Unsigned256(payload.z2),
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
            <[CanonicalFieldValue; POSSESSION_VECTOR_FIELD_COUNT]>::try_from(payload.values)
        else {
            return Err(PossessionVectorFromFieldsErrorReturn::FieldCount {
                expected: POSSESSION_VECTOR_FIELD_COUNT,
                actual,
            });
        };
        let [x, y, pk1, pk2, r1, z1, r2, z2] = fields;
        let CanonicalFieldValue::Unsigned256(x) = x else {
            return Err(PossessionVectorFromFieldsErrorReturn::FieldKind {
                index: 0,
                expected: POSSESSION_VECTOR_FIELD_KINDS[0],
            });
        };
        let CanonicalFieldValue::Unsigned256(y) = y else {
            return Err(PossessionVectorFromFieldsErrorReturn::FieldKind {
                index: 1,
                expected: POSSESSION_VECTOR_FIELD_KINDS[1],
            });
        };
        let CanonicalFieldValue::Bytes(pk1) = pk1 else {
            return Err(PossessionVectorFromFieldsErrorReturn::FieldKind {
                index: 2,
                expected: POSSESSION_VECTOR_FIELD_KINDS[2],
            });
        };
        let CanonicalFieldValue::Bytes(pk2) = pk2 else {
            return Err(PossessionVectorFromFieldsErrorReturn::FieldKind {
                index: 3,
                expected: POSSESSION_VECTOR_FIELD_KINDS[3],
            });
        };
        let CanonicalFieldValue::Bytes(r1) = r1 else {
            return Err(PossessionVectorFromFieldsErrorReturn::FieldKind {
                index: 4,
                expected: POSSESSION_VECTOR_FIELD_KINDS[4],
            });
        };
        let CanonicalFieldValue::Unsigned256(z1) = z1 else {
            return Err(PossessionVectorFromFieldsErrorReturn::FieldKind {
                index: 5,
                expected: POSSESSION_VECTOR_FIELD_KINDS[5],
            });
        };
        let CanonicalFieldValue::Bytes(r2) = r2 else {
            return Err(PossessionVectorFromFieldsErrorReturn::FieldKind {
                index: 6,
                expected: POSSESSION_VECTOR_FIELD_KINDS[6],
            });
        };
        let CanonicalFieldValue::Unsigned256(z2) = z2 else {
            return Err(PossessionVectorFromFieldsErrorReturn::FieldKind {
                index: 7,
                expected: POSSESSION_VECTOR_FIELD_KINDS[7],
            });
        };
        Ok(FromFieldsSuccessReturn {
            described: PossessionVector {
                x,
                y,
                pk1,
                pk2,
                r1,
                z1,
                r2,
                z2,
            },
        })
    }
}

pub fn possession_vectors<'a, P: IPairingArithmetic, K: IKeyAgreementAdapter<Pairing = P>>(
    deps: &PossessionVectorsDeps<'a, P, K>,
    params: PossessionVectorsParams,
    _payload: PossessionVectorsPayload,
) -> PossessionVectorsReturn {
    let scalar_bytes = |scalar: P::Scalar| -> Result<[u8; 32], PossessionVectorsErrorReturn> {
        let Ok(encoded) = deps
            .pairing
            .encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar });
        let actual = encoded.bytes.expose().as_ref().len();
        let Ok(bytes) = <[u8; 32]>::try_from(encoded.bytes.expose().as_ref()) else {
            return Err(PossessionVectorsErrorReturn::ScalarLength { actual });
        };
        Ok(bytes)
    };

    let mut records = Vec::new();
    for _ in 0..params.count.get() {
        let x_uniform = deps
            .random
            .fill_bytes(
                FillBytesParams,
                FillBytesPayload {
                    length: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                },
            )
            .map_err(PossessionVectorsErrorReturn::FillBytes)?
            .bytes;
        let y_uniform = deps
            .random
            .fill_bytes(
                FillBytesParams,
                FillBytesPayload {
                    length: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                },
            )
            .map_err(PossessionVectorsErrorReturn::FillBytes)?
            .bytes;
        let success = deps
            .key_agreement
            .generate_keys(
                GenerateKeysParams,
                GenerateKeysPayload {
                    x_uniform,
                    y_uniform,
                },
            )
            .map_err(PossessionVectorsErrorReturn::GenerateKeys)?;
        let Ok(components) = deps.key_agreement.key_pair_components(
            KeyPairComponentsParams,
            KeyPairComponentsPayload {
                key_pair: &success.key_pair,
            },
        );
        let Ok(public_keys) = deps.key_agreement.key_pair_public_keys(
            KeyPairPublicKeysParams,
            KeyPairPublicKeysPayload {
                key_pair: &success.key_pair,
            },
        );
        let Ok(proof) = deps.key_agreement.possession_components(
            PossessionComponentsParams,
            PossessionComponentsPayload {
                possession: &success.possession,
            },
        );
        let Ok(pk1) = deps.pairing.encode_g1(
            EncodeG1Params,
            EncodeG1Payload {
                point: public_keys.components.pk1,
            },
        );
        let Ok(pk2) = deps.pairing.encode_g2(
            EncodeG2Params,
            EncodeG2Payload {
                point: public_keys.components.pk2,
            },
        );
        let Ok(r1) = deps.pairing.encode_g1(
            EncodeG1Params,
            EncodeG1Payload {
                point: proof.components.r1,
            },
        );
        let Ok(r2) = deps.pairing.encode_g2(
            EncodeG2Params,
            EncodeG2Payload {
                point: proof.components.r2,
            },
        );
        records.push(PossessionVector {
            x: scalar_bytes(components.components.x.expose().clone())?,
            y: scalar_bytes(components.components.y.expose().clone())?,
            pk1: pk1.bytes.as_ref().to_vec(),
            pk2: pk2.bytes.as_ref().to_vec(),
            r1: r1.bytes.as_ref().to_vec(),
            z1: scalar_bytes(proof.components.z1)?,
            r2: r2.bytes.as_ref().to_vec(),
            z2: scalar_bytes(proof.components.z2)?,
        });
    }

    Ok(PossessionVectorsSuccessReturn { records })
}
