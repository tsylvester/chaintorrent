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
    INonConvertibilityCaseConsumer, NonConvertibilityCaseDeps, NonConvertibilityCaseErrorReturn,
    NonConvertibilityCaseParams, NonConvertibilityCasePayload, NonConvertibilityCaseReturn,
    NonConvertibilityCaseSuccessReturn, NonConvertibilityDeps, NonConvertibilityFn,
    NonConvertibilityParams, NonConvertibilityPayload, NonConvertibilityReturn,
    NonConvertibilitySuccessReturn, NonConvertibilityVector, RunNonConvertibilityCaseFn,
};
use crate::delivery::provides::{
    AdmittedKemIdentity, AdmittedKemIdentityOverrides, build_admitted_kem_identity,
};
use crate::{VectorCount, build_vector_count};

#[derive(Default)]
pub struct NonConvertibilityVectorOverrides {
    pub draws: Option<Vec<u8>>,
    pub is_valid: Option<bool>,
    pub other_is_valid: Option<bool>,
}

pub fn build_non_convertibility_vector<K: ICredentialKemAdapter>(
    credential: K::Credential,
    overrides: NonConvertibilityVectorOverrides,
) -> NonConvertibilityVector<K> {
    NonConvertibilityVector {
        draws: overrides.draws.unwrap_or_else(|| vec![0x01; 64]),
        credential,
        is_valid: overrides.is_valid.unwrap_or(true),
        other_is_valid: overrides.other_is_valid.unwrap_or(false),
    }
}

#[derive(Default)]
pub struct NonConvertibilityParamsOverrides {
    pub count: Option<VectorCount>,
}

pub fn build_non_convertibility_params(
    overrides: NonConvertibilityParamsOverrides,
) -> NonConvertibilityParams {
    NonConvertibilityParams {
        count: overrides
            .count
            .unwrap_or_else(|| build_vector_count(Default::default())),
    }
}

fn non_convertibility_params_fields(params: &NonConvertibilityParams) -> Map<String, Value> {
    let mut fields = Map::new();
    fields.insert("count".to_string(), Value::from(params.count.get()));
    fields
}

#[derive(Default)]
pub struct NonConvertibilityParamsCorruptions {
    pub count: Option<Value>,
}

pub fn invalidate_non_convertibility_params(
    corruptions: NonConvertibilityParamsCorruptions,
) -> Value {
    let mut fields =
        non_convertibility_params_fields(&build_non_convertibility_params(Default::default()));
    if let Some(count) = corruptions.count {
        fields.insert("count".to_string(), count);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct NonConvertibilityPayloadOverrides<'a> {
    pub identity: Option<AdmittedKemIdentity<'a>>,
    pub other_identity: Option<AdmittedKemIdentity<'a>>,
}

pub fn build_non_convertibility_payload<'a>(
    overrides: NonConvertibilityPayloadOverrides<'a>,
) -> NonConvertibilityPayload<'a> {
    NonConvertibilityPayload {
        identity: overrides
            .identity
            .unwrap_or_else(|| build_admitted_kem_identity(Default::default())),
        other_identity: overrides.other_identity.unwrap_or_else(|| {
            build_admitted_kem_identity(AdmittedKemIdentityOverrides {
                setup_scope: None,
                identity: Some(KemIdentity::Entitlement {
                    canonical: b"pkg@2.0.0#7",
                }),
            })
        }),
    }
}

#[derive(Default)]
pub struct NonConvertibilityDepsOverrides;

pub fn build_non_convertibility_deps<'a, K: ICredentialKemAdapter>(
    kem: &'a K,
    random: &'a dyn IRandomSourceAdapter,
    _overrides: NonConvertibilityDepsOverrides,
) -> NonConvertibilityDeps<'a, K> {
    NonConvertibilityDeps { kem, random }
}

pub struct NonConvertibilitySuccessReturnOverrides<K: ICredentialKemAdapter> {
    pub vectors: Option<Vec<NonConvertibilityVector<K>>>,
}

impl<K: ICredentialKemAdapter> Default for NonConvertibilitySuccessReturnOverrides<K> {
    fn default() -> Self {
        Self { vectors: None }
    }
}

pub fn build_non_convertibility_success_return<K: ICredentialKemAdapter>(
    overrides: NonConvertibilitySuccessReturnOverrides<K>,
) -> NonConvertibilitySuccessReturn<K> {
    NonConvertibilitySuccessReturn {
        vectors: overrides.vectors.unwrap_or_default(),
    }
}

pub fn mock_non_convertibility<'a, K: ICredentialKemAdapter>(
    _deps: &NonConvertibilityDeps<'a, K>,
    _params: NonConvertibilityParams,
    _payload: NonConvertibilityPayload<'_>,
) -> NonConvertibilityReturn<K> {
    let _: NonConvertibilityFn<'a, K> = mock_non_convertibility;
    Ok(build_non_convertibility_success_return(Default::default()))
}

#[derive(Default)]
pub struct NonConvertibilityCaseParamsOverrides {
    pub pairing: Option<CreatePairingParams>,
    pub hash_to_scalar: Option<CreateHashToScalarParams>,
    pub kem: Option<CreateKemParams>,
    pub random_source: Option<CreateRandomSourceParams>,
}

