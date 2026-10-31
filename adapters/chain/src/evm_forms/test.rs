use super::interface::{
    EvmChainIdentifierForm, EvmEntitlementForm, EvmForms, EvmIdentityForm, EvmIntervalForm,
    EvmKeyIdentifierForm, EvmSignedIntentForm,
};
use super::mock::{
    EvmChainIdentifierFormConstructorParamsOverrides, EvmEntitlementFormConstructorParamsOverrides,
    EvmFormsBuildingConsumer, EvmIdentityFormConstructorParamsOverrides,
    EvmIntervalFormConstructorParamsOverrides, EvmKeyIdentifierFormConstructorParamsOverrides,
    EvmSignedIntentFormConstructorParamsOverrides, IntentCallOverrides, IntentFieldOverrides,
    SignedIntentInputsOverrides, build_derive_key_identifier_params,
    build_evm_chain_identifier_form, build_evm_chain_identifier_form_constructor_params,
    build_evm_entitlement_form, build_evm_entitlement_form_constructor_params, build_evm_forms,
    build_evm_forms_over, build_evm_identity_form, build_evm_identity_form_constructor_params,
    build_evm_interval_form, build_evm_interval_form_constructor_params,
    build_evm_key_identifier_form, build_evm_key_identifier_form_constructor_params,
    build_evm_signed_intent_form, build_evm_signed_intent_form_constructor_params,
    build_intent_call, build_intent_field, build_signed_intent_inputs, build_signed_intent_params,
    build_signed_intent_payload,
};
use crate::factory::provides::{
    CHAIN_FORMS_INTERFACE_VERSION, ChainFormsDeclarationOverrides, ChainFormsIdentifier,
    DeriveKeyIdentifierPayload, IChainForms, IIdentityForm, IntentMemberNames,
    SignedIntentErrorReturn, build_chain_forms_declaration,
};
use domain::{
    SignatureSchemeIdentifierConstructorParamsOverrides, build_signature_scheme_identifier,
};
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, EncodingIdentifier, FromFieldErrorReturn,
    ICanonicalField, build_create_encoding_deps, build_create_encoding_params,
    build_create_encoding_payload, build_from_field_params, build_to_field_params, create_encoding,
};
use sha3::{Digest, Keccak256};

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

/// Contract: `EvmKeyIdentifierForm::try_new(params)`, one branch: any
///   params → `Ok(EvmKeyIdentifierForm { bytes })`, the array moved from
///   the params.
/// Arrange: `build_evm_key_identifier_form_constructor_params` overriding
///   `bytes` with the ascending literal.
/// Act:     `EvmKeyIdentifierForm::try_new(params)`.
/// Assert:  the `bytes` of the form the `Ok` arm holds equal the
///   ascending literal by `assert_eq!`.
#[test]
fn evm_key_identifier_form_try_new_moves_the_bytes_of_its_params_into_the_form() {
    // Arrange
    let bytes = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E,
        0x1F, 0x20,
    ];
    let params = build_evm_key_identifier_form_constructor_params(
        EvmKeyIdentifierFormConstructorParamsOverrides { bytes: Some(bytes) },
    );

    // Act
    let result = EvmKeyIdentifierForm::try_new(params);

    // Assert
    let Ok(form) = result;
    assert_eq!(form.bytes, bytes);
}

/// Contract: `EvmKeyIdentifierForm::to_field`, one branch: any form →
///   `Ok(ToFieldSuccessReturn { field: CanonicalFieldValue::FixedBytes32(payload.bytes) })`.
/// Arrange: `build_evm_key_identifier_form` overriding `bytes` with the
///   ascending literal, and `build_to_field_params` at its defaults; a
///   field built from anything but the form's bytes, or in a variant
///   other than `FixedBytes32`, fails.
/// Act:     `EvmKeyIdentifierForm::to_field(params, &form)`.
/// Assert:  the `field` of the `Ok` arm equals
///   `CanonicalFieldValue::FixedBytes32` carrying the ascending literal
///   by `assert_eq!`.
#[test]
fn evm_key_identifier_form_yields_its_bytes_as_a_fixed_32_byte_field() {
    // Arrange
    let bytes = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E,
        0x1F, 0x20,
    ];
    let form = build_evm_key_identifier_form(EvmKeyIdentifierFormConstructorParamsOverrides {
        bytes: Some(bytes),
    });
    let params = build_to_field_params(Default::default());

    // Act
    let result = EvmKeyIdentifierForm::to_field(params, &form);

    // Assert
    let Ok(success) = result;
    assert_eq!(success.field, CanonicalFieldValue::FixedBytes32(bytes));
}

/// Contract: `EvmKeyIdentifierForm::from_field`, admitted: `payload` is
///   `CanonicalFieldValue::FixedBytes32(bytes)` → `Ok(FromFieldSuccessReturn { value })`,
///   the form built from `bytes` through `try_new`.
/// Arrange: the payload `CanonicalFieldValue::FixedBytes32` carrying the
///   ascending literal, and `build_from_field_params` at its defaults; a
///   form rebuilt from anything but the payload's bytes fails.
/// Act:     `EvmKeyIdentifierForm::from_field(params, payload)`.
/// Assert:  the `bytes` of the `value` the `Ok` arm holds equal the
///   ascending literal by `assert_eq!`.
#[test]
fn evm_key_identifier_form_is_rebuilt_from_a_fixed_32_byte_field() {
    // Arrange
    let bytes = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E,
        0x1F, 0x20,
    ];
    let payload = CanonicalFieldValue::FixedBytes32(bytes);
    let params = build_from_field_params(Default::default());

    // Act
    let result = EvmKeyIdentifierForm::from_field(params, payload);

    // Assert
    assert_eq!(result.ok().map(|success| success.value.bytes), Some(bytes));
}

/// Contract: `EvmKeyIdentifierForm::from_field`, wrong kind: `payload` is
///   not `CanonicalFieldValue::FixedBytes32` →
///   `Err(FromFieldErrorReturn::WrongKind { expected: CanonicalFieldKind::FixedBytes32 })`.
/// Arrange: the payload `CanonicalFieldValue::Unsigned256` carrying the
///   ascending literal, a kind that carries as many bytes as the form's
///   and is not the form's, and `build_from_field_params` at its
///   defaults; a check that compares only the width of the bytes fails.
/// Act:     `EvmKeyIdentifierForm::from_field(params, payload)`.
/// Assert:  `result.err()` equals `Some(FromFieldErrorReturn::WrongKind { expected: CanonicalFieldKind::FixedBytes32 })`
///   by `assert_eq!`.
#[test]
fn evm_key_identifier_form_refuses_a_field_of_another_kind() {
    // Arrange
    let payload = CanonicalFieldValue::Unsigned256([
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E,
        0x1F, 0x20,
    ]);
    let params = build_from_field_params(Default::default());

    // Act
    let result = EvmKeyIdentifierForm::from_field(params, payload);

    // Assert
    assert_eq!(
        result.err(),
        Some(FromFieldErrorReturn::WrongKind {
            expected: CanonicalFieldKind::FixedBytes32,
        })
    );
}

