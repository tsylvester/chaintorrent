#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::env::temp_dir;
use std::ffi::OsString;
use std::io::ErrorKind;
use std::process::id;

use proof::UndeclaredDeliveryPurposeCode;

use super::admit_generate::provides::{AdmitGenerateErrorReturn, admit_generate};
use super::admit_output::provides::{AdmitOutputErrorReturn, admit_output};
use super::admit_selections::provides::{
    AdmitSelectionsErrorReturn, SelectionField, admit_selections,
};
use super::admit_statements::provides::{AdmitStatementsErrorReturn, admit_statements};
use super::configure;
use super::interface::*;
use super::layers::provides::{LayersErrorReturn, layers};
use super::mock::{
    ConfigureDepsOverrides, ConfigurePayloadOverrides, build_configure_deps,
    build_configure_params, build_configure_payload,
};
use crate::generate::provides::SpdxLicenseIdentifierTryNewErrorReturn;

/// Contract: the own entry `a_refused_selection_reaches_the_caller_unchanged`:
///   a command-line value `admit_selections` refuses reaches the caller as
///   `Err(ConfigureErrorReturn::Selections(error))`, the refusal whole.
/// Arrange: `deps` naming the real `layers`, `admit_selections`,
///   `admit_statements`, `admit_generate`, and `admit_output`; the payload
///   `build_configure_payload` over `HARNESS_DEFAULTS` with `arguments`
///   `"harness-crypto"`, `"--set"`, and `kem.scope="holder"`.
/// Act:     `configure(&deps, params, payload)`.
/// Assert:  the refusal is `ConfigureErrorReturn::Selections(error)` with
///   `error` equal to `AdmitSelectionsErrorReturn::UnknownValue { field:
///   SelectionField::KemScope, text: "holder" }`.
/// Boundary: the real chain `configure`, `layers`, `admit_selections`,
///   `admit_statements`, `admit_generate`, and `admit_output` through
///   crate-internal paths.
/// Mocked:  no collaborator is mocked; the filesystem the `--config` paths
///   name is the outer edge, so the block does not prove an operating-system
///   read failure other than a missing file.
#[test]
fn a_refused_selection_reaches_the_caller_unchanged() {
    // Arrange
    let deps = build_configure_deps(ConfigureDepsOverrides {
        layers: Some(layers),
        admit_selections: Some(admit_selections),
        admit_statements: Some(admit_statements),
        admit_generate: Some(admit_generate),
        admit_output: Some(admit_output),
    });
    let params = build_configure_params(Default::default());
    let payload = build_configure_payload(
        HARNESS_DEFAULTS,
        ConfigurePayloadOverrides {
            arguments: Some(vec![
                OsString::from("harness-crypto"),
                OsString::from("--set"),
                OsString::from("kem.scope=\"holder\""),
            ]),
        },
    );

    // Act
    let result = configure(&deps, params, payload);

    // Assert
    let Err(ConfigureErrorReturn::Selections(error)) = result else {
        panic!("expected the selections refusal");
    };
    assert_eq!(
        error,
        AdmitSelectionsErrorReturn::UnknownValue {
            field: SelectionField::KemScope,
            text: "holder".to_string(),
        }
    );
    // a_refused_selection_reaches_the_caller_unchanged
}

