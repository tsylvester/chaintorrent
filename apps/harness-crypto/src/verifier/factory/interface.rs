use chain::{
    ChainFormsIdentifier, ChainMemberIdentifier, FeeDescription, IChainAdapter,
    IEntitlementStateAdapter,
};
#[cfg(any(test, feature = "mocks"))]
use chain::{
    CreateChainErrorReturn, CreateChainFormsErrorReturn, CreateChainFormsParams, CreateChainParams,
};
use encoding::{CanonicalFieldValue, EncodingDeclaration, EncodingIdentifier};
#[cfg(any(test, feature = "mocks"))]
use encoding::{CreateEncodingErrorReturn, CreateEncodingParams};
#[cfg(any(test, feature = "mocks"))]
use hash_to_scalar::{CreateHashToScalarErrorReturn, CreateHashToScalarParams};
#[cfg(any(test, feature = "mocks"))]
use pairing::{
    CreatePairingErrorReturn, CreatePairingParams, IPairingArithmetic, IPairingReference,
};
use pairing::{IPairingAdapter, VerifierGroupArithmetic};
#[cfg(any(test, feature = "mocks"))]
use proof::{CreateDeliveryProofErrorReturn, CreateDeliveryProofParams};
use proof::{DeliveryRelation, IDeliveryProofAdapter, VerifyPayload};
#[cfg(any(test, feature = "mocks"))]
use random::{CreateRandomSourceErrorReturn, CreateRandomSourceParams};

use super::super::admit_verifier_composition::provides::{
    AdmitVerifierCompositionErrorReturn, AdmitVerifierCompositionFn,
};
use super::super::evm::provides::{
    EvmVerifierErrorReturn, EvmVerifierSettings, EvmVerifierTryNewErrorReturn,
};
use crate::generate::provides::GenerateDeclaration;

