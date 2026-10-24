use chain::IChainForms;
use domain::Secret;
use envelope::{
    EnvelopeCoins, GenerateKeysErrorReturn, IKeyAgreementAdapter,
    PublicKeysFromComponentsErrorReturn, WrapToErrorReturn,
};
use kem::{
    DeriveIdentityErrorReturn, ICredentialKemAdapter, IssueErrorReturn, KemIdentity,
    MasterScalarComponents, SetupErrorReturn, SetupScope,
};
use pairing::IPairingAdapter;
use proof::{
    AlgebraicMintStatement, DeliveryContext, IDeliveryProofAdapter, MintPurpose,
    ProveMintErrorReturn,
};
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
