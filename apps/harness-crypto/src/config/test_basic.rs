#![allow(clippy::expect_used)]

use core::num::NonZeroU32;

use super::admit_generate::provides::build_admitted_generate;
use super::admit_output::provides::build_admitted_output;
use super::admit_selections::provides::build_admitted_selections;
use super::admit_statements::provides::{
    build_admitted_identities, build_admitted_network_profile, build_admitted_statements,
};
use super::config;
use super::mock::{
    build_config_deps, build_config_params, build_config_payload, build_harness_configuration,
    build_measurement_settings, build_report_settings,
};
use crate::report::provides::{ReportTextConstructorParamsOverrides, build_report_text};

/// Contract: Computed → the selections, network profile, statements,
///   identities, generate settings, and output are the values the mocked
///   functions returned.
/// Arrange: the default deps, params, and payload.
/// Act:     `config`, once.
/// Assert:  the result is `Ok`; its `selections` equals
///   `build_admitted_selections(Default::default())`, its `network_profile`
///   equals `build_admitted_network_profile(Default::default())`, its
///   `identities` equals `build_admitted_identities(Default::default())`,
///   its `generate` equals `build_admitted_generate(Default::default())`,
///   its `output` equals `build_admitted_output(Default::default())`, and
///   its statements' contexts equal those of
///   `build_admitted_statements(Default::default())` with purposes' codes
///   `1` and `2`.
#[test]
fn config_returns_the_values_the_composed_functions_returned() {
    // Arrange
    let expected_selections = build_admitted_selections(Default::default());
    let expected_network_profile = build_admitted_network_profile(Default::default());
    let expected_identities = build_admitted_identities(Default::default());
    let expected_generate = build_admitted_generate(Default::default());
    let expected_output = build_admitted_output(Default::default());
    let expected_statements = build_admitted_statements(Default::default());
    // Act
    let configuration = config(
        &build_config_deps(Default::default()),
        build_config_params(Default::default()),
        build_config_payload(Default::default()),
    )
    .expect("the configuration succeeds")
    .configuration;
    // Assert
    assert_eq!(configuration.selections, expected_selections);
    assert_eq!(configuration.network_profile, expected_network_profile);
    assert_eq!(configuration.identities, expected_identities);
    assert_eq!(configuration.generate, expected_generate);
    assert_eq!(configuration.output, expected_output);
    assert_eq!(configuration.statements.mint, expected_statements.mint);
    assert_eq!(configuration.statements.mint_purpose.code(), 1);
    assert_eq!(
        configuration.statements.transfer,
        expected_statements.transfer
    );
    assert_eq!(configuration.statements.transfer_purpose.code(), 2);
    // config_returns_the_values_the_composed_functions_returned
}

/// Contract: Benchmark iterations and Measurement → the counts are
///   `NonZeroU32` over the document's values.
/// Arrange: the default deps, params, and payload.
/// Act:     `config`, once.
/// Assert:  `benchmark_iterations` equals `NonZeroU32::MIN` and `measurement`
///   equals `build_measurement_settings(Default::default())`.
#[test]
fn config_admits_the_counts_the_document_holds() {
    // Arrange
    let expected = build_measurement_settings(Default::default());
    // Act
    let configuration = config(
        &build_config_deps(Default::default()),
        build_config_params(Default::default()),
        build_config_payload(Default::default()),
    )
    .expect("the configuration succeeds")
    .configuration;
    // Assert
    assert_eq!(configuration.benchmark_iterations, NonZeroU32::MIN);
    assert_eq!(configuration.measurement, expected);
    // config_admits_the_counts_the_document_holds
}

/// Contract: Report defaults and Report adjustment → the defaults are the
///   document's piece-group size and primary curve, and an absent adjustment
///   stays absent.
/// Arrange: the default deps, params, and payload.
/// Act:     `config`, once.
/// Assert:  `report` equals `build_report_settings(Default::default())`,
///   whose `adjustment` is `None`.
#[test]
fn config_admits_the_report_defaults_and_an_absent_adjustment() {
    // Arrange
    let expected = build_report_settings(Default::default());
    // Act
    let configuration = config(
        &build_config_deps(Default::default()),
        build_config_params(Default::default()),
        build_config_payload(Default::default()),
    )
    .expect("the configuration succeeds")
    .configuration;
    // Assert
    assert_eq!(configuration.report, expected);
    assert!(configuration.report.adjustment.is_none());
    // config_admits_the_report_defaults_and_an_absent_adjustment
}

/// Contract: Curve labels → each admitted curve's label is admitted as a
///   `ReportText` at its index.
/// Arrange: the default deps, params, and payload.
/// Act:     `config`, once.
/// Assert:  `curve_labels` equals one `build_report_text` over `"curve-a"`.
#[test]
fn config_admits_one_label_per_curve_in_order() {
    // Arrange
    let expected = vec![build_report_text(ReportTextConstructorParamsOverrides {
        text: Some("curve-a".to_string()),
    })];
    // Act
    let configuration = config(
        &build_config_deps(Default::default()),
        build_config_params(Default::default()),
        build_config_payload(Default::default()),
    )
    .expect("the configuration succeeds")
    .configuration;
    // Assert
    assert_eq!(configuration.curve_labels, expected);
    // config_admits_one_label_per_curve_in_order
}

/// Contract: Verifier endpoint and Verifier concrete → the Evm arm holds the
///   endpoint, the deployment record, and the gas price oracle.
/// Arrange: the default deps, params, and payload.
/// Act:     `config`, once.
/// Assert:  `verifier` equals `build_harness_configuration(Default::default()).verifier`.
#[test]
fn config_selects_the_evm_verifier_with_the_documents_endpoint_and_addresses() {
    // Arrange
    let expected = build_harness_configuration(Default::default()).verifier;
    // Act
    let configuration = config(
        &build_config_deps(Default::default()),
        build_config_params(Default::default()),
        build_config_payload(Default::default()),
    )
    .expect("the configuration succeeds")
    .configuration;
    // Assert
    assert_eq!(configuration.verifier, expected);
    // config_selects_the_evm_verifier_with_the_documents_endpoint_and_addresses
}
