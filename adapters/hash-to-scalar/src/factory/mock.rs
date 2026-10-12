#![allow(clippy::expect_used)]

use super::interface::{
    CreateHashToScalarDeps, CreateHashToScalarParams, CreateHashToScalarPayload,
    CreateHashToScalarReturn, CreateHashToScalarSuccessReturn, HASH_TO_SCALAR_INTERFACE_VERSION,
    HashToScalarConcrete, HashToScalarDeclaration, HashToScalarErrorReturn, HashToScalarIdentifier,
    HashToScalarParams, HashToScalarPayload, HashToScalarReturn, HashToScalarSuccessReturn,
    IHashToScalarAdapter, MockIHashToScalarAdapterFailureMode,
};
use crate::domain_tag::provides::{DomainTag, build_domain_tag};
use core::convert::Infallible;
use core::iter::once;
use core::marker::PhantomData;
use domain::{Secret, SecretConstructorParams, build_secret};
use pairing::{ISampleUniformScalar, SampleUniformScalarParams, SampleUniformScalarPayload};
use serde_json::{Map, Value};
use std::sync::OnceLock;

#[derive(Default)]
pub struct HashToScalarDeclarationOverrides {
    pub identifier: Option<HashToScalarIdentifier>,
    pub adapter_version: Option<u32>,
    pub interface_version: Option<u32>,
}

pub fn build_hash_to_scalar_declaration(
    overrides: HashToScalarDeclarationOverrides,
) -> HashToScalarDeclaration {
    HashToScalarDeclaration {
        identifier: overrides
            .identifier
            .unwrap_or(HashToScalarIdentifier::Keccak256V1),
        adapter_version: overrides.adapter_version.unwrap_or(1),
        interface_version: overrides
            .interface_version
            .unwrap_or(HASH_TO_SCALAR_INTERFACE_VERSION),
    }
}

#[derive(Default)]
pub struct HashToScalarDeclarationCorruptions {
    pub identifier: Option<Value>,
    pub adapter_version: Option<Value>,
    pub interface_version: Option<Value>,
}

pub fn invalidate_hash_to_scalar_declaration(
    corruptions: HashToScalarDeclarationCorruptions,
) -> Value {
    let declaration = build_hash_to_scalar_declaration(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "identifier".to_string(),
        Value::from(match declaration.identifier {
            HashToScalarIdentifier::Keccak256V1 => "Keccak256V1",
            #[cfg(any(test, feature = "mocks"))]
            HashToScalarIdentifier::Mock => "Mock",
        }),
    );
    fields.insert(
        "adapter_version".to_string(),
        Value::from(declaration.adapter_version),
    );
    fields.insert(
        "interface_version".to_string(),
        Value::from(declaration.interface_version),
    );
    if let Some(identifier) = corruptions.identifier {
        fields.insert("identifier".to_string(), identifier);
    }
    if let Some(adapter_version) = corruptions.adapter_version {
        fields.insert("adapter_version".to_string(), adapter_version);
    }
    if let Some(interface_version) = corruptions.interface_version {
        fields.insert("interface_version".to_string(), interface_version);
    }
    Value::Object(fields)
}

static DEFAULT_HASH_TO_SCALAR_TAG: OnceLock<DomainTag> = OnceLock::new();

#[derive(Default)]
pub struct HashToScalarParamsOverrides<'a> {
    pub tag: Option<&'a DomainTag>,
}

pub fn build_hash_to_scalar_params<'a>(
    overrides: HashToScalarParamsOverrides<'a>,
) -> HashToScalarParams<'a> {
    HashToScalarParams {
        tag: overrides.tag.unwrap_or_else(|| {
            DEFAULT_HASH_TO_SCALAR_TAG.get_or_init(|| build_domain_tag(Default::default()))
        }),
    }
}

#[derive(Default)]
pub struct HashToScalarParamsCorruptions {
    pub tag: Option<Value>,
}

pub fn invalidate_hash_to_scalar_params(corruptions: HashToScalarParamsCorruptions) -> Value {
    let params = build_hash_to_scalar_params(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "tag".to_string(),
        Value::from(params.tag.as_bytes().to_vec()),
    );
    if let Some(tag) = corruptions.tag {
        fields.insert("tag".to_string(), tag);
    }
    Value::Object(fields)
}

pub const HASH_TO_SCALAR_DEFAULT_MESSAGE: &[u8] = b"hash to scalar default message";

