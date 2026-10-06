use core::convert::Infallible;
use encoding::CanonicalFieldKind;
use pairing::{DecodeG1ErrorReturn, DecodeG2ErrorReturn, IPairingAdapter};

pub const POSSESSION_STATEMENT_FIELD_COUNT: usize = 2;

pub const POSSESSION_STATEMENT_FIELD_KINDS: [CanonicalFieldKind; POSSESSION_STATEMENT_FIELD_COUNT] =
    [CanonicalFieldKind::Bytes, CanonicalFieldKind::Bytes];

pub struct PossessionG1Statement<P: IPairingAdapter> {
    pub key: P::G1,
    pub commitment: P::G1,
}

pub struct PossessionG2Statement<P: IPairingAdapter> {
    pub key: P::G2,
    pub commitment: P::G2,
}

pub struct PossessionG1StatementDescription<'a, P: IPairingAdapter> {
    pub(super) pairing: &'a P,
}

pub struct PossessionG2StatementDescription<'a, P: IPairingAdapter> {
    pub(super) pairing: &'a P,
}

pub struct PossessionG1StatementDescriptionConstructorParams<'a, P: IPairingAdapter> {
    pub pairing: &'a P,
}

pub struct PossessionG2StatementDescriptionConstructorParams<'a, P: IPairingAdapter> {
    pub pairing: &'a P,
}

pub type PossessionG1StatementDescriptionTryNewReturn<'a, P> =
    Result<PossessionG1StatementDescription<'a, P>, Infallible>;

pub type PossessionG2StatementDescriptionTryNewReturn<'a, P> =
    Result<PossessionG2StatementDescription<'a, P>, Infallible>;

#[derive(Debug, PartialEq, Eq)]
pub enum PossessionStatementFromFieldsErrorReturn {
    FieldCount {
        expected: usize,
        actual: usize,
    },
    FieldKind {
        index: usize,
        expected: CanonicalFieldKind,
    },
    InvalidG1 {
        index: usize,
        error: DecodeG1ErrorReturn,
    },
    InvalidG2 {
        index: usize,
        error: DecodeG2ErrorReturn,
    },
}
