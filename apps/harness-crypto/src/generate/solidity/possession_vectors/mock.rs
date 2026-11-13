#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_arguments
)]

use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingParams, IDecoderAdapter,
    IEncoderAdapter, IEncodingConsumer, build_create_encoding_deps, build_create_encoding_params,
    build_create_encoding_payload, create_encoding,
};
use envelope::{
    ConsumeKeyAgreementParams, ConsumeKeyAgreementPayload, CreateKeyAgreementParams,
    IKeyAgreementAdapter, IKeyAgreementConsumer, build_create_key_agreement_deps,
    build_create_key_agreement_params, build_create_key_agreement_payload, create_key_agreement,
};
use hash_to_scalar::{
    CreateHashToScalarParams, IHashToScalarAdapter, build_create_hash_to_scalar_deps,
    build_create_hash_to_scalar_params, build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParams, IPairingArithmetic,
    IPairingConsumer, IPairingReference, build_create_pairing_deps, build_create_pairing_params,
    build_create_pairing_payload, create_pairing,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParams, CreateRandomSourcePayload,
    IRandomSourceAdapter, build_create_random_source_params, create_random_source,
};

use super::super::super::factory::provides::{VectorCount, build_vector_count};
use super::interface::{
    IPossessionVectorsCaseConsumer, PossessionVector, PossessionVectorDescription,
    PossessionVectorDescriptionConstructorParams, PossessionVectorsCaseDeps,
    PossessionVectorsCaseErrorReturn, PossessionVectorsCaseParams, PossessionVectorsCasePayload,
    PossessionVectorsCaseReturn, PossessionVectorsCaseSuccessReturn, PossessionVectorsDeps,
    PossessionVectorsParams, PossessionVectorsPayload, PossessionVectorsReturn,
    PossessionVectorsSuccessReturn, RunPossessionVectorsCaseFn,
};

#[derive(Default)]
pub struct PossessionVectorOverrides {
    pub x: Option<[u8; 32]>,
    pub y: Option<[u8; 32]>,
    pub pk1: Option<Vec<u8>>,
    pub pk2: Option<Vec<u8>>,
    pub r1: Option<Vec<u8>>,
    pub z1: Option<[u8; 32]>,
    pub r2: Option<Vec<u8>>,
    pub z2: Option<[u8; 32]>,
}

pub fn build_possession_vector(overrides: PossessionVectorOverrides) -> PossessionVector {
    PossessionVector {
        x: overrides.x.unwrap_or([0x01; 32]),
        y: overrides.y.unwrap_or([0x02; 32]),
        pk1: overrides.pk1.unwrap_or_else(|| vec![0x03; 64]),
        pk2: overrides.pk2.unwrap_or_else(|| vec![0x04; 128]),
        r1: overrides.r1.unwrap_or_else(|| vec![0x05; 64]),
        z1: overrides.z1.unwrap_or([0x06; 32]),
        r2: overrides.r2.unwrap_or_else(|| vec![0x07; 128]),
        z2: overrides.z2.unwrap_or([0x08; 32]),
    }
}

#[derive(Default)]
pub struct PossessionVectorsParamsOverrides {
    pub count: Option<VectorCount>,
}

pub fn build_possession_vectors_params(
    overrides: PossessionVectorsParamsOverrides,
) -> PossessionVectorsParams {
    PossessionVectorsParams {
        count: overrides
            .count
            .unwrap_or_else(|| build_vector_count(Default::default())),
    }
}

#[derive(Default)]
pub struct PossessionVectorDescriptionConstructorParamsOverrides;

pub fn build_possession_vector_description_constructor_params(
    _overrides: PossessionVectorDescriptionConstructorParamsOverrides,
) -> PossessionVectorDescriptionConstructorParams {
    PossessionVectorDescriptionConstructorParams
}

pub fn build_possession_vector_description(
    overrides: PossessionVectorDescriptionConstructorParamsOverrides,
) -> PossessionVectorDescription {
    PossessionVectorDescription::try_new(build_possession_vector_description_constructor_params(
        overrides,
    ))
    .expect("built params are valid")
}

#[derive(Default)]
pub struct PossessionVectorsPayloadOverrides;

pub fn build_possession_vectors_payload(
    _overrides: PossessionVectorsPayloadOverrides,
) -> PossessionVectorsPayload {
    PossessionVectorsPayload
}