#[derive(Default)]
pub struct HashToScalarPayloadOverrides<'a> {
    pub message: Option<&'a [u8]>,
}

pub fn build_hash_to_scalar_payload<'a>(
    overrides: HashToScalarPayloadOverrides<'a>,
) -> HashToScalarPayload<'a> {
    HashToScalarPayload {
        message: overrides.message.unwrap_or(HASH_TO_SCALAR_DEFAULT_MESSAGE),
    }
}

#[derive(Default)]
pub struct HashToScalarPayloadCorruptions {
    pub message: Option<Value>,
}

pub fn invalidate_hash_to_scalar_payload(corruptions: HashToScalarPayloadCorruptions) -> Value {
    let payload = build_hash_to_scalar_payload(Default::default());
    let mut fields = Map::new();
    fields.insert("message".to_string(), Value::from(payload.message.to_vec()));
    if let Some(message) = corruptions.message {
        fields.insert("message".to_string(), message);
    }
    Value::Object(fields)
}

pub struct HashToScalarSuccessReturnOverrides<S> {
    pub scalar: Option<S>,
}

impl<S> Default for HashToScalarSuccessReturnOverrides<S> {
    fn default() -> Self {
        Self { scalar: None }
    }
}

pub fn build_hash_to_scalar_success_return<S: ISampleUniformScalar + Clone>(
    overrides: HashToScalarSuccessReturnOverrides<S>,
) -> HashToScalarSuccessReturn<S> {
    HashToScalarSuccessReturn {
        scalar: overrides.scalar.unwrap_or_else(|| {
            S::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload {
                    uniform: build_secret(vec![1u8; S::UNIFORM_BYTES_LENGTH], Default::default()),
                },
            )
            .expect("a uniform input of UNIFORM_BYTES_LENGTH samples a scalar")
            .scalar
            .expose()
            .clone()
        }),
    }
}

#[derive(Default)]
pub struct HashToScalarSuccessReturnCorruptions {
    pub scalar: Option<Value>,
}

pub fn invalidate_hash_to_scalar_success_return(
    corruptions: HashToScalarSuccessReturnCorruptions,
) -> Value {
    let mut fields = Map::new();
    if let Some(scalar) = corruptions.scalar {
        fields.insert("scalar".to_string(), scalar);
    }
    Value::Object(fields)
}

pub(crate) struct MockIHashToScalarAdapter<S> {
    failure_mode: MockIHashToScalarAdapterFailureMode,
    pub scalar: PhantomData<S>,
}

pub(crate) struct MockIHashToScalarAdapterConstructorParams {
    pub(crate) failure_mode: MockIHashToScalarAdapterFailureMode,
}

pub(crate) type MockIHashToScalarAdapterTryNewReturn<S> =
    Result<MockIHashToScalarAdapter<S>, Infallible>;

impl<S> MockIHashToScalarAdapter<S> {
    pub(crate) const DECLARATION: HashToScalarDeclaration = HashToScalarDeclaration {
        identifier: HashToScalarIdentifier::Mock,
        adapter_version: 1,
        interface_version: HASH_TO_SCALAR_INTERFACE_VERSION,
    };

    pub(crate) fn try_new(
        params: MockIHashToScalarAdapterConstructorParams,
    ) -> MockIHashToScalarAdapterTryNewReturn<S> {
        Ok(MockIHashToScalarAdapter {
            failure_mode: params.failure_mode,
            scalar: PhantomData,
        })
    }
}

impl<S: ISampleUniformScalar + Clone> IHashToScalarAdapter<S> for MockIHashToScalarAdapter<S> {
    fn declaration(&self) -> HashToScalarDeclaration {
        Self::DECLARATION
    }

    fn hash_to_scalar(
        &self,
        params: HashToScalarParams<'_>,
        payload: HashToScalarPayload<'_>,
    ) -> HashToScalarReturn<S> {
        let prefix = u8::try_from(params.tag.as_bytes().len()).unwrap_or(u8::MAX);
        let mut buffer = vec![0u8; S::UNIFORM_BYTES_LENGTH];
        for (index, byte) in once(prefix)
            .chain(params.tag.as_bytes().iter().copied())
            .chain(payload.message.iter().copied())
            .enumerate()
        {
            if let Some(slot) = index
                .checked_rem(buffer.len())
                .and_then(|slot| buffer.get_mut(slot))
            {
                *slot ^= byte;
            }
        }
        if self.failure_mode == MockIHashToScalarAdapterFailureMode::SamplingRefused {
            buffer = Vec::new();
        }
        let Ok(uniform) = Secret::try_new(SecretConstructorParams { value: buffer });
        match S::sample_from_uniform_bytes(
            SampleUniformScalarParams,
            SampleUniformScalarPayload { uniform },
        ) {
            Ok(success) => Ok(HashToScalarSuccessReturn {
                scalar: success.scalar.expose().clone(),
            }),
            Err(error) => Err(HashToScalarErrorReturn::Sampling(error)),
        }
    }
}