/// Contract: the own entry
///   `a_refused_statement_purpose_reaches_the_caller_unchanged`: a purpose
///   code `admit_statements` refuses reaches the caller as
///   `Err(ConfigureErrorReturn::Statements(error))`, the refusal whole.
/// Arrange: `deps` naming the real functions; the payload over
///   `HARNESS_DEFAULTS` with `arguments` `"harness-crypto"`, `"--set"`, and
///   `statements.mint_purpose=2`.
/// Act:     `configure(&deps, params, payload)`.
/// Assert:  the refusal is `ConfigureErrorReturn::Statements(error)` with
///   `error` equal to `AdmitStatementsErrorReturn::MintPurpose(
///   UndeclaredDeliveryPurposeCode { code: 2 })`.
/// Boundary: the real chain `configure`, `layers`, `admit_selections`,
///   `admit_statements`, `admit_generate`, and `admit_output` through
///   crate-internal paths.
/// Mocked:  no collaborator is mocked; the filesystem the `--config` paths
///   name is the outer edge, so the block does not prove an operating-system
///   read failure other than a missing file.
#[test]
fn a_refused_statement_purpose_reaches_the_caller_unchanged() {
    // Arrange
    let deps = build_configure_deps(ConfigureDepsOverrides {
        layers: Some(layers),
        admit_selections: Some(admit_selections),
        admit_statements: Some(admit_statements),
        admit_generate: Some(admit_generate),
        admit_output: Some(admit_output),
    });
    let params = build_configure_params(Default::default());
    let payload = build_configure_payload(
        HARNESS_DEFAULTS,
        ConfigurePayloadOverrides {
            arguments: Some(vec![
                OsString::from("harness-crypto"),
                OsString::from("--set"),
                OsString::from("statements.mint_purpose=2"),
            ]),
        },
    );

    // Act
    let result = configure(&deps, params, payload);

    // Assert
    let Err(ConfigureErrorReturn::Statements(error)) = result else {
        panic!("expected the statements refusal");
    };
    assert_eq!(
        error,
        AdmitStatementsErrorReturn::MintPurpose(UndeclaredDeliveryPurposeCode { code: 2 })
    );
    // a_refused_statement_purpose_reaches_the_caller_unchanged
}

/// Contract: the own entry
///   `a_refused_generate_setting_reaches_the_caller_unchanged`: a license
///   `admit_generate` refuses reaches the caller as
///   `Err(ConfigureErrorReturn::Generate(error))`, the refusal whole.
/// Arrange: `deps` naming the real functions; the payload over
///   `HARNESS_DEFAULTS` with `arguments` `"harness-crypto"`, `"--set"`, and
///   `generate.license=""`.
/// Act:     `configure(&deps, params, payload)`.
/// Assert:  the refusal is `ConfigureErrorReturn::Generate(error)` with
///   `error` equal to `AdmitGenerateErrorReturn::License(
///   SpdxLicenseIdentifierTryNewErrorReturn::Empty)`.
/// Boundary: the real chain `configure`, `layers`, `admit_selections`,
///   `admit_statements`, `admit_generate`, and `admit_output` through
///   crate-internal paths.
/// Mocked:  no collaborator is mocked; the filesystem the `--config` paths
///   name is the outer edge, so the block does not prove an operating-system
///   read failure other than a missing file.
#[test]
fn a_refused_generate_setting_reaches_the_caller_unchanged() {
    // Arrange
    let deps = build_configure_deps(ConfigureDepsOverrides {
        layers: Some(layers),
        admit_selections: Some(admit_selections),
        admit_statements: Some(admit_statements),
        admit_generate: Some(admit_generate),
        admit_output: Some(admit_output),
    });
    let params = build_configure_params(Default::default());
    let payload = build_configure_payload(
        HARNESS_DEFAULTS,
        ConfigurePayloadOverrides {
            arguments: Some(vec![
                OsString::from("harness-crypto"),
                OsString::from("--set"),
                OsString::from("generate.license=\"\""),
            ]),
        },
    );

    // Act
    let result = configure(&deps, params, payload);

    // Assert
    let Err(ConfigureErrorReturn::Generate(error)) = result else {
        panic!("expected the generate refusal");
    };
    assert_eq!(
        error,
        AdmitGenerateErrorReturn::License(SpdxLicenseIdentifierTryNewErrorReturn::Empty)
    );
    // a_refused_generate_setting_reaches_the_caller_unchanged
}

