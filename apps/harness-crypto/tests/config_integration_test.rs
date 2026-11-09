#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use core::num::NonZeroU32;
use std::env::temp_dir;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;

use chain::{ChainFormsConcrete, DECLARED_CHAIN_FORMS_SELECTIONS};
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, DECLARED_ENCODING_SELECTIONS,
    DECLARED_FIXED_WIDTH_FIELD_KINDS, DECLARED_UNSIGNED_FIELD_KINDS, EncodingConcrete,
};
use envelope::DECLARED_KEY_AGREEMENT_SELECTIONS;
use hash_to_scalar::{
    DECLARED_HASH_TO_SCALAR_SELECTIONS, HashToScalarConcrete, MockIHashToScalarAdapterFailureMode,
};
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::{DECLARED_KEM_SELECTIONS, IdentityScope, KemConcrete};
use pairing::{
    PAIRING_CONCRETES, PAIRING_CURVES, PRECOMPILE_ENCODINGS, PairingCurve,
    TARGET_GROUP_ENCODING_IDENTIFIERS,
};
use proof::DELIVERY_PROOF_CONCRETES;
use random::{MockIRandomSourceAdapterFailureMode, RANDOM_SOURCE_KINDS, RandomSourceKind};

use harness_crypto::{
    AdmitGenerateErrorReturn, AdmitOutputErrorReturn, AdmitSelectionsErrorReturn,
    AdmitStatementsErrorReturn, AdmittedChainForms, AdmittedCurve, AdmittedDeliveryProof,
    AdmittedEncoding, AdmittedHashToScalar, AdmittedIdentities, AdmittedIdentity, AdmittedKem,
    AdmittedKeyAgreement, AdmittedNetworkProfile, AdmittedPairing, AdmittedSelections,
    CanonicalValueRefusal, ConfigDeps, ConfigErrorReturn, ConfigParams, ConfigPayload,
    DESTINATION_KINDS, DestinationKind, EvmVerifierEndpoint, EvmVerifierEndpointConstructorParams,
    EvmVerifierEndpointTryNewErrorReturn, EvmVerifierSettings, GenerateConcrete, LayersErrorReturn,
    MeasurementSettings, NetworkProfileField, OutputDestination, ReportDefaults, ReportDestination,
    ReportSettings, ReportTextConstructorParamsOverrides, ReportTextTryNewErrorReturn,
    SHIPPED_DEFAULTS, SOLIDITY_GENERATE_DECLARATION, VERIFIER_CONCRETES, VectorKindSizing,
    VerifierConcrete, admit_generate, admit_output, admit_selections, admit_statements,
    build_report_text, config, declared_name, layers, representative_canonical_value,
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
///   holds, and the counts and report defaults the default holds, as an
///   outside caller observes it.
/// Arrange: the program name alone over `SHIPPED_DEFAULTS`; the expected
///   selections built member by member from each family's declared set, the
///   expected verifier settings per curve over the default endpoint, that
///   curve's default deployment record, and the oracle, and the expected
///   counts, texts, and destinations.
/// Act:     `config` through the crate root, once.
/// Assert:  the result is `Ok` and each field equals the expected value.
/// Boundary: the public call `config` and the real functions it composes.
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
///   default, as an outside caller observes it.
/// Arrange: a directory beneath the operating system's temporary directory
///   removed before the act; a file and a flag for `benchmark.iterations`,
///   `selections.identity_scope`, and `selections.curves` each in the three
///   interleavings file then flag, flag then file, and file alone.
/// Act:     `config` through the crate root, once per interleaving per key.
/// Assert:  each result is `Ok` and the admitted value equals the value of
///   the last layer that named its key.
/// Boundary: the public call `config` and the real functions it composes.
/// Mocked:  none; the filesystem is a real directory.
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

    // The array `selections.curves`: the file holds one curve the flag's
    // array does not carry, and the flag's array replaces it whole.
    let curves_file = directory.join("curves.toml");
    fs::write(
        &curves_file,
        "[[selections.curves]]\nlabel = \"file-curve\"\npairing_concrete = \"Bn254Halo2curves\"\ntarget_group_encoding = \"Bn254V1\"\n",
    )
    .expect("the curves file is written");
    let curves_path = curves_file.into_os_string();
    let curves_set = OsString::from(
        r#"selections.curves=[{label="flag-curve",pairing_concrete="Bls12381Halo2curves",target_group_encoding="Bls12381V1"}]"#,
    );
    let file_curve = AdmittedCurve {
        label: "file-curve".to_string(),
        pairing: AdmittedPairing {
            concrete: PAIRING_CONCRETES[1],
            target_group_encoding: TARGET_GROUP_ENCODING_IDENTIFIERS[0],
        },
    };
    let flag_curve = AdmittedCurve {
        label: "flag-curve".to_string(),
        pairing: AdmittedPairing {
            concrete: PAIRING_CONCRETES[3],
            target_group_encoding: TARGET_GROUP_ENCODING_IDENTIFIERS[1],
        },
    };
    let curve_cases: Vec<(Vec<OsString>, AdmittedCurve)> = vec![
        (
            vec![
                program.clone(),
                config_flag.clone(),
                curves_path.clone(),
                set_flag.clone(),
                curves_set.clone(),
            ],
            flag_curve.clone(),
        ),
        (
            vec![
                program.clone(),
                set_flag.clone(),
                curves_set.clone(),
                config_flag.clone(),
                curves_path.clone(),
            ],
            file_curve.clone(),
        ),
        (
            vec![program.clone(), config_flag.clone(), curves_path.clone()],
            file_curve.clone(),
        ),
    ];
    for (arguments, expected) in curve_cases {
        // Act
        let configuration = config(&deps, ConfigParams { arguments }, payload.clone())
            .expect("the layers configure")
            .configuration;
        // Assert
        assert_eq!(configuration.selections.curves, vec![expected.clone()]);
        assert_eq!(
            configuration.curve_labels,
            vec![build_report_text(ReportTextConstructorParamsOverrides {
                text: Some(expected.label),
            })]
        );
    }
    // a_file_and_a_flag_each_replace_a_shipped_default
}

