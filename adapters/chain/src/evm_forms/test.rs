use super::interface::{
    EvmChainIdentifierForm, EvmEntitlementForm, EvmForms, EvmIdentityForm, EvmIntervalForm,
};
use super::mock::{
    EvmChainIdentifierFormConstructorParamsOverrides, EvmEntitlementFormConstructorParamsOverrides,
    EvmIdentityFormConstructorParamsOverrides, EvmIntervalFormConstructorParamsOverrides,
    build_evm_chain_identifier_form, build_evm_chain_identifier_form_constructor_params,
    build_evm_entitlement_form, build_evm_entitlement_form_constructor_params,
    build_evm_forms_constructor_params, build_evm_identity_form,
    build_evm_identity_form_constructor_params, build_evm_interval_form,
    build_evm_interval_form_constructor_params,
};
use crate::factory::provides::{
    CHAIN_FORMS_INTERFACE_VERSION, ChainFormsDeclarationOverrides, ChainFormsIdentifier,
    IChainForms, build_chain_forms_declaration,
};
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, FromFieldErrorReturn, ICanonicalField,
    build_from_field_params, build_to_field_params,
};

/// Contract: `EvmIdentityForm::try_new(params)`, one branch: any params →
///   `Ok(EvmIdentityForm { bytes })`, the array moved from the params.
/// Arrange: `build_evm_identity_form_constructor_params` overriding `bytes`
///   with the ascending literal.
/// Act:     `EvmIdentityForm::try_new(params)`.
/// Assert:  the `bytes` of the form the `Ok` arm holds equal the ascending
///   literal by `assert_eq!`.
#[test]
fn evm_identity_form_try_new_moves_the_bytes_of_its_params_into_the_form() {
    // Arrange
    let bytes = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10, 0x11, 0x12, 0x13, 0x14,
    ];
    let params =
        build_evm_identity_form_constructor_params(EvmIdentityFormConstructorParamsOverrides {
            bytes: Some(bytes),
        });

    // Act
    let result = EvmIdentityForm::try_new(params);

    // Assert
    let Ok(form) = result;
    assert_eq!(form.bytes, bytes);
}

/// Contract: `EvmIdentityForm::to_field`, one branch: any form →
///   `Ok(ToFieldSuccessReturn { field: CanonicalFieldValue::FixedBytes20(payload.bytes) })`.
/// Arrange: `build_evm_identity_form` overriding `bytes` with the ascending
///   literal, and `build_to_field_params` at its defaults; a field built
///   from anything but the form's bytes, or in a variant other than
///   `FixedBytes20`, fails.
/// Act:     `EvmIdentityForm::to_field(params, &form)`.
/// Assert:  the `field` of the `Ok` arm equals
///   `CanonicalFieldValue::FixedBytes20` carrying the ascending literal by
///   `assert_eq!`.
#[test]
fn evm_identity_form_yields_its_bytes_as_a_fixed_twenty_byte_field() {
    // Arrange
    let bytes = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10, 0x11, 0x12, 0x13, 0x14,
    ];
    let form =
        build_evm_identity_form(EvmIdentityFormConstructorParamsOverrides { bytes: Some(bytes) });
    let params = build_to_field_params(Default::default());

    // Act
    let result = EvmIdentityForm::to_field(params, &form);

    // Assert
    let Ok(success) = result;
    assert_eq!(success.field, CanonicalFieldValue::FixedBytes20(bytes));
}

/// Contract: `EvmIdentityForm::from_field`, admitted: `payload` is
///   `CanonicalFieldValue::FixedBytes20(bytes)` →
///   `Ok(FromFieldSuccessReturn { value })`, the form built from `bytes`
///   through `try_new`.
/// Arrange: the payload `CanonicalFieldValue::FixedBytes20` carrying the
///   ascending literal, and `build_from_field_params` at its defaults; a
///   form rebuilt from anything but the payload's bytes fails.
/// Act:     `EvmIdentityForm::from_field(params, payload)`.
/// Assert:  the `bytes` of the `value` the `Ok` arm holds equal the
///   ascending literal by `assert_eq!`.
#[test]
fn evm_identity_form_is_rebuilt_from_a_fixed_twenty_byte_field() {
    // Arrange
    let bytes = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10, 0x11, 0x12, 0x13, 0x14,
    ];
    let payload = CanonicalFieldValue::FixedBytes20(bytes);
    let params = build_from_field_params(Default::default());

    // Act
    let result = EvmIdentityForm::from_field(params, payload);

    // Assert
    assert_eq!(result.ok().map(|success| success.value.bytes), Some(bytes));
}

