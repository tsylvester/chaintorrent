use chain::IChainForms;
#[cfg(any(test, feature = "mocks"))]
use chain::{CreateChainFormsErrorReturn, CreateChainFormsParams};
use encoding::IEncoderAdapter;
#[cfg(any(test, feature = "mocks"))]
use encoding::{CreateEncodingErrorReturn, CreateEncodingParams, IDecoderAdapter};
use envelope::IKeyAgreementAdapter;
#[cfg(any(test, feature = "mocks"))]
use envelope::{CreateKeyAgreementErrorReturn, CreateKeyAgreementParams};
#[cfg(any(test, feature = "mocks"))]
use hash_to_scalar::{
    CreateHashToScalarErrorReturn, CreateHashToScalarParams, IHashToScalarAdapter,
};
#[cfg(any(test, feature = "mocks"))]
use hashing::Root;
use kdf::IKeyDerivationAdapter;
#[cfg(any(test, feature = "mocks"))]
use kdf::{CreateKeyDerivationErrorReturn, CreateKeyDerivationParams};
use kem::ICredentialKemAdapter;
#[cfg(any(test, feature = "mocks"))]
use kem::{CreateKemErrorReturn, CreateKemParams};
#[cfg(any(test, feature = "mocks"))]
use pairing::{CreatePairingErrorReturn, CreatePairingParams, IPairingReference};
use pairing::{IPairingAdapter, IPairingArithmetic};
#[cfg(any(test, feature = "mocks"))]
use proof::{CreateDeliveryProofErrorReturn, CreateDeliveryProofParams};
use proof::{DeliveryContext, IDeliveryProofAdapter, MintPurpose, TransferPurpose};
use random::IRandomSourceAdapter;
#[cfg(any(test, feature = "mocks"))]
use random::{CreateRandomSourceErrorReturn, CreateRandomSourceParams};
use workflows::DerivationContext;

use super::cross_holder::provides::{CrossHolderErrorReturn, CrossHolderSuccessReturn};
use super::cross_set::provides::{CrossSetErrorReturn, CrossSetSuccessReturn};
use super::malformed_capsules::provides::{
    MalformedCapsulesErrorReturn, MalformedCapsulesSuccessReturn,
};
use super::non_convertibility::provides::{
    NonConvertibilityErrorReturn, NonConvertibilitySuccessReturn,
};
use super::replay::provides::{ReplayErrorReturn, ReplaySuccessReturn};
use super::rerandomization::provides::{RerandomizationErrorReturn, RerandomizationSuccessReturn};
use super::statement_mutations::provides::{
    StatementMutationsErrorReturn, StatementMutationsSuccessReturn,
};
use crate::VectorCount;
use crate::delivery::provides::AdmittedKemIdentity;

pub struct VectorsDeps<
    'a,
    P: IPairingArithmetic,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
    E: IEncoderAdapter,
> {
    pub pairing: &'a P,
    pub kem: &'a K,
    pub key_agreement: &'a A,
    pub delivery_proof: &'a D,
    pub random: &'a dyn IRandomSourceAdapter,
    pub kdf: &'a dyn IKeyDerivationAdapter,
    pub encoder: &'a E,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VectorsParams {
    pub cross_holder: VectorCount,
    pub rerandomization: VectorCount,
    pub non_convertibility: VectorCount,
    pub malformed_capsules: VectorCount,
    pub cross_set: VectorCount,
    pub statement_mutations: VectorCount,
    pub replay: VectorCount,
}

pub struct VectorsPayload<'a, F: IChainForms> {
    pub identity: AdmittedKemIdentity<'a>,
    pub other_identity: AdmittedKemIdentity<'a>,
    pub context: &'a DerivationContext,
    pub mint: &'a DeliveryContext<F, MintPurpose>,
    pub transfer: &'a DeliveryContext<F, TransferPurpose>,
    pub next: &'a DeliveryContext<F, TransferPurpose>,
}

pub struct VectorsSuccessReturn<
    P: IPairingAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