/// Contract: `EvmSignedIntentForm::try_new(params)`, one branch: any
///   params → `Ok(EvmSignedIntentForm { bytes })`, the array moved from
///   the params.
/// Arrange: `build_evm_signed_intent_form_constructor_params` overriding
///   `bytes` with the ascending literal.
/// Act:     `EvmSignedIntentForm::try_new(params)`.
/// Assert:  the `bytes` of the form the `Ok` arm holds equal the
///   ascending literal by `assert_eq!`.
#[test]
fn evm_signed_intent_form_try_new_moves_the_bytes_of_its_params_into_the_form() {
    // Arrange
    let bytes = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E,
        0x1F, 0x20,
    ];
    let params = build_evm_signed_intent_form_constructor_params(
        EvmSignedIntentFormConstructorParamsOverrides { bytes: Some(bytes) },
    );

    // Act
    let result = EvmSignedIntentForm::try_new(params);

    // Assert
    let Ok(form) = result;
    assert_eq!(form.bytes, bytes);
}

/// Contract: `EvmSignedIntentForm::to_field`, one branch: any form →
///   `Ok(ToFieldSuccessReturn { field: CanonicalFieldValue::FixedBytes32(payload.bytes) })`.
/// Arrange: `build_evm_signed_intent_form` overriding `bytes` with the
///   ascending literal, and `build_to_field_params` at its defaults; a
///   field built from anything but the form's bytes, or in a variant
///   other than `FixedBytes32`, fails.
/// Act:     `EvmSignedIntentForm::to_field(params, &form)`.
/// Assert:  the `field` of the `Ok` arm equals
///   `CanonicalFieldValue::FixedBytes32` carrying the ascending literal
///   by `assert_eq!`.
#[test]
fn evm_signed_intent_form_yields_its_bytes_as_a_fixed_32_byte_field() {
    // Arrange
    let bytes = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E,
        0x1F, 0x20,
    ];
    let form = build_evm_signed_intent_form(EvmSignedIntentFormConstructorParamsOverrides {
        bytes: Some(bytes),
    });
    let params = build_to_field_params(Default::default());

    // Act
    let result = EvmSignedIntentForm::to_field(params, &form);

    // Assert
    let Ok(success) = result;
    assert_eq!(success.field, CanonicalFieldValue::FixedBytes32(bytes));
}

/// Contract: `EvmSignedIntentForm::from_field`, admitted: `payload` is
///   `CanonicalFieldValue::FixedBytes32(bytes)` → `Ok(FromFieldSuccessReturn { value })`,
///   the form built from `bytes` through `try_new`.
/// Arrange: the payload `CanonicalFieldValue::FixedBytes32` carrying the
///   ascending literal, and `build_from_field_params` at its defaults; a
///   form rebuilt from anything but the payload's bytes fails.
/// Act:     `EvmSignedIntentForm::from_field(params, payload)`.
/// Assert:  the `bytes` of the `value` the `Ok` arm holds equal the
///   ascending literal by `assert_eq!`.
#[test]
fn evm_signed_intent_form_is_rebuilt_from_a_fixed_32_byte_field() {
    // Arrange
    let bytes = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E,
        0x1F, 0x20,
    ];
    let payload = CanonicalFieldValue::FixedBytes32(bytes);
    let params = build_from_field_params(Default::default());

    // Act
    let result = EvmSignedIntentForm::from_field(params, payload);

    // Assert
    assert_eq!(result.ok().map(|success| success.value.bytes), Some(bytes));
}

/// Contract: `EvmSignedIntentForm::from_field`, wrong kind: `payload` is
///   not `CanonicalFieldValue::FixedBytes32` →
///   `Err(FromFieldErrorReturn::WrongKind { expected: CanonicalFieldKind::FixedBytes32 })`.
/// Arrange: the payload `CanonicalFieldValue::Unsigned256` carrying the
///   ascending literal, and `build_from_field_params` at its defaults; a
///   check that compares only the width of the bytes fails.
/// Act:     `EvmSignedIntentForm::from_field(params, payload)`.
/// Assert:  `result.err()` equals `Some(FromFieldErrorReturn::WrongKind { expected: CanonicalFieldKind::FixedBytes32 })`
///   by `assert_eq!`.
#[test]
fn evm_signed_intent_form_refuses_a_field_of_another_kind() {
    // Arrange
    let payload = CanonicalFieldValue::Unsigned256([
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E,
        0x1F, 0x20,
    ]);
    let params = build_from_field_params(Default::default());

    // Act
    let result = EvmSignedIntentForm::from_field(params, payload);

    // Assert
    assert_eq!(
        result.err(),
        Some(FromFieldErrorReturn::WrongKind {
            expected: CanonicalFieldKind::FixedBytes32,
        })
    );
}

/// Contract: `ICanonicalField` for the form `IIdentityForm for
///   EvmIdentityForm` binds as `KeyIdentifier`: `KIND` is
///   `CanonicalFieldKind::FixedBytes32`.
/// Arrange: the kind is read through the trait's associated type, so a
///   binding to a form that declares another kind fails.
/// Act:     read `<<EvmIdentityForm as IIdentityForm>::KeyIdentifier as ICanonicalField>::KIND`.
/// Assert:  the kind equals `CanonicalFieldKind::FixedBytes32` by
///   `assert_eq!`.
#[test]
fn evm_forms_bind_the_key_identifier_to_the_fixed_32_byte_kind() {
    // Arrange

    // Act
    let kind = <<EvmIdentityForm as IIdentityForm>::KeyIdentifier as ICanonicalField>::KIND;

    // Assert
    assert_eq!(kind, CanonicalFieldKind::FixedBytes32);
}

