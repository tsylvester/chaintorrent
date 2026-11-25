#![allow(clippy::expect_used)]

use super::provides::{
    CreateKeyDerivationParamsOverrides, DeriveKeyErrorReturn, IKeyDerivationAdapter, KdfConcrete,
    MockIKeyDerivationAdapterDeriveKeyErrorReturn, MockIKeyDerivationAdapterFailureMode,
    build_create_key_derivation_deps, build_create_key_derivation_params,
    build_create_key_derivation_payload, build_derive_key_params, build_derive_key_payload,
    create_key_derivation,
};

/// Contract: `create_key_derivation`, mock member: `params.selection` is
///   `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable)`
///   → `Ok(CreateKeyDerivationSuccessReturn { adapter })` whose `derive_key`
///   returns
///   `Err(DeriveKeyErrorReturn::Mock(MockIKeyDerivationAdapterDeriveKeyErrorReturn::KeyMaterialLengthUnrepresentable { length }))`.
/// Collaborators: the family's mock concrete, reached through
///   `create_key_derivation` by `build_create_key_derivation_params` with the
///   `selection` override; `build_create_key_derivation_deps` and
///   `build_create_key_derivation_payload` at their defaults;
///   `build_derive_key_params` and `build_derive_key_payload` at their
///   defaults.
/// Arrange: `build_create_key_derivation_deps` and
///   `build_create_key_derivation_payload` with no override;
///   `build_create_key_derivation_params` with the `selection` override
///   `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable)`,
///   so the failure mode differs from the `NoFailure` default and a factory
///   that drops the failure mode, or constructs another concrete, returns a
///   key instead of the error; `build_derive_key_params` and
///   `build_derive_key_payload` with no override, whose key material is the
///   32-byte reference key material.
/// Act:     `create_key_derivation(&deps, params, payload)`, then
///   `adapter.derive_key` on the adapter it returns.
/// Assert:  the success arm of `create_key_derivation` is bound through
///   `expect`; `result.err()` of `derive_key` equals
///   `Some(DeriveKeyErrorReturn::Mock(MockIKeyDerivationAdapterDeriveKeyErrorReturn::KeyMaterialLengthUnrepresentable { length: 32 }))`
///   by `assert_eq!`, the length a literal and not read from the arrangement.
#[test]
fn create_key_derivation_constructs_the_mock_concrete_under_the_failure_mode_the_selection_carries()
{
    // Arrange
    let deps = build_create_key_derivation_deps(Default::default());
    let params = build_create_key_derivation_params(CreateKeyDerivationParamsOverrides {
        selection: Some(KdfConcrete::Mock(
            MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable,
        )),
    });
    let payload = build_create_key_derivation_payload(Default::default());
    let derive_params = build_derive_key_params(Default::default());
    let derive_payload = build_derive_key_payload(Default::default());

    // Act
    let result = create_key_derivation(&deps, params, payload);
    let success = result.expect("the mock selection is constructed");
    let adapter: &dyn IKeyDerivationAdapter = &*success.adapter;
    let result = adapter.derive_key(derive_params, derive_payload);

    // Assert
    assert_eq!(
        result.err(),
        Some(DeriveKeyErrorReturn::Mock(
            MockIKeyDerivationAdapterDeriveKeyErrorReturn::KeyMaterialLengthUnrepresentable {
                length: 32
            }
        ))
    );
}