#[derive(Default)]
pub struct PossessionVectorsDepsOverrides;

pub fn build_possession_vectors_deps<
    'a,
    P: IPairingArithmetic,
    K: IKeyAgreementAdapter<Pairing = P>,
>(
    pairing: &'a P,
    key_agreement: &'a K,
    random: &'a dyn IRandomSourceAdapter,
    _overrides: PossessionVectorsDepsOverrides,
) -> PossessionVectorsDeps<'a, P, K> {
    PossessionVectorsDeps {
        pairing,
        key_agreement,
        random,
    }
}

#[derive(Default)]
pub struct PossessionVectorsSuccessReturnOverrides {
    pub records: Option<Vec<PossessionVector>>,
}

pub fn build_possession_vectors_success_return(
    overrides: PossessionVectorsSuccessReturnOverrides,
) -> PossessionVectorsSuccessReturn {
    PossessionVectorsSuccessReturn {
        records: overrides
            .records
            .unwrap_or_else(|| vec![build_possession_vector(Default::default())]),
    }
}

pub fn mock_possession_vectors<'a, P: IPairingArithmetic, K: IKeyAgreementAdapter<Pairing = P>>(
    _deps: &PossessionVectorsDeps<'a, P, K>,
    _params: PossessionVectorsParams,
    _payload: PossessionVectorsPayload,
) -> PossessionVectorsReturn {
    Ok(build_possession_vectors_success_return(Default::default()))
}

#[derive(Default)]
pub struct PossessionVectorsCaseParamsOverrides {
    pub pairing: Option<CreatePairingParams>,
    pub hash_to_scalar: Option<CreateHashToScalarParams>,
    pub encoding: Option<CreateEncodingParams>,
    pub key_agreement: Option<CreateKeyAgreementParams>,
    pub random_source: Option<CreateRandomSourceParams>,
}

pub fn build_possession_vectors_case_params(
    overrides: PossessionVectorsCaseParamsOverrides,
) -> PossessionVectorsCaseParams {
    PossessionVectorsCaseParams {
        pairing: overrides
            .pairing
            .unwrap_or_else(|| build_create_pairing_params(Default::default())),
        hash_to_scalar: overrides
            .hash_to_scalar
            .unwrap_or_else(|| build_create_hash_to_scalar_params(Default::default())),
        encoding: overrides
            .encoding
            .unwrap_or_else(|| build_create_encoding_params(Default::default())),
        key_agreement: overrides
            .key_agreement
            .unwrap_or_else(|| build_create_key_agreement_params(Default::default())),
        random_source: overrides
            .random_source
            .unwrap_or_else(|| build_create_random_source_params(Default::default())),
    }
}

#[derive(Default)]
pub struct PossessionVectorsCaseDepsOverrides;

pub fn build_possession_vectors_case_deps<C: IPossessionVectorsCaseConsumer>(
    consumer: C,
    _overrides: PossessionVectorsCaseDepsOverrides,
) -> PossessionVectorsCaseDeps<C> {
    PossessionVectorsCaseDeps { consumer }
}

#[derive(Default)]
pub struct PossessionVectorsCasePayloadOverrides;

pub fn build_possession_vectors_case_payload(
    _overrides: PossessionVectorsCasePayloadOverrides,
) -> PossessionVectorsCasePayload {
    PossessionVectorsCasePayload
}

#[derive(Default)]
pub struct PossessionVectorsCaseSuccessReturnOverrides;

pub fn build_possession_vectors_case_success_return<O>(
    output: O,
    _overrides: PossessionVectorsCaseSuccessReturnOverrides,
) -> PossessionVectorsCaseSuccessReturn<O> {
    PossessionVectorsCaseSuccessReturn { output }
}

struct CaseAtKeyAgreement<'a, P: IPairingArithmetic, E, C: IPossessionVectorsCaseConsumer> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    source: &'a dyn IRandomSourceAdapter,
    consumer: &'a C,
}

