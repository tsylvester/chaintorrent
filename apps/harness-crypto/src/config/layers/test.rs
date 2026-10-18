#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use clap::Parser;
use clap::error::ErrorKind;
use std::ffi::OsString;
use std::path::PathBuf;
use toml::{Table, Value, to_string};

use super::interface::{
    ConfigArguments, HarnessConfigDocument, LayersErrorReturn, LayersSuccessReturn,
};
use super::layers;
use super::mock::{
    BenchmarkDocumentCorruptions, BenchmarkDocumentOverrides, DeliveryContextDocumentOverrides,
    HarnessConfigDocumentCorruptions, HarnessConfigDocumentOverrides, LayersPayloadOverrides,
    NetworkDocumentOverrides, RandomDocumentOverrides, StatementsDocumentOverrides,
    build_benchmark_document, build_delivery_context_document, build_harness_config_document,
    build_layers_deps, build_layers_params, build_layers_payload, build_network_document,
    build_random_document, build_statements_document, invalidate_benchmark_document,
    invalidate_harness_config_document,
};

/// Contract: the merged table holds every key of the configuration document →
///   `Ok(LayersSuccessReturn { document })`.
/// Arrange: `defaults` the text `toml::to_string` renders from
///   `build_harness_config_document(Default::default())`, whose values hold
///   both `CanonicalFieldDocumentValue` variants, an `Option` size field
///   absent from the rendered text, and a `PathBuf`; `arguments` the program
///   name alone.
/// Act:    `layers(&deps, params, payload)`.
/// Assert: `document` equals `build_harness_config_document(Default::default())`.
#[test]
fn layers_deserializes_the_defaults_into_the_document() {
    // Arrange
    let defaults = to_string(&build_harness_config_document(Default::default()))
        .expect("a built document renders as TOML");
    let deps = build_layers_deps(Default::default());
    let params = build_layers_params(Default::default());
    let payload = build_layers_payload(&defaults, Default::default());

    // Act
    let result = layers(&deps, params, payload);

    // Assert
    let Ok(LayersSuccessReturn { document }) = result else {
        panic!("the defaults deserialize into the document");
    };
    assert_eq!(document, build_harness_config_document(Default::default()));
}

/// Contract: each file's table is merged over the accumulated table in
///   command-line order → the later file's value stands.
/// Arrange: `first.toml` rendered from `build_harness_config_document` with
///   `benchmark` `build_benchmark_document` over `iterations` `7` and
///   `second.toml` rendered with `iterations` `8`, written under the test's
///   own directory in `std::env::temp_dir()` and named in that order by
///   `--config`.
/// Act:    `layers(&deps, params, payload)`.
/// Assert: `document` equals `build_harness_config_document` with `benchmark`
///   `build_benchmark_document` over `iterations` `8`.
#[test]
fn layers_applies_configuration_files_in_command_line_order() {
    // Arrange
    let directory = std::env::temp_dir()
        .join("chaintorrent-config-layers-")
        .join(std::process::id().to_string())
        .join("layers_applies_configuration_files_in_command_line_order");
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory).expect("the test's directory is created");
    let first = directory.join("first.toml");
    std::fs::write(
        &first,
        to_string(&build_harness_config_document(
            HarnessConfigDocumentOverrides {
                benchmark: Some(build_benchmark_document(BenchmarkDocumentOverrides {
                    iterations: Some(7),
                    ..Default::default()
                })),
                ..Default::default()
            },
        ))
        .expect("a built document renders as TOML"),
    )
    .expect("the file is written");
    let second = directory.join("second.toml");
    std::fs::write(
        &second,
        to_string(&build_harness_config_document(
            HarnessConfigDocumentOverrides {
                benchmark: Some(build_benchmark_document(BenchmarkDocumentOverrides {
                    iterations: Some(8),
                    ..Default::default()
                })),
                ..Default::default()
            },
        ))
        .expect("a built document renders as TOML"),
    )
    .expect("the file is written");
    let defaults = to_string(&build_harness_config_document(Default::default()))
        .expect("a built document renders as TOML");
    let deps = build_layers_deps(Default::default());
    let params = build_layers_params(Default::default());
    let payload = build_layers_payload(
        &defaults,
        LayersPayloadOverrides {
            arguments: Some(vec![
                OsString::from("harness-crypto"),
                OsString::from("--config"),
                OsString::from(first.as_os_str()),
                OsString::from("--config"),
                OsString::from(second.as_os_str()),
            ]),
        },
    );

    // Act
    let result = layers(&deps, params, payload);

    // Assert
    let Ok(LayersSuccessReturn { document }) = result else {
        panic!("both files merge over the defaults");
    };
    assert_eq!(
        document,
        build_harness_config_document(HarnessConfigDocumentOverrides {
            benchmark: Some(build_benchmark_document(BenchmarkDocumentOverrides {
                iterations: Some(8),
                ..Default::default()
            })),
            ..Default::default()
        })
    );
}

