#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use chain::IChainForms;
use encoding::{
    CanonicalFieldKind, CanonicalFieldReader, CanonicalFieldValue, CanonicalFields,
    FromFieldsParams, FromFieldsReturn, FromFieldsSuccessReturn, IEncoderAdapter,
    IEncodingContract, ToFieldsParams, ToFieldsReturn, ToFieldsSuccessReturn, TypeNameParams,
};
use pairing::{
    EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload, G1GeneratorParams,
    G1GeneratorPayload, G2GeneratorParams, G2GeneratorPayload, IPairingReference,
    ScalarFieldOrderParams, ScalarFieldOrderPayload, VerifierGroupArithmetic,
};
use proof::{
    DELIVERY_PURPOSES, DELIVERY_STATEMENT_VERSION_ONE, DeliveryPurpose, MintStatementDescription,
    TransferStatementDescription,
};

use self::interface::{
    CELL_VALUES_FIELD_KINDS, CELL_VALUES_MEMBER_NAMES, CellValues, CellValuesDescription,
    CellValuesDescriptionConstructorParams, CellValuesDescriptionTryNewReturn,
    CellValuesFromFieldsErrorReturn, CellValuesMemberNamesErrorReturn, CellValuesMemberNamesReturn,
    ConstantsDeps, ConstantsErrorReturn, ConstantsParams, ConstantsPayload, ConstantsReturn,
    ConstantsSuccessReturn,
};
use super::render::provides::{
    SolidityAbiType, SolidityAbiTypeConstructorParams, SolidityConstantName,
    SolidityConstantNameConstructorParams, SolidityConstantValue, SolidityLibraryEntry,
    SolidityMemberName, SolidityMemberNameConstructorParams, SolidityStringLiteral,
    SolidityStringLiteralConstructorParams,
};

impl CellValuesDescription {
    pub fn try_new(
        _params: CellValuesDescriptionConstructorParams,
    ) -> CellValuesDescriptionTryNewReturn {
        Ok(CellValuesDescription)
    }

    pub fn member_names(&self) -> CellValuesMemberNamesReturn {
        let mut names = Vec::new();
        for text in CELL_VALUES_MEMBER_NAMES {
            match SolidityMemberName::try_new(SolidityMemberNameConstructorParams {
                text: text.to_string(),
            }) {
                Err(error) => {
                    return Err(CellValuesMemberNamesErrorReturn::MemberName(error));
                }
                Ok(name) => names.push(name),
            }
        }
        Ok(names)
    }
}

impl IEncodingContract for CellValuesDescription {
    type Described = CellValues;
    type FromFieldsErrorReturn = CellValuesFromFieldsErrorReturn;

    const FIELDS: &'static [CanonicalFieldKind] = &CELL_VALUES_FIELD_KINDS;

