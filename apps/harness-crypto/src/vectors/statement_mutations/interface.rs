use chain::IChainForms;
#[cfg(any(test, feature = "mocks"))]
use chain::{CreateChainFormsErrorReturn, CreateChainFormsParams};
use domain::{ParameterSetIdentifierTryNewErrorReturn, SuiteIdentifierTryNewErrorReturn};
use encoding::FromFieldErrorReturn;
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
use hashing::RootTryNewErrorReturn;
use kem::ICredentialKemAdapter;
#[cfg(any(test, feature = "mocks"))]
use kem::{CreateKemErrorReturn, CreateKemParams};
#[cfg(any(test, feature = "mocks"))]
use pairing::{CreatePairingErrorReturn, CreatePairingParams, IPairingReference};
use pairing::{IPairingAdapter, IPairingArithmetic, SampleUniformScalarErrorReturn};
use proof::{
    AlgebraicMintStatement, AlgebraicTransferStatement, DeliveryContext, DeliveryRelation,
    IDeliveryProofAdapter, MintPurpose, ProofFromComponentsErrorReturn, TransferPurpose,
    VerifyErrorReturn,
};
#[cfg(any(test, feature = "mocks"))]
use proof::{CreateDeliveryProofErrorReturn, CreateDeliveryProofParams};
#[cfg(any(test, feature = "mocks"))]
use random::{CreateRandomSourceErrorReturn, CreateRandomSourceParams};
use random::{FillBytesErrorReturn, IRandomSourceAdapter};

use crate::delivery::provides::{AdmittedKemIdentity, MintErrorReturn, TransferErrorReturn};
use crate::{DeliveryContextField, VectorCount};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MintMutationField {
    SuiteIdentifier,
    Chain,
    EntitlementContract,
    AssetRoot,
    ParameterSetIdentifier,
    SourceEntitlement,
    TargetEntitlement,
    OldInterval,
    NewInterval,
    Purpose,
    Seller,
    Buyer,
    Expiry,
    Hpub,
    IdentityElement,
    BuyerPk1,
    BuyerPk2,
    EnvelopeC1,
    EnvelopeC2,
    EnvelopeD1,
    EnvelopeD2,
    Challenge,
    Alpha,
    R,
    Rho,
    Sigma,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransferMutationField {
    SuiteIdentifier,
    Chain,
    EntitlementContract,
    AssetRoot,
    ParameterSetIdentifier,
    SourceEntitlement,
    TargetEntitlement,
    OldInterval,
    NewInterval,
    Purpose,
    Seller,
    Buyer,
    Expiry,
    IdentityElement,
    SellerPk1,
    SellerPk2,
    BuyerPk1,
    BuyerPk2,
    OldEnvelopeC1,
    OldEnvelopeC2,
    OldEnvelopeD1,
    OldEnvelopeD2,
    NewEnvelopeC1,
    NewEnvelopeC2,
    NewEnvelopeD1,
    NewEnvelopeD2,
    Challenge,
    X,
    Y,
    S,
    Rho,
    Sigma,
}

pub const MINT_MUTATION_FIELDS: [MintMutationField; 26] = [
    MintMutationField::SuiteIdentifier,
    MintMutationField::Chain,
    MintMutationField::EntitlementContract,
    MintMutationField::AssetRoot,
    MintMutationField::ParameterSetIdentifier,
    MintMutationField::SourceEntitlement,
    MintMutationField::TargetEntitlement,
    MintMutationField::OldInterval,
    MintMutationField::NewInterval,
    MintMutationField::Purpose,
    MintMutationField::Seller,
    MintMutationField::Buyer,
    MintMutationField::Expiry,
    MintMutationField::Hpub,
    MintMutationField::IdentityElement,
    MintMutationField::BuyerPk1,
    MintMutationField::BuyerPk2,
    MintMutationField::EnvelopeC1,
    MintMutationField::EnvelopeC2,
    MintMutationField::EnvelopeD1,
    MintMutationField::EnvelopeD2,
    MintMutationField::Challenge,
    MintMutationField::Alpha,
    MintMutationField::R,
    MintMutationField::Rho,
    MintMutationField::Sigma,
];

pub const TRANSFER_MUTATION_FIELDS: [TransferMutationField; 32] = [
    TransferMutationField::SuiteIdentifier,
    TransferMutationField::Chain,
    TransferMutationField::EntitlementContract,
    TransferMutationField::AssetRoot,
    TransferMutationField::ParameterSetIdentifier,
    TransferMutationField::SourceEntitlement,
    TransferMutationField::TargetEntitlement,
    TransferMutationField::OldInterval,
    TransferMutationField::NewInterval,
    TransferMutationField::Purpose,
    TransferMutationField::Seller,
    TransferMutationField::Buyer,
    TransferMutationField::Expiry,
    TransferMutationField::IdentityElement,
    TransferMutationField::SellerPk1,
    TransferMutationField::SellerPk2,
    TransferMutationField::BuyerPk1,
    TransferMutationField::BuyerPk2,
    TransferMutationField::OldEnvelopeC1,
    TransferMutationField::OldEnvelopeC2,
    TransferMutationField::OldEnvelopeD1,
    TransferMutationField::OldEnvelopeD2,
    TransferMutationField::NewEnvelopeC1,
    TransferMutationField::NewEnvelopeC2,
    TransferMutationField::NewEnvelopeD1,
    TransferMutationField::NewEnvelopeD2,
    TransferMutationField::Challenge,
    TransferMutationField::X,
    TransferMutationField::Y,
    TransferMutationField::S,
    TransferMutationField::Rho,
    TransferMutationField::Sigma,
];

pub struct MintMutation<P: IPairingAdapter, D: IDeliveryProofAdapter<Pairing = P>> {
    pub field: MintMutationField,
    pub statement: AlgebraicMintStatement<D::Forms, P::G1, P::G2>,
    pub proof: D::Proof,
    pub is_valid: bool,
}

pub struct TransferMutation<P: IPairingAdapter, D: IDeliveryProofAdapter<Pairing = P>> {
    pub field: TransferMutationField,
    pub statement: AlgebraicTransferStatement<D::Forms, P::G1, P::G2>,
    pub proof: D::Proof,
    pub is_valid: bool,
}

pub struct StatementMutationsVector<P: IPairingAdapter, D: IDeliveryProofAdapter<Pairing = P>> {
    pub draws: Vec<u8>,
    pub mint_statement: AlgebraicMintStatement<D::Forms, P::G1, P::G2>,
    pub mint_proof: D::Proof,
    pub mint_is_valid: bool,
    pub mint_mutations: Vec<MintMutation<P, D>>,
    pub transfer_statement: AlgebraicTransferStatement<D::Forms, P::G1, P::G2>,
    pub transfer_proof: D::Proof,
    pub transfer_is_valid: bool,
    pub transfer_mutations: Vec<TransferMutation<P, D>>,
}

pub struct StatementMutationsDeps<
    'a,
    P: IPairingArithmetic,
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
pub struct StatementMutationsParams {
    pub count: VectorCount,
}

pub struct StatementMutationsPayload<'a, F: IChainForms> {
    pub mint: &'a DeliveryContext<F, MintPurpose>,
    pub transfer: &'a DeliveryContext<F, TransferPurpose>,
    pub identity: AdmittedKemIdentity<'a>,
}

