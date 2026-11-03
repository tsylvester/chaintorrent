#![allow(dead_code, clippy::too_many_arguments)]

use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingParams, IDecoderAdapter,
    IEncoderAdapter, IEncodingConsumer, build_create_encoding_deps, build_create_encoding_params,
    build_create_encoding_payload, create_encoding,
};
use hash_to_scalar::{
    CreateHashToScalarParams, IHashToScalarAdapter, build_create_hash_to_scalar_deps,
    build_create_hash_to_scalar_params, build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use kdf::{
    CreateKeyDerivationParams, IKeyDerivationAdapter, build_create_key_derivation_deps,
    build_create_key_derivation_params, build_create_key_derivation_payload, create_key_derivation,
};
use kem::{
    ConsumeKemParams, ConsumeKemPayload, CreateKemParams, EncapsulatedValue, ICredentialKemAdapter,
    IKemConsumer, build_create_kem_deps, build_create_kem_params, build_create_kem_payload,
    create_kem,
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
use serde_json::{Map, Value};
use workflows::{
    DerivationContext, PieceGroupKey, WrappedPieceGroupKey, build_derivation_context,
    build_piece_group_key, build_wrapped_piece_group_key,
};

use super::interface::{
    CrossSetCaseDeps, CrossSetCaseErrorReturn, CrossSetCaseParams, CrossSetCasePayload,
    CrossSetCaseReturn, CrossSetCaseSuccessReturn, CrossSetDeps, CrossSetFn, CrossSetMember,
    CrossSetParams, CrossSetPayload, CrossSetReturn, CrossSetSuccessReturn, CrossSetVector,
    ICrossSetCaseConsumer, RunCrossSetCaseFn,
};
use crate::delivery::provides::{AdmittedKemIdentity, build_admitted_kem_identity};
use crate::{VectorCount, build_vector_count};

#[derive(Default)]
pub struct CrossSetMemberOverrides {
    pub wrapped: Option<WrappedPieceGroupKey>,
    pub unwrapped: Option<PieceGroupKey>,
}

pub fn build_cross_set_member<K: ICredentialKemAdapter>(
    parameter_set: K::ParameterSet,
    identity_element: K::IdentityElement,
    credential: K::Credential,
    capsule: K::Capsule,
    encapsulated: EncapsulatedValue,
    decapsulated: EncapsulatedValue,
    overrides: CrossSetMemberOverrides,
) -> CrossSetMember<K> {
    CrossSetMember {
        parameter_set,
        identity_element,
        credential,
        capsule,
        encapsulated,
        wrapped: overrides
            .wrapped
            .unwrap_or_else(|| build_wrapped_piece_group_key(Default::default())),
        decapsulated,
        unwrapped: overrides
            .unwrapped
            .unwrap_or_else(|| build_piece_group_key(Default::default())),
    }
}

#[derive(Default)]
pub struct CrossSetVectorOverrides {
    pub draws: Option<Vec<u8>>,
    pub piece_group_key: Option<PieceGroupKey>,
}

pub fn build_cross_set_vector<K: ICredentialKemAdapter>(
    first: CrossSetMember<K>,
    second: CrossSetMember<K>,
    overrides: CrossSetVectorOverrides,
) -> CrossSetVector<K> {
    CrossSetVector {
        draws: overrides.draws.unwrap_or_else(|| vec![0x01; 64]),
        piece_group_key: overrides
            .piece_group_key
            .unwrap_or_else(|| build_piece_group_key(Default::default())),
        first,
        second,
    }
}

#[derive(Default)]
pub struct CrossSetParamsOverrides {
    pub count: Option<VectorCount>,
}

pub fn build_cross_set_params(overrides: CrossSetParamsOverrides) -> CrossSetParams {
    CrossSetParams {
        count: overrides
            .count
            .unwrap_or_else(|| build_vector_count(Default::default())),
    }
}

fn cross_set_params_fields(params: &CrossSetParams) -> Map<String, Value> {
    let mut fields = Map::new();
    fields.insert("count".to_string(), Value::from(params.count.get()));
    fields
}

#[derive(Default)]
pub struct CrossSetParamsCorruptions {
    pub count: Option<Value>,
}

pub fn invalidate_cross_set_params(corruptions: CrossSetParamsCorruptions) -> Value {
    let mut fields = cross_set_params_fields(&build_cross_set_params(Default::default()));
    if let Some(count) = corruptions.count {
        fields.insert("count".to_string(), count);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct CrossSetPayloadOverrides<'a> {
    pub identity: Option<AdmittedKemIdentity<'a>>,
}

pub fn build_cross_set_payload<'a>(
    context: &'a DerivationContext,
    overrides: CrossSetPayloadOverrides<'a>,
) -> CrossSetPayload<'a> {
    CrossSetPayload {
        identity: overrides
            .identity
            .unwrap_or_else(|| build_admitted_kem_identity(Default::default())),
        context,
    }
}

#[derive(Default)]
pub struct CrossSetDepsOverrides;

pub fn build_cross_set_deps<'a, K: ICredentialKemAdapter, E: IEncoderAdapter>(
    kem: &'a K,
    random: &'a dyn IRandomSourceAdapter,
    kdf: &'a dyn IKeyDerivationAdapter,
    encoder: &'a E,
    _overrides: CrossSetDepsOverrides,
) -> CrossSetDeps<'a, K, E> {
    CrossSetDeps {
        kem,
        random,
        kdf,
        encoder,
    }
}

