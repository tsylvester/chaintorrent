use crate::mint_statement::provides::TransferPurpose;
use chain::IChainForms;
use core::convert::Infallible;
use core::marker::PhantomData;
use domain::{
    AssetIdentityHash, AssetIdentityHashTryNewErrorReturn, ParameterSetIdentifier,
    ParameterSetIdentifierTryNewErrorReturn, SuiteIdentifier, SuiteIdentifierTryNewErrorReturn,
};
use encoding::{CanonicalFieldKind, ICanonicalField};
use envelope::{EnvelopeComponents, PublicKeysComponents};
use pairing::{DecodeG1ErrorReturn, DecodeG2ErrorReturn, IPairingAdapter};

pub const TRANSFER_STATEMENT_FIELD_COUNT: usize = 32;

pub struct TransferFirstMessages<G1, G2> {
    pub pk1: G1,
    pub c1: G1,
    pub c2: G1,
    pub pk2: G2,
    pub d1: G2,
    pub d2: G2,
}

pub struct TransferStatement<P: IPairingAdapter, F: IChainForms> {
    pub suite_identifier: SuiteIdentifier,
    pub chain: F::ChainIdentifier,
    pub entitlement_contract: F::Identity,
    pub asset_identity_hash: AssetIdentityHash,
    pub parameter_set_identifier: ParameterSetIdentifier,
    pub source_entitlement: F::Entitlement,
    pub target_entitlement: F::Entitlement,
    pub old_interval: F::Interval,
    pub new_interval: F::Interval,
    pub purpose: TransferPurpose,
    pub seller: F::Identity,
    pub buyer: F::Identity,
    pub seller_keys: PublicKeysComponents<P::G1, P::G2>,
    pub buyer_keys: PublicKeysComponents<P::G1, P::G2>,
    pub old_envelope: EnvelopeComponents<P::G1, P::G2>,
    pub new_envelope: EnvelopeComponents<P::G1, P::G2>,
    pub expiry: u64,
    pub first_messages: TransferFirstMessages<P::G1, P::G2>,
}

pub struct TransferStatementDescription<'a, P: IPairingAdapter, F: IChainForms> {
    pub(super) pairing: &'a P,
    pub(super) forms: PhantomData<F>,
}

pub struct TransferStatementDescriptionConstructorParams<'a, P: IPairingAdapter> {
    pub pairing: &'a P,
}

pub type TransferStatementDescriptionTryNewReturn<'a, P, F> =
    Result<TransferStatementDescription<'a, P, F>, Infallible>;

#[derive(Debug, PartialEq, Eq)]
pub enum TransferStatementFromFieldsErrorReturn<F: IChainForms> {
    FieldCount {
        expected: usize,
        actual: usize,
    },
    FieldKind {
        index: usize,
        expected: CanonicalFieldKind,
    },
    SuiteIdentifier(SuiteIdentifierTryNewErrorReturn),
    AssetIdentityHash(AssetIdentityHashTryNewErrorReturn),
    ParameterSetIdentifier(ParameterSetIdentifierTryNewErrorReturn),
    PurposeCode {
        index: usize,
        code: u16,
    },
    InvalidG1 {
        index: usize,
        error: DecodeG1ErrorReturn,
    },
    InvalidG2 {
        index: usize,
        error: DecodeG2ErrorReturn,
    },
    ChainIdentifier {
        index: usize,
        error: <F::ChainIdentifier as ICanonicalField>::FromFieldErrorReturn,
    },
    Identity {
        index: usize,
        error: <F::Identity as ICanonicalField>::FromFieldErrorReturn,
    },
    Entitlement {
        index: usize,
        error: <F::Entitlement as ICanonicalField>::FromFieldErrorReturn,
    },
    Interval {
        index: usize,
        error: <F::Interval as ICanonicalField>::FromFieldErrorReturn,
    },
}
