mod interface;
#[cfg(test)]
mod mock;
pub(crate) mod provides;
#[cfg(test)]
mod test;

use crate::factory::provides::{
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFields, DecodeErrorReturn, DecodeParams,
    DecodeReturn, DecodeSuccessReturn, ENCODING_INTERFACE_VERSION, EncodeErrorReturn, EncodeParams,
    EncodeReturn, EncodeSuccessReturn, EncodingDeclaration, EncodingIdentifier, EncodingTypeName,
    EncodingTypeNameConstructorParams, FromFieldsParams, IDecoderAdapter, IEncoderAdapter,
    IEncodingContract, ToFieldsParams, TypeNameParams, TypeNameReturn, TypeNameSuccessReturn,
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
    const DECLARATION: EncodingDeclaration = AbiEncoding::DECLARATION;

    fn encode<D: IEncodingContract>(
        &self,
        params: EncodeParams<'_, D>,
        payload: &D::Described,
    ) -> EncodeReturn {
        let Ok(emitted) = params.description.to_fields(ToFieldsParams, payload);
        if emitted.fields.values.len() != D::FIELDS.len() {
            return Err(EncodeErrorReturn::FieldCount {
                expected: D::FIELDS.len(),
                actual: emitted.fields.values.len(),
            });
        }
        for (index, (value, expected)) in emitted
            .fields
            .values
            .iter()
            .zip(D::FIELDS.iter())
            .enumerate()
        {
            let actual = match value {
                CanonicalFieldValue::FixedBytes32(_) => CanonicalFieldKind::FixedBytes32,
                CanonicalFieldValue::Unsigned16(_) => CanonicalFieldKind::Unsigned16,
                CanonicalFieldValue::Unsigned32(_) => CanonicalFieldKind::Unsigned32,
                CanonicalFieldValue::Unsigned64(_) => CanonicalFieldKind::Unsigned64,
                CanonicalFieldValue::Text(_) => CanonicalFieldKind::Text,
                CanonicalFieldValue::Bytes(_) => CanonicalFieldKind::Bytes,
                CanonicalFieldValue::FixedBytes20(_) => CanonicalFieldKind::FixedBytes20,
                CanonicalFieldValue::Unsigned256(_) => CanonicalFieldKind::Unsigned256,
            };
            if actual != *expected {
                return Err(EncodeErrorReturn::FieldKind {
                    index,
                    expected: *expected,
                    actual,
                });
            }
        }
        let tuple = DynSolValue::Tuple(
            emitted
                .fields
                .values
                .into_iter()
                .map(|value| match value {
                    CanonicalFieldValue::FixedBytes32(bytes) => {
                        DynSolValue::FixedBytes(B256::from(bytes), 32)
                    }
                    CanonicalFieldValue::Unsigned16(value) => {
                        DynSolValue::Uint(U256::from(value), 16)
                    }
                    CanonicalFieldValue::Unsigned32(value) => {
                        DynSolValue::Uint(U256::from(value), 32)
                    }
                    CanonicalFieldValue::Unsigned64(value) => {
                        DynSolValue::Uint(U256::from(value), 64)
                    }
                    CanonicalFieldValue::Text(text) => DynSolValue::String(text),
                    CanonicalFieldValue::Bytes(bytes) => DynSolValue::Bytes(bytes),
                    CanonicalFieldValue::FixedBytes20(bytes) => {
                        DynSolValue::FixedBytes(B256::right_padding_from(&bytes), 20)
                    }
                    CanonicalFieldValue::Unsigned256(bytes) => {
                        DynSolValue::Uint(U256::from_be_bytes(bytes), 256)
                    }
                })
                .collect(),
        );
        Ok(EncodeSuccessReturn {
            bytes: tuple.abi_encode_params(),
        })
    }

    fn type_name(&self, _params: TypeNameParams, payload: CanonicalFieldKind) -> TypeNameReturn {
        let name = match payload {
            CanonicalFieldKind::FixedBytes32 => "bytes32",
            CanonicalFieldKind::Unsigned16 => "uint16",
            CanonicalFieldKind::Unsigned32 => "uint32",
            CanonicalFieldKind::Unsigned64 => "uint64",
            CanonicalFieldKind::Text => "string",
            CanonicalFieldKind::Bytes => "bytes",
            CanonicalFieldKind::FixedBytes20 => "bytes20",
            CanonicalFieldKind::Unsigned256 => "uint256",
        };
        let Ok(name) = EncodingTypeName::try_new(EncodingTypeNameConstructorParams {
            name: name.to_string(),
        }) else {
            unreachable!("every name the table states is a valid encoding type name")
        };
        Ok(TypeNameSuccessReturn { name })
    }
}