/// Contract: own entry `an_unadmissible_value_is_refused_by_the_function_
///   that_admits_it` — `config` over `SHIPPED_DEFAULTS` with one value
///   overridden by a `--set` assignment, every callee real, returns `Err`
///   carrying the callee's refusal whole inside the variant of the function
///   that returned it, as an outside caller observes it.
/// Arrange: for each variation an argument list overriding one value — an
///   unknown flag, a key the document does not declare, the `Debug` text of
///   a family's mock selection in `selections.kem`, an unknown kind in
///   `network_profile.chain_identifier`, an unknown `generate.concrete`,
///   and an empty `output.destinations`.
/// Act:     `config` through the crate root, once per variation.
/// Assert:  each result is `Err` of the stated variant, the callee's refusal
///   carried whole.
/// Boundary: the public call `config` and the real functions it composes.
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
    let set = |assignment: &str| ConfigParams {
        arguments: vec![
            program.clone(),
            set_flag.clone(),
            OsString::from(assignment),
        ],
    };

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
    let result = config(&deps, set("undeclared=1"), payload.clone());
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
        set(&format!("selections.kem=\"{mock_kem}\"")),
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
        set(r#"network_profile.chain_identifier.kind="Unknown""#),
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
        set(r#"generate.concrete="unknown""#),
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
    let result = config(&deps, set("output.destinations=[]"), payload.clone());
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
///   the variant that names the document path, as an outside caller
///   observes it.
/// Arrange: for each variation an argument list overriding one value — a
///   zero for each count, a text of whitespace alone for each text, a curve
///   label of whitespace alone at index one, and an endpoint text no URL
///   parser admits.
/// Act:     `config` through the crate root, once per variation.
/// Assert:  each result is `Err` of the variant that names the document
///   path, any carried error the callee's own.
/// Boundary: the public call `config` and the real functions it composes.
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

    // A curve label of whitespace alone is refused at index one.
    let result = config(
        &deps,
        set(
            r#"selections.curves=[{label="ok-curve",pairing_concrete="Bn254Arkworks",target_group_encoding="Bn254V1"},{label=" ",pairing_concrete="Bls12381Arkworks",target_group_encoding="Bls12381V1"}]"#,
        ),
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
///   Arrange from the overridden values, as an outside caller observes it.
/// Arrange: each value overridden alone, the two records overridden
///   together, and the layer of a single curve's record, which leaves the
///   other curve's shipped default; the expected settings built in Arrange
///   from the overridden values.
/// Act:     `config` through the crate root, once per variation.
/// Assert:  each result is `Ok` and its `verifiers` equals the expected
///   `VerifierConcrete::Evm` in curve order.
/// Boundary: the public call `config` and the real functions it composes.
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
///   steps, every callee real, returns the earlier step's refusal, as an
///   outside caller observes it.
/// Arrange: for each pair an argument list holding a refused value at both
///   steps — `Layers` before `Selections`, `Selections` before `Statements`,
///   `Statements` before `Generate`, `Generate` before `Output`, `Output`
///   before `BenchmarkIterations`, `BenchmarkIterations` before
///   `MeasurementIterations`, and `MeasurementIterations` before
///   `VerifierEndpoint`.
/// Act:     `config` through the crate root, once per pair.
/// Assert:  each result is `Err` of the earlier step's variant.
/// Boundary: the public call `config` and the real functions it composes.
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

/// Contract: carried entry `layered_sources_merge_in_command_line_order` —
///   `config` hands the command line to `layers` unchanged and every
///   admission reads the merged document, so each admitted value is the
///   value of the last layer that named its key, as an outside caller
///   observes it.
/// Arrange: a directory beneath the operating system's temporary directory
///   removed before the act; two files and two assignments over
///   `benchmark.iterations`, `measurement.conditions`, and
///   `identities.mapping`, named in the three interleavings file then
///   assignment, assignment then file, and two files.
/// Act:     `config` through the crate root, once per interleaving.
/// Assert:  each result is `Ok`; for each key the configuration holds the
///   value of the last layer that named it, and every key no layer named
///   holds the shipped default.
/// Boundary: the public call `config`; the route `layers`, then
///   `admit_selections`, `admit_statements`, `admit_generate`, and
///   `admit_output` over the merged document, then `config`.
/// Mocked:  none; the filesystem is a real directory.
#[test]
fn layered_sources_merge_in_command_line_order() {
    // Arrange
    let directory: PathBuf = temp_dir().join(format!(
        "chaintorrent-config-{}-{:?}",
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
    let command_lines: Vec<Vec<OsString>> = vec![
        vec![
            OsString::from("harness-crypto"),
            OsString::from("--config"),
            first_argument.clone(),
            OsString::from("--set"),
            scalar_assignment.clone(),
            OsString::from("--config"),
            second_argument.clone(),
            OsString::from("--set"),
            array_assignment.clone(),
        ],
        vec![
            OsString::from("harness-crypto"),
            OsString::from("--set"),
            scalar_assignment.clone(),
            OsString::from("--config"),
            first_argument.clone(),
            OsString::from("--set"),
            array_assignment.clone(),
            OsString::from("--config"),
            second_argument.clone(),
        ],
        vec![
            OsString::from("harness-crypto"),
            OsString::from("--config"),
            first_argument.clone(),
            OsString::from("--config"),
            second_argument.clone(),
            OsString::from("--set"),
            scalar_assignment.clone(),
            OsString::from("--set"),
            array_assignment.clone(),
        ],
    ];
    let expected_iterations = [4u32, 3, 4];
    let expected_conditions = ["conditions-b", "conditions-b", "conditions-b"];
    let expected_mappings: Vec<Vec<Vec<u8>>> = vec![
        vec![vec![0x44, 0x44]],
        vec![vec![0x22, 0x22], vec![0x33, 0x33]],
        vec![vec![0x44, 0x44]],
    ];
    // Act / Assert
    for (index, arguments) in command_lines.into_iter().enumerate() {
        let configuration = config(&deps, ConfigParams { arguments }, payload.clone())
            .expect("the merged document configures")
            .configuration;
        assert_eq!(
            configuration.benchmark_iterations.get(),
            expected_iterations[index]
        );
        assert_eq!(
            configuration.measurement.conditions,
            build_report_text(ReportTextConstructorParamsOverrides {
                text: Some(expected_conditions[index].to_string()),
            })
        );
        let expected: Vec<AdmittedIdentity> = expected_mappings[index]
            .iter()
            .map(|bytes| AdmittedIdentity::Entitlement {
                canonical: bytes.clone(),
            })
            .collect();
        assert_eq!(configuration.identities.mapping, expected);
    }
    // layered_sources_merge_in_command_line_order
}

/// Contract: carried entry `every_member_of_every_declared_set_is_admitted_
///   by_its_own_name` — the admitted selections reach the configuration
///   unchanged and a refusal reaches the caller inside
///   `ConfigErrorReturn::Selections`, so `config` over `SHIPPED_DEFAULTS`
///   with one family member's `Debug` text named through `--set` returns
///   `Ok` whose `selections` holds that member with the identifier, scope,
///   and algebra it carries, as an outside caller observes it.
/// Arrange: for each member of each family's declared set a command line
///   naming the member through `--set` — the KEM and key agreement members
///   carrying their scope and algebra, every member of `PAIRING_CONCRETES`
///   crossed with every member of `TARGET_GROUP_ENCODING_IDENTIFIERS` as a
///   curve and as a benchmark concrete, and every member of
///   `PRECOMPILE_ENCODINGS`.
/// Act:     `config` through the crate root, once per member.
/// Assert:  the result is `Ok` and the `selections` field for that family
///   equals the expected value built from the member.
/// Boundary: the public call `config`; the route `layers`,
///   `admit_selections`, `member_named`, then `config`.
/// Mocked:  none.
#[test]
fn every_member_of_every_declared_set_is_admitted_by_its_own_name() {
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
    let set = |assignments: Vec<String>| ConfigParams {
        arguments: {
            let mut arguments = vec![program.clone()];
            for assignment in assignments {
                arguments.push(set_flag.clone());
                arguments.push(OsString::from(assignment));
            }
            arguments
        },
    };

    // Act / Assert — every member of `RANDOM_SOURCE_KINDS` in turn.
    for member in RANDOM_SOURCE_KINDS {
        let configuration = config(
            &deps,
            set(vec![format!(
                "selections.random_source=\"{}\"",
                declared_name(member)
            )]),
            payload.clone(),
        )
        .expect("the member is admitted")
        .configuration;
        assert_eq!(configuration.selections.random_source, *member);
    }

    // Act / Assert — every member of `DECLARED_ENCODING_SELECTIONS` in turn.
    for member in DECLARED_ENCODING_SELECTIONS {
        let configuration = config(
            &deps,
            set(vec![format!(
                "selections.encoding=\"{}\"",
                declared_name(&member.concrete)
            )]),
            payload.clone(),
        )
        .expect("the member is admitted")
        .configuration;
        assert_eq!(
            configuration.selections.encoding,
            AdmittedEncoding {
                concrete: member.concrete,
                identifier: member.identifier,
            }
        );
    }

    // Act / Assert — every member of `DECLARED_HASH_TO_SCALAR_SELECTIONS`.
    for member in DECLARED_HASH_TO_SCALAR_SELECTIONS {
        let configuration = config(
            &deps,
            set(vec![format!(
                "selections.hash_to_scalar=\"{}\"",
                declared_name(&member.concrete)
            )]),
            payload.clone(),
        )
        .expect("the member is admitted")
        .configuration;
        assert_eq!(
            configuration.selections.hash_to_scalar,
            AdmittedHashToScalar {
                concrete: member.concrete,
                identifier: member.identifier,
            }
        );
    }

    // Act / Assert — every member of `DECLARED_CHAIN_FORMS_SELECTIONS`.
    for member in DECLARED_CHAIN_FORMS_SELECTIONS {
        let configuration = config(
            &deps,
            set(vec![format!(
                "selections.chain_forms=\"{}\"",
                declared_name(&member.concrete)
            )]),
            payload.clone(),
        )
        .expect("the member is admitted")
        .configuration;
        assert_eq!(
            configuration.selections.chain_forms,
            AdmittedChainForms {
                concrete: member.concrete,
                identifier: member.identifier,
            }
        );
    }

    // Act / Assert — every member of `VERIFIER_CONCRETES` in turn.
    for member in VERIFIER_CONCRETES {
        let configuration = config(
            &deps,
            set(vec![format!(
                "selections.verifier=\"{}\"",
                declared_name(member)
            )]),
            payload.clone(),
        )
        .expect("the member is admitted")
        .configuration;
        assert_eq!(configuration.selections.verifier, *member);
    }

    // Act / Assert — every member of `DECLARED_KEM_SELECTIONS` in turn, its
    // concrete named and its scope the admitted scope.
    for member in DECLARED_KEM_SELECTIONS {
        let configuration = config(
            &deps,
            set(vec![
                format!("selections.kem=\"{}\"", declared_name(&member.concrete)),
                format!(
                    "selections.identity_scope=\"{}\"",
                    declared_name(&member.scope)
                ),
            ]),
            payload.clone(),
        )
        .expect("the member is admitted")
        .configuration;
        assert_eq!(
            configuration.selections.kem,
            AdmittedKem {
                concrete: member.concrete,
                identifier: member.identifier,
                scope: member.scope,
            }
        );
    }

    // Act / Assert — every member of `DECLARED_KEY_AGREEMENT_SELECTIONS` in
    // turn, its concrete named and its algebra the admitted algebra.
    for member in DECLARED_KEY_AGREEMENT_SELECTIONS {
        let configuration = config(
            &deps,
            set(vec![
                format!(
                    "selections.key_agreement=\"{}\"",
                    declared_name(&member.concrete)
                ),
                format!(
                    "selections.envelope_algebra=\"{}\"",
                    declared_name(&member.algebra)
                ),
            ]),
            payload.clone(),
        )
        .expect("the member is admitted")
        .configuration;
        assert_eq!(
            configuration.selections.key_agreement,
            AdmittedKeyAgreement {
                concrete: member.concrete,
                identifier: member.identifier,
                algebra: member.algebra,
            }
        );
    }

    // Act / Assert — every member of `DELIVERY_PROOF_CONCRETES` in turn.
    for member in DELIVERY_PROOF_CONCRETES {
        let configuration = config(
            &deps,
            set(vec![format!(
                "selections.delivery_proof=\"{}\"",
                declared_name(member)
            )]),
            payload.clone(),
        )
        .expect("the member is admitted")
        .configuration;
        assert_eq!(
            configuration.selections.delivery_proof,
            AdmittedDeliveryProof {
                concrete: *member,
                algebra: DECLARED_KEY_AGREEMENT_SELECTIONS[0].algebra,
                statement_version: 1,
            }
        );
    }

    // Act / Assert — every `PAIRING_CONCRETES` member crossed with every
    // `TARGET_GROUP_ENCODING_IDENTIFIERS` member, as a curve and as a
    // benchmark concrete.
    for concrete in PAIRING_CONCRETES {
        for target_group_encoding in TARGET_GROUP_ENCODING_IDENTIFIERS {
            let configuration = config(
                &deps,
                set(vec![
                    format!(
                        "selections.curves=[{{label=\"curve-x\",pairing_concrete=\"{}\",target_group_encoding=\"{}\"}}]",
                        declared_name(concrete),
                        declared_name(target_group_encoding)
                    ),
                    format!(
                        "selections.benchmark_concretes=[{{pairing_concrete=\"{}\",target_group_encoding=\"{}\"}}]",
                        declared_name(concrete),
                        declared_name(target_group_encoding)
                    ),
                ]),
                payload.clone(),
            )
            .expect("the member pair is admitted")
            .configuration;
            let expected_pairing = AdmittedPairing {
                concrete: *concrete,
                target_group_encoding: *target_group_encoding,
            };
            assert_eq!(
                configuration.selections.curves,
                vec![AdmittedCurve {
                    label: "curve-x".to_string(),
                    pairing: expected_pairing,
                }]
            );
            assert_eq!(
                configuration.selections.benchmark_concretes,
                vec![expected_pairing]
            );
        }
    }

    // Act / Assert — every member of `PRECOMPILE_ENCODINGS` in turn.
    for member in PRECOMPILE_ENCODINGS {
        let configuration = config(
            &deps,
            set(vec![format!(
                "network_profile.precompile_encodings=[\"{}\"]",
                declared_name(member)
            )]),
            payload.clone(),
        )
        .expect("the member is admitted")
        .configuration;
        assert_eq!(configuration.selections.precompile_encodings, vec![*member]);
    }
    // every_member_of_every_declared_set_is_admitted_by_its_own_name
}

/// Contract: carried entry `a_mock_selections_name_is_never_admitted` — a
///   refusal reaches the caller inside `ConfigErrorReturn::Selections`, so
///   `config` over `SHIPPED_DEFAULTS` with the `Debug` text of a family's
///   mock selection named through `--set` returns `Err` of the `Selections`
///   variant carrying the refused family's variant and that text, as an
///   outside caller observes it.
/// Arrange: for each of `random_source`, `encoding`, `chain_forms`, `kem`,
///   and `hash_to_scalar` a command line whose `--set` text for that family
///   is `declared_name` of the family's mock selection.
/// Act:     `config` through the crate root, once per family.
/// Assert:  each result is `Err` of `Selections` carrying the refused
///   family's variant and the text.
/// Boundary: the public call `config`; the route `layers`,
///   `admit_selections`, `member_named`, then `config`.
/// Mocked:  none.
#[test]
fn a_mock_selections_name_is_never_admitted() {
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
    let cases: Vec<(String, String)> = vec![
        (
            "random_source".to_string(),
            declared_name(&RandomSourceKind::Mock(
                MockIRandomSourceAdapterFailureMode::Succeeds,
            )),
        ),
        (
            "encoding".to_string(),
            declared_name(&EncodingConcrete::Mock),
        ),
        (
            "chain_forms".to_string(),
            declared_name(&ChainFormsConcrete::Mock),
        ),
        ("kem".to_string(), declared_name(&KemConcrete::Mock)),
        (
            "hash_to_scalar".to_string(),
            declared_name(&HashToScalarConcrete::Mock(
                MockIHashToScalarAdapterFailureMode::Succeeds,
            )),
        ),
    ];
    for (field, text) in cases {
        // Act
        let result = config(
            &deps,
            ConfigParams {
                arguments: vec![
                    program.clone(),
                    set_flag.clone(),
                    OsString::from(format!("selections.{field}=\"{text}\"")),
                ],
            },
            payload.clone(),
        );
        // Assert — the `Selections` variant of that family carrying the text.
        let refused = match field.as_str() {
            "random_source" => {
                matches!(
                    result,
                    Err(ConfigErrorReturn::Selections(
                        AdmitSelectionsErrorReturn::RandomSource { text: ref t }
                    )) if t == &text
                )
            }
            "encoding" => {
                matches!(
                    result,
                    Err(ConfigErrorReturn::Selections(
                        AdmitSelectionsErrorReturn::Encoding { text: ref t }
                    )) if t == &text
                )
            }
            "chain_forms" => {
                matches!(
                    result,
                    Err(ConfigErrorReturn::Selections(
                        AdmitSelectionsErrorReturn::ChainForms { text: ref t }
                    )) if t == &text
                )
            }
            "kem" => {
                matches!(
                    result,
                    Err(ConfigErrorReturn::Selections(
                        AdmitSelectionsErrorReturn::Kem { text: ref t }
                    )) if t == &text
                )
            }
            _ => {
                matches!(
                    result,
                    Err(ConfigErrorReturn::Selections(
                        AdmitSelectionsErrorReturn::HashToScalar { text: ref t }
                    )) if t == &text
                )
            }
        };
        assert!(refused, "the mock name is refused inside Selections");
    }
    // a_mock_selections_name_is_never_admitted
}

/// Contract: carried entry `every_declared_kind_admits_its_value_text_to_
///   its_own_variant` — the admitted profile and statements reach the
///   configuration unchanged, so `config` over `SHIPPED_DEFAULTS` with a
///   canonical value's `kind` named through `--set` and a value text the
///   kind admits returns `Ok` whose `network_profile.chain_identifier` and
///   `statements.mint.source_entitlement` equal the variant of that kind,
///   as an outside caller observes it.
/// Arrange: for each member of `DECLARED_FIXED_WIDTH_FIELD_KINDS` and of
///   `DECLARED_UNSIGNED_FIELD_KINDS` a command line naming the member by
///   `declared_name` in `network_profile.chain_identifier` with the value
///   text `representative_canonical_value` returns, then the same in the
///   mint context's `source_entitlement`.
/// Act:     `config` through the crate root, once per member per field.
/// Assert:  each result is `Ok` and the admitted value equals the expected
///   variant.
/// Boundary: the public call `config`; the route `layers`,
///   `admit_selections`, `admit_statements`, its private functions, then
///   `config`.
/// Mocked:  none.
#[test]
fn every_declared_kind_admits_its_value_text_to_its_own_variant() {
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
    let set = |assignments: Vec<String>| ConfigParams {
        arguments: {
            let mut arguments = vec![program.clone()];
            for assignment in assignments {
                arguments.push(set_flag.clone());
                arguments.push(OsString::from(assignment));
            }
            arguments
        },
    };
    // Act / Assert — every member of both declared sets, in
    // `chain_identifier` and in `source_entitlement`.
    for kind in DECLARED_FIXED_WIDTH_FIELD_KINDS
        .iter()
        .chain(DECLARED_UNSIGNED_FIELD_KINDS)
    {
        let (text, variant) =
            representative_canonical_value(*kind).expect("the declared kind admits a value");
        let name = declared_name(kind);
        let configuration = config(
            &deps,
            set(vec![
                format!("network_profile.chain_identifier.kind=\"{name}\""),
                format!("network_profile.chain_identifier.value=\"{text}\""),
            ]),
            payload.clone(),
        )
        .expect("the member is admitted")
        .configuration;
        assert_eq!(configuration.network_profile.chain_identifier, variant);
        let configuration = config(
            &deps,
            set(vec![
                format!("statements.mint.source_entitlement.kind=\"{name}\""),
                format!("statements.mint.source_entitlement.value=\"{text}\""),
            ]),
            payload.clone(),
        )
        .expect("the member is admitted")
        .configuration;
        assert_eq!(configuration.statements.mint.source_entitlement, variant);
    }
    // every_declared_kind_admits_its_value_text_to_its_own_variant
}

/// Contract: carried entry `a_kind_outside_the_declared_sets_is_never_
///   admitted` — a refusal reaches the caller inside
///   `ConfigErrorReturn::Statements`, so `config` over `SHIPPED_DEFAULTS`
///   with a `kind` text no declared set holds named through `--set` returns
///   `Err` of `Statements` carrying `UnknownKind` and the text, as an
///   outside caller observes it.
/// Arrange: for each member of `CanonicalFieldKind` neither declared set
///   holds — `Text` and `Bytes` — a command line naming the member by
///   `declared_name` in `network_profile.chain_identifier`.
/// Act:     `config` through the crate root, once per member.
/// Assert:  each result is `Err` of `Statements(NetworkProfile)` carrying
///   `ChainIdentifier` and `UnknownKind` of the text.
/// Boundary: the public call `config`; the route `layers`,
///   `admit_selections`, `admit_statements`, its private functions, then
///   `config`.
/// Mocked:  none.
#[test]
fn a_kind_outside_the_declared_sets_is_never_admitted() {
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
    for kind in [CanonicalFieldKind::Text, CanonicalFieldKind::Bytes] {
        let text = declared_name(&kind);
        // Act
        let result = config(
            &deps,
            ConfigParams {
                arguments: vec![
                    program.clone(),
                    set_flag.clone(),
                    OsString::from(format!("network_profile.chain_identifier.kind=\"{text}\"")),
                ],
            },
            payload.clone(),
        );
        // Assert
        assert!(
            matches!(
                result,
                Err(ConfigErrorReturn::Statements(
                    AdmitStatementsErrorReturn::NetworkProfile {
                        field: NetworkProfileField::ChainIdentifier,
                        refusal: CanonicalValueRefusal::UnknownKind { text: ref t }
                    }
                )) if t == &text
            ),
            "the kind outside the declared sets is refused inside Statements"
        );
    }
    // a_kind_outside_the_declared_sets_is_never_admitted
}

/// Contract: carried entry `every_context_carries_the_network_profiles_
///   chain_identifier_and_entitlement_contract` — the admitted profile,
///   statements, and identities reach the configuration unchanged, so
///   `config` over `SHIPPED_DEFAULTS` with `network_profile.chain_identifier`
///   and `entitlement_contract` overridden through `--set` returns `Ok`
///   whose mint and transfer contexts carry those admitted values, as an
///   outside caller observes it.
/// Arrange: a command line overriding the two network profile values with
///   texts `representative_canonical_value` returns for `Unsigned256` and
///   `FixedBytes20`.
/// Act:     `config` through the crate root, once.
/// Assert:  each context's `chain` and `entitlement_contract` equal the
///   expected variants.
/// Boundary: the public call `config`; the route `layers`,
///   `admit_selections`, `admit_statements`, its private functions, then
///   `config`.
/// Mocked:  none.
#[test]
fn every_context_carries_the_network_profiles_chain_identifier_and_entitlement_contract() {
    // Arrange
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
    let (chain_text, chain_variant) =
        representative_canonical_value(CanonicalFieldKind::Unsigned256)
            .expect("the declared kind admits a value");
    let (contract_text, contract_variant) =
        representative_canonical_value(CanonicalFieldKind::FixedBytes20)
            .expect("the declared kind admits a value");
    let params = ConfigParams {
        arguments: vec![
            OsString::from("harness-crypto"),
            OsString::from("--set"),
            OsString::from(format!(
                "network_profile.chain_identifier.value=\"{chain_text}\""
            )),
            OsString::from("--set"),
            OsString::from(format!(
                "network_profile.entitlement_contract.value=\"{contract_text}\""
            )),
        ],
    };
    // Act
    let configuration = config(&deps, params, payload)
        .expect("the profile is admitted")
        .configuration;
    // Assert
    assert_eq!(configuration.statements.mint.chain, chain_variant);
    assert_eq!(
        configuration.statements.mint.entitlement_contract,
        contract_variant
    );
    assert_eq!(configuration.statements.transfer.chain, chain_variant);
    assert_eq!(
        configuration.statements.transfer.entitlement_contract,
        contract_variant
    );
    // every_context_carries_the_network_profiles_chain_identifier_and_entitlement_contract
}

/// Contract: carried entry `identities_are_admitted_under_the_scope_the_
///   payload_names` — the scope `admit_statements` is handed is the one
///   `admit_selections` admitted, so `config` over `SHIPPED_DEFAULTS` with
///   `selections.identity_scope` overridden through `--set` returns `Ok`
///   whose `identities` hold each shipped identity under that scope, as an
///   outside caller observes it.
/// Arrange: a command line naming `Asset` in
///   `selections.identity_scope`.
/// Act:     `config` through the crate root, once.
/// Assert:  `identities.proof` and each `identities.mapping` entry are
///   `Asset` roots over the shipped texts' bytes.
/// Boundary: the public call `config`; the route `layers`,
///   `admit_selections`, `admit_statements`, its private functions, then
///   `config`.
/// Mocked:  none.
#[test]
fn identities_are_admitted_under_the_scope_the_payload_names() {
    // Arrange
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
    let params = ConfigParams {
        arguments: vec![
            OsString::from("harness-crypto"),
            OsString::from("--set"),
            OsString::from("selections.identity_scope=\"Asset\""),
        ],
    };
    let expected_proof = AdmittedIdentity::Asset {
        root: build_root(RootConstructorParamsOverrides {
            bytes: Some([0x55; 32]),
        }),
    };
    let expected_mapping = vec![
        AdmittedIdentity::Asset {
            root: build_root(RootConstructorParamsOverrides {
                bytes: Some([0x66; 32]),
            }),
        },
        AdmittedIdentity::Asset {
            root: build_root(RootConstructorParamsOverrides {
                bytes: Some([0x77; 32]),
            }),
        },
    ];
    // Act
    let configuration = config(&deps, params, payload)
        .expect("the scope is admitted")
        .configuration;
    // Assert
    assert_eq!(configuration.selections.kem.scope, IdentityScope::Asset);
    assert_eq!(configuration.identities.proof, expected_proof);
    assert_eq!(configuration.identities.mapping, expected_mapping);
    // identities_are_admitted_under_the_scope_the_payload_names
}

/// Contract: carried entry `every_curve_has_its_own_deployment_record_in_
///   curve_order` — each curve's record reaches `verifiers` at that curve's
///   index, so `config` over `SHIPPED_DEFAULTS` with
///   `network_profile.deployment_records.bn254` and
///   `network_profile.deployment_records.bls12_381` overridden to distinct
///   records returns `Ok` whose `verifiers` holds the first curve's
///   concrete at the bn254 record and the second's at the bls12_381 record,
///   as an outside caller observes it.
/// Arrange: the two records overridden to distinct values through `--set`.
/// Act:     `config` through the crate root, once.
/// Assert:  the result is `Ok` and `verifiers` holds each curve's concrete
///   at its own record in curve order.
/// Boundary: the public call `config`; the route `layers`,
///   `admit_selections`, `admit_statements`, its private functions, then
///   `config`.
/// Mocked:  none.
#[test]
fn every_curve_has_its_own_deployment_record_in_curve_order() {
    // Arrange
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
    // Act
    let configuration = config(
        &deps,
        ConfigParams {
            arguments: vec![
                OsString::from("harness-crypto"),
                OsString::from("--set"),
                OsString::from(
                    r#"network_profile.deployment_records.bn254.value="0x1111111111111111111111111111111111111111""#,
                ),
                OsString::from("--set"),
                OsString::from(
                    r#"network_profile.deployment_records.bls12_381.value="0x2222222222222222222222222222222222222222""#,
                ),
            ],
        },
        payload,
    )
    .expect("the records are admitted")
    .configuration;
    // Assert
    let expected: Vec<CanonicalFieldValue> = vec![
        CanonicalFieldValue::FixedBytes20([0x11; 20]),
        CanonicalFieldValue::FixedBytes20([0x22; 20]),
    ];
    assert_eq!(configuration.verifiers.len(), expected.len());
    for (concrete, expected_address) in configuration.verifiers.iter().zip(expected.iter()) {
        assert!(
            matches!(
                concrete,
                VerifierConcrete::Evm { settings }
                    if settings.verifier_address == *expected_address
            ),
            "each curve's concrete is Evm at its own record"
        );
    }
    // every_curve_has_its_own_deployment_record_in_curve_order
}

/// Contract: carried entry `a_curve_without_a_record_and_a_record_without_
///   a_curve_are_each_refused_with_the_label` — a refusal reaches the
///   caller inside `ConfigErrorReturn::Statements`, so `config` over
///   `SHIPPED_DEFAULTS` with a curve whose record is absent returns `Err`
///   of `MissingDeploymentRecord` carrying that curve's label, and with a
///   record no curve names returns `Err` of `UndeclaredDeploymentRecord`
///   carrying that key, as an outside caller observes it.
/// Arrange: a `--set` adding a third curve no record names; a `--set`
///   adding a record keyed by a label no curve names.
/// Act:     `config` through the crate root, once per variation.
/// Assert:  each result is `Err` of `Statements` carrying the refused
///   variant and the label.
/// Boundary: the public call `config`; the route `layers`,
///   `admit_selections`, `admit_statements`, its private functions, then
///   `config`.
/// Mocked:  none.
#[test]
fn a_curve_without_a_record_and_a_record_without_a_curve_are_each_refused_with_the_label() {
    // Arrange — the deps, the payload, and the flags are invariant.
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

    // A curve whose record is absent: three curves over the shipped text's
    // two records.
    let result = config(
        &deps,
        ConfigParams {
            arguments: vec![
                program.clone(),
                set_flag.clone(),
                OsString::from(
                    r#"selections.curves=[{label="bn254",pairing_concrete="Bn254Arkworks",target_group_encoding="Bn254V1"},{label="bls12_381",pairing_concrete="Bls12381Arkworks",target_group_encoding="Bls12381V1"},{label="extra",pairing_concrete="Bn254Arkworks",target_group_encoding="Bn254V1"}]"#,
                ),
            ],
        },
        payload.clone(),
    );
    assert!(
        matches!(
            result,
            Err(ConfigErrorReturn::Statements(
                AdmitStatementsErrorReturn::MissingDeploymentRecord { ref label }
            )) if label == "extra"
        ),
        "the refusal is MissingDeploymentRecord carrying the curve's label"
    );

    // A record keyed by a label no curve names.
    let result = config(
        &deps,
        ConfigParams {
            arguments: vec![
                program.clone(),
                set_flag.clone(),
                OsString::from(
                    r#"network_profile.deployment_records.extra={kind="FixedBytes20",value="0x1111111111111111111111111111111111111111"}"#,
                ),
            ],
        },
        payload.clone(),
    );
    assert!(
        matches!(
            result,
            Err(ConfigErrorReturn::Statements(
                AdmitStatementsErrorReturn::UndeclaredDeploymentRecord { ref label }
            )) if label == "extra"
        ),
        "the refusal is UndeclaredDeploymentRecord carrying the key"
    );
    // a_curve_without_a_record_and_a_record_without_a_curve_are_each_refused_with_the_label
}

/// Contract: carried entry `every_declared_kind_is_admitted_to_its_own_
///   settings_and_configuration` — the admitted generate settings reach
///   the configuration unchanged, so `config` over `SHIPPED_DEFAULTS`
///   returns `Ok` whose `generate.kinds` holds one `VectorKindConfiguration`
///   per kind of `SOLIDITY_GENERATE_DECLARATION` and whose `Solidity`
///   settings hold each declared kind, as an outside caller observes it.
/// Arrange: the shipped defaults alone, which declare every kind.
/// Act:     `config` through the crate root, once.
/// Assert:  each declared kind is present in `generate.kinds` with the
///   quantities its sizing declares — `count` `16` for each kind of sizing
///   `Drawn`, `DrawnTerms`, and `DrawnMessages`, `terms` `4` for
///   `G1MultiScalarMultiplication` and `G2MultiScalarMultiplication`,
///   `message_length` `32` for `HashToScalar`, no quantity for
///   `DecodeRejection` and `IdentityMapping` — and in the `Solidity`
///   settings' `kinds`.
/// Boundary: the public call `config`; the route `layers`,
///   `admit_generate`, then `config`.
/// Mocked:  none.
#[test]
fn every_declared_kind_is_admitted_to_its_own_settings_and_configuration() {
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
    // Act
    let configuration = config(&deps, params, payload)
        .expect("the shipped defaults configure")
        .configuration;
    // Assert — every declared kind admitted to its own configuration and
    // settings entry.
    let GenerateConcrete::Solidity { settings } = configuration.generate.concrete else {
        panic!("the concrete is Solidity")
    };
    for declared in SOLIDITY_GENERATE_DECLARATION.kinds {
        let configured = configuration
            .generate
            .kinds
            .iter()
            .find(|configured| configured.kind == declared.kind)
            .expect("every declared kind is configured");
        match declared.sizing {
            VectorKindSizing::Drawn => {
                assert_eq!(configured.quantities.count.expect("a count").get(), 16);
                assert!(configured.quantities.terms.is_none());
                assert!(configured.quantities.message_length.is_none());
            }
            VectorKindSizing::DrawnTerms => {
                assert_eq!(configured.quantities.count.expect("a count").get(), 16);
                assert_eq!(configured.quantities.terms.expect("terms").get(), 4);
                assert!(configured.quantities.message_length.is_none());
            }
            VectorKindSizing::DrawnMessages => {
                assert_eq!(configured.quantities.count.expect("a count").get(), 16);
                assert!(configured.quantities.terms.is_none());
                assert_eq!(configured.quantities.message_length, Some(32));
            }
            VectorKindSizing::Enumerated | VectorKindSizing::OverConfiguredInputs => {
                assert!(configured.quantities.count.is_none());
                assert!(configured.quantities.terms.is_none());
                assert!(configured.quantities.message_length.is_none());
            }
        }
        assert!(
            settings
                .kinds
                .iter()
                .any(|entry| entry.kind == declared.kind),
            "every declared kind reaches the Solidity settings"
        );
    }
    // every_declared_kind_is_admitted_to_its_own_settings_and_configuration
}

/// Contract: carried entry `every_declared_curve_directory_reaches_the_
///   field_its_curve_names` — the admitted generate settings reach the
///   configuration unchanged, so `config` over `SHIPPED_DEFAULTS` with one
///   member's key of `generate.curve_directories` overridden through
///   `--set` returns `Ok` whose `Solidity` settings hold the override in the
///   field that member names and the shipped default in the other, as an
///   outside caller observes it.
/// Arrange: for each member of `PAIRING_CURVES` a command line whose
///   `--set` names the member's `Debug` text with the directory
///   `"directory-override"`.
/// Act:     `config` through the crate root, once per member.
/// Assert:  the `curve_directories` field the member names holds the
///   override and the other field holds the shipped default.
/// Boundary: the public call `config`; the route `layers`,
///   `admit_generate`, then `config`.
/// Mocked:  none.
#[test]
fn every_declared_curve_directory_reaches_the_field_its_curve_names() {
    // Arrange — the deps and the payload are invariant across every member.
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
    // Act / Assert — every member of `PAIRING_CURVES` in turn, its directory
    // overridden and the other member's the shipped default.
    for member in PAIRING_CURVES {
        let result = config(
            &deps,
            ConfigParams {
                arguments: vec![
                    program.clone(),
                    set_flag.clone(),
                    OsString::from(format!(
                        "generate.curve_directories.{}=\"directory-override\"",
                        declared_name(member)
                    )),
                ],
            },
            payload.clone(),
        );
        let GenerateConcrete::Solidity { settings } = result
            .expect("the curve directory is admitted")
            .configuration
            .generate
            .concrete
        else {
            panic!("the concrete is Solidity")
        };
        let (overridden, other) = match member {
            PairingCurve::Bn254 => (
                settings.curve_directories.bn254.as_str(),
                settings.curve_directories.bls12_381.as_str(),
            ),
            PairingCurve::Bls12381 => (
                settings.curve_directories.bls12_381.as_str(),
                settings.curve_directories.bn254.as_str(),
            ),
            other => panic!("{other:?} names no curve directory field"),
        };
        let default_other = match member {
            PairingCurve::Bn254 => "bls12_381",
            PairingCurve::Bls12381 => "bn254",
            _ => "",
        };
        assert_eq!(overridden, "directory-override");
        assert_eq!(other, default_other);
    }
    // every_declared_curve_directory_reaches_the_field_its_curve_names
}

/// Contract: carried entry `an_undeclared_name_is_never_admitted` — a
///   refusal reaches the caller inside `ConfigErrorReturn::Generate`, so
///   `config` over `SHIPPED_DEFAULTS` with a name the generate family does
///   not declare named through `--set` returns `Err` of `Generate` carrying
///   the refused variant and the text, as an outside caller observes it.
/// Arrange: a command line naming an undeclared `generate.concrete`, an
///   undeclared `generate.kinds` key, and an undeclared
///   `generate.curve_directories` key.
/// Act:     `config` through the crate root, once per case.
/// Assert:  each result is `Err` of `Generate` carrying the refused
///   variant's text.
/// Boundary: the public call `config`; the route `layers`,
///   `admit_generate`, then `config`.
/// Mocked:  none.
#[test]
fn an_undeclared_name_is_never_admitted() {
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
    let set = |assignments: Vec<String>| ConfigParams {
        arguments: {
            let mut arguments = vec![program.clone()];
            for assignment in assignments {
                arguments.push(set_flag.clone());
                arguments.push(OsString::from(assignment));
            }
            arguments
        },
    };

    // An undeclared `concrete` name is refused.
    let result = config(
        &deps,
        set(vec!["generate.concrete=\"unknown\"".to_string()]),
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

    // An undeclared `kinds` key is refused.
    let result = config(
        &deps,
        set(vec![
            "generate.kinds.NotAKind.file=\"x.txt\"".to_string(),
            "generate.kinds.NotAKind.struct_name=\"NotAKindVector\"".to_string(),
            "generate.kinds.NotAKind.decode_function=\"decodeNotAKind\"".to_string(),
            "generate.kinds.NotAKind.path_constant=\"NOT_A_KIND_PATH\"".to_string(),
        ]),
        payload.clone(),
    );
    assert!(
        matches!(
            result,
            Err(ConfigErrorReturn::Generate(
                AdmitGenerateErrorReturn::Kind { ref text }
            )) if text == "NotAKind"
        ),
        "the refusal is Generate(Kind) carrying the text"
    );

    // An undeclared `curve_directories` key is refused.
    let result = config(
        &deps,
        set(vec![
            "generate.curve_directories.NotACurve=\"x\"".to_string(),
        ]),
        payload.clone(),
    );
    assert!(
        matches!(
            result,
            Err(ConfigErrorReturn::Generate(
                AdmitGenerateErrorReturn::CurveKey { ref text }
            )) if text == "NotACurve"
        ),
        "the refusal is Generate(CurveKey) carrying the text"
    );
    // an_undeclared_name_is_never_admitted
}

/// Contract: carried entry `every_declared_destination_kind_is_admitted_to_
///   its_own_variant` — the admitted destinations reach the configuration
///   unchanged in order, so `config` over `SHIPPED_DEFAULTS` with each
///   member of `DESTINATION_KINDS` named through `--set` in `output` and
///   `report` `destinations` returns `Ok` whose lists hold the variant of
///   that name, a file holding the path its own list's entry carried, as
///   an outside caller observes it.
/// Arrange: for each member of `DESTINATION_KINDS` a command line whose
///   `--set` names `declared_name` of the member in both lists, its path
///   chosen by an exhaustive `match` on the member (absent for
///   `StandardOutput`, a distinct path in each list for `File`).
/// Act:     `config` through the crate root, once per member.
/// Assert:  `output.destinations` equals the `OutputDestination` the member
///   names and `output.report_destinations` equals the `ReportDestination`
///   the member names.
/// Boundary: the public call `config`; the route `layers`, `admit_output`,
///   then `config`.
/// Mocked:  none.
#[test]
fn every_declared_destination_kind_is_admitted_to_its_own_variant() {
    // Arrange — the deps and the payload are invariant across every member.
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
    // Act / Assert — every member of `DESTINATION_KINDS` in turn.
    for member in DESTINATION_KINDS {
        let (output_assignment, report_assignment, expected_output, expected_report) = match member
        {
            DestinationKind::StandardOutput => (
                format!(
                    "output.destinations=[{{kind=\"{}\"}}]",
                    declared_name(member)
                ),
                format!(
                    "report.destinations=[{{kind=\"{}\"}}]",
                    declared_name(member)
                ),
                OutputDestination::StandardOutput,
                ReportDestination::StandardOutput,
            ),
            DestinationKind::File => (
                format!(
                    "output.destinations=[{{kind=\"{}\",path=\"outcome.txt\"}}]",
                    declared_name(member)
                ),
                format!(
                    "report.destinations=[{{kind=\"{}\",path=\"report.json\"}}]",
                    declared_name(member)
                ),
                OutputDestination::File {
                    path: PathBuf::from("outcome.txt"),
                },
                ReportDestination::File {
                    path: PathBuf::from("report.json"),
                },
            ),
        };
        let configuration = config(
            &deps,
            ConfigParams {
                arguments: vec![
                    program.clone(),
                    set_flag.clone(),
                    OsString::from(output_assignment),
                    set_flag.clone(),
                    OsString::from(report_assignment),
                ],
            },
            payload.clone(),
        )
        .expect("the member is admitted")
        .configuration;
        assert_eq!(configuration.output.destinations, vec![expected_output]);
        assert_eq!(
            configuration.output.report_destinations,
            vec![expected_report]
        );
    }
    // every_declared_destination_kind_is_admitted_to_its_own_variant
}

/// Contract: carried entry `a_destination_list_keeps_the_order_the_document_
///   holds` — the admitted destinations reach the configuration unchanged
///   in order, so `config` over `SHIPPED_DEFAULTS` with `output` and
///   `report` `destinations` overridden through `--set` returns `Ok` whose
///   lists hold the admitted destinations in the order the entries were
///   held, each file with its own path, as an outside caller observes it.
/// Arrange: a command line whose `--set` gives each list every member of
///   `DESTINATION_KINDS` in declared order and a command line whose
///   `--set` gives them in reverse, each file with a path distinct from
///   every other file's.
/// Act:     `config` through the crate root, once per order.
/// Assert:  each admitted list equals the expected destinations in order.
/// Boundary: the public call `config`; the route `layers`, `admit_output`,
///   then `config`.
/// Mocked:  none.
#[test]
fn a_destination_list_keeps_the_order_the_document_holds() {
    // Arrange — the deps and the payload are invariant across every order.
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
    let declared: Vec<DestinationKind> = DESTINATION_KINDS.to_vec();
    let reversed: Vec<DestinationKind> = DESTINATION_KINDS.iter().rev().copied().collect();
    // Act / Assert — the declared order and the reverse in turn.
    for order in [declared, reversed] {
        let entries = |prefix: &str, order: &[DestinationKind]| {
            order
                .iter()
                .enumerate()
                .map(|(index, member)| match member {
                    DestinationKind::StandardOutput => {
                        format!("{{kind=\"{}\"}}", declared_name(member))
                    }
                    DestinationKind::File => format!(
                        "{{kind=\"{}\",path=\"{prefix}-{index}\"}}",
                        declared_name(member)
                    ),
                })
                .collect::<Vec<_>>()
                .join(",")
        };
        let arguments = vec![
            program.clone(),
            set_flag.clone(),
            OsString::from(format!(
                "output.destinations=[{}]",
                entries("output", &order)
            )),
            set_flag.clone(),
            OsString::from(format!(
                "report.destinations=[{}]",
                entries("report", &order)
            )),
        ];
        let expected_output: Vec<OutputDestination> = order
            .iter()
            .enumerate()
            .map(|(index, member)| match member {
                DestinationKind::StandardOutput => OutputDestination::StandardOutput,
                DestinationKind::File => OutputDestination::File {
                    path: PathBuf::from(format!("output-{index}")),
                },
            })
            .collect();
        let expected_report: Vec<ReportDestination> = order
            .iter()
            .enumerate()
            .map(|(index, member)| match member {
                DestinationKind::StandardOutput => ReportDestination::StandardOutput,
                DestinationKind::File => ReportDestination::File {
                    path: PathBuf::from(format!("report-{index}")),
                },
            })
            .collect();
        let configuration = config(&deps, ConfigParams { arguments }, payload.clone())
            .expect("the entries are admitted")
            .configuration;
        assert_eq!(configuration.output.destinations, expected_output);
        assert_eq!(configuration.output.report_destinations, expected_report);
    }
    // a_destination_list_keeps_the_order_the_document_holds
}

/// Contract: carried entry `an_unadmissible_destination_list_is_never_
///   admitted` — a refusal reaches the caller inside
///   `ConfigErrorReturn::Output`, so `config` over `SHIPPED_DEFAULTS` with
///   `output` or `report` `destinations` empty, holding an entry whose
///   `kind` is no member's `Debug` text, or holding a `File` entry with no
///   path, returns `Err` of `Output` carrying the list's variant with the
///   entry's index and, for a kind, its text, as an outside caller
///   observes it.
/// Arrange: for each list a command line with that list empty, holding one
///   entry of the kind `"unknown"`, or holding an entry of the kind
///   `File` with no path, the other list the shipped default.
/// Act:     `config` through the crate root, once per case.
/// Assert:  each result is `Err` of `Output` carrying the refused list's
///   variant with the index and, for a kind, the text.
/// Boundary: the public call `config`; the route `layers`, `admit_output`,
///   then `config`.
/// Mocked:  none.
#[test]
fn an_unadmissible_destination_list_is_never_admitted() {
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

    // The output list's three refusals.
    let result = config(&deps, set("output.destinations=[]"), payload.clone());
    assert!(
        matches!(
            result,
            Err(ConfigErrorReturn::Output(
                AdmitOutputErrorReturn::OutputDestinationsEmpty
            ))
        ),
        "the refusal is Output(OutputDestinationsEmpty)"
    );
    let result = config(
        &deps,
        set(r#"output.destinations=[{kind="unknown"}]"#),
        payload.clone(),
    );
    assert!(
        matches!(
            result,
            Err(ConfigErrorReturn::Output(
                AdmitOutputErrorReturn::OutputDestinationKind {
                    index: 0,
                    ref text
                }
            )) if text == "unknown"
        ),
        "the refusal is Output(OutputDestinationKind) carrying the text"
    );
    let result = config(
        &deps,
        set(r#"output.destinations=[{kind="File"}]"#),
        payload.clone(),
    );
    assert!(
        matches!(
            result,
            Err(ConfigErrorReturn::Output(
                AdmitOutputErrorReturn::OutputDestinationPath { index: 0 }
            ))
        ),
        "the refusal is Output(OutputDestinationPath)"
    );

    // The report list's three refusals.
    let result = config(&deps, set("report.destinations=[]"), payload.clone());
    assert!(
        matches!(
            result,
            Err(ConfigErrorReturn::Output(
                AdmitOutputErrorReturn::ReportDestinationsEmpty
            ))
        ),
        "the refusal is Output(ReportDestinationsEmpty)"
    );
    let result = config(
        &deps,
        set(r#"report.destinations=[{kind="unknown"}]"#),
        payload.clone(),
    );
    assert!(
        matches!(
            result,
            Err(ConfigErrorReturn::Output(
                AdmitOutputErrorReturn::ReportDestinationKind {
                    index: 0,
                    ref text
                }
            )) if text == "unknown"
        ),
        "the refusal is Output(ReportDestinationKind) carrying the text"
    );
    let result = config(
        &deps,
        set(r#"report.destinations=[{kind="File"}]"#),
        payload.clone(),
    );
    assert!(
        matches!(
            result,
            Err(ConfigErrorReturn::Output(
                AdmitOutputErrorReturn::ReportDestinationPath { index: 0 }
            ))
        ),
        "the refusal is Output(ReportDestinationPath)"
    );
    // an_unadmissible_destination_list_is_never_admitted
}

// Public-surface enumeration — own entries, each proven by the block of the
//   same name: `the_shipped_defaults_configure_the_harness`,
//   `a_file_and_a_flag_each_replace_a_shipped_default`,
//   `an_unadmissible_value_is_refused_by_the_function_that_admits_it`,
//   `a_value_this_function_admits_is_refused_with_its_own_variant`,
//   `the_verifier_is_selected_with_the_settings_the_document_holds`, and
//   `the_first_refusal_is_returned_in_the_stated_order`; carried entries,
//   each proven by the block of the same name: `layers`
//   `layered_sources_merge_in_command_line_order`; `admit_selections`
//   `every_member_of_every_declared_set_is_admitted_by_its_own_name` and
//   `a_mock_selections_name_is_never_admitted`; `admit_statements`
//   `every_declared_kind_admits_its_value_text_to_its_own_variant`,
//   `a_kind_outside_the_declared_sets_is_never_admitted`,
//   `every_context_carries_the_network_profiles_chain_identifier_and_
//   entitlement_contract`, `identities_are_admitted_under_the_scope_the_
//   payload_names`, `every_curve_has_its_own_deployment_record_in_curve_
//   order`, and
//   `a_curve_without_a_record_and_a_record_without_a_curve_are_each_
//   refused_with_the_label`; `admit_generate`
//   `every_declared_kind_is_admitted_to_its_own_settings_and_configuration`,
//   `every_declared_curve_directory_reaches_the_field_its_curve_names`, and
//   `an_undeclared_name_is_never_admitted`; `admit_output`
//   `every_declared_destination_kind_is_admitted_to_its_own_variant`,
//   `a_destination_list_keeps_the_order_the_document_holds`, and
//   `an_unadmissible_destination_list_is_never_admitted`.