pub struct CrossSetSuccessReturnOverrides<K: ICredentialKemAdapter> {
    pub vectors: Option<Vec<CrossSetVector<K>>>,
}

impl<K: ICredentialKemAdapter> Default for CrossSetSuccessReturnOverrides<K> {
    fn default() -> Self {
        Self { vectors: None }
    }
}

pub fn build_cross_set_success_return<K: ICredentialKemAdapter>(
    overrides: CrossSetSuccessReturnOverrides<K>,
) -> CrossSetSuccessReturn<K> {
    CrossSetSuccessReturn {
        vectors: overrides.vectors.unwrap_or_default(),
    }
}

pub fn mock_cross_set<'a, K: ICredentialKemAdapter, E: IEncoderAdapter>(
    _deps: &CrossSetDeps<'a, K, E>,
    _params: CrossSetParams,
    _payload: CrossSetPayload<'_>,
) -> CrossSetReturn<K> {
    let _: CrossSetFn<'a, K, E> = mock_cross_set;
    Ok(build_cross_set_success_return(Default::default()))
}

#[derive(Default)]
pub struct CrossSetCaseParamsOverrides {
    pub pairing: Option<CreatePairingParams>,
    pub hash_to_scalar: Option<CreateHashToScalarParams>,
    pub encoding: Option<CreateEncodingParams>,
    pub kem: Option<CreateKemParams>,
    pub key_derivation: Option<CreateKeyDerivationParams>,
    pub random_source: Option<CreateRandomSourceParams>,
}

pub fn build_cross_set_case_params(overrides: CrossSetCaseParamsOverrides) -> CrossSetCaseParams {
    CrossSetCaseParams {
        pairing: overrides
            .pairing
            .unwrap_or_else(|| build_create_pairing_params(Default::default())),
        hash_to_scalar: overrides
            .hash_to_scalar
            .unwrap_or_else(|| build_create_hash_to_scalar_params(Default::default())),
        encoding: overrides
            .encoding
            .unwrap_or_else(|| build_create_encoding_params(Default::default())),
        kem: overrides
            .kem
            .unwrap_or_else(|| build_create_kem_params(Default::default())),
        key_derivation: overrides
            .key_derivation
            .unwrap_or_else(|| build_create_key_derivation_params(Default::default())),
        random_source: overrides
            .random_source
            .unwrap_or_else(|| build_create_random_source_params(Default::default())),
    }
}

#[derive(Default)]
pub struct CrossSetCaseDepsOverrides;

pub fn build_cross_set_case_deps<C: ICrossSetCaseConsumer>(
    consumer: C,
    _overrides: CrossSetCaseDepsOverrides,
) -> CrossSetCaseDeps<C> {
    CrossSetCaseDeps { consumer }
}

#[derive(Default)]
pub struct CrossSetCasePayloadOverrides {
    pub identity: Option<Vec<u8>>,
    pub context: Option<DerivationContext>,
}

pub fn build_cross_set_case_payload(
    overrides: CrossSetCasePayloadOverrides,
) -> CrossSetCasePayload {
    CrossSetCasePayload {
        identity: overrides
            .identity
            .unwrap_or_else(|| b"pkg@1.0.0#1".to_vec()),
        context: overrides
            .context
            .unwrap_or_else(|| build_derivation_context(Default::default())),
    }
}

#[derive(Default)]
pub struct CrossSetCaseSuccessReturnOverrides;

pub fn build_cross_set_case_success_return<O>(
    output: O,
    _overrides: CrossSetCaseSuccessReturnOverrides,
) -> CrossSetCaseSuccessReturn<O> {
    CrossSetCaseSuccessReturn { output }
}

struct CaseAtKem<'a, P: IPairingArithmetic, C: ICrossSetCaseConsumer, E> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    kdf: &'a dyn IKeyDerivationAdapter,
    random: &'a dyn IRandomSourceAdapter,
    independent: &'a dyn IRandomSourceAdapter,
    payload: &'a CrossSetCasePayload,
    consumer: &'a C,
}

impl<P, C, E> IKemConsumer<P> for CaseAtKem<'_, P, C, E>
where
    P: IPairingArithmetic + IPairingReference,
    C: ICrossSetCaseConsumer,
    E: IEncoderAdapter,
{
    type Output = CrossSetCaseReturn<C::Output>;

    fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
        &self,
        _params: ConsumeKemParams,
        payload: ConsumeKemPayload<K>,
    ) -> Self::Output {
        let output = self.consumer.consume_cross_set_case(
            self.pairing,
            self.hash_to_scalar,
            &payload.adapter,
            self.encoder,
            self.kdf,
            self.random,
            self.independent,
            payload.scope(),
            &self.payload.identity,
            &self.payload.context,
        );
        Ok(CrossSetCaseSuccessReturn { output })
    }
}

