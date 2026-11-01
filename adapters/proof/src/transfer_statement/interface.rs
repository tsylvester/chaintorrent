use crate::mint_statement::provides::TransferPurpose;
use chain::IChainForms;
#[cfg(any(test, feature = "mocks"))]
use chain::{CreateChainFormsErrorReturn, CreateChainFormsParams};
use core::convert::Infallible;
use core::marker::PhantomData;
use domain::{
    ParameterSetIdentifier, ParameterSetIdentifierTryNewErrorReturn, SuiteIdentifier,
    SuiteIdentifierTryNewErrorReturn,
};
use encoding::{CanonicalFieldKind, FromFieldErrorReturn};
#[cfg(any(test, feature = "mocks"))]
use encoding::{CreateEncodingErrorReturn, CreateEncodingParams, IDecoderAdapter, IEncoderAdapter};
use envelope::{EnvelopeComponents, PublicKeysComponents};
use hashing::{Root, RootTryNewErrorReturn};
#[cfg(any(test, feature = "mocks"))]
use pairing::{
    CreatePairingErrorReturn, CreatePairingParams, IPairingArithmetic, IPairingReference,
};
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
    pub asset_root: Root,
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
pub enum TransferStatementFromFieldsErrorReturn {
    FieldCount {
        expected: usize,
        actual: usize,
    },
    FieldKind {
        index: usize,
        expected: CanonicalFieldKind,
    },
    SuiteIdentifier(SuiteIdentifierTryNewErrorReturn),
    AssetRoot(RootTryNewErrorReturn),
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
        error: FromFieldErrorReturn,
    },
    Identity {
        index: usize,
        error: FromFieldErrorReturn,
    },
    Entitlement {
        index: usize,
        error: FromFieldErrorReturn,
    },
    Interval {
        index: usize,
        error: FromFieldErrorReturn,
    },
}

#[cfg(any(test, feature = "mocks"))]
pub trait ITransferStatementCaseConsumer {
    type Output;

    fn consume_transfer_statement_case<
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter + IDecoderAdapter,
        F: IChainForms,
    >(
        &self,
        pairing: &P,
        encoder: &E,
        forms: &F,
        payload: &TransferStatementCasePayload,
    ) -> Self::Output;
}

#[cfg(any(test, feature = "mocks"))]
pub struct TransferStatementCaseDeps<C: ITransferStatementCaseConsumer> {
    pub consumer: C,
}

#[cfg(any(test, feature = "mocks"))]
pub struct TransferStatementCaseParams {
    pub pairing: CreatePairingParams,
    pub encoding: CreateEncodingParams,
    pub chain_forms: CreateChainFormsParams,
}

#[cfg(any(test, feature = "mocks"))]
pub struct TransferStatementCasePayload;

#[cfg(any(test, feature = "mocks"))]
pub struct TransferStatementCaseSuccessReturn<O> {
    pub output: O,
}

#[cfg(any(test, feature = "mocks"))]
#[derive(Debug, PartialEq, Eq)]
pub enum TransferStatementCaseErrorReturn {
    CreatePairing(CreatePairingErrorReturn),
    CreateEncoding(CreateEncodingErrorReturn),
    CreateChainForms(CreateChainFormsErrorReturn),
}

#[cfg(any(test, feature = "mocks"))]
pub type TransferStatementCaseReturn<O> =
    Result<TransferStatementCaseSuccessReturn<O>, TransferStatementCaseErrorReturn>;

#[cfg(any(test, feature = "mocks"))]
pub type RunTransferStatementCaseFn<C> =
    fn(
        &TransferStatementCaseDeps<C>,
        TransferStatementCaseParams,
        TransferStatementCasePayload,
    ) -> TransferStatementCaseReturn<<C as ITransferStatementCaseConsumer>::Output>;
