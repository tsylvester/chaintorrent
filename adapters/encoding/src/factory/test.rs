#![allow(clippy::panic)]

use super::provides::{
    CreateEncodingErrorReturn, CreateEncodingParamsOverrides, EncodingIdentifier,
    MockIEncodingContract, build_canonical_fields, build_create_encoding_deps,
    build_create_encoding_params, build_create_encoding_payload, build_mock_i_encoding_consumer,
    create_encoding,
};

/// Contract: `create_encoding`, admitted: the named concrete's declared
///   identifier is `params.identifier` →
///   `Ok(CreateEncodingSuccessReturn { output })` holding the consumer's output.
/// Arrange: `build_create_encoding_deps` over the consumer,
///   `build_create_encoding_params` at its defaults, and
///   `build_create_encoding_payload`; the consumer's output carries the
///   declaration of the concrete it is handed, so a factory that hands another
///   concrete returns another declaration.
/// Act:     `create_encoding(&deps, params, payload)`.
/// Assert:  the `declaration` of the `output` of the `Ok` arm declares the
///   identifier `EncodingIdentifier::Mock` by `assert!`, the identifier the
///   family mock concrete declares.
#[test]
fn create_encoding_hands_the_selected_mock_concrete_to_the_consumer() {
    // Arrange
    let description = MockIEncodingContract;
    let consumer = build_mock_i_encoding_consumer(
        &description,
        build_canonical_fields(Default::default()),
        Vec::new(),
    );
    let deps = build_create_encoding_deps(consumer);
    let params = build_create_encoding_params(Default::default());
    let payload = build_create_encoding_payload(Default::default());

    // Act
    let result = create_encoding(&deps, params, payload);

    // Assert
    assert!(result.is_ok_and(|success| {
        success.output.declaration.identifier == EncodingIdentifier::Mock
    }));
}

/// Contract: `create_encoding`, unsupported identifier: the named concrete's
///   `DECLARATION.identifier` is not `params.identifier` →
///   `Err(CreateEncodingErrorReturn::UnsupportedEncodingIdentifier)`.
/// Arrange: `build_create_encoding_deps` over the consumer,
///   `build_create_encoding_params` overriding `identifier` with
///   `EncodingIdentifier::EthereumAbiV1` and keeping the default selection of
///   the mock concrete, which declares `EncodingIdentifier::Mock`, so the
///   identifier required differs from the identifier declared, and an
///   admission that ignores the identifier or compares it with a constant
///   fails; `build_create_encoding_payload`.
/// Act:     `create_encoding(&deps, params, payload)`.
/// Assert:  `result.err()` equals
///   `Some(CreateEncodingErrorReturn::UnsupportedEncodingIdentifier)` by
///   `assert_eq!`.
#[test]
fn create_encoding_refuses_a_concrete_that_declares_another_identifier() {
    // Arrange
    let description = MockIEncodingContract;
    let consumer = build_mock_i_encoding_consumer(
        &description,
        build_canonical_fields(Default::default()),
        Vec::new(),
    );
    let deps = build_create_encoding_deps(consumer);
    let params = build_create_encoding_params(CreateEncodingParamsOverrides {
        identifier: Some(EncodingIdentifier::EthereumAbiV1),
        ..Default::default()
    });
    let payload = build_create_encoding_payload(Default::default());

    // Act
    let result = create_encoding(&deps, params, payload);

    // Assert
    assert_eq!(
        result.err(),
        Some(CreateEncodingErrorReturn::UnsupportedEncodingIdentifier)
    );
}
