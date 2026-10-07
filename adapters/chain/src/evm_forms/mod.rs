mod interface;
#[cfg(test)]
mod mock;
pub(crate) mod provides;
#[cfg(test)]
mod test;

use crate::factory::provides::{
    CHAIN_FORMS_INTERFACE_VERSION, ChainFormsDeclaration, ChainFormsIdentifier, IChainForms,
};
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, FromFieldParams, FromFieldReturn,
    FromFieldSuccessReturn, ICanonicalField, ToFieldParams, ToFieldReturn, ToFieldSuccessReturn,
};
use interface::{
    EvmChainIdentifierForm, EvmChainIdentifierFormConstructorParams,
    EvmChainIdentifierFormTryNewReturn, EvmEntitlementForm, EvmEntitlementFormConstructorParams,
    EvmEntitlementFormTryNewReturn, EvmFormFromFieldErrorReturn, EvmForms,
    EvmFormsConstructorParams, EvmFormsTryNewReturn, EvmIdentityForm,
    EvmIdentityFormConstructorParams, EvmIdentityFormTryNewReturn, EvmIntervalForm,
    EvmIntervalFormConstructorParams, EvmIntervalFormTryNewReturn,
};

impl EvmForms {
    pub const DECLARATION: ChainFormsDeclaration = ChainFormsDeclaration {
        identifier: ChainFormsIdentifier::EvmV1,
        adapter_version: 1,
        interface_version: CHAIN_FORMS_INTERFACE_VERSION,
    };

    pub fn try_new(_params: EvmFormsConstructorParams) -> EvmFormsTryNewReturn {
        Ok(EvmForms)
    }
}

impl IChainForms for EvmForms {
    const DECLARATION: ChainFormsDeclaration = EvmForms::DECLARATION;

    type Identity = EvmIdentityForm;
    type Entitlement = EvmEntitlementForm;
    type Interval = EvmIntervalForm;
    type ChainIdentifier = EvmChainIdentifierForm;
}

impl EvmIdentityForm {
    pub fn try_new(params: EvmIdentityFormConstructorParams) -> EvmIdentityFormTryNewReturn {
        Ok(EvmIdentityForm {
            bytes: params.bytes,
        })
    }
}

impl ICanonicalField for EvmIdentityForm {
    type FromFieldErrorReturn = EvmFormFromFieldErrorReturn;
    const KIND: CanonicalFieldKind = CanonicalFieldKind::FixedBytes20;

    fn to_field(_params: ToFieldParams, payload: &Self) -> ToFieldReturn {
        Ok(ToFieldSuccessReturn {
            field: CanonicalFieldValue::FixedBytes20(payload.bytes),
        })
    }

    fn from_field(
        _params: FromFieldParams,
        payload: CanonicalFieldValue,
    ) -> FromFieldReturn<Self, Self::FromFieldErrorReturn> {
        let CanonicalFieldValue::FixedBytes20(bytes) = payload else {
            return Err(EvmFormFromFieldErrorReturn::WrongKind {
                expected: CanonicalFieldKind::FixedBytes20,
            });
        };
        let Ok(value) = EvmIdentityForm::try_new(EvmIdentityFormConstructorParams { bytes });
        Ok(FromFieldSuccessReturn { value })
    }
}

impl EvmEntitlementForm {
    pub fn try_new(params: EvmEntitlementFormConstructorParams) -> EvmEntitlementFormTryNewReturn {
        Ok(EvmEntitlementForm {
            bytes: params.bytes,
        })
    }
}

impl ICanonicalField for EvmEntitlementForm {
    type FromFieldErrorReturn = EvmFormFromFieldErrorReturn;
    const KIND: CanonicalFieldKind = CanonicalFieldKind::Unsigned256;

    fn to_field(_params: ToFieldParams, payload: &Self) -> ToFieldReturn {
        Ok(ToFieldSuccessReturn {
            field: CanonicalFieldValue::Unsigned256(payload.bytes),
        })
    }

    fn from_field(
        _params: FromFieldParams,
        payload: CanonicalFieldValue,
    ) -> FromFieldReturn<Self, Self::FromFieldErrorReturn> {
        let CanonicalFieldValue::Unsigned256(bytes) = payload else {
            return Err(EvmFormFromFieldErrorReturn::WrongKind {
                expected: CanonicalFieldKind::Unsigned256,
            });
        };
        let Ok(value) = EvmEntitlementForm::try_new(EvmEntitlementFormConstructorParams { bytes });
        Ok(FromFieldSuccessReturn { value })
    }
}

impl EvmIntervalForm {
    pub fn try_new(params: EvmIntervalFormConstructorParams) -> EvmIntervalFormTryNewReturn {
        Ok(EvmIntervalForm {
            value: params.value,
        })
    }
}

impl ICanonicalField for EvmIntervalForm {
    type FromFieldErrorReturn = EvmFormFromFieldErrorReturn;
    const KIND: CanonicalFieldKind = CanonicalFieldKind::Unsigned64;

    fn to_field(_params: ToFieldParams, payload: &Self) -> ToFieldReturn {
        Ok(ToFieldSuccessReturn {
            field: CanonicalFieldValue::Unsigned64(payload.value),
        })
    }

    fn from_field(
        _params: FromFieldParams,
        payload: CanonicalFieldValue,
    ) -> FromFieldReturn<Self, Self::FromFieldErrorReturn> {
        let CanonicalFieldValue::Unsigned64(value) = payload else {
            return Err(EvmFormFromFieldErrorReturn::WrongKind {
                expected: CanonicalFieldKind::Unsigned64,
            });
        };
        let Ok(value) = EvmIntervalForm::try_new(EvmIntervalFormConstructorParams { value });
        Ok(FromFieldSuccessReturn { value })
    }
}

impl EvmChainIdentifierForm {
    pub fn try_new(
        params: EvmChainIdentifierFormConstructorParams,
    ) -> EvmChainIdentifierFormTryNewReturn {
        Ok(EvmChainIdentifierForm {
            bytes: params.bytes,
        })
    }
}

impl ICanonicalField for EvmChainIdentifierForm {
    type FromFieldErrorReturn = EvmFormFromFieldErrorReturn;
    const KIND: CanonicalFieldKind = CanonicalFieldKind::Unsigned256;

    fn to_field(_params: ToFieldParams, payload: &Self) -> ToFieldReturn {
        Ok(ToFieldSuccessReturn {
            field: CanonicalFieldValue::Unsigned256(payload.bytes),
        })
    }

    fn from_field(
        _params: FromFieldParams,
        payload: CanonicalFieldValue,
    ) -> FromFieldReturn<Self, Self::FromFieldErrorReturn> {
        let CanonicalFieldValue::Unsigned256(bytes) = payload else {
            return Err(EvmFormFromFieldErrorReturn::WrongKind {
                expected: CanonicalFieldKind::Unsigned256,
            });
        };
        let Ok(value) =
            EvmChainIdentifierForm::try_new(EvmChainIdentifierFormConstructorParams { bytes });
        Ok(FromFieldSuccessReturn { value })
    }
}
