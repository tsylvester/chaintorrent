#![allow(clippy::expect_used)]

use std::env::temp_dir;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;

use super::interface::LayersErrorReturn;
use super::layers;
use super::mock::{
    LayersDefaultsOverrides, LayersParamsOverrides, LayersPayloadOverrides, build_layers_defaults,
    build_layers_deps, build_layers_params, build_layers_payload,
};

/// Contract: Arguments refused → `Err(LayersErrorReturn::Arguments(error))`,
///   the error unchanged.
/// Arrange: a command line with the flag `--unknown`.
/// Act:     `layers`, once.
/// Assert:  the result matches `Err(LayersErrorReturn::Arguments(error))`
///   with `error.kind()` equal to `clap::error::ErrorKind::UnknownArgument`.
#[test]
fn layers_returns_a_refused_flag_unchanged() {
    // Arrange
    let params = build_layers_params(LayersParamsOverrides {
        arguments: Some(vec![
            OsString::from("harness-crypto"),
            OsString::from("--unknown"),
        ]),
    });
    let payload = build_layers_payload(Default::default());
    // Act
    let result = layers(&build_layers_deps(Default::default()), params, payload);
    // Assert
    assert!(matches!(
        &result,
        Err(LayersErrorReturn::Arguments(error))
            if error.kind() == clap::error::ErrorKind::UnknownArgument
    ));
    // layers_returns_a_refused_flag_unchanged
}

/// Contract: Defaults refused → `Err(LayersErrorReturn::ParseDefaults(error))`.
/// Arrange: a defaults text of `"= ="`.
/// Act:     `layers`, once.
/// Assert:  the result matches `Err(LayersErrorReturn::ParseDefaults(_))`.
#[test]
fn layers_returns_a_malformed_defaults_text_unchanged() {
    // Arrange
    let params = build_layers_params(Default::default());
    let payload = build_layers_payload(LayersPayloadOverrides {
        defaults: Some("= =".to_string()),
    });
    // Act
    let result = layers(&build_layers_deps(Default::default()), params, payload);
    // Assert
    assert!(matches!(&result, Err(LayersErrorReturn::ParseDefaults(_))));
    // layers_returns_a_malformed_defaults_text_unchanged
}

