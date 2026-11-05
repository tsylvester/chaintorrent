#![allow(clippy::expect_used)]

use std::env::temp_dir;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;

use super::layers;
use super::mock::{
    BenchmarkTableOverrides, ConfigDocumentOverrides, LayersParamsOverrides,
    SelectionsTableOverrides, build_benchmark_table, build_config_document, build_layers_deps,
    build_layers_params, build_layers_payload, build_selections_table,
};

/// Contract: Defaults admitted and Document admitted → the document is the
///   defaults text's own values.
/// Arrange: the program name alone and the default defaults text.
/// Act:     `layers`, once.
/// Assert:  the result is `Ok` and its document equals the built defaults
///   document.
#[test]
fn layers_returns_the_defaults_when_no_source_is_named() {
    // Arrange
    let params = build_layers_params(Default::default());
    let payload = build_layers_payload(Default::default());
    let expected = build_config_document(Default::default());
    // Act
    let document = layers(&build_layers_deps(Default::default()), params, payload)
        .expect("the layering succeeds")
        .document;
    // Assert
    assert_eq!(document, expected);
    // layers_returns_the_defaults_when_no_source_is_named
}

/// Contract: File layer → a key the file names takes the file's value and
///   every other key keeps the defaults'.
/// Arrange: a file declaring `[benchmark]` with `iterations = 7`, named by
///   `--config`.
/// Act:     `layers`, once.
/// Assert:  the document equals the built defaults document with `iterations`
///   `7` under `benchmark`.
#[test]
fn layers_applies_a_file_over_the_defaults() {
    // Arrange
    let directory: PathBuf = temp_dir().join(format!(
        "chaintorrent-layers-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = fs::remove_dir_all(&directory);
    fs::create_dir_all(&directory).expect("the directory is created");
    let file = directory.join("benchmark.toml");
    fs::write(&file, "[benchmark]\niterations = 7\n").expect("the file is written");
    let params = build_layers_params(LayersParamsOverrides {
        arguments: Some(vec![
            OsString::from("harness-crypto"),
            OsString::from("--config"),
            file.into_os_string(),
        ]),
    });
    let payload = build_layers_payload(Default::default());
    let expected = build_config_document(ConfigDocumentOverrides {
        benchmark: Some(build_benchmark_table(BenchmarkTableOverrides {
            iterations: Some(7),
        })),
        ..Default::default()
    });
    // Act
    let document = layers(&build_layers_deps(Default::default()), params, payload)
        .expect("the layering succeeds")
        .document;
    // Assert
    assert_eq!(document, expected);
    // layers_applies_a_file_over_the_defaults
}

/// Contract: Assignment text → the assignment's value, parsed as TOML,
///   replaces the defaults' value.
/// Arrange: `--set` with `benchmark.iterations=9`.
/// Act:     `layers`, once.
/// Assert:  the document equals the built defaults document with `iterations`
///   `9` under `benchmark`.
#[test]
fn layers_applies_an_assignment_over_the_defaults() {
    // Arrange
    let params = build_layers_params(LayersParamsOverrides {
        arguments: Some(vec![
            OsString::from("harness-crypto"),
            OsString::from("--set"),
            OsString::from("benchmark.iterations=9"),
        ]),
    });
    let payload = build_layers_payload(Default::default());
    let expected = build_config_document(ConfigDocumentOverrides {
        benchmark: Some(build_benchmark_table(BenchmarkTableOverrides {
            iterations: Some(9),
        })),
        ..Default::default()
    });
    // Act
    let document = layers(&build_layers_deps(Default::default()), params, payload)
        .expect("the layering succeeds")
        .document;
    // Assert
    assert_eq!(document, expected);
    // layers_applies_an_assignment_over_the_defaults
}

/// Contract: Assignment text → a quoted value is a TOML string.
/// Arrange: `--set` with `selections.kem="kem-b"`.
/// Act:     `layers`, once.
/// Assert:  the document's `selections.kem` equals `"kem-b"` and every other
///   value is the defaults'.
#[test]
fn layers_parses_a_string_assignment_value_as_a_toml_string() {
    // Arrange
    let params = build_layers_params(LayersParamsOverrides {
        arguments: Some(vec![
            OsString::from("harness-crypto"),
            OsString::from("--set"),
            OsString::from(r#"selections.kem="kem-b""#),
        ]),
    });
    let payload = build_layers_payload(Default::default());
    let expected = build_config_document(ConfigDocumentOverrides {
        selections: Some(build_selections_table(SelectionsTableOverrides {
            kem: Some("kem-b".to_string()),
            ..Default::default()
        })),
        ..Default::default()
    });
    // Act
    let document = layers(&build_layers_deps(Default::default()), params, payload)
        .expect("the layering succeeds")
        .document;
    // Assert
    assert_eq!(document, expected);
    // layers_parses_a_string_assignment_value_as_a_toml_string
}

/// Contract: Merge → a layer naming one key of a table leaves its sibling
///   keys at the earlier layers' values.
/// Arrange: a file declaring `[measurement]` with `conditions = "conditions-b"`
///   alone.
/// Act:     `layers`, once.
/// Assert:  the document's `measurement.conditions` equals `"conditions-b"`
///   and `measurement.iterations` equals the defaults' value.
#[test]
fn layers_merges_a_table_key_by_key() {
    // Arrange
    let directory: PathBuf = temp_dir().join(format!(
        "chaintorrent-layers-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = fs::remove_dir_all(&directory);
    fs::create_dir_all(&directory).expect("the directory is created");
    let file = directory.join("measurement.toml");
    fs::write(&file, "[measurement]\nconditions = \"conditions-b\"\n")
        .expect("the file is written");
    let params = build_layers_params(LayersParamsOverrides {
        arguments: Some(vec![
            OsString::from("harness-crypto"),
            OsString::from("--config"),
            file.into_os_string(),
        ]),
    });
    let payload = build_layers_payload(Default::default());
    // Act
    let document = layers(&build_layers_deps(Default::default()), params, payload)
        .expect("the layering succeeds")
        .document;
    // Assert
    assert_eq!(document.measurement.conditions, "conditions-b");
    assert_eq!(document.measurement.iterations, 1);
    // layers_merges_a_table_key_by_key
}

/// Contract: Merge → an array a layer names replaces the earlier array and
///   is not extended.
/// Arrange: a file declaring `[identities]` with `mapping` of two entries,
///   differing from the defaults' one entry.
/// Act:     `layers`, once.
/// Assert:  the document's `identities.mapping` equals the file's two entries
///   in order.
#[test]
fn layers_replaces_an_array_whole() {
    // Arrange
    let directory: PathBuf = temp_dir().join(format!(
        "chaintorrent-layers-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = fs::remove_dir_all(&directory);
    fs::create_dir_all(&directory).expect("the directory is created");
    let file = directory.join("identities.toml");
    fs::write(
        &file,
        "[identities]\nmapping = [\"0xcccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc\", \"0xdddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd\"]\n",
    )
    .expect("the file is written");
    let params = build_layers_params(LayersParamsOverrides {
        arguments: Some(vec![
            OsString::from("harness-crypto"),
            OsString::from("--config"),
            file.into_os_string(),
        ]),
    });
    let payload = build_layers_payload(Default::default());
    // Act
    let document = layers(&build_layers_deps(Default::default()), params, payload)
        .expect("the layering succeeds")
        .document;
    // Assert
    assert_eq!(
        document.identities.mapping,
        vec![
            "0xcccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc".to_string(),
            "0xdddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd".to_string()
        ]
    );
    // layers_replaces_an_array_whole
}

/// Contract: Command-line order → a later flag's value wins over an earlier
///   flag's, whichever flag each is.
/// Arrange: two command lines over one key: a `--config` file naming
///   `benchmark.iterations` `3` then `--set` `benchmark.iterations=4`; and
///   `--set` `benchmark.iterations=4` then the file.
/// Act:     `layers`, once per command line.
/// Assert:  the first command line's document holds `4` and the second's
///   holds `3`.
#[test]
fn layers_applies_layers_in_command_line_order_across_both_flags() {
    // Arrange
    let directory: PathBuf = temp_dir().join(format!(
        "chaintorrent-layers-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = fs::remove_dir_all(&directory);
    fs::create_dir_all(&directory).expect("the directory is created");
    let file = directory.join("benchmark.toml");
    fs::write(&file, "[benchmark]\niterations = 3\n").expect("the file is written");
    let file_argument = file.into_os_string();
    let deps = build_layers_deps(Default::default());
    let file_first = build_layers_params(LayersParamsOverrides {
        arguments: Some(vec![
            OsString::from("harness-crypto"),
            OsString::from("--config"),
            file_argument.clone(),
            OsString::from("--set"),
            OsString::from("benchmark.iterations=4"),
        ]),
    });
    let file_last = build_layers_params(LayersParamsOverrides {
        arguments: Some(vec![
            OsString::from("harness-crypto"),
            OsString::from("--set"),
            OsString::from("benchmark.iterations=4"),
            OsString::from("--config"),
            file_argument,
        ]),
    });
    // Act
    let file_then_set = layers(&deps, file_first, build_layers_payload(Default::default()))
        .expect("the layering succeeds")
        .document;
    let set_then_file = layers(&deps, file_last, build_layers_payload(Default::default()))
        .expect("the layering succeeds")
        .document;
    // Assert
    assert_eq!(file_then_set.benchmark.iterations, 4);
    assert_eq!(set_then_file.benchmark.iterations, 3);
    // layers_applies_layers_in_command_line_order_across_both_flags
}

/// Contract: Command-line order → of two files naming one key, the
///   later-named wins.
/// Arrange: two files naming `benchmark.iterations`, `5` and `6`, named in
///   that order.
/// Act:     `layers`, once.
/// Assert:  the document's `benchmark.iterations` equals `6`.
#[test]
fn layers_applies_repeated_files_in_the_order_named() {
    // Arrange
    let directory: PathBuf = temp_dir().join(format!(
        "chaintorrent-layers-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = fs::remove_dir_all(&directory);
    fs::create_dir_all(&directory).expect("the directory is created");
    let first = directory.join("first.toml");
    fs::write(&first, "[benchmark]\niterations = 5\n").expect("the file is written");
    let second = directory.join("second.toml");
    fs::write(&second, "[benchmark]\niterations = 6\n").expect("the file is written");
    let params = build_layers_params(LayersParamsOverrides {
        arguments: Some(vec![
            OsString::from("harness-crypto"),
            OsString::from("--config"),
            first.into_os_string(),
            OsString::from("--config"),
            second.into_os_string(),
        ]),
    });
    let payload = build_layers_payload(Default::default());
    // Act
    let document = layers(&build_layers_deps(Default::default()), params, payload)
        .expect("the layering succeeds")
        .document;
    // Assert
    assert_eq!(document.benchmark.iterations, 6);
    // layers_applies_repeated_files_in_the_order_named
}

/// Contract: Assignment text → a dotted key names a key of a nested table.
/// Arrange: `--set` with `generate.kinds.kind-a.count=5`.
/// Act:     `layers`, once.
/// Assert:  the document's `generate.kinds["kind-a"].count` equals `Some(5)`
///   and its `file` equals the defaults' value.
#[test]
fn layers_applies_a_dotted_assignment_key_to_its_nested_table() {
    // Arrange
    let params = build_layers_params(LayersParamsOverrides {
        arguments: Some(vec![
            OsString::from("harness-crypto"),
            OsString::from("--set"),
            OsString::from("generate.kinds.kind-a.count=5"),
        ]),
    });
    let payload = build_layers_payload(Default::default());
    // Act
    let document = layers(&build_layers_deps(Default::default()), params, payload)
        .expect("the layering succeeds")
        .document;
    // Assert
    assert_eq!(document.generate.kinds["kind-a"].count, Some(5));
    assert_eq!(document.generate.kinds["kind-a"].file, "kind-a.txt");
    // layers_applies_a_dotted_assignment_key_to_its_nested_table
}

/// Contract: Document admitted → an optional key no layer names is `None`.
/// Arrange: the program name alone.
/// Act:     `layers`, once.
/// Assert:  the document's `report.adjustment` is `None`.
#[test]
fn layers_leaves_an_optional_table_absent_when_no_layer_names_it() {
    // Arrange
    let params = build_layers_params(Default::default());
    let payload = build_layers_payload(Default::default());
    // Act
    let document = layers(&build_layers_deps(Default::default()), params, payload)
        .expect("the layering succeeds")
        .document;
    // Assert
    assert!(document.report.adjustment.is_none());
    // layers_leaves_an_optional_table_absent_when_no_layer_names_it
}
