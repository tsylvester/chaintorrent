#![allow(dead_code, clippy::too_many_arguments)]

use super::interface::{
    CHAIN_FORMS_INTERFACE_VERSION, ChainFormsDeclaration, ChainFormsIdentifier,
    DeriveKeyIdentifierParams, DeriveKeyIdentifierPayload, DeriveKeyIdentifierReturn,
    DeriveKeyIdentifierSuccessReturn, IChainForms, IIdentityForm, IntentMemberNames,
    SignedIntentParams, SignedIntentPayload, SignedIntentReturn, SignedIntentSuccessReturn,
};
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, EncodingIdentifier, FromFieldErrorReturn,
    FromFieldParams, FromFieldReturn, FromFieldSuccessReturnOverrides, ICanonicalField,
    ToFieldParams, ToFieldReturn, ToFieldSuccessReturnOverrides, build_from_field_success_return,
    build_to_field_success_return,
};

#[derive(Default)]
pub struct ChainFormsDeclarationOverrides {
    pub identifier: Option<ChainFormsIdentifier>,
    pub adapter_version: Option<u32>,
    pub interface_version: Option<u32>,
    pub encoding_identifier: Option<EncodingIdentifier>,
    pub key_identifier_domain_tag: Option<&'static [u8]>,
    pub intent_domain_name: Option<&'static str>,
    pub intent_domain_version: Option<&'static str>,
    pub intent_struct_name: Option<&'static str>,
    pub intent_member_names: Option<IntentMemberNames>,
    pub nonce_kind: Option<CanonicalFieldKind>,
    pub expiry_kind: Option<CanonicalFieldKind>,
}

