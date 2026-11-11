mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test_basic;
#[cfg(test)]
mod test_error_decode;
#[cfg(test)]
mod test_error_shape;

use encoding::{
    CanonicalFieldKind, CanonicalFieldReader, CanonicalFieldValue, CanonicalFields,
    FromFieldsParams, FromFieldsReturn, FromFieldsSuccessReturn, IEncodingContract, ToFieldsParams,
    ToFieldsReturn, ToFieldsSuccessReturn,
};
use interface::{
    POSSESSION_STATEMENT_FIELD_KINDS, PossessionG1Statement, PossessionG1StatementDescription,
    PossessionG1StatementDescriptionConstructorParams,
    PossessionG1StatementDescriptionTryNewReturn, PossessionG2Statement,
    PossessionG2StatementDescription, PossessionG2StatementDescriptionConstructorParams,
    PossessionG2StatementDescriptionTryNewReturn, PossessionStatementFromFieldsErrorReturn,
};
use pairing::{
    DecodeG1Params, DecodeG2Params, EncodeG1Params, EncodeG1Payload, EncodeG2Params,
    EncodeG2Payload, IPairingAdapter,
};

impl<'a, P: IPairingAdapter> PossessionG1StatementDescription<'a, P> {
    pub fn try_new(
        params: PossessionG1StatementDescriptionConstructorParams<'a, P>,
    ) -> PossessionG1StatementDescriptionTryNewReturn<'a, P> {
        Ok(PossessionG1StatementDescription {
            pairing: params.pairing,
        })
    }
}

impl<'a, P: IPairingAdapter> PossessionG2StatementDescription<'a, P> {
    pub fn try_new(
        params: PossessionG2StatementDescriptionConstructorParams<'a, P>,
    ) -> PossessionG2StatementDescriptionTryNewReturn<'a, P> {
        Ok(PossessionG2StatementDescription {
            pairing: params.pairing,
        })
    }
}

impl<P: IPairingAdapter> IEncodingContract for PossessionG1StatementDescription<'_, P> {
    type Described = PossessionG1Statement<P>;
    type FromFieldsErrorReturn = PossessionStatementFromFieldsErrorReturn;
    const FIELDS: &'static [CanonicalFieldKind] = &POSSESSION_STATEMENT_FIELD_KINDS;

    fn to_fields(&self, _params: ToFieldsParams, payload: &Self::Described) -> ToFieldsReturn {
        let Ok(key) = self.pairing.encode_g1(
            EncodeG1Params,
            EncodeG1Payload {
                point: payload.key.clone(),
            },
        );
        let Ok(commitment) = self.pairing.encode_g1(
            EncodeG1Params,
            EncodeG1Payload {
                point: payload.commitment.clone(),
            },
        );
        Ok(ToFieldsSuccessReturn {
            fields: CanonicalFields {
                values: vec![
                    CanonicalFieldValue::Bytes(key.bytes.as_ref().to_vec()),
                    CanonicalFieldValue::Bytes(commitment.bytes.as_ref().to_vec()),
                ],
            },
        })
    }

    fn fields_to_value(
        &self,
        _params: FromFieldsParams,
        payload: CanonicalFields,
    ) -> FromFieldsReturn<Self::Described, Self::FromFieldsErrorReturn> {
        let mut reader = match CanonicalFieldReader::try_new(payload, Self::FIELDS) {
            Err(error) => return Err(PossessionStatementFromFieldsErrorReturn::Fields(error)),
            Ok(reader) => reader,
        };
        let key_bytes = match reader.read_bytes() {
            Err(error) => return Err(PossessionStatementFromFieldsErrorReturn::Fields(error)),
            Ok(bytes) => bytes,
        };
        let commitment_bytes = match reader.read_bytes() {
            Err(error) => return Err(PossessionStatementFromFieldsErrorReturn::Fields(error)),
            Ok(bytes) => bytes,
        };
        let key = match self.pairing.decode_g1(DecodeG1Params, &key_bytes) {
            Ok(success) => success.point,
            Err(error) => {
                return Err(PossessionStatementFromFieldsErrorReturn::InvalidG1 {
                    index: 0,
                    error,
                });
            }
        };
        let commitment = match self.pairing.decode_g1(DecodeG1Params, &commitment_bytes) {
            Ok(success) => success.point,
            Err(error) => {
                return Err(PossessionStatementFromFieldsErrorReturn::InvalidG1 {
                    index: 1,
                    error,
                });
            }
        };
        Ok(FromFieldsSuccessReturn {
            described: PossessionG1Statement { key, commitment },
        })
    }
}

impl<P: IPairingAdapter> IEncodingContract for PossessionG2StatementDescription<'_, P> {
    type Described = PossessionG2Statement<P>;
    type FromFieldsErrorReturn = PossessionStatementFromFieldsErrorReturn;
    const FIELDS: &'static [CanonicalFieldKind] = &POSSESSION_STATEMENT_FIELD_KINDS;

    fn to_fields(&self, _params: ToFieldsParams, payload: &Self::Described) -> ToFieldsReturn {
        let Ok(key) = self.pairing.encode_g2(
            EncodeG2Params,
            EncodeG2Payload {
                point: payload.key.clone(),
            },
        );
        let Ok(commitment) = self.pairing.encode_g2(
            EncodeG2Params,
            EncodeG2Payload {
                point: payload.commitment.clone(),
            },
        );
        Ok(ToFieldsSuccessReturn {
            fields: CanonicalFields {
                values: vec![
                    CanonicalFieldValue::Bytes(key.bytes.as_ref().to_vec()),
                    CanonicalFieldValue::Bytes(commitment.bytes.as_ref().to_vec()),
                ],
            },
        })
    }

    fn fields_to_value(
        &self,
        _params: FromFieldsParams,
        payload: CanonicalFields,
    ) -> FromFieldsReturn<Self::Described, Self::FromFieldsErrorReturn> {
        let mut reader = match CanonicalFieldReader::try_new(payload, Self::FIELDS) {
            Err(error) => return Err(PossessionStatementFromFieldsErrorReturn::Fields(error)),
            Ok(reader) => reader,
        };
        let key_bytes = match reader.read_bytes() {
            Err(error) => return Err(PossessionStatementFromFieldsErrorReturn::Fields(error)),
            Ok(bytes) => bytes,
        };
        let commitment_bytes = match reader.read_bytes() {
            Err(error) => return Err(PossessionStatementFromFieldsErrorReturn::Fields(error)),
            Ok(bytes) => bytes,
        };
        let key = match self.pairing.decode_g2(DecodeG2Params, &key_bytes) {
            Ok(success) => success.point,
            Err(error) => {
                return Err(PossessionStatementFromFieldsErrorReturn::InvalidG2 {
                    index: 0,
                    error,
                });
            }
        };
        let commitment = match self.pairing.decode_g2(DecodeG2Params, &commitment_bytes) {
            Ok(success) => success.point,
            Err(error) => {
                return Err(PossessionStatementFromFieldsErrorReturn::InvalidG2 {
                    index: 1,
                    error,
                });
            }
        };
        Ok(FromFieldsSuccessReturn {
            described: PossessionG2Statement { key, commitment },
        })
    }
}
