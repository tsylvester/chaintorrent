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
    IMalformedCapsulesCaseConsumer, MalformedCapsulesCaseDeps, MalformedCapsulesCaseErrorReturn,
    MalformedCapsulesCaseParams, MalformedCapsulesCasePayload, MalformedCapsulesCaseReturn,
    MalformedCapsulesCaseSuccessReturn, MalformedCapsulesDeps, MalformedCapsulesFn,
    MalformedCapsulesParams, MalformedCapsulesPayload, MalformedCapsulesReturn,
    MalformedCapsulesSuccessReturn, MalformedCapsulesVector, RunMalformedCapsulesCaseFn,
};
use crate::delivery::provides::{AdmittedKemIdentity, build_admitted_kem_identity};
use crate::{VectorCount, build_vector_count};

#[derive(Default)]
pub struct MalformedCapsulesVectorOverrides {
    pub draws: Option<Vec<u8>>,
    pub is_well_formed: Option<bool>,
    pub displaced_is_well_formed: Option<bool>,
}

pub fn build_malformed_capsules_vector<K: ICredentialKemAdapter>(
    parameter_set: K::ParameterSet,
    capsule: K::Capsule,
    displaced: K::Capsule,
    overrides: MalformedCapsulesVectorOverrides,
) -> MalformedCapsulesVector<K> {
    MalformedCapsulesVector {
        draws: overrides.draws.unwrap_or_else(|| vec![0x01; 64]),
        parameter_set,
        capsule,
        displaced,
        is_well_formed: overrides.is_well_formed.unwrap_or(true),
        displaced_is_well_formed: overrides.displaced_is_well_formed.unwrap_or(false),
    }
}

#[derive(Default)]
pub struct MalformedCapsulesParamsOverrides {
    pub count: Option<VectorCount>,
}

pub fn build_malformed_capsules_params(
    overrides: MalformedCapsulesParamsOverrides,
) -> MalformedCapsulesParams {
    MalformedCapsulesParams {
        count: overrides
            .count
            .unwrap_or_else(|| build_vector_count(Default::default())),
    }
}

fn malformed_capsules_params_fields(params: &MalformedCapsulesParams) -> Map<String, Value> {
    let mut fields = Map::new();
    fields.insert("count".to_string(), Value::from(params.count.get()));
    fields
}

#[derive(Default)]
pub struct MalformedCapsulesParamsCorruptions {
    pub count: Option<Value>,
}

pub fn invalidate_malformed_capsules_params(
    corruptions: MalformedCapsulesParamsCorruptions,
) -> Value {
    let mut fields =
        malformed_capsules_params_fields(&build_malformed_capsules_params(Default::default()));
    if let Some(count) = corruptions.count {
        fields.insert("count".to_string(), count);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct MalformedCapsulesPayloadOverrides<'a> {
    pub identity: Option<AdmittedKemIdentity<'a>>,
}

pub fn build_malformed_capsules_payload<'a>(
    overrides: MalformedCapsulesPayloadOverrides<'a>,
) -> MalformedCapsulesPayload<'a> {
    MalformedCapsulesPayload {
        identity: overrides
            .identity
            .unwrap_or_else(|| build_admitted_kem_identity(Default::default())),
    }
}

#[derive(Default)]
pub struct MalformedCapsulesDepsOverrides;

pub fn build_malformed_capsules_deps<'a, K: ICredentialKemAdapter>(
    kem: &'a K,
    random: &'a dyn IRandomSourceAdapter,
    _overrides: MalformedCapsulesDepsOverrides,
) -> MalformedCapsulesDeps<'a, K> {
    MalformedCapsulesDeps { kem, random }
}

pub struct MalformedCapsulesSuccessReturnOverrides<K: ICredentialKemAdapter> {
    pub vectors: Option<Vec<MalformedCapsulesVector<K>>>,
}

impl<K: ICredentialKemAdapter> Default for MalformedCapsulesSuccessReturnOverrides<K> {
    fn default() -> Self {
        Self { vectors: None }
    }
}

pub fn build_malformed_capsules_success_return<K: ICredentialKemAdapter>(
    overrides: MalformedCapsulesSuccessReturnOverrides<K>,
) -> MalformedCapsulesSuccessReturn<K> {
    MalformedCapsulesSuccessReturn {
        vectors: overrides.vectors.unwrap_or_default(),
    }
}

