use super::provides::{
    AdmitPairingSelectionErrorReturn, AdmitPairingSelectionParamsOverrides,
    AdmitPairingSelectionSuccessReturnOverrides, admit_pairing_selection,
    build_admit_pairing_selection_deps, build_admit_pairing_selection_params,
    build_admit_pairing_selection_success_return,
};
use crate::factory::provides::{MockIPairingAdapterFailureMode, PairingConcrete};
use serde_json::{Map, Value};

/// Contract: `params.identifier` is `PairingConcrete::Bn254Arkworks` and the payload is
///   an empty `Value::Object` → `Ok` whose `selection` is
///   `PairingConcrete::Bn254Arkworks`.
/// Collaborators: none; fixtures `build_admit_pairing_selection_deps`,
///   `build_admit_pairing_selection_params`, and
///   `build_admit_pairing_selection_success_return`.
/// Arrange: `build_admit_pairing_selection_params` with the identifier override
///   `PairingConcrete::Bn254Arkworks`, and `Value::Object(Map::new())`, a member whose
///   selection differs from every other member's, so an admission that returns another
///   member fails.
/// Act:     `admit_pairing_selection(&deps, params, document)`.
/// Assert:  the result equals `Ok(build_admit_pairing_selection_success_return(
///   AdmitPairingSelectionSuccessReturnOverrides { selection:
///   Some(PairingConcrete::Bn254Arkworks) }))`.
#[test]
fn admit_returns_the_bn254_arkworks_selection_for_an_empty_document() {
    // Arrange
    let deps = build_admit_pairing_selection_deps(Default::default());
    let params = build_admit_pairing_selection_params(AdmitPairingSelectionParamsOverrides {
        identifier: Some(PairingConcrete::Bn254Arkworks),
    });
    let document = Value::Object(Map::new());

    // Act
    let result = admit_pairing_selection(&deps, params, document);

    // Assert
    assert_eq!(
        result,
        Ok(build_admit_pairing_selection_success_return(
            AdmitPairingSelectionSuccessReturnOverrides {
                selection: Some(PairingConcrete::Bn254Arkworks),
            }
        ))
    );
}
// admit_returns_the_bn254_arkworks_selection_for_an_empty_document

/// Contract: `params.identifier` is `PairingConcrete::Bn254Halo2curves` and the payload
///   is an empty `Value::Object` → `Ok` whose `selection` is
///   `PairingConcrete::Bn254Halo2curves`.
/// Collaborators: none; the fixtures of the previous block.
/// Arrange: the identifier override `PairingConcrete::Bn254Halo2curves` and
///   `Value::Object(Map::new())`, a member that differs from the arkworks member of the
///   same curve, so an admission that returns the arkworks member fails.
/// Act:     `admit_pairing_selection(&deps, params, document)`.
/// Assert:  the result equals `Ok(build_admit_pairing_selection_success_return(
///   AdmitPairingSelectionSuccessReturnOverrides { selection:
///   Some(PairingConcrete::Bn254Halo2curves) }))`.
#[test]
fn admit_returns_the_bn254_halo2curves_selection_for_an_empty_document() {
    // Arrange
    let deps = build_admit_pairing_selection_deps(Default::default());
    let params = build_admit_pairing_selection_params(AdmitPairingSelectionParamsOverrides {
        identifier: Some(PairingConcrete::Bn254Halo2curves),
    });
    let document = Value::Object(Map::new());

    // Act
    let result = admit_pairing_selection(&deps, params, document);

    // Assert
    assert_eq!(
        result,
        Ok(build_admit_pairing_selection_success_return(
            AdmitPairingSelectionSuccessReturnOverrides {
                selection: Some(PairingConcrete::Bn254Halo2curves),
            }
        ))
    );
}
// admit_returns_the_bn254_halo2curves_selection_for_an_empty_document

