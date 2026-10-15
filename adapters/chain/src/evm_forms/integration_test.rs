use super::interface::{
    EvmChainIdentifierForm, EvmEntitlementForm, EvmIdentityForm, EvmIntervalForm,
};
use super::mock::{
    EvmChainIdentifierFormConstructorParamsOverrides, EvmEntitlementFormConstructorParamsOverrides,
    EvmIdentityFormConstructorParamsOverrides, EvmIntervalFormConstructorParamsOverrides,
    build_evm_chain_identifier_form, build_evm_entitlement_form, build_evm_identity_form,
    build_evm_interval_form,
};
use encoding::{ICanonicalField, build_from_field_params, build_to_field_params};

/// Contract: the own entry `evm_identity_form_survives_its_field_round_trip`:
///   `EvmIdentityForm::to_field` over a form, then
///   `EvmIdentityForm::from_field` over the field it returns →
///   `Ok(FromFieldSuccessReturn { value })` with `value` equal to the
///   original form.
/// Arrange: `build_evm_identity_form` overriding `bytes` with the ascending
///   literal, and `build_to_field_params` and `build_from_field_params` at
///   their defaults; a rebuild from the builder default, from truncated
///   bytes, or from reordered bytes fails.
/// Act:     `EvmIdentityForm::from_field(params, field)` over the `field` of
///   the `Ok` arm `EvmIdentityForm::to_field(params, &form)` returns.
/// Assert:  the `value` of the `Ok` arm `from_field` returns equals the
///   original form by `assert_eq!`.
/// Boundary: the crate-internal calls `to_field` and `from_field`; the
///   chain of real functions `to_field`, `from_field`, and `try_new`.
/// Mocked:  none; the params come from `encoding`'s official builders and
///   the form from this module's builder, so the test proves the real chain.
#[test]
fn evm_identity_form_survives_its_field_round_trip() {
    // Arrange
    let bytes = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10, 0x11, 0x12, 0x13, 0x14,
    ];
    let form =
        build_evm_identity_form(EvmIdentityFormConstructorParamsOverrides { bytes: Some(bytes) });
    let to_field_params = build_to_field_params(Default::default());
    let from_field_params = build_from_field_params(Default::default());

    // Act
    let Ok(to_field_success) = EvmIdentityForm::to_field(to_field_params, &form);
    let result = EvmIdentityForm::from_field(from_field_params, to_field_success.field);

    // Assert
    assert_eq!(result.ok().map(|success| success.value), Some(form));
}

/// Contract: the own entry `evm_entitlement_form_survives_its_field_round_trip`:
///   `EvmEntitlementForm::to_field` over a form, then
///   `EvmEntitlementForm::from_field` over the field it returns →
///   `Ok(FromFieldSuccessReturn { value })` with `value` equal to the
///   original form.
/// Arrange: `build_evm_entitlement_form` overriding `bytes` with the
///   ascending literal, and `build_to_field_params` and
///   `build_from_field_params` at their defaults; a rebuild from the
///   builder default, from truncated bytes, or from reordered bytes fails.
/// Act:     `EvmEntitlementForm::from_field(params, field)` over the
///   `field` of the `Ok` arm `EvmEntitlementForm::to_field(params, &form)`
///   returns.
/// Assert:  the `value` of the `Ok` arm `from_field` returns equals the
///   original form by `assert_eq!`.
/// Boundary: the crate-internal calls `to_field` and `from_field`; the
///   chain of real functions `to_field`, `from_field`, and `try_new`.
/// Mocked:  none; the params come from `encoding`'s official builders and
///   the form from this module's builder, so the test proves the real chain.
#[test]
fn evm_entitlement_form_survives_its_field_round_trip() {
    // Arrange
    let bytes = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e,
        0x1f, 0x20,
    ];
    let form = build_evm_entitlement_form(EvmEntitlementFormConstructorParamsOverrides {
        bytes: Some(bytes),
    });
    let to_field_params = build_to_field_params(Default::default());
    let from_field_params = build_from_field_params(Default::default());

    // Act
    let Ok(to_field_success) = EvmEntitlementForm::to_field(to_field_params, &form);
    let result = EvmEntitlementForm::from_field(from_field_params, to_field_success.field);

    // Assert
    assert_eq!(result.ok().map(|success| success.value), Some(form));
}

