use super::admit_random_selection;
use super::interface::*;
use super::mock::*;
use crate::factory::provides::{MockIRandomSourceAdapterFailureMode, RandomSourceKind};
use serde_json::{Map, Value};

/// Contract: the identifier `RandomSourceKind::OperatingSystem` with the payload
///   `Value::Null` → `Ok` holding the selection
///   `RandomSourceKind::OperatingSystem`.
/// Arrange: the identifier `OperatingSystem` and the payload `Value::Null`, a
///   document that supplies no setting and is not an object, so a check that
///   refuses a document that is not an object fails.
/// Act:     `admit_random_selection`.
/// Assert:  the result equals `Ok` holding the success return built with the
///   selection override `RandomSourceKind::OperatingSystem`.
#[test]
fn the_operating_system_member_with_an_absent_document_returns_its_own_selection() {
    // Arrange
    let deps = build_admit_random_selection_deps(Default::default());
    let params = build_admit_random_selection_params(AdmitRandomSelectionParamsOverrides {
        identifier: Some(RandomSourceKind::OperatingSystem),
    });
    let payload = Value::Null;

    // Act
    let result: AdmitRandomSelectionReturn = admit_random_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Ok(build_admit_random_selection_success_return(
            AdmitRandomSelectionSuccessReturnOverrides {
                selection: Some(RandomSourceKind::OperatingSystem),
            }
        ))
    );
}
// the_operating_system_member_with_an_absent_document_returns_its_own_selection

/// Contract: the identifier `RandomSourceKind::OperatingSystem` with the payload
///   an empty `Value::Object` → `Ok` holding the selection
///   `RandomSourceKind::OperatingSystem`.
/// Arrange: the identifier `OperatingSystem` and the payload `Value::Object`
///   over `Map::new()`, a document that supplies no setting and is not null, so
///   a check that refuses every document but `Value::Null` fails.
/// Act:     `admit_random_selection`.
/// Assert:  the result equals `Ok` holding the success return built with the
///   selection override `RandomSourceKind::OperatingSystem`.
#[test]
fn the_operating_system_member_with_an_empty_object_document_returns_its_own_selection() {
    // Arrange
    let deps = build_admit_random_selection_deps(Default::default());
    let params = build_admit_random_selection_params(AdmitRandomSelectionParamsOverrides {
        identifier: Some(RandomSourceKind::OperatingSystem),
    });
    let payload = Value::Object(Map::new());

    // Act
    let result: AdmitRandomSelectionReturn = admit_random_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Ok(build_admit_random_selection_success_return(
            AdmitRandomSelectionSuccessReturnOverrides {
                selection: Some(RandomSourceKind::OperatingSystem),
            }
        ))
    );
}
// the_operating_system_member_with_an_empty_object_document_returns_its_own_selection

/// Contract: the identifier `RandomSourceKind::Mock(mode)` with the payload
///   `Value::Null` → `Ok` holding the selection `RandomSourceKind::Mock(mode)`.
/// Arrange: the identifier
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)`
///   and the payload `Value::Null`, a mode and a member that differ from the
///   operating-system member, so a branch that returns the real member's
///   selection fails.
/// Act:     `admit_random_selection`.
/// Assert:  the result equals `Ok` holding the success return built with the
///   selection override
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)`.
#[test]
fn the_mock_member_with_an_absent_document_returns_a_selection_carrying_its_failure_mode() {
    // Arrange
    let deps = build_admit_random_selection_deps(Default::default());
    let params = build_admit_random_selection_params(AdmitRandomSelectionParamsOverrides {
        identifier: Some(RandomSourceKind::Mock(
            MockIRandomSourceAdapterFailureMode::FillBytesRefused,
        )),
    });
    let payload = Value::Null;

    // Act
    let result: AdmitRandomSelectionReturn = admit_random_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Ok(build_admit_random_selection_success_return(
            AdmitRandomSelectionSuccessReturnOverrides {
                selection: Some(RandomSourceKind::Mock(
                    MockIRandomSourceAdapterFailureMode::FillBytesRefused,
                )),
            }
        ))
    );
}
// the_mock_member_with_an_absent_document_returns_a_selection_carrying_its_failure_mode

/// Contract: the identifier `RandomSourceKind::OperatingSystem` with a payload
///   that is a `Value::Object` holding an entry → `Err` `SettingsSupplied`
///   naming `RandomSourceKind::OperatingSystem`.
/// Arrange: the identifier `OperatingSystem` and the payload `Value::Object`
///   over a `Map` holding the key `unsupported_setting`, a document that
///   supplies a setting, so a branch that admits a document that is an object
///   fails.
/// Act:     `admit_random_selection`.
/// Assert:  the result equals `Err(AdmitRandomSelectionErrorReturn::
///   SettingsSupplied { identifier: RandomSourceKind::OperatingSystem })`.
#[test]
fn the_operating_system_member_with_a_document_holding_an_entry_is_refused_naming_the_member() {
    // Arrange
    let deps = build_admit_random_selection_deps(Default::default());
    let params = build_admit_random_selection_params(AdmitRandomSelectionParamsOverrides {
        identifier: Some(RandomSourceKind::OperatingSystem),
    });
    let mut entries = Map::new();
    entries.insert(String::from("unsupported_setting"), Value::Bool(true));
    let payload = Value::Object(entries);

    // Act
    let result: AdmitRandomSelectionReturn = admit_random_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitRandomSelectionErrorReturn::SettingsSupplied {
            identifier: RandomSourceKind::OperatingSystem,
        })
    );
}
// the_operating_system_member_with_a_document_holding_an_entry_is_refused_naming_the_member

