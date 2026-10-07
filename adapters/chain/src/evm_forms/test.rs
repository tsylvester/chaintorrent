#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{
    EvmChainIdentifierForm, EvmEntitlementForm, EvmFormFromFieldErrorReturn, EvmForms,
    EvmIdentityForm, EvmIntervalForm,
};
use super::mock::{
    EvmChainIdentifierFormConstructorParamsOverrides, EvmEntitlementFormConstructorParamsOverrides,
    EvmIdentityFormConstructorParamsOverrides, EvmIntervalFormConstructorParamsOverrides,
    build_evm_chain_identifier_form, build_evm_entitlement_form, build_evm_identity_form,
    build_evm_interval_form,
};
use crate::factory::provides::{CHAIN_FORMS_INTERFACE_VERSION, ChainFormsIdentifier, IChainForms};
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, FromFieldParams, ICanonicalField, ToFieldParams,
};

/// Contract: an identity form yields the fixed twenty-byte field of its bytes.
/// Arrange: an identity form holding `[0x31; 20]`.
/// Act:     EvmIdentityForm::to_field over the form.
/// Assert:  the yielded field is `CanonicalFieldValue::FixedBytes20([0x31; 20])`.
#[test]
fn evm_identity_form_yields_its_bytes_as_a_fixed_twenty_byte_field() {
    // Arrange
    let form = build_evm_identity_form(EvmIdentityFormConstructorParamsOverrides {
        bytes: Some([0x31; 20]),
    });

    // Act
    let Ok(success) = EvmIdentityForm::to_field(ToFieldParams, &form);

    // Assert
    assert_eq!(success.field, CanonicalFieldValue::FixedBytes20([0x31; 20]));
}

/// Contract: a fixed twenty-byte field rebuilds the identity form holding its
///   bytes.
/// Arrange: a fixed twenty-byte field of `[0x31; 20]`.
/// Act:     EvmIdentityForm::from_field over the field.
/// Assert:  the rebuilt form equals one built holding `[0x31; 20]`.
#[test]
fn evm_identity_form_is_rebuilt_from_a_fixed_twenty_byte_field() {
    // Act
    let Ok(success) = EvmIdentityForm::from_field(
        FromFieldParams,
        CanonicalFieldValue::FixedBytes20([0x31; 20]),
    ) else {
        panic!("a fixed twenty-byte field is admitted");
    };

    // Assert
    assert_eq!(
        success.value,
        build_evm_identity_form(EvmIdentityFormConstructorParamsOverrides {
            bytes: Some([0x31; 20]),
        })
    );
}

/// Contract: a field that is not a fixed twenty-byte value is refused, naming
///   the kind expected.
/// Arrange: a fixed thirty-two-byte field of `[0x31; 32]`.
/// Act:     EvmIdentityForm::from_field over the field.
/// Assert:  the refusal is `WrongKind { expected: FixedBytes20 }`.
#[test]
fn evm_identity_form_refuses_a_field_of_another_kind() {
    // Act
    let Err(error) = EvmIdentityForm::from_field(
        FromFieldParams,
        CanonicalFieldValue::FixedBytes32([0x31; 32]),
    ) else {
        panic!("a field of another kind is refused");
    };

    // Assert
    assert_eq!(
        error,
        EvmFormFromFieldErrorReturn::WrongKind {
            expected: CanonicalFieldKind::FixedBytes20,
        }
    );
}

/// Contract: an entitlement form yields the 256-bit unsigned field of its
///   bytes.
/// Arrange: an entitlement form holding `[0x41; 32]`.
/// Act:     EvmEntitlementForm::to_field over the form.
/// Assert:  the yielded field is `CanonicalFieldValue::Unsigned256([0x41; 32])`.
#[test]
fn evm_entitlement_form_yields_its_bytes_as_a_256_bit_unsigned_field() {
    // Arrange
    let form = build_evm_entitlement_form(EvmEntitlementFormConstructorParamsOverrides {
        bytes: Some([0x41; 32]),
    });

    // Act
    let Ok(success) = EvmEntitlementForm::to_field(ToFieldParams, &form);

    // Assert
    assert_eq!(success.field, CanonicalFieldValue::Unsigned256([0x41; 32]));
}

