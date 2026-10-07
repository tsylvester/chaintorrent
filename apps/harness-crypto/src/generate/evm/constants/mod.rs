mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use chain::IChainForms;
use encoding::{CanonicalFieldKind, IEncodingContract};
use pairing::{
    EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload, G1GeneratorParams,
    G1GeneratorPayload, G2GeneratorParams, G2GeneratorPayload, IPairingAdapter, IPairingReference,
    ScalarFieldOrderParams, ScalarFieldOrderPayload, VerifierGroupArithmetic,
};
use proof::{
    DELIVERY_PURPOSES, DELIVERY_STATEMENT_VERSION_ONE, DeliveryPurpose, MintStatementDescription,
    TransferStatementDescription,
};

use super::render::provides::{
    SolidityAbiType, SolidityAbiTypeConstructorParams, SolidityConstantName,
    SolidityConstantNameConstructorParams, SolidityConstantValue, SolidityLibraryEntry,
    SolidityStringLiteral, SolidityStringLiteralConstructorParams, SolidityUint256,
    SolidityUint256ConstructorParams,
};
use interface::{
    ConstantsDeps, ConstantsErrorReturn, ConstantsParams, ConstantsPayload, ConstantsReturn,
    ConstantsSuccessReturn,
};

pub fn constants<'a, P: IPairingReference, F: IChainForms>(
    deps: &ConstantsDeps<'a, P>,
    params: ConstantsParams,
    payload: ConstantsPayload<'_>,
) -> ConstantsReturn {
    if params.statement_version != DELIVERY_STATEMENT_VERSION_ONE {
        return Err(ConstantsErrorReturn::UnsupportedStatementVersion {
            version: params.statement_version,
        });
    }

    let constant_name = |text: &str| -> Result<SolidityConstantName, ConstantsErrorReturn> {
        SolidityConstantName::try_new(SolidityConstantNameConstructorParams {
            text: text.to_string(),
        })
        .map_err(ConstantsErrorReturn::ConstantName)
    };
    let field_sequence = |fields: &'static [CanonicalFieldKind]| {
        let text = fields
            .iter()
            .map(|kind| {
                let Ok(abi_type) =
                    SolidityAbiType::try_new(SolidityAbiTypeConstructorParams { kind: *kind });
                abi_type.as_str()
            })
            .collect::<Vec<_>>()
            .join(",");
        SolidityStringLiteral::try_new(SolidityStringLiteralConstructorParams { text })
            .map_err(ConstantsErrorReturn::StringLiteral)
    };

    let mut entries: Vec<SolidityLibraryEntry> = Vec::new();
    entries.push(SolidityLibraryEntry {
        name: constant_name("IDENTITY_TAG")?,
        value: SolidityConstantValue::Bytes(payload.kem.identity_tag.to_vec()),
    });
    entries.push(SolidityLibraryEntry {
        name: constant_name("POSSESSION_G1_TAG")?,
        value: SolidityConstantValue::Bytes(payload.key_agreement.possession_g1_tag.to_vec()),
    });
    entries.push(SolidityLibraryEntry {
        name: constant_name("POSSESSION_G2_TAG")?,
        value: SolidityConstantValue::Bytes(payload.key_agreement.possession_g2_tag.to_vec()),
    });
    entries.push(SolidityLibraryEntry {
        name: constant_name("CHALLENGE_TAG")?,
        value: SolidityConstantValue::Bytes(payload.delivery_proof.challenge_tag.to_vec()),
    });
    entries.push(SolidityLibraryEntry {
        name: constant_name("WEIGHT_TAG")?,
        value: SolidityConstantValue::Bytes(payload.delivery_proof.weight_tag.to_vec()),
    });
    entries.push(SolidityLibraryEntry {
        name: constant_name("DELIVERY_STATEMENT_VERSION")?,
        value: SolidityConstantValue::Uint16(params.statement_version),
    });
    for purpose in DELIVERY_PURPOSES {
        let text = match purpose {
            DeliveryPurpose::Mint => "PURPOSE_MINT",
            DeliveryPurpose::Transfer => "PURPOSE_TRANSFER",
            DeliveryPurpose::Grant => "PURPOSE_GRANT",
            DeliveryPurpose::Replacement => "PURPOSE_REPLACEMENT",
        };
        entries.push(SolidityLibraryEntry {
            name: constant_name(text)?,
            value: SolidityConstantValue::Uint16(purpose.code()),
        });
    }
    entries.push(SolidityLibraryEntry {
        name: constant_name("MINT_FIELDS")?,
        value: SolidityConstantValue::String(field_sequence(
            <MintStatementDescription<'_, P, F> as IEncodingContract>::FIELDS,
        )?),
    });
    entries.push(SolidityLibraryEntry {
        name: constant_name("TRANSFER_FIELDS")?,
        value: SolidityConstantValue::String(field_sequence(
            <TransferStatementDescription<'_, P, F> as IEncodingContract>::FIELDS,
        )?),
    });

    let Ok(order) = deps
        .pairing
        .scalar_field_order(ScalarFieldOrderParams, ScalarFieldOrderPayload);
    let big_endian = match <[u8; 32]>::try_from(order.bytes) {
        Ok(bytes) => bytes,
        Err(bytes) => {
            return Err(ConstantsErrorReturn::ScalarFieldOrderLength {
                actual: bytes.len(),
            });
        }
    };
    let Ok(scalar_field_order) =
        SolidityUint256::try_new(SolidityUint256ConstructorParams { big_endian });
    entries.push(SolidityLibraryEntry {
        name: constant_name("SCALAR_FIELD_ORDER")?,
        value: SolidityConstantValue::Uint256(scalar_field_order),
    });

    let Ok(g1) = deps
        .pairing
        .g1_generator(G1GeneratorParams, G1GeneratorPayload);
    let Ok(g1_encoded) = deps
        .pairing
        .encode_g1(EncodeG1Params, EncodeG1Payload { point: g1.point });
    let Ok(g2) = deps
        .pairing
        .g2_generator(G2GeneratorParams, G2GeneratorPayload);
    let Ok(g2_encoded) = deps
        .pairing
        .encode_g2(EncodeG2Params, EncodeG2Payload { point: g2.point });
    entries.push(SolidityLibraryEntry {
        name: constant_name("G1_GENERATOR")?,
        value: SolidityConstantValue::Bytes(g1_encoded.bytes.as_ref().to_vec()),
    });
    entries.push(SolidityLibraryEntry {
        name: constant_name("G2_GENERATOR")?,
        value: SolidityConstantValue::Bytes(g2_encoded.bytes.as_ref().to_vec()),
    });

    let second_group_arithmetic = match P::DECLARATION.verifier_group_arithmetic {
        VerifierGroupArithmetic::BothGroups => true,
        VerifierGroupArithmetic::FirstGroupOnly => false,
    };
    entries.push(SolidityLibraryEntry {
        name: constant_name("SECOND_GROUP_ARITHMETIC")?,
        value: SolidityConstantValue::Bool(second_group_arithmetic),
    });

    Ok(ConstantsSuccessReturn { entries })
}
