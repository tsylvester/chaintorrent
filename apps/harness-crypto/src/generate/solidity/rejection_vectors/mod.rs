mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use encoding::{
    CanonicalFieldKind, CanonicalFieldReader, CanonicalFieldValue, CanonicalFields,
    FromFieldsParams, FromFieldsReturn, FromFieldsSuccessReturn, IEncodingContract, ToFieldsParams,
    ToFieldsReturn, ToFieldsSuccessReturn,
};
use pairing::{
    G1OutsideSubgroupEncodingParams, G1OutsideSubgroupEncodingPayload,
    G2OutsideSubgroupEncodingParams, G2OutsideSubgroupEncodingPayload, IPairingReference,
    ScalarFieldOrderParams, ScalarFieldOrderPayload,
};

use self::interface::{
    DecodeTarget, DecodeTargetConstructorParams, DecodeTargetTryNewErrorReturn,
    DecodeTargetTryNewReturn, REJECTION_VECTOR_FIELD_KINDS, REJECTION_VECTOR_MEMBER_NAMES,
    RejectionVector, RejectionVectorDescription, RejectionVectorDescriptionConstructorParams,
    RejectionVectorDescriptionTryNewReturn, RejectionVectorFromFieldsErrorReturn,
    RejectionVectorMemberNamesErrorReturn, RejectionVectorMemberNamesReturn, RejectionVectorsDeps,
    RejectionVectorsErrorReturn, RejectionVectorsParams, RejectionVectorsPayload,
    RejectionVectorsReturn, RejectionVectorsSuccessReturn,
};
use super::render::provides::{SolidityMemberName, SolidityMemberNameConstructorParams};

impl DecodeTarget {
    pub fn try_new(params: DecodeTargetConstructorParams) -> DecodeTargetTryNewReturn {
        if params.text == DecodeTarget::G1.as_str() {
            return Ok(DecodeTarget::G1);
        }
        if params.text == DecodeTarget::G2.as_str() {
            return Ok(DecodeTarget::G2);
        }
        if params.text == DecodeTarget::Scalar.as_str() {
            return Ok(DecodeTarget::Scalar);
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

    pub fn member_names(&self) -> RejectionVectorMemberNamesReturn {
        let mut names = Vec::new();
        for text in REJECTION_VECTOR_MEMBER_NAMES {
            match SolidityMemberName::try_new(SolidityMemberNameConstructorParams {
                text: text.to_string(),
            }) {
                Err(error) => {
                    return Err(RejectionVectorMemberNamesErrorReturn::MemberName(error));
                }
                Ok(name) => names.push(name),
            }
        }
        Ok(names)
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
        let mut reader = CanonicalFieldReader::try_new(payload, Self::FIELDS)
            .map_err(RejectionVectorFromFieldsErrorReturn::Fields)?;
        let text = reader
            .read_text()
            .map_err(RejectionVectorFromFieldsErrorReturn::Fields)?;
        let target = match DecodeTarget::try_new(DecodeTargetConstructorParams { text }) {
            Err(error) => {
                return Err(RejectionVectorFromFieldsErrorReturn::Target(error));
            }
            Ok(target) => target,
        };
        let input = reader
            .read_bytes()
            .map_err(RejectionVectorFromFieldsErrorReturn::Fields)?;
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
    let g1 = match deps.pairing.g1_outside_subgroup_encoding(
        G1OutsideSubgroupEncodingParams,
        G1OutsideSubgroupEncodingPayload,
    ) {
        Err(error) => {
            return Err(RejectionVectorsErrorReturn::G1OutsideSubgroupEncoding(
                error,
            ));
        }
        Ok(success) => success.bytes,
    };
    if let Some(bytes) = g1 {
        records.push(RejectionVector {
            target: DecodeTarget::G1,
            input: bytes.as_ref().to_vec(),
        });
    }
    let g2 = match deps.pairing.g2_outside_subgroup_encoding(
        G2OutsideSubgroupEncodingParams,
        G2OutsideSubgroupEncodingPayload,
    ) {
        Err(error) => {
            return Err(RejectionVectorsErrorReturn::G2OutsideSubgroupEncoding(
                error,
            ));
        }
        Ok(success) => success.bytes,
    };
    records.push(RejectionVector {
        target: DecodeTarget::G2,
        input: g2.as_ref().to_vec(),
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