/// Contract: a 256-bit unsigned field rebuilds the entitlement form holding
///   its bytes.
/// Arrange: a 256-bit unsigned field of `[0x41; 32]`.
/// Act:     EvmEntitlementForm::from_field over the field.
/// Assert:  the rebuilt form equals one built holding `[0x41; 32]`.
#[test]
fn evm_entitlement_form_is_rebuilt_from_a_256_bit_unsigned_field() {
    // Act
    let Ok(success) = EvmEntitlementForm::from_field(
        FromFieldParams,
        CanonicalFieldValue::Unsigned256([0x41; 32]),
    ) else {
        panic!("a 256-bit unsigned field is admitted");
    };

    // Assert
    assert_eq!(
        success.value,
        build_evm_entitlement_form(EvmEntitlementFormConstructorParamsOverrides {
            bytes: Some([0x41; 32]),
        })
    );
}

/// Contract: a 32-byte field of the fixed-bytes kind is not an entitlement;
///   it is refused, naming the kind expected.
/// Arrange: a fixed thirty-two-byte field of `[0x41; 32]`.
/// Act:     EvmEntitlementForm::from_field over the field.
/// Assert:  the refusal is `WrongKind { expected: Unsigned256 }`.
#[test]
fn evm_entitlement_form_refuses_a_field_of_another_kind() {
    // Act
    let Err(error) = EvmEntitlementForm::from_field(
        FromFieldParams,
        CanonicalFieldValue::FixedBytes32([0x41; 32]),
    ) else {
        panic!("a field of another kind is refused");
    };

    // Assert
    assert_eq!(
        error,
        EvmFormFromFieldErrorReturn::WrongKind {
            expected: CanonicalFieldKind::Unsigned256,
        }
    );
}

/// Contract: an interval form yields the 64-bit unsigned field of its value.
/// Arrange: an interval form holding `7`.
/// Act:     EvmIntervalForm::to_field over the form.
/// Assert:  the yielded field is `CanonicalFieldValue::Unsigned64(7)`.
#[test]
fn evm_interval_form_yields_its_value_as_a_64_bit_unsigned_field() {
    // Arrange
    let form =
        build_evm_interval_form(EvmIntervalFormConstructorParamsOverrides { value: Some(7) });

    // Act
    let Ok(success) = EvmIntervalForm::to_field(ToFieldParams, &form);

    // Assert
    assert_eq!(success.field, CanonicalFieldValue::Unsigned64(7));
}

/// Contract: a 64-bit unsigned field rebuilds the interval form holding its
///   value.
/// Arrange: a 64-bit unsigned field of `7`.
/// Act:     EvmIntervalForm::from_field over the field.
/// Assert:  the rebuilt form equals one built holding `7`.
#[test]
fn evm_interval_form_is_rebuilt_from_a_64_bit_unsigned_field() {
    // Act
    let Ok(success) =
        EvmIntervalForm::from_field(FromFieldParams, CanonicalFieldValue::Unsigned64(7))
    else {
        panic!("a 64-bit unsigned field is admitted");
    };

    // Assert
    assert_eq!(
        success.value,
        build_evm_interval_form(EvmIntervalFormConstructorParamsOverrides { value: Some(7) })
    );
}

/// Contract: an unsigned field of another width is not an interval; it is
///   refused, naming the kind expected.
/// Arrange: a 32-bit unsigned field of `7`.
/// Act:     EvmIntervalForm::from_field over the field.
/// Assert:  the refusal is `WrongKind { expected: Unsigned64 }`.
#[test]
fn evm_interval_form_refuses_a_field_of_another_kind() {
    // Act
    let Err(error) =
        EvmIntervalForm::from_field(FromFieldParams, CanonicalFieldValue::Unsigned32(7))
    else {
        panic!("a field of another kind is refused");
    };

    // Assert
    assert_eq!(
        error,
        EvmFormFromFieldErrorReturn::WrongKind {
            expected: CanonicalFieldKind::Unsigned64,
        }
    );
}

/// Contract: a chain-identifier form yields the 256-bit unsigned field of its
///   bytes.
/// Arrange: a chain-identifier form holding `[0x21; 32]`.
/// Act:     EvmChainIdentifierForm::to_field over the form.
/// Assert:  the yielded field is `CanonicalFieldValue::Unsigned256([0x21; 32])`.
#[test]
fn evm_chain_identifier_form_yields_its_bytes_as_a_256_bit_unsigned_field() {
    // Arrange
    let form = build_evm_chain_identifier_form(EvmChainIdentifierFormConstructorParamsOverrides {
        bytes: Some([0x21; 32]),
    });

    // Act
    let Ok(success) = EvmChainIdentifierForm::to_field(ToFieldParams, &form);

    // Assert
    assert_eq!(success.field, CanonicalFieldValue::Unsigned256([0x21; 32]));
}

