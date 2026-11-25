mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::factory::provides::{
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFields, EncodingConcrete, EncodingIdentifier,
    EncodingTypeName, FromFieldParams, ICanonicalField,
};
use interface::{
    CanonicalFieldReader, CanonicalFieldReaderErrorReturn, CanonicalFieldReaderReadReturn,
    CanonicalFieldReaderTryNewReturn,
};

impl CanonicalFieldReader {
    pub fn try_new(
        payload: CanonicalFields,
        fields: &'static [CanonicalFieldKind],
    ) -> CanonicalFieldReaderTryNewReturn {
        if payload.values.len() != fields.len() {
            return Err(CanonicalFieldReaderErrorReturn::Count {
                expected: fields.len(),
                actual: payload.values.len(),
            });
        }
        Ok(CanonicalFieldReader {
            values: payload.values,
            fields,
            index: 0,
        })
    }

    pub fn read_value(&mut self) -> CanonicalFieldReaderReadReturn<CanonicalFieldValue> {
        let Some(value) = self.values.get(self.index) else {
            return Err(CanonicalFieldReaderErrorReturn::Count {
                expected: self.fields.len(),
                actual: self.index + 1,
            });
        };
        let Some(expected) = self.fields.get(self.index) else {
            return Err(CanonicalFieldReaderErrorReturn::Count {
                expected: self.fields.len(),
                actual: self.index + 1,
            });
        };
        let actual = value.kind();
        if actual != *expected {
            return Err(CanonicalFieldReaderErrorReturn::Kind {
                index: self.index,
                expected: *expected,
                actual,
            });
        }
        let value = value.clone();
        self.index += 1;
        Ok(value)
    }

    pub fn read_fixed_bytes32(&mut self) -> CanonicalFieldReaderReadReturn<[u8; 32]> {
        let index = self.index;
        let value = self.read_value()?;
        let actual = value.kind();
        let CanonicalFieldValue::FixedBytes32(contents) = value else {
            let Some(expected) = self.fields.get(index).copied() else {
                return Err(CanonicalFieldReaderErrorReturn::Count {
                    expected: self.fields.len(),
                    actual: index + 1,
                });
            };
            return Err(CanonicalFieldReaderErrorReturn::Kind {
                index,
                expected,
                actual,
            });
        };
        Ok(contents)
    }

    pub fn read_unsigned16(&mut self) -> CanonicalFieldReaderReadReturn<u16> {
        let index = self.index;
        let value = self.read_value()?;
        let actual = value.kind();
        let CanonicalFieldValue::Unsigned16(contents) = value else {
            let Some(expected) = self.fields.get(index).copied() else {
                return Err(CanonicalFieldReaderErrorReturn::Count {
                    expected: self.fields.len(),
                    actual: index + 1,
                });
            };
            return Err(CanonicalFieldReaderErrorReturn::Kind {
                index,
                expected,
                actual,
            });
        };
        Ok(contents)
    }

    pub fn read_unsigned32(&mut self) -> CanonicalFieldReaderReadReturn<u32> {
        let index = self.index;
        let value = self.read_value()?;
        let actual = value.kind();
        let CanonicalFieldValue::Unsigned32(contents) = value else {
            let Some(expected) = self.fields.get(index).copied() else {
                return Err(CanonicalFieldReaderErrorReturn::Count {
                    expected: self.fields.len(),
                    actual: index + 1,
                });
            };
            return Err(CanonicalFieldReaderErrorReturn::Kind {
                index,
                expected,
                actual,
            });
        };
        Ok(contents)
    }

    pub fn read_unsigned64(&mut self) -> CanonicalFieldReaderReadReturn<u64> {
        let index = self.index;
        let value = self.read_value()?;
        let actual = value.kind();
        let CanonicalFieldValue::Unsigned64(contents) = value else {
            let Some(expected) = self.fields.get(index).copied() else {
                return Err(CanonicalFieldReaderErrorReturn::Count {
                    expected: self.fields.len(),
                    actual: index + 1,
                });
            };
            return Err(CanonicalFieldReaderErrorReturn::Kind {
                index,
                expected,
                actual,
            });
        };
        Ok(contents)
    }

    pub fn read_text(&mut self) -> CanonicalFieldReaderReadReturn<String> {
        let index = self.index;
        let value = self.read_value()?;
        let actual = value.kind();
        let CanonicalFieldValue::Text(contents) = value else {
            let Some(expected) = self.fields.get(index).copied() else {
                return Err(CanonicalFieldReaderErrorReturn::Count {
                    expected: self.fields.len(),
                    actual: index + 1,
                });
            };
            return Err(CanonicalFieldReaderErrorReturn::Kind {
                index,
                expected,
                actual,
            });
        };
        Ok(contents)
    }

    pub fn read_bytes(&mut self) -> CanonicalFieldReaderReadReturn<Vec<u8>> {
        let index = self.index;
        let value = self.read_value()?;
        let actual = value.kind();
        let CanonicalFieldValue::Bytes(contents) = value else {
            let Some(expected) = self.fields.get(index).copied() else {
                return Err(CanonicalFieldReaderErrorReturn::Count {
                    expected: self.fields.len(),
                    actual: index + 1,
                });
            };
            return Err(CanonicalFieldReaderErrorReturn::Kind {
                index,
                expected,
                actual,
            });
        };
        Ok(contents)
    }