pub fn build_non_convertibility_case_params(
    overrides: NonConvertibilityCaseParamsOverrides,
) -> NonConvertibilityCaseParams {
    NonConvertibilityCaseParams {
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
pub struct NonConvertibilityCaseDepsOverrides;

pub fn build_non_convertibility_case_deps<C: INonConvertibilityCaseConsumer>(
    consumer: C,
    _overrides: NonConvertibilityCaseDepsOverrides,
) -> NonConvertibilityCaseDeps<C> {
    NonConvertibilityCaseDeps { consumer }
}

#[derive(Default)]
pub struct NonConvertibilityCasePayloadOverrides {
    pub canonical: Option<Vec<u8>>,
    pub other_canonical: Option<Vec<u8>>,
    pub root: Option<Root>,
    pub other_root: Option<Root>,
}

pub fn build_non_convertibility_case_payload(
    overrides: NonConvertibilityCasePayloadOverrides,
) -> NonConvertibilityCasePayload {
    NonConvertibilityCasePayload {
        canonical: overrides
            .canonical
            .unwrap_or_else(|| b"pkg@1.0.0#1".to_vec()),
        other_canonical: overrides
            .other_canonical
            .unwrap_or_else(|| b"pkg@2.0.0#7".to_vec()),
        root: overrides.root.unwrap_or_else(|| {
            build_root(RootConstructorParamsOverrides {
                bytes: Some([0x44; 32]),
            })
        }),
        other_root: overrides.other_root.unwrap_or_else(|| {
            build_root(RootConstructorParamsOverrides {
                bytes: Some([0x45; 32]),
            })
        }),
    }
}

#[derive(Default)]
pub struct NonConvertibilityCaseSuccessReturnOverrides;

pub fn build_non_convertibility_case_success_return<O>(
    output: O,
    _overrides: NonConvertibilityCaseSuccessReturnOverrides,
) -> NonConvertibilityCaseSuccessReturn<O> {
    NonConvertibilityCaseSuccessReturn { output }
}

struct CaseAtKem<'a, P: IPairingArithmetic, C: INonConvertibilityCaseConsumer> {
    pairing: &'a P,
    hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
    random: &'a dyn IRandomSourceAdapter,
    independent: &'a dyn IRandomSourceAdapter,
    payload: &'a NonConvertibilityCasePayload,
    consumer: &'a C,
}

impl<'a, P, C> IKemConsumer<P> for CaseAtKem<'a, P, C>
where
    P: IPairingArithmetic + IPairingReference,
    C: INonConvertibilityCaseConsumer,
{
    type Output = NonConvertibilityCaseReturn<C::Output>;

    fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
        &self,
        _params: ConsumeKemParams,
        payload: ConsumeKemPayload<K>,
    ) -> Self::Output {
        let (identity, other_identity) = match payload.scope() {
            IdentityScope::Entitlement => (
                AdmittedKemIdentity {
                    setup_scope: SetupScope::Entitlement,
                    identity: KemIdentity::Entitlement {
                        canonical: &self.payload.canonical,
                    },
                },
                AdmittedKemIdentity {
                    setup_scope: SetupScope::Entitlement,
                    identity: KemIdentity::Entitlement {
                        canonical: &self.payload.other_canonical,
                    },
                },
            ),
            IdentityScope::Asset => (
                AdmittedKemIdentity {
                    setup_scope: SetupScope::Asset {
                        root: &self.payload.root,
                    },
                    identity: KemIdentity::Asset {
                        root: &self.payload.root,
                    },
                },
                AdmittedKemIdentity {
                    setup_scope: SetupScope::Asset {
                        root: &self.payload.other_root,
                    },
                    identity: KemIdentity::Asset {
                        root: &self.payload.other_root,
                    },
                },
            ),
        };
        let output = self.consumer.consume_non_convertibility_case(
            self.pairing,
            self.hashing,
            &payload.adapter,
            self.random,
            self.independent,
            identity,
            other_identity,
        );
        Ok(NonConvertibilityCaseSuccessReturn { output })
    }
}

struct CaseAtPairing<'a, C: INonConvertibilityCaseConsumer> {
    hash_to_scalar: &'a CreateHashToScalarParams,
    random_source: &'a CreateRandomSourceParams,
    kem: &'a CreateKemParams,
    payload: &'a NonConvertibilityCasePayload,
    consumer: &'a C,
}

impl<C: INonConvertibilityCaseConsumer> IPairingConsumer for CaseAtPairing<'_, C> {
    type Output = NonConvertibilityCaseReturn<C::Output>;

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
                return Err(NonConvertibilityCaseErrorReturn::CreateHashToScalar(error));
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
                return Err(NonConvertibilityCaseErrorReturn::CreateRandomSource(error));
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
                return Err(NonConvertibilityCaseErrorReturn::CreateRandomSource(error));
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
            Err(error) => Err(NonConvertibilityCaseErrorReturn::CreateKem(error)),
        }
    }
}

pub fn run_non_convertibility_case<C: INonConvertibilityCaseConsumer>(
    deps: &NonConvertibilityCaseDeps<C>,
    params: NonConvertibilityCaseParams,
    payload: NonConvertibilityCasePayload,
) -> NonConvertibilityCaseReturn<C::Output> {
    let _: RunNonConvertibilityCaseFn<C> = run_non_convertibility_case;
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
        Err(error) => Err(NonConvertibilityCaseErrorReturn::CreatePairing(error)),
    }
}
