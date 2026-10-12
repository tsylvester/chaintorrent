#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use domain::build_derivation_context;
use encoding::{
    CreateEncodingParamsOverrides, DECLARED_ENCODING_SELECTIONS, build_create_encoding_deps,
    build_create_encoding_params, build_create_encoding_payload,
    build_derivation_context_description, build_mock_i_encoding_consumer, create_encoding,
};
use kdf::{
    CreateKeyDerivationParamsOverrides, DECLARED_KDF_SELECTIONS, DerivationPurpose,
    DeriveKeyParamsOverrides, DeriveKeyPayloadOverrides, IKeyDerivationAdapter, KdfIdentifier,
    REFERENCE_LENGTH, build_create_key_derivation_deps, build_create_key_derivation_params,
    build_create_key_derivation_payload, build_derive_key_params, build_derive_key_payload,
    create_key_derivation, expected_derived_key, reference_context,
};

/// Contract: `create_key_derivation` is called with the selection of a
///   concrete that declares `KdfIdentifier::Blake3KeyedV1` and that
///   identifier, and the adapter derives with `DerivationPurpose::
///   WrappingKey` and `REFERENCE_LENGTH` over the reference key material
///   and the reference context → the key equals `expected_derived_key`
///   over the wrapping-key purpose, the reference context, and
///   `REFERENCE_LENGTH`.
/// Arrange: for each case, the adapter from `create_key_derivation` over
///   `build_create_key_derivation_deps`,
///   `build_create_key_derivation_params` overriding `concrete` and
///   `identifier` with the case's selection and identifier, and
///   `build_create_key_derivation_payload`, bound through `expect`;
///   `build_derive_key_params` overriding the purpose with
///   `DerivationPurpose::WrappingKey` and the length with
///   `REFERENCE_LENGTH`; `build_derive_key_payload` with no override,
///   whose key material and context are the reference key material and
///   the reference context, so the oracle's inputs are the arrangement's
///   inputs and its output is not.
/// Act:     `adapter.derive_key(params, payload)` for the case.
/// Assert:  the success arm is bound through `expect`; the key's
///   `expose()` equals `expected_derived_key` over
///   `DerivationPurpose::WrappingKey`, `reference_context()`, and
///   `REFERENCE_LENGTH` by `assert_eq!`, the expectation computed by the
///   independent oracle and not read from the arrangement.
/// Boundary: the public call `create_key_derivation` and the returned
///   adapter's `derive_key`; the route `create_key_derivation`, the
///   selected concrete's constructor, and its `derive_key`.
/// Mocked:  nothing; `blake3`, `zeroize`, and `domain` run for real.
#[test]
fn every_concrete_declaring_the_blake3_keyed_identifier_derives_the_wrapping_key_over_the_reference_inputs()
 {
    // Arrange
    for selection in DECLARED_KDF_SELECTIONS
        .iter()
        .filter(|selection| selection.identifier == KdfIdentifier::Blake3KeyedV1)
    {
        let deps = build_create_key_derivation_deps(Default::default());
        let params = build_create_key_derivation_params(CreateKeyDerivationParamsOverrides {
            concrete: Some(selection.concrete),
            identifier: Some(selection.identifier),
        });
        let payload = build_create_key_derivation_payload(Default::default());
        let success = create_key_derivation(&deps, params, payload)
            .expect("an admitted selection returns the adapter");
        let adapter: &dyn IKeyDerivationAdapter = &*success.adapter;
        let params = build_derive_key_params(DeriveKeyParamsOverrides {
            purpose: Some(DerivationPurpose::WrappingKey),
            length: Some(REFERENCE_LENGTH),
        });
        let payload = build_derive_key_payload(Default::default());

        // Act
        let result = adapter.derive_key(params, payload);

        // Assert
        let success = result.expect("the derivation returns a key");
        assert_eq!(
            success.key.expose().as_slice(),
            expected_derived_key(
                DerivationPurpose::WrappingKey,
                reference_context(),
                REFERENCE_LENGTH,
            )
            .as_slice()
        );
    }
}