struct CaseAtEncoding<'a, P: IPairingArithmetic, C: ICrossSetCaseConsumer> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    kdf: &'a dyn IKeyDerivationAdapter,
    random: &'a dyn IRandomSourceAdapter,
    independent: &'a dyn IRandomSourceAdapter,
    kem: &'a CreateKemParams,
    payload: &'a CrossSetCasePayload,
    consumer: &'a C,
}

impl<P, C> IEncodingConsumer for CaseAtEncoding<'_, P, C>
where
    P: IPairingArithmetic + IPairingReference,
    C: ICrossSetCaseConsumer,
{
    type Output = CrossSetCaseReturn<C::Output>;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
        match create_kem(
            &build_create_kem_deps(
                self.pairing,
                self.hash_to_scalar,
                CaseAtKem {
                    pairing: self.pairing,
                    hash_to_scalar: self.hash_to_scalar,
                    encoder: &payload.adapter,
                    kdf: self.kdf,
                    random: self.random,
                    independent: self.independent,
                    payload: self.payload,
                    consumer: self.consumer,
                },
                Default::default(),
            ),
            *self.kem,
            build_create_kem_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(CrossSetCaseErrorReturn::CreateKem(error)),
        }
    }
}

struct CaseAtPairing<'a, C: ICrossSetCaseConsumer> {
    hash_to_scalar: &'a CreateHashToScalarParams,
    key_derivation: &'a CreateKeyDerivationParams,
    encoding: &'a CreateEncodingParams,
    random_source: &'a CreateRandomSourceParams,
    kem: &'a CreateKemParams,
    payload: &'a CrossSetCasePayload,
    consumer: &'a C,
}

impl<C: ICrossSetCaseConsumer> IPairingConsumer for CaseAtPairing<'_, C> {
    type Output = CrossSetCaseReturn<C::Output>;

    fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let hashing = match create_hash_to_scalar::<P::Scalar>(
            &build_create_hash_to_scalar_deps(Default::default()),
            *self.hash_to_scalar,
            build_create_hash_to_scalar_payload(Default::default()),
        ) {
            Ok(hashing) => hashing,
            Err(error) => {
                return Err(CrossSetCaseErrorReturn::CreateHashToScalar(error));
            }
        };
        let kdf = match create_key_derivation(
            &build_create_key_derivation_deps(Default::default()),
            *self.key_derivation,
            build_create_key_derivation_payload(Default::default()),
        ) {
            Ok(kdf) => kdf,
            Err(error) => {
                return Err(CrossSetCaseErrorReturn::CreateKeyDerivation(error));
            }
        };
        let random = match create_random_source(
            &CreateRandomSourceDeps,
            CreateRandomSourceParams {
                kind: self.random_source.kind,
            },
            CreateRandomSourcePayload,
        ) {
            Ok(random) => random,
            Err(error) => {
                return Err(CrossSetCaseErrorReturn::CreateRandomSource(error));
            }
        };
        let independent = match create_random_source(
            &CreateRandomSourceDeps,
            CreateRandomSourceParams {
                kind: self.random_source.kind,
            },
            CreateRandomSourcePayload,
        ) {
            Ok(independent) => independent,
            Err(error) => {
                return Err(CrossSetCaseErrorReturn::CreateRandomSource(error));
            }
        };
        match create_encoding(
            &build_create_encoding_deps(CaseAtEncoding {
                pairing: &payload.adapter,
                hash_to_scalar: &*hashing.adapter,
                kdf: &*kdf.adapter,
                random: &*random.adapter,
                independent: &*independent.adapter,
                kem: self.kem,
                payload: self.payload,
                consumer: self.consumer,
            }),
            CreateEncodingParams {
                concrete: self.encoding.concrete,
                identifier: self.encoding.identifier,
            },
            build_create_encoding_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(CrossSetCaseErrorReturn::CreateEncoding(error)),
        }
    }
}

pub fn run_cross_set_case<C: ICrossSetCaseConsumer>(
    deps: &CrossSetCaseDeps<C>,
    params: CrossSetCaseParams,
    payload: CrossSetCasePayload,
) -> CrossSetCaseReturn<C::Output> {
    let _: RunCrossSetCaseFn<C> = run_cross_set_case;
    match create_pairing(
        &build_create_pairing_deps(
            CaseAtPairing {
                hash_to_scalar: &params.hash_to_scalar,
                key_derivation: &params.key_derivation,
                encoding: &params.encoding,
                random_source: &params.random_source,
                kem: &params.kem,
                payload: &payload,
                consumer: &deps.consumer,
            },
            Default::default(),
        ),
        params.pairing,
        build_create_pairing_payload(Default::default()),
    ) {
        Ok(success) => success.output,
        Err(error) => Err(CrossSetCaseErrorReturn::CreatePairing(error)),
    }
}
