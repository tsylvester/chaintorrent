use super::interface::{
    EvmChainIdentifierForm, EvmEntitlementForm, EvmIdentityForm, EvmIntervalForm,
    EvmKeyIdentifierForm, EvmSignedIntentForm,
};
use super::mock::{
    EvmChainIdentifierFormConstructorParamsOverrides, EvmEntitlementFormConstructorParamsOverrides,
    EvmIdentityFormConstructorParamsOverrides, EvmIntervalFormConstructorParamsOverrides,
    IntentCallOverrides, SignedIntentInputsOverrides, build_derive_key_identifier_params,
    build_evm_chain_identifier_form, build_evm_entitlement_form, build_evm_forms,
    build_evm_forms_over, build_evm_identity_form, build_evm_interval_form, build_intent_call,
    build_signed_intent_inputs, build_signed_intent_params, build_signed_intent_payload,
};
use crate::factory::provides::{DeriveKeyIdentifierPayload, IChainForms, IIdentityForm};
use domain::{
    SignatureSchemeIdentifierConstructorParamsOverrides, build_signature_scheme_identifier,
};
use encoding::{
    CanonicalFieldValue, CreateEncodingParams, DECLARED_ENCODING_SELECTIONS, EncodingIdentifier,
    ICanonicalField, build_create_encoding_params, build_from_field_params, build_to_field_params,
};
use sha3::{Digest, Keccak256};

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

/// Contract: the own entry
///   `evm_key_identifier_derived_through_the_identity_form_survives_its_field_round_trip`:
///   `EvmIdentityForm::derive_key_identifier` over a scheme and a key,
///   then `EvmKeyIdentifierForm::to_field` over the result and
///   `EvmKeyIdentifierForm::from_field` over the field →
///   `Ok(FromFieldSuccessReturn { value })` with `value` equal to the
///   derived key identifier, and the field equal to
///   `CanonicalFieldValue::FixedBytes32` of the Keccak-256 digest of the
///   declared tag, the scheme byte, and the key.
/// Arrange: the scheme from `build_signature_scheme_identifier`
///   overriding the identifier with `0x5A`, the key the thirty-three
///   ascending bytes, the expected digest assembled in the block with
///   `Keccak256` from the literal tag, `0x5A`, and the key, and
///   `build_to_field_params` and `build_from_field_params` at their
///   defaults; a derivation that ignores the scheme or the key, or a
///   rebuild from a default, fails.
/// Act:     `EvmKeyIdentifierForm::from_field(params, field)` over the
///   `field` of the `Ok` arm `EvmKeyIdentifierForm::to_field(params,
///   &key_identifier)` returns for the `key_identifier` of the `Ok` arm
///   `derive_key_identifier` returns.
/// Assert:  the `value` of the `Ok` arm equals the derived key
///   identifier, and the field equals
///   `CanonicalFieldValue::FixedBytes32` carrying the expected digest, by
///   `assert_eq!`.
/// Boundary: the crate-internal calls `derive_key_identifier`,
///   `to_field`, and `from_field`; the chain of real functions
///   `derive_key_identifier`, the key-identifier form's `try_new`,
///   `to_field`, and `from_field`.
/// Mocked:  none; the params come from `encoding`'s official builders and
///   the scheme from `domain`'s official builder.
#[test]
fn evm_key_identifier_derived_through_the_identity_form_survives_its_field_round_trip() {
    // Arrange
    let scheme =
        build_signature_scheme_identifier(SignatureSchemeIdentifierConstructorParamsOverrides {
            identifier: Some(0x5A),
        });
    let key = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e,
        0x1f, 0x20, 0x21,
    ];
    let mut hasher = Keccak256::new();
    Digest::update(&mut hasher, b"chaintorrent/evm/key-identifier/v1");
    Digest::update(&mut hasher, [0x5Au8]);
    Digest::update(&mut hasher, key);
    let expected: [u8; 32] = hasher.finalize().into();
    let to_field_params = build_to_field_params(Default::default());
    let from_field_params = build_from_field_params(Default::default());

    // Act
    let Ok(derived) = EvmIdentityForm::derive_key_identifier(
        build_derive_key_identifier_params(Default::default()),
        DeriveKeyIdentifierPayload { scheme, key: &key },
    );
    let Ok(field) = EvmKeyIdentifierForm::to_field(to_field_params, &derived.key_identifier);
    let result = EvmKeyIdentifierForm::from_field(from_field_params, field.field.clone());

    // Assert
    assert_eq!(field.field, CanonicalFieldValue::FixedBytes32(expected));
    assert_eq!(
        result.ok().map(|success| success.value),
        Some(derived.key_identifier)
    );
}

