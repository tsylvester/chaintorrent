#![allow(dead_code, clippy::too_many_arguments)]

use super::interface::{
    IUnwrapPieceGroupKeyCaseConsumer, RunUnwrapPieceGroupKeyCaseFn, UnwrapPieceGroupKeyCaseDeps,
    UnwrapPieceGroupKeyCaseErrorReturn, UnwrapPieceGroupKeyCaseParams,
    UnwrapPieceGroupKeyCasePayload, UnwrapPieceGroupKeyCaseReturn,
    UnwrapPieceGroupKeyCaseSuccessReturn, UnwrapPieceGroupKeyDeps, UnwrapPieceGroupKeyParams,
    UnwrapPieceGroupKeyPayload, UnwrapPieceGroupKeyReturn, UnwrapPieceGroupKeySuccessReturn,
};
use crate::sidecar::derivation_context::provides::DerivationContext;
use crate::sidecar::wrap::provides::{
    PieceGroupKey, PieceGroupKeyConstructorParamsOverrides, WrappedPieceGroupKey,
    build_piece_group_key,
};
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
    IKemConsumer, IdentityScope, build_create_kem_deps, build_create_kem_params,
    build_create_kem_payload, create_kem,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParams, IPairingArithmetic,
    IPairingConsumer, IPairingReference, build_create_pairing_deps, build_create_pairing_params,
    build_create_pairing_payload, create_pairing,
};
use random::{
    CreateRandomSourceParams, IRandomSourceAdapter, build_create_random_source_deps,
    build_create_random_source_params, build_create_random_source_payload, create_random_source,
};
use serde_json::{Map, Value};

fn byte_array(bytes: &[u8]) -> Value {
    Value::Array(bytes.iter().map(|byte| Value::from(*byte)).collect())
}

#[derive(Default)]
pub struct UnwrapPieceGroupKeyDepsOverrides;

pub fn build_unwrap_piece_group_key_deps<'a, E: IEncoderAdapter>(
    kdf: &'a dyn IKeyDerivationAdapter,
    encoder: &'a E,
    _overrides: UnwrapPieceGroupKeyDepsOverrides,
) -> UnwrapPieceGroupKeyDeps<'a, E> {
    UnwrapPieceGroupKeyDeps { kdf, encoder }
}

#[derive(Default)]
pub struct UnwrapPieceGroupKeyParamsOverrides;

pub fn build_unwrap_piece_group_key_params(
    _overrides: UnwrapPieceGroupKeyParamsOverrides,
) -> UnwrapPieceGroupKeyParams {
    UnwrapPieceGroupKeyParams
}

#[derive(Default)]
pub struct UnwrapPieceGroupKeyPayloadOverrides;

pub fn build_unwrap_piece_group_key_payload<'a>(
    encapsulated: &'a EncapsulatedValue,
    context: &'a DerivationContext,
    wrapped: &'a WrappedPieceGroupKey,
    _overrides: UnwrapPieceGroupKeyPayloadOverrides,
) -> UnwrapPieceGroupKeyPayload<'a> {
    UnwrapPieceGroupKeyPayload {
        encapsulated,
        context,
        wrapped,
    }
}

#[derive(Default)]
pub struct UnwrapPieceGroupKeySuccessReturnOverrides {
    pub piece_group_key: Option<PieceGroupKey>,
}

pub fn build_unwrap_piece_group_key_success_return(
    overrides: UnwrapPieceGroupKeySuccessReturnOverrides,
) -> UnwrapPieceGroupKeySuccessReturn {
    UnwrapPieceGroupKeySuccessReturn {
        piece_group_key: overrides.piece_group_key.unwrap_or_else(|| {
            build_piece_group_key(PieceGroupKeyConstructorParamsOverrides::default())
        }),
    }
}

#[derive(Default)]
pub struct UnwrapPieceGroupKeySuccessReturnCorruptions {
    pub piece_group_key: Option<Value>,
}

