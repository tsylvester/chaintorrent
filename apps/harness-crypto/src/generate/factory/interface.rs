use core::convert::Infallible;
use domain::AssetIdentityHashTryNewErrorReturn;
use encoding::IEncoderAdapter;
use envelope::IKeyAgreementAdapter;
use hash_to_scalar::IHashToScalarAdapter;
use kem::{ICredentialKemAdapter, IdentityScope};
use pairing::{IPairingArithmetic, IPairingReference};
use proof::{IDeliveryProofAdapter, MintPurpose, TransferPurpose};
use random::IRandomSourceAdapter;
use std::path::PathBuf;

use super::super::evm::provides::{
    CanonicalDeliveryContext, EvmGenerateErrorReturn, EvmGenerateSettings, MsmTermCount,
    VectorCount,
};

pub const GENERATE_INTERFACE_VERSION: u32 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VectorKind {
    G1Addition,
    G1Multiplication,
    PairingCheck,
    G2Addition,
    G1MultiScalarMultiplication,
    G2MultiScalarMultiplication,
    DecodeRejection,
    HashToScalar,
    IdentityMapping,
    Possession,
    Challenge,
    MintProof,
    TransferProof,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VectorKindSizing {
    Drawn,
    DrawnTerms,
    DrawnMessages,
    Enumerated,
    OverConfiguredInputs,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VectorKindDeclaration {
    pub kind: VectorKind,
    pub sizing: VectorKindSizing,
    pub requires_second_group_arithmetic: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GenerateDeclaration {
    pub kinds: &'static [VectorKindDeclaration],
    pub adapter_version: u32,
    pub interface_version: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VectorKindSize {
    Drawn {
        count: VectorCount,
    },
    DrawnTerms {
        count: VectorCount,
        terms: MsmTermCount,
    },
    DrawnMessages {
        count: VectorCount,
        message_length: usize,
    },
    Enumerated,
    OverConfiguredInputs,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VectorKindSettings {
    pub kind: VectorKind,
    pub size: VectorKindSize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VectorKindQuantities {
    pub count: Option<VectorCount>,
    pub terms: Option<MsmTermCount>,
    pub message_length: Option<usize>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VectorKindQuantity {
    Count,
    Terms,
    MessageLength,
}

pub struct EmitParams;

pub struct EmitPayload<'a> {
    pub mint: &'a CanonicalDeliveryContext,
    pub mint_purpose: MintPurpose,
    pub transfer: &'a CanonicalDeliveryContext,
    pub transfer_purpose: TransferPurpose,
    pub identities: &'a [Vec<u8>],
    pub proof_identity: &'a [u8],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EmitSuccessReturn {
    pub files: Vec<EmittedFile>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EmittedFile {
    pub kind: Option<VectorKind>,
    pub path: PathBuf,
}

#[derive(Debug)]
pub enum EmitErrorReturn {
    AssetIdentityLength {
        index: usize,
        actual: usize,
    },
    AssetIdentityHash {
        index: usize,
        error: AssetIdentityHashTryNewErrorReturn,
    },
    AssetProofIdentityLength {
        actual: usize,
    },
    AssetProofIdentityHash(AssetIdentityHashTryNewErrorReturn),
    Evm(EvmGenerateErrorReturn),
    #[cfg(any(test, feature = "mocks"))]
    Mock(MockIGenerateAdapterEmitErrorReturn),
}

#[cfg(any(test, feature = "mocks"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MockIGenerateAdapterFailureMode {
    NoFailure,
    WriteRefused,
}

#[cfg(any(test, feature = "mocks"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MockIGenerateAdapterEmitErrorReturn {
    WriteRefused,
}

pub type EmitReturn = Result<EmitSuccessReturn, EmitErrorReturn>;

pub trait IGenerateAdapter {
    const DECLARATION: GenerateDeclaration;

    fn emit(&self, params: EmitParams, payload: EmitPayload<'_>) -> EmitReturn;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GenerateConcreteKind {
    Evm,
}

pub const GENERATE_CONCRETES: &[GenerateConcreteKind] = &[GenerateConcreteKind::Evm];

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GenerateConcrete {
    Evm {
        settings: Box<EvmGenerateSettings>,
    },
    #[cfg(any(test, feature = "mocks"))]
    Mock(MockIGenerateAdapterFailureMode),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VectorKindConfiguration {
    pub kind: VectorKind,
    pub quantities: VectorKindQuantities,
}

pub trait IGenerateConsumer {
    type Output;

    fn consume_generate<G: IGenerateAdapter>(
        &self,
        params: ConsumeGenerateParams,
        payload: ConsumeGeneratePayload<G>,
    ) -> Self::Output;
}

pub struct ConsumeGenerateParams;

pub struct ConsumeGeneratePayload<G: IGenerateAdapter> {
    pub adapter: G,
}

pub struct CreateGenerateDeps<
    'a,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
    C,
> {
    pub pairing: &'a P,
    pub hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    pub encoder: &'a E,
    pub random: &'a dyn IRandomSourceAdapter,
    pub kem: &'a K,
    pub key_agreement: &'a A,
    pub delivery_proof: &'a D,
    pub consumer: C,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreateGenerateParams {
    pub concrete: GenerateConcrete,
    pub kinds: Vec<VectorKindConfiguration>,
    pub scope: IdentityScope,
    pub statement_version: u16,
}

pub struct CreateGeneratePayload;

pub struct CreateGenerateSuccessReturn<O> {
    pub output: O,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CreateGenerateErrorReturn {
    UndeclaredKind {
        kind: VectorKind,
    },
    DuplicateKind {
        kind: VectorKind,
    },
    MissingQuantity {
        kind: VectorKind,
        quantity: VectorKindQuantity,
    },
    UnexpectedQuantity {
        kind: VectorKind,
        quantity: VectorKindQuantity,
    },
    MissingKind {
        kind: VectorKind,
    },
    Evm(Infallible),
}

pub type CreateGenerateReturn<O> =
    Result<CreateGenerateSuccessReturn<O>, CreateGenerateErrorReturn>;

pub type CreateGenerateFn<'a, P, E, K, A, D, C> =
    fn(
        &CreateGenerateDeps<'a, P, E, K, A, D, C>,
        CreateGenerateParams,
        CreateGeneratePayload,
    ) -> CreateGenerateReturn<<C as IGenerateConsumer>::Output>;
