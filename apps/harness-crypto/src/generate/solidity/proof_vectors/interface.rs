use chain::IChainForms;
#[cfg(any(test, feature = "mocks"))]
use chain::{CreateChainFormsErrorReturn, CreateChainFormsParams};
use core::convert::Infallible;
use core::marker::PhantomData;
use encoding::{CanonicalFieldReaderErrorReturn, CanonicalFieldValue};
#[cfg(any(test, feature = "mocks"))]
use encoding::{CreateEncodingErrorReturn, CreateEncodingParams, IDecoderAdapter, IEncoderAdapter};
use envelope::IKeyAgreementAdapter;
#[cfg(any(test, feature = "mocks"))]
use envelope::{CreateKeyAgreementErrorReturn, CreateKeyAgreementParams};
#[cfg(any(test, feature = "mocks"))]
use hash_to_scalar::{
    CreateHashToScalarErrorReturn, CreateHashToScalarParams, IHashToScalarAdapter,
};
#[cfg(any(test, feature = "mocks"))]
use hashing::Root;
use kem::ICredentialKemAdapter;
#[cfg(any(test, feature = "mocks"))]
use kem::{CreateKemErrorReturn, CreateKemParams};
use pairing::IPairingAdapter;
#[cfg(any(test, feature = "mocks"))]
use pairing::{
    CreatePairingErrorReturn, CreatePairingParams, IPairingArithmetic, IPairingReference,
};
#[cfg(any(test, feature = "mocks"))]
use proof::{CreateDeliveryProofErrorReturn, CreateDeliveryProofParams};
use proof::{
    DeliveryContext, DeliveryRelation, IDeliveryProofAdapter, MintPurpose, TransferPurpose,
};
use random::IRandomSourceAdapter;
#[cfg(any(test, feature = "mocks"))]
use random::{CreateRandomSourceErrorReturn, CreateRandomSourceParams};