/// Contract: every assignment is merged after every file, in command-line
///   order, wherever `--set` stands relative to `--config` → the last
///   assignment's value stands.
/// Arrange: `--set benchmark.iterations=9` and `--set benchmark.iterations=11`
///   ahead of `--config` naming `first.toml` rendered with `benchmark`
///   `build_benchmark_document` over `iterations` `7`, written under the
///   test's own directory, so the file's `7` and the earlier assignment's `9`
///   are each overridden.
/// Act:    `layers(&deps, params, payload)`.
/// Assert: `document` equals `build_harness_config_document` with `benchmark`
///   `build_benchmark_document` over `iterations` `11`.
#[test]
fn layers_applies_assignments_over_files_in_command_line_order() {
    // Arrange
    let directory = std::env::temp_dir()
        .join("chaintorrent-config-layers-")
        .join(std::process::id().to_string())
        .join("layers_applies_assignments_over_files_in_command_line_order");
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory).expect("the test's directory is created");
    let first = directory.join("first.toml");
    std::fs::write(
        &first,
        to_string(&build_harness_config_document(
            HarnessConfigDocumentOverrides {
                benchmark: Some(build_benchmark_document(BenchmarkDocumentOverrides {
                    iterations: Some(7),
                    ..Default::default()
                })),
                ..Default::default()
            },
        ))
        .expect("a built document renders as TOML"),
    )
    .expect("the file is written");
    let defaults = to_string(&build_harness_config_document(Default::default()))
        .expect("a built document renders as TOML");
    let deps = build_layers_deps(Default::default());
    let params = build_layers_params(Default::default());
    let payload = build_layers_payload(
        &defaults,
        LayersPayloadOverrides {
            arguments: Some(vec![
                OsString::from("harness-crypto"),
                OsString::from("--set"),
                OsString::from("benchmark.iterations=9"),
                OsString::from("--set"),
                OsString::from("benchmark.iterations=11"),
                OsString::from("--config"),
                OsString::from(first.as_os_str()),
            ]),
        },
    );

    // Act
    let result = layers(&deps, params, payload);

    // Assert
    let Ok(LayersSuccessReturn { document }) = result else {
        panic!("the assignments merge over the file");
    };
    assert_eq!(
        document,
        build_harness_config_document(HarnessConfigDocumentOverrides {
            benchmark: Some(build_benchmark_document(BenchmarkDocumentOverrides {
                iterations: Some(11),
                ..Default::default()
            })),
            ..Default::default()
        })
    );
}

/// Contract: a table merges key by key into the table at its path → the keys
///   the assignment does not name keep their values.
/// Arrange: `--set statements.mint.expiry=5`.
/// Act:    `layers(&deps, params, payload)`.
/// Assert: `document` equals `build_harness_config_document` with `statements`
///   `build_statements_document` with `mint` `build_delivery_context_document`
///   with `expiry` `5`, every other mint field and the transfer context
///   keeping their defaults.
#[test]
fn layers_merges_a_nested_table_key_by_key() {
    // Arrange
    let defaults = to_string(&build_harness_config_document(Default::default()))
        .expect("a built document renders as TOML");
    let deps = build_layers_deps(Default::default());
    let params = build_layers_params(Default::default());
    let payload = build_layers_payload(
        &defaults,
        LayersPayloadOverrides {
            arguments: Some(vec![
                OsString::from("harness-crypto"),
                OsString::from("--set"),
                OsString::from("statements.mint.expiry=5"),
            ]),
        },
    );

    // Act
    let result = layers(&deps, params, payload);

    // Assert
    let Ok(LayersSuccessReturn { document }) = result else {
        panic!("the nested table merges key by key");
    };
    assert_eq!(
        document,
        build_harness_config_document(HarnessConfigDocumentOverrides {
            statements: Some(build_statements_document(StatementsDocumentOverrides {
                mint: Some(build_delivery_context_document(
                    DeliveryContextDocumentOverrides {
                        expiry: Some(5),
                        ..Default::default()
                    },
                )),
                ..Default::default()
            })),
            ..Default::default()
        })
    );
}

/// Contract: any value that is not a table, an array included, replaces the
///   value at its path whole → no element of the replaced array remains.
/// Arrange: `--set network.precompile_encodings=["eip2537"]`.
/// Act:    `layers(&deps, params, payload)`.
/// Assert: `document` equals `build_harness_config_document` with `network`
///   `build_network_document` with `precompile_encodings`
///   `vec!["eip2537".to_string()]`, the default `eip196_eip197` absent.
#[test]
fn layers_replaces_an_array_whole() {
    // Arrange
    let defaults = to_string(&build_harness_config_document(Default::default()))
        .expect("a built document renders as TOML");
    let deps = build_layers_deps(Default::default());
    let params = build_layers_params(Default::default());
    let payload = build_layers_payload(
        &defaults,
        LayersPayloadOverrides {
            arguments: Some(vec![
                OsString::from("harness-crypto"),
                OsString::from("--set"),
                OsString::from("network.precompile_encodings=[\"eip2537\"]"),
            ]),
        },
    );

    // Act
    let result = layers(&deps, params, payload);

    // Assert
    let Ok(LayersSuccessReturn { document }) = result else {
        panic!("the array replaces the default whole");
    };
    assert_eq!(
        document,
        build_harness_config_document(HarnessConfigDocumentOverrides {
            network: Some(build_network_document(NetworkDocumentOverrides {
                precompile_encodings: Some(vec!["eip2537".to_string()]),
                ..Default::default()
            })),
            ..Default::default()
        })
    );
}

