#![allow(clippy::expect_used)]

use core::num::NonZeroU32;
use std::env::temp_dir;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;

use chain::DECLARED_CHAIN_FORMS_SELECTIONS;
use encoding::{CanonicalFieldValue, DECLARED_ENCODING_SELECTIONS};
use envelope::DECLARED_KEY_AGREEMENT_SELECTIONS;
use hash_to_scalar::DECLARED_HASH_TO_SCALAR_SELECTIONS;
use kem::{DECLARED_KEM_SELECTIONS, IdentityScope, KemConcrete};
use pairing::{PAIRING_CONCRETES, PRECOMPILE_ENCODINGS, TARGET_GROUP_ENCODING_IDENTIFIERS};
use proof::DELIVERY_PROOF_CONCRETES;
use random::RANDOM_SOURCE_KINDS;

use super::admit_generate::provides::{AdmitGenerateErrorReturn, admit_generate};
use super::admit_output::provides::{AdmitOutputErrorReturn, OutputDestination, admit_output};
use super::admit_selections::provides::{
    AdmitSelectionsErrorReturn, AdmittedChainForms, AdmittedCurve, AdmittedDeliveryProof,
    AdmittedEncoding, AdmittedHashToScalar, AdmittedKem, AdmittedKeyAgreement, AdmittedPairing,
    AdmittedSelections, admit_selections, declared_name,
};
use super::admit_statements::provides::{
    AdmitStatementsErrorReturn, AdmittedIdentities, AdmittedIdentity, AdmittedNetworkProfile,
    CanonicalValueRefusal, NetworkProfileField, admit_statements,
};
use super::config;
use super::interface::{
    ConfigDeps, ConfigErrorReturn, ConfigParams, ConfigPayload, MeasurementSettings,
    ReportSettings, SHIPPED_DEFAULTS,
};
use super::layers::provides::{LayersErrorReturn, layers};
use crate::generate::provides::{GenerateConcrete, SOLIDITY_GENERATE_DECLARATION};
use crate::report::provides::{
    ReportDefaults, ReportDestination, ReportTextConstructorParamsOverrides,
    ReportTextTryNewErrorReturn, build_report_text,
};
use crate::verifier::provides::{
    EvmVerifierEndpoint, EvmVerifierEndpointConstructorParams,
    EvmVerifierEndpointTryNewErrorReturn, EvmVerifierSettings, VERIFIER_CONCRETES,
    VerifierConcrete,
};