use super::super::super::factory::provides::VectorCount;
use super::super::render::provides::{SolidityMemberName, SolidityMemberNameTryNewErrorReturn};
use crate::delivery::provides::{AdmittedKemIdentity, MintErrorReturn, TransferErrorReturn};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProofVectorContext {
    pub suite_identifier: [u8; 32],
    pub suite_version: u16,
    pub chain: CanonicalFieldValue,
    pub entitlement_contract: CanonicalFieldValue,
    pub asset_root: [u8; 32],
    pub parameter_set: [u8; 32],
    pub source_entitlement: CanonicalFieldValue,
    pub target_entitlement: CanonicalFieldValue,
    pub old_interval: CanonicalFieldValue,
    pub new_interval: CanonicalFieldValue,
    pub purpose: u16,
    pub seller: CanonicalFieldValue,
    pub buyer: CanonicalFieldValue,
    pub expiry: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MintProofVector {
    pub draws: Vec<u8>,
    pub context: ProofVectorContext,
    pub hpub: Vec<u8>,
    pub identity_element: Vec<u8>,
    pub buyer_pk1: Vec<u8>,
    pub buyer_pk2: Vec<u8>,
    pub envelope_c1: Vec<u8>,
    pub envelope_c2: Vec<u8>,
    pub envelope_d1: Vec<u8>,
    pub envelope_d2: Vec<u8>,
    pub challenge: [u8; 32],
    pub alpha: [u8; 32],
    pub r: [u8; 32],
    pub rho: [u8; 32],
    pub sigma: [u8; 32],
    pub second_group_first_messages: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransferProofVector {
    pub draws: Vec<u8>,
    pub context: ProofVectorContext,
    pub identity_element: Vec<u8>,
    pub seller_pk1: Vec<u8>,
    pub seller_pk2: Vec<u8>,
    pub buyer_pk1: Vec<u8>,
    pub buyer_pk2: Vec<u8>,
    pub old_envelope_c1: Vec<u8>,
    pub old_envelope_c2: Vec<u8>,
    pub old_envelope_d1: Vec<u8>,
    pub old_envelope_d2: Vec<u8>,
    pub new_envelope_c1: Vec<u8>,
    pub new_envelope_c2: Vec<u8>,
    pub new_envelope_d1: Vec<u8>,
    pub new_envelope_d2: Vec<u8>,
    pub challenge: [u8; 32],
    pub x: [u8; 32],
    pub y: [u8; 32],
    pub s: [u8; 32],
    pub rho: [u8; 32],
    pub sigma: [u8; 32],
    pub second_group_first_messages: Vec<u8>,
}

pub const MINT_PROOF_VECTOR_FIELD_COUNT: usize = 29;

pub const MINT_PROOF_VECTOR_MEMBER_NAMES: [&str; MINT_PROOF_VECTOR_FIELD_COUNT] = [
    "draws",
    "suiteIdentifier",
    "suiteVersion",
    "chain",
    "entitlementContract",
    "assetRoot",
    "parameterSet",
    "sourceEntitlement",
    "targetEntitlement",
    "oldInterval",
    "newInterval",
    "purpose",
    "seller",
    "buyer",
    "expiry",
    "hpub",
    "identityElement",
    "buyerPk1",
    "buyerPk2",
    "envelopeC1",
    "envelopeC2",
    "envelopeD1",
    "envelopeD2",
    "challenge",
    "alpha",
    "r",
    "rho",
    "sigma",
    "secondGroupFirstMessages",
];

pub const TRANSFER_PROOF_VECTOR_FIELD_COUNT: usize = 35;

pub const TRANSFER_PROOF_VECTOR_MEMBER_NAMES: [&str; TRANSFER_PROOF_VECTOR_FIELD_COUNT] = [
    "draws",
    "suiteIdentifier",
    "suiteVersion",
    "chain",
    "entitlementContract",
    "assetRoot",
    "parameterSet",
    "sourceEntitlement",
    "targetEntitlement",
    "oldInterval",
    "newInterval",
    "purpose",
    "seller",
    "buyer",
    "expiry",
    "identityElement",
    "sellerPk1",
    "sellerPk2",
    "buyerPk1",
    "buyerPk2",
    "oldEnvelopeC1",
    "oldEnvelopeC2",
    "oldEnvelopeD1",
    "oldEnvelopeD2",
    "newEnvelopeC1",
    "newEnvelopeC2",
    "newEnvelopeD1",
    "newEnvelopeD2",
    "challenge",
    "x",
    "y",
    "s",
    "rho",
    "sigma",
    "secondGroupFirstMessages",
];

pub struct MintProofVectorDescription<F: IChainForms> {
    pub(super) forms: PhantomData<F>,
}

pub struct MintProofVectorDescriptionConstructorParams;

pub type MintProofVectorDescriptionTryNewReturn<F> =
    Result<MintProofVectorDescription<F>, Infallible>;

pub struct TransferProofVectorDescription<F: IChainForms> {
    pub(super) forms: PhantomData<F>,
}

pub struct TransferProofVectorDescriptionConstructorParams;

pub type TransferProofVectorDescriptionTryNewReturn<F> =
    Result<TransferProofVectorDescription<F>, Infallible>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MintProofVectorFromFieldsErrorReturn {
    Fields(CanonicalFieldReaderErrorReturn),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransferProofVectorFromFieldsErrorReturn {
    Fields(CanonicalFieldReaderErrorReturn),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProofVectorMemberNamesErrorReturn {
    MemberName(SolidityMemberNameTryNewErrorReturn),
}

pub type ProofVectorMemberNamesReturn =
    Result<Vec<SolidityMemberName>, ProofVectorMemberNamesErrorReturn>;

pub struct ProofVectorsDeps<
    'a,
    P: IPairingAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
> {
    pub pairing: &'a P,
    pub kem: &'a K,
    pub key_agreement: &'a A,
    pub delivery_proof: &'a D,
    pub random: &'a dyn IRandomSourceAdapter,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProofVectorsParams {
    pub mint_count: VectorCount,
    pub transfer_count: VectorCount,
}

pub struct ProofVectorsPayload<'a, F: IChainForms> {
    pub mint: &'a DeliveryContext<F, MintPurpose>,
    pub transfer: &'a DeliveryContext<F, TransferPurpose>,
    pub identity: AdmittedKemIdentity<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProofVectorsSuccessReturn {
    pub mint: Vec<MintProofVector>,
    pub transfer: Vec<TransferProofVector>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ProofVectorsErrorReturn {
    Mint(MintErrorReturn),
    Transfer(TransferErrorReturn),
    ProofComponents { relation: DeliveryRelation },
    ScalarLength { actual: usize },
}

pub type ProofVectorsReturn = Result<ProofVectorsSuccessReturn, ProofVectorsErrorReturn>;

pub type ProofVectorsFn<'a, P, K, A, D> = fn(
    &ProofVectorsDeps<'a, P, K, A, D>,
    ProofVectorsParams,
    ProofVectorsPayload<'_, <D as IDeliveryProofAdapter>::Forms>,
) -> ProofVectorsReturn;

#[cfg(any(test, feature = "mocks"))]
pub trait IProofVectorsCaseConsumer {
    type Output;

    fn consume_proof_vectors_case<P, H, E, F, K, A, D>(
        &self,
        params: ConsumeProofVectorsCaseParams,
        payload: ConsumeProofVectorsCasePayload<'_, P, H, E, K, A, D>,
    ) -> Self::Output
    where
        P: IPairingArithmetic + IPairingReference,
        H: ?Sized + IHashToScalarAdapter<P::Scalar>,
        E: IEncoderAdapter + IDecoderAdapter,
        F: IChainForms,
        K: ICredentialKemAdapter<Pairing = P>,
        A: IKeyAgreementAdapter<Pairing = P>,
        D: IDeliveryProofAdapter<Pairing = P, Forms = F>;
}

#[cfg(any(test, feature = "mocks"))]
pub struct ConsumeProofVectorsCaseParams;

#[cfg(any(test, feature = "mocks"))]
pub struct ConsumeProofVectorsCasePayload<'a, P, H, E, K, A, D>
where
    P: IPairingArithmetic + IPairingReference,
    H: ?Sized + IHashToScalarAdapter<P::Scalar>,
    E: IEncoderAdapter + IDecoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
{
    pub pairing: &'a P,
    pub hash_to_scalar: &'a H,
    pub encoder: &'a E,
    pub decoder: &'a E,
    pub kem: &'a K,
    pub key_agreement: &'a A,
    pub delivery_proof: &'a D,
    pub random: &'a dyn IRandomSourceAdapter,
    pub identity: AdmittedKemIdentity<'a>,
}

#[cfg(any(test, feature = "mocks"))]
pub struct ProofVectorsCaseDeps<C: IProofVectorsCaseConsumer> {
    pub consumer: C,
}

#[cfg(any(test, feature = "mocks"))]
pub struct ProofVectorsCaseParams {
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
pub struct ProofVectorsCasePayload<'a> {
    pub canonical: &'a [u8],
    pub root: &'a Root,
}

#[cfg(any(test, feature = "mocks"))]
pub struct ProofVectorsCaseSuccessReturn<O> {
    pub output: O,
}

#[cfg(any(test, feature = "mocks"))]
#[derive(Debug, PartialEq, Eq)]
pub enum ProofVectorsCaseErrorReturn {
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
pub type ProofVectorsCaseReturn<O> =
    Result<ProofVectorsCaseSuccessReturn<O>, ProofVectorsCaseErrorReturn>;

#[cfg(any(test, feature = "mocks"))]
pub type RunProofVectorsCaseFn<C> =
    fn(
        &ProofVectorsCaseDeps<C>,
        ProofVectorsCaseParams,
        ProofVectorsCasePayload<'_>,
    ) -> ProofVectorsCaseReturn<<C as IProofVectorsCaseConsumer>::Output>;