/// Contract: a value missing along an assigned path is replaced by an empty
///   table → the assignment's value is inserted beneath it.
/// Arrange: `defaults` rendered as `to_string` of `Value::Table` from
///   `Table::try_from` of `build_harness_config_document(Default::default())`
///   with its `random` entry removed, and `--set random.source="seeded"`.
/// Act:    `layers(&deps, params, payload)`.
/// Assert: `document` equals `build_harness_config_document` with `random`
///   `build_random_document` over `"seeded"`.
#[test]
fn layers_creates_the_tables_along_an_assigned_path_the_defaults_lack() {
    // Arrange
    let mut table = Table::try_from(build_harness_config_document(Default::default()))
        .expect("a built document renders as a table");
    table.remove("random");
    let defaults = to_string(&Value::Table(table)).expect("a table renders as TOML");
    let deps = build_layers_deps(Default::default());
    let params = build_layers_params(Default::default());
    let payload = build_layers_payload(
        &defaults,
        LayersPayloadOverrides {
            arguments: Some(vec![
                OsString::from("harness-crypto"),
                OsString::from("--set"),
                OsString::from("random.source=\"seeded\""),
            ]),
        },
    );

    // Act
    let result = layers(&deps, params, payload);

    // Assert
    let Ok(LayersSuccessReturn { document }) = result else {
        panic!("the missing table is created along the assigned path");
    };
    assert_eq!(
        document,
        build_harness_config_document(HarnessConfigDocumentOverrides {
            random: Some(build_random_document(RandomDocumentOverrides {
                source: Some("seeded".to_string()),
            })),
            ..Default::default()
        })
    );
}

/// Contract: a value that is not a table along an assigned path is replaced
///   by an empty table → the assignment's value is inserted beneath it.
/// Arrange: `--set benchmark.iterations.minimum=1`.
/// Act:    `layers(&deps, params, payload)`.
/// Assert: `Err(LayersErrorReturn::Document(error))` with `error` equal to the
///   error `try_into::<HarnessConfigDocument>()` returns for `Table::try_from`
///   of `build_harness_config_document(Default::default())` with `minimum`
///   inserted as `Value::Integer(1)` into a table that replaces its
///   `benchmark` table's `iterations` entry.
#[test]
fn layers_replaces_a_value_that_is_not_a_table_along_an_assigned_path() {
    // Arrange
    let defaults = to_string(&build_harness_config_document(Default::default()))
        .expect("a built document renders as TOML");
    let deps = build_layers_deps(Default::default());
    let params = build_layers_params(Default::default());
    let payload = build_layers_payload(
        &defaults,
        LayersPayloadOverrides {
            arguments: Some(vec![
                OsString::from("harness-crypto"),
                OsString::from("--set"),
                OsString::from("benchmark.iterations.minimum=1"),
            ]),
        },
    );

    // Act
    let result = layers(&deps, params, payload);

    // Assert
    let Err(LayersErrorReturn::Document(error)) = result else {
        panic!("the non-table value along the path is replaced by a table");
    };
    let mut expected = Table::try_from(build_harness_config_document(Default::default()))
        .expect("a built document renders as a table");
    let Value::Table(benchmark) = expected
        .get_mut("benchmark")
        .expect("the document holds a benchmark table")
    else {
        panic!("benchmark is a table");
    };
    let mut minimum = Table::new();
    minimum.insert("minimum".to_string(), Value::Integer(1));
    benchmark.insert("iterations".to_string(), Value::Table(minimum));
    let expected_error = expected
        .try_into::<HarnessConfigDocument>()
        .expect_err("the corrupted table does not deserialize");
    assert_eq!(error, expected_error);
}

/// Contract: an insert step replaces whatever stands at its path, a table
///   included → the value stands whole.
/// Arrange: `--set benchmark=3`.
/// Act:    `layers(&deps, params, payload)`.
/// Assert: `Err(LayersErrorReturn::Document(error))` with `error` equal to the
///   error `try_into::<HarnessConfigDocument>()` returns for `Table::try_from`
///   of `build_harness_config_document(Default::default())` with its
///   `benchmark` entry inserted as `Value::Integer(3)`.
#[test]
fn layers_replaces_a_table_with_a_value_that_is_not_a_table() {
    // Arrange
    let defaults = to_string(&build_harness_config_document(Default::default()))
        .expect("a built document renders as TOML");
    let deps = build_layers_deps(Default::default());
    let params = build_layers_params(Default::default());
    let payload = build_layers_payload(
        &defaults,
        LayersPayloadOverrides {
            arguments: Some(vec![
                OsString::from("harness-crypto"),
                OsString::from("--set"),
                OsString::from("benchmark=3"),
            ]),
        },
    );

    // Act
    let result = layers(&deps, params, payload);

    // Assert
    let Err(LayersErrorReturn::Document(error)) = result else {
        panic!("the table is replaced by the value");
    };
    let mut expected = Table::try_from(build_harness_config_document(Default::default()))
        .expect("a built document renders as a table");
    expected.insert("benchmark".to_string(), Value::Integer(3));
    let expected_error = expected
        .try_into::<HarnessConfigDocument>()
        .expect_err("the corrupted table does not deserialize");
    assert_eq!(error, expected_error);
}

