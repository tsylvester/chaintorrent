mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::factory::provides::{
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFields, FromFieldParams, ICanonicalField,
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