pub fn build_chain_forms_declaration(
    overrides: ChainFormsDeclarationOverrides,
) -> ChainFormsDeclaration {
    ChainFormsDeclaration {
        identifier: overrides.identifier.unwrap_or(ChainFormsIdentifier::EvmV1),
        adapter_version: overrides.adapter_version.unwrap_or(1),
        interface_version: overrides
            .interface_version
            .unwrap_or(CHAIN_FORMS_INTERFACE_VERSION),
        encoding_identifier: overrides
            .encoding_identifier
            .unwrap_or(EncodingIdentifier::EthereumAbiV1),
        key_identifier_domain_tag: overrides
            .key_identifier_domain_tag
            .unwrap_or(b"chaintorrent/evm/key-identifier/v1"),
        intent_domain_name: overrides.intent_domain_name.unwrap_or("ChainTorrent"),
        intent_domain_version: overrides.intent_domain_version.unwrap_or("1"),
        intent_struct_name: overrides.intent_struct_name.unwrap_or("Intent"),
        intent_member_names: overrides.intent_member_names.unwrap_or(IntentMemberNames {
            identity: "identity",
            nonce: "nonce",
            expiry: "expiry",
            call: "call",
        }),
        nonce_kind: overrides
            .nonce_kind
            .unwrap_or(CanonicalFieldKind::Unsigned64),
        expiry_kind: overrides
            .expiry_kind
            .unwrap_or(CanonicalFieldKind::Unsigned64),
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct MockIChainForms;

impl IChainForms for MockIChainForms {
    const DECLARATION: ChainFormsDeclaration = ChainFormsDeclaration {
        identifier: ChainFormsIdentifier::EvmV1,
        adapter_version: 1,
        interface_version: CHAIN_FORMS_INTERFACE_VERSION,
        encoding_identifier: EncodingIdentifier::Mock,
        key_identifier_domain_tag: b"chaintorrent/evm/key-identifier/v1",
        intent_domain_name: "ChainTorrent",
        intent_domain_version: "1",
        intent_struct_name: "Intent",
        intent_member_names: IntentMemberNames {
            identity: "identity",
            nonce: "nonce",
            expiry: "expiry",
            call: "call",
        },
        nonce_kind: CanonicalFieldKind::Unsigned64,
        expiry_kind: CanonicalFieldKind::Unsigned64,
    };

    type Identity = MockIdentity;
    type Entitlement = MockEntitlement;
    type Interval = MockInterval;
    type ChainIdentifier = MockChainIdentifier;
    type SignedIntent = MockSignedIntent;

    fn signed_intent(
        &self,
        _params: SignedIntentParams,
        _payload: SignedIntentPayload<'_, Self>,
    ) -> SignedIntentReturn<Self::SignedIntent> {
        Ok(SignedIntentSuccessReturn {
            signed_intent: build_mock_signed_intent(Default::default()),
        })
    }
}

#[derive(Default)]
pub(crate) struct MockIChainFormsOverrides;

pub(crate) fn build_mock_i_chain_forms(_overrides: MockIChainFormsOverrides) -> MockIChainForms {
    MockIChainForms
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct MockIdentity {
    value: [u8; 20],
}

impl ICanonicalField for MockIdentity {
    const KIND: CanonicalFieldKind = CanonicalFieldKind::FixedBytes20;

    fn to_field(_params: ToFieldParams, payload: &Self) -> ToFieldReturn {
        Ok(build_to_field_success_return(
            ToFieldSuccessReturnOverrides {
                field: Some(CanonicalFieldValue::FixedBytes20(payload.value)),
            },
        ))
    }

    fn from_field(_params: FromFieldParams, payload: CanonicalFieldValue) -> FromFieldReturn<Self> {
        let CanonicalFieldValue::FixedBytes20(value) = payload else {
            return Err(FromFieldErrorReturn::WrongKind {
                expected: CanonicalFieldKind::FixedBytes20,
            });
        };
        Ok(build_from_field_success_return(
            build_mock_identity(MockIdentityOverrides { value: Some(value) }),
            FromFieldSuccessReturnOverrides,
        ))
    }
}

impl IIdentityForm for MockIdentity {
    type KeyIdentifier = MockKeyIdentifier;

    fn derive_key_identifier(
        _params: DeriveKeyIdentifierParams,
        _payload: DeriveKeyIdentifierPayload<'_>,
    ) -> DeriveKeyIdentifierReturn<Self::KeyIdentifier> {
        Ok(DeriveKeyIdentifierSuccessReturn {
            key_identifier: build_mock_key_identifier(Default::default()),
        })
    }
}

#[derive(Default)]
pub(crate) struct MockIdentityOverrides {
    pub(crate) value: Option<[u8; 20]>,
}

pub(crate) fn build_mock_identity(overrides: MockIdentityOverrides) -> MockIdentity {
    MockIdentity {
        value: overrides.value.unwrap_or([0x01; 20]),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct MockEntitlement {
    value: [u8; 32],
}

impl ICanonicalField for MockEntitlement {
    const KIND: CanonicalFieldKind = CanonicalFieldKind::Unsigned256;

    fn to_field(_params: ToFieldParams, payload: &Self) -> ToFieldReturn {
        Ok(build_to_field_success_return(
            ToFieldSuccessReturnOverrides {
                field: Some(CanonicalFieldValue::Unsigned256(payload.value)),
            },
        ))
    }

    fn from_field(_params: FromFieldParams, payload: CanonicalFieldValue) -> FromFieldReturn<Self> {
        let CanonicalFieldValue::Unsigned256(value) = payload else {
            return Err(FromFieldErrorReturn::WrongKind {
                expected: CanonicalFieldKind::Unsigned256,
            });
        };
        Ok(build_from_field_success_return(
            build_mock_entitlement(MockEntitlementOverrides { value: Some(value) }),
            FromFieldSuccessReturnOverrides,
        ))
    }
}

#[derive(Default)]
pub(crate) struct MockEntitlementOverrides {
    pub(crate) value: Option<[u8; 32]>,
}

pub(crate) fn build_mock_entitlement(overrides: MockEntitlementOverrides) -> MockEntitlement {
    MockEntitlement {
        value: overrides.value.unwrap_or([0x01; 32]),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct MockInterval {
    value: u64,
}

impl ICanonicalField for MockInterval {
    const KIND: CanonicalFieldKind = CanonicalFieldKind::Unsigned64;

    fn to_field(_params: ToFieldParams, payload: &Self) -> ToFieldReturn {
        Ok(build_to_field_success_return(
            ToFieldSuccessReturnOverrides {
                field: Some(CanonicalFieldValue::Unsigned64(payload.value)),
            },
        ))
    }

    fn from_field(_params: FromFieldParams, payload: CanonicalFieldValue) -> FromFieldReturn<Self> {
        let CanonicalFieldValue::Unsigned64(value) = payload else {
            return Err(FromFieldErrorReturn::WrongKind {
                expected: CanonicalFieldKind::Unsigned64,
            });
        };
        Ok(build_from_field_success_return(
            build_mock_interval(MockIntervalOverrides { value: Some(value) }),
            FromFieldSuccessReturnOverrides,
        ))
    }
}

#[derive(Default)]
pub(crate) struct MockIntervalOverrides {
    pub(crate) value: Option<u64>,
}

pub(crate) fn build_mock_interval(overrides: MockIntervalOverrides) -> MockInterval {
    MockInterval {
        value: overrides.value.unwrap_or(1),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct MockChainIdentifier {
    value: [u8; 32],
}

impl ICanonicalField for MockChainIdentifier {
    const KIND: CanonicalFieldKind = CanonicalFieldKind::Unsigned256;

    fn to_field(_params: ToFieldParams, payload: &Self) -> ToFieldReturn {
        Ok(build_to_field_success_return(
            ToFieldSuccessReturnOverrides {
                field: Some(CanonicalFieldValue::Unsigned256(payload.value)),
            },
        ))
    }

    fn from_field(_params: FromFieldParams, payload: CanonicalFieldValue) -> FromFieldReturn<Self> {
        let CanonicalFieldValue::Unsigned256(value) = payload else {
            return Err(FromFieldErrorReturn::WrongKind {
                expected: CanonicalFieldKind::Unsigned256,
            });
        };
        Ok(build_from_field_success_return(
            build_mock_chain_identifier(MockChainIdentifierOverrides { value: Some(value) }),
            FromFieldSuccessReturnOverrides,
        ))
    }
}

#[derive(Default)]
pub(crate) struct MockChainIdentifierOverrides {
    pub(crate) value: Option<[u8; 32]>,
}

pub(crate) fn build_mock_chain_identifier(
    overrides: MockChainIdentifierOverrides,
) -> MockChainIdentifier {
    MockChainIdentifier {
        value: overrides.value.unwrap_or([0x01; 32]),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct MockKeyIdentifier {
    value: [u8; 32],
}

impl ICanonicalField for MockKeyIdentifier {
    const KIND: CanonicalFieldKind = CanonicalFieldKind::FixedBytes32;

    fn to_field(_params: ToFieldParams, payload: &Self) -> ToFieldReturn {
        Ok(build_to_field_success_return(
            ToFieldSuccessReturnOverrides {
                field: Some(CanonicalFieldValue::FixedBytes32(payload.value)),
            },
        ))
    }

    fn from_field(_params: FromFieldParams, payload: CanonicalFieldValue) -> FromFieldReturn<Self> {
        let CanonicalFieldValue::FixedBytes32(value) = payload else {
            return Err(FromFieldErrorReturn::WrongKind {
                expected: CanonicalFieldKind::FixedBytes32,
            });
        };
        Ok(build_from_field_success_return(
            build_mock_key_identifier(MockKeyIdentifierOverrides { value: Some(value) }),
            FromFieldSuccessReturnOverrides,
        ))
    }
}

#[derive(Default)]
pub(crate) struct MockKeyIdentifierOverrides {
    pub(crate) value: Option<[u8; 32]>,
}

pub(crate) fn build_mock_key_identifier(
    overrides: MockKeyIdentifierOverrides,
) -> MockKeyIdentifier {
    MockKeyIdentifier {
        value: overrides.value.unwrap_or([0x01; 32]),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct MockSignedIntent {
    value: [u8; 32],
}

impl ICanonicalField for MockSignedIntent {
    const KIND: CanonicalFieldKind = CanonicalFieldKind::FixedBytes32;

    fn to_field(_params: ToFieldParams, payload: &Self) -> ToFieldReturn {
        Ok(build_to_field_success_return(
            ToFieldSuccessReturnOverrides {
                field: Some(CanonicalFieldValue::FixedBytes32(payload.value)),
            },
        ))
    }

    fn from_field(_params: FromFieldParams, payload: CanonicalFieldValue) -> FromFieldReturn<Self> {
        let CanonicalFieldValue::FixedBytes32(value) = payload else {
            return Err(FromFieldErrorReturn::WrongKind {
                expected: CanonicalFieldKind::FixedBytes32,
            });
        };
        Ok(build_from_field_success_return(
            build_mock_signed_intent(MockSignedIntentOverrides { value: Some(value) }),
            FromFieldSuccessReturnOverrides,
        ))
    }
}

#[derive(Default)]
pub(crate) struct MockSignedIntentOverrides {
    pub(crate) value: Option<[u8; 32]>,
}

pub(crate) fn build_mock_signed_intent(overrides: MockSignedIntentOverrides) -> MockSignedIntent {
    MockSignedIntent {
        value: overrides.value.unwrap_or([0x01; 32]),
    }
}
