use chain::IChainForms;
#[cfg(any(test, feature = "mocks"))]
use chain::{CreateChainFormsErrorReturn, CreateChainFormsParams};
use core::convert::Infallible;
use core::num::NonZeroU32;
#[cfg(any(test, feature = "mocks"))]
use encoding::{CreateEncodingErrorReturn, CreateEncodingParams, IDecoderAdapter};
use encoding::{EncodingIdentifier, IEncoderAdapter};
use envelope::IKeyAgreementAdapter;
#[cfg(any(test, feature = "mocks"))]
use envelope::{CreateKeyAgreementErrorReturn, CreateKeyAgreementParams};
use hash_to_scalar::IHashToScalarAdapter;
#[cfg(any(test, feature = "mocks"))]
use hash_to_scalar::{CreateHashToScalarErrorReturn, CreateHashToScalarParams};
#[cfg(any(test, feature = "mocks"))]
use hashing::Root;
use kem::ICredentialKemAdapter;
#[cfg(any(test, feature = "mocks"))]
use kem::{CreateKemErrorReturn, CreateKemParams};
#[cfg(any(test, feature = "mocks"))]
use pairing::{CreatePairingErrorReturn, CreatePairingParams};
use pairing::{IPairingArithmetic, IPairingReference};
#[cfg(any(test, feature = "mocks"))]
use proof::{CreateDeliveryProofErrorReturn, CreateDeliveryProofParams};
use proof::{DeliveryContext, IDeliveryProofAdapter, MintPurpose, TransferPurpose};
use random::IRandomSourceAdapter;
#[cfg(any(test, feature = "mocks"))]
use random::{CreateRandomSourceErrorReturn, CreateRandomSourceParams};
use std::path::PathBuf;

use super::super::solidity::provides::{SolidityGenerateErrorReturn, SolidityGenerateSettings};
use crate::delivery::provides::AdmittedKemIdentity;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VectorCount {
    pub(super) count: NonZeroU32,
}

pub struct VectorCountConstructorParams {
    pub count: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VectorCountTryNewErrorReturn {
    Zero,
}

pub type VectorCountTryNewReturn = Result<VectorCount, VectorCountTryNewErrorReturn>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MsmTermCount {
    pub(super) count: NonZeroU32,
}

pub struct MsmTermCountConstructorParams {
    pub count: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MsmTermCountTryNewErrorReturn {
    Zero,
}

pub type MsmTermCountTryNewReturn = Result<MsmTermCount, MsmTermCountTryNewErrorReturn>;

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
    CellValues,
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
    pub encoding_identifier: EncodingIdentifier,
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

pub struct EmitPayload<'a, F: IChainForms> {
    pub mint: &'a DeliveryContext<F, MintPurpose>,
    pub transfer: &'a DeliveryContext<F, TransferPurpose>,
    pub identities: &'a [AdmittedKemIdentity<'a>],
    pub proof_identity: AdmittedKemIdentity<'a>,
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
    MissingKindSize {
        kind: VectorKind,
    },
    Solidity(SolidityGenerateErrorReturn),
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
    type Forms: IChainForms;

    const DECLARATION: GenerateDeclaration;

