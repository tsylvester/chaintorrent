#![allow(dead_code, clippy::expect_used, clippy::too_many_arguments)]

use super::interface::{
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFields, ConsumeEncodingParams,
    ConsumeEncodingPayload, CreateEncodingDeps, CreateEncodingParams, CreateEncodingPayload,
    CreateEncodingReturn, CreateEncodingSuccessReturn, DecodeErrorReturn, DecodeParams,
    DecodeReturn, DecodeSuccessReturn, ENCODING_INTERFACE_VERSION, EncodeErrorReturn, EncodeParams,
    EncodeReturn, EncodeSuccessReturn, EncodingConcrete, EncodingDeclaration, EncodingIdentifier,
    EncodingTypeName, EncodingTypeNameConstructorParams, FromFieldParams, FromFieldSuccessReturn,
    FromFieldsParams, FromFieldsReturn, FromFieldsSuccessReturn, IDecoderAdapter, IEncoderAdapter,
    IEncodingConsumer, IEncodingContract, MockIEncodingConsumerOutput,
    MockIEncodingContractFromFieldsErrorReturn, ToFieldParams, ToFieldSuccessReturn,
    ToFieldsParams, ToFieldsReturn, ToFieldsSuccessReturn, TypeNameParams, TypeNameReturn,
    TypeNameSuccessReturn,
};

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
        for (index, value) in payload.values.iter().enumerate() {
            if let CanonicalFieldValue::Text(text) = value
                && text.is_empty()
            {
                return Err(MockIEncodingContractFromFieldsErrorReturn::EmptyText { index });
            }
        }
        Ok(FromFieldsSuccessReturn { described: payload })
    }
}

#[derive(Default)]
pub struct ToFieldsParamsOverrides;

pub fn build_to_fields_params(_overrides: ToFieldsParamsOverrides) -> ToFieldsParams {
    ToFieldsParams
}

#[derive(Default)]
pub struct FromFieldsParamsOverrides;

pub fn build_from_fields_params(_overrides: FromFieldsParamsOverrides) -> FromFieldsParams {
    FromFieldsParams
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

#[derive(Default)]
pub struct ToFieldParamsOverrides;

pub fn build_to_field_params(_overrides: ToFieldParamsOverrides) -> ToFieldParams {
    ToFieldParams
}

#[derive(Default)]
pub struct FromFieldParamsOverrides;

pub fn build_from_field_params(_overrides: FromFieldParamsOverrides) -> FromFieldParams {
    FromFieldParams
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
            .unwrap_or(CanonicalFieldValue::FixedBytes32([0x01; 32])),
    }
}

#[derive(Default)]
pub struct FromFieldSuccessReturnOverrides;

pub fn build_from_field_success_return<T>(
    value: T,
    _overrides: FromFieldSuccessReturnOverrides,
) -> FromFieldSuccessReturn<T> {
    FromFieldSuccessReturn { value }
}

#[derive(Default)]
pub struct EncodingDeclarationOverrides {
    pub identifier: Option<EncodingIdentifier>,
    pub adapter_version: Option<u32>,
    pub interface_version: Option<u32>,
}

pub fn build_encoding_declaration(overrides: EncodingDeclarationOverrides) -> EncodingDeclaration {
    EncodingDeclaration {
        identifier: overrides
            .identifier
            .unwrap_or(MOCK_I_ENCODER_ADAPTER_DECLARATION.identifier),
        adapter_version: overrides
            .adapter_version
            .unwrap_or(MOCK_I_ENCODER_ADAPTER_DECLARATION.adapter_version),
        interface_version: overrides
            .interface_version
            .unwrap_or(MOCK_I_ENCODER_ADAPTER_DECLARATION.interface_version),
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

#[derive(Default)]
pub struct EncodingTypeNameConstructorParamsOverrides {
    pub name: Option<String>,
}

pub fn build_encoding_type_name_constructor_params(
    overrides: EncodingTypeNameConstructorParamsOverrides,
) -> EncodingTypeNameConstructorParams {
    EncodingTypeNameConstructorParams {
        name: overrides.name.unwrap_or_else(|| "mockName".to_string()),
    }
}

pub fn build_encoding_type_name(
    overrides: EncodingTypeNameConstructorParamsOverrides,
) -> EncodingTypeName {
    EncodingTypeName::try_new(build_encoding_type_name_constructor_params(overrides))
        .expect("the built name is a valid encoding type name")
}

#[derive(Default)]
pub struct TypeNameParamsOverrides;

pub fn build_type_name_params(_overrides: TypeNameParamsOverrides) -> TypeNameParams {
    TypeNameParams
}

#[derive(Default)]
pub struct TypeNameSuccessReturnOverrides {
    pub name: Option<EncodingTypeName>,
}

pub fn build_type_name_success_return(
    overrides: TypeNameSuccessReturnOverrides,
) -> TypeNameSuccessReturn {
    TypeNameSuccessReturn {
        name: overrides
            .name
            .unwrap_or_else(|| build_encoding_type_name(Default::default())),
    }
}

pub fn build_encode_params<'a, D: IEncodingContract>(description: &'a D) -> EncodeParams<'a, D> {
    EncodeParams { description }
}

