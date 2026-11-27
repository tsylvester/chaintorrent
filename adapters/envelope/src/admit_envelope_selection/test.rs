#![allow(clippy::expect_used)]

use super::admit_envelope_selection;
use super::interface::AdmitEnvelopeSelectionErrorReturn;
use super::mock::{
    AdmitEnvelopeSelectionParamsOverrides, build_admit_envelope_selection_deps,
    build_admit_envelope_selection_params,
};
use crate::factory::provides::{KeyAgreementConcrete, MockIKeyAgreementAdapterFailureMode};
use serde_json::{Map, Value};

/// Contract: `admit_envelope_selection`, real member, admitted: `params.concrete`
///   is `KeyAgreementConcrete::PairingElGamal` and the document is an object
///   holding no key → `Ok` whose `selection` is
///   `KeyAgreementConcrete::PairingElGamal`.
/// Collaborators: none.
/// Arrange: `build_admit_envelope_selection_deps` with no override;
///   `build_admit_envelope_selection_params` with `concrete` overridden to
///   `KeyAgreementConcrete::PairingElGamal`; the document `Value::Object` over
///   an empty `serde_json::Map`, written directly.
/// Act:     `admit_envelope_selection(&deps, params, document)`.
/// Assert:  the success arm is bound through `expect` naming the invariant
///   that an empty object supplies no setting; `success.selection` equals
///   `KeyAgreementConcrete::PairingElGamal`.
#[test]
fn admit_envelope_selection_returns_the_pairing_elgamal_selection_for_an_empty_object() {
    // Arrange
    let deps = build_admit_envelope_selection_deps(Default::default());
    let params = build_admit_envelope_selection_params(AdmitEnvelopeSelectionParamsOverrides {
        concrete: Some(KeyAgreementConcrete::PairingElGamal),
    });
    let document = Value::Object(Map::new());

    // Act
    let result = admit_envelope_selection(&deps, params, document);

    // Assert
    let success = result.expect("an object holding no key supplies no setting");
    assert_eq!(success.selection, KeyAgreementConcrete::PairingElGamal);
}

/// Contract: `admit_envelope_selection`, real member, admitted: `params.concrete`
///   is `KeyAgreementConcrete::PairingElGamal` and the document is the JSON
///   null → `Ok` whose `selection` is `KeyAgreementConcrete::PairingElGamal`.
/// Collaborators: none.
/// Arrange: the built deps and params as in the empty-object block; the
///   document `Value::Null`, written directly.
/// Act:     `admit_envelope_selection(&deps, params, document)`.
/// Assert:  the success arm is bound through `expect` naming the invariant
///   that a null document supplies no setting; `success.selection` equals
///   `KeyAgreementConcrete::PairingElGamal`.
#[test]
fn admit_envelope_selection_returns_the_pairing_elgamal_selection_for_a_null_document() {
    // Arrange
    let deps = build_admit_envelope_selection_deps(Default::default());
    let params = build_admit_envelope_selection_params(AdmitEnvelopeSelectionParamsOverrides {
        concrete: Some(KeyAgreementConcrete::PairingElGamal),
    });
    let document = Value::Null;

    // Act
    let result = admit_envelope_selection(&deps, params, document);

    // Assert
    let success = result.expect("a null document supplies no setting");
    assert_eq!(success.selection, KeyAgreementConcrete::PairingElGamal);
}

/// Contract: `admit_envelope_selection`, real member, a setting supplied:
///   `params.concrete` is `KeyAgreementConcrete::PairingElGamal` and the
///   document is an object holding a key →
///   `Err(AdmitEnvelopeSelectionErrorReturn::SettingSupplied { concrete:
///   KeyAgreementConcrete::PairingElGamal })`, and no selection is returned.
/// Collaborators: none.
/// Arrange: the built deps and params as in the admitted blocks; the document
///   `Value::Object` over a `serde_json::Map` holding the key `setting` with
///   the integer value one, written directly.
/// Act:     `admit_envelope_selection(&deps, params, document)`.
/// Assert:  `result.err()` equals
///   `Some(AdmitEnvelopeSelectionErrorReturn::SettingSupplied { concrete:
///   KeyAgreementConcrete::PairingElGamal })`, the whole error.
#[test]
fn admit_envelope_selection_refuses_a_setting_for_the_pairing_elgamal_member() {
    // Arrange
    let deps = build_admit_envelope_selection_deps(Default::default());
    let params = build_admit_envelope_selection_params(AdmitEnvelopeSelectionParamsOverrides {
        concrete: Some(KeyAgreementConcrete::PairingElGamal),
    });
    let mut map = Map::new();
    map.insert("setting".to_string(), Value::from(1));
    let document = Value::Object(map);

    // Act
    let result = admit_envelope_selection(&deps, params, document);

    // Assert
    assert_eq!(
        result.err(),
        Some(AdmitEnvelopeSelectionErrorReturn::SettingSupplied {
            concrete: KeyAgreementConcrete::PairingElGamal,
        })
    );
}

