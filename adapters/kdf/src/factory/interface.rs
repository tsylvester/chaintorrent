use crate::blake3_keyed::provides::Blake3KeyedKdfDeriveKeyErrorReturn;
use core::convert::Infallible;
use domain::Secret;

pub const KDF_INTERFACE_VERSION: u32 = 1;

#[derive(PartialEq, Eq)]
pub enum KdfIdentifier {
    Blake3KeyedV1,
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

pub enum DeriveKeyErrorReturn {
    Blake3Keyed(Blake3KeyedKdfDeriveKeyErrorReturn),
}

pub type DeriveKeyReturn = Result<DeriveKeySuccessReturn, DeriveKeyErrorReturn>;

pub trait IKeyDerivationAdapter {
    fn derive_key(&self, params: DeriveKeyParams, payload: DeriveKeyPayload<'_>)
    -> DeriveKeyReturn;
}

pub enum KdfConcrete {
    Blake3Keyed,
}

pub struct CreateKeyDerivationDeps;

pub struct CreateKeyDerivationParams {
    pub concrete: KdfConcrete,
    pub identifier: KdfIdentifier,
}

pub struct CreateKeyDerivationPayload;

pub struct CreateKeyDerivationSuccessReturn {
    pub adapter: Box<dyn IKeyDerivationAdapter>,
    pub declaration: KdfDeclaration,
}

pub enum CreateKeyDerivationErrorReturn {
    UnsupportedKdfIdentifier,
    Blake3Keyed(Infallible),
}

pub type CreateKeyDerivationReturn =
    Result<CreateKeyDerivationSuccessReturn, CreateKeyDerivationErrorReturn>;

pub type CreateKeyDerivationFn = fn(
    &CreateKeyDerivationDeps,
    CreateKeyDerivationParams,
    CreateKeyDerivationPayload,
) -> CreateKeyDerivationReturn;
