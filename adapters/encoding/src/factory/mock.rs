#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFields, ConsumeEncodingParams,
    ConsumeEncodingPayload, CreateEncodingDeps, CreateEncodingParams, CreateEncodingPayload,
    CreateEncodingReturn, CreateEncodingSuccessReturn, DecodeErrorReturn, DecodeParams,
    DecodeReturn, DecodeSuccessReturn, ENCODING_INTERFACE_VERSION, EncodeParams, EncodeReturn,
    EncodeSuccessReturn, EncodingConcrete, EncodingDeclaration, EncodingIdentifier,
    FromFieldsParams, FromFieldsReturn, FromFieldsSuccessReturn, IDecoderAdapter, IEncoderAdapter,
    IEncodingConsumer, IEncodingContract, ToFieldsParams, ToFieldsReturn, ToFieldsSuccessReturn,
};
use core::convert::Infallible;
use core::marker::PhantomData;

#[derive(Default)]
pub struct CanonicalFieldsOverrides {
    pub values: Option<Vec<CanonicalFieldValue>>,
}

pub fn build_canonical_fields(overrides: CanonicalFieldsOverrides) -> CanonicalFields {
    CanonicalFields {
        values: overrides.values.unwrap_or_default(),
    }
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
pub struct FromFieldsSuccessReturnOverrides<T> {
    pub described: Option<T>,
}

pub fn build_from_fields_success_return<T: Default>(
    overrides: FromFieldsSuccessReturnOverrides<T>,
) -> FromFieldsSuccessReturn<T> {
    FromFieldsSuccessReturn {
        described: overrides.described.unwrap_or_default(),
    }
}

pub struct MockIEncodingContract<T> {
    pub described: PhantomData<T>,
}

impl<T: Default> IEncodingContract for MockIEncodingContract<T> {
    type Described = T;
    type FromFieldsErrorReturn = Infallible;
    const FIELDS: &'static [CanonicalFieldKind] = &[];

    fn to_fields(&self, _params: ToFieldsParams, _payload: &Self::Described) -> ToFieldsReturn {
        Ok(build_to_fields_success_return(Default::default()))
    }

    fn fields_to_value(
        &self,
        _params: FromFieldsParams,
        _payload: CanonicalFields,
    ) -> FromFieldsReturn<Self::Described, Self::FromFieldsErrorReturn> {
        Ok(build_from_fields_success_return(Default::default()))
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
        identifier: overrides
            .identifier
            .unwrap_or(EncodingIdentifier::EthereumAbiV1),
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
        bytes: overrides.bytes.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct DecodeSuccessReturnOverrides<T> {
    pub described: Option<T>,
}

pub fn build_decode_success_return<T: Default>(
    overrides: DecodeSuccessReturnOverrides<T>,
) -> DecodeSuccessReturn<T> {
    DecodeSuccessReturn {
        described: overrides.described.unwrap_or_default(),
    }
}

pub struct MockIEncoderAdapter;

impl IEncoderAdapter for MockIEncoderAdapter {
    const DECLARATION: EncodingDeclaration = EncodingDeclaration {
        identifier: EncodingIdentifier::EthereumAbiV1,
        adapter_version: 1,
        interface_version: ENCODING_INTERFACE_VERSION,
    };

    fn encode<D: IEncodingContract>(
        &self,
        _params: EncodeParams<'_, D>,
        _payload: &D::Described,
    ) -> EncodeReturn {
        Ok(build_encode_success_return(Default::default()))
    }
}

pub struct MockIDecoderAdapter;

impl IDecoderAdapter for MockIDecoderAdapter {
    fn decode<D: IEncodingContract>(
        &self,
        params: DecodeParams<'_, D>,
        _payload: &[u8],
    ) -> DecodeReturn<D::Described, D::FromFieldsErrorReturn> {
        match params
            .description
            .fields_to_value(FromFieldsParams, build_canonical_fields(Default::default()))
        {
            Ok(success) => Ok(DecodeSuccessReturn {
                described: success.described,
            }),
            Err(error) => Err(DecodeErrorReturn::Description(error)),
        }
    }
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
        concrete: overrides.concrete.unwrap_or(EncodingConcrete::Abi),
        identifier: overrides
            .identifier
            .unwrap_or(EncodingIdentifier::EthereumAbiV1),
    }
}

#[derive(Default)]
pub struct CreateEncodingSuccessReturnOverrides<O> {
    pub output: Option<O>,
}

pub fn build_create_encoding_success_return<O: Default>(
    overrides: CreateEncodingSuccessReturnOverrides<O>,
) -> CreateEncodingSuccessReturn<O> {
    CreateEncodingSuccessReturn {
        output: overrides.output.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct ConsumeEncodingPayloadOverrides<E> {
    pub adapter: Option<E>,
}

pub fn build_consume_encoding_payload<E: IEncoderAdapter + Default>(
    overrides: ConsumeEncodingPayloadOverrides<E>,
) -> ConsumeEncodingPayload<E> {
    ConsumeEncodingPayload {
        adapter: overrides.adapter.unwrap_or_default(),
    }
}

pub struct MockIEncodingConsumer;

impl IEncodingConsumer for MockIEncodingConsumer {
    type Output = ();

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        _payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
    }
}

pub fn mock_create_encoding<C: IEncodingConsumer>(
    _deps: &CreateEncodingDeps<C>,
    _params: CreateEncodingParams,
    _payload: CreateEncodingPayload,
) -> CreateEncodingReturn<C::Output>
where
    C::Output: Default,
{
    Ok(build_create_encoding_success_return(Default::default()))
}