/// Contract: the own entry
///   `a_refused_output_destination_reaches_the_caller_unchanged`: a
///   destination kind `admit_output` refuses reaches the caller as
///   `Err(ConfigureErrorReturn::Output(error))`, the refusal whole.
/// Arrange: `deps` naming the real functions; the payload over
///   `HARNESS_DEFAULTS` with `arguments` `"harness-crypto"`, `"--set"`, and
///   `output.destinations=[{kind="screen"}]`.
/// Act:     `configure(&deps, params, payload)`.
/// Assert:  the refusal is `ConfigureErrorReturn::Output(error)` with
///   `error` equal to `AdmitOutputErrorReturn::UnknownKind { index: 0,
///   text: "screen" }`.
/// Boundary: the real chain `configure`, `layers`, `admit_selections`,
///   `admit_statements`, `admit_generate`, and `admit_output` through
///   crate-internal paths.
/// Mocked:  no collaborator is mocked; the filesystem the `--config` paths
///   name is the outer edge, so the block does not prove an operating-system
///   read failure other than a missing file.
#[test]
fn a_refused_output_destination_reaches_the_caller_unchanged() {
    // Arrange
    let deps = build_configure_deps(ConfigureDepsOverrides {
        layers: Some(layers),
        admit_selections: Some(admit_selections),
        admit_statements: Some(admit_statements),
        admit_generate: Some(admit_generate),
        admit_output: Some(admit_output),
    });
    let params = build_configure_params(Default::default());
    let payload = build_configure_payload(
        HARNESS_DEFAULTS,
        ConfigurePayloadOverrides {
            arguments: Some(vec![
                OsString::from("harness-crypto"),
                OsString::from("--set"),
                OsString::from("output.destinations=[{kind=\"screen\"}]"),
            ]),
        },
    );

    // Act
    let result = configure(&deps, params, payload);

    // Assert
    let Err(ConfigureErrorReturn::Output(error)) = result else {
        panic!("expected the output refusal");
    };
    assert_eq!(
        error,
        AdmitOutputErrorReturn::UnknownKind {
            index: 0,
            text: "screen".to_string(),
        }
    );
    // a_refused_output_destination_reaches_the_caller_unchanged
}

/// Contract: the own entry `zero_benchmark_iterations_are_refused`: a zero
///   count the document deserializes and every admission accepts is refused
///   by the composition's own nonzero admission.
/// Arrange: `deps` naming the real functions; the payload over
///   `HARNESS_DEFAULTS` with `arguments` `"harness-crypto"`, `"--set"`, and
///   `benchmark.iterations=0`.
/// Act:     `configure(&deps, params, payload)`.
/// Assert:  the refusal is `ConfigureErrorReturn::ZeroBenchmarkIterations`.
/// Boundary: the real chain `configure`, `layers`, `admit_selections`,
///   `admit_statements`, `admit_generate`, and `admit_output` through
///   crate-internal paths.
/// Mocked:  no collaborator is mocked; the filesystem the `--config` paths
///   name is the outer edge, so the block does not prove an operating-system
///   read failure other than a missing file.
#[test]
fn zero_benchmark_iterations_are_refused() {
    // Arrange
    let deps = build_configure_deps(ConfigureDepsOverrides {
        layers: Some(layers),
        admit_selections: Some(admit_selections),
        admit_statements: Some(admit_statements),
        admit_generate: Some(admit_generate),
        admit_output: Some(admit_output),
    });
    let params = build_configure_params(Default::default());
    let payload = build_configure_payload(
        HARNESS_DEFAULTS,
        ConfigurePayloadOverrides {
            arguments: Some(vec![
                OsString::from("harness-crypto"),
                OsString::from("--set"),
                OsString::from("benchmark.iterations=0"),
            ]),
        },
    );

    // Act
    let result = configure(&deps, params, payload);

    // Assert
    let Err(ConfigureErrorReturn::ZeroBenchmarkIterations) = result else {
        panic!("expected the zero iteration refusal");
    };
    // zero_benchmark_iterations_are_refused
}

