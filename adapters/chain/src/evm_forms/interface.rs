use core::convert::Infallible;

use encoding::{EncodingTypeName, IEncoderAdapter};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EvmFormsTypeNames {
    pub(super) fixed_bytes_32: EncodingTypeName,
    pub(super) unsigned_16: EncodingTypeName,
    pub(super) unsigned_32: EncodingTypeName,
    pub(super) unsigned_64: EncodingTypeName,
    pub(super) text: EncodingTypeName,
    pub(super) bytes: EncodingTypeName,
    pub(super) fixed_bytes_20: EncodingTypeName,
    pub(super) unsigned_256: EncodingTypeName,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EvmForms {
    pub(super) type_names: EvmFormsTypeNames,
}

pub struct EvmFormsConstructorParams<'a, E: IEncoderAdapter> {
    pub encoder: &'a E,
}

pub type EvmFormsTryNewReturn = Result<EvmForms, Infallible>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EvmIdentityForm {
    pub(super) bytes: [u8; 20],
}

pub struct EvmIdentityFormConstructorParams {
    pub bytes: [u8; 20],
}

pub type EvmIdentityFormTryNewReturn = Result<EvmIdentityForm, Infallible>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EvmEntitlementForm {
    pub(super) bytes: [u8; 32],
}

pub struct EvmEntitlementFormConstructorParams {
    pub bytes: [u8; 32],
}

pub type EvmEntitlementFormTryNewReturn = Result<EvmEntitlementForm, Infallible>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EvmIntervalForm {
    pub(super) value: u64,
}

pub struct EvmIntervalFormConstructorParams {
    pub value: u64,
}

pub type EvmIntervalFormTryNewReturn = Result<EvmIntervalForm, Infallible>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EvmChainIdentifierForm {
    pub(super) bytes: [u8; 32],
}

pub struct EvmChainIdentifierFormConstructorParams {
    pub bytes: [u8; 32],
}

pub type EvmChainIdentifierFormTryNewReturn = Result<EvmChainIdentifierForm, Infallible>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EvmKeyIdentifierForm {
    pub(super) bytes: [u8; 32],
}

pub struct EvmKeyIdentifierFormConstructorParams {
    pub bytes: [u8; 32],
}

pub type EvmKeyIdentifierFormTryNewReturn = Result<EvmKeyIdentifierForm, Infallible>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EvmSignedIntentForm {
    pub(super) bytes: [u8; 32],
}

pub struct EvmSignedIntentFormConstructorParams {
    pub bytes: [u8; 32],
}

pub type EvmSignedIntentFormTryNewReturn = Result<EvmSignedIntentForm, Infallible>;

pub(super) const EIP712_DOMAIN_TYPE: &str =
    "EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)";

pub(super) const EIP712_PREFIX: [u8; 2] = [0x19, 0x01];
