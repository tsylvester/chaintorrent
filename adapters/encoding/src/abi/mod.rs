mod interface;
pub(crate) mod provides;
#[cfg(test)]
mod test;

use crate::factory::provides::{
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFields, DecodeErrorReturn, DecodeParams,
    DecodeReturn, DecodeSuccessReturn, ENCODING_INTERFACE_VERSION, EncodeParams, EncodeReturn,
    EncodeSuccessReturn, EncodingDeclaration, EncodingIdentifier, FromFieldsParams,
    IDecoderAdapter, IEncoderAdapter, IEncodingContract, ToFieldsParams,
};
use alloy::dyn_abi::{DynSolType, DynSolValue};
use alloy::primitives::{B256, U256};
use interface::{
    AbiDecoderErrorReturn, AbiEncoding, AbiEncodingConstructorParams, AbiEncodingTryNewReturn,
};

impl AbiEncoding {
    pub const DECLARATION: EncodingDeclaration = EncodingDeclaration {
        identifier: EncodingIdentifier::EthereumAbiV1,
        adapter_version: 1,
        interface_version: ENCODING_INTERFACE_VERSION,
    };

    pub fn try_new(_params: AbiEncodingConstructorParams) -> AbiEncodingTryNewReturn {
        Ok(AbiEncoding)
    }
}

impl IEncoderAdapter for AbiEncoding {
    fn encode<D: IEncodingContract>(
        &self,
        params: EncodeParams<'_, D>,
        payload: &D::Described,
    ) -> EncodeReturn {
        let Ok(success) = params.description.to_fields(ToFieldsParams, payload);
        let items = success
            .fields
            .values
            .into_iter()
            .map(|value| match value {
                CanonicalFieldValue::FixedBytes32(bytes) => {
                    DynSolValue::FixedBytes(B256::from(bytes), 32)
                }
                CanonicalFieldValue::Unsigned16(value) => DynSolValue::Uint(U256::from(value), 16),
                CanonicalFieldValue::Unsigned32(value) => DynSolValue::Uint(U256::from(value), 32),
                CanonicalFieldValue::Unsigned64(value) => DynSolValue::Uint(U256::from(value), 64),
                CanonicalFieldValue::Text(text) => DynSolValue::String(text),
                CanonicalFieldValue::Bytes(bytes) => DynSolValue::Bytes(bytes),
                CanonicalFieldValue::FixedBytes20(bytes) => {
                    DynSolValue::FixedBytes(B256::right_padding_from(&bytes), 20)
                }
                CanonicalFieldValue::Unsigned256(bytes) => {
                    DynSolValue::Uint(U256::from_be_bytes(bytes), 256)
                }
            })
            .collect();
        Ok(EncodeSuccessReturn {
            bytes: DynSolValue::Tuple(items).abi_encode_params(),
        })
    }
}

impl IDecoderAdapter for AbiEncoding {
    fn decode<D: IEncodingContract>(
        &self,
        params: DecodeParams<'_, D>,
        payload: &[u8],
    ) -> DecodeReturn<D::Described, D::FromFieldsErrorReturn> {
        let sol_type = DynSolType::Tuple(
            D::FIELDS
                .iter()
                .map(|kind| match kind {
                    CanonicalFieldKind::FixedBytes32 => DynSolType::FixedBytes(32),
                    CanonicalFieldKind::Unsigned16 => DynSolType::Uint(16),
                    CanonicalFieldKind::Unsigned32 => DynSolType::Uint(32),
                    CanonicalFieldKind::Unsigned64 => DynSolType::Uint(64),
                    CanonicalFieldKind::Text => DynSolType::String,
                    CanonicalFieldKind::Bytes => DynSolType::Bytes,
                    CanonicalFieldKind::FixedBytes20 => DynSolType::FixedBytes(20),
                    CanonicalFieldKind::Unsigned256 => DynSolType::Uint(256),
                })
                .collect(),
        );
        let decoded = match sol_type.abi_decode_params(payload) {
            Ok(decoded) => decoded,
            Err(error) => {
                return Err(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::Malformed(
                    error,
                )));
            }
        };
        let Some(items) = decoded.as_tuple() else {
            return Err(DecodeErrorReturn::Abi(
                AbiDecoderErrorReturn::DecodedShapeMismatch { index: 0 },
            ));
        };