/// Contract: the own entry
///   `an_unreadable_configuration_file_reaches_the_caller_unchanged`: a
///   `--config` path the filesystem does not hold reaches the caller as
///   `Err(ConfigureErrorReturn::Layers(LayersErrorReturn::ReadFile { path,
///   error }))`, the path and the error intact.
/// Arrange: `deps` naming the real functions; the payload over
///   `HARNESS_DEFAULTS` with `arguments` `"harness-crypto"`, `"--config"`,
///   and a path under `std::env::temp_dir()` that is never written.
/// Act:     `configure(&deps, params, payload)`.
/// Assert:  the refusal is `ConfigureErrorReturn::Layers(LayersErrorReturn::
///   ReadFile { path, error })` with `path` the named path and
///   `error.kind()` `ErrorKind::NotFound`.
/// Boundary: the real chain `configure`, `layers`, `admit_selections`,
///   `admit_statements`, `admit_generate`, and `admit_output` through
///   crate-internal paths.
/// Mocked:  no collaborator is mocked; the filesystem the `--config` paths
///   name is the outer edge, so the block does not prove an operating-system
///   read failure other than a missing file.
#[test]
fn an_unreadable_configuration_file_reaches_the_caller_unchanged() {
    // Arrange
    let missing = temp_dir().join(format!("chaintorrent-configure-missing-{}.toml", id()));
    let deps = build_configure_deps(ConfigureDepsOverrides {
        layers: Some(layers),
        admit_selections: Some(admit_selections),
        admit_statements: Some(admit_statements),
        admit_generate: Some(admit_generate),
        admit_output: Some(admit_output),
    });
    let params = build_configure_params(Default::default());
    let payload = build_configure_payload(
        HARNESS_DEFAULTS,
        ConfigurePayloadOverrides {
            arguments: Some(vec![
                OsString::from("harness-crypto"),
                OsString::from("--config"),
                missing.clone().into_os_string(),
            ]),
        },
    );

    // Act
    let result = configure(&deps, params, payload);

    // Assert
    let Err(ConfigureErrorReturn::Layers(LayersErrorReturn::ReadFile { path, error })) = result
    else {
        panic!("expected the layers refusal");
    };
    assert_eq!(path, missing);
    assert_eq!(error.kind(), ErrorKind::NotFound);
    // an_unreadable_configuration_file_reaches_the_caller_unchanged
}

/// Contract: the own entry
///   `the_first_refusal_in_composition_order_is_returned`: a generate
///   refusal and a selections refusal on the command line return the
///   selections refusal, composed first.
/// Arrange: `deps` naming the real functions; the payload over
///   `HARNESS_DEFAULTS` with `arguments` `"harness-crypto"`, then `"--set"`
///   `generate.license=""`, then `"--set"` `kem.scope="holder"`, a refusal
///   for each of two admissions in the order opposite to the composition's.
/// Act:     `configure(&deps, params, payload)`.
/// Assert:  the refusal is `ConfigureErrorReturn::Selections(error)` with
///   `error` equal to `AdmitSelectionsErrorReturn::UnknownValue { field:
///   SelectionField::KemScope, text: "holder" }`.
/// Boundary: the real chain `configure`, `layers`, `admit_selections`,
///   `admit_statements`, `admit_generate`, and `admit_output` through
///   crate-internal paths.
/// Mocked:  no collaborator is mocked; the filesystem the `--config` paths
///   name is the outer edge, so the block does not prove an operating-system
///   read failure other than a missing file.
#[test]
fn the_first_refusal_in_composition_order_is_returned() {
    // Arrange
    let deps = build_configure_deps(ConfigureDepsOverrides {
        layers: Some(layers),
        admit_selections: Some(admit_selections),
        admit_statements: Some(admit_statements),
        admit_generate: Some(admit_generate),
        admit_output: Some(admit_output),
    });
    let params = build_configure_params(Default::default());
    let payload = build_configure_payload(
        HARNESS_DEFAULTS,
        ConfigurePayloadOverrides {
            arguments: Some(vec![
                OsString::from("harness-crypto"),
                OsString::from("--set"),
                OsString::from("generate.license=\"\""),
                OsString::from("--set"),
                OsString::from("kem.scope=\"holder\""),
            ]),
        },
    );

    // Act
    let result = configure(&deps, params, payload);

    // Assert
    let Err(ConfigureErrorReturn::Selections(error)) = result else {
        panic!("expected the selections refusal");
    };
    assert_eq!(
        error,
        AdmitSelectionsErrorReturn::UnknownValue {
            field: SelectionField::KemScope,
            text: "holder".to_string(),
        }
    );
    // the_first_refusal_in_composition_order_is_returned
}
