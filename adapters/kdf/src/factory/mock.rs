#![allow(dead_code, clippy::expect_used, clippy::too_many_arguments)]

use super::interface::{
    CreateKeyDerivationDeps, CreateKeyDerivationParams, CreateKeyDerivationPayload,
    CreateKeyDerivationReturn, CreateKeyDerivationSuccessReturn, DerivationPurpose,
    DeriveKeyErrorReturn, DeriveKeyParams, DeriveKeyPayload, DeriveKeyReturn,
    DeriveKeySuccessReturn, IKeyDerivationAdapter, KDF_INTERFACE_VERSION, KdfConcrete,
    KdfDeclaration, KdfIdentifier, MockIKeyDerivationAdapterFailureMode,
};
use core::convert::Infallible;
use domain::{Secret, SecretConstructorParamsOverrides, build_secret};
use serde_json::{Map, Value};
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::sync::OnceLock;

pub fn reference_key_material_bytes() -> Vec<u8> {
    (0u8..32).collect()
}

pub fn reference_key_material() -> &'static Secret<Vec<u8>> {
    static REFERENCE_KEY_MATERIAL: OnceLock<Secret<Vec<u8>>> = OnceLock::new();
    REFERENCE_KEY_MATERIAL.get_or_init(|| {
        build_secret(
            reference_key_material_bytes(),
            SecretConstructorParamsOverrides,
        )
    })
}

pub fn reference_context() -> &'static [u8] {
    static REFERENCE_CONTEXT: OnceLock<Vec<u8>> = OnceLock::new();
    REFERENCE_CONTEXT.get_or_init(|| (128u8..176).collect())
}

pub const REFERENCE_LENGTH: usize = 64;

pub const REFERENCE_PREFIX_LENGTH: usize = 32;

#[derive(Default)]
pub struct KdfDeclarationOverrides {
    pub identifier: Option<KdfIdentifier>,
    pub adapter_version: Option<u32>,
    pub interface_version: Option<u32>,
}

pub fn build_kdf_declaration(overrides: KdfDeclarationOverrides) -> KdfDeclaration {
    KdfDeclaration {
        identifier: overrides.identifier.unwrap_or(KdfIdentifier::MockV1),
        adapter_version: overrides
            .adapter_version
            .unwrap_or(MockIKeyDerivationAdapter::DECLARATION.adapter_version),
        interface_version: overrides.interface_version.unwrap_or(KDF_INTERFACE_VERSION),
    }
}

#[derive(Default)]
pub struct KdfDeclarationCorruptions {
    pub identifier: Option<Value>,
    pub adapter_version: Option<Value>,
    pub interface_version: Option<Value>,
}