/// Contract: `EvmIdentityForm::from_field`, wrong kind: `payload` is not
///   `CanonicalFieldValue::FixedBytes20` →
///   `Err(FromFieldErrorReturn::WrongKind { expected: CanonicalFieldKind::FixedBytes20 })`.
/// Arrange: the payload `CanonicalFieldValue::FixedBytes32` carrying
///   thirty-two ascending bytes, a kind that is not the form's, and
///   `build_from_field_params` at its defaults; a check that admits any
///   byte array fails.
/// Act:     `EvmIdentityForm::from_field(params, payload)`.
/// Assert:  `result.err()` equals
///   `Some(FromFieldErrorReturn::WrongKind { expected: CanonicalFieldKind::FixedBytes20 })`
///   by `assert_eq!`.
#[test]
fn evm_identity_form_refuses_a_field_of_another_kind() {
    // Arrange
    let bytes = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e,
        0x1f, 0x20,
    ];
    let payload = CanonicalFieldValue::FixedBytes32(bytes);
    let params = build_from_field_params(Default::default());

    // Act
    let result = EvmIdentityForm::from_field(params, payload);

    // Assert
    assert_eq!(
        result.err(),
        Some(FromFieldErrorReturn::WrongKind {
            expected: CanonicalFieldKind::FixedBytes20,
        })
    );
}

/// Contract: `EvmEntitlementForm::try_new(params)`, one branch: any params
///   → `Ok(EvmEntitlementForm { bytes })`, the array moved from the params.
/// Arrange: `build_evm_entitlement_form_constructor_params` overriding
///   `bytes` with the ascending literal.
/// Act:     `EvmEntitlementForm::try_new(params)`.
/// Assert:  the `bytes` of the form the `Ok` arm holds equal the ascending
///   literal by `assert_eq!`.
#[test]
fn evm_entitlement_form_try_new_moves_the_bytes_of_its_params_into_the_form() {
    // Arrange
    let bytes = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e,
        0x1f, 0x20,
    ];
    let params = build_evm_entitlement_form_constructor_params(
        EvmEntitlementFormConstructorParamsOverrides { bytes: Some(bytes) },
    );

    // Act
    let result = EvmEntitlementForm::try_new(params);

    // Assert
    let Ok(form) = result;
    assert_eq!(form.bytes, bytes);
}

/// Contract: `EvmEntitlementForm::to_field`, one branch: any form →
///   `Ok(ToFieldSuccessReturn { field: CanonicalFieldValue::Unsigned256(payload.bytes) })`.
/// Arrange: `build_evm_entitlement_form` overriding `bytes` with the
///   ascending literal, and `build_to_field_params` at its defaults; a
///   field built from anything but the form's bytes, or in a variant other
///   than `Unsigned256`, fails.
/// Act:     `EvmEntitlementForm::to_field(params, &form)`.
/// Assert:  the `field` of the `Ok` arm equals
///   `CanonicalFieldValue::Unsigned256` carrying the ascending literal by
///   `assert_eq!`.
#[test]
fn evm_entitlement_form_yields_its_bytes_as_a_256_bit_unsigned_field() {
    // Arrange
    let bytes = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e,
        0x1f, 0x20,
    ];
    let form = build_evm_entitlement_form(EvmEntitlementFormConstructorParamsOverrides {
        bytes: Some(bytes),
    });
    let params = build_to_field_params(Default::default());

    // Act
    let result = EvmEntitlementForm::to_field(params, &form);

    // Assert
    let Ok(success) = result;
    assert_eq!(success.field, CanonicalFieldValue::Unsigned256(bytes));
}

/// Contract: `EvmEntitlementForm::from_field`, admitted: `payload` is
///   `CanonicalFieldValue::Unsigned256(bytes)` →
///   `Ok(FromFieldSuccessReturn { value })`, the form built from `bytes`
///   through `try_new`.
/// Arrange: the payload `CanonicalFieldValue::Unsigned256` carrying the
///   ascending literal, and `build_from_field_params` at its defaults; a
///   form rebuilt from anything but the payload's bytes fails.
/// Act:     `EvmEntitlementForm::from_field(params, payload)`.
/// Assert:  the `bytes` of the `value` the `Ok` arm holds equal the
///   ascending literal by `assert_eq!`.
#[test]
fn evm_entitlement_form_is_rebuilt_from_a_256_bit_unsigned_field() {
    // Arrange
    let bytes = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e,
        0x1f, 0x20,
    ];
    let payload = CanonicalFieldValue::Unsigned256(bytes);
    let params = build_from_field_params(Default::default());

    // Act
    let result = EvmEntitlementForm::from_field(params, payload);

    // Assert
    assert_eq!(result.ok().map(|success| success.value.bytes), Some(bytes));
}