/// Contract: the command line holds a help request →
///   `Err(LayersErrorReturn::Arguments(error))`, the `clap` error unchanged,
///   nothing printed and no exit.
/// Arrange: `--help`.
/// Act:    `layers(&deps, params, payload)`.
/// Assert: `error.kind()` equals `ErrorKind::DisplayHelp` and the `Debug`
///   rendering of `error` equals that of the error
///   `ConfigArguments::try_parse_from` returns for `harness-crypto --help`.
#[test]
fn layers_returns_a_help_request_as_the_arguments_error() {
    // Arrange
    let defaults = to_string(&build_harness_config_document(Default::default()))
        .expect("a built document renders as TOML");
    let deps = build_layers_deps(Default::default());
    let params = build_layers_params(Default::default());
    let payload = build_layers_payload(
        &defaults,
        LayersPayloadOverrides {
            arguments: Some(vec![
                OsString::from("harness-crypto"),
                OsString::from("--help"),
            ]),
        },
    );

    // Act
    let result = layers(&deps, params, payload);

    // Assert
    let Err(LayersErrorReturn::Arguments(error)) = result else {
        panic!("the help request is the arguments error");
    };
    let expected = ConfigArguments::try_parse_from(["harness-crypto", "--help"])
        .expect_err("the oracle parse is an error");
    assert_eq!(error.kind(), ErrorKind::DisplayHelp);
    assert_eq!(format!("{error:?}"), format!("{expected:?}"));
}

/// Contract: the command line holds an argument it does not declare →
///   `Err(LayersErrorReturn::Arguments(error))`, the `clap` error unchanged.
/// Arrange: `--unknown`.
/// Act:    `layers(&deps, params, payload)`.
/// Assert: `error.kind()` equals `ErrorKind::UnknownArgument` and the `Debug`
///   rendering of `error` equals that of the error
///   `ConfigArguments::try_parse_from` returns for `harness-crypto --unknown`.
#[test]
fn layers_refuses_an_unknown_argument() {
    // Arrange
    let defaults = to_string(&build_harness_config_document(Default::default()))
        .expect("a built document renders as TOML");
    let deps = build_layers_deps(Default::default());
    let params = build_layers_params(Default::default());
    let payload = build_layers_payload(
        &defaults,
        LayersPayloadOverrides {
            arguments: Some(vec![
                OsString::from("harness-crypto"),
                OsString::from("--unknown"),
            ]),
        },
    );

    // Act
    let result = layers(&deps, params, payload);

    // Assert
    let Err(LayersErrorReturn::Arguments(error)) = result else {
        panic!("the unknown argument is the arguments error");
    };
    let expected = ConfigArguments::try_parse_from(["harness-crypto", "--unknown"])
        .expect_err("the oracle parse is an error");
    assert_eq!(error.kind(), ErrorKind::UnknownArgument);
    assert_eq!(format!("{error:?}"), format!("{expected:?}"));
}

/// Contract: a named file cannot be read → `Err(LayersErrorReturn::ReadFile {
///   path, error })`, the read error unchanged.
/// Arrange: `--config` naming `missing.toml` under the test's own directory,
///   never written.
/// Act:    `layers(&deps, params, payload)`.
/// Assert: `path` equals that path, `error.kind()` equals
///   `std::io::ErrorKind::NotFound`, and the `Debug` rendering of `error`
///   equals that of the error `std::fs::read_to_string` returns for that
///   path.
#[test]
fn layers_refuses_a_file_it_cannot_read() {
    // Arrange
    let directory = std::env::temp_dir()
        .join("chaintorrent-config-layers-")
        .join(std::process::id().to_string())
        .join("layers_refuses_a_file_it_cannot_read");
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory).expect("the test's directory is created");
    let missing: PathBuf = directory.join("missing.toml");
    let defaults = to_string(&build_harness_config_document(Default::default()))
        .expect("a built document renders as TOML");
    let deps = build_layers_deps(Default::default());
    let params = build_layers_params(Default::default());
    let payload = build_layers_payload(
        &defaults,
        LayersPayloadOverrides {
            arguments: Some(vec![
                OsString::from("harness-crypto"),
                OsString::from("--config"),
                OsString::from(missing.as_os_str()),
            ]),
        },
    );

    // Act
    let result = layers(&deps, params, payload);

    // Assert
    let Err(LayersErrorReturn::ReadFile { path, error }) = result else {
        panic!("the unreadable file is the read error");
    };
    assert_eq!(path, missing);
    assert_eq!(error.kind(), std::io::ErrorKind::NotFound);
    let expected = std::fs::read_to_string(&missing).expect_err("the file is missing");
    assert_eq!(format!("{error:?}"), format!("{expected:?}"));
}

/// Contract: the defaults text is not TOML →
///   `Err(LayersErrorReturn::ParseDefaults(error))`, the parser's error
///   unchanged.
/// Arrange: `defaults` `"benchmark ="`.
/// Act:    `layers(&deps, params, payload)`.
/// Assert: `error` equals the error `"benchmark =".parse::<Table>()` returns.
#[test]
fn layers_refuses_defaults_that_are_not_toml() {
    // Arrange
    let defaults = "benchmark =";
    let deps = build_layers_deps(Default::default());
    let params = build_layers_params(Default::default());
    let payload = build_layers_payload(defaults, Default::default());

    // Act
    let result = layers(&deps, params, payload);

    // Assert
    let Err(LayersErrorReturn::ParseDefaults(error)) = result else {
        panic!("the malformed defaults are the parse error");
    };
    let expected = "benchmark ="
        .parse::<Table>()
        .expect_err("the text is not TOML");
    assert_eq!(error, expected);
}