/// Contract: the own entry `evm_interval_form_survives_its_field_round_trip`:
///   `EvmIntervalForm::to_field` over a form, then
///   `EvmIntervalForm::from_field` over the field it returns →
///   `Ok(FromFieldSuccessReturn { value })` with `value` equal to the
///   original form.
/// Arrange: `build_evm_interval_form` overriding `value` with
///   `0x0102_0304_0506_0708`, and `build_to_field_params` and
///   `build_from_field_params` at their defaults; a rebuild from the
///   builder default or from a value narrowed to 32 bits fails.
/// Act:     `EvmIntervalForm::from_field(params, field)` over the `field`
///   of the `Ok` arm `EvmIntervalForm::to_field(params, &form)` returns.
/// Assert:  the `value` of the `Ok` arm `from_field` returns equals the
///   original form by `assert_eq!`.
/// Boundary: the crate-internal calls `to_field` and `from_field`; the
///   chain of real functions `to_field`, `from_field`, and `try_new`.
/// Mocked:  none; the params come from `encoding`'s official builders and
///   the form from this module's builder, so the test proves the real chain.
#[test]
fn evm_interval_form_survives_its_field_round_trip() {
    // Arrange
    let value = 0x0102_0304_0506_0708;
    let form =
        build_evm_interval_form(EvmIntervalFormConstructorParamsOverrides { value: Some(value) });
    let to_field_params = build_to_field_params(Default::default());
    let from_field_params = build_from_field_params(Default::default());

    // Act
    let Ok(to_field_success) = EvmIntervalForm::to_field(to_field_params, &form);
    let result = EvmIntervalForm::from_field(from_field_params, to_field_success.field);

    // Assert
    assert_eq!(result.ok().map(|success| success.value), Some(form));
}

/// Contract: the own entry `evm_chain_identifier_form_survives_its_field_round_trip`:
///   `EvmChainIdentifierForm::to_field` over a form, then
///   `EvmChainIdentifierForm::from_field` over the field it returns →
///   `Ok(FromFieldSuccessReturn { value })` with `value` equal to the
///   original form.
/// Arrange: `build_evm_chain_identifier_form` overriding `bytes` with the
///   ascending literal, and `build_to_field_params` and
///   `build_from_field_params` at their defaults; a rebuild from the
///   builder default, from truncated bytes, or from reordered bytes fails.
/// Act:     `EvmChainIdentifierForm::from_field(params, field)` over the
///   `field` of the `Ok` arm
///   `EvmChainIdentifierForm::to_field(params, &form)` returns.
/// Assert:  the `value` of the `Ok` arm `from_field` returns equals the
///   original form by `assert_eq!`.
/// Boundary: the crate-internal calls `to_field` and `from_field`; the
///   chain of real functions `to_field`, `from_field`, and `try_new`.
/// Mocked:  none; the params come from `encoding`'s official builders and
///   the form from this module's builder, so the test proves the real chain.
#[test]
fn evm_chain_identifier_form_survives_its_field_round_trip() {
    // Arrange
    let bytes = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e,
        0x1f, 0x20,
    ];
    let form = build_evm_chain_identifier_form(EvmChainIdentifierFormConstructorParamsOverrides {
        bytes: Some(bytes),
    });
    let to_field_params = build_to_field_params(Default::default());
    let from_field_params = build_from_field_params(Default::default());

    // Act
    let Ok(to_field_success) = EvmChainIdentifierForm::to_field(to_field_params, &form);
    let result = EvmChainIdentifierForm::from_field(from_field_params, to_field_success.field);

    // Assert
    assert_eq!(result.ok().map(|success| success.value), Some(form));
}
