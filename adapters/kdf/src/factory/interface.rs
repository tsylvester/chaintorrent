#[cfg(any(test, feature = "mocks"))]
use super::mock::MockIKeyDerivationAdapterDeriveKeyErrorReturn;
use crate::blake3_keyed::provides::Blake3KeyedKdfDeriveKeyErrorReturn;
use core::convert::Infallible;
use domain::Secret;

pub const KDF_INTERFACE_VERSION: u32 = 1;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum KdfIdentifier {
    Blake3KeyedV1,
    #[cfg(any(test, feature = "mocks"))]
    MockV1,
}

pub struct KdfDeclaration {
    pub identifier: KdfIdentifier,
    pub adapter_version: u32,
    pub interface_version: u32,
}

pub enum DerivationPurpose {
    WrappingKey,
    PublisherRoot,
    AssetRoot,
    MasterScalar,
    IdentityBases,
    CapsuleRandomness,
    PieceGroupKey,
    PlaintextRootKey,
}

pub struct DeriveKeyParams {
    pub purpose: DerivationPurpose,
    pub length: usize,
}

pub struct DeriveKeyPayload<'a> {
    pub key_material: &'a Secret<Vec<u8>>,
    pub context: &'a [u8],
}

pub struct DeriveKeySuccessReturn {
    pub key: Secret<Vec<u8>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DeriveKeyErrorReturn {
    Blake3Keyed(Blake3KeyedKdfDeriveKeyErrorReturn),
    #[cfg(any(test, feature = "mocks"))]
    Mock(MockIKeyDerivationAdapterDeriveKeyErrorReturn),
}

pub type DeriveKeyReturn = Result<DeriveKeySuccessReturn, DeriveKeyErrorReturn>;

#[cfg(any(test, feature = "mocks"))]
#[derive(Clone, Copy)]
pub enum MockIKeyDerivationAdapterFailureMode {
    NoFailure,
    KeyMaterialLengthUnrepresentable,
}

pub trait IKeyDerivationAdapter {
    fn declaration(&self) -> KdfDeclaration;
    fn derive_key(&self, params: DeriveKeyParams, payload: DeriveKeyPayload<'_>)
    -> DeriveKeyReturn;
}

#[derive(Clone, Copy)]
pub enum KdfConcrete {
    Blake3Keyed,
    #[cfg(any(test, feature = "mocks"))]
    Mock(MockIKeyDerivationAdapterFailureMode),
}

pub struct CreateKeyDerivationDeps;

#[derive(Clone, Copy)]
pub struct CreateKeyDerivationParams {
    pub concrete: KdfConcrete,
    pub identifier: KdfIdentifier,
}

pub struct CreateKeyDerivationPayload;

pub struct CreateKeyDerivationSuccessReturn {
    pub adapter: Box<dyn IKeyDerivationAdapter>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CreateKeyDerivationErrorReturn {
    UnsupportedKdfIdentifier,
    Blake3Keyed(Infallible),
    #[cfg(any(test, feature = "mocks"))]
    Mock(Infallible),
}

pub type CreateKeyDerivationReturn =
    Result<CreateKeyDerivationSuccessReturn, CreateKeyDerivationErrorReturn>;

pub type CreateKeyDerivationFn = fn(
    &CreateKeyDerivationDeps,
    CreateKeyDerivationParams,
    CreateKeyDerivationPayload,
) -> CreateKeyDerivationReturn;

pub const DECLARED_KDF_SELECTIONS: &[CreateKeyDerivationParams] = &[CreateKeyDerivationParams {
    concrete: KdfConcrete::Blake3Keyed,
    identifier: KdfIdentifier::Blake3KeyedV1,
}];
