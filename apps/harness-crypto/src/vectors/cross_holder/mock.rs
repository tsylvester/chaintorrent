#![allow(dead_code, clippy::too_many_arguments)]

use hash_to_scalar::{
    CreateHashToScalarParams, IHashToScalarAdapter, build_create_hash_to_scalar_deps,
    build_create_hash_to_scalar_params, build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use hashing::{Root, RootConstructorParamsOverrides, build_root};
use kem::{
    ConsumeKemParams, ConsumeKemPayload, CreateKemParams, EncapsulatedValue, ICredentialKemAdapter,
    IKemConsumer, IdentityScope, KemIdentity, SetupScope, build_create_kem_deps,
    build_create_kem_params, build_create_kem_payload, create_kem,
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
    CrossHolderCaseDeps, CrossHolderCaseErrorReturn, CrossHolderCaseParams, CrossHolderCasePayload,
    CrossHolderCaseReturn, CrossHolderCaseSuccessReturn, CrossHolderDeps, CrossHolderFn,
    CrossHolderParams, CrossHolderPayload, CrossHolderReturn, CrossHolderSuccessReturn,
    CrossHolderVector, ICrossHolderCaseConsumer, RunCrossHolderCaseFn,
};
use crate::delivery::provides::{AdmittedKemIdentity, build_admitted_kem_identity};
use crate::{VectorCount, build_vector_count};

#[derive(Default)]
pub struct CrossHolderVectorOverrides {
    pub draws: Option<Vec<u8>>,
}

pub fn build_cross_holder_vector<K: ICredentialKemAdapter>(
    parameter_set: K::ParameterSet,
    identity_element: K::IdentityElement,
    issued: K::Credential,
    successor: K::Credential,
    capsule: K::Capsule,
    encapsulated: EncapsulatedValue,
    issued_decapsulated: EncapsulatedValue,
    successor_decapsulated: EncapsulatedValue,
    overrides: CrossHolderVectorOverrides,
) -> CrossHolderVector<K> {
    CrossHolderVector {
        draws: overrides.draws.unwrap_or_else(|| vec![0x01; 64]),
        parameter_set,
        identity_element,
        issued,
        successor,
        capsule,
        encapsulated,
        issued_decapsulated,
        successor_decapsulated,
    }
}

#[derive(Default)]
pub struct CrossHolderParamsOverrides {
    pub count: Option<VectorCount>,
}

pub fn build_cross_holder_params(overrides: CrossHolderParamsOverrides) -> CrossHolderParams {
    CrossHolderParams {
        count: overrides
            .count
            .unwrap_or_else(|| build_vector_count(Default::default())),
    }
}

fn cross_holder_params_fields(params: &CrossHolderParams) -> Map<String, Value> {
    let mut fields = Map::new();
    fields.insert("count".to_string(), Value::from(params.count.get()));
    fields
}

#[derive(Default)]
pub struct CrossHolderParamsCorruptions {
    pub count: Option<Value>,
}

pub fn invalidate_cross_holder_params(corruptions: CrossHolderParamsCorruptions) -> Value {
    let mut fields = cross_holder_params_fields(&build_cross_holder_params(Default::default()));
    if let Some(count) = corruptions.count {
        fields.insert("count".to_string(), count);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct CrossHolderPayloadOverrides<'a> {
    pub identity: Option<AdmittedKemIdentity<'a>>,
}

pub fn build_cross_holder_payload<'a>(
    overrides: CrossHolderPayloadOverrides<'a>,
) -> CrossHolderPayload<'a> {
    CrossHolderPayload {
        identity: overrides
            .identity
            .unwrap_or_else(|| build_admitted_kem_identity(Default::default())),
    }
}

#[derive(Default)]
pub struct CrossHolderDepsOverrides;

pub fn build_cross_holder_deps<'a, K: ICredentialKemAdapter>(
    kem: &'a K,
    random: &'a dyn IRandomSourceAdapter,
    _overrides: CrossHolderDepsOverrides,
) -> CrossHolderDeps<'a, K> {
    CrossHolderDeps { kem, random }
}

pub struct CrossHolderSuccessReturnOverrides<K: ICredentialKemAdapter> {
    pub vectors: Option<Vec<CrossHolderVector<K>>>,
}

impl<K: ICredentialKemAdapter> Default for CrossHolderSuccessReturnOverrides<K> {
    fn default() -> Self {
        Self { vectors: None }
    }
}

