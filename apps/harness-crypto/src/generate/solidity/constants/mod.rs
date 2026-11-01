#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use chain::IChainForms;
use encoding::{IEncoderAdapter, IEncodingContract, TypeNameParams};
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
    ConstantsDeps, ConstantsErrorReturn, ConstantsParams, ConstantsPayload, ConstantsReturn,
    ConstantsSuccessReturn,
};
use super::render::provides::{
    SolidityAbiType, SolidityAbiTypeConstructorParams, SolidityConstantName,
    SolidityConstantNameConstructorParams, SolidityConstantValue, SolidityLibraryEntry,
    SolidityStringLiteral, SolidityStringLiteralConstructorParams, SolidityUint256,
    SolidityUint256ConstructorParams,
};

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

    let mut entries = Vec::new();
    entries.push(entry(
        "IDENTITY_TAG",
        SolidityConstantValue::Bytes(payload.kem.identity_tag.to_vec()),
    )?);
    entries.push(entry(
        "POSSESSION_G1_TAG",
        SolidityConstantValue::Bytes(payload.key_agreement.possession_g1_tag.to_vec()),
    )?);
    entries.push(entry(
        "POSSESSION_G2_TAG",
        SolidityConstantValue::Bytes(payload.key_agreement.possession_g2_tag.to_vec()),
    )?);
    entries.push(entry(
        "CHALLENGE_TAG",
        SolidityConstantValue::Bytes(payload.delivery_proof.challenge_tag.to_vec()),
    )?);
    entries.push(entry(
        "WEIGHT_TAG",
        SolidityConstantValue::Bytes(payload.delivery_proof.weight_tag.to_vec()),
    )?);
    entries.push(entry(
        "DELIVERY_STATEMENT_VERSION",
        SolidityConstantValue::Uint16(params.statement_version),
    )?);

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

    let Ok(success) = deps
        .pairing
        .scalar_field_order(ScalarFieldOrderParams, ScalarFieldOrderPayload);
    let big_endian = match <[u8; 32]>::try_from(success.bytes) {
        Err(bytes) => {
            return Err(ConstantsErrorReturn::ScalarFieldOrderLength {
                actual: bytes.len(),
            });
        }
        Ok(big_endian) => big_endian,
    };
    let Ok(order) = SolidityUint256::try_new(SolidityUint256ConstructorParams { big_endian });
    entries.push(entry(
        "SCALAR_FIELD_ORDER",
        SolidityConstantValue::Uint256(order),
    )?);

    let Ok(g1) = deps
        .pairing
        .g1_generator(G1GeneratorParams, G1GeneratorPayload);
    let Ok(encoded_g1) = deps
        .pairing
        .encode_g1(EncodeG1Params, EncodeG1Payload { point: g1.point });
    entries.push(entry(
        "G1_GENERATOR",
        SolidityConstantValue::Bytes(encoded_g1.bytes.as_ref().to_vec()),
    )?);
    let Ok(g2) = deps
        .pairing
        .g2_generator(G2GeneratorParams, G2GeneratorPayload);
    let Ok(encoded_g2) = deps
        .pairing
        .encode_g2(EncodeG2Params, EncodeG2Payload { point: g2.point });
    entries.push(entry(
        "G2_GENERATOR",
        SolidityConstantValue::Bytes(encoded_g2.bytes.as_ref().to_vec()),
    )?);

    let second_group_arithmetic = match P::DECLARATION.verifier_group_arithmetic {
        VerifierGroupArithmetic::BothGroups => true,
        VerifierGroupArithmetic::FirstGroupOnly => false,
    };
    entries.push(entry(
        "SECOND_GROUP_ARITHMETIC",
        SolidityConstantValue::Bool(second_group_arithmetic),
    )?);

    Ok(ConstantsSuccessReturn { entries })
}