/// Contract: `ICanonicalField` for the form `IChainForms for EvmForms`
///   binds as `SignedIntent`: `KIND` is `CanonicalFieldKind::FixedBytes32`.
/// Arrange: the kind is read through the trait's associated type, so a
///   binding to a form that declares another kind fails.
/// Act:     read `<<EvmForms as IChainForms>::SignedIntent as ICanonicalField>::KIND`.
/// Assert:  the kind equals `CanonicalFieldKind::FixedBytes32` by
///   `assert_eq!`.
#[test]
fn evm_forms_bind_the_signed_intent_to_the_fixed_32_byte_kind() {
    // Arrange

    // Act
    let kind = <<EvmForms as IChainForms>::SignedIntent as ICanonicalField>::KIND;

    // Assert
    assert_eq!(kind, CanonicalFieldKind::FixedBytes32);
}

/// Contract: `EvmForms::DECLARATION`, the inherent constant the
///   interaction spec states, every field.
/// Arrange: the expected declaration from `build_chain_forms_declaration`
///   overriding every field with the literal the interaction spec
///   states; no builder default is read.
/// Act:     read `EvmForms::DECLARATION`.
/// Assert:  the declaration equals the expected declaration by
///   `assert_eq!`.
#[test]
fn evm_forms_declares_its_identifier_versions_and_intent_values() {
    // Arrange
    let expected = build_chain_forms_declaration(ChainFormsDeclarationOverrides {
        identifier: Some(ChainFormsIdentifier::EvmV1),
        adapter_version: Some(1),
        interface_version: Some(CHAIN_FORMS_INTERFACE_VERSION),
        encoding_identifier: Some(EncodingIdentifier::EthereumAbiV1),
        key_identifier_domain_tag: Some(b"chaintorrent/evm/key-identifier/v1"),
        intent_domain_name: Some("ChainTorrent"),
        intent_domain_version: Some("1"),
        intent_struct_name: Some("Intent"),
        intent_member_names: Some(IntentMemberNames {
            identity: "identity",
            nonce: "nonce",
            expiry: "expiry",
            call: "call",
        }),
        nonce_kind: Some(CanonicalFieldKind::Unsigned64),
        expiry_kind: Some(CanonicalFieldKind::Unsigned64),
    });

    // Act
    let declaration = EvmForms::DECLARATION;

    // Assert
    assert_eq!(declaration, expected);
}

/// Contract: `IChainForms for EvmForms`:
///   `const DECLARATION: ChainFormsDeclaration = EvmForms::DECLARATION;`.
/// Arrange: the expected declaration as the previous block arranges it.
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
        encoding_identifier: Some(EncodingIdentifier::EthereumAbiV1),
        key_identifier_domain_tag: Some(b"chaintorrent/evm/key-identifier/v1"),
        intent_domain_name: Some("ChainTorrent"),
        intent_domain_version: Some("1"),
        intent_struct_name: Some("Intent"),
        intent_member_names: Some(IntentMemberNames {
            identity: "identity",
            nonce: "nonce",
            expiry: "expiry",
            call: "call",
        }),
        nonce_kind: Some(CanonicalFieldKind::Unsigned64),
        expiry_kind: Some(CanonicalFieldKind::Unsigned64),
    });

    // Act
    let declaration = <EvmForms as IChainForms>::DECLARATION;

    // Assert
    assert_eq!(declaration, expected);
}

/// Contract: `EvmForms::try_new(params)`: any params →
///   `Ok(EvmForms { type_names })` with `type_names` holding, for each of
///   the eight kinds, the name the encoder returned.
/// Arrange: `build_evm_forms_over` with `build_create_encoding_params` at
///   its defaults, the mock concrete, whose names differ from every ABI
///   name and from each other, so a constructor that holds a table of
///   its own, or one name for two kinds, fails.
/// Act:     read each field of the forms' `type_names` through `as_str`.
/// Assert:  the eight texts equal the eight mock names in the order
///   listed, by `assert_eq!`.
#[test]
fn evm_forms_try_new_reads_the_type_name_of_every_kind_from_its_encoder() {
    // Arrange
    let selection = build_create_encoding_params(Default::default());

    // Act
    let forms = build_evm_forms_over(selection);

    // Assert
    assert_eq!(forms.type_names.fixed_bytes_32.as_str(), "mockFixedBytes32");
    assert_eq!(forms.type_names.unsigned_16.as_str(), "mockUnsigned16");
    assert_eq!(forms.type_names.unsigned_32.as_str(), "mockUnsigned32");
    assert_eq!(forms.type_names.unsigned_64.as_str(), "mockUnsigned64");
    assert_eq!(forms.type_names.text.as_str(), "mockText");
    assert_eq!(forms.type_names.bytes.as_str(), "mockBytes");
    assert_eq!(forms.type_names.fixed_bytes_20.as_str(), "mockFixedBytes20");
    assert_eq!(forms.type_names.unsigned_256.as_str(), "mockUnsigned256");
}

/// Contract: `EvmIdentityForm::derive_key_identifier`, one branch: any
///   payload → `Ok(DeriveKeyIdentifierSuccessReturn { key_identifier })`
///   with the form's bytes the Keccak-256 digest of the declared tag, the
///   scheme's one byte, and the key's bytes, in that order.
/// Arrange: `build_derive_key_identifier_params`; the scheme from
///   `build_signature_scheme_identifier` overriding the identifier with
///   `0x5A`; the key the thirty-three ascending bytes; the expected
///   digest assembled in the block by hashing the literal tag, the byte
///   `0x5A`, and the key with `Keccak256`, so a different order, a
///   missing part, or a different hash fails.
/// Act:     `EvmIdentityForm::derive_key_identifier(params, payload)`.
/// Assert:  the `bytes` of the `key_identifier` the `Ok` arm holds equal
///   the expected digest by `assert_eq!`.
#[test]
fn evm_identity_form_derives_the_keccak_digest_of_the_tag_the_scheme_byte_and_the_key() {
    // Arrange
    let scheme =
        build_signature_scheme_identifier(SignatureSchemeIdentifierConstructorParamsOverrides {
            identifier: Some(0x5A),
        });
    let key = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E,
        0x1F, 0x20, 0x21,
    ];
    let mut hasher = Keccak256::new();
    Digest::update(&mut hasher, b"chaintorrent/evm/key-identifier/v1");
    Digest::update(&mut hasher, [0x5Au8]);
    Digest::update(&mut hasher, key);
    let expected: [u8; 32] = hasher.finalize().into();

    // Act
    let result = EvmIdentityForm::derive_key_identifier(
        build_derive_key_identifier_params(Default::default()),
        DeriveKeyIdentifierPayload { scheme, key: &key },
    );

    // Assert
    let Ok(success) = result;
    assert_eq!(success.key_identifier.bytes, expected);
}