/// Contract: `create_key_derivation` is called with the selection of a
///   concrete that declares `KdfIdentifier::Blake3KeyedV1` and that
///   identifier, and the adapter derives with `DerivationPurpose::
///   WrappingKey` and `REFERENCE_LENGTH` over the reference key material
///   and the bytes `create_encoding` returns for a derivation context →
///   the key equals `expected_derived_key` over the wrapping-key purpose,
///   those bytes, and `REFERENCE_LENGTH`.
/// Arrange: for each case and each entry of `DECLARED_ENCODING_SELECTIONS`,
///   the context bytes obtained from `create_encoding` over
///   `build_create_encoding_deps` taking `build_mock_i_encoding_consumer`
///   over `build_derivation_context_description`, `build_derivation_context`
///   with no override, and no bytes, `build_create_encoding_params`
///   overriding `concrete` and `identifier` with the encoding entry's
///   selection and identifier, and `build_create_encoding_payload`, the
///   `encoded` of the output bound through `expect`; the adapter as in the
///   block above; `build_derive_key_params` as in the block above;
///   `build_derive_key_payload` with the context override set to a borrow
///   of those bytes, whose length and content differ from the reference
///   context's, so a derivation that frames the context by the reference
///   context's shape fails.
/// Act:     `adapter.derive_key(params, payload)` for the case and the
///   encoding entry.
/// Assert:  the success arm is bound through `expect`; the key's
///   `expose()` equals `expected_derived_key` over
///   `DerivationPurpose::WrappingKey`, the context bytes, and
///   `REFERENCE_LENGTH` by `assert_eq!`, the expectation computed by the
///   independent oracle and not read from the arrangement.
/// Boundary: the public calls `create_encoding`, `create_key_derivation`,
///   and the returned adapter's `derive_key`; the route `create_encoding`,
///   the selected encoding concrete's `encode`,
///   `DerivationContextDescription::to_fields`, and `domain`'s accessors,
///   then `create_key_derivation`, the selected concrete's constructor,
///   and its `derive_key`.
/// Mocked:  the encoding consumer, replaced by the official
///   `MockIEncodingConsumer`, so the test does not prove what a real
///   consumer does with the concrete; the description, `domain`, `blake3`,
///   and `zeroize` run for real.
#[test]
fn every_concrete_declaring_the_blake3_keyed_identifier_derives_the_wrapping_key_over_the_context_the_encoding_factory_encodes()
 {
    // Arrange
    for selection in DECLARED_KDF_SELECTIONS
        .iter()
        .filter(|selection| selection.identifier == KdfIdentifier::Blake3KeyedV1)
    {
        for encoding_selection in DECLARED_ENCODING_SELECTIONS {
            let description = build_derivation_context_description(Default::default());
            let consumer = build_mock_i_encoding_consumer(
                &description,
                build_derivation_context(Default::default()),
                Vec::new(),
            );
            let encoding_deps = build_create_encoding_deps(consumer);
            let encoding_params = build_create_encoding_params(CreateEncodingParamsOverrides {
                concrete: Some(encoding_selection.concrete),
                identifier: Some(encoding_selection.identifier),
            });
            let encoding_payload = build_create_encoding_payload(Default::default());
            let context = create_encoding(&encoding_deps, encoding_params, encoding_payload)
                .expect("an admitted encoding selection returns the consumer's output")
                .output
                .encoded
                .expect("the derivation context encodes")
                .bytes;
            let deps = build_create_key_derivation_deps(Default::default());
            let params = build_create_key_derivation_params(CreateKeyDerivationParamsOverrides {
                concrete: Some(selection.concrete),
                identifier: Some(selection.identifier),
            });
            let payload = build_create_key_derivation_payload(Default::default());
            let success = create_key_derivation(&deps, params, payload)
                .expect("an admitted selection returns the adapter");
            let adapter: &dyn IKeyDerivationAdapter = &*success.adapter;
            let params = build_derive_key_params(DeriveKeyParamsOverrides {
                purpose: Some(DerivationPurpose::WrappingKey),
                length: Some(REFERENCE_LENGTH),
            });
            let payload = build_derive_key_payload(DeriveKeyPayloadOverrides {
                context: Some(context.as_slice()),
                ..Default::default()
            });

            // Act
            let result = adapter.derive_key(params, payload);

            // Assert
            let success = result.expect("the derivation returns a key");
            assert_eq!(
                success.key.expose().as_slice(),
                expected_derived_key(
                    DerivationPurpose::WrappingKey,
                    context.as_slice(),
                    REFERENCE_LENGTH,
                )
                .as_slice()
            );
        }
    }
}