pub fn build_decode_params<'a, D: IEncodingContract>(description: &'a D) -> DecodeParams<'a, D> {
    DecodeParams { description }
}

pub fn build_decode_success_return<T>(described: T) -> DecodeSuccessReturn<T> {
    DecodeSuccessReturn { described }
}

fn mock_field_kind(value: &CanonicalFieldValue) -> CanonicalFieldKind {
    match value {
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

fn mock_take<'a>(payload: &'a [u8], cursor: &mut usize, length: usize) -> Option<&'a [u8]> {
    let end = cursor.checked_add(length)?;
    let bytes = payload.get(*cursor..end)?;
    *cursor = end;
    Some(bytes)
}

pub(crate) const MOCK_I_ENCODER_ADAPTER_DECLARATION: EncodingDeclaration = EncodingDeclaration {
    identifier: EncodingIdentifier::Mock,
    adapter_version: 1,
    interface_version: ENCODING_INTERFACE_VERSION,
};

pub(crate) struct MockIEncoderAdapter;

impl IEncoderAdapter for MockIEncoderAdapter {
    const DECLARATION: EncodingDeclaration = MOCK_I_ENCODER_ADAPTER_DECLARATION;

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
            let actual = mock_field_kind(value);
            if actual != *expected {
                return Err(EncodeErrorReturn::FieldKind {
                    index,
                    expected: *expected,
                    actual,
                });
            }
        }
        let mut bytes = Vec::new();
        for value in &emitted.fields.values {
            match value {
                CanonicalFieldValue::FixedBytes32(word) => bytes.extend_from_slice(word),
                CanonicalFieldValue::Unsigned16(value) => {
                    bytes.extend_from_slice(&value.to_be_bytes());
                }
                CanonicalFieldValue::Unsigned32(value) => {
                    bytes.extend_from_slice(&value.to_be_bytes());
                }
                CanonicalFieldValue::Unsigned64(value) => {
                    bytes.extend_from_slice(&value.to_be_bytes());
                }
                CanonicalFieldValue::Text(text) => {
                    let length =
                        u32::try_from(text.len()).expect("a text field's byte length fits a u32");
                    bytes.extend_from_slice(&length.to_be_bytes());
                    bytes.extend_from_slice(text.as_bytes());
                }
                CanonicalFieldValue::Bytes(blob) => {
                    let length =
                        u32::try_from(blob.len()).expect("a byte string's length fits a u32");
                    bytes.extend_from_slice(&length.to_be_bytes());
                    bytes.extend_from_slice(blob);
                }
                CanonicalFieldValue::FixedBytes20(word) => bytes.extend_from_slice(word),
                CanonicalFieldValue::Unsigned256(word) => bytes.extend_from_slice(word),
            }
        }
        Ok(EncodeSuccessReturn { bytes })
    }

    fn type_name(&self, _params: TypeNameParams, payload: CanonicalFieldKind) -> TypeNameReturn {
        let name = match payload {
            CanonicalFieldKind::FixedBytes32 => "mockFixedBytes32",
            CanonicalFieldKind::Unsigned16 => "mockUnsigned16",
            CanonicalFieldKind::Unsigned32 => "mockUnsigned32",
            CanonicalFieldKind::Unsigned64 => "mockUnsigned64",
            CanonicalFieldKind::Text => "mockText",
            CanonicalFieldKind::Bytes => "mockBytes",
            CanonicalFieldKind::FixedBytes20 => "mockFixedBytes20",
            CanonicalFieldKind::Unsigned256 => "mockUnsigned256",
        };
        Ok(TypeNameSuccessReturn {
            name: EncodingTypeName::try_new(EncodingTypeNameConstructorParams {
                name: name.to_string(),
            })
            .expect("every declared mock name is a valid encoding type name"),
        })
    }
}