/// Contract: File layer → a refused read is
///   `Err(LayersErrorReturn::ReadFile { path, error })`, the error unchanged.
/// Arrange: `--config` naming a path beneath the block's directory that names
///   no file.
/// Act:     `layers`, once.
/// Assert:  the result matches `Err(LayersErrorReturn::ReadFile { path, .. })`
///   with `path` equal to the named path.
#[test]
fn layers_returns_an_unreadable_file_with_its_path() {
    // Arrange
    let directory: PathBuf = temp_dir().join(format!(
        "chaintorrent-layers-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = fs::remove_dir_all(&directory);
    fs::create_dir_all(&directory).expect("the directory is created");
    let missing = directory.join("missing.toml");
    let params = build_layers_params(LayersParamsOverrides {
        arguments: Some(vec![
            OsString::from("harness-crypto"),
            OsString::from("--config"),
            missing.clone().into_os_string(),
        ]),
    });
    let payload = build_layers_payload(Default::default());
    // Act
    let result = layers(&build_layers_deps(Default::default()), params, payload);
    // Assert
    assert!(matches!(
        &result,
        Err(LayersErrorReturn::ReadFile { path, .. }) if *path == missing
    ));
    // layers_returns_an_unreadable_file_with_its_path
}

/// Contract: File text refused → `Err(LayersErrorReturn::ParseFile { path,
///   error })`.
/// Arrange: a file whose text is `"= ="`, named by `--config`.
/// Act:     `layers`, once.
/// Assert:  the result matches `Err(LayersErrorReturn::ParseFile { path, .. })`
///   with `path` equal to the file's path.
#[test]
fn layers_returns_a_malformed_file_with_its_path() {
    // Arrange
    let directory: PathBuf = temp_dir().join(format!(
        "chaintorrent-layers-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = fs::remove_dir_all(&directory);
    fs::create_dir_all(&directory).expect("the directory is created");
    let file = directory.join("malformed.toml");
    fs::write(&file, "= =").expect("the file is written");
    let params = build_layers_params(LayersParamsOverrides {
        arguments: Some(vec![
            OsString::from("harness-crypto"),
            OsString::from("--config"),
            file.clone().into_os_string(),
        ]),
    });
    let payload = build_layers_payload(Default::default());
    // Act
    let result = layers(&build_layers_deps(Default::default()), params, payload);
    // Assert
    assert!(matches!(
        &result,
        Err(LayersErrorReturn::ParseFile { path, .. }) if *path == file
    ));
    // layers_returns_a_malformed_file_with_its_path
}

/// Contract: Assignment malformed →
///   `Err(LayersErrorReturn::MalformedAssignment { assignment })`.
/// Arrange: `--set` with `benchmark.iterations`.
/// Act:     `layers`, once.
/// Assert:  the result matches `Err(LayersErrorReturn::MalformedAssignment {
///   assignment })` with `assignment` equal to `"benchmark.iterations"`.
#[test]
fn layers_returns_an_assignment_without_an_equals_sign() {
    // Arrange
    let params = build_layers_params(LayersParamsOverrides {
        arguments: Some(vec![
            OsString::from("harness-crypto"),
            OsString::from("--set"),
            OsString::from("benchmark.iterations"),
        ]),
    });
    let payload = build_layers_payload(Default::default());
    // Act
    let result = layers(&build_layers_deps(Default::default()), params, payload);
    // Assert
    assert!(matches!(
        &result,
        Err(LayersErrorReturn::MalformedAssignment { assignment })
            if *assignment == "benchmark.iterations"
    ));
    // layers_returns_an_assignment_without_an_equals_sign
}

/// Contract: Assignment malformed → a key that is empty once trimmed is
///   refused.
/// Arrange: `--set` with `  =3`.
/// Act:     `layers`, once.
/// Assert:  the result matches `Err(LayersErrorReturn::MalformedAssignment {
///   assignment })` with `assignment` equal to `"  =3"`.
#[test]
fn layers_returns_an_assignment_with_an_empty_key() {
    // Arrange
    let params = build_layers_params(LayersParamsOverrides {
        arguments: Some(vec![
            OsString::from("harness-crypto"),
            OsString::from("--set"),
            OsString::from("  =3"),
        ]),
    });
    let payload = build_layers_payload(Default::default());
    // Act
    let result = layers(&build_layers_deps(Default::default()), params, payload);
    // Assert
    assert!(matches!(
        &result,
        Err(LayersErrorReturn::MalformedAssignment { assignment })
            if *assignment == "  =3"
    ));
    // layers_returns_an_assignment_with_an_empty_key
}

/// Contract: Assignment text refused → `Err(LayersErrorReturn::ParseAssignment
///   { assignment, error })`.
/// Arrange: `--set` with `selections.kem=kem-b`, an unquoted string.
/// Act:     `layers`, once.
/// Assert:  the result matches `Err(LayersErrorReturn::ParseAssignment {
///   assignment, .. })` with `assignment` equal to `"selections.kem=kem-b"`.
#[test]
fn layers_returns_an_assignment_whose_value_is_not_toml() {
    // Arrange
    let params = build_layers_params(LayersParamsOverrides {
        arguments: Some(vec![
            OsString::from("harness-crypto"),
            OsString::from("--set"),
            OsString::from("selections.kem=kem-b"),
        ]),
    });
    let payload = build_layers_payload(Default::default());
    // Act
    let result = layers(&build_layers_deps(Default::default()), params, payload);
    // Assert
    assert!(matches!(
        &result,
        Err(LayersErrorReturn::ParseAssignment { assignment, .. })
            if *assignment == "selections.kem=kem-b"
    ));
    // layers_returns_an_assignment_whose_value_is_not_toml
}

/// Contract: Document refused → an undeclared top-level key is
///   `Err(LayersErrorReturn::Document(error))`.
/// Arrange: `--set` with `undeclared=1`.
/// Act:     `layers`, once.
/// Assert:  the result matches `Err(LayersErrorReturn::Document(error))` with
///   `error.to_string()` containing `"undeclared"`.
#[test]
fn layers_refuses_a_key_the_document_does_not_declare_at_its_top_level() {
    // Arrange
    let params = build_layers_params(LayersParamsOverrides {
        arguments: Some(vec![
            OsString::from("harness-crypto"),
            OsString::from("--set"),
            OsString::from("undeclared=1"),
        ]),
    });
    let payload = build_layers_payload(Default::default());
    // Act
    let result = layers(&build_layers_deps(Default::default()), params, payload);
    // Assert
    assert!(matches!(
        &result,
        Err(LayersErrorReturn::Document(error))
            if error.to_string().contains("undeclared")
    ));
    // layers_refuses_a_key_the_document_does_not_declare_at_its_top_level
}

/// Contract: Document refused → an undeclared key in a table is refused.
/// Arrange: `--set` with `benchmark.undeclared=1`.
/// Act:     `layers`, once.
/// Assert:  the result matches `Err(LayersErrorReturn::Document(error))` with
///   `error.to_string()` containing `"undeclared"`.
#[test]
fn layers_refuses_a_key_a_table_does_not_declare() {
    // Arrange
    let params = build_layers_params(LayersParamsOverrides {
        arguments: Some(vec![
            OsString::from("harness-crypto"),
            OsString::from("--set"),
            OsString::from("benchmark.undeclared=1"),
        ]),
    });
    let payload = build_layers_payload(Default::default());
    // Act
    let result = layers(&build_layers_deps(Default::default()), params, payload);
    // Assert
    assert!(matches!(
        &result,
        Err(LayersErrorReturn::Document(error))
            if error.to_string().contains("undeclared")
    ));
    // layers_refuses_a_key_a_table_does_not_declare
}

/// Contract: Document refused → an undeclared key in a table nested in a
///   table is refused.
/// Arrange: `--set` with `network_profile.chain_identifier.undeclared="x"`.
/// Act:     `layers`, once.
/// Assert:  the result matches `Err(LayersErrorReturn::Document(error))` with
///   `error.to_string()` containing `"undeclared"`.
#[test]
fn layers_refuses_a_key_in_a_nested_table_it_does_not_declare() {
    // Arrange
    let params = build_layers_params(LayersParamsOverrides {
        arguments: Some(vec![
            OsString::from("harness-crypto"),
            OsString::from("--set"),
            OsString::from(r#"network_profile.chain_identifier.undeclared="x""#),
        ]),
    });
    let payload = build_layers_payload(Default::default());
    // Act
    let result = layers(&build_layers_deps(Default::default()), params, payload);
    // Assert
    assert!(matches!(
        &result,
        Err(LayersErrorReturn::Document(error))
            if error.to_string().contains("undeclared")
    ));
    // layers_refuses_a_key_in_a_nested_table_it_does_not_declare
}

/// Contract: Document refused → a defaults text that omits a required key is
///   refused.
/// Arrange: a defaults text of the default text without its `[benchmark]`
///   table, through `LayersDefaultsOverrides`.
/// Act:     `layers`, once.
/// Assert:  the result matches `Err(LayersErrorReturn::Document(error))` with
///   `error.to_string()` containing `"benchmark"`.
#[test]
fn layers_refuses_a_document_missing_a_required_key() {
    // Arrange
    let params = build_layers_params(Default::default());
    let payload = build_layers_payload(LayersPayloadOverrides {
        defaults: Some(build_layers_defaults(LayersDefaultsOverrides {
            text: Some(
                build_layers_defaults(Default::default())
                    .replace("[benchmark]\niterations = 1\n", ""),
            ),
        })),
    });
    // Act
    let result = layers(&build_layers_deps(Default::default()), params, payload);
    // Assert
    assert!(matches!(
        &result,
        Err(LayersErrorReturn::Document(error))
            if error.to_string().contains("benchmark")
    ));
    // layers_refuses_a_document_missing_a_required_key
}

/// Contract: Document refused → a value of a kind the key does not take is
///   refused.
/// Arrange: `--set` with `benchmark.iterations="three"`.
/// Act:     `layers`, once.
/// Assert:  the result matches `Err(LayersErrorReturn::Document(error))` with
///   `error.to_string()` containing `"iterations"`.
#[test]
fn layers_refuses_a_value_of_the_wrong_kind() {
    // Arrange
    let params = build_layers_params(LayersParamsOverrides {
        arguments: Some(vec![
            OsString::from("harness-crypto"),
            OsString::from("--set"),
            OsString::from(r#"benchmark.iterations="three""#),
        ]),
    });
    let payload = build_layers_payload(Default::default());
    // Act
    let result = layers(&build_layers_deps(Default::default()), params, payload);
    // Assert
    assert!(matches!(
        &result,
        Err(LayersErrorReturn::Document(error))
            if error.to_string().contains("iterations")
    ));
    // layers_refuses_a_value_of_the_wrong_kind
}

/// Contract: Ordering → a layer's refusal returns before any later layer is
///   read.
/// Arrange: `--config` naming a path that names no file, then `--set` with
///   `benchmark.iterations`.
/// Act:     `layers`, once.
/// Assert:  the result matches `Err(LayersErrorReturn::ReadFile { .. })`, not
///   the malformed assignment.
#[test]
fn layers_stops_at_the_first_refused_layer() {
    // Arrange
    let directory: PathBuf = temp_dir().join(format!(
        "chaintorrent-layers-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = fs::remove_dir_all(&directory);
    fs::create_dir_all(&directory).expect("the directory is created");
    let missing = directory.join("missing.toml");
    let params = build_layers_params(LayersParamsOverrides {
        arguments: Some(vec![
            OsString::from("harness-crypto"),
            OsString::from("--config"),
            missing.into_os_string(),
            OsString::from("--set"),
            OsString::from("benchmark.iterations"),
        ]),
    });
    let payload = build_layers_payload(Default::default());
    // Act
    let result = layers(&build_layers_deps(Default::default()), params, payload);
    // Assert
    assert!(matches!(&result, Err(LayersErrorReturn::ReadFile { .. })));
    // layers_stops_at_the_first_refused_layer
}