/// Contract: `params.identifier` is `PairingConcrete::Bls12381Arkworks` and the payload
///   is an empty `Value::Object` → `Ok` whose `selection` is
///   `PairingConcrete::Bls12381Arkworks`.
/// Collaborators: none; the fixtures of the previous blocks.
/// Arrange: the identifier override `PairingConcrete::Bls12381Arkworks` and
///   `Value::Object(Map::new())`, a member of the other curve, so an admission that
///   returns a BN254 member fails.
/// Act:     `admit_pairing_selection(&deps, params, document)`.
/// Assert:  the result equals `Ok(build_admit_pairing_selection_success_return(
///   AdmitPairingSelectionSuccessReturnOverrides { selection:
///   Some(PairingConcrete::Bls12381Arkworks) }))`.
#[test]
fn admit_returns_the_bls12_381_arkworks_selection_for_an_empty_document() {
    // Arrange
    let deps = build_admit_pairing_selection_deps(Default::default());
    let params = build_admit_pairing_selection_params(AdmitPairingSelectionParamsOverrides {
        identifier: Some(PairingConcrete::Bls12381Arkworks),
    });
    let document = Value::Object(Map::new());

    // Act
    let result = admit_pairing_selection(&deps, params, document);

    // Assert
    assert_eq!(
        result,
        Ok(build_admit_pairing_selection_success_return(
            AdmitPairingSelectionSuccessReturnOverrides {
                selection: Some(PairingConcrete::Bls12381Arkworks),
            }
        ))
    );
}
// admit_returns_the_bls12_381_arkworks_selection_for_an_empty_document

/// Contract: `params.identifier` is `PairingConcrete::Bls12381Halo2curves` and the
///   payload is an empty `Value::Object` → `Ok` whose `selection` is
///   `PairingConcrete::Bls12381Halo2curves`.
/// Collaborators: none; the fixtures of the previous blocks.
/// Arrange: the identifier override `PairingConcrete::Bls12381Halo2curves` and
///   `Value::Object(Map::new())`, a member that differs from the arkworks member of the
///   same curve, so an admission that returns the arkworks member fails.
/// Act:     `admit_pairing_selection(&deps, params, document)`.
/// Assert:  the result equals `Ok(build_admit_pairing_selection_success_return(
///   AdmitPairingSelectionSuccessReturnOverrides { selection:
///   Some(PairingConcrete::Bls12381Halo2curves) }))`.
#[test]
fn admit_returns_the_bls12_381_halo2curves_selection_for_an_empty_document() {
    // Arrange
    let deps = build_admit_pairing_selection_deps(Default::default());
    let params = build_admit_pairing_selection_params(AdmitPairingSelectionParamsOverrides {
        identifier: Some(PairingConcrete::Bls12381Halo2curves),
    });
    let document = Value::Object(Map::new());

    // Act
    let result = admit_pairing_selection(&deps, params, document);

    // Assert
    assert_eq!(
        result,
        Ok(build_admit_pairing_selection_success_return(
            AdmitPairingSelectionSuccessReturnOverrides {
                selection: Some(PairingConcrete::Bls12381Halo2curves),
            }
        ))
    );
}
// admit_returns_the_bls12_381_halo2curves_selection_for_an_empty_document