/// Contract: a 256-bit unsigned field rebuilds the chain-identifier form
///   holding its bytes.
/// Arrange: a 256-bit unsigned field of `[0x21; 32]`.
/// Act:     EvmChainIdentifierForm::from_field over the field.
/// Assert:  the rebuilt form equals one built holding `[0x21; 32]`.
#[test]
fn evm_chain_identifier_form_is_rebuilt_from_a_256_bit_unsigned_field() {
    // Act
    let Ok(success) = EvmChainIdentifierForm::from_field(
        FromFieldParams,
        CanonicalFieldValue::Unsigned256([0x21; 32]),
    ) else {
        panic!("a 256-bit unsigned field is admitted");
    };

    // Assert
    assert_eq!(
        success.value,
        build_evm_chain_identifier_form(EvmChainIdentifierFormConstructorParamsOverrides {
            bytes: Some([0x21; 32]),
        },)
    );
}

/// Contract: a field that is not a 256-bit unsigned value is refused, naming
///   the kind expected.
/// Arrange: a fixed thirty-two-byte field of `[0x21; 32]`.
/// Act:     EvmChainIdentifierForm::from_field over the field.
/// Assert:  the refusal is `WrongKind { expected: Unsigned256 }`.
#[test]
fn evm_chain_identifier_form_refuses_a_field_of_another_kind() {
    // Act
    let Err(error) = EvmChainIdentifierForm::from_field(
        FromFieldParams,
        CanonicalFieldValue::FixedBytes32([0x21; 32]),
    ) else {
        panic!("a field of another kind is refused");
    };

    // Assert
    assert_eq!(
        error,
        EvmFormFromFieldErrorReturn::WrongKind {
            expected: CanonicalFieldKind::Unsigned256,
        }
    );
}

/// Contract: the EVM suite's identity, entitlement, interval, and chain
///   identifier take the fixed twenty-byte, 256-bit unsigned, 64-bit unsigned,
///   and 256-bit unsigned kinds.
/// Arrange: none; the kinds are associated constants.
/// Act:     read `KIND` of each `EvmForms` associated form.
/// Assert:  the kinds equal `FixedBytes20`, `Unsigned256`, `Unsigned64`, and
///   `Unsigned256` in that order.
#[test]
fn evm_forms_bind_the_kinds_the_suite_schemas_fix() {
    // Assert
    assert_eq!(
        <<EvmForms as IChainForms>::Identity as ICanonicalField>::KIND,
        CanonicalFieldKind::FixedBytes20
    );
    assert_eq!(
        <<EvmForms as IChainForms>::Entitlement as ICanonicalField>::KIND,
        CanonicalFieldKind::Unsigned256
    );
    assert_eq!(
        <<EvmForms as IChainForms>::Interval as ICanonicalField>::KIND,
        CanonicalFieldKind::Unsigned64
    );
    assert_eq!(
        <<EvmForms as IChainForms>::ChainIdentifier as ICanonicalField>::KIND,
        CanonicalFieldKind::Unsigned256
    );
}

/// Contract: the EVM forms declaration names the EVM forms identifier, adapter
///   version one, and the chain forms interface version.
/// Arrange: none; the declaration is an associated constant.
/// Act:     read `EvmForms::DECLARATION`.
/// Assert:  `identifier` is `ChainFormsIdentifier::EvmV1`, `adapter_version` is
///   `1`, and `interface_version` is `CHAIN_FORMS_INTERFACE_VERSION`.
#[test]
fn evm_forms_declares_its_identifier_and_versions() {
    // Assert
    assert_eq!(
        EvmForms::DECLARATION.identifier,
        ChainFormsIdentifier::EvmV1
    );
    assert_eq!(EvmForms::DECLARATION.adapter_version, 1);
    assert_eq!(
        EvmForms::DECLARATION.interface_version,
        CHAIN_FORMS_INTERFACE_VERSION
    );
}
