#![allow(dead_code, clippy::too_many_arguments)]

use super::interface::{
    CHAIN_FORMS_INTERFACE_VERSION, ChainFormsDeclaration, ChainFormsIdentifier, IChainForms,
};
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, FromFieldErrorReturn, FromFieldParams,
    FromFieldReturn, FromFieldSuccessReturnOverrides, ICanonicalField, ToFieldParams,
    ToFieldReturn, ToFieldSuccessReturnOverrides, build_from_field_success_return,
    build_to_field_success_return,
};

#[derive(Default)]
pub struct ChainFormsDeclarationOverrides {
    pub identifier: Option<ChainFormsIdentifier>,
    pub adapter_version: Option<u32>,
    pub interface_version: Option<u32>,
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
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct MockIChainForms;

impl IChainForms for MockIChainForms {
    const DECLARATION: ChainFormsDeclaration = ChainFormsDeclaration {
        identifier: ChainFormsIdentifier::EvmV1,
        adapter_version: 1,
        interface_version: CHAIN_FORMS_INTERFACE_VERSION,
    };

    type Identity = MockIdentity;
    type Entitlement = MockEntitlement;
    type Interval = MockInterval;
    type ChainIdentifier = MockChainIdentifier;
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