/// Contract: `params.identifier` is `PairingConcrete::Bls12381Arkworks` and the payload
///   is `Value::Null` → `Ok` whose `selection` is `PairingConcrete::Bls12381Arkworks`.
/// Collaborators: none; the fixtures of the previous blocks.
/// Arrange: the identifier override `PairingConcrete::Bls12381Arkworks` and
///   `Value::Null`, the absent document, a form distinct from the empty object, so an
///   admission that treats only an empty object as no setting fails.
/// Act:     `admit_pairing_selection(&deps, params, document)`.
/// Assert:  the result equals `Ok(build_admit_pairing_selection_success_return(
///   AdmitPairingSelectionSuccessReturnOverrides { selection:
///   Some(PairingConcrete::Bls12381Arkworks) }))`.
#[test]
fn admit_returns_the_selection_for_a_null_document() {
    // Arrange
    let deps = build_admit_pairing_selection_deps(Default::default());
    let params = build_admit_pairing_selection_params(AdmitPairingSelectionParamsOverrides {
        identifier: Some(PairingConcrete::Bls12381Arkworks),
    });
    let document = Value::Null;

    // Act
    let result = admit_pairing_selection(&deps, params, document);

    // Assert
    assert_eq!(
        result,
        Ok(build_admit_pairing_selection_success_return(
            AdmitPairingSelectionSuccessReturnOverrides {
                selection: Some(PairingConcrete::Bls12381Arkworks),
            }
        ))
    );
}
// admit_returns_the_selection_for_a_null_document

/// Contract: `params.identifier` is `PairingConcrete::Bn254Arkworks` and the payload is
///   a `Value::Object` with one entry →
///   `Err(AdmitPairingSelectionErrorReturn::SettingsSupplied { identifier:
///   PairingConcrete::Bn254Arkworks })`.
/// Collaborators: none; the fixtures of the previous blocks.
/// Arrange: the identifier override `PairingConcrete::Bn254Arkworks` and a
///   `Value::Object` built from a `Map` holding the key `endpoint` with the value
///   `Value::Bool(true)`, a document that differs from the empty object by one entry,
///   so an admission that ignores an object's entries fails.
/// Act:     `admit_pairing_selection(&deps, params, document)`.
/// Assert:  the result equals
///   `Err(AdmitPairingSelectionErrorReturn::SettingsSupplied { identifier:
///   PairingConcrete::Bn254Arkworks })`.
#[test]
fn admit_refuses_an_object_that_holds_an_entry() {
    // Arrange
    let deps = build_admit_pairing_selection_deps(Default::default());
    let params = build_admit_pairing_selection_params(AdmitPairingSelectionParamsOverrides {
        identifier: Some(PairingConcrete::Bn254Arkworks),
    });
    let mut entries = Map::new();
    entries.insert("endpoint".to_string(), Value::Bool(true));
    let document = Value::Object(entries);

    // Act
    let result = admit_pairing_selection(&deps, params, document);

    // Assert
    assert_eq!(
        result,
        Err(AdmitPairingSelectionErrorReturn::SettingsSupplied {
            identifier: PairingConcrete::Bn254Arkworks,
        })
    );
}
// admit_refuses_an_object_that_holds_an_entry

/// Contract: `params.identifier` is `PairingConcrete::Bn254Halo2curves` and the payload
///   is `Value::Bool(false)` → `Err(AdmitPairingSelectionErrorReturn::SettingsSupplied
///   { identifier: PairingConcrete::Bn254Halo2curves })`.
/// Collaborators: none; the fixtures of the previous blocks.
/// Arrange: the identifier override `PairingConcrete::Bn254Halo2curves` and
///   `Value::Bool(false)`, a falsy scalar document, so an admission that treats a falsy
///   value as absent fails.
/// Act:     `admit_pairing_selection(&deps, params, document)`.
/// Assert:  the result equals
///   `Err(AdmitPairingSelectionErrorReturn::SettingsSupplied { identifier:
///   PairingConcrete::Bn254Halo2curves })`.
#[test]
fn admit_refuses_a_boolean_document() {
    // Arrange
    let deps = build_admit_pairing_selection_deps(Default::default());
    let params = build_admit_pairing_selection_params(AdmitPairingSelectionParamsOverrides {
        identifier: Some(PairingConcrete::Bn254Halo2curves),
    });
    let document = Value::Bool(false);

    // Act
    let result = admit_pairing_selection(&deps, params, document);

    // Assert
    assert_eq!(
        result,
        Err(AdmitPairingSelectionErrorReturn::SettingsSupplied {
            identifier: PairingConcrete::Bn254Halo2curves,
        })
    );
}
// admit_refuses_a_boolean_document

