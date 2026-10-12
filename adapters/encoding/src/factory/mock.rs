#![allow(clippy::expect_used)]

use super::interface::{
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFields, ConsumeEncodingParams,
    ConsumeEncodingPayload, CreateEncodingDeps, CreateEncodingParams, CreateEncodingPayload,
    CreateEncodingReturn, CreateEncodingSuccessReturn, DecodeErrorReturn, DecodeParams,
    DecodeReturn, DecodeSuccessReturn, ENCODING_INTERFACE_VERSION, EncodeErrorReturn, EncodeParams,
    EncodeReturn, EncodeSuccessReturn, EncodingConcrete, EncodingDeclaration, EncodingIdentifier,
    FromFieldParams, FromFieldReturn, FromFieldSuccessReturn, FromFieldsParams, FromFieldsReturn,
    FromFieldsSuccessReturn, ICanonicalField, IDecoderAdapter, IEncoderAdapter, IEncodingConsumer,
    IEncodingContract, MockIEncodingConsumerOutput, MockIEncodingContractFromFieldsErrorReturn,
    ToFieldParams, ToFieldReturn, ToFieldSuccessReturn, ToFieldsParams, ToFieldsReturn,
    ToFieldsSuccessReturn,
};
use core::convert::Infallible;

#[derive(Default)]
pub struct CanonicalFieldsOverrides {
    pub values: Option<Vec<CanonicalFieldValue>>,
}

pub fn build_canonical_fields(overrides: CanonicalFieldsOverrides) -> CanonicalFields {
    CanonicalFields {
        values: overrides.values.unwrap_or_else(|| {
            vec![
                CanonicalFieldValue::Text("left-pad".to_string()),
                CanonicalFieldValue::Bytes(vec![0xa1, 0xa2, 0xa3]),
                CanonicalFieldValue::FixedBytes32([0x11; 32]),
                CanonicalFieldValue::Unsigned16(0x0102),
                CanonicalFieldValue::Unsigned32(0x01020304),
                CanonicalFieldValue::Unsigned64(0x0102030405060708),
                CanonicalFieldValue::FixedBytes20([0x22; 20]),
                CanonicalFieldValue::Unsigned256({
                    let mut word = [0x00; 32];
                    word[0] = 0x80;
                    word
                }),
            ]
        }),
    }
}

#[derive(Default)]
pub struct ToFieldsParamsOverrides;

pub fn build_to_fields_params(_overrides: ToFieldsParamsOverrides) -> ToFieldsParams {
    ToFieldsParams
}

#[derive(Default)]
pub struct ToFieldsSuccessReturnOverrides {
    pub fields: Option<CanonicalFields>,
}

pub fn build_to_fields_success_return(
    overrides: ToFieldsSuccessReturnOverrides,
) -> ToFieldsSuccessReturn {
    ToFieldsSuccessReturn {
        fields: overrides
            .fields
            .unwrap_or_else(|| build_canonical_fields(Default::default())),
    }
}

#[derive(Default)]
pub struct FromFieldsParamsOverrides;

pub fn build_from_fields_params(_overrides: FromFieldsParamsOverrides) -> FromFieldsParams {
    FromFieldsParams
}

#[derive(Default)]
pub struct FromFieldsSuccessReturnOverrides {
    pub described: Option<CanonicalFields>,
}

pub fn build_from_fields_success_return(
    overrides: FromFieldsSuccessReturnOverrides,
) -> FromFieldsSuccessReturn<CanonicalFields> {
    FromFieldsSuccessReturn {
        described: overrides
            .described
            .unwrap_or_else(|| build_canonical_fields(Default::default())),
    }
}

pub struct MockIEncodingContract;

