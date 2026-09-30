use crate::abi::provides::AbiDecoderErrorReturn;
use core::convert::Infallible;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CanonicalFieldKind {
    FixedBytes32,
    Unsigned16,
    Unsigned32,
    Unsigned64,
    Text,
    Bytes,
    FixedBytes20,
    Unsigned256,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CanonicalFieldValue {
    FixedBytes32([u8; 32]),
    Unsigned16(u16),
    Unsigned32(u32),
    Unsigned64(u64),
    Text(String),
    Bytes(Vec<u8>),
    FixedBytes20([u8; 20]),
    Unsigned256([u8; 32]),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CanonicalFields {
    pub values: Vec<CanonicalFieldValue>,
}

pub struct ToFieldsParams;

pub struct ToFieldsSuccessReturn {
    pub fields: CanonicalFields,
}

pub type ToFieldsReturn = Result<ToFieldsSuccessReturn, Infallible>;

pub struct FromFieldsParams;

pub struct FromFieldsSuccessReturn<T> {
    pub described: T,
}

pub type FromFieldsReturn<T, E> = Result<FromFieldsSuccessReturn<T>, E>;

pub trait IEncodingContract {
    type Described;
    type FromFieldsErrorReturn;
    const FIELDS: &'static [CanonicalFieldKind];

    fn to_fields(&self, params: ToFieldsParams, payload: &Self::Described) -> ToFieldsReturn;

    fn fields_to_value(
        &self,
        params: FromFieldsParams,
        payload: CanonicalFields,
    ) -> FromFieldsReturn<Self::Described, Self::FromFieldsErrorReturn>;
}

pub const ENCODING_INTERFACE_VERSION: u32 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EncodingIdentifier {
    EthereumAbiV1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EncodingDeclaration {
    pub identifier: EncodingIdentifier,
    pub adapter_version: u32,
    pub interface_version: u32,
}

pub struct EncodeParams<'a, D> {
    pub description: &'a D,
}

pub struct EncodeSuccessReturn {
    pub bytes: Vec<u8>,
}

pub type EncodeReturn = Result<EncodeSuccessReturn, Infallible>;

pub trait IEncoderAdapter {
    fn encode<D: IEncodingContract>(
        &self,
        params: EncodeParams<'_, D>,
        payload: &D::Described,
    ) -> EncodeReturn;
}

pub struct DecodeParams<'a, D> {
    pub description: &'a D,
}

pub struct DecodeSuccessReturn<T> {
    pub described: T,
}

#[derive(Debug)]
pub enum DecodeErrorReturn<E> {
    Abi(AbiDecoderErrorReturn),
    Description(E),
}

pub type DecodeReturn<T, E> = Result<DecodeSuccessReturn<T>, DecodeErrorReturn<E>>;

pub trait IDecoderAdapter {
    fn decode<D: IEncodingContract>(
        &self,
        params: DecodeParams<'_, D>,
        payload: &[u8],
    ) -> DecodeReturn<D::Described, D::FromFieldsErrorReturn>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EncodingConcrete {
    Abi,
}

pub trait IEncodingConsumer {
    type Output;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        params: ConsumeEncodingParams,
        payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output;
}

pub struct ConsumeEncodingParams;

pub struct ConsumeEncodingPayload<E> {
    pub adapter: E,
    pub declaration: EncodingDeclaration,
}

pub struct CreateEncodingDeps<C> {
    pub consumer: C,
}

pub struct CreateEncodingParams {
    pub concrete: EncodingConcrete,
    pub identifier: EncodingIdentifier,
}

pub struct CreateEncodingPayload;

pub struct CreateEncodingSuccessReturn<O> {
    pub output: O,
}

#[derive(Debug)]
pub enum CreateEncodingErrorReturn {
    UnsupportedEncodingIdentifier,
    Abi(Infallible),
}

pub type CreateEncodingReturn<O> =
    Result<CreateEncodingSuccessReturn<O>, CreateEncodingErrorReturn>;

pub type CreateEncodingFn<C> = fn(
    &CreateEncodingDeps<C>,
    CreateEncodingParams,
    CreateEncodingPayload,
) -> CreateEncodingReturn<<C as IEncodingConsumer>::Output>;