/// Contract: `EvmIdentityForm::derive_key_identifier`: the same key under
///   another scheme identifier → another key identifier.
/// Arrange: two schemes from `build_signature_scheme_identifier`
///   overriding the identifier with `1` and `2`, and one key of the
///   thirty-three ascending bytes; a derivation that ignores the scheme
///   fails.
/// Act:     `derive_key_identifier` under each scheme.
/// Assert:  the two key identifiers' bytes differ by `assert_ne!`.
#[test]
fn evm_identity_form_derives_another_key_identifier_under_another_scheme() {
    // Arrange
    let scheme_one =
        build_signature_scheme_identifier(SignatureSchemeIdentifierConstructorParamsOverrides {
            identifier: Some(1),
        });
    let scheme_two =
        build_signature_scheme_identifier(SignatureSchemeIdentifierConstructorParamsOverrides {
            identifier: Some(2),
        });
    let key = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E,
        0x1F, 0x20, 0x21,
    ];

    // Act
    let first = EvmIdentityForm::derive_key_identifier(
        build_derive_key_identifier_params(Default::default()),
        DeriveKeyIdentifierPayload {
            scheme: scheme_one,
            key: &key,
        },
    );
    let second = EvmIdentityForm::derive_key_identifier(
        build_derive_key_identifier_params(Default::default()),
        DeriveKeyIdentifierPayload {
            scheme: scheme_two,
            key: &key,
        },
    );

    // Assert
    let Ok(first) = first;
    let Ok(second) = second;
    assert_ne!(first.key_identifier.bytes, second.key_identifier.bytes);
}

/// Contract: `EvmIdentityForm::derive_key_identifier`: another key under
///   one scheme → another key identifier.
/// Arrange: one scheme from `build_signature_scheme_identifier` at its
///   defaults, and two keys that differ in their last byte only; a
///   derivation that reads a prefix of the key fails.
/// Act:     `derive_key_identifier` over each key.
/// Assert:  the two key identifiers' bytes differ by `assert_ne!`.
#[test]
fn evm_identity_form_derives_another_key_identifier_for_another_key() {
    // Arrange
    let scheme = build_signature_scheme_identifier(Default::default());
    let key_one = [0x01; 33];
    let mut key_two = [0x01; 33];
    key_two[32] = 0x02;

    // Act
    let first = EvmIdentityForm::derive_key_identifier(
        build_derive_key_identifier_params(Default::default()),
        DeriveKeyIdentifierPayload {
            scheme,
            key: &key_one,
        },
    );
    let second = EvmIdentityForm::derive_key_identifier(
        build_derive_key_identifier_params(Default::default()),
        DeriveKeyIdentifierPayload {
            scheme,
            key: &key_two,
        },
    );

    // Assert
    let Ok(first) = first;
    let Ok(second) = second;
    assert_ne!(first.key_identifier.bytes, second.key_identifier.bytes);
}

/// Contract: `EvmIdentityForm::derive_key_identifier`: one key under one
///   scheme always yields one key identifier.
/// Arrange: one scheme and one key, derived twice with fresh params; a
///   derivation that reads a counter or the clock fails.
/// Act:     `derive_key_identifier` twice.
/// Assert:  the two key identifiers are equal by `assert_eq!`.
#[test]
fn evm_identity_form_derives_the_same_key_identifier_for_the_same_key_and_scheme() {
    // Arrange
    let scheme =
        build_signature_scheme_identifier(SignatureSchemeIdentifierConstructorParamsOverrides {
            identifier: Some(0x5A),
        });
    let key = [0x01; 33];

    // Act
    let first = EvmIdentityForm::derive_key_identifier(
        build_derive_key_identifier_params(Default::default()),
        DeriveKeyIdentifierPayload { scheme, key: &key },
    );
    let second = EvmIdentityForm::derive_key_identifier(
        build_derive_key_identifier_params(Default::default()),
        DeriveKeyIdentifierPayload { scheme, key: &key },
    );

    // Assert
    let Ok(first) = first;
    let Ok(second) = second;
    assert_eq!(first.key_identifier, second.key_identifier);
}

