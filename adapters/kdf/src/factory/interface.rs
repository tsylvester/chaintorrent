#[cfg(any(test, feature = "mocks"))]
use super::mock::MockIKeyDerivationAdapterDeriveKeyErrorReturn;
use crate::blake3_keyed::provides::Blake3KeyedKdfDeriveKeyErrorReturn;
use crate::lookup_kdf_declaration::provides::LookupKdfDeclarationFn;
use core::convert::Infallible;
use domain::Secret;

pub const KDF_INTERFACE_VERSION: u32 = 1;

pub enum KdfIdentifier {
    Blake3KeyedV1,
}

impl KdfIdentifier {
    pub const fn name(self) -> &'static str {
        match self {
            KdfIdentifier::Blake3KeyedV1 => "blake3_keyed_v1",
        }
    }

    pub const fn wire_code(self) -> u8 {
        match self {
            KdfIdentifier::Blake3KeyedV1 => 1,
        }
    }
}

pub const KDF_IDENTIFIERS: &[KdfIdentifier] = &[KdfIdentifier::Blake3KeyedV1];

pub struct KdfDeclaration {
    pub concrete: KdfConcrete,
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

impl DerivationPurpose {
    pub const fn name(self) -> &'static str {
        match self {
            DerivationPurpose::WrappingKey => "wrapping_key",
            DerivationPurpose::PublisherRoot => "publisher_root",
            DerivationPurpose::AssetRoot => "asset_root",
            DerivationPurpose::MasterScalar => "master_scalar",
            DerivationPurpose::IdentityBases => "identity_bases",
            DerivationPurpose::CapsuleRandomness => "capsule_randomness",
            DerivationPurpose::PieceGroupKey => "piece_group_key",
            DerivationPurpose::PlaintextRootKey => "plaintext_root_key",
        }
    }
}

pub const DERIVATION_PURPOSES: &[DerivationPurpose] = &[
    DerivationPurpose::WrappingKey,
    DerivationPurpose::PublisherRoot,
    DerivationPurpose::AssetRoot,
    DerivationPurpose::MasterScalar,
    DerivationPurpose::IdentityBases,
    DerivationPurpose::CapsuleRandomness,
    DerivationPurpose::PieceGroupKey,
    DerivationPurpose::PlaintextRootKey,
];

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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MockIKeyDerivationAdapterFailureMode {
    NoFailure,
    KeyMaterialLengthUnrepresentable,
}

pub trait IKeyDerivationAdapter {
    fn derive_key(&self, params: DeriveKeyParams, payload: DeriveKeyPayload<'_>)
    -> DeriveKeyReturn;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KdfConcrete {
    Blake3Keyed,
    #[cfg(any(test, feature = "mocks"))]
    Mock(MockIKeyDerivationAdapterFailureMode),
}

impl KdfConcrete {
    pub const fn name(self) -> &'static str {
        match self {
            KdfConcrete::Blake3Keyed => "blake3_keyed",
            #[cfg(any(test, feature = "mocks"))]
            KdfConcrete::Mock(_) => "mock",
        }
    }
}

pub const KDF_CONCRETES: &[KdfConcrete] = &[KdfConcrete::Blake3Keyed];

pub trait IKeyDerivationConsumer {
    type Output;

    fn consume_key_derivation<A: IKeyDerivationAdapter>(
        &self,
        params: ConsumeKeyDerivationParams,
        payload: ConsumeKeyDerivationPayload<A>,
    ) -> Self::Output;
}

pub struct ConsumeKeyDerivationParams;

pub struct ConsumeKeyDerivationPayload<A: IKeyDerivationAdapter> {
    pub adapter: A,
    pub declaration: KdfDeclaration,
}

pub struct CreateKeyDerivationDeps<C: IKeyDerivationConsumer> {
    pub consumer: C,
    pub lookup: LookupKdfDeclarationFn,
}

#[derive(Clone, Copy)]
pub struct CreateKeyDerivationParams {
    pub selection: KdfConcrete,
}

pub struct CreateKeyDerivationPayload;

pub struct CreateKeyDerivationSuccessReturn<O> {
    pub output: O,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CreateKeyDerivationErrorReturn {
    Blake3Keyed(Infallible),
    #[cfg(any(test, feature = "mocks"))]
    Mock(Infallible),
}

pub type CreateKeyDerivationReturn<O> =
    Result<CreateKeyDerivationSuccessReturn<O>, CreateKeyDerivationErrorReturn>;

pub type CreateKeyDerivationFn<C> =
    fn(
        &CreateKeyDerivationDeps<C>,
        CreateKeyDerivationParams,
        CreateKeyDerivationPayload,
    ) -> CreateKeyDerivationReturn<<C as IKeyDerivationConsumer>::Output>;

pub const DECLARED_KDF_SELECTIONS: &[CreateKeyDerivationParams] = &[CreateKeyDerivationParams {
    selection: KdfConcrete::Blake3Keyed,
}];