    fn to_fields(&self, _params: ToFieldsParams, payload: &Self::Described) -> ToFieldsReturn {
        Ok(ToFieldsSuccessReturn {
            fields: CanonicalFields {
                values: vec![
                    CanonicalFieldValue::Text(payload.curve_name.clone()),
                    CanonicalFieldValue::Text(payload.precompile_encoding.clone()),
                    CanonicalFieldValue::FixedBytes32(payload.scalar_field_order),
                    CanonicalFieldValue::Bytes(payload.g1_generator.clone()),
                    CanonicalFieldValue::Bytes(payload.g2_generator.clone()),
                    CanonicalFieldValue::Unsigned16(u16::from(payload.second_group_arithmetic)),
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
            .map_err(CellValuesFromFieldsErrorReturn::Fields)?;
        let curve_name = reader
            .read_text()
            .map_err(CellValuesFromFieldsErrorReturn::Fields)?;
        let precompile_encoding = reader
            .read_text()
            .map_err(CellValuesFromFieldsErrorReturn::Fields)?;
        let scalar_field_order = reader
            .read_fixed_bytes32()
            .map_err(CellValuesFromFieldsErrorReturn::Fields)?;
        let g1_generator = reader
            .read_bytes()
            .map_err(CellValuesFromFieldsErrorReturn::Fields)?;
        let g2_generator = reader
            .read_bytes()
            .map_err(CellValuesFromFieldsErrorReturn::Fields)?;
        let second_group_arithmetic = match reader
            .read_unsigned16()
            .map_err(CellValuesFromFieldsErrorReturn::Fields)?
        {
            0 => false,
            1 => true,
            actual => {
                return Err(CellValuesFromFieldsErrorReturn::SecondGroupArithmetic { actual });
            }
        };
        Ok(FromFieldsSuccessReturn {
            described: CellValues {
                curve_name,
                precompile_encoding,
                scalar_field_order,
                g1_generator,
                g2_generator,
                second_group_arithmetic,
            },
        })
    }
}

pub fn constants<'a, P: IPairingReference, E: IEncoderAdapter, F: IChainForms>(
    deps: &ConstantsDeps<'a, P, E>,
    params: ConstantsParams,
    payload: ConstantsPayload<'_>,
) -> ConstantsReturn {
    if params.statement_version != DELIVERY_STATEMENT_VERSION_ONE {
        return Err(ConstantsErrorReturn::UnsupportedStatementVersion {
            version: params.statement_version,
        });
    }

    let entry = |text: &str, value: SolidityConstantValue| {
        let name = match SolidityConstantName::try_new(SolidityConstantNameConstructorParams {
            text: text.to_string(),
        }) {
            Err(error) => return Err(ConstantsErrorReturn::ConstantName(error)),
            Ok(name) => name,
        };
        Ok(SolidityLibraryEntry { name, value })
    };

    let mut entries = vec![
        entry(
            "IDENTITY_TAG",
            SolidityConstantValue::Bytes(payload.kem.identity_tag.to_vec()),
        )?,
        entry(
            "POSSESSION_G1_TAG",
            SolidityConstantValue::Bytes(payload.key_agreement.possession_g1_tag.to_vec()),
        )?,
        entry(
            "POSSESSION_G2_TAG",
            SolidityConstantValue::Bytes(payload.key_agreement.possession_g2_tag.to_vec()),
        )?,
        entry(
            "CHALLENGE_TAG",
            SolidityConstantValue::Bytes(payload.delivery_proof.challenge_tag.to_vec()),
        )?,
        entry(
            "WEIGHT_TAG",
            SolidityConstantValue::Bytes(payload.delivery_proof.weight_tag.to_vec()),
        )?,
        entry(
            "DELIVERY_STATEMENT_VERSION",
            SolidityConstantValue::Uint16(params.statement_version),
        )?,
    ];

    for purpose in DELIVERY_PURPOSES {
        let name = match purpose {
            DeliveryPurpose::Mint => "PURPOSE_MINT",
            DeliveryPurpose::Transfer => "PURPOSE_TRANSFER",
            DeliveryPurpose::Grant => "PURPOSE_GRANT",
            DeliveryPurpose::Replacement => "PURPOSE_REPLACEMENT",
        };
        entries.push(entry(name, SolidityConstantValue::Uint16(purpose.code()))?);
    }

    for (name, fields) in [
        (
            "MINT_FIELDS",
            <MintStatementDescription<'a, P, F> as IEncodingContract>::FIELDS,
        ),
        (
            "TRANSFER_FIELDS",
            <TransferStatementDescription<'a, P, F> as IEncodingContract>::FIELDS,
        ),
    ] {
        let mut names = Vec::new();
        for kind in fields {
            let Ok(success) = deps.encoder.type_name(TypeNameParams, *kind);
            let abi_type = match SolidityAbiType::try_new(SolidityAbiTypeConstructorParams {
                name: success.name.as_str().to_string(),
            }) {
                Err(error) => return Err(ConstantsErrorReturn::AbiType(error)),
                Ok(abi_type) => abi_type,
            };
            names.push(abi_type.as_str().to_string());
        }
        let literal = match SolidityStringLiteral::try_new(SolidityStringLiteralConstructorParams {
            text: names.join(","),
        }) {
            Err(error) => return Err(ConstantsErrorReturn::StringLiteral(error)),
            Ok(literal) => literal,
        };
        entries.push(entry(name, SolidityConstantValue::String(literal))?);
    }

    entries.push(entry(
        "KEY_IDENTIFIER_DOMAIN_TAG",
        SolidityConstantValue::Bytes(F::DECLARATION.key_identifier_domain_tag.to_vec()),
    )?);
    for (name, text) in [
        ("INTENT_DOMAIN_NAME", F::DECLARATION.intent_domain_name),
        (
            "INTENT_DOMAIN_VERSION",
            F::DECLARATION.intent_domain_version,
        ),
        ("INTENT_STRUCT_NAME", F::DECLARATION.intent_struct_name),
        (
            "INTENT_MEMBER_IDENTITY",
            F::DECLARATION.intent_member_names.identity,
        ),
        (
            "INTENT_MEMBER_NONCE",
            F::DECLARATION.intent_member_names.nonce,
        ),
        (
            "INTENT_MEMBER_EXPIRY",
            F::DECLARATION.intent_member_names.expiry,
        ),
        (
            "INTENT_MEMBER_CALL",
            F::DECLARATION.intent_member_names.call,
        ),
    ] {
        let literal = match SolidityStringLiteral::try_new(SolidityStringLiteralConstructorParams {
            text: text.to_string(),
        }) {
            Err(error) => return Err(ConstantsErrorReturn::StringLiteral(error)),
            Ok(literal) => literal,
        };
        entries.push(entry(name, SolidityConstantValue::String(literal))?);
    }

    let kind_type_name = |kind: CanonicalFieldKind| {
        let Ok(success) = deps.encoder.type_name(TypeNameParams, kind);
        let abi_type = match SolidityAbiType::try_new(SolidityAbiTypeConstructorParams {
            name: success.name.as_str().to_string(),
        }) {
            Err(error) => return Err(ConstantsErrorReturn::AbiType(error)),
            Ok(abi_type) => abi_type,
        };
        match SolidityStringLiteral::try_new(SolidityStringLiteralConstructorParams {
            text: abi_type.as_str().to_string(),
        }) {
            Err(error) => Err(ConstantsErrorReturn::StringLiteral(error)),
            Ok(literal) => Ok(literal),
        }
    };

    for (name, kind) in [
        ("NONCE_KIND_TYPE_NAME", F::DECLARATION.nonce_kind),
        ("EXPIRY_KIND_TYPE_NAME", F::DECLARATION.expiry_kind),
        ("FIXED_BYTES32_TYPE_NAME", CanonicalFieldKind::FixedBytes32),
        ("UNSIGNED16_TYPE_NAME", CanonicalFieldKind::Unsigned16),
        ("UNSIGNED32_TYPE_NAME", CanonicalFieldKind::Unsigned32),
        ("UNSIGNED64_TYPE_NAME", CanonicalFieldKind::Unsigned64),
        ("TEXT_TYPE_NAME", CanonicalFieldKind::Text),
        ("BYTES_TYPE_NAME", CanonicalFieldKind::Bytes),
        ("FIXED_BYTES20_TYPE_NAME", CanonicalFieldKind::FixedBytes20),
        ("UNSIGNED256_TYPE_NAME", CanonicalFieldKind::Unsigned256),
    ] {
        let literal = kind_type_name(kind)?;
        entries.push(entry(name, SolidityConstantValue::String(literal))?);
    }

    let Ok(success) = deps
        .pairing
        .scalar_field_order(ScalarFieldOrderParams, ScalarFieldOrderPayload);
    let scalar_field_order = match <[u8; 32]>::try_from(success.bytes) {
        Err(bytes) => {
            return Err(ConstantsErrorReturn::ScalarFieldOrderLength {
                actual: bytes.len(),
            });
        }
        Ok(big_endian) => big_endian,
    };

    let Ok(g1) = deps
        .pairing
        .g1_generator(G1GeneratorParams, G1GeneratorPayload);
    let Ok(encoded_g1) = deps
        .pairing
        .encode_g1(EncodeG1Params, EncodeG1Payload { point: g1.point });
    let Ok(g2) = deps
        .pairing
        .g2_generator(G2GeneratorParams, G2GeneratorPayload);
    let Ok(encoded_g2) = deps
        .pairing
        .encode_g2(EncodeG2Params, EncodeG2Payload { point: g2.point });

    let second_group_arithmetic = match P::DECLARATION.verifier_group_arithmetic {
        VerifierGroupArithmetic::BothGroups => true,
        VerifierGroupArithmetic::FirstGroupOnly => false,
    };

    let cell_values = CellValues {
        curve_name: format!("{:?}", P::DECLARATION.curve),
        precompile_encoding: format!("{:?}", P::DECLARATION.precompile_encoding),
        scalar_field_order,
        g1_generator: encoded_g1.bytes.as_ref().to_vec(),
        g2_generator: encoded_g2.bytes.as_ref().to_vec(),
        second_group_arithmetic,
    };

    Ok(ConstantsSuccessReturn {
        entries,
        cell_values,
    })
}
