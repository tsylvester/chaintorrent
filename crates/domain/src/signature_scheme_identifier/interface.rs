#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SignatureSchemeIdentifier {
    pub(super) identifier: u8,
}

pub struct SignatureSchemeIdentifierConstructorParams {
    pub identifier: u8,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SignatureSchemeIdentifierTryNewErrorReturn {
    ZeroIdentifier,
}

pub type SignatureSchemeIdentifierTryNewReturn =
    Result<SignatureSchemeIdentifier, SignatureSchemeIdentifierTryNewErrorReturn>;