impl IDecoderAdapter for AbiEncoding {
    fn decode<D: IEncodingContract>(
        &self,
        params: DecodeParams<'_, D>,
        payload: &[u8],
    ) -> DecodeReturn<D::Described, D::FromFieldsErrorReturn> {
        let tuple = DynSolType::Tuple(
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
        let decoded = match tuple.abi_decode_params(payload) {
            Err(error) => {
                return Err(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::Malformed(
                    error,
                )));
            }
            Ok(decoded) => decoded,
        };
        let mut values = Vec::with_capacity(D::FIELDS.len());
        for (index, kind) in D::FIELDS.iter().enumerate() {
            let Some(item) = decoded.as_tuple().and_then(|items| items.get(index)) else {
                return Err(DecodeErrorReturn::Abi(
                    AbiDecoderErrorReturn::DecodedShapeMismatch { index },
                ));
            };
            let value = match kind {
                CanonicalFieldKind::FixedBytes32 => {
                    let Some((word, 32)) = item.as_fixed_bytes() else {
                        return Err(DecodeErrorReturn::Abi(
                            AbiDecoderErrorReturn::DecodedShapeMismatch { index },
                        ));
                    };
                    let Ok(bytes) = <[u8; 32]>::try_from(word) else {
                        return Err(DecodeErrorReturn::Abi(
                            AbiDecoderErrorReturn::DecodedShapeMismatch { index },
                        ));
                    };
                    CanonicalFieldValue::FixedBytes32(bytes)
                }
                CanonicalFieldKind::Unsigned16 => {
                    let Some((word, 16)) = item.as_uint() else {
                        return Err(DecodeErrorReturn::Abi(
                            AbiDecoderErrorReturn::DecodedShapeMismatch { index },
                        ));
                    };
                    let Ok(value) = u16::try_from(word) else {
                        return Err(DecodeErrorReturn::Abi(
                            AbiDecoderErrorReturn::ValueOutOfRange {
                                index,
                                kind: CanonicalFieldKind::Unsigned16,
                            },
                        ));
                    };
                    CanonicalFieldValue::Unsigned16(value)
                }
                CanonicalFieldKind::Unsigned32 => {
                    let Some((word, 32)) = item.as_uint() else {
                        return Err(DecodeErrorReturn::Abi(
                            AbiDecoderErrorReturn::DecodedShapeMismatch { index },
                        ));
                    };
                    let Ok(value) = u32::try_from(word) else {
                        return Err(DecodeErrorReturn::Abi(
                            AbiDecoderErrorReturn::ValueOutOfRange {
                                index,
                                kind: CanonicalFieldKind::Unsigned32,
                            },
                        ));
                    };
                    CanonicalFieldValue::Unsigned32(value)
                }
                CanonicalFieldKind::Unsigned64 => {
                    let Some((word, 64)) = item.as_uint() else {
                        return Err(DecodeErrorReturn::Abi(
                            AbiDecoderErrorReturn::DecodedShapeMismatch { index },
                        ));
                    };
                    let Ok(value) = u64::try_from(word) else {
                        return Err(DecodeErrorReturn::Abi(
                            AbiDecoderErrorReturn::ValueOutOfRange {
                                index,
                                kind: CanonicalFieldKind::Unsigned64,
                            },
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
                    let Some((word, 20)) = item.as_fixed_bytes() else {
                        return Err(DecodeErrorReturn::Abi(
                            AbiDecoderErrorReturn::DecodedShapeMismatch { index },
                        ));
                    };
                    let Some(head) = word.get(..20) else {
                        return Err(DecodeErrorReturn::Abi(
                            AbiDecoderErrorReturn::DecodedShapeMismatch { index },
                        ));
                    };
                    let Ok(bytes) = <[u8; 20]>::try_from(head) else {
                        return Err(DecodeErrorReturn::Abi(
                            AbiDecoderErrorReturn::DecodedShapeMismatch { index },
                        ));
                    };
                    CanonicalFieldValue::FixedBytes20(bytes)
                }
                CanonicalFieldKind::Unsigned256 => {
                    let Some((word, 256)) = item.as_uint() else {
                        return Err(DecodeErrorReturn::Abi(
                            AbiDecoderErrorReturn::DecodedShapeMismatch { index },
                        ));
                    };
                    CanonicalFieldValue::Unsigned256(word.to_be_bytes::<32>())
                }
            };
            values.push(value);
        }
        let described = match params
            .description
            .fields_to_value(FromFieldsParams, CanonicalFields { values })
        {
            Err(error) => return Err(DecodeErrorReturn::Description(error)),
            Ok(success) => success.described,
        };
        let re_encoded = match self.encode(
            EncodeParams {
                description: params.description,
            },
            &described,
        ) {
            Err(error) => return Err(DecodeErrorReturn::EncoderContract(error)),
            Ok(success) => success.bytes,
        };
        if re_encoded.as_slice() != payload {
            return Err(DecodeErrorReturn::NonCanonical);
        }
        Ok(DecodeSuccessReturn { described })
    }
}