/// Contract: a file's text is not TOML → `Err(LayersErrorReturn::ParseFile {
///   path, error })`, the parser's error unchanged.
/// Arrange: the file `broken.toml` holding `[[` under the test's own
///   directory, named by `--config`.
/// Act:    `layers(&deps, params, payload)`.
/// Assert: `path` equals that file's path and `error` equals the error
///   `"[[".parse::<Table>()` returns.
#[test]
fn layers_refuses_a_file_that_is_not_toml() {
    // Arrange
    let directory = std::env::temp_dir()
        .join("chaintorrent-config-layers-")
        .join(std::process::id().to_string())
        .join("layers_refuses_a_file_that_is_not_toml");
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory).expect("the test's directory is created");
    let broken = directory.join("broken.toml");
    std::fs::write(&broken, "[[").expect("the file is written");
    let defaults = to_string(&build_harness_config_document(Default::default()))
        .expect("a built document renders as TOML");
    let deps = build_layers_deps(Default::default());
    let params = build_layers_params(Default::default());
    let payload = build_layers_payload(
        &defaults,
        LayersPayloadOverrides {
            arguments: Some(vec![
                OsString::from("harness-crypto"),
                OsString::from("--config"),
                OsString::from(broken.as_os_str()),
            ]),
        },
    );

    // Act
    let result = layers(&deps, params, payload);

    // Assert
    let Err(LayersErrorReturn::ParseFile { path, error }) = result else {
        panic!("the malformed file is the parse error");
    };
    assert_eq!(path, broken);
    let expected = "[[".parse::<Table>().expect_err("the text is not TOML");
    assert_eq!(error, expected);
}

/// Contract: an assignment's text is not TOML →
///   `Err(LayersErrorReturn::ParseAssignment { index, error })`, the parser's
///   error unchanged.
/// Arrange: `--set benchmark.iterations`.
/// Act:    `layers(&deps, params, payload)`.
/// Assert: `index` `0` and `error` equals the error
///   `"benchmark.iterations".parse::<Table>()` returns.
#[test]
fn layers_refuses_an_assignment_that_is_not_toml() {
    // Arrange
    let defaults = to_string(&build_harness_config_document(Default::default()))
        .expect("a built document renders as TOML");
    let deps = build_layers_deps(Default::default());
    let params = build_layers_params(Default::default());
    let payload = build_layers_payload(
        &defaults,
        LayersPayloadOverrides {
            arguments: Some(vec![
                OsString::from("harness-crypto"),
                OsString::from("--set"),
                OsString::from("benchmark.iterations"),
            ]),
        },
    );

    // Act
    let result = layers(&deps, params, payload);

    // Assert
    let Err(LayersErrorReturn::ParseAssignment { index, error }) = result else {
        panic!("the malformed assignment is the parse error");
    };
    assert_eq!(index, 0);
    let expected = "benchmark.iterations"
        .parse::<Table>()
        .expect_err("the text is not TOML");
    assert_eq!(error, expected);
}

/// Contract: an assignment's table does not hold exactly one top-level key →
///   `Err(LayersErrorReturn::AssignmentNotKeyValue { index, keys })`, `keys`
///   the number it holds.
/// Arrange: `--set benchmark.iterations=9` then `--set ""`.
/// Act:    `layers(&deps, params, payload)`.
/// Assert: `index` `1` and `keys` `0`.
#[test]
fn layers_refuses_an_assignment_holding_no_key() {
    // Arrange
    let defaults = to_string(&build_harness_config_document(Default::default()))
        .expect("a built document renders as TOML");
    let deps = build_layers_deps(Default::default());
    let params = build_layers_params(Default::default());
    let payload = build_layers_payload(
        &defaults,
        LayersPayloadOverrides {
            arguments: Some(vec![
                OsString::from("harness-crypto"),
                OsString::from("--set"),
                OsString::from("benchmark.iterations=9"),
                OsString::from("--set"),
                OsString::from(""),
            ]),
        },
    );

    // Act
    let result = layers(&deps, params, payload);

    // Assert
    let Err(LayersErrorReturn::AssignmentNotKeyValue { index, keys }) = result else {
        panic!("the empty assignment is the not-key-value error");
    };
    assert_eq!(index, 1);
    assert_eq!(keys, 0);
}

/// Contract: an assignment's table does not hold exactly one top-level key →
///   `Err(LayersErrorReturn::AssignmentNotKeyValue { index, keys })`, `keys`
///   the number it holds.
/// Arrange: the one assignment `--set` of the two lines
///   `random.source="seeded"` and `benchmark.iterations=3`.
/// Act:    `layers(&deps, params, payload)`.
/// Assert: `index` `0` and `keys` `2`.
#[test]
fn layers_refuses_an_assignment_holding_more_than_one_key() {
    // Arrange
    let defaults = to_string(&build_harness_config_document(Default::default()))
        .expect("a built document renders as TOML");
    let deps = build_layers_deps(Default::default());
    let params = build_layers_params(Default::default());
    let payload = build_layers_payload(
        &defaults,
        LayersPayloadOverrides {
            arguments: Some(vec![
                OsString::from("harness-crypto"),
                OsString::from("--set"),
                OsString::from("random.source=\"seeded\"\nbenchmark.iterations=3"),
            ]),
        },
    );

    // Act
    let result = layers(&deps, params, payload);

    // Assert
    let Err(LayersErrorReturn::AssignmentNotKeyValue { index, keys }) = result else {
        panic!("the two-key assignment is the not-key-value error");
    };
    assert_eq!(index, 0);
    assert_eq!(keys, 2);
}