pub fn mock_malformed_capsules<'a, K: ICredentialKemAdapter>(
    _deps: &MalformedCapsulesDeps<'a, K>,
    _params: MalformedCapsulesParams,
    _payload: MalformedCapsulesPayload<'_>,
) -> MalformedCapsulesReturn<K> {
    let _: MalformedCapsulesFn<'a, K> = mock_malformed_capsules;
    Ok(build_malformed_capsules_success_return(Default::default()))
}

#[derive(Default)]
pub struct MalformedCapsulesCaseParamsOverrides {
    pub pairing: Option<CreatePairingParams>,
    pub hash_to_scalar: Option<CreateHashToScalarParams>,
    pub kem: Option<CreateKemParams>,
    pub random_source: Option<CreateRandomSourceParams>,
}

pub fn build_malformed_capsules_case_params(
    overrides: MalformedCapsulesCaseParamsOverrides,
) -> MalformedCapsulesCaseParams {
    MalformedCapsulesCaseParams {
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
pub struct MalformedCapsulesCaseDepsOverrides;

pub fn build_malformed_capsules_case_deps<C: IMalformedCapsulesCaseConsumer>(
    consumer: C,
    _overrides: MalformedCapsulesCaseDepsOverrides,
) -> MalformedCapsulesCaseDeps<C> {
    MalformedCapsulesCaseDeps { consumer }
}

#[derive(Default)]
pub struct MalformedCapsulesCasePayloadOverrides {
    pub canonical: Option<Vec<u8>>,
    pub root: Option<Root>,
}

pub fn build_malformed_capsules_case_payload(
    overrides: MalformedCapsulesCasePayloadOverrides,
) -> MalformedCapsulesCasePayload {
    MalformedCapsulesCasePayload {
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
pub struct MalformedCapsulesCaseSuccessReturnOverrides;

pub fn build_malformed_capsules_case_success_return<O>(
    output: O,
    _overrides: MalformedCapsulesCaseSuccessReturnOverrides,
) -> MalformedCapsulesCaseSuccessReturn<O> {
    MalformedCapsulesCaseSuccessReturn { output }
}

struct CaseAtKem<'a, P: IPairingArithmetic, C: IMalformedCapsulesCaseConsumer> {
    pairing: &'a P,
    hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
    random: &'a dyn IRandomSourceAdapter,
    independent: &'a dyn IRandomSourceAdapter,
    payload: &'a MalformedCapsulesCasePayload,
    consumer: &'a C,
}

impl<'a, P, C> IKemConsumer<P> for CaseAtKem<'a, P, C>
where
    P: IPairingArithmetic + IPairingReference,
    C: IMalformedCapsulesCaseConsumer,
{
    type Output = MalformedCapsulesCaseReturn<C::Output>;

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
        let output = self.consumer.consume_malformed_capsules_case(
            self.pairing,
            self.hashing,
            &payload.adapter,
            self.random,
            self.independent,
            identity,
        );
        Ok(MalformedCapsulesCaseSuccessReturn { output })
    }
}

struct CaseAtPairing<'a, C: IMalformedCapsulesCaseConsumer> {
    hash_to_scalar: &'a CreateHashToScalarParams,
    random_source: &'a CreateRandomSourceParams,
    kem: &'a CreateKemParams,
    payload: &'a MalformedCapsulesCasePayload,
    consumer: &'a C,
}

impl<C: IMalformedCapsulesCaseConsumer> IPairingConsumer for CaseAtPairing<'_, C> {
    type Output = MalformedCapsulesCaseReturn<C::Output>;

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
                return Err(MalformedCapsulesCaseErrorReturn::CreateHashToScalar(error));
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
                return Err(MalformedCapsulesCaseErrorReturn::CreateRandomSource(error));
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
                return Err(MalformedCapsulesCaseErrorReturn::CreateRandomSource(error));
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
            Err(error) => Err(MalformedCapsulesCaseErrorReturn::CreateKem(error)),
        }
    }
}

pub fn run_malformed_capsules_case<C: IMalformedCapsulesCaseConsumer>(
    deps: &MalformedCapsulesCaseDeps<C>,
    params: MalformedCapsulesCaseParams,
    payload: MalformedCapsulesCasePayload,
) -> MalformedCapsulesCaseReturn<C::Output> {
    let _: RunMalformedCapsulesCaseFn<C> = run_malformed_capsules_case;
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
        Err(error) => Err(MalformedCapsulesCaseErrorReturn::CreatePairing(error)),
    }
}