impl IEncodingContract for MockIEncodingContract {
    type Described = CanonicalFields;
    type FromFieldsErrorReturn = MockIEncodingContractFromFieldsErrorReturn;
    const FIELDS: &'static [CanonicalFieldKind] = &[
        CanonicalFieldKind::Text,
        CanonicalFieldKind::Bytes,
        CanonicalFieldKind::FixedBytes32,
        CanonicalFieldKind::Unsigned16,
        CanonicalFieldKind::Unsigned32,
        CanonicalFieldKind::Unsigned64,
        CanonicalFieldKind::FixedBytes20,
        CanonicalFieldKind::Unsigned256,
    ];

    fn to_fields(&self, _params: ToFieldsParams, payload: &Self::Described) -> ToFieldsReturn {
        Ok(ToFieldsSuccessReturn {
            fields: payload.clone(),
        })
    }

    fn fields_to_value(
        &self,
        _params: FromFieldsParams,
        payload: CanonicalFields,
    ) -> FromFieldsReturn<Self::Described, Self::FromFieldsErrorReturn> {
        if let Some(index) = payload
            .values
            .iter()
            .position(|value| matches!(value, CanonicalFieldValue::Text(text) if text.is_empty()))
        {
            return Err(MockIEncodingContractFromFieldsErrorReturn::EmptyText { index });
        }
        Ok(FromFieldsSuccessReturn { described: payload })
    }
}

#[derive(Default)]
pub struct EncodingDeclarationOverrides {
    pub identifier: Option<EncodingIdentifier>,
    pub adapter_version: Option<u32>,
    pub interface_version: Option<u32>,
}

pub fn build_encoding_declaration(overrides: EncodingDeclarationOverrides) -> EncodingDeclaration {
    EncodingDeclaration {
        identifier: overrides.identifier.unwrap_or(EncodingIdentifier::Mock),
        adapter_version: overrides.adapter_version.unwrap_or(1),
        interface_version: overrides
            .interface_version
            .unwrap_or(ENCODING_INTERFACE_VERSION),
    }
}

#[derive(Default)]
pub struct EncodeSuccessReturnOverrides {
    pub bytes: Option<Vec<u8>>,
}

pub fn build_encode_success_return(overrides: EncodeSuccessReturnOverrides) -> EncodeSuccessReturn {
    EncodeSuccessReturn {
        bytes: overrides.bytes.unwrap_or_else(|| vec![0x01, 0x02, 0x03]),
    }
}

pub fn build_encode_params<D: IEncodingContract>(description: &D) -> EncodeParams<'_, D> {
    EncodeParams { description }
}

pub fn build_decode_params<D: IEncodingContract>(description: &D) -> DecodeParams<'_, D> {
    DecodeParams { description }
}

pub fn build_decode_success_return<T>(described: T) -> DecodeSuccessReturn<T> {
    DecodeSuccessReturn { described }
}

pub(crate) struct MockIEncoderAdapter;

impl IEncoderAdapter for MockIEncoderAdapter {
    const DECLARATION: EncodingDeclaration = EncodingDeclaration {
        identifier: EncodingIdentifier::Mock,
        adapter_version: 1,
        interface_version: ENCODING_INTERFACE_VERSION,
    };

    fn encode<D: IEncodingContract>(
        &self,
        params: EncodeParams<'_, D>,
        payload: &D::Described,
    ) -> EncodeReturn {
        let Ok(success) = params.description.to_fields(ToFieldsParams, payload);
        let values = success.fields.values;
        if values.len() != D::FIELDS.len() {
            return Err(EncodeErrorReturn::FieldCount {
                expected: D::FIELDS.len(),
                actual: values.len(),
            });
        }
        let mut bytes = Vec::new();
        for (index, (value, expected)) in values.into_iter().zip(D::FIELDS.iter()).enumerate() {
            let (actual, encoded) = match value {
                CanonicalFieldValue::FixedBytes32(value) => {
                    (CanonicalFieldKind::FixedBytes32, value.to_vec())
                }
                CanonicalFieldValue::Unsigned16(value) => {
                    (CanonicalFieldKind::Unsigned16, value.to_be_bytes().to_vec())
                }
                CanonicalFieldValue::Unsigned32(value) => {
                    (CanonicalFieldKind::Unsigned32, value.to_be_bytes().to_vec())
                }
                CanonicalFieldValue::Unsigned64(value) => {
                    (CanonicalFieldKind::Unsigned64, value.to_be_bytes().to_vec())
                }
                CanonicalFieldValue::Text(value) => {
                    let mut encoded = u32::try_from(value.len())
                        .expect("a mock field's length fits a u32")
                        .to_be_bytes()
                        .to_vec();
                    encoded.extend_from_slice(value.as_bytes());
                    (CanonicalFieldKind::Text, encoded)
                }
                CanonicalFieldValue::Bytes(value) => {
                    let mut encoded = u32::try_from(value.len())
                        .expect("a mock field's length fits a u32")
                        .to_be_bytes()
                        .to_vec();
                    encoded.extend_from_slice(&value);
                    (CanonicalFieldKind::Bytes, encoded)
                }
                CanonicalFieldValue::FixedBytes20(value) => {
                    (CanonicalFieldKind::FixedBytes20, value.to_vec())
                }
                CanonicalFieldValue::Unsigned256(value) => {
                    (CanonicalFieldKind::Unsigned256, value.to_vec())
                }
            };
            if actual != *expected {
                return Err(EncodeErrorReturn::FieldKind {
                    index,
                    expected: *expected,
                    actual,
                });
            }
            bytes.extend_from_slice(&encoded);
        }
        Ok(EncodeSuccessReturn { bytes })
    }
}

