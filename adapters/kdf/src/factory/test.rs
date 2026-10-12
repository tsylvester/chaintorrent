#![allow(clippy::expect_used)]

use super::provides::{
    CreateKeyDerivationErrorReturn, CreateKeyDerivationParamsOverrides, IKeyDerivationAdapter,
    KdfIdentifier, build_create_key_derivation_deps, build_create_key_derivation_params,
    build_create_key_derivation_payload, create_key_derivation,
};

/// Contract: `create_key_derivation`, admitted: the named concrete's declared
///   identifier is `params.identifier` →
///   `Ok(CreateKeyDerivationSuccessReturn { adapter })` whose adapter reports
///   the declaration of the concrete the selection names.
/// Arrange: `build_create_key_derivation_deps`,
///   `build_create_key_derivation_params`, and
///   `build_create_key_derivation_payload`, each with no override, so the
///   defaults select the family's mock concrete under the identifier it
///   declares; the selected concrete declares an identifier no other
///   concrete declares, so a factory that constructs another concrete
///   reports another identifier.
/// Act:     `create_key_derivation(&deps, params, payload)`.
/// Assert:  the success arm is bound through `expect`;
///   `adapter.declaration().identifier` equals `KdfIdentifier::MockV1`, the
///   identifier the family's mock concrete declares, by `assert!`, the
///   expectation a literal and not read from the arrangement.
#[test]
fn create_key_derivation_returns_the_adapter_of_the_selected_concrete() {
    // Arrange
    let deps = build_create_key_derivation_deps(Default::default());
    let params = build_create_key_derivation_params(Default::default());
    let payload = build_create_key_derivation_payload(Default::default());

    // Act
    let result = create_key_derivation(&deps, params, payload);

    // Assert
    let success = result.expect("the default selection is admitted");
    let adapter: &dyn IKeyDerivationAdapter = &*success.adapter;
    assert!(adapter.declaration().identifier == KdfIdentifier::MockV1);
}

/// Contract: `create_key_derivation`, unsupported identifier: the named
///   concrete's `DECLARATION.identifier` is not `params.identifier` →
///   `Err(CreateKeyDerivationErrorReturn::UnsupportedKdfIdentifier)`.
/// Arrange: `build_create_key_derivation_deps` and
///   `build_create_key_derivation_payload` with no override;
///   `build_create_key_derivation_params` overriding `identifier` with
///   `KdfIdentifier::Blake3KeyedV1` and keeping the default selection of the
///   mock concrete, which declares `KdfIdentifier::MockV1`, so the declared
///   identifier differs from the required one.
/// Act:     `create_key_derivation(&deps, params, payload)`.
/// Assert:  `result.err()` equals
///   `Some(CreateKeyDerivationErrorReturn::UnsupportedKdfIdentifier)` by
///   `assert_eq!`.
#[test]
fn create_key_derivation_refuses_a_concrete_that_declares_another_identifier() {
    // Arrange
    let deps = build_create_key_derivation_deps(Default::default());
    let params = build_create_key_derivation_params(CreateKeyDerivationParamsOverrides {
        identifier: Some(KdfIdentifier::Blake3KeyedV1),
        ..Default::default()
    });
    let payload = build_create_key_derivation_payload(Default::default());

    // Act
    let result = create_key_derivation(&deps, params, payload);

    // Assert
    assert_eq!(
        result.err(),
        Some(CreateKeyDerivationErrorReturn::UnsupportedKdfIdentifier)
    );
}