    fn emit(&self, params: EmitParams, payload: EmitPayload<'_, Self::Forms>) -> EmitReturn;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GenerateConcreteKind {
    Solidity,
}

pub const GENERATE_CONCRETES: &[GenerateConcreteKind] = &[GenerateConcreteKind::Solidity];

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GenerateConcrete {
    Solidity {
        settings: Box<SolidityGenerateSettings>,
    },
    #[cfg(any(test, feature = "mocks"))]
    Mock(MockIGenerateAdapterFailureMode),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VectorKindConfiguration {
    pub kind: VectorKind,
    pub quantities: VectorKindQuantities,
}

pub trait IGenerateConsumer<F: IChainForms> {
    type Output;

    fn consume_generate<G: IGenerateAdapter<Forms = F>>(
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
    UnsupportedEncodingIdentifier,
    Solidity(Infallible),
}

pub type CreateGenerateReturn<O> =
    Result<CreateGenerateSuccessReturn<O>, CreateGenerateErrorReturn>;

pub type CreateGenerateFn<'a, P, E, K, A, D, C> = fn(
    &CreateGenerateDeps<'a, P, E, K, A, D, C>,
    CreateGenerateParams,
    CreateGeneratePayload,
) -> CreateGenerateReturn<
    <C as IGenerateConsumer<<D as IDeliveryProofAdapter>::Forms>>::Output,
>;

#[cfg(any(test, feature = "mocks"))]
pub trait ICreateGenerateCaseConsumer {
    type Output;

    fn consume_create_generate_case<P, E, F, K, A, D>(
        &self,
        params: ConsumeCreateGenerateCaseParams,
        payload: ConsumeCreateGenerateCasePayload<'_, P, E, K, A, D>,
    ) -> Self::Output
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter + IDecoderAdapter,
        F: IChainForms,
        K: ICredentialKemAdapter<Pairing = P>,
        A: IKeyAgreementAdapter<Pairing = P>,
        D: IDeliveryProofAdapter<Pairing = P, Forms = F>;
}

#[cfg(any(test, feature = "mocks"))]
pub struct ConsumeCreateGenerateCaseParams;

#[cfg(any(test, feature = "mocks"))]
pub struct ConsumeCreateGenerateCasePayload<
    'a,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
> {
    pub pairing: &'a P,
    pub hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    pub encoder: &'a E,
    pub decoder: &'a E,
    pub kem: &'a K,
    pub key_agreement: &'a A,
    pub delivery_proof: &'a D,
    pub random: &'a dyn IRandomSourceAdapter,
    pub identities: Vec<AdmittedKemIdentity<'a>>,
}

#[cfg(any(test, feature = "mocks"))]
pub struct CreateGenerateCaseDeps<C: ICreateGenerateCaseConsumer> {
    pub consumer: C,
}

#[cfg(any(test, feature = "mocks"))]
pub struct CreateGenerateCaseParams {
    pub pairing: CreatePairingParams,
    pub hash_to_scalar: CreateHashToScalarParams,
    pub encoding: CreateEncodingParams,
    pub chain_forms: CreateChainFormsParams,
    pub kem: CreateKemParams,
    pub key_agreement: CreateKeyAgreementParams,
    pub delivery_proof: CreateDeliveryProofParams,
    pub random_source: CreateRandomSourceParams,
}

#[cfg(any(test, feature = "mocks"))]
pub struct CreateGenerateCasePayload<'a> {
    pub canonicals: &'a [&'a [u8]],
    pub roots: &'a [&'a Root],
}

#[cfg(any(test, feature = "mocks"))]
pub struct CreateGenerateCaseSuccessReturn<O> {
    pub output: O,
}

#[cfg(any(test, feature = "mocks"))]
#[derive(Debug, PartialEq, Eq)]
pub enum CreateGenerateCaseErrorReturn {
    CreatePairing(CreatePairingErrorReturn),
    CreateHashToScalar(CreateHashToScalarErrorReturn),
    CreateEncoding(CreateEncodingErrorReturn),
    CreateChainForms(CreateChainFormsErrorReturn),
    CreateKem(CreateKemErrorReturn),
    CreateKeyAgreement(CreateKeyAgreementErrorReturn),
    CreateDeliveryProof(CreateDeliveryProofErrorReturn),
    CreateRandomSource(CreateRandomSourceErrorReturn),
}

#[cfg(any(test, feature = "mocks"))]
pub type CreateGenerateCaseReturn<O> =
    Result<CreateGenerateCaseSuccessReturn<O>, CreateGenerateCaseErrorReturn>;

#[cfg(any(test, feature = "mocks"))]
pub type RunCreateGenerateCaseFn<C> =
    fn(
        &CreateGenerateCaseDeps<C>,
        CreateGenerateCaseParams,
        CreateGenerateCasePayload<'_>,
    ) -> CreateGenerateCaseReturn<<C as ICreateGenerateCaseConsumer>::Output>;