pub struct StatementMutationsSuccessReturn<
    P: IPairingAdapter,
    D: IDeliveryProofAdapter<Pairing = P>,
> {
    pub vectors: Vec<StatementMutationsVector<P, D>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum StatementMutationsErrorReturn {
    Mint(MintErrorReturn),
    Transfer(TransferErrorReturn),
    FillBytes(FillBytesErrorReturn),
    Verify {
        relation: DeliveryRelation,
        error: VerifyErrorReturn,
    },
    OffsetSampling(SampleUniformScalarErrorReturn),
    MutatedChainIdentifier {
        relation: DeliveryRelation,
        error: FromFieldErrorReturn,
    },
    MutatedIdentity {
        relation: DeliveryRelation,
        field: DeliveryContextField,
        error: FromFieldErrorReturn,
    },
    MutatedEntitlement {
        relation: DeliveryRelation,
        field: DeliveryContextField,
        error: FromFieldErrorReturn,
    },
    MutatedInterval {
        relation: DeliveryRelation,
        field: DeliveryContextField,
        error: FromFieldErrorReturn,
    },
    MutatedSuiteIdentifier {
        relation: DeliveryRelation,
        error: SuiteIdentifierTryNewErrorReturn,
    },
    MutatedAssetRoot {
        relation: DeliveryRelation,
        error: RootTryNewErrorReturn,
    },
    MutatedParameterSetIdentifier {
        relation: DeliveryRelation,
        error: ParameterSetIdentifierTryNewErrorReturn,
    },
    ProofComponents {
        relation: DeliveryRelation,
    },
    ProofFromComponents {
        relation: DeliveryRelation,
        error: ProofFromComponentsErrorReturn,
    },
}

pub type StatementMutationsReturn<P, D> =
    Result<StatementMutationsSuccessReturn<P, D>, StatementMutationsErrorReturn>;

pub type StatementMutationsFn<'a, P, K, A, D> = fn(
    &StatementMutationsDeps<'a, P, K, A, D>,
    StatementMutationsParams,
    StatementMutationsPayload<'_, <D as IDeliveryProofAdapter>::Forms>,
) -> StatementMutationsReturn<P, D>;

#[cfg(any(test, feature = "mocks"))]
pub trait IStatementMutationsCaseConsumer {
    type Output;

    fn consume_statement_mutations_case<P, H, E, F, K, A, D>(
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
        identity: AdmittedKemIdentity<'_>,
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
pub struct StatementMutationsCaseDeps<C: IStatementMutationsCaseConsumer> {
    pub consumer: C,
}

#[cfg(any(test, feature = "mocks"))]
pub struct StatementMutationsCaseParams {
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
pub struct StatementMutationsCasePayload<'a> {
    pub canonical: &'a [u8],
    pub root: &'a Root,
}

#[cfg(any(test, feature = "mocks"))]
pub struct StatementMutationsCaseSuccessReturn<O> {
    pub output: O,
}

#[cfg(any(test, feature = "mocks"))]
#[derive(Debug, PartialEq, Eq)]
pub enum StatementMutationsCaseErrorReturn {
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
pub type StatementMutationsCaseReturn<O> =
    Result<StatementMutationsCaseSuccessReturn<O>, StatementMutationsCaseErrorReturn>;

#[cfg(any(test, feature = "mocks"))]
pub type RunStatementMutationsCaseFn<C> =
    fn(
        &StatementMutationsCaseDeps<C>,
        StatementMutationsCaseParams,
        StatementMutationsCasePayload<'_>,
    ) -> StatementMutationsCaseReturn<<C as IStatementMutationsCaseConsumer>::Output>;