pub fn build_cross_holder_success_return<K: ICredentialKemAdapter>(
    overrides: CrossHolderSuccessReturnOverrides<K>,
) -> CrossHolderSuccessReturn<K> {
    CrossHolderSuccessReturn {
        vectors: overrides.vectors.unwrap_or_default(),
    }
}

pub fn mock_cross_holder<'a, K: ICredentialKemAdapter>(
    _deps: &CrossHolderDeps<'a, K>,
    _params: CrossHolderParams,
    _payload: CrossHolderPayload<'_>,
) -> CrossHolderReturn<K> {
    let _: CrossHolderFn<'a, K> = mock_cross_holder;
    Ok(build_cross_holder_success_return(Default::default()))
}

#[derive(Default)]
pub struct CrossHolderCaseParamsOverrides {
    pub pairing: Option<CreatePairingParams>,
    pub hash_to_scalar: Option<CreateHashToScalarParams>,
    pub kem: Option<CreateKemParams>,
    pub random_source: Option<CreateRandomSourceParams>,
}

pub fn build_cross_holder_case_params(
    overrides: CrossHolderCaseParamsOverrides,
) -> CrossHolderCaseParams {
    CrossHolderCaseParams {
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
pub struct CrossHolderCaseDepsOverrides;

pub fn build_cross_holder_case_deps<C: ICrossHolderCaseConsumer>(
    consumer: C,
    _overrides: CrossHolderCaseDepsOverrides,
) -> CrossHolderCaseDeps<C> {
    CrossHolderCaseDeps { consumer }
}

#[derive(Default)]
pub struct CrossHolderCasePayloadOverrides {
    pub canonical: Option<Vec<u8>>,
    pub root: Option<Root>,
}

pub fn build_cross_holder_case_payload(
    overrides: CrossHolderCasePayloadOverrides,
) -> CrossHolderCasePayload {
    CrossHolderCasePayload {
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
pub struct CrossHolderCaseSuccessReturnOverrides;

pub fn build_cross_holder_case_success_return<O>(
    output: O,
    _overrides: CrossHolderCaseSuccessReturnOverrides,
) -> CrossHolderCaseSuccessReturn<O> {
    CrossHolderCaseSuccessReturn { output }
}

struct CaseAtKem<'a, P: IPairingArithmetic, C: ICrossHolderCaseConsumer> {
    pairing: &'a P,
    hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
    random: &'a dyn IRandomSourceAdapter,
    independent: &'a dyn IRandomSourceAdapter,
    payload: &'a CrossHolderCasePayload,
    consumer: &'a C,
}

impl<'a, P, C> IKemConsumer<P> for CaseAtKem<'a, P, C>
where
    P: IPairingArithmetic + IPairingReference,
    C: ICrossHolderCaseConsumer,
{
    type Output = CrossHolderCaseReturn<C::Output>;

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
        let output = self.consumer.consume_cross_holder_case(
            self.pairing,
            self.hashing,
            &payload.adapter,
            self.random,
            self.independent,
            identity,
        );
        Ok(CrossHolderCaseSuccessReturn { output })
    }
}

struct CaseAtPairing<'a, C: ICrossHolderCaseConsumer> {
    hash_to_scalar: &'a CreateHashToScalarParams,
    random_source: &'a CreateRandomSourceParams,
    kem: &'a CreateKemParams,
    payload: &'a CrossHolderCasePayload,
    consumer: &'a C,
}

impl<C: ICrossHolderCaseConsumer> IPairingConsumer for CaseAtPairing<'_, C> {
    type Output = CrossHolderCaseReturn<C::Output>;

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
                return Err(CrossHolderCaseErrorReturn::CreateHashToScalar(error));
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
                return Err(CrossHolderCaseErrorReturn::CreateRandomSource(error));
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
                return Err(CrossHolderCaseErrorReturn::CreateRandomSource(error));
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
            Err(error) => Err(CrossHolderCaseErrorReturn::CreateKem(error)),
        }
    }
}

pub fn run_cross_holder_case<C: ICrossHolderCaseConsumer>(
    deps: &CrossHolderCaseDeps<C>,
    params: CrossHolderCaseParams,
    payload: CrossHolderCasePayload,
) -> CrossHolderCaseReturn<C::Output> {
    let _: RunCrossHolderCaseFn<C> = run_cross_holder_case;
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
        Err(error) => Err(CrossHolderCaseErrorReturn::CreatePairing(error)),
    }
}