impl IDecoderAdapter for MockIEncoderAdapter {
    fn decode<D: IEncodingContract>(
        &self,
        params: DecodeParams<'_, D>,
        payload: &[u8],
    ) -> DecodeReturn<D::Described, D::FromFieldsErrorReturn> {
        let mut rest = payload;
        let mut values = Vec::with_capacity(D::FIELDS.len());
        for kind in D::FIELDS {
            let value = match kind {
                CanonicalFieldKind::FixedBytes32 => {
                    let Some(head) = take(&mut rest, 32) else {
                        return Err(DecodeErrorReturn::NonCanonical);
                    };
                    CanonicalFieldValue::FixedBytes32(
                        <[u8; 32]>::try_from(head).expect("the taken slice is the field's width"),
                    )
                }
                CanonicalFieldKind::Unsigned16 => {
                    let Some(head) = take(&mut rest, 2) else {
                        return Err(DecodeErrorReturn::NonCanonical);
                    };
                    CanonicalFieldValue::Unsigned16(u16::from_be_bytes(
                        <[u8; 2]>::try_from(head).expect("the taken slice is the field's width"),
                    ))
                }
                CanonicalFieldKind::Unsigned32 => {
                    let Some(head) = take(&mut rest, 4) else {
                        return Err(DecodeErrorReturn::NonCanonical);
                    };
                    CanonicalFieldValue::Unsigned32(u32::from_be_bytes(
                        <[u8; 4]>::try_from(head).expect("the taken slice is the field's width"),
                    ))
                }
                CanonicalFieldKind::Unsigned64 => {
                    let Some(head) = take(&mut rest, 8) else {
                        return Err(DecodeErrorReturn::NonCanonical);
                    };
                    CanonicalFieldValue::Unsigned64(u64::from_be_bytes(
                        <[u8; 8]>::try_from(head).expect("the taken slice is the field's width"),
                    ))
                }
                CanonicalFieldKind::Text => {
                    let Some(head) = take_length_prefixed(&mut rest) else {
                        return Err(DecodeErrorReturn::NonCanonical);
                    };
                    let Ok(text) = String::from_utf8(head.to_vec()) else {
                        return Err(DecodeErrorReturn::NonCanonical);
                    };
                    CanonicalFieldValue::Text(text)
                }
                CanonicalFieldKind::Bytes => {
                    let Some(head) = take_length_prefixed(&mut rest) else {
                        return Err(DecodeErrorReturn::NonCanonical);
                    };
                    CanonicalFieldValue::Bytes(head.to_vec())
                }
                CanonicalFieldKind::FixedBytes20 => {
                    let Some(head) = take(&mut rest, 20) else {
                        return Err(DecodeErrorReturn::NonCanonical);
                    };
                    CanonicalFieldValue::FixedBytes20(
                        <[u8; 20]>::try_from(head).expect("the taken slice is the field's width"),
                    )
                }
                CanonicalFieldKind::Unsigned256 => {
                    let Some(head) = take(&mut rest, 32) else {
                        return Err(DecodeErrorReturn::NonCanonical);
                    };
                    CanonicalFieldValue::Unsigned256(
                        <[u8; 32]>::try_from(head).expect("the taken slice is the field's width"),
                    )
                }
            };
            values.push(value);
        }
        if !rest.is_empty() {
            return Err(DecodeErrorReturn::NonCanonical);
        }
        match params
            .description
            .fields_to_value(FromFieldsParams, CanonicalFields { values })
        {
            Ok(success) => Ok(DecodeSuccessReturn {
                described: success.described,
            }),
            Err(error) => Err(DecodeErrorReturn::Description(error)),
        }
    }
}