pub const VERIFIER_INTERFACE_VERSION: u32 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VerifierSettingBounds {
    Unbounded,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VerifierSettingDisposition {
    Fixed {
        value: &'static str,
    },
    Settable {
        bounds: VerifierSettingBounds,
        default: &'static str,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VerifierSettingStatement {
    pub name: &'static str,
    pub disposition: VerifierSettingDisposition,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VerifierDeclaration {
    pub identifier: VerifierConcreteKind,
    pub name: &'static str,
    pub verifier_forms: &'static [VerifierGroupArithmetic],
    pub statement_versions: &'static [u16],
    pub adapter_version: u32,
    pub interface_version: u32,
    pub chain_forms_identifier: ChainFormsIdentifier,
    pub chain_member: ChainMemberIdentifier,
    pub encoding_identifier: EncodingIdentifier,
    pub contract_suite: &'static str,
    pub settings: &'static [VerifierSettingStatement],
}

pub struct VerifyOnChainParams;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerifyCost {
    pub description: FeeDescription,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VerifyOnChainSuccessReturn {
    Accepted { cost: VerifyCost },
    Rejected,
}

#[derive(Debug)]
pub enum VerifyOnChainErrorReturn {
    RelationMismatch {
        statement: DeliveryRelation,
        proof: DeliveryRelation,
    },
    Evm(EvmVerifierErrorReturn),
    #[cfg(any(test, feature = "mocks"))]
    Mock(MockIVerifierAdapterVerifyOnChainErrorReturn),
}

#[cfg(any(test, feature = "mocks"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MockIVerifierAdapterFailureMode {
    NoFailure,
    RpcRefused,
}

#[cfg(any(test, feature = "mocks"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MockIVerifierAdapterVerifyOnChainErrorReturn {
    RpcRefused,
}

pub type VerifyOnChainReturn = Result<VerifyOnChainSuccessReturn, VerifyOnChainErrorReturn>;

pub trait IVerifierAdapter {
    const DECLARATION: VerifierDeclaration;

    type DeliveryProof: IDeliveryProofAdapter;

    fn verify_on_chain(
        &self,
        params: VerifyOnChainParams,
        payload: VerifyPayload<
            '_,
            <Self::DeliveryProof as IDeliveryProofAdapter>::Forms,
            <<Self::DeliveryProof as IDeliveryProofAdapter>::Pairing as IPairingAdapter>::G1,
            <<Self::DeliveryProof as IDeliveryProofAdapter>::Pairing as IPairingAdapter>::G2,
            <Self::DeliveryProof as IDeliveryProofAdapter>::Proof,
        >,
    ) -> VerifyOnChainReturn;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VerifierConcreteKind {
    Evm,
    #[cfg(any(test, feature = "mocks"))]
    Mock,
}

pub const VERIFIER_CONCRETES: &[VerifierConcreteKind] = &[VerifierConcreteKind::Evm];

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VerifierConcrete {
    Evm {
        settings: Box<EvmVerifierSettings>,
    },
    #[cfg(any(test, feature = "mocks"))]
    Mock(MockIVerifierAdapterFailureMode),
}

pub trait IVerifierConsumer<D: IDeliveryProofAdapter> {
    type Output;

    fn consume_verifier<V: IVerifierAdapter<DeliveryProof = D>>(
        &self,
        params: ConsumeVerifierParams,
        payload: ConsumeVerifierPayload<V>,
    ) -> Self::Output;
}

pub struct ConsumeVerifierParams;

pub struct ConsumeVerifierPayload<V: IVerifierAdapter> {
    pub adapter: V,
}

pub struct CreateVerifierDeps<
    'a,
    P: IPairingAdapter,
    C: IChainAdapter,
    D: IDeliveryProofAdapter<Pairing = P, Forms = <C as IEntitlementStateAdapter>::Forms>,
    K,
> {
    pub pairing: &'a P,
    pub chain: &'a C,
    pub delivery_proof: &'a D,
    pub encoder: &'a EncodingDeclaration,
    pub generate: &'a GenerateDeclaration,
    pub admit_verifier_composition: AdmitVerifierCompositionFn,
    pub consumer: K,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreateVerifierParams {
    pub concrete: VerifierConcrete,
    pub verifier_address: CanonicalFieldValue,
    pub statement_version: u16,
}

pub struct CreateVerifierPayload;

pub struct CreateVerifierSuccessReturn<O> {
    pub output: O,
}

#[derive(Debug)]
pub enum CreateVerifierErrorReturn {
    Composition(AdmitVerifierCompositionErrorReturn),
    UnsupportedStatementVersion { version: u16 },
    Evm(EvmVerifierTryNewErrorReturn),
}

pub type CreateVerifierReturn<O> =
    Result<CreateVerifierSuccessReturn<O>, CreateVerifierErrorReturn>;

pub type CreateVerifierFn<'a, P, C, D, K> =
    fn(
        &CreateVerifierDeps<'a, P, C, D, K>,
        CreateVerifierParams,
        CreateVerifierPayload,
    ) -> CreateVerifierReturn<<K as IVerifierConsumer<D>>::Output>;

#[cfg(any(test, feature = "mocks"))]
pub trait ICreateVerifierCaseConsumer {
    type Output;

    fn consume_create_verifier_case<P, C, D>(
        &self,
        params: ConsumeCreateVerifierCaseParams,
        payload: ConsumeCreateVerifierCasePayload<'_, P, C, D>,
    ) -> Self::Output
    where
        P: IPairingArithmetic + IPairingReference,
        C: IChainAdapter,
        D: IDeliveryProofAdapter<Pairing = P, Forms = <C as IEntitlementStateAdapter>::Forms>;
}

#[cfg(any(test, feature = "mocks"))]
pub struct ConsumeCreateVerifierCaseParams;

#[cfg(any(test, feature = "mocks"))]
pub struct ConsumeCreateVerifierCasePayload<
    'a,
    P: IPairingArithmetic + IPairingReference,
    C: IChainAdapter,
    D: IDeliveryProofAdapter<Pairing = P, Forms = <C as IEntitlementStateAdapter>::Forms>,
> {
    pub pairing: &'a P,
    pub chain: &'a C,
    pub delivery_proof: &'a D,
    pub encoder: &'a EncodingDeclaration,
    pub generate: &'a GenerateDeclaration,
}

#[cfg(any(test, feature = "mocks"))]
pub struct CreateVerifierCaseDeps<C: ICreateVerifierCaseConsumer> {
    pub consumer: C,
}

#[cfg(any(test, feature = "mocks"))]
pub struct CreateVerifierCaseParams {
    pub pairing: CreatePairingParams,
    pub hash_to_scalar: CreateHashToScalarParams,
    pub encoding: CreateEncodingParams,
    pub chain_forms: CreateChainFormsParams,
    pub chain: CreateChainParams,
    pub delivery_proof: CreateDeliveryProofParams,
    pub random_source: CreateRandomSourceParams,
}

#[cfg(any(test, feature = "mocks"))]
pub struct CreateVerifierCasePayload<'a> {
    pub generate: &'a GenerateDeclaration,
}

#[cfg(any(test, feature = "mocks"))]
pub struct CreateVerifierCaseSuccessReturn<O> {
    pub output: O,
}

#[cfg(any(test, feature = "mocks"))]
#[derive(Debug)]
pub enum CreateVerifierCaseErrorReturn {
    CreatePairing(CreatePairingErrorReturn),
    CreateHashToScalar(CreateHashToScalarErrorReturn),
    CreateEncoding(CreateEncodingErrorReturn),
    CreateChainForms(CreateChainFormsErrorReturn),
    CreateChain(CreateChainErrorReturn),
    CreateDeliveryProof(CreateDeliveryProofErrorReturn),
    CreateRandomSource(CreateRandomSourceErrorReturn),
}

#[cfg(any(test, feature = "mocks"))]
pub type CreateVerifierCaseReturn<O> =
    Result<CreateVerifierCaseSuccessReturn<O>, CreateVerifierCaseErrorReturn>;

#[cfg(any(test, feature = "mocks"))]
pub type RunCreateVerifierCaseFn<C> =
    fn(
        &CreateVerifierCaseDeps<C>,
        CreateVerifierCaseParams,
        CreateVerifierCasePayload<'_>,
    ) -> CreateVerifierCaseReturn<<C as ICreateVerifierCaseConsumer>::Output>;