pub fn invalidate_unwrap_piece_group_key_success_return(
    corruptions: UnwrapPieceGroupKeySuccessReturnCorruptions,
) -> Value {
    let success = build_unwrap_piece_group_key_success_return(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "piece_group_key".to_string(),
        byte_array(success.piece_group_key.expose()),
    );
    if let Some(piece_group_key) = corruptions.piece_group_key {
        fields.insert("piece_group_key".to_string(), piece_group_key);
    }
    Value::Object(fields)
}

pub fn mock_unwrap_piece_group_key<E: IEncoderAdapter>(
    _deps: &UnwrapPieceGroupKeyDeps<'_, E>,
    _params: UnwrapPieceGroupKeyParams,
    _payload: UnwrapPieceGroupKeyPayload<'_>,
) -> UnwrapPieceGroupKeyReturn {
    Ok(build_unwrap_piece_group_key_success_return(
        Default::default(),
    ))
}

#[derive(Default)]
pub struct UnwrapPieceGroupKeyCaseParamsOverrides {
    pub encoding: Option<CreateEncodingParams>,
    pub key_derivation: Option<CreateKeyDerivationParams>,
    pub pairing: Option<CreatePairingParams>,
    pub hash_to_scalar: Option<CreateHashToScalarParams>,
    pub random_source: Option<CreateRandomSourceParams>,
    pub kem: Option<CreateKemParams>,
    pub kem_second: Option<CreateKemParams>,
}

pub fn build_unwrap_piece_group_key_case_params(
    overrides: UnwrapPieceGroupKeyCaseParamsOverrides,
) -> UnwrapPieceGroupKeyCaseParams {
    UnwrapPieceGroupKeyCaseParams {
        encoding: overrides
            .encoding
            .unwrap_or_else(|| build_create_encoding_params(Default::default())),
        key_derivation: overrides
            .key_derivation
            .unwrap_or_else(|| build_create_key_derivation_params(Default::default())),
        pairing: overrides
            .pairing
            .unwrap_or_else(|| build_create_pairing_params(Default::default())),
        hash_to_scalar: overrides
            .hash_to_scalar
            .unwrap_or_else(|| build_create_hash_to_scalar_params(Default::default())),
        random_source: overrides
            .random_source
            .unwrap_or_else(|| build_create_random_source_params(Default::default())),
        kem: overrides
            .kem
            .unwrap_or_else(|| build_create_kem_params(Default::default())),
        kem_second: overrides
            .kem_second
            .unwrap_or_else(|| build_create_kem_params(Default::default())),
    }
}

#[derive(Default)]
pub struct UnwrapPieceGroupKeyCaseDepsOverrides;

pub fn build_unwrap_piece_group_key_case_deps<C: IUnwrapPieceGroupKeyCaseConsumer>(
    consumer: C,
    _overrides: UnwrapPieceGroupKeyCaseDepsOverrides,
) -> UnwrapPieceGroupKeyCaseDeps<C> {
    UnwrapPieceGroupKeyCaseDeps { consumer }
}

#[derive(Default)]
pub struct UnwrapPieceGroupKeyCasePayloadOverrides;

pub fn build_unwrap_piece_group_key_case_payload(
    _overrides: UnwrapPieceGroupKeyCasePayloadOverrides,
) -> UnwrapPieceGroupKeyCasePayload {
    UnwrapPieceGroupKeyCasePayload
}

#[derive(Default)]
pub struct UnwrapPieceGroupKeyCaseSuccessReturnOverrides;

pub fn build_unwrap_piece_group_key_case_success_return<O>(
    output: O,
    _overrides: UnwrapPieceGroupKeyCaseSuccessReturnOverrides,
) -> UnwrapPieceGroupKeyCaseSuccessReturn<O> {
    UnwrapPieceGroupKeyCaseSuccessReturn { output }
}