/// Contract: `admit_envelope_selection`, real member, a setting supplied:
///   `params.concrete` is `KeyAgreementConcrete::PairingElGamal` and the
///   document is a scalar, an array, a boolean, a string, or a number →
///   `Err(AdmitEnvelopeSelectionErrorReturn::SettingSupplied { concrete:
///   KeyAgreementConcrete::PairingElGamal })`, and no selection is returned.
/// Collaborators: none.
/// Arrange: the built deps and params as in the admitted blocks; each document
///   written directly as a scalar, an array, a boolean, a string, and a number
///   `serde_json::Value`.
/// Act:     `admit_envelope_selection(&deps, params, document)` once per
///   document shape.
/// Assert:  `result.err()` equals
///   `Some(AdmitEnvelopeSelectionErrorReturn::SettingSupplied { concrete:
///   KeyAgreementConcrete::PairingElGamal })`, the whole error.
#[test]
fn admit_envelope_selection_refuses_a_setting_for_the_pairing_elgamal_member_for_any_other_value() {
    for document in [
        Value::from(7),
        Value::Array(Vec::new()),
        Value::Bool(true),
        Value::String("supplied".to_string()),
        Value::from(1.5),
    ] {
        // Arrange
        let deps = build_admit_envelope_selection_deps(Default::default());
        let params = build_admit_envelope_selection_params(AdmitEnvelopeSelectionParamsOverrides {
            concrete: Some(KeyAgreementConcrete::PairingElGamal),
        });

        // Act
        let result = admit_envelope_selection(&deps, params, document);

        // Assert
        assert_eq!(
            result.err(),
            Some(AdmitEnvelopeSelectionErrorReturn::SettingSupplied {
                concrete: KeyAgreementConcrete::PairingElGamal,
            })
        );
    }
}

/// Contract: `admit_envelope_selection`, mock member, admitted: `params.concrete`
///   is `KeyAgreementConcrete::Mock(failure_mode)` for each variant of
///   `MockIKeyAgreementAdapterFailureMode` and the document is an object
///   holding no key → `Ok` whose `selection` is the `KeyAgreementConcrete::Mock`
///   member carrying that variant.
/// Collaborators: none.
/// Arrange: `build_admit_envelope_selection_deps` with no override;
///   `build_admit_envelope_selection_params` with `concrete` overridden to the
///   `KeyAgreementConcrete::Mock` member carrying the selected variant, the
///   variants `NoFailure`, `NonceDrawRefused`, and `CoinDrawRefused` each
///   stated as independent literals; the document `Value::Object` over an
///   empty `serde_json::Map`, written directly.
/// Act:     `admit_envelope_selection(&deps, params, document)` once per
///   variant.
/// Assert:  the success arm is bound through `expect` naming the invariant
///   that an empty object supplies no setting; `success.selection` equals the
///   `KeyAgreementConcrete::Mock` member carrying the selected variant.
#[test]
fn admit_envelope_selection_returns_the_mock_selection_carrying_each_failure_mode() {
    for failure_mode in [
        MockIKeyAgreementAdapterFailureMode::NoFailure,
        MockIKeyAgreementAdapterFailureMode::NonceDrawRefused,
        MockIKeyAgreementAdapterFailureMode::CoinDrawRefused,
    ] {
        // Arrange
        let deps = build_admit_envelope_selection_deps(Default::default());
        let params = build_admit_envelope_selection_params(AdmitEnvelopeSelectionParamsOverrides {
            concrete: Some(KeyAgreementConcrete::Mock(failure_mode)),
        });
        let document = Value::Object(Map::new());

        // Act
        let result = admit_envelope_selection(&deps, params, document);

        // Assert
        let success = result.expect("an object holding no key supplies no setting");
        assert_eq!(success.selection, KeyAgreementConcrete::Mock(failure_mode));
    }
}

