#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

// Public-surface enumeration:
//   Own entries —
//     a_configuration_refusal_is_written_to_standard_error_and_the_process_fails
//     a_refused_run_is_written_as_its_refusal_and_the_process_fails
//     a_refused_destination_write_is_written_to_standard_error_and_the_process_fails
//     a_successful_run_exits_with_success_after_every_destination_is_written
//   Carried, `harness-crypto/config` —
//     an_unadmissible_value_is_refused_by_the_function_that_admits_it
//     a_value_this_function_admits_is_refused_with_its_own_variant
//     the_first_refusal_is_returned_in_the_stated_order
//     a_file_and_a_flag_each_replace_a_shipped_default
//   Carried, `harness-crypto/run` —
//     the_shipped_defaults_reach_no_deployed_verifier
//     a_deployment_record_layer_and_a_flag_each_replace_a_shipped_default
//   Carried, `harness-crypto/output` —
//     a_refused_runs_outcome_is_written_as_its_refusal
//     a_successful_runs_outcome_is_written_with_every_value_it_holds
//   Carried, `harness-crypto/config/admit_output` —
//     every_declared_destination_kind_is_admitted_to_its_own_variant
//     a_destination_list_keeps_the_order_the_document_holds

use harness_crypto::{DESTINATION_KINDS, DestinationKind};
use pairing::{PAIRING_CONCRETES, PairingConcrete};
use serde_json::Value;
use std::env::temp_dir;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// A directory beneath the operating system's temporary directory named
/// `chaintorrent-main-`, the process id, `-`, and the current thread's
/// id as its `Debug` text, removed before the act and created fresh.
fn work_directory() -> PathBuf {
    let directory = temp_dir().join(format!(
        "chaintorrent-main-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory).expect("the work directory is created");
    directory
}

/// `path` rendered as a TOML literal string.
fn toml_path(path: &Path) -> String {
    format!("'{}'", path.display())
}

/// The `output.destinations` list of `entries` written as the shipped
/// defaults write it — `StandardOutput` ignoring its path, `File` at
/// its path.
fn toml_destinations(entries: &[(DestinationKind, PathBuf)]) -> String {
    let rendered: Vec<String> = entries
        .iter()
        .map(|(kind, path)| match kind {
            DestinationKind::StandardOutput => "{ kind = \"StandardOutput\" }".to_string(),
            DestinationKind::File => format!("{{ kind = \"File\", path = {} }}", toml_path(path)),
        })
        .collect();
    format!("[{}]", rendered.join(", "))
}

/// The `--set` assignments every block that runs the harness makes: the
/// arranged directory as the generate root and the report's file
/// destination, the lowest admitted value of each iteration count, and
/// the `output.destinations` list `destinations` renders.
fn harness_arguments(directory: &Path, destinations: String) -> Vec<String> {
    vec![
        "--set".to_string(),
        format!("generate.foundry_root={}", toml_path(directory)),
        "--set".to_string(),
        "benchmark.iterations=1".to_string(),
        "--set".to_string(),
        "measurement.iterations=1".to_string(),
        "--set".to_string(),
        format!(
            "report.destinations=[{{ kind = \"File\", path = {} }}]",
            toml_path(&directory.join("report.json"))
        ),
        "--set".to_string(),
        format!("output.destinations={destinations}"),
    ]
}

/// The binary run once with `arguments` in the package directory.
fn run_harness(arguments: &[String]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_harness-crypto"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args(arguments)
        .output()
        .expect("the process runs")
}

/// The captured standard output of `output` as text.
fn stdout_text(output: &Output) -> String {
    String::from_utf8(output.stdout.clone()).expect("standard output is text")
}

/// The captured standard error of `output` as text.
fn stderr_text(output: &Output) -> String {
    String::from_utf8(output.stderr.clone()).expect("standard error is text")
}

/// The content of the file destination `path` as text.
fn file_text(path: &Path) -> String {
    std::fs::read_to_string(path).expect("the destination file reads")
}

/// Asserts `text` is the refused document of the run — `outcome`
/// `"refused"` and `refusal` a string beginning `PairingConsumer(`.
fn assert_refused(text: &str, name: &str) {
    let document: Value = serde_json::from_str(text).expect("the destination text parses");
    assert_eq!(document["outcome"], "refused", "{name}");
    assert!(
        document["refusal"]
            .as_str()
            .expect("the refusal is a string")
            .starts_with("PairingConsumer("),
        "{name}"
    );
}

/// Asserts `text` is the success document of the run — `outcome`
/// `"succeeded"`, the `benchmarks` one member per shipped benchmark
/// concrete in the order `PAIRING_CONCRETES` declares, and the `curves`
/// one member per curve in curve order.
fn assert_succeeded(text: &str, name: &str) {
    let concretes: Vec<PairingConcrete> = PAIRING_CONCRETES.to_vec();
    let document: Value = serde_json::from_str(text).expect("the destination text parses");
    assert_eq!(document["outcome"], "succeeded", "{name}");
    let benchmarks = document["benchmarks"]
        .as_array()
        .expect("the benchmarks are a list");
    assert_eq!(benchmarks.len(), concretes.len(), "{name}");
    for (benchmark, concrete) in benchmarks.iter().zip(&concretes) {
        assert_eq!(
            benchmark["concrete"]
                .as_str()
                .expect("the concrete is text"),
            format!("{concrete:?}"),
            "{name}"
        );
    }
    let labels: Vec<&str> = document["curves"]
        .as_array()
        .expect("the curves are a list")
        .iter()
        .map(|curve| curve["label"].as_str().expect("the label is text"))
        .collect();
    assert_eq!(labels, ["bn254", "bls12_381"], "{name}");
}

/// Contract: entry `a_configuration_refusal_is_written_to_standard_error
///   _and_the_process_fails`; a refused command line fails the process,
///   writes the refusal to standard error, and calls nothing after it.
/// Arrange: the arguments `--no-such-flag`, and
///   `--set benchmark.iterations=0` beside the destination assignments,
///   the second differing from the first in the variant it refuses with;
///   the arranged directory holding no file.
/// Act:     the binary, once per variation.
/// Assert:  `!status.success()`; for the first variation standard error
///   begins `Layers(`, and for the second its text trimmed of the
///   trailing line feed equals `BenchmarkIterations`; standard output is
///   empty; no file exists beneath the arranged directory.
/// Boundary: the process call; the real chain is `layers` for the first
///   variation, `layers` and `config` for the second, then `main`.
/// Mocked:   none.
#[test]
fn a_configuration_refusal_is_written_to_standard_error_and_the_process_fails() {
    for (name, refusing, expected, whole) in [
        (
            "an unknown flag",
            vec!["--no-such-flag".to_string()],
            "Layers(",
            false,
        ),
        (
            "a zero count",
            vec!["--set".to_string(), "benchmark.iterations=0".to_string()],
            "BenchmarkIterations",
            true,
        ),
    ] {
        // Arrange
        let directory = work_directory();
        let mut arguments = harness_arguments(
            &directory,
            toml_destinations(&[(DestinationKind::File, directory.join("outcome.json"))]),
        );
        arguments.extend(refusing);

        // Act
        let output = run_harness(&arguments);

        // Assert
        assert!(!output.status.success(), "{name}");
        let stderr = stderr_text(&output);
        if whole {
            assert_eq!(stderr.trim_end(), expected, "{name}");
        } else {
            assert!(stderr.starts_with(expected), "{name}: {stderr}");
        }
        assert!(stdout_text(&output).is_empty(), "{name}");
        assert_eq!(
            std::fs::read_dir(&directory)
                .expect("the directory reads")
                .count(),
            0,
            "{name}"
        );
        // a_configuration_refusal_is_written_to_standard_error_and_the_process_fails
    }
}

/// Contract: entry `a_refused_run_is_written_as_its_refusal_and_the_
///   process_fails`; a refused run is written to every configured
///   destination as its refusal and fails the process.
/// Arrange: the shipped defaults with the standard assignments and
///   `output.destinations` as standard output alone, a file alone, and
///   both, against the success entry, so a binary that writes only a
///   success fails.
/// Act:     the binary, once per variation.
/// Assert:  `!status.success()`; the text of each configured destination
///   parses to a document whose `outcome` equals `"refused"` and whose
///   `refusal` is a string beginning `PairingConsumer(`; where both
///   destinations are configured the two texts are equal; standard error
///   is empty.
/// Boundary: the process call; the real chain is `layers`, the admission
///   functions, `config`, `run`, the benchmark and the pairing consumer
///   beneath it, `render_outcome`, `output`, then `main`.
/// Mocked:   none; the filesystem is a real directory and the network is
///   the verifier family's endpoint.
#[test]
fn a_refused_run_is_written_as_its_refusal_and_the_process_fails() {
    for (name, kinds) in [
        (
            "standard output alone",
            vec![DestinationKind::StandardOutput],
        ),
        ("a file alone", vec![DestinationKind::File]),
        (
            "standard output and a file",
            vec![DestinationKind::StandardOutput, DestinationKind::File],
        ),
    ] {
        // Arrange
        let directory = work_directory();
        let outcome_file = directory.join("outcome.json");
        let entries: Vec<(DestinationKind, PathBuf)> = kinds
            .iter()
            .map(|kind| (*kind, outcome_file.clone()))
            .collect();
        let arguments = harness_arguments(&directory, toml_destinations(&entries));

        // Act
        let output = run_harness(&arguments);

        // Assert
        assert!(!output.status.success(), "{name}");
        let mut texts = Vec::new();
        if kinds.contains(&DestinationKind::StandardOutput) {
            texts.push(stdout_text(&output));
        } else {
            assert!(stdout_text(&output).is_empty(), "{name}");
        }
        if kinds.contains(&DestinationKind::File) {
            texts.push(file_text(&outcome_file));
        } else {
            assert!(!outcome_file.exists(), "{name}");
        }
        for text in &texts {
            assert_refused(text, name);
        }
        if texts.len() > 1 {
            assert_eq!(texts[0], texts[1], "{name}");
        }
        assert!(stderr_text(&output).is_empty(), "{name}");
        // a_refused_run_is_written_as_its_refusal_and_the_process_fails
    }
}

/// Contract: entry `a_refused_destination_write_is_written_to_standard_
///   error_and_the_process_fails`; a refused write fails the process, is
///   written to standard error, and stops the writes after it.
/// Arrange: `output.destinations` holding a file destination beneath a
///   directory that does not exist and a second file destination beneath
///   the arranged directory, in that order and in the reverse order.
/// Act:     the binary, once per variation.
/// Assert:  `!status.success()`; standard error begins
///   `WriteFile { path: `; in the first order the second file does not
///   exist, and in the reverse order the second file exists and holds a
///   document whose `outcome` equals `"refused"`; standard output is
///   empty.
/// Boundary: the process call; the real chain is as the refused-run
///   entry states, ending at `output` and `main`.
/// Mocked:   none.
#[test]
fn a_refused_destination_write_is_written_to_standard_error_and_the_process_fails() {
    for (name, refused_first) in [
        ("the refused destination first", true),
        ("the refused destination last", false),
    ] {
        // Arrange
        let directory = work_directory();
        let refused = directory.join("missing").join("outcome.json");
        let written = directory.join("outcome.json");
        let entries = if refused_first {
            vec![
                (DestinationKind::File, refused.clone()),
                (DestinationKind::File, written.clone()),
            ]
        } else {
            vec![
                (DestinationKind::File, written.clone()),
                (DestinationKind::File, refused.clone()),
            ]
        };
        let arguments = harness_arguments(&directory, toml_destinations(&entries));

        // Act
        let output = run_harness(&arguments);

        // Assert
        assert!(!output.status.success(), "{name}");
        assert!(
            stderr_text(&output).starts_with("WriteFile { path: "),
            "{name}"
        );
        if refused_first {
            assert!(!written.exists(), "{name}");
        } else {
            assert_refused(&file_text(&written), name);
        }
        assert!(stdout_text(&output).is_empty(), "{name}");
        // a_refused_destination_write_is_written_to_standard_error_and_the_process_fails
    }
}

/// Contract: entry `a_successful_run_exits_with_success_after_every_
///   destination_is_written`; an accepted run writes its success
///   document to every configured destination and the process succeeds.
/// Arrange: the arguments `--config ../../contracts/evm/deployments/
///   base-sepolia.toml` before the standard assignments,
///   `output.destinations` holding standard output and a file, and the
///   layer with standard output alone and with the file alone.
/// Act:     the binary, once per variation.
/// Assert:  `status.success()`; standard output and the file each parse
///   to a document whose `outcome` equals `"succeeded"`; the two texts
///   are equal; the `benchmarks` members' `concrete` texts equal the
///   `Debug` texts of `PAIRING_CONCRETES` in the order the shipped
///   defaults name them; the `curves` members' `label` texts equal
///   `"bn254"` then `"bls12_381"`; standard error is empty.
/// Boundary: the process call; the real chain is as the refused-run
///   entry states, with the deployed verifiers.
/// Mocked:   none; runtime inputs the deployed verifiers and the layer
///   file.
#[test]
fn a_successful_run_exits_with_success_after_every_destination_is_written() {
    for (name, kinds) in [
        (
            "standard output and a file",
            vec![DestinationKind::StandardOutput, DestinationKind::File],
        ),
        (
            "standard output alone",
            vec![DestinationKind::StandardOutput],
        ),
        ("a file alone", vec![DestinationKind::File]),
    ] {
        // Arrange
        let directory = work_directory();
        let outcome_file = directory.join("outcome.json");
        let entries: Vec<(DestinationKind, PathBuf)> = kinds
            .iter()
            .map(|kind| (*kind, outcome_file.clone()))
            .collect();
        let mut arguments = vec![
            "--config".to_string(),
            "../../contracts/evm/deployments/base-sepolia.toml".to_string(),
        ];
        arguments.extend(harness_arguments(&directory, toml_destinations(&entries)));

        // Act
        let output = run_harness(&arguments);

        // Assert
        assert!(output.status.success(), "{name}");
        let mut texts = Vec::new();
        if kinds.contains(&DestinationKind::StandardOutput) {
            texts.push(stdout_text(&output));
        }
        if kinds.contains(&DestinationKind::File) {
            texts.push(file_text(&outcome_file));
        }
        for text in &texts {
            assert_succeeded(text, name);
        }
        if texts.len() > 1 {
            assert_eq!(texts[0], texts[1], "{name}");
        }
        assert!(stderr_text(&output).is_empty(), "{name}");
        // a_successful_run_exits_with_success_after_every_destination_is_written
    }
}

/// Contract, carried: entry `an_unadmissible_value_is_refused_by_the_
///   function_that_admits_it` of `harness-crypto/config`; a refusal
///   reaches standard error whole inside the variant of the function
///   that returned it, and the process fails.
/// Arrange: the arguments of an unknown flag, the `Debug` text of a
///   family's mock selection in `selections.kem`, an unknown kind in
///   `network_profile.chain_identifier`, an unknown `generate.concrete`,
///   and an empty `output.destinations`, each beside the standard
///   assignments.
/// Act:     the binary, once per variation.
/// Assert:  `!status.success()`; standard error begins `Layers(`,
///   `Selections(`, `Statements(`, `Generate(`, and `Output(` in turn;
///   standard output is empty.
/// Boundary: the process call; the real chain is `layers` or the
///   admission function that refuses, then `config`, then `main`.
/// Mocked:   none; the filesystem is a real directory.
#[test]
fn an_unadmissible_value_is_refused_by_the_function_that_admits_it() {
    for (name, refusing, expected) in [
        (
            "an unknown flag",
            vec!["--no-such-flag".to_string()],
            "Layers(",
        ),
        (
            "a mock selection's name",
            vec!["--set".to_string(), r#"selections.kem="Mock""#.to_string()],
            "Selections(",
        ),
        (
            "an unknown canonical kind",
            vec![
                "--set".to_string(),
                r#"network_profile.chain_identifier.kind="Unknown""#.to_string(),
            ],
            "Statements(",
        ),
        (
            "an unknown generate concrete",
            vec![
                "--set".to_string(),
                r#"generate.concrete="unknown""#.to_string(),
            ],
            "Generate(",
        ),
        (
            "an empty destination list",
            vec!["--set".to_string(), "output.destinations=[]".to_string()],
            "Output(",
        ),
    ] {
        // Arrange
        let directory = work_directory();
        let mut arguments = harness_arguments(
            &directory,
            toml_destinations(&[(DestinationKind::File, directory.join("outcome.json"))]),
        );
        arguments.extend(refusing);

        // Act
        let output = run_harness(&arguments);

        // Assert
        assert!(!output.status.success(), "{name}");
        let stderr = stderr_text(&output);
        assert!(stderr.starts_with(expected), "{name}: {stderr}");
        assert!(stdout_text(&output).is_empty(), "{name}");
        // an_unadmissible_value_is_refused_by_the_function_that_admits_it
    }
}

/// Contract, carried: entry `a_value_this_function_admits_is_refused_
///   with_its_own_variant` of `harness-crypto/config`; a refusal reaches
///   standard error whole inside the variant that names the document
///   path, and the process fails.
/// Arrange: `--set benchmark.iterations=0` and
///   `--set measurement.iterations=0`, each beside the standard
///   assignments.
/// Act:     the binary, once per variation.
/// Assert:  `!status.success()`; standard error trimmed of the trailing
///   line feed equals `BenchmarkIterations` and `MeasurementIterations`
///   in turn; standard output is empty.
/// Boundary: the process call; the real chain is `layers`, `config`,
///   then `main`.
/// Mocked:   none; the filesystem is a real directory.
#[test]
fn a_value_this_function_admits_is_refused_with_its_own_variant() {
    for (name, refusing, expected) in [
        (
            "the benchmark count",
            "benchmark.iterations=0",
            "BenchmarkIterations",
        ),
        (
            "the measurement count",
            "measurement.iterations=0",
            "MeasurementIterations",
        ),
    ] {
        // Arrange
        let directory = work_directory();
        let mut arguments = harness_arguments(
            &directory,
            toml_destinations(&[(DestinationKind::File, directory.join("outcome.json"))]),
        );
        arguments.extend(["--set".to_string(), refusing.to_string()]);

        // Act
        let output = run_harness(&arguments);

        // Assert
        assert!(!output.status.success(), "{name}");
        assert_eq!(stderr_text(&output).trim_end(), expected, "{name}");
        assert!(stdout_text(&output).is_empty(), "{name}");
        // a_value_this_function_admits_is_refused_with_its_own_variant
    }
}

/// Contract, carried: entry `the_first_refusal_is_returned_in_the_
///   stated_order` of `harness-crypto/config`; the earliest of two
///   refused values reaches standard error first, and the process fails.
/// Arrange: `benchmark.iterations=0` together with a `Selections` value,
///   and an unknown flag together with a `BenchmarkIterations` value.
/// Act:     the binary, once per variation.
/// Assert:  `!status.success()`; standard error begins `Selections(` for
///   the first variation and `Layers(` for the second; standard output
///   is empty.
/// Boundary: the process call; the real chain is `layers` or the
///   admission function that refuses, then `config`, then `main`.
/// Mocked:   none; the filesystem is a real directory.
#[test]
fn the_first_refusal_is_returned_in_the_stated_order() {
    for (name, refusing, expected) in [
        (
            "a selections value before a count",
            vec![
                "--set".to_string(),
                r#"selections.kem="Mock""#.to_string(),
                "--set".to_string(),
                "benchmark.iterations=0".to_string(),
            ],
            "Selections(",
        ),
        (
            "an unknown flag before a count",
            vec![
                "--set".to_string(),
                "benchmark.iterations=0".to_string(),
                "--no-such-flag".to_string(),
            ],
            "Layers(",
        ),
    ] {
        // Arrange
        let directory = work_directory();
        let mut arguments = harness_arguments(
            &directory,
            toml_destinations(&[(DestinationKind::File, directory.join("outcome.json"))]),
        );
        arguments.extend(refusing);

        // Act
        let output = run_harness(&arguments);

        // Assert
        assert!(!output.status.success(), "{name}");
        let stderr = stderr_text(&output);
        assert!(stderr.starts_with(expected), "{name}: {stderr}");
        assert!(stdout_text(&output).is_empty(), "{name}");
        // the_first_refusal_is_returned_in_the_stated_order
    }
}

/// Contract, carried: entry `a_file_and_a_flag_each_replace_a_shipped_
///   default` of `harness-crypto/config`; the destinations the last
///   layer that named `output.destinations` holds are the destinations
///   written, observed by which of standard output and a file holds the
///   outcome.
/// Arrange: a `--config` file written beneath the arranged directory
///   assigning `output.destinations` to a file, and a `--set` assignment
///   assigning it to standard output, in each order.
/// Act:     the binary, once per order.
/// Assert:  `!status.success()`; the destination the later layer names
///   holds the refused document and the other holds none; standard error
///   is empty.
/// Boundary: the process call; the real chain is `layers`, the admission
///   functions, `config`, `run`, `output`, then `main`.
/// Mocked:   none; the filesystem is a real directory and the verifier
///   family's endpoint answers the refusal.
#[test]
fn a_file_and_a_flag_each_replace_a_shipped_default() {
    for (name, flag_first) in [
        ("the flag before the file", true),
        ("the file before the flag", false),
    ] {
        // Arrange
        let directory = work_directory();
        let outcome_file = directory.join("outcome.json");
        let layer = directory.join("layer.toml");
        std::fs::write(
            &layer,
            format!(
                "[output]\ndestinations = [{{ kind = \"File\", path = {} }}]\n",
                toml_path(&outcome_file)
            ),
        )
        .expect("the layer file is written");
        let flag = vec![
            "--set".to_string(),
            "output.destinations=[{ kind = \"StandardOutput\" }]".to_string(),
        ];
        let file = vec!["--config".to_string(), layer.display().to_string()];
        let mut arguments = harness_arguments(
            &directory,
            toml_destinations(&[(DestinationKind::File, directory.join("shadowed.json"))]),
        );
        if flag_first {
            arguments.extend(flag);
            arguments.extend(file);
        } else {
            arguments.extend(file);
            arguments.extend(flag);
        }

        // Act
        let output = run_harness(&arguments);

        // Assert
        assert!(!output.status.success(), "{name}");
        if flag_first {
            assert_refused(&file_text(&outcome_file), name);
            assert!(stdout_text(&output).is_empty(), "{name}");
        } else {
            assert_refused(&stdout_text(&output), name);
            assert!(!outcome_file.exists(), "{name}");
        }
        assert!(stderr_text(&output).is_empty(), "{name}");
        // a_file_and_a_flag_each_replace_a_shipped_default
    }
}

/// Contract, carried: entry `the_shipped_defaults_reach_no_deployed_
///   verifier` of `harness-crypto/run`; a refused run the process
///   reports as a failure status and writes as its refusal, the vector
///   files its generate step wrote existing beneath the arranged
///   directory.
/// Arrange: the shipped defaults with the standard assignments.
/// Act:     the binary, once.
/// Assert:  `!status.success()`; the destination file holds the refused
///   document; the vector files the generate step wrote exist beneath
///   the arranged directory; standard error is empty.
/// Boundary: the process call; the real chain is `config`, `run`,
///   `output`, then `main`.
/// Mocked:   none; the filesystem is a real directory and the verifier
///   family's endpoint answers the refusal.
#[test]
fn the_shipped_defaults_reach_no_deployed_verifier() {
    // Arrange
    let directory = work_directory();
    let outcome_file = directory.join("outcome.json");
    let arguments = harness_arguments(
        &directory,
        toml_destinations(&[(DestinationKind::File, outcome_file.clone())]),
    );

    // Act
    let output = run_harness(&arguments);

    // Assert
    assert!(!output.status.success());
    assert_refused(&file_text(&outcome_file), "the refused document");
    assert!(
        std::fs::read_dir(directory.join("generated"))
            .map(|entries| entries.count() > 0)
            .unwrap_or(false),
        "the generate step wrote vector files beneath the foundry root"
    );
    assert!(stderr_text(&output).is_empty());
    // the_shipped_defaults_reach_no_deployed_verifier
}

/// Contract, carried: entry `a_deployment_record_layer_and_a_flag_each_
///   replace_a_shipped_default` of `harness-crypto/run`; an accepted run
///   the process reports as a success status, the iteration counts the
///   flags name reaching the document.
/// Arrange: the arguments `--config ../../contracts/evm/deployments/
///   base-sepolia.toml` before the standard assignments, the layer
///   alone, the layer with `--set benchmark.iterations=2`, and the
///   layer with `--set measurement.iterations=2`.
/// Act:     the binary, once per variation.
/// Assert:  `status.success()`; the destination file holds a document
///   whose `benchmarks` members carry the benchmark count the flag names
///   and whose `curves` members' measurements carry the measurement
///   count the flag names; standard error is empty.
/// Boundary: the process call; the real chain is `config`, `run`,
///   `output`, then `main`, with the deployed verifiers.
/// Mocked:   none; runtime inputs the deployed verifiers and the layer
///   file.
#[test]
fn a_deployment_record_layer_and_a_flag_each_replace_a_shipped_default() {
    for (name, extra, benchmark_iterations, measurement_iterations) in [
        ("the layer alone", Vec::new(), 1_u64, 1_u64),
        (
            "the layer with the benchmark flag",
            vec!["--set".to_string(), "benchmark.iterations=2".to_string()],
            2_u64,
            1_u64,
        ),
        (
            "the layer with the measurement flag",
            vec!["--set".to_string(), "measurement.iterations=2".to_string()],
            1_u64,
            2_u64,
        ),
    ] {
        // Arrange
        let directory = work_directory();
        let outcome_file = directory.join("outcome.json");
        let mut arguments = vec![
            "--config".to_string(),
            "../../contracts/evm/deployments/base-sepolia.toml".to_string(),
        ];
        arguments.extend(harness_arguments(
            &directory,
            toml_destinations(&[(DestinationKind::File, outcome_file.clone())]),
        ));
        arguments.extend(extra);

        // Act
        let output = run_harness(&arguments);

        // Assert
        assert!(output.status.success(), "{name}");
        let document: Value =
            serde_json::from_str(&file_text(&outcome_file)).expect("the document parses");
        assert_eq!(document["outcome"], "succeeded", "{name}");
        for benchmark in document["benchmarks"]
            .as_array()
            .expect("the benchmarks are a list")
        {
            assert_eq!(
                benchmark["iterations"].as_u64(),
                Some(benchmark_iterations),
                "{name}"
            );
        }
        for curve in document["curves"]
            .as_array()
            .expect("the curves are a list")
        {
            assert_eq!(
                curve["measurement"]["iterations"].as_u64(),
                Some(measurement_iterations),
                "{name}"
            );
        }
        assert!(stderr_text(&output).is_empty(), "{name}");
        // a_deployment_record_layer_and_a_flag_each_replace_a_shipped_default
    }
}

/// Contract, carried: entry `a_refused_runs_outcome_is_written_as_its_
///   refusal` of `harness-crypto/output`; the document reaches every
///   configured destination as the text of the run's refusal.
/// Arrange: the shipped defaults with the standard assignments and one
///   file destination and two.
/// Act:     the binary, once per variation.
/// Assert:  `!status.success()`; each file holds the refused document,
///   the texts equal; standard error is empty.
/// Boundary: the process call; the real chain is `run`, `render_outcome`,
///   `output`, then `main`.
/// Mocked:   none; the filesystem is a real directory and the verifier
///   family's endpoint answers the refusal.
#[test]
fn a_refused_runs_outcome_is_written_as_its_refusal() {
    for (name, count) in [("one destination", 1), ("two destinations", 2)] {
        // Arrange
        let directory = work_directory();
        let files: Vec<PathBuf> = (0..count)
            .map(|index| directory.join(format!("outcome-{index}.json")))
            .collect();
        let entries: Vec<(DestinationKind, PathBuf)> = files
            .iter()
            .map(|file| (DestinationKind::File, file.clone()))
            .collect();
        let arguments = harness_arguments(&directory, toml_destinations(&entries));

        // Act
        let output = run_harness(&arguments);

        // Assert
        assert!(!output.status.success(), "{name}");
        let texts: Vec<String> = files.iter().map(|file| file_text(file)).collect();
        for text in &texts {
            assert_refused(text, name);
        }
        for pair in texts.windows(2) {
            assert_eq!(pair[0], pair[1], "{name}");
        }
        assert!(stderr_text(&output).is_empty(), "{name}");
        // a_refused_runs_outcome_is_written_as_its_refusal
    }
}

/// Contract, carried: entry `a_successful_runs_outcome_is_written_with_
///   every_value_it_holds` of `harness-crypto/output`; the success
///   document reaches every configured destination, holding a
///   `benchmarks` member for each configured concrete in configuration
///   order and a `curves` member for each curve in curve order.
/// Arrange: the `--config` layer and the standard assignments, over one
///   file destination and two.
/// Act:     the binary, once per variation.
/// Assert:  `status.success()`; each file holds the success document,
///   the `benchmarks` one member per `PAIRING_CONCRETES` member in the
///   shipped order and the `curves` `bn254` then `bls12_381`, the texts
///   equal; standard error is empty.
/// Boundary: the process call; the real chain is `run`, `render_outcome`,
///   `output`, then `main`, with the deployed verifiers.
/// Mocked:   none; runtime inputs the deployed verifiers and the layer
///   file.
#[test]
fn a_successful_runs_outcome_is_written_with_every_value_it_holds() {
    for (name, count) in [("one destination", 1), ("two destinations", 2)] {
        // Arrange
        let directory = work_directory();
        let files: Vec<PathBuf> = (0..count)
            .map(|index| directory.join(format!("outcome-{index}.json")))
            .collect();
        let entries: Vec<(DestinationKind, PathBuf)> = files
            .iter()
            .map(|file| (DestinationKind::File, file.clone()))
            .collect();
        let mut arguments = vec![
            "--config".to_string(),
            "../../contracts/evm/deployments/base-sepolia.toml".to_string(),
        ];
        arguments.extend(harness_arguments(&directory, toml_destinations(&entries)));

        // Act
        let output = run_harness(&arguments);

        // Assert
        assert!(output.status.success(), "{name}");
        let texts: Vec<String> = files.iter().map(|file| file_text(file)).collect();
        for text in &texts {
            assert_succeeded(text, name);
        }
        for pair in texts.windows(2) {
            assert_eq!(pair[0], pair[1], "{name}");
        }
        assert!(stderr_text(&output).is_empty(), "{name}");
        // a_successful_runs_outcome_is_written_with_every_value_it_holds
    }
}

/// Contract, carried: entry `every_declared_destination_kind_is_
///   admitted_to_its_own_variant` of `harness-crypto/config/admit_output`;
///   a standard output destination and a file destination each receive
///   the document.
/// Arrange: `output.destinations` of one entry whose `kind` is the
///   `Debug` text of the member under test, the file entry's `path`
///   beneath the arranged directory, over each member of the public
///   `DESTINATION_KINDS` named through `--set`.
/// Act:     the binary, once per member.
/// Assert:  `!status.success()`; the destination the member names holds
///   the refused document, standard output for `StandardOutput` and the
///   file for `File`; standard error is empty.
/// Boundary: the process call; the real chain is `layers`, `admit_output`,
///   `config`, `output`, then `main`.
/// Mocked:   none; the filesystem is a real directory.
#[test]
fn every_declared_destination_kind_is_admitted_to_its_own_variant() {
    for member in DESTINATION_KINDS {
        // Arrange
        let name = format!("{member:?}");
        let directory = work_directory();
        let outcome_file = directory.join("outcome.json");
        let entries = vec![(*member, outcome_file.clone())];
        let arguments = harness_arguments(&directory, toml_destinations(&entries));

        // Act
        let output = run_harness(&arguments);

        // Assert
        assert!(!output.status.success(), "{name}");
        match member {
            DestinationKind::StandardOutput => {
                assert_refused(&stdout_text(&output), &name);
                assert!(!outcome_file.exists(), "{name}");
            }
            DestinationKind::File => {
                assert_refused(&file_text(&outcome_file), &name);
                assert!(stdout_text(&output).is_empty(), "{name}");
            }
        }
        assert!(stderr_text(&output).is_empty(), "{name}");
        // every_declared_destination_kind_is_admitted_to_its_own_variant
    }
}

/// Contract, carried: entry `a_destination_list_keeps_the_order_the_
///   document_holds` of `harness-crypto/config/admit_output`; a
///   destination that is refused stops the writes after it and leaves
///   the writes before it, ending in a failure status and a line on
///   standard error.
/// Arrange: `output.destinations` of a list whose first destination is
///   the path of a directory and a list whose last destination is that
///   path.
/// Act:     the binary, once per variation.
/// Assert:  `!status.success()`; standard error begins
///   `WriteFile { path: `; the first order leaves the later file absent
///   and the reverse leaves the earlier file holding the refused
///   document; standard output is empty.
/// Boundary: the process call; the real chain is `layers`, `admit_output`,
///   `config`, `output`, then `main`.
/// Mocked:   none; the filesystem is a real directory.
#[test]
fn a_destination_list_keeps_the_order_the_document_holds() {
    for (name, refused_first) in [
        ("the path of a directory first", true),
        ("the path of a directory last", false),
    ] {
        // Arrange
        let directory = work_directory();
        let refused = directory.join("a-directory");
        std::fs::create_dir(&refused).expect("the refused path is a directory");
        let written = directory.join("outcome.json");
        let entries = if refused_first {
            vec![
                (DestinationKind::File, refused.clone()),
                (DestinationKind::File, written.clone()),
            ]
        } else {
            vec![
                (DestinationKind::File, written.clone()),
                (DestinationKind::File, refused.clone()),
            ]
        };
        let arguments = harness_arguments(&directory, toml_destinations(&entries));

        // Act
        let output = run_harness(&arguments);

        // Assert
        assert!(!output.status.success(), "{name}");
        assert!(
            stderr_text(&output).starts_with("WriteFile { path: "),
            "{name}"
        );
        if refused_first {
            assert!(!written.exists(), "{name}");
        } else {
            assert_refused(&file_text(&written), name);
        }
        assert!(stdout_text(&output).is_empty(), "{name}");
        // a_destination_list_keeps_the_order_the_document_holds
    }
}