#[derive(Default)]
pub struct CreateHashToScalarDepsOverrides;

pub fn build_create_hash_to_scalar_deps(
    _overrides: CreateHashToScalarDepsOverrides,
) -> CreateHashToScalarDeps {
    CreateHashToScalarDeps
}

#[derive(Default)]
pub struct CreateHashToScalarDepsCorruptions;

pub fn invalidate_create_hash_to_scalar_deps(
    _corruptions: CreateHashToScalarDepsCorruptions,
) -> Value {
    Value::Object(Map::new())
}

#[derive(Default)]
pub struct CreateHashToScalarParamsOverrides {
    pub concrete: Option<HashToScalarConcrete>,
    pub identifier: Option<HashToScalarIdentifier>,
}

pub fn build_create_hash_to_scalar_params(
    overrides: CreateHashToScalarParamsOverrides,
) -> CreateHashToScalarParams {
    CreateHashToScalarParams {
        concrete: overrides.concrete.unwrap_or(HashToScalarConcrete::Mock(
            MockIHashToScalarAdapterFailureMode::Succeeds,
        )),
        identifier: overrides.identifier.unwrap_or(HashToScalarIdentifier::Mock),
    }
}

#[derive(Default)]
pub struct CreateHashToScalarParamsCorruptions {
    pub concrete: Option<Value>,
    pub identifier: Option<Value>,
}

pub fn invalidate_create_hash_to_scalar_params(
    corruptions: CreateHashToScalarParamsCorruptions,
) -> Value {
    let params = build_create_hash_to_scalar_params(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "concrete".to_string(),
        Value::from(match params.concrete {
            HashToScalarConcrete::Keccak256 => "Keccak256",
            #[cfg(any(test, feature = "mocks"))]
            HashToScalarConcrete::Mock(_) => "Mock",
        }),
    );
    fields.insert(
        "identifier".to_string(),
        Value::from(match params.identifier {
            HashToScalarIdentifier::Keccak256V1 => "Keccak256V1",
            #[cfg(any(test, feature = "mocks"))]
            HashToScalarIdentifier::Mock => "Mock",
        }),
    );
    if let Some(concrete) = corruptions.concrete {
        fields.insert("concrete".to_string(), concrete);
    }
    if let Some(identifier) = corruptions.identifier {
        fields.insert("identifier".to_string(), identifier);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct CreateHashToScalarPayloadOverrides;

pub fn build_create_hash_to_scalar_payload(
    _overrides: CreateHashToScalarPayloadOverrides,
) -> CreateHashToScalarPayload {
    CreateHashToScalarPayload
}

#[derive(Default)]
pub struct CreateHashToScalarPayloadCorruptions;

pub fn invalidate_create_hash_to_scalar_payload(
    _corruptions: CreateHashToScalarPayloadCorruptions,
) -> Value {
    Value::Object(Map::new())
}

#[derive(Default)]
pub struct CreateHashToScalarSuccessReturnOverrides;

pub fn build_create_hash_to_scalar_success_return<S: ISampleUniformScalar + Clone>(
    adapter: Box<dyn IHashToScalarAdapter<S>>,
    _overrides: CreateHashToScalarSuccessReturnOverrides,
) -> CreateHashToScalarSuccessReturn<S> {
    CreateHashToScalarSuccessReturn { adapter }
}

pub fn mock_create_hash_to_scalar<S: ISampleUniformScalar + Clone>(
    _deps: &CreateHashToScalarDeps,
    _params: CreateHashToScalarParams,
    _payload: CreateHashToScalarPayload,
) -> CreateHashToScalarReturn<S> {
    let Ok(adapter) =
        MockIHashToScalarAdapter::<S>::try_new(MockIHashToScalarAdapterConstructorParams {
            failure_mode: MockIHashToScalarAdapterFailureMode::Succeeds,
        });
    Ok(build_create_hash_to_scalar_success_return(
        Box::new(adapter),
        Default::default(),
    ))
}