/// Contract: the identifier `RandomSourceKind::OperatingSystem` with a payload
///   that is not an object and is not null → `Err` `SettingsSupplied` naming
///   `RandomSourceKind::OperatingSystem`.
/// Arrange: the identifier `OperatingSystem` and the payload `Value::Array`
///   over an empty `Vec`, a value an emptiness check on the document would
///   admit, so a branch that admits an empty document of any shape fails.
/// Act:     `admit_random_selection`.
/// Assert:  the result equals `Err(AdmitRandomSelectionErrorReturn::
///   SettingsSupplied { identifier: RandomSourceKind::OperatingSystem })`.
#[test]
fn the_operating_system_member_with_an_empty_array_document_is_refused_naming_the_member() {
    // Arrange
    let deps = build_admit_random_selection_deps(Default::default());
    let params = build_admit_random_selection_params(AdmitRandomSelectionParamsOverrides {
        identifier: Some(RandomSourceKind::OperatingSystem),
    });
    let payload = Value::Array(Vec::new());

    // Act
    let result: AdmitRandomSelectionReturn = admit_random_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitRandomSelectionErrorReturn::SettingsSupplied {
            identifier: RandomSourceKind::OperatingSystem,
        })
    );
}
// the_operating_system_member_with_an_empty_array_document_is_refused_naming_the_member

/// Contract: the identifier `RandomSourceKind::Mock(mode)` with a payload that
///   is a `Value::Object` holding an entry → `Err` `SettingsSupplied` naming
///   `RandomSourceKind::Mock(mode)`.
/// Arrange: the identifier
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)`
///   and the payload `Value::Object` over a `Map` holding the key
///   `unsupported_setting`, so a refusal that names the real member fails.
/// Act:     `admit_random_selection`.
/// Assert:  the result equals `Err(AdmitRandomSelectionErrorReturn::
///   SettingsSupplied { identifier: RandomSourceKind::Mock(
///   MockIRandomSourceAdapterFailureMode::FillBytesRefused) })`.
#[test]
fn the_mock_member_with_a_document_holding_an_entry_is_refused_naming_the_member_and_its_failure_mode()
 {
    // Arrange
    let deps = build_admit_random_selection_deps(Default::default());
    let params = build_admit_random_selection_params(AdmitRandomSelectionParamsOverrides {
        identifier: Some(RandomSourceKind::Mock(
            MockIRandomSourceAdapterFailureMode::FillBytesRefused,
        )),
    });
    let mut entries = Map::new();
    entries.insert(String::from("unsupported_setting"), Value::Bool(true));
    let payload = Value::Object(entries);

    // Act
    let result: AdmitRandomSelectionReturn = admit_random_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitRandomSelectionErrorReturn::SettingsSupplied {
            identifier: RandomSourceKind::Mock(
                MockIRandomSourceAdapterFailureMode::FillBytesRefused,
            ),
        })
    );
}
// the_mock_member_with_a_document_holding_an_entry_is_refused_naming_the_member_and_its_failure_mode

/// Contract: the identifier `RandomSourceKind::OperatingSystem` with the payload
///   `Value::Bool` → `Err` `SettingsSupplied` naming
///   `RandomSourceKind::OperatingSystem`.
/// Arrange: the identifier `OperatingSystem` and the payload `Value::Bool(true)`,
///   a value that is not an object and holds no entry, so a branch that admits
///   a boolean fails.
/// Act:     `admit_random_selection`.
/// Assert:  the result equals `Err(AdmitRandomSelectionErrorReturn::
///   SettingsSupplied { identifier: RandomSourceKind::OperatingSystem })`.
#[test]
fn the_operating_system_member_with_a_boolean_document_is_refused_naming_the_member() {
    // Arrange
    let deps = build_admit_random_selection_deps(Default::default());
    let params = build_admit_random_selection_params(AdmitRandomSelectionParamsOverrides {
        identifier: Some(RandomSourceKind::OperatingSystem),
    });
    let payload = Value::Bool(true);

    // Act
    let result: AdmitRandomSelectionReturn = admit_random_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitRandomSelectionErrorReturn::SettingsSupplied {
            identifier: RandomSourceKind::OperatingSystem,
        })
    );
}
// the_operating_system_member_with_a_boolean_document_is_refused_naming_the_member