/// Contract: `EvmForms::signed_intent`, admitted →
///   `Ok(SignedIntentSuccessReturn { signed_intent })` with the bytes the
///   EIP-712 digest of the domain separator and the intent's hash the
///   interaction spec states.
/// Arrange: forms from `build_evm_forms`, so every type name is a `mock…`
///   name; inputs from `build_signed_intent_inputs` at its defaults; the
///   expected digest assembled in the block, piece by piece, from the
///   literal type strings, the words the spec states for each value, the
///   literal domain type string, and the prefix `0x19 0x01`, each hashed
///   with `Keccak256`; a word of the wrong width or padding, a wrong
///   member order, a missing prefix, or a name written in the subject
///   instead of read fails.
/// Act:     `forms.signed_intent(params, payload)`.
/// Assert:  the `bytes` of the `signed_intent` the `Ok` arm holds equal
///   the expected digest by `assert_eq!`.
#[test]
fn evm_forms_signed_intent_returns_the_eip712_digest_of_its_inputs() {
    // Arrange
    let forms = build_evm_forms(Default::default());
    let params = build_signed_intent_params(Default::default());
    let inputs = build_signed_intent_inputs(Default::default());
    let payload = build_signed_intent_payload(&inputs);

    let intent_type_string = "Intent(mockFixedBytes20 identity,mockUnsigned64 nonce,mockUnsigned64 expiry,lock call)lock(mockFixedBytes32 fixedBytes32,mockUnsigned16 unsigned16,mockUnsigned32 unsigned32,mockUnsigned64 unsigned64,mockText text,mockBytes bytes,mockFixedBytes20 fixedBytes20,mockUnsigned256 unsigned256)";
    let call_type_string = "lock(mockFixedBytes32 fixedBytes32,mockUnsigned16 unsigned16,mockUnsigned32 unsigned32,mockUnsigned64 unsigned64,mockText text,mockBytes bytes,mockFixedBytes20 fixedBytes20,mockUnsigned256 unsigned256)";
    let intent_type_digest = Keccak256::digest(intent_type_string);
    let call_type_digest = Keccak256::digest(call_type_string);

    let mut word_unsigned_16 = [0u8; 32];
    word_unsigned_16[30..].copy_from_slice(&0x0C02u16.to_be_bytes());
    let mut word_unsigned_32 = [0u8; 32];
    word_unsigned_32[28..].copy_from_slice(&0x0C03_0C03u32.to_be_bytes());
    let mut word_unsigned_64 = [0u8; 32];
    word_unsigned_64[24..].copy_from_slice(&0x0C04_0C04_0C04_0C04u64.to_be_bytes());
    let word_text = Keccak256::digest("c5-text");
    let word_bytes = Keccak256::digest([0xC6u8, 0xC7, 0xC8]);
    let mut word_fixed_bytes_20 = [0u8; 32];
    word_fixed_bytes_20[..20].copy_from_slice(&[0xC9; 20]);

    let mut preimage = Vec::new();
    preimage.extend_from_slice(call_type_digest.as_slice());
    preimage.extend_from_slice(&[0xC1; 32]);
    preimage.extend_from_slice(&word_unsigned_16);
    preimage.extend_from_slice(&word_unsigned_32);
    preimage.extend_from_slice(&word_unsigned_64);
    preimage.extend_from_slice(word_text.as_slice());
    preimage.extend_from_slice(word_bytes.as_slice());
    preimage.extend_from_slice(&word_fixed_bytes_20);
    preimage.extend_from_slice(&[0xCA; 32]);
    let call_hash = Keccak256::digest(&preimage);

    let mut word_identity = [0u8; 32];
    word_identity[..20].copy_from_slice(&[0xB3; 20]);
    let mut word_nonce = [0u8; 32];
    word_nonce[24..].copy_from_slice(&7u64.to_be_bytes());
    let mut word_expiry = [0u8; 32];
    word_expiry[24..].copy_from_slice(&0x0A0B_0C0D_0E0F_1011u64.to_be_bytes());

    let mut preimage = Vec::new();
    preimage.extend_from_slice(intent_type_digest.as_slice());
    preimage.extend_from_slice(&word_identity);
    preimage.extend_from_slice(&word_nonce);
    preimage.extend_from_slice(&word_expiry);
    preimage.extend_from_slice(call_hash.as_slice());
    let intent_hash = Keccak256::digest(&preimage);

    let domain_type_digest = Keccak256::digest(
        "EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)",
    );
    let name_digest = Keccak256::digest("ChainTorrent");
    let version_digest = Keccak256::digest("1");
    let mut contract_word = [0u8; 32];
    contract_word[12..].copy_from_slice(&[0xB2; 20]);

    let mut preimage = Vec::new();
    preimage.extend_from_slice(domain_type_digest.as_slice());
    preimage.extend_from_slice(name_digest.as_slice());
    preimage.extend_from_slice(version_digest.as_slice());
    preimage.extend_from_slice(&[0xB1; 32]);
    preimage.extend_from_slice(&contract_word);
    let domain_separator = Keccak256::digest(&preimage);

    let mut preimage = Vec::new();
    preimage.extend_from_slice(&[0x19, 0x01]);
    preimage.extend_from_slice(domain_separator.as_slice());
    preimage.extend_from_slice(intent_hash.as_slice());
    let expected: [u8; 32] = Keccak256::digest(&preimage).into();

    // Act
    let result = forms.signed_intent(params, payload);

    // Assert
    assert_eq!(
        result.ok().map(|success| success.signed_intent.bytes),
        Some(expected)
    );
}

/// Contract: `EvmForms::signed_intent`: the output changes with the
///   chain.
/// Arrange: baseline inputs, and inputs overriding `chain` with a chain
///   identifier form whose bytes differ in the last byte only.
/// Act:     `signed_intent` over each.
/// Assert:  the two outputs differ by `assert_ne!`.
#[test]
fn evm_forms_signed_intent_changes_with_the_chain() {
    // Arrange
    let forms = build_evm_forms(Default::default());
    let baseline = build_signed_intent_inputs(Default::default());
    let mut bytes = [0xB1; 32];
    bytes[31] = 0xB0;
    let changed = build_signed_intent_inputs(SignedIntentInputsOverrides {
        chain: Some(build_evm_chain_identifier_form(
            EvmChainIdentifierFormConstructorParamsOverrides { bytes: Some(bytes) },
        )),
        ..Default::default()
    });

    // Act
    let first = forms.signed_intent(
        build_signed_intent_params(Default::default()),
        build_signed_intent_payload(&baseline),
    );
    let second = forms.signed_intent(
        build_signed_intent_params(Default::default()),
        build_signed_intent_payload(&changed),
    );

    // Assert
    assert_ne!(
        first.ok().map(|success| success.signed_intent),
        second.ok().map(|success| success.signed_intent)
    );
}

/// Contract: `EvmForms::signed_intent`: the output changes with the
///   contract.
/// Arrange: baseline inputs, and inputs overriding `contract` with an
///   identity form whose bytes differ in the last byte only.
/// Act:     `signed_intent` over each.
/// Assert:  the two outputs differ by `assert_ne!`.
#[test]
fn evm_forms_signed_intent_changes_with_the_contract() {
    // Arrange
    let forms = build_evm_forms(Default::default());
    let baseline = build_signed_intent_inputs(Default::default());
    let mut bytes = [0xB2; 20];
    bytes[19] = 0xB0;
    let changed = build_signed_intent_inputs(SignedIntentInputsOverrides {
        contract: Some(build_evm_identity_form(
            EvmIdentityFormConstructorParamsOverrides { bytes: Some(bytes) },
        )),
        ..Default::default()
    });

    // Act
    let first = forms.signed_intent(
        build_signed_intent_params(Default::default()),
        build_signed_intent_payload(&baseline),
    );
    let second = forms.signed_intent(
        build_signed_intent_params(Default::default()),
        build_signed_intent_payload(&changed),
    );

    // Assert
    assert_ne!(
        first.ok().map(|success| success.signed_intent),
        second.ok().map(|success| success.signed_intent)
    );
}

/// Contract: `EvmForms::signed_intent`: the output changes with the
///   acting identity, distinct from the contract.
/// Arrange: baseline inputs, and inputs overriding `identity` with an
///   identity form whose bytes differ in the last byte only.
/// Act:     `signed_intent` over each.
/// Assert:  the two outputs differ by `assert_ne!`.
#[test]
fn evm_forms_signed_intent_changes_with_the_acting_identity() {
    // Arrange
    let forms = build_evm_forms(Default::default());
    let baseline = build_signed_intent_inputs(Default::default());
    let mut bytes = [0xB3; 20];
    bytes[19] = 0xB0;
    let changed = build_signed_intent_inputs(SignedIntentInputsOverrides {
        identity: Some(build_evm_identity_form(
            EvmIdentityFormConstructorParamsOverrides { bytes: Some(bytes) },
        )),
        ..Default::default()
    });

    // Act
    let first = forms.signed_intent(
        build_signed_intent_params(Default::default()),
        build_signed_intent_payload(&baseline),
    );
    let second = forms.signed_intent(
        build_signed_intent_params(Default::default()),
        build_signed_intent_payload(&changed),
    );

    // Assert
    assert_ne!(
        first.ok().map(|success| success.signed_intent),
        second.ok().map(|success| success.signed_intent)
    );
}

