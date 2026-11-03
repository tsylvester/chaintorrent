#![allow(dead_code, clippy::too_many_arguments)]

use hash_to_scalar::{
    CreateHashToScalarParams, IHashToScalarAdapter, build_create_hash_to_scalar_deps,
    build_create_hash_to_scalar_params, build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use hashing::{Root, RootConstructorParamsOverrides, build_root};
use kem::{
    ConsumeKemParams, ConsumeKemPayload, CreateKemParams, ICredentialKemAdapter, IKemConsumer,
    IdentityScope, KemIdentity, SetupScope, build_create_kem_deps, build_create_kem_params,
    build_create_kem_payload, create_kem,
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

use super::interface::{
    IRerandomizationCaseConsumer, RerandomizationCaseDeps, RerandomizationCaseErrorReturn,
    RerandomizationCaseParams, RerandomizationCasePayload, RerandomizationCaseReturn,
    RerandomizationCaseSuccessReturn, RerandomizationDeps, RerandomizationFn,
    RerandomizationParams, RerandomizationPayload, RerandomizationReturn,
    RerandomizationSuccessReturn, RerandomizationVector, RunRerandomizationCaseFn,
};
use crate::delivery::provides::{AdmittedKemIdentity, build_admitted_kem_identity};
use crate::{VectorCount, build_vector_count};

#[derive(Default)]
pub struct RerandomizationVectorOverrides {
    pub draws: Option<Vec<u8>>,
    pub issued_is_valid: Option<bool>,
    pub rerandomized_is_valid: Option<bool>,
}

pub fn build_rerandomization_vector<K: ICredentialKemAdapter>(
    parameter_set: K::ParameterSet,
    identity_element: K::IdentityElement,
    issued: K::Credential,
    rerandomized: K::Credential,
    overrides: RerandomizationVectorOverrides,
) -> RerandomizationVector<K> {
    RerandomizationVector {
        draws: overrides.draws.unwrap_or_else(|| vec![0x01; 64]),
        parameter_set,
        identity_element,
        issued,
        rerandomized,
        issued_is_valid: overrides.issued_is_valid.unwrap_or(true),
        rerandomized_is_valid: overrides.rerandomized_is_valid.unwrap_or(true),
    }
}

#[derive(Default)]
pub struct RerandomizationParamsOverrides {
    pub count: Option<VectorCount>,
}

pub fn build_rerandomization_params(
    overrides: RerandomizationParamsOverrides,
) -> RerandomizationParams {
    RerandomizationParams {
        count: overrides
            .count
            .unwrap_or_else(|| build_vector_count(Default::default())),
    }
}

fn rerandomization_params_fields(params: &RerandomizationParams) -> Map<String, Value> {
    let mut fields = Map::new();
    fields.insert("count".to_string(), Value::from(params.count.get()));
    fields
}

#[derive(Default)]
pub struct RerandomizationParamsCorruptions {
    pub count: Option<Value>,
}

pub fn invalidate_rerandomization_params(corruptions: RerandomizationParamsCorruptions) -> Value {
    let mut fields =
        rerandomization_params_fields(&build_rerandomization_params(Default::default()));
    if let Some(count) = corruptions.count {
        fields.insert("count".to_string(), count);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct RerandomizationPayloadOverrides<'a> {
    pub identity: Option<AdmittedKemIdentity<'a>>,
}

pub fn build_rerandomization_payload<'a>(
    overrides: RerandomizationPayloadOverrides<'a>,
) -> RerandomizationPayload<'a> {
    RerandomizationPayload {
        identity: overrides
            .identity
            .unwrap_or_else(|| build_admitted_kem_identity(Default::default())),
    }
}

#[derive(Default)]
pub struct RerandomizationDepsOverrides;

pub fn build_rerandomization_deps<'a, K: ICredentialKemAdapter>(
    kem: &'a K,
    random: &'a dyn IRandomSourceAdapter,
    _overrides: RerandomizationDepsOverrides,
) -> RerandomizationDeps<'a, K> {
    RerandomizationDeps { kem, random }
}

pub struct RerandomizationSuccessReturnOverrides<K: ICredentialKemAdapter> {
    pub vectors: Option<Vec<RerandomizationVector<K>>>,
}

impl<K: ICredentialKemAdapter> Default for RerandomizationSuccessReturnOverrides<K> {
    fn default() -> Self {
        Self { vectors: None }
    }
}

pub fn build_rerandomization_success_return<K: ICredentialKemAdapter>(
    overrides: RerandomizationSuccessReturnOverrides<K>,
) -> RerandomizationSuccessReturn<K> {
    RerandomizationSuccessReturn {
        vectors: overrides.vectors.unwrap_or_default(),
    }
}

pub fn mock_rerandomization<'a, K: ICredentialKemAdapter>(
    _deps: &RerandomizationDeps<'a, K>,
    _params: RerandomizationParams,
    _payload: RerandomizationPayload<'_>,
) -> RerandomizationReturn<K> {
    let _: RerandomizationFn<'a, K> = mock_rerandomization;
    Ok(build_rerandomization_success_return(Default::default()))
}

#[derive(Default)]
pub struct RerandomizationCaseParamsOverrides {
    pub pairing: Option<CreatePairingParams>,
    pub hash_to_scalar: Option<CreateHashToScalarParams>,
    pub kem: Option<CreateKemParams>,
    pub random_source: Option<CreateRandomSourceParams>,
}