/// Contract: `params.identifier` is `PairingConcrete::Bls12381Arkworks` and the payload
///   is the number zero → `Err(AdmitPairingSelectionErrorReturn::SettingsSupplied
///   { identifier: PairingConcrete::Bls12381Arkworks })`.
/// Collaborators: none; the fixtures of the previous blocks.
/// Arrange: the identifier override `PairingConcrete::Bls12381Arkworks` and
///   `Value::Number` holding zero, a falsy number, so an admission that treats zero as
///   absent fails.
/// Act:     `admit_pairing_selection(&deps, params, document)`.
/// Assert:  the result equals
///   `Err(AdmitPairingSelectionErrorReturn::SettingsSupplied { identifier:
///   PairingConcrete::Bls12381Arkworks })`.
#[test]
fn admit_refuses_a_number_document() {
    // Arrange
    let deps = build_admit_pairing_selection_deps(Default::default());
    let params = build_admit_pairing_selection_params(AdmitPairingSelectionParamsOverrides {
        identifier: Some(PairingConcrete::Bls12381Arkworks),
    });
    let document = Value::Number(0.into());

    // Act
    let result = admit_pairing_selection(&deps, params, document);

    // Assert
    assert_eq!(
        result,
        Err(AdmitPairingSelectionErrorReturn::SettingsSupplied {
            identifier: PairingConcrete::Bls12381Arkworks,
        })
    );
}
// admit_refuses_a_number_document

/// Contract: `params.identifier` is `PairingConcrete::Bls12381Halo2curves` and the
///   payload is the empty `Value::String` →
///   `Err(AdmitPairingSelectionErrorReturn::SettingsSupplied { identifier:
///   PairingConcrete::Bls12381Halo2curves })`.
/// Collaborators: none; the fixtures of the previous blocks.
/// Arrange: the identifier override `PairingConcrete::Bls12381Halo2curves` and
///   `Value::String` holding the empty text, a falsy string, so an admission that
///   treats an empty string as absent fails.
/// Act:     `admit_pairing_selection(&deps, params, document)`.
/// Assert:  the result equals
///   `Err(AdmitPairingSelectionErrorReturn::SettingsSupplied { identifier:
///   PairingConcrete::Bls12381Halo2curves })`.
#[test]
fn admit_refuses_a_string_document() {
    // Arrange
    let deps = build_admit_pairing_selection_deps(Default::default());
    let params = build_admit_pairing_selection_params(AdmitPairingSelectionParamsOverrides {
        identifier: Some(PairingConcrete::Bls12381Halo2curves),
    });
    let document = Value::String(String::new());

    // Act
    let result = admit_pairing_selection(&deps, params, document);

    // Assert
    assert_eq!(
        result,
        Err(AdmitPairingSelectionErrorReturn::SettingsSupplied {
            identifier: PairingConcrete::Bls12381Halo2curves,
        })
    );
}
// admit_refuses_a_string_document

/// Contract: `params.identifier` is `PairingConcrete::Bn254Arkworks` and the payload is
///   the empty `Value::Array` →
///   `Err(AdmitPairingSelectionErrorReturn::SettingsSupplied { identifier:
///   PairingConcrete::Bn254Arkworks })`.
/// Collaborators: none; the fixtures of the previous blocks.
/// Arrange: the identifier override `PairingConcrete::Bn254Arkworks` and `Value::Array`
///   holding no element, an empty array that differs from the empty object, so an
///   admission that treats every empty container as absent fails.
/// Act:     `admit_pairing_selection(&deps, params, document)`.
/// Assert:  the result equals
///   `Err(AdmitPairingSelectionErrorReturn::SettingsSupplied { identifier:
///   PairingConcrete::Bn254Arkworks })`.
#[test]
fn admit_refuses_an_array_document() {
    // Arrange
    let deps = build_admit_pairing_selection_deps(Default::default());
    let params = build_admit_pairing_selection_params(AdmitPairingSelectionParamsOverrides {
        identifier: Some(PairingConcrete::Bn254Arkworks),
    });
    let document = Value::Array(Vec::new());

    // Act
    let result = admit_pairing_selection(&deps, params, document);

    // Assert
    assert_eq!(
        result,
        Err(AdmitPairingSelectionErrorReturn::SettingsSupplied {
            identifier: PairingConcrete::Bn254Arkworks,
        })
    );
}
// admit_refuses_an_array_document