struct CaseAtEncoding<'a, C: IUnwrapPieceGroupKeyCaseConsumer> {
    consumer: &'a C,
    key_derivation: CreateKeyDerivationParams,
    pairing: CreatePairingParams,
    hash_to_scalar: CreateHashToScalarParams,
    random_source: CreateRandomSourceParams,
    kem: CreateKemParams,
    kem_second: CreateKemParams,
}

impl<C: IUnwrapPieceGroupKeyCaseConsumer> IEncodingConsumer for CaseAtEncoding<'_, C> {
    type Output = UnwrapPieceGroupKeyCaseReturn<C::Output>;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
        let created = match create_key_derivation(
            &build_create_key_derivation_deps(Default::default()),
            self.key_derivation,
            build_create_key_derivation_payload(Default::default()),
        ) {
            Ok(created) => created,
            Err(error) => {
                return Err(UnwrapPieceGroupKeyCaseErrorReturn::CreateKeyDerivation(
                    error,
                ));
            }
        };
        match create_pairing(
            &build_create_pairing_deps(
                CaseAtPairing {
                    consumer: self.consumer,
                    encoder: &payload.adapter,
                    kdf: &*created.adapter,
                    hash_to_scalar: self.hash_to_scalar,
                    random_source: &self.random_source,
                    kem: self.kem,
                    kem_second: self.kem_second,
                },
                Default::default(),
            ),
            CreatePairingParams {
                concrete: self.pairing.concrete,
                supported_encodings: self.pairing.supported_encodings.clone(),
                target_group_encoding: self.pairing.target_group_encoding,
            },
            build_create_pairing_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(UnwrapPieceGroupKeyCaseErrorReturn::CreatePairing(error)),
        }
    }
}

struct CaseAtPairing<'a, C: IUnwrapPieceGroupKeyCaseConsumer, E> {
    consumer: &'a C,
    encoder: &'a E,
    kdf: &'a dyn IKeyDerivationAdapter,
    hash_to_scalar: CreateHashToScalarParams,
    random_source: &'a CreateRandomSourceParams,
    kem: CreateKemParams,
    kem_second: CreateKemParams,
}

impl<C: IUnwrapPieceGroupKeyCaseConsumer, E: IEncoderAdapter + IDecoderAdapter> IPairingConsumer
    for CaseAtPairing<'_, C, E>
{
    type Output = UnwrapPieceGroupKeyCaseReturn<C::Output>;

    fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let hashed = match create_hash_to_scalar::<P::Scalar>(
            &build_create_hash_to_scalar_deps(Default::default()),
            self.hash_to_scalar,
            build_create_hash_to_scalar_payload(Default::default()),
        ) {
            Ok(hashed) => hashed,
            Err(error) => {
                return Err(UnwrapPieceGroupKeyCaseErrorReturn::CreateHashToScalar(
                    error,
                ));
            }
        };
        let randomized = match create_random_source(
            &build_create_random_source_deps(Default::default()),
            CreateRandomSourceParams {
                kind: self.random_source.kind,
            },
            build_create_random_source_payload(Default::default()),
        ) {
            Ok(randomized) => randomized,
            Err(error) => {
                return Err(UnwrapPieceGroupKeyCaseErrorReturn::CreateRandomSource(
                    error,
                ));
            }
        };
        match create_kem(
            &build_create_kem_deps(
                &payload.adapter,
                &*hashed.adapter,
                CaseAtKem {
                    consumer: self.consumer,
                    encoder: self.encoder,
                    kdf: self.kdf,
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hashed.adapter,
                    random: &*randomized.adapter,
                    kem_second: self.kem_second,
                },
                Default::default(),
            ),
            self.kem,
            build_create_kem_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(UnwrapPieceGroupKeyCaseErrorReturn::CreateKem(error)),
        }
    }
}