pub fn build_rerandomization_case_params(
    overrides: RerandomizationCaseParamsOverrides,
) -> RerandomizationCaseParams {
    RerandomizationCaseParams {
        pairing: overrides
            .pairing
            .unwrap_or_else(|| build_create_pairing_params(Default::default())),
        hash_to_scalar: overrides
            .hash_to_scalar
            .unwrap_or_else(|| build_create_hash_to_scalar_params(Default::default())),
        kem: overrides
            .kem
            .unwrap_or_else(|| build_create_kem_params(Default::default())),
        random_source: overrides
            .random_source
            .unwrap_or_else(|| build_create_random_source_params(Default::default())),
    }
}

#[derive(Default)]
pub struct RerandomizationCaseDepsOverrides;

pub fn build_rerandomization_case_deps<C: IRerandomizationCaseConsumer>(
    consumer: C,
    _overrides: RerandomizationCaseDepsOverrides,
) -> RerandomizationCaseDeps<C> {
    RerandomizationCaseDeps { consumer }
}

#[derive(Default)]
pub struct RerandomizationCasePayloadOverrides {
    pub canonical: Option<Vec<u8>>,
    pub root: Option<Root>,
}

pub fn build_rerandomization_case_payload(
    overrides: RerandomizationCasePayloadOverrides,
) -> RerandomizationCasePayload {
    RerandomizationCasePayload {
        canonical: overrides
            .canonical
            .unwrap_or_else(|| b"pkg@1.0.0#1".to_vec()),
        root: overrides.root.unwrap_or_else(|| {
            build_root(RootConstructorParamsOverrides {
                bytes: Some([0x44; 32]),
            })
        }),
    }
}

#[derive(Default)]
pub struct RerandomizationCaseSuccessReturnOverrides;

pub fn build_rerandomization_case_success_return<O>(
    output: O,
    _overrides: RerandomizationCaseSuccessReturnOverrides,
) -> RerandomizationCaseSuccessReturn<O> {
    RerandomizationCaseSuccessReturn { output }
}

struct CaseAtKem<'a, P: IPairingArithmetic, C: IRerandomizationCaseConsumer> {
    pairing: &'a P,
    hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
    random: &'a dyn IRandomSourceAdapter,
    independent: &'a dyn IRandomSourceAdapter,
    payload: &'a RerandomizationCasePayload,
    consumer: &'a C,
}

impl<'a, P, C> IKemConsumer<P> for CaseAtKem<'a, P, C>
where
    P: IPairingArithmetic + IPairingReference,
    C: IRerandomizationCaseConsumer,
{
    type Output = RerandomizationCaseReturn<C::Output>;

    fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
        &self,
        _params: ConsumeKemParams,
        payload: ConsumeKemPayload<K>,
    ) -> Self::Output {
        let identity = match payload.scope() {
            IdentityScope::Entitlement => AdmittedKemIdentity {
                setup_scope: SetupScope::Entitlement,
                identity: KemIdentity::Entitlement {
                    canonical: &self.payload.canonical,
                },
            },
            IdentityScope::Asset => AdmittedKemIdentity {
                setup_scope: SetupScope::Asset {
                    root: &self.payload.root,
                },
                identity: KemIdentity::Asset {
                    root: &self.payload.root,
                },
            },
        };
        let output = self.consumer.consume_rerandomization_case(
            self.pairing,
            self.hashing,
            &payload.adapter,
            self.random,
            self.independent,
            identity,
        );
        Ok(RerandomizationCaseSuccessReturn { output })
    }
}

struct CaseAtPairing<'a, C: IRerandomizationCaseConsumer> {
    hash_to_scalar: &'a CreateHashToScalarParams,
    random_source: &'a CreateRandomSourceParams,
    kem: &'a CreateKemParams,
    payload: &'a RerandomizationCasePayload,
    consumer: &'a C,
}

impl<C: IRerandomizationCaseConsumer> IPairingConsumer for CaseAtPairing<'_, C> {
    type Output = RerandomizationCaseReturn<C::Output>;

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
                return Err(RerandomizationCaseErrorReturn::CreateHashToScalar(error));
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
                return Err(RerandomizationCaseErrorReturn::CreateRandomSource(error));
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
                return Err(RerandomizationCaseErrorReturn::CreateRandomSource(error));
            }
        };
        match create_kem(
            &build_create_kem_deps(
                &payload.adapter,
                &*hashing.adapter,
                CaseAtKem {
                    pairing: &payload.adapter,
                    hashing: &*hashing.adapter,
                    random: &*random.adapter,
                    independent: &*independent.adapter,
                    payload: self.payload,
                    consumer: self.consumer,
                },
                Default::default(),
            ),
            *self.kem,
            build_create_kem_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(RerandomizationCaseErrorReturn::CreateKem(error)),
        }
    }
}

pub fn run_rerandomization_case<C: IRerandomizationCaseConsumer>(
    deps: &RerandomizationCaseDeps<C>,
    params: RerandomizationCaseParams,
    payload: RerandomizationCasePayload,
) -> RerandomizationCaseReturn<C::Output> {
    let _: RunRerandomizationCaseFn<C> = run_rerandomization_case;
    match create_pairing(
        &build_create_pairing_deps(
            CaseAtPairing {
                hash_to_scalar: &params.hash_to_scalar,
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
        Err(error) => Err(RerandomizationCaseErrorReturn::CreatePairing(error)),
    }
}
