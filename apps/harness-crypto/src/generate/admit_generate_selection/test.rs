#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use serde_json;

use super::super::factory::provides::{
    GenerateConcrete, GenerateConcreteKind, GenerateDeclarationOverrides,
    MockIGenerateAdapterFailureMode, build_generate_declaration,
};
use super::super::lookup_generate_declaration::provides::{
    LookupGenerateDeclarationDeps, LookupGenerateDeclarationParams,
    LookupGenerateDeclarationPayload, LookupGenerateDeclarationReturn,
    LookupGenerateDeclarationSuccessReturnOverrides,
    build_lookup_generate_declaration_success_return, mock_lookup_generate_declaration,
};
use super::super::solidity::provides::{
    AdmitSolidityGenerateSettingsDeps, AdmitSolidityGenerateSettingsErrorReturn,
    AdmitSolidityGenerateSettingsParams, AdmitSolidityGenerateSettingsPayload,
    AdmitSolidityGenerateSettingsReturn, AdmitSolidityGenerateSettingsSuccessReturnOverrides,
    SolidityGenerateSettingsDocumentCorruptions, SolidityGenerateSettingsDocumentErrorReturn,
    SolidityGenerateSettingsOverrides, SpdxLicenseIdentifierConstructorParamsOverrides,
    build_admit_solidity_generate_settings_success_return, build_solidity_generate_settings,
    build_spdx_license_identifier, invalidate_solidity_generate_settings_document,
    mock_admit_solidity_generate_settings,
};
use super::admit_generate_selection;
use super::interface::*;
use super::mock::*;

/// Contract: the identifier `GenerateConcreteKind::Mock` → `Ok` holding
///   `GenerateConcrete::Mock(MockIGenerateAdapterFailureMode::NoFailure)`.
/// Arrange: the identifier `Mock` and a payload `serde_json::json!(3)`, a
///   value the Solidity guard refuses, so a branch that read the payload
///   would refuse.
/// Act:    `admit_generate_selection`.
/// Assert: the result equals `Ok` of the success return built over the
///   selection `GenerateConcrete::Mock(MockIGenerateAdapterFailureMode::
///   NoFailure)`.
#[test]
fn the_mock_member_selects_the_mock_concrete_under_no_failure_without_reading_the_payload() {
    // Arrange
    let deps = build_admit_generate_selection_deps(
        mock_lookup_generate_declaration,
        mock_admit_solidity_generate_settings,
        Default::default(),
    );
    let params = build_admit_generate_selection_params(AdmitGenerateSelectionParamsOverrides {
        identifier: Some(GenerateConcreteKind::Mock),
    });
    let payload = serde_json::json!(3);

    // Act
    let result = admit_generate_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Ok(build_admit_generate_selection_success_return(
            AdmitGenerateSelectionSuccessReturnOverrides {
                selection: Some(GenerateConcrete::Mock(
                    MockIGenerateAdapterFailureMode::NoFailure,
                )),
            },
        ))
    );
}
// the_mock_member_selects_the_mock_concrete_under_no_failure_without_reading_the_payload

/// Contract: the guard refuses the payload → `Err(Solidity(Document(error)))`
///   with the guard's error unchanged, and the admission handle is not
///   called.
/// Arrange: the identifier `Solidity`, the payload
///   `invalidate_solidity_generate_settings_document` with `license` `7`,
///   and as the admission handle a production-typed function returning
///   `Err(AdmitSolidityGenerateSettingsErrorReturn::UndeclaredSetting {
///   setting: "admission".into() })`, so a subject that admits before
///   narrowing, or admits at all, returns that error.
/// Act:    `admit_generate_selection`.
/// Assert: `error` equals `AdmitGenerateSelectionErrorReturn::Solidity(
///   AdmitGenerateSelectionSolidityErrorReturn::Document(
///   SolidityGenerateSettingsDocumentErrorReturn::NotText { path:
///   "license".into() }))`.
#[test]
fn the_guards_refusal_is_returned_in_the_document_variant_before_the_admission_runs() {
    // Arrange
    fn admission_refuses_with_undeclared_setting<'a>(
        _deps: &AdmitSolidityGenerateSettingsDeps<'a>,
        _params: AdmitSolidityGenerateSettingsParams,
        _payload: AdmitSolidityGenerateSettingsPayload,
    ) -> AdmitSolidityGenerateSettingsReturn {
        Err(
            AdmitSolidityGenerateSettingsErrorReturn::UndeclaredSetting {
                setting: "admission".into(),
            },
        )
    }
    let deps = build_admit_generate_selection_deps(
        mock_lookup_generate_declaration,
        admission_refuses_with_undeclared_setting,
        Default::default(),
    );
    let params = build_admit_generate_selection_params(AdmitGenerateSelectionParamsOverrides {
        identifier: Some(GenerateConcreteKind::Solidity),
    });
    let payload = invalidate_solidity_generate_settings_document(
        SolidityGenerateSettingsDocumentCorruptions {
            license: Some(serde_json::json!(7)),
            ..Default::default()
        },
    );

    // Act
    let result = admit_generate_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateSelectionErrorReturn::Solidity(
            AdmitGenerateSelectionSolidityErrorReturn::Document(
                SolidityGenerateSettingsDocumentErrorReturn::NotText {
                    path: "license".into(),
                },
            ),
        ))
    );
}
// the_guards_refusal_is_returned_in_the_document_variant_before_the_admission_runs