/// Contract: the merged table holds a key the document does not declare →
///   `Err(LayersErrorReturn::Document(error))`, the deserializer's error
///   unchanged.
/// Arrange: `--set benchmark.repeats=3`.
/// Act:    `layers(&deps, params, payload)`.
/// Assert: `error` equals the error `try_into::<HarnessConfigDocument>()`
///   returns for `Table::try_from` of
///   `build_harness_config_document(Default::default())` with `repeats`
///   inserted as `Value::Integer(3)` into its `benchmark` table, and
///   `error.message()` equals ``unknown field `repeats`, expected
///   `iterations` or `concretes```.
#[test]
fn layers_refuses_a_document_with_an_undeclared_key() {
    // Arrange
    let defaults = to_string(&build_harness_config_document(Default::default()))
        .expect("a built document renders as TOML");
    let deps = build_layers_deps(Default::default());
    let params = build_layers_params(Default::default());
    let payload = build_layers_payload(
        &defaults,
        LayersPayloadOverrides {
            arguments: Some(vec![
                OsString::from("harness-crypto"),
                OsString::from("--set"),
                OsString::from("benchmark.repeats=3"),
            ]),
        },
    );

    // Act
    let result = layers(&deps, params, payload);

    // Assert
    let Err(LayersErrorReturn::Document(error)) = result else {
        panic!("the undeclared key is the document error");
    };
    let mut expected = Table::try_from(build_harness_config_document(Default::default()))
        .expect("a built document renders as a table");
    let Value::Table(benchmark) = expected
        .get_mut("benchmark")
        .expect("the document holds a benchmark table")
    else {
        panic!("benchmark is a table");
    };
    benchmark.insert("repeats".to_string(), Value::Integer(3));
    let expected_error = expected
        .try_into::<HarnessConfigDocument>()
        .expect_err("the corrupted table does not deserialize");
    assert_eq!(error, expected_error);
    assert_eq!(
        error.message(),
        "unknown field `repeats`, expected `iterations` or `concretes`"
    );
}

/// Contract: the merged table holds a value outside its field's type →
///   `Err(LayersErrorReturn::Document(error))`, the deserializer's error
///   unchanged.
/// Arrange: `defaults` rendered as `to_string` of `Value::Table` from the
///   table `invalidate_harness_config_document` returns with `benchmark`
///   corrupted to `Value::Table` of the table `invalidate_benchmark_document`
///   returns with `iterations` corrupted to `Value::Integer(-1)`.
/// Act:    `layers(&deps, params, payload)`.
/// Assert: `error` equals the error `try_into::<HarnessConfigDocument>()`
///   returns for that table and `error.message()` equals ``invalid value:
///   integer `-1`, expected u32``.
#[test]
fn layers_refuses_a_value_outside_its_declared_type() {
    // Arrange
    let corrupted = invalidate_harness_config_document(HarnessConfigDocumentCorruptions {
        benchmark: Some(Value::Table(invalidate_benchmark_document(
            BenchmarkDocumentCorruptions {
                iterations: Some(Value::Integer(-1)),
                ..Default::default()
            },
        ))),
        ..Default::default()
    });
    let defaults = to_string(&Value::Table(corrupted.clone())).expect("a table renders as TOML");
    let deps = build_layers_deps(Default::default());
    let params = build_layers_params(Default::default());
    let payload = build_layers_payload(&defaults, Default::default());

    // Act
    let result = layers(&deps, params, payload);

    // Assert
    let Err(LayersErrorReturn::Document(error)) = result else {
        panic!("the out-of-type value is the document error");
    };
    let expected_error = corrupted
        .try_into::<HarnessConfigDocument>()
        .expect_err("the corrupted table does not deserialize");
    assert_eq!(error, expected_error);
    assert_eq!(error.message(), "invalid value: integer `-1`, expected u32");
}

/// Contract: the merged table lacks a key the document declares →
///   `Err(LayersErrorReturn::Document(error))`, the deserializer's error
///   unchanged.
/// Arrange: `defaults` rendered as `to_string` of `Value::Table` from
///   `Table::try_from` of `build_harness_config_document(Default::default())`
///   with its `benchmark` entry removed.
/// Act:    `layers(&deps, params, payload)`.
/// Assert: `error` equals the error `try_into::<HarnessConfigDocument>()`
///   returns for that table and `error.message()` equals ``missing field
///   `benchmark```.
#[test]
fn layers_refuses_defaults_missing_a_declared_table() {
    // Arrange
    let mut table = Table::try_from(build_harness_config_document(Default::default()))
        .expect("a built document renders as a table");
    table.remove("benchmark");
    let defaults = to_string(&Value::Table(table.clone())).expect("a table renders as TOML");
    let deps = build_layers_deps(Default::default());
    let params = build_layers_params(Default::default());
    let payload = build_layers_payload(&defaults, Default::default());

    // Act
    let result = layers(&deps, params, payload);

    // Assert
    let Err(LayersErrorReturn::Document(error)) = result else {
        panic!("the missing table is the document error");
    };
    let expected_error = table
        .try_into::<HarnessConfigDocument>()
        .expect_err("the incomplete table does not deserialize");
    assert_eq!(error, expected_error);
    assert_eq!(error.message(), "missing field `benchmark`");
}

