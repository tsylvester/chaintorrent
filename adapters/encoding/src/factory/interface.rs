use crate::abi::provides::AbiDecoderErrorReturn;
use crate::canonical_field_reader::provides::CanonicalFieldReaderErrorReturn;
use crate::lookup_encoding_declaration::provides::LookupEncodingDeclarationFn;
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

pub const CANONICAL_FIELD_KINDS: &[CanonicalFieldKind] = &[
    CanonicalFieldKind::FixedBytes32,
    CanonicalFieldKind::Unsigned16,
    CanonicalFieldKind::Unsigned32,
    CanonicalFieldKind::Unsigned64,
    CanonicalFieldKind::Text,
    CanonicalFieldKind::Bytes,
    CanonicalFieldKind::FixedBytes20,
    CanonicalFieldKind::Unsigned256,
];

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

pub const DECLARED_FIXED_WIDTH_FIELD_KINDS: &[CanonicalFieldKind] = &[
    CanonicalFieldKind::FixedBytes32,
    CanonicalFieldKind::FixedBytes20,
];

pub const DECLARED_UNSIGNED_FIELD_KINDS: &[CanonicalFieldKind] = &[
    CanonicalFieldKind::Unsigned16,
    CanonicalFieldKind::Unsigned32,
    CanonicalFieldKind::Unsigned64,
    CanonicalFieldKind::Unsigned256,
];

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
    type FromFieldsErrorReturn: core::fmt::Debug + PartialEq + Eq;

    const FIELDS: &'static [CanonicalFieldKind];

    fn to_fields(&self, params: ToFieldsParams, payload: &Self::Described) -> ToFieldsReturn;
    fn fields_to_value(
        &self,
        params: FromFieldsParams,
        payload: CanonicalFields,
    ) -> FromFieldsReturn<Self::Described, Self::FromFieldsErrorReturn>;
}

pub struct ToFieldParams;

pub struct ToFieldSuccessReturn {
    pub field: CanonicalFieldValue,
}

pub type ToFieldReturn = Result<ToFieldSuccessReturn, Infallible>;

pub struct FromFieldParams;

pub struct FromFieldSuccessReturn<T> {
    pub value: T,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FromFieldErrorReturn {
    WrongKind { expected: CanonicalFieldKind },
}

pub type FromFieldReturn<T> = Result<FromFieldSuccessReturn<T>, FromFieldErrorReturn>;

pub trait ICanonicalField: Sized {
    const KIND: CanonicalFieldKind;

    fn to_field(params: ToFieldParams, payload: &Self) -> ToFieldReturn;
    fn from_field(params: FromFieldParams, payload: CanonicalFieldValue) -> FromFieldReturn<Self>;
}

pub const ENCODING_INTERFACE_VERSION: u32 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EncodingIdentifier {
    EthereumAbiV1,
}

pub const ENCODING_IDENTIFIERS: &[EncodingIdentifier] = &[EncodingIdentifier::EthereumAbiV1];

pub struct EncodingDeclaration {
    pub concrete: EncodingConcrete,
    pub identifier: EncodingIdentifier,
    pub adapter_version: u32,
    pub interface_version: u32,
    pub type_name: TypeNameFn,
}

pub struct EncodeParams<'a, D> {
    pub description: &'a D,
}

pub struct EncodeSuccessReturn {
    pub bytes: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum EncodeErrorReturn {
    Fields(CanonicalFieldReaderErrorReturn),
}

pub type EncodeReturn = Result<EncodeSuccessReturn, EncodeErrorReturn>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EncodingTypeName {
    Bytes32,
    Bytes20,
    Uint16,
    Uint32,
    Uint64,
    Uint256,
    String,
    Bytes,
}

pub const ENCODING_TYPE_NAMES: &[EncodingTypeName] = &[
    EncodingTypeName::Bytes32,
    EncodingTypeName::Bytes20,
    EncodingTypeName::Uint16,
    EncodingTypeName::Uint32,
    EncodingTypeName::Uint64,
    EncodingTypeName::Uint256,
    EncodingTypeName::String,
    EncodingTypeName::Bytes,
];

pub struct TypeNameParams;

pub struct TypeNameSuccessReturn {
    pub name: EncodingTypeName,
}

pub type TypeNameReturn = Result<TypeNameSuccessReturn, Infallible>;

pub type TypeNameFn = fn(TypeNameParams, CanonicalFieldKind) -> TypeNameReturn;

pub trait IEncoderAdapter {
    fn encode<D: IEncodingContract>(
        &self,
        params: EncodeParams<'_, D>,
        payload: &D::Described,
    ) -> EncodeReturn;

    fn type_name(&self, params: TypeNameParams, payload: CanonicalFieldKind) -> TypeNameReturn;
}

pub struct DecodeParams<'a, D> {
    pub description: &'a D,
}

pub struct DecodeSuccessReturn<T> {
    pub described: T,
}

#[derive(Debug, PartialEq)]
pub enum DecodeErrorReturn<E> {
    Abi(AbiDecoderErrorReturn),
    Description(E),
    EncoderContract(EncodeErrorReturn),
    NonCanonical,
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
    #[cfg(any(test, feature = "mocks"))]
    Mock,
}

pub const ENCODING_CONCRETES: &[EncodingConcrete] = &[EncodingConcrete::Abi];

pub trait IEncodingConsumer {
    type Output;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        params: ConsumeEncodingParams,
        payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output;
}

pub struct ConsumeEncodingParams;

pub struct ConsumeEncodingPayload<E: IEncoderAdapter> {
    pub adapter: E,
    pub declaration: EncodingDeclaration,
}

pub struct CreateEncodingDeps<C> {
    pub consumer: C,
    pub lookup: LookupEncodingDeclarationFn,
}

pub struct CreateEncodingParams {
    pub selection: EncodingConcrete,
}

pub struct CreateEncodingPayload;

pub struct CreateEncodingSuccessReturn<O> {
    pub output: O,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CreateEncodingErrorReturn {
    Abi(Infallible),
}

pub type CreateEncodingReturn<O> =
    Result<CreateEncodingSuccessReturn<O>, CreateEncodingErrorReturn>;

pub type CreateEncodingFn<C> = fn(
    &CreateEncodingDeps<C>,
    CreateEncodingParams,
    CreateEncodingPayload,
) -> CreateEncodingReturn<<C as IEncodingConsumer>::Output>;

pub const DECLARED_ENCODING_SELECTIONS: &[CreateEncodingParams] = &[CreateEncodingParams {
    selection: EncodingConcrete::Abi,
}];

#[cfg(any(test, feature = "mocks"))]
#[derive(Debug, PartialEq, Eq)]
pub enum MockIEncodingContractFromFieldsErrorReturn {
    EmptyText { index: usize },
}

#[cfg(any(test, feature = "mocks"))]
pub struct MockIEncodingConsumerOutput<D: IEncodingContract> {
    pub declaration: EncodingDeclaration,
    pub encoded: EncodeReturn,
    pub round_trip: DecodeReturn<D::Described, D::FromFieldsErrorReturn>,
    pub trailing_bytes: DecodeReturn<D::Described, D::FromFieldsErrorReturn>,
    pub truncated_bytes: DecodeReturn<D::Described, D::FromFieldsErrorReturn>,
    pub decoded: DecodeReturn<D::Described, D::FromFieldsErrorReturn>,
    pub type_names: Vec<EncodingTypeName>,
}