/// Contract: the declaration the lookup handle returns for
///   `params.identifier` → the admission handle's `deps.declaration`.
/// Arrange: the identifier `Solidity`, the payload `serde_json::json!({})`,
///   as the lookup handle a production-typed function returning the
///   declaration named `LookedUp` for `GenerateConcreteKind::Solidity` and
///   `OtherIdentifier` for any other identifier, and as the admission
///   handle a production-typed function refusing with the declaration's
///   name, so a subject that names a member's declaration constant, whose
///   name is `Solidity`, or that looks up another identifier, fails.
/// Act:    `admit_generate_selection`.
/// Assert: `error` equals `AdmitGenerateSelectionErrorReturn::Solidity(
///   AdmitGenerateSelectionSolidityErrorReturn::Settings(
///   AdmitSolidityGenerateSettingsErrorReturn::UndeclaredSetting {
///   setting: "LookedUp".into() }))`.
#[test]
fn the_declaration_the_lookup_returns_for_the_identifier_is_handed_to_the_admission() {
    // Arrange
    fn lookup_returns_named_declarations(
        _deps: &LookupGenerateDeclarationDeps,
        params: LookupGenerateDeclarationParams,
        _payload: LookupGenerateDeclarationPayload,
    ) -> LookupGenerateDeclarationReturn {
        let name = if params.identifier == GenerateConcreteKind::Solidity {
            "LookedUp"
        } else {
            "OtherIdentifier"
        };
        Ok(build_lookup_generate_declaration_success_return(
            LookupGenerateDeclarationSuccessReturnOverrides {
                declaration: Some(build_generate_declaration(GenerateDeclarationOverrides {
                    name: Some(name),
                    ..Default::default()
                })),
            },
        ))
    }
    fn admission_refuses_with_the_declarations_name<'a>(
        deps: &AdmitSolidityGenerateSettingsDeps<'a>,
        _params: AdmitSolidityGenerateSettingsParams,
        _payload: AdmitSolidityGenerateSettingsPayload,
    ) -> AdmitSolidityGenerateSettingsReturn {
        Err(
            AdmitSolidityGenerateSettingsErrorReturn::UndeclaredSetting {
                setting: deps.declaration.name.into(),
            },
        )
    }
    let deps = build_admit_generate_selection_deps(
        lookup_returns_named_declarations,
        admission_refuses_with_the_declarations_name,
        Default::default(),
    );
    let params = build_admit_generate_selection_params(AdmitGenerateSelectionParamsOverrides {
        identifier: Some(GenerateConcreteKind::Solidity),
    });
    let payload = serde_json::json!({});

    // Act
    let result = admit_generate_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateSelectionErrorReturn::Solidity(
            AdmitGenerateSelectionSolidityErrorReturn::Settings(
                AdmitSolidityGenerateSettingsErrorReturn::UndeclaredSetting {
                    setting: "LookedUp".into(),
                },
            ),
        ))
    );
}
// the_declaration_the_lookup_returns_for_the_identifier_is_handed_to_the_admission