/// Contract: `EvmForms::signed_intent`: the output changes with the
///   nonce.
/// Arrange: baseline inputs, and inputs overriding `nonce` with
///   `CanonicalFieldValue::Unsigned64(8)`.
/// Act:     `signed_intent` over each.
/// Assert:  the two outputs differ by `assert_ne!`.
#[test]
fn evm_forms_signed_intent_changes_with_the_nonce() {
    // Arrange
    let forms = build_evm_forms(Default::default());
    let baseline = build_signed_intent_inputs(Default::default());
    let changed = build_signed_intent_inputs(SignedIntentInputsOverrides {
        nonce: Some(CanonicalFieldValue::Unsigned64(8)),
        ..Default::default()
    });

    // Act
    let first = forms.signed_intent(
        build_signed_intent_params(Default::default()),
        build_signed_intent_payload(&baseline),
    );
    let second = forms.signed_intent(
        build_signed_intent_params(Default::default()),
        build_signed_intent_payload(&changed),
    );

    // Assert
    assert_ne!(
        first.ok().map(|success| success.signed_intent),
        second.ok().map(|success| success.signed_intent)
    );
}

/// Contract: `EvmForms::signed_intent`: the output changes with the
///   expiry.
/// Arrange: baseline inputs, and inputs overriding `expiry` with a value
///   that differs in the lowest byte only.
/// Act:     `signed_intent` over each.
/// Assert:  the two outputs differ by `assert_ne!`.
#[test]
fn evm_forms_signed_intent_changes_with_the_expiry() {
    // Arrange
    let forms = build_evm_forms(Default::default());
    let baseline = build_signed_intent_inputs(Default::default());
    let changed = build_signed_intent_inputs(SignedIntentInputsOverrides {
        expiry: Some(CanonicalFieldValue::Unsigned64(0x0A0B_0C0D_0E0F_1010)),
        ..Default::default()
    });

    // Act
    let first = forms.signed_intent(
        build_signed_intent_params(Default::default()),
        build_signed_intent_payload(&baseline),
    );
    let second = forms.signed_intent(
        build_signed_intent_params(Default::default()),
        build_signed_intent_payload(&changed),
    );

    // Assert
    assert_ne!(
        first.ok().map(|success| success.signed_intent),
        second.ok().map(|success| success.signed_intent)
    );
}

/// Contract: `EvmForms::signed_intent`: the output changes with the
///   operation identifier.
/// Arrange: baseline inputs, and inputs overriding `call` with
///   `build_intent_call` overriding `operation` with `unlock`.
/// Act:     `signed_intent` over each.
/// Assert:  the two outputs differ by `assert_ne!`.
#[test]
fn evm_forms_signed_intent_changes_with_the_operation() {
    // Arrange
    let forms = build_evm_forms(Default::default());
    let baseline = build_signed_intent_inputs(Default::default());
    let changed = build_signed_intent_inputs(SignedIntentInputsOverrides {
        call: Some(build_intent_call(IntentCallOverrides {
            operation: Some("unlock".to_string()),
            ..Default::default()
        })),
        ..Default::default()
    });

    // Act
    let first = forms.signed_intent(
        build_signed_intent_params(Default::default()),
        build_signed_intent_payload(&baseline),
    );
    let second = forms.signed_intent(
        build_signed_intent_params(Default::default()),
        build_signed_intent_payload(&changed),
    );

    // Assert
    assert_ne!(
        first.ok().map(|success| success.signed_intent),
        second.ok().map(|success| success.signed_intent)
    );
}

/// Contract: `EvmForms::signed_intent`: the output changes with each call
///   field's value.
/// Arrange: baseline inputs, and inputs overriding `call` with a call
///   whose last field's value differs in its last byte only.
/// Act:     `signed_intent` over each.
/// Assert:  the two outputs differ by `assert_ne!`.
#[test]
fn evm_forms_signed_intent_changes_with_a_call_field_value() {
    // Arrange
    let forms = build_evm_forms(Default::default());
    let baseline = build_signed_intent_inputs(Default::default());
    let mut call = build_intent_call(Default::default());
    let mut bytes = [0xCA; 32];
    bytes[31] = 0xCB;
    call.fields[7].value = CanonicalFieldValue::Unsigned256(bytes);
    let changed = build_signed_intent_inputs(SignedIntentInputsOverrides {
        call: Some(call),
        ..Default::default()
    });

    // Act
    let first = forms.signed_intent(
        build_signed_intent_params(Default::default()),
        build_signed_intent_payload(&baseline),
    );
    let second = forms.signed_intent(
        build_signed_intent_params(Default::default()),
        build_signed_intent_payload(&changed),
    );

    // Assert
    assert_ne!(
        first.ok().map(|success| success.signed_intent),
        second.ok().map(|success| success.signed_intent)
    );
}

/// Contract: `EvmForms::signed_intent`: the output changes with each call
///   field's name.
/// Arrange: baseline inputs, and inputs overriding `call` with a call
///   whose last field is renamed `unsigned256b`.
/// Act:     `signed_intent` over each.
/// Assert:  the two outputs differ by `assert_ne!`.
#[test]
fn evm_forms_signed_intent_changes_with_a_call_field_name() {
    // Arrange
    let forms = build_evm_forms(Default::default());
    let baseline = build_signed_intent_inputs(Default::default());
    let mut call = build_intent_call(Default::default());
    call.fields[7].name = "unsigned256b".to_string();
    let changed = build_signed_intent_inputs(SignedIntentInputsOverrides {
        call: Some(call),
        ..Default::default()
    });

    // Act
    let first = forms.signed_intent(
        build_signed_intent_params(Default::default()),
        build_signed_intent_payload(&baseline),
    );
    let second = forms.signed_intent(
        build_signed_intent_params(Default::default()),
        build_signed_intent_payload(&changed),
    );

    // Assert
    assert_ne!(
        first.ok().map(|success| success.signed_intent),
        second.ok().map(|success| success.signed_intent)
    );
}