        let mut values = Vec::with_capacity(D::FIELDS.len());
        for (index, kind) in D::FIELDS.iter().enumerate() {
            let Some(item) = items.get(index) else {
                return Err(DecodeErrorReturn::Abi(
                    AbiDecoderErrorReturn::DecodedShapeMismatch { index },
                ));
            };
            let value = match kind {
                CanonicalFieldKind::FixedBytes32 => {
                    let Some((bytes, size)) = item.as_fixed_bytes() else {
                        return Err(DecodeErrorReturn::Abi(
                            AbiDecoderErrorReturn::DecodedShapeMismatch { index },
                        ));
                    };
                    if size != 32 {
                        return Err(DecodeErrorReturn::Abi(
                            AbiDecoderErrorReturn::DecodedShapeMismatch { index },
                        ));
                    }
                    let Ok(fixed) = <[u8; 32]>::try_from(bytes) else {
                        return Err(DecodeErrorReturn::Abi(
                            AbiDecoderErrorReturn::DecodedShapeMismatch { index },
                        ));
                    };
                    CanonicalFieldValue::FixedBytes32(fixed)
                }
                CanonicalFieldKind::Unsigned16 => {
                    let Some((word, width)) = item.as_uint() else {
                        return Err(DecodeErrorReturn::Abi(
                            AbiDecoderErrorReturn::DecodedShapeMismatch { index },
                        ));
                    };
                    if width != 16 {
                        return Err(DecodeErrorReturn::Abi(
                            AbiDecoderErrorReturn::DecodedShapeMismatch { index },
                        ));
                    }
                    let Ok(value) = u16::try_from(word) else {
                        return Err(DecodeErrorReturn::Abi(
                            AbiDecoderErrorReturn::ValueOutOfRange { index, kind: *kind },
                        ));
                    };
                    CanonicalFieldValue::Unsigned16(value)
                }
                CanonicalFieldKind::Unsigned32 => {
                    let Some((word, width)) = item.as_uint() else {
                        return Err(DecodeErrorReturn::Abi(
                            AbiDecoderErrorReturn::DecodedShapeMismatch { index },
                        ));
                    };
                    if width != 32 {
                        return Err(DecodeErrorReturn::Abi(
                            AbiDecoderErrorReturn::DecodedShapeMismatch { index },
                        ));
                    }
                    let Ok(value) = u32::try_from(word) else {
                        return Err(DecodeErrorReturn::Abi(
                            AbiDecoderErrorReturn::ValueOutOfRange { index, kind: *kind },
                        ));
                    };
                    CanonicalFieldValue::Unsigned32(value)
                }
                CanonicalFieldKind::Unsigned64 => {
                    let Some((word, width)) = item.as_uint() else {
                        return Err(DecodeErrorReturn::Abi(
                            AbiDecoderErrorReturn::DecodedShapeMismatch { index },
                        ));
                    };
                    if width != 64 {
                        return Err(DecodeErrorReturn::Abi(
                            AbiDecoderErrorReturn::DecodedShapeMismatch { index },
                        ));
                    }
                    let Ok(value) = u64::try_from(word) else {
                        return Err(DecodeErrorReturn::Abi(
                            AbiDecoderErrorReturn::ValueOutOfRange { index, kind: *kind },
                        ));
                    };
                    CanonicalFieldValue::Unsigned64(value)
                }
                CanonicalFieldKind::Text => {
                    let Some(text) = item.as_str() else {
                        return Err(DecodeErrorReturn::Abi(
                            AbiDecoderErrorReturn::DecodedShapeMismatch { index },
                        ));
                    };
                    CanonicalFieldValue::Text(text.to_string())
                }
                CanonicalFieldKind::Bytes => {
                    let Some(bytes) = item.as_bytes() else {
                        return Err(DecodeErrorReturn::Abi(
                            AbiDecoderErrorReturn::DecodedShapeMismatch { index },
                        ));
                    };
                    CanonicalFieldValue::Bytes(bytes.to_vec())
                }
                CanonicalFieldKind::FixedBytes20 => {
                    let Some((bytes, size)) = item.as_fixed_bytes() else {
                        return Err(DecodeErrorReturn::Abi(
                            AbiDecoderErrorReturn::DecodedShapeMismatch { index },
                        ));
                    };
                    if size != 20 {
                        return Err(DecodeErrorReturn::Abi(
                            AbiDecoderErrorReturn::DecodedShapeMismatch { index },
                        ));
                    }
                    let Some(head) = bytes.get(..20) else {
                        return Err(DecodeErrorReturn::Abi(
                            AbiDecoderErrorReturn::DecodedShapeMismatch { index },
                        ));
                    };
                    let Ok(fixed) = <[u8; 20]>::try_from(head) else {
                        return Err(DecodeErrorReturn::Abi(
                            AbiDecoderErrorReturn::DecodedShapeMismatch { index },
                        ));
                    };
                    CanonicalFieldValue::FixedBytes20(fixed)
                }
                CanonicalFieldKind::Unsigned256 => {
                    let Some((word, width)) = item.as_uint() else {
                        return Err(DecodeErrorReturn::Abi(
                            AbiDecoderErrorReturn::DecodedShapeMismatch { index },
                        ));
                    };
                    if width != 256 {
                        return Err(DecodeErrorReturn::Abi(
                            AbiDecoderErrorReturn::DecodedShapeMismatch { index },
                        ));
                    }
                    CanonicalFieldValue::Unsigned256(word.to_be_bytes::<32>())
                }
            };
            values.push(value);
        }

        let described = match params
            .description
            .fields_to_value(FromFieldsParams, CanonicalFields { values })
        {
            Ok(success) => success.described,
            Err(error) => return Err(DecodeErrorReturn::Description(error)),
        };

        let Ok(re_encoded) = self.encode(
            EncodeParams {
                description: params.description,
            },
            &described,
        );
        if re_encoded.bytes != payload {
            return Err(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::NonCanonical));
        }
        Ok(DecodeSuccessReturn { described })
    }
}