/// Contract: own entry `the_shipped_defaults_configure_the_harness` —
///   `config` over `SHIPPED_DEFAULTS` and the program name alone, every
///   callee real, returns `Ok` whose configuration holds, for each family,
///   the declared member the default text names with the identifier, scope,
///   and algebra that member carries, one curve and one curve label per
///   `[[selections.curves]]` entry in order, one benchmark concrete per
///   `[[selections.benchmark_concretes]]` entry in order, one
///   `VerifierConcrete::Evm` per curve in curve order, each over the
///   default endpoint, that curve's default deployment record, and the gas
///   price oracle, the two statements' purposes `1` and `2`, the
///   generate settings holding one kind per kind of
///   `SOLIDITY_GENERATE_DECLARATION`, the output destinations the default
///   holds, and the counts and report defaults the default holds.
/// Arrange: the program name alone over `SHIPPED_DEFAULTS`; the expected
///   selections built member by member from each family's declared set, the
///   expected verifier settings per curve over the default endpoint, that
///   curve's default deployment record, and the oracle, and the expected
///   counts, texts, and destinations.
/// Act:     `config`, once.
/// Assert:  the result is `Ok` and each field equals the expected value.
/// Boundary: the private call `config` and the chain of real functions the
///   private surface names — `layers`, `admit_selections`,
///   `admit_statements`, `admit_generate`, `admit_output`,
///   `ReportText::try_new`, and `EvmVerifierEndpoint::try_new`, all running
///   for real over `SHIPPED_DEFAULTS`.
/// Mocked:  none; the command line names no file, so the filesystem is
///   never read.
#[test]
fn the_shipped_defaults_configure_the_harness() {
    // Arrange
    let deps = ConfigDeps {
        layers,
        admit_selections,
        admit_statements,
        admit_generate,
        admit_output,
    };
    let params = ConfigParams {
        arguments: vec![OsString::from("harness-crypto")],
    };
    let payload = ConfigPayload {
        defaults: SHIPPED_DEFAULTS.to_string(),
    };
    let kem_member = DECLARED_KEM_SELECTIONS
        .iter()
        .find(|member| member.scope == IdentityScope::Entitlement)
        .expect("the declared set names the entitlement scope");
    let expected_selections = AdmittedSelections {
        random_source: RANDOM_SOURCE_KINDS[0],
        encoding: AdmittedEncoding {
            concrete: DECLARED_ENCODING_SELECTIONS[0].concrete,
            identifier: DECLARED_ENCODING_SELECTIONS[0].identifier,
        },
        hash_to_scalar: AdmittedHashToScalar {
            concrete: DECLARED_HASH_TO_SCALAR_SELECTIONS[0].concrete,
            identifier: DECLARED_HASH_TO_SCALAR_SELECTIONS[0].identifier,
        },
        chain_forms: AdmittedChainForms {
            concrete: DECLARED_CHAIN_FORMS_SELECTIONS[0].concrete,
            identifier: DECLARED_CHAIN_FORMS_SELECTIONS[0].identifier,
        },
        kem: AdmittedKem {
            concrete: kem_member.concrete,
            identifier: kem_member.identifier,
            scope: kem_member.scope,
        },
        key_agreement: AdmittedKeyAgreement {
            concrete: DECLARED_KEY_AGREEMENT_SELECTIONS[0].concrete,
            identifier: DECLARED_KEY_AGREEMENT_SELECTIONS[0].identifier,
            algebra: DECLARED_KEY_AGREEMENT_SELECTIONS[0].algebra,
        },
        delivery_proof: AdmittedDeliveryProof {
            concrete: DELIVERY_PROOF_CONCRETES[0],
            algebra: DECLARED_KEY_AGREEMENT_SELECTIONS[0].algebra,
            statement_version: 1,
        },
        verifier: VERIFIER_CONCRETES[0],
        curves: vec![
            AdmittedCurve {
                label: "bn254".to_string(),
                pairing: AdmittedPairing {
                    concrete: PAIRING_CONCRETES[0],
                    target_group_encoding: TARGET_GROUP_ENCODING_IDENTIFIERS[0],
                },
            },
            AdmittedCurve {
                label: "bls12_381".to_string(),
                pairing: AdmittedPairing {
                    concrete: PAIRING_CONCRETES[2],
                    target_group_encoding: TARGET_GROUP_ENCODING_IDENTIFIERS[1],
                },
            },
        ],
        benchmark_concretes: vec![
            AdmittedPairing {
                concrete: PAIRING_CONCRETES[0],
                target_group_encoding: TARGET_GROUP_ENCODING_IDENTIFIERS[0],
            },
            AdmittedPairing {
                concrete: PAIRING_CONCRETES[1],
                target_group_encoding: TARGET_GROUP_ENCODING_IDENTIFIERS[0],
            },
            AdmittedPairing {
                concrete: PAIRING_CONCRETES[2],
                target_group_encoding: TARGET_GROUP_ENCODING_IDENTIFIERS[1],
            },
            AdmittedPairing {
                concrete: PAIRING_CONCRETES[3],
                target_group_encoding: TARGET_GROUP_ENCODING_IDENTIFIERS[1],
            },
        ],
        precompile_encodings: PRECOMPILE_ENCODINGS.to_vec(),
    };
    let mut chain_identifier = [0u8; 32];
    chain_identifier[24..].copy_from_slice(&84532u64.to_be_bytes());
    let mut oracle = [0u8; 20];
    oracle[0] = 0x42;
    oracle[19] = 0x0f;
    let expected_network_profile = AdmittedNetworkProfile {
        chain_identifier: CanonicalFieldValue::Unsigned256(chain_identifier),
        entitlement_contract: CanonicalFieldValue::FixedBytes20([0xee; 20]),
        deployment_records: vec![
            CanonicalFieldValue::FixedBytes20([0; 20]),
            CanonicalFieldValue::FixedBytes20([0; 20]),
        ],
        gas_price_oracle: CanonicalFieldValue::FixedBytes20(oracle),
    };
    let expected_identities = AdmittedIdentities {
        proof: AdmittedIdentity::Entitlement {
            canonical: vec![0x55; 32],
        },
        mapping: vec![
            AdmittedIdentity::Entitlement {
                canonical: vec![0x66; 32],
            },
            AdmittedIdentity::Entitlement {
                canonical: vec![0x77; 32],
            },
        ],
    };
    let expected_verifiers = vec![
        VerifierConcrete::Evm {
            settings: Box::new(EvmVerifierSettings {
                endpoint: EvmVerifierEndpoint::try_new(EvmVerifierEndpointConstructorParams {
                    text: "https://sepolia.base.org".to_string(),
                })
                .expect("the endpoint is admitted"),
                verifier_address: CanonicalFieldValue::FixedBytes20([0; 20]),
                gas_price_oracle: CanonicalFieldValue::FixedBytes20(oracle),
            }),
        },
        VerifierConcrete::Evm {
            settings: Box::new(EvmVerifierSettings {
                endpoint: EvmVerifierEndpoint::try_new(EvmVerifierEndpointConstructorParams {
                    text: "https://sepolia.base.org".to_string(),
                })
                .expect("the endpoint is admitted"),
                verifier_address: CanonicalFieldValue::FixedBytes20([0; 20]),
                gas_price_oracle: CanonicalFieldValue::FixedBytes20(oracle),
            }),
        },
    ];
    let expected_curve_labels = vec![
        build_report_text(ReportTextConstructorParamsOverrides {
            text: Some("bn254".to_string()),
        }),
        build_report_text(ReportTextConstructorParamsOverrides {
            text: Some("bls12_381".to_string()),
        }),
    ];
    let expected_measurement = MeasurementSettings {
        iterations: NonZeroU32::new(100).expect("one hundred is nonzero"),
        conditions: build_report_text(ReportTextConstructorParamsOverrides {
            text: Some(
                "Base Sepolia verifier, L1 data fee as Sepolia's, one idle machine".to_string(),
            ),
        }),
    };
    let expected_report = ReportSettings {
        defaults: ReportDefaults {
            piece_group_size: NonZeroU32::new(16384).expect("the size is nonzero"),
            primary_curve: build_report_text(ReportTextConstructorParamsOverrides {
                text: Some("bls12_381".to_string()),
            }),
        },
        adjustment: None,
    };
    // Act
    let configuration = config(&deps, params, payload)
        .expect("the shipped defaults configure")
        .configuration;
    // Assert
    assert_eq!(configuration.selections, expected_selections);
    assert_eq!(configuration.network_profile, expected_network_profile);
    assert_eq!(configuration.identities, expected_identities);
    assert_eq!(configuration.verifiers, expected_verifiers);
    assert_eq!(configuration.curve_labels, expected_curve_labels);
    assert_eq!(configuration.statements.mint_purpose.code(), 1);
    assert_eq!(configuration.statements.transfer_purpose.code(), 2);
    assert!(
        matches!(
            configuration.generate.concrete,
            GenerateConcrete::Solidity { .. }
        ),
        "the concrete is Solidity"
    );
    assert_eq!(
        configuration.generate.kinds.len(),
        SOLIDITY_GENERATE_DECLARATION.kinds.len()
    );
    for declared in SOLIDITY_GENERATE_DECLARATION.kinds {
        assert!(
            configuration
                .generate
                .kinds
                .iter()
                .any(|configured| configured.kind == declared.kind),
            "one configuration per declared kind"
        );
    }
    assert_eq!(
        configuration.output.destinations,
        vec![OutputDestination::StandardOutput]
    );
    assert_eq!(
        configuration.output.report_destinations,
        vec![ReportDestination::File {
            path: PathBuf::from("harness-report.json")
        }]
    );
    assert_eq!(
        configuration.benchmark_iterations,
        NonZeroU32::new(100).expect("one hundred is nonzero")
    );
    assert_eq!(configuration.measurement, expected_measurement);
    assert_eq!(configuration.report, expected_report);
    // the_shipped_defaults_configure_the_harness
}

