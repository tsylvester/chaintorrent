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
use pairing::{
    G1OutsideSubgroupEncodingParams, G1OutsideSubgroupEncodingPayload,
    G2OutsideSubgroupEncodingParams, G2OutsideSubgroupEncodingPayload, IPairingReference,
    ScalarFieldOrderParams, ScalarFieldOrderPayload,
};

use super::render::provides::{
    SolidityMemberName, SolidityMemberNameConstructorParams, SolidityRecordMember,
    SolidityRecordMembers, SolidityRecordMembersConstructorParams,
};
use interface::{
    DecodeTarget, DecodeTargetConstructorParams, DecodeTargetTryNewErrorReturn,
    DecodeTargetTryNewReturn, REJECTION_VECTOR_FIELD_COUNT, REJECTION_VECTOR_FIELD_KINDS,
    REJECTION_VECTOR_MEMBER_NAMES, RejectionVector, RejectionVectorDescription,
    RejectionVectorDescriptionConstructorParams, RejectionVectorDescriptionTryNewReturn,
    RejectionVectorFromFieldsErrorReturn, RejectionVectorMembersErrorReturn,
    RejectionVectorMembersReturn, RejectionVectorsDeps, RejectionVectorsErrorReturn,
    RejectionVectorsParams, RejectionVectorsPayload, RejectionVectorsReturn,
    RejectionVectorsSuccessReturn,
};

impl DecodeTarget {
    pub fn try_new(params: DecodeTargetConstructorParams) -> DecodeTargetTryNewReturn {
        for variant in [DecodeTarget::G1, DecodeTarget::G2, DecodeTarget::Scalar] {
            if params.text == variant.as_str() {
                return Ok(variant);
            }
        }
        Err(DecodeTargetTryNewErrorReturn::Unknown { text: params.text })
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            DecodeTarget::G1 => "G1",
            DecodeTarget::G2 => "G2",
            DecodeTarget::Scalar => "Scalar",
        }
    }
}

impl RejectionVectorDescription {
    pub fn try_new(
        _params: RejectionVectorDescriptionConstructorParams,
    ) -> RejectionVectorDescriptionTryNewReturn {
        Ok(RejectionVectorDescription)
    }

    pub fn members(&self) -> RejectionVectorMembersReturn {
        let mut members = Vec::new();
        for (index, text) in REJECTION_VECTOR_MEMBER_NAMES.iter().enumerate() {
            let name = SolidityMemberName::try_new(SolidityMemberNameConstructorParams {
                text: text.to_string(),
            })
            .map_err(RejectionVectorMembersErrorReturn::MemberName)?;
            members.push(SolidityRecordMember {
                name,
                kind: REJECTION_VECTOR_FIELD_KINDS[index],
            });
        }
        SolidityRecordMembers::try_new(SolidityRecordMembersConstructorParams { members })
            .map_err(RejectionVectorMembersErrorReturn::RecordMembers)
    }
}

impl IEncodingContract for RejectionVectorDescription {
    type Described = RejectionVector;
    type FromFieldsErrorReturn = RejectionVectorFromFieldsErrorReturn;
    const FIELDS: &'static [CanonicalFieldKind] = &REJECTION_VECTOR_FIELD_KINDS;

    fn to_fields(&self, _params: ToFieldsParams, payload: &Self::Described) -> ToFieldsReturn {
        Ok(ToFieldsSuccessReturn {
            fields: CanonicalFields {
                values: vec![
                    CanonicalFieldValue::Text(payload.target.as_str().to_string()),
                    CanonicalFieldValue::Bytes(payload.input.clone()),
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
            <[CanonicalFieldValue; REJECTION_VECTOR_FIELD_COUNT]>::try_from(payload.values)
        else {
            return Err(RejectionVectorFromFieldsErrorReturn::FieldCount {
                expected: REJECTION_VECTOR_FIELD_COUNT,
                actual,
            });
        };
        let [target, input] = fields;
        let CanonicalFieldValue::Text(text) = target else {
            return Err(RejectionVectorFromFieldsErrorReturn::FieldKind {
                index: 0,
                expected: REJECTION_VECTOR_FIELD_KINDS[0],
            });
        };
        let CanonicalFieldValue::Bytes(input) = input else {
            return Err(RejectionVectorFromFieldsErrorReturn::FieldKind {
                index: 1,
                expected: REJECTION_VECTOR_FIELD_KINDS[1],
            });
        };
        let target = DecodeTarget::try_new(DecodeTargetConstructorParams { text })
            .map_err(RejectionVectorFromFieldsErrorReturn::Target)?;
        Ok(FromFieldsSuccessReturn {
            described: RejectionVector { target, input },
        })
    }
}

pub fn rejection_vectors<'a, P: IPairingReference>(
    deps: &RejectionVectorsDeps<'a, P>,
    _params: RejectionVectorsParams,
    _payload: RejectionVectorsPayload,
) -> RejectionVectorsReturn {
    let mut records = Vec::new();
    let first = deps
        .pairing
        .g1_outside_subgroup_encoding(
            G1OutsideSubgroupEncodingParams,
            G1OutsideSubgroupEncodingPayload,
        )
        .map_err(RejectionVectorsErrorReturn::G1OutsideSubgroupEncoding)?;
    if let Some(bytes) = first.bytes {
        records.push(RejectionVector {
            target: DecodeTarget::G1,
            input: bytes.as_ref().to_vec(),
        });
    }
    let second = deps
        .pairing
        .g2_outside_subgroup_encoding(
            G2OutsideSubgroupEncodingParams,
            G2OutsideSubgroupEncodingPayload,
        )
        .map_err(RejectionVectorsErrorReturn::G2OutsideSubgroupEncoding)?;
    records.push(RejectionVector {
        target: DecodeTarget::G2,
        input: second.bytes.as_ref().to_vec(),
    });
    let Ok(order) = deps
        .pairing
        .scalar_field_order(ScalarFieldOrderParams, ScalarFieldOrderPayload);
    records.push(RejectionVector {
        target: DecodeTarget::Scalar,
        input: order.bytes,
    });
    Ok(RejectionVectorsSuccessReturn { records })
}