/// Contract: `signed_intent`, operation not an identifier →
///   `Err(SignedIntentErrorReturn::OperationNotIdentifier { operation })`.
/// Arrange: inputs overriding `call` with a call whose operation is
///   `lock-it`, a name holding a hyphen; a check that admits any
///   non-empty text fails.
/// Act:     `forms.signed_intent(params, payload)`.
/// Assert:  `result.err()` equals `Some(SignedIntentErrorReturn::OperationNotIdentifier { operation: "lock-it" })`
///   by `assert_eq!`.
#[test]
fn evm_forms_signed_intent_refuses_an_operation_that_is_not_an_identifier() {
    // Arrange
    let forms = build_evm_forms(Default::default());
    let params = build_signed_intent_params(Default::default());
    let inputs = build_signed_intent_inputs(SignedIntentInputsOverrides {
        call: Some(build_intent_call(IntentCallOverrides {
            operation: Some("lock-it".to_string()),
            ..Default::default()
        })),
        ..Default::default()
    });

    // Act
    let result = forms.signed_intent(params, build_signed_intent_payload(&inputs));

    // Assert
    assert_eq!(
        result.err(),
        Some(SignedIntentErrorReturn::OperationNotIdentifier {
            operation: "lock-it".to_string(),
        })
    );
}

/// Contract: `signed_intent`, operation not an identifier →
///   `Err(SignedIntentErrorReturn::OperationNotIdentifier { operation })`.
/// Arrange: inputs overriding `call` with a call whose operation is
///   `2lock`; a check that tests only the characters after the first
///   fails.
/// Act:     `forms.signed_intent(params, payload)`.
/// Assert:  `result.err()` equals `Some(SignedIntentErrorReturn::OperationNotIdentifier { operation: "2lock" })`
///   by `assert_eq!`.
#[test]
fn evm_forms_signed_intent_refuses_an_operation_beginning_with_a_digit() {
    // Arrange
    let forms = build_evm_forms(Default::default());
    let params = build_signed_intent_params(Default::default());
    let inputs = build_signed_intent_inputs(SignedIntentInputsOverrides {
        call: Some(build_intent_call(IntentCallOverrides {
            operation: Some("2lock".to_string()),
            ..Default::default()
        })),
        ..Default::default()
    });

    // Act
    let result = forms.signed_intent(params, build_signed_intent_payload(&inputs));

    // Assert
    assert_eq!(
        result.err(),
        Some(SignedIntentErrorReturn::OperationNotIdentifier {
            operation: "2lock".to_string(),
        })
    );
}

/// Contract: `signed_intent`, operation named as the struct →
///   `Err(SignedIntentErrorReturn::OperationIsStructName { operation })`.
/// Arrange: inputs overriding `call` with a call whose operation is
///   `Intent`, the declared struct name, an admitted identifier.
/// Act:     `forms.signed_intent(params, payload)`.
/// Assert:  `result.err()` equals `Some(SignedIntentErrorReturn::OperationIsStructName { operation: "Intent" })`
///   by `assert_eq!`.
#[test]
fn evm_forms_signed_intent_refuses_an_operation_named_as_the_struct() {
    // Arrange
    let forms = build_evm_forms(Default::default());
    let params = build_signed_intent_params(Default::default());
    let inputs = build_signed_intent_inputs(SignedIntentInputsOverrides {
        call: Some(build_intent_call(IntentCallOverrides {
            operation: Some("Intent".to_string()),
            ..Default::default()
        })),
        ..Default::default()
    });

    // Act
    let result = forms.signed_intent(params, build_signed_intent_payload(&inputs));

    // Assert
    assert_eq!(
        result.err(),
        Some(SignedIntentErrorReturn::OperationIsStructName {
            operation: "Intent".to_string(),
        })
    );
}

/// Contract: `signed_intent`, field not an identifier →
///   `Err(SignedIntentErrorReturn::FieldNotIdentifier { name })`.
/// Arrange: inputs overriding `call` with a call whose second field is
///   named `a b`, after an admitted first field.
/// Act:     `forms.signed_intent(params, payload)`.
/// Assert:  `result.err()` equals `Some(SignedIntentErrorReturn::FieldNotIdentifier { name: "a b" })`
///   by `assert_eq!`.
#[test]
fn evm_forms_signed_intent_refuses_a_field_name_that_is_not_an_identifier() {
    // Arrange
    let forms = build_evm_forms(Default::default());
    let params = build_signed_intent_params(Default::default());
    let inputs = build_signed_intent_inputs(SignedIntentInputsOverrides {
        call: Some(build_intent_call(IntentCallOverrides {
            fields: Some(vec![
                build_intent_field(Default::default()),
                build_intent_field(IntentFieldOverrides {
                    name: Some("a b".to_string()),
                    ..Default::default()
                }),
            ]),
            ..Default::default()
        })),
        ..Default::default()
    });

    // Act
    let result = forms.signed_intent(params, build_signed_intent_payload(&inputs));

    // Assert
    assert_eq!(
        result.err(),
        Some(SignedIntentErrorReturn::FieldNotIdentifier {
            name: "a b".to_string(),
        })
    );
}

/// Contract: `signed_intent`, field repeated →
///   `Err(SignedIntentErrorReturn::FieldRepeated { name })`.
/// Arrange: inputs overriding `call` with a call whose first and third
///   fields are both named `amount`; a check that compares only
///   neighbours fails.
/// Act:     `forms.signed_intent(params, payload)`.
/// Assert:  `result.err()` equals `Some(SignedIntentErrorReturn::FieldRepeated { name: "amount" })`
///   by `assert_eq!`.
#[test]
fn evm_forms_signed_intent_refuses_a_repeated_field_name() {
    // Arrange
    let forms = build_evm_forms(Default::default());
    let params = build_signed_intent_params(Default::default());
    let inputs = build_signed_intent_inputs(SignedIntentInputsOverrides {
        call: Some(build_intent_call(IntentCallOverrides {
            fields: Some(vec![
                build_intent_field(Default::default()),
                build_intent_field(IntentFieldOverrides {
                    name: Some("other".to_string()),
                    ..Default::default()
                }),
                build_intent_field(Default::default()),
            ]),
            ..Default::default()
        })),
        ..Default::default()
    });

    // Act
    let result = forms.signed_intent(params, build_signed_intent_payload(&inputs));

    // Assert
    assert_eq!(
        result.err(),
        Some(SignedIntentErrorReturn::FieldRepeated {
            name: "amount".to_string(),
        })
    );
}