    pub fn read_fixed_bytes20(&mut self) -> CanonicalFieldReaderReadReturn<[u8; 20]> {
        let index = self.index;
        let value = self.read_value()?;
        let actual = value.kind();
        let CanonicalFieldValue::FixedBytes20(contents) = value else {
            let Some(expected) = self.fields.get(index).copied() else {
                return Err(CanonicalFieldReaderErrorReturn::Count {
                    expected: self.fields.len(),
                    actual: index + 1,
                });
            };
            return Err(CanonicalFieldReaderErrorReturn::Kind {
                index,
                expected,
                actual,
            });
        };
        Ok(contents)
    }

    pub fn read_unsigned256(&mut self) -> CanonicalFieldReaderReadReturn<[u8; 32]> {
        let index = self.index;
        let value = self.read_value()?;
        let actual = value.kind();
        let CanonicalFieldValue::Unsigned256(contents) = value else {
            let Some(expected) = self.fields.get(index).copied() else {
                return Err(CanonicalFieldReaderErrorReturn::Count {
                    expected: self.fields.len(),
                    actual: index + 1,
                });
            };
            return Err(CanonicalFieldReaderErrorReturn::Kind {
                index,
                expected,
                actual,
            });
        };
        Ok(contents)
    }

    pub fn read_form<F: ICanonicalField>(&mut self) -> CanonicalFieldReaderReadReturn<F> {
        let Some(value) = self.values.get(self.index) else {
            return Err(CanonicalFieldReaderErrorReturn::Count {
                expected: self.fields.len(),
                actual: self.index + 1,
            });
        };
        match F::from_field(FromFieldParams, value.clone()) {
            Err(error) => Err(CanonicalFieldReaderErrorReturn::Form {
                index: self.index,
                error,
            }),
            Ok(success) => {
                self.index += 1;
                Ok(success.value)
            }
        }
    }
}

impl CanonicalFieldValue {
    pub fn kind(&self) -> CanonicalFieldKind {
        match self {
            CanonicalFieldValue::FixedBytes32(_) => CanonicalFieldKind::FixedBytes32,
            CanonicalFieldValue::Unsigned16(_) => CanonicalFieldKind::Unsigned16,
            CanonicalFieldValue::Unsigned32(_) => CanonicalFieldKind::Unsigned32,
            CanonicalFieldValue::Unsigned64(_) => CanonicalFieldKind::Unsigned64,
            CanonicalFieldValue::Text(_) => CanonicalFieldKind::Text,
            CanonicalFieldValue::Bytes(_) => CanonicalFieldKind::Bytes,
            CanonicalFieldValue::FixedBytes20(_) => CanonicalFieldKind::FixedBytes20,
            CanonicalFieldValue::Unsigned256(_) => CanonicalFieldKind::Unsigned256,
        }
    }
}

impl CanonicalFieldKind {
    pub const fn name(self) -> &'static str {
        match self {
            CanonicalFieldKind::FixedBytes32 => "fixed_bytes32",
            CanonicalFieldKind::Unsigned16 => "unsigned16",
            CanonicalFieldKind::Unsigned32 => "unsigned32",
            CanonicalFieldKind::Unsigned64 => "unsigned64",
            CanonicalFieldKind::Text => "text",
            CanonicalFieldKind::Bytes => "bytes",
            CanonicalFieldKind::FixedBytes20 => "fixed_bytes20",
            CanonicalFieldKind::Unsigned256 => "unsigned256",
        }
    }

    pub const fn wire_code(self) -> u8 {
        match self {
            CanonicalFieldKind::FixedBytes32 => 1,
            CanonicalFieldKind::Unsigned16 => 2,
            CanonicalFieldKind::Unsigned32 => 3,
            CanonicalFieldKind::Unsigned64 => 4,
            CanonicalFieldKind::Text => 5,
            CanonicalFieldKind::Bytes => 6,
            CanonicalFieldKind::FixedBytes20 => 7,
            CanonicalFieldKind::Unsigned256 => 8,
        }
    }
}

impl EncodingConcrete {
    pub const fn name(self) -> &'static str {
        match self {
            EncodingConcrete::Abi => "abi",
            #[cfg(any(test, feature = "mocks"))]
            EncodingConcrete::Mock => "mock",
        }
    }
}

impl EncodingIdentifier {
    pub const fn name(self) -> &'static str {
        match self {
            EncodingIdentifier::EthereumAbiV1 => "ethereum_abi_v1",
        }
    }

    pub const fn wire_code(self) -> u8 {
        match self {
            EncodingIdentifier::EthereumAbiV1 => 1,
        }
    }
}

impl EncodingTypeName {
    pub const fn name(self) -> &'static str {
        match self {
            EncodingTypeName::Bytes32 => "bytes32",
            EncodingTypeName::Bytes20 => "bytes20",
            EncodingTypeName::Uint16 => "uint16",
            EncodingTypeName::Uint32 => "uint32",
            EncodingTypeName::Uint64 => "uint64",
            EncodingTypeName::Uint256 => "uint256",
            EncodingTypeName::String => "string",
            EncodingTypeName::Bytes => "bytes",
        }
    }

    pub const fn wire_code(self) -> u8 {
        match self {
            EncodingTypeName::Bytes32 => 1,
            EncodingTypeName::Bytes20 => 2,
            EncodingTypeName::Uint16 => 3,
            EncodingTypeName::Uint32 => 4,
            EncodingTypeName::Uint64 => 5,
            EncodingTypeName::Uint256 => 6,
            EncodingTypeName::String => 7,
            EncodingTypeName::Bytes => 8,
        }
    }
}
