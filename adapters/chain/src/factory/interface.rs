use core::convert::Infallible;

use domain::SignatureSchemeIdentifier;
use encoding::{CanonicalFieldKind, CanonicalFieldValue, EncodingIdentifier, ICanonicalField};

pub const CHAIN_FORMS_INTERFACE_VERSION: u32 = 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChainFormsIdentifier {
    EvmV1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IntentMemberNames {
    pub identity: &'static str,
    pub nonce: &'static str,
    pub expiry: &'static str,
    pub call: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChainFormsDeclaration {
    pub identifier: ChainFormsIdentifier,
    pub adapter_version: u32,
    pub interface_version: u32,
    pub encoding_identifier: EncodingIdentifier,
    pub key_identifier_domain_tag: &'static [u8],
    pub intent_domain_name: &'static str,
    pub intent_domain_version: &'static str,
    pub intent_struct_name: &'static str,
    pub intent_member_names: IntentMemberNames,
    pub nonce_kind: CanonicalFieldKind,
    pub expiry_kind: CanonicalFieldKind,
}

pub struct DeriveKeyIdentifierParams;

pub struct DeriveKeyIdentifierPayload<'a> {
    pub scheme: SignatureSchemeIdentifier,
    pub key: &'a [u8],
}

pub struct DeriveKeyIdentifierSuccessReturn<K> {
    pub key_identifier: K,
}

pub type DeriveKeyIdentifierReturn<K> = Result<DeriveKeyIdentifierSuccessReturn<K>, Infallible>;

pub trait IIdentityForm: ICanonicalField {
    type KeyIdentifier: ICanonicalField + Clone + core::fmt::Debug + PartialEq + Eq;

    fn derive_key_identifier(
        params: DeriveKeyIdentifierParams,
        payload: DeriveKeyIdentifierPayload<'_>,
    ) -> DeriveKeyIdentifierReturn<Self::KeyIdentifier>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntentField {
    pub name: String,
    pub value: CanonicalFieldValue,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntentCall {
    pub operation: String,
    pub fields: Vec<IntentField>,
}

pub struct SignedIntentParams;

pub struct SignedIntentPayload<'a, F: IChainForms> {
    pub chain: &'a F::ChainIdentifier,
    pub contract: &'a F::Identity,
    pub identity: &'a F::Identity,
    pub nonce: CanonicalFieldValue,
    pub expiry: CanonicalFieldValue,
    pub call: &'a IntentCall,
}

pub struct SignedIntentSuccessReturn<S> {
    pub signed_intent: S,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SignedIntentErrorReturn {
    OperationNotIdentifier {
        operation: String,
    },
    OperationIsStructName {
        operation: String,
    },
    FieldNotIdentifier {
        name: String,
    },
    FieldRepeated {
        name: String,
    },
    NonceKind {
        expected: CanonicalFieldKind,
        actual: CanonicalFieldKind,
    },
    ExpiryKind {
        expected: CanonicalFieldKind,
        actual: CanonicalFieldKind,
    },
}

pub type SignedIntentReturn<S> = Result<SignedIntentSuccessReturn<S>, SignedIntentErrorReturn>;

pub trait IChainForms: core::fmt::Debug + PartialEq + Eq {
    const DECLARATION: ChainFormsDeclaration;

    type Identity: IIdentityForm + Clone + core::fmt::Debug + PartialEq + Eq;
    type Entitlement: ICanonicalField + Clone + core::fmt::Debug + PartialEq + Eq;
    type Interval: ICanonicalField + Clone + core::fmt::Debug + PartialEq + Eq;
    type ChainIdentifier: ICanonicalField + Clone + core::fmt::Debug + PartialEq + Eq;
    type SignedIntent: ICanonicalField + Clone + core::fmt::Debug + PartialEq + Eq;

    fn signed_intent(
        &self,
        params: SignedIntentParams,
        payload: SignedIntentPayload<'_, Self>,
    ) -> SignedIntentReturn<Self::SignedIntent>
    where
        Self: Sized;
}
