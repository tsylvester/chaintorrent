#![allow(dead_code, clippy::too_many_arguments)]

use chain::{
    ConsumeChainFormsParams, ConsumeChainFormsPayload, CreateChainFormsParams, IChainForms,
    IChainFormsConsumer, build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, create_chain_forms,
};
use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingParams, IDecoderAdapter,
    IEncoderAdapter, IEncodingConsumer, build_create_encoding_deps, build_create_encoding_params,
    build_create_encoding_payload, create_encoding,
};
use envelope::KeyAgreementDeclaration;
use kem::KemDeclaration;
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParams, CreatePairingPayload,
    IPairingArithmetic, IPairingConsumer, IPairingReference, build_create_pairing_deps,
    build_create_pairing_params, create_pairing,
};
use proof::{DELIVERY_STATEMENT_VERSION_ONE, DeliveryProofDeclaration};

use super::super::render::provides::{SolidityLibraryEntry, build_solidity_library_entry};
use super::interface::{
    ConstantsCaseDeps, ConstantsCaseErrorReturn, ConstantsCaseParams, ConstantsCasePayload,
    ConstantsCaseReturn, ConstantsCaseSuccessReturn, ConstantsDeps, ConstantsParams,
    ConstantsPayload, ConstantsReturn, ConstantsSuccessReturn, IConstantsCaseConsumer,
};

#[derive(Default)]
pub struct ConstantsDepsOverrides;

pub fn build_constants_deps<'a, P: IPairingReference, E: IEncoderAdapter>(
    pairing: &'a P,
    encoder: &'a E,
    _overrides: ConstantsDepsOverrides,
) -> ConstantsDeps<'a, P, E> {
    ConstantsDeps { pairing, encoder }
}

#[derive(Default)]
pub struct ConstantsParamsOverrides {
    pub statement_version: Option<u16>,
}

pub fn build_constants_params(overrides: ConstantsParamsOverrides) -> ConstantsParams {
    ConstantsParams {
        statement_version: overrides
            .statement_version
            .unwrap_or(DELIVERY_STATEMENT_VERSION_ONE),
    }
}

#[derive(Default)]
pub struct ConstantsPayloadOverrides;

pub fn build_constants_payload<'a>(
    kem: &'a KemDeclaration,
    key_agreement: &'a KeyAgreementDeclaration,
    delivery_proof: &'a DeliveryProofDeclaration,
    _overrides: ConstantsPayloadOverrides,
) -> ConstantsPayload<'a> {
    ConstantsPayload {
        kem,
        key_agreement,
        delivery_proof,
    }
}

#[derive(Default)]
pub struct ConstantsSuccessReturnOverrides {
    pub entries: Option<Vec<SolidityLibraryEntry>>,
}

pub fn build_constants_success_return(
    overrides: ConstantsSuccessReturnOverrides,
) -> ConstantsSuccessReturn {
    ConstantsSuccessReturn {
        entries: overrides
            .entries
            .unwrap_or_else(|| vec![build_solidity_library_entry(Default::default())]),
    }
}

pub fn mock_constants<'a, P: IPairingReference, E: IEncoderAdapter>(
    _deps: &ConstantsDeps<'a, P, E>,
    _params: ConstantsParams,
    _payload: ConstantsPayload<'_>,
) -> ConstantsReturn {
    Ok(build_constants_success_return(Default::default()))
}

#[derive(Default)]
pub struct ConstantsCaseDepsOverrides;

pub fn build_constants_case_deps<C: IConstantsCaseConsumer>(
    consumer: C,
    _overrides: ConstantsCaseDepsOverrides,
) -> ConstantsCaseDeps<C> {
    ConstantsCaseDeps { consumer }
}

#[derive(Default)]
pub struct ConstantsCaseParamsOverrides {
    pub pairing: Option<CreatePairingParams>,
    pub encoding: Option<CreateEncodingParams>,
    pub chain_forms: Option<CreateChainFormsParams>,
}

pub fn build_constants_case_params(overrides: ConstantsCaseParamsOverrides) -> ConstantsCaseParams {
    ConstantsCaseParams {
        pairing: overrides
            .pairing
            .unwrap_or_else(|| build_create_pairing_params(Default::default())),
        encoding: overrides
            .encoding
            .unwrap_or_else(|| build_create_encoding_params(Default::default())),
        chain_forms: overrides
            .chain_forms
            .unwrap_or_else(|| build_create_chain_forms_params(Default::default())),
    }
}