/// Contract: `EvmEntitlementForm::from_field`, wrong kind: `payload` is not
///   `CanonicalFieldValue::Unsigned256` →
///   `Err(FromFieldErrorReturn::WrongKind { expected: CanonicalFieldKind::Unsigned256 })`.
/// Arrange: the payload `CanonicalFieldValue::FixedBytes32` carrying the
///   ascending literal, a kind that carries as many bytes as the form's
///   and is not the form's, and `build_from_field_params` at its defaults;
///   a check that compares only the width of the bytes fails.
/// Act:     `EvmEntitlementForm::from_field(params, payload)`.
/// Assert:  `result.err()` equals
///   `Some(FromFieldErrorReturn::WrongKind { expected: CanonicalFieldKind::Unsigned256 })`
///   by `assert_eq!`.
#[test]
fn evm_entitlement_form_refuses_a_field_of_another_kind() {
    // Arrange
    let bytes = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e,
        0x1f, 0x20,
    ];
    let payload = CanonicalFieldValue::FixedBytes32(bytes);
    let params = build_from_field_params(Default::default());

    // Act
    let result = EvmEntitlementForm::from_field(params, payload);

    // Assert
    assert_eq!(
        result.err(),
        Some(FromFieldErrorReturn::WrongKind {
            expected: CanonicalFieldKind::Unsigned256,
        })
    );
}

/// Contract: `EvmIntervalForm::try_new(params)`, one branch: any params →
///   `Ok(EvmIntervalForm { value })`, the value moved from the params.
/// Arrange: `build_evm_interval_form_constructor_params` overriding `value`
///   with `0x0102_0304_0506_0708`, a value whose upper 32 bits are non-zero
///   and which differs from the builder default.
/// Act:     `EvmIntervalForm::try_new(params)`.
/// Assert:  the `value` of the form the `Ok` arm holds equals
///   `0x0102_0304_0506_0708` by `assert_eq!`.
#[test]
fn evm_interval_form_try_new_moves_the_value_of_its_params_into_the_form() {
    // Arrange
    let value = 0x0102_0304_0506_0708;
    let params =
        build_evm_interval_form_constructor_params(EvmIntervalFormConstructorParamsOverrides {
            value: Some(value),
        });

    // Act
    let result = EvmIntervalForm::try_new(params);

    // Assert
    let Ok(form) = result;
    assert_eq!(form.value, value);
}

/// Contract: `EvmIntervalForm::to_field`, one branch: any form →
///   `Ok(ToFieldSuccessReturn { field: CanonicalFieldValue::Unsigned64(payload.value) })`.
/// Arrange: `build_evm_interval_form` overriding `value` with
///   `0x0102_0304_0506_0708`, and `build_to_field_params` at its defaults;
///   a field built from anything but the form's value, a value narrowed to
///   32 bits, or a variant other than `Unsigned64`, fails.
/// Act:     `EvmIntervalForm::to_field(params, &form)`.
/// Assert:  the `field` of the `Ok` arm equals
///   `CanonicalFieldValue::Unsigned64(0x0102_0304_0506_0708)` by
///   `assert_eq!`.
#[test]
fn evm_interval_form_yields_its_value_as_a_64_bit_unsigned_field() {
    // Arrange
    let value = 0x0102_0304_0506_0708;
    let form =
        build_evm_interval_form(EvmIntervalFormConstructorParamsOverrides { value: Some(value) });
    let params = build_to_field_params(Default::default());

    // Act
    let result = EvmIntervalForm::to_field(params, &form);

    // Assert
    let Ok(success) = result;
    assert_eq!(success.field, CanonicalFieldValue::Unsigned64(value));
}