/// Contract: `params.identifier` is `PairingConcrete::Mock(mode)` and the payload
///   supplies no setting → `Ok` whose `selection` is `PairingConcrete::Mock(mode)`.
/// Collaborators: none; the fixtures of the previous blocks.
/// Arrange: the identifier override
///   `PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG1NotOnCurve)`, a mode
///   other than `Succeeds`, and `Value::Object(Map::new())`, so an admission that
///   returns a mode other than the one named fails.
/// Act:     `admit_pairing_selection(&deps, params, document)`.
/// Assert:  the result equals `Ok(build_admit_pairing_selection_success_return(
///   AdmitPairingSelectionSuccessReturnOverrides { selection:
///   Some(PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG1NotOnCurve))
///   }))`.
#[test]
fn admit_returns_the_mock_selection_carrying_its_failure_mode() {
    // Arrange
    let deps = build_admit_pairing_selection_deps(Default::default());
    let params = build_admit_pairing_selection_params(AdmitPairingSelectionParamsOverrides {
        identifier: Some(PairingConcrete::Mock(
            MockIPairingAdapterFailureMode::DecodeG1NotOnCurve,
        )),
    });
    let document = Value::Object(Map::new());

    // Act
    let result = admit_pairing_selection(&deps, params, document);

    // Assert
    assert_eq!(
        result,
        Ok(build_admit_pairing_selection_success_return(
            AdmitPairingSelectionSuccessReturnOverrides {
                selection: Some(PairingConcrete::Mock(
                    MockIPairingAdapterFailureMode::DecodeG1NotOnCurve,
                )),
            }
        ))
    );
}
// admit_returns_the_mock_selection_carrying_its_failure_mode

/// Contract: `params.identifier` is `PairingConcrete::Mock(mode)` and the payload
///   supplies a setting → `Err(AdmitPairingSelectionErrorReturn::SettingsSupplied
///   { identifier: PairingConcrete::Mock(mode) })`.
/// Collaborators: none; the fixtures of the previous blocks.
/// Arrange: the identifier override
///   `PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG2NotOnCurve)` and a
///   `Value::Object` holding one entry, so a refusal that names a real member or
///   another mode fails.
/// Act:     `admit_pairing_selection(&deps, params, document)`.
/// Assert:  the result equals
///   `Err(AdmitPairingSelectionErrorReturn::SettingsSupplied { identifier:
///   PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG2NotOnCurve) })`.
#[test]
fn admit_refuses_a_setting_for_the_mock_member_naming_it() {
    // Arrange
    let deps = build_admit_pairing_selection_deps(Default::default());
    let params = build_admit_pairing_selection_params(AdmitPairingSelectionParamsOverrides {
        identifier: Some(PairingConcrete::Mock(
            MockIPairingAdapterFailureMode::DecodeG2NotOnCurve,
        )),
    });
    let mut entries = Map::new();
    entries.insert("endpoint".to_string(), Value::Bool(true));
    let document = Value::Object(entries);

    // Act
    let result = admit_pairing_selection(&deps, params, document);

    // Assert
    assert_eq!(
        result,
        Err(AdmitPairingSelectionErrorReturn::SettingsSupplied {
            identifier: PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG2NotOnCurve,),
        })
    );
}
// admit_refuses_a_setting_for_the_mock_member_naming_it