/// Contract: own entry `a_file_and_a_flag_each_replace_a_shipped_default` —
///   `config` over `SHIPPED_DEFAULTS` and a command line naming a `--config`
///   file, a `--set` assignment, or both in each order, each over a default
///   the document declares as a scalar, as a table key, and as an array,
///   every callee real and the filesystem real, returns `Ok` whose
///   configuration holds, for each key a later layer named, the value of the
///   last layer that named it and, for each key no layer named, the shipped
///   default.
/// Arrange: a directory beneath the operating system's temporary directory
///   removed before the act; a file and a flag for `benchmark.iterations`,
///   `selections.identity_scope`, and `selections.curves` each in the three
///   interleavings file then flag, flag then file, and file alone.
/// Act:     `config`, once per interleaving per key.
/// Assert:  each result is `Ok` and the admitted value equals the value of
///   the last layer that named its key.
/// Boundary: the private call `config` and the chain of real functions the
///   private surface names, all running for real over `SHIPPED_DEFAULTS`.
/// Mocked:  none; the filesystem is a real directory beneath the operating
///   system's temporary directory.
#[test]
fn a_file_and_a_flag_each_replace_a_shipped_default() {
    // Arrange — the directory, the deps, and the payload are invariant.
    let directory: PathBuf = temp_dir().join(format!(
        "chaintorrent-config-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = fs::remove_dir_all(&directory);
    fs::create_dir_all(&directory).expect("the directory is created");
    let deps = ConfigDeps {
        layers,
        admit_selections,
        admit_statements,
        admit_generate,
        admit_output,
    };
    let payload = ConfigPayload {
        defaults: SHIPPED_DEFAULTS.to_string(),
    };
    let program = OsString::from("harness-crypto");
    let config_flag = OsString::from("--config");
    let set_flag = OsString::from("--set");

    // The scalar `benchmark.iterations`: the file holds 3, the flag 4.
    let benchmark_file = directory.join("benchmark.toml");
    fs::write(&benchmark_file, "[benchmark]\niterations = 3\n")
        .expect("the benchmark file is written");
    let benchmark_path = benchmark_file.into_os_string();
    let benchmark_set = OsString::from("benchmark.iterations=4");
    let benchmark_cases: Vec<(Vec<OsString>, u32)> = vec![
        (
            vec![
                program.clone(),
                config_flag.clone(),
                benchmark_path.clone(),
                set_flag.clone(),
                benchmark_set.clone(),
            ],
            4,
        ),
        (
            vec![
                program.clone(),
                set_flag.clone(),
                benchmark_set.clone(),
                config_flag.clone(),
                benchmark_path.clone(),
            ],
            3,
        ),
        (
            vec![program.clone(), config_flag.clone(), benchmark_path.clone()],
            3,
        ),
    ];
    for (arguments, expected) in benchmark_cases {
        // Act
        let configuration = config(&deps, ConfigParams { arguments }, payload.clone())
            .expect("the layers configure")
            .configuration;
        // Assert
        assert_eq!(configuration.benchmark_iterations.get(), expected);
    }

    // The table key `selections.identity_scope`: the file holds `Asset`, the
    // flag `Entitlement`.
    let scope_file = directory.join("scope.toml");
    fs::write(&scope_file, "[selections]\nidentity_scope = \"Asset\"\n")
        .expect("the scope file is written");
    let scope_path = scope_file.into_os_string();
    let scope_set = OsString::from(r#"selections.identity_scope="Entitlement""#);
    let scope_cases: Vec<(Vec<OsString>, IdentityScope)> = vec![
        (
            vec![
                program.clone(),
                config_flag.clone(),
                scope_path.clone(),
                set_flag.clone(),
                scope_set.clone(),
            ],
            IdentityScope::Entitlement,
        ),
        (
            vec![
                program.clone(),
                set_flag.clone(),
                scope_set.clone(),
                config_flag.clone(),
                scope_path.clone(),
            ],
            IdentityScope::Asset,
        ),
        (
            vec![program.clone(), config_flag.clone(), scope_path.clone()],
            IdentityScope::Asset,
        ),
    ];
    for (arguments, expected) in scope_cases {
        // Act
        let configuration = config(&deps, ConfigParams { arguments }, payload.clone())
            .expect("the layers configure")
            .configuration;
        // Assert
        assert_eq!(configuration.selections.kem.scope, expected);
        match expected {
            IdentityScope::Entitlement => assert!(
                matches!(
                    configuration.identities.proof,
                    AdmittedIdentity::Entitlement { .. }
                ),
                "the proof identity is an entitlement identity"
            ),
            IdentityScope::Asset => assert!(
                matches!(
                    configuration.identities.proof,
                    AdmittedIdentity::Asset { .. }
                ),
                "the proof identity is an asset identity"
            ),
        }
    }

    // The array `selections.curves`: the file lists the shipped curves in
    // reverse order, the flag in the shipped order — each layer's array
    // replaces the other whole.
    let curves_file = directory.join("curves.toml");
    fs::write(
        &curves_file,
        "[[selections.curves]]\nlabel = \"bls12_381\"\npairing_concrete = \"Bls12381Arkworks\"\ntarget_group_encoding = \"Bls12381V1\"\n[[selections.curves]]\nlabel = \"bn254\"\npairing_concrete = \"Bn254Arkworks\"\ntarget_group_encoding = \"Bn254V1\"\n",
    )
    .expect("the curves file is written");
    let curves_path = curves_file.into_os_string();
    let curves_set = OsString::from(
        r#"selections.curves=[{label="bn254",pairing_concrete="Bn254Arkworks",target_group_encoding="Bn254V1"},{label="bls12_381",pairing_concrete="Bls12381Arkworks",target_group_encoding="Bls12381V1"}]"#,
    );
    let bn254_curve = AdmittedCurve {
        label: "bn254".to_string(),
        pairing: AdmittedPairing {
            concrete: PAIRING_CONCRETES[0],
            target_group_encoding: TARGET_GROUP_ENCODING_IDENTIFIERS[0],
        },
    };
    let bls12_381_curve = AdmittedCurve {
        label: "bls12_381".to_string(),
        pairing: AdmittedPairing {
            concrete: PAIRING_CONCRETES[2],
            target_group_encoding: TARGET_GROUP_ENCODING_IDENTIFIERS[1],
        },
    };
    let file_curves = vec![bls12_381_curve.clone(), bn254_curve.clone()];
    let flag_curves = vec![bn254_curve, bls12_381_curve];
    let curve_cases: Vec<(Vec<OsString>, Vec<AdmittedCurve>)> = vec![
        (
            vec![
                program.clone(),
                config_flag.clone(),
                curves_path.clone(),
                set_flag.clone(),
                curves_set.clone(),
            ],
            flag_curves.clone(),
        ),
        (
            vec![
                program.clone(),
                set_flag.clone(),
                curves_set.clone(),
                config_flag.clone(),
                curves_path.clone(),
            ],
            file_curves.clone(),
        ),
        (
            vec![program.clone(), config_flag.clone(), curves_path.clone()],
            file_curves.clone(),
        ),
    ];
    for (arguments, expected) in curve_cases {
        // Act
        let configuration = config(&deps, ConfigParams { arguments }, payload.clone())
            .expect("the layers configure")
            .configuration;
        // Assert
        assert_eq!(configuration.selections.curves, expected);
        assert_eq!(
            configuration.curve_labels,
            expected
                .iter()
                .map(|curve| {
                    build_report_text(ReportTextConstructorParamsOverrides {
                        text: Some(curve.label.clone()),
                    })
                })
                .collect::<Vec<_>>()
        );
    }
    // a_file_and_a_flag_each_replace_a_shipped_default
}

/// Contract: own entry `an_unadmissible_value_is_refused_by_the_function_
///   that_admits_it` — `config` over `SHIPPED_DEFAULTS` with one value
///   overridden by a `--set` assignment, every callee real, returns `Err`
///   carrying the callee's refusal whole inside the variant of the function
///   that returned it.
/// Arrange: for each variation an argument list overriding one value — an
///   unknown flag, a key the document does not declare, the `Debug` text of
///   a family's mock selection in `selections.kem`, an unknown kind in
///   `network_profile.chain_identifier`, an unknown `generate.concrete`,
///   and an empty `output.destinations`.
/// Act:     `config`, once per variation.
/// Assert:  each result is `Err` of the stated variant, the callee's refusal
///   carried whole.
/// Boundary: the private call `config` and the chain of real functions the
///   private surface names, all running for real over `SHIPPED_DEFAULTS`.
/// Mocked:  none.
#[test]
fn an_unadmissible_value_is_refused_by_the_function_that_admits_it() {
    // Arrange — the deps and the payload are invariant across every case.
    let deps = ConfigDeps {
        layers,
        admit_selections,
        admit_statements,
        admit_generate,
        admit_output,
    };
    let payload = ConfigPayload {
        defaults: SHIPPED_DEFAULTS.to_string(),
    };
    let program = OsString::from("harness-crypto");
    let set_flag = OsString::from("--set");
    let mock_kem = declared_name(&KemConcrete::Mock);

    // An unknown flag is refused by `layers` before the defaults are parsed.
    let result = config(
        &deps,
        ConfigParams {
            arguments: vec![program.clone(), OsString::from("--unknown-flag")],
        },
        payload.clone(),
    );
    assert!(
        matches!(
            result,
            Err(ConfigErrorReturn::Layers(LayersErrorReturn::Arguments(_)))
        ),
        "the refusal is Layers(Arguments)"
    );

    // A key the document does not declare is refused by `layers` at the
    // document.
    let result = config(
        &deps,
        ConfigParams {
            arguments: vec![
                program.clone(),
                set_flag.clone(),
                OsString::from("undeclared=1"),
            ],
        },
        payload.clone(),
    );
    assert!(
        matches!(
            result,
            Err(ConfigErrorReturn::Layers(LayersErrorReturn::Document(_)))
        ),
        "the refusal is Layers(Document)"
    );

    // The `Debug` text of the KEM family's mock selection is refused by
    // `admit_selections` inside `Selections`.
    let result = config(
        &deps,
        ConfigParams {
            arguments: vec![
                program.clone(),
                set_flag.clone(),
                OsString::from(format!("selections.kem=\"{mock_kem}\"")),
            ],
        },
        payload.clone(),
    );
    assert!(
        matches!(
            result,
            Err(ConfigErrorReturn::Selections(
                AdmitSelectionsErrorReturn::Kem { ref text }
            )) if text == &mock_kem
        ),
        "the refusal is Selections(Kem) carrying the text"
    );

    // An unknown kind in `network_profile.chain_identifier` is refused by
    // `admit_statements` inside `Statements`.
    let result = config(
        &deps,
        ConfigParams {
            arguments: vec![
                program.clone(),
                set_flag.clone(),
                OsString::from(r#"network_profile.chain_identifier.kind="Unknown""#),
            ],
        },
        payload.clone(),
    );
    assert!(
        matches!(
            result,
            Err(ConfigErrorReturn::Statements(
                AdmitStatementsErrorReturn::NetworkProfile {
                    field: NetworkProfileField::ChainIdentifier,
                    refusal: CanonicalValueRefusal::UnknownKind { ref text }
                }
            )) if text == "Unknown"
        ),
        "the refusal is Statements(NetworkProfile) carrying the kind text"
    );

    // An unknown `generate.concrete` is refused by `admit_generate` inside
    // `Generate`.
    let result = config(
        &deps,
        ConfigParams {
            arguments: vec![
                program.clone(),
                set_flag.clone(),
                OsString::from(r#"generate.concrete="unknown""#),
            ],
        },
        payload.clone(),
    );
    assert!(
        matches!(
            result,
            Err(ConfigErrorReturn::Generate(
                AdmitGenerateErrorReturn::Concrete { ref text }
            )) if text == "unknown"
        ),
        "the refusal is Generate(Concrete) carrying the text"
    );

    // An empty `output.destinations` is refused by `admit_output` inside
    // `Output`.
    let result = config(
        &deps,
        ConfigParams {
            arguments: vec![
                program.clone(),
                set_flag.clone(),
                OsString::from("output.destinations=[]"),
            ],
        },
        payload.clone(),
    );
    assert!(
        matches!(
            result,
            Err(ConfigErrorReturn::Output(
                AdmitOutputErrorReturn::OutputDestinationsEmpty
            ))
        ),
        "the refusal is Output(OutputDestinationsEmpty)"
    );
    // an_unadmissible_value_is_refused_by_the_function_that_admits_it
}

/// Contract: own entry `a_value_this_function_admits_is_refused_with_its_
///   own_variant` — `config` over `SHIPPED_DEFAULTS` with one value
///   overridden by a `--set` assignment, every callee real, returns `Err` of
///   the variant that names the document path.
/// Arrange: for each variation an argument list overriding one value — a
///   zero for each count, a text of whitespace alone for each text, a curve
///   label of whitespace alone at index one, and an endpoint text no URL
///   parser admits.
/// Act:     `config`, once per variation.
/// Assert:  each result is `Err` of the variant that names the document
///   path, any carried error the callee's own.
/// Boundary: the private call `config` and the chain of real functions the
///   private surface names, all running for real over `SHIPPED_DEFAULTS`.
/// Mocked:  none.
#[test]
fn a_value_this_function_admits_is_refused_with_its_own_variant() {
    // Arrange — the deps and the payload are invariant across every case.
    let deps = ConfigDeps {
        layers,
        admit_selections,
        admit_statements,
        admit_generate,
        admit_output,
    };
    let payload = ConfigPayload {
        defaults: SHIPPED_DEFAULTS.to_string(),
    };
    let program = OsString::from("harness-crypto");
    let set_flag = OsString::from("--set");
    let set = |assignment: &str| ConfigParams {
        arguments: vec![
            program.clone(),
            set_flag.clone(),
            OsString::from(assignment),
        ],
    };

    // A zero is refused for each count.
    let result = config(&deps, set("benchmark.iterations=0"), payload.clone());
    assert!(
        matches!(result, Err(ConfigErrorReturn::BenchmarkIterations)),
        "the refusal is BenchmarkIterations"
    );
    let result = config(&deps, set("measurement.iterations=0"), payload.clone());
    assert!(
        matches!(result, Err(ConfigErrorReturn::MeasurementIterations)),
        "the refusal is MeasurementIterations"
    );
    let result = config(&deps, set("report.piece_group_size=0"), payload.clone());
    assert!(
        matches!(result, Err(ConfigErrorReturn::ReportPieceGroupSize)),
        "the refusal is ReportPieceGroupSize"
    );
    let result = config(
        &deps,
        set("report.adjustment.piece_group_size=0"),
        payload.clone(),
    );
    assert!(
        matches!(result, Err(ConfigErrorReturn::AdjustmentPieceGroupSize)),
        "the refusal is AdjustmentPieceGroupSize"
    );

    // A text of whitespace alone is refused for each text.
    let result = config(&deps, set(r#"measurement.conditions=" ""#), payload.clone());
    assert!(
        matches!(
            result,
            Err(ConfigErrorReturn::MeasurementConditions(
                ReportTextTryNewErrorReturn::Empty
            ))
        ),
        "the refusal is MeasurementConditions(Empty)"
    );
    let result = config(&deps, set(r#"report.primary_curve=" ""#), payload.clone());
    assert!(
        matches!(
            result,
            Err(ConfigErrorReturn::ReportPrimaryCurve(
                ReportTextTryNewErrorReturn::Empty
            ))
        ),
        "the refusal is ReportPrimaryCurve(Empty)"
    );
    let result = config(
        &deps,
        set(r#"report.adjustment.primary_curve=" ""#),
        payload.clone(),
    );
    assert!(
        matches!(
            result,
            Err(ConfigErrorReturn::AdjustmentPrimaryCurve(
                ReportTextTryNewErrorReturn::Empty
            ))
        ),
        "the refusal is AdjustmentPrimaryCurve(Empty)"
    );

    // A curve label of whitespace alone is refused at index one; the label
    // carries a deployment record so `admit_statements` admits the profile
    // and `config` reaches the curve-label step.
    let result = config(
        &deps,
        ConfigParams {
            arguments: vec![
                program.clone(),
                set_flag.clone(),
                OsString::from(
                    r#"selections.curves=[{label="bn254",pairing_concrete="Bn254Arkworks",target_group_encoding="Bn254V1"},{label=" ",pairing_concrete="Bls12381Arkworks",target_group_encoding="Bls12381V1"},{label="bls12_381",pairing_concrete="Bls12381Arkworks",target_group_encoding="Bls12381V1"}]"#,
                ),
                set_flag.clone(),
                OsString::from(
                    r#"network_profile.deployment_records." "={kind="FixedBytes20",value="0x0000000000000000000000000000000000000000"}"#,
                ),
            ],
        },
        payload.clone(),
    );
    assert!(
        matches!(
            result,
            Err(ConfigErrorReturn::CurveLabel {
                index: 1,
                error: ReportTextTryNewErrorReturn::Empty
            })
        ),
        "the refusal is CurveLabel at index one"
    );

    // An endpoint text no URL parser admits is refused.
    let result = config(
        &deps,
        set(r#"network_profile.verifier_endpoint="no-scheme""#),
        payload.clone(),
    );
    assert!(
        matches!(
            result,
            Err(ConfigErrorReturn::VerifierEndpoint(
                EvmVerifierEndpointTryNewErrorReturn::Unparsable { error: _ }
            ))
        ),
        "the refusal is VerifierEndpoint(Unparsable)"
    );
    // a_value_this_function_admits_is_refused_with_its_own_variant
}

/// Contract: own entry `the_verifier_is_selected_with_the_settings_the_
///   document_holds` — `config` over `SHIPPED_DEFAULTS` with
///   `network_profile.verifier_endpoint`,
///   `network_profile.deployment_records.bn254`,
///   `network_profile.deployment_records.bls12_381`, and `gas_price_oracle`
///   overridden, every callee real, returns `Ok` whose `verifiers` equals
///   two `VerifierConcrete::Evm` in curve order, each over the endpoint,
///   that curve's `verifier_address`, and the `gas_price_oracle` built in
///   Arrange from the overridden values.
/// Arrange: each value overridden alone, the two records overridden
///   together, and the layer of a single curve's record, which leaves the
///   other curve's shipped default; the expected settings built in Arrange
///   from the overridden values.
/// Act:     `config`, once per variation.
/// Assert:  each result is `Ok` and its `verifiers` equals the expected
///   `VerifierConcrete::Evm` in curve order.
/// Boundary: the private call `config` and the chain of real functions the
///   private surface names, all running for real over `SHIPPED_DEFAULTS`.
/// Mocked:  none.
#[test]
fn the_verifier_is_selected_with_the_settings_the_document_holds() {
    // Arrange — the deps and the payload are invariant across every case.
    let deps = ConfigDeps {
        layers,
        admit_selections,
        admit_statements,
        admit_generate,
        admit_output,
    };
    let payload = ConfigPayload {
        defaults: SHIPPED_DEFAULTS.to_string(),
    };
    let program = OsString::from("harness-crypto");
    let set_flag = OsString::from("--set");
    let default_oracle = {
        let mut oracle = [0u8; 20];
        oracle[0] = 0x42;
        oracle[19] = 0x0f;
        oracle
    };
    let expected =
        |endpoint_text: &str,
         bn254: [u8; 20],
         bls12_381: [u8; 20],
         oracle: [u8; 20]| {
            vec![
                VerifierConcrete::Evm {
                    settings: Box::new(EvmVerifierSettings {
                        endpoint: EvmVerifierEndpoint::try_new(
                            EvmVerifierEndpointConstructorParams {
                                text: endpoint_text.to_string(),
                            },
                        )
                        .expect("the endpoint is admitted"),
                        verifier_address: CanonicalFieldValue::FixedBytes20(bn254),
                        gas_price_oracle: CanonicalFieldValue::FixedBytes20(oracle),
                    }),
                },
                VerifierConcrete::Evm {
                    settings: Box::new(EvmVerifierSettings {
                        endpoint: EvmVerifierEndpoint::try_new(
                            EvmVerifierEndpointConstructorParams {
                                text: endpoint_text.to_string(),
                            },
                        )
                        .expect("the endpoint is admitted"),
                        verifier_address: CanonicalFieldValue::FixedBytes20(bls12_381),
                        gas_price_oracle: CanonicalFieldValue::FixedBytes20(oracle),
                    }),
                },
            ]
        };

    // The endpoint overridden alone.
    let result = config(
        &deps,
        ConfigParams {
            arguments: vec![
                program.clone(),
                set_flag.clone(),
                OsString::from(r#"network_profile.verifier_endpoint="http://endpoint-b.test""#),
            ],
        },
        payload.clone(),
    );
    assert_eq!(
        result
            .expect("the configuration succeeds")
            .configuration
            .verifiers,
        expected("http://endpoint-b.test", [0; 20], [0; 20], default_oracle)
    );

    // The bn254 record overridden alone, leaving the bls12_381 shipped
    // default.
    let result = config(
        &deps,
        ConfigParams {
            arguments: vec![
                program.clone(),
                set_flag.clone(),
                OsString::from(
                    r#"network_profile.deployment_records.bn254.value="0x1111111111111111111111111111111111111111""#,
                ),
            ],
        },
        payload.clone(),
    );
    assert_eq!(
        result
            .expect("the configuration succeeds")
            .configuration
            .verifiers,
        expected("https://sepolia.base.org", [0x11; 20], [0; 20], default_oracle)
    );

    // The bls12_381 record overridden alone, leaving the bn254 shipped
    // default.
    let result = config(
        &deps,
        ConfigParams {
            arguments: vec![
                program.clone(),
                set_flag.clone(),
                OsString::from(
                    r#"network_profile.deployment_records.bls12_381.value="0x2222222222222222222222222222222222222222""#,
                ),
            ],
        },
        payload.clone(),
    );
    assert_eq!(
        result
            .expect("the configuration succeeds")
            .configuration
            .verifiers,
        expected("https://sepolia.base.org", [0; 20], [0x22; 20], default_oracle)
    );

    // The gas price oracle overridden alone.
    let result = config(
        &deps,
        ConfigParams {
            arguments: vec![
                program.clone(),
                set_flag.clone(),
                OsString::from(
                    r#"network_profile.gas_price_oracle.value="0x7777777777777777777777777777777777777777""#,
                ),
            ],
        },
        payload.clone(),
    );
    assert_eq!(
        result
            .expect("the configuration succeeds")
            .configuration
            .verifiers,
        expected("https://sepolia.base.org", [0; 20], [0; 20], [0x77; 20])
    );

    // The two records overridden together.
    let result = config(
        &deps,
        ConfigParams {
            arguments: vec![
                program.clone(),
                set_flag.clone(),
                OsString::from(
                    r#"network_profile.deployment_records.bn254.value="0x1111111111111111111111111111111111111111""#,
                ),
                set_flag.clone(),
                OsString::from(
                    r#"network_profile.deployment_records.bls12_381.value="0x2222222222222222222222222222222222222222""#,
                ),
            ],
        },
        payload.clone(),
    );
    assert_eq!(
        result
            .expect("the configuration succeeds")
            .configuration
            .verifiers,
        expected("https://sepolia.base.org", [0x11; 20], [0x22; 20], default_oracle)
    );
    // the_verifier_is_selected_with_the_settings_the_document_holds
}

/// Contract: own entry `the_first_refusal_is_returned_in_the_stated_order`
///   — `config` over `SHIPPED_DEFAULTS` with a refused value in two or more
///   steps, every callee real, returns the earlier step's refusal.
/// Arrange: for each pair an argument list holding a refused value at both
///   steps — `Layers` before `Selections`, `Selections` before `Statements`,
///   `Statements` before `Generate`, `Generate` before `Output`, `Output`
///   before `BenchmarkIterations`, `BenchmarkIterations` before
///   `MeasurementIterations`, and `MeasurementIterations` before
///   `VerifierEndpoint`.
/// Act:     `config`, once per pair.
/// Assert:  each result is `Err` of the earlier step's variant.
/// Boundary: the private call `config` and the chain of real functions the
///   private surface names, all running for real over `SHIPPED_DEFAULTS`.
/// Mocked:  none.
#[test]
fn the_first_refusal_is_returned_in_the_stated_order() {
    // Arrange — the deps and the payload are invariant across every pair.
    let deps = ConfigDeps {
        layers,
        admit_selections,
        admit_statements,
        admit_generate,
        admit_output,
    };
    let payload = ConfigPayload {
        defaults: SHIPPED_DEFAULTS.to_string(),
    };
    let program = OsString::from("harness-crypto");
    let set_flag = OsString::from("--set");
    let set = |assignments: Vec<&str>| ConfigParams {
        arguments: {
            let mut arguments = vec![program.clone()];
            for assignment in assignments {
                arguments.push(set_flag.clone());
                arguments.push(OsString::from(assignment));
            }
            arguments
        },
    };

    // `Layers` before `Selections`.
    let result = config(
        &deps,
        ConfigParams {
            arguments: vec![
                program.clone(),
                OsString::from("--unknown-flag"),
                set_flag.clone(),
                OsString::from(r#"selections.kem="Mock""#),
            ],
        },
        payload.clone(),
    );
    assert!(
        matches!(
            result,
            Err(ConfigErrorReturn::Layers(LayersErrorReturn::Arguments(_)))
        ),
        "the earlier refusal is Layers(Arguments)"
    );

    // `Selections` before `Statements`.
    let result = config(
        &deps,
        set(vec![
            r#"selections.kem="Mock""#,
            r#"network_profile.chain_identifier.kind="Unknown""#,
        ]),
        payload.clone(),
    );
    assert!(
        matches!(
            result,
            Err(ConfigErrorReturn::Selections(
                AdmitSelectionsErrorReturn::Kem { .. }
            ))
        ),
        "the earlier refusal is Selections(Kem)"
    );

    // `Statements` before `Generate`.
    let result = config(
        &deps,
        set(vec![
            r#"network_profile.chain_identifier.kind="Unknown""#,
            r#"generate.concrete="unknown""#,
        ]),
        payload.clone(),
    );
    assert!(
        matches!(
            result,
            Err(ConfigErrorReturn::Statements(
                AdmitStatementsErrorReturn::NetworkProfile {
                    field: NetworkProfileField::ChainIdentifier,
                    refusal: CanonicalValueRefusal::UnknownKind { .. }
                }
            ))
        ),
        "the earlier refusal is Statements(NetworkProfile)"
    );

    // `Generate` before `Output`.
    let result = config(
        &deps,
        set(vec![
            r#"generate.concrete="unknown""#,
            "output.destinations=[]",
        ]),
        payload.clone(),
    );
    assert!(
        matches!(
            result,
            Err(ConfigErrorReturn::Generate(
                AdmitGenerateErrorReturn::Concrete { .. }
            ))
        ),
        "the earlier refusal is Generate(Concrete)"
    );

    // `Output` before `BenchmarkIterations`.
    let result = config(
        &deps,
        set(vec!["output.destinations=[]", "benchmark.iterations=0"]),
        payload.clone(),
    );
    assert!(
        matches!(
            result,
            Err(ConfigErrorReturn::Output(
                AdmitOutputErrorReturn::OutputDestinationsEmpty
            ))
        ),
        "the earlier refusal is Output(OutputDestinationsEmpty)"
    );

    // `BenchmarkIterations` before `MeasurementIterations`.
    let result = config(
        &deps,
        set(vec!["benchmark.iterations=0", "measurement.iterations=0"]),
        payload.clone(),
    );
    assert!(
        matches!(result, Err(ConfigErrorReturn::BenchmarkIterations)),
        "the earlier refusal is BenchmarkIterations"
    );

    // `MeasurementIterations` before `VerifierEndpoint`.
    let result = config(
        &deps,
        set(vec![
            "measurement.iterations=0",
            r#"network_profile.verifier_endpoint="no-scheme""#,
        ]),
        payload.clone(),
    );
    assert!(
        matches!(result, Err(ConfigErrorReturn::MeasurementIterations)),
        "the earlier refusal is MeasurementIterations"
    );
    // the_first_refusal_is_returned_in_the_stated_order
}

// Private-surface enumeration: `the_shipped_defaults_configure_the_harness`,
//   `a_file_and_a_flag_each_replace_a_shipped_default`,
//   `an_unadmissible_value_is_refused_by_the_function_that_admits_it`,
//   `a_value_this_function_admits_is_refused_with_its_own_variant`,
//   `the_verifier_is_selected_with_the_settings_the_document_holds`, and
//   `the_first_refusal_is_returned_in_the_stated_order`, each proven by the
//   block of the same name above.