> {
    pub cross_holder: CrossHolderSuccessReturn<K>,
    pub rerandomization: RerandomizationSuccessReturn<K>,
    pub non_convertibility: NonConvertibilitySuccessReturn<K>,
    pub malformed_capsules: MalformedCapsulesSuccessReturn<K>,
    pub cross_set: CrossSetSuccessReturn<K>,
    pub statement_mutations: StatementMutationsSuccessReturn<P, D>,
    pub replay: ReplaySuccessReturn<P, D>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum VectorsErrorReturn {
    CrossHolder(CrossHolderErrorReturn),
    Rerandomization(RerandomizationErrorReturn),
    NonConvertibility(NonConvertibilityErrorReturn),
    MalformedCapsules(MalformedCapsulesErrorReturn),
    CrossSet(CrossSetErrorReturn),
    StatementMutations(StatementMutationsErrorReturn),
    Replay(ReplayErrorReturn),
}

pub type VectorsReturn<P, K, D> = Result<VectorsSuccessReturn<P, K, D>, VectorsErrorReturn>;

pub type VectorsFn<'a, P, K, A, D, E> = fn(
    &VectorsDeps<'a, P, K, A, D, E>,
    VectorsParams,
    VectorsPayload<'_, <D as IDeliveryProofAdapter>::Forms>,
) -> VectorsReturn<P, K, D>;

#[cfg(any(test, feature = "mocks"))]
pub trait IVectorsCaseConsumer {
    type Output;

    fn consume_vectors_case<P, H, E, F, K, A, D>(
        &self,
        pairing: &P,
        hash_to_scalar: &H,
        encoder: &E,
        forms: &F,
        kem: &K,
        key_agreement: &A,
        delivery_proof: &D,
        random: &dyn IRandomSourceAdapter,
        independent: &dyn IRandomSourceAdapter,
        kdf: &dyn IKeyDerivationAdapter,
        identity: AdmittedKemIdentity<'_>,
        other_identity: AdmittedKemIdentity<'_>,
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
pub struct VectorsCaseDeps<C: IVectorsCaseConsumer> {
    pub consumer: C,
}

#[cfg(any(test, feature = "mocks"))]
pub struct VectorsCaseParams {
    pub pairing: CreatePairingParams,
    pub hash_to_scalar: CreateHashToScalarParams,
    pub encoding: CreateEncodingParams,
    pub key_derivation: CreateKeyDerivationParams,
    pub chain_forms: CreateChainFormsParams,
    pub kem: CreateKemParams,
    pub key_agreement: CreateKeyAgreementParams,
    pub delivery_proof: CreateDeliveryProofParams,
    pub random_source: CreateRandomSourceParams,
}

#[cfg(any(test, feature = "mocks"))]
pub struct VectorsCasePayload<'a> {
    pub canonical: &'a [u8],
    pub other_canonical: &'a [u8],
    pub root: &'a Root,
    pub other_root: &'a Root,
}

#[cfg(any(test, feature = "mocks"))]
pub struct VectorsCaseSuccessReturn<O> {
    pub output: O,
}

#[cfg(any(test, feature = "mocks"))]
#[derive(Debug, PartialEq, Eq)]
pub enum VectorsCaseErrorReturn {
    CreatePairing(CreatePairingErrorReturn),
    CreateHashToScalar(CreateHashToScalarErrorReturn),
    CreateEncoding(CreateEncodingErrorReturn),
    CreateKeyDerivation(CreateKeyDerivationErrorReturn),
    CreateChainForms(CreateChainFormsErrorReturn),
    CreateKem(CreateKemErrorReturn),
    CreateKeyAgreement(CreateKeyAgreementErrorReturn),
    CreateDeliveryProof(CreateDeliveryProofErrorReturn),
    CreateRandomSource(CreateRandomSourceErrorReturn),
}

#[cfg(any(test, feature = "mocks"))]
pub type VectorsCaseReturn<O> = Result<VectorsCaseSuccessReturn<O>, VectorsCaseErrorReturn>;

#[cfg(any(test, feature = "mocks"))]
pub type RunVectorsCaseFn<C> = fn(
    &VectorsCaseDeps<C>,
    VectorsCaseParams,
    VectorsCasePayload<'_>,
) -> VectorsCaseReturn<<C as IVectorsCaseConsumer>::Output>;