struct CaseAtKem<'a, C: IUnwrapPieceGroupKeyCaseConsumer, E, P: IPairingArithmetic> {
    consumer: &'a C,
    encoder: &'a E,
    kdf: &'a dyn IKeyDerivationAdapter,
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    random: &'a dyn IRandomSourceAdapter,
    kem_second: CreateKemParams,
}

impl<
    C: IUnwrapPieceGroupKeyCaseConsumer,
    E: IEncoderAdapter + IDecoderAdapter,
    P: IPairingArithmetic + IPairingReference,
> IKemConsumer<P> for CaseAtKem<'_, C, E, P>
{
    type Output = UnwrapPieceGroupKeyCaseReturn<C::Output>;

    fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
        &self,
        _params: ConsumeKemParams,
        payload: ConsumeKemPayload<K>,
    ) -> Self::Output {
        match create_kem(
            &build_create_kem_deps(
                self.pairing,
                self.hash_to_scalar,
                CaseAtKemInner {
                    consumer: self.consumer,
                    encoder: self.encoder,
                    kdf: self.kdf,
                    pairing: self.pairing,
                    hash_to_scalar: self.hash_to_scalar,
                    kem_first: &payload.adapter,
                    random: self.random,
                    scope_first: payload.scope(),
                },
                Default::default(),
            ),
            self.kem_second,
            build_create_kem_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(UnwrapPieceGroupKeyCaseErrorReturn::CreateKem(error)),
        }
    }
}

struct CaseAtKemInner<'a, C: IUnwrapPieceGroupKeyCaseConsumer, E, P: IPairingArithmetic, K1> {
    consumer: &'a C,
    encoder: &'a E,
    kdf: &'a dyn IKeyDerivationAdapter,
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    kem_first: &'a K1,
    random: &'a dyn IRandomSourceAdapter,
    scope_first: IdentityScope,
}

impl<
    C: IUnwrapPieceGroupKeyCaseConsumer,
    E: IEncoderAdapter + IDecoderAdapter,
    P: IPairingArithmetic + IPairingReference,
    K1: ICredentialKemAdapter<Pairing = P>,
> IKemConsumer<P> for CaseAtKemInner<'_, C, E, P, K1>
{
    type Output = UnwrapPieceGroupKeyCaseReturn<C::Output>;

    fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
        &self,
        _params: ConsumeKemParams,
        payload: ConsumeKemPayload<K>,
    ) -> Self::Output {
        let output = self.consumer.consume_unwrap_piece_group_key_case(
            self.encoder,
            self.kdf,
            self.pairing,
            self.hash_to_scalar,
            self.kem_first,
            &payload.adapter,
            self.random,
            self.scope_first,
            payload.scope(),
            &UnwrapPieceGroupKeyCasePayload,
        );
        Ok(UnwrapPieceGroupKeyCaseSuccessReturn { output })
    }
}

pub fn run_unwrap_piece_group_key_case<C: IUnwrapPieceGroupKeyCaseConsumer>(
    deps: &UnwrapPieceGroupKeyCaseDeps<C>,
    params: UnwrapPieceGroupKeyCaseParams,
    _payload: UnwrapPieceGroupKeyCasePayload,
) -> UnwrapPieceGroupKeyCaseReturn<C::Output> {
    let _: RunUnwrapPieceGroupKeyCaseFn<C> = run_unwrap_piece_group_key_case;
    match create_encoding(
        &build_create_encoding_deps(CaseAtEncoding {
            consumer: &deps.consumer,
            key_derivation: params.key_derivation,
            pairing: params.pairing,
            hash_to_scalar: params.hash_to_scalar,
            random_source: params.random_source,
            kem: params.kem,
            kem_second: params.kem_second,
        }),
        params.encoding,
        build_create_encoding_payload(Default::default()),
    ) {
        Ok(success) => success.output,
        Err(error) => Err(UnwrapPieceGroupKeyCaseErrorReturn::CreateEncoding(error)),
    }
}