/// Contract: `EvmIntervalForm::from_field`, admitted: `payload` is
///   `CanonicalFieldValue::Unsigned64(value)` →
///   `Ok(FromFieldSuccessReturn { value })`, the form built from `value`
///   through `try_new`.
/// Arrange: the payload `CanonicalFieldValue::Unsigned64(0x0102_0304_0506_0708)`,
///   and `build_from_field_params` at its defaults; a form rebuilt from
///   anything but the payload's value, or from a value narrowed to 32
///   bits, fails.
/// Act:     `EvmIntervalForm::from_field(params, payload)`.
/// Assert:  the `value` of the form the `Ok` arm holds equals
///   `0x0102_0304_0506_0708` by `assert_eq!`.
#[test]
fn evm_interval_form_is_rebuilt_from_a_64_bit_unsigned_field() {
    // Arrange
    let value = 0x0102_0304_0506_0708;
    let payload = CanonicalFieldValue::Unsigned64(value);
    let params = build_from_field_params(Default::default());

    // Act
    let result = EvmIntervalForm::from_field(params, payload);

    // Assert
    assert_eq!(result.ok().map(|success| success.value.value), Some(value));
}

/// Contract: `EvmIntervalForm::from_field`, wrong kind: `payload` is not
///   `CanonicalFieldValue::Unsigned64` →
///   `Err(FromFieldErrorReturn::WrongKind { expected: CanonicalFieldKind::Unsigned64 })`.
/// Arrange: the payload `CanonicalFieldValue::Unsigned32(0x0102_0304)`, an
///   unsigned kind that is not the form's, and `build_from_field_params`
///   at its defaults; a check that admits any unsigned value fails.
/// Act:     `EvmIntervalForm::from_field(params, payload)`.
/// Assert:  `result.err()` equals
///   `Some(FromFieldErrorReturn::WrongKind { expected: CanonicalFieldKind::Unsigned64 })`
///   by `assert_eq!`.
#[test]
fn evm_interval_form_refuses_a_field_of_another_kind() {
    // Arrange
    let payload = CanonicalFieldValue::Unsigned32(0x0102_0304);
    let params = build_from_field_params(Default::default());

    // Act
    let result = EvmIntervalForm::from_field(params, payload);

    // Assert
    assert_eq!(
        result.err(),
        Some(FromFieldErrorReturn::WrongKind {
            expected: CanonicalFieldKind::Unsigned64,
        })
    );
}

/// Contract: `EvmChainIdentifierForm::try_new(params)`, one branch: any
///   params → `Ok(EvmChainIdentifierForm { bytes })`, the array moved from
///   the params.
/// Arrange: `build_evm_chain_identifier_form_constructor_params`
///   overriding `bytes` with the ascending literal.
/// Act:     `EvmChainIdentifierForm::try_new(params)`.
/// Assert:  the `bytes` of the form the `Ok` arm holds equal the ascending
///   literal by `assert_eq!`.
#[test]
fn evm_chain_identifier_form_try_new_moves_the_bytes_of_its_params_into_the_form() {
    // Arrange
    let bytes = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e,
        0x1f, 0x20,
    ];
    let params = build_evm_chain_identifier_form_constructor_params(
        EvmChainIdentifierFormConstructorParamsOverrides { bytes: Some(bytes) },
    );

    // Act
    let result = EvmChainIdentifierForm::try_new(params);

    // Assert
    let Ok(form) = result;
    assert_eq!(form.bytes, bytes);
}

/// Contract: `EvmChainIdentifierForm::to_field`, one branch: any form →
///   `Ok(ToFieldSuccessReturn { field: CanonicalFieldValue::Unsigned256(payload.bytes) })`.
/// Arrange: `build_evm_chain_identifier_form` overriding `bytes` with the
///   ascending literal, and `build_to_field_params` at its defaults; a
///   field built from anything but the form's bytes, or in a variant other
///   than `Unsigned256`, fails.
/// Act:     `EvmChainIdentifierForm::to_field(params, &form)`.
/// Assert:  the `field` of the `Ok` arm equals
///   `CanonicalFieldValue::Unsigned256` carrying the ascending literal by
///   `assert_eq!`.
#[test]
fn evm_chain_identifier_form_yields_its_bytes_as_a_256_bit_unsigned_field() {
    // Arrange
    let bytes = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e,
        0x1f, 0x20,
    ];
    let form = build_evm_chain_identifier_form(EvmChainIdentifierFormConstructorParamsOverrides {
        bytes: Some(bytes),
    });
    let params = build_to_field_params(Default::default());

    // Act
    let result = EvmChainIdentifierForm::to_field(params, &form);

    // Assert
    let Ok(success) = result;
    assert_eq!(success.field, CanonicalFieldValue::Unsigned256(bytes));
}