fn take<'a>(rest: &mut &'a [u8], width: usize) -> Option<&'a [u8]> {
    let head = rest.get(..width)?;
    *rest = &rest[width..];
    Some(head)
}

fn take_length_prefixed<'a>(rest: &mut &'a [u8]) -> Option<&'a [u8]> {
    let length = u32::from_be_bytes(
        <[u8; 4]>::try_from(take(rest, 4)?).expect("the taken slice is the field's width"),
    );
    take(
        rest,
        usize::try_from(length).expect("a u32 length fits a usize on every supported target"),
    )
}

#[derive(Default)]
pub struct CreateEncodingParamsOverrides {
    pub concrete: Option<EncodingConcrete>,
    pub identifier: Option<EncodingIdentifier>,
}

pub fn build_create_encoding_params(
    overrides: CreateEncodingParamsOverrides,
) -> CreateEncodingParams {
    CreateEncodingParams {
        concrete: overrides.concrete.unwrap_or(EncodingConcrete::Mock),
        identifier: overrides.identifier.unwrap_or(EncodingIdentifier::Mock),
    }
}

#[derive(Default)]
pub struct CreateEncodingPayloadOverrides;

pub fn build_create_encoding_payload(
    _overrides: CreateEncodingPayloadOverrides,
) -> CreateEncodingPayload {
    CreateEncodingPayload
}

pub fn build_create_encoding_deps<C>(consumer: C) -> CreateEncodingDeps<C> {
    CreateEncodingDeps { consumer }
}

pub fn build_create_encoding_success_return<O>(output: O) -> CreateEncodingSuccessReturn<O> {
    CreateEncodingSuccessReturn { output }
}

#[derive(Default)]
pub struct ConsumeEncodingParamsOverrides;

pub fn build_consume_encoding_params(
    _overrides: ConsumeEncodingParamsOverrides,
) -> ConsumeEncodingParams {
    ConsumeEncodingParams
}

pub fn build_consume_encoding_payload<E: IEncoderAdapter>(adapter: E) -> ConsumeEncodingPayload<E> {
    ConsumeEncodingPayload { adapter }
}

pub struct MockIEncodingConsumer<'a, D: IEncodingContract> {
    pub description: &'a D,
    pub described: D::Described,
    pub bytes: Vec<u8>,
}

impl<D: IEncodingContract> IEncodingConsumer for MockIEncodingConsumer<'_, D> {
    type Output = MockIEncodingConsumerOutput<D>;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
        let adapter = payload.adapter;
        let encoded = adapter.encode(build_encode_params(self.description), &self.described);
        let round_trip = match adapter
            .encode(build_encode_params(self.description), &self.described)
        {
            Ok(success) => adapter.decode(build_decode_params(self.description), &success.bytes),
            Err(error) => Err(DecodeErrorReturn::EncoderContract(error)),
        };
        let trailing_bytes =
            match adapter.encode(build_encode_params(self.description), &self.described) {
                Ok(success) => {
                    let mut bytes = success.bytes;
                    bytes.push(0x00);
                    adapter.decode(build_decode_params(self.description), &bytes)
                }
                Err(error) => Err(DecodeErrorReturn::EncoderContract(error)),
            };
        let truncated_bytes =
            match adapter.encode(build_encode_params(self.description), &self.described) {
                Ok(success) => {
                    let mut bytes = success.bytes;
                    bytes.truncate(bytes.len().saturating_sub(1));
                    adapter.decode(build_decode_params(self.description), &bytes)
                }
                Err(error) => Err(DecodeErrorReturn::EncoderContract(error)),
            };
        let decoded = adapter.decode(build_decode_params(self.description), &self.bytes);
        MockIEncodingConsumerOutput {
            declaration: E::DECLARATION,
            encoded,
            round_trip,
            trailing_bytes,
            truncated_bytes,
            decoded,
        }
    }
}

pub fn build_mock_i_encoding_consumer<'a, D: IEncodingContract>(
    description: &'a D,
    described: D::Described,
    bytes: Vec<u8>,
) -> MockIEncodingConsumer<'a, D> {
    MockIEncodingConsumer {
        description,
        described,
        bytes,
    }
}

