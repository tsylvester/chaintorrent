use chain::IChainForms;
use core::convert::Infallible;
use core::marker::PhantomData;
use domain::AssetIdentityHashTryNewErrorReturn;
use encoding::{CanonicalFieldKind, CanonicalFieldValue, FromFieldErrorReturn};
use envelope::{
    GenerateKeysErrorReturn, IKeyAgreementAdapter, PublicKeysFromComponentsErrorReturn,
    WrapToErrorReturn,
};
use kem::{
    DeriveIdentityErrorReturn, ICredentialKemAdapter, IdentityScope, IssueErrorReturn,
    RerandomizeErrorReturn, SetupErrorReturn,
};
use pairing::IPairingAdapter;
use proof::{
    DeliveryRelation, IDeliveryProofAdapter, MintPurpose, ProveMintErrorReturn,
    ProveTransferErrorReturn, TransferPurpose,
};
use random::{FillBytesErrorReturn, IRandomSourceAdapter};

use super::super::challenge_vectors::provides::{CanonicalDeliveryContext, DeliveryContextField};
use super::super::group_vectors::provides::VectorCount;
use super::super::render::provides::{
    SolidityMemberNameTryNewErrorReturn, SolidityRecordMembers,
    SolidityRecordMembersTryNewErrorReturn,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProofVectorContext {
    pub suite_identifier: [u8; 32],
    pub suite_version: u16,
    pub chain: CanonicalFieldValue,
    pub entitlement_contract: CanonicalFieldValue,
    pub asset_identity_hash: [u8; 32],
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
    "assetIdentityHash",
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
    "assetIdentityHash",
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
    FieldCount {
        expected: usize,
        actual: usize,
    },
    FieldKind {
        index: usize,
        expected: CanonicalFieldKind,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransferProofVectorFromFieldsErrorReturn {
    FieldCount {
        expected: usize,
        actual: usize,
    },
    FieldKind {
        index: usize,
        expected: CanonicalFieldKind,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProofVectorMembersErrorReturn {
    MemberName(SolidityMemberNameTryNewErrorReturn),
    RecordMembers(SolidityRecordMembersTryNewErrorReturn),
}

pub type ProofVectorMembersReturn = Result<SolidityRecordMembers, ProofVectorMembersErrorReturn>;

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
    pub scope: IdentityScope,
}

pub struct ProofVectorsPayload<'a> {
    pub mint: &'a CanonicalDeliveryContext,
    pub mint_purpose: MintPurpose,
    pub transfer: &'a CanonicalDeliveryContext,
    pub transfer_purpose: TransferPurpose,
    pub identity: &'a [u8],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProofVectorsSuccessReturn {
    pub mint: Vec<MintProofVector>,
    pub transfer: Vec<TransferProofVector>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ProofVectorsErrorReturn {
    ChainIdentifier {
        relation: DeliveryRelation,
        error: FromFieldErrorReturn,
    },
    Identity {
        relation: DeliveryRelation,
        field: DeliveryContextField,
        error: FromFieldErrorReturn,
    },
    Entitlement {
        relation: DeliveryRelation,
        field: DeliveryContextField,
        error: FromFieldErrorReturn,
    },
    Interval {
        relation: DeliveryRelation,
        field: DeliveryContextField,
        error: FromFieldErrorReturn,
    },
    AssetIdentityLength {
        actual: usize,
    },
    AssetIdentityHash(AssetIdentityHashTryNewErrorReturn),
    FillBytes(FillBytesErrorReturn),
    Setup(SetupErrorReturn),
    DeriveIdentity(DeriveIdentityErrorReturn),
    Issue(IssueErrorReturn),
    GenerateKeys(GenerateKeysErrorReturn),
    PublicKeysFromComponents(PublicKeysFromComponentsErrorReturn),
    WrapTo(WrapToErrorReturn),
    Rerandomize(RerandomizeErrorReturn),
    ProveMint(ProveMintErrorReturn),
    ProveTransfer(ProveTransferErrorReturn),
    ProofComponents {
        relation: DeliveryRelation,
    },
    ScalarLength {
        actual: usize,
    },
}

pub type ProofVectorsReturn = Result<ProofVectorsSuccessReturn, ProofVectorsErrorReturn>;

pub type ProofVectorsFn<'a, P, K, A, D> = fn(
    &ProofVectorsDeps<'a, P, K, A, D>,
    ProofVectorsParams,
    ProofVectorsPayload<'_>,
) -> ProofVectorsReturn;