/// Contract: the arguments are refused before the defaults are parsed, the
///   first refusal returning → `Err(LayersErrorReturn::Arguments(error))`.
/// Arrange: `--unknown` and `defaults` `"benchmark ="`.
/// Act:    `layers(&deps, params, payload)`.
/// Assert: `error.kind()` equals `ErrorKind::UnknownArgument` and the `Debug`
///   rendering of `error` equals that of the error
///   `ConfigArguments::try_parse_from` returns for `harness-crypto --unknown`.
#[test]
fn layers_refuses_the_arguments_before_reading_the_defaults() {
    // Arrange
    let defaults = "benchmark =";
    let deps = build_layers_deps(Default::default());
    let params = build_layers_params(Default::default());
    let payload = build_layers_payload(
        defaults,
        LayersPayloadOverrides {
            arguments: Some(vec![
                OsString::from("harness-crypto"),
                OsString::from("--unknown"),
            ]),
        },
    );

    // Act
    let result = layers(&deps, params, payload);

    // Assert
    let Err(LayersErrorReturn::Arguments(error)) = result else {
        panic!("the arguments refusal precedes the defaults parse");
    };
    let expected = ConfigArguments::try_parse_from(["harness-crypto", "--unknown"])
        .expect_err("the oracle parse is an error");
    assert_eq!(error.kind(), ErrorKind::UnknownArgument);
    assert_eq!(format!("{error:?}"), format!("{expected:?}"));
}

/// Contract: the defaults are parsed before any file is read, the first
///   refusal returning → `Err(LayersErrorReturn::ParseDefaults(error))`.
/// Arrange: `defaults` `"benchmark ="` and `--config` naming `missing.toml`
///   under the test's own directory, never written.
/// Act:    `layers(&deps, params, payload)`.
/// Assert: `error` equals the error `"benchmark =".parse::<Table>()` returns.
#[test]
fn layers_refuses_the_defaults_before_reading_a_file() {
    // Arrange
    let directory = std::env::temp_dir()
        .join("chaintorrent-config-layers-")
        .join(std::process::id().to_string())
        .join("layers_refuses_the_defaults_before_reading_a_file");
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory).expect("the test's directory is created");
    let missing = directory.join("missing.toml");
    let defaults = "benchmark =";
    let deps = build_layers_deps(Default::default());
    let params = build_layers_params(Default::default());
    let payload = build_layers_payload(
        defaults,
        LayersPayloadOverrides {
            arguments: Some(vec![
                OsString::from("harness-crypto"),
                OsString::from("--config"),
                OsString::from(missing.as_os_str()),
            ]),
        },
    );

    // Act
    let result = layers(&deps, params, payload);

    // Assert
    let Err(LayersErrorReturn::ParseDefaults(error)) = result else {
        panic!("the defaults refusal precedes the file read");
    };
    let expected = "benchmark ="
        .parse::<Table>()
        .expect_err("the text is not TOML");
    assert_eq!(error, expected);
}

/// Contract: every file is read before any assignment is parsed, the first
///   refusal returning → `Err(LayersErrorReturn::ReadFile { path, error })`.
/// Arrange: `--config` naming `missing.toml` under the test's own directory,
///   never written, and `--set benchmark.iterations`.
/// Act:    `layers(&deps, params, payload)`.
/// Assert: `path` equals that path, `error.kind()` equals
///   `std::io::ErrorKind::NotFound`, and the `Debug` rendering of `error`
///   equals that of the error `std::fs::read_to_string` returns for that
///   path.
#[test]
fn layers_refuses_a_file_before_parsing_an_assignment() {
    // Arrange
    let directory = std::env::temp_dir()
        .join("chaintorrent-config-layers-")
        .join(std::process::id().to_string())
        .join("layers_refuses_a_file_before_parsing_an_assignment");
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory).expect("the test's directory is created");
    let missing = directory.join("missing.toml");
    let defaults = to_string(&build_harness_config_document(Default::default()))
        .expect("a built document renders as TOML");
    let deps = build_layers_deps(Default::default());
    let params = build_layers_params(Default::default());
    let payload = build_layers_payload(
        &defaults,
        LayersPayloadOverrides {
            arguments: Some(vec![
                OsString::from("harness-crypto"),
                OsString::from("--config"),
                OsString::from(missing.as_os_str()),
                OsString::from("--set"),
                OsString::from("benchmark.iterations"),
            ]),
        },
    );

    // Act
    let result = layers(&deps, params, payload);

    // Assert
    let Err(LayersErrorReturn::ReadFile { path, error }) = result else {
        panic!("the file refusal precedes the assignment parse");
    };
    assert_eq!(path, missing);
    assert_eq!(error.kind(), std::io::ErrorKind::NotFound);
    let expected = std::fs::read_to_string(&missing).expect_err("the file is missing");
    assert_eq!(format!("{error:?}"), format!("{expected:?}"));
}

