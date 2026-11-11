#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use std::env::temp_dir;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;

use harness_crypto::{
    BenchmarkTableOverrides, ConfigDocumentOverrides, IdentitiesTableOverrides,
    LayersParamsOverrides, LayersPayloadOverrides, MeasurementTableOverrides,
    build_benchmark_table, build_config_document, build_identities_table, build_layers_defaults,
    build_layers_deps, build_layers_params, build_layers_payload, build_measurement_table, layers,
};

/// Contract: own entry `layered_sources_merge_in_command_line_order` —
///   `layers` over a command line naming two files and two assignments
///   interleaved, every collaborator real and the filesystem real, over a
///   defaults text declaring every table, returns `Ok` whose document
///   holds, for each key a later layer named, the value of the last layer
///   that named it, and, for each key no layer named, the defaults' value.
/// Arrange: a directory beneath the operating system's temporary directory
///   removed before the act; two files and two assignments over the scalar
///   `benchmark.iterations`, the table key `measurement.conditions`, and
///   the array `identities.mapping`, named in the three interleavings file
///   then assignment, assignment then file, and two files; the defaults
///   text from `build_layers_defaults` carried by the payload's override,
///   declaring the keys the layers name; an expected document per
///   interleaving built by `build_config_document` over the value each
///   key's last layer names.
/// Act:     `layers` through the crate root, once per interleaving.
/// Assert:  each result is `Ok` and its document equals the expected
///   document built in Arrange.
/// Boundary: the public call `layers` — the real command-line grammar,
///   filesystem, and TOML merge.
/// Mocked:  none; the filesystem is a real directory.
#[test]
fn layered_sources_merge_in_command_line_order() {
    // Arrange
    let directory: PathBuf = temp_dir().join(format!(
        "chaintorrent-layers-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = fs::remove_dir_all(&directory);
    fs::create_dir_all(&directory).expect("the directory is created");
    let first = directory.join("first.toml");
    fs::write(
        &first,
        "[benchmark]\niterations = 3\n\n[identities]\nmapping = [\"0x1111\"]\n",
    )
    .expect("the first file is written");
    let second = directory.join("second.toml");
    fs::write(
        &second,
        "[measurement]\nconditions = \"conditions-b\"\n\n[identities]\nmapping = [\"0x2222\", \"0x3333\"]\n",
    )
    .expect("the second file is written");
    let first_argument = first.into_os_string();
    let second_argument = second.into_os_string();
    let scalar_assignment = OsString::from("benchmark.iterations=4");
    let array_assignment = OsString::from(r#"identities.mapping=["0x4444"]"#);
    let file_then_assignment = build_layers_params(LayersParamsOverrides {
        arguments: Some(vec![
            OsString::from("harness-crypto"),
            OsString::from("--config"),
            first_argument.clone(),
            OsString::from("--set"),
            scalar_assignment.clone(),
            OsString::from("--config"),
            second_argument.clone(),
            OsString::from("--set"),
            array_assignment.clone(),
        ]),
    });
    let assignment_then_file = build_layers_params(LayersParamsOverrides {
        arguments: Some(vec![
            OsString::from("harness-crypto"),
            OsString::from("--set"),
            scalar_assignment.clone(),
            OsString::from("--config"),
            first_argument.clone(),
            OsString::from("--set"),
            array_assignment.clone(),
            OsString::from("--config"),
            second_argument.clone(),
        ]),
    });
    let two_files = build_layers_params(LayersParamsOverrides {
        arguments: Some(vec![
            OsString::from("harness-crypto"),
            OsString::from("--config"),
            first_argument,
            OsString::from("--config"),
            second_argument,
            OsString::from("--set"),
            scalar_assignment,
            OsString::from("--set"),
            array_assignment,
        ]),
    });
    let payload = build_layers_payload(LayersPayloadOverrides {
        defaults: Some(build_layers_defaults(Default::default())),
    });
    let expected_assignment_last = build_config_document(ConfigDocumentOverrides {
        benchmark: Some(build_benchmark_table(BenchmarkTableOverrides {
            iterations: Some(4),
        })),
        measurement: Some(build_measurement_table(MeasurementTableOverrides {
            conditions: Some("conditions-b".to_string()),
            ..Default::default()
        })),
        identities: Some(build_identities_table(IdentitiesTableOverrides {
            mapping: Some(vec!["0x4444".to_string()]),
            ..Default::default()
        })),
        ..Default::default()
    });
    let expected_file_last = build_config_document(ConfigDocumentOverrides {
        benchmark: Some(build_benchmark_table(BenchmarkTableOverrides {
            iterations: Some(3),
        })),
        measurement: Some(build_measurement_table(MeasurementTableOverrides {
            conditions: Some("conditions-b".to_string()),
            ..Default::default()
        })),
        identities: Some(build_identities_table(IdentitiesTableOverrides {
            mapping: Some(vec!["0x2222".to_string(), "0x3333".to_string()]),
            ..Default::default()
        })),
        ..Default::default()
    });
    let deps = build_layers_deps(Default::default());
    // Act
    let file_then_assignment_result = layers(&deps, file_then_assignment, payload.clone());
    let assignment_then_file_result = layers(&deps, assignment_then_file, payload.clone());
    let two_files_result = layers(&deps, two_files, payload);
    // Assert
    assert_eq!(
        file_then_assignment_result
            .expect("the layering succeeds")
            .document,
        expected_assignment_last
    );
    assert_eq!(
        assignment_then_file_result
            .expect("the layering succeeds")
            .document,
        expected_file_last
    );
    assert_eq!(
        two_files_result.expect("the layering succeeds").document,
        expected_assignment_last
    );
    // layered_sources_merge_in_command_line_order
}