impl IDecoderAdapter for MockIEncoderAdapter {
    fn decode<D: IEncodingContract>(
        &self,
        params: DecodeParams<'_, D>,
        payload: &[u8],
    ) -> DecodeReturn<D::Described, D::FromFieldsErrorReturn> {
        mock_decode(params, payload)
    }
}

pub(crate) const MOCK_I_DECODER_ADAPTER_DECLARATION: EncodingDeclaration = EncodingDeclaration {
    identifier: EncodingIdentifier::Mock,
    adapter_version: 1,
    interface_version: ENCODING_INTERFACE_VERSION,
};

pub(crate) struct MockIDecoderAdapter;

impl MockIDecoderAdapter {
    pub(crate) const DECLARATION: EncodingDeclaration = MOCK_I_DECODER_ADAPTER_DECLARATION;
}

impl IDecoderAdapter for MockIDecoderAdapter {
    fn decode<D: IEncodingContract>(
        &self,
        params: DecodeParams<'_, D>,
        payload: &[u8],
    ) -> DecodeReturn<D::Described, D::FromFieldsErrorReturn> {
        mock_decode(params, payload)
    }
}

fn mock_decode<D: IEncodingContract>(
    params: DecodeParams<'_, D>,
    payload: &[u8],
) -> DecodeReturn<D::Described, D::FromFieldsErrorReturn> {
    let mut cursor = 0usize;
    let mut values = Vec::with_capacity(D::FIELDS.len());
    for kind in D::FIELDS {
        let value = match kind {
            CanonicalFieldKind::FixedBytes32 => {
                let Some(word) = mock_take(payload, &mut cursor, 32) else {
                    return Err(DecodeErrorReturn::NonCanonical);
                };
                CanonicalFieldValue::FixedBytes32(
                    <[u8; 32]>::try_from(word).expect("the slice is thirty-two bytes"),
                )
            }
            CanonicalFieldKind::Unsigned16 => {
                let Some(word) = mock_take(payload, &mut cursor, 2) else {
                    return Err(DecodeErrorReturn::NonCanonical);
                };
                CanonicalFieldValue::Unsigned16(u16::from_be_bytes(
                    <[u8; 2]>::try_from(word).expect("the slice is two bytes"),
                ))
            }
            CanonicalFieldKind::Unsigned32 => {
                let Some(word) = mock_take(payload, &mut cursor, 4) else {
                    return Err(DecodeErrorReturn::NonCanonical);
                };
                CanonicalFieldValue::Unsigned32(u32::from_be_bytes(
                    <[u8; 4]>::try_from(word).expect("the slice is four bytes"),
                ))
            }
            CanonicalFieldKind::Unsigned64 => {
                let Some(word) = mock_take(payload, &mut cursor, 8) else {
                    return Err(DecodeErrorReturn::NonCanonical);
                };
                CanonicalFieldValue::Unsigned64(u64::from_be_bytes(
                    <[u8; 8]>::try_from(word).expect("the slice is eight bytes"),
                ))
            }
            CanonicalFieldKind::Text => {
                let Some(prefix) = mock_take(payload, &mut cursor, 4) else {
                    return Err(DecodeErrorReturn::NonCanonical);
                };
                let length = usize::try_from(u32::from_be_bytes(
                    <[u8; 4]>::try_from(prefix).expect("the slice is four bytes"),
                ))
                .expect("a u32 length fits a usize on every supported target");
                let Some(bytes) = mock_take(payload, &mut cursor, length) else {
                    return Err(DecodeErrorReturn::NonCanonical);
                };
                let Ok(text) = String::from_utf8(bytes.to_vec()) else {
                    return Err(DecodeErrorReturn::NonCanonical);
                };
                CanonicalFieldValue::Text(text)
            }
            CanonicalFieldKind::Bytes => {
                let Some(prefix) = mock_take(payload, &mut cursor, 4) else {
                    return Err(DecodeErrorReturn::NonCanonical);
                };
                let length = usize::try_from(u32::from_be_bytes(
                    <[u8; 4]>::try_from(prefix).expect("the slice is four bytes"),
                ))
                .expect("a u32 length fits a usize on every supported target");
                let Some(bytes) = mock_take(payload, &mut cursor, length) else {
                    return Err(DecodeErrorReturn::NonCanonical);
                };
                CanonicalFieldValue::Bytes(bytes.to_vec())
            }
            CanonicalFieldKind::FixedBytes20 => {
                let Some(word) = mock_take(payload, &mut cursor, 20) else {
                    return Err(DecodeErrorReturn::NonCanonical);
                };
                CanonicalFieldValue::FixedBytes20(
                    <[u8; 20]>::try_from(word).expect("the slice is twenty bytes"),
                )
            }
            CanonicalFieldKind::Unsigned256 => {
                let Some(word) = mock_take(payload, &mut cursor, 32) else {
                    return Err(DecodeErrorReturn::NonCanonical);
                };
                CanonicalFieldValue::Unsigned256(
                    <[u8; 32]>::try_from(word).expect("the slice is thirty-two bytes"),
                )
            }
        };
        values.push(value);
    }
    if cursor != payload.len() {
        return Err(DecodeErrorReturn::NonCanonical);
    }
    match params
        .description
        .fields_to_value(FromFieldsParams, CanonicalFields { values })
    {
        Err(error) => Err(DecodeErrorReturn::Description(error)),
        Ok(success) => Ok(DecodeSuccessReturn {
            described: success.described,
        }),
    }
}