/// Contract: every assignment is parsed before the document is deserialized,
///   the first refusal returning → `Err(LayersErrorReturn::ParseAssignment {
///   index, error })`.
/// Arrange: `defaults` rendered as `to_string` of `Value::Table` from
///   `Table::try_from` of `build_harness_config_document(Default::default())`
///   with its `benchmark` entry removed, and `--set benchmark.iterations`.
/// Act:    `layers(&deps, params, payload)`.
/// Assert: `index` `0` and `error` equals the error
///   `"benchmark.iterations".parse::<Table>()` returns.
#[test]
fn layers_refuses_an_assignment_before_deserializing_the_document() {
    // Arrange
    let mut table = Table::try_from(build_harness_config_document(Default::default()))
        .expect("a built document renders as a table");
    table.remove("benchmark");
    let defaults = to_string(&Value::Table(table)).expect("a table renders as TOML");
    let deps = build_layers_deps(Default::default());
    let params = build_layers_params(Default::default());
    let payload = build_layers_payload(
        &defaults,
        LayersPayloadOverrides {
            arguments: Some(vec![
                OsString::from("harness-crypto"),
                OsString::from("--set"),
                OsString::from("benchmark.iterations"),
            ]),
        },
    );

    // Act
    let result = layers(&deps, params, payload);

    // Assert
    let Err(LayersErrorReturn::ParseAssignment { index, error }) = result else {
        panic!("the assignment refusal precedes the deserialization");
    };
    assert_eq!(index, 0);
    let expected = "benchmark.iterations"
        .parse::<Table>()
        .expect_err("the text is not TOML");
    assert_eq!(error, expected);
}

/// Contract: files are read in command-line order, the first refusal
///   returning → `Err(LayersErrorReturn::ReadFile { path, error })` for the
///   first file.
/// Arrange: `--config` naming `first_missing.toml` and then
///   `second_missing.toml` under the test's own directory, neither written.
/// Act:    `layers(&deps, params, payload)`.
/// Assert: `path` equals the path of `first_missing.toml`, `error.kind()`
///   equals `std::io::ErrorKind::NotFound`, and the `Debug` rendering of
///   `error` equals that of the error `std::fs::read_to_string` returns for
///   that path.
#[test]
fn layers_refuses_the_first_unreadable_file_in_command_line_order() {
    // Arrange
    let directory = std::env::temp_dir()
        .join("chaintorrent-config-layers-")
        .join(std::process::id().to_string())
        .join("layers_refuses_the_first_unreadable_file_in_command_line_order");
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory).expect("the test's directory is created");
    let first_missing = directory.join("first_missing.toml");
    let second_missing = directory.join("second_missing.toml");
    let defaults = to_string(&build_harness_config_document(Default::default()))
        .expect("a built document renders as TOML");
    let deps = build_layers_deps(Default::default());
    let params = build_layers_params(Default::default());
    let payload = build_layers_payload(
        &defaults,
        LayersPayloadOverrides {
            arguments: Some(vec![
                OsString::from("harness-crypto"),
                OsString::from("--config"),
                OsString::from(first_missing.as_os_str()),
                OsString::from("--config"),
                OsString::from(second_missing.as_os_str()),
            ]),
        },
    );

    // Act
    let result = layers(&deps, params, payload);

    // Assert
    let Err(LayersErrorReturn::ReadFile { path, error }) = result else {
        panic!("the first unreadable file is the read error");
    };
    assert_eq!(path, first_missing);
    assert_eq!(error.kind(), std::io::ErrorKind::NotFound);
    let expected = std::fs::read_to_string(&first_missing).expect_err("the file is missing");
    assert_eq!(format!("{error:?}"), format!("{expected:?}"));
}

/// Contract: assignments are parsed in command-line order, the first refusal
///   returning → `Err(LayersErrorReturn::ParseAssignment { index, error })`
///   for the first assignment.
/// Arrange: `--set benchmark.iterations` and then `--set ""`.
/// Act:    `layers(&deps, params, payload)`.
/// Assert: `index` `0` and `error` equals the error
///   `"benchmark.iterations".parse::<Table>()` returns.
#[test]
fn layers_refuses_the_first_malformed_assignment_in_command_line_order() {
    // Arrange
    let defaults = to_string(&build_harness_config_document(Default::default()))
        .expect("a built document renders as TOML");
    let deps = build_layers_deps(Default::default());
    let params = build_layers_params(Default::default());
    let payload = build_layers_payload(
        &defaults,
        LayersPayloadOverrides {
            arguments: Some(vec![
                OsString::from("harness-crypto"),
                OsString::from("--set"),
                OsString::from("benchmark.iterations"),
                OsString::from("--set"),
                OsString::from(""),
            ]),
        },
    );

    // Act
    let result = layers(&deps, params, payload);

    // Assert
    let Err(LayersErrorReturn::ParseAssignment { index, error }) = result else {
        panic!("the first malformed assignment is the parse error");
    };
    assert_eq!(index, 0);
    let expected = "benchmark.iterations"
        .parse::<Table>()
        .expect_err("the text is not TOML");
    assert_eq!(error, expected);
}