/// Contract: the own entry `evm_signed_intent_survives_its_field_round_trip`:
///   `EvmForms::signed_intent` over the builder's inputs, then
///   `EvmSignedIntentForm::to_field` over the intent and
///   `EvmSignedIntentForm::from_field` over the field →
///   `Ok(FromFieldSuccessReturn { value })` with `value` equal to the
///   intent.
/// Arrange: forms from `build_evm_forms`; inputs from
///   `build_signed_intent_inputs` overriding `chain`, `contract`,
///   `identity`, `nonce`, `expiry`, and `call` with values that differ
///   from the defaults; `build_to_field_params` and
///   `build_from_field_params` at their defaults; a digest not built
///   from the inputs, or a rebuild from a default, fails.
/// Act:     `EvmSignedIntentForm::from_field(params, field)` over the
///   `field` of the `Ok` arm `EvmSignedIntentForm::to_field(params,
///   &signed_intent)` returns for the `signed_intent` of the `Ok` arm
///   `signed_intent` returns.
/// Assert:  the `value` of the `Ok` arm equals the intent by `assert_eq!`.
/// Boundary: the crate-internal calls `signed_intent`, `to_field`, and
///   `from_field`; the chain of real functions `signed_intent`, the
///   signed-intent form's `try_new`, `to_field`, and `from_field`.
/// Mocked:  the encoding family's mock concrete under the default
///   selection stands for no real concrete; nothing else.
#[test]
fn evm_signed_intent_survives_its_field_round_trip() {
    // Arrange
    let forms = build_evm_forms(Default::default());
    let inputs = build_signed_intent_inputs(SignedIntentInputsOverrides {
        chain: Some(build_evm_chain_identifier_form(
            EvmChainIdentifierFormConstructorParamsOverrides {
                bytes: Some([0xD1; 32]),
            },
        )),
        contract: Some(build_evm_identity_form(
            EvmIdentityFormConstructorParamsOverrides {
                bytes: Some([0xD2; 20]),
            },
        )),
        identity: Some(build_evm_identity_form(
            EvmIdentityFormConstructorParamsOverrides {
                bytes: Some([0xD3; 20]),
            },
        )),
        nonce: Some(CanonicalFieldValue::Unsigned64(9)),
        expiry: Some(CanonicalFieldValue::Unsigned64(0x0D04_0D04_0D04_0D04)),
        call: Some(build_intent_call(IntentCallOverrides {
            operation: Some("unlock".to_string()),
            ..Default::default()
        })),
    });
    let to_field_params = build_to_field_params(Default::default());
    let from_field_params = build_from_field_params(Default::default());

    // Act
    let result = forms.signed_intent(
        build_signed_intent_params(Default::default()),
        build_signed_intent_payload(&inputs),
    );
    assert!(result.is_ok());
    let intent = result.ok().map(|success| success.signed_intent);
    let field = intent.and_then(|intent| {
        EvmSignedIntentForm::to_field(to_field_params, &intent)
            .ok()
            .map(|success| success.field)
    });
    let rebuilt = field.and_then(|field| {
        EvmSignedIntentForm::from_field(from_field_params, field)
            .ok()
            .map(|success| success.value)
    });

    // Assert
    assert_eq!(rebuilt, intent);
}

/// Assembles the EIP-712 digest the interaction spec states for the
///   builder's default inputs, from the literal type strings the
///   selected encoder returns.
fn expected_signed_intent_bytes(intent_type_string: &str, call_type_string: &str) -> [u8; 32] {
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
    Keccak256::digest(&preimage).into()
}