pub struct MockIEncodingConsumer<'a, D: IEncodingContract> {
    pub description: &'a D,
    pub described: D::Described,
    pub bytes: Vec<u8>,
}

impl<'a, D: IEncodingContract> MockIEncodingConsumer<'a, D> {
    fn decode_written<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        adapter: &E,
        transform: impl FnOnce(Vec<u8>) -> Vec<u8>,
    ) -> DecodeReturn<D::Described, D::FromFieldsErrorReturn> {
        match adapter.encode(build_encode_params(self.description), &self.described) {
            Err(error) => Err(DecodeErrorReturn::EncoderContract(error)),
            Ok(success) => adapter.decode(
                build_decode_params(self.description),
                &transform(success.bytes),
            ),
        }
    }
}

impl<'a, D: IEncodingContract> IEncodingConsumer for MockIEncodingConsumer<'a, D> {
    type Output = MockIEncodingConsumerOutput<D>;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
        MockIEncodingConsumerOutput {
            declaration: E::DECLARATION,
            encoded: payload
                .adapter
                .encode(build_encode_params(self.description), &self.described),
            round_trip: self.decode_written(&payload.adapter, |bytes| bytes),
            trailing_bytes: self.decode_written(&payload.adapter, |mut bytes| {
                bytes.push(0x00);
                bytes
            }),
            truncated_bytes: self.decode_written(&payload.adapter, |mut bytes| {
                bytes.pop();
                bytes
            }),
            decoded: payload
                .adapter
                .decode(build_decode_params(self.description), &self.bytes),
            type_names: D::FIELDS
                .iter()
                .map(|kind| {
                    payload
                        .adapter
                        .type_name(build_type_name_params(Default::default()), *kind)
                        .expect("the encoder's type_name return is infallible")
                        .name
                })
                .collect(),
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
    pub type_names: Option<Vec<EncodingTypeName>>,
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
        type_names: overrides.type_names.unwrap_or_else(|| {
            [
                "mockText",
                "mockBytes",
                "mockFixedBytes32",
                "mockUnsigned16",
                "mockUnsigned32",
                "mockUnsigned64",
                "mockFixedBytes20",
                "mockUnsigned256",
            ]
            .into_iter()
            .map(|name| {
                build_encoding_type_name(EncodingTypeNameConstructorParamsOverrides {
                    name: Some(name.to_string()),
                })
            })
            .collect()
        }),
    }
}

#[derive(Default)]
pub struct ConsumeEncodingParamsOverrides;

pub fn build_consume_encoding_params(
    _overrides: ConsumeEncodingParamsOverrides,
) -> ConsumeEncodingParams {
    ConsumeEncodingParams
}

#[derive(Default)]
pub struct CreateEncodingPayloadOverrides;

pub fn build_create_encoding_payload(
    _overrides: CreateEncodingPayloadOverrides,
) -> CreateEncodingPayload {
    CreateEncodingPayload
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

pub fn build_consume_encoding_payload<E: IEncoderAdapter>(adapter: E) -> ConsumeEncodingPayload<E> {
    ConsumeEncodingPayload { adapter }
}

pub fn build_create_encoding_deps<C>(consumer: C) -> CreateEncodingDeps<C> {
    CreateEncodingDeps { consumer }
}

pub fn build_create_encoding_success_return<O>(output: O) -> CreateEncodingSuccessReturn<O> {
    CreateEncodingSuccessReturn { output }
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