/// Contract: the identifier `RandomSourceKind::OperatingSystem` with the payload
///   `Value::Number` → `Err` `SettingsSupplied` naming
///   `RandomSourceKind::OperatingSystem`.
/// Arrange: the identifier `OperatingSystem` and the payload
///   `Value::from(7_u64)`, a value that is not an object and holds no entry, so
///   a branch that admits a number fails.
/// Act:     `admit_random_selection`.
/// Assert:  the result equals `Err(AdmitRandomSelectionErrorReturn::
///   SettingsSupplied { identifier: RandomSourceKind::OperatingSystem })`.
#[test]
fn the_operating_system_member_with_a_number_document_is_refused_naming_the_member() {
    // Arrange
    let deps = build_admit_random_selection_deps(Default::default());
    let params = build_admit_random_selection_params(AdmitRandomSelectionParamsOverrides {
        identifier: Some(RandomSourceKind::OperatingSystem),
    });
    let payload = Value::from(7_u64);

    // Act
    let result: AdmitRandomSelectionReturn = admit_random_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitRandomSelectionErrorReturn::SettingsSupplied {
            identifier: RandomSourceKind::OperatingSystem,
        })
    );
}
// the_operating_system_member_with_a_number_document_is_refused_naming_the_member

/// Contract: the identifier `RandomSourceKind::OperatingSystem` with the payload
///   `Value::String` → `Err` `SettingsSupplied` naming
///   `RandomSourceKind::OperatingSystem`.
/// Arrange: the identifier `OperatingSystem` and the payload
///   `Value::String(String::from("text"))`, a value that is not an object and
///   holds no entry, so a branch that admits a string fails.
/// Act:     `admit_random_selection`.
/// Assert:  the result equals `Err(AdmitRandomSelectionErrorReturn::
///   SettingsSupplied { identifier: RandomSourceKind::OperatingSystem })`.
#[test]
fn the_operating_system_member_with_a_string_document_is_refused_naming_the_member() {
    // Arrange
    let deps = build_admit_random_selection_deps(Default::default());
    let params = build_admit_random_selection_params(AdmitRandomSelectionParamsOverrides {
        identifier: Some(RandomSourceKind::OperatingSystem),
    });
    let payload = Value::String(String::from("text"));

    // Act
    let result: AdmitRandomSelectionReturn = admit_random_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitRandomSelectionErrorReturn::SettingsSupplied {
            identifier: RandomSourceKind::OperatingSystem,
        })
    );
}
// the_operating_system_member_with_a_string_document_is_refused_naming_the_member

/// Contract: the identifier `RandomSourceKind::Mock(mode)` with the payload an
///   empty `Value::Object` → `Ok` holding the selection
///   `RandomSourceKind::Mock(mode)`.
/// Arrange: the identifier
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)`
///   and the payload `Value::Object` over `Map::new()`, a document that
///   supplies no setting and is not null, so a mock arm that refuses every
///   document but `Value::Null` fails.
/// Act:     `admit_random_selection`.
/// Assert:  the result equals `Ok` holding the success return built with the
///   selection override
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)`.
#[test]
fn the_mock_member_with_an_empty_object_document_returns_a_selection_carrying_its_failure_mode() {
    // Arrange
    let deps = build_admit_random_selection_deps(Default::default());
    let params = build_admit_random_selection_params(AdmitRandomSelectionParamsOverrides {
        identifier: Some(RandomSourceKind::Mock(
            MockIRandomSourceAdapterFailureMode::FillBytesRefused,
        )),
    });
    let payload = Value::Object(Map::new());

    // Act
    let result: AdmitRandomSelectionReturn = admit_random_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Ok(build_admit_random_selection_success_return(
            AdmitRandomSelectionSuccessReturnOverrides {
                selection: Some(RandomSourceKind::Mock(
                    MockIRandomSourceAdapterFailureMode::FillBytesRefused,
                )),
            }
        ))
    );
}
// the_mock_member_with_an_empty_object_document_returns_a_selection_carrying_its_failure_mode

/// Contract: the identifier
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::Succeeds)`
///   with the payload `Value::Null` → `Ok` holding the selection
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::Succeeds)`.
/// Arrange: the identifier
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::Succeeds)`
///   and the payload `Value::Null`, a mode that differs from `FillBytesRefused`,
///   which the other mock blocks arrange, so a mock arm that returns one fixed
///   mode fails.
/// Act:     `admit_random_selection`.
/// Assert:  the result equals `Ok` holding the success return built with the
///   selection override
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::Succeeds)`.
#[test]
fn the_mock_member_with_the_succeeds_mode_returns_a_selection_carrying_that_mode() {
    // Arrange
    let deps = build_admit_random_selection_deps(Default::default());
    let params = build_admit_random_selection_params(AdmitRandomSelectionParamsOverrides {
        identifier: Some(RandomSourceKind::Mock(
            MockIRandomSourceAdapterFailureMode::Succeeds,
        )),
    });
    let payload = Value::Null;

    // Act
    let result: AdmitRandomSelectionReturn = admit_random_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Ok(build_admit_random_selection_success_return(
            AdmitRandomSelectionSuccessReturnOverrides {
                selection: Some(RandomSourceKind::Mock(
                    MockIRandomSourceAdapterFailureMode::Succeeds,
                )),
            }
        ))
    );
}
// the_mock_member_with_the_succeeds_mode_returns_a_selection_carrying_that_mode