/// Contract: `EvmChainIdentifierForm::from_field`, admitted: `payload` is
///   `CanonicalFieldValue::Unsigned256(bytes)` →
///   `Ok(FromFieldSuccessReturn { value })`, the form built from `bytes`
///   through `try_new`.
/// Arrange: the payload `CanonicalFieldValue::Unsigned256` carrying the
///   ascending literal, and `build_from_field_params` at its defaults; a
///   form rebuilt from anything but the payload's bytes fails.
/// Act:     `EvmChainIdentifierForm::from_field(params, payload)`.
/// Assert:  the `bytes` of the `value` the `Ok` arm holds equal the
///   ascending literal by `assert_eq!`.
#[test]
fn evm_chain_identifier_form_is_rebuilt_from_a_256_bit_unsigned_field() {
    // Arrange
    let bytes = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e,
        0x1f, 0x20,
    ];
    let payload = CanonicalFieldValue::Unsigned256(bytes);
    let params = build_from_field_params(Default::default());

    // Act
    let result = EvmChainIdentifierForm::from_field(params, payload);

    // Assert
    assert_eq!(result.ok().map(|success| success.value.bytes), Some(bytes));
}

/// Contract: `EvmChainIdentifierForm::from_field`, wrong kind: `payload`
///   is not `CanonicalFieldValue::Unsigned256` →
///   `Err(FromFieldErrorReturn::WrongKind { expected: CanonicalFieldKind::Unsigned256 })`.
/// Arrange: the payload `CanonicalFieldValue::FixedBytes32` carrying the
///   ascending literal, a kind that carries as many bytes as the form's
///   and is not the form's, and `build_from_field_params` at its defaults;
///   a check that compares only the width of the bytes fails.
/// Act:     `EvmChainIdentifierForm::from_field(params, payload)`.
/// Assert:  `result.err()` equals
///   `Some(FromFieldErrorReturn::WrongKind { expected: CanonicalFieldKind::Unsigned256 })`
///   by `assert_eq!`.
#[test]
fn evm_chain_identifier_form_refuses_a_field_of_another_kind() {
    // Arrange
    let bytes = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e,
        0x1f, 0x20,
    ];
    let payload = CanonicalFieldValue::FixedBytes32(bytes);
    let params = build_from_field_params(Default::default());

    // Act
    let result = EvmChainIdentifierForm::from_field(params, payload);

    // Assert
    assert_eq!(
        result.err(),
        Some(FromFieldErrorReturn::WrongKind {
            expected: CanonicalFieldKind::Unsigned256,
        })
    );
}

/// Contract: `ICanonicalField` for the form `IChainForms for EvmForms`
///   binds as `Identity`: `KIND` is `CanonicalFieldKind::FixedBytes20`.
/// Arrange: the kind is read through the trait's associated type, so a
///   binding to a form that declares another kind fails.
/// Act:     read `<<EvmForms as IChainForms>::Identity as ICanonicalField>::KIND`.
/// Assert:  the kind equals `CanonicalFieldKind::FixedBytes20` by
///   `assert_eq!`.
#[test]
fn evm_forms_bind_the_identity_to_the_fixed_twenty_byte_kind() {
    // Arrange

    // Act
    let kind = <<EvmForms as IChainForms>::Identity as ICanonicalField>::KIND;

    // Assert
    assert_eq!(kind, CanonicalFieldKind::FixedBytes20);
}

/// Contract: `ICanonicalField` for the form `IChainForms for EvmForms`
///   binds as `Entitlement`: `KIND` is `CanonicalFieldKind::Unsigned256`.
/// Arrange: the kind is read through the trait's associated type, so a
///   binding to a form that declares another kind fails.
/// Act:     read `<<EvmForms as IChainForms>::Entitlement as ICanonicalField>::KIND`.
/// Assert:  the kind equals `CanonicalFieldKind::Unsigned256` by
///   `assert_eq!`.
#[test]
fn evm_forms_bind_the_entitlement_to_the_256_bit_unsigned_kind() {
    // Arrange

    // Act
    let kind = <<EvmForms as IChainForms>::Entitlement as ICanonicalField>::KIND;

    // Assert
    assert_eq!(kind, CanonicalFieldKind::Unsigned256);
}