#[derive(Default)]
pub struct ConstantsCasePayloadOverrides;

pub fn build_constants_case_payload(
    _overrides: ConstantsCasePayloadOverrides,
) -> ConstantsCasePayload {
    ConstantsCasePayload
}

#[derive(Default)]
pub struct ConstantsCaseSuccessReturnOverrides;

pub fn build_constants_case_success_return<O>(
    output: O,
    _overrides: ConstantsCaseSuccessReturnOverrides,
) -> ConstantsCaseSuccessReturn<O> {
    ConstantsCaseSuccessReturn { output }
}

struct CaseAtPairing<'a, C: IConstantsCaseConsumer> {
    consumer: &'a C,
    encoding: CreateEncodingParams,
    chain_forms: CreateChainFormsParams,
}

impl<C: IConstantsCaseConsumer> IPairingConsumer for CaseAtPairing<'_, C> {
    type Output = ConstantsCaseReturn<C::Output>;

    fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        match create_encoding(
            &build_create_encoding_deps(CaseAtEncoding {
                consumer: self.consumer,
                pairing: &payload.adapter,
                chain_forms: self.chain_forms,
            }),
            CreateEncodingParams {
                concrete: self.encoding.concrete,
                identifier: self.encoding.identifier,
            },
            build_create_encoding_payload(Default::default()),
        ) {
            Err(error) => Err(ConstantsCaseErrorReturn::CreateEncoding(error)),
            Ok(success) => success.output,
        }
    }
}

struct CaseAtEncoding<'a, 'p, C: IConstantsCaseConsumer, P: IPairingArithmetic + IPairingReference>
{
    consumer: &'a C,
    pairing: &'p P,
    chain_forms: CreateChainFormsParams,
}

impl<C: IConstantsCaseConsumer, P: IPairingArithmetic + IPairingReference> IEncodingConsumer
    for CaseAtEncoding<'_, '_, C, P>
{
    type Output = ConstantsCaseReturn<C::Output>;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
        match create_chain_forms(
            &build_create_chain_forms_deps(
                CaseAtChainForms {
                    consumer: self.consumer,
                    pairing: self.pairing,
                    encoder: &payload.adapter,
                },
                &payload.adapter,
                Default::default(),
            ),
            self.chain_forms,
            build_create_chain_forms_payload(Default::default()),
        ) {
            Err(error) => Err(ConstantsCaseErrorReturn::CreateChainForms(error)),
            Ok(success) => success.output,
        }
    }
}

struct CaseAtChainForms<
    'a,
    'p,
    C: IConstantsCaseConsumer,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
> {
    consumer: &'a C,
    pairing: &'p P,
    encoder: &'a E,
}

impl<
    C: IConstantsCaseConsumer,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
> IChainFormsConsumer for CaseAtChainForms<'_, '_, C, P, E>
{
    type Output = ConstantsCaseReturn<C::Output>;

    fn consume_chain_forms<F: IChainForms>(
        &self,
        _params: ConsumeChainFormsParams,
        payload: ConsumeChainFormsPayload<F>,
    ) -> Self::Output {
        Ok(ConstantsCaseSuccessReturn {
            output: self.consumer.consume_constants_case(
                self.pairing,
                self.encoder,
                &payload.forms,
                &ConstantsCasePayload,
            ),
        })
    }
}

pub fn run_constants_case<C: IConstantsCaseConsumer>(
    deps: &ConstantsCaseDeps<C>,
    params: ConstantsCaseParams,
    _payload: ConstantsCasePayload,
) -> ConstantsCaseReturn<C::Output> {
    match create_pairing(
        &build_create_pairing_deps(
            CaseAtPairing {
                consumer: &deps.consumer,
                encoding: params.encoding,
                chain_forms: params.chain_forms,
            },
            Default::default(),
        ),
        params.pairing,
        CreatePairingPayload,
    ) {
        Err(error) => Err(ConstantsCaseErrorReturn::CreatePairing(error)),
        Ok(success) => success.output,
    }
}