/// Contract: the admission handle refuses → `Err(Solidity(Settings(error)))`
///   with the admission's error unchanged.
/// Arrange: the identifier `Solidity`, the payload `serde_json::json!({})`,
///   which the guard admits, `mock_lookup_generate_declaration` as the
///   lookup handle, and as the admission handle a production-typed
///   function returning `Err(AdmitSolidityGenerateSettingsErrorReturn::
///   OutOfBounds { setting: "compiler_version".into(), supplied:
///   "0.9.0".into() })`, an error the guard never produces, so a refusal
///   carried in the document variant or reshaped fails.
/// Act:    `admit_generate_selection`.
/// Assert: `error` equals `AdmitGenerateSelectionErrorReturn::Solidity(
///   AdmitGenerateSelectionSolidityErrorReturn::Settings(
///   AdmitSolidityGenerateSettingsErrorReturn::OutOfBounds { setting:
///   "compiler_version".into(), supplied: "0.9.0".into() }))`.
#[test]
fn the_admissions_refusal_is_returned_unchanged_in_the_settings_variant() {
    // Arrange
    fn admission_refuses_out_of_bounds<'a>(
        _deps: &AdmitSolidityGenerateSettingsDeps<'a>,
        _params: AdmitSolidityGenerateSettingsParams,
        _payload: AdmitSolidityGenerateSettingsPayload,
    ) -> AdmitSolidityGenerateSettingsReturn {
        Err(AdmitSolidityGenerateSettingsErrorReturn::OutOfBounds {
            setting: "compiler_version".into(),
            supplied: "0.9.0".into(),
        })
    }
    let deps = build_admit_generate_selection_deps(
        mock_lookup_generate_declaration,
        admission_refuses_out_of_bounds,
        Default::default(),
    );
    let params = build_admit_generate_selection_params(AdmitGenerateSelectionParamsOverrides {
        identifier: Some(GenerateConcreteKind::Solidity),
    });
    let payload = serde_json::json!({});

    // Act
    let result = admit_generate_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateSelectionErrorReturn::Solidity(
            AdmitGenerateSelectionSolidityErrorReturn::Settings(
                AdmitSolidityGenerateSettingsErrorReturn::OutOfBounds {
                    setting: "compiler_version".into(),
                    supplied: "0.9.0".into(),
                },
            ),
        ))
    );
}
// the_admissions_refusal_is_returned_unchanged_in_the_settings_variant

/// Contract: the admission handle's settings → `Ok` whose selection is
///   `GenerateConcrete::Solidity` boxing those settings.
/// Arrange: the identifier `Solidity`, the payload `serde_json::json!({})`,
///   `mock_lookup_generate_declaration` as the lookup handle, and as the
///   admission handle a production-typed function returning settings whose
///   license is `MIT`, which differs from the declared default
///   `UNLICENSED`, so a subject that ignores the handle's settings, or
///   admits or builds settings of its own, fails.
/// Act:    `admit_generate_selection`.
/// Assert: the result equals `Ok` of the success return built over the
///   selection `GenerateConcrete::Solidity` boxing those same settings,
///   the transform under test being the member's variant and the box
///   around the handle's settings.
#[test]
fn the_admitted_settings_are_selected_boxed_in_the_solidity_variant() {
    // Arrange
    fn admission_admits_the_mit_settings<'a>(
        _deps: &AdmitSolidityGenerateSettingsDeps<'a>,
        _params: AdmitSolidityGenerateSettingsParams,
        _payload: AdmitSolidityGenerateSettingsPayload,
    ) -> AdmitSolidityGenerateSettingsReturn {
        Ok(build_admit_solidity_generate_settings_success_return(
            AdmitSolidityGenerateSettingsSuccessReturnOverrides {
                settings: Some(build_solidity_generate_settings(
                    SolidityGenerateSettingsOverrides {
                        license: Some(build_spdx_license_identifier(
                            SpdxLicenseIdentifierConstructorParamsOverrides {
                                text: Some("MIT".into()),
                            },
                        )),
                        ..Default::default()
                    },
                )),
            },
        ))
    }
    let deps = build_admit_generate_selection_deps(
        mock_lookup_generate_declaration,
        admission_admits_the_mit_settings,
        Default::default(),
    );
    let params = build_admit_generate_selection_params(AdmitGenerateSelectionParamsOverrides {
        identifier: Some(GenerateConcreteKind::Solidity),
    });
    let payload = serde_json::json!({});

    // Act
    let result = admit_generate_selection(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Ok(build_admit_generate_selection_success_return(
            AdmitGenerateSelectionSuccessReturnOverrides {
                selection: Some(GenerateConcrete::Solidity {
                    settings: Box::new(build_solidity_generate_settings(
                        SolidityGenerateSettingsOverrides {
                            license: Some(build_spdx_license_identifier(
                                SpdxLicenseIdentifierConstructorParamsOverrides {
                                    text: Some("MIT".into()),
                                },
                            )),
                            ..Default::default()
                        },
                    )),
                }),
            },
        ))
    );
}
// the_admitted_settings_are_selected_boxed_in_the_solidity_variant