impl<P, E, C> IKeyAgreementConsumer<P> for CaseAtKeyAgreement<'_, P, E, C>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
    C: IPossessionVectorsCaseConsumer,
{
    type Output = PossessionVectorsCaseReturn<C::Output>;

    fn consume_key_agreement<K: IKeyAgreementAdapter<Pairing = P>>(
        &self,
        _params: ConsumeKeyAgreementParams,
        payload: ConsumeKeyAgreementPayload<K>,
    ) -> Self::Output {
        Ok(PossessionVectorsCaseSuccessReturn {
            output: self.consumer.consume_possession_vectors_case(
                self.pairing,
                self.hash_to_scalar,
                self.encoder,
                self.encoder,
                &payload.adapter,
                self.source,
            ),
        })
    }
}

struct CaseAtEncoding<'a, P: IPairingArithmetic, C: IPossessionVectorsCaseConsumer> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    source: &'a dyn IRandomSourceAdapter,
    key_agreement: &'a CreateKeyAgreementParams,
    consumer: &'a C,
}

impl<P, C> IEncodingConsumer for CaseAtEncoding<'_, P, C>
where
    P: IPairingArithmetic + IPairingReference,
    C: IPossessionVectorsCaseConsumer,
{
    type Output = PossessionVectorsCaseReturn<C::Output>;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
        match create_key_agreement(
            &build_create_key_agreement_deps(
                self.pairing,
                self.hash_to_scalar,
                &payload.adapter,
                self.source,
                CaseAtKeyAgreement {
                    pairing: self.pairing,
                    hash_to_scalar: self.hash_to_scalar,
                    encoder: &payload.adapter,
                    source: self.source,
                    consumer: self.consumer,
                },
                Default::default(),
            ),
            *self.key_agreement,
            build_create_key_agreement_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(PossessionVectorsCaseErrorReturn::CreateKeyAgreement(error)),
        }
    }
}

struct CaseAtPairing<'a, C: IPossessionVectorsCaseConsumer> {
    hash_to_scalar: &'a CreateHashToScalarParams,
    encoding: &'a CreateEncodingParams,
    key_agreement: &'a CreateKeyAgreementParams,
    random_source: &'a CreateRandomSourceParams,
    consumer: &'a C,
}

impl<C: IPossessionVectorsCaseConsumer> IPairingConsumer for CaseAtPairing<'_, C> {
    type Output = PossessionVectorsCaseReturn<C::Output>;

    fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let hash_to_scalar = match create_hash_to_scalar::<P::Scalar>(
            &build_create_hash_to_scalar_deps(Default::default()),
            *self.hash_to_scalar,
            build_create_hash_to_scalar_payload(Default::default()),
        ) {
            Ok(hash_to_scalar) => hash_to_scalar,
            Err(error) => {
                return Err(PossessionVectorsCaseErrorReturn::CreateHashToScalar(error));
            }
        };
        let source = match create_random_source(
            &CreateRandomSourceDeps,
            CreateRandomSourceParams {
                kind: self.random_source.kind,
            },
            CreateRandomSourcePayload,
        ) {
            Ok(source) => source,
            Err(error) => {
                return Err(PossessionVectorsCaseErrorReturn::CreateRandomSource(error));
            }
        };
        match create_encoding(
            &build_create_encoding_deps(CaseAtEncoding {
                pairing: &payload.adapter,
                hash_to_scalar: &*hash_to_scalar.adapter,
                source: &*source.adapter,
                key_agreement: self.key_agreement,
                consumer: self.consumer,
            }),
            CreateEncodingParams {
                concrete: self.encoding.concrete,
                identifier: self.encoding.identifier,
            },
            build_create_encoding_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(PossessionVectorsCaseErrorReturn::CreateEncoding(error)),
        }
    }
}

pub fn run_possession_vectors_case<C: IPossessionVectorsCaseConsumer>(
    deps: &PossessionVectorsCaseDeps<C>,
    params: PossessionVectorsCaseParams,
    _payload: PossessionVectorsCasePayload,
) -> PossessionVectorsCaseReturn<C::Output> {
    let _: RunPossessionVectorsCaseFn<C> = run_possession_vectors_case;
    match create_pairing(
        &build_create_pairing_deps(
            CaseAtPairing {
                hash_to_scalar: &params.hash_to_scalar,
                encoding: &params.encoding,
                key_agreement: &params.key_agreement,
                random_source: &params.random_source,
                consumer: &deps.consumer,
            },
            Default::default(),
        ),
        params.pairing,
        build_create_pairing_payload(Default::default()),
    ) {
        Ok(success) => success.output,
        Err(error) => Err(PossessionVectorsCaseErrorReturn::CreatePairing(error)),
    }
}