/// Contract: `signed_intent`, nonce of another kind →
///   `Err(SignedIntentErrorReturn::NonceKind { expected, actual })`.
/// Arrange: inputs overriding `nonce` with
///   `CanonicalFieldValue::Unsigned32(7)`, an unsigned kind that is not
///   the declared one.
/// Act:     `forms.signed_intent(params, payload)`.
/// Assert:  `result.err()` equals `Some(SignedIntentErrorReturn::NonceKind { expected: Unsigned64, actual: Unsigned32 })`
///   by `assert_eq!`.
#[test]
fn evm_forms_signed_intent_refuses_a_nonce_of_another_kind() {
    // Arrange
    let forms = build_evm_forms(Default::default());
    let params = build_signed_intent_params(Default::default());
    let inputs = build_signed_intent_inputs(SignedIntentInputsOverrides {
        nonce: Some(CanonicalFieldValue::Unsigned32(7)),
        ..Default::default()
    });

    // Act
    let result = forms.signed_intent(params, build_signed_intent_payload(&inputs));

    // Assert
    assert_eq!(
        result.err(),
        Some(SignedIntentErrorReturn::NonceKind {
            expected: CanonicalFieldKind::Unsigned64,
            actual: CanonicalFieldKind::Unsigned32,
        })
    );
}

/// Contract: `signed_intent`, expiry of another kind →
///   `Err(SignedIntentErrorReturn::ExpiryKind { expected, actual })`.
/// Arrange: inputs overriding `expiry` with
///   `CanonicalFieldValue::Unsigned256` of thirty-two ascending bytes.
/// Act:     `forms.signed_intent(params, payload)`.
/// Assert:  `result.err()` equals `Some(SignedIntentErrorReturn::ExpiryKind { expected: Unsigned64, actual: Unsigned256 })`
///   by `assert_eq!`.
#[test]
fn evm_forms_signed_intent_refuses_an_expiry_of_another_kind() {
    // Arrange
    let forms = build_evm_forms(Default::default());
    let params = build_signed_intent_params(Default::default());
    let inputs = build_signed_intent_inputs(SignedIntentInputsOverrides {
        expiry: Some(CanonicalFieldValue::Unsigned256([
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C,
            0x1D, 0x1E, 0x1F, 0x20,
        ])),
        ..Default::default()
    });

    // Act
    let result = forms.signed_intent(params, build_signed_intent_payload(&inputs));

    // Assert
    assert_eq!(
        result.err(),
        Some(SignedIntentErrorReturn::ExpiryKind {
            expected: CanonicalFieldKind::Unsigned64,
            actual: CanonicalFieldKind::Unsigned256,
        })
    );
}

/// Contract: `signed_intent`, ordering: the operation checks precede the
///   field checks, which precede the nonce check, which precedes the
///   expiry check.
/// Arrange: inputs whose operation is `lock-it`, whose second field is
///   named `a b`, and whose nonce and expiry are of another kind; then
///   the same inputs with the operation admitted, then with the fields
///   admitted, then with the nonce admitted; a decision in another order
///   reports another refusal.
/// Act:     `forms.signed_intent(params, payload)` over each of the four
///   inputs.
/// Assert:  the four `result.err()` values equal, in order,
///   `OperationNotIdentifier`, `FieldNotIdentifier`, `NonceKind`, and
///   `ExpiryKind` by `assert_eq!`.
#[test]
fn evm_forms_signed_intent_decides_the_operation_before_the_fields_and_the_fields_before_the_nonce()
{
    // Arrange
    let forms = build_evm_forms(Default::default());
    let rejected_fields = vec![
        build_intent_field(Default::default()),
        build_intent_field(IntentFieldOverrides {
            name: Some("a b".to_string()),
            ..Default::default()
        }),
    ];
    let admitted_fields = vec![build_intent_field(Default::default())];
    let mut order_cases = Vec::new();
    order_cases.push(build_signed_intent_inputs(SignedIntentInputsOverrides {
        call: Some(build_intent_call(IntentCallOverrides {
            operation: Some("lock-it".to_string()),
            fields: Some(rejected_fields.clone()),
        })),
        nonce: Some(CanonicalFieldValue::Unsigned32(7)),
        expiry: Some(CanonicalFieldValue::Unsigned32(8)),
        ..Default::default()
    }));
    order_cases.push(build_signed_intent_inputs(SignedIntentInputsOverrides {
        call: Some(build_intent_call(IntentCallOverrides {
            fields: Some(rejected_fields),
            ..Default::default()
        })),
        nonce: Some(CanonicalFieldValue::Unsigned32(7)),
        expiry: Some(CanonicalFieldValue::Unsigned32(8)),
        ..Default::default()
    }));
    order_cases.push(build_signed_intent_inputs(SignedIntentInputsOverrides {
        call: Some(build_intent_call(IntentCallOverrides {
            fields: Some(admitted_fields),
            ..Default::default()
        })),
        nonce: Some(CanonicalFieldValue::Unsigned32(7)),
        expiry: Some(CanonicalFieldValue::Unsigned32(8)),
        ..Default::default()
    }));
    order_cases.push(build_signed_intent_inputs(SignedIntentInputsOverrides {
        call: Some(build_intent_call(Default::default())),
        expiry: Some(CanonicalFieldValue::Unsigned32(8)),
        ..Default::default()
    }));

    // Act
    let errors: Vec<_> = order_cases
        .iter()
        .map(|inputs| {
            forms
                .signed_intent(
                    build_signed_intent_params(Default::default()),
                    build_signed_intent_payload(inputs),
                )
                .err()
        })
        .collect();

    // Assert
    assert_eq!(
        errors,
        vec![
            Some(SignedIntentErrorReturn::OperationNotIdentifier {
                operation: "lock-it".to_string(),
            }),
            Some(SignedIntentErrorReturn::FieldNotIdentifier {
                name: "a b".to_string(),
            }),
            Some(SignedIntentErrorReturn::NonceKind {
                expected: CanonicalFieldKind::Unsigned64,
                actual: CanonicalFieldKind::Unsigned32,
            }),
            Some(SignedIntentErrorReturn::ExpiryKind {
                expected: CanonicalFieldKind::Unsigned64,
                actual: CanonicalFieldKind::Unsigned32,
            }),
        ]
    );
}

/// Contract: `EvmForms::try_new(params)`, one branch: any params →
///   `Ok(EvmForms)`, the error arm having no branch.
/// Arrange: `create_encoding` through `build_create_encoding_deps` over
///   `EvmFormsBuildingConsumer`, the default selection, and
///   `build_create_encoding_payload`.
/// Act:     the factory call, which constructs the forms inside the
///   consumer.
/// Assert:  the factory's `Ok` arm holds an `Ok` constructor result.
#[test]
fn evm_forms_try_new_returns_the_forms() {
    // Arrange
    let deps = build_create_encoding_deps(EvmFormsBuildingConsumer);
    let params = build_create_encoding_params(Default::default());
    let payload = build_create_encoding_payload(Default::default());

    // Act
    let result = create_encoding(&deps, params, payload);

    // Assert
    assert!(matches!(result, Ok(success) if success.output.is_ok()));
}