/// Contract: `admit_envelope_selection`, mock member, a setting supplied:
///   `params.concrete` is
///   `KeyAgreementConcrete::Mock(MockIKeyAgreementAdapterFailureMode::NonceDrawRefused)`
///   and the document is an object holding a key →
///   `Err(AdmitEnvelopeSelectionErrorReturn::SettingSupplied { concrete })`
///   with `concrete` that mock member, and no selection is returned.
/// Collaborators: none.
/// Arrange: `build_admit_envelope_selection_params` with `concrete` overridden
///   to
///   `KeyAgreementConcrete::Mock(MockIKeyAgreementAdapterFailureMode::NonceDrawRefused)`;
///   the built deps; the document `Value::Object` over a `serde_json::Map`
///   holding the key `setting` with the integer value one, written directly.
/// Act:     `admit_envelope_selection(&deps, params, document)`.
/// Assert:  `result.err()` equals
///   `Some(AdmitEnvelopeSelectionErrorReturn::SettingSupplied { concrete:
///   KeyAgreementConcrete::Mock(MockIKeyAgreementAdapterFailureMode::NonceDrawRefused) })`,
///   the whole error.
#[test]
fn admit_envelope_selection_refuses_a_setting_for_the_mock_member() {
    // Arrange
    let deps = build_admit_envelope_selection_deps(Default::default());
    let params = build_admit_envelope_selection_params(AdmitEnvelopeSelectionParamsOverrides {
        concrete: Some(KeyAgreementConcrete::Mock(
            MockIKeyAgreementAdapterFailureMode::NonceDrawRefused,
        )),
    });
    let mut map = Map::new();
    map.insert("setting".to_string(), Value::from(1));
    let document = Value::Object(map);

    // Act
    let result = admit_envelope_selection(&deps, params, document);

    // Assert
    assert_eq!(
        result.err(),
        Some(AdmitEnvelopeSelectionErrorReturn::SettingSupplied {
            concrete: KeyAgreementConcrete::Mock(
                MockIKeyAgreementAdapterFailureMode::NonceDrawRefused
            ),
        })
    );
}

/// Contract: `admit_envelope_selection`, mock member, a setting supplied:
///   `params.concrete` is
///   `KeyAgreementConcrete::Mock(MockIKeyAgreementAdapterFailureMode::CoinDrawRefused)`
///   and the document is a scalar, an array, a boolean, a string, or a number →
///   `Err(AdmitEnvelopeSelectionErrorReturn::SettingSupplied { concrete })`
///   with `concrete` that mock member, and no selection is returned.
/// Collaborators: none.
/// Arrange: `build_admit_envelope_selection_params` with `concrete` overridden
///   to
///   `KeyAgreementConcrete::Mock(MockIKeyAgreementAdapterFailureMode::CoinDrawRefused)`;
///   the built deps; each document written directly as a scalar, an array, a
///   boolean, a string, and a number `serde_json::Value`.
/// Act:     `admit_envelope_selection(&deps, params, document)` once per
///   document shape.
/// Assert:  `result.err()` equals
///   `Some(AdmitEnvelopeSelectionErrorReturn::SettingSupplied { concrete:
///   KeyAgreementConcrete::Mock(MockIKeyAgreementAdapterFailureMode::CoinDrawRefused) })`,
///   the whole error.
#[test]
fn admit_envelope_selection_refuses_a_setting_for_the_mock_member_for_any_other_value() {
    for document in [
        Value::from(7),
        Value::Array(Vec::new()),
        Value::Bool(true),
        Value::String("supplied".to_string()),
        Value::from(1.5),
    ] {
        // Arrange
        let deps = build_admit_envelope_selection_deps(Default::default());
        let params = build_admit_envelope_selection_params(AdmitEnvelopeSelectionParamsOverrides {
            concrete: Some(KeyAgreementConcrete::Mock(
                MockIKeyAgreementAdapterFailureMode::CoinDrawRefused,
            )),
        });

        // Act
        let result = admit_envelope_selection(&deps, params, document);

        // Assert
        assert_eq!(
            result.err(),
            Some(AdmitEnvelopeSelectionErrorReturn::SettingSupplied {
                concrete: KeyAgreementConcrete::Mock(
                    MockIKeyAgreementAdapterFailureMode::CoinDrawRefused
                ),
            })
        );
    }
}