#[derive(Default)]
pub struct MockIEncodingConsumerOutputOverrides {
    pub declaration: Option<EncodingDeclaration>,
    pub encoded: Option<EncodeReturn>,
    pub round_trip:
        Option<DecodeReturn<CanonicalFields, MockIEncodingContractFromFieldsErrorReturn>>,
    pub trailing_bytes:
        Option<DecodeReturn<CanonicalFields, MockIEncodingContractFromFieldsErrorReturn>>,
    pub truncated_bytes:
        Option<DecodeReturn<CanonicalFields, MockIEncodingContractFromFieldsErrorReturn>>,
    pub decoded: Option<DecodeReturn<CanonicalFields, MockIEncodingContractFromFieldsErrorReturn>>,
}

pub fn build_mock_i_encoding_consumer_output(
    overrides: MockIEncodingConsumerOutputOverrides,
) -> MockIEncodingConsumerOutput<MockIEncodingContract> {
    MockIEncodingConsumerOutput {
        declaration: overrides
            .declaration
            .unwrap_or_else(|| build_encoding_declaration(Default::default())),
        encoded: overrides
            .encoded
            .unwrap_or_else(|| Ok(build_encode_success_return(Default::default()))),
        round_trip: overrides.round_trip.unwrap_or_else(|| {
            Ok(build_decode_success_return(build_canonical_fields(
                Default::default(),
            )))
        }),
        trailing_bytes: overrides
            .trailing_bytes
            .unwrap_or_else(|| Err(DecodeErrorReturn::NonCanonical)),
        truncated_bytes: overrides
            .truncated_bytes
            .unwrap_or_else(|| Err(DecodeErrorReturn::NonCanonical)),
        decoded: overrides.decoded.unwrap_or_else(|| {
            Ok(build_decode_success_return(build_canonical_fields(
                Default::default(),
            )))
        }),
    }
}

pub fn mock_create_encoding<C: IEncodingConsumer>(
    deps: &CreateEncodingDeps<C>,
    _params: CreateEncodingParams,
    _payload: CreateEncodingPayload,
) -> CreateEncodingReturn<C::Output> {
    let output = deps.consumer.consume_encoding(
        build_consume_encoding_params(Default::default()),
        build_consume_encoding_payload(MockIEncoderAdapter),
    );
    Ok(build_create_encoding_success_return(output))
}

#[derive(Default)]
pub struct ToFieldParamsOverrides;

pub fn build_to_field_params(_overrides: ToFieldParamsOverrides) -> ToFieldParams {
    ToFieldParams
}

#[derive(Default)]
pub struct ToFieldSuccessReturnOverrides {
    pub field: Option<CanonicalFieldValue>,
}

pub fn build_to_field_success_return(
    overrides: ToFieldSuccessReturnOverrides,
) -> ToFieldSuccessReturn {
    ToFieldSuccessReturn {
        field: overrides
            .field
            .unwrap_or(CanonicalFieldValue::Unsigned256([0; 32])),
    }
}

#[derive(Default)]
pub struct FromFieldParamsOverrides;

pub fn build_from_field_params(_overrides: FromFieldParamsOverrides) -> FromFieldParams {
    FromFieldParams
}

#[derive(Default)]
pub struct FromFieldSuccessReturnOverrides<T> {
    pub value: Option<T>,
}

pub fn build_from_field_success_return<T: Default>(
    overrides: FromFieldSuccessReturnOverrides<T>,
) -> FromFieldSuccessReturn<T> {
    FromFieldSuccessReturn {
        value: overrides.value.unwrap_or_default(),
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MockICanonicalField;

impl ICanonicalField for MockICanonicalField {
    type FromFieldErrorReturn = Infallible;
    const KIND: CanonicalFieldKind = CanonicalFieldKind::Unsigned256;

    fn to_field(_params: ToFieldParams, _payload: &Self) -> ToFieldReturn {
        Ok(build_to_field_success_return(Default::default()))
    }

    fn from_field(
        _params: FromFieldParams,
        _payload: CanonicalFieldValue,
    ) -> FromFieldReturn<Self, Self::FromFieldErrorReturn> {
        Ok(build_from_field_success_return(Default::default()))
    }
}
