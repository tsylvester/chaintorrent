use chain::IChainForms;
#[cfg(any(test, feature = "mocks"))]
use chain::{CreateChainFormsErrorReturn, CreateChainFormsParams};
use domain::Secret;
#[cfg(any(test, feature = "mocks"))]
use encoding::{CreateEncodingErrorReturn, CreateEncodingParams};
#[cfg(any(test, feature = "mocks"))]
use envelope::{CreateKeyAgreementErrorReturn, CreateKeyAgreementParams};
use envelope::{
    EnvelopeCoins, GenerateKeysErrorReturn, IKeyAgreementAdapter,
    PublicKeysFromComponentsErrorReturn, WrapToErrorReturn,
};
#[cfg(any(test, feature = "mocks"))]
use hash_to_scalar::{CreateHashToScalarErrorReturn, CreateHashToScalarParams};
#[cfg(any(test, feature = "mocks"))]
use hashing::Root;
#[cfg(any(test, feature = "mocks"))]
use kem::{CreateKemErrorReturn, CreateKemParams};
use kem::{
    DeriveIdentityErrorReturn, ICredentialKemAdapter, IssueErrorReturn, KemIdentity,
    MasterScalarComponents, SetupErrorReturn, SetupScope,
};
use pairing::IPairingAdapter;
#[cfg(any(test, feature = "mocks"))]
use pairing::{
    CreatePairingErrorReturn, CreatePairingParams, IPairingArithmetic, IPairingReference,
};
use proof::{
    AlgebraicMintStatement, DeliveryContext, IDeliveryProofAdapter, MintPurpose,
    ProveMintErrorReturn,
};
#[cfg(any(test, feature = "mocks"))]
use proof::{CreateDeliveryProofErrorReturn, CreateDeliveryProofParams};
#[cfg(any(test, feature = "mocks"))]
use random::{CreateRandomSourceErrorReturn, CreateRandomSourceParams};
use random::{FillBytesErrorReturn, IRandomSourceAdapter};

#[derive(Clone, Copy)]
pub struct AdmittedKemIdentity<'a> {
    pub setup_scope: SetupScope<'a>,
    pub identity: KemIdentity<'a>,
}

pub struct DeliveryHolder<A: IKeyAgreementAdapter> {
    pub key_pair: A::KeyPair,
    pub public_keys: A::PublicKeys,
    pub envelope: A::Envelope,
}

pub struct MintWitnesses<P: IPairingAdapter> {
    pub master_scalar: MasterScalarComponents<P::Scalar>,
    pub credential_randomness: Secret<P::Scalar>,
    pub coins: EnvelopeCoins<P::Scalar>,
}

pub struct MintDelivery<
    P: IPairingAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
> {
    pub statement: AlgebraicMintStatement<D::Forms, P::G1, P::G2>,
    pub proof: D::Proof,
    pub parameter_set: K::ParameterSet,
    pub identity_element: K::IdentityElement,
    pub credential: K::Credential,
    pub witnesses: MintWitnesses<P>,
    pub recipient: DeliveryHolder<A>,
    pub draws: Vec<u8>,
}

pub struct MintDeps<
    'a,
    K: ICredentialKemAdapter,
    A: IKeyAgreementAdapter<Pairing = K::Pairing>,
    D: IDeliveryProofAdapter<Pairing = K::Pairing>,
> {
    pub kem: &'a K,
    pub key_agreement: &'a A,
    pub delivery_proof: &'a D,
    pub random: &'a dyn IRandomSourceAdapter,
}

pub struct MintParams;

pub struct MintPayload<'a, F: IChainForms> {
    pub context: &'a DeliveryContext<F, MintPurpose>,
    pub identity: AdmittedKemIdentity<'a>,
}

pub struct MintSuccessReturn<
    P: IPairingAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
> {
    pub delivery: MintDelivery<P, K, A, D>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MintErrorReturn {
    FillBytes(FillBytesErrorReturn),
    Setup(SetupErrorReturn),
    DeriveIdentity(DeriveIdentityErrorReturn),
    Issue(IssueErrorReturn),
    GenerateKeys(GenerateKeysErrorReturn),
    PublicKeysFromComponents(PublicKeysFromComponentsErrorReturn),
    WrapTo(WrapToErrorReturn),
    ProveMint(ProveMintErrorReturn),
}

pub type MintReturn<P, K, A, D> = Result<MintSuccessReturn<P, K, A, D>, MintErrorReturn>;

pub type MintFn<'a, P, K, A, D> = fn(
    &MintDeps<'a, K, A, D>,
    MintParams,
    MintPayload<'_, <D as IDeliveryProofAdapter>::Forms>,
) -> MintReturn<P, K, A, D>;

#[cfg(any(test, feature = "mocks"))]
pub trait IMintCaseConsumer {
    type Output;

    fn consume_mint_case<P, F, K, A, D>(
        &self,
        pairing: &P,
        kem: &K,
        key_agreement: &A,
        delivery_proof: &D,
        random: &dyn IRandomSourceAdapter,
        identity: AdmittedKemIdentity<'_>,
    ) -> Self::Output
    where
        P: IPairingArithmetic + IPairingReference,
        F: IChainForms,
        K: ICredentialKemAdapter<Pairing = P>,
        A: IKeyAgreementAdapter<Pairing = P>,
        D: IDeliveryProofAdapter<Pairing = P, Forms = F>;
}

#[cfg(any(test, feature = "mocks"))]
pub struct MintCaseDeps<C: IMintCaseConsumer> {
    pub consumer: C,
}

#[cfg(any(test, feature = "mocks"))]
pub struct MintCaseParams {
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
pub struct MintCasePayload<'a> {
    pub canonical: &'a [u8],
    pub identity_hash: &'a Root,
}

#[cfg(any(test, feature = "mocks"))]
pub struct MintCaseSuccessReturn<O> {
    pub output: O,
}

#[cfg(any(test, feature = "mocks"))]
#[derive(Debug, PartialEq, Eq)]
pub enum MintCaseErrorReturn {
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
pub type MintCaseReturn<O> = Result<MintCaseSuccessReturn<O>, MintCaseErrorReturn>;

#[cfg(any(test, feature = "mocks"))]
pub type RunMintCaseFn<C> = fn(
    &MintCaseDeps<C>,
    MintCaseParams,
    MintCasePayload<'_>,
) -> MintCaseReturn<<C as IMintCaseConsumer>::Output>;
