#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{
    CreateKeyDerivationDeps, CreateKeyDerivationParams, CreateKeyDerivationPayload,
    CreateKeyDerivationReturn, CreateKeyDerivationSuccessReturn, DerivationPurpose,
    DeriveKeyParams, DeriveKeyPayload, DeriveKeyReturn, DeriveKeySuccessReturn,
    IKeyDerivationAdapter, KDF_INTERFACE_VERSION, KdfConcrete, KdfDeclaration, KdfIdentifier,
};
use domain::{Secret, SecretConstructorParamsOverrides, build_secret};

#[derive(Default)]
pub struct KdfDeclarationOverrides {
    pub identifier: Option<KdfIdentifier>,
    pub adapter_version: Option<u32>,
    pub interface_version: Option<u32>,
}

pub fn build_kdf_declaration(overrides: KdfDeclarationOverrides) -> KdfDeclaration {
    KdfDeclaration {
        identifier: overrides.identifier.unwrap_or(KdfIdentifier::Blake3KeyedV1),
        adapter_version: overrides.adapter_version.unwrap_or(1),
        interface_version: overrides.interface_version.unwrap_or(KDF_INTERFACE_VERSION),
    }
}

#[derive(Default)]
pub struct DeriveKeyParamsOverrides {
    pub purpose: Option<DerivationPurpose>,
    pub length: Option<usize>,
}

pub fn build_derive_key_params(overrides: DeriveKeyParamsOverrides) -> DeriveKeyParams {
    DeriveKeyParams {
        purpose: overrides.purpose.unwrap_or(DerivationPurpose::WrappingKey),
        length: overrides.length.unwrap_or(32),
    }
}

#[derive(Default)]
pub struct DeriveKeySuccessReturnOverrides {
    pub key: Option<Secret<Vec<u8>>>,
}

pub fn build_derive_key_success_return(
    overrides: DeriveKeySuccessReturnOverrides,
) -> DeriveKeySuccessReturn {
    DeriveKeySuccessReturn {
        key: overrides.key.unwrap_or_else(|| {
            build_secret::<Vec<u8>>(SecretConstructorParamsOverrides::default())
        }),
    }
}

pub struct MockIKeyDerivationAdapter;

impl IKeyDerivationAdapter for MockIKeyDerivationAdapter {
    fn declaration(&self) -> KdfDeclaration {
        build_kdf_declaration(Default::default())
    }

    fn derive_key(
        &self,
        _params: DeriveKeyParams,
        _payload: DeriveKeyPayload<'_>,
    ) -> DeriveKeyReturn {
        Ok(build_derive_key_success_return(Default::default()))
    }
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
        concrete: overrides.concrete.unwrap_or(KdfConcrete::Blake3Keyed),
        identifier: overrides.identifier.unwrap_or(KdfIdentifier::Blake3KeyedV1),
    }
}

#[derive(Default)]
pub struct CreateKeyDerivationSuccessReturnOverrides {
    pub adapter: Option<Box<dyn IKeyDerivationAdapter>>,
}

pub fn build_create_key_derivation_success_return(
    overrides: CreateKeyDerivationSuccessReturnOverrides,
) -> CreateKeyDerivationSuccessReturn {
    CreateKeyDerivationSuccessReturn {
        adapter: overrides
            .adapter
            .unwrap_or_else(|| Box::new(MockIKeyDerivationAdapter)),
    }
}

pub fn mock_create_key_derivation(
    _deps: &CreateKeyDerivationDeps,
    _params: CreateKeyDerivationParams,
    _payload: CreateKeyDerivationPayload,
) -> CreateKeyDerivationReturn {
    Ok(build_create_key_derivation_success_return(
        Default::default(),
    ))
}
