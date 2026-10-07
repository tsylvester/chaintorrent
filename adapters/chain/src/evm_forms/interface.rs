use core::convert::Infallible;
use encoding::CanonicalFieldKind;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EvmForms;

pub struct EvmFormsConstructorParams;

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
pub enum EvmFormFromFieldErrorReturn {
    WrongKind { expected: CanonicalFieldKind },
}