/// Contract: `ICanonicalField` for the form `IChainForms for EvmForms`
///   binds as `Interval`: `KIND` is `CanonicalFieldKind::Unsigned64`.
/// Arrange: the kind is read through the trait's associated type, so a
///   binding to a form that declares another kind fails.
/// Act:     read `<<EvmForms as IChainForms>::Interval as ICanonicalField>::KIND`.
/// Assert:  the kind equals `CanonicalFieldKind::Unsigned64` by
///   `assert_eq!`.
#[test]
fn evm_forms_bind_the_interval_to_the_64_bit_unsigned_kind() {
    // Arrange

    // Act
    let kind = <<EvmForms as IChainForms>::Interval as ICanonicalField>::KIND;

    // Assert
    assert_eq!(kind, CanonicalFieldKind::Unsigned64);
}

/// Contract: `ICanonicalField` for the form `IChainForms for EvmForms`
///   binds as `ChainIdentifier`: `KIND` is `CanonicalFieldKind::Unsigned256`.
/// Arrange: the kind is read through the trait's associated type, so a
///   binding to a form that declares another kind fails.
/// Act:     read `<<EvmForms as IChainForms>::ChainIdentifier as ICanonicalField>::KIND`.
/// Assert:  the kind equals `CanonicalFieldKind::Unsigned256` by
///   `assert_eq!`.
#[test]
fn evm_forms_bind_the_chain_identifier_to_the_256_bit_unsigned_kind() {
    // Arrange

    // Act
    let kind = <<EvmForms as IChainForms>::ChainIdentifier as ICanonicalField>::KIND;

    // Assert
    assert_eq!(kind, CanonicalFieldKind::Unsigned256);
}

/// Contract: `EvmForms::DECLARATION`, the inherent constant
///   `ChainFormsDeclaration { identifier: ChainFormsIdentifier::EvmV1, adapter_version: 1, interface_version: CHAIN_FORMS_INTERFACE_VERSION }`.
/// Arrange: the expected declaration from `build_chain_forms_declaration`
///   overriding `identifier`, `adapter_version`, and `interface_version`
///   with `ChainFormsIdentifier::EvmV1`, the literal `1`, and
///   `CHAIN_FORMS_INTERFACE_VERSION`, the values the interaction spec
///   states; no builder default is read.
/// Act:     read `EvmForms::DECLARATION`.
/// Assert:  the declaration equals the expected declaration by
///   `assert_eq!`.
#[test]
fn evm_forms_declares_its_identifier_and_versions() {
    // Arrange
    let expected = build_chain_forms_declaration(ChainFormsDeclarationOverrides {
        identifier: Some(ChainFormsIdentifier::EvmV1),
        adapter_version: Some(1),
        interface_version: Some(CHAIN_FORMS_INTERFACE_VERSION),
    });

    // Act
    let declaration = EvmForms::DECLARATION;

    // Assert
    assert_eq!(declaration, expected);
}

/// Contract: `IChainForms for EvmForms`:
///   `const DECLARATION: ChainFormsDeclaration = EvmForms::DECLARATION;`.
/// Arrange: the expected declaration from `build_chain_forms_declaration`
///   overriding `identifier`, `adapter_version`, and `interface_version`
///   with `ChainFormsIdentifier::EvmV1`, the literal `1`, and
///   `CHAIN_FORMS_INTERFACE_VERSION`; no builder default is read.
/// Act:     read `<EvmForms as IChainForms>::DECLARATION`.
/// Assert:  the declaration equals the expected declaration by
///   `assert_eq!`.
#[test]
fn evm_forms_implement_the_chain_forms_interface_with_their_declaration() {
    // Arrange
    let expected = build_chain_forms_declaration(ChainFormsDeclarationOverrides {
        identifier: Some(ChainFormsIdentifier::EvmV1),
        adapter_version: Some(1),
        interface_version: Some(CHAIN_FORMS_INTERFACE_VERSION),
    });

    // Act
    let declaration = <EvmForms as IChainForms>::DECLARATION;

    // Assert
    assert_eq!(declaration, expected);
}

/// Contract: `EvmForms::try_new(params)`, one branch: any params →
///   `Ok(EvmForms)`, the error arm having no branch.
/// Arrange: `build_evm_forms_constructor_params` at its defaults.
/// Act:     `EvmForms::try_new(params)`.
/// Assert:  the result is `Ok`.
#[test]
fn evm_forms_try_new_returns_the_forms() {
    // Arrange
    let params = build_evm_forms_constructor_params(Default::default());

    // Act
    let result = EvmForms::try_new(params);

    // Assert
    assert!(result.is_ok());
}