/// Contract: the own entry
///   `evm_signed_intent_types_its_members_by_the_names_its_encoder_returns`:
///   `create_encoding` with a selection, its consumer constructing
///   `EvmForms::try_new` over the encoder it is handed and calling
///   `signed_intent` over inputs whose call holds one field of each
///   canonical field kind → the intent equals the EIP-712 digest
///   assembled from the names that selection's concrete returns.
/// Arrange: for the Ethereum ABI selection, the member of
///   `DECLARED_ENCODING_SELECTIONS` that declares
///   `EncodingIdentifier::EthereumAbiV1`, the expected digest assembled
///   in the block from the literal type strings written from the ABI
///   specification and not read from the arrangement; for the mock
///   selection, `build_create_encoding_params` at its defaults and the
///   type strings written with the eight `mock…` names; the inputs from
///   `build_signed_intent_inputs` at their defaults; a subject that
///   names a type itself, reads another encoder's names, or names the
///   fixed twenty-byte kind `address` fails.
/// Act:     `create_encoding(&deps, selection, payload)` over
///   `EvmFormsBuildingConsumer`, then `signed_intent` over the forms it
///   built.
/// Assert:  for each selection the factory result is `Ok`, and the
///   `bytes` of the intent equal the expected digest by `assert_eq!`.
/// Boundary: the crate-internal calls `create_encoding`, the selected
///   concrete's `type_name`, `EvmForms::try_new`, and `signed_intent`;
///   the chain of real functions is those and the signed-intent form's
///   `try_new`.
/// Mocked:  the encoding family's mock concrete under the mock selection
///   stands for no real concrete, so that run proves only that the forms
///   read the names an encoder returns; the Ethereum ABI run uses the
///   real concrete.
#[test]
fn evm_signed_intent_types_its_members_by_the_names_its_encoder_returns() {
    // Arrange
    let inputs = build_signed_intent_inputs(Default::default());

    for selection in DECLARED_ENCODING_SELECTIONS {
        assert_eq!(selection.identifier, EncodingIdentifier::EthereumAbiV1);
        let forms = build_evm_forms_over(CreateEncodingParams {
            concrete: selection.concrete,
            identifier: selection.identifier,
        });

        // Act
        let result = forms.signed_intent(
            build_signed_intent_params(Default::default()),
            build_signed_intent_payload(&inputs),
        );
        let expected = expected_signed_intent_bytes(
            "Intent(bytes20 identity,uint64 nonce,uint64 expiry,lock call)lock(bytes32 fixedBytes32,uint16 unsigned16,uint32 unsigned32,uint64 unsigned64,string text,bytes bytes,bytes20 fixedBytes20,uint256 unsigned256)",
            "lock(bytes32 fixedBytes32,uint16 unsigned16,uint32 unsigned32,uint64 unsigned64,string text,bytes bytes,bytes20 fixedBytes20,uint256 unsigned256)",
        );

        // Assert
        assert_eq!(
            result.ok().map(|success| success.signed_intent.bytes),
            Some(expected)
        );
    }

    let forms = build_evm_forms_over(build_create_encoding_params(Default::default()));

    // Act
    let result = forms.signed_intent(
        build_signed_intent_params(Default::default()),
        build_signed_intent_payload(&inputs),
    );
    let expected = expected_signed_intent_bytes(
        "Intent(mockFixedBytes20 identity,mockUnsigned64 nonce,mockUnsigned64 expiry,lock call)lock(mockFixedBytes32 fixedBytes32,mockUnsigned16 unsigned16,mockUnsigned32 unsigned32,mockUnsigned64 unsigned64,mockText text,mockBytes bytes,mockFixedBytes20 fixedBytes20,mockUnsigned256 unsigned256)",
        "lock(mockFixedBytes32 fixedBytes32,mockUnsigned16 unsigned16,mockUnsigned32 unsigned32,mockUnsigned64 unsigned64,mockText text,mockBytes bytes,mockFixedBytes20 fixedBytes20,mockUnsigned256 unsigned256)",
    );

    // Assert
    assert_eq!(
        result.ok().map(|success| success.signed_intent.bytes),
        Some(expected)
    );
}
