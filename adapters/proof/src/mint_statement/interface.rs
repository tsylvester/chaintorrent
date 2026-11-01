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

pub const DELIVERY_STATEMENT_VERSION_ONE: u16 = 1;

pub const MINT_STATEMENT_FIELD_COUNT: usize = 25;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeliveryRelation {
    Mint,
    Transfer,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeliveryPurpose {
    Mint,
    Transfer,
    Grant,
    Replacement,
}

impl DeliveryPurpose {
    pub fn code(&self) -> u16 {
        match self {
            DeliveryPurpose::Mint => 1,
            DeliveryPurpose::Transfer => 2,
            DeliveryPurpose::Grant => 3,
            DeliveryPurpose::Replacement => 4,
        }
    }

    pub fn relation(&self) -> DeliveryRelation {
        match self {
            DeliveryPurpose::Mint | DeliveryPurpose::Replacement => DeliveryRelation::Mint,
            DeliveryPurpose::Transfer | DeliveryPurpose::Grant => DeliveryRelation::Transfer,
        }
    }
}

pub const DELIVERY_PURPOSES: [DeliveryPurpose; 4] = [
    DeliveryPurpose::Mint,
    DeliveryPurpose::Transfer,
    DeliveryPurpose::Grant,
    DeliveryPurpose::Replacement,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UndeclaredDeliveryPurposeCode {
    pub code: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MintPurpose {
    Mint,
    Replacement,
}

impl MintPurpose {
    pub fn code(&self) -> u16 {
        match self {
            MintPurpose::Mint => DeliveryPurpose::Mint.code(),
            MintPurpose::Replacement => DeliveryPurpose::Replacement.code(),
        }
    }
}

impl TryFrom<u16> for MintPurpose {
    type Error = UndeclaredDeliveryPurposeCode;

    fn try_from(code: u16) -> Result<Self, Self::Error> {
        match code {
            1 => Ok(MintPurpose::Mint),
            4 => Ok(MintPurpose::Replacement),
            _ => Err(UndeclaredDeliveryPurposeCode { code }),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransferPurpose {
    Transfer,
    Grant,
}

impl TransferPurpose {
    pub fn code(&self) -> u16 {
        match self {
            TransferPurpose::Transfer => DeliveryPurpose::Transfer.code(),
            TransferPurpose::Grant => DeliveryPurpose::Grant.code(),
        }
    }
}

impl TryFrom<u16> for TransferPurpose {
    type Error = UndeclaredDeliveryPurposeCode;

    fn try_from(code: u16) -> Result<Self, Self::Error> {
        match code {
            2 => Ok(TransferPurpose::Transfer),
            3 => Ok(TransferPurpose::Grant),
            _ => Err(UndeclaredDeliveryPurposeCode { code }),
        }
    }
}

pub const DELIVERY_PURPOSE_MINT: MintPurpose = MintPurpose::Mint;
pub const DELIVERY_PURPOSE_TRANSFER: TransferPurpose = TransferPurpose::Transfer;
pub const DELIVERY_PURPOSE_GRANT: TransferPurpose = TransferPurpose::Grant;
pub const DELIVERY_PURPOSE_REPLACEMENT: MintPurpose = MintPurpose::Replacement;

pub struct MintFirstMessages<G1, G2> {
    pub hpub: G2,
    pub c1: G1,
    pub c2: G1,
    pub d1: G2,
    pub d2: G2,
}

pub struct MintStatement<P: IPairingAdapter, F: IChainForms> {
    pub suite_identifier: SuiteIdentifier,
    pub chain: F::ChainIdentifier,
    pub entitlement_contract: F::Identity,
    pub asset_root: Root,
    pub parameter_set_identifier: ParameterSetIdentifier,
    pub source_entitlement: F::Entitlement,
    pub target_entitlement: F::Entitlement,
    pub old_interval: F::Interval,
    pub new_interval: F::Interval,
    pub purpose: MintPurpose,
    pub seller: F::Identity,
    pub buyer: F::Identity,
    pub buyer_keys: PublicKeysComponents<P::G1, P::G2>,
    pub envelope: EnvelopeComponents<P::G1, P::G2>,
    pub expiry: u64,
    pub first_messages: MintFirstMessages<P::G1, P::G2>,
}

pub struct MintStatementDescription<'a, P: IPairingAdapter, F: IChainForms> {
    pub(super) pairing: &'a P,
    pub(super) forms: PhantomData<F>,
}

pub struct MintStatementDescriptionConstructorParams<'a, P: IPairingAdapter> {
    pub pairing: &'a P,
}

pub type MintStatementDescriptionTryNewReturn<'a, P, F> =
    Result<MintStatementDescription<'a, P, F>, Infallible>;

#[derive(Debug, PartialEq, Eq)]
pub enum MintStatementFromFieldsErrorReturn {
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
pub trait IMintStatementCaseConsumer {
    type Output;

    fn consume_mint_statement_case<
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter + IDecoderAdapter,
        F: IChainForms,
    >(
        &self,
        pairing: &P,
        encoder: &E,
        forms: &F,
        payload: &MintStatementCasePayload,
    ) -> Self::Output;
}

#[cfg(any(test, feature = "mocks"))]
pub struct MintStatementCaseDeps<C: IMintStatementCaseConsumer> {
    pub consumer: C,
}

#[cfg(any(test, feature = "mocks"))]
pub struct MintStatementCaseParams {
    pub pairing: CreatePairingParams,
    pub encoding: CreateEncodingParams,
    pub chain_forms: CreateChainFormsParams,
}

#[cfg(any(test, feature = "mocks"))]
pub struct MintStatementCasePayload;

#[cfg(any(test, feature = "mocks"))]
pub struct MintStatementCaseSuccessReturn<O> {
    pub output: O,
}

#[cfg(any(test, feature = "mocks"))]
#[derive(Debug, PartialEq, Eq)]
pub enum MintStatementCaseErrorReturn {
    CreatePairing(CreatePairingErrorReturn),
    CreateEncoding(CreateEncodingErrorReturn),
    CreateChainForms(CreateChainFormsErrorReturn),
}

#[cfg(any(test, feature = "mocks"))]
pub type MintStatementCaseReturn<O> =
    Result<MintStatementCaseSuccessReturn<O>, MintStatementCaseErrorReturn>;

#[cfg(any(test, feature = "mocks"))]
pub type RunMintStatementCaseFn<C> =
    fn(
        &MintStatementCaseDeps<C>,
        MintStatementCaseParams,
        MintStatementCasePayload,
    ) -> MintStatementCaseReturn<<C as IMintStatementCaseConsumer>::Output>;
