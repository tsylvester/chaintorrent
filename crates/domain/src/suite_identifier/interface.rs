pub const SUITE_IDENTIFIER_LENGTH: usize = 32;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SuiteIdentifier {
    pub(super) identifier: [u8; SUITE_IDENTIFIER_LENGTH],
    pub(super) version: u16,
}

pub struct SuiteIdentifierConstructorParams {
    pub identifier: [u8; SUITE_IDENTIFIER_LENGTH],
    pub version: u16,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SuiteIdentifierTryNewErrorReturn {
    AllZeroIdentifier,
    ZeroVersion,
}

pub type SuiteIdentifierTryNewReturn = Result<SuiteIdentifier, SuiteIdentifierTryNewErrorReturn>;