pub fn invalidate_kdf_declaration(corruptions: KdfDeclarationCorruptions) -> Value {
    let declaration = build_kdf_declaration(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "identifier".to_string(),
        Value::from(match declaration.identifier {
            KdfIdentifier::Blake3KeyedV1 => "Blake3KeyedV1",
            KdfIdentifier::MockV1 => "MockV1",
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

#[derive(Default)]
pub struct DeriveKeyParamsOverrides {
    pub purpose: Option<DerivationPurpose>,
    pub length: Option<usize>,
}

pub fn build_derive_key_params(overrides: DeriveKeyParamsOverrides) -> DeriveKeyParams {
    DeriveKeyParams {
        purpose: overrides.purpose.unwrap_or(DerivationPurpose::WrappingKey),
        length: overrides.length.unwrap_or(REFERENCE_LENGTH),
    }
}

#[derive(Default)]
pub struct DeriveKeyParamsCorruptions {
    pub purpose: Option<Value>,
    pub length: Option<Value>,
}

pub fn invalidate_derive_key_params(corruptions: DeriveKeyParamsCorruptions) -> Value {
    let params = build_derive_key_params(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "purpose".to_string(),
        Value::from(match params.purpose {
            DerivationPurpose::WrappingKey => "WrappingKey",
            DerivationPurpose::PublisherRoot => "PublisherRoot",
            DerivationPurpose::AssetRoot => "AssetRoot",
            DerivationPurpose::MasterScalar => "MasterScalar",
            DerivationPurpose::IdentityBases => "IdentityBases",
            DerivationPurpose::CapsuleRandomness => "CapsuleRandomness",
            DerivationPurpose::PieceGroupKey => "PieceGroupKey",
            DerivationPurpose::PlaintextRootKey => "PlaintextRootKey",
        }),
    );
    fields.insert("length".to_string(), Value::from(params.length));
    if let Some(purpose) = corruptions.purpose {
        fields.insert("purpose".to_string(), purpose);
    }
    if let Some(length) = corruptions.length {
        fields.insert("length".to_string(), length);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct DeriveKeyPayloadOverrides<'a> {
    pub key_material: Option<&'a Secret<Vec<u8>>>,
    pub context: Option<&'a [u8]>,
}

pub fn build_derive_key_payload<'a>(
    overrides: DeriveKeyPayloadOverrides<'a>,
) -> DeriveKeyPayload<'a> {
    DeriveKeyPayload {
        key_material: overrides
            .key_material
            .unwrap_or_else(|| reference_key_material()),
        context: overrides.context.unwrap_or_else(|| reference_context()),
    }
}

#[derive(Default)]
pub struct DeriveKeyPayloadCorruptions {
    pub key_material: Option<Value>,
    pub context: Option<Value>,
}

pub fn invalidate_derive_key_payload(corruptions: DeriveKeyPayloadCorruptions) -> Value {
    let payload = build_derive_key_payload(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "key_material".to_string(),
        Value::Array(
            payload
                .key_material
                .expose()
                .iter()
                .map(|byte| Value::from(*byte))
                .collect(),
        ),
    );
    fields.insert(
        "context".to_string(),
        Value::Array(
            payload
                .context
                .iter()
                .map(|byte| Value::from(*byte))
                .collect(),
        ),
    );
    if let Some(key_material) = corruptions.key_material {
        fields.insert("key_material".to_string(), key_material);
    }
    if let Some(context) = corruptions.context {
        fields.insert("context".to_string(), context);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct DeriveKeySuccessReturnOverrides {
    pub key: Option<Secret<Vec<u8>>>,
}

pub fn build_derive_key_success_return(
    overrides: DeriveKeySuccessReturnOverrides,
) -> DeriveKeySuccessReturn {
    DeriveKeySuccessReturn {
        key: overrides
            .key
            .unwrap_or_else(|| build_secret(Vec::new(), SecretConstructorParamsOverrides)),
    }
}

#[derive(Default)]
pub struct DeriveKeySuccessReturnCorruptions {
    pub key: Option<Value>,
}

pub fn invalidate_derive_key_success_return(
    corruptions: DeriveKeySuccessReturnCorruptions,
) -> Value {
    let success = build_derive_key_success_return(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "key".to_string(),
        Value::Array(
            success
                .key
                .expose()
                .iter()
                .map(|byte| Value::from(*byte))
                .collect(),
        ),
    );
    if let Some(key) = corruptions.key {
        fields.insert("key".to_string(), key);
    }
    Value::Object(fields)
}

#[derive(Debug, PartialEq, Eq)]
pub enum MockIKeyDerivationAdapterDeriveKeyErrorReturn {
    KeyMaterialLengthUnrepresentable { length: usize },
}

pub(crate) struct MockIKeyDerivationAdapterConstructorParams {
    pub failure_mode: MockIKeyDerivationAdapterFailureMode,
}

pub(crate) type MockIKeyDerivationAdapterTryNewReturn =
    Result<MockIKeyDerivationAdapter, Infallible>;

pub(crate) struct MockIKeyDerivationAdapter {
    failure_mode: MockIKeyDerivationAdapterFailureMode,
}

impl MockIKeyDerivationAdapter {
    pub const DECLARATION: KdfDeclaration = KdfDeclaration {
        identifier: KdfIdentifier::MockV1,
        adapter_version: 1,
        interface_version: KDF_INTERFACE_VERSION,
    };

    pub fn try_new(
        params: MockIKeyDerivationAdapterConstructorParams,
    ) -> MockIKeyDerivationAdapterTryNewReturn {
        Ok(MockIKeyDerivationAdapter {
            failure_mode: params.failure_mode,
        })
    }
}

impl IKeyDerivationAdapter for MockIKeyDerivationAdapter {
    fn declaration(&self) -> KdfDeclaration {
        Self::DECLARATION
    }

    fn derive_key(
        &self,
        params: DeriveKeyParams,
        payload: DeriveKeyPayload<'_>,
    ) -> DeriveKeyReturn {
        let material = payload.key_material.expose();
        match &self.failure_mode {
            MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable => {
                Err(DeriveKeyErrorReturn::Mock(
                    MockIKeyDerivationAdapterDeriveKeyErrorReturn::KeyMaterialLengthUnrepresentable {
                        length: material.len(),
                    },
                ))
            }
            MockIKeyDerivationAdapterFailureMode::NoFailure => {
                let prefix = u64::try_from(material.len())
                    .expect("a usize fits a u64 on every supported target");
                let purpose_tag = match params.purpose {
                    DerivationPurpose::WrappingKey => 0u8,
                    DerivationPurpose::PublisherRoot => 1u8,
                    DerivationPurpose::AssetRoot => 2u8,
                    DerivationPurpose::MasterScalar => 3u8,
                    DerivationPurpose::IdentityBases => 4u8,
                    DerivationPurpose::CapsuleRandomness => 5u8,
                    DerivationPurpose::PieceGroupKey => 6u8,
                    DerivationPurpose::PlaintextRootKey => 7u8,
                };
                let mut bytes = Vec::with_capacity(params.length);
                for counter in 0..params.length {
                    let mut hasher = DefaultHasher::new();
                    hasher.write_u8(purpose_tag);
                    hasher.write(&prefix.to_be_bytes());
                    hasher.write(material);
                    hasher.write(payload.context);
                    hasher.write_usize(counter);
                    bytes.push(hasher.finish().to_be_bytes()[0]);
                }
                Ok(build_derive_key_success_return(
                    DeriveKeySuccessReturnOverrides {
                        key: Some(build_secret(bytes, SecretConstructorParamsOverrides)),
                    },
                ))
            }
        }
    }
}

#[derive(Default)]
pub struct CreateKeyDerivationDepsOverrides;

pub fn build_create_key_derivation_deps(
    _overrides: CreateKeyDerivationDepsOverrides,
) -> CreateKeyDerivationDeps {
    CreateKeyDerivationDeps
}

#[derive(Default)]
pub struct CreateKeyDerivationParamsOverrides {
    pub concrete: Option<KdfConcrete>,
    pub identifier: Option<KdfIdentifier>,
}

pub fn build_create_key_derivation_params(
    overrides: CreateKeyDerivationParamsOverrides,
) -> CreateKeyDerivationParams {
    CreateKeyDerivationParams {
        concrete: overrides.concrete.unwrap_or(KdfConcrete::Mock(
            MockIKeyDerivationAdapterFailureMode::NoFailure,
        )),
        identifier: overrides.identifier.unwrap_or(KdfIdentifier::MockV1),
    }
}

#[derive(Default)]
pub struct CreateKeyDerivationParamsCorruptions {
    pub concrete: Option<Value>,
    pub identifier: Option<Value>,
}

pub fn invalidate_create_key_derivation_params(
    corruptions: CreateKeyDerivationParamsCorruptions,
) -> Value {
    let params = build_create_key_derivation_params(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "concrete".to_string(),
        match params.concrete {
            KdfConcrete::Blake3Keyed => Value::from("Blake3Keyed"),
            KdfConcrete::Mock(failure_mode) => {
                let mut entry = Map::new();
                entry.insert(
                    "Mock".to_string(),
                    Value::from(match failure_mode {
                        MockIKeyDerivationAdapterFailureMode::NoFailure => "NoFailure",
                        MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable => {
                            "KeyMaterialLengthUnrepresentable"
                        }
                    }),
                );
                Value::Object(entry)
            }
        },
    );
    fields.insert(
        "identifier".to_string(),
        Value::from(match params.identifier {
            KdfIdentifier::Blake3KeyedV1 => "Blake3KeyedV1",
            KdfIdentifier::MockV1 => "MockV1",
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
pub struct CreateKeyDerivationPayloadOverrides;

pub fn build_create_key_derivation_payload(
    _overrides: CreateKeyDerivationPayloadOverrides,
) -> CreateKeyDerivationPayload {
    CreateKeyDerivationPayload
}

#[derive(Default)]
pub struct CreateKeyDerivationSuccessReturnOverrides;

pub fn build_create_key_derivation_success_return(
    adapter: Box<dyn IKeyDerivationAdapter>,
    _overrides: CreateKeyDerivationSuccessReturnOverrides,
) -> CreateKeyDerivationSuccessReturn {
    CreateKeyDerivationSuccessReturn { adapter }
}

pub fn mock_create_key_derivation(
    _deps: &CreateKeyDerivationDeps,
    _params: CreateKeyDerivationParams,
    _payload: CreateKeyDerivationPayload,
) -> CreateKeyDerivationReturn {
    let Ok(adapter) =
        MockIKeyDerivationAdapter::try_new(MockIKeyDerivationAdapterConstructorParams {
            failure_mode: MockIKeyDerivationAdapterFailureMode::NoFailure,
        });
    Ok(build_create_key_derivation_success_return(
        Box::new(adapter),
        Default::default(),
    ))
}
