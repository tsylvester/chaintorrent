pub const PARAMETER_SET_IDENTIFIER_LENGTH: usize = 32;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParameterSetIdentifier {
    pub(super) bytes: [u8; PARAMETER_SET_IDENTIFIER_LENGTH],
}

pub struct ParameterSetIdentifierConstructorParams {
    pub bytes: [u8; PARAMETER_SET_IDENTIFIER_LENGTH],
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParameterSetIdentifierTryNewErrorReturn {
    AllZero,
}

pub type ParameterSetIdentifierTryNewReturn =
    Result<ParameterSetIdentifier, ParameterSetIdentifierTryNewErrorReturn>;
